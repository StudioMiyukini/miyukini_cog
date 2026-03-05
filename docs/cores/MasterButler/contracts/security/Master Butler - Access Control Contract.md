# Master Butler â€” Access Control Contract

## 1. Introduction

### Objet du contrat

Ce document dÃ©finit le **Master Butler Access Control Contract** : un contrat normatif, non nÃ©gociable, et de statut FONDATION qui Ã©tablit les rÃ¨gles et mÃ©canismes de contrÃ´le d'accÃ¨s aux APIs et fonctions de Master Butler dans le systÃ¨me Miyukini Core System v2.4.

Ce contrat prÃ©cise qui peut accÃ©der Ã  Master Butler, sous quelles conditions, avec quels niveaux d'autoritÃ©, et selon quelles rÃ¨gles. Il dÃ©finit les contrÃ´les d'accÃ¨s aux opÃ©rations de dÃ©finition, d'interrogation, et de gestion des capacitÃ©s et permissions.

**Important :** Ce contrat dÃ©finit le contrÃ´le d'accÃ¨s Ã  Master Butler lui-mÃªme, pas le contrÃ´le d'accÃ¨s aux ressources mÃ©tier du systÃ¨me. Master Butler dÃ©finit les permissions, mais ne vÃ©rifie jamais si elles sont accordÃ©es â€” cette responsabilitÃ© appartient Ã  StrongFather.

### PortÃ©e

Ce contrat s'applique Ã  **tous les composants** interagissant avec Master Butler et dÃ©finit de maniÃ¨re absolue :

- Les catÃ©gories d'appelants autorisÃ©s Ã  accÃ©der Ã  Master Butler
- Les niveaux d'autoritÃ© et leurs privilÃ¨ges associÃ©s
- Les rÃ¨gles d'accÃ¨s aux diffÃ©rentes APIs (Capability, Permission, Discovery)
- Les contrÃ´les d'accÃ¨s aux opÃ©rations sensibles
- L'adaptation du contrÃ´le d'accÃ¨s selon les niveaux de sÃ©curitÃ© (0-4)
- Les invariants de contrÃ´le d'accÃ¨s non nÃ©gociables
- Les comportements en cas de violation d'accÃ¨s

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il Ã©tablit des rÃ¨gles absolues qui ne peuvent Ãªtre contournÃ©es, nÃ©gociÃ©es, ou modifiÃ©es. Le contrat prime sur toute considÃ©ration pratique.

### Relation avec les autres contrats

Ce contrat complÃ¨te et respecte les documents contractuels existants :

