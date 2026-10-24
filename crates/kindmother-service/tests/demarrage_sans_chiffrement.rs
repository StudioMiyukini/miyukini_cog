//! Démarrage fail-closed sans chiffrement (K-06).
//!
//! @id     kindmother_service_tests_demarrage_sans_chiffrement
//! @do     verifier_refus_demarrage_en_clair
//! @role   test
//! @layer  core
//! @human  kindmother-service — sans `db-encryption`, refus de démarrer sauf debug + KINDMOTHER_DEV_PLAINTEXT=1.

use kindmother_service::{KindMotherServer, ENV_DEV_PLAINTEXT};

#[tokio::test]
async fn demarrage_selon_mode_de_stockage() {
    let dir = std::env::temp_dir().join(format!("km-start-{}", uuid::Uuid::new_v4()));
    let result = KindMotherServer::new(&dir).await;
    let dev_plaintext = std::env::var(ENV_DEV_PLAINTEXT).is_ok_and(|v| v == "1");
    if cfg!(feature = "db-encryption") {
        assert!(result.is_ok(), "build chiffré : démarrage attendu");
    } else if cfg!(debug_assertions) && dev_plaintext {
        assert!(result.is_ok(), "mode en clair de développement explicite");
    } else {
        assert!(result.is_err(), "sans chiffrement : refus de démarrer (fail-closed)");
    }
    let _ = std::fs::remove_dir_all(dir);
}
