# MiyuClicker — Analyse marché et besoins Toolkits

## Contexte

Ce document détaille l’**analyse PR du marché** des jeux Idle / Clicker avec aspects RPG et gestion, et l’**explicitation des besoins métier et Toolkits** pour MiyuClicker. Il complète le [Document Fondateur](MiyuClicker%20-%20Document%20Fondateur.md).

## Portée / Scope

- **Périmètre :** Marché Idle/RPG/gestion ; besoins fonctionnels et techniques ; mapping besoin → Toolkit (interne ou externe).
- **Hors périmètre :** Spécifications d’implémentation des crates, choix définitifs d’assets.

---

## 1. Analyse PR du marché — Idle, RPG, Gestion

### 1.1 Chiffres et tendances (2024–2025)

- **Idle RPG** : forte croissance en 2024 (Q2), notamment en APAC ; stratégie et puzzle progressent en parallèle. Ex. *Legend of Mushroom* dominant en Corée du Sud et au Japon.
- **Attentes joueurs** : mélange de boucles idle simples (clic, production passive) et de profondeur (RPG, gestion, stratégie). Références durables : Cookie Clicker, AdVenture Capitalist, Clicker Heroes ; évolution vers Melvor Idle, NGU Idle, Idle Champions.
- **Hybrides gestion / city-builder** : *Incremental Town RPG*, *City Idle* — ressources multiples, travailleurs/population, bâtiments, stockage, recherche, arbres de compétences.
- **Simulation** : jeux du type *Songs of Syx* — citoyens simulés, chaînes de production (primaire → secondaire → tertiaire), bonheur, capacité, commerce.

### 1.2 Positionnement MiyuClicker sur le marché

| Axe | Positionnement |
|-----|----------------|
| **Idle / Clicker** | Boucle de ressources et d’allocation de « gens » ; progression même hors ligne (tick simulé). |
| **RPG / Gestion** | Stats des troupes, moral, fécondité, recherche ; objectif long terme (conquête). |
| **Grande stratégie** | Carte de cités-États, conquête type Risk, temps de déplacement, bonus par cité. |
| **Différenciation** | Premier jeu officiel Miyukini ; démo de coexistence multi-services dans un environnement COG. |

### 1.3 Concurrents et inspirations (résumé)

- **Purs idle** : Cookie Clicker, AdVenture Capitalist.
- **Idle + RPG** : Clicker Heroes, Melvor Idle, Legend of Mushroom.
- **Idle + city / gestion** : Songs of Syx, Emperor, Incremental Town RPG, City Idle.
- **Stratégie carte** : Risk, Hearts of Iron 4.

---

## 2. Besoins métier et Toolkits — détail

### 2.1 Gestion de l’UI

| Besoin | Détail | Solution |
|--------|--------|----------|
| Fenêtre principale | Une fenêtre (desktop / web) avec zones : gestion (ressources, gens, soldats) et carte. | Dioxus (layout CSS flexbox/grid). |
| Menus et navigation | Menu principal, options, sauvegarde/chargement. | Dioxus layout CSS flexbox/grid. |
| Indicateurs en temps réel | Ressources (nourriture, matières, etc.), nombre de gens, moral, troupes. | composants RSX avec CSS, mis à jour via signaux réactifs Dioxus. |
| Formulaires | Allocation de gens (sliders ou champs), envoi de troupes (nombre + cité cible). | composants RSX avec CSS (input range, boutons, sélecteurs). |
| Thème / pack UI | Couleurs, polices, coins arrondis, boutons cohérents. | Thème CSS Dioxus ; **packs dans `ui/game_ui_pack`** : Cute_Fantasy_UI (principal), Cute_Fantasy, modernuserinterface-win — voir [Reference Packs UI Jeux](MiyuClicker%20-%20Reference%20Packs%20UI%20Jeux.md). |

**Toolkit :** Stack UI Dioxus (interne Miyukini).

### 2.2 Animations par frame

| Besoin | Détail | Solution |
|--------|--------|----------|
| Boucle de rendu | Mise à jour visuelle à chaque frame (60 FPS ou moins). | Dioxus réactivité (signaux) ; re-rendu automatique sur changement d'état. |
| Delta temps | Avancement des animations et de la simulation en fonction du temps. | `use_future` / tokio interval ou stockage `previous_time` ; `delta = now - previous_time`. |
| Animation sprite | Pour chaque entité animée : (spritesheet, animation_id, temps écoulé) → frame index. | Toolkit interne : structure Animation { frames: Vec<Rect>, fps } ; avancement `t += delta`, frame = frames[(t * fps) % len]. |
| Pause / vitesse | Pause, x1, x2 pour la simulation. | Horloge simulée : `sim_time += delta * speed_factor`. |

**Toolkit :** Animation par frame (interne) — enregistrement des animations, mise à jour du temps, sélection de la frame.

### 2.3 Spritesheets et gestion des sprites

