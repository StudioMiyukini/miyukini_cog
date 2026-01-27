# Master Butler — Toolkit Composition Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **Master Butler Toolkit Composition Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit les règles de composition des Kits d'Outils (Toolkits) dans le système Miyukini Core System v2.4.

Ce contrat définit :
- La structure formelle d'un Toolkit
- Les règles de composition (quels Tools peuvent être regroupés)
- Les métadonnées obligatoires et optionnelles
- Les invariants de composition
- Les opérations autorisées sur les Toolkits
- Les contraintes de gouvernance

### Portée

Ce contrat s'applique à **toute instance de Master Butler** et définit de manière absolue :
- La définition formelle d'un Toolkit
- Le modèle de composition (Toolkit → Tools)
- Les règles de déclaration et de validation
- Les métadonnées des Toolkits
- Les contraintes de sécurité et d'état
- Les invariants de composition
- Les opérations autorisées et interdites

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues que Master Butler applique sans exception. Ces règles ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et s'articule avec les documents contractuels existants :

- **[Master Butler - Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md)** : Définit la nature, le rôle, et les responsabilités de Master Butler
- **Master Butler - Tool Governance Contract** : Définit la gouvernance des Tools individuels (contrat complémentaire)
- **[Master Butler - Capability Registry Contract](../registry/Master%20Butler%20-%20Capability%20Registry%20Contract.md)** : Définit le registre des capacités
- **[Miyukini Conceptual References - Tools et Toolkits](../../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)** : Définit les concepts canoniques de Tool et Toolkit
- **[Miyukini Framework - Lois Autonomie Systeme](../../../../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md)** : Ce contrat respecte **LOI-1** (aucune dépendance externe critique) et **LOI-5** (coût proportionnel au hardware) en garantissant que la composition est locale, légère, et autonome.

**Complémentarité :**
- Master Butler Documentation Fondatrice = définition conceptuelle et philosophique
- Master Butler Tool Governance Contract = gouvernance des Tools individuels
- Master Butler Toolkit Composition Contract = modèle de composition des Toolkits

Ces contrats forment ensemble le système complet de catalogage et gouvernance des outils du système Miyukini Core System v2.4.

---

## 2. Doctrine Fondamentale

### Énoncé canonique

> **Les Kits d'Outils (Toolkits) sont des compositions officielles d'outils, optimisées pour l'efficience mais jamais pour l'autorité.**

### Règle fondamentale

> **👉 Un Kit d'Outils orchestre, mais n'ajoute pas de capacité.**

### Implications directes

| Règle | Description |
|-------|-------------|
| **Composition uniquement** | Un Toolkit agrège des Tools existants, il n'en crée pas |
| **Sans logique métier** | Aucune logique business dans un Toolkit |
| **Sans décision** | Un Toolkit ne décide jamais |
| **Sans capacité nouvelle** | Un Toolkit ne possède pas de capacité qui n'existe pas dans ses Tools |
| **Optimisation pure** | Un Toolkit optimise les appels, normalise les flux |

### Ce qu'un Toolkit N'EST PAS

| ❌ N'est pas | Pourquoi |
|--------------|----------|
| Un nouvel Outil | Il ne crée pas de capacité nouvelle |
| Un service | Il n'a pas de logique propre |
| Un décideur | Il n'a pas d'autorité |
| Une librairie libre | Il est gouverné |
| Un raccourci non gouverné | Il passe par la même gouvernance |

---

## 3. Définitions Formelles

### 3.1. Kit d'Outils (Toolkit)

**Définition formelle :**

Un **Toolkit** est une composition officielle de Tools, validée et déclarée par l'environnement, optimisée pour efficience, cohérence et performance. Il regroupe des Tools existants sans ajouter de capacité nouvelle.

**Caractéristiques formelles :**

- **Identité unique :** Chaque Toolkit possède un identifiant unique et immuable (ToolkitId)
- **Composition :** Le Toolkit est composé d'un ou plusieurs Tools existants
- **Officiel :** Le Toolkit est déclaré et validé par l'environnement
- **Gouverné :** Le Toolkit est soumis à la gouvernance des Cores
- **Sans capacité nouvelle :** Le Toolkit n'expose que les capacités de ses Tools constituants
- **Optimisé :** Le Toolkit optimise les appels pour efficience et performance

**Structure formelle d'un Toolkit :**

```
Toolkit {
  id: ToolkitId,               // Identifiant unique et immuable
  name: String,                // Nom lisible humain
  description: String,         // Description du Toolkit
  version: Version,            // Version du Toolkit
  tools: Set<ToolId>,          // Ensemble des Tools composant le Toolkit
  security_level: SecurityLevel, // Niveau de sécurité requis
  allowed_states: Set<SystemState>, // États système autorisés
  disallowed_states: Set<SystemState>, // États système interdits
  metadata: ToolkitMetadata,   // Métadonnées additionnelles
  created_at: Timestamp,       // Date de création
  status: ToolkitStatus        // Statut (Active, Deprecated, Removed)
}
```

