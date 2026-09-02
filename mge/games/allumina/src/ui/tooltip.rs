//! @id     games_allumina_ui_tooltip
//! @do     definir_white
//! @role   ui
//! @layer  core
//! @human  games allumina — module « ui tooltip ».

//! Diablo-style item tooltip.
//!
//! `item_tooltip` is the text version (headless); `draw` paints a bordered box
//! near the cursor with sectioned, colour-coded stats: a rarity-tinted header
//! carrying the item sprite (the same rarity-aware icon as the inventory grid)
//! beside the name/type, then weapon damage + DPS, armour + damage-reduction,
//! base attributes, rolled affixes (each tinted by its element/role), a diff
//! against the currently equipped item, and a footer with the level/attribute
//! requirements and the gold value.
//!
//! The bitmap font (`render::font`) is a 5x7 ASCII-only glyph sheet: it has no
//! accented letters and no emoji, so every label is plain ASCII and the *type*
//! of a stat is carried by colour (fire = orange, cold = cyan, lightning =
//! yellow, poison = green, leech = red, etc.). `mge-gameplay` has no elemental
//! *weapon-damage* stat (only resistances) and no item class-restriction field,
//! so weapon damage is shown as physical and there is no class-requirement row.

use mge_gameplay::stats::{Modifier, StatKind};
use mge_items::item::ItemFamily;
use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::components::item::ItemInstance;
use crate::render::font;
use crate::render::vertex::Color;
use crate::render::{item_icon, shapes};

use super::{border, fill, SUB_TOOLTIP};

const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
const GREY: Color = [0.70, 0.70, 0.75, 1.0];
/// Flavour text (italic, dimmer than GREY).
const FLAVOUR: Color = [0.55, 0.55, 0.62, 1.0];
/// Unique fixed-mod line — same golden as the rarity colour, distinct from
/// the blue used for rolled affixes.
const UNIQUE_GOLD: Color = [1.0, 0.843, 0.0, 1.0];
/// Generic affix line — #6888ff.
const AFFIX_BLUE: Color = [0.408, 0.533, 1.0, 1.0];
/// Comparison: this item is better — #44ff44.
const BETTER: Color = [0.267, 1.0, 0.267, 1.0];
/// Comparison: this item is worse / unmet requirement — #ff4444.
const WORSE: Color = [1.0, 0.267, 0.267, 1.0];
/// Gold value — #ffd700.
const GOLD_TEXT: Color = [1.0, 0.843, 0.0, 1.0];
/// Box background — black @ 88%.
const BG: Color = [0.0, 0.0, 0.0, 0.88];
/// Thin section separator.
const SEP_COLOR: Color = [0.40, 0.40, 0.45, 0.6];

// ── Per-type accent colours ──────────────────────────────────────────────────
const PHYS: Color = [0.86, 0.82, 0.74, 1.0];
const FIRE: Color = [1.0, 0.45, 0.15, 1.0];
const COLD: Color = [0.45, 0.80, 1.0, 1.0];
const LIGHTNING: Color = [1.0, 0.92, 0.35, 1.0];
const POISON: Color = [0.45, 0.85, 0.35, 1.0];
const LEECH: Color = [0.90, 0.30, 0.30, 1.0];
const MANA_C: Color = [0.40, 0.60, 0.95, 1.0];
const SPEED_C: Color = [0.55, 0.88, 0.95, 1.0];

/// Map a `Rarity::color_name` to the tooltip's rarity tint (header + border).
#[must_use]
fn rarity_color(name: &str) -> Color {
    match name {
        "blue" => AFFIX_BLUE,                // Magic
        "yellow" => GOLD_TEXT,               // Rare
        "orange" | "gold" => [1.0, 0.549, 0.0, 1.0], // Crafted / Unique — #ff8c00
        "green" => [0.30, 0.85, 0.35, 1.0],  // Set
        _ => WHITE,                          // Normal
    }
}

/// Accent colour for a bonus stat, so the player reads its element/role at a
/// glance even though the font has no icons.
#[must_use]
fn affix_color(stat: StatKind) -> Color {
    match stat {
        StatKind::FireResist => FIRE,
        StatKind::ColdResist => COLD,
        StatKind::LightningResist => LIGHTNING,
        StatKind::PoisonResist => POISON,
        StatKind::CritChance => GOLD_TEXT,
        StatKind::LifeOnHit | StatKind::Life => LEECH,
        StatKind::ManaOnKill | StatKind::Mana => MANA_C,
        StatKind::AttackSpeed | StatKind::CastSpeed | StatKind::MovementSpeed => SPEED_C,
        _ => AFFIX_BLUE,
    }
}

