//! Tool MiyuDiscovery — tool.social.discover.list.
//! Liste le contenu explore ; filtres fournis.

use crate::context::GovernedContext;
use crate::errors::MiyudiscoveryError;

/// @id: miyudiscovery_tool_discover_list
/// @role: mutator
/// @layer: tool
/// @human: Liste le contenu explore ; filtres fournis.
/// @do: discover_list_under_governance
/// tool.social.discover.list
pub fn list(
    ctx: &GovernedContext,
    _filters: &DiscoverFilters,
) -> Result<Vec<DiscoverItem>, MiyudiscoveryError> {
    if !ctx.has_mandate() {
        return Err(MiyudiscoveryError::NoMandate);
    }
    Ok(Vec::new())
}

/// Filtres explore.
#[derive(Debug, Clone, Default)]
pub struct DiscoverFilters {
    pub limit: Option<u32>,
    pub content_type: Option<String>,
}

/// Élément explore.
#[derive(Debug, Clone)]
pub struct DiscoverItem {
    pub id: String,
    pub content_type: String,
}
