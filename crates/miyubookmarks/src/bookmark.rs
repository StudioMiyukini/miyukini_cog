//! Tools MiyuBookmarks — tool.bookmark.add, tool.bookmark.remove, tool.bookmark.list.
//! Store en mémoire (KindMother côté produit pour persistance).

use crate::context::GovernedContext;
use crate::errors::MiyubookmarksError;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};
use std::collections::HashMap;
use std::sync::Mutex;

static BOOKMARKS: std::sync::OnceLock<Mutex<HashMap<String, Vec<BookmarkItem>>>> =
    std::sync::OnceLock::new();

fn bookmarks() -> &'static Mutex<HashMap<String, Vec<BookmarkItem>>> {
    BOOKMARKS.get_or_init(|| Mutex::new(HashMap::new()))
}

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
    let id = format!("bm:{}", UuidIdGenerator.generate());
    let item = BookmarkItem {
        id: id.clone(),
        target_id: target_id.to_string(),
        target_type: target_type.to_string(),
        label: label.map(String::from),
    };
    let mut guard = bookmarks()
        .lock()
        .map_err(|_| MiyubookmarksError::InvalidInput("lock".into()))?;
    guard.entry(ctx.mandate_id.clone()).or_default().push(item);
    Ok(id)
}

/// @id: miyubookmarks_tool_bookmark_remove
/// @role: mutator
/// @layer: tool
/// @human: Supprime un signet ; WriteIntent KindMother.
/// @do: bookmark_remove_under_governance
/// tool.bookmark.remove
pub fn remove(ctx: &GovernedContext, bookmark_id: &str) -> Result<(), MiyubookmarksError> {
    if !ctx.has_mandate() {
        return Err(MiyubookmarksError::NoMandate);
    }
    let mut guard = bookmarks()
        .lock()
        .map_err(|_| MiyubookmarksError::InvalidInput("lock".into()))?;
    if let Some(vec) = guard.get_mut(&ctx.mandate_id) {
        vec.retain(|b| b.id != bookmark_id);
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
    filters: &BookmarkFilters,
) -> Result<Vec<BookmarkItem>, MiyubookmarksError> {
    if !ctx.has_mandate() {
        return Err(MiyubookmarksError::NoMandate);
    }
    let guard = bookmarks()
        .lock()
        .map_err(|_| MiyubookmarksError::InvalidInput("lock".into()))?;
    let mut items: Vec<BookmarkItem> = guard
        .get(&ctx.mandate_id)
        .map(|v| {
            v.iter()
                .filter(|b| {
                    filters
                        .target_type
                        .as_ref()
                        .map(|t| b.target_type == *t)
                        .unwrap_or(true)
                })
                .cloned()
                .collect()
        })
        .unwrap_or_default();
    if let Some(limit) = filters.limit {
        items.truncate(limit as usize);
    }
    Ok(items)
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
