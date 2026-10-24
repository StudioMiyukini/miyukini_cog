//! Détecteur de patterns d'utilisation — observe les épisodes pour proposer
//! des **skills apprises**.
//!
//! Heuristique P5 (volontairement simple) :
//! - Group by `qualified_id` parmi les épisodes `Ok` d'une persona
//! - Si un même `qualified_id` revient `>= threshold` fois (default 3),
//!   on génère un `RepetitionPattern`
//! - On exclut les `qualified_id` qui ont déjà une `LearnedSkill` posée
//!   (proposed / accepted / rejected) pour éviter la re-proposition en boucle
//!
//! Les évolutions futures (NLP / clustering de paramètres / TF-IDF sur les
//! phrases) viendront en P5.bis ou P7. P5 valide la chaîne.

// @id service.alicia.memory.learning
// @role pattern_detector
// @layer 7
// @do detect_repetition_patterns
// @human Détecte les usages répétés des capabilities et propose des LearnedSkill.

use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::MemoryError;
use crate::store::MemoryStore;
use crate::types::EpisodeOutcome;

/// Un pattern de répétition détecté pour un couple `(persona, qualified_id)`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepetitionPattern {
    /// Persona à laquelle le pattern est rattaché.
    pub persona: String,
    /// Identifiant qualifié de la capability (`service_id.intent`).
    pub qualified_id: String,
    /// Nombre d'occurrences observées (`outcome == Ok` uniquement).
    pub occurrence_count: usize,
    /// Phrase utilisateur de la dernière occurrence — point de départ du
    /// `trigger_phrase` quand on transformera en `LearnedSkill`.
    pub last_phrase: String,
    /// Paramètres de la dernière occurrence — sert de template initial.
    pub last_params: Value,
    /// Timestamp de la dernière occurrence.
    pub last_seen: DateTime<Utc>,
}

/// Options du détecteur.
#[derive(Debug, Clone)]
pub struct DetectorOptions {
    /// Nombre minimum d'occurrences pour qu'un pattern soit retenu (défaut 3).
    pub threshold: usize,
    /// Nombre maximum d'épisodes scannés (les plus récents). Limite la charge.
    pub max_episodes: usize,
    /// Si `true`, exclut les patterns déjà couverts par une `LearnedSkill`
    /// existante (peu importe son statut). Défaut `true`.
    pub skip_already_learned: bool,
}

impl Default for DetectorOptions {
    fn default() -> Self {
        Self {
            threshold: 3,
            max_episodes: 200,
            skip_already_learned: true,
        }
    }
}

/// Lance la détection sur les épisodes du store, filtrés sur la persona
/// donnée. Retourne les patterns triés par nombre d'occurrences décroissant.
pub fn detect_repetitions(
    store: &MemoryStore,
    persona: &str,
    opts: &DetectorOptions,
) -> Result<Vec<RepetitionPattern>, MemoryError> {
    let episodes = store.recent_episodes(opts.max_episodes)?;

    // Group by qualified_id, en filtrant sur la persona et outcome == Ok.
    let mut groups: BTreeMap<String, GroupAccumulator> = BTreeMap::new();
    for ep in episodes
        .iter()
        .filter(|e| e.persona == persona && e.outcome == EpisodeOutcome::Ok)
    {
        let qid = format!("{}.{}", ep.service_id, ep.intent);
        groups
            .entry(qid)
            .or_default()
            .observe(&ep.user_phrase, &ep.params, ep.created_at);
    }

    let mut out = Vec::new();
    for (qid, acc) in groups {
        if acc.count < opts.threshold {
            continue;
        }
        if opts.skip_already_learned && store.learned_skill_exists(persona, &qid)? {
            continue;
        }
        out.push(RepetitionPattern {
            persona: persona.to_string(),
            qualified_id: qid,
            occurrence_count: acc.count,
            last_phrase: acc.last_phrase.unwrap_or_default(),
            last_params: acc.last_params.unwrap_or(Value::Null),
            last_seen: acc.last_seen.unwrap_or_else(Utc::now),
        });
    }

    out.sort_by(|a, b| b.occurrence_count.cmp(&a.occurrence_count));
    Ok(out)
}

#[derive(Default)]
struct GroupAccumulator {
    count: usize,
    last_phrase: Option<String>,
    last_params: Option<Value>,
    last_seen: Option<DateTime<Utc>>,
}

impl GroupAccumulator {
    fn observe(&mut self, phrase: &str, params: &Value, when: DateTime<Utc>) {
        self.count += 1;
        // recent_episodes renvoie le plus récent en premier — on capture
        // donc la 1re observation pour `last_*`.
        if self.last_seen.is_none() {
            self.last_phrase = Some(phrase.to_string());
            self.last_params = Some(params.clone());
            self.last_seen = Some(when);
        }
    }
}

