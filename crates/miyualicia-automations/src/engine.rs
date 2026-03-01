//! Moteur d'automatisations Alicia.
//!
//! `AutomationEngine` gere le cycle complet : declenchement, evaluation des conditions,
//! execution des actions. Il maintient un registre d'automatisations et un canal d'evenements.

// @id: toolkit.alicia.automations.engine
// @role: automation_scheduler
// @layer: 6
// @human: Moteur central des automatisations Alicia : registre, evenements, evaluation, execution.
// @do: schedule_and_execute_home_automations

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;

use miyualicia_devices::DeviceState;

use crate::dispatcher::{AliciaCommandDispatcher, HomeSnapshot};
use crate::errors::AutomationError;
use crate::evaluator::evaluate_conditions;
use crate::executor::execute_actions;
use crate::types::Automation;

/// Moteur d'automatisations Alicia.
///
/// # Architecture interne
///
/// Le moteur maintient :
/// 1. Une map d'automatisations en memoire (`Arc<RwLock<HashMap<Uuid, Automation>>>`)
/// 2. Un dispatcher pour envoyer les commandes aux dispositifs
///
/// # Thread-safety
///
/// `AutomationEngine` est Clone (Arc interne). Partager l'engine entre
/// le service Alicia et les handlers API est safe.
#[derive(Clone)]
pub struct AutomationEngine {
    inner: Arc<AutomationEngineInner>,
}

struct AutomationEngineInner {
    dispatcher: Arc<dyn AliciaCommandDispatcher>,
    automations: RwLock<HashMap<Uuid, Automation>>,
}

impl AutomationEngine {
    /// Cree le moteur avec le dispatcher et les automatisations initiales (chargees depuis DB).
    pub fn new(
        dispatcher: Arc<dyn AliciaCommandDispatcher>,
        initial_automations: Vec<Automation>,
    ) -> Self {
        let mut map = HashMap::new();
        for auto in initial_automations {
            map.insert(auto.id, auto);
        }

        Self {
            inner: Arc::new(AutomationEngineInner {
                dispatcher,
                automations: RwLock::new(map),
            }),
        }
    }

    /// Ajoute une automatisation au registre.
    ///
    /// Appelle `automation.validate()` avant ajout.
    pub async fn add_automation(&self, automation: Automation) -> Result<(), AutomationError> {
        automation.validate()?;

        let mut automations = self.inner.automations.write().await;
        automations.insert(automation.id, automation);

        tracing::info!("automatisation ajoutee au moteur");
        Ok(())
    }

    /// Supprime une automatisation par UUID.
    pub async fn remove_automation(&self, id: Uuid) -> Result<(), AutomationError> {
        let mut automations = self.inner.automations.write().await;
        if automations.remove(&id).is_some() {
            tracing::info!(automation_id = %id, "automatisation supprimee");
            Ok(())
        } else {
            Err(AutomationError::NotFound(id))
        }
    }

    /// Active ou desactive une automatisation.
    pub async fn set_enabled(&self, id: Uuid, enabled: bool) -> Result<(), AutomationError> {
        let mut automations = self.inner.automations.write().await;
        let auto = automations
            .get_mut(&id)
            .ok_or(AutomationError::NotFound(id))?;
        auto.enabled = enabled;
        auto.updated_at = chrono::Utc::now();
        tracing::info!(automation_id = %id, enabled = enabled, "automatisation enabled mise a jour");
        Ok(())
    }

    /// Retourne une automatisation par UUID.
    pub async fn get_automation(&self, id: Uuid) -> Result<Automation, AutomationError> {
        let automations = self.inner.automations.read().await;
        automations
            .get(&id)
            .cloned()
            .ok_or(AutomationError::NotFound(id))
    }

    /// Retourne la liste de toutes les automatisations.
    pub async fn list_automations(&self) -> Vec<Automation> {
        let automations = self.inner.automations.read().await;
        automations.values().cloned().collect()
    }

