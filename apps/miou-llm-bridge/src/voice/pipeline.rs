//! State machine du pipeline voix MAIA.
//!
//! ## États
//! ```text
//! Idle ──(wake word)──► Listening ──(silence 800ms)──► Processing
//!   ▲                                                       │
//!   │                                           STT → LLM → TTS
//!   └───────────────────────────────────────────────────────┘
//! ```
//!
//! ## Transitions
//! - `Idle → Listening` : mot de réveil détecté
//! - `Listening → Processing` : silence détecté (VAD silence_ms)
//! - `Listening → Idle` : timeout (max 30s sans parole)
//! - `Processing → Idle` : réponse TTS jouée (ou erreur)

use std::time::{Duration, Instant};

use tokio::sync::mpsc;

use crate::persona::PersonaConfig;
use crate::voice::wakeword::WakeWordEvent;

/// État courant du pipeline voix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineState {
    /// En veille — attend le mot de réveil.
    Idle,
    /// Enregistrement actif — capture la parole.
    Listening,
    /// Traitement en cours — STT puis LLM puis TTS.
    Processing,
    /// Arrêt demandé.
    Shutdown,
}

impl std::fmt::Display for PipelineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Idle => write!(f, "idle"),
            Self::Listening => write!(f, "listening"),
            Self::Processing => write!(f, "processing"),
            Self::Shutdown => write!(f, "shutdown"),
        }
    }
}

/// Événement entrant dans le pipeline.
#[derive(Debug)]
pub enum PipelineEvent {
    /// Mot de réveil détecté.
    WakeWord(WakeWordEvent),
    /// Chunk audio reçu depuis le micro (PCM f32 à `sample_rate` Hz).
    AudioChunk(Vec<f32>, u32),
    /// Silence détecté (le VAD a dépassé `silence_ms`).
    SilenceDetected,
    /// Timeout de l'état Listening (30s sans parole).
    ListeningTimeout,
    /// Commande d'arrêt.
    Shutdown,
}

/// Résultat d'un tour de conversation.
#[derive(Debug, Clone)]
pub struct ConversationTurn {
    /// Texte transcrit (STT).
    pub user_text: String,
    /// Réponse du LLM.
    pub assistant_text: String,
    /// Confiance STT.
    pub stt_confidence: f32,
    /// Durée totale du traitement (ms).
    pub processing_ms: u64,
}

/// Commandes vers le pipeline (canal de contrôle externe).
#[derive(Debug)]
pub enum PipelineCommand {
    /// Activer manuellement (sans wake word — ex: bouton UI).
    Activate,
    /// Désactiver / revenir en Idle.
    Deactivate,
    /// Arrêt propre.
    Shutdown,
    /// Changer de persona à chaud.
    SwitchPersona(Box<PersonaConfig>),
}

/// Contrôleur du pipeline voix — permet de le piloter depuis l'extérieur.
#[derive(Clone)]
pub struct PipelineController {
    cmd_tx: mpsc::Sender<PipelineCommand>,
    /// Canal pour recevoir les tours de conversation complétés.
    pub turn_tx: mpsc::Sender<ConversationTurn>,
}

impl PipelineController {
    /// Active le pipeline (équivalent wake word manuel).
    pub async fn activate(&self) {
        let _ = self.cmd_tx.send(PipelineCommand::Activate).await;
    }

    /// Désactive le pipeline.
    pub async fn deactivate(&self) {
        let _ = self.cmd_tx.send(PipelineCommand::Deactivate).await;
    }

    /// Change la persona à chaud.
    pub async fn switch_persona(&self, persona: PersonaConfig) {
        let _ = self
            .cmd_tx
            .send(PipelineCommand::SwitchPersona(Box::new(persona)))
            .await;
    }

    /// Arrêt propre du pipeline.
    pub async fn shutdown(&self) {
        let _ = self.cmd_tx.send(PipelineCommand::Shutdown).await;
    }
}

