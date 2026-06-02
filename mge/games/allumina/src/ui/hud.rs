//! HUD: life/mana orbs and the XP bar.
//!
//! `render_hud` is the text version (headless); `draw` paints the interactive
//! Diablo-style globes, XP bar, level/name, and the skill hotbar.

use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::components::Pool;
use crate::ecs::World;
use crate::render::shapes;
use crate::render::skill_icon;
use crate::render::{font, vertex};

use super::{draw_orb, fill, SUB_CONTENT, SUB_PANEL, SUB_TEXT};

/// Paint the heads-up display: life/mana globes, XP bar, level medallion, hotbar.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::too_many_lines)]
pub fn draw(batch: &mut SpriteBatch, world: &World, viewport: [f32; 2], time: f32) {
    let Some(p) = world.player_entity else { return };
    let vw = viewport[0];
    let vh = viewport[1];

    let radius = 58.0_f32.min(vh * 0.12);
    let orb_y = vh - radius - 6.0;

    // Life globe (bottom-left).
    if let Some(h) = world.health.get(&p) {
        draw_orb(batch, radius + 12.0, orb_y, radius, h.fraction(), vertex::LIFE, time);
        let (cur, n) = font::fmt_u32(h.current.max(0.0) as u32);
        font::draw_text_centered(batch, &cur[..n], radius + 12.0, orb_y - 6.0, 1.6, vertex::WHITE, RenderLayer::UiScreen, 0, SUB_TEXT + 5);
    }
    // Mana globe (bottom-right).
    if let Some(m) = world.mana.get(&p) {
        let cx = vw - radius - 12.0;
        draw_orb(batch, cx, orb_y, radius, m.fraction(), vertex::MANA, time);
        let (cur, n) = font::fmt_u32(m.current.max(0.0) as u32);
        font::draw_text_centered(batch, &cur[..n], cx, orb_y - 6.0, 1.6, vertex::WHITE, RenderLayer::UiScreen, 0, SUB_TEXT + 5);
    }

    // XP bar spanning between the globes, with a gold gradient and a
    // "current/needed" readout (or MAX when the level cap is reached).
    let xp_frac = world.progression.get(&p).map_or(0.0, |pr| pr.inner.xp_fraction());
    let xp_cur = world.progression.get(&p).map_or(0, |pr| pr.inner.experience);
    let xp_needed = world.progression.get(&p).map_or(0, |pr| pr.inner.xp_to_next());
    let bx = radius * 2.0 + 30.0;
    let bw = vw - 2.0 * (radius * 2.0 + 30.0);
    let by = vh - 14.0;
    if bw > 40.0 {
        fill(batch, [bx - 1.0, by - 1.0, bw + 2.0, 10.0], SUB_PANEL, vertex::PANEL_BORDER);
        fill(batch, [bx, by, bw, 8.0], SUB_CONTENT, [0.10, 0.08, 0.04, 1.0]);
        let fw = bw * xp_frac.clamp(0.0, 1.0);
        // Two-stop gradient: darker gold base, bright gold sheen on top.
        fill(batch, [bx, by, fw, 8.0], SUB_CONTENT + 1, [0.55, 0.42, 0.10, 1.0]);
        fill(batch, [bx, by, fw, 4.0], SUB_CONTENT + 2, vertex::XP);
        let label = if xp_needed == u64::MAX {
            "MAX".to_string()
        } else {
            format!("{xp_cur}/{xp_needed}")
        };
        font::draw_text_centered(
            batch,
            label.as_bytes(),
            vw * 0.5,
            by - 13.0,
            1.3,
            [0.9, 0.85, 0.7, 1.0],
            RenderLayer::UiScreen,
            0,
            SUB_TEXT + 5,
        );
    }

    // Level medallion centred between the globes.
    let level = world.progression.get(&p).map_or(0, |pr| pr.level());
    draw_level_medallion(batch, vw * 0.5, vh - 50.0, level);

    // Class name above the medallion.
    let class = world.classes.get(&p).map_or("HERO", |c| c.display_name());
    font::draw_text_centered(batch, class.to_uppercase().as_bytes(), vw * 0.5, vh - 84.0, 1.6, [0.85, 0.80, 0.65, 1.0], RenderLayer::UiScreen, 0, SUB_TEXT);

    draw_hotbar(batch, world, p, vw, vh);
}

/// A round gold medallion showing the character level.
#[allow(clippy::cast_possible_truncation)]
fn draw_level_medallion(batch: &mut SpriteBatch, cx: f32, cy: f32, level: u32) {
    let r = 20.0;
    shapes::circle(batch, cx, cy, r + 2.0, RenderLayer::UiScreen, 0, SUB_PANEL, vertex::PANEL_BORDER);
    shapes::circle(batch, cx, cy, r, RenderLayer::UiScreen, 0, SUB_CONTENT, [0.10, 0.09, 0.12, 1.0]);
    shapes::ring(batch, cx, cy, r, 2.0, RenderLayer::UiScreen, 0, SUB_CONTENT + 1, [0.75, 0.60, 0.25, 1.0]);
    let (lvl, ln) = font::fmt_u32(level);
    font::draw_text_centered(batch, &lvl[..ln], cx, cy - 7.0, 2.2, vertex::XP, RenderLayer::UiScreen, 0, SUB_TEXT + 5);
}

