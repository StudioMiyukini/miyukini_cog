//! Parser NLU de secours base sur des patterns regex.
//!
//! Couvre les commandes P0 les plus courantes uniquement.
//! Pour les commandes ambigues ou non reconnues, retourne `Intent::Unknown`.

// @id: service.alicia.nlu-fallback
// @role: regex_nlu_fallback
// @layer: 7
// @human: Parser regex de secours pour NLU sans miou-llm-bridge.
// @do: parse_basic_home_commands_via_regex

use crate::intent::Intent;
use std::sync::LazyLock;

/// Parser NLU de secours base sur des patterns regex.
///
/// Les patterns regex sont compiles une seule fois au demarrage
/// via `std::sync::LazyLock`. La compilation ne se produit pas au runtime.
#[derive(Debug, Clone, Default)]
pub struct FallbackNluParser;

impl FallbackNluParser {
    /// Parse une transcription en intention domotique via regex.
    pub fn parse(&self, transcript: &str) -> Intent {
        let text = normalize(transcript);

        // Priorite 1 : routines nommees
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
        Intent::Unknown {
            transcript: transcript.to_string(),
        }
    }
}

/// Normalise le texte : minuscules, suppression de la ponctuation,
/// normalisation des accents pour le matching regex.
fn normalize(text: &str) -> String {
    text.to_lowercase()
        .replace(['é', 'è', 'ê', 'ë'], "e")
        .replace(['à', 'â'], "a")
        .replace(['ù', 'û'], "u")
        .replace('ô', "o")
        .replace(['î', 'ï'], "i")
        .replace('ç', "c")
        .replace(|c: char| c.is_ascii_punctuation(), " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

// --- Patterns compiles en LazyLock ---

static ROUTINE_PATTERNS: LazyLock<Vec<(regex::Regex, &'static str)>> = LazyLock::new(|| {
    vec![
        (
            regex::Regex::new(r"bonne\s*nuit").expect("pattern bonne nuit valide"),
            "bonne nuit",
        ),
        (
            regex::Regex::new(r"je\s*pars").expect("pattern je pars valide"),
            "je pars",
        ),
        (
            regex::Regex::new(r"mode\s*cinema").expect("pattern cinema valide"),
            "cinema",
        ),
        (
            regex::Regex::new(r"(?:routine\s+)?matin").expect("pattern matin valide"),
            "matin",
        ),
    ]
});

static LIGHT_ON: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)(allume|active|mets|eclaire)\s+(?:la\s+)?(?:lumiere|lampe|lumieres|spot)(?:s)?(?:\s+d(?:u|e\s+la|e\s+l|es)\s+(?P<room>\S+))?"
    ).expect("pattern lumiere on valide")
});

static LIGHT_OFF: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)(eteins?|coupe|desactive|ferme)\s+(?:la\s+|les\s+)?(?:lumiere|lampe|lumieres)(?:s)?(?:\s+d(?:u|e\s+la|e\s+l|es)\s+(?P<room>\S+))?"
    ).expect("pattern lumiere off valide")
});

static LIGHT_BRIGHTNESS: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)(?:baisse|monte|mets|regle)\s+(?:la\s+)?(?:lumiere|lampe)\s+(?:a|au)\s+(?P<level>\d+)\s*(?:pourcent|%|pour\s+cent)?"
    ).expect("pattern luminosite valide")
});

static THERMOSTAT: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)(?:regle|mets|chauffe|monte|baisse)\s+(?:le\s+)?(?:thermostat|chauffage|temperature)\s+(?:a|au)\s+(?P<temp>\d+(?:\.\d+)?)\s*(?:degres?|°)?"
    ).expect("pattern thermostat valide")
});

static SHUTTER_OPEN: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?i)(?:ouvre|leve|monte)\s+(?:les?\s+)?(?:volet|store|rideau|persienne)s?")
        .expect("pattern volet open valide")
});

static SHUTTER_CLOSE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)(?:ferme|baisse|descend)\s+(?:les?\s+)?(?:volet|store|rideau|persienne)s?",
    )
    .expect("pattern volet close valide")
});

