# StrongFather — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter StrongFather correctement, sans violer les contrats FONDATION.

**Objectif pédagogique :** Ce document vise à aider les développeurs à comprendre comment traduire les contrats FONDATION en implémentation Rust, en respectant strictement les invariants, garanties, et interdictions.

**Avertissement :** Ce document ne doit pas être interprété abusivement. Il ne crée aucune nouvelle règle contractuelle et ne modifie aucun contrat existant. Les contrats FONDATION priment toujours sur ce guide.

**Relation avec les contrats FONDATION :** Ce document fait référence aux contrats FONDATION existants mais ne les étend pas, ne les modifie pas, et ne crée aucune nouvelle obligation contractuelle.

---

## 1. Introduction

### 1.1. Objectif

Ce document fournit des lignes directrices pour implémenter StrongFather de manière conforme aux contrats FONDATION. Il explique comment traduire les concepts contractuels en logique d'implémentation Rust sans interprétation abusive.

### 1.2. Nature informative

Ce document est **purement informatif**. Il ne définit pas de nouvelles règles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la compréhension et l'application des contrats FONDATION.

### 1.3. Sources contractuelles

Ce document se base sur tous les contrats FONDATION StrongFather v1.1, avec un focus particulier sur :

- **Documentation Fondatrice** : Invariants INV-SF-*, rôle et positionnement
- **Core Decision Contract** : Types de décisions (ACCEPTÉE, REFUSÉE, AMBIGUË, DIFFÉRÉE), garanties G-DEC-*
- **Intent Model Contract** : Structure des intentions, invariants INV-INT-*
- **Policy Engine Contract** : Types de politiques, application, résolution de conflits
- **Policy Source Contract** : Source unique, cycle de vie, invariants INV-SRC-*
- **Execution Prohibition Contract** : Interdictions absolues INTERD-EXEC-*, invariants INV-EXEC-*
- **Boundary & Isolation Contract** : Frontières, Kernel Trace Access Contract (KERN-AUTH-*)
- **Error & Rejection Model** : Distinction erreur/rejet, catégories
- **Audit & Trace Contract** : Traçabilité, niveaux de trace
- **Invariants & Guarantees** : Catalogue consolidé de tous les invariants
- **[Miyukini Framework - Lois Autonomie Systeme](docs/reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md)** : Conformité aux lois d'autonomie système dans l'implémentation

---

## 2. Principes d'implémentation généraux

### 2.1. Pureté fonctionnelle (INV-EXEC-5, INV-BEHAV-3)

**Principe contractuel :**

L'invariant INV-EXEC-5 et INV-BEHAV-3 établissent que StrongFather se comporte comme une fonction pure : pour une entrée donnée, il produit une sortie sans effet de bord.

**Traduction en logique d'implémentation Rust :**

- **Fonction pure en Rust :** Implémenter StrongFather comme une fonction ou une structure avec des méthodes qui ne modifient jamais l'état externe.

```rust
// ✅ CORRECT : Fonction pure
pub fn evaluate_intent(
    intent: &Intent,
    policies: &PolicySet,
    context: &EvaluationContext,
) -> Result<Decision, SFError> {
    // Évaluation sans effet de bord
    // Aucune mutation d'état externe
    // Aucun appel réseau, DB, ou système de fichiers
}

// ❌ INCORRECT : Mutation d'état externe
pub fn evaluate_intent(&mut self, intent: &Intent) -> Decision {
    self.cache.insert(intent.id(), intent.clone()); // ❌ Cache = effet de bord
    self.counter += 1; // ❌ Mutation d'état = effet de bord
}
```

- **Pas de mutation d'état externe :** Aucune variable globale, aucun singleton mutable, aucun état partagé modifiable.

- **Pas d'I/O :** Aucun appel réseau, aucune écriture fichier, aucune base de données, sauf pour la traçabilité (kernel autorisé selon KERN-AUTH-*).

**Référence contrat :** Execution Prohibition Contract (INTERD-EXEC-*, INTERD-STATE-*), Invariants & Guarantees (INV-EXEC-5, INV-BEHAV-3)

---

### 2.2. Séparation stricte décision/exécution (INV-AUTH-1)

**Principe contractuel :**

L'invariant INV-AUTH-1 établit que StrongFather ne possède jamais d'autorité sur l'exécution. Une décision produite n'entraîne jamais d'exécution automatique.

**Traduction en logique d'implémentation Rust :**

- **Décision = structure de données :** Une décision est une structure de données immuable, jamais une closure ou un callback exécutable.

```rust
// ✅ CORRECT : Décision = structure immuable
#[derive(Debug, Clone)]
pub struct Decision {
    pub intent_id: String,
    pub result: DecisionType,
    pub justification: Justification,
    pub policies_applied: Vec<PolicyId>,
    // Aucun champ exécutable
}

// ❌ INCORRECT : Décision avec callback exécutable
pub struct Decision {
    pub intent_id: String,
    pub execute: Box<dyn Fn() -> ()>, // ❌ Callback = exécution interdite
}
```

- **Pas de callback :** Aucun callback, aucune closure exécutable, aucun mécanisme d'exécution dans la décision.

- **Pas de side-effect :** La production d'une décision ne déclenche jamais d'action automatique.

**Référence contrat :** Execution Prohibition Contract (INTERD-EXEC-4), Documentation Fondatrice (INV-SF-1), Invariants & Guarantees (INV-AUTH-1)

---

### 2.3. Zero-trust (INV-BEHAV-2)

**Principe contractuel :**

L'invariant INV-BEHAV-2 établit que StrongFather ne fait confiance à aucun appelant. Toute intention est évaluée selon les politiques, sans présupposer la validité, l'authenticité, ou la légitimité de l'appelant.

**Traduction en logique d'implémentation Rust :**

- **Validation systématique :** Toute intention DOIT être validée structurellement avant évaluation, même si elle provient d'un adaptateur "de confiance".

```rust
// ✅ CORRECT : Validation systématique
pub fn evaluate_intent(&self, intent: Intent) -> Result<Decision, SFError> {
    // Validation structurelle obligatoire (zero-trust)
    self.validate_intent_structure(&intent)?;
    
    // Validation du contexte (zero-trust)
    self.validate_context(&intent.context)?;
    
    // Évaluation selon politiques (zero-trust)
    self.apply_policies(&intent)
}

// ❌ INCORRECT : Présupposition de validité
pub fn evaluate_intent(&self, intent: Intent) -> Decision {
    // ❌ Pas de validation = violation zero-trust
    if intent.from_trusted_adapter {
        return Decision::accepted(); // ❌ Présupposition interdite
    }
}
```

