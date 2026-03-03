// @id: MGE-UI-Tooltip @do: item-tooltip @role: front-end @layer: 2 @human: miyuk
//! Item tooltip system — quality colors, stat comparison, D2-style floating panel.

use egui::{Color32, Context, Pos2};

use crate::theme::D2Colors;

// ---------------------------------------------------------------------------
// Quality color constants ([f32; 4] = [r, g, b, a])
// ---------------------------------------------------------------------------

/// Normal item name color (white).
pub const COLOR_NORMAL: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
/// Magic item name color (blue).
pub const COLOR_MAGIC: [f32; 4] = [0.3, 0.3, 1.0, 1.0];
/// Rare item name color (yellow).
pub const COLOR_RARE: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
/// Set item name color (green).
pub const COLOR_SET: [f32; 4] = [0.0, 0.8, 0.0, 1.0];
/// Unique item name color (gold).
pub const COLOR_UNIQUE: [f32; 4] = [0.85, 0.65, 0.0, 1.0];
/// Stat improvement color (green).
pub const COLOR_BETTER: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
/// Stat loss color (red).
pub const COLOR_WORSE: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

// ---------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------

/// A single line of text in a tooltip with its display color.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TooltipLine {
    /// Display text for this line.
    pub text: String,
    /// RGBA color in linear float ([r, g, b, a]).
    pub color: [f32; 4],
}

impl TooltipLine {
    /// Construct a new `TooltipLine`.
    pub fn new(text: impl Into<String>, color: [f32; 4]) -> Self {
        Self { text: text.into(), color }
    }
}

/// Complete data model for an item tooltip including optional stat comparison.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemTooltipData {
    /// Item display name.
    pub name: String,
    /// Color for the name text (quality-dependent).
    pub name_color: [f32; 4],
    /// Base item type (e.g. "Long Sword").
    pub base_type: String,
    /// Body lines (stats, affixes, requirements, …).
    pub lines: Vec<TooltipLine>,
    /// Whether the item has been identified.
    pub is_identified: bool,
    /// Comparison lines vs currently equipped item (may be empty).
    pub comparison: Vec<TooltipLine>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Returns the quality color constant for a given quality string.
///
/// Recognized values: `"normal"`, `"magic"`, `"rare"`, `"set"`, `"unique"`.
/// Anything else falls back to [`COLOR_NORMAL`].
pub fn color_for_quality(quality: &str) -> [f32; 4] {
    match quality {
        "magic" => COLOR_MAGIC,
        "rare" => COLOR_RARE,
        "set" => COLOR_SET,
        "unique" => COLOR_UNIQUE,
        // "normal" and any unknown quality fall back to white
        _ => COLOR_NORMAL,
    }
}

/// Builds stat comparison lines between a candidate item and the equipped item.
///
/// For each stat present in `current`:
/// - If the value is higher than in `equipped`: green line `"stat: +diff"`.
/// - If the value is lower: red line `"stat: -diff"`.
/// - If the stat is absent from `equipped`: green line `"stat: +value"` (new stat).
///
/// For each stat present in `equipped` but **absent** from `current`:
/// - Red line `"stat: -value"` (stat that would be lost).
pub fn compare_stats(
    current: &[(String, i32)],
    equipped: &[(String, i32)],
) -> Vec<TooltipLine> {
    let mut lines: Vec<TooltipLine> = Vec::new();

    for (stat, cur_val) in current {
        let equipped_val = equipped
            .iter()
            .find(|(s, _)| s == stat)
            .map(|(_, v)| *v);

        match equipped_val {
            Some(eq_val) => {
                let diff = cur_val - eq_val;
                if diff > 0 {
                    lines.push(TooltipLine::new(
                        format!("{stat}: +{diff}"),
                        COLOR_BETTER,
                    ));
                } else if diff < 0 {
                    lines.push(TooltipLine::new(
                        format!("{stat}: {diff}"),
                        COLOR_WORSE,
                    ));
                }
                // diff == 0 → no change, omit
            }
            None => {
                lines.push(TooltipLine::new(
                    format!("{stat}: +{cur_val}"),
                    COLOR_BETTER,
                ));
            }
        }
    }

    // Stats in equipped but not in current → lost
    for (stat, eq_val) in equipped {
        let in_current = current.iter().any(|(s, _)| s == stat);
        if !in_current {
            lines.push(TooltipLine::new(
                format!("{stat}: -{eq_val}"),
                COLOR_WORSE,
            ));
        }
    }

    lines
}

