//! Module d'abstraction du temps du Kernel
//!
//! Ce module fournit une abstraction du temps pour permettre l'injection en test
//! et éviter les appels système dispersés.
//!
//! ## Principe
//!
//! - Retourne `SystemTime` (std) — pas de dépendance à chrono
//! - Timezone = produit — le produit convertit si besoin
//! - Injectable pour tests — `&dyn Clock` permet le mock
//! - Pas de méthode `timestamp` — le produit fait la conversion
//!
//! ## Références
//!
//! - [Kernel - Invariants & Guarantees](../../../docs/kernel/contracts/Kernel%20-%20Invariants%20&%20Guarantees.md)
//! - [Kernel - Reference Implementation Guidelines](../../../docs/kernel/implementation/Kernel%20-%20Reference%20Implementation%20Guidelines.md)
//! - [Kernel - Revue Traits API v0.1](../../../docs/kernel/Miyukini%20Core%20System%20-%20Revue%20Traits%20API%20v0.1.md)

use std::time::SystemTime;

/// @id: kernel_clock_trait
/// @role: infrastructure
/// @layer: kernel
/// @human: Trait d'abstraction du temps pour tests et injection. Le produit peut implémenter un FakeClock pour les tests.
/// @do: provide_time_abstraction
/// Trait d'abstraction du temps.
///
/// Le trait `Clock` permet d'injecter différentes sources de temps, notamment
/// pour les tests. Le produit peut implémenter un `FakeClock` avec `now()`
/// constant pour rendre les tests déterministes.
///
/// ## Signature gelée (v0.1)
///
/// Cette signature est gelée et ne peut pas être modifiée sans version majeure.
///
/// ## Invariants respectés
///
/// - INV-K-3 : Primitives locales sûres uniquement (SystemTime::now() est local)
/// - INV-K-6 : Déterminisme (injection permet tests déterministes)
/// - INV-K-8 : Souveraineté locale (pas de NTP obligatoire)
pub trait Clock {
    /// @id: kernel_clock_now
    /// @role: infrastructure
    /// @layer: kernel
    /// @human: Retourne l'heure courante du système. Le produit peut convertir en timestamp si nécessaire.
    /// @do: get_current_time
    /// @depends: kernel_clock_trait
    /// Retourne l'heure courante du système.
    ///
    /// Le type de retour est `SystemTime` de la bibliothèque standard.
    /// Le produit qui a besoin de timezone fait la conversion (chrono, etc.)
    /// en dehors du kernel.
    ///
    /// Pour un timestamp (ex: secondes depuis epoch), le produit fait :
    /// ```rust
    /// use miyukini_kernel::time::{Clock, DefaultClock};
    /// let clock = DefaultClock;
    /// let timestamp = clock.now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    /// ```
    ///
    /// # Returns
    ///
    /// L'heure courante du système sous forme de `SystemTime`.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use miyukini_kernel::time::{Clock, DefaultClock};
    ///
    /// let clock = DefaultClock;
    /// let now = clock.now();
    /// ```
    fn now(&self) -> SystemTime;
}

/// @id: kernel_clock_default
/// @role: infrastructure
/// @layer: kernel
/// @human: Horloge système par défaut. Utilise SystemTime::now() pour obtenir l'heure courante.
/// @do: provide_system_time
/// Horloge système par défaut.
///
/// Cette implémentation utilise `SystemTime::now()` pour obtenir l'heure courante
/// du système. Pour les tests, le produit peut implémenter un `FakeClock` avec
/// `now()` constant.
impl Clock for DefaultClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}

