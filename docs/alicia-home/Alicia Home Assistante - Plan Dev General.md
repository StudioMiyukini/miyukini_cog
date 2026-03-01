# Alicia Home Assistante — Plan de Développement Général

<!-- @id: doc.alicia-home.plan-dev-general -->
<!-- @role: implementation-plan -->
<!-- @layer: governance -->
<!-- @human: Plan de développement exhaustif, phase par phase, fichier par fichier, pour François, Lise et George -->
<!-- @do: guide_alicia_home_full_implementation -->

**Auteur :** Maria, Chef de Projet — transmis à Denis pour coordination
**Date :** 2026-03-01
**Version :** 1.0
**Référence :** Rapport Fondateur Alicia Home Assistante v1.0

---

## Conventions de lecture

- **[F]** = tâche Francois (backend Rust)
- **[L]** = tâche Lise (frontend Dioxus)
- **[D]** = tâche Denis (documentation / coordination)
- **[G]** = tâche George (audit)
- **[A]** = tâche Arianne (archivage)
- **BLOQUE PAR** = dépendance stricte : ne pas commencer avant la tâche listée
- Ordre d'exécution : les tâches d'une même phase peuvent être parallélisées sauf mention "BLOQUE PAR"

---

## PHASE 0 — Préparation et analyse (1 semaine)

### Objectif

Valider le rapport fondateur, analyser la concurrence, préparer l'environnement de développement.

### Tâches Denis [D]

**D-0-1 : Lire et valider le rapport fondateur**
- Fichier : `docs/alicia-home/Alicia Home Assistante - Rapport Fondateur.md`
- Action : annoter les points à préciser, valider les choix techniques
- Livrable : commentaires sur le rapport, éventuelles corrections

**D-0-2 : Inventaire complet du code existant**
- Lire : `crates/miyuvoicecapture/src/*.rs` (tous)
- Lire : `crates/miyuwakeword/src/*.rs` (tous)
- Lire : `apps/central/src/services/miyuvoice/*.rs` (tous)
- Lire : `Cargo.toml` workspace
- Livrable : liste exhaustive des symboles à renommer et des fichiers à modifier

**D-0-3 : Spécification des API publiques des nouveaux crates**
- Fichier à créer : `docs/alicia-home/specs/miyualicia-api-spec.md`
- Fichier à créer : `docs/alicia-home/specs/miyualicia-devices-spec.md`
- Fichier à créer : `docs/alicia-home/specs/miyualicia-mqtt-spec.md`
- Fichier à créer : `docs/alicia-home/specs/miyualicia-automations-spec.md`
- Contenu : signatures Rust publiques attendues, types, erreurs, exemples d'usage

**D-0-4 : Registre des ports COG**
- Vérifier que le port 7890 est disponible dans le registre des ports COG
- Fichier : `docs/reference/` (chercher fichier registre ports)
- Action : enregistrer `alicia-api: 7890`

### Tâches Fabrice [Fabrice — hors scope Claude Code, analyse concurrence]

**F-0-1 : Analyse comparative**
- Home Assistant : architecture, protocoles supportés, points forts/faibles
- Gladys Assistant : stack Node.js, API, protocoles
- Jeedom : points de différenciation avec Alicia
- Livrable : `docs/alicia-home/Alicia - Analyse Concurrence.md`

### Jalon Phase 0

- Rapport fondateur validé par Denis
- Spécifications des crates rédigées
- Analyse concurrence reçue de Fabrice

---

## PHASE 1 — Renommage et refactoring (1 semaine)

### Objectif

Renommer MiyukiniVoice en Alicia Home Assistante dans tout le code. Aucune nouvelle fonctionnalité. Les tests doivent rester verts.

### Règle stricte

Faire le renommage dans une branche git dédiée `feat/alicia-rename`. Un commit par fichier ou groupe logique de fichiers.

### Tâches Francois [F]

**F-1-1 : Renommer le crate `miyuvoicecapture`**
- BLOQUE PAR : D-0-2
- Fichier à modifier : `Cargo.toml` workspace (section members)
- Fichier à modifier : `crates/miyuvoicecapture/Cargo.toml` (name = "miyualicia-capture")
- Fichier à modifier : `crates/miyuvoicecapture/src/lib.rs` (commentaires, @id MSCM)
- Fichier à modifier : `crates/miyuvoicecapture/src/admin_cell.rs` (TOOLKIT_ID, noms)
- Action : renommer le répertoire `crates/miyuvoicecapture/` → `crates/miyualicia-capture/`
- Mettre à jour tous les `use miyuvoicecapture::` dans le workspace
- Note : conserver la fonctionnalité à l'identique, seuls les noms changent

**F-1-2 : Renommer le crate `miyuwakeword`**
- BLOQUE PAR : D-0-2
- Fichier à modifier : `Cargo.toml` workspace
- Fichier à modifier : `crates/miyuwakeword/Cargo.toml` (name = "miyualicia-wakeword")
- Fichier à modifier : tous les `src/*.rs` du crate
- Action : renommer le répertoire `crates/miyuwakeword/` → `crates/miyualicia-wakeword/`
- Mettre à jour tous les `use miyuwakeword::` dans le workspace

**F-1-3 : Mettre à jour les dépendances du workspace**
- Fichier à modifier : `Cargo.toml` workspace principal
- Fichier à modifier : `apps/central/Cargo.toml`
- Action : remplacer `miyuvoicecapture` → `miyualicia-capture`, `miyuwakeword` → `miyualicia-wakeword`
- Vérifier : `cargo build --workspace` doit compiler sans erreur

**F-1-4 : Mettre à jour le module UI existant**
- Fichier à modifier : `apps/central/src/services/miyuvoice/mod.rs`
  - Renommer `MiyuVoiceTab` → `AliciaTab`
  - Renommer `MiyuVoiceView` → `AliciaView`
  - `use miyuvoicecapture::` → `use miyualicia_capture::`
  - `use miyuwakeword::` → `use miyualicia_wakeword::`
  - String "MiyukiniVoice" → "Alicia Home" dans les textes UI
- Fichier à modifier : `apps/central/src/services/miyuvoice/state.rs`
  - Renommer `VoiceService` → `AliciaService`
  - Renommer `VoiceServiceSnapshot` → `AliciaSnapshot`
  - Renommer `SharedVoiceService` → `SharedAliciaService`
  - Strings "MiyukiniVoice" → "Alicia"
- Fichier à modifier : `apps/central/src/services/miyuvoice/dashboard.rs`
  - Strings UI
- Fichier à modifier : `apps/central/src/services/miyuvoice/rooms.rs`
  - Strings UI
