//! @id     miyufederation_note
//! @do     noter_pairs_localement
//! @role   securite
//! @layer  toolkit
//! @human  miyufederation — module « note » (note locale des pairs, ban score, jamais partagée, EX-37).

//! Note locale des pairs (EX-37, zones §16 « ban score indépendant de la
//! blacklist fédérale », repris de Bitcoin).
//!
//! - Points de faute cumulés par pair (`kid` de son identité), décroissance
//!   linéaire dans le temps ; banni localement à partir de [`SEUIL_BANNISSEMENT`]
//! - **Jamais partagée comme autorité** : aucune sérialisation, aucun export ;
//!   elle n'entre dans aucun titre signé et ne vaut ni confirmation ni
//!   accusation (seules les preuves signées + quorum révoquent)
//! - Barème et décroissance non fixés par la doc gelée : choix consignés pour
//!   un amendement

use std::collections::BTreeMap;

use miyusceau::Kid;

/// Points à partir desquels le pair est banni localement.
pub const SEUIL_BANNISSEMENT: u32 = 100;
/// Décroissance : points retirés par période.
pub const DECROISSANCE_POINTS: u32 = 10;
/// Période de décroissance (24 h).
pub const DECROISSANCE_PERIODE_SECS: u64 = 86_400;

/// Entrée de la note locale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Observation {
    /// Message mal formé, signature invalide, hors protocole (+20).
    ViolationProtocole,
    /// Faute grave (équivoque observée, titre forgé) (+100 : bannissement).
    ViolationGrave,
    /// Pair injoignable ou qui ne répond pas (+2).
    Indisponible,
    /// Échange réussi (−1, sans descendre sous 0).
    Disponible,
    /// Faute prouvée et corroborée par le quorum (+100 : bannissement).
    PreuveCorroboree,
    /// A confirmé un COG ensuite blacklisté (confirmateur responsable) (+25).
    ConfirmateurResponsable,
    /// A cosigné une accusation dont la preuve est fausse (+50).
    FauxSignalement,
}

impl Observation {
    /// Points ajoutés (le retrait de `Disponible` est traité à part).
    pub const fn points(self) -> u32 {
        match self {
            Self::ViolationProtocole => 20,
            Self::ViolationGrave | Self::PreuveCorroboree => 100,
            Self::Indisponible => 2,
            Self::Disponible => 0,
            Self::ConfirmateurResponsable => 25,
            Self::FauxSignalement => 50,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Fiche {
    points: u32,
    maj: u64,
}

impl Fiche {
    fn au(&self, maintenant: u64) -> u32 {
        let periodes = maintenant.saturating_sub(self.maj) / DECROISSANCE_PERIODE_SECS;
        let retrait = periodes.saturating_mul(u64::from(DECROISSANCE_POINTS));
        u32::try_from(u64::from(self.points).saturating_sub(retrait)).unwrap_or(0)
    }
}

/// Note locale de chaque pair (privée au COG).
#[derive(Debug, Clone, Default)]
pub struct NoteLocale {
    fiches: BTreeMap<Kid, Fiche>,
}

impl NoteLocale {
    /// Note vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enregistre une observation sur `pair`.
    pub fn observer(&mut self, pair: &Kid, observation: Observation, maintenant: u64) {
        let fiche = self.fiches.entry(*pair).or_insert(Fiche { points: 0, maj: maintenant });
        let actuels = fiche.au(maintenant);
        let points = if observation == Observation::Disponible {
            actuels.saturating_sub(1)
        } else {
            actuels.saturating_add(observation.points())
        };
        *fiche = Fiche { points, maj: maintenant.max(fiche.maj) };
    }

    /// Points actuels de `pair`.
    pub fn points(&self, pair: &Kid, maintenant: u64) -> u32 {
        self.fiches.get(pair).map_or(0, |f| f.au(maintenant))
    }

    /// Vrai si `pair` est banni localement.
    pub fn est_banni(&self, pair: &Kid, maintenant: u64) -> bool {
        self.points(pair, maintenant) >= SEUIL_BANNISSEMENT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bareme_et_decroissance() {
        let mut n = NoteLocale::new();
        let p = [1u8; 32];
        n.observer(&p, Observation::ViolationProtocole, 0);
        n.observer(&p, Observation::Indisponible, 0);
        assert_eq!(n.points(&p, 0), 22);
        n.observer(&p, Observation::Disponible, 0);
        assert_eq!(n.points(&p, 0), 21);
        assert_eq!(n.points(&p, 2 * 86_400), 1);
        assert!(!n.est_banni(&p, 0));
        n.observer(&p, Observation::ViolationGrave, 0);
        assert!(n.est_banni(&p, 0));
        assert!(!n.est_banni(&p, 3 * 86_400));
        assert_eq!(n.points(&[2; 32], 0), 0);
    }
}
