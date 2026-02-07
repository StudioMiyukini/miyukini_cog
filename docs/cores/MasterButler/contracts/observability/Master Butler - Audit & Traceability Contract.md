# Master Butler — Audit & Traceability Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **Master Butler — Audit & Traceability Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit les règles de traçabilité et d'audit pour Master Butler, définissant ce qui doit être tracé, comment les traces sont produites, et comment l'audit du registre est possible dans le système Miyukini Core System v2.4.

Ce contrat précise la nature conceptuelle de la traçabilité des capacités et permissions, les éléments obligatoirement tracés, la structure des traces, et les garanties d'audit du registre.

### Portée

Ce contrat s'applique à **toutes les opérations de traçabilité de Master Butler** et définit de manière absolue :
- la définition formelle de la traçabilité du registre,
- les éléments obligatoirement tracés (déclarations, définitions, interrogations, modifications),
- la structure des traces de registre,
- les règles de production de traces,
- les garanties d'audit du registre,
- les invariants de traçabilité.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :
- **[Master Butler — Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md)** : INV-MB-5 (traçabilité complète des définitions)
- **[Master Butler — Capability Registry Contract](../registry/Master%20Butler%20-%20Capability%20Registry%20Contract.md)** : Traçabilité et historique des capacités (section 7)
- **[Master Butler — Permission Registry Contract](../registry/Master%20Butler%20-%20Permission%20Registry%20Contract.md)** : Traçabilité et historique des permissions
- **[Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Conformité aux lois d'autonomie, notamment **LOI-3** (l'état local est souverain) : les traces d'audit locales constituent une source de vérité complète

Il n'introduit aucune contradiction, et constitue la définition formelle de la traçabilité et de l'audit dans Master Butler.

---

## 2. Nature de la traçabilité

### 2.1. Définition de la traçabilité

La **traçabilité** dans Master Butler est la capacité de suivre et de documenter toutes les opérations effectuées sur le registre des capacités et permissions, permettant une reconstruction complète de l'évolution du registre et une vérification de l'intégrité des définitions.

**Caractéristiques de la traçabilité :**

- **Complète** : Toute opération sur le registre est tracée
- **Non-intrusive** : La traçabilité ne modifie pas le comportement de Master Butler
- **Auditée** : Les traces permettent l'audit a posteriori
- **Immuable** : Les traces ne sont jamais modifiées après production

### 2.2. Objectifs de la traçabilité

La traçabilité permet :

1. **Audit du registre** : Vérifier que les capacités et permissions sont correctement déclarées et gérées
2. **Diagnostic** : Comprendre l'évolution du registre dans le temps
3. **Conformité** : Démontrer la conformité des déclarations aux règles établies
4. **Reproductibilité** : Reconstruire l'état du registre à un instant donné
5. **Transparence** : Rendre l'évolution des capacités et permissions transparente
6. **Gouvernance** : Permettre la gouvernance des Tools et Toolkits

### 2.3. Distinction traçabilité/décision

| Aspect | Traçabilité | Décision |
|--------|-------------|----------|
| Objectif | Audit et diagnostic | Autorisation |
| Produit par | Master Butler | StrongFather |
| Nature | Passive (observation) | Active (jugement) |
| Données | Capacités et permissions | Intentions et politiques |

**Principe fondamental :**

Master Butler trace les définitions de capacités et permissions, mais ne trace jamais les décisions d'autorisation (domaine de StrongFather). La traçabilité de Master Butler concerne exclusivement :
- Ce qui existe (capacités)
- Ce qui est défini (permissions)
- Qui a déclaré quoi et quand

---

## 3. Éléments obligatoirement tracés

### 3.1. Traces de déclaration de capacité

Toute déclaration de capacité DOIT être tracée avec :

**Éléments obligatoires :**

