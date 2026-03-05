# WorrySentinel - TAMR Integration Contract

## 1. Contexte

Ce document definit le **contrat d'integration entre WorrySentinel et TAMR**. Il specifie l'interface, le protocole, les regles de communication, et les garanties associees a l'integration avec TAMR en tant que Human Interaction Core de l'ecosysteme Miyukini.

Ce document complete la Section 9 de la [WorrySentinel - Documentation Fondatrice](../../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [TAMR - Documentation Fondatrice](../../../TAMR/foundation/TAMR%20-%20Documentation%20Fondatrice.md) pour la nature de TAMR
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) pour la conformite LOI-1 a LOI-6

L'integration respecte les Lois d'Autonomie Systeme : toutes les adaptations sont locales et ne requierent aucune dependance externe (**LOI-1**).

## 2. Portee / Scope

Ce document couvre :
- L'interface contractuelle entre WorrySentinel et TAMR
- Le protocole de communication (contraintes descendantes et signalements montants)
- L'impact des etats de confiance sur les interventions humaines
- L'adaptation des types d'intervention selon les niveaux de securite
- Les limites d'autorite conditionnelles selon les etats de confiance
- Les regles d'integration specifiques
- Les garanties de l'integration

Ce document **ne couvre pas** :
- Les details internes de TAMR (voir documentation TAMR)
- Les details internes de WorrySentinel (voir Architecture)
- L'interface utilisateur des interventions (responsabilite des produits)
- Les decisions d'autorisation des interventions (responsabilite de StrongFather)
- La persistance des traces d'intervention (responsabilite de KindMother)

---

## 3. Principe fondamental

**WorrySentinel gouverne les niveaux de securite et les etats de confiance. TAMR adapte les regles d'intervention humaine en consequence. WorrySentinel ne decide jamais si une intervention est autorisee, TAMR ne gouverne jamais les etats de securite.**

La relation est de **contrainte verticale** : WorrySentinel impose des contraintes qui modifient les conditions et les limites des interventions humaines. TAMR informe WorrySentinel des patterns d'intervention qui pourraient signaler une anomalie de securite.

---

## 4. Nature de la relation WorrySentinel â€” TAMR

### 4.1 Relation de contrainte verticale

**WorrySentinel contraint TAMR par :**
- L'etat de confiance global (T0-T4) qui modifie les types d'intervention autorises
- Les niveaux de securite (0-4) qui definissent les limites d'autorite par contexte
- Les alertes de durcissement qui restreignent temporairement certaines interventions
- Les exigences de tracabilite renforcee selon l'etat de confiance

**TAMR informe WorrySentinel par :**
- Les patterns d'override suspects (frequence anormale, justifications insuffisantes)
- Les escalades non resolues qui pourraient indiquer une paralysie decisionnelle
- Les tentatives d'intervention sur des zones protegees
- Les anomalies de comportement des intervenants humains

**Regle WS-TAMR-01 : Contrainte sans decision**

WorrySentinel impose des contraintes de securite sur les interventions humaines. WorrySentinel ne decide jamais si une intervention specifique est autorisee. Cette decision appartient exclusivement a StrongFather.

**Regle WS-TAMR-02 : Adaptation obligatoire**

TAMR doit adapter ses regles d'intervention selon les etats de confiance et les niveaux de securite gouvernes par WorrySentinel. L'adaptation n'est pas facultative.

**Regle WS-TAMR-03 : Observation sans modification**

WorrySentinel observe les patterns d'intervention humaine sans jamais modifier les regles de TAMR directement. L'observation alimente la detection d'anomalies.

### 4.2 Separation des responsabilites

| Responsabilite | WorrySentinel | TAMR |
|----------------|---------------|------|
| **Gouverner les etats de confiance** | âœ… Exclusif | âŒ Consomme |
| **Gouverner les niveaux de securite** | âœ… Exclusif | âŒ Consomme |
| **Definir les types d'intervention** | âŒ Contraint | âœ… Exclusif |
| **Definir les limites d'autorite** | âŒ Contraint | âœ… Exclusif |
| **Definir les points d'intervention** | âŒ Contraint | âœ… Exclusif |
| **Decider si une intervention est autorisee** | âŒ Jamais | âŒ Jamais (StrongFather) |
| **Observer les patterns d'intervention** | âœ… Source | âŒ Source |
| **Detecter les anomalies d'intervention** | âœ… Consomme | âŒ Source |

**Regle WS-TAMR-04 : Aucun chevauchement**

