# MGE — Pack Roguelike

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  
**Couche** : Layer 2 (Genre Pack)  
**Repertoire** : `mge/crates/roguelike/`  
**Nombre de crates** : 4  

---

## 1. Contexte

Le Pack Roguelike fournit les mecaniques generiques des jeux roguelike et roguelite : generation procedurale de donjons, gestion des etages, objets a affixes aleatoires et permadeath avec meta-progression. Il s'appuie sur le Pack RPG pour l'inventaire et la progression.

Tous les crates sont scaffoldes (v0.1.0). Les composants, systemes et evenements decrits dans les fichiers plugin constituent la specification d'implementation cible.

---

## 2. Portee

- **Types de jeux** : Roguelike classique, roguelite, dungeon crawler procedural, mystery dungeon.
- **Hors portee** : Rendu de carte, generation de monde ouvert (voir Pack Sandbox), combat (utiliser Pack RPG ou Pack Shooter).
- **Audience** : Developpeurs moteur, designers, LLM.
- **Prerequis** : Kernel Layer 0 (mge-ecs, mge-event). Core Universal Pack (spatial). Pack RPG (inventory, progression).

---

## 3. Vision

Le Pack Roguelike est un ensemble de plugins simulation-first. Chaque plugin :

- Fournit des composants (donnees pures) et des systemes (1 fn = 1 effet).
- Ne contient aucune logique de jeu specifique.
- S'execute en headless sans rendu.
- Produit un comportement deterministe a seed et input identiques.
- Expose ses parametres via GCL pour configuration sans recompilation.

---

## 4. Architecture globale

```
mge/crates/roguelike/
├── mge-rl-procgen/         # Generation procedurale de donjons
├── mge-rl-floor/           # Gestion etages, tiles, portes, brouillard
├── mge-rl-item/            # Objets roguelike, rarete, affixes
└── mge-rl-permadeath/      # Permadeath, run state, meta-progression
```

### Graphe de dependances intra-pack

```
mge-rl-floor ──────► mge-rl-procgen
mge-rl-item (feuille)
mge-rl-permadeath (feuille)
```

### Dependances vers Pack RPG

```
mge-rl-item ──────► mge-rpg-inventory
mge-rl-permadeath ──────► mge-rpg-progression
```

Crates feuilles (sans dependance intra-pack) : `mge-rl-item`, `mge-rl-permadeath`.

---

## 5. Sous-packs

Aucun. Les 4 crates forment un seul pack plat.

---

## 6. Liste des plugins

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 1 | `mge-rl-procgen` | `mge.rl.procgen.v1` | [mge-rl-procgen.md](mge-rl-procgen.md) | Generation procedurale de donjons (salles, corridors, layout) |
| 2 | `mge-rl-floor` | `mge.rl.floor.v1` | [mge-rl-floor.md](mge-rl-floor.md) | Etages, tiles, portes, brouillard de guerre |
| 3 | `mge-rl-item` | `mge.rl.item.v1` | [mge-rl-item.md](mge-rl-item.md) | Objets roguelike, rarete, affixes aleatoires |
| 4 | `mge-rl-permadeath` | `mge.rl.permadeath.v1` | [mge-rl-permadeath.md](mge-rl-permadeath.md) | Permadeath, run, tombstone, meta-progression |

---

## 7. Composants cles (resume)

| Plugin | Composants runtime | Composants donnees statiques |
|--------|--------------------|------------------------------|
| procgen | DungeonSeed, RoomGraph, RoomConfig, DungeonConfig | aucun |
| floor | FloorState, FloorMap, Tile, RoomInstance, DoorState | aucun |
| item | RogueItem, ItemPool, ItemRarity, ItemAffix | aucun |
| permadeath | RunState, RunStats, Tombstone, RunSeed | aucun |

---

## 8. Systemes cles (resume)

