# MGE — Kernel Specification

Spécification normative du microkernel mge-core : référence unique pour l'implémentation v0.1, la stabilité 5+ ans et l'interopérabilité IA.

## Table des matières

1. [Vision du Kernel](#1-vision-du-kernel)
2. [Responsabilités strictes](#2-responsabilités-strictes)
3. [Engine](#3-engine)
4. [World](#4-world)
5. [Scheduler](#5-scheduler)
6. [Event System](#6-event-system)
7. [RNG](#7-rng)
8. [Gestion du temps](#8-gestion-du-temps)
9. [CPU Budget et Profiling](#9-cpu-budget-et-profiling)
10. [Non-objectifs explicites](#10-non-objectifs-explicites)
11. [Design pour LLM](#11-design-pour-llm)
12. [Extensions prévues (hors kernel)](#12-extensions-prévues-hors-kernel)
13. [Annexes](#annexes)

---

## Contexte

- **Applicable à :** Implémentation mge-core v0.1, développement de plugins, Composer/LLM.
- **Audience :** Développeurs moteur, architectes, LLM.
- **Statut :** Spécification normative.

---

## 1. Vision du Kernel

### 1.1 Pourquoi un microkernel

| Alternative | Problème | Choix MGE |
|-------------|----------|-----------|
| **Monolithe** (Godot, Unity) | Couplage fort, impossible headless pur, refactors massifs. | Microkernel = couplage nul entre domaines. |
| **Rendering-first** (Bevy) | La boucle = frame ; simulation liée au rendu. | Simulation-first = tick pur, rendu optionnel. |
| **ECS tout-en-un** | Query DSL, phases hardcodées, callbacks. | API minimale, phases opaques, lecture explicite. |
| **Couche au-dessus moteur tiers** | Dépendance runtime, LOI-1 violée. | MGE autonome, crate Rust, binaire statique. |

Le microkernel garantit : **isolation**, **testabilité** (simulation sans fenêtre), **déterminisme** (ordre fixe, RNG contrôlé), **longévité** (pas de couplage = pas de refactor en chaîne).

### 1.2 Ce que contient le kernel

| Composant | Rôle |
|-----------|------|
| **Engine** | Cycle de vie, tick(), configuration, accès aux sous-systèmes. |
| **World** | Entités, composants SoA, archetypes, itération. |
| **Scheduler** | Ordre déterministe des systèmes, PhaseId opaques, budget CPU. |
| **EventQueue** | Événements typés, buffer double, lecture explicite. |
| **RNG** | Seed globale, seed par entité, reproductibilité. |
| **Time** | Delta, fixed timestep, time scale, pause. |
| **Profiling hooks** | Mesure par système, par phase ; détection overflow (exposition métriques uniquement). |

Rien d'autre. Le kernel est **minimal par design**.

### 1.3 Ce que le kernel ne contient pas

- Physique, collisions, hitbox.
- Rendu, fenêtre, GPU, sprites, caméra.
- Audio, input (clavier, souris, manette).
- Réseau, MWS, snapshot.
- Composants métier (Position, Health, Inventory).
- Grille 2D, chunks, tuiles.
- Logique gameplay (combat, quêtes, loot).

Ces éléments sont fournis par des **plugins** ou par le **Game Runtime** (boucle input → tick → render).

### 1.4 Ce que le kernel interdit explicitement

| Interdiction | Raison |
|--------------|--------|
| `run()` ou boucle intégrée | Le core ne connaît pas input/render ; separation of concerns. |
| `subscribe(callback)` ou handlers dynamiques | Dynamic dispatch, allocations, couplage caché, non-déterministe. |
| Phases gameplay hardcodées (Physics, Render, etc.) | Le kernel est généraliste ; les phases sont des conventions de plugins. |
| Reflection, introspection runtime | Complexité, allocations, instabilité ; pas de magie. |
| `rand::random()` ou `std::time` dans la simulation | Rupture de reproductibilité ; tout passe par le RNG kernel. |
| Query DSL ou macros complexes | Microkernel = API lisible, monomorphisation, pas de typage magique. |
| Logique métier dans les composants | Les composants sont des données pures (Send + Sync). |

---

## 2. Responsabilités strictes

| Composant | Responsabilité | Ce qu'il ne fait pas |
|-----------|----------------|----------------------|
| **Engine** | Création, config, add_plugin, build, tick ; accès world/events/time/rng. | Ne connaît pas la boucle, le rendu, l'input. |
| **World** | spawn, despawn, insert, get, iter2/iter3 ; stockage SoA par archetype. | Pas de Query dynamique, pas de logique. |
| **Scheduler** | Exécution ordonnée des systèmes par PhaseId ; mesure durée. | Pas de phases sémantiques, pas de parallélisme implicite. |
| **EventQueue** | emit(), iter::<E>() ; buffer double. | Pas de subscribe, pas de callbacks. |
| **RNG** | Génération déterministe ; seed global, seed par entité. | Pas d'accès rand externe. |
| **Time** | Delta, fixed timestep, accumulateur, time scale, pause. | Pas de notion de frame (rendu). |
| **Profiling** | Hooks avant/après système ; temps par phase, total tick ; détection overflow (métriques exposées, pas de décision). | Pas d'allocation dans le hot path. Pas de stratégie de réaction (plugin-level). |

---

## 3. Engine

### 3.1 Cycle de vie minimal

```
Engine::new(config)  →  add_plugin(...)  →  build()  →  tick() (répété)
```

| Phase | Description |
|-------|-------------|
| **new** | Création avec EngineConfig (seed, headless, fixed_timestep_ms, tick_budget_ms). Aucune résolution. |
| **add_plugin** | Enregistrement des plugins avant build. Ordre significatif pour résolution des dépendances. |
| **build** | Résolution topologique des dépendances, enregistrement des composants/systèmes/phases, initialisation. |
| **tick** | Avancement d'une unité de simulation. Appelé par le Game Runtime dans une boucle externe. |

### 3.2 Pourquoi tick() et pas run()

| Critère | run() intégré | tick() (choix MGE) |
|---------|---------------|-------------------|
| **Headless** | La boucle assume souvent une fenêtre (vsync, input). | Le runtime décide : tick seul en boucle (serveur) ou tick + render (client). |
| **Séparation des concerns** | Le core devient implicitement rendering-aware (frame, FPS). | Le core ignore le rendu ; le plugin rendu orchestre frames et vsync. |
| **Déterminisme** | run() mélange souvent delta temps réel et logique. | tick() reçoit un delta explicite ; fixed timestep trivial. |
| **Testabilité** | Boucle bloquante difficile à mocker. | Tests : N appels à tick() puis assert sur l'état. |
| **Lockstep réseau** | Une boucle unique complique le lockstep. | Tick = unité de simulation ; le plugin network envoie par tick. |

**Contrat tick() :** Le contenu d'un tick ne dépend que de l'état avant le tick et du delta time. Aucune notion de frame ni de rendu.

### 3.3 Schéma ASCII — Tick lifecycle

```
┌─────────────────────────────────────────────────────────────────────┐
│  TICK N                                                              │
│  1. Time::advance()      → delta, tick_counter++                     │
│  2. EventQueue::swap()   → buffer écriture ←→ buffer lecture         │
│  3. Scheduler::run()    → systèmes PhaseId ordre croissant           │
│     - Systèmes émettent events (buffer écriture)                     │
│     - Systèmes lisent events du tick N-1 (buffer lecture)          │
│  4. Profiling hooks     → métriques par système, phase, total        │
└─────────────────────────────────────────────────────────────────────┘
```

### 3.4 Configuration (conceptuelle)

```rust
/// Structure de configuration Engine (conceptuel)
pub struct EngineConfig {
    pub seed: u64,
    pub headless: bool,
    pub fixed_timestep_ms: Option<u32>,  // Si None, delta variable
    pub tick_budget_ms: Option<u32>,     // Budget CPU par tick
    // frame_budget, résolution = plugin rendu, pas le core
}
```

---

## 4. World

### 4.1 EntityId opaque

- **Format :** `EntityId` = (index, generation). Opaque : pas d'accès direct aux champs.
- **Raison :** Éviter les use-after-free (entité détruite puis réutilisée). La generation invalide les références obsolètes.
- **Création :** `world.spawn()` retourne un EntityId valide.
- **Suppression :** `world.despawn(id)` invalide l'id ; les lookups futurs retournent None.

### 4.2 SoA — Structure of Arrays

**Choix :** Chaque type de composant = tableau contigu. Les entités avec le même ensemble de composants partagent les mêmes tableaux (archetype).

**Pourquoi SoA vs AoS :**

| AoS (Array of Structures) | SoA (choix MGE) |
|--------------------------|-----------------|
| `Vec<Entity { pos, vel, health }>` | `positions: Vec<Vec2>`, `velocities: Vec<Vec2>` |
| Cache pollution : un système qui ne lit que Position charge quand même Vel, Health. | Itération sur un seul type : localité optimale, prédicteur de branchement favorable. |
| Moins efficace pour 10k+ entités. | Scaling linéaire, batch processing naturel. |

### 4.3 Archetypes

- **Définition :** Un archetype = ensemble de types de composants (ex. `CompA + CompB + CompC`).
- **Regroupement :** Les entités avec le même archetype sont stockées dans les mêmes tableaux SoA.
- **Migration :** Ajout/suppression de composant → changement d'archetype (déplacement des données).
- **Itération :** Parcourir un archetype = boucle séquentielle sur des slices contiguës ; pas de HashMap dans le hot path.

### 4.4 Queries minimalistes

**Pas de DSL.** Pas de `Query<(&Position, &mut Velocity)>` méta-framework. Pas de macros complexes.

**API v0.1 :**

```rust
for (a, b) in world.iter2::<CompA, CompB>() { ... }
for (a, b, c) in world.iter3::<CompA, CompB, CompC>() { ... }
for (a, b) in world.iter2_mut::<CompA, CompB>() { ... }  // si mutation
```

- Types résolus à la compilation (monomorphisation). Pas de dynamic dispatch.
- **Extensions futures possibles :** iter1, iter4, ou trait interne extensible — à évaluer selon besoins post-v0.1 pour éviter multiplication des variantes.
- Pas de `With<>`, `Without<>`, `Option<>` dans la Phase 1 ; extensions possibles plus tard si besoin, sans impacter le kernel.

### 4.5 Pas de logique métier dans les composants

Les composants implémentent `Component: Send + Sync + 'static`. Données pures : pas de méthodes métier, pas de callbacks. La logique est dans les **systèmes**.

### 4.6 API World (conceptuelle)

```rust
impl World {
    pub fn spawn(&mut self) -> EntityId;
    pub fn despawn(&mut self, id: EntityId);
    pub fn insert<T: Component>(&mut self, id: EntityId, component: T);
    pub fn get<T: Component>(&self, id: EntityId) -> Option<&T>;
    pub fn get_mut<T: Component>(&mut self, id: EntityId) -> Option<&mut T>;
    pub fn iter2<A, B>(&self) -> impl Iterator<Item = (EntityId, &A, &B)>;
    pub fn iter2_mut<A, B>(&mut self) -> impl Iterator<Item = (EntityId, &mut A, &mut B)>;
    pub fn iter3<A, B, C>(&self) -> impl Iterator<Item = (EntityId, &A, &B, &C)>;
}
```

---

## 5. Scheduler

### 5.1 Ordre déterministe strict

- Les systèmes sont regroupés en **phases** identifiées par `PhaseId(u32)`.
- Au sein d'une phase : ordre fixe (ordre d'ajout du plugin).
- Exécution séquentielle : **aucun parallélisme implicite**.

**Pourquoi :** Reproductibilité. Même seed + même ordre = même résultat. Le parallélisme introduit des non-déterminismes (ordre d'exécution, cache). Pour un kernel stable 10 ans, le déterminisme prime.

### 5.2 Phases génériques — PhaseId opaque

- **Le core ne connaît pas** Physics, Logic, Render, Input. Ce sont des conventions de jeu.
- Le core définit uniquement : `pub struct PhaseId(pub u32)`.
- Les **plugins** déclarent et enregistrent leurs phases (ex. `PhaseId(0)` = input, `PhaseId(1)` = simulation, `PhaseId(2)` = IA).
- Le scheduler exécute les phases par ordre croissant de PhaseId.

**Règles de stabilité (contrat déterministe) :**
- **PhaseId ne doit jamais changer de valeur entre versions majeures** — une mise à jour moteur ne réaffecte pas les numéros.
- Les plugins définissent leurs constantes (ex. `const PHASE_SIMULATION: PhaseId = PhaseId(1)`) et les conservent.
- L'ordre des phases fait partie du contrat déterministe : modifier l'ordre = changement de comportement reproductible.

**Pourquoi phases opaques :** Éviter que le kernel soit lié à un genre de jeu. Un kernel avec `Phase::Physics` hardcodé ne peut pas servir un jeu sans physique. PhaseId(u32) = contrat minimal, stable 10 ans.

### 5.3 Pas de phases gameplay codées en dur

**Interdit :** `Phase::Physics`, `Phase::Render`, etc. dans le kernel. Le rendu est **hors core** ; le Game Runtime appelle `render()` après `tick()`.

### 5.4 CPU budget hooks

- **Tick budget :** Temps max par tick (ex. 8 ms). Le core ne connaît pas le frame.
- **System budget (optionnel) :** Temps max par système.
- **Overflow :** Si dépassement → **détection et exposition des métriques uniquement**. Le kernel ne décide pas de la stratégie (Continue, Skip tick, LOD, Scale down) — cela introduirait de la logique métier dans le core. La réaction (warning, skip, réduction LOD) est du ressort du **plugin** ou du **Game Runtime**.
- Le budget frame (60 FPS) est du ressort du plugin rendu.

### 5.5 Schéma ASCII — Exécution d'un tick

```
Scheduler::run()
    │
    ├─ PhaseId(0)  →  system_0_0, system_0_1, ...  (ordre d'ajout)
    │       │
    │       └─ [Profiling hook avant/après chaque système]
    │
    ├─ PhaseId(1)  →  system_1_0, system_1_1, ...
    │
    ├─ PhaseId(2)  →  system_2_0, ...
    │
    └─ ... (phases par ordre croissant)
```

---

## 6. Event System

### 6.1 Pas de subscribe(callback)

**Interdit :** `engine.subscribe::<DamageEvent>(|e| { ... })` ou équivalent avec closures dynamiques.

**Raisons :**

| Problème | subscribe(callback) | Lecture explicite (choix MGE) |
|----------|---------------------|-------------------------------|
| **Dynamic dispatch** | Vec<Box<dyn Fn(Event)>>, indirection. | Itération directe, monomorphisation. |
| **Allocations** | Chaque subscribe = allocation. | Aucune allocation dans le hot path. |
| **Couplage caché** | Handlers dispersés, ordre implicite. | Systèmes lisent dans leur corps, ordre = ordre des phases. |
| **Profiling** | Difficile de mesurer un handler isolé. | Chaque système = unité de mesure claire. |
| **LLM** | Flux de données implicite, difficile à tracer. | Flux explicite : emit → iter ; un LLM peut suivre le graphe. |
| **Déterminisme** | Ordre des callbacks peut varier. | Ordre fixe = ordre des phases et des systèmes. |

### 6.2 Pas de handler stocké

Aucun enregistrement de closure ou de pointeur de fonction pour les événements. Les systèmes lisent **explicitement** :

```rust
fn handle_damage_system(world: &mut World, ctx: &mut Context) {
    for event in ctx.events().iter::<DamageEvent>() {
        // traitement
    }
}
```

### 6.3 Double buffer

- **Écriture** pendant le tick N : `ctx.emit(event)` → buffer A.
- **Lecture** au tick N+1 : `ctx.events().iter::<E>()` → buffer B (ancienne écriture).
- Au début du tick N+1 : swap des buffers (A ↔ B).

Évite les modifications pendant itération (undefined behavior, récursion).

### 6.4 Pas de récursion

Les événements émis pendant un tick sont lus au **tick suivant**. Pas de dispatch récursif (événement → handler → emit → handler → ...). Flux prévisible, stack bornée.

### 6.5 Robustesse LLM + déterminisme

Un LLM peut :
- Lire la spec et comprendre que les events sont en pull, pas push.
- Tracer : quel système émet `DamageEvent`, quels systèmes le lisent.
- Vérifier l'ordre : PhaseId(1) émet, PhaseId(2) lit.
- Pas de magie : pas de "quelque part un handler réagit" implicite.

---

## 7. RNG

### 7.1 Seed globale

- Fixée à l'initialisation : `Engine::new(config)` ou `engine.set_seed(u64)`.
- Une seule source de hasard au niveau Engine.
- Reproductibilité : même seed + même ordre d'exécution → même résultat.

### 7.2 Seed dérivé par entité

Pour les entités dont le RNG doit être isolé (loot, procédural par entité) :

```
entity_seed = global_seed ^ EntityId::to_bits()
```

Chaque entité a une séquence déterministe indépendante. Pas de prédiction croisée entre entités.

### 7.3 Interdiction de rand externe

- **Interdit dans la simulation :** `rand::random()`, `std::time::Instant::now()` pour la logique.
- Tout hasard passe par le RNG fourni par l'Engine (via Context dans les systèmes).
- Contrôle total = reproductibilité garantie.

### 7.4 Reproductibilité garantie

Contrat : `(seed, tick_count, ordre_systèmes)` → état du monde déterministe. Replay = même seed, mêmes inputs (si enregistrés), même nombre de ticks.

---

## 8. Gestion du temps

### 8.1 Delta time

- Temps écoulé **depuis le dernier tick** (en secondes).
- Concept **tick-based**, pas frame-based. Le core ignore le rendu.
- Utilisé pour : mouvements, timers, cooldowns.

### 8.2 Fixed timestep (optionnel)

- Mode où delta = constante (ex. 1/60 s).
- Utile : physique déterministe, lockstep réseau.
- **Accumulateur :** Si le temps réel dépasse, exécuter plusieurs ticks d'un coup (ou limiter pour éviter la spirale de mort).

### 8.3 Time scale

- Facteur multiplicatif : `effective_delta = delta * time_scale`.
- Ex. 0.5 = ralenti, 2.0 = accéléré.

### 8.4 Pause

- `paused = true` → delta = 0 pour la logique de simulation.
- Le temps « réel » continue (pour timers de pause menu, etc.).

### 8.5 Struct Time (conceptuelle)

```rust
/// Fournie via Context aux systèmes
pub struct Time {
    pub delta_secs: f32,
    pub tick_count: u64,
    pub time_scale: f32,
    pub paused: bool,
}
```

---

## 9. CPU Budget et Profiling

### 9.1 Hooks par système

- Avant chaque système : enregistrer timestamp.
- Après : calculer durée, stocker (si profiling activé).
- Métriques : temps par système, par phase, total tick.

### 9.2 Mesure par phase

- Agrégation des durées des systèmes d'une même phase.
- Détection des phases coûteuses (ex. PhaseId(1) = physics dépasse le budget).

### 9.3 Détection overflow

- Si `tick_budget_ms` dépassé : **détection** + **exposition des métriques** (durée réelle, dépassement). Le kernel ne choisit pas la réaction.
- Si `system_budget_ms` dépassé (optionnel) : même principe — détection et métriques.
- **Stratégie de réduction (Skip tick, LOD, Scale down) = plugin-level.** Le core ne contient aucune logique de décision ; il se contente de mesurer et d'exposer. Sinon on introduit du gameplay dans le kernel.

### 9.4 Pas d'allocation cachée

Le hot path (tick, systèmes) ne doit pas allouer. Pas de `Vec::new()` dans une boucle, pas de `format!` dans un système appelé 60 fois/s. Pools, arenas, pre-allocation pour les cas qui en ont besoin.

### 9.5 Feature flag

Le profiling peut être désactivé (feature flag ou config). Quand désactivé : hooks = no-op, coût nul.

---

## 10. Non-objectifs explicites

Le kernel **ne fournit pas** les éléments suivants. Chaque absence est un choix délibéré pour garder le kernel minimal et stable.

| Non-objectif | Justification |
|--------------|---------------|
| **Éditeur intégré** | Éditeurs = outils lourds, hors scope simulation. Jeux utilisent formats externes ou CLI. |
| **Scène graph** | Graphe parent-enfant = convention de rendu. Un plugin peut l'ajouter. |
| **Système asset** | Formats, chargement, pipeline = domaine jeu. Le kernel ne connaît pas les assets. |
| **Gameplay intégré** | Pas de combat, inventaire, quêtes, loot. Le core est agnostique du genre. |
| **Réseau** | Transport, snapshot, sync = plugin ou bridge. Le kernel = simulation locale pure. |
| **Rendu** | Fenêtre, GPU, sprites, caméra = plugin rendu. Headless = pas de rendu du tout. |
| **Audio** | Sons, musique = plugin. Stub si absent. |
| **Input** | Clavier, souris, manette = plugin. Le kernel reçoit des événements typés, pas des raw input. |

---

## 11. Design pour LLM

### 11.1 Pourquoi simple

- Pas de magie implicite : pas de macros qui génèrent 500 lignes, pas de typage dépendant opaque.
- Un LLM peut lire le code source et la spec et comprendre le flux sans inférence complexe.
- API lisible : `iter2`, `iter3`, `emit`, `events().iter::<E>()` — noms explicites.

### 11.2 Pourquoi explicite

- Pull model events : le flux de données est visible. `emit` → buffer → `iter` au tick suivant. Pas de "un handler quelque part".
- Pas de callbacks cachés : tout est dans les systèmes, dans l'ordre des phases.
- Un LLM peut tracer : "DamageEvent émis par system X, lu par system Y".

### 11.3 Facilement indexable via MSCM

Chaque module public a un bloc MSCM :

```
@id mge.core.engine
@role simulation
@layer core
@do orchestrate_lifecycle_and_tick
```

Le générateur MIP produit un index. Un LLM ou un outil peut résoudre `mge.core.scheduler` → fichier, lignes, responsabilité.

### 11.4 Stable dans ses signatures

- API minimale : peu de surfaces de changement.
- Pas de breaking change prévu : les signatures Engine, World, Scheduler, EventQueue sont figées pour la v0.1.
- Extensions = nouveaux types (plugins), pas modification du kernel.

### 11.5 Comment un LLM peut raisonner sur le kernel

1. Lire la spec : savoir ce que fait chaque composant.
2. Consulter l'index MIP : localiser le code.
3. Tracer les données : World (composants), EventQueue (events), Time, RNG.
4. Modifier : ajouter un système, un composant, un event — sans toucher au kernel.

---

## 12. Extensions prévues (hors kernel)

Ces éléments sont **en dehors** du kernel. Ils s'appuient sur le kernel mais ne font pas partie de mge-core.

| Extension | Emplacement | Pourquoi hors kernel |
|-----------|-------------|----------------------|
| **Plugins** (physics, ai, input, audio, network) | mge-plugin-* | Logique métier, dépendances optionnelles. |
| **Rendu** | mge-render | GPU, fenêtre, backend ; headless = pas de rendu. |
| **COG Bridge** | mge-cog-bridge | IPC, Central, Cores ; optionnel pour standalone. |
| **Game layer** | apps/allumina, etc. | Règles de jeu, contenu ; consommateur du MGE. |

Le kernel reste inchangé qu'il y ait 0 ou 10 plugins. Aucun refactor du kernel pour cause de couplage futur.

---

## Annexes

### A. API minimale complète (Rust conceptuel)

```rust
// === Engine ===
struct Engine {
    world: World,
    scheduler: Scheduler,
    events: EventQueue,
    time: Time,
    rng: Rng,
}

impl Engine {
    pub fn new(config: EngineConfig) -> Self;
    pub fn add_plugin<P: Plugin>(&mut self, plugin: P);
    pub fn build(&mut self);
    pub fn tick(&mut self);

    pub fn world(&self) -> &World;
    pub fn world_mut(&mut self) -> &mut World;
    pub fn events(&self) -> &EventQueue;
    pub fn emit<E: Event>(&mut self, event: E);
    pub fn set_seed(&mut self, seed: u64);
}

// === World ===
impl World {
    pub fn spawn(&mut self) -> EntityId;
    pub fn despawn(&mut self, id: EntityId);
    pub fn insert<T: Component>(&mut self, id: EntityId, component: T);
    pub fn get<T: Component>(&self, id: EntityId) -> Option<&T>;
    pub fn iter2<A, B>(&self) -> impl Iterator<Item = (EntityId, &A, &B)>;
    pub fn iter2_mut<A, B>(&mut self) -> impl Iterator<Item = (EntityId, &mut A, &mut B)>;
    pub fn iter3<A, B, C>(&self) -> impl Iterator<Item = (EntityId, &A, &B, &C)>;
}

// === Scheduler ===
pub struct PhaseId(pub u32);

impl Scheduler {
    pub fn add_system<F>(&mut self, phase: PhaseId, system: F)
    where F: FnMut(&mut World, &mut Context);
}

// === Context (passé aux systèmes) ===
pub struct Context<'a> {
    pub time: &'a Time,
    pub rng: &'a mut Rng,
    pub events: &'a EventQueue,
    // emit via méthode, pas dyn FnMut — évite dynamic dispatch, alloc, inlining
}

impl Context<'_> {
    pub fn emit<E: Event>(&mut self, event: E);
}
```

**Pas de run(). Pas de subscribe(). Pas de Query<Q>.** Pas de `dyn FnMut` dans Context.

### B. Références

| Document | Rôle |
|----------|------|
| [MGE - Document Fondateur](../Miyukini_Game_Engine/MGE%20-%20Document%20Fondateur.md) | Vision, philosophie, 6 piliers. |
| [MGE - Architecture Générale](../Miyukini_Game_Engine/MGE%20-%20Architecture%20Generale.md) | Couches, diagrammes, flux. |
| [MGE - Core Specification Technique](../Miyukini_Game_Engine/MGE%20-%20Core%20Specification%20Technique.md) | Spec technique détaillée (source consolidée). |
| [MGE - Plugin Contract](../Miyukini_Game_Engine/MGE%20-%20Plugin%20Contract.md) | Trait Plugin, enregistrement. |
| [MGE - Performance Philosophy](../Miyukini_Game_Engine/MGE%20-%20Performance%20Philosophy.md) | SoA, cache locality, profiling. |
| [MGE - Simulation Scaling](../Miyukini_Game_Engine/MGE%20-%20Simulation%20Scaling.md) | LOD, budget CPU, overflow. |
| [MGE - MSCM MIP Governance](../Miyukini_Game_Engine/MGE%20-%20MSCM%20MIP%20Governance.md) | Balisage, politique ID. |

### C. IDs MSCM prévus pour mge-core

| Module | @id |
|--------|-----|
| Engine | mge.core.engine |
| World | mge.core.world |
| Scheduler | mge.core.scheduler |
| EventQueue | mge.core.event_queue |
| RNG | mge.core.rng |
| Time | mge.core.time |

---

**Document :** MGE — Kernel Specification  
**Version :** 1.0  
**Date :** 2026-02-19  
**Statut :** Spécification normative
