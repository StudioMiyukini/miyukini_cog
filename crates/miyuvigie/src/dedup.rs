//! @id     miyuvigie_dedup
//! @do     dedoublonner_evenements
//! @role   securite
//! @layer  toolkit
//! @human  miyuvigie — module « dedup » : doublons et rejeus d'événements.

//! Déduplication des événements d'un capteur.
//!
//! - **Doublon** : identifiant déjà vu (fenêtre bornée de `capacite` identifiants).
//! - **Rejeu** : horodatage antérieur de plus de `fenetre_ms` au plus récent
//!   horodatage accepté. Un identifiant évincé de la mémoire bornée ne peut donc
//!   pas être rejoué s'il est plus ancien que la fenêtre ; la capacité doit
//!   couvrir le volume d'une fenêtre (sinon des doublons récents évincés
//!   seraient acceptés — dimensionnement à la charge de l'appelant).
//!
//! Le temps est celui des événements (aucune horloge lue ici).

use std::collections::{HashSet, VecDeque};

use crate::signal::EvenementId;

/// Verdict de déduplication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Admission {
    /// Événement nouveau.
    Nouveau,
    /// Identifiant déjà vu.
    Doublon,
    /// Trop ancien par rapport au plus récent vu.
    Rejeu,
}

/// Mémoire bornée des identifiants vus.
#[derive(Debug, Clone)]
pub struct Dedoublonneur {
    capacite: usize,
    fenetre_ms: u64,
    vus: HashSet<EvenementId>,
    ordre: VecDeque<EvenementId>,
    plus_recent_ms: Option<u64>,
}

impl Dedoublonneur {
    /// Crée une mémoire (`capacite` ≥ 1).
    pub fn new(capacite: usize, fenetre_ms: u64) -> Self {
        Self {
            capacite: capacite.max(1),
            fenetre_ms,
            vus: HashSet::new(),
            ordre: VecDeque::new(),
            plus_recent_ms: None,
        }
    }

    /// Admet (ou non) un événement et le mémorise s'il est nouveau.
    pub fn admettre(&mut self, id: &EvenementId, horodatage_ms: u64) -> Admission {
        if self.vus.contains(id) {
            return Admission::Doublon;
        }
        if let Some(recent) = self.plus_recent_ms {
            if horodatage_ms.saturating_add(self.fenetre_ms) < recent {
                return Admission::Rejeu;
            }
        }
        if self.ordre.len() >= self.capacite {
            if let Some(ancien) = self.ordre.pop_front() {
                self.vus.remove(&ancien);
            }
        }
        self.vus.insert(id.clone());
        self.ordre.push_back(id.clone());
        self.plus_recent_ms = Some(
            self.plus_recent_ms
                .map_or(horodatage_ms, |r| r.max(horodatage_ms)),
        );
        Admission::Nouveau
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(s: &str) -> EvenementId {
        EvenementId::new(s).unwrap()
    }

    #[test]
    fn doublon_et_rejeu() {
        let mut d = Dedoublonneur::new(2, 1000);
        assert_eq!(d.admettre(&id("a"), 10_000), Admission::Nouveau);
        assert_eq!(d.admettre(&id("a"), 10_000), Admission::Doublon);
        assert_eq!(d.admettre(&id("b"), 10_500), Admission::Nouveau);
        assert_eq!(d.admettre(&id("c"), 11_000), Admission::Nouveau);
        // « a » est évincé, mais trop ancien : rejeu.
        assert_eq!(d.admettre(&id("a"), 9_000), Admission::Rejeu);
        // Dans la fenêtre : accepté (hors mémoire).
        assert_eq!(d.admettre(&id("d"), 10_200), Admission::Nouveau);
    }
}
