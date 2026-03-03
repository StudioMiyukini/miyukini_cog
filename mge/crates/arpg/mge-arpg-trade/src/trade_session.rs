// @id: MGE-ARPG-Trade @do: trade-session @role: back-end @layer: 3 @human: miyuk
//! Atomic P2P trade session with BEGIN / COMMIT / ROLLBACK semantics.
//!
//! A [`TradeSession`] progresses through the following states:
//! ```text
//! Negotiating ──confirm()──► Confirmed ──execute()──► Completed
//!      │                         │
//!      └──cancel()───────────────┘──► Cancelled
//! ```
//! The `execute` method is fully atomic: if any validation fails, both
//! inventories are restored from a snapshot taken at BEGIN.

use std::collections::HashSet;

use crate::error::TradeError;

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

/// Lifecycle state of a [`TradeSession`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TradeState {
    /// Items and gold are still being negotiated.
    Negotiating,
    /// Both offers are locked; ready for execution.
    Confirmed,
    /// Trade executed and inventories exchanged.
    Completed,
    /// Trade was cancelled before execution.
    Cancelled,
}

// ---------------------------------------------------------------------------
// Offer
// ---------------------------------------------------------------------------

/// Items and gold one player brings to the table.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TradeOffer {
    /// The offering player's identifier.
    pub player_id: String,
    /// Instance IDs of items this player offers.
    pub items: Vec<String>,
    /// Gold this player offers.
    pub gold: i64,
}

