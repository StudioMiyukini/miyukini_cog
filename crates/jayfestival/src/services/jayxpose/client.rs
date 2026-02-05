//! Client JayXpose — lecture profil exposant, liste répertoire, fiche par id (Supabase alpha).
//!
//! @id: jayxpose_get_profile
//! @do: récupère le profil exposant pour l'utilisateur courant (mon compte).
//! @layer: domain
//!
//! @id: jayxpose_list_repertoire
//! @do: liste les exposants visibles dans le répertoire avec filtres et pagination.
//! @layer: domain
//!
//! @id: jayxpose_fiche_by_id
//! @do: récupère la fiche exposant publique par id.
//! @layer: domain

use crate::services::jayxpose::contract::{JayXposeProfile, RepertoireFilters, RepertoireItem};
use crate::supabase::{Exposant, SupabaseClient};
use std::error::Error;
use std::fmt;

/// Erreur du client JayXpose (réseau, décodage, non trouvé).
#[derive(Debug)]
pub enum JayXposeError {
    /// Erreur de requête HTTP ou réseau.
    Request(String),
    /// Erreur de décodage JSON.
    Decode(String),
    /// Exposant non trouvé (404 ou liste vide).
    NotFound,
}

impl fmt::Display for JayXposeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JayXposeError::Request(s) => write!(f, "JayXpose request error: {}", s),
            JayXposeError::Decode(s) => write!(f, "JayXpose decode error: {}", s),
            JayXposeError::NotFound => write!(f, "JayXpose: exposant not found"),
        }
    }
}

impl Error for JayXposeError {}

/// Récupère le profil exposant pour l'utilisateur courant (mon compte).
/// Requête authentifiée : GET exposants?id=eq.{user_id}.
///
/// @id: jayxpose_get_profile
/// @do: récupère le profil exposant pour l'utilisateur courant (mon compte).
/// @layer: domain
pub fn jayxpose_get_profile(
    client: &SupabaseClient,
    user_id: &str,
) -> Result<JayXposeProfile, JayXposeError> {
    let path = format!("exposants?id=eq.{}", user_id);
    let resp = client
        .rest_get(&path, true)
        .map_err(|e| JayXposeError::Request(e.to_string()))?;
    if !resp.status().is_success() {
        return Err(JayXposeError::Request(format!("status {}", resp.status())));
    }
    let body = resp
        .text()
        .map_err(|e| JayXposeError::Request(e.to_string()))?;
    let rows: Vec<Exposant> = serde_json::from_str(&body)
        .map_err(|e| JayXposeError::Decode(e.to_string()))?;
    let exposant = rows.into_iter().next().ok_or(JayXposeError::NotFound)?;
    Ok(JayXposeProfile::from(exposant))
}

/// Liste les exposants visibles dans le répertoire avec filtres et pagination.
/// GET exposants?visible_repertoire=eq.true&secteur=eq.X&order=company_name&limit&offset.
///
/// @id: jayxpose_list_repertoire
/// @do: liste les exposants visibles dans le répertoire avec filtres et pagination.
/// @layer: domain
pub fn jayxpose_list_repertoire(
    client: &SupabaseClient,
    filters: &RepertoireFilters,
) -> Result<Vec<RepertoireItem>, JayXposeError> {
    let mut path = "exposants?visible_repertoire=eq.true".to_string();
    if let Some(ref s) = filters.secteur {
        path.push_str(&format!("&secteur=eq.{}", urlencoding::encode(s)));
    }
    path.push_str("&order=company_name.asc");
    if let Some(limit) = filters.limit {
        path.push_str(&format!("&limit={}", limit));
    }
    if let Some(offset) = filters.offset {
        path.push_str(&format!("&offset={}", offset));
    }
    let resp = client
        .rest_get(&path, false)
        .map_err(|e| JayXposeError::Request(e.to_string()))?;
    if !resp.status().is_success() {
        return Err(JayXposeError::Request(format!("status {}", resp.status())));
    }
    let body = resp
        .text()
        .map_err(|e| JayXposeError::Request(e.to_string()))?;
    let rows: Vec<Exposant> = serde_json::from_str(&body)
        .map_err(|e| JayXposeError::Decode(e.to_string()))?;
    Ok(rows.into_iter().map(RepertoireItem::from).collect())
}

/// Récupère la fiche exposant publique par id.
/// GET exposants?id=eq.{exposant_id} (lecture publique).
///
/// @id: jayxpose_fiche_by_id
/// @do: récupère la fiche exposant publique par id.
/// @layer: domain
pub fn jayxpose_fiche_by_id(
    client: &SupabaseClient,
    exposant_id: &str,
) -> Result<JayXposeProfile, JayXposeError> {
    let path = format!("exposants?id=eq.{}", exposant_id);
    let resp = client
        .rest_get(&path, false)
        .map_err(|e| JayXposeError::Request(e.to_string()))?;
    if !resp.status().is_success() {
        return Err(JayXposeError::Request(format!("status {}", resp.status())));
    }
    let body = resp
        .text()
        .map_err(|e| JayXposeError::Request(e.to_string()))?;
    let rows: Vec<Exposant> = serde_json::from_str(&body)
        .map_err(|e| JayXposeError::Decode(e.to_string()))?;
    let exposant = rows.into_iter().next().ok_or(JayXposeError::NotFound)?;
    Ok(JayXposeProfile::from(exposant))
}
