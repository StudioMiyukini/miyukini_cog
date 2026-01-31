//! Tool unit : tool.calc.unit.convert.
//!
//! Convertit une valeur d'une unité vers une autre (données fournies — BOUND-2).
//! Table interne : longueur (m, km, cm, mm, in, ft), masse (kg, g), volume (L, mL).

use crate::context::GovernedContext;
use crate::errors::MiyuCalcError;

/// @id: miyucalc_tool_unit_convert
/// @role: mutator
/// @layer: tool
/// @human: Convertit une valeur d'une unité vers une autre.
/// @do: convert_unit_under_governance
/// tool.calc.unit.convert
pub fn convert(
    ctx: &GovernedContext,
    value: f64,
    from_unit: &str,
    to_unit: &str,
) -> Result<f64, MiyuCalcError> {
    if !ctx.has_mandate() {
        return Err(MiyuCalcError::NoMandate);
    }
    let from = from_unit.trim().to_lowercase();
    let to = to_unit.trim().to_lowercase();
    if from.is_empty() || to.is_empty() {
        return Err(MiyuCalcError::InvalidUnit("empty unit".to_string()));
    }
    let factor_from = unit_to_base(&from).ok_or_else(|| MiyuCalcError::InvalidUnit(format!("unknown unit: {}", from)))?;
    let factor_to = unit_to_base(&to).ok_or_else(|| MiyuCalcError::InvalidUnit(format!("unknown unit: {}", to)))?;
    if factor_from.1 != factor_to.1 {
        return Err(MiyuCalcError::InvalidUnit(format!("incompatible units: {} vs {}", from, to)));
    }
    let base = value * factor_from.0;
    Ok(base / factor_to.0)
}

/// (facteur vers unité de base, dimension: "length"|"mass"|"volume")
fn unit_to_base(unit: &str) -> Option<(f64, &'static str)> {
    let (factor, dim) = match unit {
        "m" => (1.0, "length"),
        "km" => (1000.0, "length"),
        "cm" => (0.01, "length"),
        "mm" => (0.001, "length"),
        "in" => (0.0254, "length"),
        "ft" => (0.3048, "length"),
        "kg" => (1.0, "mass"),
        "g" => (0.001, "mass"),
        "l" => (1.0, "volume"),
        "ml" => (0.001, "volume"),
        "cl" => (0.01, "volume"),
        "dl" => (0.1, "volume"),
        _ => return None,
    };
    Some((factor, dim))
}
