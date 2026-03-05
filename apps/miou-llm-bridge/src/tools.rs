//! Tool calling — conversion skills → OpenAI function calling + boucle d'exécution.
//!
//! Quand un agent chatte avec un LLM qui supporte le tool calling,
//! ce module convertit les skills autorisés en tools OpenAI,
//! gère la boucle appel→exécution→renvoi, et retourne la réponse finale.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::agents::AgentDef;
use crate::security::{self, AuditEntry, AuditLog};
use crate::skills::{SkillInfo, SkillRegistry};

/// Nombre maximum d'itérations de la boucle tool calling.
pub const MAX_TOOL_ITERATIONS: usize = 5;

// ── Types OpenAI ─────────────────────────────────────────────────────

/// Définition d'un outil au format OpenAI.
#[derive(Debug, Clone, Serialize)]
pub struct ToolDefinition {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: FunctionDefinition,
}

#[derive(Debug, Clone, Serialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// Appel d'outil reçu du LLM.
#[derive(Debug, Clone, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub function: FunctionCall,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    /// Arguments JSON sous forme de chaîne (à parser).
    pub arguments: String,
}

// ── Types résultat ───────────────────────────────────────────────────

/// Résultat final de la boucle tool calling.
#[derive(Debug)]
pub struct ToolChatResult {
    /// Contenu textuel final du LLM.
    pub content: String,
    /// Log de tous les appels d'outils effectués.
    pub tool_calls_log: Vec<ToolCallLogEntry>,
    /// Nombre d'itérations de la boucle.
    pub iterations: usize,
}

/// Log d'un appel d'outil individuel.
#[derive(Debug, Clone, Serialize)]
pub struct ToolCallLogEntry {
    pub skill_id: String,
    pub arguments: serde_json::Value,
    pub result_success: bool,
    pub duration_ms: u64,
}

/// Erreurs possibles de la boucle tool calling.
#[derive(Debug)]
pub enum ToolChatError {
    /// Erreur de communication avec le upstream LLM.
    UpstreamError(String),
    /// Limite d'itérations atteinte sans réponse finale.
    MaxIterationsExceeded,
    /// Réponse du LLM non parseable.
    ParseError(String),
}

// ── Conversion Skill → Tool ─────────────────────────────────────────

/// Convertit un `SkillInfo` en `ToolDefinition` OpenAI avec JSON Schema.
pub fn skill_to_tool(skill: &SkillInfo) -> ToolDefinition {
    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();

    for (param_name, param_desc) in &skill.parameters {
        let mut prop = serde_json::Map::new();
        prop.insert("type".into(), serde_json::Value::String("string".into()));
        prop.insert(
            "description".into(),
            serde_json::Value::String(param_desc.clone()),
        );

        // Si la description contient des valeurs séparées par " | ", générer un enum
        if param_desc.contains(" | ") {
            // Prendre la partie avant une éventuelle parenthèse
            let prefix = param_desc.split('(').next().unwrap_or(param_desc);
            let enum_values: Vec<serde_json::Value> = prefix
                .split('|')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty() && !s.contains(' '))
                .map(|s| serde_json::Value::String(s.to_string()))
                .collect();

            if !enum_values.is_empty() {
                prop.insert("enum".into(), serde_json::Value::Array(enum_values));
            }
        }

        // Heuristique : certains paramètres sont requis
        if matches!(param_name.as_str(), "action" | "command" | "url") {
            required.push(serde_json::Value::String(param_name.clone()));
        }

        properties.insert(param_name.clone(), serde_json::Value::Object(prop));
    }

    let schema = serde_json::json!({
        "type": "object",
        "properties": properties,
        "required": required,
    });

    ToolDefinition {
        tool_type: "function".into(),
        function: FunctionDefinition {
            name: skill.id.clone(),
            description: skill.description.clone(),
            parameters: schema,
        },
    }
}

/// Construit la liste de tools OpenAI pour un agent donné.
/// Ne retient que les skills pour lesquels l'agent a la permission.
pub fn agent_tools(agent: &AgentDef) -> Vec<ToolDefinition> {
    let all_skills = SkillRegistry::list_skills();

    all_skills
        .into_iter()
        .filter(|skill| {
            matches!(
                security::check_agent_skill_permission(agent, &skill.id),
                security::PermissionCheck::Allowed
            )
        })
        .map(|skill| skill_to_tool(&skill))
        .collect()
}

// ── Parsing réponse LLM ─────────────────────────────────────────────

/// Extrait les tool_calls d'une réponse LLM.
/// Retourne `None` si c'est une réponse texte normale (pas de tool_calls).
pub fn extract_tool_calls(body: &serde_json::Value) -> Option<Vec<ToolCall>> {
    let tool_calls = body
        .get("choices")?
        .get(0)?
        .get("message")?
        .get("tool_calls")?;

    let arr = tool_calls.as_array()?;
    if arr.is_empty() {
        return None;
    }

    let parsed: Vec<ToolCall> = arr
        .iter()
        .filter_map(|tc| serde_json::from_value(tc.clone()).ok())
        .collect();

    if parsed.is_empty() {
        None
    } else {
        Some(parsed)
    }
}

// ── Exécution d'un appel outil ──────────────────────────────────────

