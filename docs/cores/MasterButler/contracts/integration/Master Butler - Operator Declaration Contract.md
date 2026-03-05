# Master Butler â€” Operator Declaration Contract

## 1. Introduction

### Objet du contrat

Ce document dÃ©finit le **Master Butler â€” Operator Declaration Contract** : un contrat normatif, non nÃ©gociable, et de statut FONDATION qui Ã©tablit les rÃ¨gles de dÃ©claration des capacitÃ©s et permissions par les OpÃ©rateurs auprÃ¨s de Master Butler.

Ce contrat spÃ©cifie le protocole de dÃ©claration, les formats requis, les rÃ¨gles de validation, les responsabilitÃ©s des OpÃ©rateurs et de Master Butler, les invariants associÃ©s, et les garanties offertes par ce processus.

### PortÃ©e / Scope

Ce contrat s'applique Ã  **toute dÃ©claration de capacitÃ© ou de permission** effectuÃ©e par un OpÃ©rateur et dÃ©finit de maniÃ¨re absolue :
- le protocole de dÃ©claration des capacitÃ©s,
- le protocole de dÃ©finition des permissions,
- les formats et structures requis,
- les rÃ¨gles de validation des dÃ©clarations,
- les responsabilitÃ©s des OpÃ©rateurs dÃ©clarants,
- les responsabilitÃ©s de Master Butler lors de l'enregistrement,
- ce que la dÃ©claration PEUT et NE PEUT JAMAIS faire,
- les invariants systÃ©miques associÃ©s.

Ce document **ne couvre pas** :
- L'interrogation des capacitÃ©s (voir [Capability API Contract](../api/Master%20Butler%20-%20Capability%20API%20Contract.md))
- L'interrogation des permissions (voir [Permission API Contract](../api/Master%20Butler%20-%20Permission%20API%20Contract.md))
- La dÃ©couverte (voir [Discovery API Contract](../api/Master%20Butler%20-%20Discovery%20API%20Contract.md))
- L'intÃ©gration avec StrongFather (voir [StrongFather Integration Contract](./Master%20Butler%20-%20StrongFather%20Integration%20Contract.md))
- L'intÃ©gration avec BondingBrother (voir [BondingBrother Integration Contract](./Master%20Butler%20-%20BondingBrother%20Integration%20Contract.md))

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il Ã©tablit des rÃ¨gles absolues qui ne peuvent Ãªtre contournÃ©es, nÃ©gociÃ©es, ou modifiÃ©es. Le contrat prime sur toute considÃ©ration pratique.

### Relation avec les autres contrats