**Invariants :**
- INV-TK-1 : Tout Toolkit possède un ToolkitId unique et immuable
- INV-TK-2 : Tout Toolkit contient au moins deux Tools
- INV-TK-3 : Tous les Tools d'un Toolkit existent dans le registre
- INV-TK-4 : Un Toolkit ne déclare aucune capacité propre
- INV-TK-5 : Un Toolkit n'ajoute aucune logique métier

### 3.2. Identifiant de Toolkit (ToolkitId)

**Définition formelle :**

Un **ToolkitId** est l'identifiant unique et immuable d'un Toolkit dans le registre. Il suit un format canonique qui encode le domaine et la fonction.

**Format canonique :**

```
<domain>.<function>
```

**Exemples :**
- `ui.standard` : Kit d'Outils UI Standard
- `data.crud` : Kit d'Outils CRUD de données
- `media.management` : Kit d'Outils de gestion des médias
- `content.publishing` : Kit d'Outils de publication de contenu
- `search.full` : Kit d'Outils de recherche complet

**Règles de nommage :**
- R-TKID-1 : Le format est strictement `<domain>.<function>`
- R-TKID-2 : Tous les segments sont en minuscules, sans accents
- R-TKID-3 : Les segments sont séparés par des points (.)
- R-TKID-4 : Chaque segment contient uniquement des lettres, chiffres, et underscores
- R-TKID-5 : L'identifiant est unique dans l'ensemble du système

**Invariants :**
- INV-TKID-1 : Tout ToolkitId respecte le format canonique
- INV-TKID-2 : Tout ToolkitId est unique dans le registre
- INV-TKID-3 : Un ToolkitId ne peut jamais être modifié après création

### 3.3. Identifiant de Tool (ToolId)

**Définition formelle :**

Un **ToolId** identifie un Tool individuel qui peut être inclus dans un Toolkit.

**Format canonique :**

```
<domain>.<action>
```

**Exemples :**
- `layout.render` : Rend un layout
- `input.capture` : Capture une saisie utilisateur
- `form.validate` : Valide un formulaire
- `theme.resolve` : Résout un thème
- `event.dispatch` : Dispatch un événement
- `query.execute` : Exécute une requête
- `cache.get` : Récupère depuis le cache
- `file.read` : Lit un fichier
- `file.write` : Écrit un fichier

**Invariants :**
- INV-TOOL-1 : Tout ToolId référencé dans un Toolkit existe dans le registre des capacités
- INV-TOOL-2 : Un Tool peut appartenir à plusieurs Toolkits

### 3.4. Niveau de Sécurité (SecurityLevel)

**Définition formelle :**

Le **SecurityLevel** d'un Toolkit définit le niveau de sécurité minimum requis pour utiliser ce Toolkit.

**Niveaux définis :**

| Niveau | Description | Contexte |
|--------|-------------|----------|
| 0 | Aucune restriction | Opérations publiques |
| 1 | Authentification requise | Opérations utilisateur de base |
| 2 | Authentification + rôle | Opérations nécessitant des droits |
| 3 | Authentification + rôle élevé | Opérations sensibles |
| 4 | Maximum | Opérations critiques |

**Règle de calcul :**
- R-SEC-1 : Le SecurityLevel d'un Toolkit est au minimum égal au plus haut SecurityLevel de ses Tools
- R-SEC-2 : Le SecurityLevel d'un Toolkit peut être supérieur à celui de ses Tools (restriction supplémentaire)
- R-SEC-3 : Le SecurityLevel d'un Toolkit ne peut jamais être inférieur à celui d'un de ses Tools

**Invariants :**
- INV-SEC-1 : Le SecurityLevel d'un Toolkit ≥ max(SecurityLevel de chaque Tool)
- INV-SEC-2 : WorrySentinel valide le SecurityLevel avant utilisation

### 3.5. États Système (SystemState)

**Définition formelle :**

Les **SystemStates** définissent les états dans lesquels un Toolkit peut ou ne peut pas être utilisé.

**États système standard :**

| État | Description | Toolkit généralement |
|------|-------------|---------------------|
| HEALTHY | Système en fonctionnement normal | Autorisé |
| DEGRADED | Système en mode dégradé | Autorisé (selon config) |
| MAINTENANCE | Système en maintenance | Généralement interdit |
| SECURITY_LOCKDOWN | Verrouillage sécurité | Interdit |
| OFFLINE | Système hors ligne | Selon conception |

**Règles :**
- R-STATE-1 : Un Toolkit définit explicitement ses états autorisés
- R-STATE-2 : Un Toolkit définit explicitement ses états interdits
- R-STATE-3 : Caring Nanny vérifie l'état système avant autorisation

