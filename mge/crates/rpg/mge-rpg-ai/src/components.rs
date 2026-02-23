//! @id mge.rpg.ai.v1.components
//! @role data
//! @layer plugin
//! @domain rpg

use mge_ecs::{Component, EntityId};

/// @id mge.rpg.ai.v1.component.creaturestate
/// @role data
/// @layer plugin
/// @do etat de la FSM creature
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreatureState {
    Idle,
    Chase,
    Attack,
    Return,
}

/// @id mge.rpg.ai.v1.component.aistate
/// @fields state:CreatureState,spawn_point:(f32,f32),aggro_radius:f32,leash_radius:f32,target:Option<EntityId>,attack_range:f32
/// @role data
/// @layer plugin
/// @do etat IA creature FSM (Idle->Chase->Attack, Return)
#[derive(Debug, Clone)]
pub struct AIState {
    pub state: CreatureState,
    pub spawn_point: (f32, f32),
    pub aggro_radius: f32,
    pub leash_radius: f32,
    pub target: Option<EntityId>,
    pub attack_range: f32,
}

impl AIState {
    pub fn new(spawn_point: (f32, f32), aggro_radius: f32, leash_radius: f32) -> Self {
        Self {
            state: CreatureState::Idle,
            spawn_point,
            aggro_radius,
            leash_radius,
            target: None,
            attack_range: 1.5,
        }
    }

    pub fn with_attack_range(mut self, range: f32) -> Self {
        self.attack_range = range;
        self
    }
}

impl Component for AIState {}

/// @id mge.rpg.ai.v1.component.aitargetable
/// @role data
/// @layer plugin
/// @do marque une entite comme cible potentielle pour l'IA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AiTargetable;

impl Component for AiTargetable {}
