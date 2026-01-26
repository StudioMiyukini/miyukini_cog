# Miyukini Framework - Guide Adaptateurs Produits

> Guide complet pour créer des adaptateurs produits compatibles avec KindMother v2.4.  
> Documentation fondatrice contractuelle.

---

## Contexte

Les modules SPM CMS exposent des **contrats fonctionnels** sous forme de traits Rust (ContentManager, HierarchyManager, TaxonomyManager, MediaManager, PublicationManager, SearchManager). Ces traits définissent les opérations fonctionnelles sans imposer de détails d'implémentation.

**Le produit doit implémenter ces traits** via des **adaptateurs produits** qui traduisent les contrats fonctionnels vers la **CoreDataAPI de KindMother**.

**Règle fondamentale :** Aucun adaptateur produit ne doit jamais persister directement. Toute persistance passe exclusivement par KindMother via la CoreDataAPI.

### Pattern architectural

Ce guide définit et documente le pattern architectural **Authoritative Core with Intent-Based Adapters** :

- **Authoritative Core :** KindMother est l'unique autorité sur l'état, la validité et la temporalité des données
- **Intent-Based Adapters :** Les adaptateurs produits expriment des intentions (WriteIntent) sans calculer ou pré-valider l'état final

Ce pattern garantit la cohérence systémique, évite la duplication de règles, et centralise l'autorité sur les données.

---

## 1. Rôle et responsabilités des adaptateurs

### Qu'est-ce qu'un adaptateur produit ?

Un adaptateur produit est une **implémentation concrète d'un trait SPM CMS** qui :

1. **Reçoit les demandes** des modules SPM (opérations fonctionnelles)
2. **Traduit** ces demandes en opérations CoreDataAPI de KindMother
3. **Fournit le contexte complet** à KindMother (utilisateur, autorisations, instance)
4. **Traduit les résultats** KindMother en types SPM
5. **Retourne les résultats** aux modules SPM

**L'adaptateur ne persiste JAMAIS directement.** Il délègue toute persistance à KindMother.

### Responsabilités principales

**Traduction bidirectionnelle :**
- Types SPM → Structures de données pour CoreDataAPI
- Opérations SPM → Opérations CoreDataAPI (read, submitWriteIntent, etc.)
- Résultats CoreDataAPI → Types SPM
- Erreurs KindMother → Erreurs SPM

