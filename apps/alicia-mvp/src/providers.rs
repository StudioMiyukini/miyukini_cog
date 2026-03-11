//! Multi-provider LLM avec health check et fallback.
//!
//! Chaque provider est un endpoint OpenAI-compatible (LM Studio, Ollama, etc.)
//! Le router essaie les providers par priorite et bascule sur le suivant en cas d'echec.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use tokio::sync::RwLock;

/// Definition d'un provider LLM.
#[derive(Debug, Clone)]
pub struct LlmProvider {
    pub name: String,
    pub url: String,
    pub key: Option<String>,
    pub model: String,
    pub priority: u8,
    pub timeout_secs: u64,
}

/// Cache de health check.
struct HealthEntry {
    alive: bool,
    checked_at: Instant,
}

/// Router multi-provider avec fallback.
pub struct LlmRouter {
    providers: Vec<LlmProvider>,
    client: reqwest::Client,
    health: Arc<RwLock<HashMap<String, HealthEntry>>>,
}

impl LlmRouter {
    /// Cree un router a partir d'une liste de providers (trie par priorite).
    pub fn new(mut providers: Vec<LlmProvider>) -> Self {
        providers.sort_by_key(|p| p.priority);
        println!("  [LLM Router] {} provider(s) configures:", providers.len());
        for p in &providers {
            let key_hint = if p.key.is_some() { " (auth)" } else { "" };
            println!("    [{}] {} — {} (prio={}){}",
                p.priority, p.name, p.model, p.priority, key_hint);
        }
        Self {
            providers,
            client: reqwest::Client::new(),
            health: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Cree un router avec un seul provider (shorthand pour --llm-url).
    pub fn single(url: String, key: Option<String>, model: Option<String>) -> Self {
        let model = model.unwrap_or_else(|| "qwen/qwen3-vl-4b".to_string());
        Self::new(vec![LlmProvider {
            name: "default".to_string(),
            url,
            key,
            model,
            priority: 0,
            timeout_secs: 30,
        }])
    }

    /// Envoie une requete chat completion au premier provider disponible.
    /// Retourne (reponse JSON, nom du provider, modele utilise).
    pub async fn chat_completion(
        &self,
        mut body: serde_json::Value,
    ) -> anyhow::Result<(serde_json::Value, String, String)> {
        let mut last_error = String::new();

        for provider in &self.providers {
            // Verifier le cache de health (TTL 60s)
            if !self.is_healthy(provider).await {
                continue;
            }

            // Injecter le modele du provider
            body["model"] = serde_json::Value::String(provider.model.clone());

            let url = format!("{}/v1/chat/completions", provider.url);
            let mut req = self
                .client
                .post(&url)
                .json(&body)
                .timeout(std::time::Duration::from_secs(provider.timeout_secs));

            if let Some(key) = &provider.key {
                req = req.bearer_auth(key);
            }

            match req.send().await {
                Ok(resp) if resp.status().is_success() => {
                    match resp.json::<serde_json::Value>().await {
                        Ok(json) => {
                            self.mark_healthy(&provider.name, true).await;
                            return Ok((json, provider.name.clone(), provider.model.clone()));
                        }
                        Err(e) => {
                            last_error = format!("{}: parse error: {e}", provider.name);
                            self.mark_healthy(&provider.name, false).await;
                        }
                    }
                }
                Ok(resp) => {
                    let status = resp.status();
                    let body_text = resp.text().await.unwrap_or_default();
                    last_error = format!("{}: HTTP {} — {}", provider.name, status, &body_text[..body_text.len().min(200)]);
                    // Ne pas marquer comme down pour 4xx (erreur client)
                    if status.is_server_error() {
                        self.mark_healthy(&provider.name, false).await;
                    }
                }
                Err(e) => {
                    last_error = format!("{}: {e}", provider.name);
                    self.mark_healthy(&provider.name, false).await;
                }
            }

            println!("  [LLM Router] {} echoue, essai suivant...", provider.name);
        }

        anyhow::bail!("Tous les providers LLM ont echoue. Derniere erreur: {last_error}")
    }

    /// Health check : GET /v1/models avec timeout 3s.
    async fn is_healthy(&self, provider: &LlmProvider) -> bool {
        // Verifier le cache (TTL 60s)
        {
            let cache = self.health.read().await;
            if let Some(entry) = cache.get(&provider.name) {
                if entry.checked_at.elapsed().as_secs() < 60 {
                    return entry.alive;
                }
            }
        }

        // Pas en cache ou expire — faire un health check
        let url = format!("{}/v1/models", provider.url);
        let mut req = self
            .client
            .get(&url)
            .timeout(std::time::Duration::from_secs(3));
        if let Some(key) = &provider.key {
            req = req.bearer_auth(key);
        }

        let alive = match req.send().await {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        };

        self.mark_healthy(&provider.name, alive).await;
        if !alive {
            println!("  [LLM Router] {} injoignable, skip", provider.name);
        }
        alive
    }

    async fn mark_healthy(&self, name: &str, alive: bool) {
        let mut cache = self.health.write().await;
        cache.insert(
            name.to_string(),
            HealthEntry {
                alive,
                checked_at: Instant::now(),
            },
        );
    }

    /// Liste les providers configures.
    pub fn providers(&self) -> &[LlmProvider] {
        &self.providers
    }
}

/// Parse un provider depuis le format CLI : "name|url|model|priority|key"
pub fn parse_provider_string(s: &str) -> anyhow::Result<LlmProvider> {
    let parts: Vec<&str> = s.split('|').collect();
    if parts.len() < 3 {
        anyhow::bail!("Format: name|url|model[|priority][|key]");
    }

    Ok(LlmProvider {
        name: parts[0].to_string(),
        url: parts[1].to_string(),
        model: parts[2].to_string(),
        priority: parts.get(3).and_then(|s| s.parse().ok()).unwrap_or(0),
        key: parts.get(4).map(|s| s.to_string()).filter(|s| !s.is_empty()),
        timeout_secs: 30,
    })
}
