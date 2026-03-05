# LogisticsSteward - MasterButler Integration Contract

## 1. Contexte

Ce document dÃ©finit le **contrat d'intÃ©gration entre LogisticsSteward et MasterButler**. Il spÃ©cifie l'interface, le protocole, les rÃ¨gles de communication, et les garanties associÃ©es Ã  l'intÃ©gration avec MasterButler en tant que registre des capacitÃ©s et permissions.

Ce document complÃ¨te la Section 8.3 de la [Documentation Fondatrice](../../foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [LogisticsSteward - Architecture & Flows](../../architecture/LogisticsSteward%20-%20Architecture%20&%20Flows.md) pour les flux d'arbitrage
- [LogisticsSteward - Quota Definition Contract](../resources/LogisticsSteward%20-%20Quota%20Definition%20Contract.md) pour la dÃ©finition des quotas
- [Master Butler - Documentation Fondatrice](../../../MasterButler/foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md) pour la nature de MasterButler
- [Master Butler - Capability API Contract](../../../MasterButler/contracts/api/Master%20Butler%20-%20Capability%20API%20Contract.md) pour l'API des capacitÃ©s

L'intÃ©gration respecte les [Lois d'Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md) : toutes les interactions sont locales et ne requiÃ¨rent aucune dÃ©pendance externe (**LOI-1**).

## 2. PortÃ©e / Scope

Ce document couvre :
- L'interface contractuelle entre LogisticsSteward et MasterButler
- Le protocole de communication (interrogations et limitations)
- Les types d'interactions LogisticsSteward â†’ MasterButler
- Les rÃ¨gles d'intÃ©gration spÃ©cifiques
- La gestion des erreurs et des rÃ©ponses
- Les garanties de l'intÃ©gration

Ce document **ne couvre pas** :
- Les dÃ©tails internes de MasterButler (voir documentation MasterButler)
- Les dÃ©tails des registres de capacitÃ©s (voir Capability Registry Contract)
- Les stratÃ©gies de dÃ©gradation (voir Degradation Strategy Contract)
- L'intÃ©gration avec StrongFather (voir StrongFather Integration Contract)

---

## 3. Principe fondamental

**MasterButler expose les capacitÃ©s existantes. LogisticsSteward limite leur usage sans jamais modifier leur existence. MasterButler dit ce qui est possible, LogisticsSteward dit ce qui est autorisÃ© en termes de ressources.**

La relation est de **consultation et de limitation** :
- LogisticsSteward interroge MasterButler pour connaÃ®tre les capacitÃ©s disponibles
- LogisticsSteward applique des limitations d'usage sur ces capacitÃ©s
- MasterButler reflÃ¨te ces limitations sans les interprÃ©ter

La sÃ©paration est absolue : **l'existence d'une capacitÃ© est du ressort de MasterButler, la limitation de son usage est du ressort de LogisticsSteward**.

---

## 4. Nature de la relation LogisticsSteward â€” MasterButler

### 4.1 Relation de consultation et limitation

**LogisticsSteward consulte MasterButler pour :**
- ConnaÃ®tre les capacitÃ©s existantes pour appliquer des quotas
- Identifier les entitÃ©s consommatrices de ressources
- Obtenir les mÃ©tadonnÃ©es des capacitÃ©s pour le calcul d'arbitrage
- DÃ©couvrir les Tools et Toolkits pour la limitation d'usage

**LogisticsSteward limite les capacitÃ©s exposÃ©es par MasterButler :**
- Applique des quotas d'utilisation
- DÃ©finit des prioritÃ©s d'accÃ¨s aux capacitÃ©s
- Impose des plafonds de consommation
- Active des restrictions temporaires

**RÃ¨gle LS-MB-01 : Limitation sans modification d'existence**

LogisticsSteward ne peut jamais modifier l'existence d'une capacitÃ© dans MasterButler. Il peut uniquement limiter son usage. Une capacitÃ© dÃ©clarÃ©e dans MasterButler reste dÃ©clarÃ©e, mÃªme si son usage est entiÃ¨rement restreint.

**RÃ¨gle LS-MB-02 : ExhaustivitÃ© de la connaissance**

