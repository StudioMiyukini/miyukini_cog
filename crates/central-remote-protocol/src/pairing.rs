//! @id     central_remote_protocol_pairing
//! @do     definir_messages_appairage
//! @role   protocol
//! @layer  core
//! @human  central-remote-protocol — messages de l'appairage v2 (engagement + SAS, R-026).

//! Messages HTTP de l'appairage mobile ↔ COG, protocole v2 (R-026).
//!
//! | Étape | Point d'accès | Requête | Réponse |
//! |---|---|---|---|
//! | 1. engagement | `POST /api/bridge/pair` | [`PairCommitRequest`] | [`PairCommitResponse`] (clé + aléa du COG) |
//! | 3. révélation | `POST /api/bridge/pair/reveal` | [`PairRevealRequest`] | [`PairRevealResponse`] |
//! | suivi | `POST /api/bridge/pair/status` | [`PairStatusRequest`] | [`PairStatusResponse`] |
//!
//! Toute requête dont `pairing_version` est absente (appairage v1) ou
//! différente de [`PAIRING_PROTOCOL_VERSION`] est refusée explicitement
//! (statut `unsupported_pairing_version`), sans repli.
//!
//! Le code SAS n'est **jamais** transporté : chaque côté le calcule.
//!
//! L'engagement et la révélation portent une signature ed25519 de la clé
//! d'identité de l'appareil (preuve de possession, voir
//! `miyukini_cog_bridge::pairing`). Une requête v2 sans signature (format
//! antérieur au durcissement du 2026-10-24) est refusée explicitement
//! (statut [`STATUS_MISSING_SIGNATURE`]), sans repli. La version reste 2 :
//! le format v2 n'a été publié dans aucune version distribuée avant ce
//! durcissement (introduit le même jour).

use serde::{Deserialize, Serialize};

/// Version du protocole d'appairage (doit valoir
/// `miyukini_cog_bridge::PAIRING_PROTOCOL_VERSION`).
pub const PAIRING_PROTOCOL_VERSION: u8 = 2;

/// Point d'accès de l'étape 1 (engagement).
pub const PAIR_ENDPOINT: &str = "/api/bridge/pair";
/// Point d'accès de l'étape 3 (révélation de `Nd`).
pub const PAIR_REVEAL_ENDPOINT: &str = "/api/bridge/pair/reveal";
/// Point d'accès du suivi de la demande (attente de l'approbation desktop).
pub const PAIR_STATUS_ENDPOINT: &str = "/api/bridge/pair/status";

/// Statut d'erreur renvoyé à une requête d'appairage de version inconnue.
pub const STATUS_UNSUPPORTED_VERSION: &str = "unsupported_pairing_version";
/// Statut d'erreur : signature d'appairage absente (requête v2 non signée).
pub const STATUS_MISSING_SIGNATURE: &str = "missing_pairing_signature";
/// Statut d'erreur : signature d'appairage mal formée ou invalide.
pub const STATUS_INVALID_SIGNATURE: &str = "invalid_pairing_signature";
/// Statut d'erreur : révélation ou relance depuis une autre source que l'engagement.
pub const STATUS_SOURCE_MISMATCH: &str = "source_mismatch";

/// Étape 1 — l'appareil s'engage sur `Nd` sans le révéler.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairCommitRequest {
    /// Version du protocole d'appairage ([`PAIRING_PROTOCOL_VERSION`]).
    pub pairing_version: u8,
    pub device_id: String,
    pub device_name: String,
    pub mode: String,
    /// Clé publique d'identité E2E ed25519 de l'appareil (hex 32 octets).
    pub e2e_identity_key: String,
    /// Engagement `SHA-256(COMMIT_DOMAIN || Kd || Nd)` (hex 32 octets).
    pub commitment: String,
    /// Signature ed25519 par `Kd` de
    /// `COMMIT_SIG_DOMAIN || version || len16(device_id) || device_id || Kd || C`
    /// (hex 64 octets). Absente (vide) = refus explicite.
    #[serde(default)]
    pub commit_signature: String,
}

