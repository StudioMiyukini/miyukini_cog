# Master Butler — Association Model Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **Master Butler Association Model Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit le modèle complet des associations entre permissions, capacités, et rôles dans le système Miyukini Core System v2.4.

Ce contrat définit :
- Le modèle formel d'association permission-capacité
- Le modèle formel d'association rôle-permission
- Les règles de création, modification et suppression d'associations
- Les invariants garantissant l'intégrité référentielle
- Les opérations autorisées sur les associations
- Les mécanismes de résolution des droits effectifs

### Portée

Ce contrat s'applique à **toutes les associations** du système et définit de manière absolue :

- La définition formelle d'une Association
- Les types d'associations (Permission-Capacité, Rôle-Permission)
- Les règles de création et validation des associations
- Les mécanismes de résolution des capacités et permissions effectives
- Les invariants non négociables du modèle d'association
- Les interactions avec les registres de capacités et permissions

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues que Master Butler applique sans exception. Ces règles ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et s'articule avec les documents contractuels existants :

- **[Master Butler - Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md)** : Définit la nature, le rôle, et les responsabilités de Master Butler
- **[Master Butler - Capability Registry Contract](./Master%20Butler%20-%20Capability%20Registry%20Contract.md)** : Définit le registre des capacités
- **[Master Butler - Permission Registry Contract](./Master%20Butler%20-%20Permission%20Registry%20Contract.md)** : Définit le registre des permissions
- **[Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)** : Définitions canoniques des termes
- **[Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Ce contrat respecte **LOI-1** (aucune dépendance externe critique) et **LOI-5** (coût proportionnel au hardware)

**Complémentarité :**

- Capability Registry Contract = définition des capacités (ce qui est techniquement possible)
- Permission Registry Contract = définition des permissions (les droits qui existent)
- Association Model Contract = liens entre permissions, capacités et rôles (comment ils sont connectés)

Ces contrats forment ensemble le système complet de gestion des capacités, permissions, et associations du système Miyukini Core System v2.4.

---

## 2. Définitions formelles

### 2.1. Association (définition générale)

**Définition formelle :**

Une **Association** est un lien formel, typé, et traçable entre deux entités du système de capacités et permissions. Une association définit une relation sémantique entre ces entités.

**Caractéristiques fondamentales :**

| Caractéristique | Description | Obligatoire |
|-----------------|-------------|-------------|
| **Identifiée** | Possède un identifiant unique | ✅ Oui |
| **Typée** | Possède un type défini (Permission-Capacité, Rôle-Permission) | ✅ Oui |
| **Directionnelle** | Possède une source et une cible | ✅ Oui |
| **Validée** | Référence des entités existantes | ✅ Oui |
| **Traçable** | Son historique est enregistré | ✅ Oui |
| **Métadonnée** | Peut porter des informations additionnelles | Optionnel |

**Structure formelle d'une Association :**

```
Association {
  id: AssociationId,           // Identifiant unique
  type: AssociationType,       // Type de l'association
  source: EntityReference,     // Entité source
  target: EntityReference,     // Entité cible
  metadata: AssociationMetadata, // Métadonnées additionnelles
  created_at: Timestamp,       // Date de création
  created_by: Identity,        // Créateur
  status: AssociationStatus    // Statut (Active, Suspended, Revoked)
}
```

**Invariants :**

- INV-ASSOC-1 : Toute association possède un identifiant unique
- INV-ASSOC-2 : Toute association référence des entités existantes et valides
- INV-ASSOC-3 : Toute association possède un type défini
- INV-ASSOC-4 : L'identifiant d'une association ne peut jamais être modifié

### 2.2. Types d'Associations

**Énoncé :**

Le système reconnaît deux types d'associations primaires, chacun avec sa sémantique propre.

**Types définis :**

| Type | Source | Cible | Sémantique |
|------|--------|-------|------------|
| **PermissionCapability** | Permission | Capacité | "Cette permission couvre cette capacité" |
| **RolePermission** | Rôle | Permission | "Ce rôle possède cette permission" |

**Types secondaires (dérivés) :**

| Type | Source | Cible | Sémantique |
|------|--------|-------|------------|
| **PermissionImplication** | Permission | Permission | "Cette permission implique cette autre permission" |
| **PermissionConflict** | Permission | Permission | "Ces permissions sont mutuellement exclusives" |
| **PermissionRequirement** | Permission | Permission | "Cette permission requiert cette autre permission" |

**Invariants :**

- INV-TYPE-1 : Toute association possède exactement un type
- INV-TYPE-2 : Le type détermine les contraintes de validation applicables
- INV-TYPE-3 : Les associations de type différent sont indépendantes

### 2.3. Référence d'Entité (EntityReference)

**Définition formelle :**

Une **EntityReference** identifie de manière unique une entité dans le système de capacités et permissions.

**Types de références :**

| Type | Format | Exemple |
|------|--------|---------|
| **CapabilityRef** | `cap:<CapabilityId>` | `cap:content.create` |
| **PermissionRef** | `perm:<PermissionId>` | `perm:content.article.create.any` |
| **RoleRef** | `role:<RoleId>` | `role:content.editor` |

**Structure formelle :**

```
EntityReference {
  type: EntityType,            // Capability, Permission, Role
  id: String,                  // Identifiant de l'entité
  version: Version?            // Version optionnelle (pour traçabilité)
}
```

**Invariants :**

- INV-REF-1 : Toute référence pointe vers une entité existante
- INV-REF-2 : Le type de référence est cohérent avec l'entité référencée
- INV-REF-3 : Une référence invalide (entité supprimée) rend l'association inactive

---

## 3. Association Permission-Capacité

### 3.1. Définition

**Définition formelle :**

Une **Association Permission-Capacité** lie une permission à une capacité, indiquant que cette permission "couvre" ou "donne accès" à cette capacité.

**Phrase fondatrice :**

> **Une permission couvre des capacités. Sans capacité associée, une permission est vide de sens.**

### 3.2. Modèle de données

**Structure d'une association Permission-Capacité :**

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
| **FULL** | Accès complet à la capacité | `content.manage.all` → `content.create` (FULL) |
| **PARTIAL** | Accès limité par conditions | `content.edit.own` → `content.edit` (PARTIAL, own_only) |
| **CONDITIONAL** | Accès soumis à conditions dynamiques | `workflow.approve.assigned` → `workflow.approve` (CONDITIONAL) |

### 3.3. Conditions d'association

**Définition :**

Les **conditions d'association** définissent les restrictions applicables à la couverture d'une capacité par une permission.

**Types de conditions :**

| Condition | Description | Exemple |
|-----------|-------------|---------|
| **OWNER_ONLY** | Limité aux entités possédées | `content.edit.own` : édition de ses propres contenus |
| **SCOPE_LIMITED** | Limité à un périmètre | `content.edit.team` : édition dans l'équipe |
| **TIME_BOUND** | Limité dans le temps | Permission temporaire avec date d'expiration |
| **COUNT_LIMITED** | Limité en nombre | Maximum N actions par période |
| **CONTEXT_DEPENDENT** | Dépend du contexte runtime | Évalué par StrongFather |

**Structure des conditions :**

```
AssociationConditions {
  type: ConditionType,
  parameters: Map<String, Any>,
  evaluation_point: "ASSOCIATION" | "RUNTIME"
}
```

**Règles :**

- R-COND-1 : Les conditions ASSOCIATION sont évaluées à la création
- R-COND-2 : Les conditions RUNTIME sont évaluées par StrongFather
- R-COND-3 : Les conditions sont cumulatives (AND logique)

### 3.4. Règles de création

**Préconditions :**

| Précondition | Description | Erreur si échoue |
|--------------|-------------|------------------|
| PRE-PC-1 | La permission existe et est ACTIVE ou DRAFT | `INVALID_PERMISSION_STATE` |
| PRE-PC-2 | La capacité existe et est ACTIVE | `INVALID_CAPABILITY_STATE` |
| PRE-PC-3 | L'association n'existe pas déjà | `DUPLICATE_ASSOCIATION` |
| PRE-PC-4 | Les conditions sont valides | `INVALID_CONDITIONS` |

**Postconditions :**

| Postcondition | Description |
|---------------|-------------|
| POST-PC-1 | L'association est créée avec statut ACTIVE |
| POST-PC-2 | La permission référence la capacité |
| POST-PC-3 | L'historique est mis à jour |
| POST-PC-4 | Les index sont mis à jour |

### 3.5. Cardinalité

**Règles de cardinalité :**

| Relation | Cardinalité | Description |
|----------|-------------|-------------|
| Permission → Capacité | 1..* | Une permission couvre au moins une capacité |
| Capacité → Permission | 0..* | Une capacité peut être couverte par plusieurs permissions |

**Invariants de cardinalité :**

- INV-CARD-PC-1 : Une permission ACTIVE possède au moins une association Permission-Capacité active
- INV-CARD-PC-2 : Une capacité peut exister sans permission associée (capacité technique non exposée)

---

## 4. Association Rôle-Permission

### 4.1. Définition

**Définition formelle :**

Une **Association Rôle-Permission** lie un rôle à une permission, indiquant que ce rôle "possède" ou "détient" cette permission.

**Phrase fondatrice :**

> **Un rôle est un ensemble nommé de permissions. L'association Rôle-Permission définit cet ensemble.**

**Note importante :**

Master Butler définit les associations Rôle-Permission, mais **ne gère pas l'attribution des rôles aux utilisateurs**. L'attribution des rôles appartient au système d'identité (hors-scope de Master Butler).

### 4.2. Modèle de données

**Structure d'une association Rôle-Permission :**

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

| Type | Description | Héritage |
|------|-------------|----------|
| **DIRECT** | Attribution directe | Non héritée |
| **INHERITED** | Attribution via hiérarchie de rôles | Héritée du rôle parent |
| **DELEGATED** | Attribution par délégation | Limitée dans le temps |

### 4.3. Restriction de portée

**Définition :**

Une **restriction de portée** limite le périmètre dans lequel la permission est accordée au rôle.

**Types de restrictions :**

| Type | Description | Exemple |
|------|-------------|---------|
| **NONE** | Aucune restriction | Permission globale |
| **ORGANIZATION** | Limité à l'organisation | Édition dans l'organisation |
| **TEAM** | Limité à l'équipe | Édition dans l'équipe |
| **PROJECT** | Limité au projet | Édition dans le projet |
| **CUSTOM** | Restriction personnalisée | Périmètre défini par configuration |

**Structure de restriction :**

```
ScopeRestriction {
  type: ScopeType,
  scope_id: String?,          // Identifiant du périmètre (si applicable)
  parameters: Map<String, Any>? // Paramètres additionnels
}
```

### 4.4. Règles de création

**Préconditions :**

| Précondition | Description | Erreur si échoue |
|--------------|-------------|------------------|
| PRE-RP-1 | Le rôle existe et est ACTIVE | `INVALID_ROLE_STATE` |
| PRE-RP-2 | La permission existe et est ACTIVE | `INVALID_PERMISSION_STATE` |
| PRE-RP-3 | L'association n'existe pas déjà | `DUPLICATE_ASSOCIATION` |
| PRE-RP-4 | Le niveau de permission est compatible avec le rôle | `INCOMPATIBLE_PERMISSION_LEVEL` |
| PRE-RP-5 | Pas de conflit avec les permissions existantes du rôle | `CONFLICTING_PERMISSION` |

**Postconditions :**

| Postcondition | Description |
|---------------|-------------|
| POST-RP-1 | L'association est créée avec statut ACTIVE |
| POST-RP-2 | Le rôle possède la permission |
| POST-RP-3 | L'historique est mis à jour |
| POST-RP-4 | Les capacités effectives du rôle sont recalculées |

### 4.5. Cardinalité

**Règles de cardinalité :**

| Relation | Cardinalité | Description |
|----------|-------------|-------------|
| Rôle → Permission | 0..* | Un rôle peut posséder plusieurs permissions |
| Permission → Rôle | 0..* | Une permission peut être attribuée à plusieurs rôles |

**Invariants de cardinalité :**

- INV-CARD-RP-1 : Un rôle peut exister sans permission (rôle vide, en cours de définition)
- INV-CARD-RP-2 : Une permission peut exister sans être attribuée à un rôle

---

## 5. Modèle de Rôle

### 5.1. Définition

**Définition formelle :**

Un **Rôle** est un ensemble nommé de permissions, identifiable et attribuable. Master Butler connaît les rôles et leurs permissions associées, mais ne gère pas l'attribution des rôles aux utilisateurs.

**Caractéristiques d'un rôle :**

| Caractéristique | Description | Obligatoire |
|-----------------|-------------|-------------|
| **Identifié** | Possède un identifiant unique | ✅ Oui |
| **Nommé** | Possède un nom lisible | ✅ Oui |
| **Documenté** | Possède une description | ✅ Oui |
| **Hiérarchique** | Peut hériter d'autres rôles | Optionnel |
| **Limité** | Peut avoir un niveau maximum de permission | Optionnel |

### 5.2. Structure d'un Rôle

```
Role {
  id: RoleId,                    // Identifiant unique
  name: String,                  // Nom lisible
  description: String,           // Description du rôle
  domain: String,                // Domaine fonctionnel
  parent_roles: Set<RoleId>?,    // Rôles parents (héritage)
  max_permission_level: PermissionLevel?, // Niveau maximum autorisé
  metadata: RoleMetadata,
  status: RoleStatus             // DRAFT, ACTIVE, DEPRECATED, RETIRED
}
```

### 5.3. Hiérarchie de Rôles

**Définition :**

Les rôles peuvent former une **hiérarchie** où un rôle enfant hérite des permissions de ses rôles parents.

**Règles d'héritage :**

| Règle | Description | Statut |
|-------|-------------|--------|
| R-HIER-1 | Un rôle enfant hérite toutes les permissions de ses parents | NON NÉGOCIABLE |
| R-HIER-2 | L'héritage est transitif (grand-parent → parent → enfant) | NON NÉGOCIABLE |
| R-HIER-3 | Aucun cycle n'est autorisé dans la hiérarchie | NON NÉGOCIABLE |
| R-HIER-4 | Un rôle ne peut pas avoir un niveau supérieur à ses parents | NON NÉGOCIABLE |

**Exemple de hiérarchie :**

```
role:admin
├── hérite de: role:manager
│   ├── hérite de: role:editor
│   │   └── permissions: [content.edit.*, content.create.*]
│   └── permissions: [content.delete.*, content.publish.*]
└── permissions: [admin.*, system.config.*]

Permissions effectives de admin:
- admin.*
- system.config.*
- content.delete.*
- content.publish.*
- content.edit.*
- content.create.*
```

### 5.4. Registre des Rôles

**Définition :**

Le **Registre des Rôles** est la structure de Master Butler qui contient l'inventaire de tous les rôles définis dans le système.

**Structure :**

```
RoleRegistry {
  roles: Map<RoleId, Role>,                    // Index principal
  by_domain: Map<String, Set<RoleId>>,         // Index par domaine
  hierarchy: RoleHierarchyGraph,               // Graphe de hiérarchie
  history: RoleHistory                         // Historique des modifications
}
```

---

## 6. Résolution des Droits Effectifs

### 6.1. Capacités effectives d'une Permission

**Définition :**

Les **capacités effectives** d'une permission sont l'ensemble des capacités couvertes par cette permission, incluant les capacités des permissions impliquées.

**Algorithme de résolution :**

```
ResolvEffectiveCapabilities(permission_id) {
  result = Set()
  visited = Set()
  
  function resolve(perm_id) {
    if perm_id in visited: return
    visited.add(perm_id)
    
    permission = GetPermission(perm_id)
    
    // Ajouter les capacités directes
    for assoc in GetPermissionCapabilityAssociations(perm_id):
      if assoc.status == ACTIVE:
        result.add(assoc.target)
    
    // Résoudre les permissions impliquées
    for implied_perm_id in permission.implied_permissions:
      resolve(implied_perm_id)
  }
  
  resolve(permission_id)
  return result
}
```

**Invariants :**

- INV-RES-1 : La résolution termine toujours (pas de cycle)
- INV-RES-2 : Le résultat est stable pour un état donné du registre
- INV-RES-3 : Les capacités retournées sont toutes ACTIVE

### 6.2. Permissions effectives d'un Rôle

**Définition :**

Les **permissions effectives** d'un rôle sont l'ensemble des permissions possédées par ce rôle, incluant les permissions héritées des rôles parents.

**Algorithme de résolution :**

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
    
    // Résoudre les rôles parents
    for parent_id in role.parent_roles:
      resolve(parent_id)
  }
  
  resolve(role_id)
  return result
}
```

### 6.3. Capacités effectives d'un Rôle

**Définition :**

Les **capacités effectives** d'un rôle sont l'union des capacités effectives de toutes ses permissions effectives.

**Algorithme de résolution :**

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

### 6.4. Contexte de Capacité

**Définition :**

Le **Contexte de Capacité** est la structure qui agrège toutes les informations de droits pour un contexte donné (utilisateur, rôles, périmètre).

**Structure :**

```
CapabilityContext {
  identity: IdentityRef,                  // Identité du demandeur
  roles: Set<RoleId>,                     // Rôles du demandeur
  effective_permissions: Set<PermissionRef>, // Permissions effectives
  effective_capabilities: Set<CapabilityRef>, // Capacités effectives
  scope_restrictions: Map<PermissionRef, ScopeRestriction>, // Restrictions par permission
  computed_at: Timestamp                  // Date de calcul
}
```

**Règles de calcul :**

| Règle | Description | Statut |
|-------|-------------|--------|
| R-CTX-1 | Le contexte est calculé à la demande | NON NÉGOCIABLE |
| R-CTX-2 | Le calcul n'est pas caché entre requêtes | NON NÉGOCIABLE |
| R-CTX-3 | Le contexte est une projection, pas une décision | NON NÉGOCIABLE |
| R-CTX-4 | StrongFather utilise le contexte pour décider | NON NÉGOCIABLE |

---

## 7. Opérations sur les Associations

### 7.1. Création d'Association Permission-Capacité

**Signature conceptuelle :**

```
CreatePermissionCapabilityAssociation(
  permission_id: PermissionId,
  capability_id: CapabilityId,
  coverage_type: CoverageType,
  conditions: AssociationConditions?
) → Result<Association, AssociationError>
```

**Séquence :**

1. Valider l'existence et l'état de la permission
2. Valider l'existence et l'état de la capacité
3. Vérifier l'absence de duplication
4. Valider les conditions si présentes
5. Créer l'association
6. Mettre à jour les index
7. Historiser l'événement
8. Retourner l'association créée

### 7.2. Création d'Association Rôle-Permission

**Signature conceptuelle :**

```
CreateRolePermissionAssociation(
  role_id: RoleId,
  permission_id: PermissionId,
  grant_type: GrantType,
  scope_restriction: ScopeRestriction?
) → Result<Association, AssociationError>
```

**Séquence :**

1. Valider l'existence et l'état du rôle
2. Valider l'existence et l'état de la permission
3. Vérifier la compatibilité du niveau de permission avec le rôle
4. Vérifier l'absence de conflit avec les permissions existantes
5. Vérifier l'absence de duplication
6. Créer l'association
7. Mettre à jour les index
8. Recalculer les capacités effectives du rôle
9. Historiser l'événement
10. Retourner l'association créée

### 7.3. Suspension d'une Association

**Signature conceptuelle :**

```
SuspendAssociation(
  association_id: AssociationId,
  reason: String
) → Result<Association, AssociationError>
```

**Règles :**

| Règle | Description | Statut |
|-------|-------------|--------|
| R-SUSP-1 | Seules les associations ACTIVE peuvent être suspendues | NON NÉGOCIABLE |
| R-SUSP-2 | La suspension est réversible | NON NÉGOCIABLE |
| R-SUSP-3 | Une association suspendue n'est pas comptée dans les résolutions | NON NÉGOCIABLE |
| R-SUSP-4 | La raison de suspension est obligatoire | NON NÉGOCIABLE |

### 7.4. Révocation d'une Association

**Signature conceptuelle :**

```
RevokeAssociation(
  association_id: AssociationId,
  reason: String
) → Result<(), AssociationError>
```

**Règles :**

| Règle | Description | Statut |
|-------|-------------|--------|
| R-REV-1 | La révocation est irréversible | NON NÉGOCIABLE |
| R-REV-2 | L'historique conserve la trace complète | NON NÉGOCIABLE |
| R-REV-3 | Les capacités effectives sont recalculées | NON NÉGOCIABLE |
| R-REV-4 | La raison de révocation est obligatoire | NON NÉGOCIABLE |

### 7.5. Interrogation des Associations

**Requêtes disponibles :**

| Requête | Description | Paramètres |
|---------|-------------|------------|
| `GetAssociation` | Récupère une association par ID | `association_id` |
| `GetPermissionCapabilities` | Capacités d'une permission | `permission_id`, `include_implied` |
| `GetCapabilityPermissions` | Permissions couvrant une capacité | `capability_id` |
| `GetRolePermissions` | Permissions d'un rôle | `role_id`, `include_inherited` |
| `GetPermissionRoles` | Rôles possédant une permission | `permission_id` |
| `GetRoleCapabilities` | Capacités effectives d'un rôle | `role_id` |
| `ListAssociations` | Liste les associations selon filtres | `type`, `source`, `target`, `status` |

---

## 8. Gestion de l'Intégrité Référentielle

### 8.1. Principes d'intégrité

**Règle fondamentale :**

> **Aucune association ne peut référencer une entité inexistante ou invalide.**

**Types d'intégrité :**

| Type | Description | Application |
|------|-------------|-------------|
| **Création** | Validation à la création | Les références doivent exister et être valides |
| **Évolution** | Validation lors des modifications | Les modifications ne cassent pas les références |
| **Cascade** | Propagation des changements | La suppression d'une entité impacte ses associations |

### 8.2. Règles de cascade

**Suppression d'une Capacité :**

| Scénario | Action sur les associations Permission-Capacité |
|----------|------------------------------------------------|
| Capacité supprimée | Association devient INVALID |
| Permission n'a plus de capacité active | Permission invalidée (notification) |

**Suppression d'une Permission :**

| Scénario | Action sur les associations Rôle-Permission |
|----------|---------------------------------------------|
| Permission supprimée | Association révoquée automatiquement |
| Rôle n'a plus de permission | Aucune action (rôle peut être vide) |

**Suppression d'un Rôle :**

| Scénario | Action sur les associations |
|----------|----------------------------|
| Rôle supprimé | Associations Rôle-Permission révoquées |
| Rôles enfants | Héritage cassé (notification) |

### 8.3. Vérification d'intégrité

**Opération de vérification :**

```
VerifyAssociationIntegrity() → IntegrityReport {
  orphan_associations: List<AssociationId>,  // Associations avec références invalides
  broken_implications: List<PermissionId>,   // Permissions avec implications cassées
  cycle_detected: List<CycleInfo>,           // Cycles détectés
  inconsistencies: List<InconsistencyInfo>   // Autres incohérences
}
```

**Fréquence de vérification :**

| Événement | Vérification |
|-----------|--------------|
| Création d'association | Vérification locale |
| Suppression d'entité | Vérification des associations impactées |
| Modification d'état | Vérification de cohérence |
| Maintenance planifiée | Vérification globale |

---

## 9. Invariants Non Négociables

### INV-ASSOC-MODEL-1 : Intégrité référentielle

> **Toute association référence des entités existantes et valides.**

**Implication :** Aucune association orpheline. Aucune référence vers une entité supprimée ou invalide.

### INV-ASSOC-MODEL-2 : Unicité des associations

> **Une association entre deux entités données est unique pour un type donné.**

**Implication :** Pas de duplication. Une permission ne peut pas être associée deux fois à la même capacité.

### INV-ASSOC-MODEL-3 : Absence de cycle

> **Les graphes d'implication (permissions) et de hiérarchie (rôles) sont acycliques.**

**Implication :** La résolution des droits effectifs termine toujours. Pas de boucle infinie.

### INV-ASSOC-MODEL-4 : Traçabilité complète

> **Toute modification d'association est tracée avec contexte complet.**

**Implication :** Audit possible. Création, suspension, révocation : tout est enregistré.

### INV-ASSOC-MODEL-5 : Cohérence des états

> **L'état d'une association est cohérent avec l'état de ses entités.**

**Implication :** Une association ACTIVE ne peut pas référencer une entité RETIRED. Les états se propagent.

### INV-ASSOC-MODEL-6 : Non-attribution

> **Master Butler définit les associations, mais n'attribue jamais de rôle aux utilisateurs.**

**Implication :** L'attribution des rôles appartient au système d'identité. Master Butler connaît les rôles et leurs permissions, pas qui les possède.

### INV-ASSOC-MODEL-7 : Non-décision

> **Master Butler fournit les associations et calcule les droits effectifs, mais ne décide jamais si un accès est autorisé.**

**Implication :** La décision appartient à StrongFather. Master Butler informe, ne décide pas.

---

## 10. Schémas ASCII

### 10.1. Modèle d'Association Global

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                       ASSOCIATION MODEL                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌───────────────┐                           ┌───────────────┐              │
│  │     ROLE      │                           │  CAPABILITY   │              │
│  │  (ensemble)   │                           │  (pouvoir)    │              │
│  └───────┬───────┘                           └───────▲───────┘              │
│          │                                           │                       │
│          │ RolePermission                            │ PermissionCapability  │
│          │ Association                               │ Association           │
│          │                                           │                       │
│          ▼                                           │                       │
│  ┌───────────────┐                                   │                       │
│  │  PERMISSION   │───────────────────────────────────┘                       │
│  │   (droit)     │                                                           │
│  └───────────────┘                                                           │
│                                                                              │
│  ═══════════════════════════════════════════════════════════════════════    │
│                                                                              │
│  FLUX DE RÉSOLUTION :                                                        │
│                                                                              │
│  Role ──► Permissions effectives ──► Capacités effectives                   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 10.2. Association Permission-Capacité

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                   PERMISSION-CAPABILITY ASSOCIATION                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────────────────┐         ┌──────────────────────────┐          │
│  │  PERMISSION              │         │  CAPABILITY              │          │
│  │  content.article.edit.any│         │  content.edit            │          │
│  └────────────┬─────────────┘         └──────────▲───────────────┘          │
│               │                                  │                           │
│               │         ┌────────────────────────┘                           │
│               │         │                                                    │
│               ▼         │                                                    │
│       ┌─────────────────┴─────────────────┐                                  │
│       │  ASSOCIATION                       │                                  │
│       ├────────────────────────────────────┤                                  │
│       │  id: assoc_001                     │                                  │
│       │  type: PermissionCapability        │                                  │
│       │  source: perm:content.article...   │                                  │
│       │  target: cap:content.edit          │                                  │
│       │  coverage_type: FULL               │                                  │
│       │  conditions: null                  │                                  │
│       │  status: ACTIVE                    │                                  │
│       └────────────────────────────────────┘                                  │
│                                                                              │
│  ═══════════════════════════════════════════════════════════════════════    │
│                                                                              │
│  EXEMPLE AVEC CONDITIONS :                                                   │
│                                                                              │
│  ┌──────────────────────────┐         ┌──────────────────────────┐          │
│  │  PERMISSION              │         │  CAPABILITY              │          │
│  │  content.draft.edit.own  │         │  content.edit            │          │
│  └────────────┬─────────────┘         └──────────▲───────────────┘          │
│               │                                  │                           │
│               ▼                                  │                           │
│       ┌─────────────────────────────────────────┴┐                           │
│       │  ASSOCIATION                             │                           │
│       ├──────────────────────────────────────────┤                           │
│       │  coverage_type: PARTIAL                  │                           │
│       │  conditions:                             │                           │
│       │    type: OWNER_ONLY                      │                           │
│       │    evaluation_point: RUNTIME             │                           │
│       └──────────────────────────────────────────┘                           │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 10.3. Association Rôle-Permission avec Héritage

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    ROLE HIERARCHY AND PERMISSION INHERITANCE                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│                    ┌──────────────────┐                                      │
│                    │  role:admin      │                                      │
│                    │  ─────────────── │                                      │
│                    │  Permissions:    │                                      │
│                    │   - admin.*      │                                      │
│                    └────────┬─────────┘                                      │
│                             │ inherits                                       │
│                             ▼                                                │
│                    ┌──────────────────┐                                      │
│                    │  role:manager    │                                      │
│                    │  ─────────────── │                                      │
│                    │  Permissions:    │                                      │
│                    │   - content.*    │                                      │
│                    └────────┬─────────┘                                      │
│                             │ inherits                                       │
│                             ▼                                                │
│                    ┌──────────────────┐                                      │
│                    │  role:editor     │                                      │
│                    │  ─────────────── │                                      │
│                    │  Permissions:    │                                      │
│                    │   - content.edit │                                      │
│                    └──────────────────┘                                      │
│                                                                              │
│  ═══════════════════════════════════════════════════════════════════════    │
│                                                                              │
│  PERMISSIONS EFFECTIVES DE role:admin :                                      │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │  admin.*        ◄─── Direct (DIRECT)                                │    │
│  │  content.*      ◄─── Hérité de manager (INHERITED)                  │    │
│  │  content.edit   ◄─── Hérité de editor via manager (INHERITED)       │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 10.4. Flux de Résolution des Capacités Effectives

```
┌─────────────────────────────────────────────────────────────────────────────┐
│              EFFECTIVE CAPABILITIES RESOLUTION FLOW                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ENTRÉE: role:editor                                                         │
│                                                                              │
│  ÉTAPE 1 : Résoudre les permissions effectives                               │
│  ──────────────────────────────────────────────                              │
│                                                                              │
│  role:editor                                                                 │
│      │                                                                       │
│      ├── RolePermission ──► perm:content.article.edit.team                  │
│      └── RolePermission ──► perm:content.article.create.team                │
│                                                                              │
│  Permissions effectives: {                                                   │
│    perm:content.article.edit.team,                                          │
│    perm:content.article.create.team                                         │
│  }                                                                           │
│                                                                              │
│  ÉTAPE 2 : Résoudre les capacités de chaque permission                       │
│  ─────────────────────────────────────────────────────                       │
│                                                                              │
│  perm:content.article.edit.team                                             │
│      │                                                                       │
│      └── PermissionCapability ──► cap:content.edit                          │
│                                                                              │
│  perm:content.article.create.team                                           │
│      │                                                                       │
│      └── PermissionCapability ──► cap:content.create                        │
│                                                                              │
│  ÉTAPE 3 : Union des capacités                                               │
│  ─────────────────────────────                                               │
│                                                                              │
│  Capacités effectives de role:editor: {                                     │
│    cap:content.edit,                                                        │
│    cap:content.create                                                       │
│  }                                                                           │
│                                                                              │
│  SORTIE: CapabilityContext                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │  roles: [role:editor]                                               │    │
│  │  effective_permissions: [perm:content.article.edit.team, ...]       │    │
│  │  effective_capabilities: [cap:content.edit, cap:content.create]     │    │
│  │  scope_restrictions: { perm:...: TEAM }                             │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 11. Conformité aux Lois d'Autonomie Système

