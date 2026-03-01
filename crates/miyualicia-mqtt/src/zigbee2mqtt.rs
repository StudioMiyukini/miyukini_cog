//! Module Zigbee2MQTT : parsing des etats Z2M vers `DeviceState` et construction des commandes.
//!
//! Reference : <https://www.zigbee2mqtt.io/guide/usage/exposes.html>

// @id: toolkit.alicia.mqtt.zigbee2mqtt
// @role: z2m_protocol_adapter
// @layer: 6
// @human: Parsing et construction des payloads Zigbee2MQTT pour Alicia.
// @do: translate_z2m_to_alicia_types

use chrono::Utc;
use miyualicia_devices::DeviceState;
use uuid::Uuid;

use crate::errors::MqttError;

/// Etat brut parse depuis un message Zigbee2MQTT.
///
/// Les champs correspondent aux proprietes standard des dispositifs Z2M.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Z2mState {
    /// "ON" ou "OFF"
    pub state: Option<String>,
    /// 0-254 (echelle Z2M, converti en 0-100 % dans `to_device_state`)
    pub brightness: Option<u8>,
    /// Temperature mesuree en degres Celsius.
    pub temperature: Option<f32>,
    /// Humidite en pourcentage (0-100).
    pub humidity: Option<f32>,
    /// `true` si mouvement detecte.
    pub occupancy: Option<bool>,
    /// `true` si contact ouvert (fenetre/porte ouverte).
    /// ATTENTION : semantique inversee par rapport a Alicia (true=ferme).
    pub contact: Option<bool>,
    /// Position du volet en % (0=ferme, 100=ouvert).
    pub position: Option<u8>,
    /// Consommation en watts.
    pub power: Option<f32>,
    /// Temperature cible thermostat en degres C.
    pub occupied_heating_setpoint: Option<f32>,
}

impl Z2mState {
    /// Convertit l'etat Z2M brut en `DeviceState` Alicia.
    ///
    /// # Conversions appliquees
    ///
    /// - `state` : "ON" -> `on = Some(true)`, "OFF" -> `on = Some(false)`
    /// - `brightness` : plage 0-254 Z2M -> plage 0-100 Alicia (x * 100 / 254)
    /// - `occupancy` : mappe vers `DeviceState::motion`
    /// - `contact` : inversion semantique Z2M (true=ouvert) -> Alicia (true=ferme)
    pub fn to_device_state(&self, device_id: Uuid) -> DeviceState {
        let on = self.state.as_deref().map(|s| s.eq_ignore_ascii_case("on"));

        // brightness : Z2M 0-254 -> Alicia 0-100
        let brightness = self.brightness.map(|b| {
            let value = u16::from(b) * 100 / 254;
            // Clamp au cas ou
            u8::try_from(value.min(100)).unwrap_or(100)
        });

        // contact : Z2M true=contact ferme -> Alicia true=ferme (meme semantique en fait)
        // Mais spec dit inversion : Z2M true=ouvert -> Alicia true=ferme
        let contact = self.contact.map(|c| !c);

        DeviceState {
            device_id,
            on,
            brightness,
            color_rgb: None, // Z2M utilise XY, pas RGB directement dans ce struct
            position: self.position,
            temperature_current: self.temperature,
            temperature_target: self.occupied_heating_setpoint,
            power_w: self.power,
            locked: None,
            motion: self.occupancy,
            contact,
            humidity: self.humidity,
            updated_at: Utc::now(),
        }
    }
}

/// Couleur RGB pour les dispositifs Z2M.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Z2mColor {
    /// Composante rouge (0-255).
    pub r: u8,
    /// Composante verte (0-255).
    pub g: u8,
    /// Composante bleue (0-255).
    pub b: u8,
}

/// Commande a envoyer vers un dispositif via Zigbee2MQTT.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Z2mCommand {
    /// "ON" ou "OFF"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Niveau de luminosite (0-254, echelle Z2M).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<u8>,

    /// Couleur RGB pour les dispositifs RGBW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Z2mColor>,

    /// Temperature cible thermostat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occupied_heating_setpoint: Option<f32>,

    /// Position volet (0-100 %).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u8>,
}

