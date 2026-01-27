# Caring Nanny - Propagation Flow Contract

## 1. Contexte

Ce document définit le **contrat normatif du flux de propagation** de Caring Nanny. Le flux de propagation est le mécanisme fondamental par lequel Caring Nanny communique les changements d'état aux composants concernés du système Miyukini.

Le flux de propagation est **strictement passif et informatif** : il transmet des notifications de changement d'état sans jamais modifier l'état lui-même ni déclencher d'action corrective, conformément aux invariants **INV-CN-1** (Observateur pur) et **INV-CN-7** (Propagation fidèle).

Ce contrat est **dérivé de la Documentation Fondatrice de Caring Nanny** (Section 8 - Interactions avec l'écosystème) et complète le **Observation Flow Contract** en définissant ce qui se passe après la détection d'une transition d'état.

**Documents sources :**
- [Caring Nanny - Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)
- [Caring Nanny - Observation Flow Contract](./Caring%20Nanny%20-%20Observation%20Flow%20Contract.md)
- [Caring Nanny - BondingBrother Integration Contract](../integration/Caring%20Nanny%20-%20BondingBrother%20Integration%20Contract.md)

---

## 2. Portée / Scope

- **Applicable à :** Toutes les opérations de propagation de changement d'état dans Caring Nanny
- **Audience :** Architectes, développeurs, intégrateurs, autres cores de l'écosystème
- **Statut :** Contrat normatif — Non négociable
- **Dépendances :** Documentation Fondatrice Caring Nanny, Observation Flow Contract, BondingBrother Integration Contract, Lois d'Autonomie Système

Ce document définit :
- Les quatre étapes du flux de propagation
- Les composants impliqués à chaque étape
- Les règles et contraintes de chaque étape
- Les garanties du flux de propagation
- La relation avec BondingBrother pour la distribution

Ce document **ne couvre pas** :
- Le flux d'observation (voir Caring Nanny - Observation Flow Contract)
- Le flux de consultation (voir Caring Nanny - Consultation Contract)
- Les contrats d'intégration détaillés (voir contracts/integration/)

---

## 3. Relation avec le flux d'observation

### 3.1 Continuité des flux

Le flux de propagation est la **suite logique** du flux d'observation. Lorsque le flux d'observation détecte une transition d'état (étape 4 - Détection de transition), le flux de propagation prend le relais pour communiquer ce changement aux composants concernés.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     ARTICULATION DES FLUX                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  FLUX D'OBSERVATION                                                         │
│  ───────────────────                                                        │
│  Détection → Évaluation → Agrégation → Transition                          │
│                                            │                                │
│                                            │ transition_detected = true     │
│                                            ▼                                │
│  FLUX DE PROPAGATION                                                        │
│  ────────────────────                                                       │
│  Identification → Formulation → Dispatch → Enregistrement                  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Conditions de déclenchement

Le flux de propagation est déclenché **si et seulement si** :
- Une transition d'état a été détectée (état actuel ≠ état précédent)
- La transition nécessite une notification aux composants concernés

**Important :** Si aucune transition n'est détectée, le flux de propagation **n'est pas déclenché**. Les observations sans changement d'état sont enregistrées dans l'historique mais ne génèrent pas de propagation.

---

## 4. Vue d'ensemble du flux de propagation

Le flux de propagation est composé de **quatre étapes séquentielles et obligatoires** :

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        FLUX DE PROPAGATION                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ÉTAPE 1              ÉTAPE 2              ÉTAPE 3              ÉTAPE 4     │
│ IDENTIFICATION  ──►  FORMULATION   ──►    DISPATCH     ──►  ENREGISTREMENT │
│                                                                             │
│  Composants           Notification         Délégation à        Trace de    │
│  concernés      ──►   structurée     ──►   BondingBrother ──►  propagation │
│  identifiés           construite           pour livraison      enregistrée │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Propriétés fondamentales du flux :**

| Propriété | Description | Référence |
|-----------|-------------|-----------|
| **Séquentiel** | Les étapes s'exécutent dans l'ordre, sans saut possible | Architecture 5.2 |
| **Passif** | Aucune modification de l'état du système | INV-CN-1 |
| **Fidèle** | L'information transmise est exactement celle observée | INV-CN-7 |
| **Non-bloquant** | Le flux n'interfère jamais avec les opérations du système | INV-CN-6 |
| **Traçable** | Chaque étape produit des données auditables | INV-CN-5 |
| **Délégué** | La distribution effective est déléguée à BondingBrother | Architecture |

---

## 5. Étape 1 : Identification des destinataires

### 5.1 Définition

L'**identification des destinataires** est le mécanisme par lequel Caring Nanny détermine quels composants doivent être informés d'une transition d'état. La liste des destinataires dépend de la nature de la transition et des abonnements actifs.

### 5.2 Composants impliqués

```
Transition détectée (depuis TransitionDetector)
         │
         ▼
┌─────────────────────────────────────┐
│ RecipientResolver                   │ ← Résolution des destinataires
└────────┬────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────┐
│ SubscriptionRegistry                │ ← Registre des abonnements aux états
└────────┬────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────┐
│ RelevanceFilter                     │ ← Filtrage par pertinence
└────────┬────────────────────────────┘
         │
         ▼
     Liste des destinataires qualifiés
```

### 5.3 Types de destinataires

| Catégorie | Description | Exemples |
|-----------|-------------|----------|
| **Produits** | Applications utilisant les services du système | Product CMS, Product Auth |
| **Modules SPM** | Modules dépendant de l'état d'autres composants | Module Content, Module Search |
| **Cores** | Autres cores de l'écosystème | StrongFather (pour contexte décisionnel) |
| **Services** | Services techniques nécessitant l'état | Monitoring, Alerting |

### 5.4 Critères de qualification

Un destinataire est **qualifié** pour recevoir une notification si :

| Critère | Description |
|---------|-------------|
| **Abonnement actif** | Le destinataire a un abonnement valide aux notifications d'état |
| **Pertinence** | La transition concerne un composant que le destinataire utilise ou observe |
| **Disponibilité** | Le destinataire est atteignable (si non, notification mise en file) |

### 5.5 Règles d'identification

| Règle | Énoncé | Référence |
|-------|--------|-----------|
| **RÈGLE-IDENT-1** | L'identification est basée sur des abonnements **explicites** | Architecture 4.3 |
| **RÈGLE-IDENT-2** | Aucune inférence sur les destinataires potentiels | INV-CN-7 |
| **RÈGLE-IDENT-3** | Les abonnements sont gérés par le produit ou l'écosystème, pas par Caring Nanny | Section 6, Doc Fondatrice |
| **RÈGLE-IDENT-4** | Un destinataire non abonné ne reçoit **jamais** de notification | Principe d'opt-in |
| **RÈGLE-IDENT-5** | L'identification est **non-bloquante** même si le registre est temporairement indisponible | INV-CN-6 |

### 5.6 Format de liste de destinataires

```
RecipientList {
    transition_id     : Identifiant de la transition source
    recipients        : [
        {
            recipient_id    : Identifiant unique du destinataire
            recipient_type  : product | module | core | service
            subscription_id : Référence à l'abonnement actif
            priority        : high | normal | low
            channel_hint    : Canal préféré (optionnel)
        },
        ...
    ]
    timestamp         : Horodatage de l'identification
    qualification_log : Journal des critères de qualification appliqués
}
```

### 5.7 Conditions d'entrée et de sortie

**Entrée :** Une transition d'état détectée (depuis le flux d'observation)

