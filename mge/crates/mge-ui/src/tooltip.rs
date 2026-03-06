use serde::{Deserialize, Serialize};

/// Rarity tier affecting tooltip border color and label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rarity {
    Normal,
    Magic,
    Rare,
    Unique,
    Set,
    Runeword,
}

/// A single stat line in a tooltip.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TooltipLine {
    pub text: String,
    pub color: TooltipColor,
}

/// Semantic colors for tooltip text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TooltipColor {
    Default,
    Positive,
    Negative,
    Info,
    Rarity,
    Requirement,
    Flavor,
}

/// Tooltip content for an item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemTooltip {
    pub name: String,
    pub rarity: Rarity,
    pub item_type: String,
    pub base_stats: Vec<TooltipLine>,
    pub affixes: Vec<TooltipLine>,
    pub requirements: Vec<TooltipLine>,
    pub flavor_text: Option<String>,
}

impl ItemTooltip {
    /// Collect all lines in display order.
    pub fn all_lines(&self) -> Vec<&TooltipLine> {
        let mut lines: Vec<&TooltipLine> = Vec::new();
        lines.extend(&self.base_stats);
        lines.extend(&self.affixes);
        lines.extend(&self.requirements);
        lines
    }

    pub fn line_count(&self) -> usize {
        self.base_stats.len()
            + self.affixes.len()
            + self.requirements.len()
            + usize::from(self.flavor_text.is_some())
    }
}

/// Tooltip content for a skill.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTooltip {
    pub name: String,
    pub level: u32,
    pub description: String,
    pub stats: Vec<TooltipLine>,
    pub cooldown_secs: Option<f32>,
    pub resource_cost: Option<f32>,
    pub next_level_preview: Vec<TooltipLine>,
}

/// Tooltip content for a stat on the character sheet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatTooltip {
    pub stat_name: String,
    pub value: String,
    pub breakdown: Vec<TooltipLine>,
    pub description: String,
}

/// Union of all tooltip kinds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TooltipContent {
    Item(ItemTooltip),
    Skill(SkillTooltip),
    Stat(StatTooltip),
    Simple { title: String, body: String },
}

/// Active tooltip state (what's currently shown or pending).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TooltipState {
    pub active: Option<TooltipContent>,
    pub screen_x: f32,
    pub screen_y: f32,
}

impl TooltipState {
    pub fn show(&mut self, content: TooltipContent, x: f32, y: f32) {
        self.active = Some(content);
        self.screen_x = x;
        self.screen_y = y;
    }

    pub fn hide(&mut self) {
        self.active = None;
    }

    pub fn is_visible(&self) -> bool {
        self.active.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_tooltip_line_count() {
        let tip = ItemTooltip {
            name: "Windforce".into(),
            rarity: Rarity::Unique,
            item_type: "Hydra Bow".into(),
            base_stats: vec![TooltipLine {
                text: "Damage: 15-35".into(),
                color: TooltipColor::Default,
            }],
            affixes: vec![
                TooltipLine { text: "+250% ED".into(), color: TooltipColor::Positive },
                TooltipLine { text: "20% IAS".into(), color: TooltipColor::Positive },
            ],
            requirements: vec![TooltipLine {
                text: "Req Dex: 167".into(),
                color: TooltipColor::Requirement,
            }],
            flavor_text: Some("A legendary bow.".into()),
        };
        assert_eq!(tip.line_count(), 5);
        assert_eq!(tip.all_lines().len(), 4);
    }

    #[test]
    fn skill_tooltip_fields() {
        let tip = SkillTooltip {
            name: "Fireball".into(),
            level: 5,
            description: "Launches a ball of fire.".into(),
            stats: vec![TooltipLine {
                text: "Fire Damage: 30-50".into(),
                color: TooltipColor::Default,
            }],
            cooldown_secs: Some(1.5),
            resource_cost: Some(12.0),
            next_level_preview: vec![],
        };
        assert_eq!(tip.cooldown_secs, Some(1.5));
    }

    #[test]
    fn tooltip_state_show_hide() {
        let mut state = TooltipState::default();
        assert!(!state.is_visible());
        state.show(
            TooltipContent::Simple { title: "Gold".into(), body: "1234".into() },
            100.0,
            200.0,
        );
        assert!(state.is_visible());
        state.hide();
        assert!(!state.is_visible());
    }
}
