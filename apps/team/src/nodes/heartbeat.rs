//! Heartbeat — poll des nœuds MAIA toutes les N secondes.
//!
//! Pour chaque nœud Online dans le registre, envoie GET /maia/health.
//! Si la réponse est valide, met à jour l'entrée.
//! Si le nœud est injoignable depuis > stale_threshold, le marque Stale.

use std::time::Duration;

use serde::Deserialize;

use super::registry::{NodeHealth, NodeRegistry};

/// Réponse partielle de GET /maia/health (champs utilisés par TEAM).
#[derive(Debug, Deserialize)]
struct MaiaHealthResponse {
    llm: Option<LlmStatus>,
    hardware_tier: Option<String>,
    uptime_secs: Option<u64>,
    capabilities: Option<CapabilitiesStatus>,
}

#[derive(Debug, Deserialize)]
struct LlmStatus {
    model: Option<String>,
    backend: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CapabilitiesStatus {
    agents: Option<usize>,
    tts: Option<bool>,
}

/// Lance la tâche de heartbeat périodique.
///
/// Tourne indéfiniment — à spawner dans tokio.
pub async fn run_heartbeat_loop(
    registry: NodeRegistry,
    client: reqwest::Client,
    interval_secs: u64,
    stale_threshold_secs: u64,
) {
    let interval = Duration::from_secs(interval_secs);
    let stale_threshold = Duration::from_secs(stale_threshold_secs);

    loop {
        tokio::time::sleep(interval).await;
        poll_all_nodes(&registry, &client, stale_threshold).await;
    }
}

/// Interroge tous les nœuds online et met à jour leur état.
async fn poll_all_nodes(
    registry: &NodeRegistry,
    client: &reqwest::Client,
    stale_threshold: Duration,
) {
    // Collecter les nœuds à interroger (snapshot pour éviter de tenir le lock)
    let nodes: Vec<(String, String)> = registry
        .healthy_nodes()
        .into_iter()
        .map(|n| (n.info.id, n.info.maia_url))
        .collect();

    // Interroger chaque nœud en parallèle
    let tasks: Vec<_> = nodes
        .into_iter()
        .map(|(id, url)| {
            let client = client.clone();
            let registry = registry.clone();
            tokio::spawn(async move {
                poll_node(&id, &url, &client, &registry).await;
            })
        })
        .collect();

    for task in tasks {
        let _ = task.await;
    }

    // Marquer les nœuds qui n'ont pas répondu depuis trop longtemps
    registry.mark_stale_nodes(stale_threshold);
}

/// Interroge un nœud unique et met à jour le registre.
async fn poll_node(id: &str, maia_url: &str, client: &reqwest::Client, registry: &NodeRegistry) {
    let url = format!("{maia_url}/maia/health");
    match client
        .get(&url)
        .timeout(Duration::from_secs(5))
        .send()
        .await
    {
        Ok(resp) if resp.status().is_success() => {
            if let Ok(body) = resp.json::<MaiaHealthResponse>().await {
                let model = body.llm.as_ref().and_then(|l| l.model.clone());
                let backend = body.llm.as_ref().and_then(|l| l.backend.clone());

                let health = NodeHealth {
                    model: model.clone(),
                    backend,
                    hardware_tier: body.hardware_tier.unwrap_or_default(),
                    uptime_secs: body.uptime_secs.unwrap_or(0),
                    capabilities_agents: body
                        .capabilities
                        .as_ref()
                        .and_then(|c| c.agents)
                        .unwrap_or(0),
                    capabilities_tts: body
                        .capabilities
                        .as_ref()
                        .and_then(|c| c.tts)
                        .unwrap_or(false),
                };

                registry.heartbeat(id, health, model, None);
                tracing::debug!("Heartbeat OK : {id}");
            }
        }
        Ok(resp) => {
            tracing::warn!("Heartbeat HTTP {} : {id}", resp.status());
        }
        Err(e) => {
            tracing::debug!("Heartbeat timeout/err {id} : {e}");
        }
    }
}
