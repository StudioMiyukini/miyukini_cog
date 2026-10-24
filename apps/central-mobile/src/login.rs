//! @id     central_mobile_login
//! @do     authentifier_via_session_e2e
//! @role   security
//! @layer  app
//! @human  central-mobile — logique de connexion fail-closed (E2E obligatoire).

//! Logique de connexion au COG, séparée de l'UI pour être testable (R-023).
//!
//! Règle (C-12, fail-closed) : **aucun identifiant ne quitte l'appareil sans
//! session E2E v2 établie**. Si le handshake échoue, la connexion est abandonnée
//! avec une erreur lisible — il n'existe aucun repli en clair. La requête
//! `/auth` n'est émise que chiffrée et liée (`POST /auth`) dans la session.

use central_remote_protocol::{RemoteAuthRequest, RemoteAuthResponse};
use miyukini_cog_bridge::{parse_identity_key_hex, CogBridge, ConnectionMode, E2eIdentity};

use crate::state::SavedConnection;

/// Point d'accès d'authentification du COG (lié dans l'AAD E2E).
pub const AUTH_ENDPOINT: &str = "/auth";

/// Canal vers le COG utilisé pour la connexion (abstrait pour les tests).
pub trait AuthChannel {
    /// Négocie la session E2E v2 (handshake signé, clé du COG épinglée).
    async fn negotiate_e2e(&self) -> Result<(), String>;
    /// Vrai si une session E2E est établie.
    async fn e2e_active(&self) -> bool;
    /// Envoie la requête d'authentification **chiffrée** dans la session E2E.
    async fn authenticate_encrypted(
        &self,
        req: &RemoteAuthRequest,
    ) -> Result<RemoteAuthResponse, String>;
}

/// Causes d'échec de la connexion (une variante par cause).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginError {
    /// Le handshake E2E a échoué : rien n'a été envoyé.
    E2eFailed(String),
    /// Aucune session E2E active après le handshake : rien n'a été envoyé.
    E2eInactive,
    /// Erreur de transport pendant l'authentification chiffrée.
    Transport(String),
    /// Le COG a refusé les identifiants.
    Rejected(String),
}

impl LoginError {
    /// Message affiché à l'utilisateur (ne contient aucun secret).
    pub fn user_message(&self) -> String {
        match self {
            Self::E2eFailed(e) => format!(
                "Connexion sécurisée impossible avec le COG Host ({e}). \
                 Aucun identifiant n'a été envoyé. Ré-appairez l'appareil si le problème persiste."
            ),
            Self::E2eInactive => "Connexion sécurisée non établie : aucun identifiant n'a été envoyé."
                .to_string(),
            Self::Transport(e) => format!("Authentification impossible : {e}"),
            Self::Rejected(e) => e.clone(),
        }
    }
}

/// Connexion fail-closed : handshake E2E puis `/auth` chiffré uniquement.
///
/// Retourne le nom affiché de l'utilisateur.
pub async fn login<C: AuthChannel>(
    channel: &C,
    email: &str,
    password: &str,
) -> Result<String, LoginError> {
    channel.negotiate_e2e().await.map_err(LoginError::E2eFailed)?;
    if !channel.e2e_active().await {
        return Err(LoginError::E2eInactive);
    }
    let req = RemoteAuthRequest {
        email: email.trim().to_string(),
        password: password.to_string(),
    };
    let resp = channel
        .authenticate_encrypted(&req)
        .await
        .map_err(LoginError::Transport)?;
    if resp.success {
        Ok(resp
            .user_display_name
            .filter(|n| !n.is_empty())
            .unwrap_or_else(|| req.email.clone()))
    } else {
        Err(LoginError::Rejected(
            resp.error
                .unwrap_or_else(|| "Identifiants incorrects".to_string()),
        ))
    }
}

impl AuthChannel for CogBridge {
    async fn negotiate_e2e(&self) -> Result<(), String> {
        CogBridge::negotiate_e2e(self).await.map_err(|e| e.to_string())
    }

    async fn e2e_active(&self) -> bool {
        self.is_e2e_active().await
    }

    async fn authenticate_encrypted(
        &self,
        req: &RemoteAuthRequest,
    ) -> Result<RemoteAuthResponse, String> {
        self.request_encrypted(AUTH_ENDPOINT, req)
            .await
            .map_err(|e| e.to_string())
    }
}

