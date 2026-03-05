# MiyuClicker â€” Analyse marchÃ© et besoins Toolkits

## Contexte

Ce document dÃ©taille lâ€™**analyse PR du marchÃ©** des jeux Idle / Clicker avec aspects RPG et gestion, et lâ€™**explicitation des besoins mÃ©tier et Toolkits** pour MiyuClicker. Il complÃ¨te le [Document Fondateur](MiyukiniClicker%20-%20Document%20Fondateur.md).

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre :** MarchÃ© Idle/RPG/gestion ; besoins fonctionnels et techniques ; mapping besoin â†’ Toolkit (interne ou externe).
- **Hors pÃ©rimÃ¨tre :** SpÃ©cifications dâ€™implÃ©mentation des crates, choix dÃ©finitifs dâ€™assets.

---

## 1. Analyse PR du marchÃ© â€” Idle, RPG, Gestion

### 1.1 Chiffres et tendances (2024â€“2025)

- **Idle RPG** : forte croissance en 2024 (Q2), notamment en APAC ; stratÃ©gie et puzzle progressent en parallÃ¨le. Ex. *Legend of Mushroom* dominant en CorÃ©e du Sud et au Japon.
- **Attentes joueurs** : mÃ©lange de boucles idle simples (clic, production passive) et de profondeur (RPG, gestion, stratÃ©gie). RÃ©fÃ©rences durables : Cookie Clicker, AdVenture Capitalist, Clicker Heroes ; Ã©volution vers Melvor Idle, NGU Idle, Idle Champions.
- **Hybrides gestion / city-builder** : *Incremental Town RPG*, *City Idle* â€” ressources multiples, travailleurs/population, bÃ¢timents, stockage, recherche, arbres de compÃ©tences.
- **Simulation** : jeux du type *Songs of Syx* â€” citoyens simulÃ©s, chaÃ®nes de production (primaire â†’ secondaire â†’ tertiaire), bonheur, capacitÃ©, commerce.

### 1.2 Positionnement MiyuClicker sur le marchÃ©

| Axe | Positionnement |
|-----|----------------|
| **Idle / Clicker** | Boucle de ressources et dâ€™allocation de Â« gens Â» ; progression mÃªme hors ligne (tick simulÃ©). |
| **RPG / Gestion** | Stats des troupes, moral, fÃ©conditÃ©, recherche ; objectif long terme (conquÃªte). |
| **Grande stratÃ©gie** | Carte de citÃ©s-Ã‰tats, conquÃªte type Risk, temps de dÃ©placement, bonus par citÃ©. |
| **DiffÃ©renciation** | Premier jeu officiel Miyukini ; dÃ©mo de coexistence multi-services dans un environnement COG. |

### 1.3 Concurrents et inspirations (rÃ©sumÃ©)

- **Purs idle** : Cookie Clicker, AdVenture Capitalist.
- **Idle + RPG** : Clicker Heroes, Melvor Idle, Legend of Mushroom.
- **Idle + city / gestion** : Songs of Syx, Emperor, Incremental Town RPG, City Idle.
- **StratÃ©gie carte** : Risk, Hearts of Iron 4.

---

## 2. Besoins mÃ©tier et Toolkits â€” dÃ©tail

### 2.1 Gestion de lâ€™UI

| Besoin | DÃ©tail | Solution |
|--------|--------|----------|
| FenÃªtre principale | Une fenÃªtre (desktop / web) avec zones : gestion (ressources, gens, soldats) et carte. | Dioxus (layout CSS flexbox/grid). |
| Menus et navigation | Menu principal, options, sauvegarde/chargement. | Dioxus layout CSS flexbox/grid. |
| Indicateurs en temps rÃ©el | Ressources (nourriture, matiÃ¨res, etc.), nombre de gens, moral, troupes. | composants RSX avec CSS, mis Ã  jour via signaux rÃ©actifs Dioxus. |
| Formulaires | Allocation de gens (sliders ou champs), envoi de troupes (nombre + citÃ© cible). | composants RSX avec CSS (input range, boutons, sÃ©lecteurs). |
| ThÃ¨me / pack UI | Couleurs, polices, coins arrondis, boutons cohÃ©rents. | ThÃ¨me CSS Dioxus ; **packs dans `ui/game_ui_pack`** : Cute_Fantasy_UI (principal), Cute_Fantasy, modernuserinterface-win â€” voir [Reference Packs UI Jeux](MiyukiniClicker%20-%20Reference%20Packs%20UI%20Jeux.md). |

