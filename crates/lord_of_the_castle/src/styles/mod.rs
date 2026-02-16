//! Fonctions CSS pour le rendu UI de Lord of the Castle.
//!
//! Ce module contient les fonctions de génération de styles CSS
//! pour le positionnement et l'apparence des entités de jeu.
//!
//! @id: lord_of_the_castle.styles
//! @do: provide_css_style_functions
//! @role: ui_helper
//! @layer: ui
//! @human: Fonctions CSS partagées pour le rendu des entités de jeu.

use crate::constants::COMBAT_SURFACE_SIZE;

/// Surface de rendu (800×800 px, taille absolue de la zone de combat).
const SURFACE: f32 = COMBAT_SURFACE_SIZE;

/// Position en % de la surface de jeu (legacy, préférer px pour cohérence avec les sprites).
pub fn pct(v: f32) -> f32 {
    v / SURFACE * 100.0
}

/// Position en pixels (zone 800×800 : 1 unité monde = 1 px).
pub fn px(v: f32) -> f32 {
    v
}

/// Style CSS pour une entité (div positionnée absolument, centrée sur son point).
/// Utilise des tailles en pixels pour cohérence avec background-size des sprites.
pub fn entity_style(x: f32, y: f32, size: f32, color: &str) -> String {
    format!(
        "position:absolute;left:{xp:.0}px;top:{yp:.0}px;width:{sp:.0}px;height:{sp:.0}px;background:{c};transform:translate(-50%,-50%);pointer-events:none;border-radius:1px;",
        xp = px(x),
        yp = px(y),
        sp = px(size),
        c = color,
    )
}

/// Style CSS pour une barre de PV au-dessus d'une entité.
pub fn hp_bar_outer(x: f32, y: f32, entity_size: f32) -> String {
    let bar_height = 3.0f32;
    let top_y = y - entity_size / 2.0 - bar_height;
    let bar_width = entity_size.max(12.0);
    format!(
        "position:absolute;left:{xp:.0}px;top:{yp:.0}px;width:{wp:.0}px;height:{hp:.0}px;transform:translateX(-50%);background:#333;pointer-events:none;",
        xp = px(x),
        yp = px(top_y),
        wp = px(bar_width),
        hp = bar_height,
    )
}

/// Style CSS pour le remplissage de la barre de PV (couleur selon ratio).
pub fn hp_bar_fill_style(hp: i32, hp_max: i32) -> String {
    let ratio = if hp_max > 0 {
        (hp as f32 / hp_max as f32).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let color = if ratio > 0.6 {
        "#44cc44"
    } else if ratio > 0.3 {
        "#ccaa44"
    } else {
        "#cc4444"
    };
    format!(
        "width:{w:.1}%;height:100%;background:{c};",
        w = ratio * 100.0,
        c = color,
    )
}

/// CSS global de l'application.
pub const GLOBAL_CSS: &str = r#"
* { margin: 0; padding: 0; box-sizing: border-box; }
body { overflow: hidden; }
::-webkit-scrollbar { width: 5px; }
::-webkit-scrollbar-track { background: #1b2838; }
::-webkit-scrollbar-thumb { background: #2a3f5f; border-radius: 3px; }
button { font-family: inherit; }
button:hover { filter: brightness(1.15); }
"#;
