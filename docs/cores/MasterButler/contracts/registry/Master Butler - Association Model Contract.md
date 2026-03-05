# Master Butler â€” Association Model Contract

## 1. Introduction

### Objet du contrat

Ce document dÃ©finit le **Master Butler Association Model Contract** : un contrat normatif, non nÃ©gociable, et de statut FONDATION qui Ã©tablit le modÃ¨le complet des associations entre permissions, capacitÃ©s, et rÃ´les dans le systÃ¨me Miyukini Core System v2.4.

Ce contrat dÃ©finit :
- Le modÃ¨le formel d'association permission-capacitÃ©
- Le modÃ¨le formel d'association rÃ´le-permission
- Les rÃ¨gles de crÃ©ation, modification et suppression d'associations
- Les invariants garantissant l'intÃ©gritÃ© rÃ©fÃ©rentielle
- Les opÃ©rations autorisÃ©es sur les associations
- Les mÃ©canismes de rÃ©solution des droits effectifs

### PortÃ©e

Ce contrat s'applique Ã  **toutes les associations** du systÃ¨me et dÃ©finit de maniÃ¨re absolue :

- La dÃ©finition formelle d'une Association
- Les types d'associations (Permission-CapacitÃ©, RÃ´le-Permission)
- Les rÃ¨gles de crÃ©ation et validation des associations
- Les mÃ©canismes de rÃ©solution des capacitÃ©s et permissions effectives
- Les invariants non nÃ©gociables du modÃ¨le d'association
- Les interactions avec les registres de capacitÃ©s et permissions

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il Ã©tablit des rÃ¨gles absolues que Master Butler applique sans exception. Ces rÃ¨gles ne peuvent Ãªtre contournÃ©es, nÃ©gociÃ©es, ou modifiÃ©es. Le contrat prime sur toute considÃ©ration pratique.

### Relation avec les autres contrats

Ce contrat complÃ¨te et s'articule avec les documents contractuels existants :

- **[Master Butler - Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md)** : DÃ©finit la nature, le rÃ´le, et les responsabilitÃ©s de Master Butler
- **[Master Butler - Capability Registry Contract](./Master%20Butler%20-%20Capability%20Registry%20Contract.md)** : DÃ©finit le registre des capacitÃ©s
- **[Master Butler - Permission Registry Contract](./Master%20Butler%20-%20Permission%20Registry%20Contract.md)** : DÃ©finit le registre des permissions
- **[Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)** : DÃ©finitions canoniques des termes
- **[Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)** : Ce contrat respecte **LOI-1** (aucune dÃ©pendance externe critique) et **LOI-5** (coÃ»t proportionnel au hardware)

**ComplÃ©mentaritÃ© :**

- Capability Registry Contract = dÃ©finition des capacitÃ©s (ce qui est techniquement possible)
- Permission Registry Contract = dÃ©finition des permissions (les droits qui existent)
- Association Model Contract = liens entre permissions, capacitÃ©s et rÃ´les (comment ils sont connectÃ©s)

Ces contrats forment ensemble le systÃ¨me complet de gestion des capacitÃ©s, permissions, et associations du systÃ¨me Miyukini Core System v2.4.

---

## 2. DÃ©finitions formelles

### 2.1. Association (dÃ©finition gÃ©nÃ©rale)

**DÃ©finition formelle :**

Une **Association** est un lien formel, typÃ©, et traÃ§able entre deux entitÃ©s du systÃ¨me de capacitÃ©s et permissions. Une association dÃ©finit une relation sÃ©mantique entre ces entitÃ©s.

**CaractÃ©ristiques fondamentales :**

| CaractÃ©ristique | Description | Obligatoire |
|-----------------|-------------|-------------|
| **IdentifiÃ©e** | PossÃ¨de un identifiant unique | âœ… Oui |
| **TypÃ©e** | PossÃ¨de un type dÃ©fini (Permission-CapacitÃ©, RÃ´le-Permission) | âœ… Oui |
| **Directionnelle** | PossÃ¨de une source et une cible | âœ… Oui |
| **ValidÃ©e** | RÃ©fÃ©rence des entitÃ©s existantes | âœ… Oui |
| **TraÃ§able** | Son historique est enregistrÃ© | âœ… Oui |
| **MÃ©tadonnÃ©e** | Peut porter des informations additionnelles | Optionnel |

**Structure formelle d'une Association :**

```
Association {
  id: AssociationId,           // Identifiant unique
  type: AssociationType,       // Type de l'association
  source: EntityReference,     // EntitÃ© source
  target: EntityReference,     // EntitÃ© cible
  metadata: AssociationMetadata, // MÃ©tadonnÃ©es additionnelles
  created_at: Timestamp,       // Date de crÃ©ation
  created_by: Identity,        // CrÃ©ateur
  status: AssociationStatus    // Statut (Active, Suspended, Revoked)
}
```

**Invariants :**

- INV-ASSOC-1 : Toute association possÃ¨de un identifiant unique
- INV-ASSOC-2 : Toute association rÃ©fÃ©rence des entitÃ©s existantes et valides
- INV-ASSOC-3 : Toute association possÃ¨de un type dÃ©fini
- INV-ASSOC-4 : L'identifiant d'une association ne peut jamais Ãªtre modifiÃ©

### 2.2. Types d'Associations

**Ã‰noncÃ© :**

Le systÃ¨me reconnaÃ®t deux types d'associations primaires, chacun avec sa sÃ©mantique propre.

**Types dÃ©finis :**

| Type | Source | Cible | SÃ©mantique |
|------|--------|-------|------------|
| **PermissionCapability** | Permission | CapacitÃ© | "Cette permission couvre cette capacitÃ©" |
| **RolePermission** | RÃ´le | Permission | "Ce rÃ´le possÃ¨de cette permission" |

**Types secondaires (dÃ©rivÃ©s) :**

| Type | Source | Cible | SÃ©mantique |
|------|--------|-------|------------|
| **PermissionImplication** | Permission | Permission | "Cette permission implique cette autre permission" |
| **PermissionConflict** | Permission | Permission | "Ces permissions sont mutuellement exclusives" |
| **PermissionRequirement** | Permission | Permission | "Cette permission requiert cette autre permission" |

**Invariants :**

- INV-TYPE-1 : Toute association possÃ¨de exactement un type
- INV-TYPE-2 : Le type dÃ©termine les contraintes de validation applicables
- INV-TYPE-3 : Les associations de type diffÃ©rent sont indÃ©pendantes

### 2.3. RÃ©fÃ©rence d'EntitÃ© (EntityReference)

**DÃ©finition formelle :**

Une **EntityReference** identifie de maniÃ¨re unique une entitÃ© dans le systÃ¨me de capacitÃ©s et permissions.

