# Ever Buddy - Compatibility Rules Contract

## 1. Contexte

Ce document définit les **règles de compatibilité** gouvernées par Ever Buddy dans l'écosystème Miyukini. Il spécifie les niveaux de compatibilité, les critères d'évaluation, et les obligations associées à chaque type de changement.

**Document fondateur :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)

**Statut contractuel :** Ce document est **contractuel, normatif, et non négociable**. Il dérive directement de la Documentation Fondatrice (Section 4 - Concepts fondamentaux : Compatibilité).

---

## 2. Portée / Scope

- **Applicable à :** Tous les éléments du système possédant un cycle de vie (contrats, structures, interfaces, éléments internes)
- **Responsable :** Ever Buddy (responsabilité exclusive de définition des règles de compatibilité - Section 5)
- **Consommateurs :** Tous les cores, opérateurs, et produits de l'écosystème Miyukini
- **Ne couvre pas :** L'exécution technique de la compatibilité (responsabilité des implémentations)

---

## 3. Définition canonique de la compatibilité

### 3.1 Qu'est-ce que la compatibilité ?

La **compatibilité** est la capacité d'un élément à fonctionner avec des éléments d'autres versions. Elle caractérise la relation entre différentes versions d'un même élément ou entre éléments interdépendants.

**Référence glossaire :** La compatibilité n'est pas une garantie technique — c'est une **règle de gouvernance** définissant les attentes et obligations lors des évolutions.

### 3.2 Responsabilité d'Ever Buddy

Ever Buddy est **exclusivement responsable** de la définition des règles de compatibilité entre versions. Cette responsabilité inclut :

- Définir ce qui constitue un changement rétrocompatible
- Définir ce qui constitue une rupture de compatibilité
- Définir les périodes de transition minimales pour chaque type de changement
- Définir les exceptions autorisées (et leurs conditions strictes)

**Invariant associé :** INV-EB-5 — Toute évolution est **présumée rétrocompatible** sauf déclaration explicite contraire.

---

## 4. Niveaux de compatibilité

Ever Buddy distingue trois niveaux de compatibilité, chacun avec des implications et des obligations spécifiques.

### 4.1 Rétrocompatible (Backward Compatible)

**Définition :** Le nouveau fonctionne avec l'ancien. Les consommateurs existants continuent de fonctionner sans modification.

| Aspect | Spécification |
|--------|---------------|
| **Direction** | Nouveau → Ancien |
| **Impact consommateur** | Aucune modification requise |
| **Migration** | Optionnelle |
| **Coexistence** | Naturelle et illimitée |
| **Version** | Changement mineur ou correctif |

**Critères de rétrocompatibilité :**

1. **Interface préservée** — Toutes les interfaces existantes restent fonctionnelles
2. **Comportement préservé** — Les comportements existants produisent les mêmes résultats
3. **Contrats préservés** — Les contrats établis restent valides
4. **Données préservées** — Les structures de données existantes restent exploitables

**Exemples de changements rétrocompatibles :**

- Ajout d'un champ optionnel à une structure
- Ajout d'une nouvelle méthode à une interface
- Correction d'un bug sans changement de comportement documenté
- Amélioration de performance sans changement d'interface
- Ajout d'une nouvelle fonctionnalité indépendante

### 4.2 Compatible en amont (Forward Compatible)

**Définition :** L'ancien fonctionne avec le nouveau. Les anciennes versions peuvent consommer les nouvelles fonctionnalités (rare, souvent impossible).

| Aspect | Spécification |
|--------|---------------|
| **Direction** | Ancien → Nouveau |
| **Impact consommateur** | Aucune modification requise pour l'ancien |
| **Migration** | Non nécessaire |
| **Coexistence** | Requiert une conception explicite |
| **Version** | Cas particulier, rarement applicable |

**Critères de compatibilité en amont :**

1. **Extensibilité conçue** — L'élément ancien a été conçu pour ignorer les extensions inconnues
2. **Dégradation gracieuse** — L'absence de nouvelles fonctionnalités n'empêche pas le fonctionnement
3. **Protocole ouvert** — Le protocole de communication permet l'ajout de nouveaux éléments

