// @id: MGE-Script-Tests @do: unit-tests @role: back-end @layer: 2 @human: miyuk
//! Unit tests for the scripting subsystem.

#[cfg(test)]
mod tests {
    use crate::{ScriptContext, ScriptEngine};

    fn make_engine() -> ScriptEngine {
        ScriptEngine::new()
    }

    fn make_ctx() -> ScriptContext {
        ScriptContext::new("char_001", "rogue_encampment")
    }

    #[test]
    fn test_quest_set_and_get() {
        let mut engine = make_engine();
        engine
            .compile(
                "test_quest",
                r#"
            quest_set(ctx, "den_of_evil", "active");
            let state = quest_get(ctx, "den_of_evil");
            if state != "active" {
                log_warn("Quest state mismatch!");
            }
        "#,
            )
            .unwrap();

        let mut ctx = make_ctx();
        engine.run("test_quest", &mut ctx).unwrap();
        assert_eq!(
            ctx.quest_flags.get("den_of_evil").map(|s| s.as_str()),
            Some("active")
        );
    }

    #[test]
    fn test_reward_xp() {
        let mut engine = make_engine();
        engine
            .compile(
                "test_xp",
                r#"
            reward_xp(ctx, 1500);
            reward_xp(ctx, 500);
        "#,
            )
            .unwrap();

        let mut ctx = make_ctx();
        engine.run("test_xp", &mut ctx).unwrap();
        assert_eq!(ctx.pending_xp, 2000);
    }

    #[test]
    fn test_reward_item() {
        let mut engine = make_engine();
        engine
            .compile(
                "test_item",
                r#"
            reward_item(ctx, "identify_scroll", 5);
            reward_item(ctx, "town_portal", 3);
        "#,
            )
            .unwrap();

        let mut ctx = make_ctx();
        engine.run("test_item", &mut ctx).unwrap();
        assert_eq!(ctx.pending_item_grants.len(), 2);
        assert_eq!(
            ctx.pending_item_grants[0],
            ("identify_scroll".to_string(), 5)
        );
    }

    #[test]
    fn test_world_warp() {
        let mut engine = make_engine();
        engine
            .compile(
                "test_warp",
                r#"
            world_warp(ctx, "tristram");
        "#,
            )
            .unwrap();

        let mut ctx = make_ctx();
        engine.run("test_warp", &mut ctx).unwrap();
        assert_eq!(ctx.pending_warp.as_deref(), Some("tristram"));
    }

    #[test]
    fn test_ui_message() {
        let mut engine = make_engine();
        engine
            .compile(
                "test_ui",
                r#"
            ui_message(ctx, "Hello World");
            ui_dialog(ctx, "Bonjour aventurier");
        "#,
            )
            .unwrap();

        let mut ctx = make_ctx();
        engine.run("test_ui", &mut ctx).unwrap();
        assert_eq!(ctx.pending_messages.len(), 2);
        assert!(ctx.pending_messages[0].contains("Hello World"));
    }

    #[test]
    fn test_conditional_quest_logic() {
        let mut engine = make_engine();
        engine
            .compile(
                "test_conditional",
                r#"
            quest_set(ctx, "den_of_evil", "active");
            if quest_get(ctx, "den_of_evil") == "active" {
                reward_xp(ctx, 1000);
                quest_set(ctx, "den_of_evil", "complete");
            }
        "#,
            )
            .unwrap();

        let mut ctx = make_ctx();
        engine.run("test_conditional", &mut ctx).unwrap();
        assert_eq!(ctx.pending_xp, 1000);
        assert_eq!(
            ctx.quest_flags.get("den_of_evil").map(|s| s.as_str()),
            Some("complete")
        );
    }