Aucun chevauchement de responsabilites n'est autorise. WorrySentinel ne definit jamais de type d'intervention, TAMR ne gouverne jamais les etats de confiance.

---

## 5. Ce que WorrySentinel ne fait JAMAIS vis-a-vis de TAMR

### 5.1 Interdictions absolues

**INV-WS-TAMR-NEVER-1 : Ne definit jamais les types d'intervention**

WorrySentinel ne definit **jamais** de nouveau type d'intervention humaine. Les quatre types (Approval, Override, Escalation, Supervision) sont exclusivement definis par TAMR.

**INV-WS-TAMR-NEVER-2 : Ne decide jamais d'autoriser ou refuser une intervention**

WorrySentinel ne decide **jamais** si une intervention specifique est autorisee. Cette decision appartient a StrongFather selon les politiques applicables.

**INV-WS-TAMR-NEVER-3 : Ne definit jamais les interfaces utilisateur**

WorrySentinel ne definit **jamais** les interfaces de presentation des interventions. Cette responsabilite appartient aux produits.

**INV-WS-TAMR-NEVER-4 : Ne modifie jamais les limites infranchissables**

WorrySentinel ne peut **jamais** modifier les limites infranchissables definies par TAMR (INV-TAMR-3). Ces limites sont absolues et independantes des etats de confiance.

**INV-WS-TAMR-NEVER-5 : Ne supprime jamais la tracabilite**

WorrySentinel ne peut **jamais** supprimer ou reduire les exigences de tracabilite de TAMR (INV-TAMR-1). Il peut uniquement les renforcer.

**INV-WS-TAMR-NEVER-6 : Ne contourne jamais l'observation**

WorrySentinel ne contourne **jamais** les mecanismes de tracabilite de TAMR. Les observations sont basees sur les traces produites normalement.

---

## 6. Impact des etats de confiance sur les interventions humaines

### 6.1 Adaptation des interventions par etat de confiance

WorrySentinel gouverne les etats de confiance (T0-T4). TAMR adapte les regles d'intervention humaine en consequence :

**T0 â€” Normal**

| Type d'intervention | Disponibilite | Contraintes |
|---------------------|---------------|-------------|
| **Approval** | âœ… Disponible | Conditions normales |
| **Override** | âœ… Disponible | Justification requise (INV-TAMR-7) |
| **Escalation** | âœ… Disponible | Chemin standard |
| **Supervision** | âœ… Disponible | Passive par defaut |

| Aspect | Comportement |
|--------|--------------|
| **Tracabilite** | Tracabilite standard (INV-TAMR-1) |
| **Limites** | Limites normales, limites infranchissables toujours actives |
| **Exigences** | Exigences de justification normales |
| **Delais** | Delais standards pour escalade |

**T1 â€” Instable**

| Type d'intervention | Disponibilite | Contraintes |
|---------------------|---------------|-------------|
| **Approval** | âœ… Disponible | Tracabilite etendue |
| **Override** | âœ… Disponible | Justification detaillee requise |
| **Escalation** | âœ… Disponible | Chemin standard avec notification |
| **Supervision** | âœ… Disponible | Surveillance renforcee |

| Aspect | Comportement |
|--------|--------------|
| **Tracabilite** | Tracabilite etendue (contexte de securite ajoute) |
| **Limites** | Limites normales, alertes actives |
| **Exigences** | Justification plus detaillee pour override |
| **Delais** | Delais standards |

**T2 â€” Degrade**

| Type d'intervention | Disponibilite | Contraintes |
|---------------------|---------------|-------------|
| **Approval** | âœ… Disponible | Approbations sensibles suspendues |
| **Override** | âš ï¸ Restreint | Override limite aux cas critiques |
| **Escalation** | âœ… Disponible | Escalade acceleree |
| **Supervision** | âœ… Obligatoire | Supervision active requise |

| Aspect | Comportement |
|--------|--------------|
| **Tracabilite** | Tracabilite renforcee (audit immediat) |
| **Limites** | Limites renforcees pour operations sensibles |
| **Exigences** | Justification substantielle obligatoire |
| **Delais** | Delais reduits pour escalade |

**T3 â€” Restreint**

| Type d'intervention | Disponibilite | Contraintes |
|---------------------|---------------|-------------|
| **Approval** | âš ï¸ Restreint | Seules approbations critiques |
| **Override** | âš ï¸ Validation requise | **Override necessite validation TAMR explicite** |
| **Escalation** | âœ… Prioritaire | Escalade prioritaire vers niveau superieur |
| **Supervision** | âœ… Obligatoire | Supervision permanente |

