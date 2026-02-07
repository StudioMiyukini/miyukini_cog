# BondingBrother - Reference Implementation Guidelines

## 1. Contexte

Ce document fournit des guidelines pour l'implémentation de référence de Bonding Brother. Il guide les développeurs dans la création d'une implémentation conforme aux contrats et invariants de Bonding Brother, en fournissant des recommandations pratiques, des patterns, et des bonnes pratiques.

Ce document complète l'[Architecture & Flows](../architecture/BondingBrother%20-%20Architecture%20%26%20Flows.md) et les [Invariants & Guarantees](../contracts/governance/BondingBrother%20-%20Invariants%20%26%20Guarantees.md) en fournissant des directives concrètes d'implémentation.

L'implémentation doit respecter les [Lois d'Autonomie Système](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) : aucune dépendance externe critique (**LOI-1**), fonctionnement en mode offline (**LOI-2**), état local souverain (**LOI-3**), et coût proportionnel au hardware (**LOI-5**).

**Navigation :** [Index BondingBrother](../_index.md)

## 2. Portée / Scope

Ce document couvre :
- Les principes d'implémentation
- Les patterns recommandés
- Les structures de code recommandées
- Les bonnes pratiques
- Les pièges à éviter
- Les exemples de code
- Les guidelines de test

Ce document **ne couvre pas** :
- Les spécifications contractuelles (voir les contrats spécifiques)
- Les détails d'implémentation spécifiques à une technologie
- Les optimisations spécifiques à un environnement

---

## 3. Principes d'implémentation

### 3.1 Conformité aux contrats

**Principe IMPL-01 : Contrats d'abord**

L'implémentation doit respecter strictement tous les contrats de Bonding Brother. Aucun compromis n'est autorisé.

**Implications :**
- Tous les contrats sont implémentés intégralement
- Aucune violation d'invariant
- Tous les tests de contrat passent
- Documentation à jour

### 3.2 Simplicité et clarté

**Principe IMPL-02 : Code simple**

Le code doit être simple, clair, et facile à comprendre. La complexité n'est justifiée que si nécessaire pour respecter les contrats.

**Implications :**
- Code lisible
- Nommage explicite
- Structure claire
- Documentation inline

### 3.3 Testabilité

**Principe IMPL-03 : Testable par design**

L'implémentation doit être testable. Chaque composant doit pouvoir être testé isolément.

**Implications :**
- Injection de dépendances
- Interfaces claires
- Pas de couplage fort
- Mocks possibles

### 3.4 Observabilité

**Principe IMPL-04 : Observable**

L'implémentation doit être observable : logs, métriques, traces.

**Implications :**
- Logging structuré
- Métriques exposées
- Traces distribuées
- Health checks

---

## 4. Structure de code recommandée

### 4.1 Organisation par couches

**Structure recommandée :**
```
bonding-brother/
├── product-layer/          # Couche Produit
│   ├── gateway/
│   ├── receiver/
│   ├── emitter/
│   └── dispatcher/
├── translation-layer/      # Couche Traduction
│   ├── intent-translator/
│   ├── response-translator/
│   ├── vocabulary-mapper/
│   └── context-enricher/
├── mediation-layer/        # Couche Médiation
│   ├── orchestrator/
│   ├── router/
│   ├── filter-engine/
│   ├── journal-writer/
│   └── offline-buffer/
├── authority-layer/        # Couche Autorité
│   ├── kind-mother-adapter/
│   ├── strong-father-adapter/
│   ├── response-handler/
│   └── deferred-manager/
├── common/                 # Composants transversaux
│   ├── config/
│   ├── metrics/
│   ├── health/
│   └── errors/
└── contracts/              # Définitions de contrats
    ├── interfaces/
    └── types/
```

### 4.2 Isolation des couches

**Règle IMPL-STRUCT-01 : Isolation stricte**

Chaque couche est isolée et ne dépend que de ses interfaces, pas de l'implémentation.

**Exemple :**
```typescript
// ✅ BON : Couche Traduction dépend de l'interface, pas de l'implémentation
interface IIntentReceiver {
  receive(intent: Intent): Promise<Intent>;
}

class IntentTranslator {
  constructor(private receiver: IIntentReceiver) {}
  // ...
}

// ❌ MAUVAIS : Couplage direct à l'implémentation
class IntentTranslator {
  constructor(private receiver: IntentReceiverImpl) {}
  // ...
}
```

