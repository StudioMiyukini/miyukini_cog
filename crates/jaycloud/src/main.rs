//! Binaire `jaycloud` — bootstrap du service de sauvegarde cloud.
//!
//! En P2 (skeleton), ce binaire se contente de logger son démarrage et
//! de sortir. Le bootstrap réel (axum HTTPS + DAV + scheduler) sera
//! implémenté en PR-3 / PR-4 selon le plan de la Spec.

use jaycloud::JayCloudConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber_init();

    let config = JayCloudConfig::default();
    tracing::info!(
        ?config,
        "jaycloud P2 skeleton — service de sauvegarde cloud (charpente uniquement)"
    );
    tracing::warn!(
        "Pas d'écoute HTTP encore. PR-3 (P3.c) introduira axum + WebDAV (dav-server)."
    );

    Ok(())
}

fn tracing_subscriber_init() {
    // Skeleton : initialisation minimale via tracing (pas de tracing-subscriber
    // dans les deps du skeleton — on évite de tirer la dépendance avant
    // qu'elle soit utile). Cette fonction sera étoffée en PR-3.
    let _ = std::env::var("RUST_LOG");
}
