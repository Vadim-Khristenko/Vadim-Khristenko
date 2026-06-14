# -*- coding: utf-8 -*-
"""A unique section divider — a neon data conduit: ternary bits stream across a
glowing trace into a pulsing hexagonal node at the centre."""

from .. import theme as t

ASSET = "divider.svg"


def _hexagon(cx, cy, r):
    pts = []
    import math
    for k in range(6):
        ang = math.radians(60 * k - 30)
        pts.append(f"{cx + r * math.cos(ang):.1f},{cy + r * math.sin(ang):.1f}")
    return " ".join(pts)


def build(ctx):
    w, h = t.CARD_W, 58
    mid = h // 2
    cx = w // 2

    # Streaming ternary bits drifting toward the node from both sides.
    bits = []
    glyphs = ["+1", "0", "−1", "1", "0", "+1", "−1", "0"]
    for i in range(10):
        gx = 90 + i * 70
        side = -1 if gx < cx else 1
        col = t.GREEN if glyphs[i % len(glyphs)].startswith("+") else (t.RED if glyphs[i % len(glyphs)].startswith("−") else t.COMMENT)
        bits.append(
            f'<text x="{gx}" y="{mid+4}" font-family="{t.MONO}" font-size="11" fill="{col}" opacity="0.0" text-anchor="middle">{glyphs[i % len(glyphs)]}'
            f'<animate attributeName="opacity" values="0;0.85;0" dur="3.2s" begin="{i*0.32:.2f}s" repeatCount="indefinite"/>'
            f'<animate attributeName="x" values="{gx};{gx + side*22}" dur="3.2s" begin="{i*0.32:.2f}s" repeatCount="indefinite"/></text>'
        )

    return f"""<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}" role="img">
  <defs>
    <linearGradient id="trace" x1="0" y1="0" x2="1" y2="0">
      <stop offset="0" stop-color="{t.BG}" stop-opacity="0"/>
      <stop offset="0.2" stop-color="{t.RUST}"/>
      <stop offset="0.5" stop-color="{t.PURPLE}"/>
      <stop offset="0.8" stop-color="{t.BLUE}"/>
      <stop offset="1" stop-color="{t.BG}" stop-opacity="0"/>
    </linearGradient>
    <filter id="dg" x="-30%" y="-300%" width="160%" height="700%">
      <feGaussianBlur stdDeviation="1.5" result="b"/><feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
    </filter>
  </defs>
  <line x1="70" y1="{mid}" x2="{w-70}" y2="{mid}" stroke="url(#trace)" stroke-width="1.4" filter="url(#dg)"/>
  <line x1="70" y1="{mid}" x2="{w-70}" y2="{mid}" stroke="{t.PURPLE}" stroke-width="1.4" stroke-dasharray="2 10" opacity="0.5">
    <animate attributeName="stroke-dashoffset" values="0;-48" dur="2.4s" repeatCount="indefinite"/>
  </line>
  {''.join(bits)}
  <circle cx="70" cy="{mid}" r="3" fill="{t.CYAN}" filter="url(#dg)">
    <animate attributeName="cx" values="70;{cx-40}" dur="2.6s" repeatCount="indefinite"/>
    <animate attributeName="opacity" values="0;1;0" dur="2.6s" repeatCount="indefinite"/>
  </circle>
  <circle cx="{w-70}" cy="{mid}" r="3" fill="{t.RUST}" filter="url(#dg)">
    <animate attributeName="cx" values="{w-70};{cx+40}" dur="2.6s" begin="1.3s" repeatCount="indefinite"/>
    <animate attributeName="opacity" values="0;1;0" dur="2.6s" begin="1.3s" repeatCount="indefinite"/>
  </circle>
  <g transform="translate({cx},{mid})">
    <polygon points="{_hexagon(0,0,15)}" fill="none" stroke="{t.PURPLE}" stroke-width="1" opacity="0.5">
      <animateTransform attributeName="transform" type="rotate" from="0" to="360" dur="14s" repeatCount="indefinite"/>
    </polygon>
    <polygon points="{_hexagon(0,0,9)}" fill="{t.BG_DARK}" stroke="{t.RUST}" stroke-width="1.6" filter="url(#dg)"/>
    <circle r="2.4" fill="{t.CYAN}"><animate attributeName="r" values="1.6;3;1.6" dur="2s" repeatCount="indefinite"/>
      <animate attributeName="opacity" values="0.5;1;0.5" dur="2s" repeatCount="indefinite"/></circle>
  </g>
</svg>"""
