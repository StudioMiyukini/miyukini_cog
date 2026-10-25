//! Registre in-memory des skills disponibles dans l'environnement Alicia.
//!
//! Le registre est volontairement simple en P1 : un `BTreeMap` indexé par
//! identifiant qualifié (`service_id.intent`) avec deux index secondaires :
//! - par service_id, pour lister rapidement les capacités d'un service
//! - par catégorie, pour grouper par thème
//!
//! Les recherches par phrase utilisent un matching naïf (sous-chaîne, casse-
//! insensible). Le matching avancé sera porté par le LLM en P2.

// @id service.alicia.skills.registry
// @role skill_registry
// @layer 7
// @do register_and_lookup_skills
// @human Registre in-memory des skills, recherche par phrase, listing par service.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::capability::ServiceCapability;
use crate::errors::SkillsError;

/// Une skill du registre — capacité enrichie d'un timestamp d'enregistrement.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Skill {
    /// La capacité de base.
    pub capability: ServiceCapability,
    /// Source du manifeste (chemin de fichier, "inline", "manifest:<id>"...).
    #[serde(default)]
    pub source: String,
    /// Horodatage RFC 3339 d'enregistrement.
    #[serde(default)]
    pub registered_at: String,
}

/// Résultat d'une recherche par phrase — score naïf.
#[derive(Debug, Clone, PartialEq)]
pub struct SkillMatch<'a> {
    /// Skill candidate.
    pub skill: &'a Skill,
    /// Score normalisé (0.0..=1.0). 1.0 = phrase d'exemple identique.
    pub score: f32,
    /// Phrase d'exemple ayant déclenché le match (si applicable).
    pub matched_phrase: Option<String>,
}

/// Registre in-memory des skills.
#[derive(Debug, Default)]
pub struct SkillRegistry {
    skills: BTreeMap<String, Skill>,
}

impl SkillRegistry {
    /// Crée un registre vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Nombre de skills enregistrées.
    #[must_use]
    pub fn len(&self) -> usize {
        self.skills.len()
    }

    /// `true` si le registre est vide.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.skills.is_empty()
    }

    /// Enregistre une skill. Échoue si l'identifiant qualifié existe déjà.
    pub fn register(&mut self, skill: Skill) -> Result<(), SkillsError> {
        skill.capability.validate()?;
        let key = skill.capability.qualified_id();
        if self.skills.contains_key(&key) {
            return Err(SkillsError::DuplicateSkill {
                service: skill.capability.service_id.clone(),
                intent: skill.capability.intent.clone(),
            });
        }
        self.skills.insert(key, skill);
        Ok(())
    }

    /// Enregistre ou remplace silencieusement (use case : hot-reload P6).
    pub fn upsert(&mut self, skill: Skill) -> Result<(), SkillsError> {
        skill.capability.validate()?;
        self.skills.insert(skill.capability.qualified_id(), skill);
        Ok(())
    }

    /// Retire toutes les skills d'un service donné — retourne le nombre retiré.
    pub fn remove_service(&mut self, service_id: &str) -> usize {
        let before = self.skills.len();
        self.skills.retain(|_, s| s.capability.service_id != service_id);
        before - self.skills.len()
    }

    /// Récupère une skill par son identifiant qualifié.
    #[must_use]
    pub fn get(&self, qualified_id: &str) -> Option<&Skill> {
        self.skills.get(qualified_id)
    }

    /// Liste toutes les skills.
    pub fn iter(&self) -> impl Iterator<Item = &Skill> {
        self.skills.values()
    }

    /// Liste les skills d'un service.
    pub fn by_service<'a>(&'a self, service_id: &'a str) -> impl Iterator<Item = &'a Skill> + 'a {
        self.skills
            .values()
            .filter(move |s| s.capability.service_id == service_id)
    }

    /// Liste les skills d'une catégorie.
    pub fn by_category<'a>(&'a self, category: &'a str) -> impl Iterator<Item = &'a Skill> + 'a {
        self.skills
            .values()
            .filter(move |s| s.capability.category.as_deref() == Some(category))
    }

    /// Recherche naïve par phrase. Retourne les skills triées par score décroissant.
    ///
    /// Heuristique P1 :
    /// - Score 1.0 si une phrase d'exemple est identique (casse-insensible)
    /// - Score 0.0..=0.9 selon la couverture de mots (intersection / max)
    /// - Skills non scorables (score 0.0) sont exclues
    #[must_use]
    pub fn search_by_phrase(&self, phrase: &str, top_k: usize) -> Vec<SkillMatch<'_>> {
        let normalized = normalize(phrase);
        if normalized.is_empty() {
            return Vec::new();
        }
        let words: Vec<&str> = normalized.split_whitespace().collect();
        if words.is_empty() {
            return Vec::new();
        }

        let mut matches: Vec<SkillMatch<'_>> = self
            .skills
            .values()
            .filter_map(|skill| {
                let mut best_score = 0.0_f32;
                let mut best_phrase = None;
                for example in &skill.capability.phrases_examples {
                    let example_norm = normalize(example);
                    if example_norm == normalized {
                        return Some(SkillMatch {
                            skill,
                            score: 1.0,
                            matched_phrase: Some(example.clone()),
                        });
                    }
                    let example_words: Vec<&str> = example_norm.split_whitespace().collect();
                    if example_words.is_empty() {
                        continue;
                    }
                    let common = words
                        .iter()
                        .filter(|w| example_words.contains(w))
                        .count();
                    let denom = words.len().max(example_words.len());
                    // Nombre de mots d'une phrase : très inférieur à 2^23, conversion exacte.
                    #[allow(clippy::cast_precision_loss)]
                    let score = common as f32 / denom as f32;
                    if score > best_score {
                        best_score = score * 0.9;
                        best_phrase = Some(example.clone());
                    }
                }
                if best_score > 0.0 {
                    Some(SkillMatch {
                        skill,
                        score: best_score,
                        matched_phrase: best_phrase,
                    })
                } else {
                    None
                }
            })
            .collect();

        matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        matches.truncate(top_k);
        matches
    }
}

