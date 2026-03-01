// @id: MGE-Script-API-World @do: world-api @role: back-end @layer: 2 @human: miyuk
//! World-related Rhai API stubs.
//!
//! These functions are registered on the engine in
//! [`crate::engine::ScriptEngine::new`]. This module serves as documentation
//! and as a standalone registration helper.

use rhai::Engine;

use crate::ScriptContext;

/// Register `world_*` functions into an existing Rhai engine.
///
/// # Registered functions
///
/// | Rhai signature | Description |
/// |---|---|
/// | `world_warp(ctx, zone_id)` | Queue a zone warp |
/// | `world_unlock_waypoint(ctx, act, wp_id)` | Unlock a waypoint |
pub fn register(engine: &mut Engine) {
    engine.register_fn(
        "world_warp",
        |ctx: &mut ScriptContext, zone_id: &str| {
            ctx.pending_warp = Some(zone_id.to_string());
        },
    );
    engine.register_fn(
        "world_unlock_waypoint",
        |ctx: &mut ScriptContext, act: i64, wp_id: &str| {
            ctx.waypoints
                .entry(act as i32)
                .or_default()
                .push(wp_id.to_string());
        },
    );
}
