//! Typography: real fonts, drawn as paths.
//!
//! Web fonts never load inside a GitHub-proxied SVG, and a font embedded as a
//! data URI costs more than the few dozen glyphs a card actually uses. So the
//! engine shapes every string with rustybuzz (kerning, ligatures, marks) and
//! writes each glyph it needs once into `<defs>` as a path; text is then a
//! row of `<use>` references. The result looks identical everywhere and, as a
//! bonus, every width is known exactly, so layout never guesses.
//!
//! Fonts live in `engine/fonts` (OFL, see the README there). A char missing
//! from a face falls back along the family's chain, ending in Zen Kaku Gothic
//! New, which covers Latin, Cyrillic, kana and kanji.

use crate::svg::esc;
use rustybuzz::ttf_parser::{GlyphId, OutlineBuilder};
use rustybuzz::{Face, UnicodeBuffer};
use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::PathBuf;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Font {
    ZenLight,
    Zen,
    ZenBlack,
    Mono,
    MonoBold,
    Unbounded,
    UnboundedBold,
    Stencil,
    Pixel,
    PixelBold,
    Pt,
    PtBold,
    PtMono,
    Caveat,
    CaveatBold,
    Silk,
    Saira,
    SairaBold,
    SairaBlack,
    Michroma,
}

const ALL: [Font; 20] = [
    Font::ZenLight,
    Font::Zen,
    Font::ZenBlack,
    Font::Mono,
    Font::MonoBold,
    Font::Unbounded,
    Font::UnboundedBold,
    Font::Stencil,
    Font::Pixel,
    Font::PixelBold,
    Font::Pt,
    Font::PtBold,
    Font::PtMono,
    Font::Caveat,
    Font::CaveatBold,
    Font::Silk,
    Font::Saira,
    Font::SairaBold,
    Font::SairaBlack,
    Font::Michroma,
];

impl Font {
    fn file(self) -> &'static str {
        match self {
            Font::ZenLight => "ZenKakuGothicNew-300.ttf",
            Font::Zen => "ZenKakuGothicNew-500.ttf",
            Font::ZenBlack => "ZenKakuGothicNew-900.ttf",
            Font::Mono => "JetBrainsMono-400.ttf",
            Font::MonoBold => "JetBrainsMono-700.ttf",
            Font::Unbounded => "Unbounded-400.ttf",
            Font::UnboundedBold => "Unbounded-800.ttf",
            Font::Stencil => "BigShouldersStencil-800.ttf",
            Font::Pixel => "PixelifySans-500.ttf",
            Font::PixelBold => "PixelifySans-700.ttf",
            Font::Pt => "PTSans-400.ttf",
            Font::PtBold => "PTSans-700.ttf",
            Font::PtMono => "PTMono-400.ttf",
            Font::Caveat => "Caveat-500.ttf",
            Font::CaveatBold => "Caveat-700.ttf",
            Font::Silk => "Silkscreen-400.ttf",
            Font::Saira => "SairaCondensed-500.ttf",
            Font::SairaBold => "SairaCondensed-700.ttf",
            Font::SairaBlack => "SairaCondensed-800.ttf",
            Font::Michroma => "Michroma-400.ttf",
        }
    }

    /// Short id used in glyph element ids.
    fn tag(self) -> usize {
        self as usize
    }

    /// Where a missing char goes next. Every chain ends in a Zen weight.
    fn fallback(self) -> Option<Font> {
        match self {
            Font::ZenLight => Some(Font::Zen),
            Font::Zen | Font::ZenBlack => None,
            Font::Mono
            | Font::PtMono
            | Font::Pt
            | Font::Pixel
            | Font::Silk
            | Font::Caveat
            | Font::Saira
            | Font::Michroma
            | Font::Unbounded => Some(Font::Zen),
            Font::MonoBold
            | Font::PtBold
            | Font::PixelBold
            | Font::CaveatBold
            | Font::SairaBold
            | Font::SairaBlack
            | Font::UnboundedBold
            | Font::Stencil => Some(Font::ZenBlack),
        }
    }
}

fn fonts_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("VAI_FONTS_DIR") {
        return PathBuf::from(dir);
    }
    if let Ok(root) = crate::paths::repo_root() {
        let dir = root.join("engine").join("fonts");
        if dir.is_dir() {
            return dir;
        }
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fonts")
}

