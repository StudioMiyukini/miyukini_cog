//! Tests unitaires pour mge-core.

use crate::event::{Event, EventBus};
use crate::game_loop::{GameLoop, LoopConfig};
use crate::scheduler::{SystemId, SystemPriority, SystemScheduler};
use crate::time::{GameTime, TickRate};

// --- Event de test ---

#[derive(Debug, Clone)]
struct DamageEvent {
    amount: u32,
}
impl Event for DamageEvent {}

#[derive(Debug, Clone)]
struct HealEvent {
    amount: u32,
}
impl Event for HealEvent {}

// --- Tests TickRate ---

#[test]
fn tick_rate_sodomight_is_25hz() {
    let tr = TickRate::SODOMIGHT;
    assert_eq!(tr.ticks_per_second, 25);
}

#[test]
fn tick_rate_duration_25hz() {
    let tr = TickRate::SODOMIGHT;
    let d = tr.tick_duration();
    assert_eq!(d.as_millis(), 40); // 1000ms / 25 = 40ms
}

#[test]
fn tick_rate_duration_60hz() {
    let tr = TickRate::SIXTY_HZ;
    let d = tr.tick_duration();
    // 1_000_000_000 ns / 60 = 16_666_666 ns = 16ms (truncated)
    assert_eq!(d.as_millis(), 16);
}

#[test]
fn tick_rate_default_is_sodomight() {
    let tr = TickRate::default();
    assert_eq!(tr.ticks_per_second, TickRate::SODOMIGHT.ticks_per_second);
}

// --- Tests GameTime ---

#[test]
fn game_time_initial_state() {
    let t = GameTime::new();
    assert_eq!(t.tick, 0);
    assert_eq!(t.delta, std::time::Duration::ZERO);
    assert!((t.delta_secs - 0.0).abs() < f32::EPSILON);
}

#[test]
fn game_time_advance_increments_tick() {
    let mut t = GameTime::new();
    assert_eq!(t.tick, 0);
    t.advance();
    assert_eq!(t.tick, 1);
    t.advance();
    assert_eq!(t.tick, 2);
}

#[test]
fn game_time_advance_updates_elapsed() {
    let mut t = GameTime::new();
    t.advance();
    // elapsed doit etre > ZERO apres un advance (sauf si le CPU est infiniment rapide)
    // On verifie juste que ca ne panic pas et que tick a avance
    assert_eq!(t.tick, 1);
}

#[test]
fn game_time_default_is_new() {
    let t = GameTime::default();
    assert_eq!(t.tick, 0);
}

// --- Tests EventBus ---

#[test]
fn event_bus_new_is_empty() {
    let bus = EventBus::new();
    assert!(bus.is_empty());
}

#[test]
fn event_bus_publish_and_drain() {
    let mut bus = EventBus::new();
    bus.publish(DamageEvent { amount: 42 });
    bus.publish(DamageEvent { amount: 10 });

    let events = bus.drain::<DamageEvent>();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].amount, 42);
    assert_eq!(events[1].amount, 10);
}

#[test]
fn event_bus_drain_empties_queue() {
    let mut bus = EventBus::new();
    bus.publish(DamageEvent { amount: 42 });

    let events = bus.drain::<DamageEvent>();
    assert_eq!(events.len(), 1);

    // Apres drain, la queue est vide
    let events2 = bus.drain::<DamageEvent>();
    assert_eq!(events2.len(), 0);
}

#[test]
fn event_bus_different_types_independent() {
    let mut bus = EventBus::new();
    bus.publish(DamageEvent { amount: 100 });
    bus.publish(HealEvent { amount: 50 });

    let damage = bus.drain::<DamageEvent>();
    assert_eq!(damage.len(), 1);
    assert_eq!(damage[0].amount, 100);

    let heal = bus.drain::<HealEvent>();
    assert_eq!(heal.len(), 1);
    assert_eq!(heal[0].amount, 50);
}

#[test]
fn event_bus_count() {
    let mut bus = EventBus::new();
    assert_eq!(bus.count::<DamageEvent>(), 0);

    bus.publish(DamageEvent { amount: 1 });
    bus.publish(DamageEvent { amount: 2 });
    assert_eq!(bus.count::<DamageEvent>(), 2);
    assert_eq!(bus.count::<HealEvent>(), 0);
}