LogisticsSteward a accÃ¨s Ã  l'intÃ©gralitÃ© des capacitÃ©s dÃ©clarÃ©es dans MasterButler. Aucune capacitÃ© n'est masquÃ©e ou filtrÃ©e lors des interrogations.

**RÃ¨gle LS-MB-03 : IndÃ©pendance des registres**

Le registre des capacitÃ©s (MasterButler) et le registre des limitations (LogisticsSteward) sont strictement sÃ©parÃ©s. Aucun chevauchement n'est autorisÃ©.

### 4.2 SÃ©paration des responsabilitÃ©s

| ResponsabilitÃ© | LogisticsSteward | MasterButler |
|----------------|------------------|--------------|
| **DÃ©clarer les capacitÃ©s** | âŒ Jamais | âœ… Exclusif |
| **ConnaÃ®tre les capacitÃ©s** | âŒ Interroge | âœ… Exclusif |
| **Limiter l'usage** | âœ… Exclusif | âŒ Jamais |
| **DÃ©finir les quotas** | âœ… Exclusif | âŒ Jamais |
| **Attribuer les prioritÃ©s** | âœ… Exclusif | âŒ Jamais |
| **Appliquer les restrictions** | âœ… Exclusif | âŒ Jamais |
| **Supprimer des capacitÃ©s** | âŒ Jamais | âœ… Exclusif |
| **Exposer les permissions** | âŒ Jamais | âœ… Exclusif |

**RÃ¨gle LS-MB-04 : Aucun chevauchement**

Aucun chevauchement de responsabilitÃ©s n'est autorisÃ©. LogisticsSteward ne dÃ©clare jamais de capacitÃ©s, MasterButler ne limite jamais l'usage.

---

## 5. Types d'interactions

### 5.1 Interrogation des capacitÃ©s existantes

**CAPABILITY_CATALOG**
- **Objectif :** Obtenir la liste des capacitÃ©s pour appliquer des limitations
- **Payload :** Filtres optionnels (module, type, niveau)
- **RÃ©ponse :** Liste des capacitÃ©s avec leurs mÃ©tadonnÃ©es

**RÃ¨gle LS-MB-QUERY-01 : Catalogue complet**

LogisticsSteward peut interroger le catalogue complet des capacitÃ©s pour Ã©tablir ses rÃ¨gles de gouvernance.

### 5.2 Interrogation des entitÃ©s consommatrices

**CONSUMER_ENTITIES**
- **Objectif :** Identifier les entitÃ©s qui utilisent une capacitÃ©
- **Payload :** Identifiant de la capacitÃ©
- **RÃ©ponse :** Liste des entitÃ©s consommatrices (OpÃ©rateurs, Ã‰quipes, Services)

**RÃ¨gle LS-MB-QUERY-02 : TraÃ§abilitÃ© des consommateurs**

La liste des consommateurs permet Ã  LogisticsSteward de calculer les quotas et prioritÃ©s par entitÃ©.

### 5.3 Interrogation des Tools et Toolkits

**TOOL_METADATA**
- **Objectif :** Obtenir les mÃ©tadonnÃ©es d'un Tool pour le calcul de quota
- **Payload :** Identifiant du Tool
- **RÃ©ponse :** MÃ©tadonnÃ©es incluant coÃ»t estimÃ©, frÃ©quence d'appel, ressources requises

**TOOLKIT_COMPOSITION**
- **Objectif :** Obtenir la composition d'un Toolkit
- **Payload :** Identifiant du Toolkit
- **RÃ©ponse :** Liste des Tools avec leurs caractÃ©ristiques de consommation

**RÃ¨gle LS-MB-QUERY-03 : MÃ©tadonnÃ©es de consommation**

MasterButler expose les mÃ©tadonnÃ©es de consommation des Tools (coÃ»t, frÃ©quence, ressources) pour permettre Ã  LogisticsSteward de calculer les limitations.

### 5.4 Notification des limitations

**USAGE_LIMITATION**
- **Objectif :** Informer MasterButler d'une limitation d'usage
- **Payload :** CapacitÃ© concernÃ©e, type de limitation, paramÃ¨tres
- **RÃ©ponse :** Acquittement

