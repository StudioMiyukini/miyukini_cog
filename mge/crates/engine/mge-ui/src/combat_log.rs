// @id: MGE-UI-CombatLog @do: combat-log @role: front-end @layer: 3 @human: miyuk
//! Combat log data model — FIFO queue of coloured combat messages.

use std::collections::VecDeque;

// ---------------------------------------------------------------------------
// LogEntry
// ---------------------------------------------------------------------------

/// A single combat log message with an RGBA display colour.
#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    /// Human-readable combat message.
    pub message: String,
    /// RGBA colour components, each in `[0.0, 1.0]`.
    pub color: [f32; 4],
}

// ---------------------------------------------------------------------------
// CombatLogState
// ---------------------------------------------------------------------------

/// Ordered queue of combat log entries, capped at `max_entries`.
///
/// When the queue is full and a new message arrives the oldest entry is
/// removed first (FIFO).
#[derive(Debug, Clone)]
pub struct CombatLogState {
    entries: VecDeque<LogEntry>,
    max_entries: usize,
}

impl CombatLogState {
    /// Create a new `CombatLogState` with the default cap of 100 entries.
    pub fn new() -> Self {
        Self {
            entries: VecDeque::new(),
            max_entries: 100,
        }
    }

    /// Push a new entry. Removes the oldest entry when the cap is exceeded.
    pub fn push(&mut self, message: impl Into<String>, color: [f32; 4]) {
        while self.entries.len() >= self.max_entries {
            self.entries.pop_front();
        }
        self.entries.push_back(LogEntry {
            message: message.into(),
            color,
        });
    }

    /// Read-only access to the entry queue in insertion order (oldest first).
    pub fn entries(&self) -> &VecDeque<LogEntry> {
        &self.entries
    }

    /// Remove all entries from the log.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Default for CombatLogState {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combat_log_fifo() {
        let mut log = CombatLogState::new();
        // Push 101 entries — only the last 100 must remain.
        for i in 0..101_usize {
            log.push(format!("msg-{i}"), [1.0, 1.0, 1.0, 1.0]);
        }
        assert_eq!(log.entries().len(), 100, "must retain exactly 100 entries");
        // The oldest entry (msg-0) must have been evicted; msg-1 is now first.
        assert_eq!(
            log.entries().front().map(|e| e.message.as_str()),
            Some("msg-1"),
            "oldest entry must be evicted first"
        );
    }

    #[test]
    fn combat_log_color() {
        let mut log = CombatLogState::new();
        let red = [1.0_f32, 0.0, 0.0, 1.0];
        log.push("hit", red);
        let entry = log.entries().front().expect("entry must exist");
        assert_eq!(entry.color, red, "entry color must match what was pushed");
    }
}