- **Pas de whitelist :** Aucune liste blanche d'appelants "de confiance" qui bypasserait la validation.

- **Validation du contexte :** Le contexte d'appel DOIT être validé, jamais présupposé valide.

**Référence contrat :** Documentation Fondatrice (INV-SF-5), Invariants & Guarantees (INV-BEHAV-2), Core Decision Contract (G-ZT-*)

---

### 2.4. Zéro effet de bord (G-EXEC-1, INV-EXEC-5)

**Principe contractuel :**

La garantie G-EXEC-1 et l'invariant INV-EXEC-5 établissent qu'aucune opération d'évaluation ne produit d'effet de bord sur le système.

**Traduction en logique d'implémentation Rust :**

- **Pas de mutation :** Aucune mutation d'état système, utilisateur, session, ou configuration.

```rust
// ✅ CORRECT : Pas de mutation
pub fn evaluate_intent(&self, intent: &Intent) -> Decision {
    // self est &self (référence immuable)
    // Aucune mutation possible
}

// ❌ INCORRECT : Mutation d'état
pub fn evaluate_intent(&mut self, intent: &Intent) -> Decision {
    self.evaluation_count += 1; // ❌ Mutation d'état système
    self.last_intent = intent.clone(); // ❌ Mutation d'état
}
```

- **Pas de persistance :** Aucune écriture en base, fichier, cache, ou queue (sauf traçabilité selon Audit & Trace Contract).

- **Pas de communication externe :** Aucun appel réseau, aucune notification, aucun appel à KindMother.

**Référence contrat :** Execution Prohibition Contract (INTERD-PERS-*, INTERD-COM-*), Invariants & Guarantees (INV-EXEC-2, INV-EXEC-3, INV-EXEC-4)

---

## 3. Traduction des concepts StrongFather vers Rust

### 3.1. Intention (Intent Model Contract)

**Concept contractuel :**

Une intention est une demande conceptuelle d'évaluation avec des composants obligatoires (identifiant, type d'action, sujet, contexte) et optionnels (priorité, contraintes, métadonnées).

**Traduction Rust recommandée :**

```rust
// Structure d'intention conforme au Intent Model Contract
#[derive(Debug, Clone)]
pub struct Intent {
    // Composants obligatoires (R-ID-1, R-TYPE-1, R-SUBJ-1)
    pub intent_id: String, // INV-ID-GLOBAL : Unicité globale
    pub action_type: ActionType, // CRÉATION, MODIFICATION, SUPPRESSION, LECTURE, ÉVALUATION
    pub subject: String, // Sujet de l'intention
    pub call_context: CallContext, // Contexte d'appel obligatoire
    
    // Composants optionnels
    pub requested_priority: Option<Priority>,
    pub constraints: Vec<Constraint>,
    pub metadata: HashMap<String, String>,
    pub data: Option<IntentData>, // Données de l'intention
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionType {
    Creation,
    Modification,
    Deletion,
    Read,
    Evaluation,
}

#[derive(Debug, Clone)]
pub struct CallContext {
    pub caller_identity: String, // Obligatoire
    pub origin: String, // Obligatoire
    pub instance: String, // Obligatoire
}

// Cycle de vie (Intent Model Contract section 4)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntentState {
    Submitted,      // SOUMISE
    InEvaluation,   // EN_ÉVALUATION
    Decided,        // DÉCIDÉE
}
```

**Règles d'implémentation :**

- **Immutabilité après soumission :** Une fois soumise, l'intention ne DOIT jamais être modifiée (INV-INT-1, R-ID-2).

- **Validation structurelle :** Valider tous les composants obligatoires avant évaluation (Intent Model Contract section 6).

- **Pas de logique métier :** L'intention ne contient jamais de logique métier spécifique (Execution Prohibition Contract).

**Référence contrat :** Intent Model Contract (sections 2, 3, 4, 6), Invariants & Guarantees (INV-INT-1, INV-ID-GLOBAL)

---

### 3.2. Politique (Policy Engine Contract)

**Concept contractuel :**

Une politique est une règle déclarative avec un identifiant, un type (permission, contrainte, priorité, validation, composite), une condition, une règle, et un effet.

**Traduction Rust recommandée :**

```rust
// Structure de politique conforme au Policy Engine Contract
#[derive(Debug, Clone)]
pub struct Policy {
    // Composants obligatoires (Policy Engine Contract section 4.1)
    pub policy_id: String, // Identifiant unique
    pub policy_type: PolicyType, // Type de politique
    pub condition: PolicyCondition, // Condition d'application
    pub rule: PolicyRule, // Règle à appliquer
    pub effect: PolicyEffect, // Effet de la politique
    pub priority: PolicyPriority, // Priorité de la politique (pour résolution de conflits)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyType {
    Permission,    // Politique de permission
    Constraint,    // Politique de contrainte
    Priority,      // Politique de priorité
    Validation,    // Politique de validation
    Composite {    // Politique composite
        operator: LogicalOperator, // ET, OU, NON
        policies: Vec<String>, // Références aux politiques composantes
    },
}

#[derive(Debug, Clone)]
pub enum PolicyCondition {
    Always, // Toujours applicable
    When { condition: Box<dyn Fn(&Intent, &EvaluationContext) -> bool> }, // Condition dynamique
}

#[derive(Debug, Clone)]
pub enum PolicyEffect {
    Allow,      // Autorise
    Deny,       // Interdit
    Require,    // Requiert
    Validate,   // Valide
    Prioritize { level: u8 }, // Priorise
}
```

**Règles d'implémentation :**

- **Lecture seule :** Les politiques sont chargées depuis la source et ne sont jamais modifiées (Policy Source Contract, INV-SRC-4).

- **Immutabilité pendant évaluation :** Les politiques ne changent jamais pendant une évaluation (INV-POL-2).

- **Pas de logique d'exécution :** Les politiques ne contiennent jamais de logique d'exécution (Policy Engine Contract section 2.3).

- **Pas de logique métier :** Les politiques ne contiennent jamais de logique métier spécifique (Policy Engine Contract section 2.3).

**Référence contrat :** Policy Engine Contract (sections 2, 3, 4), Policy Source Contract (INV-SRC-4, INV-POL-SOURCE), Invariants & Guarantees (INV-POL-1, INV-POL-2)

---

### 3.3. Décision (Core Decision Contract)

**Concept contractuel :**

Une décision est le résultat produit après évaluation, avec 4 types autorisés : ACCEPTÉE, REFUSÉE, AMBIGUË, DIFFÉRÉE.

**Traduction Rust recommandée :**

