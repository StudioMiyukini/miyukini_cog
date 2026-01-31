//! Tool MiyuNotify — tool.notify.inbox.write.
//! Écrit une entrée en boîte de réception in-app ; WriteIntent KindMother.
//! Implémentation minimale : retourne un intent_id opaque pour traçabilité ; persistance = flux / KindMother.

use crate::context::GovernedContext;
use crate::errors::MiyunotifyError;
use miyukini_kernel::{IdGenerator, UuidIdGenerator};

/// Résultat d'écriture inbox (placeholder pour WriteIntent).
/// En implémentation réelle : retourne un identifiant d'intent pour KindMother.
#[derive(Debug, Clone)]
pub struct InboxWriteResult {
    /// Identifiant de l'intent (pour traçabilité).
    pub intent_id: String,
}

/// @id: miyunotify_tool_inbox_write
/// @role: mutator
/// @layer: tool
/// @human: Écrit une entrée en boîte de réception in-app ; WriteIntent KindMother.
/// @do: inbox_write_under_governance
/// tool.notify.inbox.write — ne décide pas ; produit WriteIntent.
pub fn write(
    ctx: &GovernedContext,
    _recipient: &str,
    _content: &str,
    _metadata: Option<&str>,
) -> Result<InboxWriteResult, MiyunotifyError> {
    if !ctx.has_mandate() {
        return Err(MiyunotifyError::NoMandate);
    }
    let gen = UuidIdGenerator;
    let id = gen.generate();
    Ok(InboxWriteResult {
        intent_id: format!("inbox:{}", id),
    })
}
