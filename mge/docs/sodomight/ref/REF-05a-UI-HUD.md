# REF-05a -- UI, HUD, Menus & Controles -- Reference Compacte
<!-- @id REF-05a @do reference-ui-hud @role Fabrice @layer 7 @human miyuki -->
<!-- Split de REF-05 le 2026-03-03. Formules D2, valeurs exactes, MGE mapping, sprints, cross-refs -->

**Projet** : Sodomight | **Moteur** : MGE | **Ref D2** : v1.14d / D2R 2.x

> **TL;DR** : Specification complete du HUD D2 en 800x600 : layout pixel-perfect des orbes Life/Mana, belt 4 tiers, minimap/automap, 8 fenetres UI (inventaire, char screen, skill tree, stash, cube, trade, vendor, quest log), tailles items en grille, 10 equip slots, controles souris/clavier remappables, et menus (main, lobby, char select/create, pause, death). Tout mappe vers `mge-ui` et `mge-platform`.

> **Voir aussi** : [REF-05b](REF-05b-Render-Audio-OpenD2.md) pour le rendu graphique, audio et projets open source D2.

---

## 1. UI / HUD (800x600) `[S0-S1]`

> `-> MGE: mge-ui::HudLayout, sodomight::ui::hud` | Cross-ref: [REF-02a SS3](REF-02a-Combat-Formulas.md) (breakpoints affichage)

### 1.1 Layout HUD `[S0]`

> `-> MGE: mge-ui::HudPanel` | TOML: `assets/ui/hud_layout.toml`

```
+--800px--------------------------------------------------------------+
|                                                 [Minimap overlay]   | Y=0
|                      ZONE DE JEU 800x496                            |
|                      (isometrique 2:1)                              |
+---------------------------------------------------------------------+ Y=496
|  [Stamina Bar]                                                      |
+=============================PANEL===================================+ Y=508
|  +------+  +---+  +----+  [Belt]  +----+  +---+  +------+          |
|  | LIFE |  |LMB|  |Slot|  4x1-4   |Slot|  |RMB|  | MANA |          |
|  | ORB  |  |SKL|  | 1  |  center  | 8  |  |SKL|  | ORB  |          |
|  +------+  +---+  +----+          +----+  +---+  +------+          |
+---------------------------------------------------------------------+ Y=596
|[XP=========================================================XP Bar]| Y=596-600
+---------------------------------------------------------------------+
```

### 1.2 Positions des elements (800x600) `[S0]`

> `-> MGE: mge-ui::HudElement` | TOML: `assets/ui/elements.toml`

| Element | X (centre) | Y (centre) | Taille | Rendu | Comportement |
|---------|------------|------------|--------|-------|-------------|
| Life Orb | 60 | 560 | ~80x80 ellipse | Shader masque circulaire + fill rect clippe + overlay additif | Rouge #B41414, fill bas->haut, hover "Life: X/Y" |
| Mana Orb | 740 | 560 | ~80x80 ellipse | Idem Life Orb | Bleu #1428B4, fill bas->haut, hover "Mana: X/Y" |
| LMB Skill | 117 | 555 | 50x50 | Icone skill | Clic droit = menu radial skills |
| RMB Skill | 683 | 555 | 50x50 | Icone skill | F1-F8 hotkeys |
| Belt slot [0,0] | 354 | 593 | 29x29/slot | Sprite item | 4 cols x 1-4 rows, auto-fill gravite |
| XP Bar | 0 (left) | 596 | 800x4 | Fill rectangle | Violet/dore, fill G->D |
| Stamina Bar | 350 | 505 | ~100x8 | Fill rectangle | Jaune->Rouge, R=toggle run/walk |
| Chat | 10 | 480 | Variable | Text overlay | Entree=toggle, fade ~10s |

**Orb shader pipeline** : `masque_alpha(circulaire) * fill_rect(HP%_bas_haut) * color(#B41414) + overlay_additif(reflets)`

### 1.3 Belt par type `[S0]`

> `-> MGE: mge-arpg-items::BeltDefinition` | Cross-ref: **REF-03 SS1.2** (belt item slots)

| Belt tier | Rows | Slots | Exemples |
|-----------|------|-------|----------|
| Aucun | 1 | 4 | Pas de belt |
| Sash / Light Belt | 2 | 8 | Sash, Light Belt |
| Belt / Heavy Belt | 3 | 12 | Belt, Heavy Belt |
| Plated / Exceptional / Elite | 4 | 16 | Plated Belt, War Belt, Mithril Coil |

