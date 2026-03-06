use serde::{Deserialize, Serialize};

/// An entity identifier (lightweight, ECS-style).
pub type EntityId = u64;

/// A world-space position (tile coordinates).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct WorldPos {
    pub x: f32,
    pub y: f32,
}

impl WorldPos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance_sq(self, other: Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    pub fn distance(self, other: Self) -> f32 {
        self.distance_sq(other).sqrt()
    }
}

/// Movement mode for the character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveMode {
    Walk,
    Run,
}

/// The action a character is currently executing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacterAction {
    Idle,
    Moving { destination: WorldPos, mode: MoveMode },
    Attacking { target: EntityId, windup_remaining: f32 },
    UsingSkill { skill_id: u32, target: SkillTarget, cast_remaining: f32 },
    Interacting { target: EntityId },
    Dead,
    InTown,
}

/// What a skill is aimed at.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillTarget {
    Entity(EntityId),
    Position(WorldPos),
    Self_,
}

/// Command sent to a character (from player or AI).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionCommand {
    MoveTo(WorldPos),
    RunTo(WorldPos),
    AttackEntity(EntityId),
    UseSkill { skill_id: u32, target: SkillTarget },
    Interact(EntityId),
    PickupItem(EntityId),
    ForceStand,
    EnterTown,
}

/// Character action state machine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionState {
    pub current: CharacterAction,
    pub position: WorldPos,
    pub move_speed: f32,
}

impl ActionState {
    pub fn new(position: WorldPos) -> Self {
        Self { current: CharacterAction::Idle, position, move_speed: 6.0 }
    }

    /// Process a command and transition to the appropriate action.
    pub fn apply_command(&mut self, cmd: ActionCommand) {
        match cmd {
            ActionCommand::MoveTo(dest) => {
                self.current = CharacterAction::Moving { destination: dest, mode: MoveMode::Walk };
            }
            ActionCommand::RunTo(dest) => {
                self.current = CharacterAction::Moving { destination: dest, mode: MoveMode::Run };
            }
            ActionCommand::AttackEntity(target) => {
                self.current = CharacterAction::Attacking { target, windup_remaining: 0.4 };
            }
            ActionCommand::UseSkill { skill_id, target } => {
                self.current =
                    CharacterAction::UsingSkill { skill_id, target, cast_remaining: 0.5 };
            }
            ActionCommand::Interact(target) => {
                self.current = CharacterAction::Interacting { target };
            }
            ActionCommand::ForceStand | ActionCommand::PickupItem(_) => {
                self.current = CharacterAction::Idle;
            }
            ActionCommand::EnterTown => {
                self.current = CharacterAction::InTown;
            }
        }
    }

    /// Advance the action by `dt` seconds. Returns any completed action event.
    pub fn tick(&mut self, dt: f32) -> Option<ActionComplete> {
        match &mut self.current {
            CharacterAction::Moving { destination, mode } => {
                let target_pos = *destination;
                let speed = match mode {
                    MoveMode::Walk => self.move_speed * 0.6,
                    MoveMode::Run => self.move_speed,
                };
                let remaining = self.position.distance(target_pos);
                let step = speed * dt;
                if step >= remaining {
                    self.position = target_pos;
                    self.current = CharacterAction::Idle;
                    return Some(ActionComplete::MovedTo(target_pos));
                }
                let dx = (target_pos.x - self.position.x) / remaining;
                let dy = (target_pos.y - self.position.y) / remaining;
                self.position.x += dx * step;
                self.position.y += dy * step;
                None
            }
            CharacterAction::Attacking { target, windup_remaining } => {
                *windup_remaining -= dt;
                if *windup_remaining <= 0.0 {
                    let t = *target;
                    self.current = CharacterAction::Idle;
                    return Some(ActionComplete::HitFrame(t));
                }
                None
            }
            CharacterAction::UsingSkill { skill_id, target, cast_remaining } => {
                *cast_remaining -= dt;
                if *cast_remaining <= 0.0 {
                    let id = *skill_id;
                    let t = target.clone();
                    self.current = CharacterAction::Idle;
                    return Some(ActionComplete::SkillReleased { skill_id: id, target: t });
                }
                None
            }
            CharacterAction::Interacting { target } => {
                let t = *target;
                self.current = CharacterAction::Idle;
                Some(ActionComplete::Interacted(t))
            }
            _ => None,
        }
    }

    pub fn is_idle(&self) -> bool {
        matches!(self.current, CharacterAction::Idle)
    }

    pub fn is_dead(&self) -> bool {
        matches!(self.current, CharacterAction::Dead)
    }
}

/// Events emitted when an action reaches its key moment.
#[derive(Debug, Clone)]
pub enum ActionComplete {
    MovedTo(WorldPos),
    HitFrame(EntityId),
    SkillReleased { skill_id: u32, target: SkillTarget },
    Interacted(EntityId),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_command_advances_position() {
        let mut state = ActionState::new(WorldPos::new(0.0, 0.0));
        state.apply_command(ActionCommand::RunTo(WorldPos::new(10.0, 0.0)));
        let result = state.tick(0.1);
        assert!(result.is_none()); // not arrived yet
        assert!(state.position.x > 0.0);
    }

    #[test]
    fn move_completes_when_arrived() {
        let mut state = ActionState::new(WorldPos::new(0.0, 0.0));
        state.apply_command(ActionCommand::RunTo(WorldPos::new(1.0, 0.0)));
        let result = state.tick(10.0);
        assert!(matches!(result, Some(ActionComplete::MovedTo(_))));
        assert!(state.is_idle());
    }

    #[test]
    fn attack_emits_hit_frame_after_windup() {
        let mut state = ActionState::new(WorldPos::new(0.0, 0.0));
        state.apply_command(ActionCommand::AttackEntity(42));
        let r1 = state.tick(0.2);
        assert!(r1.is_none());
        let r2 = state.tick(0.3);
        assert!(matches!(r2, Some(ActionComplete::HitFrame(42))));
    }

    #[test]
    fn skill_releases_after_cast_time() {
        let mut state = ActionState::new(WorldPos::new(0.0, 0.0));
        state.apply_command(ActionCommand::UseSkill {
            skill_id: 300,
            target: SkillTarget::Position(WorldPos::new(5.0, 5.0)),
        });
        let r = state.tick(1.0);
        assert!(matches!(r, Some(ActionComplete::SkillReleased { skill_id: 300, .. })));
    }

    #[test]
    fn interact_completes_immediately() {
        let mut state = ActionState::new(WorldPos::new(0.0, 0.0));
        state.apply_command(ActionCommand::Interact(99));
        let r = state.tick(0.016);
        assert!(matches!(r, Some(ActionComplete::Interacted(99))));
    }

    #[test]
    fn force_stand_sets_idle() {
        let mut state = ActionState::new(WorldPos::new(0.0, 0.0));
        state.apply_command(ActionCommand::AttackEntity(1));
        state.apply_command(ActionCommand::ForceStand);
        assert!(state.is_idle());
    }
}
