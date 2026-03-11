# Brief sequence 2026-03-07-sodomight-dev-2

## Statut

- Etat : APPROUVE — P3 EN COURS
- Phase : P0
- Responsable principal : Maria
- Classification : STANDARD
- Complexite : C4 — elevee
- Classe MIP : T5
- Mode autonomie : BIG_STEPS
- Date : 2026-03-07

---

## TL;DR

Cabler le pipeline wgpu sprite instancie dans `mge-render` pour passer d'un ecran brun vide a une scene isometrique D2-like visible (grille rogue_camp 16x16 + joueur + HUD).

6 etapes + BUF. Un seul nouveau fichier shader. Aucun asset externe requis.

---

## Contexte

La sequence precedente (`2026-03-06-mge-sodomight`, T5, 325 tests) a livre l'integralite de l'architecture data-layer du moteur :
- `SpriteBatch`, `IsoCamera`, `TextureRegistry`, `PassDescriptor` — structures CPU completes
- Toutes les crates gameplay/world/monsters/items — logique metier testee
- `GraphicsState` wgpu initialise

**Ce qui manque :** le pont entre la couche CPU et le GPU. `GraphicsState::render()` ne fait qu'un clear pass. Aucun shader WGSL n'existe dans le repo. Aucun asset (PNG/DDS) non plus.

Le resultat visible : une fenetre brun fonce, identique au screenshot fourni.

---

## Objectif principal

**Rendre Sodomight visuellement non-vide.** A la fin de P3, lancer `cargo run -p sodomight` doit afficher :
- Une grille isometrique 16x16 de tuiles colorees (brun/vert alternes)
- Un indicateur joueur (quad rouge) au centre
- Des barres HP/Mana (HUD rectangulaire) en bas-gauche

Ce rendu utilise une texture 1x1 blanche avec tint par instance — aucun asset artistique n'est requis, et l'architecture GPU produite est reutilisable a 100% avec de vrais sprites.

---

## Perimetre

### Inclus (P3)
- `mge-render/src/pipeline.rs` — `SpritePipeline` + `SpriteInstanceGpu`
- `mge-render/src/shader.wgsl` — WGSL vertex + fragment (instancie)
- `mge-render/src/lib.rs` — `GraphicsState::render(&batch)` cable
- `mge-render/src/atlas.rs` — `AtlasHandle::new(u32)` + `MaterialHandle::new(u32)` publics
- `sodomight/src/main.rs` — populate SpriteBatch + IsoCamera
- `mge/Cargo.toml` + `mge-render/Cargo.toml` — dep `bytemuck`

### Exclus (post-P3)
- Assets reels (PNG/DDS) — Atlas pipeline
- Vrai systeme de rendu par chunks
- Multijoueur / reseau
- Sons / animations
- Workflow CI/CD `mge-ci.yml` (recommandation Hugo, hors perimetre)

---

## Stack definie (P0)

| Composant | Choix | Statut |
|-----------|-------|--------|
| Renderer | wgpu 28.0.0 | Deja en workspace |
| Shaders | WGSL (statique, `include_str!`) | A creer |
| GPU instance data | bytemuck 1.x (#[repr(C)] Pod) | A ajouter |
| Texture | 1x1 blanche procedural | Pas d'asset |
| Camera | IsoCamera existante | Deja presente |

---

## Contraintes

- `unsafe_code = "forbid"` en workspace — bytemuck obligatoire, aucun unsafe tolere
- Tests existants (325 tests mge, 0 failed) doivent rester verts
- `cargo clippy -D warnings` : 0 violations
- Score securite >= 88/100 (perimetre local-only)
- Score efficience >= 15/20

---

## Definition of Done

| Critere | Condition |
|---------|-----------|
| Rendu visible | `cargo run -p sodomight` : grille iso + joueur + HUD visibles |
| Tests | `cargo test -p mge-render` : 0 failed |
| Clippy | `cargo clippy -p mge-render -p sodomight -- -D warnings` : 0 violations |
| Securite | Score >= 88/100, aucun `unsafe` introduit |
| Architecture | SpritePipeline reutilisable pour assets reels (UV mapping fonctionnel) |
| Compatibilite | Tests existants de la sequence 1 non casses |

---

## Plan P3 — Resume

**Etapes :** E00 → E01 → E02 → E03 → E04 → E05 → BUF

| Etape | Role | Agent | Duree est. |
|-------|------|-------|-----------|
| E00 | bytemuck dep (Cargo.toml) | Denis | 30 min |
| E01 | WGSL shader sprite instancie | Denis | 1h |
| E02 | SpriteInstanceGpu + SpritePipeline::new() | Denis | 2h |
| E03 | GraphicsState::render(&batch) cable | Denis | 2h |
| E04 | AtlasHandle/MaterialHandle::new() publics | Denis | 30 min |
| E05 | Scene rogue_camp dans main.rs | Lise | 1h30 |
| BUF | Tests unitaires pipeline + clippy + validation visuelle | Francois | 1h |

Total estime : ~8h30

---

## Risques identifies

| Risque | Impact | Mitigation |
|--------|--------|-----------|
| Erreur WGSL visible seulement au runtime | Blocant rendu | `cargo run` obligatoire en BUF |
| bytemuck::Pod struct non-valide | Panic compilation | Assert size compile-time E02 |
| Signature `render()` casse main.rs (attendu) | Normal — coordonne | E03 avant E05, sequence stricte |
| `AtlasHandle::new` non const fn | Compilation impossible comme const | Utiliser `let` au lieu de `const` si non-const |

---

## Faisabilite

**FAISABLE AVEC RESERVES** (Arianne, T08)
- Score efficience P0 : 15/20
- Toutes les reserves documentees dans le plan E03 et BUF
- Aucun bloquant architectural

---

## Prochaine etape

Approbation de ce brief par l'utilisateur → lancement P3 automatique (BIG_STEPS).
Premiere gate BIG_STEPS : apres E03 (avant E04/E05).
