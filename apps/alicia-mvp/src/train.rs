//! Utilitaire d'entrainement du wake word "Alicia".
//!
//! Enregistre 3-8 echantillons WAV depuis le micro, puis construit
//! un modele rustpotter .rpw via `WakewordRef::new_from_sample_files()`.
//!
//! Usage : `alicia-mvp train-wake-word --output models/hey_alicia.rpw --samples 5`

use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::time::Instant;

use miyualicia_capture::{AudioCapture, CaptureConfig, VoiceActivityDetector, VadState};
use rustpotter::{WakewordRef, WakewordRefBuildFromFiles, WakewordSave};

/// Nombre minimum d'echantillons requis.
const MIN_SAMPLES: usize = 3;
/// Nombre maximum d'echantillons.
const MAX_SAMPLES: usize = 8;
/// Duree max d'enregistrement par echantillon (secondes).
const MAX_RECORD_SECS: u64 = 4;
/// Taille MFCC pour le modele rustpotter.
const MFCC_SIZE: u16 = 13;

/// Lance le workflow d'entrainement du wake word.
pub async fn run_train_wake_word(
    output: PathBuf,
    num_samples: usize,
    threshold: Option<f32>,
) -> anyhow::Result<()> {
    let num_samples = num_samples.clamp(MIN_SAMPLES, MAX_SAMPLES);
    let threshold = threshold.unwrap_or(0.5);

    println!("=== Alicia MVP — Entrainement Wake Word ===");
    println!("  Sortie     : {}", output.display());
    println!("  Samples    : {num_samples}");
    println!("  Seuil      : {threshold:.2} (applique au runtime, pas dans le modele)");
    println!();
    println!("Tu vas enregistrer {num_samples} echantillons du mot \"Alicia\".");
    println!("Pour chaque echantillon, parle clairement apres le signal.\n");

    // Creer le repertoire de sortie si necessaire
    if let Some(parent) = output.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
            println!("  [Train] Repertoire cree : {}", parent.display());
        }
    }

    // Repertoire temporaire pour les WAV
    let tmp_dir = std::env::temp_dir().join("alicia-train-wakeword");
    if tmp_dir.exists() {
        std::fs::remove_dir_all(&tmp_dir)?;
    }
    std::fs::create_dir_all(&tmp_dir)?;

    let mut wav_paths: Vec<String> = Vec::new();

    for i in 1..=num_samples {
        println!("--- Echantillon {i}/{num_samples} ---");
        print!("  Appuie sur Entree quand tu es pret, puis dis \"Alicia\"... ");
        io::stdout().flush()?;
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        println!("  [Rec] Enregistrement... parle maintenant !");
        let samples = record_sample().await?;

        if samples.is_empty() {
            println!("  [Rec] Aucun audio capture, reessaye.");
            // On ne decremente pas — on re-boucle mais il faudra un sample valide
            continue;
        }

        let duration = samples.len() as f64 / 16_000.0;
        println!("  [Rec] {:.1}s capturees ({} samples)", duration, samples.len());

        // Sauvegarder en WAV 16kHz mono 16-bit
        let wav_path = tmp_dir.join(format!("sample_{i:02}.wav"));
        save_wav_16k_mono(&wav_path, &samples)?;
        println!("  [Rec] Sauvegarde : {}", wav_path.display());
        wav_paths.push(wav_path.to_string_lossy().to_string());

        if wav_paths.len() >= num_samples {
            break;
        }
    }

    if wav_paths.len() < MIN_SAMPLES {
        anyhow::bail!(
            "Pas assez d'echantillons valides ({}/{}). Minimum requis : {MIN_SAMPLES}",
            wav_paths.len(),
            num_samples
        );
    }

    println!("\n  [Train] Construction du modele .rpw avec {} echantillons...", wav_paths.len());

    // Construire le modele via rustpotter.
    // On passe None pour les thresholds : ils seront geres au runtime
    // (si Some, les valeurs du modele ecrasent les thresholds runtime).
    let wakeword = WakewordRef::new_from_sample_files(
        "hey_alicia".to_string(),
        None,
        None,
        wav_paths,
        MFCC_SIZE,
    )
    .map_err(|e| anyhow::anyhow!("Erreur construction modele: {e}"))?;

    // Sauvegarder le modele
    let output_str = output.to_string_lossy().to_string();
    wakeword
        .save_to_file(&output_str)
        .map_err(|e| anyhow::anyhow!("Erreur sauvegarde modele: {e}"))?;

    let file_size = std::fs::metadata(&output)?.len();
    println!("  [Train] Modele sauvegarde : {} ({} octets)", output.display(), file_size);

    // Nettoyage
    let _ = std::fs::remove_dir_all(&tmp_dir);

    println!("\n=== Entrainement termine ! ===");
    println!("Utilise le modele avec :");
    println!("  alicia-mvp listen --model {}", output.display());

    Ok(())
}

