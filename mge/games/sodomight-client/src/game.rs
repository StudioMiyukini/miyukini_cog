// @id: Sodomight-Client-Game @do: game-app @role: back-end @layer: 4 @human: miyuk
//! Sodomight game application -- implements the mge-platform GameApp trait.
//!
//! Sprint 3 migration: uses instanced pipeline (TextureArray, InstancedSpriteBatcher,
//! FrustumCuller), AnimationController, TextRenderer (TTF/OTF), and Overhead UI.

use mge_core::game_loop::{GameLoop, LoopConfig};
use mge_platform::{GameApp, GpuContext, InputEvent, KeyCode};
use mge_render::camera::{TILE_HEIGHT, TILE_WIDTH};
use mge_ecs::EntityId;
use mge_render::{
    AnimationBank, AnimationController, AnimationState, Camera2D, Direction,
    EmoteKind, EmoteManager, FloatingTextKind, FloatingTextManager,
    FontId, FrustumCuller, InstanceData, InstancedSpriteBatcher,
    InstancedSpritePipeline, ProgressBarManager, RenderEntity, SpriteBatcher,
    SpritePipeline, TextRenderer, TextureArray, TtfFont,
};
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

/// Tile rendering tint (plain white).
const TILE_TINT: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

/// Maximum number of instanced sprites per frame.
const MAX_INSTANCES: usize = 16_384;

/// Maximum number of texture array layers.
const MAX_TEXTURE_LAYERS: u32 = 16;

/// Texture array layer index for the grass tile.
const LAYER_GRASS: u32 = 0;
/// Texture array layer index for the white GUI texture.
const LAYER_WHITE: u32 = 1;
/// Texture array layer index for the glyph atlas (reserved for text rendering).
#[allow(dead_code)]
const LAYER_GLYPH: u32 = 2;

/// Standard texture dimensions for the texture array (all layers must match).
const TEX_ARRAY_W: u32 = 256;
const TEX_ARRAY_H: u32 = 256;

// ---------------------------------------------------------------------------
// GPU Resources
// ---------------------------------------------------------------------------

/// Instanced GPU resources initialised at startup.
struct InstancedGpuResources {
    /// Instanced sprite pipeline (storage buffer + texture array).
    instanced_pipeline: InstancedSpritePipeline,
    /// CPU-side staging buffer for sprite instances.
    instanced_batcher: InstancedSpriteBatcher,
    /// GPU texture array (grass, white, glyph atlas, etc.).
    /// Kept alive for RAII; the GPU references it via bind groups.
    #[allow(dead_code)]
    texture_array: TextureArray,
    /// GPU storage buffer for instance data.
    storage_buffer: wgpu::Buffer,
    /// Bind group for the camera uniform.
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    /// Bind group for the texture array + sampler.
    texture_bind_group: wgpu::BindGroup,
    /// Bind group for the storage buffer.
    storage_bind_group: wgpu::BindGroup,
    /// Sampler for the texture array (kept alive for RAII).
    #[allow(dead_code)]
    sampler: wgpu::Sampler,
    /// Legacy pipeline (kept for GUI rendering which uses per-texture bind groups).
    legacy_pipeline: SpritePipeline,
    legacy_batcher: SpriteBatcher,
    gui_texture: mge_render::GpuTexture,
}

// ---------------------------------------------------------------------------
// SodomightApp
// ---------------------------------------------------------------------------

