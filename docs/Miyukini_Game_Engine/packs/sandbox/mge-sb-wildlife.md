# mge-sb-wildlife

> @id mge.sandbox.wildlife.v1  
> @role plugin  
> @domain sandbox  
> @do manage_fauna_spawning_behaviors_migration  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-sb-wildlife` |
| @id MSCM | `mge.sandbox.wildlife.v1` |
| Domaine | sandbox |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-sb-world`, `mge-plugin-spatial` |
| Hot path | Oui (update_wildlife_behavior chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(w) par tick, w = animaux actifs |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `WildlifeType` | `Passive, Neutral, Hostile` | Comportement envers le joueur |
| `AnimalBehavior` | `Wandering, Feeding, Fleeing, Sleeping, Migrating` | Etat comportemental courant |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Wildlife` | `mge.sandbox.wildlife.v1.component.wildlife` | `species_id: u32, wildlife_type: WildlifeType, behavior: AnimalBehavior` | Animal. species_id pointe vers une definition statique |
| `SpawnZone` | `mge.sandbox.wildlife.v1.component.spawn_zone` | `x: f32, z: f32, radius: f32, species: Vec<u32>, max_population: u32` | Zone de spawn. Limite la population par zone |
| `MigrationPath` | `mge.sandbox.wildlife.v1.component.migration_path` | `waypoints: Vec<(f32, f32)>, current_waypoint: u32` | Chemin de migration saisonnier |
| `HerdMember` | `mge.sandbox.wildlife.v1.component.herd_member` | `herd_id: u32, is_leader: bool` | Appartenance a un troupeau. Le leader guide le deplacement |

---

## 4. Formules

```
spawn_probability = base_spawn_rate * (1.0 - current_population / max_population)
spawn_probability = 0 si current_population >= max_population

flee_trigger = distance_to_threat < flee_radius
flee_direction = normalize(animal_pos - threat_pos)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `spawn_wildlife` | `mge.sandbox.wildlife.v1.fn.spawn_wildlife` | 1545 | SpawnZone, Chunk | Wildlife | WildlifeSpawned | O(z) | Spawn animaux dans les zones actives (chunks charges). Respecte max_population |
| `update_wildlife_behavior` | `mge.sandbox.wildlife.v1.fn.update_wildlife_behavior` | 1546 | Wildlife, Position | Wildlife | none | O(w) | Evalue le comportement : Wandering par defaut, Fleeing si menace, Feeding si besoin |
| `process_migration` | `mge.sandbox.wildlife.v1.fn.process_migration` | 1547 | Wildlife, MigrationPath, SeasonChanged (event) | MigrationPath, Wildlife | MigrationStarted | O(m) | Declenche migration au changement de saison. Avance les waypoints |
| `cull_wildlife` | `mge.sandbox.wildlife.v1.fn.cull_wildlife` | 1548 | Wildlife, Chunk | Wildlife | WildlifeDespawned | O(w) | Supprime les animaux dans les chunks decharges ou surpeuples |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `WildlifeSpawned` | `mge.sandbox.wildlife.v1.event.wildlife_spawned` | `entity: EntityId, species_id: u32, x: f32, z: f32` | `spawn_wildlife` | ui, agent (reaction) |
| `WildlifeDespawned` | `mge.sandbox.wildlife.v1.event.wildlife_despawned` | `entity: EntityId, species_id: u32` | `cull_wildlife` | ui |
| `MigrationStarted` | `mge.sandbox.wildlife.v1.event.migration_started` | `herd_id: u32, destination: (f32, f32)` | `process_migration` | ui, agent |
| `HerdFormed` | `mge.sandbox.wildlife.v1.event.herd_formed` | `herd_id: u32, leader: EntityId, members: u32` | `spawn_wildlife` | ui |

---

## 7. Invariants

- `SpawnZone` ne spawn que dans les chunks `Loaded`.
- La population d'une zone ne depasse jamais `max_population`.
- Un animal `Hostile` ne fuit jamais (toujours attaque ou Wandering).
- `MigrationPath.current_waypoint` est dans [0, waypoints.len()).
- Le leader d'un troupeau est toujours un animal vivant dans le troupeau.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `base_spawn_rate` | `f32` | 0.01 | [0.001, 0.1] | Probabilite de spawn/tick par zone |
| `flee_radius` | `f32` | 10.0 | [3.0, 50.0] | Distance de declenchement fuite |
| `wander_radius` | `f32` | 15.0 | [5.0, 50.0] | Rayon de vagabondage |
| `migration_speed` | `f32` | 2.0 | [0.5, 10.0] | Vitesse de deplacement en migration |
| `max_herds_per_zone` | `u32` | 3 | [1, 10] | Nombre max de troupeaux par zone |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Spawn et despawn la faune | Ne gere pas le combat animal (→ rpg-combat) |
| Simule les comportements (wander, flee, feed) | Ne modifie pas le terrain (→ terrain) |
| Gere la migration saisonniere | Ne gere pas les saisons (→ season) |
| Organise les troupeaux | Ne gere pas le pathfinding detaille (→ core spatial) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Wildlife, SpawnZone, MigrationPath, HerdMember, Chunk, Position, SeasonChanged |
| Ecrit | Wildlife, MigrationPath, HerdMember |
| Emet | WildlifeSpawned, WildlifeDespawned, MigrationStarted, HerdFormed |
| Ne touche jamais | TerrainTile, Building, CraftingStation, Need, Agent, Weather |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-sb-wildlife/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.sandbox.wildlife.v1
    ├── components.rs     # Wildlife, SpawnZone, MigrationPath, HerdMember
    ├── systems.rs        # spawn_wildlife, update_wildlife_behavior, process_migration, cull_wildlife
    └── events.rs         # WildlifeSpawned, WildlifeDespawned, MigrationStarted, HerdFormed
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (update_wildlife_behavior) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (WildlifeType, AnimalBehavior)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : spawn limits, behavior transition, migration, herd formation, cull
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.sandbox.wildlife.v1","k":"p","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.wildlife.v1.component.wildlife","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.wildlife.v1.component.spawn_zone","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.wildlife.v1.component.migration_path","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.wildlife.v1.component.herd_member","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.wildlife.v1.fn.spawn_wildlife","k":"s","d":"sandbox","r":["SpawnZone","Chunk"],"w":["Wildlife"],"e":["WildlifeSpawned"],"p":1545,"c":"O(z)"},
  {"i":"mge.sandbox.wildlife.v1.fn.update_wildlife_behavior","k":"s","d":"sandbox","r":["Wildlife","Position"],"w":["Wildlife"],"e":[],"p":1546,"c":"O(w)"},
  {"i":"mge.sandbox.wildlife.v1.fn.process_migration","k":"s","d":"sandbox","r":["Wildlife","MigrationPath"],"w":["MigrationPath","Wildlife"],"e":["MigrationStarted"],"p":1547,"c":"O(m)"},
  {"i":"mge.sandbox.wildlife.v1.fn.cull_wildlife","k":"s","d":"sandbox","r":["Wildlife","Chunk"],"w":["Wildlife"],"e":["WildlifeDespawned"],"p":1548,"c":"O(w)"},
  {"i":"mge.sandbox.wildlife.v1.event.wildlife_spawned","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.wildlife.v1.event.wildlife_despawned","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.wildlife.v1.event.migration_started","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.wildlife.v1.event.herd_formed","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let zone = world.spawn();
world.insert(zone, SpawnZone { x: 100.0, z: 200.0, radius: 30.0, species: vec![1, 2], max_population: 10 });

let deer = world.spawn();
world.insert(deer, Wildlife { species_id: 1, wildlife_type: WildlifeType::Passive, behavior: AnimalBehavior::Wandering });
world.insert(deer, HerdMember { herd_id: 1, is_leader: false });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Sandbox - Index](_index.md) | Vue d'ensemble du pack |
| [mge-sb-world](mge-sb-world.md) | Plugin world (dependance) |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
