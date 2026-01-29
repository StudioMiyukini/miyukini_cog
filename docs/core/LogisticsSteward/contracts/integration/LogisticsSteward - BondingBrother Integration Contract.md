# LogisticsSteward - BondingBrother Integration Contract

## 1. Contexte

Ce document definit le contrat d'integration entre LogisticsSteward et BondingBrother. Il specifie l'interface, le protocole, les regles de communication, et les garanties associees a la transmission des decisions d'arbitrage vers les entites concernees.

Ce document complete la Section 8.5 de la [Documentation Fondatrice](../../foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md) et s'appuie sur le [Resource Arbitration Contract](../resources/LogisticsSteward%20-%20Resource%20Arbitration%20Contract.md) pour le processus d'arbitrage, et la documentation de BondingBrother pour les specifications du mediateur.

L'integration respecte les [Lois d'Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) : en mode offline, les decisions sont buffees et synchronisees a la reconnexion (**LOI-2**, **LOI-3**).

## 2. Portee / Scope

Ce document couvre :
- L'interface contractuelle entre LogisticsSteward et BondingBrother
- Le protocole de transmission des decisions d'arbitrage
- Les types de decisions transmises
- Les regles de transmission specifiques
- La gestion des erreurs et des acquittements
- Les garanties de l'integration
- Le mode offline et la resilience

Ce document **ne couvre pas** :
- Les details internes de BondingBrother (voir documentation BondingBrother)
- Le processus d'arbitrage (voir [Resource Arbitration Contract](../resources/LogisticsSteward%20-%20Resource%20Arbitration%20Contract.md))
- La validation par StrongFather (voir [StrongFather Integration Contract](./LogisticsSteward%20-%20StrongFather%20Integration%20Contract.md))
- L'execution par le Kernel (voir [Kernel Integration Contract](./LogisticsSteward%20-%20Kernel%20Integration%20Contract.md))

---

## 3. Principe fondamental

**LogisticsSteward emet des decisions d'arbitrage. BondingBrother les transporte fidelement vers les entites concernees, sans jamais les interpreter ni les modifier. La relation est de delegation de transport : LogisticsSteward decide, BondingBrother transmet.**

La transmission est :
- **Fidele** : la decision est transmise sans modification semantique
- **Tracable** : chaque transmission est journalisee
- **Garantie** : BondingBrother acquitte la reception
- **Resiliente** : fonctionne en mode offline avec buffer

---

## 4. Positionnement de BondingBrother

### 4.1 Role de transporteur

**BondingBrother est le transporteur exclusif des decisions d'arbitrage vers :**
- Les Operateurs (applications metier)
- Les Equipes d'Operateurs (groupes logiques)
- Les Services exposes (fonctionnalites utilisateur)
- Les autres cores (MasterButler, WorrySentinel)
- MiyukiniAdmin (avec regles specifiques)

**Regle BB-POS-01 : Transport exclusif**

Toute decision d'arbitrage destinee a une entite externe passe obligatoirement par BondingBrother. Aucune transmission directe n'est autorisee.

**Regle BB-POS-02 : Neutralite de transport**

BondingBrother transporte les decisions sans les interpreter, les modifier, les retarder intentionnellement, ou les filtrer selon son propre jugement.

**Regle BB-POS-03 : Pas de logique d'arbitrage**

BondingBrother ne contient aucune logique d'arbitrage. Il ne peut jamais decider si une allocation est juste ou non. Il transmet fidelement.

---

## 5. Types de decisions transmises

### 5.1 Decision d'arbitrage standard

**DECISION_ARBITRAGE**
- **Contenu :** Verdict d'une demande de ressource
- **Destinataires :** Demandeur, Kernel (pour execution)
- **Payload :** Decision complete avec verdict, quantite, justification

**Exemple :**
```json
{
  "type": "DECISION_ARBITRAGE",
  "decision_id": "dec-2026-005678",
  "demande_id": "dem-2026-001234",
  "destinataire_id": "operateur-cms-01",
  "verdict": "ACCORDE",
  "quantite_accordee": 100,
  "priorite_effective": 5,
  "justification": {
    "raison_principale": "QUOTA_SUFFISANT",
    "regles_appliquees": ["QUOTA-API-001"]
  },
  "conditions": [],
  "timestamp": "2026-01-28T10:30:05Z",
  "validee": true
}
```

