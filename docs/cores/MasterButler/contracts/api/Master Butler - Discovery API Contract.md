# Master Butler â€” Discovery API Contract

## 1. Introduction

### Objet du contrat

Ce document dÃ©finit le **Master Butler Discovery API Contract** : un contrat normatif, non nÃ©gociable, et de statut FONDATION qui Ã©tablit l'API de dÃ©couverte des capacitÃ©s et permissions dans le systÃ¨me Miyukini Core System v2.4.

Ce contrat dÃ©finit :

- Les opÃ©rations de dÃ©couverte disponibles
- Les modes de dÃ©couverte (par module, par type, par contexte)
- Les rÃ¨gles de filtrage et de visibilitÃ©
- Les rÃ©ponses standardisÃ©es
- Les invariants de l'API de dÃ©couverte
- Les interactions avec les autres composants

### PortÃ©e

Ce contrat s'applique Ã  **toute opÃ©ration de dÃ©couverte** via Master Butler et dÃ©finit de maniÃ¨re absolue :

- La dÃ©couverte des capacitÃ©s par module
- La dÃ©couverte des capacitÃ©s par type d'action
- La dÃ©couverte des permissions par capacitÃ©
- La dÃ©couverte du contexte de capacitÃ©
- Le filtrage selon le contexte demandeur
- Les rÃ¨gles de visibilitÃ© et d'accÃ¨s

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il Ã©tablit des rÃ¨gles absolues que Master Butler applique sans exception. Ces rÃ¨gles ne peuvent Ãªtre contournÃ©es, nÃ©gociÃ©es, ou modifiÃ©es. Le contrat prime sur toute considÃ©ration pratique.

### Relation avec les autres contrats

Ce contrat complÃ¨te les documents contractuels existants :

- **[Master Butler - Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md)** : DÃ©finit la nature, le rÃ´le, et les responsabilitÃ©s de Master Butler
- **[Master Butler - Capability Registry Contract](../registry/Master%20Butler%20-%20Capability%20Registry%20Contract.md)** : DÃ©finit le registre des capacitÃ©s (source des donnÃ©es dÃ©couvertes)
- **[Master Butler - Permission Registry Contract](../registry/Master%20Butler%20-%20Permission%20Registry%20Contract.md)** : DÃ©finit le registre des permissions (source des donnÃ©es dÃ©couvertes)
- **[Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)** : DÃ©finitions canoniques des termes
- **[Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)** : Ce contrat respecte **LOI-1** (aucune dÃ©pendance externe critique) et **LOI-5** (coÃ»t proportionnel au hardware)

**ComplÃ©mentaritÃ© :**

- Capability Registry Contract = structure et gestion des capacitÃ©s
- Permission Registry Contract = structure et gestion des permissions
- Discovery API Contract = mÃ©canismes de dÃ©couverte et d'exploration

Ces contrats forment ensemble le systÃ¨me complet de gestion et d'exploration des capacitÃ©s et permissions.

---

## 2. DÃ©finition de la DÃ©couverte

### DÃ©finition canonique

La **dÃ©couverte** est le processus par lequel un composant interroge Master Butler pour connaÃ®tre les capacitÃ©s et permissions existantes dans le systÃ¨me. La dÃ©couverte permet l'exploration dynamique des possibilitÃ©s du systÃ¨me.

**Phrase fondatrice :**

> **La dÃ©couverte expose ce qui est possible, sans jamais suggÃ©rer ce qui est autorisÃ©.**

### CaractÃ©ristiques de la dÃ©couverte

| CaractÃ©ristique | Description |
|-----------------|-------------|
| **Non-intrusive** | La dÃ©couverte ne modifie pas les registres |
| **Contextuelle** | Les rÃ©sultats peuvent Ãªtre filtrÃ©s selon le contexte |
| **Exhaustive** | Retourne toutes les informations pertinentes selon les critÃ¨res |
| **TraÃ§able** | Les requÃªtes de dÃ©couverte sont journalisÃ©es |
| **Accessible** | Disponible pour tous les composants autorisÃ©s |

### Nature de la dÃ©couverte

La dÃ©couverte est une opÃ©ration de **lecture seule** qui permet aux composants de :

1. Explorer les capacitÃ©s disponibles dans le systÃ¨me
2. Identifier les permissions associÃ©es Ã  des capacitÃ©s
3. Calculer les capacitÃ©s accessibles dans un contexte donnÃ©
4. Comprendre la structure des modules et leurs capacitÃ©s

**Important :** La dÃ©couverte fournit des informations, mais ne constitue jamais une autorisation. La dÃ©cision d'autorisation appartient exclusivement Ã  StrongFather.

### Distinction dÃ©couverte vs autorisation

| Aspect | DÃ©couverte | Autorisation |
|--------|------------|--------------|
| **DÃ©finition** | Exploration de ce qui existe | DÃ©cision d'accorder un accÃ¨s |
| **Responsable** | Master Butler | StrongFather |
| **Nature** | Lecture seule, informative | DÃ©cisionnelle |
| **Question** | "Qu'existe-t-il ?" | "Est-ce permis ?" |
| **Modification** | Aucune | Peut modifier l'Ã©tat d'autorisation |

---

## 3. Modes de DÃ©couverte

### 3.1. DÃ©couverte par Module (DiscoverByModule)

**Ã‰noncÃ© :**

L'opÃ©ration **DiscoverByModule** permet de dÃ©couvrir toutes les capacitÃ©s dÃ©clarÃ©es par un module spÃ©cifique.

**Signature conceptuelle :**

```
DiscoverByModule(
  module_id: ModuleIdentifier,
  filter: DiscoveryFilter?
) â†’ Result<DiscoveryResult, DiscoveryError>
```

**ParamÃ¨tres :**

| ParamÃ¨tre | Type | Obligatoire | Description |
|-----------|------|-------------|-------------|
| `module_id` | ModuleIdentifier | âœ… Oui | Identifiant du module source |
| `filter` | DiscoveryFilter | âŒ Non | Filtres optionnels (statut, catÃ©gorie) |

