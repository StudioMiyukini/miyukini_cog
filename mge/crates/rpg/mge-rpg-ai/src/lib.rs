//! @id mge.rpg.ai.v1
//! @role plugin
//! @layer plugin
//! @domain rpg
//! @do decision PNJ FSM, ciblage, tactiques combat

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
pub use systems::ai_tick_system;

#[cfg(test)]
mod tests {
    use mge_core::{Engine, EngineConfig, PhaseId, Plugin};
    use mge_plugin_spatial::{Position2D, Velocity2D};

    use super::*;

    struct AiTestPlugin;
    impl Plugin for AiTestPlugin {
        fn name(&self) -> &str { "ai_test" }
        fn build(&self, e: &mut Engine) {
            e.register_component::<AIState>();
            e.register_component::<AiTargetable>();
            e.register_component::<Position2D>();
            e.register_component::<Velocity2D>();
            e.register_component::<mge_rpg_stats::Health>();
            let monster = e.world_mut().spawn();
            e.world_mut().insert(monster, Position2D { x: 10.0, y: 10.0 });
            e.world_mut().insert(monster, AIState::new((10.0, 10.0), 20.0, 50.0));
            let player = e.world_mut().spawn();
            e.world_mut().insert(player, Position2D { x: 15.0, y: 15.0 });
            e.world_mut().insert(player, AiTargetable);
            e.world_mut().insert(player, mge_rpg_stats::Health::at_full(100.0));
            e.add_system(PhaseId(500), ai_tick_system);
        }
    }

    #[test]
    fn test_ai_tick_runs() {
        let mut engine = Engine::new(EngineConfig { seed: 42, headless: true, ..Default::default() });
        engine.add_plugin(AiTestPlugin);
        engine.build();
        engine.tick(0.016);
        let ai = engine.world().iter1::<AIState>().next();
        assert!(ai.is_some(), "AIState should exist");
    }
}
