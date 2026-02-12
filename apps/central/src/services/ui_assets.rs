//! Assets UI pour Miyukini UI Builder.
//!
//! Deux stratégies :
//! - **Générateurs SVG paramétriques** : produisent des SVG à n'importe quelle
//!   taille pour panneaux, boutons, barres. Les bordures gardent leur épaisseur
//!   absolue, seul le contenu s'adapte (principe 9-patch vectoriel).
//! - **PNG data URL** (base64) : curseurs pixel-art à taille fixe.
//!
//! @id: central.services.ui_assets
//! @do: provide_ui_asset_generators_and_embedded_sprites
//! @role: ui
//! @layer: infra
//! @human: Générateurs SVG paramétriques et sprites PNG embarqués pour l'UI Builder

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};

fn png_data_url(bytes: &[u8]) -> String {
    format!("data:image/png;base64,{}", BASE64.encode(bytes))
}

/// Encode une chaîne SVG en data URL utilisable dans `src` ou `background-image`.
pub fn svg_data_url(svg: &str) -> String {
    // Échapper les caractères problématiques pour une data URL non-base64
    let encoded = svg
        .replace('#', "%23")
        .replace('"', "'")
        .replace('\n', " ");
    format!("data:image/svg+xml,{encoded}")
}

// ═══════════════════════════════════════════════════════════════════════
//  SVG EMBARQUÉS (include_str!) — éléments à taille fixe
// ═══════════════════════════════════════════════════════════════════════

/// SVG embarqués (pas de redimensionnement nécessaire).
pub mod svg_fixed {
    pub const BUTTON_BROWN: &str = include_str!("../../../../ui/UI Pack - Adventure/Vector/button_brown.svg");
    pub const BUTTON_GREY: &str = include_str!("../../../../ui/UI Pack - Adventure/Vector/button_grey.svg");
    pub const BUTTON_RED: &str = include_str!("../../../../ui/UI Pack - Adventure/Vector/button_red.svg");
    pub const BUTTON_BROWN_CLOSE: &str = include_str!("../../../../ui/UI Pack - Adventure/Vector/button_brown_close.svg");
    pub const BUTTON_GREY_CLOSE: &str = include_str!("../../../../ui/UI Pack - Adventure/Vector/button_grey_close.svg");
    pub const BUTTON_RED_CLOSE: &str = include_str!("../../../../ui/UI Pack - Adventure/Vector/button_red_close.svg");

    pub const ROUND_BROWN: &str = include_str!("../../../../ui/UI Pack - Adventure/Vector/round_brown.svg");
    pub const ROUND_GREY: &str = include_str!("../../../../ui/UI Pack - Adventure/Vector/round_grey.svg");
    pub const ROUND_GREY_DETAILED_RED: &str = include_str!("../../../../ui/UI Pack - Adventure/Vector/round_grey_detailed_red.svg");
    pub const ROUND_GREY_DETAILED_GREEN: &str = include_str!("../../../../ui/UI Pack - Adventure/Vector/round_grey_detailed_green.svg");

    pub const SCROLLBAR_BROWN: &str = include_str!("../../../../ui/UI Pack - Adventure/Vector/scrollbar_brown.svg");
    pub const SCROLLBAR_GREY: &str = include_str!("../../../../ui/UI Pack - Adventure/Vector/scrollbar_grey.svg");

    pub const CHECKBOX_BROWN_CHECKED: &str = include_str!("../../../../ui/UI Pack - Adventure/Vector/checkbox_brown_checked.svg");
    pub const CHECKBOX_BROWN_EMPTY: &str = include_str!("../../../../ui/UI Pack - Adventure/Vector/checkbox_brown_empty.svg");
    pub const CHECKBOX_GREY_CHECKED: &str = include_str!("../../../../ui/UI Pack - Adventure/Vector/checkbox_grey_checked.svg");
    pub const CHECKBOX_GREY_EMPTY: &str = include_str!("../../../../ui/UI Pack - Adventure/Vector/checkbox_grey_empty.svg");

