# egui_demos_miyukini (travail en cours)

Crate vendue destinée à intégrer **tous les outils** de la démo egui dans Miyukini UI Editor.  
Source : copie de `deps/egui/crates/egui_demo_lib` avec patchs d’API pour egui 0.33 (crates.io).

## Statut

- **Patchs appliqués** : `Window::show(ui, ...)` → `Window::show(ui.ctx(), ...)` ; `ui.request_repaint()` → `ui.ctx().request_repaint()`.
- **Blocage** : le code source du dépôt egui cible une API plus récente que egui 0.33 sur crates.io (Panel vs TopBottomPanel/SidePanel, let chains edition 2024, méthodes `viewport_id`, `copy_text`, etc., fichiers `data/`).
- **Hors workspace** : cette crate n’est pas dans le workspace pour l’instant afin de ne pas casser la compilation.

## Pour avoir tous les outils

Lancer la démo officielle depuis la racine du projet :

```powershell
cd deps\egui
cargo run --release -p egui_demo_app
```

Voir aussi `docs/tools/Miyukini UI Editor - Adaptation Demo egui.md` et `docs/services/MiyukiniCentral/Miyukini Central - Service Miyukini UI Editor.md`.
