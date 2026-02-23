//! # MGE Core — mge.core
//!
//! Microkernel Miyukini Game Engine : Engine, World, Scheduler, EventQueue, RNG, Time.
//!
//! @id mge.core
//! @role simulation
//! @layer core
//! @human Microkernel MGE — coeur minimal de simulation
//! @do provide_minimal_simulation_kernel

#![allow(missing_docs)]

pub mod config;
pub mod context;
pub mod engine;
pub mod plugin;
pub mod scheduler;

pub use mge_ecs::{Component, EntityId, Query2Mut, World};
pub use mge_event::{Event, EventQueue};
pub use mge_profiler::{PhaseId, PhaseMetrics, SystemMetrics, TickMetrics};
pub use mge_rng::Rng;
pub use mge_time::Time;

pub use config::EngineConfig;
pub use context::Context;
pub use engine::Engine;
pub use plugin::Plugin;
pub use scheduler::Scheduler;

#[cfg(test)]
mod tests {
    use rand::RngCore;

    use super::*;

    #[derive(Debug, Clone)]
    struct Counter(i32);
    impl Component for Counter {}

    #[derive(Debug, Clone)]
    struct ReadTicks(Vec<u64>);
    impl Component for ReadTicks {}

    #[derive(Debug, Clone)]
    struct RngValues(Vec<u32>);
    impl Component for RngValues {}

    #[derive(Debug, Clone)]
    struct PropagationEvent {
        tick: u64,
    }
    impl Event for PropagationEvent {}

    #[derive(Debug, Clone)]
    struct Health(i32);
    impl Component for Health {}

    #[derive(Debug, Clone)]
    struct Name(String);
    impl Component for Name {}

    #[derive(Debug, Clone)]
    struct Tag;
    impl Component for Tag {}

    #[test]
    fn test_world_spawn_despawn() {
        let mut world = World::new();
        world.register_component::<Health>();

        let a = world.spawn();
        let b = world.spawn();
        world.insert(a, Health(100));
        world.insert(b, Health(50));

        assert_eq!(world.iter1::<Health>().count(), 2);
        world.despawn(a);
        assert_eq!(world.iter1::<Health>().count(), 1);
        assert!(world.get::<Health>(a).is_none());
        assert!(world.get::<Health>(b).is_some());
    }

    #[test]
    fn test_world_iter2_iter3() {
        let mut world = World::new();
        world.register_component::<Health>();
        world.register_component::<Name>();
        world.register_component::<Tag>();

        let e1 = world.spawn();
        let e2 = world.spawn();
        world.insert(e1, Health(10));
        world.insert(e1, Name("Alice".into()));
        world.insert(e1, Tag);
        world.insert(e2, Health(20));
        world.insert(e2, Name("Bob".into()));

        let pairs: Vec<_> = world.iter2::<Health, Name>().collect();
        assert_eq!(pairs.len(), 2);

        let triples: Vec<_> = world.iter3::<Health, Name, Tag>().collect();
        assert_eq!(triples.len(), 1);
        assert_eq!(triples[0].2 .0, "Alice");
    }

    #[test]
    fn test_world_for_each_mut() {
        let mut world = World::new();
        world.register_component::<Health>();
        world.register_component::<Name>();

        let e = world.spawn();
        world.insert(e, Health(10));
        world.insert(e, Name("X".into()));

        world.for_each_mut::<Health, Name, _>(|_id, h, n| {
            h.0 += 5;
            n.0.push_str("Y");
        });

        let (_, h) = world.iter1::<Health>().next().unwrap();
        let (_, n) = world.iter1::<Name>().next().unwrap();
        assert_eq!(h.0, 15);
        assert_eq!(n.0, "XY");
    }

