//! Alicia MVP — test harness pour le parcours STT -> NLU -> TTS.
//!
//! Modes :
//! - `text`           : saisie clavier -> NLU -> reponse
//! - `voice`          : micro -> STT -> NLU -> TTS -> haut-parleur
//! - `listen`         : ecoute permanente avec wake word "Alicia"
//! - `train-wake-word`: enregistre des samples pour creer un modele .rpw
//!
//! Usage :
//!   cargo run -p alicia-mvp -- text --llm-url http://127.0.0.1:1234
//!   cargo run -p alicia-mvp -- voice --stt-url http://127.0.0.1:8100
//!   cargo run -p alicia-mvp -- listen --model models/hey_alicia.rpw
//!   cargo run -p alicia-mvp -- train-wake-word --output models/hey_alicia.rpw

use std::io::{self, BufRead, Write as _};
use std::path::PathBuf;
use std::time::Instant;

use clap::{Args, Parser, Subcommand};

mod demo;
mod listen;
mod mcp;
mod nlu;
mod providers;
mod runtime;
mod scheduler;
mod shell;
mod skills;
mod tools;
mod train;
mod voice;
#[cfg(feature = "whisper-cuda")]
mod stt_whisper;

use runtime::{AliciaRuntime, ServiceArgs};

// ---------------------------------------------------------------------------
// CLI
// ---------------------------------------------------------------------------

/// Arguments communs aux modes connectes (voice, listen).
#[derive(Args, Clone, Debug)]
pub struct ConnectedArgs {
    /// URL du serveur STT. Defaut: serveur mock embarque.
    #[arg(long, default_value = "")]
    pub stt_url: String,
    /// URL du serveur TTS. Defaut: serveur mock embarque.
    #[arg(long, default_value = "")]
    pub tts_url: String,
    /// URL du LLM (LM Studio / Ollama). Si absent : fallback regex.
    #[arg(long)]
    pub llm_url: Option<String>,
    /// Cle API du LLM. Optionnel.
    #[arg(long)]
    pub llm_key: Option<String>,
    /// URL du serveur Face Recognition. Optionnel.
    #[arg(long)]
    pub face_url: Option<String>,
    /// URL du serveur de recherche web. Optionnel.
    #[arg(long)]
    pub search_url: Option<String>,
    /// Repertoire de travail pour les outils fichiers/DB. Optionnel.
    #[arg(long)]
    pub workspace: Option<PathBuf>,
}

impl From<ConnectedArgs> for ServiceArgs {
    fn from(a: ConnectedArgs) -> Self {
        Self {
            stt_url: a.stt_url,
            tts_url: a.tts_url,
            llm_url: a.llm_url,
            llm_key: a.llm_key,
            face_url: a.face_url,
            search_url: a.search_url,
            workspace: a.workspace,
        }
    }
}

#[derive(Parser)]
#[command(name = "alicia-mvp", about = "MVP test du parcours STT -> Alicia -> TTS")]
struct Cli {
    #[command(subcommand)]
    mode: Mode,
}

#[derive(Subcommand)]
enum Mode {
    /// Mode texte : saisie clavier -> NLU -> intent -> reponse
    Text {
        #[arg(long)] llm_url: Option<String>,
        #[arg(long)] llm_key: Option<String>,
        #[arg(long)] search_url: Option<String>,
        #[arg(long)] workspace: Option<PathBuf>,
    },
    /// Mode voix : micro -> STT -> NLU -> TTS -> haut-parleur
    Voice {
        #[command(flatten)]
        conn: ConnectedArgs,
    },
    /// Mode ecoute permanente : wake word "Alicia" -> commande -> STT -> NLU -> TTS
    Listen {
        #[command(flatten)]
        conn: ConnectedArgs,
        /// Chemin vers le modele wake word (.rpw ou .rpwm). Defaut: models/hey_alicia.rpw
        #[arg(long)]
        model: Option<String>,
        /// Seuil de confiance du wake word [0.0, 1.0]. Defaut: 0.5
        #[arg(long)]
        threshold: Option<f32>,
        /// ID HuggingFace du modele Whisper local pour STT GPU
        /// ex: openai/whisper-small, openai/whisper-medium (feature whisper-cuda requise)
        #[arg(long)]
        whisper_model: Option<String>,
        /// Langue STT pour le moteur Whisper local
        #[arg(long, default_value = "fr")]
        whisper_lang: String,
    },
    /// Entrainement du wake word : enregistre des echantillons et cree un modele .rpw
    TrainWakeWord {
        #[arg(long, default_value = "models/hey_alicia.rpw")] output: PathBuf,
        #[arg(long, default_value = "5")] samples: usize,
        #[arg(long)] threshold: Option<f32>,
    },
}

// ---------------------------------------------------------------------------
// Entrypoint
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,alicia_mvp=debug".parse().unwrap()),
        )
        .init();

    match Cli::parse().mode {
        Mode::Text { llm_url, llm_key, search_url, workspace } => {
            run_text_mode(llm_url, llm_key, search_url, workspace).await
        }
        Mode::Voice { conn } => {
            let a = ServiceArgs::from(conn);
            voice::run_voice_mode(a.stt_url, a.tts_url, a.llm_url, a.llm_key, a.face_url, a.search_url, a.workspace).await
        }
        Mode::Listen { conn, model, threshold, whisper_model, whisper_lang } => {
            let a = ServiceArgs::from(conn);
            listen::run_listen_mode(a.stt_url, a.tts_url, a.llm_url, a.llm_key, a.face_url, a.search_url, a.workspace, model, threshold, whisper_model, whisper_lang).await
        }
        Mode::TrainWakeWord { output, samples, threshold } => {
            train::run_train_wake_word(output, samples, threshold).await
        }
    }
}

// ---------------------------------------------------------------------------
// Mode texte
// ---------------------------------------------------------------------------

async fn run_text_mode(
    llm_url: Option<String>,
    llm_key: Option<String>,
    search_url: Option<String>,
    workspace: Option<PathBuf>,
) -> anyhow::Result<()> {
    let rt = AliciaRuntime::build_text(llm_url, llm_key, search_url, workspace).await;

    println!("=== Alicia MVP — Mode Texte [NLU: {}] ===", rt.nlu.label());
    println!("Tape une commande (ex: \"allume la lumiere du salon\"), 'quit' pour quitter.\n");

    let stdin = io::stdin();
    loop {
        print!("alicia> ");
        io::stdout().flush()?;

        let mut line = String::new();
        if stdin.lock().read_line(&mut line)? == 0 { break; }
        let line = line.trim();
        if line.is_empty() { continue; }
        if line == "quit" || line == "exit" { println!("Au revoir !"); break; }

        let start = Instant::now();
        let (intent, raw_nlu) = rt.nlu.parse(line).await;
        println!("\n  [NLU-{}] {} → {:?} ({:?})", rt.nlu.label(), raw_nlu, intent, start.elapsed());

        let response = rt.nlu.generate_response(&intent, line).await;
        println!("  [Alicia] {response}");

        rt.dispatch(&intent, line).await;
        println!();
    }

    Ok(())
}