**Types de rÃ©fÃ©rences :**

| Type | Format | Exemple |
|------|--------|---------|
| **CapabilityRef** | `cap:<CapabilityId>` | `cap:content.create` |
| **PermissionRef** | `perm:<PermissionId>` | `perm:content.article.create.any` |
| **RoleRef** | `role:<RoleId>` | `role:content.editor` |

**Structure formelle :**

```
EntityReference {
  type: EntityType,            // Capability, Permission, Role
  id: String,                  // Identifiant de l'entitÃ©
  version: Version?            // Version optionnelle (pour traÃ§abilitÃ©)
}
```

**Invariants :**

- INV-REF-1 : Toute rÃ©fÃ©rence pointe vers une entitÃ© existante
- INV-REF-2 : Le type de rÃ©fÃ©rence est cohÃ©rent avec l'entitÃ© rÃ©fÃ©rencÃ©e
- INV-REF-3 : Une rÃ©fÃ©rence invalide (entitÃ© supprimÃ©e) rend l'association inactive

---

## 3. Association Permission-CapacitÃ©

### 3.1. DÃ©finition

**DÃ©finition formelle :**

Une **Association Permission-CapacitÃ©** lie une permission Ã  une capacitÃ©, indiquant que cette permission "couvre" ou "donne accÃ¨s" Ã  cette capacitÃ©.

**Phrase fondatrice :**

> **Une permission couvre des capacitÃ©s. Sans capacitÃ© associÃ©e, une permission est vide de sens.**

### 3.2. ModÃ¨le de donnÃ©es

**Structure d'une association Permission-CapacitÃ© :**

```
PermissionCapabilityAssociation {
  id: AssociationId,
  type: "PermissionCapability",
  source: PermissionRef,
  target: CapabilityRef,
  coverage_type: CoverageType,
  conditions: AssociationConditions?,
  metadata: AssociationMetadata,
  status: AssociationStatus
}
```

**Types de couverture (CoverageType) :**

| Type | Description | Exemple |
|------|-------------|---------|
| **FULL** | AccÃ¨s complet Ã  la capacitÃ© | `content.manage.all` â†’ `content.create` (FULL) |
| **PARTIAL** | AccÃ¨s limitÃ© par conditions | `content.edit.own` â†’ `content.edit` (PARTIAL, own_only) |
| **CONDITIONAL** | AccÃ¨s soumis Ã  conditions dynamiques | `workflow.approve.assigned` â†’ `workflow.approve` (CONDITIONAL) |

### 3.3. Conditions d'association

**DÃ©finition :**

Les **conditions d'association** dÃ©finissent les restrictions applicables Ã  la couverture d'une capacitÃ© par une permission.

**Types de conditions :**

| Condition | Description | Exemple |
|-----------|-------------|---------|
| **OWNER_ONLY** | LimitÃ© aux entitÃ©s possÃ©dÃ©es | `content.edit.own` : Ã©dition de ses propres contenus |
| **SCOPE_LIMITED** | LimitÃ© Ã  un pÃ©rimÃ¨tre | `content.edit.team` : Ã©dition dans l'Ã©quipe |
| **TIME_BOUND** | LimitÃ© dans le temps | Permission temporaire avec date d'expiration |
| **COUNT_LIMITED** | LimitÃ© en nombre | Maximum N actions par pÃ©riode |
| **CONTEXT_DEPENDENT** | DÃ©pend du contexte runtime | Ã‰valuÃ© par StrongFather |

**Structure des conditions :**

```
AssociationConditions {
  type: ConditionType,
  parameters: Map<String, Any>,
  evaluation_point: "ASSOCIATION" | "RUNTIME"
}
```

**RÃ¨gles :**

- R-COND-1 : Les conditions ASSOCIATION sont Ã©valuÃ©es Ã  la crÃ©ation
- R-COND-2 : Les conditions RUNTIME sont Ã©valuÃ©es par StrongFather
- R-COND-3 : Les conditions sont cumulatives (AND logique)

### 3.4. RÃ¨gles de crÃ©ation

**PrÃ©conditions :**

| PrÃ©condition | Description | Erreur si Ã©choue |
|--------------|-------------|------------------|
| PRE-PC-1 | La permission existe et est ACTIVE ou DRAFT | `INVALID_PERMISSION_STATE` |
| PRE-PC-2 | La capacitÃ© existe et est ACTIVE | `INVALID_CAPABILITY_STATE` |
| PRE-PC-3 | L'association n'existe pas dÃ©jÃ  | `DUPLICATE_ASSOCIATION` |
| PRE-PC-4 | Les conditions sont valides | `INVALID_CONDITIONS` |

**Postconditions :**

| Postcondition | Description |
|---------------|-------------|
| POST-PC-1 | L'association est crÃ©Ã©e avec statut ACTIVE |
| POST-PC-2 | La permission rÃ©fÃ©rence la capacitÃ© |
| POST-PC-3 | L'historique est mis Ã  jour |
| POST-PC-4 | Les index sont mis Ã  jour |

### 3.5. CardinalitÃ©

**RÃ¨gles de cardinalitÃ© :**

| Relation | CardinalitÃ© | Description |
|----------|-------------|-------------|
| Permission â†’ CapacitÃ© | 1..* | Une permission couvre au moins une capacitÃ© |
| CapacitÃ© â†’ Permission | 0..* | Une capacitÃ© peut Ãªtre couverte par plusieurs permissions |

**Invariants de cardinalitÃ© :**

- INV-CARD-PC-1 : Une permission ACTIVE possÃ¨de au moins une association Permission-CapacitÃ© active
- INV-CARD-PC-2 : Une capacitÃ© peut exister sans permission associÃ©e (capacitÃ© technique non exposÃ©e)

---

## 4. Association RÃ´le-Permission

### 4.1. DÃ©finition

**DÃ©finition formelle :**

Une **Association RÃ´le-Permission** lie un rÃ´le Ã  une permission, indiquant que ce rÃ´le "possÃ¨de" ou "dÃ©tient" cette permission.

**Phrase fondatrice :**

> **Un rÃ´le est un ensemble nommÃ© de permissions. L'association RÃ´le-Permission dÃ©finit cet ensemble.**

**Note importante :**

Master Butler dÃ©finit les associations RÃ´le-Permission, mais **ne gÃ¨re pas l'attribution des rÃ´les aux utilisateurs**. L'attribution des rÃ´les appartient au systÃ¨me d'identitÃ© (hors-scope de Master Butler).

### 4.2. ModÃ¨le de donnÃ©es

**Structure d'une association RÃ´le-Permission :**

```
RolePermissionAssociation {
  id: AssociationId,
  type: "RolePermission",
  source: RoleRef,
  target: PermissionRef,
  grant_type: GrantType,
  scope_restriction: ScopeRestriction?,
  metadata: AssociationMetadata,
  status: AssociationStatus
}
```