**Fourniture du contexte :**
- Contexte utilisateur (identité de l'utilisateur)
- Contexte d'autorisation (règles de permissions conceptuelles)
- Contexte d'instance (DB Mère ou DB Fille à utiliser)

**Isolation des modules SPM :**
- Les modules SPM ne doivent jamais connaître l'existence de KindMother
- Aucune fuite de types, structures, ou erreurs KindMother vers les modules SPM
- Aucune référence à KindMother dans les types exposés par les modules SPM

**Gestion des erreurs :**
- Traduction des erreurs KindMother en erreurs SPM appropriées
- Validation des données selon les règles du produit (avant traduction)
- Logging des erreurs et warnings pour le débogage
- Retour d'erreurs claires et actionnables

### Ce que l'adaptateur ne fait PAS

**Interdictions absolues :**
- ❌ Accéder directement à une base de données (SQLite, PostgreSQL, MongoDB, etc.)
- ❌ Exécuter des requêtes SQL ou toute autre requête de persistance
- ❌ Utiliser des repositories, ORM, ou bibliothèques de persistance
- ❌ Gérer des transactions de persistance
- ❌ Gérer la synchronisation entre instances
- ❌ Gérer les permissions conceptuelles (c'est le rôle de KindMother)
- ❌ Générer des IDs manuellement (utiliser le kernel via KindMother)
- ❌ Utiliser `SystemTime::now()` directement (utiliser le kernel via KindMother)

**Règle :** L'adaptateur est un **traducteur pur** entre SPM et KindMother. Toute logique de persistance, synchronisation, ou gestion d'identité est du ressort de KindMother.

---

## 2. Architecture : position dans le système

### Flux de dépendances

```
┌─────────────────────────────────────────┐
│           PRODUIT                        │
│  ┌───────────────────────────────────┐  │
│  │  Adaptateurs Produits              │  │
│  │  (implémentent les traits SPM)     │  │
│  │  (appellent KindMother)           │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
           │
           │ appelle (uniquement)
           ▼
┌─────────────────────────────────────────┐
│         KINDMOTHER                        │
│  (moteur de données interne)             │
│  - CoreDataAPI                           │
│  - Persistance (SQLite interne)        │
│  - Synchronisation                       │
└─────────────────────────────────────────┘
           │
           │ utilise
           ▼
┌─────────────────────────────────────────┐
│         MODULES SPM CMS                  │
│  (traits fonctionnels, pas de DB)       │
└─────────────────────────────────────────┘
           │
           │ utilise
           ▼
┌─────────────────────────────────────────┐
│           KERNEL                         │
│  (Id, Clock, Logger)                     │
└─────────────────────────────────────────┘
```

### Règles architecturales strictes

**Dépendances unidirectionnelles :**
- Produit → Adaptateurs → KindMother → Kernel
- Modules SPM → Kernel (indépendamment de KindMother)
- Aucune dépendance inverse autorisée

**Séparation des responsabilités :**
- **SPM :** Contrats fonctionnels purs, logique générique, aucune notion de persistance
- **Adaptateurs :** Traduction SPM ↔ CoreDataAPI, fourniture du contexte
- **KindMother :** Persistance, synchronisation, permissions conceptuelles, cohérence
- **Produit :** Logique métier spécifique, API, UI, règles de permissions

**Interdictions :**
- ❌ Un produit ne doit jamais appeler directement KindMother (sauf via adaptateurs)
- ❌ Un module SPM ne doit jamais avoir de référence vers KindMother
- ❌ Un adaptateur ne doit jamais contourner KindMother pour accéder directement à la persistance
- ❌ Le kernel ne doit jamais appeler KindMother
- ❌ Un adaptateur ne doit jamais exposer KindMother aux modules SPM

### Schéma de flux complet

```
┌──────────┐
│ PRODUIT  │
└────┬─────┘
     │ 1. Opération fonctionnelle
     │    (ex: create_content)
     ▼
┌─────────────────────┐
│ MODULE SPM          │
│ (trait fonctionnel) │
│ - ContentManager    │
│ - HierarchyManager  │
│ - etc.              │
└────┬────────────────┘
     │ 2. Appel du trait
     │    implémenté
     ▼
┌─────────────────────┐
│ ADAPTATEUR PRODUIT  │
│ (traduction)        │
│ - Reçoit opération  │
│ - Traduit en        │
│   CoreDataAPI       │
│ - Fournit contexte  │
└────┬────────────────┘
     │ 3. Appel CoreDataAPI
     │    avec contexte
     ▼
┌─────────────────────┐
│ KINDMOTHER          │
│ (moteur de données) │
│ - Valide permissions│
│ - Persiste          │
│ - Synchronise       │
└────┬────────────────┘
     │ 4. Résultat
     ▼
┌─────────────────────┐
│ ADAPTATEUR PRODUIT  │
│ (traduction)        │
│ - Traduit résultat  │
│ - Traduit erreurs   │
└────┬────────────────┘
     │ 5. Résultat SPM
     ▼
┌─────────────────────┐
│ MODULE SPM          │
│ (retour au produit) │
└────┬────────────────┘
     │ 6. Résultat final
     ▼
┌──────────┐
│ PRODUIT  │
└──────────┘
```

---

## 3. Principes de conception

### Principe 1 : Traduction pure

L'adaptateur ne doit contenir **aucune logique métier** et **aucune logique de persistance**. Il traduit uniquement :

- Types SPM ↔ Structures pour CoreDataAPI
- Opérations SPM ↔ Opérations CoreDataAPI
- Erreurs KindMother ↔ Erreurs SPM

**Règle :** Si l'adaptateur contient de la logique métier ou de la persistance, c'est une violation architecturale.

### Principe 2 : Délégation totale à KindMother

L'adaptateur **délègue toute persistance à KindMother** via la CoreDataAPI :

- **Lecture :** `read`, `list`, `query`
- **Écriture :** `submitWriteIntent`, `submitBatchWriteIntent`
- **Synchronisation :** `sync`, `requestSync` (si nécessaire)
- **Inspection :** `getStatus`, `getSyncState`, `getPendingWriteIntents` (si nécessaire)

**Règle :** Aucune opération de persistance ne doit contourner KindMother.

**Principe d'autorité :** KindMother est l'unique autorité sur l'état, la validité et la temporalité des données. L'adaptateur ne fait qu'exprimer des intentions via WriteIntent. L'adaptateur ne doit jamais tenter de pré-calculer, pré-valider, ou reproduire les règles de cohérence de KindMother.

### Principe 3 : Fourniture du contexte complet

L'adaptateur **doit fournir un contexte complet** à KindMother pour chaque opération :

- **Contexte utilisateur :** Identité de l'utilisateur effectuant l'opération
- **Contexte d'autorisation :** Règles de permissions conceptuelles définies par le produit
- **Contexte d'instance :** Instance à utiliser (DB Mère ou DB Fille, déterminée par la configuration du produit)

**Règle :** Un contexte incomplet ou incohérent entraîne le rejet de l'opération par KindMother.

### Principe 4 : Isolation des modules SPM

L'adaptateur **doit garantir l'isolation complète** des modules SPM vis-à-vis de KindMother :

- Les modules SPM ne doivent jamais connaître l'existence de KindMother
- Aucun type, structure, ou erreur KindMother ne doit être exposé aux modules SPM
- Aucune référence à KindMother dans les types ou structures exposés par les modules SPM

**Règle :** L'isolation DOIT être totale. Aucune exception n'est autorisée.

### Principe 5 : Gestion des erreurs

L'adaptateur doit :

- **Traduire** toutes les erreurs KindMother en erreurs SPM appropriées
- **Valider** les données selon les règles du produit (avant traduction vers CoreDataAPI)
- **Logger** les erreurs et warnings pour le débogage
- **Retourner** des erreurs claires et actionnables

**Règle :** Aucune erreur KindMother ne doit être exposée directement au module SPM ou au produit.

### Principe 6 : Thread-safety

L'adaptateur doit être **thread-safe** si utilisé dans un contexte concurrent :

- Utilisation de `Arc` pour le partage
- Utilisation de `Mutex` ou `RwLock` si nécessaire pour le contexte ou les dépendances
- Gestion des appels concurrents à KindMother (KindMother garantit l'isolation transactionnelle)

---

## 4. Structure conceptuelle d'un adaptateur

### Structure de base

Un adaptateur produit contient :

**1. Dépendances KindMother :**
- Référence à KindMother (ou à la CoreDataAPI)
- Configuration de l'instance (DB Mère ou DB Fille)

**2. Contexte :**
- Contexte utilisateur (fourni par le produit)
- Contexte d'autorisation (règles de permissions définies par le produit)
- Contexte d'instance (déterminé par la configuration du produit)

**3. Méthodes de traduction :**
- `translate_spm_to_km()` : Types SPM → Structures pour CoreDataAPI
- `translate_km_to_spm()` : Résultats CoreDataAPI → Types SPM
- `translate_error()` : Erreurs KindMother → Erreurs SPM

**4. Implémentation du trait SPM :**
- Chaque méthode du trait SPM :
  1. Reçoit les paramètres SPM
  2. Traduit en opération CoreDataAPI
  3. Fournit le contexte complet
  4. Appelle KindMother
  5. Traduit le résultat
  6. Retourne le résultat SPM

### Exemple conceptuel de flux

**Opération de création :**

```
1. Module SPM appelle : create_content(input: ContentInput)
2. Adaptateur traduit :
   - ContentInput → Structure pour WriteIntent
   - Détermine le type d'opération : création
3. Adaptateur fournit le contexte :
   - Utilisateur : identité de l'utilisateur
   - Autorisation : règles de permissions
   - Instance : DB Mère ou DB Fille
4. Adaptateur appelle KindMother :
   - submitWriteIntent(write_intent, contexte)
5. KindMother :
   - Valide les permissions
   - Valide la cohérence
   - Applique le WriteIntent
   - Retourne le résultat
6. Adaptateur traduit :
   - Résultat KindMother → Id (type SPM)
   - Erreur KindMother → ContentError (type SPM)
7. Adaptateur retourne : Result<Id, ContentError>
```

**Opération de lecture :**

```
1. Module SPM appelle : get_content(id: Id)
2. Adaptateur traduit :
   - Id → Identifiant pour CoreDataAPI
3. Adaptateur fournit le contexte complet
4. Adaptateur appelle KindMother :
   - read(identifiant, contexte)
5. KindMother :
   - Vérifie les permissions
   - Lit depuis la persistance
   - Retourne les données
6. Adaptateur traduit :
   - Données KindMother → Content (type SPM)
   - Erreur KindMother → ContentError (type SPM)
7. Adaptateur retourne : Result<Content, ContentError>
```

---

## 5. Gestion des erreurs et logging

### Traduction des erreurs

L'adaptateur doit traduire toutes les erreurs KindMother en erreurs SPM appropriées :

**Types d'erreurs KindMother :**
- Erreur de permission → Erreur SPM de permission
- Erreur de cohérence → Erreur SPM de validation
- Erreur de contexte → Erreur SPM de contexte invalide
- Erreur fonctionnelle → Erreur SPM fonctionnelle
- Erreur de synchronisation → Erreur SPM de synchronisation (si applicable)

**Règle :** Aucune erreur KindMother ne doit être exposée directement. Toutes les erreurs doivent être traduites selon le contrat SPM.

### Validation des données

L'adaptateur peut valider les données selon les règles du produit **avant** de les traduire vers CoreDataAPI :

- Validation des formats
- Validation des contraintes métier spécifiques au produit
- Validation des références (si nécessaire avant traduction)

**Règle fondamentale :** Cette validation ne doit **jamais tenter de reproduire les règles de cohérence de KindMother**. L'adaptateur valide uniquement des aspects spécifiques au produit (formats, règles métier locales) qui sont complémentaires à la validation de KindMother.

**Pourquoi cette règle est essentielle :**
- Toute duplication de règles de cohérence entre l'adaptateur et KindMother crée un risque de divergence
- Si les règles divergent, le système devient incohérent
- KindMother reste l'unique autorité sur la cohérence des données

**Note :** KindMother valide les permissions, la cohérence, et l'intégrité référentielle. La validation dans l'adaptateur est strictement complémentaire et ne doit jamais dupliquer ces validations.

### Logging

L'adaptateur peut logger les opérations pour le débogage :

- Logging des traductions (optionnel, pour debug)
- Logging des erreurs (recommandé)
- Logging des appels à KindMother (optionnel, pour audit)

**Règle :** Le logging ne doit jamais exposer de détails d'implémentation KindMother aux modules SPM ou au produit.

---

## 6. Tests et validation

### Tests unitaires

Chaque méthode de l'adaptateur doit être testée :

- Traduction SPM → CoreDataAPI
- Traduction CoreDataAPI → SPM
- Traduction des erreurs
- Fourniture du contexte

**Note :** Les tests peuvent utiliser des mocks de KindMother pour isoler la logique de traduction.

### Tests d'intégration

Tester l'intégration complète avec KindMother :

- Création d'une instance KindMother de test
- Exécution des opérations via l'adaptateur
- Vérification des résultats

**Note :** Les tests d'intégration nécessitent une instance KindMother fonctionnelle (peut être une instance de test en mémoire).

### Validation du contrat

Utiliser les implémentations mémoire comme référence :

- Comparer le comportement de l'adaptateur avec l'implémentation mémoire
- Vérifier que les mêmes entrées produisent les mêmes résultats (après traduction)

---

## 7. Checklist de création d'un adaptateur

### Avant de commencer

- [ ] Lire le contrat du module SPM (`docs/spm-cms/modules/*/contrat.md`)
- [ ] Examiner l'implémentation mémoire (`crates/miyukini-spm-cms-*/src/memory.rs`)
- [ ] Lire la documentation KindMother (fondatrice, CoreDataAPI, interface)
- [ ] Comprendre la CoreDataAPI et ses opérations
- [ ] Définir les règles de permissions conceptuelles du produit

### Structure de base

- [ ] Créer la structure de l'adaptateur avec dépendances KindMother
- [ ] Implémenter le constructeur avec configuration du contexte
- [ ] Implémenter toutes les méthodes du trait SPM
- [ ] Ajouter les méthodes de traduction (SPM ↔ CoreDataAPI)

### Intégration KindMother

- [ ] Utiliser uniquement la CoreDataAPI (pas d'accès direct à la persistance)
- [ ] Fournir le contexte complet pour chaque opération
- [ ] Gérer les erreurs KindMother et les traduire en erreurs SPM
- [ ] Garantir l'isolation des modules SPM vis-à-vis de KindMother

### Gestion des erreurs

- [ ] Traduire toutes les erreurs KindMother en erreurs SPM
- [ ] Valider les données selon les règles du produit (avant traduction)
- [ ] Logger les erreurs et warnings
- [ ] Tester tous les cas d'erreur

### Tests

- [ ] Tests unitaires pour chaque méthode
- [ ] Tests d'intégration avec KindMother
- [ ] Validation du contrat (comparaison avec implémentation mémoire)

### Documentation

- [ ] Documenter la structure de l'adaptateur
- [ ] Documenter les choix de traduction
- [ ] Documenter les règles de permissions utilisées
- [ ] Documenter les cas d'usage spécifiques

---

## 8. Plan de migration conceptuel

### Ancien adaptateur (avant KindMother)

**Caractéristiques :**
- Accès direct à la base de données
- Gestion de la persistance
- Gestion des transactions
- Pas de synchronisation
- Pas de gestion centralisée des permissions

### Nouveau adaptateur (avec KindMother)

**Caractéristiques :**
- Aucun accès direct à la base de données
- Délégation totale à KindMother via CoreDataAPI
- Pas de gestion de transactions (gérée par KindMother)
- Synchronisation automatique via KindMother
- Permissions conceptuelles centralisées dans KindMother

### Étapes de migration

**1. Identifier les accès directs à la persistance :**
- Rechercher tous les appels à la base de données
- Identifier les requêtes SQL, ORM, repositories
- Lister les opérations de persistance directe

**2. Remplacer par des appels CoreDataAPI :**
- Mapper chaque opération de persistance vers une opération CoreDataAPI
- Créer les méthodes de traduction SPM ↔ CoreDataAPI
- Implémenter la fourniture du contexte

**3. Supprimer les dépendances de persistance :**
- Supprimer les dépendances vers les bibliothèques de persistance
- Supprimer les repositories, ORM, clients DB
- Supprimer toute logique de transaction

**4. Tester et valider :**
- Tester chaque opération avec KindMother
- Valider que les résultats sont identiques (après traduction)
- Vérifier l'isolation des modules SPM

**5. Documenter :**
- Documenter les choix de traduction
- Documenter les règles de permissions utilisées
- Documenter les changements architecturaux

---

## 9. Références

### Documentation SPM

- **Contrats modules :** `docs/spm-cms/modules/*/contrat.md`
- **Implémentations mémoire :** `crates/miyukini-spm-cms-*/src/memory.rs`
- **Squelette produit :** `docs/spm-cms/Miyukini Framework - Squelette Repo Produit Mini CMS.md`

### Documentation KindMother

- **Documentation fondatrice :** `docs/core/Miyukini Core System - KindMother Documentation Fondatrice.md`
- **CoreDataAPI :** `docs/core/KindMother — CoreDataAPI (Surface d'Appel Conceptuelle).md`
- **Interface & Contrat :** `docs/core/KindMother — Interface & Contrat d'Intégration.md`

### Documentation kernel

- **Kernel :** `crates/miyukini-kernel/src/`
- **Documentation kernel :** `docs/kernel/`

### Documentation adaptateurs

- **Documentation conceptuelle :** `docs/core/Miyukini Core System - Adaptateur Produit Documentation Conceptuelle.md`

---

## 10. Guides spécifiques par module

Pour des guides détaillés sur chaque adaptateur, voir :

- [Adaptateur ContentManager](adaptateurs/Miyukini%20Framework%20-%20Adaptateur%20ContentManager.md)
- [Adaptateur HierarchyManager](adaptateurs/Miyukini%20Framework%20-%20Adaptateur%20HierarchyManager.md)
- [Adaptateur TaxonomyManager](adaptateurs/Miyukini%20Framework%20-%20Adaptateur%20TaxonomyManager.md)
- [Adaptateur MediaManager](adaptateurs/Miyukini%20Framework%20-%20Adaptateur%20MediaManager.md)
- [Adaptateur PublicationManager](adaptateurs/Miyukini%20Framework%20-%20Adaptateur%20PublicationManager.md)
- [Adaptateur SearchManager](adaptateurs/Miyukini%20Framework%20-%20Adaptateur%20SearchManager.md)

**Note :** Ces guides doivent être mis à jour pour refléter l'utilisation de KindMother via la CoreDataAPI.

---

## 11. Exemples

Des exemples d'implémentations concrètes sont disponibles dans :

- `docs/spm-cms/adaptateurs/examples/`

Ces exemples couvrent différents scénarios :
- Adaptateur mémoire (référence) - voir les implémentations dans `crates/miyukini-spm-cms-*/src/memory.rs`
- Adaptateur avec KindMother (exemple conceptuel) - à venir

Voir le README dans le dossier examples pour plus de détails.

---

---

## Conclusion

Ce guide définit le pattern architectural **Authoritative Core with Intent-Based Adapters** pour l'intégration des adaptateurs produits avec KindMother v2.4.

**Points clés :**
- KindMother est l'unique autorité sur l'état, la validité et la temporalité des données
- Les adaptateurs produits expriment des intentions (WriteIntent) sans calculer l'état final
- Aucune duplication de règles de cohérence entre adaptateurs et KindMother
- Isolation complète des modules SPM vis-à-vis de KindMother
- Traduction pure entre SPM et CoreDataAPI

Cette architecture garantit la cohérence systémique, évite la divergence des règles, et centralise l'autorité sur les données.

---

**Document créé le :** 2026-01-24  
**Version :** 2.4  
**Statut :** Documentation fondatrice compatible KindMother v2.4  
**Pattern architectural :** Authoritative Core with Intent-Based Adapters  
**Référence :** Aligné avec la documentation KindMother et la CoreDataAPI
