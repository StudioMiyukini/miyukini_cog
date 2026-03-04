# REF-05b -- Rendu Graphique, Audio & Projets Open Source D2 -- Reference Compacte
<!-- @id REF-05b @do reference-render-audio @role Fabrice @layer 7 @human miyuki -->
<!-- Split de REF-05 le 2026-03-03. Formules D2, valeurs exactes, MGE mapping, sprints, cross-refs -->

**Projet** : Sodomight | **Moteur** : MGE | **Ref D2** : v1.14d / D2R 2.x

> **TL;DR** : Rendu isometrique dimetric 2:1 (projection, 8 directions + mirroring, animations breakpoint-driven, formats sprites DC6/DCC vs PNG atlas, paper doll composition COF, palettes, tiles DT1/DS1, Z-ordering, lightmap 2D, VFX, meteo par acte). Audio 4 couches (32 canaux, BGM crossfade, sons iconiques, spatial audio, ambiances par acte). Analyse des 7 projets open source D2 et lecons pour MGE.

> **Voir aussi** : [REF-05a](REF-05a-UI-HUD.md) pour l'interface, HUD, menus, controles et inventaire.

---

## 1. Rendu Graphique `[S0-S1]`

> `-> MGE: mge-render::Renderer2D, mge-render::IsoCamera`

### 1.1 Projection isometrique dimetric 2:1 `[S0]`

> `-> MGE: mge-render::IsoProjection`

```
// World -> Screen
screen_x = (tile_x - tile_y) * (TILE_WIDTH / 2)
screen_y = (tile_x + tile_y) * (TILE_HEIGHT / 2)

// Screen -> World (inverse)
tile_x = (screen_x / (TW/2) + screen_y / (TH/2)) / 2
tile_y = (screen_y / (TH/2) - screen_x / (TW/2)) / 2
```

| Version | Tile | Sub-tile | Sub-tiles/tile | Usage |
|---------|------|----------|----------------|-------|
| D2 Original | 160x80 px | 32x16 px | 5x5 = 25 | Reference format |
| Sodomight pixel-perfect | 64x32 px | 16x8 px | 4x4 = 16 | Low-res mode |
| Sodomight HD | 160x80 px | 32x16 px | 5x5 = 25 | HD mode |

Sub-tile flags : walkable (1 bit), light-blocking (1 bit), line-of-sight (1 bit). 3 bits/sub-tile.
TOML: `assets/data/tiles/tile_flags.toml`

### 1.2 Directions (8-way) + mirroring `[S0]`

> `-> MGE: mge-render::SpriteDirection`

```
N(0), NE(1), E(2), SE(3), S(4) = 5 directions stockees
SW(5) = flip_x(SE), W(6) = flip_x(E), NW(7) = flip_x(NE)
```

Flag `flip_x` dans `SpriteInstanceGpu`. Atlas reduction : 5/8 = **37.5% espace economise**.
`-> MGE: mge-render::SpriteInstanceGpu { flip_x: bool, direction: u8 }`

### 1.3 Animations `[S0-S1]`

> `-> MGE: mge-render::AnimationController` | TOML: `assets/data/animations/anim_definitions.toml`
> Cross-ref: [REF-02a SS3](REF-02a-Combat-Formulas.md) (breakpoints IAS/FCR/FHR/FBR)

| Mode | Code D2 | Frames | Variable par | Cycle | Notes |
|------|---------|--------|-------------|-------|-------|
| Idle | NU | 12-16 | Non | Loop | Default state |
| Walk | WL | 8-12 | Non | Loop | -- |
| Run | RN | 8 | Non | Loop | -- |
| Attack 1 | A1 | 12-16 | IAS breakpoints | One-shot | REF-02a SS3.1 |
| Attack 2 | A2 | 12-16 | IAS breakpoints | One-shot | REF-02a SS3.1 |
| Cast | SC | 10-19 | FCR breakpoints | One-shot | REF-02a SS3.2 |
| Get Hit | GH | 5-17 | FHR breakpoints | One-shot | REF-02a SS3.3 |
| Block | BL | 2-17 | FBR breakpoints | One-shot | REF-02a SS3.4 |
| Death | DT | 15-20 | Non | Freeze last | Corpse persiste |
| Kick | KK | 12 | IAS | One-shot | Assassin only |