- Fichier à modifier : `apps/central/src/services/miyuvoice/settings.rs`
  - Strings UI

**F-1-5 : Renommer le répertoire UI**
- Action : renommer `apps/central/src/services/miyuvoice/` → `apps/central/src/services/alicia/`
- Fichier à modifier : `apps/central/src/services/mod.rs` (import du module)
- Fichier à modifier : tout endroit qui référence le module `miyuvoice`

**F-1-6 : Vérification compilation et tests**
- Commande : `cargo build --workspace`
- Commande : `cargo test --workspace`
- Commande : `cargo clippy --workspace -- -D warnings`
- Livrable : tous les tests verts, zéro warning clippy

### Tâches Lise [L]

**L-1-1 : Vérifier les textes UI après renommage Francois**
- BLOQUE PAR : F-1-4, F-1-5
- Lancer l'application Dioxus
- Vérifier que tous les textes affichés disent "Alicia" et non "MiyukiniVoice"
- Livrable : capture d'écran ou rapport textuel des textes vérifiés

### Jalon Phase 1

- `cargo build --workspace` compile
- `cargo test --workspace` passe
- Zéro occurrence de "MiyukiniVoice" dans les strings utilisateur (hors commentaires internes)
- Répertoires renommés

---

## PHASE 2 — Crates domotique backend (2 semaines)

### Objectif

Créer les crates d'infrastructure domotique : registre des dispositifs, client MQTT, client HTTP local. Pas encore de logique métier Alicia.

### Structure des crates à créer

```
crates/
  miyualicia-devices/     # Registre et types dispositifs
  miyualicia-mqtt/        # Client MQTT rumqttc
  miyualicia-http/        # Client HTTP local pour dispositifs
```

### Tâches Francois [F]

**F-2-1 : Créer `crates/miyualicia-devices/`**
- BLOQUE PAR : Phase 1 complète
- Répertoire : `crates/miyualicia-devices/`
- Fichier à créer : `crates/miyualicia-devices/Cargo.toml`
  ```toml
  [package]
  name = "miyualicia-devices"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  serde = { version = "1", features = ["derive"] }
  uuid = { version = "1", features = ["v4"] }
  chrono = { version = "0.4", features = ["serde"] }
  thiserror = "1"
  tracing = "0.1"
  ```
- Fichier à créer : `crates/miyualicia-devices/src/lib.rs`
  - @id: toolkit.alicia.devices
  - @role: device_registry
  - @layer: 6
  - Exports : `DeviceType`, `DeviceProtocol`, `DeviceCapabilities`, `DeviceConfig`, `Device`, `DeviceState`, `DeviceRegistry`, `DeviceError`
- Fichier à créer : `crates/miyualicia-devices/src/types.rs`
  ```rust
  pub enum DeviceType { Light, Shutter, Thermostat, Outlet, Sensor, Lock }
  pub enum DeviceProtocol { Mqtt, HttpLocal, Zigbee2Mqtt }
  pub struct DeviceCapabilities { pub on_off: bool, pub dimmer: bool, pub rgb: bool, pub position: bool, pub temperature_target: bool, pub power_measure: bool }
  pub struct Device { pub id: Uuid, pub room_id: String, pub device_type: DeviceType, pub name: String, pub protocol: DeviceProtocol, pub address: String, pub capabilities: DeviceCapabilities, pub active: bool, pub created_at: DateTime<Utc>, pub updated_at: DateTime<Utc> }
  pub struct DeviceState { pub device_id: Uuid, pub on: Option<bool>, pub brightness: Option<u8>, pub color_rgb: Option<(u8,u8,u8)>, pub position: Option<u8>, pub temperature_current: Option<f32>, pub temperature_target: Option<f32>, pub power_w: Option<f32>, pub locked: Option<bool>, pub motion: Option<bool>, pub contact: Option<bool>, pub humidity: Option<f32>, pub updated_at: DateTime<Utc> }
  ```
- Fichier à créer : `crates/miyualicia-devices/src/registry.rs`
  - `DeviceRegistry` : HashMap<Uuid, (Device, DeviceState)>
  - Méthodes : `new()`, `add_device()`, `remove_device()`, `get_device()`, `get_state()`, `update_state()`, `list_by_room()`, `list_all()`
- Fichier à créer : `crates/miyualicia-devices/src/errors.rs`
- Fichier à créer : `crates/miyualicia-devices/src/admin_cell.rs`
- Ajouter `miyualicia-devices` au `Cargo.toml` workspace

**F-2-2 : Créer `crates/miyualicia-mqtt/`**
- BLOQUE PAR : F-2-1
- Répertoire : `crates/miyualicia-mqtt/`
- Fichier à créer : `crates/miyualicia-mqtt/Cargo.toml`
  ```toml
  [package]
  name = "miyualicia-mqtt"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  miyualicia-devices = { path = "../miyualicia-devices" }
  rumqttc = "0.24"
  tokio = { version = "1", features = ["full"] }
  serde = { version = "1", features = ["derive"] }
  serde_json = "1"
  thiserror = "1"
  tracing = "0.1"
  ```
- Fichier à créer : `crates/miyualicia-mqtt/src/lib.rs`
  - @id: toolkit.alicia.mqtt
  - @role: mqtt_protocol_adapter
  - @layer: 6
  - Exports : `MqttConfig`, `MqttClient`, `MqttMessage`, `MqttError`
- Fichier à créer : `crates/miyualicia-mqtt/src/config.rs`
  ```rust
  pub struct MqttConfig {
      pub broker_host: String,   // défaut: "localhost"
      pub broker_port: u16,      // défaut: 1883
      pub client_id: String,     // "alicia-home"
      pub keepalive_secs: u64,   // 60
      pub reconnect_delay_secs: u64, // 5
  }
  impl Default for MqttConfig { ... }
  ```
- Fichier à créer : `crates/miyualicia-mqtt/src/client.rs`
  - `MqttClient` wrappant `rumqttc::AsyncClient`
  - Méthodes : `connect()`, `subscribe(topic)`, `publish(topic, payload, qos)`, `disconnect()`
  - Reconnexion automatique transparente
  - Canaux tokio pour recevoir les messages entrants
- Fichier à créer : `crates/miyualicia-mqtt/src/zigbee2mqtt.rs`
  - Parsing des messages Zigbee2MQTT (`zigbee2mqtt/{name}`)
  - Fonction `parse_z2m_state(payload) -> DeviceState`
  - Fonction `build_z2m_command(state) -> serde_json::Value`
