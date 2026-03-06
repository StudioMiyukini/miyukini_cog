use mge_proto::{DeltaSnapshot, SnapshotEnvelope};
use serde::{Deserialize, Serialize};

/// A spatial cell used for interest management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterestCell {
    pub x: i32,
    pub y: i32,
    pub radius: u32,
}

impl InterestCell {
    /// Returns `true` if `(px, py)` falls within this cell's radius.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn contains(&self, px: i32, py: i32) -> bool {
        let dx = (px - self.x).unsigned_abs();
        let dy = (py - self.y).unsigned_abs();
        dx <= self.radius && dy <= self.radius
    }
}

/// Full snapshot + interest management plan for a client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationPlan {
    pub snapshot: SnapshotEnvelope,
    pub cells: Vec<InterestCell>,
}

impl ReplicationPlan {
    #[must_use]
    pub fn bootstrap(scene_id: impl Into<String>) -> Self {
        Self {
            snapshot: SnapshotEnvelope::bootstrap(scene_id),
            cells: vec![InterestCell { x: 0, y: 0, radius: 2 }],
        }
    }

    /// Add an interest cell to the plan.
    pub fn add_cell(&mut self, cell: InterestCell) {
        self.cells.push(cell);
    }

    /// Returns `true` if any cell covers `(px, py)`.
    #[must_use]
    pub fn is_in_interest(&self, px: i32, py: i32) -> bool {
        self.cells.iter().any(|c| c.contains(px, py))
    }
}

/// A set of delta snapshots being accumulated for a single connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaAccumulator {
    pub scene_id: String,
    pub baseline_tick: u64,
    pub pending: Vec<DeltaSnapshot>,
}

impl DeltaAccumulator {
    #[must_use]
    pub fn new(scene_id: impl Into<String>, baseline_tick: u64) -> Self {
        Self { scene_id: scene_id.into(), baseline_tick, pending: Vec::new() }
    }

    /// Push a new delta. Returns the current pending count.
    pub fn push(&mut self, delta: DeltaSnapshot) -> usize {
        self.pending.push(delta);
        self.pending.len()
    }

    /// Drain all pending deltas (e.g., for flush to network).
    pub fn drain(&mut self) -> Vec<DeltaSnapshot> {
        std::mem::take(&mut self.pending)
    }

    #[must_use]
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use mge_proto::DeltaSnapshot;

    #[test]
    fn interest_cell_contains_origin() {
        let cell = InterestCell { x: 0, y: 0, radius: 5 };
        assert!(cell.contains(0, 0));
        assert!(cell.contains(5, 5));
        assert!(!cell.contains(6, 0));
    }

    #[test]
    fn replication_plan_in_interest() {
        let mut plan = ReplicationPlan::bootstrap("zone");
        plan.add_cell(InterestCell { x: 10, y: 10, radius: 3 });
        assert!(plan.is_in_interest(10, 10));
        assert!(!plan.is_in_interest(20, 20));
    }

    #[test]
    fn delta_accumulator_push_and_drain() {
        let mut acc = DeltaAccumulator::new("zone", 0);
        let delta = DeltaSnapshot::new("zone", 1, 0);
        assert_eq!(acc.push(delta), 1);
        let drained = acc.drain();
        assert_eq!(drained.len(), 1);
        assert_eq!(acc.pending_count(), 0);
    }

    #[test]
    fn replication_plan_bootstrap_tick_zero() {
        let plan = ReplicationPlan::bootstrap("act1");
        assert_eq!(plan.snapshot.tick, 0);
    }
}
