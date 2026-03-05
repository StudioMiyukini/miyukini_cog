# Spec Technique — crate `miyualicia-automations`

<!-- @id: spec.alicia.automations -->
<!-- @role: technical-specification -->
<!-- @layer: 6 -->
<!-- @human: Specification technique complete du moteur d'automatisations domotiques -->
<!-- @do: define_miyualicia_automations_crate_api -->

**Auteur :** Denis, Chef Dev Senior — Miyukini AI Studio
**Date :** 2026-03-01
**Version :** 1.0
**Reference :** Rapport Fondateur Alicia Home Assistante v1.0 §3.1, §4.1 BF-04

---

## Contexte

`miyualicia-automations` est le moteur de regles et de routines d'Alicia. Il permet de
definir des automatisations sous forme de triplets `(Trigger, Conditions, Actions)` stockes
en base KindMother (format JSON interne, importables/exportables en TOML). Il integre un
scheduler `tokio-cron-scheduler` pour les declencheurs horaires, un evaluateur de conditions
sur l'etat courant de la maison, et un executeur d'actions sequentielles avec delai optionnel.

## Portee / Scope

Ce crate couvre :
- Les types de domaine : `Automation`, `TriggerType`, `Condition`, `Action`
- Le moteur `AutomationEngine` (scheduler, evaluateur, executeur)
- L'evaluateur de conditions `evaluate_conditions`
- Le parseur TOML pour les automatisations utilisateur
- Les erreurs explicites `AutomationError`

Ce crate ne couvre pas :
- La persistance (charger/sauvegarder depuis KindMother : responsabilite de `miyualicia::db`)
- L'API REST d'exposition des automatisations (responsabilite de `miyualicia-api`)
- La definition des dispositifs (responsabilite de `miyualicia-devices`)

---

## 1. Emplacement et structure

```
crates/miyualicia-automations/
├── Cargo.toml
└── src/
    ├── lib.rs          # Racine, exports publics, MSCM
    ├── admin_cell.rs   # Cellule Admin Miyukini
    ├── types.rs        # Automation, TriggerType, Condition, Action
    ├── engine.rs       # AutomationEngine (scheduler + dispatch)
    ├── evaluator.rs    # evaluate_conditions(conditions, snapshot) -> bool
    ├── executor.rs     # execute_actions(actions, alicia) -> Result<()>
    ├── parser.rs       # Parsing TOML -> Automation (+ validation schema)
    └── errors.rs       # AutomationError
```

---

## 2. `Cargo.toml`

```toml
[package]
name = "miyualicia-automations"
version = "0.1.0"
edition = "2021"
description = "Moteur d'automatisations domotiques — Alicia Home Assistante"
authors = ["Miyukini AI Studio"]

[lints.rust]
unsafe_code = "forbid"

[lints.clippy]
all = "warn"
pedantic = "warn"

[dependencies]
miyualicia-devices      = { path = "../miyualicia-devices" }

tokio                   = { version = "1", features = ["full"] }
tokio-cron-scheduler    = "0.10"
serde                   = { version = "1", features = ["derive"] }
serde_json              = "1"
toml                    = "0.8"
chrono                  = { version = "0.4", features = ["serde"] }
uuid                    = { version = "1", features = ["v4", "serde"] }
thiserror               = "1"
tracing                 = "0.1"

# miyualicia declare comme dependance optionnelle car AutomationEngine
# recoit AliciaService par injection de dependances (trait AliciaCommandDispatcher).
# Evite les dependances circulaires.
```

---

## 3. `src/lib.rs`

