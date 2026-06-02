//! Skill-tree window.
//!
//! Built around *branches*: a class natively owns three branches, and a player
//! can spend 1 skill point to unlock a single foreign branch (any of the nine
//! belonging to the three other classes). Once unlocked, branch skills are
//! invested like native ones — the underlying [`SkillTree`] accepts them
//! because [`data::skills::skill_def_for`] tags each `SkillDef` with the
//! player's own mge `Class`.

use mge_gameplay::skills::{class_skill_stubs, SkillDef, SkillId};
use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::components::player::ClassType;
use crate::components::skill::{Cooldown, SkillTree};

/// Maximum number of skills the hotbar can hold (hotkeys 1-9).
pub const HOTBAR_MAX: usize = 9;
use crate::data::skills::{active_skill_by_id, branches_for_class, prerequisite_for, skill_def_for, Branch};
use crate::ecs::World;
use crate::render::{font, skill_icon, vertex};

use super::{fill, panel, point_in, Rect, SUB_CONTENT, SUB_TEXT};

pub fn render_skill_tree(tree: &SkillTree) -> String {
    let defs = class_skill_stubs(tree.class);
    let mut out = format!("Skill Tree ({:?})   Points: {}\n", tree.class, tree.available_points);
    for d in defs {
        let lvl = tree.level_of(d.id);
        out.push_str(&format!("  [{}/{}] {:<16} (unlocks at lvl {})\n", lvl, d.max_level, d.name, d.min_level));
    }
    out
}

const ROW_W: f32 = 320.0;
const ROW_H: f32 = 22.0;
const ROW_PAD: f32 = 4.0;
const HEADER_H: f32 = 20.0;
const SECTION_GAP: f32 = 10.0;
const PLUS_W: f32 = 22.0;
const PLUS_H: f32 = 16.0;
const PANEL_W: f32 = ROW_W + 40.0;
const PANEL_H: f32 = 540.0;
const UNLOCK_BTN_W: f32 = 80.0;
const UNLOCK_BTN_H: f32 = 18.0;
const TOGGLE_BTN_W: f32 = 220.0;
const TOGGLE_BTN_H: f32 = 24.0;

/// Centred panel rect.
#[must_use]
pub fn panel_rect(viewport: [f32; 2]) -> Rect {
    [viewport[0] * 0.5 - PANEL_W * 0.5, viewport[1] * 0.5 - PANEL_H * 0.5, PANEL_W, PANEL_H]
}

/// Branch color — tints rows so foreign-branch skills are visually distinct
/// from the player's own. The hue follows the originating class, not the
/// owner.
fn class_color(class: ClassType) -> [f32; 4] {
    match class {
        ClassType::Soldat => [0.85, 0.55, 0.20, 1.0],
        ClassType::Clerc => [0.90, 0.85, 0.45, 1.0],
        ClassType::MageDeGuerre => [0.50, 0.55, 0.95, 1.0],
        ClassType::Archer => [0.45, 0.85, 0.45, 1.0],
    }
}

/// Every branch (native + unlocked-foreign) the player can currently see in
/// the tree, in display order.
fn visible_branches(world: &World, player_class: ClassType) -> Vec<Branch> {
    let mut out: Vec<Branch> = branches_for_class(player_class);
    let unlocked = world.player_entity.and_then(|p| world.unlocked_branches.get(&p)).cloned().unwrap_or_default();
    for (c, idx) in unlocked {
        if c == player_class {
            continue;
        }
        if let Some(b) = branches_for_class(c).into_iter().find(|b| b.index == idx) {
            out.push(b);
        }
    }
    out
}

/// Foreign branches not yet unlocked. Used by the "Débloquer" sub-menu.
fn lockable_branches(world: &World, player_class: ClassType) -> Vec<Branch> {
    let unlocked = world.player_entity.and_then(|p| world.unlocked_branches.get(&p)).cloned().unwrap_or_default();
    let mut out = Vec::new();
    for class in ClassType::all() {
        if class == player_class {
            continue;
        }
        for b in branches_for_class(class) {
            let already = unlocked.iter().any(|(c, i)| *c == class && *i == b.index);
            if !already {
                out.push(b);
            }
        }
    }
    out
}

