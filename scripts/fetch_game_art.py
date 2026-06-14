#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Fetch uniform game cover art from SteamGridDB.
=============================================

Pulls one landscape capsule per game listed in engine.content.GAMES, crops it
all to the same size, and writes assets/games/<key>.jpg (+ a manifest). The
profile engine (games card) then embeds these as base64 — so this script only
needs to run when the roster changes, not on every profile rebuild.

Run:
  SGDB_KEY=xxxxx uv run --with pillow scripts/fetch_game_art.py
  uv run --with pillow scripts/fetch_game_art.py --key xxxxx

Get a free key at steamgriddb.com → Preferences → API.
"""

from __future__ import annotations

import argparse
import io
import json
import os
import sys
import time
import urllib.parse
import urllib.request

for _s in (sys.stdout, sys.stderr):
    try:
        _s.reconfigure(encoding="utf-8")
    except Exception:
        pass

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from engine import content as c  # noqa: E402
from engine import log  # noqa: E402

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUT = os.path.join(ROOT, "assets", "games")
API = "https://www.steamgriddb.com/api/v2"

TARGET_W, TARGET_H = 460, 215  # uniform capsule, matches the card tile aspect


def _get(path, token):
    req = urllib.request.Request(API + path)
    req.add_header("Authorization", f"Bearer {token}")
    req.add_header("User-Agent", "vai-profile-engine")
    with urllib.request.urlopen(req, timeout=30) as r:
        return json.loads(r.read().decode("utf-8"))


def _download(url):
    req = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0"})
    return urllib.request.urlopen(req, timeout=30).read()


def find_game_id(query, token):
    data = _get(f"/search/autocomplete/{urllib.parse.quote(query)}", token)
    results = data.get("data", []) if isinstance(data, dict) else []
    return results[0]["id"] if results else None


def pick_art_url(gid, token):
    """Prefer a 460x215 capsule, then 920x430, then any grid, then a hero."""
    attempts = [
        f"/grids/game/{gid}?dimensions=460x215&types=static",
        f"/grids/game/{gid}?dimensions=920x430&types=static",
        f"/grids/game/{gid}?types=static",
        f"/heroes/game/{gid}?types=static",
    ]
    for path in attempts:
        try:
            data = _get(path, token)
        except Exception:
            continue
        items = data.get("data", []) if isinstance(data, dict) else []
        for it in items:
            url = it.get("url")
            if url:
                return url
    return None


def process(raw_bytes, dest):
    from PIL import Image
    im = Image.open(io.BytesIO(raw_bytes)).convert("RGB")
    # cover-crop to TARGET (fill, centre)
    sw, sh = im.size
    scale = max(TARGET_W / sw, TARGET_H / sh)
    nw, nh = int(sw * scale + 0.5), int(sh * scale + 0.5)
    im = im.resize((nw, nh), Image.LANCZOS)
    left = (nw - TARGET_W) // 2
    topc = (nh - TARGET_H) // 2
    im = im.crop((left, topc, left + TARGET_W, topc + TARGET_H))
    im.save(dest, "JPEG", quality=74, optimize=True)
    return os.path.getsize(dest)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--key", default=os.environ.get("SGDB_KEY") or os.environ.get("STEAMGRIDDB_KEY"))
    # --only > GAME_ONLY env / GitHub repo Variable (vars.GAME_ONLY). Blank = all.
    ap.add_argument("--only", default=os.environ.get("GAME_ONLY"),
                    help="comma list of game keys (or set GAME_ONLY env / GH variable)")
    args = ap.parse_args()
    if not args.key:
        print("! No SteamGridDB key. Set SGDB_KEY or pass --key. (steamgriddb.com → Preferences → API)", file=sys.stderr)
        sys.exit(2)

    only = set(args.only.split(",")) if args.only else None
    os.makedirs(OUT, exist_ok=True)
    log.banner("Game Art Fetcher", f"SteamGridDB → {TARGET_W}x{TARGET_H} capsules")
    log.section("Fetching covers")

    manifest = {}
    for g in c.GAMES:
        key = g["key"]
        if only and key not in only:
            continue
        try:
            gid = find_game_id(g["query"], args.key)
            if not gid:
                log.fail(key, "not found on SteamGridDB")
                continue
            url = pick_art_url(gid, args.key)
            if not url:
                log.fail(key, f"no art for id={gid}")
                continue
            size = process(_download(url), os.path.join(OUT, key + ".jpg"))
            manifest[key] = {"sgdb_id": gid, "source": url}
            log.ok(key, f"{size/1024:.1f} KB  (sgdb id={gid})")
            time.sleep(0.3)
        except Exception as e:
            log.fail(key, str(e))

    with open(os.path.join(OUT, "manifest.json"), "w", encoding="utf-8") as f:
        json.dump(manifest, f, indent=2, ensure_ascii=False)
    log.done(f"{len(manifest)}/{len(c.GAMES)} covers → assets/games/")


if __name__ == "__main__":
    main()