/// Main Sodomight client application.
pub struct SodomightApp {
    game_loop: GameLoop,
    camera: Camera2D,
    /// Movement bitflags (see `DIR_*` constants).
    move_dirs: u8,
    frame_count: u64,
    /// GPU resources (initialised in `on_init`).
    gpu_res: Option<InstancedGpuResources>,
    /// Game world with all gameplay systems.
    world: Option<SodomightWorld>,
    /// GUI overlay.
    gui: GameGui,
    /// Player animation controller (S3-T02).
    player_anim: AnimationController,
    /// Minimal animation bank for the player (single-frame placeholder).
    player_anim_bank: AnimationBank,
    /// Frustum culler for entity visibility (S3-T04).
    culler: FrustumCuller,
    /// TTF text renderer (S3-T06).
    text_renderer: Option<TextRenderer>,
    /// Font IDs for loaded fonts.
    font_hud: FontId,
    #[allow(dead_code)]
    font_narrative: FontId,
    /// Floating text manager for combat feedback (S3-T06b).
    floating_texts: FloatingTextManager,
    /// Emote manager for NPC indicators (S3-T06b).
    emotes: EmoteManager,
    /// Progress bar manager (prepared, not yet active) (S3-T06b).
    #[allow(dead_code)]
    progress_bars: ProgressBarManager,
    /// Previous combat log length for detecting new messages.
    prev_combat_log_len: usize,
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
            player_anim: AnimationController::new(),
            player_anim_bank: create_placeholder_anim_bank(),
            culler: FrustumCuller::new(256.0),
            text_renderer: None,
            font_hud: FontId(0),
            font_narrative: FontId(0),
            floating_texts: FloatingTextManager::new(64),
            emotes: EmoteManager::new(),
            progress_bars: ProgressBarManager::new(),
            prev_combat_log_len: 0,
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
        world.player_skills.add_points(1);
        let skill_defs = content::act1_skills();
        let normal_attack = skill_defs[0].id.clone();
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

    /// Initialize TTF text renderer (S3-T06).
    fn init_text_renderer(&mut self) {
        let mut text_renderer = TextRenderer::new(TEX_ARRAY_W, TEX_ARRAY_H);

        // Load HUD font (DigitalDisco).
        let hud_font = load_ttf_font("DigitalDisco.ttf");
        self.font_hud = if let Some(font) = hud_font {
            text_renderer.add_font(font)
        } else {
            FontId(0)
        };

        // Load narrative font (GentiumBookPlus-Regular).
        let narrative_font = load_ttf_font("GentiumBookPlus-Regular.ttf");
        self.font_narrative = if let Some(font) = narrative_font {
            text_renderer.add_font(font)
        } else {
            FontId(0)
        };

        self.text_renderer = Some(text_renderer);
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

    /// Spawn floating texts for new combat log messages (S3-T06b).
    fn spawn_floating_texts_from_combat_log(&mut self) {
        let Some(ref world) = self.world else {
            return;
        };

        let world_log_len = world.combat_log.len();
        if world_log_len > self.prev_combat_log_len {
            let (px, py) = world.player_position();
            let sx = (px - py) * (TILE_WIDTH / 2.0);
            let sy = (px + py) * (TILE_HEIGHT / 2.0);

            for msg in &world.combat_log[self.prev_combat_log_len..] {
                // Heuristic: determine floating text kind from message content.
                let kind = if msg.contains("critical") || msg.contains("Critical") {
                    FloatingTextKind::Critical
                } else if msg.contains("miss") || msg.contains("Miss") || msg.contains("MISS") {
                    FloatingTextKind::Evade
                } else if msg.contains("heal") || msg.contains("Heal") {
                    FloatingTextKind::Heal
                } else if msg.contains("XP") || msg.contains("experience") {
                    FloatingTextKind::Experience
                } else {
                    FloatingTextKind::Damage
                };

                // Extract numeric portion for display if present, else use short msg.
                let display = if msg.len() > 20 {
                    msg.chars().take(20).collect::<String>()
                } else {
                    msg.clone()
                };

                self.floating_texts.spawn([sx, sy - 32.0], display, kind);
            }
            self.prev_combat_log_len = world_log_len;
        }
    }

    /// Update NPC emotes based on player proximity (S3-T06b).
    fn update_npc_emotes(&mut self) {
        let Some(ref world) = self.world else {
            return;
        };

        let (px, py) = world.player_position();

        // Check monsters near the player for emote triggers.
        // For MVP: spawn Exclamation emote on alive monsters close to the player.
        let nearby = world.monsters_near(px, py, 5.0);
        for &(monster_id, mx, my, _) in &nearby {
            let sx = (mx - my) * (TILE_WIDTH / 2.0);
            let sy = (mx + my) * (TILE_HEIGHT / 2.0);
            self.emotes.spawn(monster_id.index, [sx, sy - 40.0], EmoteKind::Exclamation);
        }
    }

    /// Handle a GUI action resulting from user input.
    fn handle_gui_action(&mut self, action: &GuiAction) {
        match *action {
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
            GuiAction::None
            | GuiAction::ToggleCharacter
            | GuiAction::ToggleAutomap
            | GuiAction::ToggleRunWalk
            | GuiAction::ToggleQuestLog => {}
        }
    }

    /// Handle a click on the game world (attack nearest monster or pick up loot).
    fn handle_world_click(&mut self, screen_x: f32, screen_y: f32) {
        let Some(ref mut world) = self.world else {
            return;
        };

        let (sw, sh) = (self.camera.screen_w as f32, self.camera.screen_h as f32);
        let cam_left = self.camera.world_x - sw / (2.0 * self.camera.zoom);
        let cam_top = self.camera.world_y - sh / (2.0 * self.camera.zoom);
        let world_sx = screen_x / self.camera.zoom + cam_left;
        let world_sy = screen_y / self.camera.zoom + cam_top;

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
                match world.player_pickup_loot(loot_idx, 0) {
                    Ok(msg) => tracing::info!("{msg}"),
                    Err(e) => tracing::warn!("Loot pickup failed: {e}"),
                }
                return;
            }
        }

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

