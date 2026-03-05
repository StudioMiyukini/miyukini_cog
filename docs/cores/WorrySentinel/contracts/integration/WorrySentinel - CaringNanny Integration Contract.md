# WorrySentinel - CaringNanny Integration Contract

## 1. Contexte

Ce document dÃ©finit le **contrat d'intÃ©gration entre WorrySentinel et CaringNanny**. Il spÃ©cifie l'interface, le protocole, les rÃ¨gles de communication, et les garanties associÃ©es Ã  l'intÃ©gration entre le gouvernant de la sÃ©curitÃ© (WorrySentinel) et l'observateur d'Ã©tat du systÃ¨me (CaringNanny).

Ce document complÃ¨te la Section 9 "Relation avec CaringNanny" de la [Documentation Fondatrice WorrySentinel](../../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [CaringNanny - Documentation Fondatrice](../../../CaringNanny/foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) pour la nature de CaringNanny
- [Miyukini Conceptual References - Integrity Degradation System](..//..//..//..//miyukini-webway-system//reference//_index.md) pour les Ã©tats de confiance T0-T4
- [Miyukini Conceptual References - Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md) pour les niveaux de sÃ©curitÃ© 0-4
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) pour la conformitÃ© LOI-1 Ã  LOI-6

L'intÃ©gration respecte les Lois d'Autonomie SystÃ¨me : toutes les observations et rÃ¨gles de gouvernance sont locales et ne requiÃ¨rent aucune dÃ©pendance externe (**LOI-1**).

## 2. PortÃ©e / Scope

Ce document couvre :
- L'interface contractuelle entre WorrySentinel et CaringNanny
- Le protocole de communication bidirectionnel asymÃ©trique
- Les types d'informations Ã©changÃ©es (signaux d'observation et rÃ¨gles de gouvernance)
- L'alimentation des Ã©tats de confiance par les signaux d'observation
- Les rÃ¨gles de consolidation des signaux
- Les garanties de l'intÃ©gration

Ce document **ne couvre pas** :
- Les dÃ©tails internes de CaringNanny (voir documentation CaringNanny)
- Les dÃ©tails internes de WorrySentinel (voir Architecture WorrySentinel)
- L'intÃ©gration avec StrongFather (voir StrongFather Integration Contract)
- L'intÃ©gration avec BorderGuard (voir BorderGuard Integration Contract)
- L'intÃ©gration avec LogisticsSteward (voir LogisticsSteward Integration Contract)

---

## 3. Principe fondamental

**CaringNanny consolide les signaux d'intÃ©gritÃ© qui alimentent la gouvernance des Ã©tats de confiance. WorrySentinel gouverne les rÃ¨gles selon lesquelles CaringNanny doit consolider ces signaux. CaringNanny observe et rapporte, WorrySentinel gouverne et dÃ©cide des transitions d'Ã©tat.**

La relation est **bidirectionnelle asymÃ©trique** :
- **Flux montant** : CaringNanny fournit des signaux d'observation Ã  WorrySentinel
- **Flux descendant** : WorrySentinel impose des rÃ¨gles de consolidation Ã  CaringNanny

---

## 4. Nature de la relation WorrySentinel â€” CaringNanny

### 4.1 Relation de collaboration asymÃ©trique

**CaringNanny contribue Ã  WorrySentinel par :**
- La consolidation des signaux d'intÃ©gritÃ© de tous les composants
- La dÃ©tection des anomalies et leur classification
- L'agrÃ©gation des Ã©tats partiels en vision globale
- La proposition de transitions d'Ã©tat basÃ©es sur les observations

**WorrySentinel gouverne CaringNanny par :**
- Les rÃ¨gles de classification des signaux
- Les seuils de transition entre Ã©tats de confiance
- Les prioritÃ©s de consolidation selon les niveaux de sÃ©curitÃ©
- L'activation de modes d'observation renforcÃ©e

**RÃ¨gle WS-CN-01 : Observation sans dÃ©cision**

CaringNanny observe, consolide et rapporte, mais ne dÃ©cide jamais des transitions d'Ã©tat de confiance. La dÃ©cision de transition appartient exclusivement Ã  WorrySentinel.

**RÃ¨gle WS-CN-02 : Gouvernance sans observation**

WorrySentinel gouverne les rÃ¨gles de consolidation, mais n'observe jamais directement les composants. L'observation appartient exclusivement Ã  CaringNanny.

**RÃ¨gle WS-CN-03 : SÃ©paration stricte**

CaringNanny ne modifie jamais un Ã©tat de confiance. WorrySentinel ne collecte jamais de signal directement.

### 4.2 SÃ©paration des responsabilitÃ©s

| ResponsabilitÃ© | WorrySentinel | CaringNanny |
|----------------|---------------|-------------|
| **Observer les composants** | âŒ Jamais | âœ… Exclusif |
| **DÃ©tecter les anomalies** | âŒ Jamais | âœ… Exclusif |
| **Consolider les signaux** | âŒ Jamais | âœ… Exclusif |
| **AgrÃ©ger en vision globale** | âŒ Consomme | âœ… Exclusif |
| **DÃ©finir les rÃ¨gles de classification** | âœ… Exclusif | âŒ Applique |
| **DÃ©finir les seuils de transition** | âœ… Exclusif | âŒ Utilise |
| **DÃ©cider des transitions T0-T4** | âœ… Exclusif | âŒ Propose |
| **Gouverner les Ã©tats de confiance** | âœ… Exclusif | âŒ Jamais |
| **Modifier les Ã©tats de confiance** | âœ… Exclusif | âŒ Jamais |

