//! @id     jaycloud_files_davfs
//! @do     fournir_files_davfs
//! @role   orchestration
//! @layer  core
//! @human  jaycloud — module « files davfs ».

//! Implémentation `dav-server::DavFileSystem` au-dessus de `files_op` + CAS.
//!
//! Implémentation en PR-4 (P3.c). Toute écriture passe par `borderguard`
//! (gate Cores) et émet des événements MiyukiniNotify pertinents.
