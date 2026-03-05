# MiyuClicker â€” Guide d'implÃ©mentation Zone citÃ© et bouton de construction

## Contexte

Ce document constitue le **guide d'implÃ©mentation** des Ã©volutions suivantes de l'Ã©cran Â« Ma citÃ©e Â» : **bouton de construction** sur chaque carte bÃ¢timent (deuxiÃ¨me ligne) et **zone de reprÃ©sentation de la citÃ©** (ciel/sol, sprites personnages, dÃ©placements alÃ©atoires). Il fournit les spÃ©cifications techniques, les structures de donnÃ©es, le layout UI et l'ordre de livraison recommandÃ©.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre :** Conditions et logique du bouton construction (vert/blanc, paiement, dÃ©marrage construction), barre de progression 200 px, activation du bloc maÃ§ons ; zone citÃ© (dimensions responsive, ciel/sol, sprites 3Ã—1 px, mouvement alÃ©atoire), MIP.
- **RÃ©fÃ©rence conceptuelle :** [MiyuClicker - Zone Cite et Bouton Construction](MiyukiniClicker%20-%20Zone%20Cite%20et%20Bouton%20Construction.md).
- **RÃ©fÃ©rence code existant :** `crates/miyuclicker` (state, idlesim, app).

---

## 1. Bouton de construction (cartes bÃ¢timent)

### 1.1 Conditions pour Â« level up Â»

Pour chaque type de bÃ¢timent (Maison, Caserne, Grenier, DÃ©pÃ´t, EntrepÃ´t), les conditions pour que le bouton soit **actif** sont :

- **Ressources suffisantes** :
  - `state.bois >= coÃ»t_bois`
  - `state.pierre >= coÃ»t_pierre`
  - `state.fer >= coÃ»t_fer`

Les coÃ»ts sont ceux dÃ©finis dans [MiyuClicker - Batiments Macons et Construction](MiyukiniClicker%20-%20Batiments%20Macons%20et%20Construction.md) (ex. Maison : 30b, 20p, 5f ; Caserne : 50b, 100p, 20f ; etc.).

**Pas de condition supplÃ©mentaire** (ex. Â« construction dÃ©jÃ  dÃ©marrÃ©e Â») : le bouton peut Ãªtre cliquÃ© dÃ¨s que les ressources sont suffisantes. Au clic, le coÃ»t est prÃ©levÃ© et la construction dÃ©marre (la progression peut Ãªtre Ã  0 et avancer avec les maÃ§ons).

### 1.2 Ã‰tat visuel du bouton

| Condition | Couleur du bouton | Clic |
|-----------|-------------------|------|
| Ressources suffisantes | **Vert** (ex. CSS `background-color: rgb(0, 180, 0)` ou thÃ¨me) | Actif : prÃ©lever coÃ»t et dÃ©marrer construction. |
| Ressources insuffisantes | **Blanc** (ex. CSS `background-color: white` ou fond neutre) | Inactif : pas d'effet. |

ImplÃ©mentation suggÃ©rÃ©e : bouton RSX `rsx! { button { style: "background-color: {couleur}", onclick: ..., "Construire" } }` avec `couleur` selon `can_afford_construction(state, building_type)` ; attribut `disabled` selon la mÃªme condition pour dÃ©sactiver le clic.

### 1.3 Action au clic (conditions remplies)

Lors du clic sur le bouton **vert** :

1. **PrÃ©lever le coÃ»t** : dÃ©crÃ©menter `state.bois`, `state.pierre`, `state.fer` selon le coÃ»t du bÃ¢timent (utiliser les constantes existantes dans `idlesim`, ex. `MAISON_COUT_BOIS`, etc.).
2. **DÃ©marrer la construction** : la construction est dÃ©jÃ  Â« en cours Â» dÃ¨s qu'il y a des pts Ã  accumuler. En pratique, ne pas rÃ©initialiser `construction_*` ; les maÃ§ons allouÃ©s accumulent des pts. Si le modÃ¨le actuel prÃ©lÃ¨ve le coÃ»t uniquement Ã  la **fin** (dans `try_complete_*`), il faut **dÃ©placer le prÃ©lÃ¨vement au clic** : au clic sur Â« Construire Â», prÃ©lever le coÃ»t immÃ©diatement et marquer que ce niveau est Â« en construction Â» (la barre avance avec les maÃ§ons jusqu'Ã  atteindre les pts requis, puis on incrÃ©mente le niveau sans reprendre de ressources).

**ModÃ¨le recommandÃ© :**