**RÃ¨gle WS-CN-04 : Aucun chevauchement**

Aucun chevauchement de responsabilitÃ©s n'est autorisÃ©. CaringNanny ne gouverne jamais les Ã©tats de confiance, WorrySentinel n'observe jamais directement les composants.

### 4.3 Cycle de gouvernance des Ã©tats de confiance

Le cycle de gouvernance des Ã©tats de confiance implique les deux cores :

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                   CYCLE DE GOUVERNANCE T0-T4                         â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                      â”‚
â”‚   1. OBSERVATION (CaringNanny)                                       â”‚
â”‚      â”‚                                                               â”‚
â”‚      â”œâ”€â”€ DÃ©tecte les conditions des composants                       â”‚
â”‚      â”œâ”€â”€ Classifie selon les rÃ¨gles de WorrySentinel                â”‚
â”‚      â””â”€â”€ AgrÃ¨ge en signaux consolidÃ©s                                â”‚
â”‚                                                                      â”‚
â”‚   2. RAPPORTAGE (CaringNanny â†’ WorrySentinel)                       â”‚
â”‚      â”‚                                                               â”‚
â”‚      â”œâ”€â”€ Transmet les signaux consolidÃ©s                             â”‚
â”‚      â”œâ”€â”€ Propose des transitions si seuils atteints                  â”‚
â”‚      â””â”€â”€ Fournit le contexte et la justification                     â”‚
â”‚                                                                      â”‚
â”‚   3. GOUVERNANCE (WorrySentinel)                                     â”‚
â”‚      â”‚                                                               â”‚
â”‚      â”œâ”€â”€ Ã‰value les signaux selon les rÃ¨gles                         â”‚
â”‚      â”œâ”€â”€ DÃ©cide de la transition (ou non)                            â”‚
â”‚      â””â”€â”€ DÃ©clare le nouvel Ã©tat de confiance                         â”‚
â”‚                                                                      â”‚
â”‚   4. PROPAGATION (WorrySentinel â†’ tous les cores)                   â”‚
â”‚      â”‚                                                               â”‚
â”‚      â””â”€â”€ Notifie tous les cores du nouvel Ã©tat                       â”‚
â”‚                                                                      â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 5. Ce que WorrySentinel ne fait JAMAIS vis-Ã -vis de CaringNanny

### 5.1 Interdictions absolues

**INV-WS-CN-NEVER-1 : N'observe jamais directement**

WorrySentinel n'observe **jamais** directement les composants du systÃ¨me. L'observation est la responsabilitÃ© exclusive de CaringNanny.

**INV-WS-CN-NEVER-2 : Ne collecte jamais de signaux**

WorrySentinel ne collecte **jamais** de signaux directement des composants. Tous les signaux transitent par CaringNanny.

**INV-WS-CN-NEVER-3 : Ne modifie jamais l'Ã©tat d'un composant**

WorrySentinel ne modifie **jamais** l'Ã©tat d'un composant. La modification d'Ã©tat est hors-scope des deux cores (pas d'action corrective).

**INV-WS-CN-NEVER-4 : N'agrÃ¨ge jamais les Ã©tats partiels**

WorrySentinel n'agrÃ¨ge **jamais** les Ã©tats partiels en vision globale. L'agrÃ©gation est la responsabilitÃ© exclusive de CaringNanny.

**INV-WS-CN-NEVER-5 : N'interfÃ¨re jamais avec l'observation**

WorrySentinel n'interfÃ¨re **jamais** avec le processus d'observation de CaringNanny. Les rÃ¨gles de gouvernance guident, elles n'imposent pas de mÃ©thode d'observation.

---

## 6. Ce que CaringNanny ne fait JAMAIS vis-Ã -vis de WorrySentinel

### 6.1 Interdictions absolues

**INV-CN-WS-NEVER-1 : Ne gouverne jamais les Ã©tats de confiance**

CaringNanny ne gouverne **jamais** les Ã©tats de confiance (T0-T4). La gouvernance est la responsabilitÃ© exclusive de WorrySentinel.

**INV-CN-WS-NEVER-2 : Ne dÃ©cide jamais des transitions**

CaringNanny ne dÃ©cide **jamais** des transitions entre Ã©tats de confiance. Elle propose des transitions basÃ©es sur les seuils, mais c'est WorrySentinel qui dÃ©cide.

**INV-CN-WS-NEVER-3 : Ne modifie jamais un Ã©tat de confiance**

CaringNanny ne modifie **jamais** un Ã©tat de confiance. Seul WorrySentinel peut dÃ©clarer un changement d'Ã©tat.

**INV-CN-WS-NEVER-4 : Ne dÃ©finit jamais les rÃ¨gles de classification**

