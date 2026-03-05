//! Evaluateur de conditions pour les automatisations.
//!
//! `evaluate_conditions` evalue une liste de conditions contre le snapshot
//! de la maison (ET logique). Liste vide = toujours vrai.

// @id: toolkit.alicia.automations.evaluator
// @role: condition_evaluator
// @layer: 6
// @human: Evaluateur de conditions domotiques : proprietes DeviceState, contexte (heure, jour).
// @do: evaluate_automation_conditions

use crate::dispatcher::HomeSnapshot;
use crate::types::{Condition, ConditionOp};

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
/// # Proprietes de DeviceState (device_id = Some(...))
///
/// La propriete est extraite par nom depuis `DeviceState`.
/// Si la propriete est `None` (etat inconnu), la condition est evaluee `false`
/// (principe de securite : etat inconnu = condition non remplie).
pub fn evaluate_conditions(conditions: &[Condition], snapshot: &HomeSnapshot) -> bool {
    conditions
        .iter()
        .all(|cond| evaluate_single(cond, snapshot))
}

/// Evalue une seule condition contre le snapshot.
fn evaluate_single(condition: &Condition, snapshot: &HomeSnapshot) -> bool {
    let actual_value = get_actual_value(condition, snapshot);

    if let Some(actual) = actual_value {
        compare_json(&actual, condition.op, &condition.value)
    } else {
        tracing::debug!(
            property = %condition.property,
            "condition evaluee false : propriete inconnue ou dispositif absent"
        );
        false
    }
}

/// Extrait la valeur actuelle d'une propriete depuis le snapshot.
///
/// Pour les proprietes contextuelles (`hour`, `weekday`), utilise le snapshot directement.
/// Pour les proprietes DeviceState, cherche l'etat du dispositif dans le snapshot.
fn get_actual_value(condition: &Condition, snapshot: &HomeSnapshot) -> Option<serde_json::Value> {
    match &condition.device_id {
        // Proprietes contextuelles
        None => match condition.property.as_str() {
            "hour" => Some(serde_json::Value::from(snapshot.hour)),
            "weekday" => Some(serde_json::Value::from(snapshot.weekday)),
            _ => {
                tracing::warn!(
                    property = %condition.property,
                    "propriete contextuelle inconnue"
                );
                None
            }
        },
        // Proprietes DeviceState
        Some(device_id) => {
            let state = snapshot.device_states.get(device_id)?;
            extract_device_property(state, &condition.property)
        }
    }
}

/// Extrait une propriete nommee d'un `DeviceState` sous forme de `serde_json::Value`.
fn extract_device_property(
    state: &miyualicia_devices::DeviceState,
    property: &str,
) -> Option<serde_json::Value> {
    match property {
        "on" => state.on.map(serde_json::Value::from),
        "brightness" => state.brightness.map(serde_json::Value::from),
        "temperature_current" => state.temperature_current.map(|v| serde_json::json!(v)),
        "temperature_target" => state.temperature_target.map(|v| serde_json::json!(v)),
        "humidity" => state.humidity.map(|v| serde_json::json!(v)),
        "motion" => state.motion.map(serde_json::Value::from),
        "contact" => state.contact.map(serde_json::Value::from),
        "locked" => state.locked.map(serde_json::Value::from),
        "position" => state.position.map(serde_json::Value::from),
        "power_w" => state.power_w.map(|v| serde_json::json!(v)),
        _ => {
            tracing::warn!(
                property = %property,
                "propriete DeviceState inconnue"
            );
            None
        }
    }
}

