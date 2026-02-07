# WorrySentinel - Reference Implementation Guidelines

## 1. Contexte

Ce document définit les **guidelines d'implémentation de référence** pour WorrySentinel. Il fournit des orientations conceptuelles pour traduire les contrats de gouvernance en implémentation concrète, tout en respectant strictement les invariants et la nature non-exécutive de WorrySentinel.

**Document fondateur :** [WorrySentinel - Documentation Fondatrice](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md)

**Statut :** Ce document est un **guide d'implémentation**, pas un contrat normatif. Il fournit des recommandations, pas des obligations. Les invariants restent non négociables.

---

## 2. Portée / Scope

- **Objectif :** Guider l'implémentation de WorrySentinel en respectant les contrats
- **Public cible :** Développeurs Rust, architectes système
- **Approche :** Conceptuelle d'abord, technique ensuite
- **Langage cible :** Rust (implémentation future)
- **Ne couvre pas :** Les détails d'implémentation des contrôles de sécurité (responsabilité des cores fonctionnels)

---

## 3. Principes d'implémentation

### 3.1 Principe fondamental

**WorrySentinel est un gouvernant conceptuel, pas un exécuteur.**

L'implémentation doit refléter cette nature :

| Implémentation correcte | Implémentation incorrecte |
|-------------------------|---------------------------|
| ✅ Structures de données déclaratives | ❌ Logique d'exécution de contrôles |
| ✅ Règles de gouvernance explicites | ❌ Algorithmes de vérification |
| ✅ Interfaces de consultation | ❌ Fonctions de modification d'état |
| ✅ Traçabilité des décisions | ❌ Persistance directe de données |
| ✅ Zero-trust par défaut | ❌ Confiance implicite |

### 3.2 Traduction des invariants en code

#### INV-WS-1 : Aucune autorité sur l'implémentation

**Traduction Rust (conceptuelle) :**

```rust
// CORRECT : Structure de gouvernance déclarative
pub struct SecurityLevelGovernance {
    level: SecurityLevel,
    constraints: Vec<Constraint>,
    justification: String,
}

// INCORRECT : Implémentation de contrôle de sécurité
// pub fn verify_security(data: &Data) -> bool { ... }
```

**Règle d'implémentation :** WorrySentinel ne doit contenir aucune fonction qui implémente un contrôle de sécurité concret.

#### INV-WS-2 : Aucune autorité sur l'exécution

**Traduction Rust (conceptuelle) :**

```rust
// CORRECT : Déclaration de contrainte
pub fn declare_constraint(level: SecurityLevel) -> Constraint {
    Constraint::new(level)
}

// INCORRECT : Exécution de vérification
// pub fn execute_verification(ctx: &Context) -> Result<()> { ... }
```

**Règle d'implémentation :** WorrySentinel ne doit contenir aucune fonction qui exécute une vérification ou un contrôle.

#### INV-WS-3 : Aucune autorité sur la persistance

**Traduction Rust (conceptuelle) :**

```rust
// CORRECT : Produire une décision de gouvernance à transmettre
pub fn produce_governance_decision(
    context: &GovernanceContext
) -> GovernanceDecision {
    GovernanceDecision {
        level: context.security_level,
        state: context.trust_state,
        constraints: compute_constraints(context),
        trace: TraceMeta::new(),
    }
}

// INCORRECT : Persister directement
// pub fn save_decision(decision: &Decision) -> Result<()> { ... }
```

**Règle d'implémentation :** WorrySentinel ne doit jamais appeler de fonction de persistance directement. La persistance est transmise via les adaptateurs.

#### INV-WS-4 : Aucune modification d'état

**Traduction Rust (conceptuelle) :**

```rust
// CORRECT : Déclarer un état cible
pub fn declare_target_state(
    current: TrustState,
    signals: &[IntegritySignal]
) -> TrustStateDeclaration {
    TrustStateDeclaration {
        current,
        target: evaluate_target(current, signals),
        transition_reason: explain_transition(signals),
    }
}

// INCORRECT : Modifier l'état directement
// pub fn set_trust_state(&mut self, state: TrustState) { ... }
```

**Règle d'implémentation :** WorrySentinel ne doit jamais contenir de méthodes `&mut self` qui modifient un état système.

#### INV-WS-5 : Aucune logique temporelle technique

**Traduction Rust (conceptuelle) :**

