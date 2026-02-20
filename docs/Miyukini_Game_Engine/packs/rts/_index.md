# MGE — Pack RTS

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  
**Couche** : Layer 2 (Genre Pack)  
**Repertoire** : `mge/crates/rts/`  
**Nombre de crates** : 8  

---

## 1. Contexte

Le Pack RTS gere les mecaniques fondamentales des jeux de strategie en temps reel : selection d'unites, files de production, recolte et stockage de ressources, placement et construction de batiments, ordres et IA d'unites, minimap, brouillard de guerre et arbre technologique. Il s'appuie sur le Core Universal Pack (spatial, input) et sur le Pack Social (factions).

Tous les crates sont scaffoldes (v0.1.0). Les composants, systemes et evenements decrits dans les fichiers plugin constituent la specification d'implementation cible.

---

## 2. Portee

- **Types de jeux** : RTS classique (Age of Empires, StarCraft, Command & Conquer), tower defense, MOBA simplifie.
- **Hors portee** : Logique specifique a un jeu, rendu, audio, reseau.
- **Audience** : Developpeurs moteur, developpeurs de contenu, LLM.
- **Prerequis** : Kernel Layer 0 (mge-ecs, mge-event). Core Universal Pack (spatial, input). Pack Social (factions).

---

## 3. Vision

Le Pack RTS est un ensemble de plugins simulation-first. Chaque plugin :

- Fournit des composants (donnees pures) et des systemes (1 fn = 1 effet).
- Ne contient aucune logique de jeu specifique.
- S'execute en headless sans rendu.
- Produit un comportement deterministe a seed et input identiques.
- Expose ses parametres via GCL pour configuration sans recompilation.

---

## 4. Architecture globale

```
mge/crates/rts/
├── mge-rts-selection/      # Selection multiple, box, groupements de controle
├── mge-rts-production/     # Files de production, build, annulation
├── mge-rts-resource/       # Recolte, depots, stockage
├── mge-rts-building/       # Placement, construction, demolition
├── mge-rts-unit-ai/        # Ordres, pathfinding groupe
├── mge-rts-minimap/        # Vue reduite, icones, pings
├── mge-rts-fog-of-war/     # Visibilite, brouillard, vision partagee
└── mge-rts-tech/           # Arbre technologique, recherches
```

### Graphe de dependances intra-pack

```
mge-rts-unit-ai ──► mge-rts-selection
mge-rts-production ──► mge-rts-resource
mge-rts-building ──► mge-rts-resource
mge-rts-tech ──► mge-rts-resource
mge-rts-fog-of-war ──► (aucune dep intra-pack)
mge-rts-minimap ──► (aucune dep intra-pack)
```

Crates feuilles (sans dependance intra-pack) : `mge-rts-selection`, `mge-rts-resource`, `mge-rts-minimap`, `mge-rts-fog-of-war`.

---

## 5. Sous-packs

Aucun. Les 8 crates forment un seul pack plat.

---

## 6. Liste des plugins

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 1 | `mge-rts-selection` | `mge.rts.selection.v1` | [mge-rts-selection.md](mge-rts-selection.md) | Selection multiple, box selection, groupements de controle |
| 2 | `mge-rts-production` | `mge.rts.production.v1` | [mge-rts-production.md](mge-rts-production.md) | Files de production, temps de build, annulation |
| 3 | `mge-rts-resource` | `mge.rts.resource.v1` | [mge-rts-resource.md](mge-rts-resource.md) | Recolte, depots, stockage, types de ressources |
| 4 | `mge-rts-building` | `mge.rts.building.v1` | [mge-rts-building.md](mge-rts-building.md) | Placement, construction progressive, demolition |
| 5 | `mge-rts-unit-ai` | `mge.rts.unit-ai.v1` | [mge-rts-unit-ai.md](mge-rts-unit-ai.md) | Ordres, file d'ordres, pathfinding groupe |
| 6 | `mge-rts-minimap` | `mge.rts.minimap.v1` | [mge-rts-minimap.md](mge-rts-minimap.md) | Vue reduite, icones, pings |
| 7 | `mge-rts-fog-of-war` | `mge.rts.fog-of-war.v1` | [mge-rts-fog-of-war.md](mge-rts-fog-of-war.md) | Visibilite, brouillard, vision partagee |
| 8 | `mge-rts-tech` | `mge.rts.tech.v1` | [mge-rts-tech.md](mge-rts-tech.md) | Arbre technologique, recherches, prerequis |

---

## 7. Composants cles (resume)

