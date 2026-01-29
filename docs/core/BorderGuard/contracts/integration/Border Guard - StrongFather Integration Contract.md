# Border Guard - StrongFather Integration Contract

## 1. Contexte

Ce document définit le **contrat d'intégration entre Border Guard et StrongFather**. Il spécifie l'interface, le protocole, les règles de communication, et les garanties associées à l'intégration avec StrongFather en tant qu'autorité des décisions stratégiques et politiques.

Ce document complète la Section 8 de la [Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [StrongFather - Documentation Fondatrice](../../../StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md) pour la nature de StrongFather
- [Miyukini Conceptual References - Security Protocols](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md) pour les protocoles de sécurité temps réel et asynchrone
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) pour la conformité LOI-1 à LOI-6

L'intégration respecte les Lois d'Autonomie Système : toutes les définitions de frontières sont locales et ne requièrent aucune dépendance externe (**LOI-1**).

## 2. Portée / Scope

Ce document couvre :
- L'interface contractuelle entre Border Guard et StrongFather
- Le protocole de communication (consultation de contexte de frontière)
- Les types d'informations échangées
- Les règles d'intégration spécifiques
- La gestion des erreurs et des réponses
- Les garanties de l'intégration

Ce document **ne couvre pas** :
- Les détails internes de StrongFather (voir documentation StrongFather)
- Les détails internes du moteur de définition de frontières (voir Architecture)
- L'intégration avec BondingBrother (voir BondingBrother Integration Contract)
- L'intégration avec Caring Nanny (voir CaringNanny Integration Contract)

---

## 3. Principe fondamental

**Border Guard fournit à StrongFather le contexte de confiance et de frontière pour enrichir l'évaluation des intentions. StrongFather consulte Border Guard pour connaître le niveau de confiance d'une source et les règles de franchissement applicables. Border Guard ne participe jamais à la décision elle-même.**

La relation est de **conseil** : Border Guard informe StrongFather sur le contexte de frontière ; StrongFather décide en tenant compte de cette information. Cette relation est unidirectionnelle en termes de flux décisionnel : Border Guard informe, StrongFather décide.

---

## 4. Nature de la relation Border Guard — StrongFather

### 4.1 Relation de conseil

**Border Guard informe StrongFather de :**
- Le niveau de confiance de la source d'une intention (trusted, verified, unknown, hostile)
- La nature de la frontière traversée par l'intention (externe, interne, intégration)
- Les règles de franchissement applicables à cette frontière
- L'état de l'intégration concernée (si applicable)

**StrongFather consulte Border Guard pour :**
- Contextualiser une intention avec son niveau de confiance
- Connaître les règles de franchissement avant évaluation
- Intégrer la classification de source dans la décision

**Règle BG-SF-01 : Conseil sans décision**

Border Guard ne participe jamais aux décisions de StrongFather. Il fournit des informations de classification et de règles, sans recommandation, sans interprétation décisionnelle, sans jugement sur la validité de l'intention.

**Règle BG-SF-02 : Consultation facultative**

StrongFather peut consulter Border Guard, mais n'est pas obligé de le faire. La décision d'intégrer le contexte de frontière dans une évaluation appartient à StrongFather.

**Règle BG-SF-03 : Aucune influence sur le résultat**

Le contexte de frontière fourni par Border Guard n'influence jamais directement le résultat d'une évaluation. StrongFather utilise ce contexte comme information, mais la décision reste entièrement sous son autorité selon ses politiques.

### 4.2 Séparation des responsabilités

| Responsabilité | Border Guard | StrongFather |
|----------------|--------------|--------------|
| **Définir les frontières** | ✅ Exclusif | ❌ Jamais |
| **Classifier les niveaux de confiance** | ✅ Exclusif | ❌ Consomme |
| **Établir les règles de franchissement** | ✅ Exclusif | ❌ Consomme |
| **Décider si autorisé** | ❌ Jamais | ✅ Exclusif |
| **Appliquer des politiques** | ❌ Jamais | ✅ Exclusif |
| **Évaluer des intentions** | ❌ Jamais | ✅ Exclusif |
| **Modifier l'état** | ❌ Jamais | ❌ Jamais |
| **Fournir le contexte frontière** | ✅ Exclusif | ❌ Consomme |

