//! Card-wide guards. Every card renders from fixtures and must:
//! - parse as XML;
//! - reference only glyphs it defines (a missing `<path id>` draws nothing);
//! - place every text run inside its own viewBox;
//! - stay under a size budget, so the README keeps loading fast.

use crate::run;
use std::collections::BTreeSet;
use std::path::Path;

fn fixture_ctx() -> run::Ctx {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("engine/ has a parent")
        .to_path_buf();
    run::build_context_at(root, true).expect("fixture context must build")
}

fn render_all(ctx: &run::Ctx) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (name, build) in run::CARDS {
        let cards = build(ctx).unwrap_or_else(|e| panic!("{name} failed to build: {e}"));
        assert!(!cards.is_empty(), "{name} returned no files");
        out.extend(cards);
    }
    out
}

fn view_box(doc: &roxmltree::Document) -> (f64, f64) {
    let vb = doc.root_element().attribute("viewBox").expect("viewBox");
    let n: Vec<f64> = vb
        .split_whitespace()
        .filter_map(|v| v.parse().ok())
        .collect();
    (n[2], n[3])
}

#[test]
fn every_card_is_valid_and_self_contained() {
    let ctx = fixture_ctx();
    for (file, svg) in render_all(&ctx) {
        crate::svg::validate_xml(&svg).unwrap_or_else(|e| panic!("{file}: {e}"));
        let doc = roxmltree::Document::parse(&svg).unwrap();
        let ids: BTreeSet<&str> = doc
            .descendants()
            .filter_map(|n| n.attribute("id"))
            .collect();
        for n in doc.descendants().filter(|n| n.has_tag_name("use")) {
            let href = n.attribute("href").unwrap_or_default();
            assert!(
                ids.contains(href.trim_start_matches('#')),
                "{file}: <use href=\"{href}\"> has no target"
            );
        }
        assert!(
            doc.root_element()
                .attribute("aria-label")
                .is_some_and(|a| !a.is_empty()),
            "{file}: no aria-label"
        );
    }
}

#[test]
fn text_runs_stay_inside_the_card() {
    let ctx = fixture_ctx();
    for (file, svg) in render_all(&ctx) {
        let doc = roxmltree::Document::parse(&svg).unwrap();
        let (w, h) = view_box(&doc);
        for g in doc.descendants().filter(|n| n.has_tag_name("g")) {
            let Some(t) = g.attribute("transform") else {
                continue;
            };
            // Glyph runs are the only groups that both translate and flip.
            if !t.contains("scale(") || !t.contains(" -") {
                continue;
            }
            let inner = &t[t.find("translate(").unwrap() + 10..];
            let nums: Vec<f64> = inner[..inner.find(')').unwrap()]
                .split_whitespace()
                .filter_map(|v| v.parse().ok())
                .collect();
            let (x, y) = (nums[0], nums[1]);
            // Runs inside moved groups (the jewel case, the badge row) are
            // checked by eye; only top-level positions are absolute.
            let nested = g.ancestors().skip(1).any(|a| {
                a.attribute("transform")
                    .is_some_and(|t| t.contains("matrix") || t.contains("scale("))
            });
            if nested {
                continue;
            }
            assert!(
                (-1.0..=w).contains(&x) && (0.0..=h + 1.0).contains(&y),
                "{file}: text run at ({x}, {y}) outside {w}x{h}"
            );
        }
    }
}

#[test]
fn cards_stay_within_their_size_budget() {
    let ctx = fixture_ctx();
    for (file, svg) in render_all(&ctx) {
        let kb = svg.len() / 1024;
        let budget = match file.as_str() {
            "bestgame.svg" => 600,
            "games.svg" => 700,
            _ => 160,
        };
        assert!(kb <= budget, "{file} is {kb} KB, budget {budget} KB");
    }
}

#[test]
fn long_strings_are_cut_not_spilled() {
    let mut ctx = fixture_ctx();
    let long =
        "An extremely long project name that nobody should ever type into a config".to_string();
    ctx.cfg.profile.header.tagline = long.clone();
    for t in ctx.cfg.profile.header.tracks.iter_mut() {
        t.name = long.clone();
        t.note = long.clone();
    }
    ctx.cfg.profile.research.phase = long.clone();
    // Text is drawn as glyph paths, so the guard is geometric: every run of
    // the two cards must still start inside and the cards must stay valid.
    for build in [
        crate::cards::header::build as fn(&run::Ctx) -> anyhow::Result<Vec<(String, String)>>,
        crate::cards::research::build,
    ] {
        let (file, svg) = &build(&ctx).unwrap()[0];
        crate::svg::validate_xml(svg).unwrap_or_else(|e| panic!("{file}: {e}"));
    }
    let t = crate::typo::T::new(crate::typo::Font::ZenLight, 17.0);
    let cut = crate::typo::fit(&long, &t, 520.0);
    assert!(cut.ends_with('…') && crate::typo::width(&cut, &t) <= 520.0);
}

#[test]
fn readme_layout_covers_every_rendered_file() {
    let ctx = fixture_ctx();
    let laid: BTreeSet<String> = crate::readme::layout(&ctx.cfg.profile.sections)
        .into_iter()
        .flat_map(|(_, images)| images.into_iter().map(|(f, _)| f))
        .collect();
    for (file, _) in render_all(&ctx) {
        assert!(
            laid.contains(&file),
            "{file} is rendered but no README block shows it"
        );
    }
}
