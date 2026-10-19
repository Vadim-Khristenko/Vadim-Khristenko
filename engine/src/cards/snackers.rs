//! The Filian fandom card in the language of Filian.ru itself: a Windows 98
//! desktop with pixel hearts. Desktop shortcuts are the projects, the window
//! holds the story, and a pixel Hatsune Miku figure stands in the "gift from
//! Snackers RU" group box.

use super::kit;
use crate::run::Ctx;
use crate::typo::{Doc, Font, T};
use anyhow::Result;
use std::fmt::Write;

const H: f64 = 360.0;
const DESK: &str = "#008080";
const NAVY: &str = "#000080";
const GREY: &str = "#c0c0c0";
const GREY_L: &str = "#dfdfdf";
const GREY_D: &str = "#808080";
const BLACK: &str = "#0a0a0a";
const WHITE: &str = "#ffffff";
const PINK: &str = "#e75480";
const RU: &str = "#c8005f";

const SPRITE: [&str; 21] = [
    "......DDTTTTDD......",
    ".....DTTTTTTTTD.....",
    "..KKDTTTTTTTTTTDKK..",
    ".DTTKTTSSSSSSTTKTTD.",
    ".DTTTTSSSSSSSSTTTTD.",
    ".DTTTTSEESSEESTTTTD.",
    ".DTTTTSSSSSSSSTTTTD.",
    ".DTTT.SSSPPSSS.TTTD.",
    ".DTTT..SSSSSS..TTTD.",
    ".DTTT.GGGTTGGG.TTTD.",
    ".DTTTGGGGTTGGGGTTTD.",
    ".DTTTSGGGGGGGGSTTTD.",
    ".DTTTSGGGGGGGGSTTTD.",
    "..DTT.KKKKKKKK.TTD..",
    "..DTTKKKKKKKKKKTTD..",
    "..DTT..SS..SS..TTD..",
    "...DT..SS..SS..TD...",
    "...DT..KK..KK..TD...",
    "....D..KK..KK..D....",
    "....BBBBBBBBBBBB....",
    "...BBBBBBBBBBBBBB...",
];
const HEART: [&str; 8] = [
    ".XX...XX.",
    "XHHX.XPPX",
    "XHPPXPPPX",
    "XPPPPPPPX",
    ".XPPPPPX.",
    "..XPPPX..",
    "...XPX...",
    "....X....",
];

fn sprite_color(c: char) -> Option<&'static str> {
    Some(match c {
        'T' => "#39c5bb",
        'D' => "#1f8a83",
        'K' => "#1b1b2a",
        'S' => "#ffe3d3",
        'E' => "#137a73",
        'P' => "#e87ea1",
        'G' => "#d6dae6",
        'B' => "#4a5070",
        _ => return None,
    })
}

fn heart_color(c: char) -> Option<&'static str> {
    Some(match c {
        'X' => BLACK,
        'P' => PINK,
        'H' => WHITE,
        _ => return None,
    })
}

