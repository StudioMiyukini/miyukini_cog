# LogisticsSteward - WorrySentinel Integration Contract

## 1. Contexte

Ce document definit le **contrat d'integration entre LogisticsSteward et WorrySentinel**. Il specifie l'interface, le protocole, les regles de communication, et les garanties associees a l'integration avec WorrySentinel en tant que gouvernant de la securite et des etats de confiance.

Ce document complete la Section 8.4 de la [Documentation Fondatrice](../../foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [WorrySentinel - Documentation Fondatrice](..//..//..//WorrySentinel//foundation//WorrySentinel%20-%20Documentation%20Fondatrice.md) pour la nature de WorrySentinel
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) pour la conformite LOI-1 a LOI-6

L'integration respecte les Lois d'Autonomie Systeme : toutes les adaptations de regles sont locales et ne requierent aucune dependance externe (**LOI-1**).

## 2. Portee / Scope

Ce document couvre :
- L'interface contractuelle entre LogisticsSteward et WorrySentinel
- Le protocole de communication (contraintes descendantes et observations montantes)
- Les types d'informations echangees
- L'impact des etats de confiance sur l'arbitrage
- L'adaptation des regles selon les niveaux de securite
- Les regles d'integration specifiques
- Les garanties de l'integration

Ce document **ne couvre pas** :
- Les details internes de WorrySentinel (voir documentation WorrySentinel)
- Les details internes du moteur d'arbitrage (voir Architecture)
- L'integration avec Kernel (voir Kernel Integration Contract)
- L'integration avec StrongFather (voir StrongFather Integration Contract)
- L'integration avec MasterButler (voir MasterButler Integration Contract)
- L'integration avec BondingBrother (voir BondingBrother Integration Contract)

---

## 3. Principe fondamental

**WorrySentinel gouverne les niveaux de securite et les etats de confiance. LogisticsSteward adapte ses regles d'arbitrage en consequence. WorrySentinel ne decide jamais des allocations, LogisticsSteward n'evalue jamais les menaces.**

La relation est de **contrainte verticale** : WorrySentinel impose des contraintes de securite, LogisticsSteward adapte ses politiques d'arbitrage. WorrySentinel observe les comportements d'arbitrage, LogisticsSteward remonte les signaux de derive.

---

## 4. Nature de la relation LogisticsSteward â€” WorrySentinel

### 4.1 Relation de contrainte verticale

**WorrySentinel contraint LogisticsSteward par :**
- L'etat de confiance global (T0-T4) qui modifie la severite de l'arbitrage
- Les niveaux de securite (0-4) qui definissent les contraintes par entite
- Les alertes de durcissement qui declenchent des restrictions immediates
- Les invalidations d'etat systeme qui forcent une reevaluation

**LogisticsSteward informe WorrySentinel par :**
- Les comportements suspects d'arbitrage (demandes excessives, patterns anormaux)
- Les derives de consommation (depassements de quotas, tendances alarmantes)
- Les tentatives de contournement des regles
- Les anomalies de gouvernance detectees

**Regle LS-WS-01 : Contrainte sans decision**

WorrySentinel impose des contraintes de securite a LogisticsSteward. WorrySentinel ne decide jamais de l'allocation, de la priorite, ou de la limitation des ressources. Ces decisions appartiennent exclusivement a LogisticsSteward.

**Regle LS-WS-02 : Adaptation obligatoire**

LogisticsSteward doit adapter ses regles d'arbitrage selon les etats de confiance et les niveaux de securite gouvernes par WorrySentinel. L'adaptation n'est pas facultative.

**Regle LS-WS-03 : Observation sans modification**

WorrySentinel observe les comportements d'arbitrage de LogisticsSteward sans jamais modifier les decisions d'arbitrage. L'observation est passive et non intrusive.

### 4.2 Separation des responsabilites

| Responsabilite | LogisticsSteward | WorrySentinel |
|----------------|------------------|---------------|
| **Arbitrer l'allocation des ressources** | âœ… Exclusif | âŒ Jamais |
| **Definir les quotas et priorites** | âœ… Exclusif | âŒ Jamais |
| **Gouverner les etats de confiance** | âŒ Consomme | âœ… Exclusif |
| **Definir les niveaux de securite** | âŒ Consomme | âœ… Exclusif |
| **Declencher le durcissement** | âŒ Subit | âœ… Exclusif |
| **Invalider un etat systeme** | âŒ Reagit | âœ… Peut decider |
| **Observer les comportements** | âŒ Source | âœ… Exclusif |
| **Detecter les derives** | âœ… Source | âœ… Consomme |

**Regle LS-WS-04 : Aucun chevauchement**

Aucun chevauchement de responsabilites n'est autorise. LogisticsSteward n'evalue jamais les menaces, WorrySentinel ne decide jamais des allocations.

---

## 5. Ce que LogisticsSteward ne fait JAMAIS vis-a-vis de WorrySentinel

### 5.1 Interdictions absolues

**INV-LS-WS-NEVER-1 : Ne gouverne jamais les etats de confiance**

LogisticsSteward ne gouverne **jamais** les etats de confiance (T0-T4). La definition et la transition des etats de confiance appartiennent exclusivement a WorrySentinel.

**INV-LS-WS-NEVER-2 : Ne definit jamais les niveaux de securite**

LogisticsSteward ne definit **jamais** les niveaux de securite (0-4). La definition des niveaux de securite appartient exclusivement a WorrySentinel.

**INV-LS-WS-NEVER-3 : Ne declenche jamais de durcissement**

LogisticsSteward ne declenche **jamais** de durcissement de securite. Il peut remonter des signaux d'alerte, mais c'est WorrySentinel qui decide du durcissement.

**INV-LS-WS-NEVER-4 : N'invalide jamais un etat systeme**

LogisticsSteward n'invalide **jamais** un etat systeme. Si l'etat systeme fourni par le Kernel semble incoherent, LogisticsSteward signale a WorrySentinel, qui peut invalider.

**INV-LS-WS-NEVER-5 : N'ignore jamais les contraintes de securite**

LogisticsSteward n'ignore **jamais** les contraintes imposees par WorrySentinel. L'adaptation aux etats de confiance et niveaux de securite est obligatoire.

**INV-LS-WS-NEVER-6 : Ne contourne jamais l'observation**

LogisticsSteward ne contourne **jamais** l'observation de WorrySentinel. Les comportements d'arbitrage sont transparents et observables.

---

## 6. Impact des etats de confiance sur l'arbitrage

### 6.1 Adaptation de l'arbitrage par etat de confiance

WorrySentinel gouverne les etats de confiance (T0-T4). LogisticsSteward adapte son arbitrage en consequence :

**T0 â€” Normal**

| Aspect | Comportement |
|--------|--------------|
| **Quotas** | Application normale des quotas declares |
| **Priorites** | Priorites standards respectees |
| **Degradation** | Niveau D0 - Aucune degradation |
| **Restrictions** | Aucune restriction supplementaire |
| **Tracabilite** | Journalisation standard |

**T1 â€” Instable**

| Aspect | Comportement |
|--------|--------------|
| **Quotas** | Application normale avec surveillance renforcee |
| **Priorites** | Priorites standards avec tracabilite etendue |
| **Degradation** | Preparation possible niveau D1 |
| **Restrictions** | Aucune restriction, mais alertes actives |
| **Tracabilite** | Journalisation detaillee de toutes les decisions |

**T2 â€” Degrade**

| Aspect | Comportement |
|--------|--------------|
| **Quotas** | Quotas reduits de 20% pour entites non essentielles |
| **Priorites** | Priorite maximale reservee aux services critiques |
| **Degradation** | Niveau D1-D2 actif |
| **Restrictions** | Desactivation de fonctionnalites non critiques |
| **Tracabilite** | Journalisation complete avec contexte de securite |

**T3 â€” Restreint**

| Aspect | Comportement |
|--------|--------------|
| **Quotas** | Quotas minimaux, gel des nouvelles allocations |
| **Priorites** | Seules les priorites critiques sont honorees |
| **Degradation** | Niveau D3 actif |
| **Restrictions** | Services minimaux uniquement |
| **Tracabilite** | Journalisation exhaustive avec justification |

**T4 â€” Bloque**

| Aspect | Comportement |
|--------|--------------|
| **Quotas** | Aucune nouvelle allocation |
| **Priorites** | Preservation du coeur systeme uniquement |
| **Degradation** | Niveau D4 - Survie |
| **Restrictions** | Blocage de toutes les operations non vitales |
| **Tracabilite** | Journalisation minimale, preservation des ressources |

**Regle LS-WS-STATE-01 : Adaptation immediate**

L'adaptation de l'arbitrage a un changement d'etat de confiance est immediate. Aucun delai n'est autorise.

**Regle LS-WS-STATE-02 : Coherence degradation-confiance**

Le niveau de degradation de LogisticsSteward est coherent avec l'etat de confiance de WorrySentinel. Un etat T3 implique au minimum D2-D3.

### 6.2 Niveaux de securite par entite

WorrySentinel attribue des niveaux de securite (0-4) aux entites. LogisticsSteward adapte l'arbitrage par entite :

| Niveau | Impact sur l'arbitrage |
|--------|------------------------|
| **Niveau 0 - Public** | Priorite minimale, quotas standards, preemption possible |
| **Niveau 1 - Standard** | Priorite normale, quotas standards, preemption en dernier |
| **Niveau 2 - Sensitive** | Priorite elevee, quotas proteges, resistance a la preemption |
| **Niveau 3 - Critical** | Priorite maximale, quotas garantis, aucune preemption |
| **Niveau 4 - Highest** | Priorite absolue, ressources reservees, protection totale |

**Regle LS-WS-SEC-01 : Respect des niveaux**

LogisticsSteward respecte les niveaux de securite attribues par WorrySentinel. Un niveau de securite eleve implique une protection accrue contre la preemption et la degradation.

**Regle LS-WS-SEC-02 : Cumul etat-niveau**

Les restrictions sont cumulatives : une entite de niveau 2 en etat T2 subit les restrictions de T2 adaptees a son niveau 2 (moins severes que pour niveau 0).

---

## 7. Types d'informations echangees

### 7.1 Flux descendant : WorrySentinel â†’ LogisticsSteward

**TRUST_STATE_CHANGE**
- **Objectif :** Notifier un changement d'etat de confiance
- **Contenu :** Nouvel etat (T0-T4), justification, timestamp
- **Impact :** Adaptation immediate des regles d'arbitrage

**Structure du changement d'etat :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `notification_id` | Identifiant unique de la notification | âœ… Oui |
| `previous_state` | Etat de confiance precedent (T0-T4) | âœ… Oui |
| `new_state` | Nouvel etat de confiance (T0-T4) | âœ… Oui |
| `transition_reason` | Justification de la transition | âœ… Oui |
| `timestamp` | Horodatage de la transition | âœ… Oui |
| `constraints` | Contraintes supplementaires applicables | âŒ Optionnel |

**SECURITY_LEVEL_ASSIGNMENT**
- **Objectif :** Attribuer ou modifier le niveau de securite d'une entite
- **Contenu :** Entite concernee, niveau (0-4), justification
- **Impact :** Adaptation de l'arbitrage pour cette entite

**Structure de l'attribution de niveau :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `assignment_id` | Identifiant unique de l'attribution | âœ… Oui |
| `entity_id` | Identifiant de l'entite concernee | âœ… Oui |
| `entity_type` | Type d'entite (operator, team, service, tool) | âœ… Oui |
| `security_level` | Niveau de securite (0-4) | âœ… Oui |
| `justification` | Raison de l'attribution | âœ… Oui |
| `timestamp` | Horodatage de l'attribution | âœ… Oui |

**HARDENING_DIRECTIVE**
- **Objectif :** Declencher un durcissement immediat des regles
- **Contenu :** Type de durcissement, entites concernees, duree
- **Impact :** Restrictions supplementaires appliquees immediatement

**Structure de la directive de durcissement :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `directive_id` | Identifiant unique de la directive | âœ… Oui |
| `hardening_type` | Type de durcissement (quota_reduction, priority_freeze, allocation_block) | âœ… Oui |
| `affected_entities` | Liste des entites concernees (vide = toutes) | âŒ Optionnel |
| `severity` | Severite du durcissement (low, medium, high, critical) | âœ… Oui |
| `duration` | Duree du durcissement (null = indefini) | âŒ Optionnel |
| `justification` | Raison du durcissement | âœ… Oui |
| `timestamp` | Horodatage de la directive | âœ… Oui |

**STATE_INVALIDATION**
- **Objectif :** Invalider l'etat systeme actuel
- **Contenu :** Raison de l'invalidation, action requise
- **Impact :** LogisticsSteward doit attendre un nouvel etat systeme valide

**Structure de l'invalidation :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `invalidation_id` | Identifiant unique de l'invalidation | âœ… Oui |
| `reason` | Raison de l'invalidation | âœ… Oui |
| `required_action` | Action requise de LogisticsSteward | âœ… Oui |
| `timestamp` | Horodatage de l'invalidation | âœ… Oui |

### 7.2 Flux montant : LogisticsSteward â†’ WorrySentinel

**ANOMALY_REPORT**
- **Objectif :** Signaler un comportement d'arbitrage suspect
- **Contenu :** Nature de l'anomalie, entite concernee, contexte
- **Usage :** WorrySentinel evalue si une action de securite est necessaire

**Structure du rapport d'anomalie :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `report_id` | Identifiant unique du rapport | âœ… Oui |
| `anomaly_type` | Type d'anomalie (excessive_requests, pattern_violation, quota_bypass_attempt) | âœ… Oui |
| `affected_entity` | Entite concernee par l'anomalie | âœ… Oui |
| `context` | Contexte de l'anomalie (decisions recentes, etat) | âœ… Oui |
| `severity_assessment` | Evaluation de gravite par LogisticsSteward | âœ… Oui |
| `timestamp` | Horodatage de l'anomalie | âœ… Oui |

**DRIFT_ALERT**
- **Objectif :** Alerter sur une derive de consommation
- **Contenu :** Entite, ressource, tendance, projection
- **Usage :** WorrySentinel peut anticiper une menace

**Structure de l'alerte de derive :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `alert_id` | Identifiant unique de l'alerte | âœ… Oui |
| `entity_id` | Entite concernee | âœ… Oui |
| `resource_type` | Type de ressource concernee | âœ… Oui |
| `current_usage` | Usage actuel (pourcentage du quota) | âœ… Oui |
| `trend` | Tendance (increasing, decreasing, stable) | âœ… Oui |
| `projection` | Projection de depassement (timestamp estime) | âŒ Optionnel |
| `timestamp` | Horodatage de l'alerte | âœ… Oui |

**GOVERNANCE_ISSUE**
- **Objectif :** Signaler une anomalie de gouvernance
- **Contenu :** Nature du probleme, impact, recommendations
- **Usage :** WorrySentinel peut decider d'une action de securite

**Structure du signalement de probleme de gouvernance :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `issue_id` | Identifiant unique du probleme | âœ… Oui |
| `issue_type` | Type de probleme (rule_conflict, inconsistent_state, policy_violation) | âœ… Oui |
| `description` | Description detaillee | âœ… Oui |
| `impact` | Impact sur le systeme | âœ… Oui |
| `timestamp` | Horodatage du signalement | âœ… Oui |

---

## 8. Protocole de communication

### 8.1 Format des notifications descendantes

Les notifications de WorrySentinel suivent un format standardise.

**Structure de base :**

| Element | Description | Obligatoire |
|---------|-------------|-------------|
| `notification_id` | Identifiant unique de la notification | âœ… Oui |
| `type` | Type de notification | âœ… Oui |
| `payload` | Donnees specifiques a la notification | âœ… Oui |
| `timestamp` | Horodatage de la notification | âœ… Oui |
| `requires_ack` | Si une confirmation est requise | âœ… Oui |

**Regle LS-WS-PROT-01 : Traitement immediat**

Toutes les notifications descendantes de WorrySentinel sont traitees immediatement. Aucun delai n'est autorise.

### 8.2 Format des signalements montants

Les signalements de LogisticsSteward suivent un format standardise.

**Structure de base :**

| Element | Description | Obligatoire |
|---------|-------------|-------------|
| `signal_id` | Identifiant unique du signalement | âœ… Oui |
| `type` | Type de signalement | âœ… Oui |
| `payload` | Donnees specifiques au signalement | âœ… Oui |
| `timestamp` | Horodatage du signalement | âœ… Oui |
| `urgency` | Niveau d'urgence (low, medium, high, critical) | âœ… Oui |

**Regle LS-WS-PROT-02 : Signalement non bloquant**

Les signalements montants sont non bloquants. LogisticsSteward continue son fonctionnement apres l'envoi.

### 8.3 Confirmations et acquittements

**Regle LS-WS-PROT-03 : Acquittement obligatoire**

LogisticsSteward acquitte toutes les notifications descendantes avec `requires_ack: true`.

**Structure de l'acquittement :**

| Element | Description | Obligatoire |
|---------|-------------|-------------|
| `ack_id` | Identifiant unique de l'acquittement | âœ… Oui |
| `notification_id` | Reference a la notification | âœ… Oui |
| `status` | Statut (ACK_OK, ACK_PARTIAL, ACK_ERROR) | âœ… Oui |
| `adaptation_applied` | Confirmation de l'adaptation | âœ… Oui |
| `timestamp` | Horodatage de l'acquittement | âœ… Oui |

---

## 9. Flux d'integration typiques

### 9.1 Flux de changement d'etat de confiance

**Acteurs :** WorrySentinel, LogisticsSteward

**Sequence :**

1. WorrySentinel detecte une anomalie et decide de passer de T0 a T1
2. WorrySentinel envoie `TRUST_STATE_CHANGE` a LogisticsSteward
3. LogisticsSteward recoit la notification
4. LogisticsSteward adapte immediatement ses regles d'arbitrage (journalisation detaillee)
5. LogisticsSteward acquitte avec `ACK_OK` et `adaptation_applied: true`
6. LogisticsSteward continue l'arbitrage avec les nouvelles regles

### 9.2 Flux de durcissement d'urgence

**Acteurs :** WorrySentinel, LogisticsSteward

**Sequence :**

1. WorrySentinel detecte une menace et decide un durcissement
2. WorrySentinel envoie `HARDENING_DIRECTIVE` a LogisticsSteward
3. LogisticsSteward recoit la directive
4. LogisticsSteward applique immediatement les restrictions
5. LogisticsSteward acquitte avec `ACK_OK`
6. Les entites concernees subissent les restrictions jusqu'a levee

### 9.3 Flux de signalement de derive

**Acteurs :** LogisticsSteward, WorrySentinel

**Sequence :**

1. LogisticsSteward detecte une derive de consommation sur une entite
2. LogisticsSteward genere un `DRIFT_ALERT`
3. LogisticsSteward envoie l'alerte a WorrySentinel
4. WorrySentinel recoit et analyse l'alerte
5. WorrySentinel peut decider de durcir les regles ou surveiller

### 9.4 Diagramme de sequence

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  WorrySentinel  â”‚    â”‚LogisticsSteward â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚                      â”‚
         â”œâ”€â”€ TRUST_STATE_CHANGE â”€â–ºâ”‚
         â”‚    (T0 â†’ T1)         â”‚
         â”‚                      â”‚
         â”‚                      â”œâ”€â”€ Adaptation immediate
         â”‚                      â”‚   (regles T1)
         â”‚                      â”‚
         â”‚â—„â”€â”€ ACK_OK â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
         â”‚                      â”‚
         â”‚                      â”‚
         â”‚                      â”œâ”€â”€ Detection derive
         â”‚                      â”‚
         â”‚â—„â”€â”€ DRIFT_ALERT â”€â”€â”€â”€â”€â”€â”¤
         â”‚                      â”‚
         â”œâ”€â”€ Analyse â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
         â”‚                      â”‚
         â”œâ”€â”€ HARDENING_DIRECTIVE â”€â–ºâ”‚
         â”‚                      â”‚
         â”‚                      â”œâ”€â”€ Application restrictions
         â”‚                      â”‚
         â”‚â—„â”€â”€ ACK_OK â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
         â”‚                      â”‚
```

---

## 10. Regles d'integration

### 10.1 Regles de communication

**Regle LS-WS-INT-01 : Bidirectionnel asymetrique**

La communication est bidirectionnelle mais asymetrique. WorrySentinel impose, LogisticsSteward signale. WorrySentinel ne repond pas aux signalements de LogisticsSteward.

**Regle LS-WS-INT-02 : Priorite aux contraintes**

Les contraintes de WorrySentinel sont prioritaires sur toutes les regles d'arbitrage de LogisticsSteward. Aucune regle locale ne peut contredire une contrainte de securite.

**Regle LS-WS-INT-03 : Signalements non bloquants**

Les signalements de LogisticsSteward sont toujours non bloquants. L'envoi n'attend jamais de reponse.

### 10.2 Regles de donnees

**Regle LS-WS-INT-04 : Donnees de classification**

Les donnees echangees sont des informations de classification (etats, niveaux, alertes), jamais des donnees metier.

**Regle LS-WS-INT-05 : Pas de donnees personnelles**

Aucune donnee personnelle n'est echangee. Les signalements concernent des entites (operateurs, services), pas des utilisateurs.

**Regle LS-WS-INT-06 : Coherence garantie**

WorrySentinel garantit la coherence de ses notifications. LogisticsSteward peut se fier aux etats et niveaux communiques.

### 10.3 Regles de tracabilite

**Regle LS-WS-INT-07 : Tracabilite complete**

Toutes les interactions sont tracees avec contexte complet.

**Regle LS-WS-INT-08 : Correlation possible**

Chaque notification peut etre correlee aux adaptations d'arbitrage qui en decoulent.

---

## 11. Gestion des erreurs

### 11.1 Types d'erreurs

**Erreurs de format :**
- Notification mal formee
- Champ obligatoire manquant
- Type de notification inconnu

**Erreurs d'application :**
- Entite inconnue dans une directive
- Niveau de securite invalide
- Duree de durcissement invalide

**Erreurs internes :**
- Erreur du moteur d'arbitrage lors de l'adaptation
- Erreur de journalisation

### 11.2 Traitement des erreurs

**Regle LS-WS-ERR-01 : Acquittement avec erreur**

En cas d'erreur, LogisticsSteward acquitte avec `ACK_ERROR` et description du probleme.

**Regle LS-WS-ERR-02 : Application partielle**

Si une adaptation partielle est possible, LogisticsSteward l'applique et acquitte avec `ACK_PARTIAL`.

**Regle LS-WS-ERR-03 : Journalisation des erreurs**

Toutes les erreurs sont journalisees pour audit et diagnostic.

**Regle LS-WS-ERR-04 : Securite par defaut**

En cas d'erreur de communication avec WorrySentinel, LogisticsSteward applique le comportement le plus restrictif (principe de securite par defaut).

---

## 12. Cas particuliers

### 12.1 Etat de confiance T4 (Bloque)

En etat T4, LogisticsSteward passe en mode survie :

**Regle LS-WS-CASE-01 : Mode survie**

En T4, seuls les services vitaux recoivent des ressources. Toutes les nouvelles allocations sont bloquees. Les operations existantes non vitales sont interrompues.

### 12.2 Invalidation de l'etat systeme

Si WorrySentinel invalide l'etat systeme :

**Regle LS-WS-CASE-02 : Suspension d'arbitrage**

LogisticsSteward suspend tout nouvel arbitrage jusqu'a reception d'un etat systeme valide. Les allocations existantes sont maintenues.

### 12.3 Durcissement sans duree

Si une directive de durcissement n'a pas de duree :

**Regle LS-WS-CASE-03 : Durcissement permanent**

Le durcissement reste actif jusqu'a une directive de levee explicite de WorrySentinel.

### 12.4 Conflit entre etats et niveaux

Si une entite de niveau 4 (Highest Security) est en etat T3 :

**Regle LS-WS-CASE-04 : Protection maximale preservee**

Les entites de niveau 4 conservent leurs protections maximales meme en etat degrade. La degradation affecte en priorite les entites de niveaux inferieurs.

---

## 13. Garanties de l'integration

### 13.1 Garantie de reactivite

**Engagement :** LogisticsSteward reagit immediatement aux notifications de WorrySentinel. Aucun delai superieur a une seconde n'est acceptable.

### 13.2 Garantie de conformite

**Engagement :** LogisticsSteward applique toujours les contraintes de WorrySentinel. Aucun arbitrage ne peut contredire une contrainte de securite.

### 13.3 Garantie de transparence

**Engagement :** Les comportements d'arbitrage sont transparents pour WorrySentinel. Les signalements fournissent une visibilite complete.

### 13.4 Garantie de tracabilite

**Engagement :** Toute interaction est traÃ§able de bout en bout. L'audit complet des notifications, adaptations et signalements est possible.

### 13.5 Garantie de disponibilite

**Engagement :** L'integration ne bloque jamais LogisticsSteward. En cas de defaillance de WorrySentinel, LogisticsSteward applique la securite par defaut.

### 13.6 Garantie de non-regression

**Engagement :** Une adaptation de securite ne peut etre annulee que par une directive explicite de WorrySentinel. Pas de retour automatique a un etat moins restrictif.

---

## 14. Invariants de l'integration

### 14.1 Invariants de relation

**INV-LS-WS-1 : Contrainte unidirectionnelle**

WorrySentinel contraint LogisticsSteward. LogisticsSteward ne contraint jamais WorrySentinel.

**INV-LS-WS-2 : Observation passive**

WorrySentinel observe passivement. L'observation ne modifie jamais les decisions d'arbitrage.

**INV-LS-WS-3 : Adaptation obligatoire**

LogisticsSteward adapte obligatoirement ses regles aux contraintes de WorrySentinel.

### 14.2 Invariants de donnees

**INV-LS-WS-4 : Pas de decision de securite**

LogisticsSteward ne prend aucune decision de securite. Il adapte son arbitrage aux decisions de WorrySentinel.

**INV-LS-WS-5 : Signalements informatifs**

Les signalements sont informatifs. Ils n'imposent aucune action a WorrySentinel.

### 14.3 Invariants de protocole

**INV-LS-WS-6 : Format respecte**

Toutes les notifications et signalements respectent le format standardise.

**INV-LS-WS-7 : Tracabilite complete**

Toute interaction est traÃ§able avec son contexte complet.

---

## 15. Conformite aux Lois d'Autonomie Systeme

### LOI-1 : Aucune dependance externe critique

**Conformite :** âœ… **Conforme**

L'integration respecte LOI-1 :
- LogisticsSteward adapte ses regles localement
- WorrySentinel gouverne localement
- L'absence de connexion ne bloque ni l'arbitrage ni la gouvernance

### LOI-2 : Le systeme accepte l'isolement comme etat normal

**Conformite :** âœ… **Conforme**

L'integration respecte LOI-2 :
- En isolement, LogisticsSteward applique la securite par defaut
- Les adaptations locales restent actives
- Aucune degradation de l'integration en mode isole

### LOI-4 : Pas de temps global requis

**Conformite :** âœ… **Conforme**

L'integration respecte LOI-4 :
- Les horodatages sont locaux
- Aucune synchronisation temporelle n'est requise
- Les etats de confiance ne dependent pas de timestamps synchronises

---

## 16. Exemples

### 16.1 Notification de changement d'etat de confiance

**Notification WorrySentinel :**
```
{
  "notification_id": "notif-ws-ls-001",
  "type": "TRUST_STATE_CHANGE",
  "payload": {
    "previous_state": "T0",
    "new_state": "T1",
    "transition_reason": "Anomalie detectee sur les patterns de requetes",
    "constraints": null
  },
  "timestamp": "2026-01-28T10:00:00Z",
  "requires_ack": true
}
```

**Acquittement LogisticsSteward :**
```
{
  "ack_id": "ack-ls-001",
  "notification_id": "notif-ws-ls-001",
  "status": "ACK_OK",
  "adaptation_applied": {
    "logging_level": "detailed",
    "degradation_prepared": "D1"
  },
  "timestamp": "2026-01-28T10:00:01Z"
}
```

### 16.2 Signalement de derive

**Signalement LogisticsSteward :**
```
{
  "signal_id": "signal-ls-001",
  "type": "DRIFT_ALERT",
  "payload": {
    "alert_id": "alert-001",
    "entity_id": "operator-media-service",
    "resource_type": "computation_quota",
    "current_usage": 85,
    "trend": "increasing",
    "projection": "2026-01-28T12:00:00Z"
  },
  "timestamp": "2026-01-28T10:30:00Z",
  "urgency": "medium"
}
```

### 16.3 Directive de durcissement

**Directive WorrySentinel :**
```
{
  "notification_id": "notif-ws-ls-002",
  "type": "HARDENING_DIRECTIVE",
  "payload": {
    "directive_id": "directive-001",
    "hardening_type": "quota_reduction",
    "affected_entities": ["operator-media-service"],
    "severity": "high",
    "duration": null,
    "justification": "Comportement suspect detecte, derive confirmee"
  },
  "timestamp": "2026-01-28T10:35:00Z",
  "requires_ack": true
}
```

**Acquittement LogisticsSteward :**
```
{
  "ack_id": "ack-ls-002",
  "notification_id": "notif-ws-ls-002",
  "status": "ACK_OK",
  "adaptation_applied": {
    "quota_reduction": "50%",
    "affected_entity": "operator-media-service",
    "effective_immediately": true
  },
  "timestamp": "2026-01-28T10:35:01Z"
}
```

---

## 17. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il etablit l'interface et le protocole que LogisticsSteward doit respecter pour s'integrer avec WorrySentinel.

Toute implementation de l'integration avec WorrySentinel doit respecter ce contrat. Toute violation entraine un comportement non conforme.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif  
**Dependances :**
- LogisticsSteward - Documentation Fondatrice v1.0.0 (Section 8.4)
- WorrySentinel - Documentation Fondatrice v1.2
- Miyukini Conceptual References - Lois Autonomie Systeme v1.1

---

## 18. Mini log de generation

### Decision editoriale E1 : Direction de la relation

**Decision prise :** La relation est de contrainte verticale : WorrySentinel impose des contraintes, LogisticsSteward adapte son arbitrage. Cette direction respecte la Documentation Fondatrice de LogisticsSteward Section 8.4 et la philosophie de WorrySentinel comme "pression verticale".

**Application :** Tout le document est structure autour de cette relation de contrainte unidirectionnelle.

### Decision editoriale E2 : Adaptation obligatoire

**Decision prise :** L'adaptation aux contraintes de WorrySentinel est explicitement obligatoire, contrairement a d'autres integrations ou la consultation est facultative. WorrySentinel gouverne la securite, LogisticsSteward doit s'y conformer.

**Application :** Regle LS-WS-02, INV-LS-WS-3, et Section 5 etablissent cette obligation.

### Decision editoriale E3 : Coherence degradation-confiance

**Decision prise :** Le niveau de degradation de LogisticsSteward doit etre coherent avec l'etat de confiance de WorrySentinel. Un etat T3 implique au minimum D2-D3.

**Application :** Regle LS-WS-STATE-02 et Section 6.1 etablissent cette coherence.

### Warning W1 : Risque de confusion securite/arbitrage

**Warning rencontre :** Risque que LogisticsSteward prenne des decisions de securite deguisees en arbitrage.

**Decision prise :** Les interdictions absolues (Section 5) clarifient que LogisticsSteward n'evalue jamais les menaces et ne gouverne jamais les etats de confiance.

**Correction effectuee :** Section 5 explicite les interdictions, INV-LS-WS-4 etablit que LogisticsSteward ne prend aucune decision de securite.

### Verification de coherence

**Verification effectuee :**
- âœ… Coherence avec LogisticsSteward - Documentation Fondatrice : Confirmee (Section 8.4 respectee)
- âœ… Coherence avec WorrySentinel - Documentation Fondatrice : Confirmee (flux descendant et montant)
- âœ… Conformite LOI-1 : Confirmee (aucune dependance externe)
- âœ… Conformite LOI-2 : Confirmee (isolement gere avec securite par defaut)
- âœ… Conformite LOI-4 : Confirmee (pas de temps global requis)
- âœ… Aucune decision de securite par LogisticsSteward : Confirmee (INV-LS-WS-4)
- âœ… Adaptation obligatoire : Confirmee (INV-LS-WS-3)
- âœ… Tracabilite complete : Confirmee (INV-LS-WS-7)

**Conclusion :** Aucune contradiction detectee. Le document est coherent et non ambigu.

---

*Aucune autre erreur, warning, ou ambiguite rencontree lors de la redaction de ce document.*