    #[test]
    fn test_kill_count() {
        let mut engine = make_engine();
        engine
            .compile(
                "test_kills",
                r#"
            let kills = player_kill_count(ctx, "fallen");
            if kills >= 10 {
                quest_set(ctx, "den_of_evil", "complete");
            }
        "#,
            )
            .unwrap();

        let mut ctx = make_ctx();
        ctx.kill_counts.insert("fallen".to_string(), 15);
        engine.run("test_kills", &mut ctx).unwrap();
        assert_eq!(
            ctx.quest_flags.get("den_of_evil").map(|s| s.as_str()),
            Some("complete")
        );
    }

    #[test]
    fn test_max_operations_limit() {
        let mut engine = make_engine();
        engine
            .compile(
                "test_infinite_loop",
                r#"
            let i = 0;
            loop {
                i += 1;
            }
        "#,
            )
            .unwrap();

        let mut ctx = make_ctx();
        let result = engine.run("test_infinite_loop", &mut ctx);
        assert!(
            result.is_err(),
            "Infinite loop must be stopped by operation limit"
        );
    }

    // --- Additional coverage tests ---

    #[test]
    fn test_engine_default_trait() {
        let engine = ScriptEngine::default();
        // Must not panic; just verify the engine is usable.
        drop(engine);
    }