/// Pixel art as one path per colour, which is far smaller than a rect per
/// pixel.
fn pixels(
    rows: &[&str],
    pal: fn(char) -> Option<&'static str>,
    x0: f64,
    y0: f64,
    s: f64,
) -> String {
    let mut by: std::collections::BTreeMap<&str, String> = Default::default();
    for (j, row) in rows.iter().enumerate() {
        for (i, ch) in row.chars().enumerate() {
            if let Some(col) = pal(ch) {
                let _ = write!(
                    by.entry(col).or_default(),
                    "M{} {}h{s}v{s}h-{s}z",
                    x0 + i as f64 * s,
                    y0 + j as f64 * s
                );
            }
        }
    }
    by.into_iter()
        .map(|(c, d)| format!(r#"<path d="{d}" fill="{c}"/>"#))
        .collect()
}

fn bevel(x: f64, y: f64, w: f64, h: f64, inset: bool) -> String {
    let (hi, lo) = if inset {
        (GREY_D, WHITE)
    } else {
        (WHITE, GREY_D)
    };
    format!(
        r#"<rect x="{x}" y="{y}" width="{w}" height="{h}" fill="{GREY}"/><path d="M{x} {b} V{y} H{r}" fill="none" stroke="{hi}" stroke-width="2"/><path d="M{x} {b} H{r} V{y}" fill="none" stroke="{lo}" stroke-width="2"/>"#,
        b = y + h,
        r = x + w
    )
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let mut doc = Doc::new();
    let mut b = String::new();
    let w = kit::W;
    let wiki = kit::show(ctx, "wiki", "wiki", "filian.wiki");
    let ru = kit::show(ctx, "wiki", "ru", "filian.ru");
    let gift = kit::show(ctx, "snackers", "gift", "Hatsune Miku");

    let _ = write!(b, r#"<rect width="{w}" height="{H}" fill="{DESK}"/>"#);
    for (i, (label, kind)) in [(wiki.as_str(), 0), (ru.as_str(), 1), ("The Wall", 2)]
        .iter()
        .enumerate()
    {
        let (x, y) = (30.0, 26.0 + i as f64 * 92.0);
        match kind {
            1 => b.push_str(&pixels(&HEART, heart_color, x + 8.0, y + 4.0, 4.0)),
            0 => {
                let _ = write!(
                    b,
                    r#"<rect x="{}" y="{y}" width="30" height="38" fill="{WHITE}" stroke="{BLACK}"/><path d="M{} {y} v8 h10" fill="{GREY_L}" stroke="{BLACK}"/>"#,
                    x + 10.0,
                    x + 30.0
                );
                for k in 0..4 {
                    let _ = write!(
                        b,
                        r#"<rect x="{}" y="{}" width="{}" height="2" fill="{NAVY}"/>"#,
                        x + 15.0,
                        y + 12.0 + k as f64 * 6.0,
                        if k % 2 == 1 { 18 } else { 20 }
                    );
                }
            }
            _ => {
                for r in 0..4 {
                    for c in 0..3 {
                        let off = if r % 2 == 1 { 6.0 } else { 0.0 };
                        let _ = write!(
                            b,
                            r##"<rect x="{}" y="{}" width="12" height="8" fill="#b5523b" stroke="#5a2418"/>"##,
                            x + 3.0 + c as f64 * 13.0 + off,
                            y + 4.0 + r as f64 * 9.0
                        );
                    }
                }
            }
        }
        b.push_str(&doc.text(
            x + 25.0,
            y + 56.0,
            label,
            &T::new(Font::Pt, 11.5).fill(WHITE).mid(),
        ));
    }

    let (x, y, ww, wh) = (130.0, 22.0, 840.0, 286.0);
    b.push_str(&bevel(x, y, ww, wh, false));
    let _ = write!(
        b,
        r#"<rect x="{}" y="{}" width="{}" height="26" fill="url(#title)"/>"#,
        x + 4.0,
        y + 4.0,
        ww - 8.0
    );
    let _ = write!(
        b,
        r#"<g class="beat">{}</g>"#,
        pixels(&HEART, heart_color, x + 10.0, y + 8.0, 2.2)
    );
    b.push_str(&doc.text(
        x + 36.0,
        y + 23.0,
        "Snackers.txt · VAI",
        &T::new(Font::PixelBold, 14.0).fill(WHITE),
    ));
    for (k, glyph) in ["_", "□", "×"].iter().enumerate() {
        let bx = x + ww - 66.0 + k as f64 * 20.0;
        b.push_str(&bevel(bx, y + 8.0, 17.0, 16.0, false));
        b.push_str(&doc.text(
            bx + 8.5,
            y + 20.0,
            glyph,
            &T::new(Font::PtBold, 11.0).fill(BLACK).mid(),
        ));
    }
    let title_t = T::new(Font::PixelBold, 30.0).fill(NAVY);
    let title2_t = T::new(Font::PixelBold, 30.0).fill(RU);
    b.push_str(&doc.spans(
        x + 22.0,
        y + 70.0,
        &[("Snackers ", &title_t), ("HQ+ · RU · world", &title2_t)],
    ));
    let plain = T::new(Font::Pt, 14.0).fill(BLACK);
    let bold = T::new(Font::PtBold, 14.0).fill(NAVY);
    let lines: [(&str, String); 5] = [
        ("FilianWIKI", format!(" · {wiki}")),
        (
            "",
            "the wiki for Snackers HQ+, Snackers RU and every snacker".into(),
        ),
        ("Filian.ru", " · home of Snackers RU".into()),
        (
            "NotAnOtherVTUBERwiki",
            " · the Rust engine under both".into(),
        ),
        ("The Wall Dev", " · the dev side of the Wall".into()),
    ];
    for ((bl, rest), ly) in lines.iter().zip([104.0, 122.0, 150.0, 178.0, 206.0]) {
        b.push_str(&doc.spans(x + 24.0, y + ly, &[(bl, &bold), (rest.as_str(), &plain)]));
    }
    b.push_str(&bevel(x + 22.0, y + wh - 48.0, 86.0, 26.0, false));
    b.push_str(&doc.text(
        x + 65.0,
        y + wh - 30.0,
        "OK",
        &T::new(Font::Pt, 13.0).fill(BLACK).mid(),
    ));
    b.push_str(&bevel(x + 118.0, y + wh - 48.0, 150.0, 26.0, false));
    b.push_str(&doc.text(
        x + 193.0,
        y + wh - 30.0,
        &format!("Open {wiki}"),
        &T::new(Font::Pt, 13.0).fill(BLACK).mid(),
    ));
    let _ = write!(
        b,
        r#"<rect x="{}" y="{}" width="78" height="18" fill="none" stroke="{BLACK}" stroke-dasharray="1 1"/>"#,
        x + 26.0,
        y + wh - 44.0
    );

    let (gx, gy, gw, gh) = (x + 530.0, y + 48.0, 286.0, 220.0);
    let _ = write!(
        b,
        r#"<rect x="{gx}" y="{gy}" width="{gw}" height="{gh}" fill="none" stroke="{GREY_D}"/><rect x="{}" y="{}" width="{gw}" height="{gh}" fill="none" stroke="{WHITE}"/><rect x="{}" y="{}" width="148" height="16" fill="{GREY}"/>"#,
        gx + 1.0,
        gy + 1.0,
        gx + 10.0,
        gy - 8.0
    );
    b.push_str(&doc.text(
        gx + 14.0,
        gy + 4.0,
        "Gift from Snackers RU",
        &T::new(Font::Pt, 12.0).fill(BLACK),
    ));
    let _ = write!(
        b,
        r#"<g class="bob">{}</g><rect x="{}" y="{}" width="116" height="6" fill="{GREY_D}"/>"#,
        pixels(&SPRITE, sprite_color, gx + 28.0, gy + 30.0, 5.0),
        gx + 20.0,
        gy + 138.0
    );
    for (sx, sy, d) in [
        (gx + 22.0, gy + 34.0, 0.0),
        (gx + 128.0, gy + 52.0, 1.2),
        (gx + 118.0, gy + 22.0, 0.6),
    ] {
        let _ = write!(
            b,
            r#"<g class="spark" style="animation-delay:{d}s"><rect x="{sx}" y="{}" width="2" height="10" fill="{WHITE}"/><rect x="{}" y="{sy}" width="10" height="2" fill="{WHITE}"/></g>"#,
            sy - 4.0,
            sx - 4.0
        );
    }
    let mut gy2 = gy + 50.0;
    for word in gift.split_whitespace().take(2) {
        b.push_str(&doc.text(
            gx + 150.0,
            gy2,
            word,
            &T::new(Font::PixelBold, 17.0).fill(NAVY),
        ));
        gy2 += 20.0;
    }
    b.push_str(&doc.text(
        gx + 150.0,
        gy + 92.0,
        "a figure from",
        &T::new(Font::Pt, 12.0).fill(BLACK),
    ));
    b.push_str(&doc.text(
        gx + 150.0,
        gy + 108.0,
        "Snackers RU",
        &T::new(Font::Pt, 12.0).fill(BLACK),
    ));
    b.push_str(&doc.text(
        gx + 20.0,
        gy + 186.0,
        "Thank you, Snackers RU",
        &T::new(Font::PixelBold, 16.0).fill(RU),
    ));
    let _ = write!(
        b,
        r#"<g class="beat">{}</g>"#,
        pixels(&HEART, heart_color, gx + 226.0, gy + 172.0, 2.0)
    );

    let tb = H - 30.0;
    let _ = write!(
        b,
        r#"<rect x="0" y="{tb}" width="{w}" height="30" fill="{GREY}"/><line x1="0" y1="{}" x2="{w}" y2="{}" stroke="{WHITE}" stroke-width="2"/>"#,
        tb + 1.0,
        tb + 1.0
    );
    b.push_str(&bevel(4.0, tb + 4.0, 70.0, 22.0, false));
    b.push_str(&doc.text(
        14.0,
        tb + 20.0,
        "Start",
        &T::new(Font::PixelBold, 13.0).fill(BLACK),
    ));
    b.push_str(&bevel(84.0, tb + 4.0, 190.0, 22.0, true));
    b.push_str(&doc.text(
        94.0,
        tb + 20.0,
        "Snackers.txt · VAI",
        &T::new(Font::PtBold, 12.0).fill(BLACK),
    ));
    b.push_str(&bevel(w - 84.0, tb + 4.0, 78.0, 22.0, true));
    b.push_str(&doc.text(
        w - 45.0,
        tb + 20.0,
        "3:39 AM",
        &T::new(Font::Pt, 12.0).fill(BLACK).mid(),
    ));

    let frame = format!(r#"<g clip-path="url(#card)">{b}</g>"#);
    let css = ".beat{transform-box:fill-box;transform-origin:center;animation:beat 1.4s steps(2) infinite}@keyframes beat{0%,60%,100%{transform:scale(1)}20%{transform:scale(1.15)}}.bob{animation:bob 2.4s steps(2) infinite}@keyframes bob{50%{transform:translateY(-3px)}}.spark{animation:spark 2.4s steps(1) infinite}@keyframes spark{50%{opacity:0}}";
    let defs = format!(
        r#"<linearGradient id="title" x1="0" y1="0" x2="1" y2="0"><stop offset="0" stop-color="{NAVY}"/><stop offset="1" stop-color="{}"/></linearGradient><clipPath id="card"><rect width="{w}" height="{H}" rx="12"/></clipPath>"#,
        "#1084d0"
    );
    Ok(vec![(
        "fandom_snackers.svg".into(),
        kit::svg(w, H, "Snackers: FilianWIKI for every snacker, Filian.ru for Snackers RU, and a Miku figure gift", css, &defs, &doc, &frame),
    )])
}
