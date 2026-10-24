//! @id     miyualicia_skills_test_policy_sans_bypass
//! @do     verifier_gate_cores_toujours_active
//! @role   test
//! @layer  7
//! @human  Sans la feature `dev-bypass`, la gate cores ne peut pas être contournée (K-16).
//!
//! Ces tests ne s'exécutent que lorsque `dev-bypass` est désactivée (cas de la
//! production). Si un autre paquet du même build l'active (unification des
//! features), ils sont simplement compilés hors du binaire de test.

#![cfg(not(feature = "dev-bypass"))]

use miyualicia_skills::AliciaPolicy;

/// @id miyualicia_skills_test_policy_defaut_gate_active
/// @role test
/// @layer 7
/// @do verify_default_policy_requires_cores
//@human / @human La politique par défaut exige la gate cores.
#[test]
fn politique_par_defaut_exige_les_cores() {
    assert!(AliciaPolicy::default().requires_cores_check());
    assert!(AliciaPolicy::strict().requires_cores_check());
}

/// @id miyualicia_skills_test_policy_bypass_persiste_ignore
/// @role test
/// @layer 7
/// @do verify_persisted_bypass_ignored
//@human / @human Un bypass cores présent dans une politique persistée est ignoré
//@human (fail-closed) ; le bypass des confirmations reste, lui, honoré.
#[test]
fn bypass_cores_persiste_est_ignore() {
    let p: AliciaPolicy = serde_json::from_str(
        r#"{"bypass_cores_validation": true, "bypass_user_confirmations": true}"#,
    )
    .unwrap();
    assert!(p.requires_cores_check());
    assert!(p.bypass_user_confirmations);
}
