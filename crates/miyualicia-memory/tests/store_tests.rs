//! Tests d'intégration du `MemoryStore` (SQLite en mémoire).

use miyualicia_memory::{
    Episode, EpisodeOutcome, Fact, LearnedSkillStatus, MemoryStore, PendingStatus, Persona,
    Preference,
};
use serde_json::json;

fn store() -> MemoryStore {
    MemoryStore::open_in_memory().expect("init store")
}

#[test]
fn remember_and_recall_returns_fact() {
    let s = store();
    let f = Fact::new(
        "voice".into(),
        "user".into(),
        "Jean préfère prendre le métro".into(),
        "user".into(),
    );
    s.remember(&f).unwrap();
    let hits = s.recall("métro", 5).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].id, f.id);
    assert!(hits[0].content.contains("métro"));
}

#[test]
fn recall_empty_query_returns_nothing() {
    let s = store();
    let f = Fact::new("voice".into(), "user".into(), "x".into(), "user".into());
    s.remember(&f).unwrap();
    assert!(s.recall("   ", 5).unwrap().is_empty());
}

#[test]
fn forget_removes_fact_and_fts_entry() {
    let s = store();
    let f = Fact::new(
        "voice".into(),
        "user".into(),
        "le chat s'appelle Mochi".into(),
        "user".into(),
    );
    s.remember(&f).unwrap();
    assert_eq!(s.recall("Mochi", 5).unwrap().len(), 1);
    let n = s.forget(f.id).unwrap();
    assert_eq!(n, 1);
    assert!(s.recall("Mochi", 5).unwrap().is_empty());
}

#[test]
fn facts_for_persona_filters_correctly() {
    let s = store();
    s.remember(&Fact::new("work".into(), "user".into(), "A".into(), "user".into()))
        .unwrap();
    s.remember(&Fact::new(
        "family".into(),
        "user".into(),
        "B".into(),
        "user".into(),
    ))
    .unwrap();
    let work = s.facts_for_persona("work").unwrap();
    assert_eq!(work.len(), 1);
    assert_eq!(work[0].content, "A");
}

#[test]
fn prefer_upserts_value() {
    let s = store();
    let p = Preference::new("voice".into(), "notify".into(), json!("desktop"));
    s.prefer(&p).unwrap();
    let got = s.preference("voice", "notify").unwrap().expect("set");
    assert_eq!(got.value, json!("desktop"));

    let p2 = Preference::new("voice".into(), "notify".into(), json!("none"));
    s.prefer(&p2).unwrap();
    let got2 = s.preference("voice", "notify").unwrap().expect("set");
    assert_eq!(got2.value, json!("none"));
}

#[test]
fn preferences_for_scope_lists_all() {
    let s = store();
    s.prefer(&Preference::new("work".into(), "a".into(), json!(1)))
        .unwrap();
    s.prefer(&Preference::new("work".into(), "b".into(), json!(2)))
        .unwrap();
    s.prefer(&Preference::new("voice".into(), "c".into(), json!(3)))
        .unwrap();
    let work = s.preferences_for_scope("work").unwrap();
    assert_eq!(work.len(), 2);
    let keys: Vec<_> = work.iter().map(|p| p.key.clone()).collect();
    assert_eq!(keys, vec!["a", "b"]);
}

#[test]
fn unprefer_removes_entry() {
    let s = store();
    s.prefer(&Preference::new("voice".into(), "k".into(), json!(true)))
        .unwrap();
    let n = s.unprefer("voice", "k").unwrap();
    assert_eq!(n, 1);
    assert!(s.preference("voice", "k").unwrap().is_none());
}

#[test]
fn record_episode_and_recent_returns_in_order() {
    let s = store();
    let e1 = Episode::new(
        "voice".into(),
        "jaykoa".into(),
        "schedule_event".into(),
        "ajoute une réunion lundi".into(),
        json!({"title": "Sync", "when": "lundi"}),
        EpisodeOutcome::Ok,
        None,
    );
    s.record_episode(&e1).unwrap();
    let e2 = Episode::new(
        "voice".into(),
        "jaykonta".into(),
        "add_expense".into(),
        "note 12 euros de café".into(),
        json!({"amount": 12.0}),
        EpisodeOutcome::Ok,
        None,
    );
    s.record_episode(&e2).unwrap();

    let recent = s.recent_episodes(10).unwrap();
    assert_eq!(recent.len(), 2);
    // Plus récent en premier
    assert_eq!(recent[0].id, e2.id);
}

