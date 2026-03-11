//! Pont IPC vers le détecteur de mot de réveil (rustpotter).
//!
//! rustpotter est dans un binaire/service séparé (`miyualicia-wakeword`) pour
//! éviter le conflit de dépendance :
//!   rustpotter 3.0 requiert half ^2.5
//!   candle-core (whisper-rs feature) requiert half =2.4.1
//!
//! Ce module gère :
//! 1. Un channel tokio mpsc pour recevoir les détections de wake word.
//! 2. Un thread OS qui tourne dans le binaire miyualicia-wakeword (optionnel).
//! 3. Une API simple : `WakeWordReceiver::wait().await` → bloque jusqu'à une détection.

use tokio::sync::mpsc;

/// Événement de détection de mot de réveil.
#[derive(Debug, Clone)]
pub struct WakeWordEvent {
    /// Mot détecté.
    pub word: String,
    /// Score de confiance (0.0–1.0).
    pub confidence: f32,
    /// Timestamp de détection (epoch ms).
    pub timestamp_ms: u64,
}

/// Récepteur d'événements wake word.
///
/// Créé par `WakeWordSource::spawn()` ou `WakeWordSource::mock()` pour les tests.
pub struct WakeWordReceiver {
    rx: mpsc::Receiver<WakeWordEvent>,
}

impl WakeWordReceiver {
    /// Attend le prochain événement wake word (bloque jusqu'à la détection).
    pub async fn wait(&mut self) -> Option<WakeWordEvent> {
        self.rx.recv().await
    }

    /// Essaie de recevoir sans bloquer (non-bloquant).
    pub fn try_recv(&mut self) -> Option<WakeWordEvent> {
        self.rx.try_recv().ok()
    }
}

/// Source de wake word — produit des WakeWordEvent.
pub struct WakeWordSource {
    tx: mpsc::Sender<WakeWordEvent>,
}

impl WakeWordSource {
    /// Crée une source + récepteur avec buffer de 4 événements.
    pub fn new() -> (Self, WakeWordReceiver) {
        let (tx, rx) = mpsc::channel(4);
        (Self { tx }, WakeWordReceiver { rx })
    }

    /// Envoie manuellement un événement (pour tests ou injection externe).
    pub async fn send(&self, event: WakeWordEvent) -> bool {
        self.tx.send(event).await.is_ok()
    }

    /// Clone le sender pour l'utiliser dans un thread OS (rustpotter IPC).
    pub fn sender(&self) -> mpsc::Sender<WakeWordEvent> {
        self.tx.clone()
    }
}

impl Default for WakeWordSource {
    fn default() -> Self {
        Self::new().0
    }
}

/// Configuration du wake word pour le pipeline voix.
#[derive(Debug, Clone)]
pub struct WakeWordConfig {
    /// Mot de réveil attendu (ex: "Alicia").
    pub word: String,
    /// Sensibilité (0.0–1.0). Plus élevé = moins de faux négatifs.
    pub sensitivity: f64,
    /// Délai de ré-activation après une réponse (ms).
    pub reactivation_delay_ms: u64,
    /// Chemin vers le modèle rustpotter (.rpw) — None = désactivé.
    pub model_path: Option<String>,
}

impl WakeWordConfig {
    pub fn for_word(word: impl Into<String>) -> Self {
        Self {
            word: word.into(),
            sensitivity: 0.5,
            reactivation_delay_ms: 1500,
            model_path: None,
        }
    }
}

/// Démarre le thread wake word en mode IPC (communique avec miyualicia-wakeword).
///
/// Le processus miyualicia-wakeword tourne en parallèle et écoute le micro.
/// Il envoie sur stdin/socket les détections. Ce thread lit et forwarde.
///
/// Si `config.model_path` est None, retourne un récepteur jamais déclenché
/// (mode silencieux — l'activation est manuelle via API).
pub fn spawn_wakeword_thread(config: WakeWordConfig) -> WakeWordReceiver {
    let (source, receiver) = WakeWordSource::new();

    if config.model_path.is_none() {
        tracing::info!(
            "Wake word '{}' désactivé (pas de modèle configuré)",
            config.word
        );
        return receiver;
    }

    // Spawner un thread OS (blocking) pour rustpotter IPC
    let tx = source.sender();
    let word = config.word.clone();

    std::thread::spawn(move || {
        tracing::info!("Wake word listener démarré pour '{}'", word);
        // TODO S1-E06 : intégrer rustpotter dans miyualicia-wakeword
        // et lire les événements depuis un pipe/socket Unix/Windows.
        // Pour l'instant ce thread reste en veille.
        //
        // Exemple de futur IPC :
        // loop {
        //     if let Some(detection) = read_from_wakeword_ipc() {
        //         let _ = tx.blocking_send(WakeWordEvent { ... });
        //     }
        // }
        loop {
            std::thread::sleep(std::time::Duration::from_secs(3600));
        }
    });

    receiver
}