**Règle BG-SF-04 : Aucun chevauchement**

Aucun chevauchement de responsabilités n'est autorisé. Border Guard ne prend jamais de décision, StrongFather ne définit jamais de frontière ou de niveau de confiance.

---

## 5. Ce que Border Guard ne fait JAMAIS vis-à-vis de StrongFather

### 5.1 Interdictions absolues

**INV-BG-SF-NEVER-1 : Ne prend jamais de décision**

Border Guard ne prend **jamais** de décision basée sur les classifications effectuées. Si une source est classifiée `hostile`, Border Guard informe, mais ne décide pas de bloquer ou d'autoriser quoi que ce soit.

**INV-BG-SF-NEVER-2 : Ne modifie jamais une politique**

Border Guard ne modifie **jamais** une politique ou une contrainte de StrongFather. Les politiques appartiennent exclusivement à StrongFather.

**INV-BG-SF-NEVER-3 : Ne refuse jamais une intention**

Border Guard ne refuse **jamais** et n'accepte **jamais** une intention. L'acceptation ou le refus est la prérogative exclusive de StrongFather.

**INV-BG-SF-NEVER-4 : N'influence jamais le résultat**

Border Guard n'influence **jamais** le résultat d'une évaluation de StrongFather. Il fournit un contexte de classification, mais le résultat est déterminé uniquement par StrongFather selon ses politiques.

**INV-BG-SF-NEVER-5 : Ne recommande jamais**

Border Guard ne fournit **jamais** de recommandation à StrongFather. Il rapporte des classifications (niveaux de confiance, règles de franchissement), pas des conseils ou des suggestions de décision.

**INV-BG-SF-NEVER-6 : N'exécute jamais**

Border Guard n'exécute **jamais** d'action. Il définit les règles de franchissement, mais l'application de ces règles appartient à BondingBrother ou aux autres cores opérationnels, jamais à Border Guard.

---

## 6. Types d'informations échangées

### 6.1 Information de contexte de frontière

**BOUNDARY_CONTEXT**
- **Objectif :** Fournir le contexte de frontière pour une intention
- **Contenu :** Frontière(s) traversée(s), niveau de confiance de la source, règles applicables
- **Fréquence :** Sur demande de StrongFather

**Structure du contexte de frontière :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `context_id` | Identifiant unique du contexte | ✅ Oui |
| `source_trust_level` | Niveau de confiance de la source (trusted, verified, unknown, hostile) | ✅ Oui |
| `boundaries_crossed` | Liste des frontières traversées | ✅ Oui |
| `crossing_rules` | Règles de franchissement applicables | ✅ Oui |
| `integration_state` | État de l'intégration concernée (si applicable) | ❌ Optionnel |
| `timestamp` | Horodatage de la classification | ✅ Oui |

### 6.2 Information de niveau de confiance

**TRUST_LEVEL_INFO**
- **Objectif :** Fournir le niveau de confiance d'une source spécifique
- **Contenu :** Niveau de confiance, critères appliqués, historique de classification
- **Usage :** Enrichissement du contexte décisionnel

**Structure du niveau de confiance :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `trust_level` | Niveau (trusted, verified, unknown, hostile) | ✅ Oui |
| `criteria_applied` | Critères ayant déterminé la classification | ✅ Oui |
| `source_identifier` | Identifiant de la source classifiée | ✅ Oui |
| `classification_date` | Date de la classification | ✅ Oui |
| `previous_level` | Niveau précédent (si transition) | ❌ Optionnel |

### 6.3 Information de règles de franchissement

