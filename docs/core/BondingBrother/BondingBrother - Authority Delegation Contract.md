# BondingBrother - Authority Delegation Contract

## 1. Contexte

Ce document définit le contrat de délégation aux autorités dans Bonding Brother. Il spécifie comment Bonding Brother délègue les décisions aux autorités (Kind Mother et Strong Father), comment il transmet les demandes, et comment il gère les réponses.

Ce document complète la Section 6 de la [Documentation Fondatrice](./BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur l'[Intent Model Contract](./BondingBrother%20-%20Intent%20Model%20Contract.md) et le [Bilateral Flow Contract](./BondingBrother%20-%20Bilateral%20Flow%20Contract.md) pour définir les règles précises de délégation.

La délégation respecte **LOI-2** (isolement comme état normal) : en mode offline, la délégation est différée mais les intentions sont préservées localement. Voir les [Lois d'Autonomie Système](../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md).

## 2. Portée / Scope

Ce document couvre :
- Les principes fondamentaux de la délégation
- Les règles de délégation à Kind Mother
- Les règles de délégation à Strong Father
- Le routage vers l'autorité appropriée
- La transmission fidèle des demandes et réponses
- La gestion de l'autorité différée (mode offline)
- Les garanties de non-décision

Ce document **ne couvre pas** :
- Les détails d'intégration avec Kind Mother (voir KindMother Integration Contract)
- Les détails d'intégration avec Strong Father (voir StrongFather Integration Contract)
- Les règles de traduction (voir Translation Contract)
- Les règles de filtrage (voir Filtering & Projection Contract)
- Le mode offline détaillé (voir Offline & Deferred Authority Contract)

---

## 3. Principe fondamental

**Bonding Brother délègue toute décision aux autorités. Il ne décide jamais, ne crée aucune règle, ne détient aucune vérité. Il transmet fidèlement les demandes et les réponses, sans interprétation ni modification.**

La délégation est totale, systématique, et non négociable. Toute décision appartient exclusivement à une autorité.

---

## 4. Nature de la délégation

### 4.1 Délégation totale

**Règle DELEG-01 : Absence de décision**

Bonding Brother ne prend jamais de décision stratégique, politique, ou opérationnelle. Toute décision est déléguée à une autorité.

**Règle DELEG-02 : Absence de règle**

Bonding Brother ne crée aucune règle. Toutes les règles viennent des autorités ou de l'écosystème. Bonding Brother applique ces règles, mais ne les définit jamais.

**Règle DELEG-03 : Absence de vérité**

Bonding Brother ne détient aucune vérité sur les données, les identités, les permissions, les décisions. Toute vérité vient d'une autorité.

### 4.2 Transmission fidèle

**Règle DELEG-04 : Transmission sans modification**

Bonding Brother transmet les demandes aux autorités sans modification, sans interprétation, sans enrichissement métier.

**Règle DELEG-05 : Réception sans modification**

Bonding Brother reçoit les réponses des autorités sans modification, sans interprétation, sans remplacement.

**Règle DELEG-06 : Préservation du contexte**

Le contexte est transmis intégralement aux autorités, sans filtrage ni modification.

### 4.3 Rôle de médiateur

**Règle DELEG-07 : Canal, pas source**

Bonding Brother est un canal de communication, pas une source de décision. Il transmet, il ne décide pas.

**Règle DELEG-08 : Traducteur, pas décideur**

Bonding Brother traduit les formats et les vocabulaires, mais ne modifie jamais le sens ni la décision.

**Règle DELEG-09 : Filtre, pas créateur**

Bonding Brother filtre les informations selon les règles définies par les autorités, mais ne crée pas ces règles.

---

## 5. Routage vers les autorités

### 5.1 Identification de l'autorité cible

**Règle ROUTE-01 : Un type = une autorité**

Chaque type d'intention cible une et une seule autorité :
- Intentions de données → Kind Mother
- Intentions d'identité/permissions → Strong Father

**Règle ROUTE-02 : Routage par type**

Le routage est déterminé par le type d'intention, pas par le contenu ou le contexte.

**Règle ROUTE-03 : Pas de routage conditionnel**

Le routage n'est jamais conditionnel. Il est déterministe basé sur le type d'intention.

### 5.2 Types d'intentions par autorité

**Kind Mother (autorité des données) :**
- `CREATE_CONTENT`
- `UPDATE_CONTENT`
- `DELETE_CONTENT`
- `READ_CONTENT`
- `QUERY_CONTENT`
- `CREATE_NODE` (hiérarchie)
- `MOVE_NODE` (hiérarchie)
- `DELETE_NODE` (hiérarchie)
- Toute intention liée à la persistance ou à la cohérence des données

**StrongFather (autorité des décisions stratégiques et politiques) :**
- `AUTHENTICATE`
- `AUTHORIZE`
- `CREATE_SESSION`
- `REVOKE_SESSION`
- `CHECK_PERMISSION`
- Toute intention liée aux identités, permissions, ou décisions politiques

**Règle ROUTE-04 : Pas d'intention multi-autorité**

Une intention ne peut jamais cibler plusieurs autorités simultanément. Si une opération nécessite plusieurs autorités, elle doit être décomposée en plusieurs intentions.

### 5.3 Gestion des erreurs de routage

**Règle ROUTE-05 : Type inconnu**

Si le type d'intention n'est pas reconnu ou ne mappe vers aucune autorité, l'intention est rejetée immédiatement.

**Règle ROUTE-06 : Autorité indisponible**

Si l'autorité cible est indisponible, l'intention est mise en buffer offline (voir Offline & Deferred Authority Contract).

---

## 6. Délégation à Kind Mother

### 6.1 Domaine de délégation

**Kind Mother est l'autorité des données :**
- Persistance des données
- Cohérence des données
- Intégrité des données
- Gestion des hiérarchies
- Gestion des relations

**Règle KM-01 : Décisions de persistance**

Toute décision concernant la persistance, la modification, ou la suppression de données est déléguée à Kind Mother.

**Règle KM-02 : Décisions de cohérence**

Toute décision concernant la cohérence, l'intégrité, ou les contraintes de données est déléguée à Kind Mother.

**Règle KM-03 : Décisions de structure**

Toute décision concernant la structure, la hiérarchie, ou l'organisation des données est déléguée à Kind Mother.

### 6.2 Transmission à Kind Mother

**Règle KM-04 : Format adapté**

La demande est traduite dans le format et le vocabulaire que Kind Mother comprend.

**Règle KM-05 : Contexte complet**

Le contexte est transmis intégralement à Kind Mother, sans modification.

**Règle KM-06 : Pas d'interprétation**

Bonding Brother ne modifie jamais la demande avant transmission. Il traduit le format, pas le sens.

### 6.3 Réception de Kind Mother

**Règle KM-07 : Réception fidèle**

La réponse de Kind Mother est reçue intégralement, sans modification ni interprétation.

**Règle KM-08 : Préservation de la décision**

La décision de Kind Mother (acceptée, refusée, erreur) est préservée intégralement.

**Règle KM-09 : Transmission au produit**

La réponse est traduite et filtrée avant transmission au produit, mais la décision reste inchangée.

---

## 7. Délégation à Strong Father

### 7.1 Domaine de délégation

**Strong Father est l'autorité des identités et permissions :**
- Authentification
- Autorisation
- Gestion des sessions
- Décisions politiques
- Règles de sécurité

**Règle SF-01 : Décisions d'authentification**

Toute décision concernant l'authentification d'un utilisateur est déléguée à Strong Father.

**Règle SF-02 : Décisions d'autorisation**

Toute décision concernant l'autorisation d'une action est déléguée à Strong Father.

**Règle SF-03 : Décisions politiques**

Toute décision stratégique ou politique est déléguée à Strong Father.

**Règle SF-04 : Décisions de session**

Toute décision concernant la création, la validation, ou la révocation de sessions est déléguée à Strong Father.

### 7.2 Transmission à Strong Father

**Règle SF-05 : Format adapté**

La demande est traduite dans le format et le vocabulaire que Strong Father comprend.

**Règle SF-06 : Contexte complet**

Le contexte est transmis intégralement à Strong Father, sans modification.

**Règle SF-07 : Pas d'interprétation**

Bonding Brother ne modifie jamais la demande avant transmission. Il traduit le format, pas le sens.

### 7.3 Réception de Strong Father

**Règle SF-08 : Réception fidèle**

La réponse de Strong Father est reçue intégralement, sans modification ni interprétation.

**Règle SF-09 : Préservation de la décision**

La décision de Strong Father (autorisé, refusé, erreur) est préservée intégralement.

**Règle SF-10 : Transmission au produit**

La réponse est traduite et filtrée avant transmission au produit, mais la décision reste inchangée.

---

## 8. Autorité différée (mode offline)

### 8.1 Principe

**Règle OFFLINE-01 : Délégation différée**

Quand une autorité n'est pas accessible, la délégation est différée. L'intention est journalisée et transmise lorsque la connexion est rétablie.

**Règle OFFLINE-02 : Pas de décision locale**

Bonding Brother ne prend jamais de décision à la place de l'autorité, même en mode offline.

**Règle OFFLINE-03 : Journalisation systématique**

Toute intention en attente de délégation est journalisée avec un marqueur "offline".

### 8.2 Gestion du buffer offline

**Règle OFFLINE-04 : Stockage temporaire**

Les intentions en attente sont stockées dans un buffer temporaire, ordonné chronologiquement.

**Règle OFFLINE-05 : Transmission à la reconnexion**

Lors de la reconnexion, les intentions sont transmises dans l'ordre (FIFO) à l'autorité.

**Règle OFFLINE-06 : Réception différée**

Les réponses différées sont reçues et transmises aux produits lorsque disponibles.

### 8.3 Garanties en mode offline

**Règle OFFLINE-07 : Aucune perte**

Aucune intention n'est perdue en mode offline. Toutes sont transmises à la reconnexion.

**Règle OFFLINE-08 : Ordre préservé**

L'ordre des intentions est préservé lors de la transmission différée.

**Règle OFFLINE-09 : Traçabilité**

Toutes les intentions en mode offline sont traçables, avec horodatage de création et de transmission.

---

## 9. Transmission fidèle

### 9.1 Principe de fidélité

**Règle FID-01 : Pas de modification**

Bonding Brother ne modifie jamais le contenu d'une demande ou d'une réponse. Il traduit le format, mais préserve le sens.

**Règle FID-02 : Pas d'interprétation**

Bonding Brother ne interprète jamais une demande ou une réponse. Il transmet fidèlement ce qu'il reçoit.

**Règle FID-03 : Pas d'enrichissement métier**

Bonding Brother n'ajoute jamais d'information métier à une demande ou une réponse. Seuls les enrichissements techniques sont autorisés (métadonnées de traçabilité).

### 9.2 Enrichissements techniques autorisés

**Métadonnées de traçabilité :**
- `intention_id` (pour corrélation)
- `demande_id` (pour traçabilité)
- `timestamp_demande` (pour ordre chronologique)
- `autorité_cible` (pour routage)

**Règle FID-04 : Enrichissements non métier**

Les enrichissements techniques ne modifient jamais le sens de la demande ou de la réponse.

### 9.3 Préservation du contexte

**Règle FID-05 : Contexte intégral**

Le contexte est transmis intégralement aux autorités, sans filtrage ni modification.

**Règle FID-06 : Pas de masquage**

Bonding Brother ne masque jamais d'information du contexte aux autorités.

**Règle FID-07 : Traçabilité complète**

Le contexte complet est journalisé pour traçabilité, même s'il n'est pas utilisé par l'autorité.

---

## 10. Garanties de délégation

### 10.1 Garantie de non-décision

**Engagement :** Bonding Brother ne prend jamais de décision à la place d'une autorité. Toute décision vient exclusivement d'une autorité.

**Mesure :** Vérification structurelle que Bonding Brother n'a pas de logique de décision métier.

### 10.2 Garantie de fidélité

**Engagement :** Les demandes sont transmises fidèlement aux autorités, et les réponses sont transmises fidèlement aux produits (après traduction et filtrage).

**Mesure :** Tests de round-trip avec vérification que les décisions sont préservées.

### 10.3 Garantie de complétude

**Engagement :** Toute demande est transmise à l'autorité appropriée, et toute réponse est transmise au produit (même en mode offline).

**Mesure :** Traçabilité complète avec vérification que chaque demande a une réponse.

### 10.4 Garantie de routage correct

**Engagement :** Chaque intention est routée vers la bonne autorité, sans erreur de routage.

**Mesure :** Tests avec tous les types d'intentions vérifiant le routage correct.

### 10.5 Garantie de disponibilité

**Engagement :** La délégation fonctionne même en mode offline, avec transmission différée à la reconnexion.

**Mesure :** Tests de fonctionnement offline avec vérification de la transmission différée.

---

## 11. Violations et anti-patterns

### 11.1 Violations interdites

**Violation VIO-01 : Décision par Bonding Brother**

Bonding Brother ne doit jamais prendre de décision métier. Toute logique de décision est une violation.

**Violation VIO-02 : Modification de décision**

Bonding Brother ne doit jamais modifier une décision d'autorité. Toute modification est une violation.

**Violation VIO-03 : Interprétation de décision**

Bonding Brother ne doit jamais interpréter une décision d'autorité. Toute interprétation est une violation.

**Violation VIO-04 : Routage incorrect**

Bonding Brother ne doit jamais router une intention vers la mauvaise autorité. Tout routage incorrect est une violation.

**Violation VIO-05 : Masquage de contexte**

Bonding Brother ne doit jamais masquer d'information du contexte aux autorités. Tout masquage est une violation.

### 11.2 Anti-patterns

**Anti-pattern AP-01 : Cache de décisions**

Bonding Brother ne doit jamais mettre en cache des décisions d'autorité pour éviter de les redemander.

**Anti-pattern AP-02 : Décision par défaut**

Bonding Brother ne doit jamais prendre une décision par défaut en cas d'indisponibilité d'autorité.

**Anti-pattern AP-03 : Agrégation de décisions**

Bonding Brother ne doit jamais agréger ou combiner des décisions de plusieurs autorités.

**Anti-pattern AP-04 : Validation locale**

Bonding Brother ne doit jamais valider localement ce qui doit être validé par une autorité.

---

## 12. Exemples

### 12.1 Délégation à Kind Mother

**Intention reçue :**
```json
{
  "type": "CREATE_CONTENT",
  "payload": { "titre": "Mon article", "contenu": "..." }
}
```

**Routage :** `CREATE_CONTENT` → Kind Mother

**Demande transmise à Kind Mother :**
```json
{
  "type": "create_content",
  "données": { "title": "Mon article", "body": "..." },
  "contexte": { ... }
}
```

**Réponse de Kind Mother :**
```json
{
  "status": "accepted",
  "data": { "content_id": "content-999" }
}
```

**Décision préservée :** `accepted` → `SUCCÈS` (traduit, mais décision inchangée)

### 12.2 Délégation à Strong Father

**Intention reçue :**
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

**Routage :** `AUTHORIZE` → Strong Father

**Demande transmise à Strong Father :**
```json
{
  "type": "check_permission",
  "action": "content:delete",
  "resource_id": "content-999",
  "user_id": "user-123",
  "contexte": { ... }
}
```

**Réponse de Strong Father :**
```json
{
  "decision": "authorized",
  "reason": "User has delete permission"
}
```

**Décision préservée :** `authorized` → `AUTORISÉ` (traduit, mais décision inchangée)

### 12.3 Mode offline

**Intention reçue en mode offline :**
```json
{
  "type": "CREATE_CONTENT",
  "payload": { ... }
}
```

**Action :** Journalisation avec marqueur `offline: true`

**Buffer offline :** Intention stockée dans l'ordre chronologique

**À la reconnexion :** Transmission à Kind Mother dans l'ordre (FIFO)

**Réponse différée :** Reçue et transmise au produit

---

## 13. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit les règles de délégation aux autorités que Bonding Brother doit respecter pour garantir l'absence de décision et la transmission fidèle.

Toute délégation effectuée par Bonding Brother doit respecter ce contrat. Toute violation entraîne une erreur critique.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- Documentation Fondatrice v1.0 (Section 6)
- Intent Model Contract v1.0
- Bilateral Flow Contract v1.0
- Architecture et Composants v1.0
- Glossaire et Terminologie v1.0
