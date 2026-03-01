# MiyukiniVoice - Specifications Techniques Phase 1

> @id service.voice.miyukinivoice.spec.p1
> @role technical_specification
> @layer 7
> @do specify_audio_capture_and_wake_word_detection_phase1
> @human Documentation technique exhaustive pour la Phase 1 de MiyukiniVoice : capture audio multi-device et detection wake word

---

## Contexte

Ce document constitue la specification technique detaillee pour la Phase 1 de MiyukiniVoice, produite par Denis (Chef Dev Senior) a partir du Document Fondateur de Maria (2026-02-28). Il couvre les deux crates toolkits Strate 6 a implementer : `miyuvoicecapture` (capture audio multi-device via WASAPI) et `miyuwakeword` (detection du mot-cle "Hey Alicia" via rustpotter).

Les decisions architecturales suivantes sont verrouillees par le Document Fondateur :
- Build natif Windows avec WASAPI (Q2 decide)
- Signal analogique sur Cat6 avec carte son USB cote serveur (Q1 decide)
- Serveur unique Machine A : RTX 5070 12 Go VRAM
- `unsafe_code = "forbid"` dans nos crates (les dependances FFI gerent leur propre unsafe)

## Portee / Scope

### Inclus
- Specification complete de `miyuvoicecapture` : architecture, API, configuration WASAPI multi-device, buffer circulaire, VAD
- Specification complete de `miyuwakeword` : architecture, API, choix technique rustpotter, entrainement modele custom
- Configuration build Windows natif (x86_64-pc-windows-msvc)
- Strategie de test sans hardware complet (mock devices, fichiers WAV)
- Checklist de validation Phase 1

### Exclus
- Implementation effective (responsabilite de Francois)
- Specifications des crates Phase 2 (miyustt, miyutts, miyuvoicerouter, miyukinivoice)
- Choix modele LLM (document separe : MiyukiniVoice - Choix Modele LLM.md)

---

## A. Specification `miyuvoicecapture` (Toolkit Strate 6)

> @id toolkit.voice.miyuvoicecapture
> @role audio_capture
> @layer 6
> @do capture_multi_device_audio_streams_via_wasapi
> @human Capture audio PCM depuis 4 cartes son USB simultanees avec buffer circulaire et VAD simple

### A.1 Architecture du crate

```
crates/miyuvoicecapture/
+-- Cargo.toml
+-- src/
|   +-- lib.rs              # Racine, API publique, re-exports
|   +-- admin_cell.rs        # Metadonnees gouvernance toolkit
|   +-- context.rs           # GovernedContext standard
|   +-- errors.rs            # VoiceCaptureError
|   +-- capture.rs           # CaptureStream, buffer circulaire, flux PCM
|   +-- devices.rs           # DeviceEnumerator, DeviceInfo, identification USB
|   +-- vad.rs               # VoiceActivityDetector, seuil d'energie, debounce
+-- tests/
    +-- capture_tests.rs     # Tests unitaires capture
    +-- device_tests.rs      # Tests enumeration devices
    +-- vad_tests.rs          # Tests VAD
    +-- integration_tests.rs  # Tests avec fichiers WAV mock
```

### A.2 API publique

#### A.2.1 Module `errors.rs`

```rust
/// @id toolkit.voice.miyuvoicecapture.errors
/// @do define_error_types_for_audio_capture

#[derive(Debug, Clone)]
pub enum VoiceCaptureError {
    /// Aucun mandat de gouvernance fourni
    NoMandate,
    /// Device audio introuvable (nom, index ou serial inconnu)
    DeviceNotFound(String),
    /// Device audio deconnecte en cours de capture
    DeviceDisconnected(String),
    /// Erreur de configuration du stream audio (format, sample rate)
    StreamConfig(String),
    /// Buffer circulaire en overflow (consommateur trop lent)
    BufferOverflow { device_id: String, dropped_samples: usize },
    /// Permission refusee pour acceder au device audio
    PermissionDenied(String),
    /// Erreur interne cpal
    AudioBackend(String),
    /// Fonctionnalite non implementee
    Unimplemented,
}

impl std::fmt::Display for VoiceCaptureError { /* ... */ }
impl std::error::Error for VoiceCaptureError {}
```

**Invariants :**
- Chaque variante d'erreur contient suffisamment d'information pour diagnostiquer le probleme
- Le `device_id` est toujours le nom human-readable du device (pas un index arbitraire)
- `BufferOverflow` rapporte le nombre exact d'echantillons perdus

#### A.2.2 Module `devices.rs`

```rust
/// @id toolkit.voice.miyuvoicecapture.devices
/// @do enumerate_and_identify_usb_audio_devices

use cpal::traits::{DeviceTrait, HostTrait};

/// Informations sur un peripherique audio detecte.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Nom retourne par WASAPI (ex: "USB Audio Device (2)")
    pub name: String,
    /// Index dans la liste d'enumeration cpal (stable tant que les devices ne changent pas)
    pub index: usize,
    /// Nom convivial attribue par l'utilisateur (ex: "Chambre Theresa")
    pub friendly_name: Option<String>,
    /// Identifiant stable derive du nom WASAPI pour re-identification
    pub stable_id: String,
    /// Indique si le device supporte la capture (input)
    pub supports_input: bool,
    /// Indique si le device supporte la lecture (output)
    pub supports_output: bool,
    /// Configuration audio preferee du device
    pub default_config: Option<AudioConfig>,
}

/// Configuration audio d'un device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    /// Frequence d'echantillonnage en Hz
    pub sample_rate: u32,
    /// Nombre de canaux (1 = mono, 2 = stereo)
    pub channels: u16,
    /// Format d'echantillon
    pub sample_format: SampleFormat,
}

/// Formats d'echantillon supportes.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SampleFormat {
    /// Entier signe 16 bits (PCM standard)
    I16,
    /// Flottant 32 bits normalise [-1.0, 1.0]
    F32,
}

/// Enumere tous les peripheriques audio disponibles via WASAPI.
pub fn enumerate_devices() -> Result<Vec<DeviceInfo>, VoiceCaptureError>;

/// Recherche un device par son nom WASAPI exact.
pub fn find_device_by_name(name: &str) -> Result<DeviceInfo, VoiceCaptureError>;

/// Recherche un device par son identifiant stable.
pub fn find_device_by_stable_id(stable_id: &str) -> Result<DeviceInfo, VoiceCaptureError>;

/// Genere un identifiant stable a partir du nom WASAPI.
/// L'algorithme : normalise en minuscules, supprime les espaces, hash blake3 tronque 16 chars.
pub fn generate_stable_id(device_name: &str) -> String;

/// Verifie si un device est toujours connecte et operationnel.
pub fn is_device_available(stable_id: &str) -> bool;
```