    /// Declenche manuellement une automatisation (ignore les conditions).
    ///
    /// Utilisee par `POST /automations/{id}/trigger` et les tests.
    pub async fn trigger_automation(&self, id: Uuid) -> Result<(), AutomationError> {
        let automation = {
            let automations = self.inner.automations.read().await;
            automations
                .get(&id)
                .cloned()
                .ok_or(AutomationError::NotFound(id))?
        };

        tracing::info!(
            automation_id = %id,
            name = %automation.name,
            "declenchement manuel de l'automatisation"
        );

        execute_actions(
            &automation.actions,
            Arc::clone(&self.inner.dispatcher),
            automation.id,
        )
        .await?;

        // Mettre a jour last_triggered_at
        let mut automations = self.inner.automations.write().await;
        if let Some(auto) = automations.get_mut(&id) {
            auto.last_triggered_at = Some(chrono::Utc::now());
        }

        Ok(())
    }

    /// Evalue et execute toutes les automatisations correspondant a un changement d'etat capteur.
    ///
    /// 1. Filtrer les automatisations activees avec trigger `SensorChange` pour le device_id
    /// 2. Construire un `HomeSnapshot` depuis le dispatcher
    /// 3. Pour chaque candidat : evaluer les conditions
    /// 4. Si toutes les conditions sont vraies : executer les actions
    /// 5. Mettre a jour `last_triggered_at`
    pub async fn handle_device_state_changed(
        &self,
        device_id: Uuid,
        _state: DeviceState,
    ) -> Result<(), AutomationError> {
        // Collecter les candidats
        let candidates: Vec<Automation> = {
            let automations = self.inner.automations.read().await;
            automations
                .values()
                .filter(|a| a.matches_sensor_change(device_id))
                .cloned()
                .collect()
        };

        if candidates.is_empty() {
            return Ok(());
        }

        // Collecter tous les device_ids necessaires pour le snapshot
        let all_device_ids: Vec<Uuid> = {
            let automations = self.inner.automations.read().await;
            automations
                .values()
                .flat_map(|a| {
                    a.conditions
                        .iter()
                        .filter_map(|c| c.device_id)
                })
                .collect()
        };

        // Construire le snapshot
        let snapshot =
            HomeSnapshot::from_dispatcher(self.inner.dispatcher.as_ref(), &all_device_ids);

        for candidate in &candidates {
            if evaluate_conditions(&candidate.conditions, &snapshot) {
                tracing::info!(
                    automation_id = %candidate.id,
                    name = %candidate.name,
                    trigger = %candidate.trigger.description(),
                    "conditions remplies, execution des actions"
                );

                if let Err(e) = execute_actions(
                    &candidate.actions,
                    Arc::clone(&self.inner.dispatcher),
                    candidate.id,
                )
                .await
                {
                    tracing::error!(
                        automation_id = %candidate.id,
                        error = %e,
                        "erreur execution automatisation"
                    );
                }

                // Mettre a jour last_triggered_at
                let mut automations = self.inner.automations.write().await;
                if let Some(auto) = automations.get_mut(&candidate.id) {
                    auto.last_triggered_at = Some(chrono::Utc::now());
                }
            }
        }

        Ok(())
    }

