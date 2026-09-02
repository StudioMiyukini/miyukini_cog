//! @id     games_allumina_ui_quest_log
//! @do     panel_rect
//! @role   ui
//! @layer  core
//! @human  games allumina — module « ui quest log ».

//! Quest log window: lists Act I objectives with live progress.

use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::components::quest::QuestKind;
use crate::ecs::World;
use crate::render::font;
use crate::render::vertex;

use super::{panel, Rect, SUB_TEXT};

#[must_use]
fn panel_rect(viewport: [f32; 2]) -> Rect {
    let w = 360.0;
    let h = 220.0;
    [viewport[0] * 0.5 - w * 0.5, 80.0, w, h]
}

/// Draw the quest log.
#[allow(clippy::cast_precision_loss)]
pub fn draw(batch: &mut SpriteBatch, world: &World, viewport: [f32; 2]) {
    let prect = panel_rect(viewport);
    let inner = panel(batch, prect, b"QUESTS");

    for (i, q) in world.quests.quests.iter().enumerate() {
        let y = inner[1] + 6.0 + i as f32 * 36.0;
        let color = if q.done { [0.5, 0.9, 0.5, 1.0] } else { vertex::WHITE };
        font::draw_text(batch, q.title.as_bytes(), inner[0], y, 1.6, color, RenderLayer::UiScreen, 0, SUB_TEXT);

        // Progress line.
        let mut line = Vec::new();
        if q.done {
            line.extend_from_slice(b"DONE");
        } else {
            let (p, pn) = font::fmt_u32(q.progress);
            let (g, gn) = font::fmt_u32(q.goal());
            line.extend_from_slice(&p[..pn]);
            line.push(b'/');
            line.extend_from_slice(&g[..gn]);
            line.push(b' ');
            line.extend_from_slice(describe(q.kind));
        }
        font::draw_text(batch, &line, inner[0] + 8.0, y + 16.0, 1.3, [0.75, 0.75, 0.8, 1.0], RenderLayer::UiScreen, 0, SUB_TEXT);
    }
}

fn describe(kind: QuestKind) -> &'static [u8] {
    match kind {
        QuestKind::KillAny(_) => b"slain",
        QuestKind::KillBoss => b"boss",
        QuestKind::CollectGold(_) => b"gold",
        QuestKind::CollectItems(_) => b"items",
        QuestKind::ReachZone(_) => b"reach dungeon",
    }
}