**Strategie d'identification des devices USB :**

Les 4 cartes son USB identiques seront distinguees par leur nom WASAPI. Windows attribue des noms incrementaux : "USB Audio Device", "USB Audio Device (2)", "USB Audio Device (3)", "USB Audio Device (4)". Ce nommage est stable tant que les cartes sont branchees sur les memes ports USB.

Pour la robustesse, on utilise un identifiant stable (`stable_id`) derive du nom WASAPI, et un mapping configurable `stable_id -> friendly_name` (ex: "Chambre Theresa") persiste dans KindMother (Phase 3).

**Invariants :**
- L'enumeration ne doit jamais panic si un device est debranche pendant le scan
- Les `DeviceInfo` sont `Clone + Send + Sync` pour usage multi-thread
- Le `stable_id` est deterministe : meme nom -> meme id

#### A.2.3 Module `capture.rs`

```rust
/// @id toolkit.voice.miyuvoicecapture.capture
/// @do manage_audio_capture_streams_with_circular_buffer

use std::sync::Arc;

/// Configuration d'un flux de capture audio.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureConfig {
    /// Identifiant stable du device source
    pub device_stable_id: String,
    /// Frequence d'echantillonnage souhaitee (defaut: 16000 Hz)
    pub sample_rate: u32,
    /// Nombre de canaux (defaut: 1 = mono)
    pub channels: u16,
    /// Format d'echantillon (defaut: F32)
    pub sample_format: SampleFormat,
    /// Taille du buffer circulaire en echantillons (defaut: 32000 = 2 secondes a 16kHz)
    pub buffer_size: usize,
    /// Comportement en cas d'overflow (defaut: DropOldest)
    pub overflow_policy: OverflowPolicy,
}

/// Politique de gestion des overflows du buffer circulaire.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OverflowPolicy {
    /// Ecrase les echantillons les plus anciens (FIFO)
    DropOldest,
    /// Rejette les nouveaux echantillons
    DropNewest,
    /// Signale une erreur (arrete la capture)
    Error,
}

/// Flux de capture audio actif.
/// Encapsule le stream cpal et le buffer circulaire.
pub struct CaptureStream {
    // Champs prives : cpal::Stream, ringbuf producer/consumer, state
}

impl CaptureStream {
    /// Cree et demarre un nouveau flux de capture.
    pub fn start(config: &CaptureConfig) -> Result<Self, VoiceCaptureError>;

    /// Arrete le flux de capture.
    pub fn stop(&mut self) -> Result<(), VoiceCaptureError>;

    /// Lit les echantillons disponibles dans le buffer circulaire.
    /// Retourne un Vec<f32> d'echantillons normalises [-1.0, 1.0].
    /// Non-bloquant : retourne un vecteur vide si aucun echantillon disponible.
    pub fn read_samples(&mut self) -> Vec<f32>;

    /// Lit exactement `count` echantillons. Bloquant si necessaire.
    /// Timeout configurable via le parametre `timeout`.
    pub fn read_exact(
        &mut self,
        count: usize,
        timeout: std::time::Duration,
    ) -> Result<Vec<f32>, VoiceCaptureError>;

    /// Retourne le nombre d'echantillons actuellement dans le buffer.
    pub fn available_samples(&self) -> usize;

    /// Retourne true si le flux est actif.
    pub fn is_active(&self) -> bool;

    /// Retourne les statistiques de capture.
    pub fn stats(&self) -> CaptureStats;
}

/// Statistiques d'un flux de capture.
#[derive(Debug, Clone, Default)]
pub struct CaptureStats {
    /// Nombre total d'echantillons captures
    pub total_samples: u64,
    /// Nombre d'echantillons perdus par overflow
    pub dropped_samples: u64,
    /// Nombre d'erreurs de callback
    pub callback_errors: u64,
    /// Timestamp du dernier echantillon recu (monotone)
    pub last_sample_time: Option<std::time::Instant>,
}
```

**Buffer circulaire :**

| Parametre | Valeur par defaut | Justification |
|-----------|-------------------|---------------|
| Taille | 32 000 echantillons | 2 secondes a 16 kHz mono, suffisant pour le wake word + marge |
| Format interne | f32 normalise [-1.0, 1.0] | Compatible whisper-rs et rustpotter |
| Politique overflow | DropOldest | On veut toujours les echantillons les plus recents |

**Implementation recommandee :** Utiliser le crate `ringbuf` (lock-free SPSC ring buffer) pour le transfert entre le callback audio cpal (producteur, thread audio) et le consommateur (thread de detection).

**Format audio cible :**

| Parametre | Valeur | Justification |
|-----------|--------|---------------|
| Sample rate | 16 000 Hz | Standard Whisper, rustpotter, suffisant pour la voix |
| Channels | 1 (mono) | Les micros MAX9814 sont mono |
| Bit depth | f32 normalise | Format natif cpal, pas de conversion |
| Bitrate resultant | 64 kbit/s | Tres faible, pas de probleme de bande passante |

**Notes WASAPI :**
- cpal 0.17+ utilise l'API WASAPI shared mode par defaut sur Windows
- Le mode shared permet a plusieurs applications d'acceder au meme device
- La latence typique en shared mode est de 10-30ms, acceptable pour notre usage
- Si le sample rate natif du device differe de 16 kHz, WASAPI effectue le resampling automatiquement en shared mode