    /// Evalue et execute toutes les automatisations correspondant a une routine vocale.
    pub async fn handle_voice_routine(
        &self,
        routine_name: &str,
    ) -> Result<(), AutomationError> {
        let candidates: Vec<Automation> = {
            let automations = self.inner.automations.read().await;
            automations
                .values()
                .filter(|a| a.matches_routine(routine_name))
                .cloned()
                .collect()
        };

        if candidates.is_empty() {
            tracing::debug!(
                routine = %routine_name,
                "aucune automatisation trouvee pour cette routine vocale"
            );
            return Ok(());
        }

        // Collecter les device_ids pour le snapshot
        let all_device_ids: Vec<Uuid> = candidates
            .iter()
            .flat_map(|a| a.conditions.iter().filter_map(|c| c.device_id))
            .collect();

        let snapshot =
            HomeSnapshot::from_dispatcher(self.inner.dispatcher.as_ref(), &all_device_ids);

        for candidate in &candidates {
            if evaluate_conditions(&candidate.conditions, &snapshot) {
                tracing::info!(
                    automation_id = %candidate.id,
                    name = %candidate.name,
                    routine = %routine_name,
                    "routine vocale : conditions remplies, execution"
                );

                if let Err(e) = execute_actions(
                    &candidate.actions,
                    Arc::clone(&self.inner.dispatcher),
                    candidate.id,
                )
                .await
                {
                    tracing::error!(
                        automation_id = %candidate.id,
                        error = %e,
                        "erreur execution routine vocale"
                    );
                }

                // Mettre a jour last_triggered_at
                let mut automations = self.inner.automations.write().await;
                if let Some(auto) = automations.get_mut(&candidate.id) {
                    auto.last_triggered_at = Some(chrono::Utc::now());
                }
            }
        }

        Ok(())
    }

