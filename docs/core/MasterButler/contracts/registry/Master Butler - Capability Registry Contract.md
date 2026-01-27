# Master Butler — Capability Registry Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **Master Butler Capability Registry Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit le modèle du registre des capacités dans le système Miyukini Core System v2.4.

Ce contrat définit :
- La structure formelle du registre des capacités
- Les règles de déclaration et d'enregistrement
- Les métadonnées obligatoires et optionnelles
- Les invariants du registre
- Les opérations autorisées sur le registre
- Les relations entre capacités

### Portée

Ce contrat s'applique à **toute instance de Master Butler** et définit de manière absolue :
- La définition formelle d'une Capacité (Capability)
- La structure du Registre des Capacités (Capability Registry)
- Les règles de déclaration (Declaration Rules)
- Les métadonnées des capacités (Capability Metadata)
- Les relations entre capacités (Capability Relations)
- Les invariants du registre
- Les opérations autorisées et interdites

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues que Master Butler applique sans exception. Ces règles ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et s'articule avec les documents contractuels existants :

- **[Master Butler - Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md)** : Définit la nature, le rôle, et les responsabilités de Master Butler
- **Master Butler - Permission Registry Contract** : Définit le registre des permissions (contrat complémentaire)
- **[Miyukini Conceptual References - Tools et Toolkits](../../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)** : Définit les concepts de Tool et Toolkit
- **[Miyukini Framework - Lois Autonomie Systeme](../../../../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md)** : Ce contrat respecte **LOI-1** (aucune dépendance externe critique) et **LOI-5** (coût proportionnel au hardware) en garantissant que le registre est local, léger, et autonome.

**Complémentarité :**
- Master Butler Documentation Fondatrice = définition conceptuelle et philosophique
- Master Butler Capability Registry Contract = modèle technique du registre des capacités
- Master Butler Permission Registry Contract = modèle technique du registre des permissions

Ces contrats forment ensemble le système complet de catalogage des capacités et permissions du système Miyukini Core System v2.4.

---

## 2. Définitions formelles

### 2.1. Capacité (Capability)

**Définition formelle :**

Une **Capacité** est un pouvoir technique intrinsèque à un composant du système Miyukini. Elle représente ce qu'un composant peut faire fonctionnellement, indépendamment de toute permission ou décision.

**Caractéristiques formelles :**

- **Identité unique :** Chaque capacité possède un identifiant unique et immuable (CapabilityId)
- **Intrinsèque :** La capacité est intrinsèque au composant qui la possède
- **Technique :** La capacité décrit un pouvoir fonctionnel (pas métier)
- **Déclarative :** La capacité est déclarée par le composant qui la possède
- **Documentée :** La capacité possède des métadonnées descriptives
- **Gouvernée :** La capacité est soumise à la gouvernance des Cores

**Structure formelle d'une Capacité :**

```
Capability {
  id: CapabilityId,           // Identifiant unique et immuable
  name: String,               // Nom lisible humain
  description: String,        // Description de la capacité
  source: SourceIdentity,     // Module/composant qui déclare la capacité
  category: CapabilityCategory, // Catégorie de la capacité
  metadata: CapabilityMetadata, // Métadonnées additionnelles
  relations: CapabilityRelations, // Relations avec d'autres capacités
  created_at: Timestamp,      // Date de création
  version: Version,           // Version de la capacité
  status: CapabilityStatus    // Statut (Active, Deprecated, Removed)
}
```

**Invariants :**
- INV-CAP-1 : Toute capacité possède un CapabilityId unique et immuable
- INV-CAP-2 : Toute capacité est déclarée par exactement un composant source
- INV-CAP-3 : Toute capacité possède un nom, une description, et une catégorie
- INV-CAP-4 : L'identifiant d'une capacité ne peut jamais être modifié
- INV-CAP-5 : Une capacité supprimée ne peut jamais être recréée avec le même identifiant

### 2.2. Identifiant de Capacité (CapabilityId)

**Définition formelle :**

Un **CapabilityId** est l'identifiant unique et immuable d'une capacité dans le registre. Il suit un format canonique qui encode le domaine, le module, et l'action.

**Format canonique :**

```
<domain>.<module>.<action>[.<qualifier>]
```

**Exemples :**
- `content.create` : Capacité de créer du contenu
- `content.edit.own` : Capacité de modifier son propre contenu
- `hierarchy.reorder` : Capacité de réorganiser une hiérarchie
- `media.upload` : Capacité de téléverser des médias
- `search.index` : Capacité d'indexer pour la recherche
- `auth.login` : Capacité de s'authentifier

**Règles de nommage :**
- R-ID-1 : Le format est strictement `<domain>.<module>.<action>[.<qualifier>]`
- R-ID-2 : Tous les segments sont en minuscules, sans accents
- R-ID-3 : Les segments sont séparés par des points (.)
- R-ID-4 : Chaque segment contient uniquement des lettres, chiffres, et underscores
- R-ID-5 : Le qualifieur est optionnel et ajoute une précision sémantique
- R-ID-6 : L'identifiant est unique dans l'ensemble du système

