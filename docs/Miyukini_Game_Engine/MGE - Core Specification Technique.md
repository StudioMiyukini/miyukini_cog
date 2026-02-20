# MGE — Core Specification Technique

Spécification technique du microkernel mge-core : Engine, World, Scheduler, EventQueue, RNG et gestion du temps. Le core expose **uniquement** `tick()` — pas de `run()`, pas de phases gameplay, pas de callbacks.

## Contexte

mge-core est le cœur minimal du MGE. Il ne contient aucune logique de physique, rendu, audio ou réseau. Toute fonctionnalité au-delà est fournie par des plugins.

## Garanties généraliste et spatial-agnostic

Le core **ne suppose aucun gameplay** ni aucune structure figée :

- **Aucun composant obligatoire** — pas de Position, Transform ou équivalent imposé. Les plugins déclarent ce dont ils ont besoin.
- **Aucune grille 2D** — pas de tuiles, chunks ou espace prédéfini. Un plugin spatial (2D ou 3D) introduit la dimension.
- **Aucune notion de combat** — pas de Health, Damage ou armes au core.
- **Aucun tick rate imposé** — pas de 60 ticks/s obligatoire. Le débit des ticks est configurable.
- **Spatial-agnostic** — le core ignore 2D/3D ; la sémantique spatiale est dans un plugin.

## Portée / Scope

- **Applicable à :** Implémentation de mge-core, développement des plugins.
- **Audience :** Développeurs moteur, architectes.
- **Statut :** Spécification normative.

---

## 1. Engine

### 1.1 Principe : le core ne connaît pas la boucle

Le core **ne fournit pas** `run()`. Il fournit **uniquement** `tick()`. La boucle (input → tick → render) est orchestrée par le **Game Runtime**, pas par le core. Sinon le core devient implicitement rendering-aware.

### 1.2 Cycle de vie

```
Engine::new(config)  →  add_plugin(...)  →  build()
```

| Phase | Description |
|-------|-------------|
| **new** | Création avec config (seed, headless, tick_budget, etc.). Pas de résolution. |
| **add_plugin** | Enregistrement des plugins avant build. |
| **build** | Résolution des dépendances, enregistrement composants/systèmes/phases, initialisation. |

**Pas de run().** Le Game Runtime fait :

```rust
loop {
    input();        // plugin input (hors core)
    engine.tick();  // simulation pure
    render();       // plugin render (hors core)
}
```

### 1.3 Tick

`pub fn tick(&mut self)` — seule méthode d'avancement fournie par le core.

Un tick = une itération de simulation. Chaque tick :

1. Calcul du delta time (ou fixed timestep).
2. Préparation des événements pour lecture (buffer swap dans EventQueue).
3. Exécution des systèmes (Scheduler) dans l'ordre déterministe.
4. Incrément du tick counter.

**Contrat :** L'ordre des ticks est fixe. Le contenu d'un tick ne dépend que de l'état avant le tick et du delta time. **Aucune notion de frame ni de rendu.**

### 1.4 Seed RNG

- **Seed globale** : fixée à l'initialisation (`Engine::new` ou `engine.set_seed(u64)`).
- **Reproductibilité** : même seed + même ordre d'exécution → même résultat.
- Le seed est stocké dans l'Engine et propagé aux systèmes qui en ont besoin.

### 1.5 Configuration

```rust
/// Exemple de structure de configuration (conceptuel)
/// Note : le core ne connaît que le tick, pas la frame (rendu)
pub struct EngineConfig {
    pub seed: u64,
    pub headless: bool,
    pub fixed_timestep_ms: Option<u32>,  // Optionnel ; si None, delta variable
    pub tick_budget_ms: Option<u32>,    // Budget CPU par tick de simulation
    // frame_budget et résolution = dans le plugin rendu, pas le core
}
```

---

## 2. World

### 2.1 Stockage des entités

