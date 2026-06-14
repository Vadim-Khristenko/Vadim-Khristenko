# -*- coding: utf-8 -*-
"""Now-playing shelf. Embeds real cover art (assets/games/<key>.jpg, base64) when
present; otherwise falls back to a neon tile — all tiles share one capsule shape
so the shelf stays "one type" either way."""

import os

from .. import theme as t
from .. import content as c

ASSET = "games.svg"
_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
ART_DIR = os.path.join(_ROOT, "assets", "games")


def _art_b64(key):
    p = t.find_media(ART_DIR, key)
    return t.media_data_uri(p) if p else None


def build(ctx):
    w = t.CARD_W
    cols, gap, pad, top = 4, 16, t.MARGIN, 16
    tile_w = (w - 2 * pad - (cols - 1) * gap) / cols
    tile_h = round(tile_w / 2.14)  # Steam-capsule aspect → uniform "one type"
    rows = (len(c.GAMES) + cols - 1) // cols
    h = 42 + top + rows * tile_h + (rows - 1) * gap + 18

    defs, tiles = [], []
    real = 0
    for i, g in enumerate(c.GAMES):
        r, col = divmod(i, cols)
        x = pad + col * (tile_w + gap)
        y = top + r * (tile_h + gap)
        ca = g["ca"]
        clip = f"clip{i}"
        defs.append(f'<clipPath id="{clip}"><rect width="{tile_w:.1f}" height="{tile_h}" rx="11"/></clipPath>')
        defs.append(
            f'<linearGradient id="sh{i}" x1="0" y1="0" x2="0" y2="1">'
            f'<stop offset="0.45" stop-color="#000" stop-opacity="0"/>'
            f'<stop offset="1" stop-color="#05040a" stop-opacity="0.92"/></linearGradient>'
        )
        art = _art_b64(g["key"])
        if art:
            real += 1
            body = (
                f'<image href="{art}" x="0" y="0" width="{tile_w:.1f}" height="{tile_h}" '
                f'preserveAspectRatio="xMidYMid slice" clip-path="url(#{clip})"/>'
                f'<rect width="{tile_w:.1f}" height="{tile_h}" rx="11" fill="url(#sh{i})"/>'
            )
        else:
            defs.append(
                f'<linearGradient id="gg{i}" x1="0" y1="0" x2="1" y2="1">'
                f'<stop offset="0" stop-color="{g["cb"]}"/><stop offset="1" stop-color="{t.BG_DARK}"/></linearGradient>'
            )
            body = f'<rect width="{tile_w:.1f}" height="{tile_h}" rx="11" fill="url(#gg{i})"/>'
        tiles.append(f"""
      <g transform="translate({x:.1f},{y})">
        {body}
        <rect width="{tile_w:.1f}" height="{tile_h}" rx="11" fill="none" stroke="{ca}" stroke-width="1.3" opacity="0.85"/>
        <rect x="0" y="0" width="4" height="{tile_h}" rx="2" fill="{ca}" filter="url(#glow)"/>
        <text x="14" y="{tile_h-14}" font-family="{t.MONO}" font-size="13" font-weight="800" fill="#fff" letter-spacing="0.5">{t.esc(g["short"])}</text>
        <text x="14" y="{tile_h-2}" font-family="{t.SANS}" font-size="9.5" fill="{ca}">{t.esc(g["title"])}</text>
      </g>""")

    badge = f"{real}/{len(c.GAMES)} cover art" if real else f"{len(c.GAMES)} titles"
    inner = f'<defs>{"".join(defs)}</defs>{"".join(tiles)}'
    return t.card(w, int(h), "~/now-playing", inner, accent=t.TEAL, badge=badge)
