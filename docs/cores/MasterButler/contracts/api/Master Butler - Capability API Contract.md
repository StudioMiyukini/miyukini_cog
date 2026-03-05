# Master Butler â€” Capability API Contract

## 1. Introduction

### Objet du contrat

Ce document dÃ©finit le **Master Butler â€” Capability API Contract** : un contrat normatif, non nÃ©gociable, et de statut FONDATION qui Ã©tablit la surface d'appel unique et autorisÃ©e pour toutes les opÃ©rations relatives aux capacitÃ©s dans le systÃ¨me Miyukini.

Ce contrat prÃ©cise les opÃ©rations de dÃ©claration, d'interrogation et de dÃ©couverte des capacitÃ©s, les rÃ¨gles d'appel, les garanties offertes, et les interactions avec les autres composants du systÃ¨me.

### PortÃ©e

Ce contrat s'applique Ã  **tous les composants** interagissant avec Master Butler pour les capacitÃ©s et dÃ©finit de maniÃ¨re absolue :
- la dÃ©finition formelle de la Capability API et son rÃ´le systÃ©mique,
- la typologie des opÃ©rations autorisÃ©es (dÃ©claration, interrogation, dÃ©couverte),
- les rÃ¨gles de contexte et de validation,
- ce que la Capability API PEUT et NE PEUT JAMAIS faire,
- les garanties offertes aux appelants,
- les invariants systÃ©miques associÃ©s.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il Ã©tablit des rÃ¨gles absolues qui ne peuvent Ãªtre contournÃ©es, nÃ©gociÃ©es, ou modifiÃ©es. Le contrat prime sur toute considÃ©ration pratique.

### Relation avec les autres contrats

Ce contrat complÃ¨te et respecte les documents contractuels existants :
- **Master Butler â€” Documentation Fondatrice** : DÃ©finition fondamentale du rÃ´le de Master Butler
- **Master Butler â€” Capability Registry Contract** : ModÃ¨le conceptuel du registre des capacitÃ©s
- **Master Butler â€” Permission Registry Contract** : ModÃ¨le conceptuel du registre des permissions
- **[Miyukini Conceptual References â€” Lois Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md)** : Ce contrat respecte **LOI-1** (aucune dÃ©pendance externe critique) en garantissant que toutes les opÃ©rations fonctionnent sans appel externe obligatoire.

Il n'introduit aucune contradiction avec le corpus documentaire existant.

---

## 2. RÃ´le et nature de la Capability API

### DÃ©finition formelle

La **Capability API** est la surface d'appel conceptuelle qui constitue l'interface formelle entre les composants du systÃ¨me et Master Butler pour toutes les opÃ©rations relatives aux capacitÃ©s. Elle reprÃ©sente l'ensemble des opÃ©rations que Master Butler expose pour dÃ©clarer, interroger, et dÃ©couvrir les capacitÃ©s du systÃ¨me.

### CaractÃ©ristiques formelles fondamentales

**Surface d'appel dÃ©diÃ©e aux capacitÃ©s :** La Capability API est la surface d'appel exclusive pour toutes les opÃ©rations relatives aux capacitÃ©s. Toute interaction avec le registre des capacitÃ©s DOIT passer par cette API.

**Interface conceptuelle :** La Capability API est une interface conceptuelle, pas une implÃ©mentation technique. Elle dÃ©finit les opÃ©rations autorisÃ©es de maniÃ¨re abstraite, sans prÃ©supposer aucune technologie, aucun protocole, ou aucun format de donnÃ©es.

**Registre passif :** La Capability API expose un registre passif. Elle recense, documente, et fournit des informations sur les capacitÃ©s, mais ne prend jamais de dÃ©cision et n'exÃ©cute jamais d'action fonctionnelle.

**MÃ©diation obligatoire :** Toute opÃ©ration sur les capacitÃ©s DOIT passer par la Capability API. Aucun accÃ¨s direct au registre n'est autorisÃ©.

**Abstraction de l'implÃ©mentation :** La Capability API abstrait complÃ¨tement l'implÃ©mentation interne de Master Butler. Les appelants interagissent avec des concepts, pas avec des mÃ©canismes techniques.

### Nature systÃ©mique

La Capability API est un **concept systÃ©mique**, pas une interface technique. Elle reprÃ©sente la frontiÃ¨re conceptuelle entre les appelants (modules, produits, cores) et le registre des capacitÃ©s de Master Butler.

**Important :** Cette dÃ©finition est purement conceptuelle et systÃ©mique. Elle ne prÃ©suppose aucune technologie, aucun langage de programmation, aucun protocole de communication, ou aucun format d'Ã©change.

---

## 3. Principe de non-dÃ©cision

### Ã‰noncÃ© formel

La Capability API **ne prend jamais de dÃ©cision**. Elle fournit des informations sur les capacitÃ©s, rÃ©pond Ã  des interrogations, mais ne produit jamais de verdict "autorisÃ©" ou "refusÃ©".

### CaractÃ©ristiques du principe de non-dÃ©cision

**Information pure :** La Capability API retourne des informations, pas des dÃ©cisions. Elle indique si une capacitÃ© existe, quelles sont ses mÃ©tadonnÃ©es, quelles permissions sont associÃ©es, mais ne dÃ©cide jamais si une action est autorisÃ©e.

**Pas de boolÃ©en d'autorisation :** Aucune mÃ©thode de la Capability API ne retourne un boolÃ©en d'autorisation. Les rÃ©ponses sont des informations, pas des verdicts.

**SÃ©paration stricte :** La dÃ©cision d'autorisation appartient exclusivement Ã  StrongFather. Master Butler fournit les informations nÃ©cessaires Ã  cette dÃ©cision.

### Non-nÃ©gociabilitÃ©s

- **NODEC-1 :** La Capability API ne retourne jamais "autorisÃ©" ou "refusÃ©"
- **NODEC-2 :** La Capability API ne participe jamais Ã  une dÃ©cision d'autorisation
- **NODEC-3 :** La Capability API fournit des informations, pas des verdicts
- **NODEC-4 :** Toute dÃ©cision appartient exclusivement Ã  StrongFather

---

## 4. DÃ©finition conceptuelle d'une opÃ©ration Capability API

### DÃ©finition formelle

Une **opÃ©ration Capability API** est une demande d'action conceptuelle formulÃ©e par un appelant Ã  destination de Master Butler, relative aux capacitÃ©s du systÃ¨me.

### CaractÃ©ristiques formelles d'une opÃ©ration

**Demande d'information ou de modification du registre :** Une opÃ©ration Capability API est soit une demande d'information (interrogation, dÃ©couverte), soit une demande de modification du registre (dÃ©claration, mise Ã  jour, dÃ©prÃ©ciation).

**Contexte requis :** Chaque opÃ©ration Capability API est accompagnÃ©e d'un contexte qui identifie l'appelant et son domaine d'origine.

**AtomicitÃ© conceptuelle :** Une opÃ©ration Capability API est atomique conceptuellement. Elle est exÃ©cutÃ©e complÃ¨tement ou pas du tout.

**TraÃ§abilitÃ© obligatoire :** Chaque opÃ©ration Capability API est tracÃ©e de maniÃ¨re complÃ¨te, permettant l'audit et le debugging.

