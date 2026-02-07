# MiyuClicker — Référence des packs UI jeux (ui/game_ui_pack)

## Contexte

Le répertoire **`ui/game_ui_pack`** contient des packs d’assets UI et graphiques pour jeux. Ce document les inventorie, précise leur licence et leur contenu, et les **met en correspondance avec les besoins MiyuClicker** (Document Fondateur, Analyse Marché et Besoins Toolkits).

## Portée / Scope

- **Périmètre :** Inventaire des packs présents sous `ui/game_ui_pack`, licences, contenu, usage recommandé pour MiyuClicker.
- **Hors périmètre :** Choix définitif d’assets par écran ; modifications des fichiers d’assets.

---

## 1. Inventaire des packs

### 1.1 CatUIFree

| Attribut | Détail |
|----------|--------|
| **Chemin** | `ui/game_ui_pack/CatUIFree/` |
| **Contenu** | 2 PNG (dont `CatUIFree/free.png`) ; structure minimale. |
| **Licence** | Non documentée dans le pack — **à vérifier** avant usage commercial. |
| **Usage MiyuClicker** | Réserver pour tests ou vérifier licence ; ne pas utiliser en production sans clarification. |

---

### 1.2 Cute_Fantasy (Kenmi-art)

| Attribut | Détail |
|----------|--------|
| **Chemin** | `ui/game_ui_pack/Cute_Fantasy/Cute_Fantasy/` |
| **Licence** | Usage commercial et non commercial autorisé. Modification autorisée. **Interdiction de redistribution / revente** (même modifié). |
| **Source** | read_me.txt ; Kenmi-art, itch.io (Cute Fantasy RPG). |

**Contenu (sprites, tuiles, décors) :**

| Catégorie | Détail | Pertinence MiyuClicker |
|-----------|--------|-------------------------|
| **Animals** | ~98 PNG | Faune, ambiance monde. |
| **Buildings** | ~179 PNG | Bâtiments ville / cité — **gestion, cités-États**. |
| **Crops** | Arbres fruitiers, récoltes, vignes | **Ressource nourriture**, fermes. |
| **Enemies** | ~23 PNG | Ennemis, unités adverses. |
| **Icons** | Food, Resources, Tools, Other (Outline + No Outline) | **Icônes ressources** (nourriture, matières, outils). |
| **NPCs (Premade)** | Bartender, Chef, Farmer, Fisherman, Lumberjack, Miner | **« Gens »** par métier (affectation visuelle). |
| **Outdoor decoration** | ~137 PNG | Décors carte, villages. |
| **Player** | ~143 PNG | Personnage / unité joueur. |
| **Tiles** | Beach, Bridge, Cave, Cliff, Cobble_Road, FarmLand, Grass, Water, Waterfall, Wooden_Deck | **Carte stratégique** (tuiles terrain, routes, eau). |
| **Trees** | Chêne, bouleau, sapin, fruitiers (petit/moyen/grand), particules | Carte, forêts, ralentissements (beta). |
| **Weather effects** | Nuages, pluie, vent | Ambiance (optionnel). |

**Usage recommandé :** Sprites monde (bâtiments, tuiles, NPCs, icônes ressources), carte stratégique (tiles), cohérence visuelle « fantasy » pour gestion et conquête.

---

### 1.3 Cute_Fantasy_UI (Kenmi-art)

| Attribut | Détail |
|----------|--------|
| **Chemin** | `ui/game_ui_pack/Cute_Fantasy_UI/Cute_Fantasy_UI/` |
| **Licence** | Même principe que Cute_Fantasy : commercial / non commercial, modification OK, **pas de redistribution / revente**. |
| **Source** | read_me.txt (Cute Fantasy Dungeons). |

**Contenu (UI) :**

