# Caring Nanny - StrongFather Integration Contract

## 1. Contexte

Ce document définit le **contrat d'intégration entre Caring Nanny et StrongFather**. Il spécifie l'interface, le protocole, les règles de communication, et les garanties associées à l'intégration avec StrongFather en tant qu'autorité des décisions stratégiques et politiques.

Ce document complète la Section 3 de la [Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [Caring Nanny - Architecture et Composants](../../architecture/Caring%20Nanny%20-%20Architecture%20et%20Composants.md) pour l'architecture de Caring Nanny
- [StrongFather - Documentation Fondatrice](../../../StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md) pour la nature de StrongFather
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) pour la conformité LOI-1 à LOI-6

L'intégration respecte les Lois d'Autonomie Système : toutes les observations sont locales et ne requièrent aucune dépendance externe (**LOI-1**).

## 2. Portée / Scope

Ce document couvre :
- L'interface contractuelle entre Caring Nanny et StrongFather
- Le protocole de communication (consultation et information)
- Les types d'interrogations et d'informations échangées
- Les règles d'intégration spécifiques
- La gestion des erreurs et des réponses
- Les garanties de l'intégration

Ce document **ne couvre pas** :
- Les détails internes de StrongFather (voir documentation StrongFather)
- Les détails internes du moteur d'observation (voir Architecture et Composants)
- L'intégration avec KindMother (voir KindMother Integration Contract)
- L'intégration avec BondingBrother (voir BondingBrother Integration Contract)

---

## 3. Principe fondamental

**Caring Nanny informe StrongFather de l'état du système pour enrichir le contexte des décisions. StrongFather peut consulter Caring Nanny pour connaître l'état actuel. Caring Nanny ne participe jamais à la décision elle-même.**

La relation est d'**information** : Caring Nanny fournit l'état, StrongFather consulte cet état pour contextualiser ses décisions. Cette relation est unidirectionnelle en termes de flux décisionnel : Caring Nanny informe, StrongFather décide.

---

## 4. Nature de la relation Caring Nanny — StrongFather

### 4.1 Relation d'information

**Caring Nanny informe StrongFather de :**
- L'état actuel du système (healthy, degraded, offline, syncing, error)
- Les transitions d'état en cours
- Les conditions qui pourraient affecter les décisions
- Les anomalies détectées

**StrongFather consulte Caring Nanny pour :**
- Connaître l'état du système avant une évaluation d'intention
- Contextualiser une décision avec l'état actuel
- Intégrer les conditions d'environnement dans l'évaluation

**Règle CN-SF-01 : Information sans décision**

Caring Nanny ne participe jamais aux décisions de StrongFather. Elle fournit des informations factuelles sur l'état du système, sans recommandation, sans interprétation décisionnelle, sans jugement.

**Règle CN-SF-02 : Consultation facultative**

StrongFather peut consulter Caring Nanny, mais n'est pas obligé de le faire. La décision d'intégrer l'état système dans une évaluation appartient à StrongFather.

**Règle CN-SF-03 : Aucune influence sur le résultat**

L'état rapporté par Caring Nanny n'influence jamais directement le résultat d'une évaluation. StrongFather utilise cet état comme contexte, mais la décision reste entièrement sous son autorité.

### 4.2 Séparation des responsabilités

| Responsabilité | Caring Nanny | StrongFather |
|----------------|--------------|--------------|
| **Connaître l'état système** | ✅ Exclusif | ❌ Consulte |
| **Détecter les anomalies** | ✅ Exclusif | ❌ Informé |
| **Décider si autorisé** | ❌ Jamais | ✅ Exclusif |
| **Appliquer des politiques** | ❌ Jamais | ✅ Exclusif |
| **Évaluer des intentions** | ❌ Jamais | ✅ Exclusif |
| **Modifier l'état** | ❌ Jamais | ❌ Jamais |
| **Fournir le contexte état** | ✅ Exclusif | ❌ Consomme |

**Règle CN-SF-04 : Aucun chevauchement**

Aucun chevauchement de responsabilités n'est autorisé. Caring Nanny ne prend jamais de décision, StrongFather ne maintient jamais d'état système.

---

## 5. Ce que Caring Nanny ne fait JAMAIS vis-à-vis de StrongFather

