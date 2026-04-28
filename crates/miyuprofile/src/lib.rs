#![allow(missing_docs)]
//! # MiyuProfile — toolkit.identity.profile
//!
//! Banque de données personnelles sous gouvernance COG.
//!
//! ## Architecture
//!
//! Le profil Central regroupe toutes les informations qu'une assistante
//! personnelle a besoin pour effectuer les tâches déléguées par l'utilisateur :
//!
//! - **Identité** : état civil, nationalité, numéros officiels
//! - **Coordonnées** : adresses, téléphones, emails, web
//! - **Documents** : CNI, passeport, permis, diplômes (métadonnées)
//! - **Santé** : carnet de santé, ordonnances, médecins, allergies
//! - **Professionnel** : CV structuré, expériences, compétences
//! - **Entreprises** : SIRET, forme juridique, rôle
//! - **Contrats** : assurances, abonnements, baux, échéances
//! - **Finance** : comptes bancaires, moyens de paiement
//! - **Identifiants** : mots de passe chiffrés, codes d'accès
//! - **Vault** : stockage binaire chiffré (scans, signatures manuscrites)
//! - **Rappels** : échéances, renouvellements, expirations
//! - **Recherche** : index transversal sur toutes les sections
//! - **Export** : constitution de dossiers (inscription, candidature, etc.)
//!
//! Décision = StrongFather ; WriteIntent KindMother.
//! Alignement MIP : domaine `identity`, layer tool/toolkit.

// @id: toolkit.identity.miyuprofile
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuProfile Central ; expose les modules tools.
/// @do: expose_miyuprofile_toolkit

// ── Modules existants (rétro-compatibles) ────────────────────────
pub mod admin_cell;
pub mod avatar;
pub mod context;
pub mod errors;
pub mod field;
pub mod preferences;
pub mod profile;
pub mod rank;
pub mod signature;

// ── Nouveaux modules (profil Central) ────────────────────────────
pub mod export;
pub mod reminders;
pub mod search;
pub mod sections;
pub mod store;
pub mod vault;

// ── Ré-exports existants (rétro-compatibilité) ──────────────────
pub use admin_cell::{
    miyuprofile_admin_cell, MiyuprofileAdminCell, MiyuprofileIdentification, MiyuprofileIntegrity,
    MiyuprofileTestManifest, TOOLKIT_ID,
};
pub use avatar::{get as avatar_get, resolve as avatar_resolve, set as avatar_set};
pub use context::GovernedContext;
pub use errors::MiyuprofileError;
pub use field::{get as field_get, list as field_list, set as field_set};
pub use preferences::{get as preferences_get, set as preferences_set};
pub use profile::{get as profile_get, update as profile_update, ProfileData};
pub use rank::{list as rank_list, resolve as rank_resolve, RankItem};
pub use signature::{get as signature_get, set as signature_set};

// ── Ré-exports Central ──────────────────────────────────────────
pub use sections::CentralProfile;
pub use sections::SectionName;
pub use signature::{
    handwritten_delete as signature_handwritten_delete,
    handwritten_get as signature_handwritten_get,
    handwritten_set as signature_handwritten_set,
    HandwrittenSignature,
};
pub use vault::VaultEntry;
pub use reminders::{ReminderEntry, ReminderPriority, ReminderSource, ReminderStatus};
pub use search::{SearchOptions, SearchResult};
pub use export::{CompletenessReport, DossierTemplate};
