# MiyuClicker — Guide d'implémentation Zone cité et bouton de construction

## Contexte

Ce document constitue le **guide d'implémentation** des évolutions suivantes de l'écran « Ma citée » : **bouton de construction** sur chaque carte bâtiment (deuxième ligne) et **zone de représentation de la cité** (ciel/sol, sprites personnages, déplacements aléatoires). Il fournit les spécifications techniques, les structures de données, le layout UI et l'ordre de livraison recommandé.

## Portée / Scope

- **Périmètre :** Conditions et logique du bouton construction (vert/blanc, paiement, démarrage construction), barre de progression 200 px, activation du bloc maçons ; zone cité (dimensions responsive, ciel/sol, sprites 3×1 px, mouvement aléatoire), MIP.
- **Référence conceptuelle :** [MiyuClicker - Zone Cite et Bouton Construction](MiyuClicker%20-%20Zone%20Cite%20et%20Bouton%20Construction.md).
- **Référence code existant :** `crates/miyuclicker` (state, idlesim, app).

---

## 1. Bouton de construction (cartes bâtiment)

### 1.1 Conditions pour « level up »

Pour chaque type de bâtiment (Maison, Caserne, Grenier, Dépôt, Entrepôt), les conditions pour que le bouton soit **actif** sont :

- **Ressources suffisantes** :
  - `state.bois >= coût_bois`
  - `state.pierre >= coût_pierre`
  - `state.fer >= coût_fer`

Les coûts sont ceux définis dans [MiyuClicker - Batiments Macons et Construction](MiyuClicker%20-%20Batiments%20Macons%20et%20Construction.md) (ex. Maison : 30b, 20p, 5f ; Caserne : 50b, 100p, 20f ; etc.).

**Pas de condition supplémentaire** (ex. « construction déjà démarrée ») : le bouton peut être cliqué dès que les ressources sont suffisantes. Au clic, le coût est prélevé et la construction démarre (la progression peut être à 0 et avancer avec les maçons).

### 1.2 État visuel du bouton

| Condition | Couleur du bouton | Clic |
|-----------|-------------------|------|
| Ressources suffisantes | **Vert** (ex. CSS `background-color: rgb(0, 180, 0)` ou thème) | Actif : prélever coût et démarrer construction. |
| Ressources insuffisantes | **Blanc** (ex. CSS `background-color: white` ou fond neutre) | Inactif : pas d'effet. |

Implémentation suggérée : bouton RSX `rsx! { button { style: "background-color: {couleur}", onclick: ..., "Construire" } }` avec `couleur` selon `can_afford_construction(state, building_type)` ; attribut `disabled` selon la même condition pour désactiver le clic.

### 1.3 Action au clic (conditions remplies)

Lors du clic sur le bouton **vert** :

1. **Prélever le coût** : décrémenter `state.bois`, `state.pierre`, `state.fer` selon le coût du bâtiment (utiliser les constantes existantes dans `idlesim`, ex. `MAISON_COUT_BOIS`, etc.).
2. **Démarrer la construction** : la construction est déjà « en cours » dès qu'il y a des pts à accumuler. En pratique, ne pas réinitialiser `construction_*` ; les maçons alloués accumulent des pts. Si le modèle actuel prélève le coût uniquement à la **fin** (dans `try_complete_*`), il faut **déplacer le prélèvement au clic** : au clic sur « Construire », prélever le coût immédiatement et marquer que ce niveau est « en construction » (la barre avance avec les maçons jusqu'à atteindre les pts requis, puis on incrémente le niveau sans reprendre de ressources).

**Modèle recommandé :**

- **Option A (démarrage explicite)** : Au clic sur « Construire » (conditions OK), prélever le coût tout de suite et incrémenter une « cible » de construction (ex. `construction_target_maison = true` ou simplement compter les pts dans `construction_maison`). Quand `construction_maison >= pts_required_maison(state.maisons)`, on incrémente `state.maisons` et on remet `construction_maison = 0`. Les ressources ont déjà été payées au clic.
- **Option B (actuel)** : Aujourd'hui, le coût est prélevé dans `try_complete_*` quand la progression atteint le seuil. Pour coller au concept « payer au clic », il faut : au clic sur « Construire », prélever le coût et soit (1) mettre une marque « construction démarrée » et ne prélever qu'une fois au premier clic, soit (2) prélever au clic et dans `try_complete_*` ne plus prélever (juste incrémenter le niveau et réinitialiser la barre).

Le guide recommande **Option A** : au clic « Construire », prélever le coût immédiatement ; la progression (pts) s'accumule avec les maçons ; à 100 % on incrémente le niveau et on remet la barre à 0 (sans reprendre de ressources).

### 1.4 Barre de progression de la construction

- **Largeur** : 200 px (fixe ou `width: min(200px, 100%)`).
- **Affichage** : `progress / required` en ratio (0..1) ou en texte « X / Y pts ».
- **Widget** : div RSX avec barre de progression CSS, ex. `rsx! { div { style: "width: 200px; background: #eee;", div { style: "width: {pct}%; background: green; height: 16px;" } } }` ou texte `format!("{:.0}/{:.0}", progress, required)`.
- **Source des données** : `state.construction_maison`, `pts_required_maison_pub(state)`, etc. (déjà présents).

