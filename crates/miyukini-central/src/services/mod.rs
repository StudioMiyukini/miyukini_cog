//! @id     miyukini_central_services_mod
//! @do     fournir_services_mod
//! @role   orchestration
//! @layer  core
//! @human  miyukini-central — module « services mod ».

//! Services du Hub — couche factice désactivée (migration Tauri + React/TypeScript).
//!
//! Les implémentations eframe ont été retirées. Les services sont gérés par l'app Tauri (apps/central).
//! Ce module conserve les types pour compatibilité API / catalog.

pub use crate::catalog::ServiceId;
