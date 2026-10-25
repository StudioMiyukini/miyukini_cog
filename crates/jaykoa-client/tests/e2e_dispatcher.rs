//! Tests d'intégration end-to-end : depuis le `service.manifest.json` de
//! JayKoa jusqu'à l'audit `Episode` posé en mémoire après dispatch.
//!
//! Cette chaîne est la **première validation réelle** de l'architecture
//! Alicia P1+P2+P3 : manifeste → discovery → registry → dispatcher
//! → policy → core gate → ServiceClient (JayKoaClient) → backend
//! → audit MemoryStore.

use std::sync::Arc;

use chrono::NaiveDate;
use serde_json::{json, Value};

use jaykoa_client::{EventSummary, FakeJayKoaBackend, JayKoaClient};
use miyualicia_dispatcher::{
    BypassCoreGate, Decision, Dispatcher, MockServiceClient, ServiceCallResult,
};
use miyualicia_memory::{detect_repetitions, propose_from_pattern, DetectorOptions};
use miyualicia_memory::{EpisodeOutcome, MemoryStore, Persona};
use miyualicia_skills::{
    discover_in_dir, AliciaPolicy, ConfirmationPolicy, ManifestWatcher, ServiceCapability,
    Skill, SkillRegistry, TickOutcome,
};

/// Charge le `service.manifest.json` réel de JayKoa via la pipeline de
/// discovery officielle.
fn load_jaykoa_registry() -> SkillRegistry {
    // Le manifeste vit dans `crates/jaykoa/`. On crée un dossier temp avec
    // un sous-dossier `jaykoa/service.manifest.json` pointant sur le réel.
    let tmp = tempfile::tempdir().unwrap();
    let sub = tmp.path().join("jaykoa");
    std::fs::create_dir_all(&sub).unwrap();
    let real = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("jaykoa")
        .join("service.manifest.json");
    let bytes = std::fs::read(&real)
        .unwrap_or_else(|e| panic!("manifeste JayKoa introuvable à {}: {}", real.display(), e));
    std::fs::write(sub.join("service.manifest.json"), bytes).unwrap();

    let mut reg = SkillRegistry::new();
    let (count, errors) = discover_in_dir(tmp.path(), &mut reg).unwrap();
    assert!(errors.is_empty(), "discovery errors: {errors:?}");
    assert!(count >= 2, "le manifeste doit déclarer au moins 2 capabilities");
    reg
}

fn ev(id: &str, title: &str, start: &str) -> EventSummary {
    EventSummary {
        id: id.into(),
        title: title.into(),
        start: start.into(),
        end: None,
        location: None,
        all_day: false,
        source_service: None,
    }
}

#[tokio::test]
async fn discovery_loads_jaykoa_capabilities() {
    let reg = load_jaykoa_registry();
    assert!(reg.get("jaykoa.list_events").is_some());
    assert!(reg.get("jaykoa.next_event").is_some());

    let list_cap = &reg.get("jaykoa.list_events").unwrap().capability;
    assert!(!list_cap.writes);
    assert!(list_cap.params.contains_key("from"));
    assert!(list_cap.params.contains_key("to"));

    let next_cap = &reg.get("jaykoa.next_event").unwrap().capability;
    assert!(!next_cap.writes);
    assert!(next_cap.params.is_empty());
}

