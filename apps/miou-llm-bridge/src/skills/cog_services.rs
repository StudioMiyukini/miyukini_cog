//! Skill : Interaction avec les services COG.
//!
//! Permet aux agents d'interagir avec les services Miyukini installés
//! (JayKonta, JayKoa, MiyukiniWatch, etc.) via leurs API locales.

use super::{SkillInfo, SkillResult};
use std::collections::HashMap;

/// Port de base des services COG (convention Miyukini).
const COG_BASE_PORT: u16 = 11400;

/// Services COG connus et leurs ports par défaut.
const KNOWN_SERVICES: &[(&str, &str, u16)] = &[
    ("jaykonta", "JayKonta — Comptabilité", 11441),
    ("jaykoa", "JayKoa — Calendrier", 11442),
    ("jayxpose", "JayXpose — Vitrine", 11443),
    ("jayfestival", "JayFestival — Événements", 11444),
    ("miyukiniwatch", "MiyukiniWatch — Habitudes & Mesures", 11445),
    ("jay1tribu", "Jay1Tribu — Social & Chat", 11446),
    ("jaymanga", "JayManga — Manga", 11447),
    ("miyuclicker", "Lord of the Click — Jeu", 11448),
];

/// Métadonnées du skill.
pub fn info() -> SkillInfo {
    let mut params = HashMap::new();
    params.insert("action".into(), "list | call | status".into());
    params.insert("service".into(), "ID du service COG (ex: jaykonta)".into());
    params.insert("endpoint".into(), "Endpoint API du service (ex: /api/balance)".into());
    params.insert("method".into(), "Méthode HTTP (défaut: GET)".into());
    params.insert("body".into(), "Corps de la requête JSON".into());

    SkillInfo {
        id: "cog_services".into(),
        name: "Services COG".into(),
        description: "Interagir avec les services Miyukini COG installés (JayKonta, JayKoa, MiyukiniWatch, etc.).".into(),
        parameters: params,
        min_security_level: "sandboxed",
    }
}

/// Exécute une interaction avec un service COG.
pub async fn execute(params: &HashMap<String, serde_json::Value>) -> SkillResult {
    let action = params
        .get("action")
        .and_then(|v| v.as_str())
        .unwrap_or("list");

    match action {
        "list" => list_services().await,
        "status" => {
            let service = params.get("service").and_then(|v| v.as_str()).unwrap_or("");
            check_service_status(service).await
        }
        "call" => {
            let service = params.get("service").and_then(|v| v.as_str()).unwrap_or("");
            let endpoint = params.get("endpoint").and_then(|v| v.as_str()).unwrap_or("/health");
            let method = params.get("method").and_then(|v| v.as_str()).unwrap_or("GET");
            let body = params.get("body");
            call_service(service, endpoint, method, body).await
        }
        _ => SkillResult {
            success: false,
            output: serde_json::Value::Null,
            error: Some(format!("Action inconnue : {action}. Actions : list, status, call")),
            duration_ms: 0,
        },
    }
}

async fn list_services() -> SkillResult {
    let services: Vec<serde_json::Value> = KNOWN_SERVICES
        .iter()
        .map(|(id, name, port)| {
            serde_json::json!({
                "id": id,
                "name": name,
                "port": port,
                "url": format!("http://localhost:{port}"),
            })
        })
        .collect();

    SkillResult {
        success: true,
        output: serde_json::json!({
            "services": services,
            "count": services.len(),
        }),
        error: None,
        duration_ms: 0,
    }
}

async fn check_service_status(service_id: &str) -> SkillResult {
    let port = match find_service_port(service_id) {
        Some(p) => p,
        None => {
            return SkillResult {
                success: false,
                output: serde_json::Value::Null,
                error: Some(format!("Service inconnu : {service_id}")),
                duration_ms: 0,
            };
        }
    };

    let url = format!("http://localhost:{port}/health");
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .unwrap_or_default();

    match client.get(&url).send().await {
        Ok(resp) if resp.status().is_success() => SkillResult {
            success: true,
            output: serde_json::json!({
                "service": service_id,
                "status": "running",
                "port": port,
            }),
            error: None,
            duration_ms: 0,
        },
        _ => SkillResult {
            success: true,
            output: serde_json::json!({
                "service": service_id,
                "status": "unreachable",
                "port": port,
            }),
            error: None,
            duration_ms: 0,
        },
    }
}

async fn call_service(
    service_id: &str,
    endpoint: &str,
    method: &str,
    body: Option<&serde_json::Value>,
) -> SkillResult {
    let port = match find_service_port(service_id) {
        Some(p) => p,
        None => {
            return SkillResult {
                success: false,
                output: serde_json::Value::Null,
                error: Some(format!("Service inconnu : {service_id}")),
                duration_ms: 0,
            };
        }
    };

    let url = format!("http://localhost:{port}{endpoint}");
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_default();

    let mut req = match method.to_uppercase().as_str() {
        "GET" => client.get(&url),
        "POST" => client.post(&url),
        "PUT" => client.put(&url),
        "DELETE" => client.delete(&url),
        _ => {
            return SkillResult {
                success: false,
                output: serde_json::Value::Null,
                error: Some(format!("Méthode HTTP inconnue : {method}")),
                duration_ms: 0,
            };
        }
    };

    if let Some(body_val) = body {
        req = req.json(body_val);
    }

    match req.send().await {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let body_text = resp.text().await.unwrap_or_default();

            // Essayer de parser en JSON
            let body_value = serde_json::from_str::<serde_json::Value>(&body_text)
                .unwrap_or_else(|_| serde_json::Value::String(body_text));

            SkillResult {
                success: (200..300).contains(&(status as usize)),
                output: serde_json::json!({
                    "service": service_id,
                    "endpoint": endpoint,
                    "status": status,
                    "body": body_value,
                }),
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
            output: serde_json::json!({
                "service": service_id,
                "endpoint": endpoint,
            }),
            error: Some(format!("Service {service_id} injoignable : {e}")),
            duration_ms: 0,
        },
    }
}

fn find_service_port(service_id: &str) -> Option<u16> {
    KNOWN_SERVICES
        .iter()
        .find(|(id, _, _)| *id == service_id)
        .map(|(_, _, port)| *port)
}
