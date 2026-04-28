//! Recherche transversale — index et requêtes sur toutes les sections.
//!
//! Permet de retrouver rapidement une information à travers
//! l'ensemble du profil Central (identité, documents, contrats, etc.).

use crate::context::GovernedContext;
use crate::errors::MiyuprofileError;
use crate::sections::{CentralProfile, SectionName};
use serde::{Deserialize, Serialize};

/// Résultat de recherche.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Section où l'information a été trouvée.
    pub section: String,
    /// Chemin vers le champ (ex. "identity.first_name", "contracts[2].label").
    pub field_path: String,
    /// Valeur trouvée.
    pub value: String,
    /// Score de pertinence (0.0 — 1.0).
    pub relevance: f64,
}

/// Options de recherche.
#[derive(Debug, Clone, Default)]
pub struct SearchOptions {
    /// Limiter la recherche à certaines sections.
    pub sections: Option<Vec<SectionName>>,
    /// Nombre maximum de résultats.
    pub limit: Option<usize>,
    /// Recherche insensible à la casse (défaut : true).
    pub case_insensitive: bool,
}

impl SearchOptions {
    pub fn new() -> Self {
        Self {
            case_insensitive: true,
            ..Default::default()
        }
    }
}

/// tool.profile.search — Recherche textuelle dans le profil sérialisé.
///
/// Parcourt le JSON sérialisé du profil Central et retourne
/// les champs correspondant à la requête.
pub fn query(
    ctx: &GovernedContext,
    profile: &CentralProfile,
    search_query: &str,
    options: &SearchOptions,
) -> Result<Vec<SearchResult>, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    if search_query.is_empty() {
        return Ok(Vec::new());
    }

    let json_value = serde_json::to_value(profile)
        .map_err(|e| MiyuprofileError::InvalidInput(format!("serialize: {e}")))?;

    let needle = if options.case_insensitive {
        search_query.to_lowercase()
    } else {
        search_query.to_string()
    };

    let mut results = Vec::new();
    collect_matches(&json_value, "", &needle, options.case_insensitive, &mut results);

    // Filtrer par section si demandé.
    if let Some(ref sections) = options.sections {
        let section_names: Vec<String> = sections.iter().map(|s| format!("{s:?}").to_lowercase()).collect();
        results.retain(|r| {
            section_names.iter().any(|sn| r.section.to_lowercase() == *sn)
        });
    }

    // Trier par pertinence décroissante.
    results.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap_or(std::cmp::Ordering::Equal));

    // Limiter.
    if let Some(limit) = options.limit {
        results.truncate(limit);
    }

    Ok(results)
}

/// Parcours récursif du JSON pour trouver les valeurs correspondantes.
fn collect_matches(
    value: &serde_json::Value,
    path: &str,
    needle: &str,
    case_insensitive: bool,
    results: &mut Vec<SearchResult>,
) {
    match value {
        serde_json::Value::String(s) => {
            let haystack = if case_insensitive {
                s.to_lowercase()
            } else {
                s.clone()
            };
            if haystack.contains(needle) {
                let section = path.split('.').next().unwrap_or("").to_string();
                let relevance = if haystack == needle {
                    1.0
                } else if haystack.starts_with(needle) {
                    0.8
                } else {
                    0.5
                };
                results.push(SearchResult {
                    section,
                    field_path: path.to_string(),
                    value: s.clone(),
                    relevance,
                });
            }
        }
        serde_json::Value::Object(map) => {
            for (key, val) in map {
                let child_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{path}.{key}")
                };
                collect_matches(val, &child_path, needle, case_insensitive, results);
            }
        }
        serde_json::Value::Array(arr) => {
            for (idx, val) in arr.iter().enumerate() {
                let child_path = format!("{path}[{idx}]");
                collect_matches(val, &child_path, needle, case_insensitive, results);
            }
        }
        _ => {}
    }
}

/// tool.profile.search.sections — Liste les sections non-vides d'un profil.
pub fn populated_sections(
    ctx: &GovernedContext,
    profile: &CentralProfile,
) -> Result<Vec<SectionName>, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let mut sections = Vec::new();
    if profile.identity.is_some() {
        sections.push(SectionName::Identity);
    }
    if profile.contacts.is_some() {
        sections.push(SectionName::Contacts);
    }
    if profile.documents.is_some() {
        sections.push(SectionName::Documents);
    }
    if profile.health.is_some() {
        sections.push(SectionName::Health);
    }
    if profile.professional.is_some() {
        sections.push(SectionName::Professional);
    }
    if profile.enterprises.is_some() {
        sections.push(SectionName::Enterprises);
    }
    if profile.contracts.is_some() {
        sections.push(SectionName::Contracts);
    }
    if profile.finance.is_some() {
        sections.push(SectionName::Finance);
    }
    if profile.credentials.is_some() {
        sections.push(SectionName::Credentials);
    }
    if !profile.preferences.is_empty() {
        sections.push(SectionName::Preferences);
    }
    if !profile.custom_fields.is_empty() {
        sections.push(SectionName::CustomFields);
    }
    Ok(sections)
}
