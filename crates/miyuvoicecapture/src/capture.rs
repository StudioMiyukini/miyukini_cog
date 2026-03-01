//! Capture audio via cpal avec buffer circulaire lock-free.
//!
//! Gere le demarrage/arret des streams audio cpal, la conversion
//! vers f32 mono 16kHz, et l'ecriture dans un ring buffer SPSC.

// @id: alicia_capture_capture
// @role: processing
// @layer: toolkit
// @human: Capture audio via cpal, callback ecriture ring buffer, demarrage/arret streams.
// @do: capture_audio_stream

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, StreamConfig};
use ringbuf::{
    traits::{Consumer, Observer, Producer, Split},
    HeapRb,
};
use serde::{Deserialize, Serialize};

use crate::context::{DEFAULT_BUFFER_SIZE, DEFAULT_CHANNELS, DEFAULT_SAMPLE_RATE};
use crate::errors::VoiceCaptureError;

/// Politique de gestion des overflow du ring buffer.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OverflowPolicy {
    /// Les nouveaux samples sont perdus si le buffer est plein.
    DropNewest,
    /// Log un warning mais continue normalement (par defaut).
    #[default]
    WarnAndDrop,
}

/// Statistiques de capture audio (compteurs atomiques thread-safe).
#[derive(Debug)]
pub struct CaptureStats {
    /// Nombre total de samples recus depuis le demarrage.
    pub total_samples: AtomicU64,
    /// Nombre de samples perdus par overflow du ring buffer.
    pub dropped_samples: AtomicU64,
    /// Nombre d'erreurs de callback audio.
    pub callback_errors: AtomicU64,
}

impl CaptureStats {
    /// Cree des statistiques vierges.
    fn new() -> Self {
        Self {
            total_samples: AtomicU64::new(0),
            dropped_samples: AtomicU64::new(0),
            callback_errors: AtomicU64::new(0),
        }
    }

    /// Retourne un snapshot des statistiques courantes.
    #[must_use]
    pub fn snapshot(&self) -> CaptureStatsSnapshot {
        CaptureStatsSnapshot {
            total_samples: self.total_samples.load(Ordering::Relaxed),
            dropped_samples: self.dropped_samples.load(Ordering::Relaxed),
            callback_errors: self.callback_errors.load(Ordering::Relaxed),
        }
    }
}

/// Snapshot immutable des statistiques de capture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureStatsSnapshot {
    /// Nombre total de samples recus.
    pub total_samples: u64,
    /// Nombre de samples perdus.
    pub dropped_samples: u64,
    /// Nombre d'erreurs de callback.
    pub callback_errors: u64,
}

/// Configuration de la capture audio.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureConfig {
    /// Nom ou fragment du nom du peripherique cible. `None` = peripherique par defaut.
    pub device_name: Option<String>,
    /// Frequence d'echantillonnage cible (Hz).
    pub sample_rate: u32,
    /// Nombre de canaux cible.
    pub channels: u16,
    /// Taille du ring buffer en samples.
    pub buffer_size: usize,
    /// Identifiant de la piece associee a cette capture.
    pub room_id: String,
}

impl Default for CaptureConfig {
    fn default() -> Self {
        Self {
            device_name: None,
            sample_rate: DEFAULT_SAMPLE_RATE,
            channels: DEFAULT_CHANNELS,
            buffer_size: DEFAULT_BUFFER_SIZE,
            room_id: String::new(),
        }
    }
}

impl CaptureConfig {
    /// Cree une configuration pour une piece donnee.
    #[must_use]
    pub fn for_room(room_id: &str) -> Self {
        Self {
            room_id: room_id.to_string(),
            ..Self::default()
        }
    }

    /// Definit le peripherique cible par nom.
    #[must_use]
    pub fn with_device(mut self, device_name: &str) -> Self {
        self.device_name = Some(device_name.to_string());
        self
    }

    /// Definit la frequence d'echantillonnage.
    #[must_use]
    pub fn with_sample_rate(mut self, rate: u32) -> Self {
        self.sample_rate = rate;
        self
    }
}

/// Handle de lecture pour consommer les samples du ring buffer.
///
/// Thread-safe, peut etre passe a un thread de traitement VAD/STT.
pub struct CaptureHandle {
    /// Consommateur SPSC du ring buffer.
    consumer: ringbuf::HeapCons<f32>,
    /// Drapeau d'arret partage avec le stream.
    running: Arc<AtomicBool>,
    /// Configuration de capture associee.
    config: CaptureConfig,
    /// Statistiques de capture partagees.
    stats: Arc<CaptureStats>,
}

