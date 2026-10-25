//! @id     maia_modeles_politique
//! @do     autoriser_chargement_modele
//! @role   securite
//! @layer  toolkit
//! @human  maia-modeles — politique de chargement des modèles : ancres épinglées, dérogation « non signés » décidée par le binaire, jamais par la bibliothèque.

//! Politique de chargement des modèles (EX-26).
//!
//! # Dérogation développeur
//!
//! `--dev-allow-unsigned-models` ou `MAIA_LLM_DEV_ALLOW_UNSIGNED_MODELS=1`
//! **demande** l'admission d'un modèle non signé ([`derogation_demandee`]).
//! L'accorder ou non est une décision du **binaire** : il appelle
//! [`derogation_effective`] avec **son** `cfg!(debug_assertions)` et passe le
//! résultat à [`PolitiqueModeles::new`]. La bibliothèque ne consulte jamais
//! son propre profil de compilation (une dépendance peut être compilée en
//! debug sous un binaire de release, et inversement).

use std::path::Path;

use miyusceau::AncresDeConfiance;

use crate::fichier::{verifier_modele_ouvert, ModeleOuvert};
use crate::manifeste::{ErreurProvenance, ManifesteModele};

/// Variable d'environnement de dérogation développeur.
pub const ENV_DEROGATION: &str = "MAIA_LLM_DEV_ALLOW_UNSIGNED_MODELS";
/// Option de ligne de commande de dérogation développeur.
pub const OPTION_DEROGATION: &str = "--dev-allow-unsigned-models";

/// Vrai si la dérogation développeur est **demandée** (option ou variable).
/// Une demande n'est pas une autorisation : voir [`derogation_effective`].
pub fn derogation_demandee(args: &[String]) -> bool {
    args.iter().any(|a| a == OPTION_DEROGATION)
        || std::env::var(ENV_DEROGATION).is_ok_and(|v| v == "1")
}

/// Décision du binaire : la dérogation n'est accordée que si elle est
/// demandée **et** que le binaire appelant est un binaire de debug
/// (`binaire_de_debug` = `cfg!(debug_assertions)` évalué **dans le binaire**).
/// Une demande refusée est journalisée.
pub fn derogation_effective(demandee: bool, binaire_de_debug: bool) -> bool {
    let accordee = demandee && binaire_de_debug;
    if demandee && !accordee {
        tracing::error!(
            "Dérogation « modèles non signés » demandée mais IGNORÉE : binaire de release \
             (EX-26). Seuls les modèles signés seront chargés."
        );
    }
    accordee
}

/// Politique de chargement des modèles, fixée au démarrage.
#[derive(Debug, Clone)]
pub struct PolitiqueModeles {
    ancres: AncresDeConfiance,
    derogation_dev: bool,
}

/// Issue d'un contrôle de chargement autorisé.
#[derive(Debug, Clone)]
pub enum Autorisation {
    /// Modèle vérifié (manifeste signé, empreinte conforme).
    Verifie(ManifesteModele),
    /// Modèle non vérifié, admis par la dérogation développeur.
    DerogationDev(String),
}

impl PolitiqueModeles {
    /// Construit la politique. `derogation_dev` est la décision **déjà prise
    /// par le binaire** ([`derogation_effective`]) : la bibliothèque l'applique
    /// telle quelle (journal d'erreur si active).
    pub fn new(ancres: AncresDeConfiance, derogation_dev: bool) -> Self {
        if derogation_dev {
            tracing::error!(
                "!!! DÉROGATION DÉVELOPPEUR ACTIVE : des modèles NON SIGNÉS peuvent être chargés \
                 (EX-26 non respectée). Ne jamais utiliser en production. !!!"
            );
        }
        if ancres.is_empty() {
            tracing::warn!(
                "Aucune ancre de confiance de modèles (model_trust_anchors) : \
                 aucun modèle signé ne peut être chargé."
            );
        }
        Self {
            ancres,
            derogation_dev,
        }
    }

