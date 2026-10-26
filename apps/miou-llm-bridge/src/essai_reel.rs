//! @id     miou_llm_bridge_essai_reel
//! @do     tester_maia_de_bout_en_bout
//! @role   test
//! @layer  app
//! @human  MAIA — essais d'intégration avec un vrai modèle : maia-llm démarré avec le modèle signé, réponse de bout en bout ; moteur hors COG refusé par la politique de production.

//! Essais d'intégration réels (ignorés par défaut, exigent des fichiers hors dépôt).
//!
//! **maia-llm + modèle signé** (chemin de production) :
//!
//! ```text
//! cargo build -p maia-llm --no-default-features --release
//! MAIA_LLM_BIN=/srv/app/mws-target/release/maia-llm \
//! MAIA_MODELES_DIR=/srv/app/models/canal \
//! MAIA_MODELE_ANCRE=<clé publique hex du signataire du canal> \
//!   cargo test -p maia maia_repond_avec_le_vrai_modele -- --ignored --nocapture
//! ```
//!
//! **Moteur d'essai hors COG** (ex. koboldcpp, essais seulement) :
//!
//! ```text
//! MAIA_MOTEUR_ESSAI_URL=http://127.0.0.1:5001 \
//!   cargo test -p maia moteur_d_essai_seulement_sous_politique_d_essai -- --ignored --nocapture
//! ```

use std::io::Write as _;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::body::Body;
use axum::extract::ConnectInfo;
use axum::http::{header, Request, StatusCode};
use miyauth::http::service::{encoder_cle_publique, entetes_signees};
use miyusceau::CleSignature;
use tower::ServiceExt;

use crate::agents::AgentRegistry;
use crate::config::SecurityConfig;
use crate::context::ContextRegistry;
use crate::gouvernance::GouvernanceMaia;
use crate::inference::{InferenceRouter, MaiaLlmClient, UpstreamBackend};
use crate::mode::MaiaMode;
use crate::model_manager::ModelManager;
use crate::proxy::{proxy_router, ProxyState};
use crate::security::{AuditLog, RateLimitConfig, RateLimiter, SecurityAuditLog};
use crate::signature::{GardesMaia, USAGE_API};

/// Politiques de production de MAIA (dépôt).
fn politiques_production() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../config/strongfather/maia.toml")
}

fn etat(
    inference: InferenceRouter,
    appelant: &CleSignature,
    gouvernance: GouvernanceMaia,
) -> ProxyState {
    let client = reqwest::Client::new();
    ProxyState {
        client,
        upstream_url: String::new(),
        gardes: Arc::new(GardesMaia::new(vec![appelant.cle_publique()], vec![])),
        log_requests: false,
        hardware: crate::hardware::detect_hardware(),
        agent_registry: AgentRegistry::new(),
        context_registry: ContextRegistry::new(),
        rate_limiter: RateLimiter::new(RateLimitConfig::default()),
        audit_log: AuditLog::new(100),
        inference,
        model_manager: ModelManager::new(std::env::temp_dir().join("maia-essai-modeles")),
        security_config: SecurityConfig::default(),
        security_audit: SecurityAuditLog::new(100),
        active_persona: None,
        maia_mode: MaiaMode::Standalone,
        started_at: 0,
        team_registered: false,
        confirmations: Arc::new(crate::confirmations::RegistreConfirmations::new()),
        gouvernance: Arc::new(gouvernance),
    }
}