**CROSSING_RULES_INFO**
- **Objectif :** Fournir les règles de franchissement pour une frontière
- **Contenu :** Conditions déclaratives, niveau de confiance requis, restrictions
- **Usage :** Contextualisation de l'évaluation d'intention

**Structure des règles de franchissement :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `rule_id` | Identifiant unique de la règle | ✅ Oui |
| `boundary_id` | Identifiant de la frontière concernée | ✅ Oui |
| `required_trust_level` | Niveau de confiance minimum requis | ✅ Oui |
| `conditions` | Conditions déclaratives à satisfaire | ✅ Oui |
| `restrictions` | Restrictions applicables | ❌ Optionnel |

### 6.4 Information d'état d'intégration

**INTEGRATION_STATE_INFO**
- **Objectif :** Fournir l'état d'une intégration avec un système externe
- **Contenu :** État (active, suspendue, révoquée), niveau de confiance, frontières associées
- **Usage :** Contextualisation des intentions provenant d'intégrations

**Structure de l'état d'intégration :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `integration_id` | Identifiant unique de l'intégration | ✅ Oui |
| `state` | État (active, suspended, revoked) | ✅ Oui |
| `trust_level` | Niveau de confiance de l'intégration | ✅ Oui |
| `boundaries` | Frontières associées à cette intégration | ✅ Oui |
| `last_state_change` | Dernière modification d'état | ❌ Optionnel |

---

## 7. Types de consultations

### 7.1 Consultation de contexte de frontière

**GET_BOUNDARY_CONTEXT**
- **Initiateur :** StrongFather
- **Objectif :** Obtenir le contexte de frontière pour une intention
- **Payload :** Identifiant de l'intention, source de l'intention
- **Réponse :** Contexte de frontière complet

**Règle BG-SF-QUERY-01 : Réponse instantanée**

La réponse à une consultation de contexte est instantanée. Border Guard retourne le contexte connu au moment de la demande, sans délai.

### 7.2 Consultation de niveau de confiance

**GET_TRUST_LEVEL**
- **Initiateur :** StrongFather
- **Objectif :** Obtenir le niveau de confiance d'une source spécifique
- **Payload :** Identifiant de la source
- **Réponse :** Niveau de confiance avec critères

**Règle BG-SF-QUERY-02 : Source non classifiée**

Si la source n'a pas été explicitement classifiée, Border Guard retourne `unknown` conformément à l'invariant INV-BG-4 (classification exhaustive).

### 7.3 Consultation de règles de franchissement

**GET_CROSSING_RULES**
- **Initiateur :** StrongFather
- **Objectif :** Obtenir les règles de franchissement pour une frontière
- **Payload :** Identifiant de la frontière, direction (entrée, sortie)
- **Réponse :** Règles de franchissement déclaratives

**Règle BG-SF-QUERY-03 : Règles complètes**

Border Guard retourne toutes les règles applicables à la frontière demandée. Les règles sont déclaratives et expriment ce qui est requis, pas comment le vérifier.

### 7.4 Consultation d'état d'intégration

**GET_INTEGRATION_STATE**
- **Initiateur :** StrongFather
- **Objectif :** Obtenir l'état d'une intégration avec un système externe
- **Payload :** Identifiant de l'intégration
- **Réponse :** État complet de l'intégration

**Règle BG-SF-QUERY-04 : Intégration inconnue**

Si l'intégration demandée n'est pas gouvernée par Border Guard, la réponse est `NOT_FOUND` avec indication que l'intégration n'est pas dans le registre.

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

**Règle BG-SF-PROT-01 : Format standardisé**

Toutes les consultations respectent le format standardisé. Aucune consultation ad-hoc n'est acceptée.

### 8.2 Format des réponses

Les réponses de Border Guard suivent un format standardisé.

**Structure de base :**

