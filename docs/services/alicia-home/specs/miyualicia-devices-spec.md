# Spec Technique — crate `miyualicia-devices`

<!-- @id: spec.alicia.devices -->
<!-- @role: technical-specification -->
<!-- @layer: 6 -->
<!-- @human: Specification technique complete du crate registre de dispositifs domotiques -->
<!-- @do: define_miyualicia_devices_crate_api -->

**Auteur :** Denis, Chef Dev Senior — Miyukini AI Studio
**Date :** 2026-03-01
**Version :** 1.0
**Reference :** Rapport Fondateur Alicia Home Assistante v1.0, Plan Dev General v1.0

---

## Contexte

`miyualicia-devices` est le crate fondation de tout l'ecosysteme domotique Alicia. Il definit le
modele de donnees des dispositifs physiques (types, protocoles, capacites, etat), le registre
in-memory thread-safe, et les types d'erreur associes. Tous les autres crates Alicia dependent
de ce crate pour parler le meme langage de domaine.

## Portee / Scope

Ce crate couvre :
- La definition des types de domaine domaine (enums, structs)
- Le registre in-memory `DeviceRegistry` (lecture/ecriture concurrente)
- Les types d'erreur `DeviceError`
- Les commandes generiques sur un dispositif `DeviceCommand`
- Les tests unitaires du registre

Ce crate ne couvre pas :
- La persistance KindMother (responsabilite de `miyualicia`)
- Le transport MQTT ou HTTP (responsabilite de `miyualicia-mqtt` et `miyualicia-http`)
- La logique metier d'Alicia (responsabilite de `miyualicia`)

---

## 1. Emplacement et structure

```
crates/miyualicia-devices/
├── Cargo.toml
└── src/
    ├── lib.rs            # Racine, exports publics, annotations MSCM
    ├── admin_cell.rs     # Cellule Admin Miyukini
    ├── types.rs          # Enums et structs de domaine
    ├── registry.rs       # DeviceRegistry (Arc<RwLock<>>)
    ├── command.rs        # DeviceCommand, CommandSource
    └── errors.rs         # DeviceError (thiserror)
```

---

## 2. `Cargo.toml`

```toml
[package]
name = "miyualicia-devices"
version = "0.1.0"
edition = "2021"
description = "Registre de dispositifs domotiques — Alicia Home Assistante"
authors = ["Miyukini AI Studio"]

[lints.rust]
unsafe_code = "forbid"

[lints.clippy]
all = "warn"
pedantic = "warn"

[dependencies]
serde       = { version = "1",   features = ["derive"] }
serde_json  = "1"
uuid        = { version = "1",   features = ["v4", "serde"] }
chrono      = { version = "0.4", features = ["serde"] }
thiserror   = "1"
tracing     = "0.1"
```

---

## 3. `src/lib.rs` — Racine et annotations MSCM

```rust
//! # miyualicia-devices
//!
//! Registre de dispositifs domotiques pour Alicia Home Assistante.
//! Definit les types de domaine, le registre in-memory thread-safe,
//! les commandes generiques et les types d'erreur.
//!
//! ## Loi d'Autonomie
//!
//! Ce crate est 100 % local : aucune dependance reseau, aucun cloud.
//! Il est utilisable sans MQTT ni HTTP actifs.

#![forbid(unsafe_code)]

// @id: toolkit.alicia.devices
// @role: device_registry
// @layer: 6
// @human: Registre central des dispositifs domotiques Alicia ; types, etat, commandes.
// @do: expose_device_registry_and_types

pub mod admin_cell;
pub mod command;
pub mod errors;
pub mod registry;
pub mod types;

// Re-exports de l'API publique
pub use command::{CommandSource, DeviceCommand};
pub use errors::DeviceError;
pub use registry::DeviceRegistry;
pub use types::{
    Device, DeviceCapabilities, DeviceConfig, DeviceProtocol, DeviceState, DeviceType,
};
```

---

## 4. `src/types.rs` — Types de domaine

### 4.1 `DeviceType`

