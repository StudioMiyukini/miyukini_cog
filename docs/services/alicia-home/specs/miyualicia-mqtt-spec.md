# Spec Technique — crate `miyualicia-mqtt`

<!-- @id: spec.alicia.mqtt -->
<!-- @role: technical-specification -->
<!-- @layer: 6 -->
<!-- @human: Specification technique complete du crate client MQTT et bridge Zigbee2MQTT -->
<!-- @do: define_miyualicia_mqtt_crate_api -->

**Auteur :** Denis, Chef Dev Senior — Miyukini AI Studio
**Date :** 2026-03-01
**Version :** 1.0
**Reference :** Rapport Fondateur Alicia Home Assistante v1.0, Plan Dev General v1.0

---

## Contexte

`miyualicia-mqtt` est l'adaptateur de transport MQTT pour Alicia. Il encapsule `rumqttc`
(client MQTT async pour Tokio) et expose une interface haut niveau pour :
- Connecter Alicia a un broker Mosquitto local
- Publier des commandes vers les dispositifs
- S'abonner aux topics d'etat et recevoir les messages via un canal tokio
- Parser les messages Zigbee2MQTT vers `DeviceState`
- Assurer la reconnexion automatique transparente

## Portee / Scope

Ce crate couvre :
- La configuration du client MQTT (`MqttConfig`)
- Le client haut niveau (`MqttClient`) avec reconnexion
- L'abstraction des messages MQTT entrants/sortants
- Le module Zigbee2MQTT : parsing et construction des payloads Z2M
- Les erreurs MQTT explicites

Ce crate ne couvre pas :
- La persistance des etats (responsabilite de `miyualicia`)
- La logique de dispatch des commandes (responsabilite de `miyualicia`)
- Les dispositifs HTTP (responsabilite de `miyualicia-http`)

---

## 1. Emplacement et structure

```
crates/miyualicia-mqtt/
├── Cargo.toml
└── src/
    ├── lib.rs            # Racine, exports publics, annotations MSCM
    ├── admin_cell.rs     # Cellule Admin Miyukini
    ├── config.rs         # MqttConfig
    ├── client.rs         # MqttClient (wrappant rumqttc::AsyncClient)
    ├── message.rs        # MqttMessage, MqttIncoming
    ├── topics.rs         # Construction et parsing des topics Alicia
    ├── zigbee2mqtt.rs    # Parsing/building Z2M
    └── errors.rs         # MqttError
```

---

## 2. `Cargo.toml`

```toml
[package]
name = "miyualicia-mqtt"
version = "0.1.0"
edition = "2021"
description = "Client MQTT async et bridge Zigbee2MQTT — Alicia Home Assistante"
authors = ["Miyukini AI Studio"]

[lints.rust]
unsafe_code = "forbid"

[lints.clippy]
all = "warn"
pedantic = "warn"

[dependencies]
miyualicia-devices = { path = "../miyualicia-devices" }
rumqttc            = "0.24"
tokio              = { version = "1", features = ["full"] }
serde              = { version = "1", features = ["derive"] }
serde_json         = "1"
chrono             = { version = "0.4", features = ["serde"] }
uuid               = { version = "1", features = ["v4", "serde"] }
thiserror          = "1"
tracing            = "0.1"
```

---

## 3. `src/lib.rs`

```rust
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
```

---

## 4. `src/config.rs` — `MqttConfig`

