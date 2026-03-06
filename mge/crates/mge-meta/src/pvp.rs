use serde::{Deserialize, Serialize};

/// Consent level for `PvP` interactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PvpConsent {
    /// Player has opted out (green).
    PeaceMode,
    /// Player has flagged themselves for `PvP` (red — one-way hostile).
    Hostile,
    /// Both players have agreed to a duel.
    Duel,
}

impl PvpConsent {
    /// Returns `true` if `self` can attack `other`.
    #[must_use]
    pub fn can_attack(self, other: PvpConsent) -> bool {
        matches!((self, other), (Self::Duel, Self::Duel) | (Self::Hostile, _))
    }
}

/// Rules governing a `PvP` session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PvpRules {
    /// Minimum player level difference allowed (0 = no restriction).
    pub level_delta_cap: u32,
    /// Cooldown in ticks after flagging before attacks are enabled.
    pub flag_cooldown_ticks: u32,
    /// Whether kill counts toward an open-world ranking.
    pub affects_ladder: bool,
}

impl PvpRules {
    #[must_use]
    pub fn default_rules() -> Self {
        Self {
            level_delta_cap: 0,
            flag_cooldown_ticks: 30,
            affects_ladder: false,
        }
    }
}

/// A pending duel challenge between two players.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuelChallenge {
    pub challenger_id: u64,
    pub target_id: u64,
    pub accepted: bool,
    pub expires_tick: u64,
}

impl DuelChallenge {
    #[must_use]
    pub fn new(challenger_id: u64, target_id: u64, current_tick: u64) -> Self {
        Self {
            challenger_id,
            target_id,
            accepted: false,
            expires_tick: current_tick + 300,
        }
    }

    pub fn accept(&mut self) {
        self.accepted = true;
    }

    #[must_use]
    pub fn is_expired(&self, current_tick: u64) -> bool {
        current_tick > self.expires_tick
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peace_cannot_be_attacked_by_peace() {
        assert!(!PvpConsent::PeaceMode.can_attack(PvpConsent::PeaceMode));
    }

    #[test]
    fn hostile_can_attack_anyone() {
        assert!(PvpConsent::Hostile.can_attack(PvpConsent::PeaceMode));
        assert!(PvpConsent::Hostile.can_attack(PvpConsent::Duel));
    }

    #[test]
    fn duel_both_must_consent() {
        assert!(PvpConsent::Duel.can_attack(PvpConsent::Duel));
        assert!(!PvpConsent::Duel.can_attack(PvpConsent::PeaceMode));
    }

    #[test]
    fn duel_challenge_expires() {
        let challenge = DuelChallenge::new(1, 2, 0);
        assert!(challenge.is_expired(301));
        assert!(!challenge.is_expired(100));
    }
}