```rust
use serde::{Deserialize, Serialize};

/// Type physique d'un dispositif domotique.
///
/// # Invariant
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
            Self::Light     => "Lumière",
            Self::Shutter   => "Volet",
            Self::Thermostat => "Thermostat",
            Self::Outlet    => "Prise",
            Self::Sensor    => "Capteur",
            Self::Lock      => "Serrure",
        }
    }
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label_fr())
    }
}
```

### 4.2 `DeviceProtocol`

```rust
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
    /// Passerelle Zigbee2MQTT. Champ `address` = friendly_name Z2M.
    Zigbee2Mqtt,
}

impl std::fmt::Display for DeviceProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Mqtt       => "MQTT",
            Self::HttpLocal  => "HTTP local",
            Self::Zigbee2Mqtt => "Zigbee2MQTT",
        };
        write!(f, "{s}")
    }
}
```

### 4.3 `DeviceCapabilities`

```rust
/// Capacites fonctionnelles declarees d'un dispositif.
///
/// Champs `false` par defaut. Chaque champ active ou non l'exposition
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
            on_off:             true,
            dimmer:             false,
            rgb:                false,
            position:           false,
            temperature_target: false,
            power_measure:      false,
        }
    }
}
```

### 4.4 `DeviceConfig`

```rust
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
            auth_type:       None,
            auth_credential: None,
            mqtt_qos:        Some(1),
            extra:           serde_json::Value::Null,
        }
    }
}
```

### 4.5 `Device`

```rust
use chrono::{DateTime, Utc};
use uuid::Uuid;

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
    pub id:           Uuid,
    pub room_id:      String,
    pub device_type:  DeviceType,
    pub name:         String,
    pub protocol:     DeviceProtocol,
    /// Topic MQTT de base OU URL HTTP de base selon le protocole.
    pub address:      String,
    pub capabilities: DeviceCapabilities,
    pub config:       DeviceConfig,
    pub active:       bool,
    pub created_at:   DateTime<Utc>,
    pub updated_at:   DateTime<Utc>,
}
```

### 4.6 `DeviceState`

```rust
/// Etat courant d'un dispositif, issu de la derniere lecture protocole.
///
/// Tous les champs sont `Option` car un dispositif peut ne pas avoir encore
/// reporte son etat (etat inconnu = None sur tous les champs).
///
/// # Semantique des champs
///
/// - `on`                  : true = allume, false = eteint
/// - `brightness`          : 0-100 (pourcentage), None si non applicable
/// - `color_rgb`           : (R, G, B) 0-255 chacun
/// - `position`            : 0-100 % (0 = ferme, 100 = ouvert) pour volets
/// - `temperature_current` : temperature mesuree en degres Celsius
/// - `temperature_target`  : consigne thermostat en degres Celsius
/// - `power_w`             : consommation instantanee en watts
/// - `locked`              : true = verrouille
/// - `motion`              : true = mouvement detecte
/// - `contact`             : true = contact ferme (porte/fenetre fermee)
/// - `humidity`            : humidite relative en pourcentage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceState {
    pub device_id:           Uuid,
    pub on:                  Option<bool>,
    pub brightness:          Option<u8>,
    pub color_rgb:           Option<(u8, u8, u8)>,
    pub position:            Option<u8>,
    pub temperature_current: Option<f32>,
    pub temperature_target:  Option<f32>,
    pub power_w:             Option<f32>,
    pub locked:              Option<bool>,
    pub motion:              Option<bool>,
    pub contact:             Option<bool>,
    pub humidity:            Option<f32>,
    pub updated_at:          DateTime<Utc>,
}

impl DeviceState {
    /// Cree un etat vide (tous inconnus) pour un dispositif donne.
    pub fn unknown(device_id: Uuid) -> Self {
        Self {
            device_id,
            on:                  None,
            brightness:          None,
            color_rgb:           None,
            position:            None,
            temperature_current: None,
            temperature_target:  None,
            power_w:             None,
            locked:              None,
            motion:              None,
            contact:             None,
            humidity:            None,
            updated_at:          Utc::now(),
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
```

---

## 5. `src/registry.rs` — `DeviceRegistry`

### 5.1 Definition et thread-safety

