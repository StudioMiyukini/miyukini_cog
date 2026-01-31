//! Tools MiyuPM — tool.pm.send, tool.pm.list, tool.pm.get.
//! Envoi, liste et récupération de messages privés ; décision StrongFather, WriteIntent KindMother.

use crate::context::GovernedContext;
use crate::errors::MiyupmError;

/// @id: miyupm_tool_pm_send
/// @role: mutator
/// @layer: tool
/// @human: Envoie un message privé ; autorisation StrongFather, WriteIntent KindMother.
/// @do: pm_send_under_governance
/// tool.pm.send — ne décide pas de l'autorisation.
pub fn send(
    ctx: &GovernedContext,
    _recipient_ref: &str,
    _content: &str,
    _subject: Option<&str>,
) -> Result<String, MiyupmError> {
    if !ctx.has_mandate() {
        return Err(MiyupmError::NoMandate);
    }
    Err(MiyupmError::Unimplemented)
}

/// @id: miyupm_tool_pm_list
/// @role: accessor
/// @layer: tool
/// @human: Liste les messages (dossier, filtres fournis).
/// @do: pm_list_under_governance
/// tool.pm.list — lecture ; critères fournis.
pub fn list(
    ctx: &GovernedContext,
    _folder_ref: &str,
    _filters: Option<&str>,
) -> Result<Vec<String>, MiyupmError> {
    if !ctx.has_mandate() {
        return Err(MiyupmError::NoMandate);
    }
    Err(MiyupmError::Unimplemented)
}

/// @id: miyupm_tool_pm_get
/// @role: accessor
/// @layer: tool
/// @human: Récupère un message par identifiant.
/// @do: pm_get_under_governance
/// tool.pm.get — lecture.
pub fn get(ctx: &GovernedContext, _message_id: &str) -> Result<String, MiyupmError> {
    if !ctx.has_mandate() {
        return Err(MiyupmError::NoMandate);
    }
    Err(MiyupmError::Unimplemented)
}
