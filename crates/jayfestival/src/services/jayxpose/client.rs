//! Client JayXpose — lecture profil exposant, liste répertoire, fiche par id (base fille KindMother).
//!
//! @id: jayxpose_get_profile
//! @do: récupère le profil exposant pour l'utilisateur courant (mon compte).
//! @id: jayxpose_list_repertoire
//! @do: liste les exposants visibles dans le répertoire avec filtres et pagination.
//! @id: jayxpose_fiche_by_id
//! @do: récupère la fiche exposant publique par id.

use crate::data::JayFestivalDb;
use crate::services::jayxpose::contract::{JayXposeProfile, RepertoireFilters, RepertoireItem};
use std::error::Error;
use std::fmt;

/// Erreur du client JayXpose (base, non trouvé).
#[derive(Debug)]
pub enum JayXposeError {
    /// Erreur base de données.
    Db(String),
    /// Exposant non trouvé (liste vide ou id inconnu).
    NotFound,
}

impl fmt::Display for JayXposeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JayXposeError::Db(s) => write!(f, "JayXpose DB: {}", s),
            JayXposeError::NotFound => write!(f, "JayXpose: exposant not found"),
        }
    }
}

impl Error for JayXposeError {}

/// Récupère le profil exposant pour l'utilisateur courant (mon compte).
///
/// @id: jayxpose_get_profile
/// @do: récupère le profil exposant pour l'utilisateur courant (mon compte).
pub fn jayxpose_get_profile(db: &JayFestivalDb, user_id: &str) -> Result<JayXposeProfile, JayXposeError> {
    let exposant = db
        .exposant_by_id(user_id)
        .map_err(|e| JayXposeError::Db(e.to_string()))?
        .ok_or(JayXposeError::NotFound)?;
    Ok(JayXposeProfile::from(exposant))
}

/// Liste les exposants visibles dans le répertoire avec filtres et pagination.
///
/// @id: jayxpose_list_repertoire
/// @do: liste les exposants visibles dans le répertoire avec filtres et pagination.
pub fn jayxpose_list_repertoire(
    db: &JayFestivalDb,
    filters: &RepertoireFilters,
) -> Result<Vec<RepertoireItem>, JayXposeError> {
    let mut rows = db
        .exposants_list(true)
        .map_err(|e| JayXposeError::Db(e.to_string()))?;
    if let Some(ref secteur) = filters.secteur {
        rows.retain(|e| e.secteur.as_deref() == Some(secteur.as_str()));
    }
    if let Some(limit) = filters.limit {
        let offset = filters.offset.unwrap_or(0) as usize;
        rows = rows.into_iter().skip(offset).take(limit as usize).collect();
    }
    Ok(rows.into_iter().map(RepertoireItem::from).collect())
}

/// Récupère la fiche exposant publique par id.
///
/// @id: jayxpose_fiche_by_id
/// @do: récupère la fiche exposant publique par id.
pub fn jayxpose_fiche_by_id(
    db: &JayFestivalDb,
    exposant_id: &str,
) -> Result<JayXposeProfile, JayXposeError> {
    let exposant = db
        .exposant_by_id(exposant_id)
        .map_err(|e| JayXposeError::Db(e.to_string()))?
        .ok_or(JayXposeError::NotFound)?;
    Ok(JayXposeProfile::from(exposant))
}
