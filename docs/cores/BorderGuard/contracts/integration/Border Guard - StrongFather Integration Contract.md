# Border Guard - StrongFather Integration Contract

## 1. Contexte

Ce document dÃ©finit le **contrat d'intÃ©gration entre Border Guard et StrongFather**. Il spÃ©cifie l'interface, le protocole, les rÃ¨gles de communication, et les garanties associÃ©es Ã  l'intÃ©gration avec StrongFather en tant qu'autoritÃ© des dÃ©cisions stratÃ©giques et politiques.

Ce document complÃ¨te la Section 8 de la [Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [StrongFather - Documentation Fondatrice](../../../StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md) pour la nature de StrongFather
- [Miyukini Conceptual References - Security Protocols](..//..//..//..//miyukini-webway-system//reference//_index.md) pour les protocoles de sÃ©curitÃ© temps rÃ©el et asynchrone
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) pour la conformitÃ© LOI-1 Ã  LOI-6

L'intÃ©gration respecte les Lois d'Autonomie SystÃ¨me : toutes les dÃ©finitions de frontiÃ¨res sont locales et ne requiÃ¨rent aucune dÃ©pendance externe (**LOI-1**).

## 2. PortÃ©e / Scope

Ce document couvre :
- L'interface contractuelle entre Border Guard et StrongFather
- Le protocole de communication (consultation de contexte de frontiÃ¨re)
- Les types d'informations Ã©changÃ©es
- Les rÃ¨gles d'intÃ©gration spÃ©cifiques
- La gestion des erreurs et des rÃ©ponses
- Les garanties de l'intÃ©gration

Ce document **ne couvre pas** :
- Les dÃ©tails internes de StrongFather (voir documentation StrongFather)
- Les dÃ©tails internes du moteur de dÃ©finition de frontiÃ¨res (voir Architecture)
- L'intÃ©gration avec BondingBrother (voir BondingBrother Integration Contract)
- L'intÃ©gration avec Caring Nanny (voir CaringNanny Integration Contract)

---

## 3. Principe fondamental

**Border Guard fournit Ã  StrongFather le contexte de confiance et de frontiÃ¨re pour enrichir l'Ã©valuation des intentions. StrongFather consulte Border Guard pour connaÃ®tre le niveau de confiance d'une source et les rÃ¨gles de franchissement applicables. Border Guard ne participe jamais Ã  la dÃ©cision elle-mÃªme.**

La relation est de **conseil** : Border Guard informe StrongFather sur le contexte de frontiÃ¨re ; StrongFather dÃ©cide en tenant compte de cette information. Cette relation est unidirectionnelle en termes de flux dÃ©cisionnel : Border Guard informe, StrongFather dÃ©cide.

---

## 4. Nature de la relation Border Guard â€” StrongFather

### 4.1 Relation de conseil

**Border Guard informe StrongFather de :**
- Le niveau de confiance de la source d'une intention (trusted, verified, unknown, hostile)
- La nature de la frontiÃ¨re traversÃ©e par l'intention (externe, interne, intÃ©gration)
- Les rÃ¨gles de franchissement applicables Ã  cette frontiÃ¨re
- L'Ã©tat de l'intÃ©gration concernÃ©e (si applicable)

**StrongFather consulte Border Guard pour :**
- Contextualiser une intention avec son niveau de confiance
- ConnaÃ®tre les rÃ¨gles de franchissement avant Ã©valuation
- IntÃ©grer la classification de source dans la dÃ©cision

**RÃ¨gle BG-SF-01 : Conseil sans dÃ©cision**

Border Guard ne participe jamais aux dÃ©cisions de StrongFather. Il fournit des informations de classification et de rÃ¨gles, sans recommandation, sans interprÃ©tation dÃ©cisionnelle, sans jugement sur la validitÃ© de l'intention.

**RÃ¨gle BG-SF-02 : Consultation facultative**

StrongFather peut consulter Border Guard, mais n'est pas obligÃ© de le faire. La dÃ©cision d'intÃ©grer le contexte de frontiÃ¨re dans une Ã©valuation appartient Ã  StrongFather.

**RÃ¨gle BG-SF-03 : Aucune influence sur le rÃ©sultat**

Le contexte de frontiÃ¨re fourni par Border Guard n'influence jamais directement le rÃ©sultat d'une Ã©valuation. StrongFather utilise ce contexte comme information, mais la dÃ©cision reste entiÃ¨rement sous son autoritÃ© selon ses politiques.

### 4.2 SÃ©paration des responsabilitÃ©s

| ResponsabilitÃ© | Border Guard | StrongFather |
|----------------|--------------|--------------|
| **DÃ©finir les frontiÃ¨res** | âœ… Exclusif | âŒ Jamais |
| **Classifier les niveaux de confiance** | âœ… Exclusif | âŒ Consomme |
| **Ã‰tablir les rÃ¨gles de franchissement** | âœ… Exclusif | âŒ Consomme |
| **DÃ©cider si autorisÃ©** | âŒ Jamais | âœ… Exclusif |
| **Appliquer des politiques** | âŒ Jamais | âœ… Exclusif |
| **Ã‰valuer des intentions** | âŒ Jamais | âœ… Exclusif |
| **Modifier l'Ã©tat** | âŒ Jamais | âŒ Jamais |
| **Fournir le contexte frontiÃ¨re** | âœ… Exclusif | âŒ Consomme |

**RÃ¨gle BG-SF-04 : Aucun chevauchement**

Aucun chevauchement de responsabilitÃ©s n'est autorisÃ©. Border Guard ne prend jamais de dÃ©cision, StrongFather ne dÃ©finit jamais de frontiÃ¨re ou de niveau de confiance.

---

## 5. Ce que Border Guard ne fait JAMAIS vis-Ã -vis de StrongFather

### 5.1 Interdictions absolues

**INV-BG-SF-NEVER-1 : Ne prend jamais de dÃ©cision**

Border Guard ne prend **jamais** de dÃ©cision basÃ©e sur les classifications effectuÃ©es. Si une source est classifiÃ©e `hostile`, Border Guard informe, mais ne dÃ©cide pas de bloquer ou d'autoriser quoi que ce soit.

**INV-BG-SF-NEVER-2 : Ne modifie jamais une politique**

Border Guard ne modifie **jamais** une politique ou une contrainte de StrongFather. Les politiques appartiennent exclusivement Ã  StrongFather.

**INV-BG-SF-NEVER-3 : Ne refuse jamais une intention**

Border Guard ne refuse **jamais** et n'accepte **jamais** une intention. L'acceptation ou le refus est la prÃ©rogative exclusive de StrongFather.

**INV-BG-SF-NEVER-4 : N'influence jamais le rÃ©sultat**

Border Guard n'influence **jamais** le rÃ©sultat d'une Ã©valuation de StrongFather. Il fournit un contexte de classification, mais le rÃ©sultat est dÃ©terminÃ© uniquement par StrongFather selon ses politiques.

**INV-BG-SF-NEVER-5 : Ne recommande jamais**

Border Guard ne fournit **jamais** de recommandation Ã  StrongFather. Il rapporte des classifications (niveaux de confiance, rÃ¨gles de franchissement), pas des conseils ou des suggestions de dÃ©cision.

**INV-BG-SF-NEVER-6 : N'exÃ©cute jamais**

Border Guard n'exÃ©cute **jamais** d'action. Il dÃ©finit les rÃ¨gles de franchissement, mais l'application de ces rÃ¨gles appartient Ã  BondingBrother ou aux autres cores opÃ©rationnels, jamais Ã  Border Guard.

---

## 6. Types d'informations Ã©changÃ©es

### 6.1 Information de contexte de frontiÃ¨re

**BOUNDARY_CONTEXT**
- **Objectif :** Fournir le contexte de frontiÃ¨re pour une intention
- **Contenu :** FrontiÃ¨re(s) traversÃ©e(s), niveau de confiance de la source, rÃ¨gles applicables
- **FrÃ©quence :** Sur demande de StrongFather

**Structure du contexte de frontiÃ¨re :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `context_id` | Identifiant unique du contexte | âœ… Oui |
| `source_trust_level` | Niveau de confiance de la source (trusted, verified, unknown, hostile) | âœ… Oui |
| `boundaries_crossed` | Liste des frontiÃ¨res traversÃ©es | âœ… Oui |
| `crossing_rules` | RÃ¨gles de franchissement applicables | âœ… Oui |
| `integration_state` | Ã‰tat de l'intÃ©gration concernÃ©e (si applicable) | âŒ Optionnel |
| `timestamp` | Horodatage de la classification | âœ… Oui |

### 6.2 Information de niveau de confiance

**TRUST_LEVEL_INFO**
- **Objectif :** Fournir le niveau de confiance d'une source spÃ©cifique
- **Contenu :** Niveau de confiance, critÃ¨res appliquÃ©s, historique de classification
- **Usage :** Enrichissement du contexte dÃ©cisionnel

**Structure du niveau de confiance :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `trust_level` | Niveau (trusted, verified, unknown, hostile) | âœ… Oui |
| `criteria_applied` | CritÃ¨res ayant dÃ©terminÃ© la classification | âœ… Oui |
| `source_identifier` | Identifiant de la source classifiÃ©e | âœ… Oui |
| `classification_date` | Date de la classification | âœ… Oui |
| `previous_level` | Niveau prÃ©cÃ©dent (si transition) | âŒ Optionnel |

### 6.3 Information de rÃ¨gles de franchissement

**CROSSING_RULES_INFO**
- **Objectif :** Fournir les rÃ¨gles de franchissement pour une frontiÃ¨re
- **Contenu :** Conditions dÃ©claratives, niveau de confiance requis, restrictions
- **Usage :** Contextualisation de l'Ã©valuation d'intention

**Structure des rÃ¨gles de franchissement :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `rule_id` | Identifiant unique de la rÃ¨gle | âœ… Oui |
| `boundary_id` | Identifiant de la frontiÃ¨re concernÃ©e | âœ… Oui |
| `required_trust_level` | Niveau de confiance minimum requis | âœ… Oui |
| `conditions` | Conditions dÃ©claratives Ã  satisfaire | âœ… Oui |
| `restrictions` | Restrictions applicables | âŒ Optionnel |

### 6.4 Information d'Ã©tat d'intÃ©gration

**INTEGRATION_STATE_INFO**
- **Objectif :** Fournir l'Ã©tat d'une intÃ©gration avec un systÃ¨me externe
- **Contenu :** Ã‰tat (active, suspendue, rÃ©voquÃ©e), niveau de confiance, frontiÃ¨res associÃ©es
- **Usage :** Contextualisation des intentions provenant d'intÃ©grations

**Structure de l'Ã©tat d'intÃ©gration :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `integration_id` | Identifiant unique de l'intÃ©gration | âœ… Oui |
| `state` | Ã‰tat (active, suspended, revoked) | âœ… Oui |
| `trust_level` | Niveau de confiance de l'intÃ©gration | âœ… Oui |
| `boundaries` | FrontiÃ¨res associÃ©es Ã  cette intÃ©gration | âœ… Oui |
| `last_state_change` | DerniÃ¨re modification d'Ã©tat | âŒ Optionnel |

---

## 7. Types de consultations

### 7.1 Consultation de contexte de frontiÃ¨re

**GET_BOUNDARY_CONTEXT**
- **Initiateur :** StrongFather
- **Objectif :** Obtenir le contexte de frontiÃ¨re pour une intention
- **Payload :** Identifiant de l'intention, source de l'intention
- **RÃ©ponse :** Contexte de frontiÃ¨re complet

**RÃ¨gle BG-SF-QUERY-01 : RÃ©ponse instantanÃ©e**

La rÃ©ponse Ã  une consultation de contexte est instantanÃ©e. Border Guard retourne le contexte connu au moment de la demande, sans dÃ©lai.

### 7.2 Consultation de niveau de confiance

**GET_TRUST_LEVEL**
- **Initiateur :** StrongFather
- **Objectif :** Obtenir le niveau de confiance d'une source spÃ©cifique
- **Payload :** Identifiant de la source
- **RÃ©ponse :** Niveau de confiance avec critÃ¨res

**RÃ¨gle BG-SF-QUERY-02 : Source non classifiÃ©e**

Si la source n'a pas Ã©tÃ© explicitement classifiÃ©e, Border Guard retourne `unknown` conformÃ©ment Ã  l'invariant INV-BG-4 (classification exhaustive).

### 7.3 Consultation de rÃ¨gles de franchissement

**GET_CROSSING_RULES**
- **Initiateur :** StrongFather
- **Objectif :** Obtenir les rÃ¨gles de franchissement pour une frontiÃ¨re
- **Payload :** Identifiant de la frontiÃ¨re, direction (entrÃ©e, sortie)
- **RÃ©ponse :** RÃ¨gles de franchissement dÃ©claratives

**RÃ¨gle BG-SF-QUERY-03 : RÃ¨gles complÃ¨tes**

Border Guard retourne toutes les rÃ¨gles applicables Ã  la frontiÃ¨re demandÃ©e. Les rÃ¨gles sont dÃ©claratives et expriment ce qui est requis, pas comment le vÃ©rifier.

### 7.4 Consultation d'Ã©tat d'intÃ©gration

**GET_INTEGRATION_STATE**
- **Initiateur :** StrongFather
- **Objectif :** Obtenir l'Ã©tat d'une intÃ©gration avec un systÃ¨me externe
- **Payload :** Identifiant de l'intÃ©gration
- **RÃ©ponse :** Ã‰tat complet de l'intÃ©gration

**RÃ¨gle BG-SF-QUERY-04 : IntÃ©gration inconnue**

Si l'intÃ©gration demandÃ©e n'est pas gouvernÃ©e par Border Guard, la rÃ©ponse est `NOT_FOUND` avec indication que l'intÃ©gration n'est pas dans le registre.

---

## 8. Protocole de communication

### 8.1 Format des consultations

Les consultations de StrongFather suivent un format standardisÃ©.

**Structure de base :**

| Ã‰lÃ©ment | Description | Obligatoire |
|---------|-------------|-------------|
| `query_id` | Identifiant unique de la consultation | âœ… Oui |
| `intention_id` | RÃ©fÃ©rence Ã  l'intention en cours d'Ã©valuation | âŒ Optionnel |
| `type` | Type de consultation | âœ… Oui |
| `payload` | DonnÃ©es spÃ©cifiques Ã  la consultation | âŒ Selon type |
| `contexte_appelant` | Contexte de StrongFather | âœ… Oui |
| `timestamp` | Horodatage de la consultation | âœ… Oui |

**RÃ¨gle BG-SF-PROT-01 : Format standardisÃ©**

Toutes les consultations respectent le format standardisÃ©. Aucune consultation ad-hoc n'est acceptÃ©e.

### 8.2 Format des rÃ©ponses

Les rÃ©ponses de Border Guard suivent un format standardisÃ©.

**Structure de base :**

| Ã‰lÃ©ment | Description | Obligatoire |
|---------|-------------|-------------|
| `response_id` | Identifiant unique de la rÃ©ponse | âœ… Oui |
| `query_id` | RÃ©fÃ©rence Ã  la consultation | âœ… Oui |
| `status` | Statut de la rÃ©ponse (SUCCESS, NOT_FOUND, UNKNOWN_SOURCE, ERROR) | âœ… Oui |
| `data` | DonnÃ©es de la rÃ©ponse | Si SUCCESS |
| `error` | DÃ©tails de l'erreur | Si ERROR |
| `timestamp` | Horodatage de la rÃ©ponse | âœ… Oui |

**RÃ¨gle BG-SF-PROT-02 : RÃ©ponse toujours structurÃ©e**

Border Guard retourne toujours une rÃ©ponse structurÃ©e, mÃªme en cas d'erreur ou de source non classifiÃ©e.

**RÃ¨gle BG-SF-PROT-03 : Pas d'interprÃ©tation dÃ©cisionnelle**

Les rÃ©ponses sont des informations de classification brutes. Border Guard n'interprÃ¨te pas les donnÃ©es pour StrongFather et ne suggÃ¨re jamais de dÃ©cision.

### 8.3 Statuts de rÃ©ponse

| Statut | Signification |
|--------|---------------|
| `SUCCESS` | La consultation a abouti, les donnÃ©es sont fournies |
| `NOT_FOUND` | L'Ã©lÃ©ment recherchÃ© (frontiÃ¨re, intÃ©gration) n'existe pas |
| `UNKNOWN_SOURCE` | La source n'est pas explicitement classifiÃ©e (niveau `unknown` retournÃ©) |
| `ERROR` | Une erreur interne s'est produite |

**RÃ¨gle BG-SF-PROT-04 : UNKNOWN_SOURCE n'est pas une erreur**

Le statut `UNKNOWN_SOURCE` est une rÃ©ponse valide, pas une erreur. Il indique que la source sera traitÃ©e avec le niveau de confiance `unknown` par dÃ©faut.

---

## 9. Flux d'intÃ©gration typique

### 9.1 Flux de consultation avant Ã©valuation

**Acteurs :** BondingBrother, StrongFather, Border Guard

**SÃ©quence :**

1. BondingBrother soumet une intention Ã  StrongFather pour Ã©valuation
2. StrongFather identifie que l'intention vient de l'extÃ©rieur ou traverse une frontiÃ¨re
3. StrongFather interroge Border Guard : `GET_BOUNDARY_CONTEXT`
4. Border Guard retourne le contexte de frontiÃ¨re (niveau de confiance, rÃ¨gles)
5. StrongFather intÃ¨gre le contexte dans l'Ã©valuation de l'intention
6. StrongFather Ã©value l'intention selon les politiques (en tenant compte du contexte)
7. StrongFather produit une dÃ©cision (acceptÃ©e, refusÃ©e, ambiguÃ«)

**RÃ¨gle BG-SF-FLOW-01 : Consultation optionnelle**

La consultation de Border Guard par StrongFather est toujours optionnelle. StrongFather peut Ã©valuer une intention sans consulter le contexte de frontiÃ¨re.

### 9.2 Flux de classification pour authentification en couches

**Acteurs :** Border Guard, StrongFather (selon RT-SEC-2)

**SÃ©quence :**

1. Une requÃªte arrive avec une source identifiÃ©e
2. Border Guard classifie la source selon ses critÃ¨res
3. StrongFather consulte Border Guard pour le niveau de confiance
4. StrongFather utilise ce niveau dans l'authentification en couches
5. Master Butler vÃ©rifie les capacitÃ©s selon le niveau de confiance
6. StrongFather produit la dÃ©cision finale

### 9.3 Diagramme de sÃ©quence

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  BondingBrother â”‚    â”‚   StrongFather  â”‚    â”‚  Border Guard   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚                      â”‚                      â”‚
         â”œâ”€â”€ Intention â”€â”€â”€â”€â”€â”€â”€â”€â–ºâ”‚                      â”‚
         â”‚                      â”‚                      â”‚
         â”‚                      â”œâ”€â”€ GET_BOUNDARY_CTX â”€â–ºâ”‚
         â”‚                      â”‚                      â”‚
         â”‚                      â”‚â—„â”€â”€ Contexte â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
         â”‚                      â”‚    (trust level,     â”‚
         â”‚                      â”‚     rules, etc.)     â”‚
         â”‚                      â”‚                      â”‚
         â”‚                      â”œâ”€â”€ Ã‰valuation â”€â”€â”€â”€â”€â”€â”€â”€â”¤
         â”‚                      â”‚   (avec contexte)    â”‚
         â”‚                      â”‚                      â”‚
         â”‚â—„â”€â”€ DÃ©cision â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤                      â”‚
         â”‚                      â”‚                      â”‚
```

---

## 10. RÃ¨gles d'intÃ©gration

### 10.1 RÃ¨gles de communication

**RÃ¨gle BG-SF-INT-01 : Initiative StrongFather**

StrongFather initie les consultations. Border Guard rÃ©pond aux consultations. Border Guard ne pousse jamais d'information vers StrongFather de maniÃ¨re non sollicitÃ©e.

**RÃ¨gle BG-SF-INT-02 : Pas de dÃ©pendance obligatoire**

StrongFather peut fonctionner sans consulter Border Guard. L'intÃ©gration enrichit le contexte mais n'est pas obligatoire.

**RÃ¨gle BG-SF-INT-03 : RÃ©ponses synchrones**

Les rÃ©ponses aux consultations sont synchrones et instantanÃ©es. Aucune consultation n'est diffÃ©rÃ©e.

### 10.2 RÃ¨gles de donnÃ©es

**RÃ¨gle BG-SF-INT-04 : DonnÃ©es actuelles**

Les donnÃ©es retournÃ©es par Border Guard reflÃ¨tent les classifications actuelles au moment de la consultation.

**RÃ¨gle BG-SF-INT-05 : Classifications stables**

Les classifications de Border Guard sont stables. Un mÃªme Ã©lÃ©ment consultÃ© deux fois retourne le mÃªme niveau de confiance (sauf modification explicite de la classification).

**RÃ¨gle BG-SF-INT-06 : CohÃ©rence interne garantie**

Border Guard garantit la cohÃ©rence interne des donnÃ©es retournÃ©es. Un contexte de frontiÃ¨re et ses rÃ¨gles sont mutuellement cohÃ©rents.

### 10.3 RÃ¨gles de traÃ§abilitÃ©

**RÃ¨gle BG-SF-INT-07 : TraÃ§abilitÃ© des consultations**

Toutes les consultations de StrongFather sont tracÃ©es par Border Guard avec le contexte complet.

**RÃ¨gle BG-SF-INT-08 : CorrÃ©lation intention-consultation**

Chaque consultation peut Ãªtre corrÃ©lÃ©e Ã  une intention en cours d'Ã©valuation (si `intention_id` fourni) pour l'audit bout-en-bout.

---

## 11. Gestion des erreurs

### 11.1 Types d'erreurs

**Erreurs de format :**
- Consultation mal formÃ©e
- Champ obligatoire manquant
- Type de consultation inconnu

**Erreurs de donnÃ©es :**
- FrontiÃ¨re non dÃ©finie (NOT_FOUND)
- IntÃ©gration non gouvernÃ©e (NOT_FOUND)
- Source non classifiÃ©e (UNKNOWN_SOURCE, pas une erreur)

**Erreurs internes :**
- Erreur du moteur de dÃ©finition de frontiÃ¨res
- Erreur de calcul de rÃ¨gles

### 11.2 Traitement des erreurs

**RÃ¨gle BG-SF-ERR-01 : RÃ©ponse structurÃ©e toujours**

Border Guard retourne toujours une rÃ©ponse structurÃ©e, mÃªme en cas d'erreur. StrongFather peut toujours interprÃ©ter la rÃ©ponse.

**RÃ¨gle BG-SF-ERR-02 : UNKNOWN_SOURCE est informatif**

Le statut `UNKNOWN_SOURCE` est une information, pas une erreur. StrongFather peut utiliser cette information (source non classifiÃ©e = niveau `unknown`).

**RÃ¨gle BG-SF-ERR-03 : Journalisation des erreurs**

Toutes les erreurs sont journalisÃ©es par Border Guard pour audit et diagnostic.

**RÃ¨gle BG-SF-ERR-04 : Pas de retry automatique**

En cas d'erreur, StrongFather dÃ©cide de la stratÃ©gie (retry, continuer sans contexte). Border Guard ne retry jamais automatiquement.

---

## 12. Cas particuliers

### 12.1 Source hostile

Lorsqu'une source est classifiÃ©e `hostile`, Border Guard retourne cette classification sans bloquer.

**RÃ¨gle BG-SF-CASE-01 : Hostile est une classification, pas un blocage**

La classification `hostile` est une information factuelle. C'est StrongFather qui dÃ©cide, selon ses politiques, si une intention d'une source `hostile` doit Ãªtre refusÃ©e.

### 12.2 IntÃ©gration suspendue

Lorsqu'une intÃ©gration est suspendue, Border Guard retourne l'Ã©tat `suspended`.

**RÃ¨gle BG-SF-CASE-02 : Suspended est un Ã©tat, pas un blocage**

L'Ã©tat `suspended` est une information factuelle. C'est StrongFather qui dÃ©cide, selon ses politiques, comment traiter les intentions venant d'une intÃ©gration suspendue.

### 12.3 FrontiÃ¨re non dÃ©finie

Si une frontiÃ¨re demandÃ©e n'est pas dÃ©finie par Border Guard :

**RÃ¨gle BG-SF-CASE-03 : FrontiÃ¨re non dÃ©finie = NOT_FOUND**

Border Guard retourne `NOT_FOUND`. StrongFather peut dÃ©cider de traiter l'intention sans contexte de frontiÃ¨re ou de la refuser selon ses politiques.

---

## 13. Garanties de l'intÃ©gration

### 13.1 Garantie d'exhaustivitÃ©

**Engagement :** Les rÃ©ponses de Border Guard sont exhaustives pour le pÃ©rimÃ¨tre de dÃ©finition. Toutes les informations connues sur une frontiÃ¨re ou une classification sont fournies.

### 13.2 Garantie d'exactitude

**Engagement :** Les informations fournies par Border Guard sont exactes et reflÃ¨tent les dÃ©finitions actuelles au moment de la consultation.

### 13.3 Garantie de neutralitÃ©

**Engagement :** Border Guard fournit des informations de classification sans interprÃ©tation dÃ©cisionnelle, sans recommandation, sans jugement. La dÃ©cision appartient exclusivement Ã  StrongFather.

### 13.4 Garantie de traÃ§abilitÃ©

**Engagement :** Toute interaction entre StrongFather et Border Guard est traÃ§able de bout en bout. L'audit complet des consultations et rÃ©ponses est possible.

### 13.5 Garantie de disponibilitÃ©

**Engagement :** Border Guard est disponible pour rÃ©pondre aux consultations de StrongFather sans dÃ©pendance externe (conformitÃ© LOI-1).

### 13.6 Garantie de non-blocage

**Engagement :** Border Guard ne bloque jamais les opÃ©rations de StrongFather. Les consultations sont rÃ©pondues immÃ©diatement.

---

## 14. Invariants de l'intÃ©gration

### 14.1 Invariants de relation

**INV-BG-SF-1 : Conseil unidirectionnel**

Border Guard conseille StrongFather. Border Guard ne dÃ©cide jamais pour StrongFather.

**INV-BG-SF-2 : Consultation facultative**

StrongFather consulte Border Guard de maniÃ¨re facultative. Aucune consultation n'est obligatoire.

**INV-BG-SF-3 : Aucune autoritÃ© partagÃ©e**

Border Guard n'a aucune autoritÃ© sur les dÃ©cisions. StrongFather n'a aucune autoritÃ© sur les dÃ©finitions de frontiÃ¨res.

### 14.2 Invariants de donnÃ©es

**INV-BG-SF-4 : Lecture pure**

Les consultations sont des lectures pures. Aucune modification des dÃ©finitions n'est causÃ©e par une consultation.

**INV-BG-SF-5 : DonnÃ©es de classification**

Les donnÃ©es retournÃ©es sont des classifications (niveaux de confiance, rÃ¨gles). Aucune donnÃ©e interprÃ©tÃ©e dÃ©cisionnellement n'est retournÃ©e.

### 14.3 Invariants de protocole

**INV-BG-SF-6 : Format respectÃ©**

Toutes les consultations et rÃ©ponses respectent le format standardisÃ©.

**INV-BG-SF-7 : TraÃ§abilitÃ© complÃ¨te**

Toute interaction est traÃ§able avec son contexte complet.

---

## 15. ConformitÃ© aux Lois d'Autonomie SystÃ¨me

### LOI-1 : Aucune dÃ©pendance externe critique

**ConformitÃ© :** âœ… **Conforme**

L'intÃ©gration respecte LOI-1 :
- Border Guard dÃ©finit les frontiÃ¨res localement, sans dÃ©pendance externe
- StrongFather consulte localement, sans dÃ©pendance externe
- L'absence de connexion ne bloque ni la dÃ©finition ni la consultation

### LOI-2 : Le systÃ¨me accepte l'isolement comme Ã©tat normal

**ConformitÃ© :** âœ… **Conforme**

L'intÃ©gration respecte LOI-2 :
- L'isolement ne modifie pas les classifications de Border Guard
- StrongFather peut prendre des dÃ©cisions mÃªme en Ã©tat isolÃ©
- Aucune dÃ©gradation de l'intÃ©gration en mode isolÃ©

### LOI-4 : Pas de temps global requis

**ConformitÃ© :** âœ… **Conforme**

L'intÃ©gration respecte LOI-4 :
- Les horodatages sont locaux
- Aucune synchronisation temporelle n'est requise
- Les classifications ne dÃ©pendent pas de timestamps synchronisÃ©s

---

## 16. Exemples

### 16.1 Consultation de contexte de frontiÃ¨re

**Consultation StrongFather :**
```
{
  "query_id": "q-sf-bg-001",
  "intention_id": "intention-500",
  "type": "GET_BOUNDARY_CONTEXT",
  "payload": {
    "source": "external-api-partner-x",
    "target": "internal-content-module"
  },
  "contexte_appelant": {
    "source": "strongfather",
    "evaluation_id": "eval-200"
  },
  "timestamp": "2026-01-27T14:00:00Z"
}
```

**RÃ©ponse Border Guard :**
```
{
  "response_id": "r-bg-001",
  "query_id": "q-sf-bg-001",
  "status": "SUCCESS",
  "data": {
    "context_id": "ctx-001",
    "source_trust_level": "verified",
    "boundaries_crossed": [
      {
        "boundary_id": "boundary-external-001",
        "type": "integration",
        "direction": "inbound"
      }
    ],
    "crossing_rules": [
      {
        "rule_id": "rule-001",
        "required_trust_level": "verified",
        "conditions": ["api_key_valid", "rate_limit_respected"]
      }
    ],
    "integration_state": {
      "integration_id": "partner-x",
      "state": "active",
      "trust_level": "verified"
    },
    "timestamp": "2026-01-27T14:00:00Z"
  },
  "timestamp": "2026-01-27T14:00:01Z"
}
```

### 16.2 Source non classifiÃ©e

**Consultation StrongFather :**
```
{
  "query_id": "q-sf-bg-002",
  "type": "GET_TRUST_LEVEL",
  "payload": {
    "source": "unknown-external-request"
  },
  "contexte_appelant": {
    "source": "strongfather"
  },
  "timestamp": "2026-01-27T15:00:00Z"
}
```

**RÃ©ponse Border Guard :**
```
{
  "response_id": "r-bg-002",
  "query_id": "q-sf-bg-002",
  "status": "UNKNOWN_SOURCE",
  "data": {
    "trust_level": "unknown",
    "criteria_applied": ["default_classification"],
    "source_identifier": "unknown-external-request",
    "classification_date": "2026-01-27T15:00:00Z"
  },
  "timestamp": "2026-01-27T15:00:01Z"
}
```

**Note :** StrongFather utilise cette information pour appliquer ses politiques concernant les sources `unknown`.

---

## 17. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il Ã©tablit l'interface et le protocole que Border Guard doit respecter pour s'intÃ©grer avec StrongFather.

Toute implÃ©mentation de l'intÃ©gration avec StrongFather doit respecter ce contrat. Toute violation entraÃ®ne un comportement non conforme.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif  
**DÃ©pendances :**
- Border Guard - Documentation Fondatrice v1.5 (Section 8)
- StrongFather - Documentation Fondatrice v1.5
- Miyukini Conceptual References - Security Protocols v1.0 (RT-SEC-2)
- Miyukini Conceptual References - Lois Autonomie Systeme v1.1

---

## 18. Mini log de gÃ©nÃ©ration

### DÃ©cision Ã©ditoriale E1 : Direction de la relation

**DÃ©cision prise :** La relation est de conseil : Border Guard informe, StrongFather dÃ©cide. Cette direction respecte la Documentation Fondatrice de Border Guard Section 8 qui dÃ©finit "Border Guard informe Strong Father sur le contexte de frontiÃ¨re ; Strong Father dÃ©cide".

**Application :** Tout le document est structurÃ© autour de cette relation de conseil unidirectionnel.

### DÃ©cision Ã©ditoriale E2 : Consultation facultative

**DÃ©cision prise :** La consultation de Border Guard par StrongFather est explicitement facultative. StrongFather peut Ã©valuer des intentions sans consulter le contexte de frontiÃ¨re.

**Application :** RÃ¨gle BG-SF-02 et INV-BG-SF-2 Ã©tablissent cette facultativitÃ©.

### Warning W1 : Risque de confusion dÃ©finition/dÃ©cision

**Warning rencontrÃ© :** Risque que les dÃ©finitions de frontiÃ¨res de Border Guard soient confondues avec des dÃ©cisions.

**DÃ©cision prise :** Les interdictions absolues (Section 5) clarifient que Border Guard ne dÃ©cide jamais. Les dÃ©finitions sont des classifications, pas des dÃ©cisions.

**Correction effectuÃ©e :** Section 5 explicite les interdictions, Section 13.3 garantit la neutralitÃ©.

### VÃ©rification de cohÃ©rence

**VÃ©rification effectuÃ©e :**
- âœ… CohÃ©rence avec Border Guard - Documentation Fondatrice : ConfirmÃ©e (relation de conseil, pas de dÃ©cision)
- âœ… CohÃ©rence avec StrongFather - Documentation Fondatrice : ConfirmÃ©e (StrongFather dÃ©cide, consulte le contexte)
- âœ… ConformitÃ© LOI-1 : ConfirmÃ©e (aucune dÃ©pendance externe)
- âœ… ConformitÃ© LOI-2 : ConfirmÃ©e (isolement n'affecte pas l'intÃ©gration)
- âœ… ConformitÃ© LOI-4 : ConfirmÃ©e (pas de temps global requis)
- âœ… Aucune autoritÃ© de Border Guard sur les dÃ©cisions : ConfirmÃ©e (INV-BG-SF-1, Section 5)
- âœ… TraÃ§abilitÃ© complÃ¨te : ConfirmÃ©e (INV-BG-SF-7)

**Conclusion :** Aucune contradiction dÃ©tectÃ©e. Le document est cohÃ©rent et non ambigu.

---

*Aucune autre erreur, warning, ou ambiguÃ¯tÃ© rencontrÃ©e lors de la rÃ©daction de ce document.*

