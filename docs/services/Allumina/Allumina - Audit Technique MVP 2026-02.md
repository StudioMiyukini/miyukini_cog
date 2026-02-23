# Allumina — Audit Technique MVP

**Date :** 2026-02-23
**Auditeur :** Architecte moteur
**Scope :** `mge/` (kernel, plugins, RPG pack, allumina_prototype)
**Référence :** `Allumina - MVP Sandbox.md`
**Verdict global : PASSABLE AVEC RÉSERVES CRITIQUES**

---

## RÉSUMÉ EXÉCUTIF

Le kernel MGE est solide : zéro unsafe, API propre, 21 tests. Le prototype Allumina est fonctionnel pour la démonstration (fenêtre wgpu, rendu isométrique, pathfinding A*, spawner).

Cependant, l'audit révèle **7 non-conformités critiques** avec le document MVP, **3 violations architecturales** majeures, un **index MSCM désynchronisé**, et une **dette technique structurelle** qui compromet la phase 2.

---

# I — CONFORMITÉ AVEC LE DOCUMENT MVP

## 1.1 Matrice de couverture système par système

| # | Système MVP | Doc | Code | Câblé | Testé | Verdict |
|---|-------------|-----|------|-------|-------|---------|
| 1 | Game loop ECS (30 TPS) | Phase 10-950 | `Engine::tick()`, `Scheduler` | Oui | 21 tests kernel | **CONFORME** |
| 2 | Carte fixe (tilemap) | TileMap, Tile, TileFlags | `tilemap.rs` (260 lig.) | Oui | Non | **CONFORME** |
| 3 | Mouvement + A* | Position, Velocity, PathState | `pathfinding.rs` + `movement.rs` + `pathfinding_system.rs` | Oui | Non | **PARTIEL** — pas de Collision |
| 4 | Combat mêlée | CombatStats, Health, Armor | `mge-rpg-combat` (105 lig. systems) | Non câblé | 1 test | **NON CÂBLÉ** |
| 5 | IA monstres (FSM) | AIState, 4 états | `mge-rpg-ai` (210 lig. systems) | Non câblé | 1 test | **NON CÂBLÉ** |
| 6 | Spawn monstres | SpawnerData | `spawner.rs` (84 lig.) | Oui | Non | **PARTIEL** — spawns sans stats ni IA |
| 7 | Troupe basique | TroopOwner, TroopFollower | `troops.rs` (79 lig.) | Oui | Non | **CONFORME** |
| 8 | Stats personnage | StatBlock, DerivedStats, Health | `mge-rpg-stats` components | Non câblé | Non | **NON CÂBLÉ** |
| 9 | Compétences (gain usage) | SkillSet, SkillCheckEvent | `mge-rpg-progression` (131 lig.) | Non câblé | 1 test | **NON CÂBLÉ** |
| 10 | Loot (drop monstres) | — | — | — | — | **ABSENT** |
| 11 | Inventaire + équipement | Inventory, Equipment, Item | `mge-rpg-inventory` components | Non câblé, pas de system | Non | **PARTIEL** — composants sans logique |
| 12 | Récolte | HarvestNode | `harvest.rs` (253 lig.) | Non câblé | Non | **NON CÂBLÉ** |
| 13 | Craft (recettes directes) | CraftStation, CraftRecipe | `craft.rs` (404 lig.) | Non câblé | Non | **NON CÂBLÉ** |
| 14 | Or + NPC marchands | Gold, Merchant | `economy.rs` (489 lig.) | Non câblé | Non | **NON CÂBLÉ** |
| 15 | Trade joueur-joueur | — | — | — | — | **ABSENT** |
| 16 | Réseau (Lobby MWS) | Protocole Allumina | — | — | — | **ABSENT** |
| 17 | Persistence (snapshot) | KindMother / fichier | — | — | — | **ABSENT** |
| 18 | Solo (sauvegarde locale) | LOI-1, LOI-2 | — | — | — | **ABSENT** |

