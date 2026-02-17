//! Tool MiyuText — tool.text.replace.
//! Recherche et remplacement dans une chaîne (littéral ou regex fournis).

use crate::context::GovernedContext;
use crate::errors::MiyuTextError;

/// Mode de recherche (fourni dans le flux).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplaceMode {
    /// Littéral (chaîne exacte).
    Literal,
    /// Regex (pattern fourni).
    Regex,
}

/// @id: miyutext_tool_replace
/// @role: mutator
/// @layer: tool
/// @human: Recherche et remplacement dans une chaîne ; retourne la chaîne modifiée.
/// @do: replace_under_governance
/// tool.text.replace
pub fn replace(
    ctx: &GovernedContext,
    input: &str,
    pattern: &str,
    replacement: &str,
    mode: ReplaceMode,
) -> Result<String, MiyuTextError> {
    if !ctx.has_mandate() {
        return Err(MiyuTextError::NoMandate);
    }
    match mode {
        ReplaceMode::Literal => Ok(input.replace(pattern, replacement)),
        ReplaceMode::Regex => Err(MiyuTextError::RegexNotSupportedV01),
    }
}
