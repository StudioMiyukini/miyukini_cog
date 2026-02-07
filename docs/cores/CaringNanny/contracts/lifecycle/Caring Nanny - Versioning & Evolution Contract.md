# Caring Nanny - Versioning & Evolution Contract

## 1. Contexte

Ce document définit les règles de versionnement et d'évolution de Caring Nanny. Il établit comment Caring Nanny évolue dans le temps tout en préservant la stabilité de l'observation d'état, les invariants fondamentaux, et les garanties envers les consommateurs d'état du Miyukini Core System.

Ce document complète la Section 7 (Invariants non négociables) de la [Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) et s'appuie sur le document [Invariants et Garanties](../governance/Caring%20Nanny%20-%20Invariants%20et%20Garanties.md) pour définir les règles d'évolution des propriétés contractuelles.

L'évolution respecte les [Lois d'Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) : les nouvelles versions doivent maintenir l'autonomie locale (**LOI-1**, **LOI-2**, **LOI-3**), le fonctionnement sans temps global (**LOI-4**), et la proportionnalité des ressources (**LOI-5**).

## 2. Portée / Scope

Ce document couvre :
- Le schéma de versionnement (sémantique)
- Les règles d'évolution des contrats d'observation
- Les règles d'évolution des invariants et garanties
- Les règles de dépréciation
- Les règles de compatibilité ascendante
- Le processus d'évolution
- La gestion des breaking changes
- Les règles de gel

Ce document **ne couvre pas** :
- Les détails d'implémentation du versionnement
- Le versionnement des autorités (KindMother, StrongFather, BondingBrother)
- Les règles de test et validation (voir Testing & Validation Contract)
- Les règles de performance (voir Performance & Scalability Contract)

---

## 3. Principes fondamentaux

### 3.1 Observateur stable

**Principe EVOL-CN-01 : Observation stable**

L'interface d'observation de Caring Nanny est stable. Les consommateurs d'état peuvent compter sur la cohérence et la disponibilité de l'observation. Les changements rétro-incompatibles sont exceptionnels et suivent un processus formel.

**Implications :**
- Les composants interrogeant Caring Nanny continuent de fonctionner
- Les nouvelles catégories d'état sont additives
- Les breaking changes nécessitent une version majeure
- Période de dépréciation avant suppression

### 3.2 Évolution par extension

**Principe EVOL-CN-02 : Extension, pas modification**

