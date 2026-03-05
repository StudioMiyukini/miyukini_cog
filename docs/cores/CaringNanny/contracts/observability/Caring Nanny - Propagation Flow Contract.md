# Caring Nanny - Propagation Flow Contract

## 1. Contexte

Ce document dÃ©finit le **contrat normatif du flux de propagation** de Caring Nanny. Le flux de propagation est le mÃ©canisme fondamental par lequel Caring Nanny communique les changements d'Ã©tat aux composants concernÃ©s du systÃ¨me Miyukini.

Le flux de propagation est **strictement passif et informatif** : il transmet des notifications de changement d'Ã©tat sans jamais modifier l'Ã©tat lui-mÃªme ni dÃ©clencher d'action corrective, conformÃ©ment aux invariants **INV-CN-1** (Observateur pur) et **INV-CN-7** (Propagation fidÃ¨le).

Ce contrat est **dÃ©rivÃ© de la Documentation Fondatrice de Caring Nanny** (Section 8 - Interactions avec l'Ã©cosystÃ¨me) et complÃ¨te le **Observation Flow Contract** en dÃ©finissant ce qui se passe aprÃ¨s la dÃ©tection d'une transition d'Ã©tat.

**Documents sources :**
- [Caring Nanny - Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)
- [Caring Nanny - Observation Flow Contract](./Caring%20Nanny%20-%20Observation%20Flow%20Contract.md)
- [Caring Nanny - BondingBrother Integration Contract](../integration/Caring%20Nanny%20-%20BondingBrother%20Integration%20Contract.md)

---

## 2. PortÃ©e / Scope

- **Applicable Ã  :** Toutes les opÃ©rations de propagation de changement d'Ã©tat dans Caring Nanny
- **Audience :** Architectes, dÃ©veloppeurs, intÃ©grateurs, autres cores de l'Ã©cosystÃ¨me
- **Statut :** Contrat normatif â€” Non nÃ©gociable
- **DÃ©pendances :** Documentation Fondatrice Caring Nanny, Observation Flow Contract, BondingBrother Integration Contract, Lois d'Autonomie SystÃ¨me

Ce document dÃ©finit :
- Les quatre Ã©tapes du flux de propagation
- Les composants impliquÃ©s Ã  chaque Ã©tape
- Les rÃ¨gles et contraintes de chaque Ã©tape
- Les garanties du flux de propagation
- La relation avec BondingBrother pour la distribution

Ce document **ne couvre pas** :
- Le flux d'observation (voir Caring Nanny - Observation Flow Contract)
- Le flux de consultation (voir Caring Nanny - Consultation Contract)
- Les contrats d'intÃ©gration dÃ©taillÃ©s (voir contracts/integration/)

---

## 3. Relation avec le flux d'observation

### 3.1 ContinuitÃ© des flux

Le flux de propagation est la **suite logique** du flux d'observation. Lorsque le flux d'observation dÃ©tecte une transition d'Ã©tat (Ã©tape 4 - DÃ©tection de transition), le flux de propagation prend le relais pour communiquer ce changement aux composants concernÃ©s.

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                     ARTICULATION DES FLUX                                    â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                             â”‚
â”‚  FLUX D'OBSERVATION                                                         â”‚
â”‚  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€                                                        â”‚
â”‚  DÃ©tection â†’ Ã‰valuation â†’ AgrÃ©gation â†’ Transition                          â”‚
â”‚                                            â”‚                                â”‚
â”‚                                            â”‚ transition_detected = true     â”‚
â”‚                                            â–¼                                â”‚
â”‚  FLUX DE PROPAGATION                                                        â”‚
â”‚  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€                                                       â”‚
â”‚  Identification â†’ Formulation â†’ Dispatch â†’ Enregistrement                  â”‚
â”‚                                                                             â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 3.2 Conditions de dÃ©clenchement

Le flux de propagation est dÃ©clenchÃ© **si et seulement si** :
- Une transition d'Ã©tat a Ã©tÃ© dÃ©tectÃ©e (Ã©tat actuel â‰  Ã©tat prÃ©cÃ©dent)
- La transition nÃ©cessite une notification aux composants concernÃ©s

**Important :** Si aucune transition n'est dÃ©tectÃ©e, le flux de propagation **n'est pas dÃ©clenchÃ©**. Les observations sans changement d'Ã©tat sont enregistrÃ©es dans l'historique mais ne gÃ©nÃ¨rent pas de propagation.

---

## 4. Vue d'ensemble du flux de propagation

Le flux de propagation est composÃ© de **quatre Ã©tapes sÃ©quentielles et obligatoires** :

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                        FLUX DE PROPAGATION                                   â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                             â”‚
â”‚  Ã‰TAPE 1              Ã‰TAPE 2              Ã‰TAPE 3              Ã‰TAPE 4     â”‚
â”‚ IDENTIFICATION  â”€â”€â–º  FORMULATION   â”€â”€â–º    DISPATCH     â”€â”€â–º  ENREGISTREMENT â”‚
â”‚                                                                             â”‚
â”‚  Composants           Notification         DÃ©lÃ©gation Ã         Trace de    â”‚
â”‚  concernÃ©s      â”€â”€â–º   structurÃ©e     â”€â”€â–º   BondingBrother â”€â”€â–º  propagation â”‚
â”‚  identifiÃ©s           construite           pour livraison      enregistrÃ©e â”‚
â”‚                                                                             â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**PropriÃ©tÃ©s fondamentales du flux :**

| PropriÃ©tÃ© | Description | RÃ©fÃ©rence |
|-----------|-------------|-----------|
| **SÃ©quentiel** | Les Ã©tapes s'exÃ©cutent dans l'ordre, sans saut possible | Architecture 5.2 |
| **Passif** | Aucune modification de l'Ã©tat du systÃ¨me | INV-CN-1 |
| **FidÃ¨le** | L'information transmise est exactement celle observÃ©e | INV-CN-7 |
| **Non-bloquant** | Le flux n'interfÃ¨re jamais avec les opÃ©rations du systÃ¨me | INV-CN-6 |
| **TraÃ§able** | Chaque Ã©tape produit des donnÃ©es auditables | INV-CN-5 |
| **DÃ©lÃ©guÃ©** | La distribution effective est dÃ©lÃ©guÃ©e Ã  BondingBrother | Architecture |

---

## 5. Ã‰tape 1 : Identification des destinataires

### 5.1 DÃ©finition

L'**identification des destinataires** est le mÃ©canisme par lequel Caring Nanny dÃ©termine quels composants doivent Ãªtre informÃ©s d'une transition d'Ã©tat. La liste des destinataires dÃ©pend de la nature de la transition et des abonnements actifs.

### 5.2 Composants impliquÃ©s

```
Transition dÃ©tectÃ©e (depuis TransitionDetector)
         â”‚
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ RecipientResolver                   â”‚ â† RÃ©solution des destinataires
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ SubscriptionRegistry                â”‚ â† Registre des abonnements aux Ã©tats
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ RelevanceFilter                     â”‚ â† Filtrage par pertinence
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â–¼
     Liste des destinataires qualifiÃ©s
```

### 5.3 Types de destinataires

| CatÃ©gorie | Description | Exemples |
|-----------|-------------|----------|
| **Produits** | Applications utilisant les services du systÃ¨me | Product CMS, Product Auth |
| **Modules SPM** | Modules dÃ©pendant de l'Ã©tat d'autres composants | Module Content, Module Search |
| **Cores** | Autres cores de l'Ã©cosystÃ¨me | StrongFather (pour contexte dÃ©cisionnel) |
| **Services** | Services techniques nÃ©cessitant l'Ã©tat | Monitoring, Alerting |

### 5.4 CritÃ¨res de qualification

Un destinataire est **qualifiÃ©** pour recevoir une notification si :

| CritÃ¨re | Description |
|---------|-------------|
| **Abonnement actif** | Le destinataire a un abonnement valide aux notifications d'Ã©tat |
| **Pertinence** | La transition concerne un composant que le destinataire utilise ou observe |
| **DisponibilitÃ©** | Le destinataire est atteignable (si non, notification mise en file) |

### 5.5 RÃ¨gles d'identification

| RÃ¨gle | Ã‰noncÃ© | RÃ©fÃ©rence |
|-------|--------|-----------|
| **RÃˆGLE-IDENT-1** | L'identification est basÃ©e sur des abonnements **explicites** | Architecture 4.3 |
| **RÃˆGLE-IDENT-2** | Aucune infÃ©rence sur les destinataires potentiels | INV-CN-7 |
| **RÃˆGLE-IDENT-3** | Les abonnements sont gÃ©rÃ©s par le produit ou l'Ã©cosystÃ¨me, pas par Caring Nanny | Section 6, Doc Fondatrice |
| **RÃˆGLE-IDENT-4** | Un destinataire non abonnÃ© ne reÃ§oit **jamais** de notification | Principe d'opt-in |
| **RÃˆGLE-IDENT-5** | L'identification est **non-bloquante** mÃªme si le registre est temporairement indisponible | INV-CN-6 |

### 5.6 Format de liste de destinataires

```
RecipientList {
    transition_id     : Identifiant de la transition source
    recipients        : [
        {
            recipient_id    : Identifiant unique du destinataire
            recipient_type  : product | module | core | service
            subscription_id : RÃ©fÃ©rence Ã  l'abonnement actif
            priority        : high | normal | low
            channel_hint    : Canal prÃ©fÃ©rÃ© (optionnel)
        },
        ...
    ]
    timestamp         : Horodatage de l'identification
    qualification_log : Journal des critÃ¨res de qualification appliquÃ©s
}
```

### 5.7 Conditions d'entrÃ©e et de sortie

**EntrÃ©e :** Une transition d'Ã©tat dÃ©tectÃ©e (depuis le flux d'observation)

