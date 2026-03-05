# MiyuClicker â€” Audit qualitÃ© et optimisations

**Contexte :** Audit du code du jeu MiyuClicker (crate `miyuclicker`) pour une mÃ©trique de qualitÃ© et des pistes dâ€™optimisation.

**PortÃ©e :** `app.rs`, `state.rs`, `idlesim.rs`, `save.rs`, `carte.rs`, `combat.rs`, `lib.rs`, `main.rs`.

**Statut :** Document de rÃ©fÃ©rence (non contractuel).

---

## 1. MÃ©trique de qualitÃ© globale

| CritÃ¨re | Score (0â€“10) | Poids | Note pondÃ©rÃ©e |
|---------|---------------|-------|----------------|
| **Architecture / sÃ©paration des rÃ´les** | 8 | 20 % | 1,6 |
| **CohÃ©rence / lisibilitÃ©** | 8 | 15 % | 1,2 |
| **Documentation MIP** | 9 | 10 % | 0,9 |
| **Gestion dâ€™erreurs** | 7 | 15 % | 1,05 |
| **Performance / allocations** | 6 | 15 % | 0,9 |
| **DRY / maintenabilitÃ©** | 6 | 15 % | 0,9 |
| **TestabilitÃ©** | 5 | 10 % | 0,5 |
| **SÃ©curitÃ© / robustesse** | 7 | 10 % | 0,7 |
| **Total** | â€” | 100 % | **7,75 / 10** |

**Verdict :** QualitÃ© **bonne** pour un MVP : structure claire, MIP bien appliquÃ©, logique mÃ©tier lisible. Les gains les plus faciles portent sur les allocations, la duplication de code et la testabilitÃ©.

---

## 2. Points forts

- **SÃ©paration nette des modules** : `state` (donnÃ©es), `idlesim` (simu), `save` (I/O), `carte` (carte + dÃ©placements), `combat` (rÃ©solution combat), `app` (UI). Pas de mÃ©lange logique mÃ©tier / I/O dans lâ€™UI.
- **MIP (MSCM) systÃ©matique** : Blocs `@id`, `@do`, `@role` (et `@human` oÃ¹ utile) sur les types et fonctions publics/privÃ©s importants. Bonne base pour index et gouvernance.
- **Ã‰tat centralisÃ©** : `GameState` est la source de vÃ©ritÃ© ; sauvegarde et UI sâ€™appuient dessus sans dupliquer la logique.
- **RÃ©trocompatibilitÃ© sauvegarde** : `#[serde(default)]`, alias `habitations` â†’ `maisons`, Ã©vitent les crashs sur anciennes sauvegardes.
- **Pas de `unwrap()` dangereux en UI** : usage de `match`, `if let`, `ok_or_else` pour les lectures de state et slots.
- **Limitation du repaint** : `request_repaint` uniquement en jeu et non en pause ; log dev espacÃ© (toutes les 120 frames).

---

## 3. Points faibles et risques

| ProblÃ¨me | Fichier / zone | Impact |
|----------|----------------|--------|
| **Clone de `slot_metadata`** Ã  chaque frame sur lâ€™Ã©cran Slots | `app.rs` L.252 `for meta in &self.slot_metadata.clone()` | Allocations inutiles, lÃ©ger coÃ»t CPU. |
| **Recherche citÃ© par `id` en O(n)** rÃ©pÃ©tÃ©e (routes, dÃ©placements, clic) | `app.rs` (carte monde), `carte.rs` | Carte avec beaucoup de citÃ©s : coÃ»t linÃ©aire par frame. |
| **`routes_map` reconstruit** Ã  chaque `model_update` avec clones de `String` | `carte.rs` L.68â€“84 | Allocations et travail rÃ©pÃ©tÃ©s chaque tick. |
| **Duplication overlay Game Over** (Ma citÃ© + Carte du monde) | `app.rs` | Risque de divergence de texte/comportement. |
| **Boutons de clic** (labels, tailles) dupliquÃ©s en dur | `app.rs` ui_ma_citee | Ã‰volution coÃ»teuse (nouveau clic = plusieurs lignes). |
| **`try_complete_*` et `pts_required_*_pub`** trÃ¨s similaires | `idlesim.rs` | Beaucoup de code rÃ©pÃ©tÃ©, erreur dans un = risque dâ€™oublier les autres. |
| **Hasard combat** basÃ© sur `SystemTime::now()` | `combat.rs` | Non dÃ©terministe : pas de tests reproductibles, pas de replay. |
| **`resolve_arrival`** : `unwrap_or(0)` si citÃ© introuvable | `carte.rs` L.109 | CitÃ© supprimÃ©e ou incohÃ©rence â†’ mise Ã  jour de la mauvaise citÃ© (index 0). |

---

## 4. Optimisations proposÃ©es

### 4.1 Performance / allocations

- **Slots : ne pas cloner la liste des slots**  
  Remplacer `for meta in &self.slot_metadata.clone()` par `for meta in &self.slot_metadata` (itÃ©ration par rÃ©fÃ©rence). CoÃ»t actuel : 1 allocation `Vec` + clones de `SlotMetadata` Ã  chaque frame sur lâ€™Ã©cran Slots.

