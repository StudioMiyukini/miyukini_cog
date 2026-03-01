// @id: sd-impl-tests @do: test @role: back-end @layer: 2 @human: miyuk

//! Unit tests for mge-render (no GPU required).

#[cfg(test)]
mod tests {
    use crate::atlas::{AtlasRegistry, TextureAtlas};
    use crate::camera::Camera2D;
    use crate::sprite::{RenderLayer, SpriteBatch, SpriteInstance, SpriteRenderer};
    use crate::tile::TilePos;

    // -----------------------------------------------------------------------
    // 1. Camera: world_to_screen at origin
    // -----------------------------------------------------------------------

    #[test]
    fn camera_world_to_screen_origin() {
        let cam = Camera2D::new(800, 600);
        let (sx, sy) = cam.world_to_screen(0.0, 0.0);
        assert!(
            sx.abs() < f32::EPSILON,
            "origin sx should be 0, got {sx}"
        );
        assert!(
            sy.abs() < f32::EPSILON,
            "origin sy should be 0, got {sy}"
        );
    }

    // -----------------------------------------------------------------------
    // 2. Camera: tile (1, 0) -> (32, 16) at zoom=1
    // -----------------------------------------------------------------------

    #[test]
    fn camera_world_to_screen_tile_1_0() {
        let cam = Camera2D::new(800, 600);
        let (sx, sy) = cam.world_to_screen(1.0, 0.0);
        assert!(
            (sx - 32.0).abs() < f32::EPSILON,
            "tile (1,0) sx should be 32, got {sx}"
        );
        assert!(
            (sy - 16.0).abs() < f32::EPSILON,
            "tile (1,0) sy should be 16, got {sy}"
        );
    }

    // -----------------------------------------------------------------------
    // 3. Camera: screen_to_world roundtrip
    // -----------------------------------------------------------------------

    #[test]
    fn camera_screen_to_world_roundtrip() {
        let cam = Camera2D::new(800, 600);
        let wx = 3.5_f32;
        let wy = 7.2_f32;
        let (sx, sy) = cam.world_to_screen(wx, wy);
        let (rx, ry) = cam.screen_to_world(sx, sy);
        assert!(
            (rx - wx).abs() < 0.001,
            "roundtrip wx: expected {wx}, got {rx}"
        );
        assert!(
            (ry - wy).abs() < 0.001,
            "roundtrip wy: expected {wy}, got {ry}"
        );
    }

    // -----------------------------------------------------------------------
    // 4. Atlas: entry lookup by id
    // -----------------------------------------------------------------------

    #[test]
    fn atlas_entry_lookup() {
        let toml_str = r#"
[[frames]]
name = "grass_01"
x = 0
y = 0
w = 64
h = 32

[[frames]]
name = "stone_floor_01"
x = 64
y = 0
w = 64
h = 32
"#;
        let atlas = TextureAtlas::from_toml("tiles", toml_str, 512, 512).unwrap();
        assert!(atlas.get_uv("grass_01").is_some());
        assert!(atlas.get_uv("stone_floor_01").is_some());
        assert!(atlas.get_uv("missing_tile").is_none());
    }

    // -----------------------------------------------------------------------
    // 5. SpriteBatch: push 10 sprites, then clear
    // -----------------------------------------------------------------------

    #[test]
    fn sprite_batch_push_and_clear() {
        let mut batch = SpriteBatch::new();
        for i in 0..10 {
            batch.push(SpriteInstance {
                x: i as f32,
                ..SpriteInstance::default()
            });
        }
        assert_eq!(batch.len(), 10);
        batch.clear();
        assert_eq!(batch.len(), 0);
    }

    // -----------------------------------------------------------------------
    // 6. RenderLayer: ordering Ground(0) < Unit(2) < HUD(5)
    // -----------------------------------------------------------------------

    #[test]
    fn render_layer_ordering() {
        assert!(
            RenderLayer::Ground < RenderLayer::Unit,
            "Ground should be behind Unit"
        );
        assert!(
            RenderLayer::Unit < RenderLayer::Hud,
            "Unit should be behind HUD"
        );
        assert!(
            RenderLayer::Ground < RenderLayer::Hud,
            "Ground should be behind HUD"
        );
        // All 6 layers in order
        let ordered = [
            RenderLayer::Ground,
            RenderLayer::Shadow,
            RenderLayer::Unit,
            RenderLayer::Effect,
            RenderLayer::Overlay,
            RenderLayer::Hud,
        ];
        for i in 0..ordered.len() - 1 {
            assert!(
                ordered[i] < ordered[i + 1],
                "{:?} should be < {:?}",
                ordered[i],
                ordered[i + 1]
            );
        }
    }

    // -----------------------------------------------------------------------
    // 7. TilePos: to_screen gives correct iso coordinates
    // -----------------------------------------------------------------------

    #[test]
    fn tile_pos_to_screen() {
        let cam = Camera2D::new(800, 600);
        let pos = TilePos::new(1, 1);
        let (sx, sy) = pos.to_screen(&cam);
        // (1-1)*32 = 0, (1+1)*16 = 32
        assert!(
            sx.abs() < f32::EPSILON,
            "tile (1,1) sx should be 0, got {sx}"
        );
        assert!(
            (sy - 32.0).abs() < f32::EPSILON,
            "tile (1,1) sy should be 32, got {sy}"
        );
    }

    // -----------------------------------------------------------------------
    // 8. SpriteBatch: layer isolation (independent batches)
    // -----------------------------------------------------------------------

