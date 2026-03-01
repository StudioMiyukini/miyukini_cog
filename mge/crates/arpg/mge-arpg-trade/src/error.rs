// @id: MGE-ARPG-Trade-Error @do: trade-error @role: back-end @layer: 3 @human: miyuk
//! Trade system error types.

/// All errors that the trade subsystem can produce.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TradeError {
    /// The wallet does not hold enough gold for the operation.
    #[error("Insufficient gold: have {have}, need {need}")]
    InsufficientGold {
        /// Current balance.
        have: u64,
        /// Amount required.
        need: u64,
    },

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
