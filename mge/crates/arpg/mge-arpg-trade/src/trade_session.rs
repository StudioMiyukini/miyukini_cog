// @id: MGE-ARPG-Trade-Session @do: p2p-trade-session @role: back-end @layer: 3 @human: miyuk
//! P2P trade session state machine.

use crate::error::TradeError;

/// State of a P2P trade session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TradeState {
    /// Session is open; items and gold can be modified.
    Open,
    /// Both parties have confirmed; awaiting execution.
    BothConfirmed,
    /// Trade was executed successfully.
    Completed,
    /// Trade was cancelled by a party.
    Cancelled,
}

/// One side of a P2P trade.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TradeParty {
    /// Player identifier.
    pub player_id: String,
    /// Item instance IDs offered by this party.
    pub offered_items: Vec<String>,
    /// Gold offered by this party.
    pub offered_gold: u64,
    /// Whether this party has confirmed the trade.
    pub confirmed: bool,
}

impl TradeParty {
    fn new(player_id: impl Into<String>) -> Self {
        Self {
            player_id: player_id.into(),
            offered_items: Vec::new(),
            offered_gold: 0,
            confirmed: false,
        }
    }
}

/// A P2P trade session between two players.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TradeSession {
    /// Unique session identifier.
    pub id: String,
    /// The player who initiated the trade.
    pub initiator: TradeParty,
    /// The player who received the trade request.
    pub receiver: TradeParty,
    /// Current state of the session.
    pub state: TradeState,
}

