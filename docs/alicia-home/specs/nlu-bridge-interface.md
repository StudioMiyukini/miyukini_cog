# Spec Technique — Interface NLU avec `miou-llm-bridge`

<!-- @id: spec.alicia.nlu-bridge -->
<!-- @role: technical-specification -->
<!-- @layer: 7 -->
<!-- @human: Specification de l'interface NLU entre miyualicia et miou-llm-bridge -->
<!-- @do: define_nlu_bridge_interface_for_alicia -->

**Auteur :** Denis, Chef Dev Senior — Miyukini AI Studio
**Date :** 2026-03-01
**Version :** 1.0
**Reference :** Rapport Fondateur Alicia Home Assistante v1.0 §3.1, Plan Dev General D-3-1/D-3-2

---

## Contexte

Apres la detection du wake word "Hey Alicia" (crate `miyualicia-wakeword`, ex-`miyuwakeword`),
le pipeline NLU doit :
1. Transcrire l'audio en texte (STT — Speech to Text)
2. Extraire une intention structuree du texte (NLU — Natural Language Understanding)
3. Mapper cette intention vers une commande domotique (`DeviceCommand`) ou une routine

Ces deux etapes sont deleguees a `miou-llm-bridge`, le proxy LLM local de l'ecosysteme COG
(17 agents, skills, contextes). Le bridge tourne sur `http://127.0.0.1:3003` par defaut.

## Portee / Scope

Ce document couvre :
- L'interface HTTP entre `miyualicia` (crate `nlu_bridge.rs`) et `miou-llm-bridge`
- Le type `Intent` et la taxonomie complete des intentions domotiques
- Le fallback regex parser (mode degrade si le bridge est indisponible)
- Les tests attendus du module NLU

Ce document ne couvre pas :
- L'implementation interne de `miou-llm-bridge` (crate distinct)
- Le wake word (crate `miyualicia-wakeword`)
- La capture audio (crate `miyualicia-capture`)

---

## 1. Type `Intent` — Taxonomie complete

### 1.1 Definition Rust

