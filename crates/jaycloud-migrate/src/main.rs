//! Binaire `jaycloud-migrate` — outil one-shot de migration MiyuCloud →
//! JayCloud.
//!
//! En P2 (skeleton), ce binaire se contente de logger son intention. Le
//! pipeline de migration (§10 de la Spec) sera implémenté en PR-6 (P5).

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("jaycloud-migrate P2 skeleton — outil de migration MiyuCloud → JayCloud");
    tracing::warn!(
        "Pipeline de migration non implémenté. PR-6 (P5) livrera l'inventaire, le \
         rapatriement vers le CAS et la génération des miyucloud_redirects."
    );
    Ok(())
}
