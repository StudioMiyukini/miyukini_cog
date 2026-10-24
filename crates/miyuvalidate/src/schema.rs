//! Tool MiyuValidate — tool.validate.schema.check.
//! Valide des données selon un schéma fourni (JSON Schema simplifié) ; retourne succès ou liste d'erreurs.
//!
//! Sous-ensemble supporté (tout le reste est **refusé** avec `InvalidSchema`,
//! jamais ignoré — une contrainte ignorée serait une validation fail-open) :
//! - mots-clés racine : `required` (tableau de chaînes), `properties` (objet),
//!   `type` (uniquement `"object"`), annotations `$schema`, `$id`, `title`, `description` ;
//! - valeur d'une propriété : nom de type (`"string"`) ou objet `{"type": "string"}`
//!   (annotations `title` / `description` tolérées) ;
//! - types : `string`, `number`, `integer`, `boolean`, `object`, `array`, `null`.

use crate::context::GovernedContext;
use crate::errors::MiyuValidateError;
use serde_json::{Map, Value};

/// Taille maximale des données à valider (octets).
pub const MAX_DATA_BYTES: usize = 1024 * 1024;
/// Taille maximale du schéma (octets).
pub const MAX_SCHEMA_BYTES: usize = 64 * 1024;
/// Nombre maximal d'entrées dans `required` ou `properties`.
pub const MAX_SCHEMA_ENTRIES: usize = 256;

const ROOT_KEYWORDS: &[&str] = &[
    "$schema",
    "$id",
    "title",
    "description",
    "type",
    "required",
    "properties",
];
const PROPERTY_KEYWORDS: &[&str] = &["type", "title", "description"];
const TYPES: &[&str] = &[
    "string", "number", "integer", "boolean", "object", "array", "null",
];

/// Résultat de validation (codes d'erreur, pas de fuite métier — BOUND-*).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SchemaCheckResult {
    /// true si valide.
    pub valid: bool,
    /// Codes d'erreur (identifiants, pas messages sensibles).
    pub error_codes: Vec<String>,
}

/// @id miyuvalidate_tool_schema_check
/// @role mutator
/// @layer tool
/// @do schema_check_under_governance
//@human / @human Valide des données selon un schéma fourni ; retourne succès ou liste d'erreurs.
/// tool.validate.schema.check
pub fn check(
    ctx: &GovernedContext,
    data: &str,
    schema: &str,
) -> Result<SchemaCheckResult, MiyuValidateError> {
    if !ctx.has_mandate() {
        return Err(MiyuValidateError::NoMandate);
    }
    if data.len() > MAX_DATA_BYTES {
        return Err(MiyuValidateError::InputTooLarge {
            what: "data",
            max: MAX_DATA_BYTES,
        });
    }
    if schema.len() > MAX_SCHEMA_BYTES {
        return Err(MiyuValidateError::InputTooLarge {
            what: "schema",
            max: MAX_SCHEMA_BYTES,
        });
    }
    let data_val: Value =
        serde_json::from_str(data).map_err(|e| MiyuValidateError::InvalidData(e.to_string()))?;
    let schema_val: Value = serde_json::from_str(schema)
        .map_err(|e| MiyuValidateError::InvalidSchema(e.to_string()))?;
    let schema_obj = schema_val
        .as_object()
        .ok_or_else(|| MiyuValidateError::InvalidSchema("schema is not an object".into()))?;
    let (required, properties) = parse_schema(schema_obj)?;

    let needs_object =
        !required.is_empty() || !properties.is_empty() || schema_obj.contains_key("type");
    if !needs_object {
        return Ok(SchemaCheckResult {
            valid: true,
            error_codes: Vec::new(),
        });
    }
    let obj = data_val
        .as_object()
        .ok_or_else(|| MiyuValidateError::InvalidData("data is not an object".to_string()))?;

    let mut error_codes = Vec::new();
    for k in required {
        if !obj.contains_key(k) {
            error_codes.push(format!("missing_required:{k}"));
        }
    }
    for (key, t) in properties {
        if let Some(v) = obj.get(key) {
            if !type_matches(v, t) {
                error_codes.push(format!("type_mismatch:{key}"));
            }
        }
    }
    Ok(SchemaCheckResult {
        valid: error_codes.is_empty(),
        error_codes,
    })
}