    #[test]
    fn test_time_pause_and_scale() {
        let mut time = Time::new();

        time.advance(1.0, None);
        assert!((time.delta_secs - 1.0).abs() < 1e-6);
        assert_eq!(time.tick_count, 1);

        time.time_scale = 2.0;
        time.advance(1.0, None);
        assert!((time.delta_secs - 2.0).abs() < 1e-6);
        assert_eq!(time.tick_count, 2);

        time.paused = true;
        time.advance(1.0, None);
        assert_eq!(time.delta_secs, 0.0);
        assert_eq!(time.tick_count, 3);
    }

    #[test]
    fn test_time_fixed_timestep() {
        let mut time = Time::new();
        time.advance(0.5, Some(16));
        assert!((time.delta_secs - 0.016).abs() < 1e-6);
    }

    #[test]
    fn test_engine_panic_plugin_after_build() {
        struct P;
        impl Plugin for P {
            fn name(&self) -> &str { "p" }
            fn build(&self, e: &mut Engine) { e.register_component::<Counter>(); }
        }
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut engine = Engine::new(EngineConfig::default());
            engine.add_plugin(P);
            engine.build();
            engine.add_plugin(P);
        }));
        assert!(result.is_err(), "add_plugin après build doit panic");
    }

    #[test]
    fn test_engine_panic_double_build() {
        struct P;
        impl Plugin for P {
            fn name(&self) -> &str { "p" }
            fn build(&self, _: &mut Engine) {}
        }
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut engine = Engine::new(EngineConfig::default());
            engine.add_plugin(P);
            engine.build();
            engine.build();
        }));
        assert!(result.is_err(), "build() deux fois doit panic");
    }

    #[test]
    fn test_entity_to_bits() {
        let id = EntityId::new(1, 2);
        assert_eq!(id.to_bits(), (1u64 << 32) | 2);
    }

    #[test]
    fn test_rng_derive_for_entity() {
        let rng1 = Rng::new(42);
        let rng2 = Rng::new(42);
        let id = EntityId::new(0, 0);
        let mut d1 = rng1.derive_for_bits(id.to_bits());
        let mut d2 = rng2.derive_for_bits(id.to_bits());
        assert_eq!(d1.next_u32(), d2.next_u32());
    }

    #[test]
    fn test_tick_simple() {
        let config = EngineConfig {
            seed: 42,
            headless: true,
            fixed_timestep_ms: Some(16),
            tick_budget_ms: None,
        };
        let mut engine = Engine::new(config);
        struct SimplePlugin;
        impl Plugin for SimplePlugin {
            fn name(&self) -> &str {
                "simple"
            }
            fn build(&self, engine: &mut Engine) {
                engine.register_component::<Counter>();
                let id = engine.world_mut().spawn();
                engine.world_mut().insert(id, Counter(0));
                engine.add_system(PhaseId(0), |world, _ctx| {
                    world.for_each1_mut::<Counter, _>(|_, c| c.0 += 1);
                });
            }
        }
        engine.add_plugin(SimplePlugin);
        engine.build();
        for _ in 0..10 {
            engine.tick(0.016);
        }
        assert_eq!(engine.time().tick_count, 10);
        let (_, c) = engine.world().iter1::<Counter>().next().unwrap();
        assert_eq!(c.0, 10);
    }

    #[test]
    fn test_event_propagation_n_to_n_plus_one() {
        struct EmitPlugin;
        impl Plugin for EmitPlugin {
            fn name(&self) -> &str {
                "emit"
            }
            fn build(&self, engine: &mut Engine) {
                engine.register_component::<ReadTicks>();
                let id = engine.world_mut().spawn();
                engine.world_mut().insert(id, ReadTicks(Vec::new()));
                engine.add_system(PhaseId(0), |_world, ctx| {
                    ctx.emit(PropagationEvent {
                        tick: ctx.tick_count(),
                    });
                });
                engine.add_system(PhaseId(1), |world, ctx| {
                    let mut seen = Vec::new();
                    for e in ctx.events.iter::<PropagationEvent>() {
                        seen.push(e.tick);
                    }
                    if !seen.is_empty() {
                        world.for_each1_mut::<ReadTicks, _>(|_, rt| rt.0.extend(seen.clone()));
                    }
                });
            }
        }
        let config = EngineConfig::default();
        let mut engine = Engine::new(config);
        engine.add_plugin(EmitPlugin);
        engine.build();
        engine.tick(0.016);
        engine.tick(0.016);
        let ticks: Vec<u64> = engine
            .world()
            .iter1::<ReadTicks>()
            .next()
            .map(|(_, rt)| rt.0.clone())
            .unwrap_or_default();
        assert_eq!(
            ticks, [1],
            "events emitted during first tick (tick_count=1) read at second tick"
        );
    }

    #[test]
    fn test_seed_reproductible() {
        struct RngPlugin;
        impl Plugin for RngPlugin {
            fn name(&self) -> &str {
                "rng"
            }
            fn build(&self, engine: &mut Engine) {
                engine.register_component::<RngValues>();
                let id = engine.world_mut().spawn();
                engine.world_mut().insert(id, RngValues(Vec::new()));
                engine.add_system(PhaseId(0), |world, ctx| {
                    let v = ctx.rng.u32();
                    world.for_each1_mut::<RngValues, _>(|_, rv| rv.0.push(v));
                });
            }
        }
        let run = |seed: u64| {
            let config = EngineConfig {
                seed,
                headless: true,
                ..Default::default()
            };
            let mut engine = Engine::new(config);
            engine.add_plugin(RngPlugin);
            engine.build();
            for _ in 0..5 {
                engine.tick(0.016);
            }
            let seq: Vec<u32> = engine
                .world()
                .iter1::<RngValues>()
                .next()
                .map(|(_, rv)| rv.0.clone())
                .unwrap_or_default();
            seq
        };
        let seq1 = run(42);
        let seq2 = run(42);
        assert_eq!(seq1, seq2, "même seed → même séquence RNG");
    }

    #[test]
    fn test_free_list_recycle() {
        let mut world = World::new();
        world.register_component::<Health>();

        let a = world.spawn();
        let _b = world.spawn();
        assert_eq!(world.entity_count(), 2);

        world.despawn(a);
        assert_eq!(world.entity_count(), 1);
        assert!(!world.is_alive(a));

        let c = world.spawn();
        assert_eq!(world.entity_count(), 2);
        assert_eq!(c.index(), a.index());
        assert_ne!(c.generation(), a.generation());
        assert!(c.generation() > a.generation());
    }

    #[test]
    fn test_stale_entity_id_rejected() {
        let mut world = World::new();
        world.register_component::<Health>();

        let id = world.spawn();
        world.insert(id, Health(100));
        world.despawn(id);

        assert!(!world.is_alive(id));
        assert!(world.get::<Health>(id).is_none());

        world.insert(id, Health(999));
        let recycled = world.spawn();
        assert!(world.get::<Health>(recycled).is_none());
    }

    #[test]
    fn test_has_component() {
        let mut world = World::new();
        world.register_component::<Health>();
        world.register_component::<Name>();

        let e = world.spawn();
        world.insert(e, Health(10));
        assert!(world.has_component::<Health>(e));
        assert!(!world.has_component::<Name>(e));
    }

    #[test]
    fn test_entity_count() {
        let mut world = World::new();
        assert_eq!(world.entity_count(), 0);
        let a = world.spawn();
        let _b = world.spawn();
        assert_eq!(world.entity_count(), 2);
        world.despawn(a);
        assert_eq!(world.entity_count(), 1);
    }

    #[test]
    fn test_rng_f32_range() {
        let mut rng = Rng::new(42);
        for _ in 0..100 {
            let v = rng.f32();
            assert!((0.0..1.0).contains(&v));
        }
        for _ in 0..100 {
            let v = rng.f32_range(5.0, 10.0);
            assert!((5.0..10.0).contains(&v));
        }
    }

    #[test]
    fn test_event_queue_has_count() {
        let mut queue = EventQueue::new();
        queue.emit(PropagationEvent { tick: 1 });
        queue.emit(PropagationEvent { tick: 2 });
        queue.swap();
        assert!(queue.has::<PropagationEvent>());
        assert_eq!(queue.count::<PropagationEvent>(), 2);
        #[derive(Debug, Clone)]
        struct OtherEvent;
        impl Event for OtherEvent {}
        assert!(!queue.has::<OtherEvent>());
    }

    #[test]
    fn test_named_system_profiling() {
        let config = EngineConfig {
            seed: 0,
            headless: true,
            tick_budget_ms: Some(1000),
            ..Default::default()
        };
        let mut engine = Engine::new(config);
        struct ProfPlugin;
        impl Plugin for ProfPlugin {
            fn name(&self) -> &str { "prof" }
            fn build(&self, engine: &mut Engine) {
                engine.register_component::<Counter>();
                let id = engine.world_mut().spawn();
                engine.world_mut().insert(id, Counter(0));
                engine.add_named_system(PhaseId(0), "increment", |world, _ctx| {
                    world.for_each1_mut::<Counter, _>(|_, c| c.0 += 1);
                });
            }
        }
        engine.add_plugin(ProfPlugin);
        engine.build();
        engine.tick(0.016);

        let metrics = engine.last_tick_metrics().expect("metrics after tick");
        assert!(!metrics.budget_exceeded);
        assert_eq!(metrics.phases.len(), 1);
        assert_eq!(metrics.phases[0].phase, PhaseId(0));
        assert_eq!(metrics.phases[0].systems.len(), 1);
        assert_eq!(
            metrics.phases[0].systems[0].name.as_deref(),
            Some("increment")
        );
    }

    #[test]
    fn test_budget_exceeded() {
        let config = EngineConfig {
            seed: 0,
            headless: true,
            tick_budget_ms: Some(0),
            ..Default::default()
        };
        let mut engine = Engine::new(config);
        struct SlowPlugin;
        impl Plugin for SlowPlugin {
            fn name(&self) -> &str { "slow" }
            fn build(&self, engine: &mut Engine) {
                engine.add_system(PhaseId(0), |_w, _c| {
                    std::thread::sleep(std::time::Duration::from_millis(1));
                });
            }
        }
        engine.add_plugin(SlowPlugin);
        engine.build();
        engine.tick(0.016);
        assert!(engine.last_tick_metrics().unwrap().budget_exceeded);
    }

    #[test]
    fn test_scheduler_system_count() {
        let config = EngineConfig::default();
        let mut engine = Engine::new(config);
        struct CountPlugin;
        impl Plugin for CountPlugin {
            fn name(&self) -> &str { "count" }
            fn build(&self, engine: &mut Engine) {
                engine.add_system(PhaseId(0), |_, _| {});
                engine.add_system(PhaseId(1), |_, _| {});
                engine.add_named_system(PhaseId(0), "named", |_, _| {});
            }
        }
        engine.add_plugin(CountPlugin);
        engine.build();
        engine.tick(0.016);

        let metrics = engine.last_tick_metrics().unwrap();
        let total_systems: usize = metrics.phases.iter()
            .map(|p| p.systems.len())
            .sum();
        assert_eq!(total_systems, 3);
    }

    #[test]
    fn test_remove_component() {
        let mut world = World::new();
        let e = world.spawn();
        world.insert(e, Health(100));
        world.insert(e, Counter(5));
        assert!(world.has_component::<Health>(e));
        assert!(world.has_component::<Counter>(e));

        world.remove_component::<Health>(e);
        assert!(!world.has_component::<Health>(e));
        assert!(world.has_component::<Counter>(e));
        assert!(world.is_alive(e));
        assert!(world.get::<Health>(e).is_none());
        assert_eq!(world.get::<Counter>(e).unwrap().0, 5);

        world.remove_component::<Counter>(e);
        assert!(!world.has_component::<Counter>(e));
        assert!(world.is_alive(e));

        // remove on dead entity is no-op
        world.despawn(e);
        world.remove_component::<Health>(e);
    }
}