**RÃ¨gle LS-MB-NOTIF-01 : Notification informative**

Les notifications de limitation sont informatives. MasterButler les enregistre mais ne les applique pas lui-mÃªme. L'application est du ressort du Kernel via les dÃ©cisions de LogisticsSteward.

### 5.5 Notification de restauration

**USAGE_RESTORATION**
- **Objectif :** Informer MasterButler de la levÃ©e d'une limitation
- **Payload :** CapacitÃ© concernÃ©e, limitation levÃ©e
- **RÃ©ponse :** Acquittement

**RÃ¨gle LS-MB-NOTIF-02 : Restauration explicite**

Toute levÃ©e de limitation fait l'objet d'une notification explicite pour maintenir la cohÃ©rence des Ã©tats.

### 5.6 RÃ¨gles gÃ©nÃ©rales d'interaction

**RÃ¨gle LS-MB-QUERY-04 : Interrogation sans effet de bord**

Les interrogations de LogisticsSteward ne modifient jamais l'Ã©tat de MasterButler. Ce sont des lectures pures.

**RÃ¨gle LS-MB-QUERY-05 : Notification avec acquittement**

Les notifications de limitation ou restauration attendent un acquittement de MasterButler pour garantir la prise en compte.

**RÃ¨gle LS-MB-QUERY-06 : RÃ©ponse immÃ©diate**

Les rÃ©ponses sont fournies immÃ©diatement. Aucune interrogation n'est mise en attente ou diffÃ©rÃ©e.

---

## 6. Protocole de communication

### 6.1 Format des interrogations

Les interrogations de LogisticsSteward suivent un format standardisÃ©.

**Structure de base :**

| Ã‰lÃ©ment | Description | Obligatoire |
|---------|-------------|-------------|
| `interrogation_id` | Identifiant unique de l'interrogation | âœ… Oui |
| `arbitrage_id` | RÃ©fÃ©rence Ã  l'arbitrage en cours | âœ… Oui |
| `type` | Type d'interrogation | âœ… Oui |
| `payload` | DonnÃ©es spÃ©cifiques Ã  l'interrogation | âœ… Oui |
| `contexte_appelant` | Contexte de LogisticsSteward | âœ… Oui |
| `timestamp` | Horodatage de l'interrogation | âœ… Oui |

**RÃ¨gle LS-MB-PROT-01 : Format standardisÃ©**

Toutes les interrogations respectent le format standardisÃ©. Aucune interrogation ad-hoc n'est acceptÃ©e.

**RÃ¨gle LS-MB-PROT-02 : TraÃ§abilitÃ© par arbitrage**

Chaque interrogation rÃ©fÃ©rence l'arbitrage en cours pour assurer la traÃ§abilitÃ© bout-en-bout.

### 6.2 Format des notifications

Les notifications de LogisticsSteward suivent un format standardisÃ©.

**Structure de base :**

| Ã‰lÃ©ment | Description | Obligatoire |
|---------|-------------|-------------|
| `notification_id` | Identifiant unique de la notification | âœ… Oui |
| `decision_id` | RÃ©fÃ©rence Ã  la dÃ©cision d'arbitrage | âœ… Oui |
| `type` | Type de notification (LIMITATION, RESTORATION) | âœ… Oui |
| `payload` | DonnÃ©es spÃ©cifiques Ã  la notification | âœ… Oui |
| `contexte_appelant` | Contexte de LogisticsSteward | âœ… Oui |
| `timestamp` | Horodatage de la notification | âœ… Oui |

**RÃ¨gle LS-MB-PROT-03 : Notification structurÃ©e**

Toutes les notifications respectent le format structurÃ©. MasterButler peut les enregistrer pour audit.

### 6.3 Format des rÃ©ponses

Les rÃ©ponses de MasterButler suivent un format standardisÃ©.

**Structure de base :**

| Ã‰lÃ©ment | Description | Obligatoire |
|---------|-------------|-------------|
| `reponse_id` | Identifiant unique de la rÃ©ponse | âœ… Oui |
| `interrogation_id` | RÃ©fÃ©rence Ã  l'interrogation | âœ… Oui |
| `statut` | Statut de la rÃ©ponse (SUCCESS, NOT_FOUND, ERROR) | âœ… Oui |
| `donnees` | DonnÃ©es de la rÃ©ponse | Si SUCCESS |
| `erreur` | DÃ©tails de l'erreur | Si ERROR |
| `timestamp` | Horodatage de la rÃ©ponse | âœ… Oui |