- Identifiant de la capacité (CapabilityId)
- Type d'opération (DECLARATION)
- Source de la déclaration (SourceIdentity)
- Métadonnées complètes de la capacité
- Horodatage de déclaration
- Hash d'intégrité de la déclaration
- Résultat de l'opération (SUCCÈS, ÉCHEC avec raison)

**Règles :**

- **R-TRACE-CAP-1** : Toute déclaration de capacité est tracée immédiatement
- **R-TRACE-CAP-2** : La trace de déclaration est immuable après création
- **R-TRACE-CAP-3** : Les déclarations idempotentes (redéclarations identiques) sont également tracées
- **R-TRACE-CAP-4** : Les échecs de déclaration sont tracés avec la raison

### 3.2. Traces de définition de permission

Toute définition de permission DOIT être tracée avec :

**Éléments obligatoires :**

- Identifiant de la permission (PermissionId)
- Type d'opération (DEFINITION)
- Source de la définition (SourceIdentity)
- Capacités associées (liste des CapabilityId)
- Métadonnées complètes de la permission
- Horodatage de définition
- Hash d'intégrité de la définition
- Résultat de l'opération (SUCCÈS, ÉCHEC avec raison)

**Règles :**

- **R-TRACE-PERM-1** : Toute définition de permission est tracée immédiatement
- **R-TRACE-PERM-2** : La trace de définition est immuable après création
- **R-TRACE-PERM-3** : Les associations capacités-permissions sont explicites dans la trace
- **R-TRACE-PERM-4** : Les échecs de définition sont tracés avec la raison

### 3.3. Traces d'interrogation

Toute interrogation significative du registre DOIT être tracée avec :

**Éléments obligatoires :**

- Type d'interrogation (ById, ByCategory, BySource, Discovery, etc.)
- Critères de recherche (filtres appliqués)
- Source de l'interrogation (appelant)
- Horodatage de l'interrogation
- Nombre de résultats retournés
- Identifiant de corrélation (si interrogation liée à une intention StrongFather)

**Règles :**

- **R-TRACE-QUERY-1** : Les interrogations par StrongFather sont toujours tracées (niveau MANDATORY)
- **R-TRACE-QUERY-2** : Les interrogations de découverte (Discovery) sont toujours tracées
- **R-TRACE-QUERY-3** : Les interrogations simples peuvent être en niveau DETAILED
- **R-TRACE-QUERY-4** : La trace d'interrogation ne contient pas les résultats complets (seulement le compte)

### 3.4. Traces de modification de statut

Toute modification de statut (dépréciation, suppression) DOIT être tracée avec :

**Éléments obligatoires :**

- Identifiant de l'élément modifié (CapabilityId ou PermissionId)
- Type d'opération (DEPRECATION, REMOVAL)
- Statut avant modification
- Statut après modification
- Raison de la modification
- Successeur (si applicable)
- Source de la modification
- Horodatage de modification
- Hash d'intégrité

**Règles :**

- **R-TRACE-MOD-1** : Toute modification de statut est tracée immédiatement
- **R-TRACE-MOD-2** : La raison de modification est obligatoire et significative
- **R-TRACE-MOD-3** : Le successeur (si dépréciation) est explicitement référencé
- **R-TRACE-MOD-4** : La transition de statut est irréversible et tracée comme telle

### 3.5. Traces de relation

Toute modification des relations entre capacités DOIT être tracée avec :

**Éléments obligatoires :**

- Type de relation (Requires, Implies, Conflicts, Supersedes, Groups)
- Capacité source (from)
- Capacité cible (to)
- Type d'opération (ADDITION, REMOVAL)
- Raison de la modification
- Source de la modification
- Horodatage

**Règles :**

- **R-TRACE-REL-1** : Toute modification de relation est tracée immédiatement
- **R-TRACE-REL-2** : Les deux extrémités de la relation sont explicitement identifiées
- **R-TRACE-REL-3** : La validité de la relation est vérifiée avant traçage

