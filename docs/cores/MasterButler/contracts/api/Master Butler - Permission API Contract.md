# Master Butler â€” Permission API Contract

## 1. Introduction

### Objet du contrat

Ce document dÃ©finit le **Master Butler Permission API Contract** : un contrat normatif, non nÃ©gociable, et de statut FONDATION qui Ã©tablit la surface d'appel conceptuelle pour la dÃ©finition, l'interrogation, l'association et la gestion des permissions dans le systÃ¨me Miyukini Core System v2.4.

Ce contrat prÃ©cise les opÃ©rations autorisÃ©es sur les permissions, les rÃ¨gles d'appel, les contextes requis, les garanties offertes, et les interdictions absolues liÃ©es Ã  la gestion des permissions.

### PortÃ©e

Ce contrat s'applique Ã  **tous les composants** interagissant avec le registre des permissions de Master Butler et dÃ©finit de maniÃ¨re absolue :

- La dÃ©finition formelle de la Permission API et son rÃ´le systÃ©mique
- La typologie conceptuelle des opÃ©rations autorisÃ©es
- Les rÃ¨gles d'appel et prÃ©conditions obligatoires
- Ce que la Permission API PEUT et NE PEUT JAMAIS faire
- Les garanties offertes aux appelants conformes
- Les rÃ¨gles de rejet et comportements en cas d'erreur
- Les invariants systÃ©miques associÃ©s

Ce contrat se concentre exclusivement sur la **surface d'appel** pour la gestion des permissions, sans entrer dans les dÃ©tails d'implÃ©mentation technique.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il Ã©tablit des rÃ¨gles absolues qui ne peuvent Ãªtre contournÃ©es, nÃ©gociÃ©es, ou modifiÃ©es. Le contrat prime sur toute considÃ©ration pratique.

### Relation avec les autres contrats

Ce contrat complÃ¨te et respecte les documents contractuels existants :

- **Master Butler â€” Documentation Fondatrice** : DÃ©finit la raison d'Ãªtre et les responsabilitÃ©s de Master Butler
- **Master Butler â€” Permission Registry Contract** : DÃ©finit le modÃ¨le de donnÃ©es du registre des permissions (complÃ©mentaire)
- **Master Butler â€” Capability API Contract** : DÃ©finit la surface d'appel pour les capacitÃ©s (parallÃ¨le)
- **Master Butler â€” Association Model Contract** : DÃ©finit les associations entre permissions, rÃ´les et capacitÃ©s
- **[Miyukini Conceptual References â€” Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)** : DÃ©finitions canoniques des termes
- **[Miyukini Conceptual References â€” Lois Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md)** : Ce contrat respecte **LOI-1** (aucune dÃ©pendance externe critique) en garantissant que toutes les opÃ©rations fonctionnent sans appel externe obligatoire

**ComplÃ©mentaritÃ© :**

- Permission Registry Contract = le **modÃ¨le de donnÃ©es** des permissions
- Permission API Contract = la **surface d'appel** pour interagir avec les permissions

---

## 2. RÃ´le et nature de la Permission API

### DÃ©finition formelle

La **Permission API** est la surface d'appel conceptuelle qui constitue l'interface formelle pour toutes les opÃ©rations liÃ©es aux permissions dans Master Butler. Elle reprÃ©sente l'ensemble des opÃ©rations conceptuelles exposÃ©es pour dÃ©finir, interroger, modifier et gÃ©rer les permissions du systÃ¨me.

### CaractÃ©ristiques formelles fondamentales

**Surface d'appel dÃ©diÃ©e :** La Permission API est la surface d'appel unique pour toutes les opÃ©rations sur les permissions. Aucune autre mÃ©thode d'interaction avec les permissions n'est autorisÃ©e.

**Interface conceptuelle :** La Permission API est une interface conceptuelle, pas une implÃ©mentation technique. Elle dÃ©finit les opÃ©rations autorisÃ©es de maniÃ¨re abstraite, sans prÃ©supposer aucune technologie, aucun protocole, ou aucun format de donnÃ©es.

**MÃ©diation obligatoire :** Toute opÃ©ration sur les permissions DOIT passer par la Permission API. Aucun accÃ¨s direct au registre des permissions n'est autorisÃ©.

**Abstraction de l'implÃ©mentation :** La Permission API abstrait complÃ¨tement l'implÃ©mentation interne du registre. Les appelants interagissent avec des concepts, pas avec des mÃ©canismes techniques.

### Nature systÃ©mique

La Permission API est un **concept systÃ©mique**, pas une interface technique. Elle reprÃ©sente la frontiÃ¨re conceptuelle entre les appelants (OpÃ©rateurs, StrongFather, BondingBrother) et le registre des permissions de Master Butler.

**Important :** Cette dÃ©finition est purement conceptuelle et systÃ©mique. Elle ne prÃ©suppose aucune technologie, aucun langage de programmation, aucun protocole de communication, ou aucun format d'Ã©change.

---

## 3. Principes fondamentaux

### Principe d'unicitÃ©

La Permission API constitue l'**unique surface d'appel** pour les opÃ©rations sur les permissions.

| CaractÃ©ristique | Description |
|-----------------|-------------|
| **UnicitÃ©** | Il n'existe qu'une seule Permission API |
| **ExclusivitÃ©** | Toute opÃ©ration sur les permissions DOIT passer par cette API |
| **Non-contournabilitÃ©** | La Permission API ne peut pas Ãªtre contournÃ©e |
| **Centralisation** | Tout contrÃ´le et validation sont centralisÃ©s |

### Principe de sÃ©paration

La Permission API respecte la sÃ©paration fondamentale entre :

| ResponsabilitÃ© | PropriÃ©taire | Ce que fait la Permission API |
|----------------|--------------|-------------------------------|
| **DÃ©finition des permissions** | Master Butler | âœ… Permet de dÃ©finir |
| **Attribution des permissions** | MÃ©canismes d'attribution | âŒ Ne gÃ¨re pas |
| **VÃ©rification des permissions** | StrongFather | âŒ Ne vÃ©rifie jamais |

**RÃ¨gle absolue :**

> **La Permission API dÃ©finit ce qui existe comme droits possibles, jamais ce qui est effectivement autorisÃ©.**

### Principe de non-dÃ©cision

La Permission API **ne prend jamais de dÃ©cision d'autorisation**. Elle fournit les informations sur les permissions dÃ©finies, mais ne rÃ©pond jamais "autorisÃ©" ou "refusÃ©" pour une action.

---

## 4. DÃ©finition conceptuelle d'une opÃ©ration Permission API

### DÃ©finition formelle

Une **opÃ©ration Permission API** est une demande d'action conceptuelle formulÃ©e par un appelant Ã  destination du registre des permissions de Master Butler, accompagnÃ©e d'un contexte, et soumise Ã  validation avant exÃ©cution.