### 4.3 Interfaces explicites

**Règle IMPL-STRUCT-02 : Interfaces explicites**

Toutes les interfaces entre composants sont explicites et documentées.

**Exemple :**
```typescript
/**
 * Interface pour la réception d'intentions depuis les produits.
 * 
 * @contract IIntentSubmission
 * @version 1.0
 */
interface IIntentReceiver {
  /**
   * Reçoit une intention d'un produit.
   * 
   * @param intent - L'intention à recevoir
   * @returns L'intention reçue (validée structurellement)
   * @throws ValidationError si l'intention est invalide
   */
  receive(intent: Intent): Promise<ValidatedIntent>;
}
```

---

## 5. Patterns recommandés

### 5.1 Pattern : Traduction pure

**Contexte :** La traduction doit être pure (sans effet de bord).

**Pattern :**
```typescript
/**
 * Traducteur d'intention. Fonction pure sans effet de bord.
 */
class IntentTranslator {
  /**
   * Traduit une intention en demande pour l'autorité.
   * 
   * @param intent - L'intention à traduire
   * @param context - Le contexte de traduction
   * @returns La demande traduite
   * 
   * @invariant INV-NAT-02 : Traducteur, pas exécuteur
   * @invariant Pure function : pas d'effet de bord
   */
  translate(intent: ValidatedIntent, context: TranslationContext): Demand {
    // Traduction pure : pas de modification d'état
    // Pas d'appel à des services externes
    // Pas de journalisation (fait ailleurs)
    return this.doTranslate(intent, context);
  }
  
  private doTranslate(intent: ValidatedIntent, context: TranslationContext): Demand {
    // Implémentation de la traduction
  }
}
```

**Avantages :**
- Testable facilement
- Pas d'effet de bord
- Respecte INV-NAT-02

### 5.2 Pattern : Délégation aux autorités

**Contexte :** Toute décision est déléguée aux autorités.

**Pattern :**
```typescript
/**
 * Routeur vers les autorités. Ne décide jamais, délègue toujours.
 */
class AuthorityRouter {
  constructor(
    private kindMotherAdapter: IKindMotherAdapter,
    private strongFatherAdapter: IStrongFatherAdapter
  ) {}
  
  /**
   * Route une demande vers l'autorité appropriée.
   * 
   * @param demand - La demande à router
   * @returns La réponse de l'autorité
   * 
   * @invariant INV-NEG-01 : Jamais de décision
   * @invariant INV-NEG-04 : Jamais de contournement
   */
  async route(demand: Demand): Promise<AuthorityResponse> {
    // Décision technique uniquement : quelle autorité ?
    const authority = this.selectAuthority(demand);
    
    // Délégation : pas de décision métier
    if (authority === 'KIND_MOTHER') {
      return await this.kindMotherAdapter.send(demand);
    } else {
      return await this.strongFatherAdapter.send(demand);
    }
  }
  
  /**
   * Sélection technique de l'autorité (pas de décision métier).
   */
  private selectAuthority(demand: Demand): 'KIND_MOTHER' | 'STRONG_FATHER' {
    // Sélection basée sur le type de demande, pas sur le contenu métier
    return demand.type.startsWith('DATA_') ? 'KIND_MOTHER' : 'STRONG_FATHER';
  }
}
```

**Avantages :**
- Respecte INV-NEG-01
- Délégation claire
- Testable

### 5.3 Pattern : Journalisation systématique

**Contexte :** Toute interaction doit être journalisée.

**Pattern :**
```typescript
/**
 * Journaliseur. Journalise systématiquement toutes les interactions.
 */
class JournalWriter {
  /**
   * Journalise une interaction.
   * 
   * @param entry - L'entrée à journaliser
   * 
   * @invariant INV-FLUX-02 : Journalisation systématique
   */
  async journalize(entry: JournalEntry): Promise<void> {
    // Journalisation asynchrone (ne bloque pas le flux)
    // Mais garantie de persistance
    await this.persist(entry);
  }
  
  private async persist(entry: JournalEntry): Promise<void> {
    // Persistance dans le journal (immutable)
    // Pas de modification possible
    await this.storage.append(entry);
  }
}
```

**Avantages :**
- Respecte INV-FLUX-02
- Traçabilité complète
- Immutabilité

