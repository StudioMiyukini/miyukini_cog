# MiyuClicker â€” SystÃ¨me de bonheur (moral)

## Contexte

Ce document dÃ©crit le **systÃ¨me de bonheur** (moral) de la population dans MiyuClicker : dÃ©finition, rÃ¨gles dynamiques, effets sur la population et Game Over. La mÃ©trique est affichÃ©e en **pourcentage (0â€“100)** dans lâ€™UI, juste aprÃ¨s les points de recherche.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre :** Bonheur = moral (0â€“1 en interne, 0â€“100 % en affichage) ; Ã©volution selon nourriture ; Game Over aprÃ¨s 7 jours Ã  0 nourriture ; affichage dynamique en barre.
- **RÃ©fÃ©rence code :** `state.moral`, `idlesim.tick` (Ã©volution moral), `app.ui_bar` (Bonheur XX %).

---

## 1. DÃ©finition

| Terme | Signification |
|-------|----------------|
| **Bonheur** | MÃ©trique affichÃ©e Ã  lâ€™utilisateur (pourcentage 0â€“100). |
| **Moral** | Variable interne (0â€“1) ; bonheur = moral Ã— 100. |

Le bonheur est une **mÃ©trique dynamique** : il Ã©volue chaque tick en fonction des rÃ©serves de nourriture par rapport Ã  la population.

---

## 2. RÃ¨gles dâ€™Ã©volution (par jour simulÃ©)

Consommation : **1 nourriture / personne / jour**. Les seuils ci-dessous sâ€™appliquent **aprÃ¨s** production et consommation du tick (nourriture = stock courant).

| Condition nourriture | Effet sur le moral (par jour jeu) |
|----------------------|------------------------------------|
| Nourriture â‰¥ population | Pas de baisse ; le moral peut remonter (ex. +2 %/j jusquâ€™Ã  100 %). |
| Nourriture < population mais > population/2 | **âˆ’1 %/jour** (moral baisse). |
| Nourriture â‰¤ population/2 | **âˆ’5 %/jour** (moral baisse forte). |
| Nourriture â‰¤ 0 pendant plus de 7 jours | **Game Over** (mort du seigneur). |

- Moral plafonnÃ© Ã  **0** et **1** (0 % et 100 %).
- **Jours Ã  0 nourriture** : compteur cumulÃ© tant que nourriture â‰¤ 0 ; remis Ã  0 dÃ¨s que nourriture > 0.

---

## 3. Game Over

- **Condition :** Nourriture Ã  0 (ou sous un seuil minimal) pendant **strictement plus de 7 jours** (en temps simulÃ©).
- **Effet :** Le seigneur (joueur) meurt â†’ **Game Over**.
- **ImplÃ©mentation :** Champ `game_over: bool` dans lâ€™Ã©tat ; champ `jours_nourriture_zero: f64` pour cumuler les jours Ã  0. Quand `jours_nourriture_zero > 7`, on pose `game_over = true` et on affiche lâ€™Ã©cran Game Over (pas de tick, pas de jeu).

---

## 4. Effets du bonheur sur la population (rÃ©fÃ©rence)

- **Bonheur > 50 %** : la population peut augmenter (clic Village + Ã©ventuelle natalitÃ©).
- **Bonheur â‰¤ 50 %** : la population ne peut plus augmenter (blocage croissance).
- **Bonheur < 10 %** : la population diminue jusquâ€™Ã  remonter au-dessus de 10 %.
- **Bonheur > 90 %** : la population augmente plus vite (bonus).

*(DÃ©tail dâ€™implÃ©mentation : voir spÃ©cifications population / tick de rÃ©serve.)*

---

## 5. Affichage UI

- **Emplacement :** PremiÃ¨re ligne de la barre, **juste aprÃ¨s** les points de **Recherche**.
- **Format :** `Bonheur: XX %` (entier 0â€“100), mÃ©trique mise Ã  jour Ã  chaque frame (dynamique).
- **Source :** `state.moral * 100.0` arrondi ou tronquÃ© pour lâ€™affichage.

---

## 6. Sauvegarde

Les champs suivants sont persistÃ©s : `moral`, `fecondite`, `game_over`, `jours_nourriture_zero` (dans cap_moral ou meta selon structure). Chargement : valeurs par dÃ©faut si absent (rÃ©trocompatibilitÃ©).

---

## 7. RÃ©fÃ©rences

- [MiyuClicker - MVP Ecrans et Mecaniques](MiyukiniClicker%20-%20MVP%20Ecrans%20et%20Mecaniques.md) â€” Barre et ressources.
- [MiyuClicker - Guide Implementation MVP](MiyukiniClicker%20-%20Guide%20Implementation%20MVP.md) â€” ModÃ¨le dâ€™Ã©tat et tick.
- Code : `crates/miyuclicker/src/state.rs` (moral), `idlesim.rs` (tick moral), `app.rs` (ui_bar Bonheur).

