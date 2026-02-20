# MGE — Roadmap

Phases progressives de développement du Miyukini Game Engine : du microkernel minimal aux outils pour développeurs tiers.

## Contexte

La roadmap du MGE est découpée en 5 phases réalistes. Chaque phase livre un increment fonctionnel utilisable. Les phases sont séquentielles ; une phase repose sur la précédente.

## Portée / Scope

- **Applicable à :** Planification, priorisation, implémentation.
- **Audience :** Équipe projet, contributeurs, parties prenantes.
- **Statut :** Document de planification.

---

## Phase 1 — Microkernel minimal

**Objectif** : Cœur exécutable sans plugins métier. Boucle orchestrée par Game Runtime : engine.tick() (simulation pure). World, Scheduler, EventQueue, RNG, Time.

### Livrables

| Composant | Description |
|-----------|-------------|
| **Engine** | Cycle de vie (new, add_plugin, build, run, stop), config, seed. |
| **World** | EntityId, spawn/despawn, insert composants, queries par archetype. |
| **Scheduler** | Phases (Input, Physics, Logic, PreRender, Render, PostRender), ordre déterministe. |
| **EventQueue** | emit, lecture explicite (iter::<E>), buffer double. Pas de subscribe/callback. |
| **RNG** | Seed global, génération déterministe (ex. PCG, xoshiro). |
| **Time** | Delta time, fixed timestep optionnel, time scale, pause. |
| **Plugin trait** | build(engine), name(), dependencies(). |
| **Crate mge-core** | Structure crate, tests unitaires, MSCM sur modules publics. |

### Critère de complétion

- Game Runtime appelle engine.tick() en boucle (sans fenêtre en headless).
- Un plugin minimal enregistre un composant et un système ; le système s'exécute chaque tick.
- Même seed → même séquence RNG.
- Documentation Core Specification Technique validée par l'implémentation.

### Durée indicative

4 à 6 semaines.

---

## Phase 2 — Plugins de base

**Objectif** : Plugins physics, render, input, audio (stub). Premier jeu jouable (démo).

### Livrables

| Composant | Description |
|-----------|-------------|
| **mge-plugin-physics** | Position, Velocity, Collider (AABB), résolution collisions, déplacement. |
| **mge-render** | Abstraction backend (trait RenderBackend), headless (no-op). |
| **mge-plugin-render** | Intégration render dans la boucle, caméra, sprites (minifb ou wgpu basique). |
| **mge-plugin-input** | Clavier, souris, mapping vers événements. |
| **mge-plugin-audio** | Stub (no-op) ou intégration minimaliste (ex. rodio). |
| **Démo** | Carré contrôlable au clavier, collisions avec murs, sprite affiché. |

### Critère de complétion

- Démo jouable : déplacement, collisions, affichage.
- Mode headless inchangé (simulation sans rendu).
- Au moins un backend rendu fonctionnel (minifb ou wgpu).

### Durée indicative

6 à 8 semaines.

---

## Phase 3 — Profiling avancé

**Objectif** : Budget CPU, métriques par système, LOD comportemental. Cibles de performance atteintes.

### Livrables

| Composant | Description |
|-----------|-------------|
| **Frame budget** | Config frame_budget_ms, overflow handling (continue, skip, scale down). |
| **System budget** | Optionnel, temps max par système. |
| **Profiling hooks** | Avant/après chaque système, métriques (temps, nb entités). |
| **LOD comportemental** | Full, Reduced, Sleep ; transitions selon distance/zones. |
| **Spatial hashing** | Grille spatiale pour broadphase, culling. |
| **Scaling config** | active_radius, loaded_radius, spatial_cell_size. |

### Critère de complétion

- 10k entités Full simulées à 60 FPS (hardware moyen).
- 100k entités Sleep présentes dans le World sans impact.
- Overlay debug affichant les temps par système (optionnel).
- Documentation Simulation Scaling validée.

### Durée indicative

4 à 6 semaines.

---

## Phase 4 — Multijoueur

**Objectif** : Snapshot/delta, host authoritative, client replica, bridge MWS. Jeu multijoueur jouable.

### Livrables

| Composant | Description |
|-----------|-------------|
| **mge-plugin-network** | Connexion Lobby (MWS), envoi inputs, réception snapshot/delta. |
| **Réplication** | Sérialisation composants réplicables, application delta au World. |
| **Host logic** | Simulation autoritaire, génération snapshot/delta. |
| **Client logic** | Réception état, mise à jour World replica, envoi inputs. |
| **mge-cog-bridge** | CogService, GameRuntime, intégration Central (lancement exe). |
| **Démo multijoueur** | 2 joueurs, sync position/mouvement, hôte + client. |

### Critère de complétion

