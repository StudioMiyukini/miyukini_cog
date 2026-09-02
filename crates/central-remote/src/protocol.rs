//! @id     central_remote_protocol
//! @do     fournir_protocol
//! @role   orchestration
//! @layer  core
//! @human  central-remote — module « protocol ».

//! Protocole CentralRemote — re-export depuis `central-remote-protocol`.
//!
//! Les types sont définis dans le crate `central-remote-protocol` (léger, sans axum)
//! pour être partagés avec Central Mobile (Android).

pub use central_remote_protocol::*;