```rust
// Fichier : crates/miyualicia/src/intent.rs
//
// @id: service.alicia.intent
// @role: nlu_intent_taxonomy
// @layer: 7
// @human: Type Intent et taxonomie des intentions domotiques Alicia
// @do: model_home_assistant_intents

use serde::{Deserialize, Serialize};

/// Intention reconnue par le NLU, produite a partir de la transcription vocale.
///
/// # Taxonomie
///
/// Les intentions sont organisees en trois categories :
/// - Controle actif   : agir sur un dispositif
/// - Requete          : interroger l'etat
/// - Meta             : routines, aide, systeme
/// - Inconnu          : fallback quand aucune intention n'est reconnue
///
/// # Champs optionnels
///
/// Quand un champ est `None`, cela signifie que le NLU n'a pas pu l'extraire
/// de la transcription. L'orchestrateur peut alors :
/// - Utiliser un dispositif par defaut pour la piece courante
/// - Demander une clarification a l'utilisateur (Phase 2)
/// - Ignorer la commande et loguer une intention ambigue
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "intent", rename_all = "snake_case")]
pub enum Intent {
    /// Controle d'un dispositif : allumer, eteindre, regler.
    ///
    /// # Exemples vocaux
    ///
    /// - "Alicia, allume la lumiere du salon" → device_type="light", room_id="salon", action="on"
    /// - "Eteins la lampe" → device_type="light", action="off"
    /// - "Mets le chauffage a 20 degres" → device_type="thermostat", action="set_temperature", value=20.0
    /// - "Baisse la lumiere a 40 pourcent" → device_type="light", action="set_brightness", value=40
    /// - "Ferme les volets" → device_type="shutter", action="close"
    /// - "Coupe la prise de la cuisine" → device_type="outlet", action="off"
    /// - "Verrouille la porte" → device_type="lock", action="lock"
    ControlDevice {
        /// Type de dispositif cible. Exemples : "light", "thermostat", "shutter", "outlet", "lock".
        device_type: String,
        /// Identifiant de piece. None si non mentionne.
        /// Valeurs connues : "salon", "chambre-parentale", "chambre-theresa", "chambre-eleanore".
        room_id:     Option<String>,
        /// Action a realiser. Compatible avec `DeviceCommand::action`.
        /// Valeurs : "on", "off", "set_brightness", "set_color", "set_temperature",
        ///           "open", "close", "set_position", "lock", "unlock".
        action:      String,
        /// Valeur numerique ou structuree associee a l'action.
        /// Exemples : 20.0 (degres), 75 (pourcentage), [255,128,0] (RGB).
        value:       Option<serde_json::Value>,
    },

    /// Requete d'etat : interroger la maison ou un dispositif.
    ///
    /// # Exemples vocaux
    ///
    /// - "Quelle est la temperature du salon ?" → target="salon", property="temperature"
    /// - "Est-ce que la lumiere est allumee ?" → target="light", property="on"
    /// - "C'est quoi la consommation de la prise ?" → target="outlet", property="power"
    /// - "Quel est l'etat de la maison ?" → target="home", property="summary"
    QueryState {
        /// Cible de la requete. Peut etre :
        /// - "home" : etat global de la maison
        /// - un room_id : "salon", "chambre-parentale", etc.
        /// - un type de dispositif : "light", "thermostat", etc.
        target:   String,
        /// Propriete interrogee. Exemples : "temperature", "on", "power", "locked", "summary".
        property: Option<String>,
    },

    /// Activation d'une routine nommee.
    ///
    /// # Exemples vocaux
    ///
    /// - "Alicia, bonne nuit" → routine_name="bonne nuit"
    /// - "Alicia, je pars" → routine_name="je pars"
    /// - "Mode cinema" → routine_name="cinema"
    /// - "Lance la routine matin" → routine_name="matin"
    ActivateRoutine {
        /// Nom de la routine, tel qu'il appara dans `AutomationEngine`.
        /// Le matching est case-insensitive, les accents sont normalises.
        routine_name: String,
    },

    /// Requete meteo locale.
    ///
    /// # Exemples vocaux
    ///
    /// - "Quel temps fait-il ?" → location=None (locale configuree)
    /// - "Meteo de demain ?" → horizon="tomorrow"
    QueryWeather {
        /// Lieu. None = lieu configure dans alicia.toml.
        location: Option<String>,
        /// Horizon temporel : "now", "today", "tomorrow". Defaut : "now".
        horizon:  Option<String>,
    },

    /// Requete d'aide ou information systeme.
    ///
    /// # Exemples vocaux
    ///
    /// - "Qu'est-ce que tu sais faire ?"
    /// - "Aide moi"
    Help {
        /// Sujet de l'aide, si mentionne. Exemples : "lumiere", "thermostat", "routines".
        topic: Option<String>,
    },

    /// Intention non reconnue. Fallback final.
    ///
    /// # Usage
    ///
    /// L'orchestrateur logue l'intention `Unknown` et peut :
    /// - Repondre vocalement "Je n'ai pas compris"
    /// - Conserver la transcription pour un apprentissage futur (hors scope Phase 1)
    Unknown {
        /// La transcription brute qui n'a pas pu etre interpretee.
        transcript: String,
    },
}

impl Intent {
    /// Retourne `true` si l'intention peut generer une commande domotique directe.
    pub fn is_actionable(&self) -> bool {
        matches!(self, Self::ControlDevice { .. } | Self::ActivateRoutine { .. })
    }

    /// Retourne le nom lisible de l'intention pour les logs.
    pub fn name(&self) -> &'static str {
        match self {
            Self::ControlDevice { .. }  => "ControlDevice",
            Self::QueryState { .. }     => "QueryState",
            Self::ActivateRoutine { .. }=> "ActivateRoutine",
            Self::QueryWeather { .. }   => "QueryWeather",
            Self::Help { .. }           => "Help",
            Self::Unknown { .. }        => "Unknown",
        }
    }
}
```

---

## 2. Interface HTTP vers `miou-llm-bridge`

### 2.1 Prerequis

`miou-llm-bridge` doit etre demarre avant `miyualicia`. Si le bridge est indisponible,
`NluBridge::parse_intent()` active automatiquement le fallback regex.

