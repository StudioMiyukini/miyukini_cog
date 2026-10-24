//! @id     miyukini_whisper_lib
//! @do     definir_output_profile
//! @role   orchestration
//! @layer  app
//! @human  miyukini-whisper — module « lib ».

//! Miyukini Whisper (squelette V0).
//!
//! Cette crate assemble les toolkits MiyuSTT/MiyuTTS et expose
//! une configuration initiale de presets hardware.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::time::Duration;

use miyustt::{select_preset as select_stt_preset, HardwareProfile as SttHw, SttPreset};
use miyutts::{select_preset as select_tts_preset, HardwareProfile as TtsHw, TtsPreset};
use serde::{Deserialize, Serialize};

/// Profil de sortie texte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputProfile {
    /// Texte brut.
    Verbatim,
    /// Nettoyage local leger.
    Clean,
    /// Reecriture (LLM optionnel).
    Rewrite,
}

/// Mode fallback distant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackMode {
    /// Aucun distant.
    LocalOnly,
    /// Bridge host utilisateur autorise.
    HostBridge,
    /// Bridge host puis cloud opt-in.
    HostBridgeAndCloud,
}

/// Backend selectionne pour la reecriture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RewriteBackend {
    /// Pas de LLM.
    None,
    /// LLM local.
    Local,
    /// Bridge host.
    HostBridge,
    /// Cloud.
    Cloud,
}

/// Backend actif pour service STT/TTS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceBackend {
    /// Service local.
    Local,
    /// Bridge host utilisateur.
    HostBridge,
    /// Provider cloud.
    Cloud,
}

/// Resultat du post-processing de texte.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostProcessResult {
    /// Texte final.
    pub text: String,
    /// Backend reellement utilise.
    pub rewrite_backend: RewriteBackend,
    /// True si fallback vers clean.
    pub fell_back_to_clean: bool,
}

/// Configuration initiale du service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhisperConfig {
    /// Preset STT courant.
    pub stt_preset: SttPreset,
    /// Preset TTS courant.
    pub tts_preset: TtsPreset,
    /// Mode de sortie.
    pub output_profile: OutputProfile,
    /// Mode fallback.
    pub fallback_mode: FallbackMode,
    /// URL bridge host optionnelle.
    pub host_bridge_url: Option<String>,
    /// Cloud explicitement active.
    pub cloud_opt_in: bool,
    /// Timeout max pour mode rewrite.
    pub rewrite_timeout_ms: u64,
}

impl Default for WhisperConfig {
    fn default() -> Self {
        let stt_preset = select_stt_preset(SttHw {
            cpu_cores: 4,
            ram_total_mb: 8192,
            gpu_vram_mb: 0,
        });
        let tts_preset = select_tts_preset(TtsHw {
            ram_total_mb: 8192,
            gpu_vram_mb: 0,
        });
        Self {
            stt_preset,
            tts_preset,
            output_profile: OutputProfile::Verbatim,
            fallback_mode: FallbackMode::LocalOnly,
            host_bridge_url: None,
            cloud_opt_in: false,
            rewrite_timeout_ms: 800,
        }
    }
}

/// Selection du backend de reecriture.
#[must_use]
pub fn resolve_rewrite_backend(
    config: &WhisperConfig,
    local_llm_available: bool,
    host_bridge_available: bool,
) -> RewriteBackend {
    if config.output_profile != OutputProfile::Rewrite {
        return RewriteBackend::None;
    }

    if local_llm_available {
        return RewriteBackend::Local;
    }

    match config.fallback_mode {
        FallbackMode::LocalOnly => RewriteBackend::None,
        FallbackMode::HostBridge => {
            if host_bridge_available {
                RewriteBackend::HostBridge
            } else {
                RewriteBackend::None
            }
        }
        FallbackMode::HostBridgeAndCloud => {
            if host_bridge_available {
                RewriteBackend::HostBridge
            } else if config.cloud_opt_in {
                RewriteBackend::Cloud
            } else {
                RewriteBackend::None
            }
        }
    }
}

