# Ever Buddy - FAQ & Common Questions

## Contexte

Ce document répond aux **questions fréquemment posées** concernant Ever Buddy, le core de cycle de vie et d'évolution du Miyukini Core System. Il synthétise les interrogations courantes des architectes, développeurs, et intégrateurs qui travaillent avec Ever Buddy.

Les réponses sont dérivées de la **Documentation Fondatrice** et des contrats normatifs d'Ever Buddy. Ce document ne crée pas de nouvelles règles — il clarifie l'existant.

**Document de référence :** [Ever Buddy - Documentation Fondatrice](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)

---

## Portée / Scope

- **Applicable à :** Toute personne travaillant avec Ever Buddy
- **Audience :** Architectes, développeurs, intégrateurs, équipes produit
- **Statut :** Document de référence — Informatif
- **Dépendances :** Documentation Fondatrice Ever Buddy, Glossaire Miyukini

---

## 1. Questions générales sur Ever Buddy

### Q1.1 : Qu'est-ce qu'Ever Buddy exactement ?

**Ever Buddy est le core de cycle de vie et d'évolution** (Strate 4) du Miyukini Core System. Il représente la **conscience temporelle** du système : il observe ce qui a été, ce qui est, et ce qui sera.

**Rôle principal :** Gouverner l'évolution des structures, des contrats, et des entités dans le temps, sans jamais exécuter de migration technique ou modifier directement les données.

**Question fondamentale :** *"Comment le système évolue-t-il sans jamais se rompre ?"*

