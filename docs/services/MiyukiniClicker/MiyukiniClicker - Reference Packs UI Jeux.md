# MiyuClicker â€” RÃ©fÃ©rence des packs UI jeux (ui/game_ui_pack)

## Contexte

Le rÃ©pertoire **`ui/game_ui_pack`** contient des packs dâ€™assets UI et graphiques pour jeux. Ce document les inventorie, prÃ©cise leur licence et leur contenu, et les **met en correspondance avec les besoins MiyuClicker** (Document Fondateur, Analyse MarchÃ© et Besoins Toolkits).

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre :** Inventaire des packs prÃ©sents sous `ui/game_ui_pack`, licences, contenu, usage recommandÃ© pour MiyuClicker.
- **Hors pÃ©rimÃ¨tre :** Choix dÃ©finitif dâ€™assets par Ã©cran ; modifications des fichiers dâ€™assets.

---

## 1. Inventaire des packs

### 1.1 CatUIFree

| Attribut | DÃ©tail |
|----------|--------|
| **Chemin** | `ui/game_ui_pack/CatUIFree/` |
| **Contenu** | 2 PNG (dont `CatUIFree/free.png`) ; structure minimale. |
| **Licence** | Non documentÃ©e dans le pack â€” **Ã  vÃ©rifier** avant usage commercial. |
| **Usage MiyuClicker** | RÃ©server pour tests ou vÃ©rifier licence ; ne pas utiliser en production sans clarification. |

---

### 1.2 Cute_Fantasy (Kenmi-art)

| Attribut | DÃ©tail |
|----------|--------|
| **Chemin** | `ui/game_ui_pack/Cute_Fantasy/Cute_Fantasy/` |
| **Licence** | Usage commercial et non commercial autorisÃ©. Modification autorisÃ©e. **Interdiction de redistribution / revente** (mÃªme modifiÃ©). |
| **Source** | read_me.txt ; Kenmi-art, itch.io (Cute Fantasy RPG). |

**Contenu (sprites, tuiles, dÃ©cors) :**

| CatÃ©gorie | DÃ©tail | Pertinence MiyuClicker |
|-----------|--------|-------------------------|
| **Animals** | ~98 PNG | Faune, ambiance monde. |
| **Buildings** | ~179 PNG | BÃ¢timents ville / citÃ© â€” **gestion, citÃ©s-Ã‰tats**. |
| **Crops** | Arbres fruitiers, rÃ©coltes, vignes | **Ressource nourriture**, fermes. |
| **Enemies** | ~23 PNG | Ennemis, unitÃ©s adverses. |
| **Icons** | Food, Resources, Tools, Other (Outline + No Outline) | **IcÃ´nes ressources** (nourriture, matiÃ¨res, outils). |
| **NPCs (Premade)** | Bartender, Chef, Farmer, Fisherman, Lumberjack, Miner | **Â« Gens Â»** par mÃ©tier (affectation visuelle). |
| **Outdoor decoration** | ~137 PNG | DÃ©cors carte, villages. |
| **Player** | ~143 PNG | Personnage / unitÃ© joueur. |
| **Tiles** | Beach, Bridge, Cave, Cliff, Cobble_Road, FarmLand, Grass, Water, Waterfall, Wooden_Deck | **Carte stratÃ©gique** (tuiles terrain, routes, eau). |
| **Trees** | ChÃªne, bouleau, sapin, fruitiers (petit/moyen/grand), particules | Carte, forÃªts, ralentissements (beta). |
| **Weather effects** | Nuages, pluie, vent | Ambiance (optionnel). |

**Usage recommandÃ© :** Sprites monde (bÃ¢timents, tuiles, NPCs, icÃ´nes ressources), carte stratÃ©gique (tiles), cohÃ©rence visuelle Â« fantasy Â» pour gestion et conquÃªte.

---

### 1.3 Cute_Fantasy_UI (Kenmi-art)

| Attribut | DÃ©tail |
|----------|--------|
| **Chemin** | `ui/game_ui_pack/Cute_Fantasy_UI/Cute_Fantasy_UI/` |
| **Licence** | MÃªme principe que Cute_Fantasy : commercial / non commercial, modification OK, **pas de redistribution / revente**. |
| **Source** | read_me.txt (Cute Fantasy Dungeons). |

**Contenu (UI) :**

