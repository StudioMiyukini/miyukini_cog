# BondingBrother - Error & Rejection Model

## 1. Contexte

Ce document définit le modèle contractuel de gestion des erreurs et des rejets dans Bonding Brother. Il spécifie comment les erreurs sont détectées, classées, traitées, et communiquées aux produits et aux autorités.

Ce document complète les sections sur les invariants et garanties en détaillant les cas d'erreur légitimes (par opposition aux violations structurelles documentées dans [Violations & Anti-Patterns](../governance/BondingBrother%20-%20Violations%20&%20Anti-Patterns.md)).

La gestion des erreurs respecte les [Lois d'Autonomie Système](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) : les erreurs de connexion ne sont pas traitées comme des erreurs critiques mais comme des états d'isolement normaux (**LOI-2**), et les erreurs sont traçables localement même en mode offline (**LOI-3**).

## 2. Portée / Scope

Ce document couvre :
- La taxonomie des erreurs et rejets
- Les codes d'erreur canoniques
- Les règles de traitement des erreurs
- La communication des erreurs aux produits
- La communication des erreurs aux autorités
- Les stratégies de récupération
- Les timeouts et abandonnements

Ce document **ne couvre pas** :
- Les violations structurelles (voir [Violations & Anti-Patterns](../governance/BondingBrother%20-%20Violations%20&%20Anti-Patterns.md))
- Les détails d'implémentation des gestionnaires d'erreur
- Les protocoles de retry spécifiques (voir [Sync & Reconnection Contract](../offline/BondingBrother%20-%20Sync%20&%20Reconnection%20Contract.md))

---

## 3. Principe fondamental

**Toute erreur doit être traçable, compréhensible, et actionnable.**

Les erreurs ne sont pas des échecs à cacher, mais des événements à documenter, analyser, et communiquer clairement. Bonding Brother garantit la transparence des erreurs tout en protégeant les détails internes.

---

## 4. Taxonomie des erreurs

### 4.1 Catégories d'erreurs

Les erreurs sont organisées en catégories selon leur origine et leur nature :

| Catégorie | Code préfixe | Origine | Responsabilité |
|-----------|--------------|---------|----------------|
| Validation | `VAL_` | Bonding Brother | BB rejette avant traitement |
| Traduction | `TRAD_` | Bonding Brother | BB rejette lors de la traduction |
| Filtrage | `FILT_` | Bonding Brother | BB rejette selon règles |
| Autorité | `AUTH_` | Autorité | Autorité refuse ou erreur |
| Système | `SYS_` | Infrastructure | Erreur technique |
| Timeout | `TIMEOUT_` | Temps | Délai dépassé |

### 4.2 Types d'erreurs par catégorie

#### Validation (VAL_)

**VAL-001 : Format invalide**
- L'intention n'est pas un JSON valide
- Le format ne correspond pas au schéma attendu

**VAL-002 : Champ obligatoire manquant**
- Un champ marqué comme obligatoire est absent

**VAL-003 : Type de données incorrect**
- Un champ a un type incompatible avec le schéma

**VAL-004 : Version non supportée**
- La version du schéma d'intention n'est pas supportée

**VAL-005 : Type d'intention inconnu**
- Le type d'intention n'est pas reconnu par Bonding Brother

#### Traduction (TRAD_)

**TRAD-001 : Mapping manquant**
- Aucun mapping n'existe pour ce type d'intention vers cette autorité

**TRAD-002 : Champ non mappable**
- Un champ du payload ne peut pas être mappé vers l'autorité

**TRAD-003 : Transformation impossible**
- La transformation d'un type de données est impossible

**TRAD-004 : Demande invalide après traduction**
- La demande traduite ne respecte pas le schéma de l'autorité

#### Filtrage (FILT_)

**FILT-001 : Intention rejetée par règle de filtrage**
- Une règle de filtrage d'entrée rejette l'intention

**FILT-002 : Produit non autorisé pour ce type**
- Le produit n'est pas autorisé à soumettre ce type d'intention

**FILT-003 : Contexte insuffisant**
- Le contexte ne contient pas les informations minimales requises

#### Autorité (AUTH_)

**AUTH-001 : Refusé par l'autorité**
- L'autorité a explicitement refusé la demande

**AUTH-002 : Erreur de l'autorité**
- L'autorité a rencontré une erreur lors du traitement

**AUTH-003 : Autorité indisponible**
- L'autorité n'est pas accessible (offline, maintenance)

**AUTH-004 : Réponse invalide**
- La réponse de l'autorité ne respecte pas le schéma attendu

#### Système (SYS_)

**SYS-001 : Erreur de journalisation**
- La journalisation a échoué (critique)

**SYS-002 : Erreur de transmission**
- La transmission vers l'autorité a échoué (réseau, protocole)

**SYS-003 : Erreur interne**
- Une erreur interne inattendue s'est produite

**SYS-004 : Ressource indisponible**
- Une ressource système est indisponible (mémoire, disque)

#### Timeout (TIMEOUT_)

**TIMEOUT-001 : Timeout de transmission**
- La transmission vers l'autorité a dépassé le délai

**TIMEOUT-002 : Timeout d'évaluation**
- L'autorité n'a pas répondu dans le délai imparti

**TIMEOUT-003 : Timeout global**
- Le traitement complet a dépassé le délai maximum

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
    catégorie: CatégorieErreur;         // VAL, TRAD, FILT, AUTH, SYS, TIMEOUT
    code: CodeErreur;                    // Code canonique (ex: VAL-001)
    sévérité: Sévérité;                 // CRITIQUE, HAUTE, MOYENNE, BASSE
    
    // Description
    message: string;                     // Message lisible par humain
    message_technique?: string;          // Détails techniques (optionnel)
    
    // Contexte
    timestamp: Timestamp;                // Moment de l'erreur
    contexte_erreur?: ContexteErreur;   // Contexte supplémentaire
    
    // Traçabilité
    corrélation_id?: CorrélationId;      // ID pour traçabilité distribuée
    stack_trace?: string;               // Stack trace (développement uniquement)
}
```

### 5.2 Sévérité des erreurs

**CRITIQUE :**
- Erreur qui empêche le fonctionnement de Bonding Brother
- Exemples : Erreur de journalisation, erreur système majeure
- Action : Arrêt immédiat ou mode dégradé

**HAUTE :**
- Erreur qui empêche le traitement d'une intention
- Exemples : Validation échouée, traduction impossible
- Action : Rejet de l'intention, notification au produit

**MOYENNE :**
- Erreur qui impacte une fonctionnalité mais permet la continuation
- Exemples : Erreur de métrique, timeout partiel
- Action : Journalisation, alerte, continuation

**BASSE :**
- Erreur mineure, non bloquante
- Exemples : Métrique non collectée, log partiel
- Action : Journalisation, pas d'alerte

---

## 6. Traitement des erreurs

### 6.1 Règles de traitement

**Règle TRAIT-01 : Journalisation systématique**

Toute erreur est journalisée avec :
- L'erreur complète (structure + message)
- Le contexte de l'intention (si applicable)
- Le timestamp
- La sévérité

**Règle TRAIT-02 : Pas de masquage**

Les erreurs ne sont jamais masquées ou ignorées silencieusement (sauf erreurs BASSE en mode production).

**Règle TRAIT-03 : Notification appropriée**

Les erreurs sont notifiées selon leur sévérité :
- CRITIQUE : Notification immédiate aux administrateurs
- HAUTE : Notification au produit
- MOYENNE : Journalisation et métriques
- BASSE : Journalisation uniquement

**Règle TRAIT-04 : Pas de retry automatique pour erreurs définitives**

Les erreurs de validation, traduction, ou filtrage ne sont pas retentées (ce ne sont pas des erreurs transitoires).

**Règle TRAIT-05 : Retry pour erreurs transitoires**

Les erreurs système ou de transmission peuvent être retentées selon une stratégie configurable.

### 6.2 Flux de traitement d'erreur

```
Erreur détectée
    │
    ▼