| Fichier / thème | Rôle | Pertinence MiyuClicker |
|-----------------|------|-------------------------|
| **Fonts** | Cute_Fantasy_Font_5x9.png, CuteFantasy-5x9.ttf | Typo in-game (egui peut charger TTF). |
| **UI_ALL.png** | Assemblage UI | Référence layout. |
| **UI_Bars.png** | Barres (vie, ressource, chargement) | **Barres ressources**, moral, troupes. |
| **UI_Buttons.png** | Boutons | Menus, actions (allouer gens, envoyer troupes). |
| **UI_Button_Icons.png** | Icônes boutons | Raccourcis, actions. |
| **UI_Frames.png** | Cadres / fenêtres | Panels, fenêtres modales. |
| **UI_Icons.png** | Icônes interface | Ressources, états. |
| **UI_Pop_Up.png** | Pop-ups | Notifications, confirmations. |
| **UI_Premade.png** | Composants prêts à l’emploi | Blocs d’interface. |
| **UI_Ribbons.png** | Rubans | Titres, séparateurs. |
| **UI_Selectors.png** | Sélecteurs | Liste cités, unités. |
| **UI_Sliders.png** | Sliders | **Allocation de gens** (sliders). |
| **Book_UI.png** | Interface type livre | Écrans aide / codex (optionnel). |
| **Loading_Icon.png** | Indicateur chargement | Écran de chargement. |
| **Pointer_Click_Anim.png** | Animation clic | Curseur / feedback clic. |

**Usage recommandé :** **Pack UI principal** pour MiyuClicker — barres, boutons, cadres, sliders, icônes ; cohérent avec Cute_Fantasy (même univers visuel).

---

### 1.4 modernuserinterface-win

| Attribut | Détail |
|----------|--------|
| **Chemin** | `ui/game_ui_pack/modernuserinterface-win/` |
| **Licence** | Usage commercial et non commercial autorisé (**sauf NFT**). Modification autorisée. **Interdiction de revendre / redistribuer** les assets. **Crédits requis.** |
| **Source** | LICENSE.txt, READ_ME.txt. |

**Contenu :**

| Élément | Détail | Pertinence MiyuClicker |
|---------|--------|-------------------------|
| **16x16 / 32x32 / 48x48** | Variantes de résolution | UI adaptable (différentes densités). |
| **Modern_UI_Style_1 / 2** | Styles UI modernes | Alternative look « moderne » (non fantasy). |
| **Modern_UI_Gamepad** | Indicateurs manette | Optionnel (support manette). |
| **Animated (GIF)** | Boutons poubelle (trash) animés | Exemples d’**animations UI** (équivalent possible en spritesheet pour egui). |
| **Portrait_Generator** | Accessories, Eyes, Hairstyles, Skins (PNG + Aseprite) | **Portraits** unités / héros (beta v1.0). |
| **Portrait_Generator_ase** | Fichiers .aseprite | Édition / variantes de portraits. |

**Usage recommandé :** UI alternative « moderne » ; **Portrait Generator** pour héros / généraux en beta v1.0 ; référence pour **animations UI** (GIF → spritesheet pour egui).

#### Référence 32x32 (implémentation MiyuClicker)

Pour l’UI MiyuClicker, le pack **32x32** est la cible recommandée (barres, boutons, cadres, indicateurs Qté actuel/Qté max). Chemins relatifs à la racine du dépôt :

| Constante / usage | Chemin |
|-------------------|--------|
| **Base 32x32** | `ui/game_ui_pack/modernuserinterface-win/32x32/` |
| **Spritesheet Style 1** | `ui/game_ui_pack/modernuserinterface-win/32x32/Modern_UI_Style_1_32x32.png` |
| **Spritesheet Style 2** | `ui/game_ui_pack/modernuserinterface-win/32x32/Modern_UI_Style_2_32x32.png` |
| **Gamepad (optionnel)** | `ui/game_ui_pack/modernuserinterface-win/32x32/Modern_UI_Gamepad_32x32.png` |
| **Animated (GIF, référence)** | `ui/game_ui_pack/modernuserinterface-win/32x32/Animated_32x32/` |

