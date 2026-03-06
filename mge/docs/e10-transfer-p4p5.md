# E10 -- Dossier de transfert P3 -> P4/P5

**date**: 2026-03-06
**statut P3**: FERME
**tests**: 325 / 325 PASS, clippy -D warnings PASS

---

## Preuves de couverture P3

| Gate | Critere | Statut |
|------|---------|--------|
| G1 | Workspace compilable, lint/fmt/tests documentes | PASS |
| G2 | Runtime bootable, schemas versionnes, contrat save/load | PASS |
| G3 | Renderer, camera, sprite batching, pipeline assets | PASS |
| G4 | HUD, inventaire, vendor, quetes, menus, input | PASS |
| G5 | Classes, stats, skills, breakpoints, combat | PASS |
| G6 | Raretes, affixes, sockets, gemmes/runes, loot, economie | PASS |
| G7 | Camp + Acte 1 jouable, 11 zones, 6 quetes critiques, boss | PASS |
| G8 | Roster monstres, AI, hardcore, party, PvP, ladder, harness | PASS |
| G9 | Manifeste, package, Central integration | PASS |
| G10 | 325 tests verts, matrice freeze, equilibrage, dossier transfert | PASS |

---

## Architecture resultante

```
mge/
  crates/
    mge-runtime          -- boucle jeu, scene, autorité (LocalHost/Dedicated)
    mge-render           -- wgpu, camera iso, sprites, VFX, eclairage
    mge-audio            -- mixer, bus categories, cues
    mge-asset-baker      -- scan/hash/validate/pack/manifest
    mge-save             -- persistence save/load
    mge-proto            -- protocole reseau v2, delta snapshots
    mge-replication      -- interest management, DeltaAccumulator
    mge-gameplay         -- classes, stats, skills, combat, buffs
    mge-ui               -- shell, HUD, inventaire, feedback
    mge-items            -- items, loot, economie, cube, simulateur
    mge-world            -- zones, quetes, boss, waypoints, randomisation
    mge-monsters         -- roster, variants, AI, scripts boss
    mge-meta             -- mercs, hardcore, party, PvP, ladder, services
    mge-server-core      -- coeur serveur autoritaire
    mge-client-core      -- coeur client
    miyukini-central     -- integration Central (auth, MWS)
    mge-net              -- transport reseau
```

Toutes les crates sont independantes du monorepo `Miyukini-COG` via des paths relatifs. Le workspace `mge/` peut etre extrait et compile standalone.

---

## Contrats stables -- ne pas casser en P4

### Proto reseau (mge-proto)
- `PROTOCOL_VERSION = 2`
- `SnapshotEnvelope` avec `scene_id: String, tick: u64, version: u32`
- `DeltaSnapshot` + `DeltaField { field_id: u16, value_bytes: Vec<u8> }`
- `ClientCommand` (10 variantes), `ServerEvent` (8 variantes)
- `SnapshotEnvelope::is_compatible` utilise pour rejection de version

### Save / load
- Format versionne defini dans `mge-save`
- Migration forward-only; jamais downgrade

### Simulation deterministe
- LCG : `state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407)`
- Seed injectee par le serveur autoritaire, JAMAIS derivee du client
- `generate_layout`, `LootTable::roll_drop`, `simulate_drops` utilisant ce LCG

### Authority boundary
- `AuthorityMode::LocalHost` (mono-process) vs `AuthorityMode::Dedicated` (client/server split)
- Le coeur simulation (`mge-world`, `mge-gameplay`, `mge-items`) ne doit pas contenir de I/O reseau directe

---

## Backlog residuel classe

### Bloquant avant jouabilite reelle

| Item | Impact | Crate cible |
|------|--------|-------------|
| Integration GPU validee (vraie fenetre, test device) | Render non valide hors CI | mge-render |
| Round-trip save/load integration test | Persistence non prouvee | mge-save |
| Seed LCG injectee par serveur en production | Securite determinisme | mge-proto / mge-world |
| Assets D2-style integres (sprites, tileset, sons) | Jeu non visuellement jouable | mge-asset-baker |

