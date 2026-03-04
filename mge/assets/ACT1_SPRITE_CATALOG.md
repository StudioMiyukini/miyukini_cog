# Sodomight — Act 1 Sprite Catalog

Source : https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/

Ce document recense tous les sprites necessaires pour reconstituer l'Act 1 de Diablo 2
dans Sodomight, avec les URLs spriters-resource correspondantes.

## Legende

- **DISPO** = sprite disponible sur spriters-resource.com
- **PLACEHOLDER** = asset placeholder deja present dans `mge/assets/`
- **MANQUANT** = a creer / sourcer ailleurs

---

## 1. Classes jouables (priorite 1 : Barbarian + Sorceress)

| Sprite | Taille | Poids | URL | Statut |
|--------|--------|-------|-----|--------|
| Barbarian (Light) | 3357x11645 | 3.48 MB | `/asset/54291/` | DISPO |
| Barbarian (Medium) | — | — | `/asset/54292/` (est.) | DISPO |
| Barbarian (Heavy) | — | — | `/asset/54293/` (est.) | DISPO |
| Sorceress (Light) | 2605x6736 | 1.67 MB | `/asset/54300/` | DISPO |
| Sorceress (Heavy) | — | — | `/asset/54301/` (est.) | DISPO |
| Amazon (Light) | — | — | `/asset/54286/` | DISPO |
| Amazon (Heavy) | — | — | `/asset/54287/` | DISPO |
| Paladin (Light) | — | — | `/asset/54299/` (est.) | DISPO |
| Paladin (Medium) | — | — | `/asset/54302/` (est.) | DISPO |
| Necromancer (Light) | — | — | `/asset/54297/` | DISPO |
| Necromancer (Medium) | — | — | `/asset/54298/` (est.) | DISPO |
| Druid (Light) | — | — | `/asset/54294/` | DISPO |
| Assassin (Light) | — | — | `/asset/54288/` | DISPO |

**Animations incluses par sprite sheet** : Idle, Walk, Run, Attack (1H/2H), Cast, Hit, Death, Block, Kick — 8 directions (S, SW, W, NW, N, NE, E, SE).

**Format** : GIF / PNG, fond transparent, sprites isometriques.

**Priorite MVP** : Barbarian (Light) + Sorceress (Light) suffisent pour le Blood Moor.

---

## 2. Ennemis Act 1 (Blood Moor → Catacombs)

### 2.1 Ennemis exterieurs (Blood Moor, Cold Plains, Stony Field)

| Ennemi | Asset | Taille | Poids | URL | Statut |
|--------|-------|--------|-------|-----|--------|
| Fallen (corps) | Fallen Blood | 4851x2465 | 140 KB | `/sheet/92836/` | DISPO |
| Fallen (sabre) | Fallen Sabre | — | — | `/asset/92828/` | DISPO |
| Fallen Shaman | Fallen Shaman | 3688x4553 | 1.46 MB | `/asset/54314/` | DISPO |
| Fallen Parts (11 sheets) | Fallen Parts | — | — | Categorie "Fallen Parts" | DISPO |
| Quill Rat / Spike Fiend (Big) | Spike Fiend (Big) | 5327x4158 | 2.09 MB | `/sheet/90150/` | DISPO |
| Zombie Parts (25 sheets) | Zombie Parts | — | — | Categorie "Zombie Parts" | DISPO |
| Corrupted Rogue | Rogue Mercenary (base) | 1616x2644 | 919 KB | `/asset/84128/` | DISPO |
| Rogue Parts (14 sheets) | Rogue Parts | — | — | Categorie "Rogue Parts" | DISPO |
| Skeleton | Skeleton (object) | — | — | `/fullview/77573/` | PARTIEL |

### 2.2 Boss Act 1

| Boss | Asset | Taille | Poids | URL | Statut |
|------|-------|--------|-------|-----|--------|
| Andariel | Andariel | 5282x8127 | 7.47 MB | `/asset/54306/` | DISPO |
| Griswold (boss) | Griswold | 2920x1569 | 436 KB | `/asset/54321/` | DISPO |
| Blood Raven | Rogue variant / Cameos | — | — | `/asset/70176/` (cameos) | PARTIEL |

### 2.3 Animations ennemies typiques

Chaque sprite sheet ennemi contient : Idle, Walk, Attack (1-2 types), Hit, Death, Special — 8 directions.

---

## 3. NPCs du Rogue Encampment

| NPC | Role D2 | Taille | Poids | URL | Statut |
|-----|---------|--------|-------|-----|--------|
| Akara | Heal + Potions | 352x1225 | 107 KB | `/asset/83739/` | DISPO |
| Charsi | Forgeron | — | — | Sur page principale | DISPO |
| Kashya | Mercenaires | — | — | Sur page principale | DISPO |
| Deckard Cain | Identification | 1329x702 | 214 KB | `/fullview/65423/` | DISPO |
| Gheed | Marchand/Gambling | — | — | Sur page principale | DISPO |
| Warriv (Act 1) | Transport Act 2 | — | — | Sur page principale | DISPO |
| Warriv (Act 2) | Transport Act 1 | — | — | `/sheet/69954/` | DISPO |
| Rogue Mercenary | Compagnon | 1616x2644 | 919 KB | `/asset/84128/` | DISPO |

