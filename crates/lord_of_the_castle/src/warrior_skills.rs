//! Arbre de compétences Guerrier — définitions, prérequis, positions pour le rendu.
//!
//! @id: lord_of_the_castle_warrior_skills
//! @do: define_warrior_skill_tree
//! @role: data
//! @layer: domain
//! @human: Nœuds Guerrier : Baston, Plus fort, Tireur, etc. ; descriptions, effets, max rang, prérequis.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Identifiant unique d'une compétence Guerrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WarriorSkillId {
    // Branche mêlée / baston
    Baston,
    Bagarre,
    Kungfu,
    // Branche force / agilité
    PlusFort,
    Souple,
    EncorePlusFort,
    Rapide,
    Precis,
    Musculation,
    Balayage,
    Fulgurant,
    DetectionFaiblesses,
    PotDeWey,
    AttaqueLarge,
    DoubleArmes,
    Assassin,
    GigaChad,
    TrombiLol,
    // Branche tir
    Tireur,
    TirRapide,
    VuPercante,
    TirARepetition,
    MunitionsLourdes,
    RechargementRapide,
    Sniper,
    TirDouble,
}

/// Définition d'une compétence : nom, description, effet, rang max, prérequis.
#[derive(Debug, Clone)]
pub struct WarriorSkillDef {
    pub id: WarriorSkillId,
    pub name: &'static str,
    pub description: &'static str,
    pub effect_description: &'static str,
    pub max_rank: u32,
    /// Prérequis : au moins un doit être au rang max pour débloquer.
    pub prerequisites: &'static [WarriorSkillId],
    /// Position dans l'arbre (col, row) pour le rendu. Colonnes 0 = gauche, 7 = droite.
    pub layout_col: f32,
    pub layout_row: f32,
}

impl WarriorSkillId {
    #[must_use] 
    pub fn all() -> &'static [WarriorSkillId] {
        use WarriorSkillId::{Baston, Bagarre, Kungfu, PlusFort, Souple, EncorePlusFort, Rapide, Precis, Musculation, Balayage, Fulgurant, DetectionFaiblesses, PotDeWey, AttaqueLarge, DoubleArmes, Assassin, GigaChad, TrombiLol, Tireur, TirRapide, VuPercante, TirARepetition, MunitionsLourdes, RechargementRapide, Sniper, TirDouble};
        &[
            Baston, Bagarre, Kungfu,
            PlusFort, Souple, EncorePlusFort, Rapide, Precis, Musculation, Balayage,
            Fulgurant, DetectionFaiblesses, PotDeWey, AttaqueLarge,
            DoubleArmes, Assassin, GigaChad, TrombiLol,
            Tireur, TirRapide, VuPercante, TirARepetition,
            MunitionsLourdes, RechargementRapide, Sniper, TirDouble,
        ]
    }
}