| Besoin | Détail | Solution |
|--------|--------|----------|
| Chargement d’images | PNG/JPEG pour spritesheets. | Crate `image` ou chargement via Dioxus (inclure bytes, puis élément img avec data URL). |
| Découpage spritesheet | Grille (rows × cols) ou atlas avec coordonnées. | Toolkit interne : Spritesheet { texture, tile_width, tile_height, rows, cols } ; `frame_rect(index) -> Rect`. |
| Cache textures | Éviter de recharger les mêmes images. | Registre global ou dans l’état App : `HashMap<SpritesheetId, ImageData>` (élément img avec data URL). |
| Affichage d’un sprite | À une position (UI ou carte), éventuellement avec scale/rotation. | élément `img` RSX avec style CSS pour positionner la frame (background-position). |
| Sprites multiples | Unités, bâtiments, icônes de ressources. | Registre de sprites : (sheet_id, frame_index) ou (atlas_id, name). **Assets :** `ui/game_ui_pack` — Cute_Fantasy, Tiny RPG, ui-icn_fantasy-weapons_01 — voir [Reference Packs UI Jeux](MiyuClicker%20-%20Reference%20Packs%20UI%20Jeux.md). |

**Toolkit :** Sprites / Spritesheets (interne) — chargement, cache, API `sprite_rect(sheet, frame)` et affichage via Dioxus RSX ; alimentation depuis `ui/game_ui_pack`.

### 2.4 Carte stratégique

| Besoin | Détail | Solution |
|--------|--------|----------|
| Modèle de données | Nœuds (cités), arêtes (routes), positions 2D, propriété (joueur / adverses). | Structs : Node { id, position, troops, owner, ... }, Edge { from, to, travel_time? }. |
| Rendu carte | Dessin des nœuds (cercles/polygones), des arêtes (lignes), des labels. | éléments SVG/canvas Dioxus (circle, line, text) ; ou textures pour fond. |
| Interaction | Clic sur une cité, sélection, affichage d’infos (tooltip). | Hit-test : coordonnées clic → nœud le plus proche ; état `selected_city_id`. |
| Déplacements des troupes | Représentation visuelle (flèches, lignes animées). | Liste des mouvements en cours (from, to, progress 0..1) ; dessin de segments ou sprites le long de la route. |

**Toolkit :** Carte stratégique (interne) — modèle (graphe), rendu (SVG/canvas Dioxus), interaction (clic, survol).

### 2.5 Simulation (tick) et sauvegarde

| Besoin | Détail | Solution |
|--------|--------|----------|
| Tick simulation | Ressources, population, moral, troupes, déplacements, conquêtes. | Moteur de règles métier : `game_state.tick(delta)` ; pas de logique dans l’UI. |
| Sauvegarde | État complet de la partie (ressources, cités, troupes, carte). | serde + serde_json ; sauvegarde fichier JSON (serde + I/O). |
| Temps simulé | Vitesse du jeu (pause, x1, x2). | Variable `sim_time` ; `tick(delta * speed)` selon le mode. |

**Toolkits :** Simulation (interne — cœur métier) ; Sauvegarde (interne — sérialisation + I/O fichier).

### 2.6 Synthèse : tout ce que le jeu aura besoin pour fonctionner

| Domaine | Éléments | Priorité interne |
|---------|----------|------------------|
| **UI** | Fenêtre, panels, boutons, sliders, labels, tableaux, thème | Dioxus (interne) |
| **Rendu 2D** | Carte (formes, lignes, texte), sprites (images) | SVG/canvas Dioxus + img RSX (interne) |
| **Sprites** | Chargement, spritesheets, cache, frame → Rect | Toolkit Sprites (interne) |
| **Animation** | Delta temps, FPS, frame index par animation | Toolkit Animation (interne) |
| **Entrées** | Clic, survol, clavier | Dioxus événements RSX (interne) |
| **Simulation** | Tick ressources, gens, moral, troupes, déplacements, combat | Moteur métier (interne) |
| **Sauvegarde** | Sérialisation état, persistence | serde + I/O fichier (interne) |
| **Son** (optionnel v0.1) | SFX, musique | Crate permissive (rodio/kira) si besoin |

**Privilégier au maximum les solutions internes** : Toolkits Miyukini, stack Dioxus, moteur de simulation maison ; **assets depuis `ui/game_ui_pack`** (inventaire et mapping dans [MiyuClicker - Reference Packs UI Jeux](MiyuClicker%20-%20Reference%20Packs%20UI%20Jeux.md)). N’introduire des crates externes (macroquad, bevy, etc.) que si le rendu Dioxus s’avère insuffisant pour la cible visée.

---

## 3. Packs UI jeux (ui/game_ui_pack)

Les besoins UI et sprites sont couverts en priorité par les **packs présents dans `ui/game_ui_pack`** :

- **Cute_Fantasy_UI** : UI principale (barres, boutons, cadres, sliders, icônes, polices).
- **Cute_Fantasy** : Sprites monde (bâtiments, tuiles, NPCs « gens », icônes ressources, décors).
- **modernuserinterface-win** : UI alternative « moderne » ; Portrait Generator pour héros (beta v1.0).
- **Tiny RPG Character Asset Pack** : Unités / soldats, héros (licence à vérifier).
- **ui-icn_fantasy-weapons_01** : Icônes armes (licence PDF à consulter).
- **CatUIFree** : Contenu minimal ; licence à vérifier.

Inventaire détaillé, licences et règles d’usage : [MiyuClicker - Reference Packs UI Jeux](MiyuClicker%20-%20Reference%20Packs%20UI%20Jeux.md).

---

## 4. Références

- [MiyuClicker - Document Fondateur](MiyuClicker%20-%20Document%20Fondateur.md)
- [MiyuClicker - Reference Packs UI Jeux](MiyuClicker%20-%20Reference%20Packs%20UI%20Jeux.md)
- [Miyukini - Stack UI Dioxus](../../ux_ui/Miyukini%20-%20Stack%20UI%20Dioxus.md)

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-11  
**Statut :** Complément au Document Fondateur