#[tokio::test]
async fn end_to_end_list_events_executes_and_audits() {
    let reg = load_jaykoa_registry();
    let mem = MemoryStore::open_in_memory().unwrap();

    let backend = Arc::new(FakeJayKoaBackend::new());
    let day = NaiveDate::from_ymd_opt(2026, 5, 8).unwrap();
    backend.push_event_on(day, ev("e1", "Stand-up", "2026-05-08T09:00:00Z"));
    backend.push_event_on(day, ev("e2", "Sync", "2026-05-08T11:00:00Z"));

    let client = JayKoaClient::new(backend);
    let policy = AliciaPolicy::strict();
    let gate = BypassCoreGate;

    let dispatcher = Dispatcher::new();
    let decision = dispatcher
        .execute(
            "jaykoa.list_events",
            &json!({"from": "2026-05-08", "to": "2026-05-08"}),
            "voice",
            "qu'est-ce que j'ai aujourd'hui",
            &reg,
            &policy,
            &gate,
            &client,
            &mem,
        )
        .await
        .unwrap();

    let result = match decision {
        Decision::Executed(r) => r,
        Decision::PendingConfirmation { .. } => panic!("read-only ne doit pas demander confirmation"),
    };

    assert!(result.service_result.success);
    assert_eq!(result.service_result.data["count"], 2);
    let titles: Vec<&str> = result.service_result.data["events"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| e["title"].as_str().unwrap())
        .collect();
    assert!(titles.contains(&"Stand-up"));
    assert!(titles.contains(&"Sync"));

    // Audit posé
    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent.len(), 1);
    let ep = &recent[0];
    assert_eq!(ep.service_id, "jaykoa");
    assert_eq!(ep.intent, "list_events");
    assert_eq!(ep.persona, "voice");
    assert_eq!(ep.outcome, EpisodeOutcome::Ok);
    assert_eq!(ep.user_phrase, "qu'est-ce que j'ai aujourd'hui");
}

#[tokio::test]
async fn end_to_end_next_event_returns_summary() {
    let reg = load_jaykoa_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayKoaBackend::new());
    backend.set_next(Some(ev("e1", "Brief", "2026-05-09T08:30:00Z")));
    let client = JayKoaClient::new(backend);
    let dispatcher = Dispatcher::new();

    let decision = dispatcher
        .execute(
            "jaykoa.next_event",
            &Value::Null,
            "voice",
            "quel est mon prochain rdv",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();

    if let Decision::Executed(r) = decision {
        assert!(r.service_result.success);
        assert_eq!(r.service_result.data["event"]["title"], "Brief");
    } else {
        panic!("expected Executed");
    }
}

#[tokio::test]
async fn end_to_end_backend_failure_records_failed_episode() {
    let reg = load_jaykoa_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayKoaBackend::new());
    backend.force_failure("KindMother offline");
    let client = JayKoaClient::new(backend);
    let dispatcher = Dispatcher::new();

    let err = dispatcher
        .execute(
            "jaykoa.next_event",
            &Value::Null,
            "voice",
            "?",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap_err();

    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].outcome, EpisodeOutcome::Failed);
    assert!(recent[0].error.as_deref().unwrap().contains("KindMother offline"));
    let _ = err;
}

#[tokio::test]
async fn end_to_end_invalid_params_records_failed_episode() {
    let reg = load_jaykoa_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayKoaBackend::new());
    let client = JayKoaClient::new(backend);
    let dispatcher = Dispatcher::new();

    let res = dispatcher
        .execute(
            "jaykoa.list_events",
            &json!({"from": "pas une date"}),
            "voice",
            "?",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await;
    assert!(res.is_err(), "params invalides → ServiceFailed");
    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent[0].outcome, EpisodeOutcome::Failed);
}

// ----------------------------------------------------------------------
// P4 — persona active + résolution préférences
// ----------------------------------------------------------------------

#[tokio::test]
async fn active_persona_resolved_via_memory_then_used_by_dispatcher() {
    let reg = load_jaykoa_registry();
    let mem = MemoryStore::open_in_memory().unwrap();

    // Pose deux personas, "voice" par défaut
    mem.upsert_persona(&Persona::new(
        "voice".into(),
        "Voix".into(),
        "Persona vocal".into(),
        true,
    ))
    .unwrap();
    mem.upsert_persona(&Persona::new(
        "work".into(),
        "Travail".into(),
        "Persona pro".into(),
        false,
    ))
    .unwrap();

    // Avant tout choix : actif = default = voice
    assert_eq!(mem.active_persona_id().unwrap().as_deref(), Some("voice"));

    // L'utilisateur switch sur work
    mem.set_active_persona("work").unwrap();
    assert_eq!(mem.active_persona_id().unwrap().as_deref(), Some("work"));

    // Le dispatcher reçoit le persona actif comme paramètre — l'épisode le porte
    let backend = Arc::new(FakeJayKoaBackend::new());
    backend.set_next(Some(ev("e1", "Brief", "2026-05-09T08:30:00Z")));
    let client = JayKoaClient::new(backend);

    let dispatcher = Dispatcher::new();
    let active = mem.active_persona_id().unwrap().unwrap();
    let _ = dispatcher
        .execute(
            "jaykoa.next_event",
            &Value::Null,
            &active,
            "?",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent[0].persona, "work");
}

