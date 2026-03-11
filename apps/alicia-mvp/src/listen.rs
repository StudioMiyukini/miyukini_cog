//! Mode always-on : ecoute permanente avec detection de wake word.
//!
//! Boucle : capture micro → VAD → Rustpotter → (wake word detecte)
//!        → capture commande → STT → NLU → TTS → haut-parleur → reboucle.
//!
//! Press-to-speak : appuyer sur Entrée skip le wake word et va directement
//!                  en mode capture commande.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use miyualicia_capture::{AudioCapture, CaptureConfig};

use crate::runtime::{AliciaRuntime, ServiceArgs};
use crate::voice;

// ---------------------------------------------------------------------------
// Press-to-speak : thread stdin → AtomicBool
// ---------------------------------------------------------------------------

/// Lance un thread en arrière-plan qui lit stdin.
/// Chaque appui sur Entrée met le flag à `true`.
fn spawn_manual_trigger() -> Arc<AtomicBool> {
    let flag = Arc::new(AtomicBool::new(false));
    let flag2 = flag.clone();
    std::thread::spawn(move || {
        use std::io::BufRead;
        let stdin = std::io::stdin();
        for _ in stdin.lock().lines() {
            flag2.store(true, Ordering::Relaxed);
        }
    });
    flag
}

// ---------------------------------------------------------------------------
// run_listen_mode
// ---------------------------------------------------------------------------

