//! @id     miyucloud_application_services_device_auth_service
//! @do     definir_device_code_lifetime_secs
//! @role   securite
//! @layer  app
//! @human  miyucloud — flux d'appareil RFC 8628 terminé par l'enregistrement d'une clé d'appareil.

//! OAuth 2.0 Device Authorization Grant (RFC 8628), sans jeton porteur (lot 2, #21).
//!
//!   1. `initiate` — le client envoie sa **clé publique Ed25519** ; device_code +
//!      user_code sont générés et stockés ;
//!   2. `verify_user_code` — page de vérification ;
//!   3. `approve` — l'utilisateur connecté approuve : la clé de l'appareil est
//!      **enregistrée** pour lui (portées du flux) ; aucun jeton n'est créé ;
//!   4. `deny` ;
//!   5. `poll` — le client apprend que sa clé est enregistrée (`MiyuSignature`) et
//!      signe désormais chaque requête ([`crate::infrastructure::services::miyu_auth_service`]) ;
//!   6. `cleanup_expired`.
//!
//! Avant : la fin du flux délivrait un Bearer JWT HS256 signé par le secret
//! partagé `MIYUCLOUD_JWT_SECRET`, stocké en clair dans `auth.device_codes`.

use std::sync::Arc;

use crate::application::dtos::device_auth_dto::*;
use crate::application::ports::auth_ports::DeviceCodeStoragePort;
use crate::common::errors::{DomainError, ErrorKind};
use crate::domain::entities::device_code::{DeviceCode, DeviceCodeStatus};
use crate::infrastructure::repositories::pg::{DeviceCodePgRepository, MiyuIdentityPgRepository};
use crate::infrastructure::services::miyu_auth_service::{
    MiyuIdentityStore, USAGE_APPAREIL, analyser_cle_publique,
};
use uuid::Uuid;

/// Default device code lifetime: 15 minutes (RFC 8628 recommends 5-30 min).
const DEVICE_CODE_LIFETIME_SECS: i64 = 900;

/// Default polling interval in seconds (RFC 8628 §3.2 recommends 5s).
const DEFAULT_POLL_INTERVAL: i32 = 5;

/// Length of the device_code (hex-encoded, 64 chars = 32 bytes).
const DEVICE_CODE_BYTES: usize = 32;

/// User code format: 4 uppercase letters + hyphen + 4 digits → "ABCD-1234"
const USER_CODE_LETTER_LEN: usize = 4;
const USER_CODE_DIGIT_LEN: usize = 4;

/// Portées qu'une clé d'appareil peut recevoir (fail-closed : le reste est refusé).
const PORTEES_APPAREIL: &[&str] = &["webdav", "caldav", "carddav", "api"];

/// Normalise et valide les portées demandées.
fn portees_valides(scopes: &str) -> Result<String, DomainError> {
    let mut retenues: Vec<&str> = Vec::new();
    for p in scopes.split(',').map(str::trim).filter(|p| !p.is_empty()) {
        if !PORTEES_APPAREIL.contains(&p) {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "DeviceCode",
                format!("Unsupported scope: {p}"),
            ));
        }
        if !retenues.contains(&p) {
            retenues.push(p);
        }
    }
    if retenues.is_empty() {
        return Err(DomainError::new(
            ErrorKind::InvalidInput,
            "DeviceCode",
            "At least one scope is required",
        ));
    }
    Ok(retenues.join(","))
}

pub struct DeviceAuthService<D = DeviceCodePgRepository, S = MiyuIdentityPgRepository>
where
    D: DeviceCodeStoragePort,
    S: MiyuIdentityStore,
{
    device_code_storage: Arc<D>,
    keys: Arc<S>,
    /// Base URL of the server (e.g. "https://cloud.example.com")
    base_url: String,
}

impl<D: DeviceCodeStoragePort, S: MiyuIdentityStore> DeviceAuthService<D, S> {
    pub fn new(device_code_storage: Arc<D>, keys: Arc<S>, base_url: String) -> Self {
        Self {
            device_code_storage,
            keys,
            base_url,
        }
    }

