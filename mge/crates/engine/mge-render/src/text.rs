// @id: MGE-Render-Text @do: implement @role: back-end @layer: 2 @human: francois

//! TTF/OTF text rendering system with glyph caching and multi-font support.
//!
//! Replaces the legacy 5x7 ASCII `BitmapFont` with a full-featured text
//! pipeline powered by `fontdue`:
//!
//! - [`TtfFont`] -- loads and rasterises TTF/OTF fonts.
//! - [`GlyphCache`] -- dynamic shelf-first glyph atlas.
//! - [`TextRenderer`] -- high-level API: multi-font, multi-size, push to batcher.

use std::collections::HashMap;

use crate::errors::RenderError;
#[cfg(feature = "instanced")]
use crate::instanced::InstancedSpriteBatcher;

// ---------------------------------------------------------------------------
// FontId
// ---------------------------------------------------------------------------

/// Handle opaque vers une police enregistree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontId(pub u32);

// ---------------------------------------------------------------------------
// GlyphMetrics
// ---------------------------------------------------------------------------

/// Metriques d'un glyphe rasterise.
#[derive(Debug, Clone, Copy)]
pub struct GlyphMetrics {
    /// Width of the rasterised glyph bitmap in pixels.
    pub width: u32,
    /// Height of the rasterised glyph bitmap in pixels.
    pub height: u32,
    /// Horizontal offset from the glyph origin to the left edge of the bitmap.
    pub xmin: i32,
    /// Vertical offset from the glyph origin to the bottom edge of the bitmap.
    pub ymin: i32,
    /// Horizontal advance width (distance to next glyph origin).
    pub advance_width: f32,
}

// ---------------------------------------------------------------------------
// TtfFont
// ---------------------------------------------------------------------------

/// Encapsulation d'une police TTF/OTF via fontdue.
///
/// Wraps [`fontdue::Font`] and provides rasterisation and metrics queries
/// for individual characters at arbitrary pixel sizes.
pub struct TtfFont {
    /// The underlying fontdue font handle.
    font: fontdue::Font,
}

impl TtfFont {
    /// Charger une police depuis des bytes (TTF/OTF).
    ///
    /// # Errors
    ///
    /// Returns [`RenderError::FontLoadFailed`] if the font data is invalid
    /// or cannot be parsed by fontdue.
    pub fn from_bytes(data: &[u8]) -> Result<Self, RenderError> {
        let font = fontdue::Font::from_bytes(data, fontdue::FontSettings::default()).map_err(
            |e| RenderError::FontLoadFailed {
                details: e.to_string(),
            },
        )?;
        Ok(Self { font })
    }

    /// Charger depuis un fichier.
    ///
    /// # Errors
    ///
    /// Returns [`RenderError::FontLoadFailed`] if the file cannot be read
    /// or contains invalid font data.
    pub fn from_file(path: &std::path::Path) -> Result<Self, RenderError> {
        let data = std::fs::read(path).map_err(|e| RenderError::FontLoadFailed {
            details: e.to_string(),
        })?;
        Self::from_bytes(&data)
    }

    /// Rasteriser un caractere a la taille donnee.
    ///
    /// Retourne les metriques et le bitmap alpha (1 byte par pixel, coverage).
    pub fn rasterize(&self, character: char, px: f32) -> (GlyphMetrics, Vec<u8>) {
        let (metrics, bitmap) = self.font.rasterize(character, px);
        let glyph_metrics = GlyphMetrics {
            width: metrics.width as u32,
            height: metrics.height as u32,
            xmin: metrics.xmin,
            ymin: metrics.ymin,
            advance_width: metrics.advance_width,
        };
        (glyph_metrics, bitmap)
    }

    /// Obtenir les metriques sans rasteriser.
    pub fn metrics(&self, character: char, px: f32) -> GlyphMetrics {
        let metrics = self.font.metrics(character, px);
        GlyphMetrics {
            width: metrics.width as u32,
            height: metrics.height as u32,
            xmin: metrics.xmin,
            ymin: metrics.ymin,
            advance_width: metrics.advance_width,
        }
    }
}

// ---------------------------------------------------------------------------
// GlyphKey / CachedGlyph
// ---------------------------------------------------------------------------

/// Cle de cache pour un glyphe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlyphKey {
    /// Index of the font in the font registry.
    pub font_index: u32,
    /// The character this glyph represents.
    pub character: char,
    /// Taille en px quantifiee a 0.5px (stockee comme u32 : px * 2).
    pub quantized_size: u32,
}

