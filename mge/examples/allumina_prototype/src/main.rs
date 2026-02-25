//! @id allumina.prototype.main
//! @role entry
//! @layer application
//! @domain allumina
//! @do bootstrap_engine_and_run_game_loop
//!
//! Allumina Prototype — v2.1 : A*, FSM monstres D2, stats/combat, barres HP.
//!
//! Fenêtre wgpu 1280×720.
//! Joueur + 30 mobs scattérisés. Mobs : errance Idle → Chase → Attack.
//! Clic gauche = déplacement A* joueur. G=grille, F3=stats, molette=zoom.

mod ai;
mod animation;
pub mod biomes;
mod collision;
mod combat;
mod components;
mod constants;
mod dev_overlay;
mod gui;
mod input_handler;
mod isometric;
mod movement;
mod pathfinding;
mod plugin;
mod renderer;
mod stats;
mod tilemap;

use std::cmp::Ordering;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use mge_core::{Engine, EngineConfig};
use mge_plugin_spatial::Position2D;
use winit::event::{Event, MouseButton, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use ai::MonsterAI;
use animation::AnimSprite;
use components::{AlluminaInput, AlluminaMap, DecoMarker, EntitySprite, NpcMarker, PlayerMarker};
use constants::{
    MAP_SIZE, SAFE_ZONE_RADIUS, SAFE_ZONE_X, SAFE_ZONE_Y,
    in_camp_zone, in_safe_zone, is_camp_interior, is_camp_water,
};
use dev_overlay::DevState;
use gui::{GuiState, HudData};
use isometric::{IsoCamera, ScreenPos};
use plugin::AlluminaPlugin;
use renderer::WgpuRenderer;
use stats::{CharacterStats, CombatStats, Competences, PlayerAttack, PlayerGameStats, XpReward};
use tilemap::TileMap;

/// Texte de dégât flottant (ARPG style)
struct FloatingText {
    text: String,
    wx: f32,
    wy: f32,
    timer: f32,
    lifetime: f32,
}

/// État des touches de déplacement tenues
#[derive(Default)]
struct HeldKeys {
    up: bool,    // Z / ArrowUp    → NW isométrique (-1, -1)
    down: bool,  // S / ArrowDown  → SE isométrique (+1, +1)
    left: bool,  // Q / ArrowLeft  → SW isométrique (-1, +1)
    right: bool, // D / ArrowRight → NE isométrique (+1, -1)
}

impl HeldKeys {
    /// Direction monde isométrique normalisée depuis les touches tenues.
    ///
    /// Projection isométrique standard :
    ///   Z/↑  = écran-haut   = monde (-1, -1) // NW
    ///   S/↓  = écran-bas    = monde (+1, +1) // SE
    ///   Q/←  = écran-gauche = monde (-1, +1) // SW
    ///   D/→  = écran-droite = monde (+1, -1) // NE
    fn to_iso_dir(&self) -> (f32, f32) {
        let mut wx = 0.0f32;
        let mut wy = 0.0f32;
        if self.up    { wx -= 1.0; wy -= 1.0; }
        if self.down  { wx += 1.0; wy += 1.0; }
        if self.left  { wx -= 1.0; wy += 1.0; }
        if self.right { wx += 1.0; wy -= 1.0; }
        let len = (wx * wx + wy * wy).sqrt();
        if len > 0.001 { (wx / len, wy / len) } else { (0.0, 0.0) }
    }
}

/// Rayon de vision pour le culling sprites (tiles)
const SPRITE_CULL_RADIUS: f32 = 18.0;
/// Nombre de mobs à spawner
const MOB_COUNT: usize = 30;
/// Délai de réapparition des mobs après mort (secondes)
const RESPAWN_DELAY: f32 = 30.0;

/// Entrée dans la file de réapparition des mobs.
struct RespawnEntry {
    x: f32,
    y: f32,
    kind: u8,
    seed: u64,
    timer: f32,
}

fn main() {
    env_logger::init();
    pollster::block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new().expect("event loop");
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Allumina — ZQSD/↑↓←→=dépl  Clic-G=sélect  Clic-D=rallier  G=grille  F3=stats  P=Perso  C=Compét  I=Inventaire")
            .with_inner_size(winit::dpi::LogicalSize::new(1280u32, 720u32))
            .build(&event_loop)
            .expect("fenêtre winit"),
    );

    let mut size = window.inner_size();
    if size.width == 0 { size.width = 1280; }
    if size.height == 0 { size.height = 720; }

    // ── wgpu setup ────────────────────────────────────────────────────────────
    let instance = wgpu::Instance::default();
    let surface = instance.create_surface(window.clone()).expect("surface wgpu");
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .expect("adapter wgpu");
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("allumina_device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
            },
            None,
        )
        .await
        .expect("device wgpu");

    // ── Chemins assets ────────────────────────────────────────────────────────
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dev_assets = manifest_dir
        .join("../../assets/Dev_assets")
        .canonicalize()
        .unwrap_or_else(|_| manifest_dir.join("../../assets/Dev_assets"));

    // ── Renderer ──────────────────────────────────────────────────────────────
    let mut renderer = WgpuRenderer::new(
        device, queue, surface, &adapter,
        (size.width, size.height), &dev_assets,
    )
    .await
    .expect("renderer wgpu");

    // ── Tilemap procédurale herbe variée ─────────────────────────────────────
    let tilemap = make_grass_map(MAP_SIZE, MAP_SIZE);

    // ── ECS Engine ────────────────────────────────────────────────────────────
    let mut engine = Engine::new(EngineConfig {
        seed: 42,
        headless: false,
        fixed_timestep_ms: Some(16),
        tick_budget_ms: Some(14),
    });
    engine.add_plugin(AlluminaPlugin);
    engine.build();

    // Tilemap (singleton)
    let map_entity = engine.world_mut().spawn();
    engine.world_mut().insert(map_entity, AlluminaMap::new(tilemap.clone()));

    // Input (singleton)
    let input_entity = engine.world_mut().spawn();
    engine.world_mut().insert(input_entity, AlluminaInput::default());

    // ── Joueur dans la safe zone (centre carte) ───────────────────────────────
    let cx = SAFE_ZONE_X;
    let cy = SAFE_ZONE_Y;
    let player_cs = CharacterStats::player();
    let player_combat = CombatStats::from_char_stats(&player_cs);
    let player_entity = engine.world_mut().spawn();
    engine.world_mut().insert(player_entity, Position2D { x: cx, y: cy });
    engine.world_mut().insert(player_entity, PlayerMarker);
    engine.world_mut().insert(player_entity, EntitySprite::player());
    engine.world_mut().insert(player_entity, player_cs);
    engine.world_mut().insert(player_entity, player_combat);
    engine.world_mut().insert(player_entity, PlayerAttack::new());
    engine.world_mut().insert(player_entity, PlayerGameStats::new());
    engine.world_mut().insert(player_entity, Competences::from_char_stats(&player_cs));
    engine.world_mut().insert(player_entity, AnimSprite::new(10.0));

    // ── NPCs du camp (non-combattants) ──────────────────────────────────────
    // Positions à l'intérieur du camp (~20×16 centré à 48,48)
    let npc_defs: &[(&str, u8, f32, f32)] = &[
        ("Charsi",  0, 42.0, 44.0), // forgeron — coin NW du camp
        ("Kashya",  1, 54.0, 44.0), // recruteur — côté NE
        ("Gheed",   2, 44.0, 52.0), // serveuse — côté SW
        ("Akara",   3, 52.0, 50.0), // tavernier — centre-sud
    ];
    for &(name, kind, nx, ny) in npc_defs {
        let npc = engine.world_mut().spawn();
        engine.world_mut().insert(npc, Position2D { x: nx, y: ny });
        engine.world_mut().insert(npc, EntitySprite::npc(kind));
        engine.world_mut().insert(npc, NpcMarker { name });
    }

    // ── Décorations du camp (maison, arbres) ────────────────────────────────
    // Maison au centre-nord du camp
    {
        let house = engine.world_mut().spawn();
        engine.world_mut().insert(house, Position2D { x: 48.0, y: 43.0 });
        engine.world_mut().insert(house, EntitySprite::house());
        engine.world_mut().insert(house, DecoMarker);
    }
    // Arbres dispersés dans/autour du camp
    let tree_defs: &[(u8, f32, f32)] = &[
        (0, 40.0, 42.0), // arbre_a NW
        (1, 56.0, 42.0), // arbre_b NE
        (0, 40.0, 54.0), // arbre_a SW
        (1, 55.0, 53.0), // arbre_b SE (avant le passage)
    ];
    for &(kind, tx, ty) in tree_defs {
        let tree = engine.world_mut().spawn();
        engine.world_mut().insert(tree, Position2D { x: tx, y: ty });
        engine.world_mut().insert(tree, EntitySprite::tree(kind));
        engine.world_mut().insert(tree, DecoMarker);
    }

    // ── Mobs (scatter déterministe, hors safe zone) ───────────────────────────
    let mut rng = mge_rng::Rng::new(12345);
    let map_range = MAP_SIZE - 4;
    let mut spawned = 0;
    let mut attempts = 0u32;
    while spawned < MOB_COUNT && attempts < 2000 {
        attempts += 1;
        let mx = 2.0 + (rng.u32() % map_range) as f32 + 0.5;
        let my = 2.0 + (rng.u32() % map_range) as f32 + 0.5;
        // Ne pas spawner dans la safe zone + marge de 5 tiles
        if in_safe_zone(mx, my) {
            let dx = mx - SAFE_ZONE_X;
            let dy = my - SAFE_ZONE_Y;
            if (dx * dx + dy * dy).sqrt() < SAFE_ZONE_RADIUS + 5.0 {
                continue;
            }
        }
        // Ne pas spawner dans le camp ni sur sa bordure d'eau
        if in_camp_zone(mx, my) {
            continue;
        }
        let kind = (spawned % 5) as u8;
        let seed = 0xDEAD_0000u64 + spawned as u64;
        let mob_cs = CharacterStats::mob(kind);
        let mob_combat = CombatStats::from_char_stats(&mob_cs);
        // XP = (For + Con) × 5, Or = 1 + kind × 2
        let xp_reward = (mob_cs.for_ as u32 + mob_cs.con as u32) * 5;
        let gold_reward = 1u32 + kind as u32 * 2;
        let mob = engine.world_mut().spawn();
        engine.world_mut().insert(mob, Position2D { x: mx, y: my });
        engine.world_mut().insert(mob, EntitySprite::mob(kind));
        engine.world_mut().insert(mob, mob_cs);
        engine.world_mut().insert(mob, mob_combat);
        engine.world_mut().insert(mob, MonsterAI::new(mx, my, seed));
        engine.world_mut().insert(mob, XpReward { xp: xp_reward, gold: gold_reward });
        spawned += 1;
    }

    // ── Caméra ────────────────────────────────────────────────────────────────
    let mut camera = IsoCamera::new(cx, cy, 2.0);

    // ── Boucle événements ─────────────────────────────────────────────────────
    let mut last_time = Instant::now();
    let mut last_cursor_pos: Option<(f64, f64)> = None;
    let mut dev_state = DevState::new_with_stats();
    let mut held_keys = HeldKeys::default();
    // Bits de l'EntityId sélectionné (clic gauche), None si aucune sélection
    let mut selected_entity: Option<u64> = None;
    // Textes de dégâts flottants
    let mut floating_texts: Vec<FloatingText> = Vec::new();
    // État GUI (fenêtres, HUD)
    let mut gui_state = GuiState::default();
    // File de réapparition des mobs
    let mut respawn_queue: Vec<RespawnEntry> = Vec::new();

    event_loop
        .run(move |event, target| {
            if let Event::AboutToWait = event {
                let now = Instant::now();
                let dt = (now - last_time).as_secs_f32().min(0.05);
                last_time = now;

                // ── Injecter direction clavier dans AlluminaInput ─────────────
                let dir = held_keys.to_iso_dir();
                if let Some(inp) = engine.world_mut().get_mut::<AlluminaInput>(input_entity) {
                    inp.move_dir = dir;
                }

                // ── Snapshot HP avant simulation (pour dégâts flottants) ─────
                let pre_tick_hp: Vec<(u64, f32, f32, i32)> = engine.world()
                    .iter2::<mge_plugin_spatial::Position2D, CombatStats>()
                    .map(|(id, pos, s)| (id.to_bits(), pos.x, pos.y, s.hp))
                    .collect();

                // ── Simulation ────────────────────────────────────────────────
                engine.tick(dt);

                // ── Générer textes flottants depuis delta HP ──────────────────
                {
                    let post_hp: std::collections::HashMap<u64, i32> = engine.world()
                        .iter1::<CombatStats>()
                        .map(|(id, s)| (id.to_bits(), s.hp))
                        .collect();
                    for &(id_bits, wx, wy, old_hp) in &pre_tick_hp {
                        let new_hp = post_hp.get(&id_bits).copied().unwrap_or(old_hp);
                        let damage = old_hp - new_hp;
                        if damage > 0 {
                            floating_texts.push(FloatingText {
                                text: format!("-{}", damage),
                                wx,
                                wy,
                                timer: 0.0,
                                lifetime: 1.2,
                            });
                        }
                    }
                }

                // ── Nettoyage des morts + récompenses + respawn ───────────────
                let dead_ids: Vec<_> = engine.world()
                    .iter1::<stats::Dead>()
                    .map(|(id, _)| id)
                    .collect();

                let mut gained_xp = 0u32;
                let mut gained_gold = 0u32;

                for id in dead_ids {
                    // Ne pas supprimer le joueur (immortel dans le prototype)
                    if !engine.world().has_component::<PlayerMarker>(id) {
                        // Désélectionner si c'était l'entité sélectionnée
                        if selected_entity == Some(id.to_bits()) {
                            selected_entity = None;
                        }
                        // Collecter récompenses
                        if let Some(rew) = engine.world().get::<XpReward>(id) {
                            gained_xp += rew.xp;
                            gained_gold += rew.gold;
                        }
                        // Enregistrer pour respawn
                        let ai_patrol = engine.world()
                            .get::<MonsterAI>(id)
                            .map(|ai| (ai.patrol_x, ai.patrol_y, ai.rng_seed));
                        let kind = engine.world()
                            .get::<components::EntitySprite>(id)
                            .map(|s| s.sprite_id % 5)
                            .unwrap_or(0);
                        if let Some((px, py, seed)) = ai_patrol {
                            respawn_queue.push(RespawnEntry {
                                x: px,
                                y: py,
                                kind,
                                seed: seed.wrapping_add(0x1337),
                                timer: RESPAWN_DELAY,
                            });
                        }
                        engine.world_mut().despawn(id);
                    }
                }

                // Appliquer XP et or au joueur
                if gained_xp > 0 || gained_gold > 0 {
                    let player_id = engine.world()
                        .iter1::<PlayerMarker>()
                        .next()
                        .map(|(id, _)| id);
                    if let Some(pid) = player_id {
                        // Texte flottant XP/Or
                        let (pw, ph) = engine.world()
                            .get::<mge_plugin_spatial::Position2D>(pid)
                            .map(|p| (p.x, p.y))
                            .unwrap_or((0.0, 0.0));
                        if gained_xp > 0 {
                            floating_texts.push(FloatingText {
                                text: format!("+{}xp", gained_xp),
                                wx: pw + 0.3,
                                wy: ph,
                                timer: 0.0,
                                lifetime: 1.5,
                            });
                        }
                        if gained_gold > 0 {
                            floating_texts.push(FloatingText {
                                text: format!("+{}g", gained_gold),
                                wx: pw - 0.3,
                                wy: ph,
                                timer: 0.0,
                                lifetime: 1.5,
                            });
                        }
                        if let Some(gs) = engine.world_mut().get_mut::<PlayerGameStats>(pid) {
                            gs.xp += gained_xp;
                            gs.gold += gained_gold;
                        }
                        // Level-up loop (mut borrows séquentiels)
                        loop {
                            let (xp, xp_to_next) = engine.world()
                                .get::<PlayerGameStats>(pid)
                                .map(|gs| (gs.xp, gs.xp_to_next))
                                .unwrap_or((0, 1));
                            if xp < xp_to_next { break; }
                            if let Some(gs) = engine.world_mut().get_mut::<PlayerGameStats>(pid) {
                                gs.xp -= gs.xp_to_next;
                                gs.xp_to_next = (gs.xp_to_next * 3) / 2; // ×1.5 par niveau
                            }
                            if let Some(cs) = engine.world_mut().get_mut::<CombatStats>(pid) {
                                cs.level = cs.level.saturating_add(1);
                                cs.hp_max += 10;
                                cs.end_max += 5;
                                cs.hp = cs.hp_max;
                                cs.end = cs.end_max;
                            }
                            if let Some(gs) = engine.world_mut().get_mut::<PlayerGameStats>(pid) {
                                gs.mana = gs.mana_max;
                            }
                            let lvl = engine.world()
                                .get::<CombatStats>(pid)
                                .map(|cs| cs.level)
                                .unwrap_or(1);
                            let (lw, lh) = engine.world()
                                .get::<mge_plugin_spatial::Position2D>(pid)
                                .map(|p| (p.x, p.y))
                                .unwrap_or((0.0, 0.0));
                            floating_texts.push(FloatingText {
                                text: format!("NIVEAU {}!", lvl),
                                wx: lw,
                                wy: lh - 0.5,
                                timer: 0.0,
                                lifetime: 2.5,
                            });
                        }
                    }
                }

                // ── Tick respawn queue ────────────────────────────────────────
                {
                    let mut i = 0;
                    while i < respawn_queue.len() {
                        respawn_queue[i].timer -= dt;
                        if respawn_queue[i].timer <= 0.0 {
                            let entry = respawn_queue.swap_remove(i);
                            let mob_cs = CharacterStats::mob(entry.kind);
                            let mob_combat = CombatStats::from_char_stats(&mob_cs);
                            let xp_reward = (mob_cs.for_ as u32 + mob_cs.con as u32) * 5;
                            let gold_reward = 1u32 + entry.kind as u32 * 2;
                            let mob = engine.world_mut().spawn();
                            engine.world_mut().insert(mob, mge_plugin_spatial::Position2D { x: entry.x, y: entry.y });
                            engine.world_mut().insert(mob, components::EntitySprite::mob(entry.kind));
                            engine.world_mut().insert(mob, mob_cs);
                            engine.world_mut().insert(mob, mob_combat);
                            engine.world_mut().insert(mob, MonsterAI::new(entry.x, entry.y, entry.seed));
                            engine.world_mut().insert(mob, XpReward { xp: xp_reward, gold: gold_reward });
                        } else {
                            i += 1;
                        }
                    }
                }

                // ── Mise à jour et nettoyage des textes flottants ────────────
                for ft in &mut floating_texts {
                    ft.timer += dt;
                }
                floating_texts.retain(|ft| ft.timer < ft.lifetime);

                // ── Caméra suit le joueur ─────────────────────────────────────
                if let Some((_, _, pos)) = engine.world().iter2::<PlayerMarker, Position2D>().next() {
                    camera.center_x = pos.x;
                    camera.center_y = pos.y;
                }

                // ── Collecte entités pour le rendu ────────────────────────────
                // (wx, wy, sprite_id, hp_ratio, is_selected, sort_key)
                let mut entity_sprites: Vec<(f32, f32, u8, f32, bool, u64)> = Vec::new();
                for (eid, pos, sprite) in engine.world().iter2::<Position2D, EntitySprite>() {
                    let dx = pos.x - camera.center_x;
                    let dy = pos.y - camera.center_y;
                    if dx * dx + dy * dy > SPRITE_CULL_RADIUS * SPRITE_CULL_RADIUS {
                        continue;
                    }
                    let hp_ratio = engine.world()
                        .get::<CombatStats>(eid)
                        .map(|s| s.hp_ratio())
                        .unwrap_or(-1.0);
                    let is_selected = selected_entity == Some(eid.to_bits());
                    entity_sprites.push((pos.x, pos.y, sprite.sprite_id, hp_ratio, is_selected, eid.to_bits()));
                }

                // Y-sort déterministe : (wx+wy, wx, entity_id)
                entity_sprites.sort_by(|a, b| {
                    (a.0 + a.1)
                        .partial_cmp(&(b.0 + b.1))
                        .unwrap_or(Ordering::Equal)
                        .then_with(|| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal))
                        .then_with(|| a.5.cmp(&b.5))
                });

                // ── Stats overlay ─────────────────────────────────────────────
                let n_entities = engine.world().entity_count();
                let n_sprites = entity_sprites.len();
                let player_pos = engine.world()
                    .iter2::<PlayerMarker, Position2D>()
                    .next()
                    .map(|(_, _, pos)| (pos.x, pos.y));
                // Stats HP joueur pour l'overlay texte
                let player_hp_text = engine.world()
                    .iter2::<PlayerMarker, CombatStats>()
                    .next()
                    .map(|(_, _, s)| format!("  PV:{}/{} End:{}", s.hp, s.hp_max, s.end))
                    .unwrap_or_default();
                dev_state.update(dt, n_entities as usize, 0, n_sprites, player_pos);
                // Ajouter HP joueur au texte — seulement lors du rebuild (tous les 30 frames)
                if dev_state.frame_accumulator % 30 == 0 && !player_hp_text.is_empty() {
                    dev_state.stats_text.push_str(&player_hp_text);
                }

                // ── Données HUD joueur ────────────────────────────────────────
                let hud_data = {
                    let mut hd = HudData {
                        hp: 0, hp_max: 1, mana: 0, mana_max: 1,
                        end: 0, end_max: 1,
                        xp: 0, xp_to_next: 100, level: 1,
                        for_: 3, con: 3, agi: 3, dex: 3, per: 3,
                        vol: 3, int: 3, sag: 3, cha: 3, luk: 3,
                        atk_apt: 30, atk_speed_apt: 30, jet_apt: 30,
                        esq_apt: 30, par_apt: 30,
                        tir_corde_apt: 30, tir_poing_apt: 30, tir_epaule_apt: 30,
                        magie_apt: 3, cast_speed_apt: 3,
                        base_dmg: 9,
                        competences: Vec::new(),
                        gold: 0,
                        in_safe_zone: false,
                    };
                    if let Some((_, _, cs)) = engine.world().iter2::<PlayerMarker, CombatStats>().next() {
                        hd.hp = cs.hp; hd.hp_max = cs.hp_max;
                        hd.end = cs.end; hd.end_max = cs.end_max;
                        hd.level = cs.level;
                    }
                    if let Some((pid, _, cc)) = engine.world().iter2::<PlayerMarker, CharacterStats>().next() {
                        let cc = *cc; // Copy to release borrow
                        let pid = pid; // Copy EntityId
                        hd.for_ = cc.for_; hd.con = cc.con; hd.agi = cc.agi; hd.dex = cc.dex;
                        hd.per = cc.per; hd.vol = cc.vol; hd.int = cc.int; hd.sag = cc.sag;
                        hd.cha = cc.cha; hd.luk = cc.luk;
                        hd.atk_apt = cc.atk_apt();
                        hd.atk_speed_apt = cc.atk_speed_apt();
                        hd.jet_apt = cc.jet_apt();
                        hd.esq_apt = cc.esq_apt();
                        hd.par_apt = cc.par_apt();
                        hd.tir_corde_apt = cc.tir_corde_apt();
                        hd.tir_poing_apt = cc.tir_poing_apt();
                        hd.tir_epaule_apt = cc.tir_epaule_apt();
                        hd.magie_apt = cc.magie_apt();
                        hd.cast_speed_apt = cc.cast_speed_apt();
                        hd.base_dmg = cc.melee_damage();
                        hd.hp_max = cc.pv_max(); hd.end_max = cc.end_max();
                        hd.mana_max = cc.pm_max();
                        // Compétences : construire la liste (catégorie, nom, valeur, cap)
                        if let Some(comp) = engine.world().get::<Competences>(pid) {
                            hd.competences = comp.as_list(&cc).iter()
                                .map(|&(cat, name, val, carac)| (cat, name, val, Competences::cap(carac)))
                                .collect();
                        }
                    }
                    if let Some((_, _, gs)) = engine.world().iter2::<PlayerMarker, PlayerGameStats>().next() {
                        hd.mana = gs.mana; hd.mana_max = gs.mana_max;
                        hd.xp = gs.xp; hd.xp_to_next = gs.xp_to_next;
                        hd.gold = gs.gold;
                    }
                    if let Some((_, _, pos)) = engine.world().iter2::<PlayerMarker, Position2D>().next() {
                        hd.in_safe_zone = in_safe_zone(pos.x, pos.y);
                    }
                    hd
                };

                // ── Rendu ─────────────────────────────────────────────────────
                let tilemap_ref = engine
                    .world()
                    .iter1::<AlluminaMap>()
                    .next()
                    .map(|(_, m)| std::sync::Arc::clone(&m.tilemap));

                if let Some(ref tm) = tilemap_ref {
                    let stats_str = dev_state.stats_text.clone();
                    let stats_opt = if dev_state.show_stats {
                        Some(stats_str.as_str())
                    } else {
                        None
                    };
                    // Projeter les textes flottants en coordonnées écran
                    let ft_projected: Vec<(f32, f32, String)> = floating_texts.iter()
                        .map(|ft| {
                            let rise = (ft.timer / ft.lifetime) * 40.0;
                            let (sx, sy) = camera.project(
                                ft.wx, ft.wy,
                                size.width as f32, size.height as f32,
                            );
                            (sx, sy - rise, ft.text.clone())
                        })
                        .collect();
                    let ft_refs: Vec<(f32, f32, &str)> = ft_projected.iter()
                        .map(|(x, y, t)| (*x, *y, t.as_str()))
                        .collect();
                    match renderer.draw_frame(
                        tm,
                        &camera,
                        entity_sprites.iter().map(|&(x, y, sid, hp, sel, _)| (x, y, sid, hp, sel)),
                        Some(&dev_state),
                        stats_opt,
                        &ft_refs,
                        &gui_state,
                        &hud_data,
                    ) {
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            renderer.resize(size.width, size.height);
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => target.exit(),
                        _ => {}
                    }
                }
                window.request_redraw();
            }

            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested => target.exit(),

                    WindowEvent::Resized(new_size) => {
                        size = new_size;
                        renderer.resize(size.width, size.height);
                    }

                    WindowEvent::KeyboardInput { event, .. } => {
                        use winit::event::ElementState;
                        use winit::keyboard::{Key, NamedKey};
                        let pressed = event.state == ElementState::Pressed;
                        match &event.logical_key {
                            // Actions (pression uniquement)
                            Key::Named(NamedKey::Escape) if pressed => target.exit(),
                            Key::Named(NamedKey::F3) if pressed => dev_state.toggle_stats(),
                            // Flèches directionnelles (suivi appui/relâche)
                            Key::Named(NamedKey::ArrowUp)    => held_keys.up    = pressed,
                            Key::Named(NamedKey::ArrowDown)  => held_keys.down  = pressed,
                            Key::Named(NamedKey::ArrowLeft)  => held_keys.left  = pressed,
                            Key::Named(NamedKey::ArrowRight) => held_keys.right = pressed,
                            // ZQSD (AZERTY) — suivi appui/relâche
                            Key::Character(s) => match s.as_str() {
                                "z" | "Z" => held_keys.up    = pressed,
                                "s" | "S" => held_keys.down  = pressed,
                                "q" | "Q" => held_keys.left  = pressed,
                                "d" | "D" => held_keys.right = pressed,
                                "g" | "G" if pressed => dev_state.toggle_grid(),
                                "p" | "P" if pressed => gui_state.toggle_character(),
                                "c" | "C" if pressed => gui_state.toggle_skills(),
                                "i" | "I" if pressed => gui_state.toggle_inventory(),
                                _ => {}
                            },
                            _ => {}
                        }
                    }

                    WindowEvent::MouseWheel { delta, .. } => {
                        let scroll = match delta {
                            winit::event::MouseScrollDelta::LineDelta(_, y) => y,
                            winit::event::MouseScrollDelta::PixelDelta(p) => p.y as f32 / 60.0,
                        };
                        let new_zoom = (camera.zoom + scroll * 0.1).clamp(0.5, 4.0);
                        camera.set_zoom_clamped(new_zoom);
                    }

                    WindowEvent::MouseInput { button, state, .. } => {
                        use winit::event::ElementState;
                        if button == MouseButton::Left && state == ElementState::Released {
                            gui_state.handle_mouse_release();
                        }
                        if state == ElementState::Pressed {
                            if let Some((px, py)) = last_cursor_pos {
                                let screen = ScreenPos::new(px as f32, py as f32);
                                let world_pos = camera.screen_to_world(
                                    screen,
                                    size.width as f32,
                                    size.height as f32,
                                );

                                // Clic gauche : tester barres de titre (drag) et boutons GUI d'abord
                                if button == MouseButton::Left && gui_state.handle_mouse_press(px as f32, py as f32) {
                                    // Clic consommé par la GUI
                                } else if button == MouseButton::Left {
                                    let mut best_id: Option<u64> = None;
                                    let mut best_d_sq = 1.5f32 * 1.5f32;
                                    for (eid, pos, _) in engine.world().iter2::<Position2D, EntitySprite>() {
                                        let dx = pos.x - world_pos.x;
                                        let dy = pos.y - world_pos.y;
                                        let d_sq = dx * dx + dy * dy;
                                        if d_sq < best_d_sq {
                                            best_d_sq = d_sq;
                                            best_id = Some(eid.to_bits());
                                        }
                                    }
                                    selected_entity = best_id;
                                }

                                // Clic droit : balise de ralliement A* pour le joueur
                                if button == MouseButton::Right {
                                    if let Some(inp) =
                                        engine.world_mut().get_mut::<AlluminaInput>(input_entity)
                                    {
                                        inp.pending_move_to = Some((world_pos.x, world_pos.y));
                                    }
                                }
                            }
                        }
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        last_cursor_pos = Some((position.x, position.y));
                        gui_state.handle_mouse_move(position.x as f32, position.y as f32);
                    }

                    _ => {}
                }
            }
        })
        .expect("event loop run");
}

/// Génère une tilemap procédurale herbe variée (graphic_id 0–7)
/// avec le camp de départ D2 (bordure d'eau infranchissable, passage SE).
fn make_grass_map(width: u32, height: u32) -> TileMap {
    use tilemap::{Tile, TileFlags};
    let mut map = TileMap::new(width, height);
    let mut rng = mge_rng::Rng::new(99999);
    for ty in 0..height {
        for tx in 0..width {
            if is_camp_water(tx, ty) {
                // Bordure d'eau du camp : graphique water, non-marchable
                let water_gid = 96 + (rng.u32() % 2) as u16; // 96 ou 97
                map.set(tx, ty, Tile::new(water_gid, TileFlags(TileFlags::WATER)));
            } else if is_camp_interior(tx, ty) {
                // Intérieur du camp : herbe spéciale (on utilise les biomes clairs, 0-3)
                let gid = (rng.u32() % 4) as u16;
                map.set(tx, ty, Tile::new(gid, TileFlags(TileFlags::WALKABLE)));
            } else {
                // Extérieur : herbe variée normale
                let gid = (rng.u32() % 8) as u16;
                map.set(tx, ty, Tile::new(gid, TileFlags(TileFlags::WALKABLE)));
            }
        }
    }
    map
}