type ParsedSchema<'a> = (Vec<&'a str>, Vec<(&'a str, &'a str)>);

/// Analyse le schéma ; refuse toute construction non supportée.
fn parse_schema(schema: &Map<String, Value>) -> Result<ParsedSchema<'_>, MiyuValidateError> {
    for key in schema.keys() {
        if !ROOT_KEYWORDS.contains(&key.as_str()) {
            return Err(MiyuValidateError::InvalidSchema(format!(
                "unsupported_keyword:{key}"
            )));
        }
    }
    if let Some(t) = schema.get("type") {
        if t.as_str() != Some("object") {
            return Err(MiyuValidateError::InvalidSchema(
                "root type must be \"object\"".into(),
            ));
        }
    }
    let mut required = Vec::new();
    if let Some(r) = schema.get("required") {
        let arr = r
            .as_array()
            .ok_or_else(|| MiyuValidateError::InvalidSchema("required must be an array".into()))?;
        if arr.len() > MAX_SCHEMA_ENTRIES {
            return Err(MiyuValidateError::InputTooLarge {
                what: "schema.required",
                max: MAX_SCHEMA_ENTRIES,
            });
        }
        for item in arr {
            let k = item.as_str().ok_or_else(|| {
                MiyuValidateError::InvalidSchema("required entries must be strings".into())
            })?;
            required.push(k);
        }
    }
    let mut properties = Vec::new();
    if let Some(p) = schema.get("properties") {
        let map = p.as_object().ok_or_else(|| {
            MiyuValidateError::InvalidSchema("properties must be an object".into())
        })?;
        if map.len() > MAX_SCHEMA_ENTRIES {
            return Err(MiyuValidateError::InputTooLarge {
                what: "schema.properties",
                max: MAX_SCHEMA_ENTRIES,
            });
        }
        for (key, spec) in map {
            properties.push((key.as_str(), property_type(key, spec)?));
        }
    }
    Ok((required, properties))
}

/// Type attendu d'une propriété (`"string"` ou `{"type": "string"}`).
fn property_type<'a>(key: &str, spec: &'a Value) -> Result<&'a str, MiyuValidateError> {
    let t = match spec {
        Value::String(s) => s.as_str(),
        Value::Object(o) => {
            for k in o.keys() {
                if !PROPERTY_KEYWORDS.contains(&k.as_str()) {
                    return Err(MiyuValidateError::InvalidSchema(format!(
                        "unsupported_keyword:{key}.{k}"
                    )));
                }
            }
            o.get("type")
                .and_then(Value::as_str)
                .ok_or_else(|| MiyuValidateError::InvalidSchema(format!("missing_type:{key}")))?
        }
        _ => {
            return Err(MiyuValidateError::InvalidSchema(format!(
                "invalid_property_spec:{key}"
            )))
        }
    };
    if TYPES.contains(&t) {
        Ok(t)
    } else {
        Err(MiyuValidateError::InvalidSchema(format!(
            "unknown_type:{key}"
        )))
    }
}

