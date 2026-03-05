//! Executeur d'actions pour les automatisations.
//!
//! Execute une sequence d'actions sequentiellement, avec delais optionnels.

// @id: toolkit.alicia.automations.executor
// @role: action_executor
// @layer: 6
// @human: Executeur d'actions domotiques : commandes sequentielles avec delais.
// @do: execute_automation_actions_sequentially

use std::sync::Arc;

use miyualicia_devices::{CommandSource, DeviceCommand};
use uuid::Uuid;

use crate::dispatcher::AliciaCommandDispatcher;
use crate::errors::AutomationError;
use crate::types::Action;

/// Execute une sequence d'actions d'une automatisation.
///
/// # Sequentialite
///
/// Les actions sont executees l'une apres l'autre, dans l'ordre.
/// Si une action echoue, l'execution continue avec les suivantes
/// (pas d'arret sur erreur partielle). Les erreurs sont collectees et loggees.
///
/// # Delais
///
/// `tokio::time::sleep(Duration::from_millis(action.delay_ms))` avant chaque action.
/// Un `delay_ms = 0` ne cree pas de `sleep` (optimisation).
///
/// # Source des commandes
///
/// Toutes les commandes issues du moteur d'automatisations portent
/// `CommandSource::Automation` pour l'audit trail.
pub async fn execute_actions(
    actions: &[Action],
    dispatcher: Arc<dyn AliciaCommandDispatcher>,
    automation_id: Uuid,
) -> Result<(), AutomationError> {
    let mut errors: Vec<String> = Vec::new();

    for (i, action) in actions.iter().enumerate() {
        // Delai avant execution si > 0
        if action.delay_ms > 0 {
            tokio::time::sleep(std::time::Duration::from_millis(action.delay_ms)).await;
        }

        // Construire la commande
        let command = match &action.value {
            Some(val) => DeviceCommand::with_value(
                action.device_id,
                action.command.clone(),
                val.clone(),
                CommandSource::Automation,
            ),
            None => DeviceCommand::simple(
                action.device_id,
                action.command.clone(),
                CommandSource::Automation,
            ),
        };

        tracing::info!(
            automation_id = %automation_id,
            action_index = i,
            device_id = %action.device_id,
            command = %action.command,
            "execution action automatisation"
        );

        // Dispatch
        if let Err(e) = dispatcher.dispatch_command(command) {
            let msg = format!(
                "action #{} (device={}, cmd={}): {}",
                i + 1,
                action.device_id,
                action.command,
                e
            );
            tracing::error!(
                automation_id = %automation_id,
                error = %msg,
                "echec dispatch action automatisation"
            );
            errors.push(msg);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(AutomationError::DispatchError(errors.join("; ")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dispatcher::AliciaCommandDispatcher;
    use miyualicia_devices::{DeviceCommand, DeviceState};
    use std::sync::Mutex;

    /// Mock dispatcher qui enregistre les commandes recues.
    struct MockDispatcher {
        commands: Mutex<Vec<DeviceCommand>>,
        should_fail: bool,
    }

    impl MockDispatcher {
        fn new() -> Self {
            Self {
                commands: Mutex::new(Vec::new()),
                should_fail: false,
            }
        }

        fn failing() -> Self {
            Self {
                commands: Mutex::new(Vec::new()),
                should_fail: true,
            }
        }

        fn commands_count(&self) -> usize {
            self.commands.lock().expect("lock").len()
        }
    }

    impl AliciaCommandDispatcher for MockDispatcher {
        fn dispatch_command(&self, command: DeviceCommand) -> Result<(), AutomationError> {
            if self.should_fail {
                return Err(AutomationError::DispatchError(
                    "mock dispatch failure".to_string(),
                ));
            }
            self.commands.lock().expect("lock").push(command);
            Ok(())
        }

        fn get_device_state(&self, _device_id: &Uuid) -> Option<DeviceState> {
            None
        }
    }

    // TC-AUTO-18 (simplified) : execute_actions dispatches toutes les commandes
    #[tokio::test]
    async fn test_execute_actions_success() {
        let dispatcher = Arc::new(MockDispatcher::new());
        let auto_id = Uuid::new_v4();

        let actions = vec![
            Action {
                device_id: Uuid::new_v4(),
                command: "off".to_string(),
                value: None,
                delay_ms: 0,
            },
            Action {
                device_id: Uuid::new_v4(),
                command: "set_brightness".to_string(),
                value: Some(serde_json::json!(50)),
                delay_ms: 0,
            },
        ];

        let result = execute_actions(&actions, dispatcher.clone(), auto_id).await;
        assert!(result.is_ok());
        assert_eq!(dispatcher.commands_count(), 2);
    }

    #[tokio::test]
    async fn test_execute_actions_empty() {
        let dispatcher = Arc::new(MockDispatcher::new());
        let auto_id = Uuid::new_v4();

        let result = execute_actions(&[], dispatcher.clone(), auto_id).await;
        assert!(result.is_ok());
        assert_eq!(dispatcher.commands_count(), 0);
    }

    #[tokio::test]
    async fn test_execute_actions_dispatch_failure() {
        let dispatcher = Arc::new(MockDispatcher::failing());
        let auto_id = Uuid::new_v4();

        let actions = vec![Action {
            device_id: Uuid::new_v4(),
            command: "on".to_string(),
            value: None,
            delay_ms: 0,
        }];

        let result = execute_actions(&actions, dispatcher.clone(), auto_id).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AutomationError::DispatchError(_)
        ));
    }

    #[tokio::test]
    async fn test_execute_actions_command_source_is_automation() {
        let dispatcher = Arc::new(MockDispatcher::new());
        let auto_id = Uuid::new_v4();

        let actions = vec![Action {
            device_id: Uuid::new_v4(),
            command: "on".to_string(),
            value: None,
            delay_ms: 0,
        }];

        execute_actions(&actions, dispatcher.clone(), auto_id)
            .await
            .expect("should succeed");

        let cmds = dispatcher.commands.lock().expect("lock");
        assert_eq!(cmds[0].source, CommandSource::Automation);
    }

    // TC-AUTO-18 : execute_actions respecte les delais
    #[tokio::test]
    async fn test_execute_actions_with_delay() {
        let dispatcher = Arc::new(MockDispatcher::new());
        let auto_id = Uuid::new_v4();

        let actions = vec![
            Action {
                device_id: Uuid::new_v4(),
                command: "off".to_string(),
                value: None,
                delay_ms: 0,
            },
            Action {
                device_id: Uuid::new_v4(),
                command: "on".to_string(),
                value: None,
                delay_ms: 50, // petit delai pour le test
            },
        ];

        let start = std::time::Instant::now();
        let result = execute_actions(&actions, dispatcher.clone(), auto_id).await;
        let elapsed = start.elapsed();

        assert!(result.is_ok());
        assert_eq!(dispatcher.commands_count(), 2);
        // Verifie que le delai a bien ete respecte (au moins 40ms pour tolerance)
        assert!(
            elapsed.as_millis() >= 40,
            "elapsed: {}ms",
            elapsed.as_millis()
        );
    }
}
