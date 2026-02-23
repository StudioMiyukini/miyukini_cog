//! @id allumina.prototype.isometric
//! @role utility
//! @layer application
//! @domain allumina
//! @do isometric_camera_and_coordinate_conversion
//!
//! Conversions isométriques et caméra.
//!
//! Formules (plan MVP) :
//!   screen_x = (world_x - world_y) * TILE_HALF_W
//!   screen_y = (world_x + world_y) * TILE_HALF_H

/// Dimensions d'une tuile isométrique (half-width, half-height du diamant)
pub const TILE_HALF_W: f32 = 32.0;
pub const TILE_HALF_H: f32 = 16.0;

/// Coordonnées monde (grille de tiles, x et y en unités tile)
#[derive(Debug, Clone, Copy)]
pub struct WorldPos {
    pub x: f32,
    pub y: f32,
}

impl WorldPos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Centre de la tuile (x, y)
    pub fn from_tile(tx: i32, ty: i32) -> Self {
        Self {
            x: tx as f32 + 0.5,
            y: ty as f32 + 0.5,
        }
    }
}

/// Coordonnées écran (pixels)
#[derive(Debug, Clone, Copy)]
pub struct ScreenPos {
    pub x: f32,
    pub y: f32,
}

impl ScreenPos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Monde → Écran (projection isométrique)
#[inline]
pub fn world_to_screen(world: WorldPos) -> ScreenPos {
    ScreenPos {
        x: (world.x - world.y) * TILE_HALF_W,
        y: (world.x + world.y) * TILE_HALF_H,
    }
}

/// Écran → Monde (inverse de la projection)
#[inline]
pub fn screen_to_world(screen: ScreenPos) -> WorldPos {
    let x = (screen.x / TILE_HALF_W + screen.y / TILE_HALF_H) / 2.0;
    let y = (screen.y / TILE_HALF_H - screen.x / TILE_HALF_W) / 2.0;
    WorldPos { x, y }
}

/// Mode de zoom : pixel-perfect (snap sur grille) ou libre
#[derive(Debug, Clone, Copy, Default)]
pub enum ZoomMode {
    /// Zoom libre (valeurs quelconques)
    #[default]
    Free,
    /// Zoom pixel-perfect : presets entiers 1x, 2x, 3x, snap caméra
    PixelPerfect,
}

/// Caméra isométrique (pan, zoom)
#[derive(Debug, Clone)]
pub struct IsoCamera {
    /// Position de la caméra en coordonnées monde (centre de la vue)
    pub center_x: f32,
    pub center_y: f32,
    pub zoom: f32,
    /// Mode de zoom (pixel-perfect optionnel)
    pub zoom_mode: ZoomMode,
}

impl Default for IsoCamera {
    fn default() -> Self {
        Self {
            center_x: 32.0,
            center_y: 32.0,
            zoom: 1.0,
            zoom_mode: ZoomMode::default(),
        }
    }
}

impl IsoCamera {
    pub fn new(center_x: f32, center_y: f32, zoom: f32) -> Self {
        Self {
            center_x,
            center_y,
            zoom: zoom.max(0.1),
            zoom_mode: ZoomMode::default(),
        }
    }

    /// Projette (world_x, world_y) en (screen_x, screen_y) pour le viewport donné.
    pub fn project(&self, world_x: f32, world_y: f32, viewport_w: f32, viewport_h: f32) -> (f32, f32) {
        let screen = world_to_screen(WorldPos::new(world_x, world_y));
        let center = world_to_screen(WorldPos::new(self.center_x, self.center_y));
        let dx = (screen.x - center.x) * self.zoom + viewport_w / 2.0;
        let dy = (screen.y - center.y) * self.zoom + viewport_h / 2.0;
        (dx, dy)
    }

    /// Convertit une position monde en position écran (avec offset caméra)
    pub fn world_to_screen(&self, world: WorldPos, viewport_w: f32, viewport_h: f32) -> ScreenPos {
        let screen = world_to_screen(world);
        let center = world_to_screen(WorldPos::new(self.center_x, self.center_y));
        ScreenPos {
            x: viewport_w / 2.0 + (screen.x - center.x) * self.zoom,
            y: viewport_h / 2.0 + (screen.y - center.y) * self.zoom,
        }
    }

    /// Convertit une position écran en position monde
    pub fn screen_to_world(&self, screen: ScreenPos, viewport_w: f32, viewport_h: f32) -> WorldPos {
        let dx = (screen.x - viewport_w / 2.0) / self.zoom;
        let dy = (screen.y - viewport_h / 2.0) / self.zoom;
        let center_screen = world_to_screen(WorldPos::new(self.center_x, self.center_y));
        screen_to_world(ScreenPos::new(center_screen.x + dx, center_screen.y + dy))
    }

    /// Déplace la caméra
    pub fn pan(&mut self, dx: f32, dy: f32) {
        self.center_x += dx;
        self.center_y += dy;
    }

    /// Clamp zoom selon le mode : PixelPerfect n'autorise que 1.0, 2.0, 3.0, 4.0
    pub fn set_zoom_clamped(&mut self, zoom: f32) {
        self.zoom = match self.zoom_mode {
            ZoomMode::Free => zoom.max(0.1).min(4.0),
            ZoomMode::PixelPerfect => {
                zoom.round().clamp(1.0, 4.0)
            }
        };
    }

    /// Snap la caméra sur la grille pixel écran (réduit le wobble en scroll).
    /// Appeler après mise à jour de center_x/center_y si zoom_mode == PixelPerfect.
    pub fn snap_to_pixel_grid(&mut self, viewport_w: f32, viewport_h: f32) {
        if !matches!(self.zoom_mode, ZoomMode::PixelPerfect) {
            return;
        }
        // Snap le point de projection écran du centre sur un pixel entier
        let center_screen = world_to_screen(WorldPos::new(self.center_x, self.center_y));
        // offset en pixels de la vue = (viewport/2) — doit être entier
        let snapped_cx = (center_screen.x * self.zoom + viewport_w / 2.0).floor() - viewport_w / 2.0;
        let snapped_cy = (center_screen.y * self.zoom + viewport_h / 2.0).floor() - viewport_h / 2.0;
        // Revenir en monde
        let snapped_world = screen_to_world(ScreenPos::new(
            snapped_cx / self.zoom,
            snapped_cy / self.zoom,
        ));
        self.center_x = snapped_world.x;
        self.center_y = snapped_world.y;
    }

    /// Retourne les bornes de tuiles visibles (min_tx, min_ty, max_tx, max_ty)
    /// pour un viewport et un nombre de tuiles de marge
    pub fn visible_tile_bounds(
        &self,
        viewport_w: f32,
        viewport_h: f32,
        margin: i32,
    ) -> (i32, i32, i32, i32) {
        let half_w = viewport_w / (2.0 * self.zoom * TILE_HALF_W);
        let half_h = viewport_h / (2.0 * self.zoom * TILE_HALF_H);
        let min_tx = (self.center_x - half_w - 1.0).floor() as i32 - margin;
        let max_tx = (self.center_x + half_w + 1.0).ceil() as i32 + margin;
        let min_ty = (self.center_y - half_h - 1.0).floor() as i32 - margin;
        let max_ty = (self.center_y + half_h + 1.0).ceil() as i32 + margin;
        (min_tx, min_ty, max_tx, max_ty)
    }
}
