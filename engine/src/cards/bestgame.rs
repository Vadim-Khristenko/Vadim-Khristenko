//! Best game as the NIKKE lobby. A full-body Nikke stands in the scene like
//! the lobby character, the HUD carries the avatar ring, resources and quest
//! pills, the campaign squad sits on a glass panel with level and power, every
//! loved Nikke gets a hex on the shelf, and the bottom row repeats the lobby's
//! Outpost / Ark / Union panels and the nav bar. Data: `[best_game]`.
//!
//! Art comes from `vai-profile bestgame` (assets/bestgame); anything missing
//! degrades to an empty plate instead of failing the card.

use super::kit;
use crate::config::profile::Character;
use crate::img::{self, Embedded};
use crate::run::Ctx;
use crate::svg::find_media;
use crate::typo::{self, Doc, Font, T};
use anyhow::Result;
use std::fmt::Write;
use std::path::Path;

const H: f64 = 700.0;
const NAVY: &str = "#0d1424";
const NAVY2: &str = "#060912";
const GLASS: &str = "#0b111d";
const WHITE: &str = "#f4f7fb";
const DIM: &str = "#8e9ab3";
const BLUE: &str = "#1e9bff";
const PURPLE: &str = "#8b5cf6";
const PLATE: &str = "#1a2232";

fn art(dir: &Path, key: &str, max_w: u32, flatten: bool) -> Option<Embedded> {
    let path = find_media(dir, key)?;
    img::load(&path, max_w, flatten.then(|| img::rgb(PLATE))).ok()
}