Ce contrat respecte les Lois d'Autonomie Système définies dans [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md).

### LOI-1 : Aucune dépendance externe critique à l'exécution

**Conformité :** Conforme

Le modèle d'association est entièrement local :

- **Stockage local** : Les associations sont stockées localement
- **Résolution locale** : Tous les calculs de droits effectifs s'exécutent localement
- **Aucune API externe** : Aucun service distant n'est requis pour gérer les associations

**Vérification LOI-1** : *"Le modèle d'association fonctionne-t-il si le réseau est indisponible ?"* → **Oui.**

### LOI-5 : Le coût doit être proportionnel au hardware

**Conformité :** Conforme

Le modèle d'association a une empreinte minimale :

- **Données légères** : Les associations sont des liens simples entre identifiants
- **Calculs optimisés** : La résolution des droits effectifs est O(n) où n est le nombre d'associations
- **Pas de workers** : Aucun processus en arrière-plan pour la gestion des associations
- **Mémoire prévisible** : Proportionnelle au nombre d'associations

**Vérification LOI-5** : *"Le modèle fonctionne-t-il sur un Raspberry Pi 4 ?"* → **Oui.** Un système typique avec quelques milliers d'associations représente quelques dizaines de kilo-octets.

### Synthèse de conformité

| Loi | Statut | Raison |
|-----|--------|--------|
| LOI-1 | ✅ Conforme | Modèle local, aucune dépendance externe |
| LOI-5 | ✅ Conforme | Données légères, calculs efficaces |