| Aspect | Comportement |
|--------|--------------|
| **Tracabilite** | Tracabilite exhaustive (justification obligatoire) |
| **Limites** | Limites maximales, operations gelees |
| **Exigences** | **Override = validation TAMR + justification complete** |
| **Delais** | Delais minimaux, timeout strict sur escalade |

**REGLE CRITIQUE T3 :**

En etat T3 (Restreint), tout override necessite une **validation TAMR explicite**. Cette validation confirme que l'override est bien une intervention humaine deliberee et non une manipulation automatisee.

**T4 â€” Bloque**

| Type d'intervention | Disponibilite | Contraintes |
|---------------------|---------------|-------------|
| **Approval** | â›” Suspendu | Aucune approbation operationnelle |
| **Override** | â›” Bloque | Aucun override autorise |
| **Escalation** | âš ï¸ Urgence | Escalade d'urgence uniquement |
| **Supervision** | âœ… Lecture seule | Observation sans intervention |

| Aspect | Comportement |
|--------|--------------|
| **Tracabilite** | Tracabilite minimale (preservation ressources) |
| **Limites** | Blocage total des interventions operationnelles |
| **Exigences** | Aucune intervention sauf diagnostique |
| **Delais** | Systeme en attente de restauration |

**Regle WS-TAMR-STATE-01 : Adaptation immediate**

L'adaptation des regles d'intervention a un changement d'etat de confiance est immediate. Aucun delai n'est autorise.

**Regle WS-TAMR-STATE-02 : Preservation des limites infranchissables**

Quel que soit l'etat de confiance, les limites infranchissables de TAMR (INV-TAMR-3) restent actives. Un etat T0 ne permet pas de franchir ces limites.

### 6.2 Niveaux de securite et limites d'autorite

WorrySentinel attribue des niveaux de securite (0-4) aux produits et composants. TAMR adapte les limites d'autorite :

| Niveau | Impact sur les interventions |
|--------|------------------------------|
| **Niveau 0 - Public** | Interventions standards, limites minimales |
| **Niveau 1 - Standard** | Interventions standards, limites normales |
| **Niveau 2 - Sensitive** | Interventions avec tracabilite renforcee, limites elevees |
| **Niveau 3 - Critical** | Interventions restreintes, limites strictes, escalade requise |
| **Niveau 4 - Highest** | Interventions minimales, limites maximales, validation multiple |

**Regle WS-TAMR-SEC-01 : Respect des niveaux**

Les limites d'autorite sont adaptees au niveau de securite. Un produit de niveau 4 requiert des validations supplementaires pour tout override.

**Regle WS-TAMR-SEC-02 : Cumul etat-niveau**

Les restrictions sont cumulatives : une intervention sur un produit de niveau 3 en etat T2 subit les restrictions de T2 + les contraintes du niveau 3.

---

## 7. Types d'informations echangees

### 7.1 Flux descendant : WorrySentinel â†’ TAMR

**TRUST_STATE_CHANGE**
- **Objectif :** Notifier un changement d'etat de confiance
- **Contenu :** Nouvel etat (T0-T4), justification, timestamp
- **Impact :** Adaptation immediate des regles d'intervention

**Structure du changement d'etat :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `notification_id` | Identifiant unique de la notification | âœ… Oui |
| `previous_state` | Etat de confiance precedent (T0-T4) | âœ… Oui |
| `new_state` | Nouvel etat de confiance (T0-T4) | âœ… Oui |
| `transition_reason` | Justification de la transition | âœ… Oui |
| `timestamp` | Horodatage de la transition | âœ… Oui |
| `intervention_constraints` | Contraintes sur les types d'intervention | âŒ Optionnel |

**INTERVENTION_CONSTRAINT**
- **Objectif :** Imposer des contraintes specifiques sur les interventions
- **Contenu :** Type d'intervention concerne, nature de la contrainte, duree
- **Impact :** Restriction ou renforcement de certaines interventions

**Structure de la contrainte d'intervention :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `constraint_id` | Identifiant unique de la contrainte | âœ… Oui |
| `intervention_type` | Type d'intervention concerne (approval, override, escalation, supervision) | âœ… Oui |
| `constraint_nature` | Nature (block, restrict, require_validation, enhance_tracing) | âœ… Oui |
| `scope` | Portee (all, security_level, product, context) | âœ… Oui |
| `duration` | Duree (null = indefini) | âŒ Optionnel |
| `justification` | Raison de la contrainte | âœ… Oui |
| `timestamp` | Horodatage de la contrainte | âœ… Oui |

