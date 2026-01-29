# Kernel — Index de Navigation

## Contexte

Le Kernel est la **fondation technique minimale et reutilisable** du Miyukini Core System. Il constitue la couche d'execution et de coordination sur laquelle s'appuient tous les produits (SaaS, web, mobile, jeu) sans jamais contenir de logique metier.

Le Kernel represente les **briques transversales essentielles** : identifiants, temps, configuration, logs, lifecycle. Toutes les surfaces (web, mobile, jeu) en dependent pour ces besoins fondamentaux.

**Dans le cadre de Miyukini Core System, le terme « kernel » designe le noyau technique minimal de la fondation, et non un kernel systeme au sens OS.**

**Strate :** 0 (Fondation)  
**Role :** Infrastructure technique minimale et transverse  
**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## Question fondamentale

> **"Quelles briques techniques minimales sont necessaires pour que tout produit puisse demarrer, fonctionner et s'arreter proprement ?"**

Cette question se decline en :
- Quels mecanismes de configuration sont strictement necessaires ?
- Comment generer des identifiants de maniere uniforme ?
- Comment abstraire le temps pour les tests et l'audit ?
- Comment assurer un logging structure sans imposer de backend ?
- Comment gerer le cycle de vie (boot/shutdown) des briques techniques ?

---

## Structure de la documentation

### Foundation

Documents fondateurs definissant l'identite et le role du Kernel.

| Document | Description |
|----------|-------------|
| [Definition Kernel](./Miyukini%20Core%20System%20-%20Definition%20Kernel.md) | Definition conceptuelle, perimetre, responsabilites, exclusions |
| [Structure du Kernel](./Miyukini%20Core%20System%20-%20Structure%20du%20Kernel.md) | Crates, dependances, visibilite, conventions de nommage |
| [Revue Traits API v0.1](./Miyukini%20Core%20System%20-%20Revue%20Traits%20API%20v0.1.md) | Gel des traits publics, specification de l'API |

---

### Architecture

Documentation architecturale.

| Document | Description |
|----------|-------------|
| [Architecture & Components](./architecture/Kernel%20-%20Architecture%20&%20Components.md) | Composants du Kernel, relations internes, points d'extension |

---

### Contracts

Contrats normatifs et non negociables.

| Document | Description |
|----------|-------------|
| [Invariants & Guarantees](./contracts/Kernel%20-%20Invariants%20&%20Guarantees.md) | Catalogue consolide des invariants INV-K-1 a INV-K-10 |
| [Security Boundaries Contract](./contracts/Kernel%20-%20Security%20Boundaries%20Contract.md) | Frontieres de securite — Ce que le Kernel ne fournit pas |

---

### Implementation

Guides d'implementation.

| Document | Description |
|----------|-------------|
| [Reference Implementation Guidelines](./implementation/Kernel%20-%20Reference%20Implementation%20Guidelines.md) | Patterns d'implementation, regles de tests, exemples Rust |

---

### Reference

Documentation de reference.

| Document | Description |
|----------|-------------|
| [FAQ & Common Questions](./reference/Kernel%20-%20FAQ%20&%20Common%20Questions.md) | Questions frequentes |
| [Vocabulary & Glossary](./reference/Kernel%20-%20Vocabulary%20&%20Glossary.md) | Vocabulaire canonique du Kernel |

---

## Invariants cles

| Invariant | Description |
|-----------|-------------|
| **INV-K-1** | Aucune logique metier — Le Kernel ne contient aucune regle business, auth, facturation, gameplay |
| **INV-K-2** | Aucune dependance externe critique — Fonctionnement complet sans appel reseau obligatoire |
| **INV-K-3** | Primitives locales sures uniquement — Pas de runtime async, pas de serveur HTTP, pas de client externe |
| **INV-K-4** | Pas de protocole applicatif — Aucun HTTP, WebSocket, gRPC, ou format de serialisation impose |
| **INV-K-5** | Non-mutation — Le Kernel ne modifie jamais le code, configurations ou donnees pour "reparer" |
| **INV-K-6** | Determinisme — Toute observation ou attestation produit le meme resultat pour le meme etat d'entree |
| **INV-K-7** | Explicabilite — Toute information fournie est comprehensible sans connaissance du code source |
| **INV-K-8** | Souverainete locale — Les controles fonctionnent sans dependance externe (reseau, SaaS, agent) |
| **INV-K-9** | Cout proportionnel au hardware — Le Kernel tourne sur hardware simple, consommation maitrisee et previsible |
| **INV-K-10** | Gouvernance preservee — Aucune capacite du Kernel ne contourne la chaine de gouvernance (StrongFather, EverBuddy) |