| Élément | Description | Obligatoire |
|---------|-------------|-------------|
| `response_id` | Identifiant unique de la réponse | ✅ Oui |
| `query_id` | Référence à la consultation | ✅ Oui |
| `status` | Statut de la réponse (SUCCESS, NOT_FOUND, UNKNOWN_SOURCE, ERROR) | ✅ Oui |
| `data` | Données de la réponse | Si SUCCESS |
| `error` | Détails de l'erreur | Si ERROR |
| `timestamp` | Horodatage de la réponse | ✅ Oui |

**Règle BG-SF-PROT-02 : Réponse toujours structurée**

Border Guard retourne toujours une réponse structurée, même en cas d'erreur ou de source non classifiée.

**Règle BG-SF-PROT-03 : Pas d'interprétation décisionnelle**

Les réponses sont des informations de classification brutes. Border Guard n'interprète pas les données pour StrongFather et ne suggère jamais de décision.

### 8.3 Statuts de réponse

| Statut | Signification |
|--------|---------------|
| `SUCCESS` | La consultation a abouti, les données sont fournies |
| `NOT_FOUND` | L'élément recherché (frontière, intégration) n'existe pas |
| `UNKNOWN_SOURCE` | La source n'est pas explicitement classifiée (niveau `unknown` retourné) |
| `ERROR` | Une erreur interne s'est produite |

**Règle BG-SF-PROT-04 : UNKNOWN_SOURCE n'est pas une erreur**

Le statut `UNKNOWN_SOURCE` est une réponse valide, pas une erreur. Il indique que la source sera traitée avec le niveau de confiance `unknown` par défaut.

---

## 9. Flux d'intégration typique

### 9.1 Flux de consultation avant évaluation

**Acteurs :** BondingBrother, StrongFather, Border Guard

**Séquence :**

1. BondingBrother soumet une intention à StrongFather pour évaluation
2. StrongFather identifie que l'intention vient de l'extérieur ou traverse une frontière
3. StrongFather interroge Border Guard : `GET_BOUNDARY_CONTEXT`
4. Border Guard retourne le contexte de frontière (niveau de confiance, règles)
5. StrongFather intègre le contexte dans l'évaluation de l'intention
6. StrongFather évalue l'intention selon les politiques (en tenant compte du contexte)
7. StrongFather produit une décision (acceptée, refusée, ambiguë)

**Règle BG-SF-FLOW-01 : Consultation optionnelle**

La consultation de Border Guard par StrongFather est toujours optionnelle. StrongFather peut évaluer une intention sans consulter le contexte de frontière.

### 9.2 Flux de classification pour authentification en couches

**Acteurs :** Border Guard, StrongFather (selon RT-SEC-2)

**Séquence :**

1. Une requête arrive avec une source identifiée
2. Border Guard classifie la source selon ses critères
3. StrongFather consulte Border Guard pour le niveau de confiance
4. StrongFather utilise ce niveau dans l'authentification en couches
5. Master Butler vérifie les capacités selon le niveau de confiance
6. StrongFather produit la décision finale

### 9.3 Diagramme de séquence

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  BondingBrother │    │   StrongFather  │    │  Border Guard   │
└────────┬────────┘    └────────┬────────┘    └────────┬────────┘
         │                      │                      │
         ├── Intention ────────►│                      │
         │                      │                      │
         │                      ├── GET_BOUNDARY_CTX ─►│
         │                      │                      │
         │                      │◄── Contexte ─────────┤
         │                      │    (trust level,     │
         │                      │     rules, etc.)     │
         │                      │                      │
         │                      ├── Évaluation ────────┤
         │                      │   (avec contexte)    │
         │                      │                      │
         │◄── Décision ─────────┤                      │
         │                      │                      │
