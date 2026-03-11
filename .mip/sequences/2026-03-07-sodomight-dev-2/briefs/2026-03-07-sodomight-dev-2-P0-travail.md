# Travail P0 2026-03-07-sodomight-dev-2

## Statut

- Etat : COMPLET
- Phase : P0
- Responsable principal : Maria
- Date : 2026-03-07

## TL;DR

Pipeline wgpu sprite instancie livre en 6 etapes + BUF. La scene rogue_camp (grille iso 16x16 + joueur + HUD) sera visible a la sortie de P3.

## Temps executes

| Temps | Titre | Agent | Statut |
|-------|-------|-------|--------|
| T01 | Exploration et brainstorming | Maria | DONE |
| T02 | Ideation — 3 approches | Lise | DONE |
| T03 | Analyse concurrentielle (PoE, GD, LE) | Fabrice | DONE |
| T04 | Inventaire prerequisites | Denis/Hugo/Jean | DONE |
| T05 | Analyse securite | Victor | DONE |
| T06 | Specification technique | Francois | DONE |
| T07 | Plan exhaustif P3 | Denis | DONE |
| T08 | Audit faisabilite | Arianne/Jean | DONE |
| T09 | Verification CI/CD | Hugo | DONE |
| T10 | Synthese et brief | Maria | DONE |

---

## T01 — Exploration (Maria)

**Diagnostic racine :**

| Fichier | Etat constate |
|---------|--------------|
| `mge-render/src/lib.rs` : `render()` | Clear-only — aucun sprite drawn |
| `mge-render/src/batch.rs` | SpriteBatch correct, aucune connexion GPU |
| `mge-render/src/atlas.rs` | TextureRegistry CPU uniquement, 0 wgpu texture |
| `mge-render/src/camera.rs` | IsoCamera (maths purs), jamais integre dans render loop |
| `mge-render/src/pass.rs` | PassDescriptor + FRAME_PASSES definis, jamais executes |
| Shaders WGSL | AUCUN dans le repo |
| Assets PNG/DDS | AUCUN dans `mge/` |
| `AtlasHandle::new_test` | Marque `#[cfg(test)]` — inutilisable dans main.rs production |

**wgpu reel : 28.0.0** (pas 22 comme annonce — version workspace verifiee).

**Conclusion T01 :** La sequence 1 a livre l'architecture data-layer complete. La sequence 2 doit livrer la couche GPU : shaders + pipeline + wiring + scene initiale.

---

## T02 — Ideation (Lise)

Trois approches evaluees :

| | A (triangles flat) | B (quad instancie + tint) | C (PNG + image crate) |
|-|-|-|-|
| Complexite | 1/5 | 3/5 | 4/5 |
| Reutilisabilite | 0% | 100% | 100% |
| Assets requis | aucun | aucun | PNG minimal |
| Valide instancing | non | OUI | OUI |
| Visuel D2-like | non | partiel (colore) | oui (si asset) |

**Decision : Approche B** — texture 1x1 blanche + tint par instance.
- Architecture finale reutilisable a 100%
- Aucun asset externe requis
- Valide le pipeline complet (instancing, depth sort, camera iso, GPU buffers)

---

## T03 — Analyse concurrentielle (Fabrice)

Tous les jeux D2-like modernes (Path of Exile, Grim Dawn, Last Epoch) convergent vers :
- Instanced rendering (1 draw call par atlas pour N sprites)
- Atlas texture + UV offset par instance
- Depth Z = -(world_x + world_y) compute CPU
- Sort CPU par texture_id/material avant upload GPU

**Le SpriteBatch existant correspond exactement au pattern Grim Dawn.** L'instancing GPU est le chainon manquant.

---

## T04 — Inventaire prerequis (Denis/Hugo/Jean)

### Dependances a ajouter

| Crate | Version | Cargo.toml |
|-------|---------|-----------|
| `bytemuck` | `"1"` features `["derive"]` | `mge/Cargo.toml` (workspace) + `mge-render/Cargo.toml` |

### Fichiers a creer

