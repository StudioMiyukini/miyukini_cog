# Ever Buddy - Version Semantics Contract

## Contexte

Ce document définit le **contrat de sémantique de versionnement** gouverné par Ever Buddy. Le versionnement conceptuel est la manière dont Ever Buddy identifie et distingue les différentes versions d'un élément du système Miyukini.

Le versionnement sémantique est au cœur de la capacité du système à évoluer sans rupture. Il permet aux consommateurs d'anticiper l'impact d'une mise à jour et de planifier leurs propres évolutions en conséquence.

**Document source :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)

---

## Portée / Scope

- **Applicable à :** Tous les éléments versionnés du système Miyukini (contrats, interfaces, structures, capacités)
- **Audience :** Architectes, développeurs, cores consommateurs, produits
- **Statut :** Document contractuel normatif — VERSION SEMANTICS CONTRACT
- **Autorité :** Ever Buddy (Strate 4) — Core de cycle de vie et d'évolution

---

## 1. Principes fondamentaux du versionnement

### 1.1 Versionnement sémantique et significatif

Le versionnement conceptuel d'Ever Buddy est **sémantique et significatif**. Chaque composant du numéro de version communique une information claire sur la nature du changement :

| Composant | Signification | Impact pour les consommateurs |
|-----------|---------------|-------------------------------|
| **Majeur** | Changement incompatible, rupture de contrat | Migration obligatoire |
| **Mineur** | Ajout de fonctionnalité, rétrocompatible | Adoption optionnelle |
| **Correctif** | Correction de bug, aucun changement fonctionnel | Adoption recommandée |

### 1.2 Format de version

Le format canonique de version est :

```
MAJEUR.MINEUR.CORRECTIF
```

**Exemples :**
- `1.0.0` — Première version stable
- `1.2.0` — Ajout de fonctionnalités depuis la v1.0.0
- `1.2.3` — Trois corrections de bugs depuis la v1.2.0
- `2.0.0` — Rupture de compatibilité par rapport à la v1.x

### 1.3 Règle de non-régression numérique

Les numéros de version ne peuvent que croître. Une fois qu'une version est publiée, elle ne peut être ni retirée, ni renumérotée. Cette règle garantit la traçabilité et la confiance des consommateurs.

---

## 2. Types de changements et versionnement

### 2.1 Changement majeur (MAJEUR)

Un **changement majeur** est toute modification qui rompt la compatibilité avec les versions précédentes.

**Exemples de changements majeurs :**
- Suppression d'une capacité existante
- Modification d'une signature de contrat
- Changement de comportement d'une fonctionnalité existante
- Restructuration incompatible des données
- Suppression d'un état de cycle de vie
- Modification des règles de transition

**Conséquences :**
- Incrémentation du numéro majeur
- Remise à zéro des numéros mineur et correctif
- Période de dépréciation obligatoire de l'ancienne version (INV-EB-4)
- Documentation obligatoire du chemin de migration (INV-EB-7)

