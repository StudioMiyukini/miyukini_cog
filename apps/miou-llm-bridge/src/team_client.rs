//! @id     miou_llm_bridge_team_client
//! @do     definir_node_info
//! @role   orchestration
//! @layer  app
//! @human  miou-llm-bridge — module « team client ».

//! Client TEAM — enregistrement et heartbeat vers le hub TEAM (mode Pro uniquement).
//!
//! En mode Pro, MAIA :
//! 1. S'enregistre au démarrage via POST /team/register
//! 2. Envoie un heartbeat toutes les 30s via POST /team/heartbeat
//! 3. Se désenregistre proprement à l'arrêt via DELETE /team/nodes/:id
//!
//! Si le hub TEAM est injoignable au démarrage, MAIA continue en mode dégradé
//! et réessaie le prochain heartbeat.
//!
//! Authentification (lot 2, #7) : chaque requête est signée par la clé de service
//! de MAIA (usage [`USAGE_TEAM_NOEUD`]) ; le hub épingle sa clé publique
//! (`noeuds_autorises`) et lie le nœud enregistré à cette clé. Plus de secret
//! partagé (`TEAM_HMAC_SECRET` n'est plus lu).

use std::sync::Arc;

use miyusceau::CleSignature;
use serde::{Deserialize, Serialize};

use crate::hardware::HardwareInfo;
use crate::mode::MaiaMode;

/// Informations de nœud MAIA envoyées au hub TEAM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Identifiant unique du nœud (UUID généré au démarrage).
    pub id: String,
    /// Nom affiché (ex: "Alicia-tablette-salon").
    pub name: String,
    /// URL de l'API MAIA (ex: "http://192.168.1.42:11434").
    pub maia_url: String,
    /// Version du service.
    pub version: String,
    /// Mode de fonctionnement.
    pub mode: MaiaMode,
    /// Tier hardware.
    pub hardware_tier: u8,
    /// CPU du nœud.
    pub cpu: String,
    /// RAM totale (Mo).
    pub ram_mb: u64,
    /// Nom du GPU (None si CPU-only).
    pub gpu: Option<String>,
    /// Modèle LLM actif (None si aucun).
    pub model_active: Option<String>,
    /// Persona active (None si aucune).
    pub persona: Option<String>,
}

impl NodeInfo {
    /// Crée un NodeInfo à partir du hardware détecté.
    pub fn new(
        name: impl Into<String>,
        maia_url: impl Into<String>,
        hw: &HardwareInfo,
        mode: MaiaMode,
        model_active: Option<String>,
        persona: Option<String>,
    ) -> Self {
        Self {
            id: generate_node_id(),
            name: name.into(),
            maia_url: maia_url.into(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            mode,
            hardware_tier: hw.tier.number(),
            cpu: hw.cpu_name.clone(),
            ram_mb: hw.ram_total_mb,
            gpu: hw.gpu.as_ref().map(|g| g.name.clone()),
            model_active,
            persona,
        }
    }
}

/// Usage signé des requêtes d'un nœud MAIA vers le hub TEAM (côté hub : `team/noeud`).
pub const USAGE_TEAM_NOEUD: &str = "team/noeud";

/// Client TEAM (HTTP).
#[derive(Clone)]
pub struct TeamClient {
    client: reqwest::Client,
    team_url: String,
    node_info: NodeInfo,
    /// Clé de service de MAIA (signe chaque requête vers le hub).
    cle: Arc<CleSignature>,
}

impl TeamClient {
    pub fn new(client: reqwest::Client, team_url: String, node_info: NodeInfo, cle: Arc<CleSignature>) -> Self {
        Self {
            client,
            team_url,
            node_info,
            cle,
        }
    }

    /// Requête signée par la clé de MAIA ; `corps` JSON éventuel (exactement
    /// les octets envoyés).
    fn signee(&self, methode: reqwest::Method, url: &str, corps: Option<&serde_json::Value>) -> reqwest::RequestBuilder {
        let octets = corps.map(|c| serde_json::to_vec(c).unwrap_or_default()).unwrap_or_default();
        let mut rb = self.client.request(methode.clone(), url);
        if corps.is_some() {
            rb = rb.header(reqwest::header::CONTENT_TYPE, "application/json");
        }
        let rb = crate::signature::signer(rb, Some(&self.cle), USAGE_TEAM_NOEUD, methode.as_str(), url, &octets);
        if corps.is_some() {
            rb.body(octets)
        } else {
            rb
        }
    }

    /// Enregistre ce nœud auprès du hub TEAM.
    /// Retourne true si succès, false si le hub est injoignable.
    pub async fn register(&self) -> bool {
        let url = format!("{}/team/register", self.team_url);
        let corps = serde_json::to_value(&self.node_info).unwrap_or_default();
        match self
            .signee(reqwest::Method::POST, &url, Some(&corps))
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
        {
            Ok(r) if r.status().is_success() => {
                tracing::info!(
                    "Enregistré auprès du hub TEAM ({}) avec l'ID {}",
                    self.team_url,
                    self.node_info.id
                );
                true
            }
            Ok(r) => {
                tracing::warn!(
                    "Échec enregistrement TEAM : HTTP {}",
                    r.status()
                );
                false
            }
            Err(e) => {
                tracing::warn!("Hub TEAM injoignable ({}) : {e}", self.team_url);
                false
            }
        }
    }

