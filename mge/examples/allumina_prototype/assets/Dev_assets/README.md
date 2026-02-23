# Assets Dev

- **Test_joueur.png** : Sprite du joueur (lettre P sur fond bleu)
- Copie depuis `assets/Dev_assets/` à la racine du projet si absent

## Panneau Dev Dioxus

Pour l'interface complète avec labels (Grille, Métriques, etc.), lancer en parallèle :

```bash
cargo run -p allumina_prototype --bin dev_ui_dioxus
```

Le panneau Dioxus écrit les options dans `%LOCALAPPDATA%/AlluminaDev/options.json`.
Le jeu les lit et synchronise automatiquement les overlays.
