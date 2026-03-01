//! Client NLU vers miou-llm-bridge pour STT et extraction d'intention.
//!
//! Si le bridge est indisponible, active automatiquement le fallback regex.

// @id: service.alicia.nlu-bridge
// @role: nlu_client
// @layer: 7
// @human: Client HTTP vers miou-llm-bridge pour STT et NLU, avec fallback regex.
// @do: transcribe_and_parse_voice_intent

use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::intent::Intent;
use crate::nlu_fallback::FallbackNluParser;

/// Erreurs specifiques au bridge NLU.
#[derive(Debug, thiserror::Error)]
pub enum NluError {
    /// Bridge NLU indisponible (connexion refusee ou code >= 500).
    #[error("bridge NLU indisponible (timeout ou connexion refusee)")]
    BridgeUnavailable,

    /// Timeout sur une requete NLU.
    #[error("timeout requete NLU apres {timeout_ms}ms")]
    BridgeTimeout {
        /// Duree du timeout en millisecondes.
        timeout_ms: u64,
    },

    /// Reponse du bridge invalide (JSON malformed, champs manquants).
    #[error("reponse bridge NLU invalide : {0}")]
    InvalidResponse(String),

    /// Erreur HTTP interne.
    #[error("erreur HTTP bridge NLU : {0}")]
    HttpError(String),
}

/// Configuration du bridge NLU.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NluBridgeConfig {
    /// URL de base du bridge. Defaut : "http://127.0.0.1:3003".
    pub base_url: String,
    /// Timeout STT en millisecondes. Defaut : 5000.
    pub stt_timeout_ms: u64,
    /// Timeout NLU en millisecondes. Defaut : 3000.
    pub nlu_timeout_ms: u64,
    /// Pieces configurees (pour le contexte NLU).
    pub known_rooms: Vec<String>,
    /// Routines connues (pour le contexte NLU).
    pub known_routines: Vec<String>,
}

impl Default for NluBridgeConfig {
    fn default() -> Self {
        Self {
            base_url: "http://127.0.0.1:3003".to_string(),
            stt_timeout_ms: 5000,
            nlu_timeout_ms: 3000,
            known_rooms: vec![
                "salon".to_string(),
                "chambre-parentale".to_string(),
                "chambre-theresa".to_string(),
                "chambre-eleanore".to_string(),
            ],
            known_routines: vec!["bonne nuit".to_string(), "je pars".to_string()],
        }
    }
}

/// Requete STT vers le bridge.
#[derive(Debug, Serialize)]
struct SttRequest<'a> {
    samples: &'a [f32],
    sample_rate: u32,
    model: &'static str,
    language: &'static str,
}

/// Reponse STT du bridge.
#[derive(Debug, Deserialize)]
struct SttResponse {
    transcript: String,
    #[allow(dead_code)]
    confidence: f64,
    #[allow(dead_code)]
    duration_ms: u64,
}

/// Requete NLU vers le bridge.
#[derive(Debug, Serialize)]
struct NluRequest<'a> {
    text: &'a str,
    context: &'static str,
    rooms: &'a [String],
    device_types: Vec<&'static str>,
    known_routines: &'a [String],
}

/// Reponse NLU du bridge.
#[derive(Debug, Deserialize)]
pub struct BridgeNluResponse {
    /// Intent identifiee par le bridge.
    pub intent: String,
    /// Confiance de la classification (0.0 - 1.0).
    #[allow(dead_code)]
    pub confidence: f64,
    /// Slots extraits par le NLU.
    pub slots: HashMap<String, serde_json::Value>,
}

/// Client NLU : appels HTTP vers miou-llm-bridge avec fallback regex.
#[derive(Debug, Clone)]
pub struct NluBridge {
    config: NluBridgeConfig,
    client: reqwest::Client,
    fallback: FallbackNluParser,
}

impl NluBridge {
    /// Cree un nouveau client NLU avec la configuration et le client HTTP donnes.
    pub fn new(config: NluBridgeConfig, client: reqwest::Client) -> Self {
        Self {
            config,
            client,
            fallback: FallbackNluParser,
        }
    }