/// The skill hotbar: one cell per active skill in a gold frame with a coloured
/// icon, the hotkey number, the selection highlight, and a cooldown wipe.
#[allow(clippy::cast_precision_loss)]
fn draw_hotbar(batch: &mut SpriteBatch, world: &World, player: crate::ecs::Entity, vw: f32, vh: f32) {
    let Some(slot) = world.skills.get(&player) else { return };
    let n = slot.bar.len();
    if n == 0 {
        return;
    }
    let cell = 42.0;
    let pad = 6.0;
    let total = n as f32 * (cell + pad) - pad;
    let x0 = vw * 0.5 - total * 0.5;
    let y = vh - 82.0;
    for (i, sk) in slot.bar.iter().enumerate() {
        let x = x0 + i as f32 * (cell + pad);
        let selected = i == slot.selected;
        let icon = skill_icon::fx_color(sk.fx);
        // Frame + coloured icon plate.
        fill(batch, [x - 1.0, y - 1.0, cell + 2.0, cell + 2.0], SUB_PANEL, if selected { vertex::WHITE } else { vertex::PANEL_BORDER });
        shapes::rounded_rect(batch, x + 2.0, y + 2.0, cell - 4.0, cell - 4.0, 6.0, RenderLayer::UiScreen, 0, SUB_CONTENT, [0.07, 0.07, 0.10, 1.0]);
        shapes::rounded_rect(batch, x + 5.0, y + 5.0, cell - 10.0, cell - 10.0, 5.0, RenderLayer::UiScreen, 0, SUB_CONTENT + 1, vertex::lerp(icon, [0.05, 0.05, 0.08, 1.0], 0.62));
        // Procedural skill glyph keyed to the cast's element/effect.
        skill_icon::draw_skill_glyph(batch, sk.fx, x + cell * 0.5, y + cell * 0.5, cell * 0.64, RenderLayer::UiScreen, 0, SUB_TEXT, 1.0);
        // Hotkey number, top-left.
        let (num, nn) = font::fmt_u32((i + 1) as u32);
        font::draw_text(batch, &num[..nn], x + 4.0, y + 4.0, 1.2, [0.85, 0.85, 0.9, 1.0], RenderLayer::UiScreen, 0, SUB_TEXT + 1);
        // Cooldown wipe from the top.
        if let Some(cd) = slot.cooldowns.get(i) {
            if !cd.is_ready() && sk.cooldown > 0.0 {
                let f = (cd.remaining / sk.cooldown).clamp(0.0, 1.0);
                fill(batch, [x + 2.0, y + 2.0, cell - 4.0, (cell - 4.0) * f], SUB_CONTENT + 2, [0.0, 0.0, 0.0, 0.62]);
            }
        }
    }
}

/// A textual progress bar, e.g. `[#######---]`.
pub fn bar(fraction: f32, width: usize) -> String {
    let filled = ((fraction.clamp(0.0, 1.0) * width as f32).round() as usize).min(width);
    let mut s = String::with_capacity(width + 2);
    s.push('[');
    for i in 0..width {
        s.push(if i < filled { '#' } else { '-' });
    }
    s.push(']');
    s
}

pub fn render_hud(world: &World) -> String {
    let Some(p) = world.player_entity else { return "[HUD] no player".to_owned() };
    let class = world.classes.get(&p).map_or("?", |c| c.display_name());
    let level = world.progression.get(&p).map_or(0, |pr| pr.level());
    let hp = world.health.get(&p).copied().unwrap_or(Pool::full(1.0));
    let mana = world.mana.get(&p).copied().unwrap_or(Pool::full(1.0));
    let xp_frac = world.progression.get(&p).map_or(0.0, |pr| pr.inner.xp_fraction());
    let xp_cur = world.progression.get(&p).map_or(0, |pr| pr.inner.experience);
    let xp_needed = world.progression.get(&p).map_or(0, |pr| pr.inner.xp_to_next());
    let gold = world.inventory.get(&p).map_or(0, |i| i.gold);
    let xp_label = if xp_needed == u64::MAX {
        "MAX".to_string()
    } else {
        format!("{xp_cur}/{xp_needed}")
    };

    format!(
        "{class} Lv.{level}   HP {hp_bar} {hpc:.0}/{hpm:.0}   MP {mp_bar} {mpc:.0}/{mpm:.0}\n\
         XP {xp_bar} {xp_label}   Gold: {gold}",
        hp_bar = bar(hp.fraction(), 12),
        hpc = hp.current,
        hpm = hp.max,
        mp_bar = bar(mana.fraction(), 12),
        mpc = mana.current,
        mpm = mana.max,
        xp_bar = bar(xp_frac, 20),
    )
}
