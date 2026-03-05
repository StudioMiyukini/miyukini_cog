//! Tools MiyuForum — tool.forum.topic.* (create, list, get, update, sticky, announce, export.pdf, export.text).
//! Topics : lecture, WriteIntent KindMother ; sticky/annonce = décision StrongFather ; export = exécution seule.

use crate::context::GovernedContext;
use crate::errors::MiyuforumError;

/// @id: miyuforum_tool_forum_topic_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un topic ; WriteIntent KindMother.
/// @do: forum_topic_create_under_governance
pub fn create(
    ctx: &GovernedContext,
    _board_id: &str,
    _payload: &str,
) -> Result<String, MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Ok(String::new())
}

/// @id: miyuforum_tool_forum_topic_list
/// @role: accessor
/// @layer: tool
/// @human: Liste les topics (filtres fournis).
/// @do: forum_topic_list_under_governance
pub fn list(
    ctx: &GovernedContext,
    _board_id: &str,
    _filters: Option<&str>,
) -> Result<Vec<String>, MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Ok(Vec::new())
}

/// @id: miyuforum_tool_forum_topic_get
/// @role: accessor
/// @layer: tool
/// @human: Récupère un topic par identifiant.
/// @do: forum_topic_get_under_governance
pub fn get(ctx: &GovernedContext, _topic_id: &str) -> Result<String, MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Ok(String::new())
}

/// @id: miyuforum_tool_forum_topic_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour un topic ; WriteIntent KindMother.
/// @do: forum_topic_update_under_governance
pub fn update(
    ctx: &GovernedContext,
    _topic_id: &str,
    _payload: &str,
) -> Result<(), MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Ok(())
}

/// @id: miyuforum_tool_forum_topic_sticky
/// @role: mutator
/// @layer: tool
/// @human: Marque ou démarque sticky ; décision StrongFather.
/// @do: forum_topic_sticky_under_governance
pub fn sticky(ctx: &GovernedContext, _topic_id: &str, _sticky: bool) -> Result<(), MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Ok(())
}

/// @id: miyuforum_tool_forum_topic_announce
/// @role: mutator
/// @layer: tool
/// @human: Marque ou démarque annonce ; décision StrongFather.
/// @do: forum_topic_announce_under_governance
pub fn announce(
    ctx: &GovernedContext,
    _topic_id: &str,
    _announce: bool,
) -> Result<(), MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Ok(())
}

/// @id: miyuforum_tool_forum_topic_export_pdf
/// @role: accessor
/// @layer: tool
/// @human: Exporte un topic en PDF ; exécution seule.
/// @do: forum_topic_export_pdf_under_governance
pub fn export_pdf(ctx: &GovernedContext, _topic_id: &str) -> Result<Vec<u8>, MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Ok(Vec::new())
}

/// @id: miyuforum_tool_forum_topic_export_text
/// @role: accessor
/// @layer: tool
/// @human: Exporte un topic en texte ; exécution seule.
/// @do: forum_topic_export_text_under_governance
pub fn export_text(ctx: &GovernedContext, _topic_id: &str) -> Result<String, MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Ok(String::new())
}