    // ========================================================================
    // 1. Initiate — called by the DAV client
    // ========================================================================

    /// Start a new device authorization flow.
    ///
    /// Returns the response that the client displays to the user.
    pub async fn initiate(
        &self,
        req: DeviceAuthorizeRequestDto,
    ) -> Result<DeviceAuthorizeResponseDto, DomainError> {
        let cle = req
            .device_public_key
            .as_deref()
            .and_then(analyser_cle_publique)
            // Clé faible ou invalide : refusée à chaque requête (verify_strict).
            .ok_or_else(|| {
                DomainError::new(
                    ErrorKind::InvalidInput,
                    "DeviceCode",
                    "device_public_key (Ed25519, 64 hex or base64url) is required",
                )
            })?;
        let scopes = portees_valides(&req.scope)?;
        let device_code_token = generate_device_code();
        let user_code = generate_user_code();

        let verification_uri = format!("{}/device", self.base_url.trim_end_matches('/'));
        let verification_uri_complete = format!("{}?code={}", verification_uri, user_code);

        let dc = DeviceCode::new(
            device_code_token.clone(),
            user_code.clone(),
            req.client_name.chars().take(255).collect(),
            scopes,
            cle,
            verification_uri.clone(),
            Some(verification_uri_complete.clone()),
            DEVICE_CODE_LIFETIME_SECS,
            DEFAULT_POLL_INTERVAL,
        );

        let dc = self.device_code_storage.create_device_code(dc).await?;

        tracing::info!(
            "Device auth flow initiated: user_code={}, expires_in={}s",
            user_code,
            DEVICE_CODE_LIFETIME_SECS
        );

        Ok(DeviceAuthorizeResponseDto {
            device_code: device_code_token,
            user_code,
            verification_uri,
            verification_uri_complete: Some(verification_uri_complete),
            expires_in: dc.seconds_remaining(),
            interval: DEFAULT_POLL_INTERVAL,
        })
    }

    // ========================================================================
    // 2. Verify — user opens the verification page, looks up pending code
    // ========================================================================

    /// Look up a pending device code by user_code for the verification page.
    pub async fn verify_user_code(
        &self,
        user_code: &str,
    ) -> Result<DeviceVerifyInfoDto, DomainError> {
        let normalized = user_code.trim().to_uppercase().replace(' ', "");

        match self
            .device_code_storage
            .get_pending_by_user_code(&normalized)
            .await
        {
            Ok(dc) => {
                if dc.is_expired() {
                    return Ok(DeviceVerifyInfoDto {
                        client_name: dc.client_name().to_string(),
                        scopes: dc.scopes().to_string(),
                        valid: false,
                    });
                }
                Ok(DeviceVerifyInfoDto {
                    client_name: dc.client_name().to_string(),
                    scopes: dc.scopes().to_string(),
                    valid: true,
                })
            }
            Err(_) => Ok(DeviceVerifyInfoDto {
                client_name: String::new(),
                scopes: String::new(),
                valid: false,
            }),
        }
    }

    // ========================================================================
    // 3. Approve — authenticated user approves the device code
    // ========================================================================

