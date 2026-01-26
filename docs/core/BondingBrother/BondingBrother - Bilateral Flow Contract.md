# BondingBrother - Bilateral Flow Contract

## 1. Contexte

Ce document définit le contrat des flux bilatéraux dans Bonding Brother. Il spécifie comment les communications bidirectionnelles entre les produits et l'écosystème sont orchestrées, avec des règles distinctes pour chaque direction.

Ce document complète la Section 5 de la [Documentation Fondatrice](./BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur l'[Intent Model Contract](./BondingBrother%20-%20Intent%20Model%20Contract.md) et le [Translation Contract](./BondingBrother%20-%20Translation%20Contract.md) pour définir les flux complets.

Ces flux respectent les [Lois d'Autonomie Système](../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md), notamment **LOI-2** (isolement comme état normal) et **LOI-3** (état local souverain) : les flux fonctionnent même en mode offline, et l'état local est préservé.

## 2. Portée / Scope

Ce document couvre :
- La vue d'ensemble des flux bilatéraux
- Le flux Produit → Écosystème (détaillé)
- Le flux Écosystème → Produit (détaillé)
- Les règles d'asymétrie et d'adaptation
- La coordination entre les deux flux
- Les garanties de cohérence

Ce document **ne couvre pas** :
- Les détails du flux Produit → Écosystème (voir Product-to-Ecosystem Flow)
- Les détails du flux Écosystème → Produit (voir Ecosystem-to-Product Flow)
- Les règles de traduction (voir Translation Contract)
- Les règles de filtrage (voir Filtering & Projection Contract)
- La délégation aux autorités (voir Authority Delegation Contract)

---

## 3. Principe fondamental

**Bonding Brother gère deux flux de communication dans deux sens opposés, avec des règles différentes pour chaque sens, garantissant l'asymétrie et l'adaptation.**

L'asymétrie est fondamentale : les produits s'adaptent à Bonding Brother, Bonding Brother s'adapte aux autorités. Les produits ne connaissent pas les détails des autorités, les autorités ne connaissent pas les détails des produits.

---

## 4. Vue d'ensemble des flux

### 4.1 Les deux flux principaux

```
┌─────────────┐                    ┌─────────────┐
│   Produit   │                    │ Écosystème  │
│             │                    │  (Autorités)│
└──────┬──────┘                    └──────┬─────┘
       │                                   │
       │  FLUX ASCENDANT                   │
       │  (Intention)                      │
       │──────────────────────────────────>│
       │                                   │
       │  FLUX DESCENDANT                 │
       │  (Résultat/Notification)          │
       │<──────────────────────────────────│
       │                                   │
```

**Flux ascendant (Produit → Écosystème) :**
- Déclenché par : Expression d'une intention par un produit
- Contenu : Intention structurée
- Destination : Autorité (Kind Mother ou Strong Father)
- Résultat attendu : Résultat de l'évaluation

**Flux descendant (Écosystème → Produit) :**
- Déclenché par : Notification ou événement de l'écosystème
- Contenu : Information ou résultat
- Destination : Produit(s) concerné(s)
- Résultat attendu : Notification reçue

### 4.2 Caractéristiques communes

**Asymétrie :**
- Les produits s'adaptent à Bonding Brother
- Bonding Brother s'adapte aux autorités
- Pas d'adaptation inverse

**Traçabilité :**
- Tous les flux sont journalisés
- Chaque étape est traçable
- Aucune perte d'information de traçabilité

**Sécurité :**
- Validation à chaque étape
- Filtrage systématique
- Isolation des produits

---

## 5. Flux Produit → Écosystème

### 5.1 Vue d'ensemble

Le flux ascendant transporte une intention d'un produit vers une autorité, en passant par les étapes de validation, traduction, filtrage, et transmission.

### 5.2 Étapes du flux

```
Produit
   │
   │ 1. Expression d'intention
   ▼
┌─────────────────────┐
│ ProductGateway      │ ← Réception
└──────────┬──────────┘
           │
           │ 2. Validation structurelle
           ▼
┌─────────────────────┐
│ IntentReceiver       │ ← Validation
└──────────┬──────────┘
           │
           │ 3. Traduction ascendante
           ▼
┌─────────────────────┐
│ IntentTranslator    │ ← Intention → Demande
└──────────┬──────────┘
           │
           │ 4. Filtrage d'entrée
           ▼
┌─────────────────────┐
│ FilterEngine        │ ← Filtrage
└──────────┬──────────┘
           │
           │ 5. Journalisation
           ▼
┌─────────────────────┐
│ JournalWriter       │ ← Journalisation
└──────────┬──────────┘
           │
           │ 6. Routage vers autorité
           ▼
┌─────────────────────┐
│ AuthorityRouter      │ ← Routage
└──────────┬──────────┘
           │
      ┌────┴────┐
      ▼         ▼
┌─────────┐ ┌─────────┐
│KindMother│ │Strong   │ ← Transmission
│          │ │Father   │
└────┬─────┘ └────┬────┘
     └─────┬──────┘
           │
           │ 7. Réception réponse
           ▼
┌─────────────────────┐
│ AuthorityResponse   │ ← Réception
│ Handler             │
└──────────┬──────────┘
           │
           │ 8. Filtrage de sortie
           ▼
┌─────────────────────┐
│ FilterEngine        │ ← Filtrage
└──────────┬──────────┘
           │
           │ 9. Traduction descendante
           ▼
┌─────────────────────┐
│ ResponseTranslator   │ ← Réponse → Résultat
└──────────┬──────────┘
           │
           │ 10. Journalisation résultat
           ▼
┌─────────────────────┐
│ JournalWriter       │ ← Journalisation
└──────────┬──────────┘
           │
           │ 11. Émission vers produit
           ▼
┌─────────────────────┐
│ ResultEmitter       │ ← Émission
└──────────┬──────────┘
           │
           ▼
        Produit
```

### 5.3 Règles du flux ascendant

**Règle FLUX-ASC-01 : Ordre strict**

Les étapes doivent être exécutées dans l'ordre strict défini. Aucune étape ne peut être sautée.

**Règle FLUX-ASC-02 : Validation précoce**

La validation structurelle est effectuée avant toute traduction ou traitement métier.

**Règle FLUX-ASC-03 : Traduction avant filtrage**

La traduction ascendante est effectuée avant le filtrage d'entrée. Le filtrage valide la demande traduite.

**Règle FLUX-ASC-04 : Journalisation systématique**

Chaque étape critique est journalisée :
- Réception de l'intention
- Validation réussie/échouée
- Traduction réussie/échouée
- Filtrage réussi/échoué
- Transmission à l'autorité
- Réception de la réponse
- Émission du résultat

**Règle FLUX-ASC-05 : Routage vers autorité unique**

Chaque intention est routée vers une et une seule autorité (Kind Mother ou Strong Father).

**Règle FLUX-ASC-06 : Transmission fidèle**

La demande transmise à l'autorité est fidèle à l'intention traduite, sans modification ni interprétation.

**Règle FLUX-ASC-07 : Réception complète**

La réponse de l'autorité est reçue intégralement, sans perte ni modification.

**Règle FLUX-ASC-08 : Filtrage avant traduction (sortie)**

Le filtrage de sortie est appliqué avant la traduction descendante.

**Règle FLUX-ASC-09 : Résultat complet**

Le résultat transmis au produit contient toutes les informations nécessaires et autorisées.

### 5.4 Gestion des erreurs dans le flux ascendant

**Erreur de validation :**
- Arrêt du flux
- Rejet immédiat
- Notification au produit avec code d'erreur

**Erreur de traduction :**
- Arrêt du flux
- Rejet immédiat
- Notification au produit avec code d'erreur

**Erreur de filtrage :**
- Arrêt du flux
- Rejet immédiat
- Notification au produit avec code d'erreur

**Erreur de transmission :**
- Retry selon politique (si erreur transitoire)
- Mise en buffer offline (si mode déconnecté)
- Notification au produit avec statut approprié

**Erreur de l'autorité :**
- Réception de l'erreur
- Traduction de l'erreur
- Transmission au produit

---

## 6. Flux Écosystème → Produit

### 6.1 Vue d'ensemble

Le flux descendant transporte une notification ou un événement de l'écosystème vers un ou plusieurs produits, en passant par les étapes de réception, filtrage, traduction, et distribution.

### 6.2 Étapes du flux

```
Autorité (KM ou SF)
   │
   │ 1. Émission notification/événement
   ▼
┌─────────────────────┐
│ AuthorityResponse   │ ← Réception
│ Handler             │
└──────────┬──────────┘
           │
           │ 2. Normalisation
           ▼
┌─────────────────────┐
│ EventNormalizer     │ ← Normalisation
└──────────┬──────────┘
           │
           │ 3. Filtrage de sortie
           ▼
┌─────────────────────┐
│ FilterEngine        │ ← Filtrage
└──────────┬──────────┘
           │
           │ 4. Traduction descendante
           ▼
┌─────────────────────┐
│ ResponseTranslator  │ ← Réponse → Message
└──────────┬──────────┘
           │
           │ 5. Identification produits cibles
           ▼
┌─────────────────────┐
│ ProductSelector     │ ← Sélection produits
└──────────┬──────────┘
           │
           │ 6. Journalisation
           ▼
┌─────────────────────┐
│ JournalWriter       │ ← Journalisation
└──────────┬──────────┘
           │
           │ 7. Distribution
           ▼
┌─────────────────────┐
│ NotificationDispatcher│ ← Distribution
└──────────┬──────────┘
           │
      ┌────┴────┐
      ▼         ▼
┌─────────┐ ┌─────────┐
│Produit A│ │Produit B│ ← Réception
└─────────┘ └─────────┘
```

### 6.3 Règles du flux descendant

**Règle FLUX-DESC-01 : Ordre strict**

Les étapes doivent être exécutées dans l'ordre strict défini. Aucune étape ne peut être sautée.

**Règle FLUX-DESC-02 : Réception complète**

La notification ou l'événement est reçu intégralement de l'autorité, sans perte ni modification.

**Règle FLUX-DESC-03 : Normalisation**

La notification est normalisée dans un format standard avant traitement.

**Règle FLUX-DESC-04 : Filtrage avant traduction**

Le filtrage de sortie est appliqué avant la traduction descendante.

**Règle FLUX-DESC-05 : Traduction adaptée**

La traduction adapte le message au vocabulaire et au format de chaque produit cible.

**Règle FLUX-DESC-06 : Sélection des produits**

Seuls les produits concernés et autorisés reçoivent la notification.

**Règle FLUX-DESC-07 : Journalisation systématique**

Chaque étape critique est journalisée :
- Réception de la notification
- Normalisation
- Filtrage
- Traduction
- Sélection des produits
- Distribution

**Règle FLUX-DESC-08 : Distribution fiable**

La notification est distribuée de manière fiable à tous les produits cibles (avec retry si nécessaire).

**Règle FLUX-DESC-09 : Isolation**

Chaque produit reçoit uniquement les informations qui lui sont destinées, sans fuite vers d'autres produits.

### 6.4 Types de notifications

**Notification de résultat :**
- Résultat d'une intention précédente
- Statut d'une opération
- Confirmation d'une action

**Notification d'événement :**
- Événement système
- Changement d'état
- Synchronisation

**Notification d'erreur :**
- Erreur survenue dans l'écosystème
- Échec d'une opération
- Avertissement

### 6.5 Gestion des erreurs dans le flux descendant

**Erreur de réception :**
- Retry selon politique
- Journalisation de l'erreur
- Pas de notification au produit (pas de demande)

**Erreur de filtrage :**
- Suppression de la notification (si non autorisée)
- Journalisation
- Pas de notification au produit

**Erreur de traduction :**
- Notification générique au produit
- Journalisation de l'erreur
- Préservation de l'information essentielle

**Erreur de distribution :**
- Retry selon politique
- Mise en file d'attente si nécessaire
- Journalisation

---

## 7. Coordination entre les flux

### 7.1 Flux indépendants

**Règle COORD-01 : Indépendance**

Les flux ascendant et descendant sont indépendants. Un flux peut se produire sans l'autre.

**Règle COORD-02 : Pas de blocage mutuel**

Un flux ne bloque jamais l'autre. Les deux flux peuvent être actifs simultanément.

**Règle COORD-03 : Pas de dépendance temporelle**

Le flux descendant n'est pas nécessairement une réponse au flux ascendant. Il peut être déclenché indépendamment.

### 7.2 Corrélation des flux

**Règle COORD-04 : Corrélation par intention_id**

Quand le flux descendant est une réponse au flux ascendant, la corrélation se fait via `intention_id`.

**Règle COORD-05 : Traçabilité croisée**

Les deux flux sont traçables indépendamment, mais peuvent être corrélés via les identifiants.

**Règle COORD-06 : Ordre préservé**

Pour une même intention, l'ordre des réponses est préservé (FIFO).

### 7.3 Synchronisation

**Règle COORD-07 : Pas de synchronisation bloquante**

Bonding Brother ne bloque jamais en attendant une réponse. Les notifications sont asynchrones.

**Règle COORD-08 : Gestion des timeouts**

Si une réponse n'arrive pas dans le délai attendu, le produit est notifié avec un statut approprié.

---

## 8. Asymétrie et adaptation

### 8.1 Principe d'asymétrie

**Règle ASYM-01 : Adaptation unidirectionnelle**

Les produits s'adaptent à Bonding Brother. Bonding Brother s'adapte aux autorités. Jamais l'inverse.

**Règle ASYM-02 : Interface stable**

L'interface de Bonding Brother vers les produits est stable. Les produits doivent s'adapter aux changements (selon versionnement).

**Règle ASYM-03 : Adaptation aux autorités**

Bonding Brother s'adapte aux changements des autorités, masquant cette adaptation aux produits.

### 8.2 Adaptation dans le flux ascendant

**Adaptation du produit :**
- Format d'intention standard
- Vocabulaire canonique
- Structure imposée

**Adaptation de Bonding Brother :**
- Traduction vers le vocabulaire de l'autorité
- Adaptation du format aux contraintes de l'autorité
- Enrichissement technique (métadonnées)

### 8.3 Adaptation dans le flux descendant

**Adaptation de Bonding Brother :**
- Traduction vers le vocabulaire du produit
- Adaptation du format aux attentes du produit
- Filtrage et projection

**Adaptation du produit :**
- Réception du format standard
- Consommation du vocabulaire canonique
- Gestion des notifications asynchrones

---

## 9. Garanties des flux bilatéraux

### 9.1 Garantie de complétude

**Engagement :** Toute intention exprimée par un produit reçoit une réponse (succès, refus, ou erreur). Toute notification de l'écosystème est distribuée aux produits concernés.

**Mesure :** Traçabilité complète avec vérification que chaque intention a un résultat.

### 9.2 Garantie de fidélité

**Engagement :** Les intentions sont transmises fidèlement aux autorités, et les réponses sont transmises fidèlement aux produits (après filtrage et traduction).

**Mesure :** Tests de round-trip avec vérification de préservation du sens.

### 9.3 Garantie d'isolation

**Engagement :** Les produits sont isolés les uns des autres. Aucune fuite d'information entre produits.

**Mesure :** Tests avec plusieurs produits vérifiant l'absence de fuite.

### 9.4 Garantie de performance

**Engagement :** Les flux sont traités dans des délais raisonnables, avec des métriques de performance définies.

**Mesure :** Métriques de temps de traitement par étape.

### 9.5 Garantie de disponibilité

**Engagement :** Les flux fonctionnent même en mode offline (avec autorité différée).

**Mesure :** Tests de fonctionnement offline avec synchronisation à la reconnexion.

**Conformité autonomie :** Cette garantie implémente directement **LOI-2** (isolement comme état normal) et **LOI-3** (état local souverain) : les flux continuent de fonctionner en isolation, et l'état local est préservé jusqu'à la synchronisation.

---

## 10. Exemples

### 10.1 Flux ascendant complet

**1. Produit exprime une intention :**
```json
{
  "id": "int-123",
  "produit_id": "miyukini-cms",
  "type": "CREATE_CONTENT",
  "payload": { ... },
  "contexte": { ... }
}
```

**2. Après validation, traduction, filtrage :**
```json
{
  "demande_id": "dem-789",
  "intention_id": "int-123",
  "type": "create_content",
  "données": { ... },
  "contexte": { ... }
}
```
→ Transmis à Kind Mother

**3. Réponse de Kind Mother :**
```json
{
  "status": "accepted",
  "data": { "content_id": "content-999" }
}
```

**4. Après filtrage, traduction :**
```json
{
  "résultat_id": "res-111",
  "intention_id": "int-123",
  "statut": "SUCCÈS",
  "données": { "id": "content-999" }
}
```
→ Transmis au produit

### 10.2 Flux descendant

**1. Kind Mother émet un événement :**
```json
{
  "event_type": "content_updated",
  "content_id": "content-999",
  "changes": { ... }
}
```

**2. Après normalisation, filtrage, traduction :**
```json
{
  "type": "CONTENT_UPDATED",
  "content_id": "content-999",
  "modifications": { ... }
}
```

**3. Distribution aux produits concernés :**
→ Produit A (abonné aux mises à jour)
→ Produit B (propriétaire du contenu)

---

## 11. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit les règles des flux bilatéraux que Bonding Brother doit respecter pour garantir la communication fiable et sécurisée entre les produits et l'écosystème.

Tout flux géré par Bonding Brother doit respecter ce contrat. Toute violation entraîne un rejet ou une erreur avec code approprié.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- Documentation Fondatrice v1.0 (Section 5)
- Intent Model Contract v1.0
- Translation Contract v1.0
- Architecture et Composants v1.0
- Glossaire et Terminologie v1.0