#### A.2.4 Module `vad.rs`

```rust
/// @id toolkit.voice.miyuvoicecapture.vad
/// @do detect_voice_activity_via_energy_threshold

/// Configuration du detecteur d'activite vocale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VadConfig {
    /// Seuil d'energie RMS au-dessus duquel on considere qu'il y a de la voix.
    /// Valeur par defaut : 0.01 (echelle f32 normalisee).
    /// A calibrer empiriquement pour chaque piece / micro.
    pub energy_threshold: f32,

    /// Duree minimale d'activite vocale pour confirmer la detection (ms).
    /// Evite les faux positifs sur les bruits courts.
    /// Valeur par defaut : 200 ms.
    pub min_speech_duration_ms: u32,

    /// Duree de silence apres la fin de la parole avant de couper (ms).
    /// Permet de capturer la fin des phrases.
    /// Valeur par defaut : 500 ms.
    pub silence_timeout_ms: u32,

    /// Taille de la fenetre d'analyse en echantillons.
    /// Valeur par defaut : 480 (30 ms a 16 kHz).
    pub frame_size: usize,
}

/// Etat du detecteur VAD.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VadState {
    /// Aucune activite vocale detectee
    Silence,
    /// Activite vocale en cours (au-dessus du seuil)
    Speech,
    /// Fin de parole detectee, en attente du timeout de silence
    SpeechEnding,
}

/// Resultat d'une analyse VAD sur une trame.
#[derive(Debug, Clone)]
pub struct VadResult {
    /// Etat courant
    pub state: VadState,
    /// Energie RMS de la trame analysee
    pub energy: f32,
    /// Transition detectee (None si pas de changement)
    pub transition: Option<VadTransition>,
}

/// Transitions VAD.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VadTransition {
    /// Passage de Silence a Speech
    SpeechStart,
    /// Passage de Speech/SpeechEnding a Silence
    SpeechEnd,
}

/// Detecteur d'activite vocale base sur le seuil d'energie RMS.
pub struct VoiceActivityDetector {
    // Champs prives : config, etat interne, compteurs debounce
}

impl VoiceActivityDetector {
    /// Cree un nouveau detecteur avec la configuration donnee.
    pub fn new(config: VadConfig) -> Self;

    /// Analyse une trame d'echantillons audio et retourne le resultat.
    /// Les echantillons doivent etre en f32 normalise [-1.0, 1.0].
    pub fn process_frame(&mut self, samples: &[f32]) -> VadResult;

    /// Reinitialise l'etat du detecteur.
    pub fn reset(&mut self);

    /// Retourne l'etat courant sans traiter de nouvelles donnees.
    pub fn current_state(&self) -> VadState;

    /// Met a jour le seuil d'energie dynamiquement.
    pub fn set_threshold(&mut self, threshold: f32);
}
```

**Algorithme VAD :**

1. Calculer l'energie RMS de la trame : `sqrt(sum(sample^2) / count)`
2. Comparer au seuil (`energy_threshold`)
3. Si au-dessus et etat = Silence : commencer le compteur de debounce
4. Si le compteur depasse `min_speech_duration_ms` : transition `SpeechStart`
5. Si en dessous et etat = Speech : passer en `SpeechEnding`, demarrer le timeout
6. Si le timeout depasse `silence_timeout_ms` : transition `SpeechEnd`

**Role du VAD dans la Phase 1 :**
Le VAD est optionnel en Phase 1. Son role principal est de pre-filtrer le flux audio avant de le passer au wake word detector, pour eviter de consommer des cycles CPU sur du silence. En Phase 2, il servira aussi a delimiter les segments de parole pour le STT.

#### A.2.5 Module `lib.rs`

```rust
#![allow(missing_docs)]
//! # MiyuVoiceCapture -- toolkit.voice.miyuvoicecapture
//!
//! @id toolkit.voice.miyuvoicecapture
//! @role audio_capture
//! @layer 6
//! @do capture_multi_device_audio_streams_via_wasapi
//! @human Capture audio PCM multi-device via WASAPI avec buffer circulaire et VAD
//!
//! Toolkit Strate 6 pour la capture audio multi-device dans le pipeline MiyukiniVoice.
//! Supporte l'enumeration des cartes son USB, la capture PCM via cpal/WASAPI,
//! un buffer circulaire lock-free, et un detecteur d'activite vocale (VAD) simple.

pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod capture;
pub mod devices;
pub mod vad;

pub use admin_cell::{
    miyuvoicecapture_admin_cell, MiyuVoiceCaptureAdminCell,
    MiyuVoiceCaptureIdentification, MiyuVoiceCaptureIntegrity,
    MiyuVoiceCaptureTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::VoiceCaptureError;
pub use capture::{CaptureConfig, CaptureStream, CaptureStats, OverflowPolicy};
pub use devices::{AudioConfig, DeviceInfo, SampleFormat};
pub use vad::{VadConfig, VadResult, VadState, VadTransition, VoiceActivityDetector};
```

#### A.2.6 Module `admin_cell.rs`

```rust
/// @id toolkit.voice.miyuvoicecapture.admin
/// @do define_governance_metadata_for_voice_capture

pub const TOOLKIT_ID: &str = "toolkit.voice.miyuvoicecapture";

// Structure standard AdminCell conforme au pattern miyukini-rust-patterns :
// MiyuVoiceCaptureIdentification, MiyuVoiceCaptureTestManifest,
// MiyuVoiceCaptureIntegrity, MiyuVoiceCaptureAdminCell
// + fonction miyuvoicecapture_admin_cell(version, fingerprint)
```

Tests declares dans le TestManifest :
- `test_device_enumeration` : enumeration des devices audio disponibles
- `test_capture_stream_start_stop` : demarrage/arret d'un flux de capture
- `test_buffer_overflow_policy` : comportement du buffer circulaire en overflow
- `test_vad_speech_detection` : detection d'activite vocale sur signal synthetique
- `test_vad_silence` : absence de detection sur signal silencieux

### A.3 Dependances `Cargo.toml`

