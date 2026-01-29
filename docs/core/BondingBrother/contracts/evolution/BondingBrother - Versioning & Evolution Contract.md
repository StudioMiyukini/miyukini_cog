# BondingBrother - Versioning & Evolution Contract

## 1. Contexte

Ce document définit les règles de versionnement et d'évolution de Bonding Brother. Il établit comment Bonding Brother évolue dans le temps tout en préservant la stabilité de l'interface pour les produits et en respectant les invariants fondamentaux.

Ce document complète la Section 7 de la [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur le [Product Interface Contract](../product/BondingBrother%20-%20Product%20Interface%20Contract.md) et l'[Extension & Specialization Contract](../product/BondingBrother%20-%20Extension%20&%20Specialization%20Contract.md) pour définir les règles d'évolution de l'interface.

L'évolution respecte les [Lois d'Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) : les nouvelles versions doivent maintenir l'autonomie locale (**LOI-1**, **LOI-2**, **LOI-3**).

**Navigation :** [Index BondingBrother](../../_index.md)

## 2. Portée / Scope

Ce document couvre :
- Le schéma de versionnement (sémantique)
- Les règles d'évolution de l'interface
- Les règles d'évolution des contrats internes
- Les règles de dépréciation
- Les règles de compatibilité
- Le processus d'évolution
- La gestion des breaking changes

Ce document **ne couvre pas** :
- Les règles de migration (voir [Migration & Compatibility Contract](./BondingBrother%20-%20Migration%20&%20Compatibility%20Contract.md))
- Les détails d'implémentation du versionnement
- Le versionnement des autorités (Kind Mother, Strong Father)

---

## 3. Principes fondamentaux

### 3.1 Stabilité avant tout

**Principe EVOL-01 : Interface stable**

L'interface de Bonding Brother vers les produits est stable. Les changements rétro-incompatibles sont exceptionnels et suivent un processus formel.

**Implications :**
- Les produits existants continuent de fonctionner
- Les nouvelles fonctionnalités sont additives
- Les breaking changes nécessitent une version majeure
- Période de dépréciation avant suppression

### 3.2 Évolution par extension

**Principe EVOL-02 : Extension, pas modification**

Bonding Brother évolue par extension (ajout de fonctionnalités) plutôt que par modification (changement de fonctionnalités existantes).

**Implications :**
- Nouvelles interfaces spécialisées
- Nouvelles capacités optionnelles
- Préservation des interfaces existantes
- Coexistence de plusieurs versions d'interfaces

### 3.3 Invariants immuables

**Principe EVOL-03 : Invariants non négociables**

Les invariants de Bonding Brother ne changent jamais. Toute évolution doit les préserver.

**Implications :**
- Pas de modification des invariants
- Pas de compromis sur les invariants pour de nouvelles fonctionnalités
- Toute évolution est vérifiée contre les invariants

---

## 4. Schéma de versionnement

### 4.1 Version sémantique

Bonding Brother utilise le versionnement sémantique (Semantic Versioning) : `MAJOR.MINOR.PATCH`

**Format :** `v<MAJOR>.<MINOR>.<PATCH>[-<PRE-RELEASE>][+<BUILD>]`

**Exemples :**
- `v1.0.0` : Version majeure initiale
- `v1.1.0` : Nouvelle fonctionnalité (compatible)
- `v1.1.1` : Correction de bug (compatible)
- `v2.0.0` : Breaking change
- `v2.0.0-alpha.1` : Pré-version (alpha)
- `v2.0.0-beta.1` : Pré-version (beta)
- `v2.0.0+20260126` : Build avec métadonnées

### 4.2 Règles d'incrémentation

#### 4.2.1 Version MAJOR (X.0.0)

**Incrémentation quand :**
- Breaking change de l'interface produit
- Modification d'un invariant (interdit, voir EVOL-03)
- Changement de comportement contractuel majeur
- Suppression d'une interface publique

**Règle MAJOR-01 : Breaking change formel**

Tout breaking change nécessite :
1. Justification documentée
2. Période de dépréciation (minimum 6 mois)
3. Plan de migration
4. Communication aux produits

