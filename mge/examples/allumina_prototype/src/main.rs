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
mod combat;
mod components;
mod dev_overlay;
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
use components::{AlluminaInput, AlluminaMap, EntitySprite, PlayerMarker};
use dev_overlay::DevState;
use isometric::{IsoCamera, ScreenPos};
use plugin::AlluminaPlugin;
use renderer::WgpuRenderer;
use stats::CombatStats;
use tilemap::TileMap;

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
/// Taille de la carte (tiles)
const MAP_SIZE: u32 = 64;

fn main() {
    env_logger::init();
    pollster::block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new().expect("event loop");
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Allumina — MGE v2.1 | ZQSD/↑↓←→=dépl  Clic-G=sélect  Clic-D=rallier  G=grille  F3=stats  Molette=zoom")
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

    // ── Joueur au centre ──────────────────────────────────────────────────────
    let cx = MAP_SIZE as f32 / 2.0;
    let cy = MAP_SIZE as f32 / 2.0;
    let player_entity = engine.world_mut().spawn();
    engine.world_mut().insert(player_entity, Position2D { x: cx, y: cy });
    engine.world_mut().insert(player_entity, PlayerMarker);
    engine.world_mut().insert(player_entity, EntitySprite::player());
    engine.world_mut().insert(player_entity, CombatStats::player());

    // ── Mobs (scatter déterministe) ───────────────────────────────────────────
    let mut rng = mge_rng::Rng::new(12345);
    for i in 0..MOB_COUNT {
        let mx = 2.0 + (rng.u32() % (MAP_SIZE - 4)) as f32 + 0.5;
        let my = 2.0 + (rng.u32() % (MAP_SIZE - 4)) as f32 + 0.5;
        let kind = (i % 5) as u8;
        let seed = 0xDEAD_0000u64 + i as u64;
        let mob = engine.world_mut().spawn();
        engine.world_mut().insert(mob, Position2D { x: mx, y: my });
        engine.world_mut().insert(mob, EntitySprite::mob(kind));
        engine.world_mut().insert(mob, CombatStats::mob(kind));
        engine.world_mut().insert(mob, MonsterAI::new(mx, my, seed));
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

                // ── Simulation ────────────────────────────────────────────────
                engine.tick(dt);

                // ── Nettoyage des morts ───────────────────────────────────────
                let dead_ids: Vec<_> = engine.world()
                    .iter1::<stats::Dead>()
                    .map(|(id, _)| id)
                    .collect();
                for id in dead_ids {
                    // Ne pas supprimer le joueur (immortel dans le prototype)
                    if !engine.world().has_component::<PlayerMarker>(id) {
                        // Désélectionner si c'était l'entité sélectionnée
                        if selected_entity == Some(id.to_bits()) {
                            selected_entity = None;
                        }
                        engine.world_mut().despawn(id);
                    }
                }

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
                    .map(|(_, _, s)| format!("  HP:{}/{}", s.hp, s.hp_max))
                    .unwrap_or_default();
                dev_state.update(dt, n_entities as usize, 0, n_sprites, player_pos);
                // Ajouter HP joueur au texte de stats
                if !player_hp_text.is_empty() && !dev_state.stats_text.is_empty() {
                    dev_state.stats_text.push_str(&player_hp_text);
                }

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
                    match renderer.draw_frame(
                        tm,
                        &camera,
                        entity_sprites.iter().map(|&(x, y, sid, hp, sel, _)| (x, y, sid, hp, sel)),
                        Some(&dev_state),
                        stats_opt,
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
                        if state == winit::event::ElementState::Pressed {
                            if let Some((px, py)) = last_cursor_pos {
                                let screen = ScreenPos::new(px as f32, py as f32);
                                let world_pos = camera.screen_to_world(
                                    screen,
                                    size.width as f32,
                                    size.height as f32,
                                );

                                // Clic gauche : sélectionner l'entité la plus proche (≤ 1.5 tiles)
                                if button == MouseButton::Left {
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
                    }

                    _ => {}
                }
            }
        })
        .expect("event loop run");
}

/// Génère une tilemap procédurale herbe variée (graphic_id 0–7)
fn make_grass_map(width: u32, height: u32) -> TileMap {
    use tilemap::{Tile, TileFlags};
    let mut map = TileMap::new(width, height);
    let mut rng = mge_rng::Rng::new(99999);
    for ty in 0..height {
        for tx in 0..width {
            let gid = (rng.u32() % 8) as u16;
            map.set(tx, ty, Tile::new(gid, TileFlags(TileFlags::WALKABLE)));
        }
    }
    map
}