**Invariants :**
- INV-ID-1 : Tout CapabilityId respecte le format canonique
- INV-ID-2 : Tout CapabilityId est unique dans le registre
- INV-ID-3 : Un CapabilityId ne peut jamais être modifié après création
- INV-ID-4 : Deux capacités différentes ne peuvent jamais avoir le même CapabilityId

### 2.3. Source de Capacité (SourceIdentity)

**Définition formelle :**

Une **SourceIdentity** identifie le composant qui déclare une capacité. Elle permet de tracer l'origine de chaque capacité dans le registre.

**Types de sources :**

| Type | Description | Exemple |
|------|-------------|---------|
| **Module SPM** | Module du Standard Product Model | `spm.cms.content` |
| **Core** | Core du Miyukini Core System | `core.kindmother` |
| **Operator** | Opérateur (application) | `operator.my_app` |
| **Tool** | Outil du système | `tool.layout.render` |

**Structure formelle :**

```
SourceIdentity {
  type: SourceType,           // Module, Core, Operator, Tool
  identifier: String,         // Identifiant unique de la source
  version: Version,           // Version de la source
  environment: EnvironmentId  // Environnement de déclaration
}
```

**Invariants :**
- INV-SRC-1 : Toute capacité possède une SourceIdentity valide
- INV-SRC-2 : Une SourceIdentity identifie de manière unique le composant source
- INV-SRC-3 : La version de la source est incluse pour traçabilité

### 2.4. Catégorie de Capacité (CapabilityCategory)

**Définition formelle :**

Une **CapabilityCategory** classifie les capacités par domaine fonctionnel pour faciliter la découverte et l'organisation.

**Catégories standard :**

| Catégorie | Description | Exemples |
|-----------|-------------|----------|
| **Data** | Capacités liées aux données | `content.create`, `content.read` |
| **Hierarchy** | Capacités liées aux hiérarchies | `hierarchy.reorder`, `hierarchy.create` |
| **Media** | Capacités liées aux médias | `media.upload`, `media.delete` |
| **Search** | Capacités liées à la recherche | `search.index`, `search.query` |
| **Auth** | Capacités liées à l'authentification | `auth.login`, `auth.logout` |
| **Admin** | Capacités d'administration | `admin.config`, `admin.audit` |
| **UI** | Capacités d'interface utilisateur | `ui.render`, `ui.theme` |
| **IO** | Capacités d'entrée/sortie | `io.file.read`, `io.file.write` |
| **System** | Capacités système | `system.health`, `system.metrics` |

**Invariants :**
- INV-CAT-1 : Toute capacité appartient à exactement une catégorie
- INV-CAT-2 : Les catégories sont prédéfinies et extensibles par l'environnement
- INV-CAT-3 : Une catégorie peut contenir plusieurs capacités

### 2.5. Registre des Capacités (Capability Registry)

**Définition formelle :**

Le **Registre des Capacités** est la structure centrale de Master Butler qui contient l'inventaire exhaustif de toutes les capacités du système.

**Caractéristiques formelles :**

- **Exhaustif :** Contient toutes les capacités du système
- **Cohérent :** Aucune duplication, aucune incohérence
- **Traçable :** Historique complet de toutes les modifications
- **Indexé :** Recherche efficace par identifiant, catégorie, source
- **Dynamique :** Évolue avec le système (ajouts, dépréciations)

**Structure formelle :**

```
CapabilityRegistry {
  capabilities: Map<CapabilityId, Capability>,  // Index principal
  by_category: Map<CapabilityCategory, Set<CapabilityId>>,  // Index par catégorie
  by_source: Map<SourceIdentity, Set<CapabilityId>>,  // Index par source
  relations: CapabilityRelationGraph,  // Graphe des relations
  history: CapabilityHistory,  // Historique des modifications
  version: RegistryVersion  // Version du registre
}
```