| Phase | Plugin | Systemes |
|-------|--------|----------|
| 1800-1803 | procgen | generate_dungeon, place_rooms, connect_rooms, populate_room |
| 1820-1823 | floor | init_floor, reveal_tiles, process_door_interaction, check_floor_clear |
| 1840-1843 | item | generate_item, roll_affixes, apply_item_effect, merge_duplicates |
| 1860-1863 | permadeath | start_run, check_permadeath, generate_tombstone, process_meta_progression |

**Ordre d'execution** : procgen (1800) → floor (1820) → item (1840) → permadeath (1860).

**Justification** : la generation procedurale s'execute a l'initialisation d'un etage. Le floor gere l'etat courant de l'etage genere. Les items sont generes et places dans les salles. Le permadeath surveille la mort du joueur et gere les transitions de run.

**Total** : 16 systemes.

---

## 9. Evenements cles (resume)

| Plugin | Requests (entree) | Events (sortie) |
|--------|-------------------|------------------|
| procgen | GenerateDungeonRequest | DungeonGenerated, RoomGenerated, RoomConnected |
| floor | EnterFloorRequest, DoorInteractRequest | FloorEntered, FloorCleared, TileRevealed, DoorOpened |
| item | GenerateItemRequest | ItemGenerated, AffixRolled, ItemConsumed |
| permadeath | StartRunRequest | RunStarted, RunEnded, PermadeathTriggered, MetaUnlocked |

**Total** : 5 requests + 11 events = 16 evenements.

---

## 10. Dependances

### Dependances vers Kernel (Layer 0)

| Crate | Depend de |
|-------|-----------|
| Tous les 4 crates | `mge-ecs`, `mge-event` |

### Dependances vers Core Universal

| Crate | Depend de |
|-------|-----------|
| `mge-rl-procgen` | `mge-plugin-spatial` |
| `mge-rl-floor` | `mge-plugin-spatial` |

### Dependances vers Pack RPG

| Crate | Depend de |
|-------|-----------|
| `mge-rl-item` | `mge-rpg-inventory` |
| `mge-rl-permadeath` | `mge-rpg-progression` |

### Dependances intra-pack

| Crate | Depend de |
|-------|-----------|
| `mge-rl-floor` | `mge-rl-procgen` |

### Dependances externes (aucune)

Le Pack Roguelike n'a aucune dependance vers des crates externes.

---

## 11. Interaction avec GCL

Le GCL configure les plugins Roguelike sans recompilation.

**Parametres exposables :**

- Taille min/max des donjons, nombre de salles
- Algorithme de generation (BSP, Random, Cellular, WFC)
- Rarete des objets, nombre d'affixes
- Conditions de permadeath, meta-rewards par run
- Brouillard de guerre (rayon de vision)

Le GCL ne modifie pas la structure des composants. Il parametre les systemes.

---

## 12. Interaction avec autres packs

| Pack dependant | Crates Roguelike utilises | Usage |
|----------------|--------------------------|-------|
| (aucun actuellement) | — | — |

### Packs pre-requis

| Pack | Crates utilises | Usage |
|------|-----------------|-------|
| **RPG** | inventory, progression | Inventaire du joueur, XP et niveau par run |

Le Pack Roguelike ne sert de dependance a aucun autre pack genre actuellement.

---

## 13. Contraintes determinisme

| Contrainte | Detail |
|------------|--------|
| **Seed obligatoire** | Toute generation procedurale utilise un seed explicite |
| **RNG kernel** | Utilise mge-rng, pas rand::thread_rng |
| **Pas de HashMap order-dependent** | Iteration ordonnee pour placement |
| **Pas de static mut** | Interdit par la norme AI-Native |
| **Reproductibilite** | Meme seed + meme input = meme donjon |

---

## 14. Contraintes performance

