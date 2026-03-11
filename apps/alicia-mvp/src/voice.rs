//! Mode voix : micro -> STT -> NLU -> intent -> TTS -> haut-parleur.

use std::io::{self, Write as _};
use std::time::Instant;

use miyualicia::intent::Intent;

use crate::runtime::{AliciaRuntime, ServiceArgs};

// ---------------------------------------------------------------------------
// Capture micro (VAD-based)
// ---------------------------------------------------------------------------

/// Capture depuis le micro avec detection d'activite vocale.
pub async fn capture_speech() -> anyhow::Result<Vec<f32>> {
    use miyualicia_capture::{AudioCapture, CaptureConfig, VoiceActivityDetector};

    let rates_to_try = [16_000_u32, 48_000, 44_100];
    let mut last_err = String::new();
    let mut capture_result = None;

    for &rate in &rates_to_try {
        let config = CaptureConfig::for_room("mvp-test").with_sample_rate(rate);
        match AudioCapture::start(config) {
            Ok(result) => {
                println!("  [Mic] Capture a {rate}Hz");
                capture_result = Some((result, rate));
                break;
            }
            Err(e) => {
                last_err = e.to_string();
            }
        }
    }

    let ((_capture, mut handle), sample_rate) = capture_result
        .ok_or_else(|| anyhow::anyhow!("aucun sample rate supporte : {last_err}"))?;

    let mut vad = VoiceActivityDetector::new();
    let mut recording = false;
    let mut recorded_samples: Vec<f32> = Vec::new();
    let max_duration = std::time::Duration::from_secs(5);
    let silence_timeout = std::time::Duration::from_millis(800);
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
                miyualicia_capture::VadState::Speech
                | miyualicia_capture::VadState::MaybeSpeech => {
                    if !recording {
                        println!("  [VAD] Parole detectee !");
                        recording = true;
                    }
                    last_speech = Instant::now();
                    recorded_samples.extend_from_slice(chunk);
                }
                miyualicia_capture::VadState::MaybeEnd => {
                    if recording {
                        recorded_samples.extend_from_slice(chunk);
                    }
                }
                miyualicia_capture::VadState::Silence => {
                    if recording {
                        recorded_samples.extend_from_slice(chunk);
                        if last_speech.elapsed() > silence_timeout {
                            println!("  [VAD] Fin de parole detectee");
                            break;
                        }
                    }
                }
            }
        }

        if recording && last_speech.elapsed() > silence_timeout {
            break;
        }
        if start.elapsed() > max_duration {
            if recording {
                println!("  [VAD] Timeout 5s atteint");
            } else {
                println!("  [VAD] Aucune parole detectee en 5s");
            }
            break;
        }
    }

    handle.stop();

    if recorded_samples.is_empty() {
        anyhow::bail!("aucune parole detectee");
    }

    let final_samples = if sample_rate != 16_000 {
        let ratio = sample_rate as f64 / 16_000.0;
        let out_len = (recorded_samples.len() as f64 / ratio) as usize;
        let mut resampled = Vec::with_capacity(out_len);
        for i in 0..out_len {
            let src_idx = (i as f64 * ratio) as usize;
            if src_idx < recorded_samples.len() {
                resampled.push(recorded_samples[src_idx]);
            }
        }
        println!(
            "  [Mic] Resample {sample_rate}Hz -> 16kHz ({} -> {} samples)",
            recorded_samples.len(),
            resampled.len()
        );
        resampled
    } else {
        recorded_samples
    };

    Ok(final_samples)
}

// ---------------------------------------------------------------------------
// Mock servers (embarques)
// ---------------------------------------------------------------------------

pub async fn start_mock_servers() -> anyhow::Result<(String, String)> {
    let stt_security = miyustt::SttSecurity {
        bearer_token: None,
        enforce_local_origin: false,
    };
    let stt_router = miyustt::router_with_state(miyustt::SttState {
        security: stt_security,
        ..Default::default()
    });

    let tts_security = miyutts::TtsSecurity {
        bearer_token: None,
        enforce_local_origin: false,
    };
    let tts_router = miyutts::router_with_state(miyutts::TtsState {
        security: tts_security,
        ..Default::default()
    });

    let stt_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let stt_addr = format!("http://{}", stt_listener.local_addr()?);
    tokio::spawn(async move {
        let _ = axum::serve(stt_listener, stt_router).await;
    });

    let tts_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let tts_addr = format!("http://{}", tts_listener.local_addr()?);
    tokio::spawn(async move {
        let _ = axum::serve(tts_listener, tts_router).await;
    });

    Ok((stt_addr, tts_addr))
}

// ---------------------------------------------------------------------------
// HTTP clients STT / TTS
// ---------------------------------------------------------------------------

