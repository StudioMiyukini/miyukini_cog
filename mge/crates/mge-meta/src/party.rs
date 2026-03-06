use serde::{Deserialize, Serialize};

/// Maximum party size.
pub const MAX_PARTY_SIZE: usize = 8;

/// XP share mode for the party.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum XpShareMode {
    /// Each member gets full XP if in range.
    Individual,
    /// XP is split equally among nearby members.
    Split,
    /// Party leader controls distribution.
    LeaderControlled,
}

/// Member within a party.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartyMember {
    pub player_id: u64,
    pub name: String,
    pub zone_id: u32,
    pub is_leader: bool,
}

/// A live party definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartyDef {
    pub party_id: u64,
    pub members: Vec<PartyMember>,
    pub xp_share: XpShareMode,
}

impl PartyDef {
    #[must_use]
    pub fn new(party_id: u64, leader: PartyMember) -> Self {
        Self {
            party_id,
            members: vec![leader],
            xp_share: XpShareMode::Individual,
        }
    }

    /// Add a member. Returns `false` if the party is full.
    pub fn join(&mut self, member: PartyMember) -> bool {
        if self.members.len() >= MAX_PARTY_SIZE {
            return false;
        }
        self.members.push(member);
        true
    }

    /// Remove a member by `player_id`. Returns `false` if not found.
    pub fn leave(&mut self, player_id: u64) -> bool {
        let before = self.members.len();
        self.members.retain(|m| m.player_id != player_id);
        self.members.len() < before
    }

    /// Members in the same zone as `zone_id`.
    #[must_use]
    pub fn members_in_zone(&self, zone_id: u32) -> Vec<&PartyMember> {
        self.members.iter().filter(|m| m.zone_id == zone_id).collect()
    }

    /// Compute XP each nearby member gets from a kill of `base_xp`.
    #[must_use]
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    pub fn distributed_xp(&self, base_xp: u32, zone_id: u32) -> u32 {
        let nearby = self.members_in_zone(zone_id).len();
        if nearby == 0 {
            return base_xp;
        }
        match self.xp_share {
            XpShareMode::Individual => base_xp,
            XpShareMode::Split | XpShareMode::LeaderControlled => {
                base_xp / (nearby as u32).max(1)
            }
        }
    }

    #[must_use]
    pub fn is_full(&self) -> bool {
        self.members.len() >= MAX_PARTY_SIZE
    }

    #[must_use]
    pub fn leader(&self) -> Option<&PartyMember> {
        self.members.iter().find(|m| m.is_leader)
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_leader(id: u64) -> PartyMember {
        PartyMember { player_id: id, name: format!("Player{id}"), zone_id: 10, is_leader: true }
    }
    fn make_member(id: u64) -> PartyMember {
        PartyMember { player_id: id, name: format!("Player{id}"), zone_id: 10, is_leader: false }
    }

    #[test]
    fn join_and_leave() {
        let mut party = PartyDef::new(1, make_leader(1));
        assert!(party.join(make_member(2)));
        assert_eq!(party.members.len(), 2);
        assert!(party.leave(2));
        assert_eq!(party.members.len(), 1);
    }

    #[test]
    fn party_full_rejects_join() {
        let mut party = PartyDef::new(1, make_leader(1));
        for i in 2..=8 {
            party.join(make_member(i));
        }
        assert!(party.is_full());
        assert!(!party.join(make_member(9)));
    }

    #[test]
    fn split_xp_divides_equally() {
        let mut party = PartyDef::new(1, make_leader(1));
        party.join(make_member(2));
        party.xp_share = XpShareMode::Split;
        assert_eq!(party.distributed_xp(100, 10), 50);
    }

    #[test]
    fn individual_xp_full_amount() {
        let mut party = PartyDef::new(1, make_leader(1));
        party.join(make_member(2));
        assert_eq!(party.distributed_xp(100, 10), 100);
    }
}