/// Entree de cache pour un glyphe rasterise.
#[derive(Debug, Clone, Copy)]
pub struct CachedGlyph {
    /// UV rectangle dans l'atlas `[u_min, v_min, u_max, v_max]`.
    pub uv_rect: [f32; 4],
    /// Metriques du glyphe.
    pub metrics: GlyphMetrics,
}

// ---------------------------------------------------------------------------
// GlyphCache
// ---------------------------------------------------------------------------

/// Atlas GPU dynamique avec allocateur shelf-first.
///
/// Stores rasterised glyph bitmaps in a single RGBA atlas texture.
/// Uses a shelf-first (row-based) allocation strategy: glyphs are placed
/// left-to-right on the current row; when the row is full, a new row
/// starts below at the height of the tallest glyph in the previous row.
pub struct GlyphCache {
    /// Atlas RGBA en memoire (CPU-side).
    atlas_data: Vec<u8>,
    /// Atlas width in pixels.
    atlas_width: u32,
    /// Atlas height in pixels.
    atlas_height: u32,
    /// Cache des glyphes deja rasterises.
    cache: HashMap<GlyphKey, CachedGlyph>,
    /// Current X write position (shelf-first).
    cursor_x: u32,
    /// Current Y write position (top of current shelf).
    cursor_y: u32,
    /// Height of the tallest glyph on the current shelf row.
    row_height: u32,
    /// Flag dirty pour upload GPU conditionnel.
    dirty: bool,
}

impl GlyphCache {
    /// Create a new empty glyph atlas.
    ///
    /// The atlas is a square RGBA texture of `width x height` pixels,
    /// initialised to transparent black.
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height * 4) as usize;
        Self {
            atlas_data: vec![0u8; size],
            atlas_width: width,
            atlas_height: height,
            cache: HashMap::new(),
            cursor_x: 0,
            cursor_y: 0,
            row_height: 0,
            dirty: false,
        }
    }

    /// Get a cached glyph or rasterise, insert, and return it.
    ///
    /// # Errors
    ///
    /// Returns [`RenderError::GlyphAtlasFull`] if there is no remaining
    /// space in the atlas for the new glyph.
    pub fn get_or_insert(
        &mut self,
        key: GlyphKey,
        font: &TtfFont,
        px: f32,
    ) -> Result<CachedGlyph, RenderError> {
        if let Some(&cached) = self.cache.get(&key) {
            return Ok(cached);
        }

        let (metrics, bitmap) = font.rasterize(key.character, px);

        // Zero-size glyphs (spaces, control chars) get a degenerate UV rect.
        if metrics.width == 0 || metrics.height == 0 {
            let cached = CachedGlyph {
                uv_rect: [0.0, 0.0, 0.0, 0.0],
                metrics,
            };
            self.cache.insert(key, cached);
            self.dirty = true;
            return Ok(cached);
        }

        // Shelf-first allocation: try current row, else advance to next row.
        if self.cursor_x + metrics.width > self.atlas_width {
            // Move to next shelf row.
            self.cursor_y += self.row_height;
            self.cursor_x = 0;
            self.row_height = 0;
        }

        // Check vertical overflow.
        if self.cursor_y + metrics.height > self.atlas_height {
            return Err(RenderError::GlyphAtlasFull {
                atlas_size: self.atlas_width,
            });
        }

        // Blit alpha bitmap into RGBA atlas (white text with alpha channel).
        let gx = self.cursor_x;
        let gy = self.cursor_y;
        for row in 0..metrics.height {
            for col in 0..metrics.width {
                let src_idx = (row * metrics.width + col) as usize;
                let alpha = bitmap[src_idx];
                let dst_x = gx + col;
                let dst_y = gy + row;
                let dst_idx = ((dst_y * self.atlas_width + dst_x) * 4) as usize;
                self.atlas_data[dst_idx] = 255; // R
                self.atlas_data[dst_idx + 1] = 255; // G
                self.atlas_data[dst_idx + 2] = 255; // B
                self.atlas_data[dst_idx + 3] = alpha; // A
            }
        }

        // Compute UV coordinates.
        let u_min = gx as f32 / self.atlas_width as f32;
        let v_min = gy as f32 / self.atlas_height as f32;
        let u_max = (gx + metrics.width) as f32 / self.atlas_width as f32;
        let v_max = (gy + metrics.height) as f32 / self.atlas_height as f32;

        let cached = CachedGlyph {
            uv_rect: [u_min, v_min, u_max, v_max],
            metrics,
        };

        // Advance cursor.
        self.cursor_x += metrics.width;
        if metrics.height > self.row_height {
            self.row_height = metrics.height;
        }

        self.cache.insert(key, cached);
        self.dirty = true;

        Ok(cached)
    }

    /// Returns `true` if the atlas has been modified since the last
    /// [`clear_dirty`](Self::clear_dirty) call.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Clear the dirty flag (call after uploading atlas data to the GPU).
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    /// Return the raw RGBA atlas data for GPU upload.
    pub fn atlas_data(&self) -> &[u8] {
        &self.atlas_data
    }

    /// Return the atlas width in pixels.
    pub fn atlas_width(&self) -> u32 {
        self.atlas_width
    }

    /// Return the atlas height in pixels.
    pub fn atlas_height(&self) -> u32 {
        self.atlas_height
    }

    /// Clear the cache and reset the atlas to transparent black.
    pub fn clear(&mut self) {
        self.cache.clear();
        self.atlas_data.fill(0);
        self.cursor_x = 0;
        self.cursor_y = 0;
        self.row_height = 0;
        self.dirty = false;
    }

    /// Return the number of cached glyphs.
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Returns `true` if the cache contains no glyphs.
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