### CaractÃ©ristiques formelles d'une opÃ©ration

**Demande d'action :** Une opÃ©ration Permission API est une demande d'action sur les permissions (dÃ©finition, interrogation, modification, dÃ©prÃ©ciation).

**Contexte requis :** Chaque opÃ©ration Permission API est accompagnÃ©e d'un contexte qui inclut :
- L'identitÃ© de l'appelant
- Le niveau d'autoritÃ© de l'appelant
- Le type d'opÃ©ration demandÃ©e
- Les paramÃ¨tres de l'opÃ©ration

**Soumission Ã  validation :** Chaque opÃ©ration Permission API est soumise Ã  validation avant exÃ©cution.

**AtomicitÃ© conceptuelle :** Une opÃ©ration Permission API est atomique conceptuellement. Elle est exÃ©cutÃ©e complÃ¨tement ou pas du tout.

**TraÃ§abilitÃ© obligatoire :** Chaque opÃ©ration Permission API est tracÃ©e de maniÃ¨re complÃ¨te.

### Structure conceptuelle d'une opÃ©ration

Conceptuellement, une opÃ©ration Permission API comprend :
- **Type d'opÃ©ration :** la catÃ©gorie de l'opÃ©ration
- **ParamÃ¨tres :** les donnÃ©es nÃ©cessaires Ã  l'exÃ©cution
- **Contexte :** les informations contextuelles requises
- **RÃ©sultat attendu :** le type de rÃ©sultat retournÃ©

---

## 5. Typologie des opÃ©rations autorisÃ©es

### 5.1. OpÃ©rations de dÃ©finition

#### CrÃ©er une permission

**OpÃ©ration :** `definePermission`

**Description :** CrÃ©e une nouvelle permission dans le registre.

**ParamÃ¨tres requis :**

| ParamÃ¨tre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `id` | String | Identifiant unique de la permission | âœ… |
| `name` | String | Nom lisible de la permission | âœ… |
| `description` | String | Description dÃ©taillÃ©e | âœ… |
| `domain` | String | Domaine fonctionnel | âœ… |
| `level` | Enum | Niveau de criticitÃ© | âœ… |
| `scope_type` | Enum | Type de portÃ©e | âœ… |
| `capabilities` | Array[String] | CapacitÃ©s couvertes | âœ… |
| `implied_permissions` | Array[String] | Permissions impliquÃ©es | âŒ |
| `required_permissions` | Array[String] | Permissions prÃ©requises | âŒ |

**PrÃ©conditions :**
- L'appelant doit avoir l'autoritÃ© de dÃ©finir des permissions
- L'identifiant ne doit pas dÃ©jÃ  exister
- Toutes les capacitÃ©s rÃ©fÃ©rencÃ©es doivent exister
- Toutes les permissions impliquÃ©es/requises doivent exister
- Le niveau de criticitÃ© doit Ãªtre autorisÃ© pour l'appelant

**RÃ©sultat :**
- SuccÃ¨s : Permission crÃ©Ã©e en Ã©tat DRAFT, identifiant confirmÃ©
- Ã‰chec : Erreur explicite avec raison

#### Activer une permission

**OpÃ©ration :** `activatePermission`

**Description :** Active une permission en Ã©tat DRAFT.

**ParamÃ¨tres requis :**

| ParamÃ¨tre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Identifiant de la permission | âœ… |

**PrÃ©conditions :**
- La permission doit exister
- La permission doit Ãªtre en Ã©tat DRAFT
- L'appelant doit avoir l'autoritÃ© d'activer
- Toutes les capacitÃ©s rÃ©fÃ©rencÃ©es doivent Ãªtre ACTIVE
- Toutes les permissions impliquÃ©es doivent Ãªtre ACTIVE

**RÃ©sultat :**
- SuccÃ¨s : Permission passÃ©e en Ã©tat ACTIVE
- Ã‰chec : Erreur explicite avec raison

#### Modifier une permission

**OpÃ©ration :** `updatePermission`

**Description :** Modifie une permission existante.

**ParamÃ¨tres requis :**

| ParamÃ¨tre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Identifiant de la permission | âœ… |
| `updates` | Object | Champs Ã  modifier | âœ… |

**RÃ¨gles de modification :**

| Champ | Modifiable en DRAFT | Modifiable en ACTIVE |
|-------|---------------------|----------------------|
| `id` | âŒ Non | âŒ Non |
| `name` | âœ… Oui | âš ï¸ Avec version |
| `description` | âœ… Oui | âš ï¸ Avec version |
| `capabilities` | âœ… Oui | âš ï¸ Avec version |
| `implied_permissions` | âœ… Oui | âš ï¸ Avec version |
| `level` | âœ… Oui | âŒ Non |
| `scope_type` | âœ… Oui | âŒ Non |

**RÃ©sultat :**
- SuccÃ¨s : Permission mise Ã  jour, nouvelle version si applicable
- Ã‰chec : Erreur explicite avec raison

#### DÃ©prÃ©cier une permission

**OpÃ©ration :** `deprecatePermission`

**Description :** Marque une permission comme dÃ©prÃ©ciÃ©e.

**ParamÃ¨tres requis :**

| ParamÃ¨tre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Identifiant de la permission | âœ… |
| `reason` | String | Raison de la dÃ©prÃ©ciation | âœ… |
| `successor_id` | String | Permission de remplacement | RecommandÃ© |
| `migration_guide` | String | Guide de migration | RecommandÃ© |

**PrÃ©conditions :**
- La permission doit exister
- La permission doit Ãªtre en Ã©tat ACTIVE
- L'appelant doit avoir l'autoritÃ© de dÃ©prÃ©cier

**RÃ©sultat :**
- SuccÃ¨s : Permission passÃ©e en Ã©tat DEPRECATED
- Ã‰chec : Erreur explicite avec raison

#### Retirer une permission

**OpÃ©ration :** `retirePermission`

**Description :** Retire dÃ©finitivement une permission du systÃ¨me.

**ParamÃ¨tres requis :**

| ParamÃ¨tre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Identifiant de la permission | âœ… |

**PrÃ©conditions :**
- La permission doit exister
- La permission doit Ãªtre en Ã©tat DEPRECATED
- La pÃ©riode de dÃ©prÃ©ciation minimale doit Ãªtre Ã©coulÃ©e
- L'appelant doit avoir l'autoritÃ© de retirer

**RÃ©sultat :**
- SuccÃ¨s : Permission passÃ©e en Ã©tat RETIRED, archivÃ©e
- Ã‰chec : Erreur explicite avec raison

---

### 5.2. OpÃ©rations d'interrogation

#### Obtenir une permission

**OpÃ©ration :** `getPermission`

**Description :** RÃ©cupÃ¨re la dÃ©finition complÃ¨te d'une permission.

