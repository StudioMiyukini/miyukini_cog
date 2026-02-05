# MiyuClicker — Index des icônes Ultimate Pixel Art Fantasy RPG Icon Pack

## Contexte

Le pack **Ultimate Pixel Art Fantasy RPG Icon Pack** (Clockwork Raven Studios) contient **5616 icônes** fantasy (conditions, stats, menu, objets, gemmes, crafting, nourriture, potions, armes, boucliers, armures, accessoires, masques). Les fichiers étaient nommés `1.png` à `5616.png` ; ils ont été renommés en **`icon_NNNN.png`** (4 chiffres) pour un tri alphabétique cohérent et indexés dans un fichier JSON pour une utilisation facile par nom sémantique ou catégorie.

## Portée / Scope

- **Applicable à :** Référence et chargement d’icônes pour MiyuClicker et tout projet utilisant ce pack.
- **Audience :** Développeurs, designers, agents IA.
- **Statut :** Document de référence — index évolutif (noms sémantiques complétés au fil des analyses).

---

## 1. Structure du pack

| Élément | Détail |
|--------|--------|
| **Chemin** | `ui/game_ui_pack/UltimatePixelArtFantasyRPGIconPack/` |
| **Source** | Clockwork Raven Studios (itch.io) |
| **Index JSON** | `ui/game_ui_pack/UltimatePixelArtFantasyRPGIconPack/index.json` |
| **Variantes** | Classic, Dark Outline, Light Outline |
| **Résolutions** | 16x16, 24x24, 32x32, 64x64 |

Le renommage **`N.png` → `icon_NNNN.png`** a été appliqué au dossier **Dark Outline / 32X32**. Les autres dossiers (Classic, Light Outline, autres résolutions) conservent les noms numériques d’origine ; le même schéma de renommage peut être appliqué via les scripts fournis.

---

## 2. Format de l’index (index.json)

Chaque entrée d’icône contient :

| Champ | Type | Description |
|-------|------|-------------|
| `id` | nombre | Identifiant d’origine (1 à 5616). |
| `file_renamed` | chaîne | Nom du fichier après renommage (`icon_0001.png`, …). |
| `name` | chaîne \| null | Nom sémantique pour usage en code (ex. `poison_skull`). |
| `category` | chaîne \| null | Catégorie du pack (voir ci‑dessous). |
| `description_fr` | chaîne \| null | Description courte en français. |

**Catégories** (alignées sur les thèmes du pack itch.io) :  
`conditions_states`, `attributes_stats`, `basic_menu`, `general_items_tools`, `gems_jewels`, `crafting_materials`, `monster_hunting_drops`, `food_crops_beverages`, `alchemy_materials`, `magic_potions`, `weapons`, `shields`, `armor_clothing`, `accessories`, `masks`.

---

## 3. Icônes analysées et renommées (échantillon)

Les icônes suivantes ont été analysées visuellement et ont un `name` et une `category` renseignés dans l’index. Les autres entrées ont `name` et `category` à `null` ; elles peuvent être complétées progressivement.

| id | file_renamed | name | category | description_fr |
|----|--------------|------|----------|----------------|
| 1 | icon_0001.png | poison_skull | conditions_states | Tête de mort / poison / danger (bulle) |
| 2 | icon_0002.png | skull_warning | conditions_states | Crâne avertissement (bulle) |
| 50 | icon_0050.png | metal_scraps | crafting_materials | Ferraille / pièces métalliques |
| 100 | icon_0100.png | observation_eye | attributes_stats | Œil observation / détection (bulle) |
| 200 | icon_0200.png | bread_food | food_crops_beverages | Pain / ration / nourriture |
| 500 | icon_0500.png | mana | magic_potions | Mana / énergie magique |
| 1000 | icon_1000.png | gem_cyan | gems_jewels | Gemme / cristal cyan |

---

## 4. Utilisation

- **Par id :** charger `Dark Outline/32X32/icon_NNNN.png` avec `NNNN = id` sur 4 chiffres.
- **Par nom sémantique :** interroger `index.json` pour obtenir `file_renamed` à partir de `name` (ex. `poison_skull` → `icon_0001.png`).
- **Par catégorie :** filtrer les entrées de `icons` par `category` pour lister les icônes d’une thématique.

Scripts fournis dans le pack :

- **`rename_and_index.py`** (Python 3) : renommage dans plusieurs dossiers + régénération de l’index à partir des icônes connues.
- **`rename_icons.ps1`** (PowerShell) : renommage `N.png` → `icon_NNNN.png` dans un dossier donné (ex. `Dark Outline\32X32`).

---

## 5. Références

- [MiyuClicker - Reference Packs UI Jeux](MiyuClicker%20-%20Reference%20Packs%20UI%20Jeux.md) — à mettre à jour avec ce pack et le lien vers le présent index.
- Pack itch.io : [5600+ Ultimate Pixel Art Fantasy RPG Icon Pack](https://clockworkraven.itch.io/5600-ultimate-pixel-art-fantasy-rpg-icon-pack)
- Répertoire physique : `ui/game_ui_pack/UltimatePixelArtFantasyRPGIconPack/`

---

**Date de création :** 2026-02-01  
**Statut :** Index des icônes — renommage et index JSON en place (Dark Outline 32X32)
