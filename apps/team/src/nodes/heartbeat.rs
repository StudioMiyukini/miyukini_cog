//! @id     team_nodes_heartbeat
//! @do     definir_maia_health_response
//! @role   orchestration
//! @layer  app
//! @human  team — module « nodes heartbeat ».

//! Heartbeat — poll des nœuds MAIA toutes les N secondes.
//!
//! Pour chaque nœud Online dans le registre, envoie GET /maia/health.
//! Si la réponse est valide, met à jour l'entrée.
//! Si le nœud est injoignable depuis > stale_threshold, le marque Stale.
//!
//! Anti-SSRF : l'URL de chaque nœud est revalidée (`validate_node_url`) avant
//! **chaque** requête ; une URL devenue non conforme (changement de politique
//! ou de liste blanche) n'est pas interrogée et le nœud passe stale.

use std::sync::Arc;
use std::time::Duration;

use serde::Deserialize;

use super::registry::{NodeHealth, NodeRegistry};
use crate::config::TeamConfig;
use crate::mws::policy::validate_node_url;

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
    config: Arc<TeamConfig>,
) {
    let interval_secs = config.health_interval_secs;
    let stale_threshold_secs = config.stale_threshold_secs;
    let interval = Duration::from_secs(interval_secs);
    let stale_threshold = Duration::from_secs(stale_threshold_secs);

    loop {
        tokio::time::sleep(interval).await;
        poll_all_nodes(&registry, &client, &config, stale_threshold).await;
    }
}

/// Interroge tous les nœuds online et met à jour leur état.
async fn poll_all_nodes(
    registry: &NodeRegistry,
    client: &reqwest::Client,
    config: &TeamConfig,
    stale_threshold: Duration,
) {
    // Collecter les nœuds à interroger (snapshot pour éviter de tenir le lock)
    let nodes: Vec<(String, String)> = registry
        .healthy_nodes()
        .into_iter()
        .map(|n| (n.info.id, n.info.maia_url))
        .filter(|(id, url)| {
            match validate_node_url(url, config.isolation_policy, &config.node_allowed_hosts) {
                Ok(()) => true,
                Err(e) => {
                    tracing::warn!("Heartbeat ignoré pour {id} : URL refusée ({e})");
                    false
                }
            }
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::IsolationPolicy;
    use crate::nodes::registry::{NodeInfo, NodeMode};

    fn node(url: &str) -> NodeInfo {
        NodeInfo {
            id: "n1".into(),
            name: "n1".into(),
            maia_url: url.into(),
            version: "0".into(),
            mode: NodeMode::Pro,
            hardware_tier: 1,
            cpu: "cpu".into(),
            ram_mb: 1,
            gpu: None,
            model_active: None,
            persona: None,
        }
    }

    /// Lance un poll et indique si le nœud local a reçu une connexion.
    async fn poll_hits_listener(policy: IsolationPolicy) -> bool {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let registry = NodeRegistry::new();
        // Insertion directe : simule une URL devenue non conforme après enregistrement
        registry.register(node(&format!("http://{addr}")));
        let cfg = TeamConfig {
            isolation_policy: policy,
            ..TeamConfig::default()
        };
        let client = reqwest::Client::new();
        let accept = tokio::spawn(async move {
            tokio::time::timeout(Duration::from_millis(500), listener.accept())
                .await
                .is_ok()
        });
        poll_all_nodes(&registry, &client, &cfg, Duration::from_secs(60)).await;
        accept.await.unwrap()
    }

    #[tokio::test]
    async fn revalidation_avant_chaque_requete() {
        // Politique airgap : URL refusée → aucune requête sortante
        assert!(!poll_hits_listener(IsolationPolicy::Airgap).await);
        // Contrôle positif : loopback autorisé en internal_only
        assert!(poll_hits_listener(IsolationPolicy::InternalOnly).await);
    }
}