    pub const BANNER_HANGING: &str = include_str!("../../../../ui/UI Pack - Adventure/Vector/banner_hanging.svg");
    pub const BANNER_MODERN: &str = include_str!("../../../../ui/UI Pack - Adventure/Vector/banner_modern.svg");
}

// ═══════════════════════════════════════════════════════════════════════
//  GÉNÉRATEURS SVG PARAMÉTRIQUES — panneaux redimensionnables
// ═══════════════════════════════════════════════════════════════════════

/// Couleurs d'un thème de panneau.
struct PanelColors {
    outer: &'static str,  // bordure extérieure
    frame: &'static str,  // cadre principal
    shadow: &'static str, // ombre interne
    fill: &'static str,   // fond du contenu
}

const BROWN_COLORS: PanelColors = PanelColors {
    outer: "#6D4B27",
    frame: "#B47B41",
    shadow: "#A3703A",
    fill: "#FFF1D2",
};

const GREY_COLORS: PanelColors = PanelColors {
    outer: "#515F6B",
    frame: "#778D9F",
    shadow: "#647685",
    fill: "#94AFC6",
};

const DARK_BROWN_COLORS: PanelColors = PanelColors {
    outer: "#6D4B27",
    frame: "#B47B41",
    shadow: "#A3703A",
    fill: "#A3703A", // pas de centre clair
};

const DARK_GREY_COLORS: PanelColors = PanelColors {
    outer: "#515F6B",
    frame: "#778D9F",
    shadow: "#647685",
    fill: "#647685",
};

/// Génère un panneau plein (style panel_brown / panel_grey) à taille arbitraire.
///
/// Structure 9-patch :
/// - 2px bord extérieur arrondi (r=5)
/// - 4px cadre (2→6)
/// - 2px ombre interne (6→8)
/// - contenu à partir de 8px d'inset
fn gen_panel(w: u32, h: u32, c: &PanelColors) -> String {
    let (w, h) = (w.max(20), h.max(20));
    format!(
        r##"<svg width="{w}" height="{h}" xmlns="http://www.w3.org/2000/svg">
  <rect x="0" y="0" width="{w}" height="{h}" rx="5" fill="{outer}"/>
  <rect x="2" y="2" width="{iw2}" height="{ih2}" rx="3" fill="{frame}"/>
  <rect x="6" y="6" width="{iw6}" height="{ih6}" fill="{shadow}"/>
  <rect x="8" y="8" width="{iw8}" height="{ih8}" fill="{fill}"/>
</svg>"##,
        outer = c.outer,
        frame = c.frame,
        shadow = c.shadow,
        fill = c.fill,
        iw2 = w - 4,
        ih2 = h - 4,
        iw6 = w - 12,
        ih6 = h - 12,
        iw8 = w - 16,
        ih8 = h - 16,
    )
}

/// Panneau sans fond (cadre seul, style panel_border_grey).
fn gen_panel_border(w: u32, h: u32, c: &PanelColors) -> String {
    let (w, h) = (w.max(20), h.max(20));
    format!(
        r##"<svg width="{w}" height="{h}" xmlns="http://www.w3.org/2000/svg">
  <rect x="0" y="0" width="{w}" height="{h}" rx="5" fill="{outer}"/>
  <rect x="2" y="2" width="{iw2}" height="{ih2}" rx="3" fill="{frame}"/>
  <rect x="6" y="6" width="{iw6}" height="{ih6}" fill="{shadow}"/>
</svg>"##,
        outer = c.outer,
        frame = c.frame,
        shadow = c.shadow,
        iw2 = w - 4,
        ih2 = h - 4,
        iw6 = w - 12,
        ih6 = h - 12,
    )
}

/// Barre de progression horizontale (style progress_blue / green / red).
///
/// Couleurs : border, main, inner.
struct BarColors {
    border: &'static str,
    main: &'static str,
    inner: &'static str,
}