impl TradeOffer {
    /// Create a new offer with no items and zero gold.
    #[must_use]
    pub fn new(player_id: impl Into<String>) -> Self {
        Self {
            player_id: player_id.into(),
            items: Vec::new(),
            gold: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// Inventory (minimal simulation for trade logic)
// ---------------------------------------------------------------------------

/// Minimal in-memory player inventory used to validate and execute trades.
///
/// This is intentionally lightweight — real persistence lives in the DB layer.
#[derive(Debug, Clone)]
pub struct PlayerInventory {
    /// The owner's player identifier.
    pub player_id: String,
    /// Set of item instance IDs held by the player.
    pub items: HashSet<String>,
    /// Current gold balance.
    pub gold: i64,
}

impl PlayerInventory {
    /// Create a new empty inventory for `player_id`.
    #[must_use]
    pub fn new(player_id: impl Into<String>) -> Self {
        Self {
            player_id: player_id.into(),
            items: HashSet::new(),
            gold: 0,
        }
    }

    /// Add an item to the inventory.
    pub fn add_item(&mut self, item_id: impl Into<String>) {
        self.items.insert(item_id.into());
    }

    /// Remove an item from the inventory. Returns `true` if the item was present.
    pub fn remove_item(&mut self, item_id: &str) -> bool {
        self.items.remove(item_id)
    }

    /// Returns `true` when the player owns the given item.
    #[must_use]
    pub fn owns(&self, item_id: &str) -> bool {
        self.items.contains(item_id)
    }
}

// ---------------------------------------------------------------------------
// Session snapshot (for atomic rollback)
// ---------------------------------------------------------------------------

/// A lightweight snapshot of both inventories taken at BEGIN.
#[derive(Debug, Clone)]
struct InventorySnapshot {
    items_a: HashSet<String>,
    gold_a: i64,
    items_b: HashSet<String>,
    gold_b: i64,
}

impl InventorySnapshot {
    fn capture(inv_a: &PlayerInventory, inv_b: &PlayerInventory) -> Self {
        Self {
            items_a: inv_a.items.clone(),
            gold_a: inv_a.gold,
            items_b: inv_b.items.clone(),
            gold_b: inv_b.gold,
        }
    }

    fn restore(self, inv_a: &mut PlayerInventory, inv_b: &mut PlayerInventory) {
        inv_a.items = self.items_a;
        inv_a.gold = self.gold_a;
        inv_b.items = self.items_b;
        inv_b.gold = self.gold_b;
    }
}

// ---------------------------------------------------------------------------
// Session
// ---------------------------------------------------------------------------

/// An atomic P2P trade session between two players.
///
/// Call order:
/// 1. [`TradeSession::new`] — create with both offers.
/// 2. [`TradeSession::confirm`] — lock in the offers.
/// 3. [`TradeSession::execute`] — atomically swap items and gold.
#[derive(Debug, Clone)]
pub struct TradeSession {
    /// Offer from player A (initiator).
    pub player_a: TradeOffer,
    /// Offer from player B (counterpart).
    pub player_b: TradeOffer,
    /// Current lifecycle state.
    pub state: TradeState,
}

impl TradeSession {
    /// Create a new session in [`TradeState::Negotiating`].
    #[must_use]
    pub fn new(player_a: TradeOffer, player_b: TradeOffer) -> Self {
        Self {
            player_a,
            player_b,
            state: TradeState::Negotiating,
        }
    }

    /// Lock in the offers. Transitions from `Negotiating` → `Confirmed`.
    ///
    /// # Errors
    ///
    /// Returns [`TradeError::InvalidState`] when called from any state other
    /// than `Negotiating`.
    pub fn confirm(&mut self) -> Result<(), TradeError> {
        if self.state != TradeState::Negotiating {
            return Err(TradeError::InvalidState {
                expected: TradeState::Negotiating,
                actual: self.state,
            });
        }
        self.state = TradeState::Confirmed;
        Ok(())
    }

    /// Atomically execute the trade.
    ///
    /// # BEGIN / COMMIT / ROLLBACK contract
    ///
    /// 1. **BEGIN** — snapshot both inventories.
    /// 2. **Validate** — all offered items must be owned; gold balances sufficient.
    /// 3. **Transfer** — swap items and gold between the two inventories.
    /// 4. **COMMIT** — set state to `Completed`.
    ///
    /// If *any* step fails the snapshot is restored (**ROLLBACK**) before the
    /// error is returned, leaving both inventories completely unchanged.
    ///
    /// # Errors
    ///
    /// - [`TradeError::InvalidState`] — session not in `Confirmed` state.
    /// - [`TradeError::ItemNotOwned`] — a player offered an item they don't own.
    /// - [`TradeError::InsufficientGold`] — a player offered more gold than they hold.
    /// - [`TradeError::RolledBack`] — wraps any internal error after rollback.
    pub fn execute(
        &mut self,
        inv_a: &mut PlayerInventory,
        inv_b: &mut PlayerInventory,
    ) -> Result<(), TradeError> {
        // Guard: must be Confirmed.
        if self.state != TradeState::Confirmed {
            return Err(TradeError::InvalidState {
                expected: TradeState::Confirmed,
                actual: self.state,
            });
        }

        // BEGIN — snapshot.
        let snapshot = InventorySnapshot::capture(inv_a, inv_b);

        // Run the transfer; on failure roll back.
        match Self::do_transfer(&self.player_a, &self.player_b, inv_a, inv_b) {
            Ok(()) => {
                self.state = TradeState::Completed;
                Ok(())
            }
            Err(err) => {
                // ROLLBACK — restore both inventories from snapshot.
                snapshot.restore(inv_a, inv_b);
                Err(TradeError::RolledBack {
                    reason: err.to_string(),
                })
            }
        }
    }

    /// Cancel the trade unconditionally.
    ///
    /// May be called from `Negotiating` or `Confirmed`. Has no effect on an
    /// already-terminal session (`Completed` / `Cancelled`).
    pub fn cancel(&mut self) {
        if matches!(
            self.state,
            TradeState::Negotiating | TradeState::Confirmed
        ) {
            self.state = TradeState::Cancelled;
        }
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    /// Perform validation + transfer. Does **not** touch `self.state`.
    ///
    /// The caller is responsible for snapshot / rollback.
    fn do_transfer(
        offer_a: &TradeOffer,
        offer_b: &TradeOffer,
        inv_a: &mut PlayerInventory,
        inv_b: &mut PlayerInventory,
    ) -> Result<(), TradeError> {
        // Validate A's items.
        for item_id in &offer_a.items {
            if !inv_a.owns(item_id) {
                return Err(TradeError::ItemNotOwned {
                    player_id: offer_a.player_id.clone(),
                    item_id: item_id.clone(),
                });
            }
        }

        // Validate B's items.
        for item_id in &offer_b.items {
            if !inv_b.owns(item_id) {
                return Err(TradeError::ItemNotOwned {
                    player_id: offer_b.player_id.clone(),
                    item_id: item_id.clone(),
                });
            }
        }

        // Validate A's gold.
        if inv_a.gold < offer_a.gold {
            return Err(TradeError::InsufficientGold {
                have: inv_a.gold,
                need: offer_a.gold,
            });
        }

        // Validate B's gold.
        if inv_b.gold < offer_b.gold {
            return Err(TradeError::InsufficientGold {
                have: inv_b.gold,
                need: offer_b.gold,
            });
        }

        // Transfer items A → B.
        for item_id in &offer_a.items {
            inv_a.items.remove(item_id);
            inv_b.items.insert(item_id.clone());
        }

        // Transfer items B → A.
        for item_id in &offer_b.items {
            inv_b.items.remove(item_id);
            inv_a.items.insert(item_id.clone());
        }

        // Transfer gold.
        inv_a.gold -= offer_a.gold;
        inv_b.gold += offer_a.gold;
        inv_b.gold -= offer_b.gold;
        inv_a.gold += offer_b.gold;

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn make_offer(player_id: &str, items: &[&str], gold: i64) -> TradeOffer {
        TradeOffer {
            player_id: player_id.to_string(),
            items: items.iter().map(|s| s.to_string()).collect(),
            gold,
        }
    }

    fn make_inventory(player_id: &str, items: &[&str], gold: i64) -> PlayerInventory {
        let mut inv = PlayerInventory::new(player_id);
        for item in items {
            inv.add_item(*item);
        }
        inv.gold = gold;
        inv
    }

    fn confirmed_session(
        offer_a: TradeOffer,
        offer_b: TradeOffer,
    ) -> TradeSession {
        let mut session = TradeSession::new(offer_a, offer_b);
        session.confirm().expect("confirm should succeed from Negotiating");
        session
    }

    // -----------------------------------------------------------------------
    // Test 1 — happy path: items and gold swap atomically
    // -----------------------------------------------------------------------

    #[test]
    fn trade_atomic_success() {
        let offer_a = make_offer("alice", &["sword-01"], 100);
        let offer_b = make_offer("bob", &["shield-02"], 50);

        let mut inv_a = make_inventory("alice", &["sword-01"], 500);
        let mut inv_b = make_inventory("bob", &["shield-02"], 300);

        let mut session = confirmed_session(offer_a, offer_b);
        session.execute(&mut inv_a, &mut inv_b).expect("trade should succeed");

        // State.
        assert_eq!(session.state, TradeState::Completed);

        // Item swap.
        assert!(inv_a.owns("shield-02"), "alice should now own shield-02");
        assert!(!inv_a.owns("sword-01"), "alice should no longer own sword-01");
        assert!(inv_b.owns("sword-01"), "bob should now own sword-01");
        assert!(!inv_b.owns("shield-02"), "bob should no longer own shield-02");

        // Gold swap: alice paid 100, received 50 → 450. Bob paid 50, received 100 → 350.
        assert_eq!(inv_a.gold, 450);
        assert_eq!(inv_b.gold, 350);
    }

    // -----------------------------------------------------------------------
    // Test 2 — rollback: player A offers item they don't own
    // -----------------------------------------------------------------------

    #[test]
    fn trade_atomic_rollback() {
        let offer_a = make_offer("alice", &["ghost-item"], 0);
        let offer_b = make_offer("bob", &["shield-02"], 0);

        let mut inv_a = make_inventory("alice", &[], 200); // alice does NOT own ghost-item
        let mut inv_b = make_inventory("bob", &["shield-02"], 200);

        let snapshot_a_gold = inv_a.gold;
        let snapshot_b_gold = inv_b.gold;
        let snapshot_b_items: HashSet<String> = inv_b.items.clone();

        let mut session = confirmed_session(offer_a, offer_b);
        let err = session
            .execute(&mut inv_a, &mut inv_b)
            .expect_err("should fail because alice doesn't own ghost-item");

        // Error must be RolledBack.
        assert!(
            matches!(err, TradeError::RolledBack { .. }),
            "expected RolledBack, got {err:?}"
        );

        // Both inventories must be UNCHANGED after rollback.
        assert_eq!(inv_a.gold, snapshot_a_gold, "alice's gold must be unchanged");
        assert!(inv_a.items.is_empty(), "alice's items must be unchanged");
        assert_eq!(inv_b.gold, snapshot_b_gold, "bob's gold must be unchanged");
        assert_eq!(inv_b.items, snapshot_b_items, "bob's items must be unchanged");
    }

    // -----------------------------------------------------------------------
    // Test 3 — ItemNotOwned error surfaces inside RolledBack
    // -----------------------------------------------------------------------

    #[test]
    fn trade_wrong_owner() {
        let offer_a = make_offer("alice", &["item-x"], 0);
        let offer_b = make_offer("bob", &[], 0);

        let mut inv_a = make_inventory("alice", &[], 0); // does not own item-x
        let mut inv_b = make_inventory("bob", &[], 0);

        let mut session = confirmed_session(offer_a, offer_b);
        let err = session
            .execute(&mut inv_a, &mut inv_b)
            .expect_err("must fail: alice doesn't own item-x");

        // The rolled-back reason must mention ItemNotOwned semantics.
        match err {
            TradeError::RolledBack { reason } => {
                assert!(
                    reason.contains("alice") || reason.contains("item-x"),
                    "reason should reference the offending player/item, got: {reason}"
                );
            }
            other => panic!("expected RolledBack, got {other:?}"),
        }
    }

    // -----------------------------------------------------------------------
    // Test 4 — InsufficientGold error
    // -----------------------------------------------------------------------

    #[test]
    fn trade_insufficient_gold() {
        let offer_a = make_offer("alice", &[], 1_000); // alice offers 1000 gold
        let offer_b = make_offer("bob", &[], 0);

        let mut inv_a = make_inventory("alice", &[], 10); // alice only has 10
        let mut inv_b = make_inventory("bob", &[], 500);

        let mut session = confirmed_session(offer_a, offer_b);
        let err = session
            .execute(&mut inv_a, &mut inv_b)
            .expect_err("must fail: alice can't afford 1000 gold");

        // RolledBack wrapping InsufficientGold.
        assert!(
            matches!(err, TradeError::RolledBack { .. }),
            "expected RolledBack, got {err:?}"
        );

        // Inventories must be intact.
        assert_eq!(inv_a.gold, 10, "alice's gold must be unchanged");
        assert_eq!(inv_b.gold, 500, "bob's gold must be unchanged");
    }

    // -----------------------------------------------------------------------
    // Test 5 — cancel sets state to Cancelled
    // -----------------------------------------------------------------------

    #[test]
    fn trade_cancel() {
        let offer_a = make_offer("alice", &[], 0);
        let offer_b = make_offer("bob", &[], 0);
        let mut session = TradeSession::new(offer_a, offer_b);

        assert_eq!(session.state, TradeState::Negotiating);
        session.cancel();
        assert_eq!(session.state, TradeState::Cancelled);
    }

    // -----------------------------------------------------------------------
    // Test 6 — execute from Negotiating → InvalidState
    // -----------------------------------------------------------------------

    #[test]
    fn trade_state_check() {
        let offer_a = make_offer("alice", &[], 0);
        let offer_b = make_offer("bob", &[], 0);
        let mut session = TradeSession::new(offer_a, offer_b);
        // Do NOT call confirm() — state is still Negotiating.

        let mut inv_a = make_inventory("alice", &[], 0);
        let mut inv_b = make_inventory("bob", &[], 0);

        let err = session
            .execute(&mut inv_a, &mut inv_b)
            .expect_err("must fail: session not Confirmed");

        assert_eq!(
            err,
            TradeError::InvalidState {
                expected: TradeState::Confirmed,
                actual: TradeState::Negotiating,
            }
        );
    }

    // -----------------------------------------------------------------------
    // Additional: confirm from non-Negotiating → InvalidState
    // -----------------------------------------------------------------------

    #[test]
    fn trade_confirm_from_confirmed_is_error() {
        let offer_a = make_offer("alice", &[], 0);
        let offer_b = make_offer("bob", &[], 0);
        let mut session = TradeSession::new(offer_a, offer_b);
        session.confirm().expect("first confirm OK");

        let err = session.confirm().expect_err("second confirm must fail");
        assert_eq!(
            err,
            TradeError::InvalidState {
                expected: TradeState::Negotiating,
                actual: TradeState::Confirmed,
            }
        );
    }

    // -----------------------------------------------------------------------
    // Additional: cancel from Confirmed is allowed
    // -----------------------------------------------------------------------

    #[test]
    fn trade_cancel_from_confirmed() {
        let offer_a = make_offer("alice", &[], 0);
        let offer_b = make_offer("bob", &[], 0);
        let mut session = confirmed_session(offer_a, offer_b);

        session.cancel();
        assert_eq!(session.state, TradeState::Cancelled);
    }

    // -----------------------------------------------------------------------
    // Additional: cancel after Completed is a no-op
    // -----------------------------------------------------------------------

    #[test]
    fn trade_cancel_after_completed_is_noop() {
        let offer_a = make_offer("alice", &[], 0);
        let offer_b = make_offer("bob", &[], 0);
        let mut inv_a = make_inventory("alice", &[], 0);
        let mut inv_b = make_inventory("bob", &[], 0);

        let mut session = confirmed_session(offer_a, offer_b);
        session.execute(&mut inv_a, &mut inv_b).expect("execute OK");
        assert_eq!(session.state, TradeState::Completed);

        session.cancel(); // must be a no-op
        assert_eq!(
            session.state,
            TradeState::Completed,
            "cancel after Completed must not change state"
        );
    }
}
