//! Tool MiyuValidate — tool.validate.schema.check.
//! Valide des données selon un schéma fourni (ex. JSON Schema simplifié) ; retourne succès ou liste d'erreurs.

use crate::context::GovernedContext;
use crate::errors::MiyuValidateError;
use serde_json::Value;

/// Résultat de validation (codes d'erreur, pas de fuite métier — BOUND-*).
#[derive(Debug, Clone, Default)]
pub struct SchemaCheckResult {
    /// true si valide.
    pub valid: bool,
    /// Codes d'erreur (identifiants, pas messages sensibles).
    pub error_codes: Vec<String>,
}

/// @id: miyuvalidate_tool_schema_check
/// @role: mutator
/// @layer: tool
/// @human: Valide des données selon un schéma fourni ; retourne succès ou liste d'erreurs.
/// @do: schema_check_under_governance
/// tool.validate.schema.check
pub fn check(
    ctx: &GovernedContext,
    data: &str,
    schema: &str,
) -> Result<SchemaCheckResult, MiyuValidateError> {
    if !ctx.has_mandate() {
        return Err(MiyuValidateError::NoMandate);
    }
    let data_val: Value =
        serde_json::from_str(data).map_err(|e| MiyuValidateError::InvalidData(e.to_string()))?;
    let schema_val: Value = serde_json::from_str(schema)
        .map_err(|e| MiyuValidateError::InvalidSchema(e.to_string()))?;
    let mut error_codes = Vec::new();
    if let Some(required) = schema_val.get("required").and_then(Value::as_array) {
        let obj = data_val
            .as_object()
            .ok_or_else(|| MiyuValidateError::InvalidData("data is not an object".to_string()))?;
        for key in required {
            if let Some(k) = key.as_str() {
                if !obj.contains_key(k) {
                    error_codes.push(format!("missing_required:{k}"));
                }
            }
        }
    }
    if let Some(properties) = schema_val.get("properties").and_then(Value::as_object) {
        let obj = data_val
            .as_object()
            .ok_or_else(|| MiyuValidateError::InvalidData("data is not an object".to_string()))?;
        for (key, type_spec) in properties {
            if let Some(v) = obj.get(key) {
                if let Some(t) = type_spec.as_str() {
                    if !type_matches(v, t) {
                        error_codes.push(format!("type_mismatch:{key}"));
                    }
                }
            }
        }
    }
    Ok(SchemaCheckResult {
        valid: error_codes.is_empty(),
        error_codes,
    })
}

fn type_matches(v: &Value, t: &str) -> bool {
    match t {
        "string" => v.is_string(),
        "number" => v.is_f64() || v.is_i64(),
        "integer" => v.is_i64(),
        "boolean" => v.is_boolean(),
        "object" => v.is_object(),
        "array" => v.is_array(),
        "null" => v.is_null(),
        _ => true,
    }
}
