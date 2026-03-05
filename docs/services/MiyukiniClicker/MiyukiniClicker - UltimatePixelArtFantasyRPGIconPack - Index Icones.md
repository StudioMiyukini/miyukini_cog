# MiyuClicker â€” Index des icÃ´nes Ultimate Pixel Art Fantasy RPG Icon Pack

## Contexte

Le pack **Ultimate Pixel Art Fantasy RPG Icon Pack** (Clockwork Raven Studios) contient **5616 icÃ´nes** fantasy (conditions, stats, menu, objets, gemmes, crafting, nourriture, potions, armes, boucliers, armures, accessoires, masques). Les fichiers Ã©taient nommÃ©s `1.png` Ã  `5616.png` ; ils ont Ã©tÃ© renommÃ©s en **`icon_NNNN.png`** (4 chiffres) pour un tri alphabÃ©tique cohÃ©rent et indexÃ©s dans un fichier JSON pour une utilisation facile par nom sÃ©mantique ou catÃ©gorie.

## PortÃ©e / Scope

- **Applicable Ã  :** RÃ©fÃ©rence et chargement dâ€™icÃ´nes pour MiyuClicker et tout projet utilisant ce pack.
- **Audience :** DÃ©veloppeurs, designers, agents IA.
- **Statut :** Document de rÃ©fÃ©rence â€” index Ã©volutif (noms sÃ©mantiques complÃ©tÃ©s au fil des analyses).

---

## 1. Structure du pack

| Ã‰lÃ©ment | DÃ©tail |
|--------|--------|
| **Chemin** | `ui/game_ui_pack/UltimatePixelArtFantasyRPGIconPack/` |
| **Source** | Clockwork Raven Studios (itch.io) |
| **Index JSON** | `ui/game_ui_pack/UltimatePixelArtFantasyRPGIconPack/index.json` |
| **Variantes** | Classic, Dark Outline, Light Outline |
| **RÃ©solutions** | 16x16, 24x24, 32x32, 64x64 |

Le renommage **`N.png` â†’ `icon_NNNN.png`** a Ã©tÃ© appliquÃ© au dossier **Dark Outline / 32X32**. Les autres dossiers (Classic, Light Outline, autres rÃ©solutions) conservent les noms numÃ©riques dâ€™origine ; le mÃªme schÃ©ma de renommage peut Ãªtre appliquÃ© via les scripts fournis.

---

## 2. Format de lâ€™index (index.json)

Chaque entrÃ©e dâ€™icÃ´ne contient :

| Champ | Type | Description |
|-------|------|-------------|
| `id` | nombre | Identifiant dâ€™origine (1 Ã  5616). |
| `file_renamed` | chaÃ®ne | Nom du fichier aprÃ¨s renommage (`icon_0001.png`, â€¦). |
| `name` | chaÃ®ne \| null | Nom sÃ©mantique pour usage en code (ex. `poison_skull`). |
| `category` | chaÃ®ne \| null | CatÃ©gorie du pack (voir ciâ€‘dessous). |
| `description_fr` | chaÃ®ne \| null | Description courte en franÃ§ais. |

**CatÃ©gories** (alignÃ©es sur les thÃ¨mes du pack itch.io) :  
`conditions_states`, `attributes_stats`, `basic_menu`, `general_items_tools`, `gems_jewels`, `crafting_materials`, `monster_hunting_drops`, `food_crops_beverages`, `alchemy_materials`, `magic_potions`, `weapons`, `shields`, `armor_clothing`, `accessories`, `masks`.

---

## 3. IcÃ´nes analysÃ©es et renommÃ©es (Ã©chantillon)

Les icÃ´nes suivantes ont Ã©tÃ© analysÃ©es visuellement et ont un `name` et une `category` renseignÃ©s dans lâ€™index. Les autres entrÃ©es ont `name` et `category` Ã  `null` ; elles peuvent Ãªtre complÃ©tÃ©es progressivement.

| id | file_renamed | name | category | description_fr |
|----|--------------|------|----------|----------------|
| 1 | icon_0001.png | poison_skull | conditions_states | TÃªte de mort / poison / danger (bulle) |
| 2 | icon_0002.png | skull_warning | conditions_states | CrÃ¢ne avertissement (bulle) |
| 50 | icon_0050.png | metal_scraps | crafting_materials | Ferraille / piÃ¨ces mÃ©talliques |
| 100 | icon_0100.png | observation_eye | attributes_stats | Å’il observation / dÃ©tection (bulle) |
| 200 | icon_0200.png | bread_food | food_crops_beverages | Pain / ration / nourriture |
| 500 | icon_0500.png | mana | magic_potions | Mana / Ã©nergie magique |
| 1000 | icon_1000.png | gem_cyan | gems_jewels | Gemme / cristal cyan |

---

## 4. Utilisation

- **Par id :** charger `Dark Outline/32X32/icon_NNNN.png` avec `NNNN = id` sur 4 chiffres.
- **Par nom sÃ©mantique :** interroger `index.json` pour obtenir `file_renamed` Ã  partir de `name` (ex. `poison_skull` â†’ `icon_0001.png`).
- **Par catÃ©gorie :** filtrer les entrÃ©es de `icons` par `category` pour lister les icÃ´nes dâ€™une thÃ©matique.

Scripts fournis dans le pack :

- **`rename_and_index.py`** (Python 3) : renommage dans plusieurs dossiers + rÃ©gÃ©nÃ©ration de lâ€™index Ã  partir des icÃ´nes connues.
- **`rename_icons.ps1`** (PowerShell) : renommage `N.png` â†’ `icon_NNNN.png` dans un dossier donnÃ© (ex. `Dark Outline\32X32`).

---

## 5. RÃ©fÃ©rences

- [MiyuClicker - Reference Packs UI Jeux](MiyukiniClicker%20-%20Reference%20Packs%20UI%20Jeux.md) â€” Ã  mettre Ã  jour avec ce pack et le lien vers le prÃ©sent index.
- Pack itch.io : [5600+ Ultimate Pixel Art Fantasy RPG Icon Pack](https://clockworkraven.itch.io/5600-ultimate-pixel-art-fantasy-rpg-icon-pack)
- RÃ©pertoire physique : `ui/game_ui_pack/UltimatePixelArtFantasyRPGIconPack/`

---

**Date de crÃ©ation :** 2026-02-01  
**Statut :** Index des icÃ´nes â€” renommage et index JSON en place (Dark Outline 32X32)