```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use crate::errors::DeviceError;
use crate::types::{Device, DeviceState};

/// Registre in-memory de tous les dispositifs domotiques.
///
/// # Thread-safety
///
/// `DeviceRegistry` est concu pour un acces concurrent via `Arc<RwLock<DeviceRegistry>>`.
/// Les lectures sont sans contention entre elles. Les ecritures bloquent les lectures
/// le temps de la mutation. Les ecritures sont courtes (HashMap insert/remove).
///
/// # Usage standard
///
/// ```rust
/// use std::sync::{Arc, RwLock};
/// use miyualicia_devices::DeviceRegistry;
///
/// let registry = Arc::new(RwLock::new(DeviceRegistry::new()));
/// // Lecture :
/// let guard = registry.read().expect("registry read lock poisoned");
/// let devices = guard.list_all();
/// // Ecriture :
/// drop(guard);
/// let mut guard = registry.write().expect("registry write lock poisoned");
/// guard.add_device(device, state);
/// ```
///
/// # Invariants
///
/// - Un seul dispositif par UUID (unicite garantie par `add_device`).
/// - L'etat d'un dispositif absent du registre ne peut pas etre mis a jour.
/// - `list_by_room` retourne uniquement les dispositifs `active = true`.
#[derive(Debug, Default)]
pub struct DeviceRegistry {
    devices: HashMap<Uuid, Device>,
    states:  HashMap<Uuid, DeviceState>,
}
```

### 5.2 API publique

```rust
impl DeviceRegistry {
    /// Cree un registre vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Ajoute un dispositif et son etat initial.
    ///
    /// # Erreurs
    ///
    /// Retourne `DeviceError::AlreadyExists` si un dispositif avec le meme UUID existe.
    pub fn add_device(&mut self, device: Device, state: DeviceState) -> Result<(), DeviceError>;

    /// Supprime un dispositif et son etat associe.
    ///
    /// # Erreurs
    ///
    /// Retourne `DeviceError::NotFound` si l'UUID est absent.
    pub fn remove_device(&mut self, id: Uuid) -> Result<(), DeviceError>;

    /// Retourne une reference au dispositif, ou `DeviceError::NotFound`.
    pub fn get_device(&self, id: Uuid) -> Result<&Device, DeviceError>;

    /// Retourne une reference mutable au dispositif, ou `DeviceError::NotFound`.
    pub fn get_device_mut(&mut self, id: Uuid) -> Result<&mut Device, DeviceError>;

    /// Retourne l'etat courant d'un dispositif, ou `DeviceError::NotFound`.
    pub fn get_state(&self, id: Uuid) -> Result<&DeviceState, DeviceError>;

    /// Met a jour l'etat d'un dispositif existant.
    ///
    /// # Erreurs
    ///
    /// Retourne `DeviceError::NotFound` si le dispositif est absent du registre.
    pub fn update_state(&mut self, state: DeviceState) -> Result<(), DeviceError>;

    /// Retourne tous les dispositifs actifs d'une piece, avec leur etat.
    pub fn list_by_room(&self, room_id: &str) -> Vec<(&Device, &DeviceState)>;

    /// Retourne tous les dispositifs (actifs et inactifs), avec leur etat.
    pub fn list_all(&self) -> Vec<(&Device, &DeviceState)>;

    /// Retourne le nombre de dispositifs dans le registre.
    pub fn len(&self) -> usize;

