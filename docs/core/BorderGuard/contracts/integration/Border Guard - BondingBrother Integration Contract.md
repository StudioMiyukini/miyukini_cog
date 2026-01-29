# Border Guard - BondingBrother Integration Contract

## 1. Contexte

Ce document définit le **contrat d'intégration entre Border Guard et BondingBrother**. Il spécifie l'interface, le protocole, les règles de communication, et les garanties associées à l'intégration avec BondingBrother en tant que médiateur fraternel de l'écosystème.

Ce document complète la Section 8 de la [Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [BondingBrother - Documentation Fondatrice](../../../BondingBrother/BondingBrother%20-%20Documentation%20Fondatrice.md) pour la nature de BondingBrother
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) pour la conformité LOI-1 à LOI-6
- [Miyukini Conceptual References - Security Protocols](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md) pour les protocoles de sécurité

L'intégration respecte les Lois d'Autonomie Système : toutes les définitions de règles sont locales et ne requièrent aucune dépendance externe (**LOI-1**).

## 2. Portée / Scope

Ce document couvre :
- L'interface contractuelle entre Border Guard et BondingBrother
- Le protocole de communication (consultation des règles de franchissement)
- Les types d'informations échangées
- Les règles d'intégration spécifiques
- Le rôle dans la fédération (LOI-6)
- Les garanties de l'intégration

Ce document **ne couvre pas** :
- Les détails internes de BondingBrother (voir documentation BondingBrother)
- Les détails internes du moteur de définition de règles (voir Architecture)
- L'intégration avec StrongFather (voir StrongFather Integration Contract)
- L'intégration avec Caring Nanny (voir CaringNanny Integration Contract)

---

## 3. Principe fondamental

**Border Guard définit les règles de franchissement des frontières. BondingBrother applique ces règles lors de la médiation entre les produits et l'écosystème. Border Guard ne filtre jamais lui-même, BondingBrother ne définit jamais de frontière.**

La relation est de **définition/application** : Border Guard est l'autorité conceptuelle qui définit les règles, BondingBrother est l'exécutant opérationnel qui les applique. Cette séparation est absolue et non négociable.

---

## 4. Nature de la relation Border Guard — BondingBrother

### 4.1 Relation de définition/application

**Border Guard fournit à BondingBrother :**
- Les règles de franchissement pour chaque frontière
- Les niveaux de confiance des sources et destinations
- Les conditions déclaratives à vérifier
- L'état des intégrations avec les systèmes externes

**BondingBrother consulte Border Guard pour :**
- Obtenir les règles applicables avant médiation
- Connaître le niveau de confiance d'une source
- Vérifier si une frontière peut être franchie
- Connaître l'état d'une intégration

**Règle BG-BB-01 : Définition sans application**

Border Guard définit les règles mais ne les applique jamais. L'application concrète des règles (filtrage, blocage, validation technique) est exclusivement du ressort de BondingBrother.

**Règle BG-BB-02 : Application sans définition**

BondingBrother applique les règles mais ne les définit jamais. Toute règle de franchissement provient exclusivement de Border Guard.

**Règle BG-BB-03 : Séparation non négociable**

La séparation entre définition (Border Guard) et application (BondingBrother) est non négociable. Aucune exception n'est autorisée.

### 4.2 Séparation des responsabilités

| Responsabilité | Border Guard | BondingBrother |
|----------------|--------------|----------------|
| **Définir les frontières** | ✅ Exclusif | ❌ Jamais |
| **Définir les règles de franchissement** | ✅ Exclusif | ❌ Jamais |
| **Classifier les niveaux de confiance** | ✅ Exclusif | ❌ Jamais |
| **Appliquer les règles** | ❌ Jamais | ✅ Exclusif |
| **Filtrer les interactions** | ❌ Jamais | ✅ Exclusif |
| **Médiatiser les intentions** | ❌ Jamais | ✅ Exclusif |
| **Traduire les demandes** | ❌ Jamais | ✅ Exclusif |
| **Bloquer les accès non autorisés** | ❌ Jamais | ✅ Selon décision StrongFather |

**Règle BG-BB-04 : Aucun chevauchement**

Aucun chevauchement de responsabilités n'est autorisé. Border Guard ne filtre jamais, BondingBrother ne classifie jamais.

### 4.3 Rôle critique dans la fédération (LOI-6)

Dans le contexte de l'autonomie système et de la fédération :

