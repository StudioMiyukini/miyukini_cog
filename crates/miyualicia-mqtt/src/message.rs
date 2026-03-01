//! Types de messages MQTT entrants et sortants.

// @id: toolkit.alicia.mqtt.message
// @role: message_types
// @layer: 6
// @human: Types de messages MQTT : MqttMessage (sortant), MqttIncoming (entrant), QosLevel.
// @do: define_mqtt_message_types

use serde::{Deserialize, Serialize};

/// Niveau de qualite de service MQTT.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QosLevel {
    /// QoS 0 : at most once. Pour les etats en temps reel.
    AtMostOnce = 0,
    /// QoS 1 : at least once. Pour les commandes. Garanti delivre.
    AtLeastOnce = 1,
}

impl From<QosLevel> for rumqttc::QoS {
    fn from(q: QosLevel) -> Self {
        match q {
            QosLevel::AtMostOnce => rumqttc::QoS::AtMostOnce,
            QosLevel::AtLeastOnce => rumqttc::QoS::AtLeastOnce,
        }
    }
}

/// Message a publier vers le broker.
#[derive(Debug, Clone)]
pub struct MqttMessage {
    /// Topic MQTT cible.
    pub topic: String,
    /// Payload binaire a publier.
    pub payload: Vec<u8>,
    /// Niveau de QoS.
    pub qos: QosLevel,
    /// Si true, le broker conserve le dernier message sur le topic.
    pub retain: bool,
}

impl MqttMessage {
    /// Cree un message JSON a publier.
    ///
    /// Retourne `MqttError::SerializationError` si la valeur n'est pas serialisable.
    pub fn json(
        topic: impl Into<String>,
        value: &serde_json::Value,
        qos: QosLevel,
    ) -> Result<Self, crate::errors::MqttError> {
        let payload = serde_json::to_vec(value)?;
        Ok(Self {
            topic: topic.into(),
            payload,
            qos,
            retain: false,
        })
    }
}

/// Message recu depuis le broker.
#[derive(Debug, Clone)]
pub struct MqttIncoming {
    /// Topic MQTT source.
    pub topic: String,
    /// Payload binaire recu.
    pub payload: bytes::Bytes,
    /// Niveau de QoS du message.
    pub qos: QosLevel,
}

impl MqttIncoming {
    /// Parse le payload comme JSON.
    ///
    /// Retourne `MqttError::ParseError` si le payload n'est pas du JSON valide.
    pub fn parse_json(&self) -> Result<serde_json::Value, crate::errors::MqttError> {
        serde_json::from_slice(&self.payload).map_err(|e| crate::errors::MqttError::ParseError {
            topic: self.topic.clone(),
            message: e.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mqtt_message_json() {
        let value = serde_json::json!({"state": "ON"});
        let msg = MqttMessage::json("test/topic", &value, QosLevel::AtLeastOnce)
            .expect("create json message");
        assert_eq!(msg.topic, "test/topic");
        assert!(!msg.retain);
        assert_eq!(msg.qos, QosLevel::AtLeastOnce);
        let parsed: serde_json::Value =
            serde_json::from_slice(&msg.payload).expect("parse payload");
        assert_eq!(parsed["state"], "ON");
    }

    #[test]
    fn test_mqtt_incoming_parse_json_valid() {
        let incoming = MqttIncoming {
            topic: "zigbee2mqtt/lamp".to_string(),
            payload: bytes::Bytes::from(r#"{"state":"ON","brightness":127}"#),
            qos: QosLevel::AtMostOnce,
        };
        let value = incoming.parse_json().expect("parse json");
        assert_eq!(value["state"], "ON");
        assert_eq!(value["brightness"], 127);
    }

    #[test]
    fn test_mqtt_incoming_parse_json_invalid() {
        let incoming = MqttIncoming {
            topic: "test/bad".to_string(),
            payload: bytes::Bytes::from("not json"),
            qos: QosLevel::AtMostOnce,
        };
        let err = incoming.parse_json().unwrap_err();
        assert!(matches!(err, crate::errors::MqttError::ParseError { .. }));
    }

    #[test]
    fn test_qos_level_to_rumqttc() {
        let q0: rumqttc::QoS = QosLevel::AtMostOnce.into();
        assert_eq!(q0, rumqttc::QoS::AtMostOnce);
        let q1: rumqttc::QoS = QosLevel::AtLeastOnce.into();
        assert_eq!(q1, rumqttc::QoS::AtLeastOnce);
    }
}
