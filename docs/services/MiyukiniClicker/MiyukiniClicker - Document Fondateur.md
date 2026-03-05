# MiyuClicker â€” Document Fondateur

## Contexte

**MiyuClicker** est le **premier jeu officiel Miyukini**. Il sert de **dÃ©mo vivante** pour montrer quâ€™il est possible de faire coexister plusieurs services (OpÃ©rateurs, Toolkits) au sein dâ€™un mÃªme environnement COG, tout en offrant une expÃ©rience de jeu complÃ¨te : Idle / Clicker cÃ´tÃ© gestion, et grande stratÃ©gie (type Risk) cÃ´tÃ© conquÃªte territoriale.

Le jeu est dÃ©veloppÃ© en **Rust**, sâ€™appuie sur la **stack UI officielle Miyukini (Dioxus)** et sur un **pack UI open-source Ã  licence permissive**. Il consomme les Toolkits et OpÃ©rateurs nÃ©cessaires (gestion UI, animations, sprites, sauvegarde, etc.) en privilÃ©giant au maximum les **solutions internes** Ã  lâ€™Ã©cosystÃ¨me Miyukini.

Ce document est le **document fondateur** du jeu : il en fixe la raison dâ€™Ãªtre, lâ€™analyse marchÃ©, les besoins mÃ©tier et techniques (Toolkits), le gameplay, les versions prÃ©vues (0.1, beta v1.0) et les inspirations.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre :** DÃ©finition du jeu MiyuClicker â€” positionnement, analyse marchÃ© Idle/RPG/gestion, besoins mÃ©tier et Toolkits, gameplay (gestion + carte stratÃ©gique), roadmap versions, stack technique, intÃ©gration COG.
- **Hors pÃ©rimÃ¨tre :** SpÃ©cifications dÃ©taillÃ©es dâ€™implÃ©mentation (crates, API), assets graphiques, contenu narratif.

---

## 1. Analyse PR du marchÃ© â€” Idle / Clicker, RPG et Gestion

### 1.1 Tendances du genre (2024â€“2025)

| Aspect | Constat |
|--------|--------|
| **Idle / RPG** | Forte croissance en 2024 (Q2), notamment en APAC ; titres comme *Legend of Mushroom* dominent en CorÃ©e du Sud et au Japon. Le genre mÃªle boucles idle et mÃ©caniques RPG (progression, stats, Ã©quipement). |
| **Idle / Gestion** | Fusion avec city-builder et simulation : gestion de ressources, de population, de bÃ¢timents, avec boucles de production (primaire â†’ secondaire â†’ tertiaire). |
| **Hybrides** | Les joueurs attendent Ã  la fois du Â« clicker Â» simple (Cookie Clicker, AdVenture Capitalist, Clicker Heroes) et des couches plus profondes : arbres de compÃ©tences, recherche, commerce, diplomatie, conquÃªte. |
| **RÃ©fÃ©rences durables** | Cookie Clicker (2013), AdVenture Capitalist (2014), Clicker Heroes (2014) restent des piliers ; Melvor Idle, NGU Idle, Idle Champions of the Forgotten Realms illustrent lâ€™Ã©volution vers plus de profondeur. |

### 1.2 Positionnement MiyuClicker

- **Idle / Gestion** : boucle de ressources (nourriture, matiÃ¨res premiÃ¨res, outils, recherche, armes), allocation de Â« gens Â» (population), moral et fÃ©conditÃ©, soldats.
- **Grande stratÃ©gie** : carte de citÃ©s-Ã‰tats, conquÃªte type Risk, temps de dÃ©placement des troupes, bonus de tribu par citÃ© conquise.
- **DÃ©mo COG** : le jeu prouve que plusieurs OpÃ©rateurs et Toolkits peuvent coexister dans un mÃªme environnement (simulation, UI, carte, sauvegarde, etc.).

---

## 2. Besoins mÃ©tier et Toolkits