/// Walk the visible branches and produce one rect per skill row, tagged with
/// `(skill_id, branch_owner_class)`.
fn skill_rows(world: &World, viewport: [f32; 2]) -> Vec<(Rect, SkillId, ClassType)> {
    let Some(player) = world.player_entity else { return Vec::new() };
    if world.skill_trees.get(&player).is_none() {
        return Vec::new();
    }
    let class = world.classes.get(&player).copied().unwrap_or(world.chosen_class);
    let p = panel_rect(viewport);
    let mut rows = Vec::new();
    let mut y = p[1] + 50.0;
    let x = p[0] + 16.0;
    for branch in visible_branches(world, class) {
        // Section header consumes HEADER_H + a small gap.
        y += HEADER_H + 2.0;
        for &sid in &branch.skill_ids {
            rows.push(([x, y, ROW_W, ROW_H], sid, branch.class));
            y += ROW_H + ROW_PAD;
        }
        y += SECTION_GAP;
    }
    rows
}

/// Rect of the "+" button on a given row.
fn plus_rect_from_row(row: Rect) -> Rect {
    [row[0] + row[2] - PLUS_W - 4.0, row[1] + (ROW_H - PLUS_H) * 0.5, PLUS_W, PLUS_H]
}

/// Skill id under the cursor, if any. Used for hover highlight.
#[must_use]
pub fn node_at(world: &World, viewport: [f32; 2], mouse: [f32; 2]) -> Option<SkillId> {
    for (row, sid, _) in skill_rows(world, viewport) {
        if point_in(row, mouse) {
            return Some(sid);
        }
    }
    None
}

/// Skill id of the "+" button under the cursor, only when the skill is
/// currently learnable.
#[must_use]
pub fn plus_at(world: &World, viewport: [f32; 2], mouse: [f32; 2]) -> Option<SkillId> {
    let player = world.player_entity?;
    let char_level = world.progression.get(&player).map_or(1, |p| p.level());
    let tree = world.skill_trees.get(&player)?;
    let owner_class = world.classes.get(&player).copied().unwrap_or(world.chosen_class);
    for (row, sid, _) in skill_rows(world, viewport) {
        if point_in(plus_rect_from_row(row), mouse) {
            if let Some(def) = skill_def_for(sid, owner_class) {
                if tree.can_learn(&def, char_level).is_ok() {
                    return Some(sid);
                }
            }
        }
    }
    None
}

/// Toggle button for opening the "Débloquer une branche" sub-menu.
fn toggle_rect(viewport: [f32; 2]) -> Rect {
    let p = panel_rect(viewport);
    let x = p[0] + (p[2] - TOGGLE_BTN_W) * 0.5;
    let y = p[1] + p[3] - TOGGLE_BTN_H - 30.0;
    [x, y, TOGGLE_BTN_W, TOGGLE_BTN_H]
}

/// Hit-test the toggle button.
#[must_use]
pub fn toggle_at(viewport: [f32; 2], mouse: [f32; 2]) -> bool {
    point_in(toggle_rect(viewport), mouse)
}

/// Sub-menu rect: floats over the right edge of the main panel.
fn unlock_panel_rect(viewport: [f32; 2]) -> Rect {
    let p = panel_rect(viewport);
    let w = 320.0;
    let h = 380.0;
    [p[0] + p[2] + 12.0, p[1] + (p[3] - h) * 0.5, w, h]
}

fn unlock_row_rect(viewport: [f32; 2], i: usize) -> Rect {
    let p = unlock_panel_rect(viewport);
    #[allow(clippy::cast_precision_loss)]
    let y = p[1] + 50.0 + i as f32 * (ROW_H + ROW_PAD);
    [p[0] + 12.0, y, p[2] - 24.0, ROW_H]
}

fn unlock_btn_rect(row: Rect) -> Rect {
    [row[0] + row[2] - UNLOCK_BTN_W - 4.0, row[1] + (ROW_H - UNLOCK_BTN_H) * 0.5, UNLOCK_BTN_W, UNLOCK_BTN_H]
}

/// If the sub-menu is open and the cursor sits on an unlock-branch button,
/// return `(originating class, branch index)`.
#[must_use]
pub fn unlock_at(world: &World, viewport: [f32; 2], mouse: [f32; 2]) -> Option<(ClassType, u32)> {
    let player = world.player_entity?;
    let class = world.classes.get(&player).copied().unwrap_or(world.chosen_class);
    let candidates = lockable_branches(world, class);
    for (i, branch) in candidates.iter().enumerate() {
        let row = unlock_row_rect(viewport, i);
        if point_in(unlock_btn_rect(row), mouse) {
            return Some((branch.class, branch.index));
        }
    }
    None
}

