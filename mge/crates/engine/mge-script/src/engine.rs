// @id: MGE-Script-Engine @do: rhai-engine @role: back-end @layer: 2 @human: miyuk
//! Configured Rhai engine with security limits and registered game API.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use rhai::{Dynamic, Engine, Scope, AST};

use crate::{ScriptContext, ScriptError, ScriptResult};

/// The MGE scripting engine.
///
/// Wraps a Rhai [`Engine`] with security limits and pre-registered API
/// functions for quest logic, rewards, world interactions and UI.
pub struct ScriptEngine {
    engine: Engine,
    /// Compiled script cache: `script_id -> AST`.
    compiled: HashMap<String, AST>,
}

impl ScriptEngine {
    /// Create a new scripting engine with all safety limits and API functions.
    pub fn new() -> Self {
        let mut engine = Engine::new();

        // --- Security limits ---
        engine.set_max_operations(50_000); // anti-infinite-loop
        engine.set_max_call_levels(32);
        engine.set_max_string_size(4096);
        engine.set_max_array_size(1024);

        // Disable dangerous features
        engine.disable_symbol("eval");

        // Register the context type
        engine.register_type_with_name::<ScriptContext>("ScriptContext");

        // Register all API functions
        register_player_api(&mut engine);
        register_quest_api(&mut engine);
        register_world_api(&mut engine);
        register_ui_api(&mut engine);
        register_reward_api(&mut engine);
        register_utility_api(&mut engine);

        Self {
            engine,
            compiled: HashMap::new(),
        }
    }

    /// Compile and cache a Rhai script.
    pub fn compile(&mut self, id: &str, source: &str) -> ScriptResult<()> {
        let ast = self
            .engine
            .compile(source)
            .map_err(|e| ScriptError::Parse(e.to_string()))?;
        self.compiled.insert(id.to_string(), ast);
        Ok(())
    }

    /// Execute a compiled script, mutating the provided context in place.
    pub fn run(&self, id: &str, ctx: &mut ScriptContext) -> ScriptResult<()> {
        let ast = self
            .compiled
            .get(id)
            .ok_or_else(|| ScriptError::NotFound(id.to_string()))?;

        let mut scope = Scope::new();
        scope.push("ctx", ctx.clone());

        self.engine
            .run_ast_with_scope(&mut scope, ast)
            .map_err(|e| ScriptError::Runtime(e.to_string()))?;

        // Retrieve the modified context from the scope.
        if let Some(new_ctx) = scope.get_value::<ScriptContext>("ctx") {
            *ctx = new_ctx;
        }

        Ok(())
    }

    /// Call a specific function inside a compiled script (e.g. `"on_complete"`).
    pub fn call_fn(
        &self,
        id: &str,
        fn_name: &str,
        ctx: &mut ScriptContext,
    ) -> ScriptResult<()> {
        let ast = self
            .compiled
            .get(id)
            .ok_or_else(|| ScriptError::NotFound(id.to_string()))?;

        let mut scope = Scope::new();
        scope.push("ctx", ctx.clone());

        let _: Dynamic = self
            .engine
            .call_fn(&mut scope, ast, fn_name, (ctx.clone(),))
            .map_err(|e| ScriptError::Runtime(e.to_string()))?;

        if let Some(new_ctx) = scope.get_value::<ScriptContext>("ctx") {
            *ctx = new_ctx;
        }

        Ok(())
    }

    /// Load all `.rhai` files from a directory, compile and cache them.
    ///
    /// Returns the number of scripts successfully loaded.
    pub fn load_directory(&mut self, dir: &Path) -> ScriptResult<usize> {
        let mut count = 0;
        if !dir.exists() {
            tracing::warn!("Script directory not found: {}", dir.display());
            return Ok(0);
        }

        for entry in
            fs::read_dir(dir).map_err(|e| ScriptError::NotFound(e.to_string()))?
        {
            let entry =
                entry.map_err(|e| ScriptError::NotFound(e.to_string()))?;
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "rhai") {
                let id = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                let source = fs::read_to_string(&path)
                    .map_err(|e| ScriptError::NotFound(e.to_string()))?;
                match self.compile(&id, &source) {
                    Ok(()) => {
                        tracing::info!("Loaded script: {}", id);
                        count += 1;
                    }
                    Err(e) => {
                        tracing::error!("Script compile error {}: {}", id, e);
                    }
                }
            }
        }
        Ok(count)
    }
}

impl Default for ScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Private registration helpers (split out to keep `new()` under 100 lines)
// ---------------------------------------------------------------------------

fn register_player_api(engine: &mut Engine) {
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
    engine.register_fn(
        "player_level",
        |ctx: &mut ScriptContext| -> i64 { i64::from(ctx.level) },
    );
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

fn register_quest_api(engine: &mut Engine) {
    engine.register_fn(
        "quest_get",
        |ctx: &mut ScriptContext, quest_id: &str| -> String {
            ctx.quest_flags
                .get(quest_id)
                .cloned()
                .unwrap_or_else(|| "none".to_string())
        },
    );
    engine.register_fn(
        "quest_set",
        |ctx: &mut ScriptContext, quest_id: &str, state: &str| {
            ctx.quest_flags
                .insert(quest_id.to_string(), state.to_string());
        },
    );
    engine.register_fn(
        "quest_is_complete",
        |ctx: &mut ScriptContext, quest_id: &str| -> bool {
            ctx.quest_flags
                .get(quest_id)
                .is_some_and(|s| s == "complete")
        },
    );
}

fn register_world_api(engine: &mut Engine) {
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

fn register_ui_api(engine: &mut Engine) {
    engine.register_fn(
        "ui_message",
        |ctx: &mut ScriptContext, msg: &str| {
            ctx.pending_messages.push(msg.to_string());
        },
    );
    engine.register_fn(
        "ui_dialog",
        |ctx: &mut ScriptContext, text: &str| {
            ctx.pending_messages.push(format!("[DIALOG] {text}"));
        },
    );
}

fn register_reward_api(engine: &mut Engine) {
    engine.register_fn(
        "reward_item",
        |ctx: &mut ScriptContext, item_id: &str, qty: i64| {
            ctx.pending_item_grants
                .push((item_id.to_string(), qty as u32));
        },
    );
    engine.register_fn(
        "reward_xp",
        |ctx: &mut ScriptContext, xp: i64| {
            ctx.pending_xp += xp;
        },
    );
    engine.register_fn(
        "reward_skill_point",
        |ctx: &mut ScriptContext| {
            ctx.pending_messages
                .push("[REWARD_SKILL_POINT]".to_string());
        },
    );
}

fn register_utility_api(engine: &mut Engine) {
    engine.register_fn("log_info", |msg: &str| {
        tracing::info!("[Rhai] {}", msg);
    });
    engine.register_fn("log_warn", |msg: &str| {
        tracing::warn!("[Rhai] {}", msg);
    });
}