/// Resolution du backend STT/TTS selon disponibilites et policy fallback.
#[must_use]
pub fn resolve_service_backend(
    config: &WhisperConfig,
    local_available: bool,
    host_bridge_available: bool,
    cloud_available: bool,
) -> Option<ServiceBackend> {
    if local_available {
        return Some(ServiceBackend::Local);
    }

    match config.fallback_mode {
        FallbackMode::LocalOnly => None,
        FallbackMode::HostBridge => {
            if host_bridge_available {
                Some(ServiceBackend::HostBridge)
            } else {
                None
            }
        }
        FallbackMode::HostBridgeAndCloud => {
            if host_bridge_available {
                Some(ServiceBackend::HostBridge)
            } else if config.cloud_opt_in && cloud_available {
                Some(ServiceBackend::Cloud)
            } else {
                None
            }
        }
    }
}

/// Nettoyage local sans LLM.
#[must_use]
pub fn clean_text(raw: &str) -> String {
    let mut compact = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    compact = compact.trim().to_string();
    if compact.is_empty() {
        return compact;
    }

    let mut chars = compact.chars();
    let first = chars
        .next()
        .map(|ch| ch.to_uppercase().to_string())
        .unwrap_or_default();
    let mut cleaned = format!("{first}{}", chars.collect::<String>());

    if !cleaned.ends_with('.') && !cleaned.ends_with('!') && !cleaned.ends_with('?') {
        cleaned.push('.');
    }
    cleaned
}

/// Nombre max de reecritures en vol (threads actifs, y compris ceux dont le
/// delai a expire mais qui n'ont pas encore rendu la main).
pub const MAX_INFLIGHT_REWRITES: usize = 2;

/// Borne le nombre de threads de reecriture simultanes.
///
/// Un `rewrite_fn` synchrone ne peut pas etre interrompu de l'exterieur : a
/// l'expiration du delai, son thread continue jusqu'a son terme. Pour que des
/// reecritures expirees ne s'accumulent pas, chaque thread occupe une place
/// jusqu'a sa fin reelle ; sans place libre, on bascule directement sur `Clean`
/// sans lancer de thread. Les backends doivent en outre porter leur propre
/// delai (client HTTP, etc.) pour liberer la place.
#[derive(Debug)]
pub struct RewriteLimiter {
    inflight: AtomicUsize,
    max: usize,
}

impl RewriteLimiter {
    /// Cree un limiteur de `max` reecritures simultanees.
    #[must_use]
    pub const fn new(max: usize) -> Self {
        Self {
            inflight: AtomicUsize::new(0),
            max,
        }
    }

    /// Reecritures actuellement en vol.
    #[must_use]
    pub fn inflight(&self) -> usize {
        self.inflight.load(Ordering::Acquire)
    }

    fn try_acquire(&'static self) -> Option<RewriteSlot> {
        let mut current = self.inflight.load(Ordering::Acquire);
        loop {
            if current >= self.max {
                return None;
            }
            match self.inflight.compare_exchange_weak(
                current,
                current + 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return Some(RewriteSlot { limiter: self }),
                Err(actual) => current = actual,
            }
        }
    }
}

/// Place occupee par un thread de reecriture ; rendue a sa destruction.
struct RewriteSlot {
    limiter: &'static RewriteLimiter,
}

impl Drop for RewriteSlot {
    fn drop(&mut self) {
        self.limiter.inflight.fetch_sub(1, Ordering::AcqRel);
    }
}

static GLOBAL_REWRITE_LIMITER: RewriteLimiter = RewriteLimiter::new(MAX_INFLIGHT_REWRITES);