```rust
// CORRECT : Définir des conditions logiques
pub fn define_transition_conditions(
    state: TrustState
) -> TransitionConditions {
    TransitionConditions {
        from: state,
        conditions: vec![
            Condition::AnomalyDetected,
            Condition::PersistentInconsistency,
        ],
    }
}

// INCORRECT : Utiliser le temps technique
// pub fn schedule_verification(at: SystemTime) { ... }
// pub fn wait_timeout(duration: Duration) { ... }
```

**Règle d'implémentation :** WorrySentinel ne doit jamais utiliser `std::time`, `tokio::time`, ou toute autre bibliothèque temporelle.

### 3.3 Structure de données recommandée

#### Niveaux de sécurité

```rust
/// Niveau de sécurité (0-4)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    /// Niveau 0 — Public / Display
    Public = 0,
    /// Niveau 1 — Standard / CMS
    Standard = 1,
    /// Niveau 2 — Sensitive Data
    SensitiveData = 2,
    /// Niveau 3 — Critical System
    CriticalSystem = 3,
    /// Niveau 4 — Hardened / Isolated
    Hardened = 4,
}
```

#### États de confiance

```rust
/// État de confiance système (T0-T4)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustState {
    /// T0 — Normal (Nominal)
    Normal = 0,
    /// T1 — Instable (Doute)
    Unstable = 1,
    /// T2 — Dégradé (Suspect)
    Degraded = 2,
    /// T3 — Restreint (Critique)
    Restricted = 3,
    /// T4 — Bloqué (Compromis)
    Blocked = 4,
}
```

#### Décision de gouvernance

```rust
/// Décision de gouvernance produite par WorrySentinel
#[derive(Debug, Clone)]
pub struct GovernanceDecision {
    /// Niveau de sécurité applicable
    pub security_level: SecurityLevel,
    /// État de confiance déclaré
    pub trust_state: TrustState,
    /// Contraintes imposées aux cores
    pub constraints: Vec<CoreConstraint>,
    /// Métadonnées de traçabilité
    pub trace: TraceMeta,
}
```

#### Traçabilité

```rust
/// Métadonnées de traçabilité (INV-WS-8)
#[derive(Debug, Clone)]
pub struct TraceMeta {
    /// Contexte ayant déclenché la décision
    pub context: String,
    /// Règles de gouvernance appliquées
    pub rules_applied: Vec<String>,
    /// Justification de la décision
    pub justification: String,
}
```

---

## 4. Architecture d'implémentation

### 4.1 Structure modulaire recommandée

```
worry_sentinel/
├── lib.rs                    # Point d'entrée, exports
├── levels/
│   ├── mod.rs               # Module niveaux de sécurité
│   ├── security_level.rs    # Enum et traits SecurityLevel
│   └── level_governance.rs  # Règles de gouvernance des niveaux
├── states/
│   ├── mod.rs               # Module états de confiance
│   ├── trust_state.rs       # Enum et traits TrustState
│   └── state_governance.rs  # Règles de gouvernance des états
├── degradation/
│   ├── mod.rs               # Module dégradation
│   └── progressive.rs       # Règles de dégradation progressive
├── governance/
│   ├── mod.rs               # Module gouvernance
│   ├── decision.rs          # GovernanceDecision
│   ├── constraints.rs       # CoreConstraint
│   └── rules.rs             # Règles explicites
├── observation/
│   ├── mod.rs               # Module observation
│   ├── signals.rs           # IntegritySignal
│   └── correlation.rs       # Corrélation des signaux
├── trace/
│   ├── mod.rs               # Module traçabilité
│   └── meta.rs              # TraceMeta
└── interfaces/
    ├── mod.rs               # Module interfaces
    ├── query.rs             # Interfaces de consultation
    └── signal.rs            # Interfaces de signalement
```

### 4.2 Interfaces de consultation

```rust
/// Interface de consultation du niveau de sécurité
pub trait SecurityLevelQuery {
    /// Interroge le niveau de sécurité d'une entité
    fn query_security_level(&self, entity_id: &EntityId) -> SecurityLevel;
}

/// Interface de consultation de l'état de confiance
pub trait TrustStateQuery {
    /// Interroge l'état de confiance global
    fn query_trust_state(&self) -> TrustState;
}

/// Interface de consultation des contraintes
pub trait ConstraintQuery {
    /// Interroge les contraintes applicables à un core
    fn query_constraints(&self, core_id: &CoreId) -> Vec<CoreConstraint>;
}
```

### 4.3 Interfaces de signalement

