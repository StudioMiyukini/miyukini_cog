# Kernel â€” FAQ & Common Questions

## Contexte

Ce document repond aux questions frequemment posees sur le **Kernel Miyukini**. Il clarifie le role, les responsabilites, les limites et l'utilisation du kernel pour les developpeurs, architectes et nouveaux contributeurs.

Le Kernel est le **substrat technique neutre** de l'ecosysteme Miyukini â€” une fondation minimale, sans logique metier, sur laquelle s'appuient tous les produits.

## Portee / Scope

- **Applicable a :** Kernel (miyukini-kernel)
- **Audience :** Developpeurs, architectes, nouveaux contributeurs, integrateurs
- **Statut :** Document de reference â€” FAQ

---

## 1. Questions fondamentales

### Q1.1 : Qu'est-ce que le Kernel Miyukini ?

Le Kernel Miyukini est la **fondation technique reutilisable** sur laquelle s'appuient plusieurs produits (SaaS, web, mobile, jeu). C'est une couche d'execution et de coordination qui fournit des briques transversales : identifiants, temps, configuration, logs, lifecycle.

**Ce que le Kernel EST :**

- Une fondation technique reutilisable
- Une couche d'execution et de coordination (boot, config, arret, observabilite de base)
- Un ensemble de briques transversales en Rust
- Agnostique produit

**Ce que le Kernel N'EST PAS :**

- Un framework applicatif complet
- Un ORM ou une couche d'acces donnees
- Le lieu du metier (auth, facturation, gameplay)
- Une suite d'outils d'ops

### Q1.2 : Le Kernel est-il un kernel systeme (OS) ?

**Non.** Dans le cadre de Miyukini Core System, le terme Â« kernel Â» designe le **noyau technique minimal de la fondation**, et non un kernel systeme au sens OS.

Le Kernel Miyukini ne gere pas :

- La memoire systeme
- Les processus OS
- Les drivers materiels
- L'ordonnancement systeme

Il fournit uniquement des **primitives applicatives transverses** (config, log, id, time, lifecycle) pour les produits construits dessus.

### Q1.3 : Quelle est la difference entre le Kernel et les Cores ?

| Aspect | Kernel | Cores |
|--------|--------|-------|
| **Strate** | Entre 0 et 3 | Strate 4 |
| **Role** | Substrat technique neutre | Moteurs conceptuels de gouvernance |
| **Contenu** | Id, Logger, Clock, Config, Lifecycle | StrongFather, KindMother, Caring Nanny, etc. |
| **Metier** | Aucun | Gouvernance, decisions, persistance |
| **Execution** | Fournit des primitives | Ne font qu'observer, decider, gouverner |

Le Kernel est le **socle technique** sur lequel les Cores s'appuient. Les Cores utilisent le Kernel (pour les logs, les IDs, le temps), mais le Kernel ne connait pas les Cores.

---

## 2. Modules et responsabilites

### Q2.1 : Quels modules composent le Kernel ?

En v0.1, le Kernel contient **5 modules** :

| Module | Responsabilite | Beneficiaires |
|--------|----------------|---------------|
| **config** | Chargement de la configuration (env, fichiers, secrets) | SaaS, workers, serveurs de jeu |
| **id** | Generation d'identifiants (UUID/ULID) | API, BDD, evenements |
| **time** | Abstraction temps (now, timezone, tests) | Audit, jobs, simulation |
| **log** | Logging structure (niveaux, sortie) | Tous les binaires |
| **lifecycle** | Boot / shutdown : ordre d'init, hooks d'arret | API, worker, serveur de jeu |

### Q2.2 : Pourquoi le Kernel n'a que 5 modules ?

Le principe directeur est : **l'ensemble le plus petit pour livrer un premier produit et prouver la reutilisation**.

Chaque module doit :

1. Etre utile a **au moins deux types de produits** (ex. SaaS + jeu, ou web + worker)
2. Etre **strictement infra** (pas de metier)
3. Rester **petit** et sans dependance business

### Q2.3 : Peut-on ajouter des modules au Kernel ?

Oui, mais sous conditions strictes. **Toutes** ces regles doivent etre vraies :

1. Au moins **2 produits ou 2 surfaces** en ont besoin
2. La responsabilite est **clairement infra** (pas de metier)
3. Le module reste **petit** et sans dependance business
4. Aucun produit existant ne peut raisonnablement le fournir sans duplication inutile

