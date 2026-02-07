# Master Butler — BondingBrother Integration Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **Master Butler — BondingBrother Integration Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit les règles d'intégration entre Master Butler (Capability & Permission Core) et BondingBrother (Strate de Liaison Gouvernée).

Ce contrat précise les points d'interaction, les flux de communication, les responsabilités respectives, les invariants d'intégration, et les garanties offertes par cette relation architecturale.

### Portée

Ce contrat s'applique à **toute interaction** entre Master Butler et BondingBrother et définit de manière absolue :
- la nature de la relation entre les deux composants,
- les points d'interaction formels,
- les flux de communication autorisés,
- les responsabilités de chaque composant dans l'intégration,
- ce que l'intégration PEUT et NE PEUT JAMAIS faire,
- les invariants systémiques associés.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :
- **Master Butler — Documentation Fondatrice** : Définition fondamentale du rôle de Master Butler
- **Master Butler — Capability API Contract** : Surface d'appel pour les capacités
- **Master Butler — Permission API Contract** : Surface d'appel pour les permissions
- **Master Butler — Discovery API Contract** : Surface d'appel pour la découverte
- **BondingBrother — Documentation Fondatrice** : Définition fondamentale du rôle de BondingBrother
- **BondingBrother — Strate de Liaison Gouvernée** : Vision architecturale de BondingBrother
- **[Miyukini Conceptual References — Connexion Inter-COG](../../../../reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md)** : Protocoles de liaison inter-COG

Il n'introduit aucune contradiction avec le corpus documentaire existant.

---

## 2. Nature de la relation

### 2.1 Positionnement architectural

Master Butler et BondingBrother occupent des positions distinctes mais complémentaires dans l'architecture Miyukini :

| Composant | Position | Rôle fondamental |
|-----------|----------|------------------|
| **Master Butler** | Core (Strate 4) | Registre des capacités et permissions |
| **BondingBrother** | Strate de Liaison | Traduction et médiation des échanges |

**Relation architecturale :**

```
┌─────────────────────────────────────────────────────────────────┐
│                    FLUX TYPIQUE                                   │
│                                                                   │
│  [Entité externe]                                                 │
│        │                                                          │
│        ▼                                                          │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │               BONDING BROTHER                               ││
│  │                                                              ││
│  │   • Reçoit une intention brute                              ││
│  │   • Traduit en format Miyukini                              ││
│  │   • Interroge Master Butler pour le contexte                ││
│  │   • Transmet à StrongFather pour décision                   ││
│  └─────────────────────────────────────────────────────────────┘│
│        │                     │                                    │
│        │ Interrogation       │ Intention traduite                 │
│        ▼                     ▼                                    │
│  ┌──────────────┐      ┌──────────────┐                          │
│  │Master Butler │      │ StrongFather │                          │
│  │(informations)│      │  (décision)  │                          │
│  └──────────────┘      └──────────────┘                          │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Caractérisation de la relation

**Relation de consultation :** BondingBrother consulte Master Butler pour obtenir des informations sur les capacités et permissions. Cette relation est unidirectionnelle : BondingBrother interroge, Master Butler répond.

**Relation sans autorité :** Ni Master Butler ni BondingBrother ne possèdent d'autorité l'un sur l'autre. BondingBrother ne peut pas modifier le registre de Master Butler. Master Butler ne peut pas influencer la traduction de BondingBrother.

**Relation informationnelle :** Les échanges sont purement informationnels. Master Butler fournit des données, BondingBrother les utilise pour la traduction. Aucune décision n'est prise dans cet échange.

### 2.3 Principe fondamental

> **BondingBrother interroge Master Butler pour comprendre les capacités disponibles, sans jamais obtenir de décision ni d'autorisation.**

Ce principe est non négociable. L'intégration sert à enrichir le contexte de traduction, pas à obtenir des verdicts.

---

## 3. Points d'interaction formels

### 3.1 Vérification d'existence de capacité

**Contexte d'utilisation :**

Lors de la traduction d'une intention, BondingBrother peut avoir besoin de vérifier si une capacité existe dans le système avant de transmettre l'intention à StrongFather.

**Flux d'interaction :**

```
┌─────────────────────────────────────────────────────────────────┐
│           VÉRIFICATION D'EXISTENCE DE CAPACITÉ                   │
│                                                                   │
│  BONDING BROTHER                                                 │
│      │                                                            │
│      │ 1. Reçoit une intention mentionnant une capacité          │
│      │    ex: "créer un contenu"                                 │
│      │                                                            │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  INTERROGATION MASTER BUTLER                              │ │
│  │                                                            │ │
│  │  "La capacité 'content.create' existe-t-elle ?"          │ │
│  │                                                            │ │
│  │  → capability_api.exists("content.create")               │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      │ Réponse : { exists: true, deprecated: false }             │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  BONDING BROTHER CONTINUE                                 │ │
│  │                                                            │ │
│  │  Si exists: true → Poursuit la traduction                │ │
│  │  Si exists: false → Rejette l'intention (capacité inconnue)│ │
│  │                                                            │ │
│  │  NOTE : Ce n'est PAS une décision d'autorisation          │ │
│  │         C'est une validation de forme                     │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

