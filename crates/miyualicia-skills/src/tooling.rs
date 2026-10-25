//! Conversion `ServiceCapability` ↔ tool calling OpenAI / Anthropic.
//!
//! Le format choisi est celui d'OpenAI Function Calling (largement compatible
//! avec Anthropic, Mistral, et la plupart des bridges) :
//!
//! ```json
//! {
//!   "type": "function",
//!   "function": {
//!     "name": "jaykoa.schedule_event",
//!     "description": "Programme un nouvel événement.",
//!     "parameters": {
//!       "type": "object",
//!       "properties": {
//!         "title":    { "type": "string",  "description": "Titre" },
//!         "start_at": { "type": "string",  "format": "date-time" }
//!       },
//!       "required": ["title", "start_at"]
//!     }
//!   }
//! }
//! ```
//!
//! Le `name` du tool est l'`qualified_id` de la capability
//! (`service_id.intent`). Quand un tool call revient du LLM, on parse `name`
//! pour retrouver la capability dans le `SkillRegistry`.

// @id service.alicia.skills.tooling
// @role tool_calling_bridge
// @layer 7
// @do bridge_capability_to_llm_tools
// @human Conversion bidirectionnelle ServiceCapability ↔ tool calling LLM.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::capability::{ParamSchema, ParamType, ServiceCapability};
use crate::errors::SkillsError;

/// Définition d'un outil au format OpenAI function calling.
///
/// Sérialisable directement en JSON pour insertion dans le champ `tools`
/// d'une requête `chat/completions`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolDefinition {
    /// Toujours `"function"` pour OpenAI / Anthropic / Mistral.
    #[serde(rename = "type")]
    pub tool_type: String,
    /// Définition de la fonction.
    pub function: FunctionDefinition,
}

/// Le bloc `function` d'un `ToolDefinition`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionDefinition {
    /// Nom de la fonction = `qualified_id` de la capability.
    pub name: String,
    /// Description courte présentée au LLM.
    pub description: String,
    /// JSON Schema des paramètres (type=object).
    pub parameters: Value,
}

/// Appel d'outil reçu du LLM dans `choices[].message.tool_calls[]`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolCall {
    /// Identifiant attribué par le LLM (à renvoyer dans `tool_call_id`).
    pub id: String,
    /// Toujours `"function"`.
    #[serde(default = "default_function_type", rename = "type")]
    pub tool_type: String,
    /// Détail du call.
    pub function: FunctionCall,
}

fn default_function_type() -> String {
    "function".to_string()
}

/// Détail d'un appel : nom de la fonction + arguments JSON-stringifiés.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionCall {
    /// Nom = `qualified_id` (`service_id.intent`).
    pub name: String,
    /// Arguments JSON encodés en chaîne (format OpenAI).
    pub arguments: String,
}

/// Tool call entièrement résolu : capability + params parsés en `Value`.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedToolCall {
    /// `id` du tool_call (à renvoyer comme `tool_call_id` dans la réponse).
    pub id: String,
    /// Identifiant qualifié `service_id.intent`.
    pub qualified_id: String,
    /// `service_id` extrait du `qualified_id`.
    pub service_id: String,
    /// `intent` extrait du `qualified_id`.
    pub intent: String,
    /// Paramètres JSON parsés (object — peut être `null` si vide).
    pub params: Value,
}

// ----------------------------------------------------------------------
// Capability → Tool
// ----------------------------------------------------------------------

/// Convertit une `ServiceCapability` en `ToolDefinition` OpenAI-compatible.
#[must_use]
pub fn capability_to_tool(cap: &ServiceCapability) -> ToolDefinition {
    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();

    for (param_name, schema) in &cap.params {
        properties.insert(param_name.clone(), param_schema_to_json(schema));
        if schema.required {
            required.push(Value::String(param_name.clone()));
        }
    }

    let mut parameters = serde_json::Map::new();
    parameters.insert("type".into(), Value::String("object".into()));
    parameters.insert("properties".into(), Value::Object(properties));
    if !required.is_empty() {
        parameters.insert("required".into(), Value::Array(required));
    }

    let description = description_for(cap);

    ToolDefinition {
        tool_type: "function".into(),
        function: FunctionDefinition {
            name: cap.qualified_id(),
            description,
            parameters: Value::Object(parameters),
        },
    }
}

