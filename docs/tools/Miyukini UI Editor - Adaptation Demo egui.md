# Miyukini UI Editor — Adaptation démo egui (tous les outils)

## Contexte

Le service **Miyukini UI Editor** peut être étendu pour exposer les mêmes outils que la démo officielle [egui.rs/#demo](https://www.egui.rs/#demo) : sidebar à droite avec About egui, Bézier Curve, Code Editor, Widget Gallery, Modals, etc., et fenêtres de démo correspondantes.

La bibliothèque officielle `egui_demo_lib` vit dans le workspace [emilk/egui](https://github.com/emilk/egui) et dépend de `egui` / `egui_extras` via `workspace = true`. L’utiliser en dépendance path depuis miyukini-central impose soit d’inclure tout le workspace egui, soit de vendre le code et l’adapter.

## Contraintes

1. **API egui 0.33 (crates.io)** : `Window::show(self, ctx: &Context, add_contents: ...)` prend un `&Context`, pas un `&mut Ui`. Le code source actuel de `egui_demo_lib` dans `deps/egui` appelle encore `.show(ui, |ui| ...)` pour les fenêtres.
2. **`request_repaint`** : dans egui 0.33, `request_repaint()` est sur `Context`, pas sur `Ui`. Le code démo utilise `ui.request_repaint()` ; il faut utiliser `ui.ctx().request_repaint()`.
3. **Workspace** : utiliser `egui_demo_lib` en path depuis notre workspace fait résoudre les `workspace = true` dans notre Cargo.toml ; les dépendances du dépôt egui (chrono, criterion, etc.) ne sont pas définies chez nous.

## Choix : crate vendue `egui_demos_miyukini`

- **Emplacement** : `crates/egui_demos_miyukini/`
- **Contenu** : copie adaptée de `deps/egui/crates/egui_demo_lib` (demos + easy_mark + lib).
- **Dépendances** : `egui = "0.33"`, `egui_extras = "0.33"` (crates.io), pas de path vers deps/egui.
- **Patchs d’API** :
  1. Pour chaque fenêtre démo : remplacer `.show(ui, |ui|` par `.show(ui.ctx(), |ui|` lorsque l’appel est celui de `Window` (chaîne issue de `Window::new(...)`).
  2. Remplacer `ui.request_repaint()` par `ui.ctx().request_repaint()`.
  3. Corriger les éventuelles erreurs d’inférence de type (annotations de closure) si besoin.

Seul `Window::show` prend `Context` ; `Grid::show`, `ScrollArea::show`, `Frame::show`, `Resize::show`, `CollapsingHeader::show` prennent `&mut Ui` et ne doivent pas être modifiés.

## Intégration dans Miyukini UI Editor

- `miyukini-central` dépend de `egui_demos_miyukini`.
- Dans `EguiEditorService::show()` : appliquer le thème, panneau gauche (options thème), puis `demo_windows.ui(ui)` pour afficher la sidebar droite et toutes les fenêtres démo.
- Appeler `egui_extras::install_image_loaders(ctx)` une fois pour les démos qui chargent des images (ex. Widget Gallery).

## Fichiers impactés

- Nouveau : `crates/egui_demos_miyukini/` (Cargo.toml + arborescence copiée de egui_demo_lib, avec patchs).
- Modifié : `crates/miyukini-central/Cargo.toml` (dépendance vers egui_demos_miyukini), `crates/miyukini-central/src/services/egui_editor.rs` (utilisation de `DemoWindows`).
- Référence : `deps/egui/crates/egui_demo_lib/` (source à copier et patcher).

## Écart d’API (dépôt egui vs crates.io 0.33)

Le code source dans `deps/egui` cible une version plus récente d’egui que 0.33 sur crates.io. Une crate vendue `egui_demos_miyukini` a été créée et les patchs `Window::show(ctx)` et `request_repaint()` ont été appliqués, mais la compilation révèle en plus :

- **Rust 2024** : let chains utilisés dans le code démo (edition 2024).
- **Panel** : le dépôt utilise `egui::Panel::top/right/bottom` ; en 0.33 (crates.io) il faut `TopBottomPanel` / `SidePanel` et `.show(ctx, ...)` au lieu de `.show_inside(ui, ...)`.
- **Méthodes** : `viewport_id()`, `copy_text()`, `debug_painter()`, `tessellation_options_mut()`, `send_viewport_cmd()`, `global_style()`, `options_mut()`, `copy_image()` — noms ou emplacements différents (Context vs Ui) en 0.33.
- **Données** : `data/icon.png`, `data/peace.svg`, etc. doivent être copiées au bon chemin pour `include_image!`.

Une adaptation complète impliquerait soit d’utiliser egui depuis `deps/egui` (patch Cargo + résolution du workspace), soit d’aligner toute la crate vendue sur l’API egui 0.33 (remplacements Panel, méthodes, édition, données). En l’état, **tous les outils** restent disponibles en lançant la démo officielle depuis `deps/egui` (voir section « Lancer la démo » dans l’éditeur).

## Mise à jour ultérieure

Pour resynchroniser avec la démo officielle : recopier les sources depuis `deps/egui/crates/egui_demo_lib`, réappliquer les patchs (show ctx, request_repaint), puis résoudre les écarts d’API listés ci-dessus ou basculer le projet sur egui en path depuis `deps/egui`.

## Voir aussi

- [Miyukini Central - Service Miyukini UI Editor](../services/MiyukiniCentral/Miyukini%20Central%20-%20Service%20Miyukini%20UI%20Editor.md) : document fondateur du Service Miyukini UI Editor (personnalisation persistante, thèmes, outils UI).
