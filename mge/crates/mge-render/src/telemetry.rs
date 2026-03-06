use serde::{Deserialize, Serialize};

/// Per-frame rendering telemetry snapshot.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FrameTelemetry {
    pub draw_calls: u32,
    pub sprite_instances: u32,
    pub vfx_particles: u32,
    pub point_lights: u32,
    pub atlas_memory_bytes: u64,
    pub frame_time_us: u64,
}

/// Ring buffer of recent frame telemetry for overlay display.
#[derive(Debug, Clone)]
pub struct TelemetryRing {
    frames: Vec<FrameTelemetry>,
    capacity: usize,
    write_idx: usize,
    count: usize,
}

impl TelemetryRing {
    pub fn new(capacity: usize) -> Self {
        Self { frames: vec![FrameTelemetry::default(); capacity], capacity, write_idx: 0, count: 0 }
    }

    /// Record a new frame's telemetry.
    pub fn push(&mut self, frame: FrameTelemetry) {
        self.frames[self.write_idx] = frame;
        self.write_idx = (self.write_idx + 1) % self.capacity;
        if self.count < self.capacity {
            self.count += 1;
        }
    }

    /// Average frame time over recorded frames (microseconds).
    pub fn avg_frame_time_us(&self) -> u64 {
        if self.count == 0 {
            return 0;
        }
        let sum: u64 = self.recent(self.count).iter().map(|f| f.frame_time_us).sum();
        sum / self.count as u64
    }

    /// Latest frame telemetry.
    pub fn latest(&self) -> Option<&FrameTelemetry> {
        if self.count == 0 {
            return None;
        }
        let idx = (self.write_idx + self.capacity - 1) % self.capacity;
        Some(&self.frames[idx])
    }

    /// Get the N most recent frames (oldest first).
    pub fn recent(&self, n: usize) -> Vec<&FrameTelemetry> {
        let n = n.min(self.count);
        let mut result = Vec::with_capacity(n);
        for i in 0..n {
            let idx = (self.write_idx + self.capacity - n + i) % self.capacity;
            result.push(&self.frames[idx]);
        }
        result
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

/// Atlas streaming state tracker.
#[derive(Debug, Clone, Default)]
pub struct AtlasStreamState {
    pub loaded_atlases: u32,
    pub pending_loads: u32,
    pub total_memory_bytes: u64,
}

impl AtlasStreamState {
    pub fn record_load(&mut self, size_bytes: u64) {
        self.loaded_atlases += 1;
        self.pending_loads = self.pending_loads.saturating_sub(1);
        self.total_memory_bytes += size_bytes;
    }

    pub fn request_load(&mut self) {
        self.pending_loads += 1;
    }

    pub fn record_unload(&mut self, size_bytes: u64) {
        self.loaded_atlases = self.loaded_atlases.saturating_sub(1);
        self.total_memory_bytes = self.total_memory_bytes.saturating_sub(size_bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_records_and_averages() {
        let mut ring = TelemetryRing::new(4);
        ring.push(FrameTelemetry { frame_time_us: 16000, ..Default::default() });
        ring.push(FrameTelemetry { frame_time_us: 18000, ..Default::default() });
        assert_eq!(ring.count(), 2);
        assert_eq!(ring.avg_frame_time_us(), 17000);
    }

    #[test]
    fn ring_wraps_around() {
        let mut ring = TelemetryRing::new(2);
        ring.push(FrameTelemetry { draw_calls: 10, ..Default::default() });
        ring.push(FrameTelemetry { draw_calls: 20, ..Default::default() });
        ring.push(FrameTelemetry { draw_calls: 30, ..Default::default() });
        assert_eq!(ring.count(), 2);
        let latest = ring.latest().expect("latest");
        assert_eq!(latest.draw_calls, 30);
    }

    #[test]
    fn recent_returns_oldest_first() {
        let mut ring = TelemetryRing::new(4);
        for i in 1..=3 {
            ring.push(FrameTelemetry { draw_calls: i, ..Default::default() });
        }
        let recent = ring.recent(3);
        assert_eq!(recent[0].draw_calls, 1);
        assert_eq!(recent[2].draw_calls, 3);
    }

    #[test]
    fn atlas_stream_state_tracks_memory() {
        let mut state = AtlasStreamState::default();
        state.request_load();
        assert_eq!(state.pending_loads, 1);
        state.record_load(1024 * 1024);
        assert_eq!(state.loaded_atlases, 1);
        assert_eq!(state.pending_loads, 0);
        assert_eq!(state.total_memory_bytes, 1024 * 1024);
        state.record_unload(1024 * 1024);
        assert_eq!(state.loaded_atlases, 0);
        assert_eq!(state.total_memory_bytes, 0);
    }

    #[test]
    fn empty_ring_returns_none() {
        let ring = TelemetryRing::new(4);
        assert!(ring.latest().is_none());
        assert_eq!(ring.avg_frame_time_us(), 0);
    }
}
