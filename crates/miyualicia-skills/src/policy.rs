//! Politique d'exécution d'Alicia — gouverne les **gates** traversées avant
//! qu'une capability soit exécutée hors-limites d'Alicia.
//!
//! Deux gates indépendantes :
//!
//! 1. **Validation cores** (sécurité technique) : `BorderGuard`, `StrongFather`
//!    et le core métier compétent arbitrent toute écriture qu'Alicia tente
//!    hors de son propre périmètre. C'est la gouvernance COG, posée par
//!    construction.
//! 2. **Confirmation utilisateur** (consentement UX) : `confirmation_default`
//!    de chaque capability + préférence utilisateur. C'est l'« voulez-vous
//!    vraiment ? » avant l'action.
//!
//! Les deux peuvent être bypassées **indépendamment** via [`AliciaPolicy`] :
//! - bypass cores : pour dev / test / environnement isolé sans gouvernance
//! - bypass confirmations : pour environnement de confiance / batch / agent
//!   pleinement autonome
//!
//! **Bypass cores ≠ bypass confirmations.** Un opérateur peut vouloir
//! conserver l'audit cores mais sauter les confirmations (ex. CI), ou
//! l'inverse (ex. dev local sans cores mais garder les double-checks).
//!
//! ## Bypass cores : tests et dev uniquement (K-16, C-12)
//!
//! Le champ `bypass_cores_validation` et [`AliciaPolicy::permissive`] n'existent
//! que sous `cfg(test)` ou avec la feature `dev-bypass` (désactivée par défaut).
//! Dans un binaire de production, la gate cores est donc toujours active :
//! [`AliciaPolicy::requires_cores_check`] renvoie `true`, et un
//! `"bypass_cores_validation": true` présent dans une politique persistée est
//! ignoré à la désérialisation (fail-closed).

// @id service.alicia.skills.policy
// @role alicia_execution_policy
// @layer 7
// @do declare_alicia_policy
// @human Politique d'exécution d'Alicia — flags de bypass des gates cores et confirmation.

use serde::{Deserialize, Serialize};

use crate::capability::ConfirmationPolicy;

/// Politique d'exécution d'Alicia.
///
/// Sérialisable (`serde`) pour persistance via TOML, JSON, ou le store
/// `miyualicia-memory` (en `Preference` scope `"global"`).
///
/// Par défaut : **strict** (aucun bypass, gates pleinement actives).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AliciaPolicy {
    /// Si `true`, Alicia exécute directement les capabilities **sans router
    /// par les cores**. La gouvernance COG (BorderGuard, StrongFather, etc.)
    /// est court-circuitée.
    ///
    /// **DANGEREUX.** Réservé aux environnements de dev / test / sandbox
    /// isolés. **Jamais en production utilisateur.**
    ///
    /// L'audit (épisodes en mémoire) reste actif côté Alicia, mais les
    /// politiques d'arbitrage cross-services ne sont plus appliquées.
    ///
    /// N'existe que sous `cfg(test)` ou la feature `dev-bypass` (K-16).
    #[cfg(any(test, feature = "dev-bypass"))]
    #[serde(default)]
    pub bypass_cores_validation: bool,

    /// Si `true`, Alicia n'affiche **plus** de confirmation utilisateur avant
    /// d'exécuter une capability, quel que soit son `confirmation_default`.
    ///
    /// À utiliser pour :
    /// - environnements de confiance (kiosque domestique privé)
    /// - exécution batch / agent autonome
    /// - tests automatisés
    ///
    /// L'audit (épisodes en mémoire) reste actif. Les gates cores restent
    /// actives (sauf bypass cores, possible uniquement en test / `dev-bypass`).
    #[serde(default)]
    pub bypass_user_confirmations: bool,
}

impl AliciaPolicy {
    /// Politique stricte : les deux gates sont actives. Recommandé en prod.
    #[must_use]
    pub fn strict() -> Self {
        Self::default()
    }

    /// Politique permissive : les deux gates sont bypassées.
    ///
    /// **À n'utiliser qu'en dev / test / sandbox.** N'existe que sous
    /// `cfg(test)` ou la feature `dev-bypass` (K-16).
    #[cfg(any(test, feature = "dev-bypass"))]
    #[must_use]
    pub fn permissive() -> Self {
        Self {
            bypass_cores_validation: true,
            bypass_user_confirmations: true,
        }
    }

