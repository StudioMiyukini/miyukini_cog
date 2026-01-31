//! Tool MiyuWeb — tool.web.form.validate.
//! Validation formulaire (structure, champs) sans décider des règles métier.

use crate::context::GovernedContext;
use crate::errors::MiyuwebError;

/// @id: miyuweb_tool_web_form_validate
/// @role: accessor
/// @layer: tool
/// @human: Valide un formulaire (structure, champs) ; règles fournies dans le flux.
/// @do: web_form_validate_under_governance
/// tool.web.form.validate — ne définit pas les règles métier.
pub fn validate(
    ctx: &GovernedContext,
    _form_data: &str,
    _rules: &str,
) -> Result<bool, MiyuwebError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebError::NoMandate);
    }
    Err(MiyuwebError::Unimplemented)
}