/// True if `mouse` is inside the sub-menu's panel rect (used by `handle_left_press`
/// to swallow clicks).
#[must_use]
pub fn in_unlock_panel(viewport: [f32; 2], mouse: [f32; 2]) -> bool {
    point_in(unlock_panel_rect(viewport), mouse)
}

/// Spend one point unlocking a foreign branch. No-op if already owned or if
/// the player has no points. Returns `true` when the unlock succeeded.
pub fn unlock_branch(world: &mut World, class: ClassType, branch_index: u32) -> bool {
    let Some(player) = world.player_entity else { return false };
    let player_class = world.classes.get(&player).copied().unwrap_or(world.chosen_class);
    if class == player_class {
        return false;
    }
    // The point pool lives on the skill tree.
    let Some(tree) = world.skill_trees.get(&player) else { return false };
    if tree.available_points == 0 {
        return false;
    }
    let already = world
        .unlocked_branches
        .get(&player)
        .map(|v| v.iter().any(|(c, i)| *c == class && *i == branch_index))
        .unwrap_or(false);
    if already {
        return false;
    }
    let name = branches_for_class(class)
        .into_iter()
        .find(|b| b.index == branch_index)
        .map(|b| b.name)
        .unwrap_or_else(|| "?".to_string());
    if let Some(tree) = world.skill_trees.get_mut(&player) {
        tree.available_points -= 1;
    }
    world.unlocked_branches.entry(player).or_default().push((class, branch_index));
    world.log_event(format!("Branche débloquée : {name} ({:?}).", class));
    true
}

/// Spend a point on the given skill id.
pub fn invest(world: &mut World, id: SkillId) {
    let Some(player) = world.player_entity else { return };
    let char_level = world.progression.get(&player).map_or(1, |p| p.level());
    let owner_class = world.classes.get(&player).copied().unwrap_or(world.chosen_class);
    // Prefer the rich branch SkillDef when the id belongs to one — falls back
    // to the legacy mge stub list so tests using `class_skill_stubs` still
    // resolve.
    let def: Option<SkillDef> = skill_def_for(id, owner_class).or_else(|| {
        let class = world.skill_trees.get(&player)?.class;
        class_skill_stubs(class).into_iter().find(|d| d.id == id)
    });
    let Some(def) = def else { return };
    let result = world.skill_trees.get_mut(&player).map(|tree| {
        let ok = tree.invest(&def, char_level).is_ok();
        (ok, tree.level_of(id))
    });
    if let Some((true, rank)) = result {
        if let Some(slot) = world.skills.get_mut(&player) {
            if let Some(active) = slot.bar.iter_mut().find(|sk| sk.id == id) {
                // Already on the hotbar → just bump its rank.
                active.level = rank;
            } else if rank >= 1 && slot.bar.len() < HOTBAR_MAX {
                // First time learned → auto-place it on the hotbar.
                if let Some((_, mut active)) = active_skill_by_id(id) {
                    active.level = rank;
                    slot.bar.push(active);
                    slot.cooldowns.push(Cooldown::ready());
                }
            }
        }
        world.log_event(format!("Learned {} (rank {rank}).", def.name));
    }
}

/// The learned-skill id whose tree row is under `mouse` (excluding the `+`
/// button, handled separately). Used to (un)assign it on the hotbar.
#[must_use]
pub fn skill_row_at(world: &World, viewport: [f32; 2], mouse: [f32; 2]) -> Option<SkillId> {
    skill_rows(world, viewport)
        .into_iter()
        .find(|(row, _, _)| point_in(*row, mouse) && !point_in(plus_rect_from_row(*row), mouse))
        .map(|(_, sid, _)| sid)
}

/// Toggle a learned skill on/off the hotbar (customisable bar). Only skills the
/// player has actually learned (rank ≥ 1) can be placed.
pub fn toggle_on_hotbar(world: &mut World, id: SkillId) {
    let Some(player) = world.player_entity else { return };
    let rank = world.skill_trees.get(&player).map_or(0, |t| t.level_of(id));
    if rank == 0 {
        world.log_event("Apprends d'abord cette competence (+).");
        return;
    }
    let Some(slot) = world.skills.get_mut(&player) else { return };
    if let Some(pos) = slot.bar.iter().position(|sk| sk.id == id) {
        slot.bar.remove(pos);
        slot.cooldowns.remove(pos);
        if slot.selected >= slot.bar.len() {
            slot.selected = slot.bar.len().saturating_sub(1);
        }
        world.log_event("Retiree de la barre.");
    } else if slot.bar.len() < HOTBAR_MAX {
        if let Some((_, mut active)) = active_skill_by_id(id) {
            active.level = rank;
            slot.bar.push(active);
            slot.cooldowns.push(Cooldown::ready());
            world.log_event("Ajoutee a la barre.");
        }
    } else {
        world.log_event("Barre pleine (9 max).");
    }
}

