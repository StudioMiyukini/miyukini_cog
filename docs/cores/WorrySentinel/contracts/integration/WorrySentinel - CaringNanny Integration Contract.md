# WorrySentinel - CaringNanny Integration Contract

## 1. Contexte

Ce document définit le **contrat d'intégration entre WorrySentinel et CaringNanny**. Il spécifie l'interface, le protocole, les règles de communication, et les garanties associées à l'intégration entre le gouvernant de la sécurité (WorrySentinel) et l'observateur d'état du système (CaringNanny).

Ce document complète la Section 9 "Relation avec CaringNanny" de la [Documentation Fondatrice WorrySentinel](../../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [CaringNanny - Documentation Fondatrice](../../../CaringNanny/foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) pour la nature de CaringNanny
- [Miyukini Conceptual References - Integrity Degradation System](../../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md) pour les états de confiance T0-T4
- [Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) pour les niveaux de sécurité 0-4
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) pour la conformité LOI-1 à LOI-6

L'intégration respecte les Lois d'Autonomie Système : toutes les observations et règles de gouvernance sont locales et ne requièrent aucune dépendance externe (**LOI-1**).

## 2. Portée / Scope

Ce document couvre :
- L'interface contractuelle entre WorrySentinel et CaringNanny
- Le protocole de communication bidirectionnel asymétrique
- Les types d'informations échangées (signaux d'observation et règles de gouvernance)
- L'alimentation des états de confiance par les signaux d'observation
- Les règles de consolidation des signaux
- Les garanties de l'intégration

Ce document **ne couvre pas** :
- Les détails internes de CaringNanny (voir documentation CaringNanny)
- Les détails internes de WorrySentinel (voir Architecture WorrySentinel)
- L'intégration avec StrongFather (voir StrongFather Integration Contract)
- L'intégration avec BorderGuard (voir BorderGuard Integration Contract)
- L'intégration avec LogisticsSteward (voir LogisticsSteward Integration Contract)

---

## 3. Principe fondamental

**CaringNanny consolide les signaux d'intégrité qui alimentent la gouvernance des états de confiance. WorrySentinel gouverne les règles selon lesquelles CaringNanny doit consolider ces signaux. CaringNanny observe et rapporte, WorrySentinel gouverne et décide des transitions d'état.**

La relation est **bidirectionnelle asymétrique** :
- **Flux montant** : CaringNanny fournit des signaux d'observation à WorrySentinel
- **Flux descendant** : WorrySentinel impose des règles de consolidation à CaringNanny

---

## 4. Nature de la relation WorrySentinel — CaringNanny

### 4.1 Relation de collaboration asymétrique

**CaringNanny contribue à WorrySentinel par :**
- La consolidation des signaux d'intégrité de tous les composants
- La détection des anomalies et leur classification
- L'agrégation des états partiels en vision globale
- La proposition de transitions d'état basées sur les observations

**WorrySentinel gouverne CaringNanny par :**
- Les règles de classification des signaux
- Les seuils de transition entre états de confiance
- Les priorités de consolidation selon les niveaux de sécurité
- L'activation de modes d'observation renforcée

**Règle WS-CN-01 : Observation sans décision**

CaringNanny observe, consolide et rapporte, mais ne décide jamais des transitions d'état de confiance. La décision de transition appartient exclusivement à WorrySentinel.

**Règle WS-CN-02 : Gouvernance sans observation**

WorrySentinel gouverne les règles de consolidation, mais n'observe jamais directement les composants. L'observation appartient exclusivement à CaringNanny.

**Règle WS-CN-03 : Séparation stricte**

CaringNanny ne modifie jamais un état de confiance. WorrySentinel ne collecte jamais de signal directement.

### 4.2 Séparation des responsabilités

| Responsabilité | WorrySentinel | CaringNanny |
|----------------|---------------|-------------|
| **Observer les composants** | ❌ Jamais | ✅ Exclusif |
| **Détecter les anomalies** | ❌ Jamais | ✅ Exclusif |
| **Consolider les signaux** | ❌ Jamais | ✅ Exclusif |
| **Agréger en vision globale** | ❌ Consomme | ✅ Exclusif |
| **Définir les règles de classification** | ✅ Exclusif | ❌ Applique |
| **Définir les seuils de transition** | ✅ Exclusif | ❌ Utilise |
| **Décider des transitions T0-T4** | ✅ Exclusif | ❌ Propose |
| **Gouverner les états de confiance** | ✅ Exclusif | ❌ Jamais |
| **Modifier les états de confiance** | ✅ Exclusif | ❌ Jamais |