```toml
[package]
name = "miyuvoicecapture"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
documentation.workspace = true
keywords.workspace = true
categories.workspace = true
description = "Kit d'outils capture audio multi-device WASAPI -- Phase 1 MiyukiniVoice"

[dependencies]
miyukini-kernel = { path = "../miyukini-kernel" }
serde = { version = "1.0", features = ["derive"] }

# Audio capture cross-platform (WASAPI sur Windows)
cpal = "0.17"

# Buffer circulaire lock-free SPSC
ringbuf = "0.4"

# Hash pour generate_stable_id
blake3 = "1"

# Logging structure
tracing = "0.1"

[dev-dependencies]
# Fichiers WAV pour tests
hound = "3.5"

[lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"

[lints.clippy]
all = { level = "warn", priority = -1 }
pedantic = { level = "warn", priority = -1 }
```

**Justification des dependances :**
- `cpal 0.17` : crate audio cross-platform de reference en Rust, support WASAPI natif, ~10M downloads, activement maintenu, licence Apache-2.0. Supporte l'enumeration multi-device et la creation de streams independants. Version 0.17.3 actuelle (mise a jour depuis 0.15 sur recommandation audit George).
- `ringbuf 0.4` : buffer circulaire lock-free SPSC (Single Producer Single Consumer), ideal pour le transfert du callback audio (thread temps reel) vers le thread de traitement. Zero allocation apres init, pas de mutex.
- `blake3 1` : hash cryptographique rapide pour la generation de stable_id des devices audio. Utilise dans `generate_stable_id()` (section A.2.2).
- `hound 3.5` (dev) : lecture/ecriture de fichiers WAV, utilise exclusivement dans les tests pour charger des echantillons de reference.

---

## B. Specification `miyuwakeword` (Toolkit Strate 6)

> @id toolkit.voice.miyuwakeword
> @role wake_word_detection
> @layer 6
> @do detect_hey_alicia_wake_word_on_audio_stream
> @human Detection du mot-cle "Hey Alicia" sur flux audio continu via rustpotter

### B.1 Architecture du crate

```
crates/miyuwakeword/
+-- Cargo.toml
+-- src/
|   +-- lib.rs              # Racine, API publique, re-exports
|   +-- admin_cell.rs        # Metadonnees gouvernance toolkit
|   +-- context.rs           # GovernedContext standard
|   +-- errors.rs            # WakeWordError
|   +-- detector.rs          # WakeWordDetector, integration rustpotter
|   +-- models.rs            # Gestion du modele wake word (chargement, validation)
+-- models/
|   +-- hey_alicia.rpw       # Modele wakeword reference "Hey Alicia" (a generer)
+-- tests/
|   +-- detector_tests.rs    # Tests unitaires detecteur
|   +-- model_tests.rs       # Tests chargement modele
|   +-- integration_tests.rs # Tests detection sur fichiers WAV
+-- training/
    +-- README.md            # Instructions d'entrainement du modele
    +-- samples/             # Dossier pour les echantillons WAV d'entrainement
```

### B.2 Choix technique : rustpotter

**Decision : rustpotter 3.x**

**Justification :**

| Critere | rustpotter | Porcupine (Picovoice) |
|---------|-----------|----------------------|
| Licence | MIT (libre) | Gratuit limite / payant pro (~60 EUR/an) |
| Langage | Rust natif | C via FFI |
| `unsafe_code` | Compatible `forbid` (100% Rust) | Necessite FFI = unsafe dans le crate wrapper |
| Modeles custom | Oui, entrainable via CLI | Oui, via console web Picovoice |
| Support francais | Agnostique (base sur audio, pas sur langue) | Modeles FR officiels |
| Taille modele | 320 Ko - 3.1 Mo (selon type) | ~2 Mo |
| RAM | ~50 Mo | ~10 Mo |
| Precision | Bonne avec entrainement suffisant | Excellente (industrielle) |
| Dernier commit | Octobre 2023 (v3.0.2, stable) | Activement maintenu |
| Integration cpal | Native (accepte PCM brut) | Via buffer PCM aussi |

**Risques rustpotter et mitigations :**

| Risque | Probabilite | Mitigation |
|--------|-------------|------------|
| Detection insuffisante de "Hey Alicia" en FR | Moyenne | Entrainer un modele custom Medium ou Large avec 50+ echantillons varies (distances, volumes, locuteurs) |
| Faux positifs eleves | Faible-Moyenne | Ajuster le seuil de confiance (threshold), utiliser le mode "averaged" |
| Dernier commit en 2023 | Faible impact | La lib est stable (v3.0.2), pas de bugs bloquants connus, le code est simple et bien isole |
| Latence de detection | Faible | < 100 ms mesure en benchmark, acceptable |

**Plan de fallback :** Si apres les tests Phase 1 rustpotter ne donne pas satisfaction (< 80% detection ou > 10% faux positifs), basculer vers Porcupine via un crate wrapper FFI separe. Le trait `WakeWordDetector` permettra ce swap sans modifier le code appelant.

### B.3 API publique

#### B.3.1 Module `errors.rs`

```rust
/// @id toolkit.voice.miyuwakeword.errors
/// @do define_error_types_for_wake_word_detection

#[derive(Debug, Clone)]
pub enum WakeWordError {
    /// Aucun mandat de gouvernance fourni
    NoMandate,
    /// Fichier modele introuvable
    ModelNotFound(String),
    /// Fichier modele corrompu ou version incompatible
    ModelInvalid(String),
    /// Erreur de configuration du detecteur
    Config(String),
    /// Erreur lors du traitement audio
    AudioProcessing(String),
    /// Fonctionnalite non implementee
    Unimplemented,
}

impl std::fmt::Display for WakeWordError { /* ... */ }
impl std::error::Error for WakeWordError {}
```

#### B.3.2 Module `detector.rs`

