//! Chargement generique de fichiers TOML depuis le disque.
//!
//! <!-- @id: sd-data-load-generic @do: define @role: arpg @layer: 3 @human: miyuk -->
//!
//! Fournit deux fonctions generiques :
//! - [`load_toml_file`] : charge et deserialise un fichier TOML unique.
//! - [`load_toml_directory`] : charge tous les fichiers TOML d'un repertoire
//!   et les regroupe dans un `HashMap` indexe par ID.

use std::collections::HashMap;
use std::path::Path;

use crate::DataLoadError;

/// Charge et deserialise un fichier TOML en struct typee.
///
/// Le chemin doit pointer vers un fichier `.toml` existant.
/// Les erreurs d'IO et de parsing sont retournees via [`DataLoadError`].
pub fn load_toml_file<T: serde::de::DeserializeOwned>(
    path: &Path,
) -> Result<T, DataLoadError> {
    let content = std::fs::read_to_string(path).map_err(|e| DataLoadError::IoError {
        path: path.display().to_string(),
        message: e.to_string(),
    })?;

    toml::from_str::<T>(&content).map_err(|e| DataLoadError::ParseError {
        path: path.display().to_string(),
        message: e.to_string(),
    })
}

/// Charge tous les fichiers TOML d'un repertoire, deserialise chacun,
/// et les regroupe dans un `HashMap` indexe par l'ID de chaque entite.
///
/// La fonction `extract_entries` est appelee sur chaque fichier deserialise
/// pour en extraire les paires `(id, valeur)`.
///
/// Retourne une erreur si :
/// - le repertoire n'existe pas
/// - un fichier ne peut pas etre lu ou parse
/// - un ID est duplique
pub fn load_toml_directory<T, V, F>(
    dir: &Path,
    extract_entries: F,
) -> Result<HashMap<String, V>, DataLoadError>
where
    T: serde::de::DeserializeOwned,
    V: std::fmt::Debug,
    F: Fn(T) -> Vec<(String, V)>,
{
    let mut result = HashMap::new();

    if !dir.exists() {
        return Err(DataLoadError::DirectoryNotFound {
            path: dir.display().to_string(),
        });
    }

    let entries = std::fs::read_dir(dir).map_err(|e| DataLoadError::IoError {
        path: dir.display().to_string(),
        message: e.to_string(),
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| DataLoadError::IoError {
            path: dir.display().to_string(),
            message: e.to_string(),
        })?;

        let path = entry.path();

        // Ignorer les sous-repertoires et les fichiers non-TOML.
        if path.is_dir() {
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }

        let parsed: T = load_toml_file(&path)?;
        let extracted = extract_entries(parsed);
        for (id, value) in extracted {
            if result.contains_key(&id) {
                return Err(DataLoadError::DuplicateId {
                    id,
                    path: path.display().to_string(),
                });
            }
            result.insert(id, value);
        }
    }

    Ok(result)
}

/// Charge tous les fichiers TOML d'un repertoire de facon recursive
/// (incluant les sous-repertoires), deserialise chacun, et les regroupe
/// dans un `HashMap` indexe par l'ID de chaque entite.
pub fn load_toml_directory_recursive<T, V, F>(
    dir: &Path,
    extract_entries: &F,
) -> Result<HashMap<String, V>, DataLoadError>
where
    T: serde::de::DeserializeOwned,
    V: std::fmt::Debug,
    F: Fn(T) -> Vec<(String, V)>,
{
    let mut result = HashMap::new();

    if !dir.exists() {
        return Err(DataLoadError::DirectoryNotFound {
            path: dir.display().to_string(),
        });
    }

    let entries = std::fs::read_dir(dir).map_err(|e| DataLoadError::IoError {
        path: dir.display().to_string(),
        message: e.to_string(),
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| DataLoadError::IoError {
            path: dir.display().to_string(),
            message: e.to_string(),
        })?;

        let path = entry.path();

        if path.is_dir() {
            // Recursion dans les sous-repertoires.
            let sub_entries = load_toml_directory_recursive(&path, extract_entries)?;
            for (id, value) in sub_entries {
                if result.contains_key(&id) {
                    return Err(DataLoadError::DuplicateId {
                        id,
                        path: path.display().to_string(),
                    });
                }
                result.insert(id, value);
            }
            continue;
        }

        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }

        let parsed: T = load_toml_file(&path)?;
        let extracted = extract_entries(parsed);
        for (id, value) in extracted {
            if result.contains_key(&id) {
                return Err(DataLoadError::DuplicateId {
                    id,
                    path: path.display().to_string(),
                });
            }
            result.insert(id, value);
        }
    }

    Ok(result)
}
