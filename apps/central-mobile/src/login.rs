//! @id     central_mobile_login
//! @do     authentifier_par_passeport_appareil
//! @role   security
//! @layer  app
//! @human  central-mobile — connexion fail-closed : E2E puis passeport d'appareil + visa (aucun mot de passe).

//! Logique de connexion au COG, séparée de l'UI pour être testable (R-023).
//!
//! Sortie du lot 2 : plus de courriel + mot de passe ni de jeton au porteur.
//! L'appareil s'authentifie avec la clé d'identité épinglée à l'appairage :
//! passeport d'appareil, visa (≤ 15 min) et preuve de possession à chaque
//! requête (`miyukini_cog_bridge::identite`). Le titulaire est le profil Central
//! qui a approuvé l'appareil.
//!
//! Règle (C-12, fail-closed) : la session E2E v2 (clé du COG épinglée) est
//! établie d'abord ; si le handshake échoue, la connexion est abandonnée sans
//! repli.

use miyukini_cog_bridge::{parse_identity_key_hex, CogBridge, ConnectionMode, E2eIdentity};

use crate::state::SavedConnection;

/// Canal vers le COG utilisé pour la connexion (abstrait pour les tests).
pub trait AuthChannel {
    /// Négocie la session E2E v2 (handshake signé, clé du COG épinglée).
    async fn negotiate_e2e(&self) -> Result<(), String>;
    /// Vrai si une session E2E est établie.
    async fn e2e_active(&self) -> bool;
    /// Ouvre la session de l'appareil (passeport → visa → validation) et rend
    /// le nom affiché du titulaire.
    async fn open_device_session(&self) -> Result<String, String>;
}

/// Causes d'échec de la connexion (une variante par cause).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginError {
    /// Le handshake E2E a échoué : aucune requête d'authentification émise.
    E2eFailed(String),
    /// Aucune session E2E active après le handshake.
    E2eInactive,
    /// Le COG a refusé l'appareil (non appairé, révoqué) ou est injoignable.
    Rejected(String),
}

impl LoginError {
    /// Message affiché à l'utilisateur (ne contient aucun secret).
    pub fn user_message(&self) -> String {
        match self {
            Self::E2eFailed(e) => format!(
                "Connexion sécurisée impossible avec le COG Host ({e}). \
                 Ré-appairez l'appareil si le problème persiste."
            ),
            Self::E2eInactive => "Connexion sécurisée non établie.".to_string(),
            Self::Rejected(e) => format!("Appareil refusé par le COG Host : {e}"),
        }
    }
}

/// Connexion fail-closed : handshake E2E, puis session d'appareil.
///
/// Retourne le nom affiché du titulaire.
pub async fn login<C: AuthChannel>(channel: &C) -> Result<String, LoginError> {
    channel.negotiate_e2e().await.map_err(LoginError::E2eFailed)?;
    if !channel.e2e_active().await {
        return Err(LoginError::E2eInactive);
    }
    channel.open_device_session().await.map_err(LoginError::Rejected)
}

impl AuthChannel for CogBridge {
    async fn negotiate_e2e(&self) -> Result<(), String> {
        CogBridge::negotiate_e2e(self).await.map_err(|e| e.to_string())
    }

    async fn e2e_active(&self) -> bool {
        self.is_e2e_active().await
    }

    async fn open_device_session(&self) -> Result<String, String> {
        self.connect().await.map_err(|e| e.to_string())?;
        Ok(self
            .session_appareil()
            .await
            .map(|s| s.display_name)
            .unwrap_or_default())
    }
}

/// Construit le bridge depuis la connexion sauvegardée : mode de transport et
/// identités (identité de l'appareil + clé du COG épinglée à l'appairage).
/// Sans clé épinglée (appairage antérieur au E2E v2) ou mode « clé API »
/// (retiré) : ré-appairage requis.
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
        "apikey" => {
            return Err("Mode clé API retiré : ré-appairez l'appareil avec le COG Host.".to_string())
        }
        other => return Err(format!("Mode {other} pas encore supporté")),
    };
    Ok(CogBridge::new(mode).with_e2e_identities(device, cog_key))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    /// Transport simulé : compte les ouvertures de session émises.
    struct MockChannel {
        handshake_ok: bool,
        active_after_handshake: bool,
        accepted: bool,
        session_calls: Cell<u32>,
        handshake_calls: Cell<u32>,
    }

    impl MockChannel {
        fn new(handshake_ok: bool, active: bool, accepted: bool) -> Self {
            Self {
                handshake_ok,
                active_after_handshake: active,
                accepted,
                session_calls: Cell::new(0),
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

        async fn open_device_session(&self) -> Result<String, String> {
            self.session_calls.set(self.session_calls.get() + 1);
            if self.accepted {
                Ok("Alice".into())
            } else {
                Err("Appareil non autorisé (non appairé ou révoqué)".into())
            }
        }
    }

    fn run<F: std::future::Future>(f: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(f)
    }

    #[test]
    fn handshake_failure_opens_no_session() {
        let chan = MockChannel::new(false, false, true);
        let err = run(login(&chan)).unwrap_err();
        assert!(matches!(err, LoginError::E2eFailed(_)));
        assert_eq!(chan.handshake_calls.get(), 1);
        assert_eq!(chan.session_calls.get(), 0, "session ouverte sans E2E");
    }

    #[test]
    fn inactive_session_opens_no_session() {
        let chan = MockChannel::new(true, false, true);
        assert_eq!(run(login(&chan)).unwrap_err(), LoginError::E2eInactive);
        assert_eq!(chan.session_calls.get(), 0);
    }

    #[test]
    fn successful_device_login() {
        let chan = MockChannel::new(true, true, true);
        assert_eq!(run(login(&chan)).unwrap(), "Alice");
        assert_eq!(chan.session_calls.get(), 1);
    }

    #[test]
    fn revoked_device_reported() {
        let chan = MockChannel::new(true, true, false);
        let err = run(login(&chan)).unwrap_err();
        assert!(matches!(err, LoginError::Rejected(_)));
        assert!(err.user_message().contains("refusé"));
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
    fn bridge_requires_pinned_cog_key_and_refuses_api_key_mode() {
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
        assert!(build_bridge(&saved(Some(cog.clone()), "ssh"), E2eIdentity::generate()).is_err());
        let err = build_bridge(&saved(Some(cog), "apikey"), E2eIdentity::generate())
            .err()
            .unwrap();
        assert!(err.contains("clé API retiré"));
    }

    #[test]
    fn real_bridge_without_cog_refuses_session() {
        // COG injoignable : aucune session (pas de repli).
        let cog = hex::encode(E2eIdentity::generate().public_key_bytes());
        let mut s = saved(Some(cog), "whitelist");
        s.host = "127.0.0.1".into();
        s.port = 9;
        let Ok(bridge) = build_bridge(&s, E2eIdentity::generate()) else {
            panic!("bridge attendu");
        };
        assert!(!run(bridge.e2e_active()));
        assert!(run(bridge.open_device_session()).is_err());
    }
}