impl Z2mCommand {
    /// Construit une commande Z2M depuis une action Alicia generique.
    ///
    /// # Erreurs
    ///
    /// Retourne `MqttError::UnsupportedAction` si l'action n'a pas
    /// d'equivalent dans le protocole Z2M.
    pub fn from_device_command(
        action: &str,
        value: Option<&serde_json::Value>,
    ) -> Result<Self, MqttError> {
        match action {
            "on" => Ok(Self {
                state: Some("ON".to_string()),
                brightness: None,
                color: None,
                occupied_heating_setpoint: None,
                position: None,
            }),
            "off" => Ok(Self {
                state: Some("OFF".to_string()),
                brightness: None,
                color: None,
                occupied_heating_setpoint: None,
                position: None,
            }),
            "set_brightness" => {
                let pct = value
                    .and_then(serde_json::Value::as_u64)
                    .unwrap_or(0);
                // Alicia 0-100 -> Z2M 0-254
                let z2m_val = (pct * 254 / 100).min(254);
                let z2m_byte = u8::try_from(z2m_val).unwrap_or(254);
                Ok(Self {
                    state: None,
                    brightness: Some(z2m_byte),
                    color: None,
                    occupied_heating_setpoint: None,
                    position: None,
                })
            }
            "set_color" => {
                let arr = value.and_then(serde_json::Value::as_array);
                let (r, g, b) = if let Some(a) = arr {
                    let r = a.first().and_then(serde_json::Value::as_u64).unwrap_or(0);
                    let g = a.get(1).and_then(serde_json::Value::as_u64).unwrap_or(0);
                    let b = a.get(2).and_then(serde_json::Value::as_u64).unwrap_or(0);
                    (
                        u8::try_from(r.min(255)).unwrap_or(255),
                        u8::try_from(g.min(255)).unwrap_or(255),
                        u8::try_from(b.min(255)).unwrap_or(255),
                    )
                } else {
                    (0, 0, 0)
                };
                Ok(Self {
                    state: None,
                    brightness: None,
                    color: Some(Z2mColor { r, g, b }),
                    occupied_heating_setpoint: None,
                    position: None,
                })
            }
            "set_temperature" => {
                let temp = value
                    .and_then(serde_json::Value::as_f64)
                    .unwrap_or(20.0);
                #[allow(clippy::cast_possible_truncation)]
                let temp_f32 = temp as f32;
                Ok(Self {
                    state: None,
                    brightness: None,
                    color: None,
                    occupied_heating_setpoint: Some(temp_f32),
                    position: None,
                })
            }
            "open" => Ok(Self {
                state: None,
                brightness: None,
                color: None,
                occupied_heating_setpoint: None,
                position: Some(100),
            }),
            "close" => Ok(Self {
                state: None,
                brightness: None,
                color: None,
                occupied_heating_setpoint: None,
                position: Some(0),
            }),
            "set_position" => {
                let pos = value
                    .and_then(serde_json::Value::as_u64)
                    .unwrap_or(0);
                let pos_byte = u8::try_from(pos.min(100)).unwrap_or(100);
                Ok(Self {
                    state: None,
                    brightness: None,
                    color: None,
                    occupied_heating_setpoint: None,
                    position: Some(pos_byte),
                })
            }
            other => Err(MqttError::UnsupportedAction {
                action: other.to_string(),
            }),
        }
    }

    /// Serialise la commande en payload JSON publiable.
    pub fn to_json(&self) -> Result<serde_json::Value, MqttError> {
        serde_json::to_value(self).map_err(MqttError::from)
    }
}

