# -*- coding: utf-8 -*-
"""Unified telemetry dashboard — one full-width card combining GitHub stats,
commit activity and a stacked language bar. One width => clean mobile scaling."""

from .. import theme as t

ASSET = "dashboard.svg"

LANG_COLORS = {
    "Rust": t.RUST, "Python": "#7aa2f7", "C++": "#f7768e", "TypeScript": "#7dcfff",
    "JavaScript": "#e0af68", "Java": "#ff9e64", "Kotlin": "#bb9af7", "Cython": "#fedf5b",
    "HTML": "#f7768e", "CSS": "#7dcfff", "Vue": "#9ece6a", "Shell": "#9ece6a",
    "C": "#9aa5ce", "Go": "#2ac3de", "Dockerfile": "#7dcfff", "Makefile": "#565f89",
    "Svelte": "#ff5d2b", "SCSS": "#c6538c", "Jupyter Notebook": "#da5b0b",
}


def _fmt(n):
    if n is None:
        return "—"
    if n >= 1_000_000:
        return f"{n/1_000_000:.1f}M".replace(".0M", "M")
    if n >= 1000:
        return f"{n/1000:.1f}k".replace(".0k", "k")
    return str(n)


def _sparkline(daily, x, y, w, h):
    pts = [c for _, c in daily[-30:]] if daily else []
    if not pts:
        return f'<text x="{x}" y="{y+h}" font-family="{t.MONO}" font-size="11" fill="{t.COMMENT}">awaiting first authenticated run…</text>'
    import math
    # sqrt scaling so a single huge day doesn't flatten all the others
    peak = math.sqrt(max(pts) or 1)
    bw = w / len(pts)
    out = []
    for i, v in enumerate(pts):
        bh = max(2, (math.sqrt(v) / peak) * h) if v > 0 else 2
        bx = x + i * bw
        col = t.GREEN if v > 0 else t.BG_HL
        out.append(
            f'<rect x="{bx:.1f}" y="{y+h-bh:.1f}" width="{max(2,bw-2.5):.1f}" height="{bh:.1f}" rx="1.5" fill="{col}">'
            f'<animate attributeName="height" values="0;{bh:.1f}" dur="0.7s" begin="{0.02*i:.2f}s" fill="freeze"/>'
            f'<animate attributeName="y" values="{y+h};{y+h-bh:.1f}" dur="0.7s" begin="{0.02*i:.2f}s" fill="freeze"/></rect>'
        )
    return "".join(out)


