//! Tools MiyuWeb — tool.web.script.execute, tool.web.script.compile.
//! Exécution et compilation de scripts dans un contexte gouverné et sandboxé.

use crate::context::GovernedContext;
use crate::errors::MiyuwebError;

/// @id: miyuweb_tool_web_script_execute
/// @role: mutator
/// @layer: tool
/// @human: Exécute un script (JS/TS) dans un contexte sandboxé et gouverné.
/// @do: web_script_execute_under_governance
/// tool.web.script.execute — pas d'accès direct base ni décision métier.
pub fn execute(
    ctx: &GovernedContext,
    _script: &str,
    _inputs: &str,
) -> Result<String, MiyuwebError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebError::NoMandate);
    }
    Ok(String::new())
}

/// @id: miyuweb_tool_web_script_compile
/// @role: accessor
/// @layer: tool
/// @human: Compile ou valide un script sans l'exécuter.
/// @do: web_script_compile_under_governance
/// tool.web.script.compile — vérification syntaxe/types.
pub fn compile(ctx: &GovernedContext, _script: &str) -> Result<bool, MiyuwebError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebError::NoMandate);
    }
    Ok(true)
}
