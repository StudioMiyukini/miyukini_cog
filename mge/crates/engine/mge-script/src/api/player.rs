// @id: MGE-Script-API-Player @do: player-api @role: back-end @layer: 2 @human: miyuk
//! Player-related Rhai API stubs.
//!
//! These functions are registered on the engine in
//! [`crate::engine::ScriptEngine::new`]. This module serves as documentation
//! and as a standalone registration helper.

use rhai::Engine;

use crate::ScriptContext;

/// Register `player_*` functions into an existing Rhai engine.
///
/// # Registered functions
///
/// | Rhai signature | Description |
/// |---|---|
/// | `player_has_item(ctx, item_id)` -> bool | Check inventory |
/// | `player_item_count(ctx, item_id)` -> int | Item quantity |
/// | `player_level(ctx)` -> int | Character level |
/// | `player_zone(ctx)` -> string | Current zone id |
/// | `player_kill_count(ctx, monster)` -> int | Kills of a type |
pub fn register(engine: &mut Engine) {
    engine.register_fn(
        "player_has_item",
        |ctx: &mut ScriptContext, item_id: &str| -> bool {
            ctx.inventory.get(item_id).copied().unwrap_or(0) > 0
        },
    );
    engine.register_fn(
        "player_item_count",
        |ctx: &mut ScriptContext, item_id: &str| -> i64 {
            i64::from(ctx.inventory.get(item_id).copied().unwrap_or(0))
        },
    );
    engine.register_fn("player_level", |ctx: &mut ScriptContext| -> i64 {
        i64::from(ctx.level)
    });
    engine.register_fn(
        "player_zone",
        |ctx: &mut ScriptContext| -> String { ctx.zone_id.clone() },
    );
    engine.register_fn(
        "player_kill_count",
        |ctx: &mut ScriptContext, monster: &str| -> i64 {
            i64::from(ctx.kill_counts.get(monster).copied().unwrap_or(0))
        },
    );
}
