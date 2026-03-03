// @id: MGE-ARPG-AI-TESTS @do: tests @role: back-end @layer: 3 @human: miyuk
//! Unit tests for the `mge-arpg-ai` crate.

#[cfg(test)]
mod tests {
    use crate::aggro::{AggroRange, AggroSystem};
    use crate::agent::AiAgent;
    use crate::patrol::{PatrolLoop, PatrolPath};
    use crate::state::{AiState, AiStateMachine, AiTransition};

    // ── FSM tests ──────────────────────────────────────────────────────

    #[test]
    fn fsm_initial_state() {
        let fsm = AiStateMachine::new(AiState::Idle);
        assert_eq!(fsm.current(), AiState::Idle);
    }

    #[test]
    fn fsm_idle_to_alert_on_spotted() {
        let mut fsm = AiStateMachine::new(AiState::Idle);
        let state = fsm.transition(AiTransition::TargetSpotted);
        assert_eq!(state, AiState::Alert);
    }

    #[test]
    fn fsm_chase_to_attack_in_range() {
        let mut fsm = AiStateMachine::new(AiState::Chase);
        let state = fsm.transition(AiTransition::TargetInRange);
        assert_eq!(state, AiState::Attack);
    }

    #[test]
    fn fsm_attack_to_chase_out_of_range() {
        let mut fsm = AiStateMachine::new(AiState::Attack);
        let state = fsm.transition(AiTransition::TargetOutOfRange);
        assert_eq!(state, AiState::Chase);
    }

    #[test]
    fn fsm_any_to_dead() {
        for initial in [
            AiState::Idle,
            AiState::Patrol,
            AiState::Alert,
            AiState::Chase,
            AiState::Attack,
            AiState::Flee,
        ] {
            let mut fsm = AiStateMachine::new(initial);
            let state = fsm.transition(AiTransition::Died);
            assert_eq!(state, AiState::Dead, "Failed for initial {initial:?}");
        }
    }

    #[test]
    fn fsm_low_health_flee() {
        // From Chase
        let mut fsm = AiStateMachine::new(AiState::Chase);
        let state = fsm.transition(AiTransition::LowHealth);
        assert_eq!(state, AiState::Flee);

        // From Attack
        let mut fsm2 = AiStateMachine::new(AiState::Attack);
        let state2 = fsm2.transition(AiTransition::LowHealth);
        assert_eq!(state2, AiState::Flee);
    }

    #[test]
    fn fsm_is_hostile_states() {
        let hostile_states = [AiState::Chase, AiState::Attack, AiState::Flee];
        let non_hostile = [AiState::Idle, AiState::Patrol, AiState::Alert, AiState::Dead];

        for s in hostile_states {
            let fsm = AiStateMachine::new(s);
            assert!(fsm.is_hostile(), "{s:?} should be hostile");
        }
        for s in non_hostile {
            let fsm = AiStateMachine::new(s);
            assert!(!fsm.is_hostile(), "{s:?} should NOT be hostile");
        }
    }

    #[test]
    fn fsm_can_move_states() {
        let mobile = [
            AiState::Idle,
            AiState::Patrol,
            AiState::Alert,
            AiState::Chase,
            AiState::Flee,
        ];
        let immobile = [AiState::Attack, AiState::Dead];

        for s in mobile {
            let fsm = AiStateMachine::new(s);
            assert!(fsm.can_move(), "{s:?} should be able to move");
        }
        for s in immobile {
            let fsm = AiStateMachine::new(s);
            assert!(!fsm.can_move(), "{s:?} should NOT be able to move");
        }
    }

    // ── Aggro range tests ──────────────────────────────────────────────

    #[test]
    fn aggro_range_can_see() {
        let range = AggroRange::new(10.0, 2.0);
        assert!(range.can_see(5.0));
        assert!(range.can_see(10.0));
        assert!(!range.can_see(10.1));
    }

    #[test]
    fn aggro_range_lost_target() {
        let range = AggroRange::new(10.0, 2.0);
        // lose_range = 10.0 * 1.5 = 15.0
        assert!(!range.has_lost_target(14.9));
        assert!(!range.has_lost_target(15.0));
        assert!(range.has_lost_target(15.1));
    }

    // ── Aggro system tests ─────────────────────────────────────────────

    #[test]
    fn aggro_system_spots_target() {
        let range = AggroRange::new(10.0, 2.0);
        let transitions = AggroSystem::evaluate(
            AiState::Idle,
            Some(8.0), // within sight
            1.0,       // full HP
            0.2,
            &range,
            0,
        );
        assert_eq!(transitions, vec![AiTransition::TargetSpotted]);
    }