```rust
// Structure de décision conforme au Core Decision Contract
#[derive(Debug, Clone)]
pub struct Decision {
    // Composants obligatoires (Core Decision Contract section 4)
    pub intent_id: String, // Identifiant de l'intention évaluée
    pub decision_type: DecisionType, // Type de décision
    pub justification: Justification, // Justification obligatoire (G-JUST-1)
    pub policies_applied: Vec<PolicyId>, // Politiques appliquées (INV-TRACE-3)
    pub evaluation_context: EvaluationContext, // Contexte d'évaluation
    pub metadata: DecisionMetadata, // Métadonnées de traçabilité
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionType {
    Accepted {
        priority: Priority, // Priorité établie
    },
    Refused {
        reason: RefusalReason, // Raison explicite du refus
        violated_policies: Vec<PolicyId>, // Politiques violées
    },
    Ambiguous {
        missing_information: Vec<String>, // Informations manquantes
        clarifications_required: Vec<Clarification>, // Clarifications requises
    },
    Deferred {
        reason: DeferralReason, // Raison du différé
        context_required: Vec<String>, // Contexte futur requis
    },
}

#[derive(Debug, Clone)]
pub struct Justification {
    pub explanation: String, // Explication conceptuelle
    pub policy_references: Vec<PolicyId>, // Références aux politiques
    pub reasoning_steps: Vec<ReasoningStep>, // Étapes de raisonnement
}
```

**Règles d'implémentation :**

- **Unicité :** Pour chaque intention, exactement une décision est produite (INV-DEC-3).

- **Justification obligatoire :** Toute décision DOIT contenir une justification (G-JUST-1).

- **Non-exécutable :** Une décision n'est jamais exécutable directement (G-NOEXEC-1, INV-EXEC-1).

- **Pas de logique temporelle :** Une décision DIFFÉRÉE n'implique aucune planification (INV-DIFF-NOPLAN).

**Référence contrat :** Core Decision Contract (sections 2, 3, 4), Invariants & Guarantees (INV-DEC-1, INV-DEC-2, INV-DEC-3, INV-DIFF-NOPLAN)

---

### 3.4. Erreur vs Rejet (Error & Rejection Model)

**Concept contractuel :**

Une erreur est un dysfonctionnement interne qui empêche l'évaluation. Un rejet est un résultat normal d'évaluation (intention invalide).

**Traduction Rust recommandée :**

```rust
// Distinction erreur/rejet conforme au Error & Rejection Model
#[derive(Debug, Clone)]
pub enum SFError {
    // Erreurs (dysfonctionnements internes)
    StructuralError {
        reason: String,
        location: String,
    },
    ConsistencyError {
        violated_invariant: String,
        reason: String,
    },
    ResourceError {
        resource: String,
        reason: String,
    },
    
    // Note : Les rejets sont des Décisions (DecisionType::Refused, etc.)
    // Pas des erreurs
}

// ❌ INCORRECT : Mélanger erreur et rejet
pub enum SFError {
    Rejection { reason: String }, // ❌ Rejet ≠ Erreur
}

// ✅ CORRECT : Rejet = Décision
pub fn evaluate_intent(&self, intent: &Intent) -> Result<Decision, SFError> {
    // Erreur = dysfonctionnement → Err(SFError)
    // Rejet = résultat normal → Ok(Decision { decision_type: DecisionType::Refused })
}
```

**Règles d'implémentation :**

- **Distinction stricte :** Une erreur retourne `Err(SFError)`, un rejet retourne `Ok(Decision)` avec `DecisionType::Refused` (INV-ERR-1).

- **Pas de mélange :** Ne jamais retourner une erreur pour un rejet, ni un rejet pour une erreur.

- **Traçabilité différente :** Les erreurs sont tracées dans les logs d'erreur, les rejets dans les décisions (Audit & Trace Contract).

**Référence contrat :** Error & Rejection Model (sections 2, 3, 4), Invariants & Guarantees (INV-ERR-1)

---

## 4. Patterns d'implémentation recommandés

### 4.1. Pattern : Evaluation Surface (Architecture & Flows)

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

### 4.2. Pattern : Policy Engine (Policy Engine Contract)

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

### 4.3. Pattern : Decision Graph (Decision Graph Specification)

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

### 4.4. Pattern : Gestion des décisions (Core Decision Contract)

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

### 4.5. Pattern : Traçabilité avec kernel (Boundary & Isolation Contract, Audit & Trace Contract)

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
            // On peut logger l'échec de traçabilité mais la décision reste valide
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

## 5. Patterns strictement interdits

### 5.1. ❌ Cache décisionnel (INTERD-PERS-3, INV-EXEC-3)

**Violation contractuelle :**

Un cache décisionnel viole INTERD-PERS-3 (écriture en cache) et INV-EXEC-3 (aucune persistance).

**Exemple d'implémentation invalide :**

```rust
// ❌ INCORRECT : Cache décisionnel
pub struct StrongFather {
    decision_cache: HashMap<String, Decision>, // ❌ Cache = persistance interdite
}

impl StrongFather {
    pub fn evaluate_intent(&mut self, intent: &Intent) -> Decision {
        // ❌ Vérification de cache = persistance opérationnelle
        if let Some(cached) = self.decision_cache.get(&intent.intent_id) {
            return cached.clone(); // ❌ Réutilisation de décision = effet de bord
        }
        
        let decision = self.evaluate(intent);
        self.decision_cache.insert(intent.intent_id.clone(), decision.clone()); // ❌ Écriture en cache
        decision
    }
}
```

**Pourquoi c'est interdit :**

- **Persistance opérationnelle :** Un cache est une forme de persistance qui affecte le comportement (INTERD-PERS-3).

- **Effet de bord :** Le cache modifie l'état entre les évaluations (INV-EXEC-2, INV-EXEC-3).

- **Non-déterminisme :** Un cache peut produire des résultats différents selon l'historique (INV-POL-3).

**Référence contrat :** Execution Prohibition Contract (INTERD-PERS-3), Invariants & Guarantees (INV-EXEC-2, INV-EXEC-3, INV-POL-3)

---

### 5.2. ❌ Ordonnancement ou planification (INTERD-TIME-1, INTERD-TIME-2, INV-DIFF-NOPLAN)

**Violation contractuelle :**

L'ordonnancement viole INTERD-TIME-1, la planification viole INTERD-TIME-2, et une décision DIFFÉRÉE ne doit pas impliquer de planification (INV-DIFF-NOPLAN).

**Exemple d'implémentation invalide :**

