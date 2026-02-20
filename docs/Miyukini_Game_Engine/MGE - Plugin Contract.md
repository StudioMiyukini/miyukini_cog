# MGE — Plugin Contract

Contrat du trait Plugin : déclaration de composants, enregistrement de systèmes, écoute et publication d'événements.

## Contexte

Le MGE est extensible via des plugins. Chaque plugin implémente le trait `Plugin` et enregistre ses capacités dans l'Engine lors du build. **Isolation stricte** : les plugins communiquent uniquement via World et EventQueue, jamais par imports ou appels directs.

## Portée / Scope

- **Applicable à :** Développement de plugins MGE (officiels ou tiers).
- **Audience :** Développeurs moteur, développeurs tiers.
- **Statut :** Spécification normative.

---

## 1. Trait Plugin

### 1.1 Signature

```rust
/// Contrat Plugin MGE (conceptuel)
pub trait Plugin {
    /// Nom unique du plugin (pour logs, debugging, dépendances)
    fn name(&self) -> &str;

    /// Construit le plugin : enregistre composants, systèmes, abonnements
    fn build(&self, engine: &mut Engine);

    /// Dépendances optionnelles : noms des plugins construits AVANT celui-ci.
    /// Servent uniquement à l'ordre de build — pas à des appels directs entre plugins.
    fn dependencies(&self) -> &[&str] {
        &[]
    }
}
```

### 1.2 Invocation

- `engine.add_plugin(plugin)` enregistre le plugin.
- `engine.build()` invoque `plugin.build(engine)` pour chaque plugin, dans l'ordre résolu des dépendances.
- Les dépendances ne créent pas de couplage : un plugin ne doit jamais importer un autre plugin. La communication se fait via World et EventQueue.

---

## 2. Déclaration de composants

### 2.1 Enregistrement

```rust
// Exemple : un plugin spatial 2D enregistre ses composants (noms définis par le plugin)
engine.register_component::<Spatial2D>();
engine.register_component::<Velocity2D>();
engine.register_component::<SpriteHandle>();
```

- Chaque composant `T` doit implémenter `Component` (marqueur + `Send + Sync`).
- L'enregistrement alloue le stockage SoA pour ce type dans le World.
- Les composants non enregistrés ne peuvent pas être insérés.

### 2.2 Contraintes Component

```rust
/// Marqueur requis pour tout composant
pub trait Component: Send + Sync + 'static {}
```

- Pas de logique dans les composants ; données pures.
- `Clone` recommandé pour le snapshot (multijoueur).

---

## 3. Enregistrement de systèmes

### 3.1 Ajout d'un système

```rust
// PhaseId déclaré par le plugin (le core n'impose pas Physics, Logic, Render)
const PHASE_SIMULATION: PhaseId = PhaseId(1);
engine.add_system(PHASE_SIMULATION, movement_system);
```

- **PhaseId** : identifiant numérique ; le core ne connaît pas la sémantique. Les plugins enregistrent leurs phases.
- **System** : fonction ou closure `fn(world: &World, context: &mut Context)`.
- Les systèmes d'une même phase s'exécutent dans l'ordre d'ajout.

### 3.2 Contexte

Le `Context` fournit :
- Accès au RNG (pour déterminisme).
- Accès au Time (delta, tick).
- Possibilité d'émettre des événements.
- Ressources partagées (optionnel).

### 3.3 Signature système typique

```rust
fn movement_system(world: &World, ctx: &mut Context) {
    let dt = ctx.delta_time();
    for (pos, vel) in world.iter2::<Position, Velocity>() {
        // Itération simple ; pas de Query méta-framework
    }
}
```

- Les systèmes reçoivent `&World` ou `&mut World` selon les besoins.
- Itération via `iter2`, `iter3` — API minimale, pas de DSL.

---

## 4. Lecture des événements — pas de subscribe

**Interdit** : `subscribe::<E>(handler)` — introduit dynamic dispatch, allocations, couplage caché.

**Approche** : les systèmes lisent les événements **explicitement** dans leur corps :

