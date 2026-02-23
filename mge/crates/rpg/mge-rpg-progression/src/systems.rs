//! @id mge.rpg.progression.v1.systems
//! @role system
//! @layer plugin
//! @domain rpg

use mge_core::Context;
use mge_ecs::{EntityId, World};

use crate::components::{SkillLock, SkillSet};
use crate::events::{SkillCheckEvent, SkillGainEvent};

/// Cap individuel par défaut d'une compétence (équivalent 100.0 en notation UO).
const DEFAULT_SKILL_CAP: f64 = 100.0;

/// Système de gain de compétence par usage.
/// Consomme les SkillCheckEvent du tick précédent.
/// Formule simplifiée : gc = room × gain_factor × (0.5 + 0.5*success)
/// Si roll < gc : base += 0.1 et émission SkillGainEvent.
pub fn skill_gain_system(world: &mut World, ctx: &mut Context) {
    let mut to_process: Vec<(EntityId, u32, f64, bool)> = Vec::new();

    for ev in ctx.events.iter::<SkillCheckEvent>() {
        to_process.push((ev.entity, ev.skill_id, ev.difficulty, ev.success));
    }

    for (entity, skill_id, _difficulty, success) in to_process {
        if !world.is_alive(entity) {
            continue;
        }

        let Some(skill_set) = world.get_mut::<SkillSet>(entity) else {
            continue;
        };

        let Some(skill) = skill_set.get_mut(skill_id) else {
            continue;
        };

        if skill.lock != SkillLock::Up as u8 {
            continue;
        }

        let cap = DEFAULT_SKILL_CAP;
        if skill.base >= cap {
            continue;
        }

        let room = (cap - skill.base) / cap;
        let success_bonus = if success { 0.5 } else { 0.0 };
        let mut gc = room * skill.gain_factor * (0.5 + success_bonus);
        gc = gc.clamp(0.0, 1.0);

        let roll = ctx.rng.f32() as f64;
        if roll < gc {
            let old_value = skill.base;
            skill.base = (skill.base + 0.1).min(cap);
            ctx.emit(SkillGainEvent {
                entity,
                skill_id,
                old_value,
                new_value: skill.base,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use mge_core::{Engine, EngineConfig, PhaseId, Plugin};

    use super::*;
    use crate::components::{SkillSet, SkillValue};
    use crate::events::SkillCheckEvent;

    struct ProgressionTestPlugin;

    impl Plugin for ProgressionTestPlugin {
        fn name(&self) -> &str {
            "progression_test"
        }
        fn build(&self, engine: &mut Engine) {
            engine.register_component::<SkillSet>();
            let entity = engine.world_mut().spawn();
            let mut skill_set = SkillSet::new();
            skill_set.skills.push(SkillValue::new(1, 50.0, 1.0));
            engine.world_mut().insert(entity, skill_set);

            engine.add_system(PhaseId(0), |_world, ctx| {
                if ctx.tick_count() == 1 {
                    ctx.emit(SkillCheckEvent {
                        entity: mge_ecs::EntityId::new(0, 0),
                        skill_id: 1,
                        difficulty: 50.0,
                        success: true,
                    });
                }
            });
            engine.add_system(PhaseId(1), skill_gain_system);
        }
    }

    #[test]
    fn test_skill_gain_system_processes_events() {
        let config = EngineConfig {
            seed: 42,
            headless: true,
            fixed_timestep_ms: Some(16),
            tick_budget_ms: None,
        };
        let mut engine = Engine::new(config);
        engine.add_plugin(ProgressionTestPlugin);
        engine.build();

        engine.tick(0.016);
        engine.tick(0.016);

        let entity = mge_ecs::EntityId::new(0, 0);
        let skill = engine
            .world()
            .get::<SkillSet>(entity)
            .and_then(|s| s.get(1));
        assert!(skill.is_some());
        let base = skill.unwrap().base;
        assert!(
            (base - 50.0).abs() < 0.01 || (base - 50.1).abs() < 0.01,
            "base should be 50.0 or 50.1: {}",
            base
        );
    }
}