def build(ctx):
    d = ctx["data"]
    w, h = t.CARD_W, 404
    M = t.MARGIN
    usable = w - 2 * M

    # ---- Zone A: telemetry numbers ----------------------------------------- #
    cells = [
        ("repositories", d.repo_count, t.RUST),
        ("total stars", d.stars, t.YELLOW),
        ("followers", d.profile.get("followers", 0), t.BLUE),
        ("following", d.profile.get("following", 0), t.PURPLE),
    ]
    zone_a = []
    for i, (label, val, col) in enumerate(cells):
        cxp = M + (i + 0.5) * (usable / 4)
        zone_a.append(f"""
        <text x="{cxp:.0f}" y="70" text-anchor="middle" font-family="{t.MONO}" font-size="40" font-weight="800"
              fill="{col}" filter="url(#glow)" opacity="0">{t.esc(_fmt(val))}
          <animate attributeName="opacity" values="0;1" dur="0.8s" begin="{0.12*i}s" fill="freeze"/></text>
        <text x="{cxp:.0f}" y="92" text-anchor="middle" font-family="{t.MONO}" font-size="12" fill="{t.COMMENT}" letter-spacing="1">{t.esc(label.upper())}</text>""")

    # ---- Zone B: commit activity ------------------------------------------- #
    lab = d.commit_label
    metrics = [(f"{lab}·7d", d.commits["7d"], t.GREEN),
               (f"{lab}·30d", d.commits["30d"], t.CYAN),
               (f"{lab}·1y", d.commits["1y"], t.ORANGE)]
    zone_b = [f'<text x="{M}" y="130" font-family="{t.MONO}" font-size="12" fill="{t.COMMENT}" letter-spacing="1">COMMIT ACTIVITY</text>']
    for i, (l, v, col) in enumerate(metrics):
        cxp = M + 70 + i * 130
        zone_b.append(f"""
        <text x="{cxp}" y="172" text-anchor="middle" font-family="{t.MONO}" font-size="30" font-weight="800" fill="{col}" filter="url(#glow)" opacity="0">{t.esc(_fmt(v))}
          <animate attributeName="opacity" values="0;1" dur="0.7s" begin="{0.12*i}s" fill="freeze"/></text>
        <text x="{cxp}" y="190" text-anchor="middle" font-family="{t.MONO}" font-size="11" fill="{t.COMMENT}">{t.esc(l)}</text>""")
    loc = f"≈{_fmt(d.loc)}" if d.loc else "—"
    chips = [("lines of code", loc, t.RUST), ("streak", f"{d.streak}🔥", t.RED), ("active days", f"{d.active_days}d", t.YELLOW)]
    for i, (l, v, col) in enumerate(chips):
        cxp = M + i * 145
        zone_b.append(f"""
        <text x="{cxp}" y="220" font-family="{t.MONO}" font-size="11" fill="{t.COMMENT}">{t.esc(l)}</text>
        <text x="{cxp}" y="240" font-family="{t.MONO}" font-size="17" font-weight="700" fill="{col}">{t.esc(v)}</text>""")
    zone_b.append(f'<text x="{w-M}" y="130" text-anchor="end" font-family="{t.MONO}" font-size="11" fill="{t.COMMENT}">last 30 days</text>')
    zone_b.append(_sparkline(d.daily, M + 470, 142, usable - 470, 96))

    # ---- Zone C: stacked language bar (top 6 + "other" so it sums to 100%) -- #
    all_items = sorted(d.lang_bytes.items(), key=lambda kv: kv[1], reverse=True)
    all_items = [x for x in all_items if x[0] == "Rust"] + [x for x in all_items if x[0] != "Rust"]
    grand = sum(v for _, v in all_items) or 1
    top = all_items[:6]
    other = sum(v for _, v in all_items[6:])
    items = top + ([("other", other)] if other > 0 else [])

    seg, lx = [], M
    for lang, val in items:
        seg_w = (val / grand) * usable
        col = t.COMMENT if lang == "other" else LANG_COLORS.get(lang, t.PURPLE)
        seg.append(f'<rect x="{lx:.1f}" y="292" width="{max(0,seg_w-1.5):.1f}" height="16" fill="{col}" rx="2"/>')
        lx += seg_w

    # Legend on two tidy rows of 4 so nothing runs off the right edge.
    legend = []
    per_row = 4
    col_w = usable / per_row
    for i, (lang, val) in enumerate(items):
        pct = val / grand * 100
        row, col_i = divmod(i, per_row)
        lx0 = M + col_i * col_w
        ly = 332 + row * 22
        c = t.COMMENT if lang == "other" else LANG_COLORS.get(lang, t.PURPLE)
        star = " 🦀" if lang == "Rust" else ""
        legend.append(f'<circle cx="{lx0+5:.0f}" cy="{ly-4}" r="4.5" fill="{c}"/>'
                      f'<text x="{lx0+16:.0f}" y="{ly}" font-family="{t.MONO}" font-size="11.5" fill="{t.FG_DIM}">{t.esc(lang)} {pct:.0f}%{star}</text>')
    zone_c = [f'<text x="{M}" y="280" font-family="{t.MONO}" font-size="12" fill="{t.COMMENT}" letter-spacing="1">LANGUAGES · by bytes</text>'] + seg + legend

    sep = (f'<line x1="{M}" y1="108" x2="{w-M}" y2="108" stroke="{t.BG_HL}" stroke-width="1"/>'
           f'<line x1="{M}" y1="258" x2="{w-M}" y2="258" stroke="{t.BG_HL}" stroke-width="1"/>')
    inner = "".join(zone_a) + sep + "".join(zone_b) + "".join(zone_c)
    return t.card(w, h, "~/telemetry.dash", inner, accent=t.RUST, badge="live · GitHub")
