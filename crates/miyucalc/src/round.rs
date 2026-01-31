//! Tool round : tool.calc.round.
//!
//! Arrondit une valeur selon mode et précision fournis — BOUND-2.

use crate::context::GovernedContext;
use crate::errors::MiyuCalcError;

/// Mode d'arrondi (fourni dans le flux).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundMode {
    /// Arrondi au plus proche (demi pair).
    HalfEven,
    /// Vers le bas (plancher).
    Floor,
    /// Vers le haut (plafond).
    Ceiling,
    /// Troncature vers zéro.
    Truncate,
}

/// @id: miyucalc_tool_round
/// @role: mutator
/// @layer: tool
/// @human: Arrondit une valeur selon mode et précision fournis.
/// @do: round_under_governance
/// tool.calc.round
pub fn round(
    ctx: &GovernedContext,
    value: f64,
    decimals: u32,
    mode: RoundMode,
) -> Result<f64, MiyuCalcError> {
    if !ctx.has_mandate() {
        return Err(MiyuCalcError::NoMandate);
    }
    let scale = 10_f64.powi(decimals as i32);
    let scaled = value * scale;
    let out = match mode {
        RoundMode::HalfEven => round_half_even(scaled) / scale,
        RoundMode::Floor => scaled.floor() / scale,
        RoundMode::Ceiling => scaled.ceil() / scale,
        RoundMode::Truncate => scaled.trunc() / scale,
    };
    Ok(out)
}

fn round_half_even(x: f64) -> f64 {
    let f = x.fract().abs();
    let t = x.trunc();
    if f < 0.5 - 1e-12 {
        t
    } else if f > 0.5 + 1e-12 {
        t + x.signum()
    } else {
        if (t as i64).abs() % 2 == 0 {
            t
        } else {
            t + x.signum()
        }
    }
}
