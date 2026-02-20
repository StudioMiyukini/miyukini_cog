Crate: mge-time
Version: 0.1.0
@id: mge.kernel.time
Domain: simulation

Exports:
- Time (delta_secs, tick_count, time_scale, paused)
- advance(delta_requested_secs, fixed_timestep_ms)

Dependencies: none

Hot path: yes
Headless safe: yes
AI-Native Score: 10/10