**RÃ©sultat :**

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
# DÃ©couvrir toutes les capacitÃ©s du module CMS Content
request:
  operation: DiscoverByModule
  module_id: "spm.cms.content"
  filter: null

response:
  success: true
  data:
    capabilities:
      - id: "content.create"
        name: "CrÃ©er du contenu"
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

**RÃ¨gles :**

- R-DBM-1 : Le module doit exister dans le registre
- R-DBM-2 : Seules les capacitÃ©s avec statut correspondant au filtre sont retournÃ©es
- R-DBM-3 : Les capacitÃ©s confidentielles sont filtrÃ©es selon le contexte demandeur

### 3.2. DÃ©couverte par Type d'Action (DiscoverByAction)

**Ã‰noncÃ© :**

L'opÃ©ration **DiscoverByAction** permet de dÃ©couvrir toutes les capacitÃ©s correspondant Ã  un type d'action donnÃ©.

**Signature conceptuelle :**

```
DiscoverByAction(
  action_type: ActionType,
  domain: DomainIdentifier?,
  filter: DiscoveryFilter?
) â†’ Result<DiscoveryResult, DiscoveryError>
```

**ParamÃ¨tres :**

| ParamÃ¨tre | Type | Obligatoire | Description |
|-----------|------|-------------|-------------|
| `action_type` | ActionType | âœ… Oui | Type d'action (create, read, update, delete, etc.) |
| `domain` | DomainIdentifier | âŒ Non | Domaine de filtrage optionnel |
| `filter` | DiscoveryFilter | âŒ Non | Filtres additionnels |

**Types d'actions standards :**

| ActionType | Description | Exemples de capacitÃ©s |
|------------|-------------|----------------------|
| `create` | Actions de crÃ©ation | `content.create`, `media.upload` |
| `read` | Actions de lecture | `content.read`, `user.profile.view` |
| `update` | Actions de modification | `content.edit`, `hierarchy.reorder` |
| `delete` | Actions de suppression | `content.delete`, `media.remove` |
| `publish` | Actions de publication | `content.publish`, `media.publish` |
| `manage` | Actions de gestion | `content.manage`, `user.manage` |
| `search` | Actions de recherche | `search.query`, `search.index` |
| `export` | Actions d'export | `data.export`, `report.generate` |

**Exemples d'utilisation :**

```yaml
# DÃ©couvrir toutes les capacitÃ©s de type "create" dans le domaine "content"
request:
  operation: DiscoverByAction
  action_type: "create"
  domain: "content"

response:
  success: true
  data:
    capabilities:
      - id: "content.create"
        name: "CrÃ©er du contenu"
        source: "spm.cms.content"
      - id: "content.draft.create"
        name: "CrÃ©er un brouillon"
        source: "spm.cms.content"
    total_count: 2
```

**RÃ¨gles :**

- R-DBA-1 : Le type d'action est obligatoire
- R-DBA-2 : Le domaine est optionnel et sert de filtre
- R-DBA-3 : La correspondance est basÃ©e sur le segment d'action dans l'identifiant

### 3.3. DÃ©couverte par CatÃ©gorie (DiscoverByCategory)

**Ã‰noncÃ© :**

L'opÃ©ration **DiscoverByCategory** permet de dÃ©couvrir toutes les capacitÃ©s d'une catÃ©gorie fonctionnelle donnÃ©e.

**Signature conceptuelle :**

```
DiscoverByCategory(
  category: CapabilityCategory,
  filter: DiscoveryFilter?
) â†’ Result<DiscoveryResult, DiscoveryError>
```

**CatÃ©gories supportÃ©es :**

| CatÃ©gorie | Description |
|-----------|-------------|
| `Data` | CapacitÃ©s liÃ©es aux donnÃ©es |
| `Hierarchy` | CapacitÃ©s liÃ©es aux hiÃ©rarchies |
| `Media` | CapacitÃ©s liÃ©es aux mÃ©dias |
| `Search` | CapacitÃ©s liÃ©es Ã  la recherche |
| `Auth` | CapacitÃ©s liÃ©es Ã  l'authentification |
| `Admin` | CapacitÃ©s d'administration |
| `UI` | CapacitÃ©s d'interface utilisateur |
| `IO` | CapacitÃ©s d'entrÃ©e/sortie |
| `System` | CapacitÃ©s systÃ¨me |

**Exemples d'utilisation :**

```yaml
# DÃ©couvrir toutes les capacitÃ©s de la catÃ©gorie "Media"
request:
  operation: DiscoverByCategory
  category: "Media"

response:
  success: true
  data:
    capabilities:
      - id: "media.upload"
        name: "TÃ©lÃ©verser un mÃ©dia"
        source: "spm.cms.media"
      - id: "media.delete"
        name: "Supprimer un mÃ©dia"
        source: "spm.cms.media"
      - id: "media.transform"
        name: "Transformer un mÃ©dia"
        source: "spm.cms.media"
    total_count: 6
```

### 3.4. DÃ©couverte des Permissions par CapacitÃ© (DiscoverPermissionsForCapability)

**Ã‰noncÃ© :**

L'opÃ©ration **DiscoverPermissionsForCapability** permet de dÃ©couvrir toutes les permissions qui couvrent une capacitÃ© donnÃ©e.

**Signature conceptuelle :**

```
DiscoverPermissionsForCapability(
  capability_id: CapabilityId,
  include_implied: Boolean?
) â†’ Result<PermissionDiscoveryResult, DiscoveryError>
```

**ParamÃ¨tres :**

| ParamÃ¨tre | Type | Obligatoire | Description |
|-----------|------|-------------|-------------|
| `capability_id` | CapabilityId | âœ… Oui | Identifiant de la capacitÃ© |
| `include_implied` | Boolean | âŒ Non | Inclure les permissions impliquant d'autres permissions (dÃ©faut: true) |