**ParamÃ¨tres requis :**

| ParamÃ¨tre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Identifiant de la permission | âœ… |

**RÃ©sultat :**
- SuccÃ¨s : DÃ©finition complÃ¨te de la permission
- Ã‰chec : Erreur si permission inexistante

#### Lister les permissions

**OpÃ©ration :** `listPermissions`

**Description :** Liste les permissions selon des critÃ¨res de filtrage.

**ParamÃ¨tres optionnels :**

| ParamÃ¨tre | Type | Description |
|-----------|------|-------------|
| `domain` | String | Filtrer par domaine |
| `level` | Enum[] | Filtrer par niveaux |
| `status` | Enum[] | Filtrer par Ã©tats |
| `scope_type` | Enum[] | Filtrer par types de portÃ©e |
| `capability_id` | String | Filtrer par capacitÃ© couverte |
| `offset` | Integer | DÃ©calage pour pagination |
| `limit` | Integer | Nombre maximum de rÃ©sultats |

**RÃ©sultat :**
- Liste des permissions correspondant aux critÃ¨res
- MÃ©tadonnÃ©es de pagination

#### Rechercher des permissions

**OpÃ©ration :** `searchPermissions`

**Description :** Recherche des permissions par texte libre.

**ParamÃ¨tres requis :**

| ParamÃ¨tre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `query` | String | Texte de recherche | âœ… |
| `filters` | Object | Filtres additionnels | âŒ |

**RÃ©sultat :**
- Liste des permissions correspondant Ã  la recherche
- Score de pertinence pour chaque rÃ©sultat

#### Obtenir les capacitÃ©s d'une permission

**OpÃ©ration :** `getPermissionCapabilities`

**Description :** RÃ©cupÃ¨re les capacitÃ©s couvertes par une permission.

**ParamÃ¨tres requis :**

| ParamÃ¨tre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Identifiant de la permission | âœ… |
| `include_implied` | Boolean | Inclure les capacitÃ©s des permissions impliquÃ©es | âŒ (dÃ©faut: false) |

**RÃ©sultat :**
- Liste des capacitÃ©s directement associÃ©es
- Si `include_implied` : union de toutes les capacitÃ©s effectives

#### Obtenir la hiÃ©rarchie d'une permission

**OpÃ©ration :** `getPermissionHierarchy`

**Description :** RÃ©cupÃ¨re l'arbre des implications d'une permission.

**ParamÃ¨tres requis :**

| ParamÃ¨tre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Identifiant de la permission | âœ… |
| `direction` | Enum | `UP` (qui implique cette permission) ou `DOWN` (permissions impliquÃ©es) | âŒ (dÃ©faut: DOWN) |

**RÃ©sultat :**
- Arbre des implications dans la direction demandÃ©e
- Profondeur de chaque niveau

---

### 5.3. OpÃ©rations d'association

#### Associer une capacitÃ©

**OpÃ©ration :** `associateCapability`

**Description :** Ajoute une capacitÃ© Ã  une permission.

**ParamÃ¨tres requis :**

| ParamÃ¨tre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Identifiant de la permission | âœ… |
| `capability_id` | String | Identifiant de la capacitÃ© | âœ… |

**PrÃ©conditions :**
- La permission doit exister
- La capacitÃ© doit exister
- La permission doit Ãªtre en Ã©tat DRAFT ou ACTIVE
- L'appelant doit avoir l'autoritÃ© de modifier

**RÃ©sultat :**
- SuccÃ¨s : Association crÃ©Ã©e
- Ã‰chec : Erreur explicite avec raison

#### Dissocier une capacitÃ©

**OpÃ©ration :** `dissociateCapability`

**Description :** Retire une capacitÃ© d'une permission.

**ParamÃ¨tres requis :**

| ParamÃ¨tre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Identifiant de la permission | âœ… |
| `capability_id` | String | Identifiant de la capacitÃ© | âœ… |

**PrÃ©conditions :**
- La permission doit exister
- L'association doit exister
- La permission doit conserver au moins une capacitÃ©
- La permission doit Ãªtre en Ã©tat DRAFT ou ACTIVE

**RÃ©sultat :**
- SuccÃ¨s : Association retirÃ©e
- Ã‰chec : Erreur explicite avec raison

#### Ajouter une implication

**OpÃ©ration :** `addImplication`

**Description :** Ajoute une permission impliquÃ©e.

**ParamÃ¨tres requis :**

| ParamÃ¨tre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Permission parente | âœ… |
| `implied_permission_id` | String | Permission impliquÃ©e | âœ… |

**PrÃ©conditions :**
- Les deux permissions doivent exister
- L'ajout ne doit pas crÃ©er de cycle
- La permission parente doit Ãªtre en Ã©tat DRAFT ou ACTIVE

**RÃ©sultat :**
- SuccÃ¨s : Implication ajoutÃ©e
- Ã‰chec : Erreur explicite (notamment si cycle dÃ©tectÃ©)

#### Retirer une implication

**OpÃ©ration :** `removeImplication`

**Description :** Retire une permission impliquÃ©e.

**ParamÃ¨tres requis :**

| ParamÃ¨tre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Permission parente | âœ… |
| `implied_permission_id` | String | Permission impliquÃ©e Ã  retirer | âœ… |

**PrÃ©conditions :**
- Les deux permissions doivent exister
- L'implication doit exister
- La permission parente doit Ãªtre en Ã©tat DRAFT ou ACTIVE

**RÃ©sultat :**
- SuccÃ¨s : Implication retirÃ©e
- Ã‰chec : Erreur explicite avec raison

---

### 5.4. OpÃ©rations de validation

#### Valider une dÃ©finition

**OpÃ©ration :** `validatePermissionDefinition`

**Description :** Valide une dÃ©finition de permission sans l'enregistrer.

**ParamÃ¨tres requis :**

| ParamÃ¨tre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `definition` | Object | DÃ©finition Ã  valider | âœ… |

**RÃ©sultat :**
- SuccÃ¨s : DÃ©finition valide
- Ã‰chec : Liste des erreurs de validation

#### VÃ©rifier les cycles

**OpÃ©ration :** `checkCycles`

**Description :** VÃ©rifie si l'ajout d'une implication crÃ©erait un cycle.

**ParamÃ¨tres requis :**

| ParamÃ¨tre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Permission parente | âœ… |
| `implied_permission_id` | String | Permission Ã  impliquer | âœ… |

**RÃ©sultat :**
- SuccÃ¨s : Pas de cycle dÃ©tectÃ©
- Ã‰chec : Cycle dÃ©tectÃ© avec chemin explicite

---

## 6. Ce que la Permission API PEUT faire

### 6.1. OpÃ©rations autorisÃ©es

**PEUT-PERM-1 : DÃ©finir des permissions**

