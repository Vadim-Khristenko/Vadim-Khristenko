#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
VAI Profile Engine — orchestrator
=================================

Collects GitHub data once, then renders each card module *in isolation*:
a card that throws is logged and skipped, the rest still ship. Finally it
injects cache-busted image rows into README.md.

Usage:
  uv run scripts/build.py                 # build everything
  uv run scripts/build.py --only header,vibe
  uv run scripts/build.py --no-readme     # only regenerate SVGs
"""

from __future__ import annotations

import argparse
import datetime as _dt
import os
import sys
import xml.dom.minidom

# Windows consoles default to a legacy codepage (cp1251 here) that can't encode
# the glyphs we log. Force UTF-8 so local runs match CI (Ubuntu/UTF-8).
for _stream in (sys.stdout, sys.stderr):
    try:
        _stream.reconfigure(encoding="utf-8")
    except Exception:
        pass

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from engine import log  # noqa: E402
from engine import readme as readme_mod  # noqa: E402
from engine.data import GitHubData  # noqa: E402
from engine.cards import (  # noqa: E402
    header, divider, research, dashboard, vibe, games, footer,
)

USERNAME = os.environ.get("PROFILE_USER", "Vadim-Khristenko")
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ASSETS = os.path.join(ROOT, "assets")
README = os.path.join(ROOT, "README.md")
RAW_BASE = f"https://raw.githubusercontent.com/{USERNAME}/{USERNAME}/main/assets"

# Render order. Every card is CARD_W wide so they stack uniformly on mobile.
CARDS = [header, divider, research, dashboard, vibe, games, footer]


def _utcnow():
    return _dt.datetime.now(_dt.timezone.utc)


def build_context():
    now = _utcnow()
    iso = now.isocalendar()
    seed = iso[0] * 100 + iso[1]
    log.banner("VAI Profile Engine v2",
               f"user={USERNAME}  ·  week {iso[1]}/{iso[0]}  ·  seed={seed}")
    data = GitHubData(USERNAME).collect()
    return {
        "data": data,
        "seed": seed,
        "week": iso[1],
        "year": iso[0],
        "stamp": now.strftime("%Y-%m-%d %H:%M UTC"),
        "raw_base": RAW_BASE,
    }


def render(ctx, only=None):
    os.makedirs(ASSETS, exist_ok=True)
    log.section("Rendering cards")
    ok = []
    for module in CARDS:
        name = module.ASSET
        if only and name.replace(".svg", "") not in only:
            continue
        try:
            svg = module.build(ctx)
            xml.dom.minidom.parseString(svg.encode("utf-8"))  # validate before writing
            with open(os.path.join(ASSETS, name), "w", encoding="utf-8") as f:
                f.write(svg)
            log.ok(name, f"{len(svg)/1024:.1f} KB")
            ok.append(name)
        except Exception as e:  # autonomous: one bad card never breaks the rest
            log.fail(name, str(e))
    return ok


def update_readme(ctx):
    if not os.path.exists(README):
        log.warn("README.md not found")
        return
    log.section("Updating README")
    with open(README, "r", encoding="utf-8") as f:
        text = f.read()
    for key, content in readme_mod.blocks(RAW_BASE, str(ctx["seed"])).items():
        text = readme_mod.inject(text, key, content)
    with open(README, "w", encoding="utf-8") as f:
        f.write(text)
    log.ok("README", "markers refreshed")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--only", help="comma list of card names (e.g. header,vibe)")
    ap.add_argument("--no-readme", action="store_true")
    args = ap.parse_args()
    only = set(args.only.split(",")) if args.only else None

    ctx = build_context()
    built = render(ctx, only=only)
    if not args.no_readme and not only:
        update_readme(ctx)
    log.done(f"{len(built)} cards · seed {ctx['seed']}")


if __name__ == "__main__":
    main()