#[test]
fn recent_episodes_for_personas_filtre_en_sql() {
    let s = store();
    for persona in ["voice", "work", "intrus"] {
        s.record_episode(&Episode::new(
            persona.into(),
            "jaykoa".into(),
            "x".into(),
            format!("phrase {persona}"),
            json!({}),
            EpisodeOutcome::Ok,
            None,
        ))
        .unwrap();
    }
    let autorisees = vec!["voice".to_string(), "work".to_string()];
    let r = s.recent_episodes_for_personas(&autorisees, 10).unwrap();
    assert_eq!(r.len(), 2);
    assert!(r.iter().all(|e| e.persona != "intrus"));
    // Le filtre précède le LIMIT : limite 1 → un épisode autorisé.
    let un = s.recent_episodes_for_personas(&autorisees, 1).unwrap();
    assert_eq!(un.len(), 1);
    assert_ne!(un[0].persona, "intrus");
    // Aucune persona autorisée → rien.
    assert!(s.recent_episodes_for_personas(&[], 10).unwrap().is_empty());
}

#[test]
fn episodes_for_service_filters() {
    let s = store();
    s.record_episode(&Episode::new(
        "voice".into(),
        "jaykoa".into(),
        "x".into(),
        "y".into(),
        json!({}),
        EpisodeOutcome::Ok,
        None,
    ))
    .unwrap();
    s.record_episode(&Episode::new(
        "voice".into(),
        "jaykonta".into(),
        "z".into(),
        "w".into(),
        json!({}),
        EpisodeOutcome::Ok,
        None,
    ))
    .unwrap();
    let koa = s.episodes_for_service("jaykoa", 10).unwrap();
    assert_eq!(koa.len(), 1);
    assert_eq!(koa[0].intent, "x");
}

#[test]
fn search_episodes_finds_phrase() {
    let s = store();
    s.record_episode(&Episode::new(
        "voice".into(),
        "jaykoa".into(),
        "schedule".into(),
        "ajoute une réunion lundi".into(),
        json!({}),
        EpisodeOutcome::Ok,
        None,
    ))
    .unwrap();
    let hits = s.search_episodes("réunion", 5).unwrap();
    assert_eq!(hits.len(), 1);
}

#[test]
fn upsert_persona_and_default() {
    let s = store();
    let p = Persona::new(
        "voice".into(),
        "Voix".into(),
        "Persona vocal par défaut".into(),
        true,
    );
    s.upsert_persona(&p).unwrap();
    let got = s.persona("voice").unwrap().expect("present");
    assert!(got.is_default);
    assert_eq!(s.default_persona().unwrap().unwrap().id, "voice");

    // Mise à jour de la description, conservation du flag default
    let mut p2 = p.clone();
    p2.description = "Mis à jour".into();
    s.upsert_persona(&p2).unwrap();
    let got2 = s.persona("voice").unwrap().expect("still present");
    assert_eq!(got2.description, "Mis à jour");
}

#[test]
fn upsert_persona_rejects_empty_id() {
    let s = store();
    let p = Persona::new("".into(), "X".into(), "Y".into(), false);
    let err = s.upsert_persona(&p).unwrap_err();
    assert!(matches!(
        err,
        miyualicia_memory::MemoryError::InvalidId(_)
    ));
}

#[test]
fn personas_list_sorted() {
    let s = store();
    s.upsert_persona(&Persona::new(
        "voice".into(),
        "Voix".into(),
        "".into(),
        false,
    ))
    .unwrap();
    s.upsert_persona(&Persona::new("work".into(), "Travail".into(), "".into(), false))
        .unwrap();
    let list = s.personas().unwrap();
    let ids: Vec<_> = list.iter().map(|p| p.id.clone()).collect();
    assert_eq!(ids, vec!["voice", "work"]);
}

// ----------------------------------------------------------------------
// Persona active (P4)
// ----------------------------------------------------------------------