```rust
//! # miyualicia-automations
//!
//! Moteur d'automatisations domotiques pour Alicia Home Assistante.
//! Gere le cycle complet : declenchement → evaluation des conditions → execution des actions.
//!
//! ## Declencheurs supportes
//!
//! - `Cron` : expression cron standard (ex: "0 22 * * *" = tous les soirs 22h00)
//! - `SensorChange` : changement d'etat d'un capteur au-dela d'un seuil
//! - `VoiceCommand` : routine vocale nommee ("bonne nuit", "je pars", etc.)
//! - `ApiEvent` : evenement declenche via `POST /automations/{id}/trigger`
//!
//! ## Loi d'Autonomie
//!
//! Ce crate fonctionne 100 % en local. Le scheduler tourne dans le runtime Tokio
//! de l'application sans connexion externe. Les automatisations sont stockees
//! dans KindMother (SQLite local).

#![forbid(unsafe_code)]

// @id: toolkit.alicia.automations
// @role: automation_engine
// @layer: 6
// @human: Moteur de regles et routines Alicia ; triggers, conditions, actions, scheduler.
// @do: schedule_and_execute_home_automations

pub mod admin_cell;
pub mod engine;
pub mod errors;
pub mod evaluator;
pub mod executor;
pub mod parser;
pub mod types;

pub use engine::AutomationEngine;
pub use errors::AutomationError;
pub use evaluator::evaluate_conditions;
pub use parser::parse_automation_toml;
pub use types::{Action, Automation, Condition, ConditionOp, TriggerType};
```

---

## 4. `src/types.rs` — Types de domaine

### 4.1 `TriggerType`

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Declencheur d'une automatisation.
///
/// Un seul declencheur par automatisation. Les declencheurs sont exclusifs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TriggerType {
    /// Declencheur horaire via expression cron.
    ///
    /// # Format
    ///
    /// Expression cron 6 champs : `secondes minutes heures jours mois jours-semaine`
    ///
    /// Exemples :
    /// - `"0 0 22 * * *"` : tous les soirs a 22h00
    /// - `"0 30 6 * * 1-5"` : du lundi au vendredi a 6h30
    ///
    /// Parser : `tokio-cron-scheduler` (compatible avec la lib `cron` standard).
    Cron {
        /// Expression cron (6 champs, secondes en premier).
        expression: String,
    },

    /// Declencheur sur changement d'etat d'un dispositif.
    ///
    /// Se declenche quand la propriete `property` du dispositif `device_id`
    /// franchit le `threshold` selon l'operateur `op`.
    ///
    /// Exemple : temperature > 25 dans le salon → allumer la ventilation.
    SensorChange {
        device_id: Uuid,
        /// Nom de la propriete de `DeviceState` : "temperature_current", "motion", "contact", etc.
        property:  String,
        /// Operateur de comparaison.
        op:        ConditionOp,
        /// Valeur seuil (JSON : number, bool, string selon la propriete).
        threshold: serde_json::Value,
    },

    /// Declencheur vocal : routine appelee par nom.
    ///
    /// Quand le NLU reconnait `Intent::ActivateRoutine { routine_name }`,
    /// l'orchestrateur recherche toutes les automatisations avec ce declencheur
    /// et les execute sequentiellement.
    VoiceCommand {
        /// Nom exact de la routine (case-insensitive en matching).
        routine_name: String,
    },

    /// Declencheur via appel API explicite (`POST /automations/{id}/trigger`).
    ApiEvent {
        /// Nom optionnel de l'evenement (pour le filtrage et les logs).
        event_name: String,
    },
}

impl TriggerType {
    /// Retourne une description lisible du declencheur pour les logs.
    pub fn description(&self) -> String {
        match self {
            Self::Cron { expression } => format!("Cron({})", expression),
            Self::SensorChange { device_id, property, .. } => {
                format!("SensorChange(device={device_id}, prop={property})")
            }
            Self::VoiceCommand { routine_name } => format!("Vocal({})", routine_name),
            Self::ApiEvent { event_name } => format!("API({})", event_name),
        }
    }
}
```

### 4.2 `ConditionOp` et `Condition`

```rust
/// Operateur de comparaison pour les conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionOp {
    /// Egalite stricte.
    Eq,
    /// Strictement superieur.
    Gt,
    /// Strictement inferieur.
    Lt,
    /// Superieur ou egal.
    Gte,
    /// Inferieur ou egal.
    Lte,
    /// Dans un intervalle [min, max] inclus.
    /// La valeur `value` est un tableau JSON : `[min, max]`.
    Between,
    /// Different de.
    Ne,
}