### 3.6. Traces d'erreur

Toute erreur rencontrée DOIT être tracée avec :

**Éléments obligatoires :**

- Identifiant de l'élément concerné (si applicable)
- Type d'opération tentée
- Catégorie d'erreur (InvalidId, DuplicateId, MissingMetadata, UnauthorizedSource, InvalidRelation, etc.)
- Description de l'erreur
- Contexte de l'erreur
- Horodatage de l'erreur

**Règles :**

- **R-TRACE-ERR-1** : Toute erreur est tracée immédiatement
- **R-TRACE-ERR-2** : La trace d'erreur ne se substitue pas à la gestion d'erreur
- **R-TRACE-ERR-3** : La trace d'erreur permet le diagnostic a posteriori

---

## 4. Structure des traces

### 4.1. Structure commune

Toute trace DOIT contenir la structure commune suivante :

**Identifiant de trace :**

Un identifiant unique (TraceId) permettant de référencer la trace de manière non ambiguë.

**Type de trace :**

Le type de trace parmi :
- `CAPABILITY_DECLARATION` : Déclaration de capacité
- `CAPABILITY_DEPRECATION` : Dépréciation de capacité
- `CAPABILITY_REMOVAL` : Suppression de capacité
- `PERMISSION_DEFINITION` : Définition de permission
- `PERMISSION_DEPRECATION` : Dépréciation de permission
- `PERMISSION_REMOVAL` : Suppression de permission
- `RELATION_ADDITION` : Ajout de relation
- `RELATION_REMOVAL` : Suppression de relation
- `QUERY_STRONGFATHER` : Interrogation par StrongFather
- `QUERY_DISCOVERY` : Interrogation de découverte
- `QUERY_GENERAL` : Interrogation générale
- `ERROR` : Erreur rencontrée

**Horodatage :**

L'horodatage de production de la trace (timestamp UTC).

**Source :**

L'identité de l'acteur ayant déclenché l'opération tracée.

**Identifiant de corrélation :**

Un identifiant optionnel permettant de corréler les traces liées à une même intention StrongFather.

**Checksum :**

Un hash d'intégrité garantissant que la trace n'a pas été altérée.

### 4.2. Structure de trace de capacité

```
CapabilityTrace {
  trace_id: TraceId,
  trace_type: CAPABILITY_DECLARATION | CAPABILITY_DEPRECATION | CAPABILITY_REMOVAL,
  timestamp: Timestamp,
  source: SourceIdentity,
  correlation_id: CorrelationId?,
  capability_id: CapabilityId,
  operation_result: SUCCESS | FAILURE,
  failure_reason: String?,
  capability_snapshot: {
    name: String,
    description: String,
    category: CapabilityCategory,
    status_before: CapabilityStatus?,
    status_after: CapabilityStatus,
    metadata: CapabilityMetadata
  },
  deprecation_info: {
    reason: String,
    successor: CapabilityId?
  }?,
  checksum: Checksum
}
```

### 4.3. Structure de trace de permission

```
PermissionTrace {
  trace_id: TraceId,
  trace_type: PERMISSION_DEFINITION | PERMISSION_DEPRECATION | PERMISSION_REMOVAL,
  timestamp: Timestamp,
  source: SourceIdentity,
  correlation_id: CorrelationId?,
  permission_id: PermissionId,
  operation_result: SUCCESS | FAILURE,
  failure_reason: String?,
  permission_snapshot: {
    name: String,
    description: String,
    associated_capabilities: List<CapabilityId>,
    status_before: PermissionStatus?,
    status_after: PermissionStatus,
    metadata: PermissionMetadata
  },
  deprecation_info: {
    reason: String,
    successor: PermissionId?
  }?,
  checksum: Checksum
}
```

### 4.4. Structure de trace d'interrogation