**RÃ¨gle LS-MB-PROT-04 : RÃ©ponse toujours structurÃ©e**

MasterButler retourne toujours une rÃ©ponse structurÃ©e, mÃªme en cas d'erreur ou de non-existence.

### 6.4 Statuts de rÃ©ponse

| Statut | Signification |
|--------|---------------|
| `SUCCESS` | L'interrogation/notification a abouti |
| `NOT_FOUND` | L'Ã©lÃ©ment recherchÃ© n'existe pas dans le registre |
| `INVALID_QUERY` | L'interrogation est mal formÃ©e ou incomplÃ¨te |
| `ACKNOWLEDGED` | La notification a Ã©tÃ© prise en compte |
| `ERROR` | Une erreur interne s'est produite |

**RÃ¨gle LS-MB-PROT-05 : NOT_FOUND est informatif**

Le statut `NOT_FOUND` indique qu'une capacitÃ© n'existe pas. LogisticsSteward ne peut pas limiter une capacitÃ© inexistante.

---

## 7. Flux d'interaction typique

### 7.1 Flux de calcul de quota

**Acteurs :** LogisticsSteward, MasterButler, Kernel

**SÃ©quence :**

1. LogisticsSteward reÃ§oit une demande de ressource (via Kernel)
2. LogisticsSteward interroge MasterButler : `CAPABILITY_CATALOG` (si nÃ©cessaire)
3. MasterButler rÃ©pond avec les capacitÃ©s concernÃ©es
4. LogisticsSteward interroge MasterButler : `TOOL_METADATA`
5. MasterButler rÃ©pond avec les mÃ©tadonnÃ©es de consommation
6. LogisticsSteward calcule les quotas applicables
7. LogisticsSteward soumet la dÃ©cision Ã  StrongFather
8. Si validÃ©e, LogisticsSteward notifie MasterButler : `USAGE_LIMITATION` (si limitation)
9. MasterButler acquitte la notification

**RÃ¨gle LS-MB-FLOW-01 : Interrogation avant limitation**

LogisticsSteward interroge toujours MasterButler avant d'appliquer une limitation pour s'assurer que la capacitÃ© existe.

### 7.2 Flux de dÃ©gradation

**Acteurs :** LogisticsSteward, MasterButler, WorrySentinel

**SÃ©quence :**

1. WorrySentinel signale une situation de stress (charge Ã©levÃ©e)
2. LogisticsSteward Ã©value le niveau de dÃ©gradation requis
3. LogisticsSteward interroge MasterButler : `CAPABILITY_CATALOG` (capacitÃ©s non critiques)
4. MasterButler rÃ©pond avec les capacitÃ©s et leurs niveaux de criticitÃ©
5. LogisticsSteward calcule les limitations de dÃ©gradation
6. LogisticsSteward notifie MasterButler : `USAGE_LIMITATION` (par capacitÃ©)
7. MasterButler acquitte les notifications

**RÃ¨gle LS-MB-FLOW-02 : DÃ©gradation par criticitÃ©**

Les capacitÃ©s sont limitÃ©es par ordre de criticitÃ© inverse : les moins critiques d'abord.

### 7.3 Flux de restauration

**Acteurs :** LogisticsSteward, MasterButler, Kernel

**SÃ©quence :**

1. Kernel signale un retour Ã  la normale (charge rÃ©duite)
2. LogisticsSteward Ã©value les limitations Ã  lever
3. LogisticsSteward notifie MasterButler : `USAGE_RESTORATION` (par capacitÃ©)
4. MasterButler acquitte les notifications
5. Les capacitÃ©s reprennent leur usage normal

**RÃ¨gle LS-MB-FLOW-03 : Restauration progressive**

La restauration est progressive, par paliers, en fonction de l'Ã©tat systÃ¨me.