**RÃ©sultat :**

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
# DÃ©couvrir les permissions pour la capacitÃ© "content.create"
request:
  operation: DiscoverPermissionsForCapability
  capability_id: "content.create"
  include_implied: true

response:
  success: true
  data:
    capability:
      id: "content.create"
      name: "CrÃ©er du contenu"
    direct_permissions:
      - id: "content.create.any"
        name: "CrÃ©er n'importe quel contenu"
        level: "ELEVATED"
        scope_type: "GLOBAL"
      - id: "content.create.own"
        name: "CrÃ©er son propre contenu"
        level: "STANDARD"
        scope_type: "OWNED"
    implied_permissions:
      - id: "content.manage.all"
        name: "Gestion complÃ¨te du contenu"
        level: "CRITICAL"
        # Implique content.create.any
```

**RÃ¨gles :**

- R-DPC-1 : La capacitÃ© doit exister dans le registre
- R-DPC-2 : Les permissions RETIRED ne sont pas incluses par dÃ©faut
- R-DPC-3 : Les permissions impliquÃ©es sont rÃ©solues rÃ©cursivement si include_implied=true

### 3.5. DÃ©couverte des CapacitÃ©s par Permission (DiscoverCapabilitiesForPermission)

**Ã‰noncÃ© :**

L'opÃ©ration **DiscoverCapabilitiesForPermission** permet de dÃ©couvrir toutes les capacitÃ©s couvertes par une permission donnÃ©e.

**Signature conceptuelle :**

```
DiscoverCapabilitiesForPermission(
  permission_id: PermissionId,
  resolve_implied: Boolean?
) â†’ Result<CapabilityDiscoveryResult, DiscoveryError>
```

**ParamÃ¨tres :**

| ParamÃ¨tre | Type | Obligatoire | Description |
|-----------|------|-------------|-------------|
| `permission_id` | PermissionId | âœ… Oui | Identifiant de la permission |
| `resolve_implied` | Boolean | âŒ Non | RÃ©soudre les permissions impliquÃ©es (dÃ©faut: true) |

**RÃ©sultat :**

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
# DÃ©couvrir les capacitÃ©s couvertes par "content.manage.all"
request:
  operation: DiscoverCapabilitiesForPermission
  permission_id: "content.manage.all"
  resolve_implied: true

response:
  success: true
  data:
    permission:
      id: "content.manage.all"
      name: "Gestion complÃ¨te du contenu"
    direct_capabilities: []
    implied_capabilities:
      - id: "content.create"
        name: "CrÃ©er du contenu"
      - id: "content.edit"
        name: "Modifier du contenu"
      - id: "content.delete"
        name: "Supprimer du contenu"
```

---

## 4. Contexte de CapacitÃ©

### 4.1. DÃ©finition

Le **contexte de capacitÃ©** est l'ensemble des informations qui dÃ©finissent les capacitÃ©s et permissions disponibles dans une situation donnÃ©e.

**Composition du contexte :**

| Ã‰lÃ©ment | Description |
|---------|-------------|
| `requester_identity` | IdentitÃ© du demandeur (utilisateur, systÃ¨me, produit) |
| `requester_roles` | RÃ´les du demandeur |
| `target_module` | Module ou composant ciblÃ© |
| `security_level` | Niveau de sÃ©curitÃ© courant |
| `environment` | Environnement d'exÃ©cution |

### 4.2. Calcul du Contexte de CapacitÃ© (ComputeCapabilityContext)

**Ã‰noncÃ© :**

L'opÃ©ration **ComputeCapabilityContext** permet de calculer les capacitÃ©s et permissions accessibles dans un contexte donnÃ©.

**Signature conceptuelle :**

```
ComputeCapabilityContext(
  context: ContextSpecification
) â†’ Result<CapabilityContext, DiscoveryError>
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

**RÃ©sultat :**

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
# Calculer le contexte pour un utilisateur avec le rÃ´le "editor"
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
        name: "CrÃ©er du contenu"
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

**RÃ¨gles :**

- R-CCC-1 : Le calcul de contexte est une projection, jamais une dÃ©cision
- R-CCC-2 : Le contexte est calculÃ© Ã  la demande, jamais mis en cache dÃ©cisionnel
- R-CCC-3 : Les restrictions de sÃ©curitÃ© sont toujours appliquÃ©es
- R-CCC-4 : Le rÃ©sultat ne modifie pas les registres

### 4.3. RÃ¨gles de Calcul du Contexte

| RÃ¨gle | Description | Impact |
|-------|-------------|--------|
| **R-CTX-1** | Les capacitÃ©s DEPRECATED sont incluses avec avertissement | VisibilitÃ© avec notice |
| **R-CTX-2** | Les capacitÃ©s REMOVED sont exclues | Non visibles |
| **R-CTX-3** | Les permissions DRAFT sont exclues | Non utilisables |
| **R-CTX-4** | Le niveau de sÃ©curitÃ© filtre les capacitÃ©s | Exclusion si niveau insuffisant |
| **R-CTX-5** | Les rÃ´les dÃ©terminent les permissions accessibles | Filtrage par association |

---

## 5. Filtres de DÃ©couverte

### 5.1. Structure du Filtre

```
DiscoveryFilter {
  status: List<Status>?,        // Active, Deprecated, Removed
  category: List<Category>?,    // Data, Media, etc.
  level: List<Level>?,          // STANDARD, ELEVATED, CRITICAL, SYSTEM
  scope_type: List<ScopeType>?, // GLOBAL, SCOPED, OWNED, CONTEXTUAL
  tags: List<String>?,          // Tags de recherche
  created_after: Timestamp?,    // Date de crÃ©ation minimum
  created_before: Timestamp?,   // Date de crÃ©ation maximum
  source_type: List<SourceType>?, // Module, Core, Operator, Tool
  search_query: String?         // Recherche textuelle
}
```

### 5.2. Application des Filtres

Les filtres sont combinÃ©s avec une logique **AND** :

```yaml
# Exemple : CapacitÃ©s Active OU Deprecated, de catÃ©gorie Data, de niveau STANDARD
filter:
  status: ["Active", "Deprecated"]  # Active OR Deprecated
  category: ["Data"]                 # AND category = Data
  level: ["STANDARD"]                # AND level = STANDARD
