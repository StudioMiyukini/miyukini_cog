//! Adaptateurs inter-Services JayKoa — lecture réfléchie.
//!
//! @id: jaykoa_services_module
//! @do: expose_inter_service_sync_adapters
//! @layer: infra
//! INVARIANTS : Aucun booking, aucune logique métier externe, lecture seule stricte.

/// Adaptateur JayRDV → JayKoa (rendez-vous confirmés).
pub mod jayrdv;
