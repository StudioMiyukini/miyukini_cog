// @id: MGE-ARPG-Trade-Wallet @do: wallet @role: back-end @layer: 3 @human: miyuk
//! Gold wallet for a single player.

use crate::error::TradeError;

/// Holds a player's gold balance.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Wallet {
    gold: u64,
}

impl Wallet {
    /// Create a new wallet with a starting balance.
    pub fn new(starting_gold: u64) -> Self {
        Self { gold: starting_gold }
    }

    /// Current gold balance.
    pub fn gold(&self) -> u64 {
        self.gold
    }

    /// Add gold to the wallet (no overflow -- saturating add).
    pub fn deposit(&mut self, amount: u64) {
        self.gold = self.gold.saturating_add(amount);
    }

    /// Remove gold. Returns [`TradeError::InsufficientGold`] if the balance is too low.
    pub fn withdraw(&mut self, amount: u64) -> Result<(), TradeError> {
        if self.gold < amount {
            return Err(TradeError::InsufficientGold {
                have: self.gold,
                need: amount,
            });
        }
        self.gold -= amount;
        Ok(())
    }

    /// Returns `true` when the wallet has at least `amount` gold.
    pub fn can_afford(&self, amount: u64) -> bool {
        self.gold >= amount
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new(0)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_new() {
        let w = Wallet::new(1000);
        assert_eq!(w.gold(), 1000);
    }

    #[test]
    fn test_wallet_deposit() {
        let mut w = Wallet::new(1000);
        w.deposit(500);
        assert_eq!(w.gold(), 1500);
    }

    #[test]
    fn test_wallet_withdraw_success() {
        let mut w = Wallet::new(1000);
        let result = w.withdraw(300);
        assert!(result.is_ok());
        assert_eq!(w.gold(), 700);
    }

    #[test]
    fn test_wallet_withdraw_insufficient() {
        let mut w = Wallet::new(100);
        let result = w.withdraw(200);
        assert_eq!(
            result,
            Err(TradeError::InsufficientGold {
                have: 100,
                need: 200,
            })
        );
        // Balance unchanged after failure.
        assert_eq!(w.gold(), 100);
    }

    #[test]
    fn test_wallet_can_afford_true() {
        let w = Wallet::new(1000);
        assert!(w.can_afford(500));
    }

    #[test]
    fn test_wallet_can_afford_false() {
        let w = Wallet::new(1000);
        assert!(!w.can_afford(2000));
    }

    #[test]
    fn test_wallet_deposit_saturating() {
        let mut w = Wallet::new(u64::MAX);
        w.deposit(1);
        assert_eq!(w.gold(), u64::MAX);
    }
}