/// Compare deux valeurs JSON avec l'operateur donne.
///
/// Supports : bool==bool, number op number, number Between [min, max].
/// Retourne `false` si les types sont incompatibles.
fn compare_json(actual: &serde_json::Value, op: ConditionOp, expected: &serde_json::Value) -> bool {
    match op {
        ConditionOp::Eq => json_eq(actual, expected),
        ConditionOp::Ne => !json_eq(actual, expected),
        ConditionOp::Gt => json_cmp(actual, expected) == Some(std::cmp::Ordering::Greater),
        ConditionOp::Lt => json_cmp(actual, expected) == Some(std::cmp::Ordering::Less),
        ConditionOp::Gte => {
            json_cmp(actual, expected).is_some_and(|ord| ord != std::cmp::Ordering::Less)
        }
        ConditionOp::Lte => {
            json_cmp(actual, expected).is_some_and(|ord| ord != std::cmp::Ordering::Greater)
        }
        ConditionOp::Between => json_between(actual, expected),
    }
}

/// Egalite entre deux valeurs JSON.
///
/// Pour les nombres, compare les valeurs f64 (tolere int vs float).
fn json_eq(a: &serde_json::Value, b: &serde_json::Value) -> bool {
    match (a, b) {
        (serde_json::Value::Bool(va), serde_json::Value::Bool(vb)) => va == vb,
        (serde_json::Value::Number(na), serde_json::Value::Number(nb)) => {
            let fa = na.as_f64();
            let fb = nb.as_f64();
            match (fa, fb) {
                (Some(va), Some(vb)) => (va - vb).abs() < f64::EPSILON,
                _ => false,
            }
        }
        (serde_json::Value::String(sa), serde_json::Value::String(sb)) => sa == sb,
        _ => a == b,
    }
}

/// Comparaison ordonnee entre deux valeurs JSON numeriques.
fn json_cmp(a: &serde_json::Value, b: &serde_json::Value) -> Option<std::cmp::Ordering> {
    let fa = a.as_f64()?;
    let fb = b.as_f64()?;
    fa.partial_cmp(&fb)
}

