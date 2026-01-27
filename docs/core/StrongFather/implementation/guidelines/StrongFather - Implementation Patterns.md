# StrongFather — Implementation Patterns

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document présente les patterns d'implémentation recommandés pour StrongFather. Il complète le document [Implementation Overview](./StrongFather%20-%20Implementation%20Overview.md).

**Documents connexes :**
- [StrongFather - Implementation Overview](./StrongFather%20-%20Implementation%20Overview.md)
- [StrongFather - Implementation Prohibitions](./StrongFather%20-%20Implementation%20Prohibitions.md)

**Terminologie :** Voir [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 1. Pattern : Evaluation Surface (Architecture & Flows)

**Concept contractuel :**

La surface d'évaluation est le point d'entrée unique de StrongFather (Architecture & Flows section 3.1).

**Pattern Rust recommandé :**

```rust
// Surface d'évaluation unique conforme à Architecture & Flows
pub struct StrongFather {
    // Composants internes (Architecture & Flows section 3)
    intent_validator: IntentValidator,
    policy_engine: PolicyEngine,
    result_composer: ResultComposer,
    priority_calculator: PriorityCalculator,
    decision_producer: DecisionProducer,
    tracer: Tracer,
}

impl StrongFather {
    // Point d'entrée unique (Core Decision Contract section 2)
    pub fn evaluate_intent(
        &self,
        intent: Intent,
        context: EvaluationContext,
    ) -> Result<Decision, SFError> {
        // 1. Validation structurelle (Intent Model Contract section 6)
        self.intent_validator.validate(&intent)?;
        
        // 2. Application des politiques (Policy Engine Contract)
        let policy_results = self.policy_engine.apply(&intent, &context)?;
        
        // 3. Composition des résultats (Policy Engine Contract section 6)
        let composed_result = self.result_composer.compose(policy_results)?;
        
        // 4. Calcul de priorité (si applicable)
        let priority = self.priority_calculator.calculate(&intent, &composed_result)?;
        
        // 5. Production de décision (Core Decision Contract)
        let decision = self.decision_producer.produce(
            &intent,
            &composed_result,
            priority,
        )?;
        
        // 6. Traçabilité (Audit & Trace Contract)
        self.tracer.trace_evaluation(&intent, &decision)?;
        
        Ok(decision)
    }
}
```

**Règles d'implémentation :**

- **Point d'entrée unique :** Une seule méthode publique pour l'évaluation (Core Decision Contract section 2).

- **Pas d'entrées multiples :** Aucun autre point d'entrée pour l'évaluation.

- **Séparation des responsabilités :** Chaque composant interne a une responsabilité unique (Architecture & Flows).

**Référence contrat :** Architecture & Flows (section 3), Core Decision Contract (section 2)

---

## 2. Pattern : Policy Engine (Policy Engine Contract)

**Concept contractuel :**

Le Policy Engine applique les politiques de manière déterministe, complète, ordonnée, et traçable (Policy Engine Contract section 7).

**Pattern Rust recommandé :**

```rust
// Policy Engine conforme au Policy Engine Contract
pub struct PolicyEngine {
    policies: PolicySet, // Politiques chargées depuis source (Policy Source Contract)
}

impl PolicyEngine {
    pub fn apply(
        &self,
        intent: &Intent,
        context: &EvaluationContext,
    ) -> Result<Vec<PolicyResult>, SFError> {
        // 1. Sélection des politiques applicables (Policy Engine Contract section 5.1)
        let applicable_policies = self.select_applicable_policies(intent, context)?;
        
        // 2. Tri par priorité (Policy Engine Contract section 5.2)
        let ordered_policies = self.order_by_priority(applicable_policies);
        
        // 3. Évaluation de chaque politique (Policy Engine Contract section 5.3)
        let mut results = Vec::new();
        for policy in ordered_policies {
            let result = self.evaluate_policy(policy, intent, context)?;
            results.push(result);
            
            // Résolution de conflits si nécessaire (Policy Engine Contract section 5.4)
            if self.has_conflict(&results) {
                return self.resolve_conflict(&results)?;
            }
        }
        
        Ok(results)
    }
    
    fn select_applicable_policies(
        &self,
        intent: &Intent,
        context: &EvaluationContext,
    ) -> Result<Vec<&Policy>, SFError> {
        // Sélection selon les conditions des politiques
        // INV-POL-3 : Déterminisme
        self.policies
            .iter()
            .filter(|policy| policy.condition.matches(intent, context))
            .collect()
    }
    
    fn evaluate_policy(
        &self,
        policy: &Policy,
        intent: &Intent,
        context: &EvaluationContext,
    ) -> Result<PolicyResult, SFError> {
        // Évaluation déterministe (INV-POL-3)
        // Pas de logique d'exécution (Policy Engine Contract section 2.3)
        match policy.policy_type {
            PolicyType::Permission => self.evaluate_permission(policy, intent, context),
            PolicyType::Constraint => self.evaluate_constraint(policy, intent, context),
            PolicyType::Priority => self.evaluate_priority(policy, intent, context),
            PolicyType::Validation => self.evaluate_validation(policy, intent, context),
            PolicyType::Composite { .. } => self.evaluate_composite(policy, intent, context),
        }
    }
}
```

**Règles d'implémentation :**

- **Déterminisme :** Pour une intention et des politiques données, toujours le même résultat (INV-POL-3).

- **Lecture seule :** Les politiques sont lues depuis la source, jamais modifiées (Policy Source Contract, INV-SRC-4).

- **Pas de logique d'exécution :** L'évaluation ne déclenche jamais d'action (Policy Engine Contract section 2.3).

- **Résolution de conflits :** Appliquer les règles de résolution définies (Policy Engine Contract section 5.4).

**Référence contrat :** Policy Engine Contract (sections 5, 7), Policy Source Contract (INV-SRC-4), Invariants & Guarantees (INV-POL-3)

---

## 3. Pattern : Decision Graph (Decision Graph Specification)

**Concept contractuel :**

Le Decision Graph est un graphe orienté acyclique (DAG) modélisant le processus d'évaluation (Decision Graph Specification section 3).

**Pattern Rust recommandé :**

```rust
// Decision Graph conforme au Decision Graph Specification
pub struct DecisionGraph {
    nodes: Vec<DecisionNode>,
    edges: Vec<DecisionEdge>,
}

#[derive(Debug, Clone)]
pub enum DecisionNode {
    Entry,              // Nœud d'entrée (Decision Graph Specification section 4.1)
    Validation {        // Nœud de validation
        validator: Box<dyn IntentValidator>,
    },
    Policy {            // Nœud de politique
        policy_id: String,
    },
    Composition {       // Nœud de composition
        operator: CompositionOperator,
    },
    Priority {          // Nœud de priorité
        calculator: Box<dyn PriorityCalculator>,
    },
    Decision {          // Nœud de décision
        decision_type: DecisionType,
    },
}

#[derive(Debug, Clone)]
pub enum DecisionEdge {
    Sequence,           // Arête séquentielle (Decision Graph Specification section 5.1)
    Conditional {       // Arête conditionnelle
        condition: Box<dyn Fn(&EvaluationState) -> bool>,
    },
    Aggregation,        // Arête d'agrégation
}

impl DecisionGraph {
    pub fn evaluate(
        &self,
        intent: &Intent,
        context: &EvaluationContext,
    ) -> Result<Decision, SFError> {
        // Parcours du graphe depuis le nœud d'entrée
        let mut state = EvaluationState::new(intent, context);
        let mut current_node = self.find_entry_node()?;
        
        loop {
            match &self.nodes[current_node] {
                DecisionNode::Entry => {
                    current_node = self.follow_edge(current_node, &state)?;
                }
                DecisionNode::Validation { validator } => {
                    validator.validate(&state.intent)?;
                    current_node = self.follow_edge(current_node, &state)?;
                }
                DecisionNode::Policy { policy_id } => {
                    let policy = self.find_policy(policy_id)?;
                    let result = self.evaluate_policy(policy, &state)?;
                    state.add_policy_result(result);
                    current_node = self.follow_edge(current_node, &state)?;
                }
                DecisionNode::Composition { operator } => {
                    state.compose_results(operator)?;
                    current_node = self.follow_edge(current_node, &state)?;
                }
                DecisionNode::Priority { calculator } => {
                    state.priority = calculator.calculate(&state)?;
                    current_node = self.follow_edge(current_node, &state)?;
                }
                DecisionNode::Decision { decision_type } => {
                    return self.produce_decision(&state, decision_type);
                }
            }
            
            // Vérification d'acyclicité (Decision Graph Specification section 3.3)
            if state.visited_nodes.contains(&current_node) {
                return Err(SFError::ConsistencyError {
                    violated_invariant: "INV-GRAPH-1".to_string(),
                    reason: "Cycle détecté dans le graphe de décision".to_string(),
                });
            }
            state.visited_nodes.insert(current_node);
        }
    }
}
```

**Règles d'implémentation :**

- **DAG :** Le graphe DOIT être acyclique (Decision Graph Specification section 3.3).

- **Terminaison garantie :** Le graphe DOIT toujours terminer (Decision Graph Specification section 3.4).

- **Déterminisme :** Pour une intention donnée, le parcours est toujours le même (INV-POL-3).

**Référence contrat :** Decision Graph Specification (sections 3, 4, 5), Invariants & Guarantees (INV-POL-3)

---

## 4. Pattern : Gestion des décisions (Core Decision Contract)

**Concept contractuel :**

Les 4 types de décisions (ACCEPTÉE, REFUSÉE, AMBIGUË, DIFFÉRÉE) doivent être gérés distinctement (Core Decision Contract section 3).

**Pattern Rust recommandé :**

```rust
// Gestion des décisions conforme au Core Decision Contract
impl DecisionProducer {
    pub fn produce(
        &self,
        intent: &Intent,
        policy_results: &ComposedResult,
        priority: Option<Priority>,
    ) -> Result<Decision, SFError> {
        match policy_results {
            ComposedResult::AllSatisfied => {
                // Décision ACCEPTÉE (Core Decision Contract section 3.1)
                Ok(Decision {
                    intent_id: intent.intent_id.clone(),
                    decision_type: DecisionType::Accepted {
                        priority: priority.unwrap_or_default(),
                    },
                    justification: self.build_justification(intent, policy_results),
                    policies_applied: policy_results.policy_ids(),
                    evaluation_context: intent.call_context.clone(),
                    metadata: DecisionMetadata::new(),
                })
            }
            ComposedResult::Violated { policies } => {
                // Décision REFUSÉE (Core Decision Contract section 3.2)
                Ok(Decision {
                    intent_id: intent.intent_id.clone(),
                    decision_type: DecisionType::Refused {
                        reason: self.build_refusal_reason(policies),
                        violated_policies: policies.clone(),
                    },
                    justification: self.build_justification(intent, policy_results),
                    policies_applied: policy_results.policy_ids(),
                    evaluation_context: intent.call_context.clone(),
                    metadata: DecisionMetadata::new(),
                })
            }
            ComposedResult::Ambiguous { missing } => {
                // Décision AMBIGUË (Core Decision Contract section 3.3)
                Ok(Decision {
                    intent_id: intent.intent_id.clone(),
                    decision_type: DecisionType::Ambiguous {
                        missing_information: missing.clone(),
                        clarifications_required: self.build_clarifications(missing),
                    },
                    justification: self.build_justification(intent, policy_results),
                    policies_applied: policy_results.policy_ids(),
                    evaluation_context: intent.call_context.clone(),
                    metadata: DecisionMetadata::new(),
                })
            }
            ComposedResult::Deferred { reason, context_required } => {
                // Décision DIFFÉRÉE (Core Decision Contract section 3.4)
                // INV-DIFF-NOPLAN : Pas de planification
                Ok(Decision {
                    intent_id: intent.intent_id.clone(),
                    decision_type: DecisionType::Deferred {
                        reason: reason.clone(),
                        context_required: context_required.clone(),
                        // Pas de planification, pas d'ordonnancement
                    },
                    justification: self.build_justification(intent, policy_results),
                    policies_applied: policy_results.policy_ids(),
                    evaluation_context: intent.call_context.clone(),
                    metadata: DecisionMetadata::new(),
                })
            }
        }
    }
}
```

**Règles d'implémentation :**

- **4 types distincts :** Gérer explicitement les 4 types, jamais de type générique "résultat".

- **Justification obligatoire :** Toute décision DOIT contenir une justification (G-JUST-1).

- **Pas de planification :** Une décision DIFFÉRÉE n'implique aucune planification (INV-DIFF-NOPLAN).

**Référence contrat :** Core Decision Contract (sections 3, 4), Invariants & Guarantees (INV-DEC-1, INV-DEC-2, INV-DIFF-NOPLAN)

---

## 5. Pattern : Traçabilité avec kernel (Boundary & Isolation Contract)

**Concept contractuel :**

La traçabilité est autorisée via le kernel (Id, Logger, Clock) uniquement pour la traçabilité passive (KERN-AUTH-1, KERN-AUTH-2, KERN-AUTH-3).

**Pattern Rust recommandé :**

```rust
// Traçabilité conforme au Kernel Trace Access Contract
pub struct Tracer {
    id_generator: IdGenerator, // KERN-AUTH-1 : Id pour identification
    logger: Logger,             // KERN-AUTH-2 : Logger pour enregistrement
    clock: Clock,              // KERN-AUTH-3 : Clock pour horodatage uniquement
}

impl Tracer {
    pub fn trace_evaluation(
        &self,
        intent: &Intent,
        decision: &Decision,
    ) -> Result<(), SFError> {
        // KERN-AUTH-1 : Id pour identification de trace
        let trace_id = self.id_generator.generate();
        
        // KERN-AUTH-3 : Clock pour horodatage (après production de décision)
        let timestamp = self.clock.now(); // ✅ Autorisé uniquement pour horodatage
        
        // KERN-AUTH-2 : Logger pour enregistrement passif
        let trace = Trace {
            trace_id,
            intent_id: intent.intent_id.clone(),
            decision_type: decision.decision_type.clone(),
            timestamp, // Horodatage passif uniquement
            policies_applied: decision.policies_applied.clone(),
            justification: decision.justification.clone(),
        };
        
        // Enregistrement passif (pas d'influence sur la décision)
        if let Err(e) = self.logger.log(&trace) {
            // R-TRACE-FAIL-1 : Échec de trace = décision continue
            // La décision a déjà été produite, on ne bloque pas
            eprintln!("Échec de traçabilité: {}", e);
            // La décision continue, pas d'erreur retournée
        }
        
        Ok(())
    }
}

// ❌ INCORRECT : Utilisation de Clock pour logique décisionnelle
impl PolicyEngine {
    pub fn evaluate(&self, intent: &Intent) -> Decision {
        let now = self.clock.now(); // ❌ KERN-INTERD-1 : Clock pour logique décisionnelle
        if now.hour() > 18 {
            return Decision::refused("Trop tard"); // ❌ Logique temporelle interdite
        }
    }
}
```

**Règles d'implémentation :**

- **Id uniquement pour traces :** Id génère des identifiants de trace, jamais utilisés dans la logique décisionnelle (KERN-AUTH-1).

- **Logger uniquement pour enregistrement :** Logger enregistre passivement, jamais pour influencer la décision (KERN-AUTH-2).

- **Clock uniquement pour horodatage :** Clock horodate les traces après production de décision, jamais pour logique temporelle (KERN-AUTH-3, KERN-INTERD-1).

- **Résilience :** Si la trace échoue, la décision continue (R-TRACE-FAIL-1).

**Référence contrat :** Boundary & Isolation Contract (section 4.2.1 — Kernel Trace Access Contract), Audit & Trace Contract (sections 2, 3)

---

## 6. Pattern : Chargement des politiques (Policy Source Contract)

**Pattern recommandé :**

```rust
// Chargement conforme au Policy Source Contract
pub struct PolicySource {
    source_config: SourceConfig,
}

impl PolicySource {
    pub fn load(&self) -> Result<PolicySet, SFError> {
        // R-INIT-1 : Chargement obligatoire avant évaluation
        // R-INIT-2 : Échec bloquant
        match self.source_config.load_policies() {
            Ok(policies) => {
                // R-VAL-1 : Validation préalable (Policy Source Contract section 4.3)
                self.validate_policies(&policies)?;
                Ok(policies)
            }
            Err(e) => {
                // R-INIT-2 : Échec bloquant
                Err(SFError::ResourceError {
                    resource: "policy_source".to_string(),
                    reason: format!("Chargement des politiques échoué: {}", e),
                })
            }
        }
    }
    
    fn validate_policies(&self, policies: &PolicySet) -> Result<(), SFError> {
        // VALID-STRUCT-1 : Identifiant unique
        // VALID-STRUCT-2 : Type valide
        // VALID-STRUCT-3 : Composants obligatoires
        // VALID-COHER-1 : Pas de contradiction directe
        // VALID-COHER-2 : Références valides
        // VALID-CONT-1 : Pas de logique d'exécution
        
        // Validation structurelle
        for policy in policies.iter() {
            self.validate_policy_structure(policy)?;
        }
        
        // Validation de cohérence
        self.validate_coherence(policies)?;
        
        // Validation de contenu
        self.validate_content(policies)?;
        
        Ok(())
    }
}
```

**Règles d'implémentation :**

- **Chargement atomique :** Le chargement est atomique (tout ou rien) (R-LOAD-2, INV-SRC-5).

- **Validation préalable :** Les politiques DOIVENT être validées avant utilisation (R-VAL-1, INV-SRC-3).

- **Source unique :** Une seule source de politiques (INV-SRC-1, INV-POL-SOURCE).

**Référence contrat :** Policy Source Contract (sections 4, 5), Invariants & Guarantees (INV-SRC-*, INV-POL-SOURCE)

---

## 7. Pattern : Flux d'évaluation

**Flux recommandé :**

```rust
impl StrongFather {
    pub fn evaluate_intent(
        &self,
        intent: Intent,
        context: EvaluationContext,
    ) -> Result<Decision, SFError> {
        // 1. Validation structurelle (Intent Model Contract section 6)
        self.intent_validator.validate_structure(&intent)?;
        
        // 2. Transition d'état : SOUMISE → EN_ÉVALUATION (Intent Model Contract section 4)
        let intent_state = IntentState::InEvaluation;
        
        // 3. Application des politiques (Policy Engine Contract)
        let policy_results = self.policy_engine.apply(&intent, &context)?;
        
        // 4. Composition des résultats (Policy Engine Contract section 6)
        let composed_result = self.result_composer.compose(policy_results)?;
        
        // 5. Détection d'ambiguïté (Core Decision Contract section 3.3)
        if let Some(ambiguity) = self.detect_ambiguity(&intent, &composed_result) {
            return Ok(self.decision_producer.produce_ambiguous(&intent, ambiguity));
        }
        
        // 6. Calcul de priorité (si applicable)
        let priority = if composed_result.all_satisfied() {
            Some(self.priority_calculator.calculate(&intent, &composed_result)?)
        } else {
            None
        };
        
        // 7. Production de décision (Core Decision Contract)
        let decision = self.decision_producer.produce(
            &intent,
            &composed_result,
            priority,
        )?;
        
        // 8. Transition d'état : EN_ÉVALUATION → DÉCIDÉE (Intent Model Contract section 4)
        // Note : L'état est conceptuel, pas stocké dans StrongFather
        
        // 9. Traçabilité (Audit & Trace Contract)
        // R-TRACE-FAIL-1 : Échec de trace = décision continue
        let _ = self.tracer.trace_evaluation(&intent, &decision);
        
        Ok(decision)
    }
}
```

**Règles d'implémentation :**

- **Ordre strict :** Respecter l'ordre des étapes (Architecture & Flows section 4).

- **Pas de court-circuit :** Chaque étape DOIT être effectuée (sauf si erreur).

- **Traçabilité résiliente :** L'échec de traçabilité ne bloque pas la décision (R-TRACE-FAIL-1).

**Référence contrat :** Architecture & Flows (section 4), Audit & Trace Contract (R-TRACE-FAIL-1)

---

## 8. Pattern : Gestion des erreurs vs rejets

**Pattern recommandé :**

```rust
// Distinction erreur/rejet conforme au Error & Rejection Model
impl StrongFather {
    pub fn evaluate_intent(
        &self,
        intent: Intent,
        context: EvaluationContext,
    ) -> Result<Decision, SFError> {
        // Erreur = dysfonctionnement interne → Err(SFError)
        // Rejet = résultat normal → Ok(Decision { decision_type: Refused })
        
        // 1. Validation structurelle
        match self.intent_validator.validate_structure(&intent) {
            Ok(()) => {}
            Err(e) => {
                // ✅ CORRECT : Rejet structurel = Décision REFUSÉE
                return Ok(Decision {
                    intent_id: intent.intent_id,
                    decision_type: DecisionType::Refused {
                        reason: RefusalReason::Structural {
                            missing_components: e.missing_components,
                            violated_rules: e.violated_rules,
                        },
                        violated_policies: Vec::new(), // Aucune politique évaluée
                    },
                    justification: Justification::structural_rejection(&e),
                    policies_applied: Vec::new(),
                    evaluation_context: context,
                    metadata: DecisionMetadata::new(),
                });
            }
        }
        
        // 2. Application des politiques
        let policy_results = match self.policy_engine.apply(&intent, &context) {
            Ok(results) => results,
            Err(e) => {
                // Erreur = dysfonctionnement du Policy Engine
                return Err(SFError::ConsistencyError {
                    violated_invariant: "INV-POL-3".to_string(),
                    reason: format!("Erreur dans Policy Engine: {}", e),
                });
            }
        };
        
        // 3. Production de décision
        // Les rejets sont des décisions normales (REFUSÉE, AMBIGUË, DIFFÉRÉE)
        self.decision_producer.produce(&intent, &policy_results, None)
    }
}
```

**Règles d'implémentation :**

- **Erreur = Err(SFError) :** Un dysfonctionnement interne retourne une erreur.

- **Rejet = Ok(Decision) :** Un rejet est une décision normale avec `DecisionType::Refused`.

- **Jamais de mélange :** Ne jamais retourner une erreur pour un rejet, ni un rejet pour une erreur (INV-ERR-1).

**Référence contrat :** Error & Rejection Model (sections 2, 3, 4), Invariants & Guarantees (INV-ERR-1)

---

## 9. Stratégies de test recommandées

### 9.1. Tests de conformité aux contrats

**Pattern recommandé :**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_purity_functional() {
        // Test de pureté fonctionnelle (INV-EXEC-5, INV-BEHAV-3)
        let sf = StrongFather::new(policy_source).unwrap();
        let intent = create_test_intent();
        
        // Première évaluation
        let decision1 = sf.evaluate_intent(intent.clone(), context.clone()).unwrap();
        
        // Deuxième évaluation (même entrée)
        let decision2 = sf.evaluate_intent(intent.clone(), context.clone()).unwrap();
        
        // INV-EXEC-5 : Même entrée = même sortie
        assert_eq!(decision1, decision2);
    }
    
    #[test]
    fn test_zero_trust() {
        // Test de zero-trust (INV-BEHAV-2)
        let sf = StrongFather::new(policy_source).unwrap();
        
        // Intention avec contexte "de confiance"
        let intent = Intent {
            call_context: CallContext {
                caller_identity: "trusted_admin".to_string(),
                // ...
            },
            // ...
        };
        
        // INV-BEHAV-2 : Zero-trust = validation systématique
        let decision = sf.evaluate_intent(intent, context).unwrap();
        
        // La décision doit être basée sur les politiques, pas sur la "confiance"
        assert!(decision.justification.policy_references.len() > 0);
    }
    
    #[test]
    fn test_determinism() {
        // Test de déterminisme (INV-POL-3)
        let sf = StrongFather::new(policy_source).unwrap();
        let intent = create_test_intent();
        
        // Évaluations multiples
        let decisions: Vec<Decision> = (0..10)
            .map(|_| sf.evaluate_intent(intent.clone(), context.clone()).unwrap())
            .collect();
        
        // INV-POL-3 : Toutes les décisions doivent être identiques
        assert!(decisions.iter().all(|d| d == &decisions[0]));
    }
    
    #[test]
    fn test_error_vs_rejection() {
        // Test de distinction erreur/rejet (INV-ERR-1)
        let sf = StrongFather::new(policy_source).unwrap();
        
        // Rejet structurel = Décision REFUSÉE (pas d'erreur)
        let invalid_intent = Intent {
            intent_id: "".to_string(), // ❌ Identifiant vide = invalide
            // ...
        };
        
        let result = sf.evaluate_intent(invalid_intent, context);
        
        // INV-ERR-1 : Rejet ≠ Erreur
        assert!(result.is_ok()); // Rejet = Ok(Decision)
        let decision = result.unwrap();
        assert!(matches!(decision.decision_type, DecisionType::Refused { .. }));
    }
}
```

**Règles d'implémentation :**

- **Tests d'invariants :** Tester tous les invariants pertinents (Invariants & Guarantees).

- **Tests de garanties :** Tester toutes les garanties offertes (Invariants & Guarantees section 4).

- **Tests de violations :** Tester que les patterns interdits sont bien rejetés (Violations & Anti-Patterns).

**Référence contrat :** Invariants & Guarantees (sections 3, 4), Violations & Anti-Patterns

---

**Document créé le :** 2026-01-27  
**Version :** 1.1 (réorganisation)  
**Statut :** POST-FONDATION / NON NORMATIF / INFORMATIF  
**Référence :** StrongFather Contrats FONDATION v1.1 (gelés, non modifiables)  
**Type :** Guide d'implémentation non contractuel
