// @id: sd-impl-camera @do: implement @role: back-end @layer: 2 @human: miyuk

//! Isometric camera: world/screen conversion, smooth follow, and bounds clamping.

/// Tile width in pixels for dimetric 2:1 projection.
pub const TILE_WIDTH: f32 = 64.0;

/// Tile height in pixels for dimetric 2:1 projection.
pub const TILE_HEIGHT: f32 = 32.0;

/// 2D isometric camera with smooth follow and bounds clamping.
///
/// The camera stores its position in *screen-pixel* space. All isometric
/// conversion (world tiles to screen pixels) goes through
/// [`world_to_screen`] / [`screen_to_world`].
#[derive(Debug, Clone)]
pub struct Camera2D {
    /// Current X position of the camera centre (screen pixels).
    pub world_x: f32,
    /// Current Y position of the camera centre (screen pixels).
    pub world_y: f32,
    /// Zoom factor (1.0 = default, 2.0 = double size).
    pub zoom: f32,
    /// Viewport width in pixels.
    pub screen_w: u32,
    /// Viewport height in pixels.
    pub screen_h: u32,
    /// Target X for smooth follow.
    pub target_x: f32,
    /// Target Y for smooth follow.
    pub target_y: f32,
    /// Smooth speed factor (0.0 = no smoothing, 1.0 = snap).
    pub smooth_speed: f32,
    /// Optional bounds to clamp the camera.
    pub bounds: Option<CameraBounds>,
}

/// Axis-aligned bounds that prevent the camera from scrolling outside the map.
#[derive(Debug, Clone, Copy)]
pub struct CameraBounds {
    /// Minimum X bound (screen pixels).
    pub min_x: f32,
    /// Minimum Y bound (screen pixels).
    pub min_y: f32,
    /// Maximum X bound (screen pixels).
    pub max_x: f32,
    /// Maximum Y bound (screen pixels).
    pub max_y: f32,
}

impl Default for Camera2D {
    fn default() -> Self {
        Self::new(800, 600)
    }
}

impl Camera2D {
    /// Create a new camera centred at the origin.
    pub fn new(screen_w: u32, screen_h: u32) -> Self {
        Self {
            world_x: 0.0,
            world_y: 0.0,
            zoom: 1.0,
            screen_w,
            screen_h,
            target_x: 0.0,
            target_y: 0.0,
            smooth_speed: 0.1,
            bounds: None,
        }
    }

    /// Convert world tile coordinates to screen pixel coordinates.
    ///
    /// Dimetric 2:1 projection:
    /// ```text
    /// sx = (wx - wy) * (TILE_WIDTH / 2) * zoom
    /// sy = (wx + wy) * (TILE_HEIGHT / 2) * zoom
    /// ```
    pub fn world_to_screen(&self, wx: f32, wy: f32) -> (f32, f32) {
        let sx = (wx - wy) * (TILE_WIDTH / 2.0) * self.zoom;
        let sy = (wx + wy) * (TILE_HEIGHT / 2.0) * self.zoom;
        (sx, sy)
    }

    /// Convert screen pixel coordinates back to world tile coordinates.
    ///
    /// Exact inverse of [`world_to_screen`](Self::world_to_screen).
    pub fn screen_to_world(&self, sx: f32, sy: f32) -> (f32, f32) {
        let half_w = (TILE_WIDTH / 2.0) * self.zoom;
        let half_h = (TILE_HEIGHT / 2.0) * self.zoom;
        let wx = (sx / half_w + sy / half_h) / 2.0;
        let wy = (sy / half_h - sx / half_w) / 2.0;
        (wx, wy)
    }

    /// Update the camera position towards the target with smoothing.
    pub fn update(&mut self, dt: f32) {
        let factor = 1.0 - (1.0 - self.smooth_speed).powf(dt * 60.0);
        self.world_x += (self.target_x - self.world_x) * factor;
        self.world_y += (self.target_y - self.world_y) * factor;

        if let Some(ref b) = self.bounds {
            self.world_x = self.world_x.clamp(b.min_x, b.max_x);
            self.world_y = self.world_y.clamp(b.min_y, b.max_y);
        }
    }

    /// Set the follow target from world tile coordinates.
    pub fn follow(&mut self, tile_x: f32, tile_y: f32) {
        let (sx, sy) = self.world_to_screen(tile_x, tile_y);
        self.target_x = sx;
        self.target_y = sy;
    }
}

/// Convert world tile coordinates to screen pixels (free function, zoom = 1).
pub fn world_to_screen(tile_x: f32, tile_y: f32) -> (f32, f32) {
    let sx = (tile_x - tile_y) * (TILE_WIDTH / 2.0);
    let sy = (tile_x + tile_y) * (TILE_HEIGHT / 2.0);
    (sx, sy)
}

/// Convert screen pixels to world tile coordinates (free function, zoom = 1).
pub fn screen_to_world(screen_x: f32, screen_y: f32) -> (f32, f32) {
    let half_w = TILE_WIDTH / 2.0;
    let half_h = TILE_HEIGHT / 2.0;
    let tx = (screen_x / half_w + screen_y / half_h) / 2.0;
    let ty = (screen_y / half_h - screen_x / half_w) / 2.0;
    (tx, ty)
}

/// Convert screen pixels to integer tile coordinates (floor).
pub fn screen_to_tile(screen_x: f32, screen_y: f32) -> (i32, i32) {
    let (tx, ty) = screen_to_world(screen_x, screen_y);
    (tx.floor() as i32, ty.floor() as i32)
}