**Sortie :** Une liste de destinataires qualifiÃ©s, ou une liste vide si aucun abonnÃ©

**Cas particulier :** Si aucun destinataire n'est qualifiÃ©, le flux continue mais l'Ã©tape de dispatch est simplifiÃ©e (enregistrement uniquement).

---

## 6. Ã‰tape 2 : Formulation du message

### 6.1 DÃ©finition

La **formulation du message** est le mÃ©canisme par lequel Caring Nanny construit la notification structurÃ©e qui sera transmise aux destinataires. Le message contient l'Ã©tat prÃ©cÃ©dent, l'Ã©tat actuel, la cause de la transition, et le contexte nÃ©cessaire.

### 6.2 Composants impliquÃ©s

```
Transition dÃ©tectÃ©e + Liste des destinataires
         â”‚
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ NotificationBuilder                 â”‚ â† Construction de la notification
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ CauseExtractor                      â”‚ â† Extraction de la cause de transition
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ ContextEnricher                     â”‚ â† Enrichissement du contexte
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ FormatValidator                     â”‚ â† Validation du format BondingBrother
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â–¼
     Notification prÃªte pour dispatch
```

### 6.3 Contenu obligatoire de la notification

Chaque notification DOIT contenir les Ã©lÃ©ments suivants :

