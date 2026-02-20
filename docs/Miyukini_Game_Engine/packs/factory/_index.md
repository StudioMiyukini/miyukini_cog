# MGE — Pack Factory

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  
**Couche** : Layer 2 (Genre Pack)  
**Repertoire** : `mge/crates/factory/`  
**Nombre de crates** : 4  

---

## 1. Contexte

Le Pack Factory fournit les mecaniques generiques des jeux d'automatisation et de gestion de production : machines, recettes de transformation, convoyeurs et logistique. Il est leger et composable, utilisable en standalone pour un factory builder ou en complement du Pack Tycoon pour un jeu de gestion industrielle.

Tous les crates sont scaffoldes (v0.1.0). Les composants, systemes et evenements decrits dans les fichiers plugin constituent la specification d'implementation cible.

---

## 2. Portee

- **Types de jeux** : Factory builder, automation game, idle factory, gestion industrielle, simulation de chaines de production.
- **Hors portee** : Rendu 3D des machines, simulation electrique/fluide, vehicules de transport (voir Pack Racing), construction libre (voir Pack Sandbox).
- **Audience** : Developpeurs moteur, designers, LLM.
- **Prerequis** : Kernel Layer 0 (mge-ecs, mge-event). Core Universal Pack obligatoire (spatial).

---

## 3. Vision

Le Pack Factory est un ensemble de plugins simulation-first. Chaque plugin :

- Fournit des composants (donnees pures) et des systemes (1 fn = 1 effet).
- Ne contient aucune logique de jeu specifique.
- S'execute en headless sans rendu.
- Produit un comportement deterministe a seed et input identiques.
- Expose ses parametres via GCL pour configuration sans recompilation.

---

## 4. Architecture globale

```
mge/crates/factory/
├── mge-factory-machine/        # Machines, etats, input/output slots
├── mge-factory-recipe/         # Recettes transformation, ingredients, produits
├── mge-factory-conveyor/       # Convoyeurs, transport, buffers
└── mge-factory-logistics/      # Routage, priorites, stockage global
```

### Graphe de dependances intra-pack

```
mge-factory-logistics ──────► mge-factory-conveyor
        │                           │
        └──► mge-factory-machine ◄──┘
                    │
                    └──► mge-factory-recipe
```

Crates feuilles (sans dependance intra-pack) : `mge-factory-recipe`.

---

## 5. Sous-packs

Aucun. Les 4 crates forment un seul pack plat.

---

## 6. Liste des plugins

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 1 | `mge-factory-machine` | `mge.factory.machine.v1` | [mge-factory-machine.md](mge-factory-machine.md) | Machines, etats, slots input/output, production |
| 2 | `mge-factory-recipe` | `mge.factory.recipe.v1` | [mge-factory-recipe.md](mge-factory-recipe.md) | Recettes de transformation, ingredients, produits |
| 3 | `mge-factory-conveyor` | `mge.factory.conveyor.v1` | [mge-factory-conveyor.md](mge-factory-conveyor.md) | Convoyeurs, transport d'items, buffers |
| 4 | `mge-factory-logistics` | `mge.factory.logistics.v1` | [mge-factory-logistics.md](mge-factory-logistics.md) | Routage, priorites, stockage global |

---

## 7. Composants cles (resume)

| Plugin | Composants runtime | Composants donnees statiques |
|--------|--------------------|------------------------------|
| machine | Machine, MachineState, InputSlot, OutputSlot, ProcessingTimer | MachineDef |
| recipe | Recipe, Ingredient, Product, RecipeBook | aucun |
| conveyor | Conveyor, ConveyorSegment, ConveyorBuffer, ConveyorItem | aucun |
| logistics | LogisticsNode, LogisticsRoute, StorageContainer, RoutePriority | aucun |

---

## 8. Systemes cles (resume)

| Phase | Plugin | Systemes |
|-------|--------|----------|
| 2000-2003 | machine | tick_machine, consume_inputs, produce_outputs, update_machine_state |
| 2020-2023 | recipe | match_recipe, validate_ingredients, apply_recipe, unlock_recipe |
| 2040-2043 | conveyor | move_items, transfer_to_machine, transfer_from_machine, tick_buffer |
| 2060-2063 | logistics | compute_routes, assign_priorities, distribute_items, balance_storage |

**Ordre d'execution** : machine (2000) → recipe (2020) → conveyor (2040) → logistics (2060).

**Justification** : les machines consomment et produisent en premier. Les recettes resolvent les transformations. Les convoyeurs deplacent les items. La logistique repartit les flux globalement.

**Total** : 16 systemes.

---

## 9. Evenements cles (resume)

| Plugin | Requests (entree) | Events (sortie) |
|--------|-------------------|------------------|
| machine | PlaceMachineRequest, StartMachineRequest | MachineStarted, MachineStopped, ProductionCompleted, MachineJammed |
| recipe | CraftRequest | RecipeMatched, RecipeCompleted, RecipeFailed |
| conveyor | (automatique via connections) | ItemTransferred, BufferFull, BufferEmpty |
| logistics | RouteRequest | RouteComputed, ItemDistributed, StorageFull, StorageEmpty |

**Total** : 4 requests + 11 events = 15 evenements.

---

## 10. Dependances

### Dependances vers Kernel (Layer 0)

