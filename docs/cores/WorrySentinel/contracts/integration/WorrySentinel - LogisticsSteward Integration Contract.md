# WorrySentinel - LogisticsSteward Integration Contract

## 1. Contexte

Ce document définit le **contrat d'intégration entre WorrySentinel et LogisticsSteward**. Il spécifie l'interface, le protocole, les règles de communication, et les garanties associées à l'intégration avec LogisticsSteward en tant que core responsable de la gouvernance de l'allocation, de la priorisation et de la limitation des ressources.

Ce document complète la Section 9 "Relation avec LogisticsSteward" de la [Documentation Fondatrice](../../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [LogisticsSteward - Documentation Fondatrice](../../../LogisticsSteward/foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md) pour la nature de LogisticsSteward
- [LogisticsSteward - WorrySentinel Integration Contract](../../../LogisticsSteward/contracts/integration/LogisticsSteward%20-%20WorrySentinel%20Integration%20Contract.md) pour le contrat symétrique
- [Miyukini Conceptual References - Lois Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) pour la conformité LOI-1 à LOI-6

L'intégration respecte les Lois d'Autonomie Système : toutes les contraintes de sécurité sont locales et ne requièrent aucune dépendance externe (**LOI-1**).

## 2. Portée / Scope

Ce document couvre :
- L'interface contractuelle entre WorrySentinel et LogisticsSteward
- Le protocole de communication (contraintes descendantes et observations montantes)
- Les types d'informations échangées
- La supervision des dérives d'allocation
- Le déclenchement de durcissement des règles d'arbitrage
- Les règles d'intégration spécifiques
- Les garanties de l'intégration

Ce document **ne couvre pas** :
- Les détails internes de LogisticsSteward (voir documentation LogisticsSteward)
- Les détails internes du moteur de gouvernance de WorrySentinel
- L'intégration avec StrongFather (voir StrongFather Integration Contract)
- L'intégration avec CaringNanny (voir CaringNanny Integration Contract)
- L'intégration avec BorderGuard (voir BorderGuard Integration Contract)
- L'intégration avec TAMR (voir TAMR Integration Contract)
- L'intégration avec MiyukiniAdmin (voir MiyukiniAdmin Integration Contract)

---

## 3. Principe fondamental

**WorrySentinel gouverne les niveaux de sécurité et les états de confiance. LogisticsSteward adapte ses règles d'arbitrage en conséquence. WorrySentinel supervise les dérives d'allocation sans jamais décider des allocations. LogisticsSteward ne peut jamais définir des niveaux de sécurité ni des états de confiance.**

La relation est de **supervision verticale** : WorrySentinel observe les comportements d'arbitrage, impose des contraintes de sécurité, et peut déclencher des durcissements. LogisticsSteward reste souverain sur l'arbitrage des ressources mais doit adapter ses décisions selon les contraintes sécuritaires.

---

## 4. Nature de la relation WorrySentinel — LogisticsSteward

### 4.1 Relation de supervision verticale

**WorrySentinel supervise LogisticsSteward par :**
- L'observation des signaux d'allocation et des dérives potentielles
- L'imposition de contraintes sécuritaires selon l'état de confiance (T0-T4)
- Le déclenchement de durcissement des règles d'arbitrage
- L'invalidation d'état système jugé incohérent

**LogisticsSteward informe WorrySentinel par :**
- Les signaux d'allocation et de consommation
- Les alertes de dérive de ressources
- Les comportements suspects d'arbitrage
- Les anomalies de gouvernance détectées

**Règle WS-LS-01 : Supervision sans substitution**

WorrySentinel supervise LogisticsSteward sans se substituer à lui. LogisticsSteward reste souverain sur l'arbitrage des ressources. WorrySentinel ne décide jamais de l'allocation, de la priorité, ou de la limitation des ressources.

**Règle WS-LS-02 : Contrainte verticale obligatoire**

LogisticsSteward doit adapter ses règles d'arbitrage selon les états de confiance et les niveaux de sécurité gouvernés par WorrySentinel. L'adaptation n'est pas facultative.

**Règle WS-LS-03 : Observation continue**

WorrySentinel observe en continu les comportements d'arbitrage de LogisticsSteward pour détecter les dérives sécuritaires. L'observation est passive et non intrusive.

**Règle WS-LS-04 : Durcissement proportionnel**

Le durcissement des règles d'arbitrage est proportionnel à l'état de confiance. Un état T1 implique une vigilance accrue, un état T3 implique des restrictions sévères.

### 4.2 Séparation des responsabilités

| Responsabilité | WorrySentinel | LogisticsSteward |
|----------------|---------------|------------------|
| **Gouverner les états de confiance (T0-T4)** | ✅ Exclusif | ❌ Consomme |
| **Définir les niveaux de sécurité (0-4)** | ✅ Exclusif | ❌ Consomme |
| **Arbitrer l'allocation des ressources** | ❌ Jamais | ✅ Exclusif |
| **Définir les quotas et priorités** | ❌ Jamais | ✅ Exclusif |
| **Déclencher le durcissement** | ✅ Exclusif | ❌ Subit |
| **Invalider un état système** | ✅ Peut décider | ❌ Réagit |
| **Détecter les dérives de sécurité** | ✅ Consomme | ✅ Source |
| **Observer les comportements d'arbitrage** | ✅ Exclusif | ❌ Source |

**Règle WS-LS-05 : Aucun chevauchement**

Aucun chevauchement de responsabilités n'est autorisé. WorrySentinel ne décide jamais des allocations, LogisticsSteward n'évalue jamais les menaces de sécurité.

---

## 5. Ce que WorrySentinel ne fait JAMAIS vis-à-vis de LogisticsSteward

### 5.1 Interdictions absolues

**INV-WS-LS-NEVER-1 : Ne décide jamais de l'allocation**

WorrySentinel ne décide **jamais** de l'allocation des ressources. Il peut imposer des contraintes de sécurité, mais la décision d'allocation appartient exclusivement à LogisticsSteward.

**INV-WS-LS-NEVER-2 : Ne définit jamais les quotas**

WorrySentinel ne définit **jamais** les quotas ou les priorités. Il peut exiger des restrictions, mais c'est LogisticsSteward qui traduit ces exigences en règles d'arbitrage.

**INV-WS-LS-NEVER-3 : N'exécute jamais d'arbitrage**

WorrySentinel n'exécute **jamais** d'arbitrage de ressources. Il gouverne et contraint, mais ne participe pas à l'arbitrage.

**INV-WS-LS-NEVER-4 : Ne contourne jamais LogisticsSteward**

WorrySentinel ne contourne **jamais** LogisticsSteward pour imposer directement des allocations ou des restrictions de ressources aux entités.

**INV-WS-LS-NEVER-5 : Ne modifie jamais les règles d'arbitrage**

WorrySentinel ne modifie **jamais** directement les règles d'arbitrage de LogisticsSteward. Il impose des contraintes que LogisticsSteward traduit en règles.

**INV-WS-LS-NEVER-6 : Ne bloque jamais les signaux montants**

WorrySentinel ne bloque **jamais** les signaux montants de LogisticsSteward. Toute information de dérive doit pouvoir remonter.

---

## 6. Supervision des dérives d'allocation

### 6.1 Objectif de la supervision

WorrySentinel supervise LogisticsSteward pour détecter les dérives potentielles dans l'allocation des ressources qui pourraient compromettre la sécurité du système.

**Types de dérives surveillées :**

| Type de dérive | Description | Impact sécuritaire |
|----------------|-------------|-------------------|
| **Monopolisation** | Une entité accapare une part disproportionnée | Risque de déni de service |
| **Escalade progressive** | Augmentation graduelle de consommation | Épuisement silencieux |
| **Pattern anormal** | Comportement atypique d'allocation | Indicateur d'intrusion |
| **Contournement** | Tentatives de bypass des quotas | Violation de gouvernance |
| **Saturation ciblée** | Épuisement délibéré de ressources | Attaque par ressources |

### 6.2 Règles de détection

**Règle WS-LS-DET-01 : Observation des tendances**

WorrySentinel observe les tendances de consommation signalées par LogisticsSteward. Une tendance croissante persistante peut déclencher une alerte.

**Règle WS-LS-DET-02 : Corrélation multi-signaux**

WorrySentinel corrèle les signaux de LogisticsSteward avec les autres sources (BorderGuard, StrongFather, CaringNanny) pour identifier les patterns de menace.

**Règle WS-LS-DET-03 : Seuils d'alerte**

| Seuil | Niveau | Action |
|-------|--------|--------|
| **Usage > 70%** | Info | Surveillance accrue |
| **Usage > 85%** | Warning | Préparation durcissement |
| **Usage > 95%** | Critique | Durcissement immédiat possible |
| **Dépassement quota** | Alerte | Évaluation de la menace |

**Règle WS-LS-DET-04 : Contexte de sécurité**

La détection tient compte du niveau de sécurité de l'entité concernée. Une dérive sur une entité de niveau 4 est plus critique qu'une dérive sur une entité de niveau 0.

### 6.3 Corrélation avec l'état de confiance

| État de confiance | Sensibilité de détection | Seuils |
|-------------------|--------------------------|--------|
| **T0 — Normal** | Standard | Seuils normaux |
| **T1 — Instable** | Élevée | Seuils abaissés de 10% |
| **T2 — Dégradé** | Très élevée | Seuils abaissés de 20% |
| **T3 — Restreint** | Maximale | Seuils abaissés de 30% |
| **T4 — Bloqué** | Critique | Toute dérive est bloquante |

---

## 7. Durcissement des règles d'arbitrage

### 7.1 Principes de durcissement

WorrySentinel peut déclencher un durcissement des règles d'arbitrage de LogisticsSteward selon l'état de confiance ou en réponse à une menace détectée.

**Principe WS-LS-HARD-01 : Durcissement progressif**

Le durcissement est progressif et proportionnel à la menace. Pas de durcissement brutal sans justification.

**Principe WS-LS-HARD-02 : Durcissement réversible**

Tout durcissement est réversible par une directive explicite de levée. Le retour à la normale est possible.

**Principe WS-LS-HARD-03 : Durcissement ciblé**

Le durcissement peut être ciblé sur des entités spécifiques ou global. Le ciblage précis minimise l'impact.

### 7.2 Types de durcissement

| Type | Description | Déclencheur |
|------|-------------|-------------|
| **QUOTA_REDUCTION** | Réduction des quotas autorisés | Dérive de consommation |
| **PRIORITY_FREEZE** | Gel des priorités au niveau actuel | Tentatives d'escalade |
| **ALLOCATION_BLOCK** | Blocage des nouvelles allocations | Menace confirmée |
| **PREEMPTION_ENABLE** | Activation de la préemption | Urgence ressources |
| **DEGRADATION_FORCE** | Forçage d'un niveau de dégradation | État T2+ |

### 7.3 Directives de durcissement par état de confiance

**T0 — Normal**

| Aspect | Directive |
|--------|-----------|
| **Quotas** | Aucune modification |
| **Priorités** | Aucune modification |
| **Allocations** | Normales |
| **Durcissement** | Aucun |

**T1 — Instable**

| Aspect | Directive |
|--------|-----------|
| **Quotas** | Surveillance renforcée, alertes actives |
| **Priorités** | Aucune modification |
| **Allocations** | Normales avec traçabilité étendue |
| **Durcissement** | Préparation possible |

**T2 — Dégradé**

| Aspect | Directive |
|--------|-----------|
| **Quotas** | Réduction de 20% pour entités non essentielles |
| **Priorités** | Priorité maximale réservée aux services critiques |
| **Allocations** | Nouvelles allocations sous conditions |
| **Durcissement** | Actif, niveau modéré |

**T3 — Restreint**

| Aspect | Directive |
|--------|-----------|
| **Quotas** | Quotas minimaux, gel des nouvelles allocations |
| **Priorités** | Seules priorités critiques honorées |
| **Allocations** | Bloquées sauf services vitaux |
| **Durcissement** | Actif, niveau sévère |

**T4 — Bloqué**

| Aspect | Directive |
|--------|-----------|
| **Quotas** | Aucune allocation |
| **Priorités** | Préservation du cœur système uniquement |
| **Allocations** | Totalement bloquées |
| **Durcissement** | Maximum, mode survie |

### 7.4 Règles de durcissement (RÈGLE-WS-LS-1 à RÈGLE-WS-LS-4)

Ces règles sont définies dans la [Documentation Fondatrice](../../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md) Section 9 :

**RÈGLE-WS-LS-1 : Contraintes sécuritaires**

WorrySentinel peut imposer des contraintes sécuritaires sur les décisions d'arbitrage de LogisticsSteward. Ces contraintes sont obligatoires.

**RÈGLE-WS-LS-2 : Quotas restrictifs en état T2+**

En état T2+, LogisticsSteward doit appliquer des quotas plus restrictifs selon les directives de WorrySentinel.

**RÈGLE-WS-LS-3 : Observation des patterns**

WorrySentinel observe les patterns d'allocation de ressources pour détecter des anomalies sécuritaires.

**RÈGLE-WS-LS-4 : Traitement des dérives**

Toute dérive d'allocation signalée par WorrySentinel doit être traitée par LogisticsSteward.

---

## 8. Types d'informations échangées

### 8.1 Flux descendant : WorrySentinel → LogisticsSteward

**TRUST_STATE_CHANGE**
- **Objectif :** Notifier un changement d'état de confiance
- **Contenu :** Nouvel état (T0-T4), justification, timestamp
- **Impact :** LogisticsSteward adapte ses règles d'arbitrage

**Structure du changement d'état :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `notification_id` | Identifiant unique de la notification | ✅ Oui |
| `previous_state` | État de confiance précédent (T0-T4) | ✅ Oui |
| `new_state` | Nouvel état de confiance (T0-T4) | ✅ Oui |
| `transition_reason` | Justification de la transition | ✅ Oui |
| `timestamp` | Horodatage de la transition | ✅ Oui |
| `constraints` | Contraintes supplémentaires applicables | ❌ Optionnel |

**SECURITY_LEVEL_ASSIGNMENT**
- **Objectif :** Attribuer ou modifier le niveau de sécurité d'une entité
- **Contenu :** Entité concernée, niveau (0-4), justification
- **Impact :** LogisticsSteward adapte l'arbitrage pour cette entité

**Structure de l'attribution de niveau :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `assignment_id` | Identifiant unique de l'attribution | ✅ Oui |
| `entity_id` | Identifiant de l'entité concernée | ✅ Oui |
| `entity_type` | Type d'entité (operator, team, service, tool) | ✅ Oui |
| `security_level` | Niveau de sécurité (0-4) | ✅ Oui |
| `justification` | Raison de l'attribution | ✅ Oui |
| `timestamp` | Horodatage de l'attribution | ✅ Oui |

**HARDENING_DIRECTIVE**
- **Objectif :** Déclencher un durcissement immédiat des règles
- **Contenu :** Type de durcissement, entités concernées, durée
- **Impact :** Restrictions supplémentaires appliquées immédiatement

**Structure de la directive de durcissement :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `directive_id` | Identifiant unique de la directive | ✅ Oui |
| `hardening_type` | Type de durcissement (quota_reduction, priority_freeze, allocation_block) | ✅ Oui |
| `affected_entities` | Liste des entités concernées (vide = toutes) | ❌ Optionnel |
| `severity` | Sévérité du durcissement (low, medium, high, critical) | ✅ Oui |
| `duration` | Durée du durcissement (null = indéfini) | ❌ Optionnel |
| `justification` | Raison du durcissement | ✅ Oui |
| `timestamp` | Horodatage de la directive | ✅ Oui |

**HARDENING_LIFT**
- **Objectif :** Lever un durcissement précédemment imposé
- **Contenu :** Référence à la directive originale, justification
- **Impact :** Retour aux règles d'arbitrage normales

**Structure de la levée de durcissement :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `lift_id` | Identifiant unique de la levée | ✅ Oui |
| `directive_id` | Référence à la directive originale | ✅ Oui |
| `justification` | Raison de la levée | ✅ Oui |
| `timestamp` | Horodatage de la levée | ✅ Oui |

**STATE_INVALIDATION**
- **Objectif :** Invalider l'état système actuel
- **Contenu :** Raison de l'invalidation, action requise
- **Impact :** LogisticsSteward doit suspendre les nouveaux arbitrages

**Structure de l'invalidation :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `invalidation_id` | Identifiant unique de l'invalidation | ✅ Oui |
| `reason` | Raison de l'invalidation | ✅ Oui |
| `required_action` | Action requise de LogisticsSteward | ✅ Oui |
| `timestamp` | Horodatage de l'invalidation | ✅ Oui |

### 8.2 Flux montant : LogisticsSteward → WorrySentinel

**ANOMALY_REPORT**
- **Objectif :** Signaler un comportement d'arbitrage suspect
- **Contenu :** Nature de l'anomalie, entité concernée, contexte
- **Usage :** WorrySentinel évalue si une action de sécurité est nécessaire

**DRIFT_ALERT**
- **Objectif :** Alerter sur une dérive de consommation
- **Contenu :** Entité, ressource, tendance, projection
- **Usage :** WorrySentinel peut anticiper une menace

**GOVERNANCE_ISSUE**
- **Objectif :** Signaler une anomalie de gouvernance
- **Contenu :** Nature du problème, impact, recommandations
- **Usage :** WorrySentinel peut décider d'une action de sécurité

**ALLOCATION_PATTERN**
- **Objectif :** Signaler un pattern d'allocation atypique
- **Contenu :** Description du pattern, entités impliquées, fréquence
- **Usage :** WorrySentinel corrèle avec d'autres signaux

---

## 9. Protocole de communication

### 9.1 Format des notifications descendantes

Les notifications de WorrySentinel suivent un format standardisé.

**Structure de base :**

| Élément | Description | Obligatoire |
|---------|-------------|-------------|
| `notification_id` | Identifiant unique de la notification | ✅ Oui |
| `type` | Type de notification | ✅ Oui |
| `payload` | Données spécifiques à la notification | ✅ Oui |
| `timestamp` | Horodatage de la notification | ✅ Oui |
| `requires_ack` | Si une confirmation est requise | ✅ Oui |

**Règle WS-LS-PROT-01 : Notification obligatoire**

Toutes les notifications de WorrySentinel doivent être transmises à LogisticsSteward sans filtrage ni délai.

### 9.2 Format des signalements montants

Les signalements de LogisticsSteward suivent un format standardisé.

**Structure de base :**

| Élément | Description | Obligatoire |
|---------|-------------|-------------|
| `signal_id` | Identifiant unique du signalement | ✅ Oui |
| `type` | Type de signalement | ✅ Oui |
| `payload` | Données spécifiques au signalement | ✅ Oui |
| `timestamp` | Horodatage du signalement | ✅ Oui |
| `urgency` | Niveau d'urgence (low, medium, high, critical) | ✅ Oui |

**Règle WS-LS-PROT-02 : Réception obligatoire**

WorrySentinel doit recevoir tous les signalements de LogisticsSteward sans exception. Aucun filtrage n'est autorisé.

### 9.3 Confirmations et acquittements

**Règle WS-LS-PROT-03 : Acquittement par LogisticsSteward**

LogisticsSteward acquitte toutes les notifications descendantes avec `requires_ack: true`.

**Règle WS-LS-PROT-04 : Pas d'acquittement montant**

WorrySentinel n'acquitte pas les signalements montants. Le traitement est interne à WorrySentinel.

---

## 10. Flux d'intégration typiques

### 10.1 Flux de supervision normale

**Acteurs :** WorrySentinel, LogisticsSteward

**Séquence :**

1. LogisticsSteward procède à des arbitrages normaux
2. LogisticsSteward génère des signaux d'allocation périodiques
3. LogisticsSteward envoie `ALLOCATION_PATTERN` à WorrySentinel
4. WorrySentinel observe et corrèle les patterns
5. Si pas d'anomalie, aucune action
6. Les signaux sont tracés pour audit

### 10.2 Flux de détection de dérive

**Acteurs :** LogisticsSteward, WorrySentinel

**Séquence :**

1. LogisticsSteward détecte une dérive de consommation sur une entité
2. LogisticsSteward génère un `DRIFT_ALERT`
3. LogisticsSteward envoie l'alerte à WorrySentinel
4. WorrySentinel reçoit et analyse l'alerte
5. WorrySentinel corrèle avec d'autres signaux
6. WorrySentinel décide de l'action (surveillance, durcissement, ou escalade)

### 10.3 Flux de durcissement

**Acteurs :** WorrySentinel, LogisticsSteward

**Séquence :**

1. WorrySentinel détecte une menace confirmée (corrélation de signaux)
2. WorrySentinel génère une `HARDENING_DIRECTIVE`
3. WorrySentinel envoie la directive à LogisticsSteward
4. LogisticsSteward reçoit la directive
5. LogisticsSteward applique immédiatement les restrictions
6. LogisticsSteward acquitte avec `ACK_OK`
7. Les entités concernées subissent les restrictions

### 10.4 Flux de levée de durcissement

**Acteurs :** WorrySentinel, LogisticsSteward

**Séquence :**

1. WorrySentinel constate que la menace est résolue
2. WorrySentinel génère une `HARDENING_LIFT`
3. WorrySentinel envoie la levée à LogisticsSteward
4. LogisticsSteward reçoit la levée
5. LogisticsSteward rétablit les règles d'arbitrage normales
6. LogisticsSteward acquitte avec `ACK_OK`

### 10.5 Diagramme de séquence

```
┌─────────────────┐    ┌─────────────────┐
│  WorrySentinel  │    │LogisticsSteward │
└────────┬────────┘    └────────┬────────┘
         │                      │
         │◄── ALLOCATION_PATTERN ─┤
         │                      │
         ├── Corrélation ───────┤
         │                      │
         │◄── DRIFT_ALERT ──────┤
         │                      │
         ├── Analyse ───────────┤
         │                      │
         ├── HARDENING_DIRECTIVE ─►│
         │    (quota_reduction)  │
         │                      │
         │                      ├── Application restrictions
         │                      │
         │◄── ACK_OK ───────────┤
         │                      │
         │   ... temps ...      │
         │                      │
         ├── HARDENING_LIFT ────►│
         │                      │
         │                      ├── Rétablissement
         │                      │
         │◄── ACK_OK ───────────┤
         │                      │
```

---

## 11. Règles d'intégration

### 11.1 Règles de communication

**Règle WS-LS-INT-01 : Bidirectionnel asymétrique**

La communication est bidirectionnelle mais asymétrique. WorrySentinel impose des contraintes, LogisticsSteward signale des observations. Les rôles ne sont pas interchangeables.

**Règle WS-LS-INT-02 : Priorité des contraintes sécuritaires**

Les contraintes de WorrySentinel sont prioritaires sur toutes les règles d'arbitrage de LogisticsSteward. Aucune règle locale ne peut contredire une contrainte de sécurité.

**Règle WS-LS-INT-03 : Non-blocage des signaux**

Les signalements de LogisticsSteward sont toujours non bloquants. L'envoi n'attend jamais de réponse.

### 11.2 Règles de données

**Règle WS-LS-INT-04 : Données de classification**

Les données échangées sont des informations de classification (états, niveaux, alertes, patterns), jamais des données métier.

**Règle WS-LS-INT-05 : Pas de données personnelles**

Aucune donnée personnelle n'est échangée. Les signalements concernent des entités (opérateurs, services), pas des utilisateurs.

**Règle WS-LS-INT-06 : Cohérence garantie**

WorrySentinel garantit la cohérence de ses notifications. LogisticsSteward peut se fier aux états et niveaux communiqués.

### 11.3 Règles de traçabilité

**Règle WS-LS-INT-07 : Traçabilité complète**

Toutes les interactions sont tracées avec contexte complet par les deux parties.

**Règle WS-LS-INT-08 : Corrélation possible**

Chaque notification peut être corrélée aux adaptations d'arbitrage qui en découlent.

---

## 12. Gestion des erreurs

### 12.1 Types d'erreurs

**Erreurs de format :**
- Signalement mal formé
- Champ obligatoire manquant
- Type de signalement inconnu

**Erreurs de corrélation :**
- Signal non corrélable avec d'autres sources
- Pattern non reconnu
- Entité inconnue

**Erreurs internes :**
- Erreur du moteur de corrélation
- Erreur de journalisation

### 12.2 Traitement des erreurs

**Règle WS-LS-ERR-01 : Journalisation des erreurs**

Toutes les erreurs sont journalisées pour audit et diagnostic.

**Règle WS-LS-ERR-02 : Pas de blocage sur erreur**

Une erreur de traitement ne bloque pas la supervision. WorrySentinel continue à recevoir et traiter les autres signaux.

**Règle WS-LS-ERR-03 : Sécurité par défaut**

En cas d'erreur de communication avec LogisticsSteward, WorrySentinel applique le comportement le plus restrictif (principe de sécurité par défaut).

**Règle WS-LS-ERR-04 : Alerte sur erreurs répétées**

Des erreurs répétées déclenchent une alerte interne et peuvent influencer l'état de confiance.

---

## 13. Cas particuliers

### 13.1 État de confiance T4 (Bloqué)

En état T4, WorrySentinel impose un mode survie :

**Règle WS-LS-CASE-01 : Blocage maximal**

En T4, WorrySentinel envoie une directive `ALLOCATION_BLOCK` globale. Seuls les services vitaux reçoivent des ressources.

### 13.2 LogisticsSteward indisponible

Si LogisticsSteward ne répond pas aux notifications :

**Règle WS-LS-CASE-02 : Escalade d'alerte**

L'indisponibilité de LogisticsSteward est une alerte de sécurité. WorrySentinel peut décider de dégrader l'état de confiance.

### 13.3 Signaux contradictoires

Si les signaux de LogisticsSteward contredisent d'autres sources :

**Règle WS-LS-CASE-03 : Priorité à la sécurité**

En cas de contradiction, WorrySentinel applique le scénario le plus restrictif. La sécurité prime sur la disponibilité.

### 13.4 Dérive sur entité de niveau 4

Si une dérive est détectée sur une entité de niveau de sécurité 4 :

**Règle WS-LS-CASE-04 : Escalade immédiate**

Toute dérive sur une entité de niveau 4 déclenche une escalade immédiate. L'état de confiance peut être dégradé.

---

## 14. Garanties de l'intégration

### 14.1 Garantie de supervision continue

**Engagement :** WorrySentinel supervise en continu les signaux de LogisticsSteward. Aucune interruption de supervision n'est acceptable.

### 14.2 Garantie de réactivité

**Engagement :** WorrySentinel réagit immédiatement aux alertes de dérive critique. Aucun délai supérieur à une seconde n'est acceptable pour les alertes critiques.

### 14.3 Garantie de proportionnalité

**Engagement :** Le durcissement est toujours proportionnel à la menace détectée. Pas de durcissement excessif sans justification.

### 14.4 Garantie de réversibilité

**Engagement :** Tout durcissement peut être levé par une directive explicite. Le retour à la normale est toujours possible.

### 14.5 Garantie de traçabilité

**Engagement :** Toute interaction est traçable de bout en bout. L'audit complet des notifications, directives et signalements est possible.

### 14.6 Garantie de non-substitution

**Engagement :** WorrySentinel ne se substitue jamais à LogisticsSteward. L'arbitrage reste la responsabilité exclusive de LogisticsSteward.

---

## 15. Invariants de l'intégration

### 15.1 Invariants de relation

**INV-WS-LS-1 : Supervision sans exécution**

WorrySentinel supervise LogisticsSteward. WorrySentinel n'exécute jamais d'arbitrage.

**INV-WS-LS-2 : Contrainte unidirectionnelle**

WorrySentinel impose des contraintes à LogisticsSteward. LogisticsSteward n'impose jamais de contraintes à WorrySentinel.

**INV-WS-LS-3 : Souveraineté d'arbitrage**

LogisticsSteward reste souverain sur l'arbitrage. WorrySentinel contraint, mais ne décide pas.

### 15.2 Invariants de données

**INV-WS-LS-4 : Pas de décision d'allocation**

WorrySentinel ne prend aucune décision d'allocation. Il impose des contraintes que LogisticsSteward traduit.

**INV-WS-LS-5 : Signaux informatifs**

Les signaux de LogisticsSteward sont informatifs. Ils alimentent la corrélation mais n'imposent aucune action.

### 15.3 Invariants de protocole

**INV-WS-LS-6 : Format respecté**

Toutes les notifications et signalements respectent le format standardisé.

**INV-WS-LS-7 : Traçabilité complète**

Toute interaction est traçable avec son contexte complet.

---

## 16. Conformité aux Lois d'Autonomie Système

### LOI-1 : Aucune dépendance externe critique

**Conformité :** ✅ **Conforme**

L'intégration respecte LOI-1 :
- WorrySentinel supervise localement
- LogisticsSteward adapte ses règles localement
- L'absence de connexion ne bloque ni la supervision ni l'arbitrage

### LOI-2 : Le système accepte l'isolement comme état normal

**Conformité :** ✅ **Conforme**

L'intégration respecte LOI-2 :
- En isolement, la supervision continue avec les signaux locaux
- Les contraintes locales restent actives
- Aucune dégradation de l'intégration en mode isolé

### LOI-4 : Pas de temps global requis

**Conformité :** ✅ **Conforme**

L'intégration respecte LOI-4 :
- Les horodatages sont locaux
- Aucune synchronisation temporelle n'est requise
- La corrélation ne dépend pas de timestamps synchronisés

---

## 17. Exemples

### 17.1 Notification de changement d'état de confiance

**Notification WorrySentinel :**
```
{
  "notification_id": "notif-ws-ls-001",
  "type": "TRUST_STATE_CHANGE",
  "payload": {
    "previous_state": "T0",
    "new_state": "T2",
    "transition_reason": "Dérives multiples détectées, corrélation confirmée",
    "constraints": {
      "quota_reduction_percent": 20,
      "priority_freeze": false
    }
  },
  "timestamp": "2026-01-28T14:00:00Z",
  "requires_ack": true
}
```

### 17.2 Directive de durcissement

**Directive WorrySentinel :**
```
{
  "notification_id": "notif-ws-ls-002",
  "type": "HARDENING_DIRECTIVE",
  "payload": {
    "directive_id": "hard-001",
    "hardening_type": "quota_reduction",
    "affected_entities": ["operator-media-service", "operator-analytics"],
    "severity": "high",
    "duration": null,
    "justification": "Pattern de consommation anormal, risque de saturation"
  },
  "timestamp": "2026-01-28T14:05:00Z",
  "requires_ack": true
}
```

### 17.3 Signalement de dérive (reçu par WorrySentinel)

**Signalement LogisticsSteward :**
```
{
  "signal_id": "signal-ls-001",
  "type": "DRIFT_ALERT",
  "payload": {
    "alert_id": "drift-001",
    "entity_id": "operator-media-service",
    "resource_type": "computation_quota",
    "current_usage": 92,
    "trend": "increasing",
    "projection": "2026-01-28T16:00:00Z"
  },
  "timestamp": "2026-01-28T14:00:00Z",
  "urgency": "high"
}
```

### 17.4 Levée de durcissement

**Directive WorrySentinel :**
```
{
  "notification_id": "notif-ws-ls-003",
  "type": "HARDENING_LIFT",
  "payload": {
    "lift_id": "lift-001",
    "directive_id": "hard-001",
    "justification": "Menace résolue, consommation normalisée"
  },
  "timestamp": "2026-01-28T18:00:00Z",
  "requires_ack": true
}
```

---

## 18. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit l'interface et le protocole que WorrySentinel doit respecter pour superviser LogisticsSteward.

Toute implémentation de l'intégration avec LogisticsSteward doit respecter ce contrat. Toute violation entraîne un comportement non conforme.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dépendances :**
- WorrySentinel - Documentation Fondatrice v1.2 (Section 9)
- LogisticsSteward - Documentation Fondatrice v1.0.0 (Section 8.4)
- LogisticsSteward - WorrySentinel Integration Contract v1.0
- Miyukini Conceptual References - Lois Autonomie Système v1.1

---

## 19. Mini log de génération

### Décision éditoriale E1 : Point de vue WorrySentinel

**Décision prise :** Ce document est rédigé du point de vue de WorrySentinel (superviseur), contrairement au document symétrique qui est du point de vue de LogisticsSteward (supervisé). Cette approche assure une documentation complète et cohérente des deux côtés.

**Application :** Tout le document est structuré autour du rôle de supervision de WorrySentinel.

### Décision éditoriale E2 : Cohérence avec le document symétrique

**Décision prise :** Ce document doit être cohérent avec [LogisticsSteward - WorrySentinel Integration Contract](../../../LogisticsSteward/contracts/integration/LogisticsSteward%20-%20WorrySentinel%20Integration%20Contract.md). Les mêmes structures de données, les mêmes règles, et les mêmes invariants sont utilisés.

**Application :** Les structures de données et les règles sont alignées avec le document symétrique.

### Décision éditoriale E3 : Supervision vs Substitution

**Décision prise :** Le document insiste sur le fait que WorrySentinel supervise sans se substituer à LogisticsSteward. Cette distinction est critique pour préserver la séparation des responsabilités.

**Application :** Section 4.1 et Section 5 établissent clairement cette distinction.

### Warning W1 : Risque de confusion supervision/arbitrage

**Warning rencontré :** Risque que WorrySentinel prenne des décisions d'allocation déguisées en contraintes de sécurité.

**Décision prise :** Les interdictions absolues (Section 5) clarifient que WorrySentinel ne décide jamais de l'allocation et ne définit jamais les quotas.

**Correction effectuée :** Section 5 explicite les interdictions, INV-WS-LS-4 établit que WorrySentinel ne prend aucune décision d'allocation.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec WorrySentinel - Documentation Fondatrice : Confirmée (Section 9 respectée)
- ✅ Cohérence avec LogisticsSteward - Documentation Fondatrice : Confirmée (Section 8.4)
- ✅ Cohérence avec LogisticsSteward - WorrySentinel Integration Contract : Confirmée (symétrie)
- ✅ Conformité LOI-1 : Confirmée (aucune dépendance externe)
- ✅ Conformité LOI-2 : Confirmée (isolement géré)
- ✅ Conformité LOI-4 : Confirmée (pas de temps global requis)
- ✅ Pas de décision d'allocation par WorrySentinel : Confirmée (INV-WS-LS-4)
- ✅ Supervision sans substitution : Confirmée (INV-WS-LS-3)
- ✅ Traçabilité complète : Confirmée (INV-WS-LS-7)
- ✅ Règles RÈGLE-WS-LS-1 à RÈGLE-WS-LS-4 respectées : Confirmée

**Conclusion :** Aucune contradiction détectée. Le document est cohérent et non ambigu.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