```
QueryTrace {
  trace_id: TraceId,
  trace_type: QUERY_STRONGFATHER | QUERY_DISCOVERY | QUERY_GENERAL,
  timestamp: Timestamp,
  source: SourceIdentity,
  correlation_id: CorrelationId?,
  query_type: ById | ByCategory | BySource | ByStatus | ByTags | All | ContextQuery,
  query_filter: {
    capability_id: CapabilityId?,
    permission_id: PermissionId?,
    category: CapabilityCategory?,
    source: SourceIdentity?,
    status: Status?,
    tags: Set<String>?
  },
  result_count: Integer,
  checksum: Checksum
}
```

### 4.5. Structure de trace d'erreur

```
ErrorTrace {
  trace_id: TraceId,
  trace_type: ERROR,
  timestamp: Timestamp,
  source: SourceIdentity?,
  correlation_id: CorrelationId?,
  operation_attempted: OperationType,
  element_id: CapabilityId | PermissionId | null,
  error_category: ErrorCategory,
  error_description: String,
  error_context: Map<String, Any>,
  checksum: Checksum
}
```

### 4.6. Règles de formation

**R-STRUCT-1 : Complétude**

Toute trace DOIT contenir tous les éléments obligatoires de sa structure.

**R-STRUCT-2 : Non-ambiguïté**

Toute trace DOIT être non ambiguë et interprétable sans contexte externe.

**R-STRUCT-3 : Auto-suffisance**

Toute trace DOIT être auto-suffisante pour l'audit de l'opération qu'elle décrit.

**R-STRUCT-4 : Intégrité vérifiable**

Toute trace DOIT inclure un checksum permettant de vérifier son intégrité.

---

## 5. Règles de production de traces

### 5.1. Production systématique

**R-PROD-1 : Trace obligatoire**

Toute déclaration, définition, modification, et erreur DOIT produire une trace.

**R-PROD-2 : Production immédiate**

Les traces sont produites immédiatement après l'opération tracée.

**R-PROD-3 : Pas d'omission**

Aucune trace ne peut être omise pour des raisons de performance ou autre.

### 5.2. Production sans effet de bord

**R-PROD-4 : Pas d'effet de bord**

La production de traces ne doit jamais modifier le comportement de Master Butler.

**R-PROD-5 : Isolation**

La production de traces est isolée des opérations du registre. Une erreur de traçabilité ne doit pas affecter les opérations.

**R-PROD-6 : Aucune influence**

Les traces ne peuvent jamais influencer le contenu du registre.

### 5.3. Immutabilité

**R-PROD-7 : Traces immuables**

Une fois produite, une trace ne peut jamais être modifiée.

**R-PROD-8 : Pas de suppression**

Les traces ne peuvent jamais être supprimées par Master Butler.

**R-PROD-9 : Intégrité**

L'intégrité des traces doit être préservée et vérifiable via le checksum.

### 5.4. Corrélation

**R-PROD-10 : Corrélation StrongFather**

Lorsqu'une interrogation est effectuée dans le contexte d'une évaluation StrongFather, l'identifiant de corrélation DOIT être propagé.

**R-PROD-11 : Chaîne de corrélation**

Les traces corrélées permettent de reconstituer l'ensemble des informations consultées pour une intention donnée.

---

## 6. Garanties d'audit

### 6.1. Garanties de complétude

**G-AUD-1 : Traçabilité complète du registre**

Toute modification du registre (capacités, permissions, relations) peut être auditée avec l'ensemble des informations nécessaires.

**G-AUD-2 : Chaîne complète d'évolution**

L'évolution d'une capacité ou permission de sa création à sa suppression est entièrement traçable.

**G-AUD-3 : Sources identifiées**

Toutes les sources de déclaration et modification sont identifiées dans les traces.

### 6.2. Garanties de reconstruction

**G-AUD-4 : État à un instant donné**

L'état du registre peut être reconstruit à n'importe quel instant passé à partir des traces.

**G-AUD-5 : Opération GetStateAt**

