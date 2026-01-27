# StrongFather — Architecture & Flows

## 1. Introduction

### Objet du contrat

Ce document définit le **StrongFather — Architecture & Flows** : un contrat normatif, non négociable, et de statut FONDATION qui établit l'architecture conceptuelle de StrongFather et les flux d'évaluation, définissant comment les composants internes de StrongFather sont organisés et comment les évaluations transitent à travers le système dans le Miyukini Core System v2.4.

Ce contrat précise l'architecture conceptuelle, les composants internes, les flux d'évaluation, et les interactions entre composants.

### Portée

Ce contrat s'applique à **toute l'architecture de StrongFather** et définit de manière absolue :
- l'architecture conceptuelle de StrongFather,
- les composants internes et leurs responsabilités,
- les flux d'évaluation,
- les interactions entre composants,
- les invariants architecturaux.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat **synthétise et illustre** l'architecture définie dans :
- **StrongFather — Documentation Fondatrice** : Positionnement architectural
- **StrongFather — Boundary & Isolation Contract** : Frontières
- **StrongFather — Decision Graph Specification** : Structure des évaluations
- **StrongFather — Intent Model Contract** : Entrées du système
- **StrongFather — Core Decision Contract** : Sorties du système

Ce contrat ne contredit aucun autre contrat et constitue une vue architecturale consolidée.

---

## 2. Architecture conceptuelle

### 2.1. Vue d'ensemble

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              ÉCOSYSTÈME MIYUKINI                         │
│                                                                         │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │                          PRODUIT                                   │  │
│  │                                                                   │  │
│  │   ┌─────────────────────────────────────────────────────────┐    │  │
│  │   │              ADAPTATEUR PRODUIT                          │    │  │
│  │   │                                                         │    │  │
│  │   │   [Intention] ──────────▶ [StrongFather] ──────▶ [Décision]  │  │
│  │   │                                │                        │    │  │
│  │   │                                │                        │    │  │
│  │   │                                ▼                        │    │  │
│  │   │                          [KindMother]                   │    │  │
│  │   │                        (via adaptateur)                 │    │  │
│  │   └─────────────────────────────────────────────────────────┘    │  │
│  │                                                                   │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │                       MODULES SPM CMS                             │  │
│  │                  (traits fonctionnels, isolés)                    │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │                           KERNEL                                   │  │
│  │                     (Id, Clock, Logger)                           │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 2.2. Positionnement de StrongFather

**StrongFather est un moteur interne** :

- Il n'est pas exposé comme API publique directe
- Il n'est pas un module SPM CMS
- Il n'est pas dans le kernel
- Il est utilisé par les adaptateurs produits pour évaluer des intentions

**Dépendances :**

- StrongFather ne dépend d'aucun composant externe pour ses évaluations (conformité à **LOI-1** : aucune dépendance externe critique)
- StrongFather reçoit son contexte des adaptateurs
- StrongFather reçoit ses politiques d'une source configurée

Cette architecture respecte les lois d'autonomie système définies dans [Miyukini Conceptual References - Lois Autonomie Systeme](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md), notamment **LOI-1** (aucune dépendance externe critique) : StrongFather peut démarrer, décider, fonctionner, et être audité sans aucun appel externe obligatoire.

---

## 3. Composants internes de StrongFather

### 3.1. Surface d'évaluation

**Définition :**

La **surface d'évaluation** est le point d'entrée unique de StrongFather. Elle reçoit les intentions et retourne les décisions.

**Responsabilités :**

- Recevoir les intentions des adaptateurs
- Valider la structure des intentions
- Déléguer l'évaluation au moteur de politiques
- Retourner les décisions aux adaptateurs

**Caractéristiques :**

- Point d'entrée unique (pas d'entrées multiples)
- Interface conceptuelle standardisée
- Pas de logique métier

### 3.2. Validateur d'intention

**Définition :**

Le **validateur d'intention** vérifie la validité structurelle des intentions avant l'évaluation des politiques.

**Responsabilités :**

- Vérifier la présence des composants obligatoires
- Vérifier la cohérence structurelle
- Rejeter les intentions structurellement invalides

**Règles appliquées :**

- Intent Model Contract, section 6 (règles de formation)
- Intent Model Contract, section 8 (intentions invalides)

### 3.3. Moteur de politiques

**Définition :**

Le **moteur de politiques** applique les politiques sur les intentions et produit les résultats d'évaluation.

**Responsabilités :**

- Sélectionner les politiques applicables
- Évaluer chaque politique
- Produire les résultats d'évaluation

**Règles appliquées :**

- Policy Engine Contract, section 5 (application des politiques)

### 3.4. Compositeur de résultats

**Définition :**

Le **compositeur de résultats** agrège les résultats des évaluations de politiques selon les règles de composition.

**Responsabilités :**

- Agréger les résultats des politiques
- Appliquer les règles de composition
- Déterminer le résultat global

**Règles appliquées :**

- Policy Engine Contract, section 6 (composition des politiques)

