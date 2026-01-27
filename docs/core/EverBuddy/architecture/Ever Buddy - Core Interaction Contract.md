# Ever Buddy - Core Interaction Contract

## Contexte

Ce document formalise les **interactions d'Ever Buddy avec les autres Cores** du Miyukini Core System. Il définit les contrats d'interface, les flux d'échange, et les responsabilités de chaque partie dans les interactions.

Ever Buddy, en tant que **core de cycle de vie et d'évolution** (Strate 4), interagit avec tous les autres cores pour fournir le contexte temporel nécessaire aux décisions et aux opérations du système.

**Document de référence :** [Ever Buddy - Documentation Fondatrice](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)

## Portée / Scope

- **Applicable à :** Toute interaction entre Ever Buddy et les autres cores
- **Audience :** Architectes, développeurs, intégrateurs
- **Statut :** Document contractuel normatif — CONTRAT D'INTERACTION

---

## 1. Principes généraux d'interaction

### 1.1 Nature des relations

Ever Buddy entretient des relations avec les autres cores qui suivent des patterns spécifiques :

| Pattern | Description | Cores concernés |
|---------|-------------|-----------------|
| **Consultative** | Le core demande un contexte de cycle de vie | StrongFather |
| **Complémentaire** | Les responsabilités se complètent sans chevauchement | KindMother |
| **Guidance** | Ever Buddy guide sans imposer | BondingBrother |
| **Alimentation** | Ever Buddy fournit des indicateurs | Caring Nanny |
| **Normative** | Ever Buddy définit les règles appliquées | Border Guard |
| **Descriptive** | Ever Buddy informe sur l'état de vie | Master Butler |
| **Escalade** | Ever Buddy signale le besoin d'intervention humaine | TAMR |

### 1.2 Invariants d'interaction

**INV-INT-1 : Jamais d'autorité mutuelle**

Les cores conservent leur autonomie. Ever Buddy influence par l'information, jamais par la contrainte. Aucun core ne peut forcer Ever Buddy à modifier ses règles d'évolution.

**INV-INT-2 : Flux unidirectionnels ou bidirectionnels explicites**

Chaque interaction a une direction explicite. Les flux bidirectionnels sont documentés comme deux flux unidirectionnels distincts.

**INV-INT-3 : Aucune modification de données**

Ever Buddy ne modifie jamais les données ou états des autres cores. Il observe, enregistre, recommande, mais l'exécution reste sous l'autorité du core concerné.

---

## 2. Relations avec chaque Core

### 2.1 Relation avec KindMother

**Type de relation :** Complémentaire

**Principe fondamental :**

> Ever Buddy gouverne comment les structures de données évoluent de T à T+1. KindMother gère les données à un instant T.

**Responsabilités respectives :**

| Aspect | KindMother | Ever Buddy |
|--------|------------|------------|
| Données à instant T | ✅ Autorité | ❌ Lecture seule |
| Schémas de données | ✅ Définition | ✅ Règles d'évolution |
| Migrations de données | ✅ Exécution | ✅ Définition des règles |
| Versionnement des schémas | ❌ Non concerné | ✅ Gouvernance |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│ Ever Buddy  │                      │ KindMother  │
│             │  Règles d'évolution  │             │
│             │ ──────────────────► │             │
│             │                      │             │
│             │  État des migrations │             │
│             │ ◄────────────────── │             │
└─────────────┘                      └─────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| EB → KM | Règles de compatibilité des schémas | `CompatibilityRules` |
| EB → KM | Chemins de migration recommandés | `MigrationPath` |
| KM → EB | État d'avancement des migrations | `MigrationStatus` |
| KM → EB | Déclaration de nouveaux schémas | `SchemaDeclaration` |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-KM-1** | Ever Buddy ne modifie jamais les données gérées par KindMother |
| **COL-KM-2** | KindMother notifie Ever Buddy de tout nouveau schéma |
| **COL-KM-3** | Les migrations sont définies par Ever Buddy, exécutées par KindMother |
| **COL-KM-4** | KindMother peut refuser une migration si elle viole ses propres invariants |

