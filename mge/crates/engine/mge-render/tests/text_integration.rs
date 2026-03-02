// @id: MGE-Render-Text-Integration @do: test @role: back-end @layer: 2 @human: francois

//! Integration tests for the TTF text rendering pipeline.
//!
//! These tests load real font files from `mge/assets/fonts/` and exercise the
//! full path: load font -> rasterise glyphs -> cache -> push to batcher.

use std::path::Path;

use mge_render::text::{FontId, GlyphCache, GlyphKey, TextRenderer, TtfFont};
#[cfg(feature = "instanced")]
use mge_render::instanced::InstancedSpriteBatcher;

/// Helper: path to a font file in mge/assets/fonts/.
fn font_path(name: &str) -> std::path::PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    Path::new(manifest_dir)
        .join("..")
        .join("..")
        .join("..")
        .join("assets")
        .join("fonts")
        .join(name)
}

/// Helper: load font bytes.
fn load_font_bytes(name: &str) -> Vec<u8> {
    let path = font_path(name);
    std::fs::read(&path).unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()))
}

// ---------------------------------------------------------------------------
// Integration: full text rendering pipeline
// ---------------------------------------------------------------------------

#[cfg(feature = "instanced")]
#[test]
fn integration_text_pipeline_french_accents() {
    // Load Rooters.ttf
    let bytes = load_font_bytes("Rooters.ttf");
    let font = TtfFont::from_bytes(&bytes).expect("Rooters.ttf should parse");

    // Create TextRenderer
    let mut renderer = TextRenderer::new(1024, 1024);
    let font_id = renderer.add_font(font);
    assert_eq!(font_id, FontId(0));

    // Create batcher
    let mut batcher = InstancedSpriteBatcher::new(4096);

    // Push text with French accents
    let text = "Bonjour \u{00e9}\u{00e0}\u{00f9}"; // "Bonjour eau"
    let result = renderer.push_text(
        &mut batcher,
        text,
        10.0,
        100.0,
        24.0,
        [1.0, 1.0, 1.0, 1.0],
        font_id,
        0,
    );
    assert!(result.is_ok(), "push_text with accents should succeed: {result:?}");

    // Count non-space characters in the text
    let non_space_count = text.chars().filter(|c| *c != ' ').count();

    assert_eq!(
        batcher.len(),
        non_space_count,
        "batcher should have {non_space_count} instances (one per non-space char), got {}",
        batcher.len()
    );

    // Measure width
    let width = renderer.measure_width(text, 24.0, font_id);
    assert!(width > 0.0, "measured width should be > 0, got {width}");

    // Verify cache has entries
    let cache = renderer.cache();
    assert!(
        !cache.is_empty(),
        "glyph cache should not be empty after rendering"
    );
    assert!(
        cache.is_dirty(),
        "cache should be dirty after new glyphs were inserted"
    );
}

#[test]
fn integration_multi_font_loading() {
    // Load multiple fonts
    let rooters = TtfFont::from_bytes(&load_font_bytes("Rooters.ttf")).unwrap();
    let gentium =
        TtfFont::from_bytes(&load_font_bytes("GentiumBookPlus-Regular.ttf")).unwrap();
    let digital = TtfFont::from_bytes(&load_font_bytes("DigitalDisco.ttf")).unwrap();

    let mut renderer = TextRenderer::new(1024, 1024);
    let id0 = renderer.add_font(rooters);
    let id1 = renderer.add_font(gentium);
    let id2 = renderer.add_font(digital);

    assert_eq!(id0, FontId(0));
    assert_eq!(id1, FontId(1));
    assert_eq!(id2, FontId(2));
    assert_eq!(renderer.default_font(), FontId(0));

    // Measure same text with different fonts -- widths should differ
    let text = "Hello World";
    let w0 = renderer.measure_width(text, 24.0, id0);
    let w1 = renderer.measure_width(text, 24.0, id1);
    let w2 = renderer.measure_width(text, 24.0, id2);

    assert!(w0 > 0.0);
    assert!(w1 > 0.0);
    assert!(w2 > 0.0);

    // At least two of the three should be different (different fonts = different metrics).
    let all_same = (w0 - w1).abs() < 0.01 && (w1 - w2).abs() < 0.01;
    assert!(
        !all_same,
        "different fonts should produce different widths: w0={w0}, w1={w1}, w2={w2}"
    );
}

#[test]
fn integration_glyph_cache_atlas_data() {
    let bytes = load_font_bytes("Rooters.ttf");
    let font = TtfFont::from_bytes(&bytes).unwrap();

    let mut cache = GlyphCache::new(512, 512);

    // Insert several characters
    for ch in "ABCDEFGabcdefg0123456789".chars() {
        let key = GlyphKey {
            font_index: 0,
            character: ch,
            quantized_size: 48,
        };
        cache.get_or_insert(key, &font, 24.0).unwrap();
    }

    // Verify atlas dimensions
    assert_eq!(cache.atlas_width(), 512);
    assert_eq!(cache.atlas_height(), 512);

    // Atlas data should be RGBA (4 bytes per pixel)
    let expected_bytes = 512 * 512 * 4;
    assert_eq!(cache.atlas_data().len(), expected_bytes);

    // Some pixels should be non-zero (glyphs were written)
    let has_nonzero = cache.atlas_data().iter().any(|&b| b > 0);
    assert!(has_nonzero, "atlas should contain non-zero pixels after glyph rasterisation");

    // Dirty flag should be set
    assert!(cache.is_dirty());
}

#[test]
fn integration_rasterize_accented_characters() {
    let bytes = load_font_bytes("GentiumBookPlus-Regular.ttf");
    let font = TtfFont::from_bytes(&bytes).unwrap();

    // French accented characters
    let accented = ['e', '\u{00e9}', '\u{00e8}', '\u{00ea}', '\u{00eb}', '\u{00e0}', '\u{00f9}', '\u{00e7}'];

    for &ch in &accented {
        let (metrics, bitmap) = font.rasterize(ch, 32.0);
        assert!(
            metrics.width > 0,
            "rasterised '{ch}' (U+{:04X}) should have width > 0",
            ch as u32
        );
        assert!(
            metrics.height > 0,
            "rasterised '{ch}' (U+{:04X}) should have height > 0",
            ch as u32
        );
        assert_eq!(
            bitmap.len(),
            (metrics.width * metrics.height) as usize,
            "bitmap length should match dimensions for '{ch}'"
        );
    }
}

#[cfg(feature = "instanced")]
#[test]
fn integration_push_text_batcher_count() {
    let bytes = load_font_bytes("Rooters.ttf");
    let font = TtfFont::from_bytes(&bytes).unwrap();

    let mut renderer = TextRenderer::new(1024, 1024);
    let font_id = renderer.add_font(font);

    let mut batcher = InstancedSpriteBatcher::new(4096);

    // "A B C" has 3 visible chars + 2 spaces = 3 instances
    renderer
        .push_text(
            &mut batcher,
            "A B C",
            0.0,
            0.0,
            20.0,
            [1.0, 0.0, 0.0, 1.0],
            font_id,
            0,
        )
        .unwrap();

    assert_eq!(batcher.len(), 3, "3 non-space chars in 'A B C'");
}