Les spritesheets Style 1 et Style 2 contiennent boutons, barres, cadres et indicateurs (type « Qté actuel / Qté max »). Lors de l’intégration des textures dans egui, utiliser les constantes définies dans le crate (module `ui_assets`) pour résoudre ces chemins.

---

### 1.5 Ultimate Pixel Art Fantasy RPG Icon Pack (Clockwork Raven)

| Attribut | Détail |
|----------|--------|
| **Chemin** | `ui/game_ui_pack/UltimatePixelArtFantasyRPGIconPack/` |
| **Licence** | Usage commercial et non commercial autorisé. Modification autorisée. **Interdiction de revendre / redistribuer** les assets. Attribution bienvenue. |
| **Source** | Clockwork Raven Studios, itch.io (5600+ Ultimate Pixel Art Fantasy RPG Icon Pack). |

**Contenu :** 5616 icônes fantasy en pixel art (16x16, 24x24, 32x32, 64x64 ; Classic, Dark Outline, Light Outline). Thèmes : Conditions/States, Attributes/Stats, Basic Menu, General Items and Tools, Gems and Jewels, Crafting Materials, Monster Hunting and Drops, Food/Crops and Beverages, Alchemy Materials, Magic Potions, Weapons, Shields, Armor and Clothing, Accessories, Masks.

**Index et renommage :** Les icônes du dossier **Dark Outline / 32X32** ont été renommées `N.png` → `icon_NNNN.png` (4 chiffres) pour un tri cohérent. Un **index JSON** (`index.json`) et une **documentation d'index** listent les icônes et les noms sémantiques des entrées analysées (ex. `poison_skull`, `bread_food`, `mana`, `gem_cyan`).

| Élément | Détail |
|--------|--------|
| **Index JSON** | `ui/game_ui_pack/UltimatePixelArtFantasyRPGIconPack/index.json` |
| **Doc index** | [MiyuClicker - UltimatePixelArtFantasyRPGIconPack - Index Icones](MiyuClicker%20-%20UltimatePixelArtFantasyRPGIconPack%20-%20Index%20Icones.md) |
| **Scripts** | `rename_and_index.py` (Python), `rename_icons.ps1` (PowerShell) pour renommer d'autres dossiers. |

**Usage recommandé :** **Icônes ressources, statuts, menu, objets, potions, armes** pour MiyuClicker ; recherche par id ou par nom sémantique via l'index.

---

### 1.6 Tiny RPG Character Asset Pack v1.03b (Full 20 Characters)

| Attribut | Détail |
|----------|--------|
| **Chemin** | `ui/game_ui_pack/Tiny RPG Character Asset Pack v1.03b -Full 20 Characters/Tiny RPG Character Asset Pack v1.03 -Full 20 Characters/` |
| **Licence** | **Non documentée** dans le répertoire analysé — **à vérifier** (origine itch.io ou autre). |
| **Contenu** | 20 personnages (Archer, Knight, Orc, Skeleton, Priest, Wizard, etc.), projectiles (flèches, magie), fichiers Aseprite. |

**Contenu détaillé :**

| Catégorie | Détail | Pertinence MiyuClicker |
|-----------|--------|-------------------------|
| **Characters(100x100)** | ~343 PNG (sprites par personnage) | **Unités** (soldats, types de troupes), **héros** (beta). |
| **Arrow(Projectile)** | Flèches 100x100 et 32x32 | Projectiles combat (RTS / temps réel beta). |
| **Magic(Projectile)** | Effets Priest, Wizard | Effets de combat / héros. |
| **Aseprite file** | 20 .aseprite | Édition, animations, variantes. |

**Usage recommandé :** **Sprites d’unités** (soldats, types) et **héros** ; projectiles pour résolution combat avancée. **Ne pas utiliser en production sans confirmation de licence.**

---

