//! Module de gestion du cycle de vie du Kernel
//!
//! Ce module fournit une abstraction pour gérer le cycle de vie (boot / shutdown).
//! L'initialisation reste au produit ; le kernel gère uniquement les hooks d'arrêt.
//!
//! ## Principe
//!
//! - Shutdown uniquement — l'init reste au produit (pas d'orchestration)
//! - `FnMut() + 'static` — contrainte Rust pour `Box<dyn FnMut>`
//! - Exécution LIFO — dernier enregistré = premier fermé
//! - Pas de `Result` — panic dans un hook se propage
//!
//! ## Références
//!
//! - [Kernel - Invariants & Guarantees](../../../docs/kernel/contracts/Kernel%20-%20Invariants%20&%20Guarantees.md)
//! - [Kernel - Reference Implementation Guidelines](../../../docs/kernel/implementation/Kernel%20-%20Reference%20Implementation%20Guidelines.md)
//! - [Kernel - Revue Traits API v0.1](../../../docs/kernel/Miyukini%20Core%20System%20-%20Revue%20Traits%20API%20v0.1.md)

/// @id: kernel_lifecycle_trait
/// @role: infrastructure
/// @layer: kernel
/// @human: Trait de gestion du cycle de vie (shutdown uniquement). L'initialisation reste au produit. Pas d'orchestration de workflows métier.
/// @do: define_lifecycle_contract
/// Trait de gestion du cycle de vie : shutdown hooks uniquement.
///
/// Le contrat parle d'« ordre d'init » et d'« arrêt ». L'**init** reste au produit
/// (il enchaîne config, log, etc.). Aucun `register_init_hook` dans le kernel —
/// évite de glisser vers de l'orchestration.
///
/// ## Signature gelée (v0.1)
///
/// Cette signature est gelée et ne peut pas être modifiée sans version majeure.
///
/// ## Invariants respectés
///
/// - INV-K-1 : Aucune logique métier (pas d'orchestration de workflows métier)
/// - INV-K-2 : Aucune dépendance externe critique (hooks locaux uniquement)
/// - INV-K-10 : Gouvernance respectée (pas de décision autonome)
pub trait Lifecycle {
    /// @id: kernel_lifecycle_register_shutdown_hook
    /// @role: infrastructure
    /// @layer: kernel
    /// @human: Enregistre un hook d'arrêt. Les hooks sont exécutés en ordre LIFO lors de shutdown().
    /// @do: register_shutdown_hook
    /// @depends: kernel_lifecycle_trait
    /// Enregistre un hook d'arrêt.
    ///
    /// Les hooks sont exécutés en ordre LIFO (dernier enregistré = premier exécuté)
    /// lors de l'appel à `shutdown()`.
    ///
    /// ## Contraintes
    ///
    /// - `F: FnMut() + 'static` : Contrainte Rust pour `Box<dyn FnMut>`
    /// - `FnMut` au lieu de `FnOnce` : Contrainte d'exécution en Rust
    /// - `'static` : Pas de capture de références éphémères
    ///
    /// # Arguments
    ///
    /// * `f` - La fonction de hook à enregistrer
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use miyukini_kernel::lifecycle::{Lifecycle, DefaultLifecycle};
    ///
    /// let mut lifecycle = DefaultLifecycle::new();
    /// lifecycle.register_shutdown_hook(|| {
    ///     println!("Shutting down...");
    /// });
    /// ```
    fn register_shutdown_hook<F>(&mut self, f: F)
    where
        F: FnMut() + 'static;

    /// @id: kernel_lifecycle_shutdown
    /// @role: infrastructure
    /// @layer: kernel
    /// @human: Exécute tous les hooks d'arrêt enregistrés en ordre LIFO. Un second appel est implémentation-dépendant.
    /// @do: execute_shutdown_hooks
    /// @depends: kernel_lifecycle_trait
    /// Exécute tous les hooks d'arrêt enregistrés.
    ///
    /// Les hooks sont exécutés en ordre LIFO (dernier enregistré = premier exécuté).
    /// Une panic dans un hook se propage.
    ///
    /// ## Comportement d'implémentation
    ///
    /// Un second appel à `shutdown()` est implémentation-dépendant :
    /// - Pour `DefaultLifecycle` : no-op (les hooks sont déjà exécutés)
    /// - D'autres implémentations peuvent avoir un comportement différent
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use miyukini_kernel::lifecycle::{Lifecycle, DefaultLifecycle};
    ///
    /// let mut lifecycle = DefaultLifecycle::new();
    /// lifecycle.register_shutdown_hook(|| println!("Hook 1"));
    /// lifecycle.register_shutdown_hook(|| println!("Hook 2"));
    /// lifecycle.shutdown(); // Exécute Hook 2, puis Hook 1
    /// ```
    fn shutdown(&mut self);
}