fn type_matches(v: &Value, t: &str) -> bool {
    match t {
        "string" => v.is_string(),
        "number" => v.is_number(),
        "integer" => v.is_i64() || v.is_u64(),
        "boolean" => v.is_boolean(),
        "object" => v.is_object(),
        "array" => v.is_array(),
        "null" => v.is_null(),
        // Inatteignable (types validés en amont) ; fail-closed par construction.
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("m".into(), 1)
    }

    #[test]
    fn refuse_sans_mandat() {
        let empty = GovernedContext::new(String::new(), 1);
        assert_eq!(check(&empty, "{}", "{}"), Err(MiyuValidateError::NoMandate));
    }

    #[test]
    fn donnees_valides() {
        let schema = r#"{"type":"object","required":["nom","age"],"properties":{"nom":"string","age":{"type":"integer"}}}"#;
        let r = check(&ctx(), r#"{"nom":"Ada","age":36}"#, schema).unwrap();
        assert!(r.valid);
        assert_eq!(r.error_codes, Vec::<String>::new());
    }

    #[test]
    fn champ_requis_manquant_et_type_incorrect() {
        let schema = r#"{"required":["nom","age"],"properties":{"age":"integer"}}"#;
        let r = check(&ctx(), r#"{"age":"36"}"#, schema).unwrap();
        assert!(!r.valid);
        assert!(r.error_codes.contains(&"missing_required:nom".to_string()));
        assert!(r.error_codes.contains(&"type_mismatch:age".to_string()));
    }

    #[test]
    fn nombres_u64_et_flottants() {
        let schema = r#"{"properties":{"n":"number","i":"integer"}}"#;
        let r = check(
            &ctx(),
            r#"{"n":18446744073709551615,"i":18446744073709551615}"#,
            schema,
        )
        .unwrap();
        assert!(r.valid);
        let r = check(&ctx(), r#"{"i":1.5}"#, schema).unwrap();
        assert_eq!(r.error_codes, vec!["type_mismatch:i".to_string()]);
    }

    #[test]
    fn type_inconnu_refuse() {
        let r = check(&ctx(), r#"{"a":1}"#, r#"{"properties":{"a":"uuid"}}"#);
        assert!(matches!(r, Err(MiyuValidateError::InvalidSchema(_))));
    }

    #[test]
    fn mot_cle_non_supporte_refuse() {
        let r = check(&ctx(), r#"{"a":1}"#, r#"{"additionalProperties":false}"#);
        assert!(matches!(r, Err(MiyuValidateError::InvalidSchema(_))));
        let r = check(
            &ctx(),
            r#"{"a":"x"}"#,
            r#"{"properties":{"a":{"type":"string","maxLength":3}}}"#,
        );
        assert!(matches!(r, Err(MiyuValidateError::InvalidSchema(_))));
    }

    #[test]
    fn schema_mal_forme_refuse() {
        for s in [
            "[]",
            r#"{"required":"a"}"#,
            r#"{"required":[1]}"#,
            r#"{"properties":[]}"#,
            r#"{"properties":{"a":1}}"#,
            r#"{"properties":{"a":{}}}"#,
            r#"{"type":"array"}"#,
            "pas du json",
        ] {
            assert!(
                matches!(
                    check(&ctx(), "{}", s),
                    Err(MiyuValidateError::InvalidSchema(_))
                ),
                "schéma accepté à tort : {s}"
            );
        }
    }

    #[test]
    fn donnees_non_objet_refusees() {
        let r = check(&ctx(), "[1]", r#"{"required":["a"]}"#);
        assert!(matches!(r, Err(MiyuValidateError::InvalidData(_))));
        let r = check(&ctx(), "{", "{}");
        assert!(matches!(r, Err(MiyuValidateError::InvalidData(_))));
    }

    #[test]
    fn entrees_trop_grandes_refusees() {
        let big = format!("\"{}\"", "x".repeat(MAX_DATA_BYTES));
        assert!(matches!(
            check(&ctx(), &big, "{}"),
            Err(MiyuValidateError::InputTooLarge { what: "data", .. })
        ));
        let big_schema = format!("{{\"title\":\"{}\"}}", "x".repeat(MAX_SCHEMA_BYTES));
        assert!(matches!(
            check(&ctx(), "{}", &big_schema),
            Err(MiyuValidateError::InputTooLarge { what: "schema", .. })
        ));
        let many: Vec<String> = (0..=MAX_SCHEMA_ENTRIES)
            .map(|i| format!("\"k{i}\""))
            .collect();
        let s = format!("{{\"required\":[{}]}}", many.join(","));
        assert!(matches!(
            check(&ctx(), "{}", &s),
            Err(MiyuValidateError::InputTooLarge { .. })
        ));
    }
}