**FPS base = 25** (40ms/frame). Monstres additionnels : S1-S4(skills), RV(revive), RN(charge).

**Breakpoint formula** (frames affichees) :
```
action_frames = ceil(256 * base_frames / floor(256 * (100 + EIAS) / 100))
```
Cross-ref: [REF-02a SS3](REF-02a-Combat-Formulas.md) pour tables EIAS par classe/arme.

### 1.4 Formats sprites `[S0]`

> `-> MGE: mge-asset::SpriteAtlas, mge-asset::AtlasDefinition`

**DC6** (D2 original, statique) :
- Header : version(4B)=6, unknown(4B), unknown(4B), termination(4B)=0xEEEEEEEE, nb_directions(4B), frames_per_dir(4B)
- Frame : flip(4B), width(4B), height(4B), offset_x(4B), offset_y(4B), unknown(4B), next_block(4B), length(4B) + pixel data
- Encoding : raw pixels (no compression) + skip bytes (`0x80|n` = skip n transparent px)
- Palette : externe 256 couleurs

**DCC** (D2 original, anims) : ~5-10x compression vs DC6, diff inter-frames, bit-packing, palette externe (.DAT/.PL2).

**Sodomight/MGE** : PNG + atlas TOML. Pas de decodage DC6/DCC (assets maison).
Structure atlas : `sprite_id -> { texture, rect, directions, frames_per_dir, layers[] }`
TOML: `assets/sprites/atlas_manifest.toml`

### 1.5 Composition COF (Paper Doll) `[S1]`

> `-> MGE: mge-render::CompositionStack` | TOML: `assets/data/render/composition_order.toml`
> Cross-ref: **REF-03 SS1.2** (equip slots)

| Layer | Code | Contenu | Z-order (direction S) |
|-------|------|---------|-----------------------|
| 0 | HD | Head | 6 |
| 1 | TR | Torso | 4 |
| 2 | LG | Legs | 3 |
| 3 | RA | Right Arm (+arme) | 5 |
| 4 | LA | Left Arm (+bouclier) | 7 |
| 5 | RH | Right Hand weapon gfx | 8 |
| 6 | LH | Left Hand shield gfx | 2 |
| 7+ | SH/S1-S8 | Effets speciaux, overlays | 9+ (additif) |

Ordre dessin change par direction : bouclier devant en S, derriere en N.
8 directions x variable layers = `composition_order.toml` lookup table (8x8 matrix).

### 1.6 Palettes & recoloration `[S1]`

> `-> MGE: mge-render::PaletteManager`

D2 original : 256 couleurs/acte (`pal.dat`=768 bytes BGR, `pal.pl2`=palette+colormaps). Recoloration monstres via `utrans` (colormap shift par acte).

**Sodomight** : PNG pre-colores OU shader `color_tint` sur grayscale sprites.
Shader approach : `final_color = grayscale_sample * tint_color * intensity`. Plus flexible, moins d'assets.
TOML: `assets/data/render/palettes.toml` (tint definitions par acte/monstre variant)

### 1.7 Tiles (DT1/DS1) `[S0-S1]`

> `-> MGE: mge-render::TileRenderer, mge-arpg-world::TileMap`

**Sub-tile types** (D2 orientation IDs) :

| ID | Type | Taille | Notes |
|----|------|--------|-------|
| 0 | Floor | 32x16 losange | Walkable surface |
| 1-4 | Walls | 32x32 rect | Directions cardinales |
| 5-9 | Corners/portes | 32x32 | Special collision |
| 12 | Pillar | 32x32 | Overlay + mask |
| 13 | Shadow | 32x16 | Blend multiply |
| 14 | Tree | Variable | Alpha overlay |
| 15 | Roof | 32x32 | Offset Y = multiples 80px, alpha si joueur dessous |

DS1 = layout map fichier. **Sodomight** : LDtk maps (`mge-arpg-world::LdtkLoader`).

**Render order** (back-to-front) :
```
1. Floor tiles (Z=0)
2. Shadows (multiply blend)
3. Wall lower (sub-tile Z)
4. Entities (Y-sort, Z = tile_y * MAP_W + tile_x)
5. Wall upper (sub-tile Z + offset)
6. Roof (alpha fade when player underneath)
7. Weather particles
8. FX / projectiles
9. HUD overlay (screen-space, always on top)
```

