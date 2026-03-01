//! Adaptateur generique JSON pour les dispositifs HTTP locaux.
//!
//! Permet de configurer le mapping entre les champs JSON de la reponse
//! et les champs de `DeviceState`. Utile pour les dispositifs qui
//! n'ont pas d'adaptateur dedie.

// @id: toolkit.alicia.http.adapters.generic
// @role: generic_device_adapter
// @layer: 6
// @human: Adaptateur JSON generique configurable pour dispositifs HTTP locaux.
// @do: translate_generic_json_to_device_state

use chrono::Utc;
use miyualicia_devices::DeviceState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::HttpError;

/// Configuration du mapping JSON generique.
///
/// Chaque champ optionnel indique le chemin JSON (dot notation simple)
/// vers la valeur correspondante dans la reponse du dispositif.
///
/// # Exemple
///
/// ```json
/// {
///     "on_field": "state.on",
///     "brightness_field": "state.brightness",
///     "temperature_field": "sensors.temperature"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericFieldMapping {
    /// Chemin JSON vers le champ on/off (bool).
    pub on_field: Option<String>,
    /// Chemin JSON vers le champ brightness (0-100, u8).
    pub brightness_field: Option<String>,
    /// Chemin JSON vers la temperature courante (f32).
    pub temperature_field: Option<String>,
    /// Chemin JSON vers la puissance en watts (f32).
    pub power_field: Option<String>,
    /// Chemin JSON vers l'humidite (f32, 0-100).
    pub humidity_field: Option<String>,
    /// Chemin JSON vers la position (0-100, u8).
    pub position_field: Option<String>,
    /// Chemin JSON vers le champ locked (bool).
    pub locked_field: Option<String>,
    /// Chemin JSON vers le champ motion (bool).
    pub motion_field: Option<String>,
    /// Chemin JSON vers le champ contact (bool).
    pub contact_field: Option<String>,
}

impl Default for GenericFieldMapping {
    fn default() -> Self {
        Self {
            on_field: Some("on".to_string()),
            brightness_field: None,
            temperature_field: None,
            power_field: None,
            humidity_field: None,
            position_field: None,
            locked_field: None,
            motion_field: None,
            contact_field: None,
        }
    }
}

/// Extrait une valeur depuis un objet JSON en utilisant un chemin dot notation.
///
/// Supporte les chemins simples : "field", "parent.child", "a.b.c".
fn extract_json_path<'a>(json: &'a serde_json::Value, path: &str) -> Option<&'a serde_json::Value> {
    let mut current = json;
    for segment in path.split('.') {
        current = current.get(segment)?;
    }
    Some(current)
}