const BAR_BLUE: BarColors = BarColors { border: "#3B6891", main: "#68B8FF", inner: "#5CA2E0" };
const BAR_GREEN: BarColors = BarColors { border: "#528738", main: "#81D058", inner: "#72B84E" };
const BAR_RED: BarColors = BarColors { border: "#8E3F38", main: "#FF7366", inner: "#E2665B" };
const BAR_ORANGE: BarColors = BarColors { border: "#B47B41", main: "#FFBE5E", inner: "#E0A94F" };

fn gen_bar_h(w: u32, h: u32, c: &BarColors) -> String {
    let (w, h) = (w.max(12), h.max(12));
    let r = h / 2; // pill shape
    let r2 = r.saturating_sub(2);
    let r4 = r.saturating_sub(4);
    format!(
        r##"<svg width="{w}" height="{h}" xmlns="http://www.w3.org/2000/svg">
  <rect x="0" y="0" width="{w}" height="{h}" rx="{r}" fill="{border}"/>
  <rect x="2" y="2" width="{iw2}" height="{ih2}" rx="{r2}" fill="{main}"/>
  <rect x="4" y="4" width="{iw4}" height="{ih4}" rx="{r4}" fill="{inner}"/>
</svg>"##,
        border = c.border,
        main = c.main,
        inner = c.inner,
        iw2 = w - 4,
        ih2 = h - 4,
        iw4 = w - 8,
        ih4 = h - 8,
    )
}

// ═══════════════════════════════════════════════════════════════════════
//  API PUBLIQUE — fonctions de génération
// ═══════════════════════════════════════════════════════════════════════

/// Panneau marron plein (style inventory / dialogue).
pub fn panel_brown(w: u32, h: u32) -> String { gen_panel(w, h, &BROWN_COLORS) }

/// Panneau gris plein (style warning / system).
pub fn panel_grey(w: u32, h: u32) -> String { gen_panel(w, h, &GREY_COLORS) }

/// Panneau marron sombre (sans fond clair).
pub fn panel_brown_dark(w: u32, h: u32) -> String { gen_panel(w, h, &DARK_BROWN_COLORS) }

/// Panneau gris sombre (sans fond clair).
pub fn panel_grey_dark(w: u32, h: u32) -> String { gen_panel(w, h, &DARK_GREY_COLORS) }

/// Cadre marron (pas de fond).
pub fn border_brown(w: u32, h: u32) -> String { gen_panel_border(w, h, &BROWN_COLORS) }

/// Cadre gris (pas de fond).
pub fn border_grey(w: u32, h: u32) -> String { gen_panel_border(w, h, &GREY_COLORS) }

/// Barre horizontale bleue.
pub fn bar_blue(w: u32, h: u32) -> String { gen_bar_h(w, h, &BAR_BLUE) }

/// Barre horizontale verte.
pub fn bar_green(w: u32, h: u32) -> String { gen_bar_h(w, h, &BAR_GREEN) }

/// Barre horizontale rouge.
pub fn bar_red(w: u32, h: u32) -> String { gen_bar_h(w, h, &BAR_RED) }

/// Barre horizontale orange.
pub fn bar_orange(w: u32, h: u32) -> String { gen_bar_h(w, h, &BAR_ORANGE) }

/// Barre de fond (track) grise.
pub fn bar_track(w: u32, h: u32) -> String {
    let (w, h) = (w.max(12), h.max(12));
    let r = h / 2;
    format!(
        r##"<svg width="{w}" height="{h}" xmlns="http://www.w3.org/2000/svg">
  <rect x="0" y="0" width="{w}" height="{h}" rx="{r}" fill="#3a3a4a"/>
  <rect x="2" y="2" width="{iw}" height="{ih}" rx="{ri}" fill="#2a2a38"/>
</svg>"##,
        iw = w - 4,
        ih = h - 4,
        ri = r.saturating_sub(2),
    )
}