### 5.4 Pattern : Filtrage par règles

**Contexte :** Le filtrage applique des règles, ne décide pas.

**Pattern :**
```typescript
/**
 * Moteur de filtrage. Applique des règles, ne décide pas.
 */
class FilterEngine {
  constructor(private rules: IFilterRules) {}
  
  /**
   * Filtre une demande selon les règles.
   * 
   * @param demand - La demande à filtrer
   * @returns La demande filtrée (ou rejetée)
   * 
   * @invariant INV-NEG-03 : Jamais de création de règle
   * @invariant Applique des règles, ne décide pas
   */
  async filter(demand: Demand): Promise<FilteredDemand | Rejection> {
    // Application des règles (définies par une autorité)
    // Pas de décision arbitraire
    for (const rule of this.rules.getApplicableRules(demand)) {
      const result = await rule.evaluate(demand);
      if (result === 'REJECT') {
        return { type: 'REJECTION', reason: rule.getReason() };
      }
    }
    
    return { type: 'FILTERED', demand: this.applyFilters(demand) };
  }
}
```

**Avantages :**
- Respecte INV-NEG-03
- Règles externes
- Testable

---

## 6. Bonnes pratiques

### 6.1 Gestion des erreurs

**Pratique ERR-01 : Erreurs typées**

Utiliser des types d'erreur explicites et conformes au [Error & Rejection Model](../contracts/error/BondingBrother%20-%20Error%20%26%20Rejection%20Model.md).

**Exemple :**
```typescript
// ✅ BON : Erreur typée
class ValidationError extends Error {
  constructor(
    public code: 'VALIDATION_ERROR',
    public field: string,
    public reason: string
  ) {
    super(`Validation failed for ${field}: ${reason}`);
  }
}

// ❌ MAUVAIS : Erreur générique
throw new Error('Something went wrong');
```

**Pratique ERR-02 : Propagation fidèle**

Les erreurs des autorités sont propagées fidèlement, sans modification.

**Exemple :**
```typescript
// ✅ BON : Propagation fidèle
try {
  const response = await this.authorityAdapter.send(demand);
  return response;
} catch (error) {
  // Propagation fidèle de l'erreur de l'autorité
  throw new AuthorityError(error.code, error.message, error.context);
}

// ❌ MAUVAIS : Modification de l'erreur
catch (error) {
  throw new Error('Internal error'); // Perte d'information
}
```

### 6.2 Validation

**Pratique VAL-01 : Validation précoce**

Valider structurellement avant tout traitement.

**Exemple :**
```typescript
// ✅ BON : Validation précoce
class IntentReceiver {
  async receive(intent: Intent): Promise<ValidatedIntent> {
    // Validation structurelle d'abord
    this.validateStructure(intent);
    this.validateTypes(intent);
    this.validateRequiredFields(intent);
    
    // Puis traitement
    return this.process(intent);
  }
}

// ❌ MAUVAIS : Validation tardive
async receive(intent: Intent) {
  // Traitement d'abord...
  const processed = this.process(intent);
  // Validation après (trop tard)
  this.validate(processed);
}
```

### 6.3 Logging

**Pratique LOG-01 : Logging structuré**

Utiliser un logging structuré avec contexte.

**Exemple :**
```typescript
// ✅ BON : Logging structuré
logger.info('Intention reçue', {
  intention_id: intent.id,
  produit_id: intent.produit_id,
  type: intent.type,
  timestamp: Date.now()
});

// ❌ MAUVAIS : Logging non structuré
console.log('Intention reçue: ' + intent.id);
```

**Pratique LOG-02 : Pas de secrets**

Ne jamais logger de secrets (mots de passe, tokens).

**Exemple :**
```typescript
// ✅ BON : Pas de secrets
logger.info('Contexte reçu', {
  produit_id: context.produit_id,
  user_id: context.user_id,
  // token: context.token // ❌ JAMAIS
});

// ❌ MAUVAIS : Secret loggé
logger.info('Contexte', context); // Peut contenir des secrets
```

### 6.4 Tests

**Pratique TEST-01 : Tests d'invariants**

Tester systématiquement les invariants.

