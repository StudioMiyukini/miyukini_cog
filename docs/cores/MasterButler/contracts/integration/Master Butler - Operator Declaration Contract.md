# Master Butler — Operator Declaration Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **Master Butler — Operator Declaration Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit les règles de déclaration des capacités et permissions par les Opérateurs auprès de Master Butler.

Ce contrat spécifie le protocole de déclaration, les formats requis, les règles de validation, les responsabilités des Opérateurs et de Master Butler, les invariants associés, et les garanties offertes par ce processus.

### Portée / Scope

Ce contrat s'applique à **toute déclaration de capacité ou de permission** effectuée par un Opérateur et définit de manière absolue :
- le protocole de déclaration des capacités,
- le protocole de définition des permissions,
- les formats et structures requis,
- les règles de validation des déclarations,
- les responsabilités des Opérateurs déclarants,
- les responsabilités de Master Butler lors de l'enregistrement,
- ce que la déclaration PEUT et NE PEUT JAMAIS faire,
- les invariants systémiques associés.

Ce document **ne couvre pas** :
- L'interrogation des capacités (voir [Capability API Contract](../api/Master%20Butler%20-%20Capability%20API%20Contract.md))
- L'interrogation des permissions (voir [Permission API Contract](../api/Master%20Butler%20-%20Permission%20API%20Contract.md))
- La découverte (voir [Discovery API Contract](../api/Master%20Butler%20-%20Discovery%20API%20Contract.md))
- L'intégration avec StrongFather (voir [StrongFather Integration Contract](./Master%20Butler%20-%20StrongFather%20Integration%20Contract.md))
- L'intégration avec BondingBrother (voir [BondingBrother Integration Contract](./Master%20Butler%20-%20BondingBrother%20Integration%20Contract.md))

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :
- **[Master Butler — Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md)** : Définition fondamentale du rôle de Master Butler
- **[Master Butler — Capability Registry Contract](../registry/Master%20Butler%20-%20Capability%20Registry%20Contract.md)** : Structure du registre des capacités
- **[Master Butler — Permission Registry Contract](../registry/Master%20Butler%20-%20Permission%20Registry%20Contract.md)** : Structure du registre des permissions
- **[Miyukini Conceptual References — Operators et Terminologie](../../../../reference/Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md)** : Définition canonique des Opérateurs
- **[Miyukini Conceptual References — Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)** : Terminologie officielle
- **[Miyukini Conceptual References — Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Conformité aux lois d'autonomie

Il n'introduit aucune contradiction avec le corpus documentaire existant.

---

## 2. Principes fondamentaux

### 2.1 Définition de la déclaration

> **Une déclaration est l'acte par lequel un Opérateur informe Master Butler des capacités qu'il expose et des permissions qu'il définit.**

La déclaration est un acte fondateur : sans déclaration, aucune capacité n'existe officiellement dans l'environnement. Master Butler est le réceptacle unique de ces déclarations (INV-MB-1 : Exhaustivité du registre).

### 2.2 Principe de déclaration obligatoire

> **Toute capacité exposée par un Opérateur DOIT être déclarée à Master Butler.**

Aucun Opérateur ne peut exposer une capacité sans la déclarer préalablement. Une capacité non déclarée n'existe pas dans l'écosystème Miyukini.

**Règle DECL-01 : Déclaration préalable obligatoire**

Un Opérateur NE PEUT PAS utiliser une capacité qu'il n'a pas déclarée à Master Butler. La déclaration précède toujours l'usage.

### 2.3 Principe de souveraineté applicative

> **Un environnement Miyukini possède une bibliothèque de capacités finie, déclarée, gouvernée.**

Ce principe est non négociable :

| Règle | Description |
|-------|-------------|
| **Pas d'injection sauvage** | Aucune capacité ne peut être ajoutée sans déclaration dans Master Butler |
| **Pas de capacité locale** | Toute capacité doit être déclarée dans l'environnement |
| **Pas de dépendance cachée** | Aucune capacité externe non gouvernée |

---

## 3. Types de déclarations

### 3.1 Déclaration de capacité

Une **déclaration de capacité** enregistre un pouvoir technique qu'un Opérateur possède.

**Structure de base :**

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| `capability_id` | string | ✅ Oui | Identifiant unique et stable de la capacité |
| `name` | string | ✅ Oui | Nom lisible de la capacité |
| `description` | string | ✅ Oui | Description fonctionnelle |
| `operator_id` | string | ✅ Oui | Identifiant de l'Opérateur déclarant |
| `module_origin` | string | ✅ Oui | Module d'origine de la capacité |
| `action_type` | string | ✅ Oui | Type d'action (create, read, update, delete, execute, etc.) |
| `target_type` | string | ✅ Oui | Type de ressource ciblée |
| `exposure_level` | enum | ✅ Oui | Niveau d'exposition (internal, operator, inter_cog, public) |
| `security_level` | enum | ✅ Oui | Niveau de sécurité requis (0-4) |
| `metadata` | object | ❌ Non | Métadonnées additionnelles |
| `dependencies` | array | ❌ Non | Capacités dont cette capacité dépend |
| `version` | string | ✅ Oui | Version de la capacité |

**Exemple de déclaration de capacité :**

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

### 3.2 Définition de permission

Une **définition de permission** crée un droit accordable pour accéder à une ou plusieurs capacités.

**Structure de base :**

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| `permission_id` | string | ✅ Oui | Identifiant unique de la permission |
| `name` | string | ✅ Oui | Nom lisible de la permission |
| `description` | string | ✅ Oui | Description de ce que la permission autorise |
| `operator_id` | string | ✅ Oui | Identifiant de l'Opérateur définissant |
| `associated_capabilities` | array | ✅ Oui | Liste des capacités couvertes (minimum 1) |
| `permission_level` | enum | ✅ Oui | Niveau de permission (basic, standard, elevated, admin) |
| `scope` | enum | ✅ Oui | Portée (own, team, all) |
| `conditions` | object | ❌ Non | Conditions d'application |
| `metadata` | object | ❌ Non | Métadonnées additionnelles |
| `version` | string | ✅ Oui | Version de la permission |

**Exemple de définition de permission :**

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

### 3.3 Déclaration de Tool

Une **déclaration de Tool** enregistre une capacité exécutable gouvernée.

**Structure de base :**

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| `tool_id` | string | ✅ Oui | Identifiant unique du Tool |
| `name` | string | ✅ Oui | Nom lisible du Tool |
| `description` | string | ✅ Oui | Description fonctionnelle |
| `operator_id` | string | ✅ Oui | Identifiant de l'Opérateur déclarant |
| `linked_capability` | string | ✅ Oui | Capacité liée au Tool |
| `input_schema` | object | ✅ Oui | Schéma des entrées |
| `output_schema` | object | ✅ Oui | Schéma des sorties |
| `security_level` | enum | ✅ Oui | Niveau de sécurité requis (0-4) |
| `idempotent` | boolean | ✅ Oui | Indique si le Tool est idempotent |
| `side_effects` | boolean | ✅ Oui | Indique si le Tool a des effets de bord |
| `metadata` | object | ❌ Non | Métadonnées additionnelles |
| `version` | string | ✅ Oui | Version du Tool |

### 3.4 Déclaration de Toolkit

Une **déclaration de Toolkit** enregistre une composition officielle de Tools.

**Structure de base :**

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| `toolkit_id` | string | ✅ Oui | Identifiant unique du Toolkit |
| `name` | string | ✅ Oui | Nom lisible du Toolkit |
| `description` | string | ✅ Oui | Description fonctionnelle |
| `operator_id` | string | ✅ Oui | Identifiant de l'Opérateur déclarant |
| `composed_tools` | array | ✅ Oui | Liste des Tools composant le Toolkit (minimum 2) |
| `orchestration_rules` | object | ❌ Non | Règles d'orchestration des Tools |
| `metadata` | object | ❌ Non | Métadonnées additionnelles |
| `version` | string | ✅ Oui | Version du Toolkit |

---

## 4. Protocole de déclaration

### 4.1 Flux de déclaration de capacité

**Acteurs :** Opérateur, BondingBrother (optionnel), Master Butler

**Séquence :**

```
┌─────────────────────────────────────────────────────────────────┐
│           FLUX DE DÉCLARATION DE CAPACITÉ                        │
│                                                                   │
│  OPÉRATEUR                                                       │
│      │                                                            │
│      │ 1. Prépare la déclaration de capacité                     │
│      │    - Identifiant unique                                   │
│      │    - Métadonnées complètes                                │
│      │    - Version                                              │
│      │                                                            │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  SOUMISSION À MASTER BUTLER                              │ │
│  │                                                            │ │
│  │  → declaration_api.declare_capability(declaration)        │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  MASTER BUTLER — VALIDATION                              │ │
│  │                                                            │ │
│  │  1. Validation structurelle (champs obligatoires)        │ │
│  │  2. Validation d'unicité (capability_id)                 │ │
│  │  3. Validation des dépendances (si présentes)            │ │
│  │  4. Validation de l'Opérateur déclarant                  │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  MASTER BUTLER — ENREGISTREMENT                          │ │
│  │                                                            │ │
│  │  Si validation OK :                                       │ │
│  │    - Enregistrement dans le registre                     │ │
│  │    - Journalisation de la déclaration                    │ │
│  │    - Retour : DECLARATION_ACCEPTED                       │ │
│  │                                                            │ │
│  │  Si validation KO :                                       │ │
│  │    - Rejet de la déclaration                             │ │
│  │    - Retour : DECLARATION_REJECTED + raison              │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      ▼                                                            │
│  OPÉRATEUR REÇOIT LA CONFIRMATION                               │
└─────────────────────────────────────────────────────────────────┘
```

### 4.2 Flux de définition de permission

**Acteurs :** Opérateur, Master Butler

**Séquence :**

```
┌─────────────────────────────────────────────────────────────────┐
│           FLUX DE DÉFINITION DE PERMISSION                       │
│                                                                   │
│  OPÉRATEUR                                                       │
│      │                                                            │
│      │ 1. Prépare la définition de permission                    │
│      │    - Identifiant unique                                   │
│      │    - Capacités associées (doivent exister)               │
│      │    - Métadonnées complètes                                │
│      │                                                            │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  SOUMISSION À MASTER BUTLER                              │ │
│  │                                                            │ │
│  │  → declaration_api.define_permission(definition)          │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  MASTER BUTLER — VALIDATION                              │ │
│  │                                                            │ │
│  │  1. Validation structurelle (champs obligatoires)        │ │
│  │  2. Validation d'unicité (permission_id)                 │ │
│  │  3. Validation des capacités associées (DOIVENT EXISTER) │ │
│  │  4. Validation de l'Opérateur définissant               │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  MASTER BUTLER — ENREGISTREMENT                          │ │
│  │                                                            │ │
│  │  Si validation OK :                                       │ │
│  │    - Enregistrement dans le registre                     │ │
│  │    - Création des associations capability-permission     │ │
│  │    - Journalisation de la définition                     │ │
│  │    - Retour : DEFINITION_ACCEPTED                        │ │
│  │                                                            │ │
│  │  Si validation KO :                                       │ │
│  │    - Rejet de la définition                              │ │
│  │    - Retour : DEFINITION_REJECTED + raison               │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      ▼                                                            │
│  OPÉRATEUR REÇOIT LA CONFIRMATION                               │
└─────────────────────────────────────────────────────────────────┘
```

### 4.3 Règle d'ordre de déclaration

**Règle DECL-02 : Dépendances préalables**

Les capacités DOIVENT être déclarées avant les permissions qui les référencent. Une permission ne peut pas référencer une capacité inexistante.

**Ordre obligatoire :**
1. Déclaration des capacités
2. Définition des permissions (référençant les capacités existantes)
3. Déclaration des Tools (liant aux capacités existantes)
4. Déclaration des Toolkits (composant des Tools existants)

---

## 5. Règles de validation

### 5.1 Validation structurelle

**Règle VAL-01 : Champs obligatoires**

Tous les champs marqués comme obligatoires DOIVENT être présents et non vides. Une déclaration incomplète est rejetée.

**Règle VAL-02 : Format des identifiants**

Les identifiants (`capability_id`, `permission_id`, `tool_id`, `toolkit_id`) DOIVENT respecter le format suivant :
- Caractères autorisés : `a-z`, `0-9`, `.`, `-`, `_`
- Pas d'espaces
- Longueur minimale : 3 caractères
- Longueur maximale : 128 caractères
- Format recommandé : `domain.action.scope` (ex: `content.create.own`)

### 5.2 Validation d'unicité

**Règle VAL-03 : Unicité des identifiants**

Les identifiants DOIVENT être uniques dans leur registre respectif. Une déclaration avec un identifiant déjà existant est traitée selon les règles d'idempotence (voir section 6).

### 5.3 Validation des références

**Règle VAL-04 : Existence des capacités référencées**

Une permission DOIT référencer au moins une capacité existante. Toutes les capacités référencées DOIVENT exister dans le registre.

**Règle VAL-05 : Existence des Tools référencés**

Un Toolkit DOIT référencer au moins deux Tools existants. Tous les Tools référencés DOIVENT exister dans le registre.

**Règle VAL-06 : Existence des dépendances**

Si une capacité déclare des dépendances, toutes les dépendances DOIVENT exister dans le registre.

### 5.4 Validation de l'Opérateur

**Règle VAL-07 : Opérateur reconnu**

L'Opérateur déclarant DOIT être un Opérateur reconnu dans l'environnement. Un Opérateur inconnu ne peut pas déclarer de capacités.

**Règle VAL-08 : Cohérence Opérateur-Module**

L'Opérateur déclarant DOIT être autorisé à déclarer des capacités pour le module d'origine spécifié.

---

## 6. Règles d'idempotence

### 6.1 Principe d'idempotence

**Règle IDEM-01 : Déclarations idempotentes**

Les déclarations de capacités sont idempotentes. Déclarer deux fois la même capacité avec les mêmes données n'a pas d'effet supplémentaire. Le registre reste cohérent quel que soit l'ordre ou le nombre de déclarations.

### 6.2 Comportement en cas de redéclaration

**Scénario 1 : Redéclaration identique**

Si une capacité est déclarée avec exactement les mêmes données qu'une capacité existante :
- Aucune modification du registre
- Retour : `DECLARATION_ACCEPTED` (déjà présente)

**Scénario 2 : Redéclaration avec différences**

Si une capacité est déclarée avec le même identifiant mais des données différentes :
- Rejet de la déclaration
- Retour : `DECLARATION_REJECTED` (conflit de version)
- L'Opérateur DOIT utiliser le protocole de mise à jour (voir section 8)

### 6.3 Implications pratiques

**Règle IDEM-02 : Redéclaration au démarrage**

Les Opérateurs PEUVENT redéclarer leurs capacités à chaque démarrage sans effet indésirable. Cette pratique est encouragée pour garantir la cohérence du registre.

---

## 7. Immutabilité des identifiants

### 7.1 Principe d'immutabilité

**Règle IMMUT-01 : Identifiants immuables**

Les identifiants de capacités sont immuables. Une fois qu'une capacité est déclarée avec un identifiant, cet identifiant ne change jamais.

### 7.2 Conséquences de l'immutabilité

| Situation | Action requise |
|-----------|----------------|
| Capacité évolue significativement | Créer une nouvelle capacité avec un nouvel identifiant |
| Correction mineure | Utiliser le protocole de mise à jour (même identifiant, nouvelle version) |
| Renommage fonctionnel | Créer une nouvelle capacité, déprécier l'ancienne |

**Règle IMMUT-02 : Stabilité des références**

Les références aux capacités (dans les permissions, les logs, les configurations) DOIVENT rester valides dans le temps grâce à l'immutabilité des identifiants.

---

## 8. Protocole de mise à jour

### 8.1 Mise à jour d'une capacité

Une capacité existante peut être mise à jour dans les limites suivantes :

**Champs modifiables :**
- `name` (avec contraintes)
- `description`
- `metadata`
- `version` (obligatoirement incrémentée)

**Champs NON modifiables :**
- `capability_id` (immutable)
- `operator_id` (propriétaire fixe)
- `module_origin` (fixé à la déclaration)
- `action_type` (changement = nouvelle capacité)
- `target_type` (changement = nouvelle capacité)

**Flux de mise à jour :**

```
Opérateur → Master Butler : update_capability(capability_id, updates)
Master Butler :
  1. Vérifie l'existence de la capacité
  2. Vérifie que l'Opérateur est le propriétaire
  3. Vérifie que seuls les champs autorisés sont modifiés
  4. Vérifie que la version est incrémentée
  5. Applique la mise à jour
  6. Journalise la modification
  7. Retourne : UPDATE_ACCEPTED
```

### 8.2 Dépréciation d'une capacité

Une capacité peut être dépréciée mais pas supprimée immédiatement.

**Flux de dépréciation :**

```
Opérateur → Master Butler : deprecate_capability(capability_id, reason, successor_id?)
Master Butler :
  1. Vérifie l'existence de la capacité
  2. Vérifie que l'Opérateur est le propriétaire
  3. Marque la capacité comme DEPRECATED
  4. Enregistre la raison et le successeur (si fourni)
  5. Journalise la dépréciation
  6. Retourne : DEPRECATION_ACCEPTED
```

**Règle DEPR-01 : Période de dépréciation**

Une capacité dépréciée reste fonctionnelle pendant une période de grâce définie. Les consommateurs sont avertis de migrer vers le successeur.

---

## 9. Responsabilités

### 9.1 Responsabilités de l'Opérateur déclarant

| Responsabilité | Description |
|----------------|-------------|
| **RESP-OP-1** | Déclarer toutes ses capacités avant de les exposer |
| **RESP-OP-2** | Fournir des identifiants uniques et stables |
| **RESP-OP-3** | Fournir des métadonnées complètes et exactes |
| **RESP-OP-4** | Maintenir la cohérence des déclarations |
| **RESP-OP-5** | Gérer les versions de ses capacités |
| **RESP-OP-6** | Déprécier proprement les capacités obsolètes |
| **RESP-OP-7** | Ne jamais exposer une capacité non déclarée |

### 9.2 Responsabilités de Master Butler

| Responsabilité | Description |
|----------------|-------------|
| **RESP-MB-1** | Valider toutes les déclarations selon les règles |
| **RESP-MB-2** | Enregistrer les déclarations validées |
| **RESP-MB-3** | Rejeter les déclarations non conformes |
| **RESP-MB-4** | Garantir l'exhaustivité du registre |
| **RESP-MB-5** | Journaliser toutes les déclarations |
| **RESP-MB-6** | Garantir l'idempotence des déclarations |
| **RESP-MB-7** | Préserver l'immutabilité des identifiants |

---

## 10. Ce que la déclaration PEUT faire

### 10.1 Opérations autorisées

**PEUT-DECL-1 : Enregistrer des capacités**

Un Opérateur PEUT déclarer des capacités qu'il expose légitimement.

**PEUT-DECL-2 : Définir des permissions**

Un Opérateur PEUT définir des permissions associées à ses capacités.

**PEUT-DECL-3 : Déclarer des Tools**

Un Opérateur PEUT déclarer des Tools liés à ses capacités.

**PEUT-DECL-4 : Déclarer des Toolkits**

Un Opérateur PEUT déclarer des Toolkits composés de ses Tools.

**PEUT-DECL-5 : Mettre à jour ses déclarations**

Un Opérateur PEUT mettre à jour ses capacités dans les limites définies.

**PEUT-DECL-6 : Déprécier ses capacités**

Un Opérateur PEUT déprécier ses capacités obsolètes.

**PEUT-DECL-7 : Redéclarer au démarrage**

Un Opérateur PEUT redéclarer ses capacités à chaque démarrage (idempotence).

---

## 11. Ce que la déclaration NE PEUT JAMAIS faire

### 11.1 Interdictions absolues

**INTERDIT-DECL-1 : Déclarer des capacités d'un autre Opérateur**

Un Opérateur NE PEUT JAMAIS déclarer des capacités appartenant à un autre Opérateur.

**INTERDIT-DECL-2 : Modifier l'identifiant d'une capacité**

Un Opérateur NE PEUT JAMAIS modifier l'identifiant d'une capacité existante. Les identifiants sont immuables.

**INTERDIT-DECL-3 : Référencer des capacités inexistantes**

Une permission NE PEUT JAMAIS référencer des capacités qui n'existent pas dans le registre.

**INTERDIT-DECL-4 : Supprimer une capacité directement**

Un Opérateur NE PEUT JAMAIS supprimer une capacité directement. Le processus de dépréciation → retrait est obligatoire.

**INTERDIT-DECL-5 : Contourner la validation**

Un Opérateur NE PEUT JAMAIS contourner les règles de validation de Master Butler.

**INTERDIT-DECL-6 : Exposer sans déclarer**

Un Opérateur NE PEUT JAMAIS exposer une capacité sans l'avoir préalablement déclarée.

**INTERDIT-DECL-7 : Modifier les métadonnées d'un autre**

Un Opérateur NE PEUT JAMAIS modifier les capacités ou permissions d'un autre Opérateur.

---

## 12. Invariants de déclaration

### 12.1 Invariants globaux

**INV-DECL-1 : Exhaustivité**

Toute capacité exposée dans l'environnement DOIT être présente dans le registre de Master Butler. Aucune capacité fantôme n'est autorisée.

**INV-DECL-2 : Propriété exclusive**

Chaque capacité appartient à un Opérateur unique. Seul le propriétaire peut modifier ou déprécier sa capacité.

**INV-DECL-3 : Idempotence garantie**

Les déclarations sont idempotentes. La même déclaration peut être effectuée plusieurs fois sans effet de bord.

**INV-DECL-4 : Immutabilité des identifiants**

Les identifiants de capacités ne changent jamais après leur création.

**INV-DECL-5 : Traçabilité complète**

Toute déclaration, modification, ou dépréciation est journalisée avec son contexte complet.

**INV-DECL-6 : Cohérence des références**

Toutes les références entre entités (permission → capacité, Tool → capacité, Toolkit → Tools) pointent vers des entités existantes.

**INV-DECL-7 : Ordre de dépendance**

Les entités dépendantes sont toujours créées après leurs dépendances (capacités avant permissions, Tools avant Toolkits).

---

## 13. Traçabilité des déclarations

### 13.1 Éléments à tracer

| Élément | Description |
|---------|-------------|
| `declaration_id` | Identifiant unique de la déclaration |
| `timestamp` | Horodatage de la déclaration |
| `operator_id` | Opérateur déclarant |
| `declaration_type` | Type (capability, permission, tool, toolkit) |
| `entity_id` | Identifiant de l'entité déclarée |
| `action` | Action (create, update, deprecate) |
| `previous_state` | État précédent (si modification) |
| `new_state` | Nouvel état |
| `validation_result` | Résultat de validation (accepted, rejected) |
| `rejection_reason` | Raison du rejet (si applicable) |

### 13.2 Rétention des traces

Les traces de déclaration sont conservées indéfiniment. Elles constituent l'historique officiel du registre.

---

## 14. Gestion des erreurs

### 14.1 Types de rejets

| Code | Signification | Action Opérateur |
|------|---------------|------------------|
| `MISSING_REQUIRED_FIELD` | Champ obligatoire manquant | Compléter la déclaration |
| `INVALID_IDENTIFIER` | Format d'identifiant invalide | Corriger le format |
| `DUPLICATE_IDENTIFIER` | Identifiant déjà utilisé | Choisir un autre identifiant ou vérifier l'idempotence |
| `UNKNOWN_CAPABILITY` | Capacité référencée inexistante | Déclarer la capacité d'abord |
| `UNKNOWN_TOOL` | Tool référencé inexistant | Déclarer le Tool d'abord |
| `UNAUTHORIZED_OPERATOR` | Opérateur non autorisé | Vérifier l'identité de l'Opérateur |
| `MODULE_MISMATCH` | Incohérence Opérateur-Module | Vérifier le module d'origine |
| `IMMUTABLE_FIELD_CHANGE` | Tentative de modification d'un champ immutable | Créer une nouvelle entité |

### 14.2 Principe de gestion

> **En cas d'erreur, la déclaration est rejetée entièrement. Aucune déclaration partielle n'est acceptée.**

---

## 15. Exemples complets

### 15.1 Déclaration complète d'un Opérateur CMS

```
// Étape 1 : Déclaration des capacités
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

// Étape 2 : Définition des permissions
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

// Étape 3 : Déclaration des Tools
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

## 16. Conformité aux Lois d'Autonomie

### 16.1 LOI-1 : Aucune dépendance externe

Le protocole de déclaration fonctionne entièrement en local. Aucune dépendance externe n'est requise pour déclarer, valider, ou enregistrer des capacités.

### 16.2 LOI-5 : Coût proportionnel

Les déclarations sont des opérations légères. Le coût de stockage et de traitement est proportionnel au nombre de capacités déclarées, qui reste borné et prévisible.

---

## 17. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable les règles de déclaration des capacités et permissions par les Opérateurs auprès de Master Butler.

Il garantit que :
- toute capacité est déclarée avant d'être exposée,
- les identifiants sont uniques et immuables,
- les déclarations sont idempotentes,
- la validation est stricte et complète,
- la traçabilité est assurée,
- les responsabilités sont clairement réparties,
- les invariants sont respectés.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

**Document créé le :** 2026-01-27  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, Master Butler Documentation Fondatrice v1.4, [Miyukini Conceptual References — Operators et Terminologie](../../../../reference/Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md), [Miyukini Conceptual References — Tools et Toolkits](../../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)  
**Type :** Contrat de déclaration non négociable

---

## 18. Mini log — erreurs / warnings / ambiguïtés rencontrées et corrigées

### Ambiguïté A1 : Ordre de déclaration des entités

**Ambiguïté rencontrée :** Risque de confusion sur l'ordre de déclaration entre capacités, permissions, Tools et Toolkits.

**Décision prise :** Section 4.3 définit explicitement l'ordre obligatoire : capacités → permissions → Tools → Toolkits.

**Correction effectuée :** Règle DECL-02 et section 4.3 rédigées avec ordre explicite.

### Ambiguïté A2 : Redéclaration avec différences vs mise à jour

**Ambiguïté rencontrée :** Confusion possible entre une redéclaration idempotente et une tentative de modification.

**Décision prise :** Section 6.2 distingue explicitement les deux scénarios : redéclaration identique (acceptée) vs redéclaration avec différences (rejetée, utiliser le protocole de mise à jour).

**Correction effectuée :** Section 6.2 et section 8 clarifient les deux cas.

### Ambiguïté A3 : Propriété des capacités

**Ambiguïté rencontrée :** Question sur qui peut modifier une capacité.

**Décision prise :** Invariant INV-DECL-2 établit la propriété exclusive : seul l'Opérateur propriétaire peut modifier ou déprécier sa capacité.

**Correction effectuée :** Invariant explicite ajouté, ainsi que les règles VAL-08 et INTERDIT-DECL-7.

### Warning W1 : Suppression directe de capacités

**Warning rencontré :** Risque d'incohérence si une capacité est supprimée alors que des permissions la référencent.

**Décision prise :** Interdiction INTERDIT-DECL-4 : aucune suppression directe. Le processus de dépréciation est obligatoire.

**Correction effectuée :** Section 8.2 définit le protocole de dépréciation avec période de grâce.

### Vérification de compatibilité

**Vérification effectuée :**
- ✅ Cohérence avec Master Butler - Documentation Fondatrice : Confirmée (flux de déclaration, INV-MB-1, INV-MB-3, INV-MB-4, INV-MB-5)
- ✅ Cohérence avec Operators et Terminologie : Confirmée (définition des Opérateurs, responsabilités)
- ✅ Cohérence avec Tools et Toolkits : Confirmée (déclaration des Tools et Toolkits)
- ✅ Conformité LOI-1 : Confirmée (aucune dépendance externe)
- ✅ Conformité LOI-5 : Confirmée (opérations légères, coût proportionnel)

**Conclusion :** Aucune contradiction détectée. Le document est cohérent avec le système contractuel existant.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