/// Builds an `ItemTooltipData` for an unidentified item.
///
/// The tooltip shows a single red "Unidentified" line and `is_identified: false`.
pub fn unidentified_tooltip(base_type: &str, quality: &str) -> ItemTooltipData {
    ItemTooltipData {
        name: format!("Unidentified {base_type}"),
        name_color: color_for_quality(quality),
        base_type: base_type.to_string(),
        lines: vec![TooltipLine {
            text: "Unidentified".to_string(),
            color: [0.7, 0.0, 0.0, 1.0],
        }],
        is_identified: false,
        comparison: Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// egui helper — convert [f32; 4] to Color32
// ---------------------------------------------------------------------------

fn f32_color(c: [f32; 4]) -> Color32 {
    Color32::from_rgba_premultiplied(
        (c[0] * 255.0) as u8,
        (c[1] * 255.0) as u8,
        (c[2] * 255.0) as u8,
        (c[3] * 255.0) as u8,
    )
}

// ---------------------------------------------------------------------------
// Public draw entry-point
// ---------------------------------------------------------------------------

/// Draw an item tooltip anchored near the given screen position.
pub fn draw_tooltip(ctx: &Context, pos: Pos2, data: &ItemTooltipData) {
    egui::Area::new("item_tooltip".into())
        .fixed_pos(pos + egui::vec2(16.0, 0.0))
        .order(egui::Order::Tooltip)
        .show(ctx, |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.set_max_width(220.0);

                // Item name (quality-colored)
                ui.label(
                    egui::RichText::new(&data.name)
                        .color(f32_color(data.name_color))
                        .size(13.0)
                        .strong(),
                );

                // Base type
                ui.label(
                    egui::RichText::new(&data.base_type)
                        .color(D2Colors::TEXT_NORMAL)
                        .size(11.0),
                );

                ui.separator();

                // Body lines
                for line in &data.lines {
                    ui.label(
                        egui::RichText::new(&line.text)
                            .color(f32_color(line.color))
                            .size(11.0),
                    );
                }

                // Comparison lines (if any)
                if !data.comparison.is_empty() {
                    ui.separator();
                    for line in &data.comparison {
                        ui.label(
                            egui::RichText::new(&line.text)
                                .color(f32_color(line.color))
                                .size(10.0),
                        );
                    }
                }
            });
        });
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper — approximate float equality.
    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-6
    }

    fn color_eq(a: [f32; 4], b: [f32; 4]) -> bool {
        a.iter().zip(b.iter()).all(|(x, y)| approx_eq(*x, *y))
    }

    // -----------------------------------------------------------------------
    // 1. color_for_quality
    // -----------------------------------------------------------------------

    #[test]
    fn tooltip_color_by_quality_magic() {
        let c = color_for_quality("magic");
        assert!(color_eq(c, COLOR_MAGIC), "magic should be blue");
    }

    #[test]
    fn tooltip_color_by_quality_rare() {
        let c = color_for_quality("rare");
        assert!(color_eq(c, COLOR_RARE), "rare should be yellow");
    }

    #[test]
    fn tooltip_color_by_quality_unique() {
        let c = color_for_quality("unique");
        assert!(color_eq(c, COLOR_UNIQUE), "unique should be gold");
    }

    #[test]
    fn tooltip_color_by_quality_set() {
        let c = color_for_quality("set");
        assert!(color_eq(c, COLOR_SET), "set should be green");
    }

    #[test]
    fn tooltip_color_by_quality_normal() {
        let c = color_for_quality("normal");
        assert!(color_eq(c, COLOR_NORMAL), "normal should be white");
    }

    #[test]
    fn tooltip_color_by_quality_unknown_falls_back() {
        let c = color_for_quality("???");
        assert!(color_eq(c, COLOR_NORMAL), "unknown quality should fall back to white");
    }

    // -----------------------------------------------------------------------
    // 2. unidentified_tooltip
    // -----------------------------------------------------------------------

    #[test]
    fn tooltip_unidentified() {
        let data = unidentified_tooltip("sword", "rare");

        assert_eq!(data.name, "Unidentified sword");
        assert!(color_eq(data.name_color, COLOR_RARE));
        assert_eq!(data.base_type, "sword");
        assert_eq!(data.lines.len(), 1);
        assert_eq!(data.lines[0].text, "Unidentified");
        assert!(!data.is_identified);
        assert!(data.comparison.is_empty());
    }

    // -----------------------------------------------------------------------
    // 3. compare_stats — current stat better
    // -----------------------------------------------------------------------

    #[test]
    fn tooltip_comparison_better() {
        let current = vec![("damage".to_string(), 50)];
        let equipped = vec![("damage".to_string(), 30)];

        let lines = compare_stats(&current, &equipped);

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].text, "damage: +20");
        assert!(color_eq(lines[0].color, COLOR_BETTER));
    }

    // -----------------------------------------------------------------------
    // 4. compare_stats — current stat worse
    // -----------------------------------------------------------------------

    #[test]
    fn tooltip_comparison_worse() {
        let current = vec![("defense".to_string(), 20)];
        let equipped = vec![("defense".to_string(), 40)];

        let lines = compare_stats(&current, &equipped);

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].text, "defense: -20");
        assert!(color_eq(lines[0].color, COLOR_WORSE));
    }

    // -----------------------------------------------------------------------
    // 5. compare_stats — new stat not in equipped
    // -----------------------------------------------------------------------

    #[test]
    fn tooltip_comparison_new_stat() {
        let current = vec![("poison_resist".to_string(), 15)];
        let equipped: Vec<(String, i32)> = Vec::new();

        let lines = compare_stats(&current, &equipped);

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].text, "poison_resist: +15");
        assert!(color_eq(lines[0].color, COLOR_BETTER));
    }

    // -----------------------------------------------------------------------
    // 6. compare_stats — lost stat (in equipped, absent from current)
    // -----------------------------------------------------------------------

    #[test]
    fn tooltip_comparison_lost_stat() {
        let current: Vec<(String, i32)> = Vec::new();
        let equipped = vec![("fire_resist".to_string(), 25)];

        let lines = compare_stats(&current, &equipped);

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].text, "fire_resist: -25");
        assert!(color_eq(lines[0].color, COLOR_WORSE));
    }

    // -----------------------------------------------------------------------
    // 7. compare_stats — equal stats produce no lines
    // -----------------------------------------------------------------------

    #[test]
    fn tooltip_comparison_equal_no_line() {
        let current = vec![("speed".to_string(), 10)];
        let equipped = vec![("speed".to_string(), 10)];

        let lines = compare_stats(&current, &equipped);
        assert!(lines.is_empty(), "equal stats should produce no comparison lines");
    }
}