**Exemples de compatibilité en amont :**

- Format de données avec champs ignorés si inconnus (JSON extensible)
- Protocole de communication avec version négociée
- Interface avec méthodes optionnelles

**Avertissement :** La compatibilité en amont est **exceptionnelle**. Elle requiert une conception anticipée et ne peut être garantie rétroactivement.

### 4.3 Incompatible (Breaking)

**Définition :** Le nouveau ne fonctionne pas avec l'ancien. Une migration est obligatoire.

| Aspect | Spécification |
|--------|---------------|
| **Direction** | Aucune coexistence naturelle |
| **Impact consommateur** | Modification obligatoire |
| **Migration** | Obligatoire avec chemin documenté |
| **Coexistence** | Temporaire, période de transition |
| **Version** | Changement majeur obligatoire |

**Critères d'incompatibilité (un seul suffit) :**

1. **Interface rompue** — Une interface existante est modifiée ou supprimée
2. **Comportement modifié** — Un comportement existant produit des résultats différents
3. **Contrat violé** — Un contrat établi n'est plus respecté
4. **Données incompatibles** — Les structures de données existantes ne sont plus exploitables

**Exemples de changements incompatibles :**

- Suppression d'un champ obligatoire
- Modification de la sémantique d'une méthode
- Changement de type d'un paramètre
- Renommage d'une interface publique
- Modification du format de sérialisation

---

## 5. Obligations selon le niveau de compatibilité

### 5.1 Obligations pour les changements rétrocompatibles

| Obligation | Requis | Description |
|------------|--------|-------------|
| Annonce préalable | ❌ Non | Peut être publié sans annonce formelle |
| Période de transition | ❌ Non | Pas de période de transition requise |
| Documentation | ✅ Oui | Changement documenté dans les notes de version |
| Chemin de migration | ❌ Non | Pas de migration nécessaire |
| Test de non-régression | ✅ Oui | Vérification que l'existant fonctionne |

### 5.2 Obligations pour les changements incompatibles

| Obligation | Requis | Description |
|------------|--------|-------------|
| Annonce préalable | ✅ Oui | Communication formelle avant mise en œuvre |
| Période de transition | ✅ Oui | Période de dépréciation obligatoire (INV-EB-4) |
| Documentation | ✅ Oui | Documentation complète des différences |
| Chemin de migration | ✅ Oui | Guide de migration fourni |
| Justification | ✅ Oui | Raison documentée de la rupture |
| Impact évalué | ✅ Oui | Analyse d'impact sur les consommateurs |

**Règle absolue (INV-EB-4) :** Aucun élément ACTIVE ne peut passer directement à RETIRED ou ARCHIVED. La transition par DEPRECATED est **obligatoire**. Cela s'applique à tous les changements incompatibles.

---

## 6. Fenêtre de compatibilité (Compatibility Window)

### 6.1 Définition

La **fenêtre de compatibilité** est la plage de versions avec lesquelles un élément garantit la compatibilité.

**Format :** `[version_min, version_max]` ou `[version_min, *)` pour les versions ouvertes

**Exemples :**

- `[v2.0, v2.4]` — Compatible avec les versions 2.0 à 2.4 incluses
- `[v3.0, *)` — Compatible avec toutes les versions à partir de 3.0

### 6.2 Règles de fenêtre

| Règle | Description |
|-------|-------------|
| **RÈGLE-COMPAT-1** | Toute fenêtre de compatibilité est **explicite et documentée** |
| **RÈGLE-COMPAT-2** | La fermeture d'une fenêtre requiert une **période de transition** |
| **RÈGLE-COMPAT-3** | L'extension d'une fenêtre est **toujours autorisée** sans formalité |
| **RÈGLE-COMPAT-4** | La réduction d'une fenêtre est un **changement incompatible** |

### 6.3 Gestion des fenêtres par catégorie