/// Exécute un appel d'outil unique.
/// Vérifie les permissions, parse les arguments, exécute le skill, log dans l'audit.
/// Retourne un message JSON prêt à être inséré dans la conversation (role: "tool").
pub async fn execute_tool_call(
    tool_call: &ToolCall,
    agent: &AgentDef,
    audit_log: &AuditLog,
) -> (serde_json::Value, ToolCallLogEntry) {
    let skill_id = &tool_call.function.name;

    // Vérification des permissions
    match security::check_agent_skill_permission(agent, skill_id) {
        security::PermissionCheck::Denied(reason) => {
            let msg = serde_json::json!({
                "role": "tool",
                "tool_call_id": tool_call.id,
                "content": serde_json::json!({
                    "success": false,
                    "error": reason,
                }).to_string(),
            });
            let log_entry = ToolCallLogEntry {
                skill_id: skill_id.clone(),
                arguments: serde_json::Value::Null,
                result_success: false,
                duration_ms: 0,
            };
            return (msg, log_entry);
        }
        security::PermissionCheck::Allowed => {}
    }

    // Parser les arguments JSON
    let params: HashMap<String, serde_json::Value> =
        serde_json::from_str(&tool_call.function.arguments).unwrap_or_default();

    let args_value: serde_json::Value =
        serde_json::from_str(&tool_call.function.arguments).unwrap_or(serde_json::Value::Null);

    // Exécuter le skill
    let result = SkillRegistry::execute(skill_id, &params).await;

    // Audit
    audit_log.log(AuditEntry {
        timestamp: security::epoch_now(),
        agent_id: agent.id.clone(),
        skill_id: skill_id.clone(),
        action: params
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("tool_call")
            .into(),
        details: tool_call.function.arguments.clone(),
        success: result.success,
        client_id: "tool_calling".into(),
    });

    let log_entry = ToolCallLogEntry {
        skill_id: skill_id.clone(),
        arguments: args_value,
        result_success: result.success,
        duration_ms: result.duration_ms,
    };

    let content = serde_json::to_string(&result).unwrap_or_else(|_| "{}".into());

    let msg = serde_json::json!({
        "role": "tool",
        "tool_call_id": tool_call.id,
        "content": content,
    });

    (msg, log_entry)
}

// ── Boucle principale ───────────────────────────────────────────────

/// Boucle de tool calling : envoie la requête via InferenceRouter, si tool_calls → exécute → renvoie.
/// Itère jusqu'à obtenir une réponse textuelle finale ou atteindre MAX_TOOL_ITERATIONS.
pub async fn chat_with_tools(
    inference: &crate::inference::InferenceRouter,
    model: &str,
    mut messages: Vec<serde_json::Value>,
    tools: Vec<ToolDefinition>,
    agent: &AgentDef,
    audit_log: &AuditLog,
) -> Result<ToolChatResult, ToolChatError> {
    let mut tool_calls_log = Vec::new();

    for iteration in 0..MAX_TOOL_ITERATIONS {
        // Construire le body de la requête
        let mut body = serde_json::json!({
            "model": model,
            "messages": messages,
            "temperature": 0.7,
            "max_tokens": 2048,
        });

        if !tools.is_empty() {
            body["tools"] = serde_json::to_value(&tools)
                .map_err(|e| ToolChatError::ParseError(e.to_string()))?;
            body["tool_choice"] = serde_json::Value::String("auto".into());
        }

        // Appel via InferenceRouter (natif d'abord, upstream en backup)
        let response = inference
            .chat_completion(body)
            .await
            .map_err(|e| ToolChatError::UpstreamError(format!("{e}")))?;

        // Reconstruire le response_body pour extract_tool_calls
        let response_body = serde_json::json!({
            "choices": response.choices.iter().map(|c| {
                serde_json::json!({
                    "message": c.message,
                    "finish_reason": c.finish_reason,
                })
            }).collect::<Vec<_>>(),
        });

        // Vérifier si la réponse contient des tool_calls
        match extract_tool_calls(&response_body) {
            Some(tool_calls) => {
                // Ajouter le message assistant (avec tool_calls) à la conversation
                if let Some(assistant_msg) = response_body
                    .get("choices")
                    .and_then(|c| c.get(0))
                    .and_then(|c| c.get("message"))
                {
                    messages.push(assistant_msg.clone());
                }

                // Exécuter chaque tool call
                for tc in &tool_calls {
                    let (tool_msg, log_entry) = execute_tool_call(tc, agent, audit_log).await;
                    tool_calls_log.push(log_entry);
                    messages.push(tool_msg);
                }

                tracing::info!(
                    "Agent '{}' — iteration {}, {} tool calls exécutés",
                    agent.id,
                    iteration + 1,
                    tool_calls.len()
                );
            }
            None => {
                // Réponse textuelle normale — extraire le contenu
                let content = response
                    .choices
                    .first()
                    .and_then(|c| c.message.get("content"))
                    .and_then(|c| c.as_str())
                    .unwrap_or("Pas de réponse")
                    .to_string();

                return Ok(ToolChatResult {
                    content,
                    tool_calls_log,
                    iterations: iteration + 1,
                });
            }
        }
    }

    // Limite atteinte
    Err(ToolChatError::MaxIterationsExceeded)
}
