//! Raster art for the cards, small enough to embed.
//!
//! A PNG or lossless WebP of a character portrait weighs as much as a whole
//! card, and the pure-Rust encoders available here have no lossy WebP. So art
//! is resized to the size it is drawn at (2x for sharp screens), stored as a
//! JPEG, and when it has transparency the alpha channel travels separately as
//! a tiny grayscale PNG used as an SVG `<mask>`. Same look, a fraction of the
//! bytes, no C dependencies.

use anyhow::{Context, Result};
use base64::Engine;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::{CompressionType, FilterType as PngFilter, PngEncoder};
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageEncoder, Rgba, RgbaImage};
use std::path::Path;

const QUALITY: u8 = 82;

/// An image ready to drop into an SVG.
pub struct Embedded {
    pub jpeg: String,
    /// Grayscale PNG of the alpha channel, when the art is not opaque.
    pub mask: Option<String>,
    pub w: u32,
    pub h: u32,
}

fn b64(bytes: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

fn resize(img: DynamicImage, max_w: u32) -> DynamicImage {
    if img.width() > max_w {
        let h = (img.height() as f64 * max_w as f64 / img.width() as f64).round() as u32;
        img.resize_exact(max_w, h.max(1), FilterType::Lanczos3)
    } else {
        img
    }
}

fn jpeg(rgb: &image::RgbImage) -> Result<String> {
    let mut buf = Vec::new();
    JpegEncoder::new_with_quality(&mut buf, QUALITY)
        .write_image(
            rgb.as_raw(),
            rgb.width(),
            rgb.height(),
            image::ExtendedColorType::Rgb8,
        )
        .context("encode jpeg")?;
    Ok(format!("data:image/jpeg;base64,{}", b64(&buf)))
}

fn alpha_png(rgba: &RgbaImage) -> Result<String> {
    // Sixteen steps of alpha look the same as 256 at these sizes and make the
    // mask compress several times better.
    let gray: Vec<u8> = rgba
        .pixels()
        .map(|p| ((p[3] as u16 + 8) / 17 * 17).min(255) as u8)
        .collect();
    let mut buf = Vec::new();
    PngEncoder::new_with_quality(&mut buf, CompressionType::Best, PngFilter::Adaptive)
        .write_image(
            &gray,
            rgba.width(),
            rgba.height(),
            image::ExtendedColorType::L8,
        )
        .context("encode mask")?;
    Ok(format!("data:image/png;base64,{}", b64(&buf)))
}

/// Encode an image at most `max_w` px wide. `flatten` paints transparency
/// onto that colour instead of keeping a mask (for art sitting on a plate).
pub fn encode(img: DynamicImage, max_w: u32, flatten: Option<[u8; 3]>) -> Result<Embedded> {
    let img = resize(img, max_w);
    let (w, h) = img.dimensions();
    let rgba = img.to_rgba8();
    let translucent = rgba.pixels().any(|p| p[3] < 250);
    let bg = flatten.unwrap_or([0, 0, 0]);
    let mut rgb = image::RgbImage::new(w, h);
    for (x, y, Rgba([r, g, b, a])) in rgba.enumerate_pixels() {
        let a = *a as u32;
        let mix = |c: u8, k: u8| ((c as u32 * a + k as u32 * (255 - a)) / 255) as u8;
        rgb.put_pixel(
            x,
            y,
            image::Rgb([mix(*r, bg[0]), mix(*g, bg[1]), mix(*b, bg[2])]),
        );
    }
    let mask = if translucent && flatten.is_none() {
        Some(alpha_png(&rgba)?)
    } else {
        None
    };
    Ok(Embedded {
        jpeg: jpeg(&rgb)?,
        mask,
        w,
        h,
    })
}

pub fn load(path: &Path, max_w: u32, flatten: Option<[u8; 3]>) -> Result<Embedded> {
    let img = image::open(path).with_context(|| format!("open {}", path.display()))?;
    encode(img, max_w, flatten)
}

pub fn from_bytes(bytes: &[u8], max_w: u32, flatten: Option<[u8; 3]>) -> Result<Embedded> {
    encode(
        image::load_from_memory(bytes).context("decode image")?,
        max_w,
        flatten,
    )
}

/// Parse "#rrggbb" for `flatten`.
pub fn rgb(hex: &str) -> [u8; 3] {
    let h = hex.trim_start_matches('#');
    let p = |i: usize| u8::from_str_radix(h.get(i..i + 2).unwrap_or("00"), 16).unwrap_or(0);
    [p(0), p(2), p(4)]
}

impl Embedded {
    /// `<image>` (plus its mask) placed at x, y, w, h. `id` must be unique in
    /// the document; `extra` lands on the visible `<image>`.
    pub fn tag(&self, id: &str, x: f64, y: f64, w: f64, h: f64, extra: &str) -> String {
        let geo = format!(
            r#"x="{x}" y="{y}" width="{w}" height="{h}" preserveAspectRatio="xMidYMid slice""#
        );
        match &self.mask {
            None => format!(r#"<image href="{}" {geo} {extra}/>"#, self.jpeg),
            Some(m) => format!(
                r#"<mask id="{id}" maskUnits="userSpaceOnUse" x="{x}" y="{y}" width="{w}" height="{h}"><image href="{m}" {geo}/></mask><image href="{}" {geo} mask="url(#{id})" {extra}/>"#,
                self.jpeg
            ),
        }
    }
}

/// Download art with a short timeout. None on any failure.
pub fn fetch(url: &str) -> Option<Vec<u8>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("vai-profile-engine")
        .build()
        .ok()?;
    let resp = client.get(url).send().ok()?;
    if !resp.status().is_success() {
        return None;
    }
    resp.bytes().ok().map(|b| b.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opaque_art_needs_no_mask_and_translucent_art_gets_one() {
        let solid =
            DynamicImage::ImageRgba8(RgbaImage::from_pixel(40, 20, Rgba([200, 10, 10, 255])));
        let e = encode(solid, 20, None).unwrap();
        assert!(e.mask.is_none());
        assert_eq!((e.w, e.h), (20, 10));
        assert!(e.jpeg.starts_with("data:image/jpeg;base64,"));

        let mut glass = RgbaImage::from_pixel(10, 10, Rgba([0, 0, 0, 0]));
        glass.put_pixel(5, 5, Rgba([255, 255, 255, 255]));
        let e = encode(DynamicImage::ImageRgba8(glass.clone()), 10, None).unwrap();
        assert!(e.mask.is_some());
        let flat = encode(DynamicImage::ImageRgba8(glass), 10, Some(rgb("#1a2232"))).unwrap();
        assert!(flat.mask.is_none(), "flattened art drops the mask");
        assert!(e
            .tag("m1", 0.0, 0.0, 10.0, 10.0, "")
            .contains(r#"mask="url(#m1)""#));
    }
}
