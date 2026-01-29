# WorrySentinel - Vocabulary & Glossary

## 1. Contexte

Ce document définit le **vocabulaire canonique** de WorrySentinel. Il établit les définitions officielles des termes utilisés dans la documentation, garantissant une compréhension uniforme et non ambiguë.

**Document fondateur :** [WorrySentinel - Documentation Fondatrice](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md)

**Statut :** Ce document est **normatif**. Les définitions sont officielles et doivent être utilisées de manière cohérente dans toute la documentation WorrySentinel.

---

## 2. Termes fondamentaux

### 2.1 WorrySentinel

**Définition canonique :**

> WorrySentinel est le **core de gouvernance de sécurité transversale** du Miyukini Core System. Il gouverne les niveaux de sécurité, les états de confiance, et la dégradation progressive, sans jamais posséder d'autorité sur l'implémentation, l'exécution, ou la persistance.

**Caractéristiques :**
- Core de gouvernance, pas fonctionnel
- Pression verticale, pas brique horizontale
- Strate 4 dans la Pyramide Miyukini
- Gouverne sans exécuter

**Ne pas confondre avec :**
- Un système de sécurité (WorrySentinel ne fait pas de sécurité, il la gouverne)
- Un contrôleur (WorrySentinel ne contrôle pas, il contraint)
- Un exécuteur (WorrySentinel ne réalise pas d'action)

---

### 2.2 Gouvernance

**Définition canonique :**

> La **gouvernance** est l'action de définir des règles, des contraintes, et des niveaux qui influencent le comportement des cores fonctionnels, sans jamais implémenter ou exécuter ces règles directement.

**Distinction clé :**

| Gouvernance (WorrySentinel) | Implémentation (Cores fonctionnels) |
|-----------------------------|-------------------------------------|
| Définit les règles | Applique les règles |
| Établit les contraintes | Exécute selon les contraintes |
| Déclare les états | Réagit aux états déclarés |
| Ne possède pas de logique technique | Possède la logique technique |

**Invariant associé :** INV-GOV-7 (Séparation gouvernance/implémentation)

---

### 2.3 Pression verticale

**Définition canonique :**

> La **pression verticale** est la capacité de WorrySentinel à contraindre tous les cores fonctionnels de manière transversale, traversant toutes les couches de l'architecture sans appartenir à aucune.

**Visualisation :**

```
┌─────────────────────────────────────────────────────┐
│ Cores fonctionnels (Strate 5)                        │
│ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐    │
│ │ SF      │ │ KM      │ │ MB      │ │ BG      │    │
│ └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘    │
│      │          │          │          │            │
│      ▼          ▼          ▼          ▼            │
│   ═══════════════════════════════════════════       │
│              PRESSION WORRYSENTINEL                 │
│   ═══════════════════════════════════════════       │
│                                                      │
└─────────────────────────────────────────────────────┘
```

**Caractéristiques :**
- Traverse toutes les couches
- N'appartient à aucune couche spécifique
- Contraint sans remplacer
- Non négociable

---

## 3. Niveaux de sécurité

### 3.1 Niveau de sécurité

**Définition canonique :**

> Un **niveau de sécurité** est un profil de risque (0-4) attribué à un produit ou composant, caractérisant le degré de sensibilité des données et déterminant les contraintes de sécurité applicables.

**Les cinq niveaux :**

| Niveau | Code | Désignation | Profil de risque |
|--------|------|-------------|------------------|
| **0** | N0 | Public / Display | Minimal |
| **1** | N1 | Standard / CMS | Faible |
| **2** | N2 | Sensitive Data | Modéré |
| **3** | N3 | Critical System | Élevé |
| **4** | N4 | Hardened / Isolated | Maximal |

**Invariant associé :** INV-GOV-1 (Niveaux de sécurité explicites)

---

### 3.2 Niveau 0 — Public

**Définition canonique :**

> Le **niveau 0 (Public)** caractérise les produits et composants manipulant des données publiques, sans sensibilité, et pour lesquels aucune contrainte de sécurité stricte n'est requise.

**Principe directeur :** *"Si ça casse, ce n'est pas grave."*

**Cas d'usage :** Site vitrine, dashboards en lecture seule, affichage public.

---

### 3.3 Niveau 1 — Standard

**Définition canonique :**

> Le **niveau 1 (Standard)** caractérise les produits et composants manipulant des données standard, avec une sensibilité faible, nécessitant des contraintes de sécurité de base.

**Principe directeur :** *"On protège l'accès, pas le système."*

**Cas d'usage :** CMS, backoffice simple, contenu éditorial.

---

### 3.4 Niveau 2 — Sensitive Data

**Définition canonique :**

> Le **niveau 2 (Sensitive Data)** caractérise les produits et composants manipulant des données sensibles nécessitant une protection renforcée.

**Principe directeur :** *"On protège les données."*

**Cas d'usage :** Données personnelles, comptes utilisateurs, profils.

---

### 3.5 Niveau 3 — Critical System

**Définition canonique :**

> Le **niveau 3 (Critical System)** caractérise les produits et composants manipulant des données critiques nécessitant une protection maximale.

**Principe directeur :** *"On protège le système avant l'UX."*

**Cas d'usage :** Authentification, paiement, autorisations, cores internes.

---

### 3.6 Niveau 4 — Hardened

**Définition canonique :**

> Le **niveau 4 (Hardened / Isolated)** caractérise les produits et composants nécessitant une sécurité maximale, avec des contraintes absolues.

**Principe directeur :** *"On protège l'intégrité coûte que coûte."*

**Cas d'usage :** Environnement isolé, hardware non fiable, contexte hostile.

---

## 4. États de confiance

### 4.1 État de confiance

**Définition canonique :**

> Un **état de confiance** est un niveau d'intégrité (T0-T4) caractérisant la santé globale du système à un instant donné, déterminant les capacités disponibles et les restrictions applicables.

**Les cinq états :**

| État | Code | Désignation | Signification |
|------|------|-------------|---------------|
| **Normal** | T0 | Nominal | Système sain |
| **Instable** | T1 | Doute | Anomalie détectée |
| **Dégradé** | T2 | Suspect | Incohérence persistante |
| **Restreint** | T3 | Critique | Suspicion forte |
| **Bloqué** | T4 | Compromis | Intégrité rompue |

**Invariant associé :** INV-GOV-2 (États de confiance uniques)

---

### 4.2 T0 — Normal

**Définition canonique :**

> L'état **T0 (Normal)** indique un système sain, sans anomalie détectée, où toutes les capacités sont disponibles et le monitoring est standard.

**Symbole :** 🟢

**Capacités :** Toutes les capacités disponibles, décisions normales, extensions dynamiques autorisées.

---

### 4.3 T1 — Instable

**Définition canonique :**

> L'état **T1 (Instable)** indique qu'une anomalie a été détectée mais pas encore confirmée, nécessitant un log renforcé et une traçabilité étendue sans blocage.

**Symbole :** 🟡

**Capacités :** Log renforcé, traçabilité étendue, aucun blocage, surveillance accrue.

---

### 4.4 T2 — Dégradé

**Définition canonique :**

> L'état **T2 (Dégradé)** indique une incohérence persistante nécessitant la désactivation de certaines capacités et des décisions plus strictes.

**Symbole :** 🟠

**Capacités :** Certaines capacités désactivées, refus des extensions dynamiques, monitoring visible.

---

### 4.5 T3 — Restreint

**Définition canonique :**

> L'état **T3 (Restreint)** indique une suspicion forte d'intégrité potentiellement compromise, nécessitant le gel des produits non essentiels et l'intervention TAMR.

**Symbole :** 🔴

**Capacités :** Gel des produits non essentiels, refus de nouveaux modules, TAMR requis pour override.

---

### 4.6 T4 — Bloqué

**Définition canonique :**

> L'état **T4 (Bloqué)** indique que l'intégrité du système est rompue, nécessitant l'arrêt de toute décision opérationnelle et ne permettant que les diagnostics.

**Symbole :** ⛔

**Capacités :** Uniquement diagnostics, état lisible, sortie propre possible.

**Caractéristique :** État terminal — aucune transition sortante.

---

## 5. Dégradation progressive

### 5.1 Dégradation progressive

**Définition canonique :**

> La **dégradation progressive** est le mécanisme par lequel WorrySentinel orchestre la réduction contrôlée des capacités du système selon l'évolution de l'état de confiance, garantissant qu'aucun blocage brutal ne se produise.

**Principe fondamental :**

> *"Un système autonome ne bloque jamais brutalement. Il observe, interprète, dégrade, puis bloque seulement quand il est sûr."*

**Invariant associé :** INV-GOV-4 (Dégradation progressive uniquement)

---

### 5.2 Transition d'état

**Définition canonique :**

> Une **transition d'état** est le passage d'un état de confiance à un autre, gouverné par des règles explicites et toujours justifié et tracé.

**Transitions autorisées :**

| De | Vers | Condition |
|----|------|-----------|
| T0 | T1 | Détection d'anomalie |
| T1 | T0 | Résolution d'anomalie |
| T1 | T2 | Persistance d'anomalie |
| T2 | T1 | Amélioration de l'état |
| T2 | T3 | Aggravation de l'état |
| T3 | T2 | Confirmation de sécurité |
| T3 | T4 | Confirmation de compromission |

**Règle :** Les transitions directes (ex: T0→T4) sont interdites.

**Invariant associé :** INV-GOV-3 (Transitions justifiées)

---

## 6. Invariants

### 6.1 Invariant

**Définition canonique :**

> Un **invariant** est une règle absolue qui ne peut jamais être violée, quel que soit le contexte, la situation, ou les considérations pratiques.

**Caractéristiques d'un invariant :**
- Ne peut jamais être violé
- Est vérifiable
- Est indépendant du contexte
- Est non négociable

**Conséquence de violation :** Faute architecturale fondamentale.

---

### 6.2 Invariants WorrySentinel (INV-WS)

| Code | Énoncé court |
|------|--------------|
| **INV-WS-1** | Aucune autorité sur l'implémentation |
| **INV-WS-2** | Aucune autorité sur l'exécution |
| **INV-WS-3** | Aucune autorité sur la persistance |
| **INV-WS-4** | Aucune modification d'état |
| **INV-WS-5** | Aucune logique temporelle technique |
| **INV-WS-6** | Zero-trust |
| **INV-WS-7** | Gouvernance explicite |
| **INV-WS-8** | Traçabilité complète |

---

### 6.3 Invariants de gouvernance (INV-GOV)

| Code | Énoncé court |
|------|--------------|
| **INV-GOV-1** | Niveaux de sécurité explicites |
| **INV-GOV-2** | États de confiance uniques |
| **INV-GOV-3** | Transitions justifiées |
| **INV-GOV-4** | Dégradation progressive uniquement |
| **INV-GOV-5** | Préservation des invariants |
| **INV-GOV-6** | Cohérence inter-composants |
| **INV-GOV-7** | Séparation gouvernance/implémentation |
| **INV-GOV-8** | Traçabilité complète de gouvernance |

---

## 7. Concepts complémentaires

### 7.1 Zero-trust

**Définition canonique :**

> Le **zero-trust** est le principe selon lequel WorrySentinel ne fait confiance à aucun appelant et évalue chaque demande selon les règles, sans présupposer la validité, l'authenticité, ou la légitimité.

**Invariant associé :** INV-WS-6

---

### 7.2 Contrainte

**Définition canonique :**

> Une **contrainte** est une règle imposée par WorrySentinel aux cores fonctionnels, définissant les limites de leur comportement selon le niveau de sécurité et l'état de confiance.

**Types de contraintes :**
- Contraintes de sévérité (StrongFather)
- Contraintes de permissions (MasterButler)
- Contraintes de frontières (BorderGuard)
- Contraintes de monitoring (CaringNanny)
- Contraintes de ressources (LogisticsSteward)

---

### 7.3 Signal d'intégrité

**Définition canonique :**

> Un **signal d'intégrité** est une information remontée par un core fonctionnel vers WorrySentinel, indiquant une anomalie, une incohérence, ou un état particulier du système.

**Sources de signaux :**
- Kernel (signaux clock, id, traces)
- StrongFather (décisions refusées)
- BorderGuard (anomalies I/O)
- CaringNanny (anomalies monitoring)
- KindMother (incohérences données)
- LogisticsSteward (dérives allocation)

---

### 7.4 Corrélation

**Définition canonique :**

> La **corrélation** est l'action de WorrySentinel de croiser et analyser les signaux d'intégrité provenant de multiples sources pour déterminer l'état de confiance global du système.

**Processus :**
1. Réception des signaux
2. Analyse de cohérence
3. Détection de patterns
4. Déclaration d'état

---

### 7.5 Traçabilité

**Définition canonique :**

> La **traçabilité** est la capacité de retracer toute décision de gouvernance avec son contexte, ses règles appliquées, et sa justification.

**Éléments de traçabilité obligatoires :**
- Contexte de la décision
- Règles appliquées
- Justification
- Niveau de sécurité
- État de confiance
- Résultat

**Invariants associés :** INV-WS-8, INV-GOV-8

---

### 7.6 Médiation

**Définition canonique :**

> La **médiation** est le processus de validation gouverné par WorrySentinel permettant à un composant d'accéder à un niveau de sécurité supérieur au sien.

**Règle :** Un composant de niveau N ne peut pas accéder directement à un composant de niveau > N sans médiation explicite.

**Invariant associé :** INV-GOV-6

---

## 8. Termes à ne pas confondre

### 8.1 Niveau de sécurité vs État de confiance

| Aspect | Niveau de sécurité (0-4) | État de confiance (T0-T4) |
|--------|--------------------------|---------------------------|
| **Nature** | Profil de risque | État d'intégrité |
| **Portée** | Produit/composant | Système global |
| **Stabilité** | Statique (pendant opération) | Dynamique |
| **Déterminé par** | Profil du produit | Signaux d'intégrité |

### 8.2 Gouvernance vs Implémentation

| Aspect | Gouvernance | Implémentation |
|--------|-------------|----------------|
| **Responsable** | WorrySentinel | Cores fonctionnels |
| **Action** | Définir les règles | Appliquer les règles |
| **Nature** | Déclarative | Impérative |
| **Exécution** | Jamais | Toujours |

### 8.3 Contrainte vs Contrôle

| Aspect | Contrainte | Contrôle |
|--------|------------|----------|
| **Source** | WorrySentinel | Cores fonctionnels |
| **Nature** | Déclarative | Exécutive |
| **Action** | Limite le comportement | Vérifie et applique |

---

## 9. Références croisées

| Document | Relation |
|----------|----------|
| [WorrySentinel - Documentation Fondatrice](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md) | Définitions officielles |
| [WorrySentinel - Invariants & Guarantees](../contracts/governance/WorrySentinel%20-%20Invariants%20&%20Guarantees.md) | Invariants détaillés |
| [Miyukini Conceptual References - Security Levels](../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) | Niveaux de sécurité complets |
| [Miyukini Conceptual References - Integrity Degradation System](../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md) | États de confiance complets |
| [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Glossaire global Miyukini |

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Normatif — Vocabulaire officiel  
**Type :** Glossaire et vocabulaire