    /// Approuve un flux : la clé de l'appareil est enregistrée pour `user_id`.
    ///
    /// * `user_code` — the code from the verification page
    /// * `user_id` — the authenticated user's ID (session)
    pub async fn approve(&self, user_code: &str, user_id: Uuid) -> Result<(), DomainError> {
        let normalized = user_code.trim().to_uppercase().replace(' ', "");

        let mut dc = self
            .device_code_storage
            .get_pending_by_user_code(&normalized)
            .await?;

        if dc.is_expired() {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "DeviceCode",
                "Device code has expired. Please start a new authorization flow.",
            ));
        }

        let cle = *dc.device_public_key().ok_or_else(|| {
            DomainError::new(
                ErrorKind::InvalidInput,
                "DeviceCode",
                "Device code has no device key",
            )
        })?;
        // Enregistre la clé de l'appareil (aucun jeton n'est créé).
        self.keys
            .register_device_key(user_id, &cle, dc.client_name(), dc.scopes())
            .await?;
        dc.authorize(user_id);
        self.device_code_storage.update_device_code(dc).await?;

        tracing::info!(
            "Device code approved by user {} (user_code={})",
            user_id,
            normalized
        );

        Ok(())
    }

    // ========================================================================
    // 4. Deny — authenticated user denies the device code
    // ========================================================================

    pub async fn deny(&self, user_code: &str) -> Result<(), DomainError> {
        let normalized = user_code.trim().to_uppercase().replace(' ', "");

        let mut dc = self
            .device_code_storage
            .get_pending_by_user_code(&normalized)
            .await?;

        dc.deny();
        self.device_code_storage.update_device_code(dc).await?;

        tracing::info!("Device code denied (user_code={})", normalized);

        Ok(())
    }

    // ========================================================================
    // 5. Poll — client polls by device_code for tokens
    // ========================================================================

    /// Le client interroge le flux. Rend :
    /// - `Ok(DeviceTokenSuccessDto)` si la clé est enregistrée
    /// - `Err` with specific RFC 8628 error codes for pending/slow_down/expired/denied
    pub async fn poll(&self, device_code: &str) -> Result<DeviceTokenSuccessDto, DevicePollError> {
        let mut dc = self
            .device_code_storage
            .get_by_device_code(device_code)
            .await
            .map_err(|_| DevicePollError::InvalidDeviceCode)?;

        // Check expiry first
        if dc.is_expired() && dc.status() == DeviceCodeStatus::Pending {
            let mut expired_dc = dc.clone();
            expired_dc.mark_expired();
            let _ = self
                .device_code_storage
                .update_device_code(expired_dc)
                .await;
            return Err(DevicePollError::ExpiredToken);
        }

        match dc.status() {
            DeviceCodeStatus::Pending => {
                // Check for slow_down (polling too fast)
                if dc.is_polling_too_fast() {
                    return Err(DevicePollError::SlowDown);
                }
                // Record this poll
                dc.record_poll();
                let _ = self.device_code_storage.update_device_code(dc).await;
                Err(DevicePollError::AuthorizationPending)
            }
            DeviceCodeStatus::Authorized => {
                let cle = *dc
                    .device_public_key()
                    .ok_or(DevicePollError::InvalidDeviceCode)?;
                let enregistree = self
                    .keys
                    .active_device_key(&cle)
                    .await
                    .ok()
                    .flatten()
                    .ok_or(DevicePollError::AccessDenied)?;
                // Le code a servi : il est supprimé.
                let _ = self.device_code_storage.delete_by_id(dc.id()).await;
                Ok(DeviceTokenSuccessDto {
                    token_type: "MiyuSignature".to_string(),
                    device_id: enregistree.id.to_string(),
                    device_public_key: enregistree.public_key,
                    usage: USAGE_APPAREIL.to_string(),
                    scope: enregistree.scopes,
                })
            }
            DeviceCodeStatus::Denied => Err(DevicePollError::AccessDenied),
            DeviceCodeStatus::Expired => Err(DevicePollError::ExpiredToken),
        }
    }

    // ========================================================================
    // 6. Cleanup — purge expired entries
    // ========================================================================

    pub async fn cleanup_expired(&self) -> Result<u64, DomainError> {
        let deleted = self.device_code_storage.delete_expired().await?;
        if deleted > 0 {
            tracing::info!("Device code cleanup: {} expired entries removed", deleted);
        }
        Ok(deleted)
    }

    // ========================================================================
    // 7. List — user's authorized devices (for UI)
    // ========================================================================

    pub async fn list_user_devices(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<DeviceInfoDto>, DomainError> {
        let cles = self.keys.list_device_keys(user_id).await?;
        Ok(cles
            .into_iter()
            .map(|k| DeviceInfoDto {
                id: k.id.to_string(),
                client_name: k.client_name,
                scopes: k.scopes,
                status: "authorized".to_string(),
                created_at: k.created_at.to_rfc3339(),
                authorized_at: Some(k.created_at.to_rfc3339()),
                last_used_at: k.last_used_at.map(|t| t.to_rfc3339()),
                public_key: k.public_key,
            })
            .collect())
    }

    // ========================================================================
    // 8. Revoke — user revokes a device key
    // ========================================================================

    pub async fn revoke_device(&self, device_id: Uuid, user_id: Uuid) -> Result<(), DomainError> {
        if self.keys.revoke_device_key(user_id, device_id).await? {
            Ok(())
        } else {
            Err(DomainError::new(
                ErrorKind::NotFound,
                "DeviceCode",
                "Device authorization not found or not owned by you",
            ))
        }
    }
}

