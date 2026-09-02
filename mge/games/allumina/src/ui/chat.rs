//! @id     games_allumina_ui_chat
//! @do     definir_visible
//! @role   ui
//! @layer  core
//! @human  games allumina — module « ui chat ».

//! Chat / event-log box. For now it mirrors the sim's event log (combat, loot,
//! level-ups); once multiplayer lands (Phase 5) the same panel shows
//! `ChatBroadcast` messages from other players.

use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::render::font;
use crate::render::vertex;

use super::{fill, UiState, SUB_PANEL, SUB_TEXT};

/// Number of recent lines shown.
const VISIBLE: usize = 6;

/// Draw the chat/log box in the lower-left, above the life globe label.
#[allow(clippy::cast_precision_loss)]
pub fn draw(batch: &mut SpriteBatch, ui: &UiState, viewport: [f32; 2]) {
    let w = 360.0_f32.min(viewport[0] * 0.35);
    let line_h = 14.0;
    let h = VISIBLE as f32 * line_h + 10.0;
    let x = 140.0;
    let y = viewport[1] - h - 96.0;

    fill(batch, [x, y, w, h], SUB_PANEL, [0.02, 0.02, 0.03, 0.55]);

    let start = ui.chat.len().saturating_sub(VISIBLE);
    for (row, line) in ui.chat.iter().skip(start).enumerate() {
        let mut bytes: Vec<u8> = line.bytes().take(56).collect();
        bytes.retain(u8::is_ascii);
        let color = log_color(line);
        font::draw_text(batch, &bytes, x + 6.0, y + 5.0 + row as f32 * line_h, 1.3, color, RenderLayer::UiScreen, 0, SUB_TEXT);
    }
}

/// Tint a log line by a few keywords for readability.
fn log_color(line: &str) -> vertex::Color {
    if line.contains("slain") || line.contains("CRIT") {
        [1.0, 0.85, 0.3, 1.0]
    } else if line.contains("dropped") || line.contains("Picked up") || line.contains("Learned") {
        [0.5, 0.85, 1.0, 1.0]
    } else if line.contains("fallen") || line.contains("full") {
        [1.0, 0.4, 0.4, 1.0]
    } else {
        [0.8, 0.8, 0.85, 1.0]
    }
}