/// Condition qui doit etre vraie pour qu'une automatisation se declenche.
///
/// Toutes les conditions d'une automatisation sont en ET logique :
/// l'automatisation ne s'execute que si TOUTES les conditions sont vraies.
///
/// # Exemples
///
/// - temperature_current > 25 dans la chambre parentale
/// - heure >= 22 (condition sur l'heure courante, device_id = None)
/// - motion == true dans le salon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    /// UUID du dispositif dont l'etat est evalue.
    /// `None` pour les conditions sur le contexte (heure, date).
    pub device_id: Option<Uuid>,

    /// Propriete de `DeviceState` a evaluer.
    /// Valeurs acceptees : "on", "brightness", "temperature_current",
    /// "temperature_target", "humidity", "motion", "contact", "locked",
    /// "position", "power_w".
    /// Valeurs contextuelles (device_id = None) : "hour", "weekday".
    pub property: String,

    /// Operateur de comparaison.
    pub op: ConditionOp,

    /// Valeur de comparaison (JSON : bool, number, array pour Between).
    pub value: serde_json::Value,
}
```

### 4.3 `Action`

```rust
/// Action a executer dans une automatisation.
///
/// Les actions d'une automatisation sont executees sequentiellement,
/// dans l'ordre de la liste.
///
/// # Delai
///
/// `delay_ms = 0` (ou None) : execution immediate.
/// `delay_ms = 2000` : attente 2 secondes avant d'envoyer la commande.
/// Le delai est non-bloquant (tokio::time::sleep).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// UUID du dispositif cible. Doit exister dans le DeviceRegistry.
    pub device_id: Uuid,

    /// Action a realiser. Memes valeurs que `DeviceCommand::action`.
    /// Exemples : "on", "off", "set_brightness", "set_temperature", "lock".
    pub command: String,

    /// Valeur optionnelle pour la commande.
    /// Exemples : `75` pour set_brightness, `18.0` pour set_temperature.
    pub value: Option<serde_json::Value>,

    /// Delai en millisecondes avant l'execution de cette action.
    /// Defaut : 0 (execution immediate).
    #[serde(default)]
    pub delay_ms: u64,
}
```

### 4.4 `Automation`

```rust
use chrono::{DateTime, Utc};

/// Automatisation complete : declencheur + conditions + actions.
///
/// # Invariants
///
/// - `id` est un UUID v4, genere a la creation, immutable.
/// - `actions` doit contenir au moins 1 element (valide a la creation).
/// - `trigger` est unique et non nul.
/// - `conditions` peut etre vide (l'automatisation s'execute toujours si declenchee).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Automation {
    pub id:                Uuid,
    pub name:              String,
    pub enabled:           bool,
    pub trigger:           TriggerType,
    /// Liste de conditions (ET logique). Vide = toujours vrai.
    pub conditions:        Vec<Condition>,
    /// Liste d'actions sequentielles. Minimum 1 element.
    pub actions:           Vec<Action>,
    pub last_triggered_at: Option<DateTime<Utc>>,
    pub created_at:        DateTime<Utc>,
    pub updated_at:        DateTime<Utc>,
}

impl Automation {
    /// Valide l'automatisation. Retourne les erreurs de validation.
    ///
    /// Verifie :
    /// - `name` non vide
    /// - `actions` non vide
    /// - Pour les triggers `Cron` : expression valide via le parser cron
    /// - Pour les triggers `SensorChange` : operateur compatible avec la propriete
    pub fn validate(&self) -> Result<(), AutomationError>;

    /// Retourne `true` si l'automatisation peut etre declenchee par un VoiceCommand.
    pub fn matches_routine(&self, routine_name: &str) -> bool;
}
```

---

## 5. `src/engine.rs` — `AutomationEngine`

### 5.1 Trait d'injection de dependances

```rust
use miyualicia_devices::{DeviceCommand, DeviceState};
use uuid::Uuid;