    #[test]
    fn sprite_batch_layer_isolation() {
        let mut renderer = SpriteRenderer::new();

        renderer.push(SpriteInstance {
            layer: RenderLayer::Ground,
            x: 1.0,
            ..SpriteInstance::default()
        });
        renderer.push(SpriteInstance {
            layer: RenderLayer::Ground,
            x: 2.0,
            ..SpriteInstance::default()
        });
        renderer.push(SpriteInstance {
            layer: RenderLayer::Hud,
            x: 10.0,
            ..SpriteInstance::default()
        });

        let ground = renderer.batch(RenderLayer::Ground);
        assert_eq!(ground.map_or(0, SpriteBatch::len), 2);

        let hud = renderer.batch(RenderLayer::Hud);
        assert_eq!(hud.map_or(0, SpriteBatch::len), 1);

        let unit = renderer.batch(RenderLayer::Unit);
        assert!(unit.is_none() || unit.map_or(0, SpriteBatch::len) == 0);
    }

    // -----------------------------------------------------------------------
    // 9. Camera: zoom affects screen position
    // -----------------------------------------------------------------------

    #[test]
    fn camera_zoom_affects_screen_pos() {
        let mut cam = Camera2D::new(800, 600);
        let (sx1, sy1) = cam.world_to_screen(1.0, 0.0);

        cam.zoom = 2.0;
        let (sx2, sy2) = cam.world_to_screen(1.0, 0.0);

        assert!(
            (sx2 - sx1 * 2.0).abs() < f32::EPSILON,
            "zoom 2 should double sx: {sx1} -> {sx2}"
        );
        assert!(
            (sy2 - sy1 * 2.0).abs() < f32::EPSILON,
            "zoom 2 should double sy: {sy1} -> {sy2}"
        );
    }

    // -----------------------------------------------------------------------
    // 10. Atlas: parse a minimal TOML descriptor
    // -----------------------------------------------------------------------

    #[test]
    fn atlas_toml_parse() {
        let toml_str = r#"
[[frames]]
name = "hero_idle_s_0"
x = 0
y = 0
w = 48
h = 64

[[frames]]
name = "hero_idle_s_1"
x = 48
y = 0
w = 48
h = 64
"#;
        let atlas = TextureAtlas::from_toml("hero", toml_str, 256, 256).unwrap();
        assert_eq!(atlas.id, "hero");
        assert_eq!(atlas.frames.len(), 2);

        let uv = atlas.get_uv("hero_idle_s_0").unwrap();
        // x=0, y=0, w=48, h=64 on a 256x256 atlas
        assert!((uv[0] - 0.0).abs() < f32::EPSILON); // u_min
        assert!((uv[1] - 0.0).abs() < f32::EPSILON); // v_min
        assert!((uv[2] - 48.0 / 256.0).abs() < 0.001); // u_max
        assert!((uv[3] - 64.0 / 256.0).abs() < 0.001); // v_max
    }

    // -----------------------------------------------------------------------
    // 11. Atlas: registry insert and get
    // -----------------------------------------------------------------------

    #[test]
    fn atlas_registry_insert_and_get() {
        let toml_str = r#"
[[frames]]
name = "test"
x = 0
y = 0
w = 32
h = 32
"#;
        let atlas = TextureAtlas::from_toml("reg_test", toml_str, 128, 128).unwrap();
        let mut registry = AtlasRegistry::new();
        assert!(registry.is_empty());
        registry.insert(atlas);
        assert_eq!(registry.len(), 1);
        assert!(registry.get("reg_test").is_some());
        assert!(registry.get("nonexistent").is_none());
    }

    // -----------------------------------------------------------------------
    // 12. SpriteRenderer: sorted_instances respects layer order
    // -----------------------------------------------------------------------

    #[test]
    fn sprite_renderer_sorted_output() {
        let mut renderer = SpriteRenderer::new();
        renderer.push(SpriteInstance {
            layer: RenderLayer::Hud,
            z: 1.0,
            ..SpriteInstance::default()
        });
        renderer.push(SpriteInstance {
            layer: RenderLayer::Ground,
            z: 0.0,
            ..SpriteInstance::default()
        });
        renderer.push(SpriteInstance {
            layer: RenderLayer::Unit,
            z: 0.5,
            ..SpriteInstance::default()
        });

        let sorted = renderer.sorted_instances();
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0].layer, RenderLayer::Ground);
        assert_eq!(sorted[1].layer, RenderLayer::Unit);
        assert_eq!(sorted[2].layer, RenderLayer::Hud);
    }

    // -----------------------------------------------------------------------
    // 13. Camera: free function world_to_screen matches method at zoom=1
    // -----------------------------------------------------------------------

    #[test]
    fn free_fn_world_to_screen() {
        let cam = Camera2D::new(800, 600);
        let (sx1, sy1) = cam.world_to_screen(5.0, 3.0);
        let (sx2, sy2) = crate::camera::world_to_screen(5.0, 3.0);
        assert!((sx1 - sx2).abs() < f32::EPSILON);
        assert!((sy1 - sy2).abs() < f32::EPSILON);
    }

    // -----------------------------------------------------------------------
    // 14. RenderConfig: default values
    // -----------------------------------------------------------------------

    #[test]
    fn render_config_defaults() {
        let cfg = crate::renderer::RenderConfig::default();
        assert_eq!(cfg.width, 800);
        assert_eq!(cfg.height, 600);
        assert!(cfg.vsync);
        assert_eq!(cfg.max_sprites, 16_384);
    }

    // -----------------------------------------------------------------------
    // 15. Atlas: malformed TOML returns error
    // -----------------------------------------------------------------------

    #[test]
    fn atlas_toml_parse_error() {
        let bad_toml = "this is not valid TOML { { {";
        let result = TextureAtlas::from_toml("bad", bad_toml, 256, 256);
        assert!(result.is_err());
    }
}
