//! Tools MiyuProfile — tool.profile.rank.list, resolve.
//! Liste fixe en mémoire ; résolution = champ "rank" du profil ou défaut.

use crate::context::GovernedContext;
use crate::errors::MiyuprofileError;
use crate::profile;

/// Élément rang.
#[derive(Debug, Clone)]
pub struct RankItem {
    pub id: String,
    pub name: String,
}

fn default_ranks() -> Vec<RankItem> {
    vec![
        RankItem { id: "member".to_string(), name: "Membre".to_string() },
        RankItem { id: "vip".to_string(), name: "VIP".to_string() },
        RankItem { id: "mod".to_string(), name: "Modérateur".to_string() },
    ]
}

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
    Ok(default_ranks())
}

/// @id: miyuprofile_tool_rank_resolve
/// @role: mutator
/// @layer: tool
/// @human: Résout le rang utilisateur ; règles StrongFather.
/// @do: rank_resolve_under_governance
/// tool.profile.rank.resolve
pub fn resolve(ctx: &GovernedContext, user_id: &str) -> Result<String, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let p = profile::get(ctx, user_id)?;
    Ok(p.fields.get("rank").cloned().unwrap_or_else(|| "member".to_string()))
}
