//! Voice Activity Detection (VAD) — détection de parole dans le flux audio.
//!
//! Algorithme RMS-threshold :
//! 1. Découpe l'audio en frames de 20ms.
//! 2. Frame = "voix" si RMS > threshold (défaut : 0.01).
//! 3. Collecte les segments de voix.
//! 4. Ajoute `padding_ms` avant/après chaque segment.
//! 5. Coupe l'enregistrement après `silence_ms` de silence consécutif.
//!
//! L'audio en entrée doit être en f32 PCM, 16kHz mono.
//! (Utiliser `resample` pour convertir depuis d'autres taux.)

/// Configuration du VAD.
#[derive(Debug, Clone)]
pub struct VadConfig {
    /// Taux d'échantillonnage en Hz (doit être 16000 pour whisper).
    pub sample_rate: u32,
    /// Seuil RMS pour détecter la voix (0.0–1.0).
    pub rms_threshold: f32,
    /// Durée de silence consécutif avant de couper l'enregistrement (ms).
    pub silence_ms: u64,
    /// Padding audio à ajouter avant/après les segments de voix (ms).
    pub padding_ms: u64,
    /// Taille des frames d'analyse (ms).
    pub frame_ms: u64,
}

impl Default for VadConfig {
    fn default() -> Self {
        Self {
            sample_rate: 16_000,
            rms_threshold: 0.01,
            silence_ms: 800,
            padding_ms: 200,
            frame_ms: 20,
        }
    }
}

/// Résultat du VAD — segment de parole détecté.
#[derive(Debug, Clone)]
pub struct VadSegment {
    /// Samples PCM f32 du segment (avec padding inclus).
    pub samples: Vec<f32>,
    /// Offset de début dans l'audio original (en samples).
    pub start_sample: usize,
    /// Offset de fin dans l'audio original (en samples).
    pub end_sample: usize,
    /// Durée en millisecondes.
    pub duration_ms: u32,
}

/// Processeur VAD.
pub struct VadProcessor {
    pub config: VadConfig,
}

impl VadProcessor {
    pub fn new(config: VadConfig) -> Self {
        Self { config }
    }

    pub fn with_defaults() -> Self {
        Self {
            config: VadConfig::default(),
        }
    }

    /// Taille d'une frame en samples.
    fn frame_size(&self) -> usize {
        (self.config.sample_rate as usize * self.config.frame_ms as usize) / 1000
    }

    /// Nombre de frames correspondant à une durée en ms.
    fn ms_to_frames(&self, ms: u64) -> usize {
        (ms as usize).div_ceil(self.config.frame_ms as usize)
    }

    /// Calcule le RMS d'un slice de samples.
    fn rms(samples: &[f32]) -> f32 {
        if samples.is_empty() {
            return 0.0;
        }
        let sum_sq: f32 = samples.iter().map(|s| s * s).sum();
        (sum_sq / samples.len() as f32).sqrt()
    }

    /// Détecte les segments de voix dans `audio` et retourne les segments.
    ///
    /// Un segment est retourné uniquement si de la voix a été détectée avant
    /// `silence_ms` de silence consécutif.
    pub fn detect(&self, audio: &[f32]) -> Vec<VadSegment> {
        let frame_size = self.frame_size();
        if frame_size == 0 || audio.is_empty() {
            return Vec::new();
        }

        let silence_frames = self.ms_to_frames(self.config.silence_ms);
        let padding_samples =
            (self.config.sample_rate as usize * self.config.padding_ms as usize) / 1000;

        // Classification de chaque frame : voix (true) ou silence (false)
        let frames_count = audio.len().div_ceil(frame_size);
        let mut voice_mask: Vec<bool> = (0..frames_count)
            .map(|i| {
                let start = i * frame_size;
                let end = ((i + 1) * frame_size).min(audio.len());
                Self::rms(&audio[start..end]) > self.config.rms_threshold
            })
            .collect();

        // Supprimer les îlots de silence < silence_frames (remplissage)
        let mut i = 0;
        while i < voice_mask.len() {
            if voice_mask[i] {
                // Trouver la fin du segment voix
                let seg_start = i;
                while i < voice_mask.len() && voice_mask[i] {
                    i += 1;
                }
                let seg_end = i;
                // Compter le silence suivant
                let silence_count = voice_mask[seg_end..]
                    .iter()
                    .take(silence_frames)
                    .filter(|&&v| !v)
                    .count();
                // Si silence court après voix → remplir (bridge)
                if silence_count < silence_frames && seg_end + silence_count < voice_mask.len() {
                    for j in seg_end..(seg_end + silence_count) {
                        voice_mask[j] = true;
                    }
                }
            } else {
                i += 1;
            }
        }

        // Extraire les segments continus de voix
        let mut segments = Vec::new();
        let mut in_segment = false;
        let mut seg_start_frame = 0usize;

        for (fi, &is_voice) in voice_mask.iter().enumerate() {
            match (in_segment, is_voice) {
                (false, true) => {
                    seg_start_frame = fi;
                    in_segment = true;
                }
                (true, false) => {
                    // Vérifier si le silence qui suit est long (fin de segment)
                    let remaining_silence = voice_mask[fi..]
                        .iter()
                        .take_while(|&&v| !v)
                        .count();
                    if remaining_silence >= silence_frames || fi + remaining_silence >= voice_mask.len() {
                        // Fin du segment voix
                        let raw_start = seg_start_frame * frame_size;
                        let raw_end = (fi * frame_size).min(audio.len());
                        // Ajouter padding
                        let pad_start = raw_start.saturating_sub(padding_samples);
                        let pad_end = (raw_end + padding_samples).min(audio.len());
                        let duration_ms =
                            ((pad_end - pad_start) * 1000 / self.config.sample_rate as usize) as u32;

                        segments.push(VadSegment {
                            samples: audio[pad_start..pad_end].to_vec(),
                            start_sample: pad_start,
                            end_sample: pad_end,
                            duration_ms,
                        });
                        in_segment = false;
                    }
                    // Sinon silence court → on reste en mode segment (bridge déjà fait)
                }
                _ => {}
            }
        }

        // Clore le dernier segment si on est encore en mode voix
        if in_segment {
            let raw_start = seg_start_frame * frame_size;
            let raw_end = audio.len();
            let pad_start = raw_start.saturating_sub(padding_samples);
            let duration_ms =
                ((raw_end - pad_start) * 1000 / self.config.sample_rate as usize) as u32;

            segments.push(VadSegment {
                samples: audio[pad_start..raw_end].to_vec(),
                start_sample: pad_start,
                end_sample: raw_end,
                duration_ms,
            });
        }

        segments
    }