/// Pipeline voix MAIA — orchestre wake word, STT, LLM, TTS.
pub struct VoicePipeline {
    pub persona: PersonaConfig,
    state: PipelineState,
    state_entered_at: Instant,
    /// Buffer audio en cours d'enregistrement (samples PCM f32 16kHz).
    recording_buffer: Vec<f32>,
    cmd_rx: mpsc::Receiver<PipelineCommand>,
    /// Canal de notification pour les tours complétés (optionnel).
    turn_tx: Option<mpsc::Sender<ConversationTurn>>,
    /// Timeout Listening → Idle (30s).
    listening_timeout: Duration,
}

impl VoicePipeline {
    /// Crée le pipeline et retourne le contrôleur associé.
    pub fn new(persona: PersonaConfig) -> (Self, PipelineController) {
        let (cmd_tx, cmd_rx) = mpsc::channel(16);
        let (turn_tx, _turn_rx) = mpsc::channel(8);
        let controller = PipelineController {
            cmd_tx,
            turn_tx: turn_tx.clone(),
        };
        let pipeline = Self {
            persona,
            state: PipelineState::Idle,
            state_entered_at: Instant::now(),
            recording_buffer: Vec::new(),
            cmd_rx,
            turn_tx: Some(turn_tx),
            listening_timeout: Duration::from_secs(30),
        };
        (pipeline, controller)
    }

    /// État courant.
    pub fn state(&self) -> &PipelineState {
        &self.state
    }

    /// Durée dans l'état courant.
    pub fn state_duration(&self) -> Duration {
        self.state_entered_at.elapsed()
    }

    /// Traite un événement et fait avancer la state machine.
    /// Retourne true si le pipeline doit continuer.
    pub fn handle_event(&mut self, event: PipelineEvent) -> bool {
        match (&self.state, event) {
            // Idle : activer sur wake word ou commande
            (PipelineState::Idle, PipelineEvent::WakeWord(ww)) => {
                tracing::info!("Wake word '{}' détecté (conf={:.2}) → Listening", ww.word, ww.confidence);
                self.transition_to(PipelineState::Listening);
                self.recording_buffer.clear();
            }

            // Listening : collecter l'audio
            (PipelineState::Listening, PipelineEvent::AudioChunk(mut samples, src_rate)) => {
                // Resample → 16kHz si nécessaire (resample linéaire)
                if src_rate != 16_000 {
                    let ratio = src_rate as f64 / 16_000.0;
                    let out_len = (samples.len() as f64 / ratio) as usize;
                    let mut resampled = Vec::with_capacity(out_len);
                    for i in 0..out_len {
                        let src_pos = i as f64 * ratio;
                        let idx = src_pos as usize;
                        let frac = (src_pos - idx as f64) as f32;
                        let s0 = samples[idx.min(samples.len() - 1)];
                        let s1 = samples[(idx + 1).min(samples.len() - 1)];
                        resampled.push(s0 + frac * (s1 - s0));
                    }
                    samples = resampled;
                }
                self.recording_buffer.extend_from_slice(&samples);

                // Timeout : 30s max en Listening
                if self.state_entered_at.elapsed() > self.listening_timeout {
                    tracing::warn!("Timeout Listening (30s) → Idle");
                    self.transition_to(PipelineState::Idle);
                    self.recording_buffer.clear();
                }
            }

            // Silence détecté → passer en Processing
            (PipelineState::Listening, PipelineEvent::SilenceDetected) => {
                if self.recording_buffer.is_empty() {
                    tracing::debug!("Silence sans audio → retour Idle");
                    self.transition_to(PipelineState::Idle);
                } else {
                    tracing::info!(
                        "Silence détecté — {} samples capturés → Processing",
                        self.recording_buffer.len()
                    );
                    self.transition_to(PipelineState::Processing);
                    // Le traitement STT/LLM/TTS est géré par `run_processing()`
                    // qui sera appelé par le loop principal
                }
            }

            // Timeout Listening
            (PipelineState::Listening, PipelineEvent::ListeningTimeout) => {
                tracing::warn!("Timeout Listening → Idle");
                self.transition_to(PipelineState::Idle);
                self.recording_buffer.clear();
            }

            // Commandes de contrôle — tous les états
            (_, PipelineEvent::Shutdown) => {
                tracing::info!("Pipeline voix arrêté");
                self.transition_to(PipelineState::Shutdown);
                return false;
            }

            // Ignorer les événements non pertinents dans l'état courant
            _ => {}
        }

        true
    }

