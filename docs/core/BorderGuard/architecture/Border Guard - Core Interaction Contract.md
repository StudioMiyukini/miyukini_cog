# Border Guard - Core Interaction Contract

## 1. Contexte

Ce document formalise les **interactions de Border Guard avec les autres Cores** du Miyukini Core System. Il définit les contrats d'interface, les flux d'échange, et les responsabilités de chaque partie dans les interactions.

Border Guard, en tant que **core de définition des frontières et classification de confiance** (Strate 2 - Frontière), interagit avec tous les autres cores pour fournir le contexte de frontière nécessaire aux décisions et aux opérations du système.

**Document de référence :** [Border Guard - Documentation Fondatrice](../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

---

## 2. Portée / Scope

- **Applicable à :** Toute interaction entre Border Guard et les autres cores
- **Audience :** Architectes, développeurs, intégrateurs
- **Statut :** Document contractuel normatif — CONTRAT D'INTERACTION

---

## 3. Principes généraux d'interaction

### 3.1 Nature des relations

Border Guard entretient des relations avec les autres cores qui suivent des patterns spécifiques :

| Pattern | Description | Cores concernés |
|---------|-------------|-----------------|
| **Conseil** | Border Guard fournit un contexte informatif | StrongFather |
| **Complémentarité** | Les responsabilités se complètent sans chevauchement | KindMother |
| **Définition/Application** | Border Guard définit, l'autre applique | BondingBrother |
| **Information** | Border Guard signale des changements d'état | Caring Nanny |
| **Normative** | Border Guard reçoit des règles de compatibilité | Ever Buddy |
| **Consultation** | Border Guard fournit des informations de frontière | Master Butler |
| **Escalade** | Border Guard signale le besoin d'intervention | TAMR |

### 3.2 Invariants d'interaction

**INV-INT-BG-1 : Border Guard ne décide jamais**

Border Guard informe, classifie, définit, mais la décision finale appartient toujours au core approprié (StrongFather pour les décisions stratégiques).

**INV-INT-BG-2 : Border Guard n'exécute jamais**

Border Guard ne filtre pas, ne bloque pas, n'applique pas. L'exécution est du ressort de BondingBrother et des autres cores opérationnels.

**INV-INT-BG-3 : Flux explicites et traçables**

Chaque interaction a une direction explicite. Les flux bidirectionnels sont documentés comme deux flux unidirectionnels distincts.

**INV-INT-BG-4 : Aucune modification d'état par Border Guard**

Border Guard ne modifie jamais l'état des autres cores. Il observe, définit, conseille, mais la modification d'état reste sous l'autorité du core concerné.

---

## 4. Relations avec chaque Core

### 4.1 Relation avec StrongFather

**Type de relation :** Conseil

**Principe fondamental :**

> StrongFather décide si une action est autorisée. Border Guard fournit le contexte de confiance nécessaire à la décision.

**Responsabilités respectives :**

| Aspect | StrongFather | Border Guard |
|--------|--------------|--------------|
| Décision d'autorisation | ✅ Autorité | ❌ Aucune |
| Contexte de confiance | ❌ Consommateur | ✅ Fournisseur |
| Évaluation du risque | ✅ Décision finale | ✅ Information sur la confiance |
| Évaluation de l'intention | ✅ Autorité | ❌ Aucune |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│StrongFather │  Demande contexte    │ Border Guard│
│             │ ──────────────────► │             │
│             │                      │             │
│             │  Contexte frontière  │             │
│             │ ◄────────────────── │             │
│             │                      │             │
│  DÉCISION   │                      │  (aucune)   │
└─────────────┘                      └─────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| SF → BG | Demande de contexte pour une interaction | `BoundaryContextRequest` |
| BG → SF | Niveau de confiance de la source | `TrustLevel` |
| BG → SF | Frontières traversées | `CrossedBoundaries` |
| BG → SF | Règles applicables | `ApplicableRules` |

**Informations fournies par Border Guard à StrongFather :**

| Information | Description | Usage par StrongFather |
|-------------|-------------|------------------------|
| `source_trust_level` | trusted, verified, unknown, hostile | Évaluer la fiabilité de l'intention |
| `crossed_boundaries` | Liste des frontières traversées | Évaluer le risque du franchissement |
| `applicable_rules` | Règles de franchissement en vigueur | Vérifier la conformité de l'intention |
| `integration_state` | État de l'intégration source (si applicable) | Évaluer si la source est autorisée |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-SF-1** | StrongFather peut consulter Border Guard mais la décision finale lui appartient |
| **COL-SF-2** | Border Guard ne prend jamais de décision à la place de StrongFather |
| **COL-SF-3** | StrongFather peut ignorer les informations de Border Guard (mais c'est tracé) |
| **COL-SF-4** | Border Guard fournit le contexte dans un délai garanti (non-bloquant) |

**Référence Documentation Fondatrice :** Section 3.2 (Relation avec Strong Father) et Section 8.1 (Flux d'information vers Strong Father)

---

### 4.2 Relation avec KindMother

**Type de relation :** Complémentarité

**Principe fondamental :**

> KindMother gouverne les données et leur persistance. Border Guard gouverne les frontières et les niveaux de confiance. Ce qui vient de l'extérieur passe par Border Guard avant d'être traité par KindMother.

**Responsabilités respectives :**

| Aspect | KindMother | Border Guard |
|--------|------------|--------------|
| Persistance des données | ✅ Autorité | ❌ Aucune |
| Conditions d'entrée des données | ❌ Non concerné | ✅ Définition |
| Synchronisation | ✅ Exécution | ❌ Aucune |
| Persistance des définitions de frontières | ✅ Stockage | ✅ Définition |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│ Border Guard│  Définitions à       │ KindMother  │
│             │  persister           │             │
│             │ ──────────────────► │             │
│             │                      │             │
│  (définit)  │                      │ (stocke)    │
└─────────────┘                      └─────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| BG → KM | Définitions de frontières à persister | `BoundaryDefinition` |
| BG → KM | Classifications à persister | `TrustClassification` |
| BG → KM | Règles à persister | `CrossingRule` |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-KM-1** | Border Guard ne persiste jamais directement (INV-BG-2) |
| **COL-KM-2** | KindMother stocke les définitions de Border Guard sans les modifier |
| **COL-KM-3** | Border Guard traite les données une fois qu'elles sont "à l'intérieur" est du ressort de KindMother |
| **COL-KM-4** | La synchronisation des définitions de frontières est gérée par KindMother |

**Référence Documentation Fondatrice :** Section 3.1 (Relation avec Kind Mother)

---

### 4.3 Relation avec BondingBrother

**Type de relation :** Définition/Application

**Principe fondamental :**

> Border Guard définit les règles de franchissement des frontières. BondingBrother applique ces règles lors de la médiation entre les produits et l'écosystème.

**Responsabilités respectives :**

| Aspect | BondingBrother | Border Guard |
|--------|----------------|--------------|
| Définition des règles | ❌ Consommateur | ✅ Autorité |
| Application des règles | ✅ Exécution | ❌ Aucune |
| Médiation produits ↔ cores | ✅ Autorité | ❌ Aucune |
| Filtrage aux frontières | ✅ Exécution | ❌ Aucune |

**Flux d'interaction :**

```
┌─────────────┐                      ┌───────────────┐
│ Border Guard│  Règles franchissement│ BondingBrother│
│             │ ──────────────────► │               │
│             │                      │               │
│             │  Demande règles     │               │
│             │ ◄────────────────── │               │
│             │                      │               │
│  (définit)  │                      │  (applique)   │
└─────────────┘                      └───────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| BB → BG | Demande de règles pour une frontière | `RulesRequest` |
| BG → BB | Règles de franchissement applicables | `CrossingRules` |
| BG → BB | Niveau de confiance d'une source | `TrustLevel` |
| BB → BG | Notification de franchissement effectué | `CrossingNotification` |

**Relation fondamentale et asymétrique :**

Cette relation est **non négociable** selon la Documentation Fondatrice :

- BondingBrother ne définit **jamais** de frontière
- Border Guard n'applique **jamais** de règle
- La séparation est **absolue**

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-BB-1** | BondingBrother consulte Border Guard avant tout franchissement de frontière |
| **COL-BB-2** | Border Guard fournit les règles, BondingBrother les applique |
| **COL-BB-3** | BondingBrother notifie Border Guard des franchissements effectués (traçabilité) |
| **COL-BB-4** | Les produits ne parlent jamais directement à Border Guard |

**Référence Documentation Fondatrice :** Section 3.3 (Relation avec Bonding Brother) et Section 8.2 (Flux de règles vers Bonding Brother)

---

### 4.4 Relation avec Caring Nanny

**Type de relation :** Information

**Principe fondamental :**

> Caring Nanny observe l'état global du système. Border Guard informe Caring Nanny de l'état des frontières pour enrichir cette observation.

**Responsabilités respectives :**

| Aspect | Caring Nanny | Border Guard |
|--------|--------------|--------------|
| Observation d'état global | ✅ Autorité | ❌ Aucune |
| État des frontières | ❌ Consommateur | ✅ Fournisseur |
| Rapport de santé | ✅ Production | ❌ Contribution |
| Détection d'anomalies | ✅ Autorité | ✅ Source d'information |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│ Border Guard│  État des frontières │Caring Nanny │
│             │ ──────────────────► │             │
│             │                      │             │
│ (signale)   │                      │  RAPPORT    │
└─────────────┘                      └─────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| BG → CN | Changement d'état d'une frontière | `BoundaryStateChange` |
| BG → CN | Intégration défaillante | `IntegrationFailure` |
| BG → CN | Passage d'une source vers "hostile" | `HostileDetection` |
| BG → CN | Indicateurs de santé des frontières | `BoundaryHealthMetrics` |

**Indicateurs fournis par Border Guard :**

| Indicateur | Description | Impact sur la santé |
|------------|-------------|---------------------|
| `hostile_detections` | Nombre de sources passées à "hostile" | Risque de sécurité |
| `unknown_sources_ratio` | Ratio de sources non classifiées | Couverture de classification |
| `integration_failures` | Intégrations défaillantes | Connectivité externe |
| `closed_boundaries` | Frontières fermées | État de verrouillage |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-CN-1** | Border Guard notifie Caring Nanny de tout changement d'état significatif |
| **COL-CN-2** | Caring Nanny intègre ces informations dans son rapport de santé |
| **COL-CN-3** | Border Guard ne demande jamais à Caring Nanny de modifier un état |
| **COL-CN-4** | La fréquence de notification est définie par Border Guard |

**Référence Documentation Fondatrice :** Section 3.4 (Relation avec Caring Nanny) et Section 8.3 (Flux d'état vers Caring Nanny)

---

### 4.5 Relation avec Ever Buddy

**Type de relation :** Normative (Ever Buddy → Border Guard)

**Principe fondamental :**

> Ever Buddy définit les règles de compatibilité et d'évolution. Border Guard applique ces règles aux frontières pour les intégrations et les versions supportées.

**Responsabilités respectives :**

| Aspect | Ever Buddy | Border Guard |
|--------|------------|--------------|
| Règles de compatibilité | ✅ Définition | ❌ Consommateur |
| Versions supportées aux frontières | ✅ Définition | ✅ Application |
| Vérification d'intégration | ✅ Critères | ✅ Contexte de frontière |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│ Ever Buddy  │  Règles compatibilité│ Border Guard│
│             │ ──────────────────► │             │
│             │                      │             │
│             │  Rejets incompatib.  │             │
│             │ ◄────────────────── │             │
│             │                      │             │
│  (gouverne) │                      │ (applique)  │
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
| **COL-EB-1** | Ever Buddy définit les versions acceptables aux frontières |
| **COL-EB-2** | Border Guard intègre ces règles dans les conditions de franchissement |
| **COL-EB-3** | Border Guard notifie Ever Buddy des rejets pour incompatibilité |
| **COL-EB-4** | Les fenêtres de compatibilité sont non négociables |

---

### 4.6 Relation avec Master Butler

**Type de relation :** Consultation

**Principe fondamental :**

> Master Butler expose les capacités disponibles. Border Guard informe sur le niveau de confiance requis pour accéder à certaines capacités selon leur sensibilité.

**Responsabilités respectives :**

| Aspect | Master Butler | Border Guard |
|--------|---------------|--------------|
| Catalogue des capacités | ✅ Autorité | ❌ Aucune |
| Niveau de confiance requis | ❌ Consommateur | ✅ Définition |
| Exposition des capacités | ✅ Exécution | ❌ Aucune |
| Filtrage selon confiance | ✅ Application | ✅ Règles |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│ Border Guard│  Niveaux requis      │Master Butler│
│             │ ──────────────────► │             │
│             │                      │             │
│             │  Demande contexte    │             │
│             │ ◄────────────────── │             │
│             │                      │             │
│  (définit)  │                      │  (expose)   │
└─────────────┘                      └─────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| MB → BG | Demande de niveau de confiance requis pour capacité | `CapabilityTrustRequest` |
| BG → MB | Niveau de confiance requis | `RequiredTrustLevel` |
| BG → MB | Règles d'accès aux capacités sensibles | `CapabilityAccessRules` |

**Impact sur l'exposition des capacités :**

| Niveau de confiance source | Capacités accessibles |
|---------------------------|----------------------|
| **Trusted** | Toutes les capacités |
| **Verified** | Capacités standard + certaines sensibles |
| **Unknown** | Capacités publiques uniquement |
| **Hostile** | Aucune capacité |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-MB-1** | Master Butler peut consulter Border Guard pour les niveaux requis |
| **COL-MB-2** | Border Guard définit les règles d'accès aux capacités sensibles |
| **COL-MB-3** | Master Butler adapte son exposition selon les règles de Border Guard |
| **COL-MB-4** | Les capacités critiques sont inaccessibles pour les sources "unknown" ou "hostile" |

---

### 4.7 Relation avec TAMR

**Type de relation :** Escalade

**Principe fondamental :**

> TAMR définit quand l'humain intervient. Border Guard signale les situations de frontière qui nécessitent une intervention humaine.

**Responsabilités respectives :**

| Aspect | TAMR | Border Guard |
|--------|------|--------------|
| Points d'intervention humaine | ✅ Autorité | ❌ Aucune |
| Signalement de besoin d'intervention | ❌ Destinataire | ✅ Émetteur |
| Validation humaine des classifications | ✅ Exécution | ❌ Aucune |
| Passage vers "hostile" manuel | ✅ Validation finale | ✅ Proposition |

**Flux d'interaction :**

```
┌─────────────┐                      ┌─────────────┐
│ Border Guard│  Besoin intervention │    TAMR     │
│             │ ──────────────────► │             │
│             │                      │             │
│             │  Validation humaine  │             │
│             │ ◄────────────────── │             │
│             │                      │             │
│ (signale)   │                      │ (valide)    │
└─────────────┘                      └─────────────┘
```

**Contrat d'interface :**

| Direction | Données échangées | Format |
|-----------|-------------------|--------|
| BG → TAMR | Demande de classification manuelle | `ManualClassificationRequest` |
| BG → TAMR | Signalement de source suspecte | `SuspiciousSourceAlert` |
| TAMR → BG | Validation de classification | `HumanClassificationValidation` |
| TAMR → BG | Refus avec justification | `HumanRejection` |

**Cas nécessitant une escalade vers TAMR :**

| Cas | Description | Sévérité |
|-----|-------------|----------|
| Classification ambiguë | Source difficile à classifier automatiquement | Moyenne |
| Passage vers "hostile" | Confirmation humaine avant blacklist | Élevée |
| Révocation d'intégration | Décision de révoquer une intégration | Élevée |
| Nouvelle intégration critique | Intégration avec un système externe sensible | Critique |
| Modification de frontière FONDATION | Changement de frontière critique | Critique |

**Règles de collaboration :**

| ID | Règle |
|----|-------|
| **COL-TAMR-1** | Border Guard signale automatiquement les cas d'escalade à TAMR |
| **COL-TAMR-2** | TAMR peut valider ou refuser une classification proposée |
| **COL-TAMR-3** | Une classification refusée par TAMR ne peut être forcée par Border Guard |
| **COL-TAMR-4** | La validation TAMR est enregistrée dans l'historique (traçabilité) |

---

## 5. Flux d'interaction transversaux

### 5.1 Flux de contexte de confiance

Ce flux décrit comment le contexte de confiance circule de Border Guard vers les consommateurs.

**Séquence :**

```
1. Source externe / Interaction
   ┌─────────────┐
   │ Source      │
   │ externe     │
   └──────┬──────┘
          │
          ▼
2. Classification par Border Guard
   ┌─────────────┐
   │ Border Guard│
   │ (classifie) │
   └──────┬──────┘
          │
          ├─────────────────┬─────────────────┐
          ▼                 ▼                 ▼
3. Distribution aux consommateurs
   ┌─────────────┐  ┌───────────────┐  ┌─────────────┐
   │StrongFather │  │ BondingBrother│  │Caring Nanny │
   │ (décide)    │  │  (applique)   │  │ (observe)   │
   └─────────────┘  └───────────────┘  └─────────────┘
```

**Données du contexte :**

| Donnée | Consommateur | Usage |
|--------|--------------|-------|
| `trust_level` | StrongFather | Facteur de décision |
| `trust_level` | BondingBrother | Règles de filtrage |
| `trust_level` | Caring Nanny | Indicateur de santé |
| `crossed_boundaries` | StrongFather | Évaluation du risque |
| `applicable_rules` | BondingBrother | Application |
| `boundary_state` | Caring Nanny | État global |

### 5.2 Flux de définition de frontière

Ce flux décrit comment une nouvelle frontière est définie et propagée.

**Séquence :**

```
1. Identification du besoin
   ┌─────────────────────────────────────────┐
   │ Nouveau besoin de frontière détecté     │
   │ (architecture, nouvelle intégration...) │
   └───────────────────┬─────────────────────┘
                       │
                       ▼
2. Définition par Border Guard
   ┌─────────────────────────────────────────┐
   │ Border Guard définit :                  │
   │ - Frontière (type, direction, perm.)    │
   │ - Règles de franchissement              │
   │ - Niveau de confiance requis            │
   └───────────────────┬─────────────────────┘
                       │
                       ├─────────────────┐
                       ▼                 ▼
3. Persistance et propagation
   ┌─────────────┐            ┌───────────────┐
   │ KindMother  │            │ BondingBrother│
   │ (persiste)  │            │ (reçoit règles)│
   └─────────────┘            └───────────────┘
```

### 5.3 Flux de détection hostile

Ce flux décrit comment une source est identifiée comme hostile.

**Séquence :**

```
1. Détection de pattern malveillant
   ┌─────────────────────────────────────────┐
   │ Pattern d'attaque détecté               │
   │ (via BondingBrother ou Caring Nanny)    │
   └───────────────────┬─────────────────────┘
                       │
                       ▼
2. Proposition de classification hostile
   ┌─────────────────────────────────────────┐
   │ Border Guard propose : source → hostile │
   └───────────────────┬─────────────────────┘
                       │
                       ▼
3. Escalade vers TAMR (si nécessaire)
   ┌─────────────┐                      ┌─────────────┐
   │ Border Guard│  Demande validation  │    TAMR     │
   │             │ ──────────────────► │             │
   │             │                      │             │
   │             │  Validation humaine  │             │
   │             │ ◄────────────────── │             │
   └─────────────┘                      └─────────────┘
                       │
                       ▼
4. Notification
   ┌─────────────────────────────────────────┐
   │ Border Guard notifie :                  │
   │ - BondingBrother (blocage)              │
   │ - Caring Nanny (état)                   │
   │ - StrongFather (contexte)               │
   └─────────────────────────────────────────┘
```

---

## 6. Relation avec les produits

### 6.1 Principe fondamental

**Les produits ne parlent jamais directement à Border Guard.**

Toute interaction passe par BondingBrother qui traduit et filtre les échanges.

```
┌─────────────┐                                    ┌─────────────┐
│  Produits   │ ──────────────────────────────────► │ Border Guard│
│             │              ❌ INTERDIT            │             │
└─────────────┘                                    └─────────────┘

┌─────────────┐    via     ┌───────────────┐       ┌─────────────┐
│  Produits   │ ─────────► │BondingBrother │ ────► │ Border Guard│
│             │            │               │       │             │
└─────────────┘            └───────────────┘       └─────────────┘
               ✅ AUTORISÉ
```

### 6.2 Ce que les produits peuvent demander (via BondingBrother)

| Demande | Réponse de Border Guard |
|---------|------------------------|
| "Quelle est ma classification ?" | Niveau de confiance actuel |
| "Puis-je accéder à X ?" | Niveau de confiance requis pour X |
| "L'intégration Y est-elle active ?" | État de l'intégration |

### 6.3 Ce que les produits reçoivent (via BondingBrother)

| Type | Description |
|------|-------------|
| Niveau de confiance | Classification actuelle du produit |
| Règles applicables | Règles de franchissement qui s'appliquent |
| Alertes | Notifications de changement de classification |

---

## 7. Diagramme d'interaction globale

```
                              ┌─────────────────────────────────────────────┐
                              │                BORDER GUARD                  │
                              │  ┌───────────┐  ┌───────────┐  ┌───────────┐ │
                              │  │ Registre  │  │Classificat│  │ Règles    │ │
                              │  │ frontières│  │eur confian│  │ franchis. │ │
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
│(complémentaire)│                            │   (conseil)   │                            │(déf./applic.) │
└───────────────┘                            └───────────────┘                            └───────┬───────┘
                                                                                                  │
        ┌─────────────────────────────────────────────┬─────────────────────────────────┐         │
        │                                             │                                 │         │
        ▼                                             ▼                                 ▼         ▼
┌───────────────┐                            ┌───────────────┐                  ┌───────────────────────┐
│ Caring Nanny  │                            │  Ever Buddy   │                  │      PRODUITS         │
│ (information) │                            │  (normatif)   │                  │ (via BondingBrother)  │
└───────────────┘                            └───────────────┘                  └───────────────────────┘

        ┌─────────────────────────────────────────────┬─────────────────────────────────┐
        │                                             │                                 │
        ▼                                             ▼                                 ▼
┌───────────────┐                            ┌───────────────┐                  
│ Master Butler │                            │     TAMR      │                  
│(consultation) │                            │  (escalade)   │                  
└───────────────┘                            └───────────────┘                  
```

---

## 8. Synthèse des contrats d'interface

### 8.1 Matrice des interactions

| Core | Direction | Nature | Données échangées |
|------|-----------|--------|-------------------|
| **StrongFather** | SF → BG → SF | Conseil | Demande contexte ↔ Contexte confiance |
| **KindMother** | BG → KM | Complémentaire | Définitions à persister |
| **BondingBrother** | BB ↔ BG | Définition/Application | Demande règles ↔ Règles à appliquer |
| **Caring Nanny** | BG → CN | Information | État frontières (unidirectionnel) |
| **Ever Buddy** | EB → BG | Normative | Règles compatibilité (unidirectionnel) |
| **Master Butler** | MB → BG | Consultation | Demande niveaux requis |
| **TAMR** | BG ↔ TAMR | Escalade | Besoin validation ↔ Validation humaine |

### 8.2 Garanties de service

| Garantie | Valeur | Condition |
|----------|--------|-----------|
| Temps de réponse consultation | < 50ms | État système normal |
| Disponibilité des définitions | 99.9% | Hors maintenance |
| Non-blocage des flux | 100% | Invariant structural |
| Traçabilité des interactions | 100% | Invariant INV-BG-8 |

---

## 9. Conformité aux Lois d'Autonomie

### 9.1 LOI-1 : Aucune dépendance externe critique

Toutes les interactions sont locales. Border Guard n'a pas besoin de service externe pour interagir avec les autres cores.

### 9.2 LOI-2 : Le système accepte l'isolement

En mode isolé, Border Guard continue d'interagir avec les cores locaux. Les frontières sont définies localement.

### 9.3 LOI-6 : L'autonomie n'empêche pas la fédération

Les informations de frontière peuvent être partagées entre COG via BondingBrother, avec validation explicite de Border Guard.

---

## 10. Références

### Documents fondateurs

- [Border Guard - Documentation Fondatrice](../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

### Contrats associés

- [Border Guard - Architecture & Flows](./Border%20Guard%20-%20Architecture%20&%20Flows.md)

### Documents de référence

- [Miyukini Conceptual References - Security Protocols](../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Contrat normatif — ARCHITECTURE  
**Référence :** Border Guard - Documentation Fondatrice v1.5, Sections 3 et 8
