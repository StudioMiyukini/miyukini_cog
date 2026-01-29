# Kernel - Invariants & Guarantees

## 1. Contexte

Ce document definit les **invariants non negociables** et les **garanties** offertes par le Kernel dans l'ecosysteme Miyukini. Il consolide les regles absolues extraites de la Definition Kernel et du Kernel Maintenance Observability Contract, formant le socle contractuel de la fondation technique.

**Documents fondateurs :**

- [Miyukini Core System - Definition Kernel](../Miyukini%20Core%20System%20-%20Definition%20Kernel.md)
- [Miyukini Conceptual References - Kernel Maintenance Observability Contract](../../reference/Miyukini%20Conceptual%20References%20-%20Kernel%20Maintenance%20Observability%20Contract.md)

**Statut contractuel :** Ce document est **contractuel, normatif, et non negociable**. Il derive directement des documents fondateurs et etablit les limites absolues du Kernel.

---

## 2. Portee / Scope

- **Applicable a :** Toute implementation, extension, ou utilisation du Kernel
- **Responsable :** Kernel (fondation technique minimale)
- **Consommateurs :** Tous les Cores (StrongFather, KindMother, BondingBrother, CaringNanny, BorderGuard, MasterButler, EverBuddy), tous les produits
- **Ne couvre pas :** Les invariants des Cores (voir leurs documents fondateurs respectifs), les regles metier (qui n'appartiennent jamais au Kernel)

---

## 3. Nature des invariants

### 3.1 Qu'est-ce qu'un invariant ?

Un **invariant** est une regle absolue qui :

- **Ne peut jamais etre violee** — Aucune exception, aucune derogation, aucun contournement
- **Est verifiable** — On peut toujours determiner si l'invariant est respecte ou non
- **Est independante du contexte** — L'invariant s'applique quelle que soit la situation
- **Est non negociable** — Aucune consideration pratique ne peut justifier sa violation

**Consequence d'une violation :** Toute violation d'un invariant constitue une **faute architecturale** qui compromet la fondation meme du systeme. Un Kernel qui viole un invariant rend tout l'ecosysteme instable.

### 3.2 Hierarchie des invariants

Les invariants du Kernel sont organises en trois categories :

| Categorie | Description | Invariants |
|-----------|-------------|------------|
| **Identite** | Definissent ce que le Kernel EST et N'EST PAS | INV-K-1, INV-K-2, INV-K-3, INV-K-4 |
| **Observabilite** | Definissent les capacites de maintenance | INV-K-5, INV-K-6, INV-K-7, INV-K-8 |
| **Autonomie** | Definissent les contraintes d'independance | INV-K-9, INV-K-10 |

---

## 4. Invariants d'identite

Ces invariants definissent la nature fondamentale du Kernel : ce qu'il est et ce qu'il ne doit jamais devenir.

### 4.1 INV-K-1 : Aucune logique metier

**Enonce canonique :**

> Le Kernel ne contient **jamais** de logique metier. Il ne connait ni les entites domaine (user, order, match), ni les regles de gestion, ni les workflows applicatifs.

| Aspect | Specification |
|--------|---------------|
| **Categorie** | Identite |
| **Portee** | Absolue |
| **Verification** | Aucune reference a un concept domaine ou une regle de gestion dans le code du Kernel |
| **Consequence de violation** | Couplage avec les produits, impossibilite de reutilisation |

**Ce que cela signifie concretement :**

| Autorise | Interdit |
|----------|----------|
| ✅ Generer un identifiant unique | ❌ Generer un "user_id" avec format specifique |
| ✅ Fournir l'heure courante | ❌ Calculer une date de peremption produit |
| ✅ Logger un message structure | ❌ Logger un evenement metier type "commande validee" |
| ✅ Charger une configuration | ❌ Definir des politiques de tarification |

**Source :** Definition Kernel - Section 1 "Ce que le kernel EST" et Section 3 "Exclusions explicites"

### 4.2 INV-K-2 : Aucune dependance externe critique

**Enonce canonique :**

> Le Kernel ne depend **jamais** d'un service externe pour fonctionner. Il doit pouvoir demarrer, tourner, et s'arreter sans aucun appel reseau obligatoire.

| Aspect | Specification |
|--------|---------------|
| **Categorie** | Identite |
| **Portee** | Absolue |
| **Verification** | Aucun appel reseau obligatoire au demarrage ou pendant le fonctionnement |
| **Consequence de violation** | Perte d'autonomie, dependance a des services tiers |

**Ce que cela signifie concretement :**

| Autorise | Interdit |
|----------|----------|
| ✅ Charger la configuration depuis un fichier local | ❌ Charger la configuration depuis un service distant obligatoire |
| ✅ Generer des IDs localement (UUID, ULID) | ❌ Appeler un service de generation d'IDs distribues |
| ✅ Utiliser l'horloge systeme locale | ❌ Synchroniser obligatoirement avec un serveur NTP |
| ✅ Logger vers stdout/fichier | ❌ Exiger un backend de logging distant |

**Lien avec les Lois d'Autonomie :** Cet invariant est une application directe de **LOI-1** (Aucune dependance externe critique a l'execution).

