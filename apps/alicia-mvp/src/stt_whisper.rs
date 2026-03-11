//! STT local Whisper via candle-transformers (CUDA ou CPU).
//!
//! NOTE: Cette feature est bloquée par un conflit de dépendances :
//!   rustpotter 3.0 → candle-core 0.2.2 → half ^2.3.1
//!   candle-core 0.9 (Whisper) → half ^2.5.0
//!   Ces deux contraintes sont incompatibles dans le même binaire.
//!
//! Solution : déplacer Whisper dans un crate séparé `apps/miyuwhisper`
//! sans dépendance rustpotter, exposé en serveur HTTP STT.
//!
//! Usage futur (crate séparé) :
//!   cargo run -p miyuwhisper -- --model openai/whisper-small --lang fr
//!   # puis : cargo run -p alicia-mvp -- listen --stt-url http://127.0.0.1:8100

#![cfg(feature = "whisper-cuda")]
#![allow(dead_code)]

use anyhow::Result;

/// Mel spectrogram — implémentation autonome (rustfft, pas de candle).
const SAMPLE_RATE: usize = 16_000;
const N_FFT: usize = 400;
const HOP_LENGTH: usize = 160;
const N_FRAMES: usize = 3_000;

/// Stub — l'implémentation réelle sera dans apps/miyuwhisper.
pub struct WhisperLocal;

impl WhisperLocal {
    pub fn load(_model_id: &str) -> Result<Self> {
        anyhow::bail!(
            "WhisperLocal non disponible dans alicia-mvp (conflit rustpotter/candle). \
             Lancez apps/miyuwhisper et utilisez --stt-url."
        )
    }

    pub fn transcribe(&mut self, _samples: &[f32], _lang: &str) -> Result<String> {
        anyhow::bail!("WhisperLocal non disponible")
    }
}

// ---------------------------------------------------------------------------
// Mel spectrogram (rustfft) — conservé ici pour référence / futur crate
// ---------------------------------------------------------------------------

/// Mel filterbank triangulaire. Retourne [n_mels × (n_fft/2+1)] row-major.
#[allow(unused)]
fn mel_filterbank(n_mels: usize, n_fft: usize, sr: u32, fmin: f32, fmax: f32) -> Vec<f32> {
    let n_freqs = n_fft / 2 + 1;
    let freq_bins: Vec<f32> = (0..n_freqs)
        .map(|i| i as f32 * sr as f32 / n_fft as f32)
        .collect();

    let mel_min = hz_to_mel(fmin);
    let mel_max = hz_to_mel(fmax);
    let mel_pts: Vec<f32> = (0..=n_mels + 1)
        .map(|i| mel_to_hz(mel_min + (mel_max - mel_min) * i as f32 / (n_mels + 1) as f32))
        .collect();

    let mut out = vec![0.0_f32; n_mels * n_freqs];
    for m in 0..n_mels {
        let fl = mel_pts[m];
        let fc = mel_pts[m + 1];
        let fr = mel_pts[m + 2];
        for (k, &f) in freq_bins.iter().enumerate() {
            out[m * n_freqs + k] = if f >= fl && f <= fc {
                (f - fl) / (fc - fl).max(1e-8)
            } else if f > fc && f <= fr {
                (fr - f) / (fr - fc).max(1e-8)
            } else {
                0.0
            };
        }
    }
    out
}

fn hz_to_mel(hz: f32) -> f32 { 1127.0 * (1.0 + hz / 700.0).ln() }
fn mel_to_hz(mel: f32) -> f32 { 700.0 * ((mel / 1127.0).exp() - 1.0) }

/// PCM f32 @ 16kHz → log-mel spectrogram [n_mels * N_FRAMES] row-major.
#[allow(unused)]
fn pcm_to_mel(samples: &[f32], filters: &[f32], n_mels: usize, n_fft: usize, hop: usize) -> Vec<f32> {
    use rustfft::{num_complex::Complex, FftPlanner};

    let n_freqs = n_fft / 2 + 1;
    let window: Vec<f32> = (0..n_fft)
        .map(|i| 0.5 - 0.5 * (2.0 * std::f32::consts::PI * i as f32 / n_fft as f32).cos())
        .collect();

    let pad_left = n_fft / 2;
    let total_needed = pad_left + samples.len().max(N_FRAMES * hop + n_fft);
    let mut padded = vec![0.0_f32; total_needed];
    padded[pad_left..pad_left + samples.len().min(total_needed - pad_left)]
        .copy_from_slice(&samples[..samples.len().min(total_needed - pad_left)]);

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n_fft);
    let mut mel = vec![0.0_f32; n_mels * N_FRAMES];

    for frame in 0..N_FRAMES {
        let start = frame * hop;
        if start + n_fft > padded.len() { break; }

        let mut buf: Vec<Complex<f32>> = (0..n_fft)
            .map(|i| Complex { re: padded[start + i] * window[i], im: 0.0 })
            .collect();
        fft.process(&mut buf);

        let power: Vec<f32> = buf[..n_freqs]
            .iter()
            .map(|c| c.re * c.re + c.im * c.im)
            .collect();

        for m in 0..n_mels {
            let e: f32 = power.iter()
                .zip(&filters[m * n_freqs..])
                .map(|(&p, &f)| p * f)
                .sum::<f32>()
                .max(1e-10);
            mel[m * N_FRAMES + frame] = e.log10();
        }
    }

    let max_v = mel.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    for v in mel.iter_mut() {
        *v = ((*v).max(max_v - 8.0) + 4.0) / 4.0;
    }
    mel
}