# RÃ©sultat : (status IN [Active, Deprecated]) AND (category = Data) AND (level = STANDARD)
```

### 5.3. Pagination

Les rÃ©sultats de dÃ©couverte supportent la pagination :

```
PaginationParams {
  offset: Integer,    // DÃ©calage (dÃ©faut: 0)
  limit: Integer,     // Nombre max de rÃ©sultats (dÃ©faut: 50, max: 500)
  sort_by: String,    // Champ de tri (dÃ©faut: "id")
  sort_order: SortOrder  // ASC ou DESC (dÃ©faut: ASC)
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

## 6. RÃ©ponses StandardisÃ©es

### 6.1. Structure de RÃ©ponse

Toutes les opÃ©rations de dÃ©couverte retournent une rÃ©ponse avec la structure suivante :

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
| `DISC-001` | `INVALID_FILTER` | Filtre invalide ou mal formÃ© |
| `DISC-002` | `UNKNOWN_MODULE` | Module non trouvÃ© dans le registre |
| `DISC-003` | `UNKNOWN_CAPABILITY` | CapacitÃ© non trouvÃ©e dans le registre |
| `DISC-004` | `UNKNOWN_PERMISSION` | Permission non trouvÃ©e dans le registre |
| `DISC-005` | `INVALID_CONTEXT` | Contexte de dÃ©couverte invalide |
| `DISC-006` | `ACCESS_DENIED` | AccÃ¨s refusÃ© au composant demandeur |
| `DISC-007` | `PAGINATION_ERROR` | ParamÃ¨tres de pagination invalides |
| `DISC-008` | `TIMEOUT` | Timeout de la requÃªte de dÃ©couverte |
| `DISC-009` | `INTERNAL_ERROR` | Erreur interne du registre |

### 6.3. Exemples de RÃ©ponses

**RÃ©ponse succÃ¨s :**

```yaml
response:
  success: true
  data:
    capabilities:
      - id: "content.create"
        name: "CrÃ©er du contenu"
    total_count: 1
  metadata:
    request_id: "req_abc123"
    timestamp: "2026-01-27T10:30:00Z"
    source: "MasterButler.DiscoveryAPI"
    version: "1.0"
    processing_time_ms: 12
  errors: null
```

**RÃ©ponse erreur :**

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

## 7. RÃ¨gles de VisibilitÃ©

### 7.1. VisibilitÃ© par DÃ©faut

| Ã‰lÃ©ment | VisibilitÃ© par dÃ©faut |
|---------|----------------------|
| CapacitÃ©s ACTIVE | âœ… Visibles |
| CapacitÃ©s DEPRECATED | âœ… Visibles (avec warning) |
| CapacitÃ©s REMOVED | âŒ Non visibles |
| Permissions ACTIVE | âœ… Visibles |
| Permissions DEPRECATED | âœ… Visibles (avec warning) |
| Permissions DRAFT | âŒ Non visibles (sauf crÃ©ateur) |
| Permissions RETIRED | âŒ Non visibles |

### 7.2. VisibilitÃ© Contextuelle

Certaines capacitÃ©s et permissions peuvent avoir une visibilitÃ© restreinte selon le contexte :

| Niveau | Description | RÃ¨gle |
|--------|-------------|-------|
| **Public** | Visible par tous | Aucune restriction |
| **Internal** | Visible par les composants internes | Requiert identitÃ© systÃ¨me |
| **Restricted** | Visible par les composants autorisÃ©s | Requiert permission spÃ©cifique |
| **Confidential** | Visible uniquement par le propriÃ©taire | Requiert identitÃ© propriÃ©taire |

### 7.3. Filtrage de SÃ©curitÃ©

Les capacitÃ©s et permissions de niveau `SYSTEM` sont soumises Ã  des rÃ¨gles de visibilitÃ© renforcÃ©es :

| Demandeur | VisibilitÃ© SYSTEM |
|-----------|-------------------|
| OpÃ©rateur standard | âŒ Non visible |
| Core systÃ¨me | âœ… Visible |
| MiyukiniAdmin | âœ… Visible |

**RÃ¨gle :**

> **Les capacitÃ©s et permissions SYSTEM ne sont jamais exposÃ©es aux OpÃ©rateurs standards, mÃªme par dÃ©couverte.**

---

## 8. Interactions avec les Composants

### 8.1. Interaction avec BondingBrother

**Flux typique :**

```
BondingBrother traduit une intention
    â”‚
    â”œâ”€â”€ Interroge Master Butler : DiscoverPermissionsForCapability("content.create")
    â”‚       â”‚
    â”‚       â””â”€â”€ Master Butler retourne les permissions associÃ©es
    â”‚
    â””â”€â”€ BondingBrother enrichit l'intention avec les permissions requises
```

**RÃ¨gles :**

- BondingBrother utilise la dÃ©couverte pour la traduction des intentions
- Les rÃ©sultats de dÃ©couverte alimentent le contexte de l'intention
- BondingBrother ne prend jamais de dÃ©cision basÃ©e sur la dÃ©couverte

### 8.2. Interaction avec StrongFather

**Flux typique :**

```
StrongFather Ã©value une intention
    â”‚
    â”œâ”€â”€ Interroge Master Butler : DiscoverCapabilitiesForPermission(permission_id)
    â”‚       â”‚
    â”‚       â””â”€â”€ Master Butler retourne les capacitÃ©s couvertes
    â”‚
    â””â”€â”€ StrongFather utilise ces informations pour l'Ã©valuation
```

**RÃ¨gles :**

- StrongFather a un accÃ¨s complet Ã  la dÃ©couverte (incluant SYSTEM)
- La dÃ©couverte informe l'Ã©valuation mais ne la dÃ©termine pas
- StrongFather peut interroger sans restriction de visibilitÃ©

### 8.3. Interaction avec les OpÃ©rateurs

**Flux typique :**

```
OpÃ©rateur explore les capacitÃ©s disponibles
    â”‚
    â”œâ”€â”€ Interroge Master Butler : DiscoverByModule("spm.cms.content")
    â”‚       â”‚
    â”‚       â””â”€â”€ Master Butler retourne les capacitÃ©s (filtrÃ©es par contexte)
    â”‚
    â””â”€â”€ OpÃ©rateur utilise ces informations pour adapter son comportement
```

**RÃ¨gles :**

- Les OpÃ©rateurs voient uniquement les capacitÃ©s/permissions selon leur contexte
- La visibilitÃ© SYSTEM est masquÃ©e pour les OpÃ©rateurs standards
- Les OpÃ©rateurs peuvent dÃ©couvrir pour adapter leur comportement, jamais pour contourner

---

## 9. SchÃ©mas ASCII

### 9.1. Architecture de l'API de DÃ©couverte

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                        DISCOVERY API ARCHITECTURE                            â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜

                              â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
                              â”‚    DEMANDEURS   â”‚
                              â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                                      â”‚
          â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
          â”‚                           â”‚                           â”‚
          â–¼                           â–¼                           â–¼
   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”            â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”            â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
   â”‚BondingBrotherâ”‚           â”‚ StrongFather â”‚           â”‚  OpÃ©rateurs â”‚
   â”‚ (Traduction)â”‚            â”‚  (DÃ©cision)  â”‚           â”‚ (Adaptation)â”‚
   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜            â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜            â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
          â”‚                           â”‚                           â”‚
          â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                                      â”‚
                                      â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                         MASTER BUTLER - DISCOVERY API                        â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                              â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚                        OPÃ‰RATIONS DE DÃ‰COUVERTE                        â”‚ â”‚
â”‚  â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤ â”‚
â”‚  â”‚                                                                          â”‚ â”‚
â”‚  â”‚  DiscoverByModule      DiscoverByAction      DiscoverByCategory         â”‚ â”‚
â”‚  â”‚         â”‚                     â”‚                     â”‚                   â”‚ â”‚
â”‚  â”‚         â–¼                     â–¼                     â–¼                   â”‚ â”‚
â”‚  â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚ â”‚
â”‚  â”‚  â”‚                      MOTEUR DE FILTRAGE                           â”‚  â”‚ â”‚
â”‚  â”‚  â”‚  â€¢ Filtres de statut, catÃ©gorie, niveau                          â”‚  â”‚ â”‚
â”‚  â”‚  â”‚  â€¢ Filtrage de visibilitÃ© (contextuel)                           â”‚  â”‚ â”‚
â”‚  â”‚  â”‚  â€¢ Pagination                                                     â”‚  â”‚ â”‚
â”‚  â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚ â”‚
â”‚  â”‚                               â”‚                                         â”‚ â”‚
â”‚  â”‚  DiscoverPermissionsForCapability    DiscoverCapabilitiesForPermission â”‚ â”‚
â”‚  â”‚         â”‚                                     â”‚                         â”‚ â”‚
â”‚  â”‚         â–¼                                     â–¼                         â”‚ â”‚
â”‚  â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚ â”‚
â”‚  â”‚  â”‚                   RÃ‰SOLUTION DES ASSOCIATIONS                     â”‚  â”‚ â”‚
â”‚  â”‚  â”‚  â€¢ RÃ©solution directe                                             â”‚  â”‚ â”‚
â”‚  â”‚  â”‚  â€¢ RÃ©solution des implications                                    â”‚  â”‚ â”‚
â”‚  â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚ â”‚
â”‚  â”‚                               â”‚                                         â”‚ â”‚
â”‚  â”‚  ComputeCapabilityContext    â”‚                                         â”‚ â”‚
â”‚  â”‚         â”‚                    â”‚                                         â”‚ â”‚
â”‚  â”‚         â–¼                    â–¼                                         â”‚ â”‚
â”‚  â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚ â”‚
â”‚  â”‚  â”‚                    CALCUL DU CONTEXTE                             â”‚  â”‚ â”‚
â”‚  â”‚  â”‚  â€¢ Intersection rÃ´les-permissions                                 â”‚  â”‚ â”‚
â”‚  â”‚  â”‚  â€¢ Application des contraintes de sÃ©curitÃ©                        â”‚  â”‚ â”‚
â”‚  â”‚  â”‚  â€¢ GÃ©nÃ©ration des restrictions                                    â”‚  â”‚ â”‚
â”‚  â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚ â”‚
â”‚  â”‚                                                                          â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚                                      â”‚                                       â”‚
â”‚                                      â–¼                                       â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚                           SOURCES DE DONNÃ‰ES                           â”‚ â”‚
â”‚  â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤ â”‚
â”‚  â”‚                                                                          â”‚ â”‚
â”‚  â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”           â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”            â”‚ â”‚
â”‚  â”‚   â”‚  CAPABILITY REGISTRY â”‚           â”‚ PERMISSION REGISTRY â”‚            â”‚ â”‚
â”‚  â”‚   â”‚                       â”‚           â”‚                     â”‚            â”‚ â”‚
â”‚  â”‚   â”‚  â€¢ CapacitÃ©s         â”‚â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â–ºâ”‚  â€¢ Permissions      â”‚            â”‚ â”‚
â”‚  â”‚   â”‚  â€¢ Index par module  â”‚           â”‚  â€¢ Associations     â”‚            â”‚ â”‚
â”‚  â”‚   â”‚  â€¢ Index par catÃ©gorieâ”‚          â”‚  â€¢ HiÃ©rarchies      â”‚            â”‚ â”‚
â”‚  â”‚   â”‚  â€¢ Relations         â”‚           â”‚                     â”‚            â”‚ â”‚
â”‚  â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜           â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜            â”‚ â”‚
â”‚  â”‚                                                                          â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚                                                                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 9.2. Flux de DÃ©couverte Typique

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                        FLUX DE DÃ‰COUVERTE TYPIQUE                            â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜

   DEMANDEUR                   DISCOVERY API                   REGISTRES
       â”‚                            â”‚                              â”‚
       â”‚  DiscoverByModule(         â”‚                              â”‚
       â”‚    module_id: "spm.cms",   â”‚                              â”‚
       â”‚    filter: {status: Active}â”‚                              â”‚
       â”‚  )                         â”‚                              â”‚
       â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–ºâ”‚                              â”‚
       â”‚                            â”‚                              â”‚
       â”‚                            â”‚  1. Valider la requÃªte       â”‚
       â”‚                            â”‚                              â”‚
       â”‚                            â”‚  2. VÃ©rifier visibilitÃ©      â”‚
       â”‚                            â”‚     demandeur                â”‚
       â”‚                            â”‚                              â”‚
       â”‚                            â”‚  3. Interroger registre      â”‚
       â”‚                            â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–ºâ”‚
       â”‚                            â”‚                              â”‚
       â”‚                            â”‚  4. Appliquer filtres        â”‚
       â”‚                            â”‚â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
       â”‚                            â”‚     [CapacitÃ©s brutes]       â”‚
       â”‚                            â”‚                              â”‚
       â”‚                            â”‚  5. Filtrer par visibilitÃ©   â”‚
       â”‚                            â”‚                              â”‚
       â”‚                            â”‚  6. Enrichir rÃ©sumÃ©s         â”‚
       â”‚                            â”‚                              â”‚
       â”‚                            â”‚  7. Paginer rÃ©sultats        â”‚
       â”‚                            â”‚                              â”‚
       â”‚  DiscoveryResponse {       â”‚                              â”‚
       â”‚    success: true,          â”‚                              â”‚
       â”‚    data: {                 â”‚                              â”‚
       â”‚      capabilities: [...],  â”‚                              â”‚
       â”‚      total_count: 8        â”‚                              â”‚
       â”‚    }                       â”‚                              â”‚
       â”‚  }                         â”‚                              â”‚
       â”‚â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤                              â”‚
       â”‚                            â”‚                              â”‚
       â–¼                            â–¼                              â–¼
```

### 9.3. Calcul du Contexte de CapacitÃ©

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                   CALCUL DU CONTEXTE DE CAPACITÃ‰                             â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜

                    ContextSpecification
                           â”‚
                           â”‚
    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
    â”‚                      â”‚                      â”‚
    â–¼                      â–¼                      â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”          â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”          â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚Requesterâ”‚          â”‚  Roles  â”‚          â”‚   Target    â”‚
â”‚ Identityâ”‚          â”‚  List   â”‚          â”‚Specificationâ”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜          â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜          â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
    â”‚                      â”‚                      â”‚
    â”‚                      â”‚                      â”‚
    â–¼                      â–¼                      â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                        MOTEUR DE CALCUL DE CONTEXTE                          â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                              â”‚
â”‚   1. RÃ©cupÃ©rer toutes les capacitÃ©s du target (si spÃ©cifiÃ©)                 â”‚
â”‚                            â”‚                                                 â”‚
â”‚                            â–¼                                                 â”‚
â”‚   2. RÃ©cupÃ©rer les permissions des rÃ´les                                    â”‚
â”‚                            â”‚                                                 â”‚
â”‚                            â–¼                                                 â”‚
â”‚   3. RÃ©soudre les capacitÃ©s couvertes par ces permissions                   â”‚
â”‚                            â”‚                                                 â”‚
â”‚                            â–¼                                                 â”‚
â”‚   4. Intersection avec capacitÃ©s du target                                  â”‚
â”‚                            â”‚                                                 â”‚
â”‚                            â–¼                                                 â”‚
â”‚   5. Appliquer contraintes de sÃ©curitÃ©                                      â”‚
â”‚       â€¢ Exclure capacitÃ©s au-dessus du niveau autorisÃ©                      â”‚
â”‚       â€¢ Marquer les restrictions                                            â”‚
â”‚                            â”‚                                                 â”‚
â”‚                            â–¼                                                 â”‚
â”‚   6. GÃ©nÃ©rer le CapabilityContext                                           â”‚
â”‚                                                                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                            â”‚
                            â–¼
              â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
              â”‚    CapabilityContext    â”‚
              â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
              â”‚ accessible_capabilities â”‚
              â”‚ accessible_permissions  â”‚
              â”‚ restrictions            â”‚
              â”‚ computed_at             â”‚
              â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 10. Invariants Non NÃ©gociables

### INV-DISC-1 : Lecture Seule

> **La dÃ©couverte ne modifie jamais les registres.**

**Implications :**
- Aucune opÃ©ration de dÃ©couverte ne crÃ©e, modifie ou supprime de donnÃ©es
- Les registres sont inchangÃ©s aprÃ¨s une dÃ©couverte
- La dÃ©couverte est idempotente (mÃªmes paramÃ¨tres = mÃªmes rÃ©sultats)

### INV-DISC-2 : Non-DÃ©cision

> **La dÃ©couverte informe mais ne dÃ©cide jamais.**

**Implications :**
- Aucune rÃ©ponse de dÃ©couverte ne contient "autorisÃ©" ou "refusÃ©"
- Le contexte calculÃ© est une projection, pas une autorisation
- La dÃ©cision appartient exclusivement Ã  StrongFather

### INV-DISC-3 : ExhaustivitÃ© selon VisibilitÃ©

> **La dÃ©couverte retourne tous les Ã©lÃ©ments visibles selon le contexte demandeur.**

**Implications :**
- Aucune capacitÃ© ou permission visible n'est omise
- Les filtres rÃ©duisent mais ne cachent pas arbitrairement
- La visibilitÃ© est dÃ©terminÃ©e par des rÃ¨gles explicites

### INV-DISC-4 : CohÃ©rence Temporelle

> **Les rÃ©sultats de dÃ©couverte sont cohÃ©rents Ã  l'instant de la requÃªte.**

**Implications :**
- Snapshot cohÃ©rent des registres au moment de la requÃªte
- Pas d'Ã©tat intermÃ©diaire visible
- Les modifications pendant la requÃªte n'affectent pas le rÃ©sultat

### INV-DISC-5 : TraÃ§abilitÃ© des RequÃªtes

> **Toutes les requÃªtes de dÃ©couverte sont journalisÃ©es.**

**Implications :**
- Chaque requÃªte a un identifiant unique (request_id)
- L'identitÃ© du demandeur est enregistrÃ©e
- L'audit des dÃ©couvertes est possible

### INV-DISC-6 : Respect des Contraintes de SÃ©curitÃ©

> **Les contraintes de sÃ©curitÃ© sont toujours appliquÃ©es, sans exception.**

**Implications :**
- Les capacitÃ©s SYSTEM ne sont jamais exposÃ©es aux non-autorisÃ©s
- Le niveau de sÃ©curitÃ© du demandeur est toujours vÃ©rifiÃ©
- Aucun contournement possible par la dÃ©couverte

---

## 11. ConformitÃ© aux Lois d'Autonomie SystÃ¨me

Ce contrat respecte les Lois d'Autonomie SystÃ¨me dÃ©finies dans [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md).

### LOI-1 : Aucune dÃ©pendance externe critique Ã  l'exÃ©cution

**ConformitÃ© :** Conforme

La Discovery API opÃ¨re entiÃ¨rement sur des donnÃ©es locales :

- **Registres locaux** : Les capacitÃ©s et permissions sont stockÃ©es localement
- **Calculs locaux** : Le calcul de contexte n'utilise aucune ressource externe
- **Aucune API externe** : La dÃ©couverte ne dÃ©pend d'aucun service distant

**VÃ©rification LOI-1** : *"La Discovery API fonctionne-t-elle si le rÃ©seau est indisponible ?"* â†’ **Oui.** Toutes les opÃ©rations sont locales.

### LOI-5 : Le coÃ»t doit Ãªtre proportionnel au hardware

**ConformitÃ© :** Conforme

La Discovery API a une empreinte minimale :

- **OpÃ©rations de lecture** : Pas de computation intensive
- **Index existants** : Utilise les index des registres existants
- **Pagination** : RÃ©sultats limitÃ©s pour contrÃ´ler la mÃ©moire
- **Pas de cache permanent** : Les contextes sont calculÃ©s Ã  la demande

**VÃ©rification LOI-5** : *"La Discovery API fonctionne-t-elle sur un Raspberry Pi 4 ?"* â†’ **Oui.** Les opÃ©rations de dÃ©couverte sont des lectures simples avec filtrage.

### SynthÃ¨se de conformitÃ©

| Loi | Statut | Raison |
|-----|--------|--------|
| LOI-1 | âœ… Conforme | OpÃ©rations locales, aucune dÃ©pendance externe |
| LOI-5 | âœ… Conforme | Lectures simples, pagination, pas de cache lourd |

---

## 12. Exemples Complets

### 12.1. Exemple : DÃ©couverte des capacitÃ©s d'un module CMS

```yaml
# RequÃªte
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

# RÃ©ponse
response:
  success: true
  data:
    capabilities:
      - id: "content.create"
        name: "CrÃ©er du contenu"
        description: "CapacitÃ© de crÃ©er un nouveau contenu"
        category: "Data"
        status: "Active"
        source: "spm.cms.content"
        associated_permissions:
          - "content.create.any"
          - "content.create.own"
      - id: "content.read"
        name: "Lire du contenu"
        description: "CapacitÃ© de lire le contenu existant"
        category: "Data"
        status: "Active"
        source: "spm.cms.content"
        associated_permissions:
          - "content.read.any"
          - "content.read.own"
      - id: "content.edit"
        name: "Modifier du contenu"
        description: "CapacitÃ© de modifier un contenu existant"
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

### 12.2. Exemple : Calcul de contexte pour un Ã©diteur de contenu

```yaml
# RequÃªte
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

# RÃ©ponse
response:
  success: true
  data:
    requester:
      type: "User"
      id: "user_editor_001"
    accessible_capabilities:
      - id: "content.create"
        name: "CrÃ©er du contenu"
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
        name: "CrÃ©er son propre contenu"
        level: "STANDARD"
        scope_type: "OWNED"
      - id: "content.edit.own"
        name: "Modifier son propre contenu"
        level: "STANDARD"
        scope_type: "OWNED"
      - id: "content.edit.team"
        name: "Modifier le contenu de l'Ã©quipe"
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

### 12.3. Exemple : DÃ©couverte des permissions pour une capacitÃ©

```yaml
# RequÃªte
request:
  operation: DiscoverPermissionsForCapability
  capability_id: "media.upload"
  include_implied: true

# RÃ©ponse
response:
  success: true
  data:
    capability:
      id: "media.upload"
      name: "TÃ©lÃ©verser un mÃ©dia"
      description: "CapacitÃ© de tÃ©lÃ©verser des fichiers mÃ©dias"
      category: "Media"
      status: "Active"
    direct_permissions:
      - id: "media.upload.any"
        name: "TÃ©lÃ©verser n'importe quel mÃ©dia"
        level: "ELEVATED"
        scope_type: "GLOBAL"
        status: "Active"
      - id: "media.upload.own"
        name: "TÃ©lÃ©verser ses propres mÃ©dias"
        level: "STANDARD"
        scope_type: "OWNED"
        status: "Active"
      - id: "media.upload.team"
        name: "TÃ©lÃ©verser pour l'Ã©quipe"
        level: "STANDARD"
        scope_type: "SCOPED"
        status: "Active"
    implied_permissions:
      - id: "media.manage.all"
        name: "Gestion complÃ¨te des mÃ©dias"
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

La Discovery API de Master Butler est le mÃ©canisme par lequel les composants du systÃ¨me Miyukini peuvent explorer les capacitÃ©s et permissions disponibles. Elle permet une adaptation dynamique des comportements sans jamais participer aux dÃ©cisions d'autorisation.

Cette API incarne le principe fondateur de Master Butler : **exposer ce qui est possible, sans jamais dÃ©cider ce qui est autorisÃ©**.

### Phrase fondatrice

> **La Discovery API permet aux composants de dÃ©couvrir les possibilitÃ©s du systÃ¨me Miyukini de maniÃ¨re exhaustive, filtrÃ©e par contexte, sans jamais constituer une autorisation.**

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il Ã©tablit des rÃ¨gles absolues qui ne peuvent Ãªtre contournÃ©es, nÃ©gociÃ©es, ou modifiÃ©es. Le contrat prime sur toute considÃ©ration pratique.

Toute implÃ©mentation de la Discovery API doit respecter intÃ©gralement ce document. Toute Ã©volution doit prÃ©server les invariants dÃ©finis ici.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** FONDATION â€” Non nÃ©gociable  
**RÃ©fÃ©rence :** Miyukini Core System v2.4

**RÃ©fÃ©rences croisÃ©es :**

- [Master Butler - Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md) : DÃ©finition et responsabilitÃ©s de Master Butler
- [Master Butler - Capability Registry Contract](../registry/Master%20Butler%20-%20Capability%20Registry%20Contract.md) : Registre des capacitÃ©s
- [Master Butler - Permission Registry Contract](../registry/Master%20Butler%20-%20Permission%20Registry%20Contract.md) : Registre des permissions
- [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) : DÃ©finitions canoniques
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) : Lois d'autonomie

