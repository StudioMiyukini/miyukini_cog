# WorrySentinel — Violations & Anti-Patterns

## 1. Contexte

Ce document catalogue les **violations** des invariants de WorrySentinel et les **anti-patterns** à éviter. Il sert de référence négative : ce qu'il ne faut jamais faire, pourquoi, et comment détecter et corriger ces erreurs.

**Document fondateur :** [WorrySentinel - Documentation Fondatrice](../../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md)

**Document associé :** [WorrySentinel - Invariants & Guarantees](./WorrySentinel%20-%20Invariants%20&%20Guarantees.md)

**Statut contractuel :** Ce document est **contractuel, normatif, et préventif**. Il dérive directement de la Documentation Fondatrice (Section 13 - Violations et comportements interdits) et des invariants INV-WS-1 à INV-WS-8, INV-GOV-1 à INV-GOV-8.

---

## 2. Portée / Scope

- **Applicable à :** Toute implémentation, configuration, ou utilisation de WorrySentinel
- **Objectif :** Identifier et prévenir les violations de gouvernance de sécurité
- **Public cible :** Développeurs, architectes, auditeurs, responsables sécurité
- **Ne couvre pas :** Les violations des autres cores (voir leurs documents respectifs)

---

## 3. Taxonomie des violations

### 3.1 Qu'est-ce qu'une violation ?

Une **violation** est un acte ou une implémentation qui contredit un invariant de WorrySentinel. Toute violation :

- **Est une faute architecturale** — Pas une simple erreur de code
- **Doit être corrigée immédiatement** — Pas de compromis temporaire
- **Compromet la gouvernance de sécurité** — Effets potentiellement cascadants
- **Est détectable** — Des critères permettent de l'identifier

### 3.2 Classification des violations

| Niveau | Gravité | Impact | Action requise |
|--------|---------|--------|----------------|
| **CRITIQUE** | Bloquant | Gouvernance compromise, système incohérent | Arrêt et correction immédiate |
| **MAJEURE** | Sérieux | Comportement imprévisible, garanties non respectées | Correction prioritaire |
| **MINEURE** | Modéré | Dégradation de qualité, dette technique | Correction planifiée |

---

## 4. Ce que WorrySentinel ne fait PAS

Cette section reprend et détaille les exclusions fondamentales de WorrySentinel.

### 4.1 WorrySentinel n'IMPLÉMENTE PAS