    /// Vrai si la dérogation développeur est active.
    pub fn derogation_active(&self) -> bool {
        self.derogation_dev
    }

    /// Autorise (ou refuse) le chargement de `modele`.
    ///
    /// Rend le fichier ouvert et vérifié : le chargeur doit utiliser
    /// [`ModeleOuvert::chemin_de_chargement`] et appeler
    /// [`ModeleOuvert::inchange`] avant et après le chargement. L'ouverture
    /// (lien symbolique, fichier absent, droits) est refusée même sous dérogation.
    pub fn autoriser(
        &self,
        modele: &Path,
    ) -> Result<(Autorisation, ModeleOuvert), ErreurProvenance> {
        let mut ouvert = ModeleOuvert::ouvrir(modele)?;
        match verifier_modele_ouvert(&mut ouvert, &self.ancres) {
            Ok(m) => {
                tracing::info!(
                    modele = %m.nom_fichier,
                    licence = %m.licence,
                    coupure = %m.date_coupure,
                    "provenance du modèle vérifiée"
                );
                Ok((Autorisation::Verifie(m), ouvert))
            }
            Err(e) if self.derogation_dev => {
                tracing::error!(
                    "!!! MODÈLE NON VÉRIFIÉ CHARGÉ PAR DÉROGATION DÉVELOPPEUR : {} — {e} !!!",
                    modele.display()
                );
                Ok((Autorisation::DerogationDev(e.to_string()), ouvert))
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::essai::{ancres, cle, dossier, modele};
    use crate::outillage::sceller_modele;

    #[test]
    fn modele_signe_autorise() {
        let d = dossier("politique");
        let p = modele(&d);
        let c = cle();
        sceller_modele(&p, &c, "Apache-2.0", "2025-06").unwrap();
        let pol = PolitiqueModeles::new(ancres(&c), false);
        assert!(!pol.derogation_active());
        assert!(matches!(
            pol.autoriser(&p),
            Ok((Autorisation::Verifie(_), _))
        ));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn non_signe_refuse_sans_derogation() {
        let d = dossier("politique");
        let p = modele(&d);
        let pol = PolitiqueModeles::new(ancres(&cle()), false);
        assert!(matches!(
            pol.autoriser(&p),
            Err(ErreurProvenance::ManifesteAbsent(_))
        ));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn derogation_decidee_par_le_binaire_pas_par_la_bibliotheque() {
        // Demande + binaire de release ⇒ refusée ; seul un binaire de debug l'accorde.
        assert!(!derogation_effective(true, false));
        assert!(derogation_effective(true, true));
        assert!(!derogation_effective(false, true));
        assert!(!derogation_effective(false, false));
        // La politique applique le booléen reçu, quel que soit le profil de la bibliothèque.
        let d = dossier("politique");
        let p = modele(&d);
        let active = PolitiqueModeles::new(AncresDeConfiance::new(), true);
        assert!(active.derogation_active());
        assert!(matches!(
            active.autoriser(&p),
            Ok((Autorisation::DerogationDev(_), _))
        ));
        let inactive = PolitiqueModeles::new(AncresDeConfiance::new(), false);
        assert!(inactive.autoriser(&p).is_err());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[cfg(unix)]
    #[test]
    fn lien_symbolique_refuse_meme_sous_derogation() {
        let d = dossier("politique");
        let p = modele(&d);
        let lien = d.join("lien.gguf");
        std::os::unix::fs::symlink(&p, &lien).unwrap();
        let pol = PolitiqueModeles::new(ancres(&cle()), true);
        assert!(matches!(
            pol.autoriser(&lien),
            Err(ErreurProvenance::LectureModele(_))
        ));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn option_de_derogation_detectee() {
        assert!(derogation_demandee(&[
            "maia-llm".into(),
            OPTION_DEROGATION.into()
        ]));
    }
}
