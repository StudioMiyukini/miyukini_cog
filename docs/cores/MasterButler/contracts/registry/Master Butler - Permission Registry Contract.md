# Master Butler â€” Permission Registry Contract

## 1. Introduction

### Objet du contrat

Ce document dÃ©finit le **Master Butler Permission Registry Contract** : un contrat normatif, non nÃ©gociable, et de statut FONDATION qui Ã©tablit le modÃ¨le conceptuel, la structure, et les rÃ¨gles de gouvernance du registre des permissions dans le systÃ¨me Miyukini Core System v2.4.

Ce contrat Ã©tablit les fondations nÃ©cessaires pour comprendre comment les permissions sont dÃ©finies, organisÃ©es, associÃ©es aux capacitÃ©s, et gÃ©rÃ©es dans l'Ã©cosystÃ¨me Miyukini.

### PortÃ©e

Ce contrat s'applique Ã  **toutes les permissions** du systÃ¨me et dÃ©finit de maniÃ¨re absolue :

- La dÃ©finition formelle d'une permission
- La structure du registre des permissions
- Le modÃ¨le d'association permission-capacitÃ©
- Les rÃ¨gles de dÃ©finition, modification et rÃ©vocation
- Les mÃ©tadonnÃ©es obligatoires et optionnelles
- Les invariants non nÃ©gociables du registre
- Les interactions avec les autres composants

Ce contrat se concentre exclusivement sur les concepts du registre des permissions, sans entrer dans les dÃ©tails d'implÃ©mentation technique ou les mÃ©canismes de vÃ©rification runtime.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il Ã©tablit des dÃ©finitions absolues et stables qui ne peuvent Ãªtre contournÃ©es, nÃ©gociÃ©es, ou modifiÃ©es. Le contrat prime sur toute considÃ©ration pratique.

### Relation avec les autres contrats

Ce contrat complÃ¨te les documents contractuels existants :