Journalisation
    │
    ▼
Classification (catégorie, sévérité)
    │
    ├─ CRITIQUE → Arrêt / Mode dégradé
    ├─ HAUTE → Rejet intention + Notification produit
    ├─ MOYENNE → Journalisation + Alerte
    └─ BASSE → Journalisation
    │
    ▼
Stratégie de récupération (si applicable)
    │
    ├─ Erreur transitoire → Retry
    └─ Erreur définitive → Abandon
```

---

## 7. Communication des erreurs aux produits

### 7.1 Format de résultat avec erreur

Quand une erreur survient, le produit reçoit un résultat avec le statut `ERREUR` :

```typescript
interface RésultatErreur {
    résultat_id: RésultatId;
    intention_id: IntentionId;
    statut: "ERREUR";
    
    // Erreur
    erreur: {
        code: CodeErreur;                // Code canonique
        message: string;                  // Message lisible
        catégorie: CatégorieErreur;      // Catégorie
    };
    
    // Traçabilité
    erreur_id: ErreurId;                 // ID pour support
    corrélation_id?: CorrélationId;      // ID pour traçabilité
    
    // Métadonnées
    timestamp: Timestamp;
}
```

### 7.2 Règles de communication

**Règle COMM-PROD-01 : Message lisible**

Le message d'erreur doit être compréhensible par un développeur de produit, sans exposer les détails internes.

**Règle COMM-PROD-02 : Code d'erreur**

Le code d'erreur canonique est toujours fourni pour permettre une gestion programmatique.

**Règle COMM-PROD-03 : Pas de détails internes**

Les stack traces, détails d'implémentation, et informations d'autres produits ne sont jamais exposés.

**Règle COMM-PROD-04 : Actionnable**

Le message doit indiquer ce que le produit peut faire (corriger l'intention, réessayer plus tard, contacter le support).

### 7.3 Exemples de messages

**Erreur de validation :**
```json
{
  "statut": "ERREUR",
  "erreur": {
    "code": "VAL-002",
    "message": "Le champ 'produit_id' est obligatoire mais absent",
    "catégorie": "VALIDATION"
  },
  "erreur_id": "err-123",
  "action_suggérée": "Vérifier que tous les champs obligatoires sont présents"
}
```

**Erreur d'autorité :**
```json
{
  "statut": "ERREUR",
  "erreur": {
    "code": "AUTH-001",
    "message": "L'autorité a refusé votre demande",
    "catégorie": "AUTORITÉ"
  },
  "erreur_id": "err-456",
  "action_suggérée": "Vérifier vos permissions ou contacter l'administrateur"
}
```

---

## 8. Communication des erreurs aux autorités

### 8.1 Erreurs remontées aux autorités

Bonding Brother peut remonter certaines erreurs aux autorités pour :
- Informer d'un problème de transmission
- Signaler une incohérence détectée
- Notifier d'un timeout

**Format :**
```typescript
interface NotificationErreurAutorité {
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

### 8.2 Règles de communication

**Règle COMM-AUTH-01 : Erreurs pertinentes uniquement**

Seules les erreurs pertinentes pour l'autorité sont remontées (transmission, timeout, incohérence).

**Règle COMM-AUTH-02 : Pas d'erreurs de validation**

Les erreurs de validation, traduction, ou filtrage ne sont pas remontées (responsabilité de BB).

**Règle COMM-AUTH-03 : Contexte complet**

Les notifications d'erreur incluent le contexte nécessaire à l'autorité.

---

## 9. Rejets

### 9.1 Définition

Un **rejet** est le refus explicite de traiter une intention, avant même sa transmission à une autorité.

### 9.2 Types de rejets

**Rejet de validation :**
- L'intention ne respecte pas le schéma
- Champs obligatoires manquants
- Types incorrects

**Rejet de traduction :**
- Traduction impossible
- Mapping manquant

**Rejet de filtrage :**
- Règle de filtrage rejette l'intention
- Produit non autorisé

### 9.3 Format d'un rejet

Un rejet est communiqué comme une erreur avec le statut `REJETÉ` :

```typescript
interface RésultatRejet {
    résultat_id: RésultatId;
    intention_id: IntentionId;
    statut: "REJETÉ";
    
    rejet: {
        code: CodeErreur;                // Code de rejet
        message: string;                  // Raison du rejet
        catégorie: CatégorieErreur;      // VAL, TRAD, ou FILT
    };
    
    erreur_id: ErreurId;
    timestamp: Timestamp;
}
```

### 9.4 Différence avec erreur d'autorité

| Aspect | Rejet (BB) | Erreur d'autorité |
|--------|------------|-------------------|
| Moment | Avant transmission | Après transmission |
| Responsabilité | Bonding Brother | Autorité |
| Retry | Non (erreur définitive) | Possible (selon type) |
| Notification autorité | Non | Oui (si nécessaire) |

---

## 10. Stratégies de récupération

### 10.1 Erreurs transitoires

**Définition :** Erreurs qui peuvent être résolues en réessayant.

**Exemples :**
- Erreur de transmission réseau
- Timeout temporaire
- Autorité temporairement indisponible

**Stratégie :**
- Retry avec backoff exponentiel
- Nombre maximum de tentatives configurable
- Journalisation de chaque tentative

### 10.2 Erreurs définitives

**Définition :** Erreurs qui ne peuvent pas être résolues en réessayant.

**Exemples :**
- Erreur de validation
- Erreur de traduction
- Refus explicite de l'autorité

**Stratégie :**
- Pas de retry
- Rejet immédiat
- Notification au produit

### 10.3 Mode offline

En mode offline, les intentions sont buffées et retentées à la reconnexion. Voir [Offline & Deferred Authority Contract](../offline/BondingBrother%20-%20Offline%20&%20Deferred%20Authority%20Contract.md).

---

## 11. Timeouts et abandonnements

### 11.1 Types de timeouts

**Timeout de transmission :**
- Délai maximum pour transmettre une demande à l'autorité
- Configurable par type d'intention

**Timeout d'évaluation :**
- Délai maximum pour recevoir une réponse de l'autorité
- Configurable par autorité

**Timeout global :**
- Délai maximum pour le traitement complet d'une intention
- Inclut toutes les étapes

### 11.2 Gestion des timeouts

**Règle TIMEOUT-01 : Notification**

Un timeout est traité comme une erreur `TIMEOUT_*` et notifié au produit.

**Règle TIMEOUT-02 : Abandonnement**

Si le timeout global est atteint, l'intention est abandonnée (état `ABANDONNÉE`).

**Règle TIMEOUT-03 : Retry possible**

Les timeouts peuvent être retentés si l'intention est toujours valide.

### 11.3 Expiration

**Définition :** Une intention non résolue expire après une durée configurable (par défaut : 24 heures).

**Traitement :**
- Passage en état `EXPIRÉE`
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

**Résultat d'erreur :**
```json
{
  "résultat_id": "res-err-001",
  "intention_id": "int-123",
  "statut": "REJETÉ",
  "rejet": {
    "code": "VAL-002",
    "message": "Le champ 'payload' est obligatoire mais absent",
    "catégorie": "VALIDATION"
  },
  "erreur_id": "err-789",
  "timestamp": "2026-01-26T10:00:00Z"
}
```

### 12.2 Erreur d'autorité

**Réponse d'autorité :**
```json
{
  "status": "denied",
  "reason": "Insufficient permissions"
}
```

**Résultat d'erreur traduit :**
```json
{
  "résultat_id": "res-err-002",
  "intention_id": "int-456",
  "statut": "ERREUR",
  "erreur": {
    "code": "AUTH-001",
    "message": "L'autorité a refusé votre demande : permissions insuffisantes",
    "catégorie": "AUTORITÉ"
  },
  "erreur_id": "err-790",
  "timestamp": "2026-01-26T10:05:00Z"
}
```

---

## 13. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit le modèle d'erreur et de rejet que Bonding Brother doit respecter pour garantir la transparence et la traçabilité des erreurs.

Toute erreur ou rejet doit suivre ce modèle. Toute déviation est considérée comme une violation.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) v2.0
- [Intent Model Contract](../intent/BondingBrother%20-%20Intent%20Model%20Contract.md) v2.0
- [Invariants & Guarantees](../governance/BondingBrother%20-%20Invariants%20&%20Guarantees.md) v2.0
- [Violations & Anti-Patterns](../governance/BondingBrother%20-%20Violations%20&%20Anti-Patterns.md) v2.0
