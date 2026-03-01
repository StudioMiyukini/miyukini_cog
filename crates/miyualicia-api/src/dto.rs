//! Data Transfer Objects pour l'API REST Alicia.
//!
//! Types de requete et de reponse JSON.

// @id: service.alicia.api.dto
// @role: data_transfer_objects
// @layer: 7
// @human: DTOs request/response JSON Alicia API.
// @do: define_api_request_response_types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// --- DTOs de requete ---

/// Corps de la requete d'authentification.
#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    /// Identifiant du client.
    pub client_id: String,
    /// Secret du client.
    pub secret: String,
}

/// Corps d'une commande sur un dispositif.
#[derive(Debug, Deserialize)]
pub struct CommandRequest {
    /// Action a realiser (ex: "on", "off", "set_brightness").
    pub action: String,
    /// Valeur optionnelle associee a l'action.
    pub value: Option<serde_json::Value>,
}

/// Parametres de pagination pour GET /history.
#[derive(Debug, Deserialize)]
pub struct HistoryParams {
    /// Nombre maximum d'entrees retournees. Defaut : 50. Maximum : 500.
    #[serde(default = "HistoryParams::default_limit")]
    pub limit: u32,
    /// Offset pour la pagination.
    #[serde(default)]
    pub offset: u32,
    /// Filtrer par source.
    pub source: Option<String>,
    /// Filtrer par device_id.
    pub device_id: Option<Uuid>,
}

impl HistoryParams {
    fn default_limit() -> u32 {
        50
    }
}

// --- DTOs de reponse ---

/// Reponse d'authentification : le token JWT.
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    /// Le token JWT.
    pub access_token: String,
    /// Type : toujours "Bearer".
    pub token_type: String,
    /// Duree de validite en secondes.
    pub expires_in: u64,
    /// Scopes accordes.
    pub scopes: Vec<String>,
}

/// Etat complet de la maison.
#[derive(Debug, Serialize)]
pub struct StateDto {
    /// Pieces avec leurs dispositifs.
    pub rooms: Vec<RoomDto>,
    /// Statut de la connexion MQTT.
    pub mqtt_connected: bool,
    /// Nombre total de dispositifs.
    pub total_devices: usize,
    /// Nombre de dispositifs actifs.
    pub active_devices: usize,
    /// Timestamp du snapshot.
    pub timestamp: DateTime<Utc>,
}

/// Etat d'une piece.
#[derive(Debug, Serialize)]
pub struct RoomDto {
    /// Identifiant de la piece.
    pub id: String,
    /// Nom lisible de la piece.
    pub name: String,
    /// Dispositifs dans la piece.
    pub devices: Vec<DeviceDto>,
}

/// Etat d'un dispositif (sans credentials).
#[derive(Debug, Serialize)]
pub struct DeviceDto {
    /// Identifiant UUID du dispositif.
    pub id: Uuid,
    /// Identifiant de la piece.
    pub room_id: String,
    /// Type du dispositif (ex: "light", "thermostat").
    pub device_type: String,
    /// Nom lisible.
    pub name: String,
    /// Protocole de communication.
    pub protocol: String,
    /// Adresse de communication.
    pub address: String,
    /// Capacites du dispositif.
    pub capabilities: DeviceCapabilitiesDto,
    /// Etat courant du dispositif.
    pub state: DeviceStateDto,
    /// Le dispositif est-il actif ?
    pub active: bool,
}

/// Capacites d'un dispositif.
#[derive(Debug, Serialize)]
pub struct DeviceCapabilitiesDto {
    /// Peut etre allume/eteint.
    pub on_off: bool,
    /// Support du dimmer.
    pub dimmer: bool,
    /// Support RGB.
    pub rgb: bool,
    /// Support de la position (volets).
    pub position: bool,
    /// Support de la temperature cible.
    pub temperature_target: bool,
    /// Mesure de la consommation electrique.
    pub power_measure: bool,
}