- `mge-render/src/pipeline.rs` — SpritePipeline + SpriteInstanceGpu
- `mge-render/src/shader.wgsl` — WGSL vertex+fragment

### Fichiers a modifier

- `mge-render/src/lib.rs` — extend GraphicsState, `render(&batch)`
- `mge-render/src/atlas.rs` — `AtlasHandle::new(u32)` public (retirer cfg(test))
- `mge-render/Cargo.toml` — ajouter bytemuck
- `mge/Cargo.toml` — bytemuck dans workspace deps
- `sodomight/src/main.rs` — populate SpriteBatch chaque frame

---

## T05 — Securite (Victor)

Score estime : **88/100** (perimetre local-only)

Top 5 recommandations :
1. Clamp `instances.len()` a `MAX_INSTANCES` avant `write_buffer`
2. Pas de `unwrap()` sur `create_shader_module` — propager via `Result`
3. `const` assert size/alignement de `SpriteInstanceGpu` au compile-time
4. Assertion explicite `white.len() == 4` avant `write_texture`
5. Shader WGSL statique (`include_str!`) — jamais genere dynamiquement

---

## T06 — Spec technique (Francois)

Architecture complete dans `specs/2026-03-07-sodomight-dev-2-spec.md`.

Points cles :
- `SpriteInstanceGpu` : `#[repr(C)] #[derive(Pod, Zeroable)]`, 48 bytes
- Shader : viewport uniform (group 0), texture+sampler (group 1), NDC conversion pixel->[-1,1]
- `SpritePipeline::render()` : upload instances clampe + begin_render_pass (LoadOp::Load apres clear)
- Tests existants (batch, camera, pass, atlas) non impactes — CPU uniquement

---

## T07 — Plan P3 (Denis)

DAG : E00 -> E01 -> E02 -> E03 -> E04 -> E05 -> BUF

| Etape | Role | Agent | Duree est. |
|-------|------|-------|-----------|
| E00 | bytemuck dep | Denis | 30 min |
| E01 | WGSL shader | Denis | 1h |
| E02 | SpriteInstanceGpu + SpritePipeline | Denis | 2h |
| E03 | GraphicsState cablage | Denis | 2h |
| E04 | AtlasHandle/MaterialHandle publics | Denis | 30 min |
| E05 | Scene rogue_camp dans main.rs | Lise | 1h30 |
| BUF | Tests + clippy | Francois | 1h |

Total estime : ~8h30

---

## T08 — Audit faisabilite (Arianne + Jean)

**Verdict : FAISABLE AVEC RESERVES**

Score efficience : 15/20

Reserves critiques :
1. Signature `render()` change — coordonner E03 et E05
2. `SpriteInstanceGpu` — audit bytemuck alignment obligatoire (zero enum/bool)
3. Buffer GPU fixe (16384 instances max) recommande pour P0
4. Conversion `SpriteInstance -> SpriteInstanceGpu` (UV normalisation) a documenter dans E03

---

## T09 — CI/CD (Hugo)

- Aucun workflow CI pour `mge/` existant (le CI actuel ne couvre que `miyucloud`)
- Commandes de validation sequentielles documentees dans le brief
- Workflow `mge-ci.yml` a creer (recommandation, hors perimetre P3)

---

## Decisions cles

| Decision | Choix | Justification |
|----------|-------|--------------|
| Approche rendu | B — instanced + texture 1x1 | Reutilisable 100%, pas d'assets, valide pipeline complet |
| wgpu version | 28.0.0 (workspace) | Deja en place, stable |
| Dep ajoutee | bytemuck 1.x | `unsafe_code = forbid` oblige le cast safe |
| Buffer GPU | Fixe, 16384 instances max | Simple, suffisant pour P0 scene |
| Shader WGSL | Statique via `include_str!` | Securite Victor rec. #5 |
| AtlasHandle | `pub fn new(u32)` + conserver `new_test` test-alias | Compatibilite tests existants |
| Scene initiale | Grille iso 16x16 + joueur + HUD | Minimal D2-like visible |