### 5.1 Interdictions absolues

**INV-CN-SF-NEVER-1 : Ne prend jamais de décision**

Caring Nanny ne prend **jamais** de décision basée sur l'état observé. Si l'état est `degraded` ou `error`, Caring Nanny informe, mais ne décide pas de bloquer ou d'autoriser quoi que ce soit.

**INV-CN-SF-NEVER-2 : Ne modifie jamais une politique**

Caring Nanny ne modifie **jamais** une politique ou une contrainte de StrongFather. Les politiques appartiennent exclusivement à StrongFather.

**INV-CN-SF-NEVER-3 : Ne refuse jamais une intention**

Caring Nanny ne refuse **jamais** et n'accepte **jamais** une intention. L'acceptation ou le refus est la prérogative exclusive de StrongFather.

**INV-CN-SF-NEVER-4 : N'influence jamais le résultat**

Caring Nanny n'influence **jamais** le résultat d'une évaluation de StrongFather. Elle fournit un contexte, mais le résultat est déterminé uniquement par StrongFather selon ses politiques.

**INV-CN-SF-NEVER-5 : Ne recommande jamais**

Caring Nanny ne fournit **jamais** de recommandation à StrongFather. Elle rapporte des faits (états, conditions, anomalies), pas des conseils ou des suggestions.

---

## 6. Types d'informations échangées

### 6.1 Information d'état système

**SYSTEM_STATE**
- **Objectif :** Fournir l'état global du système
- **Contenu :** Catégorie d'état (healthy, degraded, offline, syncing, error)
- **Fréquence :** Sur demande ou lors de transitions

**Structure de l'état système :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `state_id` | Identifiant unique de l'état | ✅ Oui |
| `category` | Catégorie (healthy, degraded, offline, syncing, error) | ✅ Oui |
| `timestamp` | Horodatage de l'observation | ✅ Oui |
| `components` | États des composants individuels | ✅ Oui |
| `conditions` | Conditions observées | ✅ Oui |
| `last_transition` | Dernière transition enregistrée | ❌ Optionnel |

### 6.2 Information de transition

**STATE_TRANSITION**
- **Objectif :** Informer d'une transition d'état
- **Contenu :** État précédent, état actuel, cause de la transition
- **Déclencheur :** Changement d'état détecté

**Structure de la transition :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `transition_id` | Identifiant unique de la transition | ✅ Oui |
| `previous_state` | État avant la transition | ✅ Oui |
| `current_state` | État après la transition | ✅ Oui |
| `cause` | Condition ayant provoqué la transition | ✅ Oui |
| `timestamp` | Horodatage de la transition | ✅ Oui |
| `affected_components` | Composants concernés | ❌ Optionnel |

### 6.3 Information d'anomalie

**ANOMALY_DETECTED**
- **Objectif :** Informer d'une anomalie détectée
- **Contenu :** Nature de l'anomalie, sévérité, composant concerné
- **Déclencheur :** Détection d'une condition anormale

**Structure de l'anomalie :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `anomaly_id` | Identifiant unique de l'anomalie | ✅ Oui |
| `type` | Type d'anomalie | ✅ Oui |
| `severity` | Sévérité (info, warning, critical) | ✅ Oui |
| `component` | Composant concerné | ✅ Oui |
| `description` | Description factuelle | ✅ Oui |
| `timestamp` | Horodatage de la détection | ✅ Oui |
| `conditions` | Conditions ayant déclenché l'anomalie | ❌ Optionnel |

### 6.4 Information de condition

**CONDITION_REPORT**
- **Objectif :** Rapporter une condition pouvant affecter les décisions
- **Contenu :** Condition observée, contexte, impact potentiel
- **Usage :** Enrichissement du contexte décisionnel

**Structure de la condition :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `condition_id` | Identifiant unique de la condition | ✅ Oui |
| `type` | Type de condition | ✅ Oui |
| `value` | Valeur observée | ✅ Oui |
| `threshold` | Seuil de référence (si applicable) | ❌ Optionnel |
| `timestamp` | Horodatage de l'observation | ✅ Oui |
| `context` | Contexte d'observation | ❌ Optionnel |

---

## 7. Types de consultations