| Champ | Description | Exemple |
|-------|-------------|---------|
| `notification_id` | Identifiant unique de la notification | `"cn-not-20260127-143000-001"` |
| `transition_id` | RÃ©fÃ©rence Ã  la transition source | `"cn-trans-20260127-143000-001"` |
| `source_component` | Composant dont l'Ã©tat a changÃ© | `"kindmother"` |
| `source_type` | Type du composant source | `"core"` |
| `previous_state` | Ã‰tat avant la transition | `"healthy"` |
| `current_state` | Ã‰tat aprÃ¨s la transition | `"syncing"` |
| `cause` | Cause identifiable de la transition | `"delta_propagation_started"` |
| `timestamp` | Horodatage local de la transition | `"2026-01-27T14:30:00.000"` |
| `recipients` | Liste des destinataires identifiÃ©s | `["product_cms", "module_content"]` |

### 6.4 Contenu optionnel enrichi

| Champ | Description | Condition |
|-------|-------------|-----------|
| `trigger_condition` | Condition qui a dÃ©clenchÃ© la transition | Si disponible |
| `partial_states` | Ã‰tats partiels contributifs Ã  l'agrÃ©gation | Si pertinent |
| `severity` | Niveau de sÃ©vÃ©ritÃ© (info, warning, critical) | Selon la transition |
| `metadata` | MÃ©tadonnÃ©es additionnelles | Selon le contexte |

