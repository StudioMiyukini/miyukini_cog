# E10 -- Matrice MVP Sodomight -- Freeze P3

**freeze_date**: 2026-03-06
**workspace_tests**: 325 / 325 PASS
**clippy_status**: 0 warnings (`-D warnings`)

---

## Legende statut

| Code | Signification |
|------|---------------|
| IMPL | Implemente + tests unitaires verts |
| HARNESS | Accessible via debug harness / sim scenarios |
| CONTRAT | Contrat de donnees defini, implementation differee |
| POST-MVP | Hors perimetre P3, backloggue |

---

## Systemes D2 cibles -- statut par domaine

### Moteur / Runtime (E01 + E02 + E03)

| Systeme | Statut | Crate / Module | Tests |
|---------|--------|----------------|-------|
| Workspace Rust autonome | IMPL | `mge/` | -- |
| Boucle de jeu + scene bootstrap | IMPL | `mge-runtime` | 7 |
| Renderer wgpu + camera iso | IMPL | `mge-render` | 6 |
| Sprite batching + render layers | IMPL | `mge-render` | -- |
| Frame graph (terrain/entities/VFX/UI) | IMPL | `mge-render` | -- |
| VFX particules | IMPL | `mge-render` | -- |
| Eclairage + fog zones + color grading | IMPL | `mge-render` | -- |
| Quality tiers Low/Standard/High | IMPL | `mge-render` | -- |
| Device lost + swapchain recovery | IMPL | `mge-render` | -- |
| Telemetrie frame (`TelemetryRing`) | IMPL | `mge-render` | -- |
| Animation clips 8-dir + notify events | IMPL | `mge-render` | -- |
| Pipeline audio (`AudioMixer`, bus, cues) | IMPL | `mge-audio` | 1 |
| Asset baker (scan/hash/validate/pack) | IMPL | `mge-asset-baker` | 2 |
| Save/load persistence | IMPL | `mge-save` | -- |
| Proto reseau v2 (delta snapshots) | IMPL | `mge-proto` | 5 |
| Replication + interest management | IMPL | `mge-replication` | 4 |

### UI / HUD (E04)

| Systeme | Statut | Crate / Module | Tests |
|---------|--------|----------------|-------|
| Shell UI (screens, overlays, focus) | IMPL | `mge-ui` / `shell` | -- |
| HUD combat (orbs, skill bar, buffs) | IMPL | `mge-ui` / `hud` | -- |
| Tooltips (items, skills, stats) | IMPL | `mge-ui` / `tooltips` | -- |
| Inventaire (grid, equip, drag-drop) | IMPL | `mge-ui` / `inventory` | -- |
| Stash (multi-tab, transfer) | IMPL | `mge-ui` / `stash` | -- |
| Vendor (buy/sell/repair/identify/gamble) | IMPL | `mge-ui` / `vendor` | -- |
| Journal de quetes | IMPL | `mge-ui` / `quest` | -- |
| Fiche personnage (stats, resistances) | IMPL | `mge-ui` / `charsheet` | -- |
| Menus systeme (pause, video, audio) | IMPL | `mge-ui` / `menus` | -- |
| Input map (bindings, contextes, conflits) | IMPL | `mge-ui` / `input_map` | -- |
| Etats visuels (cooldown, resource gating) | IMPL | `mge-ui` / `visual_states` | -- |
| Feedback (event queue, accessibilite) | IMPL | `mge-ui` / `feedback` | -- |
| Debug panel (metriques, log, dev-only) | IMPL | `mge-ui` / `debug_panel` | -- |
| Total tests mge-ui | | | 56 |

### Gameplay -- Personnages / Stats / Skills (E05)

| Systeme | Statut | Crate / Module | Tests |
|---------|--------|----------------|-------|
| Classes jouables (5 classes D2-like) | IMPL | `mge-gameplay` | -- |
| Stats primaires (str/dex/vit/ene) | IMPL | `mge-gameplay` | -- |
| Stats derivees (HP, mana, AR, def) | IMPL | `mge-gameplay` | -- |
| Resistances + formules | IMPL | `mge-gameplay` | -- |
| Breakpoints (FHR, FCR, FBR, IAS) | IMPL | `mge-gameplay` | -- |
| Arbre de competences | IMPL | `mge-gameplay` | -- |
| Combat melee (hit/miss, damage) | IMPL | `mge-gameplay` | -- |
| Combat ranged (projectiles) | IMPL | `mge-gameplay` | -- |
| Combat magique (spells, elements) | IMPL | `mge-gameplay` | -- |
| Buffs / auras | IMPL | `mge-gameplay` | -- |
| Invocations + pieges | IMPL | `mge-gameplay` | -- |
| Cycle mort / resurrection | IMPL | `mge-gameplay` | -- |
| Total tests mge-gameplay | | | 48 |

### Items / Loot / Economie (E06)

