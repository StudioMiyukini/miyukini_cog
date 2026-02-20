# Miyukini Game Engine (MGE) — Index

Documentation du moteur de jeu maison Miyukini : architecture microkernel, simulation-first, déterministe, compatible écosystème COG.

## Contexte

Le MGE est le moteur généraliste 2D de l'écosystème Miyukini COG. Il adopte une philosophie simulation-first, microkernel, plugin-driven, CPU-aware et headless-capable. Les jeux (Allumina, etc.) sont des Services Inter-COG construits sur le MGE.

## Portée / Scope

- **Applicable à :** Développement du moteur, intégration des jeux, décisions architecturales.
- **Audience :** Développeurs moteur, développeurs tiers, LLM, architectes.
- **Statut :** Index normatif.

---

## Documents fondateurs et architecture

| Document | Rôle |
|----------|------|
| [MGE - Document Fondateur](./MGE%20-%20Document%20Fondateur.md) | Vision, philosophie, positionnement, 6 piliers, rôle COG |
| [MGE - Architecture Générale](./MGE%20-%20Architecture%20Generale.md) | Couches (core, plugins, render, cog, game), kernel éclaté, packs |
| [MGE - Pack Architecture](./MGE%20-%20Pack%20Architecture.md) | Philosophie packs, 16 packs genre, dépendances, composition |
| [MGE - Platform Tooling Layer v1](./MGE%20-%20Platform%20Tooling%20Layer%20v1.md) | Outils édition (8 tools), bacs, flux Design → Runtime |
| [MGE - Core Specification Technique](./MGE%20-%20Core%20Specification%20Technique.md) | Engine, World, Scheduler, EventQueue, RNG, Time ; tick() uniquement |
| [MGE - Plugin Contract](./MGE%20-%20Plugin%20Contract.md) | Trait Plugin, composants, systèmes, événements |

---

## Spécifications techniques

| Document | Rôle |
|----------|------|
| [MGE - Simulation Scaling](./MGE%20-%20Simulation%20Scaling.md) | LOD comportemental (Full/Reduced/Sleep), budget CPU, gestion de masse |
| [MGE - Mode Multijoueur](./MGE%20-%20Mode%20Multijoueur.md) | Standalone, host authoritative, client replica, snapshot/delta, MWS |
| [MGE - Intégration COG](./MGE%20-%20Integration%20COG.md) | CogService, lancement depuis Central, stop, isolation |
| [MGE - MSCM & MIP Governance](./MGE%20-%20MSCM%20MIP%20Governance.md) | Balisage MSCM, politique d'ID, intégrité, pipeline MIP |
| [MGE - Performance Philosophy](./MGE%20-%20Performance%20Philosophy.md) | SoA, batch, spatial hash, cache locality, profiling |
| [MGE - Roadmap](./MGE%20-%20Roadmap.md) | Phases 1 à 5 progressives |

---

## Références partagées

| Document | Rôle |
|----------|------|
| [MGE - Référence Commune](./reference/MGE%20-%20Reference%20Commune.md) | Types Vec2, Rect, coordonnées, glossaire moteur, cycle rendu |
| [MGE - Paramètres Déplacement Entité](./reference/MGE%20-%20Parametres%20Deplacement%20Entite.md) | Position, velocity, locomotion, orientation |
| [MGE - Hitbox et Collisions](./reference/MGE%20-%20Hitbox%20et%20Collisions%20-%20Reference.md) | Hitbox, collision, formules, MTV |
| [MGE - Pathfinding Collisions](./reference/MGE%20-%20Pathfinding%20Collisions%20-%20Guide%20Entites%20Groupes.md) | Pathfinding, coût, hitbox, groupes |
| [Miyukini - Moteur Jeux et Central Launcher](./reference/Miyukini%20-%20Moteur%20Jeux%20et%20Central%20Launcher.md) | Architecture jeux + Central, exe par jeu |

---

## Platform Tooling Layer (8 outils)

Outils d'édition autour du Kernel. Aucun outil ne modifie le runtime.

| Document | Rôle |
|----------|------|
| [MGE - Platform Tooling Layer v1](./MGE%20-%20Platform%20Tooling%20Layer%20v1.md) | Vision, flux, bacs, 8 outils |
| [Index des outils](./platform-tools/_index.md) | Data Authoring, Prefab Editor, Balance Lab, Battle Sandbox, Sprite Tool, Rule Editor, Export Pipeline, AI Assist |

---

## Packs genre (16 packs)

Documentation détaillée de chaque pack :

| Pack | Document |
|------|----------|
| RPG | [MGE - Pack RPG](./packs/MGE%20-%20Pack%20RPG.md) |
| Massive Battle | [MGE - Pack Massive Battle](./packs/MGE%20-%20Pack%20Massive%20Battle.md) |
| Social Simulation | [MGE - Pack Social Simulation](./packs/MGE%20-%20Pack%20Social%20Simulation.md) |
| RTS | [MGE - Pack RTS](./packs/MGE%20-%20Pack%20RTS.md) |
| Grand Strategy | [MGE - Pack Grand Strategy](./packs/MGE%20-%20Pack%20Grand%20Strategy.md) |
| Puzzle | [MGE - Pack Puzzle](./packs/MGE%20-%20Pack%20Puzzle.md) |
| Sandbox | [MGE - Pack Sandbox](./packs/MGE%20-%20Pack%20Sandbox.md) |
| Platformer | [MGE - Pack Platformer](./packs/MGE%20-%20Pack%20Platformer.md) |
| Shooter | [MGE - Pack Shooter](./packs/MGE%20-%20Pack%20Shooter.md) |
| Roguelike | [MGE - Pack Roguelike](./packs/MGE%20-%20Pack%20Roguelike.md) |
| Racing | [MGE - Pack Racing](./packs/MGE%20-%20Pack%20Racing.md) |
| Factory | [MGE - Pack Factory](./packs/MGE%20-%20Pack%20Factory.md) |
| Idle | [MGE - Pack Idle](./packs/MGE%20-%20Pack%20Idle.md) |
| Tycoon | [MGE - Pack Tycoon](./packs/MGE%20-%20Pack%20Tycoon.md) |
| Visual Novel | [MGE - Pack Visual Novel](./packs/MGE%20-%20Pack%20Visual%20Novel.md) |
| TCG | [MGE - Pack TCG](./packs/MGE%20-%20Pack%20TCG.md) |

---

## Legacy — Spécifications features de jeu

Les documents suivants décrivent les **capacités de jeu** (combat, loot, quêtes, etc.) plutôt que l'architecture du moteur. Ils restent valides pour les jeux comme Allumina.

| Document | Rôle |
|----------|------|
| [MGE - Référence Technique (legacy)](./legacy/MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md) | ~270 points en 24 catégories |
| [Index des points (legacy)](./legacy/points/_index.md) | Points de développement par catégorie |

---

## Services Jeux

| Service | Statut | Moteur |
|---------|--------|--------|
| **Allumina** | Concept / à développer | MGE (maison) |
| **MiyukiniSurvivor (Lord of the Castle)** | En développement | Dioxus + Blitz |

---

## Références externes

| Document | Rôle |
|----------|------|
| [MWS - Document Fondateur](../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md) | MWS : présence, découverte, transport |
| [Allumina - Document Fondateur](../services/Allumina/Allumina%20-%20Document%20Fondateur.md) | Jeu Action RPG sur MGE |

---

**Document** : MGE — Index  
**Version** : 1.0  
**Date** : 2026-02-19  
**Statut** : Index normatif