**Types d'attribution (GrantType) :**

| Type | Description | HÃ©ritage |
|------|-------------|----------|
| **DIRECT** | Attribution directe | Non hÃ©ritÃ©e |
| **INHERITED** | Attribution via hiÃ©rarchie de rÃ´les | HÃ©ritÃ©e du rÃ´le parent |
| **DELEGATED** | Attribution par dÃ©lÃ©gation | LimitÃ©e dans le temps |

### 4.3. Restriction de portÃ©e

**DÃ©finition :**

Une **restriction de portÃ©e** limite le pÃ©rimÃ¨tre dans lequel la permission est accordÃ©e au rÃ´le.

**Types de restrictions :**

| Type | Description | Exemple |
|------|-------------|---------|
| **NONE** | Aucune restriction | Permission globale |
| **ORGANIZATION** | LimitÃ© Ã  l'organisation | Ã‰dition dans l'organisation |
| **TEAM** | LimitÃ© Ã  l'Ã©quipe | Ã‰dition dans l'Ã©quipe |
| **PROJECT** | LimitÃ© au projet | Ã‰dition dans le projet |
| **CUSTOM** | Restriction personnalisÃ©e | PÃ©rimÃ¨tre dÃ©fini par configuration |

**Structure de restriction :**

```
ScopeRestriction {
  type: ScopeType,
  scope_id: String?,          // Identifiant du pÃ©rimÃ¨tre (si applicable)
  parameters: Map<String, Any>? // ParamÃ¨tres additionnels
}
```

### 4.4. RÃ¨gles de crÃ©ation

**PrÃ©conditions :**

| PrÃ©condition | Description | Erreur si Ã©choue |
|--------------|-------------|------------------|
| PRE-RP-1 | Le rÃ´le existe et est ACTIVE | `INVALID_ROLE_STATE` |
| PRE-RP-2 | La permission existe et est ACTIVE | `INVALID_PERMISSION_STATE` |
| PRE-RP-3 | L'association n'existe pas dÃ©jÃ  | `DUPLICATE_ASSOCIATION` |
| PRE-RP-4 | Le niveau de permission est compatible avec le rÃ´le | `INCOMPATIBLE_PERMISSION_LEVEL` |
| PRE-RP-5 | Pas de conflit avec les permissions existantes du rÃ´le | `CONFLICTING_PERMISSION` |

**Postconditions :**

| Postcondition | Description |
|---------------|-------------|
| POST-RP-1 | L'association est crÃ©Ã©e avec statut ACTIVE |
| POST-RP-2 | Le rÃ´le possÃ¨de la permission |
| POST-RP-3 | L'historique est mis Ã  jour |
| POST-RP-4 | Les capacitÃ©s effectives du rÃ´le sont recalculÃ©es |

### 4.5. CardinalitÃ©

**RÃ¨gles de cardinalitÃ© :**

| Relation | CardinalitÃ© | Description |
|----------|-------------|-------------|
| RÃ´le â†’ Permission | 0..* | Un rÃ´le peut possÃ©der plusieurs permissions |
| Permission â†’ RÃ´le | 0..* | Une permission peut Ãªtre attribuÃ©e Ã  plusieurs rÃ´les |

**Invariants de cardinalitÃ© :**

- INV-CARD-RP-1 : Un rÃ´le peut exister sans permission (rÃ´le vide, en cours de dÃ©finition)
- INV-CARD-RP-2 : Une permission peut exister sans Ãªtre attribuÃ©e Ã  un rÃ´le

---

## 5. ModÃ¨le de RÃ´le

### 5.1. DÃ©finition

**DÃ©finition formelle :**

Un **RÃ´le** est un ensemble nommÃ© de permissions, identifiable et attribuable. Master Butler connaÃ®t les rÃ´les et leurs permissions associÃ©es, mais ne gÃ¨re pas l'attribution des rÃ´les aux utilisateurs.

**CaractÃ©ristiques d'un rÃ´le :**

| CaractÃ©ristique | Description | Obligatoire |
|-----------------|-------------|-------------|
| **IdentifiÃ©** | PossÃ¨de un identifiant unique | âœ… Oui |
| **NommÃ©** | PossÃ¨de un nom lisible | âœ… Oui |
| **DocumentÃ©** | PossÃ¨de une description | âœ… Oui |
| **HiÃ©rarchique** | Peut hÃ©riter d'autres rÃ´les | Optionnel |
| **LimitÃ©** | Peut avoir un niveau maximum de permission | Optionnel |

### 5.2. Structure d'un RÃ´le

```
Role {
  id: RoleId,                    // Identifiant unique
  name: String,                  // Nom lisible
  description: String,           // Description du rÃ´le
  domain: String,                // Domaine fonctionnel
  parent_roles: Set<RoleId>?,    // RÃ´les parents (hÃ©ritage)
  max_permission_level: PermissionLevel?, // Niveau maximum autorisÃ©
  metadata: RoleMetadata,
  status: RoleStatus             // DRAFT, ACTIVE, DEPRECATED, RETIRED
}
```

### 5.3. HiÃ©rarchie de RÃ´les

**DÃ©finition :**

Les rÃ´les peuvent former une **hiÃ©rarchie** oÃ¹ un rÃ´le enfant hÃ©rite des permissions de ses rÃ´les parents.

**RÃ¨gles d'hÃ©ritage :**

| RÃ¨gle | Description | Statut |
|-------|-------------|--------|
| R-HIER-1 | Un rÃ´le enfant hÃ©rite toutes les permissions de ses parents | NON NÃ‰GOCIABLE |
| R-HIER-2 | L'hÃ©ritage est transitif (grand-parent â†’ parent â†’ enfant) | NON NÃ‰GOCIABLE |
| R-HIER-3 | Aucun cycle n'est autorisÃ© dans la hiÃ©rarchie | NON NÃ‰GOCIABLE |
| R-HIER-4 | Un rÃ´le ne peut pas avoir un niveau supÃ©rieur Ã  ses parents | NON NÃ‰GOCIABLE |

**Exemple de hiÃ©rarchie :**

```
role:admin
â”œâ”€â”€ hÃ©rite de: role:manager
â”‚   â”œâ”€â”€ hÃ©rite de: role:editor
â”‚   â”‚   â””â”€â”€ permissions: [content.edit.*, content.create.*]
â”‚   â””â”€â”€ permissions: [content.delete.*, content.publish.*]
â””â”€â”€ permissions: [admin.*, system.config.*]

Permissions effectives de admin:
- admin.*
- system.config.*
- content.delete.*
- content.publish.*
- content.edit.*
- content.create.*
```

### 5.4. Registre des RÃ´les

