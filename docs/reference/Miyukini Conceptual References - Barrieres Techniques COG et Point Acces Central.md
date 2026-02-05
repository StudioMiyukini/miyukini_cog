# Miyukini Conceptual References — Barrières techniques COG et point d'accès Central

## Contexte

Ce document définit les **barrières techniques** qui garantissent que le système COG (Cores) fonctionne sans contournement : persistance via KindMother, décisions via StrongFather, médiation via BondingBrother. Il fixe en outre la **règle canonique** selon laquelle **Miyukini Central** est le **point d'accès utilisateur unique** pour tous les Services de l'écosystème.

Il est **normatif** et sert de base à l'implémentation immédiate (contrats, graphe de dépendances, vérifications build et runtime).

## Portée / Scope

- **Applicable à :** Architecture COG, implémentation des Cores, Opérateurs, Miyukini Central, vérifications techniques.
- **Audience :** Architectes, développeurs, QA, outils d'analyse (lint, CI).
- **Statut :** Document de référence normatif — source de vérité pour les barrières et le point d'accès utilisateur.
- **Hors portée :** Détail des contrats par Core (KindMother, StrongFather, BondingBrother) — voir docs/core respectives.

---

## Règle canonique — Point d'accès utilisateur

> **Tous les Services ont comme point d'accès utilisateur Miyukini Central.**

Cette règle est **non négociable** et **gravée dans le marbre** :

