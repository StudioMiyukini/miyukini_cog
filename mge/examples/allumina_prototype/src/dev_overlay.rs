//! @id allumina.prototype.dev_overlay
//! @role utility
//! @layer application
//! @domain allumina
//! @do dev_overlay_state_and_stats
//!
//! Overlay de développement : grille, stats FPS, infos entités.

/// État de l'overlay de développement
#[derive(Debug, Clone, Default)]
pub struct DevState {
    /// Afficher la grille isométrique
    pub show_grid: bool,
    /// Afficher les stats FPS/entités
    pub show_stats: bool,
    /// Dernières stats formatées (mise à jour chaque seconde)
    pub stats_text: String,
    /// Compteur de frames pour throttle stats
    pub frame_accumulator: u64,
    /// FPS estimé
    pub fps: f32,
    /// Temps accumulé pour le calcul FPS
    pub fps_time: f32,
    /// Nombre de frames dans la fenêtre FPS
    pub fps_frames: u32,
}

impl DevState {
    pub fn new_with_stats() -> Self {
        Self {
            show_stats: true,
            ..Default::default()
        }
    }

    /// Met à jour les stats (appeler chaque frame)
    pub fn update(&mut self, dt: f32, entity_count: usize, tiles_rendered: usize, sprites_rendered: usize, player_pos: Option<(f32, f32)>) {
        self.fps_time += dt;
        self.fps_frames += 1;
        self.frame_accumulator += 1;

        if self.fps_time >= 0.5 {
            self.fps = self.fps_frames as f32 / self.fps_time;
            self.fps_time = 0.0;
            self.fps_frames = 0;
        }

        if self.frame_accumulator % 30 == 0 {
            let pos_str = match player_pos {
                Some((px, py)) => format!("  Pos: ({:.1},{:.1})", px, py),
                None => String::new(),
            };
            self.stats_text = format!(
                "FPS: {:.0}  Entités: {}  Tiles: {}  Sprites: {}{}",
                self.fps, entity_count, tiles_rendered, sprites_rendered, pos_str
            );
        }
    }

    /// Toggle la grille avec la touche G
    pub fn toggle_grid(&mut self) {
        self.show_grid = !self.show_grid;
    }

    /// Toggle les stats avec la touche F3
    pub fn toggle_stats(&mut self) {
        self.show_stats = !self.show_stats;
    }
}