impl CaptureHandle {
    /// Lit les samples disponibles dans le ring buffer.
    ///
    /// Retourne les samples lus. Le buffer interne est vide apres l'appel.
    pub fn read_available(&mut self) -> Vec<f32> {
        let count = self.consumer.occupied_len();
        if count == 0 {
            return Vec::new();
        }
        let mut output = vec![0.0_f32; count];
        let read = self.consumer.pop_slice(&mut output);
        output.truncate(read);
        output
    }

    /// Lit exactement `count` samples. Retourne moins si le buffer n'en contient pas assez.
    pub fn read_exact(&mut self, count: usize) -> Vec<f32> {
        let available = self.consumer.occupied_len();
        let to_read = count.min(available);
        if to_read == 0 {
            return Vec::new();
        }
        let mut output = vec![0.0_f32; to_read];
        let read = self.consumer.pop_slice(&mut output);
        output.truncate(read);
        output
    }

    /// Retourne le nombre de samples disponibles dans le buffer.
    #[must_use]
    pub fn available(&self) -> usize {
        self.consumer.occupied_len()
    }

    /// Indique si la capture est en cours.
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    /// Arrete la capture associee.
    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    /// Retourne la configuration de capture.
    #[must_use]
    pub fn config(&self) -> &CaptureConfig {
        &self.config
    }

    /// Retourne un snapshot des statistiques de capture.
    #[must_use]
    pub fn stats(&self) -> CaptureStatsSnapshot {
        self.stats.snapshot()
    }
}

/// Gestionnaire de capture audio.
///
/// Encapsule un stream cpal et un ring buffer SPSC.
/// Le stream ecrit dans le producteur du ring buffer via callback.
pub struct AudioCapture {
    /// Drapeau d'arret partage.
    running: Arc<AtomicBool>,
    /// Stream cpal (garde en vie tant que la capture tourne).
    stream: Option<cpal::Stream>,
    /// Configuration de capture.
    config: CaptureConfig,
}

impl AudioCapture {
    /// Demarre la capture audio avec la configuration donnee.
    ///
    /// Retourne un `CaptureHandle` pour lire les samples et un `AudioCapture`
    /// pour controler le cycle de vie du stream.
    pub fn start(config: CaptureConfig) -> Result<(Self, CaptureHandle), VoiceCaptureError> {
        let host = cpal::default_host();

        // Selectionner le peripherique
        let device = if let Some(ref name) = config.device_name {
            Self::find_device_by_name(&host, name)?
        } else {
            host.default_input_device()
                .ok_or(VoiceCaptureError::NoInputDevice)?
        };

        let device_name = device
            .description()
            .map_or_else(|_| "<unknown>".to_string(), |d| d.to_string());
        tracing::info!(
            device = %device_name,
            room = %config.room_id,
            "Starting audio capture"
        );

        // Configurer le stream
        let stream_config = cpal::StreamConfig {
            channels: config.channels,
            sample_rate: config.sample_rate,
            buffer_size: cpal::BufferSize::Default,
        };

        // Creer le ring buffer SPSC
        let rb = HeapRb::<f32>::new(config.buffer_size);
        let (producer, consumer) = rb.split();

        let running = Arc::new(AtomicBool::new(true));
        let running_clone = Arc::clone(&running);
        let stats = Arc::new(CaptureStats::new());

        // Detecter le format supporte et construire le stream
        let supported = device
            .default_input_config()
            .map_err(|e| VoiceCaptureError::UnsupportedConfig(e.to_string()))?;

        let stream = match supported.sample_format() {
            SampleFormat::F32 => Self::build_stream_f32(
                &device,
                &stream_config,
                producer,
                Arc::clone(&running),
                Arc::clone(&stats),
            )?,
            SampleFormat::I16 => Self::build_stream_i16(
                &device,
                &stream_config,
                producer,
                Arc::clone(&running),
                Arc::clone(&stats),
            )?,
            SampleFormat::U16 => Self::build_stream_u16(
                &device,
                &stream_config,
                producer,
                Arc::clone(&running),
                Arc::clone(&stats),
            )?,
            format => {
                return Err(VoiceCaptureError::UnsupportedConfig(
                    format!("Unsupported sample format: {format:?}"),
                ));
            }
        };

        stream
            .play()
            .map_err(|e| VoiceCaptureError::StreamError(e.to_string()))?;

        let capture = Self {
            running: running_clone,
            stream: Some(stream),
            config: config.clone(),
        };

        let handle = CaptureHandle {
            consumer,
            running: Arc::clone(&capture.running),
            config,
            stats,
        };

        Ok((capture, handle))
    }