/// Réponse à l'étape 1 — clé d'identité et aléa `Nc` du COG.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairCommitResponse {
    pub pairing_version: u8,
    /// Toujours [`PairingStatus::Committed`] en cas de succès.
    pub status: PairingStatus,
    /// Clé publique d'identité E2E du COG (hex 32 octets).
    pub cog_identity_key: String,
    /// Aléa du COG `Nc` (hex 32 octets).
    pub cog_nonce: String,
    pub message: String,
}

/// Étape 3 — l'appareil révèle `Nd` (après avoir reçu `Nc`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairRevealRequest {
    pub pairing_version: u8,
    pub device_id: String,
    /// Aléa de l'appareil `Nd` (hex 32 octets).
    pub device_nonce: String,
    /// Signature ed25519 par `Kd` de
    /// `REVEAL_SIG_DOMAIN || version || len16(device_id) || device_id || Kd || Nd || Nc`
    /// (hex 64 octets). Absente (vide) = refus explicite.
    #[serde(default)]
    pub reveal_signature: String,
}

/// Réponse à l'étape 3.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairRevealResponse {
    /// [`PairingStatus::Revealed`] en cas de succès.
    pub status: PairingStatus,
}

/// Suivi d'une demande (l'appareil attend l'approbation sur le desktop).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairStatusRequest {
    pub device_id: String,
    /// Clé d'identité de l'appareil (hex) : seule une approbation de cette
    /// clé précise vaut `approved`.
    pub e2e_identity_key: String,
}

/// Réponse au suivi.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairStatusResponse {
    pub status: PairingStatus,
}

/// État d'une demande d'appairage
/// (`Committed → Revealed → Approved | Rejected | Expired`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PairingStatus {
    /// Engagement reçu, aléa du COG envoyé, révélation attendue.
    Committed,
    /// `Nd` révélé et vérifié : code affiché, approbation desktop attendue.
    Revealed,
    /// Approuvé sur le desktop : clé épinglée dans la liste blanche.
    Approved,
    /// Refusé sur le desktop (ou révélation invalide).
    Rejected,
    /// Délai de validité dépassé sans approbation.
    Expired,
    /// Aucune demande connue pour cet appareil et cette clé.
    Unknown,
}

/// Sonde de version : lue avant toute autre interprétation du JSON.
#[derive(Deserialize)]
struct VersionProbe {
    /// Absente dans l'appairage v1.
    #[serde(default = "legacy_pairing_version")]
    pairing_version: u8,
}

const fn legacy_pairing_version() -> u8 {
    1
}

/// Vérifie la version d'appairage d'un corps JSON reçu. `Err(v)` porte la
/// version refusée (1 si le champ est absent) ; un JSON illisible est
/// également refusé (`Err(0)`).
pub fn check_pairing_version(body: &[u8]) -> Result<(), u8> {
    let probe: VersionProbe = serde_json::from_slice(body).map_err(|_| 0u8)?;
    if probe.pairing_version == PAIRING_PROTOCOL_VERSION {
        Ok(())
    } else {
        Err(probe.pairing_version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_request_without_version_refused() {
        let v1 = br#"{"device_id":"d","device_name":"x","mode":"whitelist","e2e_identity_key":"aa"}"#;
        assert_eq!(check_pairing_version(v1), Err(1));
        assert_eq!(check_pairing_version(br#"{"pairing_version":3}"#), Err(3));
        assert_eq!(check_pairing_version(b"not json"), Err(0));
        assert_eq!(check_pairing_version(br#"{"pairing_version":2}"#), Ok(()));
    }

    #[test]
    fn unsigned_v2_requests_parse_with_empty_signature() {
        // Le serveur refuse explicitement une signature vide (aucun repli).
        let c: PairCommitRequest = serde_json::from_str(
            r#"{"pairing_version":2,"device_id":"d","device_name":"x","mode":"whitelist","e2e_identity_key":"aa","commitment":"bb"}"#,
        )
        .unwrap();
        assert_eq!(c.commit_signature, "");
        let r: PairRevealRequest =
            serde_json::from_str(r#"{"pairing_version":2,"device_id":"d","device_nonce":"cc"}"#).unwrap();
        assert_eq!(r.reveal_signature, "");
    }

    #[test]
    fn status_serialized_snake_case() {
        let r = PairStatusResponse {
            status: PairingStatus::Revealed,
        };
        assert_eq!(serde_json::to_string(&r).unwrap(), r#"{"status":"revealed"}"#);
    }
}
