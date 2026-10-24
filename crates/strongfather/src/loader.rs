//! @id     strongfather_loader
//! @do     charger_politiques_depuis_fichier
//! @role   infrastructure
//! @layer  core
//! @human  Chargeur de politiques TOML / JSON, en refus par défaut (fail-closed).
//!
//! # StrongFather — Chargeur de politiques
//!
//! - Fichier illisible, trop gros, extension inconnue, format invalide, champ inconnu,
//!   version ≠ 1, aucune politique, identifiant dupliqué, type/effet incohérent, expression
//!   mal formée, composite introuvable ou cyclique ⇒ **erreur** ; jamais « vide ⇒ autoriser »
//! - Lecture seule : StrongFather ne persiste rien (INV-SF-2)
//!
//! ## Format (TOML)
//!
//! ```toml
//! version = 1
//!
//! [[policy]]
//! id = "jaykonta-lecture"
//! type = "permission"          # permission | constraint | validation | priority | composite
//! effect = "authorize"         # authorize | deny | constrain | validate | prioritize
//! priority = "normal"          # critical | important | normal | optional (défaut : normal)
//! condition = "true"           # défaut : "true"
//! rule = "caller == 'op-1'"
//! [policy.target]              # optionnel ; champ absent = tout
//! actions = ["read"]
//! subjects = ["jaykonta"]
//! callers = ["op-1"]
//! origins = ["maia"]
//! zones = ["H0"]
//! ```
//!
//! Le JSON suit la même structure (`{"version": 1, "policy": [ … ]}`).

use std::collections::HashMap;
use std::fmt;
use std::path::Path;

use serde::Deserialize;

use crate::intent::ActionType;
use crate::policy::{
    Policy, PolicyCondition, PolicyEffect, PolicyPriorityLevel, PolicyRule, PolicySet,
    PolicyTarget, PolicyType,
};
use crate::policy_engine::PolicyEngine;

/// Taille maximale d'un fichier de politiques (1 Mio).
pub const MAX_POLICY_FILE_BYTES: u64 = 1024 * 1024;

/// Version de format attendue.
pub const POLICY_FORMAT_VERSION: u32 = 1;

/// @id strongfather_policy_load_error
/// @role error
/// @layer core
/// @do represent_policy_load_error
//@human / @human Erreur de chargement de politiques. Toujours bloquante : aucun moteur n'est produit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyLoadError {
    /// Fichier illisible, trop gros ou extension inconnue.
    Io(String),
    /// Contenu non analysable (TOML / JSON invalide, champ inconnu, valeur hors énumération).
    Format(String),
    /// Contenu analysable mais politiquement invalide.
    Invalid(String),
}

impl fmt::Display for PolicyLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PolicyLoadError::Io(m) => write!(f, "politiques illisibles : {m}"),
            PolicyLoadError::Format(m) => write!(f, "politiques mal formées : {m}"),
            PolicyLoadError::Invalid(m) => write!(f, "politiques invalides : {m}"),
        }
    }
}

