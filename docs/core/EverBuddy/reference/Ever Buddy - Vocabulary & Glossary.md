# Ever Buddy - Vocabulary & Glossary

## Contexte

Ce document constitue le **vocabulaire canonique** d'Ever Buddy, le core de cycle de vie et d'évolution du Miyukini Core System (Strate 4). Il regroupe toutes les définitions officielles des termes utilisés dans le domaine d'Ever Buddy.

**Document de référence :** [Ever Buddy - Documentation Fondatrice](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)

**Glossaire général :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

## Portée / Scope

- **Applicable à :** Toute documentation Ever Buddy, communications, implémentations
- **Audience :** Architectes, développeurs, intégrateurs, équipes documentation
- **Statut :** Document de référence normatif — Vocabulaire canonique Ever Buddy

---

## États de cycle de vie

### DRAFT (BROUILLON)

État d'un élément en cours de définition. Il n'est pas encore utilisable en production, peut changer librement, et n'a aucun engagement de stabilité.

**Caractéristiques :**
- Non utilisable en production
- Changements libres autorisés
- Aucune garantie de stabilité

**Transitions possibles :** DRAFT → ACTIVE, DRAFT → ARCHIVED

**Référence glossaire :** [BROUILLON (DRAFT)](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#brouillon-draft--état-de-vie)

---

### ACTIVE (ACTIF)

État d'un élément en usage normal. Il est stable, documenté, supporté, et utilisable par tous les consommateurs autorisés. Les changements sont soumis aux règles de compatibilité.

**Caractéristiques :**
- Stable et documenté
- Supporté activement
- Changements soumis aux règles de compatibilité
- Utilisable par tous les consommateurs autorisés

**Transitions possibles :** ACTIVE → DEPRECATED

**Référence glossaire :** [ACTIF (ACTIVE)](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#actif-active--état-de-vie)

---

### DEPRECATED (DÉPRÉCIÉ)

État d'un élément toujours fonctionnel mais dont l'usage est découragé. Un successeur existe ou est en préparation. Les consommateurs sont avertis de migrer. La période de dépréciation est définie et communiquée.

**Caractéristiques :**
- Toujours fonctionnel
- Usage découragé
- Successeur identifié (ou annoncé comme "sans successeur")
- Période de dépréciation définie

**Transitions possibles :** DEPRECATED → RETIRED, DEPRECATED → ACTIVE (réactivation exceptionnelle)

**Référence glossaire :** [DÉPRÉCIÉ (DEPRECATED)](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#déprécié-deprecated--état-de-vie)

---

### RETIRED (RETIRÉ)

État d'un élément qui n'est plus activement supporté mais reste fonctionnel pour les consommateurs existants. Aucune nouvelle fonctionnalité n'est ajoutée, uniquement des corrections critiques de sécurité.

**Caractéristiques :**
- Plus activement supporté
- Fonctionnel pour consommateurs existants
- Uniquement corrections critiques de sécurité
- Aucune nouvelle fonctionnalité

**Transitions possibles :** RETIRED → ARCHIVED

**Référence glossaire :** [RETIRÉ (RETIRED)](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#retiré-retired--état-de-vie)

---

### ARCHIVED (ARCHIVÉ)

État d'un élément qui n'est plus fonctionnel. Il est conservé uniquement pour référence historique et traçabilité. Aucune garantie de fonctionnement.

**Caractéristiques :**
- Non fonctionnel
- Conservation pour référence historique uniquement
- Aucune garantie de fonctionnement
- Traçabilité préservée

**Transitions possibles :** Aucune (état terminal)

---

## Concepts de transition

### Transition

Une **transition** est le passage d'un état de cycle de vie à un autre. Les transitions sont atomiques, documentées, et validées par Ever Buddy.

**Caractéristiques :**
- Atomique (pas d'état intermédiaire)
- Documentée obligatoirement
- Validée par Ever Buddy
- Enregistrée dans l'historique immuable

**Invariant associé :** INV-EB-3 (Aucun état ambigu)

---

### Évolution contrôlée

Une **évolution contrôlée** est un changement structurel qui respecte les principes de continuité et de compatibilité.

**Composantes obligatoires :**

| Composante | Description |
|------------|-------------|
| **Annonce préalable** | L'évolution est communiquée avant sa mise en œuvre |
| **Période de transition** | Durée pendant laquelle l'ancien et le nouveau coexistent |
| **Documentation des différences** | Les changements sont explicitement documentés |
| **Chemin de migration** | Guide pour passer de l'ancien au nouveau |
| **Critères de complétion** | Conditions claires définissant la fin de transition |

---

### Coexistence

La **coexistence** est la période pendant laquelle deux versions (ou plus) d'un élément sont simultanément disponibles. Cette période permet aux consommateurs de migrer progressivement.

**Règles :**
- L'ancienne et la nouvelle version sont toutes deux fonctionnelles
- Les consommateurs choisissent leur rythme de migration
- La période a une durée définie et communiquée

---

### Sunset

Le **sunset** est le processus planifié de fin de vie d'un élément. Il comprend la séquence complète de retrait.

**Séquence du sunset :**
1. Dépréciation (ACTIVE → DEPRECATED)
2. Période de transition (coexistence)
3. Retirement (DEPRECATED → RETIRED)
4. Période de grâce (optionnelle)
5. Archivage (RETIRED → ARCHIVED)

---

### Période de grâce (Grace period)

La **période de grâce** est le temps supplémentaire accordé après la date prévue de retirement, pour permettre aux consommateurs retardataires de migrer. Cette période est accordée au cas par cas.

**Caractéristiques :**
- Accordée exceptionnellement
- Durée variable selon l'impact
- Ne suspend pas le statut RETIRED

---

## Concepts de compatibilité

### Rétrocompatibilité

Un élément est **rétrocompatible** quand le nouveau fonctionne avec l'ancien. Les consommateurs existants continuent de fonctionner sans modification.

**Invariant associé :** INV-EB-5 (Rétrocompatibilité par défaut)

**Règle :** Toute évolution est présumée rétrocompatible sauf déclaration explicite contraire.

---

### Compatibilité en amont

Un élément est **compatible en amont** quand l'ancien fonctionne avec le nouveau. Les nouvelles fonctionnalités sont accessibles aux anciennes versions.

**Note :** La compatibilité en amont est rare et souvent impossible à garantir.

---

### Incompatibilité

Un élément est **incompatible** quand le nouveau ne fonctionne pas avec l'ancien. Une migration est obligatoire.

**Conséquences :**
- Transition de version majeure requise
- Période de dépréciation obligatoire (INV-EB-4)
- Documentation explicite du breaking change

---

### Breaking change

Un **breaking change** est un changement qui rompt la compatibilité avec les versions précédentes.

**Exigences :**
- Transition de version majeure
- Période de dépréciation de l'ancienne version
- Justification documentée
- Plan de transition fourni

---

### Fenêtre de compatibilité (Compatibility window)

La **fenêtre de compatibilité** est la plage de versions avec lesquelles un élément garantit la compatibilité.

**Exemple :** "Compatible avec v2.0 à v2.4" définit une fenêtre de compatibilité de 5 versions mineures.

**Usage :** Permet aux consommateurs de planifier leurs propres évolutions.

---

## Concepts de versionnement

### Versionnement sémantique

Le **versionnement sémantique** est la manière dont Ever Buddy identifie et distingue les différentes versions d'un élément.

**Structure : MAJEUR.MINEUR.CORRECTIF**

| Niveau | Signification | Impact compatibilité |
|--------|---------------|---------------------|
| **Majeur** | Changement incompatible, rupture de contrat | Breaking change |
| **Mineur** | Ajout de fonctionnalité | Rétrocompatible |
| **Correctif** | Correction de bug | Aucun changement fonctionnel |

---

### Génération

Une **génération** est une version majeure d'un élément ou d'un ensemble d'éléments qui partagent une base conceptuelle commune.

**Caractéristiques :**
- Numérotée (1.x, 2.x, 3.x...)
- Traçable dans l'historique
- Base conceptuelle distincte de la génération précédente

**Invariant associé :** INV-EB-6 (Vision long terme obligatoire — considérer au moins deux générations)

---

### Chaîne d'évolution (Evolution chain)

La **chaîne d'évolution** est la séquence complète des versions d'un élément, de sa création à son état actuel. Elle inclut tous les prédécesseurs et successeurs.

**Contenu :**
- Toutes les versions depuis la création
- Toutes les transitions enregistrées
- Tous les successeurs et prédécesseurs

---

## Concepts de succession

### Successeur

Un **successeur** est l'élément qui remplace un élément déprécié ou retiré. Le successeur peut être une nouvelle version du même élément ou un élément entièrement différent.

**Invariant associé :** INV-EB-10 (Unicité du successeur déclaré)

**Règle :** Un élément déprécié possède au plus un successeur déclaré à tout moment.

---

### Prédécesseur

Un **prédécesseur** est l'élément qui a été remplacé par l'élément actuel. La chaîne des prédécesseurs forme l'historique d'évolution.

---

## Concepts de dette et surveillance

### Dette structurelle

La **dette structurelle** est l'ensemble des éléments DEPRECATED ou RETIRED qui persistent dans le système. Cette dette n'est pas nécessairement négative — elle est le prix de la continuité.

**Surveillance :** Ever Buddy mesure et alerte quand la dette devient excessive.

**Voir aussi :** [Ever Buddy - Debt Tracking Contract](../contracts/observability/Ever%20Buddy%20-%20Debt%20Tracking%20Contract.md)

---

### Debt ratio

Le **debt ratio** est le rapport entre les éléments DEPRECATED/RETIRED et les éléments ACTIVE.

**Formule :** `Debt ratio = (DEPRECATED + RETIRED) / ACTIVE`

**Usage :** Ever Buddy surveille ce ratio et alerte quand il dépasse des seuils définis.

---

### Taux d'adoption (Adoption rate)

Le **taux d'adoption** est le pourcentage de consommateurs qui ont migré vers le successeur d'un élément déprécié.

**Usage :** Ever Buddy surveille ce taux pour déterminer quand une transition peut être complétée.

**Critère de complétion :** Un taux d'adoption suffisant permet le passage de DEPRECATED à RETIRED.

---

## Concepts techniques

### Migration

Une **migration** est l'ensemble des actions nécessaires pour passer d'une version à une autre.

**Règle fondamentale :** Ever Buddy définit les migrations conceptuellement mais **ne les exécute jamais** (INV-EB-1).

**Responsabilité d'exécution :**
- KindMother pour les données
- Produits pour leur code

---

### Freeze (Gel)

Un **freeze** est le gel d'un élément à un état donné. Un élément gelé ne peut plus évoluer (sauf corrections critiques de sécurité).

**Usage :** Stabiliser les versions en production.

**Référence glossaire :** [Gel local (Local Freeze)](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#gel-local-local-freeze)

---

### Tombstone

Un **tombstone** est l'enregistrement minimal conservé pour un élément archivé. Il contient uniquement les métadonnées nécessaires à la traçabilité historique, pas les données fonctionnelles.

**Contenu d'un tombstone :**
- Identifiant de l'élément
- Dates de création et d'archivage
- Chaîne d'évolution (successeur, prédécesseur)
- Raison de l'archivage

---

## Catégories d'éléments

Ever Buddy distingue les éléments par leur catégorie, chaque catégorie ayant des règles d'évolution spécifiques.

### Contrats fondateurs (FONDATION)

**Caractéristiques :**
- Évolution extrêmement lente
- Périodes de transition très longues
- Ruptures quasiment interdites

**Exemples :** Documentations fondatrices des cores, invariants système

---

### Contrats opérationnels

**Caractéristiques :**
- Évolution modérée
- Périodes de transition standards
- Ruptures possibles avec justification

**Exemples :** Contrats d'API, spécifications d'interfaces

---

### Interfaces techniques

**Caractéristiques :**
- Évolution plus rapide
- Périodes de transition courtes
- Ruptures possibles avec documentation

**Exemples :** Interfaces de modules, adaptateurs

---

### Éléments internes

**Caractéristiques :**
- Évolution libre
- Pas de garantie de stabilité externe
- Ruptures sans préavis autorisées

**Exemples :** Implémentations internes, utilitaires privés

---

## Matrice récapitulative des transitions

| Depuis \ Vers | DRAFT | ACTIVE | DEPRECATED | RETIRED | ARCHIVED |
|---------------|-------|--------|------------|---------|----------|
| **DRAFT**     | —     | ✓      | ✗          | ✗       | ✓        |
| **ACTIVE**    | ✗     | —      | ✓          | ✗       | ✗        |
| **DEPRECATED**| ✗     | ✓*     | —          | ✓       | ✗        |
| **RETIRED**   | ✗     | ✗      | ✗          | —       | ✓        |
| **ARCHIVED**  | ✗     | ✗      | ✗          | ✗       | —        |

*La réactivation DEPRECATED → ACTIVE est possible uniquement si le successeur est annulé et que l'élément déprécié est toujours fonctionnel.

---

## Tableau de correspondance terminologique

| Terme anglais | Terme français | Définition courte |
|---------------|----------------|-------------------|
| Lifecycle | Cycle de vie | Ensemble des états d'un élément |
| Transition | Transition | Passage d'un état à un autre |
| Generation | Génération | Version majeure conceptuelle |
| Coexistence | Coexistence | Période de double disponibilité |
| Sunset | Sunset | Processus planifié de fin de vie |
| Successor | Successeur | Élément de remplacement |
| Predecessor | Prédécesseur | Élément remplacé |
| Breaking change | Rupture de compatibilité | Changement incompatible |
| Migration | Migration | Actions pour changer de version |
| Freeze | Gel | Blocage d'évolution |
| Debt ratio | Ratio de dette | Mesure de dette structurelle |
| Adoption rate | Taux d'adoption | Pourcentage de migration |
| Grace period | Période de grâce | Temps supplémentaire accordé |
| Compatibility window | Fenêtre de compatibilité | Plage de versions compatibles |
| Evolution chain | Chaîne d'évolution | Historique complet des versions |
| Tombstone | Tombstone | Enregistrement minimal archivé |

---

## Références croisées

### Documents Ever Buddy

- [Ever Buddy - Documentation Fondatrice](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) — Section 9 : Vocabulaire canonique
- [Ever Buddy - Lifecycle States Contract](../contracts/lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md) — États détaillés
- [Ever Buddy - Transition Rules Contract](../contracts/lifecycle/Ever%20Buddy%20-%20Transition%20Rules%20Contract.md) — Règles de transition
- [Ever Buddy - Compatibility Rules Contract](../contracts/compatibility/Ever%20Buddy%20-%20Compatibility%20Rules%20Contract.md) — Règles de compatibilité
- [Ever Buddy - Version Semantics Contract](../contracts/compatibility/Ever%20Buddy%20-%20Version%20Semantics%20Contract.md) — Versionnement sémantique
- [Ever Buddy - Debt Tracking Contract](../contracts/observability/Ever%20Buddy%20-%20Debt%20Tracking%20Contract.md) — Surveillance dette
- [Ever Buddy - Invariants & Guarantees](../contracts/governance/Ever%20Buddy%20-%20Invariants%20%26%20Guarantees.md) — INV-EB-1 à INV-EB-12

### Documents de référence Miyukini

- [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) — Glossaire général
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) — Conformité LOI-1 à LOI-6

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Document de référence normatif — Vocabulaire canonique Ever Buddy  
**Source :** Documentation Fondatrice Ever Buddy v1.3, Section 9