    #[test]
    fn aggro_system_loses_target() {
        let range = AggroRange::new(10.0, 2.0);
        // lose_range = 15.0; distance 20.0 exceeds it
        let transitions = AggroSystem::evaluate(
            AiState::Chase,
            Some(20.0),
            1.0,
            0.2,
            &range,
            0,
        );
        assert_eq!(transitions, vec![AiTransition::TargetLost]);
    }

    #[test]
    fn aggro_system_low_hp_flee() {
        let range = AggroRange::new(10.0, 2.0);
        let transitions = AggroSystem::evaluate(
            AiState::Chase,
            Some(5.0),
            0.1, // 10% HP, below 20% threshold
            0.2,
            &range,
            0,
        );
        assert_eq!(transitions, vec![AiTransition::LowHealth]);
    }

    // ── Patrol path tests ──────────────────────────────────────────────

    #[test]
    fn patrol_path_advance_loop() {
        let mut path = PatrolPath::new(
            vec![(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)],
            PatrolLoop::Loop,
        );
        assert_eq!(path.current_target(), Some((0.0, 0.0)));

        assert!(!path.advance()); // -> index 1
        assert_eq!(path.current_target(), Some((1.0, 0.0)));

        assert!(!path.advance()); // -> index 2
        assert_eq!(path.current_target(), Some((2.0, 0.0)));

        assert!(!path.advance()); // -> wraps to index 0
        assert_eq!(path.current_target(), Some((0.0, 0.0)));
    }

    #[test]
    fn patrol_path_advance_ping_pong() {
        let mut path = PatrolPath::new(
            vec![(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)],
            PatrolLoop::PingPong,
        );
        // Forward: 0 -> 1 -> 2
        assert_eq!(path.current_target(), Some((0.0, 0.0)));
        assert!(!path.advance());
        assert_eq!(path.current_target(), Some((1.0, 0.0)));
        assert!(!path.advance());
        assert_eq!(path.current_target(), Some((2.0, 0.0)));

        // At end -- reverse: 2 -> 1
        assert!(!path.advance());
        assert_eq!(path.current_target(), Some((1.0, 0.0)));

        // Continue reverse: 1 -> 0
        assert!(!path.advance());
        assert_eq!(path.current_target(), Some((0.0, 0.0)));

        // At start -- forward again: 0 -> 1
        assert!(!path.advance());
        assert_eq!(path.current_target(), Some((1.0, 0.0)));
    }

    #[test]
    fn patrol_path_once_complete() {
        let mut path = PatrolPath::new(
            vec![(0.0, 0.0), (1.0, 0.0)],
            PatrolLoop::Once,
        );
        assert!(!path.advance()); // -> index 1
        assert_eq!(path.current_target(), Some((1.0, 0.0)));

        assert!(path.advance()); // complete!
        // Still at index 1 (last waypoint)
        assert_eq!(path.current_target(), Some((1.0, 0.0)));
    }

    // ── AI Agent integration test ──────────────────────────────────────

    #[test]
    fn ai_agent_update_spots_target() {
        let aggro = AggroRange::new(10.0, 2.0);
        let mut agent = AiAgent::new(1, aggro);
        assert_eq!(agent.fsm.current(), AiState::Idle);

        // Target at distance 8 -- within sight range.
        agent.update(Some(8.0), 1.0);
        assert_eq!(agent.fsm.current(), AiState::Alert);

        // Target still visible -- escalate to Chase.
        agent.update(Some(8.0), 1.0);
        assert_eq!(agent.fsm.current(), AiState::Chase);
    }

    // ── tick()-based FSM tests (TASK-042) ────────────────────────────────

    #[test]
    fn fsm_idle_to_chase() {
        // Target inside aggro range should move through Idle -> Alert -> Chase
        // after two ticks (Idle->Alert on first sight, Alert->Chase on confirm).
        let aggro = AggroRange::new(10.0, 2.0);
        let mut agent = AiAgent::with_attack_cooldown(1, aggro, 0.5);

        let dt = 0.016; // ~60 FPS frame
        let hp = 100;
        let max_hp = 100;
        let dist = 7.0; // inside aggro (10.0) but outside attack (2.0)

        // Tick 1: Idle -> Alert (target spotted).
        let state = agent.tick(hp, max_hp, dist, 10.0, 2.0, dt);
        assert_eq!(state, AiState::Alert);

        // Tick 2: Alert -> Chase (target still visible, escalation).
        let state = agent.tick(hp, max_hp, dist, 10.0, 2.0, dt);
        assert_eq!(state, AiState::Chase);
    }

