//! Vue de configuration et test Miyukini Whisper (V3 squelette).

use dioxus::prelude::*;
use std::time::Duration;

/// Panneau de test/configuration Miyukini Whisper dans Central.
#[component]
pub fn MiyukiniWhisperView() -> Element {
    let mut stt_text = use_signal(String::new);
    let mut tts_text = use_signal(|| "Bonjour depuis Miyukini Whisper.".to_string());
    let mut transcript = use_signal(|| "Aucune transcription pour le moment.".to_string());
    let mut preset = use_signal(|| "balanced".to_string());
    let mut fallback = use_signal(|| "local_only".to_string());
    let mut output_profile = use_signal(|| "verbatim".to_string());
    let mut language = use_signal(|| "auto".to_string());
    let mut stt_base_url = use_signal(|| "http://127.0.0.1:3003".to_string());
    let mut tts_base_url = use_signal(|| "http://127.0.0.1:3004".to_string());
    let mut stt_health = use_signal(|| "Non teste".to_string());
    let mut tts_health = use_signal(|| "Non teste".to_string());
    let mut diag_error = use_signal(String::new);

    rsx! {
        div {
            style: "padding: 18px; display: flex; flex-direction: column; gap: 14px;",

            h2 { style: "margin: 0; font-size: 20px;", "Miyukini Whisper" }
            p {
                style: "margin: 0; font-size: 13px; opacity: 0.85;",
                "Test STT/TTS local, presets hardware et fallback opt-in."
            }

            div {
                style: "display:grid; grid-template-columns: 1fr 1fr; gap: 14px;",

                section {
                    style: "border:1px solid #2b3440; border-radius:10px; padding:12px;",
                    h3 { style: "margin-top:0;", "STT Test" }
                    p { style: "font-size:12px; opacity:0.8;", "Langue active" }
                    select {
                        value: "{language}",
                        onchange: move |evt| language.set(evt.value()),
                        option { value: "auto", "Auto" }
                        option { value: "fr", "Francais" }
                        option { value: "en", "English" }
                    }
                    textarea {
                        style: "width:100%; min-height:84px; margin-top:8px;",
                        value: "{stt_text}",
                        oninput: move |evt| stt_text.set(evt.value()),
                        placeholder: "Colle ici un texte de simulation STT..."
                    }
                    button {
                        style: "margin-top:8px;",
                        onclick: move |_| {
                            let src = stt_text.read().trim().to_string();
                            if src.is_empty() {
                                transcript.set("Aucune entree STT.".to_string());
                            } else {
                                transcript.set(format!("[{}] {}", language.read(), src));
                            }
                        },
                        "Simuler transcription"
                    }
                    p { style: "font-size:12px; margin-top:10px;", "Resultat" }
                    pre {
                        style: "white-space:pre-wrap; background:#111722; padding:8px; border-radius:8px;",
                        "{transcript}"
                    }
                }

                section {
                    style: "border:1px solid #2b3440; border-radius:10px; padding:12px;",
                    h3 { style: "margin-top:0;", "TTS Test" }
                    textarea {
                        style: "width:100%; min-height:84px;",
                        value: "{tts_text}",
                        oninput: move |evt| tts_text.set(evt.value()),
                        placeholder: "Texte a vocaliser"
                    }
                    button {
                        style: "margin-top:8px;",
                        onclick: move |_| {
                            // V3: simulation locale uniquement.
                            tracing::info!("Miyukini Whisper TTS test: {}", tts_text.read());
                        },
                        "Simuler lecture TTS"
                    }
                }
            }

            section {
                style: "border:1px solid #2b3440; border-radius:10px; padding:12px;",
                h3 { style: "margin-top:0;", "Presets hardware, profil texte et fallback" }
                div {
                    style: "display:grid; grid-template-columns: 1fr 1fr 1fr; gap: 10px;",
                    div {
                        p { style: "font-size:12px; opacity:0.8;", "Preset" }
                        select {
                            value: "{preset}",
                            onchange: move |evt| preset.set(evt.value()),
                            option { value: "compact", "Compact" }
                            option { value: "balanced", "Balanced" }
                            option { value: "precision", "Precision" }
                        }
                    }
                    div {
                        p { style: "font-size:12px; opacity:0.8;", "Profil sortie" }
                        select {
                            value: "{output_profile}",
                            onchange: move |evt| output_profile.set(evt.value()),
                            option { value: "verbatim", "Verbatim" }
                            option { value: "clean", "Clean" }
                            option { value: "rewrite", "Rewrite" }
                        }
                    }
                    div {
                        p { style: "font-size:12px; opacity:0.8;", "Fallback" }
                        select {
                            value: "{fallback}",
                            onchange: move |evt| fallback.set(evt.value()),
                            option { value: "local_only", "Local only (defaut)" }
                            option { value: "host_bridge", "Host bridge (opt-in)" }
                            option { value: "host_bridge_and_cloud", "Host + Cloud (opt-in)" }
                        }
                    }
                }
                p {
                    style: "font-size:12px; margin-top:10px; opacity:0.8;",
                    "Etat actuel: preset={preset} | profile={output_profile} | fallback={fallback}"
                }
            }

            section {
                style: "border:1px solid #2b3440; border-radius:10px; padding:12px;",
                h3 { style: "margin-top:0;", "Diagnostics" }
                div {
                    style: "display:grid; grid-template-columns: 1fr 1fr; gap: 10px;",
                    div {
                        p { style: "font-size:12px; opacity:0.8;", "STT Base URL" }
                        input {
                            r#type: "text",
                            style: "width:100%;",
                            value: "{stt_base_url}",
                            oninput: move |evt| stt_base_url.set(evt.value())
                        }
                    }
                    div {
                        p { style: "font-size:12px; opacity:0.8;", "TTS Base URL" }
                        input {
                            r#type: "text",
                            style: "width:100%;",
                            value: "{tts_base_url}",
                            oninput: move |evt| tts_base_url.set(evt.value())
                        }
                    }
                }
                button {
                    style: "margin-top:10px;",
                    onclick: move |_| {
                        let stt_url = stt_base_url.read().clone();
                        let tts_url = tts_base_url.read().clone();
                        let mut stt_health_sig = stt_health;
                        let mut tts_health_sig = tts_health;
                        let mut diag_error_sig = diag_error;
                        spawn(async move {
                            stt_health_sig.set("Test en cours...".to_string());
                            tts_health_sig.set("Test en cours...".to_string());
                            diag_error_sig.set(String::new());

                            let stt = fetch_health(&stt_url).await;
                            let tts = fetch_health(&tts_url).await;

                            match stt {
                                Ok(msg) => stt_health_sig.set(msg),
                                Err(err) => stt_health_sig.set(format!("Erreur: {err}")),
                            }
                            match tts {
                                Ok(msg) => tts_health_sig.set(msg),
                                Err(err) => tts_health_sig.set(format!("Erreur: {err}")),
                            }

                            if stt.is_err() || tts.is_err() {
                                diag_error_sig.set("Un ou plusieurs services sont injoignables.".to_string());
                            }
                        });
                    },
                    "Tester /api/health"
                }
                div {
                    style: "margin-top:10px; display:grid; grid-template-columns: 1fr 1fr; gap:10px;",
                    div {
                        p { style: "font-size:12px; opacity:0.8;", "STT health" }
                        pre {
                            style: "white-space:pre-wrap; background:#111722; padding:8px; border-radius:8px;",
                            "{stt_health}"
                        }
                    }
                    div {
                        p { style: "font-size:12px; opacity:0.8;", "TTS health" }
                        pre {
                            style: "white-space:pre-wrap; background:#111722; padding:8px; border-radius:8px;",
                            "{tts_health}"
                        }
                    }
                }
                if !diag_error.read().is_empty() {
                    p {
                        style: "font-size:12px; color:#fca5a5;",
                        "{diag_error}"
                    }
                }
            }
        }
    }
}

async fn fetch_health(base_url: &str) -> Result<String, String> {
    let url = format!("{}/api/health", base_url.trim_end_matches('/'));
    let response = reqwest::Client::new()
        .get(url)
        .timeout(Duration::from_secs(4))
        .send()
        .await
        .map_err(|e| format!("HTTP error: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("status {}", response.status()));
    }

    let payload: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("JSON error: {e}"))?;

    let service = payload
        .get("service")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown");
    let status = payload
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown");
    let version = payload
        .get("version")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("?");

    Ok(format!("{service} | {status} | v{version}"))
}
