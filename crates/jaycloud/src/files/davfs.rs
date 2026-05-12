//! Implémentation `dav-server::DavFileSystem` au-dessus de `files_op` + CAS.
//!
//! Implémentation en PR-4 (P3.c). Toute écriture passe par `borderguard`
//! (gate Cores) et émet des événements MiyukiniNotify pertinents.