**TRACING_DIRECTIVE**
- **Objectif :** Modifier le niveau de tracabilite requis
- **Contenu :** Niveau de tracabilite, elements a tracer, duree
- **Impact :** Renforcement de la tracabilite des interventions

**Structure de la directive de tracabilite :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `directive_id` | Identifiant unique de la directive | âœ… Oui |
| `tracing_level` | Niveau (standard, extended, reinforced, exhaustive) | âœ… Oui |
| `additional_elements` | Elements supplementaires a tracer | âŒ Optionnel |
| `duration` | Duree (null = indefini) | âŒ Optionnel |
| `justification` | Raison de la directive | âœ… Oui |
| `timestamp` | Horodatage de la directive | âœ… Oui |

**OVERRIDE_VALIDATION_REQUIRED**
- **Objectif :** Signaler que les overrides necessitent une validation TAMR explicite (etat T3)
- **Contenu :** Activation/desactivation, justification
- **Impact :** Processus de validation supplementaire pour tout override

**Structure de l'exigence de validation override :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `requirement_id` | Identifiant unique de l'exigence | âœ… Oui |
| `active` | Si l'exigence est active | âœ… Oui |
| `validation_type` | Type de validation requise (identity_confirm, challenge, dual_approval) | âœ… Oui |
| `justification` | Raison de l'exigence | âœ… Oui |
| `timestamp` | Horodatage de l'exigence | âœ… Oui |

### 7.2 Flux montant : TAMR â†’ WorrySentinel

**INTERVENTION_PATTERN_ALERT**
- **Objectif :** Alerter sur un pattern d'intervention suspect
- **Contenu :** Nature du pattern, intervenant concerne, contexte
- **Usage :** WorrySentinel evalue si une action de securite est necessaire

**Structure de l'alerte de pattern :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `alert_id` | Identifiant unique de l'alerte | âœ… Oui |
| `pattern_type` | Type de pattern (excessive_override, insufficient_justification, rapid_succession, protected_zone_attempt) | âœ… Oui |
| `involved_interventions` | Liste des interventions concernees | âœ… Oui |
| `involved_intervenants` | Intervenants concernes (identites) | âœ… Oui |
| `context` | Contexte du pattern | âœ… Oui |
| `severity_assessment` | Evaluation de gravite par TAMR | âœ… Oui |
| `timestamp` | Horodatage de l'alerte | âœ… Oui |

**ESCALATION_STALL_ALERT**
- **Objectif :** Alerter sur une escalade non resolue
- **Contenu :** Escalade concernee, duree, impact
- **Usage :** WorrySentinel peut decider d'une action de securite si paralysie detectee

**Structure de l'alerte d'escalade bloquee :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `alert_id` | Identifiant unique de l'alerte | âœ… Oui |
| `escalation_id` | Reference a l'escalade concernee | âœ… Oui |
| `stall_duration` | Duree de blocage | âœ… Oui |
| `escalation_level` | Niveau d'escalade atteint | âœ… Oui |
| `impact_assessment` | Evaluation de l'impact | âœ… Oui |
| `timestamp` | Horodatage de l'alerte | âœ… Oui |

**PROTECTED_ZONE_ATTEMPT**
- **Objectif :** Signaler une tentative d'intervention sur une zone protegee
- **Contenu :** Zone concernee, intervention tentee, intervenant
- **Usage :** WorrySentinel peut durcir les contraintes si tentatives repetees

**Structure du signalement de tentative :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `attempt_id` | Identifiant unique de la tentative | âœ… Oui |
| `protected_zone` | Zone protegee concernee | âœ… Oui |
| `attempted_intervention` | Type d'intervention tentee | âœ… Oui |
| `intervenant_id` | Identite de l'intervenant | âœ… Oui |
| `outcome` | Resultat (blocked, rejected) | âœ… Oui |
| `timestamp` | Horodatage de la tentative | âœ… Oui |

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

**Regle WS-TAMR-PROT-01 : Traitement immediat**

Toutes les notifications descendantes de WorrySentinel sont traitees immediatement. Aucun delai n'est autorise.

### 8.2 Format des signalements montants

Les signalements de TAMR suivent un format standardise.

**Structure de base :**

| Element | Description | Obligatoire |
|---------|-------------|-------------|
| `signal_id` | Identifiant unique du signalement | âœ… Oui |
| `type` | Type de signalement | âœ… Oui |
| `payload` | Donnees specifiques au signalement | âœ… Oui |
| `timestamp` | Horodatage du signalement | âœ… Oui |
| `urgency` | Niveau d'urgence (low, medium, high, critical) | âœ… Oui |

