// @id: MGE-ARPG-Wallet @do: gold-wallet @role: back-end @layer: 3 @human: miyuk
//! Gold wallet for a single player.
//!
//! Invariant: `0 <= gold <= 2_500_000_000`.

/// Maximum gold a wallet can hold (2.5 billion).
pub const GOLD_MAX: i64 = 2_500_000_000;

/// Errors produced by [`Wallet`] operations.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum WalletError {
    /// Adding `attempted` gold would exceed [`GOLD_MAX`].
    #[error("Gold overflow: attempted to add {attempted}, max is {max}")]
    Overflow {
        /// Amount that was attempted to be added.
        attempted: i64,
        /// Maximum allowed balance.
        max: i64,
    },
    /// Removing `attempted` gold would drop below zero.
    #[error("Gold underflow: attempted to remove {attempted}, current balance is {current}")]
    Underflow {
        /// Amount that was attempted to be removed.
        attempted: i64,
        /// Current balance at the time of the operation.
        current: i64,
    },
}

/// Holds a player's gold balance.
///
/// The balance is guaranteed to remain within `[0, 2_500_000_000]` at all times.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Wallet {
    gold: i64,
}

impl Wallet {
    /// Create a new wallet starting at zero gold.
    #[must_use]
    pub fn new() -> Self {
        Self { gold: 0 }
    }

    /// Current gold balance.
    #[must_use]
    pub fn gold(&self) -> i64 {
        self.gold
    }

    /// Add `amount` gold to the wallet.
    ///
    /// # Errors
    ///
    /// Returns [`WalletError::Overflow`] if the resulting balance would exceed [`GOLD_MAX`].
    pub fn add_gold(&mut self, amount: i64) -> Result<(), WalletError> {
        if self.gold + amount > GOLD_MAX {
            return Err(WalletError::Overflow {
                attempted: amount,
                max: GOLD_MAX,
            });
        }
        self.gold += amount;
        Ok(())
    }

    /// Remove `amount` gold from the wallet.
    ///
    /// # Errors
    ///
    /// Returns [`WalletError::Underflow`] if the resulting balance would drop below zero.
    pub fn remove_gold(&mut self, amount: i64) -> Result<(), WalletError> {
        if self.gold < amount {
            return Err(WalletError::Underflow {
                attempted: amount,
                current: self.gold,
            });
        }
        self.gold -= amount;
        Ok(())
    }

    /// Returns `true` when the wallet holds at least `amount` gold.
    #[must_use]
    pub fn can_afford(&self, amount: i64) -> bool {
        self.gold >= amount
    }
}

impl Default for Wallet {
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
    fn wallet_add_gold() {
        let mut w = Wallet::new();
        w.add_gold(100).unwrap();
        assert_eq!(w.gold(), 100);
    }

    #[test]
    fn wallet_overflow() {
        let mut w = Wallet::new();
        // First fill to GOLD_MAX.
        w.add_gold(GOLD_MAX).unwrap();
        // Adding any more must fail.
        let err = w.add_gold(1).unwrap_err();
        assert_eq!(
            err,
            WalletError::Overflow {
                attempted: 1,
                max: GOLD_MAX,
            }
        );
        // Balance must be unchanged.
        assert_eq!(w.gold(), GOLD_MAX);
    }

    #[test]
    fn wallet_underflow() {
        let mut w = Wallet::new();
        w.add_gold(50).unwrap();
        let err = w.remove_gold(100).unwrap_err();
        assert_eq!(
            err,
            WalletError::Underflow {
                attempted: 100,
                current: 50,
            }
        );
        // Balance must be unchanged.
        assert_eq!(w.gold(), 50);
    }

    #[test]
    fn wallet_gold_cap() {
        // Exactly GOLD_MAX must succeed.
        let mut w = Wallet::new();
        w.add_gold(GOLD_MAX).unwrap();
        assert_eq!(w.gold(), GOLD_MAX);
    }

    // -----------------------------------------------------------------------
    // Additional coverage
    // -----------------------------------------------------------------------

    #[test]
    fn wallet_remove_gold() {
        let mut w = Wallet::new();
        w.add_gold(500).unwrap();
        w.remove_gold(200).unwrap();
        assert_eq!(w.gold(), 300);
    }

    #[test]
    fn wallet_can_afford_true() {
        let mut w = Wallet::new();
        w.add_gold(1000).unwrap();
        assert!(w.can_afford(500));
    }

    #[test]
    fn wallet_can_afford_false() {
        let mut w = Wallet::new();
        w.add_gold(100).unwrap();
        assert!(!w.can_afford(200));
    }

    #[test]
    fn wallet_default_starts_at_zero() {
        let w = Wallet::default();
        assert_eq!(w.gold(), 0);
    }

    #[test]
    fn wallet_overflow_from_zero() {
        // Adding more than GOLD_MAX in a single call from zero.
        let mut w = Wallet::new();
        let err = w.add_gold(GOLD_MAX + 1).unwrap_err();
        assert_eq!(
            err,
            WalletError::Overflow {
                attempted: GOLD_MAX + 1,
                max: GOLD_MAX,
            }
        );
        assert_eq!(w.gold(), 0);
    }

    #[test]
    fn wallet_underflow_from_empty() {
        let mut w = Wallet::new();
        let err = w.remove_gold(1).unwrap_err();
        assert_eq!(
            err,
            WalletError::Underflow {
                attempted: 1,
                current: 0,
            }
        );
    }
}