// ============================================================================
// Poll error (typed for RFC 8628 error responses)
// ============================================================================

/// Typed errors for the device token polling endpoint (RFC 8628 §3.5).
#[derive(Debug)]
pub enum DevicePollError {
    /// The authorization request is still pending (user hasn't acted yet).
    AuthorizationPending,
    /// The client is polling too fast; increase the interval.
    SlowDown,
    /// The user denied the authorization request.
    AccessDenied,
    /// The device_code has expired.
    ExpiredToken,
    /// The device_code is not recognized.
    InvalidDeviceCode,
}

impl DevicePollError {
    /// RFC 8628 error string for the JSON response.
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::AuthorizationPending => "authorization_pending",
            Self::SlowDown => "slow_down",
            Self::AccessDenied => "access_denied",
            Self::ExpiredToken => "expired_token",
            Self::InvalidDeviceCode => "invalid_grant",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::AuthorizationPending => {
                "The authorization request is still pending. Continue polling."
            }
            Self::SlowDown => "You are polling too frequently. Please slow down.",
            Self::AccessDenied => "The user denied the authorization request.",
            Self::ExpiredToken => {
                "The device_code has expired. Please start a new authorization flow."
            }
            Self::InvalidDeviceCode => "The device_code is not recognized.",
        }
    }

    /// HTTP status code per RFC 8628 §3.5:
    /// - authorization_pending and slow_down: 400
    /// - access_denied: 403
    /// - expired_token: 400
    pub fn http_status(&self) -> u16 {
        match self {
            Self::AuthorizationPending | Self::SlowDown | Self::ExpiredToken => 400,
            Self::AccessDenied => 403,
            Self::InvalidDeviceCode => 400,
        }
    }
}

// ============================================================================
// Additional DTOs (used by service, not in the handler module)
// ============================================================================

/// DTO for listing authorized devices in the user's profile.
#[derive(Debug, serde::Serialize)]
pub struct DeviceInfoDto {
    pub id: String,
    pub client_name: String,
    pub scopes: String,
    pub status: String,
    pub created_at: String,
    pub authorized_at: Option<String>,
    pub last_used_at: Option<String>,
    /// Clé publique de l'appareil (base64 URL).
    pub public_key: String,
}

// ============================================================================
// Helpers
// ============================================================================

