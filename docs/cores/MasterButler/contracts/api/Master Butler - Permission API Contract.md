# Master Butler — Permission API Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **Master Butler Permission API Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit la surface d'appel conceptuelle pour la définition, l'interrogation, l'association et la gestion des permissions dans le système Miyukini Core System v2.4.

Ce contrat précise les opérations autorisées sur les permissions, les règles d'appel, les contextes requis, les garanties offertes, et les interdictions absolues liées à la gestion des permissions.

### Portée

Ce contrat s'applique à **tous les composants** interagissant avec le registre des permissions de Master Butler et définit de manière absolue :

- La définition formelle de la Permission API et son rôle systémique
- La typologie conceptuelle des opérations autorisées
- Les règles d'appel et préconditions obligatoires
- Ce que la Permission API PEUT et NE PEUT JAMAIS faire
- Les garanties offertes aux appelants conformes
- Les règles de rejet et comportements en cas d'erreur
- Les invariants systémiques associés

Ce contrat se concentre exclusivement sur la **surface d'appel** pour la gestion des permissions, sans entrer dans les détails d'implémentation technique.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :

- **Master Butler — Documentation Fondatrice** : Définit la raison d'être et les responsabilités de Master Butler
- **Master Butler — Permission Registry Contract** : Définit le modèle de données du registre des permissions (complémentaire)
- **Master Butler — Capability API Contract** : Définit la surface d'appel pour les capacités (parallèle)
- **Master Butler — Association Model Contract** : Définit les associations entre permissions, rôles et capacités
- **[Miyukini Conceptual References — Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)** : Définitions canoniques des termes
- **[Miyukini Conceptual References — Lois Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Ce contrat respecte **LOI-1** (aucune dépendance externe critique) en garantissant que toutes les opérations fonctionnent sans appel externe obligatoire

**Complémentarité :**

- Permission Registry Contract = le **modèle de données** des permissions
- Permission API Contract = la **surface d'appel** pour interagir avec les permissions

---

## 2. Rôle et nature de la Permission API

### Définition formelle

La **Permission API** est la surface d'appel conceptuelle qui constitue l'interface formelle pour toutes les opérations liées aux permissions dans Master Butler. Elle représente l'ensemble des opérations conceptuelles exposées pour définir, interroger, modifier et gérer les permissions du système.

### Caractéristiques formelles fondamentales

**Surface d'appel dédiée :** La Permission API est la surface d'appel unique pour toutes les opérations sur les permissions. Aucune autre méthode d'interaction avec les permissions n'est autorisée.

**Interface conceptuelle :** La Permission API est une interface conceptuelle, pas une implémentation technique. Elle définit les opérations autorisées de manière abstraite, sans présupposer aucune technologie, aucun protocole, ou aucun format de données.

**Médiation obligatoire :** Toute opération sur les permissions DOIT passer par la Permission API. Aucun accès direct au registre des permissions n'est autorisé.

**Abstraction de l'implémentation :** La Permission API abstrait complètement l'implémentation interne du registre. Les appelants interagissent avec des concepts, pas avec des mécanismes techniques.

### Nature systémique

La Permission API est un **concept systémique**, pas une interface technique. Elle représente la frontière conceptuelle entre les appelants (Opérateurs, StrongFather, BondingBrother) et le registre des permissions de Master Butler.

**Important :** Cette définition est purement conceptuelle et systémique. Elle ne présuppose aucune technologie, aucun langage de programmation, aucun protocole de communication, ou aucun format d'échange.

---

## 3. Principes fondamentaux

### Principe d'unicité

La Permission API constitue l'**unique surface d'appel** pour les opérations sur les permissions.

| Caractéristique | Description |
|-----------------|-------------|
| **Unicité** | Il n'existe qu'une seule Permission API |
| **Exclusivité** | Toute opération sur les permissions DOIT passer par cette API |
| **Non-contournabilité** | La Permission API ne peut pas être contournée |
| **Centralisation** | Tout contrôle et validation sont centralisés |

### Principe de séparation

La Permission API respecte la séparation fondamentale entre :

| Responsabilité | Propriétaire | Ce que fait la Permission API |
|----------------|--------------|-------------------------------|
| **Définition des permissions** | Master Butler | ✅ Permet de définir |
| **Attribution des permissions** | Mécanismes d'attribution | ❌ Ne gère pas |
| **Vérification des permissions** | StrongFather | ❌ Ne vérifie jamais |

**Règle absolue :**

> **La Permission API définit ce qui existe comme droits possibles, jamais ce qui est effectivement autorisé.**

### Principe de non-décision

La Permission API **ne prend jamais de décision d'autorisation**. Elle fournit les informations sur les permissions définies, mais ne répond jamais "autorisé" ou "refusé" pour une action.

---

## 4. Définition conceptuelle d'une opération Permission API

### Définition formelle

Une **opération Permission API** est une demande d'action conceptuelle formulée par un appelant à destination du registre des permissions de Master Butler, accompagnée d'un contexte, et soumise à validation avant exécution.

### Caractéristiques formelles d'une opération

**Demande d'action :** Une opération Permission API est une demande d'action sur les permissions (définition, interrogation, modification, dépréciation).

**Contexte requis :** Chaque opération Permission API est accompagnée d'un contexte qui inclut :
- L'identité de l'appelant
- Le niveau d'autorité de l'appelant
- Le type d'opération demandée
- Les paramètres de l'opération

**Soumission à validation :** Chaque opération Permission API est soumise à validation avant exécution.

**Atomicité conceptuelle :** Une opération Permission API est atomique conceptuellement. Elle est exécutée complètement ou pas du tout.

**Traçabilité obligatoire :** Chaque opération Permission API est tracée de manière complète.

### Structure conceptuelle d'une opération

Conceptuellement, une opération Permission API comprend :
- **Type d'opération :** la catégorie de l'opération
- **Paramètres :** les données nécessaires à l'exécution
- **Contexte :** les informations contextuelles requises
- **Résultat attendu :** le type de résultat retourné

---

## 5. Typologie des opérations autorisées

### 5.1. Opérations de définition

#### Créer une permission

**Opération :** `definePermission`

**Description :** Crée une nouvelle permission dans le registre.

**Paramètres requis :**

| Paramètre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `id` | String | Identifiant unique de la permission | ✅ |
| `name` | String | Nom lisible de la permission | ✅ |
| `description` | String | Description détaillée | ✅ |
| `domain` | String | Domaine fonctionnel | ✅ |
| `level` | Enum | Niveau de criticité | ✅ |
| `scope_type` | Enum | Type de portée | ✅ |
| `capabilities` | Array[String] | Capacités couvertes | ✅ |
| `implied_permissions` | Array[String] | Permissions impliquées | ❌ |
| `required_permissions` | Array[String] | Permissions prérequises | ❌ |

**Préconditions :**
- L'appelant doit avoir l'autorité de définir des permissions
- L'identifiant ne doit pas déjà exister
- Toutes les capacités référencées doivent exister
- Toutes les permissions impliquées/requises doivent exister
- Le niveau de criticité doit être autorisé pour l'appelant

**Résultat :**
- Succès : Permission créée en état DRAFT, identifiant confirmé
- Échec : Erreur explicite avec raison

#### Activer une permission

**Opération :** `activatePermission`

**Description :** Active une permission en état DRAFT.

**Paramètres requis :**

| Paramètre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Identifiant de la permission | ✅ |

**Préconditions :**
- La permission doit exister
- La permission doit être en état DRAFT
- L'appelant doit avoir l'autorité d'activer
- Toutes les capacités référencées doivent être ACTIVE
- Toutes les permissions impliquées doivent être ACTIVE

**Résultat :**
- Succès : Permission passée en état ACTIVE
- Échec : Erreur explicite avec raison

#### Modifier une permission

**Opération :** `updatePermission`

**Description :** Modifie une permission existante.

**Paramètres requis :**

| Paramètre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Identifiant de la permission | ✅ |
| `updates` | Object | Champs à modifier | ✅ |

**Règles de modification :**

| Champ | Modifiable en DRAFT | Modifiable en ACTIVE |
|-------|---------------------|----------------------|
| `id` | ❌ Non | ❌ Non |
| `name` | ✅ Oui | ⚠️ Avec version |
| `description` | ✅ Oui | ⚠️ Avec version |
| `capabilities` | ✅ Oui | ⚠️ Avec version |
| `implied_permissions` | ✅ Oui | ⚠️ Avec version |
| `level` | ✅ Oui | ❌ Non |
| `scope_type` | ✅ Oui | ❌ Non |

**Résultat :**
- Succès : Permission mise à jour, nouvelle version si applicable
- Échec : Erreur explicite avec raison

#### Déprécier une permission

**Opération :** `deprecatePermission`

**Description :** Marque une permission comme dépréciée.

**Paramètres requis :**

| Paramètre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Identifiant de la permission | ✅ |
| `reason` | String | Raison de la dépréciation | ✅ |
| `successor_id` | String | Permission de remplacement | Recommandé |
| `migration_guide` | String | Guide de migration | Recommandé |

**Préconditions :**
- La permission doit exister
- La permission doit être en état ACTIVE
- L'appelant doit avoir l'autorité de déprécier

**Résultat :**
- Succès : Permission passée en état DEPRECATED
- Échec : Erreur explicite avec raison

#### Retirer une permission

**Opération :** `retirePermission`

**Description :** Retire définitivement une permission du système.

**Paramètres requis :**

| Paramètre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Identifiant de la permission | ✅ |

**Préconditions :**
- La permission doit exister
- La permission doit être en état DEPRECATED
- La période de dépréciation minimale doit être écoulée
- L'appelant doit avoir l'autorité de retirer

**Résultat :**
- Succès : Permission passée en état RETIRED, archivée
- Échec : Erreur explicite avec raison

---

### 5.2. Opérations d'interrogation

#### Obtenir une permission

**Opération :** `getPermission`

**Description :** Récupère la définition complète d'une permission.

**Paramètres requis :**

| Paramètre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Identifiant de la permission | ✅ |

**Résultat :**
- Succès : Définition complète de la permission
- Échec : Erreur si permission inexistante

#### Lister les permissions

**Opération :** `listPermissions`

**Description :** Liste les permissions selon des critères de filtrage.

**Paramètres optionnels :**

| Paramètre | Type | Description |
|-----------|------|-------------|
| `domain` | String | Filtrer par domaine |
| `level` | Enum[] | Filtrer par niveaux |
| `status` | Enum[] | Filtrer par états |
| `scope_type` | Enum[] | Filtrer par types de portée |
| `capability_id` | String | Filtrer par capacité couverte |
| `offset` | Integer | Décalage pour pagination |
| `limit` | Integer | Nombre maximum de résultats |

**Résultat :**
- Liste des permissions correspondant aux critères
- Métadonnées de pagination

#### Rechercher des permissions

**Opération :** `searchPermissions`

**Description :** Recherche des permissions par texte libre.

**Paramètres requis :**

| Paramètre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `query` | String | Texte de recherche | ✅ |
| `filters` | Object | Filtres additionnels | ❌ |

**Résultat :**
- Liste des permissions correspondant à la recherche
- Score de pertinence pour chaque résultat

#### Obtenir les capacités d'une permission

**Opération :** `getPermissionCapabilities`

**Description :** Récupère les capacités couvertes par une permission.

**Paramètres requis :**

| Paramètre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Identifiant de la permission | ✅ |
| `include_implied` | Boolean | Inclure les capacités des permissions impliquées | ❌ (défaut: false) |

**Résultat :**
- Liste des capacités directement associées
- Si `include_implied` : union de toutes les capacités effectives

#### Obtenir la hiérarchie d'une permission

**Opération :** `getPermissionHierarchy`

**Description :** Récupère l'arbre des implications d'une permission.

**Paramètres requis :**

| Paramètre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Identifiant de la permission | ✅ |
| `direction` | Enum | `UP` (qui implique cette permission) ou `DOWN` (permissions impliquées) | ❌ (défaut: DOWN) |

**Résultat :**
- Arbre des implications dans la direction demandée
- Profondeur de chaque niveau

---

### 5.3. Opérations d'association

#### Associer une capacité

**Opération :** `associateCapability`

**Description :** Ajoute une capacité à une permission.

**Paramètres requis :**

| Paramètre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Identifiant de la permission | ✅ |
| `capability_id` | String | Identifiant de la capacité | ✅ |

**Préconditions :**
- La permission doit exister
- La capacité doit exister
- La permission doit être en état DRAFT ou ACTIVE
- L'appelant doit avoir l'autorité de modifier

**Résultat :**
- Succès : Association créée
- Échec : Erreur explicite avec raison

#### Dissocier une capacité

**Opération :** `dissociateCapability`

**Description :** Retire une capacité d'une permission.

**Paramètres requis :**

| Paramètre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Identifiant de la permission | ✅ |
| `capability_id` | String | Identifiant de la capacité | ✅ |

**Préconditions :**
- La permission doit exister
- L'association doit exister
- La permission doit conserver au moins une capacité
- La permission doit être en état DRAFT ou ACTIVE

**Résultat :**
- Succès : Association retirée
- Échec : Erreur explicite avec raison

#### Ajouter une implication

**Opération :** `addImplication`

**Description :** Ajoute une permission impliquée.

**Paramètres requis :**

| Paramètre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Permission parente | ✅ |
| `implied_permission_id` | String | Permission impliquée | ✅ |

**Préconditions :**
- Les deux permissions doivent exister
- L'ajout ne doit pas créer de cycle
- La permission parente doit être en état DRAFT ou ACTIVE

**Résultat :**
- Succès : Implication ajoutée
- Échec : Erreur explicite (notamment si cycle détecté)

#### Retirer une implication

**Opération :** `removeImplication`

**Description :** Retire une permission impliquée.

**Paramètres requis :**

| Paramètre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Permission parente | ✅ |
| `implied_permission_id` | String | Permission impliquée à retirer | ✅ |

**Préconditions :**
- Les deux permissions doivent exister
- L'implication doit exister
- La permission parente doit être en état DRAFT ou ACTIVE

**Résultat :**
- Succès : Implication retirée
- Échec : Erreur explicite avec raison

---

### 5.4. Opérations de validation

#### Valider une définition

**Opération :** `validatePermissionDefinition`

**Description :** Valide une définition de permission sans l'enregistrer.

**Paramètres requis :**

| Paramètre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `definition` | Object | Définition à valider | ✅ |

**Résultat :**
- Succès : Définition valide
- Échec : Liste des erreurs de validation

#### Vérifier les cycles

**Opération :** `checkCycles`

**Description :** Vérifie si l'ajout d'une implication créerait un cycle.

**Paramètres requis :**

| Paramètre | Type | Description | Obligatoire |
|-----------|------|-------------|-------------|
| `permission_id` | String | Permission parente | ✅ |
| `implied_permission_id` | String | Permission à impliquer | ✅ |

**Résultat :**
- Succès : Pas de cycle détecté
- Échec : Cycle détecté avec chemin explicite

---

## 6. Ce que la Permission API PEUT faire

### 6.1. Opérations autorisées

**PEUT-PERM-1 : Définir des permissions**

La Permission API PEUT créer de nouvelles permissions dans le registre, sous réserve que le contexte soit valide et que l'appelant ait l'autorité requise.

**PEUT-PERM-2 : Interroger les permissions**

La Permission API PEUT retourner les définitions des permissions, les lister, les rechercher, et fournir leurs métadonnées.

**PEUT-PERM-3 : Modifier les permissions**

La Permission API PEUT modifier les permissions selon les règles de modification définies dans le Permission Registry Contract.

**PEUT-PERM-4 : Gérer le cycle de vie**

La Permission API PEUT faire transiter les permissions entre les états du cycle de vie (DRAFT → ACTIVE → DEPRECATED → RETIRED).

**PEUT-PERM-5 : Gérer les associations**

La Permission API PEUT créer et supprimer les associations entre permissions et capacités, et entre permissions (implications).

**PEUT-PERM-6 : Valider les définitions**

La Permission API PEUT valider les définitions de permissions avant enregistrement et détecter les cycles d'implication.

**PEUT-PERM-7 : Retourner des erreurs explicites**

La Permission API PEUT retourner des erreurs explicites et actionnables lorsqu'une opération ne peut pas être exécutée.

### 6.2. Garanties associées

Chaque opération autorisée est accompagnée des garanties suivantes :
- Validation complète avant exécution
- Atomicité de l'opération
- Traçabilité complète
- Erreur explicite en cas de rejet
- Intégrité du registre préservée

---

## 7. Ce que la Permission API NE PEUT JAMAIS faire

### 7.1. Interdictions absolues

**INTERDIT-PERM-1 : Décider d'une autorisation**

La Permission API NE PEUT JAMAIS décider si une permission est accordée ou refusée à un contexte donné. Elle définit les permissions, elle ne vérifie pas leur attribution.

**INTERDIT-PERM-2 : Vérifier les permissions en temps réel**

La Permission API NE PEUT JAMAIS vérifier si un utilisateur ou un contexte possède effectivement une permission au moment d'une action. Cette vérification appartient à StrongFather.

**INTERDIT-PERM-3 : Retourner un verdict d'autorisation**

La Permission API NE PEUT JAMAIS retourner "autorisé" ou "refusé" comme résultat d'une opération. Elle retourne des définitions, pas des décisions.

**INTERDIT-PERM-4 : Créer une permission sans capacité**

La Permission API NE PEUT JAMAIS créer ou activer une permission qui ne référence aucune capacité existante.

**INTERDIT-PERM-5 : Créer des cycles d'implication**

La Permission API NE PEUT JAMAIS créer une implication qui formerait un cycle (direct ou indirect).

**INTERDIT-PERM-6 : Contourner les états du cycle de vie**

La Permission API NE PEUT JAMAIS permettre une transition d'état non autorisée (ex: ACTIVE → DRAFT, RETIRED → ACTIVE).

**INTERDIT-PERM-7 : Modifier l'identifiant**

La Permission API NE PEUT JAMAIS modifier l'identifiant d'une permission après sa création.

**INTERDIT-PERM-8 : Supprimer sans dépréciation**

La Permission API NE PEUT JAMAIS retirer une permission ACTIVE sans passer par l'état DEPRECATED (sauf pour les permissions DRAFT).

**INTERDIT-PERM-9 : Exposer les attributions**

La Permission API NE PEUT JAMAIS exposer qui possède quelle permission. Ces informations appartiennent aux mécanismes d'attribution et à StrongFather.

**INTERDIT-PERM-10 : Appliquer des règles métier**

La Permission API NE PEUT JAMAIS appliquer des règles métier sur l'usage des permissions. Elle définit les droits, pas leur contexte d'application.

### 7.2. Justifications

Ces interdictions sont justifiées par :
- La préservation de la séparation entre définition et décision
- Le respect de l'autorité de StrongFather pour les décisions
- La garantie de l'intégrité du registre
- L'absence de logique métier dans Master Butler
- Le principe de non-vérification de Master Butler

---

## 8. Règles absolues d'appel (préconditions)

### 8.1. Préconditions obligatoires

Chaque appel Permission API DOIT respecter les préconditions suivantes. Si une précondition n'est pas satisfaite, l'appel est rejeté immédiatement.

**PRECOND-PERM-1 : Identité de l'appelant**

Chaque appel DOIT être accompagné de l'identité de l'appelant, permettant de vérifier son autorité.

**PRECOND-PERM-2 : Autorité suffisante**

L'appelant DOIT avoir l'autorité nécessaire pour l'opération demandée :
- Définition de permission STANDARD : Opérateurs autorisés
- Définition de permission ELEVATED : Opérateurs avec autorité élevée
- Définition de permission CRITICAL : StrongFather avec validation
- Définition de permission SYSTEM : MiyukiniAdmin uniquement

**PRECOND-PERM-3 : Paramètres valides**

Tous les paramètres obligatoires DOIVENT être fournis et valides.

**PRECOND-PERM-4 : Références existantes**

Toutes les références (capacités, permissions impliquées) DOIVENT exister dans les registres respectifs.

**PRECOND-PERM-5 : Cohérence des états**

Les opérations DOIVENT être cohérentes avec l'état actuel des permissions concernées.

### 8.2. Règles de validation

- Les préconditions sont validées dans l'ordre
- Si une précondition échoue, l'appel est rejeté immédiatement
- L'erreur de rejet indique la précondition non satisfaite
- Aucune exécution partielle n'est autorisée après un échec

---

## 9. Règles absolues de rejet

### 9.1. Conditions de rejet

Un appel Permission API est rejeté si l'une des conditions suivantes est détectée :

**REJET-PERM-1 : Appelant non identifié**

L'appel est rejeté si l'identité de l'appelant n'est pas fournie ou invalide.
- Erreur : `UNKNOWN_CALLER`
- Action : Aucune modification

**REJET-PERM-2 : Autorité insuffisante**

L'appel est rejeté si l'appelant n'a pas l'autorité pour l'opération.
- Erreur : `INSUFFICIENT_AUTHORITY`
- Action : Aucune modification, tentative tracée

**REJET-PERM-3 : Permission inexistante**

L'appel est rejeté si la permission référencée n'existe pas.
- Erreur : `PERMISSION_NOT_FOUND`
- Action : Aucune modification

**REJET-PERM-4 : Identifiant dupliqué**

L'appel est rejeté si l'identifiant existe déjà lors d'une création.
- Erreur : `DUPLICATE_PERMISSION_ID`
- Action : Aucune modification

**REJET-PERM-5 : Capacité inexistante**

L'appel est rejeté si une capacité référencée n'existe pas.
- Erreur : `CAPABILITY_NOT_FOUND`
- Action : Aucune modification

**REJET-PERM-6 : Cycle détecté**

L'appel est rejeté si l'opération créerait un cycle d'implication.
- Erreur : `CYCLIC_IMPLICATION_DETECTED`
- Action : Aucune modification, chemin du cycle retourné

**REJET-PERM-7 : Transition d'état invalide**

L'appel est rejeté si la transition d'état demandée n'est pas autorisée.
- Erreur : `INVALID_STATE_TRANSITION`
- Action : Aucune modification

**REJET-PERM-8 : Modification interdite**

L'appel est rejeté si la modification demandée n'est pas autorisée pour l'état actuel.
- Erreur : `MODIFICATION_NOT_ALLOWED`
- Action : Aucune modification

**REJET-PERM-9 : Dernière capacité**

L'appel est rejeté si la dissociation laisserait la permission sans capacité.
- Erreur : `LAST_CAPABILITY_REMOVAL`
- Action : Aucune modification

**REJET-PERM-10 : Période de dépréciation**

L'appel est rejeté si le retrait est demandé avant la fin de la période de dépréciation.
- Erreur : `DEPRECATION_PERIOD_NOT_ELAPSED`
- Action : Aucune modification

### 9.2. Garanties après rejet

Après tout rejet, les garanties suivantes s'appliquent :
- L'état du registre reste inchangé
- Aucune modification partielle n'est appliquée
- L'erreur est explicite et actionnable
- La tentative est tracée pour audit
- Aucun effet de bord n'est créé

### 9.3. Règles absolues

- **R-REJ-PERM-1 :** Tout rejet laisse le registre inchangé
- **R-REJ-PERM-2 :** Tout rejet retourne une erreur explicite
- **R-REJ-PERM-3 :** Tout rejet est tracé
- **R-REJ-PERM-4 :** Aucune exception au rejet n'est autorisée

---

## 10. Garanties offertes aux appelants conformes

### 10.1. Garanties de traitement

**G-PERM-API-1 : Traitement prévisible**

Si un appelant autorisé fournit des paramètres valides et respecte les préconditions, Master Butler traite l'opération de manière prévisible et conforme au contrat.

**G-PERM-API-2 : Messages d'erreur explicites**

Si une opération est rejetée, Master Butler retourne toujours un message d'erreur explicite et actionnable.

**G-PERM-API-3 : Pas de rejet arbitraire**

Master Butler ne rejette jamais une opération de manière arbitraire. Tout rejet est justifié par une violation documentée.

**G-PERM-API-4 : Atomicité**

Toute opération Permission API est atomique. Elle est exécutée complètement ou pas du tout.

### 10.2. Garanties de cohérence

**G-PERM-API-5 : Intégrité référentielle**

Après toute opération réussie, l'intégrité référentielle du registre est garantie.

**G-PERM-API-6 : État inchangé après rejet**

Après tout rejet, l'état du registre reste inchangé.

**G-PERM-API-7 : Absence de cycle**

Après toute opération réussie, le registre ne contient aucun cycle d'implication.

### 10.3. Garanties de traçabilité

**G-PERM-API-8 : Traçabilité complète**

Toutes les opérations sont tracées de manière complète (qui, quand, quoi, résultat).

**G-PERM-API-9 : Historique préservé**

L'historique des modifications est préservé, y compris pour les permissions retirées.

### 10.4. Non-négociabilité

Ces garanties sont absolues et non négociables. Elles s'appliquent à tous les appelants conformes, sans exception.

---

## 11. Contexte requis pour les opérations

### 11.1. Structure du contexte

Chaque opération Permission API est accompagnée d'un contexte structuré :

```yaml
context:
  caller:
    id: <identifiant de l'appelant>
    type: <OPERATOR | CORE | SYSTEM>
    authority_level: <niveau d'autorité>
  operation:
    type: <type d'opération>
    timestamp: <timestamp de l'appel>
    request_id: <identifiant unique de requête>
  trace:
    correlation_id: <identifiant de corrélation>
    source: <composant source>
```

### 11.2. Contexte par type d'appelant

#### Opérateur

| Champ | Description | Requis |
|-------|-------------|--------|
| `operator_id` | Identifiant de l'Opérateur | ✅ |
| `authority_level` | Niveau d'autorité | ✅ |
| `session_id` | Identifiant de session | Recommandé |

#### Core (StrongFather, BondingBrother)

| Champ | Description | Requis |
|-------|-------------|--------|
| `core_id` | Identifiant du Core | ✅ |
| `operation_context` | Contexte de l'opération parente | ✅ |

#### System (MiyukiniAdmin)

| Champ | Description | Requis |
|-------|-------------|--------|
| `admin_id` | Identifiant administrateur | ✅ |
| `authorization_proof` | Preuve d'autorisation | ✅ |

---

## 12. Interaction avec les autres composants

### 12.1. Interaction avec StrongFather

**Flux typique d'interrogation :**

```
StrongFather évalue une intention
    │
    ├── Interroge Permission API : "Quelles permissions couvrent cette capacité ?"
    │       │
    │       └── Permission API retourne : Liste des permissions
    │
    ├── Interroge Permission API : "Quelle est la définition de cette permission ?"
    │       │
    │       └── Permission API retourne : Définition complète
    │
    └── StrongFather décide selon les politiques
```

**Règles d'interaction :**
- StrongFather est toujours autorisé à interroger
- La Permission API ne suggère jamais de décision
- Les réponses sont exhaustives et exactes

### 12.2. Interaction avec BondingBrother

**Flux typique :**

```
BondingBrother traduit une intention
    │
    ├── Interroge Permission API : "Quelles permissions sont requises pour cette action ?"
    │       │
    │       └── Permission API retourne : Permissions requises
    │
    └── BondingBrother enrichit le contexte de l'intention
```

**Règles d'interaction :**
- BondingBrother interroge pour la traduction, pas pour la décision
- Les réponses aident à construire le contexte

### 12.3. Interaction avec les Opérateurs

**Flux de définition :**

```
Opérateur définit une nouvelle permission
    │
    ├── Soumet via BondingBrother
    │       │
    │       └── Permission API valide et enregistre
    │
    └── Confirmation de l'enregistrement
```

**Flux de découverte :**

```
Opérateur découvre les permissions
    │
    ├── Interroge Permission API
    │       │
    │       └── Permission API retourne les permissions (selon autorité)
    │
    └── Opérateur utilise ces informations
```

### 12.4. Interaction avec le Capability Registry

**Dépendance :**

```
Permission API
    │
    └── Vérifie les capacités référencées dans Capability Registry
            │
            └── Capability Registry confirme l'existence
```

**Règles :**
- Toute capacité référencée DOIT exister
- La suppression d'une capacité invalide les permissions associées

---

## 13. Invariants systémiques

### INV-PERM-API-1 : Non-décision

La Permission API **ne prend jamais de décision d'autorisation**. Aucune méthode ne retourne "autorisé" ou "refusé".

### INV-PERM-API-2 : Atomicité

Toute opération Permission API est **atomique**. Elle est exécutée complètement ou pas du tout.

### INV-PERM-API-3 : Traçabilité

Toute opération Permission API est **tracée** avec contexte complet.

### INV-PERM-API-4 : Intégrité référentielle

La Permission API **préserve l'intégrité référentielle** du registre. Aucune référence invalide n'est créée.

### INV-PERM-API-5 : Absence de cycle

La Permission API **garantit l'absence de cycle** dans les implications.

### INV-PERM-API-6 : Association obligatoire

La Permission API **garantit qu'une permission active a au moins une capacité**.

### INV-PERM-API-7 : Immutabilité des identifiants

La Permission API **ne modifie jamais** un identifiant de permission après création.

### INV-PERM-API-8 : Transitions d'état valides

La Permission API **n'autorise que les transitions d'état valides** du cycle de vie.

---

## 14. Schémas ASCII

### 14.1. Position de la Permission API dans l'architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    APPELANTS                                      │
│                                                                   │
│  ┌───────────────┐  ┌───────────────┐  ┌───────────────────┐   │
│  │   Opérateurs  │  │  StrongFather │  │  BondingBrother   │   │
│  │               │  │   (décision)  │  │    (médiation)    │   │
│  └───────────────┘  └───────────────┘  └───────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                            │
                            │ Appels Permission API
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                    PERMISSION API                                 │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  OPÉRATIONS AUTORISÉES :                                  │ │
│  │                                                            │ │
│  │  DÉFINITION        INTERROGATION      ASSOCIATION         │ │
│  │  ───────────       ─────────────      ───────────         │ │
│  │  • definePermission   • getPermission   • associateCapability  │
│  │  • activatePermission • listPermissions • dissociateCapability │
│  │  • updatePermission   • searchPermissions • addImplication     │
│  │  • deprecatePermission• getCapabilities  • removeImplication   │
│  │  • retirePermission   • getHierarchy                          │
│  │                                                            │ │
│  │  VALIDATION                                                │ │
│  │  ──────────                                                │ │
│  │  • validateDefinition • checkCycles                       │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                   │
│  PRINCIPES :                                                      │
│  ✓ Surface d'appel unique pour les permissions                  │
│  ✓ Contexte obligatoire                                         │
│  ✓ Validation avant exécution                                   │
│  ✓ Atomicité des opérations                                     │
│  ✓ Traçabilité complète                                         │
│  ✓ JAMAIS de décision d'autorisation                            │
└─────────────────────────────────────────────────────────────────┘
                            │
                            │ Accède au
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│              PERMISSION REGISTRY (Registre)                       │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  • Stockage des permissions                               │ │
│  │  • Associations permission ↔ capacité                    │ │
│  │  • Hiérarchie d'implications                              │ │
│  │  • Historique des modifications                           │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### 14.2. Flux de définition d'une permission

```
┌─────────────────────────────────────────────────────────────────┐
│              FLUX DE DÉFINITION D'UNE PERMISSION                  │
│                                                                   │
│  APPELANT (Opérateur)                                            │
│      │                                                            │
│      │ 1. Soumet définition de permission                        │
│      │    • id, name, description                                │
│      │    • domain, level, scope_type                            │
│      │    • capabilities[]                                       │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              PERMISSION API                               │ │
│  │                                                            │ │
│  │  2. Validation des préconditions                          │ │
│  │     ├── Appelant identifié ?         ──→ Rejet si non    │ │
│  │     ├── Autorité suffisante ?        ──→ Rejet si non    │ │
│  │     └── Paramètres valides ?         ──→ Rejet si non    │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      │ 3. Validation de la définition                            │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  Identifiant unique ?                ──→ Rejet si non     │ │
│  │  Capacités existent toutes ?         ──→ Rejet si non     │ │
│  │  Permissions impliquées existent ?   ──→ Rejet si non     │ │
│  │  Pas de cycle d'implication ?        ──→ Rejet si cycle   │ │
│  │  Niveau autorisé pour l'appelant ?   ──→ Rejet si non     │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      │ 4. Toutes validations passées                             │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              ENREGISTREMENT                               │ │
│  │                                                            │ │
│  │  • Permission créée en état DRAFT                         │ │
│  │  • Associations créées                                    │ │
│  │  • Traçabilité enregistrée                               │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      │ 5. Retour du résultat                                     │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  SUCCÈS                              ÉCHEC                 │ │
│  │  ───────                             ─────                 │ │
│  │  • permission_id confirmé            • Erreur explicite    │ │
│  │  • état: DRAFT                       • Raison détaillée    │ │
│  │  • version: 1.0.0                    • Registre inchangé   │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      ▼                                                            │
│  APPELANT (reçoit le résultat)                                   │
└─────────────────────────────────────────────────────────────────┘
```

### 14.3. Ce que la Permission API fait vs ne fait pas

```
┌─────────────────────────────────────────────────────────────────┐
│     PERMISSION API : CE QU'ELLE FAIT VS NE FAIT PAS              │
│                                                                   │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  CE QUE LA PERMISSION API FAIT                           │   │
│  │  ═══════════════════════════════                         │   │
│  │                                                           │   │
│  │  ✓ Définit des permissions                               │   │
│  │  ✓ Interroge le registre                                 │   │
│  │  ✓ Gère les associations permission ↔ capacité          │   │
│  │  ✓ Gère les hiérarchies d'implication                   │   │
│  │  ✓ Valide les définitions                                │   │
│  │  ✓ Détecte les cycles                                    │   │
│  │  ✓ Trace toutes les opérations                           │   │
│  │  ✓ Retourne des erreurs explicites                       │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                   │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  CE QUE LA PERMISSION API NE FAIT JAMAIS                 │   │
│  │  ═══════════════════════════════════════                 │   │
│  │                                                           │   │
│  │  ✗ Décider si une permission est accordée                │   │
│  │  ✗ Vérifier les permissions en temps réel                │   │
│  │  ✗ Retourner "autorisé" ou "refusé"                      │   │
│  │  ✗ Connaître qui possède quelle permission               │   │
│  │  ✗ Appliquer des règles métier                           │   │
│  │  ✗ Exécuter des actions fonctionnelles                   │   │
│  │  ✗ Stocker des données métier                            │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                   │
│  PHRASE FONDAMENTALE :                                            │
│  ═════════════════════                                            │
│                                                                   │
│  "La Permission API définit ce qui existe comme droits           │
│   possibles, jamais ce qui est effectivement autorisé."          │
│                                                                   │
│  La DÉFINITION appartient à Master Butler.                        │
│  La DÉCISION appartient à StrongFather.                           │
└─────────────────────────────────────────────────────────────────┘
```

---

## 15. Conformité aux Lois d'Autonomie Système

Ce contrat respecte les Lois d'Autonomie Système définies dans [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md).

### LOI-1 : Aucune dépendance externe critique à l'exécution

**Conformité :** Conforme

La Permission API opère entièrement en local :

- **Opérations locales** : Toutes les opérations s'exécutent localement
- **Registre local** : Le registre des permissions est local
- **Aucune API externe** : Aucun service distant n'est requis

**Vérification LOI-1** : *"La Permission API fonctionne-t-elle si le réseau est indisponible ?"* → **Oui.**

### LOI-5 : Le coût doit être proportionnel au hardware

**Conformité :** Conforme

La Permission API a une empreinte minimale :

- **Opérations légères** : Lecture et écriture de métadonnées
- **Pas de workers** : Aucun processus en arrière-plan
- **Mémoire prévisible** : Proportionnelle au nombre de permissions

**Vérification LOI-5** : *"La Permission API fonctionne-t-elle sur un Raspberry Pi 4 ?"* → **Oui.**

### Synthèse de conformité

| Loi | Statut | Raison |
|-----|--------|--------|
| LOI-1 | ✅ Conforme | Opérations locales, aucune dépendance externe |
| LOI-5 | ✅ Conforme | Métadonnées légères, consommation minimale |

---

## 16. Conclusion contractuelle

### Essence de la Permission API

La Permission API de Master Butler est la **surface d'appel unique** pour toutes les opérations liées aux permissions dans le système Miyukini. Elle permet de définir, interroger, associer et gérer les permissions, sans jamais participer à la décision d'autorisation.

### Phrase fondatrice

> **La Permission API définit la surface d'appel pour gérer les droits possibles du système Miyukini, en garantissant l'intégrité du registre, sans jamais décider de ce qui est effectivement autorisé.**

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

Toute implémentation de la Permission API doit respecter intégralement ce document. Toute évolution doit préserver les invariants définis ici.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** FONDATION — Non négociable  
**Référence :** Miyukini Core System v2.4

**Références croisées :**

- [Master Butler - Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md) : Définition et responsabilités de Master Butler
- [Master Butler - Permission Registry Contract](../registry/Master%20Butler%20-%20Permission%20Registry%20Contract.md) : Modèle de données du registre
- [Master Butler - Capability API Contract](./Master%20Butler%20-%20Capability%20API%20Contract.md) : Surface d'appel pour les capacités
- [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) : Définitions canoniques
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) : Lois d'autonomie