### 1.5 Bloc d'allocation des maçons

- **Activation** : Le bloc maçons (boutons +/− par bâtiment) est **toujours actif** dès qu'il y a des maçons dans la pool ; pas besoin de « débloquer » après un clic sur « Construire ». Si on souhaite une sémantique stricte « actif seulement quand construction démarrée », on peut afficher les +/− uniquement quand `construction_* > 0` pour ce bâtiment (optionnel).
- **Comportement** : Inchangé par rapport à l'existant ; les maçons alloués ajoutent 1 pt/jour dans le tick.

### 1.6 Placement UI (carte bâtiment)

- **Deuxième ligne** sur chaque carte : après la ligne « Logo, niveau, nom, description, coût, barre, Maçons +/− », ajouter une ligne avec le **bouton « Construire »** (vert ou blanc selon les conditions).
- Ordre suggéré : (ligne 1) Logo + infos + barre + Maçons +/− ; (ligne 2) Bouton « Construire ».

---

## 2. Zone de représentation de la cité

### 2.1 Emplacement dans l'écran « Ma citée »

- **Entre** : le **header** (div flexbox avec ressources, horloge, vitesses) **et** la **liste des bâtiments + boutons de clic**.
- **Dans** : le conteneur principal (div RSX avec layout CSS flexbox/grid) ; ordre des enfants : header → **zone cité** → contenu (boutons clic + cartes bâtiment).

### 2.2 Dimensions (responsive)