/// Helper : enregistre une capability synthétique JayKoa.delete_event en plus
/// du registre chargé depuis le manifeste réel (qui ne déclare que les
/// read-only).
fn registry_with_user_pref_capability() -> SkillRegistry {
    let mut reg = load_jaykoa_registry();
    let cap = ServiceCapability {
        service_id: "jaykoa".into(),
        intent: "delete_event".into(),
        phrases_examples: vec!["supprime mon événement".into()],
        params: std::collections::BTreeMap::default(),
        writes: true,
        confirmation_default: ConfirmationPolicy::UserPreference,
        description: Some("Supprime un événement.".into()),
        category: Some("calendar".into()),
    };
    reg.register(Skill {
        capability: cap,
        source: "test".into(),
        registered_at: "2026-05-08T00:00:00Z".into(),
    })
    .unwrap();
    reg
}

#[tokio::test]
async fn user_preference_pending_then_skip_after_remember_choice() {
    let reg = registry_with_user_pref_capability();
    let mem = MemoryStore::open_in_memory().unwrap();
    mem.upsert_persona(&Persona::new("voice".into(), "Voix".into(), String::new(), true))
        .unwrap();

    // delete_event est une capability synthétique purement pour tester la
    // logique de préférence — JayKoaClient n'expose pas d'écriture, on utilise
    // donc un mock service client qui répond avec succès.
    let client = MockServiceClient::new();
    client.expect("jaykoa.delete_event", ServiceCallResult::ok(json!({})));
    let dispatcher = Dispatcher::new();

    // Premier appel : aucune préférence → on tombe sur PendingConfirmation
    let decision = dispatcher
        .execute(
            "jaykoa.delete_event",
            &Value::Null,
            "voice",
            "supprime",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    assert!(matches!(decision, Decision::PendingConfirmation { .. }));

    // L'utilisateur dit « ok et ne me redemande plus » — on enregistre la préférence
    mem.set_confirmation_preference("voice", "jaykoa.delete_event", false)
        .unwrap();
    // Il confirme cette fois (execute_confirmed pour matérialiser l'action)
    dispatcher
        .execute_confirmed(
            "jaykoa.delete_event",
            &Value::Null,
            "voice",
            "supprime",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();

    // Deuxième appel : la préférence est posée → exécution directe (pas de pending)
    let decision2 = dispatcher
        .execute(
            "jaykoa.delete_event",
            &Value::Null,
            "voice",
            "supprime à nouveau",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    assert!(matches!(decision2, Decision::Executed(_)));

    // Deux épisodes Ok recensés (le pending n'a pas créé d'épisode au 1er tour)
    let episodes = mem.recent_episodes(10).unwrap();
    assert_eq!(episodes.len(), 2);
    assert!(episodes
        .iter()
        .all(|e| e.outcome == EpisodeOutcome::Ok));
}

// ----------------------------------------------------------------------
// P5.5 — Full Hermes loop : observation → proposition → acceptation
//        → résolution de phrase → dispatch via JayKoaClient
// ----------------------------------------------------------------------

#[tokio::test]
async fn full_hermes_loop_observe_propose_accept_resolve_dispatch() {
    let reg = load_jaykoa_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    mem.upsert_persona(&Persona::new(
        "voice".into(),
        "Voix".into(),
        String::new(),
        true,
    ))
    .unwrap();

    // Backend JayKoa programmé avec un événement à venir
    let backend = Arc::new(FakeJayKoaBackend::new());
    backend.set_next(Some(ev("e1", "Stand-up", "2026-05-09T09:00:00Z")));
    let client = JayKoaClient::new(backend);
    let dispatcher = Dispatcher::new();

    // Étape 1 : l'utilisateur appelle 3× la même chose, dispatcher pose les épisodes
    for phrase in [
        "quel est mon prochain rendez-vous",
        "donne-moi mon prochain rdv",
        "mon prochain événement",
    ] {
        let _ = dispatcher
            .execute(
                "jaykoa.next_event",
                &Value::Null,
                "voice",
                phrase,
                &reg,
                &AliciaPolicy::strict(),
                &BypassCoreGate,
                &client,
                &mem,
            )
            .await
            .unwrap();
    }
    assert_eq!(mem.recent_episodes(10).unwrap().len(), 3);

    // Étape 2 : Alicia détecte le pattern
    let patterns = detect_repetitions(&mem, "voice", &DetectorOptions::default()).unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].qualified_id, "jaykoa.next_event");

    // Étape 3 : Alicia propose, l'utilisateur accepte avec sa propre phrase
    // courte « prochain rdv » comme trigger personnalisé
    let proposed = propose_from_pattern(&mem, &patterns[0]).unwrap();
    // L'utilisateur édite le trigger pour quelque chose de plus court
    let mut edited = proposed.clone();
    edited.trigger_phrase = "prochain rdv".into();
    mem.upsert_learned_skill(&edited).unwrap();
    mem.accept_learned_skill(edited.id).unwrap();

    // Étape 4 : nouvelle interaction — l'utilisateur dit la phrase courte
    let resolved = dispatcher
        .resolve_learned_phrase("prochain rdv", "voice", &mem)
        .unwrap()
        .expect("doit matcher");
    assert_eq!(resolved.qualified_id, "jaykoa.next_event");
    assert_eq!(resolved.learned_skill_id, edited.id);

    // Étape 5 : dispatcher exécute via le vrai JayKoaClient
    let decision = dispatcher
        .execute(
            &resolved.qualified_id,
            &resolved.params,
            "voice",
            "prochain rdv",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    let Decision::Executed(result) = decision else {
        panic!("expected Executed");
    };
    assert!(result.service_result.success);
    assert_eq!(result.service_result.data["event"]["title"], "Stand-up");

    // Audit complet : 3 épisodes initiaux + 1 nouvelle exécution = 4
    let episodes = mem.recent_episodes(10).unwrap();
    assert_eq!(episodes.len(), 4);
    assert_eq!(episodes[0].user_phrase, "prochain rdv");
}

#[tokio::test]
async fn unmatched_phrase_returns_none_no_dispatch() {
    let mem = MemoryStore::open_in_memory().unwrap();
    let dispatcher = Dispatcher::new();
    // Pas de learned skill posée
    let r = dispatcher
        .resolve_learned_phrase("phrase quelconque", "voice", &mem)
        .unwrap();
    assert!(r.is_none());
}

#[tokio::test]
async fn rejected_skill_does_not_match() {
    let mem = MemoryStore::open_in_memory().unwrap();
    let ls = mem
        .propose_learned_skill("voice", "résumé", "jaykoa.next_event", json!({}))
        .unwrap();
    mem.reject_learned_skill(ls.id).unwrap();
    let dispatcher = Dispatcher::new();
    let r = dispatcher
        .resolve_learned_phrase("résumé", "voice", &mem)
        .unwrap();
    assert!(r.is_none());
}

// ----------------------------------------------------------------------
// P6.3 — Hot swap end-to-end via ManifestWatcher
// ----------------------------------------------------------------------

/// Lit le contenu du `service.manifest.json` réel de JayKoa pour le dropper
/// dans un dossier temporaire à volonté pendant les tests hot-swap.
fn real_jaykoa_manifest_bytes() -> Vec<u8> {
    let real = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("jaykoa")
        .join("service.manifest.json");
    std::fs::read(&real).expect("manifeste JayKoa introuvable")
}

#[tokio::test]
async fn hot_swap_add_then_remove_jaykoa_manifest() {
    let tmp = tempfile::tempdir().unwrap();
    let mut reg = SkillRegistry::new();
    let mut watcher = ManifestWatcher::new();

    // Dossier vide → pas de capabilities
    match watcher.tick(tmp.path(), &mut reg).unwrap() {
        TickOutcome::Reloaded(r) => assert!(r.is_noop()),
        TickOutcome::Unchanged => {}
    }
    assert_eq!(reg.len(), 0);

    // Drop du manifeste → tick déclenche le reload, capabilities ajoutées
    let sub = tmp.path().join("jaykoa");
    std::fs::create_dir_all(&sub).unwrap();
    std::fs::write(sub.join("service.manifest.json"), real_jaykoa_manifest_bytes()).unwrap();

    match watcher.tick(tmp.path(), &mut reg).unwrap() {
        TickOutcome::Reloaded(r) => {
            assert_eq!(r.added, vec!["jaykoa".to_string()]);
            assert_eq!(r.removed, [] as [std::string::String; 0]);
        }
        TickOutcome::Unchanged => panic!("expected reload after manifest drop"),
    }
    assert!(reg.get("jaykoa.list_events").is_some());
    assert!(reg.get("jaykoa.next_event").is_some());

    // Le dispatcher exécute via JayKoaClient
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayKoaBackend::new());
    backend.set_next(Some(ev("e1", "Sync", "2026-05-08T09:00:00Z")));
    let client = JayKoaClient::new(backend);
    let dispatcher = Dispatcher::new();
    let decision = dispatcher
        .execute(
            "jaykoa.next_event",
            &Value::Null,
            "voice",
            "?",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    assert!(matches!(decision, Decision::Executed(_)));

    // Suppression du manifeste → tick retire les capabilities
    std::fs::remove_dir_all(&sub).unwrap();
    match watcher.tick(tmp.path(), &mut reg).unwrap() {
        TickOutcome::Reloaded(r) => {
            assert_eq!(r.removed, vec!["jaykoa".to_string()]);
        }
        TickOutcome::Unchanged => panic!("expected reload after manifest removal"),
    }
    assert_eq!(reg.len(), 0);

    // Le dispatcher renvoie maintenant UnknownCapability
    let err = dispatcher
        .execute(
            "jaykoa.next_event",
            &Value::Null,
            "voice",
            "?",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap_err();
    assert!(matches!(
        err,
        miyualicia_dispatcher::DispatchError::UnknownCapability(_)
    ));
}

#[tokio::test]
async fn hot_swap_idempotent_when_no_change() {
    let tmp = tempfile::tempdir().unwrap();
    let sub = tmp.path().join("jaykoa");
    std::fs::create_dir_all(&sub).unwrap();
    std::fs::write(sub.join("service.manifest.json"), real_jaykoa_manifest_bytes()).unwrap();

    let mut reg = SkillRegistry::new();
    let mut watcher = ManifestWatcher::new();

    // 1er tick → reload
    assert!(matches!(
        watcher.tick(tmp.path(), &mut reg).unwrap(),
        TickOutcome::Reloaded(_)
    ));
    // 2e tick → unchanged (mtime stable)
    assert!(matches!(
        watcher.tick(tmp.path(), &mut reg).unwrap(),
        TickOutcome::Unchanged
    ));
}

#[tokio::test]
async fn capabilities_serialize_to_openai_tools() {
    let reg = load_jaykoa_registry();
    let caps: Vec<_> = reg.iter().map(|s| s.capability.clone()).collect();
    let tools = miyualicia_skills::capabilities_to_tools(caps.iter());
    assert_eq!(tools.len(), 2);

    // Le format OpenAI doit contenir les bons noms qualifiés
    let names: Vec<&str> = tools.iter().map(|t| t.function.name.as_str()).collect();
    assert!(names.contains(&"jaykoa.list_events"));
    assert!(names.contains(&"jaykoa.next_event"));

    // Pour list_events, "from" et "to" doivent être typés "date"
    let list_tool = tools
        .iter()
        .find(|t| t.function.name == "jaykoa.list_events")
        .unwrap();
    let props = &list_tool.function.parameters["properties"];
    assert_eq!(props["from"]["type"], "string");
    assert_eq!(props["from"]["format"], "date");
    assert_eq!(props["to"]["format"], "date");
}
