# BondingBrother - Authority Delegation Contract

## 1. Contexte

Ce document dÃ©finit le contrat de dÃ©lÃ©gation aux autoritÃ©s dans Bonding Brother. Il spÃ©cifie comment Bonding Brother dÃ©lÃ¨gue les dÃ©cisions aux autoritÃ©s (Kind Mother et Strong Father), comment il transmet les demandes, et comment il gÃ¨re les rÃ©ponses.

Ce document complÃ¨te la Section 6 de la [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur l'[Intent Model Contract](../intent/BondingBrother%20-%20Intent%20Model%20Contract.md) et le [Bilateral Flow Contract](../flows/BondingBrother%20-%20Bilateral%20Flow%20Contract.md) pour dÃ©finir les rÃ¨gles prÃ©cises de dÃ©lÃ©gation.

La dÃ©lÃ©gation respecte **LOI-2** (isolement comme Ã©tat normal) : en mode offline, la dÃ©lÃ©gation est diffÃ©rÃ©e mais les intentions sont prÃ©servÃ©es localement. Voir les [Lois d'Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md).

## 2. PortÃ©e / Scope

Ce document couvre :
- Les principes fondamentaux de la dÃ©lÃ©gation
- Les rÃ¨gles de dÃ©lÃ©gation Ã  Kind Mother
- Les rÃ¨gles de dÃ©lÃ©gation Ã  Strong Father
- Le routage vers l'autoritÃ© appropriÃ©e
- La transmission fidÃ¨le des demandes et rÃ©ponses
- La gestion de l'autoritÃ© diffÃ©rÃ©e (mode offline)
- Les garanties de non-dÃ©cision

Ce document **ne couvre pas** :
- Les dÃ©tails d'intÃ©gration avec Kind Mother (voir [KindMother Integration Contract](../integration/BondingBrother%20-%20KindMother%20Integration%20Contract.md))
- Les dÃ©tails d'intÃ©gration avec Strong Father (voir [StrongFather Integration Contract](../integration/BondingBrother%20-%20StrongFather%20Integration%20Contract.md))
- Les rÃ¨gles de traduction (voir [Translation Contract](../intent/BondingBrother%20-%20Translation%20Contract.md))
- Les rÃ¨gles de filtrage (voir [Filtering & Projection Contract](../intent/BondingBrother%20-%20Filtering%20%26%20Projection%20Contract.md))
- Le mode offline dÃ©taillÃ© (voir [Offline & Deferred Authority Contract](../offline/BondingBrother%20-%20Offline%20%26%20Deferred%20Authority%20Contract.md))

---

## 3. Principe fondamental

**Bonding Brother dÃ©lÃ¨gue toute dÃ©cision aux autoritÃ©s. Il ne dÃ©cide jamais, ne crÃ©e aucune rÃ¨gle, ne dÃ©tient aucune vÃ©ritÃ©. Il transmet fidÃ¨lement les demandes et les rÃ©ponses, sans interprÃ©tation ni modification.**

La dÃ©lÃ©gation est totale, systÃ©matique, et non nÃ©gociable. Toute dÃ©cision appartient exclusivement Ã  une autoritÃ©.

---

## 4. Nature de la dÃ©lÃ©gation

### 4.1 DÃ©lÃ©gation totale

**RÃ¨gle DELEG-01 : Absence de dÃ©cision**

Bonding Brother ne prend jamais de dÃ©cision stratÃ©gique, politique, ou opÃ©rationnelle. Toute dÃ©cision est dÃ©lÃ©guÃ©e Ã  une autoritÃ©.

**RÃ¨gle DELEG-02 : Absence de rÃ¨gle**

Bonding Brother ne crÃ©e aucune rÃ¨gle. Toutes les rÃ¨gles viennent des autoritÃ©s ou de l'Ã©cosystÃ¨me. Bonding Brother applique ces rÃ¨gles, mais ne les dÃ©finit jamais.

**RÃ¨gle DELEG-03 : Absence de vÃ©ritÃ©**

Bonding Brother ne dÃ©tient aucune vÃ©ritÃ© sur les donnÃ©es, les identitÃ©s, les permissions, les dÃ©cisions. Toute vÃ©ritÃ© vient d'une autoritÃ©.

### 4.2 Transmission fidÃ¨le

**RÃ¨gle DELEG-04 : Transmission sans modification**

Bonding Brother transmet les demandes aux autoritÃ©s sans modification, sans interprÃ©tation, sans enrichissement mÃ©tier.

**RÃ¨gle DELEG-05 : RÃ©ception sans modification**

Bonding Brother reÃ§oit les rÃ©ponses des autoritÃ©s sans modification, sans interprÃ©tation, sans remplacement.

**RÃ¨gle DELEG-06 : PrÃ©servation du contexte**

Le contexte est transmis intÃ©gralement aux autoritÃ©s, sans filtrage ni modification.

### 4.3 RÃ´le de mÃ©diateur

**RÃ¨gle DELEG-07 : Canal, pas source**

Bonding Brother est un canal de communication, pas une source de dÃ©cision. Il transmet, il ne dÃ©cide pas.

**RÃ¨gle DELEG-08 : Traducteur, pas dÃ©cideur**

Bonding Brother traduit les formats et les vocabulaires, mais ne modifie jamais le sens ni la dÃ©cision.

**RÃ¨gle DELEG-09 : Filtre, pas crÃ©ateur**

Bonding Brother filtre les informations selon les rÃ¨gles dÃ©finies par les autoritÃ©s, mais ne crÃ©e pas ces rÃ¨gles.

---

## 5. Routage vers les autoritÃ©s

### 5.1 Identification de l'autoritÃ© cible

**RÃ¨gle ROUTE-01 : Un type = une autoritÃ©**

Chaque type d'intention cible une et une seule autoritÃ© :
- Intentions de donnÃ©es â†’ Kind Mother
- Intentions d'identitÃ©/permissions â†’ Strong Father

**RÃ¨gle ROUTE-02 : Routage par type**

Le routage est dÃ©terminÃ© par le type d'intention, pas par le contenu ou le contexte.

**RÃ¨gle ROUTE-03 : Pas de routage conditionnel**

Le routage n'est jamais conditionnel. Il est dÃ©terministe basÃ© sur le type d'intention.

### 5.2 Types d'intentions par autoritÃ©

**Kind Mother (autoritÃ© des donnÃ©es) :**
- `CREATE_CONTENT`
- `UPDATE_CONTENT`
- `DELETE_CONTENT`
- `READ_CONTENT`
- `QUERY_CONTENT`
- `CREATE_NODE` (hiÃ©rarchie)
- `MOVE_NODE` (hiÃ©rarchie)
- `DELETE_NODE` (hiÃ©rarchie)
- Toute intention liÃ©e Ã  la persistance ou Ã  la cohÃ©rence des donnÃ©es

**StrongFather (autoritÃ© des dÃ©cisions stratÃ©giques et politiques) :**
- `AUTHENTICATE`
- `AUTHORIZE`
- `CREATE_SESSION`
- `REVOKE_SESSION`
- `CHECK_PERMISSION`
- Toute intention liÃ©e aux identitÃ©s, permissions, ou dÃ©cisions politiques

**RÃ¨gle ROUTE-04 : Pas d'intention multi-autoritÃ©**

Une intention ne peut jamais cibler plusieurs autoritÃ©s simultanÃ©ment. Si une opÃ©ration nÃ©cessite plusieurs autoritÃ©s, elle doit Ãªtre dÃ©composÃ©e en plusieurs intentions.

### 5.3 Gestion des erreurs de routage

**RÃ¨gle ROUTE-05 : Type inconnu**

Si le type d'intention n'est pas reconnu ou ne mappe vers aucune autoritÃ©, l'intention est rejetÃ©e immÃ©diatement.

**RÃ¨gle ROUTE-06 : AutoritÃ© indisponible**

Si l'autoritÃ© cible est indisponible, l'intention est mise en buffer offline (voir [Offline & Deferred Authority Contract](../offline/BondingBrother%20-%20Offline%20%26%20Deferred%20Authority%20Contract.md)).

---

## 6. DÃ©lÃ©gation Ã  Kind Mother

### 6.1 Domaine de dÃ©lÃ©gation

**Kind Mother est l'autoritÃ© des donnÃ©es :**
- Persistance des donnÃ©es
- CohÃ©rence des donnÃ©es
- IntÃ©gritÃ© des donnÃ©es
- Gestion des hiÃ©rarchies
- Gestion des relations

**RÃ¨gle KM-01 : DÃ©cisions de persistance**

Toute dÃ©cision concernant la persistance, la modification, ou la suppression de donnÃ©es est dÃ©lÃ©guÃ©e Ã  Kind Mother.

**RÃ¨gle KM-02 : DÃ©cisions de cohÃ©rence**

Toute dÃ©cision concernant la cohÃ©rence, l'intÃ©gritÃ©, ou les contraintes de donnÃ©es est dÃ©lÃ©guÃ©e Ã  Kind Mother.

**RÃ¨gle KM-03 : DÃ©cisions de structure**

Toute dÃ©cision concernant la structure, la hiÃ©rarchie, ou l'organisation des donnÃ©es est dÃ©lÃ©guÃ©e Ã  Kind Mother.

### 6.2 Transmission Ã  Kind Mother

**RÃ¨gle KM-04 : Format adaptÃ©**

La demande est traduite dans le format et le vocabulaire que Kind Mother comprend.

**RÃ¨gle KM-05 : Contexte complet**

Le contexte est transmis intÃ©gralement Ã  Kind Mother, sans modification.

**RÃ¨gle KM-06 : Pas d'interprÃ©tation**

Bonding Brother ne modifie jamais la demande avant transmission. Il traduit le format, pas le sens.

### 6.3 RÃ©ception de Kind Mother

**RÃ¨gle KM-07 : RÃ©ception fidÃ¨le**

La rÃ©ponse de Kind Mother est reÃ§ue intÃ©gralement, sans modification ni interprÃ©tation.

**RÃ¨gle KM-08 : PrÃ©servation de la dÃ©cision**

La dÃ©cision de Kind Mother (acceptÃ©e, refusÃ©e, erreur) est prÃ©servÃ©e intÃ©gralement.

**RÃ¨gle KM-09 : Transmission au produit**

La rÃ©ponse est traduite et filtrÃ©e avant transmission au produit, mais la dÃ©cision reste inchangÃ©e.

---

## 7. DÃ©lÃ©gation Ã  Strong Father

### 7.1 Domaine de dÃ©lÃ©gation

**Strong Father est l'autoritÃ© des identitÃ©s et permissions :**
- Authentification
- Autorisation
- Gestion des sessions
- DÃ©cisions politiques
- RÃ¨gles de sÃ©curitÃ©

**RÃ¨gle SF-01 : DÃ©cisions d'authentification**

Toute dÃ©cision concernant l'authentification d'un utilisateur est dÃ©lÃ©guÃ©e Ã  Strong Father.

**RÃ¨gle SF-02 : DÃ©cisions d'autorisation**

Toute dÃ©cision concernant l'autorisation d'une action est dÃ©lÃ©guÃ©e Ã  Strong Father.

**RÃ¨gle SF-03 : DÃ©cisions politiques**

Toute dÃ©cision stratÃ©gique ou politique est dÃ©lÃ©guÃ©e Ã  Strong Father.

**RÃ¨gle SF-04 : DÃ©cisions de session**

Toute dÃ©cision concernant la crÃ©ation, la validation, ou la rÃ©vocation de sessions est dÃ©lÃ©guÃ©e Ã  Strong Father.

### 7.2 Transmission Ã  Strong Father

**RÃ¨gle SF-05 : Format adaptÃ©**

La demande est traduite dans le format et le vocabulaire que Strong Father comprend.

**RÃ¨gle SF-06 : Contexte complet**

Le contexte est transmis intÃ©gralement Ã  Strong Father, sans modification.

**RÃ¨gle SF-07 : Pas d'interprÃ©tation**

Bonding Brother ne modifie jamais la demande avant transmission. Il traduit le format, pas le sens.

### 7.3 RÃ©ception de Strong Father

**RÃ¨gle SF-08 : RÃ©ception fidÃ¨le**

La rÃ©ponse de Strong Father est reÃ§ue intÃ©gralement, sans modification ni interprÃ©tation.

**RÃ¨gle SF-09 : PrÃ©servation de la dÃ©cision**

La dÃ©cision de Strong Father (autorisÃ©, refusÃ©, erreur) est prÃ©servÃ©e intÃ©gralement.

**RÃ¨gle SF-10 : Transmission au produit**

La rÃ©ponse est traduite et filtrÃ©e avant transmission au produit, mais la dÃ©cision reste inchangÃ©e.

---

## 8. AutoritÃ© diffÃ©rÃ©e (mode offline)

### 8.1 Principe

**RÃ¨gle OFFLINE-01 : DÃ©lÃ©gation diffÃ©rÃ©e**

Quand une autoritÃ© n'est pas accessible, la dÃ©lÃ©gation est diffÃ©rÃ©e. L'intention est journalisÃ©e et transmise lorsque la connexion est rÃ©tablie.

**RÃ¨gle OFFLINE-02 : Pas de dÃ©cision locale**

Bonding Brother ne prend jamais de dÃ©cision Ã  la place de l'autoritÃ©, mÃªme en mode offline.

**RÃ¨gle OFFLINE-03 : Journalisation systÃ©matique**

Toute intention en attente de dÃ©lÃ©gation est journalisÃ©e avec un marqueur "offline".

### 8.2 Gestion du buffer offline

**RÃ¨gle OFFLINE-04 : Stockage temporaire**

Les intentions en attente sont stockÃ©es dans un buffer temporaire, ordonnÃ© chronologiquement.

**RÃ¨gle OFFLINE-05 : Transmission Ã  la reconnexion**

Lors de la reconnexion, les intentions sont transmises dans l'ordre (FIFO) Ã  l'autoritÃ©.

**RÃ¨gle OFFLINE-06 : RÃ©ception diffÃ©rÃ©e**

Les rÃ©ponses diffÃ©rÃ©es sont reÃ§ues et transmises aux produits lorsque disponibles.

### 8.3 Garanties en mode offline

**RÃ¨gle OFFLINE-07 : Aucune perte**

Aucune intention n'est perdue en mode offline. Toutes sont transmises Ã  la reconnexion.

**RÃ¨gle OFFLINE-08 : Ordre prÃ©servÃ©**

L'ordre des intentions est prÃ©servÃ© lors de la transmission diffÃ©rÃ©e.

**RÃ¨gle OFFLINE-09 : TraÃ§abilitÃ©**

Toutes les intentions en mode offline sont traÃ§ables, avec horodatage de crÃ©ation et de transmission.

---

## 9. Transmission fidÃ¨le

### 9.1 Principe de fidÃ©litÃ©

**RÃ¨gle FID-01 : Pas de modification**

Bonding Brother ne modifie jamais le contenu d'une demande ou d'une rÃ©ponse. Il traduit le format, mais prÃ©serve le sens.

**RÃ¨gle FID-02 : Pas d'interprÃ©tation**

Bonding Brother ne interprÃ¨te jamais une demande ou une rÃ©ponse. Il transmet fidÃ¨lement ce qu'il reÃ§oit.

**RÃ¨gle FID-03 : Pas d'enrichissement mÃ©tier**

Bonding Brother n'ajoute jamais d'information mÃ©tier Ã  une demande ou une rÃ©ponse. Seuls les enrichissements techniques sont autorisÃ©s (mÃ©tadonnÃ©es de traÃ§abilitÃ©).

### 9.2 Enrichissements techniques autorisÃ©s

**MÃ©tadonnÃ©es de traÃ§abilitÃ© :**
- `intention_id` (pour corrÃ©lation)
- `demande_id` (pour traÃ§abilitÃ©)
- `timestamp_demande` (pour ordre chronologique)
- `autoritÃ©_cible` (pour routage)

**RÃ¨gle FID-04 : Enrichissements non mÃ©tier**

Les enrichissements techniques ne modifient jamais le sens de la demande ou de la rÃ©ponse.

### 9.3 PrÃ©servation du contexte

**RÃ¨gle FID-05 : Contexte intÃ©gral**

Le contexte est transmis intÃ©gralement aux autoritÃ©s, sans filtrage ni modification.

**RÃ¨gle FID-06 : Pas de masquage**

Bonding Brother ne masque jamais d'information du contexte aux autoritÃ©s.

**RÃ¨gle FID-07 : TraÃ§abilitÃ© complÃ¨te**

Le contexte complet est journalisÃ© pour traÃ§abilitÃ©, mÃªme s'il n'est pas utilisÃ© par l'autoritÃ©.

---

## 10. Garanties de dÃ©lÃ©gation

### 10.1 Garantie de non-dÃ©cision

**Engagement :** Bonding Brother ne prend jamais de dÃ©cision Ã  la place d'une autoritÃ©. Toute dÃ©cision vient exclusivement d'une autoritÃ©.

**Mesure :** VÃ©rification structurelle que Bonding Brother n'a pas de logique de dÃ©cision mÃ©tier.

### 10.2 Garantie de fidÃ©litÃ©

**Engagement :** Les demandes sont transmises fidÃ¨lement aux autoritÃ©s, et les rÃ©ponses sont transmises fidÃ¨lement aux produits (aprÃ¨s traduction et filtrage).

**Mesure :** Tests de round-trip avec vÃ©rification que les dÃ©cisions sont prÃ©servÃ©es.

### 10.3 Garantie de complÃ©tude

**Engagement :** Toute demande est transmise Ã  l'autoritÃ© appropriÃ©e, et toute rÃ©ponse est transmise au produit (mÃªme en mode offline).

**Mesure :** TraÃ§abilitÃ© complÃ¨te avec vÃ©rification que chaque demande a une rÃ©ponse.

### 10.4 Garantie de routage correct

**Engagement :** Chaque intention est routÃ©e vers la bonne autoritÃ©, sans erreur de routage.

**Mesure :** Tests avec tous les types d'intentions vÃ©rifiant le routage correct.

### 10.5 Garantie de disponibilitÃ©

**Engagement :** La dÃ©lÃ©gation fonctionne mÃªme en mode offline, avec transmission diffÃ©rÃ©e Ã  la reconnexion.

**Mesure :** Tests de fonctionnement offline avec vÃ©rification de la transmission diffÃ©rÃ©e.

---

## 11. Violations et anti-patterns

### 11.1 Violations interdites

**Violation VIO-01 : DÃ©cision par Bonding Brother**

Bonding Brother ne doit jamais prendre de dÃ©cision mÃ©tier. Toute logique de dÃ©cision est une violation.

**Violation VIO-02 : Modification de dÃ©cision**

Bonding Brother ne doit jamais modifier une dÃ©cision d'autoritÃ©. Toute modification est une violation.

**Violation VIO-03 : InterprÃ©tation de dÃ©cision**

Bonding Brother ne doit jamais interprÃ©ter une dÃ©cision d'autoritÃ©. Toute interprÃ©tation est une violation.

**Violation VIO-04 : Routage incorrect**

Bonding Brother ne doit jamais router une intention vers la mauvaise autoritÃ©. Tout routage incorrect est une violation.

**Violation VIO-05 : Masquage de contexte**

Bonding Brother ne doit jamais masquer d'information du contexte aux autoritÃ©s. Tout masquage est une violation.

### 11.2 Anti-patterns

**Anti-pattern AP-01 : Cache de dÃ©cisions**

Bonding Brother ne doit jamais mettre en cache des dÃ©cisions d'autoritÃ© pour Ã©viter de les redemander.

**Anti-pattern AP-02 : DÃ©cision par dÃ©faut**

Bonding Brother ne doit jamais prendre une dÃ©cision par dÃ©faut en cas d'indisponibilitÃ© d'autoritÃ©.

**Anti-pattern AP-03 : AgrÃ©gation de dÃ©cisions**

Bonding Brother ne doit jamais agrÃ©ger ou combiner des dÃ©cisions de plusieurs autoritÃ©s.

**Anti-pattern AP-04 : Validation locale**

Bonding Brother ne doit jamais valider localement ce qui doit Ãªtre validÃ© par une autoritÃ©.

---

## 12. Exemples

### 12.1 DÃ©lÃ©gation Ã  Kind Mother

**Intention reÃ§ue :**
```json
{
  "type": "CREATE_CONTENT",
  "payload": { "titre": "Mon article", "contenu": "..." }
}
```

**Routage :** `CREATE_CONTENT` â†’ Kind Mother

**Demande transmise Ã  Kind Mother :**
```json
{
  "type": "create_content",
  "donnÃ©es": { "title": "Mon article", "body": "..." },
  "contexte": { ... }
}
```

**RÃ©ponse de Kind Mother :**
```json
{
  "status": "accepted",
  "data": { "content_id": "content-999" }
}
```

**DÃ©cision prÃ©servÃ©e :** `accepted` â†’ `SUCCÃˆS` (traduit, mais dÃ©cision inchangÃ©e)

### 12.2 DÃ©lÃ©gation Ã  Strong Father

**Intention reÃ§ue :**
```json
{
  "type": "AUTHORIZE",
  "payload": {
    "action": "content:delete",
    "ressource_id": "content-999",
    "utilisateur_id": "user-123"
  }
}
```

**Routage :** `AUTHORIZE` â†’ Strong Father

**Demande transmise Ã  Strong Father :**
```json
{
  "type": "check_permission",
  "action": "content:delete",
  "resource_id": "content-999",
  "user_id": "user-123",
  "contexte": { ... }
}
```

**RÃ©ponse de Strong Father :**
```json
{
  "decision": "authorized",
  "reason": "User has delete permission"
}
```

**DÃ©cision prÃ©servÃ©e :** `authorized` â†’ `AUTORISÃ‰` (traduit, mais dÃ©cision inchangÃ©e)

### 12.3 Mode offline

**Intention reÃ§ue en mode offline :**
```json
{
  "type": "CREATE_CONTENT",
  "payload": { ... }
}
```

**Action :** Journalisation avec marqueur `offline: true`

**Buffer offline :** Intention stockÃ©e dans l'ordre chronologique

**Ã€ la reconnexion :** Transmission Ã  Kind Mother dans l'ordre (FIFO)

**RÃ©ponse diffÃ©rÃ©e :** ReÃ§ue et transmise au produit

---

## 13. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il Ã©tablit les rÃ¨gles de dÃ©lÃ©gation aux autoritÃ©s que Bonding Brother doit respecter pour garantir l'absence de dÃ©cision et la transmission fidÃ¨le.

Toute dÃ©lÃ©gation effectuÃ©e par Bonding Brother doit respecter ce contrat. Toute violation entraÃ®ne une erreur critique.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif  
**DÃ©pendances :** 
- [Documentation Fondatrice v2.0](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) (Section 6)
- [Intent Model Contract v2.0](../intent/BondingBrother%20-%20Intent%20Model%20Contract.md)
- [Bilateral Flow Contract v2.0](../flows/BondingBrother%20-%20Bilateral%20Flow%20Contract.md)
- [Architecture & Flows v2.0](../../architecture/BondingBrother%20-%20Architecture%20%26%20Flows.md)
- [Vocabulary & Glossary v2.0](../../reference/BondingBrother%20-%20Vocabulary%20%26%20Glossary.md)