### 6.5 RÃ¨gles de formulation

| RÃ¨gle | Ã‰noncÃ© | RÃ©fÃ©rence |
|-------|--------|-----------|
| **RÃˆGLE-FORM-1** | La notification est une **information pure**, jamais une instruction | INV-CN-7 |
| **RÃˆGLE-FORM-2** | Le contenu est **exactement** ce qui a Ã©tÃ© observÃ©, sans interprÃ©tation | INV-CN-7 |
| **RÃˆGLE-FORM-3** | Aucune **recommandation d'action** n'est incluse | INV-CN-1 |
| **RÃˆGLE-FORM-4** | Le format est **compatible** avec BondingBrother | BB Integration Contract |
| **RÃˆGLE-FORM-5** | La cause est **identifiable et factuelle** | Section 4, Doc Fondatrice |
| **RÃˆGLE-FORM-6** | L'horodatage est **local** (pas de temps global) | LOI-4 |

### 6.6 Format de notification structurÃ©e

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
        trigger       : Condition (rÃ©fÃ©rence optionnelle)
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

### 6.7 Conditions d'entrÃ©e et de sortie

**EntrÃ©e :** Une transition d'Ã©tat et une liste de destinataires

**Sortie :** Une notification structurÃ©e prÃªte pour le dispatch

**Ã‰chec possible :** Si la notification ne peut pas Ãªtre formÃ©e (cause indÃ©terminable, format invalide), elle est marquÃ©e comme `anomaly:formulation_failure` et transmise avec ce marqueur.

---

## 7. Ã‰tape 3 : Dispatch (DÃ©lÃ©gation Ã  BondingBrother)

### 7.1 DÃ©finition

Le **dispatch** est le mÃ©canisme par lequel Caring Nanny dÃ©lÃ¨gue la distribution de la notification Ã  BondingBrother. Caring Nanny **ne distribue jamais directement** aux destinataires finaux â€” cette responsabilitÃ© appartient Ã  BondingBrother.

### 7.2 Composants impliquÃ©s

```
Notification structurÃ©e
         â”‚
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ PropagationDispatcher               â”‚ â† Orchestration du dispatch
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ BondingBrotherChannel               â”‚ â† Canal vers BondingBrother
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â”‚ state_propagation.dispatch(notification)
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ BONDING BROTHER                     â”‚ â† Distribution aux destinataires
â”‚ (composant externe)                 â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â–¼
     Confirmation de prise en charge
```

### 7.3 Principes de dÃ©lÃ©gation

| Principe | Description | RÃ©fÃ©rence |
|----------|-------------|-----------|
| **DÃ©lÃ©gation totale** | BondingBrother gÃ¨re entiÃ¨rement la distribution | BB Integration Contract |
| **Non-attente** | Caring Nanny ne attend pas la confirmation de rÃ©ception des destinataires | INV-CN-6 |
| **FidÃ©litÃ©** | BondingBrother propage sans altÃ©ration du contenu informationnel | INV-CN-7 |
| **Traduction** | BondingBrother peut traduire le format selon les destinataires | BB Integration Contract |
| **Asynchrone** | Le dispatch est asynchrone et non-bloquant | INV-CN-6 |

### 7.4 RÃ¨gles de dispatch

| RÃ¨gle | Ã‰noncÃ© | RÃ©fÃ©rence |
|-------|--------|-----------|
| **RÃˆGLE-DISP-1** | Caring Nanny DÃ‰LÃˆGUE la distribution, elle ne distribue JAMAIS directement | Architecture |
| **RÃˆGLE-DISP-2** | Le dispatch est **asynchrone** et ne bloque jamais le flux d'observation | INV-CN-6 |
| **RÃˆGLE-DISP-3** | Caring Nanny attend uniquement la **prise en charge** par BondingBrother, pas la livraison | BB Integration Contract |
| **RÃˆGLE-DISP-4** | En cas d'indisponibilitÃ© de BondingBrother, la notification est **mise en file locale** | RÃ©silience |
| **RÃˆGLE-DISP-5** | Les notifications critiques peuvent utiliser des **canaux alternatifs** | BB Integration Contract INT-OBS-4 |
| **RÃˆGLE-DISP-6** | Le dispatch n'inclut **aucune instruction d'action** pour les destinataires | INV-CN-1 |