/// One painted row: a coloured text line or a thin horizontal rule.
enum Seg {
    Text(String, Color),
    Sep,
}

/// Sum every rolled mod (prefixes + suffixes + fixed unique mods) per stat,
/// keeping flat and percentage parts separate.
fn collect_mods(item: &ItemInstance) -> Vec<(StatKind, i32, i32)> {
    let mut out: Vec<(StatKind, i32, i32)> = Vec::new();
    for a in item.affixes.prefixes.iter().chain(item.affixes.suffixes.iter()) {
        for m in &a.mods {
            accumulate(&mut out, m.stat, m.modifier);
        }
    }
    for m in &item.extra_mods {
        accumulate(&mut out, m.stat, m.modifier);
    }
    // Inserted gems contribute their context-dependent stats too.
    let host = crate::components::item::gem_host(item.item.base.kind);
    for gem in &item.socketed {
        for m in gem.stat_mods(host) {
            accumulate(&mut out, m.stat, m.modifier);
        }
    }
    out
}

fn accumulate(out: &mut Vec<(StatKind, i32, i32)>, stat: StatKind, m: Modifier) {
    let (f, p) = match m {
        Modifier::Flat(v) => (v, 0),
        Modifier::Percentage(v) => (0, v),
    };
    if let Some(e) = out.iter_mut().find(|e| e.0 == stat) {
        e.1 += f;
        e.2 += p;
    } else {
        out.push((stat, f, p));
    }
}

fn stat_flat(mods: &[(StatKind, i32, i32)], stat: StatKind) -> i32 {
    mods.iter().find(|e| e.0 == stat).map_or(0, |e| e.1)
}

fn stat_pct(mods: &[(StatKind, i32, i32)], stat: StatKind) -> i32 {
    mods.iter().find(|e| e.0 == stat).map_or(0, |e| e.2)
}

#[allow(clippy::cast_precision_loss)]
fn avg_damage(mods: &[(StatKind, i32, i32)]) -> f32 {
    (stat_flat(mods, StatKind::DamageMin) + stat_flat(mods, StatKind::DamageMax)) as f32 / 2.0
}

/// Damage-per-second: average physical damage scaled by the attack-speed bonus.
#[allow(clippy::cast_precision_loss)]
fn dps(mods: &[(StatKind, i32, i32)]) -> f32 {
    avg_damage(mods) * (1.0 + stat_pct(mods, StatKind::AttackSpeed) as f32 / 100.0)
}

/// Total elemental resistance (sum of the four flats), for the comparison row.
fn total_resist(mods: &[(StatKind, i32, i32)]) -> i32 {
    stat_flat(mods, StatKind::FireResist)
        + stat_flat(mods, StatKind::ColdResist)
        + stat_flat(mods, StatKind::LightningResist)
        + stat_flat(mods, StatKind::PoisonResist)
}

/// Format one affix row. Returns `None` for stats shown in dedicated sections
/// (damage, defence, base attributes) or with no value.
fn affix_line(stat: StatKind, flat: i32, pct: i32) -> Option<String> {
    let v = if flat != 0 { flat } else { pct };
    if v == 0 {
        return None;
    }
    let s = match stat {
        StatKind::AttackSpeed => format!("{v:+}% Vitesse d'attaque"),
        StatKind::CastSpeed => format!("{v:+}% Vitesse d'incantation"),
        StatKind::MovementSpeed => format!("{v:+}% Vitesse de deplacement"),
        StatKind::CritChance => format!("{v:+}% Chance de critique"),
        StatKind::FireResist => format!("{v:+}% Resist Feu"),
        StatKind::ColdResist => format!("{v:+}% Resist Glace"),
        StatKind::LightningResist => format!("{v:+}% Resist Foudre"),
        StatKind::PoisonResist => format!("{v:+}% Resist Poison"),
        StatKind::BlockChance => format!("{v:+}% Blocage"),
        StatKind::MagicFind => format!("{v:+}% Chance magique"),
        StatKind::GoldFind => format!("{v:+}% Or trouve"),
        StatKind::Life => format!("{v:+} Vie"),
        StatKind::Mana => format!("{v:+} Mana"),
        StatKind::Stamina => format!("{v:+} Endurance"),
        StatKind::AttackRating => format!("{v:+} Precision"),
        StatKind::LifeOnHit => format!("{v:+} Vol de vie"),
        StatKind::ManaOnKill => format!("{v:+} Vol de mana"),
        _ => return None,
    };
    Some(s)
}