```rust
use serde::{Deserialize, Serialize};

/// Configuration du client MQTT Alicia.
///
/// Chargee depuis la section `[mqtt]` de `alicia.toml`.
///
/// # Valeurs par defaut
///
/// Correspondent a une installation Mosquitto locale standard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttConfig {
    /// Hote du broker MQTT. Doit etre une adresse locale (LAN ou localhost).
    /// Aucune adresse cloud n'est admise (Loi d'Autonomie LOI-1).
    #[serde(default = "MqttConfig::default_host")]
    pub broker_host: String,

    /// Port du broker MQTT. Port standard : 1883.
    #[serde(default = "MqttConfig::default_port")]
    pub broker_port: u16,

    /// Identifiant client MQTT. Doit etre unique sur le broker.
    #[serde(default = "MqttConfig::default_client_id")]
    pub client_id: String,

    /// Intervalle keepalive en secondes.
    #[serde(default = "MqttConfig::default_keepalive")]
    pub keepalive_secs: u64,

    /// Delai de reconnexion automatique en secondes.
    #[serde(default = "MqttConfig::default_reconnect_delay")]
    pub reconnect_delay_secs: u64,

    /// Taille maximale du canal interne d'evenements rumqttc.
    #[serde(default = "MqttConfig::default_channel_capacity")]
    pub channel_capacity: usize,
}

impl MqttConfig {
    fn default_host() -> String { "localhost".to_string() }
    fn default_port() -> u16 { 1883 }
    fn default_client_id() -> String { "alicia-home".to_string() }
    fn default_keepalive() -> u64 { 60 }
    fn default_reconnect_delay() -> u64 { 5 }
    fn default_channel_capacity() -> usize { 256 }
}

impl Default for MqttConfig {
    fn default() -> Self {
        Self {
            broker_host:          Self::default_host(),
            broker_port:          Self::default_port(),
            client_id:            Self::default_client_id(),
            keepalive_secs:       Self::default_keepalive(),
            reconnect_delay_secs: Self::default_reconnect_delay(),
            channel_capacity:     Self::default_channel_capacity(),
        }
    }
}
```

---

## 5. `src/message.rs` — Types de messages

```rust
use serde::{Deserialize, Serialize};

/// Niveau de qualite de service MQTT.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QosLevel {
    /// QoS 0 : at most once. Pour les etats en temps reel.
    AtMostOnce  = 0,
    /// QoS 1 : at least once. Pour les commandes. Garanti delivre.
    AtLeastOnce = 1,
}

impl From<QosLevel> for rumqttc::QoS {
    fn from(q: QosLevel) -> Self {
        match q {
            QosLevel::AtMostOnce  => rumqttc::QoS::AtMostOnce,
            QosLevel::AtLeastOnce => rumqttc::QoS::AtLeastOnce,
        }
    }
}

/// Message a publier vers le broker.
#[derive(Debug, Clone)]
pub struct MqttMessage {
    pub topic:   String,
    pub payload: Vec<u8>,
    pub qos:     QosLevel,
    pub retain:  bool,
}

impl MqttMessage {
    /// Cree un message JSON a publier.
    ///
    /// # Erreurs
    ///
    /// Retourne `MqttError::SerializationError` si la valeur n'est pas serialisable.
    pub fn json(
        topic: impl Into<String>,
        value: &serde_json::Value,
        qos: QosLevel,
    ) -> Result<Self, crate::errors::MqttError> {
        let payload = serde_json::to_vec(value)?;
        Ok(Self { topic: topic.into(), payload, qos, retain: false })
    }
}

/// Message recu depuis le broker.
#[derive(Debug, Clone)]
pub struct MqttIncoming {
    pub topic:   String,
    pub payload: bytes::Bytes,
    pub qos:     QosLevel,
}

impl MqttIncoming {
    /// Parse le payload comme JSON.
    ///
    /// # Erreurs
    ///
    /// Retourne `MqttError::ParseError` si le payload n'est pas du JSON valide.
    pub fn parse_json(&self) -> Result<serde_json::Value, crate::errors::MqttError>;
}
```

---

## 6. `src/client.rs` — `MqttClient`

### 6.1 Definition

```rust
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

use crate::config::MqttConfig;
use crate::errors::MqttError;
use crate::message::{MqttIncoming, MqttMessage, QosLevel};

/// Etat de la connexion MQTT.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
}

/// Client MQTT haut niveau pour Alicia.
///
/// # Architecture interne
///
/// `MqttClient` encapsule `rumqttc::AsyncClient` et son event loop.
/// Un `tokio::spawn` tourne en arriere-plan pour traiter l'event loop rumqttc
/// et redistribuer les messages entrants via un canal `broadcast`.
///
/// # Reconnexion automatique
///
/// Quand le broker est inaccessible ou deconnecte, la boucle interne
/// attend `config.reconnect_delay_secs` puis retente la connexion.
/// Cette logique est entierement transparente pour les consommateurs.
///
/// # Thread-safety
///
/// `MqttClient` est `Clone` (Arc interne). Plusieurs composants peuvent
/// partager le meme client. Chaque clone partage la meme connexion.
#[derive(Debug, Clone)]
pub struct MqttClient {
    inner: Arc<MqttClientInner>,
}

#[derive(Debug)]
struct MqttClientInner {
    config:  MqttConfig,
    client:  Mutex<Option<rumqttc::AsyncClient>>,
    status:  Mutex<ConnectionStatus>,
    /// Canal broadcast pour les messages entrants.
    /// Chaque appelant de `subscribe_incoming()` recoit une copie.
    tx:      broadcast::Sender<MqttIncoming>,
}
```