    #[test]
    fn fsm_flee_low_hp() {
        // Agent below 20% HP while chasing should transition to Flee.
        let aggro = AggroRange::new(10.0, 2.0);
        let mut agent = AiAgent::with_attack_cooldown(1, aggro, 0.5);

        let dt = 0.016;

        // Get to Chase state first (Idle -> Alert -> Chase).
        agent.tick(100, 100, 7.0, 10.0, 2.0, dt);
        agent.tick(100, 100, 7.0, 10.0, 2.0, dt);
        assert_eq!(agent.fsm.current(), AiState::Chase);

        // Now tick with low HP (10/100 = 10%, below the 20% threshold).
        let state = agent.tick(10, 100, 7.0, 10.0, 2.0, dt);
        assert_eq!(state, AiState::Flee);
    }

    #[test]
    fn fsm_dead() {
        // 0 HP from any state should result in Dead.
        let aggro = AggroRange::new(10.0, 2.0);
        let mut agent = AiAgent::with_attack_cooldown(1, aggro, 0.5);

        let dt = 0.016;

        // From Idle with 0 HP.
        let state = agent.tick(0, 100, 50.0, 10.0, 2.0, dt);
        assert_eq!(state, AiState::Dead);

        // Also verify a second agent dying from Chase.
        let aggro2 = AggroRange::new(10.0, 2.0);
        let mut agent2 = AiAgent::with_attack_cooldown(2, aggro2, 0.5);
        agent2.tick(100, 100, 7.0, 10.0, 2.0, dt);
        agent2.tick(100, 100, 7.0, 10.0, 2.0, dt);
        assert_eq!(agent2.fsm.current(), AiState::Chase);

        let state2 = agent2.tick(0, 100, 7.0, 10.0, 2.0, dt);
        assert_eq!(state2, AiState::Dead);
    }

    #[test]
    fn fsm_attack_range() {
        // Target close enough should trigger Attack from Chase.
        let aggro = AggroRange::new(10.0, 2.0);
        let mut agent = AiAgent::with_attack_cooldown(1, aggro, 0.5);

        let dt = 0.016;

        // Idle -> Alert -> Chase.
        agent.tick(100, 100, 7.0, 10.0, 2.0, dt);
        agent.tick(100, 100, 7.0, 10.0, 2.0, dt);
        assert_eq!(agent.fsm.current(), AiState::Chase);

        // Target now at distance 1.5 -- within attack range (2.0).
        let state = agent.tick(100, 100, 1.5, 10.0, 2.0, dt);
        assert_eq!(state, AiState::Attack);
    }

    // ── Attack timer tests ──────────────────────────────────────────────

    #[test]
    fn attack_timer_decrements_in_attack_state() {
        let aggro = AggroRange::new(10.0, 2.0);
        let mut agent = AiAgent::with_attack_cooldown(1, aggro, 1.0);

        let dt = 0.016;

        // Drive to Attack state: Idle -> Alert -> Chase -> Attack.
        agent.tick(100, 100, 7.0, 10.0, 2.0, dt);
        agent.tick(100, 100, 7.0, 10.0, 2.0, dt);
        agent.tick(100, 100, 1.5, 10.0, 2.0, dt);
        assert_eq!(agent.fsm.current(), AiState::Attack);

        // First tick in Attack: timer starts at 0.0, so attack is ready
        // immediately (first attack fires right away).
        assert!(agent.is_attack_ready());

        // Simulate the caller resetting the timer after an attack.
        agent.reset_attack_timer();
        assert!(!agent.is_attack_ready());

        // Tick down: 1.0 - 0.5 = 0.5 (not ready yet).
        agent.tick(100, 100, 1.5, 10.0, 2.0, 0.5);
        assert!(!agent.is_attack_ready());

        // Tick down: 0.5 - 0.6 = -0.1 (ready).
        agent.tick(100, 100, 1.5, 10.0, 2.0, 0.6);
        assert!(agent.is_attack_ready());
    }

    #[test]
    fn attack_timer_resets_outside_attack_state() {
        let aggro = AggroRange::new(10.0, 2.0);
        let mut agent = AiAgent::with_attack_cooldown(1, aggro, 1.0);

        let dt = 0.016;

        // Drive to Attack state.
        agent.tick(100, 100, 7.0, 10.0, 2.0, dt);
        agent.tick(100, 100, 7.0, 10.0, 2.0, dt);
        agent.tick(100, 100, 1.5, 10.0, 2.0, dt);
        assert_eq!(agent.fsm.current(), AiState::Attack);

        // Reset timer (simulating attack was performed).
        agent.reset_attack_timer();
        assert!(!agent.is_attack_ready());

        // Target moves out of range -> back to Chase. Timer should reset.
        agent.tick(100, 100, 5.0, 10.0, 2.0, dt);
        assert_eq!(agent.fsm.current(), AiState::Chase);
        // attack_timer should be 0.0 now (reset outside Attack).
        assert!(agent.attack_timer <= 0.0);
    }
}