---

## Interdictions

| Code | Interdiction |
|------|--------------|
| **INTERD-K-1** | Le Kernel ne peut pas contenir de logique metier (auth, facturation, gameplay) |
| **INTERD-K-2** | Le Kernel ne peut pas dependre d'un runtime async (tokio, async-std) |
| **INTERD-K-3** | Le Kernel ne peut pas integrer de serveur ou client HTTP |
| **INTERD-K-4** | Le Kernel ne peut pas persister de donnees (ORM, SQL, Redis) |
| **INTERD-K-5** | Le Kernel ne peut pas imposer un format de serialisation (JSON, protobuf, etc.) |
| **INTERD-K-6** | Le Kernel ne peut pas contenir d'observabilite avancee (tracing, OpenTelemetry) |
| **INTERD-K-7** | Le Kernel ne peut pas executer de correction automatique |
| **INTERD-K-8** | Le Kernel ne peut pas dependre d'un service externe pour fonctionner |

---

## Modules v0.1

| Module | Responsabilite | Trait principal |
|--------|----------------|-----------------|
| **config** | Chargement configuration (env, fichiers) | `Config::get(&self, key: &str) -> Option<&str>` |
| **id** | Generation d'identifiants (UUID/ULID) | `IdGenerator::generate(&self) -> Id` |
| **time** | Abstraction temps (now, tests) | `Clock::now(&self) -> SystemTime` |
| **log** | Logging structure par niveau | `Logger::log(&self, level: Level, message: &str)` |
| **lifecycle** | Boot / shutdown des briques techniques | `Lifecycle::register_shutdown_hook`, `shutdown` |

---

## Types exposes (API v0.1 gelee)

| Module | Types publics |
|--------|---------------|
| **config** | `Config`, `EnvConfig` |
| **id** | `Id`, `IdParseError`, `IdGenerator`, `UuidIdGenerator` |
| **time** | `Clock`, `DefaultClock` |
| **log** | `Level`, `Logger`, `DefaultLogger` |
| **lifecycle** | `Lifecycle`, `DefaultLifecycle` |

---

## Relations avec les Cores

| Core | Relation |
|------|----------|
| **StrongFather** | Consommateur — Utilise les briques Kernel (config, log, id, time, lifecycle) |
| **KindMother** | Consommateur — Utilise les briques Kernel pour persistance et observabilite |
| **Bonding Brother** | Consommateur — Utilise les briques Kernel comme fondation technique |
| **Caring Nanny** | Consommateur — Utilise le logging et time pour l'observation |
| **Border Guard** | Consommateur — Utilise les briques Kernel pour classification |
| **Master Butler** | Consommateur — Utilise les briques Kernel pour orchestration technique |
| **Ever Buddy** | Consommateur — Utilise les briques Kernel pour validation |
| **LogisticsSteward** | Fournisseur — Fournit l'etat systeme abstrait (lecture seule) pour les decisions d'arbitrage des ressources |

### Diagramme de dependances

```
                    ┌─────────────────────────────────────┐
                    │           Produits                  │
                    │  (SaaS, Web, Mobile, Jeu)           │
                    └───────────────┬─────────────────────┘
                                    │
                    ┌───────────────▼─────────────────────┐
                    │             Cores                   │
                    │  StrongFather, KindMother, etc.     │
                    └───────────────┬─────────────────────┘
                                    │
                    ┌───────────────▼─────────────────────┐
                    │            KERNEL                   │
                    │  config | id | time | log | lifecycle│
                    └─────────────────────────────────────┘

    Dependance strictement unidirectionnelle :
    Produits → Cores → Kernel
    Jamais l'inverse.
```

---

## Conformite aux Lois d'Autonomie Systeme

Le Kernel est **fondamental pour l'autonomie** selon les [Lois d'Autonomie Systeme](../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) :