```rust
/// Signal d'intégrité reçu des cores
pub struct IntegritySignal {
    pub source: CoreId,
    pub signal_type: SignalType,
    pub severity: Severity,
    pub details: String,
}

/// Interface de réception des signaux
pub trait SignalReceiver {
    /// Reçoit un signal d'intégrité
    fn receive_signal(&self, signal: IntegritySignal);
}
```

---

## 5. Patterns recommandés

### 5.1 Pattern : Gouvernance déclarative

**Problème :** Comment exprimer les règles de gouvernance sans logique impérative ?

**Solution :** Utiliser des structures de données déclaratives et des règles explicites.

```rust
/// Règle de gouvernance déclarative
pub struct GovernanceRule {
    /// Identifiant unique de la règle
    pub id: RuleId,
    /// Condition d'application
    pub condition: RuleCondition,
    /// Contraintes produites
    pub constraints: Vec<CoreConstraint>,
    /// Justification
    pub justification: String,
}

/// Application des règles (déclaratif, pas impératif)
pub fn apply_rules(
    rules: &[GovernanceRule],
    context: &GovernanceContext
) -> Vec<CoreConstraint> {
    rules
        .iter()
        .filter(|rule| rule.condition.matches(context))
        .flat_map(|rule| rule.constraints.clone())
        .collect()
}
```

### 5.2 Pattern : Traçabilité systématique

**Problème :** Comment garantir la traçabilité complète (INV-WS-8) ?

**Solution :** Inclure les métadonnées de traçabilité dans chaque décision.

```rust
/// Builder de décision avec traçabilité obligatoire
pub struct GovernanceDecisionBuilder {
    security_level: Option<SecurityLevel>,
    trust_state: Option<TrustState>,
    constraints: Vec<CoreConstraint>,
    context: Option<String>,
    rules_applied: Vec<String>,
    justification: Option<String>,
}

impl GovernanceDecisionBuilder {
    pub fn new() -> Self { ... }
    
    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }
    
    pub fn with_rule(mut self, rule_id: &str) -> Self {
        self.rules_applied.push(rule_id.to_string());
        self
    }
    
    pub fn with_justification(mut self, justification: String) -> Self {
        self.justification = Some(justification);
        self
    }
    
    /// Build final — échoue si traçabilité incomplète
    pub fn build(self) -> Result<GovernanceDecision, TraceError> {
        let trace = TraceMeta {
            context: self.context.ok_or(TraceError::MissingContext)?,
            rules_applied: self.rules_applied,
            justification: self.justification.ok_or(TraceError::MissingJustification)?,
        };
        
        Ok(GovernanceDecision {
            security_level: self.security_level.unwrap_or(SecurityLevel::Standard),
            trust_state: self.trust_state.unwrap_or(TrustState::Normal),
            constraints: self.constraints,
            trace,
        })
    }
}
```

### 5.3 Pattern : Zero-trust par construction

**Problème :** Comment garantir le zero-trust (INV-WS-6) ?

**Solution :** Ne jamais présupposer la validité des entrées.

```rust
/// Contexte de gouvernance avec validation
pub struct ValidatedGovernanceContext {
    inner: GovernanceContext,
}

impl ValidatedGovernanceContext {
    /// Crée un contexte validé — échoue si invalide
    pub fn validate(ctx: GovernanceContext) -> Result<Self, ValidationError> {
        // Vérifier que le niveau de sécurité est défini
        if ctx.security_level.is_none() {
            return Err(ValidationError::MissingSecurityLevel);
        }
        
        // Vérifier que l'entité est identifiée
        if ctx.entity_id.is_empty() {
            return Err(ValidationError::MissingEntityId);
        }
        
        // Aucune confiance présupposée — tout est vérifié
        Ok(Self { inner: ctx })
    }
}
```

---

## 6. Anti-patterns à éviter

### 6.1 Anti-pattern : Implémentation de contrôle

**Mauvais :**

```rust
// ❌ WorrySentinel implémente un contrôle de sécurité
impl WorrySentinel {
    pub fn verify_token(&self, token: &str) -> bool {
        // Logique de vérification de token
        jwt::verify(token, &self.secret_key)
    }
}
```

**Correct :**

```rust
// ✅ WorrySentinel gouverne les contraintes de vérification
impl WorrySentinel {
    pub fn get_verification_constraints(&self, level: SecurityLevel) -> VerificationConstraints {
        match level {
            SecurityLevel::Public => VerificationConstraints::minimal(),
            SecurityLevel::Standard => VerificationConstraints::basic(),
            SecurityLevel::SensitiveData => VerificationConstraints::reinforced(),
            SecurityLevel::CriticalSystem => VerificationConstraints::strict(),
            SecurityLevel::Hardened => VerificationConstraints::maximum(),
        }
    }
}
```