**Règles :**

- **INT-VER-1 :** BondingBrother PEUT interroger Master Butler sur l'existence d'une capacité
- **INT-VER-2 :** La réponse est une information, pas une autorisation
- **INT-VER-3 :** BondingBrother NE DOIT PAS interpréter "exists: true" comme "autorisé"
- **INT-VER-4 :** Un rejet pour capacité inexistante est un rejet de forme, pas de fond

### 3.2 Découverte des capacités d'un module

**Contexte d'utilisation :**

BondingBrother peut avoir besoin de découvrir les capacités disponibles dans un module cible pour traduire correctement une intention vague ou pour préparer le contexte.

**Flux d'interaction :**

```
┌─────────────────────────────────────────────────────────────────┐
│           DÉCOUVERTE DES CAPACITÉS D'UN MODULE                   │
│                                                                   │
│  BONDING BROTHER                                                 │
│      │                                                            │
│      │ 1. Reçoit une intention ciblant un module                 │
│      │    ex: "je veux interagir avec le CMS"                    │
│      │                                                            │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  INTERROGATION MASTER BUTLER                              │ │
│  │                                                            │ │
│  │  "Quelles capacités le module CMS expose-t-il ?"         │ │
│  │                                                            │ │
│  │  → capability_api.discover_by_module("cms")              │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      │ Réponse : { capabilities: [...], total_count: N }         │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  BONDING BROTHER UTILISE                                  │ │
│  │                                                            │ │
│  │  • Enrichit le contexte de traduction                    │ │
│  │  • Prépare les informations pour StrongFather            │ │
│  │  • Ne filtre PAS selon les permissions                   │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

**Règles :**

- **INT-DIS-1 :** BondingBrother PEUT découvrir les capacités d'un module
- **INT-DIS-2 :** La découverte retourne toutes les capacités, sans filtrage par permissions
- **INT-DIS-3 :** BondingBrother utilise ces informations pour enrichir le contexte, pas pour filtrer
- **INT-DIS-4 :** Le filtrage par permissions appartient à StrongFather

### 3.3 Récupération des permissions requises

**Contexte d'utilisation :**

Lors de la préparation d'une intention pour StrongFather, BondingBrother peut récupérer les permissions associées à une capacité pour enrichir le contexte transmis.

**Flux d'interaction :**

```
┌─────────────────────────────────────────────────────────────────┐
│           RÉCUPÉRATION DES PERMISSIONS REQUISES                  │
│                                                                   │
│  BONDING BROTHER                                                 │
│      │                                                            │
│      │ 1. Prépare le contexte pour une intention validée         │
│      │    ex: intention "content.create"                         │
│      │                                                            │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  INTERROGATION MASTER BUTLER                              │ │
│  │                                                            │ │
│  │  "Quelles permissions sont requises pour cette capacité ?"│ │
│  │                                                            │ │
│  │  → capability_api.required_permissions("content.create") │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      │ Réponse : { required_permissions: ["content.write"] }     │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  BONDING BROTHER INCLUT                                   │ │
│  │                                                            │ │
│  │  • Inclut les permissions requises dans le contexte       │ │
│  │  • Transmet à StrongFather avec ces informations         │ │
│  │  • Ne vérifie PAS si le demandeur a ces permissions      │ │
│  │                                                            │ │
│  │  NOTE : La vérification des permissions appartient        │ │
│  │         exclusivement à StrongFather                      │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