fn find<'a>(chars: impl Iterator<Item = &'a Character>, short: &str) -> Option<&'a Character> {
    chars
        .into_iter()
        .find(|c| c.short.eq_ignore_ascii_case(short))
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let g = &ctx.cfg.profile.best_game;
    let lb = &g.lobby;
    let dir = ctx.assets.join("bestgame");
    let icons = dir.join("icons");
    let mut doc = Doc::new();
    let mut b = String::new();
    let w = kit::W;
    let sb = |size: f64| T::new(Font::SairaBold, size);
    let sx = |size: f64| T::new(Font::SairaBlack, size);

    let _ = write!(b, r#"<rect width="{w}" height="{H}" fill="url(#scene)"/>"#);
    for (x, y, r, o, d) in [
        (120, 140, 40, 0.10, 0),
        (330, 90, 22, 0.14, 2),
        (80, 420, 30, 0.08, 4),
        (380, 380, 56, 0.07, 1),
        (250, 520, 18, 0.12, 3),
    ] {
        let _ = write!(
            b,
            r#"<circle class="bokeh" style="animation-delay:{d}s" cx="{x}" cy="{y}" r="{r}" fill="{BLUE}" opacity="{o}"/>"#
        );
    }
    let _ = write!(
        b,
        r##"<g class="streak"><rect x="0" y="0" width="140" height="{H}" fill="url(#beam)" transform="skewX(-20)"/></g><ellipse cx="230" cy="360" rx="230" ry="260" fill="url(#rim)"/><ellipse cx="230" cy="632" rx="190" ry="16" fill="#000" opacity=".45"/>"##
    );
    // lobby Nikke, standing on the floor line
    if let Some(e) = art(&dir, "lobby", 480, false) {
        let ah = 560.0_f64.min(560.0 * e.h as f64 / e.w as f64 * 0.73);
        let aw = ah * e.w as f64 / e.h as f64;
        let ax = 230.0 - aw / 2.0;
        let _ = write!(
            b,
            r#"<g class="breathe">{}</g>"#,
            e.tag(
                "mLobby",
                ax.round(),
                (634.0 - ah).round(),
                aw.round(),
                ah.round(),
                ""
            )
        );
    }
    let _ = write!(
        b,
        r#"<rect x="0" y="452" width="470" height="200" fill="url(#fadeB)"/>"#
    );

    // HUD: avatar ring, level, nick
    let _ = write!(
        b,
        r##"<circle cx="54" cy="52" r="31" fill="{GLASS}" stroke="#ffffff" stroke-opacity=".2"/>"##
    );
    if let Some(av) = find(g.characters.iter().chain(&g.loved), &lb.avatar)
        .and_then(|c| art(&dir, &c.key(), 112, true))
    {
        let _ = write!(
            b,
            r#"<g clip-path="url(#av)">{}</g>"#,
            av.tag("mAv", 27.0, 25.0, 54.0, 54.0, "")
        );
    }
    let _ = write!(
        b,
        r#"<circle class="ring" cx="54" cy="52" r="30" fill="none" stroke="{BLUE}" stroke-width="3" stroke-dasharray="150 188" stroke-linecap="round" transform="rotate(-90 54 52)"/><rect x="24" y="78" width="60" height="16" rx="3" fill="{GLASS}"/>"#
    );
    b.push_str(&doc.text(
        54.0,
        91.0,
        &format!("LV.{}", g.level),
        &sx(12.0).fill(WHITE).mid(),
    ));
    b.push_str(&doc.text(54.0, 114.0, &g.nick, &sx(17.0).fill(WHITE).mid()));

    // HUD: resources
    let _ = write!(
        b,
        r##"<rect x="102" y="30" width="236" height="36" rx="10" fill="{GLASS}" fill-opacity=".85" stroke="#ffffff" stroke-opacity=".12"/>"##
    );
    if let Some(e) = art(&icons, "gem", 52, false) {
        b.push_str(&e.tag("mGem", 110.0, 33.0, 26.0, 28.0, ""));
    }
    b.push_str(&doc.text(142.0, 55.0, &kit::group(lb.gems), &sb(18.0).fill(WHITE)));
    if let Some(e) = art(&icons, "credit", 64, false) {
        b.push_str(&e.tag("mCredit", 216.0, 38.0, 32.0, 20.0, ""));
    }
    b.push_str(&doc.text(
        326.0,
        55.0,
        &kit::group(lb.credits),
        &sb(18.0).fill(WHITE).end(),
    ));

    // HUD: quest pills
    let mut pill = |doc: &mut Doc, y: f64, col: &str, label: &str| {
        let _ = write!(
            b,
            r##"<rect x="560" y="{y}" width="420" height="34" rx="6" fill="{GLASS}" fill-opacity=".8" stroke="#ffffff" stroke-opacity=".14"/><circle class="ping" cx="582" cy="{c}" r="13" fill="none" stroke="{col}" stroke-width="2"/><circle cx="582" cy="{c}" r="13" fill="{col}"/>"##,
            c = y + 17.0
        );
        b.push_str(&doc.text(582.0, y + 23.0, "!", &sx(17.0).fill(WHITE).mid()));
        b.push_str(&doc.text(604.0, y + 23.0, label, &sb(15.0).fill(WHITE)));
        let _ = write!(
            b,
            r#"<rect x="938" y="{a}" width="36" height="24" rx="4" fill="{col}"/><path d="M948 {c} h12 m-5 -6 l6 6 l-6 6" fill="none" stroke="{WHITE}" stroke-width="2.4" stroke-linecap="round"/>"#,
            a = y + 5.0,
            c = y + 17.0
        );
    };
    if !lb.next_stage.is_empty() {
        pill(
            &mut doc,
            24.0,
            BLUE,
            &format!("Clear Stage {}", lb.next_stage),
        );
    }
    pill(
        &mut doc,
        66.0,
        PURPLE,
        &format!("Campaign: normal {} · hard {}", lb.normal, lb.hard),
    );

    // squad panel
    let (px, py, pw) = (448.0, 116.0, 532.0);
    let _ = write!(
        b,
        r##"<path d="M{px} {py} H{r} V{a} H{c} L{px} {d}z" fill="{GLASS}" fill-opacity=".86" stroke="#ffffff" stroke-opacity=".14"/><path d="M{px} {py} H{e} L{f} {g2} H{px}z" fill="url(#bluep)"/>"##,
        r = px + pw,
        a = py + 232.0,
        c = px + 14.0,
        d = py + 218.0,
        e = px + 210.0,
        f = px + 196.0,
        g2 = py + 28.0
    );
    b.push_str(&doc.text(
        px + 14.0,
        py + 20.0,
        "CAMPAIGN SQUAD",
        &sx(15.0).fill(WHITE).ls(1.5),
    ));
    let dim_t = sb(13.0).fill(DIM).ls(1.0);
    let hi_t = sb(13.0).fill(WHITE).ls(1.0);
    let hdr = format!(" · CLEARED {}", lb.cleared_at);
    let runs: Vec<(&str, &T)> = vec![
        ("NORMAL ", &dim_t),
        (lb.normal.as_str(), &hi_t),
        (hdr.as_str(), &dim_t),
    ];
    let rw: f64 = runs.iter().map(|(s, t)| typo::width(s, t)).sum::<f64>() + 2.0;
    b.push_str(&doc.spans(px + pw - 12.0 - rw, py + 20.0, &runs));
    let (cw, gap) = (96.0, 8.0);
    for (i, ch) in g.characters.iter().take(5).enumerate() {
        let (x, y) = (px + 12.0 + i as f64 * (cw + gap), py + 36.0);
        let _ = write!(
            b,
            r#"<clipPath id="q{i}"><rect x="{x}" y="{y}" width="{cw}" height="{cw}" rx="6"/></clipPath><rect x="{x}" y="{y}" width="{cw}" height="{cw}" rx="6" fill="{PLATE}"/><g clip-path="url(#q{i})">"#
        );
        if let Some(e) = art(&dir, &ch.key(), 192, true) {
            b.push_str(&e.tag(&format!("mq{i}"), x, y, cw, cw, ""));
        }
        let _ = write!(
            b,
            r#"<rect x="{x}" y="{a}" width="{cw}" height="24" fill="url(#lv)" opacity=".92"/>"#,
            a = y + cw - 24.0
        );
        let lv = ch.level.map(|l| format!("LV.{l}")).unwrap_or_default();
        b.push_str(&doc.text(x + 8.0, y + cw - 6.0, &lv, &sx(17.0).fill(WHITE)));
        let _ = write!(
            b,
            r##"</g><rect x="{x}" y="{y}" width="{cw}" height="{cw}" rx="6" fill="none" stroke="#ffffff" stroke-opacity=".25"/><path d="M{hx} {hy} l-4.2 -4 a2.5 2.5 0 0 1 4.2 -3 a2.5 2.5 0 0 1 4.2 3z" fill="#ff5d8f"/><path d="M{a} {c} l10 10 m0 -10 l-10 10" stroke="{DIM}" stroke-width="2"/>"##,
            hx = x + cw - 12.0,
            hy = y + 12.0,
            a = x + 6.0,
            c = y + cw + 12.0
        );
        b.push_str(&doc.text(x + 22.0, y + cw + 22.0, &ch.power, &sb(16.0).fill(WHITE)));
        b.push_str(&doc.text(
            x,
            y + cw + 42.0,
            &ch.short.to_uppercase(),
            &sx(12.0).fill(DIM).ls(0.5),
        ));
    }
    let _ = write!(
        b,
        r##"<g clip-path="url(#sq)"><rect class="sheen" x="452" y="150" width="90" height="190" fill="url(#beam)" transform="skewX(-14)"/></g><line x1="{a}" y1="{c}" x2="{d}" y2="{c}" stroke="#ffffff" stroke-opacity=".1"/>"##,
        a = px + 12.0,
        c = py + 186.0,
        d = px + pw - 12.0
    );
    b.push_str(&doc.text(
        px + 14.0,
        py + 214.0,
        "CLEAR POWER / STANDARD",
        &sb(13.0).fill(DIM).ls(1.0),
    ));
    let cp = sx(24.0).fill(BLUE);
    let sp = sx(24.0).fill(WHITE);
    let cps = format!("{} ", kit::group(lb.clear_power));
    let sps = format!("/ {}", kit::group(lb.standard_power));
    let cw2 = typo::width(&cps, &cp) + typo::width(&sps, &sp);
    b.push_str(&doc.spans(
        px + pw - 12.0 - cw2,
        py + 216.0,
        &[(cps.as_str(), &cp), (sps.as_str(), &sp)],
    ));

    // loved shelf
    let ly = 374.0;
    b.push_str(&doc.text(
        px,
        ly,
        "♥ LOVED · ALL OF THEM, FOR THE LORE",
        &sx(13.0).fill("#ff8fb1").ls(2.0),
    ));
    b.push_str(&doc.text(
        px + pw,
        ly,
        &format!("{} NIKKES", g.loved.len()),
        &sb(12.0).fill(DIM).ls(1.0).end(),
    ));
    for (i, ch) in g.loved.iter().take(10).enumerate() {
        let (x, y) = (
            px + (i % 5) as f64 * 108.0,
            ly + 12.0 + (i / 5) as f64 * 60.0,
        );
        let col = if ch.accent.is_empty() {
            "#ff8fb1"
        } else {
            ch.accent.as_str()
        };
        let _ = write!(
            b,
            r#"<clipPath id="l{i}"><path d="M{a} {y} l22 12 v26 l-22 12 l-22 -12 v-26z"/></clipPath><path d="M{a} {c} l24 13 v28 l-24 13 l-24 -13 v-28z" fill="{col}"/><g clip-path="url(#l{i})"><rect x="{x}" y="{y}" width="48" height="50" fill="{PLATE}"/>"#,
            a = x + 24.0,
            c = y - 2.0
        );
        if let Some(e) = art(&dir, &ch.key(), 100, true) {
            b.push_str(&e.tag(&format!("ml{i}"), x, y, 48.0, 50.0, ""));
        }
        b.push_str("</g>");
        let name = ch.short.to_uppercase();
        let parts: Vec<&str> = if name.chars().count() > 9 {
            name.splitn(2, ' ').collect()
        } else {
            vec![name.as_str()]
        };
        for (k, part) in parts.iter().enumerate() {
            let yy = y + if parts.len() == 1 { 30.0 } else { 22.0 } + k as f64 * 16.0;
            b.push_str(&doc.text(x + 54.0, yy, part, &sx(13.5).fill(WHITE)));
        }
    }

    // lobby panels
    let by = 514.0;
    let _ = write!(
        b,
        r##"<path d="M{px} {by} H{a} L{c} {d} H{px}z" fill="#10203c" stroke="{BLUE}" stroke-opacity=".6"/>"##,
        a = px + 170.0,
        c = px + 160.0,
        d = by + 104.0
    );
    b.push_str(&doc.text(
        px + 12.0,
        by + 30.0,
        "Outpost",
        &sx(26.0).fill("#5fb8ff").skew(-10.0),
    ));
    let k12 = sb(12.0).fill(DIM).ls(1.0);
    let v15 = sx(15.0).fill(WHITE);
    let def_lv = format!("LV.{}", lb.outpost_defense_level);
    b.push_str(&doc.spans(
        px + 12.0,
        by + 54.0,
        &[("DEFENSE ", &k12), (def_lv.as_str(), &v15)],
    ));
    let _ = write!(
        b,
        r##"<rect x="{a}" y="{c}" width="136" height="4" fill="#ffffff" fill-opacity=".12"/><rect class="fill" x="{a}" y="{c}" width="53" height="4" fill="{BLUE}"/>"##,
        a = px + 12.0,
        c = by + 62.0
    );
    b.push_str(&doc.spans(
        px + 12.0,
        by + 88.0,
        &[
            ("RESEARCH ", &k12),
            (lb.research.as_str(), &v15),
            ("   SYNCHRO ", &k12),
            (lb.synchro.as_str(), &v15),
        ],
    ));
    let _ = write!(
        b,
        r#"<path d="M{a} {by} H{c} L{d} {e} H{f}z" fill="url(#white)"/>"#,
        a = px + 180.0,
        c = px + 350.0,
        d = px + 340.0,
        e = by + 104.0,
        f = px + 170.0
    );
    let _ = write!(
        b,
        r##"<circle cx="{a}" cy="{c}" r="14" fill="none" stroke="#2b3240" stroke-width="3"/><circle cx="{a}" cy="{c}" r="5" fill="#2b3240"/>"##,
        a = px + 210.0,
        c = by + 34.0
    );
    b.push_str(&doc.text(px + 232.0, by + 44.0, "Ark", &sx(30.0).fill("#2b3240")));
    let dk = sb(12.0).fill("#6b7486").ls(1.0);
    let dv = sx(16.0).fill("#11151d");
    b.push_str(&doc.spans(
        px + 190.0,
        by + 76.0,
        &[
            ("TOWERS ", &dk),
            (lb.towers.as_str(), &dv),
            ("   NIKKES ", &dk),
            (lb.nikkes.as_str(), &dv),
        ],
    ));
    b.push_str(&doc.spans(
        px + 190.0,
        by + 94.0,
        &[("OVERCLOCK ", &dk), (lb.overclock.as_str(), &dv)],
    ));
    let _ = write!(
        b,
        r#"<path d="M{a} {by} H{c} V{d} H{e}z" fill="url(#bluep)"/>"#,
        a = px + 360.0,
        c = px + pw,
        d = by + 104.0,
        e = px + 350.0
    );
    b.push_str(&doc.text(
        px + 374.0,
        by + 26.0,
        "UNION",
        &sb(12.0).fill("#cfe6ff").ls(1.5),
    ));
    let un = sx(26.0).fill(WHITE);
    let ul = sx(14.0).fill("#cfe6ff");
    let ulv = format!(" LV.{:02}", lb.union_level);
    b.push_str(&doc.spans(
        px + 374.0,
        by + 52.0,
        &[(lb.union.as_str(), &un), (ulv.as_str(), &ul)],
    ));
    b.push_str(&doc.text(
        px + 374.0,
        by + 76.0,
        &format!("{} members", lb.union_members),
        &sb(13.0).fill("#e6f2ff"),
    ));
    if let Some(e) = art(&icons, "rank_gold", 40, false) {
        b.push_str(&e.tag("mRank", px + 372.0, by + 81.0, 17.0, 17.0, ""));
    }
    b.push_str(&doc.text(
        px + 394.0,
        by + 94.0,
        &format!("{} rank", lb.union_rank),
        &sb(13.0).fill("#ffe08a"),
    ));
    if let Some(e) = art(&icons, "union", 120, false) {
        b.push_str(&e.tag("mUnion", px + pw - 70.0, by + 16.0, 58.0, 66.0, ""));
    }

    // nav bar
    let ny = H - 68.0;
    let _ = write!(
        b,
        r##"<rect x="0" y="{ny}" width="{w}" height="68" fill="{NAVY2}" fill-opacity=".92"/><line x1="0" y1="{ny}" x2="{w}" y2="{ny}" stroke="#ffffff" stroke-opacity=".1"/>"##
    );
    for (i, t) in ["Nikke", "Squad", "Lobby", "Inventory", "Recruit"]
        .iter()
        .enumerate()
    {
        let cx = 100.0 + i as f64 * 200.0;
        let on = *t == "Lobby";
        let col = if on { WHITE } else { "#7d879c" };
        if on {
            let _ = write!(
                b,
                r#"<rect x="{a}" y="{ny}" width="100" height="3" fill="{WHITE}"/><path d="M{c} {d} h28 l-14 22z" fill="none" stroke="{col}" stroke-width="2.4"/><circle cx="{cx}" cy="{e}" r="6" fill="none" stroke="{col}" stroke-width="2"/>"#,
                a = cx - 50.0,
                c = cx - 14.0,
                d = ny + 14.0,
                e = ny + 21.0
            );
        } else {
            let _ = write!(
                b,
                r#"<rect x="{a}" y="{c}" width="22" height="22" rx="4" transform="rotate(45 {cx} {d})" fill="none" stroke="{col}" stroke-width="2.2"/>"#,
                a = cx - 11.0,
                c = ny + 14.0,
                d = ny + 25.0
            );
        }
        b.push_str(&doc.text(cx, ny + 56.0, t, &sx(15.0).fill(col).mid()));
    }
    let lobby_name = find(g.characters.iter().chain(&g.loved), &lb.character)
        .map(|c| c.name.to_uppercase())
        .unwrap_or_default();
    let caption = format!(
        "{} · {} · UID {} · LOBBY: {lobby_name}",
        g.title.to_uppercase(),
        g.server.to_uppercase(),
        g.game_id
    );
    b.push_str(&doc.text(
        22.0,
        ny - 12.0,
        &caption,
        &sb(12.0).fill(WHITE).op(0.6).ls(2.0),
    ));

    let frame = format!(r#"<g clip-path="url(#card)">{b}</g>"#);
    let css = ".breathe{animation:breathe 5s ease-in-out infinite}@keyframes breathe{50%{transform:translateY(-4px)}}.streak{animation:streak 7s linear infinite}@keyframes streak{from{transform:translateX(-300px)}to{transform:translateX(1300px)}}.bokeh{animation:bokeh 9s ease-in-out infinite alternate}@keyframes bokeh{to{transform:translate(18px,-14px);opacity:.25}}.ping{transform-box:fill-box;transform-origin:center;animation:ping 2s ease-out infinite}@keyframes ping{0%{transform:scale(1);opacity:.9}100%{transform:scale(1.9);opacity:0}}.sheen{animation:sheen 4.8s cubic-bezier(.6,0,.2,1) infinite}@keyframes sheen{0%{transform:translateX(-140px)}55%,100%{transform:translateX(620px)}}.ring{animation:ring 2.4s cubic-bezier(.16,1,.3,1) both}@keyframes ring{from{stroke-dashoffset:188}}.fill{transform-box:fill-box;transform-origin:left;animation:fill 2.4s cubic-bezier(.16,1,.3,1) both}@keyframes fill{from{transform:scaleX(0)}}";
    let defs = format!(
        r##"<radialGradient id="scene" cx=".25" cy=".45" r=".9"><stop offset="0" stop-color="#1b2a4a"/><stop offset=".55" stop-color="{NAVY}"/><stop offset="1" stop-color="{NAVY2}"/></radialGradient><radialGradient id="rim" cx=".5" cy=".5" r=".5"><stop offset="0" stop-color="{BLUE}" stop-opacity=".35"/><stop offset="1" stop-color="{BLUE}" stop-opacity="0"/></radialGradient><linearGradient id="beam" x1="0" y1="0" x2="1" y2="0"><stop offset="0" stop-color="#fff" stop-opacity="0"/><stop offset=".5" stop-color="#fff" stop-opacity=".18"/><stop offset="1" stop-color="#fff" stop-opacity="0"/></linearGradient><linearGradient id="bluep" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="{BLUE}"/><stop offset="1" stop-color="#2a6cff"/></linearGradient><linearGradient id="lv" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#ffd36b"/><stop offset="1" stop-color="#f5a623"/></linearGradient><linearGradient id="fadeB" x1="0" y1="0" x2="0" y2="1"><stop offset=".55" stop-color="{NAVY2}" stop-opacity="0"/><stop offset="1" stop-color="{NAVY2}" stop-opacity=".95"/></linearGradient><linearGradient id="white" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="#ffffff"/><stop offset="1" stop-color="#dfe6ee"/></linearGradient><clipPath id="card"><rect width="{w}" height="{H}" rx="14"/></clipPath><clipPath id="sq"><rect x="452" y="150" width="520" height="190"/></clipPath><clipPath id="av"><circle cx="54" cy="52" r="27"/></clipPath>"##
    );
    let label = format!(
        "Best game: {}, commander {} level {}, campaign squad and {} loved Nikkes",
        g.title,
        g.nick,
        g.level,
        g.loved.len()
    );
    Ok(vec![(
        "bestgame.svg".into(),
        kit::svg(w, H, &label, css, &defs, &doc, &frame),
    )])
}
