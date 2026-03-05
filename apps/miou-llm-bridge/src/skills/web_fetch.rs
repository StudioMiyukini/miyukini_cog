//! Skill : Recherche et récupération web.
//!
//! Permet aux agents de récupérer du contenu web (pages, API publiques).
//! Restrictions : timeout, taille max, blocklist de domaines.

use super::{SkillInfo, SkillResult};
use std::collections::HashMap;

/// Timeout par défaut (secondes).
const DEFAULT_TIMEOUT_SECS: u64 = 15;
/// Taille max de réponse (512 KB).
const MAX_RESPONSE_SIZE: usize = 524_288;

/// Domaines bloqués (sécurité).
const BLOCKED_DOMAINS: &[&str] = &[
    "localhost",
    "127.0.0.1",
    "0.0.0.0",
    "169.254.", // Link-local
    "10.",      // Réseau privé
    "192.168.", // Réseau privé
    "172.16.",  // Réseau privé
];

/// Métadonnées du skill.
pub fn info() -> SkillInfo {
    let mut params = HashMap::new();
    params.insert("url".into(), "URL à récupérer (obligatoire)".into());
    params.insert("method".into(), "Méthode HTTP (défaut: GET)".into());
    params.insert(
        "headers".into(),
        "Headers additionnels (objet clé-valeur)".into(),
    );
    params.insert("body".into(), "Corps de la requête (pour POST/PUT)".into());
    params.insert(
        "timeout".into(),
        format!("Timeout en secondes (défaut {DEFAULT_TIMEOUT_SECS})"),
    );

    SkillInfo {
        id: "web_fetch".into(),
        name: "Recherche Web".into(),
        description:
            "Récupère du contenu web (pages, API publiques). Restrictions de sécurité appliquées."
                .into(),
        parameters: params,
        min_security_level: "sandboxed",
    }
}

/// Exécute une requête web.
pub async fn execute(params: &HashMap<String, serde_json::Value>) -> SkillResult {
    let url = match params.get("url").and_then(|v| v.as_str()) {
        Some(u) => u,
        None => {
            return SkillResult {
                success: false,
                output: serde_json::Value::Null,
                error: Some("Paramètre 'url' requis".into()),
                duration_ms: 0,
            };
        }
    };

    // Vérifier les domaines bloqués
    let url_lower = url.to_lowercase();
    for blocked in BLOCKED_DOMAINS {
        if url_lower.contains(blocked) {
            return SkillResult {
                success: false,
                output: serde_json::Value::Null,
                error: Some(format!(
                    "Domaine bloqué par la politique de sécurité : {url}"
                )),
                duration_ms: 0,
            };
        }
    }

    // Vérifier que c'est HTTP(S)
    if !url_lower.starts_with("http://") && !url_lower.starts_with("https://") {
        return SkillResult {
            success: false,
            output: serde_json::Value::Null,
            error: Some("Seuls les protocoles HTTP et HTTPS sont autorisés".into()),
            duration_ms: 0,
        };
    }

    let timeout_secs = params
        .get("timeout")
        .and_then(|v| v.as_u64())
        .unwrap_or(DEFAULT_TIMEOUT_SECS);

    let method = params
        .get("method")
        .and_then(|v| v.as_str())
        .unwrap_or("GET")
        .to_uppercase();

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(timeout_secs))
        .user_agent("MiyukiniAIStudio/0.1")
        .build()
        .unwrap_or_default();

    let mut req = match method.as_str() {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        "HEAD" => client.head(url),
        _ => {
            return SkillResult {
                success: false,
                output: serde_json::Value::Null,
                error: Some(format!("Méthode HTTP inconnue : {method}")),
                duration_ms: 0,
            };
        }
    };

    // Headers additionnels
    if let Some(headers) = params.get("headers").and_then(|v| v.as_object()) {
        for (key, value) in headers {
            if let Some(val) = value.as_str() {
                req = req.header(key.as_str(), val);
            }
        }
    }

    // Body (POST/PUT)
    if let Some(body) = params.get("body") {
        if method == "POST" || method == "PUT" {
            req = req.json(body);
        }
    }

    match req.send().await {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let headers: HashMap<String, String> = resp
                .headers()
                .iter()
                .filter_map(|(k, v)| v.to_str().ok().map(|val| (k.to_string(), val.to_string())))
                .collect();

            let content_type = headers
                .get("content-type")
                .map(|s| s.as_str())
                .unwrap_or("unknown");

            // Lire le body avec limite de taille
            match resp.bytes().await {
                Ok(bytes) => {
                    let mut body_str = if bytes.len() > MAX_RESPONSE_SIZE {
                        let truncated = &bytes[..MAX_RESPONSE_SIZE];
                        let mut s = String::from_utf8_lossy(truncated).to_string();
                        s.push_str("\n... [tronqué]");
                        s
                    } else {
                        String::from_utf8_lossy(&bytes).to_string()
                    };

                    // Si c'est du JSON, parser pour un affichage propre
                    let output = if content_type.contains("json") {
                        if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&body_str) {
                            serde_json::json!({
                                "status": status,
                                "content_type": content_type,
                                "body": json_val,
                                "size": bytes.len(),
                            })
                        } else {
                            serde_json::json!({
                                "status": status,
                                "content_type": content_type,
                                "body": body_str,
                                "size": bytes.len(),
                            })
                        }
                    } else {
                        // Tronquer le texte brut pour économiser les tokens
                        if body_str.len() > 4096 {
                            body_str.truncate(4096);
                            body_str.push_str("\n... [tronqué à 4KB pour économiser les tokens]");
                        }
                        serde_json::json!({
                            "status": status,
                            "content_type": content_type,
                            "body": body_str,
                            "size": bytes.len(),
                        })
                    };

                    SkillResult {
                        success: (200..300).contains(&(status as usize)),
                        output,
                        error: if status >= 400 {
                            Some(format!("HTTP {status}"))
                        } else {
                            None
                        },
                        duration_ms: 0,
                    }
                }
                Err(e) => SkillResult {
                    success: false,
                    output: serde_json::json!({ "status": status }),
                    error: Some(format!("Erreur lecture body : {e}")),
                    duration_ms: 0,
                },
            }
        }
        Err(e) => SkillResult {
            success: false,
            output: serde_json::Value::Null,
            error: Some(format!("Erreur réseau : {e}")),
            duration_ms: 0,
        },
    }
}
