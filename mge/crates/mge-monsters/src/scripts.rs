use serde::{Deserialize, Serialize};

use mge_world::zone::ZoneId;

/// A single trigger in a boss script.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptTrigger {
    /// HP percentage that activates this trigger [0–100].
    pub hp_pct: u8,
    pub description: String,
    pub spawns_adds: bool,
    pub changes_behavior: Option<String>,
}

/// A scripted boss encounter definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossScript {
    pub boss_monster_id: u32,
    pub zone_id: ZoneId,
    pub triggers: Vec<ScriptTrigger>,
    pub enrage_tick: Option<u32>,
}

impl BossScript {
    /// Find the trigger that applies at the given HP percentage.
    #[must_use]
    pub fn active_trigger(&self, hp_pct: u8) -> Option<&ScriptTrigger> {
        self.triggers
            .iter()
            .filter(|t| hp_pct <= t.hp_pct)
            .min_by_key(|t| t.hp_pct)
    }
}

/// A mini-boss encounter script.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniBossScript {
    pub mini_boss_id: u32,
    pub zone_id: ZoneId,
    /// Pack of regular monsters that accompany the mini-boss.
    pub escort_monster_ids: Vec<u32>,
    pub description: String,
}

/// Act 1 boss scripts.
#[must_use]
pub fn act1_boss_scripts() -> Vec<BossScript> {
    vec![
        BossScript {
            boss_monster_id: 100, // Blood Raven
            zone_id: 21,
            triggers: vec![
                ScriptTrigger {
                    hp_pct: 75,
                    description: "Blood Raven summons fallen archers.".into(),
                    spawns_adds: true,
                    changes_behavior: None,
                },
                ScriptTrigger {
                    hp_pct: 30,
                    description: "Blood Raven enters rage, increased attack speed.".into(),
                    spawns_adds: false,
                    changes_behavior: Some("rage_mode".into()),
                },
            ],
            enrage_tick: Some(600),
        },
        BossScript {
            boss_monster_id: 101, // Andariel
            zone_id: 42,
            triggers: vec![
                ScriptTrigger {
                    hp_pct: 60,
                    description: "Andariel increases poison nova frequency.".into(),
                    spawns_adds: false,
                    changes_behavior: Some("enrage".into()),
                },
                ScriptTrigger {
                    hp_pct: 25,
                    description: "Andariel summons lesser demons.".into(),
                    spawns_adds: true,
                    changes_behavior: None,
                },
            ],
            enrage_tick: Some(1800),
        },
    ]
}

/// Act 1 mini-boss scripts.
#[must_use]
pub fn act1_mini_boss_scripts() -> Vec<MiniBossScript> {
    vec![
        MiniBossScript {
            mini_boss_id: 2, // Fallen Shaman in Den of Evil
            zone_id: 11,
            escort_monster_ids: vec![1, 1, 1], // 3 Fallen
            description: "Den Shaman commands a pack of Fallen.".into(),
        },
        MiniBossScript {
            mini_boss_id: 7, // Skeleton Mage in Catacombs
            zone_id: 41,
            escort_monster_ids: vec![4, 4], // 2 Skeletons
            description: "Catacomb Mage flanked by skeleton warriors.".into(),
        },
    ]
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn andariel_script_has_two_triggers() {
        let scripts = act1_boss_scripts();
        let andariel = scripts.iter().find(|s| s.boss_monster_id == 101).unwrap();
        assert_eq!(andariel.triggers.len(), 2);
    }

    #[test]
    fn blood_raven_trigger_at_30_pct() {
        let scripts = act1_boss_scripts();
        let br = scripts.iter().find(|s| s.boss_monster_id == 100).unwrap();
        let trigger = br.active_trigger(30);
        assert!(trigger.is_some());
        assert!(!trigger.unwrap().spawns_adds);
    }

    #[test]
    fn trigger_at_full_hp_returns_none() {
        let scripts = act1_boss_scripts();
        let br = scripts.iter().find(|s| s.boss_monster_id == 100).unwrap();
        assert!(br.active_trigger(100).is_none());
    }

    #[test]
    fn mini_boss_scripts_non_empty() {
        assert!(!act1_mini_boss_scripts().is_empty());
    }
}
