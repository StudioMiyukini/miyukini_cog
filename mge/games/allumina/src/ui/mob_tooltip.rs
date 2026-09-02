//! @id     games_allumina_ui_mob_tooltip
//! @do     definir_bg
//! @role   ui
//! @layer  core
//! @human  games allumina — module « ui mob tooltip ».

//! Floating mob tooltip: shown when the cursor hovers an enemy. Displays the
//! mob's name, level and its elemental resistance/immunity profile so the
//! player can adapt their build (the late-game zones gate progression behind
//! immunity puzzles).
//!
//! The hover detection picks the nearest mob within half a tile of the
//! cursor's world position. The renderer in `ui::draw` calls [`draw`] after
//! the item-tooltip pass, so a hovered item still wins precedence.

use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::components::skill::Element;
use crate::components::Position2D;
use crate::data::mobs::{RES_FIRE, RES_FROST, RES_LIGHTNING, RES_POISON};
use crate::ecs::{Entity, World};
use crate::render::font;
use crate::render::vertex::Color;

use super::{border, fill, point_in, SUB_TOOLTIP};

const BG: Color = [0.0, 0.0, 0.0, 0.88];
const FRAME: Color = [0.55, 0.55, 0.60, 1.0];
const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
const GREY: Color = [0.72, 0.72, 0.76, 1.0];
const FIRE: Color = [1.0, 0.45, 0.15, 1.0];
const COLD: Color = [0.45, 0.80, 1.0, 1.0];
const LIGHTNING: Color = [1.0, 0.92, 0.35, 1.0];
const POISON: Color = [0.45, 0.85, 0.35, 1.0];
const VULN: Color = [1.0, 0.27, 0.27, 1.0];
const IMMUNE: Color = [1.0, 0.84, 0.0, 1.0];

/// Find the enemy mob under the cursor's world position, if any. Half-tile
/// radius is a generous-but-not-comical hitbox.
#[must_use]
pub fn hovered_mob(world: &World, cursor_world: Option<Position2D>) -> Option<Entity> {
    let cursor = cursor_world?;
    let mut best: Option<(Entity, f32)> = None;
    for e in world.mob_entities() {
        let Some(&pos) = world.positions.get(&e) else { continue };
        if world.health.get(&e).is_some_and(|h| h.is_empty()) {
            continue;
        }
        let d2 = pos.distance_sq(cursor);
        if d2 > 0.55 * 0.55 {
            continue;
        }
        if best.map_or(true, |(_, bd)| d2 < bd) {
            best = Some((e, d2));
        }
    }
    best.map(|(e, _)| e)
}

/// Draw the mob tooltip near `mouse`. No-op if `mob` has no `MobType` /
/// matching def. Skipped automatically when the cursor sits on a UI panel.
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]
pub fn draw(batch: &mut SpriteBatch, world: &World, mob: Entity, viewport: [f32; 2], mouse: [f32; 2]) {
    let Some(mt) = world.mob_types.get(&mob) else { return };
    let Some(def) = world.data.mob(mt.def_id) else { return };

    let lines = build_lines(world, mob, def);
    if lines.is_empty() {
        return;
    }

    let px = 1.5;
    let char_w = font::CELL * px;
    let line_h = 14.0;
    let pad = 8.0;
    let content_w = lines
        .iter()
        .map(|(t, _)| t.len() as f32 * char_w)
        .fold(0.0_f32, f32::max);
    let w = (content_w + pad * 2.0).max(160.0);
    let h = lines.len() as f32 * line_h + pad * 2.0;

    let mut x = mouse[0] + 16.0;
    if x + w > viewport[0] - 4.0 {
        x = mouse[0] - w - 16.0;
    }
    x = x.clamp(4.0, (viewport[0] - w - 4.0).max(4.0));
    let mut y = mouse[1] + 16.0;
    if y + h > viewport[1] - 4.0 {
        y = mouse[1] - h - 16.0;
    }
    y = y.clamp(4.0, (viewport[1] - h - 4.0).max(4.0));

    // Bail out early if the tooltip would sit entirely off-screen (shouldn't
    // happen after the clamps, but a defensive guard keeps stray draws out of
    // the batch).
    if !point_in([0.0, 0.0, viewport[0], viewport[1]], [x + w * 0.5, y + h * 0.5]) {
        return;
    }

    fill(batch, [x, y, w, h], SUB_TOOLTIP, BG);
    border(batch, [x, y, w, h], 2.0, SUB_TOOLTIP + 1, FRAME);
    for (i, (text, color)) in lines.iter().enumerate() {
        let cy = y + pad + i as f32 * line_h;
        font::draw_text(
            batch,
            text.as_bytes(),
            x + pad,
            cy,
            px,
            *color,
            RenderLayer::UiScreen,
            0,
            SUB_TOOLTIP + 2,
        );
    }
}

