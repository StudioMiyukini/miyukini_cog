//! Vue jouable Miyukini Life Game — God Simulator intégré à Central.
//!
//! @id: central.services.lifegame
//! @role: ui
//! @layer: domain
//! @do: render_lifegame_godview_with_terrain_powers_and_simulation
//! @human: Jeu complet : canvas pixel art, pouvoirs divins, contrôle du temps, stats.

use dioxus::prelude::*;
use lifegame_entities::{EntityManager, Vec2};
use lifegame_world::{TerrainGrid, TerrainType, WorldGenerator, WorldSize};

mod canvas;
mod sidebar;
mod stats_bar;

use canvas::WorldCanvas;
use sidebar::PowersSidebar;

// ═══════════════════════════════════════════════════════════════════════════════
// Constantes thème God Game
// ═══════════════════════════════════════════════════════════════════════════════

const BG_DARK: &str = "#1a1a2e";
const BG_PANEL: &str = "#16213e";
const BORDER: &str = "#0f3460";
const ACCENT: &str = "#e94560";
const TEXT_LIGHT: &str = "#eaeaea";
const TEXT_MUTED: &str = "#94a3b8";

/// Taille d'une tile en pixels (pour le rendu).
const TILE_SIZE: u32 = 8;

/// Vitesses de jeu disponibles.
const SPEEDS: [(f32, &str); 5] = [
    (0.0, "⏸"),
    (1.0, "▶"),
    (2.0, "▶▶"),
    (5.0, "▶▶▶"),
    (10.0, "⏩"),
];

// ═══════════════════════════════════════════════════════════════════════════════
// État du jeu
// ═══════════════════════════════════════════════════════════════════════════════

/// État local du jeu Life Game.
#[derive(Clone)]
pub struct LifeGameLocalState {
    /// Grille de terrain.
    pub terrain: TerrainGrid,
    /// Gestionnaire d'entités.
    pub entities: EntityManager,
    /// Jour actuel.
    pub current_day: u32,
    /// Temps écoulé en secondes (jeu).
    pub elapsed_time: f32,
    /// Tick actuel.
    pub tick: u64,
    /// Multiplicateur de vitesse.
    pub speed_multiplier: f32,
    /// Outil sélectionné (terrain brush ou pouvoir).
    pub selected_tool: SelectedTool,
    /// Taille du pinceau.
    pub brush_size: u32,
    /// Position de la caméra (offset en pixels).
    #[allow(dead_code)]
    pub camera_offset: Vec2,
    /// Niveau de zoom (1.0 = 100%).
    pub zoom: f32,
}

impl Default for LifeGameLocalState {
    fn default() -> Self {
        let generator = WorldGenerator::new(42); // Seed fixe pour le démo
        let terrain = generator.generate(WorldSize::Small);
        
        Self {
            terrain,
            entities: EntityManager::new(),
            current_day: 1,
            elapsed_time: 0.0,
            tick: 0,
            speed_multiplier: 1.0,
            selected_tool: SelectedTool::Terrain(TerrainType::Grass),
            brush_size: 1,
            camera_offset: Vec2::new(0.0, 0.0),
            zoom: 1.0,
        }
    }
}