    /// Arrete la capture audio.
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        self.stream = None;
        tracing::info!(room = %self.config.room_id, "Audio capture stopped");
    }

    /// Indique si la capture est en cours.
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    /// Recherche un peripherique d'entree par nom (correspondance partielle).
    fn find_device_by_name(
        host: &cpal::Host,
        name: &str,
    ) -> Result<cpal::Device, VoiceCaptureError> {
        let lower_name = name.to_lowercase();
        let devices = host
            .input_devices()
            .map_err(|e| VoiceCaptureError::DeviceEnumeration(e.to_string()))?;

        for device in devices {
            if let Ok(desc) = device.description() {
                let dev_name = desc.to_string();
                if dev_name.to_lowercase().contains(&lower_name) {
                    return Ok(device);
                }
            }
        }

        Err(VoiceCaptureError::DeviceNotFound(name.to_string()))
    }

    /// Construit un stream f32.
    fn build_stream_f32(
        device: &cpal::Device,
        config: &StreamConfig,
        mut producer: ringbuf::HeapProd<f32>,
        running: Arc<AtomicBool>,
        stats: Arc<CaptureStats>,
    ) -> Result<cpal::Stream, VoiceCaptureError> {
        let channels = config.channels as usize;
        let err_stats = Arc::clone(&stats);
        let stream = device
            .build_input_stream(
                config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if !running.load(Ordering::Relaxed) {
                        return;
                    }
                    let samples = if channels > 1 {
                        let mono: Vec<f32> = data
                            .chunks(channels)
                            .map(|frame| {
                                let sum: f32 = frame.iter().sum();
                                sum / channels as f32
                            })
                            .collect();
                        let written = producer.push_slice(&mono);
                        let total = mono.len();
                        if written < total {
                            let lost = (total - written) as u64;
                            stats.dropped_samples.fetch_add(lost, Ordering::Relaxed);
                            tracing::warn!(lost, "Ring buffer overflow in f32 stream");
                        }
                        total
                    } else {
                        let written = producer.push_slice(data);
                        if written < data.len() {
                            let lost = (data.len() - written) as u64;
                            stats.dropped_samples.fetch_add(lost, Ordering::Relaxed);
                            tracing::warn!(lost, "Ring buffer overflow in f32 stream");
                        }
                        data.len()
                    };
                    stats.total_samples.fetch_add(samples as u64, Ordering::Relaxed);
                },
                move |err| {
                    err_stats.callback_errors.fetch_add(1, Ordering::Relaxed);
                    tracing::error!(error = %err, "Audio stream error");
                },
                None,
            )
            .map_err(|e| VoiceCaptureError::StreamBuild(e.to_string()))?;
        Ok(stream)
    }

    /// Construit un stream i16 avec conversion vers f32.
    fn build_stream_i16(
        device: &cpal::Device,
        config: &StreamConfig,
        mut producer: ringbuf::HeapProd<f32>,
        running: Arc<AtomicBool>,
        stats: Arc<CaptureStats>,
    ) -> Result<cpal::Stream, VoiceCaptureError> {
        let channels = config.channels as usize;
        let err_stats = Arc::clone(&stats);
        let stream = device
            .build_input_stream(
                config,
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    if !running.load(Ordering::Relaxed) {
                        return;
                    }
                    let f32_data: Vec<f32> = data
                        .iter()
                        .map(|&s| f32::from(s) / f32::from(i16::MAX))
                        .collect();

                    let samples = if channels > 1 {
                        let mono: Vec<f32> = f32_data
                            .chunks(channels)
                            .map(|frame| {
                                let sum: f32 = frame.iter().sum();
                                sum / channels as f32
                            })
                            .collect();
                        let written = producer.push_slice(&mono);
                        let total = mono.len();
                        if written < total {
                            let lost = (total - written) as u64;
                            stats.dropped_samples.fetch_add(lost, Ordering::Relaxed);
                            tracing::warn!(lost, "Ring buffer overflow in i16 stream");
                        }
                        total
                    } else {
                        let written = producer.push_slice(&f32_data);
                        if written < f32_data.len() {
                            let lost = (f32_data.len() - written) as u64;
                            stats.dropped_samples.fetch_add(lost, Ordering::Relaxed);
                            tracing::warn!(lost, "Ring buffer overflow in i16 stream");
                        }
                        f32_data.len()
                    };
                    stats.total_samples.fetch_add(samples as u64, Ordering::Relaxed);
                },
                move |err| {
                    err_stats.callback_errors.fetch_add(1, Ordering::Relaxed);
                    tracing::error!(error = %err, "Audio stream error");
                },
                None,
            )
            .map_err(|e| VoiceCaptureError::StreamBuild(e.to_string()))?;
        Ok(stream)
    }

    /// Construit un stream u16 avec conversion vers f32.
    fn build_stream_u16(
        device: &cpal::Device,
        config: &StreamConfig,
        mut producer: ringbuf::HeapProd<f32>,
        running: Arc<AtomicBool>,
        stats: Arc<CaptureStats>,
    ) -> Result<cpal::Stream, VoiceCaptureError> {
        let channels = config.channels as usize;
        let err_stats = Arc::clone(&stats);
        let stream = device
            .build_input_stream(
                config,
                move |data: &[u16], _: &cpal::InputCallbackInfo| {
                    if !running.load(Ordering::Relaxed) {
                        return;
                    }
                    let f32_data: Vec<f32> = data
                        .iter()
                        .map(|&s| (f32::from(s) / f32::from(u16::MAX)) * 2.0 - 1.0)
                        .collect();

                    let samples = if channels > 1 {
                        let mono: Vec<f32> = f32_data
                            .chunks(channels)
                            .map(|frame| {
                                let sum: f32 = frame.iter().sum();
                                sum / channels as f32
                            })
                            .collect();
                        let written = producer.push_slice(&mono);
                        let total = mono.len();
                        if written < total {
                            let lost = (total - written) as u64;
                            stats.dropped_samples.fetch_add(lost, Ordering::Relaxed);
                            tracing::warn!(lost, "Ring buffer overflow in u16 stream");
                        }
                        total
                    } else {
                        let written = producer.push_slice(&f32_data);
                        if written < f32_data.len() {
                            let lost = (f32_data.len() - written) as u64;
                            stats.dropped_samples.fetch_add(lost, Ordering::Relaxed);
                            tracing::warn!(lost, "Ring buffer overflow in u16 stream");
                        }
                        f32_data.len()
                    };
                    stats.total_samples.fetch_add(samples as u64, Ordering::Relaxed);
                },
                move |err| {
                    err_stats.callback_errors.fetch_add(1, Ordering::Relaxed);
                    tracing::error!(error = %err, "Audio stream error");
                },
                None,
            )
            .map_err(|e| VoiceCaptureError::StreamBuild(e.to_string()))?;
        Ok(stream)
    }
}