/// Convertit une liste de capabilities en liste de tools (pour `tools: [...]`
/// dans une requête chat/completions).
#[must_use]
pub fn capabilities_to_tools<'a, I>(capabilities: I) -> Vec<ToolDefinition>
where
    I: IntoIterator<Item = &'a ServiceCapability>,
{
    capabilities.into_iter().map(capability_to_tool).collect()
}

fn description_for(cap: &ServiceCapability) -> String {
    let base = cap.description.clone().unwrap_or_else(|| {
        format!(
            "Capacité '{}' du service '{}'.",
            cap.intent, cap.service_id
        )
    });
    if cap.phrases_examples.is_empty() {
        base
    } else {
        // Few-shot léger pour aider le LLM à reconnaître les déclencheurs.
        let examples = cap
            .phrases_examples
            .iter()
            .take(3)
            .map(|p| format!("« {p} »"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("{base}\nExemples : {examples}.")
    }
}

fn param_schema_to_json(schema: &ParamSchema) -> Value {
    let mut obj = serde_json::Map::new();

    match schema.param_type {
        ParamType::String => {
            obj.insert("type".into(), Value::String("string".into()));
        }
        ParamType::Int => {
            obj.insert("type".into(), Value::String("integer".into()));
        }
        ParamType::Float => {
            obj.insert("type".into(), Value::String("number".into()));
        }
        ParamType::Bool => {
            obj.insert("type".into(), Value::String("boolean".into()));
        }
        ParamType::Datetime => {
            obj.insert("type".into(), Value::String("string".into()));
            obj.insert("format".into(), Value::String("date-time".into()));
        }
        ParamType::Date => {
            obj.insert("type".into(), Value::String("string".into()));
            obj.insert("format".into(), Value::String("date".into()));
        }
        ParamType::DurationMinutes => {
            obj.insert("type".into(), Value::String("integer".into()));
            obj.insert("minimum".into(), json!(0));
            obj.insert(
                "x-format".into(),
                Value::String("duration_minutes".into()),
            );
        }
        ParamType::Enum => {
            obj.insert("type".into(), Value::String("string".into()));
            let values: Vec<Value> = schema
                .enum_values
                .iter()
                .map(|v| Value::String(v.clone()))
                .collect();
            obj.insert("enum".into(), Value::Array(values));
        }
    }

    if let Some(desc) = &schema.description {
        obj.insert("description".into(), Value::String(desc.clone()));
    }
    if let Some(default) = &schema.default {
        obj.insert("default".into(), default.clone());
    }

    Value::Object(obj)
}

// ----------------------------------------------------------------------
// Tool call (LLM) → ResolvedToolCall
// ----------------------------------------------------------------------

/// Parse un `ToolCall` brut reçu du LLM en `ResolvedToolCall`.
///
/// - Découpe `function.name` en `service_id.intent`
/// - Désérialise `function.arguments` (JSON string) en `Value`
/// - Renvoie `SkillsError::InvalidId` si le nom n'est pas pleinement qualifié
/// - Renvoie `SkillsError::Json` si les arguments ne sont pas du JSON valide
pub fn resolve_tool_call(call: &ToolCall) -> Result<ResolvedToolCall, SkillsError> {
    let name = &call.function.name;
    let (service_id, intent) = split_qualified_id(name)?;

    // OpenAI envoie `arguments` comme une string JSON. Vide ou whitespace =
    // `null`.
    let trimmed = call.function.arguments.trim();
    let params: Value = if trimmed.is_empty() {
        Value::Null
    } else {
        serde_json::from_str(trimmed)?
    };

    Ok(ResolvedToolCall {
        id: call.id.clone(),
        qualified_id: name.clone(),
        service_id,
        intent,
        params,
    })
}

/// Extrait `(service_id, intent)` depuis un `qualified_id` `service_id.intent`.
pub fn split_qualified_id(qualified: &str) -> Result<(String, String), SkillsError> {
    let mut parts = qualified.splitn(2, '.');
    match (parts.next(), parts.next()) {
        (Some(service), Some(intent)) if !service.is_empty() && !intent.is_empty() => {
            Ok((service.to_string(), intent.to_string()))
        }
        _ => Err(SkillsError::InvalidId(qualified.to_string())),
    }
}

/// Forge la « tool message » à pousser dans la conversation après exécution.
///
/// Format OpenAI :
/// ```json
/// { "role": "tool", "tool_call_id": "<id>", "content": "<json result>" }
/// ```
#[must_use]
pub fn build_tool_response(tool_call_id: &str, result: &Value) -> Value {
    json!({
        "role": "tool",
        "tool_call_id": tool_call_id,
        "content": result.to_string(),
    })
}

// ----------------------------------------------------------------------
// Tests
// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::{ConfirmationPolicy, ParamSchema, ParamType};
    use std::collections::BTreeMap;

    fn schedule_event_capability() -> ServiceCapability {
        let mut params = BTreeMap::new();
        params.insert(
            "title".into(),
            ParamSchema {
                param_type: ParamType::String,
                required: true,
                default: None,
                enum_values: Vec::new(),
                description: Some("Titre de l'événement".into()),
            },
        );
        params.insert(
            "start_at".into(),
            ParamSchema::required_of(ParamType::Datetime),
        );
        params.insert(
            "duration".into(),
            ParamSchema::optional_of(ParamType::DurationMinutes),
        );
        params.insert(
            "category".into(),
            ParamSchema {
                param_type: ParamType::Enum,
                required: false,
                default: None,
                enum_values: vec!["work".into(), "personal".into()],
                description: None,
            },
        );
        ServiceCapability {
            service_id: "jaykoa".into(),
            intent: "schedule_event".into(),
            phrases_examples: vec!["ajoute une réunion".into(), "planifie un RDV".into()],
            params,
            writes: true,
            confirmation_default: ConfirmationPolicy::AlwaysForWrites,
            description: Some("Programme un nouvel événement.".into()),
            category: Some("calendar".into()),
        }
    }

    #[test]
    fn capability_to_tool_uses_qualified_id_as_name() {
        let tool = capability_to_tool(&schedule_event_capability());
        assert_eq!(tool.tool_type, "function");
        assert_eq!(tool.function.name, "jaykoa.schedule_event");
    }

    #[test]
    fn capability_to_tool_includes_examples_in_description() {
        let tool = capability_to_tool(&schedule_event_capability());
        assert!(tool.function.description.contains("Programme"));
        assert!(tool.function.description.contains("ajoute une réunion"));
    }

    #[test]
    fn capability_to_tool_emits_object_schema() {
        let tool = capability_to_tool(&schedule_event_capability());
        let params = &tool.function.parameters;
        assert_eq!(params["type"], "object");
        assert!(params["properties"].is_object());
    }

    #[test]
    fn capability_to_tool_marks_required_params() {
        let tool = capability_to_tool(&schedule_event_capability());
        let required = tool.function.parameters["required"].as_array().unwrap();
        // Le BTreeMap garantit l'ordre alphabétique
        let required_strs: Vec<&str> = required.iter().filter_map(|v| v.as_str()).collect();
        assert!(required_strs.contains(&"title"));
        assert!(required_strs.contains(&"start_at"));
        assert!(!required_strs.contains(&"duration"));
    }

    #[test]
    fn capability_to_tool_maps_param_types() {
        let tool = capability_to_tool(&schedule_event_capability());
        let props = &tool.function.parameters["properties"];
        assert_eq!(props["title"]["type"], "string");
        assert_eq!(props["start_at"]["type"], "string");
        assert_eq!(props["start_at"]["format"], "date-time");
        assert_eq!(props["duration"]["type"], "integer");
        assert_eq!(props["category"]["type"], "string");
        assert_eq!(props["category"]["enum"][0], "work");
        assert_eq!(props["category"]["enum"][1], "personal");
    }

    #[test]
    fn capability_to_tool_serializes_to_openai_json() {
        let tool = capability_to_tool(&schedule_event_capability());
        let s = serde_json::to_string(&tool).unwrap();
        assert!(s.contains(r#""type":"function""#));
        assert!(s.contains(r#""name":"jaykoa.schedule_event""#));
        let _back: ToolDefinition = serde_json::from_str(&s).unwrap();
    }

    #[test]
    fn capabilities_to_tools_handles_empty() {
        let tools = capabilities_to_tools(std::iter::empty());
        assert_eq!(tools.len(), 0);
    }

    #[test]
    fn capabilities_to_tools_preserves_order() {
        let cap1 = schedule_event_capability();
        let mut cap2 = cap1.clone();
        cap2.intent = "delete_event".into();
        let caps = [cap1, cap2];
        let tools = capabilities_to_tools(caps.iter());
        assert_eq!(tools.len(), 2);
        assert_eq!(tools[0].function.name, "jaykoa.schedule_event");
        assert_eq!(tools[1].function.name, "jaykoa.delete_event");
    }

    #[test]
    fn split_qualified_id_works() {
        let (s, i) = split_qualified_id("jaykoa.schedule_event").unwrap();
        assert_eq!(s, "jaykoa");
        assert_eq!(i, "schedule_event");
    }

    #[test]
    fn split_qualified_id_rejects_missing_dot() {
        assert!(split_qualified_id("jaykoa").is_err());
        assert!(split_qualified_id("").is_err());
        assert!(split_qualified_id(".intent").is_err());
        assert!(split_qualified_id("service.").is_err());
    }

    #[test]
    fn resolve_tool_call_parses_arguments() {
        let call = ToolCall {
            id: "call_1".into(),
            tool_type: "function".into(),
            function: FunctionCall {
                name: "jaykoa.schedule_event".into(),
                arguments: r#"{"title":"Sync","start_at":"2026-05-08T10:00:00Z"}"#.into(),
            },
        };
        let r = resolve_tool_call(&call).unwrap();
        assert_eq!(r.id, "call_1");
        assert_eq!(r.qualified_id, "jaykoa.schedule_event");
        assert_eq!(r.service_id, "jaykoa");
        assert_eq!(r.intent, "schedule_event");
        assert_eq!(r.params["title"], "Sync");
    }

    #[test]
    fn resolve_tool_call_handles_empty_arguments() {
        let call = ToolCall {
            id: "call_2".into(),
            tool_type: "function".into(),
            function: FunctionCall {
                name: "jaykoa.list_today".into(),
                arguments: String::new(),
            },
        };
        let r = resolve_tool_call(&call).unwrap();
        assert_eq!(r.params, Value::Null);
    }

    #[test]
    fn resolve_tool_call_rejects_bad_json_arguments() {
        let call = ToolCall {
            id: "call_3".into(),
            tool_type: "function".into(),
            function: FunctionCall {
                name: "jaykoa.x".into(),
                arguments: "not json".into(),
            },
        };
        assert!(matches!(
            resolve_tool_call(&call),
            Err(SkillsError::InvalidArguments(_))
        ));
    }

    #[test]
    fn resolve_tool_call_rejects_unqualified_name() {
        let call = ToolCall {
            id: "call_4".into(),
            tool_type: "function".into(),
            function: FunctionCall {
                name: "schedule_event".into(),
                arguments: "{}".into(),
            },
        };
        assert!(matches!(
            resolve_tool_call(&call),
            Err(SkillsError::InvalidId(_))
        ));
    }

    #[test]
    fn build_tool_response_has_role_tool() {
        let r = build_tool_response("call_1", &json!({"ok": true, "id": "evt_42"}));
        assert_eq!(r["role"], "tool");
        assert_eq!(r["tool_call_id"], "call_1");
        // content est une string JSON
        let inner: Value = serde_json::from_str(r["content"].as_str().unwrap()).unwrap();
        assert_eq!(inner["ok"], true);
    }
}
