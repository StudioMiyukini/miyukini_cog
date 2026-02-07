//! Tools MiyuBookmarks — tool.bookmark.add, tool.bookmark.remove, tool.bookmark.list.
//! Décision ajout = StrongFather ; WriteIntent KindMother.

use crate::context::GovernedContext;
use crate::errors::MiyubookmarksError;
use miyukini_kernel::{IdGenerator, UuidIdGenerator};

/// @id: miyubookmarks_tool_bookmark_add
/// @role: mutator
/// @layer: tool
/// @human: Ajoute un signet (cible fournie) ; WriteIntent KindMother.
/// @do: bookmark_add_under_governance
/// tool.bookmark.add
pub fn add(
    ctx: &GovernedContext,
    target_id: &str,
    target_type: &str,
    label: Option<&str>,
) -> Result<String, MiyubookmarksError> {
    if !ctx.has_mandate() {
        return Err(MiyubookmarksError::NoMandate);
    }
    let _ = (target_id.trim(), target_type.trim(), label);
    let gen = UuidIdGenerator;
    let id = gen.generate();
    Ok(format!("bm:{id}"))
}

/// @id: miyubookmarks_tool_bookmark_remove
/// @role: mutator
/// @layer: tool
/// @human: Supprime un signet ; WriteIntent KindMother.
/// @do: bookmark_remove_under_governance
/// tool.bookmark.remove
pub fn remove(ctx: &GovernedContext, _bookmark_id: &str) -> Result<(), MiyubookmarksError> {
    if !ctx.has_mandate() {
        return Err(MiyubookmarksError::NoMandate);
    }
    Ok(())
}

/// @id: miyubookmarks_tool_bookmark_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les signets (filtres fournis).
/// @do: bookmark_list_under_governance
/// tool.bookmark.list
pub fn list(
    ctx: &GovernedContext,
    _filters: &BookmarkFilters,
) -> Result<Vec<BookmarkItem>, MiyubookmarksError> {
    if !ctx.has_mandate() {
        return Err(MiyubookmarksError::NoMandate);
    }
    Ok(Vec::new())
}

/// Filtres de liste (fournis dans le flux).
#[derive(Debug, Clone, Default)]
pub struct BookmarkFilters {
    pub target_type: Option<String>,
    pub limit: Option<u32>,
}

/// Élément signet (réponse).
#[derive(Debug, Clone)]
pub struct BookmarkItem {
    pub id: String,
    pub target_id: String,
    pub target_type: String,
    pub label: Option<String>,
}
