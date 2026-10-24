//! @id     alicia_mvp_voice
//! @do     capture_speech
//! @role   orchestration
//! @layer  app
//! @human  alicia-mvp — module « voice ».

//! Mode voix : micro -> STT -> NLU -> intent -> TTS -> haut-parleur.

use std::io::{self, Write as _};
use std::sync::Arc;
use std::time::Instant;

use miyauth::http::service::{charger_ou_creer_cle, encoder_cle_publique, entetes_signees};
use miyusceau::CleSignature;

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

/// Adresse d'ecoute des serveurs mock embarques : loopback, port ephemere (C-21).
pub const MOCK_BIND_ADDR: &str = "127.0.0.1:0";

/// Variable d'environnement : chemin du fichier de graine (0600) de la cle
/// d'Alicia pour les serveurs STT/TTS externes (lot 2 : plus de jeton partage).
pub const VOICE_KEY_ENV: &str = "ALICIA_VOICE_KEY_FILE";

/// Chemin par defaut de la graine de la cle d'Alicia (cree une seule fois, 0600).
pub const DEFAULT_VOICE_KEY_PATH: &str = "keys/alicia-mvp.seed";

/// Serveurs mock STT/TTS demarres, avec la cle qui seule y est epinglee.
pub struct MockServers {
    pub stt_url: String,
    pub tts_url: String,
    /// Cle ephemere propre au processus : seule appelante epinglee par les deux serveurs.
    pub key: Arc<CleSignature>,
}

/// Charge (ou cree une fois, 0600) la cle d'Alicia pour les serveurs externes et
/// affiche sa cle publique, a epingler cote STT/TTS.
pub fn external_voice_key() -> anyhow::Result<Arc<CleSignature>> {
    let path = std::env::var(VOICE_KEY_ENV)
        .ok()
        .filter(|p| !p.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_VOICE_KEY_PATH.to_string());
    let key = charger_ou_creer_cle(std::path::Path::new(&path))
        .map_err(|e| anyhow::anyhow!("cle d'Alicia ({path}) : {e}"))?;
    println!(
        "  [Voix] Cle publique d'Alicia (a epingler sur STT/TTS) : {}",
        encoder_cle_publique(&key.cle_publique())
    );
    Ok(Arc::new(key))
}

/// Construit les routeurs STT/TTS proteges (seule `caller` est epinglee, origine locale stricte).
fn mock_routers(caller: [u8; 32]) -> (axum::Router, axum::Router, miyustt::SttSecurity, miyutts::TtsSecurity) {
    let stt_security = miyustt::SttSecurity::avec_appelants([caller], true);
    let stt_router = miyustt::router_with_state(miyustt::SttState {
        security: stt_security.clone(),
        ..Default::default()
    });

    let tts_security = miyutts::TtsSecurity::avec_appelants([caller], true);
    let tts_router = miyutts::router_with_state(miyutts::TtsState {
        security: tts_security.clone(),
        ..Default::default()
    });
    (stt_router, tts_router, stt_security, tts_security)
}

/// Demarre les serveurs mock STT/TTS sur `bind_addr`.
///
/// Fail-closed : l'adresse est validee par `validate_bind` (hors loopback sans
/// appelant epingle = refus) avant toute ouverture de port.
pub async fn start_mock_servers_on(bind_addr: &str) -> anyhow::Result<MockServers> {
    let key = Arc::new(CleSignature::generer());
    let (stt_router, tts_router, stt_security, tts_security) = mock_routers(key.cle_publique());

    let stt_addr = miyustt::validate_bind(bind_addr, &stt_security)?;
    let tts_addr = miyutts::validate_bind(bind_addr, &tts_security)?;

    let stt_listener = tokio::net::TcpListener::bind(stt_addr).await?;
    let stt_url = format!("http://{}", stt_listener.local_addr()?);
    tokio::spawn(async move {
        let _ = axum::serve(stt_listener, stt_router).await;
    });

    let tts_listener = tokio::net::TcpListener::bind(tts_addr).await?;
    let tts_url = format!("http://{}", tts_listener.local_addr()?);
    tokio::spawn(async move {
        let _ = axum::serve(tts_listener, tts_router).await;
    });

    Ok(MockServers { stt_url, tts_url, key })
}

/// Demarre les serveurs mock STT/TTS en loopback (defaut).
pub async fn start_mock_servers() -> anyhow::Result<MockServers> {
    start_mock_servers_on(MOCK_BIND_ADDR).await
}

// ---------------------------------------------------------------------------
// HTTP clients STT / TTS
// ---------------------------------------------------------------------------

