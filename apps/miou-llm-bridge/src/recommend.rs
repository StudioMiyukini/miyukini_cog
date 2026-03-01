//! Moteur de recommandation LLM — sélectionne le meilleur modèle GGUF
//! en fonction du hardware détecté et des modèles disponibles sur LM Studio.

use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::catalog::CATALOG;
use crate::hardware::HardwareInfo;
use crate::proxy::ProxyState;

// ── Types réponse ────────────────────────────────────────────────────────

/// Réponse GET /v1/hardware
#[derive(Debug, Serialize)]
pub struct HardwareResponse {
    pub hardware: HardwareInfo,
}

/// Un modèle recommandé.
#[derive(Debug, Clone, Serialize)]
pub struct Recommendation {
    /// ID du modèle sur LM Studio (à passer à /v1/chat/completions).
    pub model_id: String,
    /// Nom affiché (depuis le catalogue).
    pub display_name: String,
    /// Paramètres (ex: "7B").
    pub params: String,
    /// Quantization (ex: "Q4_K_M").
    pub quantization: String,
    /// Score combiné (0-100).
    pub score: u8,
    /// Description.
    pub description: String,
    /// Déjà chargé dans LM Studio.
    pub already_loaded: bool,
    /// Label du tier.
    pub tier_label: String,
}

/// Réponse GET /v1/recommend
#[derive(Debug, Serialize)]
pub struct RecommendResponse {
    /// Meilleur modèle recommandé (None si aucun disponible).
    pub recommended: Option<Recommendation>,
    /// Tous les candidats classés par score décroissant.
    pub candidates: Vec<Recommendation>,
    /// Tier hardware détecté.
    pub hardware_tier: String,
    /// Nombre de modèles trouvés sur LM Studio.
    pub models_available: usize,
}

// ── Types LM Studio API ─────────────────────────────────────────────────

/// Modèle retourné par LM Studio GET /api/v1/models (v1 REST API).
#[derive(Debug, Deserialize)]
struct LmStudioModel {
    #[serde(default)]
    key: Option<String>,
    #[serde(default)]
    display_name: Option<String>,
    #[serde(default)]
    loaded_instances: Option<Vec<serde_json::Value>>,
}

/// Réponse de GET /api/v1/models (v1 REST API — liste TOUS les modèles).
#[derive(Debug, Deserialize)]
struct LmStudioModelsV1 {
    #[serde(default)]
    models: Vec<LmStudioModel>,
}

/// Modèle retourné par GET /v1/models (OpenAI-compatible — modèles CHARGÉS).
#[derive(Debug, Deserialize)]
struct OpenAiModel {
    id: String,
}

#[derive(Debug, Deserialize)]
struct OpenAiModelsResponse {
    #[serde(default)]
    data: Vec<OpenAiModel>,
}

// ── Moteur de recommandation ─────────────────────────────────────────────

/// Score combiné : 70% qualité + 30% français.
fn combined_score(quality: u8, french: u8) -> u8 {
    let q = quality as u16;
    let f = french as u16 * 10; // 0-10 → 0-100
    let combined = (q * 7 + f * 3) / 10;
    combined.min(100) as u8
}

/// Construit la liste de recommandations à partir du hardware et des modèles disponibles.
fn recommend(
    hardware: &HardwareInfo,
    available_model_ids: &[String],
    loaded_model_ids: &[String],
) -> RecommendResponse {
    // RAM utilisable = 75% de la RAM totale (réserve pour l'OS)
    let usable_ram = hardware.ram_total_mb * 3 / 4;

    let mut candidates: Vec<Recommendation> = Vec::new();

    for model_id in available_model_ids {
        let model_id_lower = model_id.to_lowercase();

        // Chercher dans le catalogue
        if let Some(entry) = CATALOG
            .iter()
            .find(|e| model_id_lower.contains(e.match_pattern))
        {
            // Vérifier si le modèle rentre en RAM
            if entry.ram_required_mb > usable_ram {
                continue;
            }

            let score = combined_score(entry.quality_score, entry.french_score);
            let already_loaded = loaded_model_ids
                .iter()
                .any(|id| id.to_lowercase() == model_id_lower);

            candidates.push(Recommendation {
                model_id: model_id.clone(),
                display_name: entry.display_name.to_string(),
                params: entry.params.to_string(),
                quantization: entry.quantization.to_string(),
                score,
                description: entry.description.to_string(),
                already_loaded,
                tier_label: format!("Tier {}", entry.min_tier),
            });
        }
    }

    // Tri : score décroissant, préférence aux modèles déjà chargés
    candidates.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then(b.already_loaded.cmp(&a.already_loaded))
    });

    let recommended = candidates.first().cloned();

    RecommendResponse {
        recommended,
        candidates,
        hardware_tier: hardware.tier.label().to_string(),
        models_available: available_model_ids.len(),
    }
}

// ── Handlers Axum ────────────────────────────────────────────────────────

/// GET /v1/hardware — retourne les specs hardware (caché au démarrage).
pub async fn hardware_handler(State(state): State<ProxyState>) -> impl IntoResponse {
    Json(HardwareResponse {
        hardware: state.hardware.clone(),
    })
}

/// GET /v1/recommend — interroge LM Studio, croise avec le catalogue.
pub async fn recommend_handler(State(state): State<ProxyState>) -> impl IntoResponse {
    // 1. Récupérer TOUS les modèles téléchargés via l'API v1 de LM Studio
    let available_ids = fetch_all_model_ids(&state).await;

    // 2. Récupérer les modèles actuellement CHARGÉS (endpoint OpenAI-compatible)
    let loaded_ids = fetch_loaded_model_ids(&state).await;

    // 3. Recommander
    let response = recommend(&state.hardware, &available_ids, &loaded_ids);

    Json(response)
}

/// Interroge GET /api/v1/models de LM Studio pour lister tous les modèles téléchargés.
pub async fn fetch_all_model_ids(state: &ProxyState) -> Vec<String> {
    let url = format!("{}/api/v1/models", state.upstream_url);
    let resp = match state
        .client
        .get(&url)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
    {
        Ok(r) if r.status().is_success() => r,
        _ => return Vec::new(),
    };

    match resp.json::<LmStudioModelsV1>().await {
        Ok(body) => body
            .models
            .into_iter()
            .filter_map(|m| m.key.or(m.display_name))
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// Interroge GET /v1/models (OpenAI-compat) pour lister les modèles chargés.
pub async fn fetch_loaded_model_ids(state: &ProxyState) -> Vec<String> {
    let url = format!("{}/v1/models", state.upstream_url);
    let resp = match state
        .client
        .get(&url)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
    {
        Ok(r) if r.status().is_success() => r,
        _ => return Vec::new(),
    };

    match resp.json::<OpenAiModelsResponse>().await {
        Ok(body) => body.data.into_iter().map(|m| m.id).collect(),
        Err(_) => Vec::new(),
    }
}
