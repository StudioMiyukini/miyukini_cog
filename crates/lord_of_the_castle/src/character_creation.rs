//! Création de personnage — stats (For, Con, Agi, Dex, Int, Sag, Cha, Luk), phrases et effets.
//!
//! @id: lord_of_the_castle_character_creation
//! @do: define_stats_phrases_and_effects
//! @role: logic
//! @layer: domain
//! @human: 8 stats, 25 phrases humoristiques avec effets, tirage aléatoire, parcours 4 étapes.

use serde::{Deserialize, Serialize};

/// Les 8 caractéristiques du personnage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stat {
    For,  // Force
    Con,  // Constitution
    Agi,  // Agilité
    Dex,  // Dextérité
    Int,  // Intelligence
    Sag,  // Sagesse
    Cha,  // Charisme
    Luk,  // Chance
}

impl Stat {
    pub fn label(self) -> &'static str {
        match self {
            Stat::For => "For",
            Stat::Con => "Con",
            Stat::Agi => "Agi",
            Stat::Dex => "Dex",
            Stat::Int => "Int",
            Stat::Sag => "Sag",
            Stat::Cha => "Cha",
            Stat::Luk => "Luk",
        }
    }
}

/// Valeurs des 8 stats. Négatif affiché comme 1(-x).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CharacterStats {
    pub for_: i32,
    pub con: i32,
    pub agi: i32,
    pub dex: i32,
    pub int: i32,
    pub sag: i32,
    pub cha: i32,
    pub luk: i32,
}

impl CharacterStats {
    pub fn get(&self, s: Stat) -> i32 {
        match s {
            Stat::For => self.for_,
            Stat::Con => self.con,
            Stat::Agi => self.agi,
            Stat::Dex => self.dex,
            Stat::Int => self.int,
            Stat::Sag => self.sag,
            Stat::Cha => self.cha,
            Stat::Luk => self.luk,
        }
    }

    pub fn add(&mut self, s: Stat, delta: i32) {
        match s {
            Stat::For => self.for_ += delta,
            Stat::Con => self.con += delta,
            Stat::Agi => self.agi += delta,
            Stat::Dex => self.dex += delta,
            Stat::Int => self.int += delta,
            Stat::Sag => self.sag += delta,
            Stat::Cha => self.cha += delta,
            Stat::Luk => self.luk += delta,
        }
    }

    /// Affichage : si négatif "1(-x)", sinon la valeur.
    pub fn display(&self, s: Stat) -> String {
        let v = self.get(s);
        if v < 0 {
            format!("1({})", v)
        } else {
            v.to_string()
        }
    }

    /// Relance toutes les stats dans [-3, 6] (Que Nawak décide).
    pub fn reroll_all(&mut self, roll: &mut impl FnMut() -> f32) {
        for s in [
            Stat::For,
            Stat::Con,
            Stat::Agi,
            Stat::Dex,
            Stat::Int,
            Stat::Sag,
            Stat::Cha,
            Stat::Luk,
        ] {
            let v = (roll() * 10.0).floor() as i32 - 3; // 0..10 -> -3..6
            *self.get_mut(s) = v.clamp(-3, 6);
        }
    }

    fn get_mut(&mut self, s: Stat) -> &mut i32 {
        match s {
            Stat::For => &mut self.for_,
            Stat::Con => &mut self.con,
            Stat::Agi => &mut self.agi,
            Stat::Dex => &mut self.dex,
            Stat::Int => &mut self.int,
            Stat::Sag => &mut self.sag,
            Stat::Cha => &mut self.cha,
            Stat::Luk => &mut self.luk,
        }
    }
}

/// Effet d'une phrase sur les stats.
#[derive(Debug, Clone)]
pub enum PhraseEffect {
    StatBonus(Stat, i32, i32), // stat, min, max (delta)
    AllPlus1,
    RerollAllEnd,
}

/// Une phrase de création avec ses effets.
#[derive(Debug, Clone)]
pub struct PhraseDef {
    pub id: usize,
    pub text: &'static str,
    pub effects: Vec<PhraseEffect>,
}

fn roll_in(roll: &mut impl FnMut() -> f32, min: i32, max: i32) -> i32 {
    if min >= max {
        return min;
    }
    let range = (max - min + 1) as f32;
    min + (roll() * range).floor() as i32
}