/// Outil sélectionné par le joueur.
#[derive(Clone, Debug, PartialEq)]
pub enum SelectedTool {
    /// Pinceau de terrain.
    Terrain(TerrainType),
    /// Pouvoir divin (par ID).
    Power(String),
    /// Sélection/inspection.
    #[allow(dead_code)]
    Select,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Vue principale
// ═══════════════════════════════════════════════════════════════════════════════

/// Vue principale du jeu Miyukini Life Game.
#[component]
pub fn LifeGameView() -> Element {
    // État de jeu — persiste grâce au pattern keep-alive
    let mut state = use_signal(LifeGameLocalState::default);
    let mut tick_started = use_signal(|| false);
    let notif = use_signal(|| "Bienvenue, Créateur ! Façonnez votre monde.".to_string());

    // Game tick — 100ms IRL
    if !*tick_started.read() {
        tick_started.set(true);
        spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                
                // Lire la vitesse d'abord (lecture courte)
                let speed = state.read().speed_multiplier;
                
                // Ne pas simuler si en pause (comparaison robuste pour float)
                if speed < 0.01 {
                    continue;
                }
                
                // Avancer la simulation
                let mut s = state.write();
                let delta_time = 0.1 * speed;
                s.elapsed_time += delta_time;
                let previous_day = s.current_day;
                // Protection contre overflow : limite à ~11 millions d'années
                s.current_day = ((s.elapsed_time / 86400.0) as u32).saturating_add(1);
                let is_new_day = s.current_day != previous_day;
                s.tick = s.tick.saturating_add(1);
                
                // Tick de simulation
                // Note: On doit cloner le terrain car tick_world a besoin de &mut entities et &terrain
                // simultanément. Une optimisation future serait de passer à une architecture ECS.
                let terrain_ref = s.terrain.clone();
                let _result = lifegame_simulation::tick_world(
                    &mut s.entities,
                    &terrain_ref,
                    is_new_day,
                );
            }
        });
    }

    // Lecture de l'état pour le render
    let s = state.read();
    let day = s.current_day;
    let population = s.entities.units.len();
    let buildings_count = s.entities.buildings.len();
    let creatures_count = s.entities.creatures.len();
    let current_speed = s.speed_multiplier;

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%; background: {BG_DARK}; border-radius: 8px; overflow: hidden; border: 2px solid {BORDER};",

            // ═══════════════════════════════════════════════
            //  HEADER — Stats et contrôles
            // ═══════════════════════════════════════════════
            div {
                style: "display: flex; align-items: center; justify-content: space-between; padding: 8px 16px; background: {BG_PANEL}; border-bottom: 1px solid {BORDER}; flex-shrink: 0;",

                // Titre
                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    span { style: "font-size: 16px;", "🌍" }
                    span { style: "font-size: 14px; font-weight: 700; color: {TEXT_LIGHT}; letter-spacing: 1px;", "Miyukini Life Game" }
                }

                // Stats rapides
                div {
                    style: "display: flex; gap: 16px;",
                    StatBadge { icon: "📅", label: "Jour", value: format!("{day}") }
                    StatBadge { icon: "👥", label: "Population", value: format!("{population}") }
                    StatBadge { icon: "🏠", label: "Bâtiments", value: format!("{buildings_count}") }
                    StatBadge { icon: "🐾", label: "Créatures", value: format!("{creatures_count}") }
                }

                // Contrôle vitesse
                div {
                    style: "display: flex; gap: 4px;",
                    for (spd, label) in SPEEDS {
                        {
                            let is_active = (current_speed - spd).abs() < 0.01;
                            let bg = if is_active { ACCENT } else { "rgba(255,255,255,0.1)" };
                            let color = if is_active { "#fff" } else { TEXT_MUTED };
                            rsx! {
                                button {
                                    style: "padding: 4px 10px; background: {bg}; border: none; border-radius: 4px; cursor: pointer; font-size: 12px; color: {color}; transition: all 0.15s;",
                                    onclick: move |_| {
                                        state.write().speed_multiplier = spd;
                                    },
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }

            // ═══════════════════════════════════════════════
            //  CONTENU PRINCIPAL — Sidebar + Canvas
            // ═══════════════════════════════════════════════
            div {
                style: "display: flex; flex: 1; min-height: 0;",

                // Sidebar pouvoirs
                PowersSidebar { state: state }

                // Canvas du monde
                div {
                    style: "flex: 1; display: flex; flex-direction: column; min-width: 0;",

                    // Canvas
                    WorldCanvas { state: state }

                    // Barre de notification
                    div {
                        style: "padding: 8px 16px; background: {BG_PANEL}; border-top: 1px solid {BORDER}; flex-shrink: 0;",
                        span { style: "font-size: 11px; color: {TEXT_MUTED};", "📜 {notif}" }
                    }
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Sous-composants simples
// ═══════════════════════════════════════════════════════════════════════════════

/// Badge de statistique dans le header.
#[component]
fn StatBadge(icon: &'static str, label: &'static str, value: String) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 6px; padding: 4px 10px; background: rgba(255,255,255,0.05); border-radius: 4px; border: 1px solid {BORDER};",
            span { style: "font-size: 12px;", "{icon}" }
            div {
                style: "display: flex; flex-direction: column;",
                span { style: "font-size: 8px; color: {TEXT_MUTED}; text-transform: uppercase;", "{label}" }
                span { style: "font-size: 12px; color: {TEXT_LIGHT}; font-weight: 600;", "{value}" }
            }
        }
    }
}