    #[test]
    fn test_run_not_found_script() {
        let engine = make_engine();
        let mut ctx = make_ctx();
        let result = engine.run("nonexistent_script", &mut ctx);
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("not found"));
    }

    #[test]
    fn test_compile_parse_error() {
        let mut engine = make_engine();
        let result = engine.compile("bad_script", "this is not valid rhai {{{{");
        assert!(result.is_err());
    }

    #[test]
    fn test_player_has_item() {
        let mut engine = make_engine();
        engine
            .compile(
                "test_has_item",
                r#"
            let has = player_has_item(ctx, "potion");
            if has {
                ui_message(ctx, "HAS_POTION");
            } else {
                ui_message(ctx, "NO_POTION");
            }
        "#,
            )
            .unwrap();

        // Without the item
        let mut ctx = make_ctx();
        engine.run("test_has_item", &mut ctx).unwrap();
        assert_eq!(ctx.pending_messages[0], "NO_POTION");

        // With the item
        let mut ctx2 = make_ctx();
        ctx2.inventory.insert("potion".to_string(), 3);
        engine.run("test_has_item", &mut ctx2).unwrap();
        assert_eq!(ctx2.pending_messages[0], "HAS_POTION");
    }

    #[test]
    fn test_quest_is_complete() {
        let mut engine = make_engine();
        engine
            .compile(
                "test_qic",
                r#"
            quest_set(ctx, "test_q", "complete");
            let done = quest_is_complete(ctx, "test_q");
            if done {
                ui_message(ctx, "DONE");
            }
        "#,
            )
            .unwrap();

        let mut ctx = make_ctx();
        engine.run("test_qic", &mut ctx).unwrap();
        assert_eq!(ctx.pending_messages[0], "DONE");
    }

    #[test]
    fn test_reward_skill_point() {
        let mut engine = make_engine();
        engine
            .compile(
                "test_sp",
                r#"
            reward_skill_point(ctx);
        "#,
            )
            .unwrap();

        let mut ctx = make_ctx();
        engine.run("test_sp", &mut ctx).unwrap();
        assert_eq!(ctx.pending_messages[0], "[REWARD_SKILL_POINT]");
    }

    #[test]
    fn rhai_max_map_overflow() {
        let mut engine = make_engine();
        // Build a map literal with 300 entries (limit is 256).
        // Rhai enforces max_map_size on map literal construction.
        let mut entries = Vec::with_capacity(300);
        for i in 0..300 {
            entries.push(format!("k{i}: {i}"));
        }
        let script = format!("let m = #{{ {} }};", entries.join(", "));
        let result = engine.compile("test_map_overflow", &script);
        // Rhai may reject at compile or runtime; either is acceptable.
        if let Ok(()) = result {
            let mut ctx = make_ctx();
            let run_result = engine.run("test_map_overflow", &mut ctx);
            assert!(
                run_result.is_err(),
                "Map with >256 entries must be rejected by the sandbox"
            );
        }
        // If compile itself errored, the limit was enforced — test passes.
    }

    #[test]
    fn rhai_max_expr_depth() {
        let mut engine = make_engine();
        // Build a deeply nested expression: (((((...1 + 1) + 1) + 1)...))
        // 40 levels of nesting exceeds the max_expr_depth of 32.
        let mut expr = String::from("1");
        for _ in 0..40 {
            expr = format!("({expr} + 1)");
        }
        let script = format!("let x = {expr};");
        let result = engine.compile("test_deep_expr", &script);
        assert!(
            result.is_err(),
            "Expression with >32 nesting depth must be rejected at compile time"
        );
    }

    #[test]
    fn test_world_unlock_waypoint() {
        let mut engine = make_engine();
        engine
            .compile(
                "test_wp",
                r#"
            world_unlock_waypoint(ctx, 1, "blood_moor");
            world_unlock_waypoint(ctx, 1, "cold_plains");
            world_unlock_waypoint(ctx, 2, "lut_gholein");
        "#,
            )
            .unwrap();

        let mut ctx = make_ctx();
        engine.run("test_wp", &mut ctx).unwrap();
        assert_eq!(ctx.waypoints.get(&1).unwrap().len(), 2);
        assert_eq!(ctx.waypoints.get(&2).unwrap().len(), 1);
    }

    // --- SEC-09: script hash verification ---

    #[test]
    fn rhai_script_hash_fnv1a_empty() {
        // FNV-1a of empty data must equal the FNV offset basis.
        let hash = crate::engine::fnv1a_64(b"");
        assert_eq!(format!("{hash:016x}"), "cbf29ce484222325");
    }

    #[test]
    fn rhai_script_hash_fnv1a_known_content() {
        // Any non-empty content must differ from the empty hash.
        let hash_empty = crate::engine::fnv1a_64(b"");
        let hash_hello = crate::engine::fnv1a_64(b"hello");
        assert_ne!(
            format!("{hash_hello:016x}"),
            format!("{hash_empty:016x}"),
            "FNV-1a of 'hello' must differ from empty hash"
        );
    }

    #[test]
    fn rhai_script_hash_deterministic() {
        // Same input must always produce the same hash.
        let h1 = crate::engine::fnv1a_64(b"sodomight_script");
        let h2 = crate::engine::fnv1a_64(b"sodomight_script");
        assert_eq!(h1, h2, "FNV-1a must be deterministic");
    }

    // --- SEC-18: reward caps ---

    #[test]
    fn rhai_reward_xp_capped_at_100k() {
        let mut engine = make_engine();
        engine
            .compile(
                "test_xp_cap",
                r#"
            reward_xp(ctx, 999999);
        "#,
            )
            .unwrap();

        let mut ctx = make_ctx();
        engine.run("test_xp_cap", &mut ctx).unwrap();
        assert_eq!(
            ctx.pending_xp, 100_000,
            "XP must be capped at 100_000 (SEC-18)"
        );
    }

    #[test]
    fn rhai_reward_item_qty_capped_at_100() {
        let mut engine = make_engine();
        engine
            .compile(
                "test_item_cap",
                r#"
            reward_item(ctx, "potion", 9999);
        "#,
            )
            .unwrap();

        let mut ctx = make_ctx();
        engine.run("test_item_cap", &mut ctx).unwrap();
        assert_eq!(ctx.pending_item_grants.len(), 1);
        assert_eq!(
            ctx.pending_item_grants[0],
            ("potion".to_string(), 100),
            "Item quantity must be capped at 100 (SEC-18)"
        );
    }

    #[test]
    fn rhai_reward_xp_below_cap_unchanged() {
        let mut engine = make_engine();
        engine
            .compile(
                "test_xp_under_cap",
                r#"
            reward_xp(ctx, 500);
        "#,
            )
            .unwrap();

        let mut ctx = make_ctx();
        engine.run("test_xp_under_cap", &mut ctx).unwrap();
        assert_eq!(ctx.pending_xp, 500, "XP under cap must be unchanged");
    }
}