| Contrainte | Detail |
|------------|--------|
| **Hot path** | floor (reveal_tiles), procgen (generation one-shot) |
| **Budget cible** | Generation donjon < 50ms pour 100 salles |
| **Budget tick** | < 0.5ms par tick en exploration |
| **Pas d'allocation** | tick_regen, reveal_tiles sans allocation |

---

## 15. Limites v1

| Limite | Raison |
|--------|--------|
| Pas de generation 3D | 2D uniquement en v1 |
| Pas de donjons dynamiques (modification) | Les salles sont statiques apres generation |
| Pas de brouillard persistant multi-etage | Reset a chaque changement d'etage |
| Pas de crafting d'affixes | Voir Pack Sandbox |
| Pas de leaderboard | Hors scope (service externe) |

---

## 16. Extensions possibles v2

| Extension | Description |
|-----------|-------------|
| WFC (Wave Function Collapse) | Algorithme de generation avance |
| Donjons dynamiques | Salles qui changent pendant l'exploration |
| Brouillard persistant | Sauvegarde par etage |
| Synergies d'affixes | Bonus quand certains affixes se combinent |
| Daily runs | Seed deterministe par jour |

---

## 17. Exemple d'assemblage

### Minimal (headless, procgen + floor uniquement)

```rust
let mut engine = Engine::new(EngineConfig::default());
engine.add_plugin(MgePluginSpatial::default());
engine.add_plugin(MgeRlProcgenPlugin);
engine.add_plugin(MgeRlFloorPlugin);
engine.build();
```

### Complet (roguelike jouable)

```rust
let mut engine = Engine::new(EngineConfig::default());
// Core Universal
engine.add_plugin(MgePluginSpatial::default());
engine.add_plugin(MgePluginInput::default());
engine.add_plugin(MgePluginRender2d::default());
// Pack RPG (dependance)
engine.add_plugin(MgeRpgInventoryPlugin);
engine.add_plugin(MgeRpgProgressionPlugin);
engine.add_plugin(MgeRpgCombatPlugin);
engine.add_plugin(MgeRpgStatsPlugin);
// Pack Roguelike
engine.add_plugin(MgeRlProcgenPlugin);
engine.add_plugin(MgeRlFloorPlugin);
engine.add_plugin(MgeRlItemPlugin);
engine.add_plugin(MgeRlPermadeathPlugin);
engine.build();
```

---

## 18. Organisation des crates

```
mge/crates/roguelike/
├── mge-rl-procgen/
│   ├── Cargo.toml
│   ├── index.md
│   └── src/
│       ├── lib.rs           # @id mge.rl.procgen.v1
│       ├── components.rs
│       ├── systems.rs
│       └── events.rs
├── mge-rl-floor/
│   └── (meme structure)
├── mge-rl-item/
│   └── (meme structure)
└── mge-rl-permadeath/
    └── (meme structure)
```

---

## 19. Resume strategique

Le Pack Roguelike est la brique fondamentale des jeux roguelike/roguelite dans MGE. Il :

- Fournit 4 plugins couvrant procgen, etages, objets a affixes et permadeath.
- Reste generique : aucune logique specifique a un jeu.
- S'execute en headless, en deterministe (seed-based), sans rendu.
- S'appuie sur le Pack RPG pour l'inventaire et la progression.
- Expose ses parametres via GCL pour iteration rapide.
- Respecte strictement la norme AI-Native (MSCM, 1 fn = 1 effet, max 30 lignes, pas de hidden state).

Les 4 crates sont scaffoldes (v0.1.0). L'implementation suit les specifications des fichiers plugin individuels.

---

## References

| Document | Role |
|----------|------|
| [MGE - Pack Architecture](../MGE%20-%20Pack%20Architecture.md) | Couches, composition |
| [MGE - Architecture Generale](../MGE%20-%20Architecture%20Generale.md) | Couches globales |
| [MGE - Plugin Contract](../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
| [MGE - AI-Native Writing Standard v1](../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [Pack RPG - Index](../rpg/_index.md) | Pack RPG (dependance) |