**Source :** Definition Kernel - Section 4 "Frontieres du Kernel"

### 4.3 INV-K-3 : Primitives locales sures uniquement

**Enonce canonique :**

> Le Kernel n'utilise que des **primitives locales et sures**. Pas d'operations non deterministes, pas d'effets de bord caches, pas d'etat global mutable partage.

| Aspect | Specification |
|--------|---------------|
| **Categorie** | Identite |
| **Portee** | Absolue |
| **Verification** | Audit du code pour verifier l'absence d'effets de bord non controles |
| **Consequence de violation** | Comportement imprevisible, impossibilite de tester |

**Ce que cela signifie concretement :**

| Autorise | Interdit |
|----------|----------|
| ✅ Fonctions pures | ❌ Fonctions avec effets de bord caches |
| ✅ Etat immutable ou controle | ❌ Variables globales mutables partagees |
| ✅ Operations deterministes | ❌ Random non injectable pour les tests |
| ✅ Erreurs explicites (Result<T, E>) | ❌ Panics silencieux ou exceptions non gerees |

**Implication Rust :** Le Kernel privilegie les types `Result<T, E>`, les structures immutables, et les traits bien definis.

**Source :** Definition Kernel - Section 2 "Responsabilites fondamentales"

### 4.4 INV-K-4 : Pas de protocole applicatif

**Enonce canonique :**

> Le Kernel n'implemente **jamais** de protocole applicatif. HTTP, WebSocket, gRPC, GraphQL, ou tout autre protocole de communication reste du ressort des produits.

| Aspect | Specification |
|--------|---------------|
| **Categorie** | Identite |
| **Portee** | Absolue |
| **Verification** | Aucune dependance a des crates de protocole applicatif |
| **Consequence de violation** | Couplage avec les choix technologiques des produits |

**Ce que cela signifie concretement :**

| Autorise | Interdit |
|----------|----------|
| ✅ Definir des traits abstraits | ❌ Implanter un serveur HTTP |
| ✅ Fournir des primitives de configuration | ❌ Gerer des routes REST |
| ✅ Logger vers une interface abstraite | ❌ Envoyer des metriques vers un backend specifique |
| ✅ Fournir un lifecycle generique | ❌ Integrer un middleware web |

**Frameworks exclus :** Axum, Actix, Rocket, Tonic, etc. restent des choix de produit.

**Source :** Definition Kernel - Section 1 "Ce que le kernel N'EST PAS"

---

## 5. Invariants d'observabilite

Ces invariants definissent les capacites et les limites du Kernel pour assister la maintenance du code. Ils sont issus du Kernel Maintenance Observability Contract.

### 5.1 INV-K-5 : Non-mutation (derive de INV-MOC-1)

**Enonce canonique :**

> Le Kernel ne modifie **jamais** le code, les configurations, ou les donnees pour "reparer" une situation. Il observe, atteste, compare, signale — mais ne corrige pas.