### 7.4 Diagramme de sÃ©quence

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚     Kernel      â”‚    â”‚ LogisticsStewardâ”‚    â”‚  Master Butler  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚                      â”‚                      â”‚
         â”œâ”€â”€ Demande ressource â–ºâ”‚                      â”‚
         â”‚                      â”‚                      â”‚
         â”‚                      â”œâ”€â”€ CAPABILITY_CATALOGâ–ºâ”‚
         â”‚                      â”‚                      â”‚
         â”‚                      â”‚â—„â”€â”€ CapacitÃ©s â”€â”€â”€â”€â”€â”€â”€â”€â”¤
         â”‚                      â”‚                      â”‚
         â”‚                      â”œâ”€â”€ TOOL_METADATA â”€â”€â”€â”€â–ºâ”‚
         â”‚                      â”‚                      â”‚
         â”‚                      â”‚â—„â”€â”€ MÃ©tadonnÃ©es â”€â”€â”€â”€â”€â”€â”¤
         â”‚                      â”‚                      â”‚
         â”‚                      â”œâ”€â”€ Calcul quota â”€â”€â”€â”€â”€â”€â”¤
         â”‚                      â”‚   (interne)          â”‚
         â”‚                      â”‚                      â”‚
         â”‚                      â”œâ”€â”€ USAGE_LIMITATION â”€â–ºâ”‚
         â”‚                      â”‚                      â”‚
         â”‚                      â”‚â—„â”€â”€ ACKNOWLEDGED â”€â”€â”€â”€â”€â”¤
         â”‚                      â”‚                      â”‚
         â”‚â—„â”€â”€ DÃ©cision â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤                      â”‚
         â”‚                      â”‚                      â”‚
