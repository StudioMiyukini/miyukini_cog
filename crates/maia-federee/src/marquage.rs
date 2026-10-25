//! @id     maia_federee_marquage
//! @do     propager_marque_etranger_et_garder_effets
//! @role   securite
//! @layer  toolkit
//! @human  maia-federee — module « marquage » (valeurs `etranger`, validation utilisateur, gardes action / publication / mémoire / Règle / apprentissage).

//! Marquage d'origine (IA-FEDEREE §3, MEMOIRE-MAIA §10, EX-60, EX-63).
//!
//! - toute valeur issue d'une Fiche est un [`Etranger<T>`] : pas de `Deref`,
//!   pas de conversion vers `T` sans [`ValidationUtilisateur`] ; toute valeur
//!   dérivée (`map`) **reste** `etranger` ;
//! - [`autoriser`] est la garde unique des effets :
//!
//! | effet | `etranger` sans validation | `etranger` avec validation |
//! |-------|----------------------------|----------------------------|
//! | action (outil à effet) | refus | autorisé si la validation couvre l'action exacte |
//! | mémoire longue | refus | autorisé si la validation couvre l'écriture exacte |
//! | publication | refus | autorisé si la validation est **confirmée par TAMR** et couvre la Fiche exacte (A-009) |
//! | Règle | **interdit** | **interdit** (MEMOIRE-MAIA §10) |
//! | apprentissage | **interdit** | **interdit** (EX-63) |

use sha2::{Digest, Sha256};

use crate::erreurs::ErreurFederee;

/// Provenance d'une donnée (MEMOIRE-MAIA §3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Provenance {
    /// Dite par l'utilisateur.
    Utilisateur,
    /// Déduite localement.
    Deduit,
    /// Issue d'une Fiche fédérée ou d'un document relayé : non fiable.
    Etranger,
}

impl Provenance {
    /// Nom stable (journaux, mémoire).
    pub const fn nom(self) -> &'static str {
        match self {
            Self::Utilisateur => "utilisateur",
            Self::Deduit => "deduit",
            Self::Etranger => "etranger",
        }
    }
}

/// Valeur d'origine étrangère. La marque ne se perd jamais : seule une
/// [`ValidationUtilisateur`] couvrant l'usage exact permet d'en sortir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Etranger<T>(T);

impl<T> Etranger<T> {
    /// Marque une valeur (réservé à la crate : seules les lectures de Fiches marquent).
    pub(crate) fn marquer(valeur: T) -> Self {
        Self(valeur)
    }

    /// Accès en lecture, pour affichage délimité à l'utilisateur uniquement.
    pub fn valeur_non_fiable(&self) -> &T {
        &self.0
    }

    /// Provenance (toujours `etranger`).
    pub const fn provenance(&self) -> Provenance {
        Provenance::Etranger
    }

    /// Valeur dérivée : la marque est propagée.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Etranger<U> {
        Etranger(f(self.0))
    }

    /// Sort la valeur de la marque pour l'effet `effet`, décrit exactement par
    /// `description`, si l'utilisateur l'a validé (voir [`autoriser`]).
    pub fn valider(self, effet: Effet, description: &str, validation: &ValidationUtilisateur) -> Result<T, ErreurFederee> {
        autoriser(Provenance::Etranger, effet, Some(validation), description)?;
        Ok(self.0)
    }
}

/// Effet soumis à la garde.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effet {
    /// Appel d'outil à effet, argument d'outil sensible.
    Action,
    /// Publication (Fiche, message, partage).
    Publication,
    /// Écriture en mémoire longue de MAIA.
    MemoireLongue,
    /// Création d'une Règle (consigne durable).
    Regle,
    /// Apprentissage automatique (fine-tuning, mémoire apprise).
    Apprentissage,
}

/// Validation explicite de l'utilisateur, liée à **une** action exacte
/// (empreinte de sa description canonique) et à l'identifiant de la
/// confirmation (registre des confirmations de MAIA, puis décision TAMR).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationUtilisateur {
    confirmation: String,
    empreinte: [u8; 32],
    tamr: bool,
}

impl ValidationUtilisateur {
    /// À construire **uniquement** par le chemin de confirmation humaine
    /// (étape séparée qui a affiché `description` à l'utilisateur).
    pub fn nouvelle(confirmation: &str, description: &str) -> Self {
        Self { confirmation: confirmation.to_string(), empreinte: Sha256::digest(description.as_bytes()).into(), tamr: false }
    }

