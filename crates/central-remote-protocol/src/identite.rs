//! @id     central_remote_protocol_identite
//! @do     definir_messages_passeport_visa
//! @role   protocol
//! @layer  core
//! @human  central-remote-protocol — messages passeport d'appareil, défi, visa (sortie du lot 2).

//! Authentification des appareils appairés auprès de CentralRemote, sans secret
//! partagé (inventaire #1 à #4, sortie du lot 2).
//!
//! | Étape | Point d'accès | Authentification | Réponse |
//! |---|---|---|---|
//! | passeport | `POST /api/bridge/passeport` | requête signée par la clé d'appareil (usage [`USAGE_PASSEPORT`]) | [`PasseportResponse`] |
//! | défi | `POST /api/bridge/defi` | aucune (défi à usage unique, 60 s) | [`DefiResponse`] |
//! | visa | `POST /api/bridge/visa` | `MiyuPasseport` + `X-Miyu-PoP` | [`VisaResponse`] |
//! | requête | `POST /api/bridge/validate`, `GET /ws`… | `MiyuVisa` + `X-Miyu-PoP` (défi neuf) | selon le point d'accès |
//!
//! Format des en-têtes et de la preuve : `miyauth::http`. Audience unique :
//! [`AUDIENCE`]. Les titres et défis sont en base64 URL sans remplissage.

use serde::{Deserialize, Serialize};

/// Audience des visas et des défis de CentralRemote.
pub const AUDIENCE: &str = "central-remote";
/// Usage (séparation de domaine) des requêtes signées de demande de passeport.
pub const USAGE_PASSEPORT: &str = "central-remote/passeport";
/// Portée accordée aux appareils appairés (pilotage de Central).
pub const PORTEE_PILOTAGE: &str = "central-remote:pilotage";

/// Demande de passeport (requête signée par la clé d'appareil, corps vide).
pub const PASSEPORT_ENDPOINT: &str = "/api/bridge/passeport";
/// Émission d'un défi de possession.
pub const DEFI_ENDPOINT: &str = "/api/bridge/defi";
/// Échange passeport + preuve de possession → visa.
pub const VISA_ENDPOINT: &str = "/api/bridge/visa";
/// Vérification d'un visa + preuve de possession (remplace la validation par `device_id`).
pub const VALIDATE_ENDPOINT: &str = "/api/bridge/validate";
/// WebSocket de pilotage (visa + preuve de possession à l'ouverture).
pub const WS_ENDPOINT: &str = "/ws";

/// Passeport d'appareil émis par le COG.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasseportResponse {
    /// Passeport scellé (base64 URL).
    pub passeport: String,
    /// Fin de validité (secondes Unix).
    pub expire_a: u64,
    /// `cog_id` émetteur (= destination).
    pub cog_id: String,
}

/// Défi de possession (usage unique).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefiResponse {
    /// Identifiant du défi (base64 URL, 16 o), à renvoyer dans `X-Miyu-PoP`.
    pub defi_id: String,
    /// Défi (base64 URL, 32 o) à signer.
    pub defi: String,
    /// Audience liée au défi.
    pub audience: String,
    /// Expiration (secondes Unix).
    pub expire_a: u64,
    /// `cog_id` du COG.
    pub cog_id: String,
}

/// Visa délivré contre passeport + preuve de possession.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisaResponse {
    /// Visa scellé (base64 URL), audience [`AUDIENCE`], ≤ 15 min.
    pub visa: String,
    /// Fin de validité (secondes Unix).
    pub expire_a: u64,
    /// Rôle accordé (`citizen`).
    pub role: String,
    /// Identifiant opaque `pw:<cog_id>:<hex(sujet)>`.
    pub identite: String,
    /// Nom affiché (profil Central lié à l'appareil, sinon nom de l'appareil).
    pub display_name: String,
}

/// Résultat d'une vérification visa + preuve de possession.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidateResponse {
    /// Toujours `ok` (les refus sont des statuts HTTP 401 / 503).
    pub status: String,
    /// Identifiant opaque du titulaire.
    pub identite: String,
    /// Rôle accordé.
    pub role: String,
    /// Fin de validité du visa.
    pub expire_a: u64,
    /// Nom affiché.
    pub display_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reponses_aller_retour() {
        let v = VisaResponse {
            visa: "AA".into(),
            expire_a: 9,
            role: "citizen".into(),
            identite: "pw:x:00".into(),
            display_name: "Alice".into(),
        };
        let j = serde_json::to_string(&v).unwrap();
        assert_eq!(serde_json::from_str::<VisaResponse>(&j).unwrap(), v);
        assert!(AUDIENCE.bytes().all(|b| b.is_ascii_lowercase() || b == b'-'));
    }
}
