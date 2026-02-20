---
name: MGE Documentation Rewrite
overview: Reecriture complete de la documentation du Miyukini Game Engine (MGE) en 10 documents structurels, passant d'une documentation game-feature-centric a une documentation engine-architecture-centric de qualite professionnelle open-engine.
todos:
  - id: reorg-folders
    content: "Reorganiser l'arborescence : creer reference/ et legacy/, deplacer les docs existantes"
    status: completed
  - id: doc-01-fondateur
    content: Ecrire MGE - Document Fondateur.md (vision, philosophie, positionnement, 6 piliers, role COG)
    status: completed
  - id: doc-02-architecture
    content: Ecrire MGE - Architecture Generale.md (5 couches, diagrammes ASCII, responsabilites, flux)
    status: completed
  - id: doc-03-core
    content: Ecrire MGE - Core Specification Technique.md (Engine, World, Scheduler, EventBus, RNG, Time)
    status: completed
  - id: doc-04-plugin
    content: Ecrire MGE - Plugin Contract.md (trait Plugin, composants, systemes, evenements, exemples)
    status: completed
  - id: doc-05-scaling
    content: Ecrire MGE - Simulation Scaling.md (LOD comportemental, masse, budget CPU)
    status: completed
  - id: doc-06-multiplayer
    content: Ecrire MGE - Mode Multijoueur.md (standalone, host auth, client replica, MWS)
    status: completed
  - id: doc-07-cog
    content: Ecrire MGE - Integration COG.md (CogService, lancement, stop, isolation)
    status: completed
  - id: doc-08-mscm
    content: Ecrire MGE - MSCM MIP Governance.md (balisage, politique ID, integrite, exemples)
    status: completed
  - id: doc-09-perf
    content: Ecrire MGE - Performance Philosophy.md (SoA, batch, spatial hash, cache, profiling)
    status: completed
  - id: doc-10-roadmap
    content: Ecrire MGE - Roadmap.md (Phases 1-5 progressives)
    status: completed
  - id: rewrite-index
    content: Reecrire _index.md pour refleter la nouvelle structure
    status: completed
isProject: false
---

# Reecriture Documentation MGE -- Moteur Generaliste 2D Simulation-First

## Diagnostic de l'existant

La documentation actuelle dans `docs/Miyukini_Game_Engine/` est **orientee features de jeu** (270 points en 24 categories : combat, loot, quetes, economie...). Elle decrit ce qu'un jeu comme Allumina a besoin, pas ce que le moteur EST en tant qu'architecture logicielle. Il n'existe **aucun crate moteur** (`mge-core`, `mge-plugins`, etc.) -- seulement une demo pathfinding dans `demos/mge-pathfinding-labyrinthe/`.

Ce qui doit changer :

- Les docs actuelles restent valides comme specs de features de jeu (Allumina), mais ne definissent pas l'architecture du moteur
- Il faut creer une couche documentaire engine-architecture au-dessus
- Le document `Miyukini - Moteur Jeux et Central Launcher.md` est un bon point de depart mais trop superficiel
- La `Reference Commune` (types Vec2, Rect, etc.) reste utile mais sera referencee depuis les nouveaux docs

## Arborescence cible

```
docs/Miyukini_Game_Engine/
  _index.md                                          # REWRITE - Index complet MGE
  MGE - Document Fondateur.md                        # NEW - [1] Vision, philosophie, positionnement
  MGE - Architecture Generale.md                     # NEW - [2] Couches, diagrammes, responsabilites
  MGE - Core Specification Technique.md              # NEW - [3] mge-core : Engine, World, Scheduler, EventBus, RNG, Time
  MGE - Plugin Contract.md                           # NEW - [4] trait Plugin, composants, systemes, evenements
  MGE - Simulation Scaling.md                        # NEW - [5] LOD comportemental, masse, budget CPU
  MGE - Mode Multijoueur.md                          # NEW - [6] Standalone, host auth, client replica, MWS
  MGE - Integration COG.md                           # NEW - [7] CogService, lancement, stop, isolation
  MGE - MSCM MIP Governance.md                       # NEW - [8] Balisage MSCM, MIP, exemples
  MGE - Performance Philosophy.md                    # NEW - [9] SoA, batch, spatial hash, cache
  MGE - Roadmap.md                                   # NEW - [10] Phases 1-5 progressives

  # Docs existantes conservees (deplacees dans reference/)
  reference/
    MGE - Reference Commune.md                       # KEEP (types, coordonnees, glossaire moteur)
    MGE - Parametres Deplacement Entite.md            # KEEP
    MGE - Hitbox et Collisions - Reference.md         # KEEP
    MGE - Pathfinding Collisions - Guide Entites Groupes.md  # KEEP
    Miyukini - Moteur Jeux et Central Launcher.md     # MOVE (doc d'origine, conservee comme reference)

  # Legacy -- specs features de jeu (restent intactes)
  legacy/
    MGE - Miyukini Game Engine - Reference Technique.md  # MOVE (270 points features)
    points/                                              # MOVE (24 categories de points)
    scripts/                                             # MOVE
```

## Contenu des 10 documents

### [1] Document Fondateur (`MGE - Document Fondateur.md`)

- Vision : moteur generaliste 2D simulation-first
- Positionnement vs Godot (tout-en-un, GDScript), Bevy (ECS pur, rendering-first), Unity (proprietaire, lourd)
- Difference MGE : microkernel, deterministe, CPU-aware, headless-capable, plugin-driven
- Philosophie fondatrice (6 piliers) : Simulation-first, Deterministic-first, Microkernel, Plugin-driven, CPU-aware, Headless-capable
- Role dans l'ecosysteme Miyukini COG (jeu = Service, moteur = lib, Central = launcher)
- Alignement LOI-1 a LOI-8