**Référence Glossaire :** [Breaking change](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#breaking-change-changement-de-rupture)

### 2.2 Changement mineur (MINEUR)

Un **changement mineur** est un ajout de fonctionnalité qui préserve la rétrocompatibilité.

**Exemples de changements mineurs :**
- Ajout d'une nouvelle capacité
- Ajout d'un nouveau champ optionnel
- Extension d'une interface sans modification des méthodes existantes
- Ajout d'un nouvel état de vie (avec transitions définies)
- Nouvelles règles d'évolution qui n'affectent pas l'existant

**Conséquences :**
- Incrémentation du numéro mineur
- Remise à zéro du numéro correctif
- Les consommateurs existants continuent de fonctionner sans modification
- Adoption optionnelle des nouvelles fonctionnalités

### 2.3 Changement correctif (CORRECTIF)

Un **changement correctif** est une correction de bug qui ne modifie pas le comportement fonctionnel attendu.

**Exemples de changements correctifs :**
- Correction d'une erreur de calcul
- Correction d'une fuite de mémoire
- Correction d'une condition de concurrence
- Amélioration de performance sans changement fonctionnel
- Correction de documentation erronée

**Conséquences :**
- Incrémentation du numéro correctif uniquement
- Aucun impact sur les consommateurs
- Adoption recommandée pour bénéficier des corrections

---

## 3. Versionnement par catégorie d'élément

### 3.1 Contrats fondateurs (FONDATION)

Les contrats de statut **FONDATION** ont des règles de versionnement particulièrement strictes :

| Aspect | Règle |
|--------|-------|
| **Changements majeurs** | Extrêmement rares, nécessitent approbation multi-niveaux |
| **Période de dépréciation** | Minimale : 3 générations de versions |
| **Ruptures** | Quasiment interdites, uniquement en dernier recours |
| **Fréquence de changement** | Évolution très lente (années) |

**Invariant applicable :** INV-EB-6 (Vision long terme obligatoire)

### 3.2 Contrats opérationnels

Les contrats opérationnels suivent des règles de versionnement standards :

| Aspect | Règle |
|--------|-------|
| **Changements majeurs** | Possibles avec justification |
| **Période de dépréciation** | Minimale : 1 génération de versions |
| **Ruptures** | Autorisées avec plan de migration |
| **Fréquence de changement** | Évolution modérée (mois à années) |

### 3.3 Interfaces techniques

Les interfaces techniques ont des règles de versionnement plus souples :

| Aspect | Règle |
|--------|-------|
| **Changements majeurs** | Relativement courants |
| **Période de dépréciation** | Minimale : 2 cycles de release |
| **Ruptures** | Possibles avec documentation |
| **Fréquence de changement** | Évolution plus rapide (semaines à mois) |

### 3.4 Éléments internes

Les éléments internes n'ont pas de garantie de stabilité externe :

| Aspect | Règle |
|--------|-------|
| **Changements majeurs** | Libres sans contrainte externe |
| **Période de dépréciation** | Aucune obligation |
| **Ruptures** | Autorisées sans préavis |
| **Fréquence de changement** | Évolution libre |

**ATTENTION :** Les éléments internes ne doivent jamais être exposés aux consommateurs externes. Tout élément exposé devient un contrat de facto.

---

## 4. Règles de compatibilité par type de version

### 4.1 Rétrocompatibilité (comportement par défaut)

Conformément à l'invariant **INV-EB-5**, toute évolution est **présumée rétrocompatible** sauf déclaration explicite contraire.

| Type de version | Rétrocompatibilité garantie |
|-----------------|----------------------------|
| Majeur (n.0.0) | ❌ Non |
| Mineur (x.n.0) | ✅ Oui |
| Correctif (x.y.n) | ✅ Oui |

**Référence Glossaire :** [Rétrocompatible](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#retrocompatible)

### 4.2 Compatibilité en amont

La compatibilité en amont (anciennes versions fonctionnant avec les nouvelles) est :

| Type de version | Compatibilité amont |
|-----------------|---------------------|
| Majeur | ❌ Non garantie |
| Mineur | ⚠️ Partielle possible |
| Correctif | ✅ Généralement oui |

**Note :** La compatibilité en amont est rarement garantie et souvent techniquement impossible. Elle ne doit pas être une attente par défaut.

### 4.3 Fenêtre de compatibilité

Chaque élément définit une **fenêtre de compatibilité** qui spécifie les versions avec lesquelles il garantit l'interopérabilité.

**Format :**
```
Compatible avec : vX.Y à vX.Z
```

**Exemples :**
- `Compatible avec : v1.0 à v1.9` — Supporte toutes les versions 1.x
- `Compatible avec : v2.3 à v2.5` — Fenêtre restreinte

**Référence Glossaire :** [Compatibility window](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#compatibility-window)

---

## 5. Cycle de vie des versions

### 5.1 États de version

Chaque version possède un état de cycle de vie gouverné par Ever Buddy :

| État | Description | Versionnement |
|------|-------------|---------------|
| **DRAFT** | Version en développement | Version provisoire (0.x.x ou suffixe -draft) |
| **ACTIVE** | Version en usage normal | Version stable (x.y.z) |
| **DEPRECATED** | Version découragée | Conserve son numéro + marqueur dépréciation |
| **RETIRED** | Version retirée | Numéro figé, plus de correctifs |
| **ARCHIVED** | Version archivée | Numéro conservé pour référence |

### 5.2 Versions DRAFT (0.x.x)

Les versions dont le numéro majeur est **0** (zéro) sont considérées comme instables :

| Règle | Description |
|-------|-------------|
| **Stabilité** | Aucune garantie de stabilité |
| **Changements** | Tout changement possible sans incrémentation majeure |
| **Consommateurs** | Usage en connaissance de cause uniquement |
| **Transition** | Passage à 1.0.0 = première version stable |

**Exemple de progression :**
```
0.1.0 → 0.2.0 → 0.9.0 → 1.0.0 (première version stable)
```

### 5.3 Succession de versions

La **chaîne d'évolution** trace toutes les versions d'un élément :

```
v1.0.0 → v1.1.0 → v1.2.0 → v2.0.0 → v2.1.0 → ...
         └→ v1.1.1 (correctif)
```

**Règles :**
- Une version peut avoir plusieurs successeurs (branches)
- Une version a un seul prédécesseur direct
- Les correctifs se branchent depuis leur version mineure parente

**Référence Glossaire :** [Evolution chain](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#evolution-chain)

---

## 6. Déclaration des changements

### 6.1 Changelog obligatoire

Toute nouvelle version doit être accompagnée d'un **changelog** documentant :

| Section | Contenu |
|---------|---------|
| **Added** | Nouvelles fonctionnalités |
| **Changed** | Modifications de comportement |
| **Deprecated** | Éléments dépréciés |
| **Removed** | Éléments supprimés |
| **Fixed** | Corrections de bugs |
| **Security** | Corrections de sécurité |

### 6.2 Classification de l'impact

Chaque changement doit être classifié selon son impact :

| Impact | Description | Type de version |
|--------|-------------|-----------------|
| **Breaking** | Rompt la compatibilité | Majeur |
| **Feature** | Ajoute une fonctionnalité | Mineur |
| **Fix** | Corrige un bug | Correctif |
| **None** | Aucun impact fonctionnel | Correctif ou aucun |

### 6.3 Déclaration des ruptures

Conformément à l'invariant **INV-EB-5**, si une évolution est incompatible, elle doit être **explicitement déclarée** :

```markdown
## Version 2.0.0

### ⚠️ BREAKING CHANGES

- **[BREAKING]** Suppression de la méthode `legacyMethod()`
  - Migration : Utiliser `newMethod()` à la place
  - Raison : Obsolescence et maintenance coûteuse

- **[BREAKING]** Modification du format de `DataStructure`
  - Migration : Voir guide de migration section 3.2
  - Raison : Performance et cohérence
```

---

## 7. Versionnement et transitions d'état

### 7.1 Impact des transitions sur le versionnement

| Transition | Impact sur la version |
|------------|----------------------|
| DRAFT → ACTIVE | Passage à version stable (1.0.0 si première) |
| ACTIVE → DEPRECATED | Aucun changement de numéro |
| DEPRECATED → RETIRED | Aucun changement de numéro |
| RETIRED → ARCHIVED | Aucun changement de numéro |
| DEPRECATED → ACTIVE (réactivation) | Aucun changement de numéro |

### 7.2 Correctifs sur versions dépréciées

Les versions **DEPRECATED** peuvent recevoir des correctifs de sécurité critiques :

| Type de correctif | Autorisé sur DEPRECATED |
|-------------------|------------------------|
| Correction de sécurité | ✅ Oui |
| Correction de bug critique | ✅ Oui |
| Correction de bug mineur | ❌ Non |
| Nouvelle fonctionnalité | ❌ Non |

Les versions **RETIRED** ne reçoivent aucun correctif.

---

## 8. Invariants de versionnement

### 8.1 Invariants applicables

Les invariants suivants d'Ever Buddy s'appliquent au versionnement :

| Invariant | Application au versionnement |
|-----------|------------------------------|
| **INV-EB-4** | Pas de passage direct ACTIVE → RETIRED sans version DEPRECATED |
| **INV-EB-5** | Rétrocompatibilité présumée sauf déclaration majeure explicite |
| **INV-EB-6** | Impact sur au moins deux générations de versions considéré |
| **INV-EB-7** | Chaque version documentée avec raison et chemin de migration |
| **INV-EB-9** | Règles de versionnement publiques et stables |
| **INV-EB-11** | Changements de règles non rétroactifs |

### 8.2 Violations de versionnement

Les actions suivantes sont des **violations** du contrat de versionnement :

| Violation | Gravité |
|-----------|---------|
| Changement majeur sans incrément majeur | 🔴 Critique |
| Rupture de compatibilité non documentée | 🔴 Critique |
| Renumérotation d'une version publiée | 🔴 Critique |
| Correctif incluant une nouvelle fonctionnalité | 🟡 Modérée |
| Version mineure sans rétrocompatibilité | 🔴 Critique |
| Absence de changelog | 🟡 Modérée |

---

## 9. Comparaison et ordering des versions

### 9.1 Règles de comparaison

Les versions sont comparées composant par composant, de gauche à droite :

```
1.9.0 < 1.10.0 < 2.0.0
1.0.0 < 1.0.1 < 1.1.0
```

### 9.2 Précédence

```
MAJEUR > MINEUR > CORRECTIF
```

**Exemple de tri :**
```
1.0.0 → 1.0.1 → 1.1.0 → 1.1.1 → 1.2.0 → 2.0.0
```

### 9.3 Versions pré-release (DRAFT)

Les versions DRAFT (0.x.x) sont toujours inférieures à leur première version stable :

```
0.9.9 < 1.0.0
```

---

## 10. Interactions avec les autres cores

### 10.1 Consultation par StrongFather

StrongFather peut consulter Ever Buddy pour connaître :
- La version actuelle d'un élément
- La compatibilité entre deux versions
- Les règles de migration applicables

### 10.2 Guidance pour BondingBrother

BondingBrother utilise les informations de versionnement pour :
- Adapter les traductions entre versions différentes
- Communiquer les avertissements de compatibilité
- Guider les produits dans leurs migrations

### 10.3 Information vers Master Butler

Master Butler est informé par Ever Buddy de :
- L'état de vie de chaque capacité versionnée
- Les versions supportées de chaque Tool
- Les compatibilités Tool ↔ Environnement

---

## 11. Conformité aux Lois d'Autonomie

### LOI-4 : Pas de temps global requis

Le versionnement sémantique respecte LOI-4 :
- Les versions sont des **numéros discrets**, pas des timestamps
- Les comparaisons de versions entre nœuds utilisent des **numéros**, pas des dates
- Les périodes de dépréciation sont définies en **cycles de release**, pas en temps absolu

### LOI-3 : L'état local est souverain

Chaque nœud maintient son propre registre de versions :
- La version locale est la vérité locale
- À la reconnexion, réconciliation explicite des versions

---

## 12. Résumé des règles de versionnement

| Règle | Description |
|-------|-------------|
| **R-VS-1** | Format : MAJEUR.MINEUR.CORRECTIF |
| **R-VS-2** | Majeur = rupture de compatibilité |
| **R-VS-3** | Mineur = ajout rétrocompatible |
| **R-VS-4** | Correctif = bug fix uniquement |
| **R-VS-5** | Numéros croissants uniquement |
| **R-VS-6** | Version 0.x.x = instable |
| **R-VS-7** | Changelog obligatoire |
| **R-VS-8** | Ruptures explicitement déclarées |
| **R-VS-9** | Catégorie détermine les périodes de dépréciation |
| **R-VS-10** | Rétrocompatibilité présumée par défaut |

---

## 13. Références croisées

- [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) — Document source de ce contrat
- [Ever Buddy - Compatibility Rules Contract](./Ever%20Buddy%20-%20Compatibility%20Rules%20Contract.md) — Règles de compatibilité détaillées
- [Ever Buddy - Lifecycle States Contract](../lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md) — États de cycle de vie
- [Ever Buddy - Transition Rules Contract](../lifecycle/Ever%20Buddy%20-%20Transition%20Rules%20Contract.md) — Règles de transition
- [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) — Définitions canoniques
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) — Conformité LOI-1 à LOI-6

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Contractuel normatif — VERSION SEMANTICS CONTRACT  
**Autorité :** Ever Buddy (Strate 4)  
**Source :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md), Section 4 — Concepts fondamentaux
