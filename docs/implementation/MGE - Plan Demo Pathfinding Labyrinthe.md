# Plan d'implÃ©mentation â€” DÃ©mo technique pathfinding labyrinthe

DÃ©mo technique pour dÃ©montrer les calculs de pathfinding par une entitÃ© dans un labyrinthe avec obstacles. RÃ©fÃ©rence les spÃ©cifications MGE : [pathfinding](..//_index.md), [Guide EntitÃ©s Groupes](..//_index.md).

---

## 1. Objectif

- **Valider** l'algorithme A* sur grille 2D
- **DÃ©montrer** le calcul de chemin dans un labyrinthe avec obstacles statiques
- **Visualiser** le chemin calculÃ© et le dÃ©placement de l'entitÃ©
- **Servir de base** pour les tests et la documentation du MGE

---

## 2. PÃ©rimÃ¨tre

### 2.1 Inclus (MVP)

| Ã‰lÃ©ment | Description |
|---------|-------------|
| Grille 2D | Labyrinthe tile-based (ex. 32Ã—32 ou 64Ã—64) |
| Obstacles | Murs statiques, passages Ã©troits |
| EntitÃ© | Un seul agent (PNJ ou joueur) |
| Algorithme A* | 8 directions, heuristique euclidienne |
| Click-to-move | Clic sur une cellule â†’ calcul du chemin â†’ dÃ©placement |
| Visualisation | Grille, obstacles, chemin, entitÃ© |

### 2.2 Hors scope (phase ultÃ©rieure)

| Ã‰lÃ©ment | Raison |
|---------|--------|
| Dijkstra | A* suffit pour la dÃ©mo |
| Navmesh | Grille suffisante pour labyrinthe |
| Obstacles dynamiques | SimplicitÃ© |
| CoÃ»t terrain variable | Option post-MVP |
| Plusieurs entitÃ©s | Une seule pour la dÃ©mo |

---

## 3. Architecture technique

### 3.1 Stack proposÃ©

| Couche | Technologie | Alternatives |
|--------|-------------|--------------|
| Langage | Rust | â€” |
| Rendu | wgpu + winit | SDL2, minifb |
| Ou prototype rapide | Web (HTML5 Canvas / WASM) | Validation algorithme seul |

### 3.2 Modules

```
demo-pathfinding/
â”œâ”€â”€ src/
â”‚   â”œâ”€â”€ main.rs           # Boucle jeu, fenÃªtre, input
â”‚   â”œâ”€â”€ grid.rs           # Grille, tuiles, obstacles
â”‚   â”œâ”€â”€ pathfinding.rs    # A*, PathResult
â”‚   â”œâ”€â”€ entity.rs         # EntitÃ©, position, locomotion
â”‚   â””â”€â”€ render.rs         # Dessin grille, chemin, entitÃ©
â”œâ”€â”€ assets/
â”‚   â””â”€â”€ levels/
â”‚       â””â”€â”€ labyrinthe_01.dat  # Format niveau (optionnel)
â”œâ”€â”€ Cargo.toml
â””â”€â”€ README.md
```

---

## 4. Phases d'implÃ©mentation

### Phase 1 : Grille et obstacles (1â€“2 j)

| TÃ¢che | Description | Livrable |
|-------|-------------|----------|
| 1.1 | Structure `Grid` (width, height, Vec<u8> ou Vec<bool> par cellule) | `grid.rs` |
| 1.2 | Chargement ou gÃ©nÃ©ration labyrinthe (fichier ou procÃ©dural) | Niveau jouable |
| 1.3 | Convention : 0 = walkable, 1 = obstacle | API `is_walkable(x, y)` |
| 1.4 | Rendu grille (carrÃ©s colorÃ©s : blanc = sol, noir = mur) | Affichage labyrinthe |

### Phase 2 : Algorithme A* (2â€“3 j)

| TÃ¢che | Description | Livrable |
|-------|-------------|----------|
| 2.1 | Structures `GridNode`, `PathResult` (waypoints, total_cost, found) | `pathfinding.rs` |
| 2.2 | Heuristique euclidienne : `sqrt(dxÂ² + dyÂ²)` | Fonction `heuristic` |
| 2.3 | Voisins 8 directions, coÃ»t diagonal âˆš2 | Fonction `neighbors` |
| 2.4 | ImplÃ©mentation A* (open list, closed set) | `find_path(start, goal) -> PathResult` |
| 2.5 | Tests unitaires : chemin direct, contournement mur, chemin inexistant | `pathfinding` validÃ© |

### Phase 3 : EntitÃ© et locomotion (1â€“2 j)

| TÃ¢che | Description | Livrable |
|-------|-------------|----------|
| 3.1 | EntitÃ© : position (Vec2 ou GridNode), waypoints, index waypoint actuel | `entity.rs` |
| 3.2 | Mise Ã  jour : direction vers waypoint[0], avancement, pop waypoint atteint | Logique locomotion |
| 3.3 | Conversion GridNode â†” position pixel (centre de tuile) | Utilitaires |
| 3.4 | Vitesse constante ou interpolation simple | ParamÃ¨tre configurable |

### Phase 4 : Input et intÃ©gration (1 j)

| TÃ¢che | Description | Livrable |
|-------|-------------|----------|
| 4.1 | Clic souris â†’ coordonnÃ©es monde â†’ GridNode cible | Input handling |
| 4.2 | Si cible walkable : `find_path(entity.pos, cible)` | IntÃ©gration |
| 4.3 | Remplacement waypoints entitÃ© par nouveau chemin | Nouveau clic = nouveau chemin |
| 4.4 | Gestion cible obstacle ou inaccessible : affichage feedback (optionnel) | UX basique |

### Phase 5 : Visualisation (1â€“2 j)

| TÃ¢che | Description | Livrable |
|-------|-------------|----------|
| 5.1 | Dessin du chemin (lignes ou surbrillance des tuiles) | Chemin visible |
| 5.2 | EntitÃ© : sprite/carrÃ© colorÃ© Ã  sa position | EntitÃ© visible |
| 5.3 | Option debug : cellules explorÃ©es par A* (optionnel) | Mode debug |
| 5.4 | LÃ©gende / HUD : coÃ»t total, nombre de waypoints | Info debug |

### Phase 6 : Polish et documentation (0,5â€“1 j)

| TÃ¢che | Description | Livrable |
|-------|-------------|----------|
| 6.1 | README : build, run, usage | Documentation |
| 6.2 | Captures d'Ã©cran ou GIF pour dÃ©mo | Support prÃ©sentation |
| 6.3 | RÃ©daction courte : lien avec spÃ©cifications MGE | TraÃ§abilitÃ© |

---

## 5. SpÃ©cifications techniques dÃ©taillÃ©es

### 5.1 Grille

```rust
pub struct Grid {
    pub width: i32,
    pub height: i32,
    /// 0 = walkable, 1 = obstacle
    cells: Vec<u8>,
}

impl Grid {
    pub fn is_walkable(&self, x: i32, y: i32) -> bool;
    pub fn node_to_world(&self, node: GridNode, tile_size: f32) -> Vec2;
    pub fn world_to_node(&self, pos: Vec2, tile_size: f32) -> GridNode;
}
```

### 5.2 A*

- **EntrÃ©e** : `start: (i32, i32)`, `goal: (i32, i32)`, `grid: &Grid`
- **Sortie** : `PathResult { waypoints: Vec<Vec2>, total_cost: f32, found: bool }`
- **Limite** : max_iterations = 10000
- **CoÃ»t** : orthogonale = 1.0, diagonale = 1.414

### 5.3 Format niveau (optionnel)

Format texte ou binaire simple :
- Ligne 1 : `width height`
- Lignes suivantes : `0` ou `1` par cellule (sÃ©parÃ©s par espace ou concatÃ©nÃ©s)

Exemple 8Ã—8 :
```
8 8
00011000
00011000
00000000
11100111
00000000
00000000
00000000
00000000
```

---

## 6. Estimation

| Phase | DurÃ©e | Cumul |
|-------|-------|-------|
| Phase 1 | 1â€“2 j | 1â€“2 j |
| Phase 2 | 2â€“3 j | 3â€“5 j |
| Phase 3 | 1â€“2 j | 4â€“7 j |
| Phase 4 | 1 j | 5â€“8 j |
| Phase 5 | 1â€“2 j | 6â€“10 j |
| Phase 6 | 0,5â€“1 j | 6,5â€“11 j |

**Total : 7â€“11 jours** (dÃ©veloppeur familiarisÃ© avec Rust et A*).

---

## 7. CritÃ¨res de succÃ¨s

- [ ] L'entitÃ© se dÃ©place de la case de dÃ©part Ã  la case cible en contournant les obstacles
- [ ] Le chemin affichÃ© est continu et Ã©vite les murs
- [ ] Un clic sur une case accessible recalcule et suit le nouveau chemin
- [ ] Un clic sur un obstacle ou une case inaccessible ne produit pas de crash
- [ ] Les tests unitaires A* passent (chemin direct, contournement, Ã©chec)

---

## 8. DÃ©pendances et rÃ©fÃ©rences

| Document | Usage |
|----------|-------|
| [pathfinding](..//_index.md) | Spec A*, structures, heuristiques |
| [MGE - RÃ©fÃ©rence Commune](..//_index.md) | Vec2, Rect, coordonnÃ©es |
| [deplacement-8-directions](..//_index.md) | Direction, waypoints |
| [monde-tile-based](..//_index.md) | Conventions grille |

---

## 9. Emplacement du code

Proposition d'intÃ©gration dans le workspace Miyukini COG :

```
crates/
â””â”€â”€ mge-demo-pathfinding/   # Nouveau crate dÃ©mo
    â”œâ”€â”€ Cargo.toml
    â”œâ”€â”€ src/
    â”‚   â””â”€â”€ ...
    â””â”€â”€ assets/
```

Ou projet standalone dans `demos/pathfinding-labyrinthe/` si prÃ©fÃ©rÃ©.

---

**Document** : Plan dÃ©mo pathfinding labyrinthe  
**Version** : 1.0  
**Date** : 2026-02-18