```rust
// ❌ INCORRECT : Ordonnancement
impl StrongFather {
    pub fn evaluate_intent(&self, intent: &Intent) -> Decision {
        let now = self.clock.now(); // ❌ KERN-INTERD-1 : Clock pour logique décisionnelle
        
        // ❌ INTERD-TIME-1 : Ordonnancement
        if now.hour() < 9 {
            return Decision::deferred("Trop tôt, réessayer à 9h");
        }
        
        // ❌ INTERD-TIME-2 : Planification
        if intent.requires_future_context {
            self.schedule_revaluation(intent, now + Duration::hours(1)); // ❌ Planification interdite
            return Decision::deferred("Contexte futur requis");
        }
    }
    
    // ❌ INCORRECT : Planification pour décision différée
    fn schedule_revaluation(&self, intent: &Intent, when: DateTime) {
        // ❌ INV-DIFF-NOPLAN : Pas de planification
        self.scheduler.schedule(intent, when); // ❌ Ordonnancement interdit
    }
}
```

**Pourquoi c'est interdit :**

- **Logique temporelle technique :** L'ordonnancement utilise le temps technique pour influencer les décisions (INTERD-TIME-1, KERN-INTERD-1).

- **Planification interdite :** StrongFather ne planifie jamais d'exécutions futures (INTERD-TIME-2, INV-DIFF-NOPLAN).

- **Responsabilité adaptateur :** Seul l'adaptateur décide quand re-soumettre une intention différée (INV-DIFF-NOPLAN).

**Référence contrat :** Execution Prohibition Contract (INTERD-TIME-1, INTERD-TIME-2), Boundary & Isolation Contract (KERN-INTERD-1), Invariants & Guarantees (INV-DIFF-NOPLAN)

---

### 5.3. ❌ Appel à KindMother (INTERD-KM-1, INTERD-KM-2, INTERD-KM-3)

**Violation contractuelle :**

Tout appel à KindMother viole INTERD-KM-1, INTERD-KM-2, INTERD-KM-3.

**Exemple d'implémentation invalide :**

```rust
// ❌ INCORRECT : Appel à KindMother
impl StrongFather {
    pub fn evaluate_intent(&self, intent: &Intent) -> Decision {
        // ❌ INTERD-KM-2 : Lecture de données gérées par KindMother
        let existing_entity = self.kindmother.read_entity(&intent.subject)?;
        
        // ❌ INTERD-KM-1 : Appel à KindMother
        if existing_entity.is_some() {
            return Decision::refused("Entité existe déjà");
        }
        
        // ❌ INTERD-KM-3 : Demande de persistance
        if intent.action_type == ActionType::Creation {
            self.kindmother.persist(intent)?; // ❌ Persistance interdite
        }
    }
}
```

**Pourquoi c'est interdit :**

- **Indépendance absolue :** StrongFather et KindMother sont totalement indépendants (INTERD-KM-4).

- **Séparation des responsabilités :** StrongFather décide, KindMother persiste. Aucune communication directe (Boundary & Isolation Contract section 4.1).

- **Isolation garantie :** L'isolation garantit la pureté fonctionnelle de StrongFather (INV-BOUND-5).

**Référence contrat :** Boundary & Isolation Contract (section 4.1, INTERD-KM-*), Invariants & Guarantees (INV-BOUND-2, INV-BOUND-5)

---

### 5.4. ❌ Logique métier spécifique (Execution Prohibition Contract)

**Violation contractuelle :**

La logique métier spécifique viole l'interdiction d'exécution et la séparation des responsabilités.

**Exemple d'implémentation invalide :**

```rust
// ❌ INCORRECT : Logique métier spécifique
impl PolicyEngine {
    pub fn evaluate_permission(&self, policy: &Policy, intent: &Intent) -> PolicyResult {
        // ❌ Logique métier spécifique (exemple : e-commerce)
        if intent.subject.starts_with("order_") {
            let order = self.parse_order(&intent.data)?; // ❌ Parsing métier
            if order.total > 1000.0 {
                return PolicyResult::denied("Montant trop élevé"); // ❌ Règle métier
            }
        }
        
        // ❌ Validation technique
        if !self.validate_email(&intent.data.email) { // ❌ Validation technique interdite
            return PolicyResult::denied("Email invalide");
        }
    }
}
```

**Pourquoi c'est interdit :**

- **Séparation des responsabilités :** StrongFather évalue selon des politiques générales, pas des règles métier spécifiques (Execution Prohibition Contract section 3.5).

- **Réutilisabilité :** Les politiques doivent être générales et réutilisables (Policy Engine Contract section 2.3).

- **Pas de validation technique :** StrongFather ne valide jamais la structure technique des données (Core Decision Contract section 2.3).

**Référence contrat :** Execution Prohibition Contract (section 3.5), Policy Engine Contract (section 2.3), Core Decision Contract (section 2.3)

---

### 5.5. ❌ Mutation d'état système (INTERD-STATE-*, INV-EXEC-2)

**Violation contractuelle :**

Toute mutation d'état système viole INTERD-STATE-* et INV-EXEC-2.

**Exemple d'implémentation invalide :**

```rust
// ❌ INCORRECT : Mutation d'état
pub struct StrongFather {
    evaluation_count: usize,        // ❌ État système
    last_evaluated_intent: Option<Intent>, // ❌ État système
    user_preferences: HashMap<String, String>, // ❌ État utilisateur
}

impl StrongFather {
    pub fn evaluate_intent(&mut self, intent: &Intent) -> Decision {
        self.evaluation_count += 1; // ❌ INTERD-STATE-1 : État système
        self.last_evaluated_intent = Some(intent.clone()); // ❌ INTERD-STATE-1
        
        // ❌ INTERD-STATE-2 : État utilisateur
        self.user_preferences.insert("last_action".to_string(), intent.action_type.to_string());
        
        self.evaluate(intent)
    }
}
```

**Pourquoi c'est interdit :**

- **Pureté fonctionnelle :** StrongFather ne modifie jamais d'état (INV-EXEC-2, INV-BEHAV-1).

- **Isolation :** L'isolation garantit qu'aucun état externe n'est modifié (INV-BOUND-5).

- **Réversibilité :** Les évaluations doivent être réversibles conceptuellement (G-EXEC-3).

**Référence contrat :** Execution Prohibition Contract (INTERD-STATE-*, INV-EXEC-2), Invariants & Guarantees (INV-BEHAV-1, INV-BOUND-5)

---

## 6. Implémentation du moteur de décision

### 6.1. Structure générale

**Architecture recommandée :**

