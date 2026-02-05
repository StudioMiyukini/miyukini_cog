# MiyuClicker — Document Fondateur

## Contexte

**MiyuClicker** est le **premier jeu officiel Miyukini**. Il sert de **démo vivante** pour montrer qu’il est possible de faire coexister plusieurs services (Opérateurs, Toolkits) au sein d’un même environnement COG, tout en offrant une expérience de jeu complète : Idle / Clicker côté gestion, et grande stratégie (type Risk) côté conquête territoriale.

Le jeu est développé en **Rust**, s’appuie sur la **stack UI officielle Miyukini (egui / eframe)** et sur un **pack UI open-source à licence permissive**. Il consomme les Toolkits et Opérateurs nécessaires (gestion UI, animations, sprites, sauvegarde, etc.) en privilégiant au maximum les **solutions internes** à l’écosystème Miyukini.

Ce document est le **document fondateur** du jeu : il en fixe la raison d’être, l’analyse marché, les besoins métier et techniques (Toolkits), le gameplay, les versions prévues (0.1, beta v1.0) et les inspirations.

## Portée / Scope

- **Périmètre :** Définition du jeu MiyuClicker — positionnement, analyse marché Idle/RPG/gestion, besoins métier et Toolkits, gameplay (gestion + carte stratégique), roadmap versions, stack technique, intégration COG.
- **Hors périmètre :** Spécifications détaillées d’implémentation (crates, API), assets graphiques, contenu narratif.

---

## 1. Analyse PR du marché — Idle / Clicker, RPG et Gestion

### 1.1 Tendances du genre (2024–2025)

| Aspect | Constat |
|--------|--------|
| **Idle / RPG** | Forte croissance en 2024 (Q2), notamment en APAC ; titres comme *Legend of Mushroom* dominent en Corée du Sud et au Japon. Le genre mêle boucles idle et mécaniques RPG (progression, stats, équipement). |
| **Idle / Gestion** | Fusion avec city-builder et simulation : gestion de ressources, de population, de bâtiments, avec boucles de production (primaire → secondaire → tertiaire). |
| **Hybrides** | Les joueurs attendent à la fois du « clicker » simple (Cookie Clicker, AdVenture Capitalist, Clicker Heroes) et des couches plus profondes : arbres de compétences, recherche, commerce, diplomatie, conquête. |
| **Références durables** | Cookie Clicker (2013), AdVenture Capitalist (2014), Clicker Heroes (2014) restent des piliers ; Melvor Idle, NGU Idle, Idle Champions of the Forgotten Realms illustrent l’évolution vers plus de profondeur. |

### 1.2 Positionnement MiyuClicker

- **Idle / Gestion** : boucle de ressources (nourriture, matières premières, outils, recherche, armes), allocation de « gens » (population), moral et fécondité, soldats.
- **Grande stratégie** : carte de cités-États, conquête type Risk, temps de déplacement des troupes, bonus de tribu par cité conquise.
- **Démo COG** : le jeu prouve que plusieurs Opérateurs et Toolkits peuvent coexister dans un même environnement (simulation, UI, carte, sauvegarde, etc.).

---

## 2. Besoins métier et Toolkits

### 2.1 Vue d’ensemble

Le jeu nécessite des **capacités** couvrant : UI, rendu 2D (carte, sprites), animations par frame, gestion des spritesheets, entrées utilisateur, sauvegarde/chargement, simulation (tick), et éventuellement son. **Priorité : privilégier les solutions internes** (Toolkits Miyukini, egui/eframe) avant d’intégrer des crates externes.

### 2.2 Besoins détaillés et Toolkits associés