---

## 12. Exemples Concrets

### 12.1. Exemple : Création d'associations pour un module CMS

**Contexte :**
Le module CMS définit ses permissions et les associe aux capacités.

**Déclarations :**

```yaml
# Association Permission-Capacité : édition de contenu
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

# Association Permission-Capacité : création de contenu
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

### 12.2. Exemple : Définition de rôles avec héritage

**Contexte :**
Définition d'une hiérarchie de rôles pour le CMS.

```yaml
# Rôle de base : lecteur
role:
  id: "role:content.reader"
  name: "Content Reader"
  description: "Read-only access to content"
  domain: "content"
  parent_roles: []
  max_permission_level: "STANDARD"
  status: "ACTIVE"

# Rôle intermédiaire : éditeur
role:
  id: "role:content.editor"
  name: "Content Editor"
  description: "Create and edit content"
  domain: "content"
  parent_roles: ["role:content.reader"]
  max_permission_level: "ELEVATED"
  status: "ACTIVE"

# Rôle avancé : manager
role:
  id: "role:content.manager"
  name: "Content Manager"
  description: "Full content management including deletion and publishing"
  domain: "content"
  parent_roles: ["role:content.editor"]
  max_permission_level: "CRITICAL"
  status: "ACTIVE"
```

### 12.3. Exemple : Attribution de permissions aux rôles

```yaml
# Association Rôle-Permission : éditeur avec édition d'articles
association:
  id: "assoc_role_editor_edit"
  type: "RolePermission"
  source: "role:content.editor"
  target: "perm:content.article.edit.team"
  grant_type: "DIRECT"
  scope_restriction:
    type: "TEAM"
  status: "ACTIVE"