| Catégorie | Fenêtre minimale recommandée | Fermeture |
|-----------|------------------------------|-----------|
| Contrats fondateurs (FONDATION) | 3 générations majeures | Quasi interdite |
| Contrats opérationnels | 2 générations majeures | Avec justification |
| Interfaces techniques | 1 génération majeure | Avec documentation |
| Éléments internes | Aucune garantie | Libre |

---

## 7. Évaluation de la compatibilité

### 7.1 Processus d'évaluation

Toute évolution doit être évaluée pour déterminer son niveau de compatibilité :

```
1. Identification des changements
   ↓
2. Analyse d'impact sur les interfaces
   ↓
3. Analyse d'impact sur les comportements
   ↓
4. Analyse d'impact sur les contrats
   ↓
5. Analyse d'impact sur les données
   ↓
6. Classification du niveau de compatibilité
   ↓
7. Détermination des obligations associées
```

### 7.2 Questions d'évaluation

Pour chaque changement proposé, répondre aux questions suivantes :

**Interface :**
- [ ] Les interfaces existantes sont-elles préservées ?
- [ ] Les signatures de méthodes sont-elles inchangées ?
- [ ] Les points d'entrée existants restent-ils fonctionnels ?

**Comportement :**
- [ ] Les comportements documentés produisent-ils les mêmes résultats ?
- [ ] Les effets de bord sont-ils identiques ?
- [ ] Les erreurs sont-elles levées dans les mêmes conditions ?

**Contrat :**
- [ ] Les invariants existants sont-ils toujours respectés ?
- [ ] Les garanties documentées sont-elles maintenues ?
- [ ] Les pré/post-conditions sont-elles inchangées ?

**Données :**
- [ ] Les structures existantes sont-elles toujours valides ?
- [ ] Les formats de sérialisation sont-ils compatibles ?
- [ ] Les migrations de données sont-elles évitées ?

**Résultat :**
- Si **toutes les réponses sont "Oui"** → Changement **rétrocompatible**
- Si **une seule réponse est "Non"** → Changement **incompatible**

---

## 8. Règles de rupture de compatibilité

### 8.1 Conditions de rupture autorisée

Une rupture de compatibilité est autorisée **uniquement** si :

1. **Justification documentée** — La rupture est nécessaire et les alternatives ont été évaluées
2. **Impact évalué** — L'impact sur les consommateurs est documenté
3. **Période de transition** — Une période de dépréciation est planifiée
4. **Chemin de migration** — Un guide de migration est fourni
5. **Communication préalable** — L'annonce est faite en avance (minimum 1 cycle de release)

### 8.2 Ruptures exceptionnelles

Certaines ruptures peuvent être accélérées en cas de :

| Cas | Période minimale | Condition |
|-----|------------------|-----------|
| Faille de sécurité critique | Immédiate | Documentation post-facto |
| Violation légale | Immédiate | Obligation réglementaire documentée |
| Corruption de données | 1 cycle de release | Risque de perte de données |

**Avertissement :** Ces exceptions sont **strictement encadrées** et requièrent une justification formelle. Elles ne peuvent pas être utilisées pour contourner la discipline normale.

### 8.3 Ruptures interdites

Les ruptures suivantes sont **structurellement interdites** :

| Rupture interdite | Raison |
|-------------------|--------|
| Rupture rétroactive | INV-EB-11 — Les règles ne peuvent pas modifier le passé |
| Rupture sans transition | INV-EB-4 — DEPRECATED est obligatoire |
| Rupture sans documentation | INV-EB-7 — Documentation obligatoire |
| Rupture discriminatoire | INV-EB-8 — Règles universelles |

---

## 9. Interactions avec le versionnement

### 9.1 Correspondance compatibilité-version

| Type de changement | Version | Compatibilité |
|--------------------|---------|---------------|
| Correction de bug | Correctif (+0.0.1) | Rétrocompatible |
| Ajout de fonctionnalité | Mineur (+0.1.0) | Rétrocompatible |
| Rupture de compatibilité | Majeur (+1.0.0) | Incompatible |