/// Transforme un pattern en proposition de `LearnedSkill` persistée.
///
/// Le caller décide quand appeler ça (typiquement après que l'utilisateur
/// a accepté la suggestion dans le dialogue de promotion).
pub fn propose_from_pattern(
    store: &MemoryStore,
    pattern: &RepetitionPattern,
) -> Result<crate::types::LearnedSkill, MemoryError> {
    store.propose_learned_skill(
        &pattern.persona,
        &pattern.last_phrase,
        &pattern.qualified_id,
        pattern.last_params.clone(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Episode, EpisodeOutcome};
    use serde_json::json;

    fn store() -> MemoryStore {
        MemoryStore::open_in_memory().unwrap()
    }

    fn record(s: &MemoryStore, persona: &str, service: &str, intent: &str, phrase: &str) {
        s.record_episode(&Episode::new(
            persona.into(),
            service.into(),
            intent.into(),
            phrase.into(),
            json!({}),
            EpisodeOutcome::Ok,
            None,
        ))
        .unwrap();
    }

    #[test]
    fn no_repetition_returns_empty() {
        let s = store();
        record(&s, "voice", "jaykoa", "next_event", "x");
        record(&s, "voice", "jaykoa", "list_events", "y");
        let patterns = detect_repetitions(&s, "voice", &DetectorOptions::default()).unwrap();
        assert!(patterns.is_empty());
    }

    #[test]
    fn three_repetitions_yield_pattern() {
        let s = store();
        record(&s, "voice", "jaykonta", "add_expense", "12 € café");
        record(&s, "voice", "jaykonta", "add_expense", "8 € café");
        record(&s, "voice", "jaykonta", "add_expense", "5 € café");
        let patterns = detect_repetitions(&s, "voice", &DetectorOptions::default()).unwrap();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].qualified_id, "jaykonta.add_expense");
        assert_eq!(patterns[0].occurrence_count, 3);
    }

    #[test]
    fn threshold_is_respected() {
        let s = store();
        record(&s, "voice", "a", "b", "1");
        record(&s, "voice", "a", "b", "2");
        let opts = DetectorOptions {
            threshold: 3,
            ..Default::default()
        };
        assert!(detect_repetitions(&s, "voice", &opts).unwrap().is_empty());
        let opts2 = DetectorOptions {
            threshold: 2,
            ..Default::default()
        };
        assert_eq!(
            detect_repetitions(&s, "voice", &opts2).unwrap().len(),
            1
        );
    }

    #[test]
    fn other_persona_is_ignored() {
        let s = store();
        record(&s, "work", "a", "b", "1");
        record(&s, "work", "a", "b", "2");
        record(&s, "work", "a", "b", "3");
        let patterns = detect_repetitions(&s, "voice", &DetectorOptions::default()).unwrap();
        assert!(patterns.is_empty());
    }

    #[test]
    fn failed_episodes_are_excluded() {
        let s = store();
        record(&s, "voice", "a", "b", "ok");
        record(&s, "voice", "a", "b", "ok");
        s.record_episode(&Episode::new(
            "voice".into(),
            "a".into(),
            "b".into(),
            "fail".into(),
            json!({}),
            EpisodeOutcome::Failed,
            Some("err".into()),
        ))
        .unwrap();
        // 2 Ok + 1 Failed → seuil 3 non atteint car on ne compte que les Ok
        let patterns = detect_repetitions(&s, "voice", &DetectorOptions::default()).unwrap();
        assert!(patterns.is_empty());
    }

    #[test]
    fn already_learned_pattern_is_skipped_when_option_set() {
        let s = store();
        for _ in 0..3 {
            record(&s, "voice", "a", "b", "x");
        }
        s.propose_learned_skill("voice", "x", "a.b", json!({}))
            .unwrap();
        let patterns_default = detect_repetitions(&s, "voice", &DetectorOptions::default()).unwrap();
        assert!(patterns_default.is_empty());

        let opts = DetectorOptions {
            skip_already_learned: false,
            ..Default::default()
        };
        let patterns_unfiltered = detect_repetitions(&s, "voice", &opts).unwrap();
        assert_eq!(patterns_unfiltered.len(), 1);
    }

    #[test]
    fn pattern_records_last_phrase_and_params() {
        let s = store();
        s.record_episode(&Episode::new(
            "voice".into(),
            "jaykonta".into(),
            "add_expense".into(),
            "premier".into(),
            json!({"amount": 1}),
            EpisodeOutcome::Ok,
            None,
        ))
        .unwrap();
        s.record_episode(&Episode::new(
            "voice".into(),
            "jaykonta".into(),
            "add_expense".into(),
            "deuxième".into(),
            json!({"amount": 2}),
            EpisodeOutcome::Ok,
            None,
        ))
        .unwrap();
        s.record_episode(&Episode::new(
            "voice".into(),
            "jaykonta".into(),
            "add_expense".into(),
            "troisième".into(),
            json!({"amount": 3}),
            EpisodeOutcome::Ok,
            None,
        ))
        .unwrap();
        let patterns = detect_repetitions(&s, "voice", &DetectorOptions::default()).unwrap();
        // Le plus récent (troisième) doit être celui retenu — recent_episodes
        // renvoie du plus récent au plus ancien
        assert_eq!(patterns[0].last_phrase, "troisième");
        assert_eq!(patterns[0].last_params, json!({"amount": 3}));
    }

    #[test]
    fn propose_from_pattern_persists_proposed_skill() {
        let s = store();
        for i in 0..3 {
            s.record_episode(&Episode::new(
                "voice".into(),
                "a".into(),
                "b".into(),
                format!("ph{i}"),
                json!({"k": i}),
                EpisodeOutcome::Ok,
                None,
            ))
            .unwrap();
        }
        let patterns = detect_repetitions(&s, "voice", &DetectorOptions::default()).unwrap();
        let pattern = &patterns[0];
        let ls = propose_from_pattern(&s, pattern).unwrap();
        assert_eq!(ls.qualified_id, "a.b");
        assert_eq!(ls.persona, "voice");
        let pending = s.pending_learned_skills("voice").unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id, ls.id);
    }

    #[test]
    fn results_sorted_by_occurrence_count_desc() {
        let s = store();
        for _ in 0..5 {
            record(&s, "voice", "x", "a", "p");
        }
        for _ in 0..3 {
            record(&s, "voice", "x", "b", "p");
        }
        let patterns = detect_repetitions(&s, "voice", &DetectorOptions::default()).unwrap();
        assert_eq!(patterns.len(), 2);
        assert!(patterns[0].occurrence_count >= patterns[1].occurrence_count);
        assert_eq!(patterns[0].qualified_id, "x.a");
    }
}
