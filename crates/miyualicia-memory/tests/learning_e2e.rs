//! Tests d'intégration end-to-end de l'apprentissage Alicia (P5).
//!
//! Scénario complet « Hermes-style » :
//! 1. L'utilisateur exécute la même capability 3 fois (épisodes Ok)
//! 2. Le détecteur de patterns trouve la répétition
//! 3. Alicia propose la skill apprise → statut `Proposed`
//! 4. L'utilisateur l'accepte → statut `Accepted`
//! 5. La skill est listée dans `active_learned_skills` au runtime suivant
//! 6. Le détecteur ne re-propose plus la même capability

use miyualicia_memory::{
    detect_repetitions, propose_from_pattern, DetectorOptions, Episode, EpisodeOutcome,
    LearnedSkillStatus, MemoryStore,
};
use serde_json::json;

fn record_ok(store: &MemoryStore, persona: &str, qid: &str, phrase: &str, params: serde_json::Value) {
    let (service, intent) = qid.split_once('.').unwrap();
    store
        .record_episode(&Episode::new(
            persona.into(),
            service.into(),
            intent.into(),
            phrase.into(),
            params,
            EpisodeOutcome::Ok,
            None,
        ))
        .unwrap();
}

#[test]
fn full_learning_cycle_propose_accept_lookup() {
    let store = MemoryStore::open_in_memory().unwrap();

    // Étape 1 : 3 utilisations de jaykonta.add_expense par voice
    record_ok(
        &store,
        "voice",
        "jaykonta.add_expense",
        "12 € café",
        json!({"amount": 12, "category": "café"}),
    );
    record_ok(
        &store,
        "voice",
        "jaykonta.add_expense",
        "8 € café",
        json!({"amount": 8, "category": "café"}),
    );
    record_ok(
        &store,
        "voice",
        "jaykonta.add_expense",
        "5 € café au matin",
        json!({"amount": 5, "category": "café"}),
    );

    // Étape 2 : détection
    let patterns = detect_repetitions(&store, "voice", &DetectorOptions::default()).unwrap();
    assert_eq!(patterns.len(), 1);
    let pattern = &patterns[0];
    assert_eq!(pattern.qualified_id, "jaykonta.add_expense");
    assert_eq!(pattern.occurrence_count, 3);

    // Étape 3 : proposition (ce que ferait le runtime au démarrage suivant
    // ou lors d'un check périodique)
    let proposed = propose_from_pattern(&store, pattern).unwrap();
    assert_eq!(proposed.status, LearnedSkillStatus::Proposed);
    assert_eq!(
        store.pending_learned_skills("voice").unwrap().len(),
        1
    );

    // Étape 4 : l'utilisateur accepte via le dialogue de promotion
    store.accept_learned_skill(proposed.id).unwrap();

    // Étape 5 : la skill est active pour la persona voice
    let active = store.active_learned_skills("voice").unwrap();
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].id, proposed.id);
    assert_eq!(active[0].status, LearnedSkillStatus::Accepted);
    assert!(active[0].accepted_at.is_some());

    // Le pending est désormais vide
    assert!(store.pending_learned_skills("voice").unwrap().is_empty());

    // Étape 6 : le détecteur ne doit plus proposer la même chose
    let patterns_after = detect_repetitions(&store, "voice", &DetectorOptions::default()).unwrap();
    assert!(
        patterns_after.is_empty(),
        "le pattern déjà appris ne doit plus être re-proposé"
    );
}

#[test]
fn rejected_pattern_not_re_proposed() {
    let store = MemoryStore::open_in_memory().unwrap();
    for _ in 0..3 {
        record_ok(&store, "voice", "a.b", "x", json!({}));
    }
    let patterns = detect_repetitions(&store, "voice", &DetectorOptions::default()).unwrap();
    let proposed = propose_from_pattern(&store, &patterns[0]).unwrap();
    store.reject_learned_skill(proposed.id).unwrap();

    // Même 3 utilisations supplémentaires ne doivent pas re-générer un pattern
    for _ in 0..3 {
        record_ok(&store, "voice", "a.b", "x", json!({}));
    }
    let again = detect_repetitions(&store, "voice", &DetectorOptions::default()).unwrap();
    assert!(again.is_empty(), "rejeté = ne plus re-proposer");
}

#[test]
fn personas_have_independent_learnings() {
    let store = MemoryStore::open_in_memory().unwrap();
    // 3 fois en voice
    for _ in 0..3 {
        record_ok(&store, "voice", "jaykonta.add_expense", "café", json!({}));
    }
    // 3 fois en work, capability différente
    for _ in 0..3 {
        record_ok(&store, "work", "jaykoa.list_events", "agenda", json!({}));
    }

    let voice_patterns = detect_repetitions(&store, "voice", &DetectorOptions::default()).unwrap();
    assert_eq!(voice_patterns.len(), 1);
    assert_eq!(voice_patterns[0].qualified_id, "jaykonta.add_expense");

    let work_patterns = detect_repetitions(&store, "work", &DetectorOptions::default()).unwrap();
    assert_eq!(work_patterns.len(), 1);
    assert_eq!(work_patterns[0].qualified_id, "jaykoa.list_events");

    // Accepter dans voice ne doit rien changer pour work
    let v = propose_from_pattern(&store, &voice_patterns[0]).unwrap();
    store.accept_learned_skill(v.id).unwrap();
    assert!(store.active_learned_skills("voice").unwrap().len() == 1);
    assert!(store.active_learned_skills("work").unwrap().is_empty());
}

#[test]
fn high_threshold_filters_low_frequency_patterns() {
    let store = MemoryStore::open_in_memory().unwrap();
    for _ in 0..3 {
        record_ok(&store, "voice", "a.b", "x", json!({}));
    }
    let opts = DetectorOptions {
        threshold: 5,
        ..Default::default()
    };
    assert!(detect_repetitions(&store, "voice", &opts).unwrap().is_empty());
}