### Structure conceptuelle d'une opÃ©ration

Conceptuellement, une opÃ©ration Capability API comprend :
- **Type d'opÃ©ration :** la catÃ©gorie de l'opÃ©ration (dÃ©claration, interrogation, dÃ©couverte)
- **ParamÃ¨tres :** les donnÃ©es nÃ©cessaires Ã  l'exÃ©cution de l'opÃ©ration
- **Contexte :** l'identitÃ© de l'appelant et son domaine
- **RÃ©sultat attendu :** le type de rÃ©sultat que l'opÃ©ration retourne

### Nature conceptuelle

Une opÃ©ration Capability API est un **concept systÃ©mique**, pas un appel technique. Elle reprÃ©sente une demande d'action conceptuelle qui sera validÃ©e et exÃ©cutÃ©e par Master Butler.

---

## 5. Typologie des opÃ©rations autorisÃ©es

### 5.1. OpÃ©rations de dÃ©claration

#### 5.1.1. DÃ©claration de capacitÃ©

**DÃ©finition formelle :**

Une **dÃ©claration de capacitÃ©** est une opÃ©ration Capability API par laquelle un composant (module, produit, adaptateur) informe Master Butler de l'existence d'une capacitÃ© qu'il possÃ¨de.

**CaractÃ©ristiques :**