**Règles :**

- **INT-PER-1 :** BondingBrother PEUT récupérer les permissions requises pour une capacité
- **INT-PER-2 :** BondingBrother NE DOIT JAMAIS vérifier si le demandeur possède ces permissions
- **INT-PER-3 :** Les informations sont transmises à StrongFather pour décision
- **INT-PER-4 :** BondingBrother ne prend aucune décision basée sur ces permissions

### 3.4 Interrogation pour la traduction inter-COG

**Contexte d'utilisation :**

Dans le cadre d'une visite inter-COG (voir [Connexion Inter-COG](../../../../reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md)), BondingBrother interroge Master Butler pour connaître les capacités exposables du COG hôte.

**Flux d'interaction :**

```
┌─────────────────────────────────────────────────────────────────┐
│           INTERROGATION POUR VISITE INTER-COG                    │
│                                                                   │
│  BONDING BROTHER (Bridge inter-COG)                              │
│      │                                                            │
│      │ 1. Reçoit une demande de visite avec Visit Intent         │
│      │    - requested_services: ["cms", "search"]                │
│      │                                                            │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  INTERROGATION MASTER BUTLER                              │ │
│  │                                                            │ │
│  │  "Quelles capacités sont exposables pour ces services ?" │ │
│  │                                                            │ │
│  │  → capability_api.discover_exposable_capabilities(        │ │
│  │      services: ["cms", "search"],                        │ │
│  │      exposure_level: "inter_cog"                         │ │
│  │    )                                                      │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      │ Réponse : { exposable_capabilities: [...] }               │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  BONDING BROTHER PRÉPARE                                  │ │
│  │                                                            │ │
│  │  • Identifie les capacités exposables                    │ │
│  │  • Transmet à StrongFather pour décision de Visa         │ │
│  │  • Le Visa final est décidé par StrongFather             │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

**Règles :**

- **INT-COG-1 :** BondingBrother PEUT interroger Master Butler sur les capacités exposables
- **INT-COG-2 :** L'exposition est filtrée par niveau (`inter_cog`, `public`, etc.)
- **INT-COG-3 :** La décision d'accorder un Visa appartient à StrongFather
- **INT-COG-4 :** Master Butler ne connaît pas le visiteur, seulement les capacités exposables

---

## 4. Responsabilités dans l'intégration

### 4.1 Responsabilités de BondingBrother

Dans le cadre de cette intégration, BondingBrother est responsable de :

| Responsabilité | Description |
|----------------|-------------|
| **RESP-BB-1** | Formuler des interrogations valides à Master Butler |
| **RESP-BB-2** | Interpréter les réponses comme des informations, pas des décisions |
| **RESP-BB-3** | Enrichir le contexte de traduction avec les informations obtenues |
| **RESP-BB-4** | Transmettre le contexte enrichi à StrongFather |
| **RESP-BB-5** | Ne jamais prendre de décision d'autorisation basée sur les réponses |
| **RESP-BB-6** | Tracer toutes les interrogations pour audit |

### 4.2 Responsabilités de Master Butler

Dans le cadre de cette intégration, Master Butler est responsable de :

| Responsabilité | Description |
|----------------|-------------|
| **RESP-MB-1** | Répondre de manière exhaustive et exacte aux interrogations |
| **RESP-MB-2** | Ne jamais inclure de décision ou de recommandation dans les réponses |
| **RESP-MB-3** | Fournir les informations demandées sans filtrage par permissions du demandeur |
| **RESP-MB-4** | Garantir la disponibilité du registre pour les interrogations |
| **RESP-MB-5** | Tracer toutes les interrogations pour audit |

### 4.3 Responsabilités partagées

| Responsabilité | BondingBrother | Master Butler |
|----------------|----------------|---------------|
| **Traçabilité** | Trace ses interrogations | Trace les réponses fournies |
| **Format d'échange** | Formule selon le contrat | Répond selon le contrat |
| **Cohérence** | Utilise les informations correctement | Fournit des informations cohérentes |

---

## 5. Ce que l'intégration PEUT faire

### 5.1 Opérations autorisées

L'intégration entre BondingBrother et Master Butler PEUT effectuer les opérations suivantes :

**PEUT-INT-1 : Vérification d'existence de capacités**

BondingBrother PEUT vérifier si une capacité existe dans le registre de Master Butler avant de poursuivre une traduction.

**PEUT-INT-2 : Découverte de capacités**

BondingBrother PEUT découvrir les capacités d'un module, d'un type d'action, ou d'un contexte pour enrichir la traduction.

**PEUT-INT-3 : Récupération des permissions requises**

BondingBrother PEUT récupérer les permissions associées à une capacité pour les inclure dans le contexte transmis à StrongFather.

**PEUT-INT-4 : Interrogation pour contexte inter-COG**

BondingBrother PEUT interroger Master Butler sur les capacités exposables dans le cadre d'une visite inter-COG.

**PEUT-INT-5 : Enrichissement du contexte de traduction**

BondingBrother PEUT utiliser les informations de Master Butler pour enrichir le contexte de traduction sans modifier ce contexte au-delà de l'enrichissement informationnel.

**PEUT-INT-6 : Validation de forme**

BondingBrother PEUT rejeter une intention si la capacité référencée n'existe pas (validation de forme, pas de fond).

### 5.2 Garanties associées

Chaque opération autorisée est accompagnée des garanties suivantes :
- Les informations fournies par Master Butler sont exactes et exhaustives
- Les réponses reflètent l'état actuel du registre
- La traçabilité est complète des deux côtés
- Aucune décision n'est prise dans l'échange

---

## 6. Ce que l'intégration NE PEUT JAMAIS faire

### 6.1 Interdictions absolues

L'intégration entre BondingBrother et Master Butler NE PEUT JAMAIS effectuer les actions suivantes. Ces interdictions sont absolues et non négociables.

**INTERDIT-INT-1 : Prise de décision d'autorisation**

L'intégration NE PEUT JAMAIS produire une décision d'autorisation. Les informations échangées ne constituent jamais un verdict "autorisé" ou "refusé".

**INTERDIT-INT-2 : Vérification de permissions du demandeur**

L'intégration NE PEUT JAMAIS vérifier si le demandeur possède effectivement les permissions requises. Cette vérification appartient à StrongFather.

**INTERDIT-INT-3 : Filtrage par permissions**

L'intégration NE PEUT JAMAIS filtrer les capacités retournées selon les permissions du demandeur. Master Butler retourne toutes les capacités demandées, StrongFather filtre.

**INTERDIT-INT-4 : Modification du registre par BondingBrother**

BondingBrother NE PEUT JAMAIS modifier le registre de Master Butler (déclaration, mise à jour, dépréciation). BondingBrother est un consommateur en lecture seule.

**INTERDIT-INT-5 : Exécution de capacités**

L'intégration NE PEUT JAMAIS exécuter une capacité. Master Butler recense, BondingBrother traduit, ni l'un ni l'autre n'exécute.

**INTERDIT-INT-6 : Transmission directe aux produits**

L'intégration NE PEUT JAMAIS transmettre directement des informations aux produits sans passer par les flux de gouvernance (StrongFather).

**INTERDIT-INT-7 : Contournement de StrongFather**

L'intégration NE PEUT JAMAIS contourner StrongFather pour accorder un accès. Les informations obtenues de Master Butler servent à préparer le contexte pour StrongFather, pas à remplacer sa décision.

**INTERDIT-INT-8 : Inférence ou déduction**

BondingBrother NE PEUT JAMAIS déduire ou inférer des informations non fournies explicitement par Master Butler. Toute information non comprise est rejetée ou neutralisée (BB-INV-3).

### 6.2 Justifications

Ces interdictions sont justifiées par :
- le respect du principe de non-décision de Master Butler (INV-MB-2),
- le respect du principe de non-décision de BondingBrother (BB-INV-1),
- la séparation stricte des responsabilités entre cores,
- la souveraineté de StrongFather sur les décisions d'autorisation,
- le maintien de la traçabilité et de l'auditabilité.

---

## 7. Invariants d'intégration

### 7.1 Invariants globaux

**INV-INT-1 : Information pure**

Tous les échanges entre BondingBrother et Master Butler sont des échanges d'information. Aucune décision, aucune autorisation, aucun verdict n'est échangé.

**INV-INT-2 : Lecture seule pour BondingBrother**

BondingBrother est un consommateur en lecture seule de Master Butler. Il ne peut jamais modifier le registre.

**INV-INT-3 : Exhaustivité des réponses**

Master Butler répond de manière exhaustive à toutes les interrogations de BondingBrother. Aucune information n'est filtrée ou masquée.

**INV-INT-4 : Traçabilité bilatérale**

Toute interrogation est tracée côté BondingBrother ET côté Master Butler. La traçabilité est complète et auditable.

**INV-INT-5 : Pas de raccourci**

Aucun raccourci n'est autorisé. BondingBrother ne peut pas déduire, inférer, ou supposer une information non fournie explicitement par Master Butler.

**INV-INT-6 : Souveraineté de StrongFather préservée**

L'intégration préserve la souveraineté de StrongFather sur toutes les décisions d'autorisation. Les informations obtenues préparent le contexte, elles ne remplacent pas la décision.

### 7.2 Invariants de flux

**INV-FLUX-1 : Sens unique de l'interrogation**

Le flux d'interrogation est unidirectionnel : BondingBrother interroge, Master Butler répond. Master Butler ne peut jamais initier une communication vers BondingBrother.

**INV-FLUX-2 : Synchronisation des échanges**

Les échanges sont synchrones. BondingBrother attend la réponse de Master Butler avant de poursuivre.

**INV-FLUX-3 : Atomicité des interrogations**

Chaque interrogation est atomique. Elle est traitée complètement ou pas du tout.

---

## 8. Cas d'utilisation concrets

### 8.1 Traduction d'une intention utilisateur

**Scénario :** Un utilisateur veut créer un article de blog.

```
┌─────────────────────────────────────────────────────────────────┐
│  1. RÉCEPTION DE L'INTENTION BRUTE                               │
│                                                                   │
│  UI → BondingBrother                                             │
│  { action: "create", target: "blog_article", data: {...} }      │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  2. TRADUCTION ET INTERROGATION                                  │
│                                                                   │
│  BondingBrother traduit "blog_article" → capacité "content.create"│
│                                                                   │
│  BondingBrother → Master Butler                                  │
│  "La capacité 'content.create' existe-t-elle ?"                 │
│  "Quelles permissions sont requises ?"                          │
│                                                                   │
│  Master Butler → BondingBrother                                  │
│  { exists: true, required_permissions: ["content.write"] }      │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  3. ENRICHISSEMENT ET TRANSMISSION                               │
│                                                                   │
│  BondingBrother enrichit le contexte avec :                     │
│  - Capacité validée : content.create                            │
│  - Permissions requises : content.write                         │
│  - Contexte utilisateur                                         │
│                                                                   │
│  BondingBrother → StrongFather                                   │
│  { intent: {...}, context: {...} }                              │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  4. DÉCISION (HORS SCOPE DE CETTE INTÉGRATION)                   │
│                                                                   │
│  StrongFather évalue et décide : AUTORISÉ ou REFUSÉ             │
└─────────────────────────────────────────────────────────────────┘
```

### 8.2 Rejet pour capacité inexistante

**Scénario :** Une intention référence une capacité qui n'existe pas.

```
┌─────────────────────────────────────────────────────────────────┐
│  1. RÉCEPTION DE L'INTENTION BRUTE                               │
│                                                                   │
│  UI → BondingBrother                                             │
│  { action: "teleport", target: "user" }                         │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  2. TRADUCTION ET INTERROGATION                                  │
│                                                                   │
│  BondingBrother traduit → capacité "user.teleport" (supposée)   │
│                                                                   │
│  BondingBrother → Master Butler                                  │
│  "La capacité 'user.teleport' existe-t-elle ?"                  │
│                                                                   │
│  Master Butler → BondingBrother                                  │
│  { exists: false }                                              │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  3. REJET DE FORME                                               │
│                                                                   │
│  BondingBrother rejette l'intention                             │
│  Raison : UNKNOWN_CAPABILITY                                    │
│                                                                   │
│  NOTE : Ce n'est PAS une décision d'autorisation                │
│         C'est un rejet de forme (capacité inexistante)          │
│         StrongFather n'est pas impliqué                         │
└─────────────────────────────────────────────────────────────────┘
```

### 8.3 Préparation d'un Visa inter-COG

**Scénario :** Un visiteur demande accès à des services du COG hôte.

```
┌─────────────────────────────────────────────────────────────────┐
│  1. RÉCEPTION DE LA DEMANDE DE VISITE                            │
│                                                                   │
│  Bridge inter-COG → BondingBrother                               │
│  { passport: {...}, visit_intent: {                             │
│      requested_services: ["cms", "search"],                     │
│      security_level: "S2"                                       │
│  }}                                                              │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  2. INTERROGATION DES CAPACITÉS EXPOSABLES                       │
│                                                                   │
│  BondingBrother → Master Butler                                  │
│  "Quelles capacités sont exposables en inter_cog                │
│   pour les services cms et search ?"                            │
│                                                                   │
│  Master Butler → BondingBrother                                  │
│  {                                                              │
│    exposable_capabilities: [                                    │
│      { id: "content.read", service: "cms" },                   │
│      { id: "content.list", service: "cms" },                   │
│      { id: "search.query", service: "search" }                 │
│    ]                                                            │
│  }                                                              │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  3. TRANSMISSION À STRONGFATHER                                  │
│                                                                   │
│  BondingBrother prépare le contexte avec :                      │
│  - Passeport validé structurellement                            │
│  - Capacités exposables identifiées                             │
│  - Niveau de sécurité demandé                                   │
│                                                                   │
│  BondingBrother → StrongFather                                   │
│  { visit_request: {...}, available_capabilities: [...] }        │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  4. DÉCISION DE VISA (HORS SCOPE)                                │
│                                                                   │
│  StrongFather décide du Visa :                                  │
│  - Capacités accordées                                          │
│  - Limites temporelles                                          │
│  - Règles d'exécution                                           │
└─────────────────────────────────────────────────────────────────┘
```

---

## 9. Règles de traçabilité

### 9.1 Éléments à tracer côté BondingBrother

| Élément | Description |
|---------|-------------|
| `interrogation_id` | Identifiant unique de l'interrogation |
| `timestamp` | Horodatage de l'interrogation |
| `operation_type` | Type d'opération (exists, discover, permissions) |
| `parameters` | Paramètres de l'interrogation |
| `response_summary` | Résumé de la réponse reçue |
| `usage` | Comment l'information a été utilisée |

### 9.2 Éléments à tracer côté Master Butler

| Élément | Description |
|---------|-------------|
| `request_id` | Identifiant de la requête (corrélé à interrogation_id) |
| `timestamp` | Horodatage de la réponse |
| `caller` | Identifiant de BondingBrother |
| `operation_type` | Type d'opération |
| `response_content` | Contenu de la réponse |

### 9.3 Corrélation des traces

Les traces des deux côtés DOIVENT être corrélables via un identifiant partagé pour permettre l'audit complet d'un flux d'intégration.

---

## 10. Gestion des erreurs

### 10.1 Erreurs côté Master Butler

| Erreur | Signification | Action BondingBrother |
|--------|---------------|----------------------|
| `CAPABILITY_NOT_FOUND` | Capacité inexistante | Rejeter l'intention (forme) |
| `SERVICE_UNAVAILABLE` | Registre indisponible | Rejeter avec erreur système |
| `INVALID_REQUEST` | Requête mal formée | Corriger et réessayer |

### 10.2 Erreurs côté BondingBrother

| Erreur | Signification | Action Master Butler |
|--------|---------------|---------------------|
| `MALFORMED_INTERROGATION` | Interrogation mal formée | Retourner erreur explicite |
| `UNAUTHORIZED_CALLER` | Appelant non reconnu | Rejeter la requête |

### 10.3 Principe de gestion

> **En cas d'erreur, l'intégration DOIT échouer de manière explicite et traçable. Aucune dégradation silencieuse n'est autorisée.**

---

## 11. Compatibilité avec les invariants existants

### 11.1 Respect des invariants de Master Butler

| Invariant MB | Respect dans l'intégration |
|--------------|---------------------------|
| **INV-MB-1** (Exhaustivité) | ✓ Master Butler répond de manière exhaustive |
| **INV-MB-2** (Non-décision) | ✓ Aucune décision dans les réponses |
| **INV-MB-3** (Idempotence) | ✓ Interrogations idempotentes |
| **INV-MB-5** (Traçabilité) | ✓ Toutes les réponses sont tracées |
| **INV-MB-8** (Accessibilité) | ✓ BondingBrother peut interroger Master Butler |

### 11.2 Respect des invariants de BondingBrother

| Invariant BB | Respect dans l'intégration |
|--------------|---------------------------|
| **BB-INV-1** (Non-décision) | ✓ BondingBrother ne décide jamais |
| **BB-INV-2** (Non-persistance) | ✓ Pas de persistance côté BondingBrother |
| **BB-INV-3** (Non-déduction) | ✓ Pas d'inférence sur les réponses |
| **BB-INV-4** (Traçabilité) | ✓ Toutes les interrogations sont tracées |
| **BB-INV-5** (Rejet d'ambiguïté) | ✓ Réponses ambiguës rejetées |
| **BB-INV-6** (Méfiance) | ✓ Réponses validées structurellement |
| **BB-INV-7** (Contrat) | ✓ Échanges selon ce contrat |

---

## 12. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable les règles d'intégration entre Master Butler et BondingBrother.

Il garantit que :
- l'intégration est purement informationnelle,
- aucune décision n'est prise dans les échanges,
- BondingBrother est un consommateur en lecture seule,
- Master Butler répond de manière exhaustive et exacte,
- la traçabilité est complète et bilatérale,
- la souveraineté de StrongFather est préservée,
- les invariants des deux composants sont respectés.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

**Document créé le :** 2026-01-27  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, Master Butler Documentation Fondatrice, BondingBrother Documentation Fondatrice, [Miyukini Conceptual References — Connexion Inter-COG](../../../../reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md)  
**Type :** Contrat d'intégration non négociable

---

## 13. Mini log — erreurs / warnings / ambiguïtés rencontrées et corrigées

### Ambiguïté A1 : Confusion entre rejet de forme et rejet de fond

**Ambiguïté rencontrée :** Risque de confondre le rejet d'une intention pour capacité inexistante (rejet de forme par BondingBrother) avec un rejet d'autorisation (décision de StrongFather).

**Décision prise :** Clarification explicite dans les sections 3.1 et 8.2 que le rejet pour capacité inexistante est un rejet de forme (UNKNOWN_CAPABILITY), pas une décision d'autorisation.

**Correction effectuée :** Notes explicites ajoutées dans les flux et cas d'utilisation.

### Ambiguïté A2 : Filtrage des capacités par permissions

**Ambiguïté rencontrée :** Risque que BondingBrother filtre les capacités retournées par Master Butler selon les permissions du demandeur.

**Décision prise :** Interdiction explicite INTERDIT-INT-3 et règle INT-DIS-3 précisant que Master Butler retourne toutes les capacités, sans filtrage. Le filtrage appartient à StrongFather.

**Correction effectuée :** Section 6.1 et règles d'interaction rédigées avec clarification.

### Ambiguïté A3 : Rôle de l'intégration dans le contexte inter-COG

**Ambiguïté rencontrée :** Nécessité de clarifier comment l'intégration fonctionne dans le contexte des visites inter-COG.

**Décision prise :** Section 3.4 dédiée à l'interrogation pour visite inter-COG, avec cas d'utilisation 8.3 illustrant le flux complet.

**Correction effectuée :** Sections 3.4 et 8.3 rédigées avec flux explicites.

### Vérification de compatibilité

**Vérification effectuée :** Vérification systématique de la compatibilité avec les invariants de Master Butler (INV-MB-*) et de BondingBrother (BB-INV-*). Aucune contradiction détectée.

**Conclusion :** Le contrat est strictement compatible avec le système contractuel existant. Il formalise l'intégration entre les deux composants dans le respect de leurs rôles respectifs.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