### 3.5. Calculateur de priorité

**Définition :**

Le **calculateur de priorité** établit la priorité relative d'une intention si les politiques sont satisfaites.

**Responsabilités :**

- Appliquer les politiques de priorité
- Calculer la priorité relative
- Fournir la priorité à la décision

**Activation :**

- Activé uniquement si toutes les politiques sont satisfaites

### 3.6. Producteur de décision

**Définition :**

Le **producteur de décision** génère la décision finale à partir des résultats d'évaluation.

**Responsabilités :**

- Produire la décision (ACCEPTÉE, REFUSÉE, AMBIGUË, DIFFÉRÉE)
- Assembler la justification
- Référencer les politiques appliquées

**Règles appliquées :**

- Core Decision Contract, section 3 (types de décisions)
- Core Decision Contract, section 5 (sorties garanties)

### 3.7. Traceur

**Définition :**

Le **traceur** enregistre les traces d'évaluation pour audit et diagnostic.

**Responsabilités :**

- Tracer les intentions reçues
- Tracer les évaluations de politiques
- Tracer les décisions produites
- Tracer les erreurs

**Règles appliquées :**

- Audit & Trace Contract, section 3 (éléments obligatoirement tracés)

---

## 4. Flux d'évaluation

### 4.1. Flux principal

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         FLUX D'ÉVALUATION PRINCIPAL                      │
│                                                                         │
│   [Adaptateur]                                                          │
│        │                                                                │
│        │ Intention                                                      │
│        ▼                                                                │
│   ┌─────────────────────────────────────────────────────────────────┐  │
│   │  1. SURFACE D'ÉVALUATION                                         │  │
│   │     - Réception de l'intention                                   │  │
│   │     - Délégation au validateur                                   │  │
│   └──────────────────────────┬──────────────────────────────────────┘  │
│                              │                                          │
│                              ▼                                          │
│   ┌─────────────────────────────────────────────────────────────────┐  │
│   │  2. VALIDATEUR D'INTENTION                                       │  │
│   │     - Vérification structurelle                                  │  │
│   │     - Si invalide → Décision REFUSÉE (structurel)               │  │
│   └──────────────────────────┬──────────────────────────────────────┘  │
│                              │ (si valide)                              │
│                              ▼                                          │
│   ┌─────────────────────────────────────────────────────────────────┐  │
│   │  3. MOTEUR DE POLITIQUES                                         │  │
│   │     - Sélection des politiques                                   │  │
│   │     - Évaluation de chaque politique                            │  │
│   └──────────────────────────┬──────────────────────────────────────┘  │
│                              │                                          │
│                              ▼                                          │
│   ┌─────────────────────────────────────────────────────────────────┐  │
│   │  4. COMPOSITEUR DE RÉSULTATS                                     │  │
│   │     - Agrégation des résultats                                   │  │
│   │     - Détermination du résultat global                          │  │
│   └──────────────────────────┬──────────────────────────────────────┘  │
│                              │                                          │
│               ┌──────────────┼──────────────┐                          │
│               │              │              │                          │
│               ▼              ▼              ▼                          │
│        [TOUTES_SAT]    [NON_SAT]     [INDÉTERMINÉ]                    │
│               │              │              │                          │
│               ▼              │              │                          │
│   ┌───────────────────┐     │              │                          │
│   │ 5. CALCULATEUR    │     │              │                          │
│   │    DE PRIORITÉ    │     │              │                          │
│   └─────────┬─────────┘     │              │                          │
│             │               │              │                          │
│             ▼               ▼              ▼                          │
│   ┌─────────────────────────────────────────────────────────────────┐  │
│   │  6. PRODUCTEUR DE DÉCISION                                       │  │
│   │     - Production de la décision                                  │  │
│   │     - Assemblage de la justification                            │  │
│   └──────────────────────────┬──────────────────────────────────────┘  │
│                              │                                          │
│                              ▼                                          │
│   [Adaptateur] ◀──────── Décision                                      │
│                                                                         │
│   ══════════════════════════════════════════════════════════════════   │
│   │ TRACEUR (en parallèle)                                           │  │
│   │   - Trace d'intention                                            │  │
│   │   - Traces d'évaluation                                          │  │
│   │   - Trace de décision                                            │  │
│   ══════════════════════════════════════════════════════════════════   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 4.2. Flux de rejet structurel

```
[Intention invalide] → [Validateur] → [Rejet structurel] → [Décision REFUSÉE]
```

**Caractéristiques :**

- Pas d'évaluation de politiques
- Rejet immédiat
- Justification : violation des règles de formation

### 4.3. Flux de rejet de politique

```
[Intention valide] → [Politiques] → [Au moins une NON_SATISFAITE] → [Décision REFUSÉE]
```

**Caractéristiques :**

- Évaluation de toutes les politiques
- Rejet si au moins une politique n'est pas satisfaite
- Justification : politiques violées identifiées

### 4.4. Flux d'ambiguïté