### 5.2 Notification de preemption

**NOTIFICATION_PREEMPTION**
- **Contenu :** Avertissement de preemption d'allocation
- **Destinataires :** Entite preemptee
- **Payload :** Details de la preemption, raison, delai

**Exemple :**
```json
{
  "type": "NOTIFICATION_PREEMPTION",
  "notification_id": "notif-2026-001234",
  "destinataire_id": "operateur-cms-01",
  "ressource_preemptee": "REQUETES_API",
  "quantite_liberee": 50,
  "preempteur_type": "ADMIN",
  "raison": "PRIORITE_SUPERIEURE",
  "delai_grace_ms": 5000,
  "timestamp": "2026-01-28T10:35:00Z"
}
```

### 5.3 Notification de changement de quota

**NOTIFICATION_QUOTA**
- **Contenu :** Changement dans les quotas attribues
- **Destinataires :** Entite concernee
- **Payload :** Nouveau quota, raison du changement

**Exemple :**
```json
{
  "type": "NOTIFICATION_QUOTA",
  "notification_id": "notif-2026-001235",
  "destinataire_id": "operateur-cms-01",
  "ressource_type": "REQUETES_API",
  "ancien_quota": 1000,
  "nouveau_quota": 750,
  "raison": "DEGRADATION_D1",
  "effectif_a": "2026-01-28T11:00:00Z",
  "timestamp": "2026-01-28T10:55:00Z"
}
```

### 5.4 Notification de restriction

**NOTIFICATION_RESTRICTION**
- **Contenu :** Application ou levee d'une restriction
- **Destinataires :** Entite concernee
- **Payload :** Type de restriction, duree, conditions de levee

**Exemple :**
```json
{
  "type": "NOTIFICATION_RESTRICTION",
  "notification_id": "notif-2026-001236",
  "destinataire_id": "operateur-cms-01",
  "action": "APPLICATION",
  "restriction_type": "RATE_LIMIT_TEMPORAIRE",
  "raison": "SURCHARGE_DETECTEE",
  "duree_estimee_ms": 300000,
  "conditions_levee": ["charge_normale", "validation_admin"],
  "timestamp": "2026-01-28T11:00:00Z"
}
```

### 5.5 Notification de degradation

**NOTIFICATION_DEGRADATION**
- **Contenu :** Changement de niveau de degradation
- **Destinataires :** Toutes les entites concernees
- **Payload :** Nouveau niveau, impact, recommandations

**Exemple :**
```json
{
  "type": "NOTIFICATION_DEGRADATION",
  "notification_id": "notif-2026-001237",
  "niveau_precedent": "D0",
  "niveau_actuel": "D1",
  "raison": "CHARGE_ELEVEE",
  "impact": {
    "quotas_reduits": "25%",
    "fonctionnalites_desactivees": ["batch_export", "analytics_realtime"]
  },
  "recommandations": [
    "Reduire les operations non critiques",
    "Reporter les traitements batch"
  ],
  "timestamp": "2026-01-28T11:05:00Z"
}
```

---

## 6. Protocole de transmission

### 6.1 Format des messages

Les messages transmis a BondingBrother suivent un format standardise.

**Structure de base :**
```typescript
interface MessageLogisticsSteward {
    message_id: MessageId;
    type: TypeMessage;                    // DECISION_ARBITRAGE, NOTIFICATION_*, etc.
    source: "LogisticsSteward";
    destinataire_id: EntiteId;
    payload: PayloadSpecifique;
    priorite_transmission: number;        // 1-10, 10 = urgence maximale
    contexte: ContexteTransmission;
    timestamp: Timestamp;
    require_ack: boolean;                 // Acquittement requis
}
```

**Regle BB-PROT-01 : Format standardise**

Tous les messages suivent le format standardise. BondingBrother rejette les messages mal formes.

**Regle BB-PROT-02 : Priorite de transmission**

La priorite de transmission (1-10) indique l'urgence. BondingBrother ordonne sa file d'attente selon cette priorite.

**Regle BB-PROT-03 : Contexte obligatoire**

Le contexte contient les informations necessaires a la tracabilite : origine de la decision, arbitrage reference, etc.

### 6.2 Acquittement

