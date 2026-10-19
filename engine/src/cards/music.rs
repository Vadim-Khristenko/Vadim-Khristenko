//! Music as a Y2K CD on a night stage: a jewel case turned in 3D with the
//! cover inside, a holographic disc sliding out and spinning, the track title
//! extruded like a chrome logo, top artists as an isometric skyline whose
//! towers pulse, and recent scrobbles printed like a CD insert tracklist.
//! Live Last.fm data; without it the composers list from config fills in.

use super::kit;
use crate::run::Ctx;
use crate::typo::{self, Doc, Font, T};
use anyhow::Result;
use std::fmt::Write;

const H: f64 = 400.0;
const INK: &str = "#0b0a16";
const INK2: &str = "#05040b";
const CHROME: &str = "#eef0ff";
const MUTE: &str = "#8d89b8";
const HOLO: [&str; 5] = ["#ff7ac6", "#b28cff", "#6ee7f9", "#8affc1", "#ffe08a"];

struct Now {
    title: String,
    artist: String,
    label: &'static str,
    cover: Option<crate::img::Embedded>,
}

fn ago(now: i64, then: Option<i64>) -> String {
    let Some(t) = then else { return "now".into() };
    let m = ((now - t).max(0)) / 60;
    match m {
        0..=59 => format!("{m} min"),
        60..=1439 => format!("{} h", m / 60),
        _ => format!("{} d", m / 1440),
    }
}

/// Skyline: isometric prisms whose polygons SMIL-animate their height, since
/// CSS cannot move one face of a prism.
struct Iso {
    ox: f64,
    oy: f64,
    s: f64,
}

impl Iso {
    fn faces(&self, gx: f64, gy: f64, h: f64) -> [String; 3] {
        let (co, si, s) = (0.866, 0.5, self.s);
        let ox = self.ox + (gx - gy) * co * s;
        let oy = self.oy + (gx + gy) * si * s;
        let p = |pts: [(f64, f64); 4]| {
            pts.iter()
                .map(|(x, y)| format!("{:.1},{:.1}", x, y))
                .collect::<Vec<_>>()
                .join(" ")
        };
        [
            p([
                (ox, oy - h),
                (ox + co * s, oy + si * s - h),
                (ox, oy + s - h),
                (ox - co * s, oy + si * s - h),
            ]),
            p([
                (ox - co * s, oy + si * s),
                (ox, oy + s),
                (ox, oy + s - h),
                (ox - co * s, oy + si * s - h),
            ]),
            p([
                (ox, oy + s),
                (ox + co * s, oy + si * s),
                (ox + co * s, oy + si * s - h),
                (ox, oy + s - h),
            ]),
        ]
    }

    fn prism(&self, gx: f64, gy: f64, h: f64, col: &str, delay: f64) -> String {
        let (hi, lo) = (self.faces(gx, gy, h), self.faces(gx, gy, h * 0.8));
        let mut out = String::new();
        for (k, fill, op) in [(1, col, 0.55), (2, col, 0.85), (0, "#ffffff", 0.92)] {
            let _ = write!(
                out,
                r#"<polygon points="{a}" fill="{fill}" fill-opacity="{op}"><animate attributeName="points" values="{a};{b};{a}" dur="2.2s" begin="{delay}s" repeatCount="indefinite" calcMode="spline" keyTimes="0;.5;1" keySplines=".45 0 .55 1;.45 0 .55 1"/></polygon>"#,
                a = hi[k],
                b = lo[k]
            );
        }
        out
    }
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let p = &ctx.cfg.profile;
    let music = ctx.music.as_ref();
    let mut doc = Doc::new();
    let mut b = String::new();
    let w = kit::W;

    let now = match music.and_then(|m| m.track.as_ref()) {
        Some(t) => Now {
            title: t.name.clone(),
            artist: t.artist.clone(),
            label: if t.now_playing {
                "NOW PLAYING"
            } else {
                "LAST PLAYED"
            },
            cover: t
                .image
                .as_deref()
                .and_then(crate::img::fetch)
                .and_then(|bytes| crate::img::from_bytes(&bytes, 352, None).ok()),
        },
        None => {
            let c = p.composers.first();
            Now {
                title: c
                    .map(|c| c.name.clone())
                    .unwrap_or_else(|| "Hatsune Miku".into()),
                artist: c.map(|c| c.note.clone()).unwrap_or_default(),
                label: "ON LOOP",
                cover: None,
            }
        }
    };
    let live = now.label == "NOW PLAYING";
    let tops: Vec<(String, Option<u64>)> = match music {
        Some(m) if !m.top_artists.is_empty() => m
            .top_artists
            .iter()
            .take(5)
            .map(|(a, n)| (a.clone(), Some(*n)))
            .collect(),
        _ => p
            .composers
            .iter()
            .take(5)
            .map(|c| (c.name.clone(), None))
            .collect(),
    };