/// A `^ +N`/`v -N` comparison row, or `None` when there is no difference.
fn cmp_line(label: &str, diff: i32) -> Option<Seg> {
    if diff == 0 {
        return None;
    }
    // The 5x7 font has no arrow glyphs, so ^ / v stand in for up / down.
    let (arrow, color) = if diff > 0 { ("^", BETTER) } else { ("v", WORSE) };
    Some(Seg::Text(format!("{arrow} {diff:+} {label}"), color))
}

/// Drop accents so labels stay inside the 5x7 ASCII font (item/affix names are
/// already ASCII; this is a safety net).
fn sanitize(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'à' | 'â' | 'ä' => 'a',
            'î' | 'ï' => 'i',
            'ô' | 'ö' => 'o',
            'û' | 'ü' | 'ù' => 'u',
            'ç' => 'c',
            'É' | 'È' | 'Ê' => 'E',
            other => other,
        })
        .collect()
}

/// Player context used for requirement colouring and comparisons.
#[derive(Debug, Clone, Copy)]
pub struct PlayerCtx {
    pub level: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
}

/// Build the ordered, coloured rows for `item`, diffing against `compare`
/// (the item the player would replace) when provided.
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::too_many_lines)]
fn build(item: &ItemInstance, compare: Option<&ItemInstance>, ctx: PlayerCtx) -> Vec<Seg> {
    let mut segs: Vec<Seg> = Vec::new();
    let rc = rarity_color(item.rarity().color_name());
    let is_unique = matches!(item.rarity(), mge_items::rarity::Rarity::Unique);

    // ── Header: name (rarity colour) + base type (grey) ─────────────────────
    segs.push(Seg::Text(sanitize(&item.name), rc));
    segs.push(Seg::Text(sanitize(&item.item.base.name), GREY));

    // ── Prefix words (top), each on its own line ────────────────────────────
    if !item.prefix_words.is_empty() {
        segs.push(Seg::Sep);
        for word in &item.prefix_words {
            segs.push(Seg::Text(sanitize(word), AFFIX_BLUE));
        }
    }

    let mods = collect_mods(item);
    let family = item.item.base.kind.family();
    let lvl = ctx.level.max(1);

    // ── Damage (weapons): physical type, colour-coded, with DPS ─────────────
    if matches!(family, ItemFamily::Weapon) {
        let dmin = stat_flat(&mods, StatKind::DamageMin);
        let dmax = stat_flat(&mods, StatKind::DamageMax);
        if dmin > 0 || dmax > 0 {
            segs.push(Seg::Sep);
            segs.push(Seg::Text(format!("Degats Physiques: {dmin}-{dmax}"), PHYS));
            let aspd = stat_pct(&mods, StatKind::AttackSpeed);
            if aspd != 0 {
                segs.push(Seg::Text(format!("Vitesse d'attaque: {aspd:+}%"), SPEED_C));
            }
            segs.push(Seg::Text(format!("DPS: {:.1}", dps(&mods)), WHITE));
        }
    }

    // ── Defence (armour) + damage reduction ─────────────────────────────────
    if matches!(family, ItemFamily::Armor) {
        let def = stat_flat(&mods, StatKind::Defense);
        if def > 0 {
            segs.push(Seg::Sep);
            segs.push(Seg::Text(format!("Armure: {def}"), PHYS));
            let reduction = def as f32 / (def as f32 + 50.0 * lvl as f32) * 100.0;
            segs.push(Seg::Text(format!("Reduction degats: {}%", reduction.round() as i32), GREY));
        }
    }

    // ── Base attributes (white) ─────────────────────────────────────────────
    let base_attrs = [
        (StatKind::Strength, "Force"),
        (StatKind::Dexterity, "Dexterite"),
        (StatKind::Vitality, "Vitalite"),
        (StatKind::Energy, "Energie"),
    ];
    let mut attr_lines: Vec<Seg> = Vec::new();
    for (stat, label) in base_attrs {
        let v = stat_flat(&mods, stat);
        if v != 0 {
            attr_lines.push(Seg::Text(format!("{v:+} {label}"), WHITE));
        }
    }
    if !attr_lines.is_empty() {
        segs.push(Seg::Sep);
        segs.extend(attr_lines);
    }

    // ── Bonuses: every other stat, each tinted by its element/role ──────────
    let shown = [
        StatKind::DamageMin,
        StatKind::DamageMax,
        StatKind::Defense,
        StatKind::AttackSpeed, // already shown in the weapon block
        StatKind::Strength,
        StatKind::Dexterity,
        StatKind::Vitality,
        StatKind::Energy,
    ];
    let mut affix_lines: Vec<Seg> = Vec::new();
    for &(stat, flat, pct) in &mods {
        if shown.contains(&stat) {
            continue;
        }
        if let Some(text) = affix_line(stat, flat, pct) {
            // Uniques: every fixed stat is rendered in the same gold to
            // distinguish them from the blue rolled affixes on rare items.
            let color = if is_unique { UNIQUE_GOLD } else { affix_color(stat) };
            affix_lines.push(Seg::Text(text, color));
        }
    }
    if !affix_lines.is_empty() {
        segs.push(Seg::Sep);
        segs.extend(affix_lines);
    }

    // ── Comparison vs. the equipped item in the same slot ───────────────────
    if let Some(cmp) = compare {
        let cmods = collect_mods(cmp);
        let mut cmp_lines: Vec<Seg> = Vec::new();
        if matches!(family, ItemFamily::Weapon) {
            cmp_lines.extend(cmp_line("DPS", (dps(&mods) - dps(&cmods)).round() as i32));
        }
        if matches!(family, ItemFamily::Armor) {
            cmp_lines.extend(cmp_line("Armure", stat_flat(&mods, StatKind::Defense) - stat_flat(&cmods, StatKind::Defense)));
        }
        cmp_lines.extend(cmp_line("Vie", stat_flat(&mods, StatKind::Life) - stat_flat(&cmods, StatKind::Life)));
        cmp_lines.extend(cmp_line("Crit%", stat_pct(&mods, StatKind::CritChance) - stat_pct(&cmods, StatKind::CritChance)));
        cmp_lines.extend(cmp_line("Resist tot.", total_resist(&mods) - total_resist(&cmods)));
        if !cmp_lines.is_empty() {
            segs.push(Seg::Sep);
            segs.push(Seg::Text("Compare a l'equipe:".into(), GREY));
            segs.extend(cmp_lines);
        }
    }

    // ── Suffix words (bottom), each on its own line ─────────────────────────
    if !item.suffix_words.is_empty() {
        segs.push(Seg::Sep);
        for word in &item.suffix_words {
            segs.push(Seg::Text(sanitize(word), AFFIX_BLUE));
        }
    }

    // ── Footer: level + attribute requirements, then gold value ─────────────
    let req = &item.item.base.requirements;
    if req.min_level > 0
        || req.min_strength > 0
        || req.min_dexterity > 0
        || req.min_intelligence > 0
    {
        segs.push(Seg::Sep);
        if req.min_level > 0 {
            let color = if ctx.level < req.min_level { WORSE } else { GREY };
            segs.push(Seg::Text(format!("Niveau requis: {}", req.min_level), color));
        }
        if req.min_strength > 0 {
            let color = if ctx.strength < req.min_strength { WORSE } else { GREY };
            segs.push(Seg::Text(format!("Force requise: {}", req.min_strength), color));
        }
        if req.min_dexterity > 0 {
            let color = if ctx.dexterity < req.min_dexterity { WORSE } else { GREY };
            segs.push(Seg::Text(format!("Dexterite requise: {}", req.min_dexterity), color));
        }
        if req.min_intelligence > 0 {
            let color = if ctx.intelligence < req.min_intelligence { WORSE } else { GREY };
            segs.push(Seg::Text(format!("Intelligence requise: {}", req.min_intelligence), color));
        }
    }
    // ── Flavour line for uniques (italic-grey, just above the gold value) ──
    if let Some(text) = item.flavour.as_deref() {
        if !text.is_empty() {
            segs.push(Seg::Sep);
            // The 5x7 font has no italic; quotes carry the same "voice" effect.
            segs.push(Seg::Text(format!("\"{}\"", sanitize(text)), FLAVOUR));
        }
    }

    // A loose gem: spell out what it grants in each kind of host item.
    if let Some(gem) = item.gem {
        use crate::components::item::GemHost;
        segs.push(Seg::Sep);
        for (label, host) in [
            ("Arme", GemHost::Weapon),
            ("Armure/Casque", GemHost::Armor),
            ("Bouclier", GemHost::Shield),
        ] {
            segs.push(Seg::Text(format!("{label}: {}", gem.effect_desc(host)), [0.55, 0.78, 1.0, 1.0]));
        }
    }

    // Sockets: how many are filled vs open, and what sits in each.
    if item.sockets > 0 {
        segs.push(Seg::Sep);
        let host = crate::components::item::gem_host(item.item.base.kind);
        let filled = item.socketed.len();
        segs.push(Seg::Text(
            format!("Sertissages: {}/{} (glisser une gemme / U pour retirer)", filled, item.sockets),
            [0.55, 0.78, 1.0, 1.0],
        ));
        for gem in &item.socketed {
            segs.push(Seg::Text(
                format!("  {} — {}", gem.display_name(), gem.effect_desc(host)),
                [0.55, 0.78, 1.0, 1.0],
            ));
        }
    }

    segs.push(Seg::Sep);
    let value = crate::systems::vendor::price(item);
    segs.push(Seg::Text(format!("Valeur: {value} or"), GOLD_TEXT));

    segs
}