| Besoin | Description | Solution privilégiée (interne) | Solution externe si nécessaire |
|--------|-------------|---------------------------------|----------------------------------|
| **UI principale** | Menus, panels, boutons, indicateurs de ressources, listes (gens, soldats, cités). | **egui / eframe** (stack UI officielle Miyukini). | — |
| **Pack UI / thème** | Look cohérent, couleurs, typo, composants réutilisables. | Thème egui dérivé du style Miyukini (voir [Stack UI egui eframe](../../ux_ui/Miyukini%20-%20Stack%20UI%20egui%20eframe.md)) ; **packs UI jeux** présents dans `ui/game_ui_pack` (voir [MiyuClicker - Reference Packs UI Jeux](MiyuClicker%20-%20Reference%20Packs%20UI%20Jeux.md)) : Cute_Fantasy_UI (principal), Cute_Fantasy (sprites, tuiles, icônes), modernuserinterface-win (alternative, portraits). | Packs déjà présents en interne ; vérifier licences par pack (pas de redistribution des assets bruts). |
| **Rendu 2D (carte)** | Carte stratégique : nœuds (cités), arêtes (routes), déplacements, sélection. | **egui** : custom painting (`ui.painter()`) ou zone dédiée avec primitives (cercles, lignes, polygones). Textures pour fond/tiles si besoin. | Si besoin moteur 2D dédié : crates Rust à licence permissive (ex. macroquad pour canvas jeu uniquement). |
| **Sprites et spritesheets** | Personnages, unités, bâtiments, icônes ; animations par frame. | **egui** : `egui::Image` / textures à partir d’images ; découpage spritesheet en sous-rectangles ; frame courante = index dans la sheet. **Assets** : `ui/game_ui_pack` — Cute_Fantasy (bâtiments, tuiles, NPCs, icônes ressources), Tiny RPG (unités, héros), ui-icn_fantasy-weapons_01 (icônes armes) — voir [Reference Packs UI Jeux](MiyuClicker%20-%20Reference%20Packs%20UI%20Jeux.md). Toolkit interne : chargement + cache de textures, définition d’animations (plage de frames, FPS). | Crate type `image`, `png` ; éventuellement macroquad/bevy si on décide d’un rendu jeu séparé de l’UI egui. |
| **Animations par frame** | Mise à jour du numéro de frame en fonction du temps (delta). | **Toolkit interne** : boucle de jeu avec `ctx.request_repaint()` ; état `(sprite_id, animation_id, t_accumulator)` ; avancement `t += delta`, sélection de la frame. | — |
| **Gestion des sprites** | Chargement, cache, libération ; résolution des rectangles par (spritesheet, row, col) ou par ID. | **Toolkit interne** : registre de sprites (path ou bytes, dimensions, layout en grille) ; API du type `sprite_rect(sheet_id, frame_index) -> Rect`. | Crate `image` pour décodage. |
| **Entrées** | Clics, survol, clavier (raccourcis, navigation). | **egui** : `ui.input()`, réponses des widgets (`.clicked()`, `.hovered()`), zones interactives sur la carte. | — |
| **Boucle de gameplay (tick)** | Simulation discrète ou continue (ressources, moral, population, déplacements des troupes). | **Logique métier interne** : état du monde (ECS ou structs selon complexité) ; `tick(delta)` appelé depuis `App::update` ; pas de logique dans l’UI. | — |
| **Sauvegarde / chargement** | Persistance de la partie (ressources, cités, troupes, carte). | **eframe** : feature `persistence` + `App::save` / `App::load` ; sérialisation (serde) de l’état du jeu. Optionnel : KindMother si on veut centraliser les sauvegardes côté COG (hors scope v0.1). | `serde`, `serde_json` (déjà courants en Rust). |
| **Temps réel / temps simulé** | Vitesse du jeu (pause, x1, x2), temps de déplacement des troupes. | **Horloge interne** : temps simulé séparé du temps réel ; `Clock` Kernel si alignement avec la trace (optionnel). | — |
| **Son (optionnel v0.1)** | Sons d’interface, ambiances. | Toolkit interne ou crate audio permissive (MIT/Apache-2.0). | `rodio`, `kira` (licences permissives). |

### 2.3 Synthèse Toolkits à prévoir (interne)

| Toolkit | Rôle |
|---------|------|
| **UI (egui/eframe)** | Déjà stack officielle ; fenêtre principale, panels gestion + carte, widgets. |
| **Sprites / Spritesheets** | Chargement images, cache textures egui, découpage en frames, registre (sheet_id, frame) → Rect. |
| **Animation par frame** | Avancement temporel des animations (delta), sélection de la frame, boucle ou one-shot. |
| **Carte stratégique** | Modèle (nœuds, arêtes, positions) ; rendu egui (painter) ; interaction (clic, survol). |
| **Sauvegarde** | Sérialisation état jeu ; intégration eframe persistence. |
| **Simulation (tick)** | Moteur de règles métier : ressources, gens, moral, soldats, conquêtes, déplacements. |

---

## 3. Gameplay

### 3.1 Volée 1 — Gestion (fenêtre principale)

- **Boucle type Idle** : le joueur gère des **ressources** en y allouant des **gens**.
- **Ressources** : nourriture, bois, pierre, fer, outils, recherche, armes. Les **matières premières** (bois, pierre, fer) et les **produits manufacturés** (outils, armes) sont des **catégories** pour les caps de stockage, pas des ressources affichées en tant que telles — voir [MiyuClicker - Ressources et Categories](MiyuClicker%20-%20Ressources%20et%20Categories.md).
- **Gens** : consomment de la nourriture et ont des **besoins**. Ils peuvent produire : nourriture, matières premières, outils, recherche, armes. Une partie des gens peut devenir **soldats** (comptés dans le total de gens).
- **Dynamique** : la quantité de gens disponible est **dynamique** (moral, fécondité). Si la nourriture manque : moral baisse, fécondité baisse, la population peut diminuer. Le joueur doit **répartir** les gens pour maintenir croissance ou équilibre.
- **Objectif gestion** : avoir assez de troupes pour **défendre** le territoire et, à terme, **conquérir** la carte.

