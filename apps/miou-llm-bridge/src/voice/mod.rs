//! Pipeline voix MAIA — wake word → STT → LLM → TTS.
//!
//! ## Modules
//! - `tts` : Synthèse vocale via le sous-processus Piper
//! - `wakeword` : Pont IPC vers le détecteur de mot de réveil (rustpotter séparé)
//! - `pipeline` : State machine Idle → Listening → Processing → Idle

pub mod pipeline;
pub mod tts;
pub mod wakeword;

pub use pipeline::{
    ConversationTurn, PipelineCommand, PipelineController, PipelineEvent, PipelineState,
    VoicePipeline,
};
pub use tts::{PiperConfig, TtsError};
pub use wakeword::{WakeWordConfig, WakeWordEvent, WakeWordReceiver, WakeWordSource};