    /// Retourne `true` si le registre est vide.
    pub fn is_empty(&self) -> bool;
}
```

### 5.3 Type alias recommande

```rust
/// Type alias pour le registre partage entre threads.
/// A utiliser dans `AliciaService` et les handlers API.
pub type SharedDeviceRegistry = Arc<RwLock<DeviceRegistry>>;
```

---

## 6. `src/command.rs` — `DeviceCommand` et `CommandSource`

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Source d'une commande domotique.
///
/// Trace dans `alicia_commands_log` pour l'audit trail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandSource {
    /// Commande issue de la reconnaissance vocale.
    Voice,
    /// Commande issue de l'API REST externe (serveur MWS ou client tiers).
    Api,
    /// Commande issue du moteur d'automatisations.
    Automation,
    /// Commande issue directement de l'UI Dioxus (action manuelle utilisateur).
    Manual,
}

/// Commande generique a envoyer a un dispositif physique.
///
/// # Semantique des actions standard
///
/// | action            | value                         | dispositifs concernes   |
/// |-------------------|-------------------------------|-------------------------|
/// | `"on"`            | None                          | Light, Outlet, Shutter  |
/// | `"off"`           | None                          | Light, Outlet, Shutter  |
/// | `"set_brightness"`| `Number` 0-100                | Light (dimmer=true)     |
/// | `"set_color"`     | `[R, G, B]` (0-255 chacun)    | Light (rgb=true)        |
/// | `"open"`          | None                          | Shutter                 |
/// | `"close"`         | None                          | Shutter                 |
/// | `"set_position"`  | `Number` 0-100                | Shutter                 |
/// | `"set_temperature"`| `Number` (degres Celsius)     | Thermostat              |
/// | `"lock"`          | None                          | Lock                    |
/// | `"unlock"`        | None                          | Lock                    |
///
/// La traduction vers le payload protocole (MQTT JSON ou HTTP body) est
/// la responsabilite des adaptateurs dans `miyualicia-mqtt` et `miyualicia-http`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCommand {
    /// UUID du dispositif cible. Doit exister dans le `DeviceRegistry`.
    pub device_id: Uuid,
    /// Action a realiser. Voir tableau semantique ci-dessus.
    pub action:    String,
    /// Valeur optionnelle associee a l'action.
    pub value:     Option<serde_json::Value>,
    /// Origine de la commande pour l'audit trail.
    pub source:    CommandSource,
}

impl DeviceCommand {
    /// Construit une commande simple sans valeur.
    pub fn simple(device_id: Uuid, action: impl Into<String>, source: CommandSource) -> Self {
        Self { device_id, action: action.into(), value: None, source }
    }

    /// Construit une commande avec valeur.
    pub fn with_value(
        device_id: Uuid,
        action: impl Into<String>,
        value: serde_json::Value,
        source: CommandSource,
    ) -> Self {
        Self { device_id, action: action.into(), value: Some(value), source }
    }
}
```

---

## 7. `src/errors.rs` — `DeviceError`

```rust
use uuid::Uuid;

/// Erreurs du registre de dispositifs.
#[derive(Debug, thiserror::Error)]
pub enum DeviceError {
    /// Le dispositif demande n'existe pas dans le registre.
    #[error("dispositif {0} introuvable dans le registre")]
    NotFound(Uuid),

    /// Un dispositif avec le meme UUID existe deja.
    #[error("dispositif {0} deja present dans le registre")]
    AlreadyExists(Uuid),

    /// L'action demandee n'est pas supportee par les capacites du dispositif.
    #[error("action '{action}' non supportee par le dispositif {device_id} (type: {device_type})")]
    UnsupportedAction {
        device_id:   Uuid,
        device_type: String,
        action:      String,
    },

    /// Valeur fournie hors de la plage valide.
    #[error("valeur hors plage pour '{field}' : {message}")]
    InvalidValue {
        field:   String,
        message: String,
    },

    /// Erreur de serialisation/deserialisation de l'etat JSON.
    #[error("erreur de serialisation etat : {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Le verrou RwLock a ete empoisonne (panique dans un thread concurrent).
    #[error("le registre est dans un etat corrompu (lock poison) : {0}")]
    LockPoisoned(String),
}
```

---

## 8. `src/admin_cell.rs`

```rust
// @id: toolkit.alicia.devices.admin
// @role: governance_cell
// @layer: 6
// @human: Cellule d'administration du crate miyualicia-devices
// @do: identify_and_self_describe_miyualicia_devices

pub const TOOLKIT_ID: &str = "toolkit.alicia.devices";
pub const TOOLKIT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const TOOLKIT_LAYER: u8 = 6;

#[derive(Debug, Clone)]
pub struct AliciaDevicesAdminCell {
    pub id:      &'static str,
    pub version: &'static str,
    pub layer:   u8,
}

pub fn alicia_devices_admin_cell() -> AliciaDevicesAdminCell {
    AliciaDevicesAdminCell {
        id:      TOOLKIT_ID,
        version: TOOLKIT_VERSION,
        layer:   TOOLKIT_LAYER,
    }
}
```

