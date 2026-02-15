//! # MiyuCalc
//!
//! Kit d'outils (Toolkit) de calcul et formatage numérique — toolkit.calc.miyucalc.
//! Exécution sous gouvernance, sans logique métier ni décision (BOUND-1, BOUND-2).
//!
//! ## Tools exposés
//!
//! - tool.calc.expression.evaluate
//! - tool.calc.number.format
//! - tool.calc.unit.convert
//! - tool.calc.round
//!
//! ## Références
//!
//! - [MiyuCalc - Documentation Fondatrice](../../docs/tools/MiyuCalc/MiyuCalc%20-%20Documentation%20Fondatrice.md)
//! - [MiyuCalc - Tool Governance Compliance Contract](../../docs/tools/MiyuCalc/contracts/governance/MiyuCalc%20-%20Tool%20Governance%20Compliance%20Contract.md)

// @id: toolkit.calc.miyucalc
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuCalc ; expose les modules tools, pas de capacité nouvelle.
/// @do: expose_miyucalc_toolkit

pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod expression;
pub mod number;
pub mod round;
pub mod unit;

pub use admin_cell::{
    miyucalc_admin_cell, MiyuCalcAdminCell, MiyuCalcIdentification, MiyuCalcIntegrity,
    MiyuCalcTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyuCalcError;
pub use expression::evaluate as expression_evaluate;
pub use number::format as number_format;
pub use round::round as calc_round;
pub use unit::convert as unit_convert;