### Synthèse

| Statut | Nombre | Systèmes |
|--------|--------|----------|
| **CONFORME** | 3 | Game loop, Carte, Troupe |
| **PARTIEL** | 3 | Mouvement, Spawn, Inventaire |
| **CODE EXISTE MAIS NON CÂBLÉ** | 6 | Combat, IA, Stats, Skills, Récolte, Craft, Économie |
| **ABSENT** | 4 | Loot, Trade P2P, Réseau, Persistence, Solo |

**Taux de couverture MVP : 17% (3/18 conformes)**

## 1.2 Détail des non-conformités critiques

### NC-1 : Systèmes implémentés mais non câblés dans le plugin

Le `AlluminaPlugin` enregistre uniquement 5 systèmes :

```17:47:mge/examples/allumina_prototype/src/plugin.rs
pub const PHASE_INPUT: PhaseId = PhaseId(50);
pub const PHASE_MOVEMENT: PhaseId = PhaseId(100);
pub const PHASE_SPAWN: PhaseId = PhaseId(600);
// ...
engine.add_named_system(PHASE_INPUT, "input_processing", input_processing_system);
engine.add_named_system(PHASE_MOVEMENT, "pathfinding", pathfinding_system);
engine.add_named_system(PHASE_MOVEMENT, "troops_pathfinding", troops_pathfinding_system);
engine.add_named_system(PHASE_MOVEMENT, "movement", movement_system);
engine.add_named_system(PHASE_SPAWN, "spawner", spawner_system);
```

**Manquent :**
- `combat_resolution_system` (Phase 200)
- `skill_gain_system` (Phase 300)
- `harvest_system` + `harvest_respawn_system` (Phase 400)
- `craft_system` (Phase 402)
- `ai_tick_system` (Phase 500)
- `trade_buy_system` + `trade_sell_system` (Phase 800)

**Impact :** Le code existe (1 592 lignes dans harvest.rs + craft.rs + economy.rs + mge-rpg-combat/ai/progression) mais n'est jamais exécuté. Le prototype ne fait que déplacer des sprites sur une carte.

### NC-2 : Spawns de créatures sans combat ni IA

Le `spawner_system` crée des entités avec uniquement :

```67:77:mge/examples/allumina_prototype/src/spawner.rs
let new_entity = world.spawn();
world.insert(new_entity, Position2D { x: world_x, y: world_y });
world.insert(new_entity, mge_plugin_spatial::Velocity2D { x: 0.0, y: 0.0 });
world.insert(new_entity, crate::components::PathfindingState::new(2.0));
world.insert(new_entity, crate::components::EntitySprite::monster(creature_type_id));
```

**Manquent sur les créatures :**
- `AIState` (FSM)
- `AiTargetable` (marqueur)
- `CombatStats` (atk, esq, par)
- `Health` (PV)
- `Armor`
- `StatBlock`
- `LootTable`

**Impact :** Les monstres sont des sprites inertes. Pas de combat, pas de mort, pas de loot.

### NC-3 : Pas de composant Collision

Le document MVP spécifie :

```rust
pub struct Collision {
    pub radius: f32,         // 0.5 tile = standard
    pub blocking: bool,
}
```

Ce composant n'existe nulle part. Le mouvement ne vérifie pas les collisions entre entités. Deux entités peuvent se superposer sur la même tile.

### NC-4 : Réseau totalement absent

Le MVP spécifie explicitement : "Réseau (Lobby MWS, serveur autoritaire)" comme inclus.

- Aucun crate réseau dans le workspace MGE
- Aucun module network dans allumina_prototype
- Aucune séparation input client / validation serveur
- Le document `MGE - Mode Multijoueur.md` spécifie une architecture complète (host/client/snapshot/delta/MWS) qui n'a aucune implémentation

### NC-5 : Persistence totalement absente

Le MVP spécifie : "Persistence (snapshot périodique)" et "Solo (sauvegarde locale, LOI-1, LOI-2)".