#[test]
fn active_persona_falls_back_to_default_when_unset() {
    let s = store();
    s.upsert_persona(&Persona::new(
        "voice".into(),
        "Voix".into(),
        "".into(),
        true,
    ))
    .unwrap();
    s.upsert_persona(&Persona::new(
        "work".into(),
        "Travail".into(),
        "".into(),
        false,
    ))
    .unwrap();
    assert_eq!(s.active_persona_id().unwrap().as_deref(), Some("voice"));
}

#[test]
fn active_persona_returns_none_without_personas() {
    let s = store();
    assert!(s.active_persona_id().unwrap().is_none());
}

#[test]
fn set_active_persona_overrides_default() {
    let s = store();
    s.upsert_persona(&Persona::new(
        "voice".into(),
        "Voix".into(),
        "".into(),
        true,
    ))
    .unwrap();
    s.upsert_persona(&Persona::new(
        "work".into(),
        "Travail".into(),
        "".into(),
        false,
    ))
    .unwrap();
    s.set_active_persona("work").unwrap();
    assert_eq!(s.active_persona_id().unwrap().as_deref(), Some("work"));
}

#[test]
fn set_active_persona_rejects_unknown() {
    let s = store();
    let err = s.set_active_persona("ghost").unwrap_err();
    assert!(matches!(
        err,
        miyualicia_memory::MemoryError::UnknownPersona(_)
    ));
}

#[test]
fn set_active_persona_rejects_empty() {
    let s = store();
    let err = s.set_active_persona("").unwrap_err();
    assert!(matches!(err, miyualicia_memory::MemoryError::InvalidId(_)));
}

// ----------------------------------------------------------------------
// Préférences de confirmation (P4)
// ----------------------------------------------------------------------

#[test]
fn confirmation_pref_returns_none_when_unset() {
    let s = store();
    assert!(s
        .confirmation_preference("voice", "jaykoa.list_events")
        .unwrap()
        .is_none());
}

#[test]
fn confirmation_pref_persona_scope_takes_priority_over_global() {
    let s = store();
    s.set_confirmation_preference("global", "jaykoa.list_events", true)
        .unwrap();
    s.set_confirmation_preference("voice", "jaykoa.list_events", false)
        .unwrap();
    assert_eq!(
        s.confirmation_preference("voice", "jaykoa.list_events")
            .unwrap(),
        Some(false)
    );
    // Pour un autre persona : pas de scope dédié → fallback global
    assert_eq!(
        s.confirmation_preference("work", "jaykoa.list_events")
            .unwrap(),
        Some(true)
    );
}

#[test]
fn confirmation_pref_global_only_works() {
    let s = store();
    s.set_confirmation_preference("global", "jaykoa.next_event", false)
        .unwrap();
    assert_eq!(
        s.confirmation_preference("anyone", "jaykoa.next_event")
            .unwrap(),
        Some(false)
    );
}

#[test]
fn confirmation_pref_round_trip() {
    let s = store();
    s.set_confirmation_preference("voice", "jaykoa.list_events", true)
        .unwrap();
    assert_eq!(
        s.confirmation_preference("voice", "jaykoa.list_events")
            .unwrap(),
        Some(true)
    );
    s.set_confirmation_preference("voice", "jaykoa.list_events", false)
        .unwrap();
    assert_eq!(
        s.confirmation_preference("voice", "jaykoa.list_events")
            .unwrap(),
        Some(false)
    );
}

#[test]
fn set_confirmation_pref_rejects_empty_args() {
    let s = store();
    assert!(s
        .set_confirmation_preference("", "jaykoa.x", true)
        .is_err());
    assert!(s.set_confirmation_preference("voice", "", true).is_err());
}

#[test]
fn confirmation_pref_ignores_non_boolean_value() {
    let s = store();
    // Pose manuellement une valeur non booléenne
    s.prefer(&miyualicia_memory::Preference::new(
        "voice".into(),
        "confirm.jaykoa.x".into(),
        json!("oui"),
    ))
    .unwrap();
    // Comme la valeur n'est pas booléenne, on tombe sur le fallback (global) qui n'a rien
    assert!(s
        .confirmation_preference("voice", "jaykoa.x")
        .unwrap()
        .is_none());
}

// ----------------------------------------------------------------------
// Skills apprises (P5)
// ----------------------------------------------------------------------