### 6.2 API publique

```rust
impl MqttClient {
    /// Cree un nouveau client MQTT a partir de la configuration.
    /// Ne se connecte pas encore au broker.
    pub fn new(config: MqttConfig) -> Self;

    /// Demarre la connexion au broker et lance la boucle de traitement.
    ///
    /// # Semantique
    ///
    /// Cette methode est non-bloquante : elle lance un `tokio::spawn`.
    /// La connexion effective est asynchrone ; consulter `status()` pour
    /// verifier l'etat de connexion.
    ///
    /// # Reconnexion
    ///
    /// Si le broker est indisponible, la boucle interne retente periodiquement.
    /// Appeler `connect()` plusieurs fois est idempotent.
    pub async fn connect(&self) -> Result<(), MqttError>;

    /// Deconnecte proprement le client.
    pub async fn disconnect(&self) -> Result<(), MqttError>;

    /// S'abonne a un topic MQTT avec le QoS specifie.
    ///
    /// # Format topic
    ///
    /// Supports les wildcards MQTT : `+` (un niveau), `#` (multi-niveaux).
    ///
    /// # Exemple
    ///
    /// ```rust
    /// client.subscribe("zigbee2mqtt/+", QosLevel::AtMostOnce).await?;
    /// client.subscribe("alicia/home/#", QosLevel::AtMostOnce).await?;
    /// ```
    pub async fn subscribe(&self, topic: &str, qos: QosLevel) -> Result<(), MqttError>;

    /// Se desabonne d'un topic.
    pub async fn unsubscribe(&self, topic: &str) -> Result<(), MqttError>;

    /// Publie un message vers le broker.
    ///
    /// # Comportement si deconnecte
    ///
    /// Retourne `MqttError::NotConnected` si le broker est inaccessible.
    /// Le caller peut choisir de stocker le message et reessayer.
    pub async fn publish(&self, message: MqttMessage) -> Result<(), MqttError>;

    /// Publie un payload JSON vers un topic. Raccourci combine.
    ///
    /// QoS par defaut : `AtLeastOnce` (garanti pour les commandes).
    pub async fn publish_json(
        &self,
        topic: &str,
        payload: &serde_json::Value,
    ) -> Result<(), MqttError>;

    /// Retourne un `broadcast::Receiver` pour recevoir les messages entrants.
    ///
    /// Chaque appelant recoit sa propre copie des messages.
    /// Les messages sont perdus si le receiver est en retard (lagging).
    ///
    /// # Usage standard
    ///
    /// ```rust
    /// let mut rx = client.subscribe_incoming();
    /// while let Ok(msg) = rx.recv().await {
    ///     println!("topic={}, payload={:?}", msg.topic, msg.payload);
    /// }
    /// ```
    pub fn subscribe_incoming(&self) -> broadcast::Receiver<MqttIncoming>;

    /// Retourne l'etat courant de la connexion.
    pub async fn status(&self) -> ConnectionStatus;

    /// Retourne `true` si le client est actuellement connecte au broker.
    pub async fn is_connected(&self) -> bool;
}
```

---

## 7. `src/topics.rs` — Construction des topics Alicia

```rust
/// Helper pour construire et parser les topics MQTT Alicia.
///
/// # Structure des topics Alicia (custom)
///
/// Commandes : `alicia/home/{room_id}/{device_id}/command`
/// Etats     : `alicia/home/{room_id}/{device_id}/state`
///
/// # Structure des topics Zigbee2MQTT
///
/// Etats     : `zigbee2mqtt/{friendly_name}`
/// Commandes : `zigbee2mqtt/{friendly_name}/set`
/// Requetes  : `zigbee2mqtt/{friendly_name}/get`
/// Devices   : `zigbee2mqtt/bridge/devices`
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
    pub fn parse_alicia(topic: &str) -> Option<AliciaTopicParts>;
}

