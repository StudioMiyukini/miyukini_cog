// @id: MGE-UI-HudTests @do: hud-unit-tests @role: front-end @layer: 3 @human: miyuk
//! Unit tests for HUD logic — no GPU rendering required.
//!
//! Covers [`OrbState`], [`BeltState`], [`SkillSlot`], [`XpBarState`],
//! [`CombatLogState`], and [`MinimapState`].  All tests run in pure Rust
//! (no egui Context, no wgpu device).

#[cfg(test)]
mod hud_tests {
    use crate::combat_log::CombatLogState;
    use crate::hud::{BeltState, OrbState, PotionType, XpBarState};
    use crate::minimap::{MarkerKind, MinimapMarker, MinimapState};
    use crate::skill_bar::SkillSlot;

    // -----------------------------------------------------------------------
    // Orb tests (2)
    // -----------------------------------------------------------------------

    /// T1 — negative `current` must clamp `percent()` to `0.0`, never negative.
    #[test]
    fn orb_negative_current() {
        let orb = OrbState::new(-10.0, 100.0);
        assert!(
            orb.percent().abs() < f32::EPSILON,
            "negative current must clamp percent to 0.0, got {}",
            orb.percent()
        );
    }

    /// T2 — a single `update` call must move `display_percent` toward the target.
    #[test]
    fn orb_update_increases() {
        // Start fully empty (display_percent = 0.0).
        let mut orb = OrbState::new(0.0, 100.0);

        // Push toward 100 % with a large lerp step.
        orb.update(100.0, 100.0, 0.5);

        assert!(
            orb.display_percent > 0.0,
            "display_percent must increase toward target after update, got {}",
            orb.display_percent
        );
        assert!(
            orb.display_percent <= 1.0,
            "display_percent must not exceed 1.0, got {}",
            orb.display_percent
        );
    }

    // -----------------------------------------------------------------------
    // Belt tests (2)
    // -----------------------------------------------------------------------

    /// T3 — fill all 4 slots then consume each; every slot returns the correct
    /// `PotionType` and is empty afterwards.
    #[test]
    fn belt_all_slots_usable() {
        let potions = [
            PotionType::HpSmall,
            PotionType::HpMedium,
            PotionType::ManaSmall,
            PotionType::ManaMedium,
        ];

        let mut belt = BeltState::new();
        for (i, &p) in potions.iter().enumerate() {
            belt.set_slot(i, Some(p));
        }

        for (i, &expected) in potions.iter().enumerate() {
            let taken = belt.use_slot(i);
            assert_eq!(
                taken,
                Some(expected),
                "slot {i} should return {expected:?}, got {taken:?}"
            );
            assert_eq!(
                belt.slots[i], None,
                "slot {i} must be empty after use_slot"
            );
        }
    }

    /// T4 — using an empty slot must return `None` without panicking.
    #[test]
    fn belt_use_empty_slot() {
        let mut belt = BeltState::new();
        let result = belt.use_slot(2);
        assert_eq!(result, None, "use_slot on empty slot must return None");
    }

    // -----------------------------------------------------------------------
    // Skill bar tests (2)
    // -----------------------------------------------------------------------

    /// T5 — assigning a `skill_id` makes `skill_id.is_some()` return `true`.
    #[test]
    fn skill_bar_assign_skill() {
        let mut slot = SkillSlot::empty();
        slot.skill_id = Some("lightning".to_string());
        assert!(
            slot.skill_id.is_some(),
            "skill_id must be Some after assignment"
        );
    }

    /// T6 — after setting cooldown > 0 and then ticking it to `<= 0`,
    /// `is_ready()` must return `true`.
    #[test]
    fn skill_bar_cooldown_recovery() {
        let mut slot = SkillSlot {
            skill_id: Some("fireball".to_string()),
            cooldown_remaining: 2.0,
            cooldown_total: 2.0,
        };

        assert!(!slot.is_ready(), "slot must not be ready while cooldown > 0");

        // Simulate the cooldown fully expiring.
        slot.cooldown_remaining = 0.0;

        assert!(
            slot.is_ready(),
            "slot must be ready after cooldown_remaining reaches 0"
        );
    }

    // -----------------------------------------------------------------------
    // XP bar tests (2)
    // -----------------------------------------------------------------------