```rust
/// @id toolkit.voice.miyuwakeword.detector
/// @do implement_wake_word_detection_with_rustpotter

/// Evenement de detection du wake word.
#[derive(Debug, Clone)]
pub struct WakeWordDetection {
    /// Nom du mot-cle detecte (ex: "hey_alicia")
    pub keyword: String,
    /// Score de confiance [0.0, 1.0]
    pub confidence: f32,
    /// Timestamp monotone de la detection
    pub timestamp: std::time::Instant,
    /// Nombre d'echantillons traites depuis le debut du flux
    pub sample_offset: u64,
}

/// Configuration du detecteur de wake word.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WakeWordConfig {
    /// Chemin vers le fichier modele (.rpw pour reference, .rpwm pour modele entraine)
    pub model_path: String,
    /// Seuil de confiance minimum pour valider une detection [0.0, 1.0]
    /// Valeur par defaut : 0.5
    /// Plus haut = moins de faux positifs mais plus de non-detections
    pub threshold: f32,
    /// Frequence d'echantillonnage attendue (doit correspondre au modele)
    /// Valeur par defaut : 16000 Hz
    pub sample_rate: u32,
    /// Nombre de canaux attendus
    /// Valeur par defaut : 1 (mono)
    pub channels: u16,
    /// Duree minimale entre deux detections consecutives (ms)
    /// Evite les doubles detections sur un meme enonce
    /// Valeur par defaut : 2000 ms
    pub min_detection_interval_ms: u32,
    /// Mode de scoring rustpotter
    /// Valeur par defaut : Average
    pub score_mode: ScoreMode,
    /// Active le filtre passe-bande pour la voix (300-3400 Hz)
    /// Valeur par defaut : true
    pub band_pass_filter: bool,
    /// Active la normalisation de gain automatique
    /// Valeur par defaut : true
    pub gain_normalizer: bool,
}

/// Mode de calcul du score rustpotter.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScoreMode {
    /// Moyenne des scores sur les references
    Average,
    /// Score maximum parmi les references
    Max,
    /// Percentile (P75 par defaut)
    Percentile,
}

/// Trait pour le detecteur de wake word.
/// Permet de swapper l'implementation (rustpotter, Porcupine, etc.)
/// sans modifier le code appelant.
pub trait WakeWordDetector: Send {
    /// Traite un buffer d'echantillons audio (f32 normalise).
    /// Retourne Some(detection) si le wake word est detecte.
    fn process_samples(&mut self, samples: &[f32]) -> Option<WakeWordDetection>;

    /// Reinitialise l'etat interne du detecteur.
    fn reset(&mut self);

    /// Retourne le nom du mot-cle configure.
    fn keyword_name(&self) -> &str;

    /// Retourne le seuil de confiance actuel.
    fn threshold(&self) -> f32;

    /// Met a jour le seuil de confiance.
    fn set_threshold(&mut self, threshold: f32);
}

/// Implementation rustpotter du detecteur de wake word.
pub struct RustpotterDetector {
    // Champs prives : instance Rustpotter, config, etat debounce
}

impl RustpotterDetector {
    /// Cree un nouveau detecteur rustpotter a partir de la configuration.
    pub fn new(config: WakeWordConfig) -> Result<Self, WakeWordError>;
}

impl WakeWordDetector for RustpotterDetector {
    // Implementation des methodes du trait
}
```

#### B.3.3 Module `models.rs`

```rust
/// @id toolkit.voice.miyuwakeword.models
/// @do manage_wake_word_model_files

/// Types de modeles rustpotter supportes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelType {
    /// Modele reference (3-8 enregistrements WAV, DTW)
    Reference,
    /// Modele entraine (reseau de neurones, meilleure precision)
    Trained,
}

/// Informations sur un modele charge.
#[derive(Debug, Clone)]
pub struct ModelInfo {
    /// Chemin du fichier modele
    pub path: String,
    /// Type de modele (reference ou entraine)
    pub model_type: ModelType,
    /// Mot-cle associe
    pub keyword: String,
    /// Taille du fichier en octets
    pub file_size: u64,
}

/// Charge et valide un fichier modele rustpotter.
/// Verifie l'existence, la lisibilite, et la version du format.
pub fn load_model(path: &str) -> Result<ModelInfo, WakeWordError>;

/// Verifie qu'un fichier modele est valide sans le charger completement.
pub fn validate_model(path: &str) -> Result<ModelType, WakeWordError>;

/// Retourne le chemin par defaut du modele "Hey Alicia".
/// Convention : {crate_root}/models/hey_alicia.rpw
pub fn default_model_path() -> String;
```

### B.4 Entrainement du modele "Hey Alicia"

**Procedure d'entrainement :**

1. **Enregistrer les echantillons** (50-100 fichiers WAV) :
   - Format : WAV, 16 kHz, mono, 16 bits PCM
   - Locuteurs varies : au moins 3-4 personnes differentes
   - Distances variees : 0.5m, 1m, 2m, 3m
   - Volumes varies : voix normale, voix forte, chuchotement
   - Conditions variees : silence, bruit de fond modere (TV, musique)
   - Chaque fichier : 1-2 secondes, contenant "Hey Alicia" uniquement
   - Nommage : `hey_alicia_001.wav`, `hey_alicia_002.wav`, etc.

2. **Enregistrer les echantillons negatifs** (50-100 fichiers WAV) :
   - Phrases similaires : "Hey Lisa", "Hey Lydia", "Hey Maria", "Alicia", "Bonjour"
   - Bruits ambiants : TV, musique, conversations distantes
   - Nommage : `negative_001.wav`, `negative_002.wav`, etc.

3. **Structure des dossiers d'entrainement** :
   ```
   training/
   +-- train/
   |   +-- [hey_alicia]hey_alicia_001.wav
   |   +-- [hey_alicia]hey_alicia_002.wav
   |   +-- negative_001.wav
   |   +-- negative_002.wav
   +-- test/
       +-- [hey_alicia]hey_alicia_test_001.wav
       +-- negative_test_001.wav
   ```

4. **Commande d'entrainement** :
   ```bash
   rustpotter-cli train \
     --model-type medium \
     --train-dir training/train/ \
     --test-dir training/test/ \
     --output models/hey_alicia.rpwm
   ```

