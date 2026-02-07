# Border Guard — Examples & Use Cases

## Contexte

Ce document présente des **exemples concrets** et **cas d'usage** illustrant le fonctionnement de Border Guard. Ces exemples sont purement conceptuels et n'imposent aucune implémentation technique.

**Document fondateur :** [Border Guard - Documentation Fondatrice](../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

**Statut :** Document de référence — Informatif et pédagogique

---

## Portée / Scope

- **Applicable à :** Développeurs, architectes, et toute personne souhaitant comprendre Border Guard en pratique
- **Objectif :** Illustrer les concepts par des scénarios concrets
- **Nature :** Exemples conceptuels — aucune technologie prescrite
- **Avertissement :** Ces exemples sont informatifs et ne créent aucune nouvelle règle contractuelle

---

## Exemples de définition de frontières

### Exemple 1 : Frontière externe — API publique

**Scénario :** L'application Miyukini expose une API REST publique accessible depuis Internet.

**Définition de la frontière :**

```
FRONTIÈRE: FR-EXT-API-001
NOM: Frontière API publique
TYPE: Externe
DIRECTION: Entrée
PERMÉABILITÉ: Contrôlée

ZONES:
  - Source: Monde extérieur (Internet)
  - Destination: Écosystème Miyukini

RÈGLES ASSOCIÉES:
  - EXT-001: source.trust_level ≠ HOSTILE
  - EXT-002: rate.requests_per_minute ≤ 100
  - EXT-003: source.trust_level ≥ UNKNOWN

TRAÇABILITÉ:
  - Origine: BorderGuard/SecurityPolicy
  - Créé le: 2026-01-28
  - Justification: Protection de l'API contre les accès malveillants
```

**Ce que Border Guard fait :**
- Définit formellement la frontière avec toutes ses propriétés
- Établit les règles déclaratives de franchissement
- Classifie les sources entrantes (unknown par défaut)

**Ce que Border Guard ne fait PAS :**
- Filtrer les requêtes HTTP
- Implémenter le rate limiting
- Bloquer les IP malveillantes

**Qui fait quoi ensuite :**
- BondingBrother consulte cette définition et applique les règles
- StrongFather décide de l'autorisation finale si une règle échoue (DEFER)

---

### Exemple 2 : Frontière interne — Zone admin

**Scénario :** L'espace d'administration est séparé de l'espace utilisateur standard.

**Définition de la frontière :**

```
FRONTIÈRE: FR-INT-ADMIN-001
NOM: Frontière zone administration
TYPE: Interne
DIRECTION: Entrée
PERMÉABILITÉ: Contrôlée

ZONES:
  - Source: Zone utilisateur (verified)
  - Destination: Zone administration (verified+)

RÈGLES ASSOCIÉES:
  - INT-001: source.trust_level ≠ HOSTILE
  - INT-002: source.role = ADMIN
  - INT-003: source.auth_method IN [MFA, SSO]
  - INT-004: session.age < 30min

TRAÇABILITÉ:
  - Origine: BorderGuard/AdminPolicy
  - Créé le: 2026-01-28
  - Justification: Protection de l'espace d'administration
```

**Illustration du flux :**

```
Utilisateur standard
       │
       │ Tente d'accéder à /admin
       ▼
┌─────────────────────────────────────┐
│ Border Guard : contexte de frontière │
│ - Frontière: FR-INT-ADMIN-001       │
│ - Source: verified (utilisateur)    │
│ - Règles: INT-001, INT-002, etc.    │
└─────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────┐
│ BondingBrother : application        │
│ - Vérifie INT-001: OK (pas hostile) │
│ - Vérifie INT-002: FAIL (pas admin) │
│ - Action: DEFER à StrongFather      │
└─────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────┐
│ StrongFather : décision             │
│ - Contexte: utilisateur non admin   │
│ - Décision: REFUSÉ                  │
└─────────────────────────────────────┘
```

---

### Exemple 3 : Frontière d'intégration — Supabase

**Scénario :** L'application utilise Supabase comme backend.

**Définition de la frontière :**

```
FRONTIÈRE: FR-INTEG-SUPABASE-001
NOM: Frontière intégration Supabase
TYPE: Intégration
DIRECTION: Bidirectionnelle
PERMÉABILITÉ: Contrôlée

ZONES:
  - Source/Destination: Écosystème Miyukini ↔ Supabase

CLASSIFICATION DE L'INTÉGRATION:
  - Niveau de confiance initial: VERIFIED
  - État: ACTIVE

RÈGLES ASSOCIÉES:
  - INTEG-001: integration.status = ACTIVE
  - INTEG-002: source.trust_level ≥ VERIFIED
  - INTEG-003: integration.credentials_valid = true
  - INTEG-004: data.classification ≤ SENSITIVE

TRAÇABILITÉ:
  - Origine: BorderGuard/IntegrationPolicy
  - Créé le: 2026-01-28
  - Justification: Connexion sécurisée au backend Supabase
```

**Ce que Border Guard garantit :**
- L'intégration est explicitement définie
- Le niveau de confiance est établi (VERIFIED)
- Les règles d'échange sont documentées
- L'état de l'intégration est traçable

---

## Exemples de classification de confiance

### Exemple 4 : Classification d'une requête externe

**Scénario :** Une requête HTTP arrive sur l'API publique.

**Flux de classification :**

```
Requête HTTP entrante (sans authentification)
       │
       ▼
┌─────────────────────────────────────┐
│ Border Guard : classification       │
│                                     │
│ 1. Est-ce une source blacklistée ?  │
│    → NON                            │
│                                     │
│ 2. Est-ce un composant interne ?    │
│    → NON                            │
│                                     │
│ 3. Authentification valide ?        │
│    → NON (pas de token)             │
│                                     │
│ Résultat: UNKNOWN                   │
└─────────────────────────────────────┘
       │
       ▼
Niveau de confiance: UNKNOWN 🟡
```

**Impact :**
- Les règles restrictives par défaut s'appliquent
- L'accès est limité aux ressources publiques
- Des vérifications systématiques sont requises pour élever le niveau

---

### Exemple 5 : Classification après authentification

**Scénario :** Un utilisateur se connecte avec succès.

**Flux de classification :**

```
Authentification réussie
       │
       ▼
┌─────────────────────────────────────┐
│ Module Auth : résultat              │
│ - Identité: user@example.com        │
│ - Méthode: MFA                      │
│ - Session: valide                   │
└─────────────────────────────────────┘
       │
       │ Notification à Border Guard
       ▼
┌─────────────────────────────────────┐
│ Border Guard : reclassification     │
│                                     │
│ 1. Source blacklistée ?             │
│    → NON                            │
│                                     │
│ 2. Composant interne ?              │
│    → NON                            │
│                                     │
│ 3. Authentification valide ?        │
│    → OUI (MFA réussi)               │
│                                     │
│ 4. Contexte cohérent ?              │
│    → OUI (device connu, session ok) │
│                                     │
│ Résultat: VERIFIED                  │
│                                     │
│ Transition: UNKNOWN → VERIFIED      │
│ Traçabilité: enregistrée            │
└─────────────────────────────────────┘
       │
       ▼
Niveau de confiance: VERIFIED 🔵
```

---

### Exemple 6 : Détection d'une source hostile

**Scénario :** Un pattern d'attaque est détecté sur une source.

**Flux de classification :**

```
Détection d'anomalie (pattern d'attaque)
       │
       ▼
┌─────────────────────────────────────┐
│ Système de détection : alerte       │
│ - Source: IP 192.168.x.x            │
│ - Pattern: Tentative de brute force │
│ - Confiance précédente: UNKNOWN     │
└─────────────────────────────────────┘
       │
       │ Signal à Border Guard
       ▼
┌─────────────────────────────────────┐
│ Border Guard : reclassification     │
│                                     │
│ Critères de classification HOSTILE: │
│ - Pattern d'attaque détecté: OUI    │
│                                     │
│ Résultat: HOSTILE                   │
│                                     │
│ Transition: UNKNOWN → HOSTILE       │
│ (transition immédiate TRANS-2)      │
│                                     │
│ Traçabilité:                        │
│ - Raison: Pattern brute force       │
│ - Date: 2026-01-28T14:30:00Z        │
│ - Preuve: log_id_12345              │
└─────────────────────────────────────┘
       │
       ▼
Niveau de confiance: HOSTILE 🔴

       │
       │ Notification à CaringNanny
       ▼
État système mis à jour
```

**Impact :**
- Blocage systématique de toute interaction depuis cette source
- Journalisation de toutes les tentatives
- Alerte aux administrateurs
- Processus formel requis pour réhabilitation

---

## Exemples de règles de franchissement

### Exemple 7 : Règle de niveau de confiance

**Scénario :** Une frontière nécessite au minimum le niveau "verified".

**Définition de la règle :**

```
RÈGLE: CROSS-TRUST-001
NOM: Niveau de confiance minimum pour données sensibles
FRONTIÈRE: FR-INT-SENSITIVE-001
CONDITION: source.trust_level ≥ VERIFIED
TYPE: niveau_confiance
PRIORITÉ: 15
ÉCHEC: DENY
```

**Évaluation :**

| Source | Niveau | Résultat |
|--------|--------|----------|
| Visiteur anonyme | UNKNOWN | ❌ DENY |
| Utilisateur authentifié | VERIFIED | ✅ PASS |
| Core système | TRUSTED | ✅ PASS |
| Source blacklistée | HOSTILE | ❌ DENY (règle prioritaire) |

---

### Exemple 8 : Règle d'authentification renforcée

**Scénario :** L'accès à la zone critique nécessite une authentification MFA.

**Définition de la règle :**

```
RÈGLE: CROSS-AUTH-003
NOM: Authentification renforcée requise
FRONTIÈRE: FR-INT-CRITICAL-001
CONDITION: source.auth_method IN [MFA, SSO]
TYPE: authentification
PRIORITÉ: 35
ÉCHEC: DEFER
```

**Note importante :** Border Guard ne vérifie PAS techniquement le MFA. Il définit la condition "authentification renforcée requise". BondingBrother implémente la vérification technique.

---

### Exemple 9 : Règle de protection des données

**Scénario :** Les données critiques ne peuvent pas traverser une frontière externe.

**Définition de la règle :**

```
RÈGLE: CROSS-DATA-002
NOM: Protection des données critiques
FRONTIÈRE: FR-EXT-*  (toutes frontières externes)
CONDITION: data.classification ≤ SENSITIVE
TYPE: donnees
PRIORITÉ: 55
ÉCHEC: DENY

EXCEPTION:
  ID: EXC-DATA-001
  CONDITION: destination.is_backup_service = true
  JUSTIFICATION: Sauvegarde autorisée vers service certifié
  DURÉE: Permanente
```

**Évaluation :**

| Données | Classification | Destination | Résultat |
|---------|---------------|-------------|----------|
| Profil utilisateur | PUBLIC | API externe | ✅ PASS |
| Préférences | SENSITIVE | API partenaire | ✅ PASS |
| Clés de chiffrement | CRITICAL | API externe | ❌ DENY |
| Clés de chiffrement | CRITICAL | Service backup | ✅ PASS (exception) |

---

### Exemple 10 : Règle temporelle (rate limiting)

**Scénario :** Limitation du débit sur les frontières externes.

**Définition de la règle :**

```
RÈGLE: CROSS-TIME-002
NOM: Rate limiting frontière externe
FRONTIÈRE: FR-EXT-API-001
CONDITION: rate.requests_per_minute ≤ 100
TYPE: temporel
PRIORITÉ: 5
ÉCHEC: DENY + ALERT
```

**Ce que Border Guard fait :**
- Définit la condition déclarative "maximum 100 requêtes par minute"

**Ce que BondingBrother fait :**
- Implémente le compteur technique
- Vérifie le débit en temps réel
- Applique le blocage si dépassé

---

## Exemples d'interactions entre cores

### Exemple 11 : Flux complet — Requête utilisateur

**Scénario :** Un utilisateur authentifié demande l'accès à des données sensibles.

```
┌─────────────────────────────────────────────────────────────────┐
│ ÉTAPE 1 : Requête entrante                                       │
│                                                                  │
│ Utilisateur authentifié demande GET /api/sensitive-data         │
└─────────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│ ÉTAPE 2 : BondingBrother identifie les frontières               │
│                                                                  │
│ BondingBrother détecte que la requête traverse:                 │
│ - FR-INT-SENSITIVE-001 (frontière données sensibles)            │
│                                                                  │
│ → Consulte Border Guard pour les règles                         │
└─────────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│ ÉTAPE 3 : Border Guard fournit le contexte                       │
│                                                                  │
│ Border Guard retourne:                                           │
│ - Frontière: FR-INT-SENSITIVE-001                               │
│ - Source: user@example.com                                       │
│ - Niveau de confiance: VERIFIED                                  │
│ - Règles applicables:                                            │
│   - CROSS-TRUST-001: source.trust_level ≥ VERIFIED              │
│   - CROSS-DATA-001: data.type IN [text, json]                   │
│   - CROSS-TIME-003: session.age < 1h                            │
└─────────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│ ÉTAPE 4 : BondingBrother applique les règles                     │
│                                                                  │
│ Évaluation technique:                                            │
│ - CROSS-TRUST-001: VERIFIED ≥ VERIFIED → ✅ PASS                │
│ - CROSS-DATA-001: json IN [text, json] → ✅ PASS                │
│ - CROSS-TIME-003: 25min < 1h → ✅ PASS                          │
│                                                                  │
│ Toutes les règles satisfaites                                    │
└─────────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│ ÉTAPE 5 : StrongFather donne l'autorisation finale               │
│                                                                  │
│ StrongFather évalue:                                             │
│ - Contexte de confiance: OK (VERIFIED)                          │
│ - Règles: toutes satisfaites                                     │
│ - Intention: lecture de données (non destructif)                 │
│                                                                  │
│ Décision: AUTORISÉ                                               │
└─────────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│ ÉTAPE 6 : Réponse à l'utilisateur                                │
│                                                                  │
│ Les données sensibles sont retournées                            │
└─────────────────────────────────────────────────────────────────┘
```

---

### Exemple 12 : Flux de refus — Règle non satisfaite

**Scénario :** Un utilisateur tente d'accéder à des données critiques sans MFA.

```
┌─────────────────────────────────────────────────────────────────┐
│ Contexte:                                                        │
│ - Utilisateur: authenticated (password only, pas de MFA)        │
│ - Niveau de confiance: VERIFIED                                  │
│ - Destination: données critiques (nécessite MFA)                 │
└─────────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│ Border Guard : contexte de frontière                             │
│                                                                  │
│ Frontière: FR-INT-CRITICAL-001                                   │
│ Règles:                                                          │
│ - CROSS-AUTH-003: source.auth_method IN [MFA, SSO]              │
└─────────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│ BondingBrother : évaluation                                      │
│                                                                  │
│ - CROSS-AUTH-003: password NOT IN [MFA, SSO] → ❌ FAIL          │
│ - Action configurée: DEFER                                       │
│                                                                  │
│ → Soumet à StrongFather                                          │
└─────────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│ StrongFather : décision                                          │
│                                                                  │
│ Contexte:                                                        │
│ - Niveau de confiance: VERIFIED (mais auth faible)              │
│ - Règle échouée: authentification renforcée requise             │
│ - Destination: données critiques                                 │
│                                                                  │
│ Décision: REFUSÉ                                                 │
│ Raison: Authentification MFA requise pour accéder aux données   │
│         critiques                                                │
│ Action: Demander à l'utilisateur de s'authentifier avec MFA     │
└─────────────────────────────────────────────────────────────────┘
```

---

### Exemple 13 : Changement d'état d'une intégration

**Scénario :** L'intégration Supabase devient indisponible.

```
┌─────────────────────────────────────────────────────────────────┐
│ Détection: Supabase ne répond plus                               │
│ (Détection par le système de monitoring)                         │
└─────────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│ Border Guard : mise à jour de l'intégration                      │
│                                                                  │
│ Intégration: FR-INTEG-SUPABASE-001                               │
│ État précédent: ACTIVE                                           │
│ Nouvel état: SUSPENDUE                                           │
│                                                                  │
│ Règle INTEG-001 impactée:                                        │
│ - Condition: integration.status = ACTIVE                         │
│ - Résultat: FAIL pour toute requête vers Supabase               │
│                                                                  │
│ Traçabilité:                                                     │
│ - Raison: Timeout connexion                                      │
│ - Date: 2026-01-28T15:45:00Z                                    │
└─────────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│ Border Guard → CaringNanny : notification                        │
│                                                                  │
│ Événement: INTEGRATION_STATUS_CHANGED                            │
│ Intégration: FR-INTEG-SUPABASE-001                               │
│ Nouveau statut: SUSPENDUE                                        │
│ Impact: Fonctionnalités dépendantes indisponibles               │
└─────────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│ CaringNanny : mise à jour de l'état global                       │
│                                                                  │
│ État système: DEGRADED                                           │
│ Raison: Intégration Supabase suspendue                          │
│ Services impactés: persistence, auth                             │
└─────────────────────────────────────────────────────────────────┘
```

---

## Exemples d'adaptation selon le niveau de sécurité

### Exemple 14 : Même frontière, niveaux de sécurité différents

**Scénario :** La frontière externe API s'adapte selon le niveau de sécurité déclaré.

**Niveau 1 - STANDARD :**

```
FRONTIÈRE: FR-EXT-API-001
NIVEAU DE SÉCURITÉ: 1 (STANDARD)

RÈGLES ADAPTÉES:
- EXT-003: source.trust_level ≥ UNKNOWN
- Rate limit: 100 req/min
- Tolérance erreur: Standard
```

**Niveau 3 - CRITICAL :**

```
FRONTIÈRE: FR-EXT-API-001
NIVEAU DE SÉCURITÉ: 3 (CRITICAL)

RÈGLES ADAPTÉES:
- EXT-003: source.trust_level ≥ VERIFIED (auth obligatoire)
- EXT-ADD: source.auth_method IN [MFA, SSO] (auth renforcée)
- Rate limit: 20 req/min (strict)
- Tolérance erreur: Minimale
```

**Niveau 4 - HARDENED :**

```
FRONTIÈRE: FR-EXT-API-001
NIVEAU DE SÉCURITÉ: 4 (HARDENED)

RÈGLES ADAPTÉES:
- EXT-003: source.trust_level = TRUSTED (seuls trusted autorisés)
- Rate limit: 5 req/min (ultra-strict)
- Tolérance erreur: Zéro
- Perméabilité: Fermée par défaut
```

---

## Exemples d'anti-patterns (ce qu'il ne faut PAS faire)

### Exemple 15 : Anti-pattern — Filtrage dans Border Guard

**❌ MAUVAIS :**

```
// Dans Border Guard (INTERDIT)
function handleRequest(request) {
  const trustLevel = classifySource(request.source);
  if (trustLevel === 'HOSTILE') {
    return reject(request);  // ❌ Border Guard ne rejette JAMAIS
  }
  return accept(request);    // ❌ Border Guard ne décide JAMAIS
}
```

**✅ CORRECT :**

```
// Border Guard : classification uniquement
function classifySource(source) {
  // ... évaluation des critères ...
  return trustLevel;  // Retourne TRUSTED, VERIFIED, UNKNOWN, ou HOSTILE
}

// BondingBrother : application des règles
function handleRequest(request) {
  const context = borderGuard.getContext(request);
  const rules = borderGuard.getRules(context.boundary);
  
  for (const rule of rules) {
    if (!checkRule(rule, context)) {
      if (rule.failAction === 'DENY') {
        return reject(request);  // ✅ BondingBrother rejette
      }
      if (rule.failAction === 'DEFER') {
        return strongFather.decide(request, context);  // ✅ StrongFather décide
      }
    }
  }
  return process(request);
}
```

---

### Exemple 16 : Anti-pattern — Règle procédurale

**❌ MAUVAIS (procédural) :**

```
RÈGLE: CROSS-AUTH-BAD
CONDITION: |
  1. Extraire le token du header Authorization
  2. Décoder le JWT avec la clé RS256
  3. Vérifier que exp > now
  4. Vérifier que iss = "miyukini"
  5. Retourner true si toutes les étapes OK
```

**✅ CORRECT (déclaratif) :**

```
RÈGLE: CROSS-AUTH-001
CONDITION: source.authenticated = true AND source.session_valid = true
TYPE: authentification
```

La vérification technique (JWT, RS256, expiration) est du ressort de BondingBrother.

---

### Exemple 17 : Anti-pattern — Frontière implicite

**❌ MAUVAIS :**

```javascript
// Vérification de permission sans frontière définie
function getAdminData() {
  if (!user.isAdmin) {  // ❌ Frontière implicite
    throw new Error('Access denied');
  }
  return adminData;
}
```

**✅ CORRECT :**

```javascript
// Frontière explicitement définie dans Border Guard
// FR-INT-ADMIN-001: Frontière zone administration

// Dans le code : consultation de la frontière
function getAdminData(context) {
  const boundaryContext = borderGuard.getContext('FR-INT-ADMIN-001', context);
  const decision = bondingBrother.checkRules(boundaryContext);
  
  if (!decision.allowed) {
    throw new Error(decision.reason);
  }
  return adminData;
}
```

---

## Documents de référence

| Document | Relation |
|----------|----------|
| [Documentation Fondatrice](../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) | Concepts de base |
| [Boundary Definition Contract](../contracts/boundaries/Border%20Guard%20-%20Boundary%20Definition%20Contract.md) | Définition des frontières |
| [Trust Level Classification Contract](../contracts/boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) | Classification de confiance |
| [Crossing Rules Contract](../contracts/boundaries/Border%20Guard%20-%20Crossing%20Rules%20Contract.md) | Règles de franchissement |
| [Reference Implementation Guidelines](../implementation/Border%20Guard%20-%20Reference%20Implementation%20Guidelines.md) | Guide d'implémentation |
| [Violations & Anti-Patterns](../contracts/governance/Border%20Guard%20-%20Violations%20&%20Anti-Patterns.md) | Anti-patterns détaillés |
| [Vocabulary & Glossary](./Border%20Guard%20-%20Vocabulary%20&%20Glossary.md) | Définitions des termes |
| [FAQ & Common Questions](./Border%20Guard%20-%20FAQ%20&%20Common%20Questions.md) | Questions fréquentes |

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Document de référence — Exemples et cas d'usage  
**Référence :** Border Guard v1.5, Tous les contrats FONDATION
