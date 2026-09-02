//! @id     miyukini_whisper_main
//! @do     main
//! @role   orchestration
//! @layer  app
//! @human  miyukini-whisper — module « main ».

//! Entree binaire Miyukini Whisper.

use miyukini_whisper_app::{run, WhisperConfig};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("miyukini_whisper_app=info")
        .init();

    let config = WhisperConfig::default();
    run(&config);
}
