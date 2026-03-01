// @id: MGE-ARPG-Stats-StatValue @do: stat-value @role: back-end @layer: 3 @human: miyuk
//! Single stat value with base, flat additions, and multiplicative scaling.

/// A single stat value supporting additive and multiplicative modifiers.
///
/// The effective value is computed as `floor((base + added) * multiplier)`.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct StatValue {
    /// Innate base value (e.g. from character creation or class).
    pub base: i32,
    /// Flat additions from gear, buffs, stat point allocation.
    pub added: i32,
    /// Multiplicative factor (1.0 = no change, 1.2 = +20%).
    pub multiplier: f32,
}

impl Default for StatValue {
    fn default() -> Self {
        Self {
            base: 0,
            added: 0,
            multiplier: 1.0,
        }
    }
}

impl StatValue {
    /// Create a new stat value with the given base, zero additions, and 1.0 multiplier.
    #[must_use]
    pub fn new(base: i32) -> Self {
        Self {
            base,
            added: 0,
            multiplier: 1.0,
        }
    }

    /// Compute the effective (final) value: `floor((base + added) * multiplier)`.
    #[must_use]
    pub fn effective(&self) -> i32 {
        let raw = f64::from(self.base + self.added) * f64::from(self.multiplier);
        raw.floor() as i32
    }

    /// Add a flat amount to this stat.
    pub fn add(&mut self, amount: i32) {
        self.added += amount;
    }

    /// Multiply the current multiplier by a factor (stacks multiplicatively).
    pub fn multiply(&mut self, factor: f32) {
        self.multiplier *= factor;
    }
}