Ce contrat complÃ¨te et respecte les documents contractuels existants :
- **[Master Butler â€” Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md)** : DÃ©finition fondamentale du rÃ´le de Master Butler
- **[Master Butler â€” Capability Registry Contract](../registry/Master%20Butler%20-%20Capability%20Registry%20Contract.md)** : Structure du registre des capacitÃ©s
- **[Master Butler â€” Permission Registry Contract](../registry/Master%20Butler%20-%20Permission%20Registry%20Contract.md)** : Structure du registre des permissions
- **[Miyukini Conceptual References â€” Operators et Terminologie](..//..//..//..//miyukini-webway-system//reference//_index.md)** : DÃ©finition canonique des OpÃ©rateurs
- **[Miyukini Conceptual References â€” Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)** : Terminologie officielle
- **[Miyukini Conceptual References â€” Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)** : ConformitÃ© aux lois d'autonomie

Il n'introduit aucune contradiction avec le corpus documentaire existant.

---

## 2. Principes fondamentaux

### 2.1 DÃ©finition de la dÃ©claration

> **Une dÃ©claration est l'acte par lequel un OpÃ©rateur informe Master Butler des capacitÃ©s qu'il expose et des permissions qu'il dÃ©finit.**

La dÃ©claration est un acte fondateur : sans dÃ©claration, aucune capacitÃ© n'existe officiellement dans l'environnement. Master Butler est le rÃ©ceptacle unique de ces dÃ©clarations (INV-MB-1 : ExhaustivitÃ© du registre).

### 2.2 Principe de dÃ©claration obligatoire

> **Toute capacitÃ© exposÃ©e par un OpÃ©rateur DOIT Ãªtre dÃ©clarÃ©e Ã  Master Butler.**

Aucun OpÃ©rateur ne peut exposer une capacitÃ© sans la dÃ©clarer prÃ©alablement. Une capacitÃ© non dÃ©clarÃ©e n'existe pas dans l'Ã©cosystÃ¨me Miyukini.

**RÃ¨gle DECL-01 : DÃ©claration prÃ©alable obligatoire**

Un OpÃ©rateur NE PEUT PAS utiliser une capacitÃ© qu'il n'a pas dÃ©clarÃ©e Ã  Master Butler. La dÃ©claration prÃ©cÃ¨de toujours l'usage.

### 2.3 Principe de souverainetÃ© applicative

> **Un environnement Miyukini possÃ¨de une bibliothÃ¨que de capacitÃ©s finie, dÃ©clarÃ©e, gouvernÃ©e.**

Ce principe est non nÃ©gociable :

| RÃ¨gle | Description |
|-------|-------------|
| **Pas d'injection sauvage** | Aucune capacitÃ© ne peut Ãªtre ajoutÃ©e sans dÃ©claration dans Master Butler |
| **Pas de capacitÃ© locale** | Toute capacitÃ© doit Ãªtre dÃ©clarÃ©e dans l'environnement |
| **Pas de dÃ©pendance cachÃ©e** | Aucune capacitÃ© externe non gouvernÃ©e |

---

## 3. Types de dÃ©clarations

### 3.1 DÃ©claration de capacitÃ©

Une **dÃ©claration de capacitÃ©** enregistre un pouvoir technique qu'un OpÃ©rateur possÃ¨de.

**Structure de base :**

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| `capability_id` | string | âœ… Oui | Identifiant unique et stable de la capacitÃ© |
| `name` | string | âœ… Oui | Nom lisible de la capacitÃ© |
| `description` | string | âœ… Oui | Description fonctionnelle |
| `operator_id` | string | âœ… Oui | Identifiant de l'OpÃ©rateur dÃ©clarant |
| `module_origin` | string | âœ… Oui | Module d'origine de la capacitÃ© |
| `action_type` | string | âœ… Oui | Type d'action (create, read, update, delete, execute, etc.) |
| `target_type` | string | âœ… Oui | Type de ressource ciblÃ©e |
| `exposure_level` | enum | âœ… Oui | Niveau d'exposition (internal, operator, inter_cog, public) |
| `security_level` | enum | âœ… Oui | Niveau de sÃ©curitÃ© requis (0-4) |
| `metadata` | object | âŒ Non | MÃ©tadonnÃ©es additionnelles |
| `dependencies` | array | âŒ Non | CapacitÃ©s dont cette capacitÃ© dÃ©pend |
| `version` | string | âœ… Oui | Version de la capacitÃ© |

**Exemple de dÃ©claration de capacitÃ© :**

```
{
  "capability_id": "content.create",
  "name": "Create Content",
  "description": "Ability to create new content items in the CMS",
  "operator_id": "miyukini-spm-cms",
  "module_origin": "miyukini-spm-cms-content",
  "action_type": "create",
  "target_type": "content_item",
  "exposure_level": "operator",
  "security_level": 2,
  "metadata": {
    "category": "content_management",
    "tags": ["cms", "content", "creation"]
  },
  "dependencies": [],
  "version": "1.0.0"
}
```

### 3.2 DÃ©finition de permission

Une **dÃ©finition de permission** crÃ©e un droit accordable pour accÃ©der Ã  une ou plusieurs capacitÃ©s.

**Structure de base :**

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| `permission_id` | string | âœ… Oui | Identifiant unique de la permission |
| `name` | string | âœ… Oui | Nom lisible de la permission |
| `description` | string | âœ… Oui | Description de ce que la permission autorise |
| `operator_id` | string | âœ… Oui | Identifiant de l'OpÃ©rateur dÃ©finissant |
| `associated_capabilities` | array | âœ… Oui | Liste des capacitÃ©s couvertes (minimum 1) |
| `permission_level` | enum | âœ… Oui | Niveau de permission (basic, standard, elevated, admin) |
| `scope` | enum | âœ… Oui | PortÃ©e (own, team, all) |
| `conditions` | object | âŒ Non | Conditions d'application |
| `metadata` | object | âŒ Non | MÃ©tadonnÃ©es additionnelles |
| `version` | string | âœ… Oui | Version de la permission |

**Exemple de dÃ©finition de permission :**

```
{
  "permission_id": "content.create.own",
  "name": "Create Own Content",
  "description": "Permission to create content items owned by the user",
  "operator_id": "miyukini-spm-cms",
  "associated_capabilities": ["content.create"],
  "permission_level": "basic",
  "scope": "own",
  "conditions": {
    "requires_active_session": true
  },
  "metadata": {
    "category": "content_management"
  },
  "version": "1.0.0"
}
```

### 3.3 DÃ©claration de Tool

Une **dÃ©claration de Tool** enregistre une capacitÃ© exÃ©cutable gouvernÃ©e.

**Structure de base :**

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| `tool_id` | string | âœ… Oui | Identifiant unique du Tool |
| `name` | string | âœ… Oui | Nom lisible du Tool |
| `description` | string | âœ… Oui | Description fonctionnelle |
| `operator_id` | string | âœ… Oui | Identifiant de l'OpÃ©rateur dÃ©clarant |
| `linked_capability` | string | âœ… Oui | CapacitÃ© liÃ©e au Tool |
| `input_schema` | object | âœ… Oui | SchÃ©ma des entrÃ©es |
| `output_schema` | object | âœ… Oui | SchÃ©ma des sorties |
| `security_level` | enum | âœ… Oui | Niveau de sÃ©curitÃ© requis (0-4) |
| `idempotent` | boolean | âœ… Oui | Indique si le Tool est idempotent |
| `side_effects` | boolean | âœ… Oui | Indique si le Tool a des effets de bord |
| `metadata` | object | âŒ Non | MÃ©tadonnÃ©es additionnelles |
| `version` | string | âœ… Oui | Version du Tool |

### 3.4 DÃ©claration de Toolkit

Une **dÃ©claration de Toolkit** enregistre une composition officielle de Tools.

**Structure de base :**

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| `toolkit_id` | string | âœ… Oui | Identifiant unique du Toolkit |
| `name` | string | âœ… Oui | Nom lisible du Toolkit |
| `description` | string | âœ… Oui | Description fonctionnelle |
| `operator_id` | string | âœ… Oui | Identifiant de l'OpÃ©rateur dÃ©clarant |
| `composed_tools` | array | âœ… Oui | Liste des Tools composant le Toolkit (minimum 2) |
| `orchestration_rules` | object | âŒ Non | RÃ¨gles d'orchestration des Tools |
| `metadata` | object | âŒ Non | MÃ©tadonnÃ©es additionnelles |
| `version` | string | âœ… Oui | Version du Toolkit |

---

## 4. Protocole de dÃ©claration

### 4.1 Flux de dÃ©claration de capacitÃ©

**Acteurs :** OpÃ©rateur, BondingBrother (optionnel), Master Butler

**SÃ©quence :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚           FLUX DE DÃ‰CLARATION DE CAPACITÃ‰                        â”‚
â”‚                                                                   â”‚
â”‚  OPÃ‰RATEUR                                                       â”‚
â”‚      â”‚                                                            â”‚
â”‚      â”‚ 1. PrÃ©pare la dÃ©claration de capacitÃ©                     â”‚
â”‚      â”‚    - Identifiant unique                                   â”‚
â”‚      â”‚    - MÃ©tadonnÃ©es complÃ¨tes                                â”‚
â”‚      â”‚    - Version                                              â”‚
â”‚      â”‚                                                            â”‚
â”‚      â–¼                                                            â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚  SOUMISSION Ã€ MASTER BUTLER                              â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  â†’ declaration_api.declare_capability(declaration)        â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚      â”‚                                                            â”‚
â”‚      â–¼                                                            â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚  MASTER BUTLER â€” VALIDATION                              â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  1. Validation structurelle (champs obligatoires)        â”‚ â”‚
â”‚  â”‚  2. Validation d'unicitÃ© (capability_id)                 â”‚ â”‚
â”‚  â”‚  3. Validation des dÃ©pendances (si prÃ©sentes)            â”‚ â”‚
â”‚  â”‚  4. Validation de l'OpÃ©rateur dÃ©clarant                  â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚      â”‚                                                            â”‚
â”‚      â–¼                                                            â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚  MASTER BUTLER â€” ENREGISTREMENT                          â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  Si validation OK :                                       â”‚ â”‚
â”‚  â”‚    - Enregistrement dans le registre                     â”‚ â”‚
â”‚  â”‚    - Journalisation de la dÃ©claration                    â”‚ â”‚
â”‚  â”‚    - Retour : DECLARATION_ACCEPTED                       â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  Si validation KO :                                       â”‚ â”‚
â”‚  â”‚    - Rejet de la dÃ©claration                             â”‚ â”‚
â”‚  â”‚    - Retour : DECLARATION_REJECTED + raison              â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚      â”‚                                                            â”‚
â”‚      â–¼                                                            â”‚
â”‚  OPÃ‰RATEUR REÃ‡OIT LA CONFIRMATION                               â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 4.2 Flux de dÃ©finition de permission

**Acteurs :** OpÃ©rateur, Master Butler

**SÃ©quence :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚           FLUX DE DÃ‰FINITION DE PERMISSION                       â”‚
â”‚                                                                   â”‚
â”‚  OPÃ‰RATEUR                                                       â”‚
â”‚      â”‚                                                            â”‚
â”‚      â”‚ 1. PrÃ©pare la dÃ©finition de permission                    â”‚
â”‚      â”‚    - Identifiant unique                                   â”‚
â”‚      â”‚    - CapacitÃ©s associÃ©es (doivent exister)               â”‚
â”‚      â”‚    - MÃ©tadonnÃ©es complÃ¨tes                                â”‚
â”‚      â”‚                                                            â”‚
â”‚      â–¼                                                            â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚  SOUMISSION Ã€ MASTER BUTLER                              â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  â†’ declaration_api.define_permission(definition)          â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚      â”‚                                                            â”‚
â”‚      â–¼                                                            â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚  MASTER BUTLER â€” VALIDATION                              â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  1. Validation structurelle (champs obligatoires)        â”‚ â”‚
â”‚  â”‚  2. Validation d'unicitÃ© (permission_id)                 â”‚ â”‚
â”‚  â”‚  3. Validation des capacitÃ©s associÃ©es (DOIVENT EXISTER) â”‚ â”‚
â”‚  â”‚  4. Validation de l'OpÃ©rateur dÃ©finissant               â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚      â”‚                                                            â”‚
â”‚      â–¼                                                            â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚  MASTER BUTLER â€” ENREGISTREMENT                          â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  Si validation OK :                                       â”‚ â”‚
â”‚  â”‚    - Enregistrement dans le registre                     â”‚ â”‚
â”‚  â”‚    - CrÃ©ation des associations capability-permission     â”‚ â”‚
â”‚  â”‚    - Journalisation de la dÃ©finition                     â”‚ â”‚
â”‚  â”‚    - Retour : DEFINITION_ACCEPTED                        â”‚ â”‚
â”‚  â”‚                                                            â”‚ â”‚
â”‚  â”‚  Si validation KO :                                       â”‚ â”‚
â”‚  â”‚    - Rejet de la dÃ©finition                              â”‚ â”‚
â”‚  â”‚    - Retour : DEFINITION_REJECTED + raison               â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚      â”‚                                                            â”‚
â”‚      â–¼                                                            â”‚
â”‚  OPÃ‰RATEUR REÃ‡OIT LA CONFIRMATION                               â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 4.3 RÃ¨gle d'ordre de dÃ©claration

**RÃ¨gle DECL-02 : DÃ©pendances prÃ©alables**

Les capacitÃ©s DOIVENT Ãªtre dÃ©clarÃ©es avant les permissions qui les rÃ©fÃ©rencent. Une permission ne peut pas rÃ©fÃ©rencer une capacitÃ© inexistante.

**Ordre obligatoire :**
1. DÃ©claration des capacitÃ©s
2. DÃ©finition des permissions (rÃ©fÃ©renÃ§ant les capacitÃ©s existantes)
3. DÃ©claration des Tools (liant aux capacitÃ©s existantes)
4. DÃ©claration des Toolkits (composant des Tools existants)

---

## 5. RÃ¨gles de validation

### 5.1 Validation structurelle

**RÃ¨gle VAL-01 : Champs obligatoires**

Tous les champs marquÃ©s comme obligatoires DOIVENT Ãªtre prÃ©sents et non vides. Une dÃ©claration incomplÃ¨te est rejetÃ©e.

**RÃ¨gle VAL-02 : Format des identifiants**

Les identifiants (`capability_id`, `permission_id`, `tool_id`, `toolkit_id`) DOIVENT respecter le format suivant :
- CaractÃ¨res autorisÃ©s : `a-z`, `0-9`, `.`, `-`, `_`
- Pas d'espaces
- Longueur minimale : 3 caractÃ¨res
- Longueur maximale : 128 caractÃ¨res
- Format recommandÃ© : `domain.action.scope` (ex: `content.create.own`)

### 5.2 Validation d'unicitÃ©

**RÃ¨gle VAL-03 : UnicitÃ© des identifiants**

Les identifiants DOIVENT Ãªtre uniques dans leur registre respectif. Une dÃ©claration avec un identifiant dÃ©jÃ  existant est traitÃ©e selon les rÃ¨gles d'idempotence (voir section 6).

### 5.3 Validation des rÃ©fÃ©rences

**RÃ¨gle VAL-04 : Existence des capacitÃ©s rÃ©fÃ©rencÃ©es**

Une permission DOIT rÃ©fÃ©rencer au moins une capacitÃ© existante. Toutes les capacitÃ©s rÃ©fÃ©rencÃ©es DOIVENT exister dans le registre.

**RÃ¨gle VAL-05 : Existence des Tools rÃ©fÃ©rencÃ©s**

Un Toolkit DOIT rÃ©fÃ©rencer au moins deux Tools existants. Tous les Tools rÃ©fÃ©rencÃ©s DOIVENT exister dans le registre.

**RÃ¨gle VAL-06 : Existence des dÃ©pendances**

Si une capacitÃ© dÃ©clare des dÃ©pendances, toutes les dÃ©pendances DOIVENT exister dans le registre.

### 5.4 Validation de l'OpÃ©rateur

**RÃ¨gle VAL-07 : OpÃ©rateur reconnu**

L'OpÃ©rateur dÃ©clarant DOIT Ãªtre un OpÃ©rateur reconnu dans l'environnement. Un OpÃ©rateur inconnu ne peut pas dÃ©clarer de capacitÃ©s.

**RÃ¨gle VAL-08 : CohÃ©rence OpÃ©rateur-Module**

L'OpÃ©rateur dÃ©clarant DOIT Ãªtre autorisÃ© Ã  dÃ©clarer des capacitÃ©s pour le module d'origine spÃ©cifiÃ©.

---

## 6. RÃ¨gles d'idempotence

### 6.1 Principe d'idempotence

**RÃ¨gle IDEM-01 : DÃ©clarations idempotentes**

Les dÃ©clarations de capacitÃ©s sont idempotentes. DÃ©clarer deux fois la mÃªme capacitÃ© avec les mÃªmes donnÃ©es n'a pas d'effet supplÃ©mentaire. Le registre reste cohÃ©rent quel que soit l'ordre ou le nombre de dÃ©clarations.

### 6.2 Comportement en cas de redÃ©claration

**ScÃ©nario 1 : RedÃ©claration identique**

Si une capacitÃ© est dÃ©clarÃ©e avec exactement les mÃªmes donnÃ©es qu'une capacitÃ© existante :
- Aucune modification du registre
- Retour : `DECLARATION_ACCEPTED` (dÃ©jÃ  prÃ©sente)

**ScÃ©nario 2 : RedÃ©claration avec diffÃ©rences**

Si une capacitÃ© est dÃ©clarÃ©e avec le mÃªme identifiant mais des donnÃ©es diffÃ©rentes :
- Rejet de la dÃ©claration
- Retour : `DECLARATION_REJECTED` (conflit de version)
- L'OpÃ©rateur DOIT utiliser le protocole de mise Ã  jour (voir section 8)

### 6.3 Implications pratiques

**RÃ¨gle IDEM-02 : RedÃ©claration au dÃ©marrage**

Les OpÃ©rateurs PEUVENT redÃ©clarer leurs capacitÃ©s Ã  chaque dÃ©marrage sans effet indÃ©sirable. Cette pratique est encouragÃ©e pour garantir la cohÃ©rence du registre.

---

## 7. ImmutabilitÃ© des identifiants

### 7.1 Principe d'immutabilitÃ©

**RÃ¨gle IMMUT-01 : Identifiants immuables**

Les identifiants de capacitÃ©s sont immuables. Une fois qu'une capacitÃ© est dÃ©clarÃ©e avec un identifiant, cet identifiant ne change jamais.

### 7.2 ConsÃ©quences de l'immutabilitÃ©

| Situation | Action requise |
|-----------|----------------|
| CapacitÃ© Ã©volue significativement | CrÃ©er une nouvelle capacitÃ© avec un nouvel identifiant |
| Correction mineure | Utiliser le protocole de mise Ã  jour (mÃªme identifiant, nouvelle version) |
| Renommage fonctionnel | CrÃ©er une nouvelle capacitÃ©, dÃ©prÃ©cier l'ancienne |

**RÃ¨gle IMMUT-02 : StabilitÃ© des rÃ©fÃ©rences**

Les rÃ©fÃ©rences aux capacitÃ©s (dans les permissions, les logs, les configurations) DOIVENT rester valides dans le temps grÃ¢ce Ã  l'immutabilitÃ© des identifiants.

---

## 8. Protocole de mise Ã  jour

### 8.1 Mise Ã  jour d'une capacitÃ©

Une capacitÃ© existante peut Ãªtre mise Ã  jour dans les limites suivantes :

**Champs modifiables :**
- `name` (avec contraintes)
- `description`
- `metadata`
- `version` (obligatoirement incrÃ©mentÃ©e)

**Champs NON modifiables :**
- `capability_id` (immutable)
- `operator_id` (propriÃ©taire fixe)
- `module_origin` (fixÃ© Ã  la dÃ©claration)
- `action_type` (changement = nouvelle capacitÃ©)
- `target_type` (changement = nouvelle capacitÃ©)

**Flux de mise Ã  jour :**

```
OpÃ©rateur â†’ Master Butler : update_capability(capability_id, updates)
Master Butler :
  1. VÃ©rifie l'existence de la capacitÃ©
  2. VÃ©rifie que l'OpÃ©rateur est le propriÃ©taire
  3. VÃ©rifie que seuls les champs autorisÃ©s sont modifiÃ©s
  4. VÃ©rifie que la version est incrÃ©mentÃ©e
  5. Applique la mise Ã  jour
  6. Journalise la modification
  7. Retourne : UPDATE_ACCEPTED
```

### 8.2 DÃ©prÃ©ciation d'une capacitÃ©

Une capacitÃ© peut Ãªtre dÃ©prÃ©ciÃ©e mais pas supprimÃ©e immÃ©diatement.

**Flux de dÃ©prÃ©ciation :**

```
OpÃ©rateur â†’ Master Butler : deprecate_capability(capability_id, reason, successor_id?)
Master Butler :
  1. VÃ©rifie l'existence de la capacitÃ©
  2. VÃ©rifie que l'OpÃ©rateur est le propriÃ©taire
  3. Marque la capacitÃ© comme DEPRECATED
  4. Enregistre la raison et le successeur (si fourni)
  5. Journalise la dÃ©prÃ©ciation
  6. Retourne : DEPRECATION_ACCEPTED
```

**RÃ¨gle DEPR-01 : PÃ©riode de dÃ©prÃ©ciation**

Une capacitÃ© dÃ©prÃ©ciÃ©e reste fonctionnelle pendant une pÃ©riode de grÃ¢ce dÃ©finie. Les consommateurs sont avertis de migrer vers le successeur.

---

## 9. ResponsabilitÃ©s

### 9.1 ResponsabilitÃ©s de l'OpÃ©rateur dÃ©clarant

| ResponsabilitÃ© | Description |
|----------------|-------------|
| **RESP-OP-1** | DÃ©clarer toutes ses capacitÃ©s avant de les exposer |
| **RESP-OP-2** | Fournir des identifiants uniques et stables |
| **RESP-OP-3** | Fournir des mÃ©tadonnÃ©es complÃ¨tes et exactes |
| **RESP-OP-4** | Maintenir la cohÃ©rence des dÃ©clarations |
| **RESP-OP-5** | GÃ©rer les versions de ses capacitÃ©s |
| **RESP-OP-6** | DÃ©prÃ©cier proprement les capacitÃ©s obsolÃ¨tes |
| **RESP-OP-7** | Ne jamais exposer une capacitÃ© non dÃ©clarÃ©e |

### 9.2 ResponsabilitÃ©s de Master Butler

| ResponsabilitÃ© | Description |
|----------------|-------------|
| **RESP-MB-1** | Valider toutes les dÃ©clarations selon les rÃ¨gles |
| **RESP-MB-2** | Enregistrer les dÃ©clarations validÃ©es |
| **RESP-MB-3** | Rejeter les dÃ©clarations non conformes |
| **RESP-MB-4** | Garantir l'exhaustivitÃ© du registre |
| **RESP-MB-5** | Journaliser toutes les dÃ©clarations |
| **RESP-MB-6** | Garantir l'idempotence des dÃ©clarations |
| **RESP-MB-7** | PrÃ©server l'immutabilitÃ© des identifiants |

---

## 10. Ce que la dÃ©claration PEUT faire

### 10.1 OpÃ©rations autorisÃ©es

**PEUT-DECL-1 : Enregistrer des capacitÃ©s**

Un OpÃ©rateur PEUT dÃ©clarer des capacitÃ©s qu'il expose lÃ©gitimement.

**PEUT-DECL-2 : DÃ©finir des permissions**

Un OpÃ©rateur PEUT dÃ©finir des permissions associÃ©es Ã  ses capacitÃ©s.

**PEUT-DECL-3 : DÃ©clarer des Tools**

Un OpÃ©rateur PEUT dÃ©clarer des Tools liÃ©s Ã  ses capacitÃ©s.

**PEUT-DECL-4 : DÃ©clarer des Toolkits**

Un OpÃ©rateur PEUT dÃ©clarer des Toolkits composÃ©s de ses Tools.

**PEUT-DECL-5 : Mettre Ã  jour ses dÃ©clarations**

Un OpÃ©rateur PEUT mettre Ã  jour ses capacitÃ©s dans les limites dÃ©finies.

**PEUT-DECL-6 : DÃ©prÃ©cier ses capacitÃ©s**

Un OpÃ©rateur PEUT dÃ©prÃ©cier ses capacitÃ©s obsolÃ¨tes.

**PEUT-DECL-7 : RedÃ©clarer au dÃ©marrage**

Un OpÃ©rateur PEUT redÃ©clarer ses capacitÃ©s Ã  chaque dÃ©marrage (idempotence).

---

## 11. Ce que la dÃ©claration NE PEUT JAMAIS faire

### 11.1 Interdictions absolues

**INTERDIT-DECL-1 : DÃ©clarer des capacitÃ©s d'un autre OpÃ©rateur**

Un OpÃ©rateur NE PEUT JAMAIS dÃ©clarer des capacitÃ©s appartenant Ã  un autre OpÃ©rateur.

**INTERDIT-DECL-2 : Modifier l'identifiant d'une capacitÃ©**

Un OpÃ©rateur NE PEUT JAMAIS modifier l'identifiant d'une capacitÃ© existante. Les identifiants sont immuables.

**INTERDIT-DECL-3 : RÃ©fÃ©rencer des capacitÃ©s inexistantes**

Une permission NE PEUT JAMAIS rÃ©fÃ©rencer des capacitÃ©s qui n'existent pas dans le registre.

**INTERDIT-DECL-4 : Supprimer une capacitÃ© directement**

Un OpÃ©rateur NE PEUT JAMAIS supprimer une capacitÃ© directement. Le processus de dÃ©prÃ©ciation â†’ retrait est obligatoire.

**INTERDIT-DECL-5 : Contourner la validation**

Un OpÃ©rateur NE PEUT JAMAIS contourner les rÃ¨gles de validation de Master Butler.

**INTERDIT-DECL-6 : Exposer sans dÃ©clarer**

Un OpÃ©rateur NE PEUT JAMAIS exposer une capacitÃ© sans l'avoir prÃ©alablement dÃ©clarÃ©e.

**INTERDIT-DECL-7 : Modifier les mÃ©tadonnÃ©es d'un autre**

Un OpÃ©rateur NE PEUT JAMAIS modifier les capacitÃ©s ou permissions d'un autre OpÃ©rateur.

---

## 12. Invariants de dÃ©claration

### 12.1 Invariants globaux

**INV-DECL-1 : ExhaustivitÃ©**

Toute capacitÃ© exposÃ©e dans l'environnement DOIT Ãªtre prÃ©sente dans le registre de Master Butler. Aucune capacitÃ© fantÃ´me n'est autorisÃ©e.

**INV-DECL-2 : PropriÃ©tÃ© exclusive**

Chaque capacitÃ© appartient Ã  un OpÃ©rateur unique. Seul le propriÃ©taire peut modifier ou dÃ©prÃ©cier sa capacitÃ©.

**INV-DECL-3 : Idempotence garantie**

Les dÃ©clarations sont idempotentes. La mÃªme dÃ©claration peut Ãªtre effectuÃ©e plusieurs fois sans effet de bord.

**INV-DECL-4 : ImmutabilitÃ© des identifiants**

Les identifiants de capacitÃ©s ne changent jamais aprÃ¨s leur crÃ©ation.

**INV-DECL-5 : TraÃ§abilitÃ© complÃ¨te**

Toute dÃ©claration, modification, ou dÃ©prÃ©ciation est journalisÃ©e avec son contexte complet.

**INV-DECL-6 : CohÃ©rence des rÃ©fÃ©rences**

Toutes les rÃ©fÃ©rences entre entitÃ©s (permission â†’ capacitÃ©, Tool â†’ capacitÃ©, Toolkit â†’ Tools) pointent vers des entitÃ©s existantes.

**INV-DECL-7 : Ordre de dÃ©pendance**

Les entitÃ©s dÃ©pendantes sont toujours crÃ©Ã©es aprÃ¨s leurs dÃ©pendances (capacitÃ©s avant permissions, Tools avant Toolkits).

---

## 13. TraÃ§abilitÃ© des dÃ©clarations

### 13.1 Ã‰lÃ©ments Ã  tracer

| Ã‰lÃ©ment | Description |
|---------|-------------|
| `declaration_id` | Identifiant unique de la dÃ©claration |
| `timestamp` | Horodatage de la dÃ©claration |
| `operator_id` | OpÃ©rateur dÃ©clarant |
| `declaration_type` | Type (capability, permission, tool, toolkit) |
| `entity_id` | Identifiant de l'entitÃ© dÃ©clarÃ©e |
| `action` | Action (create, update, deprecate) |
| `previous_state` | Ã‰tat prÃ©cÃ©dent (si modification) |
| `new_state` | Nouvel Ã©tat |
| `validation_result` | RÃ©sultat de validation (accepted, rejected) |
| `rejection_reason` | Raison du rejet (si applicable) |

### 13.2 RÃ©tention des traces

Les traces de dÃ©claration sont conservÃ©es indÃ©finiment. Elles constituent l'historique officiel du registre.

---

## 14. Gestion des erreurs

### 14.1 Types de rejets

| Code | Signification | Action OpÃ©rateur |
|------|---------------|------------------|
| `MISSING_REQUIRED_FIELD` | Champ obligatoire manquant | ComplÃ©ter la dÃ©claration |
| `INVALID_IDENTIFIER` | Format d'identifiant invalide | Corriger le format |
| `DUPLICATE_IDENTIFIER` | Identifiant dÃ©jÃ  utilisÃ© | Choisir un autre identifiant ou vÃ©rifier l'idempotence |
| `UNKNOWN_CAPABILITY` | CapacitÃ© rÃ©fÃ©rencÃ©e inexistante | DÃ©clarer la capacitÃ© d'abord |
| `UNKNOWN_TOOL` | Tool rÃ©fÃ©rencÃ© inexistant | DÃ©clarer le Tool d'abord |
| `UNAUTHORIZED_OPERATOR` | OpÃ©rateur non autorisÃ© | VÃ©rifier l'identitÃ© de l'OpÃ©rateur |
| `MODULE_MISMATCH` | IncohÃ©rence OpÃ©rateur-Module | VÃ©rifier le module d'origine |
| `IMMUTABLE_FIELD_CHANGE` | Tentative de modification d'un champ immutable | CrÃ©er une nouvelle entitÃ© |

### 14.2 Principe de gestion

> **En cas d'erreur, la dÃ©claration est rejetÃ©e entiÃ¨rement. Aucune dÃ©claration partielle n'est acceptÃ©e.**

---

## 15. Exemples complets

### 15.1 DÃ©claration complÃ¨te d'un OpÃ©rateur CMS

```
// Ã‰tape 1 : DÃ©claration des capacitÃ©s
declare_capability({
  capability_id: "content.create",
  name: "Create Content",
  description: "Create new content items",
  operator_id: "miyukini-spm-cms",
  module_origin: "miyukini-spm-cms-content",
  action_type: "create",
  target_type: "content_item",
  exposure_level: "operator",
  security_level: 2,
  version: "1.0.0"
})

declare_capability({
  capability_id: "content.read",
  name: "Read Content",
  description: "Read existing content items",
  operator_id: "miyukini-spm-cms",
  module_origin: "miyukini-spm-cms-content",
  action_type: "read",
  target_type: "content_item",
  exposure_level: "operator",
  security_level: 1,
  version: "1.0.0"
})

// Ã‰tape 2 : DÃ©finition des permissions
define_permission({
  permission_id: "content.create.own",
  name: "Create Own Content",
  description: "Create content owned by the user",
  operator_id: "miyukini-spm-cms",
  associated_capabilities: ["content.create"],
  permission_level: "basic",
  scope: "own",
  version: "1.0.0"
})

define_permission({
  permission_id: "content.read.all",
  name: "Read All Content",
  description: "Read any content item",
  operator_id: "miyukini-spm-cms",
  associated_capabilities: ["content.read"],
  permission_level: "standard",
  scope: "all",
  version: "1.0.0"
})

// Ã‰tape 3 : DÃ©claration des Tools
declare_tool({
  tool_id: "content.create.tool",
  name: "Content Creation Tool",
  description: "Tool for creating content items",
  operator_id: "miyukini-spm-cms",
  linked_capability: "content.create",
  input_schema: { type: "object", properties: { title: { type: "string" }, body: { type: "string" } } },
  output_schema: { type: "object", properties: { content_id: { type: "string" } } },
  security_level: 2,
  idempotent: false,
  side_effects: true,
  version: "1.0.0"
})
```

---

## 16. ConformitÃ© aux Lois d'Autonomie

### 16.1 LOI-1 : Aucune dÃ©pendance externe

Le protocole de dÃ©claration fonctionne entiÃ¨rement en local. Aucune dÃ©pendance externe n'est requise pour dÃ©clarer, valider, ou enregistrer des capacitÃ©s.

### 16.2 LOI-5 : CoÃ»t proportionnel

Les dÃ©clarations sont des opÃ©rations lÃ©gÃ¨res. Le coÃ»t de stockage et de traitement est proportionnel au nombre de capacitÃ©s dÃ©clarÃ©es, qui reste bornÃ© et prÃ©visible.

---

## 17. Conclusion contractuelle

Ce contrat Ã©tablit de maniÃ¨re dÃ©finitive et non nÃ©gociable les rÃ¨gles de dÃ©claration des capacitÃ©s et permissions par les OpÃ©rateurs auprÃ¨s de Master Butler.

Il garantit que :
- toute capacitÃ© est dÃ©clarÃ©e avant d'Ãªtre exposÃ©e,
- les identifiants sont uniques et immuables,
- les dÃ©clarations sont idempotentes,
- la validation est stricte et complÃ¨te,
- la traÃ§abilitÃ© est assurÃ©e,
- les responsabilitÃ©s sont clairement rÃ©parties,
- les invariants sont respectÃ©s.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisÃ©e.

---

**Document crÃ©Ã© le :** 2026-01-27  
**Version :** 1.0  
**Statut :** FONDATION â€” Contrat normatif validÃ©  
**RÃ©fÃ©rence :** Miyukini Core System v2.4, Master Butler Documentation Fondatrice v1.4, [Miyukini Conceptual References â€” Operators et Terminologie](..//..//..//..//miyukini-webway-system//reference//_index.md), [Miyukini Conceptual References â€” Tools et Toolkits](..//..//..//..//miyukini-webway-system//reference//_index.md)  
**Type :** Contrat de dÃ©claration non nÃ©gociable

---

## 18. Mini log â€” erreurs / warnings / ambiguÃ¯tÃ©s rencontrÃ©es et corrigÃ©es

### AmbiguÃ¯tÃ© A1 : Ordre de dÃ©claration des entitÃ©s

**AmbiguÃ¯tÃ© rencontrÃ©e :** Risque de confusion sur l'ordre de dÃ©claration entre capacitÃ©s, permissions, Tools et Toolkits.

**DÃ©cision prise :** Section 4.3 dÃ©finit explicitement l'ordre obligatoire : capacitÃ©s â†’ permissions â†’ Tools â†’ Toolkits.

**Correction effectuÃ©e :** RÃ¨gle DECL-02 et section 4.3 rÃ©digÃ©es avec ordre explicite.

### AmbiguÃ¯tÃ© A2 : RedÃ©claration avec diffÃ©rences vs mise Ã  jour

**AmbiguÃ¯tÃ© rencontrÃ©e :** Confusion possible entre une redÃ©claration idempotente et une tentative de modification.

**DÃ©cision prise :** Section 6.2 distingue explicitement les deux scÃ©narios : redÃ©claration identique (acceptÃ©e) vs redÃ©claration avec diffÃ©rences (rejetÃ©e, utiliser le protocole de mise Ã  jour).

**Correction effectuÃ©e :** Section 6.2 et section 8 clarifient les deux cas.

### AmbiguÃ¯tÃ© A3 : PropriÃ©tÃ© des capacitÃ©s

**AmbiguÃ¯tÃ© rencontrÃ©e :** Question sur qui peut modifier une capacitÃ©.

**DÃ©cision prise :** Invariant INV-DECL-2 Ã©tablit la propriÃ©tÃ© exclusive : seul l'OpÃ©rateur propriÃ©taire peut modifier ou dÃ©prÃ©cier sa capacitÃ©.

**Correction effectuÃ©e :** Invariant explicite ajoutÃ©, ainsi que les rÃ¨gles VAL-08 et INTERDIT-DECL-7.

### Warning W1 : Suppression directe de capacitÃ©s

**Warning rencontrÃ© :** Risque d'incohÃ©rence si une capacitÃ© est supprimÃ©e alors que des permissions la rÃ©fÃ©rencent.

**DÃ©cision prise :** Interdiction INTERDIT-DECL-4 : aucune suppression directe. Le processus de dÃ©prÃ©ciation est obligatoire.

**Correction effectuÃ©e :** Section 8.2 dÃ©finit le protocole de dÃ©prÃ©ciation avec pÃ©riode de grÃ¢ce.

### VÃ©rification de compatibilitÃ©

**VÃ©rification effectuÃ©e :**
- âœ… CohÃ©rence avec Master Butler - Documentation Fondatrice : ConfirmÃ©e (flux de dÃ©claration, INV-MB-1, INV-MB-3, INV-MB-4, INV-MB-5)
- âœ… CohÃ©rence avec Operators et Terminologie : ConfirmÃ©e (dÃ©finition des OpÃ©rateurs, responsabilitÃ©s)
- âœ… CohÃ©rence avec Tools et Toolkits : ConfirmÃ©e (dÃ©claration des Tools et Toolkits)
- âœ… ConformitÃ© LOI-1 : ConfirmÃ©e (aucune dÃ©pendance externe)
- âœ… ConformitÃ© LOI-5 : ConfirmÃ©e (opÃ©rations lÃ©gÃ¨res, coÃ»t proportionnel)

**Conclusion :** Aucune contradiction dÃ©tectÃ©e. Le document est cohÃ©rent avec le systÃ¨me contractuel existant.

---

*Aucune autre erreur, warning, ou ambiguÃ¯tÃ© rencontrÃ©e lors de la rÃ©daction de ce document.*