La Permission API PEUT crÃ©er de nouvelles permissions dans le registre, sous rÃ©serve que le contexte soit valide et que l'appelant ait l'autoritÃ© requise.

**PEUT-PERM-2 : Interroger les permissions**

La Permission API PEUT retourner les dÃ©finitions des permissions, les lister, les rechercher, et fournir leurs mÃ©tadonnÃ©es.

**PEUT-PERM-3 : Modifier les permissions**

La Permission API PEUT modifier les permissions selon les rÃ¨gles de modification dÃ©finies dans le Permission Registry Contract.

**PEUT-PERM-4 : GÃ©rer le cycle de vie**

La Permission API PEUT faire transiter les permissions entre les Ã©tats du cycle de vie (DRAFT â†’ ACTIVE â†’ DEPRECATED â†’ RETIRED).

**PEUT-PERM-5 : GÃ©rer les associations**

La Permission API PEUT crÃ©er et supprimer les associations entre permissions et capacitÃ©s, et entre permissions (implications).

**PEUT-PERM-6 : Valider les dÃ©finitions**

La Permission API PEUT valider les dÃ©finitions de permissions avant enregistrement et dÃ©tecter les cycles d'implication.

**PEUT-PERM-7 : Retourner des erreurs explicites**

La Permission API PEUT retourner des erreurs explicites et actionnables lorsqu'une opÃ©ration ne peut pas Ãªtre exÃ©cutÃ©e.

### 6.2. Garanties associÃ©es

Chaque opÃ©ration autorisÃ©e est accompagnÃ©e des garanties suivantes :
- Validation complÃ¨te avant exÃ©cution
- AtomicitÃ© de l'opÃ©ration
- TraÃ§abilitÃ© complÃ¨te
- Erreur explicite en cas de rejet
- IntÃ©gritÃ© du registre prÃ©servÃ©e

---

## 7. Ce que la Permission API NE PEUT JAMAIS faire

### 7.1. Interdictions absolues

**INTERDIT-PERM-1 : DÃ©cider d'une autorisation**

La Permission API NE PEUT JAMAIS dÃ©cider si une permission est accordÃ©e ou refusÃ©e Ã  un contexte donnÃ©. Elle dÃ©finit les permissions, elle ne vÃ©rifie pas leur attribution.

**INTERDIT-PERM-2 : VÃ©rifier les permissions en temps rÃ©el**

La Permission API NE PEUT JAMAIS vÃ©rifier si un utilisateur ou un contexte possÃ¨de effectivement une permission au moment d'une action. Cette vÃ©rification appartient Ã  StrongFather.

**INTERDIT-PERM-3 : Retourner un verdict d'autorisation**

La Permission API NE PEUT JAMAIS retourner "autorisÃ©" ou "refusÃ©" comme rÃ©sultat d'une opÃ©ration. Elle retourne des dÃ©finitions, pas des dÃ©cisions.

**INTERDIT-PERM-4 : CrÃ©er une permission sans capacitÃ©**

La Permission API NE PEUT JAMAIS crÃ©er ou activer une permission qui ne rÃ©fÃ©rence aucune capacitÃ© existante.

**INTERDIT-PERM-5 : CrÃ©er des cycles d'implication**

La Permission API NE PEUT JAMAIS crÃ©er une implication qui formerait un cycle (direct ou indirect).

**INTERDIT-PERM-6 : Contourner les Ã©tats du cycle de vie**

La Permission API NE PEUT JAMAIS permettre une transition d'Ã©tat non autorisÃ©e (ex: ACTIVE â†’ DRAFT, RETIRED â†’ ACTIVE).

**INTERDIT-PERM-7 : Modifier l'identifiant**

La Permission API NE PEUT JAMAIS modifier l'identifiant d'une permission aprÃ¨s sa crÃ©ation.

**INTERDIT-PERM-8 : Supprimer sans dÃ©prÃ©ciation**

La Permission API NE PEUT JAMAIS retirer une permission ACTIVE sans passer par l'Ã©tat DEPRECATED (sauf pour les permissions DRAFT).

**INTERDIT-PERM-9 : Exposer les attributions**

La Permission API NE PEUT JAMAIS exposer qui possÃ¨de quelle permission. Ces informations appartiennent aux mÃ©canismes d'attribution et Ã  StrongFather.

**INTERDIT-PERM-10 : Appliquer des rÃ¨gles mÃ©tier**

La Permission API NE PEUT JAMAIS appliquer des rÃ¨gles mÃ©tier sur l'usage des permissions. Elle dÃ©finit les droits, pas leur contexte d'application.

### 7.2. Justifications

Ces interdictions sont justifiÃ©es par :
- La prÃ©servation de la sÃ©paration entre dÃ©finition et dÃ©cision
- Le respect de l'autoritÃ© de StrongFather pour les dÃ©cisions
- La garantie de l'intÃ©gritÃ© du registre
- L'absence de logique mÃ©tier dans Master Butler
- Le principe de non-vÃ©rification de Master Butler

---

## 8. RÃ¨gles absolues d'appel (prÃ©conditions)

### 8.1. PrÃ©conditions obligatoires

Chaque appel Permission API DOIT respecter les prÃ©conditions suivantes. Si une prÃ©condition n'est pas satisfaite, l'appel est rejetÃ© immÃ©diatement.

**PRECOND-PERM-1 : IdentitÃ© de l'appelant**

Chaque appel DOIT Ãªtre accompagnÃ© de l'identitÃ© de l'appelant, permettant de vÃ©rifier son autoritÃ©.

**PRECOND-PERM-2 : AutoritÃ© suffisante**

L'appelant DOIT avoir l'autoritÃ© nÃ©cessaire pour l'opÃ©ration demandÃ©e :
- DÃ©finition de permission STANDARD : OpÃ©rateurs autorisÃ©s
- DÃ©finition de permission ELEVATED : OpÃ©rateurs avec autoritÃ© Ã©levÃ©e
- DÃ©finition de permission CRITICAL : StrongFather avec validation
- DÃ©finition de permission SYSTEM : MiyukiniAdmin uniquement

**PRECOND-PERM-3 : ParamÃ¨tres valides**

Tous les paramÃ¨tres obligatoires DOIVENT Ãªtre fournis et valides.

**PRECOND-PERM-4 : RÃ©fÃ©rences existantes**

Toutes les rÃ©fÃ©rences (capacitÃ©s, permissions impliquÃ©es) DOIVENT exister dans les registres respectifs.

**PRECOND-PERM-5 : CohÃ©rence des Ã©tats**

Les opÃ©rations DOIVENT Ãªtre cohÃ©rentes avec l'Ã©tat actuel des permissions concernÃ©es.

### 8.2. RÃ¨gles de validation