- **EntityId** : identifiant unique opaque (génération + index).
- **Entités** : identifiants + ensemble de composants attachés.
- Stockage SoA (Structure of Arrays) pour la localité de cache : chaque type de composant est un tableau contigu.

### 2.2 Composants

- Types `T` qui implémentent `Component` (marqueur + constraints).
- Enregistrement : `engine.register_component::<T>()` lors du build.
- Un composant est une donnée pure : pas de logique, pas de méthodes métier complexes.
- Stockage par archetype : entités avec le même ensemble de composants partagent la même « colonne ».

### 2.3 Archetypes

- **Archetype** = ensemble de types de composants (ex. `CompA + CompB + CompC` — les noms sont définis par les plugins).
- Les entités sont groupées par archetype pour itération efficace.
- Ajout/suppression de composant → changement d'archetype (migration).

### 2.4 Itération — API minimale (pas de DSL)

**Principe :** Pas de Query méta-framework. Pas de macro géante. Pas de typage ultra-complexe. Microkernel = minimal.

- **Phase 1** : itération simple par paires/triplets de composants :
  ```rust
  for (a, b) in world.iter2::<CompA, CompB>() { ... }
  for (a, b, c) in world.iter3::<CompA, CompB, CompC>() { ... }
  ```
- Les types sont résolus à la compilation (monomorphisation). Pas de dynamic dispatch.
- Pas de `Query` trait complexe ; juste des méthodes `iter2`, `iter3` (ou équivalent minimal).

---

## 3. Scheduler

### 3.1 Ordre déterministe