**Sortie :** Une liste de destinataires qualifiés, ou une liste vide si aucun abonné

**Cas particulier :** Si aucun destinataire n'est qualifié, le flux continue mais l'étape de dispatch est simplifiée (enregistrement uniquement).

---

## 6. Étape 2 : Formulation du message

### 6.1 Définition

La **formulation du message** est le mécanisme par lequel Caring Nanny construit la notification structurée qui sera transmise aux destinataires. Le message contient l'état précédent, l'état actuel, la cause de la transition, et le contexte nécessaire.

### 6.2 Composants impliqués

```
Transition détectée + Liste des destinataires
         │
         ▼
┌─────────────────────────────────────┐
│ NotificationBuilder                 │ ← Construction de la notification
└────────┬────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────┐
│ CauseExtractor                      │ ← Extraction de la cause de transition
└────────┬────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────┐
│ ContextEnricher                     │ ← Enrichissement du contexte
└────────┬────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────┐
│ FormatValidator                     │ ← Validation du format BondingBrother
└────────┬────────────────────────────┘
         │
         ▼
     Notification prête pour dispatch
```

### 6.3 Contenu obligatoire de la notification

Chaque notification DOIT contenir les éléments suivants :

| Champ | Description | Exemple |
|-------|-------------|---------|
| `notification_id` | Identifiant unique de la notification | `"cn-not-20260127-143000-001"` |
| `transition_id` | Référence à la transition source | `"cn-trans-20260127-143000-001"` |
| `source_component` | Composant dont l'état a changé | `"kindmother"` |
| `source_type` | Type du composant source | `"core"` |
| `previous_state` | État avant la transition | `"healthy"` |
| `current_state` | État après la transition | `"syncing"` |
| `cause` | Cause identifiable de la transition | `"delta_propagation_started"` |
| `timestamp` | Horodatage local de la transition | `"2026-01-27T14:30:00.000"` |
| `recipients` | Liste des destinataires identifiés | `["product_cms", "module_content"]` |