#[derive(Debug, Clone)]
pub struct AliciaTopicParts {
    pub room_id:   String,
    pub device_id: String,
    pub kind:      TopicKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopicKind {
    Command,
    State,
}
```

---

## 8. `src/zigbee2mqtt.rs` — Module Zigbee2MQTT

### 8.1 Parsing des etats Z2M vers `DeviceState`

```rust
use miyualicia_devices::DeviceState;
use uuid::Uuid;

/// Etat brut parse depuis un message Zigbee2MQTT.
///
/// Les champs correspondent aux proprietes standard des dispositifs Z2M.
/// Reference : https://www.zigbee2mqtt.io/guide/usage/exposes.html
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Z2mState {
    /// "ON" ou "OFF"
    pub state:         Option<String>,
    /// 0-254 (echelle Z2M, converti en 0-100 % dans `to_device_state`)
    pub brightness:    Option<u8>,
    /// Temperature mesurée en centidegres (divise par 100 = degres C)
    pub temperature:   Option<f32>,
    /// Humidite en pourcentage (0-100)
    pub humidity:      Option<f32>,
    /// `true` si mouvement detecte
    pub occupancy:     Option<bool>,
    /// `true` si contact ouvert (fenetre/porte ouverte)
    pub contact:       Option<bool>,
    /// Position du volet en % (0=ferme, 100=ouvert)
    pub position:      Option<u8>,
    /// Consommation en watts
    pub power:         Option<f32>,
    /// Temperature cible thermostat en degres C
    pub occupied_heating_setpoint: Option<f32>,
}

impl Z2mState {
    /// Convertit l'etat Z2M brut en `DeviceState` Alicia.
    ///
    /// # Conversions appliquees
    ///
    /// - `state` : "ON" → `on = Some(true)`, "OFF" → `on = Some(false)`
    /// - `brightness` : plage 0-254 Z2M → plage 0-100 Alicia (x * 100 / 254)
    /// - `occupancy` : mapped vers `DeviceState::motion`
    /// - `contact` : inversion semantique Z2M (true=ouvert) → Alicia (true=ferme)
    pub fn to_device_state(&self, device_id: Uuid) -> DeviceState;
}

/// Commande a envoyer vers un dispositif via Zigbee2MQTT.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Z2mCommand {
    /// "ON" ou "OFF"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Niveau de luminosite (0-254, echelle Z2M)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<u8>,

    /// Couleur XY pour les dispositifs RGBW (format Z2M natif)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Z2mColor>,

    /// Temperature cible thermostat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occupied_heating_setpoint: Option<f32>,

    /// Position volet (0-100 %)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u8>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Z2mColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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
    ) -> Result<Self, crate::errors::MqttError>;

    /// Serialise la commande en payload JSON publiable.
    pub fn to_json(&self) -> Result<serde_json::Value, crate::errors::MqttError>;
}

/// Parse la liste des dispositifs depuis `zigbee2mqtt/bridge/devices`.
///
/// Retourne une liste de noms de dispositifs connus du bridge Z2M.
pub fn parse_z2m_bridge_devices(payload: &[u8]) -> Result<Vec<String>, crate::errors::MqttError>;
```

---

## 9. `src/errors.rs` — `MqttError`

```rust
/// Erreurs du client MQTT Alicia.
#[derive(Debug, thiserror::Error)]
pub enum MqttError {
    /// Le client n'est pas connecte au broker.
    #[error("client MQTT non connecte : operation impossible")]
    NotConnected,

    /// Echec de connexion au broker MQTT.
    #[error("echec connexion broker MQTT {host}:{port} : {source}")]
    ConnectionFailed {
        host:   String,
        port:   u16,
        #[source]
        source: rumqttc::ClientError,
    },

    /// Erreur de publication d'un message.
    #[error("echec publication topic '{topic}' : {source}")]
    PublishFailed {
        topic:  String,
        #[source]
        source: rumqttc::ClientError,
    },

    /// Echec de souscription a un topic.
    #[error("echec souscription topic '{topic}' : {source}")]
    SubscribeFailed {
        topic:  String,
        #[source]
        source: rumqttc::ClientError,
    },

    /// Erreur de serialisation d'un payload JSON.
    #[error("erreur serialisation payload MQTT : {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Erreur de parsing d'un message entrant.
    #[error("erreur parsing message MQTT sur topic '{topic}' : {message}")]
    ParseError {
        topic:   String,
        message: String,
    },