    /// Transcrit des echantillons audio en texte via le bridge STT.
    ///
    /// Retourne `NluError::BridgeUnavailable` si le bridge est indisponible.
    /// Le fallback regex ne s'applique pas a la transcription audio.
    pub async fn transcribe(&self, samples: &[f32]) -> Result<String, NluError> {
        let url = format!("{}/api/stt", self.config.base_url);
        let request_id = uuid::Uuid::new_v4().to_string();

        let body = SttRequest {
            samples,
            sample_rate: 16000,
            model: "whisper-local",
            language: "fr",
        };

        let timeout = Duration::from_millis(self.config.stt_timeout_ms);

        let response = self
            .client
            .post(&url)
            .header("X-Request-ID", &request_id)
            .header("X-Source", "alicia-home")
            .timeout(timeout)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    NluError::BridgeTimeout {
                        timeout_ms: self.config.stt_timeout_ms,
                    }
                } else {
                    NluError::BridgeUnavailable
                }
            })?;

        let status = response.status();
        if status.is_server_error() || status.as_u16() == 503 {
            return Err(NluError::BridgeUnavailable);
        }
        if !status.is_success() {
            return Err(NluError::HttpError(format!("HTTP {status}")));
        }

        let stt_response: SttResponse = response.json().await.map_err(|e| {
            NluError::InvalidResponse(format!("erreur parsing reponse STT : {e}"))
        })?;

        Ok(stt_response.transcript)
    }

    /// Parse une transcription en intention domotique.
    ///
    /// Si le bridge est indisponible, active le `FallbackNluParser` (regex).
    /// Retourne toujours un `Intent`, jamais d'erreur.
    pub async fn parse_intent(&self, transcript: &str) -> Intent {
        let url = format!("{}/api/nlu", self.config.base_url);
        let request_id = uuid::Uuid::new_v4().to_string();

        let body = NluRequest {
            text: transcript,
            context: "home_automation",
            rooms: &self.config.known_rooms,
            device_types: vec![
                "light",
                "shutter",
                "thermostat",
                "outlet",
                "sensor",
                "lock",
            ],
            known_routines: &self.config.known_routines,
        };

        let timeout = Duration::from_millis(self.config.nlu_timeout_ms);

        let result = self
            .client
            .post(&url)
            .header("X-Request-ID", &request_id)
            .header("X-Source", "alicia-home")
            .timeout(timeout)
            .json(&body)
            .send()
            .await;

        match result {
            Ok(response) if response.status().is_success() => {
                match response.json::<BridgeNluResponse>().await {
                    Ok(nlu_response) => parse_bridge_response(&nlu_response, transcript),
                    Err(e) => {
                        tracing::warn!(
                            "Reponse NLU invalide, fallback regex active : {e}"
                        );
                        self.fallback.parse(transcript)
                    }
                }
            }
            Ok(response) => {
                tracing::warn!(
                    "Bridge NLU erreur HTTP {}, fallback regex active",
                    response.status()
                );
                self.fallback.parse(transcript)
            }
            Err(e) => {
                tracing::warn!("Bridge NLU indisponible ({e}), fallback regex active");
                self.fallback.parse(transcript)
            }
        }
    }

    /// Verifie si le bridge est joignable (health check).
    pub async fn is_available(&self) -> bool {
        let url = format!("{}/api/health", self.config.base_url);
        let timeout = Duration::from_millis(1000);

        match self.client.get(&url).timeout(timeout).send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }
}

