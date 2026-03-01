//! Types de domaine pour les dispositifs domotiques Alicia.
//!
//! Enums (type, protocole), structs (capabilities, config, device, state).

// @id: toolkit.alicia.devices.types
// @role: domain_types
// @layer: 6
// @human: Types de domaine des dispositifs domotiques : DeviceType, DeviceProtocol, Device, DeviceState.
// @do: define_device_domain_types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Type physique d'un dispositif domotique.
///
/// Un dispositif ne peut avoir qu'un seul `DeviceType`. Le type est immutable
/// apres creation du dispositif en base.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceType {
    /// Lumiere (ampoule, ruban LED, spot). Capacites : on_off, dimmer, rgb.
    Light,
    /// Volet roulant ou store. Capacites : on_off (arret), position.
    Shutter,
    /// Thermostat ou tete thermostatique. Capacites : temperature_target, mode.
    Thermostat,
    /// Prise connectee. Capacites : on_off, power_measure.
    Outlet,
    /// Capteur passif (temperature, humidite, mouvement, contact).
    Sensor,
    /// Serrure connectee. Capacites : locked.
    Lock,
}

impl DeviceType {
    /// Retourne un label lisible en francais pour l'UI.
    pub fn label_fr(self) -> &'static str {
        match self {
            Self::Light => "Lumiere",
            Self::Shutter => "Volet",
            Self::Thermostat => "Thermostat",
            Self::Outlet => "Prise",
            Self::Sensor => "Capteur",
            Self::Lock => "Serrure",
        }
    }
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label_fr())
    }
}

/// Protocole de communication avec un dispositif physique.
///
/// Determine quel adaptateur (`miyualicia-mqtt` ou `miyualicia-http`) sera utilise
/// pour envoyer les commandes et lire l'etat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceProtocol {
    /// Client MQTT (broker Mosquitto local). Champ `address` = topic de base MQTT.
    Mqtt,
    /// Requete HTTP locale (Shelly, Tapo, etc.). Champ `address` = URL de base.
    HttpLocal,
    /// Passerelle Zigbee2MQTT. Champ `address` = `friendly_name` Z2M.
    Zigbee2Mqtt,
}

impl std::fmt::Display for DeviceProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Mqtt => "MQTT",
            Self::HttpLocal => "HTTP local",
            Self::Zigbee2Mqtt => "Zigbee2MQTT",
        };
        write!(f, "{s}")
    }
}

/// Capacites fonctionnelles declarees d'un dispositif.
///
/// Champs `false` par defaut (sauf `on_off`). Chaque champ active ou non l'exposition
/// des commandes correspondantes dans l'API REST et l'UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    /// Le dispositif peut etre allume ou eteint.
    pub on_off: bool,
    /// Le dispositif supporte le reglage de niveau (0-100 %).
    pub dimmer: bool,
    /// Le dispositif supporte la couleur RGB.
    pub rgb: bool,
    /// Le dispositif supporte le reglage de position (volets, 0-100 %).
    pub position: bool,
    /// Le dispositif supporte le reglage d'une temperature cible.
    pub temperature_target: bool,
    /// Le dispositif mesure la consommation electrique en watts.
    pub power_measure: bool,
}

impl Default for DeviceCapabilities {
    fn default() -> Self {
        Self {
            on_off: true,
            dimmer: false,
            rgb: false,
            position: false,
            temperature_target: false,
            power_measure: false,
        }
    }
}

/// Configuration specifique au protocole, stockee en JSON en base.
///
/// Le champ `extra` permet d'etendre la configuration sans migration de schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    /// Authentification HTTP : "none", "basic", "bearer".
    pub auth_type: Option<String>,
    /// Token ou mot de passe — STOCKE CHIFFRE via KindMother cipher.
    /// Ce champ ne doit JAMAIS apparaitre en clair dans les logs.
    pub auth_credential: Option<String>,
    /// Qualite de service MQTT (0 ou 1). Ignore si protocole != Mqtt.
    pub mqtt_qos: Option<u8>,
    /// Parametres supplementaires libres.
    #[serde(default)]
    pub extra: serde_json::Value,
}

impl Default for DeviceConfig {
    fn default() -> Self {
        Self {
            auth_type: None,
            auth_credential: None,
            mqtt_qos: Some(1),
            extra: serde_json::Value::Null,
        }
    }
}

