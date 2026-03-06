# P0 Temps 04 - Inventaire des prerequis

## Competences et ressources requises

- Engine runtime
  - ECS, tick deterministe, event bus, save/load, streaming d'assets
- Rendering
  - `wgpu`, `winit`, sprite batching, atlas, palette lighting, post-process sobre
- Audio
  - `kira` pour mixage, layering, ducking, SFX reactifs
- Collision et navigation
  - grille logique, broadphase, hit volumes, pathfinding 2D
- Packaging
  - manifeste `miyumarket`, install dir Central, executables per-platform

## Etat du depot

- `mge/` existe mais n'a pas encore de workspace ni de code
- La racine du workspace documente deja l'isolement souhaite de `mge/`
- `apps/central` sait installer des services avec binaire et manifeste
- `Central` attend des services `Standalone` lances comme processus independants

## Etapes macro retenues

1. Fonder le workspace `mge/` et le runtime minimal.
2. Construire le renderer 2D isometrique et le pipeline d'animation.
3. Construire les systemes D2, la surface UI et l'itemisation du MVP.
4. Produire le camp de depart et l'Acte 1 complet.
5. Integrer packaging, manifests, exploitation locale, tests et launch depuis Central.

## Decision structurelle

`mge/` sera un workspace autonome avec ses propres crates moteur, outils et jeux. La racine COG ne reference pas ces crates comme membres workspace tant que l'integration n'est pas volontairement formalisee.