/// Verifie qu'une valeur est dans un intervalle [min, max] inclus.
///
/// `expected` doit etre un tableau JSON de 2 elements : `[min, max]`.
fn json_between(actual: &serde_json::Value, expected: &serde_json::Value) -> bool {
    let arr = match expected.as_array() {
        Some(a) if a.len() == 2 => a,
        _ => {
            tracing::warn!("Between attend un tableau [min, max], recu : {expected}");
            return false;
        }
    };

    let Some(actual_f) = actual.as_f64() else {
        return false;
    };

    let Some(min) = arr[0].as_f64() else {
        return false;
    };

    let Some(max) = arr[1].as_f64() else {
        return false;
    };

    actual_f >= min && actual_f <= max
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyualicia_devices::DeviceState;
    use uuid::Uuid;

    fn snapshot_with_hour(hour: u32, weekday: u32) -> HomeSnapshot {
        HomeSnapshot {
            hour,
            weekday,
            device_states: std::collections::HashMap::new(),
        }
    }

    fn snapshot_with_device(
        device_id: Uuid,
        state: DeviceState,
        hour: u32,
        weekday: u32,
    ) -> HomeSnapshot {
        let mut device_states = std::collections::HashMap::new();
        device_states.insert(device_id, state);
        HomeSnapshot {
            hour,
            weekday,
            device_states,
        }
    }

    // TC-AUTO-04 : evaluate_conditions toutes vraies -> true
    #[test]
    fn test_evaluate_all_conditions_true() {
        let dev_id = Uuid::new_v4();
        let mut state = DeviceState::unknown(dev_id);
        state.temperature_current = Some(26.0);
        state.motion = Some(true);

        let snapshot = snapshot_with_device(dev_id, state, 22, 0);

        let conditions = vec![
            Condition {
                device_id: Some(dev_id),
                property: "temperature_current".to_string(),
                op: ConditionOp::Gte,
                value: serde_json::json!(25.0),
            },
            Condition {
                device_id: Some(dev_id),
                property: "motion".to_string(),
                op: ConditionOp::Eq,
                value: serde_json::json!(true),
            },
        ];

        assert!(evaluate_conditions(&conditions, &snapshot));
    }

    // TC-AUTO-05 : evaluate_conditions une fausse -> false
    #[test]
    fn test_evaluate_one_condition_false() {
        let dev_id = Uuid::new_v4();
        let mut state = DeviceState::unknown(dev_id);
        state.temperature_current = Some(24.0);
        state.motion = Some(true);

        let snapshot = snapshot_with_device(dev_id, state, 22, 0);

        let conditions = vec![
            Condition {
                device_id: Some(dev_id),
                property: "temperature_current".to_string(),
                op: ConditionOp::Gte,
                value: serde_json::json!(25.0),
            },
            Condition {
                device_id: Some(dev_id),
                property: "motion".to_string(),
                op: ConditionOp::Eq,
                value: serde_json::json!(true),
            },
        ];

        assert!(!evaluate_conditions(&conditions, &snapshot));
    }

    // TC-AUTO-06 : evaluate_conditions vide -> true
    #[test]
    fn test_evaluate_empty_conditions_is_true() {
        let snapshot = HomeSnapshot::empty();
        assert!(evaluate_conditions(&[], &snapshot));
    }

    // TC-AUTO-07 : condition hour Eq
    #[test]
    fn test_evaluate_hour_eq() {
        let snapshot = snapshot_with_hour(22, 0);
        let conditions = vec![Condition {
            device_id: None,
            property: "hour".to_string(),
            op: ConditionOp::Eq,
            value: serde_json::json!(22),
        }];
        assert!(evaluate_conditions(&conditions, &snapshot));

        let conditions_false = vec![Condition {
            device_id: None,
            property: "hour".to_string(),
            op: ConditionOp::Eq,
            value: serde_json::json!(21),
        }];
        assert!(!evaluate_conditions(&conditions_false, &snapshot));
    }

    // TC-AUTO-08 : condition Between
    #[test]
    fn test_evaluate_hour_between() {
        let snapshot = snapshot_with_hour(22, 0);
        let conditions = vec![Condition {
            device_id: None,
            property: "hour".to_string(),
            op: ConditionOp::Between,
            value: serde_json::json!([22, 23]),
        }];
        assert!(evaluate_conditions(&conditions, &snapshot));

        // Hors intervalle
        let snapshot_out = snapshot_with_hour(21, 0);
        assert!(!evaluate_conditions(&conditions, &snapshot_out));
    }

    // TC-AUTO-09 : condition Ne
    #[test]
    fn test_evaluate_ne() {
        let snapshot = snapshot_with_hour(22, 0);
        let conditions = vec![Condition {
            device_id: None,
            property: "hour".to_string(),
            op: ConditionOp::Ne,
            value: serde_json::json!(21),
        }];
        assert!(evaluate_conditions(&conditions, &snapshot));

        let conditions_eq = vec![Condition {
            device_id: None,
            property: "hour".to_string(),
            op: ConditionOp::Ne,
            value: serde_json::json!(22),
        }];
        assert!(!evaluate_conditions(&conditions_eq, &snapshot));
    }

    // TC-AUTO-10 : condition property DeviceState inconnue -> false
    #[test]
    fn test_evaluate_unknown_device_state_is_false() {
        let dev_id = Uuid::new_v4();
        // Le dispositif n'est pas dans le snapshot
        let snapshot = HomeSnapshot::empty();
        let conditions = vec![Condition {
            device_id: Some(dev_id),
            property: "motion".to_string(),
            op: ConditionOp::Eq,
            value: serde_json::json!(true),
        }];
        assert!(!evaluate_conditions(&conditions, &snapshot));
    }

    #[test]
    fn test_evaluate_unknown_property_is_false() {
        let dev_id = Uuid::new_v4();
        let state = DeviceState::unknown(dev_id);
        let snapshot = snapshot_with_device(dev_id, state, 0, 0);

        // motion est None sur un etat inconnu
        let conditions = vec![Condition {
            device_id: Some(dev_id),
            property: "motion".to_string(),
            op: ConditionOp::Eq,
            value: serde_json::json!(true),
        }];
        assert!(!evaluate_conditions(&conditions, &snapshot));
    }

    #[test]
    fn test_evaluate_temperature_above_threshold() {
        let dev_id = Uuid::new_v4();
        let mut state = DeviceState::unknown(dev_id);
        state.temperature_current = Some(26.0);

        let snapshot = snapshot_with_device(dev_id, state, 0, 0);
        let conditions = vec![Condition {
            device_id: Some(dev_id),
            property: "temperature_current".to_string(),
            op: ConditionOp::Gte,
            value: serde_json::json!(25.0),
        }];
        assert!(evaluate_conditions(&conditions, &snapshot));
    }

    #[test]
    fn test_evaluate_temperature_below_threshold() {
        let dev_id = Uuid::new_v4();
        let mut state = DeviceState::unknown(dev_id);
        state.temperature_current = Some(24.0);

        let snapshot = snapshot_with_device(dev_id, state, 0, 0);
        let conditions = vec![Condition {
            device_id: Some(dev_id),
            property: "temperature_current".to_string(),
            op: ConditionOp::Gte,
            value: serde_json::json!(25.0),
        }];
        assert!(!evaluate_conditions(&conditions, &snapshot));
    }

    #[test]
    fn test_evaluate_weekday_mismatch() {
        // weekday = 1 (mardi), on veut lundi (0)
        let snapshot = snapshot_with_hour(10, 1);
        let conditions = vec![Condition {
            device_id: None,
            property: "weekday".to_string(),
            op: ConditionOp::Eq,
            value: serde_json::json!(0),
        }];
        assert!(!evaluate_conditions(&conditions, &snapshot));
    }

    #[test]
    fn test_evaluate_gt_lt() {
        let snapshot = snapshot_with_hour(15, 0);
        let cond_gt = vec![Condition {
            device_id: None,
            property: "hour".to_string(),
            op: ConditionOp::Gt,
            value: serde_json::json!(14),
        }];
        assert!(evaluate_conditions(&cond_gt, &snapshot));

        let cond_lt = vec![Condition {
            device_id: None,
            property: "hour".to_string(),
            op: ConditionOp::Lt,
            value: serde_json::json!(16),
        }];
        assert!(evaluate_conditions(&cond_lt, &snapshot));

        let cond_gt_false = vec![Condition {
            device_id: None,
            property: "hour".to_string(),
            op: ConditionOp::Gt,
            value: serde_json::json!(15),
        }];
        assert!(!evaluate_conditions(&cond_gt_false, &snapshot));
    }

    #[test]
    fn test_evaluate_bool_eq() {
        let dev_id = Uuid::new_v4();
        let mut state = DeviceState::unknown(dev_id);
        state.locked = Some(true);

        let snapshot = snapshot_with_device(dev_id, state, 0, 0);
        let conditions = vec![Condition {
            device_id: Some(dev_id),
            property: "locked".to_string(),
            op: ConditionOp::Eq,
            value: serde_json::json!(true),
        }];
        assert!(evaluate_conditions(&conditions, &snapshot));
    }

    #[test]
    fn test_evaluate_between_malformed() {
        let snapshot = snapshot_with_hour(22, 0);
        // Between avec une seule valeur -> false
        let conditions = vec![Condition {
            device_id: None,
            property: "hour".to_string(),
            op: ConditionOp::Between,
            value: serde_json::json!([22]),
        }];
        assert!(!evaluate_conditions(&conditions, &snapshot));
    }

    #[test]
    fn test_evaluate_unknown_context_property() {
        let snapshot = snapshot_with_hour(10, 0);
        let conditions = vec![Condition {
            device_id: None,
            property: "unknown_prop".to_string(),
            op: ConditionOp::Eq,
            value: serde_json::json!(10),
        }];
        assert!(!evaluate_conditions(&conditions, &snapshot));
    }
}