**Règle WS-CN-04 : Aucun chevauchement**

Aucun chevauchement de responsabilités n'est autorisé. CaringNanny ne gouverne jamais les états de confiance, WorrySentinel n'observe jamais directement les composants.

### 4.3 Cycle de gouvernance des états de confiance

Le cycle de gouvernance des états de confiance implique les deux cores :

```
┌─────────────────────────────────────────────────────────────────────┐
│                   CYCLE DE GOUVERNANCE T0-T4                         │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│   1. OBSERVATION (CaringNanny)                                       │
│      │                                                               │
│      ├── Détecte les conditions des composants                       │
│      ├── Classifie selon les règles de WorrySentinel                │
│      └── Agrège en signaux consolidés                                │
│                                                                      │
│   2. RAPPORTAGE (CaringNanny → WorrySentinel)                       │
│      │                                                               │
│      ├── Transmet les signaux consolidés                             │
│      ├── Propose des transitions si seuils atteints                  │
│      └── Fournit le contexte et la justification                     │
│                                                                      │
│   3. GOUVERNANCE (WorrySentinel)                                     │
│      │                                                               │
│      ├── Évalue les signaux selon les règles                         │
│      ├── Décide de la transition (ou non)                            │
│      └── Déclare le nouvel état de confiance                         │
│                                                                      │
│   4. PROPAGATION (WorrySentinel → tous les cores)                   │
│      │                                                               │
│      └── Notifie tous les cores du nouvel état                       │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 5. Ce que WorrySentinel ne fait JAMAIS vis-à-vis de CaringNanny

### 5.1 Interdictions absolues

**INV-WS-CN-NEVER-1 : N'observe jamais directement**

WorrySentinel n'observe **jamais** directement les composants du système. L'observation est la responsabilité exclusive de CaringNanny.

**INV-WS-CN-NEVER-2 : Ne collecte jamais de signaux**

WorrySentinel ne collecte **jamais** de signaux directement des composants. Tous les signaux transitent par CaringNanny.

**INV-WS-CN-NEVER-3 : Ne modifie jamais l'état d'un composant**

WorrySentinel ne modifie **jamais** l'état d'un composant. La modification d'état est hors-scope des deux cores (pas d'action corrective).

**INV-WS-CN-NEVER-4 : N'agrège jamais les états partiels**

WorrySentinel n'agrège **jamais** les états partiels en vision globale. L'agrégation est la responsabilité exclusive de CaringNanny.

**INV-WS-CN-NEVER-5 : N'interfère jamais avec l'observation**

WorrySentinel n'interfère **jamais** avec le processus d'observation de CaringNanny. Les règles de gouvernance guident, elles n'imposent pas de méthode d'observation.

---

## 6. Ce que CaringNanny ne fait JAMAIS vis-à-vis de WorrySentinel

### 6.1 Interdictions absolues

**INV-CN-WS-NEVER-1 : Ne gouverne jamais les états de confiance**

CaringNanny ne gouverne **jamais** les états de confiance (T0-T4). La gouvernance est la responsabilité exclusive de WorrySentinel.

**INV-CN-WS-NEVER-2 : Ne décide jamais des transitions**

CaringNanny ne décide **jamais** des transitions entre états de confiance. Elle propose des transitions basées sur les seuils, mais c'est WorrySentinel qui décide.

**INV-CN-WS-NEVER-3 : Ne modifie jamais un état de confiance**

CaringNanny ne modifie **jamais** un état de confiance. Seul WorrySentinel peut déclarer un changement d'état.

**INV-CN-WS-NEVER-4 : Ne définit jamais les règles de classification**

CaringNanny ne définit **jamais** les règles de classification des signaux. Les règles sont définies par WorrySentinel (ou l'écosystème), CaringNanny les applique.

**INV-CN-WS-NEVER-5 : N'ignore jamais les règles de WorrySentinel**

CaringNanny n'ignore **jamais** les règles de consolidation imposées par WorrySentinel. L'application des règles est obligatoire.

---

## 7. Types d'informations échangées

### 7.1 Flux montant : CaringNanny → WorrySentinel

**INTEGRITY_SIGNAL**
- **Objectif :** Transmettre un signal d'intégrité consolidé
- **Contenu :** Source, nature du signal, classification, sévérité
- **Déclencheur :** Détection d'une condition significative

**Structure du signal d'intégrité :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `signal_id` | Identifiant unique du signal | ✅ Oui |
| `source` | Composant source (core, module, produit) | ✅ Oui |
| `signal_type` | Type de signal (anomaly, degradation, recovery, nominal) | ✅ Oui |
| `classification` | Classification selon règles WorrySentinel | ✅ Oui |
| `severity` | Sévérité (info, warning, critical, emergency) | ✅ Oui |
| `context` | Contexte de l'observation | ✅ Oui |
| `timestamp` | Horodatage de l'observation | ✅ Oui |

**TRANSITION_PROPOSAL**
- **Objectif :** Proposer une transition d'état de confiance
- **Contenu :** État actuel, état proposé, signaux justificatifs
- **Déclencheur :** Seuils de transition atteints selon les règles

**Structure de la proposition de transition :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `proposal_id` | Identifiant unique de la proposition | ✅ Oui |
| `current_state` | État de confiance actuel (T0-T4) | ✅ Oui |
| `proposed_state` | État de confiance proposé (T0-T4) | ✅ Oui |
| `justifying_signals` | Liste des signaux justifiant la proposition | ✅ Oui |
| `threshold_met` | Seuil atteint selon les règles | ✅ Oui |
| `confidence_score` | Score de confiance de la proposition | ✅ Oui |
| `timestamp` | Horodatage de la proposition | ✅ Oui |

**CONSOLIDATED_STATE**
- **Objectif :** Fournir une vision consolidée de l'état global
- **Contenu :** États partiels agrégés, tendances, indicateurs
- **Déclencheur :** Demande de WorrySentinel ou périodique

**Structure de l'état consolidé :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `consolidation_id` | Identifiant unique de la consolidation | ✅ Oui |
| `observation_window` | Fenêtre d'observation | ✅ Oui |
| `component_states` | États partiels par composant | ✅ Oui |
| `anomaly_count` | Nombre d'anomalies détectées | ✅ Oui |
| `trend` | Tendance générale (improving, stable, degrading) | ✅ Oui |
| `timestamp` | Horodatage de la consolidation | ✅ Oui |

### 7.2 Flux descendant : WorrySentinel → CaringNanny

**CLASSIFICATION_RULES**
- **Objectif :** Définir ou mettre à jour les règles de classification
- **Contenu :** Règles de classification des signaux par type et sévérité
- **Déclencheur :** Initialisation ou mise à jour des règles

**Structure des règles de classification :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `rules_id` | Identifiant unique du jeu de règles | ✅ Oui |
| `rules_version` | Version des règles | ✅ Oui |
| `signal_classifications` | Mapping signal → classification | ✅ Oui |
| `severity_weights` | Pondérations par sévérité | ✅ Oui |
| `effective_from` | Date d'effet des règles | ✅ Oui |

**TRANSITION_THRESHOLDS**
- **Objectif :** Définir les seuils de transition T0-T4
- **Contenu :** Seuils pour chaque transition autorisée
- **Déclencheur :** Initialisation ou mise à jour des seuils

**Structure des seuils de transition :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `thresholds_id` | Identifiant unique des seuils | ✅ Oui |
| `thresholds_version` | Version des seuils | ✅ Oui |
| `transitions` | Liste des transitions avec leurs seuils | ✅ Oui |
| `effective_from` | Date d'effet des seuils | ✅ Oui |

**OBSERVATION_MODE**
- **Objectif :** Activer un mode d'observation spécifique
- **Contenu :** Mode (normal, enhanced, emergency), durée
- **Déclencheur :** Changement d'état de confiance ou alerte

**Structure du mode d'observation :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `mode_id` | Identifiant unique du mode | ✅ Oui |
| `mode_type` | Type (normal, enhanced, emergency) | ✅ Oui |
| `focus_areas` | Composants à surveiller en priorité | ❌ Optionnel |
| `sampling_rate` | Fréquence d'observation | ❌ Optionnel |
| `duration` | Durée du mode (null = jusqu'à nouvel ordre) | ❌ Optionnel |
| `timestamp` | Horodatage de l'activation | ✅ Oui |

**STATE_DECLARATION**
- **Objectif :** Notifier le nouvel état de confiance décidé
- **Contenu :** État précédent, nouvel état, justification
- **Déclencheur :** Décision de transition par WorrySentinel

**Structure de la déclaration d'état :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `declaration_id` | Identifiant unique de la déclaration | ✅ Oui |
| `previous_state` | État de confiance précédent (T0-T4) | ✅ Oui |
| `new_state` | Nouvel état de confiance (T0-T4) | ✅ Oui |
| `justification` | Justification de la transition | ✅ Oui |
| `accepted_proposal` | Référence à la proposition acceptée (si applicable) | ❌ Optionnel |
| `timestamp` | Horodatage de la déclaration | ✅ Oui |

---

## 8. Adaptation de l'observation par état de confiance

### 8.1 Mode d'observation par état

WorrySentinel active des modes d'observation différents selon l'état de confiance :

**T0 — Normal**

| Aspect | Comportement |
|--------|--------------|
| **Mode d'observation** | Normal |
| **Fréquence** | Standard |
| **Focus** | Tous les composants équitablement |
| **Seuils d'alerte** | Standards |
| **Journalisation** | Standard |

**T1 — Instable**

| Aspect | Comportement |
|--------|--------------|
| **Mode d'observation** | Enhanced |
| **Fréquence** | Augmentée (+50%) |
| **Focus** | Composants sources d'anomalies |
| **Seuils d'alerte** | Abaissés (-20%) |
| **Journalisation** | Détaillée |

**T2 — Dégradé**

| Aspect | Comportement |
|--------|--------------|
| **Mode d'observation** | Enhanced |
| **Fréquence** | Haute (+100%) |
| **Focus** | Composants critiques et frontières |
| **Seuils d'alerte** | Abaissés (-40%) |
| **Journalisation** | Complète avec contexte |

**T3 — Restreint**

| Aspect | Comportement |
|--------|--------------|
| **Mode d'observation** | Emergency |
| **Fréquence** | Maximale |
| **Focus** | Composants vitaux uniquement |
| **Seuils d'alerte** | Minimaux |
| **Journalisation** | Exhaustive |

**T4 — Bloqué**

| Aspect | Comportement |
|--------|--------------|
| **Mode d'observation** | Emergency |
| **Fréquence** | Minimale (préservation ressources) |
| **Focus** | Signes de récupération |
| **Seuils d'alerte** | Uniquement récupération |
| **Journalisation** | Minimale (préservation) |

**Règle WS-CN-STATE-01 : Adaptation immédiate**

L'adaptation du mode d'observation à un changement d'état de confiance est immédiate. Aucun délai n'est autorisé.

**Règle WS-CN-STATE-02 : Préservation en T4**

En état T4, l'observation est minimale pour préserver les ressources du système en mode survie.

### 8.2 Priorités de consolidation par niveau de sécurité

WorrySentinel définit des priorités de consolidation selon les niveaux de sécurité :

| Niveau de sécurité | Priorité de consolidation |
|--------------------|---------------------------|
| **Niveau 0 - Public** | Basse — consolidation en arrière-plan |
| **Niveau 1 - Standard** | Normale — consolidation régulière |
| **Niveau 2 - Sensitive** | Haute — consolidation prioritaire |
| **Niveau 3 - Critical** | Très haute — consolidation immédiate |
| **Niveau 4 - Highest** | Maximale — consolidation en temps réel |

**Règle WS-CN-SEC-01 : Priorisation des signaux**

Les signaux provenant de composants de niveau de sécurité élevé sont prioritaires dans la consolidation.

**Règle WS-CN-SEC-02 : Cumul état-niveau**

En cas d'état de confiance dégradé ET de niveau de sécurité élevé, la priorité est maximale.

---

## 9. Protocole de communication

### 9.1 Format des signaux montants

Les signaux de CaringNanny suivent un format standardisé.

**Structure de base :**

| Élément | Description | Obligatoire |
|---------|-------------|-------------|
| `signal_id` | Identifiant unique du signal | ✅ Oui |
| `type` | Type de signal (INTEGRITY_SIGNAL, TRANSITION_PROPOSAL, CONSOLIDATED_STATE) | ✅ Oui |
| `priority` | Priorité (low, normal, high, critical) | ✅ Oui |
| `payload` | Données spécifiques au signal | ✅ Oui |
| `timestamp` | Horodatage du signal | ✅ Oui |

**Règle WS-CN-PROT-01 : Signaux non bloquants**

Les signaux montants sont non bloquants. CaringNanny continue son observation après l'envoi.

### 9.2 Format des directives descendantes

Les directives de WorrySentinel suivent un format standardisé.

**Structure de base :**

| Élément | Description | Obligatoire |
|---------|-------------|-------------|
| `directive_id` | Identifiant unique de la directive | ✅ Oui |
| `type` | Type de directive (CLASSIFICATION_RULES, TRANSITION_THRESHOLDS, OBSERVATION_MODE, STATE_DECLARATION) | ✅ Oui |
| `payload` | Données spécifiques à la directive | ✅ Oui |
| `requires_ack` | Si une confirmation est requise | ✅ Oui |
| `timestamp` | Horodatage de la directive | ✅ Oui |

**Règle WS-CN-PROT-02 : Traitement immédiat**

Toutes les directives de WorrySentinel sont traitées immédiatement par CaringNanny.

### 9.3 Acquittements

**Règle WS-CN-PROT-03 : Acquittement obligatoire**

CaringNanny acquitte toutes les directives avec `requires_ack: true`.

**Structure de l'acquittement :**

| Élément | Description | Obligatoire |
|---------|-------------|-------------|
| `ack_id` | Identifiant unique de l'acquittement | ✅ Oui |
| `directive_id` | Référence à la directive | ✅ Oui |
| `status` | Statut (ACK_OK, ACK_PARTIAL, ACK_ERROR) | ✅ Oui |
| `adaptation_applied` | Confirmation de l'adaptation | ✅ Oui |
| `timestamp` | Horodatage de l'acquittement | ✅ Oui |

---

## 10. Flux d'intégration typiques

### 10.1 Flux de proposition de transition

**Acteurs :** CaringNanny, WorrySentinel

**Séquence :**

1. CaringNanny observe des anomalies sur plusieurs composants
2. CaringNanny classifie les anomalies selon les règles de WorrySentinel
3. CaringNanny calcule que le seuil T0→T1 est atteint
4. CaringNanny envoie `TRANSITION_PROPOSAL` à WorrySentinel
5. WorrySentinel évalue la proposition
6. WorrySentinel décide d'accepter ou rejeter la transition
7. Si acceptée, WorrySentinel envoie `STATE_DECLARATION` (T0→T1)
8. WorrySentinel envoie `OBSERVATION_MODE` (enhanced) à CaringNanny
9. CaringNanny acquitte et adapte son mode d'observation

### 10.2 Flux de mise à jour des règles

**Acteurs :** WorrySentinel, CaringNanny

**Séquence :**

1. WorrySentinel décide de modifier les seuils de transition
2. WorrySentinel envoie `TRANSITION_THRESHOLDS` à CaringNanny
3. CaringNanny reçoit les nouveaux seuils
4. CaringNanny acquitte avec `ACK_OK`
5. CaringNanny applique les nouveaux seuils pour les futures propositions

### 10.3 Flux de détection d'anomalie critique

**Acteurs :** CaringNanny, WorrySentinel

**Séquence :**

1. CaringNanny détecte une anomalie critique sur un composant de sécurité niveau 4
2. CaringNanny envoie immédiatement `INTEGRITY_SIGNAL` (priority: critical)
3. WorrySentinel évalue le signal
4. WorrySentinel peut décider une transition d'état immédiate
5. WorrySentinel notifie CaringNanny et tous les cores de la transition

### 10.4 Diagramme de séquence

```
┌─────────────────┐                    ┌─────────────────┐
│  CaringNanny    │                    │  WorrySentinel  │
└────────┬────────┘                    └────────┬────────┘
         │                                      │
         │  (Observation d'anomalies)           │
         │                                      │
         ├── INTEGRITY_SIGNAL ────────────────► │
         │   (anomaly, warning)                 │
         │                                      │
         ├── INTEGRITY_SIGNAL ────────────────► │
         │   (anomaly, warning)                 │
         │                                      │
         │  (Seuil T0→T1 atteint)               │
         │                                      │
         ├── TRANSITION_PROPOSAL ─────────────► │
         │   (T0 → T1)                          │
         │                                      ├── Évalue proposition
         │                                      │
         │ ◄──────────── STATE_DECLARATION ─────┤
         │              (T0 → T1 accepté)       │
         │                                      │
         │ ◄──────────── OBSERVATION_MODE ──────┤
         │              (mode: enhanced)        │
         │                                      │
         ├── ACK_OK ───────────────────────────►│
         │                                      │
         │  (Observation renforcée)             │
         │                                      │
```

---

## 11. Règles d'intégration

### 11.1 Règles de communication

**Règle WS-CN-INT-01 : Bidirectionnel asymétrique**

La communication est bidirectionnelle mais asymétrique. CaringNanny rapporte et propose, WorrySentinel gouverne et décide.

**Règle WS-CN-INT-02 : Priorité aux directives**

Les directives de WorrySentinel sont prioritaires. CaringNanny adapte immédiatement son comportement.

**Règle WS-CN-INT-03 : Signaux continus**

CaringNanny fournit des signaux de manière continue, pas uniquement lors d'anomalies.

### 11.2 Règles de données

**Règle WS-CN-INT-04 : Signaux factuels**

Les signaux de CaringNanny sont factuels (observations, classifications). Aucune décision n'est incluse.

**Règle WS-CN-INT-05 : Règles explicites**

Les règles de WorrySentinel sont explicites et déclaratives. Aucune règle implicite.

**Règle WS-CN-INT-06 : Cohérence garantie**

WorrySentinel garantit la cohérence des règles et seuils communiqués.

### 11.3 Règles de traçabilité

**Règle WS-CN-INT-07 : Traçabilité complète**

Toutes les interactions sont tracées avec contexte complet.

**Règle WS-CN-INT-08 : Corrélation possible**

Chaque transition d'état peut être corrélée aux signaux et propositions qui l'ont provoquée.

---

## 12. Gestion des erreurs

### 12.1 Types d'erreurs

**Erreurs de format :**
- Signal mal formé
- Directive mal formée
- Type de message inconnu

**Erreurs de classification :**
- Signal inclassifiable selon les règles
- Règles incohérentes reçues

**Erreurs internes :**
- Erreur de CaringNanny lors de la consolidation
- Erreur de WorrySentinel lors de l'évaluation

### 12.2 Traitement des erreurs

**Règle WS-CN-ERR-01 : Acquittement avec erreur**

En cas d'erreur, CaringNanny acquitte avec `ACK_ERROR` et description du problème.

**Règle WS-CN-ERR-02 : Signal non classifiable**

Si un signal ne peut pas être classifié, CaringNanny le rapporte avec classification `unknown` et WorrySentinel décide de son traitement.

**Règle WS-CN-ERR-03 : Journalisation des erreurs**

Toutes les erreurs sont journalisées pour audit et diagnostic.

**Règle WS-CN-ERR-04 : Observation par défaut**

En cas de perte de communication avec WorrySentinel, CaringNanny continue avec les dernières règles connues et le mode d'observation le plus restrictif applicable.

---

## 13. Cas particuliers

### 13.1 Proposition de transition rejetée

Lorsque WorrySentinel rejette une proposition de transition :

**Règle WS-CN-CASE-01 : Rejet notifié**

WorrySentinel notifie CaringNanny du rejet avec justification. CaringNanny continue l'observation avec les paramètres actuels.

### 13.2 Transitions multiples rapides

Lorsque plusieurs seuils sont atteints rapidement :

**Règle WS-CN-CASE-02 : Proposition par transition**

CaringNanny propose les transitions une par une (T0→T1, puis T1→T2). WorrySentinel peut accepter plusieurs transitions consécutives.

### 13.3 État T4 (Bloqué)

En état T4 :

**Règle WS-CN-CASE-03 : Observation minimale**

CaringNanny réduit son observation au minimum pour préserver les ressources. Seuls les signaux de récupération sont recherchés.

### 13.4 Récupération (T2→T1, T1→T0)

Lors d'une récupération :

**Règle WS-CN-CASE-04 : Proposition de récupération**

CaringNanny peut proposer des transitions de récupération lorsque les conditions s'améliorent et que les seuils le permettent.

---

## 14. Garanties de l'intégration

### 14.1 Garantie de séparation

**Engagement :** CaringNanny observe exclusivement, WorrySentinel gouverne exclusivement. Aucun chevauchement de responsabilités.

### 14.2 Garantie de réactivité

**Engagement :** CaringNanny réagit immédiatement aux directives de WorrySentinel. Aucun délai supérieur à une seconde.

### 14.3 Garantie de conformité

**Engagement :** CaringNanny applique toujours les règles de WorrySentinel. Aucune classification ou proposition ne contredit les règles.

### 14.4 Garantie de traçabilité

**Engagement :** Toute interaction est traçable de bout en bout. L'audit complet du cycle de gouvernance est possible.

### 14.5 Garantie de continuité

**Engagement :** En cas de défaillance partielle, les deux cores continuent de fonctionner avec les dernières règles/observations connues.

### 14.6 Garantie de disponibilité

**Engagement :** L'intégration ne bloque jamais CaringNanny. En cas de défaillance de WorrySentinel, CaringNanny continue avec le mode d'observation le plus restrictif.

---

## 15. Invariants de l'intégration

### 15.1 Invariants de relation

**INV-WS-CN-1 : Observation exclusive**

L'observation appartient exclusivement à CaringNanny. WorrySentinel n'observe jamais directement.

**INV-WS-CN-2 : Gouvernance exclusive**

La gouvernance des états de confiance appartient exclusivement à WorrySentinel. CaringNanny ne gouverne jamais.

**INV-WS-CN-3 : Proposition vs décision**

CaringNanny propose des transitions. WorrySentinel décide des transitions. La distinction est fondamentale.

### 15.2 Invariants de données

**INV-WS-CN-4 : Signaux factuels**

Les signaux de CaringNanny sont factuels (observations classifiées). Aucune décision n'est incluse.

**INV-WS-CN-5 : Règles explicites**

Les règles de WorrySentinel sont explicites et déclaratives.

### 15.3 Invariants de protocole

**INV-WS-CN-6 : Format respecté**

Toutes les communications respectent le format standardisé.

**INV-WS-CN-7 : Traçabilité complète**

Toute interaction est traçable avec son contexte complet.

---

## 16. Conformité aux Lois d'Autonomie Système

### LOI-1 : Aucune dépendance externe critique

**Conformité :** ✅ **Conforme**

L'intégration respecte LOI-1 :
- CaringNanny observe localement
- WorrySentinel gouverne localement
- Les règles et signaux sont stockés localement
- L'absence de connexion ne bloque ni l'observation ni la gouvernance

### LOI-2 : Le système accepte l'isolement comme état normal

**Conformité :** ✅ **Conforme**

L'intégration respecte LOI-2 :
- En isolement, CaringNanny continue avec les dernières règles connues
- Les propositions de transition restent possibles localement
- L'intégration fonctionne sans dégradation en mode isolé

### LOI-4 : Pas de temps global requis

**Conformité :** ✅ **Conforme**

L'intégration respecte LOI-4 :
- Les horodatages sont locaux
- Aucune synchronisation temporelle n'est requise
- Les états de confiance ne dépendent pas de timestamps synchronisés

---

## 17. Exemples

### 17.1 Signal d'intégrité

**Signal CaringNanny → WorrySentinel :**
```
{
  "signal_id": "sig-cn-ws-001",
  "type": "INTEGRITY_SIGNAL",
  "priority": "high",
  "payload": {
    "source": "BorderGuard",
    "signal_type": "anomaly",
    "classification": "boundary_degraded",
    "severity": "warning",
    "context": {
      "boundary_id": "boundary-external-001",
      "previous_state": "healthy",
      "current_state": "degraded",
      "cause": "latency_exceeded"
    }
  },
  "timestamp": "2026-01-28T14:00:00Z"
}
```

### 17.2 Proposition de transition

**Proposition CaringNanny → WorrySentinel :**
```
{
  "signal_id": "sig-cn-ws-002",
  "type": "TRANSITION_PROPOSAL",
  "priority": "high",
  "payload": {
    "proposal_id": "prop-001",
    "current_state": "T0",
    "proposed_state": "T1",
    "justifying_signals": ["sig-cn-ws-001", "sig-cn-ws-000"],
    "threshold_met": "anomaly_count >= 3",
    "confidence_score": 0.85
  },
  "timestamp": "2026-01-28T14:05:00Z"
}
```

### 17.3 Déclaration d'état

**Déclaration WorrySentinel → CaringNanny :**
```
{
  "directive_id": "dir-ws-cn-001",
  "type": "STATE_DECLARATION",
  "payload": {
    "declaration_id": "decl-001",
    "previous_state": "T0",
    "new_state": "T1",
    "justification": "Anomalies persistantes détectées sur frontières et composants",
    "accepted_proposal": "prop-001"
  },
  "requires_ack": true,
  "timestamp": "2026-01-28T14:06:00Z"
}
```

### 17.4 Mode d'observation

**Directive WorrySentinel → CaringNanny :**
```
{
  "directive_id": "dir-ws-cn-002",
  "type": "OBSERVATION_MODE",
  "payload": {
    "mode_id": "mode-001",
    "mode_type": "enhanced",
    "focus_areas": ["BorderGuard", "KindMother"],
    "sampling_rate": "high",
    "duration": null
  },
  "requires_ack": true,
  "timestamp": "2026-01-28T14:06:01Z"
}
```

### 17.5 Acquittement

**Acquittement CaringNanny :**
```
{
  "ack_id": "ack-cn-001",
  "directive_id": "dir-ws-cn-002",
  "status": "ACK_OK",
  "adaptation_applied": {
    "mode": "enhanced",
    "focus_areas_activated": ["BorderGuard", "KindMother"],
    "sampling_rate_applied": "high"
  },
  "timestamp": "2026-01-28T14:06:02Z"
}
```

---

## 18. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit l'interface et le protocole que WorrySentinel et CaringNanny doivent respecter pour leur intégration.

Toute implémentation de l'intégration doit respecter ce contrat. Toute violation entraîne un comportement non conforme.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dépendances :**
- WorrySentinel - Documentation Fondatrice v1.2 (Section 9)
- CaringNanny - Documentation Fondatrice v1.6
- Miyukini Conceptual References - Integrity Degradation System v1.0 (T0-T4)
- Miyukini Conceptual References - Security Levels v1.0 (0-4)
- Miyukini Conceptual References - Lois Autonomie Systeme v1.1

---

## 19. Mini log de génération

### Décision éditoriale E1 : Nature de la relation

**Décision prise :** La relation est bidirectionnelle asymétrique : CaringNanny observe et propose, WorrySentinel gouverne et décide. Cette direction respecte la Documentation Fondatrice de WorrySentinel Section 9 qui définit "CaringNanny consolide les signaux d'intégrité qui influencent les états de confiance" et "WorrySentinel gouverne les règles selon lesquelles CaringNanny doit consolider les signaux".

**Application :** Tout le document est structuré autour de cette relation de collaboration asymétrique.

### Décision éditoriale E2 : Cycle de gouvernance

**Décision prise :** Le cycle de gouvernance T0-T4 implique les deux cores de manière complémentaire : observation (CN) → rapportage (CN→WS) → gouvernance (WS) → propagation (WS→tous).

**Application :** Section 4.3 détaille ce cycle, Section 10 illustre les flux typiques.

### Décision éditoriale E3 : Proposition vs décision

**Décision prise :** La distinction entre proposition (CaringNanny) et décision (WorrySentinel) est fondamentale. CaringNanny peut calculer que les seuils sont atteints et proposer une transition, mais seul WorrySentinel décide de l'accepter.

**Application :** INV-WS-CN-3 et Règles WS-CN-01/02 établissent cette distinction.

### Warning W1 : Risque de gouvernance implicite par CaringNanny

**Warning rencontré :** Risque que CaringNanny, en proposant des transitions basées sur des seuils, exerce une forme de gouvernance implicite.

**Décision prise :** Les interdictions absolues (Section 6) clarifient que CaringNanny ne décide jamais des transitions. Les seuils sont définis par WorrySentinel, CaringNanny les applique pour proposer, mais la décision reste à WorrySentinel qui peut rejeter la proposition.

**Correction effectuée :** INV-CN-WS-NEVER-2 et Règle WS-CN-CASE-01 explicitement établissent cette limite.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec WorrySentinel - Documentation Fondatrice : Confirmée (Section 9 respectée)
- ✅ Cohérence avec CaringNanny - Documentation Fondatrice : Confirmée (INV-CN-1 à INV-CN-7 respectés)
- ✅ Cohérence avec Integrity Degradation System : Confirmée (états T0-T4)
- ✅ Cohérence avec Security Levels : Confirmée (niveaux 0-4)
- ✅ Conformité LOI-1 : Confirmée (aucune dépendance externe)
- ✅ Conformité LOI-2 : Confirmée (fonctionnement en mode isolé)
- ✅ Conformité LOI-4 : Confirmée (pas de temps global requis)
- ✅ Séparation observation/gouvernance : Confirmée (INV-WS-CN-1, INV-WS-CN-2)
- ✅ Traçabilité complète : Confirmée (INV-WS-CN-7)

**Conclusion :** Aucune contradiction détectée. Le document est cohérent et non ambigu.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
