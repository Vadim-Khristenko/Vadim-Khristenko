# -*- coding: utf-8 -*-
"""
Orchestrator: collect GitHub data once, render every card in isolation (one bad
card never breaks the rest), then inject cache-busted image rows into README.md.
Used by the CLI (`uv run build` / `rebuild` / `preview`) and the thin
scripts/build.py shim that CI calls.
"""

from __future__ import annotations

import argparse
import datetime as _dt
import os
import sys
import xml.dom.minidom

for _stream in (sys.stdout, sys.stderr):
    try:
        _stream.reconfigure(encoding="utf-8")
    except Exception:
        pass

from . import log
from . import readme as readme_mod
from . import content as c
from .data import GitHubData
from .cards import header, divider, research, dashboard, vibe, bestgame, games, footer

USERNAME = os.environ.get("PROFILE_USER", "Vadim-Khristenko")
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
ASSETS = os.path.join(ROOT, "assets")
README = os.path.join(ROOT, "README.md")
RAW_BASE = f"https://raw.githubusercontent.com/{USERNAME}/{USERNAME}/main/assets"

# Render order. Every card is CARD_W wide so they stack uniformly on mobile.
CARDS = [header, divider, research, dashboard, vibe, bestgame, games, footer]


def _utcnow():
    return _dt.datetime.now(_dt.timezone.utc)


def build_context():
    now = _utcnow()
    iso = now.isocalendar()
    seed = iso[0] * 100 + iso[1]
    vibe_seed = now.toordinal() // 2   # rotates the vibe card every 2 days
    log.banner("VAI Profile Engine v2",
               f"user={USERNAME}  ·  week {iso[1]}/{iso[0]}  ·  vibe-bucket={vibe_seed}")
    data = GitHubData(USERNAME).collect()
    return {
        "data": data, "seed": seed, "vibe_seed": vibe_seed,
        "week": iso[1], "year": iso[0],
        "stamp": now.strftime("%Y-%m-%d %H:%M UTC"), "raw_base": RAW_BASE,
    }


def _char_key(short):
    return "char_" + "".join(x for x in short.lower() if x.isalnum())


def warn_missing_art():
    """Nudge the user when an asset is referenced by config but not fetched yet —
    the exact cause of 'I added a character but it shows an avatar'."""
    missing = []
    bg_dir = os.path.join(ASSETS, "bestgame")
    g = getattr(c, "BEST_GAME", {})
    for ch in g.get("characters", []):
        if ch.get("art_url") and not _find(bg_dir, _char_key(ch["short"])):
            missing.append(ch.get("short", ch["name"]))
    if missing:
        log.warn(f"character art not fetched yet: {', '.join(missing)}")
        log.warn("  run:  uv run --extra art rebuild   (or: uv run --extra art bestgame)")


def _find(directory, key):
    for ext in (".webp", ".gif", ".avif", ".apng", ".png", ".jpg", ".jpeg"):
        if os.path.exists(os.path.join(directory, key + ext)):
            return True
    return False


def render(ctx, only=None):
    os.makedirs(ASSETS, exist_ok=True)
    log.section("Rendering cards")
    built = []
    for module in CARDS:
        name = module.ASSET
        if only and name.replace(".svg", "") not in only:
            continue
        try:
            svg = module.build(ctx)
            xml.dom.minidom.parseString(svg.encode("utf-8"))  # validate before writing
            with open(os.path.join(ASSETS, name), "w", encoding="utf-8") as f:
                f.write(svg)
            log.ok(name, f"{len(svg) / 1024:.1f} KB")
            built.append(name)
        except Exception as e:  # autonomous: one bad card never breaks the rest
            log.fail(name, str(e))
    return built


def update_readme(ctx):
    if not os.path.exists(README):
        log.warn("README.md not found")
        return
    log.section("Updating README")
    with open(README, "r", encoding="utf-8") as f:
        text = f.read()
    salt = str(ctx["vibe_seed"])[-5:]  # cache also refreshes on the 2-day cadence
    for key, content in readme_mod.blocks(RAW_BASE, ASSETS, salt).items():
        text = readme_mod.inject(text, key, content)
    with open(README, "w", encoding="utf-8") as f:
        f.write(text)
    log.ok("README", "markers refreshed")


def build_all(only=None, no_readme=False):
    ctx = build_context()
    built = render(ctx, only=only)
    warn_missing_art()
    if not no_readme and not only:
        update_readme(ctx)
    log.done(f"{len(built)} cards · seed {ctx['seed']}")
    return ctx


def main(argv=None):
    ap = argparse.ArgumentParser(prog="build")
    ap.add_argument("--only", help="comma list of card names (e.g. header,vibe)")
    ap.add_argument("--no-readme", action="store_true")
    args = ap.parse_args(argv)
    only = set(args.only.split(",")) if args.only else None
    build_all(only=only, no_readme=args.no_readme)


if __name__ == "__main__":
    main()