pub async fn call_stt(
    client: &reqwest::Client,
    base_url: &str,
    samples: &[f32],
) -> anyhow::Result<String> {
    let url = format!("{base_url}/api/stt");
    println!("  [STT] {} samples ({:.1} KB)", samples.len(), samples.len() as f64 * 4.0 / 1024.0);
    let body = serde_json::json!({
        "samples": samples,
        "sample_rate": 16000,
        "language": "fr"
    });

    let resp = match client
        .post(&url)
        .json(&body)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            anyhow::bail!(
                "STT send failed: {e:#} (is_connect={}, is_timeout={}, is_request={})",
                e.is_connect(),
                e.is_timeout(),
                e.is_request()
            );
        }
    };

    if !resp.status().is_success() {
        let status = resp.status();
        let body_text = resp.text().await.unwrap_or_default();
        anyhow::bail!("STT HTTP {status}: {body_text}");
    }

    let json: serde_json::Value = resp.json().await?;
    Ok(json["transcript"]
        .as_str()
        .unwrap_or("(pas de transcription)")
        .to_string())
}

pub async fn call_tts(
    client: &reqwest::Client,
    base_url: &str,
    text: &str,
) -> anyhow::Result<Vec<u8>> {
    let url = format!("{base_url}/api/tts/wav");
    let body = serde_json::json!({
        "text": text,
        "voice": "fr_female_01_compact",
        "format": "wav",
        "language": "fr",
        "sample_rate": 22050
    });

    let resp = client
        .post(&url)
        .json(&body)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("TTS HTTP {}", resp.status());
    }

    Ok(resp.bytes().await?.to_vec())
}

// ---------------------------------------------------------------------------
// Audio playback
// ---------------------------------------------------------------------------

pub fn play_wav(wav_bytes: &[u8]) {
    if wav_bytes.len() < 44 {
        println!("  [Audio] WAV trop court, skip playback");
        return;
    }

    let cursor = io::Cursor::new(wav_bytes.to_vec());
    match rodio::OutputStreamBuilder::open_default_stream() {
        Ok(stream) => {
            let sink = rodio::Sink::connect_new(stream.mixer());
            match rodio::Decoder::new(cursor) {
                Ok(source) => {
                    sink.append(source);
                    println!("  [Audio] Lecture en cours...");
                    sink.sleep_until_end();
                    println!("  [Audio] Lecture terminee");
                }
                Err(e) => println!("  [Audio] Erreur decodage WAV : {e}"),
            }
        }
        Err(e) => println!("  [Audio] Pas de peripherique audio : {e}"),
    }
}

// ---------------------------------------------------------------------------
// Mode voix (boucle principale)
// ---------------------------------------------------------------------------

pub async fn run_voice_mode(
    stt_url: String,
    tts_url: String,
    llm_url: Option<String>,
    llm_key: Option<String>,
    face_url: Option<String>,
    search_url: Option<String>,
    workspace: Option<std::path::PathBuf>,
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

    println!(
        "=== Alicia MVP — Mode Voix [NLU: {}] ===",
        rt.nlu.label()
    );
    println!("  STT : {}", rt.stt_url);
    println!("  TTS : {}", rt.tts_url);
    println!("\nTape du texte ou 'mic' pour capturer. 'quit' pour quitter.\n");

    let stdin = std::io::stdin();
    loop {
        print!("alicia-voice> ");
        io::stdout().flush()?;

        let mut line = String::new();
        if std::io::BufRead::read_line(&mut stdin.lock(), &mut line)? == 0 {
            break;
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "quit" || line == "exit" {
            println!("Au revoir !");
            break;
        }

        let start = Instant::now();

        // STT
        let transcript = if line == "mic" {
            println!("  [Mic] Capture micro... parle maintenant ! (5s max)");
            match capture_speech().await {
                Ok(samples) => {
                    println!(
                        "  [Mic] {} samples ({:.1}s) — envoi STT...",
                        samples.len(),
                        samples.len() as f64 / 16_000.0
                    );
                    match call_stt(&rt.http, &rt.stt_url, &samples).await {
                        Ok(t) => { println!("  [STT] \"{t}\""); t }
                        Err(e) => { println!("  [STT] Erreur : {e:#}"); continue; }
                    }
                }
                Err(e) => { println!("  [Mic] Erreur : {e}"); continue; }
            }
        } else {
            println!("  [STT] Simulation : \"{line}\"");
            line.to_string()
        };
        println!("  [STT] Latence : {:?}", start.elapsed());

        // NLU
        let nlu_start = Instant::now();
        let (intent, raw_nlu) = rt.nlu.parse(&transcript).await;
        println!("  [NLU-{}] {} → {:?} ({:?})", rt.nlu.label(), raw_nlu, intent, nlu_start.elapsed());

        // Dispatch
        rt.dispatch(&intent, &transcript).await;

        // Response + TTS
        let response_text = rt.nlu.generate_response(&intent, &transcript).await;
        println!("  [Alicia] \"{response_text}\"");
        match call_tts(&rt.http, &rt.tts_url, &response_text).await {
            Ok(wav) => { println!("  [TTS] {} octets", wav.len()); play_wav(&wav); }
            Err(e) => println!("  [TTS] Erreur : {e}"),
        }

        println!("  [Total] {:?}\n", start.elapsed());
    }

    Ok(())
}
