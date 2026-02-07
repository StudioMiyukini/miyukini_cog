# Master Butler — Access Control Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **Master Butler Access Control Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit les règles et mécanismes de contrôle d'accès aux APIs et fonctions de Master Butler dans le système Miyukini Core System v2.4.

Ce contrat précise qui peut accéder à Master Butler, sous quelles conditions, avec quels niveaux d'autorité, et selon quelles règles. Il définit les contrôles d'accès aux opérations de définition, d'interrogation, et de gestion des capacités et permissions.

**Important :** Ce contrat définit le contrôle d'accès à Master Butler lui-même, pas le contrôle d'accès aux ressources métier du système. Master Butler définit les permissions, mais ne vérifie jamais si elles sont accordées — cette responsabilité appartient à StrongFather.

### Portée

Ce contrat s'applique à **tous les composants** interagissant avec Master Butler et définit de manière absolue :

- Les catégories d'appelants autorisés à accéder à Master Butler
- Les niveaux d'autorité et leurs privilèges associés
- Les règles d'accès aux différentes APIs (Capability, Permission, Discovery)
- Les contrôles d'accès aux opérations sensibles
- L'adaptation du contrôle d'accès selon les niveaux de sécurité (0-4)
- Les invariants de contrôle d'accès non négociables
- Les comportements en cas de violation d'accès

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :

