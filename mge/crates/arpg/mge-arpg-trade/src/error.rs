// @id: MGE-ARPG-Trade-Error @do: trade-error @role: back-end @layer: 3 @human: miyuk
//! Trade system error types.

use crate::trade_session::TradeState;

/// All errors that the trade subsystem can produce.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TradeError {
    // -----------------------------------------------------------------------
    // Wallet / gold errors
    // -----------------------------------------------------------------------

    /// The wallet does not hold enough gold for the operation.
    ///
    /// Used by both the NPC vendor and the atomic trade session.
    #[error("Insufficient gold: have {have}, need {need}")]
    InsufficientGold {
        /// Current balance.
        have: i64,
        /// Amount required.
        need: i64,
    },

    // -----------------------------------------------------------------------
    // Atomic trade session errors
    // -----------------------------------------------------------------------

    /// A player offered an item they do not own.
    #[error("Player '{player_id}' does not own item '{item_id}'")]
    ItemNotOwned {
        /// The player who made the invalid offer.
        player_id: String,
        /// The item that is not in their inventory.
        item_id: String,
    },

    /// The session was not in the expected state for the attempted operation.
    #[error("Trade not in correct state: expected {expected:?}, got {actual:?}")]
    InvalidState {
        /// State that was required.
        expected: TradeState,
        /// State that was actually observed.
        actual: TradeState,
    },

    /// The trade was rolled back; the `reason` field carries the original error message.
    #[error("Trade rolled back: {reason}")]
    RolledBack {
        /// Human-readable description of why the trade was rolled back.
        reason: String,
    },

    // -----------------------------------------------------------------------
    // Legacy P2P trade session errors (state machine)
    // -----------------------------------------------------------------------

    /// Attempted to modify a trade that has already been confirmed.
    #[error("Trade session is already confirmed")]
    TradeAlreadyConfirmed,

    /// The trade session is not in the `Open` state.
    #[error("Trade is not in Open state")]
    TradeNotOpen,

    /// The given player identifier is not a participant of the trade.
    #[error("Player '{id}' is not part of this trade")]
    PlayerNotInTrade {
        /// The offending player identifier.
        id: String,
    },

    // -----------------------------------------------------------------------
    // NPC vendor errors
    // -----------------------------------------------------------------------

    /// The requested item has zero remaining stock at the vendor.
    #[error("Item '{item_id}' is out of stock")]
    OutOfStock {
        /// Identifier of the out-of-stock item.
        item_id: String,
    },

    /// The item is not present in the party's offer list.
    #[error("Item '{item_id}' is not in the offer")]
    ItemNotOffered {
        /// Identifier of the missing item.
        item_id: String,
    },

    /// The item was not found in the vendor's catalogue.
    #[error("Item '{item_id}' not found in vendor stock")]
    ItemNotFound {
        /// Identifier of the missing item.
        item_id: String,
    },
}
