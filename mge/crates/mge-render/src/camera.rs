/// Isometric camera with world-to-screen and screen-to-world conversions.
///
/// Uses a standard 2:1 isometric projection (tile width = 2 * tile height).
/// The camera tracks a world-space focus point and produces a view matrix
/// for the sprite renderer.

#[derive(Debug, Clone, Copy)]
pub struct IsoCamera {
    /// World-space focus (center of the viewport in world units).
    pub focus: [f32; 2],
    /// Zoom factor (pixels per world unit at 1x). Default 64.
    pub pixels_per_unit: f32,
    /// Additional zoom multiplier. Clamped to `zoom_range`.
    pub zoom: f32,
    /// Allowed zoom range `[min, max]`.
    pub zoom_range: [f32; 2],
    /// Viewport size in physical pixels.
    pub viewport: [u32; 2],
}

impl Default for IsoCamera {
    fn default() -> Self {
        Self {
            focus: [0.0, 0.0],
            pixels_per_unit: 64.0,
            zoom: 1.0,
            zoom_range: [0.5, 3.0],
            viewport: [1280, 720],
        }
    }
}

impl IsoCamera {
    /// Effective scale: `pixels_per_unit * zoom`.
    pub fn scale(&self) -> f32 {
        self.pixels_per_unit * self.zoom
    }

    /// Viewport center in pixels. Precision loss is negligible for any real viewport.
    #[allow(clippy::cast_precision_loss)]
    fn viewport_center(&self) -> [f32; 2] {
        [self.viewport[0] as f32 * 0.5, self.viewport[1] as f32 * 0.5]
    }

    /// Convert world coordinates `(wx, wy)` to screen pixel coordinates.
    ///
    /// Isometric projection (2:1):
    /// ```text
    /// sx = (wx - wy) * scale
    /// sy = (wx + wy) * scale * 0.5
    /// ```
    /// Then offset so that `focus` maps to viewport center.
    pub fn world_to_screen(&self, wx: f32, wy: f32) -> [f32; 2] {
        let s = self.scale();
        let iso_x = (wx - wy) * s;
        let iso_y = (wx + wy) * s * 0.5;

        let focus_iso_x = (self.focus[0] - self.focus[1]) * s;
        let focus_iso_y = (self.focus[0] + self.focus[1]) * s * 0.5;

        let [cx, cy] = self.viewport_center();

        [iso_x - focus_iso_x + cx, iso_y - focus_iso_y + cy]
    }

    /// Convert screen pixel coordinates `(sx, sy)` back to world coordinates.
    ///
    /// Inverse of `world_to_screen`.
    pub fn screen_to_world(&self, sx: f32, sy: f32) -> [f32; 2] {
        let s = self.scale();
        let [cx, cy] = self.viewport_center();

        let focus_iso_x = (self.focus[0] - self.focus[1]) * s;
        let focus_iso_y = (self.focus[0] + self.focus[1]) * s * 0.5;

        let iso_x = sx - cx + focus_iso_x;
        let iso_y = sy - cy + focus_iso_y;

        // Inverse of: iso_x = (wx - wy) * s, iso_y = (wx + wy) * s * 0.5
        let wx = (iso_x / s + iso_y / (s * 0.5)) * 0.5;
        let wy = (iso_y / (s * 0.5) - iso_x / s) * 0.5;

        [wx, wy]
    }

    /// Set zoom, clamped to `zoom_range`.
    pub fn set_zoom(&mut self, z: f32) {
        self.zoom = z.clamp(self.zoom_range[0], self.zoom_range[1]);
    }

    /// Update viewport size (called on window resize).
    pub fn set_viewport(&mut self, width: u32, height: u32) {
        self.viewport = [width, height];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_maps_to_viewport_center() {
        let cam = IsoCamera::default();
        let [sx, sy] = cam.world_to_screen(cam.focus[0], cam.focus[1]);
        let cx = cam.viewport[0] as f32 * 0.5;
        let cy = cam.viewport[1] as f32 * 0.5;
        assert!((sx - cx).abs() < 0.01, "sx={sx} expected {cx}");
        assert!((sy - cy).abs() < 0.01, "sy={sy} expected {cy}");
    }

    #[test]
    fn world_screen_roundtrip() {
        let cam = IsoCamera { focus: [5.0, 3.0], zoom: 1.5, ..IsoCamera::default() };
        let wx = 10.0_f32;
        let wy = 7.0_f32;
        let [sx, sy] = cam.world_to_screen(wx, wy);
        let [rwx, rwy] = cam.screen_to_world(sx, sy);
        assert!((rwx - wx).abs() < 0.01, "rwx={rwx} expected {wx}");
        assert!((rwy - wy).abs() < 0.01, "rwy={rwy} expected {wy}");
    }

    #[test]
    fn zoom_clamps_to_range() {
        let mut cam = IsoCamera::default();
        cam.set_zoom(10.0);
        assert!((cam.zoom - 3.0).abs() < f32::EPSILON);
        cam.set_zoom(0.1);
        assert!((cam.zoom - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn origin_offset_from_focus() {
        let cam = IsoCamera { focus: [0.0, 0.0], ..IsoCamera::default() };
        let [sx, sy] = cam.world_to_screen(1.0, 0.0);
        let s = cam.scale();
        let cx = cam.viewport[0] as f32 * 0.5;
        let cy = cam.viewport[1] as f32 * 0.5;
        // (1,0) -> iso_x = 1*s, iso_y = 1*s*0.5
        assert!((sx - (s + cx)).abs() < 0.01);
        assert!((sy - (s * 0.5 + cy)).abs() < 0.01);
    }
}