/// Lance le mode ecoute permanente avec wake word "Alicia".
pub async fn run_listen_mode(
    stt_url: String,
    tts_url: String,
    llm_url: Option<String>,
    llm_key: Option<String>,
    face_url: Option<String>,
    search_url: Option<String>,
    workspace: Option<PathBuf>,
    model_path: Option<String>,
    threshold: Option<f32>,
    whisper_model: Option<String>,
    whisper_lang: String,
) -> anyhow::Result<()> {
    let rt = AliciaRuntime::build(ServiceArgs {
        stt_url,
        tts_url,
        llm_url,
        llm_key,
        face_url,
        search_url,
        workspace,
    })
    .await?;

    let model = model_path.unwrap_or_else(|| "models/hey_alicia.rpw".to_string());
    let threshold = threshold.unwrap_or(0.4);

    if !std::path::Path::new(&model).exists() {
        anyhow::bail!("Modele introuvable: {model}");
    }

    // samples_per_frame via un detecteur ephemere
    let samples_per_frame = {
        let mut cfg = rustpotter::RustpotterConfig::default();
        cfg.fmt.sample_rate = 16_000;
        cfg.fmt.sample_format = rustpotter::SampleFormat::F32;
        cfg.fmt.channels = 1;
        let tmp = rustpotter::Rustpotter::new(&cfg)
            .map_err(|e| anyhow::anyhow!("Rustpotter init: {e}"))?;
        tmp.get_samples_per_frame()
    };

    // Whisper local (GPU si feature activee)
    #[cfg(feature = "whisper-cuda")]
    let mut whisper_local: Option<crate::stt_whisper::WhisperLocal> = match whisper_model {
        Some(ref id) => {
            println!("  [Whisper] Chargement du modele {}...", id);
            match crate::stt_whisper::WhisperLocal::load(id) {
                Ok(w) => { println!("  [Whisper] Pret."); Some(w) }
                Err(e) => { println!("  [Whisper] ERREUR: {e:#} — fallback HTTP STT"); None }
            }
        }
        None => None,
    };

    // Press-to-speak
    let manual_trigger = spawn_manual_trigger();

    let stt_label = {
        #[cfg(feature = "whisper-cuda")]
        {
            if whisper_local.is_some() {
                format!("Whisper local ({})", whisper_model.as_deref().unwrap_or("?"))
            } else {
                rt.stt_url.clone()
            }
        }
        #[cfg(not(feature = "whisper-cuda"))]
        { rt.stt_url.clone() }
    };

    println!("=== Alicia MVP — Mode Listen [NLU: {}] ===", rt.nlu.label());
    println!("  Modele    : {model}");
    println!("  Seuil     : {threshold:.2}");
    println!("  STT       : {stt_label}");
    println!("  TTS       : {}", rt.tts_url);
    println!("\nEn attente du mot-cle... (Entrée = press-to-speak, Ctrl+C pour quitter)\n");

    loop {
        // Phase 1 : ecoute passive avec press-to-speak
        let (keyword, confidence) =
            listen_for_wake_word(&model, threshold, samples_per_frame, &manual_trigger).await?;

        if keyword == "__manual__" {
            println!("\n  [PressToSpeak] Activation manuelle — parlez !");
        } else {
            println!("\n  [WakeWord] \"{}\" detecte ! (score: {:.3})", keyword, confidence);
        }

        // Phase 2 : chime de confirmation
        play_chime();

        // Phase 3 : capturer la commande
        println!("  [Listen] Ecoute de la commande...");
        let samples = match voice::capture_speech().await {
            Ok(s) if !s.is_empty() => s,
            Ok(_) => { println!("  [Listen] Aucune parole, retour en ecoute."); continue; }
            Err(e) => { println!("  [Listen] Erreur capture : {e}"); continue; }
        };
        println!("  [Listen] {} samples ({:.1}s)", samples.len(), samples.len() as f64 / 16_000.0);

        // Phase 4 : STT (Whisper local ou HTTP)
        let start = Instant::now();
        let transcript = {
            #[cfg(feature = "whisper-cuda")]
            {
                if let Some(ref mut wh) = whisper_local {
                    match wh.transcribe(&samples, &whisper_lang) {
                        Ok(t) => { println!("  [STT/Whisper] \"{}\" ({:?})", t, start.elapsed()); t }
                        Err(e) => { println!("  [STT/Whisper] Erreur : {e:#}"); continue; }
                    }
                } else {
                    match voice::call_stt(&rt.http, &rt.stt_url, &samples).await {
                        Ok(t) => { println!("  [STT] \"{}\" ({:?})", t, start.elapsed()); t }
                        Err(e) => { println!("  [STT] Erreur : {e:#}"); continue; }
                    }
                }
            }
            #[cfg(not(feature = "whisper-cuda"))]
            {
                match voice::call_stt(&rt.http, &rt.stt_url, &samples).await {
                    Ok(t) => { println!("  [STT] \"{}\" ({:?})", t, start.elapsed()); t }
                    Err(e) => { println!("  [STT] Erreur : {e:#}"); continue; }
                }
            }
        };

        // Phase 5 : NLU
        let nlu_start = Instant::now();
        let (intent, raw) = rt.nlu.parse(&transcript).await;
        println!("  [NLU] {} → {:?} ({:?})", raw, intent, nlu_start.elapsed());

        // Phase 6 : Dispatch + TTS
        rt.dispatch(&intent, &transcript).await;
        let response = rt.nlu.generate_response(&intent, &transcript).await;
        println!("  [Alicia] \"{}\"", response);
        match voice::call_tts(&rt.http, &rt.tts_url, &response).await {
            Ok(wav) => { println!("  [TTS] {} octets", wav.len()); voice::play_wav(&wav); }
            Err(e) => println!("  [TTS] Erreur : {e}"),
        }

        println!("\nEn attente du mot-cle... (Entrée = press-to-speak)\n");
    }
}

// ---------------------------------------------------------------------------
// Wake word detection
// ---------------------------------------------------------------------------

