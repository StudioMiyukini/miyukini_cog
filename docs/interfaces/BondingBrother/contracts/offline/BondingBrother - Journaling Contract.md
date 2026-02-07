# BondingBrother - Journaling Contract

## 1. Contexte

Ce document définit le contrat de journalisation systématique dans Bonding Brother. Il spécifie comment toutes les interactions entre les produits et l'écosystème via Bonding Brother sont enregistrées de manière complète, traçable et immuable pour garantir l'auditabilité, la responsabilité et la reprise après incident.

Ce document complète la Section 8 et la Section 9 de la [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur l'[Architecture & Flows](../../architecture/BondingBrother%20-%20Architecture%20&%20Flows.md), l'[Intent Model Contract](../intent/BondingBrother%20-%20Intent%20Model%20Contract.md) et l'[Offline & Deferred Authority Contract](./BondingBrother%20-%20Offline%20&%20Deferred%20Authority%20Contract.md).

Ce contrat garantit le respect de **LOI-3** (état local souverain) : les logs locaux constituent une trace d'audit complète et autonome, accessible même en mode offline, conformément aux [Lois d'Autonomie Système](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md).

## 2. Portée / Scope

Ce document couvre :
- Le principe de journalisation systématique
- Les événements à journaliser
- La structure des entrées de journal
- Les règles d'immuabilité et de traçabilité
- La gestion du stockage et de la rétention
- L'accès et la consultation des journaux

Ce document **ne couvre pas** :
- Les détails d'audit (voir [Audit & Traceability Contract](../governance/BondingBrother%20-%20Audit%20&%20Traceability%20Contract.md))
- Les détails de synchronisation (voir [Sync & Reconnection Contract](./BondingBrother%20-%20Sync%20&%20Reconnection%20Contract.md))
- Les détails de responsabilité (voir [Responsibility Model Contract](../governance/BondingBrother%20-%20Responsibility%20Model%20Contract.md))

---

## 3. Principe fondamental

**Bonding Brother journalise systématiquement toute interaction, sans exception et de manière immuable.**

Toute intention reçue, toute traduction effectuée, toute transmission à une autorité, toute réponse reçue, tout résultat transmis est enregistré dans le journal. Cette journalisation est systématique, complète, non optionnelle et non contournable.

Ce principe garantit **LOI-3** : les logs locaux sont souverains et constituent une trace d'audit complète, indépendamment de toute connexion externe. La journalisation fonctionne même en mode offline.

---

## 4. Événements journalisés

### 4.1 Intention reçue

**Déclencheur :** Bonding Brother reçoit une intention d'un produit.

**Contenu journalisé :**
- L'intention complète (structure + payload)
- Le contexte complet
- Le timestamp de réception
- L'identité du produit
- Le statut initial (CRÉÉE)

**Règle JOURN-01 : Journalisation immédiate**

L'intention est journalisée immédiatement après réception, avant tout traitement.

### 4.2 Intention validée

**Déclencheur :** Validation structurelle réussie d'une intention.

**Contenu journalisé :**
- L'ID de l'intention
- Le résultat de la validation (succès)
- Le timestamp de validation
- Les détails de validation (si pertinents)

**Règle JOURN-02 : Journalisation des validations**

Toute validation (succès ou échec) est journalisée avec le résultat.

### 4.3 Intention traduite

**Déclencheur :** Traduction réussie d'une intention en demande.

**Contenu journalisé :**
- L'ID de l'intention
- La demande traduite (structure complète)
- Le timestamp de traduction
- Le mapping de vocabulaire utilisé
- L'autorité cible identifiée

**Règle JOURN-03 : Journalisation de la traduction**

La traduction est journalisée avec la demande résultante pour traçabilité complète.

### 4.4 Intention filtrée

**Déclencheur :** Application des règles de filtrage à une demande.

**Contenu journalisé :**
- L'ID de l'intention
- Le résultat du filtrage (acceptée / rejetée)
- Les règles de filtrage appliquées
- Le timestamp de filtrage
- La raison du rejet (si rejetée)

**Règle JOURN-04 : Journalisation du filtrage**

Tout filtrage (acceptation ou rejet) est journalisé avec les règles appliquées.

### 4.5 Intention transmise

**Déclencheur :** Transmission réussie d'une demande à une autorité.

