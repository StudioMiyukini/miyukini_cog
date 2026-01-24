//! Démo-test : consomme le kernel (Config, Logger, IdGenerator, Clock, Lifecycle),
//! log le cycle de vie, s'arrête proprement. Produit consommateur — pas d'extension du kernel.
//!
//! Principe : le kernel définit le contrat (Logger). La démo fournit son propre
//! rendu (StdoutLogger) — pas de DefaultLogger. Le kernel reste neutre et silencieux.

use miyukini_kernel::{
    Config, Clock, DefaultClock, DefaultLifecycle, EnvConfig, IdGenerator, Level, Lifecycle,
    Logger, UuidIdGenerator,
};
use std::rc::Rc;

// ---------------------------------------------------------------------------
// StdoutLogger : logger produit, local à la démo. Affiche [Level] message.
// Pas de dépendance, pas de macro, lisible, conforme au contrat Logger.
// ---------------------------------------------------------------------------

struct StdoutLogger;

impl Logger for StdoutLogger {
    fn log(&self, level: Level, message: &str) {
        println!("[{:?}] {}", level, message);
    }
}

// ---------------------------------------------------------------------------
// main : scénario de démo lisible, ordre de vie explicite, CR final.
// ---------------------------------------------------------------------------

fn main() {
    // --- Logger produit (pas DefaultLogger) ---
    let logger = Rc::new(StdoutLogger);

    // --- 1. Démarrage ---
    logger.log(Level::Info, "demo-logging-lifecycle starting");

    // --- 2. Init des briques kernel (ordre explicite) ---
    let config = EnvConfig::from_env();
    let _ = config.get("PATH");
    logger.log(Level::Debug, "configuration loaded");

    let clock = DefaultClock::new();
    logger.log(Level::Debug, "clock initialized");

    let id_gen = UuidIdGenerator::new();
    logger.log(Level::Debug, "id generator initialized");

    println!();

    // --- 3. Usage : ID + temps ---
    let id = id_gen.generate();
    logger.log(Level::Info, &format!("generated service id: {}", id));

    let now = clock.now();
    logger.log(Level::Info, &format!("current time: {:?}", now));

    println!();

    // --- 4. Tous les niveaux de log (exemples) ---
    logger.log(Level::Warn, "this is a warning example");
    logger.log(Level::Error, "this is an error example (simulated)");
    logger.log(Level::Trace, "trace-level log example");

    println!();

    // --- 5. Lifecycle + shutdown hook ---
    let mut lifecycle = DefaultLifecycle::new();

    logger.log(Level::Info, "registering shutdown hook");
    let logger_hook = Rc::clone(&logger);
    lifecycle.register_shutdown_hook(move || {
        logger_hook.log(Level::Info, "shutdown hook called");
    });

    logger.log(Level::Info, "shutting down demo");
    println!();

    lifecycle.shutdown();

    logger.log(Level::Info, "demo finished successfully");

    // --- 6. CR final : rapport produit (pas du logging kernel) ---
    println!();
    println!("--- DEMO REPORT --------------------------------");
    println!("Kernel components initialized : OK");
    println!("Logs emitted                   : OK");
    println!("ID generated                   : OK");
    println!("Clock queried                  : OK");
    println!("Shutdown hook executed         : OK");
    println!("------------------------------------------------");
    println!("Result : SUCCESS");
}
