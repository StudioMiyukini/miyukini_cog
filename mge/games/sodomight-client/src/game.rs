// @id: Sodomight-Client-Game @do: game-app @role: back-end @layer: 4 @human: miyuk
//! Sodomight game application — implements the mge-platform GameApp trait.
//!
//! Wires together the game world (SodomightWorld), GUI overlay, content data,
//! and the wgpu sprite rendering pipeline into a playable game loop.

use mge_core::game_loop::{GameLoop, LoopConfig};
use mge_platform::{GameApp, GpuContext, InputEvent, KeyCode};
use mge_render::camera::{TILE_HEIGHT, TILE_WIDTH};
use mge_render::{Camera2D, SpriteBatcher, SpritePipeline};
use sodomight_game::content;
use sodomight_game::world::MonsterRecord;
use sodomight_game::SodomightWorld;

use crate::gui::{self, GameGui, GuiAction};

/// Bitflag constants for movement directions.
const DIR_UP: u8 = 0b0001;
const DIR_DOWN: u8 = 0b0010;
const DIR_LEFT: u8 = 0b0100;
const DIR_RIGHT: u8 = 0b1000;

/// Map size in tiles.
const MAP_W: i32 = 32;
const MAP_H: i32 = 32;

/// Player movement speed in world units per tick.
const MOVE_SPEED: f32 = 0.1;

/// Click radius for picking up loot (world units).
const LOOT_PICKUP_RANGE: f32 = 2.0;

/// Click radius for attacking monsters (world units).
const ATTACK_RANGE: f32 = 3.0;

/// Monster rendering tint (red-ish).
const MONSTER_TINT: [f32; 4] = [0.9, 0.2, 0.2, 1.0];

/// Loot pile rendering tint (gold).
const LOOT_TINT: [f32; 4] = [1.0, 0.85, 0.2, 1.0];

/// Player rendering tint (blue).
const PLAYER_TINT: [f32; 4] = [0.2, 0.5, 1.0, 1.0];

/// GPU resources initialised at startup. Separated into its own struct
/// to keep `SodomightApp` free of excessive `Option` fields.
struct GpuResources {
    pipeline: SpritePipeline,
    batcher: SpriteBatcher,
    grass_texture: mge_render::GpuTexture,
    gui_texture: mge_render::GpuTexture,
}

/// Main Sodomight client application.
pub struct SodomightApp {
    game_loop: GameLoop,
    camera: Camera2D,
    /// Movement bitflags (see `DIR_*` constants).
    move_dirs: u8,
    frame_count: u64,
    /// GPU resources (initialised in `on_init`).
    gpu_res: Option<GpuResources>,
    /// Game world with all gameplay systems.
    world: Option<SodomightWorld>,
    /// GUI overlay.
    gui: GameGui,
}

impl SodomightApp {
    /// Create a new Sodomight application.
    #[must_use]
    pub fn new() -> Self {
        Self {
            game_loop: GameLoop::new(LoopConfig::default()),
            camera: Camera2D::new(1280, 720),
            move_dirs: 0,
            frame_count: 0,
            gpu_res: None,
            world: None,
            gui: GameGui::new(1280.0, 720.0),
        }
    }

    /// Check if a direction is active.
    fn moving(&self, dir: u8) -> bool {
        self.move_dirs & dir != 0
    }

    /// Set a direction active.
    fn press_dir(&mut self, dir: u8) {
        self.move_dirs |= dir;
    }

    /// Clear a direction.
    fn release_dir(&mut self, dir: u8) {
        self.move_dirs &= !dir;
    }