- Fichier à créer : `crates/miyualicia-mqtt/src/errors.rs`
- Ajouter `miyualicia-mqtt` au workspace

**F-2-3 : Créer `crates/miyualicia-http/`**
- BLOQUE PAR : F-2-1
- Répertoire : `crates/miyualicia-http/`
- Fichier à créer : `crates/miyualicia-http/Cargo.toml`
  ```toml
  [package]
  name = "miyualicia-http"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  miyualicia-devices = { path = "../miyualicia-devices" }
  reqwest = { version = "0.12", features = ["json"] }
  tokio = { version = "1", features = ["full"] }
  serde = { version = "1", features = ["derive"] }
  serde_json = "1"
  thiserror = "1"
  tracing = "0.1"
  ```
- Fichier à créer : `crates/miyualicia-http/src/lib.rs`
  - @id: toolkit.alicia.http
  - @role: http_local_protocol_adapter
  - @layer: 6
  - Exports : `HttpDeviceClient`, `HttpDeviceConfig`, `HttpError`
- Fichier à créer : `crates/miyualicia-http/src/client.rs`
  - `HttpDeviceClient` avec timeout strict 2s
  - Méthodes : `get_state(url) -> DeviceState`, `send_command(url, payload) -> Result<()>`
  - 1 retry automatique sur erreur réseau
  - Authentification Basic et Bearer
- Fichier à créer : `crates/miyualicia-http/src/adapters/mod.rs`
  - `shelly.rs` : parsing API Shelly Gen1/Gen2
  - `generic.rs` : adapter générique JSON configurable
- Fichier à créer : `crates/miyualicia-http/src/errors.rs`
- Ajouter `miyualicia-http` au workspace

**F-2-4 : Tests unitaires des 3 crates**
- Tests `miyualicia-devices` : construction registry, CRUD, filtrage par pièce
- Tests `miyualicia-mqtt` : config par défaut, parsing Zigbee2MQTT (fixtures JSON)
- Tests `miyualicia-http` : parsing réponses Shelly (fixtures JSON)
- Commande : `cargo test -p miyualicia-devices -- --nocapture`
- Commande : `cargo test -p miyualicia-mqtt -- --nocapture`
- Commande : `cargo test -p miyualicia-http -- --nocapture`

### Jalon Phase 2

- 3 crates compilent (`cargo build -p miyualicia-devices miyualicia-mqtt miyualicia-http`)
- Tests unitaires passent
- Clippy propre

---

## PHASE 3 — Orchestrateur Alicia principal (2 semaines)

### Objectif

Créer `miyualicia`, l'orchestrateur central qui connecte : pipeline vocal, dispositifs domotique, état maison, et bridge vers miou-llm-bridge pour le NLU.

### Tâches Denis [D]

**D-3-1 : Spécifier l'interface NLU miou-llm-bridge**
- Lire : `apps/miou-llm-bridge/src/inference.rs`, `apps/miou-llm-bridge/src/llm_api.rs`
- Déterminer comment appeler le bridge pour la transcription STT et le NLU
- Fichier à créer : `docs/alicia-home/specs/nlu-bridge-interface.md`
- Livrable : type `Intent` Rust attendu, format de requête au bridge

**D-3-2 : Spécifier le format des intentions domotiques**
- Fichier à créer : `docs/alicia-home/specs/intent-taxonomy.md`
- Contenu :
  ```
  Intent::ControlDevice { device_type, room_id, action, value }
  Intent::QueryState { target }
  Intent::ActivateRoutine { routine_name }
  Intent::Unknown { transcript }
  ```

### Tâches Francois [F]

**F-3-1 : Créer `crates/miyualicia/`**
- BLOQUE PAR : Phase 2 complète, D-3-1, D-3-2
- Répertoire : `crates/miyualicia/`
- Fichier à créer : `crates/miyualicia/Cargo.toml`
  ```toml
  [package]
  name = "miyualicia"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  miyualicia-devices = { path = "../miyualicia-devices" }
  miyualicia-mqtt = { path = "../miyualicia-mqtt" }
  miyualicia-http = { path = "../miyualicia-http" }
  miyualicia-capture = { path = "../miyualicia-capture" }
  miyualicia-wakeword = { path = "../miyualicia-wakeword" }
  kindmother = { path = "../kindmother" }
  tokio = { version = "1", features = ["full"] }
  serde = { version = "1", features = ["derive"] }
  serde_json = "1"
  reqwest = { version = "0.12", features = ["json"] }
  thiserror = "1"
  tracing = "0.1"
  chrono = { version = "0.4", features = ["serde"] }
  uuid = { version = "1", features = ["v4"] }
  ```

- Fichier à créer : `crates/miyualicia/src/lib.rs`
  - @id: service.alicia.orchestrator
  - @role: home_assistant_orchestrator
  - @layer: 7
  - Exports : `AliciaService`, `AliciaConfig`, `AliciaSnapshot`, `AliciaError`

- Fichier à créer : `crates/miyualicia/src/service.rs`
  ```rust
  pub struct AliciaService {
      device_registry: Arc<RwLock<DeviceRegistry>>,
      mqtt_client: Option<Arc<MqttClient>>,
      http_client: Arc<HttpDeviceClient>,
      nlu_bridge: Arc<NluBridge>,
      voice_service: Arc<Mutex<AliciaVoiceCore>>,
      command_log: Arc<Mutex<Vec<CommandLogEntry>>>,
      config: AliciaConfig,
  }
  impl AliciaService {
      pub async fn new(config: AliciaConfig) -> Result<Self, AliciaError>
      pub fn snapshot(&self) -> AliciaSnapshot
      pub async fn execute_command(&self, cmd: DeviceCommand) -> Result<(), AliciaError>
      pub async fn dispatch_intent(&self, intent: Intent) -> Result<String, AliciaError>
      pub async fn connect_mqtt(&self) -> Result<(), AliciaError>
      pub async fn reconnect_mqtt(&self) -> Result<(), AliciaError>
  }
  ```

- Fichier à créer : `crates/miyualicia/src/config.rs`
  ```rust
  pub struct AliciaConfig {
      pub mqtt: Option<MqttConfig>,
      pub api_port: u16,                    // 7890
      pub llm_bridge_url: String,           // "http://127.0.0.1:3003"
      pub wake_word: WakeWordConfig,
      pub rooms: Vec<RoomConfig>,           // les 4 pièces + extensibles
      pub db_path: String,
  }
  ```
  - Chargement depuis `alicia.toml` (TOML, serde)