- Les systèmes sont regroupés en **phases** identifiées par `PhaseId(u32)`.
- Au sein d'une phase, l'ordre est fixe (ordre d'ajout du plugin).
- Aucun parallélisme implicite : exécution séquentielle pour garantir le déterminisme.

### 3.2 Phases : le core n'impose rien

**Le core ne connaît pas** Physics, Logic, Render, etc. Ce sont des conventions de jeu.

- Le core définit uniquement : `pub struct PhaseId(pub u32)`.
- Les **plugins** déclarent et enregistrent leurs phases (ex. `PhaseId(0)` = input, `PhaseId(1)` = simulation).
- Exemple : un plugin enregistre `PhaseId(1)` pour ses systèmes de déplacement ; un autre `PhaseId(2)` pour l'IA. Le core exécute dans l'ordre des PhaseId.
- **Aucune phase « Render » hardcodée** — le rendu est hors core, orchestré par le Game Runtime.

### 3.3 Budget CPU (tick, pas frame)

- **Tick budget** : temps max par tick de simulation (ex. 8 ms). Le core ne connaît pas la notion de « frame » (rendu).
- **System budget** : optionnel, temps max par système.
- **Overflow** : si dépassement, log warning ; comportement défini (skip ou continuer).
- Le budget frame (60 FPS, etc.) est du ressort du plugin rendu, pas du core.

### 3.4 Profiling hooks

- Appel avant/après chaque système avec identifiant et durée.
- Métriques exposées : temps par système, par phase, total tick.
- Utilisable pour détection de goulots d'étranglement.

---

## 4. EventQueue — pas de subscribe/callback

### 4.1 Principe : lecture explicite, pas de callbacks

**Interdit** : `subscribe::<E>(handler)` avec closures dynamiques. Cela introduit dynamic dispatch, allocations, couplage caché, difficulté de profiling.

**Approche** : les systèmes lisent les événements **explicitement** :

```rust
for event in engine.events().iter::<DamageEvent>() {
    // traitement
}
```

- Pas de callback. Pas de handler stocké. Les systèmes itèrent sur la file.
- Évite le pattern « moteur événementiel type Unity ».

### 4.2 Rôle

- **EventQueue** : file d'événements typés, buffer double.
- `engine.emit(event)` : écriture dans le buffer (pendant le tick).
- `engine.events().iter::<E>()` : lecture par les systèmes (au tick suivant, après swap des buffers).

### 4.3 Buffer double

- Écriture pendant le tick N ; lecture au tick N+1.
- Évite les modifications pendant itération.

---

## 5. RNG déterministe

### 5.1 Seed globale

- Une seule source de hasard au niveau Engine.
- Seed fixée au démarrage ; reproductible.

### 5.2 Seed par entité (optionnel)

- Pour les entités dont le RNG doit être isolé (ex. loot, comportement procédural).
- Chaque entité peut avoir un `RngHandle` dérivé du seed global + EntityId.

### 5.3 Reprodutibilité totale

- Aucun appel à `rand::random()` ou `std::time` dans la logique de simulation.
- Tout hasard passe par l'objet RNG fourni par l'Engine.

---

## 6. Gestion du temps

### 6.1 Delta time

- Temps écoulé **depuis le dernier tick** (en secondes). Concept tick-based, pas frame-based.
- Utilisé pour mouvements, timers, cooldowns.

### 6.2 Fixed timestep (optionnel)

- Mode où delta = constante (ex. 1/60 s).
- Utile pour physique déterministe et lockstep.
- Accumulateur pour rattraper le temps réel si nécessaire.

### 6.3 Time scale

- Facteur multiplicatif : `effective_delta = delta * time_scale`.
- Ex. 0.5 = ralenti, 2.0 = accéléré.

### 6.4 Pause

- `paused = true` → delta = 0 pour la logique de jeu.
- L'UI (pause menu) peut rester active.
- Le temps « réel » continue pour les timers de pause.

---

## 7. Isolation des plugins

- Les plugins **ne doivent pas** importer ou appeler directement d'autres plugins.
- Communication **uniquement** via : World (lecture/écriture de composants) et EventQueue (lecture explicite des événements).
- Les dépendances déclaratives (ordre de build) ne servent qu'au chargement ; pas à des appels croisés.

---

## 8. API minimale — la version microkernel pure

Le core expose **seulement** :

```rust
/// Structure minimale
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

    /// Seule méthode d'avancement — simulation pure, pas de rendu
    pub fn tick(&mut self);

    pub fn world(&self) -> &World;
    pub fn world_mut(&mut self) -> &mut World;
    pub fn events(&self) -> &EventQueue;
    pub fn emit<E: Event>(&mut self, event: E);
    pub fn set_seed(&mut self, seed: u64);
}

/// World — itération simple, pas de Query méta-framework
impl World {
    pub fn spawn(&mut self) -> EntityId;
    pub fn despawn(&mut self, id: EntityId);
    pub fn insert<T: Component>(&mut self, id: EntityId, component: T);
    pub fn get<T: Component>(&self, id: EntityId) -> Option<&T>;
    pub fn iter2<A, B>(&self) -> impl Iterator<Item = (&A, &B)>;
    pub fn iter2_mut<A, B>(&mut self) -> impl Iterator<Item = (&mut A, &mut B)>;
    pub fn iter3<A, B, C>(&self) -> impl Iterator<Item = (&A, &B, &C)>;
}

/// Scheduler — PhaseId, pas de phases gameplay hardcodées
impl Scheduler {
    pub fn add_system<F>(&mut self, phase: PhaseId, system: F)
    where F: FnMut(&World, &mut Context);
}
```

**Pas de run(). Pas de subscribe(). Pas de Query<Q>.**

---

## 9. Références

| Document | Rôle |
|----------|------|
| [MGE - Architecture Générale](./MGE%20-%20Architecture%20Generale.md) | Vue d'ensemble des couches. |
| [MGE - Plugin Contract](./MGE%20-%20Plugin%20Contract.md) | Enregistrement composants, systèmes, événements. |
| [MGE - Performance Philosophy](./MGE%20-%20Performance%20Philosophy.md) | SoA, batch, cache locality. |
| [MGE - Référence Commune](./reference/MGE%20-%20Reference%20Commune.md) | Types Vec2, Rect, etc. |

---

**Document** : MGE — Core Specification Technique  
**Version** : 1.0  
**Date** : 2026-02-19  
**Statut** : Spécification normative
