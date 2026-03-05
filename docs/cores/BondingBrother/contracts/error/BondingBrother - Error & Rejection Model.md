# BondingBrother - Error & Rejection Model

## 1. Contexte

Ce document dÃ©finit le modÃ¨le contractuel de gestion des erreurs et des rejets dans Bonding Brother. Il spÃ©cifie comment les erreurs sont dÃ©tectÃ©es, classÃ©es, traitÃ©es, et communiquÃ©es aux produits et aux autoritÃ©s.

Ce document complÃ¨te les sections sur les invariants et garanties en dÃ©taillant les cas d'erreur lÃ©gitimes (par opposition aux violations structurelles documentÃ©es dans [Violations & Anti-Patterns](../governance/BondingBrother%20-%20Violations%20&%20Anti-Patterns.md)).

La gestion des erreurs respecte les [Lois d'Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md) : les erreurs de connexion ne sont pas traitÃ©es comme des erreurs critiques mais comme des Ã©tats d'isolement normaux (**LOI-2**), et les erreurs sont traÃ§ables localement mÃªme en mode offline (**LOI-3**).

## 2. PortÃ©e / Scope

Ce document couvre :
- La taxonomie des erreurs et rejets
- Les codes d'erreur canoniques
- Les rÃ¨gles de traitement des erreurs
- La communication des erreurs aux produits
- La communication des erreurs aux autoritÃ©s
- Les stratÃ©gies de rÃ©cupÃ©ration
- Les timeouts et abandonnements

Ce document **ne couvre pas** :
- Les violations structurelles (voir [Violations & Anti-Patterns](../governance/BondingBrother%20-%20Violations%20&%20Anti-Patterns.md))
- Les dÃ©tails d'implÃ©mentation des gestionnaires d'erreur
- Les protocoles de retry spÃ©cifiques (voir [Sync & Reconnection Contract](../offline/BondingBrother%20-%20Sync%20&%20Reconnection%20Contract.md))

---

## 3. Principe fondamental

**Toute erreur doit Ãªtre traÃ§able, comprÃ©hensible, et actionnable.**

Les erreurs ne sont pas des Ã©checs Ã  cacher, mais des Ã©vÃ©nements Ã  documenter, analyser, et communiquer clairement. Bonding Brother garantit la transparence des erreurs tout en protÃ©geant les dÃ©tails internes.

---

## 4. Taxonomie des erreurs

### 4.1 CatÃ©gories d'erreurs

Les erreurs sont organisÃ©es en catÃ©gories selon leur origine et leur nature :

| CatÃ©gorie | Code prÃ©fixe | Origine | ResponsabilitÃ© |
|-----------|--------------|---------|----------------|
| Validation | `VAL_` | Bonding Brother | BB rejette avant traitement |
| Traduction | `TRAD_` | Bonding Brother | BB rejette lors de la traduction |
| Filtrage | `FILT_` | Bonding Brother | BB rejette selon rÃ¨gles |
| AutoritÃ© | `AUTH_` | AutoritÃ© | AutoritÃ© refuse ou erreur |
| SystÃ¨me | `SYS_` | Infrastructure | Erreur technique |
| Timeout | `TIMEOUT_` | Temps | DÃ©lai dÃ©passÃ© |

### 4.2 Types d'erreurs par catÃ©gorie

#### Validation (VAL_)

**VAL-001 : Format invalide**
- L'intention n'est pas un JSON valide
- Le format ne correspond pas au schÃ©ma attendu

**VAL-002 : Champ obligatoire manquant**
- Un champ marquÃ© comme obligatoire est absent

**VAL-003 : Type de donnÃ©es incorrect**
- Un champ a un type incompatible avec le schÃ©ma

**VAL-004 : Version non supportÃ©e**
- La version du schÃ©ma d'intention n'est pas supportÃ©e

**VAL-005 : Type d'intention inconnu**
- Le type d'intention n'est pas reconnu par Bonding Brother

#### Traduction (TRAD_)

**TRAD-001 : Mapping manquant**
- Aucun mapping n'existe pour ce type d'intention vers cette autoritÃ©

**TRAD-002 : Champ non mappable**
- Un champ du payload ne peut pas Ãªtre mappÃ© vers l'autoritÃ©

**TRAD-003 : Transformation impossible**
- La transformation d'un type de donnÃ©es est impossible

**TRAD-004 : Demande invalide aprÃ¨s traduction**
- La demande traduite ne respecte pas le schÃ©ma de l'autoritÃ©

#### Filtrage (FILT_)

**FILT-001 : Intention rejetÃ©e par rÃ¨gle de filtrage**
- Une rÃ¨gle de filtrage d'entrÃ©e rejette l'intention

**FILT-002 : Produit non autorisÃ© pour ce type**
- Le produit n'est pas autorisÃ© Ã  soumettre ce type d'intention

**FILT-003 : Contexte insuffisant**
- Le contexte ne contient pas les informations minimales requises

#### AutoritÃ© (AUTH_)

**AUTH-001 : RefusÃ© par l'autoritÃ©**
- L'autoritÃ© a explicitement refusÃ© la demande

**AUTH-002 : Erreur de l'autoritÃ©**
- L'autoritÃ© a rencontrÃ© une erreur lors du traitement

**AUTH-003 : AutoritÃ© indisponible**
- L'autoritÃ© n'est pas accessible (offline, maintenance)

**AUTH-004 : RÃ©ponse invalide**
- La rÃ©ponse de l'autoritÃ© ne respecte pas le schÃ©ma attendu

#### SystÃ¨me (SYS_)

**SYS-001 : Erreur de journalisation**
- La journalisation a Ã©chouÃ© (critique)

**SYS-002 : Erreur de transmission**
- La transmission vers l'autoritÃ© a Ã©chouÃ© (rÃ©seau, protocole)

**SYS-003 : Erreur interne**
- Une erreur interne inattendue s'est produite

**SYS-004 : Ressource indisponible**
- Une ressource systÃ¨me est indisponible (mÃ©moire, disque)

#### Timeout (TIMEOUT_)

**TIMEOUT-001 : Timeout de transmission**
- La transmission vers l'autoritÃ© a dÃ©passÃ© le dÃ©lai

**TIMEOUT-002 : Timeout d'Ã©valuation**
- L'autoritÃ© n'a pas rÃ©pondu dans le dÃ©lai imparti

**TIMEOUT-003 : Timeout global**
- Le traitement complet a dÃ©passÃ© le dÃ©lai maximum

---

## 5. Structure d'une erreur

### 5.1 Format canonique

```typescript
interface Erreur {
    // Identifiants
    erreur_id: ErreurId;                // ID unique de l'erreur
    intention_id?: IntentionId;          // ID de l'intention (si applicable)
    demande_id?: DemandeId;              // ID de la demande (si applicable)
    
    // Classification
    catÃ©gorie: CatÃ©gorieErreur;         // VAL, TRAD, FILT, AUTH, SYS, TIMEOUT
    code: CodeErreur;                    // Code canonique (ex: VAL-001)
    sÃ©vÃ©ritÃ©: SÃ©vÃ©ritÃ©;                 // CRITIQUE, HAUTE, MOYENNE, BASSE
    
    // Description
    message: string;                     // Message lisible par humain
    message_technique?: string;          // DÃ©tails techniques (optionnel)
    
    // Contexte
    timestamp: Timestamp;                // Moment de l'erreur
    contexte_erreur?: ContexteErreur;   // Contexte supplÃ©mentaire
    
    // TraÃ§abilitÃ©
    corrÃ©lation_id?: CorrÃ©lationId;      // ID pour traÃ§abilitÃ© distribuÃ©e
    stack_trace?: string;               // Stack trace (dÃ©veloppement uniquement)
}
```

### 5.2 SÃ©vÃ©ritÃ© des erreurs

**CRITIQUE :**
- Erreur qui empÃªche le fonctionnement de Bonding Brother
- Exemples : Erreur de journalisation, erreur systÃ¨me majeure
- Action : ArrÃªt immÃ©diat ou mode dÃ©gradÃ©

**HAUTE :**
- Erreur qui empÃªche le traitement d'une intention
- Exemples : Validation Ã©chouÃ©e, traduction impossible
- Action : Rejet de l'intention, notification au produit

**MOYENNE :**
- Erreur qui impacte une fonctionnalitÃ© mais permet la continuation
- Exemples : Erreur de mÃ©trique, timeout partiel
- Action : Journalisation, alerte, continuation

**BASSE :**
- Erreur mineure, non bloquante
- Exemples : MÃ©trique non collectÃ©e, log partiel
- Action : Journalisation, pas d'alerte

---

## 6. Traitement des erreurs

### 6.1 RÃ¨gles de traitement

**RÃ¨gle TRAIT-01 : Journalisation systÃ©matique**

Toute erreur est journalisÃ©e avec :
- L'erreur complÃ¨te (structure + message)
- Le contexte de l'intention (si applicable)
- Le timestamp
- La sÃ©vÃ©ritÃ©

**RÃ¨gle TRAIT-02 : Pas de masquage**

Les erreurs ne sont jamais masquÃ©es ou ignorÃ©es silencieusement (sauf erreurs BASSE en mode production).

**RÃ¨gle TRAIT-03 : Notification appropriÃ©e**

Les erreurs sont notifiÃ©es selon leur sÃ©vÃ©ritÃ© :
- CRITIQUE : Notification immÃ©diate aux administrateurs
- HAUTE : Notification au produit
- MOYENNE : Journalisation et mÃ©triques
- BASSE : Journalisation uniquement

**RÃ¨gle TRAIT-04 : Pas de retry automatique pour erreurs dÃ©finitives**

Les erreurs de validation, traduction, ou filtrage ne sont pas retentÃ©es (ce ne sont pas des erreurs transitoires).

**RÃ¨gle TRAIT-05 : Retry pour erreurs transitoires**

Les erreurs systÃ¨me ou de transmission peuvent Ãªtre retentÃ©es selon une stratÃ©gie configurable.

### 6.2 Flux de traitement d'erreur

```
Erreur dÃ©tectÃ©e
    â”‚
    â–¼
Journalisation
    â”‚
    â–¼
Classification (catÃ©gorie, sÃ©vÃ©ritÃ©)
    â”‚
    â”œâ”€ CRITIQUE â†’ ArrÃªt / Mode dÃ©gradÃ©
    â”œâ”€ HAUTE â†’ Rejet intention + Notification produit
    â”œâ”€ MOYENNE â†’ Journalisation + Alerte
    â””â”€ BASSE â†’ Journalisation
    â”‚
    â–¼
StratÃ©gie de rÃ©cupÃ©ration (si applicable)
    â”‚
    â”œâ”€ Erreur transitoire â†’ Retry
    â””â”€ Erreur dÃ©finitive â†’ Abandon
```

---

## 7. Communication des erreurs aux produits

### 7.1 Format de rÃ©sultat avec erreur

Quand une erreur survient, le produit reÃ§oit un rÃ©sultat avec le statut `ERREUR` :

```typescript
interface RÃ©sultatErreur {
    rÃ©sultat_id: RÃ©sultatId;
    intention_id: IntentionId;
    statut: "ERREUR";
    
    // Erreur
    erreur: {
        code: CodeErreur;                // Code canonique
        message: string;                  // Message lisible
        catÃ©gorie: CatÃ©gorieErreur;      // CatÃ©gorie
    };
    
    // TraÃ§abilitÃ©
    erreur_id: ErreurId;                 // ID pour support
    corrÃ©lation_id?: CorrÃ©lationId;      // ID pour traÃ§abilitÃ©
    
    // MÃ©tadonnÃ©es
    timestamp: Timestamp;
}
```

### 7.2 RÃ¨gles de communication

**RÃ¨gle COMM-PROD-01 : Message lisible**

Le message d'erreur doit Ãªtre comprÃ©hensible par un dÃ©veloppeur de produit, sans exposer les dÃ©tails internes.

**RÃ¨gle COMM-PROD-02 : Code d'erreur**

Le code d'erreur canonique est toujours fourni pour permettre une gestion programmatique.

**RÃ¨gle COMM-PROD-03 : Pas de dÃ©tails internes**

Les stack traces, dÃ©tails d'implÃ©mentation, et informations d'autres produits ne sont jamais exposÃ©s.

**RÃ¨gle COMM-PROD-04 : Actionnable**

Le message doit indiquer ce que le produit peut faire (corriger l'intention, rÃ©essayer plus tard, contacter le support).

### 7.3 Exemples de messages

**Erreur de validation :**
```json
{
  "statut": "ERREUR",
  "erreur": {
    "code": "VAL-002",
    "message": "Le champ 'produit_id' est obligatoire mais absent",
    "catÃ©gorie": "VALIDATION"
  },
  "erreur_id": "err-123",
  "action_suggÃ©rÃ©e": "VÃ©rifier que tous les champs obligatoires sont prÃ©sents"
}
```

**Erreur d'autoritÃ© :**
```json
{
  "statut": "ERREUR",
  "erreur": {
    "code": "AUTH-001",
    "message": "L'autoritÃ© a refusÃ© votre demande",
    "catÃ©gorie": "AUTORITÃ‰"
  },
  "erreur_id": "err-456",
  "action_suggÃ©rÃ©e": "VÃ©rifier vos permissions ou contacter l'administrateur"
}
```

---

## 8. Communication des erreurs aux autoritÃ©s

### 8.1 Erreurs remontÃ©es aux autoritÃ©s

Bonding Brother peut remonter certaines erreurs aux autoritÃ©s pour :
- Informer d'un problÃ¨me de transmission
- Signaler une incohÃ©rence dÃ©tectÃ©e
- Notifier d'un timeout

**Format :**
```typescript
interface NotificationErreurAutoritÃ© {
    notification_id: NotificationId;
    type: "ERREUR_TRANSMISSION" | "ERREUR_TIMEOUT" | "ERREUR_INTERNE";
    demande_id?: DemandeId;
    erreur: {
        code: CodeErreur;
        message: string;
    };
    timestamp: Timestamp;
}
```

### 8.2 RÃ¨gles de communication

**RÃ¨gle COMM-AUTH-01 : Erreurs pertinentes uniquement**

Seules les erreurs pertinentes pour l'autoritÃ© sont remontÃ©es (transmission, timeout, incohÃ©rence).

**RÃ¨gle COMM-AUTH-02 : Pas d'erreurs de validation**

Les erreurs de validation, traduction, ou filtrage ne sont pas remontÃ©es (responsabilitÃ© de BB).

**RÃ¨gle COMM-AUTH-03 : Contexte complet**

Les notifications d'erreur incluent le contexte nÃ©cessaire Ã  l'autoritÃ©.

---

## 9. Rejets

### 9.1 DÃ©finition

Un **rejet** est le refus explicite de traiter une intention, avant mÃªme sa transmission Ã  une autoritÃ©.

### 9.2 Types de rejets

**Rejet de validation :**
- L'intention ne respecte pas le schÃ©ma
- Champs obligatoires manquants
- Types incorrects

**Rejet de traduction :**
- Traduction impossible
- Mapping manquant

**Rejet de filtrage :**
- RÃ¨gle de filtrage rejette l'intention
- Produit non autorisÃ©

### 9.3 Format d'un rejet

Un rejet est communiquÃ© comme une erreur avec le statut `REJETÃ‰` :

```typescript
interface RÃ©sultatRejet {
    rÃ©sultat_id: RÃ©sultatId;
    intention_id: IntentionId;
    statut: "REJETÃ‰";
    
    rejet: {
        code: CodeErreur;                // Code de rejet
        message: string;                  // Raison du rejet
        catÃ©gorie: CatÃ©gorieErreur;      // VAL, TRAD, ou FILT
    };
    
    erreur_id: ErreurId;
    timestamp: Timestamp;
}
```

### 9.4 DiffÃ©rence avec erreur d'autoritÃ©

| Aspect | Rejet (BB) | Erreur d'autoritÃ© |
|--------|------------|-------------------|
| Moment | Avant transmission | AprÃ¨s transmission |
| ResponsabilitÃ© | Bonding Brother | AutoritÃ© |
| Retry | Non (erreur dÃ©finitive) | Possible (selon type) |
| Notification autoritÃ© | Non | Oui (si nÃ©cessaire) |

---

## 10. StratÃ©gies de rÃ©cupÃ©ration

### 10.1 Erreurs transitoires

**DÃ©finition :** Erreurs qui peuvent Ãªtre rÃ©solues en rÃ©essayant.

**Exemples :**
- Erreur de transmission rÃ©seau
- Timeout temporaire
- AutoritÃ© temporairement indisponible

**StratÃ©gie :**
- Retry avec backoff exponentiel
- Nombre maximum de tentatives configurable
- Journalisation de chaque tentative

### 10.2 Erreurs dÃ©finitives

**DÃ©finition :** Erreurs qui ne peuvent pas Ãªtre rÃ©solues en rÃ©essayant.

**Exemples :**
- Erreur de validation
- Erreur de traduction
- Refus explicite de l'autoritÃ©

**StratÃ©gie :**
- Pas de retry
- Rejet immÃ©diat
- Notification au produit

### 10.3 Mode offline

En mode offline, les intentions sont buffÃ©es et retentÃ©es Ã  la reconnexion. Voir [Offline & Deferred Authority Contract](../offline/BondingBrother%20-%20Offline%20&%20Deferred%20Authority%20Contract.md).

---

## 11. Timeouts et abandonnements

### 11.1 Types de timeouts

**Timeout de transmission :**
- DÃ©lai maximum pour transmettre une demande Ã  l'autoritÃ©
- Configurable par type d'intention

**Timeout d'Ã©valuation :**
- DÃ©lai maximum pour recevoir une rÃ©ponse de l'autoritÃ©
- Configurable par autoritÃ©

**Timeout global :**
- DÃ©lai maximum pour le traitement complet d'une intention
- Inclut toutes les Ã©tapes

### 11.2 Gestion des timeouts

**RÃ¨gle TIMEOUT-01 : Notification**

Un timeout est traitÃ© comme une erreur `TIMEOUT_*` et notifiÃ© au produit.

**RÃ¨gle TIMEOUT-02 : Abandonnement**

Si le timeout global est atteint, l'intention est abandonnÃ©e (Ã©tat `ABANDONNÃ‰E`).

**RÃ¨gle TIMEOUT-03 : Retry possible**

Les timeouts peuvent Ãªtre retentÃ©s si l'intention est toujours valide.

### 11.3 Expiration

**DÃ©finition :** Une intention non rÃ©solue expire aprÃ¨s une durÃ©e configurable (par dÃ©faut : 24 heures).

**Traitement :**
- Passage en Ã©tat `EXPIRÃ‰E`
- Notification au produit
- Nettoyage du buffer (si applicable)

---

## 12. Exemples

### 12.1 Erreur de validation

**Intention invalide :**
```json
{
  "produit_id": "miyukini-cms",
  "type": "CREATE_CONTENT"
  // payload manquant
}
```

**RÃ©sultat d'erreur :**
```json
{
  "rÃ©sultat_id": "res-err-001",
  "intention_id": "int-123",
  "statut": "REJETÃ‰",
  "rejet": {
    "code": "VAL-002",
    "message": "Le champ 'payload' est obligatoire mais absent",
    "catÃ©gorie": "VALIDATION"
  },
  "erreur_id": "err-789",
  "timestamp": "2026-01-26T10:00:00Z"
}
```

### 12.2 Erreur d'autoritÃ©

**RÃ©ponse d'autoritÃ© :**
```json
{
  "status": "denied",
  "reason": "Insufficient permissions"
}
```

**RÃ©sultat d'erreur traduit :**
```json
{
  "rÃ©sultat_id": "res-err-002",
  "intention_id": "int-456",
  "statut": "ERREUR",
  "erreur": {
    "code": "AUTH-001",
    "message": "L'autoritÃ© a refusÃ© votre demande : permissions insuffisantes",
    "catÃ©gorie": "AUTORITÃ‰"
  },
  "erreur_id": "err-790",
  "timestamp": "2026-01-26T10:05:00Z"
}
```

---

## 13. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il Ã©tablit le modÃ¨le d'erreur et de rejet que Bonding Brother doit respecter pour garantir la transparence et la traÃ§abilitÃ© des erreurs.

Toute erreur ou rejet doit suivre ce modÃ¨le. Toute dÃ©viation est considÃ©rÃ©e comme une violation.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif  
**DÃ©pendances :** 
- [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) v2.0
- [Intent Model Contract](../intent/BondingBrother%20-%20Intent%20Model%20Contract.md) v2.0
- [Invariants & Guarantees](../governance/BondingBrother%20-%20Invariants%20&%20Guarantees.md) v2.0
- [Violations & Anti-Patterns](../governance/BondingBrother%20-%20Violations%20&%20Anti-Patterns.md) v2.0