### 3.2 Volée 2 — Carte stratégique (grande stratégie)

- **Carte** : maillage de **cités-États** reliées par des **routes**.
- **Actions** : le joueur clique sur une cité et peut décider d’**envoyer X soldats** pour la conquérir. Un peu de **hasard** + les **stats des troupes** de chaque camp déterminent le vainqueur et les troupes restantes (style Risk).
- **Déplacement** : chaque troupe envoyée met un **temps variable** pour atteindre la cité cible (dépendant de la route / distance).

### 3.3 Version 0.1 — Comportement des cités adverses

- Les cités adverses **n’ont pas de comportement** (pas d’IA offensive/diplomatique).
- Elles possèdent un **nombre de troupes** qui **évolue jusqu’à un plafond**, selon une courbe calquée sur l’évolution du joueur pour garder un minimum de **challenge**.
- Cités **proches** : plus faciles ; cités **lointaines** ou ayant eu le temps d’évoluer : plus difficiles.
- **Cité conquise** : donne un **bonus en ressources** au joueur (type « tribu »).

### 3.4 Conception Beta v1.0 (évolution prévue)

- **Diplomatie** : relations entre cités, alliances, traités.
- **Économie de marché** : ressources et outils plus variés, **caravanes** qui circulent, cités **spécialisées** (ressources).
- **Routes multiples** : plusieurs chemins vers une même destination, avec **checkpoints** (village, fort) ; routes plus longues mais plus sûres, raccourcis (ex. forêt) avec ralentissement.
- **Résolution des combats** : en **RTS** ou **temps réel** (au lieu d’un simple jet déterministe/aléatoire).
- **Héros** : impact sur les batailles et le gameplay. Ex. : **Général** (direction des combats en RTS/RPG action), **Artisan** (mini-jeux pour artefacts boost), **Érudit** (mini-jeux pour recherche).
- **Arbre de technologie** et **3 arbres de compétences**.

---

## 4. Stack technique et choix

### 4.1 Langage et UI

| Élément | Choix | Référence |
|--------|--------|------------|
| **Langage** | Rust | Écosystème Miyukini, performance, sécurité. |
| **UI** | egui + eframe | Stack UI officielle Miyukini ; licence MIT/Apache-2.0 ; desktop + Web (WASM) + Android. |
| **Pack UI / assets** | Pack open-source à licence permissive | Pour cohérence visuelle (thème, icônes, sprites génériques) ; pas de dépendance propriétaire. |

### 4.2 Rendu jeu (carte, sprites)

- **Option A (recommandée v0.1)** : tout dans **egui** — carte en custom painting, sprites via `egui::Image` et textures (spritesheets découpées). Un seul cadre (eframe), une seule boucle.
- **Option B** : si besoin de rendu 2D plus riche (effets, nombreux sprites animés), intégration d’un moteur 2D Rust (ex. macroquad) dans une fenêtre ou un viewport ; **egui** reste pour les menus et HUD. À trancher en phase d’implémentation.

### 4.3 Licences

- **egui / eframe** : MIT ou Apache-2.0.
- **Pack UI / sprites** : choix d’un pack ou d’assets **MIT, Apache-2.0, CC0** (ou équivalent permissif) pour éviter toute contrainte commerciale ou d’attribution forte.

---

## 5. Intégration COG — Opérateurs et Services

### 5.1 Rôle du jeu dans l’écosystème

- MiyuClicker est un **Opérateur d’Interface** (ou un agrégat d’Opérateurs) qui **consomme** des Toolkits (UI, sprites, animation, sauvegarde, simulation) et prouve que **plusieurs services** peuvent vivre dans le même environnement Miyukini.
- La **logique métier** (simulation, règles de combat, économie) peut être exposée sous forme de **Tools** ou de **Opérateurs de Service** réutilisables (ex. « Simulation Idle », « Résolution combat ») pour d’autres jeux ou démos.

### 5.2 Opérateurs identifiés (vision)

| Opérateur | Rôle | Type |
|-----------|------|------|
| **MiyuClickerUI** | Interface principale (gestion + carte), menus, HUD. | Opérateur d’Interface |
| **MiyuClickerSim** | Simulation tick (ressources, gens, moral, troupes, déplacements). | Opérateur de Service |
| **MiyuClickerCombat** | Résolution des combats (stats, hasard, troupes restantes). | Opérateur de Service / Tool |
| **MiyuClickerSave** | Sauvegarde / chargement de partie (état monde). | Opérateur de Service ou usage eframe + KindMother optionnel |
| **MiyuClickerCarte** | Modèle carte (cités, routes), déplacements, combats. | Opérateur de Service |