struct Loaded {
    face: Face<'static>,
    upem: f64,
}

fn faces() -> &'static Vec<Option<Loaded>> {
    static FACES: OnceLock<Vec<Option<Loaded>>> = OnceLock::new();
    FACES.get_or_init(|| {
        let dir = fonts_dir();
        ALL.iter()
            .map(|f| {
                let path = dir.join(f.file());
                let bytes = match std::fs::read(&path) {
                    Ok(b) => b,
                    Err(e) => {
                        crate::log::warn(&format!("font {} missing: {e}", path.display()));
                        return None;
                    }
                };
                let data: &'static [u8] = Box::leak(bytes.into_boxed_slice());
                let face = Face::from_slice(data, 0)?;
                let upem = face.units_per_em() as f64;
                Some(Loaded { face, upem })
            })
            .collect()
    })
}

fn loaded(font: Font) -> Option<&'static Loaded> {
    faces()[font as usize].as_ref()
}

/// The face that will draw `c`, walking the fallback chain.
fn face_for(font: Font, c: char) -> Option<Font> {
    let mut f = Some(font);
    while let Some(cur) = f {
        if let Some(l) = loaded(cur) {
            if c.is_whitespace() || l.face.glyph_index(c).is_some() {
                return Some(cur);
            }
        }
        f = cur.fallback();
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Anchor {
    Start,
    Middle,
    End,
}

/// How a string is set. Built with chained setters:
/// `T::new(Font::ZenBlack, 58.0).fill("#fff").ls(-1.0)`.
#[derive(Debug, Clone)]
pub struct T {
    pub font: Font,
    pub size: f64,
    pub fill: String,
    pub anchor: Anchor,
    /// Extra space after every glyph, in px (CSS letter-spacing).
    pub ls: f64,
    pub opacity: Option<f64>,
    pub class: Option<String>,
    pub style: Option<String>,
    /// Horizontal skew in degrees, for a slanted look without an italic face.
    pub skew: f64,
}

impl T {
    pub fn new(font: Font, size: f64) -> Self {
        T {
            font,
            size,
            fill: "#ffffff".into(),
            anchor: Anchor::Start,
            ls: 0.0,
            opacity: None,
            class: None,
            style: None,
            skew: 0.0,
        }
    }
    pub fn fill(mut self, c: &str) -> Self {
        self.fill = c.to_string();
        self
    }
    pub fn ls(mut self, px: f64) -> Self {
        self.ls = px;
        self
    }
    pub fn mid(mut self) -> Self {
        self.anchor = Anchor::Middle;
        self
    }
    pub fn end(mut self) -> Self {
        self.anchor = Anchor::End;
        self
    }
    pub fn op(mut self, o: f64) -> Self {
        self.opacity = Some(o);
        self
    }
    pub fn class(mut self, c: &str) -> Self {
        self.class = Some(c.to_string());
        self
    }
    pub fn style(mut self, s: &str) -> Self {
        self.style = Some(s.to_string());
        self
    }
    pub fn skew(mut self, deg: f64) -> Self {
        self.skew = deg;
        self
    }
}

struct Placed {
    font: Font,
    gid: u16,
    x: f64,
    y: f64,
}

/// Shape `text` and return glyph placements (px, relative to the pen start)
/// plus the advance width.
fn layout(text: &str, font: Font, size: f64, ls: f64) -> (Vec<Placed>, f64) {
    let mut runs: Vec<(Font, String)> = Vec::new();
    for c in text.chars() {
        let f = face_for(font, c).unwrap_or(font);
        match runs.last_mut() {
            Some((rf, s)) if *rf == f => s.push(c),
            _ => runs.push((f, c.to_string())),
        }
    }
    let mut out = Vec::new();
    let mut pen = 0.0;
    for (f, run) in runs {
        let Some(l) = loaded(f) else {
            pen += size * 0.55 * run.chars().count() as f64;
            continue;
        };
        let k = size / l.upem;
        let mut buf = UnicodeBuffer::new();
        buf.push_str(&run);
        let shaped = rustybuzz::shape(&l.face, &[], buf);
        for (info, pos) in shaped.glyph_infos().iter().zip(shaped.glyph_positions()) {
            out.push(Placed {
                font: f,
                gid: info.glyph_id as u16,
                x: pen + pos.x_offset as f64 * k,
                y: -(pos.y_offset as f64) * k,
            });
            pen += pos.x_advance as f64 * k + ls;
        }
    }
    if !out.is_empty() {
        pen -= ls;
    }
    (out, pen.max(0.0))
}

/// Exact advance width of `text` set in `t`.
pub fn width(text: &str, t: &T) -> f64 {
    layout(text, t.font, t.size, t.ls).1
}

/// Cut `text` to fit `max_w`, ending in an ellipsis when it had to cut.
pub fn fit(text: &str, t: &T, max_w: f64) -> String {
    if width(text, t) <= max_w {
        return text.to_string();
    }
    let chars: Vec<char> = text.chars().collect();
    let mut n = chars.len();
    while n > 0 {
        let cand: String = chars[..n].iter().collect::<String>().trim_end().to_string() + "…";
        if width(&cand, t) <= max_w {
            return cand;
        }
        n -= 1;
    }
    String::new()
}

struct PathOut(String);

impl OutlineBuilder for PathOut {
    fn move_to(&mut self, x: f32, y: f32) {
        let _ = write!(self.0, "M{} {}", x.round(), y.round());
    }
    fn line_to(&mut self, x: f32, y: f32) {
        let _ = write!(self.0, "L{} {}", x.round(), y.round());
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let _ = write!(
            self.0,
            "Q{} {} {} {}",
            x1.round(),
            y1.round(),
            x.round(),
            y.round()
        );
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let _ = write!(
            self.0,
            "C{} {} {} {} {} {}",
            x1.round(),
            y1.round(),
            x2.round(),
            y2.round(),
            x.round(),
            y.round()
        );
    }
    fn close(&mut self) {
        self.0.push('Z');
    }
}

fn num(v: f64) -> String {
    let r = (v * 10.0).round() / 10.0;
    if r == r.trunc() {
        format!("{}", r as i64)
    } else {
        format!("{r}")
    }
}

/// One SVG document's glyph cache. Every card makes one, draws text through
/// it, and puts `defs()` into its `<defs>`.
#[derive(Default)]
pub struct Doc {
    glyphs: BTreeMap<(Font, u16), Option<String>>,
}

impl Doc {
    pub fn new() -> Self {
        Self::default()
    }

    fn glyph_id(&mut self, font: Font, gid: u16) -> Option<String> {
        let entry = self.glyphs.entry((font, gid)).or_insert_with(|| {
            let l = loaded(font)?;
            let mut p = PathOut(String::new());
            l.face.outline_glyph(GlyphId(gid), &mut p)?;
            Some(p.0)
        });
        entry.as_ref().map(|_| format!("g{}_{}", font.tag(), gid))
    }

    /// Draw `text` with its baseline at (x, y).
    pub fn text(&mut self, x: f64, y: f64, text: &str, t: &T) -> String {
        self.spans(x, y, &[(text, t)])
    }

    /// Draw consecutive runs, each in its own style, on one baseline. The
    /// anchor of the first style positions the whole line.
    pub fn spans(&mut self, x: f64, y: f64, runs: &[(&str, &T)]) -> String {
        let laid: Vec<(Vec<Placed>, f64)> = runs
            .iter()
            .map(|(s, t)| layout(s, t.font, t.size, t.ls))
            .collect();
        let total: f64 = laid.iter().map(|l| l.1).sum::<f64>()
            + runs.iter().skip(1).map(|(_, t)| t.ls).sum::<f64>();
        let anchor = runs.first().map(|r| r.1.anchor).unwrap_or(Anchor::Start);
        let mut pen = match anchor {
            Anchor::Start => x,
            Anchor::Middle => x - total / 2.0,
            Anchor::End => x - total,
        };
        let mut out = String::new();
        for ((placed, w), (_, t)) in laid.into_iter().zip(runs.iter()) {
            out.push_str(&self.emit(pen, y, &placed, t));
            pen += w + t.ls;
        }
        out
    }

    fn emit(&mut self, x0: f64, y: f64, placed: &[Placed], t: &T) -> String {
        let mut by_font: BTreeMap<Font, Vec<&Placed>> = BTreeMap::new();
        for p in placed {
            by_font.entry(p.font).or_default().push(p);
        }
        let mut attrs = format!(r#" fill="{}""#, t.fill);
        if let Some(o) = t.opacity {
            let _ = write!(attrs, r#" opacity="{}""#, num(o));
        }
        if let Some(c) = &t.class {
            let _ = write!(attrs, r#" class="{}""#, esc(c));
        }
        if let Some(s) = &t.style {
            let _ = write!(attrs, r#" style="{}""#, esc(s));
        }
        let mut out = format!("<g{attrs}>");
        for (font, ps) in by_font {
            let Some(l) = loaded(font) else { continue };
            let k = t.size / l.upem;
            let skew = if t.skew != 0.0 {
                format!(" skewX({})", num(t.skew))
            } else {
                String::new()
            };
            let _ = write!(
                out,
                r#"<g transform="translate({} {}){skew} scale({:.5} -{:.5})">"#,
                num(x0),
                num(y),
                k,
                k
            );
            for p in ps {
                if let Some(id) = self.glyph_id(font, p.gid) {
                    let ux = (p.x / k).round();
                    let uy = (-p.y / k).round();
                    if uy == 0.0 {
                        let _ = write!(out, r##"<use href="#{id}" x="{ux}"/>"##);
                    } else {
                        let _ = write!(out, r##"<use href="#{id}" x="{ux}" y="{uy}"/>"##);
                    }
                }
            }
            out.push_str("</g>");
        }
        out.push_str("</g>");
        out
    }

    /// Stack the chars of `text` vertically, centred on `x`, for Japanese
    /// written top to bottom.
    pub fn vertical(&mut self, x: f64, y: f64, text: &str, t: &T, step: f64) -> String {
        let mut out = String::new();
        let one = T {
            anchor: Anchor::Middle,
            ..t.clone()
        };
        for (i, c) in text.chars().enumerate() {
            out.push_str(&self.text(x, y + i as f64 * step, &c.to_string(), &one));
        }
        out
    }

    /// Glyph outlines for `<defs>`.
    pub fn defs(&self) -> String {
        let mut out = String::new();
        for ((font, gid), path) in &self.glyphs {
            if let Some(d) = path {
                let _ = write!(out, r#"<path id="g{}_{}" d="{d}"/>"#, font.tag(), gid);
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_bundled_font_loads() {
        for f in ALL {
            assert!(loaded(f).is_some(), "{} did not load", f.file());
        }
    }

    #[test]
    fn widths_are_exact_and_monotonic() {
        let t = T::new(Font::Mono, 10.0);
        let one = width("a", &t);
        assert!(
            (width("aaaa", &t) - 4.0 * one).abs() < 0.01,
            "monospace advances add up"
        );
        assert!(width("Vadim Khristenko", &T::new(Font::ZenBlack, 58.0)) > 300.0);
        assert!(width("ab", &t.clone().ls(2.0)) > width("ab", &t));
    }

    #[test]
    fn missing_glyphs_fall_back_to_zen() {
        // Michroma has no Cyrillic or kana; both must still draw.
        let mut doc = Doc::new();
        let g = doc.text(0.0, 0.0, "Кумир ミク", &T::new(Font::Michroma, 12.0));
        assert!(g.contains(&format!("g{}_", Font::Zen.tag())));
        assert!(!doc.defs().is_empty());
    }

    #[test]
    fn fit_cuts_with_ellipsis() {
        let t = T::new(Font::Zen, 12.0);
        let s = fit("a very long project description that cannot fit", &t, 80.0);
        assert!(s.ends_with('…'));
        assert!(width(&s, &t) <= 80.0);
        assert_eq!(fit("short", &t, 200.0), "short");
    }

    #[test]
    fn anchors_shift_the_line() {
        let mut doc = Doc::new();
        let t = T::new(Font::Mono, 10.0);
        let w = width("abc", &t);
        let mid = doc.text(100.0, 10.0, "abc", &t.clone().mid());
        assert!(mid.contains(&format!("translate({} 10)", num(100.0 - w / 2.0))));
    }
}