    /// Initialize the game world with Act 1 content.
    fn init_world(&mut self) {
        let mut world = match SodomightWorld::new() {
            Ok(w) => w,
            Err(e) => {
                tracing::error!("Failed to create game world: {e}");
                return;
            }
        };

        // Register Act 1 skills.
        for skill_def in content::act1_skills() {
            world.register_skill(skill_def);
        }

        // Register Act 1 treasure classes.
        for tc in content::act1_treasure_classes() {
            world.tc_registry.register(tc);
        }

        // Register Act 1 quests.
        world.quest_registry = content::act1_quests();

        // Learn default skill (normal attack) at level 1.
        // Give player 1 skill point, then invest it.
        world.player_skills.add_points(1);
        // Use the first skill's ID from Act 1 content (normal_attack).
        let skill_defs = content::act1_skills();
        let normal_attack = skill_defs[0].id.clone();
        // Clone registry ref to avoid borrow conflict.
        let registry_clone = world.skill_registry.clone();
        let _ = world.player_skills.invest(&normal_attack, &registry_clone);

        // Spawn Act 1 monsters based on Blood Moor zone definition.
        let monsters = content::act1_monsters();
        let zone = content::find_zone("blood_moor");

        if let Some(zone) = zone {
            let mut spawn_x = 8.0_f32;
            let mut spawn_y = 8.0_f32;

            for monster_id_str in &zone.monster_ids {
                if let Some(mdef) = monsters.iter().find(|m| &m.id == monster_id_str) {
                    // Spawn a few of each type.
                    for i in 0..3 {
                        let x = spawn_x + i as f32 * 2.5;
                        let y = spawn_y + i as f32 * 1.5;

                        if let Err(e) = world.spawn_monster(
                            &mdef.name,
                            x,
                            y,
                            mdef.level,
                            mdef.health as u32,
                        ) {
                            tracing::warn!("Failed to spawn {}: {e}", mdef.name);
                        }
                    }
                    spawn_x += 6.0;
                    spawn_y += 3.0;
                }
            }
        }

        tracing::info!(
            "World initialised: {} entities",
            world.ecs.entity_count()
        );

        self.world = Some(world);
    }

    /// Sync GUI state from the authoritative game world.
    fn sync_gui_from_world(&mut self) {
        let Some(ref world) = self.world else {
            return;
        };

        let (hp, hp_max) = world.player_health();
        let (mana, mana_max) = world.player_mana();
        #[allow(clippy::cast_possible_wrap)]
        let xp = world.player_stats.level.experience as i64;
        let current_level = world.player_stats.level.level;
        let level = current_level.min(255) as u8;
        let exp_table = mge_arpg_stats::ExpTable::d2_standard();
        #[allow(clippy::cast_possible_wrap)]
        let xp_next = exp_table
            .xp_for_level(current_level + 1)
            .unwrap_or(u64::MAX) as i64;

        self.gui.update_from_world(
            hp, hp_max, mana, mana_max, xp, xp_next, level, 0,
        );

        // Sync combat log messages.
        let gui_log_len = self.gui.combat_log().len();
        let world_log = &world.combat_log;
        if world_log.len() > gui_log_len {
            for msg in &world_log[gui_log_len..] {
                self.gui.push_combat_message(msg.clone());
            }
        }
    }

    /// Handle a GUI action resulting from user input.
    fn handle_gui_action(&mut self, action: &GuiAction) {
        match *action {
            GuiAction::None => {}
            GuiAction::ToggleInventory => self.gui.toggle_inventory(),
            GuiAction::ToggleSkills => self.gui.toggle_skills(),
            GuiAction::UseSkill(slot) => {
                self.use_skill_slot(slot);
            }
            GuiAction::ClickInventorySlot(_slot) => {
                // Future: item interaction.
            }
            GuiAction::ClickWorld(sx, sy) => {
                self.handle_world_click(sx, sy);
            }
        }
    }