- Deux instances (hôte + client) synchronisées via MWS (Lobby, accord d'hôte).
- Snapshot ou delta fonctionnel ; pas de lockstep obligatoire.
- Jeu lancé depuis Central (si Central disponible).
- Documentation Mode Multijoueur et Intégration COG validée.

### Durée indicative

8 à 12 semaines.

---

## Phase 5 — Packs genre (16 packs)

**Objectif** : Créer les packs genre par vagues, documenter chaque pack, livrer des exemples par pack.

### Vagues d'implémentation

| Vague | Packs | Priorité |
|-------|-------|----------|
| **Vague 3a** | RPG, Social, Puzzle | Matures en design |
| **Vague 3b** | Massive Battle, RTS, Grand Strategy, Sandbox | Systèmes stratégiques complexes |
| **Vague 3c** | Platformer, Shooter, Roguelike, Racing | Action / arcade |
| **Vague 3d** | Factory, Idle, Tycoon, Visual Novel, TCG | Économie / narrative |

### Livrables par vague

- Crates scaffoldés avec structure AI-Native (mod.rs, components, systems, events).
- Documentation pack (`docs/Miyukini_Game_Engine/packs/`).
- index.md par crate avec AI-Native Score.

### Référence

Voir [MGE - Pack Architecture](./MGE%20-%20Pack%20Architecture.md) pour la liste complète des 16 packs et leurs dépendances.

---

## Phase 6 — Outils dev tiers

**Objectif** : API publique stable, documentation complète, exemples, templates. MGE prêt pour des développeurs externes.

### Livrables

| Composant | Description |
|-----------|-------------|
| **API stable** | Semver, changelog, breaking changes documentés. |
| **Documentation** | Rustdoc complet, guides (quickstart, plugin creation, multijoueur). |
| **Exemples** | minimal_game, rpg_demo, rts_demo, sandbox_demo, allumina_prototype. |
| **Templates** | crate template pour nouveau jeu MGE, template plugin. |
| **Pipeline MIP** | Génération index MIP incluant crates MGE, intégration CI. |
| **Checklist conformité** | MSCM sur tous les modules, vérification MWS Phase B (si applicable). |

### Critère de complétion

- Un développeur tiers peut créer un jeu MGE en suivant la doc.
- Un plugin tiers peut être développé et intégré.
- Index MIP à jour, pas d'erreur d'intégrité.
- Roadmap complète documentée et publiée.

### Durée indicative

6 à 8 semaines.

---

## Vue d'ensemble

```
Phase 1      Phase 2       Phase 3         Phase 4         Phase 5          Phase 6
(4-6 sem)   (6-8 sem)    (4-6 sem)       (8-12 sem)    (Packs genre)     (6-8 sem)
    │            │            │               │              │                │
    ▼            ▼            ▼               ▼              ▼                ▼
┌───────┐   ┌───────┐   ┌─────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│Micro- │   │Plugins│   │Profiling│    │ Multi-   │    │ 16 Packs │    │ Outils   │
│kernel │──►│  base │──►│ avancé  │───►│ joueur   │───►│  genre   │───►│ dev tiers│
└───────┘   └───────┘   └─────────┘    └──────────┘    └──────────┘    └──────────┘
    │            │            │               │              │                │
 Headless    Démo 2D      10k entités     Host auth      RPG, RTS,         API stable
 Boucle      Physics+     LOD, budget     MWS bridge     Sandbox...       Docs, examples
```

La Phase 5 (Packs genre) s'exécute en parallèle ou après Phase 4 selon les ressources.

---

## Dépendances externes

- **Phase 1-2** : Aucune dépendance MWS ou Central critique ; solo uniquement.
- **Phase 3** : Aucune.
- **Phase 4** : MWS (MiyuWebwayParticipant, Lobbys), éventuellement Central pour lancement.
- **Phase 5** : Aucune ; packs genre indépendants.
- **Phase 6** : Pipeline MIP, CI.

---

## Références

| Document | Rôle |
|----------|------|
| [MGE - Document Fondateur](./MGE%20-%20Document%20Fondateur.md) | Vision, philosophie. |
| [MGE - Core Specification Technique](./MGE%20-%20Core%20Specification%20Technique.md) | Phase 1. |
| [MGE - Plugin Contract](./MGE%20-%20Plugin%20Contract.md) | Phase 2. |
| [MGE - Simulation Scaling](./MGE%20-%20Simulation%20Scaling.md) | Phase 3. |
| [MGE - Mode Multijoueur](./MGE%20-%20Mode%20Multijoueur.md) | Phase 4. |
| [MGE - Intégration COG](./MGE%20-%20Integration%20COG.md) | Phase 4. |
| [MGE - Pack Architecture](./MGE%20-%20Pack%20Architecture.md) | Phase 5 (packs genre). |

---

**Document** : MGE — Roadmap  
**Version** : 1.1  
**Date** : 2026-02-20  
**Statut** : Document de planification
