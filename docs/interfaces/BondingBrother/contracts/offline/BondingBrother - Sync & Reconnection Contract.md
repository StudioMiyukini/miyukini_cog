# BondingBrother - Sync & Reconnection Contract

## 1. Contexte

Ce document définit le contrat de synchronisation et de reconnexion de Bonding Brother. Il spécifie comment Bonding Brother gère la synchronisation des intentions journalisées après une période de déconnexion, comment il détecte et gère les reconnexions, et comment il garantit l'intégrité et l'ordre des intentions lors de la synchronisation.

Ce document complète la Section 8 de la [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) sur le rapport au temps et à l'offline, et s'appuie sur les concepts d'offline et de journalisation pour définir le processus de synchronisation.

Ce contrat implémente **LOI-2** (isolement comme état normal) et **LOI-3** (état local souverain) définies dans les [Lois d'Autonomie Système](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md). La synchronisation préserve la souveraineté de l'état local et ne remet jamais en question les décisions prises en isolation.

## 2. Portée / Scope

Ce document couvre :
- La détection de reconnexion
- Le processus de synchronisation des intentions buffées
- La préservation de l'ordre des intentions
- La gestion des conflits et des duplications
- La transmission des résultats différés
- Les stratégies de retry en cas d'échec
- La gestion des timeouts lors de la synchronisation

Ce document **ne couvre pas** :
- Le mode offline lui-même (voir [Offline & Deferred Authority Contract](./BondingBrother%20-%20Offline%20&%20Deferred%20Authority%20Contract.md))
- La journalisation (voir [Journaling Contract](./BondingBrother%20-%20Journaling%20Contract.md))
- Les détails d'implémentation du buffer offline
- Les protocoles réseau de reconnexion

---

## 3. Principe fondamental

**La synchronisation garantit que toutes les intentions exprimées en mode offline sont transmises aux autorités dans l'ordre chronologique, sans perte ni duplication, et que leurs résultats sont transmis aux produits.**

La synchronisation est transparente pour les produits : ils continuent d'exprimer leurs intentions normalement, sans savoir si Bonding Brother est en ligne ou hors ligne.

---

## 4. Détection de reconnexion

### 4.1 Définition

Une **reconnexion** est la restauration de la connectivité entre Bonding Brother et une autorité (Kind Mother ou Strong Father) après une période de déconnexion.

### 4.2 Mécanismes de détection

**Règle DET-01 : Détection active**

Bonding Brother détecte la reconnexion en testant périodiquement la connectivité vers les autorités :
- Test de connectivité vers Kind Mother
- Test de connectivité vers Strong Father
- Fréquence configurable (par défaut : toutes les 30 secondes)

**Règle DET-02 : Détection passive**

Bonding Brother détecte également la reconnexion lors d'une tentative de transmission :
- Si une transmission réussit après un échec, la reconnexion est détectée
- La détection passive est immédiate

**Règle DET-03 : Détection par autorité**

Bonding Brother détecte la reconnexion séparément pour chaque autorité :
- Reconnexion à Kind Mother ≠ Reconnexion à Strong Father
- La synchronisation est déclenchée uniquement pour l'autorité reconnectée

### 4.3 États de connexion

| État | Définition | Action |
|------|-----------|--------|
| **CONNECTÉ** | Connectivité vérifiée récemment | Transmission immédiate |
| **DÉCONNECTÉ** | Dernière tentative échouée | Buffer offline activé |
| **EN_RECONNEXION** | Détection de reconnexion en cours | Synchronisation déclenchée |
| **INSTABLE** | Connexions intermittentes | Mode dégradé avec retry |

---

## 5. Processus de synchronisation

### 5.1 Déclenchement

**Règle SYNC-01 : Déclenchement automatique**

La synchronisation est déclenchée automatiquement dès qu'une reconnexion est détectée pour une autorité donnée.

**Règle SYNC-02 : Déclenchement manuel**

La synchronisation peut également être déclenchée manuellement (API d'administration) pour forcer une synchronisation.

**Règle SYNC-03 : Une autorité à la fois**

La synchronisation est effectuée séparément pour chaque autorité :
- Synchronisation Kind Mother (intentions ciblant KM)
- Synchronisation Strong Father (intentions ciblant SF)

### 5.2 Séquence de synchronisation

```
Détection de reconnexion
    │
    ▼
Récupération des intentions buffées (ordre chronologique)
    │
    ▼
Pour chaque intention (séquentiel) :
    │
    ├─ Transmission à l'autorité
    ├─ Réception de la réponse
    ├─ Journalisation de la réponse
    └─ Transmission du résultat au produit
    │
    ▼
Nettoyage du buffer (intentions synchronisées)
    │
    ▼
Notification de fin de synchronisation
```

### 5.3 Préservation de l'ordre

**Règle ORDRE-01 : Ordre chronologique strict**

Les intentions sont synchronisées dans l'ordre chronologique strict (FIFO) :
- Ordre basé sur le timestamp de création de l'intention
- Aucun réordonnancement n'est autorisé

**Règle ORDRE-02 : Traitement séquentiel**

Les intentions sont transmises séquentiellement (une à la fois) pour préserver l'ordre :
- Pas de transmission parallèle pour une même autorité
- La transmission suivante commence après réception de la réponse

**Règle ORDRE-03 : Blocage en cas d'échec**

Si une intention échoue lors de la synchronisation, les intentions suivantes sont bloquées jusqu'à résolution :
- Retry de l'intention en échec
- Ou passage en erreur définitive
- Puis continuation avec les intentions suivantes

**Note sur l'autonomie :** Cette règle préserve l'ordre tout en respectant **LOI-3** : l'état local reste souverain même si la synchronisation échoue partiellement. Les intentions en échec ne remettent pas en question la validité des intentions suivantes.

### 5.4 Gestion des duplications

**Règle DUP-01 : Détection de duplication**

Bonding Brother détecte les duplications potentielles :
- Vérification de l'ID d'intention avant transmission
- Comparaison avec les intentions déjà synchronisées

**Règle DUP-02 : Prévention de duplication**

Les intentions déjà synchronisées ne sont pas retransmises :
- Marquage des intentions synchronisées
- Exclusion du buffer de synchronisation

**Règle DUP-03 : Gestion des ID dupliqués**

Si une intention avec un ID déjà utilisé est détectée :
- L'intention est rejetée avec code d'erreur `SYNC-001 : ID dupliqué`
- Notification au produit
- Journalisation de l'erreur

---

## 6. Transmission des résultats différés

### 6.1 Résultats différés

Un **résultat différé** est la réponse d'une autorité à une intention qui a été exprimée en mode offline et qui est maintenant évaluée après la reconnexion.

### 6.2 Transmission aux produits

**Règle RES-01 : Transmission immédiate**

Les résultats différés sont transmis aux produits immédiatement après réception de la réponse de l'autorité :
- Pas d'attente de fin de synchronisation complète
- Transmission au fur et à mesure

**Règle RES-02 : Format identique**

Les résultats différés ont le même format que les résultats en ligne :
- Aucune différence de structure
- Marqueur optionnel indiquant que c'est un résultat différé

**Règle RES-03 : Produit indisponible**

Si le produit n'est pas disponible pour recevoir le résultat :
- Le résultat est journalisé
- Retry de transmission selon stratégie configurable
- Notification d'échec si retry échoue

### 6.3 Format de résultat différé

```typescript
interface RésultatDifféré {
    résultat_id: RésultatId;
    intention_id: IntentionId;
    statut: "ACCEPTÉ" | "REFUSÉ" | "ERREUR";
    
    // Résultat normal
    résultat?: RésultatNormal;
    
    // Métadonnées de différé
    différé: {
        créé_le: Timestamp;           // Moment de création de l'intention
        synchronisé_le: Timestamp;     // Moment de synchronisation
        délai: Durée;                   // Délai entre création et synchronisation
    };
    
    timestamp: Timestamp;
}
```

---

## 7. Stratégies de retry

### 7.1 Retry lors de la synchronisation

**Règle RETRY-01 : Retry automatique**

En cas d'échec de transmission lors de la synchronisation, Bonding Brother retente automatiquement :
- Nombre maximum de tentatives configurable (par défaut : 3)
- Backoff exponentiel entre tentatives

**Règle RETRY-02 : Types d'erreurs retentables**

Seules les erreurs transitoires sont retentées :
- Erreurs de transmission réseau
- Timeouts temporaires
- Erreurs d'autorité temporaires (indisponibilité)

**Règle RETRY-03 : Erreurs non retentables**

Les erreurs définitives ne sont pas retentées :
- Erreurs de validation (intention invalide)
- Refus explicite de l'autorité
- Erreurs de format

### 7.2 Backoff exponentiel

**Règle BACKOFF-01 : Délai initial**

Le délai initial entre tentatives est configurable (par défaut : 1 seconde).

**Règle BACKOFF-02 : Multiplicateur**

Le délai est multiplié par un facteur à chaque tentative (par défaut : 2x).

**Règle BACKOFF-03 : Délai maximum**

Le délai maximum est limité (par défaut : 60 secondes).

**Exemple :**
- Tentative 1 : Immédiate
- Tentative 2 : Après 1 seconde
- Tentative 3 : Après 2 secondes
- Tentative 4 : Après 4 secondes
- Tentative 5 : Après 8 secondes (max 60s)

### 7.3 Abandon après échec

**Règle ABANDON-01 : Nombre maximum de tentatives**

Après le nombre maximum de tentatives, l'intention est abandonnée :
- Passage en état `ABANDONNÉE`
- Notification au produit
- Journalisation de l'échec

**Règle ABANDON-02 : Notification au produit**

Le produit est notifié de l'abandon avec :
- Code d'erreur `SYNC-002 : Synchronisation échouée`
- Nombre de tentatives effectuées
- Dernière erreur rencontrée

---

## 8. Gestion des timeouts

### 8.1 Timeout de synchronisation

**Règle TIMEOUT-01 : Timeout par intention**

Chaque intention a un timeout individuel lors de la synchronisation :
- Timeout configurable (par défaut : 30 secondes)
- Timeout global de synchronisation (par défaut : 1 heure)

**Règle TIMEOUT-02 : Gestion du timeout**

Si une intention dépasse son timeout :
- Retry selon stratégie de retry
- Si tous les retry échouent, passage en état `TIMEOUT`

**Règle TIMEOUT-03 : Timeout global**

Si la synchronisation complète dépasse le timeout global :
- Les intentions non synchronisées restent dans le buffer
- Nouvelle tentative de synchronisation déclenchée
- Notification d'incomplétude

---

## 9. Conflits et résolution

### 9.1 Conflits de synchronisation

Un **conflit de synchronisation** survient quand :
- Une intention exprimée en offline entre en conflit avec l'état actuel de l'autorité
- L'autorité rejette l'intention pour cause de conflit
- Plusieurs intentions modifient la même ressource

### 9.2 Résolution des conflits

**Règle CONFLIT-01 : Délégation à l'autorité**

La résolution des conflits est déléguée à l'autorité :
- Bonding Brother transmet l'intention
- L'autorité décide de la résolution
- Bonding Brother transmet la décision au produit

**Règle CONFLIT-02 : Pas de résolution locale**

Bonding Brother ne résout jamais les conflits localement :
- Pas de logique de résolution dans BB
- Pas de modification de l'intention
- Pas de retry avec modification

**Règle CONFLIT-03 : Notification au produit**

En cas de conflit détecté par l'autorité :
- Le produit est notifié avec code d'erreur `SYNC-003 : Conflit détecté`
- Le produit peut soumettre une nouvelle intention corrigée

---

## 10. Métriques et observabilité

### 10.1 Métriques de synchronisation

Bonding Brother expose les métriques suivantes :
- Nombre d'intentions en attente de synchronisation
- Nombre d'intentions synchronisées avec succès
- Nombre d'intentions en échec de synchronisation
- Délai moyen de synchronisation
- Durée totale de synchronisation

### 10.2 Événements de synchronisation

Les événements suivants sont journalisés :
- `SYNC_STARTED` : Début de synchronisation
- `SYNC_INTENTION_SENT` : Intention transmise
- `SYNC_INTENTION_SUCCESS` : Intention synchronisée avec succès
- `SYNC_INTENTION_FAILED` : Échec de synchronisation d'une intention
- `SYNC_COMPLETED` : Fin de synchronisation
- `SYNC_PARTIAL` : Synchronisation partielle (certaines intentions en échec)

---

## 11. Exemples

### 11.1 Synchronisation réussie

**Scénario :** 5 intentions buffées, reconnexion détectée, toutes synchronisées avec succès.

**Événements :**
1. Reconnexion détectée à Kind Mother
2. 5 intentions récupérées (ordre chronologique)
3. Intention 1 transmise → Réponse reçue → Résultat envoyé au produit
4. Intention 2 transmise → Réponse reçue → Résultat envoyé au produit
5. ... (répété pour les 5 intentions)
6. Synchronisation complétée

### 11.2 Synchronisation avec échec partiel

**Scénario :** 5 intentions buffées, 3 réussies, 2 en échec après retry.

**Événements :**
1. Reconnexion détectée
2. Intention 1 : Succès
3. Intention 2 : Succès
4. Intention 3 : Échec (retry 1) → Échec (retry 2) → Échec (retry 3) → Abandonnée
5. Intention 4 : Succès
6. Intention 5 : Échec (retry 1) → Échec (retry 2) → Échec (retry 3) → Abandonnée
7. Synchronisation partielle complétée (3/5)

---

## 12. Contraintes et limites

### 12.1 Taille du buffer

**Règle LIM-01 : Taille maximale**

La taille maximale du buffer offline est configurable (par défaut : 10 000 intentions).

**Règle LIM-02 : Buffer plein**

Si le buffer est plein :
- Les nouvelles intentions sont rejetées avec code `SYNC-004 : Buffer plein`
- Notification au produit
- Journalisation de l'erreur

### 12.2 Durée de rétention

**Règle LIM-03 : Expiration**

Les intentions non synchronisées expirent après une durée configurable (par défaut : 7 jours).

**Règle LIM-04 : Nettoyage**

Les intentions expirées sont nettoyées automatiquement :
- Passage en état `EXPIRÉE`
- Notification au produit
- Suppression du buffer

---

## 13. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit le processus de synchronisation et de reconnexion que Bonding Brother doit respecter pour garantir l'intégrité des intentions en mode offline.

Toute synchronisation doit suivre ce contrat. Toute déviation est considérée comme une violation.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) v2.0 (Section 8)
- [Offline & Deferred Authority Contract](./BondingBrother%20-%20Offline%20&%20Deferred%20Authority%20Contract.md) v2.0
- [Journaling Contract](./BondingBrother%20-%20Journaling%20Contract.md) v2.0
- [Intent Model Contract](../intent/BondingBrother%20-%20Intent%20Model%20Contract.md) v2.0
- [Error & Rejection Model](../error/BondingBrother%20-%20Error%20&%20Rejection%20Model.md) v2.0
