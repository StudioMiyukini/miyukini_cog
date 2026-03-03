// @id: Sodomight-Server-Validation @do: move-validation @role: back-end @layer: 4 @human: miyuk
//! Server-side `PlayerMove` validation (speed hack prevention and rate limiting).

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Tracks the number of move events received in the current 1-second window
/// for a single player.
struct MoveRateTracker {
    /// Number of moves received in the current window.
    count: u32,
    /// Timestamp (ms) at which the current window started.
    window_start_ms: u64,
}

/// Validates incoming player move messages on the authoritative server.
///
/// Detects speed hacks (delta exceeds physics-allowed maximum) and rate
/// limiting (too many move messages per second, indicating client tampering).
pub struct MoveValidator {
    /// Maximum speed in units per second (base character speed).
    max_speed: f32,
    /// Tolerance factor to absorb network jitter (e.g. 1.1 = +10%).
    tolerance: f32,
    /// Per-player rate tracking windows (keyed by player id).
    move_counts: HashMap<u64, MoveRateTracker>,
    /// Hard cap on move messages accepted per player per second.
    max_moves_per_second: u32,
}

/// Result of a single move validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveValidationResult {
    /// Move is valid — apply it.
    Accepted,
    /// Move exceeds the maximum allowed speed (speed hack detected).
    SpeedHack {
        /// Received horizontal delta (scaled to integer units for display).
        dx: i32,
        /// Received vertical delta (scaled to integer units for display).
        dy: i32,
        /// Maximum absolute delta allowed in this tick.
        max_allowed: i32,
    },
    /// Too many moves in the current 1-second window.
    RateLimited {
        /// Number of moves already seen this window.
        count: u32,
        /// Configured maximum.
        max: u32,
    },
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl MoveValidator {
    /// Create a new `MoveValidator` with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `max_speed` — base character speed in units per second.
    /// * `tolerance` — multiplier applied to `max_speed` to allow for network
    ///   jitter (typical value: 1.1).
    /// * `max_moves_per_second` — hard cap on move messages per player per
    ///   second.
    #[must_use]
    pub fn new(max_speed: f32, tolerance: f32, max_moves_per_second: u32) -> Self {
        Self {
            max_speed,
            tolerance,
            move_counts: HashMap::new(),
            max_moves_per_second,
        }
    }

    /// Validate a player move message.
    ///
    /// # Arguments
    ///
    /// * `player_id` — unique identifier of the moving player.
    /// * `dx` — horizontal displacement requested by the client.
    /// * `dy` — vertical displacement requested by the client.
    /// * `dt` — elapsed time in seconds since the previous server tick.
    /// * `current_time_ms` — current server wall-clock in milliseconds, used
    ///   for the rate-limiting window.
    ///
    /// # Returns
    ///
    /// [`MoveValidationResult::Accepted`] when both the speed check and the
    /// rate check pass.  Otherwise the appropriate rejection variant.
    pub fn validate_move(
        &mut self,
        player_id: u64,
        dx: f32,
        dy: f32,
        dt: f32,
        current_time_ms: u64,
    ) -> MoveValidationResult {
        // --- 1. Speed check ---------------------------------------------------
        let max_delta = self.max_speed * dt * self.tolerance;

        if dx.abs() > max_delta || dy.abs() > max_delta {
            return MoveValidationResult::SpeedHack {
                dx: dx as i32,
                dy: dy as i32,
                max_allowed: max_delta as i32,
            };
        }

        // --- 2. Rate-limit check ----------------------------------------------
        let tracker = self
            .move_counts
            .entry(player_id)
            .or_insert(MoveRateTracker {
                count: 0,
                window_start_ms: current_time_ms,
            });

        // Reset window if more than 1 000 ms have elapsed.
        if current_time_ms.saturating_sub(tracker.window_start_ms) >= 1_000 {
            tracker.count = 0;
            tracker.window_start_ms = current_time_ms;
        }

        tracker.count += 1;

        if tracker.count > self.max_moves_per_second {
            return MoveValidationResult::RateLimited {
                count: tracker.count,
                max: self.max_moves_per_second,
            };
        }

        MoveValidationResult::Accepted
    }
}