**En cas de doute, on n'ajoute pas.** Il est plus facile d'extraire du produit vers le kernel que de retirer du kernel.

### Q2.4 : Qu'est-ce qui est prevu pour la Phase 2 ?

- `connection` / `pool` : Abstractions minimales pour DB, Redis, etc.
- `error` : Types d'erreur partages

Ces modules entreront au kernel **quand au moins deux produits en ont besoin** sans duplication raisonnable.

---

## 3. Dependances et integrations

### Q3.1 : Quelles dependances le Kernel autorise-t-il ?

**Autorisees (v0.1) :**

| Crate | Usage |
|-------|-------|
| **std** | Base du langage |
| **log** | Facade de logging (interface standard Rust) |
| **uuid** | Generation d'identifiants UUID |

`ulid` sera ajoute quand un 2e produit ou le jeu en aura besoin.

### Q3.2 : Quelles dependances sont interdites dans le Kernel ?

| Famille | Exemples | Raison |
|---------|----------|--------|
| Runtime async / executor | tokio, async-std | Le produit choisit le runtime |
| Serveurs / clients HTTP | axum, actix-web, reqwest | Hors perimetre |
| Donnees | sqlx, diesel, redis | Couche donnees = produit |
| Observabilite avancee | tracing, opentelemetry | Le kernel fournit au plus des hooks |
| Serialisation / formats | serde_json, toml, yaml | Chaque produit choisit |
| Auth / metier | crates JWT, OAuth | Metier et produit |

### Q3.3 : Le Kernel supporte-t-il async/Tokio ?

**Non directement.** Le Kernel definit des **contrats** (traits, types) ; le choix de Tokio ou d'un autre runtime reste au produit qui l'integre.

Le Kernel ne doit jamais imposer un runtime async pour rester minimal et agnostique.

### Q3.4 : Le Kernel depend-il de services externes (SaaS, API, cloud) ?