#[test]
fn propose_learned_skill_creates_proposed_status() {
    let s = store();
    let ls = s
        .propose_learned_skill(
            "voice",
            "note 12 euros de café",
            "jaykonta.add_expense",
            json!({"amount": 12, "category": "café"}),
        )
        .unwrap();
    assert_eq!(ls.status, LearnedSkillStatus::Proposed);
    assert!(ls.accepted_at.is_none());

    let got = s.learned_skill(ls.id).unwrap().unwrap();
    assert_eq!(got, ls);
}

#[test]
fn propose_learned_skill_rejects_empty_args() {
    let s = store();
    assert!(s
        .propose_learned_skill("", "ph", "qid", json!({}))
        .is_err());
    assert!(s
        .propose_learned_skill("voice", "", "qid", json!({}))
        .is_err());
    assert!(s
        .propose_learned_skill("voice", "ph", "", json!({}))
        .is_err());
}

#[test]
fn accept_learned_skill_marks_status_and_sets_accepted_at() {
    let s = store();
    let ls = s
        .propose_learned_skill("voice", "x", "jaykoa.x", json!({}))
        .unwrap();
    s.accept_learned_skill(ls.id).unwrap();
    let got = s.learned_skill(ls.id).unwrap().unwrap();
    assert_eq!(got.status, LearnedSkillStatus::Accepted);
    assert!(got.accepted_at.is_some());
}

#[test]
fn reject_learned_skill_marks_status() {
    let s = store();
    let ls = s
        .propose_learned_skill("voice", "x", "jaykoa.x", json!({}))
        .unwrap();
    s.reject_learned_skill(ls.id).unwrap();
    let got = s.learned_skill(ls.id).unwrap().unwrap();
    assert_eq!(got.status, LearnedSkillStatus::Rejected);
    assert!(got.accepted_at.is_none());
}

#[test]
fn accept_unknown_skill_returns_not_found() {
    let s = store();
    let err = s.accept_learned_skill(uuid::Uuid::new_v4()).unwrap_err();
    assert!(matches!(
        err,
        miyualicia_memory::MemoryError::LearnedSkillNotFound(_)
    ));
}

#[test]
fn active_learned_skills_returns_only_accepted_for_persona() {
    let s = store();
    let voice_ok = s
        .propose_learned_skill("voice", "phr1", "a.b", json!({}))
        .unwrap();
    s.accept_learned_skill(voice_ok.id).unwrap();
    let _voice_pending = s
        .propose_learned_skill("voice", "phr2", "a.c", json!({}))
        .unwrap();
    let voice_rejected = s
        .propose_learned_skill("voice", "phr3", "a.d", json!({}))
        .unwrap();
    s.reject_learned_skill(voice_rejected.id).unwrap();
    let work_ok = s
        .propose_learned_skill("work", "w1", "x.y", json!({}))
        .unwrap();
    s.accept_learned_skill(work_ok.id).unwrap();

    let active = s.active_learned_skills("voice").unwrap();
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].id, voice_ok.id);
}

#[test]
fn pending_learned_skills_returns_only_proposed_for_persona() {
    let s = store();
    let voice_ok = s
        .propose_learned_skill("voice", "p1", "a.b", json!({}))
        .unwrap();
    s.accept_learned_skill(voice_ok.id).unwrap();
    let voice_pending = s
        .propose_learned_skill("voice", "p2", "a.c", json!({}))
        .unwrap();

    let pending = s.pending_learned_skills("voice").unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].id, voice_pending.id);
}

#[test]
fn learned_skill_exists_detects_any_status() {
    let s = store();
    assert!(!s.learned_skill_exists("voice", "a.b").unwrap());
    let ls = s
        .propose_learned_skill("voice", "ph", "a.b", json!({}))
        .unwrap();
    assert!(s.learned_skill_exists("voice", "a.b").unwrap());
    s.reject_learned_skill(ls.id).unwrap();
    // Refusée mais elle existe quand même (pour ne pas re-proposer)
    assert!(s.learned_skill_exists("voice", "a.b").unwrap());
    // Autre persona / capability : indépendant
    assert!(!s.learned_skill_exists("work", "a.b").unwrap());
    assert!(!s.learned_skill_exists("voice", "x.y").unwrap());
}

