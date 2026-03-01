//! Definitions de competences (skills).
//!
//! <!-- @id: sd-data-skill-def @do: define @role: arpg @layer: 3 @human: miyuk -->
//!
//! Fichier TOML source : `data/skills/{classe}/{skill}.toml`

use serde::{Deserialize, Serialize};

/// Fichier TOML de skill (wrapper racine).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillFile {
    /// Definition du skill.
    pub skill: SkillDef,
}

/// Definition complete d'une competence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDef {
    /// Identifiant unique du skill (ex: `"bone_spear"`).
    pub id: String,
    /// Nom affiche.
    pub name: String,
    /// Classe associee (ex: `"Mortecian"`).
    pub class: String,
    /// Arbre de competences (ex: `"Bone"`).
    pub tree: String,
    /// Position dans l'arbre `[colonne, rangee]`.
    pub tree_position: [u8; 2],
    /// Niveau requis pour debloquer.
    pub required_level: u8,
    /// Niveau maximum investissable.
    pub max_level: u8,
    /// Type de skill : Active, Passive, Aura.
    pub skill_type: SkillType,
    /// Type de ciblage.
    pub target_type: SkillTargetType,
    /// Sprite d'icone dans l'UI.
    pub icon_sprite: String,
    /// Parametres de base du skill (actifs uniquement).
    #[serde(default)]
    pub base: Option<SkillBase>,
    /// Gains par niveau.
    pub per_level: SkillPerLevel,
    /// Synergies avec d'autres skills.
    #[serde(default)]
    pub synergies: Vec<SkillSynergy>,
    /// Pre-requis de skills.
    pub prerequisites: SkillPrereqs,
    /// Configuration d'aura (auras uniquement).
    #[serde(default)]
    pub aura: Option<AuraDef>,
}

/// Type de competence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillType {
    /// Competence active declenchee manuellement.
    Active,
    /// Competence passive permanente.
    Passive,
    /// Aura toggle affectant le groupe.
    Aura,
}

/// Type de ciblage d'une competence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillTargetType {
    /// Cible une entite.
    Entity,
    /// Cible une position au sol.
    Position,
    /// Cible une direction.
    Direction,
    /// S'applique a soi-meme.
    #[serde(rename = "Self")]
    SelfTarget,
    /// Pas de ciblage (passif).
    None,
}

/// Parametres de base d'un skill actif.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillBase {
    /// Cout en mana de base.
    pub mana_cost: f32,
    /// Cooldown en frames (0 = pas de cooldown).
    pub cooldown_frames: u32,
    /// Type de degats (ex: `"Magic"`, `"Fire"`, `"Physical"`).
    pub damage_type: String,
    /// Est-ce un projectile ?
    #[serde(default)]
    pub projectile: bool,
    /// Vitesse du projectile.
    #[serde(default)]
    pub projectile_speed: f32,
    /// Sprite du projectile.
    #[serde(default)]
    pub projectile_sprite: String,
    /// Rayon d'AoE (0 = impact simple).
    #[serde(default)]
    pub aoe_radius: f32,
    /// Traverse-t-il les ennemis ?
    #[serde(default)]
    pub piercing: bool,
    /// Nombre de traversees (-1 = infini).
    #[serde(default)]
    pub pierce_count: i32,
}

/// Gains par niveau d'un skill.
///
/// Tous les champs sont optionnels car chaque skill n'utilise qu'un
/// sous-ensemble de ces parametres selon son type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPerLevel {
    /// Increment du cout en mana par niveau.
    #[serde(default)]
    pub mana_cost_increment: Option<f32>,
    /// Degats min de base.
    #[serde(default)]
    pub damage_min_base: Option<i32>,
    /// Degats min par niveau.
    #[serde(default)]
    pub damage_min_per_level: Option<i32>,
    /// Degats max de base.
    #[serde(default)]
    pub damage_max_base: Option<i32>,
    /// Degats max par niveau.
    #[serde(default)]
    pub damage_max_per_level: Option<i32>,
    /// Enhanced damage de base (auras/passifs).
    #[serde(default)]
    pub enhanced_damage_base: Option<i32>,
    /// Enhanced damage par niveau.
    #[serde(default)]
    pub enhanced_damage_per_level: Option<i32>,
    /// Attack speed de base.
    #[serde(default)]
    pub attack_speed_base: Option<i32>,
    /// Attack speed par niveau.
    #[serde(default)]
    pub attack_speed_per_level: Option<i32>,
    /// Attack rating % de base.
    #[serde(default)]
    pub attack_rating_pct_base: Option<i32>,
    /// Attack rating % par niveau.
    #[serde(default)]
    pub attack_rating_pct_per_level: Option<i32>,
    /// Bonus vie par niveau (passifs).
    #[serde(default)]
    pub bonus_life_per_level: Option<i32>,
    /// Bonus degats par niveau (passifs).
    #[serde(default)]
    pub bonus_damage_per_level: Option<i32>,
    /// Bonus AR par niveau (passifs).
    #[serde(default)]
    pub bonus_ar_per_level: Option<i32>,
    /// Bonus defense par niveau (passifs).
    #[serde(default)]
    pub bonus_defense_per_level: Option<i32>,
}

/// Synergie entre deux skills.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillSynergy {
    /// ID du skill source de la synergie.
    pub skill_id: String,
    /// Bonus par niveau du skill source.
    pub bonus_per_level: i32,
}

/// Pre-requis de skills.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPrereqs {
    /// Liste des IDs de skills requis.
    pub skills_required: Vec<String>,
}

/// Definition d'une aura.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuraDef {
    /// Rayon de base de l'aura.
    pub radius_base: f32,
    /// Rayon supplementaire par niveau.
    pub radius_per_level: f32,
    /// Qui est affecte (ex: `"Party"`, `"Self"`).
    pub affects: String,
}