/// Construit le bridge depuis la connexion sauvegardée : mode de transport et
/// identités E2E (identité de l'appareil + clé du COG épinglée à l'appairage).
/// Sans clé épinglée (appairage antérieur au E2E v2) : ré-appairage requis.
pub fn build_bridge(saved: &SavedConnection, device: E2eIdentity) -> Result<CogBridge, String> {
    let cog_key_hex = saved
        .cog_identity_key
        .as_deref()
        .ok_or_else(|| "Appairage E2E absent : ré-appairez l'appareil avec le COG Host.".to_string())?;
    let cog_key = parse_identity_key_hex(cog_key_hex)
        .map_err(|_| "Clé du COG Host invalide : ré-appairez l'appareil.".to_string())?;
    let mode = match saved.mode.as_str() {
        "whitelist" => ConnectionMode::Whitelist {
            host: saved.host.clone(),
            port: saved.port,
            device_id: saved.credential.clone(),
        },
        "apikey" => ConnectionMode::ApiKey {
            host: saved.host.clone(),
            port: saved.port,
            token: saved.credential.clone(),
        },
        other => return Err(format!("Mode {other} pas encore supporté")),
    };
    Ok(CogBridge::new(mode).with_e2e_identities(device, cog_key))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    /// Transport simulé : compte les requêtes d'authentification émises.
    struct MockChannel {
        handshake_ok: bool,
        active_after_handshake: bool,
        auth_success: bool,
        auth_calls: Cell<u32>,
        handshake_calls: Cell<u32>,
    }

    impl MockChannel {
        fn new(handshake_ok: bool, active: bool, auth_success: bool) -> Self {
            Self {
                handshake_ok,
                active_after_handshake: active,
                auth_success,
                auth_calls: Cell::new(0),
                handshake_calls: Cell::new(0),
            }
        }
    }

    #[allow(clippy::unused_async_trait_impl)] // Le trait est asynchrone ; le simulacre ne l'est pas.
    impl AuthChannel for MockChannel {
        async fn negotiate_e2e(&self) -> Result<(), String> {
            self.handshake_calls.set(self.handshake_calls.get() + 1);
            if self.handshake_ok {
                Ok(())
            } else {
                Err("signature de handshake invalide".into())
            }
        }

        async fn e2e_active(&self) -> bool {
            self.handshake_ok && self.active_after_handshake
        }

        async fn authenticate_encrypted(
            &self,
            req: &RemoteAuthRequest,
        ) -> Result<RemoteAuthResponse, String> {
            self.auth_calls.set(self.auth_calls.get() + 1);
            assert_eq!(req.email, "alice@example.org");
            Ok(RemoteAuthResponse {
                success: self.auth_success,
                token: self.auth_success.then(|| "tok".to_string()),
                user_display_name: self.auth_success.then(|| "Alice".to_string()),
                error: (!self.auth_success).then(|| "Identifiants incorrects".to_string()),
            })
        }
    }

    fn run<F: std::future::Future>(f: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(f)
    }

    #[test]
    fn handshake_failure_sends_no_credentials() {
        let chan = MockChannel::new(false, false, true);
        let err = run(login(&chan, "alice@example.org", "secret")).unwrap_err();
        assert!(matches!(err, LoginError::E2eFailed(_)));
        assert_eq!(chan.handshake_calls.get(), 1);
        assert_eq!(chan.auth_calls.get(), 0, "identifiants émis sans E2E");
        assert!(err.user_message().contains("Aucun identifiant"));
        assert!(!err.user_message().contains("secret"));
    }

    #[test]
    fn inactive_session_sends_no_credentials() {
        let chan = MockChannel::new(true, false, true);
        let err = run(login(&chan, "alice@example.org", "secret")).unwrap_err();
        assert_eq!(err, LoginError::E2eInactive);
        assert_eq!(chan.auth_calls.get(), 0);
    }

    #[test]
    fn successful_login_over_e2e() {
        let chan = MockChannel::new(true, true, true);
        let name = run(login(&chan, " alice@example.org ", "secret")).unwrap();
        assert_eq!(name, "Alice");
        assert_eq!(chan.auth_calls.get(), 1);
    }

    #[test]
    fn rejected_credentials_reported() {
        let chan = MockChannel::new(true, true, false);
        let err = run(login(&chan, "alice@example.org", "faux")).unwrap_err();
        assert_eq!(err, LoginError::Rejected("Identifiants incorrects".into()));
    }

    fn saved(cog_key: Option<String>, mode: &str) -> SavedConnection {
        SavedConnection {
            mode: mode.into(),
            host: "192.168.1.10".into(),
            port: 8091,
            credential: "device-id".into(),
            cog_identity_key: cog_key,
        }
    }

    #[test]
    fn bridge_requires_pinned_cog_key() {
        // Connexion sauvegardée avant le E2E v2 : ré-appairage requis.
        let err = build_bridge(&saved(None, "whitelist"), E2eIdentity::generate())
            .err()
            .unwrap();
        assert!(err.contains("ré-appairez"));
        let err = build_bridge(&saved(Some("zz".into()), "whitelist"), E2eIdentity::generate())
            .err()
            .unwrap();
        assert!(err.contains("invalide"));
        let cog = hex::encode(E2eIdentity::generate().public_key_bytes());
        assert!(build_bridge(&saved(Some(cog.clone()), "whitelist"), E2eIdentity::generate()).is_ok());
        assert!(build_bridge(&saved(Some(cog), "ssh"), E2eIdentity::generate()).is_err());
    }

    #[test]
    fn real_bridge_without_session_refuses_auth() {
        // Sans handshake, le bridge réel refuse d'émettre /auth (aucun clair).
        let cog = hex::encode(E2eIdentity::generate().public_key_bytes());
        let Ok(bridge) = build_bridge(&saved(Some(cog), "whitelist"), E2eIdentity::generate())
        else {
            panic!("bridge attendu");
        };
        let req = RemoteAuthRequest {
            email: "alice@example.org".into(),
            password: "secret".into(),
        };
        assert!(!run(bridge.e2e_active()));
        assert!(run(bridge.authenticate_encrypted(&req)).is_err());
    }
}