| Aspect | Specification |
|--------|---------------|
| **Categorie** | Observabilite |
| **Portee** | Absolue |
| **Verification** | Aucune operation de correction automatique dans le Kernel |
| **Consequence de violation** | Perte de controle humain, opacite du systeme |

**Capacites autorisees vs interdites :**

| Autorise | Interdit |
|----------|----------|
| ✅ Observer l'etat du systeme | ❌ Modifier l'etat pour corriger |
| ✅ Attester la conformite | ❌ Auto-reparer les violations |
| ✅ Comparer deux versions | ❌ Appliquer un patch automatique |
| ✅ Signaler une anomalie | ❌ Corriger l'anomalie |

**Formulation cle :**

> **Miyukini ne maintient pas le code a la place de l'humain. Il rend le code maintenable sans ambiguite.**

**Source :** Kernel Maintenance Observability Contract - Section 6 "INV-MOC-1"

### 5.2 INV-K-6 : Determinisme (derive de INV-MOC-2)

**Enonce canonique :**

> Toute observation ou attestation produit le **meme resultat** pour le meme etat d'entree. Le comportement du Kernel est reproductible et previsible.

| Aspect | Specification |
|--------|---------------|
| **Categorie** | Observabilite |
| **Portee** | Absolue |
| **Verification** | Tests de reproductibilite sur les operations d'observation |
| **Consequence de violation** | Impossibilite d'auditer, de comparer, de diagnostiquer |

**Ce que cela signifie concretement :**

| Autorise | Interdit |
|----------|----------|
| ✅ Empreinte comportementale stable | ❌ Empreinte variant selon le contexte |
| ✅ Comparaison deterministe | ❌ Resultats differents pour meme entree |
| ✅ Signature rejouable | ❌ Hash dependant de l'heure ou du random |

**Utilite :**

- Comparer deux versions du systeme
- Detecter une derive silencieuse
- Prouver qu'un build est "equivalent" fonctionnellement

**Source :** Kernel Maintenance Observability Contract - Section 6 "INV-MOC-2"

### 5.3 INV-K-7 : Explicabilite (derive de INV-MOC-3)

**Enonce canonique :**

> Toute information fournie par le Kernel est **comprehensible par un humain** sans connaissance du code source. L'observation est explicable, pas cryptique.

| Aspect | Specification |
|--------|---------------|
| **Categorie** | Observabilite |
| **Portee** | Absolue |
| **Verification** | Les messages, signaux, et rapports sont lisibles par un non-developpeur technique |
| **Consequence de violation** | Dependance aux experts, opacite du diagnostic |

**Ce que cela signifie concretement :**

| Autorise | Interdit |
|----------|----------|
| ✅ Messages d'erreur explicites | ❌ Codes d'erreur cryptiques |
| ✅ Tracabilite gouvernee | ❌ Stacktrace technique brut |
| ✅ Chemin de decision lisible | ❌ Dump memoire incomprehensible |
| ✅ Diagnostic comprehensible | ❌ Logs pour experts uniquement |

**Mode "maintenance explicable" :**

Lorsqu'un incident survient, le Kernel peut fournir :

- Pourquoi une decision est arrivee jusqu'ici
- Quels contrats ont ete traverses
- Ou la gouvernance s'est arretee

**Source :** Kernel Maintenance Observability Contract - Section 6 "INV-MOC-3"

### 5.4 INV-K-8 : Souverainete locale (derive de INV-MOC-4)

**Enonce canonique :**

> Les controles du Kernel fonctionnent **sans dependance externe** (reseau, SaaS, agent). Ils sont operationnels en environnement isole.

| Aspect | Specification |
|--------|---------------|
| **Categorie** | Observabilite |
| **Portee** | Absolue |
| **Verification** | Tous les controles fonctionnent offline |
| **Consequence de violation** | Perte de capacite en environnement isole |

**Caracteristiques des controles :**

| Caracteristique | Statut |
|-----------------|--------|
| Fonctionnent offline | ✅ |
| Ne necessitent aucun SaaS | ✅ |
| Ne demandent aucun agent externe | ✅ |
| Sont deterministes | ✅ |
| Sont rejouables | ✅ |

**Consequences pratiques :**