/// POST JSON, signe par `key` (usage du service) si une cle est fournie.
fn signed_post(
    client: &reqwest::Client,
    url: &str,
    usage: &str,
    key: Option<&CleSignature>,
    body: &serde_json::Value,
) -> anyhow::Result<reqwest::RequestBuilder> {
    let bytes = serde_json::to_vec(body)?;
    let parsed = reqwest::Url::parse(url)?;
    let path = match parsed.query() {
        Some(q) => format!("{}?{q}", parsed.path()),
        None => parsed.path().to_string(),
    };
    let mut request = client
        .post(parsed)
        .header(reqwest::header::CONTENT_TYPE, "application/json");
    if let Some(key) = key {
        for (name, value) in entetes_signees(key, usage, "POST", &path, &bytes) {
            request = request.header(name, value);
        }
    }
    Ok(request.body(bytes))
}

pub async fn call_stt(
    client: &reqwest::Client,
    base_url: &str,
    key: Option<&CleSignature>,
    samples: &[f32],
) -> anyhow::Result<String> {
    let url = format!("{base_url}/api/stt");
    println!("  [STT] {} samples ({:.1} KB)", samples.len(), samples.len() as f64 * 4.0 / 1024.0);
    let body = serde_json::json!({
        "samples": samples,
        "sample_rate": 16000,
        "language": "fr"
    });

    let request = signed_post(client, &url, miyustt::USAGE, key, &body)?;
    let resp = match request
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
    key: Option<&CleSignature>,
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

    let request = signed_post(client, &url, miyutts::USAGE, key, &body)?;
    let resp = request
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
                    match call_stt(&rt.http, &rt.stt_url, rt.stt_key.as_deref(), &samples).await {
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
        match call_tts(&rt.http, &rt.tts_url, rt.tts_key.as_deref(), &response_text).await {
            Ok(wav) => { println!("  [TTS] {} octets", wav.len()); play_wav(&wav); }
            Err(e) => println!("  [TTS] Erreur : {e}"),
        }

        println!("  [Total] {:?}\n", start.elapsed());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stt_body() -> serde_json::Value {
        serde_json::json!({ "samples": [0.1, -0.1], "sample_rate": 16000, "language": "fr" })
    }

    #[tokio::test]
    async fn mock_servers_require_signature() {
        let mock = start_mock_servers().await.unwrap();
        let client = reqwest::Client::new();

        // Non signe : 401.
        let resp = client
            .post(format!("{}/api/stt", mock.stt_url))
            .json(&stt_body())
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), reqwest::StatusCode::UNAUTHORIZED);

        // Ancien jeton bearer : 401.
        let resp = client
            .post(format!("{}/api/tts/wav", mock.tts_url))
            .bearer_auth("mauvais")
            .json(&serde_json::json!({
                "text": "Bonjour", "voice": "", "format": "wav", "language": "fr"
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), reqwest::StatusCode::UNAUTHORIZED);

        // Cle non epinglee : refus.
        let other = CleSignature::generer();
        assert!(call_stt(&client, &mock.stt_url, Some(&other), &[0.1]).await.is_err());
        assert!(call_tts(&client, &mock.tts_url, Some(&other), "Bonjour").await.is_err());

        // Cle epinglee : 200 via les clients de l'application (aller-retour).
        let transcript = call_stt(&client, &mock.stt_url, Some(&mock.key), &[0.1, -0.1])
            .await
            .unwrap();
        assert_ne!(transcript, "");
        let wav = call_tts(&client, &mock.tts_url, Some(&mock.key), "Bonjour")
            .await
            .unwrap();
        assert!(wav.len() > 44);
        assert!(call_stt(&client, &mock.stt_url, None, &[0.1]).await.is_err());
    }

    #[tokio::test]
    async fn mock_servers_check_origin() {
        let mock = start_mock_servers().await.unwrap();
        let client = reqwest::Client::new();
        let port = mock.stt_url.rsplit(':').next().unwrap().to_string();
        let url = format!("{}/api/stt", mock.stt_url);

        let resp = signed_post(&client, &url, miyustt::USAGE, Some(&mock.key), &stt_body())
            .unwrap()
            .header("origin", "http://localhost.evil.com")
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), reqwest::StatusCode::FORBIDDEN);

        let resp = signed_post(&client, &url, miyustt::USAGE, Some(&mock.key), &stt_body())
            .unwrap()
            .header("origin", format!("http://localhost:{port}"))
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), reqwest::StatusCode::OK);
    }

    #[tokio::test]
    async fn mock_servers_bound_body_size() {
        let mock = start_mock_servers().await.unwrap();
        let client = reqwest::Client::new();
        let text = "a".repeat(miyutts::MAX_TTS_BODY_BYTES + 1);
        let body = serde_json::json!({
            "text": text, "voice": "", "format": "wav", "language": "fr"
        });
        let resp = signed_post(&client, &format!("{}/api/tts", mock.tts_url), miyutts::USAGE, Some(&mock.key), &body)
            .unwrap()
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), reqwest::StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[tokio::test]
    async fn mock_servers_listen_on_loopback_and_refuse_bad_bind() {
        let mock = start_mock_servers().await.unwrap();
        assert!(mock.stt_url.starts_with("http://127.0.0.1:"));
        assert!(mock.tts_url.starts_with("http://127.0.0.1:"));
        assert!(start_mock_servers_on("localhost:0").await.is_err());
    }
}