| Systeme | Statut | Crate / Module | Tests |
|---------|--------|----------------|-------|
| Familles / raretes (Normal..Unique) | IMPL | `mge-items` / `rarity` | -- |
| Affixes (pool, weighted pick) | IMPL | `mge-items` / `affixes` | -- |
| Uniques + sets + bonus de set | IMPL | `mge-items` / `uniques` | -- |
| Sockets (insert/clear/can_add) | IMPL | `mge-items` / `sockets` | -- |
| Gemmes + runes (El..Zod) + joyaux | IMPL | `mge-items` / `socketables` | -- |
| Runewords (validation + recipes MVP) | IMPL | `mge-items` / `runewords` | -- |
| Charms (small/large/grand, mods) | IMPL | `mge-items` / `charms` | -- |
| Economie or (prix, achat, vente, rep) | IMPL | `mge-items` / `economy` | -- |
| Vendors (buy/sell/refresh/gamble) | IMPL | `mge-items` / `vendors` | -- |
| Cube / recettes (match_recipe) | IMPL | `mge-items` / `crafting` | -- |
| Tables de loot (LCG deterministe) | IMPL | `mge-items` / `loot` | -- |
| Simulateur de drops | IMPL | `mge-items` / `simulator` | -- |
| Total tests mge-items | | | 55 |

### Monde / Quetes / Contenu Acte 1 (E07)

| Systeme | Statut | Crate / Module | Tests |
|---------|--------|----------------|-------|
| Camp (services, portails, NPCs) | IMPL | `mge-world` / `camp` | -- |
| Graphe de zones Acte 1 (11 zones) | IMPL | `mge-world` / `zone` | -- |
| Bibliotheque de chunks (6 biomes) | IMPL | `mge-world` / `chunks` | -- |
| Generateur de layout (LCG) | IMPL | `mge-world` / `randomize` | -- |
| Transitions de zones (state machine) | IMPL | `mge-world` / `transition` | -- |
| Waypoints (5 Acte 1) + Town Portal | IMPL | `mge-world` / `waypoints` | -- |
| Quetes critiques (6 Acte 1) | IMPL | `mge-world` / `quests` | -- |
| Quetes secondaires (5 Acte 1) | IMPL | `mge-world` / `secondary` | -- |
| Evenements monde (6 Acte 1) | IMPL | `mge-world` / `events` | -- |
| Boss Andariel (3 phases) | IMPL | `mge-world` / `boss` | -- |
| Conditions de passage Acte 2 | IMPL | `mge-world` / `progression` | -- |
| Walkthrough validator end-to-end | IMPL | `mge-world` / `runner` | -- |
| Total tests mge-world | | | 53 |

### Monstres / Boss AI (E08 -- mge-monsters)

| Systeme | Statut | Crate / Module | Tests |
|---------|--------|----------------|-------|
| Roster monstres Acte 1 (9 types) | IMPL | `mge-monsters` / `roster` | -- |
| Variantes elite + champion packs | IMPL | `mge-monsters` / `variants` | -- |
| Machine AI (Melee/Ranged/Caster/Flee) | IMPL | `mge-monsters` / `ai` | -- |
| Scripts boss (Blood Raven + Andariel) | IMPL | `mge-monsters` / `scripts` | -- |
| Total tests mge-monsters | | | 19 |

### Modes meta D2 (E08 -- mge-meta)

| Systeme | Statut | Crate / Module | Tests |
|---------|--------|----------------|-------|
| Mercenaires (hire/kill/resurrect) | IMPL | `mge-meta` / `mercenary` | -- |
| Mode Hardcore (death flag, HoF) | IMPL | `mge-meta` / `hardcore` | -- |
| Party co-op (8 joueurs, XP split) | IMPL | `mge-meta` / `party` | -- |
| PvP consent + duels | IMPL | `mge-meta` / `pvp` | -- |
| Ladder (saisons, types, classement) | IMPL | `mge-meta` / `ladder` | -- |
| Services MMO (topology 5 services) | IMPL | `mge-meta` / `services` | -- |
| Debug harness + sim scenarios | IMPL | `mge-meta` / `harness` | -- |
| Total tests mge-meta | | | 28 |

---

## Ecarts connus -- post-MVP

| Systeme | Raison du report | Priorite |
|---------|-----------------|----------|
| Actes 2 a 5 (zones, quetes, boss) | Hors perimetre P3 | post-MVP |
| Integration GPU validee (vraie fenetre) | Requiert machine test + assets GPU | important |
| Round-trip save/load integration test | Infrastructure de test manquante | important |
| Relay reseau (MiyuWebWay) integration | Depend P4 infra | post-MVP |
| Lecture audio reelle (device output) | Requiert driver test | important |
| Contenu Acte 1 visuellement joue | Assets D2-style non integres | post-MVP |
| Modes Nightmare / Hell | Post-Acte 1 | post-MVP |
| Trading joueur a joueur | Post-P3 | post-MVP |
| Guilde / chat social en jeu | Post-P3 | post-MVP |

---

## Scenarios end-to-end valides

Les scenarios suivants sont couverts par `run_sim_scenarios()` dans `mge-meta::harness` :

1. `quest_den_of_evil_clear` -- PASS
2. `andariel_defeat` -- PASS
3. `waypoint_activation` -- PASS
4. `merc_death_resurrection` -- PASS
5. `party_xp_split` -- PASS

Le parcours structural camp -> boss est valide via `verify_act1_structure()` dans `mge-world::runner`.

---

## Totaux de couverture P3

| Crate | Tests |
|-------|-------|
| mge-runtime + support | 16 |
| mge-gameplay | 48 |
| mge-ui | 56 |
| mge-items | 55 |
| mge-world | 53 |
| mge-monsters | 19 |
| mge-meta | 28 |
| mge-proto | 5 |
| mge-replication | 4 |
| Autres (render, save, audio, baker...) | 41 |
| **Total workspace** | **325** |
