# MiyuClicker — Architecture technique actuelle

## Contexte

Ce document décrit l'**architecture technique actuelle** du crate `miyuclicker` : modules, responsabilités et flux d'appel. Il sert de carte pour naviguer dans le code et pour aligner les évolutions.

**Référence code :** `crates/miyuclicker/src/`.

## Portée / Scope

- **Périmètre :** Structure des modules, qui appelle qui, où se trouve la logique métier (état, simulation, carte, combat, sauvegarde).
- **Hors périmètre :** Formules d'équilibrage, détails UI (widgets), format JSON de sauvegarde (voir Guide MVP et save.rs).

---

## 1. Modules et rôles

| Module | Fichier | Rôle en une phrase |
|--------|---------|--------------------|
| **app** | `app.rs` | Point d'entrée Dioxus : écrans (Loading, Landing, Slots, Ma citée, Carte du monde), dispatch UI, appels aux autres modules, état applicatif (screen, config, slot_metadata, game_speed, dev_console). |
| **state** | `state.rs` | Source de vérité : `GameState`, `Allocation`, `AllocationMacons`, `Cite`, `Route`, `Deplacement`, `ClickTarget`, `BuildingType`, `Proprietaire`. Pas de logique métier, uniquement données et helpers (cap_gens_from_maisons, cap_soldats, format_clock, new_game). |
| **idlesim** | `idlesim.rs` | Simulation idle : `tick` (production, consommation, moral, fécondité, Game Over), `apply_click`, `apply_allocation`, `apply_allocation_macons`, construction (start_construction_*, try_complete_*, pts_required_*), `convert_pop_to_macon`. Constantes (coûts clic, production/jour, pts construction). |
| **save** | `save.rs` | Persistance : `slot_list`, `slot_read`, `slot_write`. Conversion `GameState` ↔ `SaveState` (JSON). Fichiers par slot dans `data_dir`. |
| **carte** | `carte.rs` | Carte stratégique : `move_troops` (enregistrement déplacement, déduction troupes source), `model_update` (avancement progress, résolution arrivées), `resolve_arrival` (combat ou renfort), `generate_carte_mvp` (génération graphe cités/routes). Utilise `combat::resolve` à l'arrivée sur cité adverse. |
| **combat** | `combat.rs` | Résolution combat : `resolve(attaquant, defenseur, stats_att, stats_def, seed)` → `CombatResult` (vainqueur, troupes restantes). Hasard + rapport de force. |
| **ui_assets** | `ui_assets.rs` | Chemins des assets UI (pack modernuserinterface-win 32x32). Pas de chargement ni cache de textures dans ce crate. |
| **lib** | `lib.rs` | Ré-exports publics (MiyuClickerApp, Screen, GameState, etc.). |
| **main** | `main.rs` | Point d'entrée binaire : `dioxus::launch` avec le composant `App`. |

---

## 2. Flux d'appel (qui appelle qui)

### 2.1 Vue d'ensemble

```
Dioxus (main)
    → composant App (fn App() avec #[component])
        → selon screen : ui_loading | ui_landing | ui_slots | ui_ma_citee | ui_carte_monde
        → ui_config_menu (si config_menu_open)
        → ui_dev_console (si dev_console_open)
```

- **app** détient : `game_state: Option<GameState>`, `slot_metadata`, `screen`, `data_dir`, `game_speed_index`, etc.
- **app** appelle :
  - **save** : `slot_list`, `slot_read`, `slot_write` (écrans Slots, config, Nouvelle partie / Charger).
  - **idlesim** : `tick`, `apply_click`, `apply_allocation`, `apply_allocation_macons`, `start_construction_*`, `convert_pop_to_macon` (écran Ma citée).
  - **carte** : `model_update`, `move_troops`, `generate_carte_mvp` (écran Ma citée et Carte du monde).

- **carte** appelle **combat** : `combat::resolve` dans `resolve_arrival` quand la cible est une cité adverse.

- **state** n'appelle personne ; il est lu/écrit par app, idlesim, save, carte.

### 2.2 Détail par écran

| Écran | Appels depuis app |
|-------|-------------------|
| **Loading** | Aucun (progression locale) ; à la fin : `slot_list(&data_dir)`. |
| **Landing** | Aucun (bouton Jouer → Slots). |
| **Slots** | `slot_list` pour afficher les slots ; `GameState::new_game(slot_id)` pour nouvelle partie ; `slot_read` pour charger ; `slot_write` pour écraser. |
| **Ma citée** | À chaque frame (si en jeu et pas pause) : `tick(state, delta_s)` ; clics : `apply_click`, `apply_allocation`, `apply_allocation_macons`, `start_construction_*`, `convert_pop_to_macon` ; `model_update(state, delta_s)` pour avancer les déplacements. |
| **Carte du monde** | `model_update(state, delta_s)` ; `move_troops` (après validation envoi troupes) ; `generate_carte_mvp` si besoin (carte vide). |

### 2.3 Flux de données (état)

- **GameState** est la seule source de vérité partagée. Il est :
  - **lu/écrit** par `idlesim` (tick, apply_*, construction) ;
  - **lu/écrit** par `carte` (move_troops, model_update, resolve_arrival) ;
  - **lu** par `app` pour l'affichage et pour passer des références aux modules ;
  - **sérialisé/désérialisé** par `save` via `SaveState`.

- Aucun cache métier en dehors de `GameState` (sauf `route_duree_by_pair` dans `GameState` pour éviter de recalculer les durées de route à chaque tick).

---

## 3. Points d'attention (alignement Audit)

- **app** : grosse taille (~850 lignes) ; overlay Game Over déjà extrait en `ui_game_over_overlay` ; boutons de clic centralisés dans `CLIC_BUTTONS`.
- **idlesim** : duplication entre `try_complete_*` et `pts_required_*_pub` par type de bâtiment ; possibilité de factoriser par `BuildingType`.
- **carte** : recherche cité par `id` en O(n) (iter.find) ; index `id → index` possible pour grosses cartes.
- **combat** : seed passé en paramètre ; en prod l'appel vient de `carte::resolve_arrival` avec `SystemTime::now()` → non déterministe (tests/replay difficiles).

---

## 4. Schéma texte (résumé)

```text
main.rs
  └─ dioxus::launch(App)

app.rs (composant App avec #[component])
  ├─ rsx! { ... } → ui_loading | ui_landing | ui_slots | ui_ma_citee | ui_carte_monde
  ├─ game_state: Signal<Option<GameState>>  ← state.rs
  ├─ save::slot_list | slot_read | slot_write   ← save.rs
  ├─ idlesim::tick | apply_click | apply_allocation | apply_allocation_macons | start_construction_* | convert_pop_to_macon   ← idlesim.rs
  └─ carte::model_update | move_troops | generate_carte_mvp   ← carte.rs

carte.rs
  └─ combat::resolve   ← combat.rs (à l'arrivée sur cité adverse)

state.rs
  └─ (aucun appel ; uniquement types et GameState)
```

---

**Dernière mise à jour :** 2026-02-11  
**Statut :** Document de référence — architecture technique actuelle