    /// Envoie un heartbeat au hub TEAM.
    /// Met à jour le modèle actif et la persona.
    pub async fn heartbeat(&self, model_active: Option<String>, persona: Option<String>) {
        let url = format!("{}/team/heartbeat", self.team_url);
        let body = serde_json::json!({
            "id": self.node_info.id,
            "model_active": model_active,
            "persona": persona,
            "timestamp": current_epoch_secs(),
        });

        match self
            .signee(reqwest::Method::POST, &url, Some(&body))
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            Ok(r) if r.status().is_success() => {
                tracing::debug!("Heartbeat TEAM OK");
            }
            Ok(r) => {
                tracing::warn!("Heartbeat TEAM échoué : HTTP {}", r.status());
            }
            Err(e) => {
                tracing::debug!("Heartbeat TEAM injoignable : {e}");
            }
        }
    }

    /// Démarre la tâche de heartbeat périodique (toutes les 30s).
    /// Retourne un handle que l'on peut abort() à l'arrêt.
    pub fn start_heartbeat_task(
        self,
        model_fn: impl Fn() -> Option<String> + Send + Sync + 'static,
        persona_fn: impl Fn() -> Option<String> + Send + Sync + 'static,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let interval = std::time::Duration::from_secs(30);
            loop {
                tokio::time::sleep(interval).await;
                let model = model_fn();
                let persona = persona_fn();
                self.heartbeat(model, persona).await;
            }
        })
    }

    /// Désenregistre ce nœud proprement.
    pub async fn unregister(&self) {
        let url = format!("{}/team/nodes/{}", self.team_url, self.node_info.id);
        match self
            .signee(reqwest::Method::DELETE, &url, None)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            Ok(_) => tracing::info!("Nœud désenregistré du hub TEAM"),
            Err(e) => tracing::warn!("Désenregistrement TEAM échoué : {e}"),
        }
    }

    pub fn node_id(&self) -> &str {
        &self.node_info.id
    }

    pub fn team_url(&self) -> &str {
        &self.team_url
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────

/// Génère un identifiant de nœud unique (basé sur hostname + timestamp).
fn generate_node_id() -> String {
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".into());
    let ts = current_epoch_secs();
    format!("maia-{hostname}-{ts}")
}

fn current_epoch_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Aller-retour avec un faux hub : la requête porte une signature valide pour
    /// la clé de MAIA, l'usage `team/noeud`, le chemin et le corps envoyés.
    #[tokio::test]
    async fn requetes_signees_verifiables_par_le_hub() {
        use axum::{extract::Request, routing::post, Router};
        use miyauth::http::service::GardeSignee;

        let cle = Arc::new(CleSignature::generer());
        let garde = Arc::new(GardeSignee::new([cle.cle_publique()]));
        let garde_srv = garde.clone();
        let app = Router::new().route(
            "/team/register",
            post(move |req: Request| {
                let garde_srv = garde_srv.clone();
                async move {
                    let entetes_http = req.headers().clone();
                    let v = |nom: &str| entetes_http.get(nom).and_then(|x| x.to_str().ok());
                    let instant = miyauth::http::service::maintenant_unix();
                    let entetes = garde_srv
                        .controler_entetes(
                            v(miyauth::http::ENTETE_SIGNATAIRE),
                            v(miyauth::http::ENTETE_HORODATAGE),
                            v(miyauth::http::ENTETE_NONCE),
                            v(miyauth::http::ENTETE_SIGNATURE),
                            instant,
                        )
                        .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
                    let corps = axum::body::to_bytes(req.into_body(), 1 << 20).await.unwrap();
                    garde_srv.verifier(&entetes, USAGE_TEAM_NOEUD, "POST", "/team/register", &corps, instant)
                        .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
                    Ok::<_, axum::http::StatusCode>("ok")
                }
            }),
        );
        let ecoute = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let adresse = ecoute.local_addr().unwrap();
        tokio::spawn(async move {
            let _ = axum::serve(ecoute, app).await;
        });
        let hw = crate::hardware::detect_hardware();
        let noeud = NodeInfo::new("n", "http://127.0.0.1:1", &hw, MaiaMode::Pro, None, None);
        let url = format!("http://{adresse}");
        let client = TeamClient::new(reqwest::Client::new(), url.clone(), noeud.clone(), cle);
        assert!(client.register().await);
        // Une autre clé (non épinglée) est refusée par le même hub.
        let intrus = TeamClient::new(reqwest::Client::new(), url, noeud, Arc::new(CleSignature::generer()));
        assert!(!intrus.register().await);
    }
}
