//! @id mge.plugin.save-load.v1.components
//! @domain persistence

use mge_ecs::Component;

/// @id mge.plugin.save-load.v1.component.save_state
/// @fields slot:u32,dirty:bool
#[derive(Debug, Clone)]
pub struct SaveState {
    pub slot: u32,
    pub dirty: bool,
}

/// @id mge.plugin.save-load.v1.component.snapshot
/// @fields serialized:Vec<u8>
#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    pub serialized: Vec<u8>,
}

impl Component for SaveState {}
impl Component for Snapshot {}
