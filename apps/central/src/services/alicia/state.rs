//! State bridge Alicia Home Assistante — couple les crates backend avec l'UI Dioxus.
//!
//! Gere le cycle de vie des captures audio (1 par piece), les threads de
//! traitement VAD + wake word, et expose un snapshot lisible par l'UI.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

use miyualicia_capture::{
    AudioCapture, AudioDeviceInfo, CaptureConfig, CaptureHandle, CaptureStatsSnapshot,
    DeviceEnumerator, VadConfig, VadState, VoiceActivityDetector,
};
use miyualicia_wakeword::{RustpotterDetector, WakeWordConfig, WakeWordDetector};

// ─────────────────────────────────────────────────────────────────────────────
// Types partages (ecrits par les threads de traitement, lus par l'UI)
// ─────────────────────────────────────────────────────────────────────────────

/// Pieces predefinies supportees par Alicia Home Assistante.
const ROOMS: &[(&str, &str, &str)] = &[
    ("chambre-theresa", "Chambre Theresa", "\u{1F6CF}\u{FE0F}"),
    (
        "chambre-parentale",
        "Chambre parentale",
        "\u{1F6CF}\u{FE0F}",
    ),
    ("chambre-eleanore", "Chambre Eleanore", "\u{1F6CF}\u{FE0F}"),
    ("salon", "Salon", "\u{1F6CB}\u{FE0F}"),
];

/// Taille de trame pour le traitement VAD (30ms a 16 kHz).
const FRAME_SIZE: usize = 480;

/// Capacite maximale du journal d'activite.
const ACTIVITY_LOG_CAP: usize = 50;

/// Snapshot d'une detection de wake word.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DetectionSnapshot {
    pub keyword: String,
    pub confidence: f32,
    pub detected_at: String,
}

/// Entree du journal d'activite.
#[derive(Debug, Clone)]
pub struct ActivityEntry {
    pub time: String,
    pub room: String,
    pub event: String,
}

/// Etat live d'une piece (ecrit par le thread de traitement, lu par l'UI).
#[derive(Debug, Clone)]
pub struct RoomLiveState {
    pub room_id: String,
    pub room_name: String,
    pub icon: String,
    pub mic_active: bool,
    pub vad_state: VadState,
    pub last_rms: f32,
    pub last_detection: Option<DetectionSnapshot>,
    pub capture_stats: Option<CaptureStatsSnapshot>,
    pub device_name: Option<String>,
    pub vad_threshold: f32,
    pub wake_threshold: f32,
    pub error: Option<String>,
}

/// Snapshot de la configuration wake word (pour affichage dans les settings).
#[derive(Debug, Clone)]
pub struct WakeWordConfigSnapshot {
    pub keyword: String,
    pub model_path: String,
    pub threshold: f32,
    pub sample_rate: u32,
    pub score_mode: String,
    pub band_pass_filter: bool,
    pub gain_normalizer: bool,
}

impl From<&WakeWordConfig> for WakeWordConfigSnapshot {
    fn from(c: &WakeWordConfig) -> Self {
        Self {
            keyword: "Hey Alicia".to_string(),
            model_path: c.model_path.clone(),
            threshold: c.threshold,
            sample_rate: c.sample_rate,
            score_mode: format!("{:?}", c.score_mode),
            band_pass_filter: c.band_pass_filter,
            gain_normalizer: c.gain_normalizer,
        }
    }
}

