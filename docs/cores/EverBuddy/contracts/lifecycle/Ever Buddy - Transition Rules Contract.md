# Ever Buddy - Transition Rules Contract

## Contexte

Ce document définit les **règles contractuelles de transition** entre états de cycle de vie dans l'écosystème Miyukini. Il spécifie la matrice des transitions valides, les périodes minimales obligatoires, les conditions de validation, et les règles de documentation associées.

Ce contrat opérationnalise les principes définis dans la [Documentation Fondatrice d'Ever Buddy](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md), Section 4 (Concepts fondamentaux).

**Référence canonique :** [Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) — définitions des états de vie (DRAFT, ACTIVE, DEPRECATED, RETIRED, ARCHIVED).

---

## Portée / Scope

- **Applicable à :** Toutes les transitions d'état de cycle de vie dans l'écosystème Miyukini
- **Audience :** Architectes, développeurs, implémenteurs de cores, opérateurs
- **Statut :** Contrat opérationnel normatif — NON NÉGOCIABLE

---

## 1. Matrice des Transitions Valides

### 1.1 Définition

Une **transition** est le passage d'un état de cycle de vie à un autre. Les transitions sont :
- **Atomiques** : Un élément passe de l'état A à l'état B sans état transitoire (INV-EB-3)
- **Documentées** : Chaque transition requiert une documentation obligatoire (INV-EB-7)
- **Validées** : Ever Buddy vérifie que la transition respecte les règles

### 1.2 Matrice Complète

| Depuis \ Vers | DRAFT | ACTIVE | DEPRECATED | RETIRED | ARCHIVED |
|---------------|-------|--------|------------|---------|----------|
| **DRAFT**     | —     | ✓      | ✗          | ✗       | ✓        |
| **ACTIVE**    | ✗     | —      | ✓          | ✗       | ✗        |
| **DEPRECATED**| ✗     | ✓*     | —          | ✓       | ✗        |
| **RETIRED**   | ✗     | ✗      | ✗          | —       | ✓        |
| **ARCHIVED**  | ✗     | ✗      | ✗          | ✗       | —        |

**Légende :**
- ✓ = Transition valide
- ✗ = Transition invalide (structurellement interdite)
- ✓* = Transition conditionnelle (voir Section 2.3)
- — = Non applicable (même état)

### 1.3 Transitions Valides Détaillées

| Code | Transition | Description | Condition |
|------|------------|-------------|-----------|
| **T-DA** | DRAFT → ACTIVE | Activation | Élément prêt pour production |
| **T-DAR** | DRAFT → ARCHIVED | Abandon précoce | Élément abandonné avant activation |
| **T-AD** | ACTIVE → DEPRECATED | Dépréciation | Successeur identifié ou abandon décidé |
| **T-DE** | DEPRECATED → ACTIVE | Réactivation* | Successeur annulé, élément fonctionnel |
| **T-DR** | DEPRECATED → RETIRED | Retirement | Période de dépréciation écoulée |
| **T-RA** | RETIRED → ARCHIVED | Archivage | Période de grâce écoulée |

### 1.4 Transitions Interdites (Exhaustif)

Les transitions suivantes sont **structurellement interdites** :

| Transition | Raison de l'interdiction |
|------------|-------------------------|
| DRAFT → DEPRECATED | Un élément non activé ne peut être déprécié |
| DRAFT → RETIRED | Un élément non activé ne peut être retiré |
| ACTIVE → DRAFT | Régression interdite — pas de retour en brouillon |
| ACTIVE → RETIRED | **Passage obligatoire par DEPRECATED** (INV-EB-4) |
| ACTIVE → ARCHIVED | Séquence obligatoire : ACTIVE → DEPRECATED → RETIRED → ARCHIVED |
| DEPRECATED → DRAFT | Régression interdite |
| DEPRECATED → ARCHIVED | Passage obligatoire par RETIRED |
| RETIRED → DRAFT | Réactivation interdite après retirement |
| RETIRED → ACTIVE | Réactivation interdite après retirement |
| RETIRED → DEPRECATED | Régression interdite |
| ARCHIVED → * | **Aucune sortie possible** — état terminal absolu |

---

## 2. Règles de Transition

### 2.1 Règle Fondamentale : Passage Obligatoire par DEPRECATED (INV-EB-4)

> **Aucun élément ACTIVE ne peut passer directement à RETIRED ou ARCHIVED.**

La transition par DEPRECATED est **obligatoire** sans exception. Cette règle protège les consommateurs contre les ruptures brutales.

**Séquence obligatoire pour fin de vie :**

```
ACTIVE → DEPRECATED → RETIRED → ARCHIVED
```

**Violation :** Toute tentative de contournement est rejetée par Ever Buddy.

### 2.2 Règle d'Atomicité des Transitions (INV-EB-3)

Chaque élément possède **exactement un** état de cycle de vie à tout moment.

- ❌ Pas d'état intermédiaire
- ❌ Pas d'état incertain
- ❌ Pas d'état non défini
- ✓ Transitions atomiques uniquement

### 2.3 Règle de Réactivation Conditionnelle (DEPRECATED → ACTIVE)

La transition DEPRECATED → ACTIVE est **conditionnelle**. Elle n'est autorisée que si :

| Condition | Obligatoire |
|-----------|-------------|
| Le successeur prévu est annulé | ✓ |
| L'élément déprécié est encore fonctionnel | ✓ |
| La décision de réactivation est documentée avec justification | ✓ |
| L'historique conserve la trace de la dépréciation temporaire | ✓ |

**Scénario typique :** Le développement du successeur échoue ou est abandonné, nécessitant le maintien de l'élément déprécié.

### 2.4 Règle de Prédictibilité (INV-EB-9)

Les règles de transition sont **publiques et stables**.

| Garantie | Description |
|----------|-------------|
| Transparence | Tout consommateur peut connaître à l'avance les conditions de transition |
| Stabilité | Les règles ne changent pas fréquemment |
| Non-rétroactivité | Aucune règle ne peut être modifiée rétroactivement (INV-EB-11) |

### 2.5 Règle de Non-Rétroactivité (INV-EB-11)

Les règles d'évolution s'appliquent aux transitions **futures** uniquement.

- Un changement de règle ne modifie pas le statut d'éléments déjà en transition
- Les transitions en cours continuent selon les règles initiales
- Cette règle protège les transitions en cours de complétion

---

## 3. Périodes Minimales de Transition

### 3.1 Définition

Chaque type de transition possède une **période minimale non négociable**. Ces périodes sont des **minimums absolus** — Ever Buddy peut recommander des périodes plus longues selon l'impact et l'adoption.

### 3.2 Tableau des Périodes Minimales

| Transition | Période Minimale | Notes |
|------------|------------------|-------|
| **DRAFT → ACTIVE** | Aucune | Activation immédiate possible |
| **ACTIVE → DEPRECATED** | 1 cycle de release | Communication préalable obligatoire |
| **DEPRECATED → RETIRED** | Définie par catégorie | Voir Section 3.3 |
| **RETIRED → ARCHIVED** | Période de grâce | Pour consommateurs existants |
| **DRAFT → ARCHIVED** | Aucune | Abandon immédiat possible |
| **DEPRECATED → ACTIVE** | Aucune | Réactivation immédiate si conditions remplies |

### 3.3 Périodes par Catégorie d'Élément

Les périodes de dépréciation (DEPRECATED → RETIRED) varient selon la catégorie :

| Catégorie | Description | Période Minimale | Ruptures |
|-----------|-------------|------------------|----------|
| **Contrats Fondateurs (FONDATION)** | Documents contractuels non négociables | Très longue (plusieurs générations) | Quasiment interdites |
| **Contrats Opérationnels** | Contrats de fonctionnement standard | Standard (plusieurs cycles) | Possibles avec justification |
| **Interfaces Techniques** | APIs, surfaces d'appel | Courte (quelques cycles) | Possibles avec documentation |
| **Éléments Internes** | Composants internes non exposés | Aucune garantie | Sans préavis autorisées |

### 3.4 Facteurs d'Extension

Ever Buddy peut recommander des périodes **plus longues** que les minimums selon :

| Facteur | Impact |
|---------|--------|
| Nombre de consommateurs | Plus de consommateurs = période plus longue |
| Criticité de l'élément | Élément critique = période plus longue |
| Complexité de migration | Migration complexe = période plus longue |
| Taux d'adoption du successeur | Adoption lente = période plus longue |

### 3.5 Période de Grâce

La **période de grâce** est le temps supplémentaire accordé après la date prévue de retirement.

| Caractéristique | Description |
|-----------------|-------------|
| Déclenchement | Après période de retirement standard |
| But | Permettre aux consommateurs retardataires de migrer |
| Attribution | Au cas par cas, sur demande justifiée |
| Durée | Variable selon la situation |

---

## 4. Documentation Obligatoire des Transitions (INV-EB-7)

### 4.1 Règle Fondamentale

> **Toute transition d'état doit être documentée. Une transition sans documentation est invalide.**

### 4.2 Contenu Obligatoire

Chaque transition DOIT inclure :

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `transition_id` | Identifiant unique de la transition | ✓ |
| `element_id` | Identifiant de l'élément concerné | ✓ |
| `from_state` | État de départ | ✓ |
| `to_state` | État d'arrivée | ✓ |
| `reason` | Raison de la transition | ✓ |
| `impact` | Impact sur les consommateurs | ✓ |
| `migration_path` | Chemin de migration (si applicable) | Conditionnel |
| `effective_date` | Date effective de la transition | ✓ |
| `requested_by` | Demandeur de la transition | ✓ |
| `validated_by` | Validateur (Ever Buddy) | ✓ |

### 4.3 Documentation Additionnelle par Type de Transition

| Transition | Documentation Additionnelle |
|------------|----------------------------|
| ACTIVE → DEPRECATED | Successeur identifié, période de dépréciation prévue |
| DEPRECATED → RETIRED | Taux d'adoption du successeur, consommateurs non migrés |
| DEPRECATED → ACTIVE | Justification de réactivation, statut du successeur annulé |

---

## 5. Validation des Transitions

### 5.1 Processus de Validation

1. **Demande** : Un core ou produit demande une transition d'état
2. **Vérification** : Ever Buddy vérifie que la transition est valide selon ce contrat
3. **Documentation** : Ever Buddy vérifie que la documentation est complète
4. **Enregistrement** : Si valide, la transition est enregistrée dans l'historique immuable
5. **Communication** : Ever Buddy communique la transition aux consommateurs concernés

### 5.2 Critères de Rejet

Une transition est **rejetée** si :

| Critère | Description |
|---------|-------------|
| Transition invalide | La transition n'est pas dans la matrice des transitions valides |
| Documentation incomplète | Un champ obligatoire est manquant |
| Période non respectée | La période minimale n'est pas écoulée |
| Condition non remplie | Pour les transitions conditionnelles (ex: réactivation) |

### 5.3 Message de Rejet

En cas de rejet, Ever Buddy fournit :

- Le code de la transition tentée
- La raison du rejet
- Les conditions à remplir pour que la transition soit acceptée
- La référence à ce contrat

---

## 6. Invariants Applicables

Ce contrat opérationnalise les invariants suivants de la Documentation Fondatrice :

| Invariant | Énoncé | Application |
|-----------|--------|-------------|
| **INV-EB-3** | Aucun état ambigu | Transitions atomiques, un seul état à tout moment |
| **INV-EB-4** | Période de dépréciation obligatoire | Passage obligatoire par DEPRECATED |
| **INV-EB-7** | Documentation obligatoire | Chaque transition doit être documentée |
| **INV-EB-9** | Prédictibilité des transitions | Règles publiques et stables |
| **INV-EB-11** | Non-rétroactivité | Règles appliquées aux transitions futures |

---

## 7. Conformité aux Lois d'Autonomie

Ce contrat est conforme aux [Lois d'Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) :

| Loi | Conformité | Mécanisme |
|-----|------------|-----------|
| **LOI-1** | ✅ | Validation locale des transitions, pas de dépendance externe |
| **LOI-2** | ✅ | Transitions validées en mode isolé |
| **LOI-3** | ✅ | Historique des transitions souverain localement |
| **LOI-4** | ✅ | Périodes définies en cycles, pas en temps absolu |

---

## 8. Exemples de Transitions

### 8.1 Transition Standard : ACTIVE → DEPRECATED

```yaml
transition_id: "TR-2026-001"
element_id: "API-USER-V2"
from_state: "ACTIVE"
to_state: "DEPRECATED"
reason: "Nouvelle version V3 disponible avec améliorations de performance"
impact: "Les consommateurs doivent migrer vers API-USER-V3"
migration_path: "Guide de migration disponible dans docs/migration/user-api-v2-to-v3.md"
effective_date: "2026-02-01"
successor_id: "API-USER-V3"
deprecation_period: "3 cycles de release"
requested_by: "Core-Architecture"
validated_by: "Ever Buddy"
```

### 8.2 Transition Conditionnelle : DEPRECATED → ACTIVE (Réactivation)

```yaml
transition_id: "TR-2026-002"
element_id: "API-USER-V2"
from_state: "DEPRECATED"
to_state: "ACTIVE"
reason: "Développement du successeur V3 annulé - ressources insuffisantes"
impact: "API V2 redevient la version supportée"
successor_cancelled: true
successor_id: "API-USER-V3"
successor_cancellation_reason: "Contraintes techniques insurmontables"
element_functional: true
effective_date: "2026-03-15"
requested_by: "Core-Architecture"
validated_by: "Ever Buddy"
```

---

## 9. Mini log de génération

### Décision D1 : Exhaustivité des transitions interdites

**Contexte :** Nécessité de documenter explicitement toutes les transitions interdites, pas seulement celles mentionnées dans la Documentation Fondatrice.

**Décision :** Lister exhaustivement les 11 transitions interdites avec leur raison.

**Justification :** La clarté et la prédictibilité (INV-EB-9) exigent que les interdictions soient explicites, pas implicites.

### Décision D2 : Format de documentation des transitions

**Contexte :** La Documentation Fondatrice mentionne l'obligation de documentation (INV-EB-7) mais ne définit pas de format.

**Décision :** Définir un format structuré avec champs obligatoires.

**Justification :** Un format normalisé facilite la validation automatique et l'audit.

### Vérification de cohérence

**Vérifications effectuées :**
- ✅ Cohérence avec la matrice de la Documentation Fondatrice (Section 4)
- ✅ Cohérence avec les invariants INV-EB-3, INV-EB-4, INV-EB-7, INV-EB-9, INV-EB-11
- ✅ Cohérence avec les catégories d'éléments définies dans la Documentation Fondatrice
- ✅ Conformité aux Lois d'Autonomie Système

**Conclusion :** Aucune contradiction détectée avec la Documentation Fondatrice.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Contrat opérationnel — Normatif  
**Référence :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) Section 4  
**Type :** Contrat de cycle de vie