| Dimension | Règle |
|-----------|--------|
| **Largeur** | 100 % de la largeur disponible (toute la largeur d'affichage du contenu). |
| **Hauteur** | 20 % de la hauteur disponible (ou de la fenêtre), avec **minimum 200 px**. |

Implémentation Dioxus : utiliser un div RSX avec style CSS `width: 100%; height: 20vh; min-height: 200px;` pour la zone cité. Dessiner le ciel et le sol comme des div enfants avec les proportions définies, ou utiliser un élément SVG inline pour le rendu graphique (ciel + sol + sprites).

### 2.3 Découpage ciel / sol

- **Ciel** : rectangle **haut**, hauteur = 60 % de la hauteur de la zone.
- **Sol** : rectangle **bas**, hauteur = 40 % de la hauteur de la zone.

Couleurs suggérées (CSS) :

- Ciel : `background-color: rgb(135, 206, 235)` (bleu clair) ou thème.
- Sol : `background-color: rgb(210, 180, 140)` (beige / marron clair) ou thème.

Dessin : div RSX avec style CSS pour le ciel et le sol, ou éléments SVG `rect` avec `fill` pour chaque zone (ex. `rsx! { rect { fill: "rgb(135,206,235)", width: "100%", height: "60%" } }`).

### 2.4 Sprites personnages (3×1 px)

- **Taille** : 1 px de large, 3 px de haut (rectangle 1×3).
- **Structure** :
  - **Pixel du haut** : blanc (CSS `white` / SVG `fill="white"`) — tête.
  - **Pixels du bas** (2 pixels) : couleur du corps selon le type.

| Type   | Corps (2 pixels) |
|--------|-------------------|
| Gens   | Vert `rgb(0, 128, 0)` ou équivalent. |
| Soldat | Rouge `rgb(180, 0, 0)` ou équivalent. |
| Maçon  | Marron foncé `rgb(101, 67, 33)` ou équivalent. |

Dessin : pour chaque sprite, un élément SVG `rect` de 1×3 px à la position `(x, y)` (en coordonnées zone sol), puis 3 `rect` SVG de 1×1 empilés avec les couleurs tête + corps + corps (ou un div positionné en absolu avec les couleurs CSS appropriées).

### 2.5 Nombre de sprites affichés

- **Source** : `state.gens`, `state.soldats`, `state.macons`.
- **Option simple** : afficher **un sprite par unité** (gens, soldats, maçons) jusqu'à un plafond raisonnable (ex. 50 ou 100 par type) pour éviter surcharge ; au-delà, on peut afficher un nombre plafonné (ex. 50) pour garder la lisibilité.
- **Position** : chaque sprite a une position `(x, y)` dans la **zone sol** uniquement. `y` doit être dans l'intervalle vertical du sol ; `x` dans la largeur de la zone.

### 2.6 Déplacement aléatoire

- **Zone** : uniquement la zone **sol** (40 % du bas). Les coordonnées des sprites doivent rester dans le rect du sol.
- **Algorithme** : à chaque frame (ou à intervalle de temps via `use_future` ou `use_coroutine`), pour chaque sprite :
  - Mettre à jour une **vitesse** ou **direction** (ex. `dx`, `dy`) de façon aléatoire (ex. petit delta aléatoire, ou nouvelle direction toutes les N secondes).
  - Mettre à jour la position : `x += dx * dt` ; `y += dy * dt` (ou équivalent).
  - **Contraintes** : clamp `x` entre `sol.left()` et `sol.right() - 1` (pour 1 px de large) ; clamp `y` entre `sol.top()` et `sol.bottom() - 3` (pour 3 px de haut). Optionnel : rebond sur les bords (inverser `dx` ou `dy`) au lieu de clamp.
- **Déterminisme** : pour la reproductibilité des sauvegardes, soit (1) ne pas sauvegarder les positions (réinitialiser à chaque chargement), soit (2) sauvegarder une graine RNG et les positions pour reprendre à l'identique. Le guide recommande de **ne pas sauvegarder** les positions (re-init aléatoire au chargement) pour simplifier.

### 2.7 Structures de données suggérées

- **Positions des sprites** : soit dans l'état du jeu (si sauvegardées), soit dans l'état UI de l'app (recalculées à chaque frame). Recommandation : **état UI** (ex. `Vec<(f32, f32, PersonType)>` pour positions + type), mis à jour chaque frame à partir de `state.gens`, `state.soldats`, `state.macons` (nombre de sprites) et d'un RNG ou d'un timer pour le déplacement. Utiliser un signal Dioxus (`use_signal`) pour stocker et mettre à jour les positions.
- **PersonType** : enum `Gens | Soldat | Macon` pour la couleur du corps.

Exemple (pseudo-Rust) :

```text
// Dans l'app (état UI, pas sauvegardé)
struct CitizenSprite {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    person_type: PersonType, // Gens | Soldat | Macon
}
// Ou une liste par type ; mise à jour via use_signal avec positions dans la zone sol.
```

---

## 3. Ordre d'implémentation recommandé

1. **Bouton construction**
   - Ajouter une fonction `can_afford_construction(state, building_type)` (ou par type : `can_afford_maison(state)` etc.) qui compare ressources aux coûts.
   - Sur chaque carte bâtiment, ajouter en deuxième ligne un bouton RSX « Construire » : couleur vert (CSS) si `can_afford_*`, blanc sinon ; attribut `disabled` selon la même condition.
   - Au clic (conditions OK) : prélever le coût (nouvelle fonction `start_construction_maison(state)` etc. qui fait `state.bois -= ...` ; `state.pierre -= ...` ; `state.fer -= ...`) et éventuellement marquer « construction démarrée » si on utilise un flag. Adapter `try_complete_*` dans idlesim pour **ne plus prélever** le coût (déjà fait au clic) : quand `construction_* >= required`, incrémenter le niveau et mettre `construction_* = 0`.
   - Barre de progression : s'assurer qu'elle fait 200 px de large (ou min(200px, 100%)) et affiche bien progression/required.

2. **Zone cité (layout + ciel/sol)**
   - Dans l'écran Ma citée, après le header, allouer une div RSX de hauteur = 20vh (min 200 px) et largeur 100 %.
   - Dessiner le ciel (60 % haut) et le sol (40 % bas) avec les couleurs définies (div CSS ou SVG inline).

3. **Sprites et mouvement**
   - Créer la liste des sprites (ou le compteur par type) à partir de `state.gens`, `state.soldats`, `state.macons` (avec plafond si besoin).
   - Initialiser ou mettre à jour les positions dans la zone sol ; dessiner chaque sprite en 3 pixels via éléments SVG ou div positionnés (1 blanc, 2 corps selon le type).
   - Implémenter le déplacement aléatoire (dx/dy, mise à jour via `use_future` ou `use_coroutine`, clamp dans le sol).

4. **MIP**
   - Balises `@id`, `@do`, `@role`, `@layer`, `@human` sur les fonctions et blocs concernés (bouton construction, zone cité, sprites, mouvement).

---

## 4. Résumé des spécifications techniques

| Élément | Spec |
|--------|------|
| Bouton construction | 2e ligne carte ; vert si ressources OK, blanc sinon ; clic = prélever coût + démarrer construction. |
| Coût | Prélevé au clic (adapter idlesim pour ne pas reprendre dans `try_complete_*`). |
| Barre construction | 200 px large, progression / required. |
| Zone cité | Entre header et liste ; 20 % hauteur (min 200 px), 100 % largeur. |
| Ciel | 60 % hauteur, bleu clair. |
| Sol | 40 % hauteur, beige / marron clair. |
| Sprite | 1×3 px ; tête blanche ; corps vert / rouge / marron (gens / soldat / maçon). |
| Mouvement | Aléatoire dans zone sol ; positions en état UI (signal Dioxus) ; clamp ou rebond aux bords. |

---

## 5. Références

- [MiyuClicker - Zone Cite et Bouton Construction](MiyuClicker%20-%20Zone%20Cite%20et%20Bouton%20Construction.md) — Concepts.
- [MiyuClicker - Batiments Macons et Construction](MiyuClicker%20-%20Batiments%20Macons%20et%20Construction.md) — Coûts et pts de construction.
- Code : `crates/miyuclicker/src/app.rs` (UI), `idlesim.rs` (coûts, try_complete_*).
