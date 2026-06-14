# -*- coding: utf-8 -*-
"""Weekly vibe — game & track rotate by ISO week; FOCUS is real data: the repo
   where the most recent changes land. (The AI-companion block was removed.)"""

from .. import theme as t
from .. import content as c

ASSET = "vibe.svg"


def _pick(seq, seed, salt):
    return seq[(seed + salt) % len(seq)]


def build(ctx):
    seed = ctx.get("vibe_seed", ctx["seed"])  # rotates every 2 days
    d = ctx["data"]
    w, h = t.CARD_W, 150
    game = _pick(c.GAMES, seed, 1)
    game_title = game["title"]
    game_note = c.GAME_NOTES.get(game["key"], "")
    composer, comp_note = _pick(c.COMPOSERS, seed, 2)
    quote = _pick(c.QUOTES, seed, 4)
    focus_repo = d.most_active_repo or "—"

    def field(x, icon, col, label, value, note):
        return f"""
        <text x="{x}" y="40" font-family="{t.MONO}" font-size="12" fill="{col}" letter-spacing="1">{icon} {t.esc(label)}</text>
        <text x="{x}" y="68" font-family="{t.MONO}" font-size="18" font-weight="700" fill="{t.FG}">{t.esc(value)}</text>
        <text x="{x}" y="88" font-family="{t.SANS}" font-size="12" fill="{t.COMMENT}">{t.esc(note)}</text>"""

    inner = f"""
    {field(t.MARGIN, "🎮", t.RED, "PLAYING", game_title, game_note)}
    {field(360, "🎧", t.PURPLE, "ON LOOP", composer, comp_note)}
    {field(680, "🛠", t.CYAN, "FOCUS", focus_repo, "where most changes land")}
    <line x1="{t.MARGIN}" y1="104" x2="{w-t.MARGIN}" y2="104" stroke="{t.BG_HL}" stroke-width="1"/>
    <text x="{t.MARGIN}" y="128" font-family="{t.SANS}" font-size="13" font-style="italic" fill="{t.YELLOW}">“{t.esc(quote)}”</text>
    """
    badge = f"refreshed {ctx['stamp'][:10]}"
    return t.card(w, h, "~/now.vibe", inner, accent=t.MAGENTA, badge=badge)