- Fichier à créer : `crates/miyualicia/src/intent.rs`
  ```rust
  pub enum Intent {
      ControlDevice { device_type: String, room_id: Option<String>, action: String, value: Option<serde_json::Value> },
      QueryState { target: String },
      ActivateRoutine { routine_name: String },
      Unknown { transcript: String },
  }
  ```

- Fichier à créer : `crates/miyualicia/src/nlu_bridge.rs`
  - `NluBridge` : appel HTTP vers miou-llm-bridge
  - Méthode `transcribe(audio_samples: Vec<f32>) -> Result<String, AliciaError>`
  - Méthode `parse_intent(transcript: String) -> Result<Intent, AliciaError>`
  - Fallback : regex parser si bridge indisponible (voir F-3-3)

- Fichier à créer : `crates/miyualicia/src/command.rs`
  ```rust
  pub struct DeviceCommand {
      pub device_id: Uuid,
      pub action: String,                   // "on", "off", "set_brightness", etc.
      pub value: Option<serde_json::Value>,
      pub source: CommandSource,
  }
  pub enum CommandSource { Voice, Api, Automation, Manual }
  ```
  - Dispatcher : selon `DeviceProtocol`, route vers MQTT ou HTTP local

- Fichier à créer : `crates/miyualicia/src/snapshot.rs`
  ```rust
  pub struct AliciaSnapshot {
      pub rooms: Vec<RoomSnapshot>,
      pub any_listening: bool,
      pub mqtt_connected: bool,
      pub activity_log: Vec<ActivityEntry>,
      pub wake_word_config: WakeWordConfigSnapshot,
  }
  pub struct RoomSnapshot {
      pub room_id: String,
      pub room_name: String,
      pub devices: Vec<(Device, DeviceState)>,
      pub mic_active: bool,
      pub vad_state: VadState,
  }
  ```

- Fichier à créer : `crates/miyualicia/src/db.rs`
  - Fonctions d'accès KindMother : `load_devices()`, `save_device()`, `log_command()`, `load_automations()`
  - Migration SQL : tables `alicia_devices`, `alicia_device_states`, `alicia_commands_log`, `alicia_automations`, `alicia_api_tokens`

- Fichier à créer : `crates/miyualicia/src/voice_pipeline.rs`
  - Intégration du pipeline vocal existant (ex-VoiceService) dans AliciaService
  - Après détection wake word → appel `nlu_bridge.transcribe()` → `nlu_bridge.parse_intent()` → `dispatch_intent()`

- Fichier à créer : `crates/miyualicia/src/errors.rs`
- Fichier à créer : `crates/miyualicia/src/admin_cell.rs`
- Ajouter `miyualicia` au workspace

**F-3-2 : Créer les migrations KindMother**
- BLOQUE PAR : F-3-1 (fichier db.rs)
- Fichier à créer : `crates/miyualicia/migrations/001_initial.sql`
  - Contenu : toutes les tables décrites dans le Rapport Fondateur section 4.2 BT-03
  - Tables : `alicia_devices`, `alicia_device_states`, `alicia_commands_log`, `alicia_automations`, `alicia_api_tokens`

**F-3-3 : Fallback NLU (parser regex)**
- BLOQUE PAR : F-3-1
- Fichier à créer : `crates/miyualicia/src/nlu_fallback.rs`
- Patterns regex pour les commandes les plus communes :
  - "allume|éteins|active|désactive" + "lumière|lampe|prise" + pièce optionnelle
  - "règle|mets" + "thermostat|chauffage" + "à \d+" degrés"
  - "ouvre|ferme" + "volet|store|rideau"
  - Retourne `Intent::Unknown` si aucun pattern ne correspond
- Ce module est le fallback si miou-llm-bridge est hors ligne

**F-3-4 : Config TOML `alicia.toml`**
- Fichier à créer : `apps/central/alicia.toml.example`
  ```toml
  [alicia]
  api_port = 7890
  llm_bridge_url = "http://127.0.0.1:3003"
  db_path = "./data/alicia.db"

  [mqtt]
  broker_host = "localhost"
  broker_port = 1883
  client_id = "alicia-home"

  [[rooms]]
  id = "chambre-theresa"
  name = "Chambre Theresa"

  [[rooms]]
  id = "chambre-parentale"
  name = "Chambre parentale"

  [[rooms]]
  id = "chambre-eleanore"
  name = "Chambre Eleanore"

  [[rooms]]
  id = "salon"
  name = "Salon"
  ```

**F-3-5 : Tests unitaires orchestrateur**
- Test `dispatch_intent` avec mock DeviceRegistry
- Test `nlu_fallback` avec phrases de commande types
- Test `DeviceCommand` dispatcher : routing MQTT vs HTTP
- Commande : `cargo test -p miyualicia -- --nocapture`

### Jalon Phase 3

- `miyualicia` compile
- Pipeline vocal → NLU → dispatch fonctionne en test (mock)
- `alicia.toml.example` disponible

---

## PHASE 4 — API REST sécurisée (1.5 semaines)

### Objectif

Créer `crates/miyualicia-api` : serveur axum exposant l'API REST documentée dans le Rapport Fondateur, avec authentification JWT.

### Tâches Francois [F]

**F-4-1 : Créer `crates/miyualicia-api/`**
- BLOQUE PAR : Phase 3 complète (miyualicia doit exister)
- Répertoire : `crates/miyualicia-api/`
- Fichier à créer : `crates/miyualicia-api/Cargo.toml`
  ```toml
  [package]
  name = "miyualicia-api"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  miyualicia = { path = "../miyualicia" }
  miyualicia-devices = { path = "../miyualicia-devices" }
  axum = { version = "0.7", features = ["tokio", "json"] }
  tower = "0.4"
  tower-http = { version = "0.5", features = ["cors", "trace", "limit"] }
  tokio = { version = "1", features = ["full"] }
  serde = { version = "1", features = ["derive"] }
  serde_json = "1"
  jsonwebtoken = "9"
  chrono = { version = "0.4", features = ["serde"] }
  uuid = { version = "1", features = ["v4"] }
  thiserror = "1"
  tracing = "0.1"
  ```

- Fichier à créer : `crates/miyualicia-api/src/lib.rs`
  - @id: service.alicia.rest-api
  - @role: http_api_gateway
  - @layer: 7
  - Exports : `AliciaApiServer`, `ApiConfig`, `ApiError`

- Fichier à créer : `crates/miyualicia-api/src/server.rs`
  ```rust
  pub struct AliciaApiServer {
      config: ApiConfig,
      alicia: Arc<AliciaService>,
  }
  impl AliciaApiServer {
      pub async fn start(self) -> Result<(), ApiError>  // bind sur config.port
  }
  ```

