# Master Butler — Tool Governance Contract

## 1. Introduction

### Objet du contrat

Ce document definit le **Master Butler Tool Governance Contract** : un contrat normatif, non negociable, et de statut FONDATION qui etablit les regles de gouvernance des Outils (Tools) et Kits d'Outils (Toolkits) dans le systeme Miyukini Core System v2.4.

Ce contrat definit :
- Le modele de declaration des Tools dans Master Butler
- La liaison entre Capacites et Tools
- La definition et composition des Toolkits
- Les permissions d'acces aux Tools et Toolkits
- Les regles de souverainete applicative
- Les invariants de gouvernance

### Portee

Ce contrat s'applique a **toute instance de Master Butler** et definit de maniere absolue :
- La structure formelle d'un Tool dans le catalogue
- La structure formelle d'un Toolkit
- Les regles de declaration et de liaison
- Les permissions d'acces et d'utilisation
- Les contraintes de souverainete
- Les invariants non negociables

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il etablit des regles absolues que Master Butler applique sans exception. Ces regles ne peuvent etre contournees, negociees, ou modifiees. Le contrat prime sur toute consideration pratique.

### Relation avec les autres contrats

Ce contrat complete et s'articule avec les documents contractuels existants :

- **[Master Butler - Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md)** : Definit la nature, le role, et les responsabilites de Master Butler, incluant la section 10 sur la gouvernance des Tools
- **[Master Butler - Capability Registry Contract](../registry/Master%20Butler%20-%20Capability%20Registry%20Contract.md)** : Definit le registre des capacites (les Tools exposent des capacites)
- **[Master Butler - Permission Registry Contract](../registry/Master%20Butler%20-%20Permission%20Registry%20Contract.md)** : Definit le registre des permissions (acces aux Tools)
- **[Miyukini Conceptual References - Tools et Toolkits](../../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)** : Reference conceptuelle definissant Tools et Toolkits
- **[Miyukini Framework - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Ce contrat respecte **LOI-1** (aucune dependance externe critique) en garantissant une bibliotheque d'outils locale et gouvernee

**Complementarite :**
- Miyukini Conceptual References - Tools et Toolkits = definitions conceptuelles et philosophiques
- Master Butler Tool Governance Contract = modele technique de gouvernance dans Master Butler

---

## 2. Definitions Formelles

### 2.1. Outil (Tool)

**Definition formelle :**

Un **Outil (Tool)** est une capacite executable, sans autorite, sans decision metier, sans connaissance de l'Operateur appelant, gouvernee par les Cores.

**Caracteristiques formelles :**

| Propriete | Description |
|-----------|-------------|
| **Capacite executable** | Fait quelque chose de concret et atomique |
| **Sans autorite** | Ne decide jamais si l'action doit etre faite |
| **Sans decision metier** | Ne contient aucune logique business |
| **Sans connaissance du contexte** | Ignore quel Operateur l'appelle et pourquoi |
| **Gouverne par les Cores** | Soumis a l'autorisation et au controle des Cores |

**Regle fondamentale :**

> **Un Outil fait, mais ne decide jamais.**

**Structure formelle d'un Tool dans Master Butler :**

```
Tool {
  id: ToolId,                     // Identifiant unique du Tool
  name: String,                   // Nom lisible humain
  description: String,            // Description de l'Outil
  capability_id: CapabilityId,    // Capacite exposee par ce Tool
  source: SourceIdentity,         // Module/composant qui fournit le Tool
  metadata: ToolMetadata,         // Metadonnees additionnelles
  security_level: SecurityLevel,  // Niveau de securite requis
  status: ToolStatus,             // Statut (Active, Deprecated, Removed)
  created_at: Timestamp,          // Date de creation
  version: Version                // Version du Tool
}
```

**Invariants :**
- INV-TOOL-1 : Tout Tool possede un ToolId unique et immuable
- INV-TOOL-2 : Tout Tool est lie a exactement une Capability
- INV-TOOL-3 : Tout Tool est declare par exactement une source
- INV-TOOL-4 : Un Tool ne prend jamais de decision metier
- INV-TOOL-5 : Un Tool ne connait jamais l'Operateur appelant

### 2.2. Identifiant d'Outil (ToolId)

**Definition formelle :**

Un **ToolId** est l'identifiant unique et immuable d'un Tool dans le catalogue de Master Butler. Il suit un format canonique coherent avec les CapabilityIds.

**Format canonique :**

```
tool.<domain>.<action>[.<qualifier>]
```

**Exemples :**
- `tool.layout.render` : Outil de rendu de layout
- `tool.input.capture` : Outil de capture de saisie
- `tool.form.validate` : Outil de validation de formulaire
- `tool.query.execute` : Outil d'execution de requete
- `tool.file.read` : Outil de lecture de fichier
- `tool.cache.get` : Outil de recuperation en cache

**Regles de nommage :**
- R-TID-1 : Le format est strictement `tool.<domain>.<action>[.<qualifier>]`
- R-TID-2 : Le prefixe `tool.` est obligatoire
- R-TID-3 : Tous les segments sont en minuscules, sans accents
- R-TID-4 : Les segments sont separes par des points (.)
- R-TID-5 : Chaque segment contient uniquement des lettres, chiffres, et underscores
- R-TID-6 : L'identifiant est unique dans le catalogue

**Invariants :**
- INV-TID-1 : Tout ToolId respecte le format canonique
- INV-TID-2 : Tout ToolId est unique dans le catalogue
- INV-TID-3 : Un ToolId ne peut jamais etre modifie apres creation
- INV-TID-4 : Le prefixe `tool.` distingue les Tools des autres entites

### 2.3. Kit d'Outils (Toolkit)

**Definition formelle :**

Un **Kit d'Outils (Toolkit)** est une composition officielle d'Outils, validee et declaree par l'environnement, optimisee pour efficience, coherence et performance.

**Caracteristiques formelles :**

| Propriete | Description |
|-----------|-------------|
| **Composition officielle** | Agregation formelle d'Outils existants |
| **Valide par l'environnement** | Declare et gouverne |
| **Optimise** | Pour efficience, coherence, performance |
| **Sans capacite nouvelle** | N'ajoute aucune fonctionnalite que les Outils n'ont pas |
| **Sans logique metier** | Orchestration pure, pas de decision |

**Regle fondamentale :**

> **Un Kit d'Outils n'ajoute aucune capacite nouvelle, il orchestre proprement des Outils existants.**

**Structure formelle d'un Toolkit dans Master Butler :**

```
Toolkit {
  id: ToolkitId,                    // Identifiant unique du Toolkit
  name: String,                     // Nom lisible humain
  description: String,              // Description du Kit d'Outils
  tools: Set<ToolId>,               // Ensemble des Tools composes
  metadata: ToolkitMetadata,        // Metadonnees additionnelles
  security_level: SecurityLevel,    // Niveau de securite requis (max des Tools)
  allowed_states: Set<SystemState>, // Etats systeme autorisant l'utilisation
  disallowed_states: Set<SystemState>, // Etats systeme interdisant l'utilisation
  status: ToolkitStatus,            // Statut (Active, Deprecated, Removed)
  created_at: Timestamp,            // Date de creation
  version: Version                  // Version du Toolkit
}
```

**Invariants :**
- INV-TK-1 : Tout Toolkit possede un ToolkitId unique et immuable
- INV-TK-2 : Tout Toolkit contient au moins un Tool
- INV-TK-3 : Tous les Tools d'un Toolkit existent dans le catalogue
- INV-TK-4 : Un Toolkit ne cree pas de capacite nouvelle
- INV-TK-5 : Un Toolkit ne contient pas de logique metier
- INV-TK-6 : Le niveau de securite d'un Toolkit est le maximum de ses Tools

### 2.4. Identifiant de Kit d'Outils (ToolkitId)

**Definition formelle :**

Un **ToolkitId** est l'identifiant unique et immuable d'un Toolkit dans le catalogue de Master Butler.

**Format canonique :**

```
toolkit.<domain>.<name>
```

**Exemples :**
- `toolkit.ui.standard` : Kit d'Outils UI Standard
- `toolkit.data.crud` : Kit d'Outils CRUD Data
- `toolkit.io.filesystem` : Kit d'Outils systeme de fichiers

**Regles de nommage :**
- R-TKID-1 : Le format est strictement `toolkit.<domain>.<name>`
- R-TKID-2 : Le prefixe `toolkit.` est obligatoire
- R-TKID-3 : Tous les segments sont en minuscules, sans accents
- R-TKID-4 : Les segments sont separes par des points (.)
- R-TKID-5 : L'identifiant est unique dans le catalogue

**Invariants :**
- INV-TKID-1 : Tout ToolkitId respecte le format canonique
- INV-TKID-2 : Tout ToolkitId est unique dans le catalogue
- INV-TKID-3 : Un ToolkitId ne peut jamais etre modifie apres creation

---

## 3. Catalogue des Tools

### 3.1. Structure du Catalogue

**Enonce :**

Le **Catalogue des Tools** est la structure centrale de Master Butler pour la gouvernance des Outils. Il contient l'inventaire exhaustif de tous les Tools de l'environnement.

**Structure formelle :**

```
ToolCatalog {
  tools: Map<ToolId, Tool>,                 // Index principal des Tools
  toolkits: Map<ToolkitId, Toolkit>,        // Index principal des Toolkits
  by_capability: Map<CapabilityId, ToolId>, // Index Capability → Tool
  by_domain: Map<Domain, Set<ToolId>>,      // Index par domaine
  by_security_level: Map<SecurityLevel, Set<ToolId>>, // Index par niveau securite
  history: ToolCatalogHistory,              // Historique des modifications
  version: CatalogVersion                   // Version du catalogue
}
```

**Caracteristiques :**
- **Exhaustif :** Contient tous les Tools et Toolkits de l'environnement
- **Coherent :** Aucune duplication, aucune incohérence
- **Tracable :** Historique complet de toutes les modifications
- **Indexe :** Recherche efficace par identifiant, capability, domaine
- **Gouverne :** Aucun Tool ou Toolkit non declare

**Invariants :**
- INV-CAT-1 : Le catalogue contient tous les Tools declares de l'environnement
- INV-CAT-2 : Aucun Tool n'existe en dehors du catalogue
- INV-CAT-3 : Le catalogue est coherent a tout instant
- INV-CAT-4 : Toute modification du catalogue est historisee
- INV-CAT-5 : Chaque Capability a au plus un Tool associe

### 3.2. Liaison Capability-Tool

**Enonce :**

La **liaison Capability-Tool** etablit la correspondance entre une Capacite (declaree dans le Capability Registry) et l'Outil qui l'implemente.

**Regle fondamentale :**

> **Une Capability peut etre liee a un seul Tool. Un Tool expose exactement une Capability.**

**Structure de liaison :**

```
CapabilityToolBinding {
  capability_id: CapabilityId,  // Capacite exposee
  tool_id: ToolId,              // Tool qui implemente
  binding_type: BindingType,    // Type de liaison
  created_at: Timestamp         // Date de liaison
}

BindingType {
  Direct,      // Liaison directe 1:1
  Delegated    // Liaison via un autre composant
}
```

**Invariants :**
- INV-BIND-1 : Une Capability peut etre liee a au plus un Tool
- INV-BIND-2 : Un Tool est lie a exactement une Capability
- INV-BIND-3 : La Capability liee doit exister dans le Capability Registry
- INV-BIND-4 : Le Tool lie doit exister dans le Tool Catalog
- INV-BIND-5 : La liaison est tracee et historisee

### 3.3. Metadonnees des Tools

**Enonce :**

Tout Tool declare doit fournir un ensemble minimal de metadonnees obligatoires.

**Metadonnees obligatoires :**

| Metadonnee | Type | Description |
|------------|------|-------------|
| `id` | ToolId | Identifiant unique et immuable |
| `name` | String | Nom lisible humain (max 100 caracteres) |
| `description` | String | Description du Tool (max 1000 caracteres) |
| `capability_id` | CapabilityId | Capacite exposee |
| `source` | SourceIdentity | Composant qui fournit le Tool |
| `security_level` | SecurityLevel | Niveau de securite requis |
| `created_at` | Timestamp | Date et heure de creation |
| `version` | Version | Version du Tool |

**Metadonnees optionnelles :**

| Metadonnee | Type | Description |
|------------|------|-------------|
| `tags` | Set<String> | Tags de classification |
| `documentation_url` | URL | Lien vers la documentation |
| `examples` | List<String> | Exemples d'utilisation |
| `input_schema` | Schema | Schema des entrees |
| `output_schema` | Schema | Schema des sorties |
| `execution_mode` | ExecutionMode | Sync, Async, Streaming |
| `timeout_ms` | u32 | Timeout d'execution en millisecondes |
| `deprecated_at` | Timestamp | Date de depreciation |
| `successor` | ToolId | Tool de remplacement |

**Invariants :**
- INV-META-1 : Toutes les metadonnees obligatoires sont presentes et valides
- INV-META-2 : Le niveau de securite est un entier de 0 a 4
- INV-META-3 : La Capability referencee existe dans le Capability Registry

---

## 4. Operations sur le Catalogue

### 4.1. Declaration d'Outil (DeclareTool)

**Enonce :**

L'operation **DeclareTool** permet a un composant de declarer un nouveau Tool dans le catalogue.

**Signature conceptuelle :**

```
DeclareTool(
  id: ToolId,
  name: String,
  description: String,
  capability_id: CapabilityId,
  source: SourceIdentity,
  security_level: SecurityLevel,
  metadata: ToolMetadata?
) → Result<Tool, DeclarationError>
```

**Preconditions :**
- PRE-1 : Le ToolId n'existe pas deja dans le catalogue
- PRE-2 : Le ToolId respecte le format canonique
- PRE-3 : La Capability referencee existe dans le Capability Registry
- PRE-4 : La Capability n'est pas deja liee a un autre Tool
- PRE-5 : La source est autorisee a declarer ce Tool
- PRE-6 : Le niveau de securite est valide (0-4)

**Postconditions :**
- POST-1 : Le Tool est ajoute au catalogue avec statut Active
- POST-2 : La liaison Capability-Tool est creee
- POST-3 : Les index sont mis a jour
- POST-4 : L'historique est mis a jour avec l'evenement de creation
- POST-5 : La version du catalogue est incrementee

**Erreurs possibles :**

| Erreur | Description |
|--------|-------------|
| `DuplicateToolId` | Un Tool avec cet id existe deja |
| `InvalidToolId` | Le format de l'id est invalide |
| `CapabilityNotFound` | La Capability referencee n'existe pas |
| `CapabilityAlreadyBound` | La Capability est deja liee a un Tool |
| `UnauthorizedSource` | La source n'est pas autorisee |
| `InvalidSecurityLevel` | Le niveau de securite est invalide |

**Regles d'idempotence :**
- R-IDEMP-1 : Declarer deux fois le meme Tool (meme id, meme contenu) est idempotent
- R-IDEMP-2 : Declarer un Tool avec un id existant mais contenu different est une erreur

### 4.2. Declaration de Kit d'Outils (DeclareToolkit)

**Enonce :**

L'operation **DeclareToolkit** permet de declarer un nouveau Kit d'Outils composant des Tools existants.

**Signature conceptuelle :**

```
DeclareToolkit(
  id: ToolkitId,
  name: String,
  description: String,
  tools: Set<ToolId>,
  allowed_states: Set<SystemState>?,
  disallowed_states: Set<SystemState>?,
  metadata: ToolkitMetadata?
) → Result<Toolkit, DeclarationError>
```

**Preconditions :**
- PRE-1 : Le ToolkitId n'existe pas deja dans le catalogue
- PRE-2 : Le ToolkitId respecte le format canonique
- PRE-3 : Tous les Tools references existent dans le catalogue
- PRE-4 : Tous les Tools references sont au statut Active
- PRE-5 : L'ensemble des Tools n'est pas vide

**Postconditions :**
- POST-1 : Le Toolkit est ajoute au catalogue avec statut Active
- POST-2 : Le niveau de securite est calcule (max des Tools)
- POST-3 : Les index sont mis a jour
- POST-4 : L'historique est mis a jour
- POST-5 : La version du catalogue est incrementee

**Erreurs possibles :**

| Erreur | Description |
|--------|-------------|
| `DuplicateToolkitId` | Un Toolkit avec cet id existe deja |
| `InvalidToolkitId` | Le format de l'id est invalide |
| `ToolNotFound` | Un Tool reference n'existe pas |
| `ToolNotActive` | Un Tool reference n'est pas actif |
| `EmptyToolSet` | L'ensemble des Tools est vide |

### 4.3. Interrogation du Catalogue (QueryTools)

**Enonce :**

L'operation **QueryTools** permet d'interroger le catalogue pour decouvrir les Tools et Toolkits disponibles.

**Modes d'interrogation :**

| Mode | Description | Exemple |
|------|-------------|---------|
| **ById** | Recherche par identifiant | `tool.layout.render` |
| **ByCapability** | Recherche par Capability | `ui.render` |
| **ByDomain** | Recherche par domaine | `ui` |
| **BySecurityLevel** | Recherche par niveau securite | `2` |
| **ByStatus** | Recherche par statut | `Active` |
| **All** | Tous les Tools | - |

**Signature conceptuelle :**

```
QueryTools(
  filter: ToolFilter
) → Result<List<Tool>, QueryError>

ToolFilter {
  id: ToolId?,
  capability_id: CapabilityId?,
  domain: Domain?,
  security_level: SecurityLevel?,
  status: ToolStatus?,
  tags: Set<String>?
}
```

**Preconditions :**
- PRE-1 : Le filtre est valide

**Postconditions :**
- POST-1 : Les Tools correspondant au filtre sont retournes
- POST-2 : Le catalogue n'est pas modifie

**Regles :**
- R-QUERY-1 : L'interrogation est toujours en lecture seule
- R-QUERY-2 : Les filtres peuvent etre combines (AND logique)
- R-QUERY-3 : Une interrogation sans resultat retourne une liste vide

### 4.4. Verification d'Acces (CheckToolAccess)

**Enonce :**

L'operation **CheckToolAccess** permet de verifier si un contexte (Operateur, role) peut acceder a un Tool ou Toolkit.

**Signature conceptuelle :**

```
CheckToolAccess(
  tool_or_toolkit_id: ToolId | ToolkitId,
  context: AccessContext
) → Result<AccessInfo, AccessError>

AccessContext {
  operator_id: OperatorId,
  roles: Set<RoleId>,
  current_security_level: SecurityLevel,
  current_system_state: SystemState
}

AccessInfo {
  can_access: bool,
  required_permissions: Set<PermissionId>,
  required_security_level: SecurityLevel,
  denied_reasons: List<DenialReason>?
}
```

**Preconditions :**
- PRE-1 : Le Tool ou Toolkit existe dans le catalogue
- PRE-2 : Le contexte est valide

**Postconditions :**
- POST-1 : L'information d'acces est retournee
- POST-2 : Le catalogue n'est pas modifie
- POST-3 : Aucune decision n'est prise (information seulement)

**Important :**
- Cette operation **informe** mais **ne decide pas**
- La decision d'autorisation appartient a **StrongFather**
- Master Butler repond : "Voici ce qui est requis pour acceder"

---

## 5. Regles de Souverainete

### 5.1. Bibliotheque Finie et Gouvernee

**Enonce :**

> **Un environnement Miyukini possede une bibliotheque d'outils finie, declaree, gouvernee.**

**Regles de souverainete :**

| Regle | Description | Consequence |
|-------|-------------|-------------|
| **R-SOUV-1** | Pas d'injection sauvage | Aucun Tool ne peut etre ajoute sans declaration dans Master Butler |
| **R-SOUV-2** | Pas de Tool "local" | Tout Tool doit etre declare dans l'environnement |
| **R-SOUV-3** | Pas de dependance externe cachee | Aucune librairie externe non gouvernee |
| **R-SOUV-4** | Declaration obligatoire | Un Tool non declare n'existe pas |
| **R-SOUV-5** | Gouvernance complete | Tout appel passe par la gouvernance |

**Implication :**

> **C'est une souverainete applicative.**

**Invariants :**
- INV-SOUV-1 : Tout Tool appele existe dans le catalogue
- INV-SOUV-2 : Tout appel de Tool est soumis a la gouvernance des Cores
- INV-SOUV-3 : Aucun Tool externe non declare ne peut etre utilise
- INV-SOUV-4 : La bibliotheque est exhaustive et controlee

### 5.2. Flux d'Appel Gouverne

**Enonce :**

Tout appel a un Tool ou Toolkit passe par un flux de gouvernance strict.

**Flux de gouvernance :**

```
Operateur (Strate 7)
    │
    │ 1. Demande d'utilisation d'un Tool
    ▼
┌───────────────────────────────────────┐
│  BondingBrother (mediation)           │
│  - Traduit l'intention                │
│  - Prepare le contexte                │
└───────────────────────────────────────┘
    │
    │ 2. Interrogation du catalogue
    ▼
┌───────────────────────────────────────┐
│  Master Butler                         │
│  - "Ce Tool existe-t-il ?"            │
│  - "Quelles permissions requises ?"   │
│  - "Quel niveau de securite ?"        │
└───────────────────────────────────────┘
    │
    │ 3. Verification securite
    ▼
┌───────────────────────────────────────┐
│  WorrySentinel                         │
│  - "Le niveau de securite permet-il   │
│     cet appel ?"                      │
└───────────────────────────────────────┘
    │
    │ 4. Verification etat systeme
    ▼
┌───────────────────────────────────────┐
│  Caring Nanny                          │
│  - "L'etat systeme permet-il cet      │
│     appel ?"                          │
└───────────────────────────────────────┘
    │
    │ 5. Decision finale
    ▼
┌───────────────────────────────────────┐
│  StrongFather                          │
│  - Evalue l'intention complete        │
│  - Produit la decision ALLOW/DENY     │
└───────────────────────────────────────┘
    │
    │ 6. Execution (si ALLOW)
    ▼
┌───────────────────────────────────────┐
│  Tool / Toolkit (execution)            │
│  - Execute l'action                   │
│  - Retourne le resultat               │
└───────────────────────────────────────┘
```

**Invariants :**
- INV-FLUX-1 : Aucun appel direct aux Tools n'est autorise
- INV-FLUX-2 : Tout appel passe par BondingBrother
- INV-FLUX-3 : Master Butler est interroge pour chaque appel
- INV-FLUX-4 : La decision finale appartient a StrongFather

---

## 6. Ce que Master Butler Fait et Ne Fait Pas

### 6.1. Ce que Master Butler FAIT pour les Tools

| Action | Description |
|--------|-------------|
| **Declare l'existence** | Maintient le catalogue exhaustif des Tools |
| **Lie Capability → Tool** | Etablit la correspondance Capacite-Outil |
| **Definit les permissions** | Declare les permissions d'acces aux Tools |
| **Catalogue les Toolkits** | Maintient l'inventaire des compositions |
| **Informe sur les acces** | Repond aux questions sur les permissions requises |
| **Trace les declarations** | Historise toutes les modifications du catalogue |

### 6.2. Ce que Master Butler NE FAIT PAS pour les Tools

| Action | Pourquoi |
|--------|----------|
| **N'implemente pas** | Master Butler catalogue, n'implemente pas |
| **N'execute pas** | L'execution appartient aux Tools eux-memes |
| **Ne decide pas** | StrongFather decide |
| **Ne gere pas le cycle de vie technique** | Ever Buddy gere le cycle de vie |
| **Ne verifie pas l'etat systeme** | Caring Nanny verifie l'etat |
| **Ne valide pas la securite** | WorrySentinel valide la securite |

### 6.3. Questions auxquelles Master Butler Repond

Master Butler repond a la question fondamentale :

> **"Qu'est-ce qui est possible dans cet environnement ?"**

Pour les Tools, cela se traduit par :

| Question | Reponse de Master Butler |
|----------|--------------------------|
| Quels Tools sont disponibles ? | Liste des Tools declares |
| Quels Toolkits sont declares ? | Liste des Toolkits composes |
| Qui peut appeler quel Tool ? | Permissions declarees par Tool |
| Quelles permissions pour ce Tool ? | Set de PermissionIds requis |
| Quel niveau de securite requis ? | SecurityLevel du Tool |
| Ce Tool existe-t-il ? | Existence dans le catalogue |
| Quelle Capability expose ce Tool ? | CapabilityId liee |

---

## 7. Statuts et Cycle de Vie

### 7.1. Statuts des Tools

**Enonce :**

Tout Tool possede un statut qui reflete son etat dans le cycle de vie.

**Statuts possibles :**

| Statut | Description | Transitions possibles |
|--------|-------------|----------------------|
| **Active** | Tool disponible et utilisable | → Deprecated, → Removed |
| **Deprecated** | Tool obsolete, utilisation deconseillee | → Removed |
| **Removed** | Tool supprime, non utilisable | (terminal) |

**Regles de transition :**
- R-ST-1 : Un Tool nouvellement cree est toujours Active
- R-ST-2 : Un Tool Active peut etre deprecie (→ Deprecated)
- R-ST-3 : Un Tool Deprecated peut etre supprime (→ Removed)
- R-ST-4 : Un Tool Active peut etre supprime directement (→ Removed)
- R-ST-5 : Un Tool Removed ne peut pas etre reactive
- R-ST-6 : Une transition de statut est irreversible

### 7.2. Depreciation d'Outil (DeprecateTool)

**Signature conceptuelle :**

```
DeprecateTool(
  id: ToolId,
  reason: String,
  successor: ToolId?
) → Result<Tool, DeprecationError>
```

**Preconditions :**
- PRE-1 : Le Tool existe dans le catalogue
- PRE-2 : Le Tool a le statut Active
- PRE-3 : La raison de depreciation est fournie
- PRE-4 : Si un successeur est indique, il existe et est Active

**Postconditions :**
- POST-1 : Le Tool passe au statut Deprecated
- POST-2 : La date de depreciation est enregistree
- POST-3 : Tous les Toolkits contenant ce Tool sont notifies
- POST-4 : L'historique est mis a jour

### 7.3. Impact sur les Toolkits

**Enonce :**

Lorsqu'un Tool est deprecie ou supprime, les Toolkits qui le contiennent sont impactes.

**Regles d'impact :**

| Evenement | Impact sur Toolkit |
|-----------|-------------------|
| Tool Deprecated | Toolkit notifie, warning logue |
| Tool Removed | Toolkit invalide si Tool obligatoire |

**Invariants :**
- INV-IMPACT-1 : Un Toolkit ne peut contenir que des Tools actifs ou deprecies
- INV-IMPACT-2 : Un Toolkit avec un Tool supprime devient invalide
- INV-IMPACT-3 : L'invalidation d'un Toolkit est tracee

---

## 8. Invariants Non Negociables

### 8.1. Bibliotheque Gouvernee

**Invariant INV-NN-1 :**

> **Tout Tool existant dans l'environnement est catalogue dans Master Butler.**

**Implications :**
- Aucun Tool "cache" ou non declare
- Aucun contournement du catalogue
- Master Butler est la source de verite unique pour les Tools

### 8.2. Non-Decision

**Invariant INV-NN-2 :**

> **Master Butler informe sur les Tools mais ne decide jamais de leur utilisation.**

**Implications :**
- Master Butler repond "ce Tool existe et requiert ces permissions"
- Master Butler ne repond jamais "cet appel est autorise"
- La decision appartient exclusivement a StrongFather

### 8.3. Liaison Unique

**Invariant INV-NN-3 :**

> **Une Capability est liee a au plus un Tool. Un Tool expose exactement une Capability.**

**Implications :**
- Pas de duplication de capacite
- Correspondance 1:1 stricte
- Tracabilite complete

### 8.4. Composition Sans Capacite Nouvelle

**Invariant INV-NN-4 :**

> **Un Toolkit n'ajoute aucune capacite nouvelle, il compose des Tools existants.**

**Implications :**
- Toolkit = orchestration pure
- Pas de logique metier dans les Toolkits
- Les capacites viennent exclusivement des Tools

### 8.5. Souverainete Applicative

**Invariant INV-NN-5 :**

> **Aucun Tool externe non declare ne peut etre utilise dans l'environnement.**

**Implications :**
- Bibliotheque finie et controlee
- Pas d'injection dynamique
- Pas de dependance cachee

---

## 9. Schemas ASCII

### 9.1. Structure du Catalogue

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          TOOL CATALOG                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  INDEX DES TOOLS (Map<ToolId, Tool>)                                  │  │
│  ├──────────────────────────────────────────────────────────────────────┤  │
│  │  tool.layout.render    → Tool { cap: "ui.render", sec: 1, ... }      │  │
│  │  tool.input.capture    → Tool { cap: "ui.capture", sec: 1, ... }     │  │
│  │  tool.form.validate    → Tool { cap: "form.validate", sec: 2, ... }  │  │
│  │  tool.query.execute    → Tool { cap: "data.query", sec: 2, ... }     │  │
│  │  tool.file.read        → Tool { cap: "io.read", sec: 3, ... }        │  │
│  │  tool.cache.get        → Tool { cap: "cache.read", sec: 1, ... }     │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  INDEX DES TOOLKITS (Map<ToolkitId, Toolkit>)                         │  │
│  ├──────────────────────────────────────────────────────────────────────┤  │
│  │  toolkit.ui.standard → Toolkit {                                      │  │
│  │                          tools: [tool.layout.render,                  │  │
│  │                                  tool.input.capture,                  │  │
│  │                                  tool.form.validate],                 │  │
│  │                          sec: 2                                       │  │
│  │                        }                                              │  │
│  │                                                                        │  │
│  │  toolkit.data.crud   → Toolkit {                                      │  │
│  │                          tools: [tool.query.execute,                  │  │
│  │                                  tool.cache.get],                     │  │
│  │                          sec: 2                                       │  │
│  │                        }                                              │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  ┌────────────────────────────┐    ┌────────────────────────────────────┐  │
│  │  INDEX PAR CAPABILITY      │    │  INDEX PAR DOMAINE                  │  │
│  ├────────────────────────────┤    ├────────────────────────────────────┤  │
│  │  ui.render → tool.layout.  │    │  ui:                                │  │
│  │              render        │    │    - tool.layout.render             │  │
│  │  ui.capture → tool.input.  │    │    - tool.input.capture             │  │
│  │               capture      │    │                                      │  │
│  │  form.validate → tool.form │    │  data:                               │  │
│  │                 .validate  │    │    - tool.query.execute              │  │
│  │  ...                       │    │    - tool.cache.get                  │  │
│  └────────────────────────────┘    └────────────────────────────────────┘  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 9.2. Flux de Declaration d'Outil

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     FLUX DE DECLARATION D'OUTIL                              │
└─────────────────────────────────────────────────────────────────────────────┘

   COMPOSANT                    MASTER BUTLER                    CATALOGUE
       │                              │                              │
       │  DeclareTool(                │                              │
       │    id: "tool.layout.render", │                              │
       │    capability: "ui.render",  │                              │
       │    security_level: 1         │                              │
       │  )                           │                              │
       ├─────────────────────────────►│                              │
       │                              │                              │
       │                              │  1. Valider format id        │
       │                              │  2. Verifier unicite         │
       │                              │  3. Verifier Capability      │
       │                              │     existe                   │
       │                              │  4. Verifier Capability      │
       │                              │     non liee                 │
       │                              │  5. Valider security level   │
       │                              │                              │
       │                              │  [Validations OK]            │
       │                              │                              │
       │                              │  6. Creer Tool               │
       │                              ├─────────────────────────────►│
       │                              │                              │
       │                              │  7. Creer liaison            │
       │                              │     Capability-Tool          │
       │                              ├─────────────────────────────►│
       │                              │                              │
       │                              │  8. Mettre a jour index      │
       │                              ├─────────────────────────────►│
       │                              │                              │
       │                              │  9. Historiser evenement     │
       │                              ├─────────────────────────────►│
       │                              │                              │
       │                              │◄─────────────────────────────┤
       │                              │      [Tool cree]             │
       │◄─────────────────────────────┤                              │
       │    Result::Ok(Tool)          │                              │
       │                              │                              │
       ▼                              ▼                              ▼
```

### 9.3. Separation des Responsabilites

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                 SEPARATION DES RESPONSABILITES - TOOLS                       │
└─────────────────────────────────────────────────────────────────────────────┘

                     ┌─────────────────────────────┐
                     │      MASTER BUTLER          │
                     │   Capability & Permission   │
                     │           Core              │
                     ├─────────────────────────────┤
                     │  ✓ Declarer les Tools       │
                     │  ✓ Lier Capability → Tool   │
                     │  ✓ Definir permissions      │
                     │  ✓ Cataloguer Toolkits      │
                     │  ✗ NE DECIDE PAS            │
                     │  ✗ N'EXECUTE PAS            │
                     │  ✗ N'IMPLEMENTE PAS         │
                     └─────────────────────────────┘
                                   │
           ┌───────────────────────┼───────────────────────┐
           │                       │                       │
           ▼                       ▼                       ▼
┌─────────────────────┐ ┌─────────────────────┐ ┌─────────────────────┐
│   STRONGFATHER      │ │    EVER BUDDY       │ │   WORRYSENSTINEL    │
│  Decision Core      │ │  Lifecycle Core     │ │   Security Core     │
├─────────────────────┤ ├─────────────────────┤ ├─────────────────────┤
│ ✓ Decide ALLOW/DENY │ │ ✓ Gere versions     │ │ ✓ Valide securite   │
│ ✓ Evalue intentions │ │ ✓ Deprecie Tools    │ │ ✓ Verifie niveau    │
│ ✗ Ne catalogue pas  │ │ ✓ Migre Tools       │ │ ✗ Ne catalogue pas  │
└─────────────────────┘ └─────────────────────┘ └─────────────────────┘
                                   │
                                   ▼
                     ┌─────────────────────────────┐
                     │         TOOLS               │
                     │   Capacites Executables     │
                     ├─────────────────────────────┤
                     │  ✓ Executent les actions    │
                     │  ✓ Retournent resultats     │
                     │  ✗ NE DECIDENT JAMAIS       │
                     │  ✗ NE CONNAISSENT PAS       │
                     │    L'OPERATEUR              │
                     └─────────────────────────────┘
```

---

## 10. Exemples Concrets

### 10.1. Exemple : Declaration de Tools UI

**Contexte :**
Un module UI declare ses Tools au demarrage.

**Declarations :**

```
// Outil de rendu de layout
DeclareTool(
  id: "tool.layout.render",
  name: "Rendu de Layout",
  description: "Rend un layout selon une specification",
  capability_id: "ui.render",
  source: SourceIdentity {
    type: Tool,
    identifier: "module.ui.layout",
    version: "1.0.0"
  },
  security_level: 1,
  metadata: {
    tags: ["ui", "render", "layout"],
    execution_mode: Sync,
    timeout_ms: 5000
  }
)

// Outil de capture de saisie
DeclareTool(
  id: "tool.input.capture",
  name: "Capture de Saisie",
  description: "Capture une saisie utilisateur",
  capability_id: "ui.capture",
  source: SourceIdentity {
    type: Tool,
    identifier: "module.ui.input",
    version: "1.0.0"
  },
  security_level: 1,
  metadata: {
    tags: ["ui", "input", "capture"],
    execution_mode: Async
  }
)

// Outil de validation de formulaire
DeclareTool(
  id: "tool.form.validate",
  name: "Validation de Formulaire",
  description: "Valide un formulaire selon un schema",
  capability_id: "form.validate",
  source: SourceIdentity {
    type: Tool,
    identifier: "module.ui.form",
    version: "1.0.0"
  },
  security_level: 2,
  metadata: {
    tags: ["ui", "form", "validate"],
    input_schema: FormSchema,
    output_schema: ValidationResult
  }
)
```

### 10.2. Exemple : Declaration d'un Toolkit

**Contexte :**
Un Toolkit UI Standard est declare composant les Tools precedents.

**Declaration :**

```
DeclareToolkit(
  id: "toolkit.ui.standard",
  name: "Kit d'Outils UI Standard",
  description: "Composition standard des outils UI",
  tools: [
    "tool.layout.render",
    "tool.input.capture",
    "tool.form.validate"
  ],
  allowed_states: [SystemState::Healthy, SystemState::Degraded],
  disallowed_states: [SystemState::SecurityLockdown, SystemState::Maintenance],
  metadata: {
    tags: ["ui", "standard", "composition"],
    documentation_url: "https://docs.miyukini.dev/toolkits/ui-standard"
  }
)
```

**Resultat :**

```
Toolkit {
  id: "toolkit.ui.standard",
  name: "Kit d'Outils UI Standard",
  tools: ["tool.layout.render", "tool.input.capture", "tool.form.validate"],
  security_level: 2,  // max(1, 1, 2) = 2
  allowed_states: [Healthy, Degraded],
  disallowed_states: [SecurityLockdown, Maintenance],
  status: Active
}
```

### 10.3. Exemple : Interrogation du Catalogue

**Contexte :**
StrongFather interroge Master Butler lors de l'evaluation d'une intention.

**Interrogation :**

```
// Question : Ce Tool existe-t-il ?
QueryTools(filter: { id: "tool.layout.render" })
→ Result::Ok([
    Tool {
      id: "tool.layout.render",
      capability_id: "ui.render",
      security_level: 1,
      status: Active,
      ...
    }
  ])

// Question : Quel niveau de securite requis ?
CheckToolAccess(
  tool_id: "tool.form.validate",
  context: AccessContext { ... }
)
→ Result::Ok(AccessInfo {
    can_access: true,  // Information, pas decision
    required_permissions: ["form.validate.execute"],
    required_security_level: 2,
    denied_reasons: None
  })
```

---

## 11. Conclusion

Ce contrat etablit le modele technique de gouvernance des Tools et Toolkits dans Master Butler, definissant de maniere absolue :

**Points cles :**
- **Tool :** Capacite executable, sans autorite, gouvernee par les Cores
- **ToolId :** Format canonique `tool.<domain>.<action>[.<qualifier>]`
- **Toolkit :** Composition officielle de Tools, sans capacite nouvelle
- **Catalogue :** Structure exhaustive, coherente, tracee des Tools et Toolkits
- **Liaison :** Correspondance 1:1 entre Capability et Tool
- **Souverainete :** Bibliotheque finie, declaree, gouvernee

**Responsabilites de Master Butler :**
- Declarer l'existence des Tools
- Lier les Capabilities aux Tools
- Definir les permissions d'acces
- Cataloguer les Toolkits
- Informer sur les acces (sans decider)

**Invariants non negociables :**
- Bibliotheque gouvernee et exhaustive
- Non-decision (Master Butler informe, ne decide pas)
- Liaison unique Capability-Tool
- Composition sans capacite nouvelle
- Souverainete applicative

Ce contrat complete la Documentation Fondatrice de Master Butler (section 10) en definissant le modele technique de gouvernance des Tools et Toolkits.

**Non-negociabilite :** Ce contrat est absolu et non negociable. Le contrat prime sur toute consideration pratique.

---

**Document cree le :** 2026-01-27  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif valide  
**Reference :** Miyukini Core System v2.4, Master Butler Documentation Fondatrice, Miyukini Conceptual References - Tools et Toolkits  
**Type :** Contrat de gouvernance non negociable

---

## 12. Mini log — erreurs / warnings / ambiguites rencontrees et corrigees

### Ambiguite A1 : Format de l'identifiant de Tool

**Ambiguite rencontree :**
La reference Tools et Toolkits donne des exemples d'identifiants (`layout.render`, `input.capture`) sans prefixe `tool.`.

**Decision prise :**
Format canonique avec prefixe obligatoire : `tool.<domain>.<action>[.<qualifier>]` pour distinguer clairement les Tools des autres entites (Capabilities, Permissions).

**Justification :**
Le prefixe `tool.` evite toute confusion avec les CapabilityIds et permet une identification immediate du type d'entite.

**Correction effectuee :**
Section 2.2 "Identifiant d'Outil" avec format canonique et regles de nommage (R-TID-1 a R-TID-6).

### Ambiguite A2 : Niveau de securite des Toolkits

**Ambiguite rencontree :**
La reference Tools et Toolkits mentionne un `security_level` pour les Toolkits sans preciser comment il est determine.

**Decision prise :**
Le niveau de securite d'un Toolkit est le maximum des niveaux de securite de ses Tools composants.

**Justification :**
Le Toolkit doit avoir un niveau de securite au moins aussi restrictif que son Tool le plus sensible pour garantir la coherence securitaire.

**Correction effectuee :**
Invariant INV-TK-6 et postcondition POST-2 de DeclareToolkit.

### Ambiguite A3 : Relation Master Butler / StrongFather pour les Tools

**Ambiguite rencontree :**
La separation entre "informer" (Master Butler) et "decider" (StrongFather) n'etait pas explicite pour les operations sur les Tools.

**Decision prise :**
L'operation `CheckToolAccess` retourne des informations (`can_access` est informatif, pas decisif) et la decision finale appartient exclusivement a StrongFather.

**Justification :**
Coherence avec l'invariant INV-MB-2 (Non-decision) de la Documentation Fondatrice.

**Correction effectuee :**
Section 4.4 avec note "Cette operation **informe** mais **ne decide pas**" et section 6 detaillant ce que Master Butler fait et ne fait pas.

---

*Aucune autre erreur, warning, ou ambiguite rencontree lors de la redaction de ce document.*
