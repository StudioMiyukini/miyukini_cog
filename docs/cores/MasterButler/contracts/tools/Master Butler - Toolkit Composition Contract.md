# Master Butler â€” Toolkit Composition Contract

## 1. Introduction

### Objet du contrat

Ce document dÃ©finit le **Master Butler Toolkit Composition Contract** : un contrat normatif, non nÃ©gociable, et de statut FONDATION qui Ã©tablit les rÃ¨gles de composition des Kits d'Outils (Toolkits) dans le systÃ¨me Miyukini Core System v2.4.

Ce contrat dÃ©finit :
- La structure formelle d'un Toolkit
- Les rÃ¨gles de composition (quels Tools peuvent Ãªtre regroupÃ©s)
- Les mÃ©tadonnÃ©es obligatoires et optionnelles
- Les invariants de composition
- Les opÃ©rations autorisÃ©es sur les Toolkits
- Les contraintes de gouvernance

### PortÃ©e

Ce contrat s'applique Ã  **toute instance de Master Butler** et dÃ©finit de maniÃ¨re absolue :
- La dÃ©finition formelle d'un Toolkit
- Le modÃ¨le de composition (Toolkit â†’ Tools)
- Les rÃ¨gles de dÃ©claration et de validation
- Les mÃ©tadonnÃ©es des Toolkits
- Les contraintes de sÃ©curitÃ© et d'Ã©tat
- Les invariants de composition
- Les opÃ©rations autorisÃ©es et interdites

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il Ã©tablit des rÃ¨gles absolues que Master Butler applique sans exception. Ces rÃ¨gles ne peuvent Ãªtre contournÃ©es, nÃ©gociÃ©es, ou modifiÃ©es. Le contrat prime sur toute considÃ©ration pratique.

### Relation avec les autres contrats

Ce contrat complÃ¨te et s'articule avec les documents contractuels existants :