/// Side of the square item-icon shown in the header band.
const ICON_BOX: f32 = 40.0;
/// Gap between the header icon and the name/type text beside it.
const ICON_GAP: f32 = 8.0;

/// Draw the tooltip for `item` near `mouse`, clamped inside `viewport`.
/// `compare` is the item currently worn in the same slot (for diff rows);
/// `ctx` drives the requirement colours and the damage-reduction formula.
/// `time` animates the header icon's rarity glow (same sprite as the grid).
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn draw(
    batch: &mut SpriteBatch,
    item: &ItemInstance,
    compare: Option<&ItemInstance>,
    ctx: PlayerCtx,
    viewport: [f32; 2],
    mouse: [f32; 2],
    time: f32,
) {
    let segs = build(item, compare, ctx);
    if segs.is_empty() {
        return;
    }

    let px = 1.5;
    let char_w = font::CELL * px;
    let line_h = 15.0;
    let sep_h = 7.0;
    let pad = 8.0;

    // The header (item name + base type) sits to the right of the icon; the
    // split point is the first separator, after which content spans full width.
    let header_len = segs.iter().take_while(|s| matches!(s, Seg::Text(..))).count();
    let (header, body) = segs.split_at(header_len);

    let header_text_w = header
        .iter()
        .map(|s| if let Seg::Text(t, _) = s { t.len() as f32 * char_w } else { 0.0 })
        .fold(0.0_f32, f32::max);
    let header_text_h = header_len as f32 * line_h;
    let band_h = ICON_BOX.max(header_text_h);
    let band_w = ICON_BOX + ICON_GAP + header_text_w;

    // Body block dimensions (everything below the header band).
    let mut body_w = 0.0_f32;
    let mut body_h = 0.0_f32;
    for s in body {
        match s {
            Seg::Text(t, _) => {
                body_w = body_w.max(t.len() as f32 * char_w);
                body_h += line_h;
            }
            Seg::Sep => body_h += sep_h,
        }
    }

    let content_w = band_w.max(body_w);
    let w = (content_w + pad * 2.0).max(140.0);
    let h = band_h + body_h + pad * 2.0;

    // Follow the cursor, flipping side/vertical when it would clip the viewport.
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

    let rc = rarity_color(item.rarity().color_name());
    fill(batch, [x, y, w, h], SUB_TOOLTIP, BG);
    border(batch, [x, y, w, h], 2.0, SUB_TOOLTIP + 1, rc);

    // ── Header band: the item sprite (rarity-aware, same as the grid) on a dark
    //    plate, with the name/type lines vertically centred beside it. ─────────
    let icon_cx = x + pad + ICON_BOX * 0.5;
    let icon_cy = y + pad + band_h * 0.5;
    shapes::rounded_rect(
        batch,
        icon_cx - ICON_BOX * 0.5,
        icon_cy - ICON_BOX * 0.5,
        ICON_BOX,
        ICON_BOX,
        5.0,
        RenderLayer::UiScreen,
        0,
        SUB_TOOLTIP + 2,
        [0.07, 0.07, 0.10, 1.0],
    );
    item_icon::draw_item_icon(
        batch,
        item.item.base.kind,
        icon_cx,
        icon_cy,
        ICON_BOX * 0.82,
        item.rarity(),
        RenderLayer::UiScreen,
        0,
        SUB_TOOLTIP + 3,
        time,
    );

    let text_x = x + pad + ICON_BOX + ICON_GAP;
    let mut hy = y + pad + (band_h - header_text_h) * 0.5;
    for s in header {
        if let Seg::Text(t, color) = s {
            font::draw_text(batch, t.as_bytes(), text_x, hy, px, *color, RenderLayer::UiScreen, 0, SUB_TOOLTIP + 3);
            hy += line_h;
        }
    }

    // ── Body: full-width sections below the header band. ──────────────────────
    let mut cy = y + pad + band_h;
    for s in body {
        match s {
            Seg::Text(t, color) => {
                font::draw_text(
                    batch,
                    t.as_bytes(),
                    x + pad,
                    cy,
                    px,
                    *color,
                    RenderLayer::UiScreen,
                    0,
                    SUB_TOOLTIP + 2,
                );
                cy += line_h;
            }
            Seg::Sep => {
                fill(
                    batch,
                    [x + pad, cy + sep_h * 0.5 - 0.5, w - pad * 2.0, 1.0],
                    SUB_TOOLTIP + 2,
                    SEP_COLOR,
                );
                cy += sep_h;
            }
        }
    }
}

