//! Smoke test MiyuCloud v2 -- verifie que les nouveaux modules sont declares.
//!
//! Ce test compile mais les fonctions stubs retournent `todo!()`.
//! Il valide la structure du plan avant l'implementation TDD.

#[test]
fn smoke_test_v2_modules_exist() {
    // Verifie que les modules sont declares et accessibles
    // Chaque ligne verifie l'existence d'une fonction publique dans un module v2
    let _totp_enabled = miyucloud::auth::totp::is_totp_enabled;
    let _create_session = miyucloud::auth::sessions::create_session;
    let _onboarding = miyucloud::domain::onboarding::get_status;
    let _health = miyucloud::monitoring::health_check;
    let _html_escape = miyucloud::utils::sanitize::html_escape;
    let _ct_eq = miyucloud::utils::constant_time::constant_time_eq;
    let _b64_encode = miyucloud::utils::base64::encode;
}
