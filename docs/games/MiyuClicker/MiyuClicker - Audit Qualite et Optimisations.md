# MiyuClicker — Audit qualité et optimisations

**Contexte :** Audit du code du jeu MiyuClicker (crate `miyuclicker`) pour une métrique de qualité et des pistes d’optimisation.

**Portée :** `app.rs`, `state.rs`, `idlesim.rs`, `save.rs`, `carte.rs`, `combat.rs`, `lib.rs`, `main.rs`.

**Statut :** Document de référence (non contractuel).

---

## 1. Métrique de qualité globale

| Critère | Score (0–10) | Poids | Note pondérée |
|---------|---------------|-------|----------------|
| **Architecture / séparation des rôles** | 8 | 20 % | 1,6 |
| **Cohérence / lisibilité** | 8 | 15 % | 1,2 |
| **Documentation MIP** | 9 | 10 % | 0,9 |
| **Gestion d’erreurs** | 7 | 15 % | 1,05 |
| **Performance / allocations** | 6 | 15 % | 0,9 |
| **DRY / maintenabilité** | 6 | 15 % | 0,9 |
| **Testabilité** | 5 | 10 % | 0,5 |
| **Sécurité / robustesse** | 7 | 10 % | 0,7 |
| **Total** | — | 100 % | **7,75 / 10** |

**Verdict :** Qualité **bonne** pour un MVP : structure claire, MIP bien appliqué, logique métier lisible. Les gains les plus faciles portent sur les allocations, la duplication de code et la testabilité.

---

## 2. Points forts

- **Séparation nette des modules** : `state` (données), `idlesim` (simu), `save` (I/O), `carte` (carte + déplacements), `combat` (résolution combat), `app` (UI). Pas de mélange logique métier / I/O dans l’UI.
- **MIP (MSCM) systématique** : Blocs `@id`, `@do`, `@role` (et `@human` où utile) sur les types et fonctions publics/privés importants. Bonne base pour index et gouvernance.
- **État centralisé** : `GameState` est la source de vérité ; sauvegarde et UI s’appuient dessus sans dupliquer la logique.
- **Rétrocompatibilité sauvegarde** : `#[serde(default)]`, alias `habitations` → `maisons`, évitent les crashs sur anciennes sauvegardes.
- **Pas de `unwrap()` dangereux en UI** : usage de `match`, `if let`, `ok_or_else` pour les lectures de state et slots.
- **Limitation du repaint** : `request_repaint` uniquement en jeu et non en pause ; log dev espacé (toutes les 120 frames).

---

## 3. Points faibles et risques

| Problème | Fichier / zone | Impact |
|----------|----------------|--------|
| **Clone de `slot_metadata`** à chaque frame sur l’écran Slots | `app.rs` L.252 `for meta in &self.slot_metadata.clone()` | Allocations inutiles, léger coût CPU. |
| **Recherche cité par `id` en O(n)** répétée (routes, déplacements, clic) | `app.rs` (carte monde), `carte.rs` | Carte avec beaucoup de cités : coût linéaire par frame. |
| **`routes_map` reconstruit** à chaque `model_update` avec clones de `String` | `carte.rs` L.68–84 | Allocations et travail répétés chaque tick. |
| **Duplication overlay Game Over** (Ma cité + Carte du monde) | `app.rs` | Risque de divergence de texte/comportement. |
| **Boutons de clic** (labels, tailles) dupliqués en dur | `app.rs` ui_ma_citee | Évolution coûteuse (nouveau clic = plusieurs lignes). |
| **`try_complete_*` et `pts_required_*_pub`** très similaires | `idlesim.rs` | Beaucoup de code répété, erreur dans un = risque d’oublier les autres. |
| **Hasard combat** basé sur `SystemTime::now()` | `combat.rs` | Non déterministe : pas de tests reproductibles, pas de replay. |
| **`resolve_arrival`** : `unwrap_or(0)` si cité introuvable | `carte.rs` L.109 | Cité supprimée ou incohérence → mise à jour de la mauvaise cité (index 0). |

---

## 4. Optimisations proposées

### 4.1 Performance / allocations

- **Slots : ne pas cloner la liste des slots**  
  Remplacer `for meta in &self.slot_metadata.clone()` par `for meta in &self.slot_metadata` (itération par référence). Coût actuel : 1 allocation `Vec` + clones de `SlotMetadata` à chaque frame sur l’écran Slots.