**Toolkit :** Stack UI Dioxus (interne Miyukini).

### 2.2 Animations par frame

| Besoin | DÃ©tail | Solution |
|--------|--------|----------|
| Boucle de rendu | Mise Ã  jour visuelle Ã  chaque frame (60 FPS ou moins). | Dioxus rÃ©activitÃ© (signaux) ; re-rendu automatique sur changement d'Ã©tat. |
| Delta temps | Avancement des animations et de la simulation en fonction du temps. | `use_future` / tokio interval ou stockage `previous_time` ; `delta = now - previous_time`. |
| Animation sprite | Pour chaque entitÃ© animÃ©e : (spritesheet, animation_id, temps Ã©coulÃ©) â†’ frame index. | Toolkit interne : structure Animation { frames: Vec<Rect>, fps } ; avancement `t += delta`, frame = frames[(t * fps) % len]. |
| Pause / vitesse | Pause, x1, x2 pour la simulation. | Horloge simulÃ©e : `sim_time += delta * speed_factor`. |

**Toolkit :** Animation par frame (interne) â€” enregistrement des animations, mise Ã  jour du temps, sÃ©lection de la frame.

### 2.3 Spritesheets et gestion des sprites

| Besoin | DÃ©tail | Solution |
|--------|--------|----------|
| Chargement dâ€™images | PNG/JPEG pour spritesheets. | Crate `image` ou chargement via Dioxus (inclure bytes, puis Ã©lÃ©ment img avec data URL). |
| DÃ©coupage spritesheet | Grille (rows Ã— cols) ou atlas avec coordonnÃ©es. | Toolkit interne : Spritesheet { texture, tile_width, tile_height, rows, cols } ; `frame_rect(index) -> Rect`. |
| Cache textures | Ã‰viter de recharger les mÃªmes images. | Registre global ou dans lâ€™Ã©tat App : `HashMap<SpritesheetId, ImageData>` (Ã©lÃ©ment img avec data URL). |
| Affichage dâ€™un sprite | Ã€ une position (UI ou carte), Ã©ventuellement avec scale/rotation. | Ã©lÃ©ment `img` RSX avec style CSS pour positionner la frame (background-position). |
| Sprites multiples | UnitÃ©s, bÃ¢timents, icÃ´nes de ressources. | Registre de sprites : (sheet_id, frame_index) ou (atlas_id, name). **Assets :** `ui/game_ui_pack` â€” Cute_Fantasy, Tiny RPG, ui-icn_fantasy-weapons_01 â€” voir [Reference Packs UI Jeux](MiyukiniClicker%20-%20Reference%20Packs%20UI%20Jeux.md). |

**Toolkit :** Sprites / Spritesheets (interne) â€” chargement, cache, API `sprite_rect(sheet, frame)` et affichage via Dioxus RSX ; alimentation depuis `ui/game_ui_pack`.

### 2.4 Carte stratÃ©gique

| Besoin | DÃ©tail | Solution |
|--------|--------|----------|
| ModÃ¨le de donnÃ©es | NÅ“uds (citÃ©s), arÃªtes (routes), positions 2D, propriÃ©tÃ© (joueur / adverses). | Structs : Node { id, position, troops, owner, ... }, Edge { from, to, travel_time? }. |
| Rendu carte | Dessin des nÅ“uds (cercles/polygones), des arÃªtes (lignes), des labels. | Ã©lÃ©ments SVG/canvas Dioxus (circle, line, text) ; ou textures pour fond. |
| Interaction | Clic sur une citÃ©, sÃ©lection, affichage dâ€™infos (tooltip). | Hit-test : coordonnÃ©es clic â†’ nÅ“ud le plus proche ; Ã©tat `selected_city_id`. |
| DÃ©placements des troupes | ReprÃ©sentation visuelle (flÃ¨ches, lignes animÃ©es). | Liste des mouvements en cours (from, to, progress 0..1) ; dessin de segments ou sprites le long de la route. |

**Toolkit :** Carte stratÃ©gique (interne) â€” modÃ¨le (graphe), rendu (SVG/canvas Dioxus), interaction (clic, survol).

