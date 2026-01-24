//! Boot / shutdown : ordre d'init, hooks d'arrêt des briques techniques.
//! Pas l'orchestration de workflows métier — ni jobs métier, ni hooks applicatifs.

/// Contrat : enregistrement de hooks de shutdown et arrêt dans l'ordre inverse.
pub trait Lifecycle {
    /// Enregistre un hook exécuté à `shutdown`, en ordre LIFO.
    fn register_shutdown_hook<F>(&mut self, f: F)
    where
        F: FnMut() + 'static;

    /// Exécute tous les hooks en ordre inverse d'enregistrement.
    fn shutdown(&mut self);
}

/// Implémentation par défaut : stocke et exécute les hooks. Minimal et remplaçable.
pub struct DefaultLifecycle {
    hooks: Vec<Box<dyn FnMut()>>,
}

impl DefaultLifecycle {
    pub fn new() -> Self {
        Self { hooks: Vec::new() }
    }
}

impl Default for DefaultLifecycle {
    fn default() -> Self {
        Self::new()
    }
}

impl Lifecycle for DefaultLifecycle {
    fn register_shutdown_hook<F>(&mut self, f: F)
    where
        F: FnMut() + 'static,
    {
        self.hooks.push(Box::new(f));
    }

    fn shutdown(&mut self) {
        while let Some(mut hook) = self.hooks.pop() {
            hook();
        }
    }
}