L'opération `GetStateAt(timestamp)` permet de reconstituer l'état du registre à un instant donné en rejouant les traces.

### 6.3. Garanties d'intégrité

**G-AUD-6 : Intégrité des traces**

Les traces ne sont jamais altérées après production. Le checksum permet de le vérifier.

**G-AUD-7 : Corrélation fiable**

Les identifiants de corrélation permettent de reconstituer l'ensemble des consultations effectuées par StrongFather pour une intention.

**G-AUD-8 : Pas de trace fantôme**

Toute trace référence une opération réellement effectuée. Aucune trace ne peut être créée artificiellement.

### 6.4. Garanties de gouvernance

**G-AUD-9 : Audit des Tools**

Toutes les déclarations de capacités de Tools sont tracées et auditables.

**G-AUD-10 : Audit des Toolkits**

Toutes les compositions de Toolkits sont tracées et auditables.

**G-AUD-11 : Souveraineté vérifiable**

L'audit permet de vérifier qu'aucun Tool ou capacité non déclaré n'existe dans le système.

---

## 7. Invariants de traçabilité

### 7.1. Invariants de production

**INV-TRACE-1 : Production obligatoire**

Toute opération sur le registre produit des traces. Aucune opération "silencieuse" n'existe.

**INV-TRACE-2 : Production sans effet**

La production de traces ne modifie jamais le comportement de Master Butler.

**INV-TRACE-3 : Production immédiate**

Les traces sont produites au moment de l'opération, pas après.

### 7.2. Invariants d'intégrité

**INV-TRACE-4 : Immutabilité**

Les traces sont immuables après production.

**INV-TRACE-5 : Complétude structurelle**

Toute trace contient tous les éléments obligatoires de sa structure.

**INV-TRACE-6 : Corrélation valide**

Les identifiants de corrélation référencent des contextes d'évaluation valides.

**INV-TRACE-7 : Checksum valide**

Le checksum de chaque trace permet de vérifier son intégrité.

### 7.3. Invariants d'audit

**INV-TRACE-8 : Auditabilité du registre**

Toute modification du registre est auditable à partir des traces.

**INV-TRACE-9 : Reconstruction possible**

L'état du registre à tout instant passé peut être reconstruit à partir des traces.

**INV-TRACE-10 : Exhaustivité vérifiable**

L'exhaustivité du registre (INV-MB-1) peut être vérifiée par audit des traces.

---

## 8. Niveaux de trace

### 8.1. Niveau obligatoire (MANDATORY)

Le niveau obligatoire comprend les traces qui DOIVENT toujours être produites :

- Traces de déclaration de capacité (section 3.1)
- Traces de définition de permission (section 3.2)
- Traces de modification de statut (section 3.4)
- Traces de modification de relation (section 3.5)
- Traces d'erreur (section 3.6)
- Traces d'interrogation par StrongFather (R-TRACE-QUERY-1)
- Traces d'interrogation Discovery (R-TRACE-QUERY-2)

**Règle :** Ces traces ne peuvent jamais être désactivées.

### 8.2. Niveau détaillé (DETAILED)

Le niveau détaillé comprend les traces additionnelles pour un diagnostic approfondi :

- Traces d'interrogation générale (section 3.3)
- Détails de validation des déclarations
- Contexte étendu des opérations

**Règle :** Ces traces peuvent être activées/désactivées selon les besoins de diagnostic.

### 8.3. Niveau debug (DEBUG)

Le niveau debug comprend les traces pour le développement et le débogage :

- État interne des index
- Étapes de validation intermédiaires
- Métriques de performance du registre

**Règle :** Ces traces sont réservées au développement et ne doivent pas être actives en production.

---

## 9. Opérations d'audit

### 9.1. GetHistory

**Description :**

Récupère l'historique complet d'une capacité ou permission.

**Signature conceptuelle :**

