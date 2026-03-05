<!-- @id mem.project.mge
     @do provide_mge_project_decisions
     @role project
     @layer memory
     @human Décisions projet MGE — architecture, render reforge -->

# MGE — Miyukini Game Engine (dans `mge/`)

> Doc de conception : `mge/docs/MGE-Design-Document.md`

**Jeux :** Sodomight (D2 clone exact, Phase 1) + Allumina (MMO-ARPG, Phase 2)
**Decisions verrouillees :**
- Stack : winit + wgpu (pas Bevy), ECS archetype maison + sparse overlay, kira audio, Rhai scripting
- Iso : Dimetric 2:1, tiles 64x32px
- Assets : PNG + Aseprite + LDtk + TexturePacker JSON ; registry TOML avec IDs symboliques stables
- Resolution : dual mode pixel-perfect 800x600 upscale OU HD 1080p+ (config joueur)
- Data : TOML pour tout le game data (items, skills, monstres, zones)
- Persistance : KindMother (SQLite), saves cote host/serveur
- Reseau : Listen Server MVP -> serveur dedie. Sodomight-server et Allumina-server = codebases separees
- Plateformes : Windows + Linux + macOS
- Hot-reload dev : `notify` crate sur TOML + assets
- Outils : `mge-studio` en Dioxus 0.6, CLI tools (mge-packer, mge-slicer, mge-rescale, mge-mirror, mge-remap)

**Assets de test existants :** `mge/assets/Dev_assets/` (tiles herbe, arbres, NPC, monstres, sprites test, reference D2 map)

## Architecture MGE (confirmé fév. 2026)

Le workspace MGE a ete restructure en 4 couches ciblees ARPG :
- **Couche 1 Kernel** (5 crates) : mge-core, mge-ecs, mge-math, mge-asset, mge-platform
- **Couche 2 Engine** (9 crates) : mge-render, mge-audio, mge-ui, mge-pathfinding, mge-collision, mge-collision-rich, mge-script, mge-net, mge-save
- **Couche 3 Pack ARPG** (10 crates) : mge-arpg-{world,entity,combat,items,stats,skills,loot,ai,quest,trade}
- **Couche 4 Game** (3 crates) : sodomight, sodomight-server, sodomight-client
- **Outils** (7 crates) : mge-studio, mge-packer, mge-slicer, mge-rescale, mge-mirror, mge-remap, mge-anim-pack

Les anciens packs generiques (rpg, rts, grand-strategy, idle, factory, shooter, etc.) ont ete supprimes.

## MGE Render Reforge (confirmé mars 2026)

Chantier T5 livre. Tag `mge-render-reforge-v1.0`. Audit George 93/100 CONFORME.

**Modules livres dans `mge-render`** :
- Instanced sprite batcher (SpriteBatcher, InstanceData, wgpu instanced draw calls)
- Animation FSM (AnimationController, AnimationDef, AnimationMode, transitions, events, LOD)
- TTF font engine (FontEngine, GlyphCache, atlas packing, text shaping)
- Overhead UI (OverheadRenderer : floating text, emotes, progress bars, damage numbers)
- Dual camera (Camera2D : world/screen conversions, zoom, viewport)
- `mge-anim-pack` CLI tool (validation, preview, stats des fichiers animation TOML)

**Feature flags mge-render** : `instanced` (default), `dual-res` (default), `legacy-batcher`, `post-process`, `index-u32`
**RenderConfig** : `max_sprites` configurable, plafond u16=16384 / u32=262144 selon feature `index-u32`
**AnimationLod** : 4 tiers (Full <10 tiles, Reduced 10-20, Minimal 20-40, Static >40). `compute_lod()` + `tick_with_lod()`
**Metriques** : 128 tests mge-render, 798 tests workspace total, 0 clippy warnings, 0 unsafe
**Anti-patterns** : voir `.mip/memory/patterns-and-lessons.md`
