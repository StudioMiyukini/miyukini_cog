// @id: MGE-ARPG-Quest-Reward @do: quest-rewards @role: back-end @layer: 3 @human: miyuk
//! Quest reward definition.

/// Rewards granted upon quest completion.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct QuestReward {
    /// Experience points awarded.
    pub experience: u64,
    /// Gold awarded.
    pub gold: u32,
    /// Skill points awarded.
    pub skill_points: u32,
    /// Stat (attribute) points awarded.
    pub stat_points: u32,
    /// Item identifiers granted as reward.
    pub item_ids: Vec<String>,
}