**Contenu journalisé :**
- L'ID de l'intention
- L'autorité cible (Kind Mother ou Strong Father)
- Le timestamp de transmission
- Le format de transmission
- Le statut (TRANSMISE)

**Règle JOURN-05 : Journalisation de la transmission**

Toute transmission (succès ou échec) est journalisée avec l'autorité cible.

### 4.6 Réponse reçue

**Déclencheur :** Réception d'une réponse d'une autorité.

**Contenu journalisé :**
- L'ID de l'intention associée
- La réponse complète de l'autorité
- Le timestamp de réception
- L'autorité émettrice
- La décision (acceptée / refusée / erreur)

**Règle JOURN-06 : Journalisation de la réponse**

Toute réponse d'autorité est journalisée intégralement, sans filtrage.

### 4.7 Résultat transmis

**Déclencheur :** Transmission d'un résultat filtré à un produit.

**Contenu journalisé :**
- L'ID de l'intention associée
- Le résultat filtré transmis
- Le timestamp de transmission
- L'identité du produit destinataire
- Le statut final (RÉSOLUE)

**Règle JOURN-07 : Journalisation du résultat**

Tout résultat transmis est journalisé avec le produit destinataire.

### 4.8 Erreur ou rejet

**Déclencheur :** Toute erreur ou rejet à n'importe quelle étape.

**Contenu journalisé :**
- L'ID de l'intention (si applicable)
- Le type d'erreur
- Le code d'erreur
- Le message d'erreur
- Le timestamp
- Le contexte de l'erreur
- La stack trace (si applicable)

**Règle JOURN-08 : Journalisation des erreurs**

Toute erreur est journalisée avec un niveau de détail suffisant pour diagnostic.

### 4.9 Événements offline

**Déclencheur :** Tous les événements liés au mode offline.