/// @id: kernel_lifecycle_default
/// @role: infrastructure
/// @layer: kernel
/// @human: Implémentation par défaut du trait Lifecycle. Stocke les hooks dans un Vec et les exécute en LIFO.
/// @do: manage_shutdown_hooks
/// Gestion du cycle de vie : shutdown hooks uniquement.
///
/// Cette implémentation stocke les hooks dans un `Vec` et les exécute en ordre
/// LIFO lors de `shutdown()`. Un second appel à `shutdown()` est un no-op
/// (les hooks sont déjà exécutés).
impl Lifecycle for DefaultLifecycle {
    fn register_shutdown_hook<F>(&mut self, f: F)
    where
        F: FnMut() + 'static,
    {
        self.hooks.push(Box::new(f));
    }

    fn shutdown(&mut self) {
        // Exécution LIFO (dernier enregistré = premier exécuté)
        while let Some(mut hook) = self.hooks.pop() {
            hook();
        }
    }
}

/// @id: kernel_lifecycle_default_struct
/// @role: infrastructure
/// @layer: kernel
/// @human: Structure du lifecycle par défaut. Stocke les hooks dans un Vec pour exécution LIFO.
/// @do: store_shutdown_hooks
/// Gestion du cycle de vie par défaut.
///
/// Stocke les hooks d'arrêt dans un `Vec` pour exécution en ordre LIFO.
#[derive(Default)]
pub struct DefaultLifecycle {
    /// @id: kernel_lifecycle_hooks
    /// @role: storage
    /// @layer: kernel
    /// @human: Stockage interne des hooks d'arrêt. Exécutés en ordre LIFO lors de shutdown().
    /// @do: store_hooks
    /// @depends: kernel_lifecycle_default_struct
    hooks: Vec<Box<dyn FnMut()>>,
}