| Contexte | Compatible |
|----------|:----------:|
| Hardware faible (Raspberry Pi, mini PC) | ✅ |
| Environnement isole (air-gapped) | ✅ |
| Long cycle de version (LTS) | ✅ |
| Audit post-mortem | ✅ |

**Lien avec les Lois d'Autonomie :** Cet invariant est une application directe de **LOI-3** (L'etat local est souverain).

**Source :** Kernel Maintenance Observability Contract - Section 6 "INV-MOC-4"

---

## 6. Invariants d'autonomie

Ces invariants garantissent que le Kernel respecte les contraintes d'autonomie de l'ecosysteme Miyukini.

### 6.1 INV-K-9 : Cout proportionnel au hardware

**Enonce canonique :**

> Le Kernel doit tourner sur du **hardware simple** : mini PC, NAS, Raspberry Pi, VM isolee, serveur de terrain. La consommation de ressources est maitrisee et previsible.

| Aspect | Specification |
|--------|---------------|
| **Categorie** | Autonomie |
| **Portee** | Absolue |
| **Verification** | Benchmark sur Raspberry Pi 4 avec 4 Go de RAM |
| **Consequence de violation** | Exclusion des environnements a ressources limitees |

**Ce que cela signifie concretement :**

| Autorise | Interdit |
|----------|----------|
| ✅ Memoire maitrisee et previsible | ❌ Allocation memoire non bornee |
| ✅ CPU previsible, sans pics | ❌ Operations gourmandes non controlees |
| ✅ Pas de services fantomes | ❌ Workers dormants consommant des ressources |
| ✅ Demarrage rapide | ❌ Initialisation lourde bloquante |

**Question de verification :**

> *"Ce composant fonctionne-t-il de maniere acceptable sur un Raspberry Pi 4 avec 4 Go de RAM ?"*

**Lien avec les Lois d'Autonomie :** Application directe de **LOI-5** (Le cout doit etre proportionnel au hardware).

**Source :** Lois d'Autonomie Systeme - Section 4 "LOI-5"

### 6.2 INV-K-10 : Gouvernance preservee (derive de INV-MOC-5)

**Enonce canonique :**

> Aucune capacite du Kernel ne contourne la **chaine de gouvernance** (StrongFather, EverBuddy). Le Kernel reste subordonne aux decisions des autorites de l'ecosysteme.

| Aspect | Specification |
|--------|---------------|
| **Categorie** | Autonomie |
| **Portee** | Absolue |
| **Verification** | Les decisions strategiques passent par StrongFather |
| **Consequence de violation** | Rupture de la chaine de gouvernance, decisions non tracables |

**Ce que cela signifie concretement :**

| Autorise | Interdit |
|----------|----------|
| ✅ Fournir des informations pour la decision | ❌ Prendre une decision strategique |
| ✅ Executer les ordres de StrongFather | ❌ Contourner une decision de gouvernance |
| ✅ Signaler une violation | ❌ Appliquer une sanction autonome |
| ✅ Observer l'etat du systeme | ❌ Modifier l'etat sans autorisation |

**Relation avec les Cores :**

| Acteur | Role |
|--------|------|
| **StrongFather** | Decide l'autorisation des actions |
| **EverBuddy** | Valide la compatibilite des operations |
| **Kernel** | Execute et fournit les primitives |

**Source :** Kernel Maintenance Observability Contract - Section 6 "INV-MOC-5"

---

## 7. Garanties offertes

### 7.1 Nature des garanties

Une **garantie** est un engagement que le Kernel prend envers les Cores et les produits. Contrairement aux invariants (regles absolues), les garanties sont des promesses de service.

### 7.2 Garantie de reutilisabilite

**Enonce :**

> Le Kernel garantit que ses modules sont **reutilisables** par tout produit (SaaS, web, mobile, jeu) sans modification.

| Aspect | Specification |
|--------|---------------|
| **Ce que cela implique** | Un meme Kernel pour toutes les surfaces |
| **Comment c'est verifie** | Au moins deux produits utilisent le Kernel sans fork |
| **Qui en beneficie** | Tous les produits de l'ecosysteme |
| **Invariants associes** | INV-K-1, INV-K-4 |