**Règle absolue :** Un changement incompatible **doit** être accompagné d'un changement de version majeure. Un changement mineur ou correctif **ne peut jamais** être incompatible.

### 9.2 Relation avec le contrat de sémantique de version

Les règles de compatibilité sont complémentaires au contrat de sémantique de version :

- **Compatibilité** définit la relation entre versions
- **Sémantique de version** définit comment les versions sont numérotées

**Référence :** [Ever Buddy - Version Semantics Contract](./Ever%20Buddy%20-%20Version%20Semantics%20Contract.md)

---

## 10. Métriques de compatibilité

Ever Buddy surveille les métriques suivantes relatives à la compatibilité :

### 10.1 Métriques d'évolution

| Métrique | Description | Seuil d'alerte |
|----------|-------------|----------------|
| Taux de ruptures | Ratio ruptures / évolutions totales | > 20% sur 1 génération |
| Durée moyenne de transition | Temps entre DEPRECATED et RETIRED | < minimum défini |
| Fenêtre moyenne | Largeur moyenne des fenêtres de compatibilité | Réduction tendancielle |

### 10.2 Métriques d'adoption

| Métrique | Description | Seuil d'alerte |
|----------|-------------|----------------|
| Taux d'adoption du successeur | % de consommateurs ayant migré | < 80% à mi-transition |
| Consommateurs non migrés | Nombre de consommateurs restant sur l'ancien | > 0 à fin de transition |
| Temps de migration moyen | Durée moyenne de migration par consommateur | > période de transition |

---

## 11. Références croisées

### Invariants associés (Documentation Fondatrice - Section 7)

| Invariant | Énoncé | Relation |
|-----------|--------|----------|
| INV-EB-4 | Période de dépréciation obligatoire | Appliqué à toute rupture |
| INV-EB-5 | Rétrocompatibilité par défaut | Présomption de base |
| INV-EB-7 | Documentation obligatoire | Toute rupture documentée |
| INV-EB-8 | Indépendance des décisions | Règles universelles |
| INV-EB-9 | Prédictibilité des transitions | Règles publiques et stables |
| INV-EB-11 | Non-rétroactivité | Pas de rupture rétroactive |

### Documents associés

| Document | Relation |
|----------|----------|
| [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) | Document source |
| [Ever Buddy - Version Semantics Contract](./Ever%20Buddy%20-%20Version%20Semantics%20Contract.md) | Numérotation des versions |
| [Ever Buddy - Lifecycle States Contract](../lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md) | États de cycle de vie |
| [Ever Buddy - Transition Rules Contract](../lifecycle/Ever%20Buddy%20-%20Transition%20Rules%20Contract.md) | Règles de transition |

### Références glossaire

| Terme | Définition |
|-------|------------|
| **Rétrocompatible** | Le nouveau fonctionne avec l'ancien |
| **Compatible en amont** | L'ancien fonctionne avec le nouveau |
| **Incompatible** | Le nouveau ne fonctionne pas avec l'ancien |
| **Fenêtre de compatibilité** | Plage de versions garantissant la compatibilité |
| **Breaking change** | Changement qui rompt la compatibilité |

**Source :** [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 12. Synthèse contractuelle

### Garanties de ce contrat

Ce contrat garantit que :

1. **La compatibilité est définie** — Trois niveaux clairs avec critères explicites
2. **Les obligations sont connues** — Chaque niveau de compatibilité a des obligations documentées
3. **Les ruptures sont encadrées** — Conditions strictes pour les changements incompatibles
4. **La rétrocompatibilité est la norme** — Présomption par défaut (INV-EB-5)
5. **Les transitions sont protégées** — Période de dépréciation obligatoire (INV-EB-4)

### Phrase de synthèse

> **La compatibilité est la promesse faite aux consommateurs : ce qui fonctionne aujourd'hui fonctionnera demain, sauf annonce explicite, période de transition, et chemin de migration.**

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Contrat — Normatif  
**Référence :** Ever Buddy v1.0, Documentation Fondatrice Section 4  
**Type :** Contrat de compatibilité