```rust
fn handle_damage_system(world: &mut World, ctx: &mut Context) {
    for event in ctx.events().iter::<DamageEvent>() {
        // Traitement explicite
    }
}
```

- Pas de callback. Lecture par itération sur `events.iter::<E>()`.
- Les événements sont consommés au tick N+1 (buffer double).

---

## 5. Publication d'événements

### 5.1 Émission

```rust
engine.emit(CollisionEvent { a: entity_a, b: entity_b });
```

- Les événements sont mis en buffer (EventQueue) ; les systèmes les lisent au tick suivant via `events.iter::<E>()`.
- Buffer double : écriture pendant le tick, lecture au tick suivant.

### 5.2 Événements typés

- Chaque type d'événement `E` doit implémenter `Event` (marqueur).
- Pas de limite sur la taille ou la complexité ; préférer des événements petits et ciblés.

---

## 6. Dépendances entre plugins

### 6.1 Rôle : ordre de build uniquement

Les `dependencies()` définissent **quel plugin est construit avant quel autre**. Cela ne signifie pas que le plugin importe ou appelle l'autre.

```rust
impl Plugin for RenderPlugin {
    fn dependencies(&self) -> &[&str] {
        &["mge-plugin-input", "mge-plugin-spatial"]  // Construits avant RenderPlugin
    }
    // ...
}
```

- Les plugins listés doivent être ajoutés avant celui-ci.
- L'Engine vérifie à build que les dépendances sont satisfaites.
- **Interdit** : `use mge_plugin_physics::*` dans RenderPlugin. La communication se fait via World (composants) et EventQueue (lecture explicite).

### 6.2 Ordre de build

- Résolution topologique des dépendances.
- Si cycle détecté : erreur à build.

---

## 7. Exemple complet

```rust
/// Plugin physique minimal
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn name(&self) -> &str {
        "mge-plugin-physics"
    }

    fn build(&self, engine: &mut Engine) {
        engine.register_component::<Position>();
        engine.register_component::<Velocity>();
        engine.register_component::<Collider>();

        const PHASE_PHYSICS: PhaseId = PhaseId(1);  // Plugin déclare sa phase
        engine.add_system(PHASE_PHYSICS, resolve_collisions_system);
        engine.add_system(PHASE_PHYSICS, apply_velocity_system);
    }
}

fn resolve_collisions_system(world: &World, ctx: &mut Context) {
    // Lit les événements explicitement si besoin
    for _ in ctx.events().iter::<CollisionEvent>() { /* ... */ }
}

fn apply_velocity_system(world: &mut World, ctx: &mut Context) {
    let dt = ctx.delta_time();
    for (pos, vel) in world.iter2_mut::<Position, Velocity>() {
        pos.x += vel.dx * dt;
        pos.y += vel.dy * dt;
    }
}
```

---

## 8. Conventions

| Convention | Description |
|------------|-------------|
| **Nom plugin** | `mge-plugin-{nom}` pour les officiels ; préfixe libre pour les tiers. |
| **PhaseId** | Chaque plugin déclare ses phases (PhaseId(u32)) ; pas de phases hardcodées au core. |
| **Événements** | Nommer en `XxxEvent` ou `XxxRequest`. |
| **Composants** | PascalCase, pas de préfixe obligatoire. |
| **Systèmes** | `snake_case`, suffixe `_system` optionnel. |

---

## 9. Références

| Document | Rôle |
|----------|------|
| [MGE - Core Specification Technique](./MGE%20-%20Core%20Specification%20Technique.md) | Engine, World, Scheduler, EventQueue. |
| [MGE - Architecture Générale](./MGE%20-%20Architecture%20Generale.md) | Couches, flux. |
| [MGE - Simulation Scaling](./MGE%20-%20Simulation%20Scaling.md) | Budget CPU, LOD. |

---

**Document** : MGE — Plugin Contract  
**Version** : 1.0  
**Date** : 2026-02-19  
**Statut** : Spécification normative