// ---------------------------------------------------------------------------
// TextRenderer
// ---------------------------------------------------------------------------

/// Remplacement de BitmapFont : multi-fonts, multi-sizes, glyph cache.
///
/// Manages a collection of TTF fonts and a shared glyph cache atlas.
/// Text is pushed as sprite instances into an [`InstancedSpriteBatcher`].
pub struct TextRenderer {
    /// Registered fonts, indexed by [`FontId`].
    fonts: Vec<TtfFont>,
    /// Shared glyph cache atlas.
    cache: GlyphCache,
    /// Default font used when no font ID is specified.
    default_font: FontId,
}

impl TextRenderer {
    /// Create a new text renderer with an empty font registry.
    ///
    /// The glyph atlas is created with the given dimensions (default 1024x1024).
    pub fn new(atlas_width: u32, atlas_height: u32) -> Self {
        Self {
            fonts: Vec::new(),
            cache: GlyphCache::new(atlas_width, atlas_height),
            default_font: FontId(0),
        }
    }

    /// Register a TTF font and return its [`FontId`].
    pub fn add_font(&mut self, font: TtfFont) -> FontId {
        let id = FontId(self.fonts.len() as u32);
        self.fonts.push(font);
        // First font becomes the default.
        if self.fonts.len() == 1 {
            self.default_font = id;
        }
        id
    }

    /// Push text as sprite instances into a batcher.
    ///
    /// Each visible character is pushed as a separate [`InstanceData`] entry
    /// in the batcher. Spaces advance the cursor but produce no instance.
    ///
    /// # Errors
    ///
    /// Returns [`RenderError::FontLoadFailed`] if the font ID is invalid,
    /// [`RenderError::GlyphAtlasFull`] if the atlas runs out of space, or
    /// [`RenderError::BatcherOverflow`] if the batcher is full.
    #[cfg(feature = "instanced")]
    #[allow(clippy::too_many_arguments)]
    pub fn push_text(
        &mut self,
        batcher: &mut InstancedSpriteBatcher,
        text: &str,
        x: f32,
        y: f32,
        px: f32,
        color: [f32; 4],
        font_id: FontId,
        texture_layer: u32,
    ) -> Result<(), RenderError> {
        let font_index = font_id.0 as usize;
        if font_index >= self.fonts.len() {
            return Err(RenderError::FontLoadFailed {
                details: format!("font ID {font_index} not registered"),
            });
        }

        let quantized_size = (px * 2.0) as u32;
        let mut cursor_x = x;

        for ch in text.chars() {
            if ch == ' ' {
                // Space: advance cursor using metrics but don't push instance.
                let metrics = self.fonts[font_index].metrics(ch, px);
                cursor_x += metrics.advance_width;
                continue;
            }

            let key = GlyphKey {
                font_index: font_id.0,
                character: ch,
                quantized_size,
            };

            // We need to borrow fonts and cache separately.
            let font = &self.fonts[font_index];
            let cached = self.cache.get_or_insert(key, font, px)?;

            // Skip zero-size glyphs (control characters).
            if cached.metrics.width > 0 && cached.metrics.height > 0 {
                let gx = cursor_x + cached.metrics.xmin as f32;
                let gy = y - cached.metrics.ymin as f32 - cached.metrics.height as f32;

                let instance = crate::instanced::InstanceData {
                    position: [gx, gy],
                    size: [cached.metrics.width as f32, cached.metrics.height as f32],
                    uv_rect: cached.uv_rect,
                    tint: color,
                    texture_index: texture_layer,
                    z_depth: 0.0,
                    _pad: [0.0, 0.0],
                };
                batcher.push(instance)?;
            }

            cursor_x += cached.metrics.advance_width;
        }

        Ok(())
    }