/// Build the (line, colour) list rendered by [`draw`]. Public so the headless
/// path / tests can sanity-check the formatting without a SpriteBatch.
#[must_use]
pub fn build_lines(world: &World, mob: Entity, def: &crate::data::mobs::MobDef) -> Vec<(String, Color)> {
    let mut out = Vec::new();
    out.push((format!("{} - Lvl {}", ascii_safe(&def.name), def.level), WHITE));
    if let Some(h) = world.health.get(&mob) {
        out.push((format!("HP {:.0} / {:.0}", h.current.max(0.0), h.max), GREY));
    }
    if def.is_boss {
        out.push(("BOSS".to_string(), IMMUNE));
    }
    if world.elite_mobs.contains(&mob) {
        out.push(("ELITE".to_string(), IMMUNE));
    }

    let res = [
        (RES_FIRE, "Feu", FIRE, Element::Fire),
        (RES_FROST, "Glace", COLD, Element::Frost),
        (RES_LIGHTNING, "Foudre", LIGHTNING, Element::Lightning),
        (RES_POISON, "Poison", POISON, Element::Poison),
    ];
    for (slot, label, base, elem) in res {
        let r = def.resistance(elem);
        if r.abs() < 0.005 {
            continue;
        }
        let (text, color) = if def.is_immune_to(elem) {
            (format!("Immunise {}", label), IMMUNE)
        } else if r < 0.0 {
            (format!("Vulnerable {} {:+.0}%", label, r * 100.0), VULN)
        } else {
            (format!("Resist {} +{:.0}%", label, r * 100.0), base)
        };
        // `slot` is unused by the rendering code; it makes intent explicit.
        let _ = slot;
        out.push((text, color));
    }
    out
}

/// Strip diacritics from the static mob names so they survive the 5x7 ASCII
/// bitmap font. (A real character map would be larger; this keeps tooltips
/// legible without touching the font asset.)
pub fn ascii_safe(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        let mapped = match c {
            'à' | 'â' | 'ä' | 'á' => 'a',
            'À' | 'Â' | 'Ä' | 'Á' => 'A',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'É' | 'È' | 'Ê' | 'Ë' => 'E',
            'î' | 'ï' | 'í' | 'ì' => 'i',
            'Î' | 'Ï' | 'Í' | 'Ì' => 'I',
            'ô' | 'ö' | 'ó' | 'ò' => 'o',
            'Ô' | 'Ö' | 'Ó' | 'Ò' => 'O',
            'û' | 'ü' | 'ú' | 'ù' => 'u',
            'Û' | 'Ü' | 'Ú' | 'Ù' => 'U',
            'ç' => 'c',
            'Ç' => 'C',
            'ñ' => 'n',
            'Ñ' => 'N',
            '\'' | '’' => '\'',
            c if c.is_ascii() => c,
            _ => '?',
        };
        out.push(mapped);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::player::ClassType;
    use crate::components::world::ZoneId;

    fn first_mob(app: &crate::App) -> Entity {
        app.world.mob_entities()[0]
    }

    #[test]
    fn lines_include_name_and_hp() {
        let app = crate::build_app_in(ClassType::Soldat, 0xA11CE, ZoneId::GreenPlains);
        let m = first_mob(&app);
        let def = app.world.data.mob(app.world.mob_types.get(&m).unwrap().def_id).unwrap();
        let lines = build_lines(&app.world, m, def);
        assert!(lines.iter().any(|(t, _)| t.contains("Lvl")), "should show level: {:?}", lines);
        assert!(lines.iter().any(|(t, _)| t.starts_with("HP")), "should show HP: {:?}", lines);
    }

    #[test]
    fn immunity_renders_as_immune_line() {
        // Tier-20 final boss (lvl 100, id 107) carries a fire immunity.
        let app = crate::build_app_in(ClassType::Soldat, 0xBEEF, ZoneId::EternalThrone);
        let m = app
            .world
            .mob_entities()
            .into_iter()
            .find(|e| {
                app.world
                    .mob_types
                    .get(e)
                    .and_then(|m| app.world.data.mob(m.def_id))
                    .is_some_and(|d| d.is_boss)
            })
            .expect("EternalThrone has a boss");
        let def = app.world.data.mob(app.world.mob_types.get(&m).unwrap().def_id).unwrap();
        let lines = build_lines(&app.world, m, def);
        assert!(lines.iter().any(|(t, _)| t.starts_with("Immunise")), "boss should expose at least one immunity: {:?}", lines);
    }

    #[test]
    fn ascii_safe_strips_accents() {
        assert_eq!(ascii_safe("Élémentaire de feu"), "Elementaire de feu");
        assert_eq!(ascii_safe("Cœur"), "C?ur");
        assert_eq!(ascii_safe("Liche Sombre"), "Liche Sombre");
    }
}