    /// `true` si l'exécution doit traverser les cores avant action.
    ///
    /// Hors `cfg(test)` / feature `dev-bypass` : toujours `true` (fail-closed).
    #[must_use]
    pub fn requires_cores_check(&self) -> bool {
        #[cfg(any(test, feature = "dev-bypass"))]
        {
            !self.bypass_cores_validation
        }
        #[cfg(not(any(test, feature = "dev-bypass")))]
        {
            true
        }
    }

    /// `true` si la capability donnée doit demander confirmation à
    /// l'utilisateur, en fonction de son `confirmation_default`, du flag
    /// `writes`, et de la politique courante.
    ///
    /// Sémantique de `ConfirmationPolicy` :
    /// - `Never` → jamais
    /// - `Always` → toujours
    /// - `AlwaysForWrites` → si `writes == true`
    /// - `UserPreference` → laissé au caller (préférence stockée), default
    ///   conservateur = `true`
    #[must_use]
    pub fn requires_user_confirmation(
        &self,
        default: ConfirmationPolicy,
        writes: bool,
    ) -> bool {
        if self.bypass_user_confirmations {
            return false;
        }
        match default {
            ConfirmationPolicy::Never => false,
            ConfirmationPolicy::Always => true,
            ConfirmationPolicy::AlwaysForWrites => writes,
            // Default conservateur : si rien n'est posé côté préférence, on
            // demande confirmation. Le caller peut overrider.
            ConfirmationPolicy::UserPreference => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_strict() {
        let p = AliciaPolicy::default();
        assert!(p.requires_cores_check());
        assert!(!p.bypass_user_confirmations);
        assert_eq!(p, AliciaPolicy::strict());
    }

    #[test]
    fn permissive_disables_both_gates() {
        let p = AliciaPolicy::permissive();
        assert!(!p.requires_cores_check());
        assert!(p.bypass_user_confirmations);
        // Même une capability "Always" ne demande pas confirmation
        assert!(!p.requires_user_confirmation(ConfirmationPolicy::Always, true));
    }

    #[test]
    fn cores_bypass_independent_of_confirmation_bypass() {
        let p = AliciaPolicy {
            bypass_cores_validation: true,
            bypass_user_confirmations: false,
        };
        assert!(!p.requires_cores_check());
        // Confirmations restent actives
        assert!(p.requires_user_confirmation(ConfirmationPolicy::Always, false));
        assert!(p.requires_user_confirmation(ConfirmationPolicy::AlwaysForWrites, true));
        assert!(!p.requires_user_confirmation(ConfirmationPolicy::AlwaysForWrites, false));
    }

    #[test]
    fn confirmation_bypass_independent_of_cores_bypass() {
        let p = AliciaPolicy {
            bypass_cores_validation: false,
            bypass_user_confirmations: true,
        };
        // Cores actifs
        assert!(p.requires_cores_check());
        // Confirmations bypassées
        assert!(!p.requires_user_confirmation(ConfirmationPolicy::Always, true));
        assert!(!p.requires_user_confirmation(ConfirmationPolicy::AlwaysForWrites, true));
    }

    #[test]
    fn confirmation_logic_strict_default() {
        let p = AliciaPolicy::strict();
        assert!(!p.requires_user_confirmation(ConfirmationPolicy::Never, true));
        assert!(p.requires_user_confirmation(ConfirmationPolicy::Always, false));
        assert!(p.requires_user_confirmation(ConfirmationPolicy::AlwaysForWrites, true));
        assert!(!p.requires_user_confirmation(ConfirmationPolicy::AlwaysForWrites, false));
        // UserPreference : défaut conservateur = true
        assert!(p.requires_user_confirmation(ConfirmationPolicy::UserPreference, false));
    }

    #[test]
    fn json_roundtrip() {
        let p = AliciaPolicy {
            bypass_cores_validation: true,
            bypass_user_confirmations: false,
        };
        let s = serde_json::to_string(&p).unwrap();
        let back: AliciaPolicy = serde_json::from_str(&s).unwrap();
        assert_eq!(p, back);
    }

    #[test]
    fn json_default_omits_fields_and_round_trips() {
        let p = AliciaPolicy::strict();
        let s = serde_json::to_string(&p).unwrap();
        // Le JSON contient les deux flags à false explicitement.
        assert!(s.contains("bypass_cores_validation"));
        // Le serde(default) accepte les JSON vides en input
        let back: AliciaPolicy = serde_json::from_str("{}").unwrap();
        assert_eq!(back, AliciaPolicy::strict());
    }
}