Touches 1-4 consomment colonne (rangee basse d'abord, auto-monte). Items : potions, scrolls (TP, Identify).
Espacement inter-slot : 29px horizontal, 29px vertical.

### 1.4 Minimap / Automap (Tab) `[S1]`

> `-> MGE: mge-ui::AutomapRenderer, mge-arpg-world::MapExplored`

Mode overlay (plein ecran, ~50% alpha) ou minimap (coin haut-droit).

| Element | Couleur | Notes |
|---------|---------|-------|
| Murs | Blanc (#FFFFFF) | Ligne fine |
| Portes | Rouge (#FF0000) | Ligne epaisse |
| Waypoint | Violet (#FF00FF) | Croix |
| Town Portal | Bleu (#0000FF) | Croix |
| Joueur | Fleche blanche | Direction |
| Party members | Vert (#00FF00) | Croix |
| Shrines | Croix jaune (#FFFF00) | -- |
| Monsters | -- | Non visible (D2R: rouge en option) |

Fog of war : bitfield 1 bit/sub-tile dans `MapExplored`. Seules zones visitees visibles.
V=deplace. Molette=zoom. Rendu wireframe overlay : apres monde, avant HUD.

### 1.5 Fenetres UI `[S1-S2]`

> `-> MGE: mge-ui::UiWindow` | TOML: `assets/ui/windows.toml`

| Fenetre | Raccourci | Cote | Taille approx | Notes |
|---------|-----------|------|---------------|-------|
| Character Screen | C | Gauche (400x596) | Stats, paperdoll, 4 attributs + [+] | Cross-ref: [REF-02a SS2.1](REF-02a-Combat-Formulas.md) (stats) |
| Inventory | I | Droite (400x596) | Grille 10x4 (29x29/slot), paperdoll equip | Cross-ref: **REF-03 SS1.2** |
| Skill Tree | S/T | Droite | 3 onglets, prereqs, synergies | Cross-ref: [REF-02b](REF-02b-Classes-Skills.md) (skills) |
| Quest Log | Q | Gauche | 5 onglets acte, 6 quetes/acte (3 A4) | Cross-ref: [REF-04a SS1.7](REF-04a-World-Zones.md) |
| Stash | Clic coffre | Gauche | LoD: 6x8=48 slots, D2R: 10x10+2 partages | Cross-ref: **REF-03 SS1.2** |
| Horadric Cube | Clic droit | Popup | Interne 3x4=12 slots, Transmute bouton | Cross-ref: **REF-03 SS4.1** |
| Trade | Auto | Centre | Double confirmation, reset si modif | Cross-ref: [REF-04b SS2.1](REF-04b-Monsters-Multiplayer.md) |
| Vendor | Clic NPC | Droite | Buy/Sell/Repair + Gambling | Cross-ref: [REF-04a SS1.5](REF-04a-World-Zones.md) |

**Exclusivites** : Char Screen XOR Quest Log (gauche). Inventory XOR Skill Tree (droite). Cube = independant.

**Tailles items inventaire** :

| Taille | Exemples |
|--------|----------|
| 1x1 | Potions, gems, runes, rings, amulets, bolts/arrows, ears, keys |
| 1x2 | Boots, gloves, belts |
| 1x3 | Wands, sceptres, daggers |
| 2x2 | Helms, small shields, skulls |
| 2x3 | Armors, large shields, axes |
| 2x4 | Longbows, spears, polearms |

**Equip slots (10)** : Head, Amulet, Torso, Left/Right Arm, Left/Right Ring, Gloves, Belt, Boots.
`-> MGE: mge-arpg-items::Inventory { grid: BitGrid<10,4> }`. Collision O(1) bitfield 40 bits. Swap auto obligatoire.

### 1.6 Controles `[S0]`

> `-> MGE: mge-platform::InputMapper` | TOML: `assets/config/keybindings.toml`

**Souris** :

| Action | Input | Latence max | Notes |
|--------|-------|-------------|-------|
| Move/Attack/Interact/Pickup | LClick | <1 frame | Contexte-dependant |
| Skill RMB | RClick | <1 frame | Skill actif RMB |
| Stationnaire | Shift+Click | <1 frame | Attaque sans bouger |
| Stash quick-move | Ctrl+Click | -- | Item -> stash |
| **Noms items au sol** | **Alt (hold)** | **<1 frame** | **CRITIQUE UX** |
| Skill menu | LClick/RClick sur icone | -- | Menu radial selection |

**Clavier** : F1-F8=RMB slots, 1-4=belt, I/C/S/Q/O/M=fenetres, R=run/walk, W=weapon swap, Esc=menu, Space=fermer tout, Enter=chat.

`WindowEvent(winit) -> InputMapper -> GameAction(enum)`. Remappable via TOML.

### 1.7 Menus `[S0-S1]`

> `-> MGE: mge-ui::MenuScreen`

| Menu | Contenu | Anim/Fond |
|------|---------|-----------|
| Main Menu | SP/MP/Cinematics/Credits/Exit | Fond flammes loop (sprite anim 25fps) |
| Char Select | 20 slots/page, delete=type name confirm | Char idle anim preview |
| Char Creation | 7 classes, idle anim, Expansion/HC/Ladder toggles | Full body anim |
| Lobby (MP) | Create/Join, game list, chat channels | -- |
| Pause | Solo=pause reelle, Multi=overlay sans pause | Desature background |
| Death | Desature, respawn fantome ville | Corps a recuperer, perte or+XP(NM/H) |