/// `POST /v1/agents/miou/chat` signé, sans outils ni mémoire ; réponse JSON.
async fn demander(app: axum::Router, appelant: &CleSignature, message: &str) -> serde_json::Value {
    let chemin = "/v1/agents/miou/chat";
    let corps = serde_json::json!({ "message": message, "enable_tools": false }).to_string();
    let mut b = Request::builder()
        .method("POST")
        .uri(chemin)
        .header(header::CONTENT_TYPE, "application/json");
    for (n, v) in entetes_signees(appelant, USAGE_API, "POST", chemin, corps.as_bytes()) {
        b = b.header(n, v);
    }
    let mut r = b.body(Body::from(corps)).expect("requête");
    r.extensions_mut()
        .insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 40000))));
    let rep = app.oneshot(r).await.expect("réponse");
    assert_eq!(rep.status(), StatusCode::OK);
    let octets = axum::body::to_bytes(rep.into_body(), 1 << 20)
        .await
        .expect("corps");
    serde_json::from_slice(&octets).expect("JSON")
}

/// Processus maia-llm arrêté à la fin du test.
struct Processus(std::process::Child);

impl Drop for Processus {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn port_libre() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0")
        .and_then(|l| l.local_addr())
        .map(|a| a.port())
        .expect("port libre")
}

fn ecrire_config(dossier: &Path, port: u16, modeles: &str, ancre: &str, cle_maia: &str) {
    let toml = format!(
        "bind_addr = \"127.0.0.1:{port}\"\nmodels_dir = \"{modeles}\"\ngpu_layers = 0\n\
         context_size = 4096\nauto_load = true\nappelants_inference = [\"{cle_maia}\"]\n\
         appelants_modeles = [\"{cle_maia}\"]\nmodel_trust_anchors = [\"{ancre}\"]\n"
    );
    let mut f = std::fs::File::create(dossier.join("maia-llm.toml")).expect("config");
    f.write_all(toml.as_bytes()).expect("écriture config");
}

