//! Construction et parsing des topics MQTT Alicia.
//!
//! Structure des topics Alicia custom :
//! - Commandes : `alicia/home/{room_id}/{device_id}/command`
//! - Etats     : `alicia/home/{room_id}/{device_id}/state`
//!
//! Structure des topics Zigbee2MQTT :
//! - Etats     : `zigbee2mqtt/{friendly_name}`
//! - Commandes : `zigbee2mqtt/{friendly_name}/set`
//! - Requetes  : `zigbee2mqtt/{friendly_name}/get`
//! - Devices   : `zigbee2mqtt/bridge/devices`

// @id: toolkit.alicia.mqtt.topics
// @role: topic_builder
// @layer: 6
// @human: Construction et parsing des topics MQTT Alicia et Zigbee2MQTT.
// @do: build_and_parse_mqtt_topics

/// Helper pour construire et parser les topics MQTT Alicia.
pub struct AliciaTopic;

impl AliciaTopic {
    /// Topic de commande pour un dispositif Alicia custom.
    pub fn command(room_id: &str, device_id: &str) -> String {
        format!("alicia/home/{room_id}/{device_id}/command")
    }

    /// Topic d'etat pour un dispositif Alicia custom.
    pub fn state(room_id: &str, device_id: &str) -> String {
        format!("alicia/home/{room_id}/{device_id}/state")
    }

    /// Topic de commande Zigbee2MQTT.
    pub fn z2m_set(friendly_name: &str) -> String {
        format!("zigbee2mqtt/{friendly_name}/set")
    }

    /// Topic de requete Zigbee2MQTT (GET).
    pub fn z2m_get(friendly_name: &str) -> String {
        format!("zigbee2mqtt/{friendly_name}/get")
    }

    /// Topic d'etat Zigbee2MQTT.
    pub fn z2m_state(friendly_name: &str) -> String {
        format!("zigbee2mqtt/{friendly_name}")
    }

    /// Topic de decouverte des dispositifs Zigbee2MQTT.
    pub fn z2m_bridge_devices() -> &'static str {
        "zigbee2mqtt/bridge/devices"
    }

    /// Pattern de souscription pour tous les etats Alicia.
    pub fn state_wildcard() -> &'static str {
        "alicia/home/+/+/state"
    }

    /// Pattern de souscription pour tous les etats Zigbee2MQTT.
    pub fn z2m_wildcard() -> &'static str {
        "zigbee2mqtt/#"
    }

    /// Parse un topic Alicia custom et extrait (room_id, device_id, kind).
    ///
    /// Retourne `None` si le topic ne correspond pas au pattern Alicia.
    pub fn parse_alicia(topic: &str) -> Option<AliciaTopicParts> {
        let parts: Vec<&str> = topic.split('/').collect();
        // Format attendu : alicia/home/{room_id}/{device_id}/{kind}
        if parts.len() != 5 || parts[0] != "alicia" || parts[1] != "home" {
            return None;
        }
        let kind = match parts[4] {
            "command" => TopicKind::Command,
            "state" => TopicKind::State,
            _ => return None,
        };
        Some(AliciaTopicParts {
            room_id: parts[2].to_string(),
            device_id: parts[3].to_string(),
            kind,
        })
    }

    /// Parse un topic Zigbee2MQTT et extrait le friendly_name.
    ///
    /// Retourne `None` si le topic ne commence pas par `zigbee2mqtt/`
    /// ou s'il s'agit d'un topic bridge (bridge/...).
    pub fn parse_z2m_device(topic: &str) -> Option<String> {
        let stripped = topic.strip_prefix("zigbee2mqtt/")?;
        // Exclure les topics bridge
        if stripped.starts_with("bridge/") {
            return None;
        }
        // Prendre uniquement le premier segment (friendly_name)
        // "zigbee2mqtt/lampe_salon" -> "lampe_salon"
        // "zigbee2mqtt/lampe_salon/set" -> "lampe_salon"
        let friendly_name = stripped.split('/').next()?;
        if friendly_name.is_empty() {
            return None;
        }
        Some(friendly_name.to_string())
    }
}