- Les prÃ©conditions sont validÃ©es dans l'ordre
- Si une prÃ©condition Ã©choue, l'appel est rejetÃ© immÃ©diatement
- L'erreur de rejet indique la prÃ©condition non satisfaite
- Aucune exÃ©cution partielle n'est autorisÃ©e aprÃ¨s un Ã©chec

---

## 9. RÃ¨gles absolues de rejet

### 9.1. Conditions de rejet

Un appel Permission API est rejetÃ© si l'une des conditions suivantes est dÃ©tectÃ©e :

**REJET-PERM-1 : Appelant non identifiÃ©**

L'appel est rejetÃ© si l'identitÃ© de l'appelant n'est pas fournie ou invalide.
- Erreur : `UNKNOWN_CALLER`
- Action : Aucune modification

**REJET-PERM-2 : AutoritÃ© insuffisante**

L'appel est rejetÃ© si l'appelant n'a pas l'autoritÃ© pour l'opÃ©ration.
- Erreur : `INSUFFICIENT_AUTHORITY`
- Action : Aucune modification, tentative tracÃ©e

**REJET-PERM-3 : Permission inexistante**

L'appel est rejetÃ© si la permission rÃ©fÃ©rencÃ©e n'existe pas.
- Erreur : `PERMISSION_NOT_FOUND`
- Action : Aucune modification

**REJET-PERM-4 : Identifiant dupliquÃ©**

L'appel est rejetÃ© si l'identifiant existe dÃ©jÃ  lors d'une crÃ©ation.
- Erreur : `DUPLICATE_PERMISSION_ID`
- Action : Aucune modification

**REJET-PERM-5 : CapacitÃ© inexistante**

L'appel est rejetÃ© si une capacitÃ© rÃ©fÃ©rencÃ©e n'existe pas.
- Erreur : `CAPABILITY_NOT_FOUND`
- Action : Aucune modification

**REJET-PERM-6 : Cycle dÃ©tectÃ©**

L'appel est rejetÃ© si l'opÃ©ration crÃ©erait un cycle d'implication.
- Erreur : `CYCLIC_IMPLICATION_DETECTED`
- Action : Aucune modification, chemin du cycle retournÃ©

**REJET-PERM-7 : Transition d'Ã©tat invalide**

L'appel est rejetÃ© si la transition d'Ã©tat demandÃ©e n'est pas autorisÃ©e.
- Erreur : `INVALID_STATE_TRANSITION`
- Action : Aucune modification

**REJET-PERM-8 : Modification interdite**

L'appel est rejetÃ© si la modification demandÃ©e n'est pas autorisÃ©e pour l'Ã©tat actuel.
- Erreur : `MODIFICATION_NOT_ALLOWED`
- Action : Aucune modification

**REJET-PERM-9 : DerniÃ¨re capacitÃ©**

L'appel est rejetÃ© si la dissociation laisserait la permission sans capacitÃ©.
- Erreur : `LAST_CAPABILITY_REMOVAL`
- Action : Aucune modification

**REJET-PERM-10 : PÃ©riode de dÃ©prÃ©ciation**

L'appel est rejetÃ© si le retrait est demandÃ© avant la fin de la pÃ©riode de dÃ©prÃ©ciation.
- Erreur : `DEPRECATION_PERIOD_NOT_ELAPSED`
- Action : Aucune modification

### 9.2. Garanties aprÃ¨s rejet

AprÃ¨s tout rejet, les garanties suivantes s'appliquent :
- L'Ã©tat du registre reste inchangÃ©
- Aucune modification partielle n'est appliquÃ©e
- L'erreur est explicite et actionnable
- La tentative est tracÃ©e pour audit
- Aucun effet de bord n'est crÃ©Ã©

### 9.3. RÃ¨gles absolues

- **R-REJ-PERM-1 :** Tout rejet laisse le registre inchangÃ©
- **R-REJ-PERM-2 :** Tout rejet retourne une erreur explicite
- **R-REJ-PERM-3 :** Tout rejet est tracÃ©
- **R-REJ-PERM-4 :** Aucune exception au rejet n'est autorisÃ©e

---

## 10. Garanties offertes aux appelants conformes

### 10.1. Garanties de traitement

**G-PERM-API-1 : Traitement prÃ©visible**

Si un appelant autorisÃ© fournit des paramÃ¨tres valides et respecte les prÃ©conditions, Master Butler traite l'opÃ©ration de maniÃ¨re prÃ©visible et conforme au contrat.

**G-PERM-API-2 : Messages d'erreur explicites**

Si une opÃ©ration est rejetÃ©e, Master Butler retourne toujours un message d'erreur explicite et actionnable.

**G-PERM-API-3 : Pas de rejet arbitraire**

Master Butler ne rejette jamais une opÃ©ration de maniÃ¨re arbitraire. Tout rejet est justifiÃ© par une violation documentÃ©e.

**G-PERM-API-4 : AtomicitÃ©**

Toute opÃ©ration Permission API est atomique. Elle est exÃ©cutÃ©e complÃ¨tement ou pas du tout.

### 10.2. Garanties de cohÃ©rence

**G-PERM-API-5 : IntÃ©gritÃ© rÃ©fÃ©rentielle**

AprÃ¨s toute opÃ©ration rÃ©ussie, l'intÃ©gritÃ© rÃ©fÃ©rentielle du registre est garantie.

**G-PERM-API-6 : Ã‰tat inchangÃ© aprÃ¨s rejet**

AprÃ¨s tout rejet, l'Ã©tat du registre reste inchangÃ©.

**G-PERM-API-7 : Absence de cycle**

AprÃ¨s toute opÃ©ration rÃ©ussie, le registre ne contient aucun cycle d'implication.

### 10.3. Garanties de traÃ§abilitÃ©

**G-PERM-API-8 : TraÃ§abilitÃ© complÃ¨te**

Toutes les opÃ©rations sont tracÃ©es de maniÃ¨re complÃ¨te (qui, quand, quoi, rÃ©sultat).

**G-PERM-API-9 : Historique prÃ©servÃ©**

L'historique des modifications est prÃ©servÃ©, y compris pour les permissions retirÃ©es.

### 10.4. Non-nÃ©gociabilitÃ©

Ces garanties sont absolues et non nÃ©gociables. Elles s'appliquent Ã  tous les appelants conformes, sans exception.

---

## 11. Contexte requis pour les opÃ©rations

### 11.1. Structure du contexte

Chaque opÃ©ration Permission API est accompagnÃ©e d'un contexte structurÃ© :

```yaml
context:
  caller:
    id: <identifiant de l'appelant>
    type: <OPERATOR | CORE | SYSTEM>
    authority_level: <niveau d'autoritÃ©>
  operation:
    type: <type d'opÃ©ration>
    timestamp: <timestamp de l'appel>
    request_id: <identifiant unique de requÃªte>
  trace:
    correlation_id: <identifiant de corrÃ©lation>
    source: <composant source>
```