### 6.4 Contenu optionnel enrichi

| Champ | Description | Condition |
|-------|-------------|-----------|
| `trigger_condition` | Condition qui a déclenché la transition | Si disponible |
| `partial_states` | États partiels contributifs à l'agrégation | Si pertinent |
| `severity` | Niveau de sévérité (info, warning, critical) | Selon la transition |
| `metadata` | Métadonnées additionnelles | Selon le contexte |

### 6.5 Règles de formulation

| Règle | Énoncé | Référence |
|-------|--------|-----------|
| **RÈGLE-FORM-1** | La notification est une **information pure**, jamais une instruction | INV-CN-7 |
| **RÈGLE-FORM-2** | Le contenu est **exactement** ce qui a été observé, sans interprétation | INV-CN-7 |
| **RÈGLE-FORM-3** | Aucune **recommandation d'action** n'est incluse | INV-CN-1 |
| **RÈGLE-FORM-4** | Le format est **compatible** avec BondingBrother | BB Integration Contract |
| **RÈGLE-FORM-5** | La cause est **identifiable et factuelle** | Section 4, Doc Fondatrice |
| **RÈGLE-FORM-6** | L'horodatage est **local** (pas de temps global) | LOI-4 |

### 6.6 Format de notification structurée

```
StateNotification {
    notification_id   : string
    transition_id     : string
    source : {
        component_id  : string
        component_type: core | module | service
    }
    transition : {
        previous_state: healthy | degraded | offline | syncing | error
        current_state : healthy | degraded | offline | syncing | error
        cause         : string (description factuelle)
        trigger       : Condition (référence optionnelle)
    }
    context : {
        timestamp     : LocalTimestamp
        partial_states: PartialState[] (optionnel)
        severity      : info | warning | critical
        metadata      : Map<string, any>
    }
    recipients : RecipientRef[]
}
```

### 6.7 Conditions d'entrée et de sortie

**Entrée :** Une transition d'état et une liste de destinataires

**Sortie :** Une notification structurée prête pour le dispatch

**Échec possible :** Si la notification ne peut pas être formée (cause indéterminable, format invalide), elle est marquée comme `anomaly:formulation_failure` et transmise avec ce marqueur.

---

## 7. Étape 3 : Dispatch (Délégation à BondingBrother)

### 7.1 Définition

Le **dispatch** est le mécanisme par lequel Caring Nanny délègue la distribution de la notification à BondingBrother. Caring Nanny **ne distribue jamais directement** aux destinataires finaux — cette responsabilité appartient à BondingBrother.