/// Parties parsees d'un topic Alicia custom.
#[derive(Debug, Clone)]
pub struct AliciaTopicParts {
    /// Identifiant de la piece.
    pub room_id: String,
    /// Identifiant du dispositif.
    pub device_id: String,
    /// Type de topic (commande ou etat).
    pub kind: TopicKind,
}

/// Type de topic Alicia.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopicKind {
    /// Topic de commande.
    Command,
    /// Topic d'etat.
    State,
}

#[cfg(test)]
mod tests {
    use super::*;

    // TC-MQTT-03 : construction topic commande
    #[test]
    fn test_topic_command_format() {
        assert_eq!(
            AliciaTopic::command("salon", "uuid-123"),
            "alicia/home/salon/uuid-123/command"
        );
    }

    // TC-MQTT-04 : construction topic etat Z2M
    #[test]
    fn test_topic_z2m_set_format() {
        assert_eq!(
            AliciaTopic::z2m_set("lampe_salon"),
            "zigbee2mqtt/lampe_salon/set"
        );
    }

    #[test]
    fn test_topic_state_format() {
        assert_eq!(
            AliciaTopic::state("cuisine", "dev-456"),
            "alicia/home/cuisine/dev-456/state"
        );
    }

    #[test]
    fn test_topic_z2m_state_format() {
        assert_eq!(
            AliciaTopic::z2m_state("capteur_temp"),
            "zigbee2mqtt/capteur_temp"
        );
    }

    // TC-MQTT-05 : parse_alicia sur topic valide
    #[test]
    fn test_parse_alicia_topic_valid_command() {
        let parts =
            AliciaTopic::parse_alicia("alicia/home/salon/lamp-001/command").expect("should parse");
        assert_eq!(parts.room_id, "salon");
        assert_eq!(parts.device_id, "lamp-001");
        assert_eq!(parts.kind, TopicKind::Command);
    }

    #[test]
    fn test_parse_alicia_topic_valid_state() {
        let parts =
            AliciaTopic::parse_alicia("alicia/home/chambre/thermo-1/state").expect("should parse");
        assert_eq!(parts.room_id, "chambre");
        assert_eq!(parts.device_id, "thermo-1");
        assert_eq!(parts.kind, TopicKind::State);
    }

    // TC-MQTT-06 : parse_alicia sur topic inconnu retourne None
    #[test]
    fn test_parse_alicia_topic_unknown() {
        assert!(AliciaTopic::parse_alicia("zigbee2mqtt/lamp").is_none());
        assert!(AliciaTopic::parse_alicia("alicia/home/salon").is_none());
        assert!(AliciaTopic::parse_alicia("alicia/home/salon/lamp/unknown").is_none());
        assert!(AliciaTopic::parse_alicia("").is_none());
    }

    #[test]
    fn test_parse_z2m_device_valid() {
        assert_eq!(
            AliciaTopic::parse_z2m_device("zigbee2mqtt/lampe_salon"),
            Some("lampe_salon".to_string())
        );
        assert_eq!(
            AliciaTopic::parse_z2m_device("zigbee2mqtt/lampe_salon/set"),
            Some("lampe_salon".to_string())
        );
    }

    #[test]
    fn test_parse_z2m_device_bridge_excluded() {
        assert!(AliciaTopic::parse_z2m_device("zigbee2mqtt/bridge/devices").is_none());
        assert!(AliciaTopic::parse_z2m_device("zigbee2mqtt/bridge/state").is_none());
    }

    #[test]
    fn test_parse_z2m_device_invalid() {
        assert!(AliciaTopic::parse_z2m_device("mqtt/other").is_none());
        assert!(AliciaTopic::parse_z2m_device("zigbee2mqtt/").is_none());
    }

    #[test]
    fn test_wildcards() {
        assert_eq!(AliciaTopic::state_wildcard(), "alicia/home/+/+/state");
        assert_eq!(AliciaTopic::z2m_wildcard(), "zigbee2mqtt/#");
        assert_eq!(
            AliciaTopic::z2m_bridge_devices(),
            "zigbee2mqtt/bridge/devices"
        );
    }
}