**Animations NPC** : Idle, Talk/Dialogue, parfois marche. 8 directions (ou subset).

---

## 4. Tilesets Act 1

### 4.1 Sols

| Tileset | Zone | Taille | Poids | URL | Statut |
|---------|------|--------|-------|-----|--------|
| Graveyard Ground | Burial Grounds | 800x479 | 229 KB | `/asset/56313/` | DISPO |
| Barracks Floor | Inner Cloister | 800x652 | 227 KB | `/fullview/56312/` | DISPO |
| Outdoor Ground | Wilderness gen. | 2402x4667 | 7.83 MB | `/asset/56438/` | DISPO (Act 2, reutilisable) |
| Wilderness Floor | Blood Moor | — | — | Categorie "Act 1 Tiles" | A CHERCHER |
| Cathedral Floor | Catacombs | — | — | Categorie "Act 1 Tiles" | A CHERCHER |

### 4.2 Murs & Structures

| Tileset | Zone | URL | Statut |
|---------|------|-----|--------|
| Act 1 Tiles (categorie) | 63 sheets total | Page principale, section "Tiles" | DISPO |

**Note** : Les tiles sont organises par Act sur spriters-resource. La categorie "Act 1 Tiles" contient les sols, murs, portes, escaliers du Rogue Encampment, Blood Moor, Cold Plains, Stony Field, Dark Wood, Black Marsh, Tamoe Highland, Monastery, Barracks, Jail, Inner Cloister, Cathedral, Catacombs.

---

## 5. Objets & Environnement

| Objet | Usage Act 1 | URL | Statut |
|-------|-------------|-----|--------|
| Barrel | Breakable | `/sheet/72387/` | DISPO |
| Fire (Large) | Camp fire, torches | `/fullview/78100/` | DISPO |
| Mana Well | Rogue Camp | `/asset/78099/` | DISPO |
| Mana and Health Well | Rogue Camp | `/asset/77971/` | DISPO |
| Skull Pile (Chest) | Loot container | `/fullview/77745/` | DISPO |
| Skeleton (Chest) | Loot container | `/fullview/78125/` | DISPO |
| Skeleton (Container) | Decor | `/fullview/77573/` | DISPO |
| Cairn Stones | Tristram portal | `/asset/76751/` | DISPO |
| Dead Amazon | Decor Blood Moor | `/sheet/76804/` | DISPO |
| Cow | Secret level / decor | `/asset/64356/` | DISPO |

**113 objets au total** sur spriters-resource — la categorie "Objects" couvre shrines, chests, torches, wells, barrels, portals, stash, waypoints, etc.

---

## 6. Animaux & Creatures

| Creature | Usage | URL | Statut |
|----------|-------|-----|--------|
| Animals (17 sheets) | Decor, ambiance | Categorie "Animals" | DISPO |
| Cow | Secret level / decor | `/asset/64356/` | DISPO |

---

## 7. Assets existants (placeholders)

Deja dans `mge/assets/` :

| Dossier | Contenu | Utilisable pour |
|---------|---------|-----------------|
| `Dev_assets/Grass_a..h.png` | 8 tiles herbe iso | Blood Moor sol |
| `Dev_assets/water_v01/02.png` | 2 tiles eau | Riviere Blood Moor |
| `Dev_assets/arbre_a/b.png` | 2 sprites arbres | Vegetation Blood Moor |
| `Dev_assets/mur_SE/SW.png` | 2 murs iso | Monastery/Catacombs |
| `Dev_assets/Test_joueur.png` | Placeholder joueur | Remplacement par Barbarian |
| `Dev_assets/Test_mob.png` | Placeholder monstre | Remplacement par Fallen |
| `Dev_assets/Test_elite.png` | Placeholder elite | Remplacement par Fallen Shaman |
| `Dev_assets/Test_boss.png` | Placeholder boss | Remplacement par Andariel |
| `Dev_assets/gobelin.png` | Sprite gobelin | Remplacement par Fallen |
| `Dev_assets/orc.png` | Sprite orc | Pas utilise Act 1 |
| `Dev_assets/npc_village/` | 4 NPCs generiques | Remplacement par Akara/Charsi/etc |
| `Dev_assets/references/diablo2_rogue_encampment_map.png` | Reference carte D2 | Layout guide |
| `Knight_1/` | 12 animations chevalier | Remplacement par Barbarian |
| `tiles/seasons/` | 4 biomes saisonniers | Blood Moor tiles |
| `npc/Peasants_1..4/` | 4 types paysans | NPCs Rogue Camp |

---

## 8. Plan de telechargement prioritaire

### Phase 1 — Blood Moor MVP (priorite immediate)

1. **Barbarian (Light)** — `/asset/54291/` — 3.48 MB
2. **Fallen Blood** — `/sheet/92836/` — 140 KB
3. **Fallen Shaman** — `/asset/54314/` — 1.46 MB
4. **Spike Fiend (Big)** — `/sheet/90150/` — 2.09 MB
5. **Fallen Sabre** — `/asset/92828/`
6. **Graveyard Ground** — `/asset/56313/` — 229 KB (sol terrain Act 1)

