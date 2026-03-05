# Master Butler â€” Capability Registry Contract

## 1. Introduction

### Objet du contrat

Ce document dÃ©finit le **Master Butler Capability Registry Contract** : un contrat normatif, non nÃ©gociable, et de statut FONDATION qui Ã©tablit le modÃ¨le du registre des capacitÃ©s dans le systÃ¨me Miyukini Core System v2.4.

Ce contrat dÃ©finit :
- La structure formelle du registre des capacitÃ©s
- Les rÃ¨gles de dÃ©claration et d'enregistrement
- Les mÃ©tadonnÃ©es obligatoires et optionnelles
- Les invariants du registre
- Les opÃ©rations autorisÃ©es sur le registre
- Les relations entre capacitÃ©s

### PortÃ©e

Ce contrat s'applique Ã  **toute instance de Master Butler** et dÃ©finit de maniÃ¨re absolue :
- La dÃ©finition formelle d'une CapacitÃ© (Capability)
- La structure du Registre des CapacitÃ©s (Capability Registry)
- Les rÃ¨gles de dÃ©claration (Declaration Rules)
- Les mÃ©tadonnÃ©es des capacitÃ©s (Capability Metadata)
- Les relations entre capacitÃ©s (Capability Relations)
- Les invariants du registre
- Les opÃ©rations autorisÃ©es et interdites

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il Ã©tablit des rÃ¨gles absolues que Master Butler applique sans exception. Ces rÃ¨gles ne peuvent Ãªtre contournÃ©es, nÃ©gociÃ©es, ou modifiÃ©es. Le contrat prime sur toute considÃ©ration pratique.

### Relation avec les autres contrats

Ce contrat complÃ¨te et s'articule avec les documents contractuels existants :

