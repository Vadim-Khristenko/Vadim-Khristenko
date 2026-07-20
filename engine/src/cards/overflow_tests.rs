//! Card-wide overflow guard: renders EVERY card from fixtures, then walks each
//! `<text>` element, estimates its extents with the same advance model the
//! cards use for fitting (`svg::text_width_px`), and asserts nothing crosses
//! the card boundary — horizontally or vertically.
//!
//! This is a best-effort approximation (SVG text metrics depend on the
//! viewer's fonts), but because the layout code fits text with the *same*
//! model, a pass here means the invariant holds by construction.

use crate::run;
use crate::svg::text_width_px;
use std::path::Path;

fn fixture_ctx() -> run::Ctx {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("engine/ has a parent")
        .to_path_buf();
    run::build_context_at(root, true).expect("fixture context must build")
}

/// Sum of every `translate(x[,y])` in a transform attribute value.
fn translate_of(transform: &str) -> (f64, f64) {
    let (mut ox, mut oy) = (0.0, 0.0);
    let mut rest = transform;
    while let Some(pos) = rest.find("translate(") {
        rest = &rest[pos + "translate(".len()..];
        let Some(end) = rest.find(')') else { break };
        let args = &rest[..end];
        let mut parts = args.split([',', ' ']).filter(|s| !s.trim().is_empty());
        ox += parts.next().and_then(|v| v.trim().parse::<f64>().ok()).unwrap_or(0.0);
        oy += parts.next().and_then(|v| v.trim().parse::<f64>().ok()).unwrap_or(0.0);
        rest = &rest[end..];
    }
    (ox, oy)
}

fn attr_f64(node: &roxmltree::Node, name: &str, default: f64) -> f64 {
    node.attribute(name)
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(default)
}

/// Assert every text element in `svg` stays inside the card bounds.
fn assert_no_overflow(file: &str, svg: &str) {
    let doc = roxmltree::Document::parse(svg).unwrap_or_else(|e| panic!("{file}: bad XML: {e}"));
    let root = doc.root_element();
    let w = root
        .attribute("width")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or_else(|| panic!("{file}: no width"));
    let h = root
        .attribute("height")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or_else(|| panic!("{file}: no height"));

    let mut checked = 0usize;
    for node in doc.descendants().filter(|n| n.has_tag_name("text")) {
        // Accumulated translate() from the element and its ancestors
        // (rotate/scale are only used on decorative non-text shapes).
        let (mut ox, mut oy) = (0.0, 0.0);
        for anc in node.ancestors() {
            if let Some(tr) = anc.attribute("transform") {
                let (tx, ty) = translate_of(tr);
                ox += tx;
                oy += ty;
            }
        }
        let x = attr_f64(&node, "x", 0.0) + ox;
        let y = attr_f64(&node, "y", 0.0) + oy;
        let fs = attr_f64(&node, "font-size", 12.0);
        let ls = attr_f64(&node, "letter-spacing", 0.0);
        let mono = node
            .attribute("font-family")
            .map_or(true, |f| f.to_lowercase().contains("mono"));
        let anchor = node.attribute("text-anchor").unwrap_or("start");
        // Direct + descendant text (skips empty <animate> children cleanly).
        let content: String = node
            .descendants()
            .filter(|n| n.is_text())
            .filter_map(|n| n.text())
            .collect();
        let content = content.trim();
        if content.is_empty() {
            continue;
        }
        checked += 1;

        let tw = text_width_px(content, fs, mono)
            + ls * content.chars().count().saturating_sub(1) as f64;
        let (x0, x1) = match anchor {
            "end" => (x - tw, x),
            "middle" => (x - tw / 2.0, x + tw / 2.0),
            _ => (x, x + tw),
        };
        assert!(
            x0 >= 1.0 && x1 <= w - 1.0,
            "{file}: text {content:?} (fs {fs}, anchor {anchor}) spans x {x0:.1}..{x1:.1} \
             outside the card 0..{w:.0}"
        );
        // Baseline model: ascent ≈ 0.78 em above y, descent ≈ 0.22 em below.
        let (y0, y1) = (y - 0.78 * fs, y + 0.22 * fs);
        assert!(
            y0 >= 0.0 && y1 <= h,
            "{file}: text {content:?} (fs {fs}) spans y {y0:.1}..{y1:.1} \
             outside the card 0..{h:.0}"
        );
    }
    // Some dividers are pure geometry; only insist on coverage when the
    // document actually contains text markup.
    if svg.contains("<text") {
        assert!(checked > 0, "{file}: text present but none checked — parser broken?");
    }
}

#[test]
fn no_text_escapes_any_card() {
    let ctx = fixture_ctx();
    for (name, builder) in run::CARDS {
        let outputs = builder(&ctx).unwrap_or_else(|e| panic!("card {name} failed: {e:#}"));
        assert!(!outputs.is_empty(), "card {name} rendered nothing");
        for (file, svg) in outputs {
            crate::svg::validate_xml(&svg).unwrap_or_else(|e| panic!("{file}: {e:#}"));
            assert_no_overflow(&file, &svg);
        }
    }
}

/// The fitting model must also hold under adversarially long config strings:
/// stretch the editorial fields and re-render the text-heavy cards.
#[test]
fn no_text_escapes_with_hostile_config_strings() {
    let mut ctx = fixture_ctx();
    let long = "An extremely long piece of configuration text that keeps going \
                and going far past any reasonable column width 0123456789";
    let p = &mut ctx.cfg.profile;
    p.name = format!("Vadim {long}");
    p.aliases = vec![format!("VAI_{long}")];
    p.research.name = format!("Aethelgard {long}");
    p.research.subtitle = long.into();
    p.quotes = vec![format!("“{long}”")];
    for g in &mut p.games {
        g.title = format!("{} {long}", g.title);
        g.note = long.into();
    }
    p.best_game.title = format!("NIKKE {long}");
    p.best_game.subtitle = long.to_uppercase();
    p.best_game.blurb = long.into();
    for f in ctx.flagship.iter_mut() {
        f.name = format!("{} {long}", f.name);
        f.blurb = long.into();
        f.tags = vec![long.into(); 4];
    }
    for (name, builder) in run::CARDS {
        let outputs = builder(&ctx).unwrap_or_else(|e| panic!("card {name} failed: {e:#}"));
        for (file, svg) in outputs {
            assert_no_overflow(&file, &svg);
        }
    }
}