### 6.2 Anti-pattern : Modification d'état directe

**Mauvais :**

```rust
// ❌ WorrySentinel modifie l'état directement
impl WorrySentinel {
    pub fn set_trust_state(&mut self, state: TrustState) {
        self.current_state = state;
    }
}
```

**Correct :**

```rust
// ✅ WorrySentinel déclare un état cible
impl WorrySentinel {
    pub fn declare_state_transition(
        &self,
        current: TrustState,
        signals: &[IntegritySignal]
    ) -> StateTransitionDeclaration {
        StateTransitionDeclaration {
            from: current,
            to: self.evaluate_target_state(current, signals),
            justification: self.explain_transition(signals),
        }
    }
}
```

### 6.3 Anti-pattern : Logique temporelle

**Mauvais :**

```rust
// ❌ WorrySentinel utilise le temps technique
impl WorrySentinel {
    pub async fn schedule_verification(&self, delay: Duration) {
        tokio::time::sleep(delay).await;
        self.verify();
    }
}
```

**Correct :**

```rust
// ✅ WorrySentinel définit des conditions logiques
impl WorrySentinel {
    pub fn define_verification_trigger(&self) -> VerificationTrigger {
        VerificationTrigger {
            conditions: vec![
                TriggerCondition::OnAnomalyDetected,
                TriggerCondition::OnStateTransition,
                TriggerCondition::OnCriticalDecision,
            ],
        }
    }
}
```

### 6.4 Anti-pattern : Accès direct à la persistance

**Mauvais :**

```rust
// ❌ WorrySentinel accède directement à KindMother
impl WorrySentinel {
    pub async fn save_decision(&self, decision: &Decision) -> Result<()> {
        self.kind_mother.persist(decision).await
    }
}
```

**Correct :**

```rust
// ✅ WorrySentinel produit une décision à transmettre
impl WorrySentinel {
    pub fn produce_decision(&self, context: &Context) -> GovernanceDecision {
        GovernanceDecision {
            // ... décision complète avec traçabilité
        }
        // La persistance est gérée par l'adaptateur, pas par WorrySentinel
    }
}
```

---

## 7. Tests recommandés

### 7.1 Types de tests

| Type de test | Objectif | Exemple |
|--------------|----------|---------|
| **Tests de conformité** | Vérifier le respect des invariants | `test_no_state_mutation` |
| **Tests de gouvernance** | Vérifier les règles de gouvernance | `test_security_level_constraints` |
| **Tests de traçabilité** | Vérifier la complétude de la trace | `test_trace_completeness` |
| **Tests de cohérence** | Vérifier la cohérence inter-niveaux | `test_level_coherence` |

### 7.2 Tests de conformité aux invariants

```rust
#[cfg(test)]
mod invariant_tests {
    use super::*;

    /// INV-WS-1 : Aucune autorité sur l'implémentation
    #[test]
    fn test_no_implementation_logic() {
        // WorrySentinel ne doit contenir aucune fonction
        // qui implémente un contrôle de sécurité
        let ws = WorrySentinel::new();
        
        // La fonction doit retourner des contraintes, pas effectuer de vérification
        let constraints = ws.get_constraints(SecurityLevel::Standard);
        assert!(!constraints.is_empty());
        // Pas de vérification effectuée, juste des contraintes déclarées
    }

    /// INV-WS-4 : Aucune modification d'état
    #[test]
    fn test_no_state_mutation() {
        let ws = WorrySentinel::new();
        let initial_state = ws.query_trust_state();
        
        // Simuler une opération
        let _decision = ws.produce_decision(&GovernanceContext::default());
        
        // L'état ne doit pas avoir changé
        assert_eq!(initial_state, ws.query_trust_state());
    }

    /// INV-WS-8 : Traçabilité complète
    #[test]
    fn test_trace_completeness() {
        let ws = WorrySentinel::new();
        let decision = ws.produce_decision(&GovernanceContext::default());
        
        // La trace doit être complète
        assert!(!decision.trace.context.is_empty());
        assert!(!decision.trace.rules_applied.is_empty());
        assert!(!decision.trace.justification.is_empty());
    }
}
```

### 7.3 Tests de gouvernance

