//! Tool MiyuContacts — tool.contacts.list.
//! Liste les contacts (type fourni : friend, foe, ou tous).

use crate::context::GovernedContext;
use crate::errors::MiyucontactsError;
use crate::foe;
use crate::friend;
use crate::friend::ContactItem;

/// @id: miyucontacts_tool_contacts_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les contacts (type fourni).
/// @do: contacts_list_under_governance
/// tool.contacts.list
pub fn list(
    ctx: &GovernedContext,
    contact_type: Option<&str>,
) -> Result<Vec<ContactItem>, MiyucontactsError> {
    if !ctx.has_mandate() {
        return Err(MiyucontactsError::NoMandate);
    }
    match contact_type {
        Some("friend") => friend::list(ctx),
        Some("foe") => foe::list(ctx),
        _ => {
            let mut out = friend::list(ctx)?;
            out.extend(foe::list(ctx)?);
            Ok(out)
        }
    }
}