---

## 14. Mini log â€” erreurs / warnings / ambiguÃ¯tÃ©s rencontrÃ©es et corrigÃ©es

### AmbiguÃ¯tÃ© A1 : Scope de la dÃ©couverte

**AmbiguÃ¯tÃ© rencontrÃ©e :**
La documentation fondatrice mentionne "API de dÃ©couverte des capacitÃ©s par module" et "API de dÃ©couverte des capacitÃ©s par type d'action" sans dÃ©finir clairement si ce sont des opÃ©rations distinctes ou des filtres d'une mÃªme opÃ©ration.

**DÃ©cision prise :**
OpÃ©rations distinctes dÃ©finies : DiscoverByModule, DiscoverByAction, DiscoverByCategory, avec des signatures et comportements spÃ©cifiques pour chaque mode de dÃ©couverte.

**Justification :**
Des opÃ©rations distinctes permettent une API plus claire et typÃ©e, avec des paramÃ¨tres et validations spÃ©cifiques Ã  chaque mode.

**Correction effectuÃ©e :**
Section 3 "Modes de DÃ©couverte" avec opÃ©rations sÃ©parÃ©es et signatures conceptuelles.

### AmbiguÃ¯tÃ© A2 : VisibilitÃ© contextuelle

**AmbiguÃ¯tÃ© rencontrÃ©e :**
La documentation fondatrice mentionne "filtrage des capacitÃ©s selon le contexte" sans dÃ©finir les niveaux de visibilitÃ© ni les rÃ¨gles de filtrage.