/// Dispositif domotique enregistre dans le registre Alicia.
///
/// # Invariants
///
/// - `id` est un UUID v4, genere a la creation, immutable.
/// - `room_id` correspond a un identifiant de piece defini dans `alicia.toml`.
/// - `address` est le topic MQTT (pour Mqtt/Zigbee2Mqtt) ou l'URL de base (pour HttpLocal).
/// - `active = false` masque le dispositif de l'UI et de l'API sans le supprimer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    /// Identifiant unique UUID v4.
    pub id: Uuid,
    /// Identifiant de la piece (reference alicia.toml).
    pub room_id: String,
    /// Type physique du dispositif.
    pub device_type: DeviceType,
    /// Nom lisible du dispositif.
    pub name: String,
    /// Protocole de communication.
    pub protocol: DeviceProtocol,
    /// Topic MQTT de base OU URL HTTP de base selon le protocole.
    pub address: String,
    /// Capacites fonctionnelles declarees.
    pub capabilities: DeviceCapabilities,
    /// Configuration specifique au protocole.
    pub config: DeviceConfig,
    /// Si false, le dispositif est masque de l'UI et de l'API.
    pub active: bool,
    /// Date de creation (UTC, ISO 8601).
    pub created_at: DateTime<Utc>,
    /// Date de derniere modification (UTC, ISO 8601).
    pub updated_at: DateTime<Utc>,
}

/// Etat courant d'un dispositif, issu de la derniere lecture protocole.
///
/// Tous les champs sont `Option` car un dispositif peut ne pas avoir encore
/// reporte son etat (etat inconnu = None sur tous les champs).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceState {
    /// UUID du dispositif associe.
    pub device_id: Uuid,
    /// true = allume, false = eteint.
    pub on: Option<bool>,
    /// 0-100 (pourcentage), None si non applicable.
    pub brightness: Option<u8>,
    /// (R, G, B) 0-255 chacun.
    pub color_rgb: Option<(u8, u8, u8)>,
    /// 0-100 % (0 = ferme, 100 = ouvert) pour volets.
    pub position: Option<u8>,
    /// Temperature mesuree en degres Celsius.
    pub temperature_current: Option<f32>,
    /// Consigne thermostat en degres Celsius.
    pub temperature_target: Option<f32>,
    /// Consommation instantanee en watts.
    pub power_w: Option<f32>,
    /// true = verrouille.
    pub locked: Option<bool>,
    /// true = mouvement detecte.
    pub motion: Option<bool>,
    /// true = contact ferme (porte/fenetre fermee).
    pub contact: Option<bool>,
    /// Humidite relative en pourcentage.
    pub humidity: Option<f32>,
    /// Date de derniere mise a jour de l'etat.
    pub updated_at: DateTime<Utc>,
}

impl DeviceState {
    /// Cree un etat vide (tous inconnus) pour un dispositif donne.
    pub fn unknown(device_id: Uuid) -> Self {
        Self {
            device_id,
            on: None,
            brightness: None,
            color_rgb: None,
            position: None,
            temperature_current: None,
            temperature_target: None,
            power_w: None,
            locked: None,
            motion: None,
            contact: None,
            humidity: None,
            updated_at: Utc::now(),
        }
    }

    /// Retourne true si au moins un champ d'etat est connu.
    pub fn is_known(&self) -> bool {
        self.on.is_some()
            || self.brightness.is_some()
            || self.temperature_current.is_some()
            || self.locked.is_some()
            || self.motion.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_type_display() {
        assert_eq!(DeviceType::Light.to_string(), "Lumiere");
        assert_eq!(DeviceType::Shutter.to_string(), "Volet");
        assert_eq!(DeviceType::Lock.to_string(), "Serrure");
    }

    #[test]
    fn test_device_protocol_display() {
        assert_eq!(DeviceProtocol::Mqtt.to_string(), "MQTT");
        assert_eq!(DeviceProtocol::HttpLocal.to_string(), "HTTP local");
        assert_eq!(DeviceProtocol::Zigbee2Mqtt.to_string(), "Zigbee2MQTT");
    }

    #[test]
    fn test_capabilities_default_has_on_off() {
        let caps = DeviceCapabilities::default();
        assert!(caps.on_off);
        assert!(!caps.dimmer);
        assert!(!caps.rgb);
    }

    #[test]
    fn test_device_config_default() {
        let config = DeviceConfig::default();
        assert!(config.auth_type.is_none());
        assert!(config.auth_credential.is_none());
        assert_eq!(config.mqtt_qos, Some(1));
    }

    #[test]
    fn test_device_state_unknown_is_not_known() {
        let state = DeviceState::unknown(Uuid::new_v4());
        assert!(!state.is_known());
    }

    #[test]
    fn test_device_state_with_on_is_known() {
        let mut state = DeviceState::unknown(Uuid::new_v4());
        state.on = Some(true);
        assert!(state.is_known());
    }

    #[test]
    fn test_device_type_serde_roundtrip() {
        let dt = DeviceType::Thermostat;
        let json = serde_json::to_string(&dt).expect("serialize");
        let back: DeviceType = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(dt, back);
    }

    #[test]
    fn test_device_protocol_serde_roundtrip() {
        let dp = DeviceProtocol::Zigbee2Mqtt;
        let json = serde_json::to_string(&dp).expect("serialize");
        let back: DeviceProtocol = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(dp, back);
    }
}