/// Snapshot global du service Alicia (lu par l'UI).
#[derive(Debug, Clone)]
pub struct AliciaSnapshot {
    pub rooms: Vec<RoomLiveState>,
    pub activity_log: Vec<ActivityEntry>,
    pub available_devices: Vec<AudioDeviceInfo>,
    pub wake_word_config: WakeWordConfigSnapshot,
    pub any_listening: bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// AliciaService — controleur du cycle de vie
// ─────────────────────────────────────────────────────────────────────────────

/// Runtime d'une piece active (capture + thread de traitement).
struct RoomRuntime {
    /// Le `AudioCapture` qui possede le stream cpal (garde en vie ici).
    _capture: AudioCapture,
    /// Signal d'arret pour le thread de traitement.
    stop_flag: Arc<AtomicBool>,
    /// Handle du thread de traitement.
    thread_handle: Option<std::thread::JoinHandle<()>>,
}

/// Controleur du service Alicia Home Assistante.
///
/// Gere le cycle de vie des captures audio et des threads de traitement.
/// L'etat partage est lisible via `snapshot()`.
pub struct AliciaService {
    /// Etat partage entre les threads de traitement et l'UI.
    shared_state: Arc<RwLock<AliciaSnapshot>>,
    /// Runtime par piece active (capture + thread).
    rooms: HashMap<String, RoomRuntime>,
    /// Configuration wake word partagee.
    wake_config: WakeWordConfig,
}

impl AliciaService {
    /// Cree un nouveau service avec la configuration wake word donnee.
    ///
    /// Enumere les devices audio et initialise l'etat des 4 pieces (toutes inactives).
    pub fn new(wake_config: WakeWordConfig) -> Result<Self, String> {
        // Enumerer les devices audio disponibles
        let devices = Self::enumerate_devices()?;

        let wake_config_snap = WakeWordConfigSnapshot::from(&wake_config);

        let rooms: Vec<RoomLiveState> = ROOMS
            .iter()
            .map(|(id, name, icon)| RoomLiveState {
                room_id: (*id).to_string(),
                room_name: (*name).to_string(),
                icon: (*icon).to_string(),
                mic_active: false,
                vad_state: VadState::Silence,
                last_rms: 0.0,
                last_detection: None,
                capture_stats: None,
                device_name: None,
                vad_threshold: VadConfig::default().rms_threshold,
                wake_threshold: wake_config.threshold,
                error: None,
            })
            .collect();

        let snapshot = AliciaSnapshot {
            rooms,
            activity_log: Vec::new(),
            available_devices: devices,
            wake_word_config: wake_config_snap,
            any_listening: false,
        };

        Ok(Self {
            shared_state: Arc::new(RwLock::new(snapshot)),
            rooms: HashMap::new(),
            wake_config,
        })
    }

    /// Retourne un snapshot de l'etat courant (lecture non-bloquante).
    pub fn snapshot(&self) -> AliciaSnapshot {
        self.shared_state
            .read()
            .map_or_else(|_| self.default_snapshot(), |s| s.clone())
    }

    /// Demarre la capture audio pour une piece.
    ///
    /// Cree le stream cpal, le VAD, le detecteur wake word (si le modele existe),
    /// et lance un thread de traitement dedie.
    pub fn start_room(&mut self, room_id: &str, device_name: Option<&str>) -> Result<(), String> {
        // Verifier que la piece n'est pas deja active
        if self.rooms.contains_key(room_id) {
            return Err(format!("La piece {room_id} est deja active"));
        }

        // Construire la config de capture
        let mut config = CaptureConfig::for_room(room_id);
        if let Some(name) = device_name {
            config = config.with_device(name);
        }

        // Demarrer la capture audio
        let (capture, handle) =
            AudioCapture::start(config).map_err(|e| format!("Erreur capture audio: {e}"))?;

        // Creer le VAD
        let vad = VoiceActivityDetector::new();

        // Tenter de creer le detecteur de wake word
        let detector = match RustpotterDetector::new(self.wake_config.clone()) {
            Ok(d) => Some(d),
            Err(e) => {
                let msg = format!("Wake word indisponible: {e}");
                tracing::warn!(room_id, "{msg}");
                // Mettre a jour l'etat avec l'erreur
                if let Ok(mut state) = self.shared_state.write() {
                    if let Some(room) = state.rooms.iter_mut().find(|r| r.room_id == room_id) {
                        room.error = Some(msg);
                    }
                }
                None
            }
        };

        // Mettre a jour l'etat : piece active
        let device_label = device_name.map(String::from);
        if let Ok(mut state) = self.shared_state.write() {
            if let Some(room) = state.rooms.iter_mut().find(|r| r.room_id == room_id) {
                room.mic_active = true;
                room.device_name = device_label;
                room.vad_state = VadState::Silence;
                room.last_rms = 0.0;
            }
            state.any_listening = true;
        }

        // Lancer le thread de traitement
        let stop_flag = Arc::new(AtomicBool::new(false));
        let shared = Arc::clone(&self.shared_state);
        let room_id_owned = room_id.to_string();
        let stop = Arc::clone(&stop_flag);

        let thread_handle = std::thread::Builder::new()
            .name(format!("alicia-{room_id}"))
            .spawn(move || {
                processing_loop(handle, vad, detector, shared, room_id_owned, stop);
            })
            .map_err(|e| format!("Echec spawn thread: {e}"))?;

        self.rooms.insert(
            room_id.to_string(),
            RoomRuntime {
                _capture: capture,
                stop_flag,
                thread_handle: Some(thread_handle),
            },
        );

        tracing::info!(room_id, "Piece demarree");
        Ok(())
    }

