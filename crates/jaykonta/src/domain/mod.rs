//! @id     jaykonta_domain_mod
//! @do     declarer_module_account
//! @role   orchestration
//! @layer  core
//! @human  jaykonta — module « domain mod ».

//! Types de domaine JayKonta.

/// Types Account : contrepartie, devis, factures, paiements.
pub mod account;
/// Types de dashboard et de contrats (existant — vue d'ensemble).
pub mod model;
/// Types Purse : compte, mouvements, budgets, objectifs, alertes.
pub mod purse;