| Plugin | Composants runtime | Composants donnees statiques |
|--------|-------------------|------------------------------|
| selection | Selection, SelectionBox, ControlGroup | aucun |
| production | ProductionQueue, ProductionEntry, Producer | aucun |
| resource | ResourceNode, ResourceCarrier, ResourceDepot | aucun |
| building | Building, BuildSite, Footprint | aucun |
| unit-ai | OrderQueue, UnitOrder, GroupMovement | aucun |
| minimap | MinimapEntry, MinimapPing | aucun |
| fog-of-war | VisionSource, FogTile, FogGrid | aucun |
| tech | TechNode, TechTree, ResearchQueue | aucun |

---

## 8. Systemes cles (resume)

| Phase | Plugin | Systemes |
|-------|--------|----------|
| 1100-1103 | selection | process_box_selection, update_selection, assign_control_group, recall_control_group |
| 1110-1113 | production | tick_production, start_production, cancel_production, complete_production |
| 1120-1123 | resource | harvest_resource, deposit_resource, update_resource_node, check_resource_depletion |
| 1130-1133 | building | advance_construction, place_building, complete_building, demolish_building |
| 1140-1143 | unit-ai | process_orders, execute_current_order, update_group_pathfinding, check_order_completion |
| 1150-1152 | minimap | update_minimap_entries, tick_minimap_pings, remove_expired_pings |
| 1160-1163 | fog-of-war | update_vision_sources, compute_fog_grid, apply_fog_visibility, share_team_vision |
| 1170-1173 | tech | tick_research, start_research, complete_research, unlock_tech_nodes |

**Ordre d'execution** : selection (1100) → production (1110) → resource (1120) → building (1130) → unit-ai (1140) → minimap (1150) → fog-of-war (1160) → tech (1170).

**Justification** : la selection est traitee en premier car elle capture l'input joueur. La production et les ressources precedent le building qui les consomme. L'IA d'unite agit apres les ordres. La minimap et le brouillard sont des couches de visualisation. La tech est en dernier car ses effets s'appliquent au tick suivant.

**Total** : 31 systemes.

---

## 9. Evenements cles (resume)

| Plugin | Requests (entree) | Events (sortie) |
|--------|-------------------|------------------|
| selection | (aucun, ecriture directe) | SelectionChanged, ControlGroupAssigned, ControlGroupRecalled |
| production | StartProductionRequest, CancelProductionRequest | ProductionStarted, ProductionCompleted, ProductionCancelled, QueueFull |
| resource | (aucun, ecriture directe) | ResourceHarvested, ResourceDeposited, ResourceDepleted, InsufficientResources |
| building | PlaceBuildingRequest | BuildingPlaced, ConstructionProgress, BuildingCompleted, BuildingDemolished |
| unit-ai | OrderRequest | OrderIssued, OrderCompleted, OrderFailed, GroupArrived |
| minimap | PingRequest | MinimapPinged |
| fog-of-war | (aucun, lit les composants) | AreaRevealed, AreaHidden, EnemySpotted |
| tech | StartResearchRequest | ResearchStarted, ResearchCompleted, TechUnlocked, ResearchCancelled |

**Total** : 5 requests + 27 events = 32 evenements.

---

## 10. Dependances

### Dependances vers Kernel (Layer 0)

| Crate | Depend de |
|-------|-----------|
| Tous les 8 crates | `mge-ecs`, `mge-event` |

### Dependances inter-pack

| Crate | Depend de |
|-------|-----------|
| selection, unit-ai, building, minimap | Core Universal (`mge-plugin-spatial`) |
| selection | Core Universal (`mge-plugin-input`) |
| fog-of-war | Pack Social (`mge-social-faction`) |

### Dependances intra-pack

| Crate | Depend de |
|-------|-----------|
| `mge-rts-unit-ai` | `mge-rts-selection` |
| `mge-rts-production` | `mge-rts-resource` |
| `mge-rts-building` | `mge-rts-resource` |
| `mge-rts-tech` | `mge-rts-resource` |

### Dependances externes (aucune)

Le Pack RTS n'a aucune dependance vers des crates externes.

---

## 11. Interaction avec GCL

Le GCL (Game Composition Layer) configure les plugins RTS sans recompilation.

**Parametres exposables :**

- Taille max de la box de selection, nombre de control groups
- Vitesse de production, taille de queue
- Taux de recolte, capacite des depots
- Vitesse de construction, HP des batiments
- Rayon de vision, taille grille brouillard
- Duree des recherches, couts

Le GCL ne modifie pas la structure des composants. Il parametre les systemes.

---

## 12. Interaction avec autres packs

| Pack dependant | Crates RTS utilises | Usage |
|----------------|---------------------|-------|
| **Grand Strategy** | building, resource | Provinces et economie |

Le Pack RTS depend de :
- **Core Universal** : positions spatiales, input joueur
- **Pack Social** : factions pour le brouillard de guerre et la diplomatie

---

## 13. Contraintes determinisme