    /// Validation de l'utilisateur **confirmée par TAMR** (décision StrongFather
    /// `APPROVAL`) : à construire uniquement par le chemin TAMR (A-009).
    pub fn confirmee_par_tamr(confirmation: &str, description: &str) -> Self {
        Self { tamr: true, ..Self::nouvelle(confirmation, description) }
    }

    /// Vrai si la validation a été confirmée par TAMR.
    pub fn par_tamr(&self) -> bool {
        self.tamr
    }

    /// Identifiant de la confirmation.
    pub fn confirmation(&self) -> &str {
        &self.confirmation
    }

    /// Vrai si la validation porte exactement sur `description`.
    pub fn couvre(&self, description: &str) -> bool {
        let e: [u8; 32] = Sha256::digest(description.as_bytes()).into();
        e == self.empreinte
    }
}

/// Garde unique des effets selon la provenance (EX-60, EX-63).
pub fn autoriser(
    provenance: Provenance,
    effet: Effet,
    validation: Option<&ValidationUtilisateur>,
    description: &str,
) -> Result<(), ErreurFederee> {
    if provenance != Provenance::Etranger {
        return Ok(());
    }
    match effet {
        Effet::Publication => match validation {
            Some(v) if v.par_tamr() && v.couvre(description) => Ok(()),
            _ => Err(ErreurFederee::ValidationRequise),
        },
        Effet::Regle => Err(ErreurFederee::Interdit("Règle issue d'une donnée étrangère")),
        Effet::Apprentissage => Err(ErreurFederee::Interdit("apprentissage sur une donnée étrangère")),
        Effet::Action | Effet::MemoireLongue => match validation {
            Some(v) if v.couvre(description) => Ok(()),
            _ => Err(ErreurFederee::ValidationRequise),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_autorise_etranger_garde() {
        assert!(autoriser(Provenance::Utilisateur, Effet::Publication, None, "x").is_ok());
        for effet in [Effet::Action, Effet::MemoireLongue] {
            assert_eq!(autoriser(Provenance::Etranger, effet, None, "x"), Err(ErreurFederee::ValidationRequise));
        }
    }

    #[test]
    fn validation_liee_a_l_action_exacte() {
        let v = ValidationUtilisateur::nouvelle("c1", "file_ops {\"action\":\"write\"}");
        assert!(autoriser(Provenance::Etranger, Effet::Action, Some(&v), "file_ops {\"action\":\"write\"}").is_ok());
        assert_eq!(
            autoriser(Provenance::Etranger, Effet::Action, Some(&v), "shell_exec {}"),
            Err(ErreurFederee::ValidationRequise)
        );
    }

    #[test]
    fn publication_sous_tamr_seulement() {
        let simple = ValidationUtilisateur::nouvelle("c1", "republier x");
        let tamr = ValidationUtilisateur::confirmee_par_tamr("c1", "republier x");
        assert_eq!(autoriser(Provenance::Etranger, Effet::Publication, None, "republier x"), Err(ErreurFederee::ValidationRequise));
        assert_eq!(autoriser(Provenance::Etranger, Effet::Publication, Some(&simple), "republier x"), Err(ErreurFederee::ValidationRequise));
        assert_eq!(autoriser(Provenance::Etranger, Effet::Publication, Some(&tamr), "republier y"), Err(ErreurFederee::ValidationRequise));
        assert!(autoriser(Provenance::Etranger, Effet::Publication, Some(&tamr), "republier x").is_ok());
    }

    #[test]
    fn regle_apprentissage_toujours_interdits() {
        let v = ValidationUtilisateur::confirmee_par_tamr("c1", "d");
        for effet in [Effet::Regle, Effet::Apprentissage] {
            assert!(matches!(autoriser(Provenance::Etranger, effet, Some(&v), "d"), Err(ErreurFederee::Interdit(_))));
        }
    }

    #[test]
    fn marque_propagee() {
        let e = Etranger::marquer("Paris".to_string()).map(|s| s.len());
        assert_eq!(e.provenance(), Provenance::Etranger);
        assert_eq!(e.clone().valider(Effet::MemoireLongue, "m", &ValidationUtilisateur::nouvelle("c", "autre")).unwrap_err(), ErreurFederee::ValidationRequise);
        assert_eq!(e.valider(Effet::MemoireLongue, "m", &ValidationUtilisateur::nouvelle("c", "m")).unwrap(), 5);
    }
}