### 7.1 Consultation d'état actuel

**GET_CURRENT_STATE**
- **Initiateur :** StrongFather
- **Objectif :** Obtenir l'état actuel du système
- **Payload :** Aucun ou filtre optionnel (composant spécifique)
- **Réponse :** État système complet ou filtré

**Règle CN-SF-QUERY-01 : Réponse instantanée**

La réponse à une consultation d'état est instantanée. Caring Nanny retourne l'état connu au moment de la demande, sans délai.

### 7.2 Consultation d'état de composant

**GET_COMPONENT_STATE**
- **Initiateur :** StrongFather
- **Objectif :** Obtenir l'état d'un composant spécifique
- **Payload :** Identifiant du composant
- **Réponse :** État du composant avec métadonnées

**Règle CN-SF-QUERY-02 : Composant inconnu**

Si le composant demandé n'est pas observé par Caring Nanny, la réponse est `UNKNOWN` avec une indication que le composant n'est pas dans le périmètre d'observation.

### 7.3 Consultation d'historique

**GET_STATE_HISTORY**
- **Initiateur :** StrongFather
- **Objectif :** Obtenir l'historique des états récents
- **Payload :** Fenêtre temporelle, composant optionnel
- **Réponse :** Liste des états et transitions dans la fenêtre

**Règle CN-SF-QUERY-03 : Historique limité**

L'historique retourné est limité à la fenêtre de rétention configurée. Caring Nanny ne garantit pas la disponibilité d'un historique illimité.

### 7.4 Consultation de conditions actives

**GET_ACTIVE_CONDITIONS**
- **Initiateur :** StrongFather
- **Objectif :** Obtenir les conditions actuellement actives
- **Payload :** Filtre optionnel (type, sévérité)
- **Réponse :** Liste des conditions actives

**Règle CN-SF-QUERY-04 : Conditions factuelles**

Les conditions retournées sont des faits observés, sans interprétation. StrongFather interprète ces conditions selon ses politiques.

---

## 8. Protocole de communication

### 8.1 Format des consultations

Les consultations de StrongFather suivent un format standardisé.

**Structure de base :**

| Élément | Description | Obligatoire |
|---------|-------------|-------------|
| `query_id` | Identifiant unique de la consultation | ✅ Oui |
| `intention_id` | Référence à l'intention en cours d'évaluation | ❌ Optionnel |
| `type` | Type de consultation | ✅ Oui |
| `payload` | Données spécifiques à la consultation | ❌ Selon type |
| `contexte_appelant` | Contexte de StrongFather | ✅ Oui |
| `timestamp` | Horodatage de la consultation | ✅ Oui |

**Règle CN-SF-PROT-01 : Format standardisé**

Toutes les consultations respectent le format standardisé. Aucune consultation ad-hoc n'est acceptée.

### 8.2 Format des réponses

Les réponses de Caring Nanny suivent un format standardisé.

**Structure de base :**

| Élément | Description | Obligatoire |
|---------|-------------|-------------|
| `response_id` | Identifiant unique de la réponse | ✅ Oui |
| `query_id` | Référence à la consultation | ✅ Oui |
| `status` | Statut de la réponse (SUCCESS, NOT_FOUND, UNKNOWN, ERROR) | ✅ Oui |
| `data` | Données de la réponse | Si SUCCESS |
| `error` | Détails de l'erreur | Si ERROR |
| `timestamp` | Horodatage de la réponse | ✅ Oui |

**Règle CN-SF-PROT-02 : Réponse toujours structurée**

Caring Nanny retourne toujours une réponse structurée, même en cas d'erreur ou de non-connaissance.

**Règle CN-SF-PROT-03 : Pas d'interprétation décisionnelle**

Les réponses sont des informations brutes. Caring Nanny n'interprète pas les données pour StrongFather et ne suggère jamais de décision.

### 8.3 Statuts de réponse

| Statut | Signification |
|--------|---------------|
| `SUCCESS` | La consultation a abouti, les données sont fournies |
| `NOT_FOUND` | L'élément recherché n'est pas dans l'historique |
| `UNKNOWN` | Le composant n'est pas dans le périmètre d'observation |
| `ERROR` | Une erreur interne s'est produite |

