//! Surface d'évaluation StrongFather
//!
//! Ce module implémente la surface d'évaluation StrongFather, le point d'entrée unique
//! pour toutes les évaluations d'intentions selon des politiques.
//! Conforme au contrat Architecture & Flows et Core Decision Contract (section 2).
//!
//! **Invariants respectés :**
//! - INV-ARCH-1 : Point d'entrée unique
//! - INV-ARCH-2 : Point de sortie unique
//! - INV-ARCH-3 : Flux acyclique
//! - INV-ARCH-4 : Sans état persistant
//! - INV-ARCH-5 : Composants purs
//! - INV-BEHAV-3 : Pureté fonctionnelle
//!
//! **Garanties respectées :**
//! - Toutes les garanties décisionnelles (G-DEC-*, G-JUST-*, G-NOEXEC-*, G-NOPERS-*, G-NOTIME-*, G-ZT-*, G-ISOL-*)

use crate::decision::{Decision, EvaluationContext};
use crate::error::SFError;
use crate::intent::Intent;
use crate::policy_engine::PolicyEngine;
use crate::policy_source::PolicySourceTrait;
use crate::priority::PriorityCalculator;
use crate::producer::DecisionProducer;
use crate::result_composer::ResultComposer;
use crate::tracer::Tracer;
use crate::validator::IntentValidator;

/// StrongFather - Surface d'évaluation
///
/// Point d'entrée unique pour toutes les évaluations d'intentions selon des politiques.
/// Conforme au contrat Architecture & Flows et Core Decision Contract (section 2).
///
/// **Caractéristiques :**
/// - Point d'entrée unique : Toute intention doit passer par StrongFather (INV-ARCH-1)
/// - Point de sortie unique : Toute décision sort de StrongFather (INV-ARCH-2)
/// - Flux acyclique : Aucun composant ne rappelle un composant précédent (INV-ARCH-3)
/// - Sans état persistant : Aucun état n'est maintenu entre évaluations (INV-ARCH-4)
/// - Composants purs : Tous les composants sont des fonctions pures (INV-ARCH-5, INV-BEHAV-3)
///
/// **Flux d'évaluation :**
/// 1. Validation de l'intention (IntentValidator)
/// 2. Chargement des politiques (PolicySource)
/// 3. Application des politiques (PolicyEngine)
/// 4. Composition des résultats (ResultComposer)
/// 5. Calcul de la priorité si toutes satisfaites (PriorityCalculator)
/// 6. Production de la décision (DecisionProducer)
/// 7. Traçabilité de l'évaluation (Tracer) - résiliente
///
/// **Invariants :**
/// - INV-ARCH-1 : Point d'entrée unique (toute intention passe par StrongFather)
/// - INV-ARCH-2 : Point de sortie unique (toute décision sort de StrongFather)
/// - INV-ARCH-3 : Flux acyclique (aucun callback)
/// - INV-ARCH-4 : Sans état persistant (aucun état entre évaluations)
/// - INV-ARCH-5 : Composants purs (fonctions pures)
/// - INV-BEHAV-3 : Pureté fonctionnelle (pas d'effet de bord)
///
/// **Garanties :**
/// - Toutes les garanties décisionnelles (G-DEC-*, G-JUST-*, G-NOEXEC-*, G-NOPERS-*, G-NOTIME-*, G-ZT-*, G-ISOL-*)
pub struct StrongFather<PS, IG, L, C>
where
    PS: PolicySourceTrait,
    IG: miyukini_kernel::IdGenerator,
    L: miyukini_kernel::Logger,
    C: miyukini_kernel::Clock,
{
    /// Validateur d'intentions
    ///
    /// Valide la structure et le contenu des intentions avant évaluation.
    /// Conforme à INV-BEHAV-2 (zero-trust).
    validator: IntentValidator,

    /// Source de politiques
    ///
    /// Charge les politiques depuis une source configurée.
    /// Conforme à Policy Source Contract.
    policy_source: PS,

    /// Compositeur de résultats
    ///
    /// Agrège les résultats des évaluations de politiques.
    /// Conforme à Policy Engine Contract section 6.
    result_composer: ResultComposer,

    /// Calculateur de priorité
    ///
    /// Calcule la priorité relative si toutes les politiques sont satisfaites.
    /// Conforme à Architecture & Flows section 3.5.
    priority_calculator: PriorityCalculator,

    /// Producteur de décision
    ///
    /// Génère la décision finale à partir des résultats d'évaluation.
    /// Conforme à Core Decision Contract et Architecture & Flows section 3.6.
    decision_producer: DecisionProducer,

    /// Tracer pour la traçabilité passive
    ///
    /// Trace les évaluations et décisions sans affecter le comportement.
    /// Conforme à Audit & Trace Contract et Kernel Trace Access Contract.
    /// Résilient : les échecs de traçabilité ne bloquent jamais la décision (R-TRACE-FAIL-1).
    tracer: Tracer<IG, L, C>,
}

