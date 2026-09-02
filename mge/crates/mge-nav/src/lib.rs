//! @id     crates_mge_nav_lib
//! @do     definir_nav_cell
//! @role   orchestration
//! @layer  core
//! @human  crates mge-nav — module « lib ».

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavCell {
    pub x: i32,
    pub y: i32,
    pub walkable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NavGrid {
    pub cells: Vec<NavCell>,
}
