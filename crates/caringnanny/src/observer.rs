//! Module d'observation d'événements de CaringNanny

/// @id: caringnanny_observer_event_type
/// @role: data
/// @layer: core
/// @human: Type d'événement système observé.
/// @do: represent_event_type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    /// @id: caringnanny_observer_event_state_change
    /// @role: data
    /// @layer: core
    /// @human: Changement d'état système.
    /// @do: represent_state_change_event
    /// @depends: caringnanny_observer_event_type
    StateChange,
    /// @id: caringnanny_observer_event_anomaly
    /// @role: data
    /// @layer: core
    /// @human: Anomalie détectée.
    /// @do: represent_anomaly_event
    /// @depends: caringnanny_observer_event_type
    Anomaly,
    /// @id: caringnanny_observer_event_health_change
    /// @role: data
    /// @layer: core
    /// @human: Changement d'état de santé.
    /// @do: represent_health_change_event
    /// @depends: caringnanny_observer_event_type
    HealthChange,
}

/// @id: caringnanny_observer_system_event
/// @role: data
/// @layer: core
/// @human: Événement système observé.
/// @do: represent_system_event
#[derive(Debug, Clone)]
pub struct SystemEvent {
    /// @id: caringnanny_observer_system_event_type
    /// @role: data
    /// @layer: core
    /// @human: Type d'événement.
    /// @do: store_event_type
    /// @depends: caringnanny_observer_system_event
    pub event_type: EventType,
    /// @id: caringnanny_observer_system_event_component
    /// @role: data
    /// @layer: core
    /// @human: Composant concerné.
    /// @do: store_component_id
    /// @depends: caringnanny_observer_system_event
    pub component: String,
    /// @id: caringnanny_observer_system_event_message
    /// @role: data
    /// @layer: core
    /// @human: Message descriptif de l'événement.
    /// @do: store_event_message
    /// @depends: caringnanny_observer_system_event
    pub message: String,
}

/// @id: caringnanny_observer_trait
/// @role: infrastructure
/// @layer: core
/// @human: Trait d'observation d'événements système.
/// @do: define_observer_contract
pub trait Observer {
    /// @id: caringnanny_observer_observe
    /// @role: infrastructure
    /// @layer: core
    /// @human: Observe un événement système.
    /// @do: observe_system_event
    /// @depends: caringnanny_observer_trait
    fn observe(&mut self, event: SystemEvent);
}

/// Observateur par défaut : enregistre les événements en mémoire (file pour traitement ultérieur).
#[derive(Debug, Default)]
pub struct DefaultObserver {
    events: Vec<SystemEvent>,
}

impl DefaultObserver {
    /// Crée un observateur vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Retourne les événements observés (lecture).
    #[must_use]
    pub fn events(&self) -> &[SystemEvent] {
        &self.events
    }
}

impl Observer for DefaultObserver {
    fn observe(&mut self, event: SystemEvent) {
        self.events.push(event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: caringnanny_observer_test_event_creation
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'un événement système.
    /// @do: verify_event_creation
    /// @depends: caringnanny_observer_system_event
    #[test]
    fn test_event_creation() {
        let event = SystemEvent {
            event_type: EventType::StateChange,
            component: "test-component".to_string(),
            message: "Test event".to_string(),
        };
        assert_eq!(event.event_type, EventType::StateChange);
    }

    #[test]
    fn test_default_observer() {
        let mut obs = DefaultObserver::new();
        obs.observe(SystemEvent {
            event_type: EventType::Anomaly,
            component: "c1".to_string(),
            message: "msg".to_string(),
        });
        assert_eq!(obs.events().len(), 1);
        assert_eq!(obs.events()[0].component, "c1");
    }
}
