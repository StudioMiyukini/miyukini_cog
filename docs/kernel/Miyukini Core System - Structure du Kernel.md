# Miyukini Core System — Structure du Kernel

## Contexte et lien au contrat

Ce document traduit le contrat défini dans [Miyukini Core System - Definition Kernel](Miyukini%20Core%20System%20-%20Definition%20Kernel.md) en **structure concrète** (crates, dépendances, visibilité, nommage, stabilité). Aucune implémentation n’y est décrite.

**La dépendance est strictement unidirectionnelle : les produits dépendent du kernel, jamais l’inverse.**

---

## 1. Crates

### Principe

Une **seule crate** pour le kernel en v0.1. Les cinq modules (config, id, time, log, lifecycle) sont des **sous‑modules** de cette crate. Cela garde le noyau minimal, évite la multiplication de versions et respecte la règle « pas de bloat ».

### Crate principale

| Crate | Rôle | Modules internes |
|------|------|------------------|
| **miyukini-kernel** | Point d’entrée unique. Ré‑exporte les traits et types publics des modules. | `config`, `id`, `time`, `log`, `lifecycle` |

- **Nom** : `miyukini-kernel` (convention Cargo : tirets).
- **Espace de noms** : les produits utilisent `miyukini_kernel::config`, `miyukini_kernel::log`, etc., ou les ré‑exports à la racine.

### Workspace

Le dépôt **miyukini-core-system** peut contenir un workspace Cargo avec :

- `crates/miyukini-kernel` — la crate du kernel ;
- d’éventuelles crates de produits ou d’exemples **en dehors** du kernel, qui dépendent de `miyukini-kernel`.

Le kernel **ne dépend d’aucune** autre crate du workspace (sauf des dépendances externes listées plus bas). Le workspace ne doit pas imposer de dépendances partagées ; chaque crate produit gère explicitement ses dépendances.

### Évolution (Phase 2 et au‑delà)

Si un module (ex. `connection`, `error`) doit un jour être publié séparément ou avoir un cycle de release différent, on peut :

- soit le garder en sous‑module avec un `pub mod` et un feature flag `unstable` ;
- soit l’extraire en crate dédiée **après** validation du besoin par 2+ produits.

En v0.1, **aucune** crate supplémentaire dans le kernel.

---

## 2. Dépendances

### Règle générale

Toute dépendance doit être **strictement infra et transverse**. Le kernel ne doit jamais devenir un point d’attraction de dépendances applicatives.

### Autorisées (v0.1)

| Crate | Usage | Justification |
|-------|--------|---------------|
| **std** | Toujours. | Base du langage. |
| **log** | Façade de logging. | Interface standard Rust pour les logs ; pas un backend. Le kernel peut l’utiliser ou implémenter un trait compatible. La crate log est utilisée uniquement comme façade standard ; le kernel ne configure aucun backend global. |
| **uuid** | Génération d’identifiants UUID. | Minimal, infra, sans dépendance applicative. |
| **ulid** | Génération d’identifiants ULID (optionnel si on préfère n’avoir que `uuid` au démarrage). | Idem. On peut commencer avec `uuid` seul et ajouter `ulid` quand un produit en a besoin. |

**Recommandation v0.1** : `std` + `log` + `uuid`. `ulid` : ajout lorsqu’un 2ᵉ produit ou le jeu en a besoin.

### À éviter en v0.1

| Crate / famille | Raison |
|-----------------|--------|
| **serde**, **serde_json**, **toml**, **yaml** | Le kernel ne impose pas de format de config ; il fournit des mécanismes de chargement. Le produit parse. |
| **anyhow**, **thiserror** | Utiles pour les erreurs. En v0.1, le module `error` est en Phase 2. On peut les introduire quand `error` entre au kernel, pas avant, pour ne pas faire fuiter ces types dans l’API publique. |
| **chrono** | `std::time` suffit pour `now()`, instant, duration. Timezone = choix du produit. |
| **dotenvy** | .env est un format. Le kernel peut exposer « lire les variables d’environnement » via `std::env` sans crate dédiée. |

### Interdites

Les dépendances suivantes **ne doivent pas** figurer dans la crate du kernel :

| Famille | Exemples | Raison (rappel du contrat) |
|---------|----------|----------------------------|
| **Runtime async / executor** | tokio, async-std | Le kernel définit des contrats ; le produit choisit le runtime. |
| **Serveurs / clients HTTP** | axum, actix-web, rocket, warp, reqwest, hyper | Hors périmètre. |
| **Données** | sqlx, diesel, redis, mongodb | Couche données = produit. |
| **Observabilité avancée** | tracing, tracing-subscriber, opentelemetry | Le kernel fournit au plus des hooks (log) ; pas d’intégration. |
| **Sérialisation / formats** | serde_json, toml, yaml | Chaque produit choisit ; le kernel ne impose pas. |
| **Auth / métier** | crates JWT, OAuth, validation, etc. | Métier et produit. |

L’ajout d’une dépendance non listée en « Autorisées » nécessite une justification **infra et transverse** et une mise à jour de ce document.

---

## 3. Visibilité (pub / interne)

### Ce qui est `pub`