**Border Guard définit :**
- Les règles de fédération (ce qui peut être partagé)
- Les frontières entre nœuds fédérés
- Les niveaux de confiance des nœuds partenaires
- Les conditions de validation des échanges fédérés

**BondingBrother applique :**
- Les règles de fédération définies par Border Guard
- Le filtrage des échanges inter-nœuds
- La traçabilité des communications fédérées
- La réversibilité de la fédération

Cette collaboration garantit que la fédération est **explicite** (décision consciente), **contrôlée** (règles définies), **observable** (traçabilité), et **réversible** (possibilité de quitter).

---

## 5. Ce que Border Guard ne fait JAMAIS vis-à-vis de BondingBrother

### 5.1 Interdictions absolues

**INV-BG-BB-NEVER-1 : Ne filtre jamais**

Border Guard ne filtre **jamais** les interactions traversant une frontière. Le filtrage est une action d'application, pas de définition. Border Guard définit les règles de filtrage ; BondingBrother les applique.

**INV-BG-BB-NEVER-2 : Ne bloque jamais**

Border Guard ne bloque **jamais** les accès. Le blocage est une action d'exécution. Border Guard définit les conditions qui peuvent conduire à un blocage ; BondingBrother ou StrongFather exécute le blocage.

**INV-BG-BB-NEVER-3 : N'intercepte jamais**

Border Guard n'intercepte **jamais** les communications. L'interception et la médiation sont du ressort exclusif de BondingBrother.

**INV-BG-BB-NEVER-4 : Ne traduit jamais**

Border Guard ne traduit **jamais** les intentions des produits. La traduction entre le vocabulaire des produits et celui des autorités est du ressort exclusif de BondingBrother.

**INV-BG-BB-NEVER-5 : N'exécute jamais**

Border Guard n'exécute **jamais** d'action technique. Il définit des règles conceptuelles ; l'exécution technique appartient à BondingBrother et aux autres cores opérationnels.

**INV-BG-BB-NEVER-6 : Ne décide jamais**

Border Guard ne décide **jamais** d'accepter ou refuser une intention. La décision appartient à StrongFather. Border Guard fournit le contexte de règles, BondingBrother applique, StrongFather décide.

---

## 6. Types d'informations échangées

### 6.1 Information de règles de franchissement

**CROSSING_RULES**
- **Objectif :** Fournir les règles de franchissement pour une frontière
- **Contenu :** Conditions déclaratives, niveau de confiance requis, restrictions
- **Fréquence :** Sur demande de BondingBrother

**Structure des règles de franchissement :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `rule_id` | Identifiant unique de la règle | ✅ Oui |
| `boundary_id` | Identifiant de la frontière concernée | ✅ Oui |
| `boundary_type` | Type (external, internal, integration) | ✅ Oui |
| `direction` | Direction (inbound, outbound, bidirectional) | ✅ Oui |
| `required_trust_level` | Niveau de confiance minimum requis | ✅ Oui |
| `conditions` | Conditions déclaratives à satisfaire | ✅ Oui |
| `restrictions` | Restrictions applicables | ❌ Optionnel |
| `allowed_data_types` | Types de données autorisés à traverser | ❌ Optionnel |

### 6.2 Information de niveau de confiance

**TRUST_CLASSIFICATION**
- **Objectif :** Fournir le niveau de confiance d'une source ou destination
- **Contenu :** Niveau (trusted, verified, unknown, hostile), critères
- **Usage :** Application des règles de franchissement

**Structure de la classification :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `source_identifier` | Identifiant de la source/destination | ✅ Oui |
| `trust_level` | Niveau (trusted, verified, unknown, hostile) | ✅ Oui |
| `criteria_applied` | Critères ayant déterminé la classification | ✅ Oui |
| `classification_date` | Date de la classification | ✅ Oui |

### 6.3 Information d'état d'intégration

**INTEGRATION_STATE**
- **Objectif :** Fournir l'état d'une intégration avec un système externe
- **Contenu :** État (active, suspendue, révoquée), frontières associées
- **Usage :** Application des règles spécifiques aux intégrations

**Structure de l'état d'intégration :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `integration_id` | Identifiant unique de l'intégration | ✅ Oui |
| `state` | État (active, suspended, revoked) | ✅ Oui |
| `trust_level` | Niveau de confiance de l'intégration | ✅ Oui |
| `boundaries` | Frontières associées à cette intégration | ✅ Oui |
| `allowed_operations` | Opérations autorisées | ✅ Oui |
| `last_state_change` | Dernière modification d'état | ❌ Optionnel |