**Exemple :**
```typescript
describe('IntentTranslator', () => {
  it('should be pure (no side effects)', async () => {
    const translator = new IntentTranslator();
    const intent = createTestIntent();
    
    // Premier appel
    const result1 = await translator.translate(intent, context);
    
    // Deuxième appel (même entrée = même sortie)
    const result2 = await translator.translate(intent, context);
    
    expect(result1).toEqual(result2);
    // Vérification : pas d'effet de bord
  });
  
  it('should not make decisions', async () => {
    const translator = new IntentTranslator();
    const intent = createTestIntent();
    
    const demand = await translator.translate(intent, context);
    
    // Vérification : pas de décision métier dans la demande
    expect(demand.decision).toBeUndefined();
  });
});
```

**Pratique TEST-02 : Tests de contrats**

Tester la conformité aux contrats.

**Exemple :**
```typescript
describe('ProductGateway', () => {
  it('should respect IIntentSubmission contract', async () => {
    const gateway = new ProductGateway();
    
    // Test du contrat
    const intent = createValidIntent();
    const result = await gateway.submitIntent(intent);
    
    // Vérification de la conformité au contrat
    expect(result).toMatchContract(IIntentSubmission);
  });
});
```

---

## 7. Pièges à éviter

### 7.1 Piège : Cache de données métier

**Problème :** Mettre en cache des données métier pour améliorer les performances.

**Pourquoi c'est mal :** Violerait INV-NEG-02 (jamais de stockage de vérité).

**Solution :** Cache uniquement de configuration, pas de données métier.

```typescript
// ❌ MAUVAIS : Cache de données métier
class ContentCache {
  private cache = new Map<string, Content>();
  
  async getContent(id: string): Promise<Content> {
    if (this.cache.has(id)) {
      return this.cache.get(id); // ❌ Violation INV-NEG-02
    }
    // ...
  }
}

// ✅ BON : Pas de cache de données métier
class ContentTranslator {
  // Pas de cache, toujours aller chercher chez l'autorité
  async translate(intent: Intent): Promise<Demand> {
    // Traduction pure, pas de cache
  }
}
```

### 7.2 Piège : Décision basée sur la performance

**Problème :** Prendre des décisions métier basées sur la performance.

**Pourquoi c'est mal :** Violerait INV-NEG-01 (jamais de décision).

**Solution :** Décisions techniques uniquement, pas de décisions métier.

```typescript
// ❌ MAUVAIS : Décision basée sur la performance
class AuthorityRouter {
  async route(demand: Demand): Promise<Response> {
    // Décision métier basée sur la performance ❌
    if (this.kindMother.isSlow()) {
      return this.strongFather.send(demand); // ❌ Violation INV-NEG-01
    }
    // ...
  }
}

// ✅ BON : Routage technique uniquement
class AuthorityRouter {
  async route(demand: Demand): Promise<Response> {
    // Décision technique uniquement
    const authority = this.selectAuthorityByType(demand.type);
    return await this.getAdapter(authority).send(demand);
  }
}
```

### 7.3 Piège : Saut d'étape pour performance

**Problème :** Sauter des étapes du flux pour améliorer les performances.

**Pourquoi c'est mal :** Violerait INV-FLUX-01 (séquence complète).

**Solution :** Optimiser chaque étape, mais ne jamais en sauter.

```typescript
// ❌ MAUVAIS : Saut d'étape
class MediationOrchestrator {
  async process(intent: Intent): Promise<Result> {
    // Saut de la journalisation pour performance ❌
    // this.journalize(intent); // ❌ Violation INV-FLUX-02
    
    const demand = await this.translate(intent);
    return await this.route(demand);
  }
}

// ✅ BON : Toutes les étapes, optimisées
class MediationOrchestrator {
  async process(intent: Intent): Promise<Result> {
    // Toutes les étapes, mais optimisées
    const validated = await this.validate(intent);
    const demand = await this.translate(validated);
    await this.journalize(validated); // Asynchrone mais présent
    const response = await this.route(demand);
    await this.journalize(response);
    return await this.emit(response);
  }
}
```

### 7.4 Piège : Modification de décision d'autorité

**Problème :** Modifier une décision d'autorité pour "corriger" ou "améliorer".

**Pourquoi c'est mal :** Violerait INV-NEG-05 (jamais de modification de décision).

**Solution :** Transmettre fidèlement, sans modification.