- **Master Butler â€” Documentation Fondatrice** : DÃ©finit la nature et les responsabilitÃ©s de Master Butler
- **Master Butler â€” Authority Limits Contract** : DÃ©finit les limites d'autoritÃ© de Master Butler
- **Master Butler â€” Capability API Contract** : DÃ©finit les opÃ©rations sur les capacitÃ©s
- **Master Butler â€” Permission API Contract** : DÃ©finit les opÃ©rations sur les permissions
- **Master Butler â€” Threat Model Contract** : DÃ©finit les menaces de sÃ©curitÃ© (complÃ©mentaire)
- **[Miyukini Conceptual References â€” Security Protocols](..//..//..//..//miyukini-webway-system//reference//_index.md)** : Protocoles de sÃ©curitÃ© (RT-SEC-2, RT-SEC-3, AS-SEC-3)
- **[Miyukini Conceptual References â€” Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md)** : Niveaux de sÃ©curitÃ© (0-4)
- **[Miyukini Conceptual References â€” Lois Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md)** : Ce contrat respecte **LOI-1** (aucune dÃ©pendance externe critique)

---

## 2. Principes fondamentaux du contrÃ´le d'accÃ¨s

### Principe de sÃ©paration dÃ©finition/dÃ©cision

Master Butler contrÃ´le l'accÃ¨s Ã  ses propres APIs, mais ne dÃ©cide jamais de l'autorisation finale d'une action mÃ©tier.

| ResponsabilitÃ© | PropriÃ©taire | Ce que fait Master Butler |
|----------------|--------------|---------------------------|
| **ContrÃ´le d'accÃ¨s aux APIs Master Butler** | Master Butler | âœ… ContrÃ´le qui peut utiliser ses APIs |
| **VÃ©rification des permissions mÃ©tier** | StrongFather | âŒ Ne vÃ©rifie jamais |
| **DÃ©cision d'autorisation finale** | StrongFather | âŒ Ne dÃ©cide jamais |

**RÃ¨gle absolue :**

> **Master Butler contrÃ´le l'accÃ¨s Ã  son registre, jamais l'accÃ¨s aux ressources mÃ©tier du systÃ¨me.**

### Principe de moindre privilÃ¨ge

Chaque appelant reÃ§oit uniquement le niveau d'accÃ¨s nÃ©cessaire Ã  sa fonction dans le systÃ¨me. Aucun accÃ¨s excessif n'est accordÃ©.

### Principe de dÃ©fense en profondeur

Le contrÃ´le d'accÃ¨s Ã  Master Butler s'inscrit dans une chaÃ®ne de contrÃ´les :

1. **Border Guard** : Classification de la source (avant Master Butler)
2. **Master Butler** : ContrÃ´le d'accÃ¨s Ã  ses APIs (ce contrat)
3. **StrongFather** : DÃ©cision d'autorisation finale (aprÃ¨s Master Butler)

### Principe de traÃ§abilitÃ©

Tout accÃ¨s Ã  Master Butler est tracÃ©. Les tentatives d'accÃ¨s non autorisÃ©es sont enregistrÃ©es et peuvent dÃ©clencher des alertes.

---

## 3. CatÃ©gories d'appelants

### 3.1. DÃ©finition formelle

Un **appelant** est tout composant, systÃ¨me, ou processus qui soumet une requÃªte Ã  Master Butler via ses APIs. Chaque appelant est identifiÃ© et classifiÃ© selon sa catÃ©gorie.

### 3.2. CatÃ©gories reconnues

| CatÃ©gorie | Code | Description | Niveau de confiance |
|-----------|------|-------------|---------------------|
| **Core SystÃ¨me** | `CORE` | Cores de Strate 4 (StrongFather, KindMother, etc.) | Maximal |
| **Service SystÃ¨me** | `SERVICE` | Services de Strate 5 (Border Guard, TAMR, etc.) | Ã‰levÃ© |
| **OpÃ©rateur** | `OPERATOR` | OpÃ©rateurs et produits enregistrÃ©s | Standard |
| **Administration** | `ADMIN` | MiyukiniAdmin et outils d'administration | Critique |
| **Bootstrap** | `BOOTSTRAP` | Processus de dÃ©marrage du systÃ¨me | Temporaire |

### 3.3. CaractÃ©ristiques par catÃ©gorie

#### CORE â€” Cores SystÃ¨me

**IdentitÃ© :** StrongFather, KindMother, BondingBrother, Ever Buddy, Caring Nanny, WorrySentinel

**Niveau de confiance :** Maximal

**AccÃ¨s accordÃ© :**
- âœ… Interrogation complÃ¨te (toutes les APIs)
- âœ… AccÃ¨s aux permissions de tous niveaux (STANDARD Ã  SYSTEM)
- âš ï¸ DÃ©finition limitÃ©e (selon le Core)
- âŒ Modification des permissions SYSTEM (sauf StrongFather)

**Justification :** Les Cores sont des composants fondamentaux du systÃ¨me avec des responsabilitÃ©s critiques. Ils nÃ©cessitent un accÃ¨s complet aux informations pour remplir leurs fonctions.

#### SERVICE â€” Services SystÃ¨me

**IdentitÃ© :** Border Guard, Kernel Logger, etc.

**Niveau de confiance :** Ã‰levÃ©

**AccÃ¨s accordÃ© :**
- âœ… Interrogation des capacitÃ©s et permissions
- âœ… DÃ©couverte des Outils et Kits d'Outils
- âŒ DÃ©finition de permissions
- âŒ AccÃ¨s aux permissions SYSTEM

**Justification :** Les Services ont besoin d'interroger Master Butler pour leurs fonctions, mais n'ont pas besoin de dÃ©finir des permissions.

#### OPERATOR â€” OpÃ©rateurs

**IdentitÃ© :** Produits et OpÃ©rateurs enregistrÃ©s dans le systÃ¨me

**Niveau de confiance :** Standard

**AccÃ¨s accordÃ© :**
- âœ… DÃ©claration de capacitÃ©s (propres au module)
- âœ… DÃ©finition de permissions STANDARD et ELEVATED (propres au module)
- âœ… Interrogation des capacitÃ©s et permissions (selon scope)
- âœ… DÃ©couverte des capacitÃ©s disponibles
- âŒ DÃ©finition de permissions CRITICAL ou SYSTEM
- âŒ AccÃ¨s aux permissions d'autres OpÃ©rateurs (sauf publiques)

**Justification :** Les OpÃ©rateurs dÃ©finissent leurs propres capacitÃ©s et permissions, mais ne peuvent pas dÃ©finir de permissions critiques ou accÃ©der aux donnÃ©es d'autres OpÃ©rateurs.

#### ADMIN â€” Administration

**IdentitÃ© :** MiyukiniAdmin, outils d'administration autorisÃ©s

**Niveau de confiance :** Critique

**AccÃ¨s accordÃ© :**
- âœ… AccÃ¨s complet Ã  toutes les APIs
- âœ… DÃ©finition de permissions de tous niveaux (incluant SYSTEM)
- âœ… Modification des permissions SYSTEM
- âœ… Audit complet du registre
- âœ… OpÃ©rations de maintenance

**Justification :** L'administration systÃ¨me nÃ©cessite un accÃ¨s complet pour la gestion, l'audit, et la maintenance.

#### BOOTSTRAP â€” Processus de dÃ©marrage

**IdentitÃ© :** Processus de dÃ©marrage du systÃ¨me

**Niveau de confiance :** Temporaire (limitÃ© Ã  la phase de bootstrap)

**AccÃ¨s accordÃ© :**
- âœ… Enregistrement initial des capacitÃ©s fondamentales
- âœ… DÃ©finition des permissions de base
- âŒ AccÃ¨s aprÃ¨s la phase de bootstrap

**Justification :** Le bootstrap nÃ©cessite un accÃ¨s temporaire pour initialiser le systÃ¨me, mais cet accÃ¨s est rÃ©voquÃ© aprÃ¨s initialisation.

---

## 4. Niveaux d'autoritÃ©

### 4.1. DÃ©finition formelle

Un **niveau d'autoritÃ©** est un ensemble de privilÃ¨ges attribuÃ© Ã  un appelant, dÃ©terminant les opÃ©rations qu'il peut effectuer sur Master Butler.

### 4.2. HiÃ©rarchie des niveaux

| Niveau | Nom | PrivilÃ¨ges | CatÃ©gories autorisÃ©es |
|--------|-----|------------|----------------------|
| **0** | Lecture publique | Interrogation des capacitÃ©s publiques | Tous |
| **1** | Lecture standard | Interrogation complÃ¨te | OPERATOR, SERVICE, CORE, ADMIN |
| **2** | DÃ©finition standard | DÃ©finition de permissions STANDARD/ELEVATED | OPERATOR, CORE, ADMIN |
| **3** | DÃ©finition critique | DÃ©finition de permissions CRITICAL | CORE (StrongFather), ADMIN |
| **4** | DÃ©finition systÃ¨me | DÃ©finition de permissions SYSTEM | ADMIN uniquement |
| **5** | Administration | Toutes opÃ©rations, audit, maintenance | ADMIN uniquement |

### 4.3. Matrice d'accÃ¨s par niveau d'autoritÃ©

| OpÃ©ration | Niveau 0 | Niveau 1 | Niveau 2 | Niveau 3 | Niveau 4 | Niveau 5 |
|-----------|----------|----------|----------|----------|----------|----------|
| Lire capacitÃ©s publiques | âœ… | âœ… | âœ… | âœ… | âœ… | âœ… |
| Lire toutes capacitÃ©s | âŒ | âœ… | âœ… | âœ… | âœ… | âœ… |
| Lire permissions STANDARD | âŒ | âœ… | âœ… | âœ… | âœ… | âœ… |
| Lire permissions ELEVATED | âŒ | âœ… | âœ… | âœ… | âœ… | âœ… |
| Lire permissions CRITICAL | âŒ | âœ… | âœ… | âœ… | âœ… | âœ… |
| Lire permissions SYSTEM | âŒ | âŒ | âŒ | âœ… | âœ… | âœ… |
| DÃ©clarer capacitÃ©s | âŒ | âŒ | âœ… | âœ… | âœ… | âœ… |
| DÃ©finir permissions STANDARD | âŒ | âŒ | âœ… | âœ… | âœ… | âœ… |
| DÃ©finir permissions ELEVATED | âŒ | âŒ | âœ… | âœ… | âœ… | âœ… |
| DÃ©finir permissions CRITICAL | âŒ | âŒ | âŒ | âœ… | âœ… | âœ… |
| DÃ©finir permissions SYSTEM | âŒ | âŒ | âŒ | âŒ | âœ… | âœ… |
| Modifier permissions SYSTEM | âŒ | âŒ | âŒ | âŒ | âŒ | âœ… |
| Audit complet | âŒ | âŒ | âŒ | âŒ | âŒ | âœ… |
| Maintenance | âŒ | âŒ | âŒ | âŒ | âŒ | âœ… |

### 4.4. Attribution des niveaux par catÃ©gorie

| CatÃ©gorie | Niveau d'autoritÃ© par dÃ©faut | Niveau maximal atteignable |
|-----------|------------------------------|----------------------------|
| CORE (StrongFather) | 3 | 3 |
| CORE (autres) | 1 | 2 |
| SERVICE | 1 | 1 |
| OPERATOR | 1 | 2 |
| ADMIN | 5 | 5 |
| BOOTSTRAP | 2 | 2 |

---

## 5. RÃ¨gles d'accÃ¨s aux APIs

### 5.1. Capability API

#### OpÃ©rations de lecture

| OpÃ©ration | Niveau requis | Scope autorisÃ© |
|-----------|--------------|----------------|
| `getCapability` | 0 (public), 1 (complet) | Selon visibilitÃ© |
| `listCapabilities` | 0 (public), 1 (complet) | Selon visibilitÃ© |
| `searchCapabilities` | 1 | Selon visibilitÃ© |
| `getCapabilityHierarchy` | 1 | Selon visibilitÃ© |

#### OpÃ©rations de dÃ©finition

| OpÃ©ration | Niveau requis | Conditions supplÃ©mentaires |
|-----------|--------------|---------------------------|
| `declareCapability` | 2 | Module propre uniquement |
| `activateCapability` | 2 | CapacitÃ© propre uniquement |
| `updateCapability` | 2 | CapacitÃ© propre uniquement |
| `deprecateCapability` | 2 | CapacitÃ© propre uniquement |
| `retireCapability` | 5 | Administration uniquement |

**RÃ¨gle de scope :** Un OpÃ©rateur ne peut dÃ©clarer, modifier, ou dÃ©prÃ©cier que les capacitÃ©s de son propre module. Cette restriction est absolue et non contournable.

### 5.2. Permission API

#### OpÃ©rations de lecture

| OpÃ©ration | Niveau requis | Restrictions |
|-----------|--------------|--------------|
| `getPermission` | 1 | Selon niveau de permission |
| `listPermissions` | 1 | Selon niveau de permission |
| `searchPermissions` | 1 | Selon niveau de permission |
| `getPermissionCapabilities` | 1 | Selon niveau de permission |
| `getPermissionHierarchy` | 1 | Selon niveau de permission |

**Restriction par niveau de permission :**

- Permissions STANDARD/ELEVATED : Niveau d'autoritÃ© 1+
- Permissions CRITICAL : Niveau d'autoritÃ© 3+
- Permissions SYSTEM : Niveau d'autoritÃ© 3+ (lecture), 5 (modification)

#### OpÃ©rations de dÃ©finition

| OpÃ©ration | Niveau de permission cible | Niveau d'autoritÃ© requis |
|-----------|---------------------------|-------------------------|
| `definePermission` | STANDARD | 2 |
| `definePermission` | ELEVATED | 2 |
| `definePermission` | CRITICAL | 3 |
| `definePermission` | SYSTEM | 4 |
| `activatePermission` | Selon niveau | Selon niveau de la permission |
| `updatePermission` | Selon niveau | Selon niveau de la permission |
| `deprecatePermission` | Selon niveau | Selon niveau de la permission |
| `retirePermission` | Tous | 5 |

**RÃ¨gle de scope :** Un OpÃ©rateur ne peut dÃ©finir des permissions que pour les capacitÃ©s qu'il possÃ¨de, sauf pour les Cores et l'administration.

### 5.3. Discovery API

| OpÃ©ration | Niveau requis | RÃ©sultat |
|-----------|--------------|----------|
| `discoverCapabilities` | 0 (public), 1 (complet) | CapacitÃ©s filtrÃ©es selon visibilitÃ© |
| `discoverPermissions` | 1 | Permissions filtrÃ©es selon niveau |
| `discoverTools` | 0 (public), 1 (complet) | Outils filtrÃ©s selon visibilitÃ© |
| `discoverToolkits` | 0 (public), 1 (complet) | Kits d'Outils filtrÃ©s selon visibilitÃ© |
| `getCapabilityContext` | 1 | Contexte de capacitÃ© complet |

---

## 6. ContrÃ´le d'accÃ¨s adaptatif selon les niveaux de sÃ©curitÃ©

### 6.1. Principe d'adaptation

Master Butler adapte son contrÃ´le d'accÃ¨s selon le niveau de sÃ©curitÃ© dÃ©clarÃ© par l'OpÃ©rateur (0-4), conformÃ©ment au document [Miyukini Conceptual References - Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md).

### 6.2. Adaptation par niveau de sÃ©curitÃ©

#### Niveau 0 â€” PUBLIC / DISPLAY

| Aspect | Comportement |
|--------|--------------|
| **Authentification** | Non obligatoire pour la lecture publique |
| **VÃ©rification d'autoritÃ©** | SimplifiÃ©e |
| **TraÃ§abilitÃ©** | Minimale |
| **ContrÃ´les** | Validation structurelle uniquement |

#### Niveau 1 â€” STANDARD / CMS

| Aspect | Comportement |
|--------|--------------|
| **Authentification** | Obligatoire pour les opÃ©rations de dÃ©finition |
| **VÃ©rification d'autoritÃ©** | Standard |
| **TraÃ§abilitÃ©** | Normale |
| **ContrÃ´les** | Validation complÃ¨te |

#### Niveau 2 â€” SENSITIVE DATA

| Aspect | Comportement |
|--------|--------------|
| **Authentification** | Obligatoire pour toutes les opÃ©rations |
| **VÃ©rification d'autoritÃ©** | RenforcÃ©e |
| **TraÃ§abilitÃ©** | ComplÃ¨te |
| **ContrÃ´les** | Validation stricte + vÃ©rification de cohÃ©rence |

#### Niveau 3 â€” CRITICAL SYSTEM

| Aspect | Comportement |
|--------|--------------|
| **Authentification** | Obligatoire avec vÃ©rification croisÃ©e |
| **VÃ©rification d'autoritÃ©** | Stricte |
| **TraÃ§abilitÃ©** | Absolue |
| **ContrÃ´les** | Validation stricte + signatures |

#### Niveau 4 â€” HARDENED / ISOLATED

| Aspect | Comportement |
|--------|--------------|
| **Authentification** | Obligatoire avec attestations |
| **VÃ©rification d'autoritÃ©** | Maximale |
| **TraÃ§abilitÃ©** | Absolue + audit temps rÃ©el |
| **ContrÃ´les** | Validation continue + restrictions maximales |

### 6.3. Restrictions supplÃ©mentaires par niveau

| Niveau de sÃ©curitÃ© | Restrictions supplÃ©mentaires |
|--------------------|------------------------------|
| 0-1 | Aucune restriction supplÃ©mentaire |
| 2 | DÃ©finition de permissions ELEVATED requiert confirmation |
| 3 | DÃ©finition de permissions nÃ©cessite validation StrongFather |
| 4 | Seules les opÃ©rations de lecture sont autorisÃ©es sans validation explicite |

---

## 7. ContrÃ´le d'accÃ¨s au scope

### 7.1. DÃ©finition du scope

Le **scope** dÃ©finit le pÃ©rimÃ¨tre d'accÃ¨s d'un appelant aux capacitÃ©s et permissions du registre.

### 7.2. Types de scope

| Type | Description | Applicable Ã  |
|------|-------------|--------------|
| **GLOBAL** | AccÃ¨s Ã  tout le registre | ADMIN, CORE (StrongFather) |
| **MODULE** | AccÃ¨s aux capacitÃ©s/permissions du module | OPERATOR |
| **PUBLIC** | AccÃ¨s aux capacitÃ©s/permissions publiques | Tous |

### 7.3. RÃ¨gles de scope

**SCOPE-1 : Isolation des OpÃ©rateurs**

Un OpÃ©rateur n'accÃ¨de qu'aux capacitÃ©s et permissions de son propre module, plus les capacitÃ©s/permissions publiques.

**SCOPE-2 : AccÃ¨s global des Cores**

StrongFather et l'administration ont un accÃ¨s global au registre pour leurs fonctions respectives.

**SCOPE-3 : VisibilitÃ© des permissions**

La visibilitÃ© des permissions suit la hiÃ©rarchie des niveaux de criticitÃ© :
- STANDARD : Visibles par tous les appelants autorisÃ©s
- ELEVATED : Visibles par les appelants de niveau 1+
- CRITICAL : Visibles par les appelants de niveau 3+
- SYSTEM : Visibles par les appelants de niveau 3+ (lecture), 5 (modification)

### 7.4. DÃ©termination du scope

Le scope d'un appelant est dÃ©terminÃ© par :

1. **CatÃ©gorie** : CORE, SERVICE, OPERATOR, ADMIN
2. **IdentitÃ©** : Identifiant de l'appelant
3. **Module** : Module d'appartenance (pour les OpÃ©rateurs)
4. **Contexte** : Contexte de l'appel (niveau de sÃ©curitÃ©, etc.)

---

## 8. Validation du contexte d'appel

### 8.1. Structure du contexte

Chaque appel Ã  Master Butler inclut un contexte validÃ© :

```
Access Context
â”œâ”€â”€ Caller Identity
â”‚   â”œâ”€â”€ caller_id: <identifiant unique>
â”‚   â”œâ”€â”€ category: <CORE | SERVICE | OPERATOR | ADMIN | BOOTSTRAP>
â”‚   â””â”€â”€ authority_level: <0-5>
â”œâ”€â”€ Scope
â”‚   â”œâ”€â”€ scope_type: <GLOBAL | MODULE | PUBLIC>
â”‚   â””â”€â”€ module_id: <identifiant du module si applicable>
â”œâ”€â”€ Security
â”‚   â”œâ”€â”€ security_level: <0-4>
â”‚   â”œâ”€â”€ authenticated: <boolean>
â”‚   â””â”€â”€ authentication_method: <mÃ©thode si authentifiÃ©>
â””â”€â”€ Trace
    â”œâ”€â”€ request_id: <identifiant unique de requÃªte>
    â”œâ”€â”€ correlation_id: <identifiant de corrÃ©lation>
    â””â”€â”€ timestamp: <horodatage>
```

### 8.2. Validations obligatoires

| Validation | Description | Erreur si Ã©choue |
|------------|-------------|------------------|
| **V-CTX-1** | IdentitÃ© de l'appelant prÃ©sente | `MISSING_CALLER_IDENTITY` |
| **V-CTX-2** | CatÃ©gorie reconnue | `UNKNOWN_CALLER_CATEGORY` |
| **V-CTX-3** | Niveau d'autoritÃ© cohÃ©rent avec la catÃ©gorie | `INVALID_AUTHORITY_LEVEL` |
| **V-CTX-4** | Scope valide pour la catÃ©gorie | `INVALID_SCOPE` |
| **V-CTX-5** | Niveau de sÃ©curitÃ© dÃ©clarÃ© | `MISSING_SECURITY_LEVEL` |
| **V-CTX-6** | Authentification si requise | `AUTHENTICATION_REQUIRED` |

### 8.3. Ordre de validation

1. PrÃ©sence et structure du contexte
2. IdentitÃ© de l'appelant
3. CatÃ©gorie et niveau d'autoritÃ©
4. Authentification (si requise par le niveau de sÃ©curitÃ©)
5. Scope et visibilitÃ©
6. Autorisation pour l'opÃ©ration demandÃ©e

---

## 9. Comportements en cas de violation d'accÃ¨s

### 9.1. Types de violations

| Code | Type | Description | GravitÃ© |
|------|------|-------------|---------|
| `V-ACC-1` | IdentitÃ© invalide | Appelant non identifiÃ© ou identitÃ© invalide | Critique |
| `V-ACC-2` | AutoritÃ© insuffisante | Niveau d'autoritÃ© insuffisant pour l'opÃ©ration | Haute |
| `V-ACC-3` | Scope dÃ©passÃ© | Tentative d'accÃ¨s hors du scope autorisÃ© | Haute |
| `V-ACC-4` | OpÃ©ration interdite | OpÃ©ration non autorisÃ©e pour la catÃ©gorie | Haute |
| `V-ACC-5` | Niveau de permission | AccÃ¨s Ã  un niveau de permission non autorisÃ© | Moyenne |
| `V-ACC-6` | Authentification | Authentification requise mais non fournie | Moyenne |

### 9.2. Comportement par type de violation

#### V-ACC-1 : IdentitÃ© invalide

**Comportement :**
- Rejet immÃ©diat de l'appel
- Erreur : `ACCESS_DENIED_UNKNOWN_CALLER`
- TraÃ§abilitÃ© : Enregistrement de la tentative avec informations disponibles
- Alerte : Possible selon la politique de sÃ©curitÃ©

#### V-ACC-2 : AutoritÃ© insuffisante

**Comportement :**
- Rejet de l'appel
- Erreur : `ACCESS_DENIED_INSUFFICIENT_AUTHORITY`
- TraÃ§abilitÃ© : Enregistrement de la tentative avec identitÃ©
- Information : Niveau requis vs niveau fourni

#### V-ACC-3 : Scope dÃ©passÃ©

**Comportement :**
- Rejet de l'appel
- Erreur : `ACCESS_DENIED_SCOPE_VIOLATION`
- TraÃ§abilitÃ© : Enregistrement de la tentative
- Information : Scope autorisÃ© vs scope demandÃ©

#### V-ACC-4 : OpÃ©ration interdite

**Comportement :**
- Rejet de l'appel
- Erreur : `ACCESS_DENIED_OPERATION_FORBIDDEN`
- TraÃ§abilitÃ© : Enregistrement de la tentative
- Information : OpÃ©ration demandÃ©e et raison du refus

#### V-ACC-5 : Niveau de permission

**Comportement :**
- Rejet de l'appel ou filtrage des rÃ©sultats
- Erreur : `ACCESS_DENIED_PERMISSION_LEVEL` (si rejet)
- Comportement alternatif : Retour d'un sous-ensemble autorisÃ© (si filtrage)

#### V-ACC-6 : Authentification

**Comportement :**
- Rejet de l'appel
- Erreur : `AUTHENTICATION_REQUIRED`
- Information : Type d'authentification requis

### 9.3. Garanties aprÃ¨s violation

- L'Ã©tat du registre reste inchangÃ©
- Aucune modification partielle n'est appliquÃ©e
- L'erreur est explicite et actionnable
- La tentative est tracÃ©e pour audit
- Aucun effet de bord n'est crÃ©Ã©

---

## 10. IntÃ©gration avec les protocoles de sÃ©curitÃ©

### 10.1. Protocole RT-SEC-2 â€” Authentification en couches

Master Butler participe au protocole RT-SEC-2 (Authentification en couches) :

```
RequÃªte
    â†“
Border Guard (classification source)
    â†“
Master Butler (capacitÃ©s disponibles ?)  â† Ce contrat s'applique ici
    â†“
Caring Nanny (Ã©tat systÃ¨me ?)
    â†“
StrongFather (dÃ©cision finale)
```

**RÃ´le de Master Butler :** Fournir les capacitÃ©s et permissions disponibles pour le contexte, en appliquant les contrÃ´les d'accÃ¨s dÃ©finis dans ce contrat.

### 10.2. Protocole RT-SEC-3 â€” Validation systÃ©matique

Master Butler applique la validation systÃ©matique :

- âœ… Validation du contexte d'appel
- âœ… Validation de l'autoritÃ© de l'appelant
- âœ… Validation du scope demandÃ©
- âœ… Validation de l'opÃ©ration demandÃ©e

**RÃ¨gle absolue :** Aucune optimisation ne peut court-circuiter ces validations, mÃªme en temps rÃ©el.

### 10.3. Protocole AS-SEC-3 â€” Revalidation Ã  la reprise

Lors du retour en ligne (protocole AS-SEC-3), Master Butler revalide :

- L'autoritÃ© de l'appelant
- Le scope autorisÃ©
- Les permissions accessibles selon le niveau de sÃ©curitÃ©

---

## 11. Invariants de contrÃ´le d'accÃ¨s

### INV-ACC-1 : Identification obligatoire

Tout appel Ã  Master Butler DOIT Ãªtre accompagnÃ© d'une identification de l'appelant. Aucun appel anonyme n'est traitÃ©, sauf pour les opÃ©rations de lecture publique au niveau de sÃ©curitÃ© 0.

### INV-ACC-2 : Validation avant exÃ©cution

Toute opÃ©ration est validÃ©e AVANT exÃ©cution. Aucune opÃ©ration n'est exÃ©cutÃ©e partiellement avant validation complÃ¨te.

### INV-ACC-3 : Moindre privilÃ¨ge

Un appelant reÃ§oit uniquement le niveau d'accÃ¨s nÃ©cessaire Ã  sa fonction. Aucun privilÃ¨ge excessif n'est accordÃ©.

### INV-ACC-4 : Isolation des OpÃ©rateurs

Un OpÃ©rateur n'accÃ¨de jamais aux capacitÃ©s ou permissions d'un autre OpÃ©rateur, sauf si elles sont publiques.

### INV-ACC-5 : TraÃ§abilitÃ© complÃ¨te

Toute tentative d'accÃ¨s (rÃ©ussie ou refusÃ©e) est tracÃ©e avec contexte complet.

### INV-ACC-6 : Non-contournabilitÃ©

Le contrÃ´le d'accÃ¨s ne peut pas Ãªtre contournÃ©. Aucun chemin alternatif n'existe pour accÃ©der au registre sans passer par les validations.

### INV-ACC-7 : Adaptation au niveau de sÃ©curitÃ©

Le contrÃ´le d'accÃ¨s s'adapte au niveau de sÃ©curitÃ© dÃ©clarÃ©, sans exception.

### INV-ACC-8 : SÃ©paration dÃ©finition/dÃ©cision

Master Butler contrÃ´le l'accÃ¨s Ã  son registre mais ne prend jamais de dÃ©cision d'autorisation mÃ©tier. Cette responsabilitÃ© appartient Ã  StrongFather.

---

## 12. SchÃ©mas ASCII

### 12.1. Flux de contrÃ´le d'accÃ¨s

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                  FLUX DE CONTRÃ”LE D'ACCÃˆS MASTER BUTLER                  â”‚
â”‚                                                                          â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚                      APPELANT                                      â”‚  â”‚
â”‚  â”‚  (CORE, SERVICE, OPERATOR, ADMIN, BOOTSTRAP)                       â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚                                   â”‚                                      â”‚
â”‚                                   â”‚ Appel avec contexte                  â”‚
â”‚                                   â–¼                                      â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚              VALIDATION DU CONTEXTE (V-CTX)                        â”‚  â”‚
â”‚  â”‚                                                                     â”‚  â”‚
â”‚  â”‚  â”œâ”€â”€ V-CTX-1 : IdentitÃ© prÃ©sente ?         â”€â”€â†’ REJET si non       â”‚  â”‚
â”‚  â”‚  â”œâ”€â”€ V-CTX-2 : CatÃ©gorie reconnue ?        â”€â”€â†’ REJET si non       â”‚  â”‚
â”‚  â”‚  â”œâ”€â”€ V-CTX-3 : Niveau d'autoritÃ© valide ?  â”€â”€â†’ REJET si non       â”‚  â”‚
â”‚  â”‚  â”œâ”€â”€ V-CTX-4 : Scope valide ?              â”€â”€â†’ REJET si non       â”‚  â”‚
â”‚  â”‚  â”œâ”€â”€ V-CTX-5 : Niveau sÃ©curitÃ© dÃ©clarÃ© ?   â”€â”€â†’ REJET si non       â”‚  â”‚
â”‚  â”‚  â””â”€â”€ V-CTX-6 : Authentification si requise â”€â”€â†’ REJET si non       â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚                                   â”‚                                      â”‚
â”‚                                   â”‚ Contexte validÃ©                      â”‚
â”‚                                   â–¼                                      â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚              VÃ‰RIFICATION DE L'AUTORITÃ‰                            â”‚  â”‚
â”‚  â”‚                                                                     â”‚  â”‚
â”‚  â”‚  â”œâ”€â”€ Niveau d'autoritÃ© suffisant pour l'opÃ©ration ?               â”‚  â”‚
â”‚  â”‚  â”‚       â”€â”€â†’ REJET si non (V-ACC-2)                               â”‚  â”‚
â”‚  â”‚  â”‚                                                                 â”‚  â”‚
â”‚  â”‚  â”œâ”€â”€ Scope couvre la ressource demandÃ©e ?                         â”‚  â”‚
â”‚  â”‚  â”‚       â”€â”€â†’ REJET si non (V-ACC-3)                               â”‚  â”‚
â”‚  â”‚  â”‚                                                                 â”‚  â”‚
â”‚  â”‚  â””â”€â”€ OpÃ©ration autorisÃ©e pour la catÃ©gorie ?                      â”‚  â”‚
â”‚  â”‚          â”€â”€â†’ REJET si non (V-ACC-4)                               â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚                                   â”‚                                      â”‚
â”‚                                   â”‚ AutoritÃ© vÃ©rifiÃ©e                    â”‚
â”‚                                   â–¼                                      â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚              EXÃ‰CUTION DE L'OPÃ‰RATION                              â”‚  â”‚
â”‚  â”‚                                                                     â”‚  â”‚
â”‚  â”‚  â€¢ OpÃ©ration exÃ©cutÃ©e selon les rÃ¨gles des APIs                   â”‚  â”‚
â”‚  â”‚  â€¢ RÃ©sultats filtrÃ©s selon le scope autorisÃ©                      â”‚  â”‚
â”‚  â”‚  â€¢ TraÃ§abilitÃ© enregistrÃ©e                                        â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚                                   â”‚                                      â”‚
â”‚                                   â”‚ RÃ©sultat                             â”‚
â”‚                                   â–¼                                      â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚                      APPELANT                                      â”‚  â”‚
â”‚  â”‚  (ReÃ§oit le rÃ©sultat ou l'erreur explicite)                       â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 12.2. Matrice d'accÃ¨s par catÃ©gorie

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚           MATRICE D'ACCÃˆS PAR CATÃ‰GORIE D'APPELANT                       â”‚
â”‚                                                                          â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚ OPÃ‰RATION          â”‚ CORE â”‚ SERVICE â”‚ OPERATOR â”‚ ADMIN â”‚ BOOTSTRAP â”‚ â”‚
â”‚  â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤ â”‚
â”‚  â”‚ LECTURE            â”‚      â”‚         â”‚          â”‚       â”‚           â”‚ â”‚
â”‚  â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€         â”‚      â”‚         â”‚          â”‚       â”‚           â”‚ â”‚
â”‚  â”‚ CapacitÃ©s publiquesâ”‚  âœ…  â”‚   âœ…    â”‚    âœ…    â”‚  âœ…   â”‚    âœ…     â”‚ â”‚
â”‚  â”‚ CapacitÃ©s complÃ¨tesâ”‚  âœ…  â”‚   âœ…    â”‚    âš ï¸    â”‚  âœ…   â”‚    âŒ     â”‚ â”‚
â”‚  â”‚ Perms STANDARD     â”‚  âœ…  â”‚   âœ…    â”‚    âœ…    â”‚  âœ…   â”‚    âŒ     â”‚ â”‚
â”‚  â”‚ Perms ELEVATED     â”‚  âœ…  â”‚   âœ…    â”‚    âœ…    â”‚  âœ…   â”‚    âŒ     â”‚ â”‚
â”‚  â”‚ Perms CRITICAL     â”‚  âœ…  â”‚   âŒ    â”‚    âŒ    â”‚  âœ…   â”‚    âŒ     â”‚ â”‚
â”‚  â”‚ Perms SYSTEM       â”‚  âš ï¸  â”‚   âŒ    â”‚    âŒ    â”‚  âœ…   â”‚    âŒ     â”‚ â”‚
â”‚  â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤ â”‚
â”‚  â”‚ DÃ‰FINITION         â”‚      â”‚         â”‚          â”‚       â”‚           â”‚ â”‚
â”‚  â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€         â”‚      â”‚         â”‚          â”‚       â”‚           â”‚ â”‚
â”‚  â”‚ DÃ©clarer capacitÃ©s â”‚  âš ï¸  â”‚   âŒ    â”‚    âœ…    â”‚  âœ…   â”‚    âœ…     â”‚ â”‚
â”‚  â”‚ DÃ©finir STANDARD   â”‚  âš ï¸  â”‚   âŒ    â”‚    âœ…    â”‚  âœ…   â”‚    âœ…     â”‚ â”‚
â”‚  â”‚ DÃ©finir ELEVATED   â”‚  âš ï¸  â”‚   âŒ    â”‚    âœ…    â”‚  âœ…   â”‚    âœ…     â”‚ â”‚
â”‚  â”‚ DÃ©finir CRITICAL   â”‚  âš ï¸  â”‚   âŒ    â”‚    âŒ    â”‚  âœ…   â”‚    âŒ     â”‚ â”‚
â”‚  â”‚ DÃ©finir SYSTEM     â”‚  âŒ  â”‚   âŒ    â”‚    âŒ    â”‚  âœ…   â”‚    âŒ     â”‚ â”‚
â”‚  â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤ â”‚
â”‚  â”‚ ADMINISTRATION     â”‚      â”‚         â”‚          â”‚       â”‚           â”‚ â”‚
â”‚  â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€     â”‚      â”‚         â”‚          â”‚       â”‚           â”‚ â”‚
â”‚  â”‚ Modifier SYSTEM    â”‚  âŒ  â”‚   âŒ    â”‚    âŒ    â”‚  âœ…   â”‚    âŒ     â”‚ â”‚
â”‚  â”‚ Audit complet      â”‚  âŒ  â”‚   âŒ    â”‚    âŒ    â”‚  âœ…   â”‚    âŒ     â”‚ â”‚
â”‚  â”‚ Maintenance        â”‚  âŒ  â”‚   âŒ    â”‚    âŒ    â”‚  âœ…   â”‚    âŒ     â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚                                                                          â”‚
â”‚  LÃ‰GENDE :                                                               â”‚
â”‚  âœ… = AutorisÃ©                                                           â”‚
â”‚  âš ï¸ = AutorisÃ© avec restrictions (scope, niveau, etc.)                  â”‚
â”‚  âŒ = Interdit                                                           â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 12.3. Adaptation au niveau de sÃ©curitÃ©

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚        ADAPTATION DU CONTRÃ”LE D'ACCÃˆS AU NIVEAU DE SÃ‰CURITÃ‰             â”‚
â”‚                                                                          â”‚
â”‚  NIVEAU 0 â€” PUBLIC                                                       â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”‚
â”‚  â”‚  â€¢ Authentification : Non obligatoire (lecture publique)         â”‚   â”‚
â”‚  â”‚  â€¢ VÃ©rification : SimplifiÃ©e                                     â”‚   â”‚
â”‚  â”‚  â€¢ TraÃ§abilitÃ© : Minimale                                        â”‚   â”‚
â”‚  â”‚  â€¢ Impact : ðŸŸ¢ Quasi nul                                         â”‚   â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜   â”‚
â”‚                                                                          â”‚
â”‚  NIVEAU 1 â€” STANDARD                                                     â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”‚
â”‚  â”‚  â€¢ Authentification : Obligatoire pour dÃ©finition                â”‚   â”‚
â”‚  â”‚  â€¢ VÃ©rification : Standard                                       â”‚   â”‚
â”‚  â”‚  â€¢ TraÃ§abilitÃ© : Normale                                         â”‚   â”‚
â”‚  â”‚  â€¢ Impact : ðŸŸ¢ Faible                                            â”‚   â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜   â”‚
â”‚                                                                          â”‚
â”‚  NIVEAU 2 â€” SENSITIVE                                                    â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”‚
â”‚  â”‚  â€¢ Authentification : Obligatoire pour toutes opÃ©rations         â”‚   â”‚
â”‚  â”‚  â€¢ VÃ©rification : RenforcÃ©e                                      â”‚   â”‚
â”‚  â”‚  â€¢ TraÃ§abilitÃ© : ComplÃ¨te                                        â”‚   â”‚
â”‚  â”‚  â€¢ Restriction : DÃ©finition ELEVATED requiert confirmation       â”‚   â”‚
â”‚  â”‚  â€¢ Impact : ðŸŸ¡ ModÃ©rÃ©                                            â”‚   â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜   â”‚
â”‚                                                                          â”‚
â”‚  NIVEAU 3 â€” CRITICAL                                                     â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”‚
â”‚  â”‚  â€¢ Authentification : Obligatoire avec vÃ©rification croisÃ©e      â”‚   â”‚
â”‚  â”‚  â€¢ VÃ©rification : Stricte                                        â”‚   â”‚
â”‚  â”‚  â€¢ TraÃ§abilitÃ© : Absolue                                         â”‚   â”‚
â”‚  â”‚  â€¢ Restriction : DÃ©finition nÃ©cessite validation StrongFather    â”‚   â”‚
â”‚  â”‚  â€¢ Impact : ðŸŸ  AcceptÃ©                                           â”‚   â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜   â”‚
â”‚                                                                          â”‚
â”‚  NIVEAU 4 â€” HARDENED                                                     â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”‚
â”‚  â”‚  â€¢ Authentification : Obligatoire avec attestations              â”‚   â”‚
â”‚  â”‚  â€¢ VÃ©rification : Maximale                                       â”‚   â”‚
â”‚  â”‚  â€¢ TraÃ§abilitÃ© : Absolue + audit temps rÃ©el                      â”‚   â”‚
â”‚  â”‚  â€¢ Restriction : Lecture seule sans validation explicite         â”‚   â”‚
â”‚  â”‚  â€¢ Impact : ðŸ”´ Secondaire                                        â”‚   â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 13. ConformitÃ© aux Lois d'Autonomie SystÃ¨me

Ce contrat respecte les Lois d'Autonomie SystÃ¨me dÃ©finies dans [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md).

### LOI-1 : Aucune dÃ©pendance externe critique Ã  l'exÃ©cution

**ConformitÃ© :** Conforme

Le contrÃ´le d'accÃ¨s fonctionne entiÃ¨rement en local :

- **Validation locale** : Toutes les validations sont effectuÃ©es localement
- **Pas de service d'authentification externe obligatoire** : L'identitÃ© de l'appelant est fournie dans le contexte
- **Aucune API externe** : Le contrÃ´le d'accÃ¨s ne dÃ©pend d'aucun service distant

**VÃ©rification LOI-1** : *"Le contrÃ´le d'accÃ¨s fonctionne-t-il si le rÃ©seau est indisponible ?"* â†’ **Oui.**

### LOI-5 : Le coÃ»t doit Ãªtre proportionnel au hardware

**ConformitÃ© :** Conforme

Le contrÃ´le d'accÃ¨s a une empreinte minimale :

- **Validations lÃ©gÃ¨res** : Comparaisons simples d'identitÃ©s et de niveaux
- **Pas de cryptographie lourde** : Validation basÃ©e sur le contexte fourni
- **Pas de workers** : Aucun processus en arriÃ¨re-plan pour le contrÃ´le d'accÃ¨s

**VÃ©rification LOI-5** : *"Le contrÃ´le d'accÃ¨s fonctionne-t-il sur un Raspberry Pi 4 ?"* â†’ **Oui.**

### SynthÃ¨se de conformitÃ©

| Loi | Statut | Raison |
|-----|--------|--------|
| LOI-1 | âœ… Conforme | ContrÃ´le d'accÃ¨s local, aucune dÃ©pendance externe |
| LOI-5 | âœ… Conforme | Validations lÃ©gÃ¨res, empreinte minimale |

---

## 14. Conclusion contractuelle

Ce contrat Ã©tablit de maniÃ¨re dÃ©finitive et non nÃ©gociable les rÃ¨gles de contrÃ´le d'accÃ¨s Ã  Master Butler.

**Points clÃ©s :**

- **CatÃ©gories d'appelants** : CORE, SERVICE, OPERATOR, ADMIN, BOOTSTRAP
- **Niveaux d'autoritÃ©** : 0 (lecture publique) Ã  5 (administration)
- **Isolation** : Les OpÃ©rateurs n'accÃ¨dent qu'Ã  leurs propres capacitÃ©s/permissions
- **Adaptation** : Le contrÃ´le s'adapte aux niveaux de sÃ©curitÃ© (0-4)
- **SÃ©paration** : Master Butler contrÃ´le l'accÃ¨s Ã  son registre, pas aux ressources mÃ©tier

**Phrase fondatrice :**

> **Master Butler contrÃ´le l'accÃ¨s Ã  son registre de capacitÃ©s et permissions, sans jamais dÃ©cider de l'autorisation finale d'une action mÃ©tier â€” cette responsabilitÃ© appartient Ã  StrongFather.**

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisÃ©e.

---

**Document crÃ©Ã© le :** 2026-01-27  
**Version :** 1.0  
**Statut :** FONDATION â€” Contrat normatif validÃ©  
**RÃ©fÃ©rence :** Miyukini Core System v2.4, Master Butler Documentation, [Miyukini Conceptual References - Security Protocols](..//..//..//..//miyukini-webway-system//reference//_index.md), [Miyukini Conceptual References - Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md)  
**Type :** Contrat de contrÃ´le d'accÃ¨s non nÃ©gociable

---

## 15. Mini log â€” erreurs / warnings / ambiguÃ¯tÃ©s rencontrÃ©es et corrigÃ©es

### AmbiguÃ¯tÃ© A1 : Distinction entre contrÃ´le d'accÃ¨s Ã  Master Butler et vÃ©rification des permissions mÃ©tier

**AmbiguÃ¯tÃ© rencontrÃ©e :** Risque de confusion entre le contrÃ´le d'accÃ¨s aux APIs de Master Butler (ce contrat) et la vÃ©rification des permissions pour les actions mÃ©tier (StrongFather).

**DÃ©cision prise :** Clarification explicite dÃ¨s l'introduction et rappels constants que Master Butler contrÃ´le l'accÃ¨s Ã  son registre, pas aux ressources mÃ©tier.

**Correction effectuÃ©e :** Section 2 rÃ©digÃ©e avec principe de sÃ©paration dÃ©finition/dÃ©cision, et invariant INV-ACC-8 ajoutÃ©.

### AmbiguÃ¯tÃ© A2 : AccÃ¨s des Cores aux permissions SYSTEM

**AmbiguÃ¯tÃ© rencontrÃ©e :** Les Cores (StrongFather notamment) doivent-ils avoir accÃ¨s aux permissions SYSTEM ?

**DÃ©cision prise :** StrongFather peut LIRE les permissions SYSTEM (niveau 3) pour ses dÃ©cisions, mais ne peut pas les MODIFIER (rÃ©servÃ© Ã  ADMIN niveau 5).

**Correction effectuÃ©e :** Matrice d'accÃ¨s prÃ©cisÃ©e dans la section 4.3 et schÃ©ma 12.2.

### AmbiguÃ¯tÃ© A3 : Scope des OpÃ©rateurs

**AmbiguÃ¯tÃ© rencontrÃ©e :** Comment dÃ©finir prÃ©cisÃ©ment le scope d'un OpÃ©rateur ?

**DÃ©cision prise :** Un OpÃ©rateur n'accÃ¨de qu'aux capacitÃ©s/permissions de son propre module (identifiÃ© par module_id) plus les capacitÃ©s/permissions publiques.

**Correction effectuÃ©e :** Section 7 rÃ©digÃ©e avec rÃ¨gles de scope explicites.

### VÃ©rification de compatibilitÃ©

**VÃ©rification effectuÃ©e :**
- âœ… CohÃ©rence avec Authority Limits Contract : ConfirmÃ©e (Master Butler ne dÃ©cide pas)
- âœ… CohÃ©rence avec Permission API Contract : ConfirmÃ©e (niveaux d'autoritÃ© cohÃ©rents)
- âœ… CohÃ©rence avec Security Protocols : ConfirmÃ©e (RT-SEC-2, RT-SEC-3, AS-SEC-3)
- âœ… CohÃ©rence avec Security Levels : ConfirmÃ©e (adaptation niveaux 0-4)

**Conclusion :** Aucune contradiction dÃ©tectÃ©e avec les contrats existants.

---

*Aucune autre erreur, warning, ou ambiguÃ¯tÃ© rencontrÃ©e lors de la rÃ©daction de ce document.*