- **Option A (dÃ©marrage explicite)** : Au clic sur Â« Construire Â» (conditions OK), prÃ©lever le coÃ»t tout de suite et incrÃ©menter une Â« cible Â» de construction (ex. `construction_target_maison = true` ou simplement compter les pts dans `construction_maison`). Quand `construction_maison >= pts_required_maison(state.maisons)`, on incrÃ©mente `state.maisons` et on remet `construction_maison = 0`. Les ressources ont dÃ©jÃ  Ã©tÃ© payÃ©es au clic.
- **Option B (actuel)** : Aujourd'hui, le coÃ»t est prÃ©levÃ© dans `try_complete_*` quand la progression atteint le seuil. Pour coller au concept Â« payer au clic Â», il faut : au clic sur Â« Construire Â», prÃ©lever le coÃ»t et soit (1) mettre une marque Â« construction dÃ©marrÃ©e Â» et ne prÃ©lever qu'une fois au premier clic, soit (2) prÃ©lever au clic et dans `try_complete_*` ne plus prÃ©lever (juste incrÃ©menter le niveau et rÃ©initialiser la barre).

Le guide recommande **Option A** : au clic Â« Construire Â», prÃ©lever le coÃ»t immÃ©diatement ; la progression (pts) s'accumule avec les maÃ§ons ; Ã  100 % on incrÃ©mente le niveau et on remet la barre Ã  0 (sans reprendre de ressources).

### 1.4 Barre de progression de la construction

- **Largeur** : 200 px (fixe ou `width: min(200px, 100%)`).
- **Affichage** : `progress / required` en ratio (0..1) ou en texte Â« X / Y pts Â».
- **Widget** : div RSX avec barre de progression CSS, ex. `rsx! { div { style: "width: 200px; background: #eee;", div { style: "width: {pct}%; background: green; height: 16px;" } } }` ou texte `format!("{:.0}/{:.0}", progress, required)`.
- **Source des donnÃ©es** : `state.construction_maison`, `pts_required_maison_pub(state)`, etc. (dÃ©jÃ  prÃ©sents).

### 1.5 Bloc d'allocation des maÃ§ons

- **Activation** : Le bloc maÃ§ons (boutons +/âˆ’ par bÃ¢timent) est **toujours actif** dÃ¨s qu'il y a des maÃ§ons dans la pool ; pas besoin de Â« dÃ©bloquer Â» aprÃ¨s un clic sur Â« Construire Â». Si on souhaite une sÃ©mantique stricte Â« actif seulement quand construction dÃ©marrÃ©e Â», on peut afficher les +/âˆ’ uniquement quand `construction_* > 0` pour ce bÃ¢timent (optionnel).
- **Comportement** : InchangÃ© par rapport Ã  l'existant ; les maÃ§ons allouÃ©s ajoutent 1 pt/jour dans le tick.

### 1.6 Placement UI (carte bÃ¢timent)

- **DeuxiÃ¨me ligne** sur chaque carte : aprÃ¨s la ligne Â« Logo, niveau, nom, description, coÃ»t, barre, MaÃ§ons +/âˆ’ Â», ajouter une ligne avec le **bouton Â« Construire Â»** (vert ou blanc selon les conditions).
- Ordre suggÃ©rÃ© : (ligne 1) Logo + infos + barre + MaÃ§ons +/âˆ’ ; (ligne 2) Bouton Â« Construire Â».

---

## 2. Zone de reprÃ©sentation de la citÃ©

### 2.1 Emplacement dans l'Ã©cran Â« Ma citÃ©e Â»

- **Entre** : le **header** (div flexbox avec ressources, horloge, vitesses) **et** la **liste des bÃ¢timents + boutons de clic**.
- **Dans** : le conteneur principal (div RSX avec layout CSS flexbox/grid) ; ordre des enfants : header â†’ **zone citÃ©** â†’ contenu (boutons clic + cartes bÃ¢timent).

### 2.2 Dimensions (responsive)