| Fichier / thÃ¨me | RÃ´le | Pertinence MiyuClicker |
|-----------------|------|-------------------------|
| **Fonts** | Cute_Fantasy_Font_5x9.png, CuteFantasy-5x9.ttf | Typo in-game (Dioxus peut charger TTF). |
| **UI_ALL.png** | Assemblage UI | RÃ©fÃ©rence layout. |
| **UI_Bars.png** | Barres (vie, ressource, chargement) | **Barres ressources**, moral, troupes. |
| **UI_Buttons.png** | Boutons | Menus, actions (allouer gens, envoyer troupes). |
| **UI_Button_Icons.png** | IcÃ´nes boutons | Raccourcis, actions. |
| **UI_Frames.png** | Cadres / fenÃªtres | Panels, fenÃªtres modales. |
| **UI_Icons.png** | IcÃ´nes interface | Ressources, Ã©tats. |
| **UI_Pop_Up.png** | Pop-ups | Notifications, confirmations. |
| **UI_Premade.png** | Composants prÃªts Ã  lâ€™emploi | Blocs dâ€™interface. |
| **UI_Ribbons.png** | Rubans | Titres, sÃ©parateurs. |
| **UI_Selectors.png** | SÃ©lecteurs | Liste citÃ©s, unitÃ©s. |
| **UI_Sliders.png** | Sliders | **Allocation de gens** (sliders). |
| **Book_UI.png** | Interface type livre | Ã‰crans aide / codex (optionnel). |
| **Loading_Icon.png** | Indicateur chargement | Ã‰cran de chargement. |
| **Pointer_Click_Anim.png** | Animation clic | Curseur / feedback clic. |

**Usage recommandÃ© :** **Pack UI principal** pour MiyuClicker â€” barres, boutons, cadres, sliders, icÃ´nes ; cohÃ©rent avec Cute_Fantasy (mÃªme univers visuel).

---

### 1.4 modernuserinterface-win

| Attribut | DÃ©tail |
|----------|--------|
| **Chemin** | `ui/game_ui_pack/modernuserinterface-win/` |
| **Licence** | Usage commercial et non commercial autorisÃ© (**sauf NFT**). Modification autorisÃ©e. **Interdiction de revendre / redistribuer** les assets. **CrÃ©dits requis.** |
| **Source** | LICENSE.txt, READ_ME.txt. |

**Contenu :**

| Ã‰lÃ©ment | DÃ©tail | Pertinence MiyuClicker |
|---------|--------|-------------------------|
| **16x16 / 32x32 / 48x48** | Variantes de rÃ©solution | UI adaptable (diffÃ©rentes densitÃ©s). |
| **Modern_UI_Style_1 / 2** | Styles UI modernes | Alternative look Â« moderne Â» (non fantasy). |
| **Modern_UI_Gamepad** | Indicateurs manette | Optionnel (support manette). |
| **Animated (GIF)** | Boutons poubelle (trash) animÃ©s | Exemples dâ€™**animations UI** (Ã©quivalent possible en spritesheet pour Dioxus). |
| **Portrait_Generator** | Accessories, Eyes, Hairstyles, Skins (PNG + Aseprite) | **Portraits** unitÃ©s / hÃ©ros (beta v1.0). |
| **Portrait_Generator_ase** | Fichiers .aseprite | Ã‰dition / variantes de portraits. |

**Usage recommandÃ© :** UI alternative Â« moderne Â» ; **Portrait Generator** pour hÃ©ros / gÃ©nÃ©raux en beta v1.0 ; rÃ©fÃ©rence pour **animations UI** (GIF â†’ spritesheet pour Dioxus).

#### RÃ©fÃ©rence 32x32 (implÃ©mentation MiyuClicker)

Pour lâ€™UI MiyuClicker, le pack **32x32** est la cible recommandÃ©e (barres, boutons, cadres, indicateurs QtÃ© actuel/QtÃ© max). Chemins relatifs Ã  la racine du dÃ©pÃ´t :

| Constante / usage | Chemin |
|-------------------|--------|
| **Base 32x32** | `ui/game_ui_pack/modernuserinterface-win/32x32/` |
| **Spritesheet Style 1** | `ui/game_ui_pack/modernuserinterface-win/32x32/Modern_UI_Style_1_32x32.png` |
| **Spritesheet Style 2** | `ui/game_ui_pack/modernuserinterface-win/32x32/Modern_UI_Style_2_32x32.png` |
| **Gamepad (optionnel)** | `ui/game_ui_pack/modernuserinterface-win/32x32/Modern_UI_Gamepad_32x32.png` |
| **Animated (GIF, rÃ©fÃ©rence)** | `ui/game_ui_pack/modernuserinterface-win/32x32/Animated_32x32/` |

