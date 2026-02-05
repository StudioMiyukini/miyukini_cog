//! Icônes Lucide pour le header Central.
//! Charge les SVG depuis ui/lucide_icons (embed à la compilation), les rasterise
//! avec resvg/usvg et enregistre les textures dans egui.

use eframe::egui;
use resvg::render;
use tiny_skia::Pixmap;
use usvg::{Options, Tree};

const ICON_SIZE: u32 = 32;

/// Chemins des SVG (relatifs à la racine du workspace, via CARGO_MANIFEST_DIR/../..).
macro_rules! svg_path {
    ($subpath:literal) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/../../ui/lucide_icons/", $subpath)
    };
}

/// Charge un SVG Lucide (contenu embed) en texture egui.
/// Retourne un TextureHandle pour garder la texture vivante (sinon elle est libérée).
fn svg_to_texture(
    cc: &eframe::CreationContext<'_>,
    svg_str: &str,
    name: &str,
) -> Option<egui::TextureHandle> {
    let opt = Options::default();
    let tree = Tree::from_str(svg_str, &opt).ok()?;
    let size = tree.size();
    let w = size.width();
    let h = size.height();
    if w <= 0.0 || h <= 0.0 {
        return None;
    }
    let mut pixmap = Pixmap::new(ICON_SIZE, ICON_SIZE)?;
    let scale = (ICON_SIZE as f32 / w.max(h)).min(4.0);
    let ts = tiny_skia::Transform::from_scale(scale, scale);
    render(&tree, ts, &mut pixmap.as_mut());
    // Convertir en silhouette blanche : les pixels non transparents deviennent blancs,
    // pour que .tint(couleur) dans egui affiche l'icône dans la bonne couleur (clair sur fond sombre).
    {
        let pixels = pixmap.data_mut();
        for chunk in pixels.chunks_exact_mut(4) {
            let a = chunk[3];
            if a > 0 {
                chunk[0] = 255;
                chunk[1] = 255;
                chunk[2] = 255;
            }
        }
    }
    let size = [ICON_SIZE as usize, ICON_SIZE as usize];
    let color_image = egui::ColorImage {
        size,
        source_size: egui::vec2(ICON_SIZE as f32, ICON_SIZE as f32),
        pixels: pixmap
            .data()
            .chunks_exact(4)
            .map(|c| egui::Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]))
            .collect(),
    };
    let handle = cc.egui_ctx.load_texture(name, color_image, egui::TextureOptions::default());
    Some(handle)
}

/// Enregistre les icônes Lucide du header (cog, user, settings).
/// Retourne des TextureHandle pour garder les textures vivantes.
pub fn register_header_icons(cc: &eframe::CreationContext<'_>) -> (
    Option<egui::TextureHandle>,
    Option<egui::TextureHandle>,
    Option<egui::TextureHandle>,
) {
    let cog_svg = include_str!(svg_path!("account/cog.svg"));
    let user_svg = include_str!(svg_path!("account/circle-user.svg"));
    let settings_svg = include_str!(svg_path!("account/settings.svg"));

    let cog = svg_to_texture(cc, cog_svg, "lucide_header_cog");
    let user = svg_to_texture(cc, user_svg, "lucide_header_user");
    let settings = svg_to_texture(cc, settings_svg, "lucide_header_settings");

    (cog, user, settings)
}