### 11.2. Contexte par type d'appelant

#### OpÃ©rateur

| Champ | Description | Requis |
|-------|-------------|--------|
| `operator_id` | Identifiant de l'OpÃ©rateur | âœ… |
| `authority_level` | Niveau d'autoritÃ© | âœ… |
| `session_id` | Identifiant de session | RecommandÃ© |

#### Core (StrongFather, BondingBrother)

| Champ | Description | Requis |
|-------|-------------|--------|
| `core_id` | Identifiant du Core | âœ… |
| `operation_context` | Contexte de l'opÃ©ration parente | âœ… |

#### System (MiyukiniAdmin)

| Champ | Description | Requis |
|-------|-------------|--------|
| `admin_id` | Identifiant administrateur | âœ… |
| `authorization_proof` | Preuve d'autorisation | âœ… |

---

## 12. Interaction avec les autres composants

### 12.1. Interaction avec StrongFather

**Flux typique d'interrogation :**

```
StrongFather Ã©value une intention
    â”‚
    â”œâ”€â”€ Interroge Permission API : "Quelles permissions couvrent cette capacitÃ© ?"
    â”‚       â”‚
    â”‚       â””â”€â”€ Permission API retourne : Liste des permissions
    â”‚
    â”œâ”€â”€ Interroge Permission API : "Quelle est la dÃ©finition de cette permission ?"
    â”‚       â”‚
    â”‚       â””â”€â”€ Permission API retourne : DÃ©finition complÃ¨te
    â”‚
    â””â”€â”€ StrongFather dÃ©cide selon les politiques
```

**RÃ¨gles d'interaction :**
- StrongFather est toujours autorisÃ© Ã  interroger
- La Permission API ne suggÃ¨re jamais de dÃ©cision
- Les rÃ©ponses sont exhaustives et exactes

### 12.2. Interaction avec BondingBrother

**Flux typique :**

```
BondingBrother traduit une intention
    â”‚
    â”œâ”€â”€ Interroge Permission API : "Quelles permissions sont requises pour cette action ?"
    â”‚       â”‚
    â”‚       â””â”€â”€ Permission API retourne : Permissions requises
    â”‚
    â””â”€â”€ BondingBrother enrichit le contexte de l'intention
```

**RÃ¨gles d'interaction :**
- BondingBrother interroge pour la traduction, pas pour la dÃ©cision
- Les rÃ©ponses aident Ã  construire le contexte

### 12.3. Interaction avec les OpÃ©rateurs

**Flux de dÃ©finition :**

```
OpÃ©rateur dÃ©finit une nouvelle permission
    â”‚
    â”œâ”€â”€ Soumet via BondingBrother
    â”‚       â”‚
    â”‚       â””â”€â”€ Permission API valide et enregistre
    â”‚
    â””â”€â”€ Confirmation de l'enregistrement
```

**Flux de dÃ©couverte :**

```
OpÃ©rateur dÃ©couvre les permissions
    â”‚
    â”œâ”€â”€ Interroge Permission API
    â”‚       â”‚
    â”‚       â””â”€â”€ Permission API retourne les permissions (selon autoritÃ©)
    â”‚
    â””â”€â”€ OpÃ©rateur utilise ces informations
```

### 12.4. Interaction avec le Capability Registry

**DÃ©pendance :**

```
Permission API
    â”‚
    â””â”€â”€ VÃ©rifie les capacitÃ©s rÃ©fÃ©rencÃ©es dans Capability Registry
            â”‚
            â””â”€â”€ Capability Registry confirme l'existence
```

**RÃ¨gles :**
- Toute capacitÃ© rÃ©fÃ©rencÃ©e DOIT exister
- La suppression d'une capacitÃ© invalide les permissions associÃ©es

---

## 13. Invariants systÃ©miques

### INV-PERM-API-1 : Non-dÃ©cision

La Permission API **ne prend jamais de dÃ©cision d'autorisation**. Aucune mÃ©thode ne retourne "autorisÃ©" ou "refusÃ©".

### INV-PERM-API-2 : AtomicitÃ©

Toute opÃ©ration Permission API est **atomique**. Elle est exÃ©cutÃ©e complÃ¨tement ou pas du tout.

### INV-PERM-API-3 : TraÃ§abilitÃ©

Toute opÃ©ration Permission API est **tracÃ©e** avec contexte complet.

### INV-PERM-API-4 : IntÃ©gritÃ© rÃ©fÃ©rentielle

La Permission API **prÃ©serve l'intÃ©gritÃ© rÃ©fÃ©rentielle** du registre. Aucune rÃ©fÃ©rence invalide n'est crÃ©Ã©e.

### INV-PERM-API-5 : Absence de cycle

La Permission API **garantit l'absence de cycle** dans les implications.

### INV-PERM-API-6 : Association obligatoire

La Permission API **garantit qu'une permission active a au moins une capacitÃ©**.

### INV-PERM-API-7 : ImmutabilitÃ© des identifiants

La Permission API **ne modifie jamais** un identifiant de permission aprÃ¨s crÃ©ation.

### INV-PERM-API-8 : Transitions d'Ã©tat valides

La Permission API **n'autorise que les transitions d'Ã©tat valides** du cycle de vie.

---

## 14. SchÃ©mas ASCII

### 14.1. Position de la Permission API dans l'architecture

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                    APPELANTS                                      â”‚
â”‚                                                                   â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”‚
â”‚  â”‚   OpÃ©rateurs  â”‚  â”‚  StrongFather â”‚  â”‚  BondingBrother   â”‚   â”‚
â”‚  â”‚               â”‚  â”‚   (dÃ©cision)  â”‚  â”‚    (mÃ©diation)    â”‚   â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                            â”‚
                            â”‚ Appels Permission API
                            â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                    PERMISSION API                                 â”‚
â”‚                                                                   â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚  OPÃ‰RATIONS AUTORISÃ‰ES :                                  â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  DÃ‰FINITION        INTERROGATION      ASSOCIATION         â”‚ â”‚
â”‚  â”‚  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€       â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€      â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€         â”‚ â”‚
â”‚  â”‚  â€¢ definePermission   â€¢ getPermission   â€¢ associateCapability  â”‚
â”‚  â”‚  â€¢ activatePermission â€¢ listPermissions â€¢ dissociateCapability â”‚
â”‚  â”‚  â€¢ updatePermission   â€¢ searchPermissions â€¢ addImplication     â”‚
â”‚  â”‚  â€¢ deprecatePermissionâ€¢ getCapabilities  â€¢ removeImplication   â”‚
â”‚  â”‚  â€¢ retirePermission   â€¢ getHierarchy                          â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  VALIDATION                                                â”‚ â”‚
â”‚  â”‚  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€                                                â”‚ â”‚
â”‚  â”‚  â€¢ validateDefinition â€¢ checkCycles                       â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚                                                                   â”‚
â”‚  PRINCIPES :                                                      â”‚
â”‚  âœ“ Surface d'appel unique pour les permissions                  â”‚
â”‚  âœ“ Contexte obligatoire                                         â”‚
â”‚  âœ“ Validation avant exÃ©cution                                   â”‚
â”‚  âœ“ AtomicitÃ© des opÃ©rations                                     â”‚
â”‚  âœ“ TraÃ§abilitÃ© complÃ¨te                                         â”‚
â”‚  âœ“ JAMAIS de dÃ©cision d'autorisation                            â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                            â”‚
                            â”‚ AccÃ¨de au
                            â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚              PERMISSION REGISTRY (Registre)                       â”‚