**Invariants :**
- INV-STATE-1 : allowed_states ∩ disallowed_states = ∅ (pas d'intersection)
- INV-STATE-2 : Un Toolkit sans allowed_states explicites est autorisé dans tous les états non interdits

---

## 4. Modèle de Composition

### 4.1. Principe de Composition

**Énoncé :**

Un Toolkit est une **agrégation formelle** de Tools existants. La composition ne crée aucune fonctionnalité nouvelle, elle optimise l'accès à des fonctionnalités existantes.

**Schéma de composition :**

```
Toolkit (composition)
 ├─ Tool A (capacité atomique)
 ├─ Tool B (capacité atomique)
 ├─ Tool C (capacité atomique)
 └─ Tool D (capacité atomique)

Capacités exposées par le Toolkit = ∪ (Capacités de A, B, C, D)
Logique ajoutée par le Toolkit = ∅ (ensemble vide)
```

**Règles de composition :**
- R-COMP-1 : Un Toolkit contient au minimum 2 Tools
- R-COMP-2 : Un Toolkit ne peut contenir que des Tools existants et actifs
- R-COMP-3 : Un Toolkit n'ajoute aucune logique entre les Tools
- R-COMP-4 : L'ordre des Tools dans un Toolkit n'a pas de signification sémantique
- R-COMP-5 : Un Tool peut appartenir à plusieurs Toolkits

**Invariants :**
- INV-COMP-1 : |tools| ≥ 2 (au moins deux Tools)
- INV-COMP-2 : ∀ tool ∈ tools : exists(tool) ∧ status(tool) ≠ Removed
- INV-COMP-3 : capabilities(Toolkit) = ∪ capabilities(tool) pour tool ∈ tools

### 4.2. Cohérence de Composition

**Énoncé :**

Les Tools composant un Toolkit doivent être cohérents entre eux. La composition doit avoir un sens fonctionnel.

**Critères de cohérence :**

| Critère | Description | Exemple valide |
|---------|-------------|----------------|
| Domaine commun | Tools du même domaine fonctionnel | UI: layout.render + input.capture |
| Flux complémentaire | Tools qui s'utilisent ensemble | CRUD: query.execute + cache.get |
| Optimisation groupée | Tools souvent appelés ensemble | Media: media.upload + media.validate |

**Contre-exemples (compositions non cohérentes) :**

| ❌ Interdit | Raison |
|-------------|--------|
| auth.login + layout.render | Domaines non liés |
| file.read seul | Un seul Tool |
| Tools dépréciés | Statut invalide |

**Règles de cohérence :**
- R-COH-1 : Les Tools d'un Toolkit appartiennent généralement au même domaine
- R-COH-2 : La composition doit avoir une justification fonctionnelle documentée
- R-COH-3 : Un Toolkit ne regroupe pas de Tools sans lien fonctionnel

### 4.3. Ce que la Composition Apporte

**Énoncé :**

La composition en Toolkit apporte des bénéfices d'efficience sans modifier les capacités.

**Bénéfices de la composition :**

| Bénéfice | Description |
|----------|-------------|
| **Optimisation des appels** | Réduction des allers-retours de gouvernance |
| **Normalisation des flux** | Standardisation des patterns d'appel |
| **Cohérence garantie** | Ensemble de Tools validé comme fonctionnel |
| **Documentation groupée** | Point d'entrée unique pour un ensemble cohérent |
| **Gouvernance simplifiée** | Une seule vérification pour plusieurs Tools |

**Ce que la composition N'apporte PAS :**

| ❌ N'apporte pas | Raison |
|-----------------|--------|
| Nouvelles capacités | Le Toolkit ne crée pas |
| Logique métier | Le Toolkit ne décide pas |
| Autorité | Le Toolkit n'autorise pas |
| Optimisation forcée | L'Opérateur reste libre d'appeler les Tools individuellement |

---

## 5. Métadonnées des Toolkits

### 5.1. Métadonnées Obligatoires

**Énoncé :**

Tout Toolkit déclaré doit fournir un ensemble minimal de métadonnées obligatoires.

**Métadonnées obligatoires :**

| Métadonnée | Type | Description |
|------------|------|-------------|
| `id` | ToolkitId | Identifiant unique et immuable |
| `name` | String | Nom lisible humain (max 100 caractères) |
| `description` | String | Description du Toolkit (max 500 caractères) |
| `version` | Version | Version du Toolkit (semver) |
| `tools` | Set<ToolId> | Ensemble des Tools (min 2) |
| `security_level` | SecurityLevel | Niveau de sécurité requis (0-4) |
| `allowed_states` | Set<SystemState> | États système autorisés |
| `created_at` | Timestamp | Date et heure de création |

**Invariants :**
- INV-META-1 : Toutes les métadonnées obligatoires sont présentes
- INV-META-2 : Les métadonnées obligatoires ne peuvent pas être nulles ou vides
- INV-META-3 : |tools| ≥ 2

### 5.2. Métadonnées Optionnelles

**Énoncé :**

Les Toolkits peuvent fournir des métadonnées additionnelles pour enrichir la documentation et la découverte.

**Métadonnées optionnelles :**

| Métadonnée | Type | Description |
|------------|------|-------------|
| `disallowed_states` | Set<SystemState> | États système interdits |
| `tags` | Set<String> | Tags de classification |
| `documentation_url` | URL | Lien vers la documentation |
| `examples` | List<String> | Exemples d'utilisation |
| `deprecated_at` | Timestamp | Date de dépréciation (si applicable) |
| `deprecation_reason` | String | Raison de la dépréciation |
| `successor` | ToolkitId | Toolkit de remplacement (si déprécié) |
| `custom` | Map<String, Any> | Métadonnées personnalisées |

**Règles :**
- R-OPTMETA-1 : Les métadonnées optionnelles peuvent être nulles ou absentes
- R-OPTMETA-2 : Les tags sont normalisés (minuscules, sans accents)
- R-OPTMETA-3 : L'URL de documentation doit être valide si fournie

### 5.3. Statut de Toolkit

**Énoncé :**

Tout Toolkit possède un statut qui reflète son état dans le cycle de vie.

**Statuts possibles :**

| Statut | Description | Transitions possibles |
|--------|-------------|----------------------|
| **Active** | Toolkit disponible et utilisable | → Deprecated, → Removed |
| **Deprecated** | Toolkit obsolète, utilisation déconseillée | → Removed |
| **Removed** | Toolkit supprimé, non utilisable | (terminal) |

**Règles de transition :**
- R-TKST-1 : Un Toolkit nouvellement créé est toujours Active
- R-TKST-2 : Un Toolkit Active peut être déprécié (→ Deprecated)
- R-TKST-3 : Un Toolkit Deprecated peut être supprimé (→ Removed)
- R-TKST-4 : Un Toolkit Active peut être supprimé directement (→ Removed)
- R-TKST-5 : Un Toolkit Removed ne peut pas être réactivé
- R-TKST-6 : Une transition de statut est irréversible

**Impact de la dépréciation d'un Tool :**
- R-TKTOOL-1 : Si un Tool est déprécié, les Toolkits qui le contiennent reçoivent un avertissement
- R-TKTOOL-2 : Si un Tool est supprimé, les Toolkits qui le contiennent deviennent invalides
- R-TKTOOL-3 : Un Toolkit invalide doit être mis à jour ou supprimé

---

## 6. Opérations sur les Toolkits

### 6.1. Déclaration de Toolkit (DeclareToolkit)

**Énoncé :**

L'opération **DeclareToolkit** permet de déclarer un nouveau Toolkit dans le registre.

**Signature conceptuelle :**

```
DeclareToolkit(
  id: ToolkitId,
  name: String,
  description: String,
  version: Version,
  tools: Set<ToolId>,
  security_level: SecurityLevel,
  allowed_states: Set<SystemState>,
  metadata: ToolkitMetadata?
) → Result<Toolkit, DeclarationError>
```

**Préconditions :**
- PRE-1 : L'identifiant n'existe pas déjà dans le registre
- PRE-2 : L'identifiant respecte le format canonique
- PRE-3 : Les métadonnées obligatoires sont fournies et valides
- PRE-4 : |tools| ≥ 2
- PRE-5 : Tous les Tools existent et sont actifs
- PRE-6 : security_level ≥ max(security_level de chaque Tool)

**Postconditions :**
- POST-1 : Le Toolkit est ajouté au registre avec statut Active
- POST-2 : Les index sont mis à jour
- POST-3 : L'historique est mis à jour avec l'événement de création
- POST-4 : La version du registre est incrémentée

**Erreurs possibles :**

| Erreur | Condition |
|--------|-----------|
| `InvalidToolkitId` | Format de l'identifiant invalide |
| `DuplicateToolkitId` | Identifiant déjà existant |
| `InsufficientTools` | Moins de 2 Tools |
| `ToolNotFound` | Un Tool n'existe pas |
| `ToolNotActive` | Un Tool n'est pas actif |
| `SecurityLevelTooLow` | security_level < max des Tools |
| `MissingMetadata` | Métadonnée obligatoire manquante |

### 6.2. Interrogation de Toolkit (QueryToolkit)

**Énoncé :**

L'opération **QueryToolkit** permet d'interroger le registre pour obtenir les informations sur un ou plusieurs Toolkits.

**Modes d'interrogation :**

| Mode | Description | Exemple |
|------|-------------|---------|
| **ById** | Recherche par identifiant exact | `ui.standard` |
| **ByTool** | Toolkits contenant un Tool | `layout.render` |
| **BySecurityLevel** | Toolkits d'un niveau de sécurité | `2` |
| **ByStatus** | Recherche par statut | `Active` |
| **ByTags** | Recherche par tags | `["ui", "standard"]` |
| **All** | Tous les Toolkits | - |

**Signature conceptuelle :**

```
QueryToolkit(
  filter: ToolkitFilter
) → Result<List<Toolkit>, QueryError>

ToolkitFilter {
  id: ToolkitId?,
  contains_tool: ToolId?,
  security_level: SecurityLevel?,
  status: ToolkitStatus?,
  tags: Set<String>?
}
```

**Règles :**
- R-QUERY-1 : L'interrogation est toujours en lecture seule
- R-QUERY-2 : Les filtres peuvent être combinés (AND logique)
- R-QUERY-3 : Une interrogation sans résultat retourne une liste vide

### 6.3. Mise à Jour de Toolkit (UpdateToolkit)

**Énoncé :**

L'opération **UpdateToolkit** permet de modifier un Toolkit existant.

**Modifications autorisées :**

| Modification | Autorisée | Condition |
|--------------|-----------|-----------|
| Ajouter un Tool | ✅ Oui | Tool existe et est actif |
| Retirer un Tool | ✅ Oui | |tools| reste ≥ 2 |
| Modifier metadata | ✅ Oui | Validité préservée |
| Modifier security_level | ✅ Oui | Respecte R-SEC-1 à R-SEC-3 |
| Modifier allowed_states | ✅ Oui | Respecte INV-STATE-1 |
| Modifier id | ❌ Non | Immuable |
| Modifier created_at | ❌ Non | Immuable |

**Signature conceptuelle :**

```
UpdateToolkit(
  id: ToolkitId,
  changes: ToolkitChanges
) → Result<Toolkit, UpdateError>

ToolkitChanges {
  add_tools: Set<ToolId>?,
  remove_tools: Set<ToolId>?,
  security_level: SecurityLevel?,
  allowed_states: Set<SystemState>?,
  disallowed_states: Set<SystemState>?,
  metadata: ToolkitMetadata?
}
```

**Règles :**
- R-UPD-1 : La mise à jour incrémente la version mineure du Toolkit
- R-UPD-2 : L'historique enregistre les changements
- R-UPD-3 : Les invariants sont vérifiés après modification

### 6.4. Dépréciation de Toolkit (DeprecateToolkit)

**Énoncé :**

L'opération **DeprecateToolkit** permet de marquer un Toolkit comme obsolète.

**Signature conceptuelle :**

```
DeprecateToolkit(
  id: ToolkitId,
  reason: String,
  successor: ToolkitId?
) → Result<Toolkit, DeprecationError>
```

**Préconditions :**
- PRE-1 : Le Toolkit existe dans le registre
- PRE-2 : Le Toolkit a le statut Active
- PRE-3 : La raison de dépréciation est fournie
- PRE-4 : Si un successeur est indiqué, il existe et est Active

**Règles :**
- R-DEP-1 : Un Toolkit déprécié reste interrogeable
- R-DEP-2 : Un Toolkit déprécié ne peut pas être redéclaré
- R-DEP-3 : La dépréciation est irréversible

### 6.5. Suppression de Toolkit (RemoveToolkit)

**Énoncé :**

L'opération **RemoveToolkit** permet de supprimer définitivement un Toolkit du registre actif.

**Signature conceptuelle :**

```
RemoveToolkit(
  id: ToolkitId,
  reason: String
) → Result<(), RemovalError>
```

**Préconditions :**
- PRE-1 : Le Toolkit existe dans le registre
- PRE-2 : Le Toolkit a le statut Active ou Deprecated

**Postconditions :**
- POST-1 : Le Toolkit passe au statut Removed
- POST-2 : Le Toolkit n'apparaît plus dans les interrogations standard
- POST-3 : L'historique conserve la trace du Toolkit
- POST-4 : L'identifiant est réservé (non réutilisable)

---

## 7. Flux de Gouvernance

### 7.1. Flux d'Appel d'un Toolkit

**Énoncé :**

L'appel d'un Toolkit suit le même flux de gouvernance que l'appel d'un Tool individuel.

**Séquence :**

```
Opérateur (Strate 7)
    │
    │ "Je veux utiliser le Kit d'Outils UI"
    ▼
┌───────────────────────────────────────┐
│  BondingBrother (médiation)           │
│  Traduit la requête                   │
└───────────────────────────────────────┘
    │
    ▼
┌───────────────────────────────────────┐
│  Master Butler                        │
│  "Ce Toolkit existe-t-il ?"           │
│  "Quels Tools le composent ?"         │
│  "Quelles permissions sont requises ?"│
└───────────────────────────────────────┘
    │
    ▼
┌───────────────────────────────────────┐
│  StrongFather                         │
│  "L'Opérateur a-t-il le droit ?"      │
│  (évalue pour chaque Tool)            │
└───────────────────────────────────────┘
    │
    ▼
┌───────────────────────────────────────┐
│  WorrySentinel                        │
│  "Le niveau de sécurité permet-il ?"  │
│  (vérifie security_level du Toolkit)  │
└───────────────────────────────────────┘
    │
    ▼
┌───────────────────────────────────────┐
│  Caring Nanny                         │
│  "L'état système permet-il ?"         │
│  (vérifie allowed_states/disallowed)  │
└───────────────────────────────────────┘
    │
    ▼
┌───────────────────────────────────────┐
│  Tools du Toolkit (exécution)         │
│  layout.render, input.capture, ...    │
└───────────────────────────────────────┘
```

**Règles du flux :**
- R-FLUX-1 : Un Toolkit passe par la même gouvernance qu'un Tool
- R-FLUX-2 : L'autorisation est vérifiée pour chaque Tool du Toolkit
- R-FLUX-3 : Si un Tool est refusé, le Toolkit entier est refusé
- R-FLUX-4 : L'Opérateur peut toujours appeler les Tools individuellement

### 7.2. Responsabilités des Cores

**Énoncé :**

Chaque Core a des responsabilités spécifiques dans la gouvernance des Toolkits.

| Core | Responsabilité pour les Toolkits |
|------|----------------------------------|
| **Master Butler** | Déclare, catalogue, fournit les informations |
| **StrongFather** | Évalue l'autorisation (via politiques) |
| **WorrySentinel** | Vérifie le niveau de sécurité |
| **Caring Nanny** | Vérifie l'état système |
| **Ever Buddy** | Gère le cycle de vie et versions |
| **BondingBrother** | Traduit les requêtes Opérateur |

**Ce que Master Butler fait pour les Toolkits :**

| Action | Oui/Non |
|--------|---------|
| Déclare l'existence des Toolkits | ✅ Oui |
| Catalogue la composition | ✅ Oui |
| Définit les métadonnées | ✅ Oui |
| Répond aux interrogations | ✅ Oui |

**Ce que Master Butler NE fait PAS pour les Toolkits :**

| Action | Oui/Non | Responsable |
|--------|---------|-------------|
| Décider de l'autorisation | ❌ Non | StrongFather |
| Exécuter les Tools | ❌ Non | Tools eux-mêmes |
| Vérifier la sécurité | ❌ Non | WorrySentinel |
| Vérifier l'état système | ❌ Non | Caring Nanny |

---

## 8. Invariants Non Négociables

### 8.1. Non-Création de Capacité

**Invariant INV-NN-1 :**

> **Un Toolkit n'ajoute jamais de capacité nouvelle. Les capacités exposées sont exactement l'union des capacités de ses Tools.**

**Vérification formelle :**
```
∀ toolkit ∈ Toolkits :
  capabilities(toolkit) = ∪ { capabilities(tool) | tool ∈ toolkit.tools }
```

**Implications :**
- Pas de logique ajoutée
- Pas de transformation des données
- Pas de décision métier

### 8.2. Composition Minimale

**Invariant INV-NN-2 :**

> **Un Toolkit contient au minimum deux Tools.**

**Vérification formelle :**
```
∀ toolkit ∈ Toolkits : |toolkit.tools| ≥ 2
```

**Justification :**
- Un seul Tool = pas de composition
- La valeur du Toolkit est dans le regroupement

### 8.3. Validité des Tools

**Invariant INV-NN-3 :**

> **Tous les Tools d'un Toolkit existent dans le registre et sont actifs (non Removed).**

**Vérification formelle :**
```
∀ toolkit ∈ Toolkits :
  ∀ tool ∈ toolkit.tools :
    exists(tool) ∧ status(tool) ≠ Removed
```

**Implication :**
- La suppression d'un Tool invalide les Toolkits qui le contiennent

### 8.4. Cohérence du Niveau de Sécurité

**Invariant INV-NN-4 :**

> **Le niveau de sécurité d'un Toolkit est au minimum égal au plus haut niveau de ses Tools.**

**Vérification formelle :**
```
∀ toolkit ∈ Toolkits :
  toolkit.security_level ≥ max({ tool.security_level | tool ∈ toolkit.tools })
```

**Justification :**
- Pas de contournement de sécurité via la composition
- Le Toolkit ne peut pas être moins restrictif que ses composants

### 8.5. Gouvernance Obligatoire

**Invariant INV-NN-5 :**

> **Un Toolkit passe par la même gouvernance qu'un Tool individuel.**

**Implications :**
- Pas de raccourci de gouvernance
- Pas d'appel direct sans vérification
- BondingBrother est le point d'entrée obligatoire

---

## 9. Schémas ASCII

### 9.1. Structure d'un Toolkit

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          TOOLKIT : ui.standard                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  MÉTADONNÉES                                                         │  │
│  ├──────────────────────────────────────────────────────────────────────┤  │
│  │  id: "ui.standard"                                                   │  │
│  │  name: "Kit d'Outils UI Standard"                                    │  │
│  │  version: "1.0.0"                                                    │  │
│  │  security_level: 2                                                   │  │
│  │  status: Active                                                      │  │
│  │  allowed_states: [HEALTHY, DEGRADED]                                 │  │
│  │  disallowed_states: [SECURITY_LOCKDOWN, MAINTENANCE]                 │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  COMPOSITION (5 Tools)                                               │  │
│  ├──────────────────────────────────────────────────────────────────────┤  │
│  │                                                                       │  │
│  │    ┌──────────────────┐     ┌──────────────────┐                     │  │
│  │    │  layout.render   │     │  input.capture   │                     │  │
│  │    │  (sec_level: 1)  │     │  (sec_level: 2)  │                     │  │
│  │    └──────────────────┘     └──────────────────┘                     │  │
│  │                                                                       │  │
│  │    ┌──────────────────┐     ┌──────────────────┐                     │  │
│  │    │  form.validate   │     │  theme.resolve   │                     │  │
│  │    │  (sec_level: 1)  │     │  (sec_level: 1)  │                     │  │
│  │    └──────────────────┘     └──────────────────┘                     │  │
│  │                                                                       │  │
│  │    ┌──────────────────┐                                              │  │
│  │    │  event.dispatch  │                                              │  │
│  │    │  (sec_level: 1)  │                                              │  │
│  │    └──────────────────┘                                              │  │
│  │                                                                       │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  CAPACITÉS EXPOSÉES = ∪ capacités des 5 Tools                               │
│  LOGIQUE AJOUTÉE = ∅                                                         │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 9.2. Flux de Déclaration de Toolkit

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     FLUX DE DÉCLARATION DE TOOLKIT                           │
└─────────────────────────────────────────────────────────────────────────────┘

   DÉCLARANT                      MASTER BUTLER                    REGISTRE
       │                              │                              │
       │  DeclareToolkit(             │                              │
       │    id: "ui.standard",        │                              │
       │    tools: [layout.render,    │                              │
       │            input.capture,    │                              │
       │            form.validate,    │                              │
       │            theme.resolve,    │                              │
       │            event.dispatch],  │                              │
       │    security_level: 2         │                              │
       │  )                           │                              │
       ├─────────────────────────────►│                              │
       │                              │                              │
       │                              │  1. Valider format id        │
       │                              │  2. Vérifier unicité         │
       │                              │  3. Vérifier |tools| ≥ 2     │
       │                              │  4. Vérifier existence Tools │
       │                              │  5. Vérifier statut Tools    │
       │                              │  6. Vérifier security_level  │
       │                              │                              │
       │                              │  [Validations OK]            │
       │                              │                              │
       │                              │  7. Créer Toolkit            │
       │                              ├─────────────────────────────►│
       │                              │                              │
       │                              │  8. Mettre à jour index      │
       │                              ├─────────────────────────────►│
       │                              │                              │
       │                              │  9. Historiser événement     │
       │                              ├─────────────────────────────►│
       │                              │                              │
       │                              │◄─────────────────────────────┤
       │                              │      [Toolkit créé]          │
       │◄─────────────────────────────┤                              │
       │    Result::Ok(Toolkit)       │                              │
       │                              │                              │
       ▼                              ▼                              ▼
```

### 9.3. Relation Tool ↔ Toolkit

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     RELATION TOOL ↔ TOOLKIT                                  │
└─────────────────────────────────────────────────────────────────────────────┘

                   TOOLS                              TOOLKITS
              (capacités atomiques)               (compositions)

       ┌──────────────────────┐
       │    layout.render     │◄─────────────┐
       └──────────────────────┘              │
                                             │    ┌────────────────────┐
       ┌──────────────────────┐              ├────┤   ui.standard      │
       │    input.capture     │◄─────────────┤    └────────────────────┘
       └──────────────────────┘              │
                                             │
       ┌──────────────────────┐              │
       │    form.validate     │◄─────────────┤
       └──────────────────────┘              │
                                             │
       ┌──────────────────────┐              │
       │    theme.resolve     │◄─────────────┤
       └──────────────────────┘              │
                                             │
       ┌──────────────────────┐              │
       │    event.dispatch    │◄─────────────┘
       └──────────────────────┘
              │
              │                                   ┌────────────────────┐
              └──────────────────────────────────►│   event.full       │
                                                  └────────────────────┘
       ┌──────────────────────┐              │
       │    event.listen      │◄─────────────┘
       └──────────────────────┘


       👉 Un Tool peut appartenir à plusieurs Toolkits
       👉 Un Toolkit agrège plusieurs Tools
       👉 Relation N:M (many-to-many)
```

---

## 10. Exemples Concrets

### 10.1. Exemple : Manifeste de Toolkit UI Standard

**Manifeste YAML :**

```yaml
toolkit:
  id: "ui.standard"
  version: "1.0.0"
  name: "Kit d'Outils UI Standard"
  description: "Ensemble de tools pour construire des interfaces utilisateur standard"
  tools:
    - layout.render
    - input.capture
    - form.validate
    - theme.resolve
    - event.dispatch
  security_level: 2
  allowed_states:
    - HEALTHY
    - DEGRADED
  disallowed_states:
    - SECURITY_LOCKDOWN
    - MAINTENANCE
  metadata:
    tags:
      - ui
      - standard
      - frontend
    documentation_url: "https://docs.miyukini.dev/toolkits/ui-standard"
```

**Déclaration programmatique :**

```
DeclareToolkit(
  id: "ui.standard",
  name: "Kit d'Outils UI Standard",
  description: "Ensemble de tools pour construire des interfaces utilisateur standard",
  version: "1.0.0",
  tools: {
    "layout.render",
    "input.capture",
    "form.validate",
    "theme.resolve",
    "event.dispatch"
  },
  security_level: 2,
  allowed_states: { HEALTHY, DEGRADED },
  metadata: {
    disallowed_states: { SECURITY_LOCKDOWN, MAINTENANCE },
    tags: { "ui", "standard", "frontend" }
  }
)
→ Result::Ok(Toolkit { id: "ui.standard", ... })
```

### 10.2. Exemple : Toolkit de Gestion de Données CRUD

**Manifeste YAML :**

```yaml
toolkit:
  id: "data.crud"
  version: "1.0.0"
  name: "Kit d'Outils CRUD Data"
  description: "Ensemble de tools pour les opérations CRUD sur les données"
  tools:
    - query.execute
    - query.insert
    - query.update
    - query.delete
    - cache.get
    - cache.set
    - cache.invalidate
  security_level: 3
  allowed_states:
    - HEALTHY
  disallowed_states:
    - DEGRADED
    - SECURITY_LOCKDOWN
    - MAINTENANCE
    - OFFLINE
  metadata:
    tags:
      - data
      - crud
      - database
```

### 10.3. Exemple : Interrogation de Toolkits

**Recherche par Tool contenu :**

```
QueryToolkit(
  filter: { contains_tool: "layout.render" }
)
→ Result::Ok([
    Toolkit { id: "ui.standard", ... },
    Toolkit { id: "ui.advanced", ... }
  ])
```

**Recherche par niveau de sécurité :**

```
QueryToolkit(
  filter: { security_level: 2, status: Active }
)
→ Result::Ok([
    Toolkit { id: "ui.standard", ... },
    Toolkit { id: "content.publishing", ... }
  ])
```

### 10.4. Exemple : Mise à Jour de Toolkit

**Ajout d'un Tool :**

```
UpdateToolkit(
  id: "ui.standard",
  changes: {
    add_tools: { "animation.trigger" }
  }
)
→ Result::Ok(Toolkit {
    id: "ui.standard",
    version: "1.1.0",  // incrémenté
    tools: {
      "layout.render",
      "input.capture",
      "form.validate",
      "theme.resolve",
      "event.dispatch",
      "animation.trigger"  // ajouté
    },
    ...
  })
```

**Modification du niveau de sécurité :**

```
UpdateToolkit(
  id: "ui.standard",
  changes: {
    security_level: 3  // augmenté de 2 à 3
  }
)
→ Result::Ok(Toolkit {
    id: "ui.standard",
    version: "1.2.0",
    security_level: 3,
    ...
  })
```

---

## 11. Conclusion

Ce contrat établit le modèle technique de composition des Toolkits dans Master Butler, définissant de manière absolue :

**Points clés :**
- **Toolkit :** Composition officielle de Tools, sans capacité nouvelle
- **ToolkitId :** Format canonique `<domain>.<function>`
- **Composition minimale :** Au moins 2 Tools
- **Security Level :** ≥ max des Tools composants
- **États système :** allowed_states et disallowed_states explicites
- **Opérations :** Declare, Query, Update, Deprecate, Remove
- **Gouvernance :** Même flux qu'un Tool individuel

**Invariants non négociables :**
- Non-création de capacité
- Composition minimale de 2 Tools
- Validité de tous les Tools
- Cohérence du niveau de sécurité
- Gouvernance obligatoire via BondingBrother

Ce contrat complète le Tool Governance Contract en définissant comment les Tools peuvent être regroupés en Toolkits pour plus d'efficience, tout en préservant les garanties de gouvernance et de sécurité.

**Non-négociabilité :** Ce contrat est absolu et non négociable. Le contrat prime sur toute considération pratique.

---

**Document créé le :** 2026-01-27  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, Master Butler Documentation Fondatrice, [Miyukini Conceptual References - Tools et Toolkits](../../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)  
**Type :** Contrat de composition non négociable

---

## 12. Mini log — erreurs / warnings / ambiguïtés rencontrées et corrigées

### Ambiguïté A1 : Format de l'identifiant de Toolkit

**Ambiguïté rencontrée :**
La documentation de référence donne un exemple d'identifiant (`ui.standard`) sans définir un format canonique strict.

**Décision prise :**
Format canonique défini : `<domain>.<function>` avec règles de nommage strictes (minuscules, sans accents, segments séparés par points).

**Justification :**
Un format strict garantit la cohérence, évite les collisions, et facilite l'indexation et la recherche.

**Correction effectuée :**
Section 3.2 "Identifiant de Toolkit (ToolkitId)" ajoutée avec format canonique et règles de nommage (R-TKID-1 à R-TKID-5).

### Ambiguïté A2 : Nombre minimum de Tools

**Ambiguïté rencontrée :**
La documentation de référence ne spécifie pas explicitement le nombre minimum de Tools dans un Toolkit.

**Décision prise :**
Minimum de 2 Tools obligatoire (INV-NN-2).

**Justification :**
Un seul Tool ne constitue pas une "composition". La valeur d'un Toolkit réside dans le regroupement cohérent de plusieurs Tools.

**Correction effectuée :**
Invariant INV-NN-2 défini avec vérification formelle |toolkit.tools| ≥ 2.

### Ambiguïté A3 : Calcul du niveau de sécurité

**Ambiguïté rencontrée :**
La documentation de référence indique un `security_level` pour le Toolkit sans préciser sa relation avec les niveaux des Tools.

**Décision prise :**
Le SecurityLevel d'un Toolkit ≥ max(SecurityLevel de chaque Tool), formalisé en INV-NN-4.

**Justification :**
Un Toolkit ne peut pas être moins restrictif que ses composants, sinon il permettrait de contourner les contrôles de sécurité des Tools individuels.

**Correction effectuée :**
Section 3.4 "Niveau de Sécurité" avec règles R-SEC-1 à R-SEC-3 et invariant INV-NN-4.

### Ambiguïté A4 : Impact de la dépréciation d'un Tool sur les Toolkits

**Ambiguïté rencontrée :**
La documentation de référence ne précise pas ce qui se passe quand un Tool contenu dans un Toolkit est déprécié ou supprimé.

**Décision prise :**
Règles R-TKTOOL-1 à R-TKTOOL-3 définies : avertissement si dépréciation, invalidation si suppression.

**Justification :**
La cohérence du système exige que les Toolkits soient mis à jour ou supprimés si leurs composants deviennent invalides.

**Correction effectuée :**
Section 5.3 "Statut de Toolkit" avec règles d'impact.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