| Dimension | RÃ¨gle |
|-----------|--------|
| **Largeur** | 100 % de la largeur disponible (toute la largeur d'affichage du contenu). |
| **Hauteur** | 20 % de la hauteur disponible (ou de la fenÃªtre), avec **minimum 200 px**. |

ImplÃ©mentation Dioxus : utiliser un div RSX avec style CSS `width: 100%; height: 20vh; min-height: 200px;` pour la zone citÃ©. Dessiner le ciel et le sol comme des div enfants avec les proportions dÃ©finies, ou utiliser un Ã©lÃ©ment SVG inline pour le rendu graphique (ciel + sol + sprites).

### 2.3 DÃ©coupage ciel / sol

- **Ciel** : rectangle **haut**, hauteur = 60 % de la hauteur de la zone.
- **Sol** : rectangle **bas**, hauteur = 40 % de la hauteur de la zone.

Couleurs suggÃ©rÃ©es (CSS) :

- Ciel : `background-color: rgb(135, 206, 235)` (bleu clair) ou thÃ¨me.
- Sol : `background-color: rgb(210, 180, 140)` (beige / marron clair) ou thÃ¨me.

Dessin : div RSX avec style CSS pour le ciel et le sol, ou Ã©lÃ©ments SVG `rect` avec `fill` pour chaque zone (ex. `rsx! { rect { fill: "rgb(135,206,235)", width: "100%", height: "60%" } }`).

### 2.4 Sprites personnages (3Ã—1 px)

- **Taille** : 1 px de large, 3 px de haut (rectangle 1Ã—3).
- **Structure** :
  - **Pixel du haut** : blanc (CSS `white` / SVG `fill="white"`) â€” tÃªte.
  - **Pixels du bas** (2 pixels) : couleur du corps selon le type.

| Type   | Corps (2 pixels) |
|--------|-------------------|
| Gens   | Vert `rgb(0, 128, 0)` ou Ã©quivalent. |
| Soldat | Rouge `rgb(180, 0, 0)` ou Ã©quivalent. |
| MaÃ§on  | Marron foncÃ© `rgb(101, 67, 33)` ou Ã©quivalent. |

Dessin : pour chaque sprite, un Ã©lÃ©ment SVG `rect` de 1Ã—3 px Ã  la position `(x, y)` (en coordonnÃ©es zone sol), puis 3 `rect` SVG de 1Ã—1 empilÃ©s avec les couleurs tÃªte + corps + corps (ou un div positionnÃ© en absolu avec les couleurs CSS appropriÃ©es).

### 2.5 Nombre de sprites affichÃ©s

- **Source** : `state.gens`, `state.soldats`, `state.macons`.
- **Option simple** : afficher **un sprite par unitÃ©** (gens, soldats, maÃ§ons) jusqu'Ã  un plafond raisonnable (ex. 50 ou 100 par type) pour Ã©viter surcharge ; au-delÃ , on peut afficher un nombre plafonnÃ© (ex. 50) pour garder la lisibilitÃ©.
- **Position** : chaque sprite a une position `(x, y)` dans la **zone sol** uniquement. `y` doit Ãªtre dans l'intervalle vertical du sol ; `x` dans la largeur de la zone.

### 2.6 DÃ©placement alÃ©atoire

- **Zone** : uniquement la zone **sol** (40 % du bas). Les coordonnÃ©es des sprites doivent rester dans le rect du sol.
- **Algorithme** : Ã  chaque frame (ou Ã  intervalle de temps via `use_future` ou `use_coroutine`), pour chaque sprite :
  - Mettre Ã  jour une **vitesse** ou **direction** (ex. `dx`, `dy`) de faÃ§on alÃ©atoire (ex. petit delta alÃ©atoire, ou nouvelle direction toutes les N secondes).
  - Mettre Ã  jour la position : `x += dx * dt` ; `y += dy * dt` (ou Ã©quivalent).
  - **Contraintes** : clamp `x` entre `sol.left()` et `sol.right() - 1` (pour 1 px de large) ; clamp `y` entre `sol.top()` et `sol.bottom() - 3` (pour 3 px de haut). Optionnel : rebond sur les bords (inverser `dx` ou `dy`) au lieu de clamp.
- **DÃ©terminisme** : pour la reproductibilitÃ© des sauvegardes, soit (1) ne pas sauvegarder les positions (rÃ©initialiser Ã  chaque chargement), soit (2) sauvegarder une graine RNG et les positions pour reprendre Ã  l'identique. Le guide recommande de **ne pas sauvegarder** les positions (re-init alÃ©atoire au chargement) pour simplifier.

### 2.7 Structures de donnÃ©es suggÃ©rÃ©es

- **Positions des sprites** : soit dans l'Ã©tat du jeu (si sauvegardÃ©es), soit dans l'Ã©tat UI de l'app (recalculÃ©es Ã  chaque frame). Recommandation : **Ã©tat UI** (ex. `Vec<(f32, f32, PersonType)>` pour positions + type), mis Ã  jour chaque frame Ã  partir de `state.gens`, `state.soldats`, `state.macons` (nombre de sprites) et d'un RNG ou d'un timer pour le dÃ©placement. Utiliser un signal Dioxus (`use_signal`) pour stocker et mettre Ã  jour les positions.
- **PersonType** : enum `Gens | Soldat | Macon` pour la couleur du corps.

Exemple (pseudo-Rust) :

```text
// Dans l'app (Ã©tat UI, pas sauvegardÃ©)
struct CitizenSprite {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    person_type: PersonType, // Gens | Soldat | Macon
}
// Ou une liste par type ; mise Ã  jour via use_signal avec positions dans la zone sol.
```

---

## 3. Ordre d'implÃ©mentation recommandÃ©

1. **Bouton construction**
   - Ajouter une fonction `can_afford_construction(state, building_type)` (ou par type : `can_afford_maison(state)` etc.) qui compare ressources aux coÃ»ts.
   - Sur chaque carte bÃ¢timent, ajouter en deuxiÃ¨me ligne un bouton RSX Â« Construire Â» : couleur vert (CSS) si `can_afford_*`, blanc sinon ; attribut `disabled` selon la mÃªme condition.
   - Au clic (conditions OK) : prÃ©lever le coÃ»t (nouvelle fonction `start_construction_maison(state)` etc. qui fait `state.bois -= ...` ; `state.pierre -= ...` ; `state.fer -= ...`) et Ã©ventuellement marquer Â« construction dÃ©marrÃ©e Â» si on utilise un flag. Adapter `try_complete_*` dans idlesim pour **ne plus prÃ©lever** le coÃ»t (dÃ©jÃ  fait au clic) : quand `construction_* >= required`, incrÃ©menter le niveau et mettre `construction_* = 0`.
   - Barre de progression : s'assurer qu'elle fait 200 px de large (ou min(200px, 100%)) et affiche bien progression/required.

2. **Zone citÃ© (layout + ciel/sol)**
   - Dans l'Ã©cran Ma citÃ©e, aprÃ¨s le header, allouer une div RSX de hauteur = 20vh (min 200 px) et largeur 100 %.
   - Dessiner le ciel (60 % haut) et le sol (40 % bas) avec les couleurs dÃ©finies (div CSS ou SVG inline).

3. **Sprites et mouvement**
   - CrÃ©er la liste des sprites (ou le compteur par type) Ã  partir de `state.gens`, `state.soldats`, `state.macons` (avec plafond si besoin).
   - Initialiser ou mettre Ã  jour les positions dans la zone sol ; dessiner chaque sprite en 3 pixels via Ã©lÃ©ments SVG ou div positionnÃ©s (1 blanc, 2 corps selon le type).
   - ImplÃ©menter le dÃ©placement alÃ©atoire (dx/dy, mise Ã  jour via `use_future` ou `use_coroutine`, clamp dans le sol).

4. **MIP**
   - Balises `@id`, `@do`, `@role`, `@layer`, `@human` sur les fonctions et blocs concernÃ©s (bouton construction, zone citÃ©, sprites, mouvement).

---

## 4. RÃ©sumÃ© des spÃ©cifications techniques

| Ã‰lÃ©ment | Spec |
|--------|------|
| Bouton construction | 2e ligne carte ; vert si ressources OK, blanc sinon ; clic = prÃ©lever coÃ»t + dÃ©marrer construction. |
| CoÃ»t | PrÃ©levÃ© au clic (adapter idlesim pour ne pas reprendre dans `try_complete_*`). |
| Barre construction | 200 px large, progression / required. |
| Zone citÃ© | Entre header et liste ; 20 % hauteur (min 200 px), 100 % largeur. |
| Ciel | 60 % hauteur, bleu clair. |
| Sol | 40 % hauteur, beige / marron clair. |
| Sprite | 1Ã—3 px ; tÃªte blanche ; corps vert / rouge / marron (gens / soldat / maÃ§on). |
| Mouvement | AlÃ©atoire dans zone sol ; positions en Ã©tat UI (signal Dioxus) ; clamp ou rebond aux bords. |

---

## 5. RÃ©fÃ©rences

- [MiyuClicker - Zone Cite et Bouton Construction](MiyukiniClicker%20-%20Zone%20Cite%20et%20Bouton%20Construction.md) â€” Concepts.
- [MiyuClicker - Batiments Macons et Construction](MiyukiniClicker%20-%20Batiments%20Macons%20et%20Construction.md) â€” CoÃ»ts et pts de construction.
- Code : `crates/miyuclicker/src/app.rs` (UI), `idlesim.rs` (coÃ»ts, try_complete_*).