- **Master Butler â€” Documentation Fondatrice** : DÃ©finit la raison d'Ãªtre et les responsabilitÃ©s de Master Butler
- **Master Butler â€” Capability Registry Contract** : DÃ©finit le registre des capacitÃ©s (complÃ©mentaire)
- **Master Butler â€” Association Model Contract** : DÃ©finit les associations entre permissions, rÃ´les et capacitÃ©s
- **[Miyukini Conceptual References â€” Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)** : DÃ©finitions canoniques des termes
- **[Miyukini Conceptual References â€” Tools et Toolkits](..//..//..//..//miyukini-webway-system//reference//_index.md)** : Gouvernance des Outils et Kits d'Outils
- **[Miyukini Conceptual References â€” Lois Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md)** : Ce contrat respecte **LOI-1** (aucune dÃ©pendance externe critique) et **LOI-5** (coÃ»t proportionnel au hardware) en garantissant un registre local et lÃ©ger

**ComplÃ©mentaritÃ© :**

- Capability Registry Contract = ce qui est techniquement possible
- Permission Registry Contract = les droits dÃ©finis pour accÃ©der aux capacitÃ©s
- Association Model Contract = liens entre permissions, capacitÃ©s et rÃ´les

Ces contrats forment ensemble le systÃ¨me complet de gestion des capacitÃ©s et permissions du systÃ¨me Miyukini Core System v2.4.

---

## 2. DÃ©finition formelle d'une permission

### DÃ©finition canonique

Une **permission** est un droit formellement dÃ©fini dans le systÃ¨me pour accÃ©der Ã  une ou plusieurs capacitÃ©s. Elle reprÃ©sente l'autorisation conceptuelle d'utiliser des capacitÃ©s, mais ne garantit pas l'autorisation finale (qui dÃ©pend de StrongFather).

**Phrase fondatrice :**

> **Une permission dÃ©finit ce que le systÃ¨me reconnaÃ®t comme un droit possible, pas ce qui est effectivement autorisÃ©.**

### CaractÃ©ristiques fondamentales d'une permission

| CaractÃ©ristique | Description | Obligatoire |
|-----------------|-------------|-------------|
| **IdentifiÃ©e** | PossÃ¨de un identifiant unique et stable | âœ… Oui |
| **NommÃ©e** | PossÃ¨de un nom lisible et descriptif | âœ… Oui |
| **DÃ©finie** | Est crÃ©Ã©e explicitement, jamais implicite | âœ… Oui |
| **AssociÃ©e** | RÃ©fÃ©rence au moins une capacitÃ© existante | âœ… Oui |
| **DocumentÃ©e** | PossÃ¨de des mÃ©tadonnÃ©es descriptives | âœ… Oui |
| **Attribuable** | Peut Ãªtre accordÃ©e Ã  des rÃ´les ou contextes | âœ… Oui |
| **RÃ©vocable** | Peut Ãªtre retirÃ©e | âœ… Oui |
| **TraÃ§able** | Son historique est enregistrÃ© | âœ… Oui |

### Nature conceptuelle

Une permission est un **concept de droit**, pas un mÃ©canisme de vÃ©rification. Elle dÃ©finit ce qui peut Ãªtre accordÃ©, pas ce qui est effectivement vÃ©rifiÃ© Ã  l'exÃ©cution.

**Important :** La vÃ©rification effective d'une permission appartient Ã  StrongFather lors de l'Ã©valuation des intentions. Master Butler ne vÃ©rifie jamais si une permission est accordÃ©e Ã  un contexte donnÃ© â€” il fournit les dÃ©finitions, pas les vÃ©rifications.

### Distinction permission vs autorisation

| Aspect | Permission | Autorisation |
|--------|------------|--------------|
| **DÃ©finition** | Droit dÃ©fini dans le systÃ¨me | DÃ©cision d'accorder ce droit |
| **Responsable** | Master Butler | StrongFather |
| **Nature** | Statique (existe ou n'existe pas) | Dynamique (accordÃ©e ou refusÃ©e selon contexte) |
| **Question** | "Ce droit existe-t-il ?" | "Ce droit est-il accordÃ© ici et maintenant ?" |

---

## 3. Structure du registre des permissions

### DÃ©finition du registre

Le **registre des permissions** est la structure centrale de Master Butler qui contient l'inventaire exhaustif de toutes les permissions dÃ©finies dans le systÃ¨me.

**CaractÃ©ristiques du registre :**

| PropriÃ©tÃ© | Description |
|-----------|-------------|
| **Central** | Source unique de vÃ©ritÃ© pour les permissions |
| **Exhaustif** | Contient toutes les permissions, sans exception |
| **Dynamique** | Ã‰volue avec les dÃ©finitions de permissions |
| **TraÃ§able** | Historise toutes les modifications |
| **CohÃ©rent** | Maintient l'intÃ©gritÃ© rÃ©fÃ©rentielle |

### Organisation du registre

Le registre est organisÃ© selon une structure hiÃ©rarchique logique :

```
Registre des Permissions
â”œâ”€â”€ Domaine (domain)
â”‚   â”œâ”€â”€ Sous-domaine (subdomain)
â”‚   â”‚   â”œâ”€â”€ Permission 1
â”‚   â”‚   â”œâ”€â”€ Permission 2
â”‚   â”‚   â””â”€â”€ ...
â”‚   â””â”€â”€ ...
â””â”€â”€ ...
```

**Convention de nommage des identifiants :**

```
<domaine>.<sous-domaine>.<action>.<portÃ©e>
```

**Exemples :**

| Identifiant | Domaine | Sous-domaine | Action | PortÃ©e |
|-------------|---------|--------------|--------|--------|
| `content.article.create.any` | content | article | create | any |
| `content.article.edit.own` | content | article | edit | own |
| `media.image.delete.all` | media | image | delete | all |
| `hierarchy.tree.reorder.scope` | hierarchy | tree | reorder | scope |

### EntrÃ©e du registre

Chaque entrÃ©e du registre reprÃ©sente une permission et contient :

```
Permission Entry
â”œâ”€â”€ Identification
â”‚   â”œâ”€â”€ id: <identifiant unique>
â”‚   â”œâ”€â”€ name: <nom lisible>
â”‚   â””â”€â”€ version: <version de la dÃ©finition>
â”œâ”€â”€ DÃ©finition
â”‚   â”œâ”€â”€ description: <description dÃ©taillÃ©e>
â”‚   â”œâ”€â”€ domain: <domaine fonctionnel>
â”‚   â”œâ”€â”€ level: <niveau de criticitÃ©>
â”‚   â””â”€â”€ scope_type: <type de portÃ©e>
â”œâ”€â”€ Associations
â”‚   â”œâ”€â”€ capabilities: [<liste des capacitÃ©s couvertes>]
â”‚   â””â”€â”€ implied_permissions: [<permissions impliquÃ©es>]
â”œâ”€â”€ MÃ©tadonnÃ©es
â”‚   â”œâ”€â”€ created_at: <timestamp crÃ©ation>
â”‚   â”œâ”€â”€ created_by: <identitÃ© crÃ©ateur>
â”‚   â”œâ”€â”€ modified_at: <timestamp derniÃ¨re modification>
â”‚   â””â”€â”€ modified_by: <identitÃ© modificateur>
â””â”€â”€ Ã‰tat
    â”œâ”€â”€ status: <DRAFT | ACTIVE | DEPRECATED | RETIRED>
    â””â”€â”€ deprecation_info: <informations de dÃ©prÃ©ciation si applicable>
```

---

## 4. ModÃ¨le de donnÃ©es d'une permission

### Champs d'identification (obligatoires)

| Champ | Type | Description | Contraintes |
|-------|------|-------------|-------------|
| `id` | String | Identifiant unique et immuable | Format : `domain.subdomain.action.scope`, non modifiable aprÃ¨s crÃ©ation |
| `name` | String | Nom lisible et descriptif | Non vide, max 128 caractÃ¨res |
| `version` | String | Version sÃ©mantique de la dÃ©finition | Format : `major.minor.patch` |

### Champs de dÃ©finition (obligatoires)

| Champ | Type | Description | Contraintes |
|-------|------|-------------|-------------|
| `description` | String | Description dÃ©taillÃ©e du droit accordÃ© | Non vide, min 20 caractÃ¨res |
| `domain` | String | Domaine fonctionnel | Valeur du catalogue de domaines |
| `level` | Enum | Niveau de criticitÃ© | `STANDARD`, `ELEVATED`, `CRITICAL`, `SYSTEM` |
| `scope_type` | Enum | Type de portÃ©e | `GLOBAL`, `SCOPED`, `OWNED`, `CONTEXTUAL` |

### Champs d'association (obligatoires)

| Champ | Type | Description | Contraintes |
|-------|------|-------------|-------------|
| `capabilities` | Array[String] | CapacitÃ©s couvertes par cette permission | Au moins une capacitÃ©, toutes doivent exister dans le Capability Registry |

### Champs d'association (optionnels)

| Champ | Type | Description | Contraintes |
|-------|------|-------------|-------------|
| `implied_permissions` | Array[String] | Permissions impliquÃ©es (hiÃ©rarchie) | Toutes doivent exister, pas de cycle |
| `required_permissions` | Array[String] | Permissions prÃ©requises | Toutes doivent exister |
| `conflicting_permissions` | Array[String] | Permissions incompatibles | Toutes doivent exister |

### Champs de mÃ©tadonnÃ©es (automatiques)

| Champ | Type | Description | Contraintes |
|-------|------|-------------|-------------|
| `created_at` | Timestamp | Date de crÃ©ation | GÃ©nÃ©rÃ© automatiquement, immuable |
| `created_by` | String | IdentitÃ© du crÃ©ateur | TracÃ© automatiquement |
| `modified_at` | Timestamp | Date de derniÃ¨re modification | Mis Ã  jour automatiquement |
| `modified_by` | String | IdentitÃ© du modificateur | TracÃ© automatiquement |

### Champs d'Ã©tat

| Champ | Type | Description | Contraintes |
|-------|------|-------------|-------------|
| `status` | Enum | Ã‰tat du cycle de vie | `DRAFT`, `ACTIVE`, `DEPRECATED`, `RETIRED` |
| `deprecation_date` | Timestamp | Date de dÃ©prÃ©ciation | Requis si status = DEPRECATED |
| `deprecation_reason` | String | Raison de dÃ©prÃ©ciation | Requis si status = DEPRECATED |
| `successor_id` | String | Permission de remplacement | RecommandÃ© si status = DEPRECATED |

---

## 5. Niveaux de criticitÃ© des permissions

### DÃ©finition des niveaux

Les permissions sont classÃ©es selon leur niveau de criticitÃ©, qui dÃ©termine les contrÃ´les et validations applicables.

| Niveau | Nom | Description | Validation requise |
|--------|-----|-------------|-------------------|
| `STANDARD` | Standard | Permissions courantes, risque faible | Validation normale |
| `ELEVATED` | Ã‰levÃ© | Permissions sensibles, risque modÃ©rÃ© | Validation renforcÃ©e |
| `CRITICAL` | Critique | Permissions critiques, risque Ã©levÃ© | Validation stricte + audit |
| `SYSTEM` | SystÃ¨me | Permissions systÃ¨me, usage exceptionnel | Validation systÃ¨me + MiyukiniAdmin |

### CaractÃ©ristiques par niveau

#### STANDARD

| Aspect | RÃ¨gle |
|--------|-------|
| **Attribution** | Par les rÃ´les standards |
| **Audit** | Trace standard |
| **RÃ©vocation** | ProcÃ©dure normale |
| **Exemples** | `content.read.own`, `media.view.public` |

#### ELEVATED

| Aspect | RÃ¨gle |
|--------|-------|
| **Attribution** | Par les rÃ´les avec autoritÃ© Ã©levÃ©e |
| **Audit** | Trace dÃ©taillÃ©e |
| **RÃ©vocation** | ProcÃ©dure avec justification |
| **Exemples** | `content.delete.scope`, `user.invite.team` |

#### CRITICAL

| Aspect | RÃ¨gle |
|--------|-------|
| **Attribution** | Par StrongFather avec validation explicite |
| **Audit** | Trace complÃ¨te + alerte WorrySentinel |
| **RÃ©vocation** | ProcÃ©dure formelle avec approbation |
| **Exemples** | `data.export.all`, `hierarchy.restructure.global` |

#### SYSTEM

| Aspect | RÃ¨gle |
|--------|-------|
| **Attribution** | Uniquement par MiyukiniAdmin |
| **Audit** | Trace systÃ¨me inviolable |
| **RÃ©vocation** | ProcÃ©dure d'urgence uniquement |
| **Exemples** | `system.core.access`, `admin.override.security` |

---

## 6. Types de portÃ©e des permissions

### DÃ©finition des types de portÃ©e

La portÃ©e d'une permission dÃ©finit l'Ã©tendue sur laquelle le droit s'applique.

| Type | Nom | Description |
|------|-----|-------------|
| `GLOBAL` | Globale | S'applique Ã  toutes les entitÃ©s du domaine |
| `SCOPED` | DÃ©limitÃ©e | S'applique Ã  un pÃ©rimÃ¨tre dÃ©fini (Ã©quipe, projet, etc.) |
| `OWNED` | PropriÃ©taire | S'applique uniquement aux entitÃ©s possÃ©dÃ©es par le contexte |
| `CONTEXTUAL` | Contextuelle | S'applique selon des conditions contextuelles dynamiques |

### Exemples par type de portÃ©e

#### GLOBAL

```yaml
permission:
  id: "admin.user.manage.global"
  scope_type: GLOBAL
  description: "Gestion de tous les utilisateurs du systÃ¨me"
  # S'applique Ã  TOUS les utilisateurs, sans restriction
```

#### SCOPED

```yaml
permission:
  id: "content.article.edit.team"
  scope_type: SCOPED
  description: "Modification des articles de l'Ã©quipe"
  # S'applique aux articles dans le pÃ©rimÃ¨tre de l'Ã©quipe du contexte
```

#### OWNED

```yaml
permission:
  id: "content.draft.delete.own"
  scope_type: OWNED
  description: "Suppression de ses propres brouillons"
  # S'applique uniquement aux brouillons crÃ©Ã©s par le contexte
```

#### CONTEXTUAL

```yaml
permission:
  id: "workflow.task.approve.assigned"
  scope_type: CONTEXTUAL
  description: "Approbation des tÃ¢ches assignÃ©es"
  # S'applique selon des conditions Ã©valuÃ©es dynamiquement
```

---

## 7. Associations permission-capacitÃ©

### Principe fondamental

Une permission est **toujours associÃ©e** Ã  une ou plusieurs capacitÃ©s. Cette association dÃ©finit quelles capacitÃ©s sont "couvertes" par la permission.

**RÃ¨gle absolue :**

> **Une permission sans capacitÃ© associÃ©e est invalide. Une permission doit rÃ©fÃ©rencer au moins une capacitÃ© existante.**

### Types d'association

| Type | Description | Exemple |
|------|-------------|---------|
| **Directe** | Une permission couvre exactement une capacitÃ© | `content.create.any` â†’ `content.create` |
| **Multiple** | Une permission couvre plusieurs capacitÃ©s | `content.manage.all` â†’ `content.create`, `content.edit`, `content.delete` |
| **HiÃ©rarchique** | Une permission implique d'autres permissions | `admin.content.full` implique `content.manage.all` |

### ModÃ¨le d'association

```
Permission
    â”‚
    â”œâ”€â”€ capabilities (association directe)
    â”‚   â”œâ”€â”€ capability_id_1
    â”‚   â”œâ”€â”€ capability_id_2
    â”‚   â””â”€â”€ ...
    â”‚
    â””â”€â”€ implied_permissions (association hiÃ©rarchique)
        â”œâ”€â”€ permission_id_1 (qui a ses propres capabilities)
        â””â”€â”€ permission_id_2 (qui a ses propres capabilities)
```

### RÃ¨gles d'association

| RÃ¨gle | Description | Statut |
|-------|-------------|--------|
| **REG-PERM-ASSOC-1** | Toute permission doit rÃ©fÃ©rencer au moins une capacitÃ© | NON NÃ‰GOCIABLE |
| **REG-PERM-ASSOC-2** | Toute capacitÃ© rÃ©fÃ©rencÃ©e doit exister dans le Capability Registry | NON NÃ‰GOCIABLE |
| **REG-PERM-ASSOC-3** | Les associations impliquÃ©es ne doivent pas crÃ©er de cycle | NON NÃ‰GOCIABLE |
| **REG-PERM-ASSOC-4** | La suppression d'une capacitÃ© invalide les permissions associÃ©es | NON NÃ‰GOCIABLE |
| **REG-PERM-ASSOC-5** | L'ajout d'une association est une modification tracÃ©e | NON NÃ‰GOCIABLE |

### RÃ©solution des capacitÃ©s effectives

Lorsqu'une permission est interrogÃ©e, les capacitÃ©s effectives incluent :

1. Les capacitÃ©s directement associÃ©es (`capabilities`)
2. Les capacitÃ©s des permissions impliquÃ©es (`implied_permissions`), rÃ©cursivement
3. L'union de toutes ces capacitÃ©s, sans duplication

**Exemple :**

```
Permission: admin.content.full
â”œâ”€â”€ capabilities: []
â””â”€â”€ implied_permissions:
    â””â”€â”€ content.manage.all
        â”œâ”€â”€ capabilities: []
        â””â”€â”€ implied_permissions:
            â”œâ”€â”€ content.create.any
            â”‚   â””â”€â”€ capabilities: [content.create]
            â”œâ”€â”€ content.edit.any
            â”‚   â””â”€â”€ capabilities: [content.edit]
            â””â”€â”€ content.delete.any
                â””â”€â”€ capabilities: [content.delete]

CapacitÃ©s effectives de admin.content.full:
[content.create, content.edit, content.delete]
```

---

## 8. Cycle de vie d'une permission

### Ã‰tats du cycle de vie

Une permission passe par des Ã©tats de cycle de vie dÃ©finis, gÃ©rÃ©s en cohÃ©rence avec Ever Buddy.

| Ã‰tat | Description | Utilisation |
|------|-------------|-------------|
| `DRAFT` | En cours de dÃ©finition | Non utilisable en production |
| `ACTIVE` | Active et utilisable | Utilisation normale |
| `DEPRECATED` | DÃ©prÃ©ciÃ©e, usage dÃ©couragÃ© | PÃ©riode de transition |
| `RETIRED` | RetirÃ©e du systÃ¨me | Non disponible |

### Transitions autorisÃ©es

```
DRAFT â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º RETIRED
  â”‚                                                                  â–²
  â”‚ activation                                                       â”‚
  â–¼                                                                  â”‚
ACTIVE â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º DEPRECATED â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
           dÃ©prÃ©ciation           retrait
```

| Transition | Conditions | Actions |
|------------|------------|---------|
| DRAFT â†’ ACTIVE | Validation complÃ¨te, associations valides | Enregistrement, notification |
| DRAFT â†’ RETIRED | Abandon de la dÃ©finition | Suppression du draft |
| ACTIVE â†’ DEPRECATED | Justification obligatoire, successeur recommandÃ© | Notification des consommateurs |
| DEPRECATED â†’ RETIRED | PÃ©riode de dÃ©prÃ©ciation Ã©coulÃ©e | Invalidation, archivage |

### RÃ¨gles de cycle de vie

| RÃ¨gle | Description | Statut |
|-------|-------------|--------|
| **REG-PERM-LIFE-1** | Une permission ne peut Ãªtre retirÃ©e sans passer par DEPRECATED (sauf DRAFT) | NON NÃ‰GOCIABLE |
| **REG-PERM-LIFE-2** | La pÃ©riode de dÃ©prÃ©ciation minimale est dÃ©finie par politique | NON NÃ‰GOCIABLE |
| **REG-PERM-LIFE-3** | Toute transition est tracÃ©e avec contexte complet | NON NÃ‰GOCIABLE |
| **REG-PERM-LIFE-4** | Les permissions RETIRED restent dans l'historique | NON NÃ‰GOCIABLE |

---

## 9. OpÃ©rations sur le registre

### 9.1. DÃ©finition d'une permission

**Acteurs :** OpÃ©rateur, BondingBrother, Master Butler

**SÃ©quence :**

1. L'OpÃ©rateur soumet une dÃ©finition de permission
2. BondingBrother traduit et transmet Ã  Master Butler
3. Master Butler valide la structure de la dÃ©finition
4. Master Butler vÃ©rifie l'existence des capacitÃ©s rÃ©fÃ©rencÃ©es
5. Master Butler vÃ©rifie l'absence de cycle dans les implications
6. Master Butler enregistre la permission en Ã©tat DRAFT
7. Master Butler confirme l'enregistrement

**Validations obligatoires :**

| Validation | Description | Erreur si Ã©choue |
|------------|-------------|------------------|
| Structure valide | Tous les champs obligatoires prÃ©sents | `INVALID_PERMISSION_STRUCTURE` |
| Identifiant unique | L'identifiant n'existe pas dÃ©jÃ  | `DUPLICATE_PERMISSION_ID` |
| CapacitÃ©s existantes | Toutes les capacitÃ©s rÃ©fÃ©rencÃ©es existent | `UNKNOWN_CAPABILITY` |
| Pas de cycle | Les implications ne crÃ©ent pas de cycle | `CYCLIC_IMPLICATION` |
| Niveau autorisÃ© | Le crÃ©ateur peut crÃ©er ce niveau | `UNAUTHORIZED_LEVEL` |

### 9.2. Activation d'une permission

**Acteurs :** OpÃ©rateur autorisÃ©, Master Butler

**SÃ©quence :**

1. L'OpÃ©rateur demande l'activation d'une permission DRAFT
2. Master Butler vÃ©rifie que la permission est en Ã©tat DRAFT
3. Master Butler vÃ©rifie que toutes les validations sont satisfaites
4. Master Butler change l'Ã©tat Ã  ACTIVE
5. Master Butler notifie les composants concernÃ©s
6. Master Butler confirme l'activation

**Conditions d'activation :**

- Ã‰tat actuel = DRAFT
- Toutes les capacitÃ©s rÃ©fÃ©rencÃ©es sont ACTIVE
- Toutes les permissions impliquÃ©es sont ACTIVE
- L'OpÃ©rateur a l'autoritÃ© d'activer ce niveau de permission

### 9.3. Modification d'une permission

**RÃ¨gles de modification :**

| Champ | Modifiable en DRAFT | Modifiable en ACTIVE | Modifiable en DEPRECATED |
|-------|---------------------|----------------------|--------------------------|
| `id` | âŒ Non | âŒ Non | âŒ Non |
| `name` | âœ… Oui | âš ï¸ Avec version | âŒ Non |
| `description` | âœ… Oui | âš ï¸ Avec version | âŒ Non |
| `capabilities` | âœ… Oui | âš ï¸ Avec version | âŒ Non |
| `implied_permissions` | âœ… Oui | âš ï¸ Avec version | âŒ Non |
| `level` | âœ… Oui | âŒ Non | âŒ Non |
| `scope_type` | âœ… Oui | âŒ Non | âŒ Non |

**âš ï¸ Avec version** : La modification incrÃ©mente la version mineure et est tracÃ©e.

### 9.4. DÃ©prÃ©ciation d'une permission

**Acteurs :** OpÃ©rateur autorisÃ©, Master Butler

**SÃ©quence :**

1. L'OpÃ©rateur demande la dÃ©prÃ©ciation avec justification
2. Master Butler vÃ©rifie que la permission est ACTIVE
3. Master Butler enregistre la raison et la date de dÃ©prÃ©ciation
4. Master Butler change l'Ã©tat Ã  DEPRECATED
5. Master Butler notifie les consommateurs de la permission
6. Master Butler confirme la dÃ©prÃ©ciation

**Informations requises :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `deprecation_reason` | Raison de la dÃ©prÃ©ciation | âœ… Oui |
| `deprecation_date` | Date effective de dÃ©prÃ©ciation | âœ… Oui (auto si non fourni) |
| `successor_id` | Permission de remplacement | RecommandÃ© |
| `migration_guide` | Guide de migration | RecommandÃ© |

### 9.5. Retrait d'une permission

**Acteurs :** Master Butler (automatique ou manuel)

**Conditions de retrait :**

- Ã‰tat actuel = DEPRECATED
- PÃ©riode de dÃ©prÃ©ciation minimale Ã©coulÃ©e
- Aucune attribution active (ou migration forcÃ©e)

**ConsÃ©quences du retrait :**

- La permission devient inutilisable
- Les rÃ©fÃ©rences existantes deviennent invalides
- L'historique est conservÃ©
- Les attributions sont rÃ©voquÃ©es

---

## 10. Interrogation du registre

### Types de requÃªtes

Master Butler expose les requÃªtes suivantes sur le registre des permissions :

| RequÃªte | Description | ParamÃ¨tres |
|---------|-------------|------------|
| `getPermission` | RÃ©cupÃ¨re une permission par identifiant | `permission_id` |
| `listPermissions` | Liste les permissions selon critÃ¨res | `domain`, `level`, `status`, `scope_type` |
| `getPermissionCapabilities` | RÃ©cupÃ¨re les capacitÃ©s d'une permission | `permission_id`, `include_implied` |
| `searchPermissions` | Recherche par nom ou description | `query`, `filters` |
| `getPermissionHierarchy` | RÃ©cupÃ¨re la hiÃ©rarchie d'implications | `permission_id` |
| `validatePermission` | Valide une dÃ©finition de permission | `permission_definition` |

### RÃ©ponses standardisÃ©es

Toutes les rÃ©ponses incluent :

```yaml
response:
  success: <boolean>
  data: <donnÃ©es demandÃ©es>
  metadata:
    request_id: <identifiant de requÃªte>
    timestamp: <timestamp de rÃ©ponse>
    source: "MasterButler.PermissionRegistry"
  errors: [<liste d'erreurs si success = false>]
```

### Filtrage et pagination

Les requÃªtes de liste supportent :

| ParamÃ¨tre | Type | Description |
|-----------|------|-------------|
| `domain` | String | Filtrer par domaine |
| `level` | Enum[] | Filtrer par niveaux |
| `status` | Enum[] | Filtrer par Ã©tats |
| `scope_type` | Enum[] | Filtrer par types de portÃ©e |
| `offset` | Integer | DÃ©calage pour pagination |
| `limit` | Integer | Nombre maximum de rÃ©sultats |
| `sort_by` | String | Champ de tri |
| `sort_order` | Enum | `ASC` ou `DESC` |

---

## 11. Invariants non nÃ©gociables

### INV-PERM-REG-1 : ExhaustivitÃ©

Le registre des permissions est **exhaustif**. Toute permission existant dans le systÃ¨me est recensÃ©e dans le registre. Si une permission n'est pas dans le registre, elle n'existe pas officiellement dans le systÃ¨me.

**Implication :** Aucun composant ne peut reconnaÃ®tre une permission non enregistrÃ©e. Aucune attribution ne peut rÃ©fÃ©rencer une permission inexistante.

### INV-PERM-REG-2 : UnicitÃ© des identifiants

Chaque permission possÃ¨de un **identifiant unique et immuable**. Aucun doublon n'est autorisÃ©. L'identifiant ne peut jamais Ãªtre modifiÃ© aprÃ¨s crÃ©ation.

**Implication :** Les rÃ©fÃ©rences aux permissions restent valides dans le temps. Les logs et audits peuvent toujours identifier une permission de maniÃ¨re non ambiguÃ«.

### INV-PERM-REG-3 : Association obligatoire

Toute permission doit Ãªtre **associÃ©e Ã  au moins une capacitÃ© existante**. Une permission sans capacitÃ© est invalide et ne peut Ãªtre activÃ©e.

**Implication :** La suppression d'une capacitÃ© rend invalides les permissions qui ne rÃ©fÃ©rencent que cette capacitÃ©. Ces permissions doivent Ãªtre mises Ã  jour ou dÃ©prÃ©ciÃ©es.

### INV-PERM-REG-4 : IntÃ©gritÃ© rÃ©fÃ©rentielle

Toutes les rÃ©fÃ©rences dans le registre sont **valides et vÃ©rifiÃ©es**. Les capacitÃ©s rÃ©fÃ©rencÃ©es existent. Les permissions impliquÃ©es existent. Les successeurs rÃ©fÃ©rencÃ©s existent.

**Implication :** Le registre ne contient jamais de rÃ©fÃ©rence vers un Ã©lÃ©ment inexistant. Toute opÃ©ration qui crÃ©erait une rÃ©fÃ©rence invalide est rejetÃ©e.

### INV-PERM-REG-5 : Absence de cycle

Les **implications de permissions ne crÃ©ent jamais de cycle**. Une permission ne peut pas s'impliquer elle-mÃªme, directement ou indirectement.

**Implication :** La rÃ©solution des capacitÃ©s effectives termine toujours. Aucune boucle infinie n'est possible.

### INV-PERM-REG-6 : TraÃ§abilitÃ© complÃ¨te

Toute modification du registre est **tracÃ©e avec contexte complet**. CrÃ©ations, modifications, dÃ©prÃ©ciations, retraits : tout est enregistrÃ© avec qui, quand, pourquoi.

**Implication :** L'historique des permissions est auditable. Aucune modification silencieuse n'est possible. La conformitÃ© peut Ãªtre vÃ©rifiÃ©e.

### INV-PERM-REG-7 : Non-vÃ©rification

Master Butler **ne vÃ©rifie jamais** si une permission est effectivement accordÃ©e Ã  un contexte. Il dÃ©finit ce qui existe, pas ce qui est autorisÃ©.

**Implication :** Aucune mÃ©thode du registre ne retourne "accordÃ©" ou "refusÃ©". Ces dÃ©cisions appartiennent Ã  StrongFather.

### INV-PERM-REG-8 : CohÃ©rence des Ã©tats

Les **transitions d'Ã©tat suivent un chemin dÃ©fini**. Aucune transition arbitraire n'est autorisÃ©e. Les rÃ¨gles de cycle de vie sont strictement appliquÃ©es.

**Implication :** Une permission RETIRED ne peut pas redevenir ACTIVE. Une permission ACTIVE ne peut pas redevenir DRAFT.

---

## 12. Interactions avec les autres composants

### 12.1. Interaction avec StrongFather

**Flux typique :**

```
StrongFather Ã©value une intention
    â”‚
    â”œâ”€â”€ Interroge Master Butler : "Quelles permissions couvrent cette capacitÃ© ?"
    â”‚       â”‚
    â”‚       â””â”€â”€ Master Butler rÃ©pond : Liste des permissions
    â”‚
    â”œâ”€â”€ Interroge Master Butler : "Quelle est la dÃ©finition de cette permission ?"
    â”‚       â”‚
    â”‚       â””â”€â”€ Master Butler rÃ©pond : DÃ©finition complÃ¨te
    â”‚
    â””â”€â”€ StrongFather dÃ©cide selon les politiques
```

**RÃ¨gles d'interaction :**

- StrongFather est toujours autorisÃ© Ã  interroger le registre
- Master Butler ne suggÃ¨re jamais de dÃ©cision
- Les rÃ©ponses sont exhaustives et exactes
- Aucun cache de dÃ©cision dans Master Butler

### 12.2. Interaction avec BondingBrother

**Flux typique :**

```
BondingBrother traduit une intention
    â”‚
    â”œâ”€â”€ Interroge Master Butler : "Quelles permissions sont requises pour cette action ?"
    â”‚       â”‚
    â”‚       â””â”€â”€ Master Butler rÃ©pond : Permissions requises
    â”‚
    â””â”€â”€ BondingBrother enrichit le contexte de l'intention
```

**RÃ¨gles d'interaction :**

- BondingBrother interroge pour la traduction, pas pour la dÃ©cision
- Les rÃ©ponses aident Ã  construire le contexte
- Aucune interprÃ©tation par Master Butler

### 12.3. Interaction avec les OpÃ©rateurs

**Flux de dÃ©finition :**

```
OpÃ©rateur dÃ©finit une nouvelle permission
    â”‚
    â”œâ”€â”€ Soumet la dÃ©finition via BondingBrother
    â”‚       â”‚
    â”‚       â””â”€â”€ Master Butler valide et enregistre
    â”‚
    â””â”€â”€ Confirmation de l'enregistrement
```

**Flux de dÃ©couverte :**

```
OpÃ©rateur dÃ©couvre les permissions disponibles
    â”‚
    â”œâ”€â”€ Interroge Master Butler : "Quelles permissions existent pour ce domaine ?"
    â”‚       â”‚
    â”‚       â””â”€â”€ Master Butler rÃ©pond : Liste des permissions (selon contexte)
    â”‚
    â””â”€â”€ OpÃ©rateur utilise ces informations
```

### 12.4. Interaction avec Ever Buddy

**Coordination du cycle de vie :**

```
Ever Buddy gÃ¨re l'Ã©volution des permissions
    â”‚
    â”œâ”€â”€ VÃ©rifie la compatibilitÃ© des versions
    â”‚       â”‚
    â”‚       â””â”€â”€ Master Butler fournit les versions
    â”‚
    â”œâ”€â”€ GÃ¨re les dÃ©prÃ©ciations programmÃ©es
    â”‚       â”‚
    â”‚       â””â”€â”€ Master Butler exÃ©cute les transitions
    â”‚
    â””â”€â”€ Orchestre les migrations
            â”‚
            â””â”€â”€ Master Butler applique les changements
```

---

## 13. ConformitÃ© aux Lois d'Autonomie SystÃ¨me

Ce contrat respecte les Lois d'Autonomie SystÃ¨me dÃ©finies dans [Miyukini Framework - Lois Autonomie Systeme.md](..//..//..//..//miyukini-webway-system//reference//_index.md).

### LOI-1 : Aucune dÃ©pendance externe critique Ã  l'exÃ©cution

**ConformitÃ© :** Conforme

Le registre des permissions est entiÃ¨rement local :

- **Stockage local** : Les permissions sont dÃ©finies et stockÃ©es localement
- **Interrogation locale** : Toutes les requÃªtes s'exÃ©cutent localement
- **Aucune API externe** : Aucun service distant n'est requis

**VÃ©rification LOI-1** : *"Le registre des permissions fonctionne-t-il si le rÃ©seau est indisponible ?"* â†’ **Oui.**

### LOI-5 : Le coÃ»t doit Ãªtre proportionnel au hardware

**ConformitÃ© :** Conforme

Le registre des permissions a une empreinte minimale :

- **DonnÃ©es lÃ©gÃ¨res** : Les permissions sont des mÃ©tadonnÃ©es textuelles
- **Pas de workers** : Aucun processus en arriÃ¨re-plan
- **Lookups simples** : OpÃ©rations de consultation directe
- **MÃ©moire prÃ©visible** : Proportionnelle au nombre de permissions

**VÃ©rification LOI-5** : *"Le registre fonctionne-t-il sur un Raspberry Pi 4 ?"* â†’ **Oui.** Un registre typique (quelques centaines de permissions) reprÃ©sente quelques kilo-octets.

### SynthÃ¨se de conformitÃ©

| Loi | Statut | Raison |
|-----|--------|--------|
| LOI-1 | âœ… Conforme | Registre local, aucune dÃ©pendance externe |
| LOI-5 | âœ… Conforme | MÃ©tadonnÃ©es lÃ©gÃ¨res, consommation minimale |

---

## 14. Exemples de permissions

### Exemple 1 : Permission standard

```yaml
permission:
  id: "content.article.create.own"
  name: "Create Own Articles"
  version: "1.0.0"
  description: "Allows creating articles attributed to the current user"
  domain: "content"
  level: STANDARD
  scope_type: OWNED
  capabilities:
    - "content.create"
  status: ACTIVE
  created_at: "2026-01-27T10:00:00Z"
  created_by: "system:bootstrap"
```

### Exemple 2 : Permission avec implications

```yaml
permission:
  id: "content.manage.team"
  name: "Manage Team Content"
  version: "1.0.0"
  description: "Full management of team content including create, edit, delete"
  domain: "content"
  level: ELEVATED
  scope_type: SCOPED
  capabilities: []
  implied_permissions:
    - "content.article.create.team"
    - "content.article.edit.team"
    - "content.article.delete.team"
  status: ACTIVE
  created_at: "2026-01-27T10:00:00Z"
  created_by: "system:bootstrap"
```

### Exemple 3 : Permission critique

```yaml
permission:
  id: "data.export.all"
  name: "Export All Data"
  version: "1.0.0"
  description: "Allows exporting all data from the system - critical operation"
  domain: "data"
  level: CRITICAL
  scope_type: GLOBAL
  capabilities:
    - "data.export"
    - "data.read"
  required_permissions:
    - "data.read.all"
  status: ACTIVE
  created_at: "2026-01-27T10:00:00Z"
  created_by: "admin:setup"
```

### Exemple 4 : Permission dÃ©prÃ©ciÃ©e

```yaml
permission:
  id: "content.publish.direct"
  name: "Direct Publish Content"
  version: "1.2.0"
  description: "Allows publishing content without workflow - DEPRECATED"
  domain: "content"
  level: ELEVATED
  scope_type: GLOBAL
  capabilities:
    - "content.publish"
  status: DEPRECATED
  deprecation_date: "2026-01-15T00:00:00Z"
  deprecation_reason: "Replaced by workflow-based publishing"
  successor_id: "content.publish.workflow"
  created_at: "2025-06-01T10:00:00Z"
  created_by: "system:bootstrap"
```

---

## 15. Conclusion et statut contractuel

### Essence du Permission Registry Contract

Le registre des permissions de Master Butler est la **source de vÃ©ritÃ©** pour tous les droits dÃ©finis dans le systÃ¨me Miyukini. Il dÃ©finit ce qui peut Ãªtre accordÃ©, sans jamais dÃ©cider ce qui est effectivement autorisÃ©.

Ce registre incarne la sÃ©paration entre :
- **La dÃ©finition des droits** (Master Butler)
- **L'attribution des droits** (mÃ©canismes d'attribution)
- **La vÃ©rification des droits** (StrongFather)

### Phrase fondatrice

> **Le registre des permissions dÃ©finit les droits possibles du systÃ¨me Miyukini, en association avec les capacitÃ©s, sans jamais participer Ã  la dÃ©cision d'autorisation.**

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il Ã©tablit des rÃ¨gles absolues qui ne peuvent Ãªtre contournÃ©es, nÃ©gociÃ©es, ou modifiÃ©es. Le contrat prime sur toute considÃ©ration pratique.

Toute implÃ©mentation du registre des permissions doit respecter intÃ©gralement ce document. Toute Ã©volution doit prÃ©server les invariants dÃ©finis ici.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** FONDATION â€” Non nÃ©gociable  
**RÃ©fÃ©rence :** Miyukini Core System v2.4

**RÃ©fÃ©rences croisÃ©es :**

- [Master Butler - Documentation Fondatrice](..//..//foundation//Master%20Butler%20-%20Documentation%20Fondatrice.md) : DÃ©finition et responsabilitÃ©s de Master Butler
- [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) : DÃ©finitions canoniques
- [Miyukini Conceptual References - Tools et Toolkits](..//..//..//..//miyukini-webway-system//reference//_index.md) : Gouvernance des Outils
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) : Lois d'autonomie