### 7.3 Garantie de stabilite des contrats

**Enonce :**

> Le Kernel garantit que les **traits et types exposes** sont stables et versiones. Les changements de contrat sont explicitement documentes.

| Aspect | Specification |
|--------|---------------|
| **Ce que cela implique** | Les produits ne cassent pas lors d'une mise a jour mineure |
| **Comment c'est verifie** | Semver respecte, changelog maintenu |
| **Qui en beneficie** | Tous les produits dependant du Kernel |
| **Invariants associes** | INV-K-1, INV-K-3 |

### 7.4 Garantie de minimalisme

**Enonce :**

> Le Kernel garantit qu'il reste **minimal et focalise**. Aucun module n'est ajoute sans justification transverse.

| Aspect | Specification |
|--------|---------------|
| **Ce que cela implique** | Pas de bloat, pas d'abstractions prematurees |
| **Comment c'est verifie** | Regles d'ajout strictes (Definition Kernel Section 6) |
| **Qui en beneficie** | Maintenabilite long terme (5-10 ans) |
| **Invariants associes** | INV-K-1, INV-K-4, INV-K-9 |

**Regles pour AJOUTER un module :**

Toutes doivent etre vraies :

1. Au moins **2 produits ou 2 surfaces** en ont besoin
2. La responsabilite est **clairement infra** (pas de metier)
3. Le module reste **petit** et sans dependance business
4. Aucun produit existant ne peut raisonnablement le fournir sans duplication inutile

### 7.5 Garantie de transparence

**Enonce :**

> Le Kernel garantit que son comportement est **observable et explicable**. Aucune operation cachee, aucune magie.

| Aspect | Specification |
|--------|---------------|
| **Ce que cela implique** | Comprehension rapide par un nouveau developpeur |
| **Comment c'est verifie** | Documentation, logs structures, tracabilite |
| **Qui en beneficie** | Developpeurs, auditeurs, operateurs |
| **Invariants associes** | INV-K-5, INV-K-6, INV-K-7 |

### 7.6 Garantie d'autonomie operationnelle

**Enonce :**

> Le Kernel garantit qu'il fonctionne **sans dependance externe obligatoire**. Il est operationnel en environnement isole.

| Aspect | Specification |
|--------|---------------|
| **Ce que cela implique** | Demarrage, fonctionnement, arret sans reseau |
| **Comment c'est verifie** | Tests en environnement air-gapped |
| **Qui en beneficie** | Contextes industriels, terrain, associatifs |
| **Invariants associes** | INV-K-2, INV-K-8, INV-K-9 |

---

## 8. Matrice des invariants

### 8.1 Vue synthetique

| Invariant | Categorie | Enonce court | Relation principale |
|-----------|-----------|--------------|---------------------|
| **INV-K-1** | Identite | Aucune logique metier | Produits implementent le metier |
| **INV-K-2** | Identite | Aucune dependance externe critique | LOI-1 |
| **INV-K-3** | Identite | Primitives locales sures uniquement | Determinisme, testabilite |
| **INV-K-4** | Identite | Pas de protocole applicatif | Produits choisissent leur stack |
| **INV-K-5** | Observabilite | Non-mutation | Observer, pas corriger |
| **INV-K-6** | Observabilite | Determinisme | Reproductibilite |
| **INV-K-7** | Observabilite | Explicabilite | Comprehension humaine |
| **INV-K-8** | Observabilite | Souverainete locale | LOI-3, offline |
| **INV-K-9** | Autonomie | Cout proportionnel au hardware | LOI-5, ressources limitees |
| **INV-K-10** | Autonomie | Gouvernance preservee | StrongFather decide |

### 8.2 Interdependances

