# LogisticsSteward - Reference Implementation Guidelines

## 1. Contexte

Ce document fournit des guidelines pour l'implementation de reference de LogisticsSteward. Il guide les developpeurs dans la creation d'une implementation conforme aux contrats et invariants de LogisticsSteward, en fournissant des recommandations pratiques, des patterns, et des bonnes pratiques.

Ce document complete l'[Architecture & Flows](../architecture/LogisticsSteward%20-%20Architecture%20%26%20Flows.md) et les [Invariants & Guarantees](../contracts/governance/LogisticsSteward%20-%20Invariants%20%26%20Guarantees.md) en fournissant des directives concretes d'implementation.

L'implementation doit respecter les [Lois d'Autonomie Systeme](..//..//..//miyukini-webway-system//reference//_index.md) : aucune dependance externe critique (**LOI-1**), fonctionnement en mode offline (**LOI-2**), etat local souverain (**LOI-3**), et cout proportionnel au hardware (**LOI-5**).

**Navigation :** [Index LogisticsSteward](../_index.md)

## 2. Portee / Scope

Ce document couvre :
- Les principes d'implementation
- Les patterns recommandes
- Les structures de code recommandees
- Les bonnes pratiques
- Les pieges a eviter
- Les exemples de code
- Les guidelines de test

Ce document **ne couvre pas** :
- Les specifications contractuelles (voir les contrats specifiques)
- Les details d'implementation specifiques a une technologie
- Les optimisations specifiques a un environnement

---

## 3. Principes d'implementation

### 3.1 Conformite aux contrats

**Principe IMPL-LS-01 : Contrats d'abord**

L'implementation doit respecter strictement tous les contrats de LogisticsSteward. Aucun compromis n'est autorise.

**Implications :**
- Tous les contrats sont implementes integralement
- Aucune violation d'invariant (INV-LS-1 a INV-LS-10)
- Tous les tests de contrat passent
- Documentation a jour

### 3.2 Arbitrage sans execution

**Principe IMPL-LS-02 : Separation absolue**

LogisticsSteward decide, il n'execute jamais. Cette separation est fondamentale et non negociable.

**Implications :**
- Aucun appel systeme bas niveau
- Aucune allocation memoire directe
- Aucun controle de threads ou processus
- Toutes les decisions sont des recommandations pour le Kernel

```typescript
// âœ… BON : Decision pure, pas d'execution
class ArbitrationEngine {
  arbitrate(request: ResourceRequest, context: ArbitrationContext): ArbitrationDecision {
    // Evaluation des regles (pur)
    const applicableRules = this.findApplicableRules(request, context);
    const evaluation = this.evaluateRules(applicableRules, context);
    
    // Generation de la decision (pur)
    return this.generateDecision(request, evaluation);
  }
}

// âŒ MAUVAIS : Execution directe
class ArbitrationEngine {
  arbitrate(request: ResourceRequest, context: ArbitrationContext): void {
    // Violation INV-LS-1 : execution directe
    allocateMemory(request.amount);
    scheduleThread(request.priority);
  }
}
```

### 3.3 Determinisme absolu

**Principe IMPL-LS-03 : Memes entrees = meme decision**

L'arbitrage doit etre completement deterministe. Aucune source d'aleatoire, aucune dependance a l'horloge pour les decisions.

**Implications :**
- Pas de `Math.random()` ou equivalent
- Pas de dependance a `Date.now()` dans la logique de decision
- Resultats reproductibles
- Tests deterministes

```typescript
// âœ… BON : Decision deterministe
class PriorityCalculator {
  calculate(entity: Entity, context: ArbitrationContext): Priority {
    // Calcul base uniquement sur les entrees
    const basePriority = entity.declaredPriority;
    const quotaUsage = context.quotaUsage[entity.id];
    const degradationLevel = context.systemState.degradationLevel;
    
    // Formule deterministe
    return this.computePriority(basePriority, quotaUsage, degradationLevel);
  }
}

// âŒ MAUVAIS : Decision non deterministe
class PriorityCalculator {
  calculate(entity: Entity, context: ArbitrationContext): Priority {
    // Violation INV-LS-4 : composante aleatoire
    const randomBoost = Math.random() > 0.5 ? 1 : 0;
    return entity.declaredPriority + randomBoost;
  }
}
```

### 3.4 Tracabilite complete

**Principe IMPL-LS-04 : Tout est journalise**

Chaque demande, evaluation et decision doit etre journalisee avec son contexte complet pour permettre l'audit.

**Implications :**
- Journal de chaque demande recue
- Journal de chaque evaluation effectuee
- Journal de chaque decision generee
- Contexte complet pour reconstitution

```typescript
// âœ… BON : Journalisation complete
class DecisionGenerator {
  generate(
    request: ResourceRequest,
    evaluation: RuleEvaluation,
    context: ArbitrationContext
  ): ArbitrationDecision {
    const decision = this.computeDecision(request, evaluation, context);
    
    // Journalisation complete (INV-LS-6)
    this.auditJournal.log({
      decisionId: decision.id,
      timestamp: this.clock.now(),
      request: request,
      rulesApplied: evaluation.rulesApplied,
      contextSnapshot: this.snapshotContext(context),
      decision: decision,
      justification: decision.justification
    });
    
    return decision;
  }
}
```

### 3.5 Validation externe obligatoire

**Principe IMPL-LS-05 : StrongFather valide**

Toute decision doit etre soumise a StrongFather pour validation. Aucune decision auto-appliquee.

**Implications :**
- Interface avec StrongFather obligatoire
- Respect de la validation/invalidation
- Pas de contournement possible
- Gestion du cas StrongFather indisponible

```typescript
// âœ… BON : Soumission a validation
class ValidationPreparer {
  async prepareAndSubmit(decision: ArbitrationDecision): Promise<ValidationResult> {
    // Formatage pour StrongFather
    const validationRequest = this.formatForValidation(decision);
    
    // Soumission obligatoire (INV-LS-8)
    const result = await this.strongFatherAdapter.submit(validationRequest);
    
    // Respect de la decision de StrongFather
    if (result.status === 'INVALID') {
      this.handleInvalidation(decision, result);
      return result;
    }
    
    return result;
  }
}

// âŒ MAUVAIS : Decision auto-appliquee
class ValidationPreparer {
  async prepareAndSubmit(decision: ArbitrationDecision): Promise<ValidationResult> {
    // Violation INV-LS-8 : execution sans validation
    this.executeDecision(decision);
    return { status: 'APPLIED' };
  }
}
```

---

## 4. Structure de code recommandee

### 4.1 Organisation par couches

**Structure recommandee :**

```
logistics-steward/
â”œâ”€â”€ reception-layer/           # Couche Reception
â”‚   â”œâ”€â”€ request-receiver/
â”‚   â”œâ”€â”€ structural-validator/
â”‚   â”œâ”€â”€ entity-identifier/
â”‚   â””â”€â”€ request-logger/
â”œâ”€â”€ context-layer/             # Couche Contexte
â”‚   â”œâ”€â”€ system-state-reader/
â”‚   â”œâ”€â”€ entity-context-resolver/
â”‚   â”œâ”€â”€ degradation-level-reader/
â”‚   â””â”€â”€ context-assembler/
â”œâ”€â”€ evaluation-layer/          # Couche Evaluation
â”‚   â”œâ”€â”€ rule-engine/
â”‚   â”œâ”€â”€ quota-evaluator/
â”‚   â”œâ”€â”€ priority-calculator/
â”‚   â”œâ”€â”€ conflict-resolver/
â”‚   â””â”€â”€ degradation-applier/
â”œâ”€â”€ decision-layer/            # Couche Decision
â”‚   â”œâ”€â”€ decision-generator/
â”‚   â”œâ”€â”€ decision-formatter/
â”‚   â”œâ”€â”€ validation-preparer/
â”‚   â””â”€â”€ decision-logger/
â”œâ”€â”€ common/                    # Composants transversaux
â”‚   â”œâ”€â”€ rule-repository/
â”‚   â”œâ”€â”€ audit-journal/
â”‚   â”œâ”€â”€ metrics-collector/
â”‚   â”œâ”€â”€ health-monitor/
â”‚   â””â”€â”€ errors/
â””â”€â”€ contracts/                 # Definitions de contrats
    â”œâ”€â”€ interfaces/
    â””â”€â”€ types/
```

### 4.2 Isolation des couches

**Regle IMPL-STRUCT-01 : Isolation stricte**

Chaque couche est isolee et ne depend que de ses interfaces, pas de l'implementation.

```typescript
// âœ… BON : Couche Evaluation depend de l'interface, pas de l'implementation
interface IContextProvider {
  getContext(entityId: string): Promise<ArbitrationContext>;
}

class RuleEngine {
  constructor(private contextProvider: IContextProvider) {}
  
  async evaluate(request: ResourceRequest): Promise<RuleEvaluation> {
    const context = await this.contextProvider.getContext(request.entityId);
    return this.doEvaluate(request, context);
  }
}

// âŒ MAUVAIS : Couplage direct a l'implementation
class RuleEngine {
  constructor(private contextAssembler: ContextAssembler) {}
  // Depend de l'implementation concrete
}
```

### 4.3 Interfaces explicites

**Regle IMPL-STRUCT-02 : Interfaces explicites**

Toutes les interfaces entre composants sont explicites et documentees.

```typescript
/**
 * Interface pour la soumission de demandes de ressources.
 * 
 * @contract IResourceRequest
 * @version 1.0
 */
interface IResourceRequestSubmission {
  /**
   * Soumet une demande de ressource pour arbitrage.
   * 
   * @param request - La demande de ressource
   * @returns La decision d'arbitrage
   * 
   * @invariant INV-LS-1 : Pas d'execution directe
   * @invariant INV-LS-4 : Decision deterministe
   */
  submit(request: ResourceRequest): Promise<ArbitrationDecision>;
}
```

---

## 5. Patterns recommandes

### 5.1 Pattern : Arbitrage pur

**Contexte :** L'arbitrage doit etre pur (sans effet de bord), deterministe, et tracable.

**Pattern :**

```typescript
/**
 * Moteur d'arbitrage pur. Fonction pure sans effet de bord.
 * 
 * @invariant INV-LS-1 : Arbitrage sans execution
 * @invariant INV-LS-4 : Decisions deterministes
 */
class ArbitrationEngine {
  constructor(
    private ruleRepository: IRuleRepository,
    private priorityCalculator: IPriorityCalculator,
    private quotaEvaluator: IQuotaEvaluator
  ) {}
  
  /**
   * Evalue une demande et genere une decision.
   * 
   * @param request - La demande a evaluer
   * @param context - Le contexte d'arbitrage
   * @returns La decision d'arbitrage
   * 
   * @pure Aucun effet de bord
   * @deterministic Memes entrees = meme sortie
   */
  evaluate(request: ResourceRequest, context: ArbitrationContext): ArbitrationDecision {
    // 1. Recuperation des regles applicables (lecture seule)
    const rules = this.ruleRepository.findApplicable(request.resourceType);
    
    // 2. Evaluation des quotas (calcul pur)
    const quotaResult = this.quotaEvaluator.evaluate(
      request,
      context.entityContext.quotas
    );
    
    // 3. Calcul de la priorite (calcul pur)
    const priority = this.priorityCalculator.calculate(
      request.entity,
      context
    );
    
    // 4. Application des regles de degradation (calcul pur)
    const degradationAdjustment = this.applyDegradation(
      request,
      context.systemState.degradationLevel
    );
    
    // 5. Generation de la decision (deterministe)
    return this.generateDecision(
      request,
      quotaResult,
      priority,
      degradationAdjustment,
      rules
    );
  }
}
```

**Avantages :**
- Respecte INV-LS-1 (pas d'execution)
- Respecte INV-LS-4 (determinisme)
- Testable facilement
- Reproductible

### 5.2 Pattern : Lecture seule de l'etat systeme

**Contexte :** LogisticsSteward consomme l'etat systeme du Kernel sans jamais le modifier.

**Pattern :**

```typescript
/**
 * Lecteur d'etat systeme. Lecture seule, jamais de modification.
 * 
 * @invariant INV-LS-2 : Etat systeme abstrait
 * @invariant INV-LS-3 : Lecture seule du systeme
 */
class SystemStateReader {
  constructor(private kernelAdapter: IKernelAdapter) {}
  
  /**
   * Lit l'etat systeme depuis le Kernel.
   * 
   * @returns L'etat systeme abstrait certifie
   * 
   * @invariant Lecture seule : aucune modification
   * @invariant Etat certifie par le Kernel
   */
  async readState(): Promise<Readonly<SystemState>> {
    // Lecture depuis le Kernel (source de verite)
    const rawState = await this.kernelAdapter.getSystemState();
    
    // Validation de la certification
    if (!this.validateCertification(rawState)) {
      throw new UncertifiedStateError('Etat systeme non certifie');
    }
    
    // Retour en lecture seule (Object.freeze)
    return Object.freeze(this.normalizeState(rawState));
  }
  
  private normalizeState(raw: RawSystemState): SystemState {
    // Normalisation independante de l'OS
    return {
      loadLevel: this.normalizeLoadLevel(raw.load),
      resourceAvailability: this.normalizeAvailability(raw.resources),
      degradationLevel: this.mapDegradationLevel(raw.degradation),
      hardwareProfile: this.normalizeHardwareProfile(raw.hardware)
    };
  }
}
```

**Avantages :**
- Respecte INV-LS-2 (etat abstrait)
- Respecte INV-LS-3 (lecture seule)
- Separation claire avec le Kernel
- Etat normalise et certifie

### 5.3 Pattern : Regles explicites

**Contexte :** Toute regle doit etre explicite, declaree, jamais implicite.

**Pattern :**

```typescript
/**
 * Repository de regles. Regles explicites uniquement.
 * 
 * @invariant INV-LS-5 : Regles explicites
 */
class RuleRepository {
  private rules: Map<string, ArbitrationRule> = new Map();
  
  /**
   * Enregistre une regle explicite.
   * 
   * @param rule - La regle a enregistrer
   * 
   * @invariant Chaque regle a un ID unique
   * @invariant Chaque regle a une definition formelle
   */
  register(rule: ArbitrationRule): void {
    // Validation de la regle
    this.validateRule(rule);
    
    // Enregistrement avec ID unique
    if (this.rules.has(rule.id)) {
      throw new DuplicateRuleError(`Regle ${rule.id} deja enregistree`);
    }
    
    this.rules.set(rule.id, rule);
  }
  
  /**
   * Trouve les regles applicables a un type de ressource.
   * 
   * @param resourceType - Le type de ressource
   * @returns Les regles applicables (jamais implicites)
   */
  findApplicable(resourceType: ResourceType): ReadonlyArray<ArbitrationRule> {
    // Filtrage explicite, pas de regle par defaut cachee
    return Array.from(this.rules.values())
      .filter(rule => rule.appliesTo(resourceType));
  }
  
  private validateRule(rule: ArbitrationRule): void {
    // Validation : regle complete et explicite
    if (!rule.id) throw new InvalidRuleError('Regle sans ID');
    if (!rule.definition) throw new InvalidRuleError('Regle sans definition');
    if (!rule.condition) throw new InvalidRuleError('Regle sans condition');
    if (!rule.action) throw new InvalidRuleError('Regle sans action');
  }
}

/**
 * Definition d'une regle d'arbitrage explicite.
 */
interface ArbitrationRule {
  /** Identifiant unique de la regle */
  id: string;
  /** Definition formelle de la regle */
  definition: string;
  /** Condition d'application */
  condition: (request: ResourceRequest, context: ArbitrationContext) => boolean;
  /** Action a recommander si la condition est vraie */
  action: (request: ResourceRequest, context: ArbitrationContext) => RuleAction;
  /** Types de ressources auxquels la regle s'applique */
  appliesTo: (resourceType: ResourceType) => boolean;
}
```

**Avantages :**
- Respecte INV-LS-5 (regles explicites)
- Auditabilite complete
- Pas de comportement cache
- Testable

### 5.4 Pattern : Degradation controlee

**Contexte :** La degradation suit des niveaux predetermines et explicites.

**Pattern :**

```typescript
/**
 * Gestionnaire de degradation. Niveaux explicites D0-D4.
 * 
 * @invariant INV-LS-9 : Degradation controlee
 */
class DegradationManager {
  /** Niveaux de degradation predetermines */
  private static readonly DEGRADATION_LEVELS: DegradationLevelConfig[] = [
    { level: 0, code: 'D0', name: 'Normal', restrictions: [] },
    { level: 1, code: 'D1', name: 'Prudent', restrictions: ['non_critical_limited'] },
    { level: 2, code: 'D2', name: 'Restreint', restrictions: ['secondary_disabled'] },
    { level: 3, code: 'D3', name: 'Critique', restrictions: ['minimal_services_only'] },
    { level: 4, code: 'D4', name: 'Survie', restrictions: ['core_only'] }
  ];
  
  /**
   * Applique les restrictions de degradation a une demande.
   * 
   * @param request - La demande
   * @param currentLevel - Le niveau de degradation actuel
   * @returns Les restrictions a appliquer
   */
  applyDegradation(
    request: ResourceRequest,
    currentLevel: DegradationLevel
  ): DegradationRestrictions {
    const config = this.getLevelConfig(currentLevel);
    
    // Application deterministe des restrictions
    return {
      restrictions: config.restrictions,
      allowRequest: this.isRequestAllowed(request, config),
      adjustedPriority: this.adjustPriority(request, config),
      justification: `Niveau ${config.code}: ${config.name}`
    };
  }
  
  /**
   * Transition vers un nouveau niveau de degradation.
   * 
   * @param currentLevel - Niveau actuel
   * @param targetLevel - Niveau cible
   * @returns La transition (progressive, jamais brutale)
   * 
   * @invariant Pas de saut de plus d'un niveau
   */
  transitionTo(
    currentLevel: DegradationLevel,
    targetLevel: DegradationLevel
  ): DegradationTransition {
    const step = Math.sign(targetLevel.level - currentLevel.level);
    const nextLevel = currentLevel.level + step;
    
    // Transition progressive (pas de saut brutal)
    return {
      from: currentLevel,
      to: this.getLevelConfig({ level: nextLevel }),
      step: step,
      isComplete: nextLevel === targetLevel.level
    };
  }
}
```

**Avantages :**
- Respecte INV-LS-9 (degradation controlee)
- Niveaux predetermines et documentes
- Transitions progressives
- Reversible

### 5.5 Pattern : Soumission a StrongFather

**Contexte :** Toute decision doit etre validee par StrongFather.

**Pattern :**

```typescript
/**
 * Preparateur de validation. Soumission obligatoire a StrongFather.
 * 
 * @invariant INV-LS-8 : Validation StrongFather
 */
class ValidationPreparer {
  constructor(
    private strongFatherAdapter: IStrongFatherAdapter,
    private offlineBuffer: IOfflineBuffer,
    private auditJournal: IAuditJournal
  ) {}
  
  /**
   * Prepare et soumet une decision pour validation.
   * 
   * @param decision - La decision a valider
   * @returns Le resultat de validation
   * 
   * @invariant Soumission obligatoire
   * @invariant Respect de la validation/invalidation
   */
  async submit(decision: ArbitrationDecision): Promise<ValidationResult> {
    // Formatage pour StrongFather
    const validationRequest = this.formatForValidation(decision);
    
    try {
      // Tentative de soumission
      const result = await this.strongFatherAdapter.submit(validationRequest);
      
      // Journalisation du resultat
      this.auditJournal.log({
        type: 'VALIDATION_RESULT',
        decisionId: decision.id,
        result: result
      });
      
      return result;
    } catch (error) {
      // StrongFather indisponible : buffer local (LOI-2)
      return this.handleStrongFatherUnavailable(decision, error);
    }
  }
  
  /**
   * Gere le cas ou StrongFather est indisponible.
   * 
   * @invariant Pas de decision auto-appliquee
   * @invariant Buffer local pour reconciliation ulterieure
   */
  private handleStrongFatherUnavailable(
    decision: ArbitrationDecision,
    error: Error
  ): ValidationResult {
    // Mise en buffer (pas d'execution)
    this.offlineBuffer.add({
      decision: decision,
      error: error,
      timestamp: Date.now(),
      status: 'PENDING_VALIDATION'
    });
    
    // Journalisation
    this.auditJournal.log({
      type: 'VALIDATION_DEFERRED',
      decisionId: decision.id,
      reason: 'StrongFather indisponible'
    });
    
    return {
      status: 'DEFERRED',
      reason: 'StrongFather indisponible, validation differee',
      decisionId: decision.id
    };
  }
}
```

**Avantages :**
- Respecte INV-LS-8 (validation StrongFather)
- Gestion du mode degrade (LOI-2)
- Tracabilite complete
- Pas d'auto-application

---

## 6. Bonnes pratiques

### 6.1 Gestion des erreurs

**Pratique ERR-LS-01 : Erreurs typees**

Utiliser des types d'erreur explicites et semantiques.

```typescript
// âœ… BON : Erreurs typees et semantiques
class QuotaExceededError extends Error {
  constructor(
    public entityId: string,
    public resourceType: ResourceType,
    public requested: number,
    public available: number
  ) {
    super(`Quota depasse pour ${entityId}: demande ${requested}, disponible ${available}`);
    this.name = 'QuotaExceededError';
  }
}

class InvalidRuleError extends Error {
  constructor(
    public ruleId: string,
    public reason: string
  ) {
    super(`Regle invalide ${ruleId}: ${reason}`);
    this.name = 'InvalidRuleError';
  }
}

// âŒ MAUVAIS : Erreur generique
throw new Error('Something went wrong');
```

**Pratique ERR-LS-02 : Jamais de panic silencieux**

Toute erreur doit etre explicite et journalisee.

```typescript
// âœ… BON : Erreur explicite et journalisee
async arbitrate(request: ResourceRequest): Promise<ArbitrationDecision> {
  try {
    const context = await this.contextProvider.getContext(request.entityId);
    return this.evaluate(request, context);
  } catch (error) {
    // Journalisation de l'erreur
    this.auditJournal.logError({
      operation: 'arbitrate',
      request: request,
      error: error
    });
    
    // Re-throw avec contexte
    throw new ArbitrationError('Echec d\'arbitrage', request, error);
  }
}

// âŒ MAUVAIS : Erreur ignoree
async arbitrate(request: ResourceRequest): Promise<ArbitrationDecision | null> {
  try {
    return this.evaluate(request, context);
  } catch (error) {
    return null; // âŒ Erreur masquee
  }
}
```

### 6.2 Validation

**Pratique VAL-LS-01 : Validation precoce**

Valider structurellement avant tout traitement.

```typescript
// âœ… BON : Validation precoce
class RequestReceiver {
  receive(request: ResourceRequest): ValidatedRequest {
    // Validation structurelle d'abord
    this.validateStructure(request);
    this.validateRequiredFields(request);
    this.validateTypes(request);
    this.validateEntityExists(request.entityId);
    
    // Puis traitement
    return this.createValidatedRequest(request);
  }
  
  private validateStructure(request: ResourceRequest): void {
    if (!request) {
      throw new ValidationError('Requete nulle');
    }
    if (!request.entityId) {
      throw new ValidationError('entityId manquant');
    }
    if (!request.resourceType) {
      throw new ValidationError('resourceType manquant');
    }
  }
}

// âŒ MAUVAIS : Validation tardive
receive(request: ResourceRequest) {
  const processed = this.process(request); // Traitement d'abord
  this.validate(processed); // Validation apres (trop tard)
}
```

### 6.3 Logging

**Pratique LOG-LS-01 : Logging structure**

Utiliser un logging structure avec contexte complet.

```typescript
// âœ… BON : Logging structure
this.logger.info('Decision d\'arbitrage generee', {
  decision_id: decision.id,
  entity_id: request.entityId,
  resource_type: request.resourceType,
  decision_type: decision.type,
  priority_applied: decision.priorityApplied,
  degradation_level: context.degradationLevel,
  rules_applied: decision.rulesApplied.map(r => r.id),
  timestamp: Date.now()
});

// âŒ MAUVAIS : Logging non structure
console.log('Decision: ' + decision.id);
```

**Pratique LOG-LS-02 : Pas de donnees sensibles**

Ne jamais logger de donnees sensibles.

```typescript
// âœ… BON : Pas de donnees sensibles
this.logger.info('Contexte assemble', {
  entity_id: context.entityId,
  degradation_level: context.degradationLevel,
  quota_usage_percentage: context.quotaUsagePercent
  // Pas de tokens, credentials, etc.
});

// âŒ MAUVAIS : Donnees sensibles loggees
this.logger.info('Contexte', context); // Peut contenir des secrets
```

### 6.4 Tests

**Pratique TEST-LS-01 : Tests de determinisme**

Tester systematiquement le determinisme des decisions.

```typescript
describe('ArbitrationEngine', () => {
  it('should be deterministic (same inputs = same output)', async () => {
    const engine = new ArbitrationEngine(/* deps */);
    const request = createTestRequest();
    const context = createTestContext();
    
    // Executions multiples
    const result1 = await engine.evaluate(request, context);
    const result2 = await engine.evaluate(request, context);
    const result3 = await engine.evaluate(request, context);
    
    // Verification : meme resultat
    expect(result1).toEqual(result2);
    expect(result2).toEqual(result3);
  });
  
  it('should not depend on execution order', async () => {
    const engine = new ArbitrationEngine(/* deps */);
    const requests = [createTestRequest1(), createTestRequest2()];
    const context = createTestContext();
    
    // Ordre 1
    const [r1a, r2a] = await Promise.all([
      engine.evaluate(requests[0], context),
      engine.evaluate(requests[1], context)
    ]);
    
    // Ordre inverse
    const [r2b, r1b] = await Promise.all([
      engine.evaluate(requests[1], context),
      engine.evaluate(requests[0], context)
    ]);
    
    // Verification : memes resultats independamment de l'ordre
    expect(r1a).toEqual(r1b);
    expect(r2a).toEqual(r2b);
  });
});
```

**Pratique TEST-LS-02 : Tests d'invariants**

Tester systematiquement le respect des invariants.

```typescript
describe('Invariants', () => {
  describe('INV-LS-1: Arbitrage sans execution', () => {
    it('should not have any system-level execution methods', () => {
      const engine = new ArbitrationEngine(/* deps */);
      
      // Verification : pas de methodes d'execution
      expect(engine).not.toHaveProperty('allocateMemory');
      expect(engine).not.toHaveProperty('scheduleThread');
      expect(engine).not.toHaveProperty('executeDecision');
    });
  });
  
  describe('INV-LS-4: Decisions deterministes', () => {
    it('should not use random functions', () => {
      // Code review statique ou AST analysis
      const sourceCode = getSourceCode(ArbitrationEngine);
      
      expect(sourceCode).not.toContain('Math.random');
      expect(sourceCode).not.toContain('crypto.random');
    });
  });
  
  describe('INV-LS-8: Validation StrongFather', () => {
    it('should always submit to StrongFather', async () => {
      const mockStrongFather = createMockStrongFather();
      const preparer = new ValidationPreparer(mockStrongFather);
      const decision = createTestDecision();
      
      await preparer.submit(decision);
      
      // Verification : soumission effectuee
      expect(mockStrongFather.submit).toHaveBeenCalledWith(
        expect.objectContaining({ decisionId: decision.id })
      );
    });
  });
});
```

---

## 7. Pieges a eviter

### 7.1 Piege : Mesure directe des ressources

**Probleme :** Mesurer directement CPU, memoire, IO pour prendre des decisions.

**Pourquoi c'est mal :** Violerait INV-LS-2 (etat systeme abstrait) et INV-LS-7 (separation Kernel).

**Solution :** Utiliser l'etat systeme abstrait fourni par le Kernel.

```typescript
// âŒ MAUVAIS : Mesure directe
class ResourceMonitor {
  getCurrentLoad(): number {
    return os.loadavg()[0]; // âŒ Violation INV-LS-2 et INV-LS-7
  }
  
  getMemoryUsage(): number {
    return process.memoryUsage().heapUsed; // âŒ Violation
  }
}

// âœ… BON : Etat abstrait du Kernel
class SystemStateReader {
  async getLoadLevel(): Promise<LoadLevel> {
    const state = await this.kernelAdapter.getSystemState();
    return state.loadLevel; // Abstrait, certifie par le Kernel
  }
}
```

### 7.2 Piege : Decision auto-appliquee

**Probleme :** Appliquer une decision sans validation de StrongFather.

**Pourquoi c'est mal :** Violerait INV-LS-8 (validation StrongFather).

**Solution :** Toujours soumettre a StrongFather, meme en mode degrade.

```typescript
// âŒ MAUVAIS : Decision auto-appliquee
class ArbitrationService {
  async process(request: ResourceRequest): Promise<void> {
    const decision = await this.evaluate(request);
    this.applyDecision(decision); // âŒ Violation INV-LS-8
  }
}

// âœ… BON : Soumission obligatoire
class ArbitrationService {
  async process(request: ResourceRequest): Promise<ValidationResult> {
    const decision = await this.evaluate(request);
    return this.validationPreparer.submit(decision); // StrongFather valide
  }
}
```

### 7.3 Piege : Regles implicites

**Probleme :** Avoir des comportements par defaut non documentes.

**Pourquoi c'est mal :** Violerait INV-LS-5 (regles explicites).

**Solution :** Toute regle doit etre explicitement declaree.

```typescript
// âŒ MAUVAIS : Regle implicite
class QuotaEvaluator {
  evaluate(request: ResourceRequest, quotas: Quota[]): boolean {
    const quota = quotas.find(q => q.resourceType === request.resourceType);
    
    if (!quota) {
      return true; // âŒ Regle implicite : "pas de quota = autorise"
    }
    
    return quota.remaining >= request.amount;
  }
}

// âœ… BON : Regle explicite
class QuotaEvaluator {
  evaluate(request: ResourceRequest, quotas: Quota[]): QuotaResult {
    const quota = quotas.find(q => q.resourceType === request.resourceType);
    
    if (!quota) {
      // Regle explicite : NO_QUOTA_DEFINED
      return {
        allowed: false,
        reason: 'NO_QUOTA_DEFINED',
        rule: 'RULE-QUOTA-01: Tout type de ressource doit avoir un quota defini'
      };
    }
    
    if (quota.remaining < request.amount) {
      return {
        allowed: false,
        reason: 'QUOTA_EXCEEDED',
        rule: 'RULE-QUOTA-02: Demande ne peut exceder le quota restant'
      };
    }
    
    return {
      allowed: true,
      reason: 'QUOTA_OK',
      rule: 'RULE-QUOTA-03: Demande dans les limites du quota'
    };
  }
}
```

### 7.4 Piege : Composante aleatoire

**Probleme :** Utiliser des composantes aleatoires dans les decisions.

**Pourquoi c'est mal :** Violerait INV-LS-4 (decisions deterministes).

**Solution :** Calculs purs et deterministes uniquement.

```typescript
// âŒ MAUVAIS : Composante aleatoire
class ConflictResolver {
  resolve(conflicts: RuleConflict[]): ArbitrationDecision {
    // âŒ Violation INV-LS-4 : aleatoire
    const randomIndex = Math.floor(Math.random() * conflicts.length);
    return conflicts[randomIndex].resolution;
  }
}

// âœ… BON : Resolution deterministe
class ConflictResolver {
  resolve(conflicts: RuleConflict[]): ArbitrationDecision {
    // Resolution par priorite (deterministe)
    const sorted = conflicts.sort((a, b) => b.rule.priority - a.rule.priority);
    return sorted[0].resolution;
  }
}
```

### 7.5 Piege : Execution technique

**Probleme :** Implementer des actions techniques dans LogisticsSteward.

**Pourquoi c'est mal :** Violerait INV-LS-1 (arbitrage sans execution).

**Solution :** Generer des decisions, le Kernel execute.

```typescript
// âŒ MAUVAIS : Execution technique
class ResourceAllocator {
  allocate(decision: ArbitrationDecision): void {
    // âŒ Violations multiples
    process.setMaxMemory(decision.memoryLimit);
    os.setPriority(decision.processPriority);
    this.threadPool.resize(decision.threadCount);
  }
}

// âœ… BON : Decision pour le Kernel
class DecisionGenerator {
  generate(evaluation: RuleEvaluation): ArbitrationDecision {
    return {
      id: this.idGenerator.generate(),
      type: evaluation.decisionType,
      memoryLimit: evaluation.recommendedMemoryLimit,
      processPriority: evaluation.recommendedPriority,
      threadCount: evaluation.recommendedThreads,
      // Le Kernel executera cette decision apres validation
      forKernel: true
    };
  }
}
```

---

## 8. Guidelines de test

### 8.1 Structure des tests

**Organisation recommandee :**

```
tests/
â”œâ”€â”€ unit/                      # Tests unitaires
â”‚   â”œâ”€â”€ reception-layer/
â”‚   â”œâ”€â”€ context-layer/
â”‚   â”œâ”€â”€ evaluation-layer/
â”‚   â””â”€â”€ decision-layer/
â”œâ”€â”€ integration/               # Tests d'integration
â”‚   â”œâ”€â”€ flows/
â”‚   â””â”€â”€ contracts/
â”œâ”€â”€ contract/                  # Tests de contrats
â”‚   â”œâ”€â”€ invariants/
â”‚   â””â”€â”€ guarantees/
â”œâ”€â”€ determinism/               # Tests de determinisme
â”‚   â””â”€â”€ reproducibility/
â””â”€â”€ degradation/               # Tests de modes degrades
    â”œâ”€â”€ offline/
    â””â”€â”€ recovery/
```

### 8.2 Tests d'invariants

**Regle TEST-INV-01 : Tests systematiques**

Tous les invariants doivent etre testes systematiquement.

```typescript
describe('Invariants LogisticsSteward', () => {
  describe('INV-LS-1: Arbitrage sans execution', () => {
    it('should not have execution capabilities', () => {
      const components = getAllComponents();
      components.forEach(component => {
        expect(component).not.toHaveMethod('execute');
        expect(component).not.toHaveMethod('allocate');
        expect(component).not.toHaveMethod('schedule');
      });
    });
  });
  
  describe('INV-LS-3: Lecture seule du systeme', () => {
    it('should not modify system state', async () => {
      const mockKernel = createMockKernel();
      const stateReader = new SystemStateReader(mockKernel);
      
      const initialState = await mockKernel.getSystemState();
      await stateReader.readState();
      const finalState = await mockKernel.getSystemState();
      
      expect(finalState).toEqual(initialState);
    });
  });
  
  describe('INV-LS-6: Tracabilite complete', () => {
    it('should journal all decisions', async () => {
      const mockJournal = createMockJournal();
      const engine = new ArbitrationEngine(/* deps including mockJournal */);
      
      const request = createTestRequest();
      await engine.process(request);
      
      expect(mockJournal.entries.length).toBeGreaterThan(0);
      expect(mockJournal.entries).toContainEntry(
        expect.objectContaining({ type: 'DECISION' })
      );
    });
  });
});
```

### 8.3 Tests de garanties

**Regle TEST-GUAR-01 : Tests des garanties**

Toutes les garanties doivent etre testees.

```typescript
describe('Garanties LogisticsSteward', () => {
  describe('G-LS-GOV-3: Gouvernance predictible', () => {
    it('should produce identical decisions for identical inputs', async () => {
      const engine = new ArbitrationEngine(/* deps */);
      const inputs = { request: createTestRequest(), context: createTestContext() };
      
      // 100 executions
      const results = await Promise.all(
        Array(100).fill(null).map(() => 
          engine.evaluate(inputs.request, inputs.context)
        )
      );
      
      // Toutes identiques
      const first = results[0];
      results.forEach(result => expect(result).toEqual(first));
    });
  });
  
  describe('G-LS-PROT-1: Protection contre la saturation', () => {
    it('should enforce quotas', async () => {
      const evaluator = new QuotaEvaluator();
      const quota = { resourceType: 'MEMORY', total: 100, used: 90, remaining: 10 };
      
      // Demande depassant le quota
      const request = createRequest({ amount: 20 });
      const result = evaluator.evaluate(request, [quota]);
      
      expect(result.allowed).toBe(false);
      expect(result.reason).toBe('QUOTA_EXCEEDED');
    });
  });
  
  describe('G-LS-STAB-2: Degradation progressive', () => {
    it('should not skip degradation levels', async () => {
      const manager = new DegradationManager();
      const transitions: DegradationTransition[] = [];
      
      // Transition D0 -> D3
      let current = { level: 0 };
      const target = { level: 3 };
      
      while (current.level !== target.level) {
        const transition = manager.transitionTo(current, target);
        transitions.push(transition);
        current = transition.to;
      }
      
      // Verification : pas de saut > 1
      transitions.forEach((t, i) => {
        if (i > 0) {
          const prevTo = transitions[i - 1].to.level;
          expect(Math.abs(t.from.level - prevTo)).toBe(0);
        }
        expect(Math.abs(t.to.level - t.from.level)).toBeLessThanOrEqual(1);
      });
    });
  });
});
```

---

## 9. Exemples d'implementation

### 9.1 Exemple : ArbitrationService complet

```typescript
/**
 * Service d'arbitrage principal.
 * 
 * @contract IArbitrationService v1.0
 * @layer Orchestration
 */
class ArbitrationService implements IArbitrationService {
  constructor(
    private requestReceiver: IRequestReceiver,
    private contextAssembler: IContextAssembler,
    private arbitrationEngine: IArbitrationEngine,
    private validationPreparer: IValidationPreparer,
    private auditJournal: IAuditJournal,
    private logger: ILogger
  ) {}
  
  /**
   * Traite une demande de ressource complete.
   * 
   * @param request - La demande de ressource
   * @returns Le resultat de l'arbitrage
   * 
   * @invariant INV-LS-1 : Pas d'execution
   * @invariant INV-LS-6 : Tracabilite complete
   * @invariant INV-LS-8 : Validation StrongFather
   */
  async process(request: ResourceRequest): Promise<ArbitrationResult> {
    this.logger.info('Demande recue', { request_id: request.id });
    
    try {
      // 1. Reception et validation structurelle
      const validatedRequest = await this.requestReceiver.receive(request);
      
      // 2. Assemblage du contexte
      const context = await this.contextAssembler.assemble(validatedRequest);
      
      // 3. Evaluation et generation de decision
      const decision = await this.arbitrationEngine.evaluate(
        validatedRequest,
        context
      );
      
      // 4. Soumission a StrongFather
      const validationResult = await this.validationPreparer.submit(decision);
      
      // 5. Journalisation complete
      await this.auditJournal.log({
        type: 'ARBITRATION_COMPLETE',
        request: validatedRequest,
        context: context,
        decision: decision,
        validation: validationResult
      });
      
      this.logger.info('Arbitrage termine', {
        request_id: request.id,
        decision_id: decision.id,
        status: validationResult.status
      });
      
      return {
        decision: decision,
        validation: validationResult
      };
    } catch (error) {
      this.logger.error('Echec arbitrage', {
        request_id: request.id,
        error: error.message
      });
      
      await this.auditJournal.logError({
        type: 'ARBITRATION_ERROR',
        request: request,
        error: error
      });
      
      throw new ArbitrationError('Echec du processus d\'arbitrage', error);
    }
  }
}
```

### 9.2 Exemple : QuotaEvaluator complet

```typescript
/**
 * Evaluateur de quotas.
 * 
 * @contract IQuotaEvaluator v1.0
 * @layer Evaluation
 * 
 * @invariant INV-LS-4 : Evaluation deterministe
 * @invariant INV-LS-5 : Regles explicites
 */
class QuotaEvaluator implements IQuotaEvaluator {
  /** Regles explicites d'evaluation de quota */
  private static readonly RULES = {
    NO_QUOTA: 'RULE-QUOTA-01: Type de ressource sans quota defini',
    EXCEEDED: 'RULE-QUOTA-02: Demande excede le quota restant',
    ALLOWED: 'RULE-QUOTA-03: Demande dans les limites du quota',
    PARTIAL: 'RULE-QUOTA-04: Demande partiellement satisfaisable'
  };
  
  /**
   * Evalue une demande par rapport aux quotas de l'entite.
   * 
   * @param request - La demande de ressource
   * @param entityQuotas - Les quotas de l'entite
   * @returns Le resultat de l'evaluation
   */
  evaluate(
    request: ResourceRequest,
    entityQuotas: ReadonlyArray<Quota>
  ): QuotaEvaluationResult {
    // Recherche du quota applicable
    const quota = entityQuotas.find(
      q => q.resourceType === request.resourceType
    );
    
    // Cas 1 : Pas de quota defini
    if (!quota) {
      return this.createResult('DENIED', 'NO_QUOTA_DEFINED', {
        rule: QuotaEvaluator.RULES.NO_QUOTA,
        requested: request.amount,
        available: 0
      });
    }
    
    // Cas 2 : Quota suffisant
    if (quota.remaining >= request.amount) {
      return this.createResult('ALLOWED', 'QUOTA_OK', {
        rule: QuotaEvaluator.RULES.ALLOWED,
        requested: request.amount,
        available: quota.remaining,
        afterRequest: quota.remaining - request.amount
      });
    }
    
    // Cas 3 : Quota insuffisant mais partiel possible
    if (quota.remaining > 0) {
      return this.createResult('PARTIAL', 'QUOTA_PARTIAL', {
        rule: QuotaEvaluator.RULES.PARTIAL,
        requested: request.amount,
        available: quota.remaining,
        satisfiable: quota.remaining
      });
    }
    
    // Cas 4 : Quota epuise
    return this.createResult('DENIED', 'QUOTA_EXCEEDED', {
      rule: QuotaEvaluator.RULES.EXCEEDED,
      requested: request.amount,
      available: 0
    });
  }
  
  private createResult(
    status: QuotaStatus,
    reason: string,
    details: QuotaDetails
  ): QuotaEvaluationResult {
    return {
      status,
      reason,
      details,
      timestamp: Date.now()
    };
  }
}
```

---

## 10. Checklist d'implementation

### 10.1 Checklist de conformite

Avant de considerer une implementation comme complete, verifier :

**Contrats :**
- [ ] Tous les contrats sont implementes
- [ ] Tous les tests de contrat passent
- [ ] Documentation des contrats a jour

**Invariants :**
- [ ] INV-LS-1 : Aucune execution technique
- [ ] INV-LS-2 : Etat systeme abstrait uniquement
- [ ] INV-LS-3 : Lecture seule du systeme
- [ ] INV-LS-4 : Decisions deterministes
- [ ] INV-LS-5 : Regles explicites
- [ ] INV-LS-6 : Tracabilite complete
- [ ] INV-LS-7 : Separation Kernel
- [ ] INV-LS-8 : Validation StrongFather
- [ ] INV-LS-9 : Degradation controlee
- [ ] INV-LS-10 : Resilience locale

**Architecture :**
- [ ] Structure en 4 couches respectee
- [ ] Isolation des couches respectee
- [ ] Interfaces explicites definies

**Tests :**
- [ ] Couverture de code > 80%
- [ ] Tests unitaires pour tous les composants
- [ ] Tests d'integration pour tous les flux
- [ ] Tests de determinisme
- [ ] Tests de modes degrades

**Documentation :**
- [ ] Code documente
- [ ] Interfaces documentees
- [ ] Exemples fournis

### 10.2 Checklist de qualite

**Code :**
- [ ] Code lisible et maintenable
- [ ] Nommage explicite
- [ ] Pas de code mort
- [ ] Pas de duplication

**Performance :**
- [ ] Cout proportionnel au hardware (LOI-5)
- [ ] Pas de fuite memoire
- [ ] Metriques de performance conformes

**Securite :**
- [ ] Pas de secrets dans le code
- [ ] Validation des entrees
- [ ] Gestion securisee des erreurs

---

## 11. Conformite MSCM/MIP

### 11.1 Obligation de balisage MSCM

Tout code implemente pour LogisticsSteward DOIT etre balise selon le protocole MSCM v1.

**Reference :** [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md)

**Obligations minimales :**
- Chaque bloc fonctionnel DOIT avoir un identifiant unique (`@id`)
- Le role semantique DOIT etre explicite (`@role`)
- La couche architecturale DOIT etre declaree (`@layer`)
- Une description humaine DOIT accompagner chaque bloc (`@human`)

### 11.2 Integration MIP

Apres implementation, l'index MIP DOIT etre regenere pour :
- Valider l'integrite des blocs MSCM
- Mettre a jour le graphe de dependances
- Verifier la coherence hierarchique

### 11.3 Check-list MSCM

Avant toute livraison, verifier :
- [ ] Tous les blocs critiques sont balises MSCM
- [ ] Les identifiants sont uniques globalement
- [ ] Les couches (layer) sont coherentes avec l'architecture
- [ ] L'index MIP peut etre regenere sans erreur

---

## 12. Statut contractuel

Ce document est **informatif, non contractuel, et de statut GUIDELINES**. Il fournit des recommandations pour l'implementation de reference de LogisticsSteward, mais n'etablit pas de contraintes contractuelles.

Les contrats et invariants restent la source de verite. Ces guidelines sont des recommandations pour faciliter une implementation conforme.

---

## 13. Documents associes

- [LogisticsSteward - Index de Navigation](../_index.md)
- [LogisticsSteward - Documentation Fondatrice](../foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md)
- [LogisticsSteward - Architecture & Flows](../architecture/LogisticsSteward%20-%20Architecture%20%26%20Flows.md)
- [LogisticsSteward - Invariants & Guarantees](../contracts/governance/LogisticsSteward%20-%20Invariants%20%26%20Guarantees.md)
- [LogisticsSteward - Quota Definition Contract](../contracts/resources/LogisticsSteward%20-%20Quota%20Definition%20Contract.md)
- [LogisticsSteward - Priority Management Contract](../contracts/resources/LogisticsSteward%20-%20Priority%20Management%20Contract.md)
- [LogisticsSteward - Degradation Strategy Contract](../contracts/degradation/LogisticsSteward%20-%20Degradation%20Strategy%20Contract.md)
- [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//miyukini-webway-system//reference//_index.md)

---

**Version :** 1.0.0  
**Date :** 2026-01-28  
**Statut :** GUIDELINES â€” Informatif  
**Dependances :** 
- Documentation Fondatrice v1.0.0
- Architecture & Flows v1.0.0
- Invariants & Guarantees v1.0.0
- Tous les contrats v1.0.0
- MIP v1 MSCM Index Protocol