```
GetHistory(
  element_id: CapabilityId | PermissionId
) → Result<List<Trace>, AuditError>
```

**Résultat :**

Liste chronologique de toutes les traces concernant l'élément.

### 9.2. GetStateAt

**Description :**

Reconstruit l'état du registre à un instant donné.

**Signature conceptuelle :**

```
GetStateAt(
  timestamp: Timestamp
) → Result<RegistrySnapshot, AuditError>
```

**Résultat :**

Snapshot du registre tel qu'il était à l'instant spécifié.

### 9.3. VerifyIntegrity

**Description :**

Vérifie l'intégrité du registre et des traces.

**Signature conceptuelle :**

```
VerifyIntegrity() → Result<IntegrityReport, AuditError>
```

**Vérifications effectuées :**

- Tous les checksums des traces sont valides
- Les transitions de statut respectent les règles
- Les relations référencent des capacités existantes
- L'historique est cohérent avec l'état actuel

### 9.4. GetStatistics

**Description :**

Produit des statistiques sur le registre.

**Signature conceptuelle :**

```
GetStatistics() → Result<RegistryStatistics, AuditError>
```

**Statistiques disponibles :**

- Nombre total de capacités (par statut)
- Nombre total de permissions (par statut)
- Nombre de déclarations par source
- Nombre de modifications par période
- Capacités les plus interrogées
- Permissions les plus référencées

### 9.5. GetCorrelatedTraces

**Description :**

Récupère toutes les traces liées à une intention StrongFather.

**Signature conceptuelle :**

```
GetCorrelatedTraces(
  correlation_id: CorrelationId
) → Result<List<Trace>, AuditError>
```

**Résultat :**

Liste de toutes les traces d'interrogation effectuées dans le contexte de l'intention.

---

## 10. Schémas ASCII

### 10.1. Flux de production de traces

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     FLUX DE PRODUCTION DE TRACES                             │
└─────────────────────────────────────────────────────────────────────────────┘

   MODULE SPM              MASTER BUTLER                TRACE STORE
       │                        │                            │
       │  DeclareCapability()   │                            │
       ├───────────────────────►│                            │
       │                        │                            │
       │                        │  1. Valider déclaration    │
       │                        │                            │
       │                        │  2. Enregistrer capacité   │
       │                        │                            │
       │                        │  3. Produire trace         │
       │                        ├───────────────────────────►│
       │                        │  CapabilityTrace {         │
       │                        │    type: DECLARATION,      │
       │                        │    capability_id: ...,     │
       │                        │    result: SUCCESS,        │
       │                        │    checksum: ...           │
       │                        │  }                         │
       │                        │                            │
       │                        │◄───────────────────────────┤
       │                        │       [Trace stockée]      │
       │◄───────────────────────┤                            │
       │    Result::Ok(...)     │                            │
       │                        │                            │
       ▼                        ▼                            ▼
```

### 10.2. Corrélation avec StrongFather

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    CORRÉLATION STRONGFATHER - MASTER BUTLER                  │
└─────────────────────────────────────────────────────────────────────────────┘

   STRONGFATHER                MASTER BUTLER                TRACE STORE
       │                            │                            │
       │  QueryCapability(          │                            │
       │    id: "content.create",   │                            │
       │    correlation: "INT-123"  │                            │
       │  )                         │                            │
       ├───────────────────────────►│                            │
       │                            │                            │
       │                            │  1. Exécuter requête       │
       │                            │                            │
       │                            │  2. Produire trace         │
       │                            ├───────────────────────────►│
       │                            │  QueryTrace {              │
       │                            │    type: QUERY_STRONGFATHER│
       │                            │    correlation: "INT-123", │
       │                            │    result_count: 1         │
       │                            │  }                         │
       │                            │                            │
       │◄───────────────────────────┤                            │
       │  Capability { ... }        │                            │
       │                            │                            │
       │                            │                            │
       │  [AUDIT ULTÉRIEUR]         │                            │
       │                            │                            │
       │  GetCorrelatedTraces(      │                            │
       │    "INT-123"               │                            │
       │  )                         │                            │
       │                            ├───────────────────────────►│
       │                            │                            │
       │                            │◄───────────────────────────┤
       │◄───────────────────────────┤  [Toutes les traces       │
       │  Liste des traces          │   corrélées]               │
       │                            │                            │
       ▼                            ▼                            ▼

       ┌─────────────────────────────────────────────────────────┐
       │  L'audit permet de voir exactement quelles capacités   │
       │  et permissions ont été consultées pour l'intention    │
       │  INT-123, reconstituant le contexte d'évaluation.      │
       └─────────────────────────────────────────────────────────┘
```

