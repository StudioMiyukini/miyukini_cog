# WorrySentinel - LogisticsSteward Integration Contract

## 1. Contexte

Ce document dÃ©finit le **contrat d'intÃ©gration entre WorrySentinel et LogisticsSteward**. Il spÃ©cifie l'interface, le protocole, les rÃ¨gles de communication, et les garanties associÃ©es Ã  l'intÃ©gration avec LogisticsSteward en tant que core responsable de la gouvernance de l'allocation, de la priorisation et de la limitation des ressources.

Ce document complÃ¨te la Section 9 "Relation avec LogisticsSteward" de la [Documentation Fondatrice](../../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [LogisticsSteward - Documentation Fondatrice](../../../LogisticsSteward/foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md) pour la nature de LogisticsSteward
- [LogisticsSteward - WorrySentinel Integration Contract](../../../LogisticsSteward/contracts/integration/LogisticsSteward%20-%20WorrySentinel%20Integration%20Contract.md) pour le contrat symÃ©trique
- [Miyukini Conceptual References - Lois Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md) pour la conformitÃ© LOI-1 Ã  LOI-6

L'intÃ©gration respecte les Lois d'Autonomie SystÃ¨me : toutes les contraintes de sÃ©curitÃ© sont locales et ne requiÃ¨rent aucune dÃ©pendance externe (**LOI-1**).

## 2. PortÃ©e / Scope

Ce document couvre :
- L'interface contractuelle entre WorrySentinel et LogisticsSteward
- Le protocole de communication (contraintes descendantes et observations montantes)
- Les types d'informations Ã©changÃ©es
- La supervision des dÃ©rives d'allocation
- Le dÃ©clenchement de durcissement des rÃ¨gles d'arbitrage
- Les rÃ¨gles d'intÃ©gration spÃ©cifiques
- Les garanties de l'intÃ©gration

Ce document **ne couvre pas** :
- Les dÃ©tails internes de LogisticsSteward (voir documentation LogisticsSteward)
- Les dÃ©tails internes du moteur de gouvernance de WorrySentinel
- L'intÃ©gration avec StrongFather (voir StrongFather Integration Contract)
- L'intÃ©gration avec CaringNanny (voir CaringNanny Integration Contract)
- L'intÃ©gration avec BorderGuard (voir BorderGuard Integration Contract)
- L'intÃ©gration avec TAMR (voir TAMR Integration Contract)
- L'intÃ©gration avec MiyukiniAdmin (voir MiyukiniAdmin Integration Contract)

---

## 3. Principe fondamental

**WorrySentinel gouverne les niveaux de sÃ©curitÃ© et les Ã©tats de confiance. LogisticsSteward adapte ses rÃ¨gles d'arbitrage en consÃ©quence. WorrySentinel supervise les dÃ©rives d'allocation sans jamais dÃ©cider des allocations. LogisticsSteward ne peut jamais dÃ©finir des niveaux de sÃ©curitÃ© ni des Ã©tats de confiance.**

La relation est de **supervision verticale** : WorrySentinel observe les comportements d'arbitrage, impose des contraintes de sÃ©curitÃ©, et peut dÃ©clencher des durcissements. LogisticsSteward reste souverain sur l'arbitrage des ressources mais doit adapter ses dÃ©cisions selon les contraintes sÃ©curitaires.

---

## 4. Nature de la relation WorrySentinel â€” LogisticsSteward

### 4.1 Relation de supervision verticale

**WorrySentinel supervise LogisticsSteward par :**
- L'observation des signaux d'allocation et des dÃ©rives potentielles
- L'imposition de contraintes sÃ©curitaires selon l'Ã©tat de confiance (T0-T4)
- Le dÃ©clenchement de durcissement des rÃ¨gles d'arbitrage
- L'invalidation d'Ã©tat systÃ¨me jugÃ© incohÃ©rent

**LogisticsSteward informe WorrySentinel par :**
- Les signaux d'allocation et de consommation
- Les alertes de dÃ©rive de ressources
- Les comportements suspects d'arbitrage
- Les anomalies de gouvernance dÃ©tectÃ©es

**RÃ¨gle WS-LS-01 : Supervision sans substitution**

WorrySentinel supervise LogisticsSteward sans se substituer Ã  lui. LogisticsSteward reste souverain sur l'arbitrage des ressources. WorrySentinel ne dÃ©cide jamais de l'allocation, de la prioritÃ©, ou de la limitation des ressources.

**RÃ¨gle WS-LS-02 : Contrainte verticale obligatoire**

LogisticsSteward doit adapter ses rÃ¨gles d'arbitrage selon les Ã©tats de confiance et les niveaux de sÃ©curitÃ© gouvernÃ©s par WorrySentinel. L'adaptation n'est pas facultative.

**RÃ¨gle WS-LS-03 : Observation continue**

WorrySentinel observe en continu les comportements d'arbitrage de LogisticsSteward pour dÃ©tecter les dÃ©rives sÃ©curitaires. L'observation est passive et non intrusive.

**RÃ¨gle WS-LS-04 : Durcissement proportionnel**

Le durcissement des rÃ¨gles d'arbitrage est proportionnel Ã  l'Ã©tat de confiance. Un Ã©tat T1 implique une vigilance accrue, un Ã©tat T3 implique des restrictions sÃ©vÃ¨res.

### 4.2 SÃ©paration des responsabilitÃ©s

| ResponsabilitÃ© | WorrySentinel | LogisticsSteward |
|----------------|---------------|------------------|
| **Gouverner les Ã©tats de confiance (T0-T4)** | âœ… Exclusif | âŒ Consomme |
| **DÃ©finir les niveaux de sÃ©curitÃ© (0-4)** | âœ… Exclusif | âŒ Consomme |
| **Arbitrer l'allocation des ressources** | âŒ Jamais | âœ… Exclusif |
| **DÃ©finir les quotas et prioritÃ©s** | âŒ Jamais | âœ… Exclusif |
| **DÃ©clencher le durcissement** | âœ… Exclusif | âŒ Subit |
| **Invalider un Ã©tat systÃ¨me** | âœ… Peut dÃ©cider | âŒ RÃ©agit |
| **DÃ©tecter les dÃ©rives de sÃ©curitÃ©** | âœ… Consomme | âœ… Source |
| **Observer les comportements d'arbitrage** | âœ… Exclusif | âŒ Source |

**RÃ¨gle WS-LS-05 : Aucun chevauchement**

Aucun chevauchement de responsabilitÃ©s n'est autorisÃ©. WorrySentinel ne dÃ©cide jamais des allocations, LogisticsSteward n'Ã©value jamais les menaces de sÃ©curitÃ©.

---

## 5. Ce que WorrySentinel ne fait JAMAIS vis-Ã -vis de LogisticsSteward

### 5.1 Interdictions absolues

**INV-WS-LS-NEVER-1 : Ne dÃ©cide jamais de l'allocation**

WorrySentinel ne dÃ©cide **jamais** de l'allocation des ressources. Il peut imposer des contraintes de sÃ©curitÃ©, mais la dÃ©cision d'allocation appartient exclusivement Ã  LogisticsSteward.

**INV-WS-LS-NEVER-2 : Ne dÃ©finit jamais les quotas**

WorrySentinel ne dÃ©finit **jamais** les quotas ou les prioritÃ©s. Il peut exiger des restrictions, mais c'est LogisticsSteward qui traduit ces exigences en rÃ¨gles d'arbitrage.

**INV-WS-LS-NEVER-3 : N'exÃ©cute jamais d'arbitrage**

WorrySentinel n'exÃ©cute **jamais** d'arbitrage de ressources. Il gouverne et contraint, mais ne participe pas Ã  l'arbitrage.

**INV-WS-LS-NEVER-4 : Ne contourne jamais LogisticsSteward**

WorrySentinel ne contourne **jamais** LogisticsSteward pour imposer directement des allocations ou des restrictions de ressources aux entitÃ©s.

**INV-WS-LS-NEVER-5 : Ne modifie jamais les rÃ¨gles d'arbitrage**

WorrySentinel ne modifie **jamais** directement les rÃ¨gles d'arbitrage de LogisticsSteward. Il impose des contraintes que LogisticsSteward traduit en rÃ¨gles.

**INV-WS-LS-NEVER-6 : Ne bloque jamais les signaux montants**

WorrySentinel ne bloque **jamais** les signaux montants de LogisticsSteward. Toute information de dÃ©rive doit pouvoir remonter.

---

## 6. Supervision des dÃ©rives d'allocation

### 6.1 Objectif de la supervision

WorrySentinel supervise LogisticsSteward pour dÃ©tecter les dÃ©rives potentielles dans l'allocation des ressources qui pourraient compromettre la sÃ©curitÃ© du systÃ¨me.

**Types de dÃ©rives surveillÃ©es :**

| Type de dÃ©rive | Description | Impact sÃ©curitaire |
|----------------|-------------|-------------------|
| **Monopolisation** | Une entitÃ© accapare une part disproportionnÃ©e | Risque de dÃ©ni de service |
| **Escalade progressive** | Augmentation graduelle de consommation | Ã‰puisement silencieux |
| **Pattern anormal** | Comportement atypique d'allocation | Indicateur d'intrusion |
| **Contournement** | Tentatives de bypass des quotas | Violation de gouvernance |
| **Saturation ciblÃ©e** | Ã‰puisement dÃ©libÃ©rÃ© de ressources | Attaque par ressources |

### 6.2 RÃ¨gles de dÃ©tection

**RÃ¨gle WS-LS-DET-01 : Observation des tendances**

WorrySentinel observe les tendances de consommation signalÃ©es par LogisticsSteward. Une tendance croissante persistante peut dÃ©clencher une alerte.

**RÃ¨gle WS-LS-DET-02 : CorrÃ©lation multi-signaux**

WorrySentinel corrÃ¨le les signaux de LogisticsSteward avec les autres sources (BorderGuard, StrongFather, CaringNanny) pour identifier les patterns de menace.

**RÃ¨gle WS-LS-DET-03 : Seuils d'alerte**

| Seuil | Niveau | Action |
|-------|--------|--------|
| **Usage > 70%** | Info | Surveillance accrue |
| **Usage > 85%** | Warning | PrÃ©paration durcissement |
| **Usage > 95%** | Critique | Durcissement immÃ©diat possible |
| **DÃ©passement quota** | Alerte | Ã‰valuation de la menace |

**RÃ¨gle WS-LS-DET-04 : Contexte de sÃ©curitÃ©**

La dÃ©tection tient compte du niveau de sÃ©curitÃ© de l'entitÃ© concernÃ©e. Une dÃ©rive sur une entitÃ© de niveau 4 est plus critique qu'une dÃ©rive sur une entitÃ© de niveau 0.

### 6.3 CorrÃ©lation avec l'Ã©tat de confiance

| Ã‰tat de confiance | SensibilitÃ© de dÃ©tection | Seuils |
|-------------------|--------------------------|--------|
| **T0 â€” Normal** | Standard | Seuils normaux |
| **T1 â€” Instable** | Ã‰levÃ©e | Seuils abaissÃ©s de 10% |
| **T2 â€” DÃ©gradÃ©** | TrÃ¨s Ã©levÃ©e | Seuils abaissÃ©s de 20% |
| **T3 â€” Restreint** | Maximale | Seuils abaissÃ©s de 30% |
| **T4 â€” BloquÃ©** | Critique | Toute dÃ©rive est bloquante |

---

## 7. Durcissement des rÃ¨gles d'arbitrage

### 7.1 Principes de durcissement

WorrySentinel peut dÃ©clencher un durcissement des rÃ¨gles d'arbitrage de LogisticsSteward selon l'Ã©tat de confiance ou en rÃ©ponse Ã  une menace dÃ©tectÃ©e.

**Principe WS-LS-HARD-01 : Durcissement progressif**

Le durcissement est progressif et proportionnel Ã  la menace. Pas de durcissement brutal sans justification.

**Principe WS-LS-HARD-02 : Durcissement rÃ©versible**

Tout durcissement est rÃ©versible par une directive explicite de levÃ©e. Le retour Ã  la normale est possible.

**Principe WS-LS-HARD-03 : Durcissement ciblÃ©**

Le durcissement peut Ãªtre ciblÃ© sur des entitÃ©s spÃ©cifiques ou global. Le ciblage prÃ©cis minimise l'impact.

### 7.2 Types de durcissement

| Type | Description | DÃ©clencheur |
|------|-------------|-------------|
| **QUOTA_REDUCTION** | RÃ©duction des quotas autorisÃ©s | DÃ©rive de consommation |
| **PRIORITY_FREEZE** | Gel des prioritÃ©s au niveau actuel | Tentatives d'escalade |
| **ALLOCATION_BLOCK** | Blocage des nouvelles allocations | Menace confirmÃ©e |
| **PREEMPTION_ENABLE** | Activation de la prÃ©emption | Urgence ressources |
| **DEGRADATION_FORCE** | ForÃ§age d'un niveau de dÃ©gradation | Ã‰tat T2+ |

### 7.3 Directives de durcissement par Ã©tat de confiance

**T0 â€” Normal**

| Aspect | Directive |
|--------|-----------|
| **Quotas** | Aucune modification |
| **PrioritÃ©s** | Aucune modification |
| **Allocations** | Normales |
| **Durcissement** | Aucun |

**T1 â€” Instable**

| Aspect | Directive |
|--------|-----------|
| **Quotas** | Surveillance renforcÃ©e, alertes actives |
| **PrioritÃ©s** | Aucune modification |
| **Allocations** | Normales avec traÃ§abilitÃ© Ã©tendue |
| **Durcissement** | PrÃ©paration possible |

**T2 â€” DÃ©gradÃ©**

| Aspect | Directive |
|--------|-----------|
| **Quotas** | RÃ©duction de 20% pour entitÃ©s non essentielles |
| **PrioritÃ©s** | PrioritÃ© maximale rÃ©servÃ©e aux services critiques |
| **Allocations** | Nouvelles allocations sous conditions |
| **Durcissement** | Actif, niveau modÃ©rÃ© |

**T3 â€” Restreint**

| Aspect | Directive |
|--------|-----------|
| **Quotas** | Quotas minimaux, gel des nouvelles allocations |
| **PrioritÃ©s** | Seules prioritÃ©s critiques honorÃ©es |
| **Allocations** | BloquÃ©es sauf services vitaux |
| **Durcissement** | Actif, niveau sÃ©vÃ¨re |

**T4 â€” BloquÃ©**

| Aspect | Directive |
|--------|-----------|
| **Quotas** | Aucune allocation |
| **PrioritÃ©s** | PrÃ©servation du cÅ“ur systÃ¨me uniquement |
| **Allocations** | Totalement bloquÃ©es |
| **Durcissement** | Maximum, mode survie |

### 7.4 RÃ¨gles de durcissement (RÃˆGLE-WS-LS-1 Ã  RÃˆGLE-WS-LS-4)

Ces rÃ¨gles sont dÃ©finies dans la [Documentation Fondatrice](../../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md) Section 9 :

**RÃˆGLE-WS-LS-1 : Contraintes sÃ©curitaires**

WorrySentinel peut imposer des contraintes sÃ©curitaires sur les dÃ©cisions d'arbitrage de LogisticsSteward. Ces contraintes sont obligatoires.

**RÃˆGLE-WS-LS-2 : Quotas restrictifs en Ã©tat T2+**

En Ã©tat T2+, LogisticsSteward doit appliquer des quotas plus restrictifs selon les directives de WorrySentinel.

**RÃˆGLE-WS-LS-3 : Observation des patterns**

WorrySentinel observe les patterns d'allocation de ressources pour dÃ©tecter des anomalies sÃ©curitaires.

**RÃˆGLE-WS-LS-4 : Traitement des dÃ©rives**

Toute dÃ©rive d'allocation signalÃ©e par WorrySentinel doit Ãªtre traitÃ©e par LogisticsSteward.

---

## 8. Types d'informations Ã©changÃ©es

### 8.1 Flux descendant : WorrySentinel â†’ LogisticsSteward

**TRUST_STATE_CHANGE**
- **Objectif :** Notifier un changement d'Ã©tat de confiance
- **Contenu :** Nouvel Ã©tat (T0-T4), justification, timestamp
- **Impact :** LogisticsSteward adapte ses rÃ¨gles d'arbitrage

**Structure du changement d'Ã©tat :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `notification_id` | Identifiant unique de la notification | âœ… Oui |
| `previous_state` | Ã‰tat de confiance prÃ©cÃ©dent (T0-T4) | âœ… Oui |
| `new_state` | Nouvel Ã©tat de confiance (T0-T4) | âœ… Oui |
| `transition_reason` | Justification de la transition | âœ… Oui |
| `timestamp` | Horodatage de la transition | âœ… Oui |
| `constraints` | Contraintes supplÃ©mentaires applicables | âŒ Optionnel |

**SECURITY_LEVEL_ASSIGNMENT**
- **Objectif :** Attribuer ou modifier le niveau de sÃ©curitÃ© d'une entitÃ©
- **Contenu :** EntitÃ© concernÃ©e, niveau (0-4), justification
- **Impact :** LogisticsSteward adapte l'arbitrage pour cette entitÃ©

**Structure de l'attribution de niveau :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `assignment_id` | Identifiant unique de l'attribution | âœ… Oui |
| `entity_id` | Identifiant de l'entitÃ© concernÃ©e | âœ… Oui |
| `entity_type` | Type d'entitÃ© (operator, team, service, tool) | âœ… Oui |
| `security_level` | Niveau de sÃ©curitÃ© (0-4) | âœ… Oui |
| `justification` | Raison de l'attribution | âœ… Oui |
| `timestamp` | Horodatage de l'attribution | âœ… Oui |

**HARDENING_DIRECTIVE**
- **Objectif :** DÃ©clencher un durcissement immÃ©diat des rÃ¨gles
- **Contenu :** Type de durcissement, entitÃ©s concernÃ©es, durÃ©e
- **Impact :** Restrictions supplÃ©mentaires appliquÃ©es immÃ©diatement

**Structure de la directive de durcissement :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `directive_id` | Identifiant unique de la directive | âœ… Oui |
| `hardening_type` | Type de durcissement (quota_reduction, priority_freeze, allocation_block) | âœ… Oui |
| `affected_entities` | Liste des entitÃ©s concernÃ©es (vide = toutes) | âŒ Optionnel |
| `severity` | SÃ©vÃ©ritÃ© du durcissement (low, medium, high, critical) | âœ… Oui |
| `duration` | DurÃ©e du durcissement (null = indÃ©fini) | âŒ Optionnel |
| `justification` | Raison du durcissement | âœ… Oui |
| `timestamp` | Horodatage de la directive | âœ… Oui |

**HARDENING_LIFT**
- **Objectif :** Lever un durcissement prÃ©cÃ©demment imposÃ©
- **Contenu :** RÃ©fÃ©rence Ã  la directive originale, justification
- **Impact :** Retour aux rÃ¨gles d'arbitrage normales

**Structure de la levÃ©e de durcissement :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `lift_id` | Identifiant unique de la levÃ©e | âœ… Oui |
| `directive_id` | RÃ©fÃ©rence Ã  la directive originale | âœ… Oui |
| `justification` | Raison de la levÃ©e | âœ… Oui |
| `timestamp` | Horodatage de la levÃ©e | âœ… Oui |

**STATE_INVALIDATION**
- **Objectif :** Invalider l'Ã©tat systÃ¨me actuel
- **Contenu :** Raison de l'invalidation, action requise
- **Impact :** LogisticsSteward doit suspendre les nouveaux arbitrages

**Structure de l'invalidation :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `invalidation_id` | Identifiant unique de l'invalidation | âœ… Oui |
| `reason` | Raison de l'invalidation | âœ… Oui |
| `required_action` | Action requise de LogisticsSteward | âœ… Oui |
| `timestamp` | Horodatage de l'invalidation | âœ… Oui |

### 8.2 Flux montant : LogisticsSteward â†’ WorrySentinel

**ANOMALY_REPORT**
- **Objectif :** Signaler un comportement d'arbitrage suspect
- **Contenu :** Nature de l'anomalie, entitÃ© concernÃ©e, contexte
- **Usage :** WorrySentinel Ã©value si une action de sÃ©curitÃ© est nÃ©cessaire

**DRIFT_ALERT**
- **Objectif :** Alerter sur une dÃ©rive de consommation
- **Contenu :** EntitÃ©, ressource, tendance, projection
- **Usage :** WorrySentinel peut anticiper une menace

**GOVERNANCE_ISSUE**
- **Objectif :** Signaler une anomalie de gouvernance
- **Contenu :** Nature du problÃ¨me, impact, recommandations
- **Usage :** WorrySentinel peut dÃ©cider d'une action de sÃ©curitÃ©

**ALLOCATION_PATTERN**
- **Objectif :** Signaler un pattern d'allocation atypique
- **Contenu :** Description du pattern, entitÃ©s impliquÃ©es, frÃ©quence
- **Usage :** WorrySentinel corrÃ¨le avec d'autres signaux

---

## 9. Protocole de communication

### 9.1 Format des notifications descendantes

Les notifications de WorrySentinel suivent un format standardisÃ©.

**Structure de base :**

| Ã‰lÃ©ment | Description | Obligatoire |
|---------|-------------|-------------|
| `notification_id` | Identifiant unique de la notification | âœ… Oui |
| `type` | Type de notification | âœ… Oui |
| `payload` | DonnÃ©es spÃ©cifiques Ã  la notification | âœ… Oui |
| `timestamp` | Horodatage de la notification | âœ… Oui |
| `requires_ack` | Si une confirmation est requise | âœ… Oui |

**RÃ¨gle WS-LS-PROT-01 : Notification obligatoire**

Toutes les notifications de WorrySentinel doivent Ãªtre transmises Ã  LogisticsSteward sans filtrage ni dÃ©lai.

### 9.2 Format des signalements montants

Les signalements de LogisticsSteward suivent un format standardisÃ©.

**Structure de base :**

| Ã‰lÃ©ment | Description | Obligatoire |
|---------|-------------|-------------|
| `signal_id` | Identifiant unique du signalement | âœ… Oui |
| `type` | Type de signalement | âœ… Oui |
| `payload` | DonnÃ©es spÃ©cifiques au signalement | âœ… Oui |
| `timestamp` | Horodatage du signalement | âœ… Oui |
| `urgency` | Niveau d'urgence (low, medium, high, critical) | âœ… Oui |

**RÃ¨gle WS-LS-PROT-02 : RÃ©ception obligatoire**

WorrySentinel doit recevoir tous les signalements de LogisticsSteward sans exception. Aucun filtrage n'est autorisÃ©.

### 9.3 Confirmations et acquittements

**RÃ¨gle WS-LS-PROT-03 : Acquittement par LogisticsSteward**

LogisticsSteward acquitte toutes les notifications descendantes avec `requires_ack: true`.

**RÃ¨gle WS-LS-PROT-04 : Pas d'acquittement montant**

WorrySentinel n'acquitte pas les signalements montants. Le traitement est interne Ã  WorrySentinel.

---

## 10. Flux d'intÃ©gration typiques

### 10.1 Flux de supervision normale

**Acteurs :** WorrySentinel, LogisticsSteward

**SÃ©quence :**

1. LogisticsSteward procÃ¨de Ã  des arbitrages normaux
2. LogisticsSteward gÃ©nÃ¨re des signaux d'allocation pÃ©riodiques
3. LogisticsSteward envoie `ALLOCATION_PATTERN` Ã  WorrySentinel
4. WorrySentinel observe et corrÃ¨le les patterns
5. Si pas d'anomalie, aucune action
6. Les signaux sont tracÃ©s pour audit

### 10.2 Flux de dÃ©tection de dÃ©rive

**Acteurs :** LogisticsSteward, WorrySentinel

**SÃ©quence :**

1. LogisticsSteward dÃ©tecte une dÃ©rive de consommation sur une entitÃ©
2. LogisticsSteward gÃ©nÃ¨re un `DRIFT_ALERT`
3. LogisticsSteward envoie l'alerte Ã  WorrySentinel
4. WorrySentinel reÃ§oit et analyse l'alerte
5. WorrySentinel corrÃ¨le avec d'autres signaux
6. WorrySentinel dÃ©cide de l'action (surveillance, durcissement, ou escalade)

### 10.3 Flux de durcissement

**Acteurs :** WorrySentinel, LogisticsSteward

**SÃ©quence :**

1. WorrySentinel dÃ©tecte une menace confirmÃ©e (corrÃ©lation de signaux)
2. WorrySentinel gÃ©nÃ¨re une `HARDENING_DIRECTIVE`
3. WorrySentinel envoie la directive Ã  LogisticsSteward
4. LogisticsSteward reÃ§oit la directive
5. LogisticsSteward applique immÃ©diatement les restrictions
6. LogisticsSteward acquitte avec `ACK_OK`
7. Les entitÃ©s concernÃ©es subissent les restrictions

### 10.4 Flux de levÃ©e de durcissement

**Acteurs :** WorrySentinel, LogisticsSteward

**SÃ©quence :**

1. WorrySentinel constate que la menace est rÃ©solue
2. WorrySentinel gÃ©nÃ¨re une `HARDENING_LIFT`
3. WorrySentinel envoie la levÃ©e Ã  LogisticsSteward
4. LogisticsSteward reÃ§oit la levÃ©e
5. LogisticsSteward rÃ©tablit les rÃ¨gles d'arbitrage normales
6. LogisticsSteward acquitte avec `ACK_OK`

### 10.5 Diagramme de sÃ©quence

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  WorrySentinel  â”‚    â”‚LogisticsSteward â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚                      â”‚
         â”‚â—„â”€â”€ ALLOCATION_PATTERN â”€â”¤
         â”‚                      â”‚
         â”œâ”€â”€ CorrÃ©lation â”€â”€â”€â”€â”€â”€â”€â”¤
         â”‚                      â”‚
         â”‚â—„â”€â”€ DRIFT_ALERT â”€â”€â”€â”€â”€â”€â”¤
         â”‚                      â”‚
         â”œâ”€â”€ Analyse â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
         â”‚                      â”‚
         â”œâ”€â”€ HARDENING_DIRECTIVE â”€â–ºâ”‚
         â”‚    (quota_reduction)  â”‚
         â”‚                      â”‚
         â”‚                      â”œâ”€â”€ Application restrictions
         â”‚                      â”‚
         â”‚â—„â”€â”€ ACK_OK â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
         â”‚                      â”‚
         â”‚   ... temps ...      â”‚
         â”‚                      â”‚
         â”œâ”€â”€ HARDENING_LIFT â”€â”€â”€â”€â–ºâ”‚
         â”‚                      â”‚
         â”‚                      â”œâ”€â”€ RÃ©tablissement
         â”‚                      â”‚
         â”‚â—„â”€â”€ ACK_OK â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
         â”‚                      â”‚
```

---

## 11. RÃ¨gles d'intÃ©gration

### 11.1 RÃ¨gles de communication

**RÃ¨gle WS-LS-INT-01 : Bidirectionnel asymÃ©trique**

La communication est bidirectionnelle mais asymÃ©trique. WorrySentinel impose des contraintes, LogisticsSteward signale des observations. Les rÃ´les ne sont pas interchangeables.

**RÃ¨gle WS-LS-INT-02 : PrioritÃ© des contraintes sÃ©curitaires**

Les contraintes de WorrySentinel sont prioritaires sur toutes les rÃ¨gles d'arbitrage de LogisticsSteward. Aucune rÃ¨gle locale ne peut contredire une contrainte de sÃ©curitÃ©.

**RÃ¨gle WS-LS-INT-03 : Non-blocage des signaux**

Les signalements de LogisticsSteward sont toujours non bloquants. L'envoi n'attend jamais de rÃ©ponse.

### 11.2 RÃ¨gles de donnÃ©es

**RÃ¨gle WS-LS-INT-04 : DonnÃ©es de classification**

Les donnÃ©es Ã©changÃ©es sont des informations de classification (Ã©tats, niveaux, alertes, patterns), jamais des donnÃ©es mÃ©tier.

**RÃ¨gle WS-LS-INT-05 : Pas de donnÃ©es personnelles**

Aucune donnÃ©e personnelle n'est Ã©changÃ©e. Les signalements concernent des entitÃ©s (opÃ©rateurs, services), pas des utilisateurs.

**RÃ¨gle WS-LS-INT-06 : CohÃ©rence garantie**

WorrySentinel garantit la cohÃ©rence de ses notifications. LogisticsSteward peut se fier aux Ã©tats et niveaux communiquÃ©s.

### 11.3 RÃ¨gles de traÃ§abilitÃ©

**RÃ¨gle WS-LS-INT-07 : TraÃ§abilitÃ© complÃ¨te**

Toutes les interactions sont tracÃ©es avec contexte complet par les deux parties.

**RÃ¨gle WS-LS-INT-08 : CorrÃ©lation possible**

Chaque notification peut Ãªtre corrÃ©lÃ©e aux adaptations d'arbitrage qui en dÃ©coulent.

---

## 12. Gestion des erreurs

### 12.1 Types d'erreurs

**Erreurs de format :**
- Signalement mal formÃ©
- Champ obligatoire manquant
- Type de signalement inconnu

**Erreurs de corrÃ©lation :**
- Signal non corrÃ©lable avec d'autres sources
- Pattern non reconnu
- EntitÃ© inconnue

**Erreurs internes :**
- Erreur du moteur de corrÃ©lation
- Erreur de journalisation

### 12.2 Traitement des erreurs

**RÃ¨gle WS-LS-ERR-01 : Journalisation des erreurs**

Toutes les erreurs sont journalisÃ©es pour audit et diagnostic.

**RÃ¨gle WS-LS-ERR-02 : Pas de blocage sur erreur**

Une erreur de traitement ne bloque pas la supervision. WorrySentinel continue Ã  recevoir et traiter les autres signaux.

**RÃ¨gle WS-LS-ERR-03 : SÃ©curitÃ© par dÃ©faut**

En cas d'erreur de communication avec LogisticsSteward, WorrySentinel applique le comportement le plus restrictif (principe de sÃ©curitÃ© par dÃ©faut).

**RÃ¨gle WS-LS-ERR-04 : Alerte sur erreurs rÃ©pÃ©tÃ©es**

Des erreurs rÃ©pÃ©tÃ©es dÃ©clenchent une alerte interne et peuvent influencer l'Ã©tat de confiance.

---

## 13. Cas particuliers

### 13.1 Ã‰tat de confiance T4 (BloquÃ©)

En Ã©tat T4, WorrySentinel impose un mode survie :

**RÃ¨gle WS-LS-CASE-01 : Blocage maximal**

En T4, WorrySentinel envoie une directive `ALLOCATION_BLOCK` globale. Seuls les services vitaux reÃ§oivent des ressources.

### 13.2 LogisticsSteward indisponible

Si LogisticsSteward ne rÃ©pond pas aux notifications :

**RÃ¨gle WS-LS-CASE-02 : Escalade d'alerte**

L'indisponibilitÃ© de LogisticsSteward est une alerte de sÃ©curitÃ©. WorrySentinel peut dÃ©cider de dÃ©grader l'Ã©tat de confiance.

### 13.3 Signaux contradictoires

Si les signaux de LogisticsSteward contredisent d'autres sources :

**RÃ¨gle WS-LS-CASE-03 : PrioritÃ© Ã  la sÃ©curitÃ©**

En cas de contradiction, WorrySentinel applique le scÃ©nario le plus restrictif. La sÃ©curitÃ© prime sur la disponibilitÃ©.

### 13.4 DÃ©rive sur entitÃ© de niveau 4

Si une dÃ©rive est dÃ©tectÃ©e sur une entitÃ© de niveau de sÃ©curitÃ© 4 :

**RÃ¨gle WS-LS-CASE-04 : Escalade immÃ©diate**

Toute dÃ©rive sur une entitÃ© de niveau 4 dÃ©clenche une escalade immÃ©diate. L'Ã©tat de confiance peut Ãªtre dÃ©gradÃ©.

---

## 14. Garanties de l'intÃ©gration

### 14.1 Garantie de supervision continue

**Engagement :** WorrySentinel supervise en continu les signaux de LogisticsSteward. Aucune interruption de supervision n'est acceptable.

### 14.2 Garantie de rÃ©activitÃ©

**Engagement :** WorrySentinel rÃ©agit immÃ©diatement aux alertes de dÃ©rive critique. Aucun dÃ©lai supÃ©rieur Ã  une seconde n'est acceptable pour les alertes critiques.

### 14.3 Garantie de proportionnalitÃ©

**Engagement :** Le durcissement est toujours proportionnel Ã  la menace dÃ©tectÃ©e. Pas de durcissement excessif sans justification.

### 14.4 Garantie de rÃ©versibilitÃ©

**Engagement :** Tout durcissement peut Ãªtre levÃ© par une directive explicite. Le retour Ã  la normale est toujours possible.

### 14.5 Garantie de traÃ§abilitÃ©

**Engagement :** Toute interaction est traÃ§able de bout en bout. L'audit complet des notifications, directives et signalements est possible.

### 14.6 Garantie de non-substitution

**Engagement :** WorrySentinel ne se substitue jamais Ã  LogisticsSteward. L'arbitrage reste la responsabilitÃ© exclusive de LogisticsSteward.

---

## 15. Invariants de l'intÃ©gration

### 15.1 Invariants de relation

**INV-WS-LS-1 : Supervision sans exÃ©cution**

WorrySentinel supervise LogisticsSteward. WorrySentinel n'exÃ©cute jamais d'arbitrage.

**INV-WS-LS-2 : Contrainte unidirectionnelle**

WorrySentinel impose des contraintes Ã  LogisticsSteward. LogisticsSteward n'impose jamais de contraintes Ã  WorrySentinel.

**INV-WS-LS-3 : SouverainetÃ© d'arbitrage**

LogisticsSteward reste souverain sur l'arbitrage. WorrySentinel contraint, mais ne dÃ©cide pas.

### 15.2 Invariants de donnÃ©es

**INV-WS-LS-4 : Pas de dÃ©cision d'allocation**

WorrySentinel ne prend aucune dÃ©cision d'allocation. Il impose des contraintes que LogisticsSteward traduit.

**INV-WS-LS-5 : Signaux informatifs**

Les signaux de LogisticsSteward sont informatifs. Ils alimentent la corrÃ©lation mais n'imposent aucune action.

### 15.3 Invariants de protocole

**INV-WS-LS-6 : Format respectÃ©**

Toutes les notifications et signalements respectent le format standardisÃ©.

**INV-WS-LS-7 : TraÃ§abilitÃ© complÃ¨te**

Toute interaction est traÃ§able avec son contexte complet.

---

## 16. ConformitÃ© aux Lois d'Autonomie SystÃ¨me

### LOI-1 : Aucune dÃ©pendance externe critique

**ConformitÃ© :** âœ… **Conforme**

L'intÃ©gration respecte LOI-1 :
- WorrySentinel supervise localement
- LogisticsSteward adapte ses rÃ¨gles localement
- L'absence de connexion ne bloque ni la supervision ni l'arbitrage

### LOI-2 : Le systÃ¨me accepte l'isolement comme Ã©tat normal

**ConformitÃ© :** âœ… **Conforme**

L'intÃ©gration respecte LOI-2 :
- En isolement, la supervision continue avec les signaux locaux
- Les contraintes locales restent actives
- Aucune dÃ©gradation de l'intÃ©gration en mode isolÃ©

### LOI-4 : Pas de temps global requis

**ConformitÃ© :** âœ… **Conforme**

L'intÃ©gration respecte LOI-4 :
- Les horodatages sont locaux
- Aucune synchronisation temporelle n'est requise
- La corrÃ©lation ne dÃ©pend pas de timestamps synchronisÃ©s

---

## 17. Exemples

### 17.1 Notification de changement d'Ã©tat de confiance

**Notification WorrySentinel :**
```
{
  "notification_id": "notif-ws-ls-001",
  "type": "TRUST_STATE_CHANGE",
  "payload": {
    "previous_state": "T0",
    "new_state": "T2",
    "transition_reason": "DÃ©rives multiples dÃ©tectÃ©es, corrÃ©lation confirmÃ©e",
    "constraints": {
      "quota_reduction_percent": 20,
      "priority_freeze": false
    }
  },
  "timestamp": "2026-01-28T14:00:00Z",
  "requires_ack": true
}
```

### 17.2 Directive de durcissement

**Directive WorrySentinel :**
```
{
  "notification_id": "notif-ws-ls-002",
  "type": "HARDENING_DIRECTIVE",
  "payload": {
    "directive_id": "hard-001",
    "hardening_type": "quota_reduction",
    "affected_entities": ["operator-media-service", "operator-analytics"],
    "severity": "high",
    "duration": null,
    "justification": "Pattern de consommation anormal, risque de saturation"
  },
  "timestamp": "2026-01-28T14:05:00Z",
  "requires_ack": true
}
```

### 17.3 Signalement de dÃ©rive (reÃ§u par WorrySentinel)

**Signalement LogisticsSteward :**
```
{
  "signal_id": "signal-ls-001",
  "type": "DRIFT_ALERT",
  "payload": {
    "alert_id": "drift-001",
    "entity_id": "operator-media-service",
    "resource_type": "computation_quota",
    "current_usage": 92,
    "trend": "increasing",
    "projection": "2026-01-28T16:00:00Z"
  },
  "timestamp": "2026-01-28T14:00:00Z",
  "urgency": "high"
}
```

### 17.4 LevÃ©e de durcissement

**Directive WorrySentinel :**
```
{
  "notification_id": "notif-ws-ls-003",
  "type": "HARDENING_LIFT",
  "payload": {
    "lift_id": "lift-001",
    "directive_id": "hard-001",
    "justification": "Menace rÃ©solue, consommation normalisÃ©e"
  },
  "timestamp": "2026-01-28T18:00:00Z",
  "requires_ack": true
}
```

---

## 18. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il Ã©tablit l'interface et le protocole que WorrySentinel doit respecter pour superviser LogisticsSteward.

Toute implÃ©mentation de l'intÃ©gration avec LogisticsSteward doit respecter ce contrat. Toute violation entraÃ®ne un comportement non conforme.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif  
**DÃ©pendances :**
- WorrySentinel - Documentation Fondatrice v1.2 (Section 9)
- LogisticsSteward - Documentation Fondatrice v1.0.0 (Section 8.4)
- LogisticsSteward - WorrySentinel Integration Contract v1.0
- Miyukini Conceptual References - Lois Autonomie SystÃ¨me v1.1

---

## 19. Mini log de gÃ©nÃ©ration

### DÃ©cision Ã©ditoriale E1 : Point de vue WorrySentinel

**DÃ©cision prise :** Ce document est rÃ©digÃ© du point de vue de WorrySentinel (superviseur), contrairement au document symÃ©trique qui est du point de vue de LogisticsSteward (supervisÃ©). Cette approche assure une documentation complÃ¨te et cohÃ©rente des deux cÃ´tÃ©s.

**Application :** Tout le document est structurÃ© autour du rÃ´le de supervision de WorrySentinel.

### DÃ©cision Ã©ditoriale E2 : CohÃ©rence avec le document symÃ©trique

**DÃ©cision prise :** Ce document doit Ãªtre cohÃ©rent avec [LogisticsSteward - WorrySentinel Integration Contract](../../../LogisticsSteward/contracts/integration/LogisticsSteward%20-%20WorrySentinel%20Integration%20Contract.md). Les mÃªmes structures de donnÃ©es, les mÃªmes rÃ¨gles, et les mÃªmes invariants sont utilisÃ©s.

**Application :** Les structures de donnÃ©es et les rÃ¨gles sont alignÃ©es avec le document symÃ©trique.

### DÃ©cision Ã©ditoriale E3 : Supervision vs Substitution

**DÃ©cision prise :** Le document insiste sur le fait que WorrySentinel supervise sans se substituer Ã  LogisticsSteward. Cette distinction est critique pour prÃ©server la sÃ©paration des responsabilitÃ©s.

**Application :** Section 4.1 et Section 5 Ã©tablissent clairement cette distinction.

### Warning W1 : Risque de confusion supervision/arbitrage

**Warning rencontrÃ© :** Risque que WorrySentinel prenne des dÃ©cisions d'allocation dÃ©guisÃ©es en contraintes de sÃ©curitÃ©.

**DÃ©cision prise :** Les interdictions absolues (Section 5) clarifient que WorrySentinel ne dÃ©cide jamais de l'allocation et ne dÃ©finit jamais les quotas.

**Correction effectuÃ©e :** Section 5 explicite les interdictions, INV-WS-LS-4 Ã©tablit que WorrySentinel ne prend aucune dÃ©cision d'allocation.

### VÃ©rification de cohÃ©rence

**VÃ©rification effectuÃ©e :**
- âœ… CohÃ©rence avec WorrySentinel - Documentation Fondatrice : ConfirmÃ©e (Section 9 respectÃ©e)
- âœ… CohÃ©rence avec LogisticsSteward - Documentation Fondatrice : ConfirmÃ©e (Section 8.4)
- âœ… CohÃ©rence avec LogisticsSteward - WorrySentinel Integration Contract : ConfirmÃ©e (symÃ©trie)
- âœ… ConformitÃ© LOI-1 : ConfirmÃ©e (aucune dÃ©pendance externe)
- âœ… ConformitÃ© LOI-2 : ConfirmÃ©e (isolement gÃ©rÃ©)
- âœ… ConformitÃ© LOI-4 : ConfirmÃ©e (pas de temps global requis)
- âœ… Pas de dÃ©cision d'allocation par WorrySentinel : ConfirmÃ©e (INV-WS-LS-4)
- âœ… Supervision sans substitution : ConfirmÃ©e (INV-WS-LS-3)
- âœ… TraÃ§abilitÃ© complÃ¨te : ConfirmÃ©e (INV-WS-LS-7)
- âœ… RÃ¨gles RÃˆGLE-WS-LS-1 Ã  RÃˆGLE-WS-LS-4 respectÃ©es : ConfirmÃ©e

**Conclusion :** Aucune contradiction dÃ©tectÃ©e. Le document est cohÃ©rent et non ambigu.

---

*Aucune autre erreur, warning, ou ambiguÃ¯tÃ© rencontrÃ©e lors de la rÃ©daction de ce document.*