```

---

## 10. Règles d'intégration

### 10.1 Règles de communication

**Règle BG-SF-INT-01 : Initiative StrongFather**

StrongFather initie les consultations. Border Guard répond aux consultations. Border Guard ne pousse jamais d'information vers StrongFather de manière non sollicitée.

**Règle BG-SF-INT-02 : Pas de dépendance obligatoire**

StrongFather peut fonctionner sans consulter Border Guard. L'intégration enrichit le contexte mais n'est pas obligatoire.

**Règle BG-SF-INT-03 : Réponses synchrones**

Les réponses aux consultations sont synchrones et instantanées. Aucune consultation n'est différée.

### 10.2 Règles de données

**Règle BG-SF-INT-04 : Données actuelles**

Les données retournées par Border Guard reflètent les classifications actuelles au moment de la consultation.

**Règle BG-SF-INT-05 : Classifications stables**

Les classifications de Border Guard sont stables. Un même élément consulté deux fois retourne le même niveau de confiance (sauf modification explicite de la classification).

**Règle BG-SF-INT-06 : Cohérence interne garantie**

Border Guard garantit la cohérence interne des données retournées. Un contexte de frontière et ses règles sont mutuellement cohérents.

### 10.3 Règles de traçabilité

**Règle BG-SF-INT-07 : Traçabilité des consultations**

Toutes les consultations de StrongFather sont tracées par Border Guard avec le contexte complet.

**Règle BG-SF-INT-08 : Corrélation intention-consultation**

Chaque consultation peut être corrélée à une intention en cours d'évaluation (si `intention_id` fourni) pour l'audit bout-en-bout.

---

## 11. Gestion des erreurs

### 11.1 Types d'erreurs

**Erreurs de format :**
- Consultation mal formée
- Champ obligatoire manquant
- Type de consultation inconnu

**Erreurs de données :**
- Frontière non définie (NOT_FOUND)
- Intégration non gouvernée (NOT_FOUND)
- Source non classifiée (UNKNOWN_SOURCE, pas une erreur)

**Erreurs internes :**
- Erreur du moteur de définition de frontières
- Erreur de calcul de règles

### 11.2 Traitement des erreurs

**Règle BG-SF-ERR-01 : Réponse structurée toujours**

Border Guard retourne toujours une réponse structurée, même en cas d'erreur. StrongFather peut toujours interpréter la réponse.

**Règle BG-SF-ERR-02 : UNKNOWN_SOURCE est informatif**

Le statut `UNKNOWN_SOURCE` est une information, pas une erreur. StrongFather peut utiliser cette information (source non classifiée = niveau `unknown`).

**Règle BG-SF-ERR-03 : Journalisation des erreurs**

Toutes les erreurs sont journalisées par Border Guard pour audit et diagnostic.

**Règle BG-SF-ERR-04 : Pas de retry automatique**

En cas d'erreur, StrongFather décide de la stratégie (retry, continuer sans contexte). Border Guard ne retry jamais automatiquement.

---

## 12. Cas particuliers

### 12.1 Source hostile

Lorsqu'une source est classifiée `hostile`, Border Guard retourne cette classification sans bloquer.

**Règle BG-SF-CASE-01 : Hostile est une classification, pas un blocage**

La classification `hostile` est une information factuelle. C'est StrongFather qui décide, selon ses politiques, si une intention d'une source `hostile` doit être refusée.

### 12.2 Intégration suspendue

Lorsqu'une intégration est suspendue, Border Guard retourne l'état `suspended`.

**Règle BG-SF-CASE-02 : Suspended est un état, pas un blocage**

L'état `suspended` est une information factuelle. C'est StrongFather qui décide, selon ses politiques, comment traiter les intentions venant d'une intégration suspendue.

### 12.3 Frontière non définie

Si une frontière demandée n'est pas définie par Border Guard :

**Règle BG-SF-CASE-03 : Frontière non définie = NOT_FOUND**

Border Guard retourne `NOT_FOUND`. StrongFather peut décider de traiter l'intention sans contexte de frontière ou de la refuser selon ses politiques.

---

## 13. Garanties de l'intégration

### 13.1 Garantie d'exhaustivité

**Engagement :** Les réponses de Border Guard sont exhaustives pour le périmètre de définition. Toutes les informations connues sur une frontière ou une classification sont fournies.

### 13.2 Garantie d'exactitude

**Engagement :** Les informations fournies par Border Guard sont exactes et reflètent les définitions actuelles au moment de la consultation.

### 13.3 Garantie de neutralité

**Engagement :** Border Guard fournit des informations de classification sans interprétation décisionnelle, sans recommandation, sans jugement. La décision appartient exclusivement à StrongFather.

### 13.4 Garantie de traçabilité

**Engagement :** Toute interaction entre StrongFather et Border Guard est traçable de bout en bout. L'audit complet des consultations et réponses est possible.

### 13.5 Garantie de disponibilité

**Engagement :** Border Guard est disponible pour répondre aux consultations de StrongFather sans dépendance externe (conformité LOI-1).

### 13.6 Garantie de non-blocage

**Engagement :** Border Guard ne bloque jamais les opérations de StrongFather. Les consultations sont répondues immédiatement.

---

## 14. Invariants de l'intégration

### 14.1 Invariants de relation

**INV-BG-SF-1 : Conseil unidirectionnel**

Border Guard conseille StrongFather. Border Guard ne décide jamais pour StrongFather.

**INV-BG-SF-2 : Consultation facultative**

StrongFather consulte Border Guard de manière facultative. Aucune consultation n'est obligatoire.

**INV-BG-SF-3 : Aucune autorité partagée**

Border Guard n'a aucune autorité sur les décisions. StrongFather n'a aucune autorité sur les définitions de frontières.

### 14.2 Invariants de données

**INV-BG-SF-4 : Lecture pure**

Les consultations sont des lectures pures. Aucune modification des définitions n'est causée par une consultation.

**INV-BG-SF-5 : Données de classification**

Les données retournées sont des classifications (niveaux de confiance, règles). Aucune donnée interprétée décisionnellement n'est retournée.

### 14.3 Invariants de protocole

**INV-BG-SF-6 : Format respecté**

Toutes les consultations et réponses respectent le format standardisé.

**INV-BG-SF-7 : Traçabilité complète**

Toute interaction est traçable avec son contexte complet.

---

## 15. Conformité aux Lois d'Autonomie Système

### LOI-1 : Aucune dépendance externe critique

**Conformité :** ✅ **Conforme**

L'intégration respecte LOI-1 :
- Border Guard définit les frontières localement, sans dépendance externe
- StrongFather consulte localement, sans dépendance externe
- L'absence de connexion ne bloque ni la définition ni la consultation

### LOI-2 : Le système accepte l'isolement comme état normal

**Conformité :** ✅ **Conforme**

L'intégration respecte LOI-2 :
- L'isolement ne modifie pas les classifications de Border Guard
- StrongFather peut prendre des décisions même en état isolé
- Aucune dégradation de l'intégration en mode isolé

### LOI-4 : Pas de temps global requis

**Conformité :** ✅ **Conforme**

L'intégration respecte LOI-4 :
- Les horodatages sont locaux
- Aucune synchronisation temporelle n'est requise
- Les classifications ne dépendent pas de timestamps synchronisés

---

## 16. Exemples

### 16.1 Consultation de contexte de frontière

**Consultation StrongFather :**
```
{
  "query_id": "q-sf-bg-001",
  "intention_id": "intention-500",
  "type": "GET_BOUNDARY_CONTEXT",
  "payload": {
    "source": "external-api-partner-x",
    "target": "internal-content-module"
  },
  "contexte_appelant": {
    "source": "strongfather",
    "evaluation_id": "eval-200"
  },
  "timestamp": "2026-01-27T14:00:00Z"
}
```

**Réponse Border Guard :**
```
{
  "response_id": "r-bg-001",
  "query_id": "q-sf-bg-001",
  "status": "SUCCESS",
  "data": {
    "context_id": "ctx-001",
    "source_trust_level": "verified",
    "boundaries_crossed": [
      {
        "boundary_id": "boundary-external-001",
        "type": "integration",
        "direction": "inbound"
      }
    ],
    "crossing_rules": [
      {
        "rule_id": "rule-001",
        "required_trust_level": "verified",
        "conditions": ["api_key_valid", "rate_limit_respected"]
      }
    ],
    "integration_state": {
      "integration_id": "partner-x",
      "state": "active",
      "trust_level": "verified"
    },
    "timestamp": "2026-01-27T14:00:00Z"
  },
  "timestamp": "2026-01-27T14:00:01Z"
}
```

### 16.2 Source non classifiée

**Consultation StrongFather :**
```
{
  "query_id": "q-sf-bg-002",
  "type": "GET_TRUST_LEVEL",
  "payload": {
    "source": "unknown-external-request"
  },
  "contexte_appelant": {
    "source": "strongfather"
  },
  "timestamp": "2026-01-27T15:00:00Z"
}
```

**Réponse Border Guard :**
```
{
  "response_id": "r-bg-002",
  "query_id": "q-sf-bg-002",
  "status": "UNKNOWN_SOURCE",
  "data": {
    "trust_level": "unknown",
    "criteria_applied": ["default_classification"],
    "source_identifier": "unknown-external-request",
    "classification_date": "2026-01-27T15:00:00Z"
  },
  "timestamp": "2026-01-27T15:00:01Z"
}
```

**Note :** StrongFather utilise cette information pour appliquer ses politiques concernant les sources `unknown`.

---

## 17. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit l'interface et le protocole que Border Guard doit respecter pour s'intégrer avec StrongFather.

Toute implémentation de l'intégration avec StrongFather doit respecter ce contrat. Toute violation entraîne un comportement non conforme.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dépendances :**
- Border Guard - Documentation Fondatrice v1.5 (Section 8)
- StrongFather - Documentation Fondatrice v1.5
- Miyukini Conceptual References - Security Protocols v1.0 (RT-SEC-2)
- Miyukini Conceptual References - Lois Autonomie Systeme v1.1

---

## 18. Mini log de génération

### Décision éditoriale E1 : Direction de la relation

**Décision prise :** La relation est de conseil : Border Guard informe, StrongFather décide. Cette direction respecte la Documentation Fondatrice de Border Guard Section 8 qui définit "Border Guard informe Strong Father sur le contexte de frontière ; Strong Father décide".

**Application :** Tout le document est structuré autour de cette relation de conseil unidirectionnel.

### Décision éditoriale E2 : Consultation facultative

**Décision prise :** La consultation de Border Guard par StrongFather est explicitement facultative. StrongFather peut évaluer des intentions sans consulter le contexte de frontière.

**Application :** Règle BG-SF-02 et INV-BG-SF-2 établissent cette facultativité.

### Warning W1 : Risque de confusion définition/décision

**Warning rencontré :** Risque que les définitions de frontières de Border Guard soient confondues avec des décisions.

**Décision prise :** Les interdictions absolues (Section 5) clarifient que Border Guard ne décide jamais. Les définitions sont des classifications, pas des décisions.

**Correction effectuée :** Section 5 explicite les interdictions, Section 13.3 garantit la neutralité.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec Border Guard - Documentation Fondatrice : Confirmée (relation de conseil, pas de décision)
- ✅ Cohérence avec StrongFather - Documentation Fondatrice : Confirmée (StrongFather décide, consulte le contexte)
- ✅ Conformité LOI-1 : Confirmée (aucune dépendance externe)
- ✅ Conformité LOI-2 : Confirmée (isolement n'affecte pas l'intégration)
- ✅ Conformité LOI-4 : Confirmée (pas de temps global requis)
- ✅ Aucune autorité de Border Guard sur les décisions : Confirmée (INV-BG-SF-1, Section 5)
- ✅ Traçabilité complète : Confirmée (INV-BG-SF-7)

**Conclusion :** Aucune contradiction détectée. Le document est cohérent et non ambigu.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