pub fn item_tooltip(inst: &ItemInstance) -> String {
    let lines = inst.tooltip_lines();
    let width = lines.iter().map(String::len).max().unwrap_or(0);
    let border = "-".repeat(width + 2);
    let mut s = format!("+{border}+\n");
    for line in &lines {
        s.push_str(&format!("| {line:<width$} |\n"));
    }
    s.push_str(&format!("+{border}+"));
    s
}

#[cfg(test)]
mod tests {
    use mge_gameplay::stats::{Modifier, StatKind};
    use mge_items::affixes::{AffixKind, AffixMod, AppliedAffix, ItemAffixes};
    use mge_items::item::{Item, ItemKind};
    use mge_items::rarity::Rarity;

    use super::*;
    use crate::data::items::base_item;

    const HERO: PlayerCtx =
        PlayerCtx { level: 5, strength: 999, dexterity: 999, intelligence: 999 };

    fn weapon_with(mods: Vec<AffixMod>) -> ItemInstance {
        let base = base_item(ItemKind::Sword, 5);
        let item = Item::new(1, base, Rarity::Magic, 5);
        let prefix = AppliedAffix { def_id: 0, kind: AffixKind::Prefix, mods };
        let affixes = ItemAffixes { prefixes: vec![prefix], suffixes: vec![] };
        ItemInstance::new(item, affixes, vec![], "Test Blade".into())
    }

