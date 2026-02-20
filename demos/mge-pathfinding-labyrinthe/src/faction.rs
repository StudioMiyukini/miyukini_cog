//! Système de factions et comportements d'aggro pour MGE Pathfinding.
//! Faction : Neutral (monstres, animaux), Empire, Alliance, Horde.
//! AggroBehavior : Aggressive (orange), Passive (jaune), Neutral (blanc).

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Faction {
    Neutral = 0, // Monstres, animaux
    Empire = 1,
    Alliance = 2,
    Horde = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggroBehavior {
    /// Orange — attaque autre faction en vision
    Aggressive,
    /// Jaune — riposte si attaque
    Passive,
    /// Blanc — animal, riposte si attaque
    Neutral,
}

impl Faction {
    /// Retourne true si cette faction est hostile à l'autre.
    /// Neutral n'est hostile à aucune faction.
    pub fn is_hostile_to(&self, other: &Faction) -> bool {
        *self != Faction::Neutral && *other != Faction::Neutral && *self != *other
    }
}