### 2.5 Simulation (tick) et sauvegarde

| Besoin | DÃ©tail | Solution |
|--------|--------|----------|
| Tick simulation | Ressources, population, moral, troupes, dÃ©placements, conquÃªtes. | Moteur de rÃ¨gles mÃ©tier : `game_state.tick(delta)` ; pas de logique dans lâ€™UI. |
| Sauvegarde | Ã‰tat complet de la partie (ressources, citÃ©s, troupes, carte). | serde + serde_json ; sauvegarde fichier JSON (serde + I/O). |
| Temps simulÃ© | Vitesse du jeu (pause, x1, x2). | Variable `sim_time` ; `tick(delta * speed)` selon le mode. |

**Toolkits :** Simulation (interne â€” cÅ“ur mÃ©tier) ; Sauvegarde (interne â€” sÃ©rialisation + I/O fichier).

### 2.6 SynthÃ¨se : tout ce que le jeu aura besoin pour fonctionner

| Domaine | Ã‰lÃ©ments | PrioritÃ© interne |
|---------|----------|------------------|
| **UI** | FenÃªtre, panels, boutons, sliders, labels, tableaux, thÃ¨me | Dioxus (interne) |
| **Rendu 2D** | Carte (formes, lignes, texte), sprites (images) | SVG/canvas Dioxus + img RSX (interne) |
| **Sprites** | Chargement, spritesheets, cache, frame â†’ Rect | Toolkit Sprites (interne) |
| **Animation** | Delta temps, FPS, frame index par animation | Toolkit Animation (interne) |
| **EntrÃ©es** | Clic, survol, clavier | Dioxus Ã©vÃ©nements RSX (interne) |
| **Simulation** | Tick ressources, gens, moral, troupes, dÃ©placements, combat | Moteur mÃ©tier (interne) |
| **Sauvegarde** | SÃ©rialisation Ã©tat, persistence | serde + I/O fichier (interne) |
| **Son** (optionnel v0.1) | SFX, musique | Crate permissive (rodio/kira) si besoin |

**PrivilÃ©gier au maximum les solutions internes** : Toolkits Miyukini, stack Dioxus, moteur de simulation maison ; **assets depuis `ui/game_ui_pack`** (inventaire et mapping dans [MiyuClicker - Reference Packs UI Jeux](MiyukiniClicker%20-%20Reference%20Packs%20UI%20Jeux.md)). Nâ€™introduire des crates externes (macroquad, bevy, etc.) que si le rendu Dioxus sâ€™avÃ¨re insuffisant pour la cible visÃ©e.

---

## 3. Packs UI jeux (ui/game_ui_pack)

Les besoins UI et sprites sont couverts en prioritÃ© par les **packs prÃ©sents dans `ui/game_ui_pack`** :

- **Cute_Fantasy_UI** : UI principale (barres, boutons, cadres, sliders, icÃ´nes, polices).
- **Cute_Fantasy** : Sprites monde (bÃ¢timents, tuiles, NPCs Â« gens Â», icÃ´nes ressources, dÃ©cors).
- **modernuserinterface-win** : UI alternative Â« moderne Â» ; Portrait Generator pour hÃ©ros (beta v1.0).
- **Tiny RPG Character Asset Pack** : UnitÃ©s / soldats, hÃ©ros (licence Ã  vÃ©rifier).
- **ui-icn_fantasy-weapons_01** : IcÃ´nes armes (licence PDF Ã  consulter).
- **CatUIFree** : Contenu minimal ; licence Ã  vÃ©rifier.

Inventaire dÃ©taillÃ©, licences et rÃ¨gles dâ€™usage : [MiyuClicker - Reference Packs UI Jeux](MiyukiniClicker%20-%20Reference%20Packs%20UI%20Jeux.md).

---

## 4. RÃ©fÃ©rences

- [MiyuClicker - Document Fondateur](MiyukiniClicker%20-%20Document%20Fondateur.md)
- [MiyuClicker - Reference Packs UI Jeux](MiyukiniClicker%20-%20Reference%20Packs%20UI%20Jeux.md)
- [Miyukini - Stack UI Dioxus](..//..//_index.md)

---

**Document crÃ©Ã© le :** 2026-02-01  
**DerniÃ¨re mise Ã  jour :** 2026-02-11  
**Statut :** ComplÃ©ment au Document Fondateur