### 7.5 Flux d'interaction avec BondingBrother

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                     DISPATCH VERS BONDING BROTHER                            â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                             â”‚
â”‚  CARING NANNY                                                               â”‚
â”‚       â”‚                                                                     â”‚
â”‚       â”‚ state_propagation.dispatch({                                        â”‚
â”‚       â”‚     notification_id: "cn-not-12345",                               â”‚
â”‚       â”‚     source: "kindmother",                                          â”‚
â”‚       â”‚     transition: { from: "healthy", to: "syncing" },               â”‚
â”‚       â”‚     cause: "delta_propagation_started",                            â”‚
â”‚       â”‚     timestamp: "2026-01-27T14:30:00Z",                            â”‚
â”‚       â”‚     recipients: ["product_cms", "module_content"]                  â”‚
â”‚       â”‚ })                                                                 â”‚
â”‚       â–¼                                                                     â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”â”‚
â”‚  â”‚ BONDING BROTHER                                                       â”‚â”‚
â”‚  â”‚                                                                        â”‚â”‚
â”‚  â”‚ â€¢ ReÃ§oit la notification                                              â”‚â”‚
â”‚  â”‚ â€¢ Valide la structure                                                 â”‚â”‚
â”‚  â”‚ â€¢ Retourne acknowledgment immÃ©diat                                    â”‚â”‚
â”‚  â”‚ â€¢ Traduit selon les formats des destinataires                         â”‚â”‚
â”‚  â”‚ â€¢ Distribue aux destinataires (asynchrone)                            â”‚â”‚
â”‚  â”‚ â€¢ Trace les livraisons                                                â”‚â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜â”‚
â”‚       â”‚                                                                     â”‚
â”‚       â”‚ ack: { dispatch_id: "bb-prop-67890", status: "accepted" }          â”‚
â”‚       â–¼                                                                     â”‚
â”‚  CARING NANNY                                                               â”‚
â”‚  â€¢ Enregistre la dÃ©lÃ©gation avec l'identifiant de dispatch                 â”‚
â”‚  â€¢ Continue ses observations                                               â”‚
â”‚  â€¢ N'attend PAS la confirmation de rÃ©ception par les destinataires         â”‚
â”‚                                                                             â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 7.6 Gestion de l'indisponibilitÃ© de BondingBrother

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚           GESTION DE L'INDISPONIBILITÃ‰ DE BONDING BROTHER                    â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                             â”‚
â”‚  SCÃ‰NARIO : BondingBrother est indisponible ou en Ã©tat dÃ©gradÃ©             â”‚
â”‚                                                                             â”‚
â”‚  CARING NANNY                                                               â”‚
â”‚       â”‚                                                                     â”‚
â”‚       â”‚ Tentative de dispatch                                              â”‚
â”‚       â–¼                                                                     â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”â”‚
â”‚  â”‚ BONDING BROTHER â€” INDISPONIBLE                                        â”‚â”‚
â”‚  â”‚                                                                        â”‚â”‚
â”‚  â”‚ Timeout ou erreur de connexion                                        â”‚â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜â”‚
â”‚       â”‚                                                                     â”‚
â”‚       â”‚ Ã‰chec de dispatch dÃ©tectÃ©                                          â”‚
â”‚       â–¼                                                                     â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”â”‚
â”‚  â”‚ STRATÃ‰GIE DE RÃ‰SILIENCE                                               â”‚â”‚
â”‚  â”‚                                                                        â”‚â”‚
â”‚  â”‚ 1. Notification mise en FILE LOCALE (PropagationQueue)                â”‚â”‚
â”‚  â”‚ 2. Enregistrement de l'Ã©chec dans l'historique                        â”‚â”‚
â”‚  â”‚ 3. Retry automatique lors du rÃ©tablissement de BB                     â”‚â”‚
â”‚  â”‚ 4. Pour notifications CRITIQUES : canal alternatif si disponible      â”‚â”‚
â”‚  â”‚                                                                        â”‚â”‚
â”‚  â”‚ IMPORTANT : Caring Nanny ne bloque JAMAIS ses observations            â”‚â”‚
â”‚  â”‚             Le flux d'observation continue indÃ©pendamment              â”‚â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜â”‚
â”‚                                                                             â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
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
    dispatch_id     : string (identifiant cÃ´tÃ© BondingBrother)
    status          : accepted | queued | rejected
    rejection_reason: string (si rejected)
    timestamp       : LocalTimestamp
}
```

### 7.8 Conditions d'entrÃ©e et de sortie

**EntrÃ©e :** Une notification structurÃ©e prÃªte pour distribution

**Sortie :** Un accusÃ© de rÃ©ception de BondingBrother (ou une entrÃ©e en file locale si indisponible)

**Ã‰chec possible :** Si BondingBrother rejette la notification (format invalide, quota dÃ©passÃ©), l'Ã©chec est enregistrÃ© et la notification peut Ãªtre reformulÃ©e ou abandonnÃ©e selon la politique.

---

## 8. Ã‰tape 4 : Enregistrement

### 8.1 DÃ©finition

L'**enregistrement** est le mÃ©canisme par lequel Caring Nanny trace la propagation dans l'historique pour assurer l'auditabilitÃ© complÃ¨te du flux.

### 8.2 Composants impliquÃ©s

```
Dispatch effectuÃ© (ou mis en file)
         â”‚
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ PropagationLogger                   â”‚ â† Journalisation de la propagation
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ PropagationHistory                  â”‚ â† Stockage de l'historique
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â–¼
     Propagation tracÃ©e et auditable
