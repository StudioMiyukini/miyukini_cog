//! Entity interpolation for *remote* entities (other players, mobs).
//!
//! Snapshots arrive at the server tick rate (20 Hz) but the client renders at
//! 60 fps. To avoid stutter, each remote entity keeps a small history of
//! timestamped positions and is rendered slightly in the past (an interpolation
//! delay of ~100 ms), sampling between the two snapshots that bracket the
//! render time. This trades a little latency for smooth motion.

use std::collections::VecDeque;

/// Default render-behind delay, in seconds (~2 snapshots at 20 Hz).
pub const INTERP_DELAY: f64 = 0.1;

#[derive(Debug, Clone, Copy)]
struct Sample {
    t: f64,
    pos: [f32; 2],
}

/// A bounded history of an entity's positions, sampled by interpolation.
#[derive(Debug, Clone, Default)]
pub struct InterpBuffer {
    samples: VecDeque<Sample>,
}

const MAX_SAMPLES: usize = 32;

impl InterpBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record an authoritative position observed at absolute time `t` (seconds).
    pub fn push(&mut self, t: f64, pos: [f32; 2]) {
        // Ignore out-of-order/duplicate timestamps.
        if self.samples.back().is_some_and(|s| t <= s.t) {
            return;
        }
        self.samples.push_back(Sample { t, pos });
        while self.samples.len() > MAX_SAMPLES {
            self.samples.pop_front();
        }
    }

    /// Sample the interpolated position for render time `now` (seconds),
    /// rendering `delay` behind to interpolate between known snapshots.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn sample(&self, now: f64, delay: f64) -> Option<[f32; 2]> {
        let target = now - delay;
        if self.samples.is_empty() {
            return None;
        }
        // Before the first sample: clamp to it.
        if target <= self.samples[0].t {
            return Some(self.samples[0].pos);
        }
        // Find the bracketing pair [a, b] with a.t <= target <= b.t.
        for w in self.samples.iter().zip(self.samples.iter().skip(1)) {
            let (a, b) = w;
            if target >= a.t && target <= b.t {
                let span = (b.t - a.t).max(1e-6);
                let f = ((target - a.t) / span) as f32;
                return Some([
                    a.pos[0] + (b.pos[0] - a.pos[0]) * f,
                    a.pos[1] + (b.pos[1] - a.pos[1]) * f,
                ]);
            }
        }
        // Past the last sample: clamp (extrapolation avoided).
        self.samples.back().map(|s| s.pos)
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolates_midpoint() {
        let mut buf = InterpBuffer::new();
        buf.push(0.0, [0.0, 0.0]);
        buf.push(1.0, [10.0, 0.0]);
        // Render at now=1.0 with delay=0.5 → target 0.5 → midpoint.
        let p = buf.sample(1.0, 0.5).unwrap();
        assert!((p[0] - 5.0).abs() < 1e-3, "got {}", p[0]);
    }

    #[test]
    fn clamps_before_first_sample() {
        let mut buf = InterpBuffer::new();
        buf.push(5.0, [2.0, 3.0]);
        let p = buf.sample(5.0, 1.0).unwrap(); // target 4.0 < 5.0
        assert_eq!(p, [2.0, 3.0]);
    }

    #[test]
    fn clamps_after_last_sample() {
        let mut buf = InterpBuffer::new();
        buf.push(0.0, [0.0, 0.0]);
        buf.push(1.0, [10.0, 0.0]);
        let p = buf.sample(5.0, 0.0).unwrap(); // target 5.0 > 1.0
        assert_eq!(p, [10.0, 0.0]);
    }

    #[test]
    fn ignores_out_of_order() {
        let mut buf = InterpBuffer::new();
        buf.push(1.0, [1.0, 0.0]);
        buf.push(0.5, [99.0, 0.0]); // stale, ignored
        let p = buf.sample(1.0, 0.0).unwrap();
        assert_eq!(p, [1.0, 0.0]);
    }
}
