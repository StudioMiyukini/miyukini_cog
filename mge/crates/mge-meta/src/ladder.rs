use serde::{Deserialize, Serialize};

/// Type of ladder season.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LadderType {
    Standard,
    Hardcore,
    Expansion,
    HardcoreExpansion,
}

/// A single ranked entry on the ladder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderEntry {
    pub rank: u32,
    pub player_id: u64,
    pub character_name: String,
    pub level: u32,
    pub class_name: String,
    pub ladder_type: LadderType,
    /// Accumulated ladder experience.
    pub ladder_xp: u64,
}

/// A ladder board for one season and type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderBoard {
    pub season: u32,
    pub ladder_type: LadderType,
    pub entries: Vec<LadderEntry>,
    pub reset_policy_description: String,
}

impl LadderBoard {
    #[must_use]
    pub fn new(season: u32, ladder_type: LadderType) -> Self {
        Self {
            season,
            ladder_type,
            entries: Vec::new(),
            reset_policy_description: "Season resets after 6 months; top 20 get cosmetics.".into(),
        }
    }

    /// Insert or update an entry, then re-rank by `ladder_xp` descending.
    pub fn upsert(&mut self, mut entry: LadderEntry) {
        if let Some(existing) = self
            .entries
            .iter_mut()
            .find(|e| e.player_id == entry.player_id)
        {
            existing.ladder_xp = entry.ladder_xp;
            existing.level = entry.level;
        } else {
            entry.rank = 0; // will be set below
            self.entries.push(entry);
        }
        self.rerank();
    }

    #[allow(clippy::cast_possible_truncation)]
    fn rerank(&mut self) {
        self.entries.sort_by(|a, b| b.ladder_xp.cmp(&a.ladder_xp));
        for (i, e) in self.entries.iter_mut().enumerate() {
            e.rank = (i + 1) as u32;
        }
    }

    #[must_use]
    pub fn top(&self, n: usize) -> Vec<&LadderEntry> {
        self.entries.iter().take(n).collect()
    }

    /// Simulate a season reset: archive entries and clear for a new season.
    pub fn reset(&mut self) {
        self.season += 1;
        self.entries.clear();
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(player_id: u64, xp: u64) -> LadderEntry {
        LadderEntry {
            rank: 0,
            player_id,
            character_name: format!("Char{player_id}"),
            level: 10,
            class_name: "Sorceress".into(),
            ladder_type: LadderType::Standard,
            ladder_xp: xp,
        }
    }

    #[test]
    fn upsert_ranks_by_xp() {
        let mut board = LadderBoard::new(1, LadderType::Standard);
        board.upsert(entry(1, 1_000));
        board.upsert(entry(2, 5_000));
        assert_eq!(board.top(1)[0].player_id, 2);
    }

    #[test]
    fn upsert_updates_existing_entry() {
        let mut board = LadderBoard::new(1, LadderType::Standard);
        board.upsert(entry(1, 1_000));
        board.upsert(entry(1, 9_000));
        assert_eq!(board.entries.len(), 1);
        assert_eq!(board.entries[0].ladder_xp, 9_000);
    }

    #[test]
    fn reset_clears_entries_and_bumps_season() {
        let mut board = LadderBoard::new(1, LadderType::Hardcore);
        board.upsert(entry(1, 100));
        board.reset();
        assert_eq!(board.season, 2);
        assert!(board.entries.is_empty());
    }

    #[test]
    fn top_n_returns_at_most_n() {
        let mut board = LadderBoard::new(1, LadderType::Standard);
        for i in 1..=5 {
            board.upsert(entry(i, u64::from(i) * 100));
        }
        assert_eq!(board.top(3).len(), 3);
    }
}
