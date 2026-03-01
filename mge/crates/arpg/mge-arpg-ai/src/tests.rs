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
}