| Crate | Depend de |
|-------|-----------|
| Tous les 4 crates | `mge-ecs`, `mge-event` |

### Dependances vers Core Universal

| Crate | Depend de |
|-------|-----------|
| `mge-factory-machine` | `mge-plugin-spatial` |
| `mge-factory-conveyor` | `mge-plugin-spatial` |
| `mge-factory-logistics` | `mge-plugin-spatial` |

### Dependances intra-pack

| Crate | Depend de |
|-------|-----------|
| `mge-factory-machine` | `mge-factory-recipe` |
| `mge-factory-conveyor` | `mge-factory-machine` |
| `mge-factory-logistics` | `mge-factory-machine`, `mge-factory-conveyor` |

### Dependances externes (aucune)

Le Pack Factory n'a aucune dependance vers des crates externes.

---

## 11. Interaction avec GCL

Le GCL configure les plugins Factory sans recompilation.

**Parametres exposables :**

- Vitesse de production, capacite des slots
- Nombre max d'ingredients par recette
- Vitesse des convoyeurs, taille des buffers
- Algorithme de routage, priorites de distribution

Le GCL ne modifie pas la structure des composants. Il parametre les systemes.

---

## 12. Interaction avec autres packs

| Pack dependant | Crates Factory utilises | Usage |
|----------------|------------------------|-------|
| (aucun actuellement) | — | — |

Packs pouvant s'integrer :

| Pack | Integration possible |
|------|----------------------|
| **Tycoon** | Gestion financiere des usines. Cout des machines, revenus des produits |
| **Sandbox** | Crafting utilisant les recettes Factory |

Le Pack Factory ne depend d'aucun autre pack genre.

---

## 13. Contraintes determinisme

| Contrainte | Detail |
|------------|--------|
| **Pas de float non deterministe** | Operations deterministes, pas de NaN |
| **Pas de HashMap order-dependent** | Iteration ordonnee pour les routes |
| **Pas de static mut** | Interdit par la norme AI-Native |
| **Pas de thread-local** | Aucun etat cache |
| **Reproductibilite** | Meme configuration + meme input = meme production |

---

## 14. Contraintes performance

| Contrainte | Detail |
|------------|--------|
| **Hot path** | conveyor (move_items), machine (tick_machine) |
| **Budget cible** | < 2ms pour 500 machines + 1000 convoyeurs a 60 FPS |
| **Pas de dynamic dispatch** | Dans le hot path |
| **SoA storage** | Composants stockes en SoA via mge-ecs |
| **Pas d'allocation** | Dans les systemes hot path (pre-allouer) |

---

## 15. Limites v1

| Limite | Raison |
|--------|--------|
| Pas de fluides/gaz | Simplification v1, items discrets uniquement |
| Pas de chaleur/electricite | Hors scope (simulation physique) |
| Pas de convoyeurs 3D (hauteur) | 2D uniquement en v1 |
| Pas de logistique inter-usine | Single factory en v1 |
| Pas d'optimisation automatique | Hors scope (le joueur decide) |

---

## 16. Extensions possibles v2

| Extension | Description |
|-----------|-------------|
| Fluides | Pipes, reservoirs, debits |
| Electricite | Reseau electrique, consommation, production |
| Multi-usine | Logistique inter-sites |
| Recettes en chaine | Output d'une recette = input d'une autre (automatique) |
| Blueprints | Sauvegarder/charger des layouts de machines |

---

## 17. Exemple d'assemblage

### Minimal (headless, machine + recipe uniquement)

```rust
let mut engine = Engine::new(EngineConfig::default());
engine.add_plugin(MgePluginSpatial::default());
engine.add_plugin(MgeFactoryMachinePlugin);
engine.add_plugin(MgeFactoryRecipePlugin);
engine.build();
```

### Complet (factory builder jouable)

```rust
let mut engine = Engine::new(EngineConfig::default());
// Core Universal
engine.add_plugin(MgePluginSpatial::default());
engine.add_plugin(MgePluginInput::default());
engine.add_plugin(MgePluginRender2d::default());
// Pack Factory
engine.add_plugin(MgeFactoryMachinePlugin);
engine.add_plugin(MgeFactoryRecipePlugin);
engine.add_plugin(MgeFactoryConveyorPlugin);
engine.add_plugin(MgeFactoryLogisticsPlugin);
engine.build();
```

---

## 18. Organisation des crates

```
mge/crates/factory/
├── mge-factory-machine/
│   ├── Cargo.toml
│   ├── index.md
│   └── src/
│       ├── lib.rs           # @id mge.factory.machine.v1
│       ├── components.rs
│       ├── systems.rs
│       └── events.rs
├── mge-factory-recipe/
│   └── (meme structure)
├── mge-factory-conveyor/
│   └── (meme structure)
└── mge-factory-logistics/
    └── (meme structure)
```

---

## 19. Resume strategique

Le Pack Factory est la brique fondamentale des jeux d'automatisation dans MGE. Il :

- Fournit 4 plugins couvrant machines, recettes, convoyeurs et logistique.
- Reste generique : aucune logique specifique a un jeu.
- S'execute en headless, en deterministe, sans rendu.
- Peut se combiner avec le Pack Tycoon pour une dimension financiere.
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
| [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md) | GCL, outils |
