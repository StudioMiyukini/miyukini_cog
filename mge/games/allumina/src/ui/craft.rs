//! @id     games_allumina_ui_craft
//! @do     definir_cell
//! @role   ui
//! @layer  core
//! @human  games allumina — module « ui craft ».

//! The 7×7 crafting table: paint materials into the grid, see the matched
//! recipe, and forge it. Pick a material from the palette (right), left-click
//! grid cells to place it (right-click to clear), then hit "Fabriquer".

use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::ecs::World;
use crate::render::font;
use crate::render::vertex::{self, Color};
use crate::systems::craft::{match_recipe, CRAFT_SIZE};
use crate::systems::forge::Material;

use super::{border, fill, panel, point_in, Rect, SUB_CONTENT, SUB_TEXT};

const CELL: f32 = 30.0;
const N: usize = CRAFT_SIZE;
const PALETTE_W: f32 = 150.0;

/// Display colour for a material swatch.
#[must_use]
pub fn mat_color(m: Material) -> Color {
    match m {
        Material::Fer => [0.62, 0.64, 0.68, 1.0],
        Material::Bronze => [0.72, 0.45, 0.25, 1.0],
        Material::Acier => [0.74, 0.78, 0.84, 1.0],
        Material::Argent => [0.85, 0.87, 0.90, 1.0],
        Material::Or => [0.92, 0.76, 0.26, 1.0],
        Material::Bois => [0.50, 0.34, 0.18, 1.0],
        Material::Gemmes => [0.45, 0.65, 0.95, 1.0],
    }
}

/// Centred crafting panel rect.
#[must_use]
pub fn panel_rect(viewport: [f32; 2]) -> Rect {
    let gw = CELL * N as f32;
    let w = gw + PALETTE_W + 40.0;
    let h = gw + 96.0;
    [viewport[0] * 0.5 - w * 0.5, viewport[1] * 0.5 - h * 0.5, w, h]
}

fn grid_origin(viewport: [f32; 2]) -> [f32; 2] {
    let p = panel_rect(viewport);
    [p[0] + 16.0, p[1] + 44.0]
}

#[allow(clippy::cast_precision_loss)]
fn cell_rect(viewport: [f32; 2], cx: usize, cy: usize) -> Rect {
    let o = grid_origin(viewport);
    [o[0] + cx as f32 * CELL, o[1] + cy as f32 * CELL, CELL - 2.0, CELL - 2.0]
}

/// Grid cell `(col, row)` under the cursor, if any.
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn cell_at(viewport: [f32; 2], mouse: [f32; 2]) -> Option<(usize, usize)> {
    let o = grid_origin(viewport);
    let (lx, ly) = (mouse[0] - o[0], mouse[1] - o[1]);
    if lx < 0.0 || ly < 0.0 {
        return None;
    }
    let (cx, cy) = ((lx / CELL) as usize, (ly / CELL) as usize);
    (cx < N && cy < N).then_some((cx, cy))
}

#[allow(clippy::cast_precision_loss)]
fn palette_row_rect(viewport: [f32; 2], i: usize) -> Rect {
    let p = panel_rect(viewport);
    let x = p[0] + 16.0 + CELL * N as f32 + 12.0;
    [x, p[1] + 44.0 + i as f32 * 28.0, PALETTE_W - 12.0, 26.0]
}

/// Palette material under the cursor, if any.
#[must_use]
pub fn palette_at(viewport: [f32; 2], mouse: [f32; 2]) -> Option<Material> {
    Material::ALL.iter().enumerate().find_map(|(i, &m)| {
        point_in(palette_row_rect(viewport, i), mouse).then_some(m)
    })
}

/// The "Fabriquer" button rect.
#[must_use]
pub fn craft_button_rect(viewport: [f32; 2]) -> Rect {
    let p = panel_rect(viewport);
    [p[0] + 16.0, p[1] + p[3] - 38.0, CELL * N as f32, 28.0]
}

/// Whether the cursor is over the craft button.
#[must_use]
pub fn craft_button_at(viewport: [f32; 2], mouse: [f32; 2]) -> bool {
    point_in(craft_button_rect(viewport), mouse)
}