### Important (P4)

| Item | Impact | Crate cible |
|------|--------|-------------|
| HP boss scale avec party size | Equilibrage multijoueur | mge-world / mge-meta |
| Level scaling mercenaires | Stats statiques en P3 | mge-meta |
| Bonus synergie XP party | Mode split trop punitif a 8 | mge-meta |
| Integration relay reseau (MiyuWebWay) | Multijoueur reel | mge-net |
| Lecture audio device output | Audio silencieux en P3 | mge-audio |
| Smoke tests package installe | Qualite livraison | outils |
| Playtest end-to-end avec assets reels | Validation jouabilite | -- |

### Post-MVP (P5+)

| Item | Raison |
|------|--------|
| Actes 2 a 5 (contenu, zones, boss) | Hors perimetre P3 |
| Modes Nightmare / Hell | Depend Acte 2+ |
| Trading joueur a joueur | Feature sociale post-P3 |
| Guilde / chat en jeu | Feature sociale post-P3 |
| Ladder reset automatique | Infra ops post-P3 |
| Mode Expansion (Druide, Assassin) | Post-lancement |

---

## Risques et mitigations identifies

| Risque | Probabilite | Impact | Mitigation |
|--------|-------------|--------|------------|
| Performance GPU non validee (wgpu) | Moyen | Elevee | Valider sur machine cible des assets disponibles |
| Determinisme LCG compromis par race condition reseau | Faible | Elevee | Seed unique par zone-session, injectee serveur |
| Compatibilite save si schema change en P4 | Moyen | Elevee | Versionner + migration obligatoire avant toute modif schema |
| Regression lors integration assets D2-style | Moyen | Moyenne | Smoke tests pipeline baker + tests atlas/atlas-stream |
| Scope creep Acte 2 demande par P4 | Eleve | Moyenne | Geler perimetre; tout Acte 2+ passe par spec P5 |

---

## Recommandations equipe P4

1. **Commencer par les bloquants** : integration GPU + save round-trip + assets avant tout ajout de feature.
2. **Ne pas modifier `mge-proto` sans bump de `PROTOCOL_VERSION`** et test de compatibilite `is_compatible`.
3. **Le workspace `mge/` reste autonome** : eviter d'importer des crates `apps/central` ou `crates/miyucloud` dans `mge/`.
4. **Tests de regression obligatoires** : tout PR sur `mge-world`, `mge-gameplay` ou `mge-items` doit maintenir les 325 tests existants.
5. **Equilibrage via harness** : utiliser `act1_debug_harness(AuthorityMode::Dedicated)` pour les playtests boss sans parcourir tout l'Acte 1.

---

## Documentation P3 produite

| Document | Chemin | Contenu |
|----------|--------|---------|
| Bible assets | `mge/docs/asset-style-bible.md` | Style D2-like, specs sprites/tileset |
| Matrice MVP freeze | `mge/docs/e10-mvp-matrix.md` | Statut tous les systemes D2 cibles |
| Equilibrage Acte 1 | `mge/docs/e10-balancing.md` | Donnees numeriques, recommandations P4 |
| Dossier transfert | `mge/docs/e10-transfer-p4p5.md` | Ce document |
| Trace P3 | `.mip/sequences/2026-03-06-mge-sodomight/phases/p3-trace.md` | Journal execution E01-E10 |

---

## Signature de cloture P3

- `cargo test --workspace` : **325 / 325 PASS**
- `cargo clippy --workspace -- -D warnings` : **0 warnings**
- `cargo fmt --all` : **PASS**
- Tous les gates G1 a G10 : **PASS**

P3 est ferme. Le dossier est suffisant pour entrer en audit P4/P5 sans zone floue majeure.
