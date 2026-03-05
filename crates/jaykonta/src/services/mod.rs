//! Services metier JayKonta.
//!
//! Chaque service encapsule la logique metier d'un domaine fonctionnel,
//! en delegant la persistance a `JayKontaDb` (couche data).

/// Service Audit : journalisation des ecritures (CK-AUD-01).
pub mod audit_service;
/// Service Purse : dashboard, mouvements, categories.
pub mod purse_service;
