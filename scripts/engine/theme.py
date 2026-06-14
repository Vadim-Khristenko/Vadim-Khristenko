# -*- coding: utf-8 -*-
"""
Design system for the VAI Profile Engine.
=========================================

Think of this module as the stylesheet + component library of a tiny website.
Every "card" is a window pane in a Tokyo-Night-themed IDE, framed with a single
clean animated neon border, a faint grid + ternary {-1,0,+1} texture (a nod to
1.58-bit research) and a tidy title bar. Rust orange is the signature accent.

COMPOSITION RULES (so everything stacks cleanly, especially on mobile):
  * Every card is exactly CARD_W wide. GitHub scales all images to the column
    width uniformly, so identical intrinsic widths => identical rendered widths.
  * No corner brackets — they collided with the title-bar dots and the border.
    The chrome is: gradient border + title bar (dots · title · badge) + content.
  * 24px content margin everywhere; 42px title bar everywhere.

Cards call `card(...)` for the chrome and only supply their inner body.
"""

from __future__ import annotations

import html

# --------------------------------------------------------------------------- #
# Palette — Tokyo Night (night variant) + Rust signature                      #
# --------------------------------------------------------------------------- #

BG = "#1a1b26"
BG_DARK = "#15161e"
BG_PANEL = "#1f2335"
BG_HL = "#292e42"
FG = "#c0caf5"
FG_DIM = "#a9b1d6"
COMMENT = "#565f89"
BLUE = "#7aa2f7"
CYAN = "#7dcfff"
PURPLE = "#bb9af7"
GREEN = "#9ece6a"
RED = "#f7768e"
ORANGE = "#ff9e64"
YELLOW = "#e0af68"
TEAL = "#2ac3de"
MAGENTA = "#c678dd"

# Rust — the primary accent.
RUST = "#dea584"
RUST_DEEP = "#e06c47"
RUST_BRAND = "#ce422b"

# One width to rule them all (uniform mobile scaling).
CARD_W = 1000
WIDE = CARD_W          # kept for back-compat with existing imports
MARGIN = 24
BAR_H = 42

# Font stacks. Web fonts don't load inside GitHub-proxied SVG, so we lean on
# system monospace (terminal/Rust vibe) with broad fallbacks; generic
# monospace/sans-serif guarantee a graceful last resort.
MONO = "'JetBrains Mono','Cascadia Code','Fira Code','SFMono-Regular','Consolas','Liberation Mono',monospace"
SANS = "'Segoe UI','Inter',system-ui,-apple-system,'Helvetica Neue',sans-serif"


def esc(text) -> str:
    return html.escape(str(text), quote=True)


# Embedding raster/animated media as data URIs keeps SVGs self-contained (no
# cross-origin fetches through GitHub's camo proxy). Animated WEBP/GIF/AVIF are
# embedded byte-for-byte so they keep animating inside the card.
import base64 as _b64
import os as _os

_MIME = {".jpg": "jpeg", ".jpeg": "jpeg", ".png": "png", ".webp": "webp",
         ".gif": "gif", ".avif": "avif", ".apng": "apng"}


def media_data_uri(path) -> str:
    ext = _os.path.splitext(path)[1].lower()
    mime = _MIME.get(ext, "png")
    with open(path, "rb") as f:
        return f"data:image/{mime};base64," + _b64.b64encode(f.read()).decode("ascii")


def find_media(directory, key):
    """Return the first existing media file for `key` (any supported ext)."""
    for ext in (".webp", ".gif", ".avif", ".apng", ".png", ".jpg", ".jpeg"):
        p = _os.path.join(directory, key + ext)
        if _os.path.exists(p):
            return p
    return None


# --------------------------------------------------------------------------- #
# Shared <defs>: gradients, glow, grid                                        #
# --------------------------------------------------------------------------- #

def _defs(accent: str) -> str:
    return f"""
  <defs>
    <linearGradient id="panel" x1="0" y1="0" x2="0.5" y2="1">
      <stop offset="0" stop-color="{BG_PANEL}"/>
      <stop offset="1" stop-color="{BG_DARK}"/>
    </linearGradient>
    <linearGradient id="border" x1="0" y1="0" x2="1" y2="1">
      <stop offset="0" stop-color="{accent}">
        <animate attributeName="stop-color" values="{accent};{BLUE};{PURPLE};{CYAN};{accent}" dur="12s" repeatCount="indefinite"/>
      </stop>
      <stop offset="0.5" stop-color="{PURPLE}">
        <animate attributeName="stop-color" values="{PURPLE};{CYAN};{accent};{BLUE};{PURPLE}" dur="12s" repeatCount="indefinite"/>
      </stop>
      <stop offset="1" stop-color="{BLUE}">
        <animate attributeName="stop-color" values="{BLUE};{accent};{CYAN};{PURPLE};{BLUE}" dur="12s" repeatCount="indefinite"/>
      </stop>
    </linearGradient>
    <linearGradient id="accentbar" x1="0" y1="0" x2="1" y2="0">
      <stop offset="0" stop-color="{accent}"/>
      <stop offset="1" stop-color="{accent}" stop-opacity="0"/>
    </linearGradient>
    <filter id="glow" x="-40%" y="-40%" width="180%" height="180%">
      <feGaussianBlur stdDeviation="1.7" result="b"/>
      <feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
    </filter>
    <filter id="softglow" x="-60%" y="-60%" width="220%" height="220%">
      <feGaussianBlur stdDeviation="4.5" result="b"/>
      <feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
    </filter>
    <pattern id="grid" width="28" height="28" patternUnits="userSpaceOnUse">
      <path d="M28 0 L0 0 0 28" fill="none" stroke="{COMMENT}" stroke-width="0.5" opacity="0.08"/>
    </pattern>
  </defs>"""


