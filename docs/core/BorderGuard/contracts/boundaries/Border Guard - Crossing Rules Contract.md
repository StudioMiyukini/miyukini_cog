# Border Guard - Crossing Rules Contract

## 1. Contexte

Ce document définit les **règles de franchissement** gouvernées par Border Guard dans l'écosystème Miyukini. Il spécifie formellement ce qu'est une règle de franchissement, sa structure, ses types, et les conditions d'application selon les frontières et les niveaux de confiance.

**Document fondateur :** [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

**Statut contractuel :** Ce document est **contractuel, normatif, et non négociable**. Il dérive directement de la Documentation Fondatrice (Section 4 - Concepts fondamentaux : Règle de franchissement).

---

## 2. Portée / Scope

- **Applicable à :** Toute interaction traversant une frontière dans l'écosystème Miyukini
- **Responsable :** Border Guard (responsabilité exclusive d'établissement des règles - Documentation Fondatrice Section 5)
- **Exécutant :** BondingBrother (application des règles - INV-BG-7)
- **Consommateurs :** BondingBrother (application), StrongFather (contexte de décision)
- **Ne couvre pas :** L'application technique des règles (responsabilité de BondingBrother)

---

## 3. Définition canonique de la règle de franchissement

### 3.1 Qu'est-ce qu'une règle de franchissement ?

Une **règle de franchissement** est une condition déclarative qui doit être satisfaite pour qu'une interaction puisse traverser une frontière. Elle exprime **ce qui est requis**, pas **comment le vérifier techniquement**.

**Caractéristiques fondamentales :**

1. **Déclarative** — Exprime une condition, pas une procédure
2. **Non ambiguë** — Spécifie clairement les conditions sans interprétation possible
3. **Associée** — Liée à une frontière spécifique ou un ensemble de frontières
4. **Indépendante** — Ne dépend pas de l'implémentation technique

**Ce qu'une règle de franchissement n'est PAS :**

- ❌ Un algorithme de validation
- ❌ Un code de vérification
- ❌ Une procédure d'authentification
- ❌ Une règle de filtrage technique

### 3.2 Responsabilité de Border Guard

Border Guard est **exclusivement responsable** de l'établissement des règles de franchissement. Cette responsabilité inclut :

- Définir les règles associées à chaque frontière
- Spécifier les conditions de franchissement
- Établir les exceptions et cas particuliers
- Maintenir la cohérence des règles entre frontières

**Invariant associé :** INV-BG-6 — Toutes les règles de franchissement **doivent** être déclaratives. Aucune règle procédurale ou impérative n'est autorisée.

### 3.3 Séparation définition/application

**Border Guard définit.** BondingBrother applique.

| Responsabilité | Border Guard | BondingBrother |
|----------------|--------------|----------------|
| Définir les règles | ✅ | ❌ |
| Spécifier les conditions | ✅ | ❌ |
| Implémenter la vérification | ❌ | ✅ |
| Exécuter le blocage | ❌ | ✅ |
| Tracer l'application | ❌ | ✅ |

**Invariant associé :** INV-BG-7 — La définition des frontières et des règles est **strictement séparée** de leur application.

---

## 4. Structure d'une règle de franchissement

### 4.1 Propriétés obligatoires

Toute règle de franchissement possède les propriétés suivantes :

| Propriété | Description | Obligatoire |
|-----------|-------------|-------------|
| **Identifiant** | Identifiant unique de la règle | ✅ Oui |
| **Nom** | Nom descriptif de la règle | ✅ Oui |
| **Description** | Description de ce que la règle vérifie | ✅ Oui |
| **Frontière(s)** | Frontière(s) à laquelle/lesquelles la règle s'applique | ✅ Oui |
| **Condition** | Condition déclarative à satisfaire | ✅ Oui |
| **Type** | Type de règle (niveau, authentification, données, etc.) | ✅ Oui |
| **Priorité** | Priorité d'évaluation | ✅ Oui |
| **Action si échec** | Que faire si la condition n'est pas satisfaite | ✅ Oui |

### 4.2 Propriétés optionnelles

| Propriété | Description | Obligatoire |
|-----------|-------------|-------------|
| **Exceptions** | Cas où la règle ne s'applique pas | ❌ Non |
| **Conditions préalables** | Autres règles qui doivent être satisfaites avant | ❌ Non |
| **Niveau de sécurité** | Niveau de sécurité minimum pour appliquer cette règle | ❌ Non |
| **Métadonnées** | Informations supplémentaires | ❌ Non |

### 4.3 Format déclaratif

Une règle est exprimée de manière déclarative selon le format :

```
RÈGLE: <identifiant>
NOM: <nom descriptif>
FRONTIÈRE: <frontière(s) cible>
CONDITION: <condition déclarative>
TYPE: <type de règle>
PRIORITÉ: <1-100>
ÉCHEC: <action si échec>
```

**Exemple :**

```
RÈGLE: CROSS-EXT-001
NOM: Niveau de confiance minimum pour frontière externe
FRONTIÈRE: Frontières externes
CONDITION: source.trust_level ≥ VERIFIED
TYPE: niveau_confiance
PRIORITÉ: 10
ÉCHEC: REFUSER
```

---

## 5. Types de règles de franchissement

Border Guard reconnaît cinq types canoniques de règles.

### 5.1 Règles de niveau de confiance

**Définition :** Règles portant sur le niveau de confiance requis pour franchir une frontière.

| Aspect | Spécification |
|--------|---------------|
| **Code type** | `niveau_confiance` |
| **Condition type** | Comparaison de niveau (≥, =, ≤) |
| **Variables** | `source.trust_level`, `destination.trust_level` |

**Exemples :**

| Règle | Condition | Description |
|-------|-----------|-------------|
| CROSS-TRUST-001 | `source.trust_level ≥ VERIFIED` | Source doit être au moins VERIFIED |
| CROSS-TRUST-002 | `source.trust_level = TRUSTED` | Source doit être TRUSTED |
| CROSS-TRUST-003 | `source.trust_level ≠ HOSTILE` | Source ne doit pas être HOSTILE |

### 5.2 Règles d'authentification

**Définition :** Règles portant sur l'état d'authentification requis pour franchir une frontière.

| Aspect | Spécification |
|--------|---------------|
| **Code type** | `authentification` |
| **Condition type** | État d'authentification |
| **Variables** | `source.authenticated`, `source.auth_method`, `source.session_valid` |

**Exemples :**

| Règle | Condition | Description |
|-------|-----------|-------------|
| CROSS-AUTH-001 | `source.authenticated = true` | Source doit être authentifiée |
| CROSS-AUTH-002 | `source.session_valid = true` | Session doit être valide |
| CROSS-AUTH-003 | `source.auth_method IN [MFA, SSO]` | Méthode d'auth renforcée requise |

**Note :** Border Guard ne gère pas l'authentification technique. Ces règles vérifient l'**état** d'authentification fourni par le module auth.

### 5.3 Règles de données

**Définition :** Règles portant sur les données autorisées à traverser une frontière.

| Aspect | Spécification |
|--------|---------------|
| **Code type** | `donnees` |
| **Condition type** | Nature ou classification des données |
| **Variables** | `data.classification`, `data.type`, `data.sensitivity` |

**Exemples :**

| Règle | Condition | Description |
|-------|-----------|-------------|
| CROSS-DATA-001 | `data.classification ≤ PUBLIC` | Seules données publiques autorisées |
| CROSS-DATA-002 | `data.sensitivity ≠ CRITICAL` | Données critiques interdites |
| CROSS-DATA-003 | `data.type IN [text, json, image]` | Types de données autorisés |

### 5.4 Règles d'action

**Définition :** Règles portant sur les actions autorisées à traverser une frontière.

| Aspect | Spécification |
|--------|---------------|
| **Code type** | `action` |
| **Condition type** | Nature de l'action |
| **Variables** | `action.type`, `action.scope`, `action.impact` |

**Exemples :**

| Règle | Condition | Description |
|-------|-----------|-------------|
| CROSS-ACT-001 | `action.type = READ` | Seule lecture autorisée |
| CROSS-ACT-002 | `action.impact ≤ LOW` | Actions à faible impact uniquement |
| CROSS-ACT-003 | `action.type NOT IN [DELETE, DROP]` | Actions destructives interdites |

### 5.5 Règles temporelles

**Définition :** Règles portant sur les conditions temporelles de franchissement.

| Aspect | Spécification |
|--------|---------------|
| **Code type** | `temporel` |
| **Condition type** | Contraintes de temps |
| **Variables** | `timestamp`, `session.age`, `rate.limit` |

**Exemples :**

| Règle | Condition | Description |
|-------|-----------|-------------|
| CROSS-TIME-001 | `session.age < 1h` | Session de moins d'une heure |
| CROSS-TIME-002 | `rate.requests_per_minute ≤ 100` | Limite de débit |
| CROSS-TIME-003 | `timestamp.hour IN [9, 18]` | Heures ouvrables uniquement |

---

## 6. Priorités et évaluation

### 6.1 Niveaux de priorité

Les règles sont évaluées selon leur priorité (1 = plus haute priorité).

| Plage | Description | Exemples |
|-------|-------------|----------|
| **1-10** | Règles de sécurité critiques | Blocage hostile, rate limiting |
| **11-30** | Règles de niveau de confiance | Vérification TRUSTED, VERIFIED |
| **31-50** | Règles d'authentification | Session valide, MFA |
| **51-70** | Règles de données | Classification, types |
| **71-90** | Règles d'action | Lecture seule, impact |
| **91-100** | Règles temporelles et autres | Heures, quotas |

### 6.2 Algorithme d'évaluation

```
POUR chaque règle R ordonnée par priorité :
    SI R.frontière correspond à la frontière traversée :
        SI NON satisfaite(R.condition) :
            RETOURNER R.action_echec
        FIN SI
    FIN SI
FIN POUR
RETOURNER AUTORISER
```

**Règle fondamentale :** L'évaluation s'arrête à la première règle non satisfaite.

### 6.3 Combinaison de règles

| Combinaison | Description | Comportement |
|-------------|-------------|--------------|
| **ET (implicite)** | Toutes les règles doivent être satisfaites | Échec si une seule échoue |
| **OU (explicite)** | Au moins une règle doit être satisfaite | Règles alternatives |
| **EXCEPTION** | La règle ne s'applique pas dans ce cas | Bypass contrôlé |

---

## 7. Actions en cas d'échec

### 7.1 Actions canoniques

| Action | Code | Description | Gravité |
|--------|------|-------------|---------|
| **REFUSER** | `DENY` | Refuser le franchissement | Élevée |
| **DIFFÉRER** | `DEFER` | Soumettre à StrongFather pour décision | Moyenne |
| **DÉGRADER** | `DEGRADE` | Autoriser avec restrictions | Faible |
| **ALERTER** | `ALERT` | Autoriser mais alerter | Information |
| **JOURNALISER** | `LOG` | Autoriser et journaliser | Information |

### 7.2 Matrice action/gravité

| Gravité de la violation | Action recommandée |
|------------------------|-------------------|
| Critique (HOSTILE, violation grave) | `DENY` |
| Élevée (règle de sécurité) | `DENY` ou `DEFER` |
| Moyenne (règle d'authentification) | `DEFER` ou `DEGRADE` |
| Faible (règle de données/action) | `DEGRADE` ou `ALERT` |
| Information (règle temporelle) | `ALERT` ou `LOG` |

### 7.3 Escalade

```
Échec règle
    │
    ├── Gravité critique ───► DENY (immédiat)
    │
    ├── Gravité élevée ───► DENY ou DEFER (selon contexte)
    │
    ├── Gravité moyenne ───► DEFER à StrongFather
    │
    └── Gravité faible ───► DEGRADE ou ALERT
```

---

## 8. Règles par type de frontière

### 8.1 Règles pour frontières externes

**Objectif :** Protection maximale contre les entrées non autorisées.

| Règle | Priorité | Condition | Action échec |
|-------|----------|-----------|--------------|
| `EXT-001` | 1 | `source.trust_level ≠ HOSTILE` | DENY |
| `EXT-002` | 5 | `rate.requests_per_minute ≤ LIMIT` | DENY + ALERT |
| `EXT-003` | 15 | `source.trust_level ≥ UNKNOWN` | DENY |
| `EXT-004` | 35 | `source.authenticated = true` (si requis) | DEFER |
| `EXT-005` | 55 | `data.type IN ALLOWED_TYPES` | DENY |

### 8.2 Règles pour frontières internes

**Objectif :** Défense en profondeur, cloisonnement des zones.

| Règle | Priorité | Condition | Action échec |
|-------|----------|-----------|--------------|
| `INT-001` | 1 | `source.trust_level ≠ HOSTILE` | DENY |
| `INT-002` | 20 | `source.trust_level ≥ zone.required_level` | DEFER |
| `INT-003` | 40 | `source.session_valid = true` | DENY |
| `INT-004` | 60 | `action.authorized_in_zone = true` | DEFER |

### 8.3 Règles pour frontières d'intégration

**Objectif :** Contrôle des échanges avec les systèmes intégrés.

| Règle | Priorité | Condition | Action échec |
|-------|----------|-----------|--------------|
| `INTEG-001` | 1 | `integration.status = ACTIVE` | DENY |
| `INTEG-002` | 10 | `source.trust_level ≥ VERIFIED` | DEFER |
| `INTEG-003` | 30 | `integration.credentials_valid = true` | DENY |
| `INTEG-004` | 50 | `data.classification ≤ integration.max_classification` | DENY |
| `INTEG-005` | 70 | `action.type IN integration.allowed_actions` | DEFER |

---

## 9. Adaptation selon les niveaux de sécurité

Les règles de franchissement s'adaptent selon le niveau de sécurité déclaré.

**Référence :** [Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)

### 9.1 Adaptation des seuils

| Niveau de sécurité | Niveau confiance min. (externe) | Rate limit | Tolérance erreur |
|--------------------|--------------------------------|------------|------------------|
| **0 - PUBLIC** | UNKNOWN | Haut | Haute |
| **1 - STANDARD** | UNKNOWN | Standard | Standard |
| **2 - SENSITIVE** | VERIFIED | Réduit | Faible |
| **3 - CRITICAL** | VERIFIED+ | Strict | Minimale |
| **4 - HARDENED** | TRUSTED | Ultra-strict | Zéro |

### 9.2 Règles spécifiques par niveau

#### Niveau 0 - PUBLIC

```
# Règles assouplies
EXT-003: source.trust_level ≥ UNKNOWN (même non authentifié autorisé)
INT-002: source.trust_level ≥ UNKNOWN (zones ouvertes)
```

#### Niveau 3 - CRITICAL

```
# Règles strictes
EXT-003: source.trust_level ≥ VERIFIED (auth obligatoire)
EXT-ADD: source.auth_method IN [MFA, SSO] (auth renforcée)
INT-002: source.trust_level ≥ VERIFIED (cloisonnement strict)
```

#### Niveau 4 - HARDENED

```
# Règles ultra-strictes
EXT-003: source.trust_level = TRUSTED (seuls trusted autorisés)
INT-002: source.trust_level = TRUSTED (isolement)
ALL: rate.limit = MINIMAL (quasi aucun trafic)
```

---

## 10. Intégration avec les protocoles de sécurité

**Référence :** [Miyukini Conceptual References - Security Protocols](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md)

### 10.1 Protocoles temps réel (RT-SEC)

| Protocole | Règles concernées |
|-----------|-------------------|
| **RT-SEC-1** (Session éphémère) | Règles de session (`session.age`, `session.valid`) |
| **RT-SEC-2** (Auth en couches) | Règles d'authentification (`auth_method`, `context`) |
| **RT-SEC-3** (Validation systématique) | Toutes les règles (aucun bypass) |
| **RT-SEC-4** (Détection anomalie) | Règles de rate limiting, détection patterns |

### 10.2 Protocoles asynchrones (AS-SEC)

| Protocole | Règles concernées |
|-----------|-------------------|
| **AS-SEC-1** (Actions non engagées) | Règles d'action (`action.status = PENDING`) |
| **AS-SEC-2** (Signature locale faible) | Règles de signature (`signature.valid`) |
| **AS-SEC-3** (Revalidation) | Toutes les règles (réévaluation complète) |
| **AS-SEC-4** (Anti-replay) | Règles de séquence (`request.id`, `request.timestamp`) |

### 10.3 Flux avec BondingBrother

```
Border Guard                          BondingBrother
     │                                      │
     │ règles de franchissement             │
     │ (déclaratives)                       │
     │ ─────────────────────────────────►   │
     │                                      │
     │                                      │ implémentation
     │                                      │ des vérifications
     │                                      │
     │ résultat application                 │
     │ (pour traçabilité)                   │
     │ ◄─────────────────────────────────   │
```

---

## 11. Exceptions et cas particuliers

### 11.1 Définition d'une exception

Une **exception** est un cas où une règle ne s'applique pas, défini de manière explicite et traçable.

| Propriété | Description | Obligatoire |
|-----------|-------------|-------------|
| **Identifiant** | Identifiant de l'exception | ✅ Oui |
| **Règle concernée** | Quelle règle est exceptée | ✅ Oui |
| **Condition d'exception** | Quand l'exception s'applique | ✅ Oui |
| **Justification** | Pourquoi cette exception existe | ✅ Oui |
| **Durée** | Temporaire ou permanente | ✅ Oui |

### 11.2 Exceptions autorisées

| Type d'exception | Conditions | Autorité |
|------------------|------------|----------|
| **Urgence sécurité** | Faille critique, besoin immédiat | TAMR + StrongFather |
| **Migration** | Période de transition, compatibilité | EverBuddy |
| **Maintenance** | Opérations planifiées | Admin + CaringNanny |
| **Test** | Environnement de test uniquement | Environnement non-production |

### 11.3 Exceptions interdites

| Exception interdite | Raison |
|--------------------|--------|
| Exception permanente sans justification | Viole INV-BG-8 (traçabilité) |
| Exception contournant HOSTILE | Viole sécurité fondamentale |
| Exception définie par BondingBrother | Viole INV-BG-7 (séparation) |
| Exception non traçable | Viole INV-BG-8 |

---

## 12. Traçabilité des règles

### 12.1 Éléments à tracer

| Élément | Obligatoire | Description |
|---------|-------------|-------------|
| Règle évaluée | ✅ Oui | Identifiant de la règle |
| Condition | ✅ Oui | Condition évaluée |
| Résultat | ✅ Oui | Satisfaite / Non satisfaite |
| Action | ✅ Oui | Action exécutée si échec |
| Contexte | ✅ Oui | Contexte de l'évaluation |
| Timestamp | ✅ Oui | Horodatage |

### 12.2 Format de trace

```
Crossing Rule Evaluation:
- rule_id: <identifiant>
- frontier_id: <frontière traversée>
- condition: <condition évaluée>
- result: <PASS|FAIL>
- action_taken: <si FAIL>
- context: {source, destination, data, action}
- timestamp: <ISO 8601>
```

**Invariant associé :** INV-BG-8 — Toute évaluation de règle est **traçable**.

---

## 13. Références croisées

### Invariants associés (Documentation Fondatrice - Section 7)

| Invariant | Énoncé | Relation |
|-----------|--------|----------|
| INV-BG-6 | Règles déclaratives | Fondement de ce contrat |
| INV-BG-7 | Séparation définition/application | Border Guard définit, BondingBrother applique |
| INV-BG-8 | Traçabilité complète | Toute règle et évaluation est traçable |
| INV-BG-9 | Cohérence globale | Pas de contradiction entre règles |

### Documents associés

| Document | Relation |
|----------|----------|
| [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) | Document source |
| [Border Guard - Boundary Definition Contract](./Border%20Guard%20-%20Boundary%20Definition%20Contract.md) | Frontières auxquelles les règles s'appliquent |
| [Border Guard - Trust Level Classification Contract](./Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) | Niveaux utilisés dans les règles |
| [Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) | Adaptation selon niveau sécurité |
| [Miyukini Conceptual References - Security Protocols](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md) | Protocoles utilisant les règles |

### Références glossaire

| Terme | Définition |
|-------|------------|
| **Règle de franchissement** | Condition déclarative pour autoriser un franchissement |
| **Franchissement** | Acte de traverser une frontière |
| **Condition déclarative** | Expression de ce qui est requis, pas comment le vérifier |
| **Priorité** | Ordre d'évaluation des règles |
| **Exception** | Cas où une règle ne s'applique pas |

**Source :** [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 14. Synthèse contractuelle

### Garanties de ce contrat

Ce contrat garantit que :

1. **Les règles sont déclaratives** — Expriment ce qui est requis, pas comment le vérifier (INV-BG-6)
2. **La séparation est stricte** — Border Guard définit, BondingBrother applique (INV-BG-7)
3. **Cinq types de règles** — Confiance, authentification, données, action, temporel
4. **L'évaluation est déterministe** — Priorités claires, algorithme défini
5. **L'adaptation est automatique** — Les règles s'adaptent au niveau de sécurité
6. **La traçabilité est complète** — Toute règle et évaluation est traçable

### Phrase de synthèse

> **Une règle de franchissement est une condition déclarative, définie exclusivement par Border Guard et appliquée par BondingBrother, qui spécifie ce qui est requis pour traverser une frontière, selon une priorité et avec une action définie en cas d'échec.**

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Contrat — Normatif  
**Référence :** Border Guard v1.5, Documentation Fondatrice Section 4 et 5  
**Type :** Contrat de règles de franchissement