/// Convertit une reponse NLU du bridge en `Intent`.
fn parse_bridge_response(response: &BridgeNluResponse, transcript: &str) -> Intent {
    match response.intent.as_str() {
        "control_device" => Intent::ControlDevice {
            device_type: response
                .slots
                .get("device_type")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string(),
            room_id: response
                .slots
                .get("room_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            action: response
                .slots
                .get("action")
                .and_then(|v| v.as_str())
                .unwrap_or("on")
                .to_string(),
            value: response.slots.get("value").cloned(),
        },
        "activate_routine" => Intent::ActivateRoutine {
            routine_name: response
                .slots
                .get("routine_name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        },
        "query_state" => Intent::QueryState {
            target: response
                .slots
                .get("target")
                .and_then(|v| v.as_str())
                .unwrap_or("home")
                .to_string(),
            property: response
                .slots
                .get("property")
                .and_then(|v| v.as_str())
                .map(String::from),
        },
        "query_weather" => Intent::QueryWeather {
            location: response
                .slots
                .get("location")
                .and_then(|v| v.as_str())
                .map(String::from),
            horizon: response
                .slots
                .get("horizon")
                .and_then(|v| v.as_str())
                .map(String::from),
        },
        "help" => Intent::Help {
            topic: response
                .slots
                .get("topic")
                .and_then(|v| v.as_str())
                .map(String::from),
        },
        _ => Intent::Unknown {
            transcript: transcript.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bridge_response_control_device() {
        let response = BridgeNluResponse {
            intent: "control_device".to_string(),
            confidence: 0.97,
            slots: {
                let mut m = HashMap::new();
                m.insert(
                    "device_type".to_string(),
                    serde_json::json!("light"),
                );
                m.insert("room_id".to_string(), serde_json::json!("salon"));
                m.insert("action".to_string(), serde_json::json!("on"));
                m
            },
        };
        let intent = parse_bridge_response(&response, "allume la lumiere du salon");
        match intent {
            Intent::ControlDevice {
                device_type,
                room_id,
                action,
                ..
            } => {
                assert_eq!(device_type, "light");
                assert_eq!(room_id, Some("salon".to_string()));
                assert_eq!(action, "on");
            }
            other => panic!("attendu ControlDevice, obtenu {:?}", other),
        }
    }

    #[test]
    fn test_parse_bridge_response_activate_routine() {
        let response = BridgeNluResponse {
            intent: "activate_routine".to_string(),
            confidence: 0.99,
            slots: {
                let mut m = HashMap::new();
                m.insert(
                    "routine_name".to_string(),
                    serde_json::json!("bonne nuit"),
                );
                m
            },
        };
        let intent = parse_bridge_response(&response, "bonne nuit");
        assert!(matches!(
            intent,
            Intent::ActivateRoutine { routine_name } if routine_name == "bonne nuit"
        ));
    }

    #[test]
    fn test_parse_bridge_response_unknown() {
        let response = BridgeNluResponse {
            intent: "unknown".to_string(),
            confidence: 0.0,
            slots: HashMap::new(),
        };
        let intent = parse_bridge_response(&response, "commande pizza");
        assert!(matches!(intent, Intent::Unknown { .. }));
    }

    #[test]
    fn test_nlu_bridge_config_default() {
        let config = NluBridgeConfig::default();
        assert_eq!(config.base_url, "http://127.0.0.1:3003");
        assert_eq!(config.stt_timeout_ms, 5000);
        assert_eq!(config.nlu_timeout_ms, 3000);
        assert_eq!(config.known_rooms.len(), 4);
        assert_eq!(config.known_routines.len(), 2);
    }

    #[tokio::test]
    async fn test_bridge_unavailable_parse_uses_fallback() {
        let config = NluBridgeConfig {
            base_url: "http://127.0.0.1:1".to_string(), // port inaccessible
            stt_timeout_ms: 100,
            nlu_timeout_ms: 100,
            ..Default::default()
        };
        let client = reqwest::Client::new();
        let bridge = NluBridge::new(config, client);

        // parse_intent ne retourne jamais d'erreur, fallback regex
        let intent = bridge.parse_intent("allume la lumiere du salon").await;
        match intent {
            Intent::ControlDevice {
                device_type,
                action,
                ..
            } => {
                assert_eq!(device_type, "light");
                assert_eq!(action, "on");
            }
            other => panic!(
                "attendu ControlDevice via fallback, obtenu {:?}",
                other
            ),
        }
    }

    #[tokio::test]
    async fn test_bridge_is_available_returns_false_when_unreachable() {
        let config = NluBridgeConfig {
            base_url: "http://127.0.0.1:1".to_string(),
            ..Default::default()
        };
        let client = reqwest::Client::new();
        let bridge = NluBridge::new(config, client);
        assert!(!bridge.is_available().await);
    }
}
