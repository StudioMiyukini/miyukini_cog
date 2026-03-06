#![allow(missing_docs)]
//! Miyukini Connect local-first authentication service.
//!
//! @id: miyukini_connect_lib
//! @do: expose_connect_auth_modules
//! @role: auth
//! @layer: service

pub mod audit;
pub mod errors;
pub mod policy;
pub mod service;
pub mod types;

pub use errors::ConnectError;
pub use service::{ConnectService, IdentitySetup};
pub use types::{
    AuthMethod, AuthSession, AuthVerifyRequest, AuthorizeHint, OriginProbeResult,
    OriginProbeStatus, PermissionTier, RuntimeState,
};

#[cfg(test)]
mod tests {
    use super::*;
    use totp_rs::{Algorithm, TOTP};

    fn seeded_service() -> ConnectService {
        let mut service = ConnectService::new("2026.03.05");
        let mut identity = IdentitySetup::new("alice");
        identity.set_password("pw-Strong!42").expect("password hash");
        identity.enable_totp(vec![1_u8; 20]);
        identity.enable_passkey();
        identity.enable_hardware_key();
        service.register_identity(identity).expect("register");
        service
    }

    fn current_totp(secret: &[u8]) -> String {
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret.to_vec(),
            Some("Miyukini".to_string()),
            "alice".to_string(),
        )
        .expect("totp init");
        totp.generate_current().expect("totp")
    }

    #[test]
    fn e01_offline_login_totp_and_introspect() {
        let mut service = seeded_service();
        service.set_runtime_state(RuntimeState::Isolated);

        let code = current_totp(&[1_u8; 20]);
        let request = AuthVerifyRequest {
            subject_id: "alice".to_string(),
            password: "pw-Strong!42".to_string(),
            totp_code: Some(code),
            requested_tier: PermissionTier::StandardWrite,
            runtime_state: RuntimeState::Isolated,
        };

        let session = service.auth_verify(request).expect("login");
        assert_eq!(session.aal, 2);
        assert!(session.methods.contains(&AuthMethod::Password));
        assert!(session.methods.contains(&AuthMethod::Totp));

        let introspected = service
            .session_introspect(&session.session_id)
            .expect("introspect")
            .expect("present");
        assert_eq!(introspected.subject_id, "alice");
        assert_eq!(introspected.permission_tier, PermissionTier::StandardWrite);
    }

    #[test]
    fn e02_step_up_rotates_session_id_and_unlocks_sensitive_write() {
        let mut service = seeded_service();
        let code = current_totp(&[1_u8; 20]);

        let base = service
            .auth_verify(AuthVerifyRequest {
                subject_id: "alice".to_string(),
                password: "pw-Strong!42".to_string(),
                totp_code: Some(code),
                requested_tier: PermissionTier::SensitiveWrite,
                runtime_state: RuntimeState::OnlineFull,
            })
            .expect_err("must require step-up");
        assert!(matches!(base, ConnectError::StepUpRequired { .. }));

        let standard = service
            .auth_verify(AuthVerifyRequest {
                subject_id: "alice".to_string(),
                password: "pw-Strong!42".to_string(),
                totp_code: Some(current_totp(&[1_u8; 20])),
                requested_tier: PermissionTier::StandardWrite,
                runtime_state: RuntimeState::OnlineFull,
            })
            .expect("base login");

        let elevated = service
            .auth_step_up(&standard.session_id, AuthMethod::Passkey)
            .expect("step-up");
        assert_ne!(elevated.session_id, standard.session_id);
        assert!(elevated.aal >= 3);

        let hint = service
            .session_authorize_hint(&elevated.session_id, PermissionTier::SensitiveWrite)
            .expect("hint");
        assert!(hint.allowed);
        assert!(!hint.step_up_required);
    }

    #[test]
    fn e03_origin_probe_switches_runtime_states() {
        let mut service = seeded_service();
        let bootstrap = service.bootstrap();
        assert_eq!(bootstrap.runtime_state, RuntimeState::Isolated);

        service.origin_ping(OriginProbeResult {
            status: OriginProbeStatus::Ok,
            latency_ms: Some(90),
            capabilities: vec!["qr_login".to_string(), "email_otp".to_string()],
        });
        assert_eq!(service.bootstrap().runtime_state, RuntimeState::OnlineFull);

        service.origin_ping(OriginProbeResult {
            status: OriginProbeStatus::Degraded,
            latency_ms: Some(220),
            capabilities: vec!["email_otp".to_string()],
        });
        assert_eq!(service.bootstrap().runtime_state, RuntimeState::OnlineDegraded);

        service.origin_ping(OriginProbeResult {
            status: OriginProbeStatus::Unavailable,
            latency_ms: None,
            capabilities: Vec::new(),
        });
        assert_eq!(service.bootstrap().runtime_state, RuntimeState::Isolated);
    }

    #[test]
    fn e04_isolated_blocks_weak_factors_and_suspicious_hardens_policy() {
        let mut service = seeded_service();
        service.set_runtime_state(RuntimeState::Isolated);

        assert!(!service.enrollment_allowed(AuthMethod::EmailOtp));
        assert!(!service.enrollment_allowed(AuthMethod::QrSigned));
        assert!(service.enrollment_allowed(AuthMethod::Passkey));

        service.report_anomaly();
        assert_eq!(service.current_runtime_state(), RuntimeState::Suspicious);

        let hint = service
            .session_authorize_hint_for_aal(2, PermissionTier::SensitiveRead)
            .expect("hint");
        assert_eq!(hint.required_aal, 4);
        assert!(!hint.allowed);
    }

    #[test]
    fn e04_audit_chain_detects_tampering() {
        let mut service = seeded_service();
        service
            .append_audit("manual_event", Some("alice"), None, "payload")
            .expect("audit");
        service
            .append_audit("manual_event_2", Some("alice"), None, "payload-2")
            .expect("audit");
        assert!(service.verify_audit_chain());

        service.force_audit_tamper_for_test("bad-hash".to_string());
        assert!(!service.verify_audit_chain());
    }

    #[test]
    fn c1_password_is_argon2id_and_c3_integrity_fingerprint_rotates_on_step_up() {
        let mut service = seeded_service();
        let password_hash = service.password_hash_for("alice").expect("hash exists");
        assert!(password_hash.starts_with("$argon2id$"));

        let first = service
            .auth_verify(AuthVerifyRequest {
                subject_id: "alice".to_string(),
                password: "pw-Strong!42".to_string(),
                totp_code: Some(current_totp(&[1_u8; 20])),
                requested_tier: PermissionTier::StandardWrite,
                runtime_state: RuntimeState::OnlineFull,
            })
            .expect("login");
        let second = service
            .auth_step_up(&first.session_id, AuthMethod::Passkey)
            .expect("step-up");
        assert_ne!(first.integrity_fingerprint, second.integrity_fingerprint);
    }

    #[test]
    fn p4_lockout_triggers_after_repeated_failed_auth() {
        let mut service = seeded_service();
        for _ in 0..5 {
            let result = service.auth_verify(AuthVerifyRequest {
                subject_id: "alice".to_string(),
                password: "wrong-password".to_string(),
                totp_code: Some(current_totp(&[1_u8; 20])),
                requested_tier: PermissionTier::StandardWrite,
                runtime_state: RuntimeState::OnlineFull,
            });
            assert!(matches!(result, Err(ConnectError::InvalidCredentials)));
        }

        let locked = service.auth_verify(AuthVerifyRequest {
            subject_id: "alice".to_string(),
            password: "pw-Strong!42".to_string(),
            totp_code: Some(current_totp(&[1_u8; 20])),
            requested_tier: PermissionTier::StandardWrite,
            runtime_state: RuntimeState::OnlineFull,
        });
        assert!(matches!(locked, Err(ConnectError::LockedOut { .. })));
    }

    #[test]
    fn p4_expired_session_is_rejected_by_authorize_hint() {
        let mut service = seeded_service();
        let session = service
            .auth_verify(AuthVerifyRequest {
                subject_id: "alice".to_string(),
                password: "pw-Strong!42".to_string(),
                totp_code: Some(current_totp(&[1_u8; 20])),
                requested_tier: PermissionTier::StandardWrite,
                runtime_state: RuntimeState::OnlineFull,
            })
            .expect("login");
        service.force_session_expire_for_test(&session.session_id);
        let result =
            service.session_authorize_hint(&session.session_id, PermissionTier::StandardWrite);
        assert!(matches!(result, Err(ConnectError::SessionExpired)));
    }

    #[test]
    fn qr_attested_session_requires_online_full_and_grants_aal3() {
        let mut service = seeded_service();
        service.set_runtime_state(RuntimeState::Isolated);
        let blocked =
            service.auth_issue_attested_session("alice", PermissionTier::SensitiveRead, AuthMethod::QrSigned);
        assert!(matches!(blocked, Err(ConnectError::FactorNotAllowed(AuthMethod::QrSigned))));

        service.set_runtime_state(RuntimeState::OnlineFull);
        let session = service
            .auth_issue_attested_session("alice", PermissionTier::SensitiveRead, AuthMethod::QrSigned)
            .expect("qr session");
        assert_eq!(session.aal, 3);
        assert_eq!(session.methods, vec![AuthMethod::QrSigned]);
    }
}
