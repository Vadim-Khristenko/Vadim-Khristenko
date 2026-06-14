# -*- coding: utf-8 -*-
"""
Game / best-game art fetcher (SteamGridDB + arbitrary URLs / local files).

Static images are cover-cropped to a uniform size (Pillow); animated WEBP/GIF are
re-encoded compact (animation preserved) with a size cap + static fallback;
transparent sprites are kept as PNG. Downloads retry with backoff. Pillow is only
needed here — the card renderer stays stdlib-only.
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

from . import content as c
from . import log

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
OUT = os.path.join(ROOT, "assets", "games")
API = "https://www.steamgriddb.com/api/v2"
TARGET_W, TARGET_H = 460, 215
SIZE_CAP = 2_200_000
DOWNLOAD_ATTEMPTS = 4


def _get(path, token):
    req = urllib.request.Request(API + path)
    req.add_header("Authorization", f"Bearer {token}")
    req.add_header("User-Agent", "vai-profile-engine")
    with urllib.request.urlopen(req, timeout=30) as r:
        return json.loads(r.read().decode("utf-8"))


def _download(url, attempts=DOWNLOAD_ATTEMPTS):
    """Fetch bytes with retries + backoff. Raises the last error only after every
    attempt fails (callers turn that into a warning)."""
    last = None
    for i in range(attempts):
        try:
            req = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0"})
            data = urllib.request.urlopen(req, timeout=30).read()
            if not data:
                raise ValueError("empty response")
            return data
        except Exception as e:
            last = e
            if i < attempts - 1:
                log.warn(f"download retry {i + 1}/{attempts - 1}: {e}")
                time.sleep(1.2 * (i + 1))
    raise last


def get_bytes(src):
    """Fetch bytes from a URL or a local path (relative to the repo root)."""
    if src.startswith(("http://", "https://")):
        return _download(src)
    path = src if os.path.isabs(src) else os.path.join(ROOT, src)
    with open(path, "rb") as f:
        return f.read()


def find_game_id(query, token):
    data = _get(f"/search/autocomplete/{urllib.parse.quote(query)}", token)
    results = data.get("data", []) if isinstance(data, dict) else []
    return results[0]["id"] if results else None


def pick_art_url(gid, token):
    """Prefer a 460x215 capsule, then 920x430, then any grid, then a hero."""
    for path in (f"/grids/game/{gid}?dimensions=460x215&types=static",
                 f"/grids/game/{gid}?dimensions=920x430&types=static",
                 f"/grids/game/{gid}?types=static",
                 f"/heroes/game/{gid}?types=static"):
        try:
            data = _get(path, token)
        except Exception:
            continue
        for it in (data.get("data", []) if isinstance(data, dict) else []):
            if it.get("url"):
                return it["url"]
    return None


_MEDIA_EXTS = (".webp", ".gif", ".avif", ".apng", ".png", ".jpg", ".jpeg")


def _purge(dest_base):
    """Remove any prior <dest_base>.<ext> so a format switch (e.g. webp→jpg) never
    leaves a stale file that find_media() would pick by extension priority."""
    for ext in _MEDIA_EXTS:
        try:
            os.remove(dest_base + ext)
        except OSError:
            pass


def _cover_crop(im, tw, th):
    from PIL import Image
    sw, sh = im.size
    scale = max(tw / sw, th / sh)
    nw, nh = int(sw * scale + 0.5), int(sh * scale + 0.5)
    im = im.resize((nw, nh), Image.LANCZOS)
    left, topc = (nw - tw) // 2, (nh - th) // 2
    return im.crop((left, topc, left + tw, topc + th))


def process(raw_bytes, dest, tw=TARGET_W, th=TARGET_H):
    from PIL import Image
    im = Image.open(io.BytesIO(raw_bytes)).convert("RGB")
    _cover_crop(im, tw, th).save(dest, "JPEG", quality=74, optimize=True)
    return os.path.getsize(dest)


def save_media(raw_bytes, url, dest_base, tw, th):
    """dest_base.<ext>: animated → compact cropped WEBP (cap → static fallback);
    transparent static → PNG; else JPEG. Returns the written path."""
    from PIL import Image
    _purge(dest_base)  # drop any prior extension so a format switch can't go stale
    im = None
    try:
        im = Image.open(io.BytesIO(raw_bytes))
        n = getattr(im, "n_frames", 1)
        animated = getattr(im, "is_animated", False) and n > 1
    except Exception:
        animated = False

    if animated:
        step = max(1, n // 24)
        frames = []
        for i in range(0, n, step):
            im.seek(i)
            frames.append(_cover_crop(im.convert("RGBA"), tw, th))
        dur = max(60, im.info.get("duration", 80) * step)
        dest = dest_base + ".webp"
        frames[0].save(dest, "WEBP", save_all=True, append_images=frames[1:],
                       duration=dur, loop=0, quality=42, method=4)
        if os.path.getsize(dest) <= SIZE_CAP:
            return dest
        os.remove(dest)
        im.seek(0)
        dest = dest_base + ".jpg"
        _cover_crop(im.convert("RGB"), tw, th).save(dest, "JPEG", quality=74, optimize=True)
        return dest

    if im is not None and im.mode in ("RGBA", "LA", "P") and ("transparency" in im.info or im.mode != "P"):
        dest = dest_base + ".png"
        _cover_crop(im.convert("RGBA"), tw, th).save(dest, "PNG", optimize=True)
        return dest
    dest = dest_base + ".jpg"
    process(raw_bytes, dest, tw, th)
    return dest


def _char_key(short):
    return "char_" + "".join(x for x in short.lower() if x.isalnum())


def fetch_bestgame(token):
    g = c.BEST_GAME
    out = os.path.join(ROOT, "assets", "bestgame")
    os.makedirs(out, exist_ok=True)
    log.section(f"Best game: {g.get('title', '?')}")

    portrait = g.get("cover_mode", "portrait") != "landscape"
    ctw, cth = (300, 450) if portrait else (1100, 340)
    url = g.get("art_url") or ""
    if not url and token:
        gid = find_game_id(g.get("query", g.get("title", "")), token)
        if gid:
            try:
                data = _get(f"/heroes/game/{gid}?types=static", token)
                items = data.get("data", []) if isinstance(data, dict) else []
                url = items[0]["url"] if items else (pick_art_url(gid, token) or "")
            except Exception:
                url = pick_art_url(gid, token) or ""
    if url:
        try:
            dest = save_media(get_bytes(url), url, os.path.join(out, "cover"), ctw, cth)
            log.ok("cover", f"{os.path.getsize(dest) / 1024:.1f} KB  {os.path.basename(dest)}")
        except Exception as e:
            log.fail("cover", f"all retries failed ({e})")
    else:
        log.warn("cover: no art_url and no SteamGridDB key")

    for ch in g.get("characters", []):
        src = ch.get("art_url") or ""
        if not src:
            log.step(ch.get("short", ch["name"]), "—", "no art_url → avatar fallback")
            continue
        try:
            dest = save_media(get_bytes(src), src, os.path.join(out, _char_key(ch["short"])), 300, 300)
            log.ok(ch.get("short", ch["name"]), f"{os.path.getsize(dest) / 1024:.1f} KB  {os.path.basename(dest)}")
        except Exception as e:
            log.fail(ch.get("short", ch["name"]), f"all retries failed → avatar fallback ({e})")


def fetch_games(token, only=None):
    os.makedirs(OUT, exist_ok=True)
    log.section("Fetching game-shelf covers")
    manifest = {}
    for g in c.GAMES:
        if only and g["key"] not in only:
            continue
        try:
            gid = find_game_id(g["query"], token)
            if not gid:
                log.fail(g["key"], "not found on SteamGridDB")
                continue
            url = pick_art_url(gid, token)
            if not url:
                log.fail(g["key"], f"no art for id={gid}")
                continue
            _purge(os.path.join(OUT, g["key"]))   # clear any prior extension first
            size = process(_download(url), os.path.join(OUT, g["key"] + ".jpg"))
            manifest[g["key"]] = {"sgdb_id": gid, "source": url}
            log.ok(g["key"], f"{size / 1024:.1f} KB  (sgdb id={gid})")
            time.sleep(0.3)
        except Exception as e:
            log.fail(g["key"], str(e))
    with open(os.path.join(OUT, "manifest.json"), "w", encoding="utf-8") as f:
        json.dump(manifest, f, indent=2, ensure_ascii=False)
    log.step("shelf", f"{len(manifest)}/{len(c.GAMES)}", "covers → assets/games/")


def fetch_all(token, only=None, bestgame=False, games=False):
    if bestgame:
        fetch_bestgame(token)
    if games:
        fetch_games(token, only=only)


def main(argv=None):
    ap = argparse.ArgumentParser(prog="art")
    ap.add_argument("--key", default=os.environ.get("SGDB_KEY") or os.environ.get("STEAMGRIDDB_KEY"))
    ap.add_argument("--only", default=os.environ.get("GAME_ONLY"),
                    help="comma list of game keys (or GAME_ONLY env / GH variable)")
    ap.add_argument("--bestgame", action="store_true")
    ap.add_argument("--skip-games", action="store_true")
    args = ap.parse_args(argv)

    log.banner("Game Art Fetcher", "SteamGridDB → uniform covers")
    only = set(args.only.split(",")) if args.only else None
    # bestgame uses configured URLs and may not need a key; the shelf always does
    fetch_all(args.key, only=only, bestgame=args.bestgame, games=not args.skip_games)
    if not args.bestgame and not args.key:
        log.warn("no SteamGridDB key — shelf covers need SGDB_KEY")
    log.done("art → assets/")


if __name__ == "__main__":
    main()