// ----------------------------------------------------------------------
// Phrase matching (P5.4)
// ----------------------------------------------------------------------

#[test]
fn match_learned_skill_exact_normalized() {
    let s = store();
    let ls = s
        .propose_learned_skill(
            "voice",
            "12 € café",
            "jaykonta.add_expense",
            json!({"amount": 12, "category": "café"}),
        )
        .unwrap();
    s.accept_learned_skill(ls.id).unwrap();
    let m = s
        .match_learned_skill("voice", "12 € café")
        .unwrap()
        .expect("match");
    assert_eq!(m.id, ls.id);
}

#[test]
fn match_learned_skill_ignores_punctuation_and_case() {
    let s = store();
    let ls = s
        .propose_learned_skill("voice", "12 € café", "jaykonta.add_expense", json!({}))
        .unwrap();
    s.accept_learned_skill(ls.id).unwrap();
    let m = s
        .match_learned_skill("voice", "12€ Café !")
        .unwrap()
        .expect("match");
    assert_eq!(m.id, ls.id);
}

#[test]
fn match_learned_skill_phrase_contains_trigger() {
    let s = store();
    let ls = s
        .propose_learned_skill("voice", "café matin", "jaykonta.add_expense", json!({}))
        .unwrap();
    s.accept_learned_skill(ls.id).unwrap();
    // L'utilisateur entoure le trigger de mots
    let m = s
        .match_learned_skill("voice", "ajoute mon café matin habituel")
        .unwrap()
        .expect("match");
    assert_eq!(m.id, ls.id);
}

#[test]
fn match_learned_skill_empty_phrase_returns_none() {
    let s = store();
    let ls = s
        .propose_learned_skill("voice", "x", "a.b", json!({}))
        .unwrap();
    s.accept_learned_skill(ls.id).unwrap();
    assert!(s.match_learned_skill("voice", "  ").unwrap().is_none());
    assert!(s.match_learned_skill("voice", "!!!").unwrap().is_none());
}

#[test]
fn match_learned_skill_no_overlap_returns_none() {
    let s = store();
    let ls = s
        .propose_learned_skill("voice", "café matin", "jaykonta.add_expense", json!({}))
        .unwrap();
    s.accept_learned_skill(ls.id).unwrap();
    assert!(s
        .match_learned_skill("voice", "quel est mon agenda")
        .unwrap()
        .is_none());
}

#[test]
fn match_learned_skill_only_returns_accepted() {
    let s = store();
    let proposed = s
        .propose_learned_skill("voice", "café matin", "a.b", json!({}))
        .unwrap();
    // Pas accepté → ne match pas
    assert!(s
        .match_learned_skill("voice", "café matin")
        .unwrap()
        .is_none());
    s.reject_learned_skill(proposed.id).unwrap();
    assert!(s
        .match_learned_skill("voice", "café matin")
        .unwrap()
        .is_none());
}

#[test]
fn match_learned_skill_persona_isolated() {
    let s = store();
    let ls = s
        .propose_learned_skill("voice", "café matin", "a.b", json!({}))
        .unwrap();
    s.accept_learned_skill(ls.id).unwrap();
    assert!(s
        .match_learned_skill("work", "café matin")
        .unwrap()
        .is_none());
    assert!(s.match_learned_skill("voice", "café matin").unwrap().is_some());
}

#[test]
fn match_learned_skill_most_recent_wins_when_multiple_match() {
    let s = store();
    let older = s
        .propose_learned_skill("voice", "rapport", "x.old", json!({"v": 1}))
        .unwrap();
    s.accept_learned_skill(older.id).unwrap();
    // Petit délai pour que le timestamp diffère
    std::thread::sleep(std::time::Duration::from_millis(5));
    let newer = s
        .propose_learned_skill("voice", "rapport", "x.new", json!({"v": 2}))
        .unwrap();
    s.accept_learned_skill(newer.id).unwrap();
    let m = s
        .match_learned_skill("voice", "rapport")
        .unwrap()
        .expect("match");
    assert_eq!(m.id, newer.id);
    assert_eq!(m.qualified_id, "x.new");
}