/// Parse la liste des dispositifs depuis `zigbee2mqtt/bridge/devices`.
///
/// Retourne une liste de `friendly_name` des dispositifs connus du bridge Z2M.
pub fn parse_z2m_bridge_devices(payload: &[u8]) -> Result<Vec<String>, MqttError> {
    let arr: Vec<serde_json::Value> = serde_json::from_slice(payload)?;
    let names = arr
        .iter()
        .filter_map(|v| v.get("friendly_name"))
        .filter_map(serde_json::Value::as_str)
        .map(String::from)
        .collect();
    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;

    // TC-MQTT-07 : parse Z2M state lumiere allumee
    #[test]
    fn test_z2m_state_light_on() {
        let json = r#"{"state": "ON", "brightness": 127}"#;
        let z2m: Z2mState = serde_json::from_str(json).unwrap();
        let state = z2m.to_device_state(Uuid::new_v4());
        assert_eq!(state.on, Some(true));
        // brightness 127/254 * 100 = 50
        assert_eq!(state.brightness, Some(50));
    }

    #[test]
    fn test_z2m_state_light_off() {
        let json = r#"{"state": "OFF"}"#;
        let z2m: Z2mState = serde_json::from_str(json).unwrap();
        let state = z2m.to_device_state(Uuid::new_v4());
        assert_eq!(state.on, Some(false));
        assert!(state.brightness.is_none());
    }

    // TC-MQTT-08 : parse Z2M state capteur temperature/humidite
    #[test]
    fn test_z2m_state_sensor_temp_humidity() {
        let json = r#"{"temperature": 21.5, "humidity": 65.0}"#;
        let z2m: Z2mState = serde_json::from_str(json).unwrap();
        let state = z2m.to_device_state(Uuid::new_v4());
        assert!(state.on.is_none());
        assert!((state.temperature_current.unwrap() - 21.5).abs() < f32::EPSILON);
        assert!((state.humidity.unwrap() - 65.0).abs() < f32::EPSILON);
    }

    // TC-MQTT-09 : parse Z2M state contact (inversion semantique)
    #[test]
    fn test_z2m_state_contact_inversion() {
        // Z2M : contact=true signifie ouvert -> Alicia : contact=false
        let json = r#"{"contact": true}"#;
        let z2m: Z2mState = serde_json::from_str(json).unwrap();
        let state = z2m.to_device_state(Uuid::new_v4());
        assert_eq!(state.contact, Some(false));

        // Z2M : contact=false signifie ferme -> Alicia : contact=true
        let json2 = r#"{"contact": false}"#;
        let z2m2: Z2mState = serde_json::from_str(json2).unwrap();
        let state2 = z2m2.to_device_state(Uuid::new_v4());
        assert_eq!(state2.contact, Some(true));
    }

    // TC-MQTT-10 : build Z2M command from action "on"
    #[test]
    fn test_z2m_command_on() {
        let cmd = Z2mCommand::from_device_command("on", None).unwrap();
        assert_eq!(cmd.state, Some("ON".to_string()));
        assert!(cmd.brightness.is_none());
        let json = cmd.to_json().unwrap();
        assert_eq!(json["state"], "ON");
    }

    // TC-MQTT-11 : build Z2M command from action "set_brightness" value 80
    #[test]
    fn test_z2m_command_set_brightness() {
        let value = serde_json::json!(80);
        let cmd = Z2mCommand::from_device_command("set_brightness", Some(&value)).unwrap();
        // 80 * 254 / 100 = 203
        assert_eq!(cmd.brightness, Some(203));
        assert!(cmd.state.is_none());
    }

    // TC-MQTT-12 : build Z2M command action inconnue retourne UnsupportedAction
    #[test]
    fn test_z2m_command_unknown_action() {
        let err = Z2mCommand::from_device_command("dance", None).unwrap_err();
        assert!(matches!(err, MqttError::UnsupportedAction { .. }));
    }

    #[test]
    fn test_z2m_command_set_color() {
        let value = serde_json::json!([255, 128, 0]);
        let cmd = Z2mCommand::from_device_command("set_color", Some(&value)).unwrap();
        let color = cmd.color.as_ref().unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_z2m_command_set_temperature() {
        let value = serde_json::json!(22.5);
        let cmd = Z2mCommand::from_device_command("set_temperature", Some(&value)).unwrap();
        assert!((cmd.occupied_heating_setpoint.unwrap() - 22.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_z2m_command_open_close() {
        let open = Z2mCommand::from_device_command("open", None).unwrap();
        assert_eq!(open.position, Some(100));

        let close = Z2mCommand::from_device_command("close", None).unwrap();
        assert_eq!(close.position, Some(0));
    }

    #[test]
    fn test_z2m_command_set_position() {
        let value = serde_json::json!(50);
        let cmd = Z2mCommand::from_device_command("set_position", Some(&value)).unwrap();
        assert_eq!(cmd.position, Some(50));
    }

    // TC-MQTT-13 : parse_z2m_bridge_devices extrait les friendly_names
    #[test]
    fn test_z2m_bridge_devices_parsing() {
        let payload = r#"[
            {"ieee_address": "0x1234", "friendly_name": "lampe_salon", "type": "EndDevice"},
            {"ieee_address": "0x5678", "friendly_name": "capteur_temp", "type": "EndDevice"},
            {"ieee_address": "0x0000", "friendly_name": "Coordinator", "type": "Coordinator"}
        ]"#;
        let names = parse_z2m_bridge_devices(payload.as_bytes()).unwrap();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"lampe_salon".to_string()));
        assert!(names.contains(&"capteur_temp".to_string()));
        assert!(names.contains(&"Coordinator".to_string()));
    }

    #[test]
    fn test_z2m_bridge_devices_empty() {
        let payload = b"[]";
        let names = parse_z2m_bridge_devices(payload).unwrap();
        assert!(names.is_empty());
    }

    #[test]
    fn test_z2m_state_occupancy_to_motion() {
        let json = r#"{"occupancy": true}"#;
        let z2m: Z2mState = serde_json::from_str(json).unwrap();
        let state = z2m.to_device_state(Uuid::new_v4());
        assert_eq!(state.motion, Some(true));
    }

    #[test]
    fn test_z2m_state_power() {
        let json = r#"{"power": 42.5}"#;
        let z2m: Z2mState = serde_json::from_str(json).unwrap();
        let state = z2m.to_device_state(Uuid::new_v4());
        assert!((state.power_w.unwrap() - 42.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_z2m_state_full_brightness() {
        let json = r#"{"state": "ON", "brightness": 254}"#;
        let z2m: Z2mState = serde_json::from_str(json).unwrap();
        let state = z2m.to_device_state(Uuid::new_v4());
        assert_eq!(state.brightness, Some(100));
    }

    #[test]
    fn test_z2m_state_zero_brightness() {
        let json = r#"{"state": "ON", "brightness": 0}"#;
        let z2m: Z2mState = serde_json::from_str(json).unwrap();
        let state = z2m.to_device_state(Uuid::new_v4());
        assert_eq!(state.brightness, Some(0));
    }
}