// ═══════════════════════════════════════════════════════════════════════
//  FANTASY UI BORDERS — PNG embarqués (sprites 48×48 / dividers)
// ═══════════════════════════════════════════════════════════════════════

/// Bordures fantaisie Kenney (blanches sur transparent, style RPG sombre).
pub mod fantasy_borders {
    use super::*;
    use std::sync::LazyLock;

    macro_rules! fantasy_png {
        ($name:ident, $file:literal) => {
            pub static $name: LazyLock<String> = LazyLock::new(|| {
                png_data_url(include_bytes!(concat!("../../../../ui/Fantasy UI Borders/PNG/Default/", $file)))
            });
        };
    }

    // ── Panels (fond + bordure décorative) ────────────────────────────
    fantasy_png!(PANEL_000, "Panel/panel-000.png");
    fantasy_png!(PANEL_001, "Panel/panel-001.png");
    fantasy_png!(PANEL_002, "Panel/panel-002.png");
    fantasy_png!(PANEL_003, "Panel/panel-003.png");
    fantasy_png!(PANEL_004, "Panel/panel-004.png");
    fantasy_png!(PANEL_005, "Panel/panel-005.png");
    fantasy_png!(PANEL_006, "Panel/panel-006.png");
    fantasy_png!(PANEL_007, "Panel/panel-007.png");
    fantasy_png!(PANEL_008, "Panel/panel-008.png");
    fantasy_png!(PANEL_010, "Panel/panel-010.png");
    fantasy_png!(PANEL_015, "Panel/panel-015.png");
    fantasy_png!(PANEL_016, "Panel/panel-016.png");
    fantasy_png!(PANEL_020, "Panel/panel-020.png");
    fantasy_png!(PANEL_024, "Panel/panel-024.png");
    fantasy_png!(PANEL_028, "Panel/panel-028.png");
    fantasy_png!(PANEL_031, "Panel/panel-031.png");

    // ── Borders (bordure seule, centre transparent) ───────────────────
    fantasy_png!(BORDER_000, "Border/panel-border-000.png");
    fantasy_png!(BORDER_001, "Border/panel-border-001.png");
    fantasy_png!(BORDER_002, "Border/panel-border-002.png");
    fantasy_png!(BORDER_003, "Border/panel-border-003.png");
    fantasy_png!(BORDER_004, "Border/panel-border-004.png");
    fantasy_png!(BORDER_008, "Border/panel-border-008.png");
    fantasy_png!(BORDER_015, "Border/panel-border-015.png");
    fantasy_png!(BORDER_020, "Border/panel-border-020.png");
    fantasy_png!(BORDER_024, "Border/panel-border-024.png");
    fantasy_png!(BORDER_028, "Border/panel-border-028.png");
    fantasy_png!(BORDER_031, "Border/panel-border-031.png");

    // ── Transparent Borders (bordure fine, centre transparent) ────────
    fantasy_png!(TBORDER_000, "Transparent border/panel-transparent-border-000.png");
    fantasy_png!(TBORDER_004, "Transparent border/panel-transparent-border-004.png");
    fantasy_png!(TBORDER_008, "Transparent border/panel-transparent-border-008.png");
    fantasy_png!(TBORDER_015, "Transparent border/panel-transparent-border-015.png");
    fantasy_png!(TBORDER_020, "Transparent border/panel-transparent-border-020.png");
    fantasy_png!(TBORDER_024, "Transparent border/panel-transparent-border-024.png");
    fantasy_png!(TBORDER_031, "Transparent border/panel-transparent-border-031.png");

    // ── Dividers (séparateurs horizontaux) ────────────────────────────
    fantasy_png!(DIVIDER_000, "Divider/divider-000.png");
    fantasy_png!(DIVIDER_001, "Divider/divider-001.png");
    fantasy_png!(DIVIDER_002, "Divider/divider-002.png");
    fantasy_png!(DIVIDER_003, "Divider/divider-003.png");
    fantasy_png!(DIVIDER_004, "Divider/divider-004.png");
    fantasy_png!(DIVIDER_005, "Divider/divider-005.png");