**DÃ©finition :**

Le **Registre des RÃ´les** est la structure de Master Butler qui contient l'inventaire de tous les rÃ´les dÃ©finis dans le systÃ¨me.

**Structure :**

```
RoleRegistry {
  roles: Map<RoleId, Role>,                    // Index principal
  by_domain: Map<String, Set<RoleId>>,         // Index par domaine
  hierarchy: RoleHierarchyGraph,               // Graphe de hiÃ©rarchie
  history: RoleHistory                         // Historique des modifications
}
```

---

## 6. RÃ©solution des Droits Effectifs

### 6.1. CapacitÃ©s effectives d'une Permission

**DÃ©finition :**

Les **capacitÃ©s effectives** d'une permission sont l'ensemble des capacitÃ©s couvertes par cette permission, incluant les capacitÃ©s des permissions impliquÃ©es.

**Algorithme de rÃ©solution :**

```
ResolvEffectiveCapabilities(permission_id) {
  result = Set()
  visited = Set()
  
  function resolve(perm_id) {
    if perm_id in visited: return
    visited.add(perm_id)
    
    permission = GetPermission(perm_id)
    
    // Ajouter les capacitÃ©s directes
    for assoc in GetPermissionCapabilityAssociations(perm_id):
      if assoc.status == ACTIVE:
        result.add(assoc.target)
    
    // RÃ©soudre les permissions impliquÃ©es
    for implied_perm_id in permission.implied_permissions:
      resolve(implied_perm_id)
  }
  
  resolve(permission_id)
  return result
}
```

**Invariants :**

- INV-RES-1 : La rÃ©solution termine toujours (pas de cycle)
- INV-RES-2 : Le rÃ©sultat est stable pour un Ã©tat donnÃ© du registre
- INV-RES-3 : Les capacitÃ©s retournÃ©es sont toutes ACTIVE

### 6.2. Permissions effectives d'un RÃ´le

**DÃ©finition :**

Les **permissions effectives** d'un rÃ´le sont l'ensemble des permissions possÃ©dÃ©es par ce rÃ´le, incluant les permissions hÃ©ritÃ©es des rÃ´les parents.

**Algorithme de rÃ©solution :**

```
ResolveEffectivePermissions(role_id) {
  result = Set()
  visited = Set()
  
  function resolve(r_id) {
    if r_id in visited: return
    visited.add(r_id)
    
    role = GetRole(r_id)
    
    // Ajouter les permissions directes
    for assoc in GetRolePermissionAssociations(r_id):
      if assoc.status == ACTIVE:
        result.add(assoc.target)
    
    // RÃ©soudre les rÃ´les parents
    for parent_id in role.parent_roles:
      resolve(parent_id)
  }
  
  resolve(role_id)
  return result
}
```

### 6.3. CapacitÃ©s effectives d'un RÃ´le

**DÃ©finition :**

Les **capacitÃ©s effectives** d'un rÃ´le sont l'union des capacitÃ©s effectives de toutes ses permissions effectives.

**Algorithme de rÃ©solution :**

```
ResolveRoleEffectiveCapabilities(role_id) {
  permissions = ResolveEffectivePermissions(role_id)
  capabilities = Set()
  
  for perm_ref in permissions:
    perm_caps = ResolveEffectiveCapabilities(perm_ref.id)
    capabilities = capabilities.union(perm_caps)
  
  return capabilities
}
```

### 6.4. Contexte de CapacitÃ©

**DÃ©finition :**

Le **Contexte de CapacitÃ©** est la structure qui agrÃ¨ge toutes les informations de droits pour un contexte donnÃ© (utilisateur, rÃ´les, pÃ©rimÃ¨tre).

**Structure :**

```
CapabilityContext {
  identity: IdentityRef,                  // IdentitÃ© du demandeur
  roles: Set<RoleId>,                     // RÃ´les du demandeur
  effective_permissions: Set<PermissionRef>, // Permissions effectives
  effective_capabilities: Set<CapabilityRef>, // CapacitÃ©s effectives
  scope_restrictions: Map<PermissionRef, ScopeRestriction>, // Restrictions par permission
  computed_at: Timestamp                  // Date de calcul
}
```

**RÃ¨gles de calcul :**

| RÃ¨gle | Description | Statut |
|-------|-------------|--------|
| R-CTX-1 | Le contexte est calculÃ© Ã  la demande | NON NÃ‰GOCIABLE |
| R-CTX-2 | Le calcul n'est pas cachÃ© entre requÃªtes | NON NÃ‰GOCIABLE |
| R-CTX-3 | Le contexte est une projection, pas une dÃ©cision | NON NÃ‰GOCIABLE |
| R-CTX-4 | StrongFather utilise le contexte pour dÃ©cider | NON NÃ‰GOCIABLE |

---

## 7. OpÃ©rations sur les Associations

### 7.1. CrÃ©ation d'Association Permission-CapacitÃ©

**Signature conceptuelle :**

```
CreatePermissionCapabilityAssociation(
  permission_id: PermissionId,
  capability_id: CapabilityId,
  coverage_type: CoverageType,
  conditions: AssociationConditions?
) â†’ Result<Association, AssociationError>
```

**SÃ©quence :**

1. Valider l'existence et l'Ã©tat de la permission
2. Valider l'existence et l'Ã©tat de la capacitÃ©
3. VÃ©rifier l'absence de duplication
4. Valider les conditions si prÃ©sentes
5. CrÃ©er l'association
6. Mettre Ã  jour les index
7. Historiser l'Ã©vÃ©nement
8. Retourner l'association crÃ©Ã©e

### 7.2. CrÃ©ation d'Association RÃ´le-Permission

**Signature conceptuelle :**

```
CreateRolePermissionAssociation(
  role_id: RoleId,
  permission_id: PermissionId,
  grant_type: GrantType,
  scope_restriction: ScopeRestriction?
) â†’ Result<Association, AssociationError>
```

**SÃ©quence :**

1. Valider l'existence et l'Ã©tat du rÃ´le
2. Valider l'existence et l'Ã©tat de la permission
3. VÃ©rifier la compatibilitÃ© du niveau de permission avec le rÃ´le
4. VÃ©rifier l'absence de conflit avec les permissions existantes
5. VÃ©rifier l'absence de duplication
6. CrÃ©er l'association
7. Mettre Ã  jour les index
8. Recalculer les capacitÃ©s effectives du rÃ´le
9. Historiser l'Ã©vÃ©nement
10. Retourner l'association crÃ©Ã©e

### 7.3. Suspension d'une Association

**Signature conceptuelle :**

```
SuspendAssociation(
  association_id: AssociationId,
  reason: String
) â†’ Result<Association, AssociationError>
```

**RÃ¨gles :**

