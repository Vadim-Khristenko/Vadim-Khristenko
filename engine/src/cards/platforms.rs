//! The forges as units in a rack. Each unit has a status LED and its numbers;
//! patch cables join copies of the same repo across units, which is exactly
//! the mirror dedup the engine does. The bottom unit is the sum.

use super::kit::{self, pal};
use crate::model::normalize_repo_name;
use crate::run::Ctx;
use crate::typo::{self, Doc, Font, T};
use anyhow::Result;
use std::collections::BTreeMap;
use std::fmt::Write;

const UH: f64 = 84.0;
const PORTS: usize = 4;

fn brand(id: &str) -> &'static str {
    match id {
        "github" => "#c0caf5",
        "codeberg" => "#2185d0",
        _ => pal::AMBER,
    }
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let agg = &ctx.agg;
    let units: Vec<_> = agg.platforms.iter().collect();
    let h = 26.0 + (units.len() as f64 + 1.0) * (UH + 10.0) + 4.0;
    let mut doc = Doc::new();
    let mut b = String::new();
    let w = kit::W;

    // mirror groups: owned repos that live on two or more forges, biggest first
    let mut groups: BTreeMap<String, (String, Vec<usize>, u64)> = BTreeMap::new();
    for (i, p) in units.iter().enumerate() {
        for r in p.repos.iter().filter(|r| !r.fork) {
            let e = groups
                .entry(normalize_repo_name(&r.name))
                .or_insert_with(|| (r.name.clone(), vec![], 0));
            if !e.1.contains(&i) {
                e.1.push(i);
            }
            e.2 += r.stars;
        }
    }
    let mut mirrored: Vec<_> = groups.values().filter(|g| g.1.len() > 1).collect();
    mirrored.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then(b.2.cmp(&a.2)));
    let shown: Vec<_> = mirrored.iter().take(PORTS).collect();

    let _ = write!(
        b,
        r##"<rect width="{w}" height="{h}" fill="{}"/><rect x="24" y="20" width="14" height="{}" fill="#14162a"/><rect x="{}" y="20" width="14" height="{}" fill="#14162a"/>"##,
        pal::BG_DEEP,
        h - 40.0,
        w - 38.0,
        h - 40.0
    );
    let mut y = 30.0;
    while y < h - 20.0 {
        let _ = write!(
            b,
            r##"<circle cx="31" cy="{y}" r="2" fill="#0a0b16"/><circle cx="{}" cy="{y}" r="2" fill="#0a0b16"/>"##,
            w - 31.0
        );
        y += 16.0;
    }
    let (ux, uw) = (44.0, w - 88.0);
    let mut ports: BTreeMap<usize, Vec<(f64, f64)>> = BTreeMap::new();
    for (i, p) in units.iter().enumerate() {
        let y = 26.0 + i as f64 * (UH + 10.0);
        let _ = write!(
            b,
            r#"<rect x="{ux}" y="{y}" width="{uw}" height="{UH}" rx="6" fill="url(#metal)" stroke="{}"/>"#,
            pal::LINE
        );
        let led = if p.reachable { "#4ade80" } else { "#f7768e" };
        let _ = write!(
            b,
            r#"<circle class="led" style="animation-delay:{}s" cx="{}" cy="{}" r="4.5" fill="{led}"/>"#,
            i as f64 * 0.4,
            ux + 20.0,
            y + 22.0
        );
        let name_t = T::new(Font::ZenBlack, 20.0).fill(brand(&p.id));
        b.push_str(&doc.text(ux + 34.0, y + 28.0, &p.display, &name_t));
        if p.primary {
            let px = ux + 44.0 + typo::width(&p.display, &name_t);
            let _ = write!(
                b,
                r#"<rect x="{px}" y="{}" width="58" height="16" rx="8" fill="none" stroke="{}"/>"#,
                y + 15.0,
                pal::TEAL
            );
            b.push_str(&doc.text(
                px + 29.0,
                y + 26.5,
                "PRIMARY",
                &T::new(Font::MonoBold, 8.5).fill(pal::TEAL).ls(1.0).mid(),
            ));
        }
        let host = p
            .repos
            .first()
            .map(|r| r.html_url.clone())
            .unwrap_or_default();
        let host = host
            .split('/')
            .take(4)
            .collect::<Vec<_>>()
            .join("/")
            .replace("https://", "");
        let ht = T::new(Font::Mono, 10.0).fill(pal::DIM);
        b.push_str(&doc.text(ux + 34.0, y + 48.0, &typo::fit(&host, &ht, 200.0), &ht));
        let commits = p
            .rollup
            .commits
            .y1
            .map(kit::group)
            .unwrap_or_else(|| "·".into());
        for (k, (lab, val)) in [
            ("STARS", kit::group(p.rollup.stars)),
            ("REPOS", kit::group(p.rollup.repo_count)),
            ("COMMITS / Y", commits),
        ]
        .iter()
        .enumerate()
        {
            let x = ux + 250.0 + k as f64 * 106.0;
            b.push_str(&doc.text(
                x,
                y + 30.0,
                val,
                &T::new(Font::MonoBold, 20.0).fill(pal::FG_HI),
            ));
            b.push_str(&doc.text(
                x,
                y + 48.0,
                lab,
                &T::new(Font::Mono, 9.0).fill(pal::DIM).ls(1.0),
            ));
        }
        for k in 0..PORTS {
            let x = ux + 590.0 + k as f64 * 82.0;
            let on = shown.get(k).is_some_and(|g| g.1.contains(&i));
            let _ = write!(
                b,
                r##"<rect x="{}" y="{}" width="22" height="14" rx="3" fill="#0a0b16" stroke="{}" stroke-opacity="{}"/>"##,
                x - 11.0,
                y + 16.0,
                if on { pal::TEAL } else { pal::LINE },
                if on { ".9" } else { ".5" }
            );
            if on {
                let _ = write!(
                    b,
                    r#"<circle cx="{x}" cy="{}" r="3.5" fill="{}"/>"#,
                    y + 23.0,
                    pal::TEAL
                );
                ports.entry(k).or_default().push((x, y + 30.0));
            }
        }
    }
    for ps in ports.values() {
        for pair in ps.windows(2) {
            let ((x1, y1), (x2, y2)) = (pair[0], pair[1]);
            let d = format!(
                "M{x1} {y1} C {} {}, {} {}, {x2} {}",
                x1 + 26.0,
                y1 + 30.0,
                x2 + 26.0,
                y2 - 30.0,
                y2 - 14.0
            );
            let _ = write!(
                b,
                r##"<path d="{d}" fill="none" stroke="{}" stroke-width="2.2" stroke-opacity=".75"/><path class="flow" d="{d}" fill="none" stroke="#ffffff" stroke-width="1.2" stroke-opacity=".7"/>"##,
                pal::SAKURA
            );
        }
    }

    let y = 26.0 + units.len() as f64 * (UH + 10.0);
    let _ = write!(
        b,
        r#"<rect x="{ux}" y="{y}" width="{uw}" height="{}" rx="6" fill="{}" stroke="{}" stroke-opacity=".6"/>"#,
        UH - 16.0,
        pal::BG,
        pal::TEAL
    );
    b.push_str(&doc.text(
        ux + 20.0,
        y + 30.0,
        "COMBINED",
        &T::new(Font::MonoBold, 11.0).fill(pal::TEAL).ls(2.0),
    ));
    for (k, (lab, val)) in [
        ("STARS", kit::group(agg.combined.stars)),
        ("REPOS", kit::group(agg.combined.repo_count)),
        ("MIRRORED", mirrored.len().to_string()),
    ]
    .iter()
    .enumerate()
    {
        let x = ux + 250.0 + k as f64 * 106.0;
        b.push_str(&doc.text(
            x,
            y + 30.0,
            val,
            &T::new(Font::MonoBold, 20.0).fill(pal::TEAL),
        ));
        b.push_str(&doc.text(
            x,
            y + 46.0,
            lab,
            &T::new(Font::Mono, 9.0).fill(pal::DIM).ls(1.0),
        ));
    }
    for (k, g) in shown.iter().enumerate() {
        let x = ux + 590.0 + k as f64 * 82.0;
        let nt = T::new(Font::Mono, 8.5).fill(pal::FG).mid();
        b.push_str(&doc.text(x, y + 28.0, &typo::fit(&g.0, &nt, 78.0), &nt));
        b.push_str(&doc.text(
            x,
            y + 44.0,
            &format!("× {}", g.1.len()),
            &T::new(Font::MonoBold, 10.0).fill(pal::SAKURA).mid(),
        ));
    }

    let frame = format!(r#"<g clip-path="url(#card)">{b}</g>"#);
    let css = ".led{animation:led 1.6s steps(1) infinite}@keyframes led{50%{opacity:.35}}.flow{stroke-dasharray:4 8;animation:flow 1.6s linear infinite}@keyframes flow{to{stroke-dashoffset:-24}}";
    let defs = format!(
        r##"<clipPath id="card"><rect width="{w}" height="{h}" rx="14"/></clipPath><linearGradient id="metal" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#191b30"/><stop offset="1" stop-color="#101122"/></linearGradient>"##
    );
    let names: Vec<&str> = units.iter().map(|p| p.display.as_str()).collect();
    let label = format!(
        "Platforms: {}, mirrors linked and counted once",
        names.join(", ")
    );
    Ok(vec![(
        "platforms.svg".into(),
        kit::svg(w, h, &label, css, &defs, &doc, &frame),
    )])
}