| Contrainte | Detail |
|------------|--------|
| **Pas de float non deterministe** | Utiliser operations deterministes, pas de NaN |
| **Pas de HashMap order-dependent** | Iteration ordonnee si necessaire |
| **Seed RNG** | fog-of-war et unit-ai utilisent le RNG kernel (mge-rng) |
| **Pas de thread-local** | Aucun etat cache |
| **Pas de static mut** | Interdit par la norme AI-Native |

---

## 14. Contraintes performance

| Contrainte | Detail |
|------------|--------|
| **Hot path** | fog-of-war (grille), selection (chaque frame), resource (recolte continue) |
| **Budget cible** | < 4ms pour 2000 unites et 500 batiments a 30 FPS |
| **Pas de dynamic dispatch** | Dans le hot path |
| **SoA storage** | Composants stockes en SoA via mge-ecs |
| **Pas d'allocation** | Dans les systemes hot path (pre-allouer) |

---

## 15. Limites v1

| Limite | Raison |
|--------|--------|
| Pas de multi-selection cross-screen | Un seul viewport supporte |
| Pas de production en parallele | 1 queue par producteur |
| Pas de marche de ressources | Echange direct via depots |
| Pas de batiments modulaires | Footprint fixe |
| Pas de pathfinding avance (flow field) | A* groupe uniquement |
| Pas de brouillard 3D | Grille 2D uniquement |

---

## 16. Extensions possibles v2

| Extension | Description |
|-----------|-------------|
| Multi-viewport | Selection sur plusieurs ecrans |
| Production parallele | Files multiples par batiment |
| Marche / commerce | Echange de ressources entre joueurs |
| Batiments modulaires | Extensions, upgrades structurels |
| Flow field pathfinding | Navigation fluide grandes armees |
| Brouillard 3D | Support terrain hauteurs |
| Replay | Enregistrement et relecture des ordres |

---

## 17. Exemple d'assemblage

### Minimal (headless, selection + resource)

```rust
let mut engine = Engine::new(EngineConfig::default());
engine.add_plugin(MgePluginSpatial::default());
engine.add_plugin(MgeRtsSelectionPlugin);
engine.add_plugin(MgeRtsResourcePlugin);
engine.build();
```

### Complet (RTS)

```rust
let mut engine = Engine::new(EngineConfig::default());
// Core Universal
engine.add_plugin(MgePluginSpatial::default());
engine.add_plugin(MgePluginInput::default());
// Pack Social
engine.add_plugin(MgeSocialFactionPlugin);
// Pack RTS
engine.add_plugin(MgeRtsSelectionPlugin);
engine.add_plugin(MgeRtsProductionPlugin);
engine.add_plugin(MgeRtsResourcePlugin);
engine.add_plugin(MgeRtsBuildingPlugin);
engine.add_plugin(MgeRtsUnitAiPlugin);
engine.add_plugin(MgeRtsMinimapPlugin);
engine.add_plugin(MgeRtsFogOfWarPlugin);
engine.add_plugin(MgeRtsTechPlugin);
engine.build();
```

---

## 18. Organisation des crates

```
mge/crates/rts/
├── mge-rts-selection/
│   ├── Cargo.toml
│   ├── index.md
│   └── src/
│       ├── lib.rs           # @id mge.rts.selection.v1
│       ├── components.rs
│       ├── systems.rs
│       └── events.rs
├── mge-rts-production/
│   └── (meme structure)
├── mge-rts-resource/
│   └── (meme structure)
├── mge-rts-building/
│   └── (meme structure)
├── mge-rts-unit-ai/
│   └── (meme structure)
├── mge-rts-minimap/
│   └── (meme structure)
├── mge-rts-fog-of-war/
│   └── (meme structure)
└── mge-rts-tech/
    └── (meme structure)
```

---

## 19. Resume strategique

Le Pack RTS est la brique fondamentale des jeux de strategie en temps reel dans MGE. Il :

- Fournit 8 plugins couvrant selection, production, ressources, batiments, IA d'unite, minimap, brouillard de guerre et technologie.
- Reste generique : aucune logique specifique a un jeu.
- S'execute en headless, en deterministe, sans rendu.
- Depend du Core Universal pour le spatial/input et du Pack Social pour les factions.
- Expose ses parametres via GCL pour iteration rapide.
- Respecte strictement la norme AI-Native (MSCM, 1 fn = 1 effet, max 30 lignes, pas de hidden state).

Les 8 crates sont scaffoldes (v0.1.0). L'implementation suit les specifications des fichiers plugin individuels.

---

## References

| Document | Role |
|----------|------|
| [MGE - Pack Architecture](../MGE%20-%20Pack%20Architecture.md) | Couches, composition |
| [MGE - Architecture Generale](../MGE%20-%20Architecture%20Generale.md) | Couches globales |
| [MGE - Plugin Contract](../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
| [MGE - AI-Native Writing Standard v1](../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md) | GCL, outils |