URL base configurable dans `alicia.toml` : `llm_bridge_url = "http://127.0.0.1:3003"`

### 2.2 Requete STT (transcription audio → texte)

```http
POST {llm_bridge_url}/api/stt
Content-Type: application/json
X-Request-ID: <uuid-v4>
X-Source: alicia-home

{
  "samples": [0.0023, -0.0045, ...],  // Vec<f32>, f32 mono 16kHz
  "sample_rate": 16000,
  "model": "whisper-local",           // toujours "whisper-local" pour Alicia
  "language": "fr"                    // toujours "fr" Phase 1
}
```

**Reponse 200 :**

```json
{
  "transcript": "allume la lumiere du salon",
  "confidence": 0.94,
  "duration_ms": 1240,
  "model_used": "whisper-base"
}
```

**Reponse 503 (bridge indisponible)** → activer le fallback regex.

**Timeout** : 5 secondes maximum. Au-dela → `NluError::BridgeTimeout`, activer le fallback.

### 2.3 Requete NLU (texte → intent)

```http
POST {llm_bridge_url}/api/nlu
Content-Type: application/json
X-Request-ID: <uuid-v4>
X-Source: alicia-home

{
  "text": "allume la lumiere du salon",
  "context": "home_automation",          // contexte fixe pour Alicia
  "rooms": ["salon", "chambre-parentale", "chambre-theresa", "chambre-eleanore"],
  "device_types": ["light", "shutter", "thermostat", "outlet", "sensor", "lock"],
  "known_routines": ["bonne nuit", "je pars", "cinema", "matin"]
}
```

Le champ `context = "home_automation"` permet au bridge de charger le prompt systeme
domotique adapte. Les champs `rooms`, `device_types`, `known_routines` fournissent
le contexte pour l'extraction precise des slots.

**Reponse 200 :**

```json
{
  "intent": "control_device",
  "confidence": 0.97,
  "slots": {
    "device_type": "light",
    "room_id": "salon",
    "action": "on",
    "value": null
  }
}
```

```json
{
  "intent": "activate_routine",
  "confidence": 0.99,
  "slots": {
    "routine_name": "bonne nuit"
  }
}
```

```json
{
  "intent": "unknown",
  "confidence": 0.0,
  "slots": {}
}
```

**Reponse 503** → activer le fallback regex.

### 2.4 Conversion reponse NLU → `Intent`

```rust
// Dans crates/miyualicia/src/nlu_bridge.rs

fn parse_bridge_response(response: &BridgeNluResponse) -> Intent {
    match response.intent.as_str() {
        "control_device" => Intent::ControlDevice {
            device_type: response.slots.get("device_type")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string(),
            room_id: response.slots.get("room_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            action: response.slots.get("action")
                .and_then(|v| v.as_str())
                .unwrap_or("on")
                .to_string(),
            value: response.slots.get("value").cloned(),
        },
        "activate_routine" => Intent::ActivateRoutine {
            routine_name: response.slots.get("routine_name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        },
        "query_state" => Intent::QueryState {
            target: response.slots.get("target")
                .and_then(|v| v.as_str())
                .unwrap_or("home")
                .to_string(),
            property: response.slots.get("property")
                .and_then(|v| v.as_str())
                .map(String::from),
        },
        "query_weather" => Intent::QueryWeather {
            location: response.slots.get("location")
                .and_then(|v| v.as_str())
                .map(String::from),
            horizon: response.slots.get("horizon")
                .and_then(|v| v.as_str())
                .map(String::from),
        },
        "help" => Intent::Help {
            topic: response.slots.get("topic")
                .and_then(|v| v.as_str())
                .map(String::from),
        },
        _ => Intent::Unknown {
            transcript: response.slots.get("transcript")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        },
    }
}
```

---

## 3. `NluBridge` — Implementation