// ---------------------------------------------------------------------------
// Default configuration
// ---------------------------------------------------------------------------

/// Build a `MoveValidator` with sensible defaults for Sodomight:
/// - 6 units/second base speed
/// - 10% tolerance for network jitter
/// - 25 move messages per second per player
#[must_use]
pub fn default_move_validator() -> MoveValidator {
    MoveValidator::new(
        6.0, // max speed: 6 units/second
        1.1, // 10% tolerance for network jitter
        25,  // max 25 moves per second
    )
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Helpers ----------------------------------------------------------------

    fn make_validator() -> MoveValidator {
        default_move_validator()
    }

    // max_speed=6, tolerance=1.1, dt=0.05 -> max_delta = 6 * 0.05 * 1.1 = 0.33
    const DT: f32 = 0.05; // 20 Hz tick
    const T0: u64 = 1_000_000; // arbitrary wall-clock start

    // Tests ------------------------------------------------------------------

    /// Normal delta well within limits -> Accepted.
    #[test]
    fn validate_move_normal() {
        let mut v = make_validator();
        // dx=0.1, dy=0.1 << max_delta 0.33
        let result = v.validate_move(1, 0.1, 0.1, DT, T0);
        assert_eq!(result, MoveValidationResult::Accepted);
    }

    /// dx=100 is far beyond any reasonable tick delta -> SpeedHack.
    #[test]
    fn validate_move_speed_hack() {
        let mut v = make_validator();
        let result = v.validate_move(2, 100.0, 0.0, DT, T0);
        assert!(
            matches!(result, MoveValidationResult::SpeedHack { .. }),
            "expected SpeedHack, got {result:?}",
        );
    }

    /// 26 moves within the same 1-second window triggers RateLimited on the
    /// 26th move.
    #[test]
    fn validate_move_rate_limit() {
        let mut v = make_validator();
        // max_moves_per_second = 25; send 25 valid moves
        for i in 0_u32..25 {
            let result = v.validate_move(3, 0.0, 0.0, DT, T0 + u64::from(i) * 10);
            assert_eq!(
                result,
                MoveValidationResult::Accepted,
                "move {i} should be Accepted"
            );
        }
        // 26th move in the same second -> RateLimited
        let result = v.validate_move(3, 0.0, 0.0, DT, T0 + 250);
        assert!(
            matches!(result, MoveValidationResult::RateLimited { .. }),
            "expected RateLimited, got {result:?}",
        );
    }

    /// Delta slightly over base speed but within the tolerance band -> Accepted.
    ///
    /// max_speed=6, dt=0.05 -> base_max=0.30; tolerance 1.1 -> 0.33.
    /// We send dx=0.32 which is > 0.30 (base) but < 0.33 (toleranced).
    #[test]
    fn validate_move_tolerance() {
        let mut v = make_validator();
        // 0.32 > 0.30 (strict base) but < 0.33 (with 10% tolerance)
        let result = v.validate_move(4, 0.32, 0.0, DT, T0);
        assert_eq!(result, MoveValidationResult::Accepted);
    }

    /// After 1 000 ms the window resets; a fresh burst of 25 should all pass.
    #[test]
    fn validate_move_rate_window_resets() {
        let mut v = make_validator();
        // Fill window at T0
        for i in 0_u32..25 {
            let _ = v.validate_move(5, 0.0, 0.0, DT, T0 + u64::from(i) * 10);
        }
        // 1 001 ms later -> new window
        let result = v.validate_move(5, 0.0, 0.0, DT, T0 + 1_001);
        assert_eq!(result, MoveValidationResult::Accepted);
    }
}
