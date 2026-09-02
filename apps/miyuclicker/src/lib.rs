//! @id     miyuclicker_lib
//! @do     declarer_module_views
//! @role   orchestration
//! @layer  app
//! @human  miyuclicker — module « lib ».

//! MiyuClicker (Lord of the Click) — App standalone Dioxus.
pub mod views;

use dioxus::prelude::*;
use std::path::PathBuf;

/// Contexte pour le repertoire de donnees du jeu.
#[derive(Clone)]
pub struct DataDirContext {
    pub data_dir: PathBuf,
}

/// Hook pour acceder au data_dir.
pub fn use_data_dir() -> PathBuf {
    use_context::<Signal<DataDirContext>>()
        .read()
        .data_dir
        .clone()
}