```

### 8.3 Ã‰lÃ©ments enregistrÃ©s

| Ã‰lÃ©ment | Description | Obligatoire |
|---------|-------------|-------------|
| `propagation_id` | Identifiant unique de la propagation | âœ“ |
| `notification_id` | RÃ©fÃ©rence Ã  la notification propagÃ©e | âœ“ |
| `transition_id` | RÃ©fÃ©rence Ã  la transition source | âœ“ |
| `timestamp` | Horodatage de l'enregistrement | âœ“ |
| `recipients` | Liste des destinataires identifiÃ©s | âœ“ |
| `dispatch_status` | Statut du dispatch (success, queued, failed) | âœ“ |
| `dispatch_id` | Identifiant BondingBrother (si dispatch rÃ©ussi) | â—‹ |
| `failure_reason` | Raison de l'Ã©chec (si failed) | â—‹ |
| `queue_position` | Position en file (si queued) | â—‹ |

### 8.4 RÃ¨gles d'enregistrement

| RÃ¨gle | Ã‰noncÃ© | RÃ©fÃ©rence |
|-------|--------|-----------|
| **RÃˆGLE-ENR-1** | Chaque propagation est **entiÃ¨rement traÃ§able** | INV-CN-5 |
| **RÃˆGLE-ENR-2** | L'enregistrement est **synchrone** avec la fin du dispatch | AuditabilitÃ© |
| **RÃˆGLE-ENR-3** | Les Ã©checs de dispatch sont **enregistrÃ©s** avec leur cause | Diagnostic |
| **RÃˆGLE-ENR-4** | L'historique permet la **corrÃ©lation** avec les traces de BondingBrother | BB Integration Contract |
| **RÃˆGLE-ENR-5** | L'enregistrement **n'Ã©choue jamais** de maniÃ¨re silencieuse | Robustesse |

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

### 8.6 Conditions d'entrÃ©e et de sortie

**EntrÃ©e :** Un dispatch effectuÃ© (ou tentÃ©) avec son rÃ©sultat

**Sortie :** Un enregistrement de propagation dans l'historique

**Ã‰chec possible :** Si l'enregistrement Ã©choue (historique saturÃ©), une alerte est Ã©mise mais le flux ne bloque pas (INV-CN-6).

---

## 9. Garanties du flux de propagation

Le flux de propagation garantit les propriÃ©tÃ©s suivantes, dÃ©rivÃ©es des invariants de la Documentation Fondatrice :

### 9.1 Garantie de fidÃ©litÃ© (INV-CN-7)

> La notification propagÃ©e est **exactement** l'information observÃ©e, sans interprÃ©tation, sans filtrage, sans transformation.

**VÃ©rification :** Le contenu de la notification est construit directement Ã  partir de la transition dÃ©tectÃ©e, sans modification.

### 9.2 Garantie de passivitÃ© (INV-CN-1)

> Le flux de propagation ne modifie **jamais** l'Ã©tat du systÃ¨me.

**VÃ©rification :** Ã€ aucune Ã©tape, une Ã©criture ou action n'est effectuÃ©e sur les composants du systÃ¨me (hors historique de Caring Nanny).

### 9.3 Garantie de non-instruction (INV-CN-1, INV-CN-2)

> Les notifications ne contiennent **jamais** d'instruction d'action pour les destinataires.

**VÃ©rification :** Les notifications sont purement informatives. La dÃ©cision de rÃ©agir appartient aux destinataires.

### 9.4 Garantie de traÃ§abilitÃ© (INV-CN-5)

> Chaque propagation est **entiÃ¨rement traÃ§able** de la transition source jusqu'au dispatch.

**VÃ©rification :** Chaque Ã©tape produit des donnÃ©es corrÃ©lables enregistrÃ©es dans l'historique.

### 9.5 Garantie de non-blocage (INV-CN-6)

> Le flux de propagation ne bloque **jamais** les opÃ©rations du systÃ¨me.

**VÃ©rification :** Toutes les opÃ©rations sont asynchrones. L'indisponibilitÃ© de BondingBrother est gÃ©rÃ©e par mise en file.

### 9.6 Garantie de dÃ©lÃ©gation (Architecture)

> La distribution effective est **toujours** dÃ©lÃ©guÃ©e Ã  BondingBrother.

**VÃ©rification :** Caring Nanny n'a aucun canal direct vers les destinataires finaux.

### 9.7 Garantie d'autonomie (LOI-1 Ã  LOI-5)

> Le flux de propagation fonctionne **localement**, mÃªme en cas d'indisponibilitÃ© de BondingBrother.

| Loi | ConformitÃ© | MÃ©canisme |
|-----|------------|-----------|
| **LOI-1** | âœ… | File locale si BondingBrother indisponible |
| **LOI-2** | âœ… | Propagation de l'Ã©tat `offline` comme Ã©tat normal |
| **LOI-3** | âœ… | L'historique local est souverain |
| **LOI-4** | âœ… | Horodatage local, pas de temps global requis |
| **LOI-5** | âœ… | Flux lÃ©ger, file bornÃ©e, ressources minimales |

---

## 10. Cas particuliers et anomalies

### 10.1 Aucun destinataire qualifiÃ©

**Situation :** La transition est dÃ©tectÃ©e mais aucun abonnÃ© n'est qualifiÃ© pour la recevoir.

**Comportement :**
- Le flux continue jusqu'Ã  l'enregistrement
- La notification est construite mais non dispatchÃ©e
- L'enregistrement mentionne `dispatch_status: no_recipients`
- Pas d'erreur â€” c'est un comportement normal

### 10.2 BondingBrother indisponible

**Situation :** BondingBrother ne rÃ©pond pas ou est en Ã©tat dÃ©gradÃ©.

**Comportement :**
- La notification est mise en file locale (PropagationQueue)
- L'enregistrement mentionne `dispatch_status: queued`
- Retry automatique lors du rÃ©tablissement
- Pour notifications critiques : canal alternatif si disponible
- Le flux d'observation **continue normalement** (INV-CN-6)

### 10.3 Notification rejetÃ©e par BondingBrother

**Situation :** BondingBrother rejette la notification (format invalide, quota, etc.).

**Comportement :**
- L'enregistrement mentionne `dispatch_status: failed` avec la raison
- Selon la politique : reformulation ou abandon
- Pas de retry automatique pour les rejets sur le fond

### 10.4 Propagation de l'Ã©tat de BondingBrother lui-mÃªme

**Situation :** Caring Nanny dÃ©tecte une transition d'Ã©tat de BondingBrother.

**Comportement :**
- La notification est construite normalement
- Dispatch via canal alternatif si BondingBrother est dÃ©gradÃ©/indisponible
- Sinon, dispatch normal avec monitoring particulier
- Voir [BB Integration Contract](../integration/Caring%20Nanny%20-%20BondingBrother%20Integration%20Contract.md) cas 8.4

### 10.5 Historique saturÃ©

**Situation :** Le PropagationHistory atteint sa capacitÃ© maximale.

**Comportement :**
- Les enregistrements les plus anciens sont archivÃ©s selon la politique de rÃ©tention
- Une alerte est Ã©mise
- Le flux continue sans interruption

---

## 11. Invariants applicables au flux

Ce contrat est gouvernÃ© par les invariants suivants :

| Invariant | Ã‰noncÃ© | Application au flux |
|-----------|--------|---------------------|
| **INV-CN-1** | Observateur pur | Le flux ne modifie aucun Ã©tat systÃ¨me |
| **INV-CN-2** | Aucune capacitÃ© d'exÃ©cution | Le flux ne dÃ©clenche aucune action corrective |
| **INV-CN-3** | Non-autoritaire | Le flux n'impose aucune contrainte aux destinataires |
| **INV-CN-4** | Ã‰tat cohÃ©rent | Les notifications reflÃ¨tent un Ã©tat cohÃ©rent |
| **INV-CN-5** | TraÃ§abilitÃ© complÃ¨te | Chaque Ã©tape est enregistrÃ©e |
| **INV-CN-6** | Non-bloquant | Le flux ne bloque jamais |
| **INV-CN-7** | Propagation fidÃ¨le | Les notifications sont exactes et non altÃ©rÃ©es |

---

## 12. ConformitÃ© aux Lois d'Autonomie

Ce contrat respecte les Lois d'Autonomie SystÃ¨me :

| Loi | ConformitÃ© | MÃ©canisme |
|-----|------------|-----------|
| **LOI-1** | âœ… Conforme | File locale en cas d'indisponibilitÃ© de BondingBrother |
| **LOI-2** | âœ… Conforme | Ã‰tat `offline` propagÃ© comme Ã©tat normal |
| **LOI-3** | âœ… Conforme | Historique local souverain |
| **LOI-4** | âœ… Conforme | Horodatage local, pas de temps global |
| **LOI-5** | âœ… Conforme | Flux lÃ©ger, file bornÃ©e, ressources minimales |
| **LOI-6** | âœ… Conforme | Compatible avec fÃ©dÃ©ration via BondingBrother |

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 13. RÃ©fÃ©rences croisÃ©es

- **Document source :** [Caring Nanny - Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)
- **Flux prÃ©cÃ©dent :** [Caring Nanny - Observation Flow Contract](./Caring%20Nanny%20-%20Observation%20Flow%20Contract.md)
- **IntÃ©gration BondingBrother :** [Caring Nanny - BondingBrother Integration Contract](../integration/Caring%20Nanny%20-%20BondingBrother%20Integration%20Contract.md)
- **ModÃ¨le d'Ã©tat :** [Caring Nanny - State Model Contract](./Caring%20Nanny%20-%20State%20Model%20Contract.md)
- **Invariants :** [Caring Nanny - Invariants et Garanties](../governance/Caring%20Nanny%20-%20Invariants%20et%20Garanties.md)
- **Glossaire :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)
- **Lois d'Autonomie :** [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)
- **Connexion Inter-COG :** [Miyukini Conceptual References - Connexion Inter-COG](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Contrat normatif â€” Non nÃ©gociable  
**DÃ©rivÃ© de :** Caring Nanny - Documentation Fondatrice v1.6, Section 8  
**Type :** Contrat d'observabilitÃ©

