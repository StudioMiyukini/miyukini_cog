# BondingBrother - Offline & Deferred Authority Contract

## 1. Contexte

Ce document définit le contrat de gestion du mode hors ligne et de l'autorité différée dans Bonding Brother. Il spécifie comment Bonding Brother gère les intentions lorsque la connexion aux autorités n'est pas disponible, et comment l'évaluation par les autorités peut être reportée sans compromettre l'intégrité du système.

Ce document complète la Section 8 de la [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur l'[Architecture & Flows](../../architecture/BondingBrother%20-%20Architecture%20&%20Flows.md) et l'[Intent Model Contract](../intent/BondingBrother%20-%20Intent%20Model%20Contract.md).

Ce contrat est une implémentation directe de **LOI-2** (le système accepte l'isolement comme état normal) définie dans les [Lois d'Autonomie Système](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md). L'isolement n'est pas une erreur, mais un état valide du système.

## 2. Portée / Scope

Ce document couvre :
- Le fonctionnement en mode hors ligne
- Le mécanisme d'autorité différée
- La gestion du buffer offline
- Les règles de journalisation en mode offline
- La synchronisation à la reconnexion

Ce document **ne couvre pas** :
- Les détails de journalisation (voir [Journaling Contract](./BondingBrother%20-%20Journaling%20Contract.md))
- Les détails de synchronisation (voir [Sync & Reconnection Contract](./BondingBrother%20-%20Sync%20&%20Reconnection%20Contract.md))
- Les règles de délégation (voir [Authority Delegation Contract](../authority/BondingBrother%20-%20Authority%20Delegation%20Contract.md))

---

## 3. Principe fondamental

**Bonding Brother continue de fonctionner en mode hors ligne, mais l'autorité est différée jusqu'à la reconnexion.**

En mode hors ligne, Bonding Brother reçoit, traduit et journalise les intentions normalement, mais leur transmission aux autorités est différée. L'évaluation par les autorités se fait lors de la reconnexion, et les résultats sont transmis aux produits de manière différée.

---

## 4. Concepts fondamentaux

### 4.1 Mode hors ligne

**Définition :** État où Bonding Brother ne peut pas communiquer avec une ou plusieurs autorités (Kind Mother ou Strong Father).

**Causes possibles :**
- Perte de connectivité réseau
- Autorité temporairement indisponible
- Maintenance planifiée d'une autorité
- Dégradation de service

**Caractéristiques :**
- Bonding Brother continue de recevoir des intentions
- Les intentions sont traduites normalement
- Les intentions sont journalisées normalement
- Les intentions sont mises en buffer pour transmission différée
- Aucune évaluation par l'autorité n'est possible

### 4.2 Autorité différée

**Définition :** Mécanisme par lequel l'évaluation d'une intention par une autorité est reportée à un moment ultérieur (reconnexion).

**Caractéristiques :**
- L'intention est complète et valide
- L'intention est prête pour évaluation
- L'évaluation est simplement reportée dans le temps
- Aucune modification de l'intention n'est nécessaire
- L'ordre chronologique est préservé

### 4.3 Buffer offline

**Définition :** Stockage temporaire des intentions en attente de transmission aux autorités.

**Caractéristiques :**
- Stockage persistant (survit aux redémarrages)
- Ordre préservé (FIFO)
- Traçabilité complète
- Limite de capacité configurable
- Gestion des erreurs de stockage

---

## 5. Fonctionnement en mode hors ligne

### 5.1 Détection du mode hors ligne

**Règle OFFLINE-01 : Détection automatique**

Bonding Brother détecte automatiquement la perte de connexion avec une autorité :
- Par timeout de connexion
- Par erreur de communication
- Par signal explicite d'indisponibilité

**Règle OFFLINE-02 : Détection par autorité**

La détection est indépendante pour chaque autorité :
- Perte de connexion avec Kind Mother n'affecte pas Strong Father
- Perte de connexion avec Strong Father n'affecte pas Kind Mother
- Les intentions pour une autorité disponible continuent d'être traitées normalement

**Règle OFFLINE-03 : Notification aux produits**

Bonding Brother notifie les produits du passage en mode offline :
- Notification immédiate du changement d'état
- Statut de chaque autorité communiqué
- Estimation du délai de reconnexion (si disponible)

### 5.2 Traitement des intentions en mode offline

**Flux normal (en ligne) :**
```
Intention → Traduction → Filtrage → Journalisation → Transmission → Autorité
```

**Flux offline :**
```
Intention → Traduction → Filtrage → Journalisation → Buffer Offline → (attente)
```

**Règle OFFLINE-04 : Traitement identique jusqu'au buffer**

En mode offline, le traitement d'une intention est identique au mode en ligne jusqu'à l'étape de transmission :
- Validation structurelle : identique
- Traduction : identique
- Filtrage : identique
- Journalisation : identique
- Transmission : différée (mise en buffer)

**Règle OFFLINE-05 : Pas de rejet pour cause offline**

Une intention valide n'est jamais rejetée uniquement parce que l'autorité est hors ligne. Elle est mise en buffer pour transmission ultérieure. Cette règle garantit le respect de **LOI-2** : le système ne bloque jamais une opération valide au motif d'une ressource externe indisponible.

### 5.3 Gestion du buffer offline

**Règle BUFFER-01 : Stockage persistant**

Le buffer offline est stocké de manière persistante :
- Survit aux redémarrages de Bonding Brother
- Survit aux pannes système
- Garantit la non-perte d'intentions

**Règle BUFFER-02 : Ordre préservé**

Les intentions sont stockées et transmises dans l'ordre chronologique (FIFO) :
- Ordre basé sur le timestamp de l'intention
- Aucune réorganisation n'est permise
- Ordre strictement respecté lors de la synchronisation

**Règle BUFFER-03 : Limite de capacité**

Le buffer a une limite de capacité configurable :
- Par défaut : 10 000 intentions
- Configurable par environnement
- Gestion du dépassement (voir Section 6.3)

**Règle BUFFER-04 : Métadonnées de buffer**

Chaque intention en buffer contient :
- L'intention complète (structure + payload)
- Le timestamp de mise en buffer
- L'autorité cible
- Le nombre de tentatives de transmission
- Le statut (en attente, en transmission, erreur)

---

## 6. Gestion des contraintes offline

### 6.1 Contrainte CAP-01 : Capacité du buffer

**Problème :** Le buffer offline atteint sa capacité maximale.

**Gestion :**
1. Bonding Brother refuse les nouvelles intentions avec un code d'erreur spécifique
2. Les produits sont notifiés de la saturation du buffer
3. Les produits peuvent retenter après libération d'espace
4. Les intentions en buffer sont priorisées pour transmission

**Code d'erreur :** `BUFFER_FULL`

**Message :** "Le buffer offline est saturé. Veuillez réessayer plus tard."

### 6.2 Contrainte CAP-02 : Expiration des intentions

**Problème :** Une intention reste trop longtemps en buffer sans être transmise.

**Gestion :**
1. Chaque intention a une durée de vie maximale (par défaut : 7 jours)
2. Après expiration, l'intention est marquée comme expirée
3. L'intention expirée n'est plus transmise
4. Le produit est notifié de l'expiration

**Code d'erreur :** `INTENTION_EXPIRED`

**Message :** "L'intention a expiré avant d'être transmise à l'autorité."

### 6.3 Contrainte CAP-03 : Taille des intentions

**Problème :** Une intention est trop volumineuse pour le buffer.

**Gestion :**
1. Validation de la taille avant mise en buffer
2. Rejet immédiat si taille excessive
3. Code d'erreur spécifique
4. Suggestion de découpage si possible

**Code d'erreur :** `INTENTION_TOO_LARGE`

**Message :** "L'intention dépasse la taille maximale autorisée pour le buffer offline."

---

## 7. Reconnexion et synchronisation

### 7.1 Détection de la reconnexion

**Règle RECONN-01 : Détection automatique**

Bonding Brother détecte automatiquement la reconnexion avec une autorité :
- Par succès de connexion
- Par restauration du heartbeat
- Par signal explicite de disponibilité

**Règle RECONN-02 : Détection par autorité**

La détection est indépendante pour chaque autorité :
- Reconnexion avec Kind Mother déclenche la synchronisation des intentions Kind Mother
- Reconnexion avec Strong Father déclenche la synchronisation des intentions Strong Father
- Les synchronisations sont indépendantes

### 7.2 Processus de synchronisation

**Étape 1 : Préparation**
- Récupération des intentions en buffer pour l'autorité concernée
- Tri par ordre chronologique (vérification)
- Préparation des métadonnées de transmission

**Étape 2 : Transmission séquentielle**
- Transmission des intentions une par une dans l'ordre
- Attente de la réponse pour chaque intention
- Gestion des erreurs de transmission

**Étape 3 : Réception des réponses**
- Réception des réponses de l'autorité
- Association réponse → intention
- Traduction des réponses en résultats

**Étape 4 : Transmission aux produits**
- Transmission des résultats aux produits concernés
- Gestion des produits non disponibles
- Journalisation des résultats

**Étape 5 : Nettoyage**
- Suppression des intentions transmises avec succès du buffer
- Conservation des intentions en erreur pour analyse
- Mise à jour des métadonnées

### 7.3 Règles de synchronisation

**Règle SYNC-01 : Ordre strict**

Les intentions sont transmises dans l'ordre chronologique strict, sans parallélisation :
- Une intention à la fois
- Attente de la réponse avant la suivante
- Aucun réordonnancement

**Règle SYNC-02 : Gestion des erreurs**

En cas d'erreur lors de la synchronisation :
- L'intention en erreur est marquée
- Les intentions suivantes continuent d'être transmises
- Les intentions en erreur sont retentées selon une stratégie de retry

**Règle SYNC-03 : Notification aux produits**

Les produits sont notifiés de la reconnexion et de la synchronisation :
- Notification de reconnexion
- Progression de la synchronisation (optionnelle)
- Notification de réception des résultats différés

---

## 8. États et transitions

### 8.1 États du système

```
EN_LIGNE → DÉTECTION_OFFLINE → HORS_LIGNE → DÉTECTION_RECONNEXION → SYNCHRONISATION → EN_LIGNE
```

**EN_LIGNE :**
- Toutes les autorités sont disponibles
- Les intentions sont transmises immédiatement
- Aucune intention en buffer

**DÉTECTION_OFFLINE :**
- Perte de connexion détectée
- Passage en mode offline imminent
- Notification aux produits

**HORS_LIGNE :**
- Une ou plusieurs autorités sont indisponibles
- Les intentions sont mises en buffer
- Les produits sont notifiés

**DÉTECTION_RECONNEXION :**
- Reconnexion détectée
- Préparation de la synchronisation
- Notification aux produits

**SYNCHRONISATION :**
- Transmission des intentions buffées
- Réception des réponses
- Transmission des résultats aux produits

### 8.2 États d'une intention en buffer

```
EN_ATTENTE → EN_TRANSMISSION → TRANSMISE → RÉSULTAT_REÇU → RÉSULTAT_TRANSMIS
               │
               └──→ ERREUR → RETRY → (retour EN_ATTENTE ou ABANDONNÉE)
```

**EN_ATTENTE :**
- Intention en buffer, pas encore transmise
- Prête pour transmission

**EN_TRANSMISSION :**
- Intention en cours de transmission à l'autorité
- Attente de réponse

**TRANSMISE :**
- Intention transmise avec succès
- Réponse attendue

**RÉSULTAT_REÇU :**
- Réponse reçue de l'autorité
- Prête pour traduction et transmission au produit

**RÉSULTAT_TRANSMIS :**
- Résultat transmis au produit
- Intention peut être supprimée du buffer

**ERREUR :**
- Erreur lors de la transmission
- Prête pour retry

**ABANDONNÉE :**
- Trop d'erreurs ou expiration
- Ne sera plus transmise

---

## 9. Garanties et propriétés

### 9.1 Garantie GAR-01 : Non-perte d'intentions

**Énoncé :** Aucune intention valide n'est perdue en mode offline.

**Mécanismes :**
- Stockage persistant du buffer
- Journalisation systématique
- Retry en cas d'erreur
- Vérification d'intégrité

### 9.2 Garantie GAR-02 : Ordre préservé

**Énoncé :** L'ordre chronologique des intentions est préservé lors de la synchronisation.

**Mécanismes :**
- Tri par timestamp
- Transmission séquentielle
- Aucun réordonnancement

### 9.3 Garantie GAR-03 : Traçabilité complète

**Énoncé :** Toute intention en mode offline est traçable de manière complète.

**Mécanismes :**
- Journalisation avec marqueur offline
- Métadonnées de buffer
- Historique de transmission
- Logs d'audit

### 9.4 Garantie GAR-04 : Transparence pour les produits

**Énoncé :** Les produits sont informés de l'état du système et des résultats différés.

**Mécanismes :**
- Notifications d'état
- Transmission des résultats différés
- Codes d'erreur explicites
- Estimation de délai (si disponible)

---

## 10. Configuration

### 10.1 Paramètres configurables

| Paramètre | Description | Valeur par défaut | Unité |
|-----------|-------------|-------------------|-------|
| `buffer.max_capacity` | Capacité maximale du buffer | 10 000 | intentions |
| `buffer.intention_ttl` | Durée de vie maximale d'une intention | 7 | jours |
| `buffer.max_intention_size` | Taille maximale d'une intention | 1 | MB |
| `sync.retry_max_attempts` | Nombre maximum de tentatives | 3 | tentatives |
| `sync.retry_delay` | Délai entre tentatives | 60 | secondes |
| `detection.connection_timeout` | Timeout de connexion | 30 | secondes |
| `detection.heartbeat_interval` | Intervalle de heartbeat | 10 | secondes |

### 10.2 Règles de configuration

**Règle CONFIG-01 : Configuration immuable**

La configuration est immuable après le démarrage de Bonding Brother. Aucune modification dynamique n'est permise.

**Règle CONFIG-02 : Validation au démarrage**

Tous les paramètres sont validés au démarrage. Une configuration invalide empêche le démarrage.

**Règle CONFIG-03 : Documentation**

Tous les paramètres sont documentés avec leur impact et leurs valeurs recommandées.

---

## 11. Exemples

### 11.1 Exemple : Passage en mode offline

```
1. Bonding Brother détecte la perte de connexion avec Kind Mother
2. Bonding Brother passe en mode offline pour Kind Mother
3. Bonding Brother notifie les produits : "Kind Mother est hors ligne"
4. Les nouvelles intentions pour Kind Mother sont mises en buffer
5. Les intentions pour Strong Father continuent normalement
```

### 11.2 Exemple : Synchronisation à la reconnexion

```
1. Bonding Brother détecte la reconnexion avec Kind Mother
2. Bonding Brother notifie les produits : "Kind Mother est de nouveau en ligne"
3. Bonding Brother récupère 150 intentions en buffer pour Kind Mother
4. Bonding Brother transmet les intentions une par une dans l'ordre chronologique
5. Pour chaque intention :
   a. Transmission à Kind Mother
   b. Réception de la réponse
   c. Traduction en résultat
   d. Transmission du résultat au produit concerné
6. Bonding Brother supprime les intentions transmises du buffer
7. Bonding Brother notifie les produits : "Synchronisation terminée"
```

---

## 12. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit les règles de gestion du mode hors ligne et de l'autorité différée que Bonding Brother doit respecter.

Toute implémentation du mode offline doit respecter ces règles. Toute violation compromet l'intégrité et la traçabilité du système.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) v2.0 (Section 8)
- [Architecture & Flows](../../architecture/BondingBrother%20-%20Architecture%20&%20Flows.md) v2.0
- [Intent Model Contract](../intent/BondingBrother%20-%20Intent%20Model%20Contract.md) v2.0
- [Authority Delegation Contract](../authority/BondingBrother%20-%20Authority%20Delegation%20Contract.md) v2.0