/// Generate a cryptographically random device_code (hex-encoded).
fn generate_device_code() -> String {
    use rand_core::{OsRng, RngCore};
    let mut bytes = [0u8; DEVICE_CODE_BYTES];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// Generate a human-readable user_code in the format "ABCD-1234".
fn generate_user_code() -> String {
    use rand_core::{OsRng, RngCore};
    let mut rng_bytes = [0u8; 8];
    OsRng.fill_bytes(&mut rng_bytes);

    let letters: String = (0..USER_CODE_LETTER_LEN)
        .map(|i| {
            let b = rng_bytes[i] % 26;
            (b'A' + b) as char
        })
        .collect();

    let digits: String = (0..USER_CODE_DIGIT_LEN)
        .map(|i| {
            let b = rng_bytes[USER_CODE_LETTER_LEN + i] % 10;
            (b'0' + b) as char
        })
        .collect();

    format!("{}-{}", letters, digits)
}

#[cfg(test)]
mod tests {
    //! Flux d'appareil de bout en bout (sans PostgreSQL) : clé publique au début,
    //! approbation = enregistrement de la clé, requêtes signées ensuite.
    use super::*;
    use crate::infrastructure::services::miyu_auth_service::{
        MemoireIdentites, MiyuAuthService, RefusMiyu, RequeteProuvee, maintenant,
    };
    use miyauth::http::{b64_encoder, signer_requete};
    use miyusceau::CleSignature;
    use std::collections::HashMap;
    use std::sync::Mutex;

    #[derive(Default)]
    struct MemoireCodes(Mutex<HashMap<Uuid, DeviceCode>>);

    fn introuvable() -> DomainError {
        DomainError::new(ErrorKind::NotFound, "DeviceCode", "not found")
    }

    impl DeviceCodeStoragePort for MemoireCodes {
        async fn create_device_code(&self, dc: DeviceCode) -> Result<DeviceCode, DomainError> {
            self.0.lock().unwrap().insert(dc.id(), dc.clone());
            Ok(dc)
        }
        async fn get_by_device_code(&self, code: &str) -> Result<DeviceCode, DomainError> {
            let m = self.0.lock().unwrap();
            m.values()
                .find(|d| d.device_code() == code)
                .cloned()
                .ok_or_else(introuvable)
        }
        async fn get_pending_by_user_code(&self, code: &str) -> Result<DeviceCode, DomainError> {
            let m = self.0.lock().unwrap();
            m.values()
                .find(|d| {
                    d.user_code() == code
                        && d.status() == DeviceCodeStatus::Pending
                        && !d.is_expired()
                })
                .cloned()
                .ok_or_else(introuvable)
        }
        async fn update_device_code(&self, dc: DeviceCode) -> Result<(), DomainError> {
            self.0.lock().unwrap().insert(dc.id(), dc);
            Ok(())
        }
        async fn delete_expired(&self) -> Result<u64, DomainError> {
            Ok(0)
        }
        async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<DeviceCode>, DomainError> {
            let m = self.0.lock().unwrap();
            Ok(m.values()
                .filter(|d| d.user_id() == Some(user_id))
                .cloned()
                .collect())
        }
        async fn delete_by_id(&self, id: Uuid) -> Result<(), DomainError> {
            self.0.lock().unwrap().remove(&id);
            Ok(())
        }
    }

    fn bloquer<F: std::future::Future>(f: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(f)
    }

    fn requete(cle: Option<String>, scope: &str) -> DeviceAuthorizeRequestDto {
        DeviceAuthorizeRequestDto {
            client_name: "rclone".to_string(),
            scope: scope.to_string(),
            device_public_key: cle,
        }
    }

    #[test]
    fn device_flow_end_to_end_registers_key_and_issues_no_bearer() {
        let codes = Arc::new(MemoireCodes::default());
        let cles = Arc::new(MemoireIdentites::default());
        let flux = DeviceAuthService::new(codes, cles.clone(), "https://cloud.test".into());
        let verif = MiyuAuthService::new(cles, None).unwrap();
        let appareil = CleSignature::generer();
        let user = Uuid::new_v4();

        // 1. L'appareil démarre le flux avec sa clé publique.
        let debut = bloquer(flux.initiate(requete(
            Some(b64_encoder(&appareil.cle_publique())),
            "webdav, caldav",
        )))
        .unwrap();
        // 2. En attente.
        assert!(matches!(
            bloquer(flux.poll(&debut.device_code)),
            Err(DevicePollError::AuthorizationPending)
        ));
        assert!(
            bloquer(flux.verify_user_code(&debut.user_code))
                .unwrap()
                .valid
        );
        // 3. L'utilisateur approuve.
        bloquer(flux.approve(&debut.user_code, user)).unwrap();
        // 4. Fin du flux : aucun jeton, la clé est enregistrée.
        let fin = bloquer(flux.poll(&debut.device_code)).unwrap();
        assert_eq!(fin.token_type, "MiyuSignature");
        assert_eq!(fin.usage, USAGE_APPAREIL);
        assert_eq!(fin.scope, "webdav,caldav");
        assert_eq!(fin.device_public_key, b64_encoder(&appareil.cle_publique()));
        let json = serde_json::to_value(&fin).unwrap();
        assert!(json.get("access_token").is_none() && json.get("refresh_token").is_none());
        // Le code a servi : un second relevé échoue.
        assert!(matches!(
            bloquer(flux.poll(&debut.device_code)),
            Err(DevicePollError::InvalidDeviceCode)
        ));

        // 5. L'appareil signe ses requêtes : acceptée.
        let t = maintenant();
        let e = signer_requete(&appareil, USAGE_APPAREIL, "PROPFIND", "/webdav/", b"", t);
        let h = e.en_entetes();
        let r = RequeteProuvee {
            methode: "PROPFIND",
            chemin: "/webdav/",
            chemin_et_requete: "/webdav/",
            corps: b"",
        };
        let (en, cle) = bloquer(verif.preparer_appareil(
            Some(&h[0].1),
            Some(&h[1].1),
            Some(&h[2].1),
            Some(&h[3].1),
            t,
        ))
        .unwrap();
        assert_eq!(cle.user_id, user);
        assert_eq!(cle.id.to_string(), fin.device_id);
        verif.verifier_appareil(&en, r, t).unwrap();

        // 6. Révocation depuis le profil : refusée ensuite.
        let liste = bloquer(flux.list_user_devices(user)).unwrap();
        assert_eq!(liste.len(), 1);
        bloquer(flux.revoke_device(cle.id, user)).unwrap();
        assert!(bloquer(flux.list_user_devices(user)).unwrap().is_empty());
        let e2 = signer_requete(&appareil, USAGE_APPAREIL, "PROPFIND", "/webdav/", b"", t);
        let h2 = e2.en_entetes();
        assert!(matches!(
            bloquer(verif.preparer_appareil(
                Some(&h2[0].1),
                Some(&h2[1].1),
                Some(&h2[2].1),
                Some(&h2[3].1),
                t
            )),
            Err(RefusMiyu::AppareilInconnu)
        ));
        // Un autre utilisateur ne peut pas révoquer la clé d'autrui.
        assert!(bloquer(flux.revoke_device(cle.id, Uuid::new_v4())).is_err());
    }

    #[test]
    fn device_flow_requires_a_public_key_and_known_scopes() {
        let flux = DeviceAuthService::new(
            Arc::new(MemoireCodes::default()),
            Arc::new(MemoireIdentites::default()),
            "https://cloud.test".into(),
        );
        assert!(bloquer(flux.initiate(requete(None, "webdav"))).is_err());
        assert!(bloquer(flux.initiate(requete(Some("pas une clé".into()), "webdav"))).is_err());
        let cle = Some(b64_encoder(&CleSignature::generer().cle_publique()));
        assert!(bloquer(flux.initiate(requete(cle.clone(), "admin"))).is_err());
        assert!(bloquer(flux.initiate(requete(cle.clone(), ""))).is_err());
        assert!(bloquer(flux.initiate(requete(cle, "webdav"))).is_ok());
    }

    #[test]
    fn denied_flow_registers_nothing() {
        let cles = Arc::new(MemoireIdentites::default());
        let flux = DeviceAuthService::new(
            Arc::new(MemoireCodes::default()),
            cles.clone(),
            "https://cloud.test".into(),
        );
        let appareil = CleSignature::generer();
        let debut = bloquer(flux.initiate(requete(
            Some(hex::encode(appareil.cle_publique())),
            "webdav",
        )))
        .unwrap();
        bloquer(flux.deny(&debut.user_code)).unwrap();
        assert!(matches!(
            bloquer(flux.poll(&debut.device_code)),
            Err(DevicePollError::AccessDenied)
        ));
        assert!(
            bloquer(cles.active_device_key(&appareil.cle_publique()))
                .unwrap()
                .is_none()
        );
    }
}
