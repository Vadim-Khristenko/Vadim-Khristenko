//! The footer closes the score the header opened: the staff ends in a final
//! double bar marked "Fine", and the editor status bar says when and how the
//! profile was rendered.

use super::kit::{self, pal};
use crate::run::{Ctx, CARDS};
use crate::typo::{Doc, Font, T};
use anyhow::Result;
use std::fmt::Write;

const H: f64 = 150.0;

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let mut doc = Doc::new();
    let mut b = String::new();
    let w = kit::W;
    let _ = write!(b, r#"<rect width="{w}" height="{H}" fill="{}"/>"#, pal::BG);
    for i in 0..5 {
        let y = 24.0 + i as f64 * 9.0;
        let _ = write!(
            b,
            r#"<line x1="0" y1="{y}" x2="{}" y2="{y}" stroke="url(#fade)"/>"#,
            w - 60.0
        );
    }
    for (x, step) in [
        (560.0, 2.0),
        (610.0, 3.0),
        (660.0, 1.0),
        (710.0, 4.0),
        (770.0, 2.0),
    ] {
        let y = 60.0 - step * 4.5;
        let _ = write!(
            b,
            r#"<ellipse cx="{x}" cy="{y}" rx="5.4" ry="4" transform="rotate(-20 {x} {y})" fill="{t}"/><line x1="{a}" y1="{c}" x2="{a}" y2="{d}" stroke="{t}" stroke-width="1.4"/>"#,
            t = pal::TEAL,
            a = x + 5.0,
            c = y - 1.0,
            d = y - 24.0
        );
    }
    let _ = write!(
        b,
        r#"<g class="last"><ellipse cx="840" cy="51" rx="7" ry="5" transform="rotate(-20 840 51)" fill="none" stroke="{}" stroke-width="2"/></g><line x1="{a}" y1="24" x2="{a}" y2="60" stroke="{fg}" stroke-width="1.4"/><rect x="{c}" y="24" width="5" height="36" fill="{fg}"/>"#,
        pal::SAKURA,
        a = w - 72.0,
        c = w - 66.0,
        fg = pal::FG
    );
    b.push_str(
        &doc.text(
            w - 60.0,
            88.0,
            "Fine.",
            &T::new(Font::ZenBlack, 22.0)
                .fill(pal::FG_HI)
                .end()
                .skew(-12.0),
        ),
    );
    b.push_str(&doc.text(
        40.0,
        84.0,
        "Thanks for listening to the whole track.",
        &T::new(Font::ZenLight, 15.0).fill(pal::FG),
    ));
    let forges: Vec<&str> = ctx
        .agg
        .platforms
        .iter()
        .map(|p| p.display.as_str())
        .collect();
    b.push_str(&doc.text(
        40.0,
        104.0,
        &format!(
            "rendered by vai-profile, one Rust binary · {}",
            forges.join(" + ")
        ),
        &T::new(Font::Mono, 10.5).fill(pal::DIM),
    ));
    let sb = H - 30.0;
    let _ = write!(
        b,
        r#"<rect x="0" y="{sb}" width="{w}" height="30" fill="{}"/><rect x="0" y="{sb}" width="74" height="30" fill="{}"/>"#,
        pal::STATUS,
        pal::TEAL
    );
    b.push_str(&doc.text(
        37.0,
        sb + 19.0,
        "VAI",
        &T::new(Font::MonoBold, 11.0).fill(pal::BG).mid(),
    ));
    let st = T::new(Font::Mono, 10.5).fill(pal::FG);
    b.push_str(&doc.text(
        90.0,
        sb + 19.0,
        &format!(
            "{} cards    {} forges    {}",
            CARDS.len(),
            forges.len(),
            ctx.stamp
        ),
        &st,
    ));
    b.push_str(&doc.text(
        w - 18.0,
        sb + 19.0,
        "Ln 39, Col 1    Hatsune Miku Dark",
        &st.clone().end(),
    ));

    let frame = format!(
        r#"<g clip-path="url(#card)">{b}</g><rect x=".5" y=".5" width="{}" height="{}" rx="14" fill="none" stroke="{}" stroke-opacity=".9"/>"#,
        w - 1.0,
        H - 1.0,
        pal::LINE
    );
    let css = ".last{animation:last 3s ease-in-out infinite}@keyframes last{50%{opacity:.35}}";
    let defs = format!(
        r#"<clipPath id="card"><rect width="{w}" height="{H}" rx="14"/></clipPath><linearGradient id="fade" gradientUnits="userSpaceOnUse" x1="0" y1="0" x2="{}" y2="0"><stop offset="0" stop-color="{l}" stop-opacity="0"/><stop offset=".3" stop-color="{l}"/></linearGradient>"#,
        w - 60.0,
        l = pal::LINE
    );
    Ok(vec![(
        "footer.svg".into(),
        kit::svg(
            w,
            H,
            "Fine. Rendered by the VAI Profile Engine",
            css,
            &defs,
            &doc,
            &frame,
        ),
    )])
}
