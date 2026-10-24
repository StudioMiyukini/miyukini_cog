//! @id     miyukini_central_catalog
//! @do     definir_service_id
//! @role   orchestration
//! @layer  core
//! @human  miyukini-central — module « catalog ».

//! Catalogue des Services du Hub (identifiants).
//! Utilisé par l'app Tauri / API ; l'UI eGUI a été supprimée.

/// Identifiant d'un Service du Hub Central.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum ServiceId {
    JayKoa,
    JayKonta,
}