/// Post-process d'un transcript selon le profil actif.
///
/// En mode `Rewrite`, la reecriture tourne dans un thread borne par
/// [`MAX_INFLIGHT_REWRITES`] ; au-dela du delai ou sans place libre,
/// repli sur `Clean`.
#[must_use]
pub fn post_process_text<F>(
    transcript: &str,
    config: &WhisperConfig,
    local_llm_available: bool,
    host_bridge_available: bool,
    rewrite_fn: F,
) -> PostProcessResult
where
    F: Fn(RewriteBackend, &str) -> Result<String, String> + Send + Sync + 'static + Clone,
{
    post_process_text_with_limiter(
        transcript,
        config,
        local_llm_available,
        host_bridge_available,
        rewrite_fn,
        &GLOBAL_REWRITE_LIMITER,
    )
}

/// Variante de [`post_process_text`] avec un limiteur explicite.
#[must_use]
pub fn post_process_text_with_limiter<F>(
    transcript: &str,
    config: &WhisperConfig,
    local_llm_available: bool,
    host_bridge_available: bool,
    rewrite_fn: F,
    limiter: &'static RewriteLimiter,
) -> PostProcessResult
where
    F: Fn(RewriteBackend, &str) -> Result<String, String> + Send + Sync + 'static + Clone,
{
    match config.output_profile {
        OutputProfile::Verbatim => PostProcessResult {
            text: transcript.trim().to_string(),
            rewrite_backend: RewriteBackend::None,
            fell_back_to_clean: false,
        },
        OutputProfile::Clean => PostProcessResult {
            text: clean_text(transcript),
            rewrite_backend: RewriteBackend::None,
            fell_back_to_clean: false,
        },
        OutputProfile::Rewrite => {
            let backend =
                resolve_rewrite_backend(config, local_llm_available, host_bridge_available);
            if backend == RewriteBackend::None {
                return PostProcessResult {
                    text: clean_text(transcript),
                    rewrite_backend: RewriteBackend::None,
                    fell_back_to_clean: true,
                };
            }

            let fallback = || PostProcessResult {
                text: clean_text(transcript),
                rewrite_backend: backend,
                fell_back_to_clean: true,
            };

            // Sans place libre : pas de nouveau thread, repli immediat.
            let Some(slot) = limiter.try_acquire() else {
                tracing::warn!(
                    inflight = limiter.inflight(),
                    "reecriture saturee, repli sur Clean"
                );
                return fallback();
            };

            let (sender, receiver) = mpsc::channel();
            let source = transcript.to_string();
            let rewrite_fn_clone = rewrite_fn.clone();
            let spawned = std::thread::Builder::new()
                .name("whisper-rewrite".into())
                .spawn(move || {
                    // La place est rendue quand le thread se termine reellement.
                    let _slot = slot;
                    let result = rewrite_fn_clone(backend, &source);
                    let _ = sender.send(result);
                });
            if spawned.is_err() {
                return fallback();
            }

            match receiver.recv_timeout(Duration::from_millis(config.rewrite_timeout_ms)) {
                Ok(Ok(text)) => PostProcessResult {
                    text,
                    rewrite_backend: backend,
                    fell_back_to_clean: false,
                },
                _ => fallback(),
            }
        }
    }
}

