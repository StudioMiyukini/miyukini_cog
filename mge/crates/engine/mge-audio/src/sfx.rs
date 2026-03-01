//! SFX pool -- manages a bounded number of concurrent short sounds.

/// Lightweight pool that tracks active sound-effect voices.
///
/// When `max_concurrent` is reached the oldest entry is evicted (FIFO).
#[derive(Debug, Clone)]
pub struct SfxPool {
    max_concurrent: usize,
    active: Vec<String>,
}

impl SfxPool {
    /// Creates a new pool with the given concurrency cap.
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            max_concurrent,
            active: Vec::new(),
        }
    }

    /// Records a new SFX play.
    ///
    /// If the pool is at capacity the oldest entry is evicted first
    /// (FIFO order).
    pub fn play(&mut self, sound_id: &str) {
        if self.active.len() >= self.max_concurrent {
            self.active.remove(0);
        }
        self.active.push(sound_id.to_owned());
    }

    /// Number of currently active voices.
    pub fn active_count(&self) -> usize {
        self.active.len()
    }

    /// Clears every active voice.
    pub fn clear(&mut self) -> &mut Self {
        self.active.clear();
        self
    }
}