/// Draw the interactive skill tree.
#[allow(clippy::cast_precision_loss, clippy::too_many_lines)]
pub fn draw(batch: &mut SpriteBatch, world: &World, ui: &super::UiState, viewport: [f32; 2], mouse: [f32; 2]) {
    let Some(player) = world.player_entity else { return };
    let Some(tree) = world.skill_trees.get(&player) else { return };
    let char_level = world.progression.get(&player).map_or(1, |p| p.level());
    let owner_class = world.classes.get(&player).copied().unwrap_or(world.chosen_class);

    let prect = panel_rect(viewport);
    panel(batch, prect, b"SKILLS");

    // Available points header.
    let (pts, pn) = font::fmt_u32(tree.available_points);
    let mut p = Vec::new();
    p.extend_from_slice(b"POINTS DISPO ");
    p.extend_from_slice(&pts[..pn]);
    font::draw_text(batch, &p, prect[0] + 14.0, prect[1] + 28.0, 1.6, vertex::XP, RenderLayer::UiScreen, 0, SUB_TEXT);

    // Walk the visible branches: section header + each skill row.
    let mut y = prect[1] + 50.0;
    let x = prect[0] + 16.0;
    for branch in visible_branches(world, owner_class) {
        // Section header strip tinted with the branch's owning-class colour.
        let header_color = class_color(branch.class);
        fill(batch, [x, y, ROW_W, HEADER_H], SUB_CONTENT, [0.10, 0.10, 0.14, 0.95]);
        fill(batch, [x, y, 4.0, HEADER_H], SUB_CONTENT + 1, header_color);
        let mut header_text = branch.name.clone().into_bytes();
        if branch.class != owner_class {
            // Foreign branch: prepend the originating class' display name.
            let mut name = format!(" ({})", branch.class.display_name()).into_bytes();
            header_text.append(&mut name);
        }
        font::draw_text(batch, &header_text, x + 12.0, y + 4.0, 1.5, vertex::WHITE, RenderLayer::UiScreen, 0, SUB_TEXT);
        y += HEADER_H + 2.0;

        for (idx, &sid) in branch.skill_ids.iter().enumerate() {
            let row = [x, y, ROW_W, ROW_H];

            // Vertical connector from the previous skill of the branch to this
            // one — drawn in the gap before the row body so the row paints on
            // top of nothing.
            if idx > 0 {
                fill(batch, [row[0] + 12.0, y - ROW_PAD, 2.0, ROW_PAD], SUB_CONTENT, [0.35, 0.35, 0.40, 1.0]);
            }

            let lvl = tree.level_of(sid);
            let def_for_check = skill_def_for(sid, owner_class);
            let prereq_met = prerequisite_for(sid, owner_class)
                .map_or(true, |prev| tree.level_of(prev) >= 1);
            let locked = !prereq_met && lvl == 0;
            let learnable = def_for_check
                .as_ref()
                .map(|def| tree.can_learn(def, char_level).is_ok())
                .unwrap_or(false);

            let row_bg = if locked {
                [0.05, 0.05, 0.07, 1.0]
            } else if point_in(row, mouse) {
                [0.18, 0.16, 0.10, 1.0]
            } else {
                [0.07, 0.07, 0.10, 1.0]
            };
            fill(batch, row, SUB_CONTENT, row_bg);
            if branch.class != owner_class {
                fill(batch, [x, y, 3.0, ROW_H], SUB_CONTENT + 1, header_color);
            }

            // Left of the name: a padlock when the prerequisite isn't met, or
            // the skill's element glyph otherwise. Both push the name right.
            let name_x = if locked {
                let lx = row[0] + 8.0;
                let ly = row[1] + 6.0;
                // Body of the padlock.
                fill(batch, [lx, ly + 4.0, 10.0, 8.0], SUB_CONTENT + 2, [0.55, 0.45, 0.20, 1.0]);
                // Shackle (two vertical bars + top bar).
                fill(batch, [lx + 1.0, ly, 2.0, 5.0], SUB_CONTENT + 2, [0.55, 0.45, 0.20, 1.0]);
                fill(batch, [lx + 7.0, ly, 2.0, 5.0], SUB_CONTENT + 2, [0.55, 0.45, 0.20, 1.0]);
                fill(batch, [lx + 1.0, ly, 8.0, 2.0], SUB_CONTENT + 2, [0.55, 0.45, 0.20, 1.0]);
                // Keyhole.
                fill(batch, [lx + 4.0, ly + 7.0, 2.0, 3.0], SUB_CONTENT + 3, [0.10, 0.10, 0.12, 1.0]);
                row[0] + 24.0
            } else if let Some((_, active)) = active_skill_by_id(sid) {
                let dim = if lvl == 0 { 0.55 } else { 1.0 };
                skill_icon::draw_skill_glyph(batch, active.fx, row[0] + 15.0, row[1] + ROW_H * 0.5, 17.0, RenderLayer::UiScreen, 0, SUB_CONTENT + 2, dim);
                row[0] + 28.0
            } else {
                row[0] + 10.0
            };

            let name_color = if locked { [0.45, 0.45, 0.50, 1.0] } else { vertex::WHITE };
            let name = def_for_check.as_ref().map(|d| d.name.as_bytes().to_vec()).unwrap_or_default();
            font::draw_text(batch, &name, name_x, row[1] + 4.0, 1.4, name_color, RenderLayer::UiScreen, 0, SUB_TEXT);

            // RANK x/y mid-row.
            let (cur, cn) = font::fmt_u32(lvl);
            let (max, mn) = font::fmt_u32(def_for_check.as_ref().map(|d| d.max_level).unwrap_or(0));
            let mut rank = Vec::new();
            rank.extend_from_slice(&cur[..cn]);
            rank.push(b'/');
            rank.extend_from_slice(&max[..mn]);
            let rank_color = if locked { [0.40, 0.40, 0.45, 1.0] } else { [0.8, 0.8, 0.85, 1.0] };
            font::draw_text(batch, &rank, row[0] + ROW_W - PLUS_W - 70.0, row[1] + 4.0, 1.3, rank_color, RenderLayer::UiScreen, 0, SUB_TEXT);

            // "+" button.
            let pr = plus_rect_from_row(row);
            let hover = point_in(pr, mouse) && learnable;
            let (fill_c, ring_c) = if learnable {
                ([0.10, 0.22, 0.10, 1.0], if hover { vertex::WHITE } else { [0.45, 0.85, 0.45, 1.0] })
            } else if locked {
                ([0.08, 0.08, 0.10, 1.0], [0.25, 0.20, 0.10, 1.0])
            } else {
                ([0.12, 0.12, 0.14, 1.0], [0.30, 0.30, 0.34, 1.0])
            };
            fill(batch, pr, SUB_CONTENT + 2, fill_c);
            fill(batch, [pr[0], pr[1], pr[2], 1.0], SUB_CONTENT + 3, ring_c);
            fill(batch, [pr[0], pr[1] + pr[3] - 1.0, pr[2], 1.0], SUB_CONTENT + 3, ring_c);
            fill(batch, [pr[0], pr[1], 1.0, pr[3]], SUB_CONTENT + 3, ring_c);
            fill(batch, [pr[0] + pr[2] - 1.0, pr[1], 1.0, pr[3]], SUB_CONTENT + 3, ring_c);
            font::draw_text(batch, b"+", pr[0] + 6.0, pr[1] + 3.0, 1.7, if learnable { vertex::WHITE } else { [0.45, 0.45, 0.50, 1.0] }, RenderLayer::UiScreen, 0, SUB_TEXT);

            y += ROW_H + ROW_PAD;
        }
        y += SECTION_GAP;
    }

    // Toggle button at the bottom.
    let tr = toggle_rect(viewport);
    let tr_hover = point_in(tr, mouse);
    fill(batch, tr, SUB_CONTENT + 2, if tr_hover { [0.22, 0.18, 0.10, 1.0] } else { [0.10, 0.10, 0.14, 1.0] });
    let label: &[u8] = if ui.show_skills_unlock { b"FERMER LE MENU" } else { b"DEBLOQUER UNE BRANCHE" };
    font::draw_text(batch, label, tr[0] + 18.0, tr[1] + 4.0, 1.5, vertex::WHITE, RenderLayer::UiScreen, 0, SUB_TEXT);

    if ui.show_skills_unlock {
        draw_unlock_panel(batch, world, owner_class, viewport, mouse, tree.available_points);
    }
}