**Structure d'acquittement :**
```typescript
interface AcquittementBondingBrother {
    ack_id: AckId;
    message_id: MessageId;
    statut: StatutAck;                    // RECU, TRANSMIS, DELIVRE, ERREUR
    destinataire_id: EntiteId;
    timestamp_reception: Timestamp;
    timestamp_transmission?: Timestamp;
    erreur?: ErreurTransmission;
}
```

**Statuts d'acquittement :**

| Statut | Description |
|--------|-------------|
| **RECU** | BondingBrother a recu le message |
| **TRANSMIS** | Message transmis au destinataire |
| **DELIVRE** | Destinataire a confirme la reception |
| **ERREUR** | Echec de transmission |

**Regle BB-PROT-04 : Acquittement obligatoire**

Tout message avec `require_ack: true` doit recevoir un acquittement de BondingBrother.

**Regle BB-PROT-05 : Timeout d'acquittement**

Si l'acquittement n'est pas recu dans le delai imparti (configurable, defaut 30s), LogisticsSteward retransmets.

### 6.3 Workflow de transmission

```
[LogisticsSteward]
       │
       │ Message avec decision
       ▼
┌─────────────────────┐
│ BondingBrother      │
│ - Reception         │
│ - Validation format │
│ - Ack RECU          │
└──────────┬──────────┘
           │
           │ Traduction si necessaire
           ▼
┌─────────────────────┐
│ BondingBrother      │
│ - Transmission      │
│ - Ack TRANSMIS      │
└──────────┬──────────┘
           │
           │ Confirmation destinataire
           ▼
┌─────────────────────┐
│ BondingBrother      │
│ - Ack DELIVRE       │
└─────────────────────┘
```

---

## 7. Regles de transmission

### 7.1 Regles de priorite

| Code | Regle |
|------|-------|
| **BB-TX-01** | Les decisions MiyukiniAdmin ont priorite maximale (10) |
| **BB-TX-02** | Les notifications de preemption ont priorite haute (8) |
| **BB-TX-03** | Les decisions standard suivent la priorite de l'arbitrage |
| **BB-TX-04** | Les notifications de degradation sont broadcast (tous destinataires) |

### 7.2 Regles de fiabilite

| Code | Regle |
|------|-------|
| **BB-TX-05** | Retry automatique en cas d'echec de transmission (max 3) |
| **BB-TX-06** | Backoff exponentiel entre les retries (1s, 2s, 4s) |
| **BB-TX-07** | Escalade apres echec persistant (alerte WorrySentinel) |
| **BB-TX-08** | Journalisation de chaque tentative de transmission |

### 7.3 Regles de tracabilite

| Code | Regle |
|------|-------|
| **BB-TX-09** | Chaque message a un identifiant unique |
| **BB-TX-10** | Le message reference l'arbitrage source |
| **BB-TX-11** | Les acquittements sont journalises |
| **BB-TX-12** | L'historique est conserve pour audit |

### 7.4 Regles de securite

| Code | Regle |
|------|-------|
| **BB-TX-13** | Les decisions ne sont transmises qu'aux destinataires autorises |
| **BB-TX-14** | Aucune modification du payload en transit |
| **BB-TX-15** | Les credentials ne sont jamais inclus dans les messages |
| **BB-TX-16** | Les messages sensibles sont marques comme tels |

---

## 8. Gestion des erreurs

### 8.1 Erreurs de transmission

| Erreur | Code | Action |
|--------|------|--------|
| BondingBrother indisponible | ERR-BB-001 | Buffer local, retry |
| Message mal forme | ERR-BB-002 | Correction et retransmission |
| Destinataire inconnu | ERR-BB-003 | Escalade, notification WorrySentinel |
| Timeout transmission | ERR-BB-004 | Retry avec backoff |

### 8.2 Erreurs d'acquittement

| Erreur | Code | Action |
|--------|------|--------|
| Timeout acquittement | ERR-BB-010 | Retry du message |
| Acquittement ERREUR | ERR-BB-011 | Analyse cause, action corrective |
| Acquittement incoherent | ERR-BB-012 | Journalisation, alerte |

### 8.3 Traitement des erreurs

**Regle BB-ERR-01 : Pas de perte de decision**

Une decision d'arbitrage validee ne peut jamais etre perdue. En cas d'echec persistant, elle est buffee indefiniment.

**Regle BB-ERR-02 : Notification d'echec**