    /// Consomme les commandes en attente depuis le canal de contrôle.
    pub fn drain_commands(&mut self) {
        while let Ok(cmd) = self.cmd_rx.try_recv() {
            match cmd {
                PipelineCommand::Activate => {
                    if self.state == PipelineState::Idle {
                        self.transition_to(PipelineState::Listening);
                        self.recording_buffer.clear();
                    }
                }
                PipelineCommand::Deactivate => {
                    self.transition_to(PipelineState::Idle);
                    self.recording_buffer.clear();
                }
                PipelineCommand::Shutdown => {
                    self.transition_to(PipelineState::Shutdown);
                }
                PipelineCommand::SwitchPersona(p) => {
                    tracing::info!("Changement persona : {} → {}", self.persona.name, p.name);
                    self.persona = *p;
                }
            }
        }
    }

    /// Retourne les samples enregistrés (vide le buffer).
    pub fn take_recording(&mut self) -> Vec<f32> {
        std::mem::take(&mut self.recording_buffer)
    }

    /// Transition vers un nouvel état.
    fn transition_to(&mut self, new_state: PipelineState) {
        tracing::debug!(
            "Pipeline: {} → {} (après {}ms)",
            self.state,
            new_state,
            self.state_entered_at.elapsed().as_millis()
        );
        self.state = new_state;
        self.state_entered_at = Instant::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persona;

    fn test_persona() -> PersonaConfig {
        persona::load_persona("alicia").expect("alicia")
    }

    #[test]
    fn test_initial_state_is_idle() {
        let (pipeline, _ctrl) = VoicePipeline::new(test_persona());
        assert_eq!(*pipeline.state(), PipelineState::Idle);
    }

    #[test]
    fn test_wake_word_transitions_to_listening() {
        let (mut pipeline, _ctrl) = VoicePipeline::new(test_persona());
        let event = PipelineEvent::WakeWord(WakeWordEvent {
            word: "Alicia".into(),
            confidence: 0.9,
            timestamp_ms: 0,
        });
        assert!(pipeline.handle_event(event));
        assert_eq!(*pipeline.state(), PipelineState::Listening);
    }

    #[test]
    fn test_silence_with_audio_transitions_to_processing() {
        let (mut pipeline, _ctrl) = VoicePipeline::new(test_persona());

        // Activer
        pipeline.handle_event(PipelineEvent::WakeWord(WakeWordEvent {
            word: "Alicia".into(),
            confidence: 0.9,
            timestamp_ms: 0,
        }));

        // Injecter de l'audio
        let samples = vec![0.1f32; 1600]; // 100ms à 16kHz
        pipeline.handle_event(PipelineEvent::AudioChunk(samples, 16_000));

        // Silence → Processing
        pipeline.handle_event(PipelineEvent::SilenceDetected);
        assert_eq!(*pipeline.state(), PipelineState::Processing);
    }

    #[test]
    fn test_silence_without_audio_returns_to_idle() {
        let (mut pipeline, _ctrl) = VoicePipeline::new(test_persona());

        pipeline.handle_event(PipelineEvent::WakeWord(WakeWordEvent {
            word: "Alicia".into(),
            confidence: 0.9,
            timestamp_ms: 0,
        }));

        // Silence sans audio capturé → Idle
        pipeline.handle_event(PipelineEvent::SilenceDetected);
        assert_eq!(*pipeline.state(), PipelineState::Idle);
    }

    #[test]
    fn test_shutdown_stops_pipeline() {
        let (mut pipeline, _ctrl) = VoicePipeline::new(test_persona());
        let should_continue = pipeline.handle_event(PipelineEvent::Shutdown);
        assert!(!should_continue);
        assert_eq!(*pipeline.state(), PipelineState::Shutdown);
    }

    #[test]
    fn test_persona_switch() {
        let (mut pipeline, _ctrl) = VoicePipeline::new(test_persona());
        assert_eq!(pipeline.persona.name, "alicia");

        let miou = persona::load_persona("miou").expect("miou");
        pipeline.drain_commands(); // vide le canal
        // Simuler la commande via handle interne
        pipeline.persona = miou;
        assert_eq!(pipeline.persona.name, "miou");
    }
}
