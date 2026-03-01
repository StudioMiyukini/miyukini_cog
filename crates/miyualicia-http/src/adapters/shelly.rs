//! Adaptateur pour les dispositifs Shelly (Gen1 et Gen2).
//!
//! # API Shelly Gen1
//!
//! - Etat relais : `GET http://{ip}/status` -> `{"relays":[{"ison":true,...}]}`
//! - Commande relais : `GET http://{ip}/relay/0?turn=on`
//!
//! # API Shelly Gen2
//!
//! - Etat switch : `POST http://{ip}/rpc` body `{"id":1,"method":"Switch.GetStatus","params":{"id":0}}`
//! - Commande switch : `POST http://{ip}/rpc` body `{"id":1,"method":"Switch.Set","params":{"id":0,"on":true}}`

// @id: toolkit.alicia.http.adapters.shelly
// @role: shelly_device_adapter
// @layer: 6
// @human: Parsing API Shelly Gen1/Gen2 vers DeviceState Alicia.
// @do: translate_shelly_responses_to_device_state

use chrono::Utc;
use miyualicia_devices::DeviceState;
use uuid::Uuid;

use crate::errors::HttpError;

/// Resultat du parsing d'une reponse Shelly Gen1 `/status`.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ShellyGen1Status {
    /// Liste des relais.
    #[serde(default)]
    pub relays: Vec<ShellyGen1Relay>,
    /// Mesures electriques (si disponibles).
    #[serde(default)]
    pub meters: Vec<ShellyGen1Meter>,
    /// Temperature interne du dispositif.
    pub temperature: Option<f32>,
}

/// Etat d'un relais Shelly Gen1.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ShellyGen1Relay {
    /// `true` si le relais est allume.
    pub ison: bool,
    /// `true` si le timer est actif.
    #[serde(default)]
    pub has_timer: bool,
}

/// Mesure electrique Shelly Gen1.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ShellyGen1Meter {
    /// Puissance instantanee en watts.
    pub power: f32,
}

/// Parse la reponse Shelly Gen1 `/status` vers un `DeviceState` Alicia.
///
/// Utilise le premier relais (index 0) comme source d'etat on/off.
/// Utilise le premier meter (index 0) comme source de puissance.
pub fn parse_gen1_status(
    device_id: Uuid,
    json: &serde_json::Value,
) -> Result<DeviceState, HttpError> {
    let status: ShellyGen1Status =
        serde_json::from_value(json.clone()).map_err(|e| HttpError::ParseError {
            url: "shelly/status".to_string(),
            message: e.to_string(),
        })?;

    let on = status.relays.first().map(|r| r.ison);
    let power_w = status.meters.first().map(|m| m.power);

    Ok(DeviceState {
        device_id,
        on,
        brightness: None,
        color_rgb: None,
        position: None,
        temperature_current: status.temperature,
        temperature_target: None,
        power_w,
        locked: None,
        motion: None,
        contact: None,
        humidity: None,
        updated_at: Utc::now(),
    })
}

/// Resultat du parsing d'une reponse Shelly Gen2 RPC `Switch.GetStatus`.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ShellyGen2SwitchStatus {
    /// `true` si le switch est allume.
    pub output: Option<bool>,
    /// Puissance instantanee en watts (si disponible).
    #[serde(rename = "apower")]
    pub active_power: Option<f32>,
    /// Temperature interne.
    pub temperature: Option<ShellyGen2Temp>,
}

/// Temperature Shelly Gen2.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ShellyGen2Temp {
    /// Temperature en Celsius.
    #[serde(rename = "tC")]
    pub celsius: f32,
}

/// Parse la reponse Shelly Gen2 `Switch.GetStatus` vers un `DeviceState` Alicia.
pub fn parse_gen2_switch_status(
    device_id: Uuid,
    json: &serde_json::Value,
) -> Result<DeviceState, HttpError> {
    // Le resultat est dans le champ "result" de la reponse RPC
    let result = json.get("result").unwrap_or(json);

    let status: ShellyGen2SwitchStatus =
        serde_json::from_value(result.clone()).map_err(|e| HttpError::ParseError {
            url: "shelly/rpc".to_string(),
            message: e.to_string(),
        })?;

    Ok(DeviceState {
        device_id,
        on: status.output,
        brightness: None,
        color_rgb: None,
        position: None,
        temperature_current: status.temperature.map(|t| t.celsius),
        temperature_target: None,
        power_w: status.active_power,
        locked: None,
        motion: None,
        contact: None,
        humidity: None,
        updated_at: Utc::now(),
    })
}