    /// Handle a click on the game world (attack nearest monster or pick up loot).
    fn handle_world_click(&mut self, screen_x: f32, screen_y: f32) {
        let Some(ref mut world) = self.world else {
            return;
        };

        // Convert screen coords to world coords using camera.
        let (sw, sh) = (self.camera.screen_w as f32, self.camera.screen_h as f32);
        let cam_left = self.camera.world_x - sw / (2.0 * self.camera.zoom);
        let cam_top = self.camera.world_y - sh / (2.0 * self.camera.zoom);
        let world_sx = screen_x / self.camera.zoom + cam_left;
        let world_sy = screen_y / self.camera.zoom + cam_top;

        // Convert from iso screen to tile coordinates.
        let tile_x = (world_sx / (TILE_WIDTH / 2.0) + world_sy / (TILE_HEIGHT / 2.0)) / 2.0;
        let tile_y = (world_sy / (TILE_HEIGHT / 2.0) - world_sx / (TILE_WIDTH / 2.0)) / 2.0;

        // Try to pick up loot first.
        let (px, py) = world.player_position();
        if !world.pending_loot.is_empty() {
            let mut closest_loot: Option<(usize, f32)> = None;
            for (i, (lx, ly, _drops)) in world.pending_loot.iter().enumerate() {
                let dx = lx - px;
                let dy = ly - py;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < LOOT_PICKUP_RANGE {
                    if let Some((_, best_dist)) = closest_loot {
                        if dist < best_dist {
                            closest_loot = Some((i, dist));
                        }
                    } else {
                        closest_loot = Some((i, dist));
                    }
                }
            }

            if let Some((loot_idx, _)) = closest_loot {
                // Pick up first item in the pile.
                match world.player_pickup_loot(loot_idx, 0) {
                    Ok(msg) => tracing::info!("{msg}"),
                    Err(e) => tracing::warn!("Loot pickup failed: {e}"),
                }
                return;
            }
        }

        // Try to attack nearest monster to the click location.
        let monsters_near = world.monsters_near(tile_x, tile_y, ATTACK_RANGE);
        if let Some(&(monster_id, _, _, _)) = monsters_near.first() {
            match world.player_attack(monster_id) {
                Ok(msgs) => {
                    for msg in &msgs {
                        tracing::debug!("{msg}");
                    }
                }
                Err(e) => tracing::debug!("Attack failed: {e}"),
            }
        }
    }

    /// Use a skill from the skill bar slot.
    fn use_skill_slot(&mut self, slot: usize) {
        let skills = content::act1_skills();
        let Some(skill_def) = skills.get(slot) else {
            return;
        };

        let Some(ref mut world) = self.world else {
            return;
        };

        // Find nearest monster to use skill on.
        let (px, py) = world.player_position();
        let nearby = world.monsters_near(px, py, 10.0);
        let target = nearby.first().map(|&(id, _, _, _)| id);

        match world.player_use_skill(&skill_def.id, target) {
            Ok(msgs) => {
                for msg in &msgs {
                    tracing::debug!("{msg}");
                }
            }
            Err(e) => tracing::debug!("Skill use failed: {e}"),
        }
    }
}

impl Default for SodomightApp {
    fn default() -> Self {
        Self::new()
    }
}

/// Try to load a grass tile image from the Dev_assets directory.
fn load_grass_image() -> Option<image::RgbaImage> {
    let paths = [
        "assets/Dev_assets/Grass_a.png",
        "mge/assets/Dev_assets/Grass_a.png",
        "../mge/assets/Dev_assets/Grass_a.png",
    ];

    for path in &paths {
        if let Ok(img) = image::open(path) {
            tracing::info!("Loaded grass texture from {path}");
            return Some(img.to_rgba8());
        }
    }

    tracing::warn!("Grass texture not found, using fallback checkerboard");
    None
}

/// Generate a simple checkerboard RGBA image as a fallback texture.
fn checkerboard_fallback() -> image::RgbaImage {
    let (w, h) = (64_u32, 32_u32);
    let mut img = image::RgbaImage::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let checker = ((x / 8) + (y / 8)) % 2 == 0;
            let c = if checker { 100 } else { 60 };
            img.put_pixel(x, y, image::Rgba([c, 140, c, 255]));
        }
    }
    img
}

/// Batch visible isometric tiles into the sprite batcher.
fn batch_tiles(
    batcher: &mut SpriteBatcher,
    cam_left: f32,
    cam_top: f32,
    screen_w: f32,
    screen_h: f32,
) {
    let tile_w = TILE_WIDTH;
    let tile_h = TILE_HEIGHT;
    let uv = [0.0_f32, 0.0, 1.0, 1.0];
    let tint = [1.0_f32, 1.0, 1.0, 1.0];

    for ty in 0..MAP_H {
        for tx in 0..MAP_W {
            let sx = (tx - ty) as f32 * (tile_w / 2.0);
            let sy = (tx + ty) as f32 * (tile_h / 2.0);

            // Simple frustum cull
            let screen_x = sx - cam_left;
            let screen_y = sy - cam_top;
            if screen_x + tile_w < 0.0
                || screen_x > screen_w
                || screen_y + tile_h < 0.0
                || screen_y > screen_h
            {
                continue;
            }

            batcher.push(sx, sy, tile_w, tile_h, uv, tint);
        }
    }
}

