# Caring Nanny — BondingBrother Integration Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **Caring Nanny — BondingBrother Integration Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit les règles d'intégration entre Caring Nanny (Core d'Observation d'État) et BondingBrother (Strate de Liaison Gouvernée).

Ce contrat précise les points d'interaction, les flux de communication, les responsabilités respectives, les invariants d'intégration, et les garanties offertes par cette relation architecturale.

### Portée

Ce contrat s'applique à **toute interaction** entre Caring Nanny et BondingBrother et définit de manière absolue :
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
- **[Caring Nanny — Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)** : Définition fondamentale du rôle de Caring Nanny
- **[Caring Nanny — Architecture et Composants](../../architecture/Caring%20Nanny%20-%20Architecture%20et%20Composants.md)** : Structure architecturale de Caring Nanny
- **[Caring Nanny — Invariants et Garanties](../governance/Caring%20Nanny%20-%20Invariants%20et%20Garanties.md)** : Invariants fondamentaux
- **BondingBrother — Documentation Fondatrice** : Définition fondamentale du rôle de BondingBrother
- **[BondingBrother — Strate de Liaison Gouvernée](../../../BondingBrother/BondingBrother%20-%20Strate%20de%20Liaison%20Gouvernee.md)** : Vision architecturale de BondingBrother
- **[Miyukini Conceptual References — Connexion Inter-COG](../../../../reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md)** : Protocoles de liaison inter-COG

Il n'introduit aucune contradiction avec le corpus documentaire existant.

---

## 2. Nature de la relation

### 2.1 Positionnement architectural

Caring Nanny et BondingBrother occupent des positions distinctes mais complémentaires dans l'architecture Miyukini :

| Composant | Position | Rôle fondamental |
|-----------|----------|------------------|
| **Caring Nanny** | Core (Strate 4) | Observation et propagation des états système |
| **BondingBrother** | Strate de Liaison | Traduction et médiation des échanges |

**Relation architecturale :**

```
┌─────────────────────────────────────────────────────────────────┐
│                    FLUX DE PROPAGATION D'ÉTAT                    │
│                                                                   │
│  [Composant source]                                              │
│        │ Changement d'état détecté                               │
│        ▼                                                          │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                    CARING NANNY                              ││
│  │                                                              ││
│  │   • Observe le changement d'état                            ││
│  │   • Classe selon les catégories (healthy, degraded, etc.)   ││
│  │   • Identifie les destinataires concernés                   ││
│  │   • Formule la notification d'état                          ││
│  │   • Délègue la propagation à BondingBrother                 ││
│  └─────────────────────────────────────────────────────────────┘│
│        │                                                          │
│        │ Notification d'état à propager                          │
│        ▼                                                          │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                  BONDING BROTHER                             ││
│  │                                                              ││
│  │   • Reçoit la notification structurée                       ││
│  │   • Traduit selon les formats des destinataires             ││
│  │   • Propage aux composants/produits concernés               ││
│  │   • Trace la propagation                                    ││
│  └─────────────────────────────────────────────────────────────┘│
│        │                                                          │
│        ▼                                                          │
│  [Produits / Modules / Composants destinataires]                 │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Caractérisation de la relation

**Relation de délégation de propagation :** Caring Nanny observe et formule les notifications d'état, puis délègue leur propagation à BondingBrother. BondingBrother est le vecteur de transmission, jamais l'origine de l'information.

**Relation sans autorité mutuelle :** Ni Caring Nanny ni BondingBrother ne possèdent d'autorité l'un sur l'autre. Caring Nanny ne peut pas influencer la traduction de BondingBrother. BondingBrother ne peut pas modifier l'état observé par Caring Nanny.

**Relation informationnelle unidirectionnelle :** Le flux principal va de Caring Nanny vers BondingBrother. Caring Nanny produit l'information d'état, BondingBrother la transmet fidèlement.

### 2.3 Principe fondamental

> **Caring Nanny observe et formule les changements d'état. BondingBrother propage ces changements aux destinataires concernés, sans jamais altérer, filtrer, ou interpréter l'information d'état.**

Ce principe est non négociable. L'intégration sert à propager l'information d'état, pas à la modifier.

---

## 3. Points d'interaction formels

### 3.1 Transmission de notification d'état

**Contexte d'utilisation :**

Lorsque Caring Nanny détecte une transition d'état (passage d'un état à un autre), elle doit propager cette information aux composants concernés via BondingBrother.

**Flux d'interaction :**

```
┌─────────────────────────────────────────────────────────────────┐
│           TRANSMISSION DE NOTIFICATION D'ÉTAT                    │
│                                                                   │
│  CARING NANNY                                                    │
│      │                                                            │
│      │ 1. Détecte une transition d'état                          │
│      │    ex: KindMother passe de "healthy" à "syncing"         │
│      │                                                            │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  FORMULATION DE LA NOTIFICATION                           │ │
│  │                                                            │ │
│  │  Caring Nanny construit la notification :                 │ │
│  │  {                                                        │ │
│  │    source: "kindmother",                                  │ │
│  │    previous_state: "healthy",                             │ │
│  │    current_state: "syncing",                              │ │
│  │    cause: "delta_propagation_started",                    │ │
│  │    timestamp: <local_timestamp>,                          │ │
│  │    recipients: ["product_x", "module_cms"]                │ │
│  │  }                                                        │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      │ Notification structurée                                   │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  DÉLÉGATION À BONDING BROTHER                             │ │
│  │                                                            │ │
│  │  Caring Nanny → BondingBrother                            │ │
│  │  "Propage cette notification aux destinataires listés"    │ │
│  │                                                            │ │
│  │  → state_propagation.dispatch(notification)               │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      │ Confirmation de prise en charge                           │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  CARING NANNY ENREGISTRE                                  │ │
│  │                                                            │ │
│  │  • Enregistre la propagation dans l'historique            │ │
│  │  • Trace l'identifiant de propagation                     │ │
│  │  • Ne vérifie PAS la réception par les destinataires      │ │
│  │                                                            │ │
│  │  NOTE : La livraison est la responsabilité de BB          │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