### 10.3. Reconstruction d'état

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     RECONSTRUCTION D'ÉTAT (GetStateAt)                       │
└─────────────────────────────────────────────────────────────────────────────┘

   TRACE STORE                                        ÉTAT RECONSTRUIT
       │                                                    │
       │  T0: CapabilityTrace(DECLARATION, "content.create")│
       │  ─────────────────────────────────────────────────►│ cap: content.create
       │                                                    │     status: Active
       │  T1: CapabilityTrace(DECLARATION, "content.edit")  │
       │  ─────────────────────────────────────────────────►│ cap: content.edit
       │                                                    │     status: Active
       │  T2: PermissionTrace(DEFINITION, "content.manage") │
       │  ─────────────────────────────────────────────────►│ perm: content.manage
       │                                                    │     caps: [create, edit]
       │  T3: CapabilityTrace(DEPRECATION, "content.create")│
       │  ─────────────────────────────────────────────────►│ cap: content.create
       │                                                    │     status: Deprecated
       │                                                    │
       │                                                    │
   ┌───┴───────────────────────────────────────────────────┴───┐
   │                                                            │
   │  GetStateAt(T1.5) → État entre T1 et T2 :                 │
   │                                                            │
   │    Capacités:                                              │
   │      - content.create (Active)                            │
   │      - content.edit (Active)                              │
   │                                                            │
   │    Permissions:                                            │
   │      - (aucune)                                           │
   │                                                            │
   │  GetStateAt(T3.5) → État après T3 :                       │
   │                                                            │
   │    Capacités:                                              │
   │      - content.create (Deprecated)                        │
   │      - content.edit (Active)                              │
   │                                                            │
   │    Permissions:                                            │
   │      - content.manage → [create, edit]                    │
   │                                                            │
   └────────────────────────────────────────────────────────────┘
```

### 10.4. Structure hiérarchique des traces

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     HIÉRARCHIE DES TYPES DE TRACES                           │
└─────────────────────────────────────────────────────────────────────────────┘

                            ┌─────────────┐
                            │   TRACE     │
                            │  (base)     │
                            └──────┬──────┘
                                   │
        ┌──────────────────────────┼──────────────────────────┐
        │                          │                          │
        ▼                          ▼                          ▼
┌───────────────┐        ┌───────────────┐        ┌───────────────┐
│ CAPABILITY    │        │ PERMISSION    │        │ OPERATION     │
│ TRACE         │        │ TRACE         │        │ TRACE         │
└───────┬───────┘        └───────┬───────┘        └───────┬───────┘
        │                        │                        │
   ┌────┼────┐              ┌────┼────┐              ┌────┼────┐
   │    │    │              │    │    │              │    │    │
   ▼    ▼    ▼              ▼    ▼    ▼              ▼    ▼    ▼
┌────┐┌────┐┌────┐      ┌────┐┌────┐┌────┐      ┌────┐┌────┐┌────┐
│DECL││DEP ││REM │      │DEF ││DEP ││REM │      │QRY ││REL ││ERR │
└────┘└────┘└────┘      └────┘└────┘└────┘      └────┘└────┘└────┘

Légende:
  DECL = Declaration      DEP = Deprecation      REM = Removal
  DEF = Definition        QRY = Query            REL = Relation
  ERR = Error
```