```rust
// Fichier : crates/miyualicia/src/nlu_bridge.rs
//
// @id: service.alicia.nlu-bridge
// @role: nlu_client
// @layer: 7
// @human: Client HTTP vers miou-llm-bridge pour STT et NLU, avec fallback regex.
// @do: transcribe_and_parse_voice_intent

use crate::errors::AliciaError;
use crate::intent::Intent;
use crate::nlu_fallback::FallbackNluParser;

/// Erreurs specifiques au bridge NLU.
#[derive(Debug, thiserror::Error)]
pub enum NluError {
    #[error("bridge NLU indisponible (timeout ou connexion refusee)")]
    BridgeUnavailable,

    #[error("timeout requete NLU apres {timeout_ms}ms")]
    BridgeTimeout { timeout_ms: u64 },

    #[error("reponse bridge NLU invalide : {0}")]
    InvalidResponse(String),

    #[error("erreur HTTP bridge NLU : {0}")]
    HttpError(String),
}

/// Structure de configuration du bridge.
#[derive(Debug, Clone)]
pub struct NluBridgeConfig {
    /// URL de base du bridge. Defaut : "http://127.0.0.1:3003"
    pub base_url:    String,
    /// Timeout STT en millisecondes. Defaut : 5000.
    pub stt_timeout_ms: u64,
    /// Timeout NLU en millisecondes. Defaut : 3000.
    pub nlu_timeout_ms: u64,
    /// Pieces configurees (pour le contexte NLU).
    pub known_rooms:    Vec<String>,
    /// Routines connues (pour le contexte NLU).
    pub known_routines: Vec<String>,
}

impl Default for NluBridgeConfig {
    fn default() -> Self {
        Self {
            base_url:       "http://127.0.0.1:3003".to_string(),
            stt_timeout_ms: 5000,
            nlu_timeout_ms: 3000,
            known_rooms:    vec![
                "salon".to_string(),
                "chambre-parentale".to_string(),
                "chambre-theresa".to_string(),
                "chambre-eleanore".to_string(),
            ],
            known_routines: vec![
                "bonne nuit".to_string(),
                "je pars".to_string(),
            ],
        }
    }
}

/// Client NLU : wrappant les appels HTTP vers miou-llm-bridge.
///
/// # Strategie de fallback
///
/// Si le bridge est indisponible (timeout, connexion refusee, code >= 500),
/// le client active automatiquement `FallbackNluParser` (regex).
/// Le mode de fonctionnement est logue a chaque changement.
///
/// # Usage
///
/// ```rust
/// let bridge = NluBridge::new(config, http_client);
/// let transcript = bridge.transcribe(audio_samples).await?;
/// let intent = bridge.parse_intent(&transcript).await?;
/// ```
#[derive(Debug, Clone)]
pub struct NluBridge {
    config:   NluBridgeConfig,
    client:   reqwest::Client,
    fallback: FallbackNluParser,
}

impl NluBridge {
    /// Cree un nouveau client NLU.
    ///
    /// Le `reqwest::Client` doit etre configure avec les timeouts adequats.
    pub fn new(config: NluBridgeConfig, client: reqwest::Client) -> Self;

    /// Transcrit des echantillons audio en texte via le bridge STT.
    ///
    /// # Comportement si bridge indisponible
    ///
    /// Retourne `NluError::BridgeUnavailable`. Dans ce cas, l'orchestrateur
    /// ne peut pas traiter la commande vocale (impossible de transcrire sans STT).
    /// Le fallback regex ne s'applique pas a la transcription audio.
    ///
    /// # Format audio
    ///
    /// - Echantillons f32 mono, 16kHz
    /// - Duree typique : 2 a 10 secondes post wake word
    pub async fn transcribe(
        &self,
        samples: &[f32],
    ) -> Result<String, NluError>;

    /// Parse une transcription en intention domotique.
    ///
    /// # Comportement si bridge indisponible
    ///
    /// Active le `FallbackNluParser` (regex). Retourne toujours un `Intent`
    /// (au pire `Intent::Unknown { transcript }`), jamais d'erreur dans ce cas.
    pub async fn parse_intent(&self, transcript: &str) -> Intent;

