// @id: Sodomight-Game-Main @do: game-binary @role: back-end @layer: 4 @human: miyuk
//! Sodomight -- D2-like ARPG, main binary.

fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Sodomight v0.1.0 starting...");

    let config = sodomight_game::default_game_config();

    tracing::info!(
        "Window : {}x{} \"{}\"",
        config.window_width,
        config.window_height,
        config.title
    );
    tracing::info!(
        "Render : {} FPS ({} ms/frame)",
        config.target_fps,
        config.frame_ms()
    );
    tracing::info!(
        "Logic  : {} Hz ({} ms/tick)",
        config.tick_rate_hz,
        config.tick_ms()
    );
    tracing::info!("State  : {:?}", sodomight_game::GameState::default());
    tracing::info!("Sodomight ready. (headless mode -- GPU not initialized)");
}