```
INV-K-1 ─────────────────────────────────────┐
(Aucune logique metier)                      │
         │                                    │
         └──────────► INV-K-4 ◄──────────────┘
                      (Pas de protocole)
                             │
                             ▼
                      Reutilisabilite
                      (Garantie 7.2)

INV-K-2 ─────────────────────────────────────┐
(Aucune dependance externe)                  │
         │                                    │
         └──────────► INV-K-8 ◄──────────────┘
                      (Souverainete locale)
                             │
                             ▼
                      Autonomie operationnelle
                      (Garantie 7.6)

INV-K-5 ─────────────────────────────────────┐
(Non-mutation)                               │
         │                                    │
         └──────────► INV-K-6 ──────► INV-K-7
                      (Determinisme)   (Explicabilite)
                             │
                             ▼
                      Transparence
                      (Garantie 7.5)

INV-K-9 ─────────────────────────────────────┐
(Cout proportionnel)                         │
         │                                    │
         └──────────► INV-K-10
                      (Gouvernance preservee)
                             │
                             ▼
                      Minimalisme
                      (Garantie 7.4)
```

---

## 9. Modules du Kernel et invariants

### 9.1 Modules v0.1

| Module | Responsabilite | Invariants principaux |
|--------|----------------|----------------------|
| **config** | Chargement configuration (env, fichiers, secrets) | INV-K-2, INV-K-3 |
| **id** | Generation d'identifiants (UUID/ULID) | INV-K-1, INV-K-3, INV-K-6 |
| **time** | Abstraction temps (now, timezone, tests) | INV-K-3, INV-K-6, INV-K-8 |
| **log** | Logging structure (niveaux, sortie) | INV-K-3, INV-K-7 |
| **lifecycle** | Boot / shutdown : ordre d'init, hooks d'arret | INV-K-1, INV-K-2, INV-K-10 |

### 9.2 Verification par module

Chaque module du Kernel doit respecter **tous** les invariants. Le tableau ci-dessus indique les invariants les plus directement pertinents pour chaque module.

---

## 10. References croisees

### Documents associes

| Document | Relation |
|----------|----------|
| [Miyukini Core System - Definition Kernel](../Miyukini%20Core%20System%20-%20Definition%20Kernel.md) | Document source (invariants d'identite) |
| [Miyukini Conceptual References - Kernel Maintenance Observability Contract](../../reference/Miyukini%20Conceptual%20References%20-%20Kernel%20Maintenance%20Observability%20Contract.md) | Document source (invariants d'observabilite) |
| [Miyukini Conceptual References - Lois Autonomie Systeme](../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) | Contraintes d'autonomie (LOI-1 a LOI-6) |
| [Miyukini Core System - Structure du Kernel](../Miyukini%20Core%20System%20-%20Structure%20du%20Kernel.md) | Architecture des crates |
| [Miyukini Core System - Revue Traits API v0.1](../Miyukini%20Core%20System%20-%20Revue%20Traits%20API%20v0.1.md) | Gel des traits publics |

### References glossaire

| Terme | Definition |
|-------|------------|
| **Invariant** | Regle absolue qui ne peut jamais etre violee |
| **Garantie** | Engagement de service que le Kernel prend envers le systeme |
| **Kernel** | Noyau technique minimal de la fondation Miyukini (pas un kernel OS) |
| **Primitives locales** | Operations sures, deterministes, sans dependance externe |
| **Souverainete locale** | Capacite a fonctionner en autonomie complete |

---

## 11. Synthese contractuelle

### Engagements de ce contrat

Ce contrat etablit que :

1. **Les invariants sont absolus** — 10 invariants non negociables definissent les limites du Kernel
2. **Les categories sont claires** — Identite, Observabilite, Autonomie organisent les invariants
3. **Les garanties sont formelles** — 5 garanties de service envers l'ecosysteme
4. **Les interdependances sont explicites** — Les invariants se renforcent mutuellement
5. **Les violations sont identifiables** — Chaque invariant est verifiable

### Phrase de synthese

> **Le Kernel respecte 10 invariants non negociables (identite, observabilite, autonomie) et offre 5 garanties formelles (reutilisabilite, stabilite, minimalisme, transparence, autonomie), formant le socle contractuel de la fondation technique Miyukini.**

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Contrat — Normatif  
**Reference :** Definition Kernel v0.1, Kernel Maintenance Observability Contract v1.0  
**Type :** Contrat de gouvernance — Invariants et Garanties