CaringNanny ne dÃ©finit **jamais** les rÃ¨gles de classification des signaux. Les rÃ¨gles sont dÃ©finies par WorrySentinel (ou l'Ã©cosystÃ¨me), CaringNanny les applique.

**INV-CN-WS-NEVER-5 : N'ignore jamais les rÃ¨gles de WorrySentinel**

CaringNanny n'ignore **jamais** les rÃ¨gles de consolidation imposÃ©es par WorrySentinel. L'application des rÃ¨gles est obligatoire.

---

## 7. Types d'informations Ã©changÃ©es

### 7.1 Flux montant : CaringNanny â†’ WorrySentinel

**INTEGRITY_SIGNAL**
- **Objectif :** Transmettre un signal d'intÃ©gritÃ© consolidÃ©
- **Contenu :** Source, nature du signal, classification, sÃ©vÃ©ritÃ©
- **DÃ©clencheur :** DÃ©tection d'une condition significative

**Structure du signal d'intÃ©gritÃ© :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `signal_id` | Identifiant unique du signal | âœ… Oui |
| `source` | Composant source (core, module, produit) | âœ… Oui |
| `signal_type` | Type de signal (anomaly, degradation, recovery, nominal) | âœ… Oui |
| `classification` | Classification selon rÃ¨gles WorrySentinel | âœ… Oui |
| `severity` | SÃ©vÃ©ritÃ© (info, warning, critical, emergency) | âœ… Oui |
| `context` | Contexte de l'observation | âœ… Oui |
| `timestamp` | Horodatage de l'observation | âœ… Oui |

**TRANSITION_PROPOSAL**
- **Objectif :** Proposer une transition d'Ã©tat de confiance
- **Contenu :** Ã‰tat actuel, Ã©tat proposÃ©, signaux justificatifs
- **DÃ©clencheur :** Seuils de transition atteints selon les rÃ¨gles

**Structure de la proposition de transition :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `proposal_id` | Identifiant unique de la proposition | âœ… Oui |
| `current_state` | Ã‰tat de confiance actuel (T0-T4) | âœ… Oui |
| `proposed_state` | Ã‰tat de confiance proposÃ© (T0-T4) | âœ… Oui |
| `justifying_signals` | Liste des signaux justifiant la proposition | âœ… Oui |
| `threshold_met` | Seuil atteint selon les rÃ¨gles | âœ… Oui |
| `confidence_score` | Score de confiance de la proposition | âœ… Oui |
| `timestamp` | Horodatage de la proposition | âœ… Oui |

**CONSOLIDATED_STATE**
- **Objectif :** Fournir une vision consolidÃ©e de l'Ã©tat global
- **Contenu :** Ã‰tats partiels agrÃ©gÃ©s, tendances, indicateurs
- **DÃ©clencheur :** Demande de WorrySentinel ou pÃ©riodique

**Structure de l'Ã©tat consolidÃ© :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `consolidation_id` | Identifiant unique de la consolidation | âœ… Oui |
| `observation_window` | FenÃªtre d'observation | âœ… Oui |
| `component_states` | Ã‰tats partiels par composant | âœ… Oui |
| `anomaly_count` | Nombre d'anomalies dÃ©tectÃ©es | âœ… Oui |
| `trend` | Tendance gÃ©nÃ©rale (improving, stable, degrading) | âœ… Oui |
| `timestamp` | Horodatage de la consolidation | âœ… Oui |

### 7.2 Flux descendant : WorrySentinel â†’ CaringNanny

**CLASSIFICATION_RULES**
- **Objectif :** DÃ©finir ou mettre Ã  jour les rÃ¨gles de classification
- **Contenu :** RÃ¨gles de classification des signaux par type et sÃ©vÃ©ritÃ©
- **DÃ©clencheur :** Initialisation ou mise Ã  jour des rÃ¨gles

**Structure des rÃ¨gles de classification :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `rules_id` | Identifiant unique du jeu de rÃ¨gles | âœ… Oui |
| `rules_version` | Version des rÃ¨gles | âœ… Oui |
| `signal_classifications` | Mapping signal â†’ classification | âœ… Oui |
| `severity_weights` | PondÃ©rations par sÃ©vÃ©ritÃ© | âœ… Oui |
| `effective_from` | Date d'effet des rÃ¨gles | âœ… Oui |

**TRANSITION_THRESHOLDS**
- **Objectif :** DÃ©finir les seuils de transition T0-T4
- **Contenu :** Seuils pour chaque transition autorisÃ©e
- **DÃ©clencheur :** Initialisation ou mise Ã  jour des seuils

**Structure des seuils de transition :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `thresholds_id` | Identifiant unique des seuils | âœ… Oui |
| `thresholds_version` | Version des seuils | âœ… Oui |
| `transitions` | Liste des transitions avec leurs seuils | âœ… Oui |
| `effective_from` | Date d'effet des seuils | âœ… Oui |

**OBSERVATION_MODE**
- **Objectif :** Activer un mode d'observation spÃ©cifique
- **Contenu :** Mode (normal, enhanced, emergency), durÃ©e
- **DÃ©clencheur :** Changement d'Ã©tat de confiance ou alerte

**Structure du mode d'observation :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `mode_id` | Identifiant unique du mode | âœ… Oui |
| `mode_type` | Type (normal, enhanced, emergency) | âœ… Oui |
| `focus_areas` | Composants Ã  surveiller en prioritÃ© | âŒ Optionnel |
| `sampling_rate` | FrÃ©quence d'observation | âŒ Optionnel |
| `duration` | DurÃ©e du mode (null = jusqu'Ã  nouvel ordre) | âŒ Optionnel |
| `timestamp` | Horodatage de l'activation | âœ… Oui |

**STATE_DECLARATION**
- **Objectif :** Notifier le nouvel Ã©tat de confiance dÃ©cidÃ©
- **Contenu :** Ã‰tat prÃ©cÃ©dent, nouvel Ã©tat, justification
- **DÃ©clencheur :** DÃ©cision de transition par WorrySentinel

**Structure de la dÃ©claration d'Ã©tat :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `declaration_id` | Identifiant unique de la dÃ©claration | âœ… Oui |
| `previous_state` | Ã‰tat de confiance prÃ©cÃ©dent (T0-T4) | âœ… Oui |
| `new_state` | Nouvel Ã©tat de confiance (T0-T4) | âœ… Oui |
| `justification` | Justification de la transition | âœ… Oui |
| `accepted_proposal` | RÃ©fÃ©rence Ã  la proposition acceptÃ©e (si applicable) | âŒ Optionnel |
| `timestamp` | Horodatage de la dÃ©claration | âœ… Oui |

---

## 8. Adaptation de l'observation par Ã©tat de confiance

### 8.1 Mode d'observation par Ã©tat

WorrySentinel active des modes d'observation diffÃ©rents selon l'Ã©tat de confiance :

**T0 â€” Normal**

| Aspect | Comportement |
|--------|--------------|
| **Mode d'observation** | Normal |
| **FrÃ©quence** | Standard |
| **Focus** | Tous les composants Ã©quitablement |
| **Seuils d'alerte** | Standards |
| **Journalisation** | Standard |

**T1 â€” Instable**

| Aspect | Comportement |
|--------|--------------|
| **Mode d'observation** | Enhanced |
| **FrÃ©quence** | AugmentÃ©e (+50%) |
| **Focus** | Composants sources d'anomalies |
| **Seuils d'alerte** | AbaissÃ©s (-20%) |
| **Journalisation** | DÃ©taillÃ©e |

**T2 â€” DÃ©gradÃ©**

| Aspect | Comportement |
|--------|--------------|
| **Mode d'observation** | Enhanced |
| **FrÃ©quence** | Haute (+100%) |
| **Focus** | Composants critiques et frontiÃ¨res |
| **Seuils d'alerte** | AbaissÃ©s (-40%) |
| **Journalisation** | ComplÃ¨te avec contexte |

**T3 â€” Restreint**

| Aspect | Comportement |
|--------|--------------|
| **Mode d'observation** | Emergency |
| **FrÃ©quence** | Maximale |
| **Focus** | Composants vitaux uniquement |
| **Seuils d'alerte** | Minimaux |
| **Journalisation** | Exhaustive |

**T4 â€” BloquÃ©**

| Aspect | Comportement |
|--------|--------------|
| **Mode d'observation** | Emergency |
| **FrÃ©quence** | Minimale (prÃ©servation ressources) |
| **Focus** | Signes de rÃ©cupÃ©ration |
| **Seuils d'alerte** | Uniquement rÃ©cupÃ©ration |
| **Journalisation** | Minimale (prÃ©servation) |

**RÃ¨gle WS-CN-STATE-01 : Adaptation immÃ©diate**

L'adaptation du mode d'observation Ã  un changement d'Ã©tat de confiance est immÃ©diate. Aucun dÃ©lai n'est autorisÃ©.

**RÃ¨gle WS-CN-STATE-02 : PrÃ©servation en T4**

En Ã©tat T4, l'observation est minimale pour prÃ©server les ressources du systÃ¨me en mode survie.

### 8.2 PrioritÃ©s de consolidation par niveau de sÃ©curitÃ©

WorrySentinel dÃ©finit des prioritÃ©s de consolidation selon les niveaux de sÃ©curitÃ© :

| Niveau de sÃ©curitÃ© | PrioritÃ© de consolidation |
|--------------------|---------------------------|
| **Niveau 0 - Public** | Basse â€” consolidation en arriÃ¨re-plan |
| **Niveau 1 - Standard** | Normale â€” consolidation rÃ©guliÃ¨re |
| **Niveau 2 - Sensitive** | Haute â€” consolidation prioritaire |
| **Niveau 3 - Critical** | TrÃ¨s haute â€” consolidation immÃ©diate |
| **Niveau 4 - Highest** | Maximale â€” consolidation en temps rÃ©el |

**RÃ¨gle WS-CN-SEC-01 : Priorisation des signaux**

Les signaux provenant de composants de niveau de sÃ©curitÃ© Ã©levÃ© sont prioritaires dans la consolidation.

**RÃ¨gle WS-CN-SEC-02 : Cumul Ã©tat-niveau**

En cas d'Ã©tat de confiance dÃ©gradÃ© ET de niveau de sÃ©curitÃ© Ã©levÃ©, la prioritÃ© est maximale.

---

## 9. Protocole de communication

### 9.1 Format des signaux montants

Les signaux de CaringNanny suivent un format standardisÃ©.

**Structure de base :**

| Ã‰lÃ©ment | Description | Obligatoire |
|---------|-------------|-------------|
| `signal_id` | Identifiant unique du signal | âœ… Oui |
| `type` | Type de signal (INTEGRITY_SIGNAL, TRANSITION_PROPOSAL, CONSOLIDATED_STATE) | âœ… Oui |
| `priority` | PrioritÃ© (low, normal, high, critical) | âœ… Oui |
| `payload` | DonnÃ©es spÃ©cifiques au signal | âœ… Oui |
| `timestamp` | Horodatage du signal | âœ… Oui |

**RÃ¨gle WS-CN-PROT-01 : Signaux non bloquants**

Les signaux montants sont non bloquants. CaringNanny continue son observation aprÃ¨s l'envoi.

### 9.2 Format des directives descendantes

Les directives de WorrySentinel suivent un format standardisÃ©.

**Structure de base :**

| Ã‰lÃ©ment | Description | Obligatoire |
|---------|-------------|-------------|
| `directive_id` | Identifiant unique de la directive | âœ… Oui |
| `type` | Type de directive (CLASSIFICATION_RULES, TRANSITION_THRESHOLDS, OBSERVATION_MODE, STATE_DECLARATION) | âœ… Oui |
| `payload` | DonnÃ©es spÃ©cifiques Ã  la directive | âœ… Oui |
| `requires_ack` | Si une confirmation est requise | âœ… Oui |
| `timestamp` | Horodatage de la directive | âœ… Oui |

**RÃ¨gle WS-CN-PROT-02 : Traitement immÃ©diat**

Toutes les directives de WorrySentinel sont traitÃ©es immÃ©diatement par CaringNanny.

### 9.3 Acquittements

**RÃ¨gle WS-CN-PROT-03 : Acquittement obligatoire**

CaringNanny acquitte toutes les directives avec `requires_ack: true`.

**Structure de l'acquittement :**

| Ã‰lÃ©ment | Description | Obligatoire |
|---------|-------------|-------------|
| `ack_id` | Identifiant unique de l'acquittement | âœ… Oui |
| `directive_id` | RÃ©fÃ©rence Ã  la directive | âœ… Oui |
| `status` | Statut (ACK_OK, ACK_PARTIAL, ACK_ERROR) | âœ… Oui |
| `adaptation_applied` | Confirmation de l'adaptation | âœ… Oui |
| `timestamp` | Horodatage de l'acquittement | âœ… Oui |

---

## 10. Flux d'intÃ©gration typiques

### 10.1 Flux de proposition de transition

**Acteurs :** CaringNanny, WorrySentinel

**SÃ©quence :**

1. CaringNanny observe des anomalies sur plusieurs composants
2. CaringNanny classifie les anomalies selon les rÃ¨gles de WorrySentinel
3. CaringNanny calcule que le seuil T0â†’T1 est atteint
4. CaringNanny envoie `TRANSITION_PROPOSAL` Ã  WorrySentinel
5. WorrySentinel Ã©value la proposition
6. WorrySentinel dÃ©cide d'accepter ou rejeter la transition
7. Si acceptÃ©e, WorrySentinel envoie `STATE_DECLARATION` (T0â†’T1)
8. WorrySentinel envoie `OBSERVATION_MODE` (enhanced) Ã  CaringNanny
9. CaringNanny acquitte et adapte son mode d'observation

### 10.2 Flux de mise Ã  jour des rÃ¨gles

**Acteurs :** WorrySentinel, CaringNanny

**SÃ©quence :**

1. WorrySentinel dÃ©cide de modifier les seuils de transition
2. WorrySentinel envoie `TRANSITION_THRESHOLDS` Ã  CaringNanny
3. CaringNanny reÃ§oit les nouveaux seuils
4. CaringNanny acquitte avec `ACK_OK`
5. CaringNanny applique les nouveaux seuils pour les futures propositions

### 10.3 Flux de dÃ©tection d'anomalie critique

**Acteurs :** CaringNanny, WorrySentinel

**SÃ©quence :**

1. CaringNanny dÃ©tecte une anomalie critique sur un composant de sÃ©curitÃ© niveau 4
2. CaringNanny envoie immÃ©diatement `INTEGRITY_SIGNAL` (priority: critical)
3. WorrySentinel Ã©value le signal
4. WorrySentinel peut dÃ©cider une transition d'Ã©tat immÃ©diate
5. WorrySentinel notifie CaringNanny et tous les cores de la transition

### 10.4 Diagramme de sÃ©quence

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  CaringNanny    â”‚                    â”‚  WorrySentinel  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜                    â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚                                      â”‚
         â”‚  (Observation d'anomalies)           â”‚
         â”‚                                      â”‚
         â”œâ”€â”€ INTEGRITY_SIGNAL â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚
         â”‚   (anomaly, warning)                 â”‚
         â”‚                                      â”‚
         â”œâ”€â”€ INTEGRITY_SIGNAL â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚
         â”‚   (anomaly, warning)                 â”‚
         â”‚                                      â”‚
         â”‚  (Seuil T0â†’T1 atteint)               â”‚
         â”‚                                      â”‚
         â”œâ”€â”€ TRANSITION_PROPOSAL â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚
         â”‚   (T0 â†’ T1)                          â”‚
         â”‚                                      â”œâ”€â”€ Ã‰value proposition
         â”‚                                      â”‚
         â”‚ â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ STATE_DECLARATION â”€â”€â”€â”€â”€â”¤
         â”‚              (T0 â†’ T1 acceptÃ©)       â”‚
         â”‚                                      â”‚
         â”‚ â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ OBSERVATION_MODE â”€â”€â”€â”€â”€â”€â”¤
         â”‚              (mode: enhanced)        â”‚
         â”‚                                      â”‚
         â”œâ”€â”€ ACK_OK â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–ºâ”‚
         â”‚                                      â”‚
         â”‚  (Observation renforcÃ©e)             â”‚
         â”‚                                      â”‚
```

---

## 11. RÃ¨gles d'intÃ©gration

### 11.1 RÃ¨gles de communication

**RÃ¨gle WS-CN-INT-01 : Bidirectionnel asymÃ©trique**

La communication est bidirectionnelle mais asymÃ©trique. CaringNanny rapporte et propose, WorrySentinel gouverne et dÃ©cide.

**RÃ¨gle WS-CN-INT-02 : PrioritÃ© aux directives**

Les directives de WorrySentinel sont prioritaires. CaringNanny adapte immÃ©diatement son comportement.

**RÃ¨gle WS-CN-INT-03 : Signaux continus**

CaringNanny fournit des signaux de maniÃ¨re continue, pas uniquement lors d'anomalies.

### 11.2 RÃ¨gles de donnÃ©es

**RÃ¨gle WS-CN-INT-04 : Signaux factuels**

Les signaux de CaringNanny sont factuels (observations, classifications). Aucune dÃ©cision n'est incluse.

**RÃ¨gle WS-CN-INT-05 : RÃ¨gles explicites**

Les rÃ¨gles de WorrySentinel sont explicites et dÃ©claratives. Aucune rÃ¨gle implicite.

**RÃ¨gle WS-CN-INT-06 : CohÃ©rence garantie**

WorrySentinel garantit la cohÃ©rence des rÃ¨gles et seuils communiquÃ©s.

### 11.3 RÃ¨gles de traÃ§abilitÃ©

**RÃ¨gle WS-CN-INT-07 : TraÃ§abilitÃ© complÃ¨te**

Toutes les interactions sont tracÃ©es avec contexte complet.

**RÃ¨gle WS-CN-INT-08 : CorrÃ©lation possible**

Chaque transition d'Ã©tat peut Ãªtre corrÃ©lÃ©e aux signaux et propositions qui l'ont provoquÃ©e.

---

## 12. Gestion des erreurs

### 12.1 Types d'erreurs

**Erreurs de format :**
- Signal mal formÃ©
- Directive mal formÃ©e
- Type de message inconnu

**Erreurs de classification :**
- Signal inclassifiable selon les rÃ¨gles
- RÃ¨gles incohÃ©rentes reÃ§ues

**Erreurs internes :**
- Erreur de CaringNanny lors de la consolidation
- Erreur de WorrySentinel lors de l'Ã©valuation

### 12.2 Traitement des erreurs

**RÃ¨gle WS-CN-ERR-01 : Acquittement avec erreur**

En cas d'erreur, CaringNanny acquitte avec `ACK_ERROR` et description du problÃ¨me.

**RÃ¨gle WS-CN-ERR-02 : Signal non classifiable**

Si un signal ne peut pas Ãªtre classifiÃ©, CaringNanny le rapporte avec classification `unknown` et WorrySentinel dÃ©cide de son traitement.

**RÃ¨gle WS-CN-ERR-03 : Journalisation des erreurs**

Toutes les erreurs sont journalisÃ©es pour audit et diagnostic.

**RÃ¨gle WS-CN-ERR-04 : Observation par dÃ©faut**

En cas de perte de communication avec WorrySentinel, CaringNanny continue avec les derniÃ¨res rÃ¨gles connues et le mode d'observation le plus restrictif applicable.

---

## 13. Cas particuliers

### 13.1 Proposition de transition rejetÃ©e

Lorsque WorrySentinel rejette une proposition de transition :

**RÃ¨gle WS-CN-CASE-01 : Rejet notifiÃ©**

WorrySentinel notifie CaringNanny du rejet avec justification. CaringNanny continue l'observation avec les paramÃ¨tres actuels.

### 13.2 Transitions multiples rapides

Lorsque plusieurs seuils sont atteints rapidement :

**RÃ¨gle WS-CN-CASE-02 : Proposition par transition**

CaringNanny propose les transitions une par une (T0â†’T1, puis T1â†’T2). WorrySentinel peut accepter plusieurs transitions consÃ©cutives.

### 13.3 Ã‰tat T4 (BloquÃ©)

En Ã©tat T4 :

**RÃ¨gle WS-CN-CASE-03 : Observation minimale**

CaringNanny rÃ©duit son observation au minimum pour prÃ©server les ressources. Seuls les signaux de rÃ©cupÃ©ration sont recherchÃ©s.

### 13.4 RÃ©cupÃ©ration (T2â†’T1, T1â†’T0)

Lors d'une rÃ©cupÃ©ration :

**RÃ¨gle WS-CN-CASE-04 : Proposition de rÃ©cupÃ©ration**

CaringNanny peut proposer des transitions de rÃ©cupÃ©ration lorsque les conditions s'amÃ©liorent et que les seuils le permettent.

---

## 14. Garanties de l'intÃ©gration

### 14.1 Garantie de sÃ©paration

**Engagement :** CaringNanny observe exclusivement, WorrySentinel gouverne exclusivement. Aucun chevauchement de responsabilitÃ©s.

### 14.2 Garantie de rÃ©activitÃ©

**Engagement :** CaringNanny rÃ©agit immÃ©diatement aux directives de WorrySentinel. Aucun dÃ©lai supÃ©rieur Ã  une seconde.

### 14.3 Garantie de conformitÃ©

**Engagement :** CaringNanny applique toujours les rÃ¨gles de WorrySentinel. Aucune classification ou proposition ne contredit les rÃ¨gles.

### 14.4 Garantie de traÃ§abilitÃ©

**Engagement :** Toute interaction est traÃ§able de bout en bout. L'audit complet du cycle de gouvernance est possible.

### 14.5 Garantie de continuitÃ©

**Engagement :** En cas de dÃ©faillance partielle, les deux cores continuent de fonctionner avec les derniÃ¨res rÃ¨gles/observations connues.

### 14.6 Garantie de disponibilitÃ©

**Engagement :** L'intÃ©gration ne bloque jamais CaringNanny. En cas de dÃ©faillance de WorrySentinel, CaringNanny continue avec le mode d'observation le plus restrictif.

---

## 15. Invariants de l'intÃ©gration

### 15.1 Invariants de relation

**INV-WS-CN-1 : Observation exclusive**

L'observation appartient exclusivement Ã  CaringNanny. WorrySentinel n'observe jamais directement.

**INV-WS-CN-2 : Gouvernance exclusive**

La gouvernance des Ã©tats de confiance appartient exclusivement Ã  WorrySentinel. CaringNanny ne gouverne jamais.

**INV-WS-CN-3 : Proposition vs dÃ©cision**

CaringNanny propose des transitions. WorrySentinel dÃ©cide des transitions. La distinction est fondamentale.

### 15.2 Invariants de donnÃ©es

**INV-WS-CN-4 : Signaux factuels**

Les signaux de CaringNanny sont factuels (observations classifiÃ©es). Aucune dÃ©cision n'est incluse.

**INV-WS-CN-5 : RÃ¨gles explicites**

Les rÃ¨gles de WorrySentinel sont explicites et dÃ©claratives.

### 15.3 Invariants de protocole

**INV-WS-CN-6 : Format respectÃ©**

Toutes les communications respectent le format standardisÃ©.

**INV-WS-CN-7 : TraÃ§abilitÃ© complÃ¨te**

Toute interaction est traÃ§able avec son contexte complet.

---

## 16. ConformitÃ© aux Lois d'Autonomie SystÃ¨me

### LOI-1 : Aucune dÃ©pendance externe critique

**ConformitÃ© :** âœ… **Conforme**

L'intÃ©gration respecte LOI-1 :
- CaringNanny observe localement
- WorrySentinel gouverne localement
- Les rÃ¨gles et signaux sont stockÃ©s localement
- L'absence de connexion ne bloque ni l'observation ni la gouvernance

### LOI-2 : Le systÃ¨me accepte l'isolement comme Ã©tat normal

**ConformitÃ© :** âœ… **Conforme**

L'intÃ©gration respecte LOI-2 :
- En isolement, CaringNanny continue avec les derniÃ¨res rÃ¨gles connues
- Les propositions de transition restent possibles localement
- L'intÃ©gration fonctionne sans dÃ©gradation en mode isolÃ©

### LOI-4 : Pas de temps global requis

**ConformitÃ© :** âœ… **Conforme**

L'intÃ©gration respecte LOI-4 :
- Les horodatages sont locaux
- Aucune synchronisation temporelle n'est requise
- Les Ã©tats de confiance ne dÃ©pendent pas de timestamps synchronisÃ©s

---

## 17. Exemples

### 17.1 Signal d'intÃ©gritÃ©

**Signal CaringNanny â†’ WorrySentinel :**
```
{
  "signal_id": "sig-cn-ws-001",
  "type": "INTEGRITY_SIGNAL",
  "priority": "high",
  "payload": {
    "source": "BorderGuard",
    "signal_type": "anomaly",
    "classification": "boundary_degraded",
    "severity": "warning",
    "context": {
      "boundary_id": "boundary-external-001",
      "previous_state": "healthy",
      "current_state": "degraded",
      "cause": "latency_exceeded"
    }
  },
  "timestamp": "2026-01-28T14:00:00Z"
}
```

### 17.2 Proposition de transition

**Proposition CaringNanny â†’ WorrySentinel :**
```
{
  "signal_id": "sig-cn-ws-002",
  "type": "TRANSITION_PROPOSAL",
  "priority": "high",
  "payload": {
    "proposal_id": "prop-001",
    "current_state": "T0",
    "proposed_state": "T1",
    "justifying_signals": ["sig-cn-ws-001", "sig-cn-ws-000"],
    "threshold_met": "anomaly_count >= 3",
    "confidence_score": 0.85
  },
  "timestamp": "2026-01-28T14:05:00Z"
}
```

### 17.3 DÃ©claration d'Ã©tat

**DÃ©claration WorrySentinel â†’ CaringNanny :**
```
{
  "directive_id": "dir-ws-cn-001",
  "type": "STATE_DECLARATION",
  "payload": {
    "declaration_id": "decl-001",
    "previous_state": "T0",
    "new_state": "T1",
    "justification": "Anomalies persistantes dÃ©tectÃ©es sur frontiÃ¨res et composants",
    "accepted_proposal": "prop-001"
  },
  "requires_ack": true,
  "timestamp": "2026-01-28T14:06:00Z"
}
```

### 17.4 Mode d'observation

**Directive WorrySentinel â†’ CaringNanny :**
```
{
  "directive_id": "dir-ws-cn-002",
  "type": "OBSERVATION_MODE",
  "payload": {
    "mode_id": "mode-001",
    "mode_type": "enhanced",
    "focus_areas": ["BorderGuard", "KindMother"],
    "sampling_rate": "high",
    "duration": null
  },
  "requires_ack": true,
  "timestamp": "2026-01-28T14:06:01Z"
}
```

### 17.5 Acquittement

**Acquittement CaringNanny :**
```
{
  "ack_id": "ack-cn-001",
  "directive_id": "dir-ws-cn-002",
  "status": "ACK_OK",
  "adaptation_applied": {
    "mode": "enhanced",
    "focus_areas_activated": ["BorderGuard", "KindMother"],
    "sampling_rate_applied": "high"
  },
  "timestamp": "2026-01-28T14:06:02Z"
}
```

---

## 18. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il Ã©tablit l'interface et le protocole que WorrySentinel et CaringNanny doivent respecter pour leur intÃ©gration.

Toute implÃ©mentation de l'intÃ©gration doit respecter ce contrat. Toute violation entraÃ®ne un comportement non conforme.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif  
**DÃ©pendances :**
- WorrySentinel - Documentation Fondatrice v1.2 (Section 9)
- CaringNanny - Documentation Fondatrice v1.6
- Miyukini Conceptual References - Integrity Degradation System v1.0 (T0-T4)
- Miyukini Conceptual References - Security Levels v1.0 (0-4)
- Miyukini Conceptual References - Lois Autonomie Systeme v1.1

---

## 19. Mini log de gÃ©nÃ©ration

### DÃ©cision Ã©ditoriale E1 : Nature de la relation

**DÃ©cision prise :** La relation est bidirectionnelle asymÃ©trique : CaringNanny observe et propose, WorrySentinel gouverne et dÃ©cide. Cette direction respecte la Documentation Fondatrice de WorrySentinel Section 9 qui dÃ©finit "CaringNanny consolide les signaux d'intÃ©gritÃ© qui influencent les Ã©tats de confiance" et "WorrySentinel gouverne les rÃ¨gles selon lesquelles CaringNanny doit consolider les signaux".

**Application :** Tout le document est structurÃ© autour de cette relation de collaboration asymÃ©trique.

### DÃ©cision Ã©ditoriale E2 : Cycle de gouvernance

**DÃ©cision prise :** Le cycle de gouvernance T0-T4 implique les deux cores de maniÃ¨re complÃ©mentaire : observation (CN) â†’ rapportage (CNâ†’WS) â†’ gouvernance (WS) â†’ propagation (WSâ†’tous).

**Application :** Section 4.3 dÃ©taille ce cycle, Section 10 illustre les flux typiques.

### DÃ©cision Ã©ditoriale E3 : Proposition vs dÃ©cision

**DÃ©cision prise :** La distinction entre proposition (CaringNanny) et dÃ©cision (WorrySentinel) est fondamentale. CaringNanny peut calculer que les seuils sont atteints et proposer une transition, mais seul WorrySentinel dÃ©cide de l'accepter.

**Application :** INV-WS-CN-3 et RÃ¨gles WS-CN-01/02 Ã©tablissent cette distinction.

### Warning W1 : Risque de gouvernance implicite par CaringNanny

**Warning rencontrÃ© :** Risque que CaringNanny, en proposant des transitions basÃ©es sur des seuils, exerce une forme de gouvernance implicite.

**DÃ©cision prise :** Les interdictions absolues (Section 6) clarifient que CaringNanny ne dÃ©cide jamais des transitions. Les seuils sont dÃ©finis par WorrySentinel, CaringNanny les applique pour proposer, mais la dÃ©cision reste Ã  WorrySentinel qui peut rejeter la proposition.

**Correction effectuÃ©e :** INV-CN-WS-NEVER-2 et RÃ¨gle WS-CN-CASE-01 explicitement Ã©tablissent cette limite.

### VÃ©rification de cohÃ©rence

**VÃ©rification effectuÃ©e :**
- âœ… CohÃ©rence avec WorrySentinel - Documentation Fondatrice : ConfirmÃ©e (Section 9 respectÃ©e)
- âœ… CohÃ©rence avec CaringNanny - Documentation Fondatrice : ConfirmÃ©e (INV-CN-1 Ã  INV-CN-7 respectÃ©s)
- âœ… CohÃ©rence avec Integrity Degradation System : ConfirmÃ©e (Ã©tats T0-T4)
- âœ… CohÃ©rence avec Security Levels : ConfirmÃ©e (niveaux 0-4)
- âœ… ConformitÃ© LOI-1 : ConfirmÃ©e (aucune dÃ©pendance externe)
- âœ… ConformitÃ© LOI-2 : ConfirmÃ©e (fonctionnement en mode isolÃ©)
- âœ… ConformitÃ© LOI-4 : ConfirmÃ©e (pas de temps global requis)
- âœ… SÃ©paration observation/gouvernance : ConfirmÃ©e (INV-WS-CN-1, INV-WS-CN-2)
- âœ… TraÃ§abilitÃ© complÃ¨te : ConfirmÃ©e (INV-WS-CN-7)

**Conclusion :** Aucune contradiction dÃ©tectÃ©e. Le document est cohÃ©rent et non ambigu.

---

*Aucune autre erreur, warning, ou ambiguÃ¯tÃ© rencontrÃ©e lors de la rÃ©daction de ce document.*

