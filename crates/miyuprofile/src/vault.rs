//! Vault — stockage binaire chiffré de documents numérisés.
//!
//! Scans de CNI, ordonnances, contrats signés, RIB, diplômes, etc.
//! Chaque entrée est identifiée par un `vault_id` référencé depuis
//! les sections (champ `vault_refs`).
//!
//! Le chiffrement réel est délégué à BorderGuard ;
//! le Vault stocke les blobs opaques + métadonnées.

use crate::context::GovernedContext;
use crate::errors::MiyuprofileError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

/// Métadonnées d'un document dans le Vault.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultEntry {
    /// Identifiant unique.
    pub id: String,
    /// Nom du fichier original.
    pub filename: String,
    /// Type MIME (application/pdf, image/jpeg…).
    pub mime_type: String,
    /// Taille en octets du blob chiffré.
    pub size_bytes: u64,
    /// SHA-256 du contenu original (avant chiffrement).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    /// Date d'ajout (ISO 8601).
    pub created_at: String,
    /// Description / label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Tags pour recherche.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// Section source (ex. "documents", "health", "contracts").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_section: Option<String>,
}

/// Blob stocké (métadonnées + données binaires).
#[derive(Debug, Clone)]
struct StoredBlob {
    entry: VaultEntry,
    data: Vec<u8>,
}

static VAULT_STORE: std::sync::OnceLock<Mutex<HashMap<String, HashMap<String, StoredBlob>>>> =
    std::sync::OnceLock::new();

fn vault_store() -> &'static Mutex<HashMap<String, HashMap<String, StoredBlob>>> {
    VAULT_STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// tool.profile.vault.store — Stocke un document dans le Vault.
pub fn store(
    ctx: &GovernedContext,
    user_id: &str,
    entry: VaultEntry,
    data: &[u8],
) -> Result<String, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let vault_id = entry.id.clone();
    let blob = StoredBlob {
        entry,
        data: data.to_vec(),
    };
    let mut guard = vault_store()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("vault lock".into()))?;
    guard
        .entry(user_id.to_string())
        .or_default()
        .insert(vault_id.clone(), blob);
    Ok(vault_id)
}

/// tool.profile.vault.get — Récupère un document du Vault.
pub fn get(
    ctx: &GovernedContext,
    user_id: &str,
    vault_id: &str,
) -> Result<(VaultEntry, Vec<u8>), MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let guard = vault_store()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("vault lock".into()))?;
    let user_vault = guard
        .get(user_id)
        .ok_or_else(|| MiyuprofileError::NotFound("vault user".into()))?;
    let blob = user_vault
        .get(vault_id)
        .ok_or_else(|| MiyuprofileError::NotFound(format!("vault:{vault_id}")))?;
    Ok((blob.entry.clone(), blob.data.clone()))
}

/// tool.profile.vault.metadata — Récupère uniquement les métadonnées.
pub fn metadata(
    ctx: &GovernedContext,
    user_id: &str,
    vault_id: &str,
) -> Result<VaultEntry, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let guard = vault_store()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("vault lock".into()))?;
    let user_vault = guard
        .get(user_id)
        .ok_or_else(|| MiyuprofileError::NotFound("vault user".into()))?;
    let blob = user_vault
        .get(vault_id)
        .ok_or_else(|| MiyuprofileError::NotFound(format!("vault:{vault_id}")))?;
    Ok(blob.entry.clone())
}

/// tool.profile.vault.list — Liste tous les documents du Vault.
pub fn list(
    ctx: &GovernedContext,
    user_id: &str,
) -> Result<Vec<VaultEntry>, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let guard = vault_store()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("vault lock".into()))?;
    let entries = guard
        .get(user_id)
        .map(|v| v.values().map(|b| b.entry.clone()).collect())
        .unwrap_or_default();
    Ok(entries)
}

/// tool.profile.vault.delete — Supprime un document du Vault.
pub fn delete(
    ctx: &GovernedContext,
    user_id: &str,
    vault_id: &str,
) -> Result<(), MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let mut guard = vault_store()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("vault lock".into()))?;
    let user_vault = guard
        .get_mut(user_id)
        .ok_or_else(|| MiyuprofileError::NotFound("vault user".into()))?;
    user_vault
        .remove(vault_id)
        .ok_or_else(|| MiyuprofileError::NotFound(format!("vault:{vault_id}")))?;
    Ok(())
}

/// tool.profile.vault.list_by_tag — Filtre les documents par tag.
pub fn list_by_tag(
    ctx: &GovernedContext,
    user_id: &str,
    tag: &str,
) -> Result<Vec<VaultEntry>, MiyuprofileError> {
    let all = list(ctx, user_id)?;
    Ok(all.into_iter().filter(|e| e.tags.contains(&tag.to_string())).collect())
}