```

---

## 8. RÃ¨gles d'intÃ©gration

### 8.1 RÃ¨gles de communication

**RÃ¨gle LS-MB-INT-01 : LogisticsSteward initie**

LogisticsSteward initie les interrogations et notifications. MasterButler ne sollicite jamais LogisticsSteward spontanÃ©ment pour des questions de limitation.

**RÃ¨gle LS-MB-INT-02 : Notification aprÃ¨s validation**

Les notifications de limitation sont Ã©mises uniquement aprÃ¨s validation par StrongFather. Aucune limitation n'est notifiÃ©e avant validation.

**RÃ¨gle LS-MB-INT-03 : Synchronisme des interrogations**

Les interrogations sont synchrones. LogisticsSteward attend la rÃ©ponse avant de poursuivre l'arbitrage.

### 8.2 RÃ¨gles de donnÃ©es

**RÃ¨gle LS-MB-INT-04 : DonnÃ©es fraÃ®ches**

Les donnÃ©es retournÃ©es par MasterButler reflÃ¨tent l'Ã©tat actuel du registre au moment de l'interrogation.

**RÃ¨gle LS-MB-INT-05 : Cache autorisÃ© pour mÃ©tadonnÃ©es statiques**

LogisticsSteward peut mettre en cache les mÃ©tadonnÃ©es statiques des capacitÃ©s (coÃ»t, description) mais pas les donnÃ©es dynamiques (consommateurs actuels).

**RÃ¨gle LS-MB-INT-06 : CohÃ©rence des limitations**

Les limitations notifiÃ©es par LogisticsSteward sont cohÃ©rentes avec les capacitÃ©s dÃ©clarÃ©es dans MasterButler.

### 8.3 RÃ¨gles de traÃ§abilitÃ©

**RÃ¨gle LS-MB-INT-07 : TraÃ§abilitÃ© des interrogations**

Toutes les interrogations de LogisticsSteward sont tracÃ©es par les deux parties.

**RÃ¨gle LS-MB-INT-08 : CorrÃ©lation arbitrage-interrogation**

Chaque interrogation est corrÃ©lÃ©e Ã  l'arbitrage en cours pour permettre l'audit bout-en-bout.

**RÃ¨gle LS-MB-INT-09 : Historique des limitations**

MasterButler maintient un historique des notifications de limitation reÃ§ues pour audit.

---

## 9. Gestion des erreurs

### 9.1 Types d'erreurs

**Erreurs de format :**
- Interrogation mal formÃ©e
- Champ obligatoire manquant
- Type d'interrogation inconnu

**Erreurs de donnÃ©es :**
- CapacitÃ© inexistante (NOT_FOUND)
- Tool inexistant (NOT_FOUND)
- Limitation sur capacitÃ© inexistante

**Erreurs internes :**
- Erreur de registre MasterButler
- Erreur de calcul de limitation

### 9.2 Traitement des erreurs

**RÃ¨gle LS-MB-ERR-01 : RÃ©ponse structurÃ©e toujours**

MasterButler retourne toujours une rÃ©ponse structurÃ©e, mÃªme en cas d'erreur. LogisticsSteward peut toujours interprÃ©ter la rÃ©ponse.

**RÃ¨gle LS-MB-ERR-02 : NOT_FOUND bloque la limitation**

Si une capacitÃ© n'existe pas (NOT_FOUND), LogisticsSteward ne peut pas la limiter. L'arbitrage Ã©choue pour cette capacitÃ©.

**RÃ¨gle LS-MB-ERR-03 : Journalisation des erreurs**

Toutes les erreurs sont journalisÃ©es par les deux parties pour audit et diagnostic.

**RÃ¨gle LS-MB-ERR-04 : Pas de retry automatique**

En cas d'erreur, LogisticsSteward dÃ©cide de la stratÃ©gie (retry, Ã©chec de l'arbitrage). Aucun retry automatique.

### 9.3 Cas de capacitÃ© supprimÃ©e

**RÃ¨gle LS-MB-ERR-05 : Limitation orpheline**

Si une capacitÃ© est supprimÃ©e de MasterButler alors qu'une limitation existe, LogisticsSteward doit Ãªtre notifiÃ© pour nettoyer sa limitation orpheline.

**RÃ¨gle LS-MB-ERR-06 : Notification de suppression**

MasterButler notifie LogisticsSteward lors de la suppression d'une capacitÃ© pour permettre le nettoyage des limitations associÃ©es.

---

## 10. Garanties de l'intÃ©gration

### 10.1 Garantie de visibilitÃ©

**Engagement :** LogisticsSteward a une visibilitÃ© complÃ¨te sur les capacitÃ©s dÃ©clarÃ©es dans MasterButler. Aucune capacitÃ© n'est masquÃ©e.

### 10.2 Garantie de non-interfÃ©rence

**Engagement :** LogisticsSteward n'interfÃ¨re jamais avec l'existence des capacitÃ©s. Les limitations concernent uniquement l'usage.

### 10.3 Garantie de cohÃ©rence

**Engagement :** Les limitations notifiÃ©es sont cohÃ©rentes avec les capacitÃ©s existantes. Aucune limitation orpheline n'est crÃ©Ã©e intentionnellement.

### 10.4 Garantie de traÃ§abilitÃ©

**Engagement :** Toute interaction entre LogisticsSteward et MasterButler est traÃ§able de bout en bout. L'audit complet est possible.

### 10.5 Garantie de disponibilitÃ©

**Engagement :** MasterButler est disponible pour rÃ©pondre aux interrogations de LogisticsSteward sans dÃ©pendance externe (conformitÃ© LOI-1).

### 10.6 Garantie de restauration

**Engagement :** Toute limitation peut Ãªtre levÃ©e. Le systÃ¨me peut toujours revenir Ã  un Ã©tat sans limitation.

---

## 11. Invariants de l'intÃ©gration

### 11.1 Invariants de relation

**INV-LS-MB-1 : SÃ©paration existence/usage**

L'existence des capacitÃ©s (MasterButler) et la limitation de leur usage (LogisticsSteward) sont strictement sÃ©parÃ©es.

**INV-LS-MB-2 : Limitation sur capacitÃ© existante**

LogisticsSteward ne peut limiter que des capacitÃ©s existantes dans MasterButler.

**INV-LS-MB-3 : Non-modification d'existence**

LogisticsSteward ne peut jamais crÃ©er, modifier, ou supprimer une capacitÃ© dans MasterButler.

### 11.2 Invariants de donnÃ©es

**INV-LS-MB-4 : Lecture pure**

Les interrogations sont des lectures pures. Aucune modification du registre n'est causÃ©e par une interrogation.

**INV-LS-MB-5 : Notification avec acquittement**

Toute notification de limitation ou restauration attend un acquittement avant d'Ãªtre considÃ©rÃ©e comme appliquÃ©e.

### 11.3 Invariants de protocole

**INV-LS-MB-6 : Format respectÃ©**

Toutes les interrogations, notifications, et rÃ©ponses respectent le format standardisÃ©.

**INV-LS-MB-7 : TraÃ§abilitÃ© complÃ¨te**

Toute interaction est traÃ§able avec son contexte complet.

**INV-LS-MB-8 : Validation prÃ©alable**

Toute limitation notifiÃ©e a Ã©tÃ© prÃ©alablement validÃ©e par StrongFather.

---

## 12. Exemples

### 12.1 Interrogation du catalogue de capacitÃ©s

**Interrogation LogisticsSteward :**
```
{
  "interrogation_id": "int-ls-001",
  "arbitrage_id": "arb-100",
  "type": "CAPABILITY_CATALOG",
  "payload": {
    "module_filter": "miyukini-spm-cms",
    "include_metadata": true
  },
  "contexte_appelant": {
    "source": "logisticssteward",
    "reason": "quota_calculation"
  },
  "timestamp": "2026-01-28T10:00:00Z"
}
```

**RÃ©ponse MasterButler :**
```
{
  "reponse_id": "resp-mb-001",
  "interrogation_id": "int-ls-001",
  "statut": "SUCCESS",
  "donnees": {
    "capabilities": [
      {
        "id": "content.create",
        "name": "Create Content",
        "module": "miyukini-spm-cms-content",
        "criticality": "standard",
        "estimated_cost": "medium"
      },
      {
        "id": "content.publish",
        "name": "Publish Content",
        "module": "miyukini-spm-cms-content",
        "criticality": "high",
        "estimated_cost": "high"
      }
    ],
    "total_count": 2
  },
  "timestamp": "2026-01-28T10:00:01Z"
}
```

### 12.2 Notification de limitation d'usage

**Notification LogisticsSteward :**
```
{
  "notification_id": "notif-ls-001",
  "decision_id": "dec-arb-100",
  "type": "USAGE_LIMITATION",
  "payload": {
    "capability_id": "content.publish",
    "limitation_type": "quota",
    "parameters": {
      "max_calls_per_hour": 10,
      "affected_entities": ["operator-cms-001"],
      "reason": "high_system_load"
    },
    "expiration": "2026-01-28T12:00:00Z"
  },
  "contexte_appelant": {
    "source": "logisticssteward",
    "strongfather_validation": "val-sf-050"
  },
  "timestamp": "2026-01-28T10:05:00Z"
}
```

**RÃ©ponse MasterButler :**
```
{
  "reponse_id": "resp-mb-002",
  "notification_id": "notif-ls-001",
  "statut": "ACKNOWLEDGED",
  "donnees": {
    "limitation_registered": true,
    "affected_capability": "content.publish",
    "registration_id": "lim-mb-001"
  },
  "timestamp": "2026-01-28T10:05:01Z"
}
```

### 12.3 Notification de restauration

**Notification LogisticsSteward :**
```
{
  "notification_id": "notif-ls-002",
  "decision_id": "dec-arb-101",
  "type": "USAGE_RESTORATION",
  "payload": {
    "capability_id": "content.publish",
    "limitation_id": "lim-mb-001",
    "reason": "system_load_normalized"
  },
  "contexte_appelant": {
    "source": "logisticssteward",
    "strongfather_validation": "val-sf-051"
  },
  "timestamp": "2026-01-28T12:30:00Z"
}
```

**RÃ©ponse MasterButler :**
```
{
  "reponse_id": "resp-mb-003",
  "notification_id": "notif-ls-002",
  "statut": "ACKNOWLEDGED",
  "donnees": {
    "limitation_removed": true,
    "capability_id": "content.publish",
    "full_access_restored": true
  },
  "timestamp": "2026-01-28T12:30:01Z"
}
```

### 12.4 CapacitÃ© inexistante

**Interrogation LogisticsSteward :**
```
{
  "interrogation_id": "int-ls-002",
  "arbitrage_id": "arb-102",
  "type": "TOOL_METADATA",
  "payload": {
    "tool_id": "nonexistent.tool"
  },
  "contexte_appelant": {
    "source": "logisticssteward",
    "reason": "quota_calculation"
  },
  "timestamp": "2026-01-28T11:00:00Z"
}
```

**RÃ©ponse MasterButler :**
```
{
  "reponse_id": "resp-mb-004",
  "interrogation_id": "int-ls-002",
  "statut": "NOT_FOUND",
  "donnees": {
    "exists": false,
    "tool_id": "nonexistent.tool"
  },
  "timestamp": "2026-01-28T11:00:01Z"
}
```

**Note :** LogisticsSteward ne peut pas crÃ©er de limitation sur ce Tool inexistant.

---

## 13. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il Ã©tablit l'interface et le protocole que LogisticsSteward doit respecter pour s'intÃ©grer avec MasterButler.

Toute implÃ©mentation de l'intÃ©gration avec MasterButler doit respecter ce contrat. Toute violation entraÃ®ne un comportement non conforme.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif  
**DÃ©pendances :**
- LogisticsSteward - Documentation Fondatrice v1.0.0 (Section 8.3)
- LogisticsSteward - Quota Definition Contract
- Master Butler - Documentation Fondatrice
- Master Butler - Capability API Contract

---

## 14. Mini log de gÃ©nÃ©ration

### DÃ©cision Ã©ditoriale E1 : Direction de la relation

**DÃ©cision prise :** La relation est bidirectionnelle asymÃ©trique : LogisticsSteward interroge et notifie, MasterButler rÃ©pond et acquitte. LogisticsSteward est l'initiateur, MasterButler est le rÃ©pondant.

**Application :** Le document est structurÃ© autour de cette direction d'interaction.

### DÃ©cision Ã©ditoriale E2 : Types d'interactions

**DÃ©cision prise :** Les interactions sont divisÃ©es en interrogations (lecture) et notifications (Ã©criture informative). Les interrogations portent sur les capacitÃ©s, les notifications portent sur les limitations.

**Application :** Section 5 dÃ©finit chaque type avec objectif, payload, et rÃ©ponse.

### Warning W1 : Limitation sur capacitÃ© inexistante

**Warning rencontrÃ© :** Risque de crÃ©er des limitations orphelines sur des capacitÃ©s qui n'existent pas ou plus.

**DÃ©cision prise :** LogisticsSteward doit interroger l'existence avant de limiter. Les limitations orphelines sont nettoyÃ©es via notification de suppression.

**Correction effectuÃ©e :** RÃ¨gles LS-MB-ERR-02, LS-MB-ERR-05, LS-MB-ERR-06 ajoutÃ©es.

### Warning W2 : SÃ©paration existence/usage

**Warning rencontrÃ© :** Risque de confusion entre "capacitÃ© inexistante" et "capacitÃ© limitÃ©e Ã  zÃ©ro".

**DÃ©cision prise :** La distinction est explicite : une capacitÃ© peut Ãªtre totalement limitÃ©e mais existe toujours. LogisticsSteward ne peut pas supprimer une capacitÃ©.

**Correction effectuÃ©e :** INV-LS-MB-1 et INV-LS-MB-3 clarifient cette sÃ©paration.

### VÃ©rification de cohÃ©rence

**VÃ©rification effectuÃ©e :**
- âœ… CohÃ©rence avec LogisticsSteward - Documentation Fondatrice : ConfirmÃ©e (Section 8.3 respectÃ©e)
- âœ… CohÃ©rence avec Master Butler - Documentation Fondatrice : ConfirmÃ©e (registre des capacitÃ©s respectÃ©)
- âœ… ConformitÃ© LOI-1 : ConfirmÃ©e (aucune dÃ©pendance externe pour les interactions)
- âœ… ConformitÃ© INV-LS-7 : ConfirmÃ©e (sÃ©paration avec le Kernel maintenue)

**Conclusion :** Aucune contradiction dÃ©tectÃ©e. Le document est cohÃ©rent et non ambigu.

---

*Aucune autre erreur, warning, ou ambiguÃ¯tÃ© rencontrÃ©e lors de la rÃ©daction de ce document.*