â”‚                                                                   â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚  â€¢ Stockage des permissions                               â”‚ â”‚
â”‚  â”‚  â€¢ Associations permission â†” capacitÃ©                    â”‚ â”‚
â”‚  â”‚  â€¢ HiÃ©rarchie d'implications                              â”‚ â”‚
â”‚  â”‚  â€¢ Historique des modifications                           â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 14.2. Flux de dÃ©finition d'une permission

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚              FLUX DE DÃ‰FINITION D'UNE PERMISSION                  â”‚
â”‚                                                                   â”‚
â”‚  APPELANT (OpÃ©rateur)                                            â”‚
â”‚      â”‚                                                            â”‚
â”‚      â”‚ 1. Soumet dÃ©finition de permission                        â”‚
â”‚      â”‚    â€¢ id, name, description                                â”‚
â”‚      â”‚    â€¢ domain, level, scope_type                            â”‚
â”‚      â”‚    â€¢ capabilities[]                                       â”‚
â”‚      â–¼                                                            â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚              PERMISSION API                               â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  2. Validation des prÃ©conditions                          â”‚ â”‚
â”‚  â”‚     â”œâ”€â”€ Appelant identifiÃ© ?         â”€â”€â†’ Rejet si non    â”‚ â”‚
â”‚  â”‚     â”œâ”€â”€ AutoritÃ© suffisante ?        â”€â”€â†’ Rejet si non    â”‚ â”‚
â”‚  â”‚     â””â”€â”€ ParamÃ¨tres valides ?         â”€â”€â†’ Rejet si non    â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚      â”‚                                                            â”‚
â”‚      â”‚ 3. Validation de la dÃ©finition                            â”‚
â”‚      â–¼                                                            â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚  Identifiant unique ?                â”€â”€â†’ Rejet si non     â”‚ â”‚
â”‚  â”‚  CapacitÃ©s existent toutes ?         â”€â”€â†’ Rejet si non     â”‚ â”‚
â”‚  â”‚  Permissions impliquÃ©es existent ?   â”€â”€â†’ Rejet si non     â”‚ â”‚
â”‚  â”‚  Pas de cycle d'implication ?        â”€â”€â†’ Rejet si cycle   â”‚ â”‚
â”‚  â”‚  Niveau autorisÃ© pour l'appelant ?   â”€â”€â†’ Rejet si non     â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚      â”‚                                                            â”‚
â”‚      â”‚ 4. Toutes validations passÃ©es                             â”‚
â”‚      â–¼                                                            â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚              ENREGISTREMENT                               â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  â€¢ Permission crÃ©Ã©e en Ã©tat DRAFT                         â”‚ â”‚
â”‚  â”‚  â€¢ Associations crÃ©Ã©es                                    â”‚ â”‚
â”‚  â”‚  â€¢ TraÃ§abilitÃ© enregistrÃ©e                               â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚      â”‚                                                            â”‚
â”‚      â”‚ 5. Retour du rÃ©sultat                                     â”‚
â”‚      â–¼                                                            â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚  SUCCÃˆS                              Ã‰CHEC                 â”‚ â”‚
â”‚  â”‚  â”€â”€â”€â”€â”€â”€â”€                             â”€â”€â”€â”€â”€                 â”‚ â”‚
â”‚  â”‚  â€¢ permission_id confirmÃ©            â€¢ Erreur explicite    â”‚ â”‚
â”‚  â”‚  â€¢ Ã©tat: DRAFT                       â€¢ Raison dÃ©taillÃ©e    â”‚ â”‚
â”‚  â”‚  â€¢ version: 1.0.0                    â€¢ Registre inchangÃ©   â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚      â”‚                                                            â”‚
â”‚      â–¼                                                            â”‚
â”‚  APPELANT (reÃ§oit le rÃ©sultat)                                   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 14.3. Ce que la Permission API fait vs ne fait pas

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚     PERMISSION API : CE QU'ELLE FAIT VS NE FAIT PAS              â”‚
â”‚                                                                   â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”‚
â”‚  â”‚  CE QUE LA PERMISSION API FAIT                           â”‚   â”‚
â”‚  â”‚  â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•                         â”‚   â”‚
â”‚  â”‚                                                           â”‚   â”‚
â”‚  â”‚  âœ“ DÃ©finit des permissions                               â”‚   â”‚
â”‚  â”‚  âœ“ Interroge le registre                                 â”‚   â”‚
â”‚  â”‚  âœ“ GÃ¨re les associations permission â†” capacitÃ©          â”‚   â”‚
â”‚  â”‚  âœ“ GÃ¨re les hiÃ©rarchies d'implication                   â”‚   â”‚
â”‚  â”‚  âœ“ Valide les dÃ©finitions                                â”‚   â”‚
â”‚  â”‚  âœ“ DÃ©tecte les cycles                                    â”‚   â”‚
â”‚  â”‚  âœ“ Trace toutes les opÃ©rations                           â”‚   â”‚
â”‚  â”‚  âœ“ Retourne des erreurs explicites                       â”‚   â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜   â”‚
â”‚                                                                   â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”‚
â”‚  â”‚  CE QUE LA PERMISSION API NE FAIT JAMAIS                 â”‚   â”‚
â”‚  â”‚  â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•                 â”‚   â”‚
â”‚  â”‚                                                           â”‚   â”‚
â”‚  â”‚  âœ— DÃ©cider si une permission est accordÃ©e                â”‚   â”‚
â”‚  â”‚  âœ— VÃ©rifier les permissions en temps rÃ©el                â”‚   â”‚
â”‚  â”‚  âœ— Retourner "autorisÃ©" ou "refusÃ©"                      â”‚   â”‚
â”‚  â”‚  âœ— ConnaÃ®tre qui possÃ¨de quelle permission               â”‚   â”‚
â”‚  â”‚  âœ— Appliquer des rÃ¨gles mÃ©tier                           â”‚   â”‚
â”‚  â”‚  âœ— ExÃ©cuter des actions fonctionnelles                   â”‚   â”‚
â”‚  â”‚  âœ— Stocker des donnÃ©es mÃ©tier                            â”‚   â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜   â”‚
â”‚                                                                   â”‚
â”‚  PHRASE FONDAMENTALE :                                            â”‚
â”‚  â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•                                            â”‚
â”‚                                                                   â”‚
â”‚  "La Permission API dÃ©finit ce qui existe comme droits           â”‚
â”‚   possibles, jamais ce qui est effectivement autorisÃ©."          â”‚
â”‚                                                                   â”‚
â”‚  La DÃ‰FINITION appartient Ã  Master Butler.                        â”‚
â”‚  La DÃ‰CISION appartient Ã  StrongFather.                           â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 15. ConformitÃ© aux Lois d'Autonomie SystÃ¨me