    /// Action inconnue ou non supportee par le protocole Z2M.
    #[error("action '{action}' non supportee par Zigbee2MQTT")]
    UnsupportedAction {
        action: String,
    },

    /// Le canal broadcast est sature (lagging).
    #[error("canal messages MQTT sature : {0} messages perdus")]
    ChannelLagging(u64),
}
```

---

## 10. Tests attendus

### 10.1 `config.rs`

```rust
// TC-MQTT-01 : MqttConfig::default() produit des valeurs coherentes
#[test]
fn test_config_default_values() { ... }

// TC-MQTT-02 : deserialisation TOML partielle (champs optionnels)
#[test]
fn test_config_partial_toml_deserialization() { ... }
```

### 10.2 `topics.rs`

```rust
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

// TC-MQTT-05 : parse_alicia sur topic valide
#[test]
fn test_parse_alicia_topic_valid() { ... }

// TC-MQTT-06 : parse_alicia sur topic inconnu retourne None
#[test]
fn test_parse_alicia_topic_unknown() { ... }
```

### 10.3 `zigbee2mqtt.rs`

```rust
// TC-MQTT-07 : parse Z2M state lumiere allumee
#[test]
fn test_z2m_state_light_on() {
    let json = r#"{"state": "ON", "brightness": 127}"#;
    let z2m: Z2mState = serde_json::from_str(json).unwrap();
    let state = z2m.to_device_state(Uuid::new_v4());
    assert_eq!(state.on, Some(true));
    // brightness 127/254 * 100 ≈ 50
    assert!(state.brightness.is_some());
}

// TC-MQTT-08 : parse Z2M state capteur temperature/humidite
#[test]
fn test_z2m_state_sensor_temp_humidity() { ... }

// TC-MQTT-09 : parse Z2M state contact (inversion semantique)
#[test]
fn test_z2m_state_contact_inversion() { ... }

// TC-MQTT-10 : build Z2M command from action "on"
#[test]
fn test_z2m_command_on() { ... }

// TC-MQTT-11 : build Z2M command from action "set_brightness" value 80
#[test]
fn test_z2m_command_set_brightness() { ... }

// TC-MQTT-12 : build Z2M command action inconnue retourne UnsupportedAction
#[test]
fn test_z2m_command_unknown_action() { ... }

// TC-MQTT-13 : parse_z2m_bridge_devices extrait les friendly_names
#[test]
fn test_z2m_bridge_devices_parsing() { ... }
```

---

## 11. Annotations MSCM — recap

| Fichier           | @id                        | @layer | @role                    |
|-------------------|----------------------------|--------|--------------------------|
| `lib.rs`          | `toolkit.alicia.mqtt`      | 6      | `mqtt_protocol_adapter`  |
| `admin_cell.rs`   | `toolkit.alicia.mqtt.admin`| 6      | `governance_cell`        |
| `config.rs`       | (inline)                   | 6      | `mqtt_configuration`     |
| `client.rs`       | (inline)                   | 6      | `mqtt_async_client`      |
| `topics.rs`       | (inline)                   | 6      | `topic_builder`          |
| `zigbee2mqtt.rs`  | (inline)                   | 6      | `z2m_protocol_adapter`   |
| `errors.rs`       | (inline)                   | 6      | `error_types`            |

---

## 12. Securite et conformite

- **Loi LOI-1** : Uniquement broker local. L'URL du broker est validee a la construction :
  si elle contient une adresse publique routable (non RFC1918, non loopback), un warning
  de niveau `tracing::warn!` est emis. Aucun cloud MQTT n'est bloque techniquement,
  mais la configuration par defaut et la documentation excluent tout usage cloud.
- **Credentials MQTT** : si le broker requiert un nom d'utilisateur/mot de passe,
  ils sont stockes dans `DeviceConfig::auth_credential` chiffre via KindMother.
  Ils ne transitent jamais en clair dans les logs (`#[tracing::instrument(skip(password))]`).
- **QoS commandes** : toutes les commandes utilisent `QoS::AtLeastOnce` pour garantir
  la livraison (pas de commandes perdues silencieusement).
- **Pas de retain** sur les commandes : evite les commandes "fantomes" au redemarrage.

---

*Denis — Chef Dev Senior — Miyukini AI Studio — 2026-03-01*
