//! Modules services réutilisés depuis Central via `#[path]`.
//!
//! Les fichiers sources restent dans `apps/central/src/services/` — on les
//! inclut directement pour éviter la duplication de code. Les macros
//! `include_str!` / `include_bytes!` résolvent toujours relativement au
//! fichier source original.

#[path = "../../../central/src/services/ui_assets.rs"]
pub mod ui_assets;

#[path = "../../../central/src/services/ui_builder.rs"]
pub mod ui_builder;