**Contenu journalisé :**
- Passage en mode offline (autorité, timestamp)
- Mise en buffer (intention ID, timestamp)
- Reconnexion (autorité, timestamp)
- Synchronisation (début, fin, nombre d'intentions)
- Erreurs de synchronisation

**Règle JOURN-09 : Journalisation offline**

Tous les événements offline sont journalisés avec un marqueur spécifique.

---

## 5. Structure d'une entrée de journal

### 5.1 Schéma de base

```typescript
interface JournalEntry {
    // Identifiants
    entry_id: JournalEntryId;        // ID unique de l'entrée
    intention_id?: IntentionId;       // ID de l'intention (si applicable)
    correlation_id?: CorrelationId;   // ID de corrélation
    
    // Type et contenu
    event_type: EventType;            // Type d'événement
    event_data: EventData;            // Données de l'événement
    
    // Contexte
    produit_id?: ProduitId;           // Produit concerné
    autorité?: AutoritéType;          // Autorité concernée (si applicable)
    timestamp: Timestamp;              // Moment de l'événement
    
    // Métadonnées
    version: VersionJournal;          // Version du schéma de journal
    environment: Environnement;        // Environnement (dev, staging, prod)
    metadata?: Map<string, any>;      // Métadonnées complémentaires
}
```

### 5.2 Types d'événements

| Type d'événement | Description | Données associées |
|------------------|-------------|-------------------|
| `INTENTION_RECEIVED` | Intention reçue | Intention complète |
| `INTENTION_VALIDATED` | Intention validée | Résultat validation |
| `INTENTION_TRANSLATED` | Intention traduite | Demande traduite |
| `INTENTION_FILTERED` | Intention filtrée | Résultat filtrage |
| `INTENTION_TRANSMITTED` | Intention transmise | Autorité cible |
| `RESPONSE_RECEIVED` | Réponse reçue | Réponse autorité |
| `RESULT_TRANSMITTED` | Résultat transmis | Résultat filtré |
| `ERROR_OCCURRED` | Erreur survenue | Détails erreur |
| `OFFLINE_MODE_ENTERED` | Passage en mode offline | Autorité, raison |
| `OFFLINE_MODE_EXITED` | Sortie du mode offline | Autorité |
| `INTENTION_BUFFERED` | Intention mise en buffer | Détails buffer |
| `SYNC_STARTED` | Synchronisation démarrée | Autorité, nombre |
| `SYNC_COMPLETED` | Synchronisation terminée | Autorité, résultats |

### 5.3 Règles de structure

**Règle STRUCT-01 : Champs obligatoires**

Toute entrée de journal doit contenir :
- `entry_id` (obligatoire)
- `event_type` (obligatoire)
- `event_data` (obligatoire)
- `timestamp` (obligatoire)
- `version` (obligatoire)
- `environment` (obligatoire)

**Règle STRUCT-02 : Champs conditionnels**

Certains champs sont conditionnels :
- `intention_id` : obligatoire si l'événement est lié à une intention
- `produit_id` : obligatoire si l'événement implique un produit
- `autorité` : obligatoire si l'événement implique une autorité

**Règle STRUCT-03 : Format standardisé**

Toutes les entrées suivent le même format pour faciliter la recherche et l'analyse.

---

## 6. Propriétés du journal

### 6.1 Immutabilité

**Règle IMMUT-01 : Aucune modification**

Une fois écrite, une entrée de journal ne peut jamais être modifiée :
- Pas de mise à jour
- Pas de correction
- Pas de suppression
- Pas d'édition

**Règle IMMUT-02 : Corrections par nouvelles entrées**

Les corrections se font par ajout de nouvelles entrées :
- Entrée de correction liée à l'entrée originale
- Traçabilité de la correction
- Historique complet préservé

**Règle IMMUT-03 : Stockage immuable**

Le stockage du journal garantit l'immuabilité :
- Stockage en append-only
- Pas de réécriture possible
- Vérification d'intégrité

### 6.2 Traçabilité

**Règle TRACE-01 : Chaîne complète**

Pour toute intention, on peut reconstruire la chaîne complète :
- Réception → Validation → Traduction → Filtrage → Transmission → Réponse → Résultat

**Règle TRACE-02 : Corrélation**

Toutes les entrées liées à une intention sont corrélées :
- Par `intention_id`
- Par `correlation_id` (si applicable)
- Par timestamp

**Règle TRACE-03 : Contexte préservé**

Tout le contexte nécessaire est préservé :
- Identité du produit
- Identité de l'utilisateur (si applicable)
- Permissions déclarées
- Environnement

### 6.3 Complétude

**Règle COMPL-01 : Aucun événement omis**

Tout événement significatif est journalisé :
- Aucun événement silencieux
- Aucun événement ignoré
- Aucun événement filtré avant journalisation

**Règle COMPL-02 : Données complètes**

Les données journalisées sont complètes :
- Pas de données tronquées
- Pas de données masquées
- Pas de données résumées

**Règle COMPL-03 : Vérification de complétude**

Des mécanismes vérifient la complétude :
- Vérification de séquence
- Détection de trous
- Alertes en cas d'incomplétude

---

## 7. Stockage et rétention

### 7.1 Stockage

**Règle STOCK-01 : Stockage persistant**

Le journal est stocké de manière persistante :
- Survit aux redémarrages
- Survit aux pannes
- Garantit la durabilité

**Règle STOCK-02 : Performance**

Le stockage ne doit pas impacter les performances :
- Écriture asynchrone (si possible)
- Pas de blocage du flux principal
- Optimisation pour écriture

**Règle STOCK-03 : Intégrité**

L'intégrité du journal est garantie :
- Vérification de checksum
- Détection de corruption
- Réplication (si applicable)

### 7.2 Rétention

**Règle RET-01 : Durée de rétention**

La durée de rétention est configurable :
- Par défaut : 1 an
- Configurable par environnement
- Règles de rétention par type d'événement

**Règle RET-02 : Archivage**

Les entrées anciennes peuvent être archivées :
- Archivage avant suppression
- Format d'archivage standardisé
- Accessibilité des archives

**Règle RET-03 : Suppression**

La suppression suit des règles strictes :
- Suppression uniquement après archivage
- Suppression uniquement après expiration
- Traçabilité de la suppression

---

## 8. Accès et consultation

### 8.1 Accès au journal

**Règle ACCES-01 : Accès contrôlé**

L'accès au journal est contrôlé :
- Authentification requise
- Autorisation basée sur les rôles
- Audit des accès

**Règle ACCES-02 : Accès par produit**

Un produit peut accéder à ses propres entrées :
- Filtrage automatique par `produit_id`
- Pas d'accès aux entrées d'autres produits
- API dédiée pour consultation

**Règle ACCES-03 : Accès par intention**

Recherche possible par `intention_id` :
- Récupération de toutes les entrées d'une intention
- Reconstruction de la chaîne complète
- Format structuré

### 8.2 Consultation

**Règle CONS-01 : Formats de consultation**

Plusieurs formats de consultation disponibles :
- JSON structuré
- Format lisible (texte)
- Export CSV (pour analyse)

**Règle CONS-02 : Filtrage**

Possibilité de filtrer les entrées :
- Par type d'événement
- Par produit
- Par période
- Par intention
- Par autorité

**Règle CONS-03 : Performance**

La consultation est optimisée :
- Indexation des champs fréquents
- Pagination pour grandes quantités
- Cache pour requêtes répétées

---

## 9. Journalisation en mode offline

### 9.1 Journalisation normale

**Règle OFFLINE-JOURN-01 : Journalisation continue**

En mode offline, la journalisation continue normalement :
- Tous les événements sont journalisés
- Aucune différence avec le mode en ligne
- Marqueur offline ajouté aux entrées

### 9.2 Marqueur offline

**Règle OFFLINE-JOURN-02 : Identification offline**

Toutes les entrées créées en mode offline sont marquées :
- Champ `offline: true` dans les métadonnées
- Autorité concernée identifiée
- Timestamp de mise en buffer

**Règle OFFLINE-JOURN-03 : Synchronisation journalisée**

La synchronisation est entièrement journalisée :
- Début de synchronisation
- Chaque intention synchronisée
- Fin de synchronisation
- Erreurs de synchronisation

---

## 10. Configuration

### 10.1 Paramètres configurables

| Paramètre | Description | Valeur par défaut | Unité |
|-----------|-------------|-------------------|-------|
| `journal.retention_duration` | Durée de rétention | 365 | jours |
| `journal.archive_before_delete` | Archivage avant suppression | true | booléen |
| `journal.async_write` | Écriture asynchrone | true | booléen |
| `journal.batch_size` | Taille des lots d'écriture | 100 | entrées |
| `journal.flush_interval` | Intervalle de flush | 5 | secondes |
| `journal.max_entry_size` | Taille maximale d'une entrée | 10 | MB |

### 10.2 Règles de configuration

**Règle CONFIG-JOURN-01 : Configuration immuable**

La configuration est immuable après le démarrage.

**Règle CONFIG-JOURN-02 : Validation**

Tous les paramètres sont validés au démarrage.

---

## 11. Exemples

### 11.1 Exemple : Entrée de journal pour intention reçue

```json
{
  "entry_id": "journal-001",
  "intention_id": "intention-123",
  "event_type": "INTENTION_RECEIVED",
  "event_data": {
    "intention": {
      "id": "intention-123",
      "produit_id": "miyukini-cms",
      "type": "CREATE_CONTENT",
      "payload": { /* ... */ },
      "contexte": { /* ... */ }
    }
  },
  "produit_id": "miyukini-cms",
  "timestamp": "2026-01-26T10:30:00Z",
  "version": "1.0.0",
  "environment": "production"
}
```

### 11.2 Exemple : Chaîne complète d'une intention

```
1. INTENTION_RECEIVED (intention-123)
2. INTENTION_VALIDATED (intention-123, succès)
3. INTENTION_TRANSLATED (intention-123, demande-KM)
4. INTENTION_FILTERED (intention-123, acceptée)
5. INTENTION_TRANSMITTED (intention-123, KindMother)
6. RESPONSE_RECEIVED (intention-123, réponse-KM)
7. RESULT_TRANSMITTED (intention-123, résultat-filtré)
```

---

## 12. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit les règles de journalisation que Bonding Brother doit respecter pour garantir l'auditabilité et la traçabilité.

Toute implémentation de la journalisation doit respecter ces règles. Toute violation compromet l'auditabilité et la responsabilité du système.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) v2.0 (Sections 8 et 9)
- [Architecture & Flows](../../architecture/BondingBrother%20-%20Architecture%20&%20Flows.md) v2.0
- [Intent Model Contract](../intent/BondingBrother%20-%20Intent%20Model%20Contract.md) v2.0
- [Offline & Deferred Authority Contract](./BondingBrother%20-%20Offline%20&%20Deferred%20Authority%20Contract.md) v2.0