/// Construit l'URL de commande Shelly Gen1 pour un relais.
///
/// # Exemples
///
/// - `build_gen1_relay_url("192.168.1.100", 0, true)` -> `"http://192.168.1.100/relay/0?turn=on"`
/// - `build_gen1_relay_url("192.168.1.100", 0, false)` -> `"http://192.168.1.100/relay/0?turn=off"`
pub fn build_gen1_relay_url(host: &str, relay_index: u8, turn_on: bool) -> String {
    let action = if turn_on { "on" } else { "off" };
    format!("http://{host}/relay/{relay_index}?turn={action}")
}

/// Construit le body RPC Shelly Gen2 pour `Switch.Set`.
pub fn build_gen2_switch_set(switch_id: u8, on: bool) -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "method": "Switch.Set",
        "params": {
            "id": switch_id,
            "on": on
        }
    })
}

/// Construit le body RPC Shelly Gen2 pour `Switch.GetStatus`.
pub fn build_gen2_switch_get_status(switch_id: u8) -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "method": "Switch.GetStatus",
        "params": {
            "id": switch_id
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gen1_status_relay_on() {
        let json = serde_json::json!({
            "relays": [{"ison": true, "has_timer": false}],
            "meters": [{"power": 42.5}],
            "temperature": 35.0
        });
        let id = Uuid::new_v4();
        let state = parse_gen1_status(id, &json).unwrap();
        assert_eq!(state.device_id, id);
        assert_eq!(state.on, Some(true));
        assert!((state.power_w.unwrap() - 42.5).abs() < f32::EPSILON);
        assert!((state.temperature_current.unwrap() - 35.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_parse_gen1_status_relay_off_no_meter() {
        let json = serde_json::json!({
            "relays": [{"ison": false}]
        });
        let state = parse_gen1_status(Uuid::new_v4(), &json).unwrap();
        assert_eq!(state.on, Some(false));
        assert!(state.power_w.is_none());
        assert!(state.temperature_current.is_none());
    }

    #[test]
    fn test_parse_gen1_status_empty_relays() {
        let json = serde_json::json!({
            "relays": [],
            "meters": []
        });
        let state = parse_gen1_status(Uuid::new_v4(), &json).unwrap();
        assert!(state.on.is_none());
        assert!(state.power_w.is_none());
    }

    #[test]
    fn test_parse_gen2_switch_status() {
        let json = serde_json::json!({
            "result": {
                "output": true,
                "apower": 15.3,
                "temperature": {"tC": 28.5}
            }
        });
        let id = Uuid::new_v4();
        let state = parse_gen2_switch_status(id, &json).unwrap();
        assert_eq!(state.on, Some(true));
        assert!((state.power_w.unwrap() - 15.3).abs() < f32::EPSILON);
        assert!((state.temperature_current.unwrap() - 28.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_parse_gen2_switch_status_off() {
        let json = serde_json::json!({
            "result": {
                "output": false
            }
        });
        let state = parse_gen2_switch_status(Uuid::new_v4(), &json).unwrap();
        assert_eq!(state.on, Some(false));
        assert!(state.power_w.is_none());
    }

    #[test]
    fn test_build_gen1_relay_url_on() {
        let url = build_gen1_relay_url("192.168.1.100", 0, true);
        assert_eq!(url, "http://192.168.1.100/relay/0?turn=on");
    }

    #[test]
    fn test_build_gen1_relay_url_off() {
        let url = build_gen1_relay_url("192.168.1.100", 0, false);
        assert_eq!(url, "http://192.168.1.100/relay/0?turn=off");
    }

    #[test]
    fn test_build_gen2_switch_set() {
        let body = build_gen2_switch_set(0, true);
        assert_eq!(body["method"], "Switch.Set");
        assert_eq!(body["params"]["id"], 0);
        assert_eq!(body["params"]["on"], true);
    }

    #[test]
    fn test_build_gen2_switch_get_status() {
        let body = build_gen2_switch_get_status(1);
        assert_eq!(body["method"], "Switch.GetStatus");
        assert_eq!(body["params"]["id"], 1);
    }
}