static OUTLET_ON: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)(?:allume|active|branche)\s+(?:la\s+)?(?:prise|multiprise)(?:\s+d(?:u|e\s+la|e\s+l|es)\s+(?P<room>\S+))?"
    ).expect("pattern prise on valide")
});

static OUTLET_OFF: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)(?:eteins?|coupe|debranche|desactive)\s+(?:la\s+)?(?:prise|multiprise)(?:\s+d(?:u|e\s+la|e\s+l|es)\s+(?P<room>\S+))?"
    ).expect("pattern prise off valide")
});

static LOCK_PATTERN: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)(?P<action>verrouille|deverrouille|ouvre|ferme)\s+(?:la\s+)?(?:porte|serrure)",
    )
    .expect("pattern serrure valide")
});

// --- Fonctions de parsing ---

fn try_parse_routine(text: &str) -> Option<Intent> {
    for (pattern, name) in ROUTINE_PATTERNS.iter() {
        if pattern.is_match(text) {
            return Some(Intent::ActivateRoutine {
                routine_name: (*name).to_string(),
            });
        }
    }
    None
}

fn try_parse_light(text: &str) -> Option<Intent> {
    // Brightness en premier (plus specifique)
    if let Some(caps) = LIGHT_BRIGHTNESS.captures(text) {
        let level: u8 = caps
            .name("level")
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(50);
        return Some(Intent::ControlDevice {
            device_type: "light".to_string(),
            room_id: None,
            action: "set_brightness".to_string(),
            value: Some(serde_json::json!(level)),
        });
    }

    if let Some(caps) = LIGHT_ON.captures(text) {
        let room = caps.name("room").and_then(|m| extract_room_id(m.as_str()));
        return Some(Intent::ControlDevice {
            device_type: "light".to_string(),
            room_id: room,
            action: "on".to_string(),
            value: None,
        });
    }

    if let Some(caps) = LIGHT_OFF.captures(text) {
        let room = caps.name("room").and_then(|m| extract_room_id(m.as_str()));
        return Some(Intent::ControlDevice {
            device_type: "light".to_string(),
            room_id: room,
            action: "off".to_string(),
            value: None,
        });
    }

    None
}

fn try_parse_thermostat(text: &str) -> Option<Intent> {
    let caps = THERMOSTAT.captures(text)?;
    let temp: f64 = caps
        .name("temp")
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or(20.0);
    Some(Intent::ControlDevice {
        device_type: "thermostat".to_string(),
        room_id: None,
        action: "set_temperature".to_string(),
        value: Some(serde_json::json!(temp)),
    })
}

fn try_parse_shutter(text: &str) -> Option<Intent> {
    if SHUTTER_OPEN.is_match(text) {
        return Some(Intent::ControlDevice {
            device_type: "shutter".to_string(),
            room_id: None,
            action: "open".to_string(),
            value: None,
        });
    }
    if SHUTTER_CLOSE.is_match(text) {
        return Some(Intent::ControlDevice {
            device_type: "shutter".to_string(),
            room_id: None,
            action: "close".to_string(),
            value: None,
        });
    }
    None
}

fn try_parse_outlet(text: &str) -> Option<Intent> {
    if let Some(caps) = OUTLET_ON.captures(text) {
        let room = caps.name("room").and_then(|m| extract_room_id(m.as_str()));
        return Some(Intent::ControlDevice {
            device_type: "outlet".to_string(),
            room_id: room,
            action: "on".to_string(),
            value: None,
        });
    }
    if let Some(caps) = OUTLET_OFF.captures(text) {
        let room = caps.name("room").and_then(|m| extract_room_id(m.as_str()));
        return Some(Intent::ControlDevice {
            device_type: "outlet".to_string(),
            room_id: room,
            action: "off".to_string(),
            value: None,
        });
    }
    None
}

fn try_parse_lock(text: &str) -> Option<Intent> {
    let caps = LOCK_PATTERN.captures(text)?;
    let raw_action = caps.name("action")?.as_str().to_lowercase();
    let action = match raw_action.as_str() {
        "verrouille" | "ferme" => "lock",
        "deverrouille" | "ouvre" => "unlock",
        _ => return None,
    };
    Some(Intent::ControlDevice {
        device_type: "lock".to_string(),
        room_id: None,
        action: action.to_string(),
        value: None,
    })
}

