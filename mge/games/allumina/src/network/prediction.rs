//! Client-side movement prediction and server reconciliation.
//!
//! The client applies its own movement inputs immediately (so the local avatar
//! responds at 60 fps without waiting for the 20 Hz server), tagging each with
//! a monotonic sequence number. When an authoritative snapshot arrives carrying
//! the last input the server processed (`ack_seq`), the client snaps to the
//! authoritative position and *replays* every still-unacknowledged input on top
//! — converging to the server while staying responsive.

use std::collections::VecDeque;

/// One movement input sample.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Input {
    pub seq: u32,
    /// Movement direction (need not be normalized; [`apply`] normalizes).
    pub dx: f32,
    pub dy: f32,
    pub dt: f32,
}

/// Integrate one input onto a position at `speed` tiles/sec.
#[must_use]
pub fn apply(pos: [f32; 2], input: Input, speed: f32) -> [f32; 2] {
    let len = (input.dx * input.dx + input.dy * input.dy).sqrt();
    if len <= 1e-6 {
        return pos;
    }
    let step = speed * input.dt;
    [pos[0] + input.dx / len * step, pos[1] + input.dy / len * step]
}

/// Predicts local movement and reconciles against server snapshots.
#[derive(Debug, Clone)]
pub struct Predictor {
    pub pos: [f32; 2],
    pub speed: f32,
    pending: VecDeque<Input>,
    next_seq: u32,
}

impl Predictor {
    pub fn new(pos: [f32; 2], speed: f32) -> Self {
        Self { pos, speed, pending: VecDeque::new(), next_seq: 1 }
    }

    /// Apply a local input immediately and queue it for reconciliation.
    /// Returns the sequence number to send to the server.
    pub fn push_input(&mut self, dx: f32, dy: f32, dt: f32) -> u32 {
        let seq = self.next_seq;
        self.next_seq += 1;
        let input = Input { seq, dx, dy, dt };
        self.pos = apply(self.pos, input, self.speed);
        self.pending.push_back(input);
        seq
    }

    /// Number of inputs awaiting acknowledgement.
    pub fn pending_len(&self) -> usize {
        self.pending.len()
    }

    /// Reconcile against the server's authoritative position. `ack_seq` is the
    /// last input the server has applied for us. Drops acknowledged inputs and
    /// replays the rest on top of the authoritative position.
    pub fn reconcile(&mut self, authoritative: [f32; 2], ack_seq: u32) {
        while self.pending.front().is_some_and(|i| i.seq <= ack_seq) {
            self.pending.pop_front();
        }
        let mut pos = authoritative;
        for input in &self.pending {
            pos = apply(pos, *input, self.speed);
        }
        self.pos = pos;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_moves_along_normalized_direction() {
        let p = apply([0.0, 0.0], Input { seq: 1, dx: 3.0, dy: 0.0, dt: 0.5 }, 4.0);
        // speed 4 * dt 0.5 = 2.0 along +x
        assert!((p[0] - 2.0).abs() < 1e-4);
        assert!(p[1].abs() < 1e-4);
    }

    #[test]
    fn prediction_advances_immediately() {
        let mut pred = Predictor::new([0.0, 0.0], 5.0);
        pred.push_input(1.0, 0.0, 0.1);
        assert!(pred.pos[0] > 0.0);
        assert_eq!(pred.pending_len(), 1);
    }

    #[test]
    fn full_ack_snaps_to_authoritative() {
        let mut pred = Predictor::new([0.0, 0.0], 5.0);
        let s1 = pred.push_input(1.0, 0.0, 0.1);
        let s2 = pred.push_input(1.0, 0.0, 0.1);
        // Server applied both; authoritative slightly behind its own integration.
        pred.reconcile([0.95, 0.0], s2.max(s1));
        assert_eq!(pred.pending_len(), 0);
        assert!((pred.pos[0] - 0.95).abs() < 1e-4);
    }

    #[test]
    fn partial_ack_replays_unacked_inputs() {
        let mut pred = Predictor::new([0.0, 0.0], 10.0);
        let s1 = pred.push_input(1.0, 0.0, 0.1); // +1.0
        let _s2 = pred.push_input(1.0, 0.0, 0.1); // +1.0 (still pending)
        // Server acked only the first input; authoritative reflects just that.
        pred.reconcile([1.0, 0.0], s1);
        assert_eq!(pred.pending_len(), 1);
        // Authoritative 1.0 + replayed unacked input (+1.0) = 2.0
        assert!((pred.pos[0] - 2.0).abs() < 1e-4, "pos was {}", pred.pos[0]);
    }
}
