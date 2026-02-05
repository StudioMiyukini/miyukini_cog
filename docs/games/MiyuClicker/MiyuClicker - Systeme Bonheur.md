# MiyuClicker — Système de bonheur (moral)

## Contexte

Ce document décrit le **système de bonheur** (moral) de la population dans MiyuClicker : définition, règles dynamiques, effets sur la population et Game Over. La métrique est affichée en **pourcentage (0–100)** dans l’UI, juste après les points de recherche.

## Portée / Scope

- **Périmètre :** Bonheur = moral (0–1 en interne, 0–100 % en affichage) ; évolution selon nourriture ; Game Over après 7 jours à 0 nourriture ; affichage dynamique en barre.
- **Référence code :** `state.moral`, `idlesim.tick` (évolution moral), `app.ui_bar` (Bonheur XX %).

---

## 1. Définition

| Terme | Signification |
|-------|----------------|
| **Bonheur** | Métrique affichée à l’utilisateur (pourcentage 0–100). |
| **Moral** | Variable interne (0–1) ; bonheur = moral × 100. |

Le bonheur est une **métrique dynamique** : il évolue chaque tick en fonction des réserves de nourriture par rapport à la population.

---

## 2. Règles d’évolution (par jour simulé)

Consommation : **1 nourriture / personne / jour**. Les seuils ci-dessous s’appliquent **après** production et consommation du tick (nourriture = stock courant).

| Condition nourriture | Effet sur le moral (par jour jeu) |
|----------------------|------------------------------------|
| Nourriture ≥ population | Pas de baisse ; le moral peut remonter (ex. +2 %/j jusqu’à 100 %). |
| Nourriture < population mais > population/2 | **−1 %/jour** (moral baisse). |
| Nourriture ≤ population/2 | **−5 %/jour** (moral baisse forte). |
| Nourriture ≤ 0 pendant plus de 7 jours | **Game Over** (mort du seigneur). |

- Moral plafonné à **0** et **1** (0 % et 100 %).
- **Jours à 0 nourriture** : compteur cumulé tant que nourriture ≤ 0 ; remis à 0 dès que nourriture > 0.

---

## 3. Game Over

- **Condition :** Nourriture à 0 (ou sous un seuil minimal) pendant **strictement plus de 7 jours** (en temps simulé).
- **Effet :** Le seigneur (joueur) meurt → **Game Over**.
- **Implémentation :** Champ `game_over: bool` dans l’état ; champ `jours_nourriture_zero: f64` pour cumuler les jours à 0. Quand `jours_nourriture_zero > 7`, on pose `game_over = true` et on affiche l’écran Game Over (pas de tick, pas de jeu).

---

## 4. Effets du bonheur sur la population (référence)

- **Bonheur > 50 %** : la population peut augmenter (clic Village + éventuelle natalité).
- **Bonheur ≤ 50 %** : la population ne peut plus augmenter (blocage croissance).
- **Bonheur < 10 %** : la population diminue jusqu’à remonter au-dessus de 10 %.
- **Bonheur > 90 %** : la population augmente plus vite (bonus).

*(Détail d’implémentation : voir spécifications population / tick de réserve.)*

---

## 5. Affichage UI

- **Emplacement :** Première ligne de la barre, **juste après** les points de **Recherche**.
- **Format :** `Bonheur: XX %` (entier 0–100), métrique mise à jour à chaque frame (dynamique).
- **Source :** `state.moral * 100.0` arrondi ou tronqué pour l’affichage.

---

## 6. Sauvegarde

Les champs suivants sont persistés : `moral`, `fecondite`, `game_over`, `jours_nourriture_zero` (dans cap_moral ou meta selon structure). Chargement : valeurs par défaut si absent (rétrocompatibilité).

---

## 7. Références

- [MiyuClicker - MVP Ecrans et Mecaniques](MiyuClicker%20-%20MVP%20Ecrans%20et%20Mecaniques.md) — Barre et ressources.
- [MiyuClicker - Guide Implementation MVP](MiyuClicker%20-%20Guide%20Implementation%20MVP.md) — Modèle d’état et tick.
- Code : `crates/miyuclicker/src/state.rs` (moral), `idlesim.rs` (tick moral), `app.rs` (ui_bar Bonheur).