### 7.2 Composants impliqués

```
Notification structurée
         │
         ▼
┌─────────────────────────────────────┐
│ PropagationDispatcher               │ ← Orchestration du dispatch
└────────┬────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────┐
│ BondingBrotherChannel               │ ← Canal vers BondingBrother
└────────┬────────────────────────────┘
         │
         │ state_propagation.dispatch(notification)
         ▼
┌─────────────────────────────────────┐
│ BONDING BROTHER                     │ ← Distribution aux destinataires
│ (composant externe)                 │
└────────┬────────────────────────────┘
         │
         ▼
     Confirmation de prise en charge
```

### 7.3 Principes de délégation

| Principe | Description | Référence |
|----------|-------------|-----------|
| **Délégation totale** | BondingBrother gère entièrement la distribution | BB Integration Contract |
| **Non-attente** | Caring Nanny ne attend pas la confirmation de réception des destinataires | INV-CN-6 |
| **Fidélité** | BondingBrother propage sans altération du contenu informationnel | INV-CN-7 |
| **Traduction** | BondingBrother peut traduire le format selon les destinataires | BB Integration Contract |
| **Asynchrone** | Le dispatch est asynchrone et non-bloquant | INV-CN-6 |

### 7.4 Règles de dispatch

| Règle | Énoncé | Référence |
|-------|--------|-----------|
| **RÈGLE-DISP-1** | Caring Nanny DÉLÈGUE la distribution, elle ne distribue JAMAIS directement | Architecture |
| **RÈGLE-DISP-2** | Le dispatch est **asynchrone** et ne bloque jamais le flux d'observation | INV-CN-6 |
| **RÈGLE-DISP-3** | Caring Nanny attend uniquement la **prise en charge** par BondingBrother, pas la livraison | BB Integration Contract |
| **RÈGLE-DISP-4** | En cas d'indisponibilité de BondingBrother, la notification est **mise en file locale** | Résilience |
| **RÈGLE-DISP-5** | Les notifications critiques peuvent utiliser des **canaux alternatifs** | BB Integration Contract INT-OBS-4 |
| **RÈGLE-DISP-6** | Le dispatch n'inclut **aucune instruction d'action** pour les destinataires | INV-CN-1 |

### 7.5 Flux d'interaction avec BondingBrother

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     DISPATCH VERS BONDING BROTHER                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  CARING NANNY                                                               │
│       │                                                                     │
│       │ state_propagation.dispatch({                                        │
│       │     notification_id: "cn-not-12345",                               │
│       │     source: "kindmother",                                          │
│       │     transition: { from: "healthy", to: "syncing" },               │
│       │     cause: "delta_propagation_started",                            │
│       │     timestamp: "2026-01-27T14:30:00Z",                            │
│       │     recipients: ["product_cms", "module_content"]                  │
│       │ })                                                                 │
│       ▼                                                                     │
│  ┌───────────────────────────────────────────────────────────────────────┐│
│  │ BONDING BROTHER                                                       ││
│  │                                                                        ││
│  │ • Reçoit la notification                                              ││
│  │ • Valide la structure                                                 ││
│  │ • Retourne acknowledgment immédiat                                    ││
│  │ • Traduit selon les formats des destinataires                         ││
│  │ • Distribue aux destinataires (asynchrone)                            ││
│  │ • Trace les livraisons                                                ││
│  └───────────────────────────────────────────────────────────────────────┘│
│       │                                                                     │
│       │ ack: { dispatch_id: "bb-prop-67890", status: "accepted" }          │
│       ▼                                                                     │
│  CARING NANNY                                                               │
│  • Enregistre la délégation avec l'identifiant de dispatch                 │
│  • Continue ses observations                                               │
│  • N'attend PAS la confirmation de réception par les destinataires         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 7.6 Gestion de l'indisponibilité de BondingBrother

