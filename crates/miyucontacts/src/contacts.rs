//! Tool MiyuContacts — tool.contacts.list.
//! Liste les contacts (type fourni).

use crate::context::GovernedContext;
use crate::errors::MiyucontactsError;
use crate::friend::ContactItem;

/// @id: miyucontacts_tool_contacts_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les contacts (type fourni).
/// @do: contacts_list_under_governance
/// tool.contacts.list
pub fn list(
    ctx: &GovernedContext,
    _contact_type: Option<&str>,
) -> Result<Vec<ContactItem>, MiyucontactsError> {
    if !ctx.has_mandate() {
        return Err(MiyucontactsError::NoMandate);
    }
    Err(MiyucontactsError::Unimplemented)
}
