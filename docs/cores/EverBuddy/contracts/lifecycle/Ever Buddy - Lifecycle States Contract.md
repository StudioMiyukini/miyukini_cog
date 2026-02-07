# Ever Buddy - Lifecycle States Contract

## 1. Contexte

Ce document définit le **contrat normatif des états de cycle de vie** gouvernés par Ever Buddy. Les états de cycle de vie sont les fondations de la gouvernance temporelle du système Miyukini. Chaque élément du système (contrat, structure, interface, entité) possède un état de vie qui détermine son statut, ses garanties, et les actions possibles le concernant.

Ce contrat est **dérivé de la Documentation Fondatrice d'Ever Buddy** (Section 4 - Concepts fondamentaux) et constitue la référence normative pour toute implémentation.

**Document source :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)

---

## 2. Portée / Scope

- **Applicable à :** Tous les éléments gouvernés par Ever Buddy (contrats, structures, interfaces, entités, Tools, Toolkits)
- **Audience :** Architectes, développeurs, cores, adaptateurs
- **Statut :** Contrat normatif — Non négociable
- **Dépendances :** Documentation Fondatrice Ever Buddy, Glossaire Miyukini

---

## 3. Définition des états de cycle de vie

Chaque élément du système possède **exactement un** état de cycle de vie à tout moment (INV-EB-3). Les cinq états valides sont définis ci-dessous.

### 3.1 DRAFT (Brouillon)

**Définition canonique :**

> L'élément est en cours de définition. Il n'est pas encore utilisable en production, peut changer librement, et n'a aucun engagement de stabilité.

**Caractéristiques :**

| Propriété | Valeur |
|-----------|--------|
| **Disponibilité production** | ❌ Non |
| **Stabilité garantie** | ❌ Aucune |
| **Changements autorisés** | ✅ Libres, sans contrainte |
| **Consommateurs attendus** | Aucun (développement interne uniquement) |
| **Documentation requise** | Minimale (intention et direction) |
| **Support** | Aucun |

**Comportements :**

- Un élément DRAFT **n'est pas exposé** aux consommateurs externes
- Les modifications sont **libres et non annoncées**
- Aucun engagement de **rétrocompatibilité**
- L'élément peut être **abandonné sans préavis**
- Les tests sont exploratoires, pas de validation formelle requise

**Conditions de sortie :**

- L'élément peut transitionner vers **ACTIVE** quand il est jugé prêt pour la production
- L'élément peut transitionner directement vers **ARCHIVED** s'il est abandonné avant activation