- **Carte : index cités par id**  
  Une fois par frame (ou quand `state.cites` change), construire `HashMap<&str, usize>` (id → index). Remplacer les `state.cites.iter().find(|c| c.id == x)` par un accès par index. Réduit les recherches de O(n) à O(1) pour routes, déplacements et clic.

- **`model_update` : cache du `routes_map`**  
  Stocker `routes_map` (ou une structure équivalente) dans `GameState` (ou dans un cache côté carte), et ne la recalculer que si `state.routes` a changé. Évite des clones de `String` à chaque tick.

- **Éviter des `format!` inutiles**  
  Là où c’est possible (ex. barre de ressources), réutiliser des buffers ou des `Cow<str>` pour les valeurs qui changent peu (ex. labels numériques). Gain marginal mais utile si la barre est redessinée très souvent.

### 4.2 DRY / maintenabilité

- **Overlay Game Over unique**  
  Extraire une fonction `fn ui_game_over_overlay(ui, &mut screen)` (ou équivalent) et l’appeler depuis `ui_ma_citee` et `ui_carte_monde`. Un seul texte et un seul comportement (ex. retour aux slots).

- **Boutons de clic depuis des données**  
  Définir une table (ex. `const` ou petit tableau) : `(ClickTarget, label, size)`. Boucler sur cette table pour générer les boutons. Ajouter un nouveau clic = une ligne de données.

- **Construction : boucle sur les bâtiments**  
  Regrouper les paramètres par type (Maison, Caserne, Grenier, …) dans une structure ou un enum, et avoir une seule fonction `try_complete(state, building_type)` (et une seule `pts_required(state, building_type)`). Réduit la duplication entre `try_complete_maison`, `try_complete_caserne`, etc.

### 4.3 Robustesse

- **`resolve_arrival`**  
  Ne pas utiliser `unwrap_or(0)` pour `to_idx`. Si la cité cible n’existe plus : soit `position()` retourne `None` et on skip l’arrivée (et on log ou on nettoie le déplacement), soit on utilise `if let Some(to_idx) = ...` et on ne touche pas à `state.cites` si `None`.

- **Validation sauvegarde**  
  Après `serde_json::from_str`, vérifier des invariants (ex. `maisons >= 0`, `slot_id in 1..=3`) et rejeter ou corriger au chargement. Limite les états incohérents après migration de version.

### 4.4 Testabilité

- **Combats déterministes**  
  Remplacer l’usage direct de `SystemTime::now()` par un trait (ex. `Rng` ou `fn() -> u64`) injecté dans `resolve`. En jeu : implémentation temps réel ; en test : RNG fixe (seed). Permet des tests unitaires et des replays.

- **Tests unitaires ciblés**  
  Ajouter des tests pour : `apply_click` (chaque `ClickTarget`), `tick` (moral, caps, construction), `save_to_game_state` / `game_state_to_save` (round-trip), `resolve` (attaquant 0, défenseur 0, cas gagnant/perdant avec seed).

---

## 5. Synthèse des gains estimés

| Action | Effort | Gain |
|--------|--------|------|
| Supprimer `slot_metadata.clone()` dans ui_slots | Faible | Moins d’allocations, code plus clair. |
| Overlay Game Over unique | Faible | Moins de duplication, un seul endroit à maintenir. |
| Index cités par id (carte monde) | Moyen | Meilleure perf sur cartes avec beaucoup de cités. |
| Cache `routes_map` dans model_update | Moyen | Moins d’allocations et de travail par tick. |
| Table + boucle pour boutons de clic | Moyen | Évolution plus simple, moins d’erreurs. |
| Boucle / généricité pour try_complete_* | Élevé | Beaucoup moins de code, évolution bâtiments centralisée. |
| RNG injectable (combat) + tests | Moyen | Combats testables et reproductibles. |
| Corriger resolve_arrival (to_idx) | Faible | Comportement sûr même si données incohérentes. |

---

## 6. Références

- [MiyuClicker - Batiments Macons et Construction](./MiyuClicker%20-%20Batiments%20Macons%20et%20Construction.md)
- [MiyuClicker - Systeme Bonheur](./MiyuClicker%20-%20Systeme%20Bonheur.md)
- MIP v1 (MSCM Index Protocol) — balisage @id, @do, @role

---

**Date :** 2026-02-01  
**Version :** 1.0
