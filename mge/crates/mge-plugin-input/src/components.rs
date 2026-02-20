//! @id mge.plugin.input.v1.components
//! @domain input

use std::collections::HashSet;

use mge_ecs::Component;

/// @id mge.plugin.input.v1.component.input_state
/// @fields keys_pressed:HashSet<u32>,keys_just_pressed:HashSet<u32>
#[derive(Debug, Clone, Default)]
pub struct InputState {
    pub keys_pressed: HashSet<u32>,
    pub keys_just_pressed: HashSet<u32>,
}

/// @id mge.plugin.input.v1.component.key_binding
/// @fields action_id:u32,key_code:u32
#[derive(Debug, Clone)]
pub struct KeyBinding {
    pub action_id: u32,
    pub key_code: u32,
}

impl Component for InputState {}
impl Component for KeyBinding {}