Les spritesheets Style 1 et Style 2 contiennent boutons, barres, cadres et indicateurs (type Â« QtÃ© actuel / QtÃ© max Â»). Lors de lâ€™intÃ©gration des textures dans Dioxus, utiliser les constantes dÃ©finies dans le crate (module `ui_assets`) pour rÃ©soudre ces chemins.

---

### 1.5 Ultimate Pixel Art Fantasy RPG Icon Pack (Clockwork Raven)

| Attribut | DÃ©tail |
|----------|--------|
| **Chemin** | `ui/game_ui_pack/UltimatePixelArtFantasyRPGIconPack/` |
| **Licence** | Usage commercial et non commercial autorisÃ©. Modification autorisÃ©e. **Interdiction de revendre / redistribuer** les assets. Attribution bienvenue. |
| **Source** | Clockwork Raven Studios, itch.io (5600+ Ultimate Pixel Art Fantasy RPG Icon Pack). |

**Contenu :** 5616 icÃ´nes fantasy en pixel art (16x16, 24x24, 32x32, 64x64 ; Classic, Dark Outline, Light Outline). ThÃ¨mes : Conditions/States, Attributes/Stats, Basic Menu, General Items and Tools, Gems and Jewels, Crafting Materials, Monster Hunting and Drops, Food/Crops and Beverages, Alchemy Materials, Magic Potions, Weapons, Shields, Armor and Clothing, Accessories, Masks.

**Index et renommage :** Les icÃ´nes du dossier **Dark Outline / 32X32** ont Ã©tÃ© renommÃ©es `N.png` â†’ `icon_NNNN.png` (4 chiffres) pour un tri cohÃ©rent. Un **index JSON** (`index.json`) et une **documentation d'index** listent les icÃ´nes et les noms sÃ©mantiques des entrÃ©es analysÃ©es (ex. `poison_skull`, `bread_food`, `mana`, `gem_cyan`).

| Ã‰lÃ©ment | DÃ©tail |
|--------|--------|
| **Index JSON** | `ui/game_ui_pack/UltimatePixelArtFantasyRPGIconPack/index.json` |
| **Doc index** | [MiyuClicker - UltimatePixelArtFantasyRPGIconPack - Index Icones](MiyukiniClicker%20-%20UltimatePixelArtFantasyRPGIconPack%20-%20Index%20Icones.md) |
| **Scripts** | `rename_and_index.py` (Python), `rename_icons.ps1` (PowerShell) pour renommer d'autres dossiers. |

**Usage recommandÃ© :** **IcÃ´nes ressources, statuts, menu, objets, potions, armes** pour MiyuClicker ; recherche par id ou par nom sÃ©mantique via l'index.

---

### 1.6 Tiny RPG Character Asset Pack v1.03b (Full 20 Characters)

| Attribut | DÃ©tail |
|----------|--------|
| **Chemin** | `ui/game_ui_pack/Tiny RPG Character Asset Pack v1.03b -Full 20 Characters/Tiny RPG Character Asset Pack v1.03 -Full 20 Characters/` |
| **Licence** | **Non documentÃ©e** dans le rÃ©pertoire analysÃ© â€” **Ã  vÃ©rifier** (origine itch.io ou autre). |
| **Contenu** | 20 personnages (Archer, Knight, Orc, Skeleton, Priest, Wizard, etc.), projectiles (flÃ¨ches, magie), fichiers Aseprite. |

**Contenu dÃ©taillÃ© :**

| CatÃ©gorie | DÃ©tail | Pertinence MiyuClicker |
|-----------|--------|-------------------------|
| **Characters(100x100)** | ~343 PNG (sprites par personnage) | **UnitÃ©s** (soldats, types de troupes), **hÃ©ros** (beta). |
| **Arrow(Projectile)** | FlÃ¨ches 100x100 et 32x32 | Projectiles combat (RTS / temps rÃ©el beta). |
| **Magic(Projectile)** | Effets Priest, Wizard | Effets de combat / hÃ©ros. |
| **Aseprite file** | 20 .aseprite | Ã‰dition, animations, variantes. |

**Usage recommandÃ© :** **Sprites dâ€™unitÃ©s** (soldats, types) et **hÃ©ros** ; projectiles pour rÃ©solution combat avancÃ©e. **Ne pas utiliser en production sans confirmation de licence.**

---

### 1.7 ui-icn_fantasy-weapons_01 (Misbug)