    // ── Divider Fades ─────────────────────────────────────────────────
    fantasy_png!(DIVIDER_FADE_000, "Divider Fade/divider-fade-000.png");
    fantasy_png!(DIVIDER_FADE_001, "Divider Fade/divider-fade-001.png");
    fantasy_png!(DIVIDER_FADE_002, "Divider Fade/divider-fade-002.png");
    fantasy_png!(DIVIDER_FADE_003, "Divider Fade/divider-fade-003.png");
    fantasy_png!(DIVIDER_FADE_004, "Divider Fade/divider-fade-004.png");
    fantasy_png!(DIVIDER_FADE_005, "Divider Fade/divider-fade-005.png");
}

// ═══════════════════════════════════════════════════════════════════════
//  CURSEURS — PNG embarqués (pixel art à taille fixe)
// ═══════════════════════════════════════════════════════════════════════

/// Curseurs Cursor Pack (Basic/Default) — organisés par catégorie.
pub mod cursors {
    use super::*;
    use std::sync::LazyLock;

    macro_rules! cursor_basic {
        ($name:ident, $file:literal) => {
            pub static $name: LazyLock<String> = LazyLock::new(|| {
                png_data_url(include_bytes!(concat!("../../../../ui/Cursor Pack/PNG/Basic/Default/", $file)))
            });
        };
    }

    // ── Pointers ──────────────────────────────────────────────────────
    cursor_basic!(POINTER_A, "pointer_a.png");
    cursor_basic!(POINTER_B, "pointer_b.png");
    cursor_basic!(POINTER_C, "pointer_c.png");
    cursor_basic!(POINTER_D, "pointer_d.png");
    cursor_basic!(POINTER_E, "pointer_e.png");
    cursor_basic!(POINTER_F, "pointer_f.png");
    cursor_basic!(POINTER_G, "pointer_g.png");
    cursor_basic!(POINTER_H, "pointer_h.png");
    cursor_basic!(POINTER_I, "pointer_i.png");
    cursor_basic!(POINTER_J, "pointer_j.png");
    cursor_basic!(POINTER_K, "pointer_k.png");
    cursor_basic!(POINTER_L, "pointer_l.png");
    cursor_basic!(POINTER_SCIFI_A, "pointer_scifi_a.png");
    cursor_basic!(POINTER_SCIFI_B, "pointer_scifi_b.png");
    cursor_basic!(POINTER_TOON_A, "pointer_toon_a.png");
    cursor_basic!(POINTER_TOON_B, "pointer_toon_b.png");

    // ── Hands ─────────────────────────────────────────────────────────
    cursor_basic!(HAND_POINT, "hand_point.png");
    cursor_basic!(HAND_POINT_E, "hand_point_e.png");
    cursor_basic!(HAND_POINT_N, "hand_point_n.png");
    cursor_basic!(HAND_OPEN, "hand_open.png");
    cursor_basic!(HAND_CLOSED, "hand_closed.png");
    cursor_basic!(HAND_SMALL_POINT, "hand_small_point.png");
    cursor_basic!(HAND_THIN_POINT, "hand_thin_point.png");

    // ── Gauntlets ─────────────────────────────────────────────────────
    cursor_basic!(GAUNTLET_DEFAULT, "gauntlet_default.png");
    cursor_basic!(GAUNTLET_OPEN, "gauntlet_open.png");
    cursor_basic!(GAUNTLET_POINT, "gauntlet_point.png");