#[test]
fn learned_skill_round_trip_preserves_params_template() {
    let s = store();
    let template = json!({
        "amount": 12,
        "category": "café",
        "tags": ["matin", "récurrent"]
    });
    let ls = s
        .propose_learned_skill("voice", "phrase", "jaykonta.add_expense", template.clone())
        .unwrap();
    let got = s.learned_skill(ls.id).unwrap().unwrap();
    assert_eq!(got.params_template, template);
}

// ----------------------------------------------------------------------
// Pending confirmations queue (Q.1)
// ----------------------------------------------------------------------

#[test]
fn enqueue_pending_creates_pending_status() {
    let s = store();
    let pc = s
        .enqueue_pending_confirmation(
            "demo.shout",
            json!({"loud": true}),
            "voice",
            "shout!",
            "Veux-tu vraiment crier ?",
        )
        .unwrap();
    assert_eq!(pc.status, PendingStatus::Pending);
    assert!(pc.resolved_at.is_none());
    assert_eq!(pc.qualified_id, "demo.shout");
    assert_eq!(pc.params, json!({"loud": true}));
}

#[test]
fn enqueue_pending_rejects_empty_args() {
    let s = store();
    assert!(s
        .enqueue_pending_confirmation("", json!({}), "voice", "x", "p")
        .is_err());
    assert!(s
        .enqueue_pending_confirmation("a.b", json!({}), "", "x", "p")
        .is_err());
}

#[test]
fn list_pending_returns_only_pending_status() {
    let s = store();
    let pc1 = s
        .enqueue_pending_confirmation("a.b", json!({}), "voice", "x", "p1")
        .unwrap();
    let pc2 = s
        .enqueue_pending_confirmation("c.d", json!({}), "voice", "y", "p2")
        .unwrap();
    s.mark_pending_confirmed(pc1.id).unwrap();
    let pending = s.list_pending_confirmations().unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].id, pc2.id);
}

#[test]
fn pending_confirmations_for_persona_filters() {
    let s = store();
    s.enqueue_pending_confirmation("a.b", json!({}), "voice", "x", "p")
        .unwrap();
    s.enqueue_pending_confirmation("c.d", json!({}), "work", "y", "p")
        .unwrap();
    let voice = s.pending_confirmations_for_persona("voice").unwrap();
    assert_eq!(voice.len(), 1);
    let work = s.pending_confirmations_for_persona("work").unwrap();
    assert_eq!(work.len(), 1);
    let other = s.pending_confirmations_for_persona("other").unwrap();
    assert!(other.is_empty());
}

#[test]
fn mark_confirmed_transitions_status_and_sets_resolved_at() {
    let s = store();
    let pc = s
        .enqueue_pending_confirmation("a.b", json!({}), "voice", "x", "p")
        .unwrap();
    s.mark_pending_confirmed(pc.id).unwrap();
    let got = s.pending_confirmation(pc.id).unwrap().unwrap();
    assert_eq!(got.status, PendingStatus::Confirmed);
    assert!(got.resolved_at.is_some());
}

#[test]
fn mark_cancelled_transitions_status() {
    let s = store();
    let pc = s
        .enqueue_pending_confirmation("a.b", json!({}), "voice", "x", "p")
        .unwrap();
    s.mark_pending_cancelled(pc.id).unwrap();
    let got = s.pending_confirmation(pc.id).unwrap().unwrap();
    assert_eq!(got.status, PendingStatus::Cancelled);
}

#[test]
fn cannot_re_resolve_already_resolved_pending() {
    let s = store();
    let pc = s
        .enqueue_pending_confirmation("a.b", json!({}), "voice", "x", "p")
        .unwrap();
    s.mark_pending_confirmed(pc.id).unwrap();
    let err = s.mark_pending_cancelled(pc.id).unwrap_err();
    assert!(matches!(
        err,
        miyualicia_memory::MemoryError::PendingConfirmationAlreadyResolved { .. }
    ));
}

#[test]
fn mark_unknown_pending_returns_not_found() {
    let s = store();
    let err = s.mark_pending_confirmed(uuid::Uuid::new_v4()).unwrap_err();
    assert!(matches!(
        err,
        miyualicia_memory::MemoryError::PendingConfirmationNotFound(_)
    ));
}

