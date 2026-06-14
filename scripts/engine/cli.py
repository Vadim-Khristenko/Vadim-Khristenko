# -*- coding: utf-8 -*-
"""
Friendly command-line front-end for the engine. Exposed as uv entry points:

  uv run build                 # render cards + refresh README (no network art)
  uv run build --only header   # one/few cards, skips README rewrite
  uv run preview               # build, then open a stacked HTML preview in the browser
  uv run rebuild               # fetch best-game art (+ --games) then build everything
  uv run --extra art bestgame  # (re)download best-game cover + character art
  uv run --extra art art       # (re)download the game-shelf covers   [--only key,key]

All art commands accept a SteamGridDB key via --key or the SGDB_KEY env var, and
--only / GAME_ONLY to target specific games. Art fetching needs Pillow — use the
`art` extra (uv run --extra art ...) or `uv sync --extra art`.
"""

from __future__ import annotations

import argparse
import os
import sys
import webbrowser

from . import log, run, art


def _have_pillow():
    try:
        import PIL  # noqa: F401
        return True
    except Exception:
        return False


def _key(args):
    return args.key or os.environ.get("SGDB_KEY") or os.environ.get("STEAMGRIDDB_KEY")


# --------------------------------------------------------------------------- #
# Commands                                                                    #
# --------------------------------------------------------------------------- #

def cmd_build(argv):
    ap = argparse.ArgumentParser(prog="build")
    ap.add_argument("--only", help="comma list of card names")
    ap.add_argument("--no-readme", action="store_true")
    a = ap.parse_args(argv)
    only = set(a.only.split(",")) if a.only else None
    run.build_all(only=only, no_readme=a.no_readme)


def cmd_rebuild(argv):
    ap = argparse.ArgumentParser(prog="rebuild")
    ap.add_argument("--only", help="comma list of card names to render")
    ap.add_argument("--games", action="store_true", help="also re-fetch the game-shelf covers")
    ap.add_argument("--no-fetch", action="store_true", help="skip art fetching, just render")
    ap.add_argument("--key", help="SteamGridDB key (or SGDB_KEY env)")
    a = ap.parse_args(argv)

    if not a.no_fetch:
        if not _have_pillow():
            log.warn("Pillow not available → skipping art fetch. Use:  uv run --extra art rebuild")
        else:
            key = _key(a)
            art.fetch_bestgame(key)                       # picks up new characters
            if a.games:
                game_only = set(os.environ["GAME_ONLY"].split(",")) if os.environ.get("GAME_ONLY") else None
                art.fetch_games(key, only=game_only)
    only = set(a.only.split(",")) if a.only else None
    run.build_all(only=only)


def cmd_preview(argv):
    ap = argparse.ArgumentParser(prog="preview")
    ap.add_argument("--only", help="comma list of card names to preview")
    ap.add_argument("--no-open", action="store_true", help="write the file but don't open a browser")
    a = ap.parse_args(argv)
    only = set(a.only.split(",")) if a.only else None
    run.build_all(only=only, no_readme=True)

    order = ["header", "divider", "research", "dashboard", "vibe", "bestgame", "games", "footer"]
    parts = []
    for name in order:
        if only and name not in only:
            continue
        p = os.path.join(run.ASSETS, name + ".svg")
        if os.path.exists(p):
            with open(p, encoding="utf-8") as f:
                parts.append(f'<div style="margin:12px 0">{f.read()}</div>')
    out_dir = os.path.join(run.ROOT, "tmp_prev")
    os.makedirs(out_dir, exist_ok=True)
    out = os.path.join(out_dir, "preview.html")
    with open(out, "w", encoding="utf-8") as f:
        f.write('<!doctype html><meta charset="utf-8"><title>VAI profile preview</title>'
                '<body style="margin:0;padding:20px;background:#0d0d12">' + "".join(parts) + "</body>")
    log.ok("preview", out)
    if not a.no_open:
        webbrowser.open("file:///" + out.replace("\\", "/"))


def cmd_art(argv):
    art.main(argv)


def cmd_bestgame(argv):
    art.main(["--bestgame", "--skip-games", *argv])


COMMANDS = {
    "build": cmd_build, "rebuild": cmd_rebuild, "preview": cmd_preview,
    "art": cmd_art, "bestgame": cmd_bestgame,
}


def main(argv=None):
    argv = list(sys.argv[1:] if argv is None else argv)
    cmd = argv[0] if argv and not argv[0].startswith("-") else "build"
    rest = argv[1:] if argv and not argv[0].startswith("-") else argv
    handler = COMMANDS.get(cmd)
    if not handler:
        print(f"unknown command: {cmd}\navailable: {', '.join(COMMANDS)}", file=sys.stderr)
        sys.exit(2)
    handler(rest)


# uv entry points (one per command — they read their own flags from sys.argv)
def build():    cmd_build(sys.argv[1:])
def rebuild():  cmd_rebuild(sys.argv[1:])
def preview():  cmd_preview(sys.argv[1:])
def art_cmd():  cmd_art(sys.argv[1:])
def bestgame(): cmd_bestgame(sys.argv[1:])


if __name__ == "__main__":
    main()
