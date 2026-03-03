// @id: Sodomight-Client-Game @do: game-app @role: back-end @layer: 4 @human: miyuk
//! Sodomight game application -- implements the mge-platform GameApp trait.
//!
//! Sprint 3 migration: uses instanced pipeline (TextureArray, InstancedSpriteBatcher,
//! FrustumCuller), AnimationController, TextRenderer (TTF/OTF), and Overhead UI.

use mge_core::game_loop::{GameLoop, LoopConfig};
use mge_platform::{GameApp, GpuContext, InputEvent, KeyCode, MouseButton};
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
use crate::tilemap::{Tile, TileMap};

/// Bitflag constants for movement directions.
const DIR_UP: u8 = 0b0001;
const DIR_DOWN: u8 = 0b0010;
const DIR_LEFT: u8 = 0b0100;
const DIR_RIGHT: u8 = 0b1000;

/// Map size in tiles (procedurally generated).
const MAP_W: i32 = 64;
const MAP_H: i32 = 64;

/// Player run speed in world units per tick.
const RUN_SPEED: f32 = 0.1;
/// Player walk speed in world units per tick.
const WALK_SPEED: f32 = 0.05;

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
/// Texture array layer index for the stone floor tile.
const LAYER_STONE: u32 = 3;

/// Wall tile tint (dark grey stone).
const WALL_TINT: [f32; 4] = [0.25, 0.22, 0.20, 1.0];
/// Stone floor tint (slightly desaturated tan).
const FLOOR_TINT: [f32; 4] = [0.65, 0.58, 0.45, 1.0];
/// Path/corridor tint (dirt brown).
const PATH_TINT: [f32; 4] = [0.50, 0.42, 0.30, 1.0];
/// Water tint (dark blue).
const WATER_TINT: [f32; 4] = [0.15, 0.20, 0.45, 1.0];

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
    /// Bitmap font for HUD text rendering.
    bitmap_font: crate::bitmap_font::BitmapFont,
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
    /// Procedural dungeon tilemap.
    tilemap: Option<TileMap>,
    /// Target position for click-to-move (world coords).
    move_target: Option<(f32, f32)>,
    /// Current mouse position (screen coords).
    mouse_screen: (f32, f32),
    /// Whether left mouse button is currently held (for auto-attack).
    left_mouse_held: bool,
    /// Cooldown ticks remaining before next auto-attack.
    auto_attack_cooldown: u32,
    /// Whether the player is dead (waiting for respawn).
    player_dead: bool,
    /// Whether Alt key is held (show loot labels on ground).
    show_loot_labels: bool,
}