    /// Measure the width of a text string in pixels.
    ///
    /// Returns 0.0 if the font ID is invalid.
    pub fn measure_width(&self, text: &str, px: f32, font_id: FontId) -> f32 {
        let font_index = font_id.0 as usize;
        if font_index >= self.fonts.len() {
            return 0.0;
        }

        let font = &self.fonts[font_index];
        let mut width = 0.0_f32;
        for ch in text.chars() {
            let metrics = font.metrics(ch, px);
            width += metrics.advance_width;
        }
        width
    }

    /// Line height for a given pixel size (approximately px * 1.2).
    pub fn line_height(px: f32) -> f32 {
        px * 1.2
    }

    /// Access the glyph cache (read-only, for GPU upload inspection).
    pub fn cache(&self) -> &GlyphCache {
        &self.cache
    }

    /// Access the glyph cache (mutable, for clearing or direct manipulation).
    pub fn cache_mut(&mut self) -> &mut GlyphCache {
        &mut self.cache
    }

    /// Return the default font ID.
    pub fn default_font(&self) -> FontId {
        self.default_font
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// Helper: path to Rooters.ttf in assets (relative to workspace root).
    /// CARGO_MANIFEST_DIR = mge/crates/engine/mge-render, so 3 levels up to mge/.
    fn rooters_ttf_path() -> std::path::PathBuf {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        Path::new(manifest_dir)
            .join("..")
            .join("..")
            .join("..")
            .join("assets")
            .join("fonts")
            .join("Rooters.ttf")
    }

    /// Helper: load Rooters.ttf bytes.
    fn rooters_ttf_bytes() -> Vec<u8> {
        std::fs::read(rooters_ttf_path()).expect("Rooters.ttf should exist in mge/assets/fonts/")
    }

    // -- TtfFont tests -------------------------------------------------------

    #[test]
    fn ttf_font_load_bytes() {
        let bytes = rooters_ttf_bytes();
        let font = TtfFont::from_bytes(&bytes);
        assert!(font.is_ok(), "should load Rooters.ttf from bytes");
    }

    #[test]
    fn ttf_font_rasterize_a() {
        let bytes = rooters_ttf_bytes();
        let font = TtfFont::from_bytes(&bytes).unwrap();
        let (metrics, bitmap) = font.rasterize('A', 24.0);
        assert!(metrics.width > 0, "rasterised 'A' width should be > 0");
        assert!(metrics.height > 0, "rasterised 'A' height should be > 0");
        assert_eq!(
            bitmap.len(),
            (metrics.width * metrics.height) as usize,
            "bitmap length should match width*height"
        );
    }

    #[test]
    fn ttf_font_metrics_advance() {
        let bytes = rooters_ttf_bytes();
        let font = TtfFont::from_bytes(&bytes).unwrap();
        let metrics = font.metrics('M', 24.0);
        assert!(
            metrics.advance_width > 0.0,
            "advance_width for 'M' should be > 0"
        );
    }

    #[test]
    fn ttf_font_load_file() {
        let path = rooters_ttf_path();
        let font = TtfFont::from_file(&path);
        assert!(font.is_ok(), "should load Rooters.ttf from file path");
    }

    #[test]
    fn ttf_font_load_file_error() {
        let path = Path::new("/nonexistent/path/to/font.ttf");
        let result = TtfFont::from_file(path);
        assert!(result.is_err(), "nonexistent file should fail");
        match result {
            Err(RenderError::FontLoadFailed { details }) => {
                assert!(!details.is_empty(), "error details should not be empty");
            }
            _ => panic!("expected FontLoadFailed error"),
        }
    }

    // -- GlyphCache tests ----------------------------------------------------

    #[test]
    fn glyph_cache_insert_and_retrieve() {
        let bytes = rooters_ttf_bytes();
        let font = TtfFont::from_bytes(&bytes).unwrap();
        let mut cache = GlyphCache::new(256, 256);

        let key = GlyphKey {
            font_index: 0,
            character: 'A',
            quantized_size: 48, // 24px * 2
        };

        let result = cache.get_or_insert(key, &font, 24.0);
        assert!(result.is_ok(), "inserting 'A' should succeed");

        let cached = result.unwrap();
        assert!(cached.uv_rect[2] > cached.uv_rect[0], "u_max > u_min");
        assert!(cached.uv_rect[3] > cached.uv_rect[1], "v_max > v_min");
    }

    #[test]
    fn glyph_cache_dedup() {
        let bytes = rooters_ttf_bytes();
        let font = TtfFont::from_bytes(&bytes).unwrap();
        let mut cache = GlyphCache::new(256, 256);

        let key = GlyphKey {
            font_index: 0,
            character: 'A',
            quantized_size: 48,
        };

        cache.get_or_insert(key, &font, 24.0).unwrap();
        cache.get_or_insert(key, &font, 24.0).unwrap();

        assert_eq!(cache.len(), 1, "duplicate insert should not add a second entry");
    }

    #[test]
    fn glyph_cache_dirty_flag() {
        let bytes = rooters_ttf_bytes();
        let font = TtfFont::from_bytes(&bytes).unwrap();
        let mut cache = GlyphCache::new(256, 256);

        assert!(!cache.is_dirty(), "new cache should not be dirty");

        let key = GlyphKey {
            font_index: 0,
            character: 'B',
            quantized_size: 48,
        };
        cache.get_or_insert(key, &font, 24.0).unwrap();
        assert!(cache.is_dirty(), "cache should be dirty after insert");

        cache.clear_dirty();
        assert!(!cache.is_dirty(), "cache should not be dirty after clear_dirty");
    }

    #[test]
    fn glyph_cache_clear() {
        let bytes = rooters_ttf_bytes();
        let font = TtfFont::from_bytes(&bytes).unwrap();
        let mut cache = GlyphCache::new(256, 256);

        let key = GlyphKey {
            font_index: 0,
            character: 'C',
            quantized_size: 48,
        };
        cache.get_or_insert(key, &font, 24.0).unwrap();
        assert!(!cache.is_empty());

        cache.clear();
        assert!(cache.is_empty(), "cache should be empty after clear");
        assert_eq!(cache.len(), 0);
    }

    // -- TextRenderer tests --------------------------------------------------

    #[test]
    fn text_renderer_add_font() {
        let bytes = rooters_ttf_bytes();
        let font1 = TtfFont::from_bytes(&bytes).unwrap();
        let font2 = TtfFont::from_bytes(&bytes).unwrap();

        let mut renderer = TextRenderer::new(256, 256);
        let id1 = renderer.add_font(font1);
        let id2 = renderer.add_font(font2);

        assert_eq!(id1, FontId(0));
        assert_eq!(id2, FontId(1));
        assert_eq!(renderer.default_font(), FontId(0));
    }

    #[test]
    fn text_renderer_measure_width() {
        let bytes = rooters_ttf_bytes();
        let font = TtfFont::from_bytes(&bytes).unwrap();
        let mut renderer = TextRenderer::new(256, 256);
        let id = renderer.add_font(font);

        let width = renderer.measure_width("Hello", 24.0, id);
        assert!(width > 0.0, "measured width of 'Hello' should be > 0");
    }

    #[cfg(feature = "instanced")]
    #[test]
    fn text_renderer_push_text() {
        let bytes = rooters_ttf_bytes();
        let font = TtfFont::from_bytes(&bytes).unwrap();
        let mut renderer = TextRenderer::new(512, 512);
        let id = renderer.add_font(font);

        let mut batcher = InstancedSpriteBatcher::new(1024);

        let result = renderer.push_text(
            &mut batcher,
            "OK",
            0.0,
            0.0,
            24.0,
            [1.0, 1.0, 1.0, 1.0],
            id,
            0,
        );
        assert!(result.is_ok(), "push_text should succeed");
        assert_eq!(batcher.len(), 2, "2 non-space chars should produce 2 instances");
    }

    #[test]
    fn text_renderer_line_height() {
        let lh = TextRenderer::line_height(24.0);
        assert!(
            (lh - 28.8).abs() < 0.01,
            "line_height(24) should be ~28.8, got {lh}"
        );
    }
}