**DÃ©cision prise :**
Quatre niveaux de visibilitÃ© dÃ©finis (Public, Internal, Restricted, Confidential) avec des rÃ¨gles explicites de filtrage basÃ©es sur l'identitÃ© et le niveau de sÃ©curitÃ© du demandeur.

**Justification :**
Des niveaux de visibilitÃ© explicites garantissent une sÃ©curitÃ© cohÃ©rente et prÃ©visible.

**Correction effectuÃ©e :**
Section 7 "RÃ¨gles de VisibilitÃ©" avec niveaux et rÃ¨gles de filtrage.

### AmbiguÃ¯tÃ© A3 : Contexte de capacitÃ©

**AmbiguÃ¯tÃ© rencontrÃ©e :**
Le "contexte de capacitÃ©" est mentionnÃ© dans la documentation fondatrice avec une liste d'Ã©lÃ©ments inclus, mais sans dÃ©finition formelle de la structure ni du mÃ©canisme de calcul.

**DÃ©cision prise :**
Structure formelle dÃ©finie (ContextSpecification, CapabilityContext) avec un algorithme de calcul en 6 Ã©tapes clairement documentÃ©.

**Justification :**
Une structure et un algorithme formels garantissent un comportement prÃ©visible et vÃ©rifiable.

**Correction effectuÃ©e :**
Section 4 "Contexte de CapacitÃ©" avec structures de donnÃ©es et rÃ¨gles de calcul.

### AmbiguÃ¯tÃ© A4 : RÃ©ponses d'erreur

**AmbiguÃ¯tÃ© rencontrÃ©e :**
Aucune documentation existante ne dÃ©finit les codes d'erreur spÃ©cifiques Ã  la Discovery API.

**DÃ©cision prise :**
Catalogue de 9 codes d'erreur dÃ©finis (DISC-001 Ã  DISC-009) couvrant les cas d'erreur typiques.

**Justification :**
Des codes d'erreur standardisÃ©s facilitent le diagnostic et le traitement des erreurs.

**Correction effectuÃ©e :**
Section 6.2 "Codes d'Erreur" avec codes et descriptions.

---

*Aucune autre erreur, warning, ou ambiguÃ¯tÃ© rencontrÃ©e lors de la rÃ©daction de ce document.*

