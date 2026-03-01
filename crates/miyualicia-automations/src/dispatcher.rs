//! Trait d'injection de dependances pour le dispatch des commandes.
//!
//! `AliciaCommandDispatcher` est implemente par `miyualicia::AliciaService`
//! pour eviter la dependance circulaire. `HomeSnapshot` fournit un instantane
//! de l'etat de la maison pour l'evaluateur de conditions.

// @id: toolkit.alicia.automations.dispatcher
// @role: command_dispatcher_trait
// @layer: 6
// @human: Trait d'injection de dependances pour le dispatch des commandes Alicia.
// @do: define_dispatcher_trait_and_home_snapshot

use std::collections::HashMap;

use miyualicia_devices::{DeviceCommand, DeviceState};
use uuid::Uuid;

use crate::errors::AutomationError;

/// Interface de dispatch des commandes vers le service Alicia.
///
/// Ce trait permet de tester `AutomationEngine` sans dependre de `miyualicia`
/// (evite la dependance circulaire). `miyualicia::AliciaService` implemente ce trait.
pub trait AliciaCommandDispatcher: Send + Sync {
    /// Envoie une commande vers un dispositif.
    fn dispatch_command(
        &self,
        command: DeviceCommand,
    ) -> Result<(), AutomationError>;

    /// Retourne l'etat courant d'un dispositif.
    fn get_device_state(&self, device_id: &Uuid) -> Option<DeviceState>;
}

/// Snapshot de l'etat de la maison pour l'evaluateur de conditions.
///
/// Cree par `AliciaService` au moment de l'evaluation d'une automatisation.
/// Contient les etats de tous les dispositifs et le contexte temporel.
#[derive(Debug, Clone)]
pub struct HomeSnapshot {
    /// Heure courante (0-23).
    pub hour: u32,
    /// Jour de la semaine (0=lundi, 6=dimanche).
    pub weekday: u32,
    /// Etats de tous les dispositifs, indexes par UUID.
    pub device_states: HashMap<Uuid, DeviceState>,
}

impl HomeSnapshot {
    /// Cree un snapshot a partir du dispatcher.
    ///
    /// Collecte l'heure locale et le jour de la semaine depuis le systeme.
    pub fn from_dispatcher(dispatcher: &dyn AliciaCommandDispatcher, device_ids: &[Uuid]) -> Self {
        let now = chrono::Local::now();
        let hour = now.format("%H").to_string().parse::<u32>().unwrap_or(0);
        let weekday = now.format("%u").to_string().parse::<u32>().unwrap_or(1) - 1; // %u = 1-7, on veut 0-6

        let mut device_states = HashMap::new();
        for id in device_ids {
            if let Some(state) = dispatcher.get_device_state(id) {
                device_states.insert(*id, state);
            }
        }

        Self {
            hour,
            weekday,
            device_states,
        }
    }

    /// Cree un snapshot vide (pour les tests).
    pub fn empty() -> Self {
        Self {
            hour: 0,
            weekday: 0,
            device_states: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_home_snapshot_empty() {
        let snap = HomeSnapshot::empty();
        assert_eq!(snap.hour, 0);
        assert_eq!(snap.weekday, 0);
        assert!(snap.device_states.is_empty());
    }

    #[test]
    fn test_home_snapshot_with_states() {
        let id = Uuid::new_v4();
        let mut snap = HomeSnapshot::empty();
        snap.device_states.insert(id, DeviceState::unknown(id));
        assert_eq!(snap.device_states.len(), 1);
        assert!(snap.device_states.contains_key(&id));
    }
}