Ce contrat respecte les Lois d'Autonomie SystÃ¨me dÃ©finies dans [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md).

### LOI-1 : Aucune dÃ©pendance externe critique Ã  l'exÃ©cution

**ConformitÃ© :** Conforme

La Permission API opÃ¨re entiÃ¨rement en local :

- **OpÃ©rations locales** : Toutes les opÃ©rations s'exÃ©cutent localement
- **Registre local** : Le registre des permissions est local
- **Aucune API externe** : Aucun service distant n'est requis

**VÃ©rification LOI-1** : *"La Permission API fonctionne-t-elle si le rÃ©seau est indisponible ?"* â†’ **Oui.**

### LOI-5 : Le coÃ»t doit Ãªtre proportionnel au hardware

**ConformitÃ© :** Conforme

La Permission API a une empreinte minimale :

- **OpÃ©rations lÃ©gÃ¨res** : Lecture et Ã©criture de mÃ©tadonnÃ©es
- **Pas de workers** : Aucun processus en arriÃ¨re-plan
- **MÃ©moire prÃ©visible** : Proportionnelle au nombre de permissions

**VÃ©rification LOI-5** : *"La Permission API fonctionne-t-elle sur un Raspberry Pi 4 ?"* â†’ **Oui.**

### SynthÃ¨se de conformitÃ©

| Loi | Statut | Raison |
|-----|--------|--------|
| LOI-1 | âœ… Conforme | OpÃ©rations locales, aucune dÃ©pendance externe |
| LOI-5 | âœ… Conforme | MÃ©tadonnÃ©es lÃ©gÃ¨res, consommation minimale |

---

## 16. Conclusion contractuelle

### Essence de la Permission API

La Permission API de Master Butler est la **surface d'appel unique** pour toutes les opÃ©rations liÃ©es aux permissions dans le systÃ¨me Miyukini. Elle permet de dÃ©finir, interroger, associer et gÃ©rer les permissions, sans jamais participer Ã  la dÃ©cision d'autorisation.

### Phrase fondatrice

> **La Permission API dÃ©finit la surface d'appel pour gÃ©rer les droits possibles du systÃ¨me Miyukini, en garantissant l'intÃ©gritÃ© du registre, sans jamais dÃ©cider de ce qui est effectivement autorisÃ©.**

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il Ã©tablit des rÃ¨gles absolues qui ne peuvent Ãªtre contournÃ©es, nÃ©gociÃ©es, ou modifiÃ©es. Le contrat prime sur toute considÃ©ration pratique.

Toute implÃ©mentation de la Permission API doit respecter intÃ©gralement ce document. Toute Ã©volution doit prÃ©server les invariants dÃ©finis ici.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** FONDATION â€” Non nÃ©gociable  
**RÃ©fÃ©rence :** Miyukini Core System v2.4

**RÃ©fÃ©rences croisÃ©es :**

- [Master Butler - Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md) : DÃ©finition et responsabilitÃ©s de Master Butler
- [Master Butler - Permission Registry Contract](../registry/Master%20Butler%20-%20Permission%20Registry%20Contract.md) : ModÃ¨le de donnÃ©es du registre
- [Master Butler - Capability API Contract](./Master%20Butler%20-%20Capability%20API%20Contract.md) : Surface d'appel pour les capacitÃ©s
- [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) : DÃ©finitions canoniques
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) : Lois d'autonomie

---

## 17. Mini log â€” erreurs / warnings / ambiguÃ¯tÃ©s rencontrÃ©es et corrigÃ©es

### AmbiguÃ¯tÃ© A1 : Confusion entre Permission API et Permission Registry

**AmbiguÃ¯tÃ© rencontrÃ©e :** Risque de confusion entre la Permission API (surface d'appel) et le Permission Registry Contract (modÃ¨le de donnÃ©es).

**DÃ©cision prise :** DÃ©finition explicite de la complÃ©mentaritÃ© : le Registry Contract dÃ©finit le modÃ¨le de donnÃ©es, l'API Contract dÃ©finit la surface d'appel pour interagir avec ce modÃ¨le.

**Correction effectuÃ©e :** Section 1 et section 2 rÃ©digÃ©es avec clarification explicite de cette distinction.

### AmbiguÃ¯tÃ© A2 : ResponsabilitÃ© de vÃ©rification des permissions

**AmbiguÃ¯tÃ© rencontrÃ©e :** NÃ©cessitÃ© de clarifier que la Permission API ne vÃ©rifie jamais si une permission est accordÃ©e Ã  un contexte.

**DÃ©cision prise :** Interdiction explicite (INTERDIT-PERM-1, INTERDIT-PERM-2, INTERDIT-PERM-3) et rappel constant que la dÃ©cision appartient Ã  StrongFather.

**Correction effectuÃ©e :** Sections 3, 7, et schÃ©ma 14.3 rÃ©digÃ©s avec emphase sur cette sÃ©paration.

### AmbiguÃ¯tÃ© A3 : Gestion des cycles d'implication

**AmbiguÃ¯tÃ© rencontrÃ©e :** NÃ©cessitÃ© de dÃ©finir clairement le comportement en cas de tentative de crÃ©ation de cycle.

**DÃ©cision prise :** Ajout d'une opÃ©ration de validation `checkCycles` et interdiction explicite (INTERDIT-PERM-5) avec rejet REJET-PERM-6.

**Correction effectuÃ©e :** Sections 5.4, 7, et 9 rÃ©digÃ©es avec gestion explicite des cycles.

### VÃ©rification de compatibilitÃ©

**VÃ©rification effectuÃ©e :** VÃ©rification systÃ©matique de la compatibilitÃ© avec le Permission Registry Contract et la Documentation Fondatrice. Aucune contradiction dÃ©tectÃ©e.

**Conclusion :** Le contrat est strictement compatible avec le systÃ¨me contractuel existant.

---

*Aucune autre erreur, warning, ou ambiguÃ¯tÃ© rencontrÃ©e lors de la rÃ©daction de ce document.*

