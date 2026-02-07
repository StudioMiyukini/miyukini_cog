//! Outil COG spritesheet — découpage et affichage de sprites par-dessus une hitbox.
//!
//! Permet de charger une texture (strip horizontal), de calculer le rect UV d'une frame
//! et d'afficher la frame centrée sur une position (hitbox) avec transparence.
//!
//! @id: lord_of_the_castle_spritesheet
//! @do: load_and_draw_spritesheet_frames
//! @role: ui
//! @layer: infra
//! @human: Chargement PNG, découpage strip horizontal, affichage frame centrée (flip horizontal).

use eframe::egui;
use std::path::Path;

/// Charge une image PNG depuis le chemin et la convertit en `egui::ColorImage` (RGBA).
/// Retourne `None` si le fichier est absent ou invalide.
#[must_use] 
pub fn load_image_from_path(path: &Path) -> Option<egui::ColorImage> {
    let data = std::fs::read(path).ok()?;
    let img = image::load_from_memory(&data).ok()?;
    let img = img.to_rgba8();
    let size = [img.width() as usize, img.height() as usize];
    let pixels: Vec<egui::Color32> = img
        .chunks_exact(4)
        .map(|c| egui::Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]))
        .collect();
    Some(egui::ColorImage {
        size,
        source_size: egui::vec2(size[0] as f32, size[1] as f32),
        pixels,
    })
}

/// Description d'un spritesheet horizontal : nombre de frames, taille d'une frame.
#[derive(Debug, Clone)]
pub struct SpritesheetDesc {
    /// Nombre de frames (ex. 6 pour Knight-Idle).
    pub frame_count: usize,
    /// Largeur d'une frame en pixels (texteure).
    pub frame_width: u32,
    /// Hauteur d'une frame en pixels.
    pub frame_height: u32,
}

impl SpritesheetDesc {
    /// Crée une description pour un strip horizontal (toutes les frames ont la même taille).
    #[must_use] 
    pub fn horizontal(total_width: u32, total_height: u32, frame_count: usize) -> Self {
        let frame_width = total_width / frame_count.max(1) as u32;
        Self {
            frame_count,
            frame_width,
            frame_height: total_height,
        }
    }

    /// Retourne le rect UV (0–1) pour la frame d'index donné (strip horizontal).
    #[must_use] 
    pub fn frame_uv_rect(&self, frame_index: usize) -> egui::Rect {
        let n = self.frame_count.max(1);
        let i = frame_index % n;
        let u0 = (i as f32) / (n as f32);
        let u1 = ((i + 1) as f32) / (n as f32);
        egui::Rect::from_min_max(
            egui::pos2(u0, 0.0),
            egui::pos2(u1, 1.0),
        )
    }

    /// Taille d'une frame en pixels (pour dimensionner l'affichage à l'écran).
    #[must_use] 
    pub fn frame_size(&self) -> (f32, f32) {
        (self.frame_width as f32, self.frame_height as f32)
    }
}

/// Dessine une frame de spritesheet centrée sur (center_screen_x, center_screen_y).
/// La hitbox reste inchangée ; le sprite est affiché par-dessus (sans modifier la logique de collision).
/// Si `flip_h` est true, la frame est affichée en miroir horizontal (pour gauche en 2D plateforme).
pub fn draw_sprite_frame(
    painter: &egui::Painter,
    texture: &egui::TextureHandle,
    uv_rect: egui::Rect,
    center_screen: egui::Pos2,
    frame_size: (f32, f32),
    tint: egui::Color32,
    flip_h: bool,
) {
    let uv = if flip_h {
        egui::Rect::from_min_max(
            egui::pos2(uv_rect.max.x, uv_rect.min.y),
            egui::pos2(uv_rect.min.x, uv_rect.max.y),
        )
    } else {
        uv_rect
    };
    let (w, h) = frame_size;
    let half_w = w / 2.0;
    let half_h = h / 2.0;
    let rect = egui::Rect::from_min_max(
        egui::pos2(center_screen.x - half_w, center_screen.y - half_h),
        egui::pos2(center_screen.x + half_w, center_screen.y + half_h),
    );
    painter.image(texture.id(), rect, uv, tint);
}