- `mge-plugin-save-load` est une dépendance mais ses systèmes sont vides (stub)
- Aucun composant n'implémente `Serialize`/`Deserialize` (sauf les structs JSON de config)
- Aucun système Phase 900 (persistence) n'est enregistré
- Les données de jeu disparaissent à la fermeture

### NC-6 : Trade joueur-joueur absent

Le MVP l'inclut : "Trade joueur-joueur : Interaction sociale fondamentale". Aucune implémentation ni même event.

### NC-7 : Loot absent

Bien que `LootTable` et `LootEntry` soient définis dans `mge-rpg-inventory`, aucun système de loot n'existe. Les monstres ne droppent rien à la mort.

---

# II — CONFORMITÉ ARCHITECTURALE

## 2.1 ECS pur

| Critère | Statut | Détail |
|---------|--------|--------|
| Composants = données pures | **CONFORME** | Tous les composants sont des struct sans logique complexe |
| Systèmes = fonctions pures | **PARTIEL** | Les systèmes sont `fn(World, Context)` mais certains font des I/O (tilemap clone) |
| Pas d'héritage OO | **CONFORME** | Aucun héritage, composition par composants |
| No God Objects | **VIOLATION** | `AlluminaMap` contient une `TileMap` complète clonée (potentiellement 256 Ko+) |
| Séparation simulation/rendu | **VIOLATION** | `main.rs` mélange winit events, wgpu rendering, et `engine.tick()` dans la même boucle |

### Violation : Couplage simulation/rendu

```154:181:mge/examples/allumina_prototype/src/main.rs
if let Event::AboutToWait = event {
    // ... dt calculation ...
    engine.tick(dt);   // SIMULATION
    // ... entity_sprites collection ...
    renderer.draw_tilemap(&map.tilemap, &camera, &entity_sprites);  // RENDU
}
```

La boucle principale mélange :
1. Calcul du delta time (horloge murale)
2. Tick de simulation
3. Collecte des sprites pour rendu
4. Appel wgpu pour le rendu
5. Gestion des événements winit

**Problème :** Le rendu bloque la simulation. Si le GPU est lent, le tick rate chute. Pour un serveur autoritaire, la simulation DOIT tourner indépendamment du rendu.

## 2.2 Déterminisme

| Critère | Statut | Détail |
|---------|--------|--------|
| RNG seedable | **CONFORME** | `Rng::new(seed)`, dérivation par entité via XOR |
| Fixed timestep | **PARTIEL** | `fixed_timestep_ms: Some(33)` configuré, mais `dt` réel passé à `tick()` |
| Résultats reproductibles | **VIOLATION** | HashMap dans A* → ordre d'itération non déterministe |
| Flottants déterministes | **RISQUE** | f32 peut varier entre architectures (x87 vs SSE) |

### Violation : Fixed timestep ignoré en pratique

Le code configure `fixed_timestep_ms: Some(33)` mais passe le dt réel à `engine.tick(dt)` :

```155:159:mge/examples/allumina_prototype/src/main.rs
let now = Instant::now();
let dt = (now - last_time).as_secs_f32().min(0.05);
last_time = now;
engine.tick(dt);
```

En interne, `Time::advance()` remplace ce `dt` par `33ms / 1000 = 0.033s` quand `fixed_timestep_ms` est `Some`. C'est correct, **mais** l'appelant ne le sait pas et calcule un dt inutilement. Le vrai problème : si le tick prend plus de 33ms, il n'y a pas d'accumulation ni de rattrapage. Le tick rate baisse simplement.

### Violation : A* non déterministe

```170:171:mge/examples/allumina_prototype/src/pathfinding.rs
let mut g_score: HashMap<(i32, i32), f32> = HashMap::new();
let mut parent: HashMap<(i32, i32), GridNode> = HashMap::new();
```