### 1.8 Z-Ordering `[S0]`

> `-> MGE: mge-render::ZSort, mge-render::IsoPosition`

```
sort_key = tile_y * MAP_WIDTH + tile_x
```

| Cas special | Regle |
|-------------|-------|
| Mur | Sub-tile Z (per-sub-tile, pas per-tile) |
| Pilier | Overlay + mask (draw 2x: behind + front) |
| Toit | Alpha fade 0.0 quand joueur en dessous |
| Projectile | Z = ombre au sol (pas position 3D) |
| Grand monstre | Z = max(tiles occupees) |
| Items au sol | Z = tile_z + 0.1 (devant floor, derriere entites) |

Algorithme : Painter's CPU, tri `IsoPosition::sort_key()` chaque frame. O(n log n), n = entites visibles.
`-> MGE: mge-render::IsoPosition { pub fn sort_key(&self) -> u32 }`

### 1.9 Eclairage `[S1]`

> `-> MGE: mge-render::Lightmap2D`

| Source | Radius (sub-tiles) | Comportement |
|--------|---------------------|-------------|
| Joueur | 13 (base), 18 (max), 1 (min) | Constant, +bonus items |
| Torche murale | ~5 | Flicker sin(t) +/- 1 |
| Feu au sol | ~3-5 | Flicker random |
| Holy Fire aura | ~6 | Constant + pulse |
| Lightning skill | ~8 | Flash 1-2 frames |
| Town | Infini | Pas de lightmap en ville |

Lightmap : 2D basse res (200x150 px = 1/4 game res), upsamplee bilineaire, multipliee avec rendu monde.
Exterieurs : pas de lightmap (full bright). Caves/donjons : lightmap active.
`-> MGE: mge-render::Lightmap2D { buffer: Vec<f32>, width: 200, height: 150 }`

### 1.10 Effets visuels `[S1-S2]`

> `-> MGE: mge-render::VfxSystem`

| Effet | Blend mode | Pool max | Exemples |
|-------|-----------|----------|----------|
| Feu | Additive | 64 | Inferno, Fire Wall, Meteor |
| Glace | Alpha | 32 | Blizzard, Frozen Orb |
| Foudre | Additive | 48 | Lightning, Chain Lightning |
| Poison | Alpha (vert) | 32 | Poison Nova |
| Holy/Portal/Aura | Additive | 16 | Holy Bolt, Town Portal, Paladin auras |
| Sang | Alpha (rouge) | 32 | Coups critiques, Corpse Explosion |
| Particules | Additive/Alpha | 128 | Meteo, debris |

**Screen effects** :

| Effet | Duree | Technique |
|-------|-------|-----------|
| Flash blanc (hit boss) | 2-4 frames | Full-screen additive overlay |
| Screen shake | 4-8 frames | Camera offset sin(t) * amplitude |
| Desaturation (mort) | Fade 0.5s | Shader saturation lerp -> 0 |
| Red flash (damage) | 2 frames | Screen tint overlay |
| Level up | 30 frames | Particle burst + light flash |

### 1.11 Meteo & Resolution `[S1-S2]`

> `-> MGE: mge-render::WeatherSystem, mge-render::RenderPipeline`

**Meteo par acte** :

| Acte | Outdoor | Particules | Son |
|------|---------|------------|-----|
| 1 | Pluie + eclairs | Gouttes (alpha, gravity) | Rain loop + thunder |
| 2 | Rien (chaleur) | Shimmer heat (optionnel) | Wind |
| 3 | Brume optionnelle | Fog particles | -- |
| 4 | Cendres | Ash particles (additive) | Fire ambient |
| 5 | Neige + vent | Flocons (alpha, wind drift) | Blizzard loop |

**Resolution pipeline** :

| Mode | Offscreen | Upscale | HUD | Sprites |
|------|-----------|---------|-----|---------|
| Pixel | 800x600 | Nearest -> native | Native res | Pixel art |
| HD | Native (1080p+) | -- | Native res | HD sprites |