Apres epuisement des retries, LogisticsSteward notifie WorrySentinel de l'echec de transmission.

**Regle BB-ERR-03 : Journalisation complete**

Toutes les erreurs sont journalisees avec contexte complet pour analyse.

**Regle BB-ERR-04 : Pas de decision alternative**

LogisticsSteward ne prend jamais de decision alternative suite a un echec de transmission. La decision originale reste valide.

---

## 9. Mode offline

### 9.1 Comportement en mode offline

En mode offline, BondingBrother peut etre indisponible. LogisticsSteward :

1. Buffer les decisions dans une file locale
2. Marque les decisions comme "en attente de transmission"
3. Retente la transmission lors de la reconnexion
4. Transmet dans l'ordre chronologique a la reconnexion

**Regle BB-OFFLINE-01 : Buffer systematique**

Toute decision destinee a BondingBrother est buffee si le mediateur est indisponible.

**Regle BB-OFFLINE-02 : Ordre preserve**

L'ordre chronologique des decisions est preserve lors de la transmission differee.

**Regle BB-OFFLINE-03 : Pas d'expiration**

Les decisions d'arbitrage n'expirent jamais dans le buffer. Elles sont transmises des que possible.

**Regle BB-OFFLINE-04 : Notification de retard**

A la reconnexion, les decisions differees sont accompagnees d'une indication de delai.

### 9.2 Reconciliation a la reconnexion

A la reconnexion avec BondingBrother :

1. Verification de l'etat de BondingBrother
2. Transmission des decisions buffees (ordre chronologique)
3. Reception des acquittements
4. Mise a jour des etats locaux
5. Notification des entites du retard eventuel

**Regle BB-OFFLINE-05 : Reconciliation explicite**

La reconciliation est un processus explicite, pas une correction silencieuse.

---

## 10. Garanties de l'integration

### 10.1 Garantie de fidelite (GAR-BB-01)

**Engagement :** La decision transmise a BondingBrother est semantiquement identique a la decision emise par LogisticsSteward.

**Verification :**
- Hash de verification du payload
- Comparaison source/destination
- Tests de non-alteration

### 10.2 Garantie de livraison (GAR-BB-02)

**Engagement :** Toute decision validee sera eventuellement livree au destinataire (best effort avec retries).

**Verification :**
- Suivi des acquittements
- Metriques de livraison
- Alertes en cas d'echec persistant

### 10.3 Garantie de tracabilite (GAR-BB-03)

**Engagement :** Toute transmission est tracable de bout en bout : emission, reception BondingBrother, transmission, livraison.

**Verification :**
- Journal complet par message
- Correlation des identifiants
- Audit possible a posteriori

### 10.4 Garantie de non-interpretation (GAR-BB-04)

**Engagement :** BondingBrother ne modifie jamais le sens d'une decision. Il transporte, il ne juge pas.

**Verification :**
- Tests de non-modification
- Validation des payloads
- Absence de logique d'arbitrage dans BondingBrother

### 10.5 Garantie d'ordre (GAR-BB-05)

**Engagement :** Les decisions pour un meme destinataire sont transmises dans l'ordre d'emission.

**Verification :**
- Sequence de numeros
- Tests d'ordre
- Validation chronologique

### 10.6 Garantie de resilience (GAR-BB-06)

**Engagement :** L'integration fonctionne en mode degrade (offline) avec buffer et reconciliation, selon **LOI-2** des Lois d'Autonomie.

**Verification :**
- Tests en mode deconnecte
- Tests de reconciliation
- Validation du buffer

---

## 11. Invariants de l'integration

| Code | Invariant |
|------|-----------|
| **INV-BB-01** | LogisticsSteward n'envoie que des decisions validees par StrongFather |
| **INV-BB-02** | BondingBrother ne modifie jamais le verdict d'une decision |
| **INV-BB-03** | Chaque transmission a un acquittement (explicite ou timeout) |
| **INV-BB-04** | Les decisions buffees sont transmises dans l'ordre |
| **INV-BB-05** | Aucune decision validee n'est perdue (buffer persistant) |
| **INV-BB-06** | La tracabilite est complete de l'emission a la livraison |
| **INV-BB-07** | Les erreurs de transmission n'invalident pas les decisions |

---

## 12. Performance et limites

### 12.1 Delais