/// Normalisation d'une phrase pour le matching : minuscules + suppression de la
/// ponctuation.
fn normalize(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c.is_whitespace() {
                c.to_lowercase().next().unwrap_or(c)
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::{ConfirmationPolicy, ParamSchema, ParamType};
    use std::collections::BTreeMap;

    fn skill(service: &str, intent: &str, phrases: &[&str]) -> Skill {
        let mut params = BTreeMap::new();
        params.insert("title".to_string(), ParamSchema::required_of(ParamType::String));
        Skill {
            capability: ServiceCapability {
                service_id: service.to_string(),
                intent: intent.to_string(),
                phrases_examples: phrases.iter().map(ToString::to_string).collect(),
                params,
                writes: false,
                confirmation_default: ConfirmationPolicy::Never,
                description: None,
                category: Some("calendar".into()),
            },
            source: "test".to_string(),
            registered_at: "2026-05-07T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn register_and_get() {
        let mut reg = SkillRegistry::new();
        reg.register(skill("jaykoa", "schedule_event", &["ajoute une réunion"]))
            .unwrap();
        assert_eq!(reg.len(), 1);
        let got = reg.get("jaykoa.schedule_event").expect("present");
        assert_eq!(got.capability.intent, "schedule_event");
    }

    #[test]
    fn duplicate_register_fails() {
        let mut reg = SkillRegistry::new();
        reg.register(skill("jaykoa", "schedule_event", &["x"])).unwrap();
        let err = reg
            .register(skill("jaykoa", "schedule_event", &["y"]))
            .unwrap_err();
        assert!(matches!(err, SkillsError::DuplicateSkill { .. }));
    }

    #[test]
    fn upsert_replaces_silently() {
        let mut reg = SkillRegistry::new();
        reg.register(skill("jaykoa", "schedule_event", &["a"])).unwrap();
        reg.upsert(skill("jaykoa", "schedule_event", &["b"])).unwrap();
        assert_eq!(reg.len(), 1);
        assert_eq!(
            reg.get("jaykoa.schedule_event").unwrap().capability.phrases_examples[0],
            "b"
        );
    }

    #[test]
    fn remove_service_drops_all_its_skills() {
        let mut reg = SkillRegistry::new();
        reg.register(skill("jaykoa", "a", &["x"])).unwrap();
        reg.register(skill("jaykoa", "b", &["y"])).unwrap();
        reg.register(skill("jaykonta", "c", &["z"])).unwrap();
        let removed = reg.remove_service("jaykoa");
        assert_eq!(removed, 2);
        assert_eq!(reg.len(), 1);
    }

    #[test]
    fn search_finds_exact_match_score_1() {
        let mut reg = SkillRegistry::new();
        reg.register(skill("jaykoa", "schedule", &["ajoute une réunion"]))
            .unwrap();
        let matches = reg.search_by_phrase("Ajoute une réunion", 5);
        assert_eq!(matches.len(), 1);
        assert!((matches[0].score - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn search_partial_word_overlap() {
        let mut reg = SkillRegistry::new();
        reg.register(skill("jaykoa", "schedule", &["ajoute une réunion demain"]))
            .unwrap();
        let matches = reg.search_by_phrase("ajoute une note", 5);
        // "ajoute" et "une" en commun sur 3 mots vs 4
        assert_eq!(matches.len(), 1);
        assert!(matches[0].score > 0.0);
        assert!(matches[0].score < 1.0);
    }

    #[test]
    fn search_returns_empty_on_empty_phrase() {
        let mut reg = SkillRegistry::new();
        reg.register(skill("jaykoa", "x", &["foo"])).unwrap();
        assert_eq!(reg.search_by_phrase("   ", 5).len(), 0);
    }

    #[test]
    fn by_service_filters() {
        let mut reg = SkillRegistry::new();
        reg.register(skill("jaykoa", "a", &["x"])).unwrap();
        reg.register(skill("jaykonta", "b", &["y"])).unwrap();
        let koas: Vec<_> = reg.by_service("jaykoa").collect();
        assert_eq!(koas.len(), 1);
        assert_eq!(koas[0].capability.intent, "a");
    }

    #[test]
    fn by_category_filters() {
        let mut reg = SkillRegistry::new();
        reg.register(skill("jaykoa", "a", &["x"])).unwrap();
        let cal: Vec<_> = reg.by_category("calendar").collect();
        assert_eq!(cal.len(), 1);
        let none: Vec<_> = reg.by_category("finance").collect();
        assert_eq!(none.len(), 0);
    }

    #[test]
    fn normalize_strips_punctuation_and_lowercases() {
        assert_eq!(normalize("Ajoute, une réunion!"), "ajoute une réunion");
    }
}
