# Plan d'implémentation — Démo technique pathfinding labyrinthe

Démo technique pour démontrer les calculs de pathfinding par une entité dans un labyrinthe avec obstacles. Référence les spécifications MGE : [pathfinding](../Miyukini_Game_Engine/points/03-deplacement-locomotion/pathfinding.md), [Guide Entités Groupes](../Miyukini_Game_Engine/MGE%20-%20Pathfinding%20Collisions%20-%20Guide%20Entites%20Groupes.md).

---

## 1. Objectif

- **Valider** l'algorithme A* sur grille 2D
- **Démontrer** le calcul de chemin dans un labyrinthe avec obstacles statiques
- **Visualiser** le chemin calculé et le déplacement de l'entité
- **Servir de base** pour les tests et la documentation du MGE

---

## 2. Périmètre

### 2.1 Inclus (MVP)

| Élément | Description |
|---------|-------------|
| Grille 2D | Labyrinthe tile-based (ex. 32×32 ou 64×64) |
| Obstacles | Murs statiques, passages étroits |
| Entité | Un seul agent (PNJ ou joueur) |
| Algorithme A* | 8 directions, heuristique euclidienne |
| Click-to-move | Clic sur une cellule → calcul du chemin → déplacement |
| Visualisation | Grille, obstacles, chemin, entité |

### 2.2 Hors scope (phase ultérieure)

| Élément | Raison |
|---------|--------|
| Dijkstra | A* suffit pour la démo |
| Navmesh | Grille suffisante pour labyrinthe |
| Obstacles dynamiques | Simplicité |
| Coût terrain variable | Option post-MVP |
| Plusieurs entités | Une seule pour la démo |

---

## 3. Architecture technique

### 3.1 Stack proposé

| Couche | Technologie | Alternatives |
|--------|-------------|--------------|
| Langage | Rust | — |
| Rendu | wgpu + winit | SDL2, minifb |
| Ou prototype rapide | Web (HTML5 Canvas / WASM) | Validation algorithme seul |

### 3.2 Modules

```
demo-pathfinding/
├── src/
│   ├── main.rs           # Boucle jeu, fenêtre, input
│   ├── grid.rs           # Grille, tuiles, obstacles
│   ├── pathfinding.rs    # A*, PathResult
│   ├── entity.rs         # Entité, position, locomotion
│   └── render.rs         # Dessin grille, chemin, entité
├── assets/
│   └── levels/
│       └── labyrinthe_01.dat  # Format niveau (optionnel)
├── Cargo.toml
└── README.md
```

---

## 4. Phases d'implémentation

### Phase 1 : Grille et obstacles (1–2 j)

| Tâche | Description | Livrable |
|-------|-------------|----------|
| 1.1 | Structure `Grid` (width, height, Vec<u8> ou Vec<bool> par cellule) | `grid.rs` |
| 1.2 | Chargement ou génération labyrinthe (fichier ou procédural) | Niveau jouable |
| 1.3 | Convention : 0 = walkable, 1 = obstacle | API `is_walkable(x, y)` |
| 1.4 | Rendu grille (carrés colorés : blanc = sol, noir = mur) | Affichage labyrinthe |

### Phase 2 : Algorithme A* (2–3 j)

| Tâche | Description | Livrable |
|-------|-------------|----------|
| 2.1 | Structures `GridNode`, `PathResult` (waypoints, total_cost, found) | `pathfinding.rs` |
| 2.2 | Heuristique euclidienne : `sqrt(dx² + dy²)` | Fonction `heuristic` |
| 2.3 | Voisins 8 directions, coût diagonal √2 | Fonction `neighbors` |
| 2.4 | Implémentation A* (open list, closed set) | `find_path(start, goal) -> PathResult` |
| 2.5 | Tests unitaires : chemin direct, contournement mur, chemin inexistant | `pathfinding` validé |

### Phase 3 : Entité et locomotion (1–2 j)

| Tâche | Description | Livrable |
|-------|-------------|----------|
| 3.1 | Entité : position (Vec2 ou GridNode), waypoints, index waypoint actuel | `entity.rs` |
| 3.2 | Mise à jour : direction vers waypoint[0], avancement, pop waypoint atteint | Logique locomotion |
| 3.3 | Conversion GridNode ↔ position pixel (centre de tuile) | Utilitaires |
| 3.4 | Vitesse constante ou interpolation simple | Paramètre configurable |