**Règle CN-SF-PROT-04 : UNKNOWN n'est pas une erreur**

Le statut `UNKNOWN` est une réponse valide, pas une erreur. Il indique que Caring Nanny n'observe pas le composant demandé.

---

## 9. Format des notifications

### 9.1 Notifications proactives

Caring Nanny peut notifier StrongFather de manière proactive lors de certains événements.

**Événements déclencheurs :**
- Transition d'état système (healthy → degraded, etc.)
- Détection d'anomalie critique
- Conditions pouvant affecter les décisions en cours

**Règle CN-SF-NOTIF-01 : Notification informative**

Les notifications sont purement informatives. Elles n'exigent aucune action de StrongFather et n'attendent aucune réponse.

**Règle CN-SF-NOTIF-02 : Pas de notification bloquante**

Les notifications ne bloquent jamais les opérations de StrongFather. Elles sont envoyées de manière asynchrone et non bloquante.

### 9.2 Structure des notifications

| Élément | Description | Obligatoire |
|---------|-------------|-------------|
| `notification_id` | Identifiant unique de la notification | ✅ Oui |
| `type` | Type de notification (STATE_CHANGE, ANOMALY, CONDITION) | ✅ Oui |
| `severity` | Sévérité (info, warning, critical) | ✅ Oui |
| `data` | Données de la notification | ✅ Oui |
| `timestamp` | Horodatage de la notification | ✅ Oui |

---

## 10. Flux d'intégration typique

### 10.1 Flux de consultation avant évaluation

**Acteurs :** BondingBrother, StrongFather, Caring Nanny

**Séquence :**

1. BondingBrother soumet une intention à StrongFather pour évaluation
2. StrongFather décide de consulter l'état du système (optionnel)
3. StrongFather interroge Caring Nanny : `GET_CURRENT_STATE`
4. Caring Nanny répond avec l'état actuel du système
5. StrongFather intègre l'état dans le contexte d'évaluation
6. StrongFather évalue l'intention selon les politiques
7. StrongFather produit une décision (acceptée, refusée, ambiguë)

**Règle CN-SF-FLOW-01 : Consultation optionnelle**

La consultation de Caring Nanny par StrongFather est toujours optionnelle. StrongFather peut évaluer une intention sans consulter l'état système.

### 10.2 Flux de notification de transition

**Acteurs :** Caring Nanny, StrongFather

**Séquence :**

1. Caring Nanny détecte une transition d'état (ex: healthy → degraded)
2. Caring Nanny enregistre la transition dans l'historique
3. Caring Nanny notifie StrongFather : `STATE_TRANSITION`
4. StrongFather reçoit la notification (informatif)
5. StrongFather peut intégrer cette information dans les évaluations futures

**Règle CN-SF-FLOW-02 : Notification sans accusé**

StrongFather n'accuse pas réception des notifications. Caring Nanny envoie et continue ses observations sans attendre de confirmation.

### 10.3 Diagramme de séquence

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  BondingBrother │    │   StrongFather  │    │  Caring Nanny   │
└────────┬────────┘    └────────┬────────┘    └────────┬────────┘
         │                      │                      │
         ├── Intention ────────►│                      │
         │                      │                      │
         │                      ├── GET_CURRENT_STATE ►│
         │                      │                      │
         │                      │◄── État système ─────┤
         │                      │                      │
         │                      ├── Évaluation ────────┤
         │                      │   (avec contexte)    │
         │                      │                      │
         │◄── Décision ─────────┤                      │
         │                      │                      │
         │                      │                      │
         │                      │  (plus tard...)      │
         │                      │                      │
         │                      │◄── STATE_TRANSITION ─┤
         │                      │    (notification)    │
         │                      │                      │