/// Applique les effets d'une phrase aux stats (roll pour les tirages).
/// Retourne true si la phrase contenait RerollAllEnd (à appliquer en fin de création).
pub fn apply_phrase_effects(stats: &mut CharacterStats, effects: &[PhraseEffect], roll: &mut impl FnMut() -> f32) -> bool {
    let mut reroll_at_end = false;
    for e in effects {
        match e {
            PhraseEffect::StatBonus(stat, min, max) => {
                stats.add(*stat, roll_in(roll, *min, *max));
            }
            PhraseEffect::AllPlus1 => {
                stats.for_ += 1;
                stats.con += 1;
                stats.agi += 1;
                stats.dex += 1;
                stats.int += 1;
                stats.sag += 1;
                stats.cha += 1;
                stats.luk += 1;
            }
            PhraseEffect::RerollAllEnd => {
                reroll_at_end = true;
            }
        }
    }
    reroll_at_end
}

/// Pool de toutes les phrases (id = index).
pub fn all_phrases() -> Vec<PhraseDef> {
    vec![
        PhraseDef {
            id: 0,
            text: "Je fais du sport avec mon gros shaker de wey.",
            effects: vec![PhraseEffect::StatBonus(Stat::For, 1, 3), PhraseEffect::StatBonus(Stat::Con, 1, 3)],
        },
        PhraseDef {
            id: 1,
            text: "Je suis un(e) vrai cleptomane.",
            effects: vec![PhraseEffect::StatBonus(Stat::Agi, 1, 3), PhraseEffect::StatBonus(Stat::Dex, 1, 3)],
        },
        PhraseDef {
            id: 2,
            text: "Balékouille frère!",
            effects: vec![PhraseEffect::StatBonus(Stat::Int, -1, -1), PhraseEffect::StatBonus(Stat::Con, 1, 1)],
        },
        PhraseDef {
            id: 3,
            text: "Parce que 42!",
            effects: vec![PhraseEffect::StatBonus(Stat::Int, 1, 5)],
        },
        PhraseDef {
            id: 4,
            text: "Je sais ce qu'est \"Les fleurs du mal\".",
            effects: vec![PhraseEffect::StatBonus(Stat::Int, 1, 3), PhraseEffect::StatBonus(Stat::Sag, 1, 3)],
        },
        PhraseDef {
            id: 5,
            text: "Mes parents m'ont vendu pour une barrette de RAM.",
            effects: vec![PhraseEffect::StatBonus(Stat::Luk, 2, 6)],
        },
        PhraseDef {
            id: 6,
            text: "Je mange mes crottes de nez.",
            effects: vec![PhraseEffect::StatBonus(Stat::For, 1, 1), PhraseEffect::StatBonus(Stat::Int, -1, -1)],
        },
        PhraseDef {
            id: 7,
            text: "The emperor protects!",
            effects: vec![PhraseEffect::StatBonus(Stat::Cha, 1, 1), PhraseEffect::StatBonus(Stat::Con, 1, 1)],
        },
        PhraseDef {
            id: 8,
            text: "Je suis le sosie d'Henry Cavill.",
            effects: vec![
                PhraseEffect::StatBonus(Stat::Cha, 3, 6),
                PhraseEffect::StatBonus(Stat::For, -1, -1),
                PhraseEffect::StatBonus(Stat::Int, -1, -1),
            ],
        },
        PhraseDef {
            id: 9,
            text: "La mère me tape avec des claquettes.",
            effects: vec![
                PhraseEffect::StatBonus(Stat::Agi, 3, 6),
                PhraseEffect::StatBonus(Stat::Cha, -1, -1),
                PhraseEffect::StatBonus(Stat::Luk, -1, -1),
            ],
        },
        PhraseDef {
            id: 10,
            text: "J'aime manger épicé.",
            effects: vec![PhraseEffect::StatBonus(Stat::Cha, 1, 1), PhraseEffect::StatBonus(Stat::Con, 1, 1)],
        },
        PhraseDef {
            id: 11,
            text: "Pizza à l'ananas.",
            effects: vec![PhraseEffect::StatBonus(Stat::Dex, 1, 1)],
        },
        PhraseDef {
            id: 12,
            text: "Ratatatatatatatatata...",
            effects: vec![PhraseEffect::StatBonus(Stat::Agi, 2, 2), PhraseEffect::StatBonus(Stat::Dex, 2, 2)],
        },
        PhraseDef {
            id: 13,
            text: "WAAAAAAAARG",
            effects: vec![
                PhraseEffect::StatBonus(Stat::For, 4, 4),
                PhraseEffect::StatBonus(Stat::Con, 3, 3),
                PhraseEffect::StatBonus(Stat::Agi, 1, 1),
                PhraseEffect::StatBonus(Stat::Int, -4, -4),
                PhraseEffect::StatBonus(Stat::Sag, -2, -2),
                PhraseEffect::StatBonus(Stat::Cha, -3, -3),
                PhraseEffect::StatBonus(Stat::Luk, 3, 3),
            ],
        },
        PhraseDef {
            id: 14,
            text: "Je m'appelle Pauline et je fais de l'équitation.",
            effects: vec![
                PhraseEffect::StatBonus(Stat::Agi, 2, 2),
                PhraseEffect::StatBonus(Stat::Dex, 2, 2),
                PhraseEffect::StatBonus(Stat::Cha, -1, -1),
            ],
        },
        PhraseDef {
            id: 15,
            text: "Marche ou crève.",
            effects: vec![
                PhraseEffect::StatBonus(Stat::Con, 3, 3),
                PhraseEffect::StatBonus(Stat::For, 2, 2),
                PhraseEffect::StatBonus(Stat::Cha, 1, 1),
            ],
        },
        PhraseDef {
            id: 16,
            text: "Que Nawak décide...",
            effects: vec![PhraseEffect::RerollAllEnd],
        },
        PhraseDef {
            id: 17,
            text: "Ta soeur est ta mère.",
            effects: vec![
                PhraseEffect::StatBonus(Stat::For, 1, 1),
                PhraseEffect::StatBonus(Stat::Con, 1, 1),
                PhraseEffect::StatBonus(Stat::Agi, 1, 1),
                PhraseEffect::StatBonus(Stat::Dex, 1, 1),
                PhraseEffect::StatBonus(Stat::Int, -1, -1),
                PhraseEffect::StatBonus(Stat::Sag, -1, -1),
                PhraseEffect::StatBonus(Stat::Cha, -1, -1),
                PhraseEffect::StatBonus(Stat::Luk, -1, -1),
            ],
        },
        PhraseDef {
            id: 18,
            text: "J'ai marché sur une crotte.",
            effects: vec![PhraseEffect::StatBonus(Stat::Luk, -2, 4)],
        },
        PhraseDef {
            id: 19,
            text: "J'ai été délégué de classe.",
            effects: vec![PhraseEffect::StatBonus(Stat::Int, 2, 2), PhraseEffect::StatBonus(Stat::Cha, 2, 2)],
        },
        PhraseDef {
            id: 20,
            text: "Why are you gay?",
            effects: vec![],
        },
        PhraseDef {
            id: 21,
            text: "8 blessés et 6 morts, 86 ma lubulule.",
            effects: vec![],
        },
        PhraseDef {
            id: 22,
            text: "Avec Véga Myssil, vous êtes satellisé!",
            effects: vec![
                PhraseEffect::StatBonus(Stat::Agi, 4, 4),
                PhraseEffect::StatBonus(Stat::Dex, 2, 2),
                PhraseEffect::StatBonus(Stat::Cha, 1, 1),
            ],
        },
        PhraseDef {
            id: 23,
            text: "Je suis banal(e).",
            effects: vec![PhraseEffect::AllPlus1],
        },
        PhraseDef {
            id: 24,
            text: "Mangez 5 fruits et légumes par jour.",
            effects: vec![PhraseEffect::AllPlus1],
        },
    ]
}

/// Choisit 3 indices aléatoires parmi `available` (sans remise). Retourne les 3 phrases et les indices retirés de available.
pub fn pick_three_phrases(
    available: &mut Vec<usize>,
    roll: &mut impl FnMut() -> f32,
) -> Vec<PhraseDef> {
    let pool = all_phrases();
    let mut chosen = Vec::with_capacity(3);
    for _ in 0..3 {
        if available.is_empty() {
            break;
        }
        let idx = (roll() * available.len() as f32).floor() as usize;
        let phrase_id = available.remove(idx);
        if let Some(p) = pool.iter().find(|x| x.id == phrase_id) {
            chosen.push(p.clone());
        }
    }
    chosen
}
