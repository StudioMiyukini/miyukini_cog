//! Tools MiyuProfile — tool.profile.rank.list, resolve.
//! Règles StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyuprofileError;

/// @id: miyuprofile_tool_rank_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les rangs ; règles StrongFather.
/// @do: rank_list_under_governance
/// tool.profile.rank.list
pub fn list(ctx: &GovernedContext) -> Result<Vec<RankItem>, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    Err(MiyuprofileError::Unimplemented)
}

/// @id: miyuprofile_tool_rank_resolve
/// @role: mutator
/// @layer: tool
/// @human: Résout le rang utilisateur ; règles StrongFather.
/// @do: rank_resolve_under_governance
/// tool.profile.rank.resolve
pub fn resolve(ctx: &GovernedContext, _user_id: &str) -> Result<String, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    Err(MiyuprofileError::Unimplemented)
}

/// Élément rang.
#[derive(Debug, Clone)]
pub struct RankItem {
    pub id: String,
    pub name: String,
}