#[test]
fn pending_round_trip_preserves_params() {
    let s = store();
    let payload = json!({"amount": 12, "label": "café", "tags": ["recur"]});
    let pc = s
        .enqueue_pending_confirmation(
            "jaykonta.add_movement",
            payload.clone(),
            "voice",
            "12 € café",
            "Confirmer la dépense ?",
        )
        .unwrap();
    let got = s.pending_confirmation(pc.id).unwrap().unwrap();
    assert_eq!(got.params, payload);
    assert_eq!(got.qualified_id, "jaykonta.add_movement");
    assert_eq!(got.user_phrase, "12 € café");
    assert_eq!(got.prompt, "Confirmer la dépense ?");
}

// ----------------------------------------------------------------------
// R-017 : réclamation atomique avant exécution
// ----------------------------------------------------------------------

#[test]
fn claim_pending_moves_to_executing_and_returns_stored_record() {
    let s = store();
    let pc = s
        .enqueue_pending_confirmation("a.b", json!({"k": 1}), "voice", "x", "p")
        .unwrap();
    let claimed = s.claim_pending_confirmation(pc.id).unwrap();
    assert_eq!(claimed.status, PendingStatus::Executing);
    assert_eq!(claimed.qualified_id, "a.b");
    assert_eq!(claimed.params, json!({"k": 1}));
    // N'apparaît plus dans la liste des pending
    assert_eq!(s.list_pending_confirmations().unwrap().len(), 0);
}

#[test]
fn second_claim_is_refused() {
    let s = store();
    let pc = s
        .enqueue_pending_confirmation("a.b", json!({}), "voice", "x", "p")
        .unwrap();
    s.claim_pending_confirmation(pc.id).unwrap();
    let err = s.claim_pending_confirmation(pc.id).unwrap_err();
    assert!(matches!(
        err,
        miyualicia_memory::MemoryError::PendingConfirmationAlreadyResolved { .. }
    ));
}

#[test]
fn concurrent_claims_from_threads_only_one_wins() {
    let s = std::sync::Arc::new(store());
    let pc = s
        .enqueue_pending_confirmation("a.b", json!({}), "voice", "x", "p")
        .unwrap();
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let s = s.clone();
            std::thread::spawn(move || s.claim_pending_confirmation(pc.id).is_ok())
        })
        .collect();
    let wins = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .filter(|ok| *ok)
        .count();
    assert_eq!(wins, 1);
}

#[test]
fn claim_unknown_returns_not_found() {
    let s = store();
    let err = s
        .claim_pending_confirmation(uuid::Uuid::new_v4())
        .unwrap_err();
    assert!(matches!(
        err,
        miyualicia_memory::MemoryError::PendingConfirmationNotFound(_)
    ));
}

#[test]
fn cannot_cancel_claimed_pending() {
    let s = store();
    let pc = s
        .enqueue_pending_confirmation("a.b", json!({}), "voice", "x", "p")
        .unwrap();
    s.claim_pending_confirmation(pc.id).unwrap();
    assert!(s.mark_pending_cancelled(pc.id).is_err());
}

#[test]
fn finalize_sets_confirmed_or_failed_only_from_executing() {
    let s = store();
    let ok = s
        .enqueue_pending_confirmation("a.b", json!({}), "voice", "x", "p")
        .unwrap();
    let ko = s
        .enqueue_pending_confirmation("a.b", json!({}), "voice", "y", "p")
        .unwrap();
    // Finaliser sans réclamation : refusé
    assert!(s.finalize_pending_confirmation(ok.id, true).is_err());

    s.claim_pending_confirmation(ok.id).unwrap();
    s.finalize_pending_confirmation(ok.id, true).unwrap();
    let got = s.pending_confirmation(ok.id).unwrap().unwrap();
    assert_eq!(got.status, PendingStatus::Confirmed);
    assert!(got.resolved_at.is_some());

    s.claim_pending_confirmation(ko.id).unwrap();
    s.finalize_pending_confirmation(ko.id, false).unwrap();
    let got = s.pending_confirmation(ko.id).unwrap().unwrap();
    assert_eq!(got.status, PendingStatus::Failed);
    // Terminal : ni re-finalisation ni nouvelle réclamation
    assert!(s.finalize_pending_confirmation(ko.id, true).is_err());
    assert!(s.claim_pending_confirmation(ko.id).is_err());
}