/// Batch monster sprites as coloured quads on the isometric map.
fn batch_monsters(
    batcher: &mut SpriteBatcher,
    world: &SodomightWorld,
    cam_left: f32,
    cam_top: f32,
    screen_w: f32,
    screen_h: f32,
) {
    let uv = [0.0_f32, 0.0, 1.0, 1.0];
    let monster_size = 24.0_f32;

    // Iterate all living monsters through the public ECS.
    for &entity_id in world.ai_agents_keys() {
        if !world.ecs.is_alive(entity_id) {
            continue;
        }

        let Ok(mr) = world.ecs.get_component::<MonsterRecord>(entity_id) else {
            continue;
        };

        if !mr.health.is_alive() {
            continue;
        }

        let mx = mr.position.x();
        let my = mr.position.y();

        // Convert world tile coords to iso screen coords.
        let sx = (mx - my) * (TILE_WIDTH / 2.0);
        let sy = (mx + my) * (TILE_HEIGHT / 2.0);

        // Frustum cull.
        let screen_x = sx - cam_left;
        let screen_y = sy - cam_top;
        if screen_x + monster_size < 0.0
            || screen_x > screen_w
            || screen_y + monster_size < 0.0
            || screen_y > screen_h
        {
            continue;
        }

        // Centre the monster quad on the tile.
        let offset = (TILE_WIDTH - monster_size) / 2.0;
        batcher.push(sx + offset, sy - monster_size, monster_size, monster_size, uv, MONSTER_TINT);
    }
}

/// Batch loot piles as small gold quads.
fn batch_loot(
    batcher: &mut SpriteBatcher,
    world: &SodomightWorld,
    cam_left: f32,
    cam_top: f32,
    screen_w: f32,
    screen_h: f32,
) {
    let uv = [0.0_f32, 0.0, 1.0, 1.0];
    let loot_size = 12.0_f32;

    for (lx, ly, _drops) in &world.pending_loot {
        let sx = (lx - ly) * (TILE_WIDTH / 2.0);
        let sy = (lx + ly) * (TILE_HEIGHT / 2.0);

        let screen_x = sx - cam_left;
        let screen_y = sy - cam_top;
        if screen_x + loot_size < 0.0
            || screen_x > screen_w
            || screen_y + loot_size < 0.0
            || screen_y > screen_h
        {
            continue;
        }

        let offset = (TILE_WIDTH - loot_size) / 2.0;
        batcher.push(sx + offset, sy, loot_size, loot_size, uv, LOOT_TINT);
    }
}

/// Batch the player character as a coloured quad.
fn batch_player(
    batcher: &mut SpriteBatcher,
    px: f32,
    py: f32,
    cam_left: f32,
    cam_top: f32,
    screen_w: f32,
    screen_h: f32,
) {
    let uv = [0.0_f32, 0.0, 1.0, 1.0];
    let player_size = 28.0_f32;

    let sx = (px - py) * (TILE_WIDTH / 2.0);
    let sy = (px + py) * (TILE_HEIGHT / 2.0);

    let screen_x = sx - cam_left;
    let screen_y = sy - cam_top;
    if screen_x + player_size < 0.0
        || screen_x > screen_w
        || screen_y + player_size < 0.0
        || screen_y > screen_h
    {
        return;
    }

    let offset = (TILE_WIDTH - player_size) / 2.0;
    batcher.push(sx + offset, sy - player_size, player_size, player_size, uv, PLAYER_TINT);
}

impl GameApp for SodomightApp {
    fn on_init(&mut self, gpu: &GpuContext) {
        let (w, h) = gpu.surface_size();
        self.camera = Camera2D::new(w, h);
        self.gui.set_screen_size(w as f32, h as f32);

        // Init game world.
        self.init_world();

        // Set camera to player position.
        if let Some(ref world) = self.world {
            let (px, py) = world.player_position();
            let sx = (px - py) * (TILE_WIDTH / 2.0);
            let sy = (px + py) * (TILE_HEIGHT / 2.0);
            self.camera.follow(sx, sy);
        }

        let pipeline = SpritePipeline::new(&gpu.device, gpu.surface_format());
        let batcher = SpriteBatcher::new(&gpu.device, 8192);

        let grass_img = load_grass_image().unwrap_or_else(checkerboard_fallback);
        let grass_texture = mge_render::GpuTexture::from_image(
            &gpu.device,
            &gpu.queue,
            &pipeline,
            &grass_img,
            "grass_tile",
        );

        let gui_texture = gui::create_white_texture(&gpu.device, &gpu.queue, &pipeline);

        self.gpu_res = Some(GpuResources {
            pipeline,
            batcher,
            grass_texture,
            gui_texture,
        });

        tracing::info!("Sodomight initialised: {w}x{h}, sprite pipeline + world ready");
    }