- **[Master Butler - Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md)** : DÃ©finit la nature, le rÃ´le, et les responsabilitÃ©s de Master Butler
- **Master Butler - Permission Registry Contract** : DÃ©finit le registre des permissions (contrat complÃ©mentaire)
- **[Miyukini Conceptual References - Tools et Toolkits](..//..//..//..//miyukini-webway-system//reference//_index.md)** : DÃ©finit les concepts de Tool et Toolkit
- **[Miyukini Framework - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)** : Ce contrat respecte **LOI-1** (aucune dÃ©pendance externe critique) et **LOI-5** (coÃ»t proportionnel au hardware) en garantissant que le registre est local, lÃ©ger, et autonome.

**ComplÃ©mentaritÃ© :**
- Master Butler Documentation Fondatrice = dÃ©finition conceptuelle et philosophique
- Master Butler Capability Registry Contract = modÃ¨le technique du registre des capacitÃ©s
- Master Butler Permission Registry Contract = modÃ¨le technique du registre des permissions

Ces contrats forment ensemble le systÃ¨me complet de catalogage des capacitÃ©s et permissions du systÃ¨me Miyukini Core System v2.4.

---

## 2. DÃ©finitions formelles

### 2.1. CapacitÃ© (Capability)

**DÃ©finition formelle :**

Une **CapacitÃ©** est un pouvoir technique intrinsÃ¨que Ã  un composant du systÃ¨me Miyukini. Elle reprÃ©sente ce qu'un composant peut faire fonctionnellement, indÃ©pendamment de toute permission ou dÃ©cision.

**CaractÃ©ristiques formelles :**

- **IdentitÃ© unique :** Chaque capacitÃ© possÃ¨de un identifiant unique et immuable (CapabilityId)
- **IntrinsÃ¨que :** La capacitÃ© est intrinsÃ¨que au composant qui la possÃ¨de
- **Technique :** La capacitÃ© dÃ©crit un pouvoir fonctionnel (pas mÃ©tier)
- **DÃ©clarative :** La capacitÃ© est dÃ©clarÃ©e par le composant qui la possÃ¨de
- **DocumentÃ©e :** La capacitÃ© possÃ¨de des mÃ©tadonnÃ©es descriptives
- **GouvernÃ©e :** La capacitÃ© est soumise Ã  la gouvernance des Cores

**Structure formelle d'une CapacitÃ© :**

```
Capability {
  id: CapabilityId,           // Identifiant unique et immuable
  name: String,               // Nom lisible humain
  description: String,        // Description de la capacitÃ©
  source: SourceIdentity,     // Module/composant qui dÃ©clare la capacitÃ©
  category: CapabilityCategory, // CatÃ©gorie de la capacitÃ©
  metadata: CapabilityMetadata, // MÃ©tadonnÃ©es additionnelles
  relations: CapabilityRelations, // Relations avec d'autres capacitÃ©s
  created_at: Timestamp,      // Date de crÃ©ation
  version: Version,           // Version de la capacitÃ©
  status: CapabilityStatus    // Statut (Active, Deprecated, Removed)
}
```

**Invariants :**
- INV-CAP-1 : Toute capacitÃ© possÃ¨de un CapabilityId unique et immuable
- INV-CAP-2 : Toute capacitÃ© est dÃ©clarÃ©e par exactement un composant source
- INV-CAP-3 : Toute capacitÃ© possÃ¨de un nom, une description, et une catÃ©gorie
- INV-CAP-4 : L'identifiant d'une capacitÃ© ne peut jamais Ãªtre modifiÃ©
- INV-CAP-5 : Une capacitÃ© supprimÃ©e ne peut jamais Ãªtre recrÃ©Ã©e avec le mÃªme identifiant

### 2.2. Identifiant de CapacitÃ© (CapabilityId)

**DÃ©finition formelle :**

Un **CapabilityId** est l'identifiant unique et immuable d'une capacitÃ© dans le registre. Il suit un format canonique qui encode le domaine, le module, et l'action.

**Format canonique :**

```
<domain>.<module>.<action>[.<qualifier>]
```

**Exemples :**
- `content.create` : CapacitÃ© de crÃ©er du contenu
- `content.edit.own` : CapacitÃ© de modifier son propre contenu
- `hierarchy.reorder` : CapacitÃ© de rÃ©organiser une hiÃ©rarchie
- `media.upload` : CapacitÃ© de tÃ©lÃ©verser des mÃ©dias
- `search.index` : CapacitÃ© d'indexer pour la recherche
- `auth.login` : CapacitÃ© de s'authentifier

**RÃ¨gles de nommage :**
- R-ID-1 : Le format est strictement `<domain>.<module>.<action>[.<qualifier>]`
- R-ID-2 : Tous les segments sont en minuscules, sans accents
- R-ID-3 : Les segments sont sÃ©parÃ©s par des points (.)
- R-ID-4 : Chaque segment contient uniquement des lettres, chiffres, et underscores
- R-ID-5 : Le qualifieur est optionnel et ajoute une prÃ©cision sÃ©mantique
- R-ID-6 : L'identifiant est unique dans l'ensemble du systÃ¨me

**Invariants :**
- INV-ID-1 : Tout CapabilityId respecte le format canonique
- INV-ID-2 : Tout CapabilityId est unique dans le registre
- INV-ID-3 : Un CapabilityId ne peut jamais Ãªtre modifiÃ© aprÃ¨s crÃ©ation
- INV-ID-4 : Deux capacitÃ©s diffÃ©rentes ne peuvent jamais avoir le mÃªme CapabilityId

### 2.3. Source de CapacitÃ© (SourceIdentity)

**DÃ©finition formelle :**

Une **SourceIdentity** identifie le composant qui dÃ©clare une capacitÃ©. Elle permet de tracer l'origine de chaque capacitÃ© dans le registre.

**Types de sources :**

| Type | Description | Exemple |
|------|-------------|---------|
| **Module SPM** | Module du Standard Product Model | `spm.cms.content` |
| **Core** | Core du Miyukini Core System | `core.kindmother` |
| **Operator** | OpÃ©rateur (application) | `operator.my_app` |
| **Tool** | Outil du systÃ¨me | `tool.layout.render` |

**Structure formelle :**

```
SourceIdentity {
  type: SourceType,           // Module, Core, Operator, Tool
  identifier: String,         // Identifiant unique de la source
  version: Version,           // Version de la source
  environment: EnvironmentId  // Environnement de dÃ©claration
}
```

**Invariants :**
- INV-SRC-1 : Toute capacitÃ© possÃ¨de une SourceIdentity valide
- INV-SRC-2 : Une SourceIdentity identifie de maniÃ¨re unique le composant source
- INV-SRC-3 : La version de la source est incluse pour traÃ§abilitÃ©

### 2.4. CatÃ©gorie de CapacitÃ© (CapabilityCategory)

**DÃ©finition formelle :**

Une **CapabilityCategory** classifie les capacitÃ©s par domaine fonctionnel pour faciliter la dÃ©couverte et l'organisation.

**CatÃ©gories standard :**

| CatÃ©gorie | Description | Exemples |
|-----------|-------------|----------|
| **Data** | CapacitÃ©s liÃ©es aux donnÃ©es | `content.create`, `content.read` |
| **Hierarchy** | CapacitÃ©s liÃ©es aux hiÃ©rarchies | `hierarchy.reorder`, `hierarchy.create` |
| **Media** | CapacitÃ©s liÃ©es aux mÃ©dias | `media.upload`, `media.delete` |
| **Search** | CapacitÃ©s liÃ©es Ã  la recherche | `search.index`, `search.query` |
| **Auth** | CapacitÃ©s liÃ©es Ã  l'authentification | `auth.login`, `auth.logout` |
| **Admin** | CapacitÃ©s d'administration | `admin.config`, `admin.audit` |
| **UI** | CapacitÃ©s d'interface utilisateur | `ui.render`, `ui.theme` |
| **IO** | CapacitÃ©s d'entrÃ©e/sortie | `io.file.read`, `io.file.write` |
| **System** | CapacitÃ©s systÃ¨me | `system.health`, `system.metrics` |

**Invariants :**
- INV-CAT-1 : Toute capacitÃ© appartient Ã  exactement une catÃ©gorie
- INV-CAT-2 : Les catÃ©gories sont prÃ©dÃ©finies et extensibles par l'environnement
- INV-CAT-3 : Une catÃ©gorie peut contenir plusieurs capacitÃ©s

### 2.5. Registre des CapacitÃ©s (Capability Registry)

**DÃ©finition formelle :**

Le **Registre des CapacitÃ©s** est la structure centrale de Master Butler qui contient l'inventaire exhaustif de toutes les capacitÃ©s du systÃ¨me.

**CaractÃ©ristiques formelles :**

- **Exhaustif :** Contient toutes les capacitÃ©s du systÃ¨me
- **CohÃ©rent :** Aucune duplication, aucune incohÃ©rence
- **TraÃ§able :** Historique complet de toutes les modifications
- **IndexÃ© :** Recherche efficace par identifiant, catÃ©gorie, source
- **Dynamique :** Ã‰volue avec le systÃ¨me (ajouts, dÃ©prÃ©ciations)

**Structure formelle :**

```
CapabilityRegistry {
  capabilities: Map<CapabilityId, Capability>,  // Index principal
  by_category: Map<CapabilityCategory, Set<CapabilityId>>,  // Index par catÃ©gorie
  by_source: Map<SourceIdentity, Set<CapabilityId>>,  // Index par source
  relations: CapabilityRelationGraph,  // Graphe des relations
  history: CapabilityHistory,  // Historique des modifications
  version: RegistryVersion  // Version du registre
}
```

**Invariants :**
- INV-REG-1 : Le registre contient toutes les capacitÃ©s dÃ©clarÃ©es du systÃ¨me
- INV-REG-2 : Aucune capacitÃ© n'existe en dehors du registre
- INV-REG-3 : Le registre est cohÃ©rent Ã  tout instant (pas d'Ã©tat intermÃ©diaire)
- INV-REG-4 : Toute modification du registre est historisÃ©e
- INV-REG-5 : Le registre est indexÃ© pour une recherche efficace

---

## 3. MÃ©tadonnÃ©es des CapacitÃ©s

### 3.1. MÃ©tadonnÃ©es obligatoires

**Ã‰noncÃ© :**

Toute capacitÃ© dÃ©clarÃ©e doit fournir un ensemble minimal de mÃ©tadonnÃ©es obligatoires.

**MÃ©tadonnÃ©es obligatoires :**

| MÃ©tadonnÃ©e | Type | Description |
|------------|------|-------------|
| `id` | CapabilityId | Identifiant unique et immuable |
| `name` | String | Nom lisible humain (max 100 caractÃ¨res) |
| `description` | String | Description de la capacitÃ© (max 1000 caractÃ¨res) |
| `source` | SourceIdentity | Composant qui dÃ©clare la capacitÃ© |
| `category` | CapabilityCategory | CatÃ©gorie de la capacitÃ© |
| `created_at` | Timestamp | Date et heure de crÃ©ation |
| `version` | Version | Version de la capacitÃ© |

**Invariants :**
- INV-META-1 : Toute capacitÃ© fournit toutes les mÃ©tadonnÃ©es obligatoires
- INV-META-2 : Les mÃ©tadonnÃ©es obligatoires ne peuvent pas Ãªtre nulles ou vides
- INV-META-3 : Le nom respecte la limite de 100 caractÃ¨res
- INV-META-4 : La description respecte la limite de 1000 caractÃ¨res

### 3.2. MÃ©tadonnÃ©es optionnelles

**Ã‰noncÃ© :**

Les capacitÃ©s peuvent fournir des mÃ©tadonnÃ©es additionnelles pour enrichir la documentation et la dÃ©couverte.

**MÃ©tadonnÃ©es optionnelles :**

| MÃ©tadonnÃ©e | Type | Description |
|------------|------|-------------|
| `tags` | Set<String> | Tags de classification |
| `documentation_url` | URL | Lien vers la documentation |
| `examples` | List<String> | Exemples d'utilisation |
| `related_permissions` | Set<PermissionId> | Permissions souvent associÃ©es |
| `minimum_security_level` | SecurityLevel | Niveau de sÃ©curitÃ© minimum |
| `deprecated_at` | Timestamp | Date de dÃ©prÃ©ciation (si applicable) |
| `deprecation_reason` | String | Raison de la dÃ©prÃ©ciation |
| `successor` | CapabilityId | CapacitÃ© de remplacement (si dÃ©prÃ©ciÃ©e) |
| `custom` | Map<String, Any> | MÃ©tadonnÃ©es personnalisÃ©es |

**RÃ¨gles :**
- R-META-1 : Les mÃ©tadonnÃ©es optionnelles peuvent Ãªtre nulles ou absentes
- R-META-2 : Les tags sont normalisÃ©s (minuscules, sans accents)
- R-META-3 : L'URL de documentation doit Ãªtre valide si fournie
- R-META-4 : Les mÃ©tadonnÃ©es personnalisÃ©es sont libres mais doivent Ãªtre sÃ©rialisables

### 3.3. Statut de CapacitÃ©

**Ã‰noncÃ© :**

Toute capacitÃ© possÃ¨de un statut qui reflÃ¨te son Ã©tat dans le cycle de vie.

**Statuts possibles :**

| Statut | Description | Transitions possibles |
|--------|-------------|----------------------|
| **Active** | CapacitÃ© disponible et utilisable | â†’ Deprecated, â†’ Removed |
| **Deprecated** | CapacitÃ© obsolÃ¨te, utilisation dÃ©conseillÃ©e | â†’ Removed |
| **Removed** | CapacitÃ© supprimÃ©e, non utilisable | (terminal) |

**RÃ¨gles de transition :**
- R-ST-1 : Une capacitÃ© nouvellement crÃ©Ã©e est toujours Active
- R-ST-2 : Une capacitÃ© Active peut Ãªtre dÃ©prÃ©ciÃ©e (â†’ Deprecated)
- R-ST-3 : Une capacitÃ© Deprecated peut Ãªtre supprimÃ©e (â†’ Removed)
- R-ST-4 : Une capacitÃ© Active peut Ãªtre supprimÃ©e directement (â†’ Removed)
- R-ST-5 : Une capacitÃ© Removed ne peut pas Ãªtre rÃ©activÃ©e
- R-ST-6 : Une transition de statut est irrÃ©versible

**Invariants :**
- INV-ST-1 : Toute capacitÃ© possÃ¨de exactement un statut
- INV-ST-2 : Les transitions de statut respectent les rÃ¨gles dÃ©finies
- INV-ST-3 : Une capacitÃ© Removed ne peut jamais Ãªtre rÃ©utilisÃ©e

---

## 4. OpÃ©rations sur le Registre

### 4.1. DÃ©claration de CapacitÃ© (DeclareCapability)

**Ã‰noncÃ© :**

L'opÃ©ration **DeclareCapability** permet Ã  un composant de dÃ©clarer une nouvelle capacitÃ© dans le registre.

**Signature conceptuelle :**

```
DeclareCapability(
  id: CapabilityId,
  name: String,
  description: String,
  source: SourceIdentity,
  category: CapabilityCategory,
  metadata: CapabilityMetadata?
) â†’ Result<Capability, DeclarationError>
```

**PrÃ©conditions :**
- PRE-1 : L'identifiant n'existe pas dÃ©jÃ  dans le registre
- PRE-2 : L'identifiant respecte le format canonique
- PRE-3 : Les mÃ©tadonnÃ©es obligatoires sont fournies et valides
- PRE-4 : La source est autorisÃ©e Ã  dÃ©clarer cette capacitÃ©

**Postconditions :**
- POST-1 : La capacitÃ© est ajoutÃ©e au registre avec statut Active
- POST-2 : Les index sont mis Ã  jour (par catÃ©gorie, par source)
- POST-3 : L'historique est mis Ã  jour avec l'Ã©vÃ©nement de crÃ©ation
- POST-4 : La version du registre est incrÃ©mentÃ©e

**RÃ¨gles d'idempotence :**
- R-IDEMP-1 : DÃ©clarer deux fois la mÃªme capacitÃ© (mÃªme id, mÃªme contenu) est idempotent
- R-IDEMP-2 : DÃ©clarer une capacitÃ© avec un id existant mais contenu diffÃ©rent est une erreur

**Invariants prÃ©servÃ©s :**
- INV-REG-1, INV-REG-3, INV-REG-4, INV-REG-5

### 4.2. Interrogation de CapacitÃ© (QueryCapability)

**Ã‰noncÃ© :**

L'opÃ©ration **QueryCapability** permet d'interroger le registre pour obtenir les informations sur une ou plusieurs capacitÃ©s.

**Modes d'interrogation :**

| Mode | Description | Exemple |
|------|-------------|---------|
| **ById** | Recherche par identifiant exact | `content.create` |
| **ByCategory** | Recherche par catÃ©gorie | `Data` |
| **BySource** | Recherche par source | `spm.cms.content` |
| **ByStatus** | Recherche par statut | `Active` |
| **ByTags** | Recherche par tags | `["write", "content"]` |
| **All** | Toutes les capacitÃ©s | - |

**Signature conceptuelle :**

```
QueryCapability(
  filter: CapabilityFilter
) â†’ Result<List<Capability>, QueryError>

CapabilityFilter {
  id: CapabilityId?,
  category: CapabilityCategory?,
  source: SourceIdentity?,
  status: CapabilityStatus?,
  tags: Set<String>?
}
```

**PrÃ©conditions :**
- PRE-1 : Le filtre est valide (au moins un critÃ¨re ou All)

**Postconditions :**
- POST-1 : Les capacitÃ©s correspondant au filtre sont retournÃ©es
- POST-2 : Le registre n'est pas modifiÃ©

**RÃ¨gles :**
- R-QUERY-1 : L'interrogation est toujours en lecture seule
- R-QUERY-2 : Les filtres peuvent Ãªtre combinÃ©s (AND logique)
- R-QUERY-3 : Une interrogation sans rÃ©sultat retourne une liste vide

### 4.3. DÃ©prÃ©ciation de CapacitÃ© (DeprecateCapability)

**Ã‰noncÃ© :**

L'opÃ©ration **DeprecateCapability** permet de marquer une capacitÃ© comme obsolÃ¨te.

**Signature conceptuelle :**

```
DeprecateCapability(
  id: CapabilityId,
  reason: String,
  successor: CapabilityId?
) â†’ Result<Capability, DeprecationError>
```

**PrÃ©conditions :**
- PRE-1 : La capacitÃ© existe dans le registre
- PRE-2 : La capacitÃ© a le statut Active
- PRE-3 : La raison de dÃ©prÃ©ciation est fournie
- PRE-4 : Si un successeur est indiquÃ©, il existe et est Active

**Postconditions :**
- POST-1 : La capacitÃ© passe au statut Deprecated
- POST-2 : La date de dÃ©prÃ©ciation est enregistrÃ©e
- POST-3 : La raison et le successeur sont enregistrÃ©s
- POST-4 : L'historique est mis Ã  jour

**RÃ¨gles :**
- R-DEP-1 : Une capacitÃ© dÃ©prÃ©ciÃ©e reste interrogeable
- R-DEP-2 : Une capacitÃ© dÃ©prÃ©ciÃ©e ne peut pas Ãªtre redÃ©clarÃ©e
- R-DEP-3 : La dÃ©prÃ©ciation est irrÃ©versible

### 4.4. Suppression de CapacitÃ© (RemoveCapability)

**Ã‰noncÃ© :**

L'opÃ©ration **RemoveCapability** permet de supprimer dÃ©finitivement une capacitÃ© du registre actif.

**Signature conceptuelle :**

```
RemoveCapability(
  id: CapabilityId,
  reason: String
) â†’ Result<(), RemovalError>
```

**PrÃ©conditions :**
- PRE-1 : La capacitÃ© existe dans le registre
- PRE-2 : La capacitÃ© a le statut Active ou Deprecated
- PRE-3 : Aucune permission active ne rÃ©fÃ©rence cette capacitÃ©

**Postconditions :**
- POST-1 : La capacitÃ© passe au statut Removed
- POST-2 : La capacitÃ© n'apparaÃ®t plus dans les interrogations standard
- POST-3 : L'historique conserve la trace de la capacitÃ©
- POST-4 : L'identifiant est rÃ©servÃ© (non rÃ©utilisable)

**RÃ¨gles :**
- R-REM-1 : Une capacitÃ© supprimÃ©e n'est plus utilisable
- R-REM-2 : L'identifiant reste rÃ©servÃ© pour Ã©viter les conflits
- R-REM-3 : L'historique complet est conservÃ©

---

## 5. Relations entre CapacitÃ©s

### 5.1. Types de Relations

**Ã‰noncÃ© :**

Les capacitÃ©s peuvent Ãªtre liÃ©es par des relations sÃ©mantiques qui dÃ©finissent leurs interactions et dÃ©pendances.

**Types de relations :**

| Type | Description | Exemple |
|------|-------------|---------|
| **Requires** | A nÃ©cessite B pour fonctionner | `content.publish` requires `content.read` |
| **Implies** | A implique B (B est automatique si A) | `admin.full` implies `content.manage` |
| **Conflicts** | A et B sont mutuellement exclusives | `content.lock` conflicts `content.edit` |
| **Supersedes** | A remplace B (pour dÃ©prÃ©ciation) | `content.create.v2` supersedes `content.create` |
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
- INV-REL-1 : Les deux capacitÃ©s d'une relation existent dans le registre
- INV-REL-2 : Une relation Requires ne peut pas crÃ©er de cycle
- INV-REL-3 : Une relation Conflicts est symÃ©trique
- INV-REL-4 : Une relation Supersedes implique la dÃ©prÃ©ciation de la capacitÃ© remplacÃ©e

### 5.2. Graphe des Relations

**Ã‰noncÃ© :**

Le **Graphe des Relations** est la structure qui modÃ©lise toutes les relations entre capacitÃ©s dans le registre.

**CaractÃ©ristiques :**
- OrientÃ© : Les relations ont une direction (from â†’ to)
- Acyclique : Pas de cycles dans les relations Requires
- Complet : Toutes les relations dÃ©clarÃ©es sont prÃ©sentes
- CohÃ©rent : Pas de relation vers une capacitÃ© inexistante

**OpÃ©rations sur le graphe :**

| OpÃ©ration | Description |
|-----------|-------------|
| `GetDependencies(cap)` | CapacitÃ©s requises par cap |
| `GetDependents(cap)` | CapacitÃ©s qui requiÃ¨rent cap |
| `GetImplied(cap)` | CapacitÃ©s impliquÃ©es par cap |
| `GetConflicts(cap)` | CapacitÃ©s en conflit avec cap |
| `GetGroup(cap)` | CapacitÃ©s regroupÃ©es par cap |

**Invariants :**
- INV-GRAPH-1 : Le graphe des relations Requires est acyclique
- INV-GRAPH-2 : Toute capacitÃ© rÃ©fÃ©rencÃ©e existe dans le registre
- INV-GRAPH-3 : Le graphe est cohÃ©rent avec le statut des capacitÃ©s

---

## 6. RÃ¨gles de DÃ©claration

### 6.1. Qui peut dÃ©clarer des CapacitÃ©s

**Ã‰noncÃ© :**

Seuls certains types de composants sont autorisÃ©s Ã  dÃ©clarer des capacitÃ©s dans le registre.

**Sources autorisÃ©es :**

| Source | Peut dÃ©clarer | Exemples |
|--------|---------------|----------|
| **Module SPM** | Ses propres capacitÃ©s | `spm.cms.content.create` |
| **Core** | Ses capacitÃ©s de gouvernance | `core.strongfather.evaluate` |
| **Operator** | Ses capacitÃ©s spÃ©cifiques | `operator.myapp.custom_action` |
| **Tool** | Ses capacitÃ©s atomiques | `tool.layout.render` |
| **Toolkit** | Aucune (composition uniquement) | - |

**RÃ¨gles :**
- R-DECL-1 : Un composant ne peut dÃ©clarer que ses propres capacitÃ©s
- R-DECL-2 : Un composant ne peut pas dÃ©clarer de capacitÃ©s au nom d'un autre
- R-DECL-3 : La dÃ©claration est vÃ©rifiÃ©e par Master Butler
- R-DECL-4 : Un Toolkit ne dÃ©clare pas de capacitÃ© (il rÃ©fÃ©rence des capacitÃ©s existantes)

### 6.2. Quand dÃ©clarer les CapacitÃ©s

**Ã‰noncÃ© :**

Les capacitÃ©s doivent Ãªtre dÃ©clarÃ©es Ã  des moments spÃ©cifiques du cycle de vie du systÃ¨me.

**Moments de dÃ©claration :**

| Moment | Description |
|--------|-------------|
| **Initialisation** | Au dÃ©marrage du composant |
| **Mise Ã  jour** | Lors de l'ajout de nouvelles fonctionnalitÃ©s |
| **Migration** | Lors du remplacement d'une capacitÃ© |

**RÃ¨gles :**
- R-WHEN-1 : Les capacitÃ©s sont dÃ©clarÃ©es avant toute utilisation
- R-WHEN-2 : Une capacitÃ© non dÃ©clarÃ©e ne peut pas Ãªtre utilisÃ©e
- R-WHEN-3 : La dÃ©claration peut Ãªtre rÃ©pÃ©tÃ©e (idempotente)
- R-WHEN-4 : La mise Ã  jour des mÃ©tadonnÃ©es est autorisÃ©e

### 6.3. Validation des DÃ©clarations

**Ã‰noncÃ© :**

Master Butler valide toutes les dÃ©clarations avant de les enregistrer dans le registre.

**Validations effectuÃ©es :**

| Validation | Description | Erreur si Ã©chec |
|------------|-------------|-----------------|
| Format de l'identifiant | Respect du format canonique | `InvalidCapabilityId` |
| UnicitÃ© de l'identifiant | Pas de duplication | `DuplicateCapabilityId` |
| MÃ©tadonnÃ©es obligatoires | PrÃ©sence et validitÃ© | `MissingMetadata` |
| Autorisation de la source | Source autorisÃ©e Ã  dÃ©clarer | `UnauthorizedSource` |
| CohÃ©rence des relations | Relations vers capacitÃ©s existantes | `InvalidRelation` |
| Pas de cycle | Pas de cycle dans les dÃ©pendances | `CyclicDependency` |

**Invariants :**
- INV-VAL-1 : Aucune capacitÃ© invalide n'est enregistrÃ©e
- INV-VAL-2 : Toutes les validations sont effectuÃ©es atomiquement
- INV-VAL-3 : Un Ã©chec de validation ne modifie pas le registre

---

## 7. TraÃ§abilitÃ© et Historique

### 7.1. Historique des CapacitÃ©s

**Ã‰noncÃ© :**

Toute modification du registre des capacitÃ©s est enregistrÃ©e dans un historique immuable.

**Ã‰vÃ©nements historisÃ©s :**

| Ã‰vÃ©nement | Description | DonnÃ©es enregistrÃ©es |
|-----------|-------------|---------------------|
| `CapabilityCreated` | Nouvelle capacitÃ© dÃ©clarÃ©e | CapacitÃ© complÃ¨te, source, timestamp |
| `CapabilityUpdated` | MÃ©tadonnÃ©es modifiÃ©es | Champs modifiÃ©s, ancienne/nouvelle valeur |
| `CapabilityDeprecated` | CapacitÃ© dÃ©prÃ©ciÃ©e | Raison, successeur, timestamp |
| `CapabilityRemoved` | CapacitÃ© supprimÃ©e | Raison, timestamp |
| `RelationAdded` | Nouvelle relation | Relation complÃ¨te |
| `RelationRemoved` | Relation supprimÃ©e | Relation, raison |

**Structure d'un Ã©vÃ©nement :**

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
- INV-HIST-2 : Chaque Ã©vÃ©nement possÃ¨de un identifiant unique
- INV-HIST-3 : L'ordre des Ã©vÃ©nements est prÃ©servÃ©
- INV-HIST-4 : Chaque Ã©vÃ©nement est signÃ© (checksum)

### 7.2. Audit du Registre

**Ã‰noncÃ© :**

Le registre peut Ãªtre auditÃ© Ã  tout moment pour vÃ©rifier sa cohÃ©rence et retracer l'Ã©volution des capacitÃ©s.

**OpÃ©rations d'audit :**

| OpÃ©ration | Description |
|-----------|-------------|
| `GetHistory(cap)` | Historique complet d'une capacitÃ© |
| `GetStateAt(timestamp)` | Ã‰tat du registre Ã  un instant donnÃ© |
| `VerifyIntegrity()` | VÃ©rification de la cohÃ©rence |
| `GetStatistics()` | Statistiques du registre |

**Informations d'audit disponibles :**
- Nombre total de capacitÃ©s (par statut)
- Historique de chaque capacitÃ©
- Sources les plus actives
- Relations les plus utilisÃ©es
- DÃ©prÃ©ciations rÃ©centes

---

## 8. Invariants Non NÃ©gociables

### 8.1. ExhaustivitÃ©

**Invariant INV-NN-1 :**

> **Toute capacitÃ© existant dans le systÃ¨me est recensÃ©e dans le registre de Master Butler.**

**Implications :**
- Aucune capacitÃ© "cachÃ©e" ou non dÃ©clarÃ©e
- Aucun contournement du registre
- Master Butler est la source de vÃ©ritÃ© unique

### 8.2. UnicitÃ© des Identifiants

**Invariant INV-NN-2 :**

> **Chaque capacitÃ© possÃ¨de un identifiant unique et immuable, jamais rÃ©utilisable.**

**Implications :**
- Pas de collision d'identifiants
- Pas de rÃ©utilisation aprÃ¨s suppression
- TraÃ§abilitÃ© parfaite

### 8.3. Idempotence des DÃ©clarations

**Invariant INV-NN-3 :**

> **Les dÃ©clarations de capacitÃ©s sont idempotentes. DÃ©clarer deux fois la mÃªme capacitÃ© n'a pas d'effet supplÃ©mentaire.**

**Implications :**
- RedÃ©claration au dÃ©marrage autorisÃ©e
- Pas d'effet de bord sur les redÃ©clarations
- CohÃ©rence garantie

### 8.4. TraÃ§abilitÃ© ComplÃ¨te

**Invariant INV-NN-4 :**

> **Toute modification du registre est tracÃ©e avec contexte complet (qui, quand, quoi).**

**Implications :**
- Audit possible Ã  tout moment
- Historique immuable
- ResponsabilitÃ© identifiable

### 8.5. Non-DÃ©cision

**Invariant INV-NN-5 :**

> **Le registre recense les capacitÃ©s mais ne dÃ©cide jamais de leur utilisation.**

**Implications :**
- Master Butler informe, ne dÃ©cide pas
- La dÃ©cision appartient Ã  StrongFather
- SÃ©paration stricte connaissance/dÃ©cision

---

## 9. SchÃ©mas ASCII

### 9.1. Structure du Registre

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                        CAPABILITY REGISTRY                                   â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                              â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚  INDEX PRINCIPAL (Map<CapabilityId, Capability>)                      â”‚  â”‚
â”‚  â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤  â”‚
â”‚  â”‚  content.create      â†’ Capability { name: "CrÃ©er contenu", ... }     â”‚  â”‚
â”‚  â”‚  content.edit        â†’ Capability { name: "Modifier contenu", ... }  â”‚  â”‚
â”‚  â”‚  content.delete      â†’ Capability { name: "Supprimer contenu", ... } â”‚  â”‚
â”‚  â”‚  media.upload        â†’ Capability { name: "TÃ©lÃ©verser mÃ©dia", ... }  â”‚  â”‚
â”‚  â”‚  hierarchy.reorder   â†’ Capability { name: "RÃ©ordonner", ... }        â”‚  â”‚
â”‚  â”‚  search.index        â†’ Capability { name: "Indexer", ... }           â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚                                                                              â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚  INDEX PAR CATÃ‰GORIE   â”‚    â”‚  INDEX PAR SOURCE                       â”‚  â”‚
â”‚  â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤    â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤  â”‚
â”‚  â”‚  Data:                 â”‚    â”‚  spm.cms.content:                       â”‚  â”‚
â”‚  â”‚    - content.create    â”‚    â”‚    - content.create                     â”‚  â”‚
â”‚  â”‚    - content.edit      â”‚    â”‚    - content.edit                       â”‚  â”‚
â”‚  â”‚    - content.delete    â”‚    â”‚    - content.delete                     â”‚  â”‚
â”‚  â”‚                        â”‚    â”‚                                          â”‚  â”‚
â”‚  â”‚  Media:                â”‚    â”‚  spm.cms.media:                          â”‚  â”‚
â”‚  â”‚    - media.upload      â”‚    â”‚    - media.upload                        â”‚  â”‚
â”‚  â”‚                        â”‚    â”‚                                          â”‚  â”‚
â”‚  â”‚  Hierarchy:            â”‚    â”‚  spm.cms.hierarchy:                      â”‚  â”‚
â”‚  â”‚    - hierarchy.reorder â”‚    â”‚    - hierarchy.reorder                   â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚                                                                              â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚  GRAPHE DES RELATIONS                                                 â”‚  â”‚
â”‚  â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤  â”‚
â”‚  â”‚                                                                        â”‚  â”‚
â”‚  â”‚  content.publish â”€â”€[Requires]â”€â”€â–º content.read                         â”‚  â”‚
â”‚  â”‚  content.manage  â”€â”€[Groups]â”€â”€â”€â”€â–º content.create                       â”‚  â”‚
â”‚  â”‚                  â”€â”€[Groups]â”€â”€â”€â”€â–º content.edit                         â”‚  â”‚
â”‚  â”‚                  â”€â”€[Groups]â”€â”€â”€â”€â–º content.delete                       â”‚  â”‚
â”‚  â”‚  content.lock    â”€â”€[Conflicts]â”€â–º content.edit                         â”‚  â”‚
â”‚  â”‚                                                                        â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚                                                                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 9.2. Flux de DÃ©claration

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                     FLUX DE DÃ‰CLARATION DE CAPACITÃ‰                          â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜

   MODULE SPM                    MASTER BUTLER                    REGISTRE
       â”‚                              â”‚                              â”‚
       â”‚  DeclareCapability(          â”‚                              â”‚
       â”‚    id: "content.create",     â”‚                              â”‚
       â”‚    name: "CrÃ©er contenu",    â”‚                              â”‚
       â”‚    source: "spm.cms",        â”‚                              â”‚
       â”‚    category: Data            â”‚                              â”‚
       â”‚  )                           â”‚                              â”‚
       â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–ºâ”‚                              â”‚
       â”‚                              â”‚                              â”‚
       â”‚                              â”‚  1. Valider format id        â”‚
       â”‚                              â”‚  2. VÃ©rifier unicitÃ©         â”‚
       â”‚                              â”‚  3. Valider mÃ©tadonnÃ©es      â”‚
       â”‚                              â”‚  4. VÃ©rifier autorisation    â”‚
       â”‚                              â”‚                              â”‚
       â”‚                              â”‚  [Validations OK]            â”‚
       â”‚                              â”‚                              â”‚
       â”‚                              â”‚  5. CrÃ©er Capability         â”‚
       â”‚                              â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–ºâ”‚
       â”‚                              â”‚                              â”‚
       â”‚                              â”‚  6. Mettre Ã  jour index      â”‚
       â”‚                              â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–ºâ”‚
       â”‚                              â”‚                              â”‚
       â”‚                              â”‚  7. Historiser Ã©vÃ©nement     â”‚
       â”‚                              â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–ºâ”‚
       â”‚                              â”‚                              â”‚
       â”‚                              â”‚â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
       â”‚                              â”‚      [Capability crÃ©Ã©e]      â”‚
       â”‚â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤                              â”‚
       â”‚    Result::Ok(Capability)    â”‚                              â”‚
       â”‚                              â”‚                              â”‚
       â–¼                              â–¼                              â–¼
```

### 9.3. Cycle de Vie d'une CapacitÃ©

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                     CYCLE DE VIE D'UNE CAPACITÃ‰                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜

                              DeclareCapability()
                                     â”‚
                                     â–¼
                        â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
                        â”‚                        â”‚
                        â”‚       ACTIVE           â”‚
                        â”‚                        â”‚
                        â”‚   âœ“ Utilisable         â”‚
                        â”‚   âœ“ Interrogeable      â”‚
                        â”‚   âœ“ RÃ©fÃ©renÃ§able       â”‚
                        â”‚                        â”‚
                        â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                                â”‚          â”‚
                                â”‚          â”‚
          DeprecateCapability() â”‚          â”‚ RemoveCapability()
                                â”‚          â”‚
                                â–¼          â”‚
                        â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
                        â”‚                        â”‚
                        â”‚      DEPRECATED        â”‚
                        â”‚                        â”‚
                        â”‚   âœ“ Interrogeable      â”‚
                        â”‚   âš  Utilisation        â”‚
                        â”‚     dÃ©conseillÃ©e       â”‚
                        â”‚   âœ“ Successeur indiquÃ© â”‚
                        â”‚                        â”‚
                        â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                                     â”‚
                                     â”‚ RemoveCapability()
                                     â”‚
                                     â–¼
                        â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
                        â”‚                        â”‚
                        â”‚       REMOVED          â”‚
                        â”‚                        â”‚
                        â”‚   âœ— Non utilisable     â”‚
                        â”‚   âœ— Non interrogeable  â”‚
                        â”‚   âœ“ Historique conservÃ©â”‚
                        â”‚   âœ— Id non rÃ©utilisableâ”‚
                        â”‚                        â”‚
                        â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                                     â”‚
                                     â”‚ (TERMINAL)
                                     â–¼

                    âš ï¸ TRANSITIONS IRRÃ‰VERSIBLES âš ï¸
```

---

## 10. Exemples Concrets

### 10.1. Exemple : DÃ©claration de CapacitÃ©s CMS

**Contexte :**
Le module SPM CMS Content dÃ©clare ses capacitÃ©s au dÃ©marrage.

**DÃ©clarations :**

```
// CapacitÃ© de crÃ©ation de contenu
DeclareCapability(
  id: "content.create",
  name: "CrÃ©er du contenu",
  description: "CapacitÃ© de crÃ©er un nouveau contenu dans le systÃ¨me CMS",
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

// CapacitÃ© de lecture de contenu
DeclareCapability(
  id: "content.read",
  name: "Lire du contenu",
  description: "CapacitÃ© de lire le contenu existant",
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

// CapacitÃ© de publication avec dÃ©pendance
DeclareCapability(
  id: "content.publish",
  name: "Publier du contenu",
  description: "CapacitÃ© de publier un contenu existant",
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
StrongFather interroge Master Butler pour Ã©valuer une intention.

**Interrogation par identifiant :**

```
QueryCapability(
  filter: { id: "content.create" }
)
â†’ Result::Ok([
    Capability {
      id: "content.create",
      name: "CrÃ©er du contenu",
      status: Active,
      ...
    }
  ])
```

**Interrogation par catÃ©gorie :**

```
QueryCapability(
  filter: { category: Data, status: Active }
)
â†’ Result::Ok([
    Capability { id: "content.create", ... },
    Capability { id: "content.read", ... },
    Capability { id: "content.edit", ... },
    Capability { id: "content.delete", ... },
    Capability { id: "content.publish", ... }
  ])
```

### 10.3. Exemple : DÃ©prÃ©ciation et Migration

**Contexte :**
Une capacitÃ© est remplacÃ©e par une nouvelle version.

**DÃ©prÃ©ciation :**

```
// CrÃ©er la nouvelle version
DeclareCapability(
  id: "content.create.v2",
  name: "CrÃ©er du contenu (v2)",
  description: "Nouvelle version avec support multi-langue",
  ...
)

// DÃ©prÃ©cier l'ancienne version
DeprecateCapability(
  id: "content.create",
  reason: "RemplacÃ©e par content.create.v2 avec support multi-langue",
  successor: "content.create.v2"
)
```

**Ã‰tat aprÃ¨s dÃ©prÃ©ciation :**

```
Capability {
  id: "content.create",
  status: Deprecated,
  deprecated_at: "2026-01-27T15:30:00Z",
  deprecation_reason: "RemplacÃ©e par content.create.v2",
  successor: "content.create.v2"
}
```

---

## 11. Conclusion

Ce contrat Ã©tablit le modÃ¨le technique du registre des capacitÃ©s de Master Butler, dÃ©finissant de maniÃ¨re absolue :

**Points clÃ©s :**
- **CapacitÃ© :** Pouvoir technique intrinsÃ¨que, identifiÃ© de maniÃ¨re unique et immuable
- **CapabilityId :** Format canonique `<domain>.<module>.<action>[.<qualifier>]`
- **Registre :** Structure exhaustive, cohÃ©rente, tracÃ©e, et indexÃ©e
- **MÃ©tadonnÃ©es :** Obligatoires (id, name, description, source, category) et optionnelles
- **Statuts :** Active â†’ Deprecated â†’ Removed (transitions irrÃ©versibles)
- **Relations :** Requires, Implies, Conflicts, Supersedes, Groups
- **OpÃ©rations :** Declare, Query, Deprecate, Remove (toutes validÃ©es)
- **TraÃ§abilitÃ© :** Historique immuable de toutes les modifications

**Invariants non nÃ©gociables :**
- ExhaustivitÃ© du registre
- UnicitÃ© et immutabilitÃ© des identifiants
- Idempotence des dÃ©clarations
- TraÃ§abilitÃ© complÃ¨te
- Non-dÃ©cision (Master Butler informe, ne dÃ©cide pas)

Ce contrat complÃ¨te la Documentation Fondatrice de Master Butler en dÃ©finissant le modÃ¨le technique du registre des capacitÃ©s. Il s'articule avec le Permission Registry Contract pour former le systÃ¨me complet de catalogage des capacitÃ©s et permissions.

**Non-nÃ©gociabilitÃ© :** Ce contrat est absolu et non nÃ©gociable. Le contrat prime sur toute considÃ©ration pratique.

---

**Document crÃ©Ã© le :** 2026-01-27  
**Version :** 1.0  
**Statut :** FONDATION â€” Contrat normatif validÃ©  
**RÃ©fÃ©rence :** Miyukini Core System v2.4, Master Butler Documentation Fondatrice  
**Type :** Contrat de registre non nÃ©gociable

---

## 12. Mini log â€” erreurs / warnings / ambiguÃ¯tÃ©s rencontrÃ©es et corrigÃ©es

### AmbiguÃ¯tÃ© A1 : Format de l'identifiant de capacitÃ©

**AmbiguÃ¯tÃ© rencontrÃ©e :**
La documentation fondatrice donne des exemples d'identifiants (`content.create`, `hierarchy.reorder`) sans dÃ©finir un format canonique strict.

**DÃ©cision prise :**
Format canonique dÃ©fini : `<domain>.<module>.<action>[.<qualifier>]` avec rÃ¨gles de nommage strictes (minuscules, sans accents, segments sÃ©parÃ©s par points).

**Justification :**
Un format strict garantit la cohÃ©rence, Ã©vite les collisions, et facilite l'indexation et la recherche.

**Correction effectuÃ©e :**
Section 2.2 "Identifiant de CapacitÃ©" ajoutÃ©e avec format canonique et rÃ¨gles de nommage (R-ID-1 Ã  R-ID-6).

### AmbiguÃ¯tÃ© A2 : Cycle de vie des capacitÃ©s

**AmbiguÃ¯tÃ© rencontrÃ©e :**
La documentation fondatrice mentionne l'historique des capacitÃ©s (ajouts, suppressions, modifications) sans dÃ©finir un cycle de vie formel.

**DÃ©cision prise :**
Cycle de vie Ã  trois Ã©tats (Active, Deprecated, Removed) avec transitions irrÃ©versibles et rÃ¨gles explicites.

**Justification :**
Un cycle de vie formel garantit la cohÃ©rence temporelle et permet une gestion propre des migrations et dÃ©prÃ©ciations.

**Correction effectuÃ©e :**
Section 3.3 "Statut de CapacitÃ©" ajoutÃ©e avec Ã©tats, transitions, et rÃ¨gles (R-ST-1 Ã  R-ST-6).

### AmbiguÃ¯tÃ© A3 : Relations entre capacitÃ©s

**AmbiguÃ¯tÃ© rencontrÃ©e :**
La documentation fondatrice mentionne des "relations entre capacitÃ©s (dÃ©pendances, hiÃ©rarchies)" sans les dÃ©finir formellement.

**DÃ©cision prise :**
Cinq types de relations dÃ©finis (Requires, Implies, Conflicts, Supersedes, Groups) avec sÃ©mantique et invariants.

**Justification :**
Des types de relations formels permettent de modÃ©liser toutes les interactions entre capacitÃ©s de maniÃ¨re cohÃ©rente.

**Correction effectuÃ©e :**
Section 5 "Relations entre CapacitÃ©s" ajoutÃ©e avec types, graphe, et invariants (INV-REL-1 Ã  INV-REL-4, INV-GRAPH-1 Ã  INV-GRAPH-3).

### AmbiguÃ¯tÃ© A4 : Idempotence des dÃ©clarations

**AmbiguÃ¯tÃ© rencontrÃ©e :**
La documentation fondatrice mentionne que les dÃ©clarations sont idempotentes sans prÃ©ciser le comportement exact en cas de redÃ©claration avec contenu diffÃ©rent.

**DÃ©cision prise :**
Idempotence stricte : mÃªme id + mÃªme contenu = ok, mÃªme id + contenu diffÃ©rent = erreur.

**Justification :**
Cette rÃ¨gle Ã©vite les incohÃ©rences tout en permettant la redÃ©claration au dÃ©marrage.

**Correction effectuÃ©e :**
Section 4.1 "DÃ©claration de CapacitÃ©" avec rÃ¨gles d'idempotence (R-IDEMP-1 Ã  R-IDEMP-2).

---

*Aucune autre erreur, warning, ou ambiguÃ¯tÃ© rencontrÃ©e lors de la rÃ©daction de ce document.*