### 6.4 Information de frontière

**BOUNDARY_INFO**
- **Objectif :** Fournir les caractéristiques d'une frontière
- **Contenu :** Type, direction, perméabilité, règles associées
- **Usage :** Identification des frontières traversées

**Structure de la frontière :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `boundary_id` | Identifiant unique de la frontière | ✅ Oui |
| `type` | Type (external, internal, integration) | ✅ Oui |
| `direction` | Direction (inbound, outbound, bidirectional) | ✅ Oui |
| `permeability` | Perméabilité (open, controlled, closed) | ✅ Oui |
| `zones` | Zones connectées par cette frontière | ✅ Oui |

---

## 7. Types de consultations

### 7.1 Consultation des règles de franchissement

**GET_CROSSING_RULES**
- **Initiateur :** BondingBrother
- **Objectif :** Obtenir les règles pour une frontière avant médiation
- **Payload :** Identifiant de la frontière, direction
- **Réponse :** Règles de franchissement complètes

**Règle BG-BB-QUERY-01 : Règles déclaratives**

Les règles retournées sont déclaratives. Elles expriment ce qui est requis (niveau de confiance, conditions), pas comment le vérifier techniquement. L'implémentation technique de la vérification appartient à BondingBrother.

### 7.2 Consultation du niveau de confiance

**GET_TRUST_LEVEL**
- **Initiateur :** BondingBrother
- **Objectif :** Obtenir le niveau de confiance d'une source
- **Payload :** Identifiant de la source
- **Réponse :** Classification de confiance

**Règle BG-BB-QUERY-02 : Classification par défaut**

Si la source n'est pas explicitement classifiée, Border Guard retourne `unknown` conformément à l'invariant INV-BG-4.

### 7.3 Consultation de l'état d'intégration

**GET_INTEGRATION_STATE**
- **Initiateur :** BondingBrother
- **Objectif :** Obtenir l'état d'une intégration externe
- **Payload :** Identifiant de l'intégration
- **Réponse :** État complet de l'intégration

**Règle BG-BB-QUERY-03 : État actuel**

Border Guard retourne l'état actuel de l'intégration. BondingBrother applique les règles correspondant à cet état.

### 7.4 Consultation des frontières traversées

**GET_BOUNDARIES_CROSSED**
- **Initiateur :** BondingBrother
- **Objectif :** Identifier les frontières traversées par une interaction
- **Payload :** Source, destination
- **Réponse :** Liste des frontières avec leurs caractéristiques

**Règle BG-BB-QUERY-04 : Frontières explicites**

Border Guard retourne uniquement les frontières explicitement définies (conformément à INV-BG-5). Si aucune frontière n'est définie entre source et destination, la réponse est vide.

---

## 8. Protocole de communication

### 8.1 Format des consultations

Les consultations de BondingBrother suivent un format standardisé.

**Structure de base :**

| Élément | Description | Obligatoire |
|---------|-------------|-------------|
| `query_id` | Identifiant unique de la consultation | ✅ Oui |
| `intention_id` | Référence à l'intention en cours de médiation | ❌ Optionnel |
| `type` | Type de consultation | ✅ Oui |
| `payload` | Données spécifiques à la consultation | ❌ Selon type |
| `contexte_appelant` | Contexte de BondingBrother | ✅ Oui |
| `timestamp` | Horodatage de la consultation | ✅ Oui |

**Règle BG-BB-PROT-01 : Format standardisé**

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

**Règle BG-BB-PROT-02 : Réponse toujours structurée**

Border Guard retourne toujours une réponse structurée, même en cas d'erreur.

**Règle BG-BB-PROT-03 : Règles sans implémentation**

Les règles retournées sont purement déclaratives. Border Guard ne fournit jamais de code ou de logique d'implémentation.

### 8.3 Statuts de réponse

| Statut | Signification |
|--------|---------------|
| `SUCCESS` | La consultation a abouti, les données sont fournies |
| `NOT_FOUND` | L'élément recherché (frontière, intégration) n'existe pas |
| `UNKNOWN_SOURCE` | La source n'est pas explicitement classifiée (niveau `unknown` retourné) |
| `ERROR` | Une erreur interne s'est produite |

---

## 9. Flux d'intégration typique

### 9.1 Flux de médiation avec vérification de frontière

**Acteurs :** Produit, BondingBrother, Border Guard, StrongFather