    /// Update player animation controller based on movement (S3-T02).
    fn update_player_animation(&mut self) {
        let is_moving = self.move_dirs != 0;
        let new_state = if is_moving {
            AnimationState::Walk
        } else {
            AnimationState::Idle
        };
        self.player_anim.set_state(new_state);

        // Determine direction from movement bitflags.
        let dir = movement_direction(self.move_dirs);
        self.player_anim.set_direction(dir);

        // Tick animation (16ms per frame at 60fps).
        let _events = self.player_anim.tick(16, &self.player_anim_bank);
    }
}

impl Default for SodomightApp {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

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

/// Resize an image to fit the texture array dimensions.
///
/// If the image is already the right size, returns a clone. Otherwise scales
/// to fit within `TEX_ARRAY_W x TEX_ARRAY_H` and pads with transparent black.
fn fit_to_texture_array(img: &image::RgbaImage) -> Vec<u8> {
    let (iw, ih) = img.dimensions();
    let tw = TEX_ARRAY_W;
    let th = TEX_ARRAY_H;

    let mut data = vec![0u8; (tw * th * 4) as usize];

    // Copy the source image into the top-left corner, clamping dimensions.
    let copy_w = iw.min(tw);
    let copy_h = ih.min(th);

    for y in 0..copy_h {
        for x in 0..copy_w {
            let pixel = img.get_pixel(x, y);
            let dst_idx = ((y * tw + x) * 4) as usize;
            data[dst_idx] = pixel[0];
            data[dst_idx + 1] = pixel[1];
            data[dst_idx + 2] = pixel[2];
            data[dst_idx + 3] = pixel[3];
        }
    }

    data
}

/// Create a solid white texture data for the texture array.
fn white_texture_data() -> Vec<u8> {
    vec![255u8; (TEX_ARRAY_W * TEX_ARRAY_H * 4) as usize]
}

/// Try to load a TTF font from the mge/assets/fonts/ directory.
fn load_ttf_font(filename: &str) -> Option<TtfFont> {
    let paths = [
        format!("assets/fonts/{filename}"),
        format!("mge/assets/fonts/{filename}"),
        format!("../mge/assets/fonts/{filename}"),
    ];

    for path in &paths {
        let p = std::path::Path::new(path);
        if p.exists() {
            match TtfFont::from_file(p) {
                Ok(font) => {
                    tracing::info!("Loaded font from {path}");
                    return Some(font);
                }
                Err(e) => {
                    tracing::warn!("Failed to load font {path}: {e}");
                }
            }
        }
    }

    tracing::warn!("Font {filename} not found in any search path");
    None
}

/// Create a placeholder animation bank with single-frame Idle and Walk clips.
fn create_placeholder_anim_bank() -> AnimationBank {
    use mge_render::AnimationClip;

    let mut clips = Vec::new();

    // Create Idle and Walk clips for all 4 rendered directions.
    for &dir in Direction::rendered() {
        clips.push(AnimationClip {
            state: AnimationState::Idle,
            direction: dir,
            frame_count: 1,
            frame_duration_ms: 1000,
            looping: true,
            events: Vec::new(),
            atlas_start_frame: 0,
        });

        clips.push(AnimationClip {
            state: AnimationState::Walk,
            direction: dir,
            frame_count: 1,
            frame_duration_ms: 200,
            looping: true,
            events: Vec::new(),
            atlas_start_frame: 0,
        });
    }

    AnimationBank {
        entity_id: "player_placeholder".to_string(),
        clips,
    }
}

/// Determine facing direction from movement bitflags.
fn movement_direction(move_dirs: u8) -> Direction {
    let up = move_dirs & DIR_UP != 0;
    let down = move_dirs & DIR_DOWN != 0;
    let left = move_dirs & DIR_LEFT != 0;
    let right = move_dirs & DIR_RIGHT != 0;

    match (up, down, left, right) {
        (true, false, false, false) => Direction::NW,
        (false, true, false, false) => Direction::SE,
        (false, false, true, false) => Direction::SW,
        (false, false, false, true) => Direction::NE,
        (true, false, true, false) => Direction::W,
        (true, false, false, true) => Direction::N,
        (false, true, false, true) => Direction::E,
        _ => Direction::S,
    }
}

// ---------------------------------------------------------------------------
// Instanced batching functions (S3-T03)
// ---------------------------------------------------------------------------

/// Push visible isometric tiles as instances into the batcher (S3-T03).
fn batch_tiles_instanced(
    batcher: &mut InstancedSpriteBatcher,
    cam_left: f32,
    cam_top: f32,
    screen_w: f32,
    screen_h: f32,
) {
    let tile_w = TILE_WIDTH;
    let tile_h = TILE_HEIGHT;

    for ty in 0..MAP_H {
        for tx in 0..MAP_W {
            let sx = (tx - ty) as f32 * (tile_w / 2.0);
            let sy = (tx + ty) as f32 * (tile_h / 2.0);

            // Simple frustum cull.
            let screen_x = sx - cam_left;
            let screen_y = sy - cam_top;
            if screen_x + tile_w < 0.0
                || screen_x > screen_w
                || screen_y + tile_h < 0.0
                || screen_y > screen_h
            {
                continue;
            }

            // z_depth: tiles are always behind entities. Use a large z_depth
            // so they sort to the back (ascending sort = back-to-front).
            // Tile depth based on row (tx+ty) for correct iso ordering.
            let row_depth = (tx + ty) as f32;
            let z_depth = 10000.0 + row_depth;

            let instance = InstanceData {
                position: [sx, sy],
                size: [tile_w, tile_h],
                uv_rect: [0.0, 0.0, tile_w / TEX_ARRAY_W as f32, tile_h / TEX_ARRAY_H as f32],
                tint: TILE_TINT,
                texture_index: LAYER_GRASS,
                z_depth,
                _pad: [0.0, 0.0],
            };

            if batcher.push(instance).is_err() {
                return;
            }
        }
    }
}

/// Push monster sprites as instanced coloured quads (S3-T03).
fn batch_monsters_instanced(
    batcher: &mut InstancedSpriteBatcher,
    world: &SodomightWorld,
    visible_ids: &[EntityId],
) {
    let monster_size = 24.0_f32;

    for &vis_id in visible_ids {
        if !world.ecs.is_alive(vis_id) {
            continue;
        }

        let Ok(mr) = world.ecs.get_component::<MonsterRecord>(vis_id) else {
            continue;
        };

        if !mr.health.is_alive() {
            continue;
        }

        let mx = mr.position.x();
        let my = mr.position.y();

        let sx = (mx - my) * (TILE_WIDTH / 2.0);
        let sy = (mx + my) * (TILE_HEIGHT / 2.0);

        let offset = (TILE_WIDTH - monster_size) / 2.0;

        // z_depth: based on Y position for iso depth sorting. Lower z = drawn later (in front).
        // Entity tile_y = (mx+my), higher tile_y = closer to camera = smaller z_depth.
        let tile_y_sum = mx + my;
        let z_depth = 5000.0 - tile_y_sum;

        let instance = InstanceData {
            position: [sx + offset, sy - monster_size],
            size: [monster_size, monster_size],
            uv_rect: [0.0, 0.0, 1.0, 1.0],
            tint: MONSTER_TINT,
            texture_index: LAYER_WHITE,
            z_depth,
            _pad: [0.0, 0.0],
        };

        if batcher.push(instance).is_err() {
            return;
        }
    }
}

/// Push loot piles as instanced coloured quads (S3-T03).
fn batch_loot_instanced(
    batcher: &mut InstancedSpriteBatcher,
    world: &SodomightWorld,
    cam_left: f32,
    cam_top: f32,
    screen_w: f32,
    screen_h: f32,
) {
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
        let tile_y_sum = lx + ly;
        let z_depth = 5000.0 - tile_y_sum + 0.1;

        let instance = InstanceData {
            position: [sx + offset, sy],
            size: [loot_size, loot_size],
            uv_rect: [0.0, 0.0, 1.0, 1.0],
            tint: LOOT_TINT,
            texture_index: LAYER_WHITE,
            z_depth,
            _pad: [0.0, 0.0],
        };

        if batcher.push(instance).is_err() {
            return;
        }
    }
}