---

## 17. Mini log — erreurs / warnings / ambiguïtés rencontrées et corrigées

### Ambiguïté A1 : Confusion entre Permission API et Permission Registry

**Ambiguïté rencontrée :** Risque de confusion entre la Permission API (surface d'appel) et le Permission Registry Contract (modèle de données).

**Décision prise :** Définition explicite de la complémentarité : le Registry Contract définit le modèle de données, l'API Contract définit la surface d'appel pour interagir avec ce modèle.

**Correction effectuée :** Section 1 et section 2 rédigées avec clarification explicite de cette distinction.

### Ambiguïté A2 : Responsabilité de vérification des permissions

**Ambiguïté rencontrée :** Nécessité de clarifier que la Permission API ne vérifie jamais si une permission est accordée à un contexte.

**Décision prise :** Interdiction explicite (INTERDIT-PERM-1, INTERDIT-PERM-2, INTERDIT-PERM-3) et rappel constant que la décision appartient à StrongFather.

**Correction effectuée :** Sections 3, 7, et schéma 14.3 rédigés avec emphase sur cette séparation.

### Ambiguïté A3 : Gestion des cycles d'implication

**Ambiguïté rencontrée :** Nécessité de définir clairement le comportement en cas de tentative de création de cycle.

**Décision prise :** Ajout d'une opération de validation `checkCycles` et interdiction explicite (INTERDIT-PERM-5) avec rejet REJET-PERM-6.

**Correction effectuée :** Sections 5.4, 7, et 9 rédigées avec gestion explicite des cycles.

### Vérification de compatibilité

**Vérification effectuée :** Vérification systématique de la compatibilité avec le Permission Registry Contract et la Documentation Fondatrice. Aucune contradiction détectée.

**Conclusion :** Le contrat est strictement compatible avec le système contractuel existant.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