- Fichier à créer : `crates/miyualicia-api/src/router.rs`
  - Définition des routes axum (préfixe `/api/v1/alicia`)
  - Middleware : JWT extractor sur routes protégées
  - Middleware : rate limiting (100 req/min par IP)
  - Route publique : `POST /auth/token`

- Fichier à créer : `crates/miyualicia-api/src/auth.rs`
  ```rust
  pub struct JwtClaims { pub sub: String, pub exp: u64, pub scopes: Vec<String> }
  pub struct JwtMiddleware;  // axum extractor
  pub fn generate_token(client_id: &str, scopes: Vec<String>, secret: &[u8]) -> Result<String, ApiError>
  pub fn verify_token(token: &str, secret: &[u8]) -> Result<JwtClaims, ApiError>
  ```
  - Token durée : 1h par défaut, configurable
  - Secret : généré à la première installation, stocké en KindMother chiffré

- Fichier à créer : `crates/miyualicia-api/src/handlers/mod.rs`
  - `state.rs` : `GET /state`
  - `rooms.rs` : `GET /rooms`, `GET /rooms/{id}`, `GET /rooms/{id}/devices`
  - `devices.rs` : `GET /devices`, `GET /devices/{id}`, `POST /devices/{id}/command`
  - `automations.rs` : CRUD `/automations`
  - `history.rs` : `GET /history` (pagination)
  - `health.rs` : `GET /health` (sans auth)

- Fichier à créer : `crates/miyualicia-api/src/dto.rs`
  - DTOs de réponse JSON : `DeviceDto`, `RoomDto`, `StateDto`, `CommandRequest`, `CommandResponse`
  - Tous avec `#[derive(Serialize, Deserialize)]`

- Fichier à créer : `crates/miyualicia-api/src/errors.rs`
  - `ApiError` → codes HTTP appropriés (401, 403, 404, 422, 500)
  - `impl IntoResponse for ApiError`

**F-4-2 : Tests API REST**
- Tests d'intégration avec `axum::test` ou client reqwest local
- Test auth : token invalide → 401
- Test commande : device inconnu → 404
- Test rate limiting : > 100 req/min → 429
- Fichier à créer : `crates/miyualicia-api/src/tests/mod.rs`

**F-4-3 : Intégration dans `apps/central`**
- Fichier à modifier : `apps/central/src/main.rs` ou service d'initialisation
- Lancer `AliciaApiServer::start()` dans un `tokio::spawn` au démarrage de Central
- Ne pas bloquer l'UI si le port est déjà pris

**F-4-4 : Documentation API**
- Fichier à créer : `docs/alicia-home/Alicia Home - API REST Reference.md`
- Format : chaque endpoint avec méthode, URL, corps, réponse, exemple curl

### Jalon Phase 4

- API démarre sur le port 7890
- `POST /auth/token` retourne un JWT valide
- `GET /state` avec JWT valide retourne l'état JSON
- `POST /devices/{id}/command` exécute une commande

---

## PHASE 5 — Moteur d'automatisations (1.5 semaines)

### Objectif

Créer `crates/miyualicia-automations` : scheduler, évaluateur de conditions, exécuteur de routines.

### Tâches Francois [F]

**F-5-1 : Créer `crates/miyualicia-automations/`**
- BLOQUE PAR : Phase 3 complète
- Répertoire : `crates/miyualicia-automations/`
- Fichier à créer : `crates/miyualicia-automations/Cargo.toml`
  ```toml
  [package]
  name = "miyualicia-automations"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  miyualicia = { path = "../miyualicia" }
  miyualicia-devices = { path = "../miyualicia-devices" }
  tokio = { version = "1", features = ["full"] }
  tokio-cron-scheduler = "0.10"
  serde = { version = "1", features = ["derive"] }
  serde_json = "1"
  thiserror = "1"
  tracing = "0.1"
  chrono = { version = "0.4", features = ["serde"] }
  uuid = { version = "1", features = ["v4"] }
  ```

- Fichier à créer : `crates/miyualicia-automations/src/lib.rs`
  - @id: toolkit.alicia.automations
  - @role: automation_engine
  - @layer: 6
  - Exports : `AutomationEngine`, `Automation`, `Trigger`, `Condition`, `Action`, `AutomationError`

- Fichier à créer : `crates/miyualicia-automations/src/types.rs`
  ```rust
  pub enum TriggerType {
      Cron { expression: String },
      SensorChange { device_id: Uuid, property: String, threshold: serde_json::Value },
      VoiceCommand { routine_name: String },
      ApiEvent { event_name: String },
  }
  pub enum ConditionOp { Eq, Gt, Lt, Gte, Lte, Between }
  pub struct Condition {
      pub device_id: Option<Uuid>,
      pub property: String,
      pub op: ConditionOp,
      pub value: serde_json::Value,
  }
  pub struct Action {
      pub device_id: Uuid,
      pub command: String,
      pub value: Option<serde_json::Value>,
      pub delay_ms: Option<u64>,
  }
  pub struct Automation {
      pub id: Uuid,
      pub name: String,
      pub enabled: bool,
      pub trigger: TriggerType,
      pub conditions: Vec<Condition>,
      pub actions: Vec<Action>,
  }
  ```

- Fichier à créer : `crates/miyualicia-automations/src/engine.rs`
  - `AutomationEngine` : gère le scheduler tokio-cron-scheduler
  - Méthodes : `start()`, `stop()`, `add_automation()`, `remove_automation()`, `trigger_automation(id)`, `evaluate_conditions(automation, snapshot) -> bool`
  - Loop d'écoute des événements capteurs (tokio channel depuis AliciaService)
  - Exécution séquentielle des `Action` avec délai optionnel

- Fichier à créer : `crates/miyualicia-automations/src/evaluator.rs`
  - `evaluate_condition(cond, snapshot) -> bool`
  - Support opérateurs : Eq, Gt, Lt, Between

- Fichier à créer : `crates/miyualicia-automations/src/parser.rs`
  - Parsing TOML d'une automatisation → `Automation`
  - Validation schema : déclencheur obligatoire, au moins 1 action

- Fichier à créer : `crates/miyualicia-automations/src/errors.rs`

**F-5-2 : Intégration automatisations dans `miyualicia`**
- Fichier à modifier : `crates/miyualicia/src/service.rs`
  - Ajouter `automation_engine: Arc<AutomationEngine>` dans `AliciaService`
  - Au démarrage : charger les automatisations depuis KindMother, les injecter dans le moteur
  - Émettre les événements capteurs vers le moteur