**Référence Glossaire :** [KindMother](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#kindmother)

---

### 2.2 Relation avec StrongFather

**Type de relation :** Consultative

**Principe fondamental :**

> StrongFather décide si une action est autorisée. Ever Buddy fournit le contexte de cycle de vie nécessaire à la décision.

**Responsabilités respectives :**

| Aspect | StrongFather | Ever Buddy |
|--------|--------------|------------|
| Décision d'autorisation | ✅ Autorité | ❌ Aucune |
| Contexte de cycle de vie | ❌ Consommateur | ✅ Fournisseur |
| Évaluation de l'impact | ✅ Décision finale | ✅ Information sur l'évolution |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│StrongFather │  Demande de contexte │ Ever Buddy  │
│             │ ──────────────────► │             │
│             │                      │             │
│             │  Contexte cycle vie  │             │
│             │ ◄────────────────── │             │
│             │                      │             │
│  DÉCISION   │                      │  (aucune)   │
└─────────────┘                      └─────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| SF → EB | Demande de contexte pour un élément | `LifecycleContextRequest` |
| EB → SF | État de cycle de vie actuel | `LifecycleState` |
| EB → SF | Historique de transitions | `TransitionHistory` |
| EB → SF | Recommandations associées | `EvolutionRecommendations` |

**Informations fournies par Ever Buddy à StrongFather :**

| Information | Description | Usage par StrongFather |
|-------------|-------------|------------------------|
| `current_state` | DRAFT, ACTIVE, DEPRECATED, RETIRED, ARCHIVED | Évaluer si l'action est permise |
| `deprecation_date` | Date de dépréciation (si applicable) | Évaluer l'urgence de migration |
| `successor_id` | Identifiant du successeur (si existe) | Rediriger vers le successeur |
| `compatibility_level` | Niveau de compatibilité | Évaluer les risques de l'action |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-SF-1** | StrongFather peut consulter Ever Buddy mais la décision finale lui appartient |
| **COL-SF-2** | Ever Buddy ne prend jamais de décision à la place de StrongFather |
| **COL-SF-3** | StrongFather peut ignorer les recommandations d'Ever Buddy (mais c'est tracé) |
| **COL-SF-4** | Ever Buddy fournit le contexte dans un délai garanti (non-bloquant) |

