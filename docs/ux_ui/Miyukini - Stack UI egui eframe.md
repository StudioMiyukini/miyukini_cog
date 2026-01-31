# Miyukini — Stack UI egui / eframe

## Contexte

Miyukini adopte **egui** et **eframe** comme stack UI native pour les applications desktop (Hub de Services, outils d’administration, clients locaux). Ce choix permet un **pur Rust**, une **licence permissive** (MIT ou Apache-2.0), et une couverture **desktop + web + Android** (et embedded ultérieurement) sans contrainte commerciale ni attribution obligatoire.

**Usage cible prioritaire :** Hub de Services Miyukini (vitrine du Registre d’Opérateurs, catalogue, lancement de Services), puis tout client natif (Windows, Linux, Mac) ou web (WASM) nécessitant une UI gouvernée.

## Portee / Scope

- **Applicable a :** Applications UI Miyukini en Rust (Hub, clients natifs, outils).
- **Cibles :** Desktop (Windows, Linux, Mac), Web (WASM), Android (eframe), embedded (egui compatible, sans eframe selon cible).
- **Statut :** Reference officielle — stack UI adoptee pour le natif et le web.
- **Hors scope :** MiyukiniAdmin (UI web HTML/JS actuelle), composants HyperUI (Tailwind, contexte web/maquettes).

---

## 1. Licence et compatibilite Miyukini

| Composant | Licence | Compatibilite Miyukini |
|-----------|---------|-------------------------|
| **egui** | MIT **ou** Apache-2.0 (au choix) | Oui — aucune restriction commerciale, embedded ou attribution. |
| **eframe** | MIT **ou** Apache-2.0 (au choix) | Oui — idem. |

Vous pouvez utiliser egui/eframe sous **Apache-2.0** pour rester aligne avec une base Apache 2.0, ou sous **MIT** selon votre preference. Vendre Miyukini (licence commerciale societes/collectivites) avec egui/eframe a bord ne pose aucun probleme de licence.

---

## 2. Crates et dependances

### Cores

```toml
[dependencies]
egui = "0.33"
eframe = { version = "0.33", default-features = false, features = ["default_fonts"] }
```

- **egui** : bibliotheque UI en mode immediat (immediate mode) ; dessin des widgets, gestion des entrees, layout.
- **eframe** : cadre d’application multiplateforme ; point d’entree `run_native` (desktop) et `run_web` (WASM), boucle d’evenements, persistence optionnelle.

### Features utiles (eframe)

| Feature | Role |
|---------|------|
| `default_fonts` | Polices par defaut (recommandé). |
| `persistence` | Sauvegarde/restauration de l’etat (positions fenetres, etc.) via `App::save`. |
| `glow` | Backend OpenGL (glow) au lieu de wgpu (utile sur certains environnements). |
| `wgpu` | Backend wgpu (par defaut sur beaucoup de plateformes). |

Pour le **web (WASM)** : activer les features adequates (`default_fonts`, `persistence` si besoin) ; eframe gere `wasm-bindgen` et le canvas.

---

## 3. Modele : immediate mode et trait App

### Immediate mode

En **immediate mode**, l’UI n’est pas un arbre de widgets maintenu en memoire : a chaque frame, le code redessine l’interface. Les widgets sont declares a chaque appel a `update()` ; egui gere l’etat interne (clic, focus, etc.). Pas de callbacks ni de binding reactif — tout passe par `ctx` et l’etat de votre struct `App`.

### Trait eframe::App

Votre application implemente le trait **`eframe::App`** :

```rust
use eframe::egui;

struct MyApp {
    label: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hub Miyukini");
            ui.label(&self.label);
            if ui.button("Actualiser").clicked() {
                self.label = "Actualise.".to_string();
            }
        });
    }
}
```

- **`update(&mut self, ctx: &Context, frame: &mut Frame)`** : appele a chaque frame. C’est ici que vous construisez toute l’UI (panels, fenetres, boutons, etc.).
- **`ctx`** : contexte egui (requetes de repaint, stockage, input).
- **`frame`** : fenetre/application (titre, fermeture, etc.).

Methodes optionnelles utiles :

- **`save(&mut self, storage: &mut dyn Storage)`** : sauvegarde de l’etat a la fermeture (et selon `auto_save_interval`) si feature `persistence`.
- **`on_exit(&mut self, _gl: Option<&Context>)`** : nettoyage a la sortie.
- **`auto_save_interval(&self) -> Duration`** : intervalle de sauvegarde automatique.

---

## 4. Structure d’une application (desktop et web)

### Point d’entree commun

```rust
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("Miyukini Hub"),
        ..Default::default()
    };
    eframe::run_native(
        "Miyukini Hub",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
```

- **`eframe::run_native(...)`** : lance l’app **desktop** (Windows, Linux, Mac). Boucle d’evenements geree par eframe.
- **`cc`** : `CreationContext` (polices, theme, etc.) ; utile pour initialiser votre `MyApp`.

### Cible Web (WASM)

En conditional compilation, le point d’entree web utilise **`eframe::run_web`** :

```rust
#[cfg(target_arch = "wasm32")]
fn main() {
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
            )
            .await
            .expect("eframe WebRunner failed");
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    // run_native comme ci-dessus
}
```