    #[test]
    fn rarity_colors_match_spec() {
        assert_eq!(rarity_color("blue"), AFFIX_BLUE);
        assert_eq!(rarity_color("yellow"), GOLD_TEXT);
        assert_eq!(rarity_color("gold"), [1.0, 0.549, 0.0, 1.0]);
        assert_eq!(rarity_color("white"), WHITE);
    }

    #[test]
    fn collect_mods_sums_per_stat() {
        let inst = weapon_with(vec![
            AffixMod { stat: StatKind::DamageMax, modifier: Modifier::Flat(6) },
            AffixMod { stat: StatKind::DamageMax, modifier: Modifier::Flat(4) },
        ]);
        let mods = collect_mods(&inst);
        assert_eq!(stat_flat(&mods, StatKind::DamageMax), 10);
    }

    #[test]
    fn weapon_section_emits_damage_and_dps() {
        let inst = weapon_with(vec![
            AffixMod { stat: StatKind::DamageMin, modifier: Modifier::Flat(4) },
            AffixMod { stat: StatKind::DamageMax, modifier: Modifier::Flat(10) },
        ]);
        let segs = build(&inst, None, HERO);
        let texts: Vec<&str> = segs
            .iter()
            .filter_map(|s| match s {
                Seg::Text(t, _) => Some(t.as_str()),
                Seg::Sep => None,
            })
            .collect();
        assert!(texts.iter().any(|t| *t == "Degats Physiques: 4-10"));
        assert!(texts.iter().any(|t| t.starts_with("DPS:")));
    }

