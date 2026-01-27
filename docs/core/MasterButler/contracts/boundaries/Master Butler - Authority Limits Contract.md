# Master Butler — Authority Limits Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **Master Butler Authority Limits Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit les limites absolues de l'autorité de Master Butler dans le système Miyukini Core System v2.4.

Master Butler est le **Capability & Permission Core** (Strate 4). Il recense les capacités, définit les permissions, et fournit ces informations à tous les composants autorisés. Ce contrat définit précisément ce que Master Butler peut faire, ce qu'il ne peut jamais faire, et les frontières de son autorité.

### Portée

Ce contrat s'applique à **Master Butler** et définit de manière absolue :
- Les limites formelles de l'autorité de Master Butler
- Les frontières entre Master Butler et les autres Cores
- Les actions que Master Butler ne peut jamais entreprendre
- Les responsabilités exclusives de Master Butler
- Les responsabilités qui n'appartiennent jamais à Master Butler
- Les invariants d'autorité non négociables
- Les schémas de frontières d'autorité

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues que Master Butler applique sans exception. Ces règles ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète les documents contractuels existants :

- **Master Butler - Documentation Fondatrice** : Définit la nature, le rôle, et les responsabilités de Master Butler
- **Master Butler - Capability Registry Contract** : Définit le modèle du registre des capacités
- **Master Butler - Permission Registry Contract** : Définit le modèle du registre des permissions
- **Master Butler - Boundary & Scope Contract** : Définit le périmètre et les frontières fonctionnelles
- **[Miyukini Conceptual References — Lois Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Ce contrat respecte **LOI-1** (aucune dépendance externe critique) en garantissant que les limites d'autorité fonctionnent localement.

---

## 2. Définition formelle des limites d'autorité

### Définition formelle

Une **limite d'autorité** est une frontière absolue, non négociable, et permanente qui définit ce que Master Butler peut et ne peut pas faire dans le système Miyukini. Les limites d'autorité sont intrinsèques à la nature de Master Butler et ne dépendent d'aucune configuration ou contexte.

### Caractéristiques formelles

**Absolue :** Une limite d'autorité est absolue et s'applique sans exception. Aucun contexte, aucune urgence, aucune considération pratique ne peut justifier le franchissement d'une limite d'autorité.

**Non négociable :** Une limite d'autorité ne peut être négociée, relâchée, ou contournée. Le contrat prime sur toute demande externe ou interne.

**Permanente :** Une limite d'autorité est permanente et s'applique pour toute la durée de vie de Master Butler dans l'environnement.

**Intrinsèque :** Une limite d'autorité est intrinsèque à la nature de Master Butler. Elle découle de sa définition fondatrice et de son positionnement dans l'architecture.

**Vérifiable :** Une limite d'autorité est vérifiable. Toute implémentation peut être auditée pour confirmer le respect des limites.

### Positionnement architectural formel

Master Butler se situe dans la **Strate 4 (Cores Système)** de la pyramide Miyukini. Ses limites d'autorité sont définies par :

- **Position horizontale** : Relations avec les autres Cores de Strate 4 (StrongFather, KindMother, etc.)
- **Position verticale** : Relations avec les strates supérieures (Outils, Opérateurs) et inférieures (Kernel, Hardware)
- **Nature fondatrice** : Rôle de registre passif, sans pouvoir d'exécution ou de décision

---

## 3. Autorité exclusive de Master Butler

Master Butler possède une autorité exclusive dans les domaines suivants. Cette autorité est non partagée et non délégable.

### AE-1 : Registre central des capacités

**Autorité exclusive :** Master Butler est l'unique autorité pour le registre des capacités du système.

**Application :**
- Toute capacité doit être déclarée à Master Butler
- Aucun autre composant ne maintient de registre des capacités
- Le registre de Master Butler est la source de vérité unique pour les capacités

**Limite associée :** Master Butler recense les capacités mais ne les implémente jamais, ne les exécute jamais.

### AE-2 : Registre central des permissions

**Autorité exclusive :** Master Butler est l'unique autorité pour le registre des permissions du système.

**Application :**
- Toute permission doit être définie dans Master Butler
- Aucun autre composant ne définit de permissions
- Le registre de Master Butler est la source de vérité unique pour les permissions

**Limite associée :** Master Butler définit les permissions mais ne vérifie jamais en temps réel si elles sont accordées, ne prend jamais de décision d'autorisation.

### AE-3 : Catalogue des Outils et Kits d'Outils

**Autorité exclusive :** Master Butler est l'unique catalogue des Outils et Kits d'Outils du système.

**Application :**
- Tout Outil doit être déclaré dans Master Butler
- Tout Kit d'Outils doit être défini dans Master Butler
- Les associations Capacité → Outil sont maintenues exclusivement par Master Butler

**Limite associée :** Master Butler catalogue les Outils mais ne les implémente jamais, ne les exécute jamais, ne gère jamais leur cycle de vie technique.

### AE-4 : API de découverte des capacités

**Autorité exclusive :** Master Butler est l'unique fournisseur de l'API de découverte des capacités et permissions.

**Application :**
- Toute découverte de capacités passe par Master Butler
- Toute découverte de permissions passe par Master Butler
- Aucun autre composant ne fournit d'API de découverte pour ces domaines

**Limite associée :** Master Butler expose la découverte mais ne filtre jamais selon des critères métier, ne recommande jamais une capacité plutôt qu'une autre.

### AE-5 : Traçabilité des définitions

**Autorité exclusive :** Master Butler est l'unique responsable de la traçabilité des définitions de capacités et permissions.

**Application :**
- Toute création de capacité est tracée par Master Butler
- Toute définition de permission est tracée par Master Butler
- L'historique des définitions est maintenu exclusivement par Master Butler

**Limite associée :** Master Butler trace les définitions mais ne trace jamais les décisions d'autorisation (StrongFather), ne trace jamais les exécutions (Outils/Opérateurs).

---

## 4. Limites absolues : Ce que Master Butler ne fait JAMAIS

Master Butler ne commet **JAMAIS** les actions suivantes. Ces limites sont absolues, non négociables, et primordiales sur toute considération pratique.

### L-1 : Ne décide JAMAIS

**Limite absolue :** Master Butler **ne prend jamais de décision** sur l'autorisation ou le refus d'une action.

**Application :**
- Aucune méthode de Master Butler ne retourne un booléen d'autorisation
- Master Butler ne répond jamais "autorisé" ou "refusé"
- Master Butler fournit des informations, jamais des verdicts

**Justification :** La décision appartient exclusivement à StrongFather. Master Butler expose les possibilités, StrongFather décide.

**Violation hypothétique :**
```
❌ MasterButler.isAuthorized(user, action) → boolean
❌ MasterButler.canExecute(context, capability) → boolean
❌ MasterButler.hasPermission(user, permission) → boolean
```

**Comportement correct :**
```
✅ MasterButler.getCapabilities(module) → List<Capability>
✅ MasterButler.getPermissionsForCapability(capability) → List<Permission>
✅ MasterButler.getCapabilityContext(context) → CapabilityContext
```

### L-2 : Ne vérifie JAMAIS les permissions en temps réel

**Limite absolue :** Master Butler **ne vérifie jamais** si un utilisateur ou un contexte possède effectivement une permission au moment d'une action.

**Application :**
- Master Butler fournit les définitions de permissions
- Master Butler ne valide jamais "ce contexte a-t-il cette permission maintenant ?"
- La vérification en temps réel appartient à StrongFather

**Justification :** La vérification des permissions en temps réel implique une décision. Toute décision appartient à StrongFather.

**Violation hypothétique :**
```
❌ MasterButler.validatePermission(context, permission) → boolean
❌ MasterButler.checkAccess(user, resource) → AccessResult
```

### L-3 : N'exécute JAMAIS

**Limite absolue :** Master Butler **n'exécute jamais** d'action fonctionnelle, technique, ou métier.

**Application :**
- Master Butler ne crée jamais de contenu
- Master Butler ne modifie jamais de données
- Master Butler n'appelle jamais un Outil
- Master Butler ne déclenche jamais une opération

**Justification :** L'exécution appartient aux Outils et aux Opérateurs. Master Butler est un registre passif.

### L-4 : Ne stocke JAMAIS de données métier

**Limite absolue :** Master Butler **ne stocke jamais** de données métier ou applicatives.

**Application :**
- Master Butler stocke uniquement des métadonnées (capacités, permissions, associations)
- Aucune donnée utilisateur n'est stockée dans Master Butler
- Aucune donnée de domaine n'est stockée dans Master Butler

**Justification :** Les données métier appartiennent aux modules et à KindMother. Master Butler ne gère que des métadonnées structurelles.

### L-5 : Ne gère JAMAIS les identités

**Limite absolue :** Master Butler **ne gère jamais** les identités des utilisateurs ou des systèmes.

**Application :**
- Master Butler connaît les rôles et permissions associées
- Master Butler ne crée jamais d'identité
- Master Butler ne valide jamais une identité
- Master Butler ne stocke jamais de credentials

**Justification :** L'identité appartient au système d'authentification externe et à WorrySentinel pour la gouvernance de sécurité.

### L-6 : Ne définit JAMAIS de politiques

**Limite absolue :** Master Butler **ne définit jamais** de politiques de décision ou de règles métier.

**Application :**
- Master Butler ne définit jamais "quand une permission est accordée"
- Master Butler ne définit jamais "sous quelles conditions une action est autorisée"
- Les politiques appartiennent exclusivement à StrongFather

**Justification :** Les politiques sont des règles de décision. Toute décision appartient à StrongFather.

### L-7 : N'applique JAMAIS de contraintes métier

**Limite absolue :** Master Butler **n'applique jamais** de contraintes métier, de règles de domaine, ou de limites fonctionnelles.

**Application :**
- Master Butler ne limite jamais "un utilisateur ne peut créer que 10 contenus"
- Master Butler ne valide jamais des règles de domaine
- Master Butler ne connaît pas les contraintes applicatives

**Justification :** Les contraintes métier appartiennent aux modules métier et à StrongFather. Master Butler ignore le domaine.

### L-8 : Ne persiste JAMAIS directement

**Limite absolue :** Master Butler **ne gère jamais** directement la persistance de son registre.

**Application :**
- Master Butler ne manipule jamais directement une base de données
- Master Butler ne manipule jamais directement un système de fichiers
- Si le registre doit être persisté, Master Butler utilise KindMother comme support

**Justification :** La persistance appartient à KindMother. Master Butler est agnostique de la couche de stockage.

### L-9 : N'implémente JAMAIS d'Outils

**Limite absolue :** Master Butler **n'implémente jamais** la logique d'un Outil ou d'un Kit d'Outils.

**Application :**
- Master Butler catalogue les Outils mais ne contient pas leur code
- Master Butler définit les associations mais n'exécute pas les Outils
- L'implémentation des Outils appartient à la Strate 6

**Justification :** Master Butler est un catalogue, pas un exécutant. La séparation catalogue/implémentation est fondamentale.

### L-10 : Ne gère JAMAIS le cycle de vie technique

**Limite absolue :** Master Butler **ne gère jamais** le cycle de vie technique des Outils (versions, dépréciation, migration technique).

**Application :**
- Master Butler connaît l'existence des Outils
- La gestion des versions appartient à Ever Buddy
- La migration technique appartient à Ever Buddy et aux Opérateurs

**Justification :** Le cycle de vie technique appartient à Ever Buddy. Master Butler maintient un catalogue statique à un instant T.

---

## 5. Frontières avec les autres Cores

### Frontière Master Butler ↔ StrongFather

| Aspect | Master Butler | StrongFather |
|--------|--------------|--------------|
| **Question** | "Quelles capacités existent ?" | "Cette action est-elle autorisée ?" |
| **Responsabilité** | Recenser les possibilités | Décider de leur usage |
| **Output** | Informations (capacités, permissions) | Décisions (autorisé, refusé) |
| **Autorité** | Registre (définition) | Décision (évaluation) |

**Règle de frontière :** Master Butler fournit les informations, StrongFather les utilise pour décider. Aucun chevauchement n'est permis.

**Flux typique :**
```
1. StrongFather reçoit une intention
2. StrongFather interroge Master Butler : "Cette capacité existe-t-elle ?"
3. Master Butler répond avec les informations
4. StrongFather évalue selon les politiques
5. StrongFather produit une décision
```

**Interdiction formelle :** Master Butler ne participe jamais à l'étape 4 ou 5. Master Butler ne suggère jamais de décision.

### Frontière Master Butler ↔ KindMother

| Aspect | Master Butler | KindMother |
|--------|--------------|------------|
| **Domaine** | Métadonnées (capacités, permissions) | Données métier |
| **Responsabilité** | Cataloguer les possibilités | Persister les données |
| **Stockage** | Registre de métadonnées | Données applicatives |

**Règle de frontière :** Master Butler peut utiliser KindMother pour persister son registre, mais ne gère jamais directement la persistance.

**Interdiction formelle :** Master Butler ne stocke jamais de données métier, ne gère jamais la persistance des données applicatives.

### Frontière Master Butler ↔ BondingBrother

| Aspect | Master Butler | BondingBrother |
|--------|--------------|----------------|
| **Rôle** | Fournir les informations | Traduire les intentions |
| **Interaction** | Répond aux interrogations | Interroge pour la traduction |

**Règle de frontière :** BondingBrother interroge Master Butler pour comprendre les capacités disponibles. Master Butler répond sans interpréter l'intention.

**Flux typique :**
```
1. BondingBrother traduit une intention
2. BondingBrother demande : "Quelles capacités sont disponibles pour ce module ?"
3. Master Butler répond avec la liste des capacités
4. BondingBrother utilise ces informations pour sa traduction
```

### Frontière Master Butler ↔ Ever Buddy

| Aspect | Master Butler | Ever Buddy |
|--------|--------------|------------|
| **Domaine** | Catalogue actuel | Évolution temporelle |
| **Responsabilité** | Ce qui existe maintenant | Ce qui a été, ce qui sera |
| **Gestion** | Registre statique | Cycle de vie dynamique |

**Règle de frontière :** Master Butler maintient le catalogue actuel. Ever Buddy gère les versions, dépréciations, et migrations.

**Interdiction formelle :** Master Butler ne gère jamais le versioning ou la dépréciation des Outils. Cette responsabilité appartient exclusivement à Ever Buddy.

### Frontière Master Butler ↔ WorrySentinel

| Aspect | Master Butler | WorrySentinel |
|--------|--------------|---------------|
| **Domaine** | Capacités et permissions | Sécurité et confiance |
| **Responsabilité** | Définir les permissions | Gouverner les niveaux de sécurité |

**Règle de frontière :** Master Butler définit les permissions disponibles. WorrySentinel gouverne les niveaux de sécurité et peut bloquer certaines capacités selon l'état de confiance.

**Interdiction formelle :** Master Butler ne bloque jamais une capacité pour des raisons de sécurité. Cette responsabilité appartient à WorrySentinel et StrongFather.

### Frontière Master Butler ↔ Caring Nanny

| Aspect | Master Butler | Caring Nanny |
|--------|--------------|--------------|
| **Domaine** | Capacités disponibles | État du système |
| **Responsabilité** | Cataloguer | Observer |

**Règle de frontière :** Master Butler catalogue les capacités disponibles. Caring Nanny observe si l'état du système permet leur usage.

**Interdiction formelle :** Master Butler ne bloque jamais une capacité selon l'état du système. Cette responsabilité appartient à Caring Nanny.

---

## 6. Invariants d'autorité non négociables

### INV-AL-1 : Séparation registre/décision

**Invariant :** La séparation entre le registre (Master Butler) et la décision (StrongFather) est **absolue et non négociable**.

**Implication :** Aucune méthode de Master Butler ne peut retourner un verdict d'autorisation. Toute décision appartient à StrongFather.

### INV-AL-2 : Registre passif

**Invariant :** Master Butler est un **registre passif** qui répond aux interrogations mais ne prend jamais l'initiative.

**Implication :** Master Butler ne déclenche jamais d'action, ne recommande jamais de capacité, ne suggère jamais de décision.

### INV-AL-3 : Agnosticisme métier

**Invariant :** Master Butler est **agnostique du métier** et ne connaît aucune règle de domaine.

**Implication :** Master Butler ne valide jamais selon des critères métier, ne connaît pas les contraintes applicatives.

### INV-AL-4 : Non-exécution

**Invariant :** Master Butler **n'exécute jamais** d'action fonctionnelle ou technique.

**Implication :** Master Butler catalogue mais n'implémente pas, recense mais n'exécute pas.

### INV-AL-5 : Unicité du registre

**Invariant :** Le registre de Master Butler est **l'unique source de vérité** pour les capacités et permissions.

**Implication :** Aucun autre composant ne maintient de registre concurrent. Toute information sur les capacités provient de Master Butler.

### INV-AL-6 : Non-délégation de l'autorité exclusive

**Invariant :** L'autorité exclusive de Master Butler sur le registre **ne peut jamais être déléguée**.

**Implication :** Aucun composant ne peut devenir le registre des capacités à la place de Master Butler, même temporairement.

---

## 7. Schéma ASCII des frontières d'autorité

### 7.1. Vue d'ensemble des limites d'autorité

```
┌─────────────────────────────────────────────────────────────────────────┐
│                      ZONE D'AUTORITÉ MASTER BUTLER                       │
│                                                                          │
│  ┌────────────────────────────────────────────────────────────────────┐ │
│  │  AUTORITÉ EXCLUSIVE                                                 │ │
│  │                                                                     │ │
│  │  ✅ Registre des capacités                                         │ │
│  │  ✅ Registre des permissions                                       │ │
│  │  ✅ Catalogue des Outils et Kits d'Outils                          │ │
│  │  ✅ API de découverte                                              │ │
│  │  ✅ Traçabilité des définitions                                    │ │
│  │  ✅ Associations Capacité → Outil                                  │ │
│  │  ✅ Associations Permission → Capacité                             │ │
│  │                                                                     │ │
│  │  👉 Master Butler EXPOSE ce qui existe                              │ │
│  └────────────────────────────────────────────────────────────────────┘ │
│                                                                          │
│  ┌────────────────────────────────────────────────────────────────────┐ │
│  │  LIMITES ABSOLUES — CE QUE MASTER BUTLER NE FAIT JAMAIS            │ │
│  │                                                                     │ │
│  │  ❌ Ne décide jamais (autorisation/refus)                          │ │
│  │  ❌ Ne vérifie jamais les permissions en temps réel                │ │
│  │  ❌ N'exécute jamais d'action fonctionnelle                        │ │
│  │  ❌ Ne stocke jamais de données métier                              │ │
│  │  ❌ Ne gère jamais les identités                                   │ │
│  │  ❌ Ne définit jamais de politiques                                │ │
│  │  ❌ N'applique jamais de contraintes métier                        │ │
│  │  ❌ Ne persiste jamais directement                                 │ │
│  │  ❌ N'implémente jamais d'Outils                                   │ │
│  │  ❌ Ne gère jamais le cycle de vie technique                       │ │
│  │                                                                     │ │
│  │  👉 Master Butler N'EXÉCUTE et NE DÉCIDE jamais                    │ │
│  └────────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────────┘
```

### 7.2. Frontières avec les autres Cores

```
                            STRATE 4 — CORES SYSTÈME
┌─────────────────────────────────────────────────────────────────────────┐
│                                                                          │
│  ┌──────────────────┐      ┌──────────────────┐      ┌──────────────┐  │
│  │   StrongFather   │      │   Master Butler  │      │  KindMother  │  │
│  │                  │      │                  │      │              │  │
│  │  👑 DÉCIDE       │ ←──→ │  📋 CATALOGUE    │ ←──→ │  💾 PERSISTE │  │
│  │                  │      │                  │      │              │  │
│  │  • Politiques    │      │  • Capacités     │      │  • Données   │  │
│  │  • Autorisations │      │  • Permissions   │      │  • États     │  │
│  │  • Verdicts      │      │  • Outils        │      │  • Entités   │  │
│  └──────────────────┘      └──────────────────┘      └──────────────┘  │
│          │                          │                        │         │
│          │                          │                        │         │
│          │         ┌────────────────┴───────────────┐        │         │
│          │         │                                │        │         │
│          │         ▼                                ▼        │         │
│          │  ┌──────────────┐              ┌──────────────┐   │         │
│          │  │  Ever Buddy  │              │ WorrySentinel│   │         │
│          │  │              │              │              │   │         │
│          │  │  🔄 ÉVOLUE   │              │  🛡️ SÉCURISE │   │         │
│          │  │              │              │              │   │         │
│          │  │  • Versions  │              │  • Niveaux   │   │         │
│          │  │  • Migration │              │  • Confiance │   │         │
│          │  └──────────────┘              └──────────────┘   │         │
│          │                                                   │         │
│          └───────────────────────┬───────────────────────────┘         │
│                                  │                                      │
│                                  ▼                                      │
│                     ┌──────────────────────┐                           │
│                     │    Caring Nanny      │                           │
│                     │                      │                           │
│                     │    👁️ OBSERVE        │                           │
│                     │                      │                           │
│                     │    • État système    │                           │
│                     └──────────────────────┘                           │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘

LÉGENDE DES FRONTIÈRES :
═══════════════════════

┌────────────────────────────────────────────────────────────────────────┐
│                                                                         │
│  Master Butler         →    StrongFather                               │
│  "Voici les capacités"      "J'autorise ou je refuse"                  │
│                                                                         │
│  Master Butler         →    KindMother                                 │
│  "Persiste mon registre"    "Je gère le stockage"                      │
│                                                                         │
│  Master Butler         →    Ever Buddy                                 │
│  "Voici le catalogue"       "Je gère les versions"                     │
│                                                                         │
│  Master Butler         →    WorrySentinel                              │
│  "Voici les permissions"    "Je gouverne la sécurité"                  │
│                                                                         │
│  Master Butler         →    Caring Nanny                               │
│  "Voici ce qui existe"      "J'observe si c'est utilisable"            │
│                                                                         │
└────────────────────────────────────────────────────────────────────────┘
```

### 7.3. Flux d'information et limites

```
INTERROGATION DE MASTER BUTLER
══════════════════════════════

      ┌───────────────────────────────────────┐
      │         COMPOSANT APPELANT            │
      │  (StrongFather, BondingBrother,       │
      │   Opérateur via BondingBrother)       │
      └───────────────────────────────────────┘
                          │
                          │ Interrogation
                          │ "Quelles capacités ?"
                          │ "Quelles permissions ?"
                          │ "Quel contexte ?"
                          ▼
      ┌───────────────────────────────────────┐
      │            MASTER BUTLER              │
      │                                       │
      │  ┌─────────────────────────────────┐  │
      │  │  OPÉRATIONS AUTORISÉES          │  │
      │  │                                 │  │
      │  │  • getCapabilities()            │  │
      │  │  • getPermissions()             │  │
      │  │  • getCapabilityContext()       │  │
      │  │  • discoverTools()              │  │
      │  │  • getAssociations()            │  │
      │  └─────────────────────────────────┘  │
      │                                       │
      │  ┌─────────────────────────────────┐  │
      │  │  OPÉRATIONS INTERDITES          │  │
      │  │                                 │  │
      │  │  ❌ isAuthorized()              │  │
      │  │  ❌ validatePermission()        │  │
      │  │  ❌ executeTool()               │  │
      │  │  ❌ blockCapability()           │  │
      │  │  ❌ enforcePolicy()             │  │
      │  └─────────────────────────────────┘  │
      └───────────────────────────────────────┘
                          │
                          │ Réponse
                          │ (Informations uniquement,
                          │  jamais de décision)
                          ▼
      ┌───────────────────────────────────────┐
      │         COMPOSANT APPELANT            │
      │                                       │
      │  Utilise les informations pour :      │
      │  • StrongFather : prendre une décision│
      │  • BondingBrother : traduire          │
      │  • Opérateur : découvrir              │
      └───────────────────────────────────────┘
```

### 7.4. Matrice des responsabilités et limites

```
MATRICE DES RESPONSABILITÉS
═══════════════════════════

┌────────────────────────┬──────────┬─────────────┬───────────┬──────────┐
│ ACTION                 │ Master   │ Strong      │ Kind      │ Ever     │
│                        │ Butler   │ Father      │ Mother    │ Buddy    │
├────────────────────────┼──────────┼─────────────┼───────────┼──────────┤
│ Recenser capacités     │ ✅ OUI   │ ❌ Non      │ ❌ Non    │ ❌ Non   │
│ Définir permissions    │ ✅ OUI   │ ❌ Non      │ ❌ Non    │ ❌ Non   │
│ Cataloguer Outils      │ ✅ OUI   │ ❌ Non      │ ❌ Non    │ ❌ Non   │
│ Fournir découverte     │ ✅ OUI   │ ❌ Non      │ ❌ Non    │ ❌ Non   │
├────────────────────────┼──────────┼─────────────┼───────────┼──────────┤
│ Décider autorisation   │ ❌ NON   │ ✅ Oui      │ ❌ Non    │ ❌ Non   │
│ Vérifier permissions   │ ❌ NON   │ ✅ Oui      │ ❌ Non    │ ❌ Non   │
│ Appliquer politiques   │ ❌ NON   │ ✅ Oui      │ ❌ Non    │ ❌ Non   │
├────────────────────────┼──────────┼─────────────┼───────────┼──────────┤
│ Persister données      │ ❌ NON   │ ❌ Non      │ ✅ Oui    │ ❌ Non   │
│ Gérer cohérence        │ ❌ NON   │ ❌ Non      │ ✅ Oui    │ ❌ Non   │
├────────────────────────┼──────────┼─────────────┼───────────┼──────────┤
│ Gérer versions         │ ❌ NON   │ ❌ Non      │ ❌ Non    │ ✅ Oui   │
│ Gérer dépréciation     │ ❌ NON   │ ❌ Non      │ ❌ Non    │ ✅ Oui   │
│ Gérer migration        │ ❌ NON   │ ❌ Non      │ ❌ Non    │ ✅ Oui   │
├────────────────────────┼──────────┼─────────────┼───────────┼──────────┤
│ Exécuter Outils        │ ❌ NON   │ ❌ Non      │ ❌ Non    │ ❌ Non   │
│ (→ Strate 6)           │          │             │           │          │
├────────────────────────┼──────────┼─────────────┼───────────┼──────────┤
│ Implémenter Outils     │ ❌ NON   │ ❌ Non      │ ❌ Non    │ ❌ Non   │
│ (→ Strate 6)           │          │             │           │          │
└────────────────────────┴──────────┴─────────────┴───────────┴──────────┘

LÉGENDE :
✅ OUI = Responsabilité exclusive
❌ NON = Limite absolue, jamais
```

---

## 8. Violations des limites d'autorité

### Définition d'une violation

Une **violation des limites d'autorité** est toute implémentation, configuration, ou comportement de Master Butler qui franchit les limites définies dans ce contrat.

### Gravité des violations

| Catégorie | Exemples | Gravité |
|-----------|----------|---------|
| **V-CRIT** | Master Butler prend une décision d'autorisation | Critique |
| **V-CRIT** | Master Butler exécute un Outil | Critique |
| **V-HIGH** | Master Butler stocke des données métier | Haute |
| **V-HIGH** | Master Butler applique des contraintes métier | Haute |
| **V-MED** | Master Butler persiste directement | Moyenne |
| **V-MED** | Master Butler gère des versions | Moyenne |

### Conséquences des violations

**Violations critiques (V-CRIT) :**
- L'implémentation n'est pas conforme à l'architecture Miyukini
- L'intégrité du système est compromise
- Correction immédiate requise

**Violations hautes (V-HIGH) :**
- L'implémentation dérive de l'architecture
- Des effets de bord indésirables peuvent survenir
- Correction prioritaire requise

**Violations moyennes (V-MED) :**
- L'implémentation contourne les recommandations
- La maintenabilité est compromise
- Correction planifiée requise

### Détection des violations

Les violations peuvent être détectées par :
- **Audit de code** : Vérification que les méthodes de Master Butler respectent les limites
- **Audit d'architecture** : Vérification des flux de données et de décision
- **Tests d'intégration** : Vérification que Master Butler ne prend jamais de décision

---

## 9. Conformité aux Lois d'Autonomie Système

Ce contrat respecte les Lois d'Autonomie Système définies dans [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md).

### LOI-1 : Aucune dépendance externe critique à l'exécution

**Conformité :** Conforme

Les limites d'autorité de Master Butler fonctionnent entièrement localement :

- **Registre local** : Les capacités et permissions sont maintenues localement
- **Interrogations locales** : Toutes les interrogations sont traitées localement
- **Aucune décision externe** : Master Butler ne dépend d'aucun service externe pour ses fonctions fondamentales

### LOI-5 : Le coût doit être proportionnel au hardware

**Conformité :** Conforme

Les limites d'autorité garantissent une empreinte minimale :

- **Registre de métadonnées** : Données légères, empreinte mémoire prévisible
- **Pas d'exécution** : Master Butler ne consomme pas de ressources pour l'exécution
- **Pas de workers** : Pas de processus en arrière-plan

### Synthèse de conformité

| Loi | Statut | Raison |
|-----|--------|--------|
| LOI-1 | ✅ Conforme | Limites fonctionnent localement, aucune dépendance externe |
| LOI-5 | ✅ Conforme | Registre passif, empreinte minimale |

---

## 10. Conclusion

Ce contrat établit les limites absolues de l'autorité de Master Butler dans le système Miyukini.

**Points clés :**

- **Autorité exclusive** : Registre des capacités et permissions, catalogue des Outils, API de découverte
- **Limites absolues** : Ne décide jamais, n'exécute jamais, ne stocke pas de données métier
- **Frontières claires** : Séparation stricte avec StrongFather (décision), KindMother (persistance), Ever Buddy (cycle de vie)
- **Invariants** : Séparation registre/décision, registre passif, agnosticisme métier

**Phrase fondatrice :**

> **Master Butler expose ce qui est possible, sans jamais décider de ce qui est autorisé, sans jamais exécuter ce qui est demandé.**

**Non-négociabilité :** Ce contrat est absolu et non négociable. Le contrat prime sur toute considération pratique.

---

**Document créé le :** 2026-01-27  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, Master Butler Documentation, [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)  
**Type :** Contrat de limites d'autorité non négociable

---

## 11. Mini log — erreurs / warnings / ambiguïtés rencontrées et corrigées

*Aucune erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