**Violation de :** INV-WS-1 (Aucune autorité sur l'implémentation), INV-GOV-7 (Séparation gouvernance/implémentation)

**Principe :**

> WorrySentinel ne possède **jamais** d'autorité sur l'implémentation des contrôles de sécurité. Une règle de gouvernance produite par WorrySentinel n'entraîne jamais d'implémentation automatique.

**Symptômes de violation :**

| Symptôme | Exemple | Gravité |
|----------|---------|---------|
| Code de contrôle de sécurité dans WorrySentinel | `if (!hashVerified(data)) { reject(); }` | CRITIQUE |
| Algorithme cryptographique dans WorrySentinel | `const signature = crypto.sign(data, key)` | CRITIQUE |
| Validation technique de sécurité | `validateJWT(token)` | CRITIQUE |
| Mécanisme de chiffrement | `encrypt(payload, algorithm)` | CRITIQUE |

**Pourquoi c'est interdit :**

- Confusion des responsabilités entre gouvernance et implémentation
- Impossible de modifier l'implémentation indépendamment des règles de gouvernance
- Couplage fort entre WorrySentinel et les mécanismes techniques

**Correction :**

```
❌ VIOLATION:
WorrySentinel.enforceSecurityLevel(data) {
  if (this.securityLevel >= 3) {
    return crypto.encrypt(data, 'AES-256');
  }
  return data;
}

✅ CORRECT:
WorrySentinel.getSecurityLevelRequirements(component) {
  return {
    securityLevel: this.getSecurityLevel(component),
    encryptionRequired: this.isEncryptionRequired(component),
    // Pas d'implémentation, juste des règles
  };
}

// Un autre composant (StrongFather ou adaptateur) applique les règles
SecurityAdapter.applySecurityRules(data, requirements) {
  if (requirements.encryptionRequired) {
    return encryptionService.encrypt(data);
  }
  return data;
}
```

### 4.2 WorrySentinel n'EXÉCUTE PAS

**Violation de :** INV-WS-2 (Aucune autorité sur l'exécution), INV-GOV-7

**Principe :**

> WorrySentinel ne possède **jamais** d'autorité sur l'exécution des vérifications de sécurité. WorrySentinel gouverne, mais n'exécute jamais.

**Symptômes de violation :**

| Symptôme | Exemple | Gravité |
|----------|---------|---------|
| Méthode `execute()` dans WorrySentinel | `worrySentinel.executeSecurityCheck()` | CRITIQUE |
| Appel de vérification depuis WorrySentinel | `await securityService.verify(data)` | CRITIQUE |
| Ordonnancement d'exécution | `scheduleSecurityScan()` | MAJEURE |
| Surveillance active | `monitorExecution(process)` | MAJEURE |

**Pourquoi c'est interdit :**

- WorrySentinel usurpe le rôle des cores exécutants
- Impossible de tracer qui a réellement exécuté
- Création de dépendances externes non contrôlées

**Correction :**

```
❌ VIOLATION:
WorrySentinel.verifyIntegrity(system) {
  const result = await integrityService.scan(system);
  if (!result.valid) {
    this.transitionToState('T2');
  }
  return result;
}

✅ CORRECT:
WorrySentinel.getIntegrityRules() {
  return {
    requiredChecks: ['hash', 'signature', 'timestamp'],
    thresholds: { maxAnomalies: 3 }
  };
}

// CaringNanny observe et signale
CaringNanny.onIntegritySignal(signal) {
  // WorrySentinel reçoit le signal, ne fait pas la vérification
  const stateProposal = WorrySentinel.evaluateSignal(signal);
  return stateProposal;
}
```

### 4.3 WorrySentinel ne PERSISTE PAS

**Violation de :** INV-WS-3 (Aucune autorité sur la persistance)

**Principe :**

> WorrySentinel ne possède **jamais** d'autorité sur la persistance. WorrySentinel ne peut jamais modifier, lire, ou accéder à des données persistées.

**Symptômes de violation :**

| Symptôme | Exemple | Gravité |
|----------|---------|---------|
| Accès base de données dans WorrySentinel | `await db.query("SELECT * FROM trust_states")` | CRITIQUE |
| Écriture fichier par WorrySentinel | `fs.writeFileSync('security_config.json', config)` | CRITIQUE |
| Appel à KindMother depuis WorrySentinel | `await kindMother.persist('state', trustState)` | CRITIQUE |
| Cache persisté dans WorrySentinel | `redis.set('security:level', level)` | MAJEURE |

**Pourquoi c'est interdit :**

- Violation de la souveraineté de KindMother sur les données
- WorrySentinel devient dépendant de la persistance
- Risque de désynchronisation entre gouvernance et données

**Correction :**

```
❌ VIOLATION:
class WorrySentinel {
  async saveSecurityState(state) {
    await this.db.securityStates.insert(state);
  }
}

✅ CORRECT:
class WorrySentinel {
  governSecurityState() {
    // Définition en mémoire uniquement
    return {
      currentLevel: this.currentSecurityLevel,
      trustState: this.currentTrustState,
      rules: this.governanceRules
    };
  }
}

// La persistance est gérée par KindMother via les adaptateurs
class SecurityAdapter {
  async persistGovernanceState(state) {
    // L'adaptateur appelle KindMother, pas WorrySentinel
    await kindMother.persist('governance', state);
  }
}
```

### 4.4 WorrySentinel ne MODIFIE PAS L'ÉTAT

**Violation de :** INV-WS-4 (Aucune modification d'état)

**Principe :**

> WorrySentinel ne modifie **jamais** un état ou un fait. WorrySentinel gouverne et définit, mais ne change jamais l'état du système.

**Symptômes de violation :**

| Symptôme | Exemple | Gravité |
|----------|---------|---------|
| WorrySentinel change l'état système | `systemState.trustLevel = T2` | CRITIQUE |
| WorrySentinel crée un fait | `this.facts.add(newSecurityFact)` | CRITIQUE |
| WorrySentinel supprime un état | `delete this.states['anomaly']` | CRITIQUE |
| WorrySentinel met à jour un fait | `this.facts.update(factId, newValue)` | CRITIQUE |

**Pourquoi c'est interdit :**

- L'état est observé et modifié par d'autres cores, pas par WorrySentinel
- Modifications non coordonnées créent des incohérences
- Impossible de tracer qui a modifié quoi

**Correction :**

```
❌ VIOLATION:
WorrySentinel.degradeSystem() {
  this.systemState.trustLevel = 'T2';
  this.systemState.capabilities = this.reducedCapabilities;
}

✅ CORRECT:
WorrySentinel.proposeStateTransition(currentState, signals) {
  // WorrySentinel propose, ne modifie pas
  return {
    proposedState: 'T2',
    justification: 'Anomalies persistantes détectées',
    requiredCapabilities: this.getCapabilitiesForState('T2')
  };
}

// Un autre composant applique la transition
StateManager.applyTransition(proposal) {
  // Après validation par StrongFather
  this.systemState = proposal.proposedState;
}
```

### 4.5 WorrySentinel ne GÈRE PAS LE TEMPS TECHNIQUE

**Violation de :** INV-WS-5 (Aucune logique temporelle technique)

**Principe :**

> WorrySentinel ne possède **jamais** de logique temporelle technique. WorrySentinel ne gère jamais le temps, les horodatages, ou l'ordonnancement technique.

**Symptômes de violation :**

| Symptôme | Exemple | Gravité |
|----------|---------|---------|
| Gestion de timestamps | `const now = Date.now()` | MAJEURE |
| Ordonnancement temporel | `setTimeout(() => this.checkState(), 5000)` | MAJEURE |
| Calcul de durées | `if (elapsed > threshold) { ... }` | MAJEURE |
| Synchronisation temporelle | `await this.waitForSync()` | MAJEURE |

**Pourquoi c'est interdit :**

- Le temps technique est du ressort du Kernel (Clock)
- WorrySentinel gouverne des concepts, pas des mécanismes temporels
- Création de dépendances implicites avec le temps système

**Correction :**

```
❌ VIOLATION:
WorrySentinel.checkExpiration() {
  const now = Date.now();
  if (now - this.lastCheck > 60000) {
    this.refreshState();
  }
}

✅ CORRECT:
WorrySentinel.evaluateTemporalContext(context) {
  // WorrySentinel reçoit le contexte temporel, ne le génère pas
  return {
    stateValid: context.period === 'current',
    refreshRequired: context.stale === true
  };
}

// Le Kernel fournit le temps
Kernel.provideTemporalContext() {
  return {
    period: this.clock.getCurrentPeriod(),
    stale: this.clock.isStale()
  };
}
```

### 4.6 WorrySentinel ne PREND PAS DE DÉCISION SPÉCIFIQUE

**Violation de :** INV-WS-6 (Zero-trust), Documentation Fondatrice Section 5

**Principe :**

> WorrySentinel ne prend **jamais** de décision spécifique d'autorisation ou de refus. La décision est du ressort exclusif de StrongFather. WorrySentinel gouverne les niveaux et les états, pas les cas concrets.

**Symptômes de violation :**

| Symptôme | Exemple | Gravité |
|----------|---------|---------|
| Méthode `decide()` dans WorrySentinel | `worrySentinel.decideAccess(request)` | CRITIQUE |
| Retour accept/reject | `return { decision: 'denied' }` | CRITIQUE |
| Évaluation d'intention spécifique | `if (intent.action === 'delete') { deny() }` | MAJEURE |
| Application de politique à un cas | `applyPolicy(intent)` | MAJEURE |

**Pourquoi c'est interdit :**

- Usurpation du rôle de StrongFather
- Décisions prises sans vision globale des politiques
- Impossibilité d'appel ou de révision des décisions

**Correction :**

```
❌ VIOLATION:
WorrySentinel.evaluateRequest(request) {
  if (this.trustState === 'T3' && request.action === 'write') {
    return { decision: 'DENIED', reason: 'System in restricted state' };
  }
  return { decision: 'ALLOWED' };
}

✅ CORRECT:
WorrySentinel.provideGovernanceContext() {
  return {
    trustState: this.trustState,
    securityLevel: this.securityLevel,
    allowedModes: this.getAllowedModesForState(),
    // Pas de décision, juste du contexte de gouvernance
  };
}

StrongFather.evaluateIntent(intent) {
  const context = WorrySentinel.provideGovernanceContext();
  // StrongFather prend la décision basée sur le contexte
  return this.applyPolicies(intent, context);
}
```

### 4.7 WorrySentinel ne GÈRE PAS DE MÉCANISMES CRYPTOGRAPHIQUES

**Violation de :** Documentation Fondatrice Section 5 (Hors-scope explicite)

**Principe :**

> WorrySentinel ne définit **jamais** d'algorithme cryptographique, ne spécifie jamais de protocole de chiffrement, ne gère jamais de clés cryptographiques.

**Symptômes de violation :**

| Symptôme | Exemple | Gravité |
|----------|---------|---------|
| Référence à des algorithmes | `algorithm: 'AES-256-GCM'` | CRITIQUE |
| Gestion de clés | `this.rotateKeys()` | CRITIQUE |
| Protocoles de chiffrement | `tlsVersion: '1.3'` | MAJEURE |
| Vérification de signatures | `verifySignature(data, sig)` | CRITIQUE |

**Pourquoi c'est interdit :**

- WorrySentinel gouverne des concepts, pas des mécanismes techniques
- Couplage avec des bibliothèques cryptographiques
- Impossibilité de changer de crypto sans modifier WorrySentinel

**Correction :**

```
❌ VIOLATION (règle):
{
  securityLevel: 3,
  requirements: {
    encryption: 'AES-256-GCM',
    keyRotation: '90 days',
    signature: 'RSA-4096'
  }
}

✅ CORRECT (règle déclarative):
{
  securityLevel: 3,
  requirements: {
    encryptionRequired: true,
    keyRotationRequired: true,
    signatureRequired: true
    // Les détails techniques sont gérés ailleurs
  }
}
```

### 4.8 WorrySentinel ne contient PAS DE LOGIQUE MÉTIER

**Violation de :** Documentation Fondatrice Section 2 (Ce que WorrySentinel ne décide pas)

**Principe :**

> WorrySentinel ne contient **jamais** de logique métier spécifique aux produits. Il définit des concepts généraux (niveaux de sécurité, états de confiance, dégradation) applicables à tous les produits.

**Symptômes de violation :**

| Symptôme | Exemple | Gravité |
|----------|---------|---------|
| Règles spécifiques à un produit | `if (product === 'ecommerce') { ... }` | MAJEURE |
| Référence à des entités métier | `if (transaction.amount > 10000) { ... }` | MAJEURE |
| Logique conditionnelle produit | `switch(productType) { ... }` | MAJEURE |
| Workflows spécifiques | `if (orderState === 'checkout') { ... }` | MINEURE |

**Pourquoi c'est interdit :**

- WorrySentinel doit rester générique et réutilisable
- Couplage avec un produit rend WorrySentinel non portable
- Violation du principe de séparation des préoccupations

---

## 5. Violations de gouvernance

### 5.1 Violations des états de confiance

**Catégorie :** CRITIQUE

**Source :** Documentation Fondatrice Section 7, INV-GOV-2, INV-GOV-3

**VIOL-GOV-1 : Modification directe d'état de confiance**

Un composant modifie directement l'état de confiance sans passer par WorrySentinel.

*Invariants violés : INV-GOV-2 (États de confiance uniques), INV-GOV-3 (Transitions justifiées)*

**Symptômes :**
- Composant qui change `trustState` directement
- État de confiance modifié sans signal de gouvernance
- Transition non tracée dans les logs

**VIOL-GOV-2 : Transition brutale**

Le système passe brutalement d'un état de confiance à un autre sans passer par les états intermédiaires.

*Invariant violé : INV-GOV-4 (Dégradation progressive uniquement)*

**Symptômes :**
- Passage direct de T0 à T3 ou T4
- Absence d'états intermédiaires dans l'historique
- Transition sans dégradation progressive

### 5.2 Violations des niveaux de sécurité

**Catégorie :** MAJEURE

**Source :** Documentation Fondatrice Section 6, INV-GOV-1, INV-GOV-6

**VIOL-GOV-3 : Niveau de sécurité implicite**

Un produit ou composant fonctionne sans niveau de sécurité explicite défini.

*Invariant violé : INV-GOV-1 (Niveaux de sécurité explicites)*

**Symptômes :**
- Composant sans attribut `securityLevel`
- Produit sans déclaration de niveau dans la configuration
- Fonctionnement sans vérification du niveau requis

**VIOL-GOV-4 : Incohérence inter-composants**

Un composant de niveau N accède directement à un composant de niveau > N sans médiation.

*Invariant violé : INV-GOV-6 (Cohérence inter-composants)*

**Symptômes :**
- Composant niveau 1 accédant directement à un composant niveau 3
- Absence de médiation par un adaptateur ou StrongFather
- Appels directs entre niveaux incompatibles

### 5.3 Violations de séparation

**Catégorie :** CRITIQUE

**Source :** Documentation Fondatrice Section 4, INV-WS-1, INV-WS-2, INV-GOV-7

**VIOL-GOV-5 : Implémentation par WorrySentinel**

WorrySentinel implémente directement un contrôle de sécurité.

*Invariants violés : INV-WS-1 (Aucune autorité sur l'implémentation), INV-GOV-7 (Séparation gouvernance/implémentation)*

**Symptômes :**
- Code de contrôle technique dans WorrySentinel
- Mécanismes de sécurité implémentés dans WorrySentinel
- Validation technique effectuée par WorrySentinel

**VIOL-GOV-6 : Exécution par WorrySentinel**

WorrySentinel exécute directement une vérification de sécurité.

*Invariants violés : INV-WS-2 (Aucune autorité sur l'exécution), INV-GOV-7 (Séparation gouvernance/implémentation)*

**Symptômes :**
- Appels à des services de sécurité depuis WorrySentinel
- Ordonnancement de vérifications par WorrySentinel
- Surveillance active effectuée par WorrySentinel

### 5.4 Violations de traçabilité

**Catégorie :** MAJEURE

**Source :** INV-WS-8, INV-GOV-8

**VIOL-GOV-7 : Décision de gouvernance non tracée**

Une décision de gouvernance est produite sans traçabilité.

*Invariants violés : INV-WS-8 (Traçabilité complète), INV-GOV-8 (Traçabilité complète)*

**Symptômes :**
- Transition d'état sans justification
- Changement de niveau sans contexte enregistré
- Décision sans identifiant de trace

**VIOL-GOV-8 : Règle implicite appliquée**

Une règle implicite (non déclarée) est appliquée par WorrySentinel.

*Invariant violé : INV-WS-7 (Gouvernance explicite)*

**Symptômes :**
- Comportement non documenté dans les règles
- Décision basée sur des règles non déclarées
- Logique conditionnelle cachée

---

## 6. Comportements interdits

### 6.1 Interdictions absolues

| Code | Interdiction | Invariant source | Gravité |
|------|--------------|------------------|---------|
| **INTERD-WS-1** | WorrySentinel ne peut pas implémenter de contrôle de sécurité | INV-WS-1 | CRITIQUE |
| **INTERD-WS-2** | WorrySentinel ne peut pas exécuter de vérification de sécurité | INV-WS-2 | CRITIQUE |
| **INTERD-WS-3** | WorrySentinel ne peut pas accéder à KindMother directement | INV-WS-3 | CRITIQUE |
| **INTERD-WS-4** | WorrySentinel ne peut pas modifier l'état du système | INV-WS-4 | CRITIQUE |
| **INTERD-WS-5** | WorrySentinel ne peut pas prendre de décision spécifique d'autorisation | Section 5 | CRITIQUE |
| **INTERD-WS-6** | WorrySentinel ne peut pas gérer de mécanismes cryptographiques | Section 5 | CRITIQUE |
| **INTERD-WS-7** | WorrySentinel ne peut pas contenir de logique métier | Section 2 | MAJEURE |
| **INTERD-WS-8** | WorrySentinel ne peut pas contourner les invariants FONDATION | Section 14 | CRITIQUE |

### 6.2 Interdictions de gouvernance

| Code | Interdiction | Source |
|------|--------------|--------|
| **INTERD-GOV-1** | Aucun composant ne peut contourner la gouvernance de WorrySentinel | Section 13 |
| **INTERD-GOV-2** | Aucune modification de gouvernance sans traçabilité complète | INV-GOV-8 |
| **INTERD-GOV-3** | Aucune transition entre états de confiance sans justification explicite | INV-GOV-3 |
| **INTERD-GOV-4** | Aucune dégradation brutale — Toute dégradation doit être progressive | INV-GOV-4 |

---

## 7. Anti-patterns

### 7.1 AP-WS-01 : WorrySentinel comme orchestrateur de sécurité

**Description :**

Utiliser WorrySentinel pour orchestrer des contrôles de sécurité, des workflows de vérification, ou des processus d'audit au lieu de simplement gouverner les niveaux et états.

**Pourquoi c'est un anti-pattern :**

WorrySentinel est un gouvernant de sécurité, pas un orchestrateur. L'orchestration implique l'exécution et le contrôle de flux, ce qui viole l'interdiction d'exécution.

**Symptômes :**

- WorrySentinel déclenche des vérifications suite à des transitions d'état
- WorrySentinel maintient un état de workflow de sécurité
- WorrySentinel attend des événements pour progresser dans un processus

**Solution :**

L'orchestration des contrôles de sécurité doit être effectuée par les adaptateurs produits ou par un composant dédié, pas par WorrySentinel.

### 7.2 AP-WS-02 : WorrySentinel comme cache de sécurité

**Description :**

Utiliser WorrySentinel pour stocker et mettre en cache des informations de sécurité pour accès ultérieur.

**Pourquoi c'est un anti-pattern :**

WorrySentinel ne persiste pas de données. Utiliser WorrySentinel comme cache viole l'interdiction de persistance et crée des états incohérents.

**Symptômes :**

- WorrySentinel mémorise des états de confiance pour réutilisation
- WorrySentinel maintient un historique des transitions en mémoire longue
- WorrySentinel optimise via la mise en cache de résultats de gouvernance

**Solution :**

Le cache et la persistance doivent être gérés par KindMother via les adaptateurs, pas par WorrySentinel.

### 7.3 AP-WS-03 : Contournement de gouvernance par adaptateur

**Description :**

Utiliser un adaptateur pour contourner les règles de gouvernance de WorrySentinel en appliquant des niveaux de sécurité ou des états de confiance non gouvernés.

**Pourquoi c'est un anti-pattern :**

Le contournement via adaptateur viole l'esprit des contrats et peut introduire des incohérences systémiques graves.

**Symptômes :**

- L'adaptateur définit ses propres niveaux de sécurité sans consulter WorrySentinel
- L'adaptateur ignore l'état de confiance global
- L'adaptateur modifie des décisions de gouvernance avant de les appliquer

**Solution :**

Les adaptateurs doivent respecter la gouvernance de WorrySentinel et ne jamais la contourner ou la modifier.

### 7.4 AP-WS-04 : Niveaux de sécurité techniques

**Description :**

Définir des niveaux de sécurité qui portent sur des aspects techniques (algorithmes, protocoles, configurations) au lieu du profil de risque conceptuel.

**Pourquoi c'est un anti-pattern :**

WorrySentinel gouverne des niveaux de sécurité conceptuels, pas des configurations techniques. Les détails techniques sont hors-scope.

**Symptômes :**

- Niveaux de sécurité définis par algorithme (`level3 = AES-256`)
- Niveaux liés à des protocoles (`level4 = TLS1.3`)
- Configuration technique dans la définition des niveaux

**Solution :**

Les niveaux de sécurité doivent être définis conceptuellement (Public, Standard, Sensitive, Critical, Highest). Les détails techniques sont gérés par les composants d'implémentation.

### 7.5 AP-WS-05 : États de confiance métier

**Description :**

Confondre les états de confiance système (T0-T4) avec des états métier spécifiques à un produit.

**Pourquoi c'est un anti-pattern :**

Les états de confiance (T0-T4) caractérisent l'intégrité du système global, pas des états fonctionnels produit. L'inclusion d'états métier crée un couplage inapproprié.

**Symptômes :**

- États de confiance liés à des événements métier (`T2 = commande suspecte`)
- Transition d'état basée sur des règles métier
- États de confiance différents selon le produit

**Solution :**

Les états de confiance restent globaux et génériques. Les états métier sont gérés par les produits, pas par WorrySentinel.

### 7.6 AP-WS-06 : Dépendance temporelle technique

**Description :**

Faire dépendre la gouvernance de WorrySentinel du temps technique (horodatages, timestamps, délais).

**Pourquoi c'est un anti-pattern :**

WorrySentinel ne possède pas de logique temporelle technique. La gouvernance ne doit pas dépendre du temps technique.

**Symptômes :**

- Transitions d'état basées sur des durées (`if (elapsed > 10min) → T2`)
- Gouvernance qui change selon l'heure
- Niveaux de sécurité avec expiration temporelle

**Solution :**

Le temps conceptuel (période, cycle, contexte) peut être utilisé via les signaux de CaringNanny ou du Kernel, mais pas le temps technique direct.

### 7.7 AP-WS-07 : WorrySentinel comme point d'entrée unique

**Description :**

Faire de WorrySentinel le point d'entrée unique de toutes les opérations du système, même celles qui ne nécessitent pas de gouvernance de sécurité.

**Pourquoi c'est un anti-pattern :**

WorrySentinel est un gouvernant de sécurité, pas une gateway. Toutes les opérations ne nécessitent pas une évaluation de gouvernance.

**Symptômes :**

- Toutes les requêtes passent par WorrySentinel
- WorrySentinel est appelé pour des opérations triviales
- WorrySentinel devient un goulot d'étranglement

**Solution :**

WorrySentinel doit être utilisé uniquement pour la gouvernance de sécurité (niveaux, états, dégradation), pas pour toutes les opérations.

### 7.8 AP-WS-08 : Gouvernance réactive

**Description :**

Utiliser WorrySentinel de manière purement réactive (après les faits) au lieu de manière proactive (avant les opérations).

**Pourquoi c'est un anti-pattern :**

WorrySentinel doit contraindre les comportements avant qu'ils ne se produisent (pression verticale), pas réagir après les faits.

**Symptômes :**

- Transition d'état uniquement après détection d'incident
- Absence de contraintes préventives
- Gouvernance qui "court après" les événements

**Solution :**

WorrySentinel doit exercer une pression verticale proactive sur les cores fonctionnels, pas seulement réagir aux signaux.

---

## 8. Matrice de détection

### 8.1 Checklist de vérification

| # | Vérification | Invariant | Gravité si violé |
|---|--------------|-----------|------------------|
| 1 | WorrySentinel ne contient pas de code d'implémentation de sécurité | INV-WS-1 | CRITIQUE |
| 2 | WorrySentinel n'exécute pas de vérifications de sécurité | INV-WS-2 | CRITIQUE |
| 3 | WorrySentinel n'accède pas à la base de données ou KindMother | INV-WS-3 | CRITIQUE |
| 4 | WorrySentinel ne modifie pas l'état du système | INV-WS-4 | CRITIQUE |
| 5 | WorrySentinel ne gère pas le temps technique | INV-WS-5 | MAJEURE |
| 6 | WorrySentinel ne retourne pas de décision (accept/reject) | INV-WS-6 | CRITIQUE |
| 7 | Toutes les règles de gouvernance sont explicites | INV-WS-7 | MAJEURE |
| 8 | Toute décision de gouvernance est tracée | INV-WS-8 | MAJEURE |
| 9 | Tout composant a un niveau de sécurité explicite | INV-GOV-1 | MAJEURE |
| 10 | L'état de confiance est unique et global | INV-GOV-2 | CRITIQUE |
| 11 | Toute transition d'état est justifiée | INV-GOV-3 | MAJEURE |
| 12 | La dégradation est progressive (pas de saut) | INV-GOV-4 | CRITIQUE |
| 13 | Les invariants FONDATION sont préservés | INV-GOV-5 | CRITIQUE |
| 14 | Cohérence des niveaux entre composants | INV-GOV-6 | MAJEURE |

### 8.2 Signaux d'alerte dans le code

| Signal | Module suspect | Action |
|--------|----------------|--------|
| `import { db }` | WorrySentinel | Vérifier INV-WS-3 |
| `import { kindMother }` | WorrySentinel | Vérifier INV-WS-3 |
| `crypto.*` | WorrySentinel | Vérifier INTERD-WS-6 |
| `Date.now()` ou `setTimeout` | WorrySentinel | Vérifier INV-WS-5 |
| `execute*` ou `run*` | Méthodes WorrySentinel | Vérifier INV-WS-2 |
| `return.*decision` | WorrySentinel | Vérifier INTERD-WS-5 |
| `if (product === ...)` | Logique WorrySentinel | Vérifier logique métier |
| `T0.*T3` ou `T0.*T4` | Transitions | Vérifier INV-GOV-4 |

---

## 9. Procédure de correction

### 9.1 Étapes de correction d'une violation

```
┌──────────────────────────────────────────────────────────┐
│ 1. IDENTIFIER                                            │
│    - Quel invariant est violé ?                          │
│    - Quelle est la gravité ?                             │
│    - Quel est l'impact sur la gouvernance ?              │
└──────────────────────────────────────────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────────────────┐
│ 2. ISOLER                                                │
│    - Localiser le code/définition fautif                 │
│    - Identifier les dépendances                          │
│    - Évaluer la propagation                              │
└──────────────────────────────────────────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────────────────┐
│ 3. CORRIGER                                              │
│    - Appliquer le pattern correct                        │
│    - Déplacer la logique vers le core approprié          │
│    - Restaurer la séparation gouvernance/implémentation  │
└──────────────────────────────────────────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────────────────┐
│ 4. VÉRIFIER                                              │
│    - Passer la checklist de détection                    │
│    - Valider la cohérence globale                        │
│    - Tester les interactions avec les autres cores       │
└──────────────────────────────────────────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────────────────┐
│ 5. DOCUMENTER                                            │
│    - Tracer la correction                                │
│    - Mettre à jour les définitions                       │
│    - Ajouter à l'historique de gouvernance               │
└──────────────────────────────────────────────────────────┘
```

### 9.2 Responsabilités de correction

| Violation de | Core responsable de l'action |
|--------------|------------------------------|
| INV-WS-1 (implémentation) | Déplacer vers adaptateurs/StrongFather |
| INV-WS-2 (exécution) | Déplacer vers CaringNanny/adaptateurs |
| INV-WS-3 (persistance) | Déplacer vers KindMother |
| INV-WS-4 (modification d'état) | Déplacer vers composants exécutants |
| INV-WS-5 (temps technique) | Utiliser signaux Kernel |
| INV-WS-6 (zero-trust) | Déplacer décisions vers StrongFather |
| INV-WS-7 (explicite) | Déclarer règles manquantes |
| INV-WS-8 (traçabilité) | Ajouter traces complètes |
| INV-GOV-1 (niveaux explicites) | Déclarer niveaux manquants |
| INV-GOV-2 (état unique) | Centraliser gestion d'état |
| INV-GOV-3 (transitions justifiées) | Ajouter justifications |
| INV-GOV-4 (dégradation progressive) | Implémenter transitions intermédiaires |
| INV-GOV-6 (cohérence) | Ajouter médiation entre niveaux |

---

## 10. Conséquences des violations

### 10.1 Violations critiques

**Conséquences :**

1. **Non-conformité immédiate** : L'implémentation est considérée non conforme
2. **Gouvernance compromise** : La gouvernance de sécurité n'est plus garantie
3. **Audit obligatoire** : Un audit de sécurité doit être effectué
4. **Correction impérative** : La correction est obligatoire avant toute utilisation

### 10.2 Violations majeures

**Conséquences :**

1. **Warning de non-conformité** : L'implémentation est signalée comme non conforme
2. **Gouvernance dégradée** : La gouvernance peut être incohérente
3. **Correction requise** : La correction doit être planifiée

### 10.3 Violations mineures

**Conséquences :**

1. **Signalement** : La violation est signalée
2. **Correction recommandée** : La correction est recommandée
3. **Traçabilité** : La violation est tracée pour suivi

---

## 11. Références croisées

### Documents associés

| Document | Relation |
|----------|----------|
| [WorrySentinel - Documentation Fondatrice](../../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md) | Document source (Section 13, 14) |
| [WorrySentinel - Invariants & Guarantees](./WorrySentinel%20-%20Invariants%20&%20Guarantees.md) | Définition des invariants violés |
| [WorrySentinel - StrongFather Integration Contract](../integration/WorrySentinel%20-%20StrongFather%20Integration%20Contract.md) | Où déplacer les décisions |
| [WorrySentinel - CaringNanny Integration Contract](../integration/WorrySentinel%20-%20CaringNanny%20Integration%20Contract.md) | Où déplacer l'observation |
| [WorrySentinel - MiyukiniAdmin Integration Contract](../integration/WorrySentinel%20-%20MiyukiniAdmin%20Integration%20Contract.md) | Interface de configuration |

### Références glossaire

| Terme | Définition |
|-------|------------|
| **Violation** | Acte ou implémentation qui contredit un invariant de WorrySentinel |
| **Anti-pattern** | Pattern de conception à éviter car contraire aux invariants |
| **Gravité** | Niveau d'impact d'une violation (critique, majeure, mineure) |
| **Faute architecturale** | Erreur de conception qui compromet l'intégrité de la gouvernance |
| **Gouvernance** | Définition des règles sans exécution |
| **Pression verticale** | Contrainte imposée sur tous les cores depuis la Strate 4 |

---

## 12. Synthèse contractuelle

### Engagements de ce contrat

Ce contrat établit que :

1. **Les violations sont identifiées** — 8 exclusions fondamentales documentées
2. **Les anti-patterns sont catalogués** — 8 anti-patterns avec exemples et corrections
3. **La détection est systématique** — Checklist et signaux d'alerte fournis
4. **La correction est guidée** — Procédure en 5 étapes avec responsabilités
5. **Les gravités sont classifiées** — CRITIQUE, MAJEURE, MINEURE

### Phrase de synthèse

> **WorrySentinel ne possède aucune autorité sur l'implémentation, l'exécution, la persistance, la modification d'état, le temps technique, les décisions spécifiques, les mécanismes cryptographiques, ou la logique métier — toute violation de ces exclusions est une faute architecturale qui compromet la gouvernance de sécurité et doit être immédiatement corrigée.**

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Contrat FONDATION — Préventif  
**Référence :** WorrySentinel v1.2, Documentation Fondatrice Section 13, Section 14  
**Type :** Contrat de gouvernance — Violations et Anti-Patterns

---

## 13. Mini log de génération

### Décision éditoriale E1 : Structure du document

**Décision prise :** Respect de la structure établie par les documents Violations & Anti-Patterns de StrongFather et BorderGuard pour assurer la cohérence documentaire.

**Application :** Structure en 12 sections principales avec taxonomie, exclusions, violations, anti-patterns, matrice de détection, et procédure de correction.

### Décision éditoriale E2 : Consolidation des violations

**Décision prise :** Consolidation de toutes les violations de la Documentation Fondatrice (Section 13) et des interdictions de l'index en un catalogue unique.

**Application :** 8 violations de gouvernance (VIOL-GOV-1 à VIOL-GOV-8) et 12 interdictions (INTERD-WS-1 à INTERD-WS-8, INTERD-GOV-1 à INTERD-GOV-4) cataloguées.

### Décision éditoriale E3 : Anti-patterns spécifiques

**Décision prise :** Identification de 8 anti-patterns spécifiques à WorrySentinel, distincts de ceux de StrongFather ou BorderGuard.

**Application :** Anti-patterns AP-WS-01 à AP-WS-08 documentés avec description, symptômes et solutions.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Toutes les violations de la Documentation Fondatrice sont incluses
- ✅ Toutes les interdictions de l'index sont référencées
- ✅ Les références aux invariants INV-WS-1 à INV-WS-8 sont correctes
- ✅ Les références aux invariants INV-GOV-1 à INV-GOV-8 sont correctes
- ✅ Les gravités sont cohérentes avec l'importance des règles
- ✅ Cohérence avec la structure des documents BorderGuard et StrongFather

**Conclusion :** Catalogue complet et cohérent.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