    // ── System ────────────────────────────────────────────────────────
    cursor_basic!(HELP, "cursor_help.png");
    cursor_basic!(BUSY, "cursor_busy.png");
    cursor_basic!(COPY, "cursor_copy.png");
    cursor_basic!(DISABLED, "cursor_disabled.png");
    cursor_basic!(ALIAS, "cursor_alias.png");
    cursor_basic!(COGS, "cursor_cogs.png");
    cursor_basic!(EXCLAMATION, "cursor_exclamation.png");
    cursor_basic!(MENU, "cursor_menu.png");
    cursor_basic!(CURSOR_NONE, "cursor_none.png");

    // ── Resize ────────────────────────────────────────────────────────
    cursor_basic!(RESIZE_H, "resize_a_horizontal.png");
    cursor_basic!(RESIZE_V, "resize_a_vertical.png");
    cursor_basic!(RESIZE_DIAG, "resize_a_diagonal.png");
    cursor_basic!(RESIZE_DIAG_MIRROR, "resize_a_diagonal_mirror.png");
    cursor_basic!(RESIZE_CROSS, "resize_a_cross.png");
    cursor_basic!(RESIZE_CROSS_DIAG, "resize_a_cross_diagonal.png");

    // ── Drawing ───────────────────────────────────────────────────────
    cursor_basic!(DRAWING_PEN, "drawing_pen.png");
    cursor_basic!(DRAWING_PENCIL, "drawing_pencil.png");
    cursor_basic!(DRAWING_BRUSH, "drawing_brush.png");
    cursor_basic!(DRAWING_BUCKET, "drawing_bucket.png");
    cursor_basic!(DRAWING_ERASER, "drawing_eraser.png");
    cursor_basic!(DRAWING_PICKER, "drawing_picker.png");
    cursor_basic!(DRAWING_SPRAY, "drawing_spray.png");

    // ── Tools ─────────────────────────────────────────────────────────
    cursor_basic!(TOOL_SWORD_A, "tool_sword_a.png");
    cursor_basic!(TOOL_SWORD_B, "tool_sword_b.png");
    cursor_basic!(TOOL_AXE, "tool_axe.png");
    cursor_basic!(TOOL_HAMMER, "tool_hammer.png");
    cursor_basic!(TOOL_PICKAXE, "tool_pickaxe.png");
    cursor_basic!(TOOL_SHOVEL, "tool_shovel.png");
    cursor_basic!(TOOL_BOW, "tool_bow.png");
    cursor_basic!(TOOL_BOMB, "tool_bomb.png");
    cursor_basic!(TOOL_TORCH, "tool_torch.png");
    cursor_basic!(TOOL_WAND, "tool_wand.png");
    cursor_basic!(TOOL_WRENCH, "tool_wrench.png");

    // ── Navigation ────────────────────────────────────────────────────
    cursor_basic!(NAV_N, "navigation_n.png");
    cursor_basic!(NAV_S, "navigation_s.png");
    cursor_basic!(NAV_E, "navigation_e.png");
    cursor_basic!(NAV_W, "navigation_w.png");
    cursor_basic!(NAV_NE, "navigation_ne.png");
    cursor_basic!(NAV_NW, "navigation_nw.png");
    cursor_basic!(NAV_SE, "navigation_se.png");
    cursor_basic!(NAV_SW, "navigation_sw.png");

    // ── Zoom ──────────────────────────────────────────────────────────
    cursor_basic!(ZOOM_IN, "zoom_in.png");
    cursor_basic!(ZOOM_OUT, "zoom_out.png");
    cursor_basic!(ZOOM_RESET, "zoom_reset.png");
    cursor_basic!(ZOOM, "zoom.png");