| Attribut | DÃ©tail |
|----------|--------|
| **Chemin** | `ui/game_ui_pack/ui-icn_fantasy-weapons_01/` |
| **Licence** | Contrat dans `Misbug's Assets Licence Agreement.pdf` â€” **Ã  consulter** pour usage commercial et attribution. |
| **Contenu** | IcÃ´nes dâ€™armes fantasy (basic_Icons, drop_shadow, white_outline) : ~64+ armes (Ã©pÃ©e, hache, arc, bouclier, etc.). |

**Usage recommandÃ© :** **IcÃ´nes dâ€™armes** pour Ã©quipement, troupes, type dâ€™unitÃ© ; cohÃ©rence fantasy. VÃ©rifier la licence (PDF) avant usage commercial.

---

## 2. SynthÃ¨se par besoin MiyuClicker

| Besoin | Packs Ã  privilÃ©gier | Remarque |
|--------|----------------------|-----------|
| **UI (barres, boutons, cadres, sliders)** | **Cute_Fantasy_UI** (principal) ; modernuserinterface-win (alternative moderne) | Cute_Fantasy_UI alignÃ© avec Cute_Fantasy. |
| **IcÃ´nes ressources (nourriture, matiÃ¨res, outils)** | **Cute_Fantasy** â†’ Icons ; **UltimatePixelArtFantasyRPGIconPack** (index JSON, nom sÃ©mantique) | Spritesheets Ã  dÃ©couper ou chargement par id/nom via index. |
| **Carte (tuiles terrain, routes, eau)** | **Cute_Fantasy** â†’ Tiles (Grass, Cobble_Road, Water, Bridge, etc.) | Carte stratÃ©gique, checkpoints (beta). |
| **BÃ¢timents / citÃ©s** | **Cute_Fantasy** â†’ Buildings | ReprÃ©sentation citÃ©s-Ã‰tats, gestion. |
| **Â« Gens Â» / mÃ©tiers** | **Cute_Fantasy** â†’ NPCs (Premade) ; **Tiny RPG** â†’ Characters (si licence OK) | Affectation visuelle par rÃ´le. |
| **UnitÃ©s / soldats** | **Tiny RPG** â†’ Characters (si licence OK) ; Cute_Fantasy â†’ Enemies, Player | Troupes, types dâ€™unitÃ©s. |
| **Armes / Ã©quipement (icÃ´nes)** | **ui-icn_fantasy-weapons_01** (aprÃ¨s vÃ©rif. licence) | Type dâ€™unitÃ©, Ã©quipement. |
| **Portraits hÃ©ros (beta v1.0)** | **modernuserinterface-win** â†’ Portrait_Generator | GÃ©nÃ©raux, hÃ©ros. |
| **Animations UI (feedback)** | Cute_Fantasy_UI â†’ Pointer_Click_Anim ; modernuserinterface-win â†’ GIF (rÃ©fÃ©rence spritesheet) | Toolkit animation par frame. |
| **Polices** | Cute_Fantasy_UI â†’ Fonts (ttf) | Dioxus : chargement de polices. |

---

## 3. RÃ¨gles dâ€™usage et conformitÃ©

| RÃ¨gle | DÃ©tail |
|-------|--------|
| **Pas de redistribution des assets** | Cute_Fantasy, Cute_Fantasy_UI, modernuserinterface-win : interdiction de revendre/redistribuer les fichiers bruts. Les utiliser **dans** le jeu compilÃ© est autorisÃ© selon leurs licences. |
| **CrÃ©dits** | modernuserinterface-win : **crÃ©dits requis**. PrÃ©voir Ã©cran Â« CrÃ©dits Â» ou fichier LICENSES avec noms des packs. |
| **VÃ©rification licence** | CatUIFree, Tiny RPG, ui-icn_fantasy-weapons_01 : confirmer licence (et attribution si nÃ©cessaire) avant usage en production. |
| **CohÃ©rence visuelle** | Pour une dÃ©mo homogÃ¨ne : **Cute_Fantasy + Cute_Fantasy_UI** (mÃªme univers) en cÅ“ur ; autres packs en complÃ©ment selon besoin. |

---

## 4. RÃ©fÃ©rences

- [MiyuClicker - Document Fondateur](MiyukiniClicker%20-%20Document%20Fondateur.md)
- [MiyuClicker - Analyse Marche et Besoins Toolkits](MiyukiniClicker%20-%20Analyse%20Marche%20et%20Besoins%20Toolkits.md)
- RÃ©pertoire physique : `ui/game_ui_pack/`

---

**Document crÃ©Ã© le :** 2026-02-01  
**Statut :** RÃ©fÃ©rence des packs UI jeux pour MiyuClicker

