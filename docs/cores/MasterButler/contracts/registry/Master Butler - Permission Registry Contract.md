# Master Butler — Permission Registry Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **Master Butler Permission Registry Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit le modèle conceptuel, la structure, et les règles de gouvernance du registre des permissions dans le système Miyukini Core System v2.4.

Ce contrat établit les fondations nécessaires pour comprendre comment les permissions sont définies, organisées, associées aux capacités, et gérées dans l'écosystème Miyukini.

### Portée

Ce contrat s'applique à **toutes les permissions** du système et définit de manière absolue :

- La définition formelle d'une permission
- La structure du registre des permissions
- Le modèle d'association permission-capacité
- Les règles de définition, modification et révocation
- Les métadonnées obligatoires et optionnelles
- Les invariants non négociables du registre
- Les interactions avec les autres composants

Ce contrat se concentre exclusivement sur les concepts du registre des permissions, sans entrer dans les détails d'implémentation technique ou les mécanismes de vérification runtime.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des définitions absolues et stables qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète les documents contractuels existants :

- **Master Butler — Documentation Fondatrice** : Définit la raison d'être et les responsabilités de Master Butler
- **Master Butler — Capability Registry Contract** : Définit le registre des capacités (complémentaire)
- **Master Butler — Association Model Contract** : Définit les associations entre permissions, rôles et capacités
- **[Miyukini Conceptual References — Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)** : Définitions canoniques des termes
- **[Miyukini Conceptual References — Tools et Toolkits](../../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)** : Gouvernance des Outils et Kits d'Outils
- **[Miyukini Conceptual References — Lois Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Ce contrat respecte **LOI-1** (aucune dépendance externe critique) et **LOI-5** (coût proportionnel au hardware) en garantissant un registre local et léger

**Complémentarité :**

- Capability Registry Contract = ce qui est techniquement possible
- Permission Registry Contract = les droits définis pour accéder aux capacités
- Association Model Contract = liens entre permissions, capacités et rôles

Ces contrats forment ensemble le système complet de gestion des capacités et permissions du système Miyukini Core System v2.4.

---

## 2. Définition formelle d'une permission

### Définition canonique

Une **permission** est un droit formellement défini dans le système pour accéder à une ou plusieurs capacités. Elle représente l'autorisation conceptuelle d'utiliser des capacités, mais ne garantit pas l'autorisation finale (qui dépend de StrongFather).

**Phrase fondatrice :**

> **Une permission définit ce que le système reconnaît comme un droit possible, pas ce qui est effectivement autorisé.**

### Caractéristiques fondamentales d'une permission

| Caractéristique | Description | Obligatoire |
|-----------------|-------------|-------------|
| **Identifiée** | Possède un identifiant unique et stable | ✅ Oui |
| **Nommée** | Possède un nom lisible et descriptif | ✅ Oui |
| **Définie** | Est créée explicitement, jamais implicite | ✅ Oui |
| **Associée** | Référence au moins une capacité existante | ✅ Oui |
| **Documentée** | Possède des métadonnées descriptives | ✅ Oui |
| **Attribuable** | Peut être accordée à des rôles ou contextes | ✅ Oui |
| **Révocable** | Peut être retirée | ✅ Oui |
| **Traçable** | Son historique est enregistré | ✅ Oui |

### Nature conceptuelle

Une permission est un **concept de droit**, pas un mécanisme de vérification. Elle définit ce qui peut être accordé, pas ce qui est effectivement vérifié à l'exécution.

**Important :** La vérification effective d'une permission appartient à StrongFather lors de l'évaluation des intentions. Master Butler ne vérifie jamais si une permission est accordée à un contexte donné — il fournit les définitions, pas les vérifications.

### Distinction permission vs autorisation

| Aspect | Permission | Autorisation |
|--------|------------|--------------|
| **Définition** | Droit défini dans le système | Décision d'accorder ce droit |
| **Responsable** | Master Butler | StrongFather |
| **Nature** | Statique (existe ou n'existe pas) | Dynamique (accordée ou refusée selon contexte) |
| **Question** | "Ce droit existe-t-il ?" | "Ce droit est-il accordé ici et maintenant ?" |

---

## 3. Structure du registre des permissions

### Définition du registre

Le **registre des permissions** est la structure centrale de Master Butler qui contient l'inventaire exhaustif de toutes les permissions définies dans le système.

**Caractéristiques du registre :**

| Propriété | Description |
|-----------|-------------|
| **Central** | Source unique de vérité pour les permissions |
| **Exhaustif** | Contient toutes les permissions, sans exception |
| **Dynamique** | Évolue avec les définitions de permissions |
| **Traçable** | Historise toutes les modifications |
| **Cohérent** | Maintient l'intégrité référentielle |

### Organisation du registre

Le registre est organisé selon une structure hiérarchique logique :

```
Registre des Permissions
├── Domaine (domain)
│   ├── Sous-domaine (subdomain)
│   │   ├── Permission 1
│   │   ├── Permission 2
│   │   └── ...
│   └── ...
└── ...
```

**Convention de nommage des identifiants :**

```
<domaine>.<sous-domaine>.<action>.<portée>
```

**Exemples :**

| Identifiant | Domaine | Sous-domaine | Action | Portée |
|-------------|---------|--------------|--------|--------|
| `content.article.create.any` | content | article | create | any |
| `content.article.edit.own` | content | article | edit | own |
| `media.image.delete.all` | media | image | delete | all |
| `hierarchy.tree.reorder.scope` | hierarchy | tree | reorder | scope |

### Entrée du registre

Chaque entrée du registre représente une permission et contient :

```
Permission Entry
├── Identification
│   ├── id: <identifiant unique>
│   ├── name: <nom lisible>
│   └── version: <version de la définition>
├── Définition
│   ├── description: <description détaillée>
│   ├── domain: <domaine fonctionnel>
│   ├── level: <niveau de criticité>
│   └── scope_type: <type de portée>
├── Associations
│   ├── capabilities: [<liste des capacités couvertes>]
│   └── implied_permissions: [<permissions impliquées>]
├── Métadonnées
│   ├── created_at: <timestamp création>
│   ├── created_by: <identité créateur>
│   ├── modified_at: <timestamp dernière modification>
│   └── modified_by: <identité modificateur>
└── État
    ├── status: <DRAFT | ACTIVE | DEPRECATED | RETIRED>
    └── deprecation_info: <informations de dépréciation si applicable>
```

---

## 4. Modèle de données d'une permission

### Champs d'identification (obligatoires)

| Champ | Type | Description | Contraintes |
|-------|------|-------------|-------------|
| `id` | String | Identifiant unique et immuable | Format : `domain.subdomain.action.scope`, non modifiable après création |
| `name` | String | Nom lisible et descriptif | Non vide, max 128 caractères |
| `version` | String | Version sémantique de la définition | Format : `major.minor.patch` |

### Champs de définition (obligatoires)

| Champ | Type | Description | Contraintes |
|-------|------|-------------|-------------|
| `description` | String | Description détaillée du droit accordé | Non vide, min 20 caractères |
| `domain` | String | Domaine fonctionnel | Valeur du catalogue de domaines |
| `level` | Enum | Niveau de criticité | `STANDARD`, `ELEVATED`, `CRITICAL`, `SYSTEM` |
| `scope_type` | Enum | Type de portée | `GLOBAL`, `SCOPED`, `OWNED`, `CONTEXTUAL` |

### Champs d'association (obligatoires)

| Champ | Type | Description | Contraintes |
|-------|------|-------------|-------------|
| `capabilities` | Array[String] | Capacités couvertes par cette permission | Au moins une capacité, toutes doivent exister dans le Capability Registry |

### Champs d'association (optionnels)

| Champ | Type | Description | Contraintes |
|-------|------|-------------|-------------|
| `implied_permissions` | Array[String] | Permissions impliquées (hiérarchie) | Toutes doivent exister, pas de cycle |
| `required_permissions` | Array[String] | Permissions prérequises | Toutes doivent exister |
| `conflicting_permissions` | Array[String] | Permissions incompatibles | Toutes doivent exister |

### Champs de métadonnées (automatiques)

| Champ | Type | Description | Contraintes |
|-------|------|-------------|-------------|
| `created_at` | Timestamp | Date de création | Généré automatiquement, immuable |
| `created_by` | String | Identité du créateur | Tracé automatiquement |
| `modified_at` | Timestamp | Date de dernière modification | Mis à jour automatiquement |
| `modified_by` | String | Identité du modificateur | Tracé automatiquement |

### Champs d'état

| Champ | Type | Description | Contraintes |
|-------|------|-------------|-------------|
| `status` | Enum | État du cycle de vie | `DRAFT`, `ACTIVE`, `DEPRECATED`, `RETIRED` |
| `deprecation_date` | Timestamp | Date de dépréciation | Requis si status = DEPRECATED |
| `deprecation_reason` | String | Raison de dépréciation | Requis si status = DEPRECATED |
| `successor_id` | String | Permission de remplacement | Recommandé si status = DEPRECATED |

---

## 5. Niveaux de criticité des permissions

### Définition des niveaux

Les permissions sont classées selon leur niveau de criticité, qui détermine les contrôles et validations applicables.

| Niveau | Nom | Description | Validation requise |
|--------|-----|-------------|-------------------|
| `STANDARD` | Standard | Permissions courantes, risque faible | Validation normale |
| `ELEVATED` | Élevé | Permissions sensibles, risque modéré | Validation renforcée |
| `CRITICAL` | Critique | Permissions critiques, risque élevé | Validation stricte + audit |
| `SYSTEM` | Système | Permissions système, usage exceptionnel | Validation système + MiyukiniAdmin |

### Caractéristiques par niveau

#### STANDARD

| Aspect | Règle |
|--------|-------|
| **Attribution** | Par les rôles standards |
| **Audit** | Trace standard |
| **Révocation** | Procédure normale |
| **Exemples** | `content.read.own`, `media.view.public` |

#### ELEVATED

| Aspect | Règle |
|--------|-------|
| **Attribution** | Par les rôles avec autorité élevée |
| **Audit** | Trace détaillée |
| **Révocation** | Procédure avec justification |
| **Exemples** | `content.delete.scope`, `user.invite.team` |

#### CRITICAL

| Aspect | Règle |
|--------|-------|
| **Attribution** | Par StrongFather avec validation explicite |
| **Audit** | Trace complète + alerte WorrySentinel |
| **Révocation** | Procédure formelle avec approbation |
| **Exemples** | `data.export.all`, `hierarchy.restructure.global` |

#### SYSTEM

| Aspect | Règle |
|--------|-------|
| **Attribution** | Uniquement par MiyukiniAdmin |
| **Audit** | Trace système inviolable |
| **Révocation** | Procédure d'urgence uniquement |
| **Exemples** | `system.core.access`, `admin.override.security` |

---

## 6. Types de portée des permissions

### Définition des types de portée

La portée d'une permission définit l'étendue sur laquelle le droit s'applique.

| Type | Nom | Description |
|------|-----|-------------|
| `GLOBAL` | Globale | S'applique à toutes les entités du domaine |
| `SCOPED` | Délimitée | S'applique à un périmètre défini (équipe, projet, etc.) |
| `OWNED` | Propriétaire | S'applique uniquement aux entités possédées par le contexte |
| `CONTEXTUAL` | Contextuelle | S'applique selon des conditions contextuelles dynamiques |

### Exemples par type de portée

#### GLOBAL

```yaml
permission:
  id: "admin.user.manage.global"
  scope_type: GLOBAL
  description: "Gestion de tous les utilisateurs du système"
  # S'applique à TOUS les utilisateurs, sans restriction
```

#### SCOPED

```yaml
permission:
  id: "content.article.edit.team"
  scope_type: SCOPED
  description: "Modification des articles de l'équipe"
  # S'applique aux articles dans le périmètre de l'équipe du contexte
```

#### OWNED

```yaml
permission:
  id: "content.draft.delete.own"
  scope_type: OWNED
  description: "Suppression de ses propres brouillons"
  # S'applique uniquement aux brouillons créés par le contexte
```

#### CONTEXTUAL

```yaml
permission:
  id: "workflow.task.approve.assigned"
  scope_type: CONTEXTUAL
  description: "Approbation des tâches assignées"
  # S'applique selon des conditions évaluées dynamiquement
```

---

## 7. Associations permission-capacité

### Principe fondamental

Une permission est **toujours associée** à une ou plusieurs capacités. Cette association définit quelles capacités sont "couvertes" par la permission.

**Règle absolue :**

> **Une permission sans capacité associée est invalide. Une permission doit référencer au moins une capacité existante.**

### Types d'association

| Type | Description | Exemple |
|------|-------------|---------|
| **Directe** | Une permission couvre exactement une capacité | `content.create.any` → `content.create` |
| **Multiple** | Une permission couvre plusieurs capacités | `content.manage.all` → `content.create`, `content.edit`, `content.delete` |
| **Hiérarchique** | Une permission implique d'autres permissions | `admin.content.full` implique `content.manage.all` |

### Modèle d'association

```
Permission
    │
    ├── capabilities (association directe)
    │   ├── capability_id_1
    │   ├── capability_id_2
    │   └── ...
    │
    └── implied_permissions (association hiérarchique)
        ├── permission_id_1 (qui a ses propres capabilities)
        └── permission_id_2 (qui a ses propres capabilities)
```

### Règles d'association

| Règle | Description | Statut |
|-------|-------------|--------|
| **REG-PERM-ASSOC-1** | Toute permission doit référencer au moins une capacité | NON NÉGOCIABLE |
| **REG-PERM-ASSOC-2** | Toute capacité référencée doit exister dans le Capability Registry | NON NÉGOCIABLE |
| **REG-PERM-ASSOC-3** | Les associations impliquées ne doivent pas créer de cycle | NON NÉGOCIABLE |
| **REG-PERM-ASSOC-4** | La suppression d'une capacité invalide les permissions associées | NON NÉGOCIABLE |
| **REG-PERM-ASSOC-5** | L'ajout d'une association est une modification tracée | NON NÉGOCIABLE |

### Résolution des capacités effectives

Lorsqu'une permission est interrogée, les capacités effectives incluent :

1. Les capacités directement associées (`capabilities`)
2. Les capacités des permissions impliquées (`implied_permissions`), récursivement
3. L'union de toutes ces capacités, sans duplication

**Exemple :**

```
Permission: admin.content.full
├── capabilities: []
└── implied_permissions:
    └── content.manage.all
        ├── capabilities: []
        └── implied_permissions:
            ├── content.create.any
            │   └── capabilities: [content.create]
            ├── content.edit.any
            │   └── capabilities: [content.edit]
            └── content.delete.any
                └── capabilities: [content.delete]

Capacités effectives de admin.content.full:
[content.create, content.edit, content.delete]
```

---

## 8. Cycle de vie d'une permission

### États du cycle de vie

Une permission passe par des états de cycle de vie définis, gérés en cohérence avec Ever Buddy.

| État | Description | Utilisation |
|------|-------------|-------------|
| `DRAFT` | En cours de définition | Non utilisable en production |
| `ACTIVE` | Active et utilisable | Utilisation normale |
| `DEPRECATED` | Dépréciée, usage découragé | Période de transition |
| `RETIRED` | Retirée du système | Non disponible |

### Transitions autorisées

```
DRAFT ──────────────────────────────────────────────────────────► RETIRED
  │                                                                  ▲
  │ activation                                                       │
  ▼                                                                  │
ACTIVE ──────────────────► DEPRECATED ───────────────────────────────┘
           dépréciation           retrait
```

| Transition | Conditions | Actions |
|------------|------------|---------|
| DRAFT → ACTIVE | Validation complète, associations valides | Enregistrement, notification |
| DRAFT → RETIRED | Abandon de la définition | Suppression du draft |
| ACTIVE → DEPRECATED | Justification obligatoire, successeur recommandé | Notification des consommateurs |
| DEPRECATED → RETIRED | Période de dépréciation écoulée | Invalidation, archivage |

### Règles de cycle de vie

| Règle | Description | Statut |
|-------|-------------|--------|
| **REG-PERM-LIFE-1** | Une permission ne peut être retirée sans passer par DEPRECATED (sauf DRAFT) | NON NÉGOCIABLE |
| **REG-PERM-LIFE-2** | La période de dépréciation minimale est définie par politique | NON NÉGOCIABLE |
| **REG-PERM-LIFE-3** | Toute transition est tracée avec contexte complet | NON NÉGOCIABLE |
| **REG-PERM-LIFE-4** | Les permissions RETIRED restent dans l'historique | NON NÉGOCIABLE |

---

## 9. Opérations sur le registre

### 9.1. Définition d'une permission

**Acteurs :** Opérateur, BondingBrother, Master Butler

**Séquence :**

1. L'Opérateur soumet une définition de permission
2. BondingBrother traduit et transmet à Master Butler
3. Master Butler valide la structure de la définition
4. Master Butler vérifie l'existence des capacités référencées
5. Master Butler vérifie l'absence de cycle dans les implications
6. Master Butler enregistre la permission en état DRAFT
7. Master Butler confirme l'enregistrement

**Validations obligatoires :**

| Validation | Description | Erreur si échoue |
|------------|-------------|------------------|
| Structure valide | Tous les champs obligatoires présents | `INVALID_PERMISSION_STRUCTURE` |
| Identifiant unique | L'identifiant n'existe pas déjà | `DUPLICATE_PERMISSION_ID` |
| Capacités existantes | Toutes les capacités référencées existent | `UNKNOWN_CAPABILITY` |
| Pas de cycle | Les implications ne créent pas de cycle | `CYCLIC_IMPLICATION` |
| Niveau autorisé | Le créateur peut créer ce niveau | `UNAUTHORIZED_LEVEL` |

### 9.2. Activation d'une permission

**Acteurs :** Opérateur autorisé, Master Butler

**Séquence :**

1. L'Opérateur demande l'activation d'une permission DRAFT
2. Master Butler vérifie que la permission est en état DRAFT
3. Master Butler vérifie que toutes les validations sont satisfaites
4. Master Butler change l'état à ACTIVE
5. Master Butler notifie les composants concernés
6. Master Butler confirme l'activation

**Conditions d'activation :**

- État actuel = DRAFT
- Toutes les capacités référencées sont ACTIVE
- Toutes les permissions impliquées sont ACTIVE
- L'Opérateur a l'autorité d'activer ce niveau de permission

### 9.3. Modification d'une permission

**Règles de modification :**

| Champ | Modifiable en DRAFT | Modifiable en ACTIVE | Modifiable en DEPRECATED |
|-------|---------------------|----------------------|--------------------------|
| `id` | ❌ Non | ❌ Non | ❌ Non |
| `name` | ✅ Oui | ⚠️ Avec version | ❌ Non |
| `description` | ✅ Oui | ⚠️ Avec version | ❌ Non |
| `capabilities` | ✅ Oui | ⚠️ Avec version | ❌ Non |
| `implied_permissions` | ✅ Oui | ⚠️ Avec version | ❌ Non |
| `level` | ✅ Oui | ❌ Non | ❌ Non |
| `scope_type` | ✅ Oui | ❌ Non | ❌ Non |

**⚠️ Avec version** : La modification incrémente la version mineure et est tracée.

### 9.4. Dépréciation d'une permission

**Acteurs :** Opérateur autorisé, Master Butler

**Séquence :**

1. L'Opérateur demande la dépréciation avec justification
2. Master Butler vérifie que la permission est ACTIVE
3. Master Butler enregistre la raison et la date de dépréciation
4. Master Butler change l'état à DEPRECATED
5. Master Butler notifie les consommateurs de la permission
6. Master Butler confirme la dépréciation

**Informations requises :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `deprecation_reason` | Raison de la dépréciation | ✅ Oui |
| `deprecation_date` | Date effective de dépréciation | ✅ Oui (auto si non fourni) |
| `successor_id` | Permission de remplacement | Recommandé |
| `migration_guide` | Guide de migration | Recommandé |

### 9.5. Retrait d'une permission

**Acteurs :** Master Butler (automatique ou manuel)

**Conditions de retrait :**

- État actuel = DEPRECATED
- Période de dépréciation minimale écoulée
- Aucune attribution active (ou migration forcée)

**Conséquences du retrait :**

- La permission devient inutilisable
- Les références existantes deviennent invalides
- L'historique est conservé
- Les attributions sont révoquées

---

## 10. Interrogation du registre

### Types de requêtes

Master Butler expose les requêtes suivantes sur le registre des permissions :

| Requête | Description | Paramètres |
|---------|-------------|------------|
| `getPermission` | Récupère une permission par identifiant | `permission_id` |
| `listPermissions` | Liste les permissions selon critères | `domain`, `level`, `status`, `scope_type` |
| `getPermissionCapabilities` | Récupère les capacités d'une permission | `permission_id`, `include_implied` |
| `searchPermissions` | Recherche par nom ou description | `query`, `filters` |
| `getPermissionHierarchy` | Récupère la hiérarchie d'implications | `permission_id` |
| `validatePermission` | Valide une définition de permission | `permission_definition` |

### Réponses standardisées

Toutes les réponses incluent :

```yaml
response:
  success: <boolean>
  data: <données demandées>
  metadata:
    request_id: <identifiant de requête>
    timestamp: <timestamp de réponse>
    source: "MasterButler.PermissionRegistry"
  errors: [<liste d'erreurs si success = false>]
```

### Filtrage et pagination

Les requêtes de liste supportent :

| Paramètre | Type | Description |
|-----------|------|-------------|
| `domain` | String | Filtrer par domaine |
| `level` | Enum[] | Filtrer par niveaux |
| `status` | Enum[] | Filtrer par états |
| `scope_type` | Enum[] | Filtrer par types de portée |
| `offset` | Integer | Décalage pour pagination |
| `limit` | Integer | Nombre maximum de résultats |
| `sort_by` | String | Champ de tri |
| `sort_order` | Enum | `ASC` ou `DESC` |

---

## 11. Invariants non négociables

### INV-PERM-REG-1 : Exhaustivité

Le registre des permissions est **exhaustif**. Toute permission existant dans le système est recensée dans le registre. Si une permission n'est pas dans le registre, elle n'existe pas officiellement dans le système.

**Implication :** Aucun composant ne peut reconnaître une permission non enregistrée. Aucune attribution ne peut référencer une permission inexistante.

### INV-PERM-REG-2 : Unicité des identifiants

Chaque permission possède un **identifiant unique et immuable**. Aucun doublon n'est autorisé. L'identifiant ne peut jamais être modifié après création.

**Implication :** Les références aux permissions restent valides dans le temps. Les logs et audits peuvent toujours identifier une permission de manière non ambiguë.

### INV-PERM-REG-3 : Association obligatoire

Toute permission doit être **associée à au moins une capacité existante**. Une permission sans capacité est invalide et ne peut être activée.

**Implication :** La suppression d'une capacité rend invalides les permissions qui ne référencent que cette capacité. Ces permissions doivent être mises à jour ou dépréciées.

### INV-PERM-REG-4 : Intégrité référentielle

Toutes les références dans le registre sont **valides et vérifiées**. Les capacités référencées existent. Les permissions impliquées existent. Les successeurs référencés existent.

**Implication :** Le registre ne contient jamais de référence vers un élément inexistant. Toute opération qui créerait une référence invalide est rejetée.

### INV-PERM-REG-5 : Absence de cycle

Les **implications de permissions ne créent jamais de cycle**. Une permission ne peut pas s'impliquer elle-même, directement ou indirectement.

**Implication :** La résolution des capacités effectives termine toujours. Aucune boucle infinie n'est possible.

### INV-PERM-REG-6 : Traçabilité complète

Toute modification du registre est **tracée avec contexte complet**. Créations, modifications, dépréciations, retraits : tout est enregistré avec qui, quand, pourquoi.

**Implication :** L'historique des permissions est auditable. Aucune modification silencieuse n'est possible. La conformité peut être vérifiée.

### INV-PERM-REG-7 : Non-vérification

Master Butler **ne vérifie jamais** si une permission est effectivement accordée à un contexte. Il définit ce qui existe, pas ce qui est autorisé.

**Implication :** Aucune méthode du registre ne retourne "accordé" ou "refusé". Ces décisions appartiennent à StrongFather.

### INV-PERM-REG-8 : Cohérence des états

Les **transitions d'état suivent un chemin défini**. Aucune transition arbitraire n'est autorisée. Les règles de cycle de vie sont strictement appliquées.

**Implication :** Une permission RETIRED ne peut pas redevenir ACTIVE. Une permission ACTIVE ne peut pas redevenir DRAFT.

---

## 12. Interactions avec les autres composants

### 12.1. Interaction avec StrongFather

**Flux typique :**

```
StrongFather évalue une intention
    │
    ├── Interroge Master Butler : "Quelles permissions couvrent cette capacité ?"
    │       │
    │       └── Master Butler répond : Liste des permissions
    │
    ├── Interroge Master Butler : "Quelle est la définition de cette permission ?"
    │       │
    │       └── Master Butler répond : Définition complète
    │
    └── StrongFather décide selon les politiques
```

**Règles d'interaction :**

- StrongFather est toujours autorisé à interroger le registre
- Master Butler ne suggère jamais de décision
- Les réponses sont exhaustives et exactes
- Aucun cache de décision dans Master Butler

### 12.2. Interaction avec BondingBrother

**Flux typique :**

```
BondingBrother traduit une intention
    │
    ├── Interroge Master Butler : "Quelles permissions sont requises pour cette action ?"
    │       │
    │       └── Master Butler répond : Permissions requises
    │
    └── BondingBrother enrichit le contexte de l'intention
```

**Règles d'interaction :**

- BondingBrother interroge pour la traduction, pas pour la décision
- Les réponses aident à construire le contexte
- Aucune interprétation par Master Butler

### 12.3. Interaction avec les Opérateurs

**Flux de définition :**

```
Opérateur définit une nouvelle permission
    │
    ├── Soumet la définition via BondingBrother
    │       │
    │       └── Master Butler valide et enregistre
    │
    └── Confirmation de l'enregistrement
```

**Flux de découverte :**

```
Opérateur découvre les permissions disponibles
    │
    ├── Interroge Master Butler : "Quelles permissions existent pour ce domaine ?"
    │       │
    │       └── Master Butler répond : Liste des permissions (selon contexte)
    │
    └── Opérateur utilise ces informations
```

### 12.4. Interaction avec Ever Buddy

**Coordination du cycle de vie :**

```
Ever Buddy gère l'évolution des permissions
    │
    ├── Vérifie la compatibilité des versions
    │       │
    │       └── Master Butler fournit les versions
    │
    ├── Gère les dépréciations programmées
    │       │
    │       └── Master Butler exécute les transitions
    │
    └── Orchestre les migrations
            │
            └── Master Butler applique les changements
```

---

## 13. Conformité aux Lois d'Autonomie Système

Ce contrat respecte les Lois d'Autonomie Système définies dans [Miyukini Framework - Lois Autonomie Systeme.md](../../../../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md).

### LOI-1 : Aucune dépendance externe critique à l'exécution

**Conformité :** Conforme

Le registre des permissions est entièrement local :

- **Stockage local** : Les permissions sont définies et stockées localement
- **Interrogation locale** : Toutes les requêtes s'exécutent localement
- **Aucune API externe** : Aucun service distant n'est requis

**Vérification LOI-1** : *"Le registre des permissions fonctionne-t-il si le réseau est indisponible ?"* → **Oui.**

### LOI-5 : Le coût doit être proportionnel au hardware

**Conformité :** Conforme

Le registre des permissions a une empreinte minimale :

- **Données légères** : Les permissions sont des métadonnées textuelles
- **Pas de workers** : Aucun processus en arrière-plan
- **Lookups simples** : Opérations de consultation directe
- **Mémoire prévisible** : Proportionnelle au nombre de permissions

**Vérification LOI-5** : *"Le registre fonctionne-t-il sur un Raspberry Pi 4 ?"* → **Oui.** Un registre typique (quelques centaines de permissions) représente quelques kilo-octets.

### Synthèse de conformité

| Loi | Statut | Raison |
|-----|--------|--------|
| LOI-1 | ✅ Conforme | Registre local, aucune dépendance externe |
| LOI-5 | ✅ Conforme | Métadonnées légères, consommation minimale |

---

## 14. Exemples de permissions

### Exemple 1 : Permission standard

```yaml
permission:
  id: "content.article.create.own"
  name: "Create Own Articles"
  version: "1.0.0"
  description: "Allows creating articles attributed to the current user"
  domain: "content"
  level: STANDARD
  scope_type: OWNED
  capabilities:
    - "content.create"
  status: ACTIVE
  created_at: "2026-01-27T10:00:00Z"
  created_by: "system:bootstrap"
```

### Exemple 2 : Permission avec implications

```yaml
permission:
  id: "content.manage.team"
  name: "Manage Team Content"
  version: "1.0.0"
  description: "Full management of team content including create, edit, delete"
  domain: "content"
  level: ELEVATED
  scope_type: SCOPED
  capabilities: []
  implied_permissions:
    - "content.article.create.team"
    - "content.article.edit.team"
    - "content.article.delete.team"
  status: ACTIVE
  created_at: "2026-01-27T10:00:00Z"
  created_by: "system:bootstrap"
```

### Exemple 3 : Permission critique

```yaml
permission:
  id: "data.export.all"
  name: "Export All Data"
  version: "1.0.0"
  description: "Allows exporting all data from the system - critical operation"
  domain: "data"
  level: CRITICAL
  scope_type: GLOBAL
  capabilities:
    - "data.export"
    - "data.read"
  required_permissions:
    - "data.read.all"
  status: ACTIVE
  created_at: "2026-01-27T10:00:00Z"
  created_by: "admin:setup"
```

### Exemple 4 : Permission dépréciée

```yaml
permission:
  id: "content.publish.direct"
  name: "Direct Publish Content"
  version: "1.2.0"
  description: "Allows publishing content without workflow - DEPRECATED"
  domain: "content"
  level: ELEVATED
  scope_type: GLOBAL
  capabilities:
    - "content.publish"
  status: DEPRECATED
  deprecation_date: "2026-01-15T00:00:00Z"
  deprecation_reason: "Replaced by workflow-based publishing"
  successor_id: "content.publish.workflow"
  created_at: "2025-06-01T10:00:00Z"
  created_by: "system:bootstrap"
```

---

## 15. Conclusion et statut contractuel

### Essence du Permission Registry Contract

Le registre des permissions de Master Butler est la **source de vérité** pour tous les droits définis dans le système Miyukini. Il définit ce qui peut être accordé, sans jamais décider ce qui est effectivement autorisé.

Ce registre incarne la séparation entre :
- **La définition des droits** (Master Butler)
- **L'attribution des droits** (mécanismes d'attribution)
- **La vérification des droits** (StrongFather)

### Phrase fondatrice

> **Le registre des permissions définit les droits possibles du système Miyukini, en association avec les capacités, sans jamais participer à la décision d'autorisation.**

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

Toute implémentation du registre des permissions doit respecter intégralement ce document. Toute évolution doit préserver les invariants définis ici.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** FONDATION — Non négociable  
**Référence :** Miyukini Core System v2.4

**Références croisées :**

- [Master Butler - Documentation Fondatrice](../../Master%20Butler%20-%20Documentation%20Fondatrice.md) : Définition et responsabilités de Master Butler
- [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) : Définitions canoniques
- [Miyukini Conceptual References - Tools et Toolkits](../../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) : Gouvernance des Outils
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) : Lois d'autonomie