### Phase 2 — Rogue Encampment

7. **Akara** — `/asset/83739/` — 107 KB
8. **Deckard Cain** — fullview/65423 — 214 KB
9. **Charsi** — depuis page principale
10. **Kashya** — depuis page principale
11. **Gheed** — depuis page principale
12. **Fire (Large)** — `/fullview/78100/`
13. **Mana and Health Well** — `/asset/77971/`
14. **Barrel** — `/sheet/72387/`

### Phase 3 — Dungeons + Boss

15. **Andariel** — `/asset/54306/` — 7.47 MB
16. **Griswold** — `/asset/54321/` — 436 KB
17. **Barracks Floor** — `/fullview/56312/` — 227 KB
18. **Skeleton containers** — `/fullview/78125/` + `/fullview/77573/`
19. **Rogue Mercenary** — `/asset/84128/` — 919 KB

### Phase 4 — Classes supplementaires

20. **Sorceress (Light)** — `/asset/54300/` — 1.67 MB
21. **Amazon (Light)** — `/asset/54286/`
22. **Paladin (Light)** — `/asset/54299/` (est.)
23. **Necromancer (Light)** — `/asset/54297/`

---

## 9. Format & Integration

### Sprite Sheet Format D2

- **Directions** : 8 (S=0, SW=1, W=2, NW=3, N=4, NE=5, E=6, SE=7)
- **Layout** : Rangees = directions, Colonnes = frames d'animation
- **Fond** : Transparent (alpha)
- **Taille frame typique** : 96x96 a 256x256 px selon le monstre

### Integration dans MGE

1. Telecharger les PNG depuis spriters-resource
2. Placer dans `mge/assets/d2_sprites/` (nouveau dossier)
3. Decouper en frames individuelles via `mge-slicer`
4. Creer les atlas via `mge-packer`
5. Enregistrer dans le registry TOML avec IDs symboliques
6. Remplacer les placeholders dans le code

### Structure cible

```
mge/assets/d2_sprites/
  act1/
    classes/
      barbarian_light.png
      sorceress_light.png
    enemies/
      fallen_blood.png
      fallen_shaman.png
      spike_fiend_big.png
      fallen_sabre.png
      andariel.png
      griswold.png
    npcs/
      akara.png
      charsi.png
      kashya.png
      deckard_cain.png
      gheed.png
      warriv.png
      rogue_mercenary.png
    tiles/
      graveyard_ground.png
      barracks_floor.png
      outdoor_ground.png
    objects/
      barrel.png
      fire_large.png
      mana_well.png
      health_well.png
      skull_pile_chest.png
      skeleton_chest.png
      cairn_stones.png
      dead_amazon.png
    parts/
      fallen_parts/    (11 sheets)
      zombie_parts/    (25 sheets)
      rogue_parts/     (14 sheets)
```

---

## 10. URLs completes de reference

Base URL : `https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/`

| # | Asset | URL complete |
|---|-------|-------------|
| 1 | Barbarian (Light) | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/asset/54291/ |
| 2 | Sorceress (Light) | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/asset/54300/ |
| 3 | Fallen Blood | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/sheet/92836/ |
| 4 | Fallen Sabre | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/asset/92828/ |
| 5 | Fallen Shaman | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/asset/54314/ |
| 6 | Spike Fiend (Big) | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/sheet/90150/ |
| 7 | Andariel | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/asset/54306/ |
| 8 | Griswold | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/asset/54321/ |
| 9 | Akara | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/asset/83739/ |
| 10 | Deckard Cain | https://www.spriters-resource.com/fullview/65423/ |
| 11 | Rogue Mercenary | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/asset/84128/ |
| 12 | Graveyard Ground | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/asset/56313/ |
| 13 | Barracks Floor | https://www.spriters-resource.com/fullview/56312/ |
| 14 | Outdoor Ground | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/asset/56438/ |
| 15 | Barrel | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/sheet/72387/ |
| 16 | Fire (Large) | https://www.spriters-resource.com/fullview/78100/ |
| 17 | Mana Well | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/asset/78099/ |
| 18 | Mana and Health Well | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/asset/77971/ |
| 19 | Skull Pile (Chest) | https://www.spriters-resource.com/fullview/77745/ |
| 20 | Skeleton (Chest) | https://www.spriters-resource.com/fullview/78125/ |
| 21 | Skeleton (Container) | https://www.spriters-resource.com/fullview/77573/ |
| 22 | Cairn Stones | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/asset/76751/ |
| 23 | Dead Amazon | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/sheet/76804/ |
| 24 | Amazon (Light) | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/asset/54286/ |
| 25 | Necromancer (Light) | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/asset/54297/ |
| 26 | Cameos (Blood Raven ref) | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/asset/70176/ |
| 27 | Cow | https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/asset/64356/ |

---

Total : **453 sprite sheets** sur le site, dont **~80 directement utiles pour Act 1**.
Phase 1 MVP (Blood Moor) necessite **6 sprites** prioritaires.