/// Ecoute passive avec rustpotter direct.
/// Retourne `("__manual__", 1.0)` si l'utilisateur appuie sur Entrée.
async fn listen_for_wake_word(
    model_path: &str,
    threshold: f32,
    samples_per_frame: usize,
    manual_trigger: &Arc<AtomicBool>,
) -> anyhow::Result<(String, f32)> {
    let mut rp_cfg = rustpotter::RustpotterConfig::default();
    rp_cfg.fmt.sample_rate = 16_000;
    rp_cfg.fmt.channels = 1;
    rp_cfg.fmt.sample_format = rustpotter::SampleFormat::F32;
    rp_cfg.detector.threshold = threshold;
    rp_cfg.detector.avg_threshold = threshold * 0.7;
    rp_cfg.detector.eager = true;
    rp_cfg.detector.min_scores = 3;
    rp_cfg.filters.gain_normalizer.enabled = true;
    rp_cfg.filters.band_pass.enabled = true;

    let mut rp = rustpotter::Rustpotter::new(&rp_cfg)
        .map_err(|e| anyhow::anyhow!("Rustpotter init: {e}"))?;
    rp.add_wakeword_from_file("hey_alicia", model_path)
        .map_err(|e| anyhow::anyhow!("Chargement modele: {e}"))?;

    // Capture micro
    let rates_to_try = [48_000_u32, 44_100, 16_000];
    let mut last_err = String::new();
    let mut capture_result = None;

    for &rate in &rates_to_try {
        match AudioCapture::start(CaptureConfig::for_room("listen").with_sample_rate(rate)) {
            Ok(result) => { capture_result = Some((result, rate)); break; }
            Err(e) => last_err = e.to_string(),
        }
    }

    let ((_capture, mut handle), sample_rate) = capture_result
        .ok_or_else(|| anyhow::anyhow!("aucun sample rate supporte : {last_err}"))?;

    println!("  [Listen] Capture {sample_rate}Hz → 16kHz → rustpotter ({samples_per_frame} samples/frame)");

    let mut carry: Vec<f32> = Vec::with_capacity(samples_per_frame * 4);
    let mut frames_processed: u64 = 0;
    let mut last_diag = std::time::Instant::now();
    let mut total_received: u64 = 0;
    let mut best_score: f32 = 0.0;

    loop {
        // Press-to-speak : Entrée → activation immédiate
        if manual_trigger.swap(false, Ordering::Relaxed) {
            handle.stop();
            return Ok(("__manual__".to_string(), 1.0));
        }

        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        let raw = handle.read_available();
        if raw.is_empty() { continue; }

        total_received += raw.len() as u64;
        let samples = resample_avg(&raw, sample_rate, 16_000);
        carry.extend_from_slice(&samples);

        while carry.len() >= samples_per_frame {
            let frame: Vec<f32> = carry.drain(..samples_per_frame).collect();
            frames_processed += 1;

            if let Some(det) = rp.process_samples(frame) {
                if det.score > best_score { best_score = det.score; }
                if det.score > 0.1 {
                    println!("  [WW] score={:.3}  avg={:.3}  seuil={:.2}  mot={}", det.score, det.avg_score, threshold, det.name);
                }
                if det.score >= threshold {
                    handle.stop();
                    return Ok((det.name, det.score));
                }
            }
        }

        // Log diagnostic toutes les 5s
        if last_diag.elapsed().as_secs() >= 5 {
            println!(
                "  [Listen] {frames_processed} frames | {} samples | meilleur score: {:.3}",
                total_received, best_score
            );
            last_diag = std::time::Instant::now();
            best_score = 0.0;
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

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

/// Bip de confirmation 880Hz / 150ms.
fn play_chime() {
    let sr = 22_050_u32;
    let n = (sr as usize * 150) / 1000;
    let freq = 880.0_f32;

    let mut wav = Vec::with_capacity(44 + n * 2);
    let data_size = (n * 2) as u32;
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(data_size + 36).to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16_u32.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&sr.to_le_bytes());
    wav.extend_from_slice(&(sr * 2).to_le_bytes());
    wav.extend_from_slice(&2_u16.to_le_bytes());
    wav.extend_from_slice(&16_u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_size.to_le_bytes());

    for i in 0..n {
        let t = i as f32 / sr as f32;
        let env = if t < 0.01 { t / 0.01 } else if t > 0.12 { (0.15 - t) / 0.03 } else { 1.0 };
        let s = (0.3 * env * (2.0 * std::f32::consts::PI * freq * t).sin() * 32767.0) as i16;
        wav.extend_from_slice(&s.to_le_bytes());
    }

    voice::play_wav(&wav);
}
