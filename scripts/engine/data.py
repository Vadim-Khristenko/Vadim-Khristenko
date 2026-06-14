# -*- coding: utf-8 -*-
"""
Data layer: everything the cards need from GitHub.

Sources, with graceful degradation:
  * REST   /users, /repos, /repos/*/languages
  * GraphQL contributionsCollection (precise commit counts; needs a token)
  * HTML scrape of /users/<u>/contributions (daily calendar; works tokenless)

Lines of code are estimated from total language bytes (always available and
immediate) — GitHub's /stats/code_frequency endpoint is too flaky (persistent
202s) to depend on for a live profile. Stdlib only; a missing token just means
commit counts fall back to the tokenless calendar scrape.
"""

from __future__ import annotations

import datetime as _dt
import json
import os
import re
import urllib.error
import urllib.request

from . import log

AVG_BYTES_PER_LINE = 34  # rough constant for the LOC estimate


def _utcnow():
    return _dt.datetime.now(_dt.timezone.utc)


class GitHubData:
    def __init__(self, username: str):
        self.user = username
        self.token = os.environ.get("GH_TOKEN") or os.environ.get("GITHUB_TOKEN")
        self.profile = {}
        self.repos = []
        self.stars = 0
        self.forks = 0
        self.repo_count = 0
        self.lang_bytes = {}
        self.total_bytes = 0
        self.loc = 0
        self.commits = {"7d": None, "30d": None, "1y": None}
        self.daily = []
        self.streak = 0
        self.best_day = 0
        self.active_days = 0
        self.commit_label = "contributions"
        self.most_active_repo = None

    # -- low level ---------------------------------------------------------- #

    def _rest(self, path: str):
        req = urllib.request.Request(f"https://api.github.com{path}")
        req.add_header("Accept", "application/vnd.github+json")
        req.add_header("User-Agent", "vai-profile-engine")
        if self.token:
            req.add_header("Authorization", f"Bearer {self.token}")
        with urllib.request.urlopen(req, timeout=30) as r:
            return r.status, json.loads(r.read().decode("utf-8"))

    def _graphql(self, query: str, variables: dict):
        if not self.token:
            return None
        body = json.dumps({"query": query, "variables": variables}).encode("utf-8")
        req = urllib.request.Request("https://api.github.com/graphql", data=body, method="POST")
        req.add_header("Authorization", f"Bearer {self.token}")
        req.add_header("User-Agent", "vai-profile-engine")
        req.add_header("Content-Type", "application/json")
        try:
            with urllib.request.urlopen(req, timeout=30) as r:
                return json.loads(r.read().decode("utf-8"))
        except Exception as e:
            log.warn(f"graphql: {e}")
            return None

    # -- collectors --------------------------------------------------------- #

    def fetch_profile(self):
        try:
            _, self.profile = self._rest(f"/users/{self.user}")
            log.step("profile", self.profile.get("login", "?"),
                     f"{self.profile.get('followers', 0)} followers")
        except Exception as e:
            log.warn(f"profile: {e}")

    def fetch_repos(self):
        page = 1
        while page <= 10:
            try:
                _, chunk = self._rest(
                    f"/users/{self.user}/repos?per_page=100&page={page}&type=owner&sort=pushed"
                )
            except Exception as e:
                log.warn(f"repos p{page}: {e}")
                break
            if not chunk:
                break
            self.repos.extend(chunk)
            if len(chunk) < 100:
                break
            page += 1

        owned = [r for r in self.repos if not r.get("fork")]
        self.repo_count = len(owned)
        self.stars = sum(r.get("stargazers_count", 0) for r in self.repos)
        self.forks = sum(r.get("forks_count", 0) for r in self.repos)
        # owned is already sorted by pushed desc → the freshest is where changes flow.
        # Skip the profile repo itself: its constant engine auto-commits would
        # otherwise always win and pin it to the top.
        for r in owned:
            if r.get("name", "").lower() != self.user.lower():
                self.most_active_repo = r.get("name")
                break
        else:
            self.most_active_repo = owned[0].get("name") if owned else None
        log.step("repositories", self.repo_count, f"{self.stars}★  {self.forks} forks")
        return owned

    def fetch_languages(self, owned):
        for r in owned:
            try:
                _, langs = self._rest(f"/repos/{self.user}/{r['name']}/languages")
                for k, v in langs.items():
                    self.lang_bytes[k] = self.lang_bytes.get(k, 0) + v
            except Exception:
                lang = r.get("language")
                if lang:
                    self.lang_bytes[lang] = self.lang_bytes.get(lang, 0) + 1
        self.total_bytes = sum(self.lang_bytes.values())
        self.loc = self.total_bytes // AVG_BYTES_PER_LINE
        top = max(self.lang_bytes.items(), key=lambda kv: kv[1])[0] if self.lang_bytes else "—"
        log.step("languages", len(self.lang_bytes), f"top={top}  ≈{self.loc} LOC")

    def fetch_commits_graphql(self):
        if not self.token:
            return
        q = """query($login:String!,$from:DateTime!,$to:DateTime!){
          user(login:$login){ contributionsCollection(from:$from,to:$to){ totalCommitContributions } } }"""
        now = _utcnow()
        ranges = {"7d": now - _dt.timedelta(days=7),
                  "30d": now - _dt.timedelta(days=30),
                  "1y": now - _dt.timedelta(days=365)}
        ok = False
        for key, frm in ranges.items():
            res = self._graphql(q, {"login": self.user,
                                    "from": frm.strftime("%Y-%m-%dT%H:%M:%SZ"),
                                    "to": now.strftime("%Y-%m-%dT%H:%M:%SZ")})
            try:
                self.commits[key] = res["data"]["user"]["contributionsCollection"]["totalCommitContributions"]
                ok = True
            except Exception:
                pass
        if ok:
            self.commit_label = "commits"
            log.step("commits", f"{self.commits['7d']}/{self.commits['30d']}/{self.commits['1y']}", "7d/30d/1y")

    def fetch_contributions_scrape(self):
        try:
            req = urllib.request.Request(f"https://github.com/users/{self.user}/contributions")
            req.add_header("User-Agent", "Mozilla/5.0")
            html_text = urllib.request.urlopen(req, timeout=30).read().decode("utf-8", "ignore")
        except Exception as e:
            log.warn(f"contributions scrape: {e}")
            return

        counts = {}
        for m in re.finditer(r'<tool-tip[^>]*for="([^"]+)"[^>]*>([^<]*)</tool-tip>', html_text):
            cell_id, text = m.group(1), m.group(2)
            if text.startswith("No "):
                counts[cell_id] = 0
            else:
                num = re.match(r"([\d,]+)", text)
                counts[cell_id] = int(num.group(1).replace(",", "")) if num else 0

        days = []
        for m in re.finditer(r'<td[^>]*data-date="(\d{4}-\d{2}-\d{2})"[^>]*id="([^"]+)"', html_text):
            days.append((m.group(1), counts.get(m.group(2), 0)))
        days.sort(key=lambda d: d[0])
        self.daily = days
        if not days:
            return

        seq = [c for _, c in days]
        self.active_days = sum(1 for c in seq if c > 0)
        self.best_day = max(seq)
        # current streak — allow today (trailing 0) not to break it
        i = len(seq) - 1
        if seq[-1] == 0:
            i -= 1
        s = 0
        while i >= 0 and seq[i] > 0:
            s += 1
            i -= 1
        self.streak = s
        if self.commits["7d"] is None:
            self.commits["7d"] = sum(seq[-7:])
        if self.commits["30d"] is None:
            self.commits["30d"] = sum(seq[-30:])
        if self.commits["1y"] is None:
            self.commits["1y"] = sum(seq)
        log.step("calendar", f"{len(days)}d", f"streak={self.streak}  active={self.active_days}d  best={self.best_day}")

    # -- orchestration ------------------------------------------------------ #

    def collect(self):
        log.section("Fetching GitHub data")
        mode = "authenticated" if self.token else "tokenless (limited)"
        log.step("mode", mode)
        self.fetch_profile()
        owned = self.fetch_repos()
        self.fetch_languages(owned)
        self.fetch_commits_graphql()
        self.fetch_contributions_scrape()
        log.step("most active", self.most_active_repo or "—", "where changes land")
        return self
