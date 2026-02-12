//! Bibliothèque JayShop — Service commerce, point de vente et vente en ligne.
//!
//! @id: jayshop_lib
//! @do: expose_public_modules_jayshop
//! @layer: infra
//!
//! JayShop s'appuie sur JayXpose pour le catalogue et les stocks,
//! et transmet les données comptables à JayKonta.
//! Intégration JayFestival pour la gestion des événements/festivals.

/// Persistance KindMother (libSQL via KindMother Service).
pub mod data;
/// Types et logique de domaine JayShop.
pub mod domain;
/// Intégrations inter-services (JayXpose, JayKonta, JayFestival).
pub mod integrations;
/// Services métier (ticket, paiement, caisse, rapports, sync, événements).
pub mod services;
