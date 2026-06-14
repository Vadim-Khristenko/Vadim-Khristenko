# -*- coding: utf-8 -*-
"""Header — the hero terminal pane: name, rotating aliases, Rust-flavoured tagline."""

from .. import theme as t
from .. import content as c

ASSET = "header.svg"


def _alias_rotator(x, y):
    """Cycle the aliases with crossfades. alias[0] is fully visible at rest (t=0)
    so the line never renders blank in a static first frame."""
    aliases = c.ALIASES
    n = len(aliases)
    loop = n * 2.4
    cf = 0.03
    nodes = []
    for i, alias in enumerate(aliases):
        s = i / n
        e = (i + 1) / n
        if i == 0:
            # visible at the very start AND wrap-around end of the loop
            kt = [0.0, e - cf, e, 1 - cf, 1.0]
            kv = [1, 1, 0, 0, 1]
        else:
            kt = [0.0, s - cf, s, e - cf, e, 1.0]
            kv = [0, 0, 1, 1, 0, 0]
        kt = [round(min(max(v, 0.0), 1.0), 4) for v in kt]
        nodes.append(
            f'<text x="{x}" y="{y}" font-family="{t.MONO}" font-size="27" font-weight="700" '
            f'fill="{t.ORANGE}" opacity="{kv[0]}" filter="url(#glow)">{t.esc(alias)}'
            f'<animate attributeName="opacity" values="{";".join(map(str, kv))}" '
            f'keyTimes="{";".join(map(str, kt))}" dur="{loop}s" repeatCount="indefinite"/></text>'
        )
    return "".join(nodes)


def build(ctx):
    w, h = t.CARD_W, 300

    def chip(y, dot, label):
        return (
            f'<circle cx="724" cy="{y - 5}" r="4" fill="{dot}"><animate attributeName="opacity" '
            f'values="0.4;1;0.4" dur="2.4s" repeatCount="indefinite"/></circle>'
            f'<text x="740" y="{y}" font-family="{t.MONO}" font-size="14" fill="{t.FG_DIM}">{t.esc(label)}</text>'
        )

    inner = f"""
    <text x="{t.MARGIN}" y="78" font-family="{t.MONO}" font-size="44" font-weight="800" fill="{t.FG}" filter="url(#softglow)">Vadim Khristenko</text>
    <rect x="{t.MARGIN+2}" y="96" width="0" height="3" rx="1.5" fill="{t.RUST}">
      <animate attributeName="width" values="0;340" dur="1.2s" begin="0.2s" fill="freeze"/>
    </rect>
    <text x="{t.MARGIN}" y="143" font-family="{t.MONO}" font-size="18" fill="{t.COMMENT}">aka&gt;</text>
    <g transform="translate(64,0)">{_alias_rotator(t.MARGIN, 143)}</g>
    <rect x="328" y="127" width="11" height="20" fill="{t.ORANGE}" opacity="0.9">
      <animate attributeName="opacity" values="1;0;1" dur="1.05s" repeatCount="indefinite"/>
    </rect>
    <text x="{t.MARGIN}" y="185" font-family="{t.MONO}" font-size="15" fill="{t.COMMENT}">// backend architect · systems engineer · chaos engineer by hobby</text>
    <text x="{t.MARGIN}" y="217" font-family="{t.MONO}" font-size="15" fill="{t.PURPLE}">let</text>
    <text x="{t.MARGIN+34}" y="217" font-family="{t.MONO}" font-size="15" fill="{t.FG}">primary</text>
    <text x="{t.MARGIN+112}" y="217" font-family="{t.MONO}" font-size="15" fill="{t.COMMENT}">=</text>
    <text x="{t.MARGIN+130}" y="217" font-family="{t.MONO}" font-size="15" fill="{t.CYAN}">Language</text>
    <text x="{t.MARGIN+208}" y="217" font-family="{t.MONO}" font-size="15" fill="{t.COMMENT}">::</text>
    <text x="{t.MARGIN+226}" y="217" font-family="{t.MONO}" font-size="15" fill="{t.ORANGE}">Rust</text>
    <text x="{t.MARGIN+265}" y="217" font-family="{t.MONO}" font-size="15" fill="{t.FG}">;</text>
    <text x="{t.MARGIN+282}" y="217" font-family="{t.MONO}" font-size="15">🦀</text>
    <text x="724" y="56" font-family="{t.MONO}" font-size="12" fill="{t.COMMENT}" letter-spacing="2">// KNOWN FOR</text>
    {chip(86, t.RED, "The Wall Dev")}
    {chip(116, t.ORANGE, "AmneziaWG Architect")}
    {chip(146, t.PURPLE, "Aethelgard TQ-1.58")}
    {chip(176, t.GREEN, "production bots & fleets")}
    <line x1="704" y1="40" x2="704" y2="196" stroke="{t.BG_HL}" stroke-width="1"/>
    """
    return t.card(w, h, "vadim@vai-rice:~$ whoami", inner, accent=t.RUST, badge="online")