5. **Approche alternative (reference, plus rapide)** :
   ```bash
   rustpotter-cli build-ref \
     --name hey_alicia \
     --samples training/train/[hey_alicia]*.wav \
     --output models/hey_alicia.rpw
   ```

**Recommandation Denis :** Commencer par un modele reference (3-8 echantillons) pour le prototypage rapide, puis passer a un modele entraine Medium (2.1 Mo) une fois les echantillons negatifs collectes.

### B.5 Seuils de confiance et faux positifs

| Parametre | Valeur recommandee | Plage de calibration |
|-----------|--------------------|----------------------|
| Threshold (reference) | 0.5 | 0.3 - 0.7 |
| Threshold (modele entraine) | 0.7 | 0.5 - 0.9 |
| Min detection interval | 2000 ms | 1000 - 5000 ms |
| Band pass filter | true | - |
| Gain normalizer | true | - |
| Score mode (reference) | Average | Average / Max / Percentile |

**Procedure de calibration :**
1. Demarrer avec un threshold de 0.5
2. Tester pendant 30 minutes en conditions reelles
3. Si trop de faux positifs : augmenter de 0.05 jusqu'a < 5% de faux positifs
4. Si trop de non-detections : baisser de 0.05 jusqu'a > 90% de detection
5. L'objectif est de trouver le sweet spot pour chaque piece

### B.6 Integration avec miyuvoicecapture

```
[miyuvoicecapture]          [miyuwakeword]

CaptureStream               RustpotterDetector
    |                            |
    +-- read_samples() --------> process_samples()
    |   (Vec<f32>, 480          (analyse frame)
    |    samples = 30ms)             |
    |                           [Detection?]
    |                                |
    +-- (continue capture) <--- None (pas de detection)
    |                           Some(WakeWordDetection)
    |                                |
    +-- (capture continue,      [Evenement remonte]
         audio recent dans      au pipeline Phase 2
         le buffer pour STT)
```

**Flux d'integration :**
1. `CaptureStream` capture en continu a 16 kHz mono
2. Le thread de detection lit par frames de 480 echantillons (30 ms)
3. Chaque frame est passe a `RustpotterDetector::process_samples()`
4. Si detection : l'evenement `WakeWordDetection` est envoye au pipeline
5. Le buffer circulaire conserve les dernieres 2 secondes d'audio, utilisable par le STT en Phase 2

### B.7 Dependances `Cargo.toml`

```toml
[package]
name = "miyuwakeword"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
documentation.workspace = true
keywords.workspace = true
categories.workspace = true
description = "Kit d'outils detection wake word -- Phase 1 MiyukiniVoice"

[dependencies]
miyukini-kernel = { path = "../miyukini-kernel" }
serde = { version = "1.0", features = ["derive"] }

# Detection wake word 100% Rust
rustpotter = "3.0"

# Logging structure
tracing = "0.1"

[dev-dependencies]
# Fichiers WAV pour tests
hound = "3.5"

[lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"

[lints.clippy]
all = { level = "warn", priority = -1 }
pedantic = { level = "warn", priority = -1 }
```

**Justification :**
- `rustpotter 3.0` : version stable, 100% Rust (compatible `unsafe_code = "forbid"`), MIT license, supporte les modeles reference et entraines, ~50 Mo RAM en execution. Le dernier commit date d'octobre 2023 mais le crate est stable et sans bugs bloquants connus.

### B.8 Tests unitaires prevus

| Test | Description | Type |
|------|-------------|------|
| `test_detector_creation` | Verifier la creation d'un detecteur avec config valide | Unitaire |
| `test_detector_invalid_model` | Verifier l'erreur sur modele inexistant | Unitaire |
| `test_detector_threshold_update` | Verifier la mise a jour dynamique du seuil | Unitaire |
| `test_detection_on_positive_wav` | Charger un WAV "Hey Alicia", verifier detection | Integration |
| `test_no_detection_on_negative_wav` | Charger un WAV negatif, verifier absence de detection | Integration |
| `test_detection_debounce` | Envoyer 2 "Hey Alicia" rapproches, verifier qu'un seul event est emis | Integration |
| `test_model_load_valid` | Charger un modele valide, verifier les metadonnees | Unitaire |
| `test_model_load_corrupted` | Tenter de charger un fichier corrompu, verifier l'erreur | Unitaire |
| `test_band_pass_filter` | Verifier que le filtre passe-bande ne degrade pas la detection | Integration |

---

## C. Configuration build Windows natif

### C.1 Target et toolchain

```
Target : x86_64-pc-windows-msvc
Toolchain : stable-x86_64-pc-windows-msvc
MSVC : Visual Studio Build Tools 2022+ (necessaire pour cpal WASAPI)
```

**Verification :**
```bash
rustup show
# Doit afficher : stable-x86_64-pc-windows-msvc (default)

rustup target list --installed
# Doit inclure : x86_64-pc-windows-msvc
```

### C.2 Cargo.toml workspace

Les crates `miyuvoicecapture` et `miyuwakeword` doivent etre ajoutes au workspace racine :

```toml
# Dans Cargo.toml racine, section [workspace] members
"crates/miyuvoicecapture",
"crates/miyuwakeword",
```

### C.3 Specificites WASAPI

**Permissions :**
- Aucune permission speciale requise pour l'audio en mode shared WASAPI
- Les applications Windows ont acces aux devices audio par defaut
- En cas de refus : verifier les parametres de confidentialite Windows (Parametres > Confidentialite > Microphone)

**Multi-device :**
- cpal enumere tous les devices via `cpal::default_host().input_devices()`
- Chaque device peut avoir un stream independant
- 4 streams simultanees ne posent pas de probleme de performance (les callbacks sont tres legers)
- Les devices USB sont visibles immediatement apres branchement (hotplug OS)

**Latence :**
- WASAPI shared mode : 10-30 ms de latence typique
- Configurable via `cpal::StreamConfig` (buffer size)
- Pour MiyukiniVoice, la latence audio n'est pas critique (wake word tolere 100ms+)

### C.4 Tests sans hardware complet

**Strategie de mock :**

