//! Tracer pour StrongFather
//!
//! Ce module implémente la traçabilité passive des évaluations et décisions de StrongFather.
//! Conforme au contrat Audit & Trace Contract et au Kernel Trace Access Contract.
//!
//! **Invariants respectés :**
//! - INV-TRACE-1 : Production obligatoire (toute évaluation produit des traces)
//! - INV-TRACE-2 : Production sans effet (la traçabilité ne modifie pas le comportement)
//! - INV-TRACE-3 : Production immédiate (traces produites au moment de l'événement)
//! - INV-TRACE-KERNEL : Utilisation kernel strictement passive
//!
//! **Garanties respectées :**
//! - KERN-AUTH-1 : Id pour identification de trace
//! - KERN-AUTH-2 : Logger pour enregistrement de trace
//! - KERN-AUTH-3 : Clock pour horodatage de trace uniquement
//!
//! **Règle de résilience :**
//! - R-TRACE-FAIL-1 : Échec de trace = Décision continue (la traçabilité ne bloque jamais la décision)

use crate::error::SFError;
use crate::intent::Intent;
use crate::decision::Decision;
use crate::types::DecisionType;
use miyukini_kernel::{Id, IdGenerator, Logger, Level, Clock};
use std::time::SystemTime;

/// Tracer pour StrongFather
///
/// Le Tracer est responsable de la traçabilité passive des évaluations et décisions.
/// Il utilise le kernel (Id, Logger, Clock) uniquement pour la traçabilité, conformément
/// au Kernel Trace Access Contract.
///
/// **Caractéristiques :**
/// - Traçabilité passive : n'influence jamais le résultat d'une évaluation
/// - Résilience : les échecs de traçabilité ne bloquent jamais la décision
/// - Immutabilité : les traces produites sont immuables
///
/// **Invariants :**
/// - INV-TRACE-KERNEL : Utilisation kernel strictement passive
/// - INV-TRACE-1 : Production obligatoire
/// - INV-TRACE-2 : Production sans effet
/// - INV-TRACE-3 : Production immédiate
pub struct Tracer<IG, L, C>
where
    IG: IdGenerator,
    L: Logger,
    C: Clock,
{
    /// Générateur d'identifiants pour les traces (KERN-AUTH-1)
    id_generator: IG,
    /// Logger pour l'enregistrement des traces (KERN-AUTH-2)
    logger: L,
    /// Horloge pour l'horodatage des traces (KERN-AUTH-3)
    clock: C,
}