    /// Retourne une reference au dispatcher (pour les tests).
    pub fn dispatcher(&self) -> &Arc<dyn AliciaCommandDispatcher> {
        &self.inner.dispatcher
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dispatcher::AliciaCommandDispatcher;
    use crate::types::{Action, ConditionOp, TriggerType};
    use miyualicia_devices::{DeviceCommand, DeviceState};
    use std::sync::Mutex;

    /// Mock dispatcher pour les tests du moteur.
    struct MockDispatcher {
        commands: Mutex<Vec<DeviceCommand>>,
        states: Mutex<HashMap<Uuid, DeviceState>>,
    }

    impl MockDispatcher {
        fn new() -> Self {
            Self {
                commands: Mutex::new(Vec::new()),
                states: Mutex::new(HashMap::new()),
            }
        }

        fn with_state(device_id: Uuid, state: DeviceState) -> Self {
            let mut states = HashMap::new();
            states.insert(device_id, state);
            Self {
                commands: Mutex::new(Vec::new()),
                states: Mutex::new(states),
            }
        }

        fn commands_count(&self) -> usize {
            self.commands.lock().expect("lock").len()
        }
    }

    impl AliciaCommandDispatcher for MockDispatcher {
        fn dispatch_command(
            &self,
            command: DeviceCommand,
        ) -> Result<(), AutomationError> {
            self.commands.lock().expect("lock").push(command);
            Ok(())
        }

        fn get_device_state(&self, device_id: &Uuid) -> Option<DeviceState> {
            self.states.lock().expect("lock").get(device_id).cloned()
        }
    }

    fn make_action(device_id: Uuid) -> Action {
        Action {
            device_id,
            command: "on".to_string(),
            value: None,
            delay_ms: 0,
        }
    }

    fn make_automation_voice(name: &str, routine: &str) -> Automation {
        Automation {
            id: Uuid::new_v4(),
            name: name.to_string(),
            enabled: true,
            trigger: TriggerType::VoiceCommand {
                routine_name: routine.to_string(),
            },
            conditions: vec![],
            actions: vec![make_action(Uuid::new_v4())],
            last_triggered_at: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn make_automation_sensor(
        name: &str,
        device_id: Uuid,
        action_device_id: Uuid,
    ) -> Automation {
        Automation {
            id: Uuid::new_v4(),
            name: name.to_string(),
            enabled: true,
            trigger: TriggerType::SensorChange {
                device_id,
                property: "temperature_current".to_string(),
                op: ConditionOp::Gt,
                threshold: serde_json::json!(25.0),
            },
            conditions: vec![],
            actions: vec![make_action(action_device_id)],
            last_triggered_at: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    // TC-AUTO-11 : engine add/remove/list
    #[tokio::test]
    async fn test_add_remove_list() {
        let dispatcher = Arc::new(MockDispatcher::new());
        let engine = AutomationEngine::new(dispatcher, vec![]);

        let auto = make_automation_voice("Test", "test");
        let auto_id = auto.id;

        // Add
        engine.add_automation(auto).await.expect("add should work");
        let list = engine.list_automations().await;
        assert_eq!(list.len(), 1);

        // Get
        let found = engine.get_automation(auto_id).await;
        assert!(found.is_ok());
        assert_eq!(found.expect("found").name, "Test");

        // Remove
        engine
            .remove_automation(auto_id)
            .await
            .expect("remove should work");
        let list = engine.list_automations().await;
        assert!(list.is_empty());

        // Remove non-existent
        let err = engine.remove_automation(Uuid::new_v4()).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn test_add_invalid_automation_rejected() {
        let dispatcher = Arc::new(MockDispatcher::new());
        let engine = AutomationEngine::new(dispatcher, vec![]);

        let auto = Automation {
            id: Uuid::new_v4(),
            name: "".to_string(),
            enabled: true,
            trigger: TriggerType::VoiceCommand {
                routine_name: "test".to_string(),
            },
            conditions: vec![],
            actions: vec![make_action(Uuid::new_v4())],
            last_triggered_at: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let result = engine.add_automation(auto).await;
        assert!(result.is_err());
    }

    // TC-AUTO-14 : trigger VoiceCommand matching
    #[tokio::test]
    async fn test_voice_routine_matching() {
        let dispatcher = Arc::new(MockDispatcher::new());
        let auto = make_automation_voice("Bonne nuit", "bonne nuit");

        let engine = AutomationEngine::new(dispatcher.clone(), vec![auto]);

        engine
            .handle_voice_routine("bonne nuit")
            .await
            .expect("should work");
        assert_eq!(dispatcher.commands_count(), 1);

        // Non-matching routine should not trigger
        engine
            .handle_voice_routine("bonjour")
            .await
            .expect("should work");
        // Still 1 command (the previous one)
        assert_eq!(dispatcher.commands_count(), 1);
    }

    // TC-AUTO-15 : trigger SensorChange matching
    #[tokio::test]
    async fn test_sensor_change_matching() {
        let sensor_id = Uuid::new_v4();
        let action_device_id = Uuid::new_v4();
        let dispatcher = Arc::new(MockDispatcher::new());
        let auto = make_automation_sensor("Temp alert", sensor_id, action_device_id);

        let engine = AutomationEngine::new(dispatcher.clone(), vec![auto]);

        let state = DeviceState::unknown(sensor_id);
        engine
            .handle_device_state_changed(sensor_id, state)
            .await
            .expect("should work");
        // L'automatisation n'a pas de conditions, donc elle devrait s'executer
        assert_eq!(dispatcher.commands_count(), 1);
    }

    // TC-AUTO-16 : automatisation disabled n'est pas declenchee
    #[tokio::test]
    async fn test_disabled_automation_not_triggered() {
        let dispatcher = Arc::new(MockDispatcher::new());
        let mut auto = make_automation_voice("Disabled", "bonne nuit");
        auto.enabled = false;

        let engine = AutomationEngine::new(dispatcher.clone(), vec![auto]);

        engine
            .handle_voice_routine("bonne nuit")
            .await
            .expect("should work");
        assert_eq!(dispatcher.commands_count(), 0);
    }

    // TC-AUTO-17 : handle_voice_routine trouve la bonne automatisation
    #[tokio::test]
    async fn test_voice_routine_selects_correct_automation() {
        let dispatcher = Arc::new(MockDispatcher::new());
        let auto1 = make_automation_voice("Bonne nuit", "bonne nuit");
        let auto2 = make_automation_voice("Bonjour", "bonjour");

        let engine = AutomationEngine::new(dispatcher.clone(), vec![auto1, auto2]);

        engine
            .handle_voice_routine("bonjour")
            .await
            .expect("should work");
        assert_eq!(dispatcher.commands_count(), 1);
    }

    #[tokio::test]
    async fn test_trigger_automation_manual() {
        let dispatcher = Arc::new(MockDispatcher::new());
        let auto = make_automation_voice("Test manual", "test");
        let auto_id = auto.id;

        let engine = AutomationEngine::new(dispatcher.clone(), vec![auto]);

        engine
            .trigger_automation(auto_id)
            .await
            .expect("should work");
        assert_eq!(dispatcher.commands_count(), 1);

        // Verify last_triggered_at was updated
        let updated = engine.get_automation(auto_id).await.expect("should exist");
        assert!(updated.last_triggered_at.is_some());
    }

    #[tokio::test]
    async fn test_trigger_nonexistent_automation() {
        let dispatcher = Arc::new(MockDispatcher::new());
        let engine = AutomationEngine::new(dispatcher, vec![]);

        let result = engine.trigger_automation(Uuid::new_v4()).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AutomationError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_set_enabled() {
        let dispatcher = Arc::new(MockDispatcher::new());
        let auto = make_automation_voice("Test enable", "test");
        let auto_id = auto.id;

        let engine = AutomationEngine::new(dispatcher, vec![auto]);

        engine
            .set_enabled(auto_id, false)
            .await
            .expect("should work");
        let updated = engine.get_automation(auto_id).await.expect("should exist");
        assert!(!updated.enabled);

        engine
            .set_enabled(auto_id, true)
            .await
            .expect("should work");
        let updated = engine.get_automation(auto_id).await.expect("should exist");
        assert!(updated.enabled);
    }

    #[tokio::test]
    async fn test_initial_automations_loaded() {
        let dispatcher = Arc::new(MockDispatcher::new());
        let auto1 = make_automation_voice("Auto 1", "routine1");
        let auto2 = make_automation_voice("Auto 2", "routine2");

        let engine = AutomationEngine::new(dispatcher, vec![auto1, auto2]);
        let list = engine.list_automations().await;
        assert_eq!(list.len(), 2);
    }

    #[tokio::test]
    async fn test_sensor_change_with_conditions() {
        let sensor_id = Uuid::new_v4();
        let action_device_id = Uuid::new_v4();

        let mut state = DeviceState::unknown(sensor_id);
        state.temperature_current = Some(26.0);
        let dispatcher = Arc::new(MockDispatcher::with_state(sensor_id, state.clone()));

        let mut auto = make_automation_sensor("Temp alert", sensor_id, action_device_id);
        // Ajouter une condition : temperature > 25
        auto.conditions.push(crate::types::Condition {
            device_id: Some(sensor_id),
            property: "temperature_current".to_string(),
            op: ConditionOp::Gt,
            value: serde_json::json!(25.0),
        });

        let engine = AutomationEngine::new(dispatcher.clone(), vec![auto]);

        engine
            .handle_device_state_changed(sensor_id, state)
            .await
            .expect("should work");
        assert_eq!(dispatcher.commands_count(), 1);
    }

    #[tokio::test]
    async fn test_sensor_change_conditions_not_met() {
        let sensor_id = Uuid::new_v4();
        let action_device_id = Uuid::new_v4();

        let mut state = DeviceState::unknown(sensor_id);
        state.temperature_current = Some(20.0); // en dessous du seuil
        let dispatcher = Arc::new(MockDispatcher::with_state(sensor_id, state.clone()));

        let mut auto = make_automation_sensor("Temp alert", sensor_id, action_device_id);
        auto.conditions.push(crate::types::Condition {
            device_id: Some(sensor_id),
            property: "temperature_current".to_string(),
            op: ConditionOp::Gt,
            value: serde_json::json!(25.0),
        });

        let engine = AutomationEngine::new(dispatcher.clone(), vec![auto]);

        engine
            .handle_device_state_changed(sensor_id, state)
            .await
            .expect("should work");
        // Conditions non remplies, aucune commande envoyee
        assert_eq!(dispatcher.commands_count(), 0);
    }
}