#### 4.2.2 Version MINOR (x.Y.0)

**Incrémentation quand :**
- Nouvelle fonctionnalité (additive, compatible)
- Nouvelle interface spécialisée
- Nouvelle capacité optionnelle
- Extension d'une interface existante (champs optionnels)

**Règle MINOR-01 : Compatibilité ascendante**

Les versions mineures sont rétro-compatibles :
- Les produits utilisant une version mineure antérieure continuent de fonctionner
- Les nouvelles fonctionnalités sont optionnelles
- Les champs existants ne sont pas modifiés

#### 4.2.3 Version PATCH (x.y.Z)

**Incrémentation quand :**
- Correction de bug
- Correction de sécurité
- Amélioration de performance (sans changement d'interface)
- Correction de documentation

**Règle PATCH-01 : Pas de changement d'interface**

Les versions patch ne modifient jamais l'interface publique.

---

## 5. Versionnement des composants

### 5.1 Interface produit

**Composant :** `ProductGateway`, `IIntentSubmission`, `IResultConsumption`, `INotificationSubscription`

**Règles :**
- Versionnée indépendamment : `v<MAJOR>.<MINOR>.<PATCH>`
- Breaking change = nouvelle version MAJOR
- Extension = nouvelle version MINOR
- Correction = nouvelle version PATCH

**Exemple :**
- `IIntentSubmission v1.0.0` : Interface initiale
- `IIntentSubmission v1.1.0` : Nouveau champ optionnel
- `IIntentSubmission v2.0.0` : Champ obligatoire modifié (breaking)

### 5.2 Contrats internes

**Composants :** `ITranslation`, `IFiltering`, `IJournaling`, `IAuthorityRouting`

**Règles :**
- Versionnés indépendamment
- Changements internes n'affectent pas la version de l'interface produit
- Breaking change interne = nouvelle version MAJOR du contrat interne

**Exemple :**
- `ITranslation v1.0.0` : Contrat initial
- `ITranslation v2.0.0` : Nouvelle méthode obligatoire (breaking interne)
- Interface produit reste `v1.x.x` si compatible

### 5.3 Configuration

**Composant :** `ConfigurationStore`, règles de configuration

**Règles :**
- Versionnée avec Bonding Brother
- Changements de format de configuration = version MAJOR
- Nouvelles options = version MINOR
- Corrections = version PATCH

---

## 6. Règles d'évolution de l'interface

### 6.1 Ajout de fonctionnalités

**Règle EVOL-IFACE-01 : Additif uniquement**

Les nouvelles fonctionnalités sont ajoutées sans modifier les fonctionnalités existantes.

**Autorisé :**
- Nouvelle méthode dans une interface
- Nouveau champ optionnel dans une structure
- Nouveau type d'intention
- Nouvelle capacité optionnelle

**Interdit :**
- Modification d'une méthode existante
- Suppression d'une méthode existante
- Modification d'un champ existant (sauf version MAJOR)
- Changement de signature

### 6.2 Modification de fonctionnalités

**Règle EVOL-IFACE-02 : Dépréciation puis modification**

Avant de modifier une fonctionnalité existante :
1. Dépréciation (marquage comme `@deprecated`)
2. Période de dépréciation (minimum 6 mois)
3. Communication aux produits
4. Modification dans version MAJOR suivante

**Processus :**
```
v1.0.0 : Méthode `createContent()` disponible
v1.1.0 : Méthode `createContent()` marquée @deprecated, nouvelle méthode `createContentV2()` ajoutée
v1.x.x : Période de dépréciation (6+ mois)
v2.0.0 : Méthode `createContent()` supprimée, `createContentV2()` devient `createContent()`
```

### 6.3 Suppression de fonctionnalités

**Règle EVOL-IFACE-03 : Dépréciation obligatoire**

Aucune fonctionnalité publique n'est supprimée sans dépréciation préalable.

**Processus :**
1. Marquage `@deprecated` avec message d'avertissement
2. Documentation de la migration
3. Période de dépréciation (minimum 6 mois, recommandé 12 mois)
4. Suppression dans version MAJOR suivante

### 6.4 Extension par spécialisation

**Règle EVOL-IFACE-04 : Spécialisation autorisée**

De nouvelles interfaces spécialisées peuvent être créées pour étendre les capacités sans modifier les interfaces existantes.

**Exemple :**
- `IIntentSubmission v1.0.0` : Interface de base
- `IAdvancedIntentSubmission v1.0.0` : Interface spécialisée (hérite de `IIntentSubmission`)
- Les deux coexistent, les produits peuvent choisir

---

## 7. Règles de compatibilité

### 7.1 Compatibilité ascendante

**Règle COMPAT-01 : Rétrocompatibilité**

Les versions mineures et patch sont rétro-compatibles :
- Un produit utilisant `v1.0.0` fonctionne avec Bonding Brother `v1.5.0`
- Un produit utilisant `v1.5.0` fonctionne avec Bonding Brother `v1.0.0` (sauf nouvelles fonctionnalités)

**Garanties :**
- Les interfaces existantes ne sont pas modifiées
- Les comportements existants sont préservés
- Les nouvelles fonctionnalités sont optionnelles

### 7.2 Compatibilité descendante

**Règle COMPAT-02 : Pas de garantie descendante**

Bonding Brother ne garantit pas la compatibilité descendante :
- Un produit utilisant `v2.0.0` peut ne pas fonctionner avec Bonding Brother `v1.5.0`
- Les produits doivent utiliser une version compatible de Bonding Brother

**Implications :**
- Les produits doivent spécifier la version minimale requise
- Les breaking changes sont documentés
- Les migrations sont guidées

### 7.3 Coexistence de versions

**Règle COMPAT-03 : Multi-version supportée**

Bonding Brother peut supporter plusieurs versions d'interfaces simultanément :
- `IIntentSubmission v1.0.0` : Supportée
- `IIntentSubmission v2.0.0` : Supportée
- Les deux coexistent, routage selon la version utilisée par le produit

**Durée de support :**
- Version N : Supportée
- Version N-1 : Supportée (minimum 12 mois après version N)
- Version N-2 : Dépréciée (support limité)
- Version N-3 : Non supportée

---

## 8. Processus d'évolution

### 8.1 Proposition d'évolution

**Étape 1 : Proposition**
- Description de l'évolution
- Justification (besoin, bénéfice)
- Impact (produits, autorités, invariants)
- Plan de migration (si breaking change)

**Étape 2 : Revue**
- Vérification contre les invariants
- Vérification de compatibilité
- Validation architecturale
- Approbation

### 8.2 Implémentation

**Étape 3 : Implémentation**
- Développement selon les règles d'évolution
- Tests de compatibilité
- Tests de régression
- Documentation

**Étape 4 : Dépréciation (si nécessaire)**
- Marquage `@deprecated`
- Communication aux produits
- Période de dépréciation

### 8.3 Publication

**Étape 5 : Release**
- Versionnement selon les règles
- Notes de version
- Documentation de migration
- Communication

**Étape 6 : Support**
- Support de la nouvelle version
- Support des versions précédentes (selon politique)
- Monitoring des migrations

---

## 9. Gestion des breaking changes

### 9.1 Types de breaking changes

**Breaking change d'interface :**
- Modification de signature de méthode
- Suppression de méthode
- Modification de champ obligatoire
- Changement de comportement contractuel

**Breaking change de contrat :**
- Modification d'un invariant (interdit, voir EVOL-03)
- Modification d'une garantie
- Changement de format de journal

**Breaking change de configuration :**
- Modification de format de configuration
- Suppression d'option de configuration
- Changement de valeur par défaut (si impactant)

### 9.2 Processus de breaking change

**Règle BREAK-01 : Processus formel**

Tout breaking change suit un processus formel :

1. **Justification :** Pourquoi ce breaking change est nécessaire
2. **Impact analysis :** Quels produits sont affectés
3. **Plan de migration :** Comment migrer
4. **Période de dépréciation :** Minimum 6 mois
5. **Communication :** Annonce, documentation, support
6. **Version MAJOR :** Incrémentation obligatoire

### 9.3 Exceptions

**Exception BREAK-EXCEPT-01 : Sécurité critique**

En cas de vulnérabilité de sécurité critique, un breaking change peut être appliqué immédiatement avec version MAJOR, mais avec communication urgente et support de migration.

**Exception BREAK-EXCEPT-02 : Correction d'invariant violé**

Si un invariant est violé par erreur dans une version précédente, la correction (qui peut être un breaking change) est appliquée avec version MAJOR et communication.

---

## 10. Dépréciation

### 10.1 Marquage de dépréciation

**Règle DEPREC-01 : Marquage explicite**

Toute fonctionnalité dépréciée est marquée explicitement :
- Annotation `@deprecated` dans le code
- Documentation de dépréciation
- Message d'avertissement dans les logs
- Date de suppression prévue

**Format de message :**
```
@deprecated Since v1.5.0, will be removed in v2.0.0. Use createContentV2() instead.
```

### 10.2 Période de dépréciation

**Règle DEPREC-02 : Minimum 6 mois**

La période de dépréciation est d'au minimum 6 mois, recommandé 12 mois.

**Calcul :**
- Date de dépréciation : Date de publication de la version avec `@deprecated`
- Date de suppression : Date de publication de la version MAJOR suivante
- Période : Minimum 6 mois entre les deux

### 10.3 Communication de dépréciation

**Règle DEPREC-03 : Communication proactive**

La dépréciation est communiquée :
- Dans les notes de version
- Dans la documentation
- Via des alertes (si configuré)
- Via le support (si contact)

**Contenu :**
- Ce qui est déprécié
- Pourquoi c'est déprécié
- Quand ce sera supprimé
- Comment migrer

---

## 11. Versionnement des documents

### 11.1 Documents contractuels

**Règle DOC-VER-01 : Versionnement aligné**

Les documents contractuels sont versionnés et alignés avec les versions de Bonding Brother :
- Document `v1.0.0` correspond à Bonding Brother `v1.0.0`
- Document `v2.0.0` correspond à Bonding Brother `v2.0.0`

**Format :**
- En-tête du document : `Version : 1.0`
- Historique des versions dans le document
- Liens vers versions précédentes

### 11.2 Évolution des documents

**Règle DOC-VER-02 : Préservation de l'historique**

Les documents évoluent en préservant l'historique :
- Nouvelle version = nouveau document ou section
- Anciennes versions restent accessibles
- Changelog documenté

---

## 12. Exemples

### 12.1 Évolution mineure (v1.0.0 → v1.1.0)

**Changement :** Ajout d'un nouveau type d'intention `SYNC_CONTENT`

**Impact :** Aucun (additif)

**Compatibilité :** Rétro-compatible

**Migration :** Aucune nécessaire

### 12.2 Évolution majeure (v1.5.0 → v2.0.0)

**Changement :** Suppression de la méthode `createContent()`, remplacement par `createContentV2()`

**Processus :**
1. v1.5.0 : `createContent()` marquée `@deprecated`
2. v1.6.0 - v1.9.0 : Période de dépréciation (12 mois)
3. v2.0.0 : `createContent()` supprimée, `createContentV2()` devient `createContent()`

**Impact :** Produits utilisant `createContent()` doivent migrer

**Migration :** Guide de migration fourni

---

## 13. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit les règles de versionnement et d'évolution de Bonding Brother qui doivent être respectées pour garantir la stabilité et l'évolutivité.

Toute évolution de Bonding Brother doit respecter ces règles. Toute violation doit être corrigée ou justifiée par une exception documentée.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- [Documentation Fondatrice v1.0](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) (Section 7)
- [Product Interface Contract v1.0](../product/BondingBrother%20-%20Product%20Interface%20Contract.md)
- [Extension & Specialization Contract v1.0](../product/BondingBrother%20-%20Extension%20&%20Specialization%20Contract.md)
- [Architecture & Flows v1.0](../../architecture/BondingBrother%20-%20Architecture%20&%20Flows.md)