**Règles :**

- **INT-NOT-1 :** Caring Nanny DOIT formuler les notifications selon le format contractuel
- **INT-NOT-2 :** Caring Nanny DOIT identifier les destinataires avant la délégation
- **INT-NOT-3 :** Caring Nanny NE DOIT PAS attendre la confirmation de réception des destinataires
- **INT-NOT-4 :** La notification est une information pure, jamais une instruction

### 3.2 Fourniture du contexte d'état pour une intention

**Contexte d'utilisation :**

BondingBrother peut interroger Caring Nanny pour obtenir le contexte d'état actuel lors de la traduction d'une intention. Ce contexte enrichit l'information transmise à StrongFather.

**Flux d'interaction :**

```
┌─────────────────────────────────────────────────────────────────┐
│           FOURNITURE DU CONTEXTE D'ÉTAT                          │
│                                                                   │
│  BONDING BROTHER                                                 │
│      │                                                            │
│      │ 1. Traduit une intention utilisateur                      │
│      │    ex: intention de création de contenu                   │
│      │                                                            │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  INTERROGATION DE CARING NANNY                            │ │
│  │                                                            │ │
│  │  "Quel est l'état actuel des composants concernés ?"     │ │
│  │                                                            │ │
│  │  → state_observation.get_context({                        │ │
│  │      components: ["kindmother", "cms_module"]             │ │
│  │    })                                                     │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      │ Réponse : { states: {...}, global_state: "healthy" }      │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  BONDING BROTHER ENRICHIT                                 │ │
│  │                                                            │ │
│  │  • Inclut le contexte d'état dans l'intention traduite   │ │
│  │  • Transmet à StrongFather avec ce contexte              │ │
│  │  • Ne prend AUCUNE décision basée sur l'état             │ │
│  │                                                            │ │
│  │  NOTE : La décision basée sur l'état appartient          │ │
│  │         exclusivement à StrongFather                      │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

**Règles :**

- **INT-CTX-1 :** BondingBrother PEUT interroger Caring Nanny sur l'état actuel
- **INT-CTX-2 :** La réponse de Caring Nanny est une information, pas une recommandation
- **INT-CTX-3 :** BondingBrother NE DOIT PAS interpréter l'état comme une autorisation
- **INT-CTX-4 :** StrongFather décide seul de l'impact de l'état sur l'intention

### 3.3 Observation de l'état de BondingBrother

**Contexte d'utilisation :**

Caring Nanny observe également l'état de BondingBrother lui-même, comme tout autre composant du système. Cette observation est unidirectionnelle et passive.

**Flux d'interaction :**

```
┌─────────────────────────────────────────────────────────────────┐
│           OBSERVATION DE L'ÉTAT DE BONDING BROTHER              │
│                                                                   │
│  CARING NANNY                                                    │
│      │                                                            │
│      │ 1. Observe l'état de santé de BondingBrother             │
│      │    via les canaux d'observation standards                 │
│      │                                                            │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  DÉTECTION DE CONDITION                                   │ │
│  │                                                            │ │
│  │  Caring Nanny détecte :                                   │ │
│  │  - Temps de réponse de BondingBrother                     │ │
│  │  - Disponibilité des canaux                               │ │
│  │  - Erreurs de propagation                                 │ │
│  │  - Saturation éventuelle                                  │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      │ État observé                                              │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  CLASSIFICATION ET ENREGISTREMENT                         │ │
│  │                                                            │ │
│  │  • Classe l'état de BondingBrother                        │ │
│  │  • Enregistre dans l'historique                           │ │
│  │  • Propage si transition (via autre canal si BB dégradé) │ │
│  │                                                            │ │
│  │  NOTE : Caring Nanny observe BB, ne le contrôle pas       │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