impl Drop for AudioCapture {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_config_default() {
        let config = CaptureConfig::default();
        assert_eq!(config.sample_rate, 16_000);
        assert_eq!(config.channels, 1);
        assert_eq!(config.buffer_size, 480_000);
        assert!(config.device_name.is_none());
        assert!(config.room_id.is_empty());
    }

    #[test]
    fn test_capture_config_for_room() {
        let config = CaptureConfig::for_room("chambre-theresa");
        assert_eq!(config.room_id, "chambre-theresa");
        assert_eq!(config.sample_rate, 16_000);
    }

    #[test]
    fn test_capture_config_builder() {
        let config = CaptureConfig::for_room("salon")
            .with_device("USB Microphone")
            .with_sample_rate(48_000);
        assert_eq!(config.room_id, "salon");
        assert_eq!(config.device_name, Some("USB Microphone".to_string()));
        assert_eq!(config.sample_rate, 48_000);
    }

    #[test]
    fn test_ring_buffer_read_write() {
        // Test du ring buffer SPSC independamment de cpal
        let rb = HeapRb::<f32>::new(1024);
        let (mut producer, mut consumer) = rb.split();

        let data: Vec<f32> = (0..100).map(|i| i as f32 / 100.0).collect();
        let written = producer.push_slice(&data);
        assert_eq!(written, 100);

        assert_eq!(consumer.occupied_len(), 100);

        let mut output = vec![0.0_f32; 100];
        let read = consumer.pop_slice(&mut output);
        assert_eq!(read, 100);
        assert!((output[0]).abs() < f32::EPSILON);
        assert!((output[99] - 0.99).abs() < 0.001);
    }

    #[test]
    fn test_ring_buffer_overflow() {
        let rb = HeapRb::<f32>::new(50);
        let (mut producer, _consumer) = rb.split();

        let data = vec![1.0_f32; 100];
        let written = producer.push_slice(&data);
        // Seulement 50 samples ecrits (buffer plein)
        assert_eq!(written, 50);
    }
}