---

## 9. Tests attendus

### 9.1 Couverture minimale (>= 70 %)

Fichier : `crates/miyualicia-devices/src/registry.rs` — section `#[cfg(test)]`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Device, DeviceCapabilities, DeviceConfig, DeviceProtocol, DeviceState, DeviceType};
    use uuid::Uuid;
    use chrono::Utc;

    fn make_device(room_id: &str) -> (Device, DeviceState) {
        let id = Uuid::new_v4();
        let device = Device {
            id,
            room_id: room_id.to_string(),
            device_type: DeviceType::Light,
            name: "Lampe test".to_string(),
            protocol: DeviceProtocol::Mqtt,
            address: "alicia/salon/lampe".to_string(),
            capabilities: DeviceCapabilities::default(),
            config: DeviceConfig::default(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let state = DeviceState::unknown(id);
        (device, state)
    }

    // TC-DEV-01 : ajouter un dispositif au registre
    #[test]
    fn test_add_device_success() { ... }

    // TC-DEV-02 : double ajout retourne AlreadyExists
    #[test]
    fn test_add_device_duplicate_fails() { ... }

    // TC-DEV-03 : get_device retourne le bon dispositif
    #[test]
    fn test_get_device_found() { ... }

    // TC-DEV-04 : get_device sur UUID absent retourne NotFound
    #[test]
    fn test_get_device_not_found() { ... }

    // TC-DEV-05 : update_state met a jour l'etat
    #[test]
    fn test_update_state_success() { ... }

    // TC-DEV-06 : update_state sur dispositif absent retourne NotFound
    #[test]
    fn test_update_state_not_found() { ... }

    // TC-DEV-07 : list_by_room filtre correctement
    #[test]
    fn test_list_by_room_filters_correctly() { ... }

    // TC-DEV-08 : list_by_room exclut les dispositifs inactifs
    #[test]
    fn test_list_by_room_excludes_inactive() { ... }

    // TC-DEV-09 : remove_device puis get_device retourne NotFound
    #[test]
    fn test_remove_device() { ... }

    // TC-DEV-10 : DeviceState::unknown a is_known() == false
    #[test]
    fn test_device_state_unknown_is_not_known() { ... }

    // TC-DEV-11 : DeviceCommand::simple construit correctement
    #[test]
    fn test_device_command_simple() { ... }
}
```

---

## 10. Annotations MSCM — recap

| Fichier         | @id                           | @layer | @role               |
|-----------------|-------------------------------|--------|---------------------|
| `lib.rs`        | `toolkit.alicia.devices`      | 6      | `device_registry`   |
| `admin_cell.rs` | `toolkit.alicia.devices.admin`| 6      | `governance_cell`   |
| `types.rs`      | (inline comments par struct)  | 6      | `domain_types`      |
| `registry.rs`   | (inline comments)             | 6      | `in_memory_registry`|
| `command.rs`    | (inline comments)             | 6      | `command_model`     |
| `errors.rs`     | (inline comments)             | 6      | `error_types`       |

---

## 11. Dependances inverses (qui depend de ce crate)

```
miyualicia-devices
    ├── miyualicia-mqtt          (pour parser les etats Z2M vers DeviceState)
    ├── miyualicia-http          (pour traduire les reponses HTTP en DeviceState)
    ├── miyualicia               (orchestrateur, source de verite)
    ├── miyualicia-api           (DTOs derives de Device et DeviceState)
    └── miyualicia-automations   (conditions evaluees sur DeviceState)
```

---

## 12. Securite et conformite

- `auth_credential` dans `DeviceConfig` : ne jamais logger, ne jamais serialiser vers l'API REST.
  Le champ est marque `#[serde(skip_serializing)]` dans les DTOs d'exposition.
- Pas de `unwrap()` en dehors des tests.
- Les `RwLock` poissones sont captures en `DeviceError::LockPoisoned` et reportes vers le caller.
- Pas de donnees personnelles dans ce crate (les dispositifs sont des objets physiques anonymes).

---

*Denis — Chef Dev Senior — Miyukini AI Studio — 2026-03-01*