    #[allow(clippy::too_many_lines)]
    fn on_frame(&mut self, gpu: &GpuContext) {
        // --- Fixed timestep ticks ---
        let ticks = self.game_loop.begin_frame();

        // Get player position for movement.
        let (mut px, mut py) = self
            .world
            .as_ref()
            .map_or((5.0, 5.0), SodomightWorld::player_position);

        for _ in 0..ticks {
            if self.moving(DIR_UP) {
                py -= MOVE_SPEED;
            }
            if self.moving(DIR_DOWN) {
                py += MOVE_SPEED;
            }
            if self.moving(DIR_LEFT) {
                px -= MOVE_SPEED;
            }
            if self.moving(DIR_RIGHT) {
                px += MOVE_SPEED;
            }

            // Update world player position.
            if let Some(ref mut world) = self.world {
                world.set_player_position(px, py);
                world.tick();
            }
        }

        // Update camera to follow player's iso position.
        let sx = (px - py) * (TILE_WIDTH / 2.0);
        let sy = (px + py) * (TILE_HEIGHT / 2.0);
        self.camera.follow(sx, sy);

        let alpha = self.game_loop.alpha();
        self.camera.update(alpha);

        // Sync GUI from world state.
        self.sync_gui_from_world();

        let frame = match gpu.begin_frame() {
            Ok(f) => f,
            Err(e) => {
                tracing::warn!("Frame error: {e}");
                return;
            }
        };

        let Some(ref mut res) = self.gpu_res else {
            frame.clear_and_present(gpu, 0.02, 0.02, 0.08);
            return;
        };

        // Camera top-left in screen-pixel space.
        let (sw, sh) = gpu.surface_size();
        let screen_w = sw as f32;
        let screen_h = sh as f32;
        let cam_left = self.camera.world_x - screen_w / (2.0 * self.camera.zoom);
        let cam_top = self.camera.world_y - screen_h / (2.0 * self.camera.zoom);

        res.pipeline.update_camera(
            &gpu.queue,
            screen_w,
            screen_h,
            cam_left,
            cam_top,
            self.camera.zoom,
        );

        // --- Batch world geometry (tiles + entities) ---
        res.batcher.begin();
        batch_tiles(&mut res.batcher, cam_left, cam_top, screen_w, screen_h);

        // Batch game entities.
        if let Some(ref world) = self.world {
            batch_loot(&mut res.batcher, world, cam_left, cam_top, screen_w, screen_h);
            batch_monsters(&mut res.batcher, world, cam_left, cam_top, screen_w, screen_h);
        }

        batch_player(&mut res.batcher, px, py, cam_left, cam_top, screen_w, screen_h);

        let world_vert_count = res.batcher.flush(&gpu.queue);

        // --- Render ---
        let (mut encoder, view, output) = frame.into_parts();
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("sprite_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.02,
                            g: 0.02,
                            b: 0.08,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });

            // Draw world (tiles + entities) with grass texture.
            pass.set_pipeline(&res.pipeline.render_pipeline);
            pass.set_bind_group(0, &res.pipeline.camera_bind_group, &[]);
            pass.set_bind_group(1, &res.grass_texture.bind_group, &[]);
            res.batcher.draw(&mut pass, world_vert_count);
        }

        // --- GUI pass (uses identity camera so GUI is in screen space) ---
        res.pipeline.update_camera(&gpu.queue, screen_w, screen_h, 0.0, 0.0, 1.0);

        res.batcher.begin();
        self.gui.draw(&mut res.batcher);
        let gui_vert_count = res.batcher.flush(&gpu.queue);

        if gui_vert_count > 0 {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("gui_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });

            pass.set_pipeline(&res.pipeline.render_pipeline);
            pass.set_bind_group(0, &res.pipeline.camera_bind_group, &[]);
            pass.set_bind_group(1, &res.gui_texture.bind_group, &[]);
            res.batcher.draw(&mut pass, gui_vert_count);
        }

        gpu.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.frame_count += 1;
        if self.frame_count.is_multiple_of(300) {
            let sprite_count = world_vert_count / 4;
            let player_pos = self
                .world
                .as_ref()
                .map_or((0.0, 0.0), SodomightWorld::player_position);

            tracing::debug!(
                "Frame {} | Player ({:.1}, {:.1}) | Camera ({:.1}, {:.1}) | Sprites: {} | GUI quads: {}",
                self.frame_count,
                player_pos.0,
                player_pos.1,
                self.camera.world_x,
                self.camera.world_y,
                sprite_count,
                gui_vert_count / 4,
            );
        }
    }

    fn on_input(&mut self, event: InputEvent) {
        // Let GUI handle input first.
        let gui_action = self.gui.handle_input(&event);
        if gui_action != GuiAction::None {
            self.handle_gui_action(&gui_action);
            return;
        }

        // Movement input.
        match event {
            InputEvent::KeyDown { key } => match key {
                KeyCode::W | KeyCode::Up => self.press_dir(DIR_UP),
                KeyCode::S | KeyCode::Down => self.press_dir(DIR_DOWN),
                KeyCode::A | KeyCode::Left => self.press_dir(DIR_LEFT),
                KeyCode::D | KeyCode::Right => self.press_dir(DIR_RIGHT),
                _ => {}
            },
            InputEvent::KeyUp { key } => match key {
                KeyCode::W | KeyCode::Up => self.release_dir(DIR_UP),
                KeyCode::S | KeyCode::Down => self.release_dir(DIR_DOWN),
                KeyCode::A | KeyCode::Left => self.release_dir(DIR_LEFT),
                KeyCode::D | KeyCode::Right => self.release_dir(DIR_RIGHT),
                _ => {}
            },
            InputEvent::WindowResize { width, height } => {
                self.gui.set_screen_size(width as f32, height as f32);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sodomight_app_default() {
        let app = SodomightApp::new();
        assert_eq!(app.frame_count, 0);
        assert!(app.world.is_none());
        assert!(!app.gui.is_inventory_open());
    }

    #[test]
    fn test_input_sets_movement_flags() {
        let mut app = SodomightApp::new();
        assert!(!app.moving(DIR_UP));

        app.on_input(InputEvent::KeyDown { key: KeyCode::W });
        assert!(app.moving(DIR_UP));

        app.on_input(InputEvent::KeyUp { key: KeyCode::W });
        assert!(!app.moving(DIR_UP));
    }

    #[test]
    fn test_arrow_keys_movement() {
        let mut app = SodomightApp::new();

        app.on_input(InputEvent::KeyDown { key: KeyCode::Up });
        assert!(app.moving(DIR_UP));

        app.on_input(InputEvent::KeyDown { key: KeyCode::Left });
        assert!(app.moving(DIR_LEFT));

        app.on_input(InputEvent::KeyUp { key: KeyCode::Up });
        assert!(!app.moving(DIR_UP));
        assert!(app.moving(DIR_LEFT));
    }

    #[test]
    fn test_default_trait() {
        let app = SodomightApp::default();
        assert_eq!(app.frame_count, 0);
    }

    #[test]
    fn test_checkerboard_fallback() {
        let img = checkerboard_fallback();
        assert_eq!(img.width(), 64);
        assert_eq!(img.height(), 32);
    }

    #[test]
    fn test_gui_toggle_via_input() {
        let mut app = SodomightApp::new();
        assert!(!app.gui.is_inventory_open());

        app.on_input(InputEvent::KeyDown { key: KeyCode::I });
        assert!(app.gui.is_inventory_open());

        app.on_input(InputEvent::KeyDown { key: KeyCode::I });
        assert!(!app.gui.is_inventory_open());
    }

    #[test]
    fn test_init_world_creates_entities() {
        let mut app = SodomightApp::new();
        app.init_world();
        assert!(app.world.is_some());

        let world = app.world.as_ref().unwrap();
        // Player + 6 monsters (3 fallen + 3 quill rats).
        assert!(world.ecs.entity_count() >= 7);
    }

    #[test]
    fn test_skill_bar_keys() {
        let mut app = SodomightApp::new();
        app.on_input(InputEvent::KeyDown { key: KeyCode::Num1 });
        // Skill use without world is a no-op; verify no panic.
    }
}