    #[test]
    fn attack_speed_raises_dps() {
        let slow = weapon_with(vec![AffixMod { stat: StatKind::DamageMax, modifier: Modifier::Flat(10) }]);
        let fast = weapon_with(vec![
            AffixMod { stat: StatKind::DamageMax, modifier: Modifier::Flat(10) },
            AffixMod { stat: StatKind::AttackSpeed, modifier: Modifier::Percentage(50) },
        ]);
        assert!(dps(&collect_mods(&fast)) > dps(&collect_mods(&slow)));
    }

    #[test]
    fn comparison_diffs_dps() {
        let better = weapon_with(vec![AffixMod { stat: StatKind::DamageMax, modifier: Modifier::Flat(20) }]);
        let worse = weapon_with(vec![AffixMod { stat: StatKind::DamageMax, modifier: Modifier::Flat(6) }]);
        let segs = build(&better, Some(&worse), HERO);
        let has_up = segs.iter().any(|s| matches!(s, Seg::Text(t, c) if t.contains("DPS") && *c == BETTER));
        assert!(has_up);
    }

    #[test]
    fn unmet_strength_requirement_is_red() {
        let weak = PlayerCtx { level: 5, strength: 1, dexterity: 1, intelligence: 1 };
        // The Short Sword base requires some strength; a near-zero hero fails it.
        let inst = weapon_with(vec![]);
        let req = inst.item.base.requirements.min_strength;
        if req > 0 {
            let segs = build(&inst, None, weak);
            let red_req = segs.iter().any(|s| matches!(s, Seg::Text(t, c) if t.starts_with("Force requise") && *c == WORSE));
            assert!(red_req);
        }
    }

    #[test]
    fn affix_color_is_per_element() {
        assert_eq!(affix_color(StatKind::FireResist), FIRE);
        assert_eq!(affix_color(StatKind::ColdResist), COLD);
        assert_eq!(affix_color(StatKind::LightningResist), LIGHTNING);
        assert_eq!(affix_color(StatKind::PoisonResist), POISON);
        assert_eq!(affix_color(StatKind::LifeOnHit), LEECH);
    }

    #[test]
    fn affix_line_formats_resist_as_percent() {
        let s = affix_line(StatKind::FireResist, 15, 0).unwrap();
        assert_eq!(s, "+15% Resist Feu");
        assert!(affix_line(StatKind::DamageMax, 6, 0).is_none());
    }

    #[test]
    fn draw_emits_the_item_icon_in_the_header() {
        // The tooltip header now carries the item sprite (icon plate + the
        // same rarity-aware glyph used by the inventory grid). A Unique drives
        // the glyph through its glow/contour/particles path, so the rendered
        // batch must exceed a plain text-only layout by at least those quads.
        let base = base_item(ItemKind::Sword, 5);
        let item = Item::new(1, base, Rarity::Unique, 5);
        let inst = ItemInstance::new(item, ItemAffixes::default(), vec![], "Test Blade".into());

        let mut with_icon = SpriteBatch::new();
        draw(&mut with_icon, &inst, None, HERO, [800.0, 600.0], [100.0, 100.0], 0.0);

        let mut icon_only = SpriteBatch::new();
        item_icon::draw_item_icon(
            &mut icon_only, ItemKind::Sword, 0.0, 0.0, ICON_BOX * 0.82,
            Rarity::Unique, RenderLayer::UiScreen, 0, 0, 0.0,
        );
        // Tooltip = background + border + icon plate + the full icon + text rows.
        assert!(
            with_icon.len() > icon_only.len() + 2,
            "tooltip ({}) should contain the icon ({}) plus chrome/text",
            with_icon.len(),
            icon_only.len(),
        );
    }
}