`HashMap` a un ordre d'itération aléatoire. Bien que l'algorithme A* ne dépende pas de l'ordre d'itération (il utilise un `BinaryHeap`), les HashMap sont consultés via `get()` uniquement, donc **c'est fonctionnellement déterministe**. Cependant, le `BinaryHeap` avec des f32 peut produire des ordres différents en cas d'égalité de coût — ce qui est un risque de non-déterminisme pour le réseau.

## 2.3 Server authoritative

| Critère | Statut |
|---------|--------|
| Validation des inputs serveur | **ABSENT** |
| Séparation input/simulation | **PARTIEL** — AlluminaInput existe mais pas de validation |
| Anti-triche | **ABSENT** |
| Rate limiting sur événements | **ABSENT** |

**Constat :** L'architecture actuelle est purement client-side. Aucune fondation pour un modèle serveur autoritaire n'existe. L'ajout du réseau nécessitera une refonte significative de la boucle principale.

---

# III — CONFORMITÉ MSCM / MIP

## 3.1 Annotations MSCM dans le code

| Zone | Fichiers avec @id | Fichiers sans @id | Couverture |
|------|-------------------|-------------------|------------|
| Kernel (7 crates) | 21/21 | 0 | **100%** |
| Core Universal (6 crates) | 24/24 | 0 | **100%** |
| Pack RPG (7 crates) | 28/28 | 0 | **100%** |
| Autres packs (~92 crates) | ~368/368 | 0 | **100%** |
| **allumina_prototype** | **0/17** | **17** | **0%** |

**Constat CRITIQUE :** Le prototype Allumina (3 360 lignes de code, 17 fichiers) n'a **aucune** annotation MSCM. Cela inclut les systèmes les plus complexes du projet : pathfinding, craft, economy, harvest, renderer.

Les fichiers Allumina contiennent des annotations informelles dans les doc-comments (`//! @phase 400`, `/// @fields`, `/// @requires`), mais ce ne sont **pas** des blocs MSCM conformes (il manque `@id`, `@role`, `@layer`, `@do`).

## 3.2 Index MIP

| Métrique | Valeur |
|----------|--------|
| Blocs indexés | 1 578 |
| Fichiers indexés | 696 |
| Domaines | 127 |
| Layers | 11 |
| Blocs MGE kernel indexés | 15 |
| Blocs MGE packs indexés | **0** |
| Blocs Allumina indexés | **0** |

**Constat CRITIQUE :** L'index MIP ne couvre que l'infrastructure COG (`crates/` racine). Les 400+ fichiers MGE annotés sous `mge/crates/` ne sont **pas indexés**. L'index est inutilisable pour la navigation du moteur de jeu.

**Cause probable :** Le scanner MIP scanne `crates/` depuis la racine workspace mais ne descend pas dans le sous-workspace `mge/crates/`.

## 3.3 Impact sur la vérification MWS