L’element HTML doit contenir un `<canvas id="the_canvas_id">` (ou l’id choisi).

---

## 5. Panels et fenetres (egui)

L’UI se compose de **panels** et de **fenetres** (windows).

| Composant | Role |
|-----------|------|
| **`egui::CentralPanel::default()`** | Zone centrale (contenu principal). |
| **`egui::SidePanel::left/right(id)`** | Barre laterale (menu, navigation). |
| **`egui::TopBottomPanel::top/bottom(id)`** | Barre haute (titre, menu) ou basse (statut). |
| **`egui::Window::new(name)`** | Fenetre flottante (modale, palette, etc.). |
| **`egui::Area`** | Zone libre (overlay, tooltips, etc.). |

Exemple : layout type Hub (sidebar + contenu central).

```rust
fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::SidePanel::left("sidebar").show(ctx, |ui| {
        ui.heading("Services");
        ui.separator();
        if ui.button("Catalogue").clicked() {
            self.current_view = View::Catalogue;
        }
        if ui.button("Mes services").clicked() {
            self.current_view = View::MyServices;
        }
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        match self.current_view {
            View::Catalogue => self.ui_catalogue(ui),
            View::MyServices => self.ui_my_services(ui),
        }
    });
}
```

Chaque **`.show(ctx, |ui| { ... })`** donne un `ui` pour ajouter des widgets (boutons, labels, champs, tableaux, etc.).

---

## 6. Widgets et interactions

- **`ui.label(text)`**, **`ui.heading(text)`** : texte.
- **`ui.button(text)`** : bouton ; `.clicked()` dans la meme frame indique un clic.
- **`ui.add(egui::TextEdit::singleline(&mut self.input))`** : champ texte.
- **`ui.checkbox(&mut self.flag, "Label")`** : case a cocher.
- **`ui.selectable_value(&mut self.selected, value, "Label")`** : selection.
- **`ui.separator()`**, **`ui.horizontal(|ui| { ... })`**, **`ui.vertical(|ui| { ... })`** : layout.
- **`ui.collapsing("Titre", |ui| { ... })`** : section repliable.

Pour forcer un repaint (ex. timer, thread) : **`ctx.request_repaint()`**.

Documentation complete des widgets : [egui docs](https://docs.rs/egui/latest/egui/).

---

## 7. Cibles de deploiement

| Cible | Commande / methode | Remarque |
|-------|--------------------|----------|
| **Windows / Linux / Mac** | `cargo build --release` puis lancer le binaire | Backend wgpu ou glow selon plateforme. |
| **Web (WASM)** | `cargo build --target wasm32-unknown-unknown --release` puis `wasm-bindgen` (ou script fourni par eframe) | Integrer le JS + WASM dans une page HTML. |
| **Android** | eframe supporte Android ; projet Android + NDK + build Rust pour `aarch64-linux-android` | Voir doc eframe / egui pour template Android. |

Le meme code `impl App` est reutilise pour desktop et web ; seuls le point d’entree et les options (NativeOptions vs WebOptions) different.

---

## 8. Integration Miyukini (Hub de Services)

- Le **Hub** est un **Opérateur d’Interface** (Strate 7) : il expose le catalogue de Services (Registre d’Opérateurs) et permet de lancer des Services. L’UI Hub est implementee en **egui/eframe** (client natif ou web).
- **Pas de logique metier dans l’UI** : l’app egui/eframe envoie des **intentions** vers le COG (API locale ou distante) ; BondingBrother, StrongFather, Master Butler gouvernent. L’UI affiche les reponses et declenche les actions (ouvrir un Service = demande de Mandat, puis ouverture de l’interface du Service).
- **Licence** : egui/eframe sous MIT ou Apache-2.0 ; vendre Miyukini avec le Hub a bord (societes, collectivites, embedded) est autorise sans restriction supplementaire.

---

## 9. Bonnes pratiques

- **Persistence** : activer la feature `persistence` et implementer `App::save` pour restaurer taille/position des fenetres et etat minimal (vue courante, filtres).
- **Repaint** : si une tache longue ou un thread met a jour l’etat, appeler **`ctx.request_repaint()`** pour rafraichir l’UI.
- **Viewports** : pour plusieurs fenetres OS natives, utiliser **`ctx.show_viewport_deferred`** (egui 0.33+).
- **Themes** : `ctx.set_visuals(egui::Visuals::dark())` ou `.light()` ; personnalisation via `egui::Style`.
- **Performance** : eviter de recalculer des donnees lourdes a chaque frame ; mettre en cache dans votre struct `App` et invalider au besoin.

---

## 10. References

| Ressource | Lien |
|-----------|------|
| **egui** | [docs.rs/egui](https://docs.rs/egui/latest/egui/) |
| **eframe** | [docs.rs/eframe](https://docs.rs/eframe/latest/eframe/) |
| **Depots** | [github.com/emilk/egui](https://github.com/emilk/egui) |
| **Crates** | [crates.io/crates/egui](https://crates.io/crates/egui), [crates.io/crates/eframe](https://crates.io/crates/eframe) |
| **Guide officiel** | [https://www.egui.rs/](https://www.egui.rs/) |

---

**Date de creation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Reference officielle — stack UI Miyukini (egui / eframe)