**Séquence :**

1. Produit exprime une intention via BondingBrother
2. BondingBrother identifie qu'une frontière est potentiellement traversée
3. BondingBrother interroge Border Guard : `GET_BOUNDARIES_CROSSED`
4. Border Guard retourne les frontières identifiées
5. BondingBrother interroge Border Guard : `GET_CROSSING_RULES`
6. Border Guard retourne les règles déclaratives
7. BondingBrother applique les règles et prépare le contexte pour StrongFather
8. StrongFather évalue et décide
9. BondingBrother exécute la décision

**Règle BG-BB-FLOW-01 : Consultation avant application**

BondingBrother doit consulter Border Guard pour obtenir les règles avant d'appliquer un filtrage ou une restriction liée aux frontières.

### 9.2 Flux de fédération inter-nœuds

**Acteurs :** BondingBrother A, Border Guard A, BondingBrother B (nœud distant)

**Séquence :**

1. BondingBrother A reçoit une demande d'échange fédéré
2. BondingBrother A interroge Border Guard A : `GET_CROSSING_RULES` (frontière fédération)
3. Border Guard A retourne les règles de fédération
4. BondingBrother A vérifie les conditions et le niveau de confiance du nœud B
5. Si conforme, BondingBrother A procède à l'échange
6. L'échange est journalisé (traçabilité LOI-6)

### 9.3 Diagramme de séquence

```
┌────────────┐  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│  Produit   │  │  BondingBrother │  │   Border Guard  │  │   StrongFather  │
└─────┬──────┘  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘
      │                  │                    │                    │
      ├── Intention ────►│                    │                    │
      │                  │                    │                    │
      │                  ├── GET_BOUNDARIES ─►│                    │
      │                  │                    │                    │
      │                  │◄── Frontières ─────┤                    │
      │                  │                    │                    │
      │                  ├── GET_RULES ──────►│                    │
      │                  │                    │                    │
      │                  │◄── Règles ─────────┤                    │
      │                  │    (déclaratives)  │                    │
      │                  │                    │                    │
      │                  ├── Application ─────┼────────────────────┤
      │                  │    (vérifie règles)│                    │
      │                  │                    │                    │
      │                  ├── Demande décision ┼───────────────────►│
      │                  │                    │                    │
      │                  │◄── Décision ───────┼────────────────────┤
      │                  │                    │                    │
      │◄── Résultat ─────┤                    │                    │
      │                  │                    │                    │
```

---

## 10. Règles d'intégration

### 10.1 Règles de communication

**Règle BG-BB-INT-01 : Initiative BondingBrother**

BondingBrother initie les consultations. Border Guard répond aux consultations. Border Guard ne pousse jamais d'information vers BondingBrother de manière non sollicitée.

**Règle BG-BB-INT-02 : Consultation avant application**

BondingBrother doit consulter Border Guard avant d'appliquer une règle de frontière. Aucune règle ne peut être appliquée sans consultation préalable.

**Règle BG-BB-INT-03 : Réponses synchrones**

Les réponses aux consultations sont synchrones et instantanées. Aucune consultation n'est différée.

### 10.2 Règles d'application

**Règle BG-BB-INT-04 : Application fidèle**

BondingBrother applique fidèlement les règles définies par Border Guard. Aucune interprétation créative ou modification des règles n'est autorisée.

**Règle BG-BB-INT-05 : Pas de règle inventée**

BondingBrother n'invente jamais de règle de franchissement. Toute règle appliquée provient exclusivement de Border Guard.

**Règle BG-BB-INT-06 : Traçabilité de l'application**

Toute application de règle par BondingBrother est traçable avec référence à la règle source de Border Guard.

### 10.3 Règles de fédération

**Règle BG-BB-INT-07 : Fédération contrôlée**

Les échanges fédérés entre nœuds sont contrôlés par les règles de Border Guard. BondingBrother applique ces règles sans exception.

**Règle BG-BB-INT-08 : Réversibilité garantie**

Border Guard peut modifier les règles de fédération à tout moment. BondingBrother applique immédiatement les nouvelles règles, permettant la réversibilité de la fédération.

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
- Incohérence interne des règles

### 11.2 Traitement des erreurs

**Règle BG-BB-ERR-01 : Réponse structurée toujours**

Border Guard retourne toujours une réponse structurée, même en cas d'erreur.

**Règle BG-BB-ERR-02 : NOT_FOUND = pas de règle**