**F-5-3 : Automatisations prédéfinies**
- Fichier à créer : `apps/central/automations/bonne-nuit.toml.example`
  ```toml
  [[automations]]
  name = "Bonne nuit"
  trigger = { type = "voice", routine_name = "bonne nuit" }
  actions = [
    { device_type = "light", room = "salon", command = "off" },
    { device_type = "thermostat", room = "chambre-parentale", command = "set_temperature", value = 18.0, delay_ms = 0 },
    { device_type = "lock", command = "lock", delay_ms = 1000 },
  ]
  ```

**F-5-4 : Tests moteur d'automatisations**
- Test évaluateur : conditions vraies/fausses avec snapshot mock
- Test déclenchement cron : schedule expression, tick simulé
- Test routines vocales : mapping nom routine → automation

### Jalon Phase 5

- AutomationEngine démarre et charge les automatisations depuis DB
- Routine "bonne nuit" déclenche les actions configurées
- Automatisations CRUD via API REST (Phase 4)

---

## PHASE 6 — UI Dioxus : refonte complète (2 semaines)

### Objectif

Refondre entièrement l'interface Alicia dans `apps/central/src/services/alicia/` pour supporter domotique, automatisations, contrôle dispositifs.

### Rappel des pièges RSX Dioxus 0.6

- Pas de nested braces dans les format strings : extraire en variable locale
- Pas de named format args dans les text nodes
- Pas de read+set sur le même signal dans une expression

### Tâches Lise [L]

**L-6-1 : Refonte `dashboard.rs`**
- BLOQUE PAR : Phase 3 (AliciaSnapshot disponible)
- Fichier à modifier : `apps/central/src/services/alicia/dashboard.rs`
- Nouveau contenu :
  - Vue synthèse : "N dispositifs actifs", "M automatisations actives", badge MQTT
  - Grille des pièces : chaque pièce = mini-carte avec ses dispositifs et statuts
  - Chaque dispositif dans la carte : icône type + nom + état + bouton action rapide
  - Journal d'activité récent (20 dernières entrées)
  - Zone alertes : dispositifs en erreur, MQTT déconnecté

**L-6-2 : Créer `devices.rs`**
- BLOQUE PAR : Phase 3
- Fichier à créer : `apps/central/src/services/alicia/devices.rs`
- Composant `DevicesScreen` :
  - Liste tous les dispositifs par pièce (accordéon pièce)
  - Pour chaque dispositif : nom, type, protocole, état live, boutons commandes
  - Lumière : toggle + slider luminosité (0-100) + sélecteur couleur si RGB
  - Thermostat : affichage temp actuelle + input temp cible + sélecteur mode
  - Volet : boutons Ouvrir/Arrêter/Fermer + slider position
  - Prise : toggle + indicateur consommation si disponible
  - Capteur : valeurs live (temp, humidité, mouvement)
  - Serrure : boutons Verrouiller/Déverrouiller + indicateur statut
  - Bouton "Ajouter un dispositif" → modal formulaire

**L-6-3 : Créer `device_form.rs`**
- BLOQUE PAR : L-6-2
- Fichier à créer : `apps/central/src/services/alicia/device_form.rs`
- Composant `DeviceFormModal` :
  - Sélecteur pièce (dropdown parmi les 4 pièces)
  - Sélecteur type dispositif (dropdown)
  - Sélecteur protocole (MQTT, HTTP local, Zigbee2MQTT)
  - Champ adresse (topic MQTT ou URL HTTP)
  - Champ nom
  - Cases à cocher capacités (on_off, dimmer, rgb, etc.)
  - Boutons Sauvegarder / Annuler
  - Envoi vers `SharedAliciaService` → `add_device()`

**L-6-4 : Créer `automations.rs`**
- BLOQUE PAR : Phase 5 (AutomationEngine)
- Fichier à créer : `apps/central/src/services/alicia/automations.rs`
- Composant `AutomationsScreen` :
  - Liste des automatisations : nom, déclencheur, actif/inactif, toggle activer, bouton déclencher maintenant
  - Badge couleur selon type de déclencheur (cron = bleu, capteur = orange, vocal = vert)
  - Bouton "Nouvelle automatisation" → `AutomationFormModal`
  - Bouton supprimer avec confirmation

**L-6-5 : Créer `automation_form.rs`**
- BLOQUE PAR : L-6-4
- Fichier à créer : `apps/central/src/services/alicia/automation_form.rs`
- Composant `AutomationFormModal` :
  - Champ nom
  - Sélecteur type déclencheur (cron, capteur, vocal, événement API)
  - Si cron : input expression cron (avec aide)
  - Si capteur : sélecteur dispositif, sélecteur propriété, opérateur, valeur seuil
  - Si vocal : input nom de routine
  - Section actions : liste dynamique d'actions (ajouter/supprimer), pour chaque action : sélecteur dispositif, commande, valeur, délai

**L-6-6 : Refonte `settings.rs`**
- Fichier à modifier : `apps/central/src/services/alicia/settings.rs`
- Nouvelles sections :
  - "Connexion MQTT" : host, port, statut connexion, bouton tester/reconnecter
  - "API REST" : port, statut (écoute/arrêté), gestion des tokens (liste, créer, révoquer)
  - "Services COG" : URL miou-llm-bridge, statut, test connexion
  - "Wake Word" : config existante (conservée)
  - "Pièces" : liste des pièces, ajouter/supprimer

**L-6-7 : Refonte `mod.rs`**
- Fichier à modifier : `apps/central/src/services/alicia/mod.rs`
- Nouveaux onglets : Tableau de bord | Dispositifs | Automatisations | Parametres
- Enum `AliciaTab` : Dashboard, Devices, Automations, Settings
- Remplacer `AliciaTab::Rooms` par `AliciaTab::Devices`
- Polling toujours 200ms sur `AliciaSnapshot` (inchangé)
- En-tête : badge "Alicia ecoute/veille" + badge "MQTT : connecté/déconnecté"

**L-6-8 : Créer les composants réutilisables**
- Fichier à créer : `apps/central/src/services/alicia/components/mod.rs`
- Fichier à créer : `apps/central/src/services/alicia/components/device_badge.rs`
  - Composant `DeviceBadge { device_type, state }` : icône + couleur selon état
- Fichier à créer : `apps/central/src/services/alicia/components/status_pill.rs`
  - `StatusPill { label, color, dot }` : badge générique (réutilisé partout)
- Fichier à créer : `apps/central/src/services/alicia/components/slider.rs`
  - `PercentageSlider { value, on_change }` : slider 0-100% accessible