### 2.1 Vue dâ€™ensemble

Le jeu nÃ©cessite des **capacitÃ©s** couvrant : UI, rendu 2D (carte, sprites), animations par frame, gestion des spritesheets, entrÃ©es utilisateur, sauvegarde/chargement, simulation (tick), et Ã©ventuellement son. **PrioritÃ© : privilÃ©gier les solutions internes** (Toolkits Miyukini, Dioxus) avant dâ€™intÃ©grer des crates externes.

### 2.2 Besoins dÃ©taillÃ©s et Toolkits associÃ©s

| Besoin | Description | Solution privilÃ©giÃ©e (interne) | Solution externe si nÃ©cessaire |
|--------|-------------|---------------------------------|----------------------------------|
| **UI principale** | Menus, panels, boutons, indicateurs de ressources, listes (gens, soldats, citÃ©s). | **Dioxus** (stack UI officielle Miyukini). | â€” |
| **Pack UI / thÃ¨me** | Look cohÃ©rent, couleurs, typo, composants rÃ©utilisables. | ThÃ¨me CSS Dioxus dÃ©rivÃ© du style Miyukini (voir [Stack UI Dioxus](..//..//_index.md)) ; **packs UI jeux** prÃ©sents dans `ui/game_ui_pack` (voir [MiyuClicker - Reference Packs UI Jeux](MiyukiniClicker%20-%20Reference%20Packs%20UI%20Jeux.md)) : Cute_Fantasy_UI (principal), Cute_Fantasy (sprites, tuiles, icÃ´nes), modernuserinterface-win (alternative, portraits). | Packs dÃ©jÃ  prÃ©sents en interne ; vÃ©rifier licences par pack (pas de redistribution des assets bruts). |
| **Rendu 2D (carte)** | Carte stratÃ©gique : nÅ“uds (citÃ©s), arÃªtes (routes), dÃ©placements, sÃ©lection. | **Dioxus** : Ã©lÃ©ments SVG/canvas pour la carte avec primitives (cercles, lignes, polygones). Textures pour fond/tiles si besoin. | Si besoin moteur 2D dÃ©diÃ© : crates Rust Ã  licence permissive (ex. macroquad pour canvas jeu uniquement). |
| **Sprites et spritesheets** | Personnages, unitÃ©s, bÃ¢timents, icÃ´nes ; animations par frame. | **Dioxus** : Ã©lÃ©ments `img` RSX / textures Ã  partir dâ€™images ; dÃ©coupage spritesheet en sous-rectangles ; frame courante = index dans la sheet. **Assets** : `ui/game_ui_pack` â€” Cute_Fantasy (bÃ¢timents, tuiles, NPCs, icÃ´nes ressources), Tiny RPG (unitÃ©s, hÃ©ros), ui-icn_fantasy-weapons_01 (icÃ´nes armes) â€” voir [Reference Packs UI Jeux](MiyukiniClicker%20-%20Reference%20Packs%20UI%20Jeux.md). Toolkit interne : chargement + cache de textures, dÃ©finition dâ€™animations (plage de frames, FPS). | Crate type `image`, `png` ; Ã©ventuellement macroquad/bevy si on dÃ©cide dâ€™un rendu jeu sÃ©parÃ© de lâ€™UI Dioxus. |
| **Animations par frame** | Mise Ã  jour du numÃ©ro de frame en fonction du temps (delta). | **Toolkit interne** : boucle de jeu avec signaux rÃ©actifs Dioxus ; Ã©tat `(sprite_id, animation_id, t_accumulator)` ; avancement `t += delta`, sÃ©lection de la frame. | â€” |
| **Gestion des sprites** | Chargement, cache, libÃ©ration ; rÃ©solution des rectangles par (spritesheet, row, col) ou par ID. | **Toolkit interne** : registre de sprites (path ou bytes, dimensions, layout en grille) ; API du type `sprite_rect(sheet_id, frame_index) -> Rect`. | Crate `image` pour dÃ©codage. |
| **EntrÃ©es** | Clics, survol, clavier (raccourcis, navigation). | **Dioxus** : Ã©vÃ©nements RSX (`onclick`, `onmouseover`, `onkeydown`), zones interactives sur la carte. | â€” |
| **Boucle de gameplay (tick)** | Simulation discrÃ¨te ou continue (ressources, moral, population, dÃ©placements des troupes). | **Logique mÃ©tier interne** : Ã©tat du monde (ECS ou structs selon complexitÃ©) ; `tick(delta)` appelÃ© via `use_future` / tokio interval ; pas de logique dans lâ€™UI. | â€” |
| **Sauvegarde / chargement** | Persistance de la partie (ressources, citÃ©s, troupes, carte). | **Sauvegarde fichier JSON** (serde + I/O direct) ; sÃ©rialisation (serde) de lâ€™Ã©tat du jeu. Optionnel : KindMother si on veut centraliser les sauvegardes cÃ´tÃ© COG (hors scope v0.1). | `serde`, `serde_json` (dÃ©jÃ  courants en Rust). |
| **Temps rÃ©el / temps simulÃ©** | Vitesse du jeu (pause, x1, x2), temps de dÃ©placement des troupes. | **Horloge interne** : temps simulÃ© sÃ©parÃ© du temps rÃ©el ; `Clock` Kernel si alignement avec la trace (optionnel). | â€” |
| **Son (optionnel v0.1)** | Sons dâ€™interface, ambiances. | Toolkit interne ou crate audio permissive (MIT/Apache-2.0). | `rodio`, `kira` (licences permissives). |

### 2.3 SynthÃ¨se Toolkits Ã  prÃ©voir (interne)

| Toolkit | RÃ´le |
|---------|------|
| **UI (Dioxus)** | DÃ©jÃ  stack officielle ; fenÃªtre principale, layout CSS flexbox/grid, composants RSX. |
| **Sprites / Spritesheets** | Chargement images, cache textures, dÃ©coupage en frames, registre (sheet_id, frame) â†’ Rect. |
| **Animation par frame** | Avancement temporel des animations (delta), sÃ©lection de la frame, boucle ou one-shot. |
| **Carte stratÃ©gique** | ModÃ¨le (nÅ“uds, arÃªtes, positions) ; rendu SVG/canvas Dioxus ; interaction (clic, survol). |
| **Sauvegarde** | SÃ©rialisation Ã©tat jeu ; sauvegarde fichier JSON (serde + I/O). |
| **Simulation (tick)** | Moteur de rÃ¨gles mÃ©tier : ressources, gens, moral, soldats, conquÃªtes, dÃ©placements. |

---

## 3. Gameplay

### 3.1 VolÃ©e 1 â€” Gestion (fenÃªtre principale)

- **Boucle type Idle** : le joueur gÃ¨re des **ressources** en y allouant des **gens**.
- **Ressources** : nourriture, bois, pierre, fer, outils, recherche, armes. Les **matiÃ¨res premiÃ¨res** (bois, pierre, fer) et les **produits manufacturÃ©s** (outils, armes) sont des **catÃ©gories** pour les caps de stockage, pas des ressources affichÃ©es en tant que telles â€” voir [MiyuClicker - Ressources et Categories](MiyukiniClicker%20-%20Ressources%20et%20Categories.md).
- **Gens** : consomment de la nourriture et ont des **besoins**. Ils peuvent produire : nourriture, matiÃ¨res premiÃ¨res, outils, recherche, armes. Une partie des gens peut devenir **soldats** (comptÃ©s dans le total de gens).
- **Dynamique** : la quantitÃ© de gens disponible est **dynamique** (moral, fÃ©conditÃ©). Si la nourriture manque : moral baisse, fÃ©conditÃ© baisse, la population peut diminuer. Le joueur doit **rÃ©partir** les gens pour maintenir croissance ou Ã©quilibre.
- **Objectif gestion** : avoir assez de troupes pour **dÃ©fendre** le territoire et, Ã  terme, **conquÃ©rir** la carte.

### 3.2 VolÃ©e 2 â€” Carte stratÃ©gique (grande stratÃ©gie)

- **Carte** : maillage de **citÃ©s-Ã‰tats** reliÃ©es par des **routes**.
- **Actions** : le joueur clique sur une citÃ© et peut dÃ©cider dâ€™**envoyer X soldats** pour la conquÃ©rir. Un peu de **hasard** + les **stats des troupes** de chaque camp dÃ©terminent le vainqueur et les troupes restantes (style Risk).
- **DÃ©placement** : chaque troupe envoyÃ©e met un **temps variable** pour atteindre la citÃ© cible (dÃ©pendant de la route / distance).

### 3.3 Version 0.1 â€” Comportement des citÃ©s adverses

- Les citÃ©s adverses **nâ€™ont pas de comportement** (pas dâ€™IA offensive/diplomatique).
- Elles possÃ¨dent un **nombre de troupes** qui **Ã©volue jusquâ€™Ã  un plafond**, selon une courbe calquÃ©e sur lâ€™Ã©volution du joueur pour garder un minimum de **challenge**.
- CitÃ©s **proches** : plus faciles ; citÃ©s **lointaines** ou ayant eu le temps dâ€™Ã©voluer : plus difficiles.
- **CitÃ© conquise** : donne un **bonus en ressources** au joueur (type Â« tribu Â»).

### 3.4 Conception Beta v1.0 (Ã©volution prÃ©vue)

- **Diplomatie** : relations entre citÃ©s, alliances, traitÃ©s.
- **Ã‰conomie de marchÃ©** : ressources et outils plus variÃ©s, **caravanes** qui circulent, citÃ©s **spÃ©cialisÃ©es** (ressources).
- **Routes multiples** : plusieurs chemins vers une mÃªme destination, avec **checkpoints** (village, fort) ; routes plus longues mais plus sÃ»res, raccourcis (ex. forÃªt) avec ralentissement.
- **RÃ©solution des combats** : en **RTS** ou **temps rÃ©el** (au lieu dâ€™un simple jet dÃ©terministe/alÃ©atoire).
- **HÃ©ros** : impact sur les batailles et le gameplay. Ex. : **GÃ©nÃ©ral** (direction des combats en RTS/RPG action), **Artisan** (mini-jeux pour artefacts boost), **Ã‰rudit** (mini-jeux pour recherche).
- **Arbre de technologie** et **3 arbres de compÃ©tences**.

---

## 4. Stack technique et choix

### 4.1 Langage et UI

| Ã‰lÃ©ment | Choix | RÃ©fÃ©rence |
|--------|--------|------------|
| **Langage** | Rust | Ã‰cosystÃ¨me Miyukini, performance, sÃ©curitÃ©. |
| **UI** | Dioxus 0.6 (desktop natif via Blitz/WGPU) | Stack UI officielle Miyukini ; licence MIT/Apache-2.0 ; desktop natif. |
| **Pack UI / assets** | Pack open-source Ã  licence permissive | Pour cohÃ©rence visuelle (thÃ¨me, icÃ´nes, sprites gÃ©nÃ©riques) ; pas de dÃ©pendance propriÃ©taire. |

### 4.2 Rendu jeu (carte, sprites)

- **Option A (recommandÃ©e v0.1)** : tout dans **Dioxus** â€” carte via Ã©lÃ©ments SVG/canvas, sprites via Ã©lÃ©ments `img` RSX et textures (spritesheets dÃ©coupÃ©es). Un seul point d'entrÃ©e (`dioxus::launch`), une seule boucle rÃ©active.
- **Option B** : si besoin de rendu 2D plus riche (effets, nombreux sprites animÃ©s), intÃ©gration dâ€™un moteur 2D Rust (ex. macroquad) dans une fenÃªtre ou un viewport ; **Dioxus** reste pour les menus et HUD. Ã€ trancher en phase dâ€™implÃ©mentation.

### 4.3 Licences

- **Dioxus** : MIT ou Apache-2.0.
- **Pack UI / sprites** : choix dâ€™un pack ou dâ€™assets **MIT, Apache-2.0, CC0** (ou Ã©quivalent permissif) pour Ã©viter toute contrainte commerciale ou dâ€™attribution forte.

---

## 5. IntÃ©gration COG â€” OpÃ©rateurs et Services

### 5.1 RÃ´le du jeu dans lâ€™Ã©cosystÃ¨me

- MiyuClicker est un **OpÃ©rateur dâ€™Interface** (ou un agrÃ©gat dâ€™OpÃ©rateurs) qui **consomme** des Toolkits (UI, sprites, animation, sauvegarde, simulation) et prouve que **plusieurs services** peuvent vivre dans le mÃªme environnement Miyukini.
- La **logique mÃ©tier** (simulation, rÃ¨gles de combat, Ã©conomie) peut Ãªtre exposÃ©e sous forme de **Tools** ou de **OpÃ©rateurs de Service** rÃ©utilisables (ex. Â« Simulation Idle Â», Â« RÃ©solution combat Â») pour dâ€™autres jeux ou dÃ©mos.

### 5.2 OpÃ©rateurs identifiÃ©s (vision)

| OpÃ©rateur | RÃ´le | Type |
|-----------|------|------|
| **MiyuClickerUI** | Interface principale (gestion + carte), menus, HUD. | OpÃ©rateur dâ€™Interface |
| **MiyuClickerSim** | Simulation tick (ressources, gens, moral, troupes, dÃ©placements). | OpÃ©rateur de Service |
| **MiyuClickerCombat** | RÃ©solution des combats (stats, hasard, troupes restantes). | OpÃ©rateur de Service / Tool |
| **MiyuClickerSave** | Sauvegarde / chargement de partie (Ã©tat monde). | OpÃ©rateur de Service ou sauvegarde fichier JSON + KindMother optionnel |
| **MiyuClickerCarte** | ModÃ¨le carte (citÃ©s, routes), dÃ©placements, combats. | OpÃ©rateur de Service |

Les **Toolkits** (Sprites, Animation, Carte, IdleSim, Save, Combat) sont des **Kits dâ€™Outils** ou **Outils** (Strate 6) utilisÃ©s par ces OpÃ©rateurs, gouvernÃ©s par Master Butler et les Cores. **DÃ©tail MVP et mapping :** [MiyuClicker - MVP Ecrans et Mecaniques](MiyukiniClicker%20-%20MVP%20Ecrans%20et%20Mecaniques.md), [MiyuClicker - Operateurs et Toolkits](MiyukiniClicker%20-%20Operateurs%20et%20Toolkits.md).

---

## 6. Inspirations fortes

| Jeu / univers | Apport pour MiyuClicker |
|---------------|--------------------------|
| **Songs of Syx** | City-builder fantasy, simulation de population, chaÃ®nes de production (primaire â†’ secondaire â†’ tertiaire), gestion de bonheur/capacitÃ©, grande Ã©chelle. |
| **Emperor : Lâ€™empire du milieu** | City-builder historique, gestion des ressources et des habitants, ambiance et progression. |
| **Hearts of Iron 4** | Grande stratÃ©gie, carte, troupes, dÃ©placements, conquÃªte. |
| **Risk** | ConquÃªte par territoires, rÃ©solution de batailles par jets + troupes, objectif de contrÃ´le de la carte. |

---

## 7. DÃ©cisions structurantes

| Id | DÃ©cision | Justification |
|----|----------|---------------|
| **DS-01** | Premier jeu officiel Miyukini = dÃ©mo multi-services COG | Prouver la coexistence dâ€™OpÃ©rateurs et Toolkits dans un mÃªme environnement. |
| **DS-02** | Rust + Dioxus + pack UI permissif | Alignement stack Miyukini, licence sans contrainte commerciale. |
| **DS-03** | PrivilÃ©gier solutions internes (Toolkits UI, sprites, animation, sauvegarde) | RÃ©utilisabilitÃ©, gouvernance, cohÃ©rence avec la pyramide Miyukini. |
| **DS-04** | Deux volÃ©es : Gestion (Idle) + Carte (grande stratÃ©gie) | Boucle idle claire + objectif long terme (conquÃªte). |
| **DS-05** | Version 0.1 : citÃ©s sans IA, courbe de troupes adverses, bonus tribu | Scope maÃ®trisable ; beta v1.0 pour diplomatie, marchÃ©, routes, hÃ©ros, RTS. |
| **DS-06** | Inspirations : Songs of Syx, Emperor, HOI4, Risk | Ancrage dans des rÃ©fÃ©rences reconnues Idle / gestion / stratÃ©gie. |

---

## 8. Packs UI jeux (ui/game_ui_pack)

Le rÃ©pertoire **`ui/game_ui_pack`** contient des packs dâ€™assets UI et graphiques pour jeux, analysÃ©s et inventoriÃ©s dans un document dÃ©diÃ© :

| Pack | RÃ´le pour MiyuClicker | Licence (rÃ©sumÃ©) |
|------|------------------------|-------------------|
| **Cute_Fantasy** | BÃ¢timents, tuiles carte, NPCs (Â« gens Â»), icÃ´nes ressources (nourriture, matiÃ¨res, outils), dÃ©cors. | Commercial / non commercial, modifiable ; pas de redistribution. |
| **Cute_Fantasy_UI** | Barres, boutons, cadres, sliders, icÃ´nes UI, polices â€” **pack UI principal**. | Idem Cute_Fantasy. |
| **modernuserinterface-win** | UI alternative Â« moderne Â» ; Portrait Generator (hÃ©ros, beta v1.0) ; ex. animations (GIF). | Commercial / non commercial (sauf NFT) ; crÃ©dits requis ; pas de redistribution. |
| **Tiny RPG Character Asset Pack** | UnitÃ©s (soldats, types), hÃ©ros, projectiles. | Ã€ vÃ©rifier (non documentÃ©e dans le pack). |
| **ui-icn_fantasy-weapons_01** | IcÃ´nes dâ€™armes fantasy (Ã©quipement, type dâ€™unitÃ©). | Contrat PDF (Misbug) â€” Ã  consulter. |
| **CatUIFree** | Contenu minimal (2 PNG). | Ã€ vÃ©rifier. |

**RÃ©fÃ©rence complÃ¨te :** [MiyuClicker - Reference Packs UI Jeux](MiyukiniClicker%20-%20Reference%20Packs%20UI%20Jeux.md) â€” inventaire dÃ©taillÃ©, licences, mapping besoin â†’ pack, rÃ¨gles dâ€™usage.

---

## 9. RÃ©fÃ©rences

| Document | Lien |
|----------|------|
| **Stack UI Dioxus** | [Miyukini - Stack UI Dioxus](..//..//_index.md) |
| **Packs UI jeux** | [MiyuClicker - Reference Packs UI Jeux](MiyukiniClicker%20-%20Reference%20Packs%20UI%20Jeux.md) |
| **Glossaire Miyukini** | Miyukini Conceptual References - Glossaire (OpÃ©rateur, Toolkit, COG) |
| **Document Fondateur type** | [Miyukini Sales - Document Fondateur](../../services/MiyukiniSales/Miyukini%20Sales%20-%20Document%20Fondateur.md) |

---

**Document crÃ©Ã© le :** 2026-02-01  
**DerniÃ¨re mise Ã  jour :** 2026-02-11  
**Statut :** Document fondateur â€” premier jeu officiel Miyukini