    // ── Misc ──────────────────────────────────────────────────────────
    cursor_basic!(LOOK_A, "look_a.png");
    cursor_basic!(LOOK_B, "look_b.png");
    cursor_basic!(LOOK_C, "look_c.png");
    cursor_basic!(LOOK_D, "look_d.png");
    cursor_basic!(LOCK, "lock.png");
    cursor_basic!(LOCK_UNLOCKED, "lock_unlocked.png");
    cursor_basic!(TARGET_A, "target_a.png");
    cursor_basic!(TARGET_B, "target_b.png");
    cursor_basic!(BOOT, "boot.png");
    cursor_basic!(DOOR, "door.png");
    cursor_basic!(DOOR_ENTER, "door_enter.png");
    cursor_basic!(DOOR_EXIT, "door_exit.png");
    cursor_basic!(MARK_EXCLAMATION, "mark_exclamation.png");
    cursor_basic!(MARK_QUESTION, "mark_question.png");
    cursor_basic!(STAIRS, "stairs.png");
    cursor_basic!(STEPS, "steps.png");
    cursor_basic!(CROSS_LARGE, "cross_large.png");
    cursor_basic!(CROSS_SMALL, "cross_small.png");
    cursor_basic!(ROTATE_CW, "rotate_cw.png");
    cursor_basic!(ROTATE_CCW, "rotate_ccw.png");
}

/// Curseurs Cursor Pixel Pack (16×16 pixel art).
#[allow(dead_code)]
pub mod cursors_pixel {
    use super::*;
    use std::sync::LazyLock;

    macro_rules! pixel_cursor {
        ($name:ident, $file:literal) => {
            pub static $name: LazyLock<String> = LazyLock::new(|| {
                png_data_url(include_bytes!(concat!("../../../../ui/Cursor Pixel Pack/Tiles/", $file)))
            });
        };
    }

    // Sélection représentative (16×16 pixel art)
    pixel_cursor!(POINTER, "tile_0000.png");        // pointeur classique
    pixel_cursor!(POINTER_ALT, "tile_0020.png");    // pointeur alt
    pixel_cursor!(RESIZE, "tile_0001.png");          // resize
    pixel_cursor!(CROSSHAIR, "tile_0004.png");       // croix ciblage
    pixel_cursor!(ZOOM_IN, "tile_0005.png");         // zoom +
    pixel_cursor!(ZOOM_OUT, "tile_0006.png");        // zoom -
    pixel_cursor!(ZOOM_AREA, "tile_0007.png");       // zoom zone
    pixel_cursor!(ZOOM_AREA2, "tile_0008.png");      // zoom zone alt (réservé)
    pixel_cursor!(TEXT, "tile_0009.png");             // curseur texte
    pixel_cursor!(GRID, "tile_0010.png");            // grille
    pixel_cursor!(COPY, "tile_0011.png");            // copie
    pixel_cursor!(DISABLED, "tile_0012.png");        // interdit
    pixel_cursor!(CROSS, "tile_0013.png");           // croix
    pixel_cursor!(STAIRS, "tile_0014.png");          // escalier
    pixel_cursor!(LOCK, "tile_0015.png");            // cadenas
    pixel_cursor!(ARROW_RIGHT, "tile_0016.png");     // flèche droite
    pixel_cursor!(FLAG, "tile_0017.png");            // drapeau
    pixel_cursor!(MOVE, "tile_0002.png");            // déplacement
    pixel_cursor!(HAND, "tile_0003.png");            // main
    pixel_cursor!(PENCIL, "tile_0040.png");          // crayon
    pixel_cursor!(ARROW_UP, "tile_0041.png");        // flèche haut
    pixel_cursor!(BUCKET, "tile_0042.png");          // seau
    pixel_cursor!(ERASER, "tile_0043.png");          // gomme
    pixel_cursor!(PICKER, "tile_0044.png");          // pipette
    pixel_cursor!(SPRAY, "tile_0045.png");           // spray
    pixel_cursor!(ROTATE, "tile_0060.png");          // rotation
    pixel_cursor!(NAV_N, "tile_0061.png");           // nav nord
    pixel_cursor!(NAV_E, "tile_0062.png");           // nav est
    pixel_cursor!(DIAMOND, "tile_0063.png");         // diamant
    pixel_cursor!(LINK, "tile_0064.png");            // lien
}
