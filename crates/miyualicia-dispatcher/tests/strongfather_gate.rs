//! @id     service.alicia.dispatcher.tests.strongfather_gate
//! @do     tester_gate_strongfather_alicia
//! @role   test
//! @layer  7
//! @human  La gate cores de production d'Alicia décide via StrongFather (K-11, K-16).
//!
//! - (a) une capability refusée par la politique n'est pas exécutée (service non appelé)
//! - (b) la politique de base `config/strongfather/alicia.toml` se charge et autorise
//!   les cas légitimes (lecture, écriture confirmée vers un service COG connu)
//! - (c) fichier absent ou invalide ⇒ refus de tout
//! - BorderGuard alimente l'intention ; un franchissement refusé ⇒ refus SF

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

use serde_json::{json, Value};

use miyualicia_dispatcher::core_gate::{
    chemin_politiques_alicia, EnrichisseurIntention, StrongFatherCoreGate,
};
use miyualicia_dispatcher::{
    CoreGate, CoreGateOutcome, Decision, DispatchError, Dispatcher, MockServiceClient,
    ServiceCallResult,
};
use miyualicia_memory::MemoryStore;
use miyualicia_skills::{
    AliciaPolicy, ConfirmationPolicy, ServiceCapability, Skill, SkillRegistry,
};

fn politiques_de_base() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../config/strongfather")
        .join("alicia.toml")
}

fn cap(service: &str, intent: &str, writes: bool) -> ServiceCapability {
    ServiceCapability {
        service_id: service.into(),
        intent: intent.into(),
        phrases_examples: vec![],
        params: BTreeMap::new(),
        writes,
        confirmation_default: if writes {
            ConfirmationPolicy::AlwaysForWrites
        } else {
            ConfirmationPolicy::Never
        },
        description: None,
        category: None,
    }
}

fn registre(caps: Vec<ServiceCapability>) -> SkillRegistry {
    let mut reg = SkillRegistry::new();
    for c in caps {
        reg.register(Skill {
            capability: c,
            source: "test".into(),
            registered_at: "2026-10-24T00:00:00Z".into(),
        })
        .unwrap();
    }
    reg
}

fn gate_de_base() -> StrongFatherCoreGate {
    let gate = StrongFatherCoreGate::depuis_fichier(politiques_de_base());
    assert!(gate.est_operationnelle(), "alicia.toml doit se charger");
    gate
}

#[test]
fn chemin_par_defaut_et_repertoire_explicite() {
    assert_eq!(
        chemin_politiques_alicia(Some(std::path::Path::new("/etc/cog"))),
        PathBuf::from("/etc/cog/alicia.toml")
    );
    assert!(chemin_politiques_alicia(None).ends_with("alicia.toml"));
}

#[tokio::test]
async fn politique_de_base_autorise_les_cas_legitimes() {
    let gate = gate_de_base();
    let legitimes = [
        cap("jaykoa", "list_events", false),
        cap("jaykonta", "get_balance", false),
        cap("jaymeteo", "query_forecast", false),
        cap("jaykonta", "add_movement", true),
        cap("jaymail", "send_mail", true),
        cap("jayrdv", "book_appointment", true),
    ];
    for c in legitimes {
        assert_eq!(
            gate.validate(&c, &Value::Null, "voice").await,
            CoreGateOutcome::Allowed,
            "{}",
            c.qualified_id()
        );
    }
}

#[tokio::test]
async fn politique_de_base_refuse_le_reste() {
    let gate = gate_de_base();
    for c in [
        cap("service_inconnu", "lire", false),
        cap("service_inconnu", "ecrire", true),
        // jaymeteo n'expose aucune écriture : refusée par défaut.
        cap("jaymeteo", "set_station", true),
    ] {
        match gate.validate(&c, &Value::Null, "voice").await {
            CoreGateOutcome::Denied(m) => assert!(m.contains("StrongFather"), "{m}"),
            CoreGateOutcome::Allowed => panic!("{} aurait dû être refusée", c.qualified_id()),
        }
    }
}

#[tokio::test]
async fn capability_refusee_jamais_executee() {
    let reg = registre(vec![cap("service_inconnu", "lire", false)]);
    let mem = MemoryStore::open_in_memory().unwrap();
    let svc = MockServiceClient::new();
    svc.expect("service_inconnu.lire", ServiceCallResult::ok(json!({})));

    let err = Dispatcher::new()
        .execute(
            "service_inconnu.lire",
            &Value::Null,
            "voice",
            "lis",
            &reg,
            &AliciaPolicy::strict(),
            &gate_de_base(),
            &svc,
            &mem,
        )
        .await
        .unwrap_err();
    assert!(matches!(err, DispatchError::CoresDenied(_)));
    assert!(svc.calls().is_empty(), "le service ne doit pas être appelé");
}