**L-6-9 : Tests UI (vérification manuelle)**
- Vérifier que les pièges RSX ne déclenchent aucune erreur de compilation
- Vérifier polling 200ms : les états domotique s'actualisent en temps réel
- Vérifier l'affichage avec 0 dispositif, 1 dispositif, 10 dispositifs
- Vérifier le formulaire de création de dispositif (validation des champs vides)

### Jalon Phase 6

- 4 onglets fonctionnels (Dashboard, Dispositifs, Automatisations, Parametres)
- Création d'un dispositif depuis l'UI
- Commande d'un dispositif (toggle lumière) depuis l'UI
- Formulaire automatisation fonctionnel

---

## PHASE 7 — Intégration COG complète (1 semaine)

### Objectif

Connecter Alicia à tous les services COG : miou-llm-bridge (NLU live), miyunotify (alertes), KindMother (persistance finale). Validation end-to-end du pipeline vocal.

### Tâches Francois [F]

**F-7-1 : Pipeline NLU vocal en conditions réelles**
- BLOQUE PAR : Phase 3, Phase 6
- Tester avec miou-llm-bridge réel démarré sur localhost:3003
- Flux : wake word → audio buffer → transcription STT → NLU → intent → dispatch → commande dispositif
- Mesurer latence totale : objectif < 1.5s
- Ajuster si nécessaire (timeout STT, taille buffer audio transmis)

**F-7-2 : Intégration miyunotify**
- Fichier à modifier : `crates/miyualicia/src/service.rs`
- Ajouter envoi notification via miyunotify lors :
  - Détection wake word (si configuré)
  - Exécution automatisation (notification optionnelle par automation)
  - Erreur dispositif critique (device unreachable)
- Import : `use miyunotify::NotifyClient;`

**F-7-3 : Persistance KindMother complète**
- Vérifier que toutes les actions sont persistées dans `alicia_commands_log`
- Vérifier que les états dispositifs sont mis à jour dans `alicia_device_states` à chaque changement MQTT
- Vérifier que les automatisations créées via UI/API sont sauvegardées et rechargées au redémarrage

**F-7-4 : Service météo local**
- Fichier à créer : `crates/miyualicia/src/weather.rs`
- Appel `api.open-meteo.com` (pas de clé API, conforme Loi Autonomie car service public sans inscription)
- Retourne : `WeatherSummary { temperature_c, condition, humidity_pct, wind_kmh }`
- Cache : 10 minutes en mémoire
- Utilisé pour : réponses vocales météo, conditions d'automatisation (si pluvieux → fermer les volets)
- Optionnel si réseau indisponible : retourne `None`, pas d'erreur bloquante

### Tâches Lise [L]

**L-7-1 : Affichage météo dans Dashboard**
- BLOQUE PAR : F-7-4
- Fichier à modifier : `apps/central/src/services/alicia/dashboard.rs`
- Ajouter carte "Météo" dans le résumé global si données disponibles

### Jalon Phase 7

- Pipeline vocal complet testé en conditions réelles
- Notifications miyunotify reçues lors d'événements Alicia
- Redémarrage du COG : Alicia recharge tous les dispositifs et automatisations depuis KindMother

---

## PHASE 8 — Tests, audit George, corrections (1.5 semaines)

### Objectif

Audit complet par George : conformité COG, sécurité API, UX. Correction de tous les points soulevés.

### Tâches George [G]

**G-8-1 : Audit conformité COG**
- Vérifier : toutes les Lois d'Autonomie respectées
  - Loi 1 : Alicia démarre sans réseau (MQTT optionnel, LLM bridge graceful degradation)
  - Loi 2 : Isolation testée (couper le LAN, Alicia reste fonctionnel localement)
  - Loi 3 : État local complet en KindMother
  - Loi 5 : Pas de service cloud obligatoire
  - Loi 7 : Crates Cores non modifiés
- Vérifier : annotations MSCM (@id, @do, @role, @layer, @human) dans tous les nouveaux fichiers
- Vérifier : `#![forbid(unsafe_code)]` dans tous les Cargo.toml nouveaux

**G-8-2 : Audit sécurité API**
- Test JWT : token expiré → 401, token invalide → 401, token valide → 200
- Test rate limiting : dépasser 100 req/min → 429
- Test injection : payload malformé → 422 sans panic
- Test accès sans token → 401
- Vérifier : aucune clé JWT dans les logs
- Vérifier : audit trail complet dans `alicia_commands_log`

**G-8-3 : Audit UX**
- Navigation : tous les onglets accessibles, retour toujours possible
- États d'erreur : messages clairs (MQTT déconnecté, dispositif injoignable)
- États de chargement : spinner ou message pendant initialisation
- Formulaires : validation côté UI avant envoi (champs obligatoires, formats)
- Cohérence visuelle : respect du thème COG existant

**G-8-4 : Tests globaux**
- `cargo test --workspace` : zéro test en échec
- `cargo clippy --workspace -- -D warnings` : zéro warning
- Test de démarrage complet : COG démarre, Alicia s'initialise, API répond sur 7890

**G-8-5 : Rapport d'audit**
- Fichier à créer : `docs/alicia-home/Alicia Home - Rapport Audit George.md`
- Format : liste des points testés, statut (OK/KO), corrections requises

### Tâches Francois + Lise (corrections)

**F-8-1 / L-8-1 : Corrections issues de l'audit George**
- BLOQUE PAR : G-8-5
- Traiter tous les points KO du rapport George
- Re-run `cargo test --workspace` après chaque correction
- Re-run audit concerné par George pour validation

### Jalon Phase 8

- Rapport George : tous les points statut OK
- `cargo test --workspace` : tous verts
- `cargo clippy --workspace -- -D warnings` : zéro warning

---

## PHASE 9 — Documentation finale et archivage (0.5 semaine)

### Tâches Denis [D]

**D-9-1 : Documentation technique finale**
- Fichier à créer : `docs/alicia-home/Alicia Home - Guide Technique.md`
  - Comment configurer `alicia.toml`
  - Comment ajouter un dispositif MQTT
  - Comment créer une automatisation TOML
  - Comment appeler l'API REST (exemples curl)

**D-9-2 : Guide d'installation**
- Fichier à créer : `docs/alicia-home/Alicia Home - Guide Installation.md`
  - Prérequis : Mosquitto (optionnel), Zigbee2MQTT (optionnel), miou-llm-bridge
  - Configuration initiale : générer le token API, configurer les pièces
  - Premier dispositif : pas à pas complet

### Tâches Arianne [A]