| RÃ¨gle | Description | Statut |
|-------|-------------|--------|
| R-SUSP-1 | Seules les associations ACTIVE peuvent Ãªtre suspendues | NON NÃ‰GOCIABLE |
| R-SUSP-2 | La suspension est rÃ©versible | NON NÃ‰GOCIABLE |
| R-SUSP-3 | Une association suspendue n'est pas comptÃ©e dans les rÃ©solutions | NON NÃ‰GOCIABLE |
| R-SUSP-4 | La raison de suspension est obligatoire | NON NÃ‰GOCIABLE |

### 7.4. RÃ©vocation d'une Association

**Signature conceptuelle :**

```
RevokeAssociation(
  association_id: AssociationId,
  reason: String
) â†’ Result<(), AssociationError>
```

**RÃ¨gles :**

| RÃ¨gle | Description | Statut |
|-------|-------------|--------|
| R-REV-1 | La rÃ©vocation est irrÃ©versible | NON NÃ‰GOCIABLE |
| R-REV-2 | L'historique conserve la trace complÃ¨te | NON NÃ‰GOCIABLE |
| R-REV-3 | Les capacitÃ©s effectives sont recalculÃ©es | NON NÃ‰GOCIABLE |
| R-REV-4 | La raison de rÃ©vocation est obligatoire | NON NÃ‰GOCIABLE |

### 7.5. Interrogation des Associations

**RequÃªtes disponibles :**

| RequÃªte | Description | ParamÃ¨tres |
|---------|-------------|------------|
| `GetAssociation` | RÃ©cupÃ¨re une association par ID | `association_id` |
| `GetPermissionCapabilities` | CapacitÃ©s d'une permission | `permission_id`, `include_implied` |
| `GetCapabilityPermissions` | Permissions couvrant une capacitÃ© | `capability_id` |
| `GetRolePermissions` | Permissions d'un rÃ´le | `role_id`, `include_inherited` |
| `GetPermissionRoles` | RÃ´les possÃ©dant une permission | `permission_id` |
| `GetRoleCapabilities` | CapacitÃ©s effectives d'un rÃ´le | `role_id` |
| `ListAssociations` | Liste les associations selon filtres | `type`, `source`, `target`, `status` |

---

## 8. Gestion de l'IntÃ©gritÃ© RÃ©fÃ©rentielle

### 8.1. Principes d'intÃ©gritÃ©

**RÃ¨gle fondamentale :**

> **Aucune association ne peut rÃ©fÃ©rencer une entitÃ© inexistante ou invalide.**

**Types d'intÃ©gritÃ© :**

| Type | Description | Application |
|------|-------------|-------------|
| **CrÃ©ation** | Validation Ã  la crÃ©ation | Les rÃ©fÃ©rences doivent exister et Ãªtre valides |
| **Ã‰volution** | Validation lors des modifications | Les modifications ne cassent pas les rÃ©fÃ©rences |
| **Cascade** | Propagation des changements | La suppression d'une entitÃ© impacte ses associations |

### 8.2. RÃ¨gles de cascade

**Suppression d'une CapacitÃ© :**

| ScÃ©nario | Action sur les associations Permission-CapacitÃ© |
|----------|------------------------------------------------|
| CapacitÃ© supprimÃ©e | Association devient INVALID |
| Permission n'a plus de capacitÃ© active | Permission invalidÃ©e (notification) |

**Suppression d'une Permission :**

| ScÃ©nario | Action sur les associations RÃ´le-Permission |
|----------|---------------------------------------------|
| Permission supprimÃ©e | Association rÃ©voquÃ©e automatiquement |
| RÃ´le n'a plus de permission | Aucune action (rÃ´le peut Ãªtre vide) |

**Suppression d'un RÃ´le :**

| ScÃ©nario | Action sur les associations |
|----------|----------------------------|
| RÃ´le supprimÃ© | Associations RÃ´le-Permission rÃ©voquÃ©es |
| RÃ´les enfants | HÃ©ritage cassÃ© (notification) |

### 8.3. VÃ©rification d'intÃ©gritÃ©

**OpÃ©ration de vÃ©rification :**

```
VerifyAssociationIntegrity() â†’ IntegrityReport {
  orphan_associations: List<AssociationId>,  // Associations avec rÃ©fÃ©rences invalides
  broken_implications: List<PermissionId>,   // Permissions avec implications cassÃ©es
  cycle_detected: List<CycleInfo>,           // Cycles dÃ©tectÃ©s
  inconsistencies: List<InconsistencyInfo>   // Autres incohÃ©rences
}
```

**FrÃ©quence de vÃ©rification :**

| Ã‰vÃ©nement | VÃ©rification |
|-----------|--------------|
| CrÃ©ation d'association | VÃ©rification locale |
| Suppression d'entitÃ© | VÃ©rification des associations impactÃ©es |
| Modification d'Ã©tat | VÃ©rification de cohÃ©rence |
| Maintenance planifiÃ©e | VÃ©rification globale |

---

## 9. Invariants Non NÃ©gociables

### INV-ASSOC-MODEL-1 : IntÃ©gritÃ© rÃ©fÃ©rentielle

> **Toute association rÃ©fÃ©rence des entitÃ©s existantes et valides.**

**Implication :** Aucune association orpheline. Aucune rÃ©fÃ©rence vers une entitÃ© supprimÃ©e ou invalide.

### INV-ASSOC-MODEL-2 : UnicitÃ© des associations

> **Une association entre deux entitÃ©s donnÃ©es est unique pour un type donnÃ©.**

**Implication :** Pas de duplication. Une permission ne peut pas Ãªtre associÃ©e deux fois Ã  la mÃªme capacitÃ©.

### INV-ASSOC-MODEL-3 : Absence de cycle

> **Les graphes d'implication (permissions) et de hiÃ©rarchie (rÃ´les) sont acycliques.**

**Implication :** La rÃ©solution des droits effectifs termine toujours. Pas de boucle infinie.

### INV-ASSOC-MODEL-4 : TraÃ§abilitÃ© complÃ¨te

> **Toute modification d'association est tracÃ©e avec contexte complet.**

**Implication :** Audit possible. CrÃ©ation, suspension, rÃ©vocation : tout est enregistrÃ©.

### INV-ASSOC-MODEL-5 : CohÃ©rence des Ã©tats

> **L'Ã©tat d'une association est cohÃ©rent avec l'Ã©tat de ses entitÃ©s.**

**Implication :** Une association ACTIVE ne peut pas rÃ©fÃ©rencer une entitÃ© RETIRED. Les Ã©tats se propagent.

### INV-ASSOC-MODEL-6 : Non-attribution

> **Master Butler dÃ©finit les associations, mais n'attribue jamais de rÃ´le aux utilisateurs.**