```rust
#[cfg(test)]
mod governance_tests {
    use super::*;

    /// Test des contraintes par niveau de sécurité
    #[test]
    fn test_security_level_constraints() {
        let ws = WorrySentinel::new();
        
        // Niveau 0 : contraintes minimales
        let c0 = ws.get_constraints(SecurityLevel::Public);
        assert!(c0.len() <= ws.get_constraints(SecurityLevel::Standard).len());
        
        // Niveau 4 : contraintes maximales
        let c4 = ws.get_constraints(SecurityLevel::Hardened);
        assert!(c4.len() >= ws.get_constraints(SecurityLevel::CriticalSystem).len());
    }

    /// Test des transitions d'état autorisées
    #[test]
    fn test_state_transitions() {
        let ws = WorrySentinel::new();
        
        // T0 → T1 autorisé
        assert!(ws.is_transition_allowed(TrustState::Normal, TrustState::Unstable));
        
        // T0 → T4 interdit (pas de saut brutal)
        assert!(!ws.is_transition_allowed(TrustState::Normal, TrustState::Blocked));
    }
}
```

---

## 8. Intégration avec l'écosystème

### 8.1 Point d'intégration avec les adaptateurs

WorrySentinel s'intègre avec les adaptateurs produits via les interfaces définies :

```rust
/// Adaptateur produit interrogeant WorrySentinel
pub trait ProductAdapter {
    fn query_governance(&self, ws: &WorrySentinel) -> GovernanceDecision {
        let context = self.build_governance_context();
        ws.produce_decision(&context)
    }
}
```

### 8.2 Point d'intégration avec les cores

Les cores fonctionnels consultent WorrySentinel pour adapter leur comportement :

```rust
/// Exemple d'intégration avec StrongFather
impl StrongFather {
    pub fn evaluate_intent(&self, intent: &Intent, ws: &WorrySentinel) -> Decision {
        // Consulter WorrySentinel pour les contraintes
        let constraints = ws.query_constraints(&CoreId::StrongFather);
        
        // Adapter la sévérité selon les contraintes
        let severity = self.compute_severity(&constraints);
        
        // Évaluer l'intention avec la sévérité adaptée
        self.evaluate_with_severity(intent, severity)
    }
}
```

---

## 9. Checklist d'implémentation

### 9.1 Avant de commencer

| Vérification | Statut |
|--------------|--------|
| Documentation Fondatrice lue et comprise | ☐ |
| Invariants INV-WS-1 à INV-WS-8 maîtrisés | ☐ |
| Invariants INV-GOV-1 à INV-GOV-8 maîtrisés | ☐ |
| Contrats de gouvernance compris | ☐ |
| Architecture conceptuelle validée | ☐ |

### 9.2 Pendant l'implémentation

| Vérification | Statut |
|--------------|--------|
| Aucune fonction d'implémentation de contrôle | ☐ |
| Aucune fonction d'exécution de vérification | ☐ |
| Aucun accès direct à la persistance | ☐ |
| Aucune modification d'état `&mut self` | ☐ |
| Aucune utilisation de bibliothèque temporelle | ☐ |
| Zero-trust appliqué à toutes les entrées | ☐ |
| Traçabilité complète sur chaque décision | ☐ |
| Tests de conformité aux invariants | ☐ |

### 9.3 Après l'implémentation

| Vérification | Statut |
|--------------|--------|
| Revue de code par un pair | ☐ |
| Tests de conformité passés | ☐ |
| Tests de gouvernance passés | ☐ |
| Documentation à jour | ☐ |
| Audit de sécurité (si niveau 3+) | ☐ |

---

## 10. Documents associés

| Document | Relation |
|----------|----------|
| [WorrySentinel - Documentation Fondatrice](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md) | Document source |
| [WorrySentinel - Invariants & Guarantees](../contracts/governance/WorrySentinel%20-%20Invariants%20&%20Guarantees.md) | Invariants à respecter |
| [WorrySentinel - Architecture & Flows](../architecture/WorrySentinel%20-%20Architecture%20&%20Flows.md) | Architecture conceptuelle |
| [WorrySentinel - Security Levels Governance Contract](../contracts/levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md) | Règles niveaux de sécurité |
| [WorrySentinel - Trust States Governance Contract](../contracts/levels/WorrySentinel%20-%20Trust%20States%20Governance%20Contract.md) | Règles états de confiance |

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Guide d'implémentation — Recommandations  
**Langage cible :** Rust  
**Type :** Guidelines d'implémentation de référence