Si une frontière n'est pas trouvée (NOT_FOUND), BondingBrother considère qu'aucune règle de franchissement ne s'applique. L'intention peut être médiée sans restriction de frontière.

**Règle BG-BB-ERR-03 : UNKNOWN_SOURCE = niveau unknown**

Si une source n'est pas classifiée, BondingBrother applique les règles pour le niveau `unknown`.

**Règle BG-BB-ERR-04 : Journalisation**

Toutes les erreurs sont journalisées par les deux parties pour audit et diagnostic.

---

## 12. Cas particuliers

### 12.1 Frontière fermée

Lorsqu'une frontière a une perméabilité `closed` :

**Règle BG-BB-CASE-01 : Closed est une définition**

Border Guard définit la frontière comme `closed`. BondingBrother applique cette définition en refusant les franchissements ou en les soumettant à StrongFather pour décision exceptionnelle.

### 12.2 Intégration révoquée

Lorsqu'une intégration est révoquée :

**Règle BG-BB-CASE-02 : Révoquée = règles appliquées**

Border Guard retourne l'état `revoked`. BondingBrother applique les règles correspondantes (généralement blocage des communications avec cette intégration).

### 12.3 Mode offline

Lorsque le système est en mode offline :

**Règle BG-BB-CASE-03 : Règles locales**

Border Guard retourne les règles locales sans dépendance externe. BondingBrother applique ces règles normalement. L'intégration fonctionne sans dégradation en mode offline (LOI-1, LOI-2).

---

## 13. Garanties de l'intégration

### 13.1 Garantie de séparation

**Engagement :** La séparation entre définition (Border Guard) et application (BondingBrother) est absolue. Aucune exception n'est possible.

### 13.2 Garantie d'exhaustivité des règles

**Engagement :** Border Guard fournit toutes les règles applicables à une frontière. Aucune règle cachée ou implicite n'existe.

### 13.3 Garantie de cohérence

**Engagement :** Les règles fournies par Border Guard sont cohérentes entre elles. Aucune contradiction n'est possible.

### 13.4 Garantie de traçabilité

**Engagement :** Toute consultation et application est traçable de bout en bout.

### 13.5 Garantie de disponibilité

**Engagement :** Border Guard est disponible pour répondre aux consultations sans dépendance externe (LOI-1).

### 13.6 Garantie de neutralité technique

**Engagement :** Les règles de Border Guard sont conceptuelles et neutres techniquement. BondingBrother choisit l'implémentation technique de leur application.

---

## 14. Invariants de l'intégration

### 14.1 Invariants de relation

**INV-BG-BB-1 : Définition/Application**

Border Guard définit, BondingBrother applique. Cette relation est non négociable.

**INV-BG-BB-2 : Pas de filtrage par Border Guard**

Border Guard ne filtre jamais. Tout filtrage est effectué par BondingBrother.

**INV-BG-BB-3 : Pas de définition par BondingBrother**

BondingBrother ne définit jamais de frontière ou de règle. Toute définition provient de Border Guard.

### 14.2 Invariants de données

**INV-BG-BB-4 : Règles déclaratives**

Les règles sont toujours déclaratives. Aucune logique procédurale n'est fournie.

**INV-BG-BB-5 : Application fidèle**

BondingBrother applique fidèlement les règles sans interprétation ou modification.

### 14.3 Invariants de protocole

**INV-BG-BB-6 : Format respecté**

Toutes les consultations et réponses respectent le format standardisé.

**INV-BG-BB-7 : Traçabilité complète**

Toute interaction est traçable avec son contexte complet.

---

## 15. Conformité aux Lois d'Autonomie Système

### LOI-1 : Aucune dépendance externe critique

**Conformité :** ✅ **Conforme**

L'intégration respecte LOI-1 :
- Les règles de Border Guard sont locales
- L'application par BondingBrother est locale
- L'absence de connexion ne bloque ni la définition ni l'application

### LOI-2 : Le système accepte l'isolement comme état normal

**Conformité :** ✅ **Conforme**

L'intégration respecte LOI-2 :
- L'isolement ne modifie pas les règles de Border Guard
- BondingBrother peut appliquer les règles en mode isolé
- Aucune dégradation de l'intégration en mode offline

### LOI-6 : L'autonomie n'empêche pas la fédération

**Conformité :** ✅ **Conforme — Rôle critique**

