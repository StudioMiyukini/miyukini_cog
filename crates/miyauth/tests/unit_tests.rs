//! Tests unitaires MiyuAuth — toolkit.identity.miyauth.
//!
//! Tests des types d'identité, erreurs, contexte gouverné et outils.
//!
//! @id: miyauth_unit_tests
//! @role: test
//! @layer: toolkit

use miyauth::{
    identity_attest, identity_resolve, identity_role, identity_verify, GovernedContext,
    IdentityArtefacts, IdentityContext, IdentityRole, MiyauthError, VerificationResult,
};

// ═══════════════════════════════════════════════════════════════════════════
// Helpers de test
// ═══════════════════════════════════════════════════════════════════════════

fn governed_ctx() -> GovernedContext {
    GovernedContext::new("mandate-test-001".to_string(), 2)
}

fn empty_ctx() -> GovernedContext {
    GovernedContext::new(String::new(), 2)
}

fn sample_identity_context(role: IdentityRole) -> IdentityContext {
    IdentityContext {
        role,
        opaque_id: "user-opaque-id-12345".to_string(),
    }
}

fn sample_artefacts() -> IdentityArtefacts {
    IdentityArtefacts {
        passport_raw: Some(b"passport-data".to_vec()),
        visa_raw: Some(b"visa-data".to_vec()),
        session_ref: Some("session-abc-123".to_string()),
    }
}