Le protocole MIP sert à la **Phase B de vérification relay** (attestation d'intégrité par blocs MSCM). Si Allumina doit tourner comme Service sur un COG :

- Les 17 fichiers Allumina sans annotations ne peuvent pas être vérifiés
- Les systèmes de jeu ne sont pas gouvernés
- La conformité MWS est **impossible** en l'état

---

# IV — DETTE TECHNIQUE

## 4.1 Allocations par tick (performance)

| Source | Allocation | Fréquence | Coût estimé |
|--------|-----------|-----------|-------------|
| `for_each1_mut<A>` | `Vec<EntityId>` clone | Chaque appel | O(n) par query |
| `iter1`, `iter2`, `iter3` | `Vec<EntityId>` clone | Chaque appel | O(n) par query |
| `iter3` | `HashSet` + filtrage | Chaque appel | O(n) par query |
| `entry.name.clone()` dans Scheduler | `String` clone | Par système nommé par tick | 6+ clones/tick |
| `TileMap::clone()` dans spawner | Clone 256KB+ | Chaque tick (spawner_system) | O(width × height) |
| A* `HashMap` + `BinaryHeap` | Nouvelles collections | Par pathfinding request | O(path_length²) |

**Impact à 1 000 entités :** Chaque `for_each_mut` alloue un Vec de 1 000 EntityIds. Avec 6+ systèmes appelant des queries, c'est ~6 000 EntityIds alloués puis libérés par tick. À 30 TPS = 180 000 allocations/seconde juste pour les queries.

**Recommandation :** Scratch buffer réutilisable (arena allocator ou `Vec` persistant dans le Scheduler).

## 4.2 Pas de `remove_component<T>()` sur World

`SparseSet::remove()` existe en interne mais `World` ne l'expose pas publiquement. Or le gameplay nécessite de retirer des composants :
- Retirer `DeadTag` lors de la résurrection
- Retirer `AttackCooldown` quand l'arme est désécquipée
- Retirer `TroopFollower` quand un suiveur est libéré

**Impact :** Oblige à `despawn` + `spawn` nouveau, ce qui change l'`EntityId` et casse toutes les références (inventaire, cibles IA, troupe).

## 4.3 Pas de Spatial Index / Partitionnement

`SpatialHash` est défini dans `mge-plugin-spatial` comme composant mais :
- Aucun système ne met à jour le `cell_id`
- Aucune structure de données spatiale n'existe (grille, quadtree, sector)
- Toutes les queries spatiales sont O(n) sur TOUTES les entités

L'IA utilise une boucle O(n) pour trouver la cible la plus proche :

```rust
// Dans mge-rpg-ai/src/systems.rs — find_target
fn find_target(world: &World, ...) -> Option<(EntityId, f32, f32)> {
    let mut best = None;
    for (eid, pos, _) in world.iter2::<Position2D, AiTargetable>() {
        // ... distance check ...
    }
    best
}
```

**Impact à 1 000 entités :** Chaque monstre scanne les 1 000 entités pour trouver sa cible. Avec 200 monstres = 200 000 comparaisons/tick. À 30 TPS = 6M comparaisons/seconde.

## 4.4 Clonage massif de TileMap

Le `spawner_system` clone la TileMap complète à chaque tick :

```21:24:mge/examples/allumina_prototype/src/spawner.rs
let tilemap = world
    .iter1::<AlluminaMap>()
    .next()
    .map(|(_, m)| m.tilemap.clone());
```

Pour une carte 256×256, `TileMap` = 256 × 256 × `size_of::<Tile>()` (5 bytes) = ~327 Ko alloués et copiés à chaque tick.

## 4.5 `Plugin::dependencies()` non résolu

Le trait `Plugin` déclare `dependencies()` :

```rust
fn dependencies(&self) -> &[&str] { &[] }
```

`AlluminaPlugin` déclare une dépendance :

```50:52:mge/examples/allumina_prototype/src/plugin.rs
fn dependencies(&self) -> &[&str] {
    &["mge-plugin-spatial"]
}
```

Mais `Engine::build()` **ignore complètement** cette méthode. Les plugins sont buildés dans l'ordre d'ajout. Si un plugin dépend d'un autre qui n'est pas ajouté, rien ne le signale. Code mort.

## 4.6 `mge-query` crate vide

Le crate `mge-query` (13 lignes) ne fait que réexporter `mge-ecs`. Aucune valeur ajoutée. Il apparaît dans le workspace mais n'est utilisé par aucun autre crate.

## 4.7 `PhaseId` sans validation de collision

`PhaseId(pub u32)` est un tuple struct avec champ public. N'importe quel plugin peut utiliser n'importe quel numéro. Aucun mécanisme ne vérifie que deux plugins n'enregistrent pas des systèmes sur le même `PhaseId` avec des ordres contradictoires.

L'Allumina prototype utilise Phase 10, 50, 100, 600. Le MVP doc spécifie des phases jusqu'à 950. `mge-rpg-combat` utilise Phase 200. `mge-rpg-ai` utilise Phase 500. Pas de conflit actuellement, mais aucune protection structurelle.

---

# V — RISQUES DE SCALABILITÉ

## 5.1 Single-threaded complet

Le `Scheduler` exécute tous les systèmes séquentiellement sur un seul thread :

```rust
for (phase, entries) in &mut self.systems {
    for entry in entries {
        (entry.func)(world, ctx);
    }
}
```

Tous les systèmes partagent `&mut World` — impossible de paralléliser sans refonte du borrow model.

**Impact :** La simulation est bornée par un seul cœur. Pour 200 joueurs + 500 monstres + A* + combat + économie, le budget de 30ms/tick sera rapidement dépassé.

## 5.2 EventQueue sans backpressure

L'`EventQueue` accepte un nombre illimité d'événements :

```rust
pub fn emit<E: Event>(&mut self, event: E) {
    self.write_buffer
        .entry(TypeId::of::<E>())
        .or_default()
        .push(Box::new(event));
}
```

Chaque événement est boxé (`Box<dyn Any>`) — allocation heap. Pas de limite, pas de backpressure, pas de pooling. Un burst de 10 000 `AttackRequestEvent` en un tick alloue 10 000 Box.

## 5.3 Pas d'Interest Management

Aucun mécanisme ne limite les données envoyées aux clients. En l'état (si le réseau était implémenté), chaque client recevrait l'état de TOUTES les entités du monde.

Pour 200 joueurs + 500 monstres, ça représente ~700 × (position + health + ...) × 30 TPS = potentiellement 1+ Mo/s par client.

---

# VI — FAILLES D'EXPLOITATION

## 6.1 SpawnerData.current_count jamais décrémenté

Quand un monstre meurt, rien ne décrémente `SpawnerData.current_count`. Le spawner atteindra `max_count` et cessera de spawner indéfiniment.

**Exploit :** Tuer tous les monstres d'une zone rend la zone définitivement vide.

## 6.2 Pas de validation de portée sur craft/harvest

Les distances dans craft (16.0) et harvest (4.0) sont en distance² monde. Mais aucune vérification n'empêche un joueur d'émettre un `CraftRequestEvent` ou `HarvestRequestEvent` sans être à portée — car ces systèmes ne sont pas câblés et les événements viennent directement du client.

## 6.3 Gold overflow théorique

`Gold.amount` est `u64`. `saturating_add` empêche le panic mais permet d'atteindre `u64::MAX` (18.4 quintillions). Pas de plafond logique.

## 6.4 Consommation d'items non atomique dans craft

`consume_items()` dans `craft.rs` collecte d'abord les items à consommer, puis les consomme un par un. Si une erreur survient au milieu (impossible en mono-thread, mais possible en multi-thread ou réseau), certains items seraient consommés sans que le craft aboutisse.

## 6.5 Race condition sur l'inventaire

Le pattern récurrent dans economy.rs et craft.rs est :
1. Vérifier `first_free_slot()` → slot disponible
2. ... autre logique (spawn item, etc.) ...
3. Insérer dans `inv.slots[idx]`

L'index `idx` n'est pas verrouillé. En mono-thread c'est safe, mais en multi-thread ou avec des événements concurrents sur le même tick, deux systèmes pourraient cibler le même slot.

---

# VII — DÉRIVES DE SCOPE

## 7.1 Dépendances non justifiées

Le `Cargo.toml` d'allumina_prototype déclare **57 dépendances** dont :

| Pack | Crates importés | Utilisés réellement | Verdict |
|------|----------------|--------------------|---------| 
| Social | 6 (relationship, faction, reputation, need, schedule, personality) | 0 | **BALLAST** |
| Visual Novel | 4 (script, character, choice, branch) | 0 | **BALLAST** |
| RPG Quest | 1 | 0 | **BALLAST** |
| RPG Dialogue | 1 | 0 | **BALLAST** |
| Plugin Audio | 1 | 0 | **BALLAST** |
| Plugin Basic Physics | 1 | 0 | **BALLAST** |

12 dépendances inutilisées sur 57 = 21% de ballast.

**Risque :** Augmente le temps de compilation sans valeur. Crée une confusion sur le scope réel.

## 7.2 Description Cargo.toml incohérente

```1:5:mge/examples/allumina_prototype/Cargo.toml
[package]
name = "allumina_prototype"
version = "0.1.0"
edition = "2021"
description = "MGE Example — Multi-pack (RPG + Social + Narrative)"
```

La description dit "Multi-pack (RPG + Social + Narrative)" alors que le Social et le Narrative ne sont pas utilisés. Le MVP est un sandbox RPG, pas un multi-pack.

## 7.3 Pas de crate Sandbox (mge-sb-*)

Le document MVP décrit un sandbox (harvest, craft, economy, terrain) mais le prototype n'importe **aucun** crate du pack Sandbox (`mge-sb-crafting`, `mge-sb-terrain`, `mge-sb-wildlife`, etc.). Toute la logique sandbox est réimplémentée localement dans le prototype.

---

# VIII — SOLIDITÉ DES FONDATIONS POUR PHASE 2

## 8.1 Ce qui est solide

| Fondation | Qualité | Justification |
|-----------|---------|---------------|
| Kernel ECS | **Excellente** | Zero unsafe, generational IDs, free-list, SparseSet correct |
| Plugin trait | **Bonne** | API propre, extensible |
| EventQueue double-buffer | **Bonne** | Propagation tick N → N+1 correcte |
| RNG déterministe | **Bonne** | Seed reproductible, dérivation par entité |
| Profiler tick budget | **Bonne** | Métriques par phase/système, détection overflow |
| Pathfinding A* | **Bonne** | Implémentation correcte, configurable |
| Rendu wgpu | **Correcte** | Atlas grassland, instancing, projection iso |

## 8.2 Ce qui bloque la phase 2

| Blocage | Sévérité | Effort estimé |
|---------|----------|---------------|
| Pas de `remove_component<T>()` | **Critique** | 1 jour |
| Pas de spatial index | **Critique** | 3-5 jours |
| Pas de sérialisation sur composants | **Critique** | 2-3 jours |
| Pas de networking | **Majeur** | 2-4 semaines |
| Pas de persistence | **Majeur** | 1-2 semaines |
| Pas de multi-threading | **Majeur** | 2-4 semaines (refonte scheduler) |
| Pas de loot system | **Modéré** | 2-3 jours |
| Pas de trade P2P | **Modéré** | 3-5 jours |
| Séparation simulation/rendu | **Modéré** | 1 semaine |

## 8.3 Évaluation globale phase 2

La **couche kernel** est prête pour la phase 2. Son API est stable, bien testée, et extensible.

La **couche gameplay** est à 30% : les composants et 3 systèmes (combat, AI, progression) sont implémentés et testés dans les crates RPG pack, mais ils ne sont pas câblés dans le prototype. 5 systèmes existent localement (harvest, craft, economy) mais ne sont pas câblés non plus.

La **couche infrastructure** (réseau, persistence, spatial index) est à **0%**. C'est le plus gros risque pour la phase 2. Sans réseau et persistence, il n'y a pas de MMO sandbox.

---

# IX — RECOMMANDATIONS PRIORISÉES

## Priorité 1 — Corrections critiques (1 semaine)

| # | Action | Effort |
|---|--------|--------|
| R1 | Câbler les 7 systèmes existants dans AlluminaPlugin (combat, AI, stats, skills, harvest, craft, economy) | 1 jour |
| R2 | Ajouter `AIState`, `CombatStats`, `Health`, `Armor`, `StatBlock`, `LootTable` aux créatures spawnées | 1 jour |
| R3 | Ajouter `remove_component<T>()` public sur World | 0.5 jour |
| R4 | Implémenter `SpawnerData.current_count` décrémentation à la mort des créatures | 0.5 jour |
| R5 | Ajouter `Collision` component et vérification dans movement_system | 1 jour |
| R6 | Ajouter annotations MSCM aux 17 fichiers allumina_prototype | 1 jour |

## Priorité 2 — Fondations phase 2 (2-3 semaines)

| # | Action | Effort |
|---|--------|--------|
| R7 | Implémenter un spatial index (grille ou sectors) dans mge-plugin-spatial | 3 jours |
| R8 | Séparer la boucle simulation de la boucle rendu (thread ou alternance) | 3 jours |
| R9 | Implémenter le loot system (drop monstres → items) | 2 jours |
| R10 | Implémenter le trade joueur-joueur (direct, pas de marché) | 3 jours |
| R11 | Ajouter `Serialize`/`Deserialize` (serde) sur les composants clés | 2 jours |
| R12 | Implémenter la persistence par snapshot JSON | 3 jours |

## Priorité 3 — Nettoyage (1 semaine)

| # | Action | Effort |
|---|--------|--------|
| R13 | Supprimer les 12 dépendances inutilisées de Cargo.toml | 0.5 jour |
| R14 | Corriger la description Cargo.toml | 5 min |
| R15 | Regénérer l'index MIP en incluant `mge/crates/` | 1 jour |
| R16 | Éliminer le clone TileMap dans spawner_system (passer par une query read) | 0.5 jour |
| R17 | Remplacer les allocations Vec par tick dans les queries par un scratch buffer | 2 jours |
| R18 | Supprimer ou intégrer le crate mge-query (vide) | 0.5 jour |

---

# X — MÉTRIQUES QUANTITATIVES

## Lignes de code

| Zone | Lignes | % du total |
|------|--------|-----------|
| Kernel MGE (7 crates) | 1 747 | 29% |
| Plugins core (6 crates) | ~300 | 5% |
| RPG pack (7 crates, non-stubs) | ~850 | 14% |
| Allumina prototype (17 fichiers) | 3 150 | 52% |
| **Total code effectif** | **~6 047** | 100% |

## Tests

| Zone | Tests | Couverture |
|------|-------|-----------|
| Kernel | 21 | Bonne (tick, events, RNG, profiler, ECS) |
| RPG combat | 1 | Smoke test |
| RPG AI | 1 | Smoke test |
| RPG progression | 1 | Smoke test |
| Allumina prototype | 0 | **Aucune** |
| **Total** | **24** | **Insuffisante** |

## Dépendances externes (runtime)

| Crate | Version | Taille estimée |
|-------|---------|---------------|
| rand | 0.8 | ~30 Ko |
| winit | 0.29 | ~500 Ko |
| wgpu | 24 | ~5 Mo |
| image | 0.25 | ~1 Mo |
| serde | 1 | ~200 Ko |
| serde_json | 1 | ~100 Ko |
| bytemuck | 1.14 | ~10 Ko |
| pollster | 0.3 | ~5 Ko |
| anyhow | 1 | ~20 Ko |
| env_logger | 0.10 | ~50 Ko |
| log | 0.4 | ~10 Ko |

**Surface d'attaque externe :** 11 crates, dominée par wgpu (5 Mo). Le kernel pur n'a qu'une seule dépendance (rand 0.8).

---

# CONCLUSION

Le kernel MGE est une fondation saine et bien conçue. Zero unsafe, API minimaliste, tests corrects. La philosophie architecturale (ECS pur, tick déterministe, event bus, profiling) est conforme aux objectifs.

Le prototype Allumina est un **démonstrateur visuel** fonctionnel (fenêtre wgpu, rendu isométrique, pathfinding, spawner) mais il ne constitue **pas** un MVP jouable au sens du document de spécification. Sur 18 systèmes spécifiés, 3 sont conformes, 6 ont du code non câblé, et 4 sont totalement absents.

Les trois blocages structurels pour atteindre un MVP réel sont : (1) câbler les systèmes existants, (2) implémenter réseau + persistence, (3) séparer simulation et rendu. Avec les recommandations de priorité 1 et 2, un MVP fonctionnel est atteignable en 4-6 semaines supplémentaires.