    /// Arrete la capture pour une piece.
    pub fn stop_room(&mut self, room_id: &str) {
        if let Some(mut runtime) = self.rooms.remove(room_id) {
            // Signaler l'arret au thread de traitement
            runtime.stop_flag.store(true, Ordering::Relaxed);

            // Attendre la fin du thread (max 500ms)
            if let Some(handle) = runtime.thread_handle.take() {
                let _ = handle.join();
            }

            // Mettre a jour l'etat
            if let Ok(mut state) = self.shared_state.write() {
                if let Some(room) = state.rooms.iter_mut().find(|r| r.room_id == room_id) {
                    room.mic_active = false;
                    room.vad_state = VadState::Silence;
                    room.last_rms = 0.0;
                    room.error = None;
                }
                state.any_listening = state.rooms.iter().any(|r| r.mic_active);
            }

            tracing::info!(room_id, "Piece arretee");
        }
    }

    /// Arrete toutes les pieces actives.
    pub fn stop_all(&mut self) {
        let room_ids: Vec<String> = self.rooms.keys().cloned().collect();
        for room_id in room_ids {
            self.stop_room(&room_id);
        }
    }

    /// Re-enumere les devices audio et met a jour le snapshot.
    pub fn refresh_devices(&self) -> Result<Vec<AudioDeviceInfo>, String> {
        let devices = Self::enumerate_devices()?;
        if let Ok(mut state) = self.shared_state.write() {
            state.available_devices = devices.clone();
        }
        Ok(devices)
    }

    /// Enumere les peripheriques audio disponibles.
    fn enumerate_devices() -> Result<Vec<AudioDeviceInfo>, String> {
        let enumerator = DeviceEnumerator::new();
        enumerator
            .list_input_devices()
            .map_err(|e| format!("Enumeration devices: {e}"))
    }