    /// Verifie si le bridge est joignable.
    ///
    /// Utilise pour l'indicateur de sante dans `GET /health`.
    pub async fn is_available(&self) -> bool;
}
```

---

## 4. Fallback — Parser regex (`nlu_fallback.rs`)

### 4.1 Objectif

Quand `miou-llm-bridge` est indisponible (demarrage, panne reseau, modele non charge),
le fallback regex permet a Alicia de continuer a fonctionner pour les commandes les plus
communes. Le mode degrade est logue avec `tracing::warn!`.

### 4.2 Implementation

```rust
// Fichier : crates/miyualicia/src/nlu_fallback.rs
//
// @id: service.alicia.nlu-fallback
// @role: regex_nlu_fallback
// @layer: 7
// @human: Parser regex de secours pour NLU sans miou-llm-bridge.
// @do: parse_basic_home_commands_via_regex

use crate::intent::Intent;
use std::sync::LazyLock;

/// Parser NLU de secours base sur des patterns regex.
///
/// # Couverture
///
/// Couvre les commandes P0 les plus courantes uniquement.
/// Pour les commandes ambigues ou non reconnues, retourne `Intent::Unknown`.
///
/// # Performance
///
/// Les patterns regex sont compiles une seule fois au demarrage
/// via `std::sync::LazyLock`. La compilation ne se produit pas au runtime.
#[derive(Debug, Clone, Default)]
pub struct FallbackNluParser;

impl FallbackNluParser {
    pub fn parse(&self, transcript: &str) -> Intent {
        let text = normalize(transcript);

        // Priorite 1 : routines nommees (avant le controle dispositifs)
        if let Some(intent) = try_parse_routine(&text) {
            return intent;
        }
        // Priorite 2 : controle lumiere
        if let Some(intent) = try_parse_light(&text) {
            return intent;
        }
        // Priorite 3 : controle thermostat
        if let Some(intent) = try_parse_thermostat(&text) {
            return intent;
        }
        // Priorite 4 : controle volets
        if let Some(intent) = try_parse_shutter(&text) {
            return intent;
        }
        // Priorite 5 : controle prises
        if let Some(intent) = try_parse_outlet(&text) {
            return intent;
        }
        // Priorite 6 : controle serrures
        if let Some(intent) = try_parse_lock(&text) {
            return intent;
        }
        // Fallback final
        Intent::Unknown { transcript: transcript.to_string() }
    }
}

/// Normalise le texte : minuscules, suppression de la ponctuation,
/// normalisation des accents (e → e, é → e, etc.) pour le matching regex.
fn normalize(text: &str) -> String { ... }

// --- Patterns ---

static PATTERNS_ROUTINE: LazyLock<Vec<(&str, &str)>> = LazyLock::new(|| {
    vec![
        (r"bonne\s*nuit", "bonne nuit"),
        (r"je\s*pars", "je pars"),
        (r"mode\s*cinema", "cinema"),
        (r"(routine\s+)?matin", "matin"),
    ]
});

static PATTERNS_LIGHT_ON: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)(allume|active|mets|ouvre|eclaire)\s+(la\s+)?(lumiere|lampe|lumieres|spot)(\s+du?\s+(?P<room>\w+))?"
    ).expect("pattern lumiere on valide")
});

static PATTERNS_LIGHT_OFF: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)(eteins?|coupe|desactive|ferme)\s+(la\s+)?(lumiere|lampe|lumieres)(\s+du?\s+(?P<room>\w+))?"
    ).expect("pattern lumiere off valide")
});

static PATTERNS_LIGHT_BRIGHTNESS: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)(baisse|monte|mets|regle)\s+(la\s+)?(lumiere|lampe)\s+(a|au|à)\s+(?P<level>\d+)\s*(pourcent|%|pour\s+cent)?"
    ).expect("pattern luminosite valide")
});

static PATTERNS_THERMOSTAT: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)(regle|mets|chauffe|monte|baisse)\s+(le\s+)?(thermostat|chauffage|temperature)\s+(a|au|à)\s+(?P<temp>\d+(\.\d+)?)\s*(degres|°|degre)?"
    ).expect("pattern thermostat valide")
});

static PATTERNS_SHUTTER_OPEN: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)(ouvre|leve|monte)\s+(les?\s+)?(volet|store|rideau|persienne)s?"
    ).expect("pattern volet open valide")
});

static PATTERNS_SHUTTER_CLOSE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)(ferme|baisse|descend)\s+(les?\s+)?(volet|store|rideau|persienne)s?"
    ).expect("pattern volet close valide")
});