**A-9-1 : Archivage complet**
- Archiver : Rapport Fondateur v1.0 (2026-03-01)
- Archiver : Plan Dev Général v1.0 (2026-03-01)
- Archiver : Rapport Audit George
- Archiver : Décisions de conception verrouillées (section 10 Rapport Fondateur)
- Créer index MIP pour les documents Alicia Home

**A-9-2 : Mise à jour MEMORY.md**
- Ajouter section "Alicia Home Assistante" dans `~/.claude/projects/.../MEMORY.md`
- Contenu : décisions verrouillées, nouveaux crates, port 7890, protocoles

### Jalon Phase 9 (= Jalon Final)

- Documentation complète disponible dans `docs/alicia-home/`
- Archives Arianne complètes
- Pull Request de merge de la branche Alicia vers main
- Alicia Home Assistante v1.0 opérationnel

---

## Résumé des fichiers à créer / modifier

### Fichiers à créer (nouveaux crates)

```
crates/miyualicia-devices/
  Cargo.toml
  src/lib.rs
  src/types.rs
  src/registry.rs
  src/errors.rs
  src/admin_cell.rs

crates/miyualicia-mqtt/
  Cargo.toml
  src/lib.rs
  src/config.rs
  src/client.rs
  src/zigbee2mqtt.rs
  src/errors.rs

crates/miyualicia-http/
  Cargo.toml
  src/lib.rs
  src/client.rs
  src/adapters/mod.rs
  src/adapters/shelly.rs
  src/adapters/generic.rs
  src/errors.rs

crates/miyualicia/
  Cargo.toml
  src/lib.rs
  src/service.rs
  src/config.rs
  src/intent.rs
  src/nlu_bridge.rs
  src/nlu_fallback.rs
  src/command.rs
  src/snapshot.rs
  src/db.rs
  src/voice_pipeline.rs
  src/weather.rs
  src/errors.rs
  src/admin_cell.rs
  migrations/001_initial.sql

crates/miyualicia-api/
  Cargo.toml
  src/lib.rs
  src/server.rs
  src/router.rs
  src/auth.rs
  src/dto.rs
  src/errors.rs
  src/handlers/mod.rs
  src/handlers/state.rs
  src/handlers/rooms.rs
  src/handlers/devices.rs
  src/handlers/automations.rs
  src/handlers/history.rs
  src/handlers/health.rs
  src/tests/mod.rs

crates/miyualicia-automations/
  Cargo.toml
  src/lib.rs
  src/types.rs
  src/engine.rs
  src/evaluator.rs
  src/parser.rs
  src/errors.rs
```

### Fichiers UI à créer (nouveaux)

```
apps/central/src/services/alicia/
  components/mod.rs
  components/device_badge.rs
  components/status_pill.rs
  components/slider.rs
  devices.rs
  device_form.rs
  automations.rs
  automation_form.rs
```

### Fichiers à modifier (renommage / évolution)

```
Cargo.toml (workspace)                         — membres, renommages
apps/central/Cargo.toml                        — dépendances
apps/central/src/services/mod.rs               — import module alicia
apps/central/src/main.rs                       — lancement API Alicia

# Renommages Phase 1
crates/miyuvoicecapture/ → crates/miyualicia-capture/
crates/miyuwakeword/ → crates/miyualicia-wakeword/
apps/central/src/services/miyuvoice/ → apps/central/src/services/alicia/

# Évolutions Phase 6
apps/central/src/services/alicia/mod.rs        — 4 onglets, AliciaTab
apps/central/src/services/alicia/dashboard.rs  — refonte complète
apps/central/src/services/alicia/rooms.rs      — retiré ou transformé en devices.rs
apps/central/src/services/alicia/settings.rs   — nouvelles sections
apps/central/src/services/alicia/state.rs      — AliciaService, AliciaSnapshot
```

### Fichiers de configuration à créer

```
apps/central/alicia.toml.example
apps/central/automations/bonne-nuit.toml.example
```

### Fichiers de documentation à créer

```
docs/alicia-home/Alicia Home Assistante - Rapport Fondateur.md  (ce document)
docs/alicia-home/Alicia Home Assistante - Plan Dev General.md   (ce document)
docs/alicia-home/Alicia Home - API REST Reference.md
docs/alicia-home/Alicia Home - Guide Technique.md
docs/alicia-home/Alicia Home - Guide Installation.md
docs/alicia-home/Alicia Home - Rapport Audit George.md
docs/alicia-home/Alicia - Analyse Concurrence.md
docs/alicia-home/specs/miyualicia-api-spec.md
docs/alicia-home/specs/miyualicia-devices-spec.md
docs/alicia-home/specs/miyualicia-mqtt-spec.md
docs/alicia-home/specs/miyualicia-automations-spec.md
docs/alicia-home/specs/nlu-bridge-interface.md
docs/alicia-home/specs/intent-taxonomy.md
```

---

## Tableau de dépendances critiques

```
Phase 0 ─────────────────────────────────────────────────┐
    │                                                      │
    ▼                                                      ▼
Phase 1 (renommage)                              Fabrice (analyse)
    │
    ▼
Phase 2 (devices, mqtt, http)
    │
    ├──────────────────────────┐
    ▼                          ▼
Phase 3 (miyualicia)    [D-3-1 spec NLU]
    │
    ├──────────────────┬──────────────────┐
    ▼                  ▼                  ▼
Phase 4 (API)    Phase 5 (automations)  Phase 6 (UI)
    │                  │                  │
    └──────────────────┴──────────────────┘
                       │
                       ▼
                  Phase 7 (intégration COG)
                       │
                       ▼
                  Phase 8 (audit George + corrections)
                       │
                       ▼
                  Phase 9 (docs + archivage)
```

---

## Checklist de livraison finale

- [ ] `cargo build --workspace` : OK
- [ ] `cargo test --workspace` : tous verts
- [ ] `cargo clippy --workspace -- -D warnings` : zéro warning
- [ ] Zéro occurrence de "MiyukiniVoice" dans les strings utilisateur
- [ ] API REST répond sur port 7890
- [ ] JWT auth fonctionnel
- [ ] Au moins 1 dispositif MQTT contrôlable depuis l'UI
- [ ] Au moins 1 automatisation déclenchable
- [ ] Pipeline vocal end-to-end : wake word → commande domotique
- [ ] Rapport George : tous les points OK
- [ ] Documentation complète dans `docs/alicia-home/`
- [ ] Archives Arianne complètes
- [ ] `alicia.toml.example` présent

---

*Maria — Chef de Projet Miyukini AI Studio — transmis à Denis le 2026-03-01*