- **Traits** du contrat : `Config`, `Logger`, `IdGenerator`, `Clock`, `Lifecycle` — et leurs méthodes.
- **Types** exposés dans le contrat : types de retour et paramètres de ces traits, types d’erreur publics du kernel.
- **Fonctions ou constructeurs** nécessaires pour faire tourner un produit : par ex. une implémentation par défaut instanciable (`default_logger`, `default_clock`) si le contrat le prévoit — sans imposer un backend ou un format.
- **Ré‑exports** utiles : `pub use config::Config` (ou équivalent) à la racine pour simplifier l’usage.

Règle : **tout ce qui apparaît dans le contrat (Definition Kernel) comme « le kernel expose » est public.**

### Ce qui est interne (`pub(crate)` ou privé)

- **Implémentations par défaut** (détails internes, formats, backends) — tant qu’elles ne font pas partie du contrat de traits.
- **Aides internes**, types auxiliaires, fonctions utilitaires non exposées.
- **Détails de chargement** (fichiers, env) qui ne sont pas dans l’interface `Config`.

Le kernel **ne ré‑exporte pas** de types de dépendances (`uuid::Uuid`, `log::Level`) sauf s’ils font explicitement partie du contrat (ex. « le trait `IdGenerator` retourne un type `Id` » défini par le kernel, qui peut envelopper ou utiliser `Uuid` en interne). Le type `Id` du kernel est opaque ; son format interne n’est pas garanti et ne doit pas être persisté ou interprété sans passer par les APIs du produit.

---

## 4. Conventions de nommage

### Crates

- **snake_case** pour le nom Cargo : `miyukini-kernel` (Cargo accepte les tirets, qui sont affichés en `_` dans `extern crate` / imports).

### Modules

- **snake_case** : `config`, `id`, `time`, `log`, `lifecycle`. Un seul mot quand c’est possible.

### Traits

- **PascalCase** : `Config`, `Logger`, `IdGenerator`, `Clock`, `Lifecycle`. Noms courts, alignés sur le contrat.

### Types

- **PascalCase** : `Config`, `Logger`, etc. Pour les implémentations par défaut : `DefaultLogger`, `DefaultClock` — préfixe `Default` ou nom explicite qui indique qu’il s’agit d’une implémentation remplaçable.

### Fonctions et méthodes

- **snake_case** : `load`, `now`, `generate`, `info`, `shutdown`.

### Constantes

- **SCREAMING_SNAKE_CASE** si ce sont des constantes purement techniques (noms de clés internes, niveaux par défaut) ; à éviter dans l’API publique pour ne pas figer une politique.

### Fichiers de modules

- Un fichier par module : `config.rs`, `id.rs`, `time.rs`, `log.rs`, `lifecycle.rs`. Si un module grossit, sous‑dossier `config/mod.rs` avec sous‑modules en `pub(crate)`.

---

## 5. Stable vs expérimental

### Stable (contractuellement figé)

En v0.1, les **traits et les signatures des méthodes** des modules **config, id, time, log, lifecycle** sont considérés comme **stables** : on ne les modifie pas sans versioning explicite ni sans mettre à jour ce document et le Definition Kernel.

- **Stable** = le contrat (noms, types, sens des opérations) ne change pas de façon breaking entre patchs. En semver 0.x, des changements breaking restent possibles, mais ils sont **documentés** et **annoncés**.
- Les **implémentations par défaut** (comportement, backends, formats) peuvent évoluer en mode **compatible** (extension, correction de bug) tant qu’elles restent « minimales et remplaçables » et n’imposent pas un backend ou un format.

### Expérimental

- **Tout nouveau module** (Phase 2 : `connection` / `pool`, `error`) est **expérimental** tant qu’il n’a pas été validé par au moins 2 produits et que son contrat n’a pas été figé. On le signale dans la doc (attribut `#[doc(hidden)]` ou section « Unstable ») et, si utile, derrière un feature **`unstable`** ou **`connection`**, **`error`**.
- **Nouvelles méthodes ou paramètres** ajoutés à un module existant, en cours d’itération, sont **expérimentaux** jusqu’à validation. On évite de les promettre dans le contrat tant qu’ils ne sont pas stabilisés.

### Règle de passage stable → expérimental

Un élément **ne doit pas** passer de stable à expérimental : on peut déprécier, puis supprimer à la prochaine version majeure, mais on ne « dé‑stabilise » pas un contrat déjà promis.

---

## 6. Résumé des règles

| Thème | Règle |
|-------|--------|
| **Crates** | Une seule crate `miyukini-kernel` en v0.1 ; modules `config`, `id`, `time`, `log`, `lifecycle`. |
| **Dépendances autorisées** | `std`, `log`, `uuid` (et `ulid` quand 2+ produits). |
| **Dépendances interdites** | Runtime async, HTTP, DB, tracing/OpenTelemetry, sérialisation, auth, métier. |
| **Visibilité** | `pub` = contrat (traits, types, constructeurs exposés). Le reste = `pub(crate)` ou privé. |
| **Nommage** | Crates : tirets ; modules : snake_case ; traits/types : PascalCase ; fonctions : snake_case. |
| **Stable** | Contrats des 5 modules v0.1 ; pas de changement breaking non documenté. |
| **Expérimental** | Nouveaux modules (Phase 2) et nouvelles méthodes/params en cours de validation. |

Ce document doit être mis à jour dès qu’une crate, une dépendance, une règle de visibilité ou de stabilité change.