- **[Master Butler - Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md)** : DÃ©finit la nature, le rÃ´le, et les responsabilitÃ©s de Master Butler
- **Master Butler - Tool Governance Contract** : DÃ©finit la gouvernance des Tools individuels (contrat complÃ©mentaire)
- **[Master Butler - Capability Registry Contract](../registry/Master%20Butler%20-%20Capability%20Registry%20Contract.md)** : DÃ©finit le registre des capacitÃ©s
- **[Miyukini Conceptual References - Tools et Toolkits](..//..//..//..//miyukini-webway-system//reference//_index.md)** : DÃ©finit les concepts canoniques de Tool et Toolkit
- **[Miyukini Framework - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)** : Ce contrat respecte **LOI-1** (aucune dÃ©pendance externe critique) et **LOI-5** (coÃ»t proportionnel au hardware) en garantissant que la composition est locale, lÃ©gÃ¨re, et autonome.

**ComplÃ©mentaritÃ© :**
- Master Butler Documentation Fondatrice = dÃ©finition conceptuelle et philosophique
- Master Butler Tool Governance Contract = gouvernance des Tools individuels
- Master Butler Toolkit Composition Contract = modÃ¨le de composition des Toolkits

Ces contrats forment ensemble le systÃ¨me complet de catalogage et gouvernance des outils du systÃ¨me Miyukini Core System v2.4.

---

## 2. Doctrine Fondamentale

### Ã‰noncÃ© canonique

> **Les Kits d'Outils (Toolkits) sont des compositions officielles d'outils, optimisÃ©es pour l'efficience mais jamais pour l'autoritÃ©.**

### RÃ¨gle fondamentale

> **ðŸ‘‰ Un Kit d'Outils orchestre, mais n'ajoute pas de capacitÃ©.**

### Implications directes

| RÃ¨gle | Description |
|-------|-------------|
| **Composition uniquement** | Un Toolkit agrÃ¨ge des Tools existants, il n'en crÃ©e pas |
| **Sans logique mÃ©tier** | Aucune logique business dans un Toolkit |
| **Sans dÃ©cision** | Un Toolkit ne dÃ©cide jamais |
| **Sans capacitÃ© nouvelle** | Un Toolkit ne possÃ¨de pas de capacitÃ© qui n'existe pas dans ses Tools |
| **Optimisation pure** | Un Toolkit optimise les appels, normalise les flux |

### Ce qu'un Toolkit N'EST PAS

| âŒ N'est pas | Pourquoi |
|--------------|----------|
| Un nouvel Outil | Il ne crÃ©e pas de capacitÃ© nouvelle |
| Un service | Il n'a pas de logique propre |
| Un dÃ©cideur | Il n'a pas d'autoritÃ© |
| Une librairie libre | Il est gouvernÃ© |
| Un raccourci non gouvernÃ© | Il passe par la mÃªme gouvernance |

---

## 3. DÃ©finitions Formelles

### 3.1. Kit d'Outils (Toolkit)

**DÃ©finition formelle :**

Un **Toolkit** est une composition officielle de Tools, validÃ©e et dÃ©clarÃ©e par l'environnement, optimisÃ©e pour efficience, cohÃ©rence et performance. Il regroupe des Tools existants sans ajouter de capacitÃ© nouvelle.

**CaractÃ©ristiques formelles :**

- **IdentitÃ© unique :** Chaque Toolkit possÃ¨de un identifiant unique et immuable (ToolkitId)
- **Composition :** Le Toolkit est composÃ© d'un ou plusieurs Tools existants
- **Officiel :** Le Toolkit est dÃ©clarÃ© et validÃ© par l'environnement
- **GouvernÃ© :** Le Toolkit est soumis Ã  la gouvernance des Cores
- **Sans capacitÃ© nouvelle :** Le Toolkit n'expose que les capacitÃ©s de ses Tools constituants
- **OptimisÃ© :** Le Toolkit optimise les appels pour efficience et performance

**Structure formelle d'un Toolkit :**

```
Toolkit {
  id: ToolkitId,               // Identifiant unique et immuable
  name: String,                // Nom lisible humain
  description: String,         // Description du Toolkit
  version: Version,            // Version du Toolkit
  tools: Set<ToolId>,          // Ensemble des Tools composant le Toolkit
  security_level: SecurityLevel, // Niveau de sÃ©curitÃ© requis
  allowed_states: Set<SystemState>, // Ã‰tats systÃ¨me autorisÃ©s
  disallowed_states: Set<SystemState>, // Ã‰tats systÃ¨me interdits
  metadata: ToolkitMetadata,   // MÃ©tadonnÃ©es additionnelles
  created_at: Timestamp,       // Date de crÃ©ation
  status: ToolkitStatus        // Statut (Active, Deprecated, Removed)
}
```

**Invariants :**
- INV-TK-1 : Tout Toolkit possÃ¨de un ToolkitId unique et immuable
- INV-TK-2 : Tout Toolkit contient au moins deux Tools
- INV-TK-3 : Tous les Tools d'un Toolkit existent dans le registre
- INV-TK-4 : Un Toolkit ne dÃ©clare aucune capacitÃ© propre
- INV-TK-5 : Un Toolkit n'ajoute aucune logique mÃ©tier

### 3.2. Identifiant de Toolkit (ToolkitId)

**DÃ©finition formelle :**

Un **ToolkitId** est l'identifiant unique et immuable d'un Toolkit dans le registre. Il suit un format canonique qui encode le domaine et la fonction.

**Format canonique :**

```
<domain>.<function>
```

**Exemples :**
- `ui.standard` : Kit d'Outils UI Standard
- `data.crud` : Kit d'Outils CRUD de donnÃ©es
- `media.management` : Kit d'Outils de gestion des mÃ©dias
- `content.publishing` : Kit d'Outils de publication de contenu
- `search.full` : Kit d'Outils de recherche complet

**RÃ¨gles de nommage :**
- R-TKID-1 : Le format est strictement `<domain>.<function>`
- R-TKID-2 : Tous les segments sont en minuscules, sans accents
- R-TKID-3 : Les segments sont sÃ©parÃ©s par des points (.)
- R-TKID-4 : Chaque segment contient uniquement des lettres, chiffres, et underscores
- R-TKID-5 : L'identifiant est unique dans l'ensemble du systÃ¨me

**Invariants :**
- INV-TKID-1 : Tout ToolkitId respecte le format canonique
- INV-TKID-2 : Tout ToolkitId est unique dans le registre
- INV-TKID-3 : Un ToolkitId ne peut jamais Ãªtre modifiÃ© aprÃ¨s crÃ©ation

### 3.3. Identifiant de Tool (ToolId)

**DÃ©finition formelle :**

Un **ToolId** identifie un Tool individuel qui peut Ãªtre inclus dans un Toolkit.

**Format canonique :**

```
<domain>.<action>
```

**Exemples :**
- `layout.render` : Rend un layout
- `input.capture` : Capture une saisie utilisateur
- `form.validate` : Valide un formulaire
- `theme.resolve` : RÃ©sout un thÃ¨me
- `event.dispatch` : Dispatch un Ã©vÃ©nement
- `query.execute` : ExÃ©cute une requÃªte
- `cache.get` : RÃ©cupÃ¨re depuis le cache
- `file.read` : Lit un fichier
- `file.write` : Ã‰crit un fichier

**Invariants :**
- INV-TOOL-1 : Tout ToolId rÃ©fÃ©rencÃ© dans un Toolkit existe dans le registre des capacitÃ©s
- INV-TOOL-2 : Un Tool peut appartenir Ã  plusieurs Toolkits

### 3.4. Niveau de SÃ©curitÃ© (SecurityLevel)

**DÃ©finition formelle :**

Le **SecurityLevel** d'un Toolkit dÃ©finit le niveau de sÃ©curitÃ© minimum requis pour utiliser ce Toolkit.

**Niveaux dÃ©finis :**

| Niveau | Description | Contexte |
|--------|-------------|----------|
| 0 | Aucune restriction | OpÃ©rations publiques |
| 1 | Authentification requise | OpÃ©rations utilisateur de base |
| 2 | Authentification + rÃ´le | OpÃ©rations nÃ©cessitant des droits |
| 3 | Authentification + rÃ´le Ã©levÃ© | OpÃ©rations sensibles |
| 4 | Maximum | OpÃ©rations critiques |

**RÃ¨gle de calcul :**
- R-SEC-1 : Le SecurityLevel d'un Toolkit est au minimum Ã©gal au plus haut SecurityLevel de ses Tools
- R-SEC-2 : Le SecurityLevel d'un Toolkit peut Ãªtre supÃ©rieur Ã  celui de ses Tools (restriction supplÃ©mentaire)
- R-SEC-3 : Le SecurityLevel d'un Toolkit ne peut jamais Ãªtre infÃ©rieur Ã  celui d'un de ses Tools

**Invariants :**
- INV-SEC-1 : Le SecurityLevel d'un Toolkit â‰¥ max(SecurityLevel de chaque Tool)
- INV-SEC-2 : WorrySentinel valide le SecurityLevel avant utilisation

### 3.5. Ã‰tats SystÃ¨me (SystemState)

**DÃ©finition formelle :**

Les **SystemStates** dÃ©finissent les Ã©tats dans lesquels un Toolkit peut ou ne peut pas Ãªtre utilisÃ©.

**Ã‰tats systÃ¨me standard :**

| Ã‰tat | Description | Toolkit gÃ©nÃ©ralement |
|------|-------------|---------------------|
| HEALTHY | SystÃ¨me en fonctionnement normal | AutorisÃ© |
| DEGRADED | SystÃ¨me en mode dÃ©gradÃ© | AutorisÃ© (selon config) |
| MAINTENANCE | SystÃ¨me en maintenance | GÃ©nÃ©ralement interdit |
| SECURITY_LOCKDOWN | Verrouillage sÃ©curitÃ© | Interdit |
| OFFLINE | SystÃ¨me hors ligne | Selon conception |

**RÃ¨gles :**
- R-STATE-1 : Un Toolkit dÃ©finit explicitement ses Ã©tats autorisÃ©s
- R-STATE-2 : Un Toolkit dÃ©finit explicitement ses Ã©tats interdits
- R-STATE-3 : Caring Nanny vÃ©rifie l'Ã©tat systÃ¨me avant autorisation

**Invariants :**
- INV-STATE-1 : allowed_states âˆ© disallowed_states = âˆ… (pas d'intersection)
- INV-STATE-2 : Un Toolkit sans allowed_states explicites est autorisÃ© dans tous les Ã©tats non interdits

---

## 4. ModÃ¨le de Composition

### 4.1. Principe de Composition

**Ã‰noncÃ© :**

Un Toolkit est une **agrÃ©gation formelle** de Tools existants. La composition ne crÃ©e aucune fonctionnalitÃ© nouvelle, elle optimise l'accÃ¨s Ã  des fonctionnalitÃ©s existantes.

**SchÃ©ma de composition :**

```
Toolkit (composition)
 â”œâ”€ Tool A (capacitÃ© atomique)
 â”œâ”€ Tool B (capacitÃ© atomique)
 â”œâ”€ Tool C (capacitÃ© atomique)
 â””â”€ Tool D (capacitÃ© atomique)

CapacitÃ©s exposÃ©es par le Toolkit = âˆª (CapacitÃ©s de A, B, C, D)
Logique ajoutÃ©e par le Toolkit = âˆ… (ensemble vide)
```

**RÃ¨gles de composition :**
- R-COMP-1 : Un Toolkit contient au minimum 2 Tools
- R-COMP-2 : Un Toolkit ne peut contenir que des Tools existants et actifs
- R-COMP-3 : Un Toolkit n'ajoute aucune logique entre les Tools
- R-COMP-4 : L'ordre des Tools dans un Toolkit n'a pas de signification sÃ©mantique
- R-COMP-5 : Un Tool peut appartenir Ã  plusieurs Toolkits

**Invariants :**
- INV-COMP-1 : |tools| â‰¥ 2 (au moins deux Tools)
- INV-COMP-2 : âˆ€ tool âˆˆ tools : exists(tool) âˆ§ status(tool) â‰  Removed
- INV-COMP-3 : capabilities(Toolkit) = âˆª capabilities(tool) pour tool âˆˆ tools

### 4.2. CohÃ©rence de Composition

**Ã‰noncÃ© :**

Les Tools composant un Toolkit doivent Ãªtre cohÃ©rents entre eux. La composition doit avoir un sens fonctionnel.

**CritÃ¨res de cohÃ©rence :**

| CritÃ¨re | Description | Exemple valide |
|---------|-------------|----------------|
| Domaine commun | Tools du mÃªme domaine fonctionnel | UI: layout.render + input.capture |
| Flux complÃ©mentaire | Tools qui s'utilisent ensemble | CRUD: query.execute + cache.get |
| Optimisation groupÃ©e | Tools souvent appelÃ©s ensemble | Media: media.upload + media.validate |

**Contre-exemples (compositions non cohÃ©rentes) :**

| âŒ Interdit | Raison |
|-------------|--------|
| auth.login + layout.render | Domaines non liÃ©s |
| file.read seul | Un seul Tool |
| Tools dÃ©prÃ©ciÃ©s | Statut invalide |

**RÃ¨gles de cohÃ©rence :**
- R-COH-1 : Les Tools d'un Toolkit appartiennent gÃ©nÃ©ralement au mÃªme domaine
- R-COH-2 : La composition doit avoir une justification fonctionnelle documentÃ©e
- R-COH-3 : Un Toolkit ne regroupe pas de Tools sans lien fonctionnel

### 4.3. Ce que la Composition Apporte

**Ã‰noncÃ© :**

La composition en Toolkit apporte des bÃ©nÃ©fices d'efficience sans modifier les capacitÃ©s.

**BÃ©nÃ©fices de la composition :**

| BÃ©nÃ©fice | Description |
|----------|-------------|
| **Optimisation des appels** | RÃ©duction des allers-retours de gouvernance |
| **Normalisation des flux** | Standardisation des patterns d'appel |
| **CohÃ©rence garantie** | Ensemble de Tools validÃ© comme fonctionnel |
| **Documentation groupÃ©e** | Point d'entrÃ©e unique pour un ensemble cohÃ©rent |
| **Gouvernance simplifiÃ©e** | Une seule vÃ©rification pour plusieurs Tools |

**Ce que la composition N'apporte PAS :**

| âŒ N'apporte pas | Raison |
|-----------------|--------|
| Nouvelles capacitÃ©s | Le Toolkit ne crÃ©e pas |
| Logique mÃ©tier | Le Toolkit ne dÃ©cide pas |
| AutoritÃ© | Le Toolkit n'autorise pas |
| Optimisation forcÃ©e | L'OpÃ©rateur reste libre d'appeler les Tools individuellement |

---

## 5. MÃ©tadonnÃ©es des Toolkits

### 5.1. MÃ©tadonnÃ©es Obligatoires

**Ã‰noncÃ© :**

Tout Toolkit dÃ©clarÃ© doit fournir un ensemble minimal de mÃ©tadonnÃ©es obligatoires.

**MÃ©tadonnÃ©es obligatoires :**

| MÃ©tadonnÃ©e | Type | Description |
|------------|------|-------------|
| `id` | ToolkitId | Identifiant unique et immuable |
| `name` | String | Nom lisible humain (max 100 caractÃ¨res) |
| `description` | String | Description du Toolkit (max 500 caractÃ¨res) |
| `version` | Version | Version du Toolkit (semver) |
| `tools` | Set<ToolId> | Ensemble des Tools (min 2) |
| `security_level` | SecurityLevel | Niveau de sÃ©curitÃ© requis (0-4) |
| `allowed_states` | Set<SystemState> | Ã‰tats systÃ¨me autorisÃ©s |
| `created_at` | Timestamp | Date et heure de crÃ©ation |

**Invariants :**
- INV-META-1 : Toutes les mÃ©tadonnÃ©es obligatoires sont prÃ©sentes
- INV-META-2 : Les mÃ©tadonnÃ©es obligatoires ne peuvent pas Ãªtre nulles ou vides
- INV-META-3 : |tools| â‰¥ 2

### 5.2. MÃ©tadonnÃ©es Optionnelles

**Ã‰noncÃ© :**

Les Toolkits peuvent fournir des mÃ©tadonnÃ©es additionnelles pour enrichir la documentation et la dÃ©couverte.

**MÃ©tadonnÃ©es optionnelles :**

| MÃ©tadonnÃ©e | Type | Description |
|------------|------|-------------|
| `disallowed_states` | Set<SystemState> | Ã‰tats systÃ¨me interdits |
| `tags` | Set<String> | Tags de classification |
| `documentation_url` | URL | Lien vers la documentation |
| `examples` | List<String> | Exemples d'utilisation |
| `deprecated_at` | Timestamp | Date de dÃ©prÃ©ciation (si applicable) |
| `deprecation_reason` | String | Raison de la dÃ©prÃ©ciation |
| `successor` | ToolkitId | Toolkit de remplacement (si dÃ©prÃ©ciÃ©) |
| `custom` | Map<String, Any> | MÃ©tadonnÃ©es personnalisÃ©es |

**RÃ¨gles :**
- R-OPTMETA-1 : Les mÃ©tadonnÃ©es optionnelles peuvent Ãªtre nulles ou absentes
- R-OPTMETA-2 : Les tags sont normalisÃ©s (minuscules, sans accents)
- R-OPTMETA-3 : L'URL de documentation doit Ãªtre valide si fournie

### 5.3. Statut de Toolkit

**Ã‰noncÃ© :**

Tout Toolkit possÃ¨de un statut qui reflÃ¨te son Ã©tat dans le cycle de vie.

**Statuts possibles :**

| Statut | Description | Transitions possibles |
|--------|-------------|----------------------|
| **Active** | Toolkit disponible et utilisable | â†’ Deprecated, â†’ Removed |
| **Deprecated** | Toolkit obsolÃ¨te, utilisation dÃ©conseillÃ©e | â†’ Removed |
| **Removed** | Toolkit supprimÃ©, non utilisable | (terminal) |

**RÃ¨gles de transition :**
- R-TKST-1 : Un Toolkit nouvellement crÃ©Ã© est toujours Active
- R-TKST-2 : Un Toolkit Active peut Ãªtre dÃ©prÃ©ciÃ© (â†’ Deprecated)
- R-TKST-3 : Un Toolkit Deprecated peut Ãªtre supprimÃ© (â†’ Removed)
- R-TKST-4 : Un Toolkit Active peut Ãªtre supprimÃ© directement (â†’ Removed)
- R-TKST-5 : Un Toolkit Removed ne peut pas Ãªtre rÃ©activÃ©
- R-TKST-6 : Une transition de statut est irrÃ©versible

**Impact de la dÃ©prÃ©ciation d'un Tool :**
- R-TKTOOL-1 : Si un Tool est dÃ©prÃ©ciÃ©, les Toolkits qui le contiennent reÃ§oivent un avertissement
- R-TKTOOL-2 : Si un Tool est supprimÃ©, les Toolkits qui le contiennent deviennent invalides
- R-TKTOOL-3 : Un Toolkit invalide doit Ãªtre mis Ã  jour ou supprimÃ©

---

## 6. OpÃ©rations sur les Toolkits

### 6.1. DÃ©claration de Toolkit (DeclareToolkit)

**Ã‰noncÃ© :**

L'opÃ©ration **DeclareToolkit** permet de dÃ©clarer un nouveau Toolkit dans le registre.

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
) â†’ Result<Toolkit, DeclarationError>
```

**PrÃ©conditions :**
- PRE-1 : L'identifiant n'existe pas dÃ©jÃ  dans le registre
- PRE-2 : L'identifiant respecte le format canonique
- PRE-3 : Les mÃ©tadonnÃ©es obligatoires sont fournies et valides
- PRE-4 : |tools| â‰¥ 2
- PRE-5 : Tous les Tools existent et sont actifs
- PRE-6 : security_level â‰¥ max(security_level de chaque Tool)

**Postconditions :**
- POST-1 : Le Toolkit est ajoutÃ© au registre avec statut Active
- POST-2 : Les index sont mis Ã  jour
- POST-3 : L'historique est mis Ã  jour avec l'Ã©vÃ©nement de crÃ©ation
- POST-4 : La version du registre est incrÃ©mentÃ©e

**Erreurs possibles :**

| Erreur | Condition |
|--------|-----------|
| `InvalidToolkitId` | Format de l'identifiant invalide |
| `DuplicateToolkitId` | Identifiant dÃ©jÃ  existant |
| `InsufficientTools` | Moins de 2 Tools |
| `ToolNotFound` | Un Tool n'existe pas |
| `ToolNotActive` | Un Tool n'est pas actif |
| `SecurityLevelTooLow` | security_level < max des Tools |
| `MissingMetadata` | MÃ©tadonnÃ©e obligatoire manquante |

### 6.2. Interrogation de Toolkit (QueryToolkit)

**Ã‰noncÃ© :**

L'opÃ©ration **QueryToolkit** permet d'interroger le registre pour obtenir les informations sur un ou plusieurs Toolkits.

**Modes d'interrogation :**

| Mode | Description | Exemple |
|------|-------------|---------|
| **ById** | Recherche par identifiant exact | `ui.standard` |
| **ByTool** | Toolkits contenant un Tool | `layout.render` |
| **BySecurityLevel** | Toolkits d'un niveau de sÃ©curitÃ© | `2` |
| **ByStatus** | Recherche par statut | `Active` |
| **ByTags** | Recherche par tags | `["ui", "standard"]` |
| **All** | Tous les Toolkits | - |

**Signature conceptuelle :**

```
QueryToolkit(
  filter: ToolkitFilter
) â†’ Result<List<Toolkit>, QueryError>

ToolkitFilter {
  id: ToolkitId?,
  contains_tool: ToolId?,
  security_level: SecurityLevel?,
  status: ToolkitStatus?,
  tags: Set<String>?
}
```

**RÃ¨gles :**
- R-QUERY-1 : L'interrogation est toujours en lecture seule
- R-QUERY-2 : Les filtres peuvent Ãªtre combinÃ©s (AND logique)
- R-QUERY-3 : Une interrogation sans rÃ©sultat retourne une liste vide

### 6.3. Mise Ã  Jour de Toolkit (UpdateToolkit)

**Ã‰noncÃ© :**

L'opÃ©ration **UpdateToolkit** permet de modifier un Toolkit existant.

**Modifications autorisÃ©es :**

| Modification | AutorisÃ©e | Condition |
|--------------|-----------|-----------|
| Ajouter un Tool | âœ… Oui | Tool existe et est actif |
| Retirer un Tool | âœ… Oui | |tools| reste â‰¥ 2 |
| Modifier metadata | âœ… Oui | ValiditÃ© prÃ©servÃ©e |
| Modifier security_level | âœ… Oui | Respecte R-SEC-1 Ã  R-SEC-3 |
| Modifier allowed_states | âœ… Oui | Respecte INV-STATE-1 |
| Modifier id | âŒ Non | Immuable |
| Modifier created_at | âŒ Non | Immuable |

**Signature conceptuelle :**

```
UpdateToolkit(
  id: ToolkitId,
  changes: ToolkitChanges
) â†’ Result<Toolkit, UpdateError>

ToolkitChanges {
  add_tools: Set<ToolId>?,
  remove_tools: Set<ToolId>?,
  security_level: SecurityLevel?,
  allowed_states: Set<SystemState>?,
  disallowed_states: Set<SystemState>?,
  metadata: ToolkitMetadata?
}
```

**RÃ¨gles :**
- R-UPD-1 : La mise Ã  jour incrÃ©mente la version mineure du Toolkit
- R-UPD-2 : L'historique enregistre les changements
- R-UPD-3 : Les invariants sont vÃ©rifiÃ©s aprÃ¨s modification

### 6.4. DÃ©prÃ©ciation de Toolkit (DeprecateToolkit)

**Ã‰noncÃ© :**

L'opÃ©ration **DeprecateToolkit** permet de marquer un Toolkit comme obsolÃ¨te.

**Signature conceptuelle :**

```
DeprecateToolkit(
  id: ToolkitId,
  reason: String,
  successor: ToolkitId?
) â†’ Result<Toolkit, DeprecationError>
```

**PrÃ©conditions :**
- PRE-1 : Le Toolkit existe dans le registre
- PRE-2 : Le Toolkit a le statut Active
- PRE-3 : La raison de dÃ©prÃ©ciation est fournie
- PRE-4 : Si un successeur est indiquÃ©, il existe et est Active

**RÃ¨gles :**
- R-DEP-1 : Un Toolkit dÃ©prÃ©ciÃ© reste interrogeable
- R-DEP-2 : Un Toolkit dÃ©prÃ©ciÃ© ne peut pas Ãªtre redÃ©clarÃ©
- R-DEP-3 : La dÃ©prÃ©ciation est irrÃ©versible

### 6.5. Suppression de Toolkit (RemoveToolkit)

**Ã‰noncÃ© :**

L'opÃ©ration **RemoveToolkit** permet de supprimer dÃ©finitivement un Toolkit du registre actif.

**Signature conceptuelle :**

```
RemoveToolkit(
  id: ToolkitId,
  reason: String
) â†’ Result<(), RemovalError>
```

**PrÃ©conditions :**
- PRE-1 : Le Toolkit existe dans le registre
- PRE-2 : Le Toolkit a le statut Active ou Deprecated

**Postconditions :**
- POST-1 : Le Toolkit passe au statut Removed
- POST-2 : Le Toolkit n'apparaÃ®t plus dans les interrogations standard
- POST-3 : L'historique conserve la trace du Toolkit
- POST-4 : L'identifiant est rÃ©servÃ© (non rÃ©utilisable)

---

## 7. Flux de Gouvernance

### 7.1. Flux d'Appel d'un Toolkit

**Ã‰noncÃ© :**

L'appel d'un Toolkit suit le mÃªme flux de gouvernance que l'appel d'un Tool individuel.

**SÃ©quence :**

```
OpÃ©rateur (Strate 7)
    â”‚
    â”‚ "Je veux utiliser le Kit d'Outils UI"
    â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  BondingBrother (mÃ©diation)           â”‚
â”‚  Traduit la requÃªte                   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
    â”‚
    â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Master Butler                        â”‚
â”‚  "Ce Toolkit existe-t-il ?"           â”‚
â”‚  "Quels Tools le composent ?"         â”‚
â”‚  "Quelles permissions sont requises ?"â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
    â”‚
    â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  StrongFather                         â”‚
â”‚  "L'OpÃ©rateur a-t-il le droit ?"      â”‚
â”‚  (Ã©value pour chaque Tool)            â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
    â”‚
    â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  WorrySentinel                        â”‚
â”‚  "Le niveau de sÃ©curitÃ© permet-il ?"  â”‚
â”‚  (vÃ©rifie security_level du Toolkit)  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
    â”‚
    â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Caring Nanny                         â”‚
â”‚  "L'Ã©tat systÃ¨me permet-il ?"         â”‚
â”‚  (vÃ©rifie allowed_states/disallowed)  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
    â”‚
    â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Tools du Toolkit (exÃ©cution)         â”‚
â”‚  layout.render, input.capture, ...    â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**RÃ¨gles du flux :**
- R-FLUX-1 : Un Toolkit passe par la mÃªme gouvernance qu'un Tool
- R-FLUX-2 : L'autorisation est vÃ©rifiÃ©e pour chaque Tool du Toolkit
- R-FLUX-3 : Si un Tool est refusÃ©, le Toolkit entier est refusÃ©
- R-FLUX-4 : L'OpÃ©rateur peut toujours appeler les Tools individuellement

### 7.2. ResponsabilitÃ©s des Cores

**Ã‰noncÃ© :**

Chaque Core a des responsabilitÃ©s spÃ©cifiques dans la gouvernance des Toolkits.

| Core | ResponsabilitÃ© pour les Toolkits |
|------|----------------------------------|
| **Master Butler** | DÃ©clare, catalogue, fournit les informations |
| **StrongFather** | Ã‰value l'autorisation (via politiques) |
| **WorrySentinel** | VÃ©rifie le niveau de sÃ©curitÃ© |
| **Caring Nanny** | VÃ©rifie l'Ã©tat systÃ¨me |
| **Ever Buddy** | GÃ¨re le cycle de vie et versions |
| **BondingBrother** | Traduit les requÃªtes OpÃ©rateur |

**Ce que Master Butler fait pour les Toolkits :**

| Action | Oui/Non |
|--------|---------|
| DÃ©clare l'existence des Toolkits | âœ… Oui |
| Catalogue la composition | âœ… Oui |
| DÃ©finit les mÃ©tadonnÃ©es | âœ… Oui |
| RÃ©pond aux interrogations | âœ… Oui |

**Ce que Master Butler NE fait PAS pour les Toolkits :**

| Action | Oui/Non | Responsable |
|--------|---------|-------------|
| DÃ©cider de l'autorisation | âŒ Non | StrongFather |
| ExÃ©cuter les Tools | âŒ Non | Tools eux-mÃªmes |
| VÃ©rifier la sÃ©curitÃ© | âŒ Non | WorrySentinel |
| VÃ©rifier l'Ã©tat systÃ¨me | âŒ Non | Caring Nanny |

---

## 8. Invariants Non NÃ©gociables

### 8.1. Non-CrÃ©ation de CapacitÃ©

**Invariant INV-NN-1 :**

> **Un Toolkit n'ajoute jamais de capacitÃ© nouvelle. Les capacitÃ©s exposÃ©es sont exactement l'union des capacitÃ©s de ses Tools.**

**VÃ©rification formelle :**
```
âˆ€ toolkit âˆˆ Toolkits :
  capabilities(toolkit) = âˆª { capabilities(tool) | tool âˆˆ toolkit.tools }
```

**Implications :**
- Pas de logique ajoutÃ©e
- Pas de transformation des donnÃ©es
- Pas de dÃ©cision mÃ©tier

### 8.2. Composition Minimale

**Invariant INV-NN-2 :**

> **Un Toolkit contient au minimum deux Tools.**

**VÃ©rification formelle :**
```
âˆ€ toolkit âˆˆ Toolkits : |toolkit.tools| â‰¥ 2
```

**Justification :**
- Un seul Tool = pas de composition
- La valeur du Toolkit est dans le regroupement

### 8.3. ValiditÃ© des Tools

**Invariant INV-NN-3 :**

> **Tous les Tools d'un Toolkit existent dans le registre et sont actifs (non Removed).**

**VÃ©rification formelle :**
```
âˆ€ toolkit âˆˆ Toolkits :
  âˆ€ tool âˆˆ toolkit.tools :
    exists(tool) âˆ§ status(tool) â‰  Removed
```

**Implication :**
- La suppression d'un Tool invalide les Toolkits qui le contiennent

### 8.4. CohÃ©rence du Niveau de SÃ©curitÃ©

**Invariant INV-NN-4 :**

> **Le niveau de sÃ©curitÃ© d'un Toolkit est au minimum Ã©gal au plus haut niveau de ses Tools.**

**VÃ©rification formelle :**
```
âˆ€ toolkit âˆˆ Toolkits :
  toolkit.security_level â‰¥ max({ tool.security_level | tool âˆˆ toolkit.tools })
```

**Justification :**
- Pas de contournement de sÃ©curitÃ© via la composition
- Le Toolkit ne peut pas Ãªtre moins restrictif que ses composants

### 8.5. Gouvernance Obligatoire

**Invariant INV-NN-5 :**

> **Un Toolkit passe par la mÃªme gouvernance qu'un Tool individuel.**

**Implications :**
- Pas de raccourci de gouvernance
- Pas d'appel direct sans vÃ©rification
- BondingBrother est le point d'entrÃ©e obligatoire

---

## 9. SchÃ©mas ASCII

### 9.1. Structure d'un Toolkit

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                          TOOLKIT : ui.standard                               â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                              â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚  MÃ‰TADONNÃ‰ES                                                         â”‚  â”‚
â”‚  â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤  â”‚
â”‚  â”‚  id: "ui.standard"                                                   â”‚  â”‚
â”‚  â”‚  name: "Kit d'Outils UI Standard"                                    â”‚  â”‚
â”‚  â”‚  version: "1.0.0"                                                    â”‚  â”‚
â”‚  â”‚  security_level: 2                                                   â”‚  â”‚
â”‚  â”‚  status: Active                                                      â”‚  â”‚
â”‚  â”‚  allowed_states: [HEALTHY, DEGRADED]                                 â”‚  â”‚
â”‚  â”‚  disallowed_states: [SECURITY_LOCKDOWN, MAINTENANCE]                 â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚                                                                              â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚  COMPOSITION (5 Tools)                                               â”‚  â”‚
â”‚  â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤  â”‚
â”‚  â”‚                                                                       â”‚  â”‚
â”‚  â”‚    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”     â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                     â”‚  â”‚
â”‚  â”‚    â”‚  layout.render   â”‚     â”‚  input.capture   â”‚                     â”‚  â”‚
â”‚  â”‚    â”‚  (sec_level: 1)  â”‚     â”‚  (sec_level: 2)  â”‚                     â”‚  â”‚
â”‚  â”‚    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜     â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                     â”‚  â”‚
â”‚  â”‚                                                                       â”‚  â”‚
â”‚  â”‚    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”     â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                     â”‚  â”‚
â”‚  â”‚    â”‚  form.validate   â”‚     â”‚  theme.resolve   â”‚                     â”‚  â”‚
â”‚  â”‚    â”‚  (sec_level: 1)  â”‚     â”‚  (sec_level: 1)  â”‚                     â”‚  â”‚
â”‚  â”‚    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜     â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                     â”‚  â”‚
â”‚  â”‚                                                                       â”‚  â”‚
â”‚  â”‚    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                                              â”‚  â”‚
â”‚  â”‚    â”‚  event.dispatch  â”‚                                              â”‚  â”‚
â”‚  â”‚    â”‚  (sec_level: 1)  â”‚                                              â”‚  â”‚
â”‚  â”‚    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                                              â”‚  â”‚
â”‚  â”‚                                                                       â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚                                                                              â”‚
â”‚  CAPACITÃ‰S EXPOSÃ‰ES = âˆª capacitÃ©s des 5 Tools                               â”‚
â”‚  LOGIQUE AJOUTÃ‰E = âˆ…                                                         â”‚
â”‚                                                                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 9.2. Flux de DÃ©claration de Toolkit

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                     FLUX DE DÃ‰CLARATION DE TOOLKIT                           â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜

   DÃ‰CLARANT                      MASTER BUTLER                    REGISTRE
       â”‚                              â”‚                              â”‚
       â”‚  DeclareToolkit(             â”‚                              â”‚
       â”‚    id: "ui.standard",        â”‚                              â”‚
       â”‚    tools: [layout.render,    â”‚                              â”‚
       â”‚            input.capture,    â”‚                              â”‚
       â”‚            form.validate,    â”‚                              â”‚
       â”‚            theme.resolve,    â”‚                              â”‚
       â”‚            event.dispatch],  â”‚                              â”‚
       â”‚    security_level: 2         â”‚                              â”‚
       â”‚  )                           â”‚                              â”‚
       â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–ºâ”‚                              â”‚
       â”‚                              â”‚                              â”‚
       â”‚                              â”‚  1. Valider format id        â”‚
       â”‚                              â”‚  2. VÃ©rifier unicitÃ©         â”‚
       â”‚                              â”‚  3. VÃ©rifier |tools| â‰¥ 2     â”‚
       â”‚                              â”‚  4. VÃ©rifier existence Tools â”‚
       â”‚                              â”‚  5. VÃ©rifier statut Tools    â”‚
       â”‚                              â”‚  6. VÃ©rifier security_level  â”‚
       â”‚                              â”‚                              â”‚
       â”‚                              â”‚  [Validations OK]            â”‚
       â”‚                              â”‚                              â”‚
       â”‚                              â”‚  7. CrÃ©er Toolkit            â”‚
       â”‚                              â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–ºâ”‚
       â”‚                              â”‚                              â”‚
       â”‚                              â”‚  8. Mettre Ã  jour index      â”‚
       â”‚                              â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–ºâ”‚
       â”‚                              â”‚                              â”‚
       â”‚                              â”‚  9. Historiser Ã©vÃ©nement     â”‚
       â”‚                              â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–ºâ”‚
       â”‚                              â”‚                              â”‚
       â”‚                              â”‚â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
       â”‚                              â”‚      [Toolkit crÃ©Ã©]          â”‚
       â”‚â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤                              â”‚
       â”‚    Result::Ok(Toolkit)       â”‚                              â”‚
       â”‚                              â”‚                              â”‚
       â–¼                              â–¼                              â–¼
```

### 9.3. Relation Tool â†” Toolkit

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                     RELATION TOOL â†” TOOLKIT                                  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜

                   TOOLS                              TOOLKITS
              (capacitÃ©s atomiques)               (compositions)

       â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
       â”‚    layout.render     â”‚â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
       â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜              â”‚
                                             â”‚    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
       â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”              â”œâ”€â”€â”€â”€â”¤   ui.standard      â”‚
       â”‚    input.capture     â”‚â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
       â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜              â”‚
                                             â”‚
       â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”              â”‚
       â”‚    form.validate     â”‚â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
       â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜              â”‚
                                             â”‚
       â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”              â”‚
       â”‚    theme.resolve     â”‚â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
       â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜              â”‚
                                             â”‚
       â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”              â”‚
       â”‚    event.dispatch    â”‚â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
       â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
              â”‚
              â”‚                                   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
              â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–ºâ”‚   event.full       â”‚
                                                  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
       â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”              â”‚
       â”‚    event.listen      â”‚â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
       â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜


       ðŸ‘‰ Un Tool peut appartenir Ã  plusieurs Toolkits
       ðŸ‘‰ Un Toolkit agrÃ¨ge plusieurs Tools
       ðŸ‘‰ Relation N:M (many-to-many)
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

**DÃ©claration programmatique :**

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
â†’ Result::Ok(Toolkit { id: "ui.standard", ... })
```

### 10.2. Exemple : Toolkit de Gestion de DonnÃ©es CRUD

**Manifeste YAML :**

```yaml
toolkit:
  id: "data.crud"
  version: "1.0.0"
  name: "Kit d'Outils CRUD Data"
  description: "Ensemble de tools pour les opÃ©rations CRUD sur les donnÃ©es"
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
â†’ Result::Ok([
    Toolkit { id: "ui.standard", ... },
    Toolkit { id: "ui.advanced", ... }
  ])
```

**Recherche par niveau de sÃ©curitÃ© :**

```
QueryToolkit(
  filter: { security_level: 2, status: Active }
)
â†’ Result::Ok([
    Toolkit { id: "ui.standard", ... },
    Toolkit { id: "content.publishing", ... }
  ])
```

### 10.4. Exemple : Mise Ã  Jour de Toolkit

**Ajout d'un Tool :**

```
UpdateToolkit(
  id: "ui.standard",
  changes: {
    add_tools: { "animation.trigger" }
  }
)
â†’ Result::Ok(Toolkit {
    id: "ui.standard",
    version: "1.1.0",  // incrÃ©mentÃ©
    tools: {
      "layout.render",
      "input.capture",
      "form.validate",
      "theme.resolve",
      "event.dispatch",
      "animation.trigger"  // ajoutÃ©
    },
    ...
  })
```

**Modification du niveau de sÃ©curitÃ© :**

```
UpdateToolkit(
  id: "ui.standard",
  changes: {
    security_level: 3  // augmentÃ© de 2 Ã  3
  }
)
â†’ Result::Ok(Toolkit {
    id: "ui.standard",
    version: "1.2.0",
    security_level: 3,
    ...
  })
```

---

## 11. Conclusion

Ce contrat Ã©tablit le modÃ¨le technique de composition des Toolkits dans Master Butler, dÃ©finissant de maniÃ¨re absolue :

**Points clÃ©s :**
- **Toolkit :** Composition officielle de Tools, sans capacitÃ© nouvelle
- **ToolkitId :** Format canonique `<domain>.<function>`
- **Composition minimale :** Au moins 2 Tools
- **Security Level :** â‰¥ max des Tools composants
- **Ã‰tats systÃ¨me :** allowed_states et disallowed_states explicites
- **OpÃ©rations :** Declare, Query, Update, Deprecate, Remove
- **Gouvernance :** MÃªme flux qu'un Tool individuel

**Invariants non nÃ©gociables :**
- Non-crÃ©ation de capacitÃ©
- Composition minimale de 2 Tools
- ValiditÃ© de tous les Tools
- CohÃ©rence du niveau de sÃ©curitÃ©
- Gouvernance obligatoire via BondingBrother

Ce contrat complÃ¨te le Tool Governance Contract en dÃ©finissant comment les Tools peuvent Ãªtre regroupÃ©s en Toolkits pour plus d'efficience, tout en prÃ©servant les garanties de gouvernance et de sÃ©curitÃ©.

**Non-nÃ©gociabilitÃ© :** Ce contrat est absolu et non nÃ©gociable. Le contrat prime sur toute considÃ©ration pratique.

---

**Document crÃ©Ã© le :** 2026-01-27  
**Version :** 1.0  
**Statut :** FONDATION â€” Contrat normatif validÃ©  
**RÃ©fÃ©rence :** Miyukini Core System v2.4, Master Butler Documentation Fondatrice, [Miyukini Conceptual References - Tools et Toolkits](..//..//..//..//miyukini-webway-system//reference//_index.md)  
**Type :** Contrat de composition non nÃ©gociable

---

## 12. Mini log â€” erreurs / warnings / ambiguÃ¯tÃ©s rencontrÃ©es et corrigÃ©es

### AmbiguÃ¯tÃ© A1 : Format de l'identifiant de Toolkit

**AmbiguÃ¯tÃ© rencontrÃ©e :**
La documentation de rÃ©fÃ©rence donne un exemple d'identifiant (`ui.standard`) sans dÃ©finir un format canonique strict.

**DÃ©cision prise :**
Format canonique dÃ©fini : `<domain>.<function>` avec rÃ¨gles de nommage strictes (minuscules, sans accents, segments sÃ©parÃ©s par points).

**Justification :**
Un format strict garantit la cohÃ©rence, Ã©vite les collisions, et facilite l'indexation et la recherche.

**Correction effectuÃ©e :**
Section 3.2 "Identifiant de Toolkit (ToolkitId)" ajoutÃ©e avec format canonique et rÃ¨gles de nommage (R-TKID-1 Ã  R-TKID-5).

### AmbiguÃ¯tÃ© A2 : Nombre minimum de Tools

**AmbiguÃ¯tÃ© rencontrÃ©e :**
La documentation de rÃ©fÃ©rence ne spÃ©cifie pas explicitement le nombre minimum de Tools dans un Toolkit.

**DÃ©cision prise :**
Minimum de 2 Tools obligatoire (INV-NN-2).

**Justification :**
Un seul Tool ne constitue pas une "composition". La valeur d'un Toolkit rÃ©side dans le regroupement cohÃ©rent de plusieurs Tools.

**Correction effectuÃ©e :**
Invariant INV-NN-2 dÃ©fini avec vÃ©rification formelle |toolkit.tools| â‰¥ 2.

### AmbiguÃ¯tÃ© A3 : Calcul du niveau de sÃ©curitÃ©

**AmbiguÃ¯tÃ© rencontrÃ©e :**
La documentation de rÃ©fÃ©rence indique un `security_level` pour le Toolkit sans prÃ©ciser sa relation avec les niveaux des Tools.

**DÃ©cision prise :**
Le SecurityLevel d'un Toolkit â‰¥ max(SecurityLevel de chaque Tool), formalisÃ© en INV-NN-4.

**Justification :**
Un Toolkit ne peut pas Ãªtre moins restrictif que ses composants, sinon il permettrait de contourner les contrÃ´les de sÃ©curitÃ© des Tools individuels.

**Correction effectuÃ©e :**
Section 3.4 "Niveau de SÃ©curitÃ©" avec rÃ¨gles R-SEC-1 Ã  R-SEC-3 et invariant INV-NN-4.

### AmbiguÃ¯tÃ© A4 : Impact de la dÃ©prÃ©ciation d'un Tool sur les Toolkits

**AmbiguÃ¯tÃ© rencontrÃ©e :**
La documentation de rÃ©fÃ©rence ne prÃ©cise pas ce qui se passe quand un Tool contenu dans un Toolkit est dÃ©prÃ©ciÃ© ou supprimÃ©.

**DÃ©cision prise :**
RÃ¨gles R-TKTOOL-1 Ã  R-TKTOOL-3 dÃ©finies : avertissement si dÃ©prÃ©ciation, invalidation si suppression.

**Justification :**
La cohÃ©rence du systÃ¨me exige que les Toolkits soient mis Ã  jour ou supprimÃ©s si leurs composants deviennent invalides.

**Correction effectuÃ©e :**
Section 5.3 "Statut de Toolkit" avec rÃ¨gles d'impact.

---

*Aucune autre erreur, warning, ou ambiguÃ¯tÃ© rencontrÃ©e lors de la rÃ©daction de ce document.*