**Invariants :**
- INV-REG-1 : Le registre contient toutes les capacités déclarées du système
- INV-REG-2 : Aucune capacité n'existe en dehors du registre
- INV-REG-3 : Le registre est cohérent à tout instant (pas d'état intermédiaire)
- INV-REG-4 : Toute modification du registre est historisée
- INV-REG-5 : Le registre est indexé pour une recherche efficace

---

## 3. Métadonnées des Capacités

### 3.1. Métadonnées obligatoires

**Énoncé :**

Toute capacité déclarée doit fournir un ensemble minimal de métadonnées obligatoires.

**Métadonnées obligatoires :**

| Métadonnée | Type | Description |
|------------|------|-------------|
| `id` | CapabilityId | Identifiant unique et immuable |
| `name` | String | Nom lisible humain (max 100 caractères) |
| `description` | String | Description de la capacité (max 1000 caractères) |
| `source` | SourceIdentity | Composant qui déclare la capacité |
| `category` | CapabilityCategory | Catégorie de la capacité |
| `created_at` | Timestamp | Date et heure de création |
| `version` | Version | Version de la capacité |

**Invariants :**
- INV-META-1 : Toute capacité fournit toutes les métadonnées obligatoires
- INV-META-2 : Les métadonnées obligatoires ne peuvent pas être nulles ou vides
- INV-META-3 : Le nom respecte la limite de 100 caractères
- INV-META-4 : La description respecte la limite de 1000 caractères

### 3.2. Métadonnées optionnelles

**Énoncé :**

Les capacités peuvent fournir des métadonnées additionnelles pour enrichir la documentation et la découverte.

**Métadonnées optionnelles :**

| Métadonnée | Type | Description |
|------------|------|-------------|
| `tags` | Set<String> | Tags de classification |
| `documentation_url` | URL | Lien vers la documentation |
| `examples` | List<String> | Exemples d'utilisation |
| `related_permissions` | Set<PermissionId> | Permissions souvent associées |
| `minimum_security_level` | SecurityLevel | Niveau de sécurité minimum |
| `deprecated_at` | Timestamp | Date de dépréciation (si applicable) |
| `deprecation_reason` | String | Raison de la dépréciation |
| `successor` | CapabilityId | Capacité de remplacement (si dépréciée) |
| `custom` | Map<String, Any> | Métadonnées personnalisées |

**Règles :**
- R-META-1 : Les métadonnées optionnelles peuvent être nulles ou absentes
- R-META-2 : Les tags sont normalisés (minuscules, sans accents)
- R-META-3 : L'URL de documentation doit être valide si fournie
- R-META-4 : Les métadonnées personnalisées sont libres mais doivent être sérialisables

### 3.3. Statut de Capacité

**Énoncé :**

Toute capacité possède un statut qui reflète son état dans le cycle de vie.

**Statuts possibles :**

| Statut | Description | Transitions possibles |
|--------|-------------|----------------------|
| **Active** | Capacité disponible et utilisable | → Deprecated, → Removed |
| **Deprecated** | Capacité obsolète, utilisation déconseillée | → Removed |
| **Removed** | Capacité supprimée, non utilisable | (terminal) |

**Règles de transition :**
- R-ST-1 : Une capacité nouvellement créée est toujours Active
- R-ST-2 : Une capacité Active peut être dépréciée (→ Deprecated)
- R-ST-3 : Une capacité Deprecated peut être supprimée (→ Removed)
- R-ST-4 : Une capacité Active peut être supprimée directement (→ Removed)
- R-ST-5 : Une capacité Removed ne peut pas être réactivée
- R-ST-6 : Une transition de statut est irréversible

**Invariants :**
- INV-ST-1 : Toute capacité possède exactement un statut
- INV-ST-2 : Les transitions de statut respectent les règles définies
- INV-ST-3 : Une capacité Removed ne peut jamais être réutilisée

---

## 4. Opérations sur le Registre

### 4.1. Déclaration de Capacité (DeclareCapability)

**Énoncé :**

L'opération **DeclareCapability** permet à un composant de déclarer une nouvelle capacité dans le registre.

**Signature conceptuelle :**

```
DeclareCapability(
  id: CapabilityId,
  name: String,
  description: String,
  source: SourceIdentity,
  category: CapabilityCategory,
  metadata: CapabilityMetadata?
) → Result<Capability, DeclarationError>
```

**Préconditions :**
- PRE-1 : L'identifiant n'existe pas déjà dans le registre
- PRE-2 : L'identifiant respecte le format canonique
- PRE-3 : Les métadonnées obligatoires sont fournies et valides
- PRE-4 : La source est autorisée à déclarer cette capacité

**Postconditions :**
- POST-1 : La capacité est ajoutée au registre avec statut Active
- POST-2 : Les index sont mis à jour (par catégorie, par source)
- POST-3 : L'historique est mis à jour avec l'événement de création
- POST-4 : La version du registre est incrémentée

**Règles d'idempotence :**
- R-IDEMP-1 : Déclarer deux fois la même capacité (même id, même contenu) est idempotent
- R-IDEMP-2 : Déclarer une capacité avec un id existant mais contenu différent est une erreur

**Invariants préservés :**
- INV-REG-1, INV-REG-3, INV-REG-4, INV-REG-5

### 4.2. Interrogation de Capacité (QueryCapability)

**Énoncé :**

L'opération **QueryCapability** permet d'interroger le registre pour obtenir les informations sur une ou plusieurs capacités.

**Modes d'interrogation :**

| Mode | Description | Exemple |
|------|-------------|---------|
| **ById** | Recherche par identifiant exact | `content.create` |
| **ByCategory** | Recherche par catégorie | `Data` |
| **BySource** | Recherche par source | `spm.cms.content` |
| **ByStatus** | Recherche par statut | `Active` |
| **ByTags** | Recherche par tags | `["write", "content"]` |
| **All** | Toutes les capacités | - |

**Signature conceptuelle :**

```
QueryCapability(
  filter: CapabilityFilter
) → Result<List<Capability>, QueryError>

CapabilityFilter {
  id: CapabilityId?,
  category: CapabilityCategory?,
  source: SourceIdentity?,
  status: CapabilityStatus?,
  tags: Set<String>?
}
```

**Préconditions :**
- PRE-1 : Le filtre est valide (au moins un critère ou All)

**Postconditions :**
- POST-1 : Les capacités correspondant au filtre sont retournées
- POST-2 : Le registre n'est pas modifié

**Règles :**
- R-QUERY-1 : L'interrogation est toujours en lecture seule
- R-QUERY-2 : Les filtres peuvent être combinés (AND logique)
- R-QUERY-3 : Une interrogation sans résultat retourne une liste vide

### 4.3. Dépréciation de Capacité (DeprecateCapability)

**Énoncé :**

L'opération **DeprecateCapability** permet de marquer une capacité comme obsolète.

**Signature conceptuelle :**

```
DeprecateCapability(
  id: CapabilityId,
  reason: String,
  successor: CapabilityId?
) → Result<Capability, DeprecationError>
```

**Préconditions :**
- PRE-1 : La capacité existe dans le registre
- PRE-2 : La capacité a le statut Active
- PRE-3 : La raison de dépréciation est fournie
- PRE-4 : Si un successeur est indiqué, il existe et est Active

**Postconditions :**
- POST-1 : La capacité passe au statut Deprecated
- POST-2 : La date de dépréciation est enregistrée
- POST-3 : La raison et le successeur sont enregistrés
- POST-4 : L'historique est mis à jour

**Règles :**
- R-DEP-1 : Une capacité dépréciée reste interrogeable
- R-DEP-2 : Une capacité dépréciée ne peut pas être redéclarée
- R-DEP-3 : La dépréciation est irréversible

### 4.4. Suppression de Capacité (RemoveCapability)

**Énoncé :**

L'opération **RemoveCapability** permet de supprimer définitivement une capacité du registre actif.

**Signature conceptuelle :**

```
RemoveCapability(
  id: CapabilityId,
  reason: String
) → Result<(), RemovalError>
```

**Préconditions :**
- PRE-1 : La capacité existe dans le registre
- PRE-2 : La capacité a le statut Active ou Deprecated
- PRE-3 : Aucune permission active ne référence cette capacité

**Postconditions :**
- POST-1 : La capacité passe au statut Removed
- POST-2 : La capacité n'apparaît plus dans les interrogations standard
- POST-3 : L'historique conserve la trace de la capacité
- POST-4 : L'identifiant est réservé (non réutilisable)

**Règles :**
- R-REM-1 : Une capacité supprimée n'est plus utilisable
- R-REM-2 : L'identifiant reste réservé pour éviter les conflits
- R-REM-3 : L'historique complet est conservé

---

## 5. Relations entre Capacités

### 5.1. Types de Relations

**Énoncé :**

Les capacités peuvent être liées par des relations sémantiques qui définissent leurs interactions et dépendances.

**Types de relations :**

| Type | Description | Exemple |
|------|-------------|---------|
| **Requires** | A nécessite B pour fonctionner | `content.publish` requires `content.read` |
| **Implies** | A implique B (B est automatique si A) | `admin.full` implies `content.manage` |
| **Conflicts** | A et B sont mutuellement exclusives | `content.lock` conflicts `content.edit` |
| **Supersedes** | A remplace B (pour dépréciation) | `content.create.v2` supersedes `content.create` |
| **Groups** | A regroupe B, C, D (composition) | `content.manage` groups `content.create`, `content.edit`, `content.delete` |

**Structure formelle :**

```
CapabilityRelation {
  type: RelationType,
  from: CapabilityId,
  to: CapabilityId,
  metadata: RelationMetadata?
}
```

**Invariants :**
- INV-REL-1 : Les deux capacités d'une relation existent dans le registre
- INV-REL-2 : Une relation Requires ne peut pas créer de cycle
- INV-REL-3 : Une relation Conflicts est symétrique
- INV-REL-4 : Une relation Supersedes implique la dépréciation de la capacité remplacée

### 5.2. Graphe des Relations

**Énoncé :**

Le **Graphe des Relations** est la structure qui modélise toutes les relations entre capacités dans le registre.

**Caractéristiques :**
- Orienté : Les relations ont une direction (from → to)
- Acyclique : Pas de cycles dans les relations Requires
- Complet : Toutes les relations déclarées sont présentes
- Cohérent : Pas de relation vers une capacité inexistante

**Opérations sur le graphe :**

| Opération | Description |
|-----------|-------------|
| `GetDependencies(cap)` | Capacités requises par cap |
| `GetDependents(cap)` | Capacités qui requièrent cap |
| `GetImplied(cap)` | Capacités impliquées par cap |
| `GetConflicts(cap)` | Capacités en conflit avec cap |
| `GetGroup(cap)` | Capacités regroupées par cap |

**Invariants :**
- INV-GRAPH-1 : Le graphe des relations Requires est acyclique
- INV-GRAPH-2 : Toute capacité référencée existe dans le registre
- INV-GRAPH-3 : Le graphe est cohérent avec le statut des capacités

---

## 6. Règles de Déclaration

### 6.1. Qui peut déclarer des Capacités

**Énoncé :**

Seuls certains types de composants sont autorisés à déclarer des capacités dans le registre.

**Sources autorisées :**

| Source | Peut déclarer | Exemples |
|--------|---------------|----------|
| **Module SPM** | Ses propres capacités | `spm.cms.content.create` |
| **Core** | Ses capacités de gouvernance | `core.strongfather.evaluate` |
| **Operator** | Ses capacités spécifiques | `operator.myapp.custom_action` |
| **Tool** | Ses capacités atomiques | `tool.layout.render` |
| **Toolkit** | Aucune (composition uniquement) | - |

**Règles :**
- R-DECL-1 : Un composant ne peut déclarer que ses propres capacités
- R-DECL-2 : Un composant ne peut pas déclarer de capacités au nom d'un autre
- R-DECL-3 : La déclaration est vérifiée par Master Butler
- R-DECL-4 : Un Toolkit ne déclare pas de capacité (il référence des capacités existantes)

### 6.2. Quand déclarer les Capacités

**Énoncé :**

Les capacités doivent être déclarées à des moments spécifiques du cycle de vie du système.

**Moments de déclaration :**

| Moment | Description |
|--------|-------------|
| **Initialisation** | Au démarrage du composant |
| **Mise à jour** | Lors de l'ajout de nouvelles fonctionnalités |
| **Migration** | Lors du remplacement d'une capacité |

**Règles :**
- R-WHEN-1 : Les capacités sont déclarées avant toute utilisation
- R-WHEN-2 : Une capacité non déclarée ne peut pas être utilisée
- R-WHEN-3 : La déclaration peut être répétée (idempotente)
- R-WHEN-4 : La mise à jour des métadonnées est autorisée

### 6.3. Validation des Déclarations

**Énoncé :**

Master Butler valide toutes les déclarations avant de les enregistrer dans le registre.

**Validations effectuées :**

| Validation | Description | Erreur si échec |
|------------|-------------|-----------------|
| Format de l'identifiant | Respect du format canonique | `InvalidCapabilityId` |
| Unicité de l'identifiant | Pas de duplication | `DuplicateCapabilityId` |
| Métadonnées obligatoires | Présence et validité | `MissingMetadata` |
| Autorisation de la source | Source autorisée à déclarer | `UnauthorizedSource` |
| Cohérence des relations | Relations vers capacités existantes | `InvalidRelation` |
| Pas de cycle | Pas de cycle dans les dépendances | `CyclicDependency` |

**Invariants :**
- INV-VAL-1 : Aucune capacité invalide n'est enregistrée
- INV-VAL-2 : Toutes les validations sont effectuées atomiquement
- INV-VAL-3 : Un échec de validation ne modifie pas le registre

---

## 7. Traçabilité et Historique

### 7.1. Historique des Capacités

**Énoncé :**

Toute modification du registre des capacités est enregistrée dans un historique immuable.

**Événements historisés :**

| Événement | Description | Données enregistrées |
|-----------|-------------|---------------------|
| `CapabilityCreated` | Nouvelle capacité déclarée | Capacité complète, source, timestamp |
| `CapabilityUpdated` | Métadonnées modifiées | Champs modifiés, ancienne/nouvelle valeur |
| `CapabilityDeprecated` | Capacité dépréciée | Raison, successeur, timestamp |
| `CapabilityRemoved` | Capacité supprimée | Raison, timestamp |
| `RelationAdded` | Nouvelle relation | Relation complète |
| `RelationRemoved` | Relation supprimée | Relation, raison |

**Structure d'un événement :**

```
CapabilityHistoryEvent {
  event_id: EventId,
  event_type: EventType,
  capability_id: CapabilityId,
  timestamp: Timestamp,
  source: SourceIdentity,
  data: EventData,
  checksum: Checksum
}
```

**Invariants :**
- INV-HIST-1 : L'historique est immuable (append-only)
- INV-HIST-2 : Chaque événement possède un identifiant unique
- INV-HIST-3 : L'ordre des événements est préservé
- INV-HIST-4 : Chaque événement est signé (checksum)

### 7.2. Audit du Registre

**Énoncé :**

Le registre peut être audité à tout moment pour vérifier sa cohérence et retracer l'évolution des capacités.

**Opérations d'audit :**

| Opération | Description |
|-----------|-------------|
| `GetHistory(cap)` | Historique complet d'une capacité |
| `GetStateAt(timestamp)` | État du registre à un instant donné |
| `VerifyIntegrity()` | Vérification de la cohérence |
| `GetStatistics()` | Statistiques du registre |

**Informations d'audit disponibles :**
- Nombre total de capacités (par statut)
- Historique de chaque capacité
- Sources les plus actives
- Relations les plus utilisées
- Dépréciations récentes

---

## 8. Invariants Non Négociables

### 8.1. Exhaustivité

**Invariant INV-NN-1 :**

> **Toute capacité existant dans le système est recensée dans le registre de Master Butler.**

**Implications :**
- Aucune capacité "cachée" ou non déclarée
- Aucun contournement du registre
- Master Butler est la source de vérité unique

### 8.2. Unicité des Identifiants

**Invariant INV-NN-2 :**

> **Chaque capacité possède un identifiant unique et immuable, jamais réutilisable.**

**Implications :**
- Pas de collision d'identifiants
- Pas de réutilisation après suppression
- Traçabilité parfaite

### 8.3. Idempotence des Déclarations

**Invariant INV-NN-3 :**

> **Les déclarations de capacités sont idempotentes. Déclarer deux fois la même capacité n'a pas d'effet supplémentaire.**

**Implications :**
- Redéclaration au démarrage autorisée
- Pas d'effet de bord sur les redéclarations
- Cohérence garantie

### 8.4. Traçabilité Complète

**Invariant INV-NN-4 :**

> **Toute modification du registre est tracée avec contexte complet (qui, quand, quoi).**

**Implications :**
- Audit possible à tout moment
- Historique immuable
- Responsabilité identifiable

### 8.5. Non-Décision

**Invariant INV-NN-5 :**

> **Le registre recense les capacités mais ne décide jamais de leur utilisation.**

**Implications :**
- Master Butler informe, ne décide pas
- La décision appartient à StrongFather
- Séparation stricte connaissance/décision

---

## 9. Schémas ASCII

### 9.1. Structure du Registre

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        CAPABILITY REGISTRY                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  INDEX PRINCIPAL (Map<CapabilityId, Capability>)                      │  │
│  ├──────────────────────────────────────────────────────────────────────┤  │
│  │  content.create      → Capability { name: "Créer contenu", ... }     │  │
│  │  content.edit        → Capability { name: "Modifier contenu", ... }  │  │
│  │  content.delete      → Capability { name: "Supprimer contenu", ... } │  │
│  │  media.upload        → Capability { name: "Téléverser média", ... }  │  │
│  │  hierarchy.reorder   → Capability { name: "Réordonner", ... }        │  │
│  │  search.index        → Capability { name: "Indexer", ... }           │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  ┌────────────────────────┐    ┌────────────────────────────────────────┐  │
│  │  INDEX PAR CATÉGORIE   │    │  INDEX PAR SOURCE                       │  │
│  ├────────────────────────┤    ├────────────────────────────────────────┤  │
│  │  Data:                 │    │  spm.cms.content:                       │  │
│  │    - content.create    │    │    - content.create                     │  │
│  │    - content.edit      │    │    - content.edit                       │  │
│  │    - content.delete    │    │    - content.delete                     │  │
│  │                        │    │                                          │  │
│  │  Media:                │    │  spm.cms.media:                          │  │
│  │    - media.upload      │    │    - media.upload                        │  │
│  │                        │    │                                          │  │
│  │  Hierarchy:            │    │  spm.cms.hierarchy:                      │  │
│  │    - hierarchy.reorder │    │    - hierarchy.reorder                   │  │
│  └────────────────────────┘    └────────────────────────────────────────┘  │
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  GRAPHE DES RELATIONS                                                 │  │
│  ├──────────────────────────────────────────────────────────────────────┤  │
│  │                                                                        │  │
│  │  content.publish ──[Requires]──► content.read                         │  │
│  │  content.manage  ──[Groups]────► content.create                       │  │
│  │                  ──[Groups]────► content.edit                         │  │
│  │                  ──[Groups]────► content.delete                       │  │
│  │  content.lock    ──[Conflicts]─► content.edit                         │  │
│  │                                                                        │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 9.2. Flux de Déclaration

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     FLUX DE DÉCLARATION DE CAPACITÉ                          │
└─────────────────────────────────────────────────────────────────────────────┘

   MODULE SPM                    MASTER BUTLER                    REGISTRE
       │                              │                              │
       │  DeclareCapability(          │                              │
       │    id: "content.create",     │                              │
       │    name: "Créer contenu",    │                              │
       │    source: "spm.cms",        │                              │
       │    category: Data            │                              │
       │  )                           │                              │
       ├─────────────────────────────►│                              │
       │                              │                              │
       │                              │  1. Valider format id        │
       │                              │  2. Vérifier unicité         │
       │                              │  3. Valider métadonnées      │
       │                              │  4. Vérifier autorisation    │
       │                              │                              │
       │                              │  [Validations OK]            │
       │                              │                              │
       │                              │  5. Créer Capability         │
       │                              ├─────────────────────────────►│
       │                              │                              │
       │                              │  6. Mettre à jour index      │
       │                              ├─────────────────────────────►│
       │                              │                              │
       │                              │  7. Historiser événement     │
       │                              ├─────────────────────────────►│
       │                              │                              │
       │                              │◄─────────────────────────────┤
       │                              │      [Capability créée]      │
       │◄─────────────────────────────┤                              │
       │    Result::Ok(Capability)    │                              │
       │                              │                              │
       ▼                              ▼                              ▼
```

### 9.3. Cycle de Vie d'une Capacité

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     CYCLE DE VIE D'UNE CAPACITÉ                              │
└─────────────────────────────────────────────────────────────────────────────┘

                              DeclareCapability()
                                     │
                                     ▼
                        ┌────────────────────────┐
                        │                        │
                        │       ACTIVE           │
                        │                        │
                        │   ✓ Utilisable         │
                        │   ✓ Interrogeable      │
                        │   ✓ Référençable       │
                        │                        │
                        └────────────────────────┘
                                │          │
                                │          │
          DeprecateCapability() │          │ RemoveCapability()
                                │          │
                                ▼          │
                        ┌────────────────────────┐
                        │                        │
                        │      DEPRECATED        │
                        │                        │
                        │   ✓ Interrogeable      │
                        │   ⚠ Utilisation        │
                        │     déconseillée       │
                        │   ✓ Successeur indiqué │
                        │                        │
                        └────────────────────────┘
                                     │
                                     │ RemoveCapability()
                                     │
                                     ▼
                        ┌────────────────────────┐
                        │                        │
                        │       REMOVED          │
                        │                        │
                        │   ✗ Non utilisable     │
                        │   ✗ Non interrogeable  │
                        │   ✓ Historique conservé│
                        │   ✗ Id non réutilisable│
                        │                        │
                        └────────────────────────┘
                                     │
                                     │ (TERMINAL)
                                     ▼

                    ⚠️ TRANSITIONS IRRÉVERSIBLES ⚠️
```

---

## 10. Exemples Concrets

### 10.1. Exemple : Déclaration de Capacités CMS

**Contexte :**
Le module SPM CMS Content déclare ses capacités au démarrage.

**Déclarations :**

```
// Capacité de création de contenu
DeclareCapability(
  id: "content.create",
  name: "Créer du contenu",
  description: "Capacité de créer un nouveau contenu dans le système CMS",
  source: SourceIdentity {
    type: Module,
    identifier: "spm.cms.content",
    version: "1.0.0"
  },
  category: Data,
  metadata: {
    tags: ["write", "content", "cms"],
    minimum_security_level: 2
  }
)

// Capacité de lecture de contenu
DeclareCapability(
  id: "content.read",
  name: "Lire du contenu",
  description: "Capacité de lire le contenu existant",
  source: SourceIdentity {
    type: Module,
    identifier: "spm.cms.content",
    version: "1.0.0"
  },
  category: Data,
  metadata: {
    tags: ["read", "content", "cms"]
  }
)

// Capacité de publication avec dépendance
DeclareCapability(
  id: "content.publish",
  name: "Publier du contenu",
  description: "Capacité de publier un contenu existant",
  source: SourceIdentity {
    type: Module,
    identifier: "spm.cms.content",
    version: "1.0.0"
  },
  category: Data,
  metadata: {
    tags: ["write", "content", "cms", "publish"]
  },
  relations: [
    { type: Requires, to: "content.read" }
  ]
)
```

### 10.2. Exemple : Interrogation du Registre

**Contexte :**
StrongFather interroge Master Butler pour évaluer une intention.

**Interrogation par identifiant :**

```
QueryCapability(
  filter: { id: "content.create" }
)
→ Result::Ok([
    Capability {
      id: "content.create",
      name: "Créer du contenu",
      status: Active,
      ...
    }
  ])
```

**Interrogation par catégorie :**

```
QueryCapability(
  filter: { category: Data, status: Active }
)
→ Result::Ok([
    Capability { id: "content.create", ... },
    Capability { id: "content.read", ... },
    Capability { id: "content.edit", ... },
    Capability { id: "content.delete", ... },
    Capability { id: "content.publish", ... }
  ])
```

### 10.3. Exemple : Dépréciation et Migration

**Contexte :**
Une capacité est remplacée par une nouvelle version.

**Dépréciation :**

```
// Créer la nouvelle version
DeclareCapability(
  id: "content.create.v2",
  name: "Créer du contenu (v2)",
  description: "Nouvelle version avec support multi-langue",
  ...
)

// Déprécier l'ancienne version
DeprecateCapability(
  id: "content.create",
  reason: "Remplacée par content.create.v2 avec support multi-langue",
  successor: "content.create.v2"
)
```

**État après dépréciation :**

```
Capability {
  id: "content.create",
  status: Deprecated,
  deprecated_at: "2026-01-27T15:30:00Z",
  deprecation_reason: "Remplacée par content.create.v2",
  successor: "content.create.v2"
}
```

---

## 11. Conclusion

Ce contrat établit le modèle technique du registre des capacités de Master Butler, définissant de manière absolue :

**Points clés :**
- **Capacité :** Pouvoir technique intrinsèque, identifié de manière unique et immuable
- **CapabilityId :** Format canonique `<domain>.<module>.<action>[.<qualifier>]`
- **Registre :** Structure exhaustive, cohérente, tracée, et indexée
- **Métadonnées :** Obligatoires (id, name, description, source, category) et optionnelles
- **Statuts :** Active → Deprecated → Removed (transitions irréversibles)
- **Relations :** Requires, Implies, Conflicts, Supersedes, Groups
- **Opérations :** Declare, Query, Deprecate, Remove (toutes validées)
- **Traçabilité :** Historique immuable de toutes les modifications

**Invariants non négociables :**
- Exhaustivité du registre
- Unicité et immutabilité des identifiants
- Idempotence des déclarations
- Traçabilité complète
- Non-décision (Master Butler informe, ne décide pas)

Ce contrat complète la Documentation Fondatrice de Master Butler en définissant le modèle technique du registre des capacités. Il s'articule avec le Permission Registry Contract pour former le système complet de catalogage des capacités et permissions.

**Non-négociabilité :** Ce contrat est absolu et non négociable. Le contrat prime sur toute considération pratique.

---

**Document créé le :** 2026-01-27  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, Master Butler Documentation Fondatrice  
**Type :** Contrat de registre non négociable

---

## 12. Mini log — erreurs / warnings / ambiguïtés rencontrées et corrigées

### Ambiguïté A1 : Format de l'identifiant de capacité

**Ambiguïté rencontrée :**
La documentation fondatrice donne des exemples d'identifiants (`content.create`, `hierarchy.reorder`) sans définir un format canonique strict.

**Décision prise :**
Format canonique défini : `<domain>.<module>.<action>[.<qualifier>]` avec règles de nommage strictes (minuscules, sans accents, segments séparés par points).

**Justification :**
Un format strict garantit la cohérence, évite les collisions, et facilite l'indexation et la recherche.

**Correction effectuée :**
Section 2.2 "Identifiant de Capacité" ajoutée avec format canonique et règles de nommage (R-ID-1 à R-ID-6).

### Ambiguïté A2 : Cycle de vie des capacités

**Ambiguïté rencontrée :**
La documentation fondatrice mentionne l'historique des capacités (ajouts, suppressions, modifications) sans définir un cycle de vie formel.

**Décision prise :**
Cycle de vie à trois états (Active, Deprecated, Removed) avec transitions irréversibles et règles explicites.

**Justification :**
Un cycle de vie formel garantit la cohérence temporelle et permet une gestion propre des migrations et dépréciations.

**Correction effectuée :**
Section 3.3 "Statut de Capacité" ajoutée avec états, transitions, et règles (R-ST-1 à R-ST-6).

### Ambiguïté A3 : Relations entre capacités

**Ambiguïté rencontrée :**
La documentation fondatrice mentionne des "relations entre capacités (dépendances, hiérarchies)" sans les définir formellement.

**Décision prise :**
Cinq types de relations définis (Requires, Implies, Conflicts, Supersedes, Groups) avec sémantique et invariants.

**Justification :**
Des types de relations formels permettent de modéliser toutes les interactions entre capacités de manière cohérente.

**Correction effectuée :**
Section 5 "Relations entre Capacités" ajoutée avec types, graphe, et invariants (INV-REL-1 à INV-REL-4, INV-GRAPH-1 à INV-GRAPH-3).

### Ambiguïté A4 : Idempotence des déclarations

**Ambiguïté rencontrée :**
La documentation fondatrice mentionne que les déclarations sont idempotentes sans préciser le comportement exact en cas de redéclaration avec contenu différent.

**Décision prise :**
Idempotence stricte : même id + même contenu = ok, même id + contenu différent = erreur.

**Justification :**
Cette règle évite les incohérences tout en permettant la redéclaration au démarrage.

**Correction effectuée :**
Section 4.1 "Déclaration de Capacité" avec règles d'idempotence (R-IDEMP-1 à R-IDEMP-2).

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