static PATTERNS_OUTLET_ON: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)(allume|active|branche)\s+(la\s+)?(prise|multiprise)(\s+du?\s+(?P<room>\w+))?"
    ).expect("pattern prise on valide")
});

static PATTERNS_OUTLET_OFF: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)(eteins?|coupe|debranche|desactive)\s+(la\s+)?(prise|multiprise)(\s+du?\s+(?P<room>\w+))?"
    ).expect("pattern prise off valide")
});

static PATTERNS_LOCK: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)(?P<action>verrouille|deverrouille|ouvre|ferme)\s+(la\s+)?(porte|serrure)"
    ).expect("pattern serrure valide")
});

// --- Fonctions de parsing ---

fn try_parse_routine(text: &str) -> Option<Intent>;
fn try_parse_light(text: &str) -> Option<Intent>;
fn try_parse_thermostat(text: &str) -> Option<Intent>;
fn try_parse_shutter(text: &str) -> Option<Intent>;
fn try_parse_outlet(text: &str) -> Option<Intent>;
fn try_parse_lock(text: &str) -> Option<Intent>;

/// Extrait le room_id depuis un nom de piece mentionne en langage naturel.
///
/// Mapping :
/// - "salon" → "salon"
/// - "cuisine" → None (pas de cuisine configuree en Phase 1)
/// - "chambre" (ambigue) → None
/// - "chambre theresa", "chambre de theresa" → "chambre-theresa"
/// - "chambre parentale", "chambre des parents" → "chambre-parentale"
/// - "chambre eleanore", "chambre d'eleanore" → "chambre-eleanore"
fn extract_room_id(text: &str) -> Option<String>;
```

### 4.3 Tableau de mapping — exemples de phrases

| Phrase d'entree (normalisee)                      | Intent produite                                            |
|---------------------------------------------------|------------------------------------------------------------|
| "allume la lumiere du salon"                      | ControlDevice{type:light, room:salon, action:on}          |
| "eteins les lumieres"                             | ControlDevice{type:light, room:None, action:off}          |
| "baisse la lampe a 40 pourcent"                   | ControlDevice{type:light, action:set_brightness, value:40}|
| "regle le thermostat a 20 degres"                 | ControlDevice{type:thermostat, action:set_temperature, value:20.0}|
| "mets le chauffage a 18"                          | ControlDevice{type:thermostat, action:set_temperature, value:18.0}|
| "ouvre les volets"                                | ControlDevice{type:shutter, action:open}                  |
| "ferme le store"                                  | ControlDevice{type:shutter, action:close}                  |
| "eteins la prise"                                 | ControlDevice{type:outlet, action:off}                    |
| "verrouille la porte"                             | ControlDevice{type:lock, action:lock}                     |
| "deverrouille la serrure"                         | ControlDevice{type:lock, action:unlock}                   |
| "bonne nuit"                                      | ActivateRoutine{routine_name:"bonne nuit"}                |
| "je pars"                                         | ActivateRoutine{routine_name:"je pars"}                   |
| "ouvre la fenetre"                                | Unknown{transcript:"ouvre la fenetre"}                    |
| "commande une pizza"                              | Unknown{transcript:"commande une pizza"}                  |

---

## 5. Tests attendus

### 5.1 `nlu_bridge.rs`

```rust
// TC-NLU-01 : bridge disponible → transcription STT correcte (mock reqwest)
#[tokio::test]
async fn test_transcribe_bridge_available() { ... }

// TC-NLU-02 : bridge timeout → NluError::BridgeTimeout
#[tokio::test]
async fn test_transcribe_bridge_timeout() { ... }

// TC-NLU-03 : bridge disponible → parse_intent ControlDevice
#[tokio::test]
async fn test_parse_intent_control_device() { ... }

// TC-NLU-04 : bridge indisponible → fallback regex active, pas d'erreur
#[tokio::test]
async fn test_parse_intent_bridge_down_uses_fallback() { ... }