impl std::error::Error for PolicyLoadError {}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PolicyFile {
    version: u32,
    #[serde(default)]
    policy: Vec<PolicyDto>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PolicyDto {
    id: String,
    #[serde(rename = "type")]
    policy_type: TypeDto,
    effect: EffectDto,
    #[serde(default)]
    priority: Option<PriorityDto>,
    #[serde(default)]
    condition: Option<String>,
    rule: String,
    #[serde(default)]
    target: Option<TargetDto>,
    #[serde(default)]
    composite_policies: Option<Vec<String>>,
    #[serde(default)]
    metadata: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TargetDto {
    #[serde(default)]
    actions: Option<Vec<String>>,
    #[serde(default)]
    subjects: Option<Vec<String>>,
    #[serde(default)]
    callers: Option<Vec<String>>,
    #[serde(default)]
    origins: Option<Vec<String>>,
    #[serde(default)]
    zones: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum TypeDto {
    Permission,
    Constraint,
    Priority,
    Validation,
    Composite,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum EffectDto {
    Authorize,
    Deny,
    Constrain,
    Prioritize,
    Validate,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PriorityDto {
    Critical,
    Important,
    Normal,
    Optional,
}

fn build(file: PolicyFile) -> Result<PolicySet, PolicyLoadError> {
    if file.version != POLICY_FORMAT_VERSION {
        return Err(PolicyLoadError::Invalid(format!(
            "version {} non supportée (attendue : {POLICY_FORMAT_VERSION})",
            file.version
        )));
    }
    if file.policy.is_empty() {
        return Err(PolicyLoadError::Invalid(
            "aucune politique déclarée".to_string(),
        ));
    }
    let mut set = PolicySet::new();
    for dto in file.policy {
        if dto.id.trim().is_empty() {
            return Err(PolicyLoadError::Invalid(
                "politique sans identifiant".to_string(),
            ));
        }
        let policy_type = match dto.policy_type {
            TypeDto::Permission => PolicyType::Permission,
            TypeDto::Constraint => PolicyType::Constraint,
            TypeDto::Priority => PolicyType::Priority,
            TypeDto::Validation => PolicyType::Validation,
            TypeDto::Composite => PolicyType::Composite,
        };
        let effect = match dto.effect {
            EffectDto::Authorize => PolicyEffect::Authorize,
            EffectDto::Deny => PolicyEffect::Deny,
            EffectDto::Constrain => PolicyEffect::Constrain,
            EffectDto::Prioritize => PolicyEffect::Prioritize,
            EffectDto::Validate => PolicyEffect::Validate,
        };
        let priority = match dto.priority {
            None | Some(PriorityDto::Normal) => PolicyPriorityLevel::Normal,
            Some(PriorityDto::Critical) => PolicyPriorityLevel::Critical,
            Some(PriorityDto::Important) => PolicyPriorityLevel::Important,
            Some(PriorityDto::Optional) => PolicyPriorityLevel::Optional,
        };
        let target = match dto.target {
            None => PolicyTarget::default(),
            Some(t) => PolicyTarget {
                actions: t
                    .actions
                    .map(|v| {
                        v.iter()
                            .map(|a| a.parse::<ActionType>())
                            .collect::<Result<Vec<_>, _>>()
                    })
                    .transpose()
                    .map_err(|e| {
                        PolicyLoadError::Invalid(format!("politique '{}' : {e}", dto.id))
                    })?,
                subjects: t.subjects,
                callers: t.callers,
                origins: t.origins,
                zones: t.zones,
            },
        };
        let mut policy = Policy::new(
            dto.id,
            policy_type,
            PolicyCondition {
                expression: dto.condition.unwrap_or_else(|| "true".to_string()),
            },
            PolicyRule {
                expression: dto.rule,
            },
            effect,
        )
        .with_priority(priority)
        .with_metadata(dto.metadata)
        .with_target(target);
        policy.composite_policies = dto.composite_policies;
        set.add(policy).map_err(PolicyLoadError::Invalid)?;
    }
    // Validation complète (cohérence, expressions, composites) : fail-closed.
    let engine = PolicyEngine::try_new(set).map_err(|e| PolicyLoadError::Invalid(e.to_string()))?;
    Ok(engine.policies().clone())
}

impl PolicySet {
    /// @id strongfather_policy_set_from_toml
    /// @role parser
    /// @layer core
    /// @do load_policies_from_toml
    //@human / @human Charge et valide un ensemble de politiques TOML ; toute anomalie ⇒ erreur.
    pub fn from_toml_str(source: &str) -> Result<Self, PolicyLoadError> {
        let file: PolicyFile =
            toml::from_str(source).map_err(|e| PolicyLoadError::Format(e.to_string()))?;
        build(file)
    }

    /// @id strongfather_policy_set_from_json
    /// @role parser
    /// @layer core
    /// @do load_policies_from_json
    //@human / @human Charge et valide un ensemble de politiques JSON ; toute anomalie ⇒ erreur.
    pub fn from_json_str(source: &str) -> Result<Self, PolicyLoadError> {
        let file: PolicyFile =
            serde_json::from_str(source).map_err(|e| PolicyLoadError::Format(e.to_string()))?;
        build(file)
    }

    /// @id strongfather_policy_set_load_file
    /// @role parser
    /// @layer core
    /// @do load_policies_from_file
    //@human / @human Charge un fichier `.toml` ou `.json` (≤ 1 Mio) ; toute anomalie ⇒ erreur.
    pub fn load_file(path: impl AsRef<Path>) -> Result<Self, PolicyLoadError> {
        let path = path.as_ref();
        let meta = std::fs::metadata(path)
            .map_err(|e| PolicyLoadError::Io(format!("{} : {e}", path.display())))?;
        if !meta.is_file() {
            return Err(PolicyLoadError::Io(format!(
                "{} n'est pas un fichier",
                path.display()
            )));
        }
        if meta.len() > MAX_POLICY_FILE_BYTES {
            return Err(PolicyLoadError::Io(format!(
                "{} dépasse {MAX_POLICY_FILE_BYTES} octets",
                path.display()
            )));
        }
        let source = std::fs::read_to_string(path)
            .map_err(|e| PolicyLoadError::Io(format!("{} : {e}", path.display())))?;
        match path.extension().and_then(|e| e.to_str()) {
            Some("toml") => Self::from_toml_str(&source),
            Some("json") => Self::from_json_str(&source),
            _ => Err(PolicyLoadError::Io(format!(
                "{} : extension inconnue (attendu .toml ou .json)",
                path.display()
            ))),
        }
    }
}

impl PolicyEngine {
    /// @id strongfather_policy_engine_load_file
    /// @role constructor
    /// @layer core
    /// @do create_engine_from_file
    //@human / @human Construit un moteur validé depuis un fichier de politiques (fail-closed).
    pub fn load_file(path: impl AsRef<Path>) -> Result<Self, PolicyLoadError> {
        let set = PolicySet::load_file(path)?;
        PolicyEngine::try_new(set).map_err(|e| PolicyLoadError::Invalid(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decision::Verdict;
    use crate::intent::{CallContext, Intent, IntentData};

    const VALIDE: &str = r#"
version = 1

[[policy]]
id = "jaykonta-lecture"
type = "permission"
effect = "authorize"
rule = "caller == 'op-1'"
[policy.target]
actions = ["read"]
subjects = ["jaykonta"]

[[policy]]
id = "plafond"
type = "constraint"
effect = "constrain"
priority = "critical"
condition = "has(data.montant)"
rule = "data.montant <= 100"
"#;

    fn lecture(subject: &str) -> Intent {
        Intent::new(
            "i".to_string(),
            ActionType::Read,
            subject.to_string(),
            CallContext {
                caller_identity: "op-1".to_string(),
                origin: "maia".to_string(),
                instance: "cog".to_string(),
            },
            IntentData {
                content: HashMap::new(),
            },
        )
    }

    /// @id strongfather_loader_test_valid_toml
    /// @role test
    /// @layer core
    /// @do test_valid_toml_loads
    //@human / @human Un fichier TOML valide se charge et produit les décisions attendues.
    #[test]
    fn test_toml_valide() {
        let set = PolicySet::from_toml_str(VALIDE).unwrap();
        assert_eq!(set.len(), 2);
        let e = PolicyEngine::new(set);
        assert_eq!(e.decide(&lecture("jaykonta")).verdict(), Verdict::Allow);
        assert_eq!(e.decide(&lecture("jaykonta-evil")).verdict(), Verdict::Deny);
    }

    /// @id strongfather_loader_test_valid_json
    /// @role test
    /// @layer core
    /// @do test_valid_json_loads
    //@human / @human Le format JSON équivalent se charge.
    #[test]
    fn test_json_valide() {
        let src = r#"{"version":1,"policy":[{"id":"a","type":"permission","effect":"deny","rule":"true"}]}"#;
        let set = PolicySet::from_json_str(src).unwrap();
        assert_eq!(set.len(), 1);
    }

    /// @id strongfather_loader_test_fail_closed
    /// @role test
    /// @layer core
    /// @do test_loader_fail_closed
    //@human / @human Tout fichier vide, invalide ou incohérent est une erreur (jamais « vide ⇒ autoriser »).
    #[test]
    fn test_chargeur_fail_closed() {
        let cas = [
            "",
            "version = 1",
            "version = 2\n[[policy]]\nid='a'\ntype='permission'\neffect='deny'\nrule='true'",
            "version = 1\n[[policy]]\nid='a'\ntype='permission'\neffect='deny'\nrule='true'\ninconnu=1",
            "version = 1\n[[policy]]\nid='a'\ntype='permision'\neffect='deny'\nrule='true'",
            "version = 1\n[[policy]]\nid='a'\ntype='permission'\neffect='deny'",
            "version = 1\n[[policy]]\nid='a'\ntype='permission'\neffect='deny'\nrule='caller =='",
            "version = 1\n[[policy]]\nid='a'\ntype='constraint'\neffect='authorize'\nrule='true'",
            "version = 1\n[[policy]]\nid='a'\ntype='permission'\neffect='deny'\nrule='true'\n[[policy]]\nid='a'\ntype='permission'\neffect='deny'\nrule='true'",
            "version = 1\n[[policy]]\nid='a'\ntype='permission'\neffect='deny'\nrule='true'\n[policy.target]\nactions=['lire']",
            "version = 1\n[[policy]]\nid='c'\ntype='composite'\neffect='authorize'\nrule='true'\ncomposite_policies=['x']",
            "version = 1\n[[policy]]\nid=' '\ntype='permission'\neffect='deny'\nrule='true'",
            "pas du toml [",
        ];
        for src in cas {
            assert!(
                PolicySet::from_toml_str(src).is_err(),
                "doit échouer : {src:?}"
            );
        }
        assert!(PolicySet::from_json_str("{}").is_err());
        assert!(PolicySet::from_json_str(r#"{"version":1,"policy":[]}"#).is_err());
    }

    /// @id strongfather_loader_test_file
    /// @role test
    /// @layer core
    /// @do test_load_file
    //@human / @human Fichier absent ou d'extension inconnue ⇒ erreur ; fichier valide ⇒ moteur.
    #[test]
    fn test_fichier() {
        let dir = std::env::temp_dir().join(format!("sf-loader-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let ok = dir.join("politiques.toml");
        std::fs::write(&ok, VALIDE).unwrap();
        assert!(PolicyEngine::load_file(&ok).is_ok());
        let ext = dir.join("politiques.yaml");
        std::fs::write(&ext, VALIDE).unwrap();
        assert!(matches!(
            PolicyEngine::load_file(&ext),
            Err(PolicyLoadError::Io(_))
        ));
        assert!(matches!(
            PolicyEngine::load_file(dir.join("absent.toml")),
            Err(PolicyLoadError::Io(_))
        ));
        assert!(matches!(
            PolicyEngine::load_file(&dir),
            Err(PolicyLoadError::Io(_))
        ));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