/// Etat courant d'un dispositif.
#[derive(Debug, Serialize)]
pub struct DeviceStateDto {
    /// Allume/eteint.
    pub on: Option<bool>,
    /// Luminosite (0-100).
    pub brightness: Option<u8>,
    /// Couleur RGB.
    pub color_rgb: Option<[u8; 3]>,
    /// Position (0-100, pour volets).
    pub position: Option<u8>,
    /// Temperature courante.
    pub temperature_current: Option<f32>,
    /// Temperature cible (thermostat).
    pub temperature_target: Option<f32>,
    /// Consommation en watts.
    pub power_w: Option<f32>,
    /// Verrouille/deverrouille.
    pub locked: Option<bool>,
    /// Mouvement detecte.
    pub motion: Option<bool>,
    /// Contact detecte.
    pub contact: Option<bool>,
    /// Humidite relative.
    pub humidity: Option<f32>,
    /// Derniere mise a jour.
    pub last_updated: DateTime<Utc>,
    /// `true` si au moins un champ d'etat est connu.
    pub is_known: bool,
}

/// Reponse a une commande.
#[derive(Debug, Serialize)]
pub struct CommandResponse {
    /// Succes de la commande.
    pub success: bool,
    /// UUID du dispositif.
    pub device_id: Uuid,
    /// Action executee.
    pub action: String,
    /// Latence d'execution en millisecondes.
    pub latency_ms: u64,
    /// Message optionnel.
    pub message: Option<String>,
}

/// Entree dans le journal d'activite.
#[derive(Debug, Serialize)]
pub struct HistoryEntryDto {
    /// Identifiant de l'entree.
    pub id: Uuid,
    /// Source de la commande.
    pub source: String,
    /// Detail de la source.
    pub source_detail: Option<String>,
    /// UUID du dispositif.
    pub device_id: Option<Uuid>,
    /// Nom du dispositif.
    pub device_name: Option<String>,
    /// Commande executee.
    pub command: serde_json::Value,
    /// Succes.
    pub success: bool,
    /// Message d'erreur.
    pub error_message: Option<String>,
    /// Latence en ms.
    pub latency_ms: Option<u64>,
    /// Date d'execution.
    pub executed_at: DateTime<Utc>,
}

/// Reponse paginee de l'historique.
#[derive(Debug, Serialize)]
pub struct HistoryResponse {
    /// Entrees du journal.
    pub entries: Vec<HistoryEntryDto>,
    /// Nombre total d'entrees.
    pub total: u64,
    /// Limite par page.
    pub limit: u32,
    /// Offset courant.
    pub offset: u32,
}

/// Etat de sante du service.
#[derive(Debug, Serialize)]
pub struct HealthDto {
    /// Statut : "ok".
    pub status: &'static str,
    /// Version du service.
    pub version: &'static str,
    /// Temps d'activite en secondes.
    pub uptime_secs: u64,
    /// Statut MQTT.
    pub mqtt_connected: bool,
    /// Nombre de dispositifs.
    pub devices_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_response_serialize() {
        let resp = TokenResponse {
            access_token: "eyJ...".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            scopes: vec!["read".to_string(), "write".to_string()],
        };
        let json = serde_json::to_string(&resp).expect("serialize");
        assert!(json.contains("access_token"));
        assert!(json.contains("Bearer"));
    }

    #[test]
    fn test_health_dto_serialize() {
        let health = HealthDto {
            status: "ok",
            version: "0.1.0",
            uptime_secs: 120,
            mqtt_connected: true,
            devices_count: 5,
        };
        let json = serde_json::to_string(&health).expect("serialize");
        assert!(json.contains("\"ok\""));
        assert!(json.contains("0.1.0"));
    }

    #[test]
    fn test_command_request_deserialize() {
        let json = r#"{"action": "set_brightness", "value": 75}"#;
        let req: CommandRequest = serde_json::from_str(json).expect("deserialize");
        assert_eq!(req.action, "set_brightness");
        assert_eq!(req.value, Some(serde_json::json!(75)));
    }

    #[test]
    fn test_command_request_without_value() {
        let json = r#"{"action": "on"}"#;
        let req: CommandRequest = serde_json::from_str(json).expect("deserialize");
        assert_eq!(req.action, "on");
        assert!(req.value.is_none());
    }

    #[test]
    fn test_history_params_defaults() {
        let json = r#"{}"#;
        let params: HistoryParams = serde_json::from_str(json).expect("deserialize");
        assert_eq!(params.limit, 50);
        assert_eq!(params.offset, 0);
        assert!(params.source.is_none());
        assert!(params.device_id.is_none());
    }
}