/// Parse une reponse JSON generique vers un `DeviceState` en utilisant le mapping configure.
pub fn parse_generic_response(
    device_id: Uuid,
    json: &serde_json::Value,
    mapping: &GenericFieldMapping,
) -> Result<DeviceState, HttpError> {
    let on = mapping
        .on_field
        .as_ref()
        .and_then(|path| extract_json_path(json, path))
        .and_then(serde_json::Value::as_bool);

    let brightness = mapping
        .brightness_field
        .as_ref()
        .and_then(|path| extract_json_path(json, path))
        .and_then(serde_json::Value::as_u64)
        .and_then(|v| u8::try_from(v.min(100)).ok());

    let temperature_current = mapping
        .temperature_field
        .as_ref()
        .and_then(|path| extract_json_path(json, path))
        .and_then(serde_json::Value::as_f64)
        .map(|v| v as f32);

    let power_w = mapping
        .power_field
        .as_ref()
        .and_then(|path| extract_json_path(json, path))
        .and_then(serde_json::Value::as_f64)
        .map(|v| v as f32);

    let humidity = mapping
        .humidity_field
        .as_ref()
        .and_then(|path| extract_json_path(json, path))
        .and_then(serde_json::Value::as_f64)
        .map(|v| v as f32);

    let position = mapping
        .position_field
        .as_ref()
        .and_then(|path| extract_json_path(json, path))
        .and_then(serde_json::Value::as_u64)
        .and_then(|v| u8::try_from(v.min(100)).ok());

    let locked = mapping
        .locked_field
        .as_ref()
        .and_then(|path| extract_json_path(json, path))
        .and_then(serde_json::Value::as_bool);

    let motion = mapping
        .motion_field
        .as_ref()
        .and_then(|path| extract_json_path(json, path))
        .and_then(serde_json::Value::as_bool);

    let contact = mapping
        .contact_field
        .as_ref()
        .and_then(|path| extract_json_path(json, path))
        .and_then(serde_json::Value::as_bool);

    Ok(DeviceState {
        device_id,
        on,
        brightness,
        color_rgb: None,
        position,
        temperature_current,
        temperature_target: None,
        power_w,
        locked,
        motion,
        contact,
        humidity,
        updated_at: Utc::now(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_path_simple() {
        let json = serde_json::json!({"on": true, "brightness": 80});
        assert_eq!(extract_json_path(&json, "on"), Some(&serde_json::json!(true)));
        assert_eq!(
            extract_json_path(&json, "brightness"),
            Some(&serde_json::json!(80))
        );
        assert!(extract_json_path(&json, "missing").is_none());
    }

    #[test]
    fn test_extract_json_path_nested() {
        let json = serde_json::json!({
            "state": {"on": true},
            "sensors": {"temperature": 21.5}
        });
        assert_eq!(
            extract_json_path(&json, "state.on"),
            Some(&serde_json::json!(true))
        );
        assert_eq!(
            extract_json_path(&json, "sensors.temperature"),
            Some(&serde_json::json!(21.5))
        );
    }

    #[test]
    fn test_parse_generic_response_default_mapping() {
        let json = serde_json::json!({"on": true});
        let mapping = GenericFieldMapping::default();
        let id = Uuid::new_v4();
        let state = parse_generic_response(id, &json, &mapping).unwrap();
        assert_eq!(state.device_id, id);
        assert_eq!(state.on, Some(true));
    }

    #[test]
    fn test_parse_generic_response_custom_mapping() {
        let json = serde_json::json!({
            "device": {
                "power": true,
                "level": 75,
                "temp": 22.0,
                "watts": 15.5
            }
        });
        let mapping = GenericFieldMapping {
            on_field: Some("device.power".to_string()),
            brightness_field: Some("device.level".to_string()),
            temperature_field: Some("device.temp".to_string()),
            power_field: Some("device.watts".to_string()),
            humidity_field: None,
            position_field: None,
            locked_field: None,
            motion_field: None,
            contact_field: None,
        };
        let state = parse_generic_response(Uuid::new_v4(), &json, &mapping).unwrap();
        assert_eq!(state.on, Some(true));
        assert_eq!(state.brightness, Some(75));
        assert!((state.temperature_current.unwrap() - 22.0).abs() < f32::EPSILON);
        assert!((state.power_w.unwrap() - 15.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_parse_generic_response_missing_fields() {
        let json = serde_json::json!({});
        let mapping = GenericFieldMapping::default();
        let state = parse_generic_response(Uuid::new_v4(), &json, &mapping).unwrap();
        assert!(state.on.is_none());
        assert!(state.brightness.is_none());
    }

    #[test]
    fn test_parse_generic_response_all_fields() {
        let json = serde_json::json!({
            "on": false,
            "bright": 50,
            "temp": 19.5,
            "power": 100.0,
            "hum": 65.0,
            "pos": 30,
            "lock": true,
            "move": false,
            "contact": true
        });
        let mapping = GenericFieldMapping {
            on_field: Some("on".to_string()),
            brightness_field: Some("bright".to_string()),
            temperature_field: Some("temp".to_string()),
            power_field: Some("power".to_string()),
            humidity_field: Some("hum".to_string()),
            position_field: Some("pos".to_string()),
            locked_field: Some("lock".to_string()),
            motion_field: Some("move".to_string()),
            contact_field: Some("contact".to_string()),
        };
        let state = parse_generic_response(Uuid::new_v4(), &json, &mapping).unwrap();
        assert_eq!(state.on, Some(false));
        assert_eq!(state.brightness, Some(50));
        assert!((state.temperature_current.unwrap() - 19.5).abs() < f32::EPSILON);
        assert!((state.power_w.unwrap() - 100.0).abs() < f32::EPSILON);
        assert!((state.humidity.unwrap() - 65.0).abs() < f32::EPSILON);
        assert_eq!(state.position, Some(30));
        assert_eq!(state.locked, Some(true));
        assert_eq!(state.motion, Some(false));
        assert_eq!(state.contact, Some(true));
    }

    #[test]
    fn test_field_mapping_serde_roundtrip() {
        let mapping = GenericFieldMapping::default();
        let json = serde_json::to_string(&mapping).expect("serialize");
        let back: GenericFieldMapping = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.on_field, mapping.on_field);
    }
}