impl<IG, L, C> Tracer<IG, L, C>
where
    IG: IdGenerator,
    L: Logger,
    C: Clock,
{
    /// Crée un nouveau Tracer avec les composants kernel fournis
    ///
    /// # Arguments
    /// * `id_generator` - Générateur d'identifiants pour les traces
    /// * `logger` - Logger pour l'enregistrement des traces
    /// * `clock` - Horloge pour l'horodatage des traces
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::tracer::Tracer;
    /// use miyukini_kernel::{UuidIdGenerator, DefaultLogger, DefaultClock};
    ///
    /// let id_gen = UuidIdGenerator::new();
    /// let logger = DefaultLogger::new();
    /// let clock = DefaultClock::new();
    /// let tracer = Tracer::new(id_gen, logger, clock);
    /// ```
    pub fn new(id_generator: IG, logger: L, clock: C) -> Self {
        Self {
            id_generator,
            logger,
            clock,
        }
    }

    /// Trace une évaluation complète (intention + décision)
    ///
    /// Cette méthode trace l'évaluation d'une intention et la décision produite,
    /// conformément au contrat Audit & Trace Contract.
    ///
    /// **Résilience :** Conforme à R-TRACE-FAIL-1, si la traçabilité échoue,
    /// cette méthode retourne une erreur mais ne bloque jamais la décision.
    /// L'appelant doit continuer normalement même en cas d'échec de trace.
    ///
    /// # Arguments
    /// * `intent` - L'intention évaluée
    /// * `decision` - La décision produite
    ///
    /// # Retour
    /// * `Ok(())` si la trace a été produite avec succès
    /// * `Err(SFError)` si la trace a échoué (mais la décision reste valide)
    ///
    /// # Conformité
    /// - Audit & Trace Contract section 3.1 (traces d'intention)
    /// - Audit & Trace Contract section 3.3 (traces de décision)
    /// - Kernel Trace Access Contract (KERN-AUTH-1, KERN-AUTH-2, KERN-AUTH-3)
    /// - R-TRACE-FAIL-1 (résilience)
    pub fn trace_evaluation(&self, intent: &Intent, decision: &Decision) -> Result<(), SFError> {
        // Génération de l'identifiant de trace (KERN-AUTH-1)
        // Résilience : si la génération échoue, on continue avec un identifiant dégradé
        let trace_id = self.generate_trace_id().unwrap_or_else(|| {
            // En cas d'échec, on utilise un identifiant dégradé
            // La trace sera marquée comme dégradée mais la décision continue
            Id::parse("00000000-0000-0000-0000-000000000000").unwrap_or_else(|_| {
                // Si même le parse échoue, on crée un Id minimal (ne devrait jamais arriver)
                // En production, on devrait logger cet échec mais continuer
                Id::parse("00000000-0000-0000-0000-000000000000").unwrap()
            })
        });

        // Horodatage de la trace (KERN-AUTH-3)
        // Résilience : si l'horodatage échoue, on continue sans horodatage
        let timestamp = self.clock.now();

        // Construction de la trace d'intention
        let intent_trace = self.build_intent_trace(intent, &trace_id, timestamp);
        
        // Construction de la trace de décision
        let decision_trace = self.build_decision_trace(decision, &trace_id, timestamp);

        // Enregistrement des traces (KERN-AUTH-2)
        // Résilience : si l'enregistrement échoue, on retourne une erreur mais la décision continue
        self.log_trace(&intent_trace)?;
        self.log_trace(&decision_trace)?;

        Ok(())
    }

    /// Génère un identifiant de trace unique
    ///
    /// Utilise le générateur d'identifiants du kernel (KERN-AUTH-1).
    /// Résilience : en cas d'échec, retourne None (l'appelant gère la dégradation).
    fn generate_trace_id(&self) -> Option<Id> {
        Some(self.id_generator.generate())
    }

    /// Construit la trace d'intention
    ///
    /// Conforme à Audit & Trace Contract section 3.1.
    fn build_intent_trace(&self, intent: &Intent, trace_id: &Id, timestamp: SystemTime) -> String {
        format!(
            "[TRACE-INTENT] id={} trace_id={} action_type={} subject={} caller={} origin={} instance={} timestamp={:?}",
            intent.intent_id(),
            trace_id,
            intent.action_type().as_str(),
            intent.subject(),
            intent.call_context().caller_identity(),
            intent.call_context().origin(),
            intent.call_context().instance(),
            timestamp
        )
    }

    /// Construit la trace de décision
    ///
    /// Conforme à Audit & Trace Contract section 3.3.
    fn build_decision_trace(&self, decision: &Decision, trace_id: &Id, timestamp: SystemTime) -> String {
        let decision_type_str = match decision.decision_type() {
            DecisionType::Accepted { priority } => {
                format!("ACCEPTED priority={}", priority.value())
            }
            DecisionType::Refused { reason, violated_policies } => {
                format!(
                    "REFUSED reason={:?} violated_policies={:?}",
                    reason,
                    violated_policies
                )
            }
            DecisionType::Ambiguous { missing_information, clarifications_required } => {
                format!(
                    "AMBIGUOUS missing={:?} clarifications={}",
                    missing_information,
                    clarifications_required.len()
                )
            }
            DecisionType::Deferred { reason, context_required } => {
                format!(
                    "DEFERRED reason={:?} context_required={:?}",
                    reason,
                    context_required
                )
            }
        };

        format!(
            "[TRACE-DECISION] intent_id={} trace_id={} decision_type={} policies_applied={:?} justification={} timestamp={:?}",
            decision.intent_id(),
            trace_id,
            decision_type_str,
            decision.policies_applied(),
            decision.justification().explanation,
            timestamp
        )
    }

    /// Enregistre une trace via le Logger
    ///
    /// Utilise le Logger du kernel (KERN-AUTH-2).
    /// Résilience : en cas d'échec, retourne une erreur (mais la décision continue).
    fn log_trace(&self, trace: &str) -> Result<(), SFError> {
        // Le Logger du kernel ne retourne pas d'erreur, donc on considère
        // que l'enregistrement réussit toujours. Si le Logger échoue en interne,
        // c'est géré par le produit, pas par StrongFather.
        self.logger.log(Level::Info, trace);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::{CallContext, Intent, IntentData};
    use crate::decision::{Decision, DecisionMetadata, EvaluationContext};
    use crate::types::{ActionType, DecisionType, Justification, Priority, RefusalReason};
    use miyukini_kernel::{UuidIdGenerator, DefaultLogger, DefaultClock};
    use std::time::SystemTime;

    // Logger de test qui capture les messages
    struct TestLogger {
        logs: std::sync::Arc<std::sync::Mutex<Vec<(Level, String)>>>,
    }

    impl TestLogger {
        fn new() -> Self {
            Self {
                logs: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            }
        }
    }

    impl Logger for TestLogger {
        fn log(&self, level: Level, message: &str) {
            self.logs.lock().unwrap().push((level, message.to_string()));
        }
    }

    // Clock de test avec temps fixe
    struct TestClock {
        time: SystemTime,
    }

    impl TestClock {
        fn new(time: SystemTime) -> Self {
            Self { time }
        }
    }

    impl Clock for TestClock {
        fn now(&self) -> SystemTime {
            self.time
        }
    }

    fn create_test_intent() -> Intent {
        let context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        Intent::new(
            "intent-001".to_string(),
            ActionType::Creation,
            "document".to_string(),
            context,
            IntentData::empty(),
        )
    }

    fn create_test_decision_accepted() -> Decision {
        let justification = Justification::new("Intention valide".to_string());
        let context = EvaluationContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let metadata = DecisionMetadata::new();

        Decision::new(
            "intent-001".to_string(),
            DecisionType::Accepted {
                priority: Priority::new(100),
            },
            justification,
            vec!["policy1".to_string()],
            context,
            metadata,
        )
    }

    fn create_test_decision_refused() -> Decision {
        let justification = Justification::new("Intention invalide".to_string());
        let context = EvaluationContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let metadata = DecisionMetadata::new();

        let reason = RefusalReason::PolicyViolation {
            description: "Politique violée".to_string(),
        };

        Decision::new(
            "intent-001".to_string(),
            DecisionType::Refused {
                reason,
                violated_policies: vec!["policy1".to_string()],
            },
            justification,
            vec!["policy1".to_string()],
            context,
            metadata,
        )
    }

    #[test]
    fn test_tracer_creation() {
        let id_gen = UuidIdGenerator::new();
        let logger = DefaultLogger::new();
        let clock = DefaultClock::new();
        let _tracer = Tracer::new(id_gen, logger, clock);
        
        // Vérification que le tracer est créé sans erreur
        // Le test passe si la création réussit (pas de panic)
    }

    #[test]
    fn test_trace_evaluation_accepted() {
        let id_gen = UuidIdGenerator::new();
        let logger = TestLogger::new();
        let logger_clone = std::sync::Arc::clone(&logger.logs);
        let clock = TestClock::new(SystemTime::now());
        let tracer = Tracer::new(id_gen, logger, clock);

        let intent = create_test_intent();
        let decision = create_test_decision_accepted();

        // La trace ne doit pas échouer
        let result = tracer.trace_evaluation(&intent, &decision);
        assert!(result.is_ok(), "La traçabilité ne doit pas échouer");

        // Vérification que les traces ont été enregistrées
        let logs = logger_clone.lock().unwrap();
        assert_eq!(logs.len(), 2, "Deux traces doivent être enregistrées (intention + décision)");

        // Vérification de la trace d'intention
        let intent_log = logs.iter().find(|(_, msg)| msg.contains("[TRACE-INTENT]"));
        assert!(intent_log.is_some(), "La trace d'intention doit être présente");
        let (_, intent_msg) = intent_log.unwrap();
        assert!(intent_msg.contains("intent-001"), "La trace doit contenir l'identifiant d'intention");
        assert!(intent_msg.contains("Creation"), "La trace doit contenir le type d'action");
        assert!(intent_msg.contains("document"), "La trace doit contenir le sujet");

        // Vérification de la trace de décision
        let decision_log = logs.iter().find(|(_, msg)| msg.contains("[TRACE-DECISION]"));
        assert!(decision_log.is_some(), "La trace de décision doit être présente");
        let (_, decision_msg) = decision_log.unwrap();
        assert!(decision_msg.contains("intent-001"), "La trace doit contenir l'identifiant d'intention");
        assert!(decision_msg.contains("ACCEPTED"), "La trace doit contenir le type de décision");
        assert!(decision_msg.contains("priority=100"), "La trace doit contenir la priorité");
    }

    #[test]
    fn test_trace_evaluation_refused() {
        let id_gen = UuidIdGenerator::new();
        let logger = TestLogger::new();
        let logger_clone = std::sync::Arc::clone(&logger.logs);
        let clock = TestClock::new(SystemTime::now());
        let tracer = Tracer::new(id_gen, logger, clock);

        let intent = create_test_intent();
        let decision = create_test_decision_refused();

        let result = tracer.trace_evaluation(&intent, &decision);
        assert!(result.is_ok(), "La traçabilité ne doit pas échouer");

        let logs = logger_clone.lock().unwrap();
        assert_eq!(logs.len(), 2, "Deux traces doivent être enregistrées");

        // Vérification de la trace de décision refusée
        let decision_log = logs.iter().find(|(_, msg)| msg.contains("[TRACE-DECISION]"));
        assert!(decision_log.is_some(), "La trace de décision doit être présente");
        let (_, decision_msg) = decision_log.unwrap();
        assert!(decision_msg.contains("REFUSED"), "La trace doit contenir le type de décision REFUSED");
    }

    #[test]
    fn test_trace_evaluation_correlation() {
        let id_gen = UuidIdGenerator::new();
        let logger = TestLogger::new();
        let logger_clone = std::sync::Arc::clone(&logger.logs);
        let clock = TestClock::new(SystemTime::now());
        let tracer = Tracer::new(id_gen, logger, clock);

        let intent = create_test_intent();
        let decision = create_test_decision_accepted();

        let result = tracer.trace_evaluation(&intent, &decision);
        assert!(result.is_ok());

        let logs = logger_clone.lock().unwrap();
        
        // Extraction des trace_id des deux traces
        let intent_trace_id = extract_trace_id(&logs[0].1);
        let decision_trace_id = extract_trace_id(&logs[1].1);

        // Les deux traces doivent avoir le même trace_id pour la corrélation
        assert_eq!(
            intent_trace_id, decision_trace_id,
            "Les traces d'intention et de décision doivent avoir le même trace_id pour la corrélation"
        );
    }

    fn extract_trace_id(trace: &str) -> String {
        // Extraction simple du trace_id depuis la trace
        // Format attendu: trace_id=...
        if let Some(start) = trace.find("trace_id=") {
            let start_idx = start + "trace_id=".len();
            let end_idx = trace[start_idx..]
                .find(' ')
                .map(|i| start_idx + i)
                .unwrap_or(trace.len());
            trace[start_idx..end_idx].to_string()
        } else {
            String::new()
        }
    }

    #[test]
    fn test_trace_evaluation_timestamp() {
        // Test que les traces contiennent un horodatage
        let id_gen = UuidIdGenerator::new();
        let logger = TestLogger::new();
        let logger_clone = std::sync::Arc::clone(&logger.logs);
        let fixed_time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1234567890);
        let clock = TestClock::new(fixed_time);
        let tracer = Tracer::new(id_gen, logger, clock);

        let intent = create_test_intent();
        let decision = create_test_decision_accepted();

        let _ = tracer.trace_evaluation(&intent, &decision);

        let logs = logger_clone.lock().unwrap();
        assert_eq!(logs.len(), 2);

        // Vérification que les traces contiennent un timestamp
        for (_, log_msg) in logs.iter() {
            assert!(log_msg.contains("timestamp="), "La trace doit contenir un horodatage");
        }
    }

    #[test]
    fn test_trace_evaluation_immutability() {
        // Test que la traçabilité ne modifie pas l'intention ni la décision
        let id_gen = UuidIdGenerator::new();
        let logger = TestLogger::new();
        let clock = TestClock::new(SystemTime::now());
        let tracer = Tracer::new(id_gen, logger, clock);

        let intent = create_test_intent();
        let decision = create_test_decision_accepted();

        // Sauvegarde des valeurs avant traçabilité
        let intent_id_before = intent.intent_id().to_string();
        let decision_type_before = decision.decision_type().clone();

        // Traçabilité
        let _ = tracer.trace_evaluation(&intent, &decision);

        // Vérification que les valeurs n'ont pas changé
        assert_eq!(intent.intent_id(), intent_id_before, "L'intention ne doit pas être modifiée");
        assert_eq!(
            decision.decision_type(),
            &decision_type_before,
            "La décision ne doit pas être modifiée"
        );
    }

    #[test]
    fn test_trace_evaluation_resilience() {
        // Test de résilience : même si la traçabilité échoue, la décision reste valide
        // Dans ce test, on utilise un logger qui fonctionne normalement
        // La résilience est testée par le fait que trace_evaluation retourne Ok
        // même si certains composants échouent (testé dans les tests d'intégration)
        
        let id_gen = UuidIdGenerator::new();
        let logger = TestLogger::new();
        let clock = TestClock::new(SystemTime::now());
        let _tracer = Tracer::new(id_gen, logger, clock);

        let intent = create_test_intent();
        let decision = create_test_decision_accepted();

        // La traçabilité ne doit pas échouer dans un cas normal
        let result = _tracer.trace_evaluation(&intent, &decision);
        assert!(result.is_ok(), "La traçabilité doit réussir dans un cas normal");

        // La décision reste valide même après traçabilité
        assert_eq!(decision.intent_id(), "intent-001");
        assert!(decision.is_accepted());
    }
}