**Référence :** [Documentation Fondatrice, Section 1](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#1-introduction)

---

### Q1.2 : Pourquoi Ever Buddy existe-t-il ? Quel problème résout-il ?

Ever Buddy résout les problèmes liés à l'évolution non contrôlée des systèmes :

| Problème | Solution Ever Buddy |
|----------|---------------------|
| **Ruptures non contrôlées** | États de cycle de vie explicites et transitions validées |
| **Dette structurelle invisible** | Surveillance continue du debt ratio |
| **Transitions brutales** | Période de dépréciation obligatoire (INV-EB-4) |
| **Perte de mémoire** | Traçabilité complète et immuable (INV-EB-2) |
| **Évolutions contradictoires** | Gouvernance centralisée des évolutions |

**Référence :** [Documentation Fondatrice, Section 2](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#2-raison-dêtre)

---

### Q1.3 : Quelle est la phrase fondatrice d'Ever Buddy ?

> **Ever Buddy est le compagnon de toujours qui observe, enregistre, et guide l'évolution du système, garantissant que chaque changement respecte la continuité, que chaque transition est traçable, et que l'avenir est préparé sans sacrifier le présent.**

Cette phrase résume l'essence d'Ever Buddy :
- **Compagnon** : Présent mais non autoritaire
- **Observateur** : Pas exécuteur
- **Guide** : Influence sans contrainte
- **Gardien de la continuité** : Vision long terme

**Référence :** [Documentation Fondatrice, Section 11](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#11-conclusion-et-statut-contractuel)

---

## 2. Questions sur les états de cycle de vie

### Q2.1 : Quels sont les états de cycle de vie possibles ?

Ever Buddy définit **cinq états de cycle de vie** :

| État | Description | Production | Stabilité |
|------|-------------|------------|-----------|
| **DRAFT** | En cours de définition | ❌ | ❌ |
| **ACTIVE** | En usage normal, supporté | ✅ | ✅ |
| **DEPRECATED** | Fonctionnel mais usage découragé | ⚠️ | ✅ |
| **RETIRED** | Non supporté, corrections critiques seulement | ⚠️ | ⚠️ |
| **ARCHIVED** | Non fonctionnel, référence historique | ❌ | ❌ |

**Référence Glossaire :** [DRAFT](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#brouillon-draft--état-de-vie), [ACTIVE](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#actif-active--état-de-vie), [DEPRECATED](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#déprécié-deprecated--état-de-vie), [RETIRED](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#retiré-retired--état-de-vie)

**Référence contrat :** [Lifecycle States Contract](../contracts/lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md)

---

### Q2.2 : Un élément peut-il être dans plusieurs états à la fois ?

**Non, jamais.** L'invariant INV-EB-3 garantit :

> Chaque élément du système possède **exactement un** état de cycle de vie à tout moment. Il n'existe pas d'état intermédiaire, incertain, ou non défini.

Les transitions sont **atomiques** : un élément passe de l'état A à l'état B sans état transitoire.

**Violations détectées :**
- Un élément sans état déclaré
- Un élément avec plusieurs états simultanés
- Un élément dans un état "en transition"

**Référence :** [Documentation Fondatrice, INV-EB-3](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#inv-eb-3--aucun-état-ambigu)

---

### Q2.3 : Peut-on passer directement de ACTIVE à RETIRED ?

**Non, c'est structurellement interdit.** L'invariant INV-EB-4 établit :

> Aucun élément ACTIVE ne peut passer directement à RETIRED ou ARCHIVED. La transition par DEPRECATED est **obligatoire**.

La matrice des transitions valides :

| Depuis \ Vers | DRAFT | ACTIVE | DEPRECATED | RETIRED | ARCHIVED |
|---------------|-------|--------|------------|---------|----------|
| DRAFT         | —     | ✓      | ✗          | ✗       | ✓        |
| ACTIVE        | ✗     | —      | ✓          | ✗       | ✗        |
| DEPRECATED    | ✗     | ✓*     | —          | ✓       | ✗        |
| RETIRED       | ✗     | ✗      | ✗          | —       | ✓        |
| ARCHIVED      | ✗     | ✗      | ✗          | ✗       | —        |

*La réactivation DEPRECATED → ACTIVE est exceptionnelle (successeur annulé).

**Référence contrat :** [Transition Rules Contract](../contracts/lifecycle/Ever%20Buddy%20-%20Transition%20Rules%20Contract.md)

---

### Q2.4 : Peut-on réactiver un élément déprécié ?

**Oui, mais c'est exceptionnel.** La transition DEPRECATED → ACTIVE est possible uniquement si :

1. Le successeur prévu est annulé
2. L'élément déprécié est toujours fonctionnel
3. La décision de réactivation est documentée avec justification

L'historique conserve la trace de la dépréciation temporaire.

**Référence :** [Documentation Fondatrice, Section 10, Scénario 4](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#scénario-4--réactivation-dun-élément-déprécié)

---

### Q2.5 : Un élément ARCHIVED peut-il être réactivé ?

**Non, jamais.** L'état ARCHIVED est **terminal et définitif**. Aucune transition n'est possible depuis ARCHIVED.

Si un élément archivé doit revivre, il faut créer un **nouvel élément** inspiré de l'archivé, pas le réactiver.

**Référence contrat :** [Lifecycle States Contract, Section 3.5](../contracts/lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md#35-archived-archivé)

---

## 3. Questions sur la compatibilité et les versions

### Q3.1 : Qu'est-ce que la rétrocompatibilité par défaut ?

L'invariant INV-EB-5 établit :

> Toute évolution est **présumée rétrocompatible** sauf déclaration explicite contraire.

Cela signifie que :
- Si vous ne déclarez rien, votre évolution est considérée rétrocompatible
- Une évolution incompatible **doit** être explicitement déclarée
- Les breaking changes nécessitent une justification et un plan de transition

**Référence :** [Documentation Fondatrice, INV-EB-5](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#inv-eb-5--rétrocompatibilité-par-défaut)

---

### Q3.2 : Comment fonctionne le versionnement sémantique ?

Ever Buddy utilise un **versionnement sémantique** (majeur.mineur.correctif) :

| Type | Signification | Exemple |
|------|---------------|---------|
| **Majeur** | Changement incompatible, rupture de contrat | 1.0 → 2.0 |
| **Mineur** | Ajout de fonctionnalité, rétrocompatible | 1.0 → 1.1 |
| **Correctif** | Correction de bug, aucun changement fonctionnel | 1.0.0 → 1.0.1 |

**Important :** Le versionnement sémantique ne dépend pas d'horloges synchronisées (conformité LOI-4).

**Référence contrat :** [Version Semantics Contract](../contracts/compatibility/Ever%20Buddy%20-%20Version%20Semantics%20Contract.md)

---

### Q3.3 : Qu'est-ce qu'une fenêtre de compatibilité ?

La **fenêtre de compatibilité** est la plage de versions avec lesquelles un élément garantit la compatibilité.

**Exemple :** "Compatible avec v2.0 à v2.4" signifie que l'élément fonctionne avec les versions 2.0, 2.1, 2.2, 2.3 et 2.4.

Les fenêtres de compatibilité sont définies par Ever Buddy et appliquées par Border Guard aux frontières du système.

**Référence Glossaire :** [Compatibility Window](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

### Q3.4 : Qu'est-ce qu'un breaking change ?

Un **breaking change** est un changement qui rompt la compatibilité avec les versions précédentes.

**Conséquences d'un breaking change :**
- Transition de version majeure obligatoire
- Période de dépréciation de l'ancienne version
- Documentation explicite des différences
- Chemin de migration fourni

**Référence :** [Documentation Fondatrice, Section 9 - Vocabulaire](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#9-vocabulaire-canonique)

---

## 4. Questions sur les responsabilités et limites

### Q4.1 : Ever Buddy exécute-t-il des migrations ?

**Non, jamais.** L'invariant INV-EB-1 est absolu :

> Ever Buddy ne possède **jamais** la capacité d'exécuter une migration, une transformation, ou une modification de données.

Ever Buddy **gouverne** les migrations :
- Il définit les règles de migration
- Il communique les chemins de migration
- Il observe l'avancement des migrations

Mais l'**exécution** est la responsabilité de :
- **KindMother** pour les données
- **Les produits** pour leur code

**Référence :** [Documentation Fondatrice, INV-EB-1](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#inv-eb-1--aucune-exécution-de-migration)

---

### Q4.2 : Ever Buddy peut-il forcer une évolution ?

**Non, jamais.** Ever Buddy influence par la guidance, pas par la contrainte.

Il peut :
- ✅ Recommander
- ✅ Alerter
- ✅ Planifier

Il ne peut pas :
- ❌ Imposer
- ❌ Forcer
- ❌ Contraindre

Les produits et les autres cores conservent leur autonomie.

**Référence :** [Documentation Fondatrice, Section 6](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#6-ce-que-ever-buddy-ne-fait-pas)

---

### Q4.3 : Ever Buddy décide-t-il des permissions ?

**Non.** Ever Buddy **ne décide jamais** si une action est permise. Cette décision appartient à **StrongFather**.

Ever Buddy fournit le **contexte** nécessaire à la décision :
- L'élément est-il DEPRECATED ?
- Quelle est la fenêtre de compatibilité ?
- Y a-t-il un successeur ?

Mais la **décision finale** est prise par StrongFather.

**Référence Glossaire :** [StrongFather](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#strongfather)

**Référence contrat :** [Core Interaction Contract, Section 2.2](../architecture/Ever%20Buddy%20-%20Core%20Interaction%20Contract.md#22-relation-avec-strongfather)

---

### Q4.4 : Ever Buddy modifie-t-il les données ?

**Non, jamais.** Ever Buddy ne modifie jamais les données gérées par KindMother.

Il peut :
- ✅ Observer
- ✅ Enregistrer (son propre historique)
- ✅ Recommander

Il ne peut pas :
- ❌ Modifier
- ❌ Supprimer
- ❌ Transformer

Toute modification est sous l'autorité exclusive de **KindMother**.

**Référence :** [Documentation Fondatrice, Section 6](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#ne-modifie-jamais-les-données)

---

## 5. Questions sur les interactions avec les autres cores

### Q5.1 : Comment Ever Buddy interagit-il avec KindMother ?

**Relation : Complémentaire**

| Aspect | KindMother | Ever Buddy |
|--------|------------|------------|
| Données à instant T | ✅ Autorité | ❌ Lecture seule |
| Schémas de données | ✅ Définition | ✅ Règles d'évolution |
| Migrations de données | ✅ Exécution | ✅ Définition des règles |

KindMother notifie Ever Buddy de tout nouveau schéma. Ever Buddy définit les règles d'évolution. KindMother peut refuser une migration si elle viole ses propres invariants.

**Référence contrat :** [Core Interaction Contract, Section 2.1](../architecture/Ever%20Buddy%20-%20Core%20Interaction%20Contract.md#21-relation-avec-kindmother)

---

### Q5.2 : Comment StrongFather utilise-t-il Ever Buddy ?

**Relation : Consultative**

StrongFather **consulte** Ever Buddy pour obtenir le contexte de cycle de vie :

| Information demandée | Usage par StrongFather |
|---------------------|------------------------|
| `current_state` | Évaluer si l'action est permise |
| `deprecation_date` | Évaluer l'urgence de migration |
| `successor_id` | Rediriger vers le successeur |
| `compatibility_level` | Évaluer les risques |

StrongFather peut ignorer les recommandations d'Ever Buddy (mais c'est tracé).

**Référence contrat :** [Core Interaction Contract, Section 2.2](../architecture/Ever%20Buddy%20-%20Core%20Interaction%20Contract.md#22-relation-avec-strongfather)

---

### Q5.3 : Les produits parlent-ils directement à Ever Buddy ?

**Non, jamais.** Les produits interagissent avec Ever Buddy **exclusivement via BondingBrother**.

```
Produits → BondingBrother → Ever Buddy
             (traduction)

❌ Produits → Ever Buddy (INTERDIT)
```

BondingBrother traduit les demandes et filtre les réponses.

**Référence contrat :** [Core Interaction Contract, Section 4](../architecture/Ever%20Buddy%20-%20Core%20Interaction%20Contract.md#4-relation-avec-les-produits)

---

### Q5.4 : Quand Ever Buddy escalade-t-il vers TAMR ?

Ever Buddy signale à TAMR (intervention humaine) les transitions critiques :

| Cas | Sévérité |
|-----|----------|
| Migration majeure (version majeure) | Élevée |
| Rupture de compatibilité (breaking change) | Élevée |
| Accélération de dépréciation | Moyenne |
| Archivage d'éléments FONDATION | Critique |
| Réactivation DEPRECATED → ACTIVE | Moyenne |

TAMR peut bloquer une transition en attente de validation humaine.

**Référence contrat :** [Core Interaction Contract, Section 2.7](../architecture/Ever%20Buddy%20-%20Core%20Interaction%20Contract.md#27-relation-avec-tamr)

---

## 6. Questions sur la dette structurelle

### Q6.1 : Qu'est-ce que la dette structurelle ?

La **dette structurelle** est l'ensemble des éléments DEPRECATED ou RETIRED qui persistent dans le système.

Cette dette n'est **pas nécessairement négative** — elle est le prix de la continuité. Cependant, Ever Buddy la surveille et alerte quand elle devient excessive.

**Référence :** [Documentation Fondatrice, Section 4](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#dette-structurelle)

---

### Q6.2 : Comment la dette est-elle mesurée ?

Ever Buddy utilise le **debt ratio** :

```
debt_ratio = (DEPRECATED + RETIRED) / ACTIVE
```

| Debt Ratio | Signification |
|------------|---------------|
| < 0.1 | Sain |
| 0.1 - 0.3 | Acceptable |
| 0.3 - 0.5 | Attention requise |
| > 0.5 | Critique, action requise |

**Référence contrat :** [Debt Tracking Contract](../contracts/observability/Ever%20Buddy%20-%20Debt%20Tracking%20Contract.md)

---

### Q6.3 : Que faire quand la dette est excessive ?

Ever Buddy recommande un **plan de nettoyage** :

1. Identifier les éléments RETIRED les plus anciens
2. Vérifier qu'aucun consommateur ne les utilise encore
3. Les faire transitionner vers ARCHIVED
4. Répéter jusqu'à ce que le debt ratio revienne sous le seuil

**Référence :** [Documentation Fondatrice, Section 10, Scénario 5](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#scénario-5--dette-structurelle-excessive)

---

## 7. Questions sur la traçabilité

### Q7.1 : L'historique des évolutions peut-il être modifié ?

**Non, jamais.** L'invariant INV-EB-2 garantit :

> Toute transition d'état de cycle de vie est **obligatoirement** enregistrée et cet enregistrement est **immuable**. L'historique ne peut être ni modifié, ni effacé, ni falsifié.

L'immuabilité de l'historique garantit l'auditabilité et la compréhension des évolutions passées.

**Référence :** [Documentation Fondatrice, INV-EB-2](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#inv-eb-2--traçabilité-complète-et-immuable)

---

### Q7.2 : Quelle documentation est requise pour une transition ?

L'invariant INV-EB-7 exige que toute transition soit **documentée** avec :

| Information | Obligatoire |
|-------------|-------------|
| Raison de la transition | ✅ |
| Impact sur les consommateurs | ✅ |
| Chemin de migration (si applicable) | ✅ |
| Date effective | ✅ |

**Une transition sans documentation est invalide.**

**Référence :** [Documentation Fondatrice, INV-EB-7](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#inv-eb-7--documentation-obligatoire)

---

### Q7.3 : Qu'est-ce qu'un tombstone ?

Un **tombstone** est l'enregistrement minimal conservé pour un élément archivé.

**Ce qui est conservé :**
- ✅ Métadonnées (ID, nom, version, dates)
- ✅ Historique des transitions
- ✅ Documentation finale (snapshot)
- ✅ Raison de l'archivage
- ✅ Référence au successeur (si applicable)

**Ce qui n'est pas conservé :**
- ❌ Données fonctionnelles

**Référence :** [Documentation Fondatrice, Section 9](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#tombstone)

---

## 8. Questions sur les Tools et Toolkits

### Q8.1 : Ever Buddy gère-t-il le cycle de vie des Tools ?

**Oui.** Ever Buddy est responsable du **cycle de vie** des Tools et Toolkits (Strate 6) :

| Responsabilité | Description |
|----------------|-------------|
| **Versions** | Gère les versions de chaque Tool |
| **Dépréciation** | Marque les Tools comme DEPRECATED |
| **Compatibilité** | Vérifie Tool ↔ Environnement |
| **Migration** | Gère la transition vers nouvelle version |

**Question à laquelle Ever Buddy répond :**

> *"Est-ce que cet outil existe encore, est compatible, ou doit être migré ?"*

**Référence :** [Documentation Fondatrice, Section 3](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#responsabilité-spécifique--cycle-de-vie-des-tools-et-toolkits)

---

### Q8.2 : Quelles sont les règles spécifiques aux Tools ?

| Règle | Description |
|-------|-------------|
| **RÈGLE-TOOL-EV-1** | Tout Tool a un état de vie explicite |
| **RÈGLE-TOOL-EV-2** | Un Tool DEPRECATED a un successeur identifié |
| **RÈGLE-TOOL-EV-3** | La transition vers RETIRED passe obligatoirement par DEPRECATED |
| **RÈGLE-TOOL-EV-4** | La compatibilité Tool ↔ Environnement est vérifiée |

**Référence :** [Tools et Toolkits](../../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)

---

## 9. Questions sur la conformité

### Q9.1 : Ever Buddy respecte-t-il les Lois d'Autonomie ?

**Oui, pleinement.** Ever Buddy est conforme à toutes les lois :

| Loi | Conformité | Mécanisme |
|-----|------------|-----------|
| **LOI-1** | ✅ | Registre d'états local, règles statiques |
| **LOI-2** | ✅ | Transitions validées localement |
| **LOI-3** | ✅ | Historique immuable local (INV-EB-2) |
| **LOI-4** | ✅ | États discrets et versionnement sémantique |
| **LOI-5** | ✅ | Observation pure, pas d'exécution |
| **LOI-6** | ✅ | Fédération via BondingBrother optionnelle |

**Question de validation :** *"Est-ce que Ever Buddy fonctionne encore si le système est seul, lent, et isolé ?"* — **Oui.**

**Référence :** [Documentation Fondatrice, Section 12](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#12-conformité-aux-lois-dautonomie-système)

**Référence Glossaire :** [LOI-1 à LOI-8](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md#loi-1-à-loi-8-lois-dautonomie)

---

### Q9.2 : Ever Buddy fonctionne-t-il en mode isolé ?

**Oui.** En mode isolé, Ever Buddy continue de :
- Gouverner les cycles de vie locaux
- Valider les transitions localement
- Maintenir l'historique local

La synchronisation des états entre nœuds (via BondingBrother) est optionnelle et non bloquante.

**Référence :** [Documentation Fondatrice, LOI-2](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#loi-2--le-système-accepte-lisolement-comme-état-normal)

---

## 10. Questions pratiques

### Q10.1 : Comment déprécier un élément ?

**Étapes obligatoires :**

1. **Identifier le successeur** (ou déclarer "aucun successeur")
2. **Définir la période de dépréciation** (minimum selon la catégorie)
3. **Documenter la transition** avec :
   - Raison de dépréciation
   - Guide de migration
   - Date de retirement prévue
4. **Communiquer** via BondingBrother aux consommateurs
5. **Transition** : ACTIVE → DEPRECATED

**Attention :** La communication préalable est obligatoire (minimum 1 cycle de release).

**Référence :** [Documentation Fondatrice, Section 4](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#périodes-minimales-de-transition)

---

### Q10.2 : Combien de temps dure une période de dépréciation ?

La durée dépend de la **catégorie de l'élément** :

| Catégorie | Période minimale |
|-----------|------------------|
| **Contrats fondateurs (FONDATION)** | Très longue (plusieurs générations) |
| **Contrats opérationnels** | Standard |
| **Interfaces techniques** | Courte |
| **Éléments internes** | Optionnelle |

Ces périodes sont des **minimums**. Ever Buddy peut recommander des périodes plus longues selon l'impact et l'adoption.

**Référence contrat :** [Lifecycle States Contract, Section 7](../contracts/lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md#7-catégories-déléments-et-règles-détat)

---

### Q10.3 : Comment savoir si mon élément est compatible avec une version ?

**Demander à Ever Buddy** (via BondingBrother) :
- La **fenêtre de compatibilité** de votre élément
- Le **niveau de compatibilité** avec la version cible

**Niveaux de compatibilité :**

| Niveau | Signification |
|--------|---------------|
| **Rétrocompatible** | Le nouveau fonctionne avec l'ancien |
| **Compatible en amont** | L'ancien fonctionne avec le nouveau (rare) |
| **Incompatible** | Migration obligatoire |

**Référence :** [Documentation Fondatrice, Section 4 - Compatibilité](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#compatibilité)

---

### Q10.4 : Que se passe-t-il si je n'ai pas migré à temps ?

Si vous n'avez pas migré avant la date de retirement :

1. **Période de grâce** : Temps supplémentaire accordé au cas par cas
2. **Restrictions** : L'élément RETIRED n'est plus proposé aux nouveaux consommateurs
3. **Support minimal** : Uniquement corrections critiques de sécurité
4. **Archivage** : Après la période de grâce, l'élément devient ARCHIVED et non fonctionnel

**Recommandation :** Migrer pendant la période DEPRECATED, pas après.

**Référence :** [Documentation Fondatrice, Section 9 - Grace period](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#grace-period)

---

## 11. Questions sur les invariants

### Q11.1 : Quels sont les invariants d'Ever Buddy ?

| Invariant | Énoncé |
|-----------|--------|
| **INV-EB-1** | Aucune exécution de migration |
| **INV-EB-2** | Traçabilité complète et immuable |
| **INV-EB-3** | Aucun état ambigu |
| **INV-EB-4** | Période de dépréciation obligatoire |
| **INV-EB-5** | Rétrocompatibilité par défaut |
| **INV-EB-6** | Vision long terme obligatoire |
| **INV-EB-7** | Documentation obligatoire |
| **INV-EB-8** | Indépendance des décisions |
| **INV-EB-9** | Prédictibilité des transitions |
| **INV-EB-10** | Unicité du successeur déclaré |
| **INV-EB-11** | Non-rétroactivité des changements de règles |
| **INV-EB-12** | Responsabilité de l'annonce |

**Référence contrat :** [Invariants & Guarantees](../contracts/governance/Ever%20Buddy%20-%20Invariants%20%26%20Guarantees.md)

---

### Q11.2 : Les invariants peuvent-ils être modifiés ?

**Non pour les invariants fondamentaux.** Les invariants de la Documentation Fondatrice sont de statut **FONDATION** — non négociables.

L'invariant INV-EB-11 établit que :

> Les règles d'évolution s'appliquent aux transitions **futures**. Un changement de règle ne peut pas modifier le statut d'éléments déjà en transition.

**Référence :** [Documentation Fondatrice, INV-EB-11](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md#inv-eb-11--non-rétroactivité-des-changements-de-règles)

---

## 12. Références

### Documents fondateurs

- [Ever Buddy - Documentation Fondatrice](../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)

### Contrats associés

- [Lifecycle States Contract](../contracts/lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md)
- [Transition Rules Contract](../contracts/lifecycle/Ever%20Buddy%20-%20Transition%20Rules%20Contract.md)
- [Compatibility Rules Contract](../contracts/compatibility/Ever%20Buddy%20-%20Compatibility%20Rules%20Contract.md)
- [Version Semantics Contract](../contracts/compatibility/Ever%20Buddy%20-%20Version%20Semantics%20Contract.md)
- [Invariants & Guarantees](../contracts/governance/Ever%20Buddy%20-%20Invariants%20%26%20Guarantees.md)
- [Debt Tracking Contract](../contracts/observability/Ever%20Buddy%20-%20Debt%20Tracking%20Contract.md)
- [Core Interaction Contract](../architecture/Ever%20Buddy%20-%20Core%20Interaction%20Contract.md)

### Références externes

- [Glossaire Miyukini](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- [Lois d'Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)
- [Tools et Toolkits](../../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Document de référence — Informatif  
**Dérivé de :** Ever Buddy - Documentation Fondatrice v1.3  
**Type :** FAQ et questions fréquentes