**Regle WS-TAMR-PROT-02 : Signalement non bloquant**

Les signalements montants sont non bloquants. TAMR continue son fonctionnement apres l'envoi.

### 8.3 Confirmations et acquittements

**Regle WS-TAMR-PROT-03 : Acquittement obligatoire**

TAMR acquitte toutes les notifications descendantes avec `requires_ack: true`.

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

**Acteurs :** WorrySentinel, TAMR

**Sequence :**

1. WorrySentinel detecte une anomalie et decide de passer de T0 a T1
2. WorrySentinel envoie `TRUST_STATE_CHANGE` a TAMR
3. TAMR recoit la notification
4. TAMR adapte immediatement ses regles d'intervention (tracabilite etendue)
5. TAMR acquitte avec `ACK_OK` et `adaptation_applied: true`
6. Les interventions futures sont soumises aux nouvelles regles

### 9.2 Flux de passage en etat T3 (override avec validation)

**Acteurs :** WorrySentinel, TAMR, Intervenant humain, StrongFather

**Sequence :**

1. WorrySentinel detecte une menace et decide de passer en T3
2. WorrySentinel envoie `TRUST_STATE_CHANGE` a TAMR
3. WorrySentinel envoie `OVERRIDE_VALIDATION_REQUIRED` a TAMR
4. TAMR active le mode "override avec validation explicite"
5. Un intervenant demande un override
6. TAMR declenche la validation TAMR (challenge, confirmation identite)
7. Si validation reussie, StrongFather evalue si l'override est autorise
8. Toute l'operation est tracee avec contexte de securite complet

### 9.3 Flux de signalement de pattern suspect

**Acteurs :** TAMR, WorrySentinel

**Sequence :**

1. TAMR detecte un pattern d'override suspect (frequence elevee sur un produit)
2. TAMR genere un `INTERVENTION_PATTERN_ALERT`
3. TAMR envoie l'alerte a WorrySentinel
4. WorrySentinel recoit et analyse l'alerte
5. WorrySentinel peut decider de durcir les contraintes ou surveiller

### 9.4 Diagramme de sequence

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  WorrySentinel  â”‚    â”‚      TAMR       â”‚    â”‚  Intervenant   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚                      â”‚                     â”‚
         â”œâ”€â”€ TRUST_STATE_CHANGE â”€â–ºâ”‚                     â”‚
         â”‚    (T0 â†’ T3)         â”‚                     â”‚
         â”‚                      â”‚                     â”‚
         â”‚                      â”œâ”€â”€ Adaptation        â”‚
         â”‚                      â”‚   (mode T3)         â”‚
         â”‚                      â”‚                     â”‚
         â”œâ”€â”€ OVERRIDE_VALIDATION â”‚                     â”‚
         â”‚    _REQUIRED â”€â”€â”€â”€â”€â”€â”€â”€â–ºâ”‚                     â”‚
         â”‚                      â”‚                     â”‚
         â”‚â—„â”€â”€ ACK_OK â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤                     â”‚
         â”‚                      â”‚                     â”‚
         â”‚                      â”‚â—„â”€â”€ Demande override â”¤
         â”‚                      â”‚                     â”‚
         â”‚                      â”œâ”€â”€ Challenge â”€â”€â”€â”€â”€â”€â”€â”€â–ºâ”‚
         â”‚                      â”‚                     â”‚
         â”‚                      â”‚â—„â”€â”€ Reponse â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
         â”‚                      â”‚                     â”‚
         â”‚                      â”œâ”€â”€ Validation OK     â”‚
         â”‚                      â”‚   â†’ StrongFather    â”‚
         â”‚                      â”‚                     â”‚
         â”‚â—„â”€â”€ Pattern alert â”€â”€â”€â”€â”¤                     â”‚
         â”‚    (si suspect)      â”‚                     â”‚
         â”‚                      â”‚                     â”‚
