# MGE Pathfinding — Démo labyrinthe

Démo technique pour démontrer les calculs de pathfinding A* par une entité dans un labyrinthe avec obstacles. Conforme aux spécifications MGE : [pathfinding](../../docs/Miyukini_Game_Engine/points/03-deplacement-locomotion/pathfinding.md).

## Lancement

```bash
cargo run -p mge-pathfinding-labyrinthe
```

Ou depuis le dossier démo :

```bash
cd demos/mge-pathfinding-labyrinthe
cargo run
```

## Usage

- **Clic gauche** : Cliquer sur une case accessible pour calculer un chemin et déplacer l'entité
- **Échap** : Quitter

L'A* minimise le coût total ; l'entité préfère les routes (marron) aux chemins en sable (jaune) ou forêt (vert foncé).

## Tests

```bash
cargo test -p mge-pathfinding-labyrinthe
```

## Spécifications

| Élément | Implémentation |
|---------|----------------|
| Grille | 32×32, terrain avec coûts de déplacement |
| Terrain | Route (0.8), Herbe (1.0), Sable (1.5), Forêt (2.0), Obstacle (∞) |
| A* | 8 directions, heuristique euclidienne, coût diagonal × terrain |
| Entité | Vitesse constante 120 px/s, waypoints |
| Rendu | minifb, couleurs par terrain (route, herbe, sable, forêt, mur) |

## Structure

```
demos/mge-pathfinding-labyrinthe/
├── src/
│   ├── main.rs       # Boucle jeu, input clic
│   ├── grid.rs       # Grille, GridNode, Vec2
│   ├── pathfinding.rs # A*
│   ├── entity.rs     # Entité, waypoints
│   └── render.rs    # Dessin
├── Cargo.toml
└── README.md
```

## Références

- [Plan d'implémentation](../../docs/implementation/MGE%20-%20Plan%20Demo%20Pathfinding%20Labyrinthe.md)
- [Point pathfinding MGE](../../docs/Miyukini_Game_Engine/points/03-deplacement-locomotion/pathfinding.md)
- [Paramètres déplacement entité](../../docs/Miyukini_Game_Engine/MGE%20-%20Parametres%20Deplacement%20Entite.md)