/// Interface de dispatch des commandes vers le service Alicia.
///
/// Ce trait permet de tester `AutomationEngine` sans depend de `miyualicia`
/// (evite la dependance circulaire). `miyualicia::AliciaService` implemente ce trait.
#[async_trait::async_trait]
pub trait AliciaCommandDispatcher: Send + Sync {
    /// Envoie une commande vers un dispositif.
    async fn dispatch_command(&self, command: DeviceCommand) -> Result<(), AutomationError>;

    /// Retourne l'etat courant d'un dispositif.
    async fn get_device_state(&self, device_id: Uuid) -> Result<DeviceState, AutomationError>;
}
```

### 5.2 Snapshot pour evaluation des conditions

```rust
/// Snapshot de l'etat de la maison pour l'evaluateur de conditions.
///
/// Cree par `AliciaService` au moment de l'evaluation d'une automatisation.
/// Contient les etats de tous les dispositifs et le contexte temporel.
#[derive(Debug, Clone)]
pub struct HomeSnapshot {
    /// Etats de tous les dispositifs, indexes par UUID.
    pub device_states: std::collections::HashMap<Uuid, DeviceState>,
    /// Heure courante (pour conditions "hour", "weekday").
    pub now:           chrono::DateTime<chrono::Local>,
}
```

### 5.3 `AutomationEngine`

```rust
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex, RwLock};
use uuid::Uuid;

/// Evenement emis vers le moteur pour declenchement externe.
#[derive(Debug, Clone)]
pub enum AutomationEvent {
    /// Evenement capteur : l'etat d'un dispositif a change.
    DeviceStateChanged { device_id: Uuid, state: DeviceState },
    /// Commande vocale : routine a declencer.
    VoiceRoutine { routine_name: String },
    /// Declenchement API direct.
    ApiTrigger { automation_id: Uuid },
}

/// Moteur d'automatisations Alicia.
///
/// # Architecture interne
///
/// Le moteur maintient :
/// 1. Une liste d'automatisations en memoire (`Arc<RwLock<Vec<Automation>>>`)
/// 2. Un scheduler `tokio-cron-scheduler` pour les triggers `Cron`
/// 3. Un canal `broadcast` pour recevoir les evenements du service Alicia
///
/// # Cycle de vie
///
/// ```
/// let engine = AutomationEngine::new(dispatcher, automations_from_db);
/// engine.start().await?;  // lance le scheduler et le listener d'evenements
/// // ... execution normale ...
/// engine.stop().await;
/// ```
///
/// # Thread-safety
///
/// `AutomationEngine` est Clone (Arc interne). Partager l'engine entre
/// le service Alicia et les handlers API est safe.
#[derive(Clone)]
pub struct AutomationEngine {
    inner: Arc<AutomationEngineInner>,
}

struct AutomationEngineInner {
    dispatcher:    Arc<dyn AliciaCommandDispatcher>,
    automations:   RwLock<Vec<Automation>>,
    event_tx:      broadcast::Sender<AutomationEvent>,
    running:       Mutex<bool>,
}

impl AutomationEngine {
    /// Cree le moteur avec le dispatcher et les automatisations initiales (chargees depuis DB).
    pub fn new(
        dispatcher: Arc<dyn AliciaCommandDispatcher>,
        initial_automations: Vec<Automation>,
    ) -> Self;

    /// Demarre le scheduler et le listener d'evenements.
    ///
    /// Non-bloquant : lance des `tokio::spawn` en interne.
    /// Appeler `stop()` pour arreter proprement.
    pub async fn start(&self) -> Result<(), AutomationError>;

    /// Arrete le scheduler et le listener d'evenements.
    pub async fn stop(&self);

    /// Ajoute une automatisation au registre et au scheduler si Cron.
    ///
    /// # Validation
    ///
    /// Appelle `automation.validate()` avant ajout.
    /// Retourne `AutomationError::ValidationError` si invalide.
    pub async fn add_automation(&self, automation: Automation) -> Result<(), AutomationError>;