```rust
// Structure conforme à Architecture & Flows
pub struct StrongFather {
    // Composants internes (Architecture & Flows section 3)
    intent_validator: IntentValidator,
    policy_engine: PolicyEngine,
    result_composer: ResultComposer,
    priority_calculator: PriorityCalculator,
    decision_producer: DecisionProducer,
    tracer: Tracer,
    
    // Source de politiques (Policy Source Contract)
    policy_source: PolicySource,
}

impl StrongFather {
    pub fn new(policy_source: PolicySource) -> Result<Self, SFError> {
        // Chargement initial des politiques (Policy Source Contract section 5.1)
        let policies = policy_source.load()?;
        
        Ok(Self {
            intent_validator: IntentValidator::new(),
            policy_engine: PolicyEngine::new(policies),
            result_composer: ResultComposer::new(),
            priority_calculator: PriorityCalculator::new(),
            decision_producer: DecisionProducer::new(),
            tracer: Tracer::new(),
            policy_source,
        })
    }
}
```

**Règles d'implémentation :**

- **Pas de logique métier :** Aucune logique métier spécifique dans le moteur (Execution Prohibition Contract).

- **Séparation des composants :** Chaque composant a une responsabilité unique (Architecture & Flows).

- **Source de politiques :** Les politiques proviennent d'une source unique configurée (Policy Source Contract, INV-POL-SOURCE).

**Référence contrat :** Architecture & Flows (section 3), Policy Source Contract (INV-POL-SOURCE)

---

### 6.2. Flux d'évaluation

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

## 7. Implémentation du Policy Engine

### 7.1. Chargement des politiques (Policy Source Contract)

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

### 7.2. Application des politiques (Policy Engine Contract)

**Pattern recommandé :**

```rust
// Application conforme au Policy Engine Contract
impl PolicyEngine {
    pub fn apply(
        &self,
        intent: &Intent,
        context: &EvaluationContext,
    ) -> Result<Vec<PolicyResult>, SFError> {
        // 1. Sélection des politiques applicables (Policy Engine Contract section 5.1)
        let applicable = self.select_applicable(intent, context)?;
        
        // 2. Tri par priorité (Policy Engine Contract section 5.2)
        let ordered = self.order_by_priority(applicable);
        
        // 3. Évaluation de chaque politique (Policy Engine Contract section 5.3)
        let mut results = Vec::new();
        for policy in ordered {
            let result = self.evaluate_single_policy(policy, intent, context)?;
            results.push(result);
            
            // 4. Résolution de conflits si nécessaire (Policy Engine Contract section 5.4)
            if self.has_conflict(&results) {
                return self.resolve_conflict(&results);
            }
        }
        
        Ok(results)
    }
    
    fn evaluate_single_policy(
        &self,
        policy: &Policy,
        intent: &Intent,
        context: &EvaluationContext,
    ) -> Result<PolicyResult, SFError> {
        // INV-POL-3 : Déterminisme
        // Pour une intention et une politique données, toujours le même résultat
        
        match &policy.policy_type {
            PolicyType::Permission => {
                // Évaluation de permission (Policy Engine Contract section 3.1)
                self.evaluate_permission(policy, intent, context)
            }
            PolicyType::Constraint => {
                // Évaluation de contrainte (Policy Engine Contract section 3.2)
                self.evaluate_constraint(policy, intent, context)
            }
            PolicyType::Priority => {
                // Évaluation de priorité (Policy Engine Contract section 3.3)
                self.evaluate_priority(policy, intent, context)
            }
            PolicyType::Validation => {
                // Évaluation de validation (Policy Engine Contract section 3.4)
                self.evaluate_validation(policy, intent, context)
            }
            PolicyType::Composite { operator, policies } => {
                // Évaluation composite (Policy Engine Contract section 3.5)
                self.evaluate_composite(policy, operator, policies, intent, context)
            }
        }
    }
}
```

**Règles d'implémentation :**

- **Déterminisme :** Pour une intention et des politiques données, toujours le même résultat (INV-POL-3).

- **Lecture seule :** Les politiques sont lues, jamais modifiées (INV-SRC-4).

- **Pas de logique d'exécution :** L'évaluation ne déclenche jamais d'action (Policy Engine Contract section 2.3).

- **Résolution de conflits :** Appliquer les règles de résolution (Policy Engine Contract section 5.4).

**Référence contrat :** Policy Engine Contract (sections 3, 5, 7), Invariants & Guarantees (INV-POL-3)

---

## 8. Implémentation du Decision Graph

### 8.1. Structure du graphe (Decision Graph Specification)

**Pattern recommandé :**

```rust
// Decision Graph conforme au Decision Graph Specification
pub struct DecisionGraph {
    nodes: Vec<DecisionNode>,
    edges: Vec<DecisionEdge>,
}

impl DecisionGraph {
    pub fn new() -> Self {
        // Construction du graphe selon Decision Graph Specification
        // Le graphe est construit à partir de la configuration des politiques
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    
    pub fn build_from_policies(&mut self, policies: &PolicySet) -> Result<(), SFError> {
        // Construction du graphe selon Decision Graph Specification section 4
        
        // 1. Nœud d'entrée (Decision Graph Specification section 4.1)
        let entry_node = DecisionNode::Entry;
        self.nodes.push(entry_node);
        
        // 2. Nœuds de validation (Decision Graph Specification section 4.2)
        let validation_node = DecisionNode::Validation {
            validator: Box::new(IntentValidator::new()),
        };
        self.nodes.push(validation_node);
        self.edges.push(DecisionEdge::Sequence {
            from: 0, // Entry
            to: 1,   // Validation
        });
        
        // 3. Nœuds de politique (Decision Graph Specification section 4.3)
        for (idx, policy) in policies.iter().enumerate() {
            let policy_node = DecisionNode::Policy {
                policy_id: policy.policy_id.clone(),
            };
            self.nodes.push(policy_node);
            
            // Arêtes conditionnelles selon les politiques
            self.edges.push(DecisionEdge::Conditional {
                from: 1 + idx,
                to: 2 + idx,
                condition: Box::new(move |state| {
                    policy.condition.matches(&state.intent, &state.context)
                }),
            });
        }
        
        // 4. Nœud de composition (Decision Graph Specification section 4.4)
        let composition_node = DecisionNode::Composition {
            operator: CompositionOperator::And,
        };
        self.nodes.push(composition_node);
        
        // 5. Nœud de priorité (Decision Graph Specification section 4.5)
        let priority_node = DecisionNode::Priority {
            calculator: Box::new(PriorityCalculator::new()),
        };
        self.nodes.push(priority_node);
        
        // 6. Nœuds de décision (Decision Graph Specification section 4.6)
        let accepted_node = DecisionNode::Decision {
            decision_type: DecisionType::Accepted { priority: None },
        };
        self.nodes.push(accepted_node);
        
        // Vérification d'acyclicité (Decision Graph Specification section 3.3)
        self.verify_acyclic()?;
        
        Ok(())
    }
    
    fn verify_acyclic(&self) -> Result<(), SFError> {
        // Vérification que le graphe est acyclique (Decision Graph Specification section 3.3)
        // Utiliser un algorithme de détection de cycles (DFS)
        // Si cycle détecté → erreur de cohérence
        Ok(())
    }
}
```