**Implication :** L'attribution des rÃ´les appartient au systÃ¨me d'identitÃ©. Master Butler connaÃ®t les rÃ´les et leurs permissions, pas qui les possÃ¨de.

### INV-ASSOC-MODEL-7 : Non-dÃ©cision

> **Master Butler fournit les associations et calcule les droits effectifs, mais ne dÃ©cide jamais si un accÃ¨s est autorisÃ©.**

**Implication :** La dÃ©cision appartient Ã  StrongFather. Master Butler informe, ne dÃ©cide pas.

---

## 10. SchÃ©mas ASCII

### 10.1. ModÃ¨le d'Association Global

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                       ASSOCIATION MODEL                                      â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                              â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                           â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”              â”‚
â”‚  â”‚     ROLE      â”‚                           â”‚  CAPABILITY   â”‚              â”‚
â”‚  â”‚  (ensemble)   â”‚                           â”‚  (pouvoir)    â”‚              â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”˜                           â””â”€â”€â”€â”€â”€â”€â”€â–²â”€â”€â”€â”€â”€â”€â”€â”˜              â”‚
â”‚          â”‚                                           â”‚                       â”‚
â”‚          â”‚ RolePermission                            â”‚ PermissionCapability  â”‚
â”‚          â”‚ Association                               â”‚ Association           â”‚
â”‚          â”‚                                           â”‚                       â”‚
â”‚          â–¼                                           â”‚                       â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                                   â”‚                       â”‚
â”‚  â”‚  PERMISSION   â”‚â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                       â”‚
â”‚  â”‚   (droit)     â”‚                                                           â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                                                           â”‚
â”‚                                                                              â”‚
â”‚  â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•    â”‚
â”‚                                                                              â”‚
â”‚  FLUX DE RÃ‰SOLUTION :                                                        â”‚
â”‚                                                                              â”‚
â”‚  Role â”€â”€â–º Permissions effectives â”€â”€â–º CapacitÃ©s effectives                   â”‚
â”‚                                                                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 10.2. Association Permission-CapacitÃ©

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                   PERMISSION-CAPABILITY ASSOCIATION                          â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                              â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”         â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”          â”‚
â”‚  â”‚  PERMISSION              â”‚         â”‚  CAPABILITY              â”‚          â”‚
â”‚  â”‚  content.article.edit.anyâ”‚         â”‚  content.edit            â”‚          â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜         â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–²â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜          â”‚
â”‚               â”‚                                  â”‚                           â”‚
â”‚               â”‚         â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                           â”‚
â”‚               â”‚         â”‚                                                    â”‚
â”‚               â–¼         â”‚                                                    â”‚
â”‚       â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                                  â”‚
â”‚       â”‚  ASSOCIATION                       â”‚                                  â”‚
â”‚       â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤                                  â”‚
â”‚       â”‚  id: assoc_001                     â”‚                                  â”‚
â”‚       â”‚  type: PermissionCapability        â”‚                                  â”‚
â”‚       â”‚  source: perm:content.article...   â”‚                                  â”‚
â”‚       â”‚  target: cap:content.edit          â”‚                                  â”‚
â”‚       â”‚  coverage_type: FULL               â”‚                                  â”‚
â”‚       â”‚  conditions: null                  â”‚                                  â”‚
â”‚       â”‚  status: ACTIVE                    â”‚                                  â”‚
â”‚       â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                                  â”‚
â”‚                                                                              â”‚
â”‚  â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•    â”‚
â”‚                                                                              â”‚
â”‚  EXEMPLE AVEC CONDITIONS :                                                   â”‚
â”‚                                                                              â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”         â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”          â”‚
â”‚  â”‚  PERMISSION              â”‚         â”‚  CAPABILITY              â”‚          â”‚
â”‚  â”‚  content.draft.edit.own  â”‚         â”‚  content.edit            â”‚          â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜         â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–²â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜          â”‚
â”‚               â”‚                                  â”‚                           â”‚
â”‚               â–¼                                  â”‚                           â”‚
â”‚       â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”                           â”‚
â”‚       â”‚  ASSOCIATION                             â”‚                           â”‚
â”‚       â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤                           â”‚
â”‚       â”‚  coverage_type: PARTIAL                  â”‚                           â”‚
â”‚       â”‚  conditions:                             â”‚                           â”‚
â”‚       â”‚    type: OWNER_ONLY                      â”‚                           â”‚
â”‚       â”‚    evaluation_point: RUNTIME             â”‚                           â”‚
â”‚       â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                           â”‚
â”‚                                                                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 10.3. Association RÃ´le-Permission avec HÃ©ritage

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                    ROLE HIERARCHY AND PERMISSION INHERITANCE                 â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                              â”‚
â”‚                    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                                      â”‚
â”‚                    â”‚  role:admin      â”‚                                      â”‚
â”‚                    â”‚  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ â”‚                                      â”‚
â”‚                    â”‚  Permissions:    â”‚                                      â”‚
â”‚                    â”‚   - admin.*      â”‚                                      â”‚
â”‚                    â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                                      â”‚
â”‚                             â”‚ inherits                                       â”‚
â”‚                             â–¼                                                â”‚
â”‚                    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                                      â”‚
â”‚                    â”‚  role:manager    â”‚                                      â”‚
â”‚                    â”‚  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ â”‚                                      â”‚
â”‚                    â”‚  Permissions:    â”‚                                      â”‚
â”‚                    â”‚   - content.*    â”‚                                      â”‚
â”‚                    â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                                      â”‚
â”‚                             â”‚ inherits                                       â”‚
â”‚                             â–¼                                                â”‚
â”‚                    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                                      â”‚
â”‚                    â”‚  role:editor     â”‚                                      â”‚
â”‚                    â”‚  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ â”‚                                      â”‚
â”‚                    â”‚  Permissions:    â”‚                                      â”‚
â”‚                    â”‚   - content.edit â”‚                                      â”‚
â”‚                    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                                      â”‚
â”‚                                                                              â”‚
â”‚  â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•    â”‚
â”‚                                                                              â”‚
â”‚  PERMISSIONS EFFECTIVES DE role:admin :                                      â”‚
â”‚                                                                              â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”‚
â”‚  â”‚  admin.*        â—„â”€â”€â”€ Direct (DIRECT)                                â”‚    â”‚
â”‚  â”‚  content.*      â—„â”€â”€â”€ HÃ©ritÃ© de manager (INHERITED)                  â”‚    â”‚
â”‚  â”‚  content.edit   â—„â”€â”€â”€ HÃ©ritÃ© de editor via manager (INHERITED)       â”‚    â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â”‚
â”‚                                                                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 10.4. Flux de RÃ©solution des CapacitÃ©s Effectives

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚              EFFECTIVE CAPABILITIES RESOLUTION FLOW                          â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                              â”‚
â”‚  ENTRÃ‰E: role:editor                                                         â”‚
â”‚                                                                              â”‚
â”‚  Ã‰TAPE 1 : RÃ©soudre les permissions effectives                               â”‚
â”‚  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€                              â”‚
â”‚                                                                              â”‚
â”‚  role:editor                                                                 â”‚
â”‚      â”‚                                                                       â”‚
â”‚      â”œâ”€â”€ RolePermission â”€â”€â–º perm:content.article.edit.team                  â”‚
â”‚      â””â”€â”€ RolePermission â”€â”€â–º perm:content.article.create.team                â”‚
â”‚                                                                              â”‚
â”‚  Permissions effectives: {                                                   â”‚
â”‚    perm:content.article.edit.team,                                          â”‚
â”‚    perm:content.article.create.team                                         â”‚
â”‚  }                                                                           â”‚
â”‚                                                                              â”‚
â”‚  Ã‰TAPE 2 : RÃ©soudre les capacitÃ©s de chaque permission                       â”‚
â”‚  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€                       â”‚
â”‚                                                                              â”‚
â”‚  perm:content.article.edit.team                                             â”‚
â”‚      â”‚                                                                       â”‚
â”‚      â””â”€â”€ PermissionCapability â”€â”€â–º cap:content.edit                          â”‚
â”‚                                                                              â”‚
â”‚  perm:content.article.create.team                                           â”‚
â”‚      â”‚                                                                       â”‚
â”‚      â””â”€â”€ PermissionCapability â”€â”€â–º cap:content.create                        â”‚
â”‚                                                                              â”‚
â”‚  Ã‰TAPE 3 : Union des capacitÃ©s                                               â”‚
â”‚  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€                                               â”‚
â”‚                                                                              â”‚
â”‚  CapacitÃ©s effectives de role:editor: {                                     â”‚
â”‚    cap:content.edit,                                                        â”‚
â”‚    cap:content.create                                                       â”‚
â”‚  }                                                                           â”‚
â”‚                                                                              â”‚
â”‚  SORTIE: CapabilityContext                                                   â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”‚
â”‚  â”‚  roles: [role:editor]                                               â”‚    â”‚
â”‚  â”‚  effective_permissions: [perm:content.article.edit.team, ...]       â”‚    â”‚
â”‚  â”‚  effective_capabilities: [cap:content.edit, cap:content.create]     â”‚    â”‚
â”‚  â”‚  scope_restrictions: { perm:...: TEAM }                             â”‚    â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â”‚
â”‚                                                                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 11. ConformitÃ© aux Lois d'Autonomie SystÃ¨me