#[tokio::test]
#[ignore = "exige MAIA_LLM_BIN, MAIA_MODELES_DIR (modèle signé) et MAIA_MODELE_ANCRE"]
async fn maia_repond_avec_le_vrai_modele() {
    let bin = std::env::var("MAIA_LLM_BIN").expect("MAIA_LLM_BIN");
    let modeles = std::env::var("MAIA_MODELES_DIR").expect("MAIA_MODELES_DIR");
    let ancre = std::env::var("MAIA_MODELE_ANCRE").expect("MAIA_MODELE_ANCRE");

    let donnees = std::env::temp_dir().join(format!("maia-essai-reel-{}", std::process::id()));
    std::fs::create_dir_all(&donnees).expect("dossier");
    let cle_maia = Arc::new(CleSignature::generer());
    let port = port_libre();
    ecrire_config(
        &donnees,
        port,
        &modeles,
        &ancre,
        &encoder_cle_publique(&cle_maia.cle_publique()),
    );
    // Premier plan, arrêté à la fin du test (aucun service installé).
    let _maia_llm = Processus(
        std::process::Command::new(&bin)
            .env("MAIA_DATA_DIR", &donnees)
            .env("RUST_LOG", "maia_llm=warn")
            .spawn()
            .expect("lancement de maia-llm"),
    );

    let url = format!("http://127.0.0.1:{port}");
    let client = MaiaLlmClient::new(url, Some(cle_maia), reqwest::Client::new());
    let debut = Instant::now();
    while !(client.is_available().await && client.is_loaded()) {
        assert!(
            debut.elapsed() < Duration::from_secs(180),
            "maia-llm n'a pas chargé le modèle"
        );
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    println!(
        "maia-llm prêt en {} ms : {:?}",
        debut.elapsed().as_millis(),
        client.loaded_model_name()
    );

    // Un moteur hors COG configuré est refusé par la politique de production.
    let gouvernance = GouvernanceMaia::depuis_fichier(politiques_production());
    assert!(gouvernance.est_operationnelle());
    assert!(!gouvernance.autoriser_inference_externe("http://127.0.0.1:5001", "production"));
    let amont = UpstreamBackend::new(reqwest::Client::new(), "http://127.0.0.1:5001".into())
        .autoriser(gouvernance.autoriser_inference_externe("http://127.0.0.1:5001", "production"));
    assert!(!amont.est_autorise());

    let appelant = CleSignature::generer();
    let app = proxy_router(etat(
        InferenceRouter::new(client, amont, true),
        &appelant,
        gouvernance,
    ));

    let t = Instant::now();
    let v = demander(
        app.clone(),
        &appelant,
        "Bonjour ! Qui es-tu, en une phrase ?",
    )
    .await;
    println!("réponse en {} ms : {v}", t.elapsed().as_millis());
    assert_eq!(v["fallback"], false, "{v}");
    assert_eq!(v["backend"], "native", "{v}");
    assert_eq!(v["genere_par_ia"], true);
    assert!(v["mention_ia"].as_str().unwrap_or("").contains("IA"));
    let contenu = v["content"].as_str().unwrap_or("");
    assert!(contenu.chars().count() > 3, "réponse vide : {v}");

    let _ = std::fs::remove_dir_all(&donnees);
}

#[tokio::test]
#[ignore = "exige MAIA_MOTEUR_ESSAI_URL (moteur compatible OpenAI en boucle locale, essais seulement)"]
async fn moteur_d_essai_seulement_sous_politique_d_essai() {
    let url = std::env::var("MAIA_MOTEUR_ESSAI_URL").expect("MAIA_MOTEUR_ESSAI_URL");
    // Production : refus.
    let production = GouvernanceMaia::depuis_fichier(politiques_production());
    assert!(!production.autoriser_inference_externe(&url, "production"));
    // Politique d'essai (jamais livrée) : moteur en boucle locale, contexte « essai ».
    let politique_essai =
        std::env::temp_dir().join(format!("maia-essai-{}.toml", std::process::id()));
    std::fs::write(
        &politique_essai,
        "version = 1\n[[policy]]\nid = \"essai-moteur-local\"\ntype = \"permission\"\n\
         effect = \"authorize\"\ncondition = \"data.operation == 'inference.externe'\"\n\
         rule = \"data.contexte == 'essai' && data.loopback == 'true'\"\n\
         [policy.target]\nactions = [\"read\"]\norigins = [\"maia\"]\n",
    )
    .expect("politique d'essai");
    let essai = GouvernanceMaia::depuis_fichier(&politique_essai);
    assert!(!essai.autoriser_inference_externe(&url, "production"));
    assert!(!essai.autoriser_inference_externe("https://api.exemple.com", "essai"));
    let autorise = essai.autoriser_inference_externe(&url, "essai");
    assert!(autorise);
    let amont = UpstreamBackend::new(reqwest::Client::new(), url).autoriser(autorise);
    // Aucun maia-llm : seul le moteur d'essai répond.
    let natif = MaiaLlmClient::new("http://127.0.0.1:9".into(), None, reqwest::Client::new());
    let appelant = CleSignature::generer();
    let app = proxy_router(etat(
        InferenceRouter::new(natif, amont, true),
        &appelant,
        essai,
    ));
    let t = Instant::now();
    let v = demander(app, &appelant, "Bonjour ! Qui es-tu, en une phrase ?").await;
    println!(
        "réponse (moteur d'essai) en {} ms : {v}",
        t.elapsed().as_millis()
    );
    assert_eq!(v["backend"], "upstream", "{v}");
    assert_eq!(v["genere_par_ia"], true);
    assert!(
        v["content"].as_str().unwrap_or("").chars().count() > 3,
        "{v}"
    );
    let _ = std::fs::remove_file(&politique_essai);
}

#[test]
fn politique_de_production_refuse_tout_moteur_hors_cog() {
    let g = GouvernanceMaia::depuis_fichier(politiques_production());
    assert!(g.est_operationnelle(), "maia.toml doit se charger");
    for url in [
        "http://127.0.0.1:5001",
        "http://localhost:1234",
        "https://api.exemple.com",
    ] {
        assert!(!g.autoriser_inference_externe(url, "production"), "{url}");
        assert!(!g.autoriser_inference_externe(url, "essai"), "{url}");
    }
    assert!(!g.autoriser_inference_externe("", "essai"));
}