/// Passeport/Visa mock au format MiyuAuth v1 : [0x01][expiry_nanos 8 bytes LE]. expiry=0 = pas d'expiration.
fn mock_passport_valid_no_expiry() -> Vec<u8> {
    let mut b = vec![0x01u8];
    b.extend_from_slice(&0u64.to_le_bytes());
    b
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests du contexte gouverné (GovernedContext)
// ═══════════════════════════════════════════════════════════════════════════

/// @id: MAUTH_CTX_001
/// @role: test
/// @layer: toolkit
/// @human: GovernedContext création - vérifie la création avec mandat.
#[test]
fn mauth_ctx_001_context_with_mandate() {
    let ctx = GovernedContext::new("mandate-123".to_string(), 3);

    assert!(ctx.has_mandate());
    assert_eq!(ctx.mandate_id, "mandate-123");
    assert_eq!(ctx.security_level, 3);
}

/// @id: MAUTH_CTX_002
/// @role: test
/// @layer: toolkit
/// @human: GovernedContext sans mandat - vérifie le comportement sans mandat.
#[test]
fn mauth_ctx_002_context_without_mandate() {
    let ctx = GovernedContext::new(String::new(), 1);

    assert!(!ctx.has_mandate());
    assert!(ctx.mandate_id.is_empty());
}

/// @id: MAUTH_CTX_003
/// @role: test
/// @layer: toolkit
/// @human: GovernedContext niveaux de sécurité - vérifie les différents niveaux.
#[test]
fn mauth_ctx_003_security_levels() {
    for level in 0..=5 {
        let ctx = GovernedContext::new("mandate".to_string(), level);
        assert_eq!(ctx.security_level, level);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests des types d'identité
// ═══════════════════════════════════════════════════════════════════════════

/// @id: MAUTH_T_001
/// @role: test
/// @layer: toolkit
/// @human: IdentityRole variants - vérifie les trois rôles.
#[test]
fn mauth_t_001_identity_role_variants() {
    let citizen = IdentityRole::Citizen;
    let visitor = IdentityRole::Visitor;
    let external = IdentityRole::External;

    // Les rôles sont distincts
    assert_ne!(citizen, visitor);
    assert_ne!(visitor, external);
    assert_ne!(citizen, external);

    // Clone fonctionne
    let citizen_clone = citizen;
    assert_eq!(citizen, citizen_clone);
}

/// @id: MAUTH_T_002
/// @role: test
/// @layer: toolkit
/// @human: IdentityContext création - vérifie la structure.
#[test]
fn mauth_t_002_identity_context_creation() {
    let context = sample_identity_context(IdentityRole::Citizen);

    assert_eq!(context.role, IdentityRole::Citizen);
    assert!(!context.opaque_id.is_empty());
}

/// @id: MAUTH_T_003
/// @role: test
/// @layer: toolkit
/// @human: IdentityArtefacts création - vérifie tous les champs optionnels.
#[test]
fn mauth_t_003_identity_artefacts_all_fields() {
    let artefacts = sample_artefacts();

    assert!(artefacts.passport_raw.is_some());
    assert!(artefacts.visa_raw.is_some());
    assert!(artefacts.session_ref.is_some());
}

/// @id: MAUTH_T_004
/// @role: test
/// @layer: toolkit
/// @human: IdentityArtefacts vide - vérifie le Default.
#[test]
fn mauth_t_004_identity_artefacts_empty() {
    let artefacts = IdentityArtefacts::default();

    assert!(artefacts.passport_raw.is_none());
    assert!(artefacts.visa_raw.is_none());
    assert!(artefacts.session_ref.is_none());
}

/// @id: MAUTH_T_005
/// @role: test
/// @layer: toolkit
/// @human: VerificationResult variants - vérifie les trois résultats.
#[test]
fn mauth_t_005_verification_result_variants() {
    let valid = VerificationResult::Valid;
    let invalid = VerificationResult::Invalid;
    let expired = VerificationResult::Expired;

    assert_ne!(valid, invalid);
    assert_ne!(valid, expired);
    assert_ne!(invalid, expired);
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests des erreurs
// ═══════════════════════════════════════════════════════════════════════════

/// @id: MAUTH_E_001
/// @role: test
/// @layer: toolkit
/// @human: MiyauthError::NoMandate display - vérifie le message.
#[test]
fn mauth_e_001_error_no_mandate_display() {
    let err = MiyauthError::NoMandate;
    let msg = err.to_string();

    assert!(msg.contains("no governed mandate"));
}

/// @id: MAUTH_E_002
/// @role: test
/// @layer: toolkit
/// @human: MiyauthError::Unimplemented display - vérifie le message.
#[test]
fn mauth_e_002_error_unimplemented_display() {
    let err = MiyauthError::Unimplemented;
    let msg = err.to_string();

    assert!(msg.contains("not yet implemented"));
}

/// @id: MAUTH_E_003
/// @role: test
/// @layer: toolkit
/// @human: MiyauthError Clone - vérifie que Clone fonctionne.
#[test]
fn mauth_e_003_error_clone() {
    let err = MiyauthError::NoMandate;
    let err_clone = err.clone();

    // Les deux ont le même message
    assert_eq!(err.to_string(), err_clone.to_string());
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests des outils - Gouvernance (NoMandate)
// ═══════════════════════════════════════════════════════════════════════════

/// @id: MAUTH_TOOL_001
/// @role: test
/// @layer: toolkit
/// @human: identity_resolve sans mandat - vérifie le refus.
#[test]
fn mauth_tool_001_resolve_no_mandate() {
    let ctx = empty_ctx();
    let artefacts = sample_artefacts();

    let result = identity_resolve(&ctx, &artefacts);

    assert!(matches!(result, Err(MiyauthError::NoMandate)));
}

/// @id: MAUTH_TOOL_002
/// @role: test
/// @layer: toolkit
/// @human: identity_verify sans mandat - vérifie le refus.
#[test]
fn mauth_tool_002_verify_no_mandate() {
    let ctx = empty_ctx();
    let passport_data = b"passport-raw-data";

    let result = identity_verify(&ctx, passport_data);

    assert!(matches!(result, Err(MiyauthError::NoMandate)));
}

/// @id: MAUTH_TOOL_003
/// @role: test
/// @layer: toolkit
/// @human: identity_attest sans mandat - vérifie le refus.
#[test]
fn mauth_tool_003_attest_no_mandate() {
    let ctx = empty_ctx();
    let identity_ctx = sample_identity_context(IdentityRole::Citizen);

    let result = identity_attest(&ctx, &identity_ctx);

    assert!(matches!(result, Err(MiyauthError::NoMandate)));
}

/// @id: MAUTH_TOOL_004
/// @role: test
/// @layer: toolkit
/// @human: identity_role sans mandat - vérifie le refus.
#[test]
fn mauth_tool_004_role_no_mandate() {
    let ctx = empty_ctx();
    let identity_ctx = sample_identity_context(IdentityRole::Visitor);

    let result = identity_role(&ctx, &identity_ctx);

    assert!(matches!(result, Err(MiyauthError::NoMandate)));
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests des outils - Avec mandat
// ═══════════════════════════════════════════════════════════════════════════

/// @id: MAUTH_TOOL_010
/// @role: test
/// @layer: toolkit
/// @human: identity_resolve avec mandat - vérifie résolution (Citizen car Passeport présent).
#[test]
fn mauth_tool_010_resolve_with_mandate() {
    let ctx = governed_ctx();
    let artefacts = sample_artefacts();

    let result = identity_resolve(&ctx, &artefacts);

    assert!(result.is_ok());
    let context = result.unwrap();
    assert_eq!(context.role, IdentityRole::Citizen);
    assert!(!context.opaque_id.is_empty());
    assert_eq!(context.opaque_id, "session-abc-123");
}

/// @id: MAUTH_TOOL_011
/// @role: test
/// @layer: toolkit
/// @human: identity_verify avec mandat - vérifie Passeport mock valide (format v1, pas d'expiration).
#[test]
fn mauth_tool_011_verify_with_mandate() {
    let ctx = governed_ctx();
    let passport_data = mock_passport_valid_no_expiry();

    let result = identity_verify(&ctx, &passport_data);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), VerificationResult::Valid);
}

/// @id: MAUTH_TOOL_012
/// @role: test
/// @layer: toolkit
/// @human: identity_attest avec mandat - vérifie production d'une attestation.
#[test]
fn mauth_tool_012_attest_with_mandate() {
    let ctx = governed_ctx();
    let identity_ctx = sample_identity_context(IdentityRole::Citizen);

    let result = identity_attest(&ctx, &identity_ctx);

    assert!(result.is_ok());
    let att = result.unwrap();
    assert!(!att.attestation_id.is_empty());
}

/// @id: MAUTH_TOOL_013
/// @role: test
/// @layer: toolkit
/// @human: identity_role avec mandat - vérifie le retour du rôle.
#[test]
fn mauth_tool_013_role_with_mandate() {
    let ctx = governed_ctx();

    // Test avec chaque rôle
    for role in [
        IdentityRole::Citizen,
        IdentityRole::Visitor,
        IdentityRole::External,
    ] {
        let identity_ctx = sample_identity_context(role);
        let result = identity_role(&ctx, &identity_ctx);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), role);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests de robustesse
// ═══════════════════════════════════════════════════════════════════════════

/// @id: MAUTH_R_001
/// @role: test
/// @layer: toolkit
/// @human: Artefacts avec données vides - résolution en External, opaque_id généré.
#[test]
fn mauth_r_001_artefacts_empty_data() {
    let ctx = governed_ctx();
    let artefacts = IdentityArtefacts {
        passport_raw: Some(vec![]), // Passeport vide
        visa_raw: None,
        session_ref: Some(String::new()), // Session vide
    };

    let result = identity_resolve(&ctx, &artefacts);

    assert!(result.is_ok());
    let context = result.unwrap();
    assert_eq!(context.role, IdentityRole::External);
    assert!(!context.opaque_id.is_empty());
    assert!(context.opaque_id.starts_with("opaque:"));
}

/// @id: MAUTH_R_002
/// @role: test
/// @layer: toolkit
/// @human: Vérification avec données binaires vides - retourne Invalid.
#[test]
fn mauth_r_002_verify_empty_data() {
    let ctx = governed_ctx();

    let result = identity_verify(&ctx, &[]);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), VerificationResult::Invalid);
}

/// @id: MAUTH_R_003
/// @role: test
/// @layer: toolkit
/// @human: IdentityContext avec opaque_id vide - vérifie le comportement.
#[test]
fn mauth_r_003_identity_context_empty_opaque_id() {
    let ctx = governed_ctx();
    let identity_ctx = IdentityContext {
        role: IdentityRole::External,
        opaque_id: String::new(),
    };

    let result = identity_role(&ctx, &identity_ctx);

    // Le rôle doit toujours être retourné
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), IdentityRole::External);
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests Debug trait
// ═══════════════════════════════════════════════════════════════════════════

/// @id: MAUTH_D_001
/// @role: test
/// @layer: toolkit
/// @human: Debug sur IdentityRole - vérifie l'affichage.
#[test]
fn mauth_d_001_debug_identity_role() {
    let citizen = IdentityRole::Citizen;
    let debug_str = format!("{:?}", citizen);

    assert!(debug_str.contains("Citizen"));
}

/// @id: MAUTH_D_002
/// @role: test
/// @layer: toolkit
/// @human: Debug sur VerificationResult - vérifie l'affichage.
#[test]
fn mauth_d_002_debug_verification_result() {
    let valid = VerificationResult::Valid;
    let debug_str = format!("{:?}", valid);

    assert!(debug_str.contains("Valid"));
}

/// @id: MAUTH_D_003
/// @role: test
/// @layer: toolkit
/// @human: Debug sur MiyauthError - vérifie l'affichage.
#[test]
fn mauth_d_003_debug_error() {
    let err = MiyauthError::NoMandate;
    let debug_str = format!("{:?}", err);

    assert!(debug_str.contains("NoMandate"));
}