```

---

## 10. Regles d'integration

### 10.1 Regles de communication

**Regle WS-TAMR-INT-01 : Bidirectionnel asymetrique**

La communication est bidirectionnelle mais asymetrique. WorrySentinel impose des contraintes, TAMR signale des patterns. WorrySentinel ne repond pas aux signalements de TAMR.

**Regle WS-TAMR-INT-02 : Priorite aux contraintes de securite**

Les contraintes de WorrySentinel sont prioritaires sur les regles standards de TAMR. Aucune regle de TAMR ne peut contredire une contrainte de securite.

**Regle WS-TAMR-INT-03 : Signalements non bloquants**

Les signalements de TAMR sont toujours non bloquants. L'envoi n'attend jamais de reponse.

### 10.2 Regles de donnees

**Regle WS-TAMR-INT-04 : Donnees de gouvernance uniquement**

Les donnees echangees sont des informations de gouvernance (etats, contraintes, alertes), jamais des donnees metier ou personnelles detaillees.

**Regle WS-TAMR-INT-05 : Identites minimales**

Les identites d'intervenants sont transmises de maniere minimale (identifiant), jamais avec details personnels.

**Regle WS-TAMR-INT-06 : Coherence garantie**

WorrySentinel garantit la coherence de ses notifications. TAMR peut se fier aux etats et contraintes communiques.

### 10.3 Regles de tracabilite

**Regle WS-TAMR-INT-07 : Tracabilite complete**

Toutes les interactions sont tracees avec contexte complet.

**Regle WS-TAMR-INT-08 : Correlation possible**

Chaque notification peut etre correlee aux adaptations d'intervention qui en decoulent.

**Regle WS-TAMR-INT-09 : Renforcement uniquement**

WorrySentinel peut uniquement renforcer la tracabilite, jamais la reduire.

---

## 11. Gestion des erreurs

### 11.1 Types d'erreurs

**Erreurs de format :**
- Notification mal formee
- Champ obligatoire manquant
- Type de notification inconnu

**Erreurs d'application :**
- Type d'intervention inconnu dans une contrainte
- Niveau de tracabilite invalide
- Duree de contrainte invalide

**Erreurs internes :**
- Erreur lors de l'adaptation des regles
- Erreur de journalisation

### 11.2 Traitement des erreurs

**Regle WS-TAMR-ERR-01 : Acquittement avec erreur**

En cas d'erreur, TAMR acquitte avec `ACK_ERROR` et description du probleme.

**Regle WS-TAMR-ERR-02 : Application partielle**

Si une adaptation partielle est possible, TAMR l'applique et acquitte avec `ACK_PARTIAL`.

**Regle WS-TAMR-ERR-03 : Journalisation des erreurs**

Toutes les erreurs sont journalisees pour audit et diagnostic.

**Regle WS-TAMR-ERR-04 : Securite par defaut**

En cas d'erreur de communication avec WorrySentinel, TAMR applique le comportement le plus restrictif (principe de securite par defaut).

---

## 12. Cas particuliers

### 12.1 Etat de confiance T4 (Bloque)

En etat T4, les interventions humaines operationnelles sont bloquees :

**Regle WS-TAMR-CASE-01 : Mode urgence**

En T4, seules les escalades d'urgence sont possibles. Approval et Override sont bloques. La supervision est en lecture seule.

### 12.2 Override en T3

Si un override est demande en etat T3 :

**Regle WS-TAMR-CASE-02 : Validation TAMR obligatoire**

Tout override en T3 necessite une validation TAMR explicite (challenge, confirmation identite, dual approval selon configuration). Cette validation est en plus de l'autorisation StrongFather.

### 12.3 Escalade non resolue

Si une escalade n'est pas resolue dans le delai :

**Regle WS-TAMR-CASE-03 : Timeout avec signalement**

TAMR applique INV-TAMR-8 (escalade non bloquante) ET signale a WorrySentinel via `ESCALATION_STALL_ALERT`.

### 12.4 Limites infranchissables

Independamment de l'etat de confiance :

**Regle WS-TAMR-CASE-04 : Limites absolues preservees**

Les limites infranchissables de TAMR (INV-TAMR-3) restent actives meme en T0. WorrySentinel ne peut pas les modifier.

### 12.5 Transition d'etat pendant une intervention

Si l'etat de confiance change pendant une intervention en cours :

**Regle WS-TAMR-CASE-05 : Reevaluation immediate**

L'intervention en cours est reevaluee selon le nouvel etat. Si les nouvelles contraintes l'interdisent, l'intervention est interrompue.

---

## 13. Garanties de l'integration

### 13.1 Garantie de reactivite

**Engagement :** TAMR reagit immediatement aux notifications de WorrySentinel. Aucun delai superieur a une seconde n'est acceptable.

### 13.2 Garantie de conformite

**Engagement :** TAMR applique toujours les contraintes de WorrySentinel. Aucune intervention ne peut contredire une contrainte de securite.

### 13.3 Garantie de transparence

**Engagement :** Les patterns d'intervention sont transparents pour WorrySentinel. Les signalements fournissent une visibilite sur les anomalies.

### 13.4 Garantie de tracabilite

**Engagement :** Toute interaction est traÃ§able de bout en bout. L'audit complet des notifications, adaptations et signalements est possible.

### 13.5 Garantie de disponibilite

**Engagement :** L'integration ne bloque jamais TAMR. En cas de defaillance de WorrySentinel, TAMR applique la securite par defaut.

### 13.6 Garantie de preservation des invariants TAMR

**Engagement :** L'integration respecte tous les invariants de TAMR (INV-TAMR-1 a INV-TAMR-8). Les contraintes de WorrySentinel ne peuvent jamais violer ces invariants.

---

## 14. Invariants de l'integration

### 14.1 Invariants de relation

**INV-WS-TAMR-1 : Contrainte unidirectionnelle**

WorrySentinel contraint TAMR. TAMR ne contraint jamais WorrySentinel.

**INV-WS-TAMR-2 : Observation passive**

WorrySentinel observe passivement. L'observation ne modifie jamais les regles de TAMR directement.

**INV-WS-TAMR-3 : Adaptation obligatoire**

TAMR adapte obligatoirement ses regles aux contraintes de WorrySentinel.

### 14.2 Invariants de donnees

**INV-WS-TAMR-4 : Pas de definition de type d'intervention**

WorrySentinel ne definit aucun type d'intervention. Les types sont exclusivement definis par TAMR.

**INV-WS-TAMR-5 : Signalements informatifs**

Les signalements sont informatifs. Ils n'imposent aucune action a WorrySentinel.

**INV-WS-TAMR-6 : Preservation des limites infranchissables**

WorrySentinel ne peut jamais modifier les limites infranchissables de TAMR.

### 14.3 Invariants de protocole

**INV-WS-TAMR-7 : Format respecte**

Toutes les notifications et signalements respectent le format standardise.

**INV-WS-TAMR-8 : Tracabilite complete**

Toute interaction est traÃ§able avec son contexte complet.

**INV-WS-TAMR-9 : Renforcement tracabilite uniquement**

WorrySentinel peut uniquement renforcer la tracabilite, jamais la reduire.

---

## 15. Conformite aux Lois d'Autonomie Systeme

### LOI-1 : Aucune dependance externe critique

**Conformite :** âœ… **Conforme**

L'integration respecte LOI-1 :
- TAMR adapte ses regles localement
- WorrySentinel gouverne localement
- L'absence de connexion ne bloque ni les interventions ni la gouvernance

### LOI-2 : Le systeme accepte l'isolement comme etat normal

**Conformite :** âœ… **Conforme**

L'integration respecte LOI-2 :
- En isolement, TAMR applique la securite par defaut
- Les adaptations locales restent actives
- INV-TAMR-8 (escalade non bloquante) garantit que les escalades ne bloquent pas en mode isole

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
  "notification_id": "notif-ws-tamr-001",
  "type": "TRUST_STATE_CHANGE",
  "payload": {
    "previous_state": "T0",
    "new_state": "T1",
    "transition_reason": "Anomalie detectee sur les patterns de connexion",
    "intervention_constraints": null
  },
  "timestamp": "2026-01-28T10:00:00Z",
  "requires_ack": true
}
```