impl DefaultLifecycle {
    /// @id: kernel_lifecycle_new
    /// @role: constructor
    /// @layer: kernel
    /// @human: Crée une nouvelle instance de DefaultLifecycle avec un Vec vide de hooks.
    /// @do: create_lifecycle_instance
    /// @depends: kernel_lifecycle_default_struct
    /// Crée une nouvelle instance de `DefaultLifecycle`.
    ///
    /// # Returns
    ///
    /// Une instance vide de `DefaultLifecycle` prête à recevoir des hooks.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use miyukini_kernel::lifecycle::DefaultLifecycle;
    ///
    /// let mut lifecycle = DefaultLifecycle::new();
    /// ```
    #[must_use] 
    pub fn new() -> Self {
        Self {
            hooks: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: test_lifecycle_register_hook
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant qu'un hook peut être enregistré sans erreur.
    /// @do: verify_hook_registration
    /// @depends: kernel_lifecycle_register_shutdown_hook
    #[test]
    fn test_lifecycle_register_hook() {
        let mut lifecycle = DefaultLifecycle::new();
        lifecycle.register_shutdown_hook(|| {});
        // Pas d'erreur
    }

    /// @id: test_lifecycle_shutdown_executes_hooks
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que shutdown() exécute les hooks enregistrés.
    /// @do: verify_hooks_execution
    /// @depends: kernel_lifecycle_shutdown
    #[test]
    fn test_lifecycle_shutdown_executes_hooks() {
        use std::rc::Rc;
        use std::cell::RefCell;

        let mut lifecycle = DefaultLifecycle::new();
        let called = Rc::new(RefCell::new(false));

        let called_clone = called.clone();
        lifecycle.register_shutdown_hook(move || {
            *called_clone.borrow_mut() = true;
        });
        assert!(!*called.borrow());

        lifecycle.shutdown();
        assert!(*called.borrow());
    }

    /// @id: test_lifecycle_shutdown_lifo_order
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que shutdown() exécute les hooks en ordre LIFO (dernier enregistré = premier exécuté).
    /// @do: verify_lifo_order
    /// @depends: kernel_lifecycle_shutdown
    #[test]
    fn test_lifecycle_shutdown_lifo_order() {
        use std::rc::Rc;
        use std::cell::RefCell;

        let mut lifecycle = DefaultLifecycle::new();
        let order = Rc::new(RefCell::new(Vec::new()));

        let order_clone = order.clone();
        lifecycle.register_shutdown_hook(move || order_clone.borrow_mut().push(1));
        
        let order_clone = order.clone();
        lifecycle.register_shutdown_hook(move || order_clone.borrow_mut().push(2));
        
        let order_clone = order.clone();
        lifecycle.register_shutdown_hook(move || order_clone.borrow_mut().push(3));

        lifecycle.shutdown();

        assert_eq!(*order.borrow(), vec![3, 2, 1]); // LIFO
    }

    /// @id: test_lifecycle_multiple_hooks
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que tous les hooks sont exécutés, même s'il y en a plusieurs.
    /// @do: verify_all_hooks_executed
    /// @depends: kernel_lifecycle_shutdown
    #[test]
    fn test_lifecycle_multiple_hooks() {
        use std::rc::Rc;
        use std::cell::RefCell;

        let mut lifecycle = DefaultLifecycle::new();
        let count = Rc::new(RefCell::new(0));

        let count_clone = count.clone();
        lifecycle.register_shutdown_hook(move || *count_clone.borrow_mut() += 1);
        
        let count_clone = count.clone();
        lifecycle.register_shutdown_hook(move || *count_clone.borrow_mut() += 1);
        
        let count_clone = count.clone();
        lifecycle.register_shutdown_hook(move || *count_clone.borrow_mut() += 1);

        lifecycle.shutdown();

        assert_eq!(*count.borrow(), 3);
    }

    /// @id: test_lifecycle_shutdown_idempotent
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant qu'un second appel à shutdown() est idempotent (no-op pour DefaultLifecycle).
    /// @do: verify_shutdown_idempotence
    /// @depends: kernel_lifecycle_shutdown
    #[test]
    fn test_lifecycle_shutdown_idempotent() {
        use std::rc::Rc;
        use std::cell::RefCell;

        let mut lifecycle = DefaultLifecycle::new();
        let count = Rc::new(RefCell::new(0));

        let count_clone = count.clone();
        lifecycle.register_shutdown_hook(move || *count_clone.borrow_mut() += 1);

        lifecycle.shutdown();
        assert_eq!(*count.borrow(), 1);

        // Second appel = no-op (les hooks sont déjà exécutés)
        lifecycle.shutdown();
        assert_eq!(*count.borrow(), 1); // Pas d'incrémentation supplémentaire
    }

    /// @id: test_lifecycle_no_init_hooks
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant qu'aucun hook d'init n'existe. Seulement shutdown (pas d'orchestration).
    /// @do: verify_no_init_hooks
    /// @depends: kernel_lifecycle_trait
    #[test]
    fn test_lifecycle_no_init_hooks() {
        // Vérifier qu'aucun hook d'init n'existe
        // Le kernel ne doit pas avoir de register_init_hook()
        let mut lifecycle = DefaultLifecycle::new();
        lifecycle.register_shutdown_hook(|| {});
        // Si on pouvait faire : lifecycle.register_init_hook(), cela violerait le contrat
    }

    /// @id: test_lifecycle_no_business_logic
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant qu'aucun hook métier prédéfini n'existe, conformément à INV-K-1.
    /// @do: verify_no_business_hooks
    /// @depends: kernel_lifecycle_trait
    #[test]
    fn test_lifecycle_no_business_logic() {
        // Vérifier qu'aucun hook métier n'est prédéfini
        // Le kernel ne doit pas avoir de hooks comme shutdown_database(), etc.
        let mut lifecycle = DefaultLifecycle::new();
        lifecycle.register_shutdown_hook(|| {
            // Le produit définit ses propres hooks
        });
        // Si on pouvait faire : lifecycle.register_database_shutdown(), cela violerait INV-K-1
    }

    /// @id: test_lifecycle_deterministic
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que l'ordre d'exécution est déterministe : même ordre d'enregistrement → même ordre d'exécution, conformément à INV-K-6.
    /// @do: verify_order_determinism
    /// @depends: kernel_lifecycle_shutdown
    #[test]
    fn test_lifecycle_deterministic() {
        use std::rc::Rc;
        use std::cell::RefCell;

        let mut lifecycle1 = DefaultLifecycle::new();
        let order1 = Rc::new(RefCell::new(Vec::new()));
        
        let order1_clone = order1.clone();
        lifecycle1.register_shutdown_hook(move || order1_clone.borrow_mut().push(1));
        
        let order1_clone = order1.clone();
        lifecycle1.register_shutdown_hook(move || order1_clone.borrow_mut().push(2));
        lifecycle1.shutdown();

        let mut lifecycle2 = DefaultLifecycle::new();
        let order2 = Rc::new(RefCell::new(Vec::new()));
        
        let order2_clone = order2.clone();
        lifecycle2.register_shutdown_hook(move || order2_clone.borrow_mut().push(1));
        
        let order2_clone = order2.clone();
        lifecycle2.register_shutdown_hook(move || order2_clone.borrow_mut().push(2));
        lifecycle2.shutdown();

        // Même ordre d'enregistrement → même ordre d'exécution (déterministe)
        assert_eq!(*order1.borrow(), *order2.borrow());
        assert_eq!(*order1.borrow(), vec![2, 1]); // LIFO
    }
}