```
┌─────────────────────────────────────────────────────────────────────────────┐
│           GESTION DE L'INDISPONIBILITÉ DE BONDING BROTHER                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  SCÉNARIO : BondingBrother est indisponible ou en état dégradé             │
│                                                                             │
│  CARING NANNY                                                               │
│       │                                                                     │
│       │ Tentative de dispatch                                              │
│       ▼                                                                     │
│  ┌───────────────────────────────────────────────────────────────────────┐│
│  │ BONDING BROTHER — INDISPONIBLE                                        ││
│  │                                                                        ││
│  │ Timeout ou erreur de connexion                                        ││
│  └───────────────────────────────────────────────────────────────────────┘│
│       │                                                                     │
│       │ Échec de dispatch détecté                                          │
│       ▼                                                                     │
│  ┌───────────────────────────────────────────────────────────────────────┐│
│  │ STRATÉGIE DE RÉSILIENCE                                               ││
│  │                                                                        ││
│  │ 1. Notification mise en FILE LOCALE (PropagationQueue)                ││
│  │ 2. Enregistrement de l'échec dans l'historique                        ││
│  │ 3. Retry automatique lors du rétablissement de BB                     ││
│  │ 4. Pour notifications CRITIQUES : canal alternatif si disponible      ││
│  │                                                                        ││
│  │ IMPORTANT : Caring Nanny ne bloque JAMAIS ses observations            ││
│  │             Le flux d'observation continue indépendamment              ││
│  └───────────────────────────────────────────────────────────────────────┘│
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 7.7 Format de dispatch

```
DispatchRequest {
    notification    : StateNotification
    dispatch_options: {
        priority    : high | normal | low
        retry_policy: immediate | delayed | no_retry
        timeout_ms  : number
    }
}

DispatchAcknowledgment {
    dispatch_id     : string (identifiant côté BondingBrother)
    status          : accepted | queued | rejected
    rejection_reason: string (si rejected)
    timestamp       : LocalTimestamp
}
```

### 7.8 Conditions d'entrée et de sortie

**Entrée :** Une notification structurée prête pour distribution

**Sortie :** Un accusé de réception de BondingBrother (ou une entrée en file locale si indisponible)

**Échec possible :** Si BondingBrother rejette la notification (format invalide, quota dépassé), l'échec est enregistré et la notification peut être reformulée ou abandonnée selon la politique.

---

## 8. Étape 4 : Enregistrement

### 8.1 Définition

L'**enregistrement** est le mécanisme par lequel Caring Nanny trace la propagation dans l'historique pour assurer l'auditabilité complète du flux.

### 8.2 Composants impliqués

```
Dispatch effectué (ou mis en file)
         │
         ▼
┌─────────────────────────────────────┐
│ PropagationLogger                   │ ← Journalisation de la propagation
└────────┬────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────┐
│ PropagationHistory                  │ ← Stockage de l'historique
└────────┬────────────────────────────┘
         │
         ▼
     Propagation tracée et auditable