/// Extrait le `room_id` depuis un nom de piece mentionne en langage naturel.
fn extract_room_id(text: &str) -> Option<String> {
    let normalized = normalize(text);
    match normalized.as_str() {
        "salon" => Some("salon".to_string()),
        "chambre parentale" | "chambre des parents" | "parents" => {
            Some("chambre-parentale".to_string())
        }
        "chambre theresa" | "chambre de theresa" | "theresa" => Some("chambre-theresa".to_string()),
        "chambre eleanore" | "chambre d eleanore" | "eleanore" => {
            Some("chambre-eleanore".to_string())
        }
        other if other.contains("salon") => Some("salon".to_string()),
        other if other.contains("theresa") => Some("chambre-theresa".to_string()),
        other if other.contains("eleanore") => Some("chambre-eleanore".to_string()),
        other if other.contains("parentale") || other.contains("parents") => {
            Some("chambre-parentale".to_string())
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TC-NLU-06 : "allume la lumiere du salon" -> ControlDevice on
    #[test]
    fn test_fallback_light_on_with_room() {
        let parser = FallbackNluParser;
        let intent = parser.parse("allume la lumiere du salon");
        match intent {
            Intent::ControlDevice {
                device_type,
                action,
                room_id,
                ..
            } => {
                assert_eq!(device_type, "light");
                assert_eq!(action, "on");
                assert_eq!(room_id, Some("salon".to_string()));
            }
            other => panic!("attendu ControlDevice, obtenu {:?}", other),
        }
    }

    // TC-NLU-07 : "eteins les lumieres" -> ControlDevice off sans room
    #[test]
    fn test_fallback_light_off_no_room() {
        let parser = FallbackNluParser;
        let intent = parser.parse("eteins les lumieres");
        match intent {
            Intent::ControlDevice {
                device_type,
                action,
                room_id,
                ..
            } => {
                assert_eq!(device_type, "light");
                assert_eq!(action, "off");
                assert!(room_id.is_none());
            }
            other => panic!("attendu ControlDevice off, obtenu {:?}", other),
        }
    }

    // TC-NLU-08 : "baisse la lampe a 50 pourcent" -> set_brightness 50
    #[test]
    fn test_fallback_light_brightness() {
        let parser = FallbackNluParser;
        let intent = parser.parse("baisse la lampe a 50 pourcent");
        match intent {
            Intent::ControlDevice {
                device_type,
                action,
                value,
                ..
            } => {
                assert_eq!(device_type, "light");
                assert_eq!(action, "set_brightness");
                assert_eq!(value, Some(serde_json::json!(50)));
            }
            other => panic!("attendu set_brightness, obtenu {:?}", other),
        }
    }

    // TC-NLU-09 : "regle le thermostat a 20 degres" -> set_temperature 20.0
    #[test]
    fn test_fallback_thermostat_temperature() {
        let parser = FallbackNluParser;
        let intent = parser.parse("regle le thermostat a 20 degres");
        match intent {
            Intent::ControlDevice {
                device_type,
                action,
                value,
                ..
            } => {
                assert_eq!(device_type, "thermostat");
                assert_eq!(action, "set_temperature");
                assert_eq!(value, Some(serde_json::json!(20.0)));
            }
            other => panic!("attendu set_temperature, obtenu {:?}", other),
        }
    }

    // TC-NLU-10 : "mets le chauffage a 18" (sans "degres") -> set_temperature 18.0
    #[test]
    fn test_fallback_thermostat_no_unit() {
        let parser = FallbackNluParser;
        let intent = parser.parse("mets le chauffage a 18");
        match intent {
            Intent::ControlDevice {
                device_type,
                action,
                value,
                ..
            } => {
                assert_eq!(device_type, "thermostat");
                assert_eq!(action, "set_temperature");
                assert_eq!(value, Some(serde_json::json!(18.0)));
            }
            other => panic!("attendu set_temperature 18, obtenu {:?}", other),
        }
    }

    // TC-NLU-11 : "ouvre les volets" -> ControlDevice shutter open
    #[test]
    fn test_fallback_shutter_open() {
        let parser = FallbackNluParser;
        let intent = parser.parse("ouvre les volets");
        match intent {
            Intent::ControlDevice {
                device_type,
                action,
                ..
            } => {
                assert_eq!(device_type, "shutter");
                assert_eq!(action, "open");
            }
            other => panic!("attendu shutter open, obtenu {:?}", other),
        }
    }

    // TC-NLU-12 : "bonne nuit" -> ActivateRoutine
    #[test]
    fn test_fallback_routine_bonne_nuit() {
        let parser = FallbackNluParser;
        let intent = parser.parse("bonne nuit");
        assert!(matches!(
            intent,
            Intent::ActivateRoutine { routine_name } if routine_name == "bonne nuit"
        ));
    }

    // TC-NLU-13 : "commande une pizza" -> Unknown
    #[test]
    fn test_fallback_unknown_intent() {
        let parser = FallbackNluParser;
        let intent = parser.parse("commande une pizza");
        assert!(matches!(intent, Intent::Unknown { .. }));
    }

    // TC-NLU-14 : Intent::is_actionable() correct
    #[test]
    fn test_intent_is_actionable() {
        let parser = FallbackNluParser;

        let on = parser.parse("allume la lumiere");
        assert!(on.is_actionable());

        let routine = parser.parse("bonne nuit");
        assert!(routine.is_actionable());

        let unknown = parser.parse("quelle heure est-il");
        assert!(!unknown.is_actionable());
    }

    // TC-NLU-15 : normalisation accents
    #[test]
    fn test_normalize_accents() {
        let result = normalize("éteins la lumière du château");
        assert_eq!(result, "eteins la lumiere du chateau");
    }

    // TC-NLU-16 : extraction room_id "chambre de theresa" -> "chambre-theresa"
    #[test]
    fn test_extract_room_id_chambre_theresa() {
        let room = extract_room_id("theresa");
        assert_eq!(room, Some("chambre-theresa".to_string()));
    }

    #[test]
    fn test_fallback_shutter_close() {
        let parser = FallbackNluParser;
        let intent = parser.parse("ferme les volets");
        match intent {
            Intent::ControlDevice {
                device_type,
                action,
                ..
            } => {
                assert_eq!(device_type, "shutter");
                assert_eq!(action, "close");
            }
            other => panic!("attendu shutter close, obtenu {:?}", other),
        }
    }

    #[test]
    fn test_fallback_outlet_off() {
        let parser = FallbackNluParser;
        let intent = parser.parse("eteins la prise");
        match intent {
            Intent::ControlDevice {
                device_type,
                action,
                ..
            } => {
                assert_eq!(device_type, "outlet");
                assert_eq!(action, "off");
            }
            other => panic!("attendu outlet off, obtenu {:?}", other),
        }
    }

    #[test]
    fn test_fallback_lock() {
        let parser = FallbackNluParser;
        let intent = parser.parse("verrouille la porte");
        match intent {
            Intent::ControlDevice {
                device_type,
                action,
                ..
            } => {
                assert_eq!(device_type, "lock");
                assert_eq!(action, "lock");
            }
            other => panic!("attendu lock, obtenu {:?}", other),
        }
    }

    #[test]
    fn test_fallback_unlock() {
        let parser = FallbackNluParser;
        let intent = parser.parse("deverrouille la serrure");
        match intent {
            Intent::ControlDevice {
                device_type,
                action,
                ..
            } => {
                assert_eq!(device_type, "lock");
                assert_eq!(action, "unlock");
            }
            other => panic!("attendu unlock, obtenu {:?}", other),
        }
    }

    #[test]
    fn test_fallback_routine_je_pars() {
        let parser = FallbackNluParser;
        let intent = parser.parse("je pars");
        assert!(matches!(
            intent,
            Intent::ActivateRoutine { routine_name } if routine_name == "je pars"
        ));
    }

    #[test]
    fn test_extract_room_id_salon() {
        assert_eq!(extract_room_id("salon"), Some("salon".to_string()));
    }

    #[test]
    fn test_extract_room_id_unknown() {
        assert_eq!(extract_room_id("garage"), None);
    }
}