/// Push the player character as an instanced coloured quad (S3-T03).
fn batch_player_instanced(
    batcher: &mut InstancedSpriteBatcher,
    px: f32,
    py: f32,
    cam_left: f32,
    cam_top: f32,
    screen_w: f32,
    screen_h: f32,
) {
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
    let tile_y_sum = px + py;
    let z_depth = 5000.0 - tile_y_sum;

    let instance = InstanceData {
        position: [sx + offset, sy - player_size],
        size: [player_size, player_size],
        uv_rect: [0.0, 0.0, 1.0, 1.0],
        tint: PLAYER_TINT,
        texture_index: LAYER_WHITE,
        z_depth,
        _pad: [0.0, 0.0],
    };

    let _ = batcher.push(instance);
}

// ---------------------------------------------------------------------------
// GameApp implementation
// ---------------------------------------------------------------------------

impl GameApp for SodomightApp {
    #[allow(clippy::too_many_lines)]
    fn on_init(&mut self, gpu: &GpuContext) {
        let (w, h) = gpu.surface_size();
        self.camera = Camera2D::new(w, h);
        self.gui.set_screen_size(w as f32, h as f32);

        // Init game world.
        self.init_world();

        // Set camera to player position.
        if let Some(ref world) = self.world {
            let (px, py) = world.player_position();
            self.camera.follow(px, py);
        }

        // Init text renderer (S3-T06).
        self.init_text_renderer();

        // --- Instanced pipeline setup (S3-T01, S3-T03) ---
        let instanced_pipeline = InstancedSpritePipeline::new(&gpu.device, gpu.surface_format());
        let instanced_batcher = InstancedSpriteBatcher::new(MAX_INSTANCES);

        // Create texture array (S3-T01).
        let mut texture_array = TextureArray::new(
            &gpu.device,
            TEX_ARRAY_W,
            TEX_ARRAY_H,
            MAX_TEXTURE_LAYERS,
        );

        // Layer 0: grass tile.
        let grass_img = load_grass_image().unwrap_or_else(checkerboard_fallback);
        let grass_data = fit_to_texture_array(&grass_img);
        if let Err(e) = texture_array.add_layer(&gpu.queue, &grass_data) {
            tracing::error!("Failed to add grass layer to texture array: {e}");
        }

        // Layer 1: solid white (for tinted quads: monsters, loot, player, GUI).
        let white_data = white_texture_data();
        if let Err(e) = texture_array.add_layer(&gpu.queue, &white_data) {
            tracing::error!("Failed to add white layer to texture array: {e}");
        }

        // Layer 2: glyph atlas (initially transparent, updated when text is rendered).
        let glyph_data = vec![0u8; (TEX_ARRAY_W * TEX_ARRAY_H * 4) as usize];
        if let Err(e) = texture_array.add_layer(&gpu.queue, &glyph_data) {
            tracing::error!("Failed to add glyph atlas layer to texture array: {e}");
        }

        // Create camera uniform buffer + bind group.
        let camera_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instanced Camera Uniform Buffer"),
            size: 64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera_bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Instanced Camera Bind Group"),
            layout: &instanced_pipeline.camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        // Create sampler.
        let sampler = gpu.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Instanced Sprite Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        // Create texture bind group (texture array + sampler).
        let texture_bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Instanced Texture Array Bind Group"),
            layout: &instanced_pipeline.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture_array.view()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        // Create storage buffer for instance data.
        let storage_buffer_size = (MAX_INSTANCES * std::mem::size_of::<InstanceData>()) as u64;
        let storage_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instanced Storage Buffer"),
            size: storage_buffer_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let storage_bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Instanced Storage Bind Group"),
            layout: &instanced_pipeline.storage_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: storage_buffer.as_entire_binding(),
            }],
        });

        // --- Legacy pipeline (for GUI only) ---
        let legacy_pipeline = SpritePipeline::new(&gpu.device, gpu.surface_format());
        let legacy_batcher = SpriteBatcher::new(&gpu.device, 4096);
        let gui_texture = gui::create_white_texture(&gpu.device, &gpu.queue, &legacy_pipeline);

        let layer_count = texture_array.layer_count();

        self.gpu_res = Some(InstancedGpuResources {
            instanced_pipeline,
            instanced_batcher,
            texture_array,
            storage_buffer,
            camera_buffer,
            camera_bind_group,
            texture_bind_group,
            storage_bind_group,
            sampler,
            legacy_pipeline,
            legacy_batcher,
            gui_texture,
        });

        tracing::info!(
            "Sodomight initialised: {w}x{h}, instanced pipeline + {layer_count} texture layers",
        );
    }

    #[allow(clippy::too_many_lines)]
    fn on_frame(&mut self, gpu: &GpuContext) {
        // --- Fixed timestep ticks ---
        let ticks = self.game_loop.begin_frame();

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

            if let Some(ref mut world) = self.world {
                world.set_player_position(px, py);
                world.tick();
            }
        }

        // Update camera to follow player.
        self.camera.follow(px, py);

        let alpha = self.game_loop.alpha();
        self.camera.update(alpha);

        // Update player animation (S3-T02).
        self.update_player_animation();

        // Sync GUI from world state.
        self.sync_gui_from_world();

        // Spawn floating texts from new combat log messages (S3-T06b).
        self.spawn_floating_texts_from_combat_log();

        // Update NPC emotes (S3-T06b).
        self.update_npc_emotes();

        // Tick overhead UI managers (S3-T06b).
        let dt = 1.0 / 60.0;
        self.floating_texts.tick(dt);
        self.emotes.tick(dt);

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

        let (sw, sh) = gpu.surface_size();
        let screen_w = sw as f32;
        let screen_h = sh as f32;
        let cam_left = self.camera.world_x - screen_w / (2.0 * self.camera.zoom);
        let cam_top = self.camera.world_y - screen_h / (2.0 * self.camera.zoom);

        // Update camera uniform (instanced pipeline).
        let ew = screen_w / self.camera.zoom;
        let eh = screen_h / self.camera.zoom;
        #[rustfmt::skip]
        let proj: [[f32; 4]; 4] = [
            [ 2.0 / ew,                                  0.0,   0.0, 0.0],
            [      0.0,                        -2.0 / eh,       0.0, 0.0],
            [      0.0,                                  0.0,   1.0, 0.0],
            [-(1.0 + 2.0 * cam_left / ew), 1.0 + 2.0 * cam_top / eh, 0.0, 1.0],
        ];
        gpu.queue.write_buffer(
            &res.camera_buffer,
            0,
            bytemuck::cast_slice(&proj),
        );

        // --- Build instanced batch (S3-T03) ---
        res.instanced_batcher.clear();

        // Tiles (S3-T03).
        batch_tiles_instanced(
            &mut res.instanced_batcher,
            cam_left,
            cam_top,
            screen_w,
            screen_h,
        );

        // Entity frustum culling (S3-T04).
        if let Some(ref world) = self.world {
            // Build render entities for the culler.
            // We use entity_id.index as the u32 ID for the culler, and store
            // a mapping from index -> EntityId for reconstruction.
            let mut render_entities = Vec::new();
            let mut id_map: Vec<EntityId> = Vec::new();
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
                let sx = (mx - my) * (TILE_WIDTH / 2.0);
                let sy = (mx + my) * (TILE_HEIGHT / 2.0);

                let culler_id = id_map.len() as u32;
                id_map.push(entity_id);
                render_entities.push(RenderEntity {
                    id: culler_id,
                    position: [sx, sy],
                    size: [24.0, 24.0],
                });
            }

            // Rebuild spatial grid and cull (S3-T04).
            self.culler.rebuild(&render_entities);
            let visible_rect = self.camera.visible_rect(screen_w, screen_h);
            let visible_culler_ids = self.culler.cull(visible_rect);

            // Map culler IDs back to EntityIds for the batcher.
            let visible_entity_ids: Vec<EntityId> = visible_culler_ids
                .iter()
                .filter_map(|&cid| id_map.get(cid as usize).copied())
                .collect();

            // Batch visible entities (S3-T03 + S3-T04 + S3-T05 depth sorting).
            batch_monsters_instanced(&mut res.instanced_batcher, world, &visible_entity_ids);
            batch_loot_instanced(
                &mut res.instanced_batcher,
                world,
                cam_left,
                cam_top,
                screen_w,
                screen_h,
            );
        }

        // Player (S3-T03).
        batch_player_instanced(
            &mut res.instanced_batcher,
            px,
            py,
            cam_left,
            cam_top,
            screen_w,
            screen_h,
        );

        // Sort all instances by z_depth (S3-T05).
        res.instanced_batcher.sort_by_depth();

        // Upload instance data to GPU storage buffer.
        let instance_count = res.instanced_batcher.len() as u32;
        let instance_bytes = res.instanced_batcher.as_bytes();
        if !instance_bytes.is_empty() {
            gpu.queue.write_buffer(&res.storage_buffer, 0, instance_bytes);
        }

        // --- Render ---
        let (mut encoder, view, output) = frame.into_parts();

        // Clear pass.
        {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("clear_pass"),
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
            // Pass drops here, clearing the framebuffer.
        }

        // Instanced world draw (S3-T03): single draw call for tiles + entities.
        res.instanced_pipeline.render(
            &mut encoder,
            &view,
            &res.camera_bind_group,
            &res.texture_bind_group,
            &res.storage_bind_group,
            instance_count,
        );

        // --- GUI pass (legacy pipeline, screen space) ---
        res.legacy_pipeline.update_camera(
            &gpu.queue,
            screen_w,
            screen_h,
            0.0,
            0.0,
            1.0,
        );

        res.legacy_batcher.begin();
        self.gui.draw(&mut res.legacy_batcher);
        let gui_vert_count = res.legacy_batcher.flush(&gpu.queue);

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

            pass.set_pipeline(&res.legacy_pipeline.render_pipeline);
            pass.set_bind_group(0, &res.legacy_pipeline.camera_bind_group, &[]);
            pass.set_bind_group(1, &res.gui_texture.bind_group, &[]);
            res.legacy_batcher.draw(&mut pass, gui_vert_count);
        }

        gpu.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.frame_count += 1;
        if self.frame_count.is_multiple_of(300) {
            let player_pos = self
                .world
                .as_ref()
                .map_or((0.0, 0.0), SodomightWorld::player_position);

            tracing::debug!(
                "Frame {} | Player ({:.1}, {:.1}) | Camera ({:.1}, {:.1}) | Instanced: {} | GUI quads: {} | Floating texts: {} | Emotes: {}",
                self.frame_count,
                player_pos.0,
                player_pos.1,
                self.camera.world_x,
                self.camera.world_y,
                instance_count,
                gui_vert_count / 4,
                self.floating_texts.len(),
                self.emotes.len(),
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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

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

        let world = app.world.as_ref().expect("world should exist after init");
        // Player + 6 monsters (3 fallen + 3 quill rats).
        assert!(world.ecs.entity_count() >= 7);
    }

    #[test]
    fn test_skill_bar_keys() {
        let mut app = SodomightApp::new();
        app.on_input(InputEvent::KeyDown { key: KeyCode::Num1 });
        // Skill use without world is a no-op; verify no panic.
    }

    #[test]
    fn test_movement_direction_mapping() {
        assert_eq!(movement_direction(DIR_UP), Direction::NW);
        assert_eq!(movement_direction(DIR_DOWN), Direction::SE);
        assert_eq!(movement_direction(DIR_LEFT), Direction::SW);
        assert_eq!(movement_direction(DIR_RIGHT), Direction::NE);
        assert_eq!(movement_direction(DIR_UP | DIR_LEFT), Direction::W);
        assert_eq!(movement_direction(DIR_DOWN | DIR_RIGHT), Direction::E);
        assert_eq!(movement_direction(0), Direction::S);
    }

    #[test]
    fn test_placeholder_anim_bank_valid() {
        let bank = create_placeholder_anim_bank();
        assert_eq!(bank.entity_id, "player_placeholder");
        assert_eq!(bank.clips.len(), 8); // 2 states * 4 directions

        // Check that Idle and Walk clips exist for all rendered directions.
        for &dir in Direction::rendered() {
            assert!(
                bank.get_clip(AnimationState::Idle, dir).is_some(),
                "missing Idle clip for {dir:?}"
            );
            assert!(
                bank.get_clip(AnimationState::Walk, dir).is_some(),
                "missing Walk clip for {dir:?}"
            );
        }
    }

    #[test]
    fn test_fit_to_texture_array_small_image() {
        let img = image::RgbaImage::new(32, 16);
        let data = fit_to_texture_array(&img);
        let expected_size = (TEX_ARRAY_W * TEX_ARRAY_H * 4) as usize;
        assert_eq!(data.len(), expected_size);
    }

    #[test]
    fn test_white_texture_data() {
        let data = white_texture_data();
        let expected_size = (TEX_ARRAY_W * TEX_ARRAY_H * 4) as usize;
        assert_eq!(data.len(), expected_size);
        assert!(data.iter().all(|&b| b == 255));
    }

    #[test]
    fn test_instanced_batcher_tiles() {
        let mut batcher = InstancedSpriteBatcher::new(MAX_INSTANCES);
        batch_tiles_instanced(&mut batcher, 0.0, 0.0, 1280.0, 720.0);
        // Should have pushed some tiles (not all 32x32 = 1024, but the visible ones).
        assert!(batcher.len() > 0);
        assert!(batcher.len() <= 1024);
    }

    #[test]
    fn test_player_animation_controller() {
        let mut app = SodomightApp::new();
        assert_eq!(app.player_anim.current_state(), AnimationState::Idle);

        app.press_dir(DIR_UP);
        app.update_player_animation();
        assert_eq!(app.player_anim.current_state(), AnimationState::Walk);
        assert_eq!(app.player_anim.current_direction(), Direction::NW);

        app.release_dir(DIR_UP);
        app.update_player_animation();
        assert_eq!(app.player_anim.current_state(), AnimationState::Idle);
    }

    #[test]
    fn test_floating_text_manager_init() {
        let app = SodomightApp::new();
        assert!(app.floating_texts.is_empty());
        assert!(app.emotes.is_empty());
        assert!(app.progress_bars.is_empty());
    }
}
