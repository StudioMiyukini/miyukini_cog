//! @id     service.alicia.tests.strongfather_runtime
//! @do     tester_runtime_capacites_gouverne
//! @role   test
//! @layer  7
//! @human  Runtime capacités de production : StrongFather décide, BorderGuard alimente (K-11, K-16).

use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde_json::{json, Value};

use borderguard::TrustLevel;
use miyualicia::{CapabilitiesRuntime, CapabilitiesRuntimeConfig};
use miyualicia_cores_gate::test_helpers::simple_gate;
use miyualicia_dispatcher::{
    Decision, EnrichisseurIntention, MockServiceClient, ServiceCallResult,
};

fn repertoire_politiques() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../config/strongfather")
}

fn manifestes(dir: &Path) {
    for (id, intent, writes) in [("jaykoa", "list_events", false), ("inconnu", "lire", false)] {
        let sub = dir.join(id);
        std::fs::create_dir_all(&sub).unwrap();
        std::fs::write(
            sub.join("service.manifest.json"),
            format!(
                r#"{{"id":"{id}","alicia_capabilities":[{{"intent":"{intent}","phrases_examples":["x"],"params":{{}},"writes":{writes}}}]}}"#
            ),
        )
        .unwrap();
    }
}

async fn runtime(
    politiques: Option<&Path>,
    enrichisseurs: Vec<Arc<dyn EnrichisseurIntention>>,
) -> (
    tempfile::TempDir,
    CapabilitiesRuntime,
    Arc<MockServiceClient>,
    Arc<MockServiceClient>,
) {
    let tmp = tempfile::tempdir().unwrap();
    manifestes(tmp.path());
    let cfg = CapabilitiesRuntimeConfig {
        services_dir: Some(tmp.path().to_path_buf()),
        ..Default::default()
    };
    let rt = CapabilitiesRuntime::new_gouverne(cfg, politiques, enrichisseurs)
        .await
        .unwrap();
    let jaykoa = Arc::new(MockServiceClient::new());
    jaykoa.expect(
        "jaykoa.list_events",
        ServiceCallResult::ok(json!({"events": []})),
    );
    let inconnu = Arc::new(MockServiceClient::new());
    inconnu.expect("inconnu.lire", ServiceCallResult::ok(json!({})));
    rt.register_service_client("jaykoa", jaykoa.clone()).await;
    rt.register_service_client("inconnu", inconnu.clone()).await;
    (tmp, rt, jaykoa, inconnu)
}

#[tokio::test]
async fn politique_de_base_autorise_lecture_et_refuse_service_inconnu() {
    let (_tmp, rt, jaykoa, inconnu) = runtime(Some(&repertoire_politiques()), Vec::new()).await;

    let d = rt
        .dispatch_capability("jaykoa.list_events", &Value::Null, "voice", "agenda")
        .await
        .unwrap();
    assert!(matches!(d, Decision::Executed(_)));
    assert_eq!(jaykoa.calls().len(), 1);

    let err = rt
        .dispatch_capability("inconnu.lire", &Value::Null, "voice", "lis")
        .await
        .unwrap_err();
    assert!(format!("{err}").contains("StrongFather"), "{err}");
    assert!(
        inconnu.calls().is_empty(),
        "capability refusée ⇒ non exécutée"
    );
}

#[tokio::test]
async fn politiques_absentes_tout_est_refuse() {
    let vide = tempfile::tempdir().unwrap();
    let (_tmp, rt, jaykoa, _) = runtime(Some(vide.path()), Vec::new()).await;
    let err = rt
        .dispatch_capability("jaykoa.list_events", &Value::Null, "voice", "agenda")
        .await
        .unwrap_err();
    assert!(format!("{err}").contains("indisponible"), "{err}");
    assert!(jaykoa.calls().is_empty(), "refus ⇒ aucune exécution");
}

#[tokio::test]
async fn borderguard_refuse_le_franchissement_strongfather_refuse() {
    // voice = Verified, la frontière exige Trusted ⇒ franchissement « refuse ».
    let bg = simple_gate(
        [("jaykoa", TrustLevel::Trusted, true)],
        [("voice", TrustLevel::Verified)],
    );
    let (_tmp, rt, jaykoa, _) = runtime(Some(&repertoire_politiques()), vec![Arc::new(bg)]).await;
    let err = rt
        .dispatch_capability("jaykoa.list_events", &Value::Null, "voice", "agenda")
        .await
        .unwrap_err();
    assert!(
        format!("{err}").contains("alicia-frontiere-borderguard-refusee"),
        "{err}"
    );
    assert!(jaykoa.calls().is_empty(), "refus ⇒ aucune exécution");

    // Persona de confiance : BorderGuard autorise, StrongFather décide « allow ».
    let bg = simple_gate(
        [("jaykoa", TrustLevel::Verified, true)],
        [("voice", TrustLevel::Trusted)],
    );
    let (_tmp, rt, jaykoa, _) = runtime(Some(&repertoire_politiques()), vec![Arc::new(bg)]).await;
    let d = rt
        .dispatch_capability("jaykoa.list_events", &Value::Null, "voice", "agenda")
        .await
        .unwrap();
    assert!(matches!(d, Decision::Executed(_)));
    assert_eq!(jaykoa.calls().len(), 1);
}
