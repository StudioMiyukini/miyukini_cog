# Master Butler — Discovery API Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **Master Butler Discovery API Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit l'API de découverte des capacités et permissions dans le système Miyukini Core System v2.4.

Ce contrat définit :

- Les opérations de découverte disponibles
- Les modes de découverte (par module, par type, par contexte)
- Les règles de filtrage et de visibilité
- Les réponses standardisées
- Les invariants de l'API de découverte
- Les interactions avec les autres composants

### Portée

Ce contrat s'applique à **toute opération de découverte** via Master Butler et définit de manière absolue :

- La découverte des capacités par module
- La découverte des capacités par type d'action
- La découverte des permissions par capacité
- La découverte du contexte de capacité
- Le filtrage selon le contexte demandeur
- Les règles de visibilité et d'accès

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues que Master Butler applique sans exception. Ces règles ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète les documents contractuels existants :

- **[Master Butler - Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md)** : Définit la nature, le rôle, et les responsabilités de Master Butler
- **[Master Butler - Capability Registry Contract](../registry/Master%20Butler%20-%20Capability%20Registry%20Contract.md)** : Définit le registre des capacités (source des données découvertes)
- **[Master Butler - Permission Registry Contract](../registry/Master%20Butler%20-%20Permission%20Registry%20Contract.md)** : Définit le registre des permissions (source des données découvertes)
- **[Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)** : Définitions canoniques des termes
- **[Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Ce contrat respecte **LOI-1** (aucune dépendance externe critique) et **LOI-5** (coût proportionnel au hardware)

**Complémentarité :**

- Capability Registry Contract = structure et gestion des capacités
- Permission Registry Contract = structure et gestion des permissions
- Discovery API Contract = mécanismes de découverte et d'exploration

Ces contrats forment ensemble le système complet de gestion et d'exploration des capacités et permissions.

---

## 2. Définition de la Découverte

### Définition canonique

La **découverte** est le processus par lequel un composant interroge Master Butler pour connaître les capacités et permissions existantes dans le système. La découverte permet l'exploration dynamique des possibilités du système.

**Phrase fondatrice :**

> **La découverte expose ce qui est possible, sans jamais suggérer ce qui est autorisé.**

### Caractéristiques de la découverte

| Caractéristique | Description |
|-----------------|-------------|
| **Non-intrusive** | La découverte ne modifie pas les registres |
| **Contextuelle** | Les résultats peuvent être filtrés selon le contexte |
| **Exhaustive** | Retourne toutes les informations pertinentes selon les critères |
| **Traçable** | Les requêtes de découverte sont journalisées |
| **Accessible** | Disponible pour tous les composants autorisés |

### Nature de la découverte

La découverte est une opération de **lecture seule** qui permet aux composants de :

1. Explorer les capacités disponibles dans le système
2. Identifier les permissions associées à des capacités
3. Calculer les capacités accessibles dans un contexte donné
4. Comprendre la structure des modules et leurs capacités

**Important :** La découverte fournit des informations, mais ne constitue jamais une autorisation. La décision d'autorisation appartient exclusivement à StrongFather.

### Distinction découverte vs autorisation

| Aspect | Découverte | Autorisation |
|--------|------------|--------------|
| **Définition** | Exploration de ce qui existe | Décision d'accorder un accès |
| **Responsable** | Master Butler | StrongFather |
| **Nature** | Lecture seule, informative | Décisionnelle |
| **Question** | "Qu'existe-t-il ?" | "Est-ce permis ?" |
| **Modification** | Aucune | Peut modifier l'état d'autorisation |

---

## 3. Modes de Découverte

### 3.1. Découverte par Module (DiscoverByModule)

**Énoncé :**

L'opération **DiscoverByModule** permet de découvrir toutes les capacités déclarées par un module spécifique.

**Signature conceptuelle :**

```
DiscoverByModule(
  module_id: ModuleIdentifier,
  filter: DiscoveryFilter?
) → Result<DiscoveryResult, DiscoveryError>
```

**Paramètres :**

| Paramètre | Type | Obligatoire | Description |
|-----------|------|-------------|-------------|
| `module_id` | ModuleIdentifier | ✅ Oui | Identifiant du module source |
| `filter` | DiscoveryFilter | ❌ Non | Filtres optionnels (statut, catégorie) |

**Résultat :**

```
DiscoveryResult {
  capabilities: List<CapabilitySummary>,
  metadata: DiscoveryMetadata,
  total_count: Integer,
  filtered_count: Integer
}

CapabilitySummary {
  id: CapabilityId,
  name: String,
  description: String,
  category: CapabilityCategory,
  status: CapabilityStatus,
  associated_permissions: List<PermissionId>
}
```

**Exemples d'utilisation :**

```yaml
# Découvrir toutes les capacités du module CMS Content
request:
  operation: DiscoverByModule
  module_id: "spm.cms.content"
  filter: null

response:
  success: true
  data:
    capabilities:
      - id: "content.create"
        name: "Créer du contenu"
        category: "Data"
        status: "Active"
        associated_permissions: ["content.create.any", "content.create.own"]
      - id: "content.read"
        name: "Lire du contenu"
        category: "Data"
        status: "Active"
        associated_permissions: ["content.read.any", "content.read.own"]
    total_count: 8
    filtered_count: 8
```

**Règles :**

- R-DBM-1 : Le module doit exister dans le registre
- R-DBM-2 : Seules les capacités avec statut correspondant au filtre sont retournées
- R-DBM-3 : Les capacités confidentielles sont filtrées selon le contexte demandeur

### 3.2. Découverte par Type d'Action (DiscoverByAction)

**Énoncé :**

L'opération **DiscoverByAction** permet de découvrir toutes les capacités correspondant à un type d'action donné.

**Signature conceptuelle :**

```
DiscoverByAction(
  action_type: ActionType,
  domain: DomainIdentifier?,
  filter: DiscoveryFilter?
) → Result<DiscoveryResult, DiscoveryError>
```

**Paramètres :**

| Paramètre | Type | Obligatoire | Description |
|-----------|------|-------------|-------------|
| `action_type` | ActionType | ✅ Oui | Type d'action (create, read, update, delete, etc.) |
| `domain` | DomainIdentifier | ❌ Non | Domaine de filtrage optionnel |
| `filter` | DiscoveryFilter | ❌ Non | Filtres additionnels |

**Types d'actions standards :**

| ActionType | Description | Exemples de capacités |
|------------|-------------|----------------------|
| `create` | Actions de création | `content.create`, `media.upload` |
| `read` | Actions de lecture | `content.read`, `user.profile.view` |
| `update` | Actions de modification | `content.edit`, `hierarchy.reorder` |
| `delete` | Actions de suppression | `content.delete`, `media.remove` |
| `publish` | Actions de publication | `content.publish`, `media.publish` |
| `manage` | Actions de gestion | `content.manage`, `user.manage` |
| `search` | Actions de recherche | `search.query`, `search.index` |
| `export` | Actions d'export | `data.export`, `report.generate` |

**Exemples d'utilisation :**

```yaml
# Découvrir toutes les capacités de type "create" dans le domaine "content"
request:
  operation: DiscoverByAction
  action_type: "create"
  domain: "content"

response:
  success: true
  data:
    capabilities:
      - id: "content.create"
        name: "Créer du contenu"
        source: "spm.cms.content"
      - id: "content.draft.create"
        name: "Créer un brouillon"
        source: "spm.cms.content"
    total_count: 2
```

**Règles :**

- R-DBA-1 : Le type d'action est obligatoire
- R-DBA-2 : Le domaine est optionnel et sert de filtre
- R-DBA-3 : La correspondance est basée sur le segment d'action dans l'identifiant

### 3.3. Découverte par Catégorie (DiscoverByCategory)

**Énoncé :**

L'opération **DiscoverByCategory** permet de découvrir toutes les capacités d'une catégorie fonctionnelle donnée.

**Signature conceptuelle :**

```
DiscoverByCategory(
  category: CapabilityCategory,
  filter: DiscoveryFilter?
) → Result<DiscoveryResult, DiscoveryError>
```

**Catégories supportées :**

| Catégorie | Description |
|-----------|-------------|
| `Data` | Capacités liées aux données |
| `Hierarchy` | Capacités liées aux hiérarchies |
| `Media` | Capacités liées aux médias |
| `Search` | Capacités liées à la recherche |
| `Auth` | Capacités liées à l'authentification |
| `Admin` | Capacités d'administration |
| `UI` | Capacités d'interface utilisateur |
| `IO` | Capacités d'entrée/sortie |
| `System` | Capacités système |

**Exemples d'utilisation :**

```yaml
# Découvrir toutes les capacités de la catégorie "Media"
request:
  operation: DiscoverByCategory
  category: "Media"

response:
  success: true
  data:
    capabilities:
      - id: "media.upload"
        name: "Téléverser un média"
        source: "spm.cms.media"
      - id: "media.delete"
        name: "Supprimer un média"
        source: "spm.cms.media"
      - id: "media.transform"
        name: "Transformer un média"
        source: "spm.cms.media"
    total_count: 6
```

### 3.4. Découverte des Permissions par Capacité (DiscoverPermissionsForCapability)

**Énoncé :**

L'opération **DiscoverPermissionsForCapability** permet de découvrir toutes les permissions qui couvrent une capacité donnée.

**Signature conceptuelle :**

```
DiscoverPermissionsForCapability(
  capability_id: CapabilityId,
  include_implied: Boolean?
) → Result<PermissionDiscoveryResult, DiscoveryError>
```

**Paramètres :**

| Paramètre | Type | Obligatoire | Description |
|-----------|------|-------------|-------------|
| `capability_id` | CapabilityId | ✅ Oui | Identifiant de la capacité |
| `include_implied` | Boolean | ❌ Non | Inclure les permissions impliquant d'autres permissions (défaut: true) |

**Résultat :**

```
PermissionDiscoveryResult {
  capability: CapabilitySummary,
  direct_permissions: List<PermissionSummary>,
  implied_permissions: List<PermissionSummary>,
  metadata: DiscoveryMetadata
}

PermissionSummary {
  id: PermissionId,
  name: String,
  description: String,
  level: PermissionLevel,
  scope_type: ScopeType,
  status: PermissionStatus
}
```

**Exemples d'utilisation :**

```yaml
# Découvrir les permissions pour la capacité "content.create"
request:
  operation: DiscoverPermissionsForCapability
  capability_id: "content.create"
  include_implied: true

response:
  success: true
  data:
    capability:
      id: "content.create"
      name: "Créer du contenu"
    direct_permissions:
      - id: "content.create.any"
        name: "Créer n'importe quel contenu"
        level: "ELEVATED"
        scope_type: "GLOBAL"
      - id: "content.create.own"
        name: "Créer son propre contenu"
        level: "STANDARD"
        scope_type: "OWNED"
    implied_permissions:
      - id: "content.manage.all"
        name: "Gestion complète du contenu"
        level: "CRITICAL"
        # Implique content.create.any
```

**Règles :**

- R-DPC-1 : La capacité doit exister dans le registre
- R-DPC-2 : Les permissions RETIRED ne sont pas incluses par défaut
- R-DPC-3 : Les permissions impliquées sont résolues récursivement si include_implied=true

### 3.5. Découverte des Capacités par Permission (DiscoverCapabilitiesForPermission)

**Énoncé :**

L'opération **DiscoverCapabilitiesForPermission** permet de découvrir toutes les capacités couvertes par une permission donnée.

**Signature conceptuelle :**

```
DiscoverCapabilitiesForPermission(
  permission_id: PermissionId,
  resolve_implied: Boolean?
) → Result<CapabilityDiscoveryResult, DiscoveryError>
```

**Paramètres :**

| Paramètre | Type | Obligatoire | Description |
|-----------|------|-------------|-------------|
| `permission_id` | PermissionId | ✅ Oui | Identifiant de la permission |
| `resolve_implied` | Boolean | ❌ Non | Résoudre les permissions impliquées (défaut: true) |

**Résultat :**

```
CapabilityDiscoveryResult {
  permission: PermissionSummary,
  direct_capabilities: List<CapabilitySummary>,
  implied_capabilities: List<CapabilitySummary>,
  metadata: DiscoveryMetadata
}
```

**Exemples d'utilisation :**

```yaml
# Découvrir les capacités couvertes par "content.manage.all"
request:
  operation: DiscoverCapabilitiesForPermission
  permission_id: "content.manage.all"
  resolve_implied: true

response:
  success: true
  data:
    permission:
      id: "content.manage.all"
      name: "Gestion complète du contenu"
    direct_capabilities: []
    implied_capabilities:
      - id: "content.create"
        name: "Créer du contenu"
      - id: "content.edit"
        name: "Modifier du contenu"
      - id: "content.delete"
        name: "Supprimer du contenu"
```

---

## 4. Contexte de Capacité

### 4.1. Définition

Le **contexte de capacité** est l'ensemble des informations qui définissent les capacités et permissions disponibles dans une situation donnée.

**Composition du contexte :**

| Élément | Description |
|---------|-------------|
| `requester_identity` | Identité du demandeur (utilisateur, système, produit) |
| `requester_roles` | Rôles du demandeur |
| `target_module` | Module ou composant ciblé |
| `security_level` | Niveau de sécurité courant |
| `environment` | Environnement d'exécution |

### 4.2. Calcul du Contexte de Capacité (ComputeCapabilityContext)

**Énoncé :**

L'opération **ComputeCapabilityContext** permet de calculer les capacités et permissions accessibles dans un contexte donné.

**Signature conceptuelle :**

```
ComputeCapabilityContext(
  context: ContextSpecification
) → Result<CapabilityContext, DiscoveryError>
```

**Structure du contexte :**

```
ContextSpecification {
  requester: RequesterIdentity,
  roles: List<RoleId>,
  target: TargetSpecification?,
  security_constraints: SecurityConstraints?
}

RequesterIdentity {
  type: RequesterType,  // User, System, Operator, Tool
  id: String
}

TargetSpecification {
  module: ModuleIdentifier?,
  domain: DomainIdentifier?,
  resource: ResourceIdentifier?
}

SecurityConstraints {
  max_level: SecurityLevel?,
  required_clearance: ClearanceLevel?
}
```

**Résultat :**

```
CapabilityContext {
  requester: RequesterIdentity,
  accessible_capabilities: List<CapabilitySummary>,
  accessible_permissions: List<PermissionSummary>,
  restrictions: List<Restriction>,
  computed_at: Timestamp,
  valid_for: Duration?,
  metadata: ContextMetadata
}

Restriction {
  type: RestrictionType,
  reason: String,
  affected_capabilities: List<CapabilityId>
}
```

**Exemples d'utilisation :**

```yaml
# Calculer le contexte pour un utilisateur avec le rôle "editor"
request:
  operation: ComputeCapabilityContext
  context:
    requester:
      type: "User"
      id: "user_123"
    roles: ["editor", "content_creator"]
    target:
      module: "spm.cms.content"
    security_constraints:
      max_level: "ELEVATED"

response:
  success: true
  data:
    requester:
      type: "User"
      id: "user_123"
    accessible_capabilities:
      - id: "content.create"
        name: "Créer du contenu"
      - id: "content.edit"
        name: "Modifier du contenu"
      - id: "content.read"
        name: "Lire du contenu"
    accessible_permissions:
      - id: "content.create.own"
        level: "STANDARD"
      - id: "content.edit.own"
        level: "STANDARD"
      - id: "content.edit.team"
        level: "ELEVATED"
    restrictions:
      - type: "SECURITY_LEVEL"
        reason: "CRITICAL level capabilities excluded"
        affected_capabilities: ["content.delete.all"]
    computed_at: "2026-01-27T10:30:00Z"
```

**Règles :**

- R-CCC-1 : Le calcul de contexte est une projection, jamais une décision
- R-CCC-2 : Le contexte est calculé à la demande, jamais mis en cache décisionnel
- R-CCC-3 : Les restrictions de sécurité sont toujours appliquées
- R-CCC-4 : Le résultat ne modifie pas les registres

### 4.3. Règles de Calcul du Contexte

| Règle | Description | Impact |
|-------|-------------|--------|
| **R-CTX-1** | Les capacités DEPRECATED sont incluses avec avertissement | Visibilité avec notice |
| **R-CTX-2** | Les capacités REMOVED sont exclues | Non visibles |
| **R-CTX-3** | Les permissions DRAFT sont exclues | Non utilisables |
| **R-CTX-4** | Le niveau de sécurité filtre les capacités | Exclusion si niveau insuffisant |
| **R-CTX-5** | Les rôles déterminent les permissions accessibles | Filtrage par association |

---

## 5. Filtres de Découverte

### 5.1. Structure du Filtre

```
DiscoveryFilter {
  status: List<Status>?,        // Active, Deprecated, Removed
  category: List<Category>?,    // Data, Media, etc.
  level: List<Level>?,          // STANDARD, ELEVATED, CRITICAL, SYSTEM
  scope_type: List<ScopeType>?, // GLOBAL, SCOPED, OWNED, CONTEXTUAL
  tags: List<String>?,          // Tags de recherche
  created_after: Timestamp?,    // Date de création minimum
  created_before: Timestamp?,   // Date de création maximum
  source_type: List<SourceType>?, // Module, Core, Operator, Tool
  search_query: String?         // Recherche textuelle
}
```

### 5.2. Application des Filtres

Les filtres sont combinés avec une logique **AND** :

```yaml
# Exemple : Capacités Active OU Deprecated, de catégorie Data, de niveau STANDARD
filter:
  status: ["Active", "Deprecated"]  # Active OR Deprecated
  category: ["Data"]                 # AND category = Data
  level: ["STANDARD"]                # AND level = STANDARD
# Résultat : (status IN [Active, Deprecated]) AND (category = Data) AND (level = STANDARD)
```

### 5.3. Pagination

Les résultats de découverte supportent la pagination :

```
PaginationParams {
  offset: Integer,    // Décalage (défaut: 0)
  limit: Integer,     // Nombre max de résultats (défaut: 50, max: 500)
  sort_by: String,    // Champ de tri (défaut: "id")
  sort_order: SortOrder  // ASC ou DESC (défaut: ASC)
}

PaginatedResult {
  items: List<T>,
  total_count: Integer,
  offset: Integer,
  limit: Integer,
  has_more: Boolean
}
```

---

## 6. Réponses Standardisées

### 6.1. Structure de Réponse

Toutes les opérations de découverte retournent une réponse avec la structure suivante :

```
DiscoveryResponse<T> {
  success: Boolean,
  data: T?,
  metadata: ResponseMetadata,
  errors: List<DiscoveryError>?
}

ResponseMetadata {
  request_id: String,
  timestamp: Timestamp,
  source: "MasterButler.DiscoveryAPI",
  version: String,
  processing_time_ms: Integer
}
```

### 6.2. Codes d'Erreur

| Code | Nom | Description |
|------|-----|-------------|
| `DISC-001` | `INVALID_FILTER` | Filtre invalide ou mal formé |
| `DISC-002` | `UNKNOWN_MODULE` | Module non trouvé dans le registre |
| `DISC-003` | `UNKNOWN_CAPABILITY` | Capacité non trouvée dans le registre |
| `DISC-004` | `UNKNOWN_PERMISSION` | Permission non trouvée dans le registre |
| `DISC-005` | `INVALID_CONTEXT` | Contexte de découverte invalide |
| `DISC-006` | `ACCESS_DENIED` | Accès refusé au composant demandeur |
| `DISC-007` | `PAGINATION_ERROR` | Paramètres de pagination invalides |
| `DISC-008` | `TIMEOUT` | Timeout de la requête de découverte |
| `DISC-009` | `INTERNAL_ERROR` | Erreur interne du registre |

### 6.3. Exemples de Réponses

**Réponse succès :**

```yaml
response:
  success: true
  data:
    capabilities:
      - id: "content.create"
        name: "Créer du contenu"
    total_count: 1
  metadata:
    request_id: "req_abc123"
    timestamp: "2026-01-27T10:30:00Z"
    source: "MasterButler.DiscoveryAPI"
    version: "1.0"
    processing_time_ms: 12
  errors: null
```

**Réponse erreur :**

```yaml
response:
  success: false
  data: null
  metadata:
    request_id: "req_def456"
    timestamp: "2026-01-27T10:30:00Z"
    source: "MasterButler.DiscoveryAPI"
    version: "1.0"
    processing_time_ms: 5
  errors:
    - code: "DISC-002"
      message: "Module 'spm.unknown.module' not found in registry"
      details:
        module_id: "spm.unknown.module"
```

---

## 7. Règles de Visibilité

### 7.1. Visibilité par Défaut

| Élément | Visibilité par défaut |
|---------|----------------------|
| Capacités ACTIVE | ✅ Visibles |
| Capacités DEPRECATED | ✅ Visibles (avec warning) |
| Capacités REMOVED | ❌ Non visibles |
| Permissions ACTIVE | ✅ Visibles |
| Permissions DEPRECATED | ✅ Visibles (avec warning) |
| Permissions DRAFT | ❌ Non visibles (sauf créateur) |
| Permissions RETIRED | ❌ Non visibles |

### 7.2. Visibilité Contextuelle

Certaines capacités et permissions peuvent avoir une visibilité restreinte selon le contexte :

| Niveau | Description | Règle |
|--------|-------------|-------|
| **Public** | Visible par tous | Aucune restriction |
| **Internal** | Visible par les composants internes | Requiert identité système |
| **Restricted** | Visible par les composants autorisés | Requiert permission spécifique |
| **Confidential** | Visible uniquement par le propriétaire | Requiert identité propriétaire |

### 7.3. Filtrage de Sécurité

Les capacités et permissions de niveau `SYSTEM` sont soumises à des règles de visibilité renforcées :

| Demandeur | Visibilité SYSTEM |
|-----------|-------------------|
| Opérateur standard | ❌ Non visible |
| Core système | ✅ Visible |
| MiyukiniAdmin | ✅ Visible |

**Règle :**

> **Les capacités et permissions SYSTEM ne sont jamais exposées aux Opérateurs standards, même par découverte.**

---

## 8. Interactions avec les Composants

### 8.1. Interaction avec BondingBrother

**Flux typique :**

```
BondingBrother traduit une intention
    │
    ├── Interroge Master Butler : DiscoverPermissionsForCapability("content.create")
    │       │
    │       └── Master Butler retourne les permissions associées
    │
    └── BondingBrother enrichit l'intention avec les permissions requises
```

**Règles :**

- BondingBrother utilise la découverte pour la traduction des intentions
- Les résultats de découverte alimentent le contexte de l'intention
- BondingBrother ne prend jamais de décision basée sur la découverte

### 8.2. Interaction avec StrongFather

**Flux typique :**

```
StrongFather évalue une intention
    │
    ├── Interroge Master Butler : DiscoverCapabilitiesForPermission(permission_id)
    │       │
    │       └── Master Butler retourne les capacités couvertes
    │
    └── StrongFather utilise ces informations pour l'évaluation
```

**Règles :**

- StrongFather a un accès complet à la découverte (incluant SYSTEM)
- La découverte informe l'évaluation mais ne la détermine pas
- StrongFather peut interroger sans restriction de visibilité

### 8.3. Interaction avec les Opérateurs

**Flux typique :**

```
Opérateur explore les capacités disponibles
    │
    ├── Interroge Master Butler : DiscoverByModule("spm.cms.content")
    │       │
    │       └── Master Butler retourne les capacités (filtrées par contexte)
    │
    └── Opérateur utilise ces informations pour adapter son comportement
```

**Règles :**

- Les Opérateurs voient uniquement les capacités/permissions selon leur contexte
- La visibilité SYSTEM est masquée pour les Opérateurs standards
- Les Opérateurs peuvent découvrir pour adapter leur comportement, jamais pour contourner

---

## 9. Schémas ASCII

### 9.1. Architecture de l'API de Découverte

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        DISCOVERY API ARCHITECTURE                            │
└─────────────────────────────────────────────────────────────────────────────┘

                              ┌─────────────────┐
                              │    DEMANDEURS   │
                              └─────────────────┘
                                      │
          ┌───────────────────────────┼───────────────────────────┐
          │                           │                           │
          ▼                           ▼                           ▼
   ┌─────────────┐            ┌─────────────┐            ┌─────────────┐
   │BondingBrother│           │ StrongFather │           │  Opérateurs │
   │ (Traduction)│            │  (Décision)  │           │ (Adaptation)│
   └─────────────┘            └─────────────┘            └─────────────┘
          │                           │                           │
          └───────────────────────────┼───────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         MASTER BUTLER - DISCOVERY API                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                        OPÉRATIONS DE DÉCOUVERTE                        │ │
│  ├────────────────────────────────────────────────────────────────────────┤ │
│  │                                                                          │ │
│  │  DiscoverByModule      DiscoverByAction      DiscoverByCategory         │ │
│  │         │                     │                     │                   │ │
│  │         ▼                     ▼                     ▼                   │ │
│  │  ┌──────────────────────────────────────────────────────────────────┐  │ │
│  │  │                      MOTEUR DE FILTRAGE                           │  │ │
│  │  │  • Filtres de statut, catégorie, niveau                          │  │ │
│  │  │  • Filtrage de visibilité (contextuel)                           │  │ │
│  │  │  • Pagination                                                     │  │ │
│  │  └──────────────────────────────────────────────────────────────────┘  │ │
│  │                               │                                         │ │
│  │  DiscoverPermissionsForCapability    DiscoverCapabilitiesForPermission │ │
│  │         │                                     │                         │ │
│  │         ▼                                     ▼                         │ │
│  │  ┌──────────────────────────────────────────────────────────────────┐  │ │
│  │  │                   RÉSOLUTION DES ASSOCIATIONS                     │  │ │
│  │  │  • Résolution directe                                             │  │ │
│  │  │  • Résolution des implications                                    │  │ │
│  │  └──────────────────────────────────────────────────────────────────┘  │ │
│  │                               │                                         │ │
│  │  ComputeCapabilityContext    │                                         │ │
│  │         │                    │                                         │ │
│  │         ▼                    ▼                                         │ │
│  │  ┌──────────────────────────────────────────────────────────────────┐  │ │
│  │  │                    CALCUL DU CONTEXTE                             │  │ │
│  │  │  • Intersection rôles-permissions                                 │  │ │
│  │  │  • Application des contraintes de sécurité                        │  │ │
│  │  │  • Génération des restrictions                                    │  │ │
│  │  └──────────────────────────────────────────────────────────────────┘  │ │
│  │                                                                          │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                      │                                       │
│                                      ▼                                       │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                           SOURCES DE DONNÉES                           │ │
│  ├────────────────────────────────────────────────────────────────────────┤ │
│  │                                                                          │ │
│  │   ┌─────────────────────┐           ┌─────────────────────┐            │ │
│  │   │  CAPABILITY REGISTRY │           │ PERMISSION REGISTRY │            │ │
│  │   │                       │           │                     │            │ │
│  │   │  • Capacités         │◄─────────►│  • Permissions      │            │ │
│  │   │  • Index par module  │           │  • Associations     │            │ │
│  │   │  • Index par catégorie│          │  • Hiérarchies      │            │ │
│  │   │  • Relations         │           │                     │            │ │
│  │   └─────────────────────┘           └─────────────────────┘            │ │
│  │                                                                          │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 9.2. Flux de Découverte Typique

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        FLUX DE DÉCOUVERTE TYPIQUE                            │
└─────────────────────────────────────────────────────────────────────────────┘

   DEMANDEUR                   DISCOVERY API                   REGISTRES
       │                            │                              │
       │  DiscoverByModule(         │                              │
       │    module_id: "spm.cms",   │                              │
       │    filter: {status: Active}│                              │
       │  )                         │                              │
       ├───────────────────────────►│                              │
       │                            │                              │
       │                            │  1. Valider la requête       │
       │                            │                              │
       │                            │  2. Vérifier visibilité      │
       │                            │     demandeur                │
       │                            │                              │
       │                            │  3. Interroger registre      │
       │                            ├─────────────────────────────►│
       │                            │                              │
       │                            │  4. Appliquer filtres        │
       │                            │◄─────────────────────────────┤
       │                            │     [Capacités brutes]       │
       │                            │                              │
       │                            │  5. Filtrer par visibilité   │
       │                            │                              │
       │                            │  6. Enrichir résumés         │
       │                            │                              │
       │                            │  7. Paginer résultats        │
       │                            │                              │
       │  DiscoveryResponse {       │                              │
       │    success: true,          │                              │
       │    data: {                 │                              │
       │      capabilities: [...],  │                              │
       │      total_count: 8        │                              │
       │    }                       │                              │
       │  }                         │                              │
       │◄───────────────────────────┤                              │
       │                            │                              │
       ▼                            ▼                              ▼
```

### 9.3. Calcul du Contexte de Capacité

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                   CALCUL DU CONTEXTE DE CAPACITÉ                             │
└─────────────────────────────────────────────────────────────────────────────┘

                    ContextSpecification
                           │
                           │
    ┌──────────────────────┼──────────────────────┐
    │                      │                      │
    ▼                      ▼                      ▼
┌─────────┐          ┌─────────┐          ┌─────────────┐
│Requester│          │  Roles  │          │   Target    │
│ Identity│          │  List   │          │Specification│
└─────────┘          └─────────┘          └─────────────┘
    │                      │                      │
    │                      │                      │
    ▼                      ▼                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                        MOTEUR DE CALCUL DE CONTEXTE                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│   1. Récupérer toutes les capacités du target (si spécifié)                 │
│                            │                                                 │
│                            ▼                                                 │
│   2. Récupérer les permissions des rôles                                    │
│                            │                                                 │
│                            ▼                                                 │
│   3. Résoudre les capacités couvertes par ces permissions                   │
│                            │                                                 │
│                            ▼                                                 │
│   4. Intersection avec capacités du target                                  │
│                            │                                                 │
│                            ▼                                                 │
│   5. Appliquer contraintes de sécurité                                      │
│       • Exclure capacités au-dessus du niveau autorisé                      │
│       • Marquer les restrictions                                            │
│                            │                                                 │
│                            ▼                                                 │
│   6. Générer le CapabilityContext                                           │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
                            │
                            ▼
              ┌─────────────────────────┐
              │    CapabilityContext    │
              ├─────────────────────────┤
              │ accessible_capabilities │
              │ accessible_permissions  │
              │ restrictions            │
              │ computed_at             │
              └─────────────────────────┘
```

---

## 10. Invariants Non Négociables

### INV-DISC-1 : Lecture Seule

> **La découverte ne modifie jamais les registres.**

**Implications :**
- Aucune opération de découverte ne crée, modifie ou supprime de données
- Les registres sont inchangés après une découverte
- La découverte est idempotente (mêmes paramètres = mêmes résultats)

### INV-DISC-2 : Non-Décision

> **La découverte informe mais ne décide jamais.**

**Implications :**
- Aucune réponse de découverte ne contient "autorisé" ou "refusé"
- Le contexte calculé est une projection, pas une autorisation
- La décision appartient exclusivement à StrongFather

### INV-DISC-3 : Exhaustivité selon Visibilité

> **La découverte retourne tous les éléments visibles selon le contexte demandeur.**

**Implications :**
- Aucune capacité ou permission visible n'est omise
- Les filtres réduisent mais ne cachent pas arbitrairement
- La visibilité est déterminée par des règles explicites

### INV-DISC-4 : Cohérence Temporelle

> **Les résultats de découverte sont cohérents à l'instant de la requête.**

**Implications :**
- Snapshot cohérent des registres au moment de la requête
- Pas d'état intermédiaire visible
- Les modifications pendant la requête n'affectent pas le résultat

### INV-DISC-5 : Traçabilité des Requêtes

> **Toutes les requêtes de découverte sont journalisées.**

**Implications :**
- Chaque requête a un identifiant unique (request_id)
- L'identité du demandeur est enregistrée
- L'audit des découvertes est possible

### INV-DISC-6 : Respect des Contraintes de Sécurité

> **Les contraintes de sécurité sont toujours appliquées, sans exception.**

**Implications :**
- Les capacités SYSTEM ne sont jamais exposées aux non-autorisés
- Le niveau de sécurité du demandeur est toujours vérifié
- Aucun contournement possible par la découverte

---

## 11. Conformité aux Lois d'Autonomie Système

Ce contrat respecte les Lois d'Autonomie Système définies dans [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md).

### LOI-1 : Aucune dépendance externe critique à l'exécution

**Conformité :** Conforme

La Discovery API opère entièrement sur des données locales :

- **Registres locaux** : Les capacités et permissions sont stockées localement
- **Calculs locaux** : Le calcul de contexte n'utilise aucune ressource externe
- **Aucune API externe** : La découverte ne dépend d'aucun service distant

**Vérification LOI-1** : *"La Discovery API fonctionne-t-elle si le réseau est indisponible ?"* → **Oui.** Toutes les opérations sont locales.

### LOI-5 : Le coût doit être proportionnel au hardware

**Conformité :** Conforme

La Discovery API a une empreinte minimale :

- **Opérations de lecture** : Pas de computation intensive
- **Index existants** : Utilise les index des registres existants
- **Pagination** : Résultats limités pour contrôler la mémoire
- **Pas de cache permanent** : Les contextes sont calculés à la demande

**Vérification LOI-5** : *"La Discovery API fonctionne-t-elle sur un Raspberry Pi 4 ?"* → **Oui.** Les opérations de découverte sont des lectures simples avec filtrage.

### Synthèse de conformité

| Loi | Statut | Raison |
|-----|--------|--------|
| LOI-1 | ✅ Conforme | Opérations locales, aucune dépendance externe |
| LOI-5 | ✅ Conforme | Lectures simples, pagination, pas de cache lourd |

---

## 12. Exemples Complets

### 12.1. Exemple : Découverte des capacités d'un module CMS

```yaml
# Requête
request:
  operation: DiscoverByModule
  module_id: "spm.cms.content"
  filter:
    status: ["Active"]
    category: ["Data"]
  pagination:
    offset: 0
    limit: 10
    sort_by: "id"
    sort_order: "ASC"

# Réponse
response:
  success: true
  data:
    capabilities:
      - id: "content.create"
        name: "Créer du contenu"
        description: "Capacité de créer un nouveau contenu"
        category: "Data"
        status: "Active"
        source: "spm.cms.content"
        associated_permissions:
          - "content.create.any"
          - "content.create.own"
      - id: "content.read"
        name: "Lire du contenu"
        description: "Capacité de lire le contenu existant"
        category: "Data"
        status: "Active"
        source: "spm.cms.content"
        associated_permissions:
          - "content.read.any"
          - "content.read.own"
      - id: "content.edit"
        name: "Modifier du contenu"
        description: "Capacité de modifier un contenu existant"
        category: "Data"
        status: "Active"
        source: "spm.cms.content"
        associated_permissions:
          - "content.edit.any"
          - "content.edit.own"
    total_count: 8
    filtered_count: 3
    has_more: true
  metadata:
    request_id: "disc_001"
    timestamp: "2026-01-27T10:30:00Z"
    source: "MasterButler.DiscoveryAPI"
    version: "1.0"
    processing_time_ms: 15
```

### 12.2. Exemple : Calcul de contexte pour un éditeur de contenu

```yaml
# Requête
request:
  operation: ComputeCapabilityContext
  context:
    requester:
      type: "User"
      id: "user_editor_001"
    roles:
      - "content_editor"
      - "media_viewer"
    target:
      module: "spm.cms.content"
    security_constraints:
      max_level: "ELEVATED"

# Réponse
response:
  success: true
  data:
    requester:
      type: "User"
      id: "user_editor_001"
    accessible_capabilities:
      - id: "content.create"
        name: "Créer du contenu"
        category: "Data"
      - id: "content.edit"
        name: "Modifier du contenu"
        category: "Data"
      - id: "content.read"
        name: "Lire du contenu"
        category: "Data"
      - id: "content.publish"
        name: "Publier du contenu"
        category: "Data"
    accessible_permissions:
      - id: "content.create.own"
        name: "Créer son propre contenu"
        level: "STANDARD"
        scope_type: "OWNED"
      - id: "content.edit.own"
        name: "Modifier son propre contenu"
        level: "STANDARD"
        scope_type: "OWNED"
      - id: "content.edit.team"
        name: "Modifier le contenu de l'équipe"
        level: "ELEVATED"
        scope_type: "SCOPED"
      - id: "content.publish.own"
        name: "Publier son propre contenu"
        level: "STANDARD"
        scope_type: "OWNED"
    restrictions:
      - type: "SECURITY_LEVEL"
        reason: "CRITICAL level excluded by security_constraints"
        affected_capabilities:
          - "content.delete"
      - type: "ROLE_MISSING"
        reason: "Role 'content_admin' required"
        affected_capabilities:
          - "content.manage.all"
    computed_at: "2026-01-27T10:35:00Z"
    valid_for: null
  metadata:
    request_id: "disc_ctx_001"
    timestamp: "2026-01-27T10:35:00Z"
    source: "MasterButler.DiscoveryAPI"
    version: "1.0"
    processing_time_ms: 28
```

### 12.3. Exemple : Découverte des permissions pour une capacité

```yaml
# Requête
request:
  operation: DiscoverPermissionsForCapability
  capability_id: "media.upload"
  include_implied: true

# Réponse
response:
  success: true
  data:
    capability:
      id: "media.upload"
      name: "Téléverser un média"
      description: "Capacité de téléverser des fichiers médias"
      category: "Media"
      status: "Active"
    direct_permissions:
      - id: "media.upload.any"
        name: "Téléverser n'importe quel média"
        level: "ELEVATED"
        scope_type: "GLOBAL"
        status: "Active"
      - id: "media.upload.own"
        name: "Téléverser ses propres médias"
        level: "STANDARD"
        scope_type: "OWNED"
        status: "Active"
      - id: "media.upload.team"
        name: "Téléverser pour l'équipe"
        level: "STANDARD"
        scope_type: "SCOPED"
        status: "Active"
    implied_permissions:
      - id: "media.manage.all"
        name: "Gestion complète des médias"
        level: "CRITICAL"
        scope_type: "GLOBAL"
        status: "Active"
        implies: ["media.upload.any", "media.delete.any", "media.edit.any"]
  metadata:
    request_id: "disc_perm_001"
    timestamp: "2026-01-27T10:40:00Z"
    source: "MasterButler.DiscoveryAPI"
    version: "1.0"
    processing_time_ms: 18
```

---

## 13. Conclusion et Statut Contractuel

### Essence de la Discovery API

La Discovery API de Master Butler est le mécanisme par lequel les composants du système Miyukini peuvent explorer les capacités et permissions disponibles. Elle permet une adaptation dynamique des comportements sans jamais participer aux décisions d'autorisation.

Cette API incarne le principe fondateur de Master Butler : **exposer ce qui est possible, sans jamais décider ce qui est autorisé**.

### Phrase fondatrice

> **La Discovery API permet aux composants de découvrir les possibilités du système Miyukini de manière exhaustive, filtrée par contexte, sans jamais constituer une autorisation.**

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

Toute implémentation de la Discovery API doit respecter intégralement ce document. Toute évolution doit préserver les invariants définis ici.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** FONDATION — Non négociable  
**Référence :** Miyukini Core System v2.4

**Références croisées :**

- [Master Butler - Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md) : Définition et responsabilités de Master Butler
- [Master Butler - Capability Registry Contract](../registry/Master%20Butler%20-%20Capability%20Registry%20Contract.md) : Registre des capacités
- [Master Butler - Permission Registry Contract](../registry/Master%20Butler%20-%20Permission%20Registry%20Contract.md) : Registre des permissions
- [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) : Définitions canoniques
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) : Lois d'autonomie