Ce contrat respecte les Lois d'Autonomie SystÃ¨me dÃ©finies dans [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md).

### LOI-1 : Aucune dÃ©pendance externe critique Ã  l'exÃ©cution

**ConformitÃ© :** Conforme

Le modÃ¨le d'association est entiÃ¨rement local :

- **Stockage local** : Les associations sont stockÃ©es localement
- **RÃ©solution locale** : Tous les calculs de droits effectifs s'exÃ©cutent localement
- **Aucune API externe** : Aucun service distant n'est requis pour gÃ©rer les associations

**VÃ©rification LOI-1** : *"Le modÃ¨le d'association fonctionne-t-il si le rÃ©seau est indisponible ?"* â†’ **Oui.**

### LOI-5 : Le coÃ»t doit Ãªtre proportionnel au hardware

**ConformitÃ© :** Conforme

Le modÃ¨le d'association a une empreinte minimale :

- **DonnÃ©es lÃ©gÃ¨res** : Les associations sont des liens simples entre identifiants
- **Calculs optimisÃ©s** : La rÃ©solution des droits effectifs est O(n) oÃ¹ n est le nombre d'associations
- **Pas de workers** : Aucun processus en arriÃ¨re-plan pour la gestion des associations
- **MÃ©moire prÃ©visible** : Proportionnelle au nombre d'associations

**VÃ©rification LOI-5** : *"Le modÃ¨le fonctionne-t-il sur un Raspberry Pi 4 ?"* â†’ **Oui.** Un systÃ¨me typique avec quelques milliers d'associations reprÃ©sente quelques dizaines de kilo-octets.

### SynthÃ¨se de conformitÃ©

| Loi | Statut | Raison |
|-----|--------|--------|
| LOI-1 | âœ… Conforme | ModÃ¨le local, aucune dÃ©pendance externe |
| LOI-5 | âœ… Conforme | DonnÃ©es lÃ©gÃ¨res, calculs efficaces |

---

## 12. Exemples Concrets

### 12.1. Exemple : CrÃ©ation d'associations pour un module CMS

**Contexte :**
Le module CMS dÃ©finit ses permissions et les associe aux capacitÃ©s.

**DÃ©clarations :**

```yaml
# Association Permission-CapacitÃ© : Ã©dition de contenu
association:
  id: "assoc_cms_edit_001"
  type: "PermissionCapability"
  source: "perm:content.article.edit.team"
  target: "cap:content.edit"
  coverage_type: "PARTIAL"
  conditions:
    type: "SCOPE_LIMITED"
    parameters:
      scope: "team"
    evaluation_point: "RUNTIME"
  status: "ACTIVE"

# Association Permission-CapacitÃ© : crÃ©ation de contenu
association:
  id: "assoc_cms_create_001"
  type: "PermissionCapability"
  source: "perm:content.article.create.team"
  target: "cap:content.create"
  coverage_type: "PARTIAL"
  conditions:
    type: "SCOPE_LIMITED"
    parameters:
      scope: "team"
    evaluation_point: "RUNTIME"
  status: "ACTIVE"
```

### 12.2. Exemple : DÃ©finition de rÃ´les avec hÃ©ritage

**Contexte :**
DÃ©finition d'une hiÃ©rarchie de rÃ´les pour le CMS.

```yaml
# RÃ´le de base : lecteur
role:
  id: "role:content.reader"
  name: "Content Reader"
  description: "Read-only access to content"
  domain: "content"
  parent_roles: []
  max_permission_level: "STANDARD"
  status: "ACTIVE"

# RÃ´le intermÃ©diaire : Ã©diteur
role:
  id: "role:content.editor"
  name: "Content Editor"
  description: "Create and edit content"
  domain: "content"
  parent_roles: ["role:content.reader"]
  max_permission_level: "ELEVATED"
  status: "ACTIVE"

# RÃ´le avancÃ© : manager
role:
  id: "role:content.manager"
  name: "Content Manager"
  description: "Full content management including deletion and publishing"
  domain: "content"
  parent_roles: ["role:content.editor"]
  max_permission_level: "CRITICAL"
  status: "ACTIVE"
```

### 12.3. Exemple : Attribution de permissions aux rÃ´les