**Acquittement TAMR :**
```
{
  "ack_id": "ack-tamr-001",
  "notification_id": "notif-ws-tamr-001",
  "status": "ACK_OK",
  "adaptation_applied": {
    "tracing_level": "extended",
    "override_justification": "detailed",
    "supervision_mode": "reinforced"
  },
  "timestamp": "2026-01-28T10:00:01Z"
}
```

### 16.2 Activation de la validation override (T3)

**Notification WorrySentinel :**
```
{
  "notification_id": "notif-ws-tamr-002",
  "type": "OVERRIDE_VALIDATION_REQUIRED",
  "payload": {
    "requirement_id": "req-001",
    "active": true,
    "validation_type": "identity_confirm",
    "justification": "Passage en etat T3 - Suspicion forte"
  },
  "timestamp": "2026-01-28T11:00:00Z",
  "requires_ack": true
}
```

**Acquittement TAMR :**
```
{
  "ack_id": "ack-tamr-002",
  "notification_id": "notif-ws-tamr-002",
  "status": "ACK_OK",
  "adaptation_applied": {
    "override_validation_active": true,
    "validation_type": "identity_confirm"
  },
  "timestamp": "2026-01-28T11:00:01Z"
}
```

### 16.3 Signalement de pattern suspect

**Signalement TAMR :**
```
{
  "signal_id": "signal-tamr-001",
  "type": "INTERVENTION_PATTERN_ALERT",
  "payload": {
    "alert_id": "alert-001",
    "pattern_type": "excessive_override",
    "involved_interventions": [
      "int-001", "int-002", "int-003", "int-004", "int-005"
    ],
    "involved_intervenants": ["user-admin-42"],
    "context": {
      "time_window": "30 minutes",
      "affected_product": "product-finance",
      "justifications_quality": "low"
    },
    "severity_assessment": "high"
  },
  "timestamp": "2026-01-28T12:00:00Z",
  "urgency": "high"
}
```