    /// Supprime une automatisation par UUID.
    ///
    /// Desactive le job cron correspondant si applicable.
    pub async fn remove_automation(&self, id: Uuid) -> Result<(), AutomationError>;

    /// Active ou desactive une automatisation.
    pub async fn set_enabled(&self, id: Uuid, enabled: bool) -> Result<(), AutomationError>;

    /// Declenche manuellement une automatisation (ignore conditions).
    ///
    /// Utilisee par `POST /automations/{id}/trigger` et les tests.
    pub async fn trigger_automation(&self, id: Uuid) -> Result<(), AutomationError>;

    /// Retourne la liste de toutes les automatisations.
    pub async fn list_automations(&self) -> Vec<Automation>;

    /// Retourne le canal emetteur pour envoyer des evenements au moteur.
    ///
    /// Utilise par `AliciaService` pour signaler les changements d'etat.
    pub fn event_sender(&self) -> broadcast::Sender<AutomationEvent>;

    /// Evalue et execute toutes les automatisations correspondant a un evenement capteur.
    ///
    /// # Algorithme
    ///
    /// 1. Filtrer les automatisations activees avec trigger `SensorChange` pour le device_id
    /// 2. Construire un `HomeSnapshot` depuis le dispatcher
    /// 3. Pour chaque candidat : evaluer les conditions avec `evaluate_conditions`
    /// 4. Si toutes les conditions sont vraies : executer les actions
    /// 5. Mettre a jour `last_triggered_at` dans la liste
    async fn handle_device_state_changed(
        &self,
        device_id: Uuid,
        state: DeviceState,
    ) -> Result<(), AutomationError>;

    /// Evalue et execute toutes les automatisations correspondant a une routine vocale.
    async fn handle_voice_routine(&self, routine_name: &str) -> Result<(), AutomationError>;
}
```

---

## 6. `src/evaluator.rs` — `evaluate_conditions`

```rust
use crate::types::{Condition, ConditionOp};
use crate::engine::HomeSnapshot;
use crate::errors::AutomationError;

/// Evalue une liste de conditions contre le snapshot de la maison.
///
/// # Semantique ET logique
///
/// Toutes les conditions doivent etre vraies pour que la fonction retourne `true`.
/// Une liste vide retourne `true` (pas de conditions = toujours vrai).
///
/// # Proprietes de contexte (device_id = None)
///
/// - `"hour"` : heure locale courante (0-23), comparee comme nombre
/// - `"weekday"` : jour de la semaine (0=lundi, 6=dimanche), compare comme nombre
///
/// # Proprietes de DeviceState (device_id != None)
///
/// La propriete est extraite par reflexion depuis `DeviceState`.
/// Si la propriete est `None` (etat inconnu), la condition est evaluee `false`
/// (principe de securite : etat inconnu = condition non remplie).
///
/// # Erreurs
///
/// Retourne `AutomationError::EvaluationError` si :
/// - La propriete demandee n'existe pas dans `DeviceState`
/// - Les types sont incompatibles (comparer un bool avec un nombre)
///
/// # Tests garantis
///
/// - Condition vide → true
/// - `temperature_current >= 25` avec temperature = 26 → true
/// - `temperature_current >= 25` avec temperature = 24 → false
/// - `motion == true` avec etat inconnu (None) → false
/// - `hour Between [22, 23]` a 22h00 → true
pub fn evaluate_conditions(
    conditions: &[Condition],
    snapshot: &HomeSnapshot,
) -> Result<bool, AutomationError>;

/// Evalue une seule condition.
fn evaluate_single(
    condition: &Condition,
    snapshot: &HomeSnapshot,
) -> Result<bool, AutomationError>;

