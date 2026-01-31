//! Tool MiyuLocale — tool.locale.translate.
//! Résout une clé de traduction dans un catalogue fourni ; retourne la chaîne ou la clé si absent.

use crate::context::GovernedContext;
use crate::errors::MiyuLocaleError;
use std::collections::HashMap;

/// @id: miyulocale_tool_translate
/// @role: mutator
/// @layer: tool
/// @human: Résout une clé de traduction dans un catalogue fourni.
/// @do: translate_under_governance
/// tool.locale.translate
pub fn translate(
    ctx: &GovernedContext,
    key: &str,
    catalogue: &HashMap<String, String>,
    fallback_key: Option<&str>,
) -> Result<String, MiyuLocaleError> {
    if !ctx.has_mandate() {
        return Err(MiyuLocaleError::NoMandate);
    }
    let k = key.trim();
    Ok(catalogue
        .get(k)
        .cloned()
        .or_else(|| fallback_key.and_then(|f| catalogue.get(f.trim()).cloned()))
        .unwrap_or_else(|| k.to_string()))
}