**Non.** Le Kernel respecte la **LOI-1** (Lois d'Autonomie) :

> **Aucune dependance externe critique a l'execution.**

Le Kernel fonctionne **sans reseau**, **sans SaaS**, **sans agent externe**. Toutes ses capacites sont locales, deterministes et rejouables.

### Q3.5 : Comment utiliser le Kernel dans mon produit ?

```rust
use miyukini_kernel::config::EnvConfig;
use miyukini_kernel::id::UuidIdGenerator;
use miyukini_kernel::time::DefaultClock;
use miyukini_kernel::log::{DefaultLogger, Level};
use miyukini_kernel::lifecycle::DefaultLifecycle;

fn main() {
    // Configuration
    let config = EnvConfig::from_env();
    let db_url = config.get("DATABASE_URL");

    // Identifiants
    let id_gen = UuidIdGenerator::new();
    let user_id = id_gen.generate();

    // Temps
    let clock = DefaultClock;
    let now = clock.now();

    // Logs
    let logger = DefaultLogger;
    logger.log(Level::Info, "Application demarree");

    // Lifecycle
    let mut lifecycle = DefaultLifecycle::new();
    lifecycle.register_shutdown_hook(|| {
        println!("Fermeture des ressources...");
    });
    // ... application ...
    lifecycle.shutdown();
}
```

---

## 4. Maintenance et observabilite

### Q4.1 : Le Kernel corrige-t-il automatiquement les erreurs ?

**Non, jamais.** C'est un principe fondamental (INV-MOC-1) :

> **Le Kernel ne modifie jamais le code, les configurations, ou les donnees pour "reparer" une situation.**

Le Kernel peut :

- âœ… Observer
- âœ… Attester
- âœ… Comparer
- âœ… Signaler
- âœ… Expliquer

Le Kernel ne peut **jamais** :

- âŒ Corriger
- âŒ Muter
- âŒ Auto-reparer

**Miyukini ne maintient pas le code a la place de l'humain. Il rend le code maintenable sans ambiguite.**

### Q4.2 : Qu'est-ce que l'empreinte comportementale ?

L'**empreinte comportementale** (Behavior Fingerprint) est une signature structurelle du systeme charge, produite par le Kernel.

Elle capture :

- L'ordre de chargement des composants
- Le graphe d'appel structurel (pas metier)
- Les contrats invoques
- Les invariants sollicites

**Utilite :**

- Comparer deux versions du systeme
- Detecter une derive silencieuse
- Prouver qu'un build est "equivalent" fonctionnellement

### Q4.3 : Qu'est-ce qu'une divergence silencieuse ?

Une **divergence silencieuse** est une situation ou :

- Un systeme declare une meme version
- Mais presente une empreinte comportementale differente

**Causes typiques :**

- Build recompile differemment
- Dependance modifiee silencieusement
- Compilation non reproductible
- Injection de code ou modification post-build

Le Kernel **signale** la divergence mais ne la corrige jamais.

### Q4.4 : Qu'est-ce que le gel local ?

Le **gel local** est la capacite du Kernel a marquer un composant comme gele structurellement, sans affecter le reste du systeme.

**Actions permises :**

- Marquer un composant comme gele
- Refuser son remplacement ou rechargement
- Laisser le reste du systeme evoluer

**Gouvernance :**

| Acteur | Role |
|--------|------|
| StrongFather | Decide l'autorisation du gel |
| EverBuddy | Valide la compatibilite du gel |
| Kernel | Execute le gel et l'applique |

Le gel est **decide par la gouvernance**, **execute par le Kernel**, jamais inversÃ©.

### Q4.5 : Le Kernel peut-il fonctionner offline ?

**Oui, completement.** Tous les controles du Kernel :

- Fonctionnent offline
- Ne necessitent aucun SaaS
- Ne demandent aucun agent externe
- Sont deterministes
- Sont rejouables

Compatible avec :

- Hardware faible (Raspberry Pi, mini PC)
- Environnement isole (air-gapped)
- Long cycle de version (LTS)
- Audit post-mortem

---

## 5. API et traits publics

### Q5.1 : Quels sont les traits publics du Kernel ?

| Trait | Methode(s) gelee(s) |
|-------|---------------------|
| **Config** | `get(&self, key: &str) -> Option<&str>` |
| **IdGenerator** | `generate(&self) -> Id` |
| **Clock** | `now(&self) -> SystemTime` |
| **Logger** | `log(&self, level: Level, message: &str)` |
| **Lifecycle** | `register_shutdown_hook`, `shutdown` |

Ces signatures sont **figees** pour la v0.1. Tout changement est un breaking change documente.

### Q5.2 : Pourquoi `Config` n'a pas de methodes `get_i64`, `get_bool`, etc. ?

Le Kernel ne impose pas de politique de configuration. Il fournit uniquement le mecanisme d'acces (`get` retourne `Option<&str>`). Le **produit** decide comment parser et interpreter les valeurs.

Cela respecte le principe : **le kernel fournit les mecanismes, le produit definit les politiques**.

### Q5.3 : Pourquoi `Logger` n'a qu'une seule methode `log` ?

Pour rester minimal. Au lieu d'avoir `info()`, `warn()`, `error()`, etc., le trait a une seule methode avec un parametre `Level` explicite.

Le produit qui veut du logging structure :

- Formate le `message` en JSON ou cle-valeur avant l'appel
- Ou implemente un `Logger` custom

### Q5.4 : Le type `Id` est-il opaque ?

**Oui.** Le champ interne (`uuid::Uuid`) n'est pas expose. Le format interne n'est pas garanti et ne doit pas etre persiste ou interprete sans passer par les APIs du produit.

Pour persister un ID :

- Utiliser `id.to_string()` (via `Display`)
- Reconstruire avec `Id::parse(&str)`

`Debug` est implemente pour le developpement uniquement, pas pour la persistance.

### Q5.5 : Pourquoi `Lifecycle` n'a pas de `register_init_hook` ?

Le contrat du Kernel couvre **l'arret** (shutdown), pas l'orchestration de l'initialisation. L'init reste au produit qui enchaine config, log, etc. selon ses besoins.

Ajouter des hooks d'init risquerait de glisser vers de l'orchestration metier, ce qui est **explicitement exclu** du Kernel.

---

## 6. Exclusions et limites

### Q6.1 : Pourquoi l'auth n'est pas dans le Kernel ?

L'authentification (JWT, OAuth, sessions, RBAC) est du **metier specifique produit**. Un module auth peut importer le kernel ; le kernel n'importe pas l'auth.

**Regle :** Si c'est du metier, ca reste dans le produit.

### Q6.2 : Pourquoi pas de serde/JSON dans le Kernel ?

Le Kernel ne impose pas de format de configuration ou de serialisation. Chaque produit choisit ses formats (JSON, TOML, YAML, etc.) et ses libraries (serde, etc.).

Le Kernel fournit des mecanismes de chargement et d'acces ; le produit parse.

### Q6.3 : Pourquoi pas de metriques/tracing/OpenTelemetry dans le Kernel ?

Ce sont des outils d'ops avances. Le Kernel fournit au plus des **hooks** (via `Logger`), pas l'integration complete.

Le produit qui a besoin de tracing distribue l'ajoute dans sa couche applicative.

### Q6.4 : Le Kernel gere-t-il les erreurs metier ?

**Non.** Le module `error` est en Phase 2 et concernera des types d'erreur **infra** partages (pas metier).

Les erreurs metier (utilisateur non trouve, paiement echoue, etc.) restent dans le produit.

---

## 7. Evolution et compatibilite

### Q7.1 : Comment le Kernel evolue-t-il ?

Le Kernel grandit **uniquement** quand une necessite **transversale** et **infra** apparait.

Priorites :

1. Stabilite des contrats (traits, signatures)
2. Ne pas casser les produits existants
3. Dependances externes minimales et justifiees

### Q7.2 : Qu'est-ce qui est stable vs experimental ?

**Stable (v0.1) :** Les traits et les signatures des methodes des 5 modules (config, id, time, log, lifecycle). Le contrat ne change pas de facon breaking entre patchs.

**Experimental :** Tout nouveau module (Phase 2) tant qu'il n'a pas ete valide par au moins 2 produits. Signale par `#[doc(hidden)]` ou feature `unstable`.

### Q7.3 : Comment migrer si l'API change ?

En semver 0.x, des changements breaking restent possibles mais sont **documentes** et **annonces**.

Lors d'un changement :

1. La documentation est mise a jour (Definition Kernel, Structure du Kernel, Revue Traits API)
2. Le changement est versionne explicitement
3. Les produits affectes sont identifies

---

## 8. Cas d'usage

### Q8.1 : Le Kernel peut-il servir pour un SaaS ?

**Oui.** C'est le cas d'usage prioritaire. Le Kernel fournit :

- Configuration (env, secrets)
- Identifiants (UUID pour les entites)
- Horodatage (audit, logs)
- Logging structure
- Lifecycle (demarrage, arret propre)

### Q8.2 : Le Kernel peut-il servir pour un serveur de jeu ?

**Oui.** Le Kernel est agnostique surface. Un serveur de jeu utilise :

- Config : parametres du serveur
- Id : identifiants de joueurs, d'entites
- Time : ticks, simulation (avec injection en test)
- Log : observabilite
- Lifecycle : arret propre

### Q8.3 : Le Kernel peut-il servir pour du mobile ?

**Potentiellement.** Si du Rust partage (logique offline, etc.) tourne cote mobile, les memes contrats (config, id, time, log) s'appliquent. Pas de contrat specifique Â« mobile Â» dans le kernel.

---

## References croisees

- [Miyukini Core System - Definition Kernel](../Miyukini%20Core%20System%20-%20Definition%20Kernel.md) â€” Document fondateur
- [Miyukini Core System - Structure du Kernel](../Miyukini%20Core%20System%20-%20Structure%20du%20Kernel.md) â€” Crates, dependances, visibilite
- [Miyukini Core System - Revue Traits API v0.1](../Miyukini%20Core%20System%20-%20Revue%20Traits%20API%20v0.1.md) â€” Gel des traits publics
- [Kernel - Invariants & Guarantees](../contracts/Kernel%20-%20Invariants%20%26%20Guarantees.md) â€” Catalogue des invariants
- [Kernel - Architecture & Components](../architecture/Kernel%20-%20Architecture%20%26%20Components.md) â€” Composants et relations
- [Kernel - Reference Implementation Guidelines](../implementation/Kernel%20-%20Reference%20Implementation%20Guidelines.md) â€” Patterns d'implementation
- [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) â€” Terminologie officielle
- [Miyukini Conceptual References - Kernel Maintenance Observability Contract](..//..//miyukini-webway-system//reference//_index.md) â€” Capacites de maintenance

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** REFERENCE â€” FAQ