---

## 11. Règles de fermeture du contrat

### 11.1. Contrat fermé

Ce contrat est **fermé**. Seuls les types de traces, les structures, et les règles explicitement définis dans ce contrat sont valides.

### 11.2. Interdiction d'extension implicite

Aucune extension implicite n'est autorisée :

- **INTERD-TRACE-1** : Aucun type de trace non défini n'est reconnu
- **INTERD-TRACE-2** : Aucune règle de production non définie n'est applicable
- **INTERD-TRACE-3** : Aucun invariant non défini n'est garanti

---

## 12. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable la traçabilité et l'audit de Master Butler.

Il garantit que :
- toutes les opérations sur le registre sont tracées,
- les structures de traces sont standardisées,
- les règles de production sont explicites,
- les garanties d'audit sont respectées,
- les invariants de traçabilité sont maintenus,
- l'état du registre peut être reconstruit à tout instant,
- la corrélation avec StrongFather permet l'audit des évaluations,
- le contrat est fermé et non extensible implicitement.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

**Document créé le :** 2026-01-27  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, Master Butler Documentation Fondatrice, Capability Registry Contract  
**Type :** Contrat de traçabilité et audit non négociable

---

## 13. Mini log — erreurs / warnings / ambiguïtés rencontrées et corrigées

### Ambiguïté A1 : Traçabilité Master Butler vs StrongFather

**Ambiguïté rencontrée :** Risque de confusion entre la traçabilité de Master Butler (registre) et celle de StrongFather (décisions).

**Décision prise :** Clarification explicite que Master Butler trace les définitions (ce qui existe), tandis que StrongFather trace les décisions (ce qui est autorisé).

**Correction effectuée :** Section 2.3 définit la distinction traçabilité/décision avec tableau comparatif.

### Ambiguïté A2 : Niveau de traçabilité des interrogations

**Ambiguïté rencontrée :** Faut-il tracer toutes les interrogations du registre ou seulement certaines ?

**Décision prise :** Définition de 3 catégories avec niveaux différents :
- MANDATORY : Interrogations StrongFather et Discovery
- DETAILED : Interrogations générales
- Les traces ne contiennent pas les résultats complets (seulement le compte)

**Correction effectuée :** Section 3.3 avec règles R-TRACE-QUERY-1 à R-TRACE-QUERY-4 et section 8.

### Ambiguïté A3 : Corrélation avec les intentions StrongFather

**Ambiguïté rencontrée :** Comment lier les traces Master Butler aux évaluations StrongFather ?

**Décision prise :** Introduction d'un identifiant de corrélation (CorrelationId) propagé lors des interrogations dans le contexte d'une intention.

**Correction effectuée :** Sections 4.1 (structure commune), 5.4 (règles de corrélation), et 9.5 (opération GetCorrelatedTraces).

### Ambiguïté A4 : Reconstruction d'état historique

**Ambiguïté rencontrée :** La Documentation Fondatrice mentionne l'historique mais pas la reconstruction d'état.

**Décision prise :** Garantie formelle que l'état du registre peut être reconstruit à n'importe quel instant passé (GetStateAt).

**Correction effectuée :** Section 6.2 avec garanties G-AUD-4 et G-AUD-5, et section 9.2.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec Documentation Fondatrice (INV-MB-5) : Confirmée
- ✅ Cohérence avec Capability Registry Contract (section 7) : Confirmée
- ✅ Cohérence avec LOI-3 (état local souverain) : Confirmée
- ✅ Pas de confusion avec traçabilité StrongFather : Confirmée
- ✅ Pas de décision dans les traces : Confirmée

**Conclusion :** Aucune contradiction détectée avec les contrats existants.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