    let _ = write!(b, r#"<rect width="{w}" height="{H}" fill="url(#sky)"/>"#);
    let (vx, fy) = (500.0, 300.0);
    for i in -14..=14 {
        let _ = write!(
            b,
            r##"<line x1="{}" y1="{fy}" x2="{}" y2="{H}" stroke="#6b4bd6" stroke-opacity=".28"/>"##,
            vx + i as f64 * 8.0,
            vx + i as f64 * 120.0
        );
    }
    for k in 0..7 {
        let y = fy + (H - fy) * (k as f64 / 6.0).powf(1.8);
        let _ = write!(
            b,
            r##"<line x1="0" y1="{y:.1}" x2="{w}" y2="{y:.1}" stroke="#6b4bd6" stroke-opacity="{:.2}"/>"##,
            0.12 + k as f64 * 0.04
        );
    }
    let _ = write!(
        b,
        r##"<rect x="0" y="{}" width="{w}" height="40" fill="url(#floorfade)"/><line x1="0" y1="{fy}" x2="{w}" y2="{fy}" stroke="#ff7ac6" stroke-opacity=".35"/>"##,
        fy - 2.0
    );

    // jewel case and disc, turned in 3D with one shared matrix
    let mut wedges = String::new();
    for j in 0..12 {
        let t = j as f64 * 0.52;
        let _ = write!(
            wedges,
            r##"<path d="M0 0 L{:.1} {:.1} A80 80 0 0 1 {:.1} {:.1}z" fill="#ffffff" opacity="{}"/>"##,
            80.0 * t.cos(),
            80.0 * t.sin(),
            80.0 * (t + 0.26).cos(),
            80.0 * (t + 0.26).sin(),
            if j % 2 == 1 { ".22" } else { ".06" }
        );
    }
    let spin = if live { "spin" } else { "spin slow" };
    let _ = write!(
        b,
        r##"<g class="float"><g transform="matrix(0.93,-0.16,0.02,1,62,108)"><g class="slide"><g transform="translate(150 88)"><g clip-path="url(#disc)"><circle r="80" fill="url(#holo)"/><g class="{spin}">{wedges}<rect x="-80" y="-80" width="160" height="160" fill="none"/></g><circle r="80" fill="url(#hole)"/></g><circle r="80" fill="none" stroke="#ffffff" stroke-opacity=".5"/></g></g><rect x="-9" y="5" width="176" height="176" fill="#120f26"/><g clip-path="url(#case)">"##
    );
    match &now.cover {
        Some(e) => b.push_str(&e.tag("mCover", 0.0, 0.0, 176.0, 176.0, "")),
        None => {
            b.push_str(r#"<rect width="176" height="176" fill="url(#cover)"/>"#);
            for i in 0..5 {
                let _ = write!(
                    b,
                    r##"<circle cx="{}" cy="{}" r="{}" fill="none" stroke="#ffffff" stroke-opacity=".12"/>"##,
                    40 + i * 30,
                    60 + (i % 2) * 40,
                    18 + i * 6
                );
            }
            let ct = T::new(Font::ZenBlack, 22.0).fill("#ffffff");
            b.push_str(&doc.text(14.0, 150.0, &typo::fit(&now.title, &ct, 150.0), &ct));
            let ca = T::new(Font::Michroma, 8.0).fill("#ffffff").op(0.85);
            b.push_str(&doc.text(14.0, 166.0, &typo::fit(&now.artist, &ca, 150.0), &ca));
        }
    }
    b.push_str(r##"<g class="shine"><rect x="0" y="-20" width="60" height="220" fill="#ffffff" opacity=".18" transform="skewX(-20)"/></g></g><rect width="176" height="176" fill="url(#glass)" stroke="#ffffff" stroke-opacity=".55"/><rect width="12" height="176" fill="#ffffff" fill-opacity=".14"/></g></g>"##);
    let _ = write!(
        b,
        r##"<ellipse cx="160" cy="{}" rx="130" ry="10" fill="#000" opacity=".5"/>"##,
        fy + 18.0
    );

    // now playing and the extruded title
    let tx = 340.0;
    b.push_str(&doc.text(
        tx,
        64.0,
        now.label,
        &T::new(Font::Michroma, 10.0).fill(HOLO[2]).ls(3.0),
    ));
    if live {
        let eqx = tx + typo::width(now.label, &T::new(Font::Michroma, 10.0).ls(3.0)) + 12.0;
        for (i, c) in HOLO.iter().take(4).enumerate() {
            let _ = write!(
                b,
                r#"<rect class="eq" style="animation-delay:{:.2}s" x="{}" y="53" width="3.5" height="12" fill="{c}"/>"#,
                i as f64 * 0.15,
                eqx + i as f64 * 6.0
            );
        }
    }
    let mut size = 50.0;
    while size > 28.0 && typo::width(&now.title, &T::new(Font::ZenBlack, size)) > 340.0 {
        size -= 2.0;
    }
    let title = typo::fit(&now.title, &T::new(Font::ZenBlack, size), 340.0);
    for d in (1..=7).rev() {
        let dd = d as f64 * 0.9;
        b.push_str(
            &doc.text(
                tx + dd,
                118.0 + dd,
                &title,
                &T::new(Font::ZenBlack, size)
                    .fill("#3a1f7a")
                    .op(0.35 + (7 - d) as f64 * 0.08),
            ),
        );
    }
    b.push_str(&doc.text(
        tx,
        118.0,
        &title,
        &T::new(Font::ZenBlack, size).fill("url(#chrome)"),
    ));
    let at = T::new(Font::Michroma, 13.0).fill(CHROME).ls(2.0);
    b.push_str(&doc.text(tx + 2.0, 148.0, &typo::fit(&now.artist, &at, 300.0), &at));
    let _ = write!(
        b,
        r##"<rect x="{tx}" y="168" width="270" height="4" rx="2" fill="#ffffff" fill-opacity=".12"/><rect class="{}" x="{tx}" y="168" width="270" height="4" rx="2" fill="url(#holo)"/>"##,
        if live { "played" } else { "played done" }
    );
    let scrobbles = music
        .and_then(|m| m.total_scrobbles)
        .map(|n| format!("LAST.FM · {} SCROBBLES", kit::group(n)))
        .unwrap_or_else(|| "LAST.FM".into());
    let stale = if music.is_some_and(|m| m.stale) {
        " · CACHED"
    } else {
        ""
    };
    b.push_str(&doc.text(
        tx,
        192.0,
        &format!("{scrobbles}{stale}"),
        &T::new(Font::Michroma, 8.5).fill(MUTE).ls(1.5),
    ));

    let recent: Vec<_> = music
        .map(|m| m.recent.iter().take(3).collect())
        .unwrap_or_default();
    if !recent.is_empty() {
        b.push_str(&doc.text(
            tx,
            226.0,
            "RECENTLY",
            &T::new(Font::Michroma, 8.5).fill(MUTE).ls(2.0),
        ));
        let fetched = music.map(|m| m.fetched_at).unwrap_or(0);
        for (i, t) in recent.iter().enumerate() {
            let y = 248.0 + i as f64 * 20.0;
            b.push_str(&doc.text(
                tx,
                y,
                &format!("0{}", i + 1),
                &T::new(Font::Michroma, 9.0).fill(HOLO[i]),
            ));
            let nt = T::new(Font::ZenBlack, 13.0).fill(CHROME);
            b.push_str(&doc.text(tx + 26.0, y, &typo::fit(&t.name, &nt, 136.0), &nt));
            let art = T::new(Font::Zen, 12.0).fill(MUTE);
            b.push_str(&doc.text(tx + 170.0, y, &typo::fit(&t.artist, &art, 96.0), &art));
            if fetched > 0 {
                b.push_str(&doc.text(
                    tx + 290.0,
                    y,
                    &ago(fetched, t.played_at),
                    &T::new(Font::Michroma, 8.5).fill(MUTE).end(),
                ));
            }
        }
    }

    // isometric skyline
    let iso = Iso {
        ox: 836.0,
        oy: 214.0,
        s: 28.0,
    };
    for gx in 0..5 {
        for gy in 0..5 {
            let _ = write!(
                b,
                r##"<polygon points="{}" fill="#1a1440" stroke="#6b4bd6" stroke-opacity=".5"/>"##,
                iso.faces(gx as f64, gy as f64, 0.0)[0]
            );
        }
    }
    let spots = [(2.0, 2.0), (1.0, 3.0), (3.0, 1.0), (0.0, 4.0), (4.0, 0.0)];
    let peak = tops.iter().filter_map(|t| t.1).max().unwrap_or(1).max(1) as f64;
    for (i, (_, plays)) in tops.iter().enumerate() {
        let h = match plays {
            Some(n) => 22.0 + 110.0 * *n as f64 / peak,
            None => 120.0 - i as f64 * 20.0,
        };
        b.push_str(&iso.prism(spots[i].0, spots[i].1, h, HOLO[i], i as f64 * 0.3));
    }
    b.push_str(&doc.text(
        w - 26.0,
        40.0,
        if music.is_some() {
            "TOP ARTISTS · THIS MONTH"
        } else {
            "ON HEAVY LOOP"
        },
        &T::new(Font::Michroma, 8.5).fill(MUTE).ls(2.0).end(),
    ));
    for (i, (name, plays)) in tops.iter().enumerate() {
        let y = 62.0 + i as f64 * 18.0;
        let _ = write!(
            b,
            r#"<rect x="{}" y="{}" width="8" height="8" fill="{}"/>"#,
            w - 178.0,
            y - 8.0,
            HOLO[i]
        );
        let nt = T::new(Font::ZenBlack, 12.0).fill(CHROME);
        b.push_str(&doc.text(w - 164.0, y, &typo::fit(name, &nt, 110.0), &nt));
        if let Some(n) = plays {
            b.push_str(&doc.text(
                w - 26.0,
                y,
                &n.to_string(),
                &T::new(Font::Michroma, 9.0).fill(MUTE).end(),
            ));
        }
    }

    let frame = format!(
        r##"<g clip-path="url(#card)">{b}</g><rect x=".5" y=".5" width="{}" height="{}" rx="16" fill="none" stroke="#3a2f78"/>"##,
        w - 1.0,
        H - 1.0
    );
    let css = ".spin{transform-box:fill-box;transform-origin:center;animation:spin 5s linear infinite}.slow{animation-duration:30s}@keyframes spin{to{transform:rotate(360deg)}}.slide{animation:slide 6s cubic-bezier(.65,0,.35,1) infinite alternate}@keyframes slide{from{transform:translateX(-26px)}to{transform:translateX(0)}}.eq{transform-box:fill-box;transform-origin:bottom;animation:eq .8s ease-in-out infinite alternate}@keyframes eq{from{transform:scaleY(.2)}to{transform:scaleY(1)}}.shine{animation:shine 5s ease-in-out infinite}@keyframes shine{0%,100%{transform:translateX(-220px)}50%{transform:translateX(260px)}}.float{animation:float 6s ease-in-out infinite}@keyframes float{50%{transform:translateY(-6px)}}.played{transform-box:fill-box;transform-origin:left;transform:scaleX(.38);animation:played 40s linear infinite}.done{animation:none;transform:scaleX(1)}@keyframes played{from{transform:scaleX(.38)}to{transform:scaleX(1)}}";
    let holo: String = HOLO
        .iter()
        .enumerate()
        .map(|(i, c)| format!(r#"<stop offset="{:.2}" stop-color="{c}"/>"#, i as f64 / 4.0))
        .collect();
    let defs = format!(
        r##"<radialGradient id="sky" cx=".5" cy=".38" r=".8"><stop offset="0" stop-color="#1c1638"/><stop offset=".6" stop-color="{INK}"/><stop offset="1" stop-color="{INK2}"/></radialGradient><linearGradient id="holo" x1="0" y1="0" x2="1" y2="1">{holo}</linearGradient><linearGradient id="chrome" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#ffffff"/><stop offset=".48" stop-color="{CHROME}"/><stop offset=".52" stop-color="#b9bde6"/><stop offset="1" stop-color="#f6e9ff"/></linearGradient><linearGradient id="cover" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="#ff5fa8"/><stop offset=".5" stop-color="#5b2bd6"/><stop offset="1" stop-color="#14104a"/></linearGradient><linearGradient id="glass" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="#ffffff" stop-opacity=".28"/><stop offset=".35" stop-color="#ffffff" stop-opacity=".04"/><stop offset="1" stop-color="#ffffff" stop-opacity=".12"/></linearGradient><linearGradient id="floorfade" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="{INK2}"/><stop offset="1" stop-color="{INK2}" stop-opacity="0"/></linearGradient><radialGradient id="hole" cx=".5" cy=".5" r=".5"><stop offset=".22" stop-color="{INK}"/><stop offset=".26" stop-color="#d8dcf5"/><stop offset=".42" stop-color="#c3c7e6" stop-opacity=".6"/><stop offset=".44" stop-color="#ffffff" stop-opacity="0"/></radialGradient><clipPath id="card"><rect width="{w}" height="{H}" rx="16"/></clipPath><clipPath id="disc"><circle r="80"/></clipPath><clipPath id="case"><rect width="176" height="176"/></clipPath>"##
    );
    let label = format!(
        "Music: {} {} by {}",
        now.label.to_lowercase(),
        now.title,
        now.artist
    );
    Ok(vec![(
        "music.svg".into(),
        kit::svg(w, H, &label, css, &defs, &doc, &frame),
    )])
}