#[tokio::test]
async fn capability_autorisee_executee_via_dispatcher() {
    let reg = registre(vec![cap("jaykoa", "list_events", false)]);
    let mem = MemoryStore::open_in_memory().unwrap();
    let svc = MockServiceClient::new();
    svc.expect(
        "jaykoa.list_events",
        ServiceCallResult::ok(json!({"events": []})),
    );

    let d = Dispatcher::new()
        .execute(
            "jaykoa.list_events",
            &Value::Null,
            "voice",
            "mon agenda",
            &reg,
            &AliciaPolicy::strict(),
            &gate_de_base(),
            &svc,
            &mem,
        )
        .await
        .unwrap();
    assert!(matches!(d, Decision::Executed(_)));
    assert_eq!(svc.calls().len(), 1);
}

#[tokio::test]
async fn fichier_absent_ou_invalide_refuse_tout() {
    let dir = tempfile::tempdir().unwrap();
    let absent = StrongFatherCoreGate::depuis_fichier(dir.path().join("alicia.toml"));
    assert!(!absent.est_operationnelle());

    let invalide_chemin = dir.path().join("invalide.toml");
    std::fs::write(&invalide_chemin, "version = 1\n[[policy]]\nid = \"x\"\n").unwrap();
    let invalide = StrongFatherCoreGate::depuis_fichier(&invalide_chemin);
    assert!(!invalide.est_operationnelle());

    let vide_chemin = dir.path().join("vide.toml");
    std::fs::write(&vide_chemin, "version = 1\n").unwrap();
    let vide = StrongFatherCoreGate::depuis_fichier(&vide_chemin);
    assert!(!vide.est_operationnelle());

    let c = cap("jaykoa", "list_events", false);
    for gate in [absent, invalide, vide] {
        match gate.validate(&c, &Value::Null, "voice").await {
            CoreGateOutcome::Denied(m) => assert!(m.contains("indisponible"), "{m}"),
            CoreGateOutcome::Allowed => panic!("politiques indisponibles ⇒ refus"),
        }
    }

    // Et via le dispatcher : rien n'est exécuté.
    let reg = registre(vec![c]);
    let mem = MemoryStore::open_in_memory().unwrap();
    let svc = MockServiceClient::new();
    let err = Dispatcher::new()
        .execute(
            "jaykoa.list_events",
            &Value::Null,
            "voice",
            "agenda",
            &reg,
            &AliciaPolicy::strict(),
            &StrongFatherCoreGate::depuis_fichier(dir.path().join("absent.toml")),
            &svc,
            &mem,
        )
        .await
        .unwrap_err();
    assert!(matches!(err, DispatchError::CoresDenied(_)));
    assert!(svc.calls().is_empty(), "refus ⇒ aucune exécution");
}

struct FranchissementBorderGuard(&'static str);

impl EnrichisseurIntention for FranchissementBorderGuard {
    fn donnees(&self, _capability: &ServiceCapability, _persona: &str) -> Vec<(String, String)> {
        vec![
            ("borderguard_franchissement".into(), self.0.into()),
            // Une source tierce ne peut pas écraser une clé canonique.
            ("service".into(), "jaykoa".into()),
        ]
    }
}

#[tokio::test]
async fn borderguard_alimente_mais_strongfather_decide() {
    let c = cap("jaykoa", "list_events", false);
    let refuse = gate_de_base().avec_enrichisseur(Arc::new(FranchissementBorderGuard("refuse")));
    match refuse.validate(&c, &Value::Null, "voice").await {
        CoreGateOutcome::Denied(m) => assert!(m.contains("alicia-frontiere-borderguard-refusee")),
        CoreGateOutcome::Allowed => panic!("franchissement refusé ⇒ refus SF"),
    }
    let autorise =
        gate_de_base().avec_enrichisseur(Arc::new(FranchissementBorderGuard("autorise")));
    assert_eq!(
        autorise.validate(&c, &Value::Null, "voice").await,
        CoreGateOutcome::Allowed
    );

    // Clé canonique protégée : un service inconnu reste inconnu.
    let inconnu = cap("service_inconnu", "lire", false);
    let intent = autorise.intention(&inconnu, "voice");
    assert_eq!(
        intent.data.content.get("service").map(String::as_str),
        Some("service_inconnu")
    );
    assert!(matches!(
        autorise.validate(&inconnu, &Value::Null, "voice").await,
        CoreGateOutcome::Denied(_)
    ));
}

#[test]
fn forme_de_l_intention() {
    let intent = gate_de_base().intention(&cap("JayKoa", "List_Events", true), "voice");
    assert_eq!(intent.subject, "alicia.skill.jaykoa.list_events");
    assert_eq!(intent.action_type.as_str(), "modification");
    assert_eq!(intent.call_context.caller_identity, "local");
    assert_eq!(intent.call_context.origin, "alicia");
    for k in [
        "service",
        "intent",
        "skill",
        "ecriture",
        "persona",
        "confirmation",
        "categorie",
    ] {
        assert!(intent.data.content.contains_key(k), "{k}");
    }
    assert_eq!(intent.data.content["ecriture"], "true");
    assert_eq!(intent.data.content["confirmation"], "always_for_writes");
}
