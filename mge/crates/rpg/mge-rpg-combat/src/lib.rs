//! @id mge.rpg.combat.v1
//! @role plugin
//! @layer plugin
//! @domain rpg
//! @do scaffold_combat_module

pub mod components;
pub mod events;
pub mod systems;

pub use components::*;
pub use events::*;
pub use systems::{combat_resolution_system, COMBAT_PHASE};

#[cfg(test)]
mod tests {
    use mge_core::{Engine, EngineConfig, Plugin};
    use mge_rpg_stats::{DeadTag, Health};

    use super::*;

    #[test]
    fn test_combat_resolution_hit_and_damage() {
        struct CombatTestPlugin;
        impl Plugin for CombatTestPlugin {
            fn name(&self) -> &str {
                "combat_test"
            }
            fn build(&self, engine: &mut mge_core::Engine) {
                engine.register_component::<CombatStats>();
                engine.register_component::<Armor>();
                engine.register_component::<AttackCooldown>();
                engine.register_component::<Health>();
                engine.register_component::<DeadTag>();

                let attacker = engine.world_mut().spawn();
                let defender = engine.world_mut().spawn();

                engine.world_mut().insert(
                    attacker,
                    CombatStats {
                        atk: 80.0,
                        esq: 10.0,
                        par: 5.0,
                        atk_speed: 40.0,
                        damage_base: 20.0,
                        damage_type: DAMAGE_TYPE_SLASH,
                    },
                );
                engine.world_mut().insert(
                    defender,
                    CombatStats {
                        atk: 5.0,
                        esq: 5.0,
                        par: 5.0,
                        atk_speed: 20.0,
                        damage_base: 5.0,
                        damage_type: DAMAGE_TYPE_SLASH,
                    },
                );
                engine
                    .world_mut()
                    .insert(defender, Health::at_full(50.0));
                engine.world_mut().insert(
                    defender,
                    Armor {
                        ar: [10.0, 0.0, 0.0],
                        resistance: 100.0,
                    },
                );

                engine.add_system(COMBAT_PHASE, combat_resolution_system);
            }
        }

        let config = EngineConfig {
            seed: 42,
            headless: true,
            ..Default::default()
        };
        let mut engine = Engine::new(config);
        engine.add_plugin(CombatTestPlugin);
        engine.build();

        let defender = engine
            .world()
            .iter2::<CombatStats, Health>()
            .next()
            .map(|(id, _, _)| id)
            .unwrap();
        let attacker = engine
            .world()
            .iter1::<CombatStats>()
            .find(|(id, _)| *id != defender)
            .map(|(id, _)| id)
            .unwrap();

        engine.emit(AttackRequestEvent {
            attacker,
            target: defender,
        });
        engine.tick(0.016);

        let health = engine.world().get::<Health>(defender).unwrap();
        assert!(
            health.current < 50.0,
            "defender should have taken damage (hp={})",
            health.current
        );
    }
}