- **Carte : index citÃ©s par id**  
  Une fois par frame (ou quand `state.cites` change), construire `HashMap<&str, usize>` (id â†’ index). Remplacer les `state.cites.iter().find(|c| c.id == x)` par un accÃ¨s par index. RÃ©duit les recherches de O(n) Ã  O(1) pour routes, dÃ©placements et clic.

- **`model_update` : cache du `routes_map`**  
  Stocker `routes_map` (ou une structure Ã©quivalente) dans `GameState` (ou dans un cache cÃ´tÃ© carte), et ne la recalculer que si `state.routes` a changÃ©. Ã‰vite des clones de `String` Ã  chaque tick.

- **Ã‰viter des `format!` inutiles**  
  LÃ  oÃ¹ câ€™est possible (ex. barre de ressources), rÃ©utiliser des buffers ou des `Cow<str>` pour les valeurs qui changent peu (ex. labels numÃ©riques). Gain marginal mais utile si la barre est redessinÃ©e trÃ¨s souvent.

### 4.2 DRY / maintenabilitÃ©

- **Overlay Game Over unique**  
  Extraire une fonction `fn ui_game_over_overlay(ui, &mut screen)` (ou Ã©quivalent) et lâ€™appeler depuis `ui_ma_citee` et `ui_carte_monde`. Un seul texte et un seul comportement (ex. retour aux slots).

- **Boutons de clic depuis des donnÃ©es**  
  DÃ©finir une table (ex. `const` ou petit tableau) : `(ClickTarget, label, size)`. Boucler sur cette table pour gÃ©nÃ©rer les boutons. Ajouter un nouveau clic = une ligne de donnÃ©es.

- **Construction : boucle sur les bÃ¢timents**  
  Regrouper les paramÃ¨tres par type (Maison, Caserne, Grenier, â€¦) dans une structure ou un enum, et avoir une seule fonction `try_complete(state, building_type)` (et une seule `pts_required(state, building_type)`). RÃ©duit la duplication entre `try_complete_maison`, `try_complete_caserne`, etc.

### 4.3 Robustesse

- **`resolve_arrival`**  
  Ne pas utiliser `unwrap_or(0)` pour `to_idx`. Si la citÃ© cible nâ€™existe plus : soit `position()` retourne `None` et on skip lâ€™arrivÃ©e (et on log ou on nettoie le dÃ©placement), soit on utilise `if let Some(to_idx) = ...` et on ne touche pas Ã  `state.cites` si `None`.

- **Validation sauvegarde**  
  AprÃ¨s `serde_json::from_str`, vÃ©rifier des invariants (ex. `maisons >= 0`, `slot_id in 1..=3`) et rejeter ou corriger au chargement. Limite les Ã©tats incohÃ©rents aprÃ¨s migration de version.

### 4.4 TestabilitÃ©

- **Combats dÃ©terministes**  
  Remplacer lâ€™usage direct de `SystemTime::now()` par un trait (ex. `Rng` ou `fn() -> u64`) injectÃ© dans `resolve`. En jeu : implÃ©mentation temps rÃ©el ; en test : RNG fixe (seed). Permet des tests unitaires et des replays.

- **Tests unitaires ciblÃ©s**  
  Ajouter des tests pour : `apply_click` (chaque `ClickTarget`), `tick` (moral, caps, construction), `save_to_game_state` / `game_state_to_save` (round-trip), `resolve` (attaquant 0, dÃ©fenseur 0, cas gagnant/perdant avec seed).

---

## 5. SynthÃ¨se des gains estimÃ©s

| Action | Effort | Gain |
|--------|--------|------|
| Supprimer `slot_metadata.clone()` dans ui_slots | Faible | Moins dâ€™allocations, code plus clair. |
| Overlay Game Over unique | Faible | Moins de duplication, un seul endroit Ã  maintenir. |
| Index citÃ©s par id (carte monde) | Moyen | Meilleure perf sur cartes avec beaucoup de citÃ©s. |
| Cache `routes_map` dans model_update | Moyen | Moins dâ€™allocations et de travail par tick. |
| Table + boucle pour boutons de clic | Moyen | Ã‰volution plus simple, moins dâ€™erreurs. |
| Boucle / gÃ©nÃ©ricitÃ© pour try_complete_* | Ã‰levÃ© | Beaucoup moins de code, Ã©volution bÃ¢timents centralisÃ©e. |
| RNG injectable (combat) + tests | Moyen | Combats testables et reproductibles. |
| Corriger resolve_arrival (to_idx) | Faible | Comportement sÃ»r mÃªme si donnÃ©es incohÃ©rentes. |

---

## 6. RÃ©fÃ©rences

- [MiyuClicker - Batiments Macons et Construction](MiyukiniClicker%20-%20Batiments%20Macons%20et%20Construction.md)
- [MiyuClicker - Systeme Bonheur](MiyukiniClicker%20-%20Systeme%20Bonheur.md)
- MIP v1 (MSCM Index Protocol) â€” balisage @id, @do, @role

---

**Date :** 2026-02-01  
**Version :** 1.0

