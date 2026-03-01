// @id: Sodomight-Server-Main @do: server-binary @role: back-end @layer: 4 @human: miyuk
//! Sodomight dedicated server binary.

fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Sodomight Server v0.1.0 starting...");

    let config = sodomight_server::default_server_config();

    tracing::info!("Bind address  : {}", config.bind_addr);
    tracing::info!("Max clients   : {}", config.max_clients);
    tracing::info!("Tick rate     : {} Hz ({} ms/tick)", config.tick_rate_hz, config.tick_ms());
    tracing::info!("Save interval : {} ticks", config.save_interval_ticks);
    tracing::info!("Data dir      : {}", config.data_dir);
    tracing::info!("Sodomight Server ready (headless mode).");
}