| Code | Énoncé |
|------|--------|
| **CANON-CENTRAL-1** | Aucun Service (Opérateur porteur d'un Service) ne doit exposer à l'utilisateur final un point d'entrée UI concurrent du Hub. |
| **CANON-CENTRAL-2** | L'utilisateur accède aux Services (catalogue, lancement, « Mes Services ») **uniquement** via Miyukini Central. |
| **CANON-CENTRAL-3** | Les Opérateurs qui portent les Services sont **invoqués** ou **ouverts** après passage par le Hub (Mandat StrongFather, puis ouverture de l'UI du Service). Ils ne sont pas lancés ni découverts par un autre canal utilisateur. |
| **CANON-CENTRAL-4** | Exception : **MiyukiniAdmin** (Strate 9, console souveraine d'administration) reste un point d'entrée distinct, réservé à l'administration système, pas à l'utilisateur final des Services. |

**Conséquences :**

- Les écrans « catalogue », « mes applications », « lancement de service » sont **du ressort exclusif de Miyukini Central**.
- Tout nouvel Opérateur ou Service est **découvert et lancé** via le Hub ; pas de launcher parallèle, pas de raccourci utilisateur direct vers l'Opérateur sans passer par le Hub.
- L’implémentation technique (graphe de dépendances, runtime) doit **vérifier** que le point d’accès utilisateur aux Services reste bien Central (voir section Vérification).

---

## 1. Barrières techniques

Les barrières suivantes rendent le flux COG **obligatoire** en pratique : aucun chemin technique légitime ne doit permettre de persister, décider ou médier en contournant les Cores.

### 1.1 Point d'entrée unique côté appelants (façade)

| Identifiant | Description |
|-------------|-------------|
| **BAR-FACADE-1** | Les Opérateurs et Miyukini Central n'ont **pas** de dépendance directe vers les crates `kindmother` ou `strongfather`. |
| **BAR-FACADE-2** | Le seul point d'entrée technique vers les Cores (données, décisions) est une **façade** exposée par le runtime COG — en pratique **BondingBrother** ou un client « COG » qui dépend de BondingBrother, KindMother et StrongFather. |
| **BAR-FACADE-3** | Les crates « opérateurs » et `miyukini-central` dépendent au plus du crate qui expose cette façade (traits, types d'intentions/résultats) ; ils ne voient pas les types CoreDataAPI ni les types StrongFather. |

**Implémentation immédiate :**

- Introduire un crate (ex. `cog-client` ou usage explicite de `bondingbrother`) comme seule dépendance autorisée pour « appeler les Cores ».
- Vérifier en CI / lint que les crates opérateurs et `miyukini-central` n’ont pas de dépendance vers `kindmother` ni `strongfather`.

---

### 1.2 KindMother — Aucun accès direct au Storage hors du Core

| Identifiant | Description |
|-------------|-------------|
| **BAR-KM-1** | Le trait `Storage` (read / write / delete) est utilisé **uniquement** à l'intérieur du crate `kindmother`. |
| **BAR-KM-2** | Aucun autre crate (bondingbrother, opérateurs, miyukini-central) ne dépend du trait `Storage` ni d'un type qui l'implémente. |
| **BAR-KM-3** | Toute persistance métier transite par la **CoreDataAPI** de KindMother (read, list, query, submitWriteIntent, submitBatchWriteIntent). Aucune écriture directe en base ou en fichier « métier » en dehors de KindMother. |

**Implémentation immédiate :**

- Ne pas exposer `Storage` en public aux crates « hauts » ; ou restreindre son usage au seul crate `kindmother`.
- Les adaptateurs et produits n’appellent que la surface CoreDataAPI (WriteIntent, contexte, pas de handle sur le stockage physique).

---

### 1.3 StrongFather — Seule source de verdict pour les décisions gouvernées

| Identifiant | Description |
|-------------|-------------|
| **BAR-SF-1** | Pour les flux qui exigent une décision stratégique (ex. écritures sensibles, franchissement de frontière, émission de Mandat), le code ne doit pas pouvoir « décider tout seul » ; il doit **obtenir un verdict** (APPROVED / DENIED / etc.) depuis une abstraction exposée par le runtime. |
| **BAR-SF-2** | La seule implémentation « légale » en production de cette abstraction est celle qui appelle StrongFather (politiques, Policy Source). |
| **BAR-SF-3** | Aucune implémentation « bypass » (ex. toujours APPROVED sans appel StrongFather) ne doit être utilisée en production pour les cas gouvernés. |

**Implémentation immédiate :**

- Définir un trait ou une interface (ex. `DecisionAuthority`, `PolicyGateway`) dont l’implémentation production délègue à StrongFather.
- Les chemins qui nécessitent une décision (lancement de Service, WriteIntent sensibles) passent par cette interface.

---

### 1.4 BondingBrother — Seul médiateur vers les Cores

| Identifiant | Description |
|-------------|-------------|
| **BAR-BB-1** | Toute intention métier (lecture, écriture, décision) est formulée en **intentions** (types du domaine BondingBrother / contrat BB). |
| **BAR-BB-2** | BondingBrother traduit ces intentions en appels CoreDataAPI (KindMother) et/ou en demandes de décision (StrongFather). Les Opérateurs et le Central n'appellent **jamais** KindMother ou StrongFather directement. |
| **BAR-BB-3** | Les signatures des méthodes exposées aux Opérateurs et au Central prennent des **intentions** et retournent des **résultats** ; les types `CoreDataAPI` ou StrongFather n'apparaissent pas dans l'API publique des appelants. |

**Implémentation immédiate :**

- API publique BondingBrother (ou façade COG) : entrée = intentions (ex. `IntentCatalogue`, `IntentLancerService`, `IntentWrite`), sortie = résultats typés (ex. `CatalogueResult`, `MandatResult`, `WriteResult`).
- Aucun type interne KindMother/StrongFather exposé dans les crates opérateurs ou miyukini-central.

---

## 2. Vérification en vue d'implémentation immédiate

Les vérifications suivantes permettent de s'assurer que les barrières sont respectées et que Miyukini Central reste le point d'accès utilisateur unique.

### 2.1 Vérification à la build / lint

| Identifiant | Règle | Moyen |
|-------------|--------|--------|
| **VERIF-BUILD-1** | Aucun crate « opérateur » (ex. crates `miyu*` porteurs de Services) ne dépend de `kindmother` ni de `strongfather`. | Graphe de dépendances (Cargo) : vérifier que `kindmother` et `strongfather` n'apparaissent pas dans les `Cargo.toml` des crates opérateurs ni de `miyukini-central`. |
| **VERIF-BUILD-2** | Seul un crate « runtime » ou « façade COG » (ex. `bondingbrother` ou `cog-runtime`) dépend de `kindmother` et `strongfather`. | Idem : script ou règle CI qui liste les dépendants de `kindmother` / `strongfather` et exige que ce soit au plus le runtime / BondingBrother. |
| **VERIF-BUILD-3** | Le crate `kindmother` n'exporte pas le trait `Storage` (ou ne l'exporte pas aux crates autres que lui-même). | Revue de l'API publique du crate `kindmother` (pub use, visibilité). |

**Implémentation immédiate :**

- Script (ex. PowerShell / Rust) ou job CI qui :
  - Parse les `Cargo.toml` du workspace ;
  - Liste les crates qui dépendent de `kindmother` ou `strongfather` ;
  - Échoue si un crate « opérateur » ou `miyukini-central` apparaît dans cette liste.
- Optionnel : règle Clippy ou custom lint si le graphe peut être interrogé depuis le build.

---

### 2.2 Vérification au runtime

| Identifiant | Règle | Moyen |
|-------------|--------|--------|
| **VERIF-RUNTIME-1** | Au bootstrap du COG, une phase « santé » vérifie que BondingBrother est bien connecté à KindMother et (si utilisé) à StrongFather. | Au démarrage du runtime COG : appel de test (ex. getStatus KindMother, ping StrongFather) ; si échec, démarrage en erreur ou mode dégradé documenté. |
| **VERIF-RUNTIME-2** | Les adaptateurs / produits enregistrés n'ont pas de handle direct sur Storage ni sur une API « back-door » d'écriture. | Revue des points d'injection (DI) : seuls KindMother (CoreDataAPI) et BondingBrother (façade) sont injectés aux Opérateurs ; pas de `Storage` ni de client SQL brut. |
| **VERIF-RUNTIME-3** | Le point d'accès utilisateur aux Services est bien Miyukini Central : pas de launcher alternatif enregistré comme entrée « catalogue » ou « mes services ». | Checklist déploiement ou test d'intégration : tous les parcours « utilisateur final → catalogue → lancement Service » passent par le Hub ; aucun raccourci direct vers un Opérateur qui contourne le Hub. |

**Implémentation immédiate :**

- Dans le code de bootstrap (ex. runtime ou binaire qui monte le COG) : séquence d'auto-contrôle (BondingBrother → KindMother, BondingBrother → StrongFather) et log/erreur si indisponible.
- Documentation déploiement : « Point d'entrée utilisateur = Miyukini Central ; ne pas exposer d'URL ou de raccourci direct vers un Opérateur en tant que catalogue. »

---

## 3. Flux cible et tableau récapitulatif

### 3.1 Flux utilisateur (point d'accès Central)

```
Utilisateur
    │
    ▼
Miyukini Central (seul point d'accès utilisateur pour les Services)
    │ intentions (catalogue, lancer Service, etc.)
    ▼
BondingBrother (médiation)
    │
    ├──► StrongFather (décision, Mandat)
    │
    └──► KindMother (CoreDataAPI : read, WriteIntent)
              │
              ▼
         Storage (interne à KindMother, invisible aux appelants)
```

### 3.2 Flux données (persistance)

| Étape | Acteur | Barrière |
|-------|--------|----------|
| 1 | Opérateur / Central | Émet une **intention** (ex. WriteIntent) vers BondingBrother. |
| 2 | BondingBrother | Traduit en demande CoreDataAPI (submitWriteIntent) vers KindMother. |
| 3 | KindMother | Valide (Runtime Boundaries), applique, persiste via Storage. |
| 4 | Storage | Utilisé **uniquement** par KindMother ; aucun autre crate n'y a accès. |

### 3.3 Tableau des barrières

| Barrière | Code | Implémentation immédiate |
|----------|------|---------------------------|
| Façade unique | BAR-FACADE-1 à 3 | Crate façade / BondingBrother comme seule dépendance Cores pour opérateurs et Central ; vérif. graphe de dépendances. |
| KindMother Storage | BAR-KM-1 à 3 | Storage non exposé ; persistance métier uniquement via CoreDataAPI. |
| StrongFather verdict | BAR-SF-1 à 3 | Trait DecisionAuthority / PolicyGateway ; implémentation prod = StrongFather. |
| BondingBrother médiation | BAR-BB-1 à 3 | API intentions / résultats ; pas de types KindMother/StrongFather dans l'API appelants. |

### 3.4 Tableau des vérifications

| Vérification | Code | Implémentation immédiate |
|--------------|------|---------------------------|
| Build : pas de dép. KM/SF dans opérateurs / Central | VERIF-BUILD-1, VERIF-BUILD-2 | Script CI sur Cargo.toml. |
| Build : Storage non exporté | VERIF-BUILD-3 | Revue API crate kindmother. |
| Runtime : BondingBrother ↔ KindMother / StrongFather | VERIF-RUNTIME-1 | Bootstrap : santé COG au démarrage. |
| Runtime : pas de handle Storage / back-door | VERIF-RUNTIME-2 | DI : seuls CoreDataAPI et façade injectés. |
| Runtime : point d'accès = Central | VERIF-RUNTIME-3 | Checklist / test : parcours utilisateur via Hub uniquement. |

---

## 4. Références

| Document | Description |
|----------|-------------|
| [Miyukini Conceptual References - Miyukini Central Hub Services](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Central%20Hub%20Services.md) | Hub : rôle, écrans, flux ; complété par la règle canonique du présent document. |
| [Miyukini Conceptual References - Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Terminologie officielle (Opérateur, Service, COG, etc.). |
| [Miyukini Conceptual References - Acces DB et Droits Agents IA](./Miyukini%20Conceptual%20References%20-%20Acces%20DB%20et%20Droits%20Agents%20IA.md) | Chemin des écritures (WriteIntent, KindMother, StrongFather). |
| [KindMother - CoreDataAPI](../../core/KindMother/contracts/api/KindMother%20-%20CoreDataAPI%20(Surface%20d'Appel%20Conceptuelle).md) | Surface d'appel KindMother ; absence de bypass. |
| [BondingBrother - Core Interaction Contract](../../core/BondingBrother/architecture/BondingBrother%20-%20Core%20Interaction%20Contract.md) | Interaction BondingBrother ↔ KindMother / StrongFather. |

---

**Date de création :** 2026-02-02  
**Version :** 1.0  
**Statut :** Document de référence normatif — Barrières techniques COG et point d'accès utilisateur Miyukini Central.