- **Master Butler — Documentation Fondatrice** : Définit la nature et les responsabilités de Master Butler
- **Master Butler — Authority Limits Contract** : Définit les limites d'autorité de Master Butler
- **Master Butler — Capability API Contract** : Définit les opérations sur les capacités
- **Master Butler — Permission API Contract** : Définit les opérations sur les permissions
- **Master Butler — Threat Model Contract** : Définit les menaces de sécurité (complémentaire)
- **[Miyukini Conceptual References — Security Protocols](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md)** : Protocoles de sécurité (RT-SEC-2, RT-SEC-3, AS-SEC-3)
- **[Miyukini Conceptual References — Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)** : Niveaux de sécurité (0-4)
- **[Miyukini Conceptual References — Lois Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Ce contrat respecte **LOI-1** (aucune dépendance externe critique)

---

## 2. Principes fondamentaux du contrôle d'accès

### Principe de séparation définition/décision

Master Butler contrôle l'accès à ses propres APIs, mais ne décide jamais de l'autorisation finale d'une action métier.

| Responsabilité | Propriétaire | Ce que fait Master Butler |
|----------------|--------------|---------------------------|
| **Contrôle d'accès aux APIs Master Butler** | Master Butler | ✅ Contrôle qui peut utiliser ses APIs |
| **Vérification des permissions métier** | StrongFather | ❌ Ne vérifie jamais |
| **Décision d'autorisation finale** | StrongFather | ❌ Ne décide jamais |

**Règle absolue :**

> **Master Butler contrôle l'accès à son registre, jamais l'accès aux ressources métier du système.**

### Principe de moindre privilège

Chaque appelant reçoit uniquement le niveau d'accès nécessaire à sa fonction dans le système. Aucun accès excessif n'est accordé.

### Principe de défense en profondeur

Le contrôle d'accès à Master Butler s'inscrit dans une chaîne de contrôles :

1. **Border Guard** : Classification de la source (avant Master Butler)
2. **Master Butler** : Contrôle d'accès à ses APIs (ce contrat)
3. **StrongFather** : Décision d'autorisation finale (après Master Butler)

### Principe de traçabilité

Tout accès à Master Butler est tracé. Les tentatives d'accès non autorisées sont enregistrées et peuvent déclencher des alertes.

---

## 3. Catégories d'appelants

### 3.1. Définition formelle

Un **appelant** est tout composant, système, ou processus qui soumet une requête à Master Butler via ses APIs. Chaque appelant est identifié et classifié selon sa catégorie.

### 3.2. Catégories reconnues

| Catégorie | Code | Description | Niveau de confiance |
|-----------|------|-------------|---------------------|
| **Core Système** | `CORE` | Cores de Strate 4 (StrongFather, KindMother, etc.) | Maximal |
| **Service Système** | `SERVICE` | Services de Strate 5 (Border Guard, TAMR, etc.) | Élevé |
| **Opérateur** | `OPERATOR` | Opérateurs et produits enregistrés | Standard |
| **Administration** | `ADMIN` | MiyukiniAdmin et outils d'administration | Critique |
| **Bootstrap** | `BOOTSTRAP` | Processus de démarrage du système | Temporaire |

### 3.3. Caractéristiques par catégorie

#### CORE — Cores Système

**Identité :** StrongFather, KindMother, BondingBrother, Ever Buddy, Caring Nanny, WorrySentinel

**Niveau de confiance :** Maximal

**Accès accordé :**
- ✅ Interrogation complète (toutes les APIs)
- ✅ Accès aux permissions de tous niveaux (STANDARD à SYSTEM)
- ⚠️ Définition limitée (selon le Core)
- ❌ Modification des permissions SYSTEM (sauf StrongFather)

**Justification :** Les Cores sont des composants fondamentaux du système avec des responsabilités critiques. Ils nécessitent un accès complet aux informations pour remplir leurs fonctions.

#### SERVICE — Services Système

**Identité :** Border Guard, Kernel Logger, etc.

**Niveau de confiance :** Élevé

**Accès accordé :**
- ✅ Interrogation des capacités et permissions
- ✅ Découverte des Outils et Kits d'Outils
- ❌ Définition de permissions
- ❌ Accès aux permissions SYSTEM

**Justification :** Les Services ont besoin d'interroger Master Butler pour leurs fonctions, mais n'ont pas besoin de définir des permissions.

#### OPERATOR — Opérateurs

**Identité :** Produits et Opérateurs enregistrés dans le système

**Niveau de confiance :** Standard

**Accès accordé :**
- ✅ Déclaration de capacités (propres au module)
- ✅ Définition de permissions STANDARD et ELEVATED (propres au module)
- ✅ Interrogation des capacités et permissions (selon scope)
- ✅ Découverte des capacités disponibles
- ❌ Définition de permissions CRITICAL ou SYSTEM
- ❌ Accès aux permissions d'autres Opérateurs (sauf publiques)

**Justification :** Les Opérateurs définissent leurs propres capacités et permissions, mais ne peuvent pas définir de permissions critiques ou accéder aux données d'autres Opérateurs.

#### ADMIN — Administration

**Identité :** MiyukiniAdmin, outils d'administration autorisés

**Niveau de confiance :** Critique

**Accès accordé :**
- ✅ Accès complet à toutes les APIs
- ✅ Définition de permissions de tous niveaux (incluant SYSTEM)
- ✅ Modification des permissions SYSTEM
- ✅ Audit complet du registre
- ✅ Opérations de maintenance

**Justification :** L'administration système nécessite un accès complet pour la gestion, l'audit, et la maintenance.

#### BOOTSTRAP — Processus de démarrage

**Identité :** Processus de démarrage du système

**Niveau de confiance :** Temporaire (limité à la phase de bootstrap)

**Accès accordé :**
- ✅ Enregistrement initial des capacités fondamentales
- ✅ Définition des permissions de base
- ❌ Accès après la phase de bootstrap

**Justification :** Le bootstrap nécessite un accès temporaire pour initialiser le système, mais cet accès est révoqué après initialisation.

---

## 4. Niveaux d'autorité

### 4.1. Définition formelle

Un **niveau d'autorité** est un ensemble de privilèges attribué à un appelant, déterminant les opérations qu'il peut effectuer sur Master Butler.

### 4.2. Hiérarchie des niveaux

| Niveau | Nom | Privilèges | Catégories autorisées |
|--------|-----|------------|----------------------|
| **0** | Lecture publique | Interrogation des capacités publiques | Tous |
| **1** | Lecture standard | Interrogation complète | OPERATOR, SERVICE, CORE, ADMIN |
| **2** | Définition standard | Définition de permissions STANDARD/ELEVATED | OPERATOR, CORE, ADMIN |
| **3** | Définition critique | Définition de permissions CRITICAL | CORE (StrongFather), ADMIN |
| **4** | Définition système | Définition de permissions SYSTEM | ADMIN uniquement |
| **5** | Administration | Toutes opérations, audit, maintenance | ADMIN uniquement |

### 4.3. Matrice d'accès par niveau d'autorité

| Opération | Niveau 0 | Niveau 1 | Niveau 2 | Niveau 3 | Niveau 4 | Niveau 5 |
|-----------|----------|----------|----------|----------|----------|----------|
| Lire capacités publiques | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Lire toutes capacités | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Lire permissions STANDARD | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Lire permissions ELEVATED | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Lire permissions CRITICAL | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Lire permissions SYSTEM | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ |
| Déclarer capacités | ❌ | ❌ | ✅ | ✅ | ✅ | ✅ |
| Définir permissions STANDARD | ❌ | ❌ | ✅ | ✅ | ✅ | ✅ |
| Définir permissions ELEVATED | ❌ | ❌ | ✅ | ✅ | ✅ | ✅ |
| Définir permissions CRITICAL | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ |
| Définir permissions SYSTEM | ❌ | ❌ | ❌ | ❌ | ✅ | ✅ |
| Modifier permissions SYSTEM | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Audit complet | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Maintenance | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |

### 4.4. Attribution des niveaux par catégorie

| Catégorie | Niveau d'autorité par défaut | Niveau maximal atteignable |
|-----------|------------------------------|----------------------------|
| CORE (StrongFather) | 3 | 3 |
| CORE (autres) | 1 | 2 |
| SERVICE | 1 | 1 |
| OPERATOR | 1 | 2 |
| ADMIN | 5 | 5 |
| BOOTSTRAP | 2 | 2 |

---

## 5. Règles d'accès aux APIs

### 5.1. Capability API

#### Opérations de lecture

| Opération | Niveau requis | Scope autorisé |
|-----------|--------------|----------------|
| `getCapability` | 0 (public), 1 (complet) | Selon visibilité |
| `listCapabilities` | 0 (public), 1 (complet) | Selon visibilité |
| `searchCapabilities` | 1 | Selon visibilité |
| `getCapabilityHierarchy` | 1 | Selon visibilité |

#### Opérations de définition

| Opération | Niveau requis | Conditions supplémentaires |
|-----------|--------------|---------------------------|
| `declareCapability` | 2 | Module propre uniquement |
| `activateCapability` | 2 | Capacité propre uniquement |
| `updateCapability` | 2 | Capacité propre uniquement |
| `deprecateCapability` | 2 | Capacité propre uniquement |
| `retireCapability` | 5 | Administration uniquement |

**Règle de scope :** Un Opérateur ne peut déclarer, modifier, ou déprécier que les capacités de son propre module. Cette restriction est absolue et non contournable.

### 5.2. Permission API

#### Opérations de lecture

| Opération | Niveau requis | Restrictions |
|-----------|--------------|--------------|
| `getPermission` | 1 | Selon niveau de permission |
| `listPermissions` | 1 | Selon niveau de permission |
| `searchPermissions` | 1 | Selon niveau de permission |
| `getPermissionCapabilities` | 1 | Selon niveau de permission |
| `getPermissionHierarchy` | 1 | Selon niveau de permission |

**Restriction par niveau de permission :**

- Permissions STANDARD/ELEVATED : Niveau d'autorité 1+
- Permissions CRITICAL : Niveau d'autorité 3+
- Permissions SYSTEM : Niveau d'autorité 3+ (lecture), 5 (modification)

#### Opérations de définition

| Opération | Niveau de permission cible | Niveau d'autorité requis |
|-----------|---------------------------|-------------------------|
| `definePermission` | STANDARD | 2 |
| `definePermission` | ELEVATED | 2 |
| `definePermission` | CRITICAL | 3 |
| `definePermission` | SYSTEM | 4 |
| `activatePermission` | Selon niveau | Selon niveau de la permission |
| `updatePermission` | Selon niveau | Selon niveau de la permission |
| `deprecatePermission` | Selon niveau | Selon niveau de la permission |
| `retirePermission` | Tous | 5 |

**Règle de scope :** Un Opérateur ne peut définir des permissions que pour les capacités qu'il possède, sauf pour les Cores et l'administration.

### 5.3. Discovery API

| Opération | Niveau requis | Résultat |
|-----------|--------------|----------|
| `discoverCapabilities` | 0 (public), 1 (complet) | Capacités filtrées selon visibilité |
| `discoverPermissions` | 1 | Permissions filtrées selon niveau |
| `discoverTools` | 0 (public), 1 (complet) | Outils filtrés selon visibilité |
| `discoverToolkits` | 0 (public), 1 (complet) | Kits d'Outils filtrés selon visibilité |
| `getCapabilityContext` | 1 | Contexte de capacité complet |

---

## 6. Contrôle d'accès adaptatif selon les niveaux de sécurité

### 6.1. Principe d'adaptation

Master Butler adapte son contrôle d'accès selon le niveau de sécurité déclaré par l'Opérateur (0-4), conformément au document [Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md).

### 6.2. Adaptation par niveau de sécurité

#### Niveau 0 — PUBLIC / DISPLAY

| Aspect | Comportement |
|--------|--------------|
| **Authentification** | Non obligatoire pour la lecture publique |
| **Vérification d'autorité** | Simplifiée |
| **Traçabilité** | Minimale |
| **Contrôles** | Validation structurelle uniquement |

#### Niveau 1 — STANDARD / CMS

| Aspect | Comportement |
|--------|--------------|
| **Authentification** | Obligatoire pour les opérations de définition |
| **Vérification d'autorité** | Standard |
| **Traçabilité** | Normale |
| **Contrôles** | Validation complète |

#### Niveau 2 — SENSITIVE DATA

| Aspect | Comportement |
|--------|--------------|
| **Authentification** | Obligatoire pour toutes les opérations |
| **Vérification d'autorité** | Renforcée |
| **Traçabilité** | Complète |
| **Contrôles** | Validation stricte + vérification de cohérence |

#### Niveau 3 — CRITICAL SYSTEM

| Aspect | Comportement |
|--------|--------------|
| **Authentification** | Obligatoire avec vérification croisée |
| **Vérification d'autorité** | Stricte |
| **Traçabilité** | Absolue |
| **Contrôles** | Validation stricte + signatures |

#### Niveau 4 — HARDENED / ISOLATED

| Aspect | Comportement |
|--------|--------------|
| **Authentification** | Obligatoire avec attestations |
| **Vérification d'autorité** | Maximale |
| **Traçabilité** | Absolue + audit temps réel |
| **Contrôles** | Validation continue + restrictions maximales |

### 6.3. Restrictions supplémentaires par niveau

| Niveau de sécurité | Restrictions supplémentaires |
|--------------------|------------------------------|
| 0-1 | Aucune restriction supplémentaire |
| 2 | Définition de permissions ELEVATED requiert confirmation |
| 3 | Définition de permissions nécessite validation StrongFather |
| 4 | Seules les opérations de lecture sont autorisées sans validation explicite |

---

## 7. Contrôle d'accès au scope

### 7.1. Définition du scope

Le **scope** définit le périmètre d'accès d'un appelant aux capacités et permissions du registre.

### 7.2. Types de scope

| Type | Description | Applicable à |
|------|-------------|--------------|
| **GLOBAL** | Accès à tout le registre | ADMIN, CORE (StrongFather) |
| **MODULE** | Accès aux capacités/permissions du module | OPERATOR |
| **PUBLIC** | Accès aux capacités/permissions publiques | Tous |

### 7.3. Règles de scope

**SCOPE-1 : Isolation des Opérateurs**

Un Opérateur n'accède qu'aux capacités et permissions de son propre module, plus les capacités/permissions publiques.

**SCOPE-2 : Accès global des Cores**

StrongFather et l'administration ont un accès global au registre pour leurs fonctions respectives.

**SCOPE-3 : Visibilité des permissions**

La visibilité des permissions suit la hiérarchie des niveaux de criticité :
- STANDARD : Visibles par tous les appelants autorisés
- ELEVATED : Visibles par les appelants de niveau 1+
- CRITICAL : Visibles par les appelants de niveau 3+
- SYSTEM : Visibles par les appelants de niveau 3+ (lecture), 5 (modification)

### 7.4. Détermination du scope

Le scope d'un appelant est déterminé par :

1. **Catégorie** : CORE, SERVICE, OPERATOR, ADMIN
2. **Identité** : Identifiant de l'appelant
3. **Module** : Module d'appartenance (pour les Opérateurs)
4. **Contexte** : Contexte de l'appel (niveau de sécurité, etc.)

---

## 8. Validation du contexte d'appel

### 8.1. Structure du contexte

Chaque appel à Master Butler inclut un contexte validé :

```
Access Context
├── Caller Identity
│   ├── caller_id: <identifiant unique>
│   ├── category: <CORE | SERVICE | OPERATOR | ADMIN | BOOTSTRAP>
│   └── authority_level: <0-5>
├── Scope
│   ├── scope_type: <GLOBAL | MODULE | PUBLIC>
│   └── module_id: <identifiant du module si applicable>
├── Security
│   ├── security_level: <0-4>
│   ├── authenticated: <boolean>
│   └── authentication_method: <méthode si authentifié>
└── Trace
    ├── request_id: <identifiant unique de requête>
    ├── correlation_id: <identifiant de corrélation>
    └── timestamp: <horodatage>
```

### 8.2. Validations obligatoires

| Validation | Description | Erreur si échoue |
|------------|-------------|------------------|
| **V-CTX-1** | Identité de l'appelant présente | `MISSING_CALLER_IDENTITY` |
| **V-CTX-2** | Catégorie reconnue | `UNKNOWN_CALLER_CATEGORY` |
| **V-CTX-3** | Niveau d'autorité cohérent avec la catégorie | `INVALID_AUTHORITY_LEVEL` |
| **V-CTX-4** | Scope valide pour la catégorie | `INVALID_SCOPE` |
| **V-CTX-5** | Niveau de sécurité déclaré | `MISSING_SECURITY_LEVEL` |
| **V-CTX-6** | Authentification si requise | `AUTHENTICATION_REQUIRED` |

### 8.3. Ordre de validation

1. Présence et structure du contexte
2. Identité de l'appelant
3. Catégorie et niveau d'autorité
4. Authentification (si requise par le niveau de sécurité)
5. Scope et visibilité
6. Autorisation pour l'opération demandée

---

## 9. Comportements en cas de violation d'accès

### 9.1. Types de violations

| Code | Type | Description | Gravité |
|------|------|-------------|---------|
| `V-ACC-1` | Identité invalide | Appelant non identifié ou identité invalide | Critique |
| `V-ACC-2` | Autorité insuffisante | Niveau d'autorité insuffisant pour l'opération | Haute |
| `V-ACC-3` | Scope dépassé | Tentative d'accès hors du scope autorisé | Haute |
| `V-ACC-4` | Opération interdite | Opération non autorisée pour la catégorie | Haute |
| `V-ACC-5` | Niveau de permission | Accès à un niveau de permission non autorisé | Moyenne |
| `V-ACC-6` | Authentification | Authentification requise mais non fournie | Moyenne |

### 9.2. Comportement par type de violation

#### V-ACC-1 : Identité invalide

**Comportement :**
- Rejet immédiat de l'appel
- Erreur : `ACCESS_DENIED_UNKNOWN_CALLER`
- Traçabilité : Enregistrement de la tentative avec informations disponibles
- Alerte : Possible selon la politique de sécurité

#### V-ACC-2 : Autorité insuffisante

**Comportement :**
- Rejet de l'appel
- Erreur : `ACCESS_DENIED_INSUFFICIENT_AUTHORITY`
- Traçabilité : Enregistrement de la tentative avec identité
- Information : Niveau requis vs niveau fourni

#### V-ACC-3 : Scope dépassé

**Comportement :**
- Rejet de l'appel
- Erreur : `ACCESS_DENIED_SCOPE_VIOLATION`
- Traçabilité : Enregistrement de la tentative
- Information : Scope autorisé vs scope demandé

#### V-ACC-4 : Opération interdite

**Comportement :**
- Rejet de l'appel
- Erreur : `ACCESS_DENIED_OPERATION_FORBIDDEN`
- Traçabilité : Enregistrement de la tentative
- Information : Opération demandée et raison du refus

#### V-ACC-5 : Niveau de permission

**Comportement :**
- Rejet de l'appel ou filtrage des résultats
- Erreur : `ACCESS_DENIED_PERMISSION_LEVEL` (si rejet)
- Comportement alternatif : Retour d'un sous-ensemble autorisé (si filtrage)

#### V-ACC-6 : Authentification

**Comportement :**
- Rejet de l'appel
- Erreur : `AUTHENTICATION_REQUIRED`
- Information : Type d'authentification requis

### 9.3. Garanties après violation

- L'état du registre reste inchangé
- Aucune modification partielle n'est appliquée
- L'erreur est explicite et actionnable
- La tentative est tracée pour audit
- Aucun effet de bord n'est créé

---

## 10. Intégration avec les protocoles de sécurité

### 10.1. Protocole RT-SEC-2 — Authentification en couches

Master Butler participe au protocole RT-SEC-2 (Authentification en couches) :

```
Requête
    ↓
Border Guard (classification source)
    ↓
Master Butler (capacités disponibles ?)  ← Ce contrat s'applique ici
    ↓
Caring Nanny (état système ?)
    ↓
StrongFather (décision finale)
```

**Rôle de Master Butler :** Fournir les capacités et permissions disponibles pour le contexte, en appliquant les contrôles d'accès définis dans ce contrat.

### 10.2. Protocole RT-SEC-3 — Validation systématique

Master Butler applique la validation systématique :

- ✅ Validation du contexte d'appel
- ✅ Validation de l'autorité de l'appelant
- ✅ Validation du scope demandé
- ✅ Validation de l'opération demandée

**Règle absolue :** Aucune optimisation ne peut court-circuiter ces validations, même en temps réel.

### 10.3. Protocole AS-SEC-3 — Revalidation à la reprise

Lors du retour en ligne (protocole AS-SEC-3), Master Butler revalide :

- L'autorité de l'appelant
- Le scope autorisé
- Les permissions accessibles selon le niveau de sécurité

---

## 11. Invariants de contrôle d'accès

### INV-ACC-1 : Identification obligatoire

Tout appel à Master Butler DOIT être accompagné d'une identification de l'appelant. Aucun appel anonyme n'est traité, sauf pour les opérations de lecture publique au niveau de sécurité 0.

### INV-ACC-2 : Validation avant exécution

Toute opération est validée AVANT exécution. Aucune opération n'est exécutée partiellement avant validation complète.

### INV-ACC-3 : Moindre privilège

Un appelant reçoit uniquement le niveau d'accès nécessaire à sa fonction. Aucun privilège excessif n'est accordé.

### INV-ACC-4 : Isolation des Opérateurs

Un Opérateur n'accède jamais aux capacités ou permissions d'un autre Opérateur, sauf si elles sont publiques.

### INV-ACC-5 : Traçabilité complète

Toute tentative d'accès (réussie ou refusée) est tracée avec contexte complet.

### INV-ACC-6 : Non-contournabilité

Le contrôle d'accès ne peut pas être contourné. Aucun chemin alternatif n'existe pour accéder au registre sans passer par les validations.

### INV-ACC-7 : Adaptation au niveau de sécurité

Le contrôle d'accès s'adapte au niveau de sécurité déclaré, sans exception.

### INV-ACC-8 : Séparation définition/décision

Master Butler contrôle l'accès à son registre mais ne prend jamais de décision d'autorisation métier. Cette responsabilité appartient à StrongFather.

---

## 12. Schémas ASCII

### 12.1. Flux de contrôle d'accès

```
┌─────────────────────────────────────────────────────────────────────────┐
│                  FLUX DE CONTRÔLE D'ACCÈS MASTER BUTLER                  │
│                                                                          │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │                      APPELANT                                      │  │
│  │  (CORE, SERVICE, OPERATOR, ADMIN, BOOTSTRAP)                       │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                   │                                      │
│                                   │ Appel avec contexte                  │
│                                   ▼                                      │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │              VALIDATION DU CONTEXTE (V-CTX)                        │  │
│  │                                                                     │  │
│  │  ├── V-CTX-1 : Identité présente ?         ──→ REJET si non       │  │
│  │  ├── V-CTX-2 : Catégorie reconnue ?        ──→ REJET si non       │  │
│  │  ├── V-CTX-3 : Niveau d'autorité valide ?  ──→ REJET si non       │  │
│  │  ├── V-CTX-4 : Scope valide ?              ──→ REJET si non       │  │
│  │  ├── V-CTX-5 : Niveau sécurité déclaré ?   ──→ REJET si non       │  │
│  │  └── V-CTX-6 : Authentification si requise ──→ REJET si non       │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                   │                                      │
│                                   │ Contexte validé                      │
│                                   ▼                                      │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │              VÉRIFICATION DE L'AUTORITÉ                            │  │
│  │                                                                     │  │
│  │  ├── Niveau d'autorité suffisant pour l'opération ?               │  │
│  │  │       ──→ REJET si non (V-ACC-2)                               │  │
│  │  │                                                                 │  │
│  │  ├── Scope couvre la ressource demandée ?                         │  │
│  │  │       ──→ REJET si non (V-ACC-3)                               │  │
│  │  │                                                                 │  │
│  │  └── Opération autorisée pour la catégorie ?                      │  │
│  │          ──→ REJET si non (V-ACC-4)                               │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                   │                                      │
│                                   │ Autorité vérifiée                    │
│                                   ▼                                      │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │              EXÉCUTION DE L'OPÉRATION                              │  │
│  │                                                                     │  │
│  │  • Opération exécutée selon les règles des APIs                   │  │
│  │  • Résultats filtrés selon le scope autorisé                      │  │
│  │  • Traçabilité enregistrée                                        │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                   │                                      │
│                                   │ Résultat                             │
│                                   ▼                                      │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │                      APPELANT                                      │  │
│  │  (Reçoit le résultat ou l'erreur explicite)                       │  │
│  └───────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────┘
```

### 12.2. Matrice d'accès par catégorie

```
┌─────────────────────────────────────────────────────────────────────────┐
│           MATRICE D'ACCÈS PAR CATÉGORIE D'APPELANT                       │
│                                                                          │
│  ┌────────────────────┬──────┬─────────┬──────────┬───────┬───────────┐ │
│  │ OPÉRATION          │ CORE │ SERVICE │ OPERATOR │ ADMIN │ BOOTSTRAP │ │
│  ├────────────────────┼──────┼─────────┼──────────┼───────┼───────────┤ │
│  │ LECTURE            │      │         │          │       │           │ │
│  │ ──────────         │      │         │          │       │           │ │
│  │ Capacités publiques│  ✅  │   ✅    │    ✅    │  ✅   │    ✅     │ │
│  │ Capacités complètes│  ✅  │   ✅    │    ⚠️    │  ✅   │    ❌     │ │
│  │ Perms STANDARD     │  ✅  │   ✅    │    ✅    │  ✅   │    ❌     │ │
│  │ Perms ELEVATED     │  ✅  │   ✅    │    ✅    │  ✅   │    ❌     │ │
│  │ Perms CRITICAL     │  ✅  │   ❌    │    ❌    │  ✅   │    ❌     │ │
│  │ Perms SYSTEM       │  ⚠️  │   ❌    │    ❌    │  ✅   │    ❌     │ │
│  ├────────────────────┼──────┼─────────┼──────────┼───────┼───────────┤ │
│  │ DÉFINITION         │      │         │          │       │           │ │
│  │ ──────────         │      │         │          │       │           │ │
│  │ Déclarer capacités │  ⚠️  │   ❌    │    ✅    │  ✅   │    ✅     │ │
│  │ Définir STANDARD   │  ⚠️  │   ❌    │    ✅    │  ✅   │    ✅     │ │
│  │ Définir ELEVATED   │  ⚠️  │   ❌    │    ✅    │  ✅   │    ✅     │ │
│  │ Définir CRITICAL   │  ⚠️  │   ❌    │    ❌    │  ✅   │    ❌     │ │
│  │ Définir SYSTEM     │  ❌  │   ❌    │    ❌    │  ✅   │    ❌     │ │
│  ├────────────────────┼──────┼─────────┼──────────┼───────┼───────────┤ │
│  │ ADMINISTRATION     │      │         │          │       │           │ │
│  │ ──────────────     │      │         │          │       │           │ │
│  │ Modifier SYSTEM    │  ❌  │   ❌    │    ❌    │  ✅   │    ❌     │ │
│  │ Audit complet      │  ❌  │   ❌    │    ❌    │  ✅   │    ❌     │ │
│  │ Maintenance        │  ❌  │   ❌    │    ❌    │  ✅   │    ❌     │ │
│  └────────────────────┴──────┴─────────┴──────────┴───────┴───────────┘ │
│                                                                          │
│  LÉGENDE :                                                               │
│  ✅ = Autorisé                                                           │
│  ⚠️ = Autorisé avec restrictions (scope, niveau, etc.)                  │
│  ❌ = Interdit                                                           │
└─────────────────────────────────────────────────────────────────────────┘
```

### 12.3. Adaptation au niveau de sécurité

```
┌─────────────────────────────────────────────────────────────────────────┐
│        ADAPTATION DU CONTRÔLE D'ACCÈS AU NIVEAU DE SÉCURITÉ             │
│                                                                          │
│  NIVEAU 0 — PUBLIC                                                       │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │  • Authentification : Non obligatoire (lecture publique)         │   │
│  │  • Vérification : Simplifiée                                     │   │
│  │  • Traçabilité : Minimale                                        │   │
│  │  • Impact : 🟢 Quasi nul                                         │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
│  NIVEAU 1 — STANDARD                                                     │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │  • Authentification : Obligatoire pour définition                │   │
│  │  • Vérification : Standard                                       │   │
│  │  • Traçabilité : Normale                                         │   │
│  │  • Impact : 🟢 Faible                                            │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
│  NIVEAU 2 — SENSITIVE                                                    │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │  • Authentification : Obligatoire pour toutes opérations         │   │
│  │  • Vérification : Renforcée                                      │   │
│  │  • Traçabilité : Complète                                        │   │
│  │  • Restriction : Définition ELEVATED requiert confirmation       │   │
│  │  • Impact : 🟡 Modéré                                            │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
│  NIVEAU 3 — CRITICAL                                                     │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │  • Authentification : Obligatoire avec vérification croisée      │   │
│  │  • Vérification : Stricte                                        │   │
│  │  • Traçabilité : Absolue                                         │   │
│  │  • Restriction : Définition nécessite validation StrongFather    │   │
│  │  • Impact : 🟠 Accepté                                           │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
│  NIVEAU 4 — HARDENED                                                     │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │  • Authentification : Obligatoire avec attestations              │   │
│  │  • Vérification : Maximale                                       │   │
│  │  • Traçabilité : Absolue + audit temps réel                      │   │
│  │  • Restriction : Lecture seule sans validation explicite         │   │
│  │  • Impact : 🔴 Secondaire                                        │   │
│  └─────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 13. Conformité aux Lois d'Autonomie Système

Ce contrat respecte les Lois d'Autonomie Système définies dans [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md).

### LOI-1 : Aucune dépendance externe critique à l'exécution

**Conformité :** Conforme

Le contrôle d'accès fonctionne entièrement en local :

- **Validation locale** : Toutes les validations sont effectuées localement
- **Pas de service d'authentification externe obligatoire** : L'identité de l'appelant est fournie dans le contexte
- **Aucune API externe** : Le contrôle d'accès ne dépend d'aucun service distant

**Vérification LOI-1** : *"Le contrôle d'accès fonctionne-t-il si le réseau est indisponible ?"* → **Oui.**

### LOI-5 : Le coût doit être proportionnel au hardware

**Conformité :** Conforme

Le contrôle d'accès a une empreinte minimale :

- **Validations légères** : Comparaisons simples d'identités et de niveaux
- **Pas de cryptographie lourde** : Validation basée sur le contexte fourni
- **Pas de workers** : Aucun processus en arrière-plan pour le contrôle d'accès

**Vérification LOI-5** : *"Le contrôle d'accès fonctionne-t-il sur un Raspberry Pi 4 ?"* → **Oui.**

### Synthèse de conformité

| Loi | Statut | Raison |
|-----|--------|--------|
| LOI-1 | ✅ Conforme | Contrôle d'accès local, aucune dépendance externe |
| LOI-5 | ✅ Conforme | Validations légères, empreinte minimale |

---

## 14. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable les règles de contrôle d'accès à Master Butler.

**Points clés :**

- **Catégories d'appelants** : CORE, SERVICE, OPERATOR, ADMIN, BOOTSTRAP
- **Niveaux d'autorité** : 0 (lecture publique) à 5 (administration)
- **Isolation** : Les Opérateurs n'accèdent qu'à leurs propres capacités/permissions
- **Adaptation** : Le contrôle s'adapte aux niveaux de sécurité (0-4)
- **Séparation** : Master Butler contrôle l'accès à son registre, pas aux ressources métier

**Phrase fondatrice :**

> **Master Butler contrôle l'accès à son registre de capacités et permissions, sans jamais décider de l'autorisation finale d'une action métier — cette responsabilité appartient à StrongFather.**

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

**Document créé le :** 2026-01-27  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, Master Butler Documentation, [Miyukini Conceptual References - Security Protocols](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md), [Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)  
**Type :** Contrat de contrôle d'accès non négociable

---

## 15. Mini log — erreurs / warnings / ambiguïtés rencontrées et corrigées

### Ambiguïté A1 : Distinction entre contrôle d'accès à Master Butler et vérification des permissions métier

**Ambiguïté rencontrée :** Risque de confusion entre le contrôle d'accès aux APIs de Master Butler (ce contrat) et la vérification des permissions pour les actions métier (StrongFather).

**Décision prise :** Clarification explicite dès l'introduction et rappels constants que Master Butler contrôle l'accès à son registre, pas aux ressources métier.

**Correction effectuée :** Section 2 rédigée avec principe de séparation définition/décision, et invariant INV-ACC-8 ajouté.

### Ambiguïté A2 : Accès des Cores aux permissions SYSTEM

**Ambiguïté rencontrée :** Les Cores (StrongFather notamment) doivent-ils avoir accès aux permissions SYSTEM ?

**Décision prise :** StrongFather peut LIRE les permissions SYSTEM (niveau 3) pour ses décisions, mais ne peut pas les MODIFIER (réservé à ADMIN niveau 5).

**Correction effectuée :** Matrice d'accès précisée dans la section 4.3 et schéma 12.2.

### Ambiguïté A3 : Scope des Opérateurs

**Ambiguïté rencontrée :** Comment définir précisément le scope d'un Opérateur ?

**Décision prise :** Un Opérateur n'accède qu'aux capacités/permissions de son propre module (identifié par module_id) plus les capacités/permissions publiques.

**Correction effectuée :** Section 7 rédigée avec règles de scope explicites.

### Vérification de compatibilité

**Vérification effectuée :**
- ✅ Cohérence avec Authority Limits Contract : Confirmée (Master Butler ne décide pas)
- ✅ Cohérence avec Permission API Contract : Confirmée (niveaux d'autorité cohérents)
- ✅ Cohérence avec Security Protocols : Confirmée (RT-SEC-2, RT-SEC-3, AS-SEC-3)
- ✅ Cohérence avec Security Levels : Confirmée (adaptation niveaux 0-4)

**Conclusion :** Aucune contradiction détectée avec les contrats existants.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