/// Draw the crafting table.
#[allow(clippy::cast_possible_truncation)]
pub fn draw(batch: &mut SpriteBatch, world: &World, ui: &super::UiState, viewport: [f32; 2], mouse: [f32; 2]) {
    let Some(player) = world.player_entity else { return };
    let prect = panel_rect(viewport);
    panel(batch, prect, b"FABRICATION");

    // Grid cells.
    for cy in 0..N {
        for cx in 0..N {
            let r = cell_rect(viewport, cx, cy);
            let hovered = point_in(r, mouse);
            fill(batch, r, SUB_CONTENT, if hovered { [0.16, 0.16, 0.20, 1.0] } else { [0.09, 0.09, 0.12, 1.0] });
            border(batch, r, 1.0, SUB_CONTENT + 1, [0.28, 0.28, 0.33, 1.0]);
            if let Some(m) = ui.craft_grid[cy][cx] {
                let inner = [r[0] + 3.0, r[1] + 3.0, r[2] - 6.0, r[3] - 6.0];
                fill(batch, inner, SUB_CONTENT + 2, mat_color(m));
            }
        }
    }

    // Palette: each material with its swatch and the player's stock count.
    let stock = world.inventory.get(&player).map(|i| i.materials.clone()).unwrap_or_default();
    for (i, &m) in Material::ALL.iter().enumerate() {
        let r = palette_row_rect(viewport, i);
        let sel = ui.craft_selected == Some(m);
        fill(batch, r, SUB_CONTENT, if sel { [0.22, 0.22, 0.16, 1.0] } else { [0.10, 0.10, 0.13, 1.0] });
        border(batch, r, 1.0, SUB_CONTENT + 1, if sel { vertex::WHITE } else { [0.30, 0.30, 0.35, 1.0] });
        fill(batch, [r[0] + 4.0, r[1] + 4.0, 18.0, 18.0], SUB_CONTENT + 2, mat_color(m));
        let label: Vec<u8> = m.label().bytes().filter(u8::is_ascii).collect();
        font::draw_text(batch, &label, r[0] + 28.0, r[1] + 7.0, 1.2, SUB_TEXT_COL, RenderLayer::UiScreen, 0, SUB_TEXT);
        let (c, cn) = font::fmt_u32(stock.get(m));
        font::draw_text(batch, &c[..cn], r[0] + r[2] - 30.0, r[1] + 7.0, 1.2, vertex::XP, RenderLayer::UiScreen, 0, SUB_TEXT);
    }

    // Output preview + craft button.
    let matched = match_recipe(&ui.craft_grid);
    let out_label: Vec<u8> = matched.map_or_else(
        || b"Resultat: --".to_vec(),
        |r| {
            let mut v = b"Resultat: ".to_vec();
            v.extend(r.name.bytes().filter(u8::is_ascii));
            v
        },
    );
    font::draw_text(batch, &out_label, prect[0] + 16.0, prect[1] + prect[3] - 64.0, 1.3, SUB_TEXT_COL, RenderLayer::UiScreen, 0, SUB_TEXT);

    let br = craft_button_rect(viewport);
    let ready = matched.is_some();
    let hovered = point_in(br, mouse);
    fill(batch, br, SUB_CONTENT, if ready { if hovered { [0.18, 0.42, 0.20, 1.0] } else { [0.14, 0.32, 0.16, 1.0] } } else { [0.16, 0.16, 0.18, 1.0] });
    border(batch, br, 1.0, SUB_CONTENT + 1, if ready { [0.5, 0.9, 0.5, 1.0] } else { [0.34, 0.34, 0.38, 1.0] });
    font::draw_text_centered(batch, b"FABRIQUER", br[0] + br[2] * 0.5, br[1] + 8.0, 1.4, if ready { vertex::WHITE } else { [0.55, 0.55, 0.58, 1.0] }, RenderLayer::UiScreen, 0, SUB_TEXT);
}

const SUB_TEXT_COL: Color = [0.82, 0.82, 0.88, 1.0];