/// Compare deux valeurs JSON avec l'operateur donne.
///
/// Supports : bool==bool, number op number, number Between [min, max].
/// Retourne `false` si les types sont incompatibles (pas d'erreur silencieuse :
/// un warning tracing est emis).
fn compare_json(
    actual: &serde_json::Value,
    op: ConditionOp,
    expected: &serde_json::Value,
) -> bool;
```

---

## 7. `src/executor.rs` — Execution des actions

```rust
use crate::engine::AliciaCommandDispatcher;
use crate::errors::AutomationError;
use crate::types::Action;
use miyualicia_devices::{CommandSource, DeviceCommand};
use std::sync::Arc;
use uuid::Uuid;

/// Execute une sequence d'actions d'une automatisation.
///
/// # Sequentialite
///
/// Les actions sont executees l'une apres l'autre, dans l'ordre.
/// Si une action echoue, l'execution continue avec les suivantes
/// (pas d'arret sur erreur partielle). Les erreurs sont collectees et loggees.
///
/// # Delais
///
/// `tokio::time::sleep(Duration::from_millis(action.delay_ms))` avant chaque action.
/// Un `delay_ms = 0` ne cree pas de `sleep` (optimisation).
///
/// # Source des commandes
///
/// Toutes les commandes issues du moteur d'automatisations portent
/// `CommandSource::Automation` pour l'audit trail.
///
/// # Retour
///
/// Retourne `Ok(())` si toutes les actions ont ete dispatched (meme si certaines
/// ont echoue au niveau du protocole). Retourne `Err` uniquement si le dispatcher
/// est inaccessible.
pub async fn execute_actions(
    actions: &[Action],
    dispatcher: Arc<dyn AliciaCommandDispatcher>,
    automation_id: Uuid,
) -> Result<(), AutomationError>;
```

---

## 8. `src/parser.rs` — Parser TOML

### 8.1 Format TOML des automatisations

```toml
# Exemple : alicia.toml ou fichier dedie
[[automations]]
name = "Bonne nuit"
enabled = true

[automations.trigger]
type = "voice_command"
routine_name = "bonne nuit"

# Pas de conditions (toujours execute)
conditions = []

[[automations.actions]]
device_id = "uuid-lumiere-salon"
command = "off"
delay_ms = 0

[[automations.actions]]
device_id = "uuid-thermostat-chambre"
command = "set_temperature"
value = 18.0
delay_ms = 500

[[automations.actions]]
device_id = "uuid-serrure-entree"
command = "lock"
delay_ms = 1000

---

# Exemple : declencheur cron avec conditions
[[automations]]
name = "Extinction automatique salon"
enabled = true

[automations.trigger]
type = "cron"
expression = "0 0 23 * * *"

[[automations.conditions]]
device_id = "uuid-capteur-salon"
property = "motion"
op = "eq"
value = false

[[automations.actions]]
device_id = "uuid-lumiere-salon"
command = "off"
delay_ms = 0
```

### 8.2 API du parseur

```rust
use crate::types::Automation;
use crate::errors::AutomationError;

/// Structure intermediaire pour la deserialisation TOML.
#[derive(Debug, serde::Deserialize)]
struct AutomationToml {
    automations: Vec<AutomationRaw>,
}

/// Parse un fichier TOML complet et retourne la liste des automatisations.
///
/// # Validation
///
/// Chaque automatisation parsee est validee via `Automation::validate()`.
/// Les automatisations invalides sont collectees dans `AutomationError::ValidationErrors`
/// (pluriel) pour que l'utilisateur voit toutes les erreurs d'un coup.
///
/// # Erreurs
///
/// - `AutomationError::ParseError` : TOML malformed
/// - `AutomationError::ValidationErrors` : une ou plusieurs automatisations invalides
///
/// # Exemple
///
/// ```rust
/// let toml_content = std::fs::read_to_string("alicia-automations.toml")?;
/// let automations = parse_automation_toml(&toml_content)?;
/// for auto in automations {
///     engine.add_automation(auto).await?;
/// }
/// ```
pub fn parse_automation_toml(content: &str) -> Result<Vec<Automation>, AutomationError>;