1. **Fichiers WAV de reference** :
   - Creer un dossier `tests/fixtures/` dans chaque crate
   - Y placer des fichiers WAV de test : silence, parole, bruit, "Hey Alicia"
   - Utiliser `hound` pour lire les fichiers WAV dans les tests
   - Passer les echantillons directement a `VoiceActivityDetector::process_frame()` et `RustpotterDetector::process_samples()`

2. **Mock du CaptureStream** :
   - Dans les tests d'integration, ne pas ouvrir de vrai device cpal
   - Utiliser un `Vec<f32>` charge depuis un fichier WAV comme source
   - Le trait `WakeWordDetector` est testable independamment du hardware

3. **Tests CI (sans device audio)** :
   ```rust
   #[test]
   fn test_device_enumeration_no_panic() {
       // Ce test verifie juste que l'enumeration ne panic pas
       // meme si aucun device n'est disponible (CI)
       let devices = enumerate_devices();
       // Ok ou Err, jamais panic
       assert!(devices.is_ok() || devices.is_err());
   }
   ```

4. **Tests avec device virtuel (optionnel)** :
   - Sur Windows, on peut installer un driver audio virtuel (VB-Cable, VoiceMeeter)
   - Permet de tester le pipeline complet sans micro physique
   - Non requis pour la CI, utile pour le dev local

### C.5 Environnement de dev recommande

```
OS : Windows 11
IDE : VS Code / Cursor avec rust-analyzer
Toolchain : rustup stable-x86_64-pc-windows-msvc
Build Tools : Visual Studio Build Tools 2022
CUDA : CUDA Toolkit 12.x (requis Phase 2 pour whisper-rs, pas Phase 1)
```

---

## D. Checklist de validation Phase 1

### D.1 Criteres de succes (du Document Fondateur)

| # | Critere | Seuil | Methode de mesure |
|---|---------|-------|-------------------|
| C1 | Wake word detecte a 2m de distance | Detection positive | Test en conditions reelles, micro MAX9814, 2m |
| C2 | Bruit ambiant modere tolere | Detection malgre TV a volume normal | Test avec TV allumee dans la meme piece |
| C3 | Taux de faux positifs | < 5% sur 1h | Compter les declenchements non voulus sur 1h de fonctionnement |
| C4 | Taux de detection | > 90% en conditions normales | 20 tests consecutifs, >= 18 detections |
| C5 | Latence detection | < 200 ms apres fin du mot-cle | Mesure par timestamp (fin audio -> event detection) |

### D.2 Procedure de test detaillee

#### Phase 1.A : Tests unitaires et CI

| Etape | Action | Critere de succes |
|-------|--------|-------------------|
| 1 | `cargo test -p miyuvoicecapture` | 100% des tests passent |
| 2 | `cargo test -p miyuwakeword` | 100% des tests passent |
| 3 | `cargo clippy -p miyuvoicecapture -- -D warnings` | 0 warnings |
| 4 | `cargo clippy -p miyuwakeword -- -D warnings` | 0 warnings |
| 5 | `cargo build -p miyuvoicecapture` | Build sans erreur |
| 6 | `cargo build -p miyuwakeword` | Build sans erreur |

#### Phase 1.B : Tests d'integration avec hardware

| Etape | Action | Critere de succes |
|-------|--------|-------------------|
| 7 | Brancher 1 carte son USB + micro MAX9814 | Device visible dans l'enumeration cpal |
| 8 | Lancer la capture, parler, verifier les niveaux audio | Forme d'onde non nulle, amplitude > 0.01 |
| 9 | Tester le VAD : parler puis silence | Transitions SpeechStart/SpeechEnd correctes |
| 10 | Charger le modele "Hey Alicia" | Pas d'erreur, ModelInfo valide |
| 11 | Dire "Hey Alicia" a 0.5m | Detection avec confiance > 0.5 |
| 12 | Dire "Hey Alicia" a 1m | Detection avec confiance > 0.4 |
| 13 | Dire "Hey Alicia" a 2m | Detection avec confiance > 0.3 |
| 14 | Attendre 5 min en silence | 0 faux positifs |
| 15 | Allumer la TV (volume normal), attendre 10 min | <= 1 faux positif |
| 16 | Dire "Hey Alicia" avec TV allumee a 1m | Detection positive |

#### Phase 1.C : Tests multi-device (preparation Phase 3)

| Etape | Action | Critere de succes |
|-------|--------|-------------------|
| 17 | Brancher 4 cartes son USB | 4 devices enumeres avec noms distincts |
| 18 | Lancer 4 CaptureStream simultanement | 4 streams actifs, pas d'erreur |
| 19 | Parler sur chaque micro, verifier l'isolation | Chaque stream capte uniquement son micro |

### D.3 Metriques a mesurer

| Metrique | Unite | Outil de mesure |
|----------|-------|-----------------|
| Latence de detection | ms | Timestamp `Instant::now()` avant/apres `process_samples` |
| Taux de detection (TPR) | % | Compteur detections / tentatives sur 20 essais |
| Taux de faux positifs (FPR) | % / heure | Compteur faux positifs / duree de test |
| Utilisation CPU | % | `sysinfo` crate ou Gestionnaire des taches |
| Utilisation RAM | Mo | `sysinfo` crate ou Gestionnaire des taches |
| Qualite signal (SNR) | dB | Calcul sur segments silence vs parole |
| Energie RMS moyenne silence | f32 | VoiceActivityDetector output |
| Energie RMS moyenne parole | f32 | VoiceActivityDetector output |

### D.4 Rapport de tests attendu

A la fin de la Phase 1, George (Audit Expert) produira un rapport contenant :
1. Resultats de chaque etape de la checklist (pass/fail)
2. Metriques mesurees avec valeurs et comparaison aux seuils
3. Anomalies detectees et recommandations
4. Go/No-Go pour le passage en Phase 2
5. Ajustements de seuil recommandes pour le detecteur

---

## E. Distribution des taches

### E.1 Taches Francois (Back-End)