/// Retourne la définition d'une compétence.
#[must_use] 
pub fn warrior_skill_def(id: WarriorSkillId) -> WarriorSkillDef {
    use WarriorSkillId::{Baston, Bagarre, Rapide, Kungfu, PlusFort, Souple, EncorePlusFort, Precis, Musculation, Balayage, Fulgurant, DetectionFaiblesses, PotDeWey, AttaqueLarge, DoubleArmes, Assassin, GigaChad, TrombiLol, Tireur, TirRapide, VuPercante, TirARepetition, MunitionsLourdes, RechargementRapide, Sniper, TirDouble};
    match id {
        Baston => WarriorSkillDef {
            id: Baston,
            name: "Baston",
            description: "Combat à mains nues : bonus de Force quand aucune arme n'est équipée.",
            effect_description: "+1 Force par niveau (sans arme équipée). Jusqu'à +10 Force. 10 niveaux max.",
            max_rank: 10,
            prerequisites: &[],
            layout_col: 0.0,
            layout_row: 0.0,
        },
        Bagarre => WarriorSkillDef {
            id: Bagarre,
            name: "Bagarre",
            description: "Enchaînement de coups plus rapide.",
            effect_description: "Vitesse d'attaque +10 % par niveau. 5 niveaux max.",
            max_rank: 5,
            prerequisites: &[Baston, Rapide],
            layout_col: 0.0,
            layout_row: 1.0,
        },
        Kungfu => WarriorSkillDef {
            id: Kungfu,
            name: "Kungfu",
            description: "Portée du cône d'auto-attaque augmentée.",
            effect_description: "Portée du cône d'auto-attaque +10 % par niveau. 5 niveaux max.",
            max_rank: 5,
            prerequisites: &[Bagarre],
            layout_col: 0.0,
            layout_row: 2.0,
        },
        PlusFort => WarriorSkillDef {
            id: PlusFort,
            name: "Plus fort",
            description: "Dégâts des armes de mêlée.",
            effect_description: "+1 dégât aux armes de mêlée par niveau. 3 niveaux max.",
            max_rank: 3,
            prerequisites: &[],
            layout_col: 2.0,
            layout_row: 0.0,
        },
        Souple => WarriorSkillDef {
            id: Souple,
            name: "Souple",
            description: "Vitesse d'attaque avec une arme de mêlée équipée.",
            effect_description: "Vitesse d'attaque +10 % par niveau (arme de mêlée équipée). 3 niveaux max.",
            max_rank: 3,
            prerequisites: &[PlusFort],
            layout_col: 1.0,
            layout_row: 1.0,
        },
        EncorePlusFort => WarriorSkillDef {
            id: EncorePlusFort,
            name: "Encore plus fort",
            description: "Gain de Force permanent.",
            effect_description: "+1 Force par niveau. 3 niveaux max.",
            max_rank: 3,
            prerequisites: &[PlusFort],
            layout_col: 3.0,
            layout_row: 1.0,
        },
        Rapide => WarriorSkillDef {
            id: Rapide,
            name: "Rapide",
            description: "Vélocité des mouvements.",
            effect_description: "Vitesse d'attaque augmentée.",
            max_rank: 5,
            prerequisites: &[Souple],
            layout_col: 1.0,
            layout_row: 2.0,
        },
        Precis => WarriorSkillDef {
            id: Precis,
            name: "Précis",
            description: "Chance de porter un coup critique.",
            effect_description: "Taux de coup critique +10 % par niveau. 5 niveaux max.",
            max_rank: 5,
            prerequisites: &[Souple],
            layout_col: 2.0,
            layout_row: 2.0,
        },
        Musculation => WarriorSkillDef {
            id: Musculation,
            name: "Musculation",
            description: "Constitution renforcée.",
            effect_description: "+1 Constitution par niveau. 5 niveaux max.",
            max_rank: 5,
            prerequisites: &[EncorePlusFort],
            layout_col: 3.0,
            layout_row: 2.0,
        },
        Balayage => WarriorSkillDef {
            id: Balayage,
            name: "Balayage",
            description: "Cône d'auto-attaque élargi.",
            effect_description: "Cône d'auto-attaque à 90° et portée +20 %. 1 niveau.",
            max_rank: 1,
            prerequisites: &[EncorePlusFort],
            layout_col: 4.0,
            layout_row: 2.0,
        },
        Fulgurant => WarriorSkillDef {
            id: Fulgurant,
            name: "Fulgurant",
            description: "Vitesse d'attaque fulgurante.",
            effect_description: "Vitesse d'attaque +10 % par niveau. 5 niveaux max.",
            max_rank: 5,
            prerequisites: &[Rapide],
            layout_col: 1.0,
            layout_row: 3.0,
        },
        DetectionFaiblesses => WarriorSkillDef {
            id: DetectionFaiblesses,
            name: "Détection des faiblesses",
            description: "Dégâts des coups critiques augmentés.",
            effect_description: "Dégâts des coups critiques +10 % par niveau. 5 niveaux max.",
            max_rank: 5,
            prerequisites: &[Precis],
            layout_col: 2.0,
            layout_row: 3.0,
        },
        PotDeWey => WarriorSkillDef {
            id: PotDeWey,
            name: "Pot de Wey",
            description: "Points de vie maximum augmentés.",
            effect_description: "PV max +15 % par niveau. 3 niveaux max.",
            max_rank: 3,
            prerequisites: &[Musculation],
            layout_col: 3.0,
            layout_row: 3.0,
        },
        AttaqueLarge => WarriorSkillDef {
            id: AttaqueLarge,
            name: "Attaque large",
            description: "Cône d'attaque très élargi.",
            effect_description: "Dès le 1er point : cône à 180°. Dégâts +10 % par niveau. 5 niveaux max.",
            max_rank: 5,
            prerequisites: &[Balayage],
            layout_col: 4.0,
            layout_row: 3.0,
        },
        DoubleArmes => WarriorSkillDef {
            id: DoubleArmes,
            name: "Double armes",
            description: "Équiper une arme à une main en main gauche.",
            effect_description: "Main gauche : arme à une main. Cumul des dégâts des deux armes. Taux de critique +10 %. 1 niveau.",
            max_rank: 1,
            prerequisites: &[Fulgurant],
            layout_col: 1.0,
            layout_row: 4.0,
        },
        Assassin => WarriorSkillDef {
            id: Assassin,
            name: "Assassin",
            description: "Coups critiques et mobilité.",
            effect_description: "Chance de coup critique +10 %, dégâts critiques +10 %, vitesse de déplacement +20 %. 1 niveau.",
            max_rank: 1,
            prerequisites: &[DetectionFaiblesses],
            layout_col: 2.0,
            layout_row: 4.0,
        },
        GigaChad => WarriorSkillDef {
            id: GigaChad,
            name: "GigaChad",
            description: "Régénération de vie.",
            effect_description: "Régénération : 1 PV par seconde. 1 niveau.",
            max_rank: 1,
            prerequisites: &[PotDeWey],
            layout_col: 3.0,
            layout_row: 4.0,
        },
        TrombiLol => WarriorSkillDef {
            id: TrombiLol,
            name: "Trombe",
            description: "Attaque à 360° sur toute la portée.",
            effect_description: "Attaque à 360° sur tous les ennemis à portée, +3 dégâts finaux plats. 1 niveau.",
            max_rank: 1,
            prerequisites: &[AttaqueLarge],
            layout_col: 4.0,
            layout_row: 4.0,
        },
        Tireur => WarriorSkillDef {
            id: Tireur,
            name: "Tireur",
            description: "Base du tir à distance.",
            effect_description: "+1 Dextérité. 1 niveau.",
            max_rank: 1,
            prerequisites: &[],
            layout_col: 6.0,
            layout_row: 0.0,
        },
        TirRapide => WarriorSkillDef {
            id: TirRapide,
            name: "Tir rapide",
            description: "Cadence de tir à distance.",
            effect_description: "Vitesse d'attaque avec arme à distance +10 % par niveau. 3 niveaux max.",
            max_rank: 3,
            prerequisites: &[Tireur],
            layout_col: 6.0,
            layout_row: 1.0,
        },
        VuPercante => WarriorSkillDef {
            id: VuPercante,
            name: "Vu perçante",
            description: "Dégâts des armes à distance.",
            effect_description: "Dégâts des armes à distance +10 % par niveau. 3 niveaux max.",
            max_rank: 3,
            prerequisites: &[TirRapide],
            layout_col: 5.0,
            layout_row: 2.0,
        },
        TirARepetition => WarriorSkillDef {
            id: TirARepetition,
            name: "Tir à répétition",
            description: "Vitesse et dégâts de tir.",
            effect_description: "Vitesse de tir +10 % et dégâts +10 % par niveau. 3 niveaux max.",
            max_rank: 3,
            prerequisites: &[TirRapide],
            layout_col: 7.0,
            layout_row: 2.0,
        },
        MunitionsLourdes => WarriorSkillDef {
            id: MunitionsLourdes,
            name: "Munitions lourdes",
            description: "Projectile qui traverse les ennemis.",
            effect_description: "Un projectile traverse 1 ennemi supplémentaire par niveau. 3 niveaux max.",
            max_rank: 3,
            prerequisites: &[VuPercante],
            layout_col: 5.0,
            layout_row: 3.0,
        },
        RechargementRapide => WarriorSkillDef {
            id: RechargementRapide,
            name: "Rechargement rapide",
            description: "Vitesse de rechargement.",
            effect_description: "Vitesse de tir +10 % par niveau. 3 niveaux max.",
            max_rank: 3,
            prerequisites: &[TirARepetition],
            layout_col: 7.0,
            layout_row: 3.0,
        },
        Sniper => WarriorSkillDef {
            id: Sniper,
            name: "Sniper",
            description: "Tir d'élite à longue portée.",
            effect_description: "Chance de critique et dégâts critiques à distance +30 %, portée +50 %. 1 niveau.",
            max_rank: 1,
            prerequisites: &[MunitionsLourdes],
            layout_col: 5.0,
            layout_row: 4.0,
        },
        TirDouble => WarriorSkillDef {
            id: TirDouble,
            name: "Tir double",
            description: "Deux projectiles par tir.",
            effect_description: "2 projectiles par tir au lieu d'un. 1 niveau.",
            max_rank: 1,
            prerequisites: &[RechargementRapide],
            layout_col: 7.0,
            layout_row: 4.0,
        },
    }
}

