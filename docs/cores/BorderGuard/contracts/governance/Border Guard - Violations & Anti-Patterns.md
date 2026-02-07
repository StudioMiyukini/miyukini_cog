# Border Guard - Violations & Anti-Patterns

## 1. Contexte

Ce document catalogue les **violations** des invariants de Border Guard et les **anti-patterns** à éviter. Il sert de référence négative : ce qu'il ne faut jamais faire, pourquoi, et comment détecter et corriger ces erreurs.

**Document fondateur :** [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

**Document associé :** [Border Guard - Invariants & Guarantees](./Border%20Guard%20-%20Invariants%20&%20Guarantees.md)

**Statut contractuel :** Ce document est **contractuel, normatif, et préventif**. Il dérive directement de la Documentation Fondatrice (Section 6 - Ce que Border Guard ne fait PAS) et des invariants (Section 7).

---

## 2. Portée / Scope

- **Applicable à :** Toute implémentation, configuration, ou utilisation de Border Guard
- **Objectif :** Identifier et prévenir les violations architecturales
- **Public cible :** Développeurs, architectes, auditeurs, responsables sécurité
- **Ne couvre pas :** Les violations des autres cores (voir leurs documents respectifs)

---

## 3. Taxonomie des violations

### 3.1 Qu'est-ce qu'une violation ?

Une **violation** est un acte ou une implémentation qui contredit un invariant de Border Guard. Toute violation :

- **Est une faute architecturale** — Pas une simple erreur de code
- **Doit être corrigée immédiatement** — Pas de compromis temporaire
- **Compromet l'intégrité du système** — Effets potentiellement cascadants
- **Est détectable** — Des critères permettent de l'identifier

### 3.2 Classification des violations

| Niveau | Gravité | Impact | Action requise |
|--------|---------|--------|----------------|
| **CRITIQUE** | Bloquant | Système incohérent, sécurité compromise | Arrêt et correction immédiate |
| **MAJEURE** | Sérieux | Comportement imprévisible, garanties non respectées | Correction prioritaire |
| **MINEURE** | Modéré | Dégradation de qualité, dette technique | Correction planifiée |

---

## 4. Ce que Border Guard ne fait PAS

Cette section reprend et détaille les exclusions fondamentales de Border Guard.

### 4.1 Border Guard ne FILTRE PAS

**Violation de :** INV-BG-1 (Aucune capacité d'exécution), INV-BG-7 (Séparation définition/application)

**Principe :**

> Border Guard ne filtre **jamais** les interactions. Le filtrage est une action d'application, pas de définition. Border Guard définit les règles de filtrage ; BondingBrother les applique.

**Symptômes de violation :**

| Symptôme | Exemple | Gravité |
|----------|---------|---------|
| Code de filtrage dans Border Guard | `if (!isAllowed(request)) { reject(); }` | CRITIQUE |
| Border Guard intercepte des données | Middleware Border Guard qui inspecte les payloads | CRITIQUE |
| Logique conditionnelle d'exclusion | `filter(data => data.trustLevel >= VERIFIED)` | MAJEURE |

**Pourquoi c'est interdit :**

- Confusion des responsabilités entre définition et exécution
- Impossible de modifier le filtrage indépendamment des règles
- Couplage fort entre Border Guard et les flux de données

**Correction :**

```
❌ VIOLATION:
BorderGuard.filterIncomingData(data) {
  return data.filter(d => this.rules.allows(d));
}

✅ CORRECT:
BorderGuard.getRulesFor(boundary) {
  return this.boundaries[boundary].rules;
}

BondingBrother.filterWithRules(data, rules) {
  return data.filter(d => rules.allows(d));
}
```

### 4.2 Border Guard ne BLOQUE PAS

**Violation de :** INV-BG-1 (Aucune capacité d'exécution), INV-BG-3 (Aucune décision autonome)

**Principe :**

> Border Guard ne bloque **jamais** les accès. Le blocage est une action d'exécution. Border Guard définit les conditions qui peuvent conduire à un blocage ; BondingBrother ou StrongFather exécute le blocage.

**Symptômes de violation :**

| Symptôme | Exemple | Gravité |
|----------|---------|---------|
| Méthode `block()` dans Border Guard | `borderGuard.blockSource(sourceId)` | CRITIQUE |
| Exception de blocage lancée par BG | `throw new AccessBlockedException()` | CRITIQUE |
| Border Guard retourne false/denied | `return { access: 'denied' }` | MAJEURE |

**Pourquoi c'est interdit :**

- Border Guard usurpe le rôle de StrongFather (décision)
- Border Guard usurpe le rôle de BondingBrother (exécution)
- Impossible de tracer qui a réellement bloqué

**Correction :**

```
❌ VIOLATION:
BorderGuard.checkAccess(source) {
  if (this.getTrustLevel(source) === HOSTILE) {
    throw new AccessBlocked("Source is hostile");
  }
}

✅ CORRECT:
BorderGuard.getContext(source) {
  return {
    trustLevel: this.getTrustLevel(source),
    boundary: this.getBoundaryFor(source),
    rules: this.getRulesFor(source)
  };
}

StrongFather.evaluateAccess(context) {
  // Décision basée sur le contexte
  if (context.trustLevel === HOSTILE) {
    return Decision.REFUSED;
  }
}
```

### 4.3 Border Guard n'AUTHENTIFIE PAS

**Violation de :** INV-BG-1 (Aucune capacité d'exécution), INV-BG-10 (Neutralité conceptuelle)

**Principe :**

> Border Guard ne gère **jamais** l'authentification technique. L'authentification (tokens, sessions, OAuth, JWT) est du ressort du produit ou d'un module auth dédié. Border Guard définit les niveaux de confiance ; l'authentification technique détermine comment atteindre ces niveaux.

**Symptômes de violation :**

| Symptôme | Exemple | Gravité |
|----------|---------|---------|
| Validation de tokens dans BG | `borderGuard.validateJWT(token)` | CRITIQUE |
| Gestion de sessions par BG | `borderGuard.checkSession(sessionId)` | CRITIQUE |
| Référence à OAuth/JWT dans les règles | `rule: "JWT must be RS256 signed"` | MAJEURE |
| Vérification de credentials par BG | `borderGuard.verifyPassword(hash)` | CRITIQUE |

**Pourquoi c'est interdit :**

- Couplage avec une technologie d'authentification spécifique
- Border Guard devient dépendant de bibliothèques crypto
- Impossible de changer de méthode d'auth sans modifier Border Guard

**Correction :**

```
❌ VIOLATION (règle):
{
  boundary: "api-external",
  rule: "JWT token must be valid and signed with RS256"
}

✅ CORRECT (règle déclarative):
{
  boundary: "api-external",
  rule: "Authentication required",
  requiredTrustLevel: "verified"
}
```

### 4.4 Border Guard ne PERSISTE PAS

**Violation de :** INV-BG-2 (Aucune persistance directe)

**Principe :**

> Border Guard ne persiste **jamais** de données. La persistance est du ressort exclusif de KindMother. Border Guard définit des frontières et des règles ; leur stockage est délégué à KindMother.

**Symptômes de violation :**

| Symptôme | Exemple | Gravité |
|----------|---------|---------|
| Accès base de données dans BG | `await db.query("SELECT * FROM boundaries")` | CRITIQUE |
| Écriture fichier par BG | `fs.writeFileSync('rules.json', rules)` | CRITIQUE |
| Cache persisté dans BG | `redis.set('boundary:123', boundary)` | MAJEURE |
| Import de drivers DB dans BG | `import { supabase } from '@supabase/client'` | MAJEURE |

**Pourquoi c'est interdit :**

- Violation de la souveraineté de KindMother sur les données
- Risque de désynchronisation entre Border Guard et la persistance
- Complexification de la gestion des données

**Correction :**

```
❌ VIOLATION:
class BorderGuard {
  async saveBoundary(boundary) {
    await this.db.boundaries.insert(boundary);
  }
}

✅ CORRECT:
class BorderGuard {
  defineBoundary(boundary) {
    // Définition en mémoire
    this.boundaries.set(boundary.id, boundary);
    // Notification pour persistance
    this.emit('boundary-defined', boundary);
  }
}

class KindMother {
  onBoundaryDefined(boundary) {
    await this.storage.persist('boundaries', boundary);
  }
}
```

### 4.5 Border Guard ne DÉCIDE PAS

**Violation de :** INV-BG-3 (Aucune décision autonome)

**Principe :**

> Border Guard ne prend **jamais** de décision stratégique ou politique. La décision est du ressort exclusif de StrongFather. Border Guard informe sur le contexte de confiance ; StrongFather décide.

**Symptômes de violation :**

| Symptôme | Exemple | Gravité |
|----------|---------|---------|
| Méthode `decide()` dans BG | `borderGuard.decideAccess(request)` | CRITIQUE |
| Retour accept/reject par BG | `return { decision: 'accept' }` | CRITIQUE |
| Logique if/else décisionnelle | `if (risk > threshold) return false` | MAJEURE |
| BG émet des verdicts | `emit('verdict', { allowed: true })` | MAJEURE |

**Pourquoi c'est interdit :**

- Usurpation du rôle de StrongFather
- Décisions prises sans vision globale du système
- Impossibilité d'appel ou de révision des décisions

**Correction :**

```
❌ VIOLATION:
BorderGuard.evaluateRequest(request) {
  const trust = this.getTrustLevel(request.source);
  const risk = this.assessRisk(request);
  
  if (trust < VERIFIED || risk > HIGH) {
    return { decision: 'REJECT', reason: 'Insufficient trust' };
  }
  return { decision: 'ACCEPT' };
}

✅ CORRECT:
BorderGuard.provideContext(request) {
  return {
    trustLevel: this.getTrustLevel(request.source),
    boundaryType: this.getBoundaryType(request),
    applicableRules: this.getRules(request),
    // Pas de décision, juste du contexte
  };
}

StrongFather.evaluate(request) {
  const context = BorderGuard.provideContext(request);
  // StrongFather prend la décision
  return this.makeDecision(request, context);
}
```

### 4.6 Border Guard n'EXÉCUTE PAS

**Violation de :** INV-BG-1 (Aucune capacité d'exécution)

**Principe :**

> Border Guard n'exécute **jamais** d'action technique. L'exécution est du ressort des cores opérationnels (BondingBrother, adaptateurs, produits). Border Guard est purement conceptuel.

**Symptômes de violation :**

| Symptôme | Exemple | Gravité |
|----------|---------|---------|
| Appels HTTP depuis BG | `await fetch(url)` | CRITIQUE |
| Manipulation de fichiers | `fs.chmod(file, mode)` | CRITIQUE |
| Envoi de notifications | `sendNotification(alert)` | MAJEURE |
| Appel de services externes | `await externalService.call()` | CRITIQUE |

**Pourquoi c'est interdit :**

- Border Guard doit rester purement conceptuel
- Toute exécution crée des dépendances externes
- Impossible de tester Border Guard en isolation

**Correction :**

```
❌ VIOLATION:
BorderGuard.notifyBreach(breach) {
  await this.alertService.send({
    type: 'SECURITY_BREACH',
    details: breach
  });
}

✅ CORRECT:
BorderGuard.signalBreach(breach) {
  // Émet un signal, ne fait pas l'action
  this.emit('breach-detected', {
    boundaryId: breach.boundary,
    severity: breach.severity,
    timestamp: Date.now()
  });
}

// Un autre composant gère l'action
AlertHandler.onBreach(signal) {
  await alertService.send(signal);
}
```

### 4.7 Border Guard ne MODIFIE PAS L'ÉTAT

**Violation de :** INV-BG-1, INV-BG-3

**Principe :**

> Border Guard ne modifie **jamais** l'état du système. L'observation de l'état est du ressort de CaringNanny, la modification de l'état est du ressort des cores exécutants. Border Guard définit, il ne modifie pas.

**Symptômes de violation :**

| Symptôme | Exemple | Gravité |
|----------|---------|---------|
| BG change l'état système | `systemState.securityLevel = HIGH` | CRITIQUE |
| BG active/désactive des modes | `this.quarantineMode = true` | MAJEURE |
| BG modifie des configurations | `config.set('boundary.open', false)` | MAJEURE |

**Pourquoi c'est interdit :**

- L'état est observé par CaringNanny, pas modifié par Border Guard
- Modifications non coordonnées créent des incohérences
- Impossible de tracer qui a modifié quoi

### 4.8 Border Guard ne contient PAS DE LOGIQUE MÉTIER

**Violation de :** INV-BG-10 (Neutralité conceptuelle)

**Principe :**

> Border Guard ne contient **jamais** de logique métier spécifique aux produits. Il définit des concepts généraux (frontières, confiance, règles) applicables à tous les produits. La logique métier spécifique reste dans les produits.

**Symptômes de violation :**

| Symptôme | Exemple | Gravité |
|----------|---------|---------|
| Règles spécifiques à un produit | `if (product === 'ecommerce') { ... }` | MAJEURE |
| Référence à des entités métier | `rule: "Orders > 1000€ require admin"` | MAJEURE |
| Logique de pricing/billing | `if (subscription.tier === 'premium')` | MAJEURE |
| Workflows spécifiques | `if (step === 'checkout') { ... }` | MINEURE |

**Pourquoi c'est interdit :**

- Border Guard doit rester générique et réutilisable
- Couplage avec un produit rend Border Guard non portable
- Violation du principe de séparation des préoccupations

---

## 5. Anti-patterns de définition

### 5.1 AP-01 : Frontière implicite

**Violation de :** INV-BG-5 (Frontières explicites)

**Description :** Une frontière existe de fait dans le système mais n'est pas formellement définie par Border Guard.

**Symptômes :**

- Code qui vérifie des permissions sans frontière déclarée
- Zones de confiance non documentées
- Points d'entrée non référencés dans Border Guard

**Exemple de violation :**

```
❌ ANTI-PATTERN:
// Dans un service, sans frontière définie
if (request.isAdmin) {
  // Zone admin implicite
  return adminData;
}
```

**Correction :**

```
✅ CORRECT:
// Frontière explicitement définie
BorderGuard.defineBoundary({
  id: 'admin-zone',
  type: 'internal',
  sourceZone: 'user-zone',
  destinationZone: 'admin-zone',
  rules: ['admin-role-required']
});
```

### 5.2 AP-02 : Frontière flottante

**Violation de :** INV-BG-5, INV-BG-9 (Cohérence globale)

**Description :** Une frontière définie sans zones clairement identifiées.

**Symptômes :**

- Frontière sans `sourceZone`
- Frontière sans `destinationZone`
- Zones référencées inexistantes

**Exemple de violation :**

```
❌ ANTI-PATTERN:
{
  id: 'my-boundary',
  type: 'external',
  // Pas de sourceZone ni destinationZone
  rules: ['auth-required']
}
```

### 5.3 AP-03 : Frontière sans règles

**Violation de :** INV-BG-5

**Description :** Une frontière définie sans aucune règle de franchissement.

**Symptômes :**

- Propriété `rules` vide ou absente
- Frontière qui ne contrôle rien
- Point d'entrée sans vérification

**Exemple de violation :**

```
❌ ANTI-PATTERN:
{
  id: 'api-boundary',
  type: 'external',
  sourceZone: 'internet',
  destinationZone: 'app',
  rules: []  // Vide !
}
```

### 5.4 AP-04 : Frontière technique

**Violation de :** INV-BG-10 (Neutralité conceptuelle)

**Description :** Une frontière définie en termes techniques plutôt que conceptuels.

**Symptômes :**

- Références à des ports, IPs, protocoles
- Mention de technologies spécifiques
- Configuration technique dans la définition

**Exemple de violation :**

```
❌ ANTI-PATTERN:
{
  id: 'api-boundary',
  type: 'external',
  implementation: 'nginx-reverse-proxy',  // Technique !
  port: 443,                               // Technique !
  protocol: 'HTTPS'                        // Technique !
}
```

### 5.5 AP-05 : Règle procédurale

**Violation de :** INV-BG-6 (Règles déclaratives)

**Description :** Une règle qui décrit comment faire plutôt que ce qui est requis.

**Symptômes :**

- Verbes d'action dans la règle
- Séquence d'étapes décrites
- Pseudo-code dans la définition

**Exemple de violation :**

```
❌ ANTI-PATTERN:
{
  rule: "1. Extract JWT from header\n2. Decode base64\n3. Verify signature with public key\n4. Check expiration"
}

✅ CORRECT:
{
  rule: "Authentication token required and valid"
}
```

### 5.6 AP-06 : Classification absente

**Violation de :** INV-BG-4 (Classification exhaustive)

**Description :** Une source ou interaction traitée sans classification de confiance.

**Symptômes :**

- Accès accordé sans vérification du niveau
- Source inconnue traitée comme verified
- Absence de défaut sur unknown

**Exemple de violation :**

```
❌ ANTI-PATTERN:
// Pas de classification
processRequest(request) {
  // Traitement direct sans vérifier la confiance
  return handleRequest(request);
}
```

### 5.7 AP-07 : Traçabilité manquante

**Violation de :** INV-BG-8 (Traçabilité complète)

**Description :** Une définition sans métadonnées de traçabilité.

**Symptômes :**

- Pas de `createdAt`
- Pas de `createdBy`
- Pas de `justification`
- Pas d'historique des modifications

**Exemple de violation :**

```
❌ ANTI-PATTERN:
{
  id: 'critical-boundary',
  type: 'internal',
  rules: ['admin-only']
  // Aucune traçabilité !
}

✅ CORRECT:
{
  id: 'critical-boundary',
  type: 'internal',
  rules: ['admin-only'],
  metadata: {
    createdAt: '2026-01-28T10:00:00Z',
    createdBy: 'system/security-policy',
    justification: 'Protection des données critiques niveau 4',
    version: '1.0'
  }
}
```

### 5.8 AP-08 : Règles contradictoires

**Violation de :** INV-BG-9 (Cohérence globale)

**Description :** Deux règles sur la même frontière qui se contredisent.

**Symptômes :**

- Une règle permet ce qu'une autre interdit
- Conditions mutuellement exclusives sur la même frontière
- Résultats différents selon l'ordre d'évaluation

**Exemple de violation :**

```
❌ ANTI-PATTERN:
{
  boundary: 'api-external',
  rules: [
    { id: 'rule-1', condition: 'trustLevel >= verified', action: 'allow' },
    { id: 'rule-2', condition: 'trustLevel == verified', action: 'deny' }
    // Contradiction : verified est >= verified ET == verified
  ]
}
```

---

## 6. Matrice de détection

### 6.1 Checklist de vérification

| # | Vérification | Invariant | Gravité si violé |
|---|--------------|-----------|------------------|
| 1 | Border Guard ne contient pas de code de filtrage | INV-BG-1 | CRITIQUE |
| 2 | Border Guard ne lance pas d'exceptions de blocage | INV-BG-1 | CRITIQUE |
| 3 | Border Guard n'accède pas à la base de données | INV-BG-2 | CRITIQUE |
| 4 | Border Guard ne retourne pas de décision (accept/reject) | INV-BG-3 | CRITIQUE |
| 5 | Toute source a un niveau de confiance | INV-BG-4 | MAJEURE |
| 6 | Toute frontière est explicitement définie | INV-BG-5 | MAJEURE |
| 7 | Toute règle est déclarative | INV-BG-6 | MAJEURE |
| 8 | Aucun code d'application dans Border Guard | INV-BG-7 | CRITIQUE |
| 9 | Toute définition a des métadonnées de traçabilité | INV-BG-8 | MINEURE |
| 10 | Aucune contradiction entre définitions | INV-BG-9 | MAJEURE |
| 11 | Aucune référence technique dans les définitions | INV-BG-10 | MAJEURE |

### 6.2 Signaux d'alerte dans le code

| Signal | Fichier/Module suspect | Action |
|--------|------------------------|--------|
| `import { db }` | Border Guard | Vérifier INV-BG-2 |
| `throw.*Block` | Border Guard | Vérifier INV-BG-1 |
| `return.*decision` | Border Guard | Vérifier INV-BG-3 |
| `JWT\|token\|session` | Règles de franchissement | Vérifier INV-BG-10 |
| `filter\|block\|deny` | Méthodes Border Guard | Vérifier INV-BG-1 |
| `if.*then.*else` complexe | Logique Border Guard | Vérifier INV-BG-3 |

---

## 7. Procédure de correction

### 7.1 Étapes de correction d'une violation

```
┌──────────────────────────────────────────────────────────┐
│ 1. IDENTIFIER                                            │
│    - Quel invariant est violé ?                          │
│    - Quelle est la gravité ?                             │
│    - Quel est l'impact ?                                 │
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
│    - Ajouter les métadonnées manquantes                  │
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
│    - Ajouter à l'historique                              │
└──────────────────────────────────────────────────────────┘
```

### 7.2 Responsabilités de correction

| Violation de | Core responsable de l'action |
|--------------|------------------------------|
| INV-BG-1 (exécution) | Déplacer vers BondingBrother |
| INV-BG-2 (persistance) | Déplacer vers KindMother |
| INV-BG-3 (décision) | Déplacer vers StrongFather |
| INV-BG-4 (classification) | Ajouter classification dans Border Guard |
| INV-BG-5 (frontières) | Définir explicitement dans Border Guard |
| INV-BG-6 (règles) | Reformuler de manière déclarative |
| INV-BG-7 (séparation) | Séparer définition et application |
| INV-BG-8 (traçabilité) | Ajouter métadonnées |
| INV-BG-9 (cohérence) | Résoudre contradictions |
| INV-BG-10 (neutralité) | Abstraire les détails techniques |

---

## 8. Références croisées

### Documents associés

| Document | Relation |
|----------|----------|
| [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) | Document source (Section 6, 7) |
| [Border Guard - Invariants & Guarantees](./Border%20Guard%20-%20Invariants%20&%20Guarantees.md) | Définition des invariants violés |
| [Border Guard - BondingBrother Integration Contract](../integration/Border%20Guard%20-%20BondingBrother%20Integration%20Contract.md) | Où déplacer l'exécution |
| [Border Guard - StrongFather Integration Contract](../integration/Border%20Guard%20-%20StrongFather%20Integration%20Contract.md) | Où déplacer les décisions |
| [Border Guard - KindMother Integration Contract](../integration/Border%20Guard%20-%20KindMother%20Integration%20Contract.md) | Où déplacer la persistance |

### Références glossaire

| Terme | Définition |
|-------|------------|
| **Violation** | Acte ou implémentation qui contredit un invariant |
| **Anti-pattern** | Pattern de conception à éviter car contraire aux invariants |
| **Gravité** | Niveau d'impact d'une violation (critique, majeure, mineure) |
| **Faute architecturale** | Erreur de conception qui compromet l'intégrité du système |

---

## 9. Synthèse contractuelle

### Engagements de ce contrat

Ce contrat établit que :

1. **Les violations sont identifiées** — 8 exclusions fondamentales documentées
2. **Les anti-patterns sont catalogués** — 8 anti-patterns avec exemples et corrections
3. **La détection est systématique** — Checklist et signaux d'alerte fournis
4. **La correction est guidée** — Procédure en 5 étapes avec responsabilités
5. **Les gravités sont classifiées** — CRITIQUE, MAJEURE, MINEURE

### Phrase de synthèse

> **Border Guard ne filtre pas, ne bloque pas, n'authentifie pas, ne persiste pas, ne décide pas, n'exécute pas, ne modifie pas l'état, et ne contient pas de logique métier — toute violation de ces exclusions est une faute architecturale qui doit être immédiatement corrigée.**

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Contrat — Préventif  
**Référence :** Border Guard v1.5, Documentation Fondatrice Section 6, Section 7  
**Type :** Contrat de gouvernance — Violations et Anti-Patterns