### Phase 4 : Input et intégration (1 j)

| Tâche | Description | Livrable |
|-------|-------------|----------|
| 4.1 | Clic souris → coordonnées monde → GridNode cible | Input handling |
| 4.2 | Si cible walkable : `find_path(entity.pos, cible)` | Intégration |
| 4.3 | Remplacement waypoints entité par nouveau chemin | Nouveau clic = nouveau chemin |
| 4.4 | Gestion cible obstacle ou inaccessible : affichage feedback (optionnel) | UX basique |

### Phase 5 : Visualisation (1–2 j)

| Tâche | Description | Livrable |
|-------|-------------|----------|
| 5.1 | Dessin du chemin (lignes ou surbrillance des tuiles) | Chemin visible |
| 5.2 | Entité : sprite/carré coloré à sa position | Entité visible |
| 5.3 | Option debug : cellules explorées par A* (optionnel) | Mode debug |
| 5.4 | Légende / HUD : coût total, nombre de waypoints | Info debug |

### Phase 6 : Polish et documentation (0,5–1 j)

| Tâche | Description | Livrable |
|-------|-------------|----------|
| 6.1 | README : build, run, usage | Documentation |
| 6.2 | Captures d'écran ou GIF pour démo | Support présentation |
| 6.3 | Rédaction courte : lien avec spécifications MGE | Traçabilité |

---

## 5. Spécifications techniques détaillées

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

- **Entrée** : `start: (i32, i32)`, `goal: (i32, i32)`, `grid: &Grid`
- **Sortie** : `PathResult { waypoints: Vec<Vec2>, total_cost: f32, found: bool }`
- **Limite** : max_iterations = 10000
- **Coût** : orthogonale = 1.0, diagonale = 1.414

### 5.3 Format niveau (optionnel)

Format texte ou binaire simple :
- Ligne 1 : `width height`
- Lignes suivantes : `0` ou `1` par cellule (séparés par espace ou concaténés)

Exemple 8×8 :
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

| Phase | Durée | Cumul |
|-------|-------|-------|
| Phase 1 | 1–2 j | 1–2 j |
| Phase 2 | 2–3 j | 3–5 j |
| Phase 3 | 1–2 j | 4–7 j |
| Phase 4 | 1 j | 5–8 j |
| Phase 5 | 1–2 j | 6–10 j |
| Phase 6 | 0,5–1 j | 6,5–11 j |

**Total : 7–11 jours** (développeur familiarisé avec Rust et A*).

---

## 7. Critères de succès

- [ ] L'entité se déplace de la case de départ à la case cible en contournant les obstacles
- [ ] Le chemin affiché est continu et évite les murs
- [ ] Un clic sur une case accessible recalcule et suit le nouveau chemin
- [ ] Un clic sur un obstacle ou une case inaccessible ne produit pas de crash
- [ ] Les tests unitaires A* passent (chemin direct, contournement, échec)

---

## 8. Dépendances et références

| Document | Usage |
|----------|-------|
| [pathfinding](../Miyukini_Game_Engine/points/03-deplacement-locomotion/pathfinding.md) | Spec A*, structures, heuristiques |
| [MGE - Référence Commune](../Miyukini_Game_Engine/MGE%20-%20Reference%20Commune.md) | Vec2, Rect, coordonnées |
| [deplacement-8-directions](../Miyukini_Game_Engine/points/03-deplacement-locomotion/deplacement-8-directions.md) | Direction, waypoints |
| [monde-tile-based](../Miyukini_Game_Engine/points/01-affichage-rendu/monde-tile-based.md) | Conventions grille |

---

## 9. Emplacement du code

Proposition d'intégration dans le workspace Miyukini COG :

```
crates/
└── mge-demo-pathfinding/   # Nouveau crate démo
    ├── Cargo.toml
    ├── src/
    │   └── ...
    └── assets/
```

Ou projet standalone dans `demos/pathfinding-labyrinthe/` si préféré.

---

**Document** : Plan démo pathfinding labyrinthe  
**Version** : 1.0  
**Date** : 2026-02-18
