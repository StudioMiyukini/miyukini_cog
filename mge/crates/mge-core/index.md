Crate: mge-core
Version: 0.1.0
@id: mge.core
Domain: core

Exports:
- Engine (orchestration, tick, plugins)
- EngineConfig (seed, headless, fixed_timestep_ms, tick_budget_ms)
- Plugin (trait), Scheduler, PhaseId
- Context (time, rng, events, emit)
- Re-exports: Component, EntityId, World, Query2Mut, Event, EventQueue, Time, Rng, TickMetrics

Dependencies: mge-ecs, mge-event, mge-profiler, mge-rng, mge-time

Hot path: yes
Headless safe: yes
AI-Native Score: 9/10