def _ternary_texture(w: int, h: int, dense: bool = False) -> str:
    """Scattered ternary glyphs {-1, 0, +1} — the 1.58-bit signature motif."""
    glyphs = ["−1", "0", "+1", "−1", "0", "+1", "1", "0"]
    step_x = 58 if dense else 130
    step_y = 30 if dense else 76
    out = []
    i = 0
    y = BAR_H + 22
    while y < h - 8:
        x = 24
        while x < w - 18:
            g = glyphs[i % len(glyphs)]
            op = 0.06 if dense else 0.045
            flick = ""
            if dense and i % 5 == 0:
                flick = (f'<animate attributeName="opacity" values="{op};0.30;{op}" '
                         f'dur="{2 + i % 4}s" begin="{(i % 7) * 0.4}s" repeatCount="indefinite"/>')
            out.append(f'<text x="{x}" y="{y}" font-family="{MONO}" font-size="11" fill="{PURPLE}" opacity="{op}">{g}{flick}</text>')
            x += step_x
            i += 1
        y += step_y
    return "".join(out)


# --------------------------------------------------------------------------- #
# The window-pane chrome shared by every card                                 #
# --------------------------------------------------------------------------- #

def card(
    w: int,
    h: int,
    title: str,
    inner: str,
    *,
    accent: str = RUST,
    badge: str = "",
    texture: str = "grid",
) -> str:
    """
    Compose a complete standalone SVG card with shared chrome:
    panel + grid/ternary texture + animated neon border + title bar
    (traffic-light dots · monospace title · optional right badge). `inner`
    is placed in a group translated below the title bar.
    """
    dense = (texture == "ternary")
    if texture == "plain":
        tex = ""
    else:
        tex = (f'<rect x="6" y="6" width="{w - 12}" height="{h - 12}" rx="13" fill="url(#grid)"/>'
               + _ternary_texture(w, h, dense=dense))

    dots = "".join(
        f'<circle cx="{MARGIN + i * 20}" cy="{BAR_H // 2}" r="5.5" fill="{c}" opacity="0.92"/>'
        for i, c in enumerate((RED, YELLOW, GREEN))
    )
    title_x = MARGIN + 3 * 20 + 12
    badge_node = ""
    if badge:
        badge_node = (
            f'<circle cx="{w - MARGIN - 1}" cy="{BAR_H // 2}" r="3.5" fill="{accent}">'
            f'<animate attributeName="opacity" values="0.4;1;0.4" dur="2.2s" repeatCount="indefinite"/></circle>'
            f'<text x="{w - MARGIN - 12}" y="{BAR_H // 2 + 4}" text-anchor="end" font-family="{MONO}" '
            f'font-size="12" fill="{COMMENT}">{esc(badge)}</text>'
        )

    return f"""<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}" role="img" font-family="{SANS}">
  {_defs(accent)}
  <rect x="2" y="2" width="{w - 4}" height="{h - 4}" rx="15" fill="url(#panel)"/>
  {tex}
  <rect x="2.5" y="2.5" width="{w - 5}" height="{h - 5}" rx="14" fill="none" stroke="url(#border)" stroke-width="2"/>
  <rect x="3.5" y="3.5" width="{w - 7}" height="{h - 7}" rx="13" fill="none" stroke="{BG_HL}" stroke-width="1" opacity="0.6"/>
  <path d="M2 18 Q2 2 18 2" fill="none" stroke="{accent}" stroke-width="2.4" opacity="0.9"/>
  {dots}
  <text x="{title_x}" y="{BAR_H // 2 + 4}" font-family="{MONO}" font-size="13" fill="{FG_DIM}">{esc(title)}</text>
  {badge_node}
  <rect x="{MARGIN}" y="{BAR_H}" width="{w - 2 * MARGIN}" height="2" fill="url(#accentbar)"/>
  <g transform="translate(0,{BAR_H})">
    {inner}
  </g>
</svg>"""