```typescript
// ❌ MAUVAIS : Modification de décision
class ResponseTranslator {
  translate(response: AuthorityResponse): Result {
    // Modification de la décision ❌
    if (response.status === 'REJECTED') {
      return { status: 'ACCEPTED' }; // ❌ Violation INV-NEG-05
    }
    // ...
  }
}

// ✅ BON : Transmission fidèle
class ResponseTranslator {
  translate(response: AuthorityResponse): Result {
    // Transmission fidèle de la décision
    return {
      status: response.status, // Fidèle
      data: this.filterData(response.data), // Filtrage OK
    };
  }
}
```

---

## 8. Guidelines de test

### 8.1 Structure des tests

**Organisation recommandée :**
```
tests/
├── unit/                   # Tests unitaires
│   ├── product-layer/
│   ├── translation-layer/
│   ├── mediation-layer/
│   └── authority-layer/
├── integration/           # Tests d'intégration
│   ├── flows/
│   └── contracts/
├── contract/               # Tests de contrats
│   ├── invariants/
│   └── guarantees/
└── e2e/                    # Tests end-to-end
    └── scenarios/
```

### 8.2 Tests d'invariants

**Règle TEST-INV-01 : Tests systématiques**

Tous les invariants doivent être testés systématiquement.

**Exemple :**
```typescript
describe('Invariants', () => {
  describe('INV-NAT-01: Médiateur, pas autorité', () => {
    it('should not have decision methods', () => {
      const components = getAllComponents();
      components.forEach(component => {
        expect(component).not.toHaveMethod('decide');
        expect(component).not.toHaveMethod('rule');
        expect(component).not.toHaveMethod('store_truth');
      });
    });
  });
  
  describe('INV-FLUX-02: Journalisation systématique', () => {
    it('should journalize all interactions', async () => {
      const journal = new InMemoryJournal();
      const orchestrator = new MediationOrchestrator(journal);
      
      const intent = createTestIntent();
      await orchestrator.process(intent);
      
      // Vérification : toutes les étapes sont journalisées
      expect(journal.entries).toHaveLength(2); // Intention + Résultat
    });
  });
});
```

### 8.3 Tests de contrats

**Règle TEST-CONTRACT-01 : Tests de conformité**

Tous les contrats doivent être testés pour conformité.

**Exemple :**
```typescript
describe('Contract Compliance', () => {
  describe('IIntentSubmission', () => {
    it('should comply with contract', async () => {
      const gateway = new ProductGateway();
      const contract = IIntentSubmissionContract;
      
      // Test de conformité
      const intent = createValidIntent();
      const result = await gateway.submitIntent(intent);
      
      // Vérification
      expect(contract.validate(result)).toBe(true);
    });
  });
});
```

---

## 9. Exemples d'implémentation

### 9.1 Exemple : ProductGateway complet

```typescript
/**
 * Gateway produit. Point d'entrée pour les intentions des produits.
 * 
 * @contract IIntentSubmission v1.0
 * @layer Product
 */
class ProductGateway implements IIntentSubmission {
  constructor(
    private receiver: IIntentReceiver,
    private orchestrator: IMediationOrchestrator,
    private emitter: IResultEmitter,
    private logger: ILogger
  ) {}
  
  /**
   * Soumet une intention d'un produit.
   * 
   * @param intent - L'intention à soumettre
   * @returns Le résultat de l'intention
   * 
   * @invariant INV-FLUX-01 : Séquence complète
   * @invariant INV-FLUX-02 : Journalisation systématique
   */
  async submitIntent(intent: Intent): Promise<Result> {
    this.logger.info('Intention reçue', { intention_id: intent.id });
    
    try {
      // 1. Réception et validation
      const validated = await this.receiver.receive(intent);
      
      // 2. Orchestration complète
      const result = await this.orchestrator.process(validated);
      
      // 3. Émission du résultat
      await this.emitter.emit(result);
      
      this.logger.info('Intention traitée', { 
        intention_id: intent.id, 
        status: result.status 
      });
      
      return result;
    } catch (error) {
      this.logger.error('Erreur traitement intention', { 
        intention_id: intent.id, 
        error 
      });
      throw error;
    }
  }
}
```

### 9.2 Exemple : IntentTranslator complet