```

---

## 11. Règles d'intégration

### 11.1 Règles de communication

**Règle CN-SF-INT-01 : Initiative mixte**

StrongFather initie les consultations. Caring Nanny initie les notifications. Les deux types de communication coexistent sans interférence.

**Règle CN-SF-INT-02 : Pas de dépendance obligatoire**

StrongFather peut fonctionner sans consulter Caring Nanny. L'intégration enrichit le contexte mais n'est pas obligatoire.

**Règle CN-SF-INT-03 : Réponses synchrones, notifications asynchrones**

Les réponses aux consultations sont synchrones. Les notifications sont asynchrones et non bloquantes.

### 11.2 Règles de données

**Règle CN-SF-INT-04 : Données fraîches**

Les données retournées par Caring Nanny reflètent l'état observé au moment de la consultation.

**Règle CN-SF-INT-05 : Pas de cache côté StrongFather recommandé**

StrongFather ne devrait pas mettre en cache les états de Caring Nanny de manière prolongée. L'état peut changer à tout moment.

**Règle CN-SF-INT-06 : Cohérence interne garantie**

Caring Nanny garantit la cohérence interne des données retournées. Un état et ses conditions sont mutuellement cohérents.

### 11.3 Règles de traçabilité

**Règle CN-SF-INT-07 : Traçabilité des consultations**

Toutes les consultations de StrongFather sont tracées par Caring Nanny avec le contexte complet.

**Règle CN-SF-INT-08 : Corrélation intention-consultation**

Chaque consultation peut être corrélée à une intention en cours d'évaluation (si `intention_id` fourni) pour l'audit bout-en-bout.

---

## 12. Gestion des erreurs

### 12.1 Types d'erreurs

**Erreurs de format :**
- Consultation mal formée
- Champ obligatoire manquant
- Type de consultation inconnu

**Erreurs de données :**
- Composant non observé (UNKNOWN, pas une erreur)
- Historique non disponible pour la période demandée (NOT_FOUND)

**Erreurs internes :**
- Erreur du moteur d'observation
- Erreur de calcul d'agrégation

### 12.2 Traitement des erreurs

**Règle CN-SF-ERR-01 : Réponse structurée toujours**

Caring Nanny retourne toujours une réponse structurée, même en cas d'erreur. StrongFather peut toujours interpréter la réponse.

**Règle CN-SF-ERR-02 : UNKNOWN est informatif**

Le statut `UNKNOWN` est une information, pas une erreur. StrongFather peut utiliser cette information (composant non observé = contexte partiel).

**Règle CN-SF-ERR-03 : Journalisation des erreurs**

Toutes les erreurs sont journalisées par Caring Nanny pour audit et diagnostic.

**Règle CN-SF-ERR-04 : Pas de retry automatique**

En cas d'erreur, StrongFather décide de la stratégie (retry, continuer sans contexte). Caring Nanny ne retry jamais automatiquement.

---

## 13. Cas particuliers

### 13.1 État système `offline`

Lorsque l'état système est `offline`, Caring Nanny continue de répondre aux consultations avec les observations locales disponibles.

**Règle CN-SF-CASE-01 : Offline n'est pas une erreur**

L'état `offline` est un état valide, pas une erreur. Caring Nanny rapporte cet état comme un fait, StrongFather l'interprète selon ses politiques.

### 13.2 État système `syncing`

Lorsque l'état système est `syncing`, Caring Nanny informe que certaines données peuvent être en cours de synchronisation.

**Règle CN-SF-CASE-02 : Syncing avec données disponibles**

Même en état `syncing`, Caring Nanny fournit les données disponibles localement. StrongFather peut décider d'attendre ou de procéder avec le contexte partiel.

### 13.3 État système `error`

Lorsque l'état système est `error`, Caring Nanny informe de l'erreur mais continue de fonctionner pour les composants non affectés.

**Règle CN-SF-CASE-03 : Error localisé**

Un état `error` peut être localisé à certains composants. Caring Nanny fournit le détail des composants affectés et non affectés.

---

## 14. Garanties de l'intégration

### 14.1 Garantie d'exhaustivité

**Engagement :** Les réponses de Caring Nanny sont exhaustives pour le périmètre observé. Toutes les informations connues sur un état ou composant sont fournies.

### 14.2 Garantie d'exactitude

**Engagement :** Les informations fournies par Caring Nanny sont exactes et reflètent l'observation au moment de la consultation.

### 14.3 Garantie de neutralité

**Engagement :** Caring Nanny fournit des informations sans interprétation décisionnelle, sans recommandation, sans jugement. La décision appartient exclusivement à StrongFather.

### 14.4 Garantie de traçabilité

**Engagement :** Toute interaction entre StrongFather et Caring Nanny est traçable de bout en bout. L'audit complet des consultations et réponses est possible.

### 14.5 Garantie de disponibilité

**Engagement :** Caring Nanny est disponible pour répondre aux consultations de StrongFather sans dépendance externe (conformité LOI-1).

### 14.6 Garantie de non-blocage

**Engagement :** Caring Nanny ne bloque jamais les opérations de StrongFather. Les consultations sont répondues immédiatement, les notifications sont asynchrones.

---

## 15. Invariants de l'intégration

### 15.1 Invariants de relation

**INV-CN-SF-1 : Information unidirectionnelle**

Caring Nanny informe StrongFather. Caring Nanny ne décide jamais pour StrongFather.

**INV-CN-SF-2 : Consultation facultative**

StrongFather consulte Caring Nanny de manière facultative. Aucune consultation n'est obligatoire.

**INV-CN-SF-3 : Aucune autorité partagée**

Caring Nanny n'a aucune autorité sur les décisions. StrongFather n'a aucune autorité sur les observations.

### 15.2 Invariants de données

**INV-CN-SF-4 : Lecture pure**

Les consultations sont des lectures pures. Aucune modification de l'état n'est causée par une consultation.

**INV-CN-SF-5 : Données factuelles**

Les données retournées sont factuelles (état, condition, anomalie). Aucune donnée interprétée décisionnellement n'est retournée.

### 15.3 Invariants de protocole

**INV-CN-SF-6 : Format respecté**

Toutes les consultations et réponses respectent le format standardisé.

**INV-CN-SF-7 : Traçabilité complète**

Toute interaction est traçable avec son contexte complet.

---

## 16. Conformité aux Lois d'Autonomie Système

### LOI-1 : Aucune dépendance externe critique

**Conformité :** ✅ **Conforme**

L'intégration respecte LOI-1 :
- Caring Nanny observe localement, sans dépendance externe
- StrongFather consulte localement, sans dépendance externe
- L'absence de connexion ne bloque ni l'observation ni la consultation

### LOI-2 : Le système accepte l'isolement comme état normal

**Conformité :** ✅ **Conforme**

L'intégration respecte LOI-2 :
- L'état `offline` est un état normal rapporté par Caring Nanny
- StrongFather peut prendre des décisions même en état `offline`
- Aucune dégradation de l'intégration en mode isolé

### LOI-4 : Pas de temps global requis

**Conformité :** ✅ **Conforme**

L'intégration respecte LOI-4 :
- Les horodatages sont locaux
- Aucune synchronisation temporelle n'est requise
- Les comparaisons temporelles inter-nœuds ne sont pas utilisées

---

## 17. Exemples

### 17.1 Consultation d'état actuel

**Consultation StrongFather :**
```
{
  "query_id": "q-sf-001",
  "intention_id": "intention-500",
  "type": "GET_CURRENT_STATE",
  "payload": null,
  "contexte_appelant": {
    "source": "strongfather",
    "evaluation_id": "eval-200"
  },
  "timestamp": "2026-01-27T14:00:00Z"
}
```

**Réponse Caring Nanny :**
```
{
  "response_id": "r-cn-001",
  "query_id": "q-sf-001",
  "status": "SUCCESS",
  "data": {
    "state_id": "state-current",
    "category": "healthy",
    "timestamp": "2026-01-27T14:00:00Z",
    "components": {
      "kindmother": "healthy",
      "storage": "healthy",
      "network": "healthy"
    },
    "conditions": []
  },
  "timestamp": "2026-01-27T14:00:01Z"
}
```

### 17.2 État dégradé

**Consultation StrongFather :**
```
{
  "query_id": "q-sf-002",
  "type": "GET_CURRENT_STATE",
  "payload": null,
  "contexte_appelant": {
    "source": "strongfather"
  },
  "timestamp": "2026-01-27T15:00:00Z"
}
```

**Réponse Caring Nanny :**
```
{
  "response_id": "r-cn-002",
  "query_id": "q-sf-002",
  "status": "SUCCESS",
  "data": {
    "state_id": "state-current",
    "category": "degraded",
    "timestamp": "2026-01-27T15:00:00Z",
    "components": {
      "kindmother": "healthy",
      "storage": "degraded",
      "network": "healthy"
    },
    "conditions": [
      {
        "condition_id": "cond-001",
        "type": "storage_latency",
        "value": "high",
        "timestamp": "2026-01-27T14:55:00Z"
      }
    ],
    "last_transition": {
      "transition_id": "trans-001",
      "previous_state": "healthy",
      "current_state": "degraded",
      "cause": "storage_latency_threshold_exceeded",
      "timestamp": "2026-01-27T14:55:00Z"
    }
  },
  "timestamp": "2026-01-27T15:00:01Z"
}
```

**Note :** StrongFather utilise cette information pour contextualiser ses décisions, mais la décision reste entièrement sous son autorité.

### 17.3 Notification de transition

**Notification Caring Nanny → StrongFather :**
```
{
  "notification_id": "notif-cn-001",
  "type": "STATE_TRANSITION",
  "severity": "warning",
  "data": {
    "transition_id": "trans-002",
    "previous_state": "degraded",
    "current_state": "offline",
    "cause": "network_connection_lost",
    "timestamp": "2026-01-27T16:00:00Z",
    "affected_components": ["network", "sync"]
  },
  "timestamp": "2026-01-27T16:00:01Z"
}
```

**Note :** Cette notification est purement informative. StrongFather n'accuse pas réception.

---

## 18. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit l'interface et le protocole que Caring Nanny doit respecter pour s'intégrer avec StrongFather.

Toute implémentation de l'intégration avec StrongFather doit respecter ce contrat. Toute violation entraîne un comportement non conforme.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** CONTRAT — Normatif  
**Dépendances :**
- Caring Nanny - Documentation Fondatrice v1.6 (Section 3)
- Caring Nanny - Architecture et Composants v1.0
- StrongFather - Documentation Fondatrice v1.5
- Miyukini Conceptual References - Lois Autonomie Systeme

---

## 19. Mini log de génération

### Décision éditoriale E1 : Direction de la relation

**Décision prise :** La relation est d'information : Caring Nanny informe, StrongFather consulte. Cette direction respecte la Documentation Fondatrice de Caring Nanny qui définit la relation comme "relation d'information, pas de délégation".

**Application :** Tout le document est structuré autour de cette relation unidirectionnelle en termes de flux décisionnel.

### Décision éditoriale E2 : Consultation facultative

**Décision prise :** La consultation de Caring Nanny par StrongFather est explicitement facultative. StrongFather peut évaluer des intentions sans consulter l'état système.

**Application :** Règle CN-SF-02 et INV-CN-SF-2 établissent cette facultativité.

### Warning W1 : Risque d'influence décisionnelle

**Warning rencontré :** Risque que l'état rapporté par Caring Nanny soit interprété comme une recommandation de décision.

**Décision prise :** Les interdictions absolues (Section 5) et les invariants clarifient que Caring Nanny ne participe jamais à la décision. L'état est un contexte, pas une recommandation.

**Correction effectuée :** Section 5 explicite les interdictions, Section 14.3 garantit la neutralité.

### Warning W2 : État offline vs erreur

**Warning rencontré :** Risque de confusion entre l'état `offline` (normal) et l'état `error` (anomalie).

**Décision prise :** La Section 13 traite explicitement les cas particuliers d'états et confirme que `offline` est un état normal conforme à LOI-2.

**Correction effectuée :** Règles CN-SF-CASE-01 à CN-SF-CASE-03 clarifient chaque cas.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec Caring Nanny - Documentation Fondatrice : Confirmée (relation d'information, pas de décision)
- ✅ Cohérence avec StrongFather - Documentation Fondatrice : Confirmée (StrongFather décide, consulte le contexte)
- ✅ Conformité LOI-1 : Confirmée (aucune dépendance externe)
- ✅ Conformité LOI-2 : Confirmée (offline est un état normal)
- ✅ Conformité LOI-4 : Confirmée (pas de temps global requis)
- ✅ Aucune autorité de Caring Nanny sur les décisions : Confirmée (INV-CN-SF-1, Section 5)
- ✅ Traçabilité complète : Confirmée (INV-CN-SF-7)

**Conclusion :** Aucune contradiction détectée. Le document est cohérent et non ambigu.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