| Loi | Conformite | Note |
|-----|------------|------|
| **LOI-1** | ✅ Role critique | Aucune dependance externe — primitives locales uniquement |
| **LOI-2** | ✅ | Kernel fonctionne en isolation totale |
| **LOI-3** | ✅ | Configuration et logs locaux souverains |
| **LOI-4** | ✅ | Time abstrait, pas de dependance a une horloge reseau |
| **LOI-5** | ✅ Role critique | Minimal, leger, prevu pour hardware simple |
| **LOI-6** | ✅ | Kernel neutre, n'empeche pas la federation |

---

## Concepts cles

| Concept | Description |
|---------|-------------|
| **Brique technique** | Composant minimal reutilisable par tous les produits |
| **Contrat** | Interface (trait) definissant les capacites exposees |
| **Implementation par defaut** | Implementation remplacable fournie par le Kernel |
| **Stabilite** | Garantie que les signatures ne changent pas sans versioning |
| **Phase** | Etape d'evolution du Kernel (v0.1, Phase 2, etc.) |
| **Surface** | Type de produit consommateur (web, mobile, jeu, worker) |

---

## Phrase fondatrice

> **Le Kernel est la fondation technique minimale et agnostique qui fournit les briques transversales (config, id, time, log, lifecycle) a tous les produits Miyukini, sans jamais contenir de logique metier, de protocole applicatif, ou de dependance externe critique.**

---

## Frontieres de securite

Le Kernel ne fournit **aucune fonctionnalite de securite active**. La securite est assuree par les Cores et les Security Engines.

### Ce que le Kernel ne fournit PAS

| Fonctionnalite | Fournie par | Raison |
|----------------|-------------|--------|
| Authentification | Security Engines | Logique metier (INV-K-1) |
| Cryptographie | Security Engines | Protocole applicatif (INV-K-4) |
| Controle d'acces | Master Butler, StrongFather | Logique metier (INV-K-1) |
| Validation des entrees | Validation Engine | Logique metier (INV-K-1) |
| Detection d'anomalies | Caring Nanny, Integrity Engine | Logique metier (INV-K-1) |

### Contribution du Kernel a la securite

Le Kernel contribue a la securite **par sa nature** :

| Principe | Comment le Kernel l'applique |
|----------|------------------------------|
| Surface d'attaque minimale | Uniquement 5 modules techniques |
| Comportement deterministe | Audit et forensic facilites |
| Souverainete locale | Resilience aux attaques externes |
| Gouvernance preservee | Subordination aux Cores |

Pour les details, voir [Kernel - Security Boundaries Contract](./contracts/Kernel%20-%20Security%20Boundaries%20Contract.md).

### Documentation securite associee

| Document | Description |
|----------|-------------|
| [Security - Core Integration Map](../security/architecture/Security%20-%20Core%20Integration%20Map.md) | Cartographie des roles securite par Core |
| [Security - Documentation Fondatrice](../security/foundation/Security%20-%20Documentation%20Fondatrice.md) | Vision operationnelle de la securite |
| [Doctrine Securite Fondamentale](../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md) | Principes fondateurs |

---

## Documents de reference

- [Miyukini Conceptual References - Glossaire](../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) — Terminologie officielle
- [Miyukini Conceptual References - Pyramide Architecture Complete](../reference/Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) — Position du Kernel
- [Miyukini Conceptual References - Kernel Maintenance Observability Contract](../reference/Miyukini%20Conceptual%20References%20-%20Kernel%20Maintenance%20Observability%20Contract.md) — Capacites bas niveau
- [Miyukini Conceptual References - Lois Autonomie Systeme](../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) — Conformite aux lois

---

## Gel et Versionnement

| Document | Description |
|----------|-------------|
| [Audit Phase 3 Verification](./Kernel%20-%20Audit%20Phase%203%20Verification.md) | Audit de verification Phase 3 |
| [Gel et Versionnement v0.1](./Kernel%20-%20Gel%20et%20Versionnement%20v0.1.md) | Acte de gel officiel de la documentation v0.1 |

---

**Date de creation :** 2026-01-28  
**Version :** 0.1.0  
**Statut :** GELE — Documentation de reference v0.1.0
