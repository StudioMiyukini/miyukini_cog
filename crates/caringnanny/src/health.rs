//! Module d'état de santé de CaringNanny

/// @id caringnanny_health_status
/// @role data
/// @layer core
/// @do represent_health_status
//@human / @human État de santé d'un composant ou du système.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    /// @id caringnanny_health_status_healthy
    /// @role data
    /// @layer core
    /// @do represent_healthy_status
    //@human / @human Composant en bonne santé.
    /// @depends caringnanny_health_status
    Healthy,
    /// @id caringnanny_health_status_degraded
    /// @role data
    /// @layer core
    /// @do represent_degraded_status
    //@human / @human Composant dégradé mais fonctionnel.
    /// @depends caringnanny_health_status
    Degraded,
    /// @id caringnanny_health_status_unhealthy
    /// @role data
    /// @layer core
    /// @do represent_unhealthy_status
    //@human / @human Composant non fonctionnel.
    /// @depends caringnanny_health_status
    Unhealthy,
    /// @id caringnanny_health_status_inconnu
    /// @role data
    /// @layer core
    /// @do represent_unknown_status
    //@human / @human Aucune observation pour ce composant : état inconnu. Ne doit jamais
    //@human être traité comme « sain » (C-12, fail-closed).
    /// @depends caringnanny_health_status
    Inconnu,
}

impl HealthStatus {
    /// @id caringnanny_health_status_rang_gravite
    /// @role accessor
    /// @layer core
    /// @do rank_health_status_severity
    //@human / @human Rang de gravité pour l'agrégation (le pire l'emporte) :
    //@human Healthy < Degraded < Inconnu < Unhealthy. `Inconnu` n'est jamais
    //@human meilleur que `Degraded` (fail-closed, C-12).
    /// @depends caringnanny_health_status
    pub fn rang_gravite(self) -> u8 {
        match self {
            HealthStatus::Healthy => 0,
            HealthStatus::Degraded => 1,
            HealthStatus::Inconnu => 2,
            HealthStatus::Unhealthy => 3,
        }
    }

    /// Le pire des deux statuts.
    pub fn pire(self, autre: HealthStatus) -> HealthStatus {
        if autre.rang_gravite() > self.rang_gravite() {
            autre
        } else {
            self
        }
    }
}

/// @id caringnanny_health_checker_trait
/// @role infrastructure
/// @layer core
/// @do define_health_checker_contract
//@human / @human Trait de vérification de l'état de santé.
pub trait HealthChecker {
    /// @id caringnanny_health_checker_check
    /// @role accessor
    /// @layer core
    /// @do check_component_health
    //@human / @human Vérifie l'état de santé d'un composant.
    /// @depends caringnanny_health_checker_trait
    fn check(&self, component: &str) -> HealthStatus;
}

/// Vérificateur par défaut : registre composant -> statut, défaut Healthy.
#[derive(Debug, Default)]
pub struct DefaultHealthChecker {
    status_by_component: std::collections::HashMap<String, HealthStatus>,
}

impl DefaultHealthChecker {
    /// Crée un vérificateur vide (tous les composants sont `Inconnu` tant
    /// qu'aucun statut n'a été enregistré).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Enregistre le statut d'un composant (pour tests ou mise à jour par observations).
    pub fn set_status(&mut self, component: impl Into<String>, status: HealthStatus) {
        self.status_by_component.insert(component.into(), status);
    }
}

impl HealthChecker for DefaultHealthChecker {
    fn check(&self, component: &str) -> HealthStatus {
        self.status_by_component
            .get(component)
            .copied()
            .unwrap_or(HealthStatus::Inconnu)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id caringnanny_health_test_status_equality
    /// @role test
    /// @layer core
    /// @do verify_health_status_equality
    //@human / @human Test d'égalité des états de santé.
    /// @depends caringnanny_health_status
    #[test]
    fn test_health_status_equality() {
        assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
        assert_ne!(HealthStatus::Healthy, HealthStatus::Unhealthy);
    }

    /// @id caringnanny_health_test_composant_inconnu
    /// @role test
    /// @layer core
    /// @do verify_unknown_component_not_healthy
    //@human / @human Un composant jamais observé est `Inconnu`, pas `Healthy`.
    /// @depends caringnanny_health_status_inconnu
    #[test]
    fn test_default_health_checker() {
        let mut checker = DefaultHealthChecker::new();
        assert_eq!(checker.check("unknown"), HealthStatus::Inconnu);
        assert_ne!(checker.check("unknown"), HealthStatus::Healthy);
        checker.set_status("db", HealthStatus::Degraded);
        assert_eq!(checker.check("db"), HealthStatus::Degraded);
    }
}
