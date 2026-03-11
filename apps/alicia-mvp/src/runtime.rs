//! Runtime partage entre tous les modes d'Alicia (text, voice, listen).
//!
//! `AliciaRuntime` encapsule :
//! - le moteur NLU
//! - le service Alicia (devices, intents)
//! - le client HTTP
//! - les URLs STT / TTS resolues (mock ou externes)
//! - la face_url optionnelle
//!
//! Une seule fonction `AliciaRuntime::build()` gere la resolution des URLs
//! et le demarrage eventuel des serveurs mock.

use std::path::PathBuf;
use std::sync::Arc;

use miyualicia::config::AliciaConfig;
use miyualicia::intent::Intent;
use miyualicia::AliciaService;
use miyualicia_devices::CommandSource;

use crate::demo;
use crate::nlu::NluEngine;
use crate::voice;

/// Arguments communs a tous les modes connectes (voice, listen).
#[derive(Debug, Clone)]
pub struct ServiceArgs {
    pub stt_url: String,
    pub tts_url: String,
    pub llm_url: Option<String>,
    pub llm_key: Option<String>,
    pub face_url: Option<String>,
    pub search_url: Option<String>,
    pub workspace: Option<PathBuf>,
}

/// Runtime partage initialise une seule fois.
pub struct AliciaRuntime {
    pub nlu: NluEngine,
    pub service: Arc<AliciaService>,
    pub http: reqwest::Client,
    pub stt_url: String,
    pub tts_url: String,
    pub face_url: Option<String>,
}

impl AliciaRuntime {
    /// Construit le runtime : resout les URLs STT/TTS (demarre les mocks si vide),
    /// cree le service Alicia et enregistre les devices demo.
    pub async fn build(args: ServiceArgs) -> anyhow::Result<Self> {
        let nlu = NluEngine::new(args.llm_url, args.llm_key, args.search_url, args.workspace);

        let (stt_url, tts_url) = resolve_urls(args.stt_url, args.tts_url).await?;

        let config = AliciaConfig {
            mqtt: None,
            stt_service_url: stt_url.clone(),
            llm_bridge_url: stt_url.clone(),
            tts_service_url: tts_url.clone(),
            tts_feature_enabled: true,
            ..Default::default()
        };
        let service = Arc::new(AliciaService::new(config));
        demo::register_demo_devices(&service).await;

        Ok(Self {
            nlu,
            service,
            http: reqwest::Client::new(),
            stt_url,
            tts_url,
            face_url: args.face_url,
        })
    }

    /// Construit un runtime leger pour le mode texte (pas de STT/TTS).
    pub async fn build_text(
        llm_url: Option<String>,
        llm_key: Option<String>,
        search_url: Option<String>,
        workspace: Option<PathBuf>,
    ) -> Self {
        let nlu = NluEngine::new(llm_url, llm_key, search_url, workspace);
        let service = Arc::new(demo::build_demo_service().await);
        Self {
            nlu,
            service,
            http: reqwest::Client::new(),
            stt_url: String::new(),
            tts_url: String::new(),
            face_url: None,
        }
    }

    /// Dispatche un intent : devices, launch_app, face recognition.
    pub async fn dispatch(&self, intent: &Intent, transcript: &str) {
        let _ = transcript;
        if let Intent::Unknown { transcript: ref t } = intent {
            if let Some(app_name) = t.strip_prefix("__launch_app:") {
                match demo::launch_app(app_name) {
                    Ok(()) => println!("  [Launch] Application '{app_name}' lancee"),
                    Err(e) => println!("  [Launch] Erreur : {e}"),
                }
            } else if t == "__identify_face" {
                if let Some(ref face_base) = self.face_url {
                    match demo::call_face_identify(&self.http, face_base).await {
                        Ok(msg) => println!("  [Face] {msg}"),
                        Err(e) => println!("  [Face] Erreur : {e}"),
                    }
                } else {
                    println!("  [Face] Pas de serveur face configure (--face-url)");
                }
            } else if let Some(name) = t.strip_prefix("__register_face:") {
                if let Some(ref face_base) = self.face_url {
                    match demo::call_face_register(&self.http, face_base, name).await {
                        Ok(msg) => println!("  [Face] {msg}"),
                        Err(e) => println!("  [Face] Erreur : {e}"),
                    }
                } else {
                    println!("  [Face] Pas de serveur face configure (--face-url)");
                }
            }
        } else if intent.is_actionable() {
            match self.service.dispatch_intent(intent, CommandSource::Voice).await {
                Ok(Some(latency)) => println!("  [Dispatch] OK en {latency}ms"),
                Ok(None) => println!("  [Dispatch] Traite (pas de device)"),
                Err(e) => println!("  [Dispatch] Erreur : {e} (normal sans vrais devices)"),
            }
        }
    }
}

/// Resout les URLs STT et TTS : demarre un unique serveur mock si les deux sont vides,
/// ou demarre le mock uniquement pour celle qui est vide.
async fn resolve_urls(stt_url: String, tts_url: String) -> anyhow::Result<(String, String)> {
    match (stt_url.is_empty(), tts_url.is_empty()) {
        (false, false) => Ok((stt_url, tts_url)),
        (true, true) => {
            let (stt, tts) = voice::start_mock_servers().await?;
            println!("  STT mock : {stt}");
            println!("  TTS mock : {tts}");
            Ok((stt, tts))
        }
        (true, false) => {
            let (stt, _) = voice::start_mock_servers().await?;
            println!("  STT mock : {stt}");
            Ok((stt, tts_url))
        }
        (false, true) => {
            let (_, tts) = voice::start_mock_servers().await?;
            println!("  TTS mock : {tts}");
            Ok((stt_url, tts))
        }
    }
}