/// Parse une seule automatisation depuis un objet JSON (pour l'API REST).
///
/// Utilise pour la creation via `POST /automations`.
pub fn parse_automation_json(value: &serde_json::Value) -> Result<Automation, AutomationError>;
```

---

## 9. `src/errors.rs` — `AutomationError`

```rust
use uuid::Uuid;

/// Erreurs du moteur d'automatisations.
#[derive(Debug, thiserror::Error)]
pub enum AutomationError {
    /// Automatisation non trouvee par UUID.
    #[error("automatisation {0} introuvable")]
    NotFound(Uuid),

    /// Erreur de validation d'une automatisation.
    #[error("validation automatisation '{name}' echouee : {reason}")]
    ValidationError {
        name:   String,
        reason: String,
    },

    /// Plusieurs erreurs de validation (lors d'un import TOML).
    #[error("erreurs de validation : {0}")]
    ValidationErrors(String),

    /// Erreur de parsing TOML.
    #[error("erreur parsing TOML automatisations : {0}")]
    ParseError(#[from] toml::de::Error),

    /// Expression cron invalide.
    #[error("expression cron invalide '{expression}' : {reason}")]
    InvalidCronExpression {
        expression: String,
        reason:     String,
    },

    /// Erreur du scheduler tokio-cron-scheduler.
    #[error("erreur scheduler : {0}")]
    SchedulerError(String),

    /// Erreur d'evaluation d'une condition.
    #[error("erreur evaluation condition (propriete '{property}') : {reason}")]
    EvaluationError {
        property: String,
        reason:   String,
    },

    /// Erreur de dispatch de commande vers le service Alicia.
    #[error("erreur dispatch commande automatisation : {0}")]
    DispatchError(String),

    /// Erreur de serialisation JSON.
    #[error("erreur serialisation automatisation : {0}")]
    SerializationError(#[from] serde_json::Error),
}
```

---

## 10. Tests attendus

### 10.1 `evaluator.rs`

```rust
// TC-AUTO-01 : conditions vides → true
#[test]
fn test_evaluate_empty_conditions_is_true() {
    let snapshot = HomeSnapshot { device_states: Default::default(), now: chrono::Local::now() };
    let result = evaluate_conditions(&[], &snapshot).unwrap();
    assert!(result);
}

// TC-AUTO-02 : temperature >= 25 avec valeur 26 → true
#[test]
fn test_evaluate_temperature_above_threshold() { ... }

// TC-AUTO-03 : temperature >= 25 avec valeur 24 → false
#[test]
fn test_evaluate_temperature_below_threshold() { ... }

// TC-AUTO-04 : motion == true avec etat inconnu (None) → false
#[test]
fn test_evaluate_unknown_state_is_false() { ... }

// TC-AUTO-05 : ET logique : toutes les conditions vraies → true
#[test]
fn test_evaluate_all_conditions_true() { ... }

// TC-AUTO-06 : ET logique : une condition fausse → false
#[test]
fn test_evaluate_one_condition_false() { ... }

// TC-AUTO-07 : condition hour Between [22, 23] a 22h30 → true
#[test]
fn test_evaluate_hour_between() { ... }

// TC-AUTO-08 : condition weekday == 0 (lundi) le mardi → false
#[test]
fn test_evaluate_weekday_mismatch() { ... }
```

### 10.2 `parser.rs`

```rust
// TC-AUTO-09 : parse TOML valide avec trigger voice_command
#[test]
fn test_parse_toml_voice_command() { ... }

// TC-AUTO-10 : parse TOML valide avec trigger cron
#[test]
fn test_parse_toml_cron() { ... }

// TC-AUTO-11 : TOML invalide (champ manquant) retourne ParseError
#[test]
fn test_parse_toml_missing_field() { ... }

// TC-AUTO-12 : actions vides retourne ValidationError
#[test]
fn test_validate_empty_actions_fails() { ... }

// TC-AUTO-13 : expression cron invalide retourne InvalidCronExpression
#[test]
fn test_invalid_cron_expression() { ... }
```

### 10.3 `engine.rs`

```rust
// TC-AUTO-14 : add_automation + list_automations
#[tokio::test]
async fn test_add_and_list_automations() { ... }

// TC-AUTO-15 : trigger_automation declenche les actions (mock dispatcher)
#[tokio::test]
async fn test_trigger_automation_dispatches_actions() { ... }

// TC-AUTO-16 : automatisation disabled n'est pas declenchee par VoiceRoutine
#[tokio::test]
async fn test_disabled_automation_not_triggered() { ... }

// TC-AUTO-17 : handle_voice_routine trouve la bonne automatisation
#[tokio::test]
async fn test_voice_routine_matching() { ... }

// TC-AUTO-18 : execute_actions respecte les delais
#[tokio::test]
async fn test_execute_actions_with_delay() { ... }
```

---

## 11. Automatisations predefinies (exemples)

Fichier : `apps/central/automations/examples/bonne-nuit.toml`

```toml
[[automations]]
name = "Bonne nuit"
enabled = false  # desactive par defaut, activer dans l'UI

[automations.trigger]
type = "voice_command"
routine_name = "bonne nuit"

conditions = []

[[automations.actions]]
device_id = "REMPLACER-PAR-UUID-LUMIERE-SALON"
command = "off"
delay_ms = 0

[[automations.actions]]
device_id = "REMPLACER-PAR-UUID-THERMOSTAT-CHAMBRE"
command = "set_temperature"
value = 18.0
delay_ms = 500

[[automations.actions]]
device_id = "REMPLACER-PAR-UUID-SERRURE-ENTREE"
command = "lock"
delay_ms = 1000
```

Fichier : `apps/central/automations/examples/extinction-nuit.toml`

```toml
[[automations]]
name = "Extinction automatique 23h"
enabled = false

[automations.trigger]
type = "cron"
expression = "0 0 23 * * *"

[[automations.conditions]]
device_id = "REMPLACER-PAR-UUID-CAPTEUR-SALON"
property = "motion"
op = "eq"
value = false

[[automations.actions]]
device_id = "REMPLACER-PAR-UUID-LUMIERE-SALON"
command = "off"
delay_ms = 0
```

---

## 12. Annotations MSCM — recap

| Fichier         | @id                                | @layer | @role                    |
|-----------------|------------------------------------|--------|--------------------------|
| `lib.rs`        | `toolkit.alicia.automations`       | 6      | `automation_engine`      |
| `admin_cell.rs` | `toolkit.alicia.automations.admin` | 6      | `governance_cell`        |
| `types.rs`      | (inline)                           | 6      | `automation_domain_types`|
| `engine.rs`     | (inline)                           | 6      | `automation_scheduler`   |
| `evaluator.rs`  | (inline)                           | 6      | `condition_evaluator`    |
| `executor.rs`   | (inline)                           | 6      | `action_executor`        |
| `parser.rs`     | (inline)                           | 6      | `toml_parser`            |
| `errors.rs`     | (inline)                           | 6      | `error_types`            |

---

## 13. Securite et conformite

- **Pas d'execution arbitraire** : les actions ne peuvent executor que les commandes connues de
  `DeviceCommand`. Pas de shell, pas de code dynamique, pas d'evaluation de script.
- **Isolation TOML** : le parsing TOML utilise des types Rust structures (pas d'eval).
  Les expressions cron sont validees avant insertion dans le scheduler.
- **Delais bornes** : `delay_ms` est limite a 60 000 ms (1 minute) par la validation.
  Un delai superieur est rejete avec `ValidationError`.
- **Conditions sur etat inconnu = false** : pas d'action sur un dispositif dont l'etat n'est pas connu.
  Principe de securite : l'inaction est preferable a une action non souhaitee.
- **Audit trail** : toutes les executions d'actions sont loggees dans `alicia_commands_log`
  avec `source = CommandSource::Automation` et `source_detail = automation_id`.

---

*Denis — Chef Dev Senior — Miyukini AI Studio — 2026-03-01*
