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
