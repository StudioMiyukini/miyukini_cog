# Kernel — Vocabulary & Glossary

## Contexte

Ce document constitue le **dictionnaire officiel** de la terminologie specifique au Kernel Miyukini. Il regroupe les definitions des modules, traits, types, capacites de maintenance et invariants propres au substrat technique neutre.

**Ce glossaire est la source de verite terminologique pour le Kernel.**

## Portee / Scope

- **Applicable a :** Documentation Kernel, implementation, tests
- **Audience :** Developpeurs, architectes, mainteneurs
- **Statut :** Document de reference normatif — GLOSSAIRE KERNEL
- **Relation :** Complete le [Glossaire General Miyukini](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## A

### Abstraction temps (Time Abstraction)

**Module `time` du Kernel.** Fournit un point de verite unique pour `now()`, timezone, horodatage. Permet l'injection en test et evite les appels systeme disperses.

**Voir aussi :** Clock, DefaultClock

---

## B

### Boot

**Phase de demarrage** geree par le module `lifecycle`. Comprend le chargement de la configuration, l'initialisation des briques techniques, dans un ordre maitrise.

**Ce que le boot N'EST PAS :**

- ❌ Orchestration de workflows metier
- ❌ Initialisation de jobs applicatifs
- ❌ Hooks applicatifs

**Voir aussi :** Lifecycle, Shutdown

---

## C

### Carte de complexite structurelle (Complexity Heatmap)

**Capacite de maintenance du Kernel** qui expose des metriques structurelles sans analyser le metier.

| Metrique | Description |
|----------|-------------|
| **Profondeur de graphe** | Nombre de niveaux de dependances |
| **Densite de dependances** | Ratio connexions/composants |
| **Zones a fort couplage** | Composants tres interconnectes |
| **Zones a faible stabilite** | Composants frequemment modifies ou fragiles |

**Utilite :** Anticiper la dette technique, planifier les refactorings — **sans jamais toucher au code.**

**Voir aussi :** Kernel Maintenance Observability

---

### Clock

**Trait public du module `time`.** Interface d'abstraction pour l'horloge systeme.

**Signature gelee (v0.1) :**

```rust
trait Clock {
    fn now(&self) -> SystemTime;  // std::time::SystemTime
}
```

**Caracteristiques :**

- Retourne `std::time::SystemTime` (type standard, pas de dependance)
- Timezone = choix du produit (pas de chrono dans le kernel)
- Permet l'injection de FakeClock en test

**Voir aussi :** DefaultClock, Abstraction temps

---

### Config

**Trait public du module `config`.** Interface d'acces a la configuration.

**Signature gelee (v0.1) :**

```rust
trait Config {
    fn get(&self, key: &str) -> Option<&str>;
}
```

**Caracteristiques :**

- Le kernel fournit le mecanisme d'acces, pas la politique
- Le produit choisit les cles et les valeurs
- Pas de typage (int, bool) — le produit parse

**Ce que Config NE fait PAS :**

- ❌ Definir les noms de variables
- ❌ Imposer une structure metier
- ❌ Fournir des methodes `get_i64`, `get_bool`

**Voir aussi :** EnvConfig, Configuration

---

### Configuration

**Module `config` du Kernel.** Gestion du chargement et de l'acces a la configuration (env, fichiers, secrets).

**Sources supportees en v0.1 :**

- Variables d'environnement (via `EnvConfig`)

**Sources Phase 2+ :**

- Fichiers
- Secrets (vault)

**Voir aussi :** Config, EnvConfig

---

## D

### DefaultClock

**Implementation par defaut** du trait `Clock`. Utilise `std::time::SystemTime::now()`.

**Statut :** Implementation, hors contrat du trait — peut evoluer.

**Voir aussi :** Clock

---

### DefaultLifecycle

**Implementation par defaut** du trait `Lifecycle`. Execute les hooks de shutdown en ordre LIFO.

**Comportement :**

- Un second appel a `shutdown()` est no-op (implementation-dependant)
- Les panics dans les hooks se propagent

**Statut :** Implementation, hors contrat du trait — peut evoluer.

**Voir aussi :** Lifecycle

---

### DefaultLogger

**Implementation par defaut** du trait `Logger`. Minimal et remplacable.

**Caracteristiques :**

- N'impose pas de backend
- N'impose pas de format
- Le produit peut implementer un Logger custom

**Statut :** Implementation, hors contrat du trait — peut evoluer.

**Voir aussi :** Logger

---

### Detection d'ambiguite contractuelle (Contract Ambiguity Detection)

**Capacite de maintenance du Kernel** pour signaler les zones non claires du systeme.

| Signal | Description |
|--------|-------------|
| **Contrat invoque mais incomplet** | Contrat partiellement defini ou utilise |
| **Invariant jamais active** | Invariant declare mais jamais verifie |
| **Regle jamais rencontree** | Politique jamais evaluee en runtime |

> **Pas une erreur. Un signal de maintenance.**

**Utilite :** Alleger le systeme, detecter le code mort, preparer les refactorings.

**Voir aussi :** Kernel Maintenance Observability

---

### Divergence silencieuse (Silent Divergence)

**Situation detectable par le Kernel** ou un systeme declare une version mais presente une empreinte comportementale differente.

**Causes typiques :**

- Build recompile differemment
- Dependance modifiee silencieusement
- Compilation non reproductible
- Injection de code ou modification post-build

**Caracteristiques :**

- Signal de maintenance, pas d'erreur
- Detectable sans reseau
- Deterministe et rejouable

> **Le Kernel signale la divergence mais ne la corrige jamais.**

**Voir aussi :** Empreinte comportementale, INV-MOC-1

---

## E

### Empreinte comportementale (Behavior Fingerprint)

**Signature structurelle** du systeme charge, produite par le Kernel.

| Element capture | Description |
|-----------------|-------------|
| **Ordre de chargement** | Sequence d'initialisation des composants |
| **Graphe d'appel structurel** | Relations entre composants (pas metier) |
| **Contrats invoques** | Liste des contrats actives |
| **Invariants sollicites** | Invariants verifies au chargement |

**Caracteristiques :**

- C'est une signature, pas un log
- Aucun contenu metier
- Aucune donnee runtime
- Deterministe et rejouable

> **L'empreinte observe et atteste, mais ne corrige jamais.**

**Voir aussi :** Divergence silencieuse, Kernel Maintenance Observability

---

### EnvConfig

**Implementation par defaut** du trait `Config` utilisant les variables d'environnement.

**Constructeur :**

```rust
EnvConfig::from_env() -> Self
```

**Statut :** Implementation, hors contrat du trait — peut evoluer.

**Voir aussi :** Config, Configuration

---

## G

### Gel local (Local Freeze)

**Capacite du Kernel** a marquer un composant comme gele structurellement, sans affecter le reste du systeme.

| Action | Description |
|--------|-------------|
| **Marquer un composant comme gele** | Gele structurellement |
| **Refuser son remplacement** | Blocage du rechargement |
| **Laisser le reste evoluer** | Isolation du gel |

**Gouvernance :**

| Acteur | Role |
|--------|------|
| **StrongFather** | Decide l'autorisation du gel |
| **EverBuddy** | Valide la compatibilite du gel |
| **Kernel** | Execute le gel et l'applique |

> **Le gel est decide par la gouvernance, execute par le Kernel, jamais inverse.**

**Voir aussi :** Kernel Maintenance Observability

---

## I

### Id

**Type opaque du module `id`.** Represente un identifiant unique genere par le Kernel.

**API gelee (v0.1) :**

```rust
struct Id { /* opaque */ }
impl Id {
    pub fn parse(s: &str) -> Result<Id, IdParseError>;
}
// Display, Debug, Clone, Copy, Eq, Hash, PartialEq pour Id
```

**Caracteristiques :**

- Format interne non garanti (UUID actuellement)
- Ne pas persister ou interpreter sans passer par les APIs du produit
- `Debug` = usage developpement uniquement, pas pour persistance
- `Display` = serialisation pour le produit

**Voir aussi :** IdGenerator, IdParseError, Identifiants

---

### IdGenerator

**Trait public du module `id`.** Interface de generation d'identifiants.

**Signature gelee (v0.1) :**

```rust
trait IdGenerator {
    fn generate(&self) -> Id;
}
```

**Caracteristiques :**

- Pas de parametre (seed, determinisme) en v0.1
- Le produit peut implementer un generateur deterministe pour le jeu
- Le kernel ne reenvoie pas `uuid::Uuid`

**Voir aussi :** Id, UuidIdGenerator, Identifiants

---

### IdParseError

**Type d'erreur du module `id`.** Erreur lors du parsing d'un identifiant.

**Traits implementes :**

- `Debug`
- `Display`
- `Error`

**Caracteristiques :**

- Pas de `From<uuid::Error>` pour ne pas lier au detail d'impl
- Type minimal

**Voir aussi :** Id, IdGenerator

---

### Identifiants (module `id`)

**Module du Kernel** pour la generation d'identifiants uniques (UUID, ULID, derives deterministes).

**Types publics :**

- `Id` — Type opaque
- `IdParseError` — Erreur de parsing
- `IdGenerator` — Trait
- `UuidIdGenerator` — Implementation

**Voir aussi :** Id, IdGenerator

---

### INV-K-1 a INV-K-8 (Invariants Kernel)

**Invariants fondamentaux du Kernel** extraits du contrat de definition et du contrat de maintenance.

| Invariant | Enonce |
|-----------|--------|
| **INV-K-1** | Aucune logique metier dans le kernel |
| **INV-K-2** | Aucune dependance externe critique |
| **INV-K-3** | Primitives locales sures uniquement |
| **INV-K-4** | Pas de protocole applicatif (HTTP, WebSocket, etc.) |
| **INV-K-5** | Non-mutation (INV-MOC-1) |
| **INV-K-6** | Determinisme (INV-MOC-2) |
| **INV-K-7** | Explicabilite (INV-MOC-3) |
| **INV-K-8** | Souverainete locale (INV-MOC-4) |

**Voir aussi :** INV-MOC-1 a INV-MOC-5

---

### INV-MOC-1 a INV-MOC-5 (Invariants Maintenance Observability)

**Invariants du contrat de maintenance observability.**

| Invariant | Enonce |
|-----------|--------|
| **INV-MOC-1** | Le Kernel ne modifie jamais le code, les configurations, ou les donnees pour "reparer" une situation |
| **INV-MOC-2** | Toute observation ou attestation produit le meme resultat pour le meme etat d'entree |
| **INV-MOC-3** | Toute information fournie est comprehensible par un humain sans connaissance du code source |
| **INV-MOC-4** | Les controles fonctionnent sans dependance externe (reseau, SaaS, agent) |
| **INV-MOC-5** | Aucune capacite d'observation ne contourne la chaine de gouvernance (StrongFather, EverBuddy) |

**Voir aussi :** Kernel Maintenance Observability

---

## K

### Kernel

**Substrat technique neutre** de l'ecosysteme Miyukini. Fondation technique reutilisable, agnostique, sans logique metier.

**Modules (v0.1) :**

| Module | Responsabilite |
|--------|----------------|
| **config** | Chargement de la configuration |
| **id** | Generation d'identifiants |
| **time** | Abstraction temps |
| **log** | Logging structure |
| **lifecycle** | Boot / shutdown |

**Ce que le kernel EST :**

- Fondation technique reutilisable
- Couche d'execution et de coordination
- Ensemble de briques transversales
- Agnostique produit

**Ce que le kernel N'EST PAS :**

- ❌ Un framework applicatif (Axum, Actix, Rocket)
- ❌ Un ORM ou une couche d'acces donnees
- ❌ Le lieu du metier (auth, facturation, gameplay)
- ❌ Une suite d'outils d'ops (APM, tracing distribue)

**Voir aussi :** miyukini-kernel

---

### Kernel Maintenance Observability

**Ensemble de capacites bas niveau du Kernel** pour assister la maintenance du code sans jamais executer de correction automatique.

**Capacites incluses :**

| # | Capacite | Utilite principale |
|---|----------|-------------------|
| 1 | Empreinte comportementale | Comparaison, equivalence |
| 2 | Detecteur de divergence | Audit, securite |
| 3 | Carte de complexite | Planification, dette technique |
| 4 | Gel local | Stabilisation, SLA |
| 5 | Detection d'ambiguite | Simplification, code mort |
| 6 | Maintenance explicable | Diagnostic, tracabilite |

**Ce que le Kernel PEUT faire :**

- Observer, attester, comparer, signaler, expliquer

**Ce que le Kernel ne peut JAMAIS faire :**

- ❌ Corriger, muter, auto-reparer

> **Miyukini ne maintient pas le code a la place de l'humain. Il rend le code maintenable sans ambiguite.**

**Voir aussi :** INV-MOC-1 a INV-MOC-5

---

## L

### Level

**Enumeration du module `log`.** Niveaux de logging.

**Valeurs gelees (v0.1) :**

```rust
enum Level { Error, Warn, Info, Debug, Trace }
```

**Caracteristiques :**

- Enum kernel, pas de reexport de `log::Level`
- Pas de niveau custom en v0.1
- Aligne avec la facade log standard Rust

**Voir aussi :** Logger, Logging structure

---

### Lifecycle

**Trait public du module `lifecycle`.** Interface de gestion du cycle de vie (boot/shutdown).

**Signature gelee (v0.1) :**

```rust
trait Lifecycle {
    fn register_shutdown_hook<F>(&mut self, f: F)
    where
        F: FnMut() + 'static;

    fn shutdown(&mut self);
}
```

**Caracteristiques :**

- Shutdown uniquement (pas d'init hooks)
- Hooks executes en LIFO
- `FnMut` pour raison technique Rust (`Box<dyn FnOnce>`)
- `'static` : pas de capture de references ephemeres

**Ce que Lifecycle NE fait PAS :**

- ❌ Orchestration de workflows metier
- ❌ Jobs metier
- ❌ Hooks applicatifs

**Voir aussi :** DefaultLifecycle, Boot, Shutdown

---

### Logger

**Trait public du module `log`.** Interface de logging.

**Signature gelee (v0.1) :**

```rust
trait Logger {
    fn log(&self, level: Level, message: &str);
}
```

**Caracteristiques :**

- Une seule methode avec niveau explicite
- Le produit formate en JSON ou autre avant l'appel
- Pas de methodes `info()`, `warn()`, `error()` separees

**Ce que Logger NE fournit PAS :**

- ❌ `log_with_fields` ou type `Fields`
- ❌ Methodes par niveau (`info`, `warn`, etc.)
- ❌ Backend ou format impose

**Voir aussi :** Level, DefaultLogger, Logging structure

---

### Logging structure (module `log`)

**Module du Kernel** pour le logging par niveau avec format structure.

**Types publics :**

- `Level` — Niveaux de log
- `Logger` — Trait
- `DefaultLogger` — Implementation

**Caracteristiques :**

- Le produit choisit les backends et les niveaux
- Le kernel fournit le contrat et une implementation par defaut
- L'implementation par defaut est minimale et remplacable

**Voir aussi :** Logger, Level, DefaultLogger

---

## M

### Maintenance explicable (Explainable Maintenance)

**Mode de diagnostic du Kernel** pour fournir une tracabilite gouvernee lors d'incidents.

**Informations fournies :**

- Pourquoi une decision est arrivee jusqu'ici
- Quels contrats ont ete traverses
- Ou la gouvernance s'est arretee

**Ce qui n'est JAMAIS fourni :**

| Exclusion | Raison |
|-----------|--------|
| Stacktrace classique | Fuite d'information technique |
| Dump memoire | Fuite de donnees sensibles |
| Donnees utilisateur | Protection vie privee |

> **Le diagnostic explique le chemin de gouvernance, jamais l'implementation.**

**Voir aussi :** Kernel Maintenance Observability

---

### miyukini-kernel

**Crate Rust principale** du kernel. Point d'entree unique, reexporte les traits et types publics des modules.

**Espace de noms :**

- `miyukini_kernel::config`
- `miyukini_kernel::id`
- `miyukini_kernel::time`
- `miyukini_kernel::log`
- `miyukini_kernel::lifecycle`

**Conventions :**

- Nom Cargo : `miyukini-kernel` (tirets)
- Import Rust : `miyukini_kernel` (underscores)

**Voir aussi :** Kernel

---

## S

### Shutdown

**Phase d'arret** geree par le module `lifecycle`. Execute les hooks d'arret des briques techniques en ordre LIFO.

**Caracteristiques :**

- Ordre inverse d'enregistrement (LIFO)
- Les panics se propagent
- Un second appel est implementation-dependant

**Voir aussi :** Lifecycle, Boot

---

### Stable vs Experimental

**Classification de maturite** des elements du kernel.

**Stable (contractuellement fige) :**

- Traits et signatures des 5 modules v0.1
- Pas de changement breaking non documente
- Les implementations par defaut peuvent evoluer en mode compatible

**Experimental :**

- Tout nouveau module (Phase 2 : `connection`, `error`)
- Nouvelles methodes ou parametres en cours d'iteration
- Signale dans la doc ou derriere feature flag `unstable`

> **Un element ne doit pas passer de stable a experimental.**

**Voir aussi :** API v0.1

---

### SystemTime

**Type standard Rust** (`std::time::SystemTime`) utilise comme retour de `Clock::now()`.

**Caracteristiques :**

- Le kernel ne reexporte pas ce type
- Le produit l'importe de `std::time` si besoin
- Conversion timezone = choix du produit (chrono, etc.)

**Voir aussi :** Clock

---

## U

### UuidIdGenerator

**Implementation par defaut** du trait `IdGenerator` utilisant UUID v4.

**Statut :** Implementation, hors contrat du trait — peut evoluer.

**Voir aussi :** IdGenerator, Id

---

## V

### v0.1 (API Version)

**Premiere version gelee** de l'API publique du kernel. Les traits et types listes sont stables.

**Elements geles :**

| Module | Traits | Types |
|--------|--------|-------|
| **config** | `Config` | `EnvConfig` |
| **id** | `IdGenerator` | `Id`, `IdParseError`, `UuidIdGenerator` |
| **time** | `Clock` | `DefaultClock` |
| **log** | `Logger` | `Level`, `DefaultLogger` |
| **lifecycle** | `Lifecycle` | `DefaultLifecycle` |

**A ne pas faire en v0.1 :**

- ❌ Ajouter `Config::load`, `Config::get_i64`
- ❌ Ajouter `IdGenerator::generate_with_seed`
- ❌ Ajouter `Logger::info`, `Logger::log_with_fields`
- ❌ Ajouter `Lifecycle::register_init_hook`
- ❌ Reexporter `uuid::Uuid`, `log::Level`

**Voir aussi :** Stable vs Experimental

---

## Tableau de correspondance terminologique

| Terme incorrect | Terme correct |
|-----------------|---------------|
| Core Kernel | **Kernel** (le Kernel n'est pas un Core) |
| Kernel Module | **Module** (ex: module `config`) |
| Logger Interface | **Trait `Logger`** |
| ID Type | **Type `Id`** |
| Time Module | **Module `time`** ou **Abstraction temps** |
| Boot Hook | ❌ N'existe pas (shutdown uniquement) |
| Auto-correction | ❌ Interdit (INV-MOC-1) |

---

## Phrases Fondatrices

### Frontiere du Kernel

> **Le Kernel est le substrat technique neutre. Il ne decide pas, il ne gouverne pas, il ne connait pas le metier.**

### Maintenance

> **Miyukini ne maintient pas le code a la place de l'humain. Il rend le code maintenable sans ambiguite.**

### Autonomie

> **Les controles fonctionnent offline, sans SaaS, sans agent externe, de maniere deterministe et rejouable.**

### Non-mutation

> **Le Kernel ne modifie jamais le code, les configurations, ou les donnees pour "reparer" une situation.**

---

**Date de creation :** 2026-01-28  
**Version :** 1.0  
**Statut :** Document de reference normatif — GLOSSAIRE KERNEL

**References croisees :**

- [Miyukini Core System - Definition Kernel](../Miyukini%20Core%20System%20-%20Definition%20Kernel.md) : Definition du kernel
- [Miyukini Core System - Structure du Kernel](../Miyukini%20Core%20System%20-%20Structure%20du%20Kernel.md) : Crates, dependances, visibilite
- [Miyukini Core System - Revue Traits API v0.1](../Miyukini%20Core%20System%20-%20Revue%20Traits%20API%20v0.1.md) : Gel des traits publics
- [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) : Glossaire general
- [Miyukini Conceptual References - Kernel Maintenance Observability Contract](../../reference/Miyukini%20Conceptual%20References%20-%20Kernel%20Maintenance%20Observability%20Contract.md) : Capacites bas niveau
