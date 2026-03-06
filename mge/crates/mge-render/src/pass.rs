use crate::batch::{RenderLayer, SpriteBatch};

/// Describes which layers a render pass will draw.
#[derive(Debug, Clone)]
pub struct PassDescriptor {
    pub name: &'static str,
    pub layers: &'static [RenderLayer],
    pub alpha: bool,
}

/// Predefined render passes following the frame graph spec.
pub const PASS_TERRAIN: PassDescriptor = PassDescriptor {
    name: "terrain_props",
    layers: &[RenderLayer::Terrain, RenderLayer::Decals, RenderLayer::Props],
    alpha: false,
};

pub const PASS_ENTITIES: PassDescriptor = PassDescriptor {
    name: "entities_projectiles",
    layers: &[RenderLayer::Entities, RenderLayer::Projectiles],
    alpha: false,
};

pub const PASS_VFX_OPAQUE: PassDescriptor =
    PassDescriptor { name: "vfx_opaque", layers: &[RenderLayer::VfxOpaque], alpha: false };

pub const PASS_VFX_ALPHA: PassDescriptor =
    PassDescriptor { name: "vfx_alpha", layers: &[RenderLayer::VfxAlpha], alpha: true };

pub const PASS_UI: PassDescriptor = PassDescriptor {
    name: "ui",
    layers: &[RenderLayer::UiWorld, RenderLayer::UiScreen],
    alpha: true,
};

/// The full ordered list of render passes for one frame.
pub const FRAME_PASSES: &[&PassDescriptor] =
    &[&PASS_TERRAIN, &PASS_ENTITIES, &PASS_VFX_OPAQUE, &PASS_VFX_ALPHA, &PASS_UI];

/// Extract the sub-range of sorted batch instances that belong to the given pass layers.
/// Returns `(start, end)` indices into the sorted batch.
pub fn pass_range(batch: &SpriteBatch, pass: &PassDescriptor) -> (usize, usize) {
    let instances = batch.instances();
    let mut start = instances.len();
    let mut end = 0;

    for (i, inst) in instances.iter().enumerate() {
        if pass.layers.contains(&inst.sort_key.layer) {
            if i < start {
                start = i;
            }
            end = i + 1;
        }
    }

    if start >= end {
        (0, 0)
    } else {
        (start, end)
    }
}

/// Count of instances per pass (for telemetry).
pub fn pass_instance_counts(batch: &SpriteBatch) -> Vec<(&'static str, usize)> {
    FRAME_PASSES
        .iter()
        .map(|p| {
            let (s, e) = pass_range(batch, p);
            (p.name, e - s)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atlas::{AtlasHandle, MaterialHandle, SpriteRect};
    use crate::batch::{SortKey, SpriteInstance};

    fn inst(layer: RenderLayer, y: i32) -> SpriteInstance {
        SpriteInstance {
            screen_pos: [0.0, 0.0],
            src: SpriteRect { x: 0, y: 0, w: 32, h: 32 },
            atlas: AtlasHandle::new_test(0),
            material: MaterialHandle::new_test(0),
            sort_key: SortKey { layer, y_sort: y, sub_order: 0 },
            tint: [1.0; 4],
            scale: [1.0; 2],
        }
    }

    #[test]
    fn pass_range_extracts_correct_layers() {
        let mut batch = SpriteBatch::new();
        batch.push(inst(RenderLayer::Terrain, 0));
        batch.push(inst(RenderLayer::Terrain, 1));
        batch.push(inst(RenderLayer::Entities, 2));
        batch.push(inst(RenderLayer::Entities, 5));
        batch.push(inst(RenderLayer::Projectiles, 3));
        batch.push(inst(RenderLayer::VfxAlpha, 0));
        batch.sort();

        let (s, e) = pass_range(&batch, &PASS_TERRAIN);
        assert_eq!(e - s, 2);

        let (s, e) = pass_range(&batch, &PASS_ENTITIES);
        assert_eq!(e - s, 3); // 2 entities + 1 projectile

        let (s, e) = pass_range(&batch, &PASS_VFX_ALPHA);
        assert_eq!(e - s, 1);
    }

    #[test]
    fn empty_batch_returns_zero_range() {
        let batch = SpriteBatch::new();
        let (s, e) = pass_range(&batch, &PASS_TERRAIN);
        assert_eq!((s, e), (0, 0));
    }

    #[test]
    fn frame_passes_cover_all_layers() {
        let all_layers: Vec<RenderLayer> =
            FRAME_PASSES.iter().flat_map(|p| p.layers.iter().copied()).collect();
        assert!(all_layers.contains(&RenderLayer::Terrain));
        assert!(all_layers.contains(&RenderLayer::Entities));
        assert!(all_layers.contains(&RenderLayer::Projectiles));
        assert!(all_layers.contains(&RenderLayer::VfxOpaque));
        assert!(all_layers.contains(&RenderLayer::VfxAlpha));
        assert!(all_layers.contains(&RenderLayer::UiWorld));
        assert!(all_layers.contains(&RenderLayer::UiScreen));
    }

    #[test]
    fn pass_instance_counts_sums_correctly() {
        let mut batch = SpriteBatch::new();
        batch.push(inst(RenderLayer::Terrain, 0));
        batch.push(inst(RenderLayer::Entities, 1));
        batch.push(inst(RenderLayer::Entities, 2));
        batch.sort();

        let counts = pass_instance_counts(&batch);
        let terrain_count = counts.iter().find(|(n, _)| *n == "terrain_props").map(|(_, c)| *c);
        let entity_count =
            counts.iter().find(|(n, _)| *n == "entities_projectiles").map(|(_, c)| *c);
        assert_eq!(terrain_count, Some(1));
        assert_eq!(entity_count, Some(2));
    }
}
