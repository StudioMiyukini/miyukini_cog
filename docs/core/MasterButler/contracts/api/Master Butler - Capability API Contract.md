# Master Butler — Capability API Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **Master Butler — Capability API Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit la surface d'appel unique et autorisée pour toutes les opérations relatives aux capacités dans le système Miyukini.

Ce contrat précise les opérations de déclaration, d'interrogation et de découverte des capacités, les règles d'appel, les garanties offertes, et les interactions avec les autres composants du système.

### Portée

Ce contrat s'applique à **tous les composants** interagissant avec Master Butler pour les capacités et définit de manière absolue :
- la définition formelle de la Capability API et son rôle systémique,
- la typologie des opérations autorisées (déclaration, interrogation, découverte),
- les règles de contexte et de validation,
- ce que la Capability API PEUT et NE PEUT JAMAIS faire,
- les garanties offertes aux appelants,
- les invariants systémiques associés.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :
- **Master Butler — Documentation Fondatrice** : Définition fondamentale du rôle de Master Butler
- **Master Butler — Capability Registry Contract** : Modèle conceptuel du registre des capacités
- **Master Butler — Permission Registry Contract** : Modèle conceptuel du registre des permissions
- **[Miyukini Conceptual References — Lois Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Ce contrat respecte **LOI-1** (aucune dépendance externe critique) en garantissant que toutes les opérations fonctionnent sans appel externe obligatoire.

Il n'introduit aucune contradiction avec le corpus documentaire existant.

---

## 2. Rôle et nature de la Capability API

### Définition formelle

La **Capability API** est la surface d'appel conceptuelle qui constitue l'interface formelle entre les composants du système et Master Butler pour toutes les opérations relatives aux capacités. Elle représente l'ensemble des opérations que Master Butler expose pour déclarer, interroger, et découvrir les capacités du système.

### Caractéristiques formelles fondamentales

**Surface d'appel dédiée aux capacités :** La Capability API est la surface d'appel exclusive pour toutes les opérations relatives aux capacités. Toute interaction avec le registre des capacités DOIT passer par cette API.

**Interface conceptuelle :** La Capability API est une interface conceptuelle, pas une implémentation technique. Elle définit les opérations autorisées de manière abstraite, sans présupposer aucune technologie, aucun protocole, ou aucun format de données.

**Registre passif :** La Capability API expose un registre passif. Elle recense, documente, et fournit des informations sur les capacités, mais ne prend jamais de décision et n'exécute jamais d'action fonctionnelle.

**Médiation obligatoire :** Toute opération sur les capacités DOIT passer par la Capability API. Aucun accès direct au registre n'est autorisé.

**Abstraction de l'implémentation :** La Capability API abstrait complètement l'implémentation interne de Master Butler. Les appelants interagissent avec des concepts, pas avec des mécanismes techniques.

### Nature systémique

La Capability API est un **concept systémique**, pas une interface technique. Elle représente la frontière conceptuelle entre les appelants (modules, produits, cores) et le registre des capacités de Master Butler.

**Important :** Cette définition est purement conceptuelle et systémique. Elle ne présuppose aucune technologie, aucun langage de programmation, aucun protocole de communication, ou aucun format d'échange.

---

## 3. Principe de non-décision

### Énoncé formel

La Capability API **ne prend jamais de décision**. Elle fournit des informations sur les capacités, répond à des interrogations, mais ne produit jamais de verdict "autorisé" ou "refusé".

### Caractéristiques du principe de non-décision

**Information pure :** La Capability API retourne des informations, pas des décisions. Elle indique si une capacité existe, quelles sont ses métadonnées, quelles permissions sont associées, mais ne décide jamais si une action est autorisée.

**Pas de booléen d'autorisation :** Aucune méthode de la Capability API ne retourne un booléen d'autorisation. Les réponses sont des informations, pas des verdicts.

**Séparation stricte :** La décision d'autorisation appartient exclusivement à StrongFather. Master Butler fournit les informations nécessaires à cette décision.

### Non-négociabilités

- **NODEC-1 :** La Capability API ne retourne jamais "autorisé" ou "refusé"
- **NODEC-2 :** La Capability API ne participe jamais à une décision d'autorisation
- **NODEC-3 :** La Capability API fournit des informations, pas des verdicts
- **NODEC-4 :** Toute décision appartient exclusivement à StrongFather

---

## 4. Définition conceptuelle d'une opération Capability API

### Définition formelle

Une **opération Capability API** est une demande d'action conceptuelle formulée par un appelant à destination de Master Butler, relative aux capacités du système.

### Caractéristiques formelles d'une opération

**Demande d'information ou de modification du registre :** Une opération Capability API est soit une demande d'information (interrogation, découverte), soit une demande de modification du registre (déclaration, mise à jour, dépréciation).

**Contexte requis :** Chaque opération Capability API est accompagnée d'un contexte qui identifie l'appelant et son domaine d'origine.

**Atomicité conceptuelle :** Une opération Capability API est atomique conceptuellement. Elle est exécutée complètement ou pas du tout.

**Traçabilité obligatoire :** Chaque opération Capability API est tracée de manière complète, permettant l'audit et le debugging.

### Structure conceptuelle d'une opération

Conceptuellement, une opération Capability API comprend :
- **Type d'opération :** la catégorie de l'opération (déclaration, interrogation, découverte)
- **Paramètres :** les données nécessaires à l'exécution de l'opération
- **Contexte :** l'identité de l'appelant et son domaine
- **Résultat attendu :** le type de résultat que l'opération retourne

### Nature conceptuelle

Une opération Capability API est un **concept systémique**, pas un appel technique. Elle représente une demande d'action conceptuelle qui sera validée et exécutée par Master Butler.

---

## 5. Typologie des opérations autorisées

### 5.1. Opérations de déclaration

#### 5.1.1. Déclaration de capacité

**Définition formelle :**

Une **déclaration de capacité** est une opération Capability API par laquelle un composant (module, produit, adaptateur) informe Master Butler de l'existence d'une capacité qu'il possède.

**Caractéristiques :**

- **Acte fondateur :** La déclaration est l'acte par lequel une capacité entre dans le registre officiel
- **Idempotence :** Déclarer deux fois la même capacité n'a pas d'effet supplémentaire
- **Métadonnées obligatoires :** La déclaration inclut les métadonnées requises (identifiant, nom, description, module d'origine)
- **Validation de forme :** Master Butler valide la forme de la déclaration (identifiant unique, métadonnées complètes)
- **Pas de validation métier :** Master Butler ne valide pas la pertinence métier de la capacité

**Paramètres conceptuels :**

| Paramètre | Description | Obligation |
|-----------|-------------|------------|
| `capability_id` | Identifiant unique de la capacité | Obligatoire |
| `name` | Nom humain-lisible de la capacité | Obligatoire |
| `description` | Description de la capacité | Obligatoire |
| `module_origin` | Module déclarant la capacité | Obligatoire |
| `version` | Version de la capacité | Optionnel |
| `metadata` | Métadonnées additionnelles | Optionnel |

**Résultat conceptuel :**

- Succès : Confirmation d'enregistrement avec timestamp
- Erreur : Indication de l'erreur de forme (identifiant dupliqué, métadonnées manquantes)

#### 5.1.2. Mise à jour de capacité

**Définition formelle :**

Une **mise à jour de capacité** est une opération Capability API par laquelle un composant modifie les métadonnées d'une capacité existante.

**Caractéristiques :**

- **Capacité existante :** Seule une capacité existante peut être mise à jour
- **Immutabilité de l'identifiant :** L'identifiant de la capacité ne peut jamais être modifié
- **Module d'origine requis :** Seul le module d'origine peut mettre à jour la capacité
- **Traçabilité :** La mise à jour est tracée avec l'état avant/après

**Paramètres conceptuels :**

| Paramètre | Description | Obligation |
|-----------|-------------|------------|
| `capability_id` | Identifiant de la capacité à mettre à jour | Obligatoire |
| `module_origin` | Module demandant la mise à jour | Obligatoire |
| `updates` | Modifications à appliquer | Obligatoire |

**Résultat conceptuel :**

- Succès : Confirmation de mise à jour avec nouveau state
- Erreur : Indication de l'erreur (capacité inexistante, module non autorisé)

#### 5.1.3. Dépréciation de capacité

**Définition formelle :**

Une **dépréciation de capacité** est une opération Capability API par laquelle un composant marque une capacité comme obsolète.

**Caractéristiques :**

- **Marquage soft :** La dépréciation ne supprime pas la capacité, elle la marque comme obsolète
- **Information de remplacement :** La dépréciation peut indiquer une capacité de remplacement
- **Traçabilité :** La dépréciation est tracée avec la raison et la date
- **Module d'origine requis :** Seul le module d'origine peut déprécier la capacité

**Paramètres conceptuels :**

| Paramètre | Description | Obligation |
|-----------|-------------|------------|
| `capability_id` | Identifiant de la capacité à déprécier | Obligatoire |
| `module_origin` | Module demandant la dépréciation | Obligatoire |
| `reason` | Raison de la dépréciation | Obligatoire |
| `replacement_id` | Identifiant de la capacité de remplacement | Optionnel |
| `deprecation_date` | Date effective de dépréciation | Optionnel |

**Résultat conceptuel :**

- Succès : Confirmation de dépréciation avec nouveau state
- Erreur : Indication de l'erreur (capacité inexistante, module non autorisé)

### 5.2. Opérations d'interrogation

#### 5.2.1. Vérification d'existence

**Définition formelle :**

Une **vérification d'existence** est une opération Capability API qui détermine si une capacité existe dans le registre.

**Caractéristiques :**

- **Réponse booléenne :** L'opération retourne vrai si la capacité existe, faux sinon
- **Pas de décision :** La réponse est une information, pas une autorisation
- **Inclusion des dépréciées :** Par défaut, les capacités dépréciées sont considérées comme existantes (avec un flag)

**Paramètres conceptuels :**

| Paramètre | Description | Obligation |
|-----------|-------------|------------|
| `capability_id` | Identifiant de la capacité à vérifier | Obligatoire |
| `include_deprecated` | Inclure les capacités dépréciées | Optionnel (défaut: true) |

**Résultat conceptuel :**

```
{
  exists: boolean,
  deprecated: boolean | null,
  deprecation_info: { reason, replacement_id, date } | null
}
```

#### 5.2.2. Récupération de capacité

**Définition formelle :**

Une **récupération de capacité** est une opération Capability API qui retourne les informations complètes d'une capacité.

**Caractéristiques :**

- **Informations complètes :** Toutes les métadonnées de la capacité sont retournées
- **Capacité existante requise :** La capacité doit exister dans le registre
- **Inclusion des associations :** Les permissions associées peuvent être incluses

**Paramètres conceptuels :**

| Paramètre | Description | Obligation |
|-----------|-------------|------------|
| `capability_id` | Identifiant de la capacité à récupérer | Obligatoire |
| `include_permissions` | Inclure les permissions associées | Optionnel (défaut: false) |
| `include_history` | Inclure l'historique des modifications | Optionnel (défaut: false) |

**Résultat conceptuel :**

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

#### 5.2.3. Listage des capacités

**Définition formelle :**

Un **listage des capacités** est une opération Capability API qui retourne une liste de capacités selon des critères de filtrage.

**Caractéristiques :**

- **Filtrage :** Les capacités peuvent être filtrées par module, par état, par métadonnées
- **Pagination :** Les résultats peuvent être paginés pour les grands registres
- **Tri :** Les résultats peuvent être triés par différents critères

**Paramètres conceptuels :**

| Paramètre | Description | Obligation |
|-----------|-------------|------------|
| `filter_module` | Filtrer par module d'origine | Optionnel |
| `filter_deprecated` | Filtrer par état de dépréciation | Optionnel |
| `filter_metadata` | Filtrer par métadonnées | Optionnel |
| `pagination` | Paramètres de pagination | Optionnel |
| `sort` | Critère de tri | Optionnel |

**Résultat conceptuel :**

```
{
  capabilities: array,
  total_count: number,
  pagination_info: object | null
}
```

#### 5.2.4. Récupération des permissions requises

**Définition formelle :**

Une **récupération des permissions requises** est une opération Capability API qui retourne les permissions associées à une capacité.

**Caractéristiques :**

- **Information pour décideurs :** Cette opération est principalement utilisée par StrongFather pour connaître les permissions requises
- **Liste complète :** Toutes les permissions associées sont retournées
- **Pas de décision :** L'opération retourne des informations, pas un verdict

**Paramètres conceptuels :**

| Paramètre | Description | Obligation |
|-----------|-------------|------------|
| `capability_id` | Identifiant de la capacité | Obligatoire |

**Résultat conceptuel :**

```
{
  capability_id: string,
  required_permissions: array,
  permission_details: array
}
```

### 5.3. Opérations de découverte

#### 5.3.1. Découverte par module

**Définition formelle :**

Une **découverte par module** est une opération Capability API qui retourne toutes les capacités déclarées par un module spécifique.

**Caractéristiques :**

- **Scope module :** La découverte est limitée à un module spécifique
- **Informations complètes :** Chaque capacité est retournée avec ses métadonnées
- **Exclusion optionnelle des dépréciées :** Les capacités dépréciées peuvent être exclues

**Paramètres conceptuels :**

| Paramètre | Description | Obligation |
|-----------|-------------|------------|
| `module_id` | Identifiant du module | Obligatoire |
| `include_deprecated` | Inclure les capacités dépréciées | Optionnel (défaut: false) |

**Résultat conceptuel :**

```
{
  module_id: string,
  capabilities: array,
  total_count: number
}
```

#### 5.3.2. Découverte par type d'action

**Définition formelle :**

Une **découverte par type d'action** est une opération Capability API qui retourne toutes les capacités correspondant à un type d'action (create, read, update, delete, etc.).

**Caractéristiques :**

- **Scope action :** La découverte est basée sur le type d'action
- **Pattern matching :** Les capacités sont filtrées par pattern d'identifiant (ex: `*.create`, `content.*`)
- **Cross-module :** La découverte traverse tous les modules

**Paramètres conceptuels :**

| Paramètre | Description | Obligation |
|-----------|-------------|------------|
| `action_pattern` | Pattern de type d'action | Obligatoire |
| `include_deprecated` | Inclure les capacités dépréciées | Optionnel (défaut: false) |

**Résultat conceptuel :**

```
{
  action_pattern: string,
  capabilities: array,
  total_count: number
}
```

#### 5.3.3. Découverte contextuelle

**Définition formelle :**

Une **découverte contextuelle** est une opération Capability API qui retourne les capacités accessibles dans un contexte donné (rôle, permissions détenues).

**Caractéristiques :**

- **Scope contexte :** La découverte est filtrée par le contexte fourni
- **Projection :** Retourne une projection des capacités accessibles, pas une décision d'autorisation
- **Information préparatoire :** Utilisée pour préparer les informations avant une décision de StrongFather

**Paramètres conceptuels :**

| Paramètre | Description | Obligation |
|-----------|-------------|------------|
| `context_roles` | Rôles du contexte | Obligatoire |
| `context_permissions` | Permissions du contexte | Optionnel |
| `target_module` | Module cible (optionnel) | Optionnel |

**Résultat conceptuel :**

```
{
  context_summary: object,
  accessible_capabilities: array,
  total_count: number,
  note: "This is a projection, not an authorization decision"
}
```

**IMPORTANT :** Cette opération retourne une **projection informationnelle**, pas une décision d'autorisation. La décision finale appartient à StrongFather.

#### 5.3.4. Recherche de capacités

**Définition formelle :**

Une **recherche de capacités** est une opération Capability API qui permet de rechercher des capacités par mots-clés dans les métadonnées.

**Caractéristiques :**

- **Recherche textuelle :** La recherche porte sur les noms, descriptions, et métadonnées
- **Résultats pondérés :** Les résultats peuvent être triés par pertinence
- **Cross-module :** La recherche traverse tous les modules

**Paramètres conceptuels :**

| Paramètre | Description | Obligation |
|-----------|-------------|------------|
| `query` | Mots-clés de recherche | Obligatoire |
| `search_fields` | Champs à rechercher | Optionnel (défaut: tous) |
| `include_deprecated` | Inclure les capacités dépréciées | Optionnel (défaut: false) |
| `limit` | Nombre maximum de résultats | Optionnel |

**Résultat conceptuel :**

```
{
  query: string,
  results: array,
  total_count: number
}
```

---

## 6. Ce que la Capability API PEUT faire

### 6.1. Opérations autorisées

La Capability API PEUT effectuer les opérations suivantes :

**PEUT-1 : Enregistrer des déclarations de capacités**

La Capability API PEUT enregistrer des déclarations de capacités provenant de modules, produits, ou adaptateurs, sous réserve que la déclaration soit valide formellement.

**PEUT-2 : Mettre à jour des capacités existantes**

La Capability API PEUT mettre à jour les métadonnées de capacités existantes, sous réserve que le module d'origine autorise la modification.

**PEUT-3 : Marquer des capacités comme dépréciées**

La Capability API PEUT marquer des capacités comme dépréciées, sous réserve que le module d'origine autorise la dépréciation.

**PEUT-4 : Répondre aux interrogations**

La Capability API PEUT répondre à toutes les interrogations sur les capacités : existence, métadonnées, permissions associées.

**PEUT-5 : Permettre la découverte**

La Capability API PEUT permettre la découverte des capacités par module, par type d'action, par contexte, ou par recherche.

**PEUT-6 : Fournir des projections contextuelles**

La Capability API PEUT fournir des projections de capacités accessibles dans un contexte donné, à titre informatif.

**PEUT-7 : Tracer toutes les opérations**

La Capability API PEUT et DOIT tracer toutes les opérations pour permettre l'audit.

**PEUT-8 : Retourner des erreurs explicites**

La Capability API PEUT retourner des erreurs explicites et actionnables lorsqu'une opération ne peut pas être exécutée.

### 6.2. Garanties associées

Chaque opération autorisée est accompagnée des garanties suivantes :
- Validation de forme avant exécution
- Idempotence des déclarations
- Traçabilité complète
- Erreur explicite en cas de rejet
- Cohérence préservée après exécution

---

## 7. Ce que la Capability API NE PEUT JAMAIS faire

### 7.1. Interdictions absolues

La Capability API NE PEUT JAMAIS effectuer les actions suivantes. Ces interdictions sont absolues et non négociables.

**INTERDIT-1 : Prendre des décisions d'autorisation**

La Capability API NE PEUT JAMAIS retourner une décision d'autorisation. Elle fournit des informations, pas des verdicts. La décision appartient exclusivement à StrongFather.

**INTERDIT-2 : Vérifier des permissions en temps réel**

La Capability API NE PEUT JAMAIS vérifier si un utilisateur ou contexte possède effectivement une permission au moment d'une action. Cette vérification appartient à StrongFather.

**INTERDIT-3 : Exécuter des actions fonctionnelles**

La Capability API NE PEUT JAMAIS exécuter d'action fonctionnelle. Elle ne crée pas de contenu, ne modifie pas de données métier, ne téléverse pas de fichiers. Elle gère uniquement les métadonnées de capacités.

**INTERDIT-4 : Stocker des données métier**

La Capability API NE PEUT JAMAIS stocker de données métier. Elle stocke uniquement des métadonnées : définitions de capacités, associations, historiques.

**INTERDIT-5 : Gérer les identités**

La Capability API NE PEUT JAMAIS gérer les identités des utilisateurs ou des systèmes. Elle connaît les associations rôles-permissions-capacités, mais pas les identités.

**INTERDIT-6 : Définir des politiques**

La Capability API NE PEUT JAMAIS définir de politiques de décision. Les politiques appartiennent à StrongFather.

**INTERDIT-7 : Appliquer des contraintes métier**

La Capability API NE PEUT JAMAIS appliquer de contraintes métier. Elle définit ce qui existe, pas comment l'utiliser.

**INTERDIT-8 : Modifier l'identifiant d'une capacité**

La Capability API NE PEUT JAMAIS modifier l'identifiant d'une capacité existante. Les identifiants sont immuables.

**INTERDIT-9 : Supprimer physiquement une capacité**

La Capability API NE PEUT JAMAIS supprimer physiquement une capacité du registre. Les capacités peuvent être dépréciées, jamais supprimées.

**INTERDIT-10 : Contourner la traçabilité**

La Capability API NE PEUT JAMAIS effectuer une opération sans traçabilité. Toute opération est enregistrée.

### 7.2. Justifications

Ces interdictions sont justifiées par :
- le respect du principe de non-décision de Master Butler,
- la séparation stricte des responsabilités entre cores,
- la préservation de l'intégrité du registre,
- le maintien de la traçabilité complète,
- le respect de l'architecture Miyukini.

---

## 8. Règles absolues d'appel (préconditions)

### 8.1. Préconditions obligatoires

Chaque appel Capability API DOIT respecter les préconditions suivantes.

**PRECOND-1 : Identifiant d'appelant obligatoire**

Chaque appel Capability API DOIT identifier l'appelant (module, core, produit). Les appels anonymes sont rejetés.

**PRECOND-2 : Format d'identifiant valide (pour déclarations)**

Pour les opérations de déclaration, l'identifiant de capacité DOIT respecter le format canonique : `module.action` ou `module.domain.action`.

**PRECOND-3 : Métadonnées complètes (pour déclarations)**

Pour les opérations de déclaration, les métadonnées obligatoires DOIVENT être fournies (identifiant, nom, description, module d'origine).

**PRECOND-4 : Capacité existante (pour interrogations spécifiques)**

Pour les opérations d'interrogation spécifique (get, permissions), la capacité DOIT exister dans le registre.

**PRECOND-5 : Module d'origine correspondant (pour modifications)**

Pour les opérations de modification (update, deprecate), le module appelant DOIT être le module d'origine de la capacité.

**PRECOND-6 : Appel légal**

L'opération demandée DOIT être une opération légale et documentée de la Capability API.

### 8.2. Règles de validation des préconditions

- Les préconditions sont validées dans l'ordre
- Si une précondition échoue, l'appel est rejeté immédiatement
- L'erreur de rejet indique la précondition non satisfaite
- Aucune exécution partielle n'est autorisée après un échec de précondition

---

## 9. Règles absolues de rejet

### 9.1. Conditions de rejet

Un appel Capability API est rejeté si l'une des conditions suivantes est détectée :

**REJET-1 : Appelant non identifié**

L'appel est rejeté si l'appelant n'est pas identifié.
- Erreur retournée : `CALLER_NOT_IDENTIFIED`
- Traçabilité : tentative tracée

**REJET-2 : Format d'identifiant invalide**

L'appel est rejeté si l'identifiant de capacité ne respecte pas le format canonique.
- Erreur retournée : `INVALID_CAPABILITY_ID_FORMAT`
- Traçabilité : erreur de format tracée

**REJET-3 : Métadonnées incomplètes**

L'appel est rejeté si les métadonnées obligatoires sont manquantes.
- Erreur retournée : `INCOMPLETE_METADATA`
- Traçabilité : erreur de métadonnées tracée

**REJET-4 : Capacité inexistante (pour interrogations)**

L'appel est rejeté si la capacité demandée n'existe pas dans le registre.
- Erreur retournée : `CAPABILITY_NOT_FOUND`
- Traçabilité : tentative tracée

**REJET-5 : Module non autorisé (pour modifications)**

L'appel est rejeté si le module appelant n'est pas le module d'origine de la capacité.
- Erreur retournée : `MODULE_NOT_AUTHORIZED`
- Traçabilité : tentative non autorisée tracée

**REJET-6 : Identifiant dupliqué (pour déclarations)**

L'appel est rejeté si l'identifiant de capacité existe déjà (sauf si idempotence s'applique).
- Erreur retournée : `CAPABILITY_ID_EXISTS`
- Traçabilité : duplication tracée

**REJET-7 : Opération illégale**

L'appel est rejeté si l'opération demandée n'est pas une opération légale de la Capability API.
- Erreur retournée : `ILLEGAL_OPERATION`
- Traçabilité : tentative tracée

### 9.2. Garanties après rejet

Après tout rejet, les garanties suivantes s'appliquent :
- L'état du registre reste inchangé
- Aucune modification partielle n'est appliquée
- L'erreur est explicite et actionnable
- Le rejet est tracé pour audit

### 9.3. Règles absolues

- **R-REJ-1 :** Tout rejet laisse l'état inchangé
- **R-REJ-2 :** Tout rejet retourne une erreur explicite
- **R-REJ-3 :** Tout rejet est tracé
- **R-REJ-4 :** Aucune exception au rejet n'est autorisée

---

## 10. Garanties offertes aux appelants

### 10.1. Garanties de traitement

**G-CAP-1 : Traitement prévisible des opérations valides**

Si un appelant fournit un contexte valide et effectue des appels légaux, Master Butler traite les opérations de manière prévisible et conforme au contrat.

**G-CAP-2 : Messages d'erreur explicites et actionnables**

Si une opération est rejetée, Master Butler retourne toujours un message d'erreur explicite et actionnable qui permet à l'appelant de comprendre et corriger le problème.

**G-CAP-3 : Pas de rejet arbitraire**

Master Butler ne rejette jamais une opération de manière arbitraire. Tout rejet est justifié par une violation de précondition ou une condition de rejet documentée.

**G-CAP-4 : Idempotence des déclarations**

Les déclarations de capacités sont idempotentes. Déclarer deux fois la même capacité avec les mêmes métadonnées n'a pas d'effet supplémentaire.

### 10.2. Garanties de cohérence

**G-CAP-5 : Cohérence du registre**

Après toute opération réussie, le registre reste cohérent et conforme aux contraintes structurelles.

**G-CAP-6 : État inchangé après rejet**

Après tout rejet, l'état du registre reste inchangé.

**G-CAP-7 : Immutabilité des identifiants**

Les identifiants de capacités sont immuables. Une capacité déclarée garde son identifiant à jamais.

### 10.3. Garanties de traçabilité

**G-CAP-8 : Traçabilité complète**

Toutes les opérations sont tracées de manière complète, permettant l'audit.

**G-CAP-9 : Historique des modifications**

L'historique des modifications de chaque capacité est conservé et accessible.

### 10.4. Garanties de disponibilité

**G-CAP-10 : Registre local**

Le registre des capacités est local. Toutes les opérations fonctionnent sans dépendance externe.

Cette garantie respecte **LOI-1** (aucune dépendance externe critique) : le registre fonctionne localement sans nécessiter d'appels externes.

### 10.5. Non-négociabilité

Ces garanties sont absolues et non négociables. Elles s'appliquent à tous les appelants, sans exception.

---

## 11. Interactions avec les autres composants

### 11.1. Interaction avec StrongFather

**Relation formelle :**

StrongFather interroge la Capability API pour obtenir les informations nécessaires à ses décisions d'autorisation.

**Points d'interaction :**

- **Vérification d'existence :** StrongFather vérifie si une capacité existe avant d'évaluer une intention
- **Permissions requises :** StrongFather récupère les permissions associées à une capacité
- **Contexte de capacité :** StrongFather peut demander une projection contextuelle

**Règles :**

- StrongFather est toujours autorisé à interroger la Capability API
- Les réponses sont exhaustives et exactes
- La Capability API ne suggère pas de décision

### 11.2. Interaction avec BondingBrother

**Relation formelle :**

BondingBrother interroge la Capability API lors de la traduction des intentions.

**Points d'interaction :**

- **Vérification d'existence :** BondingBrother vérifie si une capacité existe dans un module
- **Découverte :** BondingBrother peut découvrir les capacités d'un module cible
- **Permissions requises :** BondingBrother peut récupérer les permissions associées pour préparer le contexte

**Règles :**

- BondingBrother peut interroger la Capability API pour la traduction
- Les informations préparent le contexte pour StrongFather
- BondingBrother ne prend pas de décision basée sur ces informations

### 11.3. Interaction avec les modules et produits

**Relation formelle :**

Les modules et produits utilisent la Capability API pour déclarer leurs capacités et découvrir les capacités des autres.

**Points d'interaction :**

- **Déclaration :** Les modules déclarent leurs capacités au démarrage
- **Découverte :** Les modules découvrent les capacités disponibles
- **Mise à jour :** Les modules mettent à jour leurs capacités si nécessaire
- **Dépréciation :** Les modules déprécient leurs anciennes capacités

**Règles :**

- Les modules ne peuvent modifier que leurs propres capacités
- La déclaration est obligatoire pour toute capacité exposée
- La découverte est accessible à tous les composants autorisés

### 11.4. Interaction avec Permission API

**Relation formelle :**

La Capability API et la Permission API sont complémentaires et interagissent via les associations.

**Points d'interaction :**

- **Associations :** Les permissions référencent des capacités via la Capability API
- **Validation :** Lors de la définition d'une permission, l'existence des capacités référencées est validée

**Règles :**

- Une permission ne peut pas référencer une capacité inexistante
- La dépréciation d'une capacité n'invalide pas automatiquement les permissions associées (warning)

---

## 12. Invariants systémiques liés à la Capability API

### 12.1. Invariants globaux

**INV-CAP-1 : Exhaustivité du registre**

Le registre de Master Butler est exhaustif. Toute capacité existant dans le système est recensée. Si une capacité n'est pas dans le registre, elle n'existe pas officiellement.

**INV-CAP-2 : Non-décision**

La Capability API ne prend jamais de décision. Elle fournit des informations, pas des verdicts.

**INV-CAP-3 : Idempotence des déclarations**

Les déclarations de capacités sont idempotentes. Déclarer deux fois la même capacité n'a pas d'effet supplémentaire.

**INV-CAP-4 : Immutabilité des identifiants**

Les identifiants de capacités sont immuables. Une fois déclarés, ils ne changent jamais.

**INV-CAP-5 : Traçabilité complète**

Toute opération est tracée. Aucune opération sans traçabilité n'est possible.

**INV-CAP-6 : Pas de suppression physique**

Les capacités ne sont jamais supprimées physiquement. Elles peuvent être dépréciées, jamais effacées.

### 12.2. Invariants de déclaration

**INV-DECL-1 : Format canonique**

Tout identifiant de capacité respecte le format canonique.

**INV-DECL-2 : Module d'origine requis**

Toute capacité a un module d'origine identifié.

**INV-DECL-3 : Métadonnées complètes**

Toute capacité a des métadonnées complètes (nom, description).

### 12.3. Invariants d'interrogation

**INV-INT-1 : Réponse complète**

Toute interrogation retourne une réponse complète, jamais partielle.

**INV-INT-2 : Cohérence temporelle**

Les données retournées reflètent l'état du registre au moment de l'interrogation.

---

## 13. Cas explicitement hors périmètre

### 13.1. Ce que la Capability API n'inclut PAS

Les éléments suivants sont **explicitement hors du périmètre** de la Capability API :

**HORS-1 : Détails d'implémentation**

La Capability API ne définit pas les détails d'implémentation techniques (langages, protocoles, formats de données). Elle est purement conceptuelle.

**HORS-2 : Décisions d'autorisation**

La Capability API ne définit pas et n'exécute pas de décisions d'autorisation. Les décisions appartiennent à StrongFather.

**HORS-3 : Gestion des permissions**

La gestion des permissions (définition, association, attribution) appartient à la Permission API, pas à la Capability API.

**HORS-4 : Exécution des capacités**

L'exécution des capacités appartient aux modules et produits, pas à Master Butler.

**HORS-5 : Logique métier**

La Capability API ne définit pas la logique métier des capacités. Elle recense ce qui existe, pas ce que cela fait concrètement.

**HORS-6 : Cycle de vie technique des Tools**

Le cycle de vie technique des Tools (versions, compatibilité) appartient à Ever Buddy, pas à Master Butler.

### 13.2. Justification

Ces éléments sont hors périmètre car :
- la Capability API est une abstraction conceptuelle, pas une implémentation technique,
- la séparation des responsabilités entre cores est stricte,
- Master Butler recense, mais ne décide pas et n'exécute pas.

---

## 14. Schémas ASCII

### 14.1. Position de la Capability API dans l'architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    APPELANTS                                     │
│                                                                   │
│  ┌───────────────┐  ┌───────────────┐  ┌───────────────────┐  │
│  │  StrongFather │  │ BondingBrother│  │  Modules/Produits │  │
│  │  (décideur)   │  │  (médiateur)  │  │  (déclarants)     │  │
│  └───────────────┘  └───────────────┘  └───────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                            │
                            │ Appels Capability API
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                    CAPABILITY API                                 │
│                    (Master Butler)                                │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  OPÉRATIONS AUTORISÉES :                                  │ │
│  │                                                            │ │
│  │  DÉCLARATION          INTERROGATION        DÉCOUVERTE     │ │
│  │  • declare            • exists             • by_module    │ │
│  │  • update             • get                • by_action    │ │
│  │  • deprecate          • list               • contextual   │ │
│  │                       • permissions        • search       │ │
│  │                                                            │ │
│  │  PRINCIPES :                                              │ │
│  │  ✓ Information pure (pas de décision)                     │ │
│  │  ✓ Idempotence des déclarations                          │ │
│  │  ✓ Immutabilité des identifiants                         │ │
│  │  ✓ Traçabilité complète                                  │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                            │
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                    REGISTRE DES CAPACITÉS                        │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  • Inventaire exhaustif des capacités                     │ │
│  │  • Métadonnées de chaque capacité                        │ │
│  │  • Associations avec permissions                          │ │
│  │  • Historique des modifications                           │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### 14.2. Flux de déclaration de capacité

```
┌─────────────────────────────────────────────────────────────────┐
│              FLUX DE DÉCLARATION DE CAPACITÉ                     │
│                                                                   │
│  MODULE                                                           │
│      │                                                            │
│      │ 1. Préparation de la déclaration                          │
│      │    - Identifiant unique (format canonique)                │
│      │    - Métadonnées complètes                                │
│      │                                                            │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              CAPABILITY API                               │ │
│  │                                                            │ │
│  │  2. Validation des préconditions                         │ │
│  │     - Appelant identifié ?                               │ │
│  │     - Format d'identifiant valide ?                      │ │
│  │     - Métadonnées complètes ?                            │ │
│  │     - Identifiant unique ?                               │ │
│  │                                                            │ │
│  │  Si échec → REJET avec erreur explicite                  │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      │ 3. Toutes validations passées                             │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              ENREGISTREMENT                               │ │
│  │                                                            │ │
│  │  4. Enregistrement dans le registre                      │ │
│  │     - Capacité ajoutée au registre                       │ │
│  │     - Métadonnées stockées                               │ │
│  │     - Timestamp de création                              │ │
│  │                                                            │ │
│  │  5. Traçabilité                                          │ │
│  │     - Déclaration enregistrée dans l'historique          │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              RÉSULTAT                                     │ │
│  │                                                            │ │
│  │  • Succès : Confirmation avec timestamp                  │ │
│  │  • La capacité est maintenant dans le registre officiel  │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      ▼                                                            │
│  MODULE (reçoit la confirmation)                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 14.3. Flux d'interrogation par StrongFather

```
┌─────────────────────────────────────────────────────────────────┐
│           FLUX D'INTERROGATION PAR STRONGFATHER                  │
│                                                                   │
│  STRONGFATHER                                                     │
│      │                                                            │
│      │ 1. Évaluation d'une intention                             │
│      │    - Intention reçue de BondingBrother                    │
│      │    - Besoin d'informations sur les capacités              │
│      │                                                            │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              INTERROGATION 1 : EXISTENCE                  │ │
│  │                                                            │ │
│  │  "Cette capacité existe-t-elle ?"                        │ │
│  │                                                            │ │
│  │  → capability_api.exists("content.create")               │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      │ Réponse : { exists: true, deprecated: false }             │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              INTERROGATION 2 : PERMISSIONS                │ │
│  │                                                            │ │
│  │  "Quelles permissions sont requises ?"                   │ │
│  │                                                            │ │
│  │  → capability_api.required_permissions("content.create") │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      │ Réponse : { required_permissions: ["content.write"] }     │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              STRONGFATHER CONTINUE                        │ │
│  │                                                            │ │
│  │  StrongFather a maintenant les informations :            │ │
│  │  - La capacité existe                                    │ │
│  │  - Les permissions requises sont connues                 │ │
│  │                                                            │ │
│  │  → StrongFather peut évaluer l'intention                 │ │
│  │  → StrongFather produit une DÉCISION                     │ │
│  │                                                            │ │
│  │  NOTE : Master Butler n'a pas participé à la décision    │ │
│  │         Il a fourni des informations, c'est tout         │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      ▼                                                            │
│  DÉCISION DE STRONGFATHER (autorisé/refusé)                      │
└─────────────────────────────────────────────────────────────────┘
```

### 14.4. Principe de non-décision

```
┌─────────────────────────────────────────────────────────────────┐
│              PRINCIPE DE NON-DÉCISION                            │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │                    MASTER BUTLER                          │ │
│  │                                                            │ │
│  │  CE QUE MASTER BUTLER FAIT :                             │ │
│  │  ══════════════════════════                              │ │
│  │  ✓ "Cette capacité existe"                               │ │
│  │  ✓ "Ces permissions sont associées"                      │ │
│  │  ✓ "Ce module déclare ces capacités"                     │ │
│  │  ✓ "Cette capacité est dépréciée"                        │ │
│  │                                                            │ │
│  │  CE QUE MASTER BUTLER NE FAIT JAMAIS :                   │ │
│  │  ══════════════════════════════════                      │ │
│  │  ✗ "Cette action est autorisée"                          │ │
│  │  ✗ "L'utilisateur peut accéder"                          │ │
│  │  ✗ "La permission est accordée"                          │ │
│  │  ✗ "L'intention est valide"                              │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                   │
│                        ║                                          │
│                        ║  SÉPARATION STRICTE                     │
│                        ▼                                          │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │                    STRONGFATHER                           │ │
│  │                                                            │ │
│  │  CE QUE STRONGFATHER FAIT :                              │ │
│  │  ══════════════════════════                              │ │
│  │  ✓ "Cette action est autorisée"                          │ │
│  │  ✓ "Cette action est refusée"                            │ │
│  │  ✓ "L'intention est validée"                             │ │
│  │  ✓ "L'intention est rejetée"                             │ │
│  │                                                            │ │
│  │  StrongFather UTILISE les informations de Master Butler  │ │
│  │  pour PRENDRE ses décisions                              │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                   │
│  RÈGLE ABSOLUE :                                                 │
│  ═══════════════                                                 │
│  Master Butler INFORME, StrongFather DÉCIDE                     │
│  Cette séparation est NON NÉGOCIABLE                            │
└─────────────────────────────────────────────────────────────────┘
```

---

## 15. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable la surface d'appel pour toutes les opérations relatives aux capacités dans Master Butler.

Il garantit que :
- la Capability API est la surface d'appel dédiée aux capacités,
- les opérations de déclaration, interrogation, et découverte sont clairement définies,
- le principe de non-décision est respecté absolument,
- les déclarations sont idempotentes et les identifiants immuables,
- la traçabilité est complète,
- les erreurs sont explicites et actionnables,
- le registre des capacités reste cohérent et exhaustif.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

**Document créé le :** 2026-01-27  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, Master Butler Documentation Fondatrice, [Miyukini Conceptual References — Tools et Toolkits](../../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)  
**Type :** Contrat de surface d'appel non négociable

---

## 16. Mini log — erreurs / warnings / ambiguïtés rencontrées et corrigées

### Ambiguïté A1 : Confusion possible entre information et décision

**Ambiguïté rencontrée :** Risque de confondre les projections contextuelles (informations sur les capacités accessibles dans un contexte) avec des décisions d'autorisation.

**Décision prise :** Ajout de notes explicites dans les opérations de découverte contextuelle précisant que les résultats sont des projections informationnelles, pas des décisions. Schéma ASCII 14.4 dédié au principe de non-décision.

**Correction effectuée :** Section 5.3.3 et schéma 14.4 rédigés avec clarification explicite.

### Ambiguïté A2 : Distinction entre dépréciation et suppression

**Ambiguïté rencontrée :** Nécessité de clarifier que les capacités ne peuvent pas être supprimées physiquement.

**Décision prise :** Interdiction explicite INTERDIT-9 et invariant INV-CAP-6 ajoutés pour clarifier que les capacités peuvent être dépréciées mais jamais supprimées.

**Correction effectuée :** Sections 7.1 et 12.1 rédigées avec règles explicites.

### Ambiguïté A3 : Relation avec la Permission API

**Ambiguïté rencontrée :** Nécessité de clarifier l'interaction entre Capability API et Permission API.

**Décision prise :** Section 11.4 dédiée à l'interaction entre les deux APIs, précisant que les permissions référencent des capacités et que l'existence est validée lors de la définition.

**Correction effectuée :** Section 11.4 rédigée avec points d'interaction explicites.

### Vérification de compatibilité

**Vérification effectuée :** Vérification systématique de la compatibilité avec la documentation fondatrice de Master Butler, le Capability Registry Contract, et les références conceptuelles (Tools et Toolkits). Aucune contradiction détectée.

**Conclusion :** Le contrat est strictement compatible avec le système contractuel existant. Il complète les contrats existants en définissant formellement la surface d'appel pour les capacités.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