---

## 17. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il etablit l'interface et le protocole que WorrySentinel et TAMR doivent respecter pour leur integration.

Toute implementation de l'integration entre WorrySentinel et TAMR doit respecter ce contrat. Toute violation entraine un comportement non conforme.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif  
**Dependances :**
- WorrySentinel - Documentation Fondatrice v1.2 (Section 9)
- TAMR - Documentation Fondatrice v1.4
- Miyukini Conceptual References - Lois Autonomie Systeme v1.1

---

## 18. Mini log de generation

### Decision editoriale E1 : Direction de la relation

**Decision prise :** La relation est de contrainte verticale : WorrySentinel impose des contraintes sur les interventions humaines, TAMR adapte ses regles en consequence. Cette direction respecte la position de WorrySentinel comme "pression verticale" (Strate 4) sur les cores fonctionnels (Strate 5).

**Application :** Tout le document est structure autour de cette relation de contrainte descendante.

### Decision editoriale E2 : Validation TAMR en T3

**Decision prise :** En etat T3, tout override necessite une validation TAMR explicite. Cette regle specifique est mentionnee dans la Documentation Fondatrice de WorrySentinel ("T3 â€” TAMR requis pour override").

**Application :** Section 6.1 (T3), Regle WS-TAMR-CASE-02, et Flux 9.2 etablissent cette exigence.

### Decision editoriale E3 : Preservation des invariants TAMR

**Decision prise :** Les invariants de TAMR (notamment INV-TAMR-3 limites infranchissables) sont preserves quel que soit l'etat de confiance. WorrySentinel peut contraindre mais jamais violer les fondements de TAMR.

**Application :** INV-WS-TAMR-NEVER-4, INV-WS-TAMR-6, Regle WS-TAMR-STATE-02.

### Warning W1 : Risque de confusion contrainte/decision

**Warning rencontre :** Risque que WorrySentinel soit percu comme decidant si une intervention est autorisee.

**Decision prise :** Clarification explicite que WorrySentinel contraint les conditions et limites, mais StrongFather decide de l'autorisation. INV-WS-TAMR-NEVER-2 etablit cette separation.

**Correction effectuee :** Sections 4, 5, et 10 redigees avec cette distinction explicite.

### Warning W2 : Renforcement vs modification de tracabilite

**Warning rencontre :** Risque que WorrySentinel puisse reduire la tracabilite de TAMR.

**Decision prise :** WorrySentinel peut uniquement renforcer la tracabilite, jamais la reduire. INV-WS-TAMR-NEVER-5 et INV-WS-TAMR-9 etablissent cette limite.

**Correction effectuee :** Regle WS-TAMR-INT-09 ajoutee.

### Verification de coherence

**Verification effectuee :**
- âœ… Coherence avec WorrySentinel - Documentation Fondatrice : Confirmee (flux descendant TAMR, T3 override)
- âœ… Coherence avec TAMR - Documentation Fondatrice : Confirmee (types d'intervention, invariants)
- âœ… Conformite LOI-1 : Confirmee (aucune dependance externe)
- âœ… Conformite LOI-2 : Confirmee (isolement gere avec securite par defaut)
- âœ… Conformite LOI-4 : Confirmee (pas de temps global requis)
- âœ… Aucune decision d'autorisation par WorrySentinel : Confirmee (INV-WS-TAMR-NEVER-2)
- âœ… Preservation des invariants TAMR : Confirmee (INV-WS-TAMR-6)
- âœ… Tracabilite complete : Confirmee (INV-WS-TAMR-8)

**Conclusion :** Aucune contradiction detectee. Le document est coherent et non ambigu.

---

*Aucune autre erreur, warning, ou ambiguite rencontree lors de la redaction de ce document.*