**Règles d'implémentation :**

- **DAG :** Le graphe DOIT être acyclique (Decision Graph Specification section 3.3).

- **Terminaison garantie :** Le graphe DOIT toujours terminer (Decision Graph Specification section 3.4).

- **Déterminisme :** Le parcours est toujours le même pour une intention donnée (INV-POL-3).

**Référence contrat :** Decision Graph Specification (sections 3, 4), Invariants & Guarantees (INV-POL-3)

---

## 9. Gestion des erreurs vs rejets

### 9.1. Distinction stricte (Error & Rejection Model)

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
                // ❌ INCORRECT : Retourner une erreur pour un rejet structurel
                // return Err(e); // ❌ Rejet structurel ≠ Erreur
                
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
        }
        
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

## 10. Stratégies de test recommandées

### 10.1. Tests de conformité aux contrats

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
        // Même un "admin de confiance" doit être validé
        let decision = sf.evaluate_intent(intent, context).unwrap();
        
        // La décision doit être basée sur les politiques, pas sur la "confiance"
        assert!(decision.justification.policy_references.len() > 0);
    }
    
    #[test]
    fn test_no_execution() {
        // Test d'interdiction d'exécution (INV-EXEC-1)
        let sf = StrongFather::new(policy_source).unwrap();
        let intent = create_test_intent();
        
        let decision = sf.evaluate_intent(intent, context).unwrap();
        
        // INV-EXEC-1 : Aucune exécution
        // Vérifier qu'aucune action n'a été déclenchée
        // (dépend de l'infrastructure de test)
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

## 11. Pièges classiques et erreurs fréquentes

### 11.1. Piège : Cache "pour performance"

**Erreur fréquente :**

Implémenter un cache pour améliorer les performances, violant ainsi INTERD-PERS-3 et INV-EXEC-3.

**Pourquoi c'est un piège :**

- **Violation contractuelle :** Un cache est une forme de persistance opérationnelle (INTERD-PERS-3).

- **Non-déterminisme :** Un cache peut produire des résultats différents selon l'historique (INV-POL-3).

- **Effet de bord :** Un cache modifie l'état entre les évaluations (INV-EXEC-2).

**Solution :**

- **Pas de cache :** Accepter que chaque évaluation soit indépendante.

- **Optimisation autorisée :** Optimiser l'algorithme d'évaluation, pas ajouter un cache.

**Référence contrat :** Execution Prohibition Contract (INTERD-PERS-3), Invariants & Guarantees (INV-EXEC-2, INV-EXEC-3, INV-POL-3)

---

### 11.2. Piège : Utilisation de Clock pour logique décisionnelle

**Erreur fréquente :**

Utiliser `Clock` pour déterminer si une intention est valide selon l'heure, violant KERN-INTERD-1.

**Pourquoi c'est un piège :**

- **Violation contractuelle :** Clock est autorisé uniquement pour l'horodatage de traces (KERN-AUTH-3, KERN-INTERD-1).

- **Logique temporelle interdite :** StrongFather ne gère jamais le temps technique (INTERD-TIME-*).

- **Confusion conceptuelle :** Le temps conceptuel (saison, période) ≠ temps technique (horodatage).

**Solution :**

- **Clock uniquement pour traces :** Utiliser Clock uniquement après production de décision pour horodater la trace.

- **Pas de logique temporelle :** Ne jamais utiliser Clock pour influencer une évaluation.

**Référence contrat :** Boundary & Isolation Contract (KERN-AUTH-3, KERN-INTERD-1), Execution Prohibition Contract (INTERD-TIME-*)

---

### 11.3. Piège : Mélanger erreur et rejet

**Erreur fréquente :**

Retourner une erreur (`Err(SFError)`) pour un rejet structurel, violant INV-ERR-1.

**Pourquoi c'est un piège :**

- **Violation contractuelle :** Un rejet est un résultat normal, pas un dysfonctionnement (INV-ERR-1).

- **Confusion sémantique :** Erreur = interne, Rejet = externe (Error & Rejection Model section 2).

- **Traçabilité incorrecte :** Les erreurs et rejets sont tracés différemment (Audit & Trace Contract).

**Solution :**

- **Rejet = Decision :** Un rejet structurel produit une `Decision` avec `DecisionType::Refused`.

- **Erreur = SFError :** Un dysfonctionnement interne retourne `Err(SFError)`.

**Référence contrat :** Error & Rejection Model (section 2), Invariants & Guarantees (INV-ERR-1)

---

### 11.4. Piège : Planification pour décision DIFFÉRÉE

**Erreur fréquente :**

Implémenter un scheduler pour les décisions DIFFÉRÉES, violant INV-DIFF-NOPLAN et INTERD-TIME-2.

**Pourquoi c'est un piège :**

- **Violation contractuelle :** Une décision DIFFÉRÉE n'implique aucune planification (INV-DIFF-NOPLAN).

- **Ordonnancement interdit :** StrongFather ne planifie jamais d'exécutions futures (INTERD-TIME-2).

- **Responsabilité adaptateur :** Seul l'adaptateur décide quand re-soumettre (INV-DIFF-NOPLAN).

**Solution :**

- **Pas de scheduler :** Ne jamais implémenter de scheduler ou de planification.

- **Décision pure :** Une décision DIFFÉRÉE indique uniquement que le contexte futur est requis.

**Référence contrat :** Invariants & Guarantees (INV-DIFF-NOPLAN), Execution Prohibition Contract (INTERD-TIME-2)

---

### 11.5. Piège : Logique métier dans les politiques

**Erreur fréquente :**

Implémenter de la logique métier spécifique dans les politiques, violant Policy Engine Contract section 2.3.

**Pourquoi c'est un piège :**

- **Violation contractuelle :** Les politiques ne contiennent jamais de logique métier spécifique (Policy Engine Contract section 2.3).

- **Réutilisabilité :** Les politiques doivent être générales et réutilisables.

- **Séparation des responsabilités :** StrongFather évalue selon des politiques générales, pas des règles métier.

**Solution :**

- **Politiques générales :** Les politiques expriment des règles générales (permission, contrainte, priorité).

- **Pas de parsing métier :** Ne jamais parser des structures métier spécifiques dans les politiques.

**Référence contrat :** Policy Engine Contract (section 2.3), Execution Prohibition Contract (section 3.5)

---

## 12. Implémentations INVALIDES (à ne jamais faire)

### 12.1. ❌ StrongFather avec cache

```rust
// ❌ IMPLÉMENTATION INVALIDE : Cache décisionnel
pub struct StrongFather {
    cache: HashMap<String, Decision>, // ❌ INTERD-PERS-3, INV-EXEC-3
}

impl StrongFather {
    pub fn evaluate_intent(&mut self, intent: &Intent) -> Decision {
        if let Some(cached) = self.cache.get(&intent.intent_id) {
            return cached.clone(); // ❌ Réutilisation = effet de bord
        }
        let decision = self.evaluate(intent);
        self.cache.insert(intent.intent_id.clone(), decision.clone()); // ❌ Écriture en cache
        decision
    }
}
```

**Violations :**
- INTERD-PERS-3 : Écriture en cache
- INV-EXEC-3 : Aucune persistance
- INV-EXEC-2 : Modification d'état
- INV-POL-3 : Non-déterminisme potentiel

---

### 12.2. ❌ StrongFather avec appel à KindMother

```rust
// ❌ IMPLÉMENTATION INVALIDE : Appel à KindMother
pub struct StrongFather {
    kindmother: KindMotherClient, // ❌ INTERD-KM-1, INTERD-KM-4
}

impl StrongFather {
    pub fn evaluate_intent(&self, intent: &Intent) -> Decision {
        // ❌ INTERD-KM-2 : Lecture de données gérées par KindMother
        let existing = self.kindmother.read_entity(&intent.subject)?;
        
        if existing.is_some() {
            return Decision::refused("Existe déjà"); // ❌ Dépendance à KindMother
        }
        
        // ❌ INTERD-KM-3 : Demande de persistance
        if intent.action_type == ActionType::Creation {
            self.kindmother.persist(intent)?; // ❌ Persistance interdite
        }
        
        Decision::accepted()
    }
}
```

**Violations :**
- INTERD-KM-1 : Appel à KindMother
- INTERD-KM-2 : Lecture de données KindMother
- INTERD-KM-3 : Demande de persistance
- INTERD-KM-4 : Connaissance de KindMother
- INV-BOUND-2 : Indépendance KindMother

---

### 12.3. ❌ StrongFather avec ordonnancement

```rust
// ❌ IMPLÉMENTATION INVALIDE : Ordonnancement
pub struct StrongFather {
    scheduler: Scheduler, // ❌ INTERD-TIME-1, INTERD-TIME-2
    clock: Clock,
}

impl StrongFather {
    pub fn evaluate_intent(&self, intent: &Intent) -> Decision {
        let now = self.clock.now(); // ❌ KERN-INTERD-1 : Clock pour logique décisionnelle
        
        // ❌ INTERD-TIME-1 : Ordonnancement
        if now.hour() < 9 {
            return Decision::deferred("Trop tôt");
        }
        
        // ❌ INTERD-TIME-2 : Planification
        if intent.requires_future_context {
            self.scheduler.schedule(intent, now + Duration::hours(1)); // ❌ Planification
            return Decision::deferred("Contexte futur requis");
        }
        
        Decision::accepted()
    }
}
```

**Violations :**
- INTERD-TIME-1 : Ordonnancement
- INTERD-TIME-2 : Planification
- KERN-INTERD-1 : Clock pour logique décisionnelle
- INV-DIFF-NOPLAN : Décision différée sans planification

---

### 12.4. ❌ StrongFather avec mutation d'état

```rust
// ❌ IMPLÉMENTATION INVALIDE : Mutation d'état
pub struct StrongFather {
    evaluation_count: usize, // ❌ INTERD-STATE-1
    last_intent: Option<Intent>, // ❌ INTERD-STATE-1
    user_stats: HashMap<String, UserStats>, // ❌ INTERD-STATE-2
}

impl StrongFather {
    pub fn evaluate_intent(&mut self, intent: &Intent) -> Decision {
        self.evaluation_count += 1; // ❌ INTERD-STATE-1 : État système
        self.last_intent = Some(intent.clone()); // ❌ INTERD-STATE-1
        
        // ❌ INTERD-STATE-2 : État utilisateur
        let stats = self.user_stats.entry(intent.call_context.caller_identity.clone())
            .or_insert_with(UserStats::new);
        stats.increment_evaluations();
        
        self.evaluate(intent)
    }
}
```

**Violations :**
- INTERD-STATE-1 : État système
- INTERD-STATE-2 : État utilisateur
- INV-EXEC-2 : Aucune modification d'état
- INV-BEHAV-1 : Non-modification d'état
- G-EXEC-1 : Aucun effet de bord

---

### 12.5. ❌ StrongFather avec logique métier

```rust
// ❌ IMPLÉMENTATION INVALIDE : Logique métier
impl PolicyEngine {
    pub fn evaluate_permission(&self, policy: &Policy, intent: &Intent) -> PolicyResult {
        // ❌ Logique métier spécifique (exemple : e-commerce)
        if intent.subject.starts_with("order_") {
            let order: Order = serde_json::from_str(&intent.data)?; // ❌ Parsing métier
            if order.total > 1000.0 {
                return PolicyResult::denied("Montant trop élevé"); // ❌ Règle métier
            }
            if !self.validate_credit_card(&order.payment) { // ❌ Validation technique
                return PolicyResult::denied("Carte invalide");
            }
        }
        
        PolicyResult::allowed()
    }
}
```

**Violations :**
- Policy Engine Contract section 2.3 : Pas de logique métier spécifique
- Execution Prohibition Contract section 3.5 : Pas de logique métier
- Core Decision Contract section 2.3 : Pas de validation technique

---

### 12.6. ❌ StrongFather avec callback exécutable

```rust
// ❌ IMPLÉMENTATION INVALIDE : Callback exécutable
#[derive(Debug, Clone)]
pub struct Decision {
    pub intent_id: String,
    pub decision_type: DecisionType,
    pub execute: Box<dyn Fn() -> ()>, // ❌ Callback exécutable
}

impl Decision {
    pub fn call(&self) {
        (self.execute)(); // ❌ Exécution interdite
    }
}
```

**Violations :**
- INV-EXEC-1 : Aucune exécution
- G-NOEXEC-1 : Aucune exécution
- Core Decision Contract section 2.3 : Décision non-exécutable

---

### 12.7. ❌ StrongFather avec mélange erreur/rejet

```rust
// ❌ IMPLÉMENTATION INVALIDE : Mélange erreur/rejet
impl StrongFather {
    pub fn evaluate_intent(&self, intent: &Intent) -> Result<Decision, SFError> {
        // ❌ INCORRECT : Retourner une erreur pour un rejet structurel
        if intent.intent_id.is_empty() {
            return Err(SFError::StructuralError {
                reason: "Identifiant vide".to_string(),
                location: "IntentValidator".to_string(),
            }); // ❌ Rejet structurel ≠ Erreur
        }
        
        // ❌ INCORRECT : Retourner un rejet pour une erreur
        if self.policy_engine.is_corrupted() {
            return Ok(Decision {
                decision_type: DecisionType::Refused {
                    reason: RefusalReason::InternalError, // ❌ Erreur ≠ Rejet
                },
                // ...
            });
        }
    }
}
```

**Violations :**
- INV-ERR-1 : Distinction erreur/rejet
- Error & Rejection Model section 2 : Distinction fondamentale

---

## 13. Mini log de génération

### 13.1. Erreurs d'interprétation rencontrées

**E1 : Cache "pour performance"**

**Erreur d'interprétation :** Un développeur pourrait penser qu'un cache en mémoire est acceptable car "ce n'est pas de la persistance sur disque".

**Correction :** Clarification que toute forme de persistance opérationnelle (cache, état mutable) est interdite, même en mémoire. Référence : INTERD-PERS-3, INV-EXEC-3.

---

**E2 : Clock pour "validation temporelle"**

**Erreur d'interprétation :** Un développeur pourrait penser qu'utiliser Clock pour valider si une intention est "trop ancienne" est acceptable.

**Correction :** Clarification que Clock est autorisé uniquement pour l'horodatage de traces après production de décision, jamais pour la logique décisionnelle. Référence : KERN-AUTH-3, KERN-INTERD-1.

---

**E3 : Rejet = Erreur**

**Erreur d'interprétation :** Un développeur pourrait penser qu'un rejet structurel doit retourner une erreur (`Err(SFError)`).

**Correction :** Clarification que les rejets sont des résultats normaux d'évaluation (décisions REFUSÉES), pas des dysfonctionnements. Référence : Error & Rejection Model section 2, INV-ERR-1.

---

**E4 : Planification pour DIFFÉRÉE**

**Erreur d'interprétation :** Un développeur pourrait penser qu'une décision DIFFÉRÉE doit être "planifiée" pour réévaluation automatique.

**Correction :** Clarification que INV-DIFF-NOPLAN interdit toute planification. Seul l'adaptateur décide quand re-soumettre. Référence : INV-DIFF-NOPLAN, INTERD-TIME-2.

---

### 13.2. Ambiguïtés corrigées

**A1 : "Pureté fonctionnelle" vs "État interne"**

**Ambiguïté :** Un développeur pourrait se demander si un état interne (comme le Policy Engine avec ses politiques chargées) viole la pureté fonctionnelle.

**Clarification :** La pureté fonctionnelle concerne l'absence d'effet de bord sur le système externe. Un état interne immuable (politiques chargées) est acceptable. Ce qui est interdit : mutation d'état entre évaluations, cache, compteurs, etc.

**Référence :** INV-EXEC-5, INV-BEHAV-3, G-EXEC-1

---

**A2 : "Traçabilité" vs "Persistance opérationnelle"**

**Ambiguïté :** Un développeur pourrait se demander si la traçabilité (via Logger) viole l'interdiction de persistance.

**Clarification :** La traçabilité est autorisée via le kernel (KERN-AUTH-2) car elle est passive et n'affecte pas le comportement. La persistance opérationnelle (cache, état mutable) est interdite car elle affecte le comportement.

**Référence :** Audit & Trace Contract, Boundary & Isolation Contract (KERN-AUTH-2), Execution Prohibition Contract (INTERD-PERS-*)

---

**A3 : "Déterminisme" vs "Performance"**

**Ambiguïté :** Un développeur pourrait se demander si le déterminisme empêche toute optimisation.

**Clarification :** Le déterminisme (INV-POL-3) garantit que pour une entrée donnée, la sortie est toujours la même. Les optimisations d'algorithme sont autorisées tant qu'elles préservent le déterminisme. Ce qui est interdit : cache, état mutable entre évaluations, sources de non-déterminisme.

**Référence :** INV-POL-3, Policy Engine Contract section 7

---

### 13.3. Décisions éditoriales prises

**D1 : Structure du document**

**Décision :** Organiser le document en sections thématiques (principes, traduction, patterns, interdictions) plutôt que par contrat, pour faciliter la lecture par un développeur.

**Justification :** Un développeur implémente par composant, pas par contrat. L'organisation thématique facilite la compréhension.

---

**D2 : Exemples de code**

**Décision :** Inclure des exemples de code Rust concrets mais non exécutables (pseudo-code illustratif).

**Justification :** Les exemples facilitent la compréhension mais ne doivent pas être copiés-collés sans adaptation. Le pseudo-code illustre les concepts sans imposer une implémentation spécifique.

---

**D3 : Section "Implémentations INVALIDES"**

**Décision :** Ajouter une section dédiée aux implémentations invalides avec code d'exemple et violations explicites.

**Justification :** Les contre-exemples sont aussi pédagogiques que les exemples positifs. Montrer ce qu'il ne faut PAS faire évite les erreurs courantes.

---

**D4 : Références contractuelles systématiques**

**Décision :** Référencer systématiquement les contrats et règles violées pour chaque pattern interdit.

**Justification :** Permet au développeur de comprendre pourquoi un pattern est interdit et de vérifier dans les contrats FONDATION.

---

**D5 : Ton pédagogique mais strict**

**Décision :** Utiliser un ton pédagogique (explications, justifications) mais strict (interdictions absolues, pas de compromis).

**Justification :** Le document doit guider sans être complaisant. Les contrats FONDATION sont absolus, le guide doit refléter cette rigueur.

---

**Conclusion :** Ce document guide l'implémentation de StrongFather en respectant strictement tous les contrats FONDATION v1.1. Toute interprétation qui contredit un contrat FONDATION est invalide. Les contrats FONDATION priment toujours sur ce guide.

---

**Document créé le :** 2026-01-25  
**Version :** 1.0  
**Statut :** POST-FONDATION / NON NORMATIF / INFORMATIF  
**Référence :** StrongFather Contrats FONDATION v1.1 (gelés, non modifiables)  
**Type :** Guide d'implémentation non contractuel
