# -*- coding: utf-8 -*-
"""Best game — a configurable spotlight (content.BEST_GAME).

Two cover modes: "portrait" (cover as a 2:3 panel on the left) and "landscape"
(cover full-bleed behind the text). Up to 5 favourite characters are laid out
automatically. Cover + character art may be remote URLs or local files under
assets/bestgame/, including animated WEBP/GIF (embedded as-is); missing character
art falls back to a stylised initials avatar."""

import os

from .. import theme as t
from .. import content as c

ASSET = "bestgame.svg"
_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
ART_DIR = os.path.join(_ROOT, "assets", "bestgame")


def _media(key):
    p = t.find_media(ART_DIR, key)
    return t.media_data_uri(p) if p else None


def _char_key(ch):
    return "char_" + "".join(x for x in ch["short"].lower() if x.isalnum())


def _chip(x, y, label, value, accent):
    lw, vw = len(label) * 7, len(str(value)) * 9
    w = 18 + lw + 8 + vw
    return (
        f'<g transform="translate({x},{y})">'
        f'<rect x="0" y="-17" width="{w}" height="25" rx="12.5" fill="#0c0a12" fill-opacity="0.55" stroke="{accent}" stroke-width="0.9"/>'
        f'<text x="11" y="0" font-family="{t.MONO}" font-size="11" fill="{t.COMMENT}">{t.esc(label)}</text>'
        f'<text x="{11+lw+6}" y="0" font-family="{t.MONO}" font-size="12.5" font-weight="700" fill="{t.FG}">{t.esc(value)}</text>'
        f'</g>'
    ), w


def _avatar(ch, cx, cy, r):
    key = _char_key(ch)
    accent = ch.get("accent", t.RUST)
    media = _media(key)
    cid = "clip_" + key
    ring = (
        f'<circle cx="{cx}" cy="{cy}" r="{r+3}" fill="none" stroke="{accent}" stroke-width="2" opacity="0.9"/>'
        f'<circle cx="{cx}" cy="{cy}" r="{r+3}" fill="none" stroke="{accent}" stroke-width="2" opacity="0.3" filter="url(#softglow)"/>'
    )
    if media:
        inner = (
            f'<clipPath id="{cid}"><circle cx="{cx}" cy="{cy}" r="{r}"/></clipPath>'
            f'<circle cx="{cx}" cy="{cy}" r="{r}" fill="{t.BG_HL}"/>'
            f'<image href="{media}" x="{cx-r}" y="{cy-r}" width="{2*r}" height="{2*r}" '
            f'preserveAspectRatio="xMidYMid slice" clip-path="url(#{cid})"/>'
        )
    else:
        initials = "".join(word[0] for word in ch["short"].split()[:2]).upper()
        inner = (
            f'<circle cx="{cx}" cy="{cy}" r="{r}" fill="{t.BG_HL}"/>'
            f'<text x="{cx}" y="{cy+7}" text-anchor="middle" font-family="{t.MONO}" font-size="{int(r*0.62)}" '
            f'font-weight="800" fill="{accent}">{t.esc(initials)}</text>'
        )
    name = ch.get("short", ch["name"])
    label = (f'<text x="{cx}" y="{cy+r+18}" text-anchor="middle" font-family="{t.MONO}" '
             f'font-size="11" fill="{t.FG_DIM}">{t.esc(name)}</text>')
    star = f'<text x="{cx+r-3}" y="{cy-r+5}" font-family="{t.SANS}" font-size="12">⭐</text>'
    return inner + ring + star + label


def _squad(x0, total_w, cy, chars, accent, label_x, label_y):
    n = max(1, len(chars))
    slot = total_w / n
    r = int(max(20, min(30, slot / 2 - 16)))
    out = [f'<text x="{label_x}" y="{label_y}" font-family="{t.MONO}" font-size="12" fill="{accent}" letter-spacing="2">★ FAVOURITE SQUAD</text>']
    for i, ch in enumerate(chars):
        cx = x0 + slot * (i + 0.5)
        out.append(_avatar(ch, round(cx), cy, r))
    return "".join(out)


def _chips(x, y, g, accent):
    data = [("LV", g.get("level")), ("SERVER", g.get("server")), ("ID", g.get("game_id"))]
    if g.get("nick"):
        data.insert(0, ("NICK", g["nick"]))
    out, cx = [], x
    for label, value in data:
        if not value:
            continue
        node, cw = _chip(cx, y, label, value, accent)
        out.append(node)
        cx += cw + 10
    return "".join(out)


