//! Parsing du bloc `alicia_capabilities` d'un manifeste de service.
//!
//! Le manifeste complet (par ex. `service.manifest.json` côté miyumarket)
//! n'est pas chargé par ce crate : on ne consomme que la partie qui nous
//! concerne. Cela permet de rester découplé du crate `miyumarket`.

// @id service.alicia.skills.manifest
// @role manifest_parsing
// @layer 7
// @do parse_alicia_capabilities
// @human Parse le bloc alicia_capabilities depuis un fichier service.manifest.json.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::capability::{is_valid_id, ConfirmationPolicy, ParamSchema, ServiceCapability};
use crate::errors::SkillsError;

/// Représentation brute d'une capacité telle qu'elle apparaît dans le JSON
/// du manifeste service. Elle est convertie en [`ServiceCapability`] runtime
/// après validation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ManifestEntry {
    /// Identifiant de l'intention.
    pub intent: String,

    /// Phrases d'exemple pour le matching et le few-shot LLM.
    #[serde(default)]
    pub phrases_examples: Vec<String>,

    /// Schéma des paramètres.
    #[serde(default)]
    pub params: BTreeMap<String, ParamSchema>,

    /// `true` si la capacité mute (default `false`).
    #[serde(default)]
    pub writes: bool,

    /// Politique de confirmation par défaut.
    #[serde(default)]
    pub confirmation_default: ConfirmationPolicy,

    /// Description.
    #[serde(default)]
    pub description: Option<String>,

    /// Catégorie thématique.
    #[serde(default)]
    pub category: Option<String>,
}

/// Manifeste « projeté » : juste les champs dont Alicia a besoin pour découvrir
/// les capacités d'un service. Les autres champs (`name`, `version`, …) sont
/// ignorés volontairement.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AliciaCapabilityManifest {
    /// Identifiant du service.
    pub id: String,

    /// Capacités déclarées. Vide si le service ne s'expose pas à Alicia.
    #[serde(default)]
    pub alicia_capabilities: Vec<ManifestEntry>,
}

impl AliciaCapabilityManifest {
    /// Parse un manifeste depuis du JSON brut.
    pub fn from_json(json: &str, source_path: &Path) -> Result<Self, SkillsError> {
        serde_json::from_str(json).map_err(|e| SkillsError::Json {
            path: source_path.to_path_buf(),
            source: e,
        })
    }

    /// Charge un manifeste depuis un fichier.
    pub fn from_file(path: &Path) -> Result<Self, SkillsError> {
        let raw = std::fs::read_to_string(path).map_err(|e| SkillsError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;
        Self::from_json(&raw, path)
    }

    /// Convertit en liste de capacités runtime, validées.
    ///
    /// Les capacités invalides sont retournées sous forme d'erreur ; un manifeste
    /// peut donc contenir une capacité incorrecte sans que tout soit rejeté
    /// — le caller décide de la stratégie (skip, log, fail-fast).
    pub fn into_capabilities(
        self,
    ) -> impl Iterator<Item = Result<ServiceCapability, SkillsError>> {
        let service_id = self.id.clone();
        let service_id_for_check = self.id.clone();
        let header_check = if is_valid_id(&service_id_for_check) {
            None
        } else {
            Some(SkillsError::InvalidId(service_id_for_check))
        };
        let entries = self.alicia_capabilities.into_iter();
        // Si l'identifiant du service est invalide, on retourne UNE erreur globale
        // au début, puis on n'émet pas les capacités (qui seraient toutes rejetées
        // pour la même raison).
        let header = header_check.into_iter().map(Err);
        let body = entries.map(move |entry| {
            let cap = ServiceCapability {
                service_id: service_id.clone(),
                intent: entry.intent,
                phrases_examples: entry.phrases_examples,
                params: entry.params,
                writes: entry.writes,
                confirmation_default: entry.confirmation_default,
                description: entry.description,
                category: entry.category,
            };
            cap.validate().map(|()| cap)
        });
        header.chain(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
        "id": "jaykoa",
        "name": "JayKoa",
        "version": "0.1.0",
        "alicia_capabilities": [
            {
                "intent": "schedule_event",
                "phrases_examples": ["ajoute une réunion", "planifie un rendez-vous"],
                "params": {
                    "title": { "type": "string", "required": true },
                    "start_at": { "type": "datetime", "required": true },
                    "duration_minutes": { "type": "duration_minutes", "default": 60 }
                },
                "writes": true,
                "confirmation_default": "always_for_writes",
                "description": "Programme un nouvel événement.",
                "category": "calendar"
            },
            {
                "intent": "query_next_event",
                "phrases_examples": ["mon prochain rdv"],
                "params": {
                    "horizon": {
                        "type": "enum",
                        "enum_values": ["today", "week", "next"],
                        "default": "today"
                    }
                },
                "writes": false
            }
        ]
    }"#;

    #[test]
    fn parses_minimal_manifest() {
        let m = AliciaCapabilityManifest::from_json(SAMPLE, Path::new("test.json")).expect("parse");
        assert_eq!(m.id, "jaykoa");
        assert_eq!(m.alicia_capabilities.len(), 2);
    }

    #[test]
    fn into_capabilities_validates_and_yields_runtime_caps() {
        let m = AliciaCapabilityManifest::from_json(SAMPLE, Path::new("t.json")).expect("parse");
        let caps: Vec<ServiceCapability> = m.into_capabilities().collect::<Result<_, _>>().unwrap();
        assert_eq!(caps.len(), 2);
        assert_eq!(caps[0].qualified_id(), "jaykoa.schedule_event");
        assert!(caps[0].writes);
        assert_eq!(caps[1].qualified_id(), "jaykoa.query_next_event");
        assert!(!caps[1].writes);
    }

    #[test]
    fn ignores_unknown_top_level_fields() {
        let json = r#"{
            "id": "jaykonta",
            "name": "JayKonta",
            "alicia_capabilities": [],
            "totally_unknown": [1, 2, 3]
        }"#;
        let m = AliciaCapabilityManifest::from_json(json, Path::new("t.json")).expect("parse");
        assert_eq!(m.id, "jaykonta");
        assert_eq!(m.alicia_capabilities.len(), 0);
    }

    #[test]
    fn rejects_invalid_service_id_via_into_capabilities() {
        let json = r#"{
            "id": "bad id with spaces",
            "alicia_capabilities": []
        }"#;
        let m = AliciaCapabilityManifest::from_json(json, Path::new("t.json")).expect("parse");
        let results: Vec<_> = m.into_capabilities().collect();
        assert!(results.first().is_some_and(Result::is_err));
    }

    #[test]
    fn from_json_returns_io_for_bad_syntax() {
        let err = AliciaCapabilityManifest::from_json("{not json", Path::new("t.json")).unwrap_err();
        assert!(matches!(err, SkillsError::Json { .. }));
    }

    #[test]
    fn manifest_with_no_alicia_block_is_valid() {
        let json = r#"{ "id": "jayshop" }"#;
        let m = AliciaCapabilityManifest::from_json(json, Path::new("t.json")).expect("parse");
        assert_eq!(m.id, "jayshop");
        assert_eq!(m.alicia_capabilities.len(), 0);
    }

    #[test]
    fn from_file_reads_file_and_parses() {
        use std::io::Write;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("service.manifest.json");
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(SAMPLE.as_bytes()).unwrap();
        drop(f);
        let m = AliciaCapabilityManifest::from_file(&path).expect("parse");
        assert_eq!(m.id, "jaykoa");
    }
}