Les **Toolkits** (Sprites, Animation, Carte, IdleSim, Save, Combat) sont des **Kits d’Outils** ou **Outils** (Strate 6) utilisés par ces Opérateurs, gouvernés par Master Butler et les Cores. **Détail MVP et mapping :** [MiyuClicker - MVP Ecrans et Mecaniques](MiyuClicker%20-%20MVP%20Ecrans%20et%20Mecaniques.md), [MiyuClicker - Operateurs et Toolkits](MiyuClicker%20-%20Operateurs%20et%20Toolkits.md).

---

## 6. Inspirations fortes

| Jeu / univers | Apport pour MiyuClicker |
|---------------|--------------------------|
| **Songs of Syx** | City-builder fantasy, simulation de population, chaînes de production (primaire → secondaire → tertiaire), gestion de bonheur/capacité, grande échelle. |
| **Emperor : L’empire du milieu** | City-builder historique, gestion des ressources et des habitants, ambiance et progression. |
| **Hearts of Iron 4** | Grande stratégie, carte, troupes, déplacements, conquête. |
| **Risk** | Conquête par territoires, résolution de batailles par jets + troupes, objectif de contrôle de la carte. |

---

## 7. Décisions structurantes

| Id | Décision | Justification |
|----|----------|---------------|
| **DS-01** | Premier jeu officiel Miyukini = démo multi-services COG | Prouver la coexistence d’Opérateurs et Toolkits dans un même environnement. |
| **DS-02** | Rust + egui/eframe + pack UI permissif | Alignement stack Miyukini, licence sans contrainte commerciale. |
| **DS-03** | Privilégier solutions internes (Toolkits UI, sprites, animation, sauvegarde) | Réutilisabilité, gouvernance, cohérence avec la pyramide Miyukini. |
| **DS-04** | Deux volées : Gestion (Idle) + Carte (grande stratégie) | Boucle idle claire + objectif long terme (conquête). |
| **DS-05** | Version 0.1 : cités sans IA, courbe de troupes adverses, bonus tribu | Scope maîtrisable ; beta v1.0 pour diplomatie, marché, routes, héros, RTS. |
| **DS-06** | Inspirations : Songs of Syx, Emperor, HOI4, Risk | Ancrage dans des références reconnues Idle / gestion / stratégie. |

---

## 8. Packs UI jeux (ui/game_ui_pack)

Le répertoire **`ui/game_ui_pack`** contient des packs d’assets UI et graphiques pour jeux, analysés et inventoriés dans un document dédié :

| Pack | Rôle pour MiyuClicker | Licence (résumé) |
|------|------------------------|-------------------|
| **Cute_Fantasy** | Bâtiments, tuiles carte, NPCs (« gens »), icônes ressources (nourriture, matières, outils), décors. | Commercial / non commercial, modifiable ; pas de redistribution. |
| **Cute_Fantasy_UI** | Barres, boutons, cadres, sliders, icônes UI, polices — **pack UI principal**. | Idem Cute_Fantasy. |
| **modernuserinterface-win** | UI alternative « moderne » ; Portrait Generator (héros, beta v1.0) ; ex. animations (GIF). | Commercial / non commercial (sauf NFT) ; crédits requis ; pas de redistribution. |
| **Tiny RPG Character Asset Pack** | Unités (soldats, types), héros, projectiles. | À vérifier (non documentée dans le pack). |
| **ui-icn_fantasy-weapons_01** | Icônes d’armes fantasy (équipement, type d’unité). | Contrat PDF (Misbug) — à consulter. |
| **CatUIFree** | Contenu minimal (2 PNG). | À vérifier. |

**Référence complète :** [MiyuClicker - Reference Packs UI Jeux](MiyuClicker%20-%20Reference%20Packs%20UI%20Jeux.md) — inventaire détaillé, licences, mapping besoin → pack, règles d’usage.

---

## 9. Références

| Document | Lien |
|----------|------|
| **Stack UI egui / eframe** | [Miyukini - Stack UI egui eframe](../../ux_ui/Miyukini%20-%20Stack%20UI%20egui%20eframe.md) |
| **Packs UI jeux** | [MiyuClicker - Reference Packs UI Jeux](MiyuClicker%20-%20Reference%20Packs%20UI%20Jeux.md) |
| **Glossaire Miyukini** | Miyukini Conceptual References - Glossaire (Opérateur, Toolkit, COG) |
| **Document Fondateur type** | [Miyukini Sales - Document Fondateur](../../services/MiyukiniSales/Miyukini%20Sales%20-%20Document%20Fondateur.md) |

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01  
**Statut :** Document fondateur — premier jeu officiel Miyukini
