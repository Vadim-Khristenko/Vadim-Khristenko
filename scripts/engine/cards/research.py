# -*- coding: utf-8 -*-
"""Active research — Aethelgard TQ-1.58 HVRL, with flipping ternary weight cells."""

from .. import theme as t
from .. import content as c

ASSET = "research.svg"


def _weight_grid(x, y, cols, rows):
    """A lattice of {-1, 0, +1} cells that flicker — a live 1.58-bit weight tile."""
    cell = 22
    states = ["−1", "0", "+1"]
    colors = {"−1": t.RED, "0": t.COMMENT, "+1": t.GREEN}
    out = []
    k = 0
    for r in range(rows):
        for col in range(cols):
            cx = x + col * cell
            cy = y + r * cell
            s0 = states[(r + col) % 3]
            s1 = states[(r + col + 1) % 3]
            s2 = states[(r + col + 2) % 3]
            dur = 3 + (k % 5) * 0.7
            begin = (k % 8) * 0.3
            out.append(
                f'<g transform="translate({cx},{cy})">'
                f'<rect x="1" y="-13" width="{cell-3}" height="{cell-3}" rx="3" fill="{t.BG_DARK}" stroke="{t.BG_HL}" stroke-width="0.6"/>'
                f'<text x="{(cell-2)/2}" y="2" text-anchor="middle" font-family="{t.MONO}" font-size="10" fill="{colors[s0]}">'
                f'{s0}'
                f'<animate attributeName="fill" values="{colors[s0]};{colors[s1]};{colors[s2]};{colors[s0]}" dur="{dur}s" begin="{begin}s" repeatCount="indefinite"/>'
                f'</text></g>'
            )
            k += 1
    return "".join(out)


def build(ctx):
    w, h = t.WIDE, 232
    r = c.RESEARCH
    tags = ["verifiable training", "hierarchical RL", "domain: software-engineering", "ternary {-1,0,+1}"]
    chips = []
    cx = 40
    cy = 168
    for tag in tags:
        tw = 14 + len(tag) * 7.4
        chips.append(
            f'<rect x="{cx}" y="{cy-16}" width="{tw:.0f}" height="24" rx="12" fill="{t.BG_HL}" stroke="{t.PURPLE}" stroke-width="0.8" opacity="0.9"/>'
            f'<text x="{cx+tw/2:.0f}" y="{cy}" text-anchor="middle" font-family="{t.MONO}" font-size="12" fill="{t.FG_DIM}">{t.esc(tag)}</text>'
        )
        cx += tw + 12

    # Manual two-line wrap so the prose never runs under the weight lattice (x≥800).
    line1 = "A low-bit agentic reasoning model with verifiable training,"
    line2 = "hierarchical RL & domain specialization for software engineering."
    inner = f"""
    <text x="40" y="44" font-family="{t.MONO}" font-size="12" fill="{t.RED}" letter-spacing="2">● ACTIVE RESEARCH · low-bit ML</text>
    <text x="40" y="84" font-family="{t.MONO}" font-size="29" font-weight="800" fill="{t.PURPLE}" filter="url(#softglow)">{t.esc(r['name'])}</text>
    <text x="40" y="108" font-family="{t.MONO}" font-size="14" fill="{t.CYAN}">{t.esc(r['subtitle'])}</text>
    <text x="40" y="132" font-family="{t.SANS}" font-size="13" fill="{t.FG_DIM}">{t.esc(line1)}</text>
    <text x="40" y="150" font-family="{t.SANS}" font-size="13" fill="{t.FG_DIM}">{t.esc(line2)}</text>
    {"".join(chips)}
    <text x="{w-245}" y="36" font-family="{t.MONO}" font-size="11" fill="{t.COMMENT}">weights ∈ {{-1, 0, +1}}</text>
    {_weight_grid(w-245, 56, 10, 5)}
    """
    return t.card(w, h, "~/research/aethelgard.rs", inner, accent=t.PURPLE, badge="TQ-1.58 HVRL", texture="ternary")
