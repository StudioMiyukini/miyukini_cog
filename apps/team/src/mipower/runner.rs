//! Exécuteur d'étapes MIPOWER — dispatche vers les nœuds MAIA.
//!
//! Chaque étape = POST /v1/agents/{agent_id}/chat vers le nœud sélectionné.

use serde::{Deserialize, Serialize};

use crate::nodes::registry::NodeRegistry;
use crate::orchestrator::router::{select_node, NodeSelector};

/// Résultat de l'exécution d'une étape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub step_id: String,
    pub node_id: String,
    pub node_url: String,
    pub response_text: String,
    pub success: bool,
    pub error: Option<String>,
    pub elapsed_ms: u64,
}

/// Requête d'exécution d'une étape MIPOWER.
#[derive(Debug, Clone)]
pub struct StepExecRequest {
    pub step_id: String,
    pub agent_id: String,
    pub messages: Vec<serde_json::Value>,
    pub model_pattern: Option<String>,
    pub min_tier: Option<u8>,
    pub timeout_secs: u64,
}

/// Exécuteur d'étapes MIPOWER.
pub struct MiPowerRunner {
    client: reqwest::Client,
    registry: NodeRegistry,
}

impl MiPowerRunner {
    pub fn new(client: reqwest::Client, registry: NodeRegistry) -> Self {
        Self { client, registry }
    }

    /// Exécute une étape en sélectionnant le meilleur nœud.
    pub async fn execute_step(&self, req: StepExecRequest) -> StepResult {
        let start = std::time::Instant::now();

        let selector = NodeSelector {
            min_tier: req.min_tier,
            model_pattern: req.model_pattern.clone(),
            persona: None,
        };

        let node = match select_node(&self.registry, &selector) {
            Some(n) => n,
            None => {
                return StepResult {
                    step_id: req.step_id,
                    node_id: String::new(),
                    node_url: String::new(),
                    response_text: String::new(),
                    success: false,
                    error: Some("Aucun nœud disponible".into()),
                    elapsed_ms: start.elapsed().as_millis() as u64,
                };
            }
        };

        let url = format!(
            "{}/v1/agents/{}/chat",
            node.info.maia_url, req.agent_id
        );

        let body = serde_json::json!({
            "messages": req.messages,
            "stream": false,
        });

        let result = self
            .client
            .post(&url)
            .json(&body)
            .timeout(std::time::Duration::from_secs(req.timeout_secs))
            .send()
            .await;

        let elapsed_ms = start.elapsed().as_millis() as u64;

        match result {
            Ok(resp) if resp.status().is_success() => {
                let response_text = resp
                    .json::<serde_json::Value>()
                    .await
                    .ok()
                    .and_then(|v| {
                        v.get("choices")
                            .and_then(|c| c.get(0))
                            .and_then(|c| c.get("message"))
                            .and_then(|m| m.get("content"))
                            .and_then(|c| c.as_str())
                            .map(|s| s.to_string())
                    })
                    .unwrap_or_default();

                StepResult {
                    step_id: req.step_id,
                    node_id: node.info.id,
                    node_url: url,
                    response_text,
                    success: true,
                    error: None,
                    elapsed_ms,
                }
            }
            Ok(resp) => StepResult {
                step_id: req.step_id,
                node_id: node.info.id,
                node_url: url,
                response_text: String::new(),
                success: false,
                error: Some(format!("HTTP {}", resp.status())),
                elapsed_ms,
            },
            Err(e) => StepResult {
                step_id: req.step_id,
                node_id: node.info.id,
                node_url: url,
                response_text: String::new(),
                success: false,
                error: Some(e.to_string()),
                elapsed_ms,
            },
        }
    }
}
