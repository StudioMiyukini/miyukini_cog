// @id: Sodomight-Client-Main @do: client-binary @role: back-end @layer: 4 @human: miyuk
//! Sodomight client binary — launches the game window.

fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Sodomight Client v0.1.0");

    let config = mge_platform::PlatformConfig::new(1280, 720, "Sodomight");
    let app = sodomight_client::SodomightApp::new();

    if let Err(e) = mge_platform::run(config, app) {
        tracing::error!("Fatal error: {e}");
        std::process::exit(1);
    }
}