```

### 8.3 Éléments enregistrés

| Élément | Description | Obligatoire |
|---------|-------------|-------------|
| `propagation_id` | Identifiant unique de la propagation | ✓ |
| `notification_id` | Référence à la notification propagée | ✓ |
| `transition_id` | Référence à la transition source | ✓ |
| `timestamp` | Horodatage de l'enregistrement | ✓ |
| `recipients` | Liste des destinataires identifiés | ✓ |
| `dispatch_status` | Statut du dispatch (success, queued, failed) | ✓ |
| `dispatch_id` | Identifiant BondingBrother (si dispatch réussi) | ○ |
| `failure_reason` | Raison de l'échec (si failed) | ○ |
| `queue_position` | Position en file (si queued) | ○ |

### 8.4 Règles d'enregistrement

| Règle | Énoncé | Référence |
|-------|--------|-----------|
| **RÈGLE-ENR-1** | Chaque propagation est **entièrement traçable** | INV-CN-5 |
| **RÈGLE-ENR-2** | L'enregistrement est **synchrone** avec la fin du dispatch | Auditabilité |
| **RÈGLE-ENR-3** | Les échecs de dispatch sont **enregistrés** avec leur cause | Diagnostic |
| **RÈGLE-ENR-4** | L'historique permet la **corrélation** avec les traces de BondingBrother | BB Integration Contract |
| **RÈGLE-ENR-5** | L'enregistrement **n'échoue jamais** de manière silencieuse | Robustesse |

### 8.5 Format d'enregistrement de propagation

```
PropagationRecord {
    propagation_id    : string
    notification_id   : string
    transition_id     : string
    source : {
        component_id  : string
        component_type: string
    }
    transition : {
        previous_state: string
        current_state : string
    }
    recipients        : RecipientRef[]
    dispatch : {
        status        : success | queued | failed
        dispatch_id   : string (si success)
        queue_position: number (si queued)
        failure_reason: string (si failed)
        channel_used  : primary | alternative
    }
    timestamps : {
        transition_detected : LocalTimestamp
        notification_built  : LocalTimestamp
        dispatch_attempted  : LocalTimestamp
        record_created      : LocalTimestamp
    }
    metadata          : Map<string, any>
}
```

### 8.6 Conditions d'entrée et de sortie

**Entrée :** Un dispatch effectué (ou tenté) avec son résultat

**Sortie :** Un enregistrement de propagation dans l'historique

**Échec possible :** Si l'enregistrement échoue (historique saturé), une alerte est émise mais le flux ne bloque pas (INV-CN-6).

---

## 9. Garanties du flux de propagation

Le flux de propagation garantit les propriétés suivantes, dérivées des invariants de la Documentation Fondatrice :

### 9.1 Garantie de fidélité (INV-CN-7)

> La notification propagée est **exactement** l'information observée, sans interprétation, sans filtrage, sans transformation.

**Vérification :** Le contenu de la notification est construit directement à partir de la transition détectée, sans modification.

### 9.2 Garantie de passivité (INV-CN-1)

> Le flux de propagation ne modifie **jamais** l'état du système.

**Vérification :** À aucune étape, une écriture ou action n'est effectuée sur les composants du système (hors historique de Caring Nanny).

### 9.3 Garantie de non-instruction (INV-CN-1, INV-CN-2)

> Les notifications ne contiennent **jamais** d'instruction d'action pour les destinataires.

**Vérification :** Les notifications sont purement informatives. La décision de réagir appartient aux destinataires.

### 9.4 Garantie de traçabilité (INV-CN-5)

> Chaque propagation est **entièrement traçable** de la transition source jusqu'au dispatch.

**Vérification :** Chaque étape produit des données corrélables enregistrées dans l'historique.

### 9.5 Garantie de non-blocage (INV-CN-6)

> Le flux de propagation ne bloque **jamais** les opérations du système.

**Vérification :** Toutes les opérations sont asynchrones. L'indisponibilité de BondingBrother est gérée par mise en file.

### 9.6 Garantie de délégation (Architecture)

> La distribution effective est **toujours** déléguée à BondingBrother.

**Vérification :** Caring Nanny n'a aucun canal direct vers les destinataires finaux.

### 9.7 Garantie d'autonomie (LOI-1 à LOI-5)

> Le flux de propagation fonctionne **localement**, même en cas d'indisponibilité de BondingBrother.

| Loi | Conformité | Mécanisme |
|-----|------------|-----------|
| **LOI-1** | ✅ | File locale si BondingBrother indisponible |
| **LOI-2** | ✅ | Propagation de l'état `offline` comme état normal |
| **LOI-3** | ✅ | L'historique local est souverain |
| **LOI-4** | ✅ | Horodatage local, pas de temps global requis |
| **LOI-5** | ✅ | Flux léger, file bornée, ressources minimales |

---

## 10. Cas particuliers et anomalies

### 10.1 Aucun destinataire qualifié

**Situation :** La transition est détectée mais aucun abonné n'est qualifié pour la recevoir.

**Comportement :**
- Le flux continue jusqu'à l'enregistrement
- La notification est construite mais non dispatchée
- L'enregistrement mentionne `dispatch_status: no_recipients`
- Pas d'erreur — c'est un comportement normal

### 10.2 BondingBrother indisponible

**Situation :** BondingBrother ne répond pas ou est en état dégradé.

**Comportement :**
- La notification est mise en file locale (PropagationQueue)
- L'enregistrement mentionne `dispatch_status: queued`
- Retry automatique lors du rétablissement
- Pour notifications critiques : canal alternatif si disponible
- Le flux d'observation **continue normalement** (INV-CN-6)

### 10.3 Notification rejetée par BondingBrother

**Situation :** BondingBrother rejette la notification (format invalide, quota, etc.).

**Comportement :**
- L'enregistrement mentionne `dispatch_status: failed` avec la raison
- Selon la politique : reformulation ou abandon
- Pas de retry automatique pour les rejets sur le fond

### 10.4 Propagation de l'état de BondingBrother lui-même

**Situation :** Caring Nanny détecte une transition d'état de BondingBrother.

**Comportement :**
- La notification est construite normalement
- Dispatch via canal alternatif si BondingBrother est dégradé/indisponible
- Sinon, dispatch normal avec monitoring particulier
- Voir [BB Integration Contract](../integration/Caring%20Nanny%20-%20BondingBrother%20Integration%20Contract.md) cas 8.4

### 10.5 Historique saturé

**Situation :** Le PropagationHistory atteint sa capacité maximale.

**Comportement :**
- Les enregistrements les plus anciens sont archivés selon la politique de rétention
- Une alerte est émise
- Le flux continue sans interruption

---

## 11. Invariants applicables au flux

Ce contrat est gouverné par les invariants suivants :

| Invariant | Énoncé | Application au flux |
|-----------|--------|---------------------|
| **INV-CN-1** | Observateur pur | Le flux ne modifie aucun état système |
| **INV-CN-2** | Aucune capacité d'exécution | Le flux ne déclenche aucune action corrective |
| **INV-CN-3** | Non-autoritaire | Le flux n'impose aucune contrainte aux destinataires |
| **INV-CN-4** | État cohérent | Les notifications reflètent un état cohérent |
| **INV-CN-5** | Traçabilité complète | Chaque étape est enregistrée |
| **INV-CN-6** | Non-bloquant | Le flux ne bloque jamais |
| **INV-CN-7** | Propagation fidèle | Les notifications sont exactes et non altérées |

---

## 12. Conformité aux Lois d'Autonomie

Ce contrat respecte les Lois d'Autonomie Système :

| Loi | Conformité | Mécanisme |
|-----|------------|-----------|
| **LOI-1** | ✅ Conforme | File locale en cas d'indisponibilité de BondingBrother |
| **LOI-2** | ✅ Conforme | État `offline` propagé comme état normal |
| **LOI-3** | ✅ Conforme | Historique local souverain |
| **LOI-4** | ✅ Conforme | Horodatage local, pas de temps global |
| **LOI-5** | ✅ Conforme | Flux léger, file bornée, ressources minimales |
| **LOI-6** | ✅ Conforme | Compatible avec fédération via BondingBrother |

**Référence :** [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)

---

## 13. Références croisées

- **Document source :** [Caring Nanny - Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)
- **Flux précédent :** [Caring Nanny - Observation Flow Contract](./Caring%20Nanny%20-%20Observation%20Flow%20Contract.md)
- **Intégration BondingBrother :** [Caring Nanny - BondingBrother Integration Contract](../integration/Caring%20Nanny%20-%20BondingBrother%20Integration%20Contract.md)
- **Modèle d'état :** [Caring Nanny - State Model Contract](./Caring%20Nanny%20-%20State%20Model%20Contract.md)
- **Invariants :** [Caring Nanny - Invariants et Garanties](../governance/Caring%20Nanny%20-%20Invariants%20et%20Garanties.md)
- **Glossaire :** [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- **Lois d'Autonomie :** [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)
- **Connexion Inter-COG :** [Miyukini Conceptual References - Connexion Inter-COG](../../../../reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md)

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Contrat normatif — Non négociable  
**Dérivé de :** Caring Nanny - Documentation Fondatrice v1.6, Section 8  
**Type :** Contrat d'observabilité
