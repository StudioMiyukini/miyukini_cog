use serde::{Deserialize, Serialize};

/// Item rarity tier (D2-like).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Rarity {
    Normal,
    Magic,
    Rare,
    Crafted,
    Set,
    Unique,
}

impl Rarity {
    /// UI display colour name for this rarity.
    pub fn color_name(self) -> &'static str {
        match self {
            Self::Normal => "white",
            Self::Magic => "blue",
            Self::Rare => "yellow",
            Self::Crafted => "orange",
            Self::Set => "green",
            Self::Unique => "gold",
        }
    }

    /// Maximum number of random affixes this rarity may carry.
    pub fn max_affixes(self) -> u8 {
        match self {
            Self::Magic => 2,                             // up to 1 prefix + 1 suffix
            Self::Rare => 6,                              // up to 3 prefix + 3 suffix
            Self::Crafted => 4,                           // 2 forced + 2 random
            Self::Normal | Self::Set | Self::Unique => 0, // pre-defined mods only
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_outranks_magic() {
        assert!(Rarity::Unique > Rarity::Magic);
    }

    #[test]
    fn rare_allows_six_affixes() {
        assert_eq!(Rarity::Rare.max_affixes(), 6);
    }

    #[test]
    fn unique_color_is_gold() {
        assert_eq!(Rarity::Unique.color_name(), "gold");
    }

    #[test]
    fn normal_has_zero_affixes() {
        assert_eq!(Rarity::Normal.max_affixes(), 0);
    }
}