- **Acte fondateur :** La dÃ©claration est l'acte par lequel une capacitÃ© entre dans le registre officiel
- **Idempotence :** DÃ©clarer deux fois la mÃªme capacitÃ© n'a pas d'effet supplÃ©mentaire
- **MÃ©tadonnÃ©es obligatoires :** La dÃ©claration inclut les mÃ©tadonnÃ©es requises (identifiant, nom, description, module d'origine)
- **Validation de forme :** Master Butler valide la forme de la dÃ©claration (identifiant unique, mÃ©tadonnÃ©es complÃ¨tes)
- **Pas de validation mÃ©tier :** Master Butler ne valide pas la pertinence mÃ©tier de la capacitÃ©

**ParamÃ¨tres conceptuels :**

| ParamÃ¨tre | Description | Obligation |
|-----------|-------------|------------|
| `capability_id` | Identifiant unique de la capacitÃ© | Obligatoire |
| `name` | Nom humain-lisible de la capacitÃ© | Obligatoire |
| `description` | Description de la capacitÃ© | Obligatoire |
| `module_origin` | Module dÃ©clarant la capacitÃ© | Obligatoire |
| `version` | Version de la capacitÃ© | Optionnel |
| `metadata` | MÃ©tadonnÃ©es additionnelles | Optionnel |

**RÃ©sultat conceptuel :**

- SuccÃ¨s : Confirmation d'enregistrement avec timestamp
- Erreur : Indication de l'erreur de forme (identifiant dupliquÃ©, mÃ©tadonnÃ©es manquantes)

#### 5.1.2. Mise Ã  jour de capacitÃ©

**DÃ©finition formelle :**

Une **mise Ã  jour de capacitÃ©** est une opÃ©ration Capability API par laquelle un composant modifie les mÃ©tadonnÃ©es d'une capacitÃ© existante.

**CaractÃ©ristiques :**

- **CapacitÃ© existante :** Seule une capacitÃ© existante peut Ãªtre mise Ã  jour
- **ImmutabilitÃ© de l'identifiant :** L'identifiant de la capacitÃ© ne peut jamais Ãªtre modifiÃ©
- **Module d'origine requis :** Seul le module d'origine peut mettre Ã  jour la capacitÃ©
- **TraÃ§abilitÃ© :** La mise Ã  jour est tracÃ©e avec l'Ã©tat avant/aprÃ¨s

**ParamÃ¨tres conceptuels :**

| ParamÃ¨tre | Description | Obligation |
|-----------|-------------|------------|
| `capability_id` | Identifiant de la capacitÃ© Ã  mettre Ã  jour | Obligatoire |
| `module_origin` | Module demandant la mise Ã  jour | Obligatoire |
| `updates` | Modifications Ã  appliquer | Obligatoire |

**RÃ©sultat conceptuel :**

- SuccÃ¨s : Confirmation de mise Ã  jour avec nouveau state
- Erreur : Indication de l'erreur (capacitÃ© inexistante, module non autorisÃ©)

#### 5.1.3. DÃ©prÃ©ciation de capacitÃ©

**DÃ©finition formelle :**

Une **dÃ©prÃ©ciation de capacitÃ©** est une opÃ©ration Capability API par laquelle un composant marque une capacitÃ© comme obsolÃ¨te.

**CaractÃ©ristiques :**

- **Marquage soft :** La dÃ©prÃ©ciation ne supprime pas la capacitÃ©, elle la marque comme obsolÃ¨te
- **Information de remplacement :** La dÃ©prÃ©ciation peut indiquer une capacitÃ© de remplacement
- **TraÃ§abilitÃ© :** La dÃ©prÃ©ciation est tracÃ©e avec la raison et la date
- **Module d'origine requis :** Seul le module d'origine peut dÃ©prÃ©cier la capacitÃ©

**ParamÃ¨tres conceptuels :**

| ParamÃ¨tre | Description | Obligation |
|-----------|-------------|------------|
| `capability_id` | Identifiant de la capacitÃ© Ã  dÃ©prÃ©cier | Obligatoire |
| `module_origin` | Module demandant la dÃ©prÃ©ciation | Obligatoire |
| `reason` | Raison de la dÃ©prÃ©ciation | Obligatoire |
| `replacement_id` | Identifiant de la capacitÃ© de remplacement | Optionnel |
| `deprecation_date` | Date effective de dÃ©prÃ©ciation | Optionnel |

**RÃ©sultat conceptuel :**

- SuccÃ¨s : Confirmation de dÃ©prÃ©ciation avec nouveau state
- Erreur : Indication de l'erreur (capacitÃ© inexistante, module non autorisÃ©)

### 5.2. OpÃ©rations d'interrogation

#### 5.2.1. VÃ©rification d'existence

**DÃ©finition formelle :**

Une **vÃ©rification d'existence** est une opÃ©ration Capability API qui dÃ©termine si une capacitÃ© existe dans le registre.

**CaractÃ©ristiques :**

- **RÃ©ponse boolÃ©enne :** L'opÃ©ration retourne vrai si la capacitÃ© existe, faux sinon
- **Pas de dÃ©cision :** La rÃ©ponse est une information, pas une autorisation
- **Inclusion des dÃ©prÃ©ciÃ©es :** Par dÃ©faut, les capacitÃ©s dÃ©prÃ©ciÃ©es sont considÃ©rÃ©es comme existantes (avec un flag)

**ParamÃ¨tres conceptuels :**

| ParamÃ¨tre | Description | Obligation |
|-----------|-------------|------------|
| `capability_id` | Identifiant de la capacitÃ© Ã  vÃ©rifier | Obligatoire |
| `include_deprecated` | Inclure les capacitÃ©s dÃ©prÃ©ciÃ©es | Optionnel (dÃ©faut: true) |

**RÃ©sultat conceptuel :**

```
{
  exists: boolean,
  deprecated: boolean | null,
  deprecation_info: { reason, replacement_id, date } | null
}
```

#### 5.2.2. RÃ©cupÃ©ration de capacitÃ©

**DÃ©finition formelle :**

Une **rÃ©cupÃ©ration de capacitÃ©** est une opÃ©ration Capability API qui retourne les informations complÃ¨tes d'une capacitÃ©.

**CaractÃ©ristiques :**

- **Informations complÃ¨tes :** Toutes les mÃ©tadonnÃ©es de la capacitÃ© sont retournÃ©es
- **CapacitÃ© existante requise :** La capacitÃ© doit exister dans le registre
- **Inclusion des associations :** Les permissions associÃ©es peuvent Ãªtre incluses

**ParamÃ¨tres conceptuels :**

| ParamÃ¨tre | Description | Obligation |
|-----------|-------------|------------|
| `capability_id` | Identifiant de la capacitÃ© Ã  rÃ©cupÃ©rer | Obligatoire |
| `include_permissions` | Inclure les permissions associÃ©es | Optionnel (dÃ©faut: false) |
| `include_history` | Inclure l'historique des modifications | Optionnel (dÃ©faut: false) |

**RÃ©sultat conceptuel :**

```
{
  capability_id: string,
  name: string,
  description: string,
  module_origin: string,
  version: string | null,
  metadata: object,
  created_at: timestamp,
  updated_at: timestamp,
  deprecated: boolean,
  deprecation_info: object | null,
  permissions: array | null,
  history: array | null
}
```

#### 5.2.3. Listage des capacitÃ©s

**DÃ©finition formelle :**

Un **listage des capacitÃ©s** est une opÃ©ration Capability API qui retourne une liste de capacitÃ©s selon des critÃ¨res de filtrage.

**CaractÃ©ristiques :**

- **Filtrage :** Les capacitÃ©s peuvent Ãªtre filtrÃ©es par module, par Ã©tat, par mÃ©tadonnÃ©es
- **Pagination :** Les rÃ©sultats peuvent Ãªtre paginÃ©s pour les grands registres
- **Tri :** Les rÃ©sultats peuvent Ãªtre triÃ©s par diffÃ©rents critÃ¨res

**ParamÃ¨tres conceptuels :**

| ParamÃ¨tre | Description | Obligation |
|-----------|-------------|------------|
| `filter_module` | Filtrer par module d'origine | Optionnel |
| `filter_deprecated` | Filtrer par Ã©tat de dÃ©prÃ©ciation | Optionnel |
| `filter_metadata` | Filtrer par mÃ©tadonnÃ©es | Optionnel |
| `pagination` | ParamÃ¨tres de pagination | Optionnel |
| `sort` | CritÃ¨re de tri | Optionnel |

**RÃ©sultat conceptuel :**

```
{
  capabilities: array,
  total_count: number,
  pagination_info: object | null
}
```

#### 5.2.4. RÃ©cupÃ©ration des permissions requises

**DÃ©finition formelle :**

Une **rÃ©cupÃ©ration des permissions requises** est une opÃ©ration Capability API qui retourne les permissions associÃ©es Ã  une capacitÃ©.

**CaractÃ©ristiques :**

- **Information pour dÃ©cideurs :** Cette opÃ©ration est principalement utilisÃ©e par StrongFather pour connaÃ®tre les permissions requises
- **Liste complÃ¨te :** Toutes les permissions associÃ©es sont retournÃ©es
- **Pas de dÃ©cision :** L'opÃ©ration retourne des informations, pas un verdict

**ParamÃ¨tres conceptuels :**

| ParamÃ¨tre | Description | Obligation |
|-----------|-------------|------------|
| `capability_id` | Identifiant de la capacitÃ© | Obligatoire |

**RÃ©sultat conceptuel :**

```
{
  capability_id: string,
  required_permissions: array,
  permission_details: array
}
```

### 5.3. OpÃ©rations de dÃ©couverte

#### 5.3.1. DÃ©couverte par module

**DÃ©finition formelle :**

Une **dÃ©couverte par module** est une opÃ©ration Capability API qui retourne toutes les capacitÃ©s dÃ©clarÃ©es par un module spÃ©cifique.

**CaractÃ©ristiques :**

- **Scope module :** La dÃ©couverte est limitÃ©e Ã  un module spÃ©cifique
- **Informations complÃ¨tes :** Chaque capacitÃ© est retournÃ©e avec ses mÃ©tadonnÃ©es
- **Exclusion optionnelle des dÃ©prÃ©ciÃ©es :** Les capacitÃ©s dÃ©prÃ©ciÃ©es peuvent Ãªtre exclues

**ParamÃ¨tres conceptuels :**

| ParamÃ¨tre | Description | Obligation |
|-----------|-------------|------------|
| `module_id` | Identifiant du module | Obligatoire |
| `include_deprecated` | Inclure les capacitÃ©s dÃ©prÃ©ciÃ©es | Optionnel (dÃ©faut: false) |

**RÃ©sultat conceptuel :**

```
{
  module_id: string,
  capabilities: array,
  total_count: number
}
```

#### 5.3.2. DÃ©couverte par type d'action

**DÃ©finition formelle :**

Une **dÃ©couverte par type d'action** est une opÃ©ration Capability API qui retourne toutes les capacitÃ©s correspondant Ã  un type d'action (create, read, update, delete, etc.).

**CaractÃ©ristiques :**

- **Scope action :** La dÃ©couverte est basÃ©e sur le type d'action
- **Pattern matching :** Les capacitÃ©s sont filtrÃ©es par pattern d'identifiant (ex: `*.create`, `content.*`)
- **Cross-module :** La dÃ©couverte traverse tous les modules

**ParamÃ¨tres conceptuels :**

| ParamÃ¨tre | Description | Obligation |
|-----------|-------------|------------|
| `action_pattern` | Pattern de type d'action | Obligatoire |
| `include_deprecated` | Inclure les capacitÃ©s dÃ©prÃ©ciÃ©es | Optionnel (dÃ©faut: false) |

**RÃ©sultat conceptuel :**

```
{
  action_pattern: string,
  capabilities: array,
  total_count: number
}
```

#### 5.3.3. DÃ©couverte contextuelle

**DÃ©finition formelle :**

Une **dÃ©couverte contextuelle** est une opÃ©ration Capability API qui retourne les capacitÃ©s accessibles dans un contexte donnÃ© (rÃ´le, permissions dÃ©tenues).

**CaractÃ©ristiques :**

- **Scope contexte :** La dÃ©couverte est filtrÃ©e par le contexte fourni
- **Projection :** Retourne une projection des capacitÃ©s accessibles, pas une dÃ©cision d'autorisation
- **Information prÃ©paratoire :** UtilisÃ©e pour prÃ©parer les informations avant une dÃ©cision de StrongFather

**ParamÃ¨tres conceptuels :**

| ParamÃ¨tre | Description | Obligation |
|-----------|-------------|------------|
| `context_roles` | RÃ´les du contexte | Obligatoire |
| `context_permissions` | Permissions du contexte | Optionnel |
| `target_module` | Module cible (optionnel) | Optionnel |

**RÃ©sultat conceptuel :**

```
{
  context_summary: object,
  accessible_capabilities: array,
  total_count: number,
  note: "This is a projection, not an authorization decision"
}
```

**IMPORTANT :** Cette opÃ©ration retourne une **projection informationnelle**, pas une dÃ©cision d'autorisation. La dÃ©cision finale appartient Ã  StrongFather.

#### 5.3.4. Recherche de capacitÃ©s

**DÃ©finition formelle :**

Une **recherche de capacitÃ©s** est une opÃ©ration Capability API qui permet de rechercher des capacitÃ©s par mots-clÃ©s dans les mÃ©tadonnÃ©es.

**CaractÃ©ristiques :**

- **Recherche textuelle :** La recherche porte sur les noms, descriptions, et mÃ©tadonnÃ©es
- **RÃ©sultats pondÃ©rÃ©s :** Les rÃ©sultats peuvent Ãªtre triÃ©s par pertinence
- **Cross-module :** La recherche traverse tous les modules

**ParamÃ¨tres conceptuels :**

| ParamÃ¨tre | Description | Obligation |
|-----------|-------------|------------|
| `query` | Mots-clÃ©s de recherche | Obligatoire |
| `search_fields` | Champs Ã  rechercher | Optionnel (dÃ©faut: tous) |
| `include_deprecated` | Inclure les capacitÃ©s dÃ©prÃ©ciÃ©es | Optionnel (dÃ©faut: false) |
| `limit` | Nombre maximum de rÃ©sultats | Optionnel |

**RÃ©sultat conceptuel :**

```
{
  query: string,
  results: array,
  total_count: number
}
```

---

## 6. Ce que la Capability API PEUT faire

### 6.1. OpÃ©rations autorisÃ©es

La Capability API PEUT effectuer les opÃ©rations suivantes :

**PEUT-1 : Enregistrer des dÃ©clarations de capacitÃ©s**

La Capability API PEUT enregistrer des dÃ©clarations de capacitÃ©s provenant de modules, produits, ou adaptateurs, sous rÃ©serve que la dÃ©claration soit valide formellement.

**PEUT-2 : Mettre Ã  jour des capacitÃ©s existantes**

La Capability API PEUT mettre Ã  jour les mÃ©tadonnÃ©es de capacitÃ©s existantes, sous rÃ©serve que le module d'origine autorise la modification.

**PEUT-3 : Marquer des capacitÃ©s comme dÃ©prÃ©ciÃ©es**

La Capability API PEUT marquer des capacitÃ©s comme dÃ©prÃ©ciÃ©es, sous rÃ©serve que le module d'origine autorise la dÃ©prÃ©ciation.

**PEUT-4 : RÃ©pondre aux interrogations**

La Capability API PEUT rÃ©pondre Ã  toutes les interrogations sur les capacitÃ©s : existence, mÃ©tadonnÃ©es, permissions associÃ©es.

**PEUT-5 : Permettre la dÃ©couverte**

La Capability API PEUT permettre la dÃ©couverte des capacitÃ©s par module, par type d'action, par contexte, ou par recherche.

**PEUT-6 : Fournir des projections contextuelles**

La Capability API PEUT fournir des projections de capacitÃ©s accessibles dans un contexte donnÃ©, Ã  titre informatif.

**PEUT-7 : Tracer toutes les opÃ©rations**

La Capability API PEUT et DOIT tracer toutes les opÃ©rations pour permettre l'audit.

**PEUT-8 : Retourner des erreurs explicites**

La Capability API PEUT retourner des erreurs explicites et actionnables lorsqu'une opÃ©ration ne peut pas Ãªtre exÃ©cutÃ©e.

### 6.2. Garanties associÃ©es

Chaque opÃ©ration autorisÃ©e est accompagnÃ©e des garanties suivantes :
- Validation de forme avant exÃ©cution
- Idempotence des dÃ©clarations
- TraÃ§abilitÃ© complÃ¨te
- Erreur explicite en cas de rejet
- CohÃ©rence prÃ©servÃ©e aprÃ¨s exÃ©cution

---

## 7. Ce que la Capability API NE PEUT JAMAIS faire

### 7.1. Interdictions absolues

La Capability API NE PEUT JAMAIS effectuer les actions suivantes. Ces interdictions sont absolues et non nÃ©gociables.

**INTERDIT-1 : Prendre des dÃ©cisions d'autorisation**

La Capability API NE PEUT JAMAIS retourner une dÃ©cision d'autorisation. Elle fournit des informations, pas des verdicts. La dÃ©cision appartient exclusivement Ã  StrongFather.

**INTERDIT-2 : VÃ©rifier des permissions en temps rÃ©el**

La Capability API NE PEUT JAMAIS vÃ©rifier si un utilisateur ou contexte possÃ¨de effectivement une permission au moment d'une action. Cette vÃ©rification appartient Ã  StrongFather.

**INTERDIT-3 : ExÃ©cuter des actions fonctionnelles**

La Capability API NE PEUT JAMAIS exÃ©cuter d'action fonctionnelle. Elle ne crÃ©e pas de contenu, ne modifie pas de donnÃ©es mÃ©tier, ne tÃ©lÃ©verse pas de fichiers. Elle gÃ¨re uniquement les mÃ©tadonnÃ©es de capacitÃ©s.

**INTERDIT-4 : Stocker des donnÃ©es mÃ©tier**

La Capability API NE PEUT JAMAIS stocker de donnÃ©es mÃ©tier. Elle stocke uniquement des mÃ©tadonnÃ©es : dÃ©finitions de capacitÃ©s, associations, historiques.

**INTERDIT-5 : GÃ©rer les identitÃ©s**

La Capability API NE PEUT JAMAIS gÃ©rer les identitÃ©s des utilisateurs ou des systÃ¨mes. Elle connaÃ®t les associations rÃ´les-permissions-capacitÃ©s, mais pas les identitÃ©s.

**INTERDIT-6 : DÃ©finir des politiques**

La Capability API NE PEUT JAMAIS dÃ©finir de politiques de dÃ©cision. Les politiques appartiennent Ã  StrongFather.

**INTERDIT-7 : Appliquer des contraintes mÃ©tier**

La Capability API NE PEUT JAMAIS appliquer de contraintes mÃ©tier. Elle dÃ©finit ce qui existe, pas comment l'utiliser.

**INTERDIT-8 : Modifier l'identifiant d'une capacitÃ©**

La Capability API NE PEUT JAMAIS modifier l'identifiant d'une capacitÃ© existante. Les identifiants sont immuables.

**INTERDIT-9 : Supprimer physiquement une capacitÃ©**

La Capability API NE PEUT JAMAIS supprimer physiquement une capacitÃ© du registre. Les capacitÃ©s peuvent Ãªtre dÃ©prÃ©ciÃ©es, jamais supprimÃ©es.

**INTERDIT-10 : Contourner la traÃ§abilitÃ©**

La Capability API NE PEUT JAMAIS effectuer une opÃ©ration sans traÃ§abilitÃ©. Toute opÃ©ration est enregistrÃ©e.

### 7.2. Justifications

Ces interdictions sont justifiÃ©es par :
- le respect du principe de non-dÃ©cision de Master Butler,
- la sÃ©paration stricte des responsabilitÃ©s entre cores,
- la prÃ©servation de l'intÃ©gritÃ© du registre,
- le maintien de la traÃ§abilitÃ© complÃ¨te,
- le respect de l'architecture Miyukini.

---

## 8. RÃ¨gles absolues d'appel (prÃ©conditions)

### 8.1. PrÃ©conditions obligatoires

Chaque appel Capability API DOIT respecter les prÃ©conditions suivantes.

**PRECOND-1 : Identifiant d'appelant obligatoire**

Chaque appel Capability API DOIT identifier l'appelant (module, core, produit). Les appels anonymes sont rejetÃ©s.

**PRECOND-2 : Format d'identifiant valide (pour dÃ©clarations)**

Pour les opÃ©rations de dÃ©claration, l'identifiant de capacitÃ© DOIT respecter le format canonique : `module.action` ou `module.domain.action`.

**PRECOND-3 : MÃ©tadonnÃ©es complÃ¨tes (pour dÃ©clarations)**

Pour les opÃ©rations de dÃ©claration, les mÃ©tadonnÃ©es obligatoires DOIVENT Ãªtre fournies (identifiant, nom, description, module d'origine).

**PRECOND-4 : CapacitÃ© existante (pour interrogations spÃ©cifiques)**

Pour les opÃ©rations d'interrogation spÃ©cifique (get, permissions), la capacitÃ© DOIT exister dans le registre.

**PRECOND-5 : Module d'origine correspondant (pour modifications)**

Pour les opÃ©rations de modification (update, deprecate), le module appelant DOIT Ãªtre le module d'origine de la capacitÃ©.

**PRECOND-6 : Appel lÃ©gal**

L'opÃ©ration demandÃ©e DOIT Ãªtre une opÃ©ration lÃ©gale et documentÃ©e de la Capability API.

### 8.2. RÃ¨gles de validation des prÃ©conditions

- Les prÃ©conditions sont validÃ©es dans l'ordre
- Si une prÃ©condition Ã©choue, l'appel est rejetÃ© immÃ©diatement
- L'erreur de rejet indique la prÃ©condition non satisfaite
- Aucune exÃ©cution partielle n'est autorisÃ©e aprÃ¨s un Ã©chec de prÃ©condition

---

## 9. RÃ¨gles absolues de rejet

### 9.1. Conditions de rejet

Un appel Capability API est rejetÃ© si l'une des conditions suivantes est dÃ©tectÃ©e :

**REJET-1 : Appelant non identifiÃ©**

L'appel est rejetÃ© si l'appelant n'est pas identifiÃ©.
- Erreur retournÃ©e : `CALLER_NOT_IDENTIFIED`
- TraÃ§abilitÃ© : tentative tracÃ©e

**REJET-2 : Format d'identifiant invalide**

L'appel est rejetÃ© si l'identifiant de capacitÃ© ne respecte pas le format canonique.
- Erreur retournÃ©e : `INVALID_CAPABILITY_ID_FORMAT`
- TraÃ§abilitÃ© : erreur de format tracÃ©e

**REJET-3 : MÃ©tadonnÃ©es incomplÃ¨tes**

L'appel est rejetÃ© si les mÃ©tadonnÃ©es obligatoires sont manquantes.
- Erreur retournÃ©e : `INCOMPLETE_METADATA`
- TraÃ§abilitÃ© : erreur de mÃ©tadonnÃ©es tracÃ©e

**REJET-4 : CapacitÃ© inexistante (pour interrogations)**

L'appel est rejetÃ© si la capacitÃ© demandÃ©e n'existe pas dans le registre.
- Erreur retournÃ©e : `CAPABILITY_NOT_FOUND`
- TraÃ§abilitÃ© : tentative tracÃ©e

**REJET-5 : Module non autorisÃ© (pour modifications)**

L'appel est rejetÃ© si le module appelant n'est pas le module d'origine de la capacitÃ©.
- Erreur retournÃ©e : `MODULE_NOT_AUTHORIZED`
- TraÃ§abilitÃ© : tentative non autorisÃ©e tracÃ©e

**REJET-6 : Identifiant dupliquÃ© (pour dÃ©clarations)**

L'appel est rejetÃ© si l'identifiant de capacitÃ© existe dÃ©jÃ  (sauf si idempotence s'applique).
- Erreur retournÃ©e : `CAPABILITY_ID_EXISTS`
- TraÃ§abilitÃ© : duplication tracÃ©e

**REJET-7 : OpÃ©ration illÃ©gale**

L'appel est rejetÃ© si l'opÃ©ration demandÃ©e n'est pas une opÃ©ration lÃ©gale de la Capability API.
- Erreur retournÃ©e : `ILLEGAL_OPERATION`
- TraÃ§abilitÃ© : tentative tracÃ©e

### 9.2. Garanties aprÃ¨s rejet

AprÃ¨s tout rejet, les garanties suivantes s'appliquent :
- L'Ã©tat du registre reste inchangÃ©
- Aucune modification partielle n'est appliquÃ©e
- L'erreur est explicite et actionnable
- Le rejet est tracÃ© pour audit

### 9.3. RÃ¨gles absolues

- **R-REJ-1 :** Tout rejet laisse l'Ã©tat inchangÃ©
- **R-REJ-2 :** Tout rejet retourne une erreur explicite
- **R-REJ-3 :** Tout rejet est tracÃ©
- **R-REJ-4 :** Aucune exception au rejet n'est autorisÃ©e

---

## 10. Garanties offertes aux appelants

### 10.1. Garanties de traitement

**G-CAP-1 : Traitement prÃ©visible des opÃ©rations valides**

Si un appelant fournit un contexte valide et effectue des appels lÃ©gaux, Master Butler traite les opÃ©rations de maniÃ¨re prÃ©visible et conforme au contrat.

**G-CAP-2 : Messages d'erreur explicites et actionnables**

Si une opÃ©ration est rejetÃ©e, Master Butler retourne toujours un message d'erreur explicite et actionnable qui permet Ã  l'appelant de comprendre et corriger le problÃ¨me.

**G-CAP-3 : Pas de rejet arbitraire**

Master Butler ne rejette jamais une opÃ©ration de maniÃ¨re arbitraire. Tout rejet est justifiÃ© par une violation de prÃ©condition ou une condition de rejet documentÃ©e.

**G-CAP-4 : Idempotence des dÃ©clarations**

Les dÃ©clarations de capacitÃ©s sont idempotentes. DÃ©clarer deux fois la mÃªme capacitÃ© avec les mÃªmes mÃ©tadonnÃ©es n'a pas d'effet supplÃ©mentaire.

### 10.2. Garanties de cohÃ©rence

**G-CAP-5 : CohÃ©rence du registre**

AprÃ¨s toute opÃ©ration rÃ©ussie, le registre reste cohÃ©rent et conforme aux contraintes structurelles.

**G-CAP-6 : Ã‰tat inchangÃ© aprÃ¨s rejet**

AprÃ¨s tout rejet, l'Ã©tat du registre reste inchangÃ©.

**G-CAP-7 : ImmutabilitÃ© des identifiants**

Les identifiants de capacitÃ©s sont immuables. Une capacitÃ© dÃ©clarÃ©e garde son identifiant Ã  jamais.

### 10.3. Garanties de traÃ§abilitÃ©

**G-CAP-8 : TraÃ§abilitÃ© complÃ¨te**

Toutes les opÃ©rations sont tracÃ©es de maniÃ¨re complÃ¨te, permettant l'audit.

**G-CAP-9 : Historique des modifications**

L'historique des modifications de chaque capacitÃ© est conservÃ© et accessible.

### 10.4. Garanties de disponibilitÃ©

**G-CAP-10 : Registre local**

Le registre des capacitÃ©s est local. Toutes les opÃ©rations fonctionnent sans dÃ©pendance externe.

Cette garantie respecte **LOI-1** (aucune dÃ©pendance externe critique) : le registre fonctionne localement sans nÃ©cessiter d'appels externes.

### 10.5. Non-nÃ©gociabilitÃ©

Ces garanties sont absolues et non nÃ©gociables. Elles s'appliquent Ã  tous les appelants, sans exception.

---

## 11. Interactions avec les autres composants

### 11.1. Interaction avec StrongFather

**Relation formelle :**

StrongFather interroge la Capability API pour obtenir les informations nÃ©cessaires Ã  ses dÃ©cisions d'autorisation.

**Points d'interaction :**

- **VÃ©rification d'existence :** StrongFather vÃ©rifie si une capacitÃ© existe avant d'Ã©valuer une intention
- **Permissions requises :** StrongFather rÃ©cupÃ¨re les permissions associÃ©es Ã  une capacitÃ©
- **Contexte de capacitÃ© :** StrongFather peut demander une projection contextuelle

**RÃ¨gles :**

- StrongFather est toujours autorisÃ© Ã  interroger la Capability API
- Les rÃ©ponses sont exhaustives et exactes
- La Capability API ne suggÃ¨re pas de dÃ©cision

### 11.2. Interaction avec BondingBrother

**Relation formelle :**

BondingBrother interroge la Capability API lors de la traduction des intentions.

**Points d'interaction :**

- **VÃ©rification d'existence :** BondingBrother vÃ©rifie si une capacitÃ© existe dans un module
- **DÃ©couverte :** BondingBrother peut dÃ©couvrir les capacitÃ©s d'un module cible
- **Permissions requises :** BondingBrother peut rÃ©cupÃ©rer les permissions associÃ©es pour prÃ©parer le contexte

**RÃ¨gles :**

- BondingBrother peut interroger la Capability API pour la traduction
- Les informations prÃ©parent le contexte pour StrongFather
- BondingBrother ne prend pas de dÃ©cision basÃ©e sur ces informations

### 11.3. Interaction avec les modules et produits

**Relation formelle :**

Les modules et produits utilisent la Capability API pour dÃ©clarer leurs capacitÃ©s et dÃ©couvrir les capacitÃ©s des autres.

**Points d'interaction :**

- **DÃ©claration :** Les modules dÃ©clarent leurs capacitÃ©s au dÃ©marrage
- **DÃ©couverte :** Les modules dÃ©couvrent les capacitÃ©s disponibles
- **Mise Ã  jour :** Les modules mettent Ã  jour leurs capacitÃ©s si nÃ©cessaire
- **DÃ©prÃ©ciation :** Les modules dÃ©prÃ©cient leurs anciennes capacitÃ©s

**RÃ¨gles :**

- Les modules ne peuvent modifier que leurs propres capacitÃ©s
- La dÃ©claration est obligatoire pour toute capacitÃ© exposÃ©e
- La dÃ©couverte est accessible Ã  tous les composants autorisÃ©s

### 11.4. Interaction avec Permission API

**Relation formelle :**

La Capability API et la Permission API sont complÃ©mentaires et interagissent via les associations.

**Points d'interaction :**

- **Associations :** Les permissions rÃ©fÃ©rencent des capacitÃ©s via la Capability API
- **Validation :** Lors de la dÃ©finition d'une permission, l'existence des capacitÃ©s rÃ©fÃ©rencÃ©es est validÃ©e

**RÃ¨gles :**

- Une permission ne peut pas rÃ©fÃ©rencer une capacitÃ© inexistante
- La dÃ©prÃ©ciation d'une capacitÃ© n'invalide pas automatiquement les permissions associÃ©es (warning)

---

## 12. Invariants systÃ©miques liÃ©s Ã  la Capability API

### 12.1. Invariants globaux

**INV-CAP-1 : ExhaustivitÃ© du registre**

Le registre de Master Butler est exhaustif. Toute capacitÃ© existant dans le systÃ¨me est recensÃ©e. Si une capacitÃ© n'est pas dans le registre, elle n'existe pas officiellement.

**INV-CAP-2 : Non-dÃ©cision**

La Capability API ne prend jamais de dÃ©cision. Elle fournit des informations, pas des verdicts.

**INV-CAP-3 : Idempotence des dÃ©clarations**

Les dÃ©clarations de capacitÃ©s sont idempotentes. DÃ©clarer deux fois la mÃªme capacitÃ© n'a pas d'effet supplÃ©mentaire.

**INV-CAP-4 : ImmutabilitÃ© des identifiants**

Les identifiants de capacitÃ©s sont immuables. Une fois dÃ©clarÃ©s, ils ne changent jamais.

**INV-CAP-5 : TraÃ§abilitÃ© complÃ¨te**

Toute opÃ©ration est tracÃ©e. Aucune opÃ©ration sans traÃ§abilitÃ© n'est possible.

**INV-CAP-6 : Pas de suppression physique**

Les capacitÃ©s ne sont jamais supprimÃ©es physiquement. Elles peuvent Ãªtre dÃ©prÃ©ciÃ©es, jamais effacÃ©es.

### 12.2. Invariants de dÃ©claration

**INV-DECL-1 : Format canonique**

Tout identifiant de capacitÃ© respecte le format canonique.

**INV-DECL-2 : Module d'origine requis**

Toute capacitÃ© a un module d'origine identifiÃ©.

**INV-DECL-3 : MÃ©tadonnÃ©es complÃ¨tes**

Toute capacitÃ© a des mÃ©tadonnÃ©es complÃ¨tes (nom, description).

### 12.3. Invariants d'interrogation

**INV-INT-1 : RÃ©ponse complÃ¨te**

Toute interrogation retourne une rÃ©ponse complÃ¨te, jamais partielle.

**INV-INT-2 : CohÃ©rence temporelle**

Les donnÃ©es retournÃ©es reflÃ¨tent l'Ã©tat du registre au moment de l'interrogation.

---

## 13. Cas explicitement hors pÃ©rimÃ¨tre

### 13.1. Ce que la Capability API n'inclut PAS

Les Ã©lÃ©ments suivants sont **explicitement hors du pÃ©rimÃ¨tre** de la Capability API :

**HORS-1 : DÃ©tails d'implÃ©mentation**

La Capability API ne dÃ©finit pas les dÃ©tails d'implÃ©mentation techniques (langages, protocoles, formats de donnÃ©es). Elle est purement conceptuelle.

**HORS-2 : DÃ©cisions d'autorisation**

La Capability API ne dÃ©finit pas et n'exÃ©cute pas de dÃ©cisions d'autorisation. Les dÃ©cisions appartiennent Ã  StrongFather.

**HORS-3 : Gestion des permissions**

La gestion des permissions (dÃ©finition, association, attribution) appartient Ã  la Permission API, pas Ã  la Capability API.

**HORS-4 : ExÃ©cution des capacitÃ©s**

L'exÃ©cution des capacitÃ©s appartient aux modules et produits, pas Ã  Master Butler.

**HORS-5 : Logique mÃ©tier**

La Capability API ne dÃ©finit pas la logique mÃ©tier des capacitÃ©s. Elle recense ce qui existe, pas ce que cela fait concrÃ¨tement.

**HORS-6 : Cycle de vie technique des Tools**

Le cycle de vie technique des Tools (versions, compatibilitÃ©) appartient Ã  Ever Buddy, pas Ã  Master Butler.

### 13.2. Justification

Ces Ã©lÃ©ments sont hors pÃ©rimÃ¨tre car :
- la Capability API est une abstraction conceptuelle, pas une implÃ©mentation technique,
- la sÃ©paration des responsabilitÃ©s entre cores est stricte,
- Master Butler recense, mais ne dÃ©cide pas et n'exÃ©cute pas.

---

## 14. SchÃ©mas ASCII

### 14.1. Position de la Capability API dans l'architecture

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                    APPELANTS                                     â”‚
â”‚                                                                   â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚  StrongFather â”‚  â”‚ BondingBrotherâ”‚  â”‚  Modules/Produits â”‚  â”‚
â”‚  â”‚  (dÃ©cideur)   â”‚  â”‚  (mÃ©diateur)  â”‚  â”‚  (dÃ©clarants)     â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                            â”‚
                            â”‚ Appels Capability API
                            â”‚
                            â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                    CAPABILITY API                                 â”‚
â”‚                    (Master Butler)                                â”‚
â”‚                                                                   â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚  OPÃ‰RATIONS AUTORISÃ‰ES :                                  â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  DÃ‰CLARATION          INTERROGATION        DÃ‰COUVERTE     â”‚ â”‚
â”‚  â”‚  â€¢ declare            â€¢ exists             â€¢ by_module    â”‚ â”‚
â”‚  â”‚  â€¢ update             â€¢ get                â€¢ by_action    â”‚ â”‚
â”‚  â”‚  â€¢ deprecate          â€¢ list               â€¢ contextual   â”‚ â”‚
â”‚  â”‚                       â€¢ permissions        â€¢ search       â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  PRINCIPES :                                              â”‚ â”‚
â”‚  â”‚  âœ“ Information pure (pas de dÃ©cision)                     â”‚ â”‚
â”‚  â”‚  âœ“ Idempotence des dÃ©clarations                          â”‚ â”‚
â”‚  â”‚  âœ“ ImmutabilitÃ© des identifiants                         â”‚ â”‚
â”‚  â”‚  âœ“ TraÃ§abilitÃ© complÃ¨te                                  â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                            â”‚
                            â”‚
                            â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                    REGISTRE DES CAPACITÃ‰S                        â”‚
â”‚                                                                   â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚  â€¢ Inventaire exhaustif des capacitÃ©s                     â”‚ â”‚
â”‚  â”‚  â€¢ MÃ©tadonnÃ©es de chaque capacitÃ©                        â”‚ â”‚
â”‚  â”‚  â€¢ Associations avec permissions                          â”‚ â”‚
â”‚  â”‚  â€¢ Historique des modifications                           â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 14.2. Flux de dÃ©claration de capacitÃ©

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚              FLUX DE DÃ‰CLARATION DE CAPACITÃ‰                     â”‚
â”‚                                                                   â”‚
â”‚  MODULE                                                           â”‚
â”‚      â”‚                                                            â”‚
â”‚      â”‚ 1. PrÃ©paration de la dÃ©claration                          â”‚
â”‚      â”‚    - Identifiant unique (format canonique)                â”‚
â”‚      â”‚    - MÃ©tadonnÃ©es complÃ¨tes                                â”‚
â”‚      â”‚                                                            â”‚
â”‚      â–¼                                                            â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚              CAPABILITY API                               â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  2. Validation des prÃ©conditions                         â”‚ â”‚
â”‚  â”‚     - Appelant identifiÃ© ?                               â”‚ â”‚
â”‚  â”‚     - Format d'identifiant valide ?                      â”‚ â”‚
â”‚  â”‚     - MÃ©tadonnÃ©es complÃ¨tes ?                            â”‚ â”‚
â”‚  â”‚     - Identifiant unique ?                               â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  Si Ã©chec â†’ REJET avec erreur explicite                  â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚      â”‚                                                            â”‚
â”‚      â”‚ 3. Toutes validations passÃ©es                             â”‚
â”‚      â–¼                                                            â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚              ENREGISTREMENT                               â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  4. Enregistrement dans le registre                      â”‚ â”‚
â”‚  â”‚     - CapacitÃ© ajoutÃ©e au registre                       â”‚ â”‚
â”‚  â”‚     - MÃ©tadonnÃ©es stockÃ©es                               â”‚ â”‚
â”‚  â”‚     - Timestamp de crÃ©ation                              â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  5. TraÃ§abilitÃ©                                          â”‚ â”‚
â”‚  â”‚     - DÃ©claration enregistrÃ©e dans l'historique          â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚      â”‚                                                            â”‚
â”‚      â–¼                                                            â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚              RÃ‰SULTAT                                     â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  â€¢ SuccÃ¨s : Confirmation avec timestamp                  â”‚ â”‚
â”‚  â”‚  â€¢ La capacitÃ© est maintenant dans le registre officiel  â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚      â”‚                                                            â”‚
â”‚      â–¼                                                            â”‚
â”‚  MODULE (reÃ§oit la confirmation)                                 â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 14.3. Flux d'interrogation par StrongFather

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚           FLUX D'INTERROGATION PAR STRONGFATHER                  â”‚
â”‚                                                                   â”‚
â”‚  STRONGFATHER                                                     â”‚
â”‚      â”‚                                                            â”‚
â”‚      â”‚ 1. Ã‰valuation d'une intention                             â”‚
â”‚      â”‚    - Intention reÃ§ue de BondingBrother                    â”‚
â”‚      â”‚    - Besoin d'informations sur les capacitÃ©s              â”‚
â”‚      â”‚                                                            â”‚
â”‚      â–¼                                                            â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚              INTERROGATION 1 : EXISTENCE                  â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  "Cette capacitÃ© existe-t-elle ?"                        â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  â†’ capability_api.exists("content.create")               â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚      â”‚                                                            â”‚
â”‚      â”‚ RÃ©ponse : { exists: true, deprecated: false }             â”‚
â”‚      â–¼                                                            â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚              INTERROGATION 2 : PERMISSIONS                â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  "Quelles permissions sont requises ?"                   â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  â†’ capability_api.required_permissions("content.create") â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚      â”‚                                                            â”‚
â”‚      â”‚ RÃ©ponse : { required_permissions: ["content.write"] }     â”‚
â”‚      â–¼                                                            â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚              STRONGFATHER CONTINUE                        â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  StrongFather a maintenant les informations :            â”‚ â”‚
â”‚  â”‚  - La capacitÃ© existe                                    â”‚ â”‚
â”‚  â”‚  - Les permissions requises sont connues                 â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  â†’ StrongFather peut Ã©valuer l'intention                 â”‚ â”‚
â”‚  â”‚  â†’ StrongFather produit une DÃ‰CISION                     â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  NOTE : Master Butler n'a pas participÃ© Ã  la dÃ©cision    â”‚ â”‚
â”‚  â”‚         Il a fourni des informations, c'est tout         â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚      â”‚                                                            â”‚
â”‚      â–¼                                                            â”‚
â”‚  DÃ‰CISION DE STRONGFATHER (autorisÃ©/refusÃ©)                      â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 14.4. Principe de non-dÃ©cision

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚              PRINCIPE DE NON-DÃ‰CISION                            â”‚
â”‚                                                                   â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚                    MASTER BUTLER                          â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  CE QUE MASTER BUTLER FAIT :                             â”‚ â”‚
â”‚  â”‚  â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•                              â”‚ â”‚
â”‚  â”‚  âœ“ "Cette capacitÃ© existe"                               â”‚ â”‚
â”‚  â”‚  âœ“ "Ces permissions sont associÃ©es"                      â”‚ â”‚
â”‚  â”‚  âœ“ "Ce module dÃ©clare ces capacitÃ©s"                     â”‚ â”‚
â”‚  â”‚  âœ“ "Cette capacitÃ© est dÃ©prÃ©ciÃ©e"                        â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  CE QUE MASTER BUTLER NE FAIT JAMAIS :                   â”‚ â”‚
â”‚  â”‚  â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•                      â”‚ â”‚
â”‚  â”‚  âœ— "Cette action est autorisÃ©e"                          â”‚ â”‚
â”‚  â”‚  âœ— "L'utilisateur peut accÃ©der"                          â”‚ â”‚
â”‚  â”‚  âœ— "La permission est accordÃ©e"                          â”‚ â”‚
â”‚  â”‚  âœ— "L'intention est valide"                              â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚                                                                   â”‚
â”‚                        â•‘                                          â”‚
â”‚                        â•‘  SÃ‰PARATION STRICTE                     â”‚
â”‚                        â–¼                                          â”‚
â”‚                                                                   â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚                    STRONGFATHER                           â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  CE QUE STRONGFATHER FAIT :                              â”‚ â”‚
â”‚  â”‚  â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•                              â”‚ â”‚
â”‚  â”‚  âœ“ "Cette action est autorisÃ©e"                          â”‚ â”‚
â”‚  â”‚  âœ“ "Cette action est refusÃ©e"                            â”‚ â”‚
â”‚  â”‚  âœ“ "L'intention est validÃ©e"                             â”‚ â”‚
â”‚  â”‚  âœ“ "L'intention est rejetÃ©e"                             â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  StrongFather UTILISE les informations de Master Butler  â”‚ â”‚
â”‚  â”‚  pour PRENDRE ses dÃ©cisions                              â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚                                                                   â”‚
â”‚  RÃˆGLE ABSOLUE :                                                 â”‚
â”‚  â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•                                                 â”‚
â”‚  Master Butler INFORME, StrongFather DÃ‰CIDE                     â”‚
â”‚  Cette sÃ©paration est NON NÃ‰GOCIABLE                            â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 15. Conclusion contractuelle

Ce contrat Ã©tablit de maniÃ¨re dÃ©finitive et non nÃ©gociable la surface d'appel pour toutes les opÃ©rations relatives aux capacitÃ©s dans Master Butler.

Il garantit que :
- la Capability API est la surface d'appel dÃ©diÃ©e aux capacitÃ©s,
- les opÃ©rations de dÃ©claration, interrogation, et dÃ©couverte sont clairement dÃ©finies,
- le principe de non-dÃ©cision est respectÃ© absolument,
- les dÃ©clarations sont idempotentes et les identifiants immuables,
- la traÃ§abilitÃ© est complÃ¨te,
- les erreurs sont explicites et actionnables,
- le registre des capacitÃ©s reste cohÃ©rent et exhaustif.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisÃ©e.

---

**Document crÃ©Ã© le :** 2026-01-27  
**Version :** 1.0  
**Statut :** FONDATION â€” Contrat normatif validÃ©  
**RÃ©fÃ©rence :** Miyukini Core System v2.4, Master Butler Documentation Fondatrice, [Miyukini Conceptual References â€” Tools et Toolkits](..//..//..//..//miyukini-webway-system//reference//_index.md)  
**Type :** Contrat de surface d'appel non nÃ©gociable

---

## 16. Mini log â€” erreurs / warnings / ambiguÃ¯tÃ©s rencontrÃ©es et corrigÃ©es

### AmbiguÃ¯tÃ© A1 : Confusion possible entre information et dÃ©cision

**AmbiguÃ¯tÃ© rencontrÃ©e :** Risque de confondre les projections contextuelles (informations sur les capacitÃ©s accessibles dans un contexte) avec des dÃ©cisions d'autorisation.

**DÃ©cision prise :** Ajout de notes explicites dans les opÃ©rations de dÃ©couverte contextuelle prÃ©cisant que les rÃ©sultats sont des projections informationnelles, pas des dÃ©cisions. SchÃ©ma ASCII 14.4 dÃ©diÃ© au principe de non-dÃ©cision.

**Correction effectuÃ©e :** Section 5.3.3 et schÃ©ma 14.4 rÃ©digÃ©s avec clarification explicite.

### AmbiguÃ¯tÃ© A2 : Distinction entre dÃ©prÃ©ciation et suppression

**AmbiguÃ¯tÃ© rencontrÃ©e :** NÃ©cessitÃ© de clarifier que les capacitÃ©s ne peuvent pas Ãªtre supprimÃ©es physiquement.

**DÃ©cision prise :** Interdiction explicite INTERDIT-9 et invariant INV-CAP-6 ajoutÃ©s pour clarifier que les capacitÃ©s peuvent Ãªtre dÃ©prÃ©ciÃ©es mais jamais supprimÃ©es.

**Correction effectuÃ©e :** Sections 7.1 et 12.1 rÃ©digÃ©es avec rÃ¨gles explicites.

### AmbiguÃ¯tÃ© A3 : Relation avec la Permission API

**AmbiguÃ¯tÃ© rencontrÃ©e :** NÃ©cessitÃ© de clarifier l'interaction entre Capability API et Permission API.

**DÃ©cision prise :** Section 11.4 dÃ©diÃ©e Ã  l'interaction entre les deux APIs, prÃ©cisant que les permissions rÃ©fÃ©rencent des capacitÃ©s et que l'existence est validÃ©e lors de la dÃ©finition.

**Correction effectuÃ©e :** Section 11.4 rÃ©digÃ©e avec points d'interaction explicites.

### VÃ©rification de compatibilitÃ©

**VÃ©rification effectuÃ©e :** VÃ©rification systÃ©matique de la compatibilitÃ© avec la documentation fondatrice de Master Butler, le Capability Registry Contract, et les rÃ©fÃ©rences conceptuelles (Tools et Toolkits). Aucune contradiction dÃ©tectÃ©e.

**Conclusion :** Le contrat est strictement compatible avec le systÃ¨me contractuel existant. Il complÃ¨te les contrats existants en dÃ©finissant formellement la surface d'appel pour les capacitÃ©s.

---

*Aucune autre erreur, warning, ou ambiguÃ¯tÃ© rencontrÃ©e lors de la rÃ©daction de ce document.*