**Règles :**

- **INT-OBS-1 :** Caring Nanny PEUT observer l'état de BondingBrother
- **INT-OBS-2 :** L'observation est passive et sans effet de bord
- **INT-OBS-3 :** Caring Nanny NE PEUT PAS modifier le comportement de BondingBrother
- **INT-OBS-4 :** En cas de dégradation de BB, Caring Nanny utilise des canaux alternatifs

### 3.4 Propagation dans le contexte inter-COG

**Contexte d'utilisation :**

Dans le cadre d'une visite inter-COG (voir [Connexion Inter-COG](../../../../reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md)), Caring Nanny peut fournir l'état du système au Bridge inter-COG de BondingBrother pour enrichir le contexte de vérification.

**Flux d'interaction :**

```
┌─────────────────────────────────────────────────────────────────┐
│           CONTEXTE D'ÉTAT POUR VISITE INTER-COG                  │
│                                                                   │
│  BONDING BROTHER (Bridge inter-COG)                              │
│      │                                                            │
│      │ 1. Reçoit une demande de visite avec Passeport           │
│      │    - Besoin de connaître l'état du COG hôte              │
│      │                                                            │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  INTERROGATION DE CARING NANNY                            │ │
│  │                                                            │ │
│  │  "Quel est l'état global du COG hôte ?"                  │ │
│  │  "Quels services sont en état dégradé ?"                 │ │
│  │                                                            │ │
│  │  → state_observation.get_global_state()                   │ │
│  │  → state_observation.get_degraded_services()              │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      │ Réponse : { global_state: "healthy", degraded: [] }       │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  BONDING BROTHER TRANSMET                                 │ │
│  │                                                            │ │
│  │  • Inclut l'état dans le contexte de vérification        │ │
│  │  • Transmet à StrongFather pour décision de Visa         │ │
│  │  • L'état peut influencer les capacités accordées        │ │
│  │                                                            │ │
│  │  NOTE : StrongFather décide, pas Caring Nanny            │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

**Règles :**

- **INT-COG-1 :** BondingBrother PEUT interroger Caring Nanny sur l'état global du COG
- **INT-COG-2 :** L'état fourni est factuel, sans recommandation d'accès
- **INT-COG-3 :** La décision d'accorder un Visa appartient à StrongFather
- **INT-COG-4 :** Caring Nanny ne connaît pas le visiteur, seulement l'état local

---

## 4. Responsabilités dans l'intégration

### 4.1 Responsabilités de Caring Nanny

Dans le cadre de cette intégration, Caring Nanny est responsable de :

| Responsabilité | Description |
|----------------|-------------|
| **RESP-CN-1** | Formuler des notifications d'état complètes et structurées |
| **RESP-CN-2** | Identifier les destinataires pertinents pour chaque notification |
| **RESP-CN-3** | Déléguer la propagation à BondingBrother via les canaux définis |
| **RESP-CN-4** | Répondre aux interrogations d'état de manière exhaustive et exacte |
| **RESP-CN-5** | Tracer toutes les délégations et interrogations pour audit |
| **RESP-CN-6** | Ne jamais inclure de décision ou recommandation dans les informations |

### 4.2 Responsabilités de BondingBrother

Dans le cadre de cette intégration, BondingBrother est responsable de :

| Responsabilité | Description |
|----------------|-------------|
| **RESP-BB-1** | Propager les notifications d'état aux destinataires identifiés |
| **RESP-BB-2** | Traduire les notifications selon les formats des destinataires |
| **RESP-BB-3** | Ne jamais altérer le contenu informationnel des notifications |
| **RESP-BB-4** | Tracer toutes les propagations effectuées |
| **RESP-BB-5** | Signaler les échecs de propagation (sans bloquer Caring Nanny) |
| **RESP-BB-6** | Interroger Caring Nanny pour enrichir le contexte des intentions |

### 4.3 Responsabilités partagées

| Responsabilité | Caring Nanny | BondingBrother |
|----------------|--------------|----------------|
| **Traçabilité** | Trace ses délégations | Trace ses propagations |
| **Format d'échange** | Formule selon le contrat | Traduit selon les destinataires |
| **Cohérence** | Fournit des informations cohérentes | Transmet sans altération |
| **Non-décision** | Ne recommande jamais | Ne filtre jamais sur le fond |

---

## 5. Ce que l'intégration PEUT faire

### 5.1 Opérations autorisées

L'intégration entre Caring Nanny et BondingBrother PEUT effectuer les opérations suivantes :

**PEUT-INT-1 : Délégation de propagation d'état**

Caring Nanny PEUT déléguer à BondingBrother la propagation des notifications de changement d'état aux destinataires identifiés.

**PEUT-INT-2 : Fourniture de contexte d'état**

Caring Nanny PEUT fournir à BondingBrother le contexte d'état actuel pour enrichir les intentions traduites.

**PEUT-INT-3 : Observation de l'état de BondingBrother**

Caring Nanny PEUT observer l'état de santé de BondingBrother comme tout autre composant du système.

**PEUT-INT-4 : Interrogation d'état pour visite inter-COG**

BondingBrother PEUT interroger Caring Nanny sur l'état global du COG dans le contexte d'une visite inter-COG.

**PEUT-INT-5 : Traduction des notifications**

BondingBrother PEUT traduire les notifications de Caring Nanny selon les formats attendus par les destinataires.

**PEUT-INT-6 : Utilisation de canaux alternatifs**

En cas de dégradation de BondingBrother, Caring Nanny PEUT utiliser des canaux alternatifs pour les propagations critiques.

### 5.2 Garanties associées

Chaque opération autorisée est accompagnée des garanties suivantes :
- Les notifications sont transmises fidèlement, sans altération de contenu
- Les informations d'état fournies sont exactes et à jour
- La traçabilité est complète des deux côtés
- Aucune décision n'est prise dans l'échange
- La propagation est non-bloquante (INV-CN-6)

---

## 6. Ce que l'intégration NE PEUT JAMAIS faire

### 6.1 Interdictions absolues

L'intégration entre Caring Nanny et BondingBrother NE PEUT JAMAIS effectuer les actions suivantes. Ces interdictions sont absolues et non négociables.

**INTERDIT-INT-1 : Modification de l'état par BondingBrother**

BondingBrother NE PEUT JAMAIS modifier l'état observé ou rapporté par Caring Nanny. L'état est en lecture seule pour BondingBrother.

**INTERDIT-INT-2 : Filtrage des notifications sur le fond**

BondingBrother NE PEUT JAMAIS filtrer les notifications de Caring Nanny selon des critères de fond. Il traduit et transmet, il ne juge jamais.

**INTERDIT-INT-3 : Prise de décision basée sur l'état**

Ni Caring Nanny ni BondingBrother NE PEUVENT prendre de décision basée sur l'état observé. Les décisions appartiennent à StrongFather.

**INTERDIT-INT-4 : Médiation d'intentions par Caring Nanny**

Caring Nanny NE PEUT JAMAIS médiatiser des intentions. La médiation est du ressort exclusif de BondingBrother.

**INTERDIT-INT-5 : Action corrective par Caring Nanny**

Caring Nanny NE PEUT JAMAIS déclencher d'action corrective via BondingBrother. Elle informe, elle ne corrige jamais.

**INTERDIT-INT-6 : Blocage des opérations**

L'intégration NE PEUT JAMAIS bloquer les opérations du système. La propagation est non-bloquante (INV-CN-6).

**INTERDIT-INT-7 : Inférence ou enrichissement non autorisé**

BondingBrother NE PEUT JAMAIS enrichir ou inférer des informations non fournies par Caring Nanny. Toute information ajoutée doit être explicitement identifiée comme métadonnée de transport.

**INTERDIT-INT-8 : Contournement de la traçabilité**

L'intégration NE PEUT JAMAIS contourner la traçabilité. Toute délégation et propagation DOIT être enregistrée.

### 6.2 Justifications

Ces interdictions sont justifiées par :
- le respect du principe d'observateur pur de Caring Nanny (INV-CN-1),
- le respect du principe de non-décision de BondingBrother (BB-INV-1),
- le respect du principe de propagation fidèle (INV-CN-7),
- la séparation stricte des responsabilités entre cores,
- la souveraineté de StrongFather sur les décisions,
- le maintien de la traçabilité et de l'auditabilité.

---

## 7. Invariants d'intégration

### 7.1 Invariants globaux

**INV-INT-1 : Information pure**

Tous les échanges entre Caring Nanny et BondingBrother sont des échanges d'information. Aucune décision, aucune instruction d'action, aucune recommandation n'est échangée.

**INV-INT-2 : Fidélité de propagation**

Les notifications propagées par BondingBrother DOIVENT être fidèles à celles formulées par Caring Nanny. Le contenu informationnel est inaltérable.

**INV-INT-3 : Non-blocage**

L'intégration ne bloque jamais. La délégation de propagation est asynchrone et ne bloque pas Caring Nanny. Les interrogations sont synchrones mais ne bloquent pas les observations.

**INV-INT-4 : Traçabilité bilatérale**

Toute délégation est tracée côté Caring Nanny ET toute propagation est tracée côté BondingBrother. La traçabilité est complète et auditable.

**INV-INT-5 : Souveraineté d'observation préservée**

Caring Nanny reste l'unique source de vérité pour l'état observé. BondingBrother ne peut jamais contredire ou modifier cette observation.

**INV-INT-6 : Pas de raccourci**

Aucun raccourci n'est autorisé. BondingBrother ne peut pas déduire, inférer, ou supposer une information d'état non fournie explicitement par Caring Nanny.

### 7.2 Invariants de flux

**INV-FLUX-1 : Délégation unidirectionnelle**

Le flux de délégation de propagation est unidirectionnel : Caring Nanny délègue, BondingBrother propage. BondingBrother ne peut jamais initier une propagation d'état sans délégation de Caring Nanny.

**INV-FLUX-2 : Interrogation bidirectionnelle encadrée**

Le flux d'interrogation permet à BondingBrother d'interroger Caring Nanny pour obtenir le contexte d'état. Ce flux est encadré et ne permet pas de modifier l'état.

**INV-FLUX-3 : Atomicité des notifications**

Chaque notification est atomique. Elle est propagée complètement ou pas du tout. Pas de propagation partielle.

---

## 8. Cas d'utilisation concrets

### 8.1 Propagation d'une transition d'état KindMother

**Scénario :** KindMother passe de l'état "healthy" à "syncing" lors d'une synchronisation de delta.

```
┌─────────────────────────────────────────────────────────────────┐
│  1. DÉTECTION DE LA TRANSITION                                   │
│                                                                   │
│  [Canal d'observation KindMother] → Caring Nanny                │
│  { component: "kindmother", event: "state_change",              │
│    from: "healthy", to: "syncing" }                             │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  2. CLASSIFICATION ET FORMULATION                                │
│                                                                   │
│  Caring Nanny :                                                  │
│  • Classifie : transition normale (healthy → syncing)           │
│  • Identifie les destinataires : produits utilisant KindMother  │
│  • Formule la notification :                                    │
│    {                                                            │
│      notification_id: "cn-not-12345",                          │
│      source: "kindmother",                                      │
│      transition: { from: "healthy", to: "syncing" },           │
│      cause: "delta_propagation_started",                        │
│      timestamp: "2026-01-27T14:30:00Z",                        │
│      recipients: ["product_cms", "module_content"]              │
│    }                                                            │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  3. DÉLÉGATION À BONDING BROTHER                                 │
│                                                                   │
│  Caring Nanny → BondingBrother                                  │
│  state_propagation.dispatch(notification)                       │
│                                                                   │
│  BondingBrother :                                                │
│  • Confirme la prise en charge                                  │
│  • Traduit pour chaque destinataire                             │
│  • Propage via les canaux appropriés                            │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  4. ENREGISTREMENT ET SUIVI                                      │
│                                                                   │
│  Caring Nanny :                                                  │
│  • Enregistre la délégation dans l'historique                   │
│  • Continue ses observations                                     │
│                                                                   │
│  BondingBrother :                                                │
│  • Trace les propagations effectuées                            │
│  • Signale les éventuels échecs (non-bloquant)                  │
└─────────────────────────────────────────────────────────────────┘
```

### 8.2 Enrichissement du contexte d'une intention

**Scénario :** Un utilisateur veut créer un contenu. BondingBrother enrichit l'intention avec le contexte d'état.

```
┌─────────────────────────────────────────────────────────────────┐
│  1. RÉCEPTION DE L'INTENTION                                     │
│                                                                   │
│  UI → BondingBrother                                            │
│  { action: "create", target: "content", data: {...} }           │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  2. INTERROGATION DE CARING NANNY                                │
│                                                                   │
│  BondingBrother → Caring Nanny                                  │
│  "Quel est l'état des composants concernés ?"                   │
│                                                                   │
│  → state_observation.get_context({                              │
│      components: ["kindmother", "cms_module"]                   │
│    })                                                           │
│                                                                   │
│  Caring Nanny → BondingBrother                                  │
│  {                                                              │
│    states: {                                                    │
│      kindmother: "syncing",                                     │
│      cms_module: "healthy"                                      │
│    },                                                           │
│    global_state: "degraded",                                    │
│    timestamp: "2026-01-27T14:30:05Z"                           │
│  }                                                              │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  3. ENRICHISSEMENT ET TRANSMISSION                               │
│                                                                   │
│  BondingBrother enrichit l'intention :                          │
│  {                                                              │
│    intent: { action: "create", target: "content", ... },       │
│    state_context: { global: "degraded", kindmother: "syncing" } │
│  }                                                              │
│                                                                   │
│  BondingBrother → StrongFather                                  │
│  (StrongFather décide si l'opération est autorisée)             │
│                                                                   │
│  NOTE : BondingBrother ne décide PAS que l'opération est        │
│         interdite parce que le système est en état "syncing"    │
└─────────────────────────────────────────────────────────────────┘
```

### 8.3 Contexte d'état pour visite inter-COG

**Scénario :** Un visiteur demande accès au COG. BondingBrother interroge Caring Nanny sur l'état du COG hôte.

```
┌─────────────────────────────────────────────────────────────────┐
│  1. RÉCEPTION DE LA DEMANDE DE VISITE                            │
│                                                                   │
│  Bridge inter-COG → BondingBrother                              │
│  { passport: {...}, visit_intent: {                             │
│      requested_services: ["cms", "search"]                      │
│  }}                                                              │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  2. INTERROGATION DE L'ÉTAT DU COG                               │
│                                                                   │
│  BondingBrother → Caring Nanny                                  │
│  "Quel est l'état global du COG ?"                             │
│  "Y a-t-il des services en état dégradé ?"                     │
│                                                                   │
│  Caring Nanny → BondingBrother                                  │
│  {                                                              │
│    global_state: "healthy",                                     │
│    degraded_services: [],                                       │
│    components_state: {                                          │
│      kindmother: "healthy",                                     │
│      strongfather: "healthy",                                   │
│      cms_service: "healthy",                                    │
│      search_service: "healthy"                                  │
│    }                                                            │
│  }                                                              │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  3. TRANSMISSION À STRONGFATHER                                  │
│                                                                   │
│  BondingBrother prépare le contexte :                           │
│  • Passeport validé structurellement                            │
│  • État du COG fourni par Caring Nanny                          │
│  • Services demandés                                            │
│                                                                   │
│  BondingBrother → StrongFather                                  │
│  { visit_request: {...}, cog_state: {...} }                    │
│                                                                   │
│  StrongFather décide du Visa avec connaissance de l'état        │
└─────────────────────────────────────────────────────────────────┘
```

### 8.4 Dégradation de BondingBrother détectée

**Scénario :** Caring Nanny détecte que BondingBrother est en état dégradé.

```
┌─────────────────────────────────────────────────────────────────┐
│  1. DÉTECTION DE LA DÉGRADATION                                  │
│                                                                   │
│  Caring Nanny observe :                                         │
│  • Temps de réponse de BondingBrother augmenté                  │
│  • Erreurs de propagation fréquentes                            │
│                                                                   │
│  Classification : BondingBrother passe de "healthy" à "degraded"│
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  2. NOTIFICATION DE LA DÉGRADATION                               │
│                                                                   │
│  Caring Nanny formule une notification :                        │
│  {                                                              │
│    source: "bondingbrother",                                    │
│    transition: { from: "healthy", to: "degraded" },            │
│    cause: "high_latency_detected",                              │
│    recipients: ["strongfather", "monitoring_service"]           │
│  }                                                              │
│                                                                   │
│  PROBLÈME : BondingBrother est le canal de propagation habituel │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  3. UTILISATION DE CANAUX ALTERNATIFS                            │
│                                                                   │
│  Caring Nanny :                                                  │
│  • Utilise un canal de propagation de secours                   │
│  • Ou enregistre localement en attente de rétablissement        │
│  • Signale l'état critique aux composants critiques             │
│                                                                   │
│  NOTE : Caring Nanny n'essaie PAS de corriger BondingBrother   │
│         Elle informe, elle n'agit jamais                        │
└─────────────────────────────────────────────────────────────────┘
```

---

## 9. Règles de traçabilité

### 9.1 Éléments à tracer côté Caring Nanny

| Élément | Description |
|---------|-------------|
| `delegation_id` | Identifiant unique de la délégation |
| `timestamp` | Horodatage de la délégation |
| `notification_content` | Contenu de la notification déléguée |
| `recipients` | Liste des destinataires identifiés |
| `bondingbrother_ack` | Confirmation de prise en charge par BB |

### 9.2 Éléments à tracer côté BondingBrother

| Élément | Description |
|---------|-------------|
| `propagation_id` | Identifiant de la propagation (corrélé à delegation_id) |
| `timestamp` | Horodatage de la propagation |
| `source` | Identifiant de Caring Nanny |
| `recipients_reached` | Destinataires effectivement atteints |
| `delivery_status` | Statut de livraison par destinataire |

### 9.3 Éléments à tracer pour les interrogations

| Élément | Description |
|---------|-------------|
| `query_id` | Identifiant unique de l'interrogation |
| `timestamp` | Horodatage de l'interrogation |
| `requester` | BondingBrother (identifiant) |
| `query_type` | Type d'interrogation (context, global_state, etc.) |
| `response_summary` | Résumé de la réponse fournie |

### 9.4 Corrélation des traces

Les traces des deux côtés DOIVENT être corrélables via un identifiant partagé pour permettre l'audit complet d'un flux d'intégration.

---

## 10. Gestion des erreurs

### 10.1 Erreurs côté Caring Nanny

| Erreur | Signification | Action BondingBrother |
|--------|---------------|----------------------|
| `STATE_UNAVAILABLE` | État non observable temporairement | Transmettre sans contexte d'état |
| `COMPONENT_UNKNOWN` | Composant non reconnu | Ignorer le composant dans la requête |
| `INTERNAL_ERROR` | Erreur interne Caring Nanny | Réessayer ou procéder sans contexte |

### 10.2 Erreurs côté BondingBrother

| Erreur | Signification | Action Caring Nanny |
|--------|---------------|---------------------|
| `PROPAGATION_FAILED` | Échec de propagation | Enregistrer l'échec, réessayer si critique |
| `RECIPIENT_UNREACHABLE` | Destinataire non atteignable | Enregistrer, ne pas bloquer |
| `SERVICE_DEGRADED` | BondingBrother en dégradation | Utiliser canal alternatif si disponible |

### 10.3 Principe de gestion

> **En cas d'erreur, l'intégration DOIT échouer de manière explicite et traçable. Caring Nanny ne bloque jamais ses observations en attendant la propagation. BondingBrother ne bloque jamais ses traductions en attendant le contexte d'état.**

---

## 11. Compatibilité avec les invariants existants

### 11.1 Respect des invariants de Caring Nanny

| Invariant CN | Respect dans l'intégration |
|--------------|---------------------------|
| **INV-CN-1** (Observateur pur) | ✓ Caring Nanny observe et informe, jamais n'agit |
| **INV-CN-2** (Aucune exécution) | ✓ Aucune action corrective déclenchée |
| **INV-CN-3** (Non-autoritaire) | ✓ Aucune autorité exercée sur BondingBrother |
| **INV-CN-4** (État cohérent) | ✓ Informations d'état cohérentes fournies |
| **INV-CN-5** (Traçabilité) | ✓ Toutes les délégations sont tracées |
| **INV-CN-6** (Non-bloquant) | ✓ Délégation asynchrone et non-bloquante |
| **INV-CN-7** (Propagation fidèle) | ✓ Notifications transmises sans altération |

### 11.2 Respect des invariants de BondingBrother

| Invariant BB | Respect dans l'intégration |
|--------------|---------------------------|
| **BB-INV-1** (Non-décision) | ✓ BondingBrother ne décide jamais sur la base de l'état |
| **BB-INV-2** (Non-persistance) | ✓ Pas de persistance d'état côté BondingBrother |
| **BB-INV-3** (Non-déduction) | ✓ Pas d'inférence sur les informations d'état |
| **BB-INV-4** (Traçabilité) | ✓ Toutes les propagations sont tracées |
| **BB-INV-5** (Rejet d'ambiguïté) | ✓ Notifications ambiguës rejetées |
| **BB-INV-6** (Méfiance) | ✓ Notifications validées structurellement |
| **BB-INV-7** (Contrat) | ✓ Échanges selon ce contrat |

---

## 12. Conformité aux Lois d'Autonomie Système

### 12.1 LOI-1 : Aucune dépendance externe critique

L'intégration respecte LOI-1 :
- La propagation via BondingBrother n'est pas bloquante
- En cas d'indisponibilité de BondingBrother, Caring Nanny continue ses observations
- Les notifications peuvent être mises en file locale en attendant le rétablissement

### 12.2 LOI-2 : L'isolement comme état normal

L'intégration respecte LOI-2 :
- L'état "offline" est propagé comme un état normal, pas comme une erreur
- BondingBrother traduit correctement l'état d'isolement aux destinataires

### 12.3 LOI-3 : L'état local est souverain

L'intégration respecte LOI-3 :
- Caring Nanny est l'unique source de vérité pour l'état local
- BondingBrother ne peut jamais contredire l'état rapporté

### 12.4 LOI-4 : Pas de temps global requis

L'intégration respecte LOI-4 :
- Les horodatages sont locaux (kernel Clock)
- Aucune synchronisation temporelle n'est requise pour la propagation

### 12.5 LOI-5 : Coût proportionnel au hardware

L'intégration respecte LOI-5 :
- La propagation est non-bloquante et légère
- Pas de workers permanents dédiés à l'intégration

---

## 13. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable les règles d'intégration entre Caring Nanny et BondingBrother.

Il garantit que :
- Caring Nanny observe, formule, et délègue les notifications d'état,
- BondingBrother propage fidèlement ces notifications aux destinataires,
- aucune décision n'est prise dans les échanges,
- aucune modification d'état n'est effectuée par BondingBrother,
- la traçabilité est complète et bilatérale,
- l'intégration est non-bloquante et résiliente,
- les invariants des deux composants sont respectés,
- la conformité aux Lois d'Autonomie Système est maintenue.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

**Document créé le :** 2026-01-27  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, [Caring Nanny Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md), [BondingBrother — Strate de Liaison Gouvernée](../../../BondingBrother/BondingBrother%20-%20Strate%20de%20Liaison%20Gouvernee.md), [Miyukini Conceptual References — Connexion Inter-COG](../../../../reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md)  
**Type :** Contrat d'intégration non négociable

---

## 14. Mini log — erreurs / warnings / ambiguïtés rencontrées et corrigées

### Ambiguïté A1 : Direction du flux principal

**Ambiguïté rencontrée :** Confusion possible sur qui initie les échanges — Caring Nanny ou BondingBrother ?

**Décision prise :** Clarification explicite que le flux principal est la délégation de propagation (CN → BB), avec un flux secondaire d'interrogation (BB → CN pour le contexte).

**Correction effectuée :** Section 2.2 et diagrammes rédigés avec flux explicites.

### Ambiguïté A2 : Observation de BondingBrother par Caring Nanny

**Ambiguïté rencontrée :** Caring Nanny observe tous les composants, y compris BondingBrother. Comment gérer la propagation de l'état de BB via BB lui-même ?

**Décision prise :** Caring Nanny PEUT utiliser des canaux alternatifs en cas de dégradation de BondingBrother. Cas d'utilisation 8.4 ajouté pour illustrer ce scénario.

**Correction effectuée :** Section 3.3 et cas d'utilisation 8.4 rédigés avec cette clarification.

### Ambiguïté A3 : Rôle de l'état dans les décisions

**Ambiguïté rencontrée :** Risque que BondingBrother utilise le contexte d'état pour prendre des décisions (ex: refuser une intention car le système est "syncing").

**Décision prise :** Interdiction explicite INTERDIT-INT-3 et règle INT-CTX-3 précisant que seul StrongFather décide sur la base de l'état.

**Correction effectuée :** Sections 3.2, 6.1 et cas d'utilisation 8.2 rédigés avec clarification.

### Vérification de compatibilité

**Vérification effectuée :** Vérification systématique de la compatibilité avec les invariants de Caring Nanny (INV-CN-*) et de BondingBrother (BB-INV-*). Aucune contradiction détectée.

**Conclusion :** Le contrat est strictement compatible avec le système contractuel existant. Il formalise l'intégration entre les deux composants dans le respect de leurs rôles respectifs : Caring Nanny observe et informe, BondingBrother traduit et propage.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