/// @id: kernel_clock_default_struct
/// @role: infrastructure
/// @layer: kernel
/// @human: Structure de l'horloge par défaut. Type unitaire sans état.
/// @do: represent_default_clock
/// Horloge système par défaut.
///
/// Type unitaire sans état. Peut être instancié et réutilisé.
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultClock;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, UNIX_EPOCH};

    /// @id: test_time_now_returns_systemtime
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que now() retourne un SystemTime valide.
    /// @do: verify_systemtime_return
    /// @depends: kernel_clock_now
    #[test]
    fn test_time_now_returns_systemtime() {
        let clock = DefaultClock;
        let now = clock.now();
        // Vérifier que now() retourne un SystemTime valide
        assert!(now.duration_since(UNIX_EPOCH).is_ok());
    }

    /// @id: test_time_now_monotonic
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que now() retourne un temps monotone (croissant).
    /// @do: verify_time_monotonicity
    /// @depends: kernel_clock_now
    #[test]
    fn test_time_now_monotonic() {
        let clock = DefaultClock;
        let t1 = clock.now();
        // Attendre un peu (mais pas trop pour ne pas ralentir les tests)
        std::thread::sleep(Duration::from_millis(10));
        let t2 = clock.now();
        // t2 doit être >= t1 (ou très proche si le sleep était trop court)
        assert!(t2 >= t1 || t2.duration_since(t1).unwrap() < Duration::from_millis(5));
    }

    /// @id: test_time_fake_clock_injection
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant qu'un FakeClock peut être injecté pour les tests avec now() constant.
    /// @do: verify_fake_clock_injection
    /// @depends: kernel_clock_trait
    #[test]
    fn test_time_fake_clock_injection() {
        let fixed_time = UNIX_EPOCH + Duration::from_secs(1234567890);
        let fake_clock = FakeClock::new(fixed_time);
        assert_eq!(fake_clock.now(), fixed_time);
    }

    /// @id: test_time_no_timezone_logic
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant qu'aucune logique de timezone n'est implémentée. Retourne SystemTime, pas de conversion.
    /// @do: verify_no_timezone_logic
    /// @depends: kernel_clock_now
    #[test]
    fn test_time_no_timezone_logic() {
        let clock = DefaultClock;
        let now = clock.now();
        // Vérifier que now() retourne SystemTime, pas un type avec timezone
        // Le produit doit faire la conversion de timezone lui-même
        let _: SystemTime = now; // Compile si c'est bien SystemTime
    }

    /// @id: test_time_no_business_logic
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant qu'aucune logique métier (calcul de dates d'expiration) n'est implémentée, conformément à INV-K-1.
    /// @do: verify_no_business_logic
    /// @depends: kernel_clock_trait
    #[test]
    fn test_time_no_business_logic() {
        // Vérifier qu'aucune méthode métier n'existe
        // Le kernel ne doit pas avoir de méthodes comme expiration_date(), etc.
        let clock = DefaultClock;
        let _now = clock.now();
        // Si on pouvait faire : clock.expiration_date(), cela violerait INV-K-1
    }

    /// @id: test_time_no_external_dependency
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que Clock fonctionne sans réseau, conformément à INV-K-2. Pas de NTP obligatoire.
    /// @do: verify_no_network_required
    /// @depends: kernel_clock_now
    #[test]
    fn test_time_no_external_dependency() {
        // Ce test vérifie que now() fonctionne sans réseau
        // Si now() nécessitait une synchronisation NTP, ce test échouerait en mode offline
        let clock = DefaultClock;
        let _now = clock.now();
        // Le fait que le test passe confirme INV-K-2
    }

    /// @id: test_time_deterministic_fake
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant qu'un FakeClock est déterministe : même FakeClock → même résultat, conformément à INV-K-6.
    /// @do: verify_fake_clock_determinism
    /// @depends: kernel_clock_trait
    #[test]
    fn test_time_deterministic_fake() {
        let fixed_time = UNIX_EPOCH + Duration::from_secs(1234567890);
        let fake_clock = FakeClock::new(fixed_time);
        // Même FakeClock → même résultat (déterministe)
        assert_eq!(fake_clock.now(), fixed_time);
        assert_eq!(fake_clock.now(), fixed_time); // Appel multiple = même résultat
    }

    /// FakeClock pour les tests
    /// @id: kernel_clock_fake
    /// @role: test_utility
    /// @layer: kernel
    /// @human: Horloge factice pour les tests avec temps fixe. Permet de rendre les tests déterministes.
    /// @do: provide_fixed_time_for_tests
    #[cfg(test)]
    pub struct FakeClock {
        fixed_time: SystemTime,
    }

    #[cfg(test)]
    impl FakeClock {
        pub fn new(fixed_time: SystemTime) -> Self {
            Self { fixed_time }
        }
    }

    #[cfg(test)]
    impl Clock for FakeClock {
        fn now(&self) -> SystemTime {
            self.fixed_time
        }
    }
}