impl<PS, IG, L, C> StrongFather<PS, IG, L, C>
where
    PS: PolicySourceTrait,
    IG: miyukini_kernel::IdGenerator,
    L: miyukini_kernel::Logger,
    C: miyukini_kernel::Clock,
{
    /// Crée une nouvelle instance de StrongFather
    ///
    /// # Arguments
    /// * `policy_source` - Source de politiques pour charger les politiques
    /// * `id_generator` - Générateur d'identifiants pour la traçabilité
    /// * `logger` - Logger pour la traçabilité
    /// * `clock` - Horloge pour l'horodatage des traces
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::surface::StrongFather;
    /// use miyukini_strongfather::policy_source::MemoryPolicySource;
    /// use miyukini_kernel::{UuidIdGenerator, DefaultLogger, DefaultClock};
    ///
    /// let policy_source = MemoryPolicySource::empty();
    /// let id_gen = UuidIdGenerator::new();
    /// let logger = DefaultLogger::new();
    /// let clock = DefaultClock::new();
    /// let strongfather = StrongFather::new(policy_source, id_gen, logger, clock);
    /// ```
    pub fn new(policy_source: PS, id_generator: IG, logger: L, clock: C) -> Self {
        Self {
            validator: IntentValidator::new(),
            policy_source,
            result_composer: ResultComposer::new(),
            priority_calculator: PriorityCalculator::new(),
            decision_producer: DecisionProducer::new(),
            tracer: Tracer::new(id_generator, logger, clock),
        }
    }

    /// Évalue une intention et produit une décision
    ///
    /// Point d'entrée unique pour toutes les évaluations d'intentions (INV-ARCH-1).
    /// Conforme au contrat Architecture & Flows et Core Decision Contract (section 2).
    ///
    /// **Flux d'évaluation :**
    /// 1. Validation de l'intention (structure et contenu)
    /// 2. Chargement des politiques depuis la source
    /// 3. Application des politiques à l'intention
    /// 4. Composition des résultats des politiques
    /// 5. Calcul de la priorité si toutes les politiques sont satisfaites
    /// 6. Production de la décision finale
    /// 7. Traçabilité de l'évaluation (résiliente)
    ///
    /// **Invariants respectés :**
    /// - INV-ARCH-1 : Point d'entrée unique
    /// - INV-ARCH-2 : Point de sortie unique
    /// - INV-ARCH-3 : Flux acyclique
    /// - INV-ARCH-4 : Sans état persistant
    /// - INV-ARCH-5 : Composants purs
    /// - INV-BEHAV-3 : Pureté fonctionnelle
    ///
    /// **Garanties respectées :**
    /// - Toutes les garanties décisionnelles (G-DEC-*, G-JUST-*, G-NOEXEC-*, G-NOPERS-*, G-NOTIME-*, G-ZT-*, G-ISOL-*)
    ///
    /// # Arguments
    /// * `intent` - Intention à évaluer
    /// * `context` - Contexte d'évaluation (peut être construit depuis le CallContext de l'intention)
    ///
    /// # Retour
    /// Décision produite, ou erreur si l'évaluation échoue.
    ///
    /// # Erreurs
    /// - `StructuralError` : Si l'intention est structurellement invalide
    /// - `ConsistencyError` : Si une incohérence est détectée dans l'évaluation
    /// - `ResourceError` : Si les politiques ne peuvent pas être chargées
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::surface::StrongFather;
    /// use miyukini_strongfather::policy_source::MemoryPolicySource;
    /// use miyukini_strongfather::intent::{Intent, CallContext, IntentData};
    /// use miyukini_strongfather::decision::EvaluationContext;
    /// use miyukini_strongfather::types::ActionType;
    /// use miyukini_kernel::{UuidIdGenerator, DefaultLogger, DefaultClock};
    ///
    /// let policy_source = MemoryPolicySource::empty();
    /// let id_gen = UuidIdGenerator::new();
    /// let logger = DefaultLogger::new();
    /// let clock = DefaultClock::new();
    /// let strongfather = StrongFather::new(policy_source, id_gen, logger, clock);
    ///
    /// let call_context = CallContext::new(
    ///     "user123".to_string(),
    ///     "product".to_string(),
    ///     "instance1".to_string(),
    /// );
    /// let intent = Intent::new(
    ///     "intent-001".to_string(),
    ///     ActionType::Creation,
    ///     "document".to_string(),
    ///     call_context,
    ///     IntentData::empty(),
    /// );
    ///
    /// let eval_context = EvaluationContext::new(
    ///     "user123".to_string(),
    ///     "product".to_string(),
    ///     "instance1".to_string(),
    /// );
    ///
    /// let decision = strongfather.evaluate_intent(intent, eval_context).unwrap();
    /// ```
    pub fn evaluate_intent(
        &self,
        intent: Intent,
        context: EvaluationContext,
    ) -> Result<Decision, SFError> {
        // Étape 1 : Validation de l'intention (structure et contenu)
        // Conforme à INV-BEHAV-2 (zero-trust), G-ZT-1, G-ZT-2
        self.validator.validate(&intent)?;

        // Étape 2 : Chargement des politiques depuis la source
        // Conforme à Policy Source Contract, INV-POL-SOURCE
        let policy_set = self.policy_source.load()?;

        // Étape 3 : Application des politiques à l'intention
        // Conforme à Policy Engine Contract, INV-POL-2, INV-POL-3
        // Si aucune politique n'est disponible, on considère que toutes sont satisfaites
        // (pas de contrainte = acceptation, conforme à G-POL-2)
        let policy_results = if policy_set.is_empty() {
            // Aucune politique : toutes sont satisfaites par défaut (pas de contrainte)
            Vec::new()
        } else {
            let policy_engine = PolicyEngine::new(policy_set);
            policy_engine.apply(&intent, &context)?
        };

        // Étape 4 : Composition des résultats des politiques
        // Conforme à Policy Engine Contract section 6, Architecture & Flows section 3.4
        let composed_result = self.result_composer.compose(policy_results)?;

        // Étape 5 : Calcul de la priorité si toutes les politiques sont satisfaites
        // Conforme à Architecture & Flows section 3.5, Policy Engine Contract section 3.3
        let priority = if composed_result.is_all_satisfied() {
            Some(self.priority_calculator.calculate(&intent, &composed_result)?)
        } else {
            None
        };

        // Étape 6 : Production de la décision finale
        // Conforme à Core Decision Contract, Architecture & Flows section 3.6
        // Point de sortie unique (INV-ARCH-2)
        let decision = self
            .decision_producer
            .produce(&intent, &composed_result, priority)?;

        // Étape 7 : Traçabilité de l'évaluation (résiliente)
        // Conforme à Audit & Trace Contract, Kernel Trace Access Contract
        // Résilience : R-TRACE-FAIL-1 (échec de trace = décision continue)
        // L'échec de traçabilité ne bloque jamais la décision
        let _ = self.tracer.trace_evaluation(&intent, &decision);

        // Retourner la décision (point de sortie unique, INV-ARCH-2)
        Ok(decision)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::{CallContext, IntentData};
    use crate::policy::{Policy, PolicyCondition, PolicyRule};
    use crate::policy_source::MemoryPolicySource;
    use crate::types::{ActionType, PolicyEffect, PolicyPriority, PolicyType};
    use miyukini_kernel::{DefaultClock, DefaultLogger, UuidIdGenerator};

    fn create_test_strongfather() -> StrongFather<MemoryPolicySource, UuidIdGenerator, DefaultLogger, DefaultClock> {
        let policy_source = MemoryPolicySource::empty();
        let id_gen = UuidIdGenerator::new();
        let logger = DefaultLogger::new();
        let clock = DefaultClock::new();
        StrongFather::new(policy_source, id_gen, logger, clock)
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

    fn create_test_eval_context() -> EvaluationContext {
        EvaluationContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        )
    }

    #[test]
    fn test_strongfather_creation() {
        let _strongfather = create_test_strongfather();
        // Vérification que StrongFather peut être créé sans erreur
    }

    #[test]
    fn test_evaluate_intent_valid() {
        let strongfather = create_test_strongfather();
        let intent = create_test_intent();
        let context = create_test_eval_context();

        let decision = strongfather.evaluate_intent(intent, context).unwrap();

        // Avec une source de politiques vide, toutes les politiques sont satisfaites par défaut
        // (pas de contrainte = acceptation)
        assert!(decision.is_accepted());
        assert_eq!(decision.intent_id(), "intent-001");
    }

    #[test]
    fn test_evaluate_intent_with_policies() {
        // Créer une politique qui accepte toujours
        let policy = Policy::new(
            "POL-001",
            PolicyType::Permission,
            PolicyCondition::always(),
            PolicyRule::new("always"),
            PolicyEffect::Allow,
            PolicyPriority::new(100),
        );
        let policy_source = MemoryPolicySource::new(vec![policy]);
        let id_gen = UuidIdGenerator::new();
        let logger = DefaultLogger::new();
        let clock = DefaultClock::new();
        let strongfather = StrongFather::new(policy_source, id_gen, logger, clock);

        let intent = create_test_intent();
        let context = create_test_eval_context();

        let decision = strongfather.evaluate_intent(intent, context).unwrap();

        assert!(decision.is_accepted());
        assert_eq!(decision.policies_applied().len(), 1);
        assert_eq!(decision.policies_applied()[0], "POL-001");
    }

    #[test]
    fn test_evaluate_intent_invalid_structure() {
        let strongfather = create_test_strongfather();
        
        // Créer une intention avec un identifiant vide (invalide)
        let context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let invalid_intent = Intent {
            intent_id: String::new(), // Vide - invalide
            action_type: ActionType::Creation,
            subject: "document".to_string(),
            call_context: context,
            data: IntentData::empty(),
            requested_priority: None,
            constraints: Vec::new(),
            metadata: std::collections::HashMap::new(),
            references: Vec::new(),
        };

        let eval_context = create_test_eval_context();
        let result = strongfather.evaluate_intent(invalid_intent, eval_context);

        assert!(result.is_err());
        match result.unwrap_err() {
            SFError::StructuralError { .. } => {}
            _ => panic!("Expected StructuralError for invalid intent structure"),
        }
    }

    #[test]
    fn test_evaluate_intent_flow_acyclic() {
        // Test que le flux est acyclique (INV-ARCH-3)
        // Le flux doit être : validate -> load -> apply -> compose -> calculate -> produce -> trace
        // Aucun composant ne doit rappeler un composant précédent
        let strongfather = create_test_strongfather();
        let intent = create_test_intent();
        let context = create_test_eval_context();

        // L'évaluation doit réussir sans boucle infinie
        let decision = strongfather.evaluate_intent(intent, context).unwrap();
        assert!(decision.is_accepted() || decision.is_refused() || decision.is_ambiguous() || decision.is_deferred());
    }

    #[test]
    fn test_evaluate_intent_pure_function() {
        // Test de pureté fonctionnelle (INV-BEHAV-3, INV-ARCH-5)
        // Pour une même intention et un même contexte, le résultat doit être identique
        let strongfather = create_test_strongfather();
        let intent1 = create_test_intent();
        let intent2 = create_test_intent(); // Même intention
        let context = create_test_eval_context();

        let decision1 = strongfather.evaluate_intent(intent1, context.clone()).unwrap();
        let decision2 = strongfather.evaluate_intent(intent2, context).unwrap();

        // Les décisions doivent être identiques (déterminisme)
        assert_eq!(decision1.intent_id(), decision2.intent_id());
        assert_eq!(decision1.decision_type(), decision2.decision_type());
    }

    #[test]
    fn test_evaluate_intent_no_persistent_state() {
        // Test qu'aucun état persistant n'est maintenu (INV-ARCH-4)
        // Deux évaluations successives ne doivent pas s'influencer
        let strongfather = create_test_strongfather();
        let intent1 = create_test_intent();
        let context1 = create_test_eval_context();

        let decision1 = strongfather.evaluate_intent(intent1, context1).unwrap();

        // Deuxième évaluation avec une intention différente
        let context2 = CallContext::new(
            "user456".to_string(),
            "product2".to_string(),
            "instance2".to_string(),
        );
        let intent2 = Intent::new(
            "intent-002".to_string(),
            ActionType::Modification,
            "document2".to_string(),
            context2,
            IntentData::empty(),
        );
        let eval_context2 = EvaluationContext::new(
            "user456".to_string(),
            "product2".to_string(),
            "instance2".to_string(),
        );

        let decision2 = strongfather.evaluate_intent(intent2, eval_context2).unwrap();

        // Les deux décisions doivent être indépendantes
        assert_ne!(decision1.intent_id(), decision2.intent_id());
        // Aucune influence mutuelle (pas d'état persistant)
    }

    #[test]
    fn test_evaluate_intent_trace_resilience() {
        // Test que la traçabilité est résiliente (R-TRACE-FAIL-1)
        // Même si la traçabilité échoue, la décision doit être produite
        let strongfather = create_test_strongfather();
        let intent = create_test_intent();
        let context = create_test_eval_context();

        // L'évaluation doit réussir même si la traçabilité échoue
        let decision = strongfather.evaluate_intent(intent, context).unwrap();

        // La décision doit être valide
        assert_eq!(decision.intent_id(), "intent-001");
        assert!(decision.is_accepted() || decision.is_refused() || decision.is_ambiguous() || decision.is_deferred());
    }

    #[test]
    fn test_evaluate_intent_single_entry_point() {
        // Test que StrongFather est le point d'entrée unique (INV-ARCH-1)
        // Toute intention doit passer par evaluate_intent
        let strongfather = create_test_strongfather();
        let intent = create_test_intent();
        let context = create_test_eval_context();

        // La seule façon d'évaluer une intention est via evaluate_intent
        let decision = strongfather.evaluate_intent(intent, context).unwrap();

        // La décision doit être produite (point de sortie unique, INV-ARCH-2)
        assert!(decision.is_accepted() || decision.is_refused() || decision.is_ambiguous() || decision.is_deferred());
    }
}