| # | Tache | Priorite | Estimation | Dependance |
|---|-------|----------|------------|------------|
| F1 | Creer le crate squelette `miyuvoicecapture` (structure, admin_cell, context, errors) | Haute | 0.5 jour | Aucune |
| F2 | Implementer `devices.rs` : enumeration, identification, stable_id | Haute | 1 jour | F1 |
| F3 | Implementer `capture.rs` : CaptureStream avec cpal + ringbuf | Haute | 2 jours | F1, F2 |
| F4 | Implementer `vad.rs` : VoiceActivityDetector, algorithme RMS | Moyenne | 1 jour | F1 |
| F5 | Tests unitaires miyuvoicecapture (WAV fixtures) | Haute | 1 jour | F2, F3, F4 |
| F6 | Creer le crate squelette `miyuwakeword` (structure, admin_cell, context, errors) | Haute | 0.5 jour | Aucune |
| F7 | Implementer `detector.rs` : trait + RustpotterDetector | Haute | 2 jours | F6 |
| F8 | Implementer `models.rs` : chargement, validation | Moyenne | 0.5 jour | F6 |
| F9 | Generer le modele reference "Hey Alicia" (3-8 echantillons initiaux) | Haute | 1 jour | F7 (+ micro branche) |
| F10 | Tests unitaires miyuwakeword (WAV fixtures) | Haute | 1 jour | F7, F8 |
| F11 | Test d'integration bout-en-bout : capture -> wake word -> log | Haute | 1 jour | F3, F7, F9 |
| F12 | Ajouter les crates au workspace Cargo.toml racine | Haute | 0.25 jour | F1, F6 |

**Estimation totale Francois : 10-12 jours de travail**

### E.2 Taches Denis (supervision)

| # | Tache | Priorite |
|---|-------|----------|
| D1 | Revue de code F1-F12 au fil de l'eau | Continue |
| D2 | Validation des fixtures WAV (qualite, format) | Haute |
| D3 | Calibration des seuils wake word avec Francois | Haute |
| D4 | Execution tests finaux (`cargo test --workspace`) | Haute |
| D5 | Coordination avec George pour l'audit Phase 1 | Haute |
| D6 | Mise a jour de la documentation technique si ajustements | Moyenne |

---

## F. Securite et conformite

### F.1 Vie privee (RGPD)

| Exigence | Implementation Phase 1 |
|----------|----------------------|
| Pas d'enregistrement continu | Le buffer circulaire ecrase les echantillons anciens (2 secondes max). Aucun fichier audio n'est ecrit sur disque en production. |
| Traitement post-wake-word uniquement | En Phase 1, le STT n'est pas encore implemente. Le pipeline s'arrete apres la detection du wake word (log uniquement). |
| Pas de transmission reseau | Aucune donnee audio ne quitte la machine. Zero cloud. LOI-1 respectee. |
| Consentement | Le systeme ne s'active que sur le wake word. Les membres du foyer sont informes de la presence du micro (usage domestique, pas de tiers). |

### F.2 Securite du code

| Exigence | Verification |
|----------|-------------|
| `unsafe_code = "forbid"` | Present dans les 2 Cargo.toml. Verifie par `cargo clippy`. |
| Pas de `unwrap()` en production | Revue de code manuelle + clippy `unwrap_used` lint. |
| Types d'erreur explicites | `VoiceCaptureError`, `WakeWordError` — pas de `String` generique. |
| Pas de donnees sensibles en clair | Aucune donnee sensible manipulee en Phase 1 (pas de credentials, pas de tokens). |
| Dependances auditees | `cpal` (Apache-2.0, RustAudio), `rustpotter` (MIT, GiviMAD), `ringbuf` (MIT/Apache-2.0) — toutes open source, code auditable. |

### F.3 Invariants

| Invariant | Description | Verification |
|-----------|-------------|-------------|
| INV-VC-01 | Le buffer circulaire ne depasse jamais sa taille configuree | Test `test_buffer_overflow_policy` |
| INV-VC-02 | Les echantillons audio sont toujours en f32 normalise [-1.0, 1.0] | Assert dans `CaptureStream::read_samples()` |
| INV-VC-03 | Un device deconnecte ne provoque pas de panic | Test `test_device_disconnected` + erreur gracieuse |
| INV-VC-04 | Le VAD ne produit pas de transition sans donnees | `process_frame` requiert `samples.len() >= frame_size` |
| INV-WW-01 | Le detecteur ne retourne pas de detection sous le seuil | Verifie dans `process_samples`, assert confidence >= threshold |
| INV-WW-02 | Le debounce empeche les doubles detections | `min_detection_interval_ms` respecte, test `test_detection_debounce` |
| INV-WW-03 | Un modele invalide est rejete au chargement | `load_model` retourne `Err(ModelInvalid)` |

---

## G. Notes pour la Phase 2

Les decisions suivantes ont ete preparees mais ne sont pas implementees en Phase 1 :

1. **whisper-rs** : version 0.15.1 confirmee, CUDA feature flag supporte, instructions de build Windows disponibles. A activer en Phase 2 avec `whisper-rs = { version = "0.15", features = ["cuda"] }`.

2. **Piper TTS** : Le depot officiel rhasspy/piper a ete **archive le 6 octobre 2025**. Cependant, les modeles vocaux restent disponibles et le crate `piper-rs` (par thewh1teagle) est maintenu et fonctionnel via ONNX Runtime. Voir le document Q7/Q8 et le document LLM pour les details.

3. **Kokoro TTS** : Alternative emergente a Piper, avec support du francais et implementations Rust via `kokorox` et `kokoro-onnx`. A evaluer en Phase 2 comme option complementaire ou de remplacement.

4. **Buffer audio pour STT** : Le buffer circulaire de 2 secondes est dimensionne pour que le STT Phase 2 puisse recuperer l'audio recent (incluant le contexte avant et apres le wake word).

---

*Document produit par Denis, Chef Dev Senior Miyukini AI Studio -- 2026-02-28*
*A destination de : Francois (implementation), George (audit), Arianne (archivage)*
*Ref : MiyukiniVoice - Document Fondateur (Maria, 2026-02-28)*
