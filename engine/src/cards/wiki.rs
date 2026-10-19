//! NotAnOtherVTUBERwiki drawn as a FilianWIKI article page in the Snackers
//! dark skin: Article/Talk tabs, "From FilianWIKI", footnote markers, a
//! contents box and an infobox. Colours and faces come from skins/snackers.

use super::kit;
use crate::run::Ctx;
use crate::typo::{Doc, Font, T};
use anyhow::Result;
use std::fmt::Write;

const H: f64 = 420.0;
const BGW: &str = "#0b0713";
const ELEV: &str = "#150e22";
const SUNK: &str = "#211732";
const INKW: &str = "#ffffff";
const SOFT: &str = "#e8d5e2";
const MUT: &str = "#a89fba";
const LN: &str = "#2d1e45";
const BRD: &str = "#4a2d6b";
const ACC: &str = "#ff3d8b";
const ACC2: &str = "#a366ff";

enum Run {
    Plain(&'static str),
    Bold(&'static str),
    Sup(&'static str),
}

const LEAD: [&[Run]; 5] = [
    &[
        Run::Bold("NotAnotherWiki"),
        Run::Plain(" (codename NotAnOtherVTUBERwiki) is a wiki engine for fan"),
    ],
    &[
        Run::Plain("communities, written in Rust by VAI_PROG."),
        Run::Sup("[1]"),
        Run::Plain(" Pages render on write and are"),
    ],
    &[Run::Plain(
        "served from cache, so reading and editing work with JavaScript off.",
    )],
    &[
        Run::Plain("Its first deployment is "),
        Run::Bold("FilianWIKI"),
        Run::Plain(", built for Snackers HQ+, Snackers RU"),
    ],
    &[
        Run::Plain("and every snacker in the world. Unofficial, not part of the ARG."),
        Run::Sup("[2]"),
    ],
];

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let mut doc = Doc::new();
    let mut b = String::new();
    let w = kit::W;
    let repo = kit::show(ctx, "wiki", "repo", "NotAnOtherVTUBERwiki");
    let wiki = kit::show(ctx, "wiki", "wiki", "filian.wiki");
    let beta = kit::show(ctx, "wiki", "beta", "beta.filian.wiki");
    let ru = kit::show(ctx, "wiki", "ru", "filian.ru");

    let _ = write!(
        b,
        r#"<rect width="{w}" height="{H}" fill="{BGW}"/><rect width="{w}" height="52" fill="{ELEV}"/><line x1="0" y1="52" x2="{w}" y2="52" stroke="{LN}"/>"#
    );
    let _ = write!(
        b,
        r#"<rect x="32" y="16" width="20" height="20" rx="3" fill="{ACC}"/>"#
    );
    b.push_str(&doc.text(
        42.0,
        31.0,
        "F",
        &T::new(Font::PixelBold, 14.0).fill(BGW).mid(),
    ));
    b.push_str(&doc.text(
        60.0,
        32.0,
        "FilianWIKI",
        &T::new(Font::PixelBold, 18.0).fill(INKW),
    ));
    let _ = write!(
        b,
        r#"<rect x="300" y="14" width="360" height="24" rx="12" fill="{SUNK}" stroke="{BRD}"/><rect class="caret" x="314" y="19" width="1.5" height="14" fill="{ACC}"/>"#
    );
    b.push_str(&doc.text(
        320.0,
        30.0,
        "Search the wiki",
        &T::new(Font::Pt, 12.0).fill(MUT),
    ));
    let _ = write!(
        b,
        r#"<circle class="live" cx="{}" cy="26" r="4" fill="{ACC}"/>"#,
        w - 170.0
    );
    b.push_str(&doc.text(
        w - 160.0,
        30.0,
        &beta,
        &T::new(Font::PtMono, 11.5).fill(SOFT),
    ));

    b.push_str(&doc.text(
        40.0,
        80.0,
        "Article",
        &T::new(Font::PtBold, 13.0).fill(INKW),
    ));
    let _ = write!(
        b,
        r#"<rect x="40" y="88" width="46" height="2" fill="{ACC}"/><line x1="32" y1="90" x2="640" y2="90" stroke="{LN}"/>"#
    );
    b.push_str(&doc.text(104.0, 80.0, "Talk", &T::new(Font::Pt, 13.0).fill(MUT)));
    for (i, t) in ["Read", "Edit", "History"].iter().enumerate() {
        b.push_str(&doc.text(
            470.0 + i as f64 * 58.0,
            80.0,
            t,
            &T::new(Font::Pt, 13.0).fill(if i == 0 { SOFT } else { MUT }),
        ));
    }

    b.push_str(&doc.text(
        40.0,
        132.0,
        "NotAnotherWiki Engine",
        &T::new(Font::UnboundedBold, 25.0).fill(INKW).ls(-0.3),
    ));
    b.push_str(&doc.text(
        40.0,
        154.0,
        "From FilianWIKI, the wiki of the Snackers",
        &T::new(Font::Pt, 12.5).fill(MUT).skew(-10.0),
    ));
    let plain = T::new(Font::Pt, 14.0).fill(SOFT);
    let bold = T::new(Font::PtBold, 14.0).fill(INKW);
    let sup = T::new(Font::Pt, 9.0).fill(ACC);
    for (i, line) in LEAD.iter().enumerate() {
        let y = 184.0 + i as f64 * 21.0;
        let mut x = 40.0;
        for run in line.iter() {
            let (s, t, dy) = match run {
                Run::Plain(s) => (*s, &plain, 0.0),
                Run::Bold(s) => (*s, &bold, 0.0),
                Run::Sup(s) => (*s, &sup, -5.0),
            };
            b.push_str(&doc.text(x, y + dy, s, t));
            x += crate::typo::width(s, t);
        }
    }

    let cy = 300.0;
    let _ = write!(
        b,
        r#"<rect x="40" y="{cy}" width="250" height="96" rx="6" fill="{ELEV}" stroke="{LN}"/>"#
    );
    b.push_str(&doc.text(
        56.0,
        cy + 22.0,
        "Contents",
        &T::new(Font::PtBold, 12.5).fill(INKW),
    ));
    for (i, t) in [
        "Why it exists",
        "Architecture",
        "Skins: default and snackers",
        &format!("{ru} for Snackers RU"),
    ]
    .iter()
    .enumerate()
    {
        let num_t = T::new(Font::Pt, 12.0).fill(MUT);
        let link_t = T::new(Font::Pt, 12.0).fill(ACC2);
        b.push_str(&doc.spans(
            56.0,
            cy + 42.0 + i as f64 * 16.0,
            &[(&format!("{}  ", i + 1), &num_t), (t, &link_t)],
        ));
    }
    b.push_str(&doc.text(
        316.0,
        cy + 22.0,
        "Zero ads. Zero trackers.",
        &T::new(Font::Pt, 12.0).fill(MUT),
    ));
    b.push_str(&doc.text(
        316.0,
        cy + 40.0,
        "Funded by donations.",
        &T::new(Font::Pt, 12.0).fill(MUT),
    ));
    b.push_str(&doc.text(
        316.0,
        cy + 66.0,
        &format!("{wiki} · {ru}"),
        &T::new(Font::PtMono, 11.0).fill(SOFT),
    ));

    let (ix, iy, iw) = (676.0, 72.0, 290.0);
    let rows: Vec<(&str, String)> = vec![
        ("Written in", "Rust · Axum".into()),
        ("Worker", "Bun".into()),
        ("Storage", "PostgreSQL 18 · Valkey 9".into()),
        ("Status", "open beta".into()),
        ("Cache hit p50", "under 20 ms".into()),
        ("Needs JS", "no".into()),
        ("Ads · trackers", "0 · 0".into()),
        ("License", "AGPL-3.0-or-later".into()),
        ("Wiki", wiki.clone()),
        ("RU hub", ru.clone()),
    ];
    let rh = 27.0;
    let _ = write!(
        b,
        r#"<rect x="{ix}" y="{iy}" width="{iw}" height="{}" rx="8" fill="{ELEV}" stroke="{BRD}"/><rect x="{ix}" y="{iy}" width="{iw}" height="36" rx="8" fill="{ACC}"/><rect x="{ix}" y="{}" width="{iw}" height="16" fill="{ACC}"/>"#,
        48.0 + rh * rows.len() as f64,
        iy + 20.0
    );
    b.push_str(&doc.text(
        ix + iw / 2.0,
        iy + 24.0,
        "NotAnotherWiki",
        &T::new(Font::UnboundedBold, 13.0).fill(BGW).mid(),
    ));
    for (i, (k, v)) in rows.iter().enumerate() {
        let y = iy + 58.0 + i as f64 * rh;
        b.push_str(&doc.text(ix + 14.0, y, k, &T::new(Font::PtBold, 12.0).fill(SOFT)));
        let link = *k == "Wiki" || *k == "RU hub";
        b.push_str(&doc.text(
            ix + 122.0,
            y,
            v,
            &T::new(Font::Pt, 12.0).fill(if link { ACC } else { SOFT }),
        ));
        if i + 1 < rows.len() {
            let _ = write!(
                b,
                r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{LN}"/>"#,
                ix + 12.0,
                y + 9.0,
                ix + iw - 12.0,
                y + 9.0
            );
        }
    }
    b.push_str(&kit::live_chip(
        &mut doc,
        ctx,
        &repo,
        640.0,
        H - 14.0,
        MUT,
        ACC,
    ));

    let frame = format!(
        r#"<g clip-path="url(#card)">{b}</g><rect x=".5" y=".5" width="{}" height="{}" rx="14" fill="none" stroke="{BRD}"/>"#,
        w - 1.0,
        H - 1.0
    );
    let css = ".live{animation:live 1.8s ease-in-out infinite}@keyframes live{50%{opacity:.25}}.caret{animation:caret 1.1s steps(1) infinite}@keyframes caret{50%{opacity:0}}";
    let defs =
        format!(r#"<clipPath id="card"><rect width="{w}" height="{H}" rx="14"/></clipPath>"#);
    Ok(vec![(
        "repo_wiki.svg".into(),
        kit::svg(
            w,
            H,
            "NotAnOtherVTUBERwiki engine, running FilianWIKI",
            css,
            &defs,
            &doc,
            &frame,
        ),
    )])
}