Caring Nanny évolue par extension (ajout de fonctionnalités d'observation) plutôt que par modification (changement de fonctionnalités existantes).

**Implications :**
- Nouvelles catégories d'état (au-delà de healthy, degraded, offline, syncing, error)
- Nouvelles conditions détectables
- Nouveaux formats de notification
- Préservation des interfaces d'interrogation existantes
- Coexistence de plusieurs versions d'interfaces

### 3.3 Invariants immuables

**Principe EVOL-CN-03 : Invariants non négociables**

Les invariants de Caring Nanny ne changent jamais. Toute évolution doit les préserver.

**Invariants protégés (non modifiables) :**
- **INV-CN-1** : Observateur pur (pas de modification d'état)
- **INV-CN-2** : Aucune capacité d'exécution
- **INV-CN-3** : Non-autoritaire
- **INV-CN-4** : État cohérent
- **INV-CN-5** : Traçabilité complète
- **INV-CN-6** : Non-bloquant
- **INV-CN-7** : Propagation fidèle

**Implications :**
- Pas de modification des invariants fondamentaux
- Pas de compromis sur les invariants pour de nouvelles fonctionnalités
- Toute évolution est vérifiée contre les invariants

### 3.4 Conformité aux Lois d'Autonomie

**Principe EVOL-CN-04 : Autonomie préservée**

Toute évolution de Caring Nanny doit maintenir la conformité aux Lois d'Autonomie Système.

**Règles :**
- **LOI-1** : L'observation doit continuer à fonctionner localement sans dépendance externe
- **LOI-2** : L'état "offline" reste un état normal, pas une erreur
- **LOI-3** : L'historique local reste souverain
- **LOI-4** : Pas de temps global requis pour les nouvelles fonctionnalités
- **LOI-5** : Les nouvelles fonctionnalités respectent la proportionnalité des ressources

---

## 4. Schéma de versionnement

### 4.1 Version sémantique

Caring Nanny utilise le versionnement sémantique (Semantic Versioning) : `MAJEUR.MINEUR.PATCH`

**Format :** `v<MAJEUR>.<MINEUR>.<PATCH>[-<PRE-RELEASE>][+<BUILD>]`

**Exemples :**
- `v1.0.0` : Version majeure initiale
- `v1.1.0` : Nouvelle fonctionnalité d'observation (compatible)
- `v1.1.1` : Correction de bug (compatible)
- `v2.0.0` : Breaking change
- `v2.0.0-alpha.1` : Pré-version (alpha)
- `v2.0.0-beta.1` : Pré-version (beta)
- `v2.0.0+20260127` : Build avec métadonnées

### 4.2 Règles d'incrémentation

#### 4.2.1 Version MAJEUR (X.0.0)

**Incrémentation quand :**
- Breaking change de l'interface d'observation
- Modification d'une garantie envers les consommateurs
- Changement de comportement de propagation
- Suppression d'une catégorie d'état
- Modification du format de l'historique

**Règle R-VER-CN-1 : Breaking change formel**

Tout breaking change nécessite :
1. Justification documentée
2. Période de dépréciation (minimum 6 mois)
3. Plan de migration
4. Communication aux consommateurs d'état

#### 4.2.2 Version MINEUR (x.Y.0)

**Incrémentation quand :**
- Nouvelle catégorie d'état (additive, compatible)
- Nouvelle condition détectable
- Nouveau type de notification
- Extension de l'interface d'observation (champs optionnels)
- Nouvelle garantie (sans modification des existantes)

**Règle R-VER-CN-2 : Compatibilité ascendante**

Les versions mineures sont rétro-compatibles :
- Les consommateurs utilisant une version mineure antérieure continuent de fonctionner
- Les nouvelles fonctionnalités d'observation sont optionnelles
- Les catégories d'état existantes ne sont pas modifiées

#### 4.2.3 Version PATCH (x.y.Z)

**Incrémentation quand :**
- Correction de bug d'observation
- Amélioration de performance (sans changement d'interface)
- Correction de documentation
- Correction de formulation

**Règle R-VER-CN-3 : Pas de changement d'interface**

Les versions patch ne modifient jamais l'interface d'observation publique.

---

## 5. Versionnement des composants conceptuels

### 5.1 Interface d'observation

**Composants conceptuels :** Observer, StateAggregator, TransitionDetector, Propagator, HistoryKeeper

**Règles :**
- Versionnés ensemble avec Caring Nanny
- Breaking change = nouvelle version MAJEUR
- Extension = nouvelle version MINEUR
- Correction = nouvelle version PATCH

### 5.2 Catégories d'état

**États actuels :** healthy, degraded, offline, syncing, error

**Règle R-VER-CN-4 : Extension des catégories**

- Ajout d'une nouvelle catégorie : version MINEUR
- Modification de la sémantique d'une catégorie : version MAJEUR
- Suppression d'une catégorie : version MAJEUR (avec dépréciation préalable)

**Exemple :**
- `v1.0.0` : États de base (healthy, degraded, offline, syncing, error)
- `v1.1.0` : Ajout de `maintenance` (nouvel état)
- `v2.0.0` : Modification de la sémantique de `error` (breaking)

### 5.3 Format de l'historique

**Composant :** Structure des observations enregistrées

**Règles :**
- Ajout de champs optionnels : version MINEUR
- Modification de champs existants : version MAJEUR
- Changement de format de stockage : version MAJEUR

### 5.4 Contrats internes

**Documents :** Invariants & Garanties, State Model Contract, Observation Flow Contract, Propagation Flow Contract

**Règle R-VER-CN-5 : Alignement des contrats**

Les documents contractuels sont versionnés et alignés avec les versions de Caring Nanny :
- Document `v1.0.0` correspond à Caring Nanny `v1.0.0`
- Document `v2.0.0` correspond à Caring Nanny `v2.0.0`

---

## 6. Règles de compatibilité

### 6.1 Compatibilité ascendante

**Règle R-COMP-CN-1 : Rétrocompatibilité**

Les versions mineures et patch sont rétro-compatibles :
- Un consommateur utilisant `v1.0.0` fonctionne avec Caring Nanny `v1.5.0`
- Un consommateur utilisant `v1.5.0` fonctionne avec Caring Nanny `v1.0.0` (sauf nouvelles fonctionnalités)

**Garanties :**
- Les interfaces d'observation existantes ne sont pas modifiées
- Les catégories d'état existantes sont préservées
- Les comportements de propagation existants sont préservés
- Les nouvelles fonctionnalités sont optionnelles

### 6.2 Compatibilité descendante

**Règle R-COMP-CN-2 : Pas de garantie descendante**

Caring Nanny ne garantit pas la compatibilité descendante :
- Un consommateur utilisant `v2.0.0` peut ne pas fonctionner avec Caring Nanny `v1.5.0`
- Les consommateurs doivent utiliser une version compatible de Caring Nanny

**Implications :**
- Les consommateurs doivent spécifier la version minimale requise
- Les breaking changes sont documentés
- Les migrations sont guidées

### 6.3 Coexistence de versions

**Règle R-COMP-CN-3 : Support multi-version**

Caring Nanny peut supporter plusieurs versions d'interfaces simultanément pendant les périodes de transition.

**Durée de support :**
- Version N : Supportée (actuelle)
- Version N-1 : Supportée (minimum 12 mois après version N)
- Version N-2 : Dépréciée (support limité)
- Version N-3 : Non supportée

---

## 7. Évolution des invariants

### 7.1 Règles d'évolution des invariants

**Règle R-EVOL-INV-CN-1 : Ajout d'invariant**

Un nouvel invariant peut être ajouté dans une version MINEUR s'il :
- N'affaiblit aucun invariant existant
- N'introduit pas d'incompatibilité pour les consommateurs
- Est documenté et justifié
- Préserve la nature d'observateur pur

**Règle R-EVOL-INV-CN-2 : Modification d'invariant**

Un invariant existant ne peut être modifié que dans une version MAJEUR avec :
- Justification de la modification
- Guide de migration
- Période de dépréciation si applicable

**Règle R-EVOL-INV-CN-3 : Suppression d'invariant**

Un invariant existant ne peut être supprimé que dans une version MAJEUR après :
- Dépréciation dans au moins deux versions MINEUR
- Justification de la suppression
- Guide de migration

**Règle R-EVOL-INV-CN-4 : Invariants fondamentaux**

Les invariants fondamentaux (INV-CN-1 à INV-CN-7) ne peuvent **jamais** être modifiés ou supprimés. Ils définissent l'essence de Caring Nanny.

### 7.2 Garanties d'évolution des invariants

**G-EVOL-INV-CN-1 : Compatibilité préservée**

L'ajout d'un invariant ne peut pas rendre non conforme une implémentation conforme.

**G-EVOL-INV-CN-2 : Dépréciation avant suppression**

Tout invariant supprimé doit avoir été déprécié au préalable (sauf invariants fondamentaux qui ne peuvent être supprimés).

---

## 8. Évolution des garanties

### 8.1 Règles d'évolution des garanties

**Règle R-EVOL-GAR-CN-1 : Ajout de garantie**

Une nouvelle garantie peut être ajoutée dans une version MINEUR si elle :
- N'affaiblit aucune garantie existante
- N'introduit pas d'incompatibilité
- Est documentée et justifiée

**Règle R-EVOL-GAR-CN-2 : Modification de garantie**

Une garantie existante ne peut être modifiée que dans une version MAJEUR avec :
- Justification de la modification
- Guide de migration
- Période de dépréciation si applicable

**Règle R-EVOL-GAR-CN-3 : Suppression de garantie**

Une garantie existante ne peut être supprimée que dans une version MAJEUR après :
- Dépréciation dans au moins deux versions MINEUR
- Justification de la suppression
- Guide de migration

**Règle R-EVOL-GAR-CN-4 : Garanties fondamentales**

Les garanties envers les autorités (GAR-AUTH-*) et les garanties de cohérence (GAR-CONS-01 à GAR-CONS-04) ne peuvent jamais être modifiées ou supprimées.

### 8.2 Garanties d'évolution des garanties

**G-EVOL-GAR-CN-1 : Compatibilité préservée**

L'ajout d'une garantie ne peut pas rendre non conforme une implémentation conforme.

**G-EVOL-GAR-CN-2 : Dépréciation avant suppression**

Toute garantie supprimée doit avoir été dépréciée au préalable (sauf garanties fondamentales qui ne peuvent être supprimées).

---

## 9. Dépréciation

### 9.1 Marquage de dépréciation

**Règle R-DEPR-CN-1 : Marquage explicite**

Tout élément déprécié est marqué explicitement avec :
- Le statut DÉPRÉCIÉ
- La version de dépréciation
- La version de suppression prévue
- La raison de la dépréciation
- Les instructions de migration

**Format de marquage :**
```
@deprecated Depuis v1.5.0, sera supprimé en v2.0.0. 
Raison : [justification]
Migration : [instructions]
```

### 9.2 Période de dépréciation

**Règle R-DEPR-CN-2 : Minimum 6 mois**

La période de dépréciation est d'au minimum 6 mois, recommandé 12 mois pour les éléments largement utilisés.

**Calcul :**
- Date de dépréciation : Date de publication de la version avec marquage DÉPRÉCIÉ
- Date de suppression : Date de publication de la version MAJEUR suivante
- Période : Minimum 6 mois entre les deux

### 9.3 Communication de dépréciation

**Règle R-DEPR-CN-3 : Communication proactive**

La dépréciation est communiquée :
- Dans les notes de version
- Dans la documentation
- Dans les logs d'observation (si applicable)

**Contenu :**
- Ce qui est déprécié
- Pourquoi c'est déprécié
- Quand ce sera supprimé
- Comment migrer

### 9.4 Cas interdits de dépréciation

**Règle R-DEPR-CN-4 : Éléments non dépréciables**

Les éléments suivants ne peuvent jamais être dépréciés :
- Invariants fondamentaux (INV-CN-1 à INV-CN-7)
- Garanties fondamentales (GAR-AUTH-*, GAR-CONS-01 à GAR-CONS-04)
- Nature d'observateur pur
- Distinction avec les autorités (KindMother, StrongFather, BondingBrother)

---

## 10. Migration conceptuelle

### 10.1 Types de migrations

**MIG-TYPE-CN-1 : Migration automatique**

Une migration est **automatique** si elle ne nécessite aucune modification pour les consommateurs d'état.

**Exemple :** Ajout d'une nouvelle catégorie d'état optionnelle.

**MIG-TYPE-CN-2 : Migration guidée**

Une migration est **guidée** si elle nécessite des modifications documentées.

**Exemple :** Changement de format d'une notification de changement d'état.

**MIG-TYPE-CN-3 : Migration majeure**

Une migration est **majeure** si elle nécessite une adaptation significative des consommateurs.

**Exemple :** Passage d'une version MAJEUR avec modifications de l'interface d'observation.

### 10.2 Processus de migration

**Phase 1 : Analyse**
1. Identification des changements incompatibles
2. Évaluation de l'impact sur les consommateurs d'état
3. Définition du plan de migration

**Phase 2 : Documentation**
1. Rédaction du guide de migration
2. Documentation des changements
3. Création des tests de migration

**Phase 3 : Implémentation**
1. Adaptation de l'implémentation
2. Exécution des tests de migration
3. Vérification de la conformité

**Phase 4 : Validation**
1. Tests de conformité aux invariants
2. Validation de la migration
3. Certification de conformité

### 10.3 Garanties de migration

**G-MIG-CN-1 : Guide disponible**

Un guide de migration est toujours disponible pour toute version MAJEUR.

**G-MIG-CN-2 : Migration testable**

Toute migration peut être vérifiée par des tests de conformité.

**G-MIG-CN-3 : Support de transition**

Un support de transition est fourni pendant la période de migration (minimum 12 mois).

---

## 11. Règles de gel

### 11.1 Définition du gel

**Définition :**

Le **gel** est l'état d'un contrat Caring Nanny où aucune modification n'est autorisée, garantissant la stabilité absolue du contrat.

**Caractéristiques :**
- **Immutabilité** : Un contrat gelé ne peut plus être modifié
- **Stabilité** : Un contrat gelé garantit la stabilité contractuelle
- **Irréversibilité** : Un gel ne peut pas être annulé
- **Permanence** : Un contrat gelé reste gelé définitivement

### 11.2 Conditions de gel

**Règle R-GEL-CN-1 : Gel après stabilisation**

Un contrat peut être gelé après une période de stabilisation et de validation complète.

**Règle R-GEL-CN-2 : Gel par décision formelle**

Le gel d'un contrat est une décision architecturale formelle, documentée et irréversible.

**Règle R-GEL-CN-3 : Gel des contrats fondateurs**

Les contrats fondateurs (Documentation Fondatrice, Invariants & Garanties) peuvent être gelés après validation complète.

### 11.3 Règles de gel

**Règle R-GEL-CN-4 : Aucune modification autorisée**

Un contrat gelé ne peut plus être modifié, même pour des corrections mineures.

**Règle R-GEL-CN-5 : Nouvelle version pour évolution**

Toute évolution d'un contrat gelé nécessite la création d'un nouveau contrat ou d'une nouvelle version MAJEUR.

**Règle R-GEL-CN-6 : Documentation du gel**

Le gel d'un contrat DOIT être documenté avec :
- La date de gel
- La version gelée
- La raison du gel
- Les implications du gel

### 11.4 Garanties de gel

**G-GEL-CN-1 : Immutabilité garantie**

Un contrat gelé ne peut jamais être modifié.

**G-GEL-CN-2 : Stabilité garantie**

Un contrat gelé garantit la stabilité contractuelle absolue.

**G-GEL-CN-3 : Compatibilité préservée**

Un contrat gelé reste compatible avec toutes les implémentations conformes.

---

## 12. Processus d'évolution

### 12.1 Proposition d'évolution

**Étape 1 : Proposition**
- Description de l'évolution
- Justification (besoin, bénéfice)
- Impact (consommateurs, autorités, invariants)
- Plan de migration (si breaking change)

**Étape 2 : Vérification**
- Vérification contre les invariants (INV-CN-1 à INV-CN-7)
- Vérification de conformité aux Lois d'Autonomie
- Vérification de compatibilité
- Validation architecturale

### 12.2 Implémentation

**Étape 3 : Implémentation**
- Développement selon les règles d'évolution
- Tests de compatibilité
- Tests de régression
- Documentation

**Étape 4 : Dépréciation (si nécessaire)**
- Marquage DÉPRÉCIÉ
- Communication aux consommateurs
- Période de dépréciation

### 12.3 Publication

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

## 13. Gestion des breaking changes

### 13.1 Types de breaking changes

**Breaking change d'interface :**
- Modification du format d'interrogation
- Suppression d'une méthode d'observation
- Modification de la structure des réponses

**Breaking change de contrat :**
- Modification d'une garantie
- Changement de format de l'historique
- Modification de la sémantique d'une catégorie d'état

**Breaking change de comportement :**
- Modification du flux d'observation
- Modification du flux de propagation
- Changement de règles d'agrégation

### 13.2 Processus de breaking change

**Règle R-BREAK-CN-1 : Processus formel**

Tout breaking change suit un processus formel :

1. **Justification :** Pourquoi ce breaking change est nécessaire
2. **Impact analysis :** Quels consommateurs sont affectés
3. **Vérification invariants :** Conformité aux invariants fondamentaux
4. **Plan de migration :** Comment migrer
5. **Période de dépréciation :** Minimum 6 mois
6. **Communication :** Annonce, documentation, support
7. **Version MAJEUR :** Incrémentation obligatoire

### 13.3 Exceptions

**Exception BREAK-CN-EXCEPT-1 : Sécurité critique**

En cas de vulnérabilité de sécurité critique, un breaking change peut être appliqué immédiatement avec version MAJEUR, mais avec communication urgente et support de migration.

**Exception BREAK-CN-EXCEPT-2 : Correction d'invariant violé**

Si un invariant est violé par erreur dans une version précédente, la correction (qui peut être un breaking change) est appliquée avec version MAJEUR et communication.

---

## 14. Exemples

### 14.1 Évolution mineure (v1.0.0 → v1.1.0)

**Changement :** Ajout d'une nouvelle catégorie d'état `maintenance`

**Impact :** Aucun (additif)

**Compatibilité :** Rétro-compatible

**Migration :** Aucune nécessaire (les consommateurs qui ne connaissent pas `maintenance` continuent de fonctionner)

### 14.2 Évolution majeure (v1.5.0 → v2.0.0)

**Changement :** Modification du format de notification de changement d'état

**Processus :**
1. v1.5.0 : Ancien format marqué `@deprecated`, nouveau format ajouté
2. v1.6.0 - v1.9.0 : Période de dépréciation (12 mois)
3. v2.0.0 : Ancien format supprimé

**Impact :** Consommateurs utilisant l'ancien format doivent migrer

**Migration :** Guide de migration fourni

### 14.3 Évolution patch (v1.5.0 → v1.5.1)

**Changement :** Correction d'un bug dans la détection de transition

**Impact :** Aucun (correction)

**Compatibilité :** Rétro-compatible

**Migration :** Aucune nécessaire

---

## 15. Règles de fermeture du contrat

### 15.1 Contrat fermé

Ce contrat est **fermé**. Seules les règles de versioning, compatibilité, dépréciation, migration, et gel explicitement définies sont valides.

### 15.2 Interdiction d'extension implicite

Aucune extension implicite des règles d'évolution n'est autorisée. Toute nouvelle règle doit être ajoutée explicitement via une nouvelle version du contrat.

---

## 16. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit les règles de versionnement et d'évolution de Caring Nanny qui doivent être respectées pour garantir la stabilité et l'évolutivité.

Toute évolution de Caring Nanny doit respecter ces règles. Toute violation doit être corrigée ou justifiée par une exception documentée.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** CONTRAT — Normatif  
**Dépendances :**
- [Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) v1.6 (Section 7)
- [Invariants et Garanties](../governance/Caring%20Nanny%20-%20Invariants%20et%20Garanties.md) v1.0
- [Lois d'Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)