### 1.7 ui-icn_fantasy-weapons_01 (Misbug)

| Attribut | Détail |
|----------|--------|
| **Chemin** | `ui/game_ui_pack/ui-icn_fantasy-weapons_01/` |
| **Licence** | Contrat dans `Misbug's Assets Licence Agreement.pdf` — **à consulter** pour usage commercial et attribution. |
| **Contenu** | Icônes d’armes fantasy (basic_Icons, drop_shadow, white_outline) : ~64+ armes (épée, hache, arc, bouclier, etc.). |

**Usage recommandé :** **Icônes d’armes** pour équipement, troupes, type d’unité ; cohérence fantasy. Vérifier la licence (PDF) avant usage commercial.

---

## 2. Synthèse par besoin MiyuClicker

| Besoin | Packs à privilégier | Remarque |
|--------|----------------------|-----------|
| **UI (barres, boutons, cadres, sliders)** | **Cute_Fantasy_UI** (principal) ; modernuserinterface-win (alternative moderne) | Cute_Fantasy_UI aligné avec Cute_Fantasy. |
| **Icônes ressources (nourriture, matières, outils)** | **Cute_Fantasy** → Icons ; **UltimatePixelArtFantasyRPGIconPack** (index JSON, nom sémantique) | Spritesheets à découper ou chargement par id/nom via index. |
| **Carte (tuiles terrain, routes, eau)** | **Cute_Fantasy** → Tiles (Grass, Cobble_Road, Water, Bridge, etc.) | Carte stratégique, checkpoints (beta). |
| **Bâtiments / cités** | **Cute_Fantasy** → Buildings | Représentation cités-États, gestion. |
| **« Gens » / métiers** | **Cute_Fantasy** → NPCs (Premade) ; **Tiny RPG** → Characters (si licence OK) | Affectation visuelle par rôle. |
| **Unités / soldats** | **Tiny RPG** → Characters (si licence OK) ; Cute_Fantasy → Enemies, Player | Troupes, types d’unités. |
| **Armes / équipement (icônes)** | **ui-icn_fantasy-weapons_01** (après vérif. licence) | Type d’unité, équipement. |
| **Portraits héros (beta v1.0)** | **modernuserinterface-win** → Portrait_Generator | Généraux, héros. |
| **Animations UI (feedback)** | Cute_Fantasy_UI → Pointer_Click_Anim ; modernuserinterface-win → GIF (référence spritesheet) | Toolkit animation par frame. |
| **Polices** | Cute_Fantasy_UI → Fonts (ttf) | egui : chargement de polices. |

---

## 3. Règles d’usage et conformité

| Règle | Détail |
|-------|--------|
| **Pas de redistribution des assets** | Cute_Fantasy, Cute_Fantasy_UI, modernuserinterface-win : interdiction de revendre/redistribuer les fichiers bruts. Les utiliser **dans** le jeu compilé est autorisé selon leurs licences. |
| **Crédits** | modernuserinterface-win : **crédits requis**. Prévoir écran « Crédits » ou fichier LICENSES avec noms des packs. |
| **Vérification licence** | CatUIFree, Tiny RPG, ui-icn_fantasy-weapons_01 : confirmer licence (et attribution si nécessaire) avant usage en production. |
| **Cohérence visuelle** | Pour une démo homogène : **Cute_Fantasy + Cute_Fantasy_UI** (même univers) en cœur ; autres packs en complément selon besoin. |

---

## 4. Références

- [MiyuClicker - Document Fondateur](MiyuClicker%20-%20Document%20Fondateur.md)
- [MiyuClicker - Analyse Marche et Besoins Toolkits](MiyuClicker%20-%20Analyse%20Marche%20et%20Besoins%20Toolkits.md)
- Répertoire physique : `ui/game_ui_pack/`

---

**Document créé le :** 2026-02-01  
**Statut :** Référence des packs UI jeux pour MiyuClicker