**Référence Glossaire :** [BROUILLON (DRAFT)](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#brouillon-draft--état-de-vie)

---

### 3.2 ACTIVE (Actif)

**Définition canonique :**

> L'élément est en usage normal. Il est stable, documenté, supporté, et utilisable par tous les consommateurs autorisés. Les changements sont soumis aux règles de compatibilité.

**Caractéristiques :**

| Propriété | Valeur |
|-----------|--------|
| **Disponibilité production** | ✅ Oui |
| **Stabilité garantie** | ✅ Complète |
| **Changements autorisés** | Soumis aux règles de compatibilité |
| **Consommateurs attendus** | Tous les consommateurs autorisés |
| **Documentation requise** | Complète et à jour |
| **Support** | Actif (corrections, évolutions mineures) |

**Comportements :**

- L'élément est **la version de référence** pour les consommateurs
- Toute modification est soumise aux **règles de compatibilité** (INV-EB-5)
- Les évolutions mineures (rétrocompatibles) sont autorisées
- Les évolutions majeures (incompatibles) nécessitent une **nouvelle version**
- L'élément est **documenté, testé, et supporté**

**Garanties aux consommateurs :**

| Garantie | Description |
|----------|-------------|
| **Stabilité fonctionnelle** | Le comportement documenté ne change pas |
| **Rétrocompatibilité par défaut** | Les évolutions préservent la compatibilité sauf déclaration explicite |
| **Support actif** | Les bugs critiques sont corrigés |
| **Documentation maintenue** | La documentation reflète l'état actuel |
| **Préavis de dépréciation** | Minimum 1 cycle de release avant dépréciation |

**Conditions de sortie :**

- L'élément peut transitionner vers **DEPRECATED** avec annonce préalable obligatoire
- L'élément **ne peut jamais** transitionner directement vers RETIRED ou ARCHIVED (INV-EB-4)

**Référence Glossaire :** [ACTIF (ACTIVE)](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#actif-active--état-de-vie)

---

### 3.3 DEPRECATED (Déprécié)

**Définition canonique :**

> L'élément est toujours fonctionnel mais son usage est découragé. Un successeur existe ou est en préparation. Les consommateurs sont avertis de migrer. La période de dépréciation est définie et communiquée.

**Caractéristiques :**

| Propriété | Valeur |
|-----------|--------|
| **Disponibilité production** | ✅ Oui (mais usage découragé) |
| **Stabilité garantie** | ✅ Maintenue |
| **Changements autorisés** | Corrections critiques uniquement |
| **Consommateurs attendus** | Existants (nouveaux usages découragés) |
| **Documentation requise** | Complète + guide de migration |
| **Support** | Maintenance minimale |

**Comportements :**

- L'élément **reste fonctionnel** pendant toute la période de dépréciation
- Un **successeur est identifié** (ou l'absence de successeur est explicite) (INV-EB-10)
- La **période de dépréciation** est définie et communiquée
- Les consommateurs reçoivent des **alertes de migration**
- Seules les **corrections critiques de sécurité** sont appliquées
- Les **nouvelles fonctionnalités sont refusées**

**Informations obligatoires lors de la dépréciation :**

| Information | Obligatoire | Description |
|-------------|-------------|-------------|
| **Raison de dépréciation** | ✅ | Pourquoi l'élément est déprécié |
| **Successeur identifié** | ✅ | L'élément de remplacement (ou "aucun") |
| **Date de début de dépréciation** | ✅ | Quand la période commence |
| **Date prévue de retirement** | ✅ | Quand la période se termine |
| **Guide de migration** | ✅ | Comment migrer vers le successeur |
| **Impact sur les consommateurs** | ✅ | Ce qui change pour eux |

**Conditions de sortie :**

- L'élément peut transitionner vers **RETIRED** à la fin de la période de dépréciation
- L'élément peut être **réactivé vers ACTIVE** si le successeur est annulé (cas exceptionnel)

**Référence Glossaire :** [DÉPRÉCIÉ (DEPRECATED)](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#déprécié-deprecated--état-de-vie)

---

### 3.4 RETIRED (Retiré)

**Définition canonique :**

> L'élément n'est plus activement supporté mais reste fonctionnel pour les consommateurs existants. Aucune nouvelle fonctionnalité, uniquement des corrections critiques de sécurité.

**Caractéristiques :**

| Propriété | Valeur |
|-----------|--------|
| **Disponibilité production** | ⚠️ Limitée (existants uniquement) |
| **Stabilité garantie** | ⚠️ Best effort |
| **Changements autorisés** | Corrections sécurité critiques uniquement |
| **Consommateurs attendus** | Uniquement ceux n'ayant pas pu migrer |
| **Documentation requise** | Archivée (non maintenue) |
| **Support** | Aucun (sauf sécurité critique) |

**Comportements :**

- L'élément **reste techniquement fonctionnel** mais n'est plus recommandé
- **Aucune nouvelle fonctionnalité** n'est ajoutée
- **Aucune correction de bug** (sauf sécurité critique)
- Les **nouveaux consommateurs sont bloqués** (l'élément n'est pas proposé)
- La **documentation n'est plus maintenue**
- Une **période de grâce** peut être accordée aux retardataires

**Droits résiduels des consommateurs existants :**

| Droit | Garanti |
|-------|---------|
| **Fonctionnement continu** | ✅ Tant que l'élément est RETIRED |
| **Corrections de sécurité** | ✅ Critiques uniquement |
| **Support technique** | ❌ Non |
| **Nouvelles fonctionnalités** | ❌ Non |
| **Documentation à jour** | ❌ Non |

**Conditions de sortie :**

- L'élément peut transitionner vers **ARCHIVED** après la période de grâce
- L'élément **ne peut jamais** revenir à ACTIVE ou DEPRECATED

**Référence Glossaire :** [RETIRÉ (RETIRED)](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#retiré-retired--état-de-vie)

---

### 3.5 ARCHIVED (Archivé)

**Définition canonique :**

> L'élément n'est plus fonctionnel. Il est conservé uniquement pour référence historique et traçabilité. Aucune garantie de fonctionnement.

**Caractéristiques :**

| Propriété | Valeur |
|-----------|--------|
| **Disponibilité production** | ❌ Non |
| **Stabilité garantie** | ❌ Aucune |
| **Changements autorisés** | ❌ Aucun (élément gelé) |
| **Consommateurs attendus** | Aucun |
| **Documentation requise** | Archivée (lecture seule) |
| **Support** | ❌ Aucun |

**Comportements :**

- L'élément **n'est plus exécutable** en production
- L'élément est conservé comme **tombstone** (référence historique)
- La **traçabilité complète** est maintenue (INV-EB-2)
- **Aucune modification** n'est possible
- L'élément **ne peut jamais être réactivé**

**Ce qui est conservé dans l'archive :**

| Élément | Conservé |
|---------|----------|
| **Métadonnées** | ✅ ID, nom, version, dates |
| **Historique des transitions** | ✅ Chaîne d'évolution complète |
| **Documentation finale** | ✅ Snapshot au moment de l'archivage |
| **Raison de l'archivage** | ✅ Justification documentée |
| **Référence au successeur** | ✅ Si applicable |
| **Données fonctionnelles** | ❌ Non (tombstone uniquement) |

**Conditions de sortie :**

- **Aucune transition possible** depuis ARCHIVED
- L'état ARCHIVED est **terminal et définitif**

---

## 4. Tableau récapitulatif des états

| État | Production | Stabilité | Support | Évolutions | Réversible |
|------|------------|-----------|---------|------------|------------|
| **DRAFT** | ❌ | ❌ | ❌ | Libres | ✅ → ACTIVE ou ARCHIVED |
| **ACTIVE** | ✅ | ✅ | ✅ | Compatibles | ✅ → DEPRECATED |
| **DEPRECATED** | ⚠️ | ✅ | ⚠️ | Sécurité seulement | ⚠️ → RETIRED ou ACTIVE* |
| **RETIRED** | ⚠️ | ⚠️ | ❌ | Sécurité critique | ✅ → ARCHIVED |
| **ARCHIVED** | ❌ | ❌ | ❌ | Aucune | ❌ Terminal |

*La réactivation DEPRECATED → ACTIVE est exceptionnelle et conditionnée.

---

## 5. Règles applicables aux états

### 5.1 Règle d'unicité d'état (INV-EB-3)

> Chaque élément du système possède **exactement un** état de cycle de vie à tout moment. Il n'existe pas d'état intermédiaire, incertain, ou non défini.

**Violations :**
- Un élément sans état déclaré
- Un élément avec plusieurs états simultanés
- Un élément dans un état "en transition"

### 5.2 Règle de dépréciation obligatoire (INV-EB-4)

> Aucun élément ACTIVE ne peut passer directement à RETIRED ou ARCHIVED. La transition par DEPRECATED est **obligatoire**.

**Violations :**
- ACTIVE → RETIRED (interdit)
- ACTIVE → ARCHIVED (interdit)
- Toute tentative de "fast-track" vers retirement

### 5.3 Règle de rétrocompatibilité par défaut (INV-EB-5)

> Toute évolution est **présumée rétrocompatible** sauf déclaration explicite contraire.

**Implication pour les états :**
- Un élément ACTIVE qui évolue reste ACTIVE
- Une évolution incompatible crée un **nouvel élément** (nouvelle version majeure)
- L'ancien élément passe à DEPRECATED

### 5.4 Règle de documentation obligatoire (INV-EB-7)

> Toute transition d'état doit être **documentée**.

**Documentation minimale par transition :**

| Transition | Documentation requise |
|------------|----------------------|
| DRAFT → ACTIVE | Raison d'activation, documentation complète |
| ACTIVE → DEPRECATED | Raison, successeur, période, guide de migration |
| DEPRECATED → RETIRED | Confirmation fin de période, consommateurs restants |
| RETIRED → ARCHIVED | Raison d'archivage, snapshot final |
| DRAFT → ARCHIVED | Raison d'abandon |
| DEPRECATED → ACTIVE | Justification de réactivation (exceptionnel) |

---

## 6. Application aux Tools et Toolkits

Ever Buddy gouverne le cycle de vie des **Tools** (Strate 6) avec les mêmes états, mais avec des règles spécifiques.

### 6.1 États de vie des Tools

| État | Description Tool |
|------|------------------|
| **DRAFT** | Tool en développement, non disponible |
| **ACTIVE** | Tool disponible et supporté |
| **DEPRECATED** | Tool fonctionnel mais usage découragé |
| **RETIRED** | Tool retiré, non disponible |

Note : L'état ARCHIVED n'est généralement pas utilisé pour les Tools (ils sont directement retirés puis oubliés au niveau opérationnel, mais tracés au niveau historique).

### 6.2 Règles spécifiques aux Tools

| Règle | Description |
|-------|-------------|
| **RÈGLE-TOOL-EV-1** | Tout Tool a un état de vie explicite |
| **RÈGLE-TOOL-EV-2** | Un Tool DEPRECATED a un successeur identifié |
| **RÈGLE-TOOL-EV-3** | La transition vers RETIRED passe obligatoirement par DEPRECATED |
| **RÈGLE-TOOL-EV-4** | La compatibilité Tool ↔ Environnement est vérifiée |

**Référence :** [Miyukini Conceptual References - Tools et Toolkits](../../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)

---

## 7. Catégories d'éléments et règles d'état

Les règles d'état varient selon la **catégorie** de l'élément.

### 7.1 Contrats fondateurs (FONDATION)

| Aspect | Valeur |
|--------|--------|
| **Temps minimum en ACTIVE** | Très long (plusieurs générations) |
| **Période de dépréciation** | Très longue |
| **Ruptures** | Quasi interdites |
| **Exemple** | Documentation Fondatrice d'un core |

### 7.2 Contrats opérationnels

| Aspect | Valeur |
|--------|--------|
| **Temps minimum en ACTIVE** | Standard |
| **Période de dépréciation** | Standard |
| **Ruptures** | Possibles avec justification |
| **Exemple** | API Contract, Interface Contract |

### 7.3 Interfaces techniques

| Aspect | Valeur |
|--------|--------|
| **Temps minimum en ACTIVE** | Court |
| **Période de dépréciation** | Courte |
| **Ruptures** | Possibles avec documentation |
| **Exemple** | Adaptateur, Tool |

### 7.4 Éléments internes

| Aspect | Valeur |
|--------|--------|
| **Temps minimum en ACTIVE** | Aucun minimum |
| **Période de dépréciation** | Optionnelle |
| **Ruptures** | Sans préavis autorisées |
| **Exemple** | Implémentation interne |

---

## 8. Invariants applicables aux états

Ce contrat est gouverné par les invariants suivants de la Documentation Fondatrice :

| Invariant | Énoncé | Application aux états |
|-----------|--------|----------------------|
| **INV-EB-2** | Traçabilité complète et immuable | Chaque transition d'état est enregistrée |
| **INV-EB-3** | Aucun état ambigu | Un seul état à tout moment |
| **INV-EB-4** | Période de dépréciation obligatoire | DEPRECATED obligatoire avant RETIRED |
| **INV-EB-5** | Rétrocompatibilité par défaut | Les évolutions en ACTIVE sont compatibles |
| **INV-EB-7** | Documentation obligatoire | Chaque transition est documentée |
| **INV-EB-9** | Prédictibilité des transitions | Les règles d'état sont publiques |

---

## 9. Conformité aux Lois d'Autonomie

Ce contrat respecte les Lois d'Autonomie Système :

| Loi | Conformité | Mécanisme |
|-----|------------|-----------|
| **LOI-1** | ✅ | États stockés localement, pas de dépendance externe |
| **LOI-2** | ✅ | États valides en mode isolé |
| **LOI-3** | ✅ | État local souverain |
| **LOI-4** | ✅ | États discrets, pas de temps global requis |

**Référence :** [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)

---

## 10. Références croisées

- **Document source :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)
- **Contrat complémentaire :** [Ever Buddy - Transition Rules Contract](./Ever%20Buddy%20-%20Transition%20Rules%20Contract.md) (transitions entre états)
- **Glossaire :** [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- **Lois d'Autonomie :** [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)
- **Tools et Toolkits :** [Miyukini Conceptual References - Tools et Toolkits](../../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Contrat normatif — Non négociable  
**Dérivé de :** Ever Buddy - Documentation Fondatrice v1.3, Section 4  
**Type :** Contrat de cycle de vie
