//! Definitions de quetes et objectifs.
//!
//! <!-- @id: sd-data-quest-def @do: define @role: arpg @layer: 3 @human: miyuk -->
//!
//! Fichier TOML source : `data/quests/act{N}/{quete}.toml`

use serde::{Deserialize, Serialize};

/// Fichier TOML de quete (wrapper racine).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestFile {
    /// Definition de la quete.
    pub quest: QuestDef,
}

/// Definition complete d'une quete.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestDef {
    /// Identifiant unique (ex: `"den_of_evil"`).
    pub id: String,
    /// Nom affiche.
    pub name: String,
    /// Numero d'acte.
    pub act: u8,
    /// ID du NPC donneur de quete.
    pub quest_giver: String,
    /// Description de la quete.
    pub description: String,
    /// Declencheur de la quete.
    pub trigger: QuestTriggerDef,
    /// Objectif principal.
    pub objective: QuestObjectiveDef,
    /// Condition de completion.
    pub completion_trigger: QuestCompletionDef,
    /// Recompenses.
    pub rewards: Vec<QuestRewardDef>,
    /// Scripts de dialogue.
    pub dialogue: QuestDialogue,
}

/// Declencheur de quete (comment la quete commence).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestTriggerDef {
    /// Type de declencheur (ex: `"NpcTalked"`).
    #[serde(rename = "type")]
    pub trigger_type: String,
    /// ID du NPC (si applicable).
    #[serde(default)]
    pub npc_id: Option<String>,
    /// ID de la zone (si applicable).
    #[serde(default)]
    pub zone_id: Option<String>,
    /// ID du monstre (si applicable).
    #[serde(default)]
    pub monster_id: Option<String>,
}

/// Objectif de quete.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestObjectiveDef {
    /// Type d'objectif (ex: `"ClearZone"`, `"KillBoss"`, `"FindItem"`).
    #[serde(rename = "type")]
    pub objective_type: String,
    /// ID de la zone (si applicable).
    #[serde(default)]
    pub zone_id: Option<String>,
    /// ID du monstre (si applicable).
    #[serde(default)]
    pub monster_id: Option<String>,
    /// ID de l'item (si applicable).
    #[serde(default)]
    pub item_id: Option<String>,
    /// ID du NPC (si applicable).
    #[serde(default)]
    pub npc_id: Option<String>,
}

/// Condition de completion de quete.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestCompletionDef {
    /// Type de completion (ex: `"AllMonstersKilled"`).
    #[serde(rename = "type")]
    pub completion_type: String,
    /// ID de la zone (si applicable).
    #[serde(default)]
    pub zone_id: Option<String>,
    /// ID du monstre (si applicable).
    #[serde(default)]
    pub monster_id: Option<String>,
}

/// Recompense de quete.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestRewardDef {
    /// Type de recompense (ex: `"SkillPoint"`, `"SkillReset"`).
    #[serde(rename = "type")]
    pub reward_type: String,
    /// Valeur de la recompense.
    pub value: i32,
    /// Description optionnelle.
    #[serde(default)]
    pub description: Option<String>,
}

/// Scripts de dialogue pour une quete.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestDialogue {
    /// Script d'acceptation.
    pub accept_script: String,
    /// Script de progression.
    pub progress_script: String,
    /// Script de completion.
    pub complete_script: String,
}