    /// Retourne true si l'audio contient de la parole détectable.
    pub fn has_speech(&self, audio: &[f32]) -> bool {
        !self.detect(audio).is_empty()
    }
}

// ── Resample ────────────────────────────────────────────────────────────

/// Rééchantillonnage linéaire de `src_rate` Hz vers `dst_rate` Hz.
///
/// Suffisant pour la parole (précision < 16kHz). Pour usage critique,
/// envisager une implémentation sinc via `rubato`.
pub fn resample_linear(samples: &[f32], src_rate: u32, dst_rate: u32) -> Vec<f32> {
    if src_rate == dst_rate {
        return samples.to_vec();
    }
    if samples.is_empty() {
        return Vec::new();
    }

    let ratio = src_rate as f64 / dst_rate as f64;
    let out_len = (samples.len() as f64 / ratio) as usize;
    let mut out = Vec::with_capacity(out_len);

    for i in 0..out_len {
        let src_pos = i as f64 * ratio;
        let idx = src_pos as usize;
        let frac = (src_pos - idx as f64) as f32;

        let s0 = samples[idx.min(samples.len() - 1)];
        let s1 = samples[(idx + 1).min(samples.len() - 1)];
        out.push(s0 + frac * (s1 - s0));
    }

    out
}

/// Normalise les samples en amplitude : ramène le pic à 0.9 (évite la saturation).
pub fn normalize(samples: &[f32]) -> Vec<f32> {
    let peak = samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
    if peak < 1e-6 {
        return samples.to_vec();
    }
    let gain = 0.9 / peak;
    samples.iter().map(|s| s * gain).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_audio(sr: u32, silence_ms: u64, speech_ms: u64, silence2_ms: u64) -> Vec<f32> {
        let sil1 = vec![0.0f32; (sr as usize * silence_ms as usize) / 1000];
        // Signal sinusoïdal à 440 Hz comme "parole"
        let speech_samples = (sr as usize * speech_ms as usize) / 1000;
        let speech: Vec<f32> = (0..speech_samples)
            .map(|i| 0.5 * (2.0 * std::f32::consts::PI * 440.0 * i as f32 / sr as f32).sin())
            .collect();
        let sil2 = vec![0.0f32; (sr as usize * silence2_ms as usize) / 1000];
        [sil1, speech, sil2].concat()
    }

    #[test]
    fn test_vad_detects_speech() {
        let audio = make_audio(16_000, 200, 500, 1000);
        let vad = VadProcessor::with_defaults();
        let segments = vad.detect(&audio);
        assert!(!segments.is_empty(), "Doit détecter au moins un segment");
    }

    #[test]
    fn test_vad_silence_trim() {
        // 1s silence + 500ms parole + 1s silence
        let audio = make_audio(16_000, 1000, 500, 1000);
        let vad = VadProcessor::with_defaults();
        let segments = vad.detect(&audio);

        assert!(!segments.is_empty(), "Doit détecter la parole");
        let seg = &segments[0];
        // Le segment extrait doit être bien plus court que l'audio total
        let total_duration = audio.len() * 1000 / 16_000;
        assert!(
            seg.duration_ms < total_duration as u32,
            "Segment ({} ms) doit être plus court que l'audio total ({} ms)",
            seg.duration_ms,
            total_duration
        );
    }

    #[test]
    fn test_vad_silence_only() {
        let audio = vec![0.0f32; 16_000]; // 1s silence pur
        let vad = VadProcessor::with_defaults();
        assert!(vad.detect(&audio).is_empty(), "Silence pur → aucun segment");
    }

    #[test]
    fn test_resample_linear() {
        let samples: Vec<f32> = (0..44100).map(|i| (i as f32 / 44100.0).sin()).collect();
        let resampled = resample_linear(&samples, 44_100, 16_000);
        let expected_len = (samples.len() as f64 * 16_000.0 / 44_100.0) as usize;
        assert!(
            (resampled.len() as i64 - expected_len as i64).abs() <= 2,
            "Taille resample inattendue : {} vs {expected_len}",
            resampled.len()
        );
    }

    #[test]
    fn test_normalize() {
        let samples = vec![-0.5, 0.0, 0.3, -0.2];
        let norm = normalize(&samples);
        let peak = norm.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
        assert!((peak - 0.9).abs() < 0.001, "Pic normalisé doit être ~0.9");
    }
}