```yaml
# Association RÃ´le-Permission : Ã©diteur avec Ã©dition d'articles
association:
  id: "assoc_role_editor_edit"
  type: "RolePermission"
  source: "role:content.editor"
  target: "perm:content.article.edit.team"
  grant_type: "DIRECT"
  scope_restriction:
    type: "TEAM"
  status: "ACTIVE"

# Association RÃ´le-Permission : manager avec suppression
association:
  id: "assoc_role_manager_delete"
  type: "RolePermission"
  source: "role:content.manager"
  target: "perm:content.article.delete.team"
  grant_type: "DIRECT"
  scope_restriction:
    type: "TEAM"
  status: "ACTIVE"
```

### 12.4. Exemple : RÃ©solution de contexte de capacitÃ©

**RequÃªte :**
```
GetCapabilityContext(identity: "user:alice", roles: ["role:content.editor"])
```

**RÃ©ponse :**
```yaml
capability_context:
  identity: "user:alice"
  roles: ["role:content.editor"]
  effective_permissions:
    - "perm:content.article.edit.team"
    - "perm:content.article.create.team"
    - "perm:content.article.read.any"  # hÃ©ritÃ© de role:content.reader
  effective_capabilities:
    - "cap:content.edit"
    - "cap:content.create"
    - "cap:content.read"
  scope_restrictions:
    "perm:content.article.edit.team": { type: "TEAM" }
    "perm:content.article.create.team": { type: "TEAM" }
    "perm:content.article.read.any": { type: "NONE" }
  computed_at: "2026-01-27T15:30:00Z"
```

---

## 13. Conclusion et Statut Contractuel

### Essence du Association Model Contract

Le modÃ¨le d'association de Master Butler est le **tissu connectif** qui lie les capacitÃ©s, permissions, et rÃ´les dans un systÃ¨me cohÃ©rent et traÃ§able. Il dÃ©finit comment ces entitÃ©s sont reliÃ©es, comment les droits se propagent, et comment les capacitÃ©s effectives sont calculÃ©es.

Ce modÃ¨le incarne la sÃ©paration entre :
- **La dÃ©finition des liens** (Master Butler)
- **L'attribution des rÃ´les** (systÃ¨me d'identitÃ©)
- **La dÃ©cision d'autorisation** (StrongFather)

### Phrase fondatrice

> **Le modÃ¨le d'association dÃ©finit les liens entre permissions, capacitÃ©s et rÃ´les, permettant le calcul des droits effectifs sans jamais participer Ã  la dÃ©cision d'autorisation.**

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il Ã©tablit des rÃ¨gles absolues qui ne peuvent Ãªtre contournÃ©es, nÃ©gociÃ©es, ou modifiÃ©es. Le contrat prime sur toute considÃ©ration pratique.

Toute implÃ©mentation du modÃ¨le d'association doit respecter intÃ©gralement ce document. Toute Ã©volution doit prÃ©server les invariants dÃ©finis ici.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** FONDATION â€” Non nÃ©gociable  
**RÃ©fÃ©rence :** Miyukini Core System v2.4

**RÃ©fÃ©rences croisÃ©es :**

- [Master Butler - Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md) : DÃ©finition et responsabilitÃ©s de Master Butler
- [Master Butler - Capability Registry Contract](./Master%20Butler%20-%20Capability%20Registry%20Contract.md) : Registre des capacitÃ©s
- [Master Butler - Permission Registry Contract](./Master%20Butler%20-%20Permission%20Registry%20Contract.md) : Registre des permissions
- [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) : DÃ©finitions canoniques
- [Miyukini Conceptual References - Tools et Toolkits](..//..//..//..//miyukini-webway-system//reference//_index.md) : Gouvernance des Outils
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) : Lois d'autonomie

---

## 14. Mini log â€” erreurs / warnings / ambiguites rencontrees et corrigees

### AmbiguÃ¯tÃ© A1 : Distinction entre association et attribution

**AmbiguÃ¯tÃ© rencontrÃ©e :**
La documentation fondatrice mentionne que Master Butler "connaÃ®t les associations entre rÃ´les et permissions" mais indique aussi que l'attribution des rÃ´les appartient au systÃ¨me d'identitÃ©. La frontiÃ¨re n'Ã©tait pas clairement dÃ©finie.

**DÃ©cision prise :**
Distinction explicite : Master Butler gÃ¨re les associations RÃ´le-Permission (quelles permissions un rÃ´le possÃ¨de) mais pas les attributions Utilisateur-RÃ´le (quels utilisateurs ont quels rÃ´les).

**Justification :**
Cette sÃ©paration prÃ©serve le principe de responsabilitÃ© unique : Master Butler catalogue les droits, le systÃ¨me d'identitÃ© gÃ¨re les identitÃ©s.

**Correction effectuÃ©e :**
Section 4.1 avec note explicite et invariant INV-ASSOC-MODEL-6.

### AmbiguÃ¯tÃ© A2 : Types de couverture des associations Permission-CapacitÃ©

**AmbiguÃ¯tÃ© rencontrÃ©e :**
La documentation fondatrice ne dÃ©finit pas formellement les diffÃ©rents types de couverture (complÃ¨te, partielle, conditionnelle).

**DÃ©cision prise :**
Trois types dÃ©finis : FULL (accÃ¨s complet), PARTIAL (accÃ¨s limitÃ©), CONDITIONAL (accÃ¨s soumis Ã  conditions runtime).

**Justification :**
Cette classification permet de modÃ©liser tous les scÃ©narios d'accÃ¨s tout en gardant la simplicitÃ© du modÃ¨le.

**Correction effectuÃ©e :**
Section 3.2 avec types de couverture et conditions d'association.

### AmbiguÃ¯tÃ© A3 : HÃ©ritage des permissions dans la hiÃ©rarchie de rÃ´les

**AmbiguÃ¯tÃ© rencontrÃ©e :**
La documentation fondatrice mentionne des "rÃ´les" comme ensembles de permissions mais ne dÃ©taille pas le mÃ©canisme d'hÃ©ritage.

**DÃ©cision prise :**
ModÃ¨le d'hÃ©ritage explicite : un rÃ´le hÃ©rite toutes les permissions de ses rÃ´les parents, de maniÃ¨re transitive, sans cycle.

**Justification :**
L'hÃ©ritage simplifie la gestion des rÃ´les complexes tout en maintenant la cohÃ©rence (pas de cycle = pas de boucle infinie).

**Correction effectuÃ©e :**
Section 5.3 avec rÃ¨gles d'hÃ©ritage et exemple de hiÃ©rarchie.

---

*Aucune autre erreur, warning, ou ambiguÃ¯tÃ© rencontrÃ©e lors de la rÃ©daction de ce document.*