```
[Intention valide] → [Politiques] → [Au moins une INDÉTERMINÉE] → [Décision AMBIGUË]
```

**Caractéristiques :**

- Évaluation de toutes les politiques
- Ambiguïté si au moins une politique est indéterminée
- Clarifications requises identifiées

### 4.5. Flux d'acceptation

```
[Intention valide] → [Politiques] → [TOUTES_SATISFAITES] → [Priorité] → [Décision ACCEPTÉE]
```

**Caractéristiques :**

- Évaluation de toutes les politiques
- Toutes les politiques satisfaites
- Priorité calculée
- Justification : politiques satisfaites

---

## 5. Interactions entre composants

### 5.1. Règles d'interaction

**R-INTER-1 : Flux unidirectionnel**

Le flux d'évaluation est unidirectionnel : de l'entrée vers la sortie.

**R-INTER-2 : Pas de callback**

Aucun composant ne rappelle un composant précédent dans le flux.

**R-INTER-3 : Indépendance du traceur**

Le traceur fonctionne en parallèle sans affecter le flux principal.

**R-INTER-4 : Composition explicite**

Les interactions entre composants sont explicites et documentées.

### 5.2. Dépendances entre composants

```
Surface d'évaluation
        │
        └──▶ Validateur d'intention
                    │
                    └──▶ Moteur de politiques
                                │
                                └──▶ Compositeur de résultats
                                            │
                                            ├──▶ Calculateur de priorité (conditionnel)
                                            │
                                            └──▶ Producteur de décision

Traceur ──▶ (observe tous les composants)
```

---

## 6. Invariants architecturaux

### 6.1. Invariants de structure

**INV-ARCH-1 : Point d'entrée unique**

La surface d'évaluation est le seul point d'entrée de StrongFather.

**INV-ARCH-2 : Point de sortie unique**

Le producteur de décision est le seul point de sortie de StrongFather.

**INV-ARCH-3 : Flux acyclique**

Le flux d'évaluation est acyclique. Aucun composant ne rappelle un composant précédent.

### 6.2. Invariants de comportement

**INV-ARCH-4 : Composants sans état persistant**

Aucun composant ne maintient d'état persistant entre évaluations.

**INV-ARCH-5 : Composants purs**

Tous les composants se comportent comme des fonctions pures.

**INV-ARCH-6 : Traceur isolé**

Le traceur n'affecte jamais le comportement des autres composants.

---

## 7. Règles de fermeture du contrat

### 7.1. Contrat fermé

Ce contrat est **fermé**. Seuls les composants, les flux, et les interactions explicitement définis sont valides.

### 7.2. Interdiction d'extension implicite

Aucun composant, flux, ou interaction non défini n'est autorisé.

---

## 8. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable l'architecture et les flux de StrongFather.

Il garantit que :
- l'architecture est explicitement définie,
- les composants internes sont identifiés et documentés,
- les flux d'évaluation sont formalisés,
- les interactions sont explicites,
- les invariants architecturaux sont maintenus,
- le contrat est fermé et non extensible implicitement.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

## 9. Validation conceptuelle

### 9.1. Cas conformes

Les cas suivants sont **conformes** à ce contrat :

1. **Flux standard** : Une intention traverse tous les composants dans l'ordre défini et produit une décision.

2. **Rejet précoce** : Une intention structurellement invalide est rejetée par le validateur sans atteindre le moteur de politiques.

### 9.2. Cas de violation

Les cas suivants **violent** ce contrat :

1. **Entrée multiple** : Une intention entre directement dans le moteur de politiques sans passer par la surface d'évaluation. Viole INV-ARCH-1.

2. **Callback** : Le producteur de décision rappelle le validateur pour une re-validation. Viole INV-ARCH-3.

3. **État persistant** : Le moteur de politiques mémorise des résultats entre évaluations. Viole INV-ARCH-4.

---

**Document créé le :** 2026-01-25  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, StrongFather Documentation Fondatrice  
**Type :** Architecture et flux non négociables

---

## 10. Mini log de génération

### Décision éditoriale E1 : Composants internes

**Décision prise :** Définition de 7 composants internes (surface, validateur, moteur, compositeur, calculateur, producteur, traceur).

**Application :** Section 3 définit chaque composant avec ses responsabilités.

### Décision éditoriale E2 : Diagrammes ASCII

**Décision prise :** Utilisation de diagrammes ASCII pour illustrer l'architecture et les flux.

**Application :** Sections 2, 4, et 5 contiennent des diagrammes ASCII.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec Documentation Fondatrice : Confirmée (positionnement)
- ✅ Cohérence avec Decision Graph Specification : Confirmée (flux d'évaluation)
- ✅ Cohérence avec Intent Model Contract : Confirmée (entrées)
- ✅ Cohérence avec Core Decision Contract : Confirmée (sorties)
- ✅ Cohérence avec Policy Engine Contract : Confirmée (moteur de politiques)

**Conclusion :** Aucune contradiction détectée.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