| Metrique | Valeur |
|----------|--------|
| Delai transmission standard | < 100ms |
| Timeout acquittement | 30s (configurable) |
| Delai retry initial | 1s |
| Delai retry max | 4s |

### 12.2 Limites

| Limite | Valeur |
|--------|--------|
| Taille maximale message | 1 MB |
| Messages simultanes | 1000 |
| Buffer offline | 10000 messages |
| Retries par message | 3 |

---

## 13. Exemples

### 13.1 Transmission d'une decision standard

**Message emis :**
```json
{
  "message_id": "msg-2026-001234",
  "type": "DECISION_ARBITRAGE",
  "source": "LogisticsSteward",
  "destinataire_id": "operateur-cms-01",
  "payload": {
    "decision_id": "dec-2026-005678",
    "demande_id": "dem-2026-001234",
    "verdict": "ACCORDE",
    "quantite_accordee": 100,
    "priorite_effective": 5,
    "justification": {
      "raison_principale": "QUOTA_SUFFISANT"
    }
  },
  "priorite_transmission": 5,
  "contexte": {
    "arbitrage_ref": "arb-2026-005678",
    "validation_sf": "val-2026-001234"
  },
  "timestamp": "2026-01-28T10:30:05Z",
  "require_ack": true
}
```

**Acquittement recu :**
```json
{
  "ack_id": "ack-2026-001234",
  "message_id": "msg-2026-001234",
  "statut": "DELIVRE",
  "destinataire_id": "operateur-cms-01",
  "timestamp_reception": "2026-01-28T10:30:05.050Z",
  "timestamp_transmission": "2026-01-28T10:30:05.100Z"
}
```

### 13.2 Notification de preemption

**Message emis :**
```json
{
  "message_id": "msg-2026-001235",
  "type": "NOTIFICATION_PREEMPTION",
  "source": "LogisticsSteward",
  "destinataire_id": "operateur-cms-01",
  "payload": {
    "notification_id": "notif-2026-001234",
    "ressource_preemptee": "REQUETES_API",
    "quantite_liberee": 50,
    "preempteur_type": "ADMIN",
    "raison": "PRIORITE_SUPERIEURE",
    "delai_grace_ms": 5000
  },
  "priorite_transmission": 8,
  "contexte": {
    "preemption_ref": "preemp-2026-001234"
  },
  "timestamp": "2026-01-28T10:35:00Z",
  "require_ack": true
}
```

### 13.3 Scenario offline

**Etape 1 : Decision emise, BondingBrother indisponible**
```
[LogisticsSteward] → Decision emise
[Buffer local] ← Decision stockee (BB indisponible)
[Log] : "Decision dec-2026-005679 buffee, BB offline"
```

**Etape 2 : Reconnexion**
```
[BondingBrother] : Disponible
[LogisticsSteward] : Reconciliation
[Transmission] : Decision dec-2026-005679 (differee)
[Acquittement] : DELIVRE (delai: 300s)
[Log] : "Decision dec-2026-005679 transmise, delai 300s"
```

---

## 14. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il etablit l'interface et le protocole que LogisticsSteward doit respecter pour s'integrer avec BondingBrother.

Toute implementation de l'integration avec BondingBrother doit respecter ce contrat. Toute violation entraine un comportement non conforme.

---

## 15. Documents associes

- [Documentation Fondatrice](../../foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md) (Section 8.5)
- [Index de Navigation](../../_index.md)
- [Resource Arbitration Contract](../resources/LogisticsSteward%20-%20Resource%20Arbitration%20Contract.md)
- [StrongFather Integration Contract](./LogisticsSteward%20-%20StrongFather%20Integration%20Contract.md)
- [Kernel Integration Contract](./LogisticsSteward%20-%20Kernel%20Integration%20Contract.md)
- [BondingBrother - Documentation Fondatrice](../../../BondingBrother/foundation/BondingBrother%20-%20Documentation%20Fondatrice.md)
- [BondingBrother - Architecture & Flows](../../../BondingBrother/architecture/BondingBrother%20-%20Architecture%20&%20Flows.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)

---

**Version :** 1.0.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dependencies :**
- [Documentation Fondatrice](../../foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md) v1.0.0 (Section 8.5)
- [BondingBrother - Documentation Fondatrice](../../../BondingBrother/foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) v2.0
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)