/// Extract the first numeric substring from a combat log message.
/// Returns the number as a string, or "?" if none found.
fn extract_number(msg: &str) -> String {
    let mut start = None;
    for (i, ch) in msg.char_indices() {
        if ch.is_ascii_digit() {
            if start.is_none() {
                start = Some(i);
            }
        } else if start.is_some() {
            // SAFETY: char_indices guarantees valid UTF-8 boundaries.
            #[allow(clippy::unwrap_used)]
            return msg[start.unwrap()..i].to_owned();
        }
    }
    if let Some(s) = start {
        return msg[s..].to_owned();
    }
    "?".to_owned()
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
            tilemap: None,
            move_target: None,
            mouse_screen: (0.0, 0.0),
            left_mouse_held: false,
            auto_attack_cooldown: 0,
            player_dead: false,
            show_loot_labels: false,
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

        // Generate procedural dungeon tilemap.
        let monsters = content::act1_monsters();
        let tilemap = TileMap::generate_dungeon(
            MAP_W,
            MAP_H,
            &mut world.rng,
            monsters.len().min(5), // Use first 5 monster types
        );

        // Set player position to tilemap's spawn point.
        let (spawn_x, spawn_y) = tilemap.player_spawn;
        world.set_player_position(spawn_x, spawn_y);

        // Spawn monsters at generated spawn points.
        for &(mx, my, type_idx) in &tilemap.spawn_points {
            let mdef = &monsters[type_idx.min(monsters.len() - 1)];
            if let Err(e) = world.spawn_monster(
                &mdef.name,
                mx,
                my,
                mdef.level,
                mdef.health as u32,
            ) {
                tracing::warn!("Failed to spawn {}: {e}", mdef.name);
            }
        }

        tracing::info!(
            "World initialised: {} entities, map {}x{}, {} spawn points",
            world.ecs.entity_count(),
            tilemap.width,
            tilemap.height,
            tilemap.spawn_points.len(),
        );

        // Send tilemap data to minimap for dungeon layout rendering.
        self.gui
            .set_minimap_tiles(tilemap.width, tilemap.height, tilemap.minimap_data());

        // Build automap tile positions (all non-void tiles for Tab overlay).
        let mut automap_tiles = Vec::new();
        for y in 0..tilemap.height {
            for x in 0..tilemap.width {
                let tile = tilemap.get(x, y);
                if tile != Tile::Void {
                    automap_tiles.push((x as f32, y as f32));
                }
            }
        }
        self.gui.update_automap_tiles(&automap_tiles);

        self.tilemap = Some(tilemap);
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

        let gold = world.player_gold;
        self.gui.update_from_world(
            hp, hp_max, mana, mana_max, xp, xp_next, level, gold,
        );

        // Sync character stats (for C panel).
        let base = &world.player_stats.base;
        let derived = &world.player_stats.derived;
        #[allow(clippy::cast_possible_wrap)]
        let stat_pts = world.player_stats.level.stat_points as i32;
        self.gui.update_character_stats(
            base.strength.effective(),
            base.dexterity.effective(),
            base.vitality.effective(),
            base.energy.effective(),
            stat_pts,
            derived.defense_rating,
            derived.min_damage,
            derived.max_damage,
            derived.attack_rating,
        );

        // Sync skill points.
        #[allow(clippy::cast_possible_wrap)]
        let skill_pts = world.player_stats.level.skill_points as i32;
        self.gui.set_skill_points(skill_pts);

        // Sync combat log messages.
        let gui_log_len = self.gui.combat_log().len();
        let world_log = &world.combat_log;
        if world_log.len() > gui_log_len {
            for msg in &world_log[gui_log_len..] {
                self.gui.push_combat_message(msg.clone());
            }
        }

        // Sync minimap: player position + alive monster positions.
        let (px, py) = world.player_position();
        let monster_positions: Vec<(f32, f32)> = world
            .ai_agents_keys()
            .filter_map(|&eid| {
                let mr = world.ecs.get_component::<MonsterRecord>(eid).ok()?;
                if mr.health.is_alive() {
                    Some((mr.position.x(), mr.position.y()))
                } else {
                    None
                }
            })
            .collect();
        self.gui.update_minimap((px, py), &monster_positions);
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
                let (kind, display) =
                    if msg.contains("critical") || msg.contains("Critical") {
                        // Critical: extract damage number, Ragnarok Online style.
                        let num = extract_number(msg);
                        (FloatingTextKind::Critical, num)
                    } else if msg.contains("miss") || msg.contains("Miss") || msg.contains("MISS") {
                        (FloatingTextKind::Evade, "DODGE".to_owned())
                    } else if msg.contains("block") || msg.contains("Block") || msg.contains("BLOCK") {
                        (FloatingTextKind::Block, "BLOCK".to_owned())
                    } else if msg.contains("heal") || msg.contains("Heal") {
                        let num = extract_number(msg);
                        (FloatingTextKind::Heal, format!("+{num}"))
                    } else if msg.contains("XP") || msg.contains("experience") {
                        let num = extract_number(msg);
                        (FloatingTextKind::Experience, format!("+{num} XP"))
                    } else if msg.contains("hit") || msg.contains("damage") {
                        // Normal damage: just the number, RO style.
                        let num = extract_number(msg);
                        (FloatingTextKind::Damage, num)
                    } else {
                        // Fallback: short truncated message.
                        let short = if msg.len() > 20 {
                            msg.chars().take(20).collect::<String>()
                        } else {
                            msg.clone()
                        };
                        (FloatingTextKind::Damage, short)
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
            GuiAction::ClickInventorySlot(slot) => {
                self.handle_inventory_click(slot);
            }
            GuiAction::RightClickWorld(sx, sy) => {
                self.handle_right_click_world(sx, sy);
            }
            GuiAction::AllocateStat(stat_idx) => {
                self.allocate_stat_point(stat_idx);
            }
            GuiAction::InvestSkill(skill_idx) => {
                self.invest_skill_point(skill_idx);
            }
            GuiAction::ClickWorld(sx, sy) => {
                self.handle_world_click(sx, sy);
            }
            GuiAction::UseBeltPotion(slot) => {
                self.use_belt_potion(slot);
            }
            GuiAction::ToggleCharacter => self.gui.toggle_character(),
            GuiAction::ToggleAutomap => self.gui.toggle_automap(),
            GuiAction::ToggleRunWalk => self.gui.toggle_run_walk(),
            GuiAction::ToggleQuestLog => self.gui.toggle_quest_log(),
            GuiAction::None => {}
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
            return;
        }

        // No loot or monster found: click-to-move to the target tile.
        let walkable = self
            .tilemap
            .as_ref()
            .is_none_or(|t| t.is_walkable(tile_x, tile_y));
        if walkable {
            self.move_target = Some((tile_x, tile_y));
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

    /// Use a belt potion slot (D2-style: 1-4 keys).
    /// Slot 0-1 = health potion, slot 2 = mana potion, slot 3 = reserved.
    fn use_belt_potion(&mut self, slot: usize) {
        let Some(ref mut world) = self.world else {
            return;
        };
        // Belt layout: slots 0-1 default HP, slot 2 default mana.
        let result = match slot {
            0 | 1 => world.use_health_potion(),
            2 | 3 => world.use_mana_potion(),
            _ => return,
        };
        match result {
            Ok(msg) => tracing::info!("{msg}"),
            Err(e) => tracing::debug!("Potion use failed: {e}"),
        }
    }

    /// Auto-attack: if left mouse is held and a monster is nearby, attack it.
    fn tick_auto_attack(&mut self) {
        if !self.left_mouse_held || self.player_dead {
            return;
        }
        if self.auto_attack_cooldown > 0 {
            self.auto_attack_cooldown -= 1;
            return;
        }

        let (mx, my) = self.mouse_screen;
        let Some(ref mut world) = self.world else {
            return;
        };

        // Convert screen to world tile coords.
        let (sw, sh) = (self.camera.screen_w as f32, self.camera.screen_h as f32);
        let cam_left = self.camera.world_x - sw / (2.0 * self.camera.zoom);
        let cam_top = self.camera.world_y - sh / (2.0 * self.camera.zoom);
        let world_sx = mx / self.camera.zoom + cam_left;
        let world_sy = my / self.camera.zoom + cam_top;
        let tile_x = (world_sx / (TILE_WIDTH / 2.0) + world_sy / (TILE_HEIGHT / 2.0)) / 2.0;
        let tile_y = (world_sy / (TILE_HEIGHT / 2.0) - world_sx / (TILE_WIDTH / 2.0)) / 2.0;

        let monsters_near = world.monsters_near(tile_x, tile_y, ATTACK_RANGE);
        if let Some(&(monster_id, _, _, _)) = monsters_near.first() {
            match world.player_attack(monster_id) {
                Ok(msgs) => {
                    for msg in &msgs {
                        tracing::debug!("{msg}");
                    }
                }
                Err(e) => tracing::debug!("Auto-attack failed: {e}"),
            }
            // ~2 attacks per second at 60fps.
            self.auto_attack_cooldown = 30;
        }
    }

    /// Auto-pickup: collect loot piles the player walks over.
    fn tick_auto_pickup(&mut self) {
        let Some(ref mut world) = self.world else {
            return;
        };
        let (px, py) = world.player_position();
        // Check all loot piles within pickup range.
        let mut picked = true;
        while picked {
            picked = false;
            for i in 0..world.pending_loot.len() {
                let (lx, ly, _) = &world.pending_loot[i];
                let dx = lx - px;
                let dy = ly - py;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < 1.5 {
                    match world.player_pickup_loot(i, 0) {
                        Ok(msg) => tracing::info!("{msg}"),
                        Err(e) => tracing::warn!("Auto-pickup failed: {e}"),
                    }
                    picked = true;
                    break; // Index shifted, restart.
                }
            }
        }
    }

    /// Check if player is dead and handle death state.
    fn check_player_death(&mut self) {
        let Some(ref world) = self.world else {
            return;
        };
        let (hp, _) = world.player_health();
        if hp <= 0 && !self.player_dead {
            self.player_dead = true;
            self.move_target = None;
            tracing::info!("Player has died! Press Space to respawn.");
        }
    }

    /// Respawn the player at the dungeon spawn point.
    fn respawn_player(&mut self) {
        let Some(ref mut world) = self.world else {
            return;
        };
        // Restore health to full.
        let (_, max_hp) = world.player_health();
        world.player_stats.restore_life(max_hp);
        let (_, max_mana) = world.player_mana();
        world.player_stats.restore_mana(max_mana);

        // Move to spawn point.
        if let Some(ref tilemap) = self.tilemap {
            let (sx, sy) = tilemap.player_spawn;
            world.set_player_position(sx, sy);
        }

        self.player_dead = false;
        tracing::info!("Player respawned.");
    }

    /// Handle right-click on the game world: cast the right-assigned skill on the nearest monster.
    fn handle_right_click_world(&mut self, screen_x: f32, screen_y: f32) {
        let Some(ref mut world) = self.world else {
            return;
        };

        // Convert screen to world tile coords.
        let (sw, sh) = (self.camera.screen_w as f32, self.camera.screen_h as f32);
        let cam_left = self.camera.world_x - sw / (2.0 * self.camera.zoom);
        let cam_top = self.camera.world_y - sh / (2.0 * self.camera.zoom);
        let world_sx = screen_x / self.camera.zoom + cam_left;
        let world_sy = screen_y / self.camera.zoom + cam_top;
        let tile_x = (world_sx / (TILE_WIDTH / 2.0) + world_sy / (TILE_HEIGHT / 2.0)) / 2.0;
        let tile_y = (world_sy / (TILE_HEIGHT / 2.0) - world_sx / (TILE_WIDTH / 2.0)) / 2.0;

        // Find the right-click skill from Act 1 skills (default: index 1 = Fire Bolt).
        let skills = content::act1_skills();
        let right_skill = skills.get(1).or_else(|| skills.first());
        let Some(skill_def) = right_skill else {
            return;
        };

        let monsters_near = world.monsters_near(tile_x, tile_y, ATTACK_RANGE * 2.0);
        let target = monsters_near.first().map(|&(id, _, _, _)| id);

        match world.player_use_skill(&skill_def.id, target) {
            Ok(msgs) => {
                for msg in &msgs {
                    tracing::debug!("{msg}");
                }
            }
            Err(e) => tracing::debug!("Right-click skill failed: {e}"),
        }
    }

    /// Handle clicking on an inventory slot: attempt to equip the item.
    fn handle_inventory_click(&mut self, slot: usize) {
        let Some(ref mut world) = self.world else {
            return;
        };

        let col = slot % 10; // INV_COLS
        let row = slot / 10;

        // Check if there's an item in this slot.
        let item = world.player_inventory.get(col, row);
        let Some(item) = item else {
            return;
        };

        // Determine equip slot from item type (use base_id heuristic).
        let base_id = item.base_id.clone();
        let equip_slot = guess_equip_slot(&base_id);

        match world.player_equip(col, row, equip_slot) {
            Ok(msg) => tracing::info!("{msg}"),
            Err(e) => tracing::debug!("Equip failed: {e}"),
        }
    }

    /// Allocate a stat point to a primary attribute (0=str, 1=dex, 2=vit, 3=ene).
    fn allocate_stat_point(&mut self, stat_idx: u8) {
        let Some(ref mut world) = self.world else {
            return;
        };

        if !world.player_stats.level.spend_stat_point() {
            tracing::debug!("No stat points available.");
            return;
        }

        match stat_idx {
            0 => world.player_stats.base.strength.add(1),
            1 => world.player_stats.base.dexterity.add(1),
            2 => world.player_stats.base.vitality.add(1),
            3 => world.player_stats.base.energy.add(1),
            _ => return,
        }

        // Recalculate derived stats after allocation.
        world.player_stats.recalculate();

        let stat_name = match stat_idx {
            0 => "Strength",
            1 => "Dexterity",
            2 => "Vitality",
            3 => "Energy",
            _ => "Unknown",
        };
        tracing::info!("Allocated 1 point to {stat_name}.");
    }

    /// Invest a skill point into a skill at the given slot index.
    fn invest_skill_point(&mut self, skill_idx: usize) {
        let Some(ref mut world) = self.world else {
            return;
        };

        let skills = content::act1_skills();
        let Some(skill_def) = skills.get(skill_idx) else {
            tracing::debug!("No skill at index {skill_idx}.");
            return;
        };

        if !world.player_stats.level.spend_skill_point() {
            tracing::debug!("No skill points available.");
            return;
        }

        let registry_clone = world.skill_registry.clone();
        match world.player_skills.invest(&skill_def.id, &registry_clone) {
            Ok(()) => {
                let new_level = world.player_skills.level_of(&skill_def.id);
                tracing::info!("Invested in {}: now level {new_level}.", skill_def.name);
            }
            Err(e) => {
                // Refund the point if invest fails.
                world.player_stats.level.skill_points += 1;
                tracing::debug!("Skill invest failed: {e}");
            }
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

/// Generate a procedural stone floor texture (grey noise with crack lines).
fn generate_stone_texture() -> Vec<u8> {
    let w = TEX_ARRAY_W as usize;
    let h = TEX_ARRAY_H as usize;
    let mut data = vec![0u8; w * h * 4];

    // Simple pseudo-random noise for stone appearance.
    let mut seed: u32 = 0xCAFE_BABE;
    for y in 0..h {
        for x in 0..w {
            // LCG pseudo-random.
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let noise = ((seed >> 16) & 0xFF) as u8;

            // Base stone colour: grey with slight warm tint.
            let base_r = 180_u8.saturating_add(noise / 8);
            let base_g = 170_u8.saturating_add(noise / 10);
            let base_b = 155_u8.saturating_add(noise / 12);

            // Add grid lines for tile/brick pattern.
            let grid_x = x % 32 == 0 || x % 32 == 31;
            let grid_y = y % 32 == 0 || y % 32 == 31;
            let on_grid = grid_x || grid_y;

            let idx = (y * w + x) * 4;
            if on_grid {
                // Dark mortar lines.
                data[idx] = 60;
                data[idx + 1] = 55;
                data[idx + 2] = 50;
            } else {
                data[idx] = base_r;
                data[idx + 1] = base_g;
                data[idx + 2] = base_b;
            }
            data[idx + 3] = 255;
        }
    }
    data
}

/// Try to load a TTF font from the mge/assets/fonts/ directory.
fn load_ttf_font(filename: &str) -> Option<TtfFont> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let paths = [
        format!("assets/fonts/{filename}"),
        format!("mge/assets/fonts/{filename}"),
        format!("../mge/assets/fonts/{filename}"),
        format!("{manifest_dir}/../../assets/fonts/{filename}"),
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

/// Guess the equipment slot based on item base_id naming convention.
fn guess_equip_slot(base_id: &str) -> mge_arpg_items::ItemSlot {
    use mge_arpg_items::ItemSlot;
    let lower = base_id.to_lowercase();
    if lower.contains("helm") || lower.contains("cap") || lower.contains("crown") {
        ItemSlot::Helm
    } else if lower.contains("armor") || lower.contains("plate") || lower.contains("robe") {
        ItemSlot::Armor
    } else if lower.contains("glove") || lower.contains("gauntlet") {
        ItemSlot::Gloves
    } else if lower.contains("belt") || lower.contains("sash") {
        ItemSlot::Belt
    } else if lower.contains("boot") || lower.contains("greave") {
        ItemSlot::Boots
    } else if lower.contains("amulet") || lower.contains("necklace") {
        ItemSlot::Amulet
    } else if lower.contains("ring") {
        ItemSlot::Ring1
    } else if lower.contains("shield") || lower.contains("buckler") {
        ItemSlot::WeaponOff
    } else {
        // Default: treat as weapon.
        ItemSlot::WeaponMain
    }
}

/// Map a loot quality to D2-style colors for item labels.
fn quality_color(quality: mge_arpg_loot::DropQuality) -> [f32; 4] {
    use mge_arpg_loot::DropQuality;
    match quality {
        DropQuality::Normal => [1.0, 1.0, 1.0, 1.0],       // White
        DropQuality::Magic => [0.35, 0.45, 1.0, 1.0],       // Blue
        DropQuality::Rare => [1.0, 1.0, 0.2, 1.0],          // Yellow
        DropQuality::Unique => [0.65, 0.5, 0.15, 1.0],      // Gold/brown
        DropQuality::Set => [0.0, 0.85, 0.0, 1.0],          // Green
    }
}

// ---------------------------------------------------------------------------
// Instanced batching functions (S3-T03)
// ---------------------------------------------------------------------------

/// Push visible isometric tiles as instances into the batcher (S3-T03).
///
/// Uses the procedural tilemap to render different tile types with distinct
/// textures and tints (grass, stone floor, walls, water, paths).
fn batch_tiles_instanced(
    batcher: &mut InstancedSpriteBatcher,
    tilemap: Option<&TileMap>,
    cam_left: f32,
    cam_top: f32,
    screen_w: f32,
    screen_h: f32,
) {
    let tile_w = TILE_WIDTH;
    let tile_h = TILE_HEIGHT;

    let map_w = tilemap.map_or(MAP_W, |t| t.width);
    let map_h = tilemap.map_or(MAP_H, |t| t.height);

    for ty in 0..map_h {
        for tx in 0..map_w {
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

            let tile = tilemap.map_or(Tile::Grass, |t| t.get(tx, ty));

            // Skip void tiles (nothing to render).
            if tile == Tile::Void {
                continue;
            }

            // Choose texture layer and tint based on tile type.
            let (texture_index, tint) = match tile {
                Tile::Grass => (LAYER_GRASS, TILE_TINT),
                Tile::Floor => (LAYER_STONE, FLOOR_TINT),
                Tile::Wall => (LAYER_WHITE, WALL_TINT),
                Tile::Water => (LAYER_WHITE, WATER_TINT),
                Tile::Path => (LAYER_STONE, PATH_TINT),
                Tile::Void => unreachable!(),
            };

            // Wall tiles are taller for a pseudo-3D effect.
            let (render_h, y_offset) = if tile == Tile::Wall {
                (tile_h * 1.5, -tile_h * 0.5)
            } else {
                (tile_h, 0.0)
            };

            let row_depth = (tx + ty) as f32;
            let z_depth = row_depth;

            let instance = InstanceData {
                position: [sx, sy + y_offset],
                size: [tile_w, render_h],
                uv_rect: [0.0, 0.0, tile_w / TEX_ARRAY_W as f32, tile_h / TEX_ARRAY_H as f32],
                tint,
                texture_index,
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
    // Placeholder: ~0.6 tile wide, ~2.25 tiles tall.
    let monster_w = 40.0_f32;
    let monster_h = 72.0_f32;

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

        let offset_x = (TILE_WIDTH - monster_w) / 2.0;

        // z_depth: entities are foreground (drawn after tiles in ascending sort).
        // Higher tile_y = closer to camera = larger z = drawn later = in front.
        let tile_y_sum = mx + my;
        let z_depth = 10000.0 + tile_y_sum;

        let instance = InstanceData {
            position: [sx + offset_x, sy - monster_h],
            size: [monster_w, monster_h],
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
    // Placeholder: small ground item.
    let loot_w = 20.0_f32;
    let loot_h = 20.0_f32;

    for (lx, ly, _drops) in &world.pending_loot {
        let sx = (lx - ly) * (TILE_WIDTH / 2.0);
        let sy = (lx + ly) * (TILE_HEIGHT / 2.0);

        let screen_x = sx - cam_left;
        let screen_y = sy - cam_top;
        if screen_x + loot_w < 0.0
            || screen_x > screen_w
            || screen_y + loot_h < 0.0
            || screen_y > screen_h
        {
            continue;
        }

        let offset_x = (TILE_WIDTH - loot_w) / 2.0;
        // Loot sits on the ground: between tiles and standing entities.
        let tile_y_sum = lx + ly;
        let z_depth = 5000.0 + tile_y_sum;

        let instance = InstanceData {
            position: [sx + offset_x, sy - loot_h],
            size: [loot_w, loot_h],
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
    // Placeholder: ~0.75 tile wide, ~3 tiles tall.
    let player_w = 48.0_f32;
    let player_h = 96.0_f32;

    let sx = (px - py) * (TILE_WIDTH / 2.0);
    let sy = (px + py) * (TILE_HEIGHT / 2.0);

    let screen_x = sx - cam_left;
    let screen_y = sy - cam_top;
    if screen_x + player_w < 0.0
        || screen_x > screen_w
        || screen_y + player_h < 0.0
        || screen_y > screen_h
    {
        return;
    }

    let offset_x = (TILE_WIDTH - player_w) / 2.0;
    // Player is foreground, same layer as monsters.
    let tile_y_sum = px + py;
    let z_depth = 10000.0 + tile_y_sum;

    let instance = InstanceData {
        position: [sx + offset_x, sy - player_h],
        size: [player_w, player_h],
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

        // Layer 3: stone floor tile (procedural noise texture).
        let stone_data = generate_stone_texture();
        if let Err(e) = texture_array.add_layer(&gpu.queue, &stone_data) {
            tracing::error!("Failed to add stone layer to texture array: {e}");
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
        let bitmap_font = crate::bitmap_font::BitmapFont::new(&gpu.device, &gpu.queue, &legacy_pipeline);

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
            bitmap_font,
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
            // Skip movement when dead.
            if self.player_dead {
                if let Some(ref mut world) = self.world {
                    world.tick();
                }
                continue;
            }

            let mut dx = 0.0_f32;
            let mut dy = 0.0_f32;
            let speed = if self.gui.is_running() { RUN_SPEED } else { WALK_SPEED };

            // Screen-aligned movement in dimetric 2:1 isometric space.
            if self.moving(DIR_UP) {
                dx -= speed;
                dy -= speed;
            }
            if self.moving(DIR_DOWN) {
                dx += speed;
                dy += speed;
            }
            if self.moving(DIR_LEFT) {
                dx -= speed;
                dy += speed;
            }
            if self.moving(DIR_RIGHT) {
                dx += speed;
                dy -= speed;
            }

            // Click-to-move: walk towards target position.
            if dx == 0.0 && dy == 0.0 {
                if let Some((tx, ty)) = self.move_target {
                    let to_x = tx - px;
                    let to_y = ty - py;
                    let dist = (to_x * to_x + to_y * to_y).sqrt();
                    if dist > 0.15 {
                        dx = to_x / dist * speed;
                        dy = to_y / dist * speed;
                    } else {
                        self.move_target = None;
                    }
                }
            } else {
                // WASD cancels click-to-move.
                self.move_target = None;
            }

            // Tilemap collision: only move if destination is walkable.
            if dx != 0.0 || dy != 0.0 {
                let new_x = px + dx;
                let new_y = py + dy;
                let walkable = self
                    .tilemap
                    .as_ref()
                    .is_none_or(|t| t.is_walkable(new_x, new_y));
                if walkable {
                    px = new_x;
                    py = new_y;
                } else {
                    // Try sliding along axes individually.
                    let walk_x = self
                        .tilemap
                        .as_ref()
                        .is_none_or(|t| t.is_walkable(px + dx, py));
                    let walk_y = self
                        .tilemap
                        .as_ref()
                        .is_none_or(|t| t.is_walkable(px, py + dy));
                    if walk_x {
                        px += dx;
                    }
                    if walk_y {
                        py += dy;
                    }
                }
            }

            if let Some(ref mut world) = self.world {
                world.set_player_position(px, py);
                world.tick();

                // Enforce tilemap collision on monsters (prevent walking through walls).
                if let Some(ref tilemap) = self.tilemap {
                    let monster_ids: Vec<EntityId> =
                        world.ai_agents_keys().copied().collect();
                    for mid in monster_ids {
                        let Ok(mr) = world.ecs.get_component::<MonsterRecord>(mid) else {
                            continue;
                        };
                        let mx = mr.position.x();
                        let my = mr.position.y();
                        if !tilemap.is_walkable(mx, my) {
                            // Snap monster back to nearest walkable tile.
                            let best_x = mx.round();
                            let best_y = my.round();
                            if tilemap.is_walkable(best_x, best_y) {
                                let _ = world.ecs.modify_component::<MonsterRecord>(mid, |m| {
                                    m.position = mge_arpg_entity::Position::new(best_x, best_y);
                                });
                            }
                        }
                    }
                }
            }
        }

        // Auto-attack (hold left mouse button on monster).
        self.tick_auto_attack();

        // Auto-pickup loot when walking nearby.
        self.tick_auto_pickup();

        // Check for player death.
        self.check_player_death();

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
        // z_depth is used only for CPU-side painter's-algorithm sorting;
        // zero out the Z column so clip.z = 0 (always inside [0,1] NDC range).
        #[rustfmt::skip]
        let proj: [[f32; 4]; 4] = [
            [ 2.0 / ew,                                  0.0,   0.0, 0.0],
            [      0.0,                        -2.0 / eh,       0.0, 0.0],
            [      0.0,                                  0.0,   0.0, 0.0],
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
            self.tilemap.as_ref(),
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
                    size: [40.0, 72.0],
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

            // Populate monster health bar overlays for visible monsters.
            let mut overlays = Vec::with_capacity(visible_entity_ids.len());
            for &eid in &visible_entity_ids {
                if let Ok(mr) = world.ecs.get_component::<MonsterRecord>(eid) {
                    if mr.health.is_alive() {
                        let mx = mr.position.x();
                        let my = mr.position.y();
                        let sx = (mx - my) * (TILE_WIDTH / 2.0) - cam_left;
                        let sy = (mx + my) * (TILE_HEIGHT / 2.0) - cam_top;
                        overlays.push(gui::MonsterOverlay {
                            screen_x: sx,
                            screen_y: sy - 36.0, // Above sprite.
                            name: mr.name.clone(),
                            level: mr.level.get(),
                            hp_ratio: mr.health.ratio(),
                        });
                    }
                }
            }
            self.gui.set_monster_overlays(overlays);
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

        // --- Floating text background pass (white texture for coloured quads) ---
        res.legacy_batcher.begin();
        for ft in self.floating_texts.texts() {
            let sx = ft.world_pos[0] - cam_left;
            let sy = ft.world_pos[1] - cam_top - ft.y_offset;
            if sx < -200.0 || sx > screen_w + 200.0 || sy < -100.0 || sy > screen_h + 100.0 {
                continue;
            }
            let scale = ft.font_size / 8.0;
            let shake_x = if ft.has_shake {
                (ft.age * 30.0).sin() * 3.0 * ft.opacity
            } else {
                0.0
            };
            let char_w = crate::bitmap_font::BitmapFont::char_width(scale);
            let text_w = char_w * ft.text.len() as f32;
            let text_h = crate::bitmap_font::BitmapFont::line_height(scale);
            let pad = 4.0_f32;
            let uv_white: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

            // Critical: red background rectangle behind text (RO style).
            if ft.kind == mge_render::FloatingTextKind::Critical {
                let bg_col = [0.85, 0.05, 0.05, 0.85 * ft.opacity];
                res.legacy_batcher.push(
                    sx + shake_x - pad,
                    sy - pad,
                    text_w + pad * 2.0,
                    text_h + pad * 2.0,
                    uv_white,
                    bg_col,
                );
            }

            // Dark outline/shadow behind all floating texts for contrast.
            let shadow_col = [0.0, 0.0, 0.0, 0.6 * ft.opacity];
            let outline = 2.0_f32;
            res.legacy_batcher.push(
                sx + shake_x - outline,
                sy - outline,
                text_w + outline * 2.0,
                text_h + outline * 2.0,
                uv_white,
                shadow_col,
            );
        }
        let float_bg_count = res.legacy_batcher.flush(&gpu.queue);
        if float_bg_count > 0 {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("float_bg_pass"),
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
            res.legacy_batcher.draw(&mut pass, float_bg_count);
        }

        // --- Text pass (bitmap font) ---
        res.legacy_batcher.begin();
        self.gui.draw_all_text(&mut res.legacy_batcher);

        // Floating combat texts (world-space → screen-space).
        for ft in self.floating_texts.texts() {
            let sx = ft.world_pos[0] - cam_left;
            let sy = ft.world_pos[1] - cam_top - ft.y_offset;

            // Skip if off-screen.
            if sx < -200.0 || sx > screen_w + 200.0 || sy < -100.0 || sy > screen_h + 100.0 {
                continue;
            }

            let scale = ft.font_size / 8.0;
            let color = [ft.color[0], ft.color[1], ft.color[2], ft.opacity];

            // Horizontal shake for critical hits.
            let shake_x = if ft.has_shake {
                (ft.age * 30.0).sin() * 3.0 * ft.opacity
            } else {
                0.0
            };

            crate::bitmap_font::BitmapFont::push_text(
                &mut res.legacy_batcher,
                sx + shake_x,
                sy,
                &ft.text,
                color,
                scale,
            );
        }

        // Loot labels on ground (Alt key held, D2 style).
        if self.show_loot_labels {
            if let Some(ref world) = self.world {
                let label_scale = 1.5_f32;
                for (lx, ly, drops) in &world.pending_loot {
                    let sx = (*lx - *ly) * (TILE_WIDTH / 2.0) - cam_left;
                    let sy = (*lx + *ly) * (TILE_HEIGHT / 2.0) - cam_top - 20.0;
                    if sx < -200.0 || sx > screen_w + 200.0 || sy < -50.0 || sy > screen_h {
                        continue;
                    }
                    for (di, drop) in drops.iter().enumerate() {
                        let label = if drop.item_id == "gold" {
                            format!("{} Gold", drop.quantity)
                        } else {
                            drop.item_id.replace('_', " ")
                        };
                        let label_y = sy - di as f32 * 14.0;
                        // Gold = yellow, items = D2 quality color.
                        let color = if drop.item_id == "gold" {
                            [1.0, 0.85, 0.2, 1.0]
                        } else {
                            quality_color(drop.quality)
                        };
                        // Shadow.
                        crate::bitmap_font::BitmapFont::push_text(
                            &mut res.legacy_batcher,
                            sx + 1.0,
                            label_y + 1.0,
                            &label,
                            [0.0, 0.0, 0.0, 0.7],
                            label_scale,
                        );
                        // Text.
                        crate::bitmap_font::BitmapFont::push_text(
                            &mut res.legacy_batcher,
                            sx,
                            label_y,
                            &label,
                            color,
                            label_scale,
                        );
                    }
                }
            }
        }

        // Death overlay text.
        if self.player_dead {
            let death_scale = 4.0;
            let death_text = "YOU DIED";
            let text_w = death_text.len() as f32 * 8.0 * death_scale;
            let cx = screen_w / 2.0 - text_w / 2.0;
            let cy = screen_h / 3.0;
            // Dark shadow.
            crate::bitmap_font::BitmapFont::push_text(
                &mut res.legacy_batcher,
                cx + 2.0,
                cy + 2.0,
                death_text,
                [0.0, 0.0, 0.0, 0.8],
                death_scale,
            );
            // Red text.
            crate::bitmap_font::BitmapFont::push_text(
                &mut res.legacy_batcher,
                cx,
                cy,
                death_text,
                [0.9, 0.1, 0.1, 1.0],
                death_scale,
            );
            // Subtitle.
            let sub_text = "Press SPACE to respawn";
            let sub_scale = 2.0;
            let sub_w = sub_text.len() as f32 * 8.0 * sub_scale;
            crate::bitmap_font::BitmapFont::push_text(
                &mut res.legacy_batcher,
                screen_w / 2.0 - sub_w / 2.0,
                cy + 40.0,
                sub_text,
                [0.8, 0.7, 0.5, 1.0],
                sub_scale,
            );
        }

        let text_vert_count = res.legacy_batcher.flush(&gpu.queue);

        if text_vert_count > 0 {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("text_pass"),
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
            pass.set_bind_group(1, &res.bitmap_font.texture.bind_group, &[]);
            res.legacy_batcher.draw(&mut pass, text_vert_count);
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
                KeyCode::Space => {
                    if self.player_dead {
                        self.respawn_player();
                    }
                }
                KeyCode::Alt => self.show_loot_labels = true,
                _ => {}
            },
            InputEvent::KeyUp { key } => match key {
                KeyCode::W | KeyCode::Up => self.release_dir(DIR_UP),
                KeyCode::S | KeyCode::Down => self.release_dir(DIR_DOWN),
                KeyCode::A | KeyCode::Left => self.release_dir(DIR_LEFT),
                KeyCode::D | KeyCode::Right => self.release_dir(DIR_RIGHT),
                KeyCode::Alt => self.show_loot_labels = false,
                _ => {}
            },
            InputEvent::MouseMove { x, y } => {
                self.mouse_screen = (x as f32, y as f32);
            }
            InputEvent::MouseButtonEvent {
                button: MouseButton::Left,
                pressed,
            } => {
                self.left_mouse_held = pressed;
                if pressed {
                    // Left click: attack/loot/move via handle_world_click.
                    let (mx, my) = self.mouse_screen;
                    self.handle_world_click(mx, my);
                }
            }
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
    fn test_belt_potion_keys() {
        let mut app = SodomightApp::new();
        app.on_input(InputEvent::KeyDown { key: KeyCode::Num1 });
        // Belt potion use without world is a no-op; verify no panic.
    }

    #[test]
    fn test_skill_bar_keys_f1() {
        let mut app = SodomightApp::new();
        app.on_input(InputEvent::KeyDown { key: KeyCode::F1 });
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
        batch_tiles_instanced(&mut batcher, None, 0.0, 0.0, 1280.0, 720.0);
        // Should have pushed some tiles (visible portion of 64x64 map).
        assert!(batcher.len() > 0);
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