    /// T7 — at level start `current_xp = 0` must yield `progress() == 0.0`.
    #[test]
    fn xp_bar_zero_xp() {
        let mut xp = XpBarState::new();
        xp.update(1, 0, 1_000);
        assert!(
            xp.progress().abs() < f32::EPSILON,
            "zero current_xp must yield progress 0.0, got {}",
            xp.progress()
        );
    }

    /// T8 — `current_xp == next_level_xp` must yield `progress() == 1.0`.
    #[test]
    fn xp_bar_exactly_full() {
        let mut xp = XpBarState::new();
        xp.update(10, 5_000, 5_000);
        assert!(
            (xp.progress() - 1.0).abs() < f32::EPSILON,
            "current_xp == next_level_xp must yield progress 1.0, got {}",
            xp.progress()
        );
    }

    // -----------------------------------------------------------------------
    // Combat log tests (2)
    // -----------------------------------------------------------------------

    /// T9 — a freshly created `CombatLogState` must have zero entries.
    #[test]
    fn combat_log_empty() {
        let log = CombatLogState::new();
        assert_eq!(
            log.entries().len(),
            0,
            "new CombatLogState must have 0 entries"
        );
    }

    /// T10 — entries must be accessible in FIFO (insertion) order.
    #[test]
    fn combat_log_order() {
        let mut log = CombatLogState::new();
        log.push("first", [1.0, 1.0, 1.0, 1.0]);
        log.push("second", [1.0, 0.0, 0.0, 1.0]);
        log.push("third", [0.0, 1.0, 0.0, 1.0]);

        let messages: Vec<&str> = log.entries().iter().map(|e| e.message.as_str()).collect();
        assert_eq!(
            messages,
            ["first", "second", "third"],
            "entries must appear in insertion order"
        );
    }

    // -----------------------------------------------------------------------
    // Minimap tests (2)
    // -----------------------------------------------------------------------

    /// T11 — marking several tiles individually must make each one explored.
    #[test]
    fn minimap_mark_multiple() {
        let mut state = MinimapState::new();
        let tiles = [(0, 0), (1, 0), (0, 1), (5, -3), (-10, 7)];

        for (tx, ty) in tiles {
            state.mark_explored(tx, ty);
        }

        for (tx, ty) in tiles {
            assert!(
                state.is_explored(tx, ty),
                "tile ({tx},{ty}) must be marked as explored"
            );
        }
    }

    /// T12 — clearing the `markers` vec must leave it empty.
    #[test]
    fn minimap_markers_cleared() {
        let mut state = MinimapState::new();
        state.markers.push(MinimapMarker {
            x: 10.0,
            y: 20.0,
            kind: MarkerKind::Monster,
        });
        state.markers.push(MinimapMarker {
            x: 0.0,
            y: 0.0,
            kind: MarkerKind::Player,
        });

        state.markers.clear();

        assert!(
            state.markers.is_empty(),
            "markers must be empty after clear()"
        );
    }

    // -----------------------------------------------------------------------
    // Div-by-zero guards (3)
    // -----------------------------------------------------------------------

    /// T13 — `max = 0` must not panic inside `update()` or `percent()`.
    #[test]
    fn orb_max_zero_no_panic() {
        let mut orb = OrbState::new(50.0, 0.0);
        // update must not panic even when max stays zero.
        orb.update(100.0, 0.0, 1.0);
        let p = orb.percent();
        assert!(
            p.abs() < f32::EPSILON,
            "percent must be 0.0 when max is zero, got {p}"
        );
    }

    /// T14 — `next_level_xp = 0` must not panic and must clamp progress to 1.0
    /// (max-level sentinel defined by the game spec).
    #[test]
    fn xp_bar_next_zero() {
        let mut xp = XpBarState::new();
        // This must not panic.
        xp.update(99, 0, 0);
        let p = xp.progress();
        assert!(
            (p - 1.0).abs() < f32::EPSILON,
            "next_level_xp=0 must yield progress 1.0 (max-level sentinel), got {p}"
        );
    }

    /// T15 — `set_slot` with an out-of-bounds index must clamp silently
    /// (no panic, index 99 is treated as index 3 per the contract).
    #[test]
    fn belt_out_of_bounds() {
        let mut belt = BeltState::new();
        // Must not panic.
        belt.set_slot(99, Some(PotionType::Rejuvenation));
        // Contract: clamped to index 3.
        assert_eq!(
            belt.slots[3],
            Some(PotionType::Rejuvenation),
            "out-of-bounds set_slot must clamp to slot 3"
        );
    }
}
