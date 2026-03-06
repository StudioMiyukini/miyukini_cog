# Sodomight Asset Style Bible

> Engine-side reference for artists, tool authors and content pipeline.
> Canonical game-design source: `.mip/sequences/2026-03-06-mge-sodomight/ressources/requirements/23-sodomight-asset-style-bible.md`

---

## 1. Visual Identity

Sodomight evokes Diablo II through readability, isometric angle, detail density
and animation rhythm -- without ever being an exact copy.

### Allowed

- Reproduce the broad asset families (hero classes, NPCs, monsters, tiles, VFX).
- Follow the same principles of silhouette contrast and directional spritesheets.
- Use tilesets organized by biome / act.

### Forbidden

- Re-use, trace, repaint or recolorise any D2 sprite.
- Clone D2 UI borders, runes, icons or boss silhouettes.

### Sodomight Signatures

| Trait | Description |
|-------|-------------|
| Silhouettes | Slightly taller and more angular than D2 |
| Materials | Corroded iron, bone ivory, wax red, verdigris |
| Motifs | Original religious and occult patterns |
| UI | Gothic, restrained -- less baroque than D2 |
| VFX | Clear read, restrained saturation |

---

## 2. Technical Constraints

### Sprite Format

- **Format**: PNG 32-bit RGBA, pre-multiplied alpha.
- **Max sprite**: 512x512 px (atlas packing limit for `pack_shelf`).
- **Atlas size**: 2048x2048 default (`QualityTier::Standard`).
- **Naming**: `lowercase_underscore_only`, validated by `mge-asset-baker::validate_name`.
- **Directions**: 8 (S, SW, W, NW, N, NE, E, SE) matching `mge-render::animation::Direction`.

### Spritesheet Layout

```
<name>_<clip>_<direction>.png
Example: warrior_idle_s.png, warrior_run_ne.png
```

Each frame in a horizontal strip, uniform cell size. Metadata in companion `.ron` file.

### Animation Clips (per character archetype)

| Clip | Frames (target) | FPS | Loop |
|------|-----------------|-----|------|
| idle | 8 | 8 | yes |
| town_idle | 12 | 6 | yes |
| walk | 8 | 12 | yes |
| run | 8 | 15 | yes |
| attack_1 | 6 | 15 | no |
| attack_2 | 6 | 15 | no |
| cast_start | 4 | 12 | no |
| cast_release | 4 | 15 | no |
| hit_react | 4 | 15 | no |
| block | 3 | 12 | no |
| interact | 6 | 10 | no |
| death | 8 | 10 | no |
| corpse | 1 | 1 | no |

### Notify Events

Clips may embed notify events at specific frames (see `AnimNotify` in `mge-render`):

- `ActiveFrame` -- damage / hitbox activation
- `Footstep` -- foot lands (triggers SFX)
- `CastRelease` -- projectile or spell fires
- `ProjectileSpawn` -- visual projectile emitted
- `Sfx(name)` -- arbitrary named sound cue

---

## 3. Asset Families -- MVP Act 1

### 3.1 Playable Heroes

7 archetypes, 3 armour silhouette families (light / medium / heavy).

Weapon overlays: sword, axe, mace, dagger, staff, wand, polearm, bow, crossbow.
Off-hand overlays: small shield, large shield, occult focus.

### 3.2 Camp NPCs

| NPC | Role |
|-----|------|
| blacksmith | Repair, upgrade |
| healer | Heal, cure |
| merchant | Buy / sell general |
| occultist | Identify, enchant |
| gambler | Random items |
| mercenary_chief | Hire mercenaries |
| elder | Quest giver |
| stash_keeper | Stash access |
| waypoint_keeper | Fast travel |

### 3.3 Act 1 Monsters

| Monster | Type |
|---------|------|
| demonettes | Melee swarm |
| shamans | Summoners |
| husks | Undead melee |
| corrupt_archers | Ranged |
| brutes | Heavy melee |
| skeletons_melee | Melee undead |
| skeletons_ranged | Ranged undead |
| wraiths | Ethereal |
| arachnids | Vermin |
| carrion_flyers | Flying |
| elites | Palette-swap + unique accessories |
| act1_boss | Final boss |

### 3.4 Environments (tile families)

camp, bloody_moor, graveyard, cave, crypt, ruined_village,
monastery_exterior, barracks, prison, cathedral, catacombs, boss_room.

### 3.5 Props and Interactables

chest, barrel, sarcophagus, torch, brazier, well, dead_tree, rock,
barricade, door, gate, wagon, tent, altar, waypoint, portal, stash, vendor_stand.

### 3.6 VFX

melee_trail, phys_impact, blood, poison, fire, ice, lightning,
curse, aura, portal_vfx, resurrection, loot_sparkle, local_weather.

### 3.7 UI and Icons

hud_frame, life_orb, resource_orb, inventory_bg, stash_bg,
vendor_bg, quest_journal, char_sheet, skill_bar, potion_icon,
skill_icons, affix_icons, cursors, map_pings.

---

## 4. Asset Pipeline

```
1. Source creation (3D blockout / concept paint)
2. Render under reference camera & light
3. Paintover & weathering pass
4. Palette reduction / polish
5. Sheet cut + metadata generation
6. mge-asset-baker: validate names, hash, pack atlas, emit manifest
7. Runtime: TextureRegistry loads atlas + SpriteEntry lookup
```

### Baker Input

```json
{
  "atlas_name": "act1_monsters",
  "atlas_size": 2048,
  "source_dir": "assets/act1/monsters"
}
```

### Baker Output

- `<source_dir>/<atlas_name>_atlas.json` -- `AtlasManifest` with per-sprite rects and hashes.

---

## 5. Quality Budgets (from `mge-render::quality`)

| Budget | Low | Standard | High |
|--------|-----|----------|------|
| Max point lights | 8 | 32 | 64 |
| Max VFX particles | 256 | 1024 | 4096 |
| Max dynamic batches | 64 | 256 | 512 |
| Normal maps | no | no | yes |
| Shadow mode | None | BlobOnly | ContactShadow |
| Post-process | no | yes | yes |

---

## 6. Naming Conventions Summary

| Category | Pattern | Example |
|----------|---------|---------|
| Hero sprite | `<archetype>_<clip>_<dir>` | `warrior_idle_s` |
| Monster sprite | `<monster>_<clip>_<dir>` | `husk_walk_ne` |
| NPC sprite | `<npc>_<clip>_<dir>` | `blacksmith_idle_s` |
| Tile | `<biome>_<variant>` | `crypt_floor_a` |
| Prop | `<prop>_<state>` | `chest_closed` |
| VFX | `<effect>_<frame>` | `fire_03` |
| UI | `ui_<element>` | `ui_life_orb` |
| Icon | `icon_<category>_<name>` | `icon_skill_fireball` |

All names must pass `validate_name()`: lowercase ASCII + digits + underscores only.
