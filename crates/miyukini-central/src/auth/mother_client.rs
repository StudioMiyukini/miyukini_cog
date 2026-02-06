//! Client de validation des ID de profils auprès de la DB mère.
//!
//! À la première connexion à internet puis à intervalle régulier, l'environnement
//! interroge la DB mère pour valider les ID des profils. Si un ID existe déjà
//! pour un autre compte, la mère alloue un nouvel ID et la fille l'inscrit en local.

use crate::auth::db::CentralProfile;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Erreur du client DB mère (réseau ou format).
#[derive(Debug)]
pub struct MotherClientError(pub String);

impl std::fmt::Display for MotherClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DB mère: {}", self.0)
    }
}

impl std::error::Error for MotherClientError {}

/// Référence profil envoyée à la DB mère.
#[derive(Serialize)]
struct ProfileRef<'a> {
    id: &'a str,
    email: &'a str,
}

/// Corps de la requête vers la DB mère (validation des profils).
#[derive(Serialize)]
struct ValidateRequest<'a> {
    profiles: Vec<ProfileRef<'a>>,
}

/// Un résultat par profil : ID actuel et ID attribué par la mère (peut différer en cas de conflit).
#[derive(Deserialize)]
struct ValidateResultItem {
    id: String,
    #[allow(dead_code)]
    email: String,
    assigned_id: String,
}

/// Réponse de la DB mère.
#[derive(Deserialize)]
struct ValidateResponse {
    results: Vec<ValidateResultItem>,
}

const VALIDATE_PATH: &str = "profiles/validate";
const REQUEST_TIMEOUT_SEC: u64 = 15;

/// Appelle la DB mère pour valider les ID des profils.
///
/// **Contrat API DB mère** (à implémenter côté serveur) :
/// - `POST {base_url}/profiles/validate`
/// - Corps : `{ "profiles": [ { "id": "uuid", "email": "..." } ] }`
/// - Réponse 200 : `{ "results": [ { "id": "...", "email": "...", "assigned_id": "..." } ] }`
/// - Si un `id` existe déjà pour un autre compte, la mère alloue un nouvel UUID et le renvoie dans `assigned_id` ; sinon `assigned_id` = `id`.
/// Retourne la liste des réattributions à appliquer en local : `(ancien_id, nouvel_id)`.
/// En cas d'erreur réseau ou de format, retourne une erreur (pas de mise à jour).
pub fn validate_profiles_with_mother(
    base_url: &str,
    profiles: &[CentralProfile],
) -> Result<Vec<(String, String)>, MotherClientError> {
    if profiles.is_empty() {
        return Ok(Vec::new());
    }
    let base = base_url.trim_end_matches('/');
    let url = format!("{}/{}", base, VALIDATE_PATH);
    let body = ValidateRequest {
        profiles: profiles
            .iter()
            .map(|p| ProfileRef {
                id: p.id.as_str(),
                email: p.email.as_str(),
            })
            .collect(),
    };
    let body_json = serde_json::to_vec(&body).map_err(|e| MotherClientError(e.to_string()))?;
    let agent = ureq::Agent::new();
    let response = agent
        .post(&url)
        .set("Content-Type", "application/json")
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SEC))
        .send_bytes(&body_json);

    let resp = response.map_err(|e| MotherClientError(e.to_string()))?;
    let status = resp.status();
    if status != 200 {
        let body_text = resp.into_string().unwrap_or_default();
        return Err(MotherClientError(format!(
            "HTTP {}: {}",
            status,
            body_text.lines().next().unwrap_or("")
        )));
    }
    let response_body: ValidateResponse = resp
        .into_json()
        .map_err(|e| MotherClientError(format!("Réponse invalide: {}", e)))?;
    let updates: Vec<(String, String)> = response_body
        .results
        .into_iter()
        .filter(|r| r.id != r.assigned_id)
        .map(|r| (r.id, r.assigned_id))
        .collect();
    Ok(updates)
}