    /// Snapshot par defaut (en cas d'erreur de lock).
    fn default_snapshot(&self) -> AliciaSnapshot {
        AliciaSnapshot {
            rooms: ROOMS
                .iter()
                .map(|(id, name, icon)| RoomLiveState {
                    room_id: (*id).to_string(),
                    room_name: (*name).to_string(),
                    icon: (*icon).to_string(),
                    mic_active: false,
                    vad_state: VadState::Silence,
                    last_rms: 0.0,
                    last_detection: None,
                    capture_stats: None,
                    device_name: None,
                    vad_threshold: VadConfig::default().rms_threshold,
                    wake_threshold: self.wake_config.threshold,
                    error: None,
                })
                .collect(),
            activity_log: Vec::new(),
            available_devices: Vec::new(),
            wake_word_config: WakeWordConfigSnapshot::from(&self.wake_config),
            any_listening: false,
        }
    }
}

impl Drop for AliciaService {
    fn drop(&mut self) {
        self.stop_all();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Thread de traitement (1 par piece active)
// ─────────────────────────────────────────────────────────────────────────────

/// Boucle de traitement audio pour une piece.
///
/// Lit les samples depuis le `CaptureHandle` (SPSC), traite par frames de 480
/// samples (30ms @ 16 kHz), execute le VAD et le detecteur de wake word,
/// puis met a jour l'etat partage.
#[allow(clippy::needless_pass_by_value)]
fn processing_loop(
    mut handle: CaptureHandle,
    mut vad: VoiceActivityDetector,
    mut detector: Option<RustpotterDetector>,
    state: Arc<RwLock<AliciaSnapshot>>,
    room_id: String,
    stop: Arc<AtomicBool>,
) {
    tracing::debug!(room_id = %room_id, "Thread de traitement demarre");

    while !stop.load(Ordering::Relaxed) && handle.is_running() {
        let samples = handle.read_available();
        if samples.is_empty() {
            std::thread::sleep(Duration::from_millis(20));
            continue;
        }

        // Traiter par frames
        for chunk in samples.chunks(FRAME_SIZE) {
            if stop.load(Ordering::Relaxed) {
                break;
            }

            // VAD
            let vad_state = vad.process_frame(chunk);
            let rms = vad.last_rms();

            // Wake word (sur tous les samples, rustpotter gere son buffering interne)
            let detection = detector.as_mut().and_then(|d| d.process_samples(chunk));

            // Mettre a jour l'etat partage
            if let Ok(mut snap) = state.write() {
                // Extraire les infos de la room (nom, etat VAD precedent) avant de
                // muter d'autres champs du snapshot, pour eviter les conflits de borrow.
                let room_info = snap
                    .rooms
                    .iter()
                    .find(|r| r.room_id == room_id)
                    .map(|r| (r.room_name.clone(), r.vad_state));

                if let Some((room_name, prev_vad)) = room_info {
                    // Preparer les entrees de log avant de muter rooms
                    let mut log_entries: Vec<ActivityEntry> = Vec::new();

                    if let Some(ref det) = detection {
                        let now = chrono::Local::now();
                        let confidence_pct = (det.confidence * 100.0) as u32;
                        log_entries.push(ActivityEntry {
                            time: now.format("%H:%M").to_string(),
                            room: room_name.clone(),
                            event: format!(
                                "Wake word detecte \u{2014} \"{}\" ({confidence_pct}%)",
                                det.keyword,
                            ),
                        });
                    }

                    // Transition VAD : log uniquement quand on entre en Speech
                    if vad_state == VadState::Speech && !matches!(prev_vad, VadState::Speech) {
                        let now = chrono::Local::now();
                        log_entries.push(ActivityEntry {
                            time: now.format("%H:%M").to_string(),
                            room: room_name,
                            event: "Voix detectee (VAD actif)".to_string(),
                        });
                    }

                    // Inserer les logs
                    for entry in log_entries {
                        snap.activity_log.insert(0, entry);
                    }
                    snap.activity_log.truncate(ACTIVITY_LOG_CAP);

                    // Maintenant muter la room
                    if let Some(room) = snap.rooms.iter_mut().find(|r| r.room_id == room_id) {
                        room.vad_state = vad_state;
                        room.last_rms = rms;

                        if let Some(det) = detection {
                            let detected_at = chrono::Local::now().format("%H:%M:%S").to_string();
                            room.last_detection = Some(DetectionSnapshot {
                                keyword: det.keyword.clone(),
                                confidence: det.confidence,
                                detected_at,
                            });
                        }

                        room.capture_stats = Some(handle.stats());
                    }
                }
            }
        }
    }

    // Marquer la piece comme inactive a la sortie du thread
    if let Ok(mut snap) = state.write() {
        if let Some(room) = snap.rooms.iter_mut().find(|r| r.room_id == room_id) {
            room.mic_active = false;
            room.vad_state = VadState::Silence;
            room.last_rms = 0.0;
            if !stop.load(Ordering::Relaxed) {
                // Le thread est sorti sans signal d'arret = device deconnecte
                room.error = Some("Device audio deconnecte".to_string());
            }
        }
        snap.any_listening = snap.rooms.iter().any(|r| r.mic_active);
    }

    tracing::debug!(room_id = %room_id, "Thread de traitement termine");
}

/// Alias pour le type du service partage dans le contexte Dioxus.
pub type SharedAliciaService = Arc<Mutex<AliciaService>>;
