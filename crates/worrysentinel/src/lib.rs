//! @id     worrysentinel_lib
//! @do     declarer_module_degradation
//! @role   orchestration
//! @layer  core
//! @human  worrysentinel — module « lib ».

//! # WorrySentinel
//!
//! Détecteur de menaces et dégradation du Miyukini Core System.
//!
//! WorrySentinel a pour rôle de détecter les menaces, d'évaluer les niveaux de
//! sécurité et de gérer la dégradation progressive.
//!
//! ## État réel (K-17)
//!
//! Ce crate est un **modèle** : types (niveaux de sécurité, menaces, états de
//! dégradation) et gestionnaires en mémoire qui renvoient des valeurs fixées par
//! l'appelant. Ce n'est **pas** une gate active : aucune détection réelle n'a lieu
//! (`DefaultThreatDetector` ne fait que restituer les signaux qu'on lui fournit)
//! et le niveau courant ne pilote aucune frontière.
//!
//! Lot 3 : [`voies::SentinelleVoies`] consomme les rapports de CaringNanny
//! (signaux des Vigies, santé par voie) et décide, **par voie**, surveillance
//! ou coupure (EX-49), à côté de l'état global T0–T4 (escalade par étapes,
//! T4 jamais automatique). Les décisions sont explicables (motif, preuves) et
//! tracées dans un journal d'audit ; leur exécution est déléguée au
//! consommateur [`voies::CoupureVoie`] (le Portail), WorrySentinel n'exécutant
//! rien (INV-WS-2).
//!
//! Prévu, non implémenté : lien niveau WorrySentinel → contraintes StrongFather /
//! BorderGuard, soumission TAMR des décisions en T3, inscription du journal au
//! registre Merkle (lot 4) et OVERRIDE (constat K-17, à mettre au plan).

// C-10 : aucun unwrap / expect / panic! / todo! / unimplemented! / unreachable! hors tests.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::unreachable
    )
)]

pub mod degradation;
pub mod security_level;
pub mod threat_detector;
pub mod voies;

pub use degradation::{
    DefaultDegradationManager, Degradation, DegradationManager, DegradationState,
};
pub use security_level::{DefaultSecurityLevelManager, SecurityLevel, SecurityLevelManager};
pub use threat_detector::{DefaultThreatDetector, ThreatDetector, ThreatLevel};
pub use voies::{
    Action, CoupureVoie, Decision, ErreurCoupure, EtatVoie, EvenementAudit, Motif, NatureAudit,
    OrdreCoupure, PolitiqueVoies, Refus, ReponseVoieAveugle, SentinelleVoies,
};