# Association Rôle-Permission : manager avec suppression
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

### 12.4. Exemple : Résolution de contexte de capacité

**Requête :**
```
GetCapabilityContext(identity: "user:alice", roles: ["role:content.editor"])
```

**Réponse :**
```yaml
capability_context:
  identity: "user:alice"
  roles: ["role:content.editor"]
  effective_permissions:
    - "perm:content.article.edit.team"
    - "perm:content.article.create.team"
    - "perm:content.article.read.any"  # hérité de role:content.reader
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

Le modèle d'association de Master Butler est le **tissu connectif** qui lie les capacités, permissions, et rôles dans un système cohérent et traçable. Il définit comment ces entités sont reliées, comment les droits se propagent, et comment les capacités effectives sont calculées.

Ce modèle incarne la séparation entre :
- **La définition des liens** (Master Butler)
- **L'attribution des rôles** (système d'identité)
- **La décision d'autorisation** (StrongFather)

### Phrase fondatrice

> **Le modèle d'association définit les liens entre permissions, capacités et rôles, permettant le calcul des droits effectifs sans jamais participer à la décision d'autorisation.**

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

Toute implémentation du modèle d'association doit respecter intégralement ce document. Toute évolution doit préserver les invariants définis ici.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** FONDATION — Non négociable  
**Référence :** Miyukini Core System v2.4

**Références croisées :**

- [Master Butler - Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md) : Définition et responsabilités de Master Butler
- [Master Butler - Capability Registry Contract](./Master%20Butler%20-%20Capability%20Registry%20Contract.md) : Registre des capacités
- [Master Butler - Permission Registry Contract](./Master%20Butler%20-%20Permission%20Registry%20Contract.md) : Registre des permissions
- [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) : Définitions canoniques
- [Miyukini Conceptual References - Tools et Toolkits](../../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) : Gouvernance des Outils
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) : Lois d'autonomie

---

## 14. Mini log — erreurs / warnings / ambiguites rencontrees et corrigees

### Ambiguïté A1 : Distinction entre association et attribution

**Ambiguïté rencontrée :**
La documentation fondatrice mentionne que Master Butler "connaît les associations entre rôles et permissions" mais indique aussi que l'attribution des rôles appartient au système d'identité. La frontière n'était pas clairement définie.

**Décision prise :**
Distinction explicite : Master Butler gère les associations Rôle-Permission (quelles permissions un rôle possède) mais pas les attributions Utilisateur-Rôle (quels utilisateurs ont quels rôles).

**Justification :**
Cette séparation préserve le principe de responsabilité unique : Master Butler catalogue les droits, le système d'identité gère les identités.

**Correction effectuée :**
Section 4.1 avec note explicite et invariant INV-ASSOC-MODEL-6.

### Ambiguïté A2 : Types de couverture des associations Permission-Capacité

**Ambiguïté rencontrée :**
La documentation fondatrice ne définit pas formellement les différents types de couverture (complète, partielle, conditionnelle).

**Décision prise :**
Trois types définis : FULL (accès complet), PARTIAL (accès limité), CONDITIONAL (accès soumis à conditions runtime).

**Justification :**
Cette classification permet de modéliser tous les scénarios d'accès tout en gardant la simplicité du modèle.

**Correction effectuée :**
Section 3.2 avec types de couverture et conditions d'association.

### Ambiguïté A3 : Héritage des permissions dans la hiérarchie de rôles

**Ambiguïté rencontrée :**
La documentation fondatrice mentionne des "rôles" comme ensembles de permissions mais ne détaille pas le mécanisme d'héritage.

**Décision prise :**
Modèle d'héritage explicite : un rôle hérite toutes les permissions de ses rôles parents, de manière transitive, sans cycle.

**Justification :**
L'héritage simplifie la gestion des rôles complexes tout en maintenant la cohérence (pas de cycle = pas de boucle infinie).

**Correction effectuée :**
Section 5.3 avec règles d'héritage et exemple de hiérarchie.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