#[allow(clippy::cast_precision_loss)]
fn draw_unlock_panel(
    batch: &mut SpriteBatch,
    world: &World,
    owner_class: ClassType,
    viewport: [f32; 2],
    mouse: [f32; 2],
    available_points: u32,
) {
    let r = unlock_panel_rect(viewport);
    panel(batch, r, b"DEBLOQUER");
    let candidates = lockable_branches(world, owner_class);
    let unlocked = world.player_entity.and_then(|p| world.unlocked_branches.get(&p)).cloned().unwrap_or_default();
    let mut i = 0_usize;
    for branch in candidates {
        let row = unlock_row_rect(viewport, i);
        let class_c = class_color(branch.class);
        fill(batch, row, SUB_CONTENT, [0.07, 0.07, 0.10, 1.0]);
        fill(batch, [row[0], row[1], 3.0, row[3]], SUB_CONTENT + 1, class_c);
        let label_text = format!("{} ({})", branch.name, branch.class.display_name());
        font::draw_text(batch, label_text.as_bytes(), row[0] + 12.0, row[1] + 4.0, 1.4, vertex::WHITE, RenderLayer::UiScreen, 0, SUB_TEXT);

        let bt = unlock_btn_rect(row);
        let can_buy = available_points > 0;
        let hover = point_in(bt, mouse) && can_buy;
        let (fill_c, ring_c) = if can_buy {
            ([0.10, 0.22, 0.10, 1.0], if hover { vertex::WHITE } else { [0.45, 0.85, 0.45, 1.0] })
        } else {
            ([0.12, 0.12, 0.14, 1.0], [0.30, 0.30, 0.34, 1.0])
        };
        fill(batch, bt, SUB_CONTENT + 2, fill_c);
        fill(batch, [bt[0], bt[1], bt[2], 1.0], SUB_CONTENT + 3, ring_c);
        fill(batch, [bt[0], bt[1] + bt[3] - 1.0, bt[2], 1.0], SUB_CONTENT + 3, ring_c);
        fill(batch, [bt[0], bt[1], 1.0, bt[3]], SUB_CONTENT + 3, ring_c);
        fill(batch, [bt[0] + bt[2] - 1.0, bt[1], 1.0, bt[3]], SUB_CONTENT + 3, ring_c);
        font::draw_text(batch, b"1 pt", bt[0] + 22.0, bt[1] + 3.0, 1.3, vertex::WHITE, RenderLayer::UiScreen, 0, SUB_TEXT);
        i += 1;
    }
    // Footer listing what's already owned.
    let mut footer = b"Possedees: ".to_vec();
    if unlocked.is_empty() {
        footer.extend_from_slice(b"(aucune)");
    } else {
        for (c, idx) in &unlocked {
            footer.push(b' ');
            if let Some(b) = branches_for_class(*c).into_iter().find(|b| b.index == *idx) {
                footer.extend_from_slice(b.name.as_bytes());
            }
        }
    }
    font::draw_text(batch, &footer, r[0] + 12.0, r[1] + r[3] - 26.0, 1.2, [0.8, 0.8, 0.85, 1.0], RenderLayer::UiScreen, 0, SUB_TEXT);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::player::ClassType;
    use crate::data::maps::village_map;
    use crate::data::skills::branches_for_class;
    use crate::data::GameData;
    use crate::plugins::player::spawn_player_system;

    fn world_with_player() -> World {
        let mut world = World::new(village_map(), GameData::load(), 1);
        world.chosen_class = ClassType::Soldat;
        spawn_player_system(&mut world);
        world
    }

    /// First skill id of the player's first native branch — the canonical
    /// "first node" in the new branch-based tree.
    fn first_native_skill_id(world: &World) -> SkillId {
        let class = world.classes.get(&world.player_entity.unwrap()).copied().unwrap();
        branches_for_class(class)[0].skill_ids[0]
    }

    #[test]
    fn invest_consumes_a_point_and_lifts_skill_level() {
        let mut world = world_with_player();
        let p = world.player_entity.unwrap();
        world.skill_trees.get_mut(&p).unwrap().add_points(2);

        let id = first_native_skill_id(&world);
        let before = world.skill_trees.get(&p).unwrap().available_points;
        invest(&mut world, id);
        let after = world.skill_trees.get(&p).unwrap().available_points;

        assert_eq!(before - after, 1, "one click spends exactly one point");
        assert_eq!(world.skill_trees.get(&p).unwrap().level_of(id), 1);
    }

    #[test]
    fn cannot_invest_with_no_points() {
        let mut world = world_with_player();
        let p = world.player_entity.unwrap();
        let id = first_native_skill_id(&world);
        // spawn_player_system pre-loads 1 point — burn it first.
        invest(&mut world, id);
        let rank_before = world.skill_trees.get(&p).unwrap().level_of(id);
        assert_eq!(world.skill_trees.get(&p).unwrap().available_points, 0);

        invest(&mut world, id);
        assert_eq!(world.skill_trees.get(&p).unwrap().level_of(id), rank_before);
    }

    #[test]
    fn invest_syncs_hotbar_active_skill_level() {
        // The hotbar `ActiveSkill.level` is initialised to 1 by the spawn data
        // while the `SkillTree` rank starts at 0. Investing rewrites the
        // hotbar to mirror the new rank, so investing twice in a row should
        // leave the hotbar reading 2.
        let mut world = world_with_player();
        let p = world.player_entity.unwrap();
        world.skill_trees.get_mut(&p).unwrap().add_points(3);

        let id = first_native_skill_id(&world);
        invest(&mut world, id);
        invest(&mut world, id);
        let rank = world.skill_trees.get(&p).unwrap().level_of(id);
        let hotbar = world
            .skills
            .get(&p)
            .and_then(|s| s.bar.iter().find(|sk| sk.id == id).map(|sk| sk.level))
            .unwrap();
        assert_eq!(rank, 2);
        assert_eq!(hotbar, rank, "hotbar ActiveSkill.level must mirror tree rank");
    }

    #[test]
    fn unlocking_a_foreign_branch_costs_one_point() {
        let mut world = world_with_player();
        let p = world.player_entity.unwrap();
        world.skill_trees.get_mut(&p).unwrap().add_points(2);
        let before = world.skill_trees.get(&p).unwrap().available_points;
        // Soldat unlocks Mage "Feu" (index 0).
        assert!(unlock_branch(&mut world, ClassType::MageDeGuerre, 0));
        let after = world.skill_trees.get(&p).unwrap().available_points;
        assert_eq!(before - after, 1, "unlocking costs exactly 1 point");
        let unlocked = world.unlocked_branches.get(&p).cloned().unwrap_or_default();
        assert!(unlocked.contains(&(ClassType::MageDeGuerre, 0)));
    }

    #[test]
    fn unlocking_without_points_fails() {
        let mut world = world_with_player();
        let p = world.player_entity.unwrap();
        // Spend the single starter point first.
        let id = first_native_skill_id(&world);
        invest(&mut world, id);
        assert_eq!(world.skill_trees.get(&p).unwrap().available_points, 0);

        assert!(!unlock_branch(&mut world, ClassType::MageDeGuerre, 0));
        assert!(world.unlocked_branches.get(&p).cloned().unwrap_or_default().is_empty());
    }

    #[test]
    fn unlocking_native_branch_is_a_noop() {
        let mut world = world_with_player();
        let p = world.player_entity.unwrap();
        world.skill_trees.get_mut(&p).unwrap().add_points(3);
        // Soldat is the player's own class — its branches are implicit.
        assert!(!unlock_branch(&mut world, ClassType::Soldat, 0));
        assert_eq!(world.skill_trees.get(&p).unwrap().available_points, 4);
    }

    #[test]
    fn after_unlock_foreign_skill_is_investable() {
        let mut world = world_with_player();
        let p = world.player_entity.unwrap();
        // Need at least 2 points: 1 to unlock, 1 to invest.
        world.skill_trees.get_mut(&p).unwrap().add_points(2);
        assert!(unlock_branch(&mut world, ClassType::MageDeGuerre, 0));
        // Boule de feu (3001) is the entry of the Mage "Feu" branch.
        invest(&mut world, 3001);
        assert_eq!(world.skill_trees.get(&p).unwrap().level_of(3001), 1);
    }

    #[test]
    fn cannot_unlock_the_same_branch_twice() {
        let mut world = world_with_player();
        let p = world.player_entity.unwrap();
        world.skill_trees.get_mut(&p).unwrap().add_points(5);
        assert!(unlock_branch(&mut world, ClassType::MageDeGuerre, 0));
        let after_first = world.skill_trees.get(&p).unwrap().available_points;
        assert!(!unlock_branch(&mut world, ClassType::MageDeGuerre, 0));
        let after_second = world.skill_trees.get(&p).unwrap().available_points;
        assert_eq!(after_first, after_second, "double-unlock must not spend a point");
    }

    #[test]
    fn first_skill_of_branch_is_always_investable() {
        // No prior investment, no foreign unlock needed: 1001 (Grand coup) must
        // accept a point straight away.
        let mut world = world_with_player();
        let p = world.player_entity.unwrap();
        // spawn gives 1 starter point, that's enough.
        invest(&mut world, 1001);
        assert_eq!(world.skill_trees.get(&p).unwrap().level_of(1001), 1);
    }

    #[test]
    fn cannot_invest_second_skill_when_first_at_zero() {
        // Taille (1002) sits behind Grand coup (1001) in the Combat branch and
        // must reject investment while 1001 is still at rank 0.
        let mut world = world_with_player();
        let p = world.player_entity.unwrap();
        world.skill_trees.get_mut(&p).unwrap().add_points(5);
        let pts_before = world.skill_trees.get(&p).unwrap().available_points;
        invest(&mut world, 1002);
        assert_eq!(world.skill_trees.get(&p).unwrap().level_of(1002), 0, "blocked by prereq");
        assert_eq!(
            world.skill_trees.get(&p).unwrap().available_points,
            pts_before,
            "rejected invest must not spend a point"
        );
    }

    #[test]
    fn investing_first_unlocks_second_in_branch() {
        let mut world = world_with_player();
        let p = world.player_entity.unwrap();
        world.skill_trees.get_mut(&p).unwrap().add_points(3);
        invest(&mut world, 1001);
        // 1002 should now accept its first point.
        invest(&mut world, 1002);
        assert_eq!(world.skill_trees.get(&p).unwrap().level_of(1002), 1);
    }

    #[test]
    fn third_skill_locked_until_second_invested() {
        let mut world = world_with_player();
        let p = world.player_entity.unwrap();
        world.skill_trees.get_mut(&p).unwrap().add_points(5);
        invest(&mut world, 1001);
        // 1003 (Estocs) still needs 1002 (Taille) at >= 1.
        invest(&mut world, 1003);
        assert_eq!(world.skill_trees.get(&p).unwrap().level_of(1003), 0, "blocked: needs 1002");
        invest(&mut world, 1002);
        invest(&mut world, 1003);
        assert_eq!(world.skill_trees.get(&p).unwrap().level_of(1003), 1);
    }

    #[test]
    fn plus_at_returns_none_for_locked_skill() {
        // The "+" hitbox must refuse a click on a locked node so the UI matches
        // the system gate.
        let mut world = world_with_player();
        let p = world.player_entity.unwrap();
        world.skill_trees.get_mut(&p).unwrap().add_points(3);
        // Build the row layout and pick the centre of the "+" of skill 1002.
        let viewport = [1280.0, 720.0];
        let rows = skill_rows(&world, viewport);
        let (row_1002, _, _) = rows
            .iter()
            .find(|(_, sid, _)| *sid == 1002)
            .copied()
            .expect("Taille row present");
        let pr = plus_rect_from_row(row_1002);
        let mouse = [pr[0] + pr[2] * 0.5, pr[1] + pr[3] * 0.5];
        // 1001 not yet invested ⇒ 1002 locked ⇒ plus_at must say None.
        assert_eq!(plus_at(&world, viewport, mouse), None);
        // After investing 1001, plus_at must accept the click on 1002.
        invest(&mut world, 1001);
        let _ = p; // keep binding alive for borrow clarity
        let rows = skill_rows(&world, viewport);
        let (row_1002, _, _) = rows
            .iter()
            .find(|(_, sid, _)| *sid == 1002)
            .copied()
            .expect("Taille row still present");
        let pr = plus_rect_from_row(row_1002);
        let mouse = [pr[0] + pr[2] * 0.5, pr[1] + pr[3] * 0.5];
        assert_eq!(plus_at(&world, viewport, mouse), Some(1002));
    }

    #[test]
    fn save_round_trip_preserves_unlocked_branches() {
        use crate::save::{apply_to_world, snapshot};
        let mut world = world_with_player();
        let p = world.player_entity.unwrap();
        world.skill_trees.get_mut(&p).unwrap().add_points(3);
        assert!(unlock_branch(&mut world, ClassType::MageDeGuerre, 0));
        let snap = snapshot(&world, "Hero", "pid", 0).expect("snapshot");
        assert!(snap.unlocked_branches.iter().any(|(c, i)| c == "mage" && *i == 0));

        let mut world2 = World::new(village_map(), GameData::load(), 1);
        world2.chosen_class = ClassType::Soldat;
        spawn_player_system(&mut world2);
        apply_to_world(&mut world2, &snap);
        let p2 = world2.player_entity.unwrap();
        let restored = world2.unlocked_branches.get(&p2).cloned().unwrap_or_default();
        assert!(restored.contains(&(ClassType::MageDeGuerre, 0)));
    }
}