L'intégration est critique pour LOI-6 :
- Border Guard définit les règles de fédération
- BondingBrother applique ces règles pour les échanges inter-nœuds
- La fédération est explicite, contrôlée, observable, et réversible

---

## 16. Exemples

### 16.1 Consultation des règles de franchissement

**Consultation BondingBrother :**
```
{
  "query_id": "q-bb-bg-001",
  "intention_id": "intention-500",
  "type": "GET_CROSSING_RULES",
  "payload": {
    "boundary_id": "boundary-external-001",
    "direction": "inbound"
  },
  "contexte_appelant": {
    "source": "bondingbrother",
    "mediation_id": "med-100"
  },
  "timestamp": "2026-01-27T14:00:00Z"
}
```

**Réponse Border Guard :**
```
{
  "response_id": "r-bg-001",
  "query_id": "q-bb-bg-001",
  "status": "SUCCESS",
  "data": {
    "rules": [
      {
        "rule_id": "rule-001",
        "boundary_id": "boundary-external-001",
        "boundary_type": "external",
        "direction": "inbound",
        "required_trust_level": "verified",
        "conditions": [
          "authentication_valid",
          "rate_limit_respected",
          "payload_size_within_limit"
        ],
        "restrictions": [
          "no_admin_operations",
          "read_only_for_unknown"
        ]
      }
    ]
  },
  "timestamp": "2026-01-27T14:00:01Z"
}
```

### 16.2 Application des règles par BondingBrother

**Exemple d'application :**

BondingBrother reçoit les règles ci-dessus et :
1. Vérifie que `authentication_valid` est satisfait (via le produit/auth)
2. Vérifie que `rate_limit_respected` (compteur local)
3. Vérifie que `payload_size_within_limit` (inspection du payload)
4. Si source avec niveau `unknown`, applique `read_only_for_unknown`
5. Bloque `admin_operations` conformément à `no_admin_operations`

**Note :** L'implémentation technique de chaque vérification appartient à BondingBrother. Border Guard a seulement fourni les conditions déclaratives.

---

## 17. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit l'interface et le protocole que Border Guard et BondingBrother doivent respecter pour leur intégration.

Toute implémentation de l'intégration doit respecter ce contrat. Toute violation entraîne un comportement non conforme.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dépendances :**
- Border Guard - Documentation Fondatrice v1.5 (Section 8)
- BondingBrother - Documentation Fondatrice v1.4
- Miyukini Conceptual References - Lois Autonomie Systeme v1.1 (LOI-6)
- Miyukini Conceptual References - Security Protocols v1.0

---

## 18. Mini log de génération

### Décision éditoriale E1 : Relation définition/application

**Décision prise :** La relation est de définition/application : Border Guard définit, BondingBrother applique. Cette direction respecte la Documentation Fondatrice de Border Guard Section 3.3 qui définit "Border Guard définit les règles de franchissement des frontières, BondingBrother applique ces règles".

**Application :** Tout le document est structuré autour de cette séparation absolue.

### Décision éditoriale E2 : Rôle dans la fédération

**Décision prise :** L'intégration joue un rôle critique pour LOI-6 (fédération). Border Guard définit les règles de fédération, BondingBrother les applique.

**Application :** Section 4.3 et Section 15 détaillent ce rôle critique.

### Warning W1 : Risque de confusion définition/exécution

**Warning rencontré :** Risque que Border Guard soit tenté d'exécuter les règles qu'il définit.

**Décision prise :** Les interdictions absolues (Section 5) clarifient que Border Guard ne filtre, ne bloque, n'intercepte jamais.

**Correction effectuée :** Section 5 explicite les interdictions, INV-BG-BB-2 confirme l'impossibilité de filtrage.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec Border Guard - Documentation Fondatrice : Confirmée (définition sans application)
- ✅ Cohérence avec BondingBrother - Documentation Fondatrice : Confirmée (application sans définition)
- ✅ Conformité LOI-1 : Confirmée (aucune dépendance externe)
- ✅ Conformité LOI-2 : Confirmée (fonctionnement en mode isolé)
- ✅ Conformité LOI-6 : Confirmée (fédération explicite, contrôlée, réversible)
- ✅ Séparation absolue : Confirmée (INV-BG-BB-1, INV-BG-BB-2, INV-BG-BB-3)
- ✅ Traçabilité complète : Confirmée (INV-BG-BB-7)

**Conclusion :** Aucune contradiction détectée. Le document est cohérent et non ambigu.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