/// Arêtes de l'arbre (prérequis → compétence) pour tracer les lignes.
#[must_use] 
pub fn warrior_skill_edges() -> Vec<(WarriorSkillId, WarriorSkillId)> {
    use WarriorSkillId::{Baston, Bagarre, Rapide, Kungfu, PlusFort, Souple, EncorePlusFort, Precis, Musculation, Balayage, Fulgurant, DetectionFaiblesses, PotDeWey, AttaqueLarge, DoubleArmes, Assassin, GigaChad, TrombiLol, Tireur, TirRapide, VuPercante, TirARepetition, MunitionsLourdes, RechargementRapide, Sniper, TirDouble};
    vec![
        (Baston, Bagarre),
        (Rapide, Bagarre),
        (Bagarre, Kungfu),
        (PlusFort, Souple),
        (PlusFort, EncorePlusFort),
        (Souple, Rapide),
        (Souple, Precis),
        (EncorePlusFort, Musculation),
        (EncorePlusFort, Balayage),
        (Rapide, Fulgurant),
        (Precis, DetectionFaiblesses),
        (Musculation, PotDeWey),
        (Balayage, AttaqueLarge),
        (Fulgurant, DoubleArmes),
        (DetectionFaiblesses, Assassin),
        (PotDeWey, GigaChad),
        (AttaqueLarge, TrombiLol),
        (Tireur, TirRapide),
        (TirRapide, VuPercante),
        (TirRapide, TirARepetition),
        (VuPercante, MunitionsLourdes),
        (TirARepetition, RechargementRapide),
        (MunitionsLourdes, Sniper),
        (RechargementRapide, TirDouble),
    ]
}

/// Vérifie si les prérequis sont satisfaits (chaque prérequis au moins au rang 1).
#[must_use] 
pub fn prerequisites_met(ranks: &HashMap<WarriorSkillId, u32>, def: &WarriorSkillDef) -> bool {
    if def.prerequisites.is_empty() {
        return true;
    }
    def.prerequisites.iter().any(|&prereq| {
        ranks.get(&prereq).copied().unwrap_or(0) >= 1
    })
}