/// Point d'entree logique (placeholder V0).
pub fn run(config: &WhisperConfig) {
    let post_processed = post_process_text(
        "test diction locale",
        config,
        false,
        false,
        |_backend, input| Ok(input.to_string()),
    );
    tracing::info!(
        stt = ?config.stt_preset,
        tts = ?config.tts_preset,
        output = ?config.output_profile,
        fallback = ?config.fallback_mode,
        rewrite_backend = ?post_processed.rewrite_backend,
        post_processed = %post_processed.text,
        "Miyukini Whisper V0 started"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_profile_normalizes_text() {
        let cleaned = clean_text("bonjour   tout le monde");
        assert_eq!(cleaned, "Bonjour tout le monde.");
    }

    #[test]
    fn rewrite_mode_falls_back_when_no_backend_available() {
        let cfg = WhisperConfig {
            output_profile: OutputProfile::Rewrite,
            fallback_mode: FallbackMode::LocalOnly,
            ..WhisperConfig::default()
        };
        let out = post_process_text("salut", &cfg, false, false, |_backend, _input| {
            Ok("x".to_string())
        });
        assert!(out.fell_back_to_clean);
        assert_eq!(out.rewrite_backend, RewriteBackend::None);
        assert_eq!(out.text, "Salut.");
    }

    #[test]
    fn rewrite_timeout_falls_back_to_clean() {
        let cfg = WhisperConfig {
            output_profile: OutputProfile::Rewrite,
            fallback_mode: FallbackMode::HostBridgeAndCloud,
            cloud_opt_in: true,
            rewrite_timeout_ms: 20,
            ..WhisperConfig::default()
        };
        let out = post_process_text("salut", &cfg, false, true, |_backend, _input| {
            std::thread::sleep(Duration::from_millis(80));
            Ok("rewrite".to_string())
        });
        assert!(out.fell_back_to_clean);
        assert_eq!(out.text, "Salut.");
    }

    #[test]
    fn timed_out_rewrites_cannot_pile_up() {
        static LIMITER: RewriteLimiter = RewriteLimiter::new(1);
        let cfg = WhisperConfig {
            output_profile: OutputProfile::Rewrite,
            fallback_mode: FallbackMode::HostBridge,
            rewrite_timeout_ms: 10,
            ..WhisperConfig::default()
        };
        let (release_tx, release_rx) = mpsc::channel::<()>();
        let release_rx = std::sync::Arc::new(std::sync::Mutex::new(release_rx));
        let blocking = {
            let release_rx = release_rx.clone();
            move |_b: RewriteBackend, _i: &str| {
                if let Ok(rx) = release_rx.lock() {
                    let _ = rx.recv_timeout(Duration::from_secs(5));
                }
                Ok("rewrite".to_string())
            }
        };

        // 1re requete : expire, son thread garde la place.
        let first =
            post_process_text_with_limiter("salut", &cfg, false, true, blocking.clone(), &LIMITER);
        assert!(first.fell_back_to_clean);
        assert_eq!(LIMITER.inflight(), 1);

        // 2e requete : aucune place, aucun nouveau thread.
        let second = post_process_text_with_limiter("salut", &cfg, false, true, blocking, &LIMITER);
        assert!(second.fell_back_to_clean);
        assert_eq!(second.text, "Salut.");
        assert_eq!(LIMITER.inflight(), 1);

        // Liberation : la place est rendue a la fin reelle du thread.
        let _ = release_tx.send(());
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        while LIMITER.inflight() != 0 && std::time::Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(LIMITER.inflight(), 0);

        let cfg_ok = WhisperConfig {
            rewrite_timeout_ms: 2_000,
            ..cfg
        };
        let third = post_process_text_with_limiter(
            "salut",
            &cfg_ok,
            false,
            true,
            |_b, _i| Ok("Bonjour.".to_string()),
            &LIMITER,
        );
        assert!(!third.fell_back_to_clean);
        assert_eq!(third.text, "Bonjour.");
    }

    #[test]
    fn service_backend_prefers_local_when_available() {
        let cfg = WhisperConfig::default();
        let backend = resolve_service_backend(&cfg, true, true, true);
        assert_eq!(backend, Some(ServiceBackend::Local));
    }

    #[test]
    fn service_backend_uses_host_bridge_when_local_down() {
        let cfg = WhisperConfig {
            fallback_mode: FallbackMode::HostBridge,
            ..WhisperConfig::default()
        };
        let backend = resolve_service_backend(&cfg, false, true, true);
        assert_eq!(backend, Some(ServiceBackend::HostBridge));
    }

    #[test]
    fn service_backend_uses_cloud_only_if_opt_in() {
        let cfg = WhisperConfig {
            fallback_mode: FallbackMode::HostBridgeAndCloud,
            cloud_opt_in: true,
            ..WhisperConfig::default()
        };
        let backend = resolve_service_backend(&cfg, false, false, true);
        assert_eq!(backend, Some(ServiceBackend::Cloud));
    }
}