/// Enregistre un echantillon audio depuis le micro avec detection VAD.
async fn record_sample() -> anyhow::Result<Vec<f32>> {
    let rates_to_try = [48_000_u32, 44_100, 16_000];
    let mut last_err = String::new();
    let mut capture_result = None;

    for &rate in &rates_to_try {
        let config = CaptureConfig::for_room("train").with_sample_rate(rate);
        match AudioCapture::start(config) {
            Ok(result) => {
                capture_result = Some((result, rate));
                break;
            }
            Err(e) => last_err = e.to_string(),
        }
    }

    let ((_capture, mut handle), sample_rate) = capture_result
        .ok_or_else(|| anyhow::anyhow!("aucun sample rate supporte : {last_err}"))?;

    let mut vad = VoiceActivityDetector::new();
    let mut recording = false;
    let mut recorded_samples: Vec<f32> = Vec::new();
    let max_duration = std::time::Duration::from_secs(MAX_RECORD_SECS);
    let silence_timeout = std::time::Duration::from_millis(600);
    let start = Instant::now();
    let mut last_speech = Instant::now();
    let frame_size = (sample_rate / 50) as usize;

    loop {
        tokio::time::sleep(std::time::Duration::from_millis(30)).await;

        let samples = handle.read_available();
        if samples.is_empty() {
            if start.elapsed() > max_duration {
                break;
            }
            continue;
        }

        for chunk in samples.chunks(frame_size) {
            let state = vad.process_frame(chunk);
            match state {
                VadState::Speech | VadState::MaybeSpeech => {
                    if !recording {
                        println!("  [VAD] Parole detectee !");
                        recording = true;
                    }
                    last_speech = Instant::now();
                    recorded_samples.extend_from_slice(chunk);
                }
                VadState::MaybeEnd => {
                    if recording {
                        recorded_samples.extend_from_slice(chunk);
                    }
                }
                VadState::Silence => {
                    if recording {
                        recorded_samples.extend_from_slice(chunk);
                        if last_speech.elapsed() > silence_timeout {
                            break;
                        }
                    }
                }
            }
        }

        if recording && last_speech.elapsed() > silence_timeout {
            println!("  [VAD] Fin de parole");
            break;
        }
        if start.elapsed() > max_duration {
            if !recording {
                println!("  [VAD] Timeout {MAX_RECORD_SECS}s — aucune parole");
            }
            break;
        }
    }

    handle.stop();

    // Resample par moyennage si necessaire (48k/16k=3, pas de decimation pure)
    if sample_rate != 16_000 && !recorded_samples.is_empty() {
        Ok(resample_avg(&recorded_samples, sample_rate, 16_000))
    } else {
        Ok(recorded_samples)
    }
}

/// Resample par moyennage (ratio entier) ou interpolation lineaire.
fn resample_avg(samples: &[f32], from: u32, to: u32) -> Vec<f32> {
    if from == to { return samples.to_vec(); }
    if from % to == 0 {
        let n = (from / to) as usize;
        samples.chunks(n)
            .map(|chunk| chunk.iter().sum::<f32>() / chunk.len() as f32)
            .collect()
    } else {
        let ratio = from as f64 / to as f64;
        let out_len = (samples.len() as f64 / ratio) as usize;
        (0..out_len).filter_map(|i| {
            let pos = i as f64 * ratio;
            let idx = pos as usize;
            let frac = (pos - idx as f64) as f32;
            match (samples.get(idx), samples.get(idx + 1)) {
                (Some(&a), Some(&b)) => Some(a * (1.0 - frac) + b * frac),
                (Some(&a), None) => Some(a),
                _ => None,
            }
        }).collect()
    }
}

/// Sauvegarde des samples f32 en WAV 16kHz mono 16-bit.
fn save_wav_16k_mono(path: &Path, samples: &[f32]) -> anyhow::Result<()> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16_000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(path, spec)?;
    for &s in samples {
        let clamped = s.clamp(-1.0, 1.0);
        let sample_i16 = (clamped * 32767.0) as i16;
        writer.write_sample(sample_i16)?;
    }
    writer.finalize()?;
    Ok(())
}