HUD toujours rendu en resolution native (pas d'upscale). Mode perspective (sub-tiles etires par distance centre) = nice-to-have S3.

---

## 2. Audio `[S1-S2]`

> `-> MGE: mge-audio::AudioEngine` | TOML: `assets/data/audio/sounds.toml`

### 2.1 Architecture 4 couches `[S1]`

> `-> MGE: mge-audio::MixerLayer`

| Prio | Layer | Canaux reserves | Volume | Interruption |
|------|-------|-----------------|--------|-------------|
| 4 | Voice | 2 | 100% | Jamais |
| 3 | UI SFX | 4 | 100% | Jamais |
| 2 | Combat SFX | 16 | Distance-scaled | Coupe si pool sature |
| 1 | Ambient | 6 | Distance-scaled | Coupe si pool sature |
| 0 | Music | 2 (stereo) | Global setting | Fade/cut transitions |

**Pool total : 32 canaux** simultanes. Allocation par priorite.
`-> MGE: mge-audio::ChannelPool { max_channels: 32 }`

### 2.2 Musique (19 pistes + menu) `[S1]`

> `-> MGE: mge-audio::BgmPlayer` | TOML: `assets/data/audio/music.toml`

| Acte | Town | Outdoor | Indoor | Boss/Special |
|------|------|---------|--------|-------------|
| 1 | Rogue | Wilderness | Sisters/Crypt | -- |
| 2 | Toru (Lut Gholein) | Desert | Tombs/Lair | -- |
| 3 | Jungle (Kurast) | Mesa | Zakarum | -- |
| 4 | Sanctuary | -- (Sanctuary) | -- (Sanctuary) | Leoric (Chaos/Diablo) |
| 5 | Harrogath | Outdoor A5 | Indoor A5 | Nihlathak, WSK, Baal |
| -- | -- | -- | -- | Main Menu (flammes) |

Transitions : `CrossFade(2s)` normal, `Cut` immediat pour boss.
```
Normal: BGM fade_out(2s) -> silence(0.5s) -> BGM fade_in(2s)
Boss:   BGM CUT -> boss_theme (immediate, no fade)
```

### 2.3 Sons iconiques `[S1-S2]`

> `-> MGE: mge-audio::SfxLibrary` | TOML: `assets/data/audio/sfx.toml`

**Items** (CRITIQUE pour game feel) :

| Son | Priorite | Vol min | Variantes | Notes |
|-----|----------|---------|-----------|-------|
| **Unique drop** | **MAX** | **Garanti a distance** | 1 | Tintement metallique grave, JAMAIS reutilise |
| Set drop | Haute | Garanti | 1 | Similar + variation pitch |
| Rare drop | Moyenne | Distance-scaled | 1 | Tintement prononce |
| Gold pickup | Basse | Distance-scaled | 2 | Clink |
| Equip metal/leather/cloth | Basse | Local | 3 chacun | Par material type |
| Identify | Moyenne | Local | 1 | Zing |

**Combat** (min 3 variantes/son) :

| Categorie | Variantes min | Notes |
|-----------|--------------|-------|
| Hit flesh | 3-4 | Par arme type |
| Hit metal | 3-4 | Shield/armor |
| Miss | 2-3 | Swoosh |
| Block | 2-3 | Shield clang |
| Critical hit | 2 | Extra impact |
| Bow release | 2 | String twang |
| Monster death | 3-4/famille | Specifique par famille |
| Monster grunt | 3-4/famille | Attack sounds |
| Player grunt | 2/classe | Hit reaction |

**Skills par element** :
- Fire : whoosh + crackle (fire base), roar (Meteor), sizzle (Fire Wall)
- Ice : crack + shatter, crystal break (Frozen Orb), wind (Blizzard)
- Lightning : crackle + thunder, zap (Chain Lightning)
- Poison : sizzle + bubble, hiss (Poison Nova)
- Holy : chime + bell, choir (Blessed Hammer)
- Physical : impact + crunch, bone break (melee skills)
- Summon : growl + roar (Raise Skeleton, Revive)
- Curse : whisper + echo (all curses)

**UI** : level up(fanfare, 2s), quest complete(chime), WP activate(chime+hum), portal(whoosh+loop), potion(gulp), click(light), error(buzz, 0.2s).

### 2.4 Audio spatial & transitions `[S1]`

> `-> MGE: mge-audio::SpatialAudio`

```
volume = base_volume * max(0.0, 1.0 - distance / MAX_AUDIBLE)   // MAX_AUDIBLE = ~20 tiles
pan    = clamp((sound_x - camera_x) / (SCREEN_W / 2), -1.0, 1.0)
```

| Regle | Comportement |
|-------|-------------|
| UI SFX | Plein vol, pas de spatialisation |
| Combat joueur | Volume fixe (propre joueur), spatial (autres) |
| Musique | Global, pas de spatialisation |
| Unique drop | Vol minimum garanti a toute distance |
| Ambient | Full spatial + distance falloff |

TOML structure `sounds.toml` :
```toml
[[sound]]
id = "hit_flesh_01"
file = "sfx/combat/hit_flesh_01.ogg"
volume = 200          # 0-255
group_size = 4        # variantes dans le groupe
loop = false
priority = 2          # layer priority
falloff = 20.0        # tiles
```

### 2.5 Ambiances par acte `[S2]`

> `-> MGE: mge-audio::AmbientLayer`

| Acte | Outdoor | Indoor | Town |
|------|---------|--------|------|
| 1 | Grillons, vent, oiseaux, pluie intermittente | Gouttes, grincements, echos | Foule, feu de camp, forgeron |
| 2 | Vent sable, silence lourd | Echos tombes, torches crackling | Marche bruyant, fontaine |
| 3 | Insectes, oiseaux exotiques, eau | Eau stagnante, bubbling | Port, eau, bateaux |
| 4 | Vent infernal, cris lointains, feu | -- | Silence, echos metalliques |
| 5 | Vent glacial, blizzard, craquements | Echos glace, craquements | Forge, marteaux, neige |

Ambient = 2-3 layers superposes (base loop + random one-shots + weather).

---

## 3. Projets Open Source D2 -- Lecons `[S0]`

> Cross-ref: tous REF-01 a REF-04 pour les formules documentees

### 3.1 Synthese

| Projet | Lang | Stars | Valeur MGE | Statut |
|--------|------|-------|-----------|--------|
| OpenDiablo2 | Go | 15k | Archi modulaire, renderer ref | Archive (abandonne) |
| Abyss Engine | C | ~1k | Separation engine/game(JS) | Archive |
| OpenD2 | C/C++ | ~500 | Modularite idTech (DLL) | Inactif |
| Worldstone | C++ | ~200 | **Decodeurs reference** DC6/DCC/PL2/COF | Reference technique |
| D2MOO | C | ~300 | **Formules exactes** D2 (reverse-eng) | Actif, source de verite |
| HellSpawner | Go | ~500 | Editeur desktop (viewers tous formats) | Reference pour mge-studio |
| D2ModMaker | Go | ~800 | Data-driven design (randomizer) | Reference pour TOML data |

### 3.2 Lecons pour MGE

**8 lecons** : (1) Separation moteur/jeu = deja fait (S0). (2) Formats standards PNG/TOML/LDtk/OGG eliminent 60-80% reverse-eng (S0). (3) Gameplay = plus dur qu'engine -> TDD massif combat/loot/AI (S1-S2). (4) ECS indispensable = `mge-ecs` deja fait (S0). (5) Scripting quetes Rhai + Rust combat (S2). (6) Commencer petit : S0=fenetre, S1=1 classe 1 zone, S2=loot, S3=multi. (7) Editeur critique `mge-studio` Dioxus en parallele (S1+). (8) Formules documentees REF-01-04 = fondation (S0).

### 3.3 Correspondance OS -> MGE Crates

OD2 fileformats -> `mge-asset` (PNG+TOML, pas de legacy). OD2 d2map/d2render -> `mge-render` + `mge-arpg-world` (LDtk). OD2 d2audio -> `mge-audio` (OGG, kira). D2MOO -> `mge-arpg-combat`/`mge-arpg-ai`/`mge-arpg-quest` (formules Rust). HellSpawner -> `mge-studio` (Dioxus). D2ModMaker -> TOML `assets/data/`. Worldstone decoders -> non utilise (assets maison).
