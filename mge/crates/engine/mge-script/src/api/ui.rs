// @id: MGE-Script-API-UI @do: ui-api @role: back-end @layer: 2 @human: miyuk
//! UI-related Rhai API stubs.
//!
//! These functions are registered on the engine in
//! [`crate::engine::ScriptEngine::new`]. This module serves as documentation
//! and as a standalone registration helper.

use rhai::Engine;

use crate::ScriptContext;

/// Register `ui_*` functions into an existing Rhai engine.
///
/// # Registered functions
///
/// | Rhai signature | Description |
/// |---|---|
/// | `ui_message(ctx, msg)` | Queue a plain message |
/// | `ui_dialog(ctx, text)` | Queue a dialog message (prefixed `[DIALOG]`) |
pub fn register(engine: &mut Engine) {
    engine.register_fn(
        "ui_message",
        |ctx: &mut ScriptContext, msg: &str| {
            ctx.pending_messages.push(msg.to_string());
        },
    );
    engine.register_fn(
        "ui_dialog",
        |ctx: &mut ScriptContext, text: &str| {
            ctx.pending_messages
                .push(format!("[DIALOG] {text}"));
        },
    );
}
