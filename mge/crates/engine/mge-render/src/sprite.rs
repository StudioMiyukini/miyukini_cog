// @id: sd-impl-sprite @do: implement @role: back-end @layer: 2 @human: miyuk

//! Sprite instance data, render layers, and batching.

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// RenderLayer
// ---------------------------------------------------------------------------

/// Visual layers drawn in painter's-algorithm order (low index = behind).
///
/// At least 6 layers as required by the spec. The full Sodomight renderer
/// uses 9 (see `z_order.rs`) but the public API exposes these 6 for
/// external consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RenderLayer {
    /// 0 -- Ground tiles (grass, stone, water).
    Ground = 0,
    /// 1 -- Entity shadows.
    Shadow = 1,
    /// 2 -- Entities (players, monsters, NPCs).
    Unit = 2,
    /// 3 -- Visual effects (AoE, auras).
    Effect = 3,
    /// 4 -- Foreground overlays (tree canopies, roofs).
    Overlay = 4,
    /// 5 -- HUD / world UI (names, bars, damage numbers).
    Hud = 5,
}

impl RenderLayer {
    /// Numeric index used for sorting.
    pub fn index(self) -> u8 {
        self as u8
    }
}

// ---------------------------------------------------------------------------
// SpriteInstance
// ---------------------------------------------------------------------------

/// CPU-side sprite instance ready for batching.
///
/// This is the user-facing sprite struct. The GPU-side representation
/// (`SpriteInstanceGpu`) lives in `renderer.rs` and is derived from this
/// during the flush step.
#[deprecated(note = "Use the animation module (AnimationController + AnimationBank) for animated sprites")]
#[derive(Debug, Clone, Copy)]
pub struct SpriteInstance {
    /// World X position (tile float).
    pub x: f32,
    /// World Y position (tile float).
    pub y: f32,
    /// Depth sub-order within a layer.
    pub z: f32,
    /// UV rectangle `[u_min, v_min, u_max, v_max]` (normalised).
    pub uv: [f32; 4],
    /// RGBA tint colour.
    pub tint: [f32; 4],
    /// Render layer.
    pub layer: RenderLayer,
}

#[allow(deprecated)]
impl Default for SpriteInstance {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            uv: [0.0, 0.0, 1.0, 1.0],
            tint: [1.0, 1.0, 1.0, 1.0],
            layer: RenderLayer::Unit,
        }
    }
}

// ---------------------------------------------------------------------------
// SpriteBatch
// ---------------------------------------------------------------------------

/// Collection of sprite instances for a single render layer.
#[deprecated(note = "Use the animation module (AnimationController + AnimationBank) for animated sprites")]
#[derive(Debug, Clone, Default)]
pub struct SpriteBatch {
    #[allow(deprecated)]
    instances: Vec<SpriteInstance>,
}

#[allow(deprecated)]
impl SpriteBatch {
    /// Create an empty batch.
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a sprite into this batch.
    pub fn push(&mut self, instance: SpriteInstance) {
        self.instances.push(instance);
    }

    /// Remove all sprites from the batch.
    pub fn clear(&mut self) {
        self.instances.clear();
    }

    /// Number of sprites currently in the batch.
    pub fn len(&self) -> usize {
        self.instances.len()
    }

    /// Returns `true` when the batch contains no sprites.
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    /// Immutable access to the underlying instances.
    pub fn instances(&self) -> &[SpriteInstance] {
        &self.instances
    }

    /// Sort sprites by Z (painter's algorithm -- ascending).
    pub fn sort_by_depth(&mut self) {
        self.instances
            .sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap_or(std::cmp::Ordering::Equal));
    }
}

// ---------------------------------------------------------------------------
// SpriteRenderer
// ---------------------------------------------------------------------------

/// Collects sprites into per-layer batches and produces a sorted draw list.
#[allow(deprecated)]
#[derive(Debug, Clone, Default)]
pub struct SpriteRenderer {
    batches: HashMap<u8, SpriteBatch>,
}

#[allow(deprecated)]
impl SpriteRenderer {
    /// Create a new sprite renderer with empty batches.
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a sprite into the batch for its layer.
    pub fn push(&mut self, instance: SpriteInstance) {
        self.batches
            .entry(instance.layer.index())
            .or_default()
            .push(instance);
    }

    /// Clear all batches.
    pub fn clear(&mut self) {
        for batch in self.batches.values_mut() {
            batch.clear();
        }
    }

    /// Return the batch for a given layer (if any).
    pub fn batch(&self, layer: RenderLayer) -> Option<&SpriteBatch> {
        self.batches.get(&layer.index())
    }

    /// Produce a flat, depth-sorted list of all sprites across all layers.
    ///
    /// Layers are iterated in ascending order (Ground first, HUD last).
    /// Within each layer the sprites are sorted by their `z` field.
    pub fn sorted_instances(&self) -> Vec<SpriteInstance> {
        let mut keys: Vec<u8> = self.batches.keys().copied().collect();
        keys.sort_unstable();

        let mut out = Vec::new();
        for key in keys {
            if let Some(batch) = self.batches.get(&key) {
                let mut layer_sprites = batch.instances.clone();
                layer_sprites
                    .sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap_or(std::cmp::Ordering::Equal));
                out.extend(layer_sprites);
            }
        }
        out
    }

    /// Total number of sprites across all layers.
    pub fn total_sprites(&self) -> usize {
        self.batches.values().map(SpriteBatch::len).sum()
    }
}
