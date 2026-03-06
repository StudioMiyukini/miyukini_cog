use crate::atlas::{AtlasHandle, MaterialHandle, SpriteRect};
use serde::{Deserialize, Serialize};

/// Render layer for depth ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum RenderLayer {
    Terrain = 0,
    Decals = 1,
    Props = 2,
    Entities = 3,
    Projectiles = 4,
    VfxOpaque = 5,
    VfxAlpha = 6,
    UiWorld = 7,
    UiScreen = 8,
}

/// Sort key for stable ordering within a layer.
///
/// Composed of `(layer, y_sort, sub_order)` so that entities at higher Y
/// (further from camera in iso view) draw first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SortKey {
    pub layer: RenderLayer,
    /// Y position quantized to i32 for stable sort. Higher Y = earlier draw.
    pub y_sort: i32,
    /// Tie-breaker within the same Y (e.g. entity id).
    pub sub_order: u32,
}

/// A single sprite draw command in the batch.
#[derive(Debug, Clone)]
pub struct SpriteInstance {
    /// Screen position (x, y) in pixels (after camera transform).
    pub screen_pos: [f32; 2],
    /// Source rectangle in atlas.
    pub src: SpriteRect,
    /// Atlas handle.
    pub atlas: AtlasHandle,
    /// Material handle.
    pub material: MaterialHandle,
    /// Sort key for ordering.
    pub sort_key: SortKey,
    /// Tint RGBA multiplier.
    pub tint: [f32; 4],
    /// Scale multiplier.
    pub scale: [f32; 2],
}

/// Collects sprite instances for one frame, sorts them, and produces
/// an ordered list ready for GPU submission.
#[derive(Debug, Default)]
pub struct SpriteBatch {
    instances: Vec<SpriteInstance>,
}

impl SpriteBatch {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a sprite instance to the batch.
    pub fn push(&mut self, instance: SpriteInstance) {
        self.instances.push(instance);
    }

    /// Sort all instances by sort key `(layer, y, sub_order)`.
    pub fn sort(&mut self) {
        self.instances.sort_by(|a, b| a.sort_key.cmp(&b.sort_key));
    }

    /// Get sorted instances for iteration / GPU upload.
    pub fn instances(&self) -> &[SpriteInstance] {
        &self.instances
    }

    /// Number of draw commands in this batch.
    pub fn len(&self) -> usize {
        self.instances.len()
    }

    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    /// Clear all instances for the next frame.
    pub fn clear(&mut self) {
        self.instances.clear();
    }

    /// Iterate instances grouped by contiguous material for batched draw calls.
    /// Returns `(material, start_index, count)` runs.
    pub fn material_runs(&self) -> Vec<(MaterialHandle, usize, usize)> {
        if self.instances.is_empty() {
            return Vec::new();
        }
        let mut runs = Vec::new();
        let mut current_mat = self.instances[0].material;
        let mut start = 0;
        for (i, inst) in self.instances.iter().enumerate().skip(1) {
            if inst.material != current_mat {
                runs.push((current_mat, start, i - start));
                current_mat = inst.material;
                start = i;
            }
        }
        runs.push((current_mat, start, self.instances.len() - start));
        runs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atlas::{AtlasHandle, MaterialHandle, SpriteRect};

    fn make_atlas() -> AtlasHandle {
        AtlasHandle::new_test(0)
    }

    fn make_mat(id: u32) -> MaterialHandle {
        MaterialHandle::new_test(id)
    }

    fn make_instance(layer: RenderLayer, y: i32, mat_id: u32) -> SpriteInstance {
        SpriteInstance {
            screen_pos: [100.0, 200.0],
            src: SpriteRect { x: 0, y: 0, w: 64, h: 64 },
            atlas: make_atlas(),
            material: make_mat(mat_id),
            sort_key: SortKey { layer, y_sort: y, sub_order: 0 },
            tint: [1.0, 1.0, 1.0, 1.0],
            scale: [1.0, 1.0],
        }
    }

    #[test]
    fn batch_sorts_by_layer_then_y() {
        let mut batch = SpriteBatch::new();
        batch.push(make_instance(RenderLayer::Entities, 10, 0));
        batch.push(make_instance(RenderLayer::Terrain, 5, 0));
        batch.push(make_instance(RenderLayer::Entities, 5, 0));
        batch.sort();

        let layers: Vec<RenderLayer> = batch.instances().iter().map(|i| i.sort_key.layer).collect();
        assert_eq!(
            layers,
            vec![RenderLayer::Terrain, RenderLayer::Entities, RenderLayer::Entities]
        );

        let ys: Vec<i32> = batch.instances().iter().map(|i| i.sort_key.y_sort).collect();
        assert_eq!(ys[1], 5);
        assert_eq!(ys[2], 10);
    }

    #[test]
    fn material_runs_groups_contiguous() {
        let mut batch = SpriteBatch::new();
        batch.push(make_instance(RenderLayer::Terrain, 0, 0));
        batch.push(make_instance(RenderLayer::Terrain, 1, 0));
        batch.push(make_instance(RenderLayer::Terrain, 2, 1));
        batch.push(make_instance(RenderLayer::Terrain, 3, 1));
        batch.push(make_instance(RenderLayer::Terrain, 4, 0));
        batch.sort();

        let runs = batch.material_runs();
        assert_eq!(runs.len(), 3);
        assert_eq!(runs[0].2, 2); // first 2 with mat 0
        assert_eq!(runs[1].2, 2); // next 2 with mat 1
        assert_eq!(runs[2].2, 1); // last 1 with mat 0
    }

    #[test]
    fn clear_resets_batch() {
        let mut batch = SpriteBatch::new();
        batch.push(make_instance(RenderLayer::Props, 0, 0));
        assert_eq!(batch.len(), 1);
        batch.clear();
        assert!(batch.is_empty());
    }
}