---

## 14. Mini log — erreurs / warnings / ambiguïtés rencontrées et corrigées

### Ambiguïté A1 : Scope de la découverte

**Ambiguïté rencontrée :**
La documentation fondatrice mentionne "API de découverte des capacités par module" et "API de découverte des capacités par type d'action" sans définir clairement si ce sont des opérations distinctes ou des filtres d'une même opération.

**Décision prise :**
Opérations distinctes définies : DiscoverByModule, DiscoverByAction, DiscoverByCategory, avec des signatures et comportements spécifiques pour chaque mode de découverte.

**Justification :**
Des opérations distinctes permettent une API plus claire et typée, avec des paramètres et validations spécifiques à chaque mode.

**Correction effectuée :**
Section 3 "Modes de Découverte" avec opérations séparées et signatures conceptuelles.

### Ambiguïté A2 : Visibilité contextuelle

**Ambiguïté rencontrée :**
La documentation fondatrice mentionne "filtrage des capacités selon le contexte" sans définir les niveaux de visibilité ni les règles de filtrage.

**Décision prise :**
Quatre niveaux de visibilité définis (Public, Internal, Restricted, Confidential) avec des règles explicites de filtrage basées sur l'identité et le niveau de sécurité du demandeur.

**Justification :**
Des niveaux de visibilité explicites garantissent une sécurité cohérente et prévisible.

**Correction effectuée :**
Section 7 "Règles de Visibilité" avec niveaux et règles de filtrage.

### Ambiguïté A3 : Contexte de capacité

**Ambiguïté rencontrée :**
Le "contexte de capacité" est mentionné dans la documentation fondatrice avec une liste d'éléments inclus, mais sans définition formelle de la structure ni du mécanisme de calcul.

**Décision prise :**
Structure formelle définie (ContextSpecification, CapabilityContext) avec un algorithme de calcul en 6 étapes clairement documenté.

**Justification :**
Une structure et un algorithme formels garantissent un comportement prévisible et vérifiable.

**Correction effectuée :**
Section 4 "Contexte de Capacité" avec structures de données et règles de calcul.

### Ambiguïté A4 : Réponses d'erreur

**Ambiguïté rencontrée :**
Aucune documentation existante ne définit les codes d'erreur spécifiques à la Discovery API.

**Décision prise :**
Catalogue de 9 codes d'erreur définis (DISC-001 à DISC-009) couvrant les cas d'erreur typiques.

**Justification :**
Des codes d'erreur standardisés facilitent le diagnostic et le traitement des erreurs.

**Correction effectuée :**
Section 6.2 "Codes d'Erreur" avec codes et descriptions.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