// TC-NLU-05 : is_available retourne false si bridge unreachable
#[tokio::test]
async fn test_bridge_availability_check() { ... }
```

### 5.2 `nlu_fallback.rs`

```rust
// TC-NLU-06 : "allume la lumiere du salon" → ControlDevice on
#[test]
fn test_fallback_light_on_with_room() {
    let parser = FallbackNluParser;
    let intent = parser.parse("allume la lumiere du salon");
    assert!(matches!(
        intent,
        Intent::ControlDevice { device_type, action, room_id: Some(room), .. }
        if device_type == "light" && action == "on" && room == "salon"
    ));
}

// TC-NLU-07 : "eteins les lumieres" → ControlDevice off sans room
#[test]
fn test_fallback_light_off_no_room() { ... }

// TC-NLU-08 : "baisse la lampe a 50 pourcent" → ControlDevice set_brightness 50
#[test]
fn test_fallback_light_brightness() { ... }

// TC-NLU-09 : "regle le thermostat a 20 degres" → ControlDevice set_temperature 20.0
#[test]
fn test_fallback_thermostat_temperature() { ... }

// TC-NLU-10 : "mets le chauffage a 18" (sans "degres") → set_temperature 18.0
#[test]
fn test_fallback_thermostat_no_unit() { ... }

// TC-NLU-11 : "ouvre les volets" → ControlDevice shutter open
#[test]
fn test_fallback_shutter_open() { ... }

// TC-NLU-12 : "bonne nuit" → ActivateRoutine
#[test]
fn test_fallback_routine_bonne_nuit() {
    let parser = FallbackNluParser;
    let intent = parser.parse("bonne nuit");
    assert!(matches!(
        intent,
        Intent::ActivateRoutine { routine_name }
        if routine_name == "bonne nuit"
    ));
}

// TC-NLU-13 : "commande une pizza" → Unknown
#[test]
fn test_fallback_unknown_intent() {
    let parser = FallbackNluParser;
    let intent = parser.parse("commande une pizza");
    assert!(matches!(intent, Intent::Unknown { .. }));
}

// TC-NLU-14 : Intent::is_actionable() correct
#[test]
fn test_intent_is_actionable() { ... }

// TC-NLU-15 : normalisation accents ("éteins" == "eteins")
#[test]
fn test_normalize_accents() { ... }

// TC-NLU-16 : extraction room_id "chambre de theresa" → "chambre-theresa"
#[test]
fn test_extract_room_id_chambre_theresa() { ... }
```

---

## 6. Dependances Cargo a ajouter dans `miyualicia`

```toml
# Dans crates/miyualicia/Cargo.toml, ajouter :
regex = "1"          # Pour le fallback NLU
reqwest = { version = "0.12", features = ["json"] }  # Deja present pour NluBridge
```

---

## 7. Annotations MSCM — recap

| Fichier               | @id                           | @layer | @role                   |
|-----------------------|-------------------------------|--------|-------------------------|
| `intent.rs`           | `service.alicia.intent`       | 7      | `nlu_intent_taxonomy`   |
| `nlu_bridge.rs`       | `service.alicia.nlu-bridge`   | 7      | `nlu_client`            |
| `nlu_fallback.rs`     | `service.alicia.nlu-fallback` | 7      | `regex_nlu_fallback`    |

---

## 8. Securite et conformite

- **Pas de donnees sensibles dans les requetes NLU** : les transcriptions vocales sont envoyees
  au bridge NLU **local uniquement** (`127.0.0.1`). Le bridge ne doit jamais les transmettre
  a un service cloud. Ajouter un header `X-Source: alicia-home` pour audit interne.
- **Timeout strict** : STT = 5s, NLU = 3s. Au-dela, le fallback s'active.
  L'utilisateur ne doit jamais attendre plus de 1.5s pour une reponse (critere BT mesurable).
- **Pas de retry infini** : 1 seul appel au bridge. Si echec → fallback immediat.
- **Transcriptions non persistees** : les transcriptions audio brutes ne sont jamais
  ecrites en base. Seule l'intention finale et son resultat sont traces dans
  `alicia_commands_log`.
- **RGPD** : les echantillons audio traites par le STT local ne quittent jamais le reseau local.
  La transcription texte n'est pas une donnee personnelle identifiante (commandes domotiques).

---

*Denis — Chef Dev Senior — Miyukini AI Studio — 2026-03-01*