### [2] Architecture Generale (`MGE - Architecture Generale.md`)

- 5 couches avec diagrammes ASCII :
  - `mge-core` -- microkernel (Engine, World, Scheduler, EventBus, RNG, Time)
  - `mge-plugins` -- plugins officiels (physics, render, input, audio, ai, network)
  - `mge-render` -- abstraction rendu (headless-safe, backend-agnostic)
  - `mge-cog-bridge` -- integration COG (CogService, Cores, MWS)
  - Game layer -- code jeu (Allumina, etc.)
- Diagramme de flux : boot -> plugin registration -> game loop -> shutdown
- Responsabilites precises de chaque couche
- Regles de dependance (game -> plugins -> core, jamais l'inverse)

### [3] Core Specification Technique (`MGE - Core Specification Technique.md`)

- **Engine** : cycle de vie (init -> run -> shutdown), tick, seed RNG, config
- **World** : stockage entites (SoA), composants (typemap), archetypes, queries
- **Scheduler** : ordre deterministe, systemes ordonnables, budget CPU par frame, profiling hooks
- **EventBus** : communication inter-plugins, typed events, broadcast/targeted, buffer double
- **RNG deterministe** : seed globale, seed par entite, reproductibilite totale
- **Gestion du temps** : fixed timestep, delta time, time scale, pause

### [4] Plugin Contract (`MGE - Plugin Contract.md`)

- Trait `Plugin` : `fn build(&self, engine: &mut Engine)`
- Declaration de composants : `engine.register_component::<T>()`
- Enregistrement de systemes : `engine.add_system(Phase, system_fn)`
- Ecoute d'evenements : `engine.subscribe::<E>(handler)`
- Publication d'evenements : `engine.emit(event)`
- Dependances entre plugins
- Exemples concrets

### [5] Simulation Scaling (`MGE - Simulation Scaling.md`)

- LOD comportemental : 3 niveaux (Full, Reduced, Sleep)
- Gestion de masse : batching, spatial partitioning
- Desactivation adaptative : entites hors range = dormantes
- Budget CPU : frame budget, system budgets, overflow handling
- Cibles : 10k entites simulees, 100k dormantes

### [6] Mode Multijoueur (`MGE - Mode Multijoueur.md`)

- Standalone (solo offline)
- Host authoritative (COG heberge, autorite sur le state)
- Client replica (COG rejoint, recoit snapshots)
- Snapshot / delta sync
- Pas de lockstep obligatoire
- Compatible MWS (Lobbys, Permis de circulation, accord d'hote)
- Protocole transport au-dessus du tunnel MWS

### [7] Integration COG (`MGE - Integration COG.md`)

- Trait `CogService` : `start()`, `stop()`, `status()`, `config()`
- `GameRuntime` impl CogService
- Lancement depuis Central (exe separe, `Command::new`)
- Communication IPC (events, status)
- Isolation (processus separe, pas de shared memory directe)
- Dependances Cores : KindMother (persistance), StrongFather (autorisation), MWS (reseau)

### [8] MSCM & MIP Governance (`MGE - MSCM MIP Governance.md`)

- Obligations MSCM : tout module public balise (`@id`, `@do`, `@role`, `@layer`, `@human`)
- Politique d'ID : `mge.core.`*, `mge.plugin.`*, `mge.render.*`, `mge.cog.*`
- Regles d'integrite : unique, pas d'orphelin, pas de cycle
- Structure attendue des blocs (exemples dans mge-core)
- Integration au pipeline MIP (scan -> parse -> index)
- Verification MWS (Phase B : blocs MIP pour attestation)

### [9] Performance Philosophy (`MGE - Performance Philosophy.md`)

- SoA vs AoS : pourquoi SoA pour les composants
- Batch processing : systemes itèrent par archetype
- Spatial hashing : grille spatiale pour queries de proximite
- Cache locality : donnees contigues, pas de pointeurs indirects
- Pas de dynamic dispatch inutile : generics + monomorphisation
- No hidden allocations : arenas, pools, pre-allocation
- Profiling : hooks dans le scheduler, metriques par systeme

### [10] Roadmap (`MGE - Roadmap.md`)

- **Phase 1** -- Microkernel minimal : Engine, World, Scheduler, EventBus, RNG, Time, Plugin trait
- **Phase 2** -- Plugins de base : physics, render (minifb/wgpu), input, audio stub
- **Phase 3** -- Profiling avance : CPU budget, system metrics, LOD comportemental
- **Phase 4** -- Multijoueur : snapshot/delta, host auth, client replica, bridge MWS
- **Phase 5** -- Outils dev tiers : API publique stable, documentation, exemples, templates

## Conventions respectees

- Nomenclature docs : `MGE - Sujet.md` ([miyukini-docs skill](docs/Miyukini_Game_Engine/))
- Terminologie officielle : glossaire Miyukini (Core, Operateur, Outil, Service, COG, MWS, etc.)
- Balisage MSCM dans les exemples de code
- Alignement LOI-1 a LOI-8
- Structure standard Miyukini (H1, Contexte, Portee, contenu oriente action)
- Pas d'accents dans les noms de fichiers

## Ce qui n'est PAS fait

- Aucun code d'implementation (pas de crates/)
- Aucune reecriture des docs Allumina
- Les docs existantes (270 points, Reference Commune, etc.) sont **conservees** et deplacees dans `reference/` ou `legacy/`
- Pas d'integration directe de Bevy au core
- Pas de moteur monolithique