**Référence Glossaire :** [StrongFather](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#strongfather)

---

### 2.3 Relation avec BondingBrother

**Type de relation :** Guidance

**Principe fondamental :**

> BondingBrother traduit les intentions entre produits et autorités. Ever Buddy guide les traductions selon les règles de compatibilité et d'évolution.

**Responsabilités respectives :**

| Aspect | BondingBrother | Ever Buddy |
|--------|----------------|------------|
| Traduction des intentions | ✅ Exécution | ❌ Aucune |
| Règles de compatibilité | ❌ Consommateur | ✅ Fournisseur |
| Adaptation entre versions | ✅ Application | ✅ Définition |
| Médiation produits ↔ cores | ✅ Autorité | ❌ Aucune |

**Flux d'interaction :**

```
┌─────────────────┐                      ┌─────────────┐
│ BondingBrother  │  Demande compat.     │ Ever Buddy  │
│                 │ ──────────────────► │             │
│                 │                      │             │
│                 │  Règles adaptation   │             │
│                 │ ◄────────────────── │             │
│                 │                      │             │
│  TRADUCTION     │                      │  (aucune)   │
└─────────────────┘                      └─────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| BB → EB | Demande de règles de compatibilité | `CompatibilityRequest` |
| BB → EB | Transmission d'alertes aux produits | `AlertForwarding` |
| EB → BB | Règles d'adaptation entre versions | `AdaptationRules` |
| EB → BB | Alertes de dépréciation | `DeprecationAlert` |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-BB-1** | BondingBrother peut adapter ses traductions selon les conseils d'Ever Buddy |
| **COL-BB-2** | Ever Buddy ne traduit jamais lui-même |
| **COL-BB-3** | Les alertes d'Ever Buddy sont transmises aux produits via BondingBrother |
| **COL-BB-4** | Les produits ne parlent jamais directement à Ever Buddy |

**Référence Glossaire :** [BondingBrother](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#bondingbrother)

---

### 2.4 Relation avec Caring Nanny

**Type de relation :** Alimentation

**Principe fondamental :**

> Caring Nanny observe l'état de santé du système. Ever Buddy fournit les indicateurs d'évolution qui affectent cette santé.

**Responsabilités respectives :**

| Aspect | Caring Nanny | Ever Buddy |
|--------|--------------|------------|
| Observation d'état global | ✅ Autorité | ❌ Aucune |
| Indicateurs d'évolution | ❌ Consommateur | ✅ Fournisseur |
| Rapport de santé | ✅ Production | ❌ Contribution |
| Détection d'anomalies | ✅ Autorité | ❌ Source de données |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│ Ever Buddy  │  Indicateurs évol.   │Caring Nanny │
│             │ ──────────────────► │             │
│             │                      │             │
│  (aucun)    │                      │  RAPPORT    │
└─────────────┘                      └─────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| EB → CN | Transitions en cours | `ActiveTransitions` |
| EB → CN | Dépréciations imminentes | `PendingDeprecations` |
| EB → CN | Debt ratio actuel | `DebtMetrics` |
| EB → CN | Alertes d'évolution | `EvolutionAlerts` |

**Indicateurs fournis par Ever Buddy :**

| Indicateur | Description | Impact sur la santé |
|------------|-------------|---------------------|
| `active_transitions` | Nombre de transitions en cours | Charge d'évolution |
| `pending_deprecations` | Éléments bientôt retirés | Risque de rupture |
| `debt_ratio` | (DEPRECATED + RETIRED) / ACTIVE | Dette structurelle |
| `blocked_transitions` | Transitions au-delà de la période prévue | Problème d'adoption |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-CN-1** | Ever Buddy publie proactivement ses indicateurs vers Caring Nanny |
| **COL-CN-2** | Caring Nanny intègre ces indicateurs dans son rapport de santé |
| **COL-CN-3** | La fréquence de publication est définie par Ever Buddy |
| **COL-CN-4** | Caring Nanny ne demande jamais de modifier un état de cycle de vie |

**Référence Glossaire :** [Caring Nanny](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#caring-nanny)

---

### 2.5 Relation avec Border Guard

**Type de relation :** Normative

**Principe fondamental :**

> Border Guard applique les règles aux frontières. Ever Buddy définit les règles de compatibilité qui s'appliquent.

**Responsabilités respectives :**

| Aspect | Border Guard | Ever Buddy |
|--------|--------------|------------|
| Application aux frontières | ✅ Autorité | ❌ Aucune |
| Définition des règles de compatibilité | ❌ Consommateur | ✅ Fournisseur |
| Versions supportées | ❌ Application | ✅ Définition |
| Vérification d'intégration | ✅ Exécution | ✅ Critères |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│ Ever Buddy  │  Règles compatibilité│Border Guard │
│             │ ──────────────────► │             │
│             │                      │             │
│  (définit)  │                      │ (applique)  │
└─────────────┘                      └─────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| EB → BG | Versions supportées par interface | `SupportedVersions` |
| EB → BG | Règles de compatibilité en vigueur | `CompatibilityRules` |
| EB → BG | Fenêtres de compatibilité | `CompatibilityWindows` |
| BG → EB | Intégrations refusées pour incompatibilité | `RejectionReport` |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-BG-1** | Ever Buddy définit les versions acceptables aux frontières |
| **COL-BG-2** | Border Guard applique ces règles sans les modifier |
| **COL-BG-3** | Border Guard notifie Ever Buddy des rejets pour incompatibilité |
| **COL-BG-4** | Les fenêtres de compatibilité sont non négociables |

**Référence Glossaire :** [Border Guard](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#border-guard)

---

### 2.6 Relation avec Master Butler

**Type de relation :** Descriptive

**Principe fondamental :**

> Master Butler expose les capacités disponibles. Ever Buddy indique l'état de vie de chaque capacité.

**Responsabilités respectives :**

| Aspect | Master Butler | Ever Buddy |
|--------|---------------|------------|
| Catalogue des capacités | ✅ Autorité | ❌ Aucune |
| État de vie des capacités | ❌ Consommateur | ✅ Fournisseur |
| Exposition des capacités | ✅ Exécution | ❌ Aucune |
| Versionnement des capacités | ❌ Application | ✅ Gouvernance |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│ Ever Buddy  │  État vie capacités  │Master Butler│
│             │ ──────────────────► │             │
│             │                      │             │
│             │  Nouvelles capacités │             │
│             │ ◄────────────────── │             │
│             │                      │             │
│  (gouverne) │                      │  (expose)   │
└─────────────┘                      └─────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| MB → EB | Déclaration de nouvelle capacité | `CapabilityDeclaration` |
| EB → MB | État de vie de chaque capacité | `CapabilityLifecycle` |
| EB → MB | Capacités dépréciées | `DeprecatedCapabilities` |
| EB → MB | Capacités retirées | `RetiredCapabilities` |

**Impact sur l'exposition des capacités :**

| État EB | Comportement Master Butler |
|---------|---------------------------|
| DRAFT | Capacité non exposée |
| ACTIVE | Capacité pleinement exposée |
| DEPRECATED | Capacité exposée avec avertissement |
| RETIRED | Capacité non exposée (erreur si appelée) |
| ARCHIVED | Capacité inexistante |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-MB-1** | Master Butler notifie Ever Buddy de toute nouvelle capacité |
| **COL-MB-2** | Ever Buddy assigne un état de vie initial (DRAFT ou ACTIVE) |
| **COL-MB-3** | Master Butler adapte son exposition selon l'état fourni par Ever Buddy |
| **COL-MB-4** | Les capacités RETIRED ne sont plus exposées par Master Butler |

**Référence Glossaire :** [Master Butler](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#master-butler)

---

### 2.7 Relation avec TAMR

**Type de relation :** Escalade

**Principe fondamental :**

> TAMR définit quand l'humain intervient. Ever Buddy signale les transitions qui nécessitent une intervention humaine.

**Responsabilités respectives :**

| Aspect | TAMR | Ever Buddy |
|--------|------|------------|
| Points d'intervention humaine | ✅ Autorité | ❌ Aucune |
| Signalement de besoin d'intervention | ❌ Destinataire | ✅ Émetteur |
| Validation humaine des transitions | ✅ Exécution | ❌ Aucune |
| Décision de transition majeure | ✅ Validation finale | ✅ Proposition |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│ Ever Buddy  │  Besoin intervention │    TAMR     │
│             │ ──────────────────► │             │
│             │                      │             │
│             │  Validation humaine  │             │
│             │ ◄────────────────── │             │
│             │                      │             │
│ (enregistre)│                      │ (valide)    │
└─────────────┘                      └─────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| EB → TAMR | Demande de validation de transition majeure | `TransitionValidationRequest` |
| EB → TAMR | Signalement de rupture de compatibilité | `BreakingChangeAlert` |
| TAMR → EB | Validation de la transition | `HumanValidation` |
| TAMR → EB | Refus avec justification | `HumanRejection` |

**Cas nécessitant une escalade vers TAMR :**

| Cas | Description | Sévérité |
|-----|-------------|----------|
| Migration majeure | Changement de version majeure | Élevée |
| Rupture de compatibilité | Breaking change déclaré | Élevée |
| Accélération de dépréciation | Réduction de la période de dépréciation | Moyenne |
| Archivage d'éléments critiques | Éléments marqués FONDATION | Critique |
| Réactivation après DEPRECATED | Retour DEPRECATED → ACTIVE | Moyenne |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-TAMR-1** | Ever Buddy signale automatiquement les transitions critiques à TAMR |
| **COL-TAMR-2** | TAMR peut bloquer une transition en attente de validation humaine |
| **COL-TAMR-3** | Une transition bloquée par TAMR ne peut être forcée par Ever Buddy |
| **COL-TAMR-4** | La validation TAMR est enregistrée dans l'historique immuable |

**Référence Glossaire :** [TAMR](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#tamr-trust--authority-mediation-resolver)

---

### 2.8 Relation avec WorrySentinel

**Type de relation :** Informative bidirectionnelle

**Principe fondamental :**

> WorrySentinel gouverne la sécurité. Ever Buddy informe des évolutions qui peuvent affecter la sécurité et reçoit les alertes de sécurité qui peuvent bloquer des transitions.

**Responsabilités respectives :**

| Aspect | WorrySentinel | Ever Buddy |
|--------|---------------|------------|
| États de confiance (T0-T4) | ✅ Autorité | ❌ Consommateur |
| Impact sécurité des évolutions | ❌ Destinataire | ✅ Signalement |
| Blocage de transitions pour sécurité | ✅ Autorité | ❌ Soumis |
| Audit des transitions | ❌ Consommateur | ✅ Fournisseur |

**Flux d'interaction :**

```
┌─────────────┐                      ┌───────────────┐
│ Ever Buddy  │  Évolutions à risque │WorrySentinel  │
│             │ ──────────────────► │               │
│             │                      │               │
│             │  État de confiance   │               │
│             │ ◄────────────────── │               │
│             │                      │               │
│ (adapte)    │                      │ (gouverne)    │
└─────────────┘                      └───────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| EB → WS | Transitions avec impact sécurité potentiel | `SecurityImpactAlert` |
| EB → WS | Historique des transitions pour audit | `TransitionAuditLog` |
| WS → EB | État de confiance actuel | `TrustState` |
| WS → EB | Blocage de transition pour raison de sécurité | `SecurityBlock` |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-WS-1** | En état T3 ou T4, les transitions non critiques sont suspendues |
| **COL-WS-2** | WorrySentinel peut bloquer une transition pour raison de sécurité |
| **COL-WS-3** | Ever Buddy fournit l'historique complet pour les audits de sécurité |
| **COL-WS-4** | Les transitions bloquées par sécurité sont tracées séparément |

**Référence Glossaire :** [WorrySentinel](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#worrysentinel)

---

## 3. Flux d'interaction transversaux

### 3.1 Flux d'observation

Ever Buddy observe continuellement l'état du système pour maintenir sa connaissance des cycles de vie.

**Séquence :**

```
1. Réception des déclarations
   ┌─────────────┐    déclaration    ┌─────────────┐
   │ Cores/      │ ───────────────► │ Ever Buddy  │
   │ Produits    │                   │             │
   └─────────────┘                   └─────────────┘

2. Enregistrement de l'état
   Ever Buddy enregistre l'état de cycle de vie de chaque élément

3. Surveillance des transitions
   Ever Buddy détecte les demandes de transition d'état

4. Validation des transitions
   Ever Buddy vérifie que la transition respecte les règles

5. Enregistrement de la transition
   Si valide, la transition est enregistrée dans l'historique immuable
```

**Sources d'observation :**

| Source | Type de déclaration |
|--------|---------------------|
| KindMother | Nouveaux schémas de données |
| Master Butler | Nouvelles capacités |
| BondingBrother | Nouvelles interfaces de traduction |
| Produits (via BB) | Nouveaux éléments métier |

### 3.2 Flux de consultation

Les autres cores consultent Ever Buddy pour obtenir des informations de cycle de vie.

**Séquence :**

```
1. Demande de contexte
   ┌─────────────┐    demande contexte   ┌─────────────┐
   │ Core        │ ─────────────────────► │ Ever Buddy  │
   │ demandeur   │                        │             │
   └─────────────┘                        └─────────────┘

2. Recherche de l'état
   Ever Buddy recherche l'état actuel et l'historique de l'élément

3. Fourniture du contexte
   ┌─────────────┐                        ┌─────────────┐
   │ Core        │    contexte complet    │ Ever Buddy  │
   │ demandeur   │ ◄───────────────────── │             │
   └─────────────┘                        └─────────────┘

4. Utilisation par le demandeur
   Le core demandeur utilise ce contexte pour sa propre décision
```

**Temps de réponse garanti :**

| Type de demande | Temps de réponse maximum |
|-----------------|-------------------------|
| État actuel simple | < 10ms |
| Historique complet | < 100ms |
| Recommandations | < 50ms |
| Contexte complet | < 200ms |

### 3.3 Flux de planification

Ever Buddy communique les planifications d'évolution aux consommateurs.

**Séquence :**

```
1. Définition du plan
   Ever Buddy définit un plan de transition
   (dépréciation, retirement, archivage)

2. Communication
   ┌─────────────┐    plan transition    ┌─────────────────┐
   │ Ever Buddy  │ ────────────────────► │ BondingBrother  │
   └─────────────┘                       └────────┬────────┘
                                                  │
                                                  ▼
                                         ┌─────────────────┐
                                         │   Produits      │
                                         └─────────────────┘

3. Période de transition
   L'ancien et le nouveau coexistent

4. Suivi de l'adoption
   Ever Buddy observe l'adoption du nouveau par les consommateurs

5. Complétion
   À la fin de la période, la transition est complétée
```

**Canaux de communication :**

| Destinataire | Canal | Fréquence |
|--------------|-------|-----------|
| Cores | Direct | Immédiat |
| Produits | Via BondingBrother | Immédiat |
| Caring Nanny | Publication métriques | Périodique |

### 3.4 Flux d'alerte

Ever Buddy alerte quand des conditions anormales sont détectées.

**Séquence :**

```
1. Détection
   Ever Buddy détecte une condition anormale

2. Évaluation
   ┌───────────────────────────────────────┐
   │ Évaluation de la gravité et urgence   │
   │ - Dette excessive                     │
   │ - Transition bloquée                  │
   │ - Incompatibilité détectée            │
   └───────────────────────────────────────┘

3. Alerte
   ┌─────────────┐    alerte    ┌─────────────────┐
   │ Ever Buddy  │ ───────────► │ Destinataires   │
   └─────────────┘              │ (selon gravité) │
                                └─────────────────┘

4. Recommandation
   Ever Buddy fournit des recommandations pour résoudre

5. Suivi
   Ever Buddy suit la résolution et clôture l'alerte
```

**Niveaux d'alerte :**

| Niveau | Description | Destinataires |
|--------|-------------|---------------|
| INFO | Information non critique | Caring Nanny |
| WARNING | Situation à surveiller | Caring Nanny, StrongFather |
| CRITICAL | Action requise | Tous les cores, TAMR |
| EMERGENCY | Blocage imminent | Tous les cores, TAMR, WorrySentinel |

---

## 4. Relation avec les produits

### 4.1 Principe fondamental

**Les produits ne parlent jamais directement à Ever Buddy.**

Toute interaction passe par BondingBrother qui traduit et filtre les échanges.

```
┌─────────────┐                                    ┌─────────────┐
│  Produits   │ ──────────────────────────────────► │ Ever Buddy  │
│             │              ❌ INTERDIT            │             │
└─────────────┘                                    └─────────────┘

┌─────────────┐    via     ┌───────────────┐       ┌─────────────┐
│  Produits   │ ─────────► │BondingBrother │ ────► │ Ever Buddy  │
│             │            │               │       │             │
└─────────────┘            └───────────────┘       └─────────────┘
               ✅ AUTORISÉ
```

### 4.2 Ce que les produits peuvent demander (via BondingBrother)

| Demande | Réponse d'Ever Buddy |
|---------|---------------------|
| "Est-ce que X est encore supporté ?" | État de cycle de vie de X |
| "Quelle est la version recommandée de Y ?" | Successeur de Y (si existe) |
| "Quand Z sera-t-il retiré ?" | Date de retirement prévue |
| "Suis-je compatible avec W ?" | Niveau de compatibilité |

### 4.3 Ce que les produits reçoivent (via BondingBrother)

| Type | Description |
|------|-------------|
| Alertes de dépréciation | Éléments utilisés bientôt retirés |
| Recommandations de migration | Chemins vers les successeurs |
| Fenêtres de compatibilité | Versions avec lesquelles ils sont compatibles |
| Notifications de transition | Changements d'état des éléments utilisés |

---

## 5. Diagramme d'interaction globale

```
                              ┌─────────────────────────────────────────────┐
                              │                 EVER BUDDY                   │
                              │  ┌───────────┐  ┌───────────┐  ┌───────────┐ │
                              │  │ Registre  │  │  Règles   │  │ Historique│ │
                              │  │ des états │  │ évolution │  │  immuable │ │
                              │  └─────┬─────┘  └─────┬─────┘  └─────┬─────┘ │
                              │        │              │              │       │
                              │        └──────────────┼──────────────┘       │
                              │                       │                      │
                              └───────────────────────┼──────────────────────┘
                                                      │
        ┌─────────────────────────────────────────────┼─────────────────────────────────────────────┐
        │                                             │                                             │
        ▼                                             ▼                                             ▼
┌───────────────┐                            ┌───────────────┐                            ┌───────────────┐
│  KindMother   │                            │ StrongFather  │                            │BondingBrother │
│ (complémen.)  │                            │ (consultatif) │                            │  (guidance)   │
└───────────────┘                            └───────────────┘                            └───────┬───────┘
                                                                                                  │
        ┌─────────────────────────────────────────────┬─────────────────────────────────┐         │
        │                                             │                                 │         │
        ▼                                             ▼                                 ▼         ▼
┌───────────────┐                            ┌───────────────┐                  ┌───────────────────────┐
│ Caring Nanny  │                            │ Border Guard  │                  │      PRODUITS         │
│ (alimentation)│                            │  (normatif)   │                  │ (via BondingBrother)  │
└───────────────┘                            └───────────────┘                  └───────────────────────┘

        ┌─────────────────────────────────────────────┬─────────────────────────────────┐
        │                                             │                                 │
        ▼                                             ▼                                 ▼
┌───────────────┐                            ┌───────────────┐                  ┌───────────────┐
│ Master Butler │                            │     TAMR      │                  │WorrySentinel  │
│ (descriptif)  │                            │  (escalade)   │                  │(informatif bi)│
└───────────────┘                            └───────────────┘                  └───────────────┘
```

---

## 6. Synthèse des contrats d'interface

### 6.1 Matrice des interactions

| Core | Direction | Nature | Données échangées |
|------|-----------|--------|-------------------|
| **KindMother** | EB ↔ KM | Complémentaire | Règles évolution ↔ État migrations |
| **StrongFather** | SF → EB | Consultative | Demande contexte → État cycle vie |
| **BondingBrother** | EB ↔ BB | Guidance | Règles compat ↔ Alertes transmises |
| **Caring Nanny** | EB → CN | Alimentation | Indicateurs évolution (unidirectionnel) |
| **Border Guard** | EB → BG | Normative | Règles compatibilité (unidirectionnel) |
| **Master Butler** | EB ↔ MB | Descriptive | État capacités ↔ Nouvelles capacités |
| **TAMR** | EB ↔ TAMR | Escalade | Besoin validation ↔ Validation humaine |
| **WorrySentinel** | EB ↔ WS | Bidirectionnelle | Impact sécurité ↔ État confiance |

### 6.2 Garanties de service

| Garantie | Valeur | Condition |
|----------|--------|-----------|
| Temps de réponse consultation | < 200ms | État système normal |
| Disponibilité du registre | 99.9% | Hors maintenance |
| Immuabilité de l'historique | 100% | Invariant structural |
| Délai de propagation des alertes | < 1s | État système normal |

---

## 7. Conformité aux Lois d'Autonomie

### 7.1 LOI-1 : Aucune dépendance externe critique

Toutes les interactions sont locales. Ever Buddy n'a pas besoin de service externe pour interagir avec les autres cores.

### 7.2 LOI-2 : Le système accepte l'isolement

En mode isolé, Ever Buddy continue d'interagir avec les cores locaux. Les interactions avec les produits distants sont suspendues mais pas perdues.

### 7.3 LOI-6 : L'autonomie n'empêche pas la fédération

Les informations de cycle de vie peuvent être partagées entre COG via BondingBrother, sans créer de dépendance obligatoire.

---

## 8. Références

### Documents fondateurs

- [Ever Buddy - Documentation Fondatrice](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)

### Contrats associés

- [Ever Buddy - Lifecycle States Contract](../contracts/lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md)
- [Ever Buddy - Transition Rules Contract](../contracts/lifecycle/Ever%20Buddy%20-%20Transition%20Rules%20Contract.md)
- [Ever Buddy - Compatibility Rules Contract](../contracts/compatibility/Ever%20Buddy%20-%20Compatibility%20Rules%20Contract.md)

### Références Glossaire

- [Glossaire - Ever Buddy](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#ever-buddy)
- [Glossaire - Cores](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#cores)

### Lois d'Autonomie

- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Contrat normatif — ARCHITECTURE  
**Référence :** Ever Buddy - Documentation Fondatrice v1.3, Sections 3 et 8