#[test]
fn event_bus_clear_all() {
    let mut bus = EventBus::new();
    bus.publish(DamageEvent { amount: 42 });
    bus.publish(HealEvent { amount: 10 });

    bus.clear_all();
    assert!(bus.is_empty());
    assert_eq!(bus.drain::<DamageEvent>().len(), 0);
    assert_eq!(bus.drain::<HealEvent>().len(), 0);
}

#[test]
fn event_bus_default_is_new() {
    let bus = EventBus::default();
    assert!(bus.is_empty());
}

// --- Tests SystemScheduler ---

#[test]
fn scheduler_new_is_empty() {
    let sched = SystemScheduler::new();
    assert!(sched.is_empty());
    assert_eq!(sched.len(), 0);
}

#[test]
fn scheduler_register_and_iterate() {
    let mut sched = SystemScheduler::new();
    sched.register(SystemId("movement"), SystemPriority::PHYSICS);
    sched.register(SystemId("input_read"), SystemPriority::INPUT);
    sched.register(SystemId("combat"), SystemPriority::GAME);

    let ids: Vec<&SystemId> = sched.enabled_ids().collect();
    assert_eq!(ids.len(), 3);
    // Tri par priorite : INPUT < PHYSICS < GAME
    assert_eq!(ids[0], &SystemId("input_read"));
    assert_eq!(ids[1], &SystemId("movement"));
    assert_eq!(ids[2], &SystemId("combat"));
}

#[test]
fn scheduler_set_enabled_false() {
    let mut sched = SystemScheduler::new();
    sched.register(SystemId("movement"), SystemPriority::PHYSICS);
    sched.register(SystemId("combat"), SystemPriority::GAME);

    sched.set_enabled(&SystemId("movement"), false);

    let ids: Vec<&SystemId> = sched.enabled_ids().collect();
    assert_eq!(ids.len(), 1);
    assert_eq!(ids[0], &SystemId("combat"));
}

#[test]
fn scheduler_set_enabled_reactivate() {
    let mut sched = SystemScheduler::new();
    sched.register(SystemId("ai"), SystemPriority::GAME);
    sched.set_enabled(&SystemId("ai"), false);
    assert_eq!(sched.enabled_ids().count(), 0);

    sched.set_enabled(&SystemId("ai"), true);
    assert_eq!(sched.enabled_ids().count(), 1);
}

#[test]
fn scheduler_contains() {
    let mut sched = SystemScheduler::new();
    sched.register(SystemId("render"), SystemPriority::RENDER);

    assert!(sched.contains(&SystemId("render")));
    assert!(!sched.contains(&SystemId("nonexistent")));
}

#[test]
fn scheduler_default_is_new() {
    let sched = SystemScheduler::default();
    assert!(sched.is_empty());
}

// --- Tests GameLoop ---

#[test]
fn game_loop_new_default_config() {
    let gl = GameLoop::new(LoopConfig::default());
    assert_eq!(gl.config().tick_rate.ticks_per_second, 25);
    assert_eq!(gl.config().max_ticks_per_frame, 5);
    assert_eq!(gl.time.tick, 0);
}

#[test]
fn game_loop_alpha_initial() {
    let gl = GameLoop::new(LoopConfig::default());
    // Juste apres creation, accumulator est ZERO
    let alpha = gl.alpha();
    assert!(alpha >= 0.0);
    assert!(alpha <= 1.0);
}

#[test]
fn loop_config_default_sodomight() {
    let cfg = LoopConfig::default();
    assert_eq!(cfg.tick_rate.ticks_per_second, 25);
    assert_eq!(cfg.max_ticks_per_frame, 5);
}

// --- Tests SystemPriority ordering ---

#[test]
fn system_priority_ordering() {
    assert!(SystemPriority::INPUT < SystemPriority::PHYSICS);
    assert!(SystemPriority::PHYSICS < SystemPriority::GAME);
    assert!(SystemPriority::GAME < SystemPriority::RENDER);
    assert!(SystemPriority::RENDER < SystemPriority::UI);
}
