//! @id     maia_memoire_lib
//! @do     exposer_memoire_maia
//! @role   securite
//! @layer  8
//! @human  maia-memoire — mémoire stratifiée de MAIA : N0–N4 sous budget de tokens, identifiants hiérarchiques, versions, provenance, oubli tracé, une base KindMother chiffrée par utilisateur.

//! # Mémoire de MAIA (lot 7)
//!
//! Spécification : `docs/architecture/MEMOIRE-MAIA.md` (gelé).
//!
//! - **EX-50** contexte assemblé sous budget de jetons **mesuré**, N0 toujours
//!   présent ([`assemblage::assembler`], [`jetons::Tokeniseur`]) ;
//! - **EX-51** identifiants hiérarchiques stables `domaine.sujet.f12`
//!   ([`ident`]), ouverture de branche à la demande ([`VueMaia::ouvrir`],
//!   outil `memoire.ouvrir`, lecture seule) ;
//! - **EX-52** notes versionnées en ajout seul ; seul l'oubli demandé efface,
//!   avec pierre tombale sans contenu ([`entrepot::Ecriture`]) ;
//! - **EX-53** provenance obligatoire `utilisateur` / `deduit` / `etranger` ;
//!   une proposition de MAIA reste candidate tant que l'utilisateur ne l'a pas
//!   validée ([`VueMaia::proposer`], [`AccesUtilisateur::valider`]) ;
//! - **EX-54** les Règles ne naissent que par [`AccesUtilisateur::poser_regle`] ;
//! - **EX-55** une base par utilisateur `maia-<utilisateur>` dans KindMother,
//!   chiffrée (SQLCipher, clé par base), étiquetée `nationale`
//!   ([`kindmother::EntrepotKindMother`]) ;
//! - **EX-56** consulter, corriger, exporter, oublier ([`AccesUtilisateur`]).
//!
//! Tout cas non spécifié est un refus (C-12).

pub mod assemblage;
pub mod entrepot;
pub mod erreurs;
pub mod ident;
pub mod jetons;
pub mod kindmother;
pub mod memoire;
#[cfg(feature = "pont-alicia")]
pub mod pont_alicia;
pub mod types;

pub use assemblage::{BlocMemoire, Strate};
pub use entrepot::{Entrepot, EntrepotMemoire};
pub use erreurs::ErreurMemoire;
pub use ident::{Branche, Chemin, IdMemoire, TypeUnite};
pub use jetons::{BorneOctets, Profil, Tokeniseur};
pub use kindmother::EntrepotKindMother;
pub use memoire::{AccesUtilisateur, ConfirmationTamr, Ecrit, Issue, MemoireMaia, Notee, Proposition, Validation, VueMaia};
/// Identifiant des candidats (réexporté pour les appelants sans dépendance directe).
pub use uuid::Uuid;
pub use types::{Candidat, EmpreinteSource, Etiquette, ExportMemoire, Note, PierreTombale, Provenance, ETIQUETTE};

/// Description de l'outil `memoire.ouvrir` (§7) au format des outils OpenAI,
/// pour l'enregistrement par le planificateur de MAIA. Lecture seule.
pub fn outil_ouvrir() -> serde_json::Value {
    serde_json::json!({
        "type": "function",
        "function": {
            "name": "memoire_ouvrir",
            "description": "Ouvre une branche ou une note de la mémoire de l'utilisateur (lecture seule). \
                            Argument : identifiant hiérarchique cité dans le bloc <memoire> (ex. travail.cog ou travail.cog.f12).",
            "parameters": {
                "type": "object",
                "properties": { "id": { "type": "string" } },
                "required": ["id"],
                "additionalProperties": false
            }
        }
    })
}