```typescript
/**
 * Traducteur d'intention. Traduit intention → demande.
 * 
 * @contract ITranslation v1.0
 * @layer Translation
 * 
 * @invariant INV-NAT-02 : Traducteur, pas exécuteur
 * @invariant Pure function : pas d'effet de bord
 */
class IntentTranslator implements ITranslation {
  constructor(
    private vocabularyMapper: IVocabularyMapper,
    private contextEnricher: IContextEnricher
  ) {}
  
  /**
   * Traduit une intention en demande.
   * 
   * @param intent - L'intention à traduire
   * @param context - Le contexte de traduction
   * @returns La demande traduite
   */
  translate(intent: ValidatedIntent, context: TranslationContext): Demand {
    // Traduction pure : pas d'effet de bord
    // Pas de modification d'état
    // Pas d'appel à des services externes
    
    // 1. Mapping du vocabulaire
    const mappedType = this.vocabularyMapper.mapType(intent.type);
    const mappedPayload = this.vocabularyMapper.mapPayload(intent.payload);
    
    // 2. Enrichissement du contexte
    const enrichedContext = this.contextEnricher.enrich(context);
    
    // 3. Construction de la demande
    return {
      id: generateDemandId(),
      intention_id: intent.id,
      type: mappedType,
      data: mappedPayload,
      context: enrichedContext,
      timestamp: Date.now()
    };
  }
}
```

---

## 10. Checklist d'implémentation

### 10.1 Checklist de conformité

Avant de considérer une implémentation comme complète, vérifier :

**Contrats :**
- [ ] Tous les contrats sont implémentés
- [ ] Tous les tests de contrat passent
- [ ] Documentation des contrats à jour

**Invariants :**
- [ ] Tous les invariants sont respectés
- [ ] Tests d'invariants passent
- [ ] Aucune violation détectée

**Architecture :**
- [ ] Structure en 4 couches respectée
- [ ] Isolation des couches respectée
- [ ] Interfaces explicites définies

**Tests :**
- [ ] Couverture de code > 80%
- [ ] Tests unitaires pour tous les composants
- [ ] Tests d'intégration pour tous les flux
- [ ] Tests de contrats passent

**Documentation :**
- [ ] Code documenté
- [ ] Interfaces documentées
- [ ] Exemples fournis

### 10.2 Checklist de qualité

**Code :**
- [ ] Code lisible et maintenable
- [ ] Nommage explicite
- [ ] Pas de code mort
- [ ] Pas de duplication

**Performance :**
- [ ] Métriques de performance respectées
- [ ] Pas d'optimisation prématurée
- [ ] Profiling effectué

**Sécurité :**
- [ ] Pas de secrets dans le code
- [ ] Validation des entrées
- [ ] Gestion sécurisée des erreurs

---

## 11. Conformité MSCM/MIP

### 11.1 Obligation de balisage MSCM

Tout code implémenté pour Bonding Brother DOIT être balisé selon le protocole MSCM v1.

**Référence :** [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md)

**Obligations minimales :**
- Chaque bloc fonctionnel DOIT avoir un identifiant unique (`@id`)
- Le rôle sémantique DOIT être explicite (`@role`)
- La couche architecturale DOIT être déclarée (`@layer`)
- Une description humaine DOIT accompagner chaque bloc (`@human`)

### 11.2 Intégration MIP

Après implémentation, l'index MIP DOIT être régénéré pour :
- Valider l'intégrité des blocs MSCM
- Mettre à jour le graphe de dépendances
- Vérifier la cohérence hiérarchique

### 11.3 Check-list MSCM

Avant toute livraison, vérifier :
- [ ] Tous les blocs critiques sont balisés MSCM
- [ ] Les identifiants sont uniques globalement
- [ ] Les couches (layer) sont cohérentes avec l'architecture
- [ ] L'index MIP peut être régénéré sans erreur

---

## 12. Statut contractuel

Ce document est **informatif, non contractuel, et de statut GUIDELINES**. Il fournit des recommandations pour l'implémentation de référence de Bonding Brother, mais n'établit pas de contraintes contractuelles.

Les contrats et invariants restent la source de vérité. Ces guidelines sont des recommandations pour faciliter une implémentation conforme.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** GUIDELINES — Informatif  
**Dépendances :** 
- Architecture & Flows v2.0
- Invariants & Guarantees v2.0
- Documentation Fondatrice v2.0
- Tous les contrats v2.0
- MIP v1 MSCM Index Protocol
