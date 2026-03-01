//! # miyualicia-mqtt
//!
//! Adaptateur de transport MQTT pour Alicia Home Assistante.
//! Encapsule rumqttc (async) et expose un client haut niveau avec reconnexion,
//! ainsi qu'un module Zigbee2MQTT pour parser/builder les payloads Z2M.
//!
//! ## Loi d'Autonomie
//!
//! Ce crate se connecte exclusivement a un broker MQTT LOCAL (Mosquitto).
//! Aucune connexion a un broker cloud n'est supportee ni prevue.
//! Si le broker est absent, le client reste en attente de reconnexion
//! sans bloquer le reste du systeme Alicia.

#![forbid(unsafe_code)]

// @id: toolkit.alicia.mqtt
// @role: mqtt_protocol_adapter
// @layer: 6
// @human: Client MQTT async pour Alicia ; reconnexion, Z2M, topics, QoS.
// @do: provide_mqtt_transport_for_alicia

pub mod admin_cell;
pub mod client;
pub mod config;
pub mod errors;
pub mod message;
pub mod topics;
pub mod zigbee2mqtt;

pub use client::MqttClient;
pub use config::MqttConfig;
pub use errors::MqttError;
pub use message::{MqttIncoming, MqttMessage, QosLevel};
pub use topics::AliciaTopic;
pub use zigbee2mqtt::{Z2mCommand, Z2mState};