impl TradeSession {
    /// Create a new trade session in `Open` state.
    pub fn new(
        id: impl Into<String>,
        initiator_id: impl Into<String>,
        receiver_id: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            initiator: TradeParty::new(initiator_id),
            receiver: TradeParty::new(receiver_id),
            state: TradeState::Open,
        }
    }

    // ------------------------------------------------------------------
    // Internal helpers
    // ------------------------------------------------------------------

    fn party_mut(&mut self, player_id: &str) -> Result<&mut TradeParty, TradeError> {
        if self.initiator.player_id == player_id {
            Ok(&mut self.initiator)
        } else if self.receiver.player_id == player_id {
            Ok(&mut self.receiver)
        } else {
            Err(TradeError::PlayerNotInTrade {
                id: player_id.to_string(),
            })
        }
    }

    fn ensure_open(&self) -> Result<(), TradeError> {
        if self.state == TradeState::Open {
            Ok(())
        } else {
            Err(TradeError::TradeNotOpen)
        }
    }

    // ------------------------------------------------------------------
    // Public API
    // ------------------------------------------------------------------

    /// Add an item to a party's offer. Resets both confirmations.
    pub fn offer_item(&mut self, player_id: &str, item_id: String) -> Result<(), TradeError> {
        self.ensure_open()?;
        let party = self.party_mut(player_id)?;
        party.offered_items.push(item_id);
        party.confirmed = false;
        // Reset the other party's confirmation as well.
        self.reset_other_confirmation(player_id);
        Ok(())
    }

    /// Remove an item from a party's offer. Resets both confirmations.
    pub fn revoke_item(&mut self, player_id: &str, item_id: &str) -> Result<(), TradeError> {
        self.ensure_open()?;
        let party = self.party_mut(player_id)?;
        let pos = party
            .offered_items
            .iter()
            .position(|i| i == item_id)
            .ok_or_else(|| TradeError::ItemNotOffered {
                item_id: item_id.to_string(),
            })?;
        party.offered_items.remove(pos);
        party.confirmed = false;
        self.reset_other_confirmation(player_id);
        Ok(())
    }

    /// Set the gold offer for a party. Resets both confirmations.
    pub fn offer_gold(&mut self, player_id: &str, amount: u64) -> Result<(), TradeError> {
        self.ensure_open()?;
        let party = self.party_mut(player_id)?;
        party.offered_gold = amount;
        party.confirmed = false;
        self.reset_other_confirmation(player_id);
        Ok(())
    }

    /// Confirm the trade for a party. If both confirm, moves to `BothConfirmed`.
    pub fn confirm(&mut self, player_id: &str) -> Result<(), TradeError> {
        self.ensure_open()?;
        self.party_mut(player_id)?.confirmed = true;
        if self.initiator.confirmed && self.receiver.confirmed {
            self.state = TradeState::BothConfirmed;
        }
        Ok(())
    }

    /// Execute the trade (call after `BothConfirmed`).
    pub fn execute(&mut self) -> Result<(), TradeError> {
        if self.state != TradeState::BothConfirmed {
            return Err(TradeError::TradeNotOpen);
        }
        self.state = TradeState::Completed;
        Ok(())
    }

    /// Cancel the trade. The cancelling player must be a participant.
    pub fn cancel(&mut self, player_id: &str) -> Result<(), TradeError> {
        if self.state == TradeState::Completed || self.state == TradeState::Cancelled {
            return Err(TradeError::TradeNotOpen);
        }
        // Validate the player belongs to this session.
        if self.initiator.player_id != player_id && self.receiver.player_id != player_id {
            return Err(TradeError::PlayerNotInTrade {
                id: player_id.to_string(),
            });
        }
        self.state = TradeState::Cancelled;
        Ok(())
    }

    /// Returns `true` when the trade was successfully completed.
    pub fn is_complete(&self) -> bool {
        self.state == TradeState::Completed
    }

    /// Returns `true` when the trade was cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.state == TradeState::Cancelled
    }

    // ------------------------------------------------------------------
    // Private
    // ------------------------------------------------------------------

    /// Resets the confirmation of the party that is *not* `player_id`.
    fn reset_other_confirmation(&mut self, player_id: &str) {
        if self.initiator.player_id == player_id {
            self.receiver.confirmed = false;
        } else {
            self.initiator.confirmed = false;
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_session() -> TradeSession {
        TradeSession::new("session-1", "alice", "bob")
    }

    #[test]
    fn test_trade_session_new() {
        let ts = make_session();
        assert_eq!(ts.state, TradeState::Open);
        assert!(ts.initiator.offered_items.is_empty());
        assert!(ts.receiver.offered_items.is_empty());
        assert_eq!(ts.initiator.offered_gold, 0);
        assert_eq!(ts.receiver.offered_gold, 0);
    }

    #[test]
    fn test_trade_offer_item() {
        let mut ts = make_session();
        ts.offer_item("alice", "sword-01".to_string()).unwrap();
        assert_eq!(ts.initiator.offered_items, vec!["sword-01"]);
    }

    #[test]
    fn test_trade_revoke_item() {
        let mut ts = make_session();
        ts.offer_item("alice", "sword-01".to_string()).unwrap();
        ts.revoke_item("alice", "sword-01").unwrap();
        assert!(ts.initiator.offered_items.is_empty());
    }

    #[test]
    fn test_trade_revoke_not_offered() {
        let mut ts = make_session();
        let err = ts.revoke_item("alice", "nope").unwrap_err();
        assert_eq!(
            err,
            TradeError::ItemNotOffered {
                item_id: "nope".to_string(),
            }
        );
    }

    #[test]
    fn test_trade_confirm_both() {
        let mut ts = make_session();
        ts.confirm("alice").unwrap();
        ts.confirm("bob").unwrap();
        assert_eq!(ts.state, TradeState::BothConfirmed);
    }

    #[test]
    fn test_trade_confirm_one() {
        let mut ts = make_session();
        ts.confirm("alice").unwrap();
        // Only one side confirmed -- state stays Open.
        assert_eq!(ts.state, TradeState::Open);
        assert!(ts.initiator.confirmed);
        assert!(!ts.receiver.confirmed);
    }

    #[test]
    fn test_trade_cancel() {
        let mut ts = make_session();
        ts.cancel("alice").unwrap();
        assert_eq!(ts.state, TradeState::Cancelled);
        assert!(ts.is_cancelled());
    }

    #[test]
    fn test_trade_player_not_in_trade() {
        let mut ts = make_session();
        let err = ts.offer_item("charlie", "item-x".to_string()).unwrap_err();
        assert_eq!(
            err,
            TradeError::PlayerNotInTrade {
                id: "charlie".to_string(),
            }
        );
    }

    #[test]
    fn test_trade_execute() {
        let mut ts = make_session();
        ts.confirm("alice").unwrap();
        ts.confirm("bob").unwrap();
        assert_eq!(ts.state, TradeState::BothConfirmed);
        ts.execute().unwrap();
        assert!(ts.is_complete());
        assert_eq!(ts.state, TradeState::Completed);
    }
}
