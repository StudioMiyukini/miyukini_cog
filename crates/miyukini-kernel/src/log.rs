//! Logging structuré. Façade uniquement ; le kernel ne configure aucun backend global.
//! Implémentation par défaut minimale et remplaçable.

/// Niveaux de log. Le kernel ne ré-exporte pas `log::Level`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

fn to_log_facade_level(level: Level) -> log_facade::Level {
    match level {
        Level::Error => log_facade::Level::Error,
        Level::Warn => log_facade::Level::Warn,
        Level::Info => log_facade::Level::Info,
        Level::Debug => log_facade::Level::Debug,
        Level::Trace => log_facade::Level::Trace,
    }
}

/// Contrat : envoi d'un message à un niveau donné.
pub trait Logger {
    fn log(&self, level: Level, message: &str);
}

/// Implémentation par défaut : délègue à la façade `log`.
/// Minimal et remplaçable ; n'impose pas de backend.
#[derive(Debug, Default)]
pub struct DefaultLogger;

impl DefaultLogger {
    pub fn new() -> Self {
        Self
    }
}

impl Logger for DefaultLogger {
    fn log(&self, level: Level, message: &str) {
        log!(to_log_facade_level(level), "{}", message);
    }
}
