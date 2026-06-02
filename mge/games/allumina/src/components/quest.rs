//! Quest log for Act I. Five simple objectives tracked by event hooks (kills,
//! pickups) and per-tick polling (gold, current zone).

use crate::components::world::ZoneId;

/// What a quest asks of the player.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestKind {
    /// Slay `n` hostiles of any kind.
    KillAny(u32),
    /// Slay the act boss.
    KillBoss,
    /// Accumulate `n` gold total.
    CollectGold(u64),
    /// Pick up `n` items.
    CollectItems(u32),
    /// Reach a specific zone.
    ReachZone(ZoneId),
}

/// One quest with live progress.
#[derive(Debug, Clone)]
pub struct Quest {
    pub id: u32,
    pub title: String,
    pub kind: QuestKind,
    pub progress: u32,
    pub done: bool,
    pub reward_gold: u64,
    pub reward_xp: u64,
}

impl Quest {
    /// Target value the progress counts toward.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn goal(&self) -> u32 {
        match self.kind {
            QuestKind::KillAny(n) | QuestKind::CollectItems(n) => n,
            QuestKind::KillBoss | QuestKind::ReachZone(_) => 1,
            QuestKind::CollectGold(g) => g.min(u64::from(u32::MAX)) as u32,
        }
    }

    #[must_use]
    pub fn reached(&self) -> bool {
        self.progress >= self.goal()
    }
}

/// The player's quest log.
#[derive(Debug, Clone, Default)]
pub struct QuestLog {
    pub quests: Vec<Quest>,
}

impl QuestLog {
    /// The five Act I quests.
    pub fn act1() -> Self {
        let quests = vec![
            Quest { id: 1, title: "Cull the Husks".into(), kind: QuestKind::KillAny(6), progress: 0, done: false, reward_gold: 100, reward_xp: 200 },
            Quest { id: 2, title: "Spoils of War".into(), kind: QuestKind::CollectItems(3), progress: 0, done: false, reward_gold: 80, reward_xp: 120 },
            Quest { id: 3, title: "Fill the Coffers".into(), kind: QuestKind::CollectGold(500), progress: 0, done: false, reward_gold: 0, reward_xp: 150 },
            Quest { id: 4, title: "Descent".into(), kind: QuestKind::ReachZone(ZoneId::WerewolfDen), progress: 0, done: false, reward_gold: 120, reward_xp: 200 },
            Quest { id: 5, title: "Slay the Maw".into(), kind: QuestKind::KillBoss, progress: 0, done: false, reward_gold: 1000, reward_xp: 5000 },
        ];
        Self { quests }
    }

    /// Record a kill (boss or not), advancing matching quests.
    pub fn record_kill(&mut self, is_boss: bool) {
        for q in self.quests.iter_mut().filter(|q| !q.done) {
            match q.kind {
                QuestKind::KillAny(_) => q.progress += 1,
                QuestKind::KillBoss if is_boss => q.progress = 1,
                _ => {}
            }
        }
    }

    /// Record an item pickup.
    pub fn record_item(&mut self) {
        for q in self.quests.iter_mut().filter(|q| !q.done) {
            if matches!(q.kind, QuestKind::CollectItems(_)) {
                q.progress += 1;
            }
        }
    }

    /// Update gold-total quests.
    #[allow(clippy::cast_possible_truncation)]
    pub fn set_gold(&mut self, gold: u64) {
        for q in self.quests.iter_mut().filter(|q| !q.done) {
            if let QuestKind::CollectGold(goal) = q.kind {
                q.progress = gold.min(goal).min(u64::from(u32::MAX)) as u32;
            }
        }
    }

    /// Update zone-reached quests.
    pub fn set_zone(&mut self, zone: ZoneId) {
        for q in self.quests.iter_mut().filter(|q| !q.done) {
            if matches!(q.kind, QuestKind::ReachZone(z) if z == zone) {
                q.progress = 1;
            }
        }
    }

    /// Mark newly-finished quests done and return their `(title, gold, xp)`
    /// rewards for the caller to grant.
    pub fn take_completed(&mut self) -> Vec<(String, u64, u64)> {
        let mut out = Vec::new();
        for q in self.quests.iter_mut() {
            if !q.done && q.reached() {
                q.done = true;
                out.push((q.title.clone(), q.reward_gold, q.reward_xp));
            }
        }
        out
    }

    pub fn completed_count(&self) -> usize {
        self.quests.iter().filter(|q| q.done).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kills_advance_and_complete() {
        let mut log = QuestLog::act1();
        for _ in 0..6 {
            log.record_kill(false);
        }
        let done = log.take_completed();
        assert!(done.iter().any(|(t, _, _)| t == "Cull the Husks"));
    }

    #[test]
    fn boss_quest_needs_boss() {
        let mut log = QuestLog::act1();
        log.record_kill(false);
        assert!(!log.quests.iter().find(|q| q.id == 5).unwrap().reached());
        log.record_kill(true);
        assert!(log.quests.iter().find(|q| q.id == 5).unwrap().reached());
    }

    #[test]
    fn gold_and_zone_polling() {
        let mut log = QuestLog::act1();
        log.set_gold(500);
        log.set_zone(ZoneId::WerewolfDen);
        let done = log.take_completed();
        assert!(done.iter().any(|(t, _, _)| t == "Fill the Coffers"));
        assert!(done.iter().any(|(t, _, _)| t == "Descent"));
    }
}