def build(ctx):
    g = c.BEST_GAME
    w, h = t.CARD_W, 364
    accent = g.get("accent", t.RUST)
    inner_h = h - t.BAR_H
    mode = g.get("cover_mode", "portrait")
    chars = g.get("characters", [])[:5]
    cover = _media("cover")
    M = t.MARGIN

    defs = (
        f'<linearGradient id="cover_sheen" x1="0" y1="0" x2="0" y2="1">'
        f'<stop offset="0.55" stop-color="#0a0810" stop-opacity="0"/>'
        f'<stop offset="1" stop-color="#0a0810" stop-opacity="0.6"/></linearGradient>'
        f'<linearGradient id="land_scrim" x1="0" y1="0" x2="1" y2="0.2">'
        f'<stop offset="0" stop-color="#0a0810" stop-opacity="0.95"/>'
        f'<stop offset="0.6" stop-color="#0a0810" stop-opacity="0.72"/>'
        f'<stop offset="1" stop-color="#0a0810" stop-opacity="0.42"/></linearGradient>'
        f'<linearGradient id="land_bottom" x1="0" y1="0" x2="0" y2="1">'
        f'<stop offset="0.45" stop-color="#0a0810" stop-opacity="0"/>'
        f'<stop offset="1" stop-color="#0a0810" stop-opacity="0.92"/></linearGradient>'
    )

    if mode == "landscape":
        defs += f'<clipPath id="bgclip"><rect width="{w}" height="{inner_h}"/></clipPath>'
        if cover:
            bg = (f'<image href="{cover}" x="0" y="0" width="{w}" height="{inner_h}" '
                  f'preserveAspectRatio="xMidYMid slice" clip-path="url(#bgclip)"/>'
                  f'<rect width="{w}" height="{inner_h}" fill="url(#land_scrim)"/>'
                  f'<rect width="{w}" height="{inner_h}" fill="url(#land_bottom)"/>')
        else:
            bg = f'<rect width="{w}" height="{inner_h}" fill="url(#land_bottom)"/>'
        tx = M
        squad = _squad(M, w - 2 * M, inner_h - 58, chars, accent, M, inner_h - 106)
        body = f"""
        {bg}
        <text x="{tx}" y="44" font-family="{t.MONO}" font-size="12" fill="{accent}" letter-spacing="3">★ BEST GAME · my pick</text>
        <text x="{tx}" y="92" font-family="{t.MONO}" font-size="46" font-weight="800" fill="{t.FG}" filter="url(#softglow)">{t.esc(g['title'])}</text>
        <text x="{tx}" y="116" font-family="{t.MONO}" font-size="13" fill="{t.FG_DIM}" letter-spacing="2">{t.esc(g.get('subtitle',''))}</text>
        {_chips(tx, 150, g, accent)}
        <text x="{tx}" y="182" font-family="{t.SANS}" font-size="13" fill="{t.FG_DIM}">{t.esc(g.get('blurb',''))}</text>
        {squad}"""
    else:  # portrait
        pad = 14
        cw = 180
        chh = inner_h - 2 * pad
        cx0, cy0 = M, pad
        defs += f'<clipPath id="coverclip"><rect x="{cx0}" y="{cy0}" width="{cw}" height="{chh}" rx="12"/></clipPath>'
        if cover:
            panel = (f'<image href="{cover}" x="{cx0}" y="{cy0}" width="{cw}" height="{chh}" '
                     f'preserveAspectRatio="xMidYMid slice" clip-path="url(#coverclip)"/>'
                     f'<rect x="{cx0}" y="{cy0}" width="{cw}" height="{chh}" rx="12" fill="url(#cover_sheen)"/>')
        else:
            panel = (f'<rect x="{cx0}" y="{cy0}" width="{cw}" height="{chh}" rx="12" fill="{t.BG_HL}"/>'
                     f'<text x="{cx0+cw//2}" y="{cy0+chh//2}" text-anchor="middle" font-family="{t.MONO}" font-size="13" fill="{t.COMMENT}">no cover</text>')
        panel += f'<rect x="{cx0}" y="{cy0}" width="{cw}" height="{chh}" rx="12" fill="none" stroke="{accent}" stroke-width="1.5" opacity="0.85"/>'
        rx = M + cw + 28
        avail = w - rx - M
        squad = _squad(rx, avail, 268, chars, accent, rx, 212)
        body = f"""
        {panel}
        <text x="{rx}" y="44" font-family="{t.MONO}" font-size="12" fill="{accent}" letter-spacing="3">★ BEST GAME · my pick</text>
        <text x="{rx}" y="92" font-family="{t.MONO}" font-size="44" font-weight="800" fill="{t.FG}" filter="url(#softglow)">{t.esc(g['title'])}</text>
        <text x="{rx}" y="116" font-family="{t.MONO}" font-size="13" fill="{t.FG_DIM}" letter-spacing="2">{t.esc(g.get('subtitle',''))}</text>
        {_chips(rx, 152, g, accent)}
        <text x="{rx}" y="184" font-family="{t.SANS}" font-size="13" fill="{t.FG_DIM}">{t.esc(g.get('blurb',''))}</text>
        {squad}"""

    inner = f"<defs>{defs}</defs>{body}"
    return t.card(w, h, "~/best-game.cfg", inner, accent=accent, badge=g.get("title", ""), texture="plain")
