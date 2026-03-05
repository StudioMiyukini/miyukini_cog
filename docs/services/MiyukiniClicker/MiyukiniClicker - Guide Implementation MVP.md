# MiyuClicker â€” Guide dâ€™implÃ©mentation MVP

## Contexte

Ce document constitue le **guide dâ€™implÃ©mentation** du MVP MiyuClicker. Il fournit le **modÃ¨le dâ€™Ã©tat**, les **spÃ©cifications dÃ©taillÃ©es par Ã©cran**, les **APIs des Toolkits et OpÃ©rateurs**, le **format de sauvegarde** et les **phases dâ€™implÃ©mentation** pour permettre le dÃ©veloppement du jeu de faÃ§on structurÃ©e et alignÃ©e avec lâ€™architecture Miyukini.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre :** ModÃ¨le de donnÃ©es (Ã©tat partie), specs UI par Ã©cran (widgets, layout, transitions), signatures des Tools/OpÃ©rateurs, schÃ©ma de sauvegarde, ordre de livraison par phases.
- **Hors pÃ©rimÃ¨tre :** Formules dâ€™Ã©quilibrage numÃ©riques, assets graphiques, implÃ©mentation effective du code (crates, fichiers) â€” ce guide est une **spÃ©cification pour lâ€™implÃ©mentation**.

---

## 1. ModÃ¨le dâ€™Ã©tat (Ã©tat de la partie)

Lâ€™Ã©tat du jeu est la **source de vÃ©ritÃ©** partagÃ©e entre MiyuClickerSim, MiyuClickerSave, MiyuClickerCarte et MiyuClickerUI. Il doit Ãªtre **sÃ©rialisable** pour la sauvegarde.

### 1.1 Ressources (stocks)

| Champ | Type | Description | Valeur initiale (nouvelle partie) |
|-------|------|-------------|-----------------------------------|
| `or` | `i64` ou `f64` | Stock dâ€™or | 0 |
| `gens` | `i64` | Population civile (disponible + affectÃ©e) | 10 |
| `soldats` | `i64` | Nombre de soldats | 0 |
| `recherche` | `i64` ou `f64` | Points de recherche | 0 |
| `nourriture` | `f64` | Stock nourriture | 100 |
| `bois` | `f64` | Stock bois | 50 |
| `pierre` | `f64` | Stock pierre | 50 |
| `fer` | `f64` | Stock fer | 0 |
| `outils` | `f64` | Stock outils | 20 |
| `armes` | `i64` | Stock armes | 0 |

**RÃ¨gle :** Les stocks ne peuvent pas Ãªtre nÃ©gatifs (clamp Ã  0 aprÃ¨s chaque tick / action).

### 1.2 Cap et moral

| Champ | Type | Description | Valeur initiale |
|-------|------|-------------|-----------------|
| `cap_gens` | `i64` | Plafond de population = f(habitations) | 20 |
| `habitations` | `i64` | Nombre dâ€™habitations construites | 1 |
| `moral` | `f64` | Moral (0..1) ; impacte fÃ©conditÃ© | 1.0 |
| `fecondite` | `f64` | FÃ©conditÃ© (0..1) ; impacte croissance gens | 1.0 |

**RÃ¨gle mÃ©tier :** Si `nourriture` < consommation sur un tick, `moral` et `fecondite` baissent ; sinon ils remontent (plafonnÃ©s Ã  1.0).

### 1.3 Affectation des gens (allocation)

Structure : **map** lieu â†’ nombre de gens affectÃ©s.

| Lieu (clÃ©) | Ressource produite par tick |
|------------|-----------------------------|
| `champs` | Nourriture |
| `ateliers` | Outils |
| `scierie` | Bois |
| `carriere` | Pierre |
| `mine` | Fer |
| `recherche` | Points de recherche |

**Contrainte :** Somme des gens affectÃ©s â‰¤ `gens`. Les gens Â« disponibles Â» = `gens` âˆ’ somme(affectations).

**Type proposÃ© (Rust) :** `HashMap<String, i64>` ou struct `Allocation { champs, ateliers, scierie, carriere, mine, recherche }`.

### 1.4 Carte (MVP)

| Ã‰lÃ©ment | Type | Description |
|--------|------|-------------|
| **CitÃ©s** | Liste de `Cite` | Chaque citÃ© : `id`, `nom`, `x`, `y` (coords Ã©cran ou logiques), `proprietaire` (joueur / adverse), `troupes`, `niveau` (optionnel). |
| **Routes** | Liste de `Route` | Paire `(cite_id_a, cite_id_b)`, `duree_deplacement` (secondes simulÃ©es). |
| **DÃ©placements en cours** | Liste de `Deplacement` | `from_cite_id`, `to_cite_id`, `troupes`, `progress` (0..1), `attaquant` (bool). |

**PropriÃ©taire :** `Joueur` | `Adverse(id)` pour distinguer les citÃ©s adverses.

### 1.5 MÃ©tadonnÃ©es partie

| Champ | Type | Description |
|-------|------|-------------|
| `temps_simule_s` | `f64` | Temps simulÃ© Ã©coulÃ© (secondes) |
| `slot_id` | `u8` | Slot de sauvegarde (1, 2, 3) |
| `version_sauvegarde` | `String` | Ex. `"1.0"` pour compatibilitÃ© future |

### 1.6 SynthÃ¨se structure Ã©tat (pseudo-Rust)

```text
struct GameState {
    // Ressources
    or: i64,
    gens: i64,
    soldats: i64,
    recherche: i64,
    nourriture: f64,
    bois: f64,
    pierre: f64,
    fer: f64,
    outils: f64,
    armes: i64,
    // Cap et moral
    cap_gens: i64,
    habitations: i64,
    moral: f64,
    fecondite: f64,
    // Affectation
    allocation: Allocation,
    // Carte
    cites: Vec<Cite>,
    routes: Vec<Route>,
    deplacements: Vec<Deplacement>,
    // Meta
    temps_simule_s: f64,
    slot_id: u8,
    version_sauvegarde: String,
}
```

---

## 2. SpÃ©cifications dÃ©taillÃ©es par Ã©cran

### 2.1 Ã‰cran Loading

| Ã‰lÃ©ment | SpÃ©cification |
|--------|----------------|
| **Layout** | Plein Ã©cran (ou zone centrale). Fond : couleur ou texture. |
| **Widgets** | Titre / logo (centrÃ©) ; barre de progression ou spinner ; texte optionnel Â« Chargementâ€¦ Â» ou pourcentage. |
| **DonnÃ©es** | Progression 0..1 (assets chargÃ©s, init terminÃ©e). |
| **Transition** | DÃ¨s que `loading_done == true` â†’ Ã©cran **Landing**. Pas de bouton utilisateur. |
| **Erreur** | Si Ã©chec chargement : message + bouton Â« RÃ©essayer Â» ou Â« Quitter Â». |

**Ã‰tats UI :** `Loading { progress }` â†’ `Landing`.

### 2.2 Ã‰cran Landing

| Ã‰lÃ©ment | SpÃ©cification |
|--------|----------------|
| **Layout** | Titre / logo en haut ou centre ; bouton **[Jouer]** bien visible (centre ou sous le titre) ; icÃ´ne **roue de configuration** en haut Ã  droite. |
| **Widgets** | `Label` titre ; `Button` Â« Jouer Â» ; `Button` icÃ´ne engrenage â†’ ouvre menu dÃ©roulant. |
| **Menu config (dÃ©roulant)** | AncrÃ© Ã  droite de la roue. EntrÃ©es : **Sauvegarder** (dÃ©sactivÃ© ou masquÃ© au Landing), **Changer la rÃ©solution**, **Langue**, **Ã€ propos**. Clic extÃ©rieur ou aprÃ¨s action â†’ fermeture. |
| **Transition** | Clic [Jouer] â†’ Ã©cran **Slots**. |

**Ã‰tats UI :** `Landing` ; sous-Ã©tat optionnel `ConfigMenuOpen`.

### 2.3 Ã‰cran SÃ©lection des slots

| Ã‰lÃ©ment | SpÃ©cification |
|--------|----------------|
| **Layout** | Titre Â« Choisir une sauvegarde Â» ; 3 zones (slot 1, 2, 3) ; chaque zone : rÃ©sumÃ© (date/heure si existant, ou Â« Vide Â»), bouton **[Nouvelle partie]** ou **[Charger]** ; bouton **[Retour]** (vers Landing). |
| **DonnÃ©es** | Appel `save.slot_list()` â†’ mÃ©tadonnÃ©es des 3 slots (date, rÃ©sumÃ©, occupÃ©). |
| **Actions** | **[Nouvelle partie]** sur slot N : init Ã©tat initial, `slot_id = N`, transition â†’ **Ma citÃ©e**. **[Charger]** sur slot N : `save.slot_read(N)` â†’ Ã©tat, transition â†’ **Ma citÃ©e**. **[Retour]** â†’ **Landing**. |
| **Ã‰crasement** | Si Â« Nouvelle partie Â» sur slot occupÃ© : afficher confirmation Â« Ã‰craser la sauvegarde ? Â» ; si oui â†’ Ã©craser puis lancer. |

**Ã‰tats UI :** `Slots` ; sous-Ã©tat optionnel `ConfirmOverwrite(slot_id)`.

### 2.4 Ã‰cran Ma citÃ©e (fenÃªtre principale â€” gestion)

| Zone | Widgets / Layout |
|------|------------------|
| **Barre haut (ligne 1)** | IcÃ´ne + valeur pour **Or**, **Gens**, **Soldats**, **Recherche** (ordre fixe) ; puis **Ma citÃ©e** (actif / surlignÃ©), **Carte du monde** (lien), **âš™** (roue config). |
| **Barre haut (ligne 2)** | IcÃ´ne + valeur pour **Nourriture**, **Bois**, **Pierre**, **Fer**, **Outils**, **Armes** (ordre fixe). |
| **Partie gauche** | 4 **gros boutons** : **Champs**, **Ateliers**, **ChÃ¢teau**, **Village**. Clic â†’ intention vers Sim (gain clic). |
| **Partie droite** | **Liste dÃ©roulante** (ou panneau extensible) : pour chaque lieu (Champs, Ateliers, Scierie, CarriÃ¨re, Mine, Recherche), afficher lieu + ressource gÃ©nÃ©rÃ©e + **nombre de gens affectÃ©s** (slider ou spinbox). Contrainte : total affectÃ© â‰¤ gens. |
| **Menu config** | Identique au Landing ; **Sauvegarder** actif â†’ appelle `save.slot_write(slot_id, state)`. |

**Transitions :** Clic Â« Carte du monde Â» â†’ Ã©cran **Carte du monde**. Clic roue â†’ menu config. Tick (timer via `use_future` ou `use_coroutine`) â†’ appel Sim `tick(state, delta)` ; mise Ã  jour Ã©tat via signaux rÃ©actifs Dioxus (`use_signal`).

**DonnÃ©es affichÃ©es :** Lecture de `GameState` (ressources, allocation) ; mise Ã  jour aprÃ¨s chaque tick et aprÃ¨s chaque clic (intention traitÃ©e par Sim).

### 2.5 Ã‰cran Carte du monde

| Ã‰lÃ©ment | SpÃ©cification |
|--------|----------------|
| **Layout** | MÃªme barre haut (ligne 1 + 2) que Ma citÃ©e ; **Carte du monde** surlignÃ©, **Ma citÃ©e** lien. Zone carte : Ã©lÃ©ments SVG inline ou canvas Dioxus, ou zone cliquable. |
| **Rendu carte** | NÅ“uds = citÃ©s (cercles ou sprites) ; arÃªtes = routes (lignes) ; dÃ©placements en cours (ligne en pointillÃ©s ou segment progressif). Couleur / icÃ´ne selon propriÃ©taire (joueur / adverse). |
| **Interaction** | Clic sur une citÃ© â†’ afficher panneau latÃ©ral ou popup : nom, propriÃ©taire, troupes. Si citÃ© adverse et citÃ© joueur sÃ©lectionnÃ©e : bouton Â« Envoyer X soldats Â» (X saisi ou slider) â†’ intention `carte.move_troops(from, to, count)`. |
| **Tick** | Avancement des dÃ©placements (`progress += delta / duree`) ; Ã  arrivÃ©e (`progress >= 1`) â†’ `carte.resolve_arrival` â†’ Combat si citÃ© adverse â†’ mise Ã  jour citÃ©s et troupes. |

**DonnÃ©es** : `GameState.cites`, `routes`, `deplacements` ; lecture par UI, mise Ã  jour par OpÃ©rateur MiyuClickerCarte.

---

## 3. APIs Toolkits et OpÃ©rateurs

Les signatures ci-dessous sont **contractuelles** pour lâ€™implÃ©mentation : entrÃ©es/sorties, pas de logique dÃ©cisionnelle dans les Tools (ils exÃ©cutent uniquement).

### 3.1 MiyuClickerSprites (Toolkit)

| Tool | EntrÃ©es | Sortie | Description |
|------|---------|--------|-------------|
| `tool.miyuclicker.sprites.load` | `path: &str` ou `bytes`, `id: &str` | `Result<(), Error>` | Charge une image / spritesheet, enregistre dans le cache avec `id`. |
| `tool.miyuclicker.sprites.frame_rect` | `id: &str`, `frame_index: u32` | `Option<Rect>` | Retourne le rectangle (UV ou pixels) pour la frame donnÃ©e (grille spritesheet). |
| `tool.miyuclicker.sprites.animate` | `id: &str`, `anim_id: &str`, `delta_s: f32` | `u32` (frame courante) | Avance lâ€™animation de `delta_s` ; retourne lâ€™index de frame Ã  afficher. |

### 3.2 MiyuClickerIdleSim (Toolkit)

| Tool | EntrÃ©es | Sortie | Description |
|------|---------|--------|-------------|
| `tool.miyuclicker.idlesim.tick` | `state: &mut GameState`, `delta_s: f64` | `()` | Met Ã  jour ressources (production selon allocation, consommation nourriture), moral, fÃ©conditÃ©, cap_gens. Applique gains/coÃ»ts des clics si prÃ©-appliquÃ©s dans state (ou appel sÃ©parÃ©). |
| `tool.miyuclicker.idlesim.apply_allocation` | `state: &mut GameState`, `allocation: &Allocation` | `Result<(), Error>` | Affecte lâ€™allocation dans state (avec vÃ©rification somme â‰¤ gens). |
| `tool.miyuclicker.idlesim.apply_click` | `state: &mut GameState`, `target: ClickTarget` | `()` | Applique un gain au clic : Champs â†’ +nourriture, Ateliers â†’ +outils, ChÃ¢teau â†’ +soldats (si coÃ»t OK), Village â†’ +gens (si nourriture et cap OK). |

**ClickTarget :** enum `Champs | Ateliers | Chateau | Village`.

### 3.3 MiyuClickerSave (Toolkit)

| Tool | EntrÃ©es | Sortie | Description |
|------|---------|--------|-------------|
| `tool.miyuclicker.save.slot_write` | `slot_id: u8`, `state: &GameState` | `Result<(), Error>` | SÃ©rialise `state` et Ã©crit dans le slot (fichier JSON via serde + I/O). |
| `tool.miyuclicker.save.slot_read` | `slot_id: u8` | `Result<GameState, Error>` | Lit et dÃ©sÃ©rialise lâ€™Ã©tat du slot. |
| `tool.miyuclicker.save.slot_list` | `()` | `Vec<SlotMetadata>` | Retourne pour chaque slot (1, 2, 3) : occupÃ©, date, rÃ©sumÃ© (optionnel). |

**SlotMetadata :** `{ slot_id, occupied: bool, saved_at: Option<DateTime>`, `summary: Option<String> }`.

### 3.4 MiyuClickerCombat (Tool)

| Tool | EntrÃ©es | Sortie | Description |
|------|---------|--------|-------------|
| `tool.miyuclicker.combat.resolve` | `attaquant: i64`, `defenseur: i64`, `stats_att: f64`, `stats_def: f64` | `CombatResult` | RÃ©solution combat : hasard + puissance relative â†’ `{ vainqueur: Joueur|Adverse, troupes_att_restantes: i64, troupes_def_restantes: i64 }`. |

### 3.5 MiyuClickerCarte (Toolkit)

| Tool | EntrÃ©es | Sortie | Description |
|------|---------|--------|-------------|
| `tool.miyuclicker.carte.move_troops` | `state: &mut GameState`, `from_cite_id: &str`, `to_cite_id: &str`, `count: i64` | `Result<(), Error>` | Enregistre un dÃ©placement (from, to, count) ; dÃ©duit les troupes de la citÃ© source. |
| `tool.miyuclicker.carte.model_update` | `state: &mut GameState`, `delta_s: f64` | `()` | Avance les dÃ©placements (progress) ; Ã  arrivÃ©e, appelle `combat.resolve` si citÃ© adverse, sinon renfort ; met Ã  jour citÃ©s (conquÃªte, troupes). |
| `tool.miyuclicker.carte.resolve_arrival` | `state: &mut GameState`, `deplacement_id: usize` | `()` | Traite lâ€™arrivÃ©e dâ€™un dÃ©placement : combat ou renfort ; met Ã  jour state. |

### 3.6 OpÃ©rateurs â€” flux dâ€™appel (rÃ©sumÃ©)

| OpÃ©rateur | AppelÃ© par | Appelle |
|------------|------------|---------|
| **MiyuClickerUI** | Boucle Dioxus | MiyuClickerSim (tick, apply_click, apply_allocation), MiyuClickerSave (slot_list, slot_read, slot_write), MiyuClickerCarte (move_troops, model_update pour tick carte). |
| **MiyuClickerSim** | MiyuClickerUI | MiyuClickerIdleSim (tick, apply_allocation, apply_click). |
| **MiyuClickerSave** | MiyuClickerUI | MiyuClickerSave Tools (slot_*). |
| **MiyuClickerCarte** | MiyuClickerUI (tick carte) | MiyuClickerCarte (model_update), MiyuClickerCombat (resolve). |

---

## 4. Format de sauvegarde

Format recommandÃ© : **JSON** (serde_json) pour lisibilitÃ© et debug ; binaire (bincode) possible pour taille.

### 4.1 SchÃ©ma JSON (structure)

- **Version** : champ `version` (ex. `"1.0"`) en tÃªte pour migrations futures.
- **Ã‰tat** : mÃªme structure que `GameState` (Â§ 1) :
  - `resources` : or, gens, soldats, recherche, nourriture, bois, pierre, fer, outils, armes
  - `cap_moral` : cap_gens, habitations, moral, fecondite
  - `allocation` : objet { champs, ateliers, scierie, carriere, mine, recherche }
  - `carte` : cites (liste), routes (liste), deplacements (liste)
  - `meta` : temps_simule_s, slot_id, version_sauvegarde

### 4.2 Fichiers slots

- **Emplacement** : rÃ©pertoire de donnÃ©es applicatif (ex. `data_dir`) ou rÃ©pertoire projet : `miyuclicker_slot_1.json`, `miyuclicker_slot_2.json`, `miyuclicker_slot_3.json`.
- **MÃ©tadonnÃ©es pour slot_list** : dÃ©rivÃ©es du fichier (date modification) et dâ€™un champ `saved_at` + `summary` dans le JSON (ou en en-tÃªte court).

### 4.3 Exemple minimal (JSON)

```json
{
  "version": "1.0",
  "resources": {
    "or": 0, "gens": 10, "soldats": 0, "recherche": 0,
    "nourriture": 100, "bois": 50, "pierre": 50, "fer": 0,
    "outils": 20, "armes": 0
  },
  "cap_moral": { "cap_gens": 20, "habitations": 1, "moral": 1.0, "fecondite": 1.0 },
  "allocation": { "champs": 0, "ateliers": 0, "scierie": 0, "carriere": 0, "mine": 0, "recherche": 0 },
  "carte": { "cites": [], "routes": [], "deplacements": [] },
  "meta": { "temps_simule_s": 0, "slot_id": 1, "version_sauvegarde": "1.0" }
}
```

---

## 5. Phases dâ€™implÃ©mentation

### Phase 1 â€” Shell et navigation (sans simulation)

| Livrable | Contenu |
|----------|---------|
| **Projet Dioxus** | CrÃ©ation du projet Rust (crate binaire), dÃ©pendances dioxus (feature `desktop`), serde, serde_json. |
| **Ã‰crans** | Loading (barre ou spinner) â†’ Landing (titre, [Jouer], roue config) â†’ Slots (3 slots, appel mock slot_list, [Retour]). Pas de chargement/sauvegarde rÃ©el. |
| **Ã‰tat global** | `AppState` : Ã©cran courant (Loading, Landing, Slots, MaCitee, CarteMonde), `GameState` minimal (ressources Ã  0 ou valeurs de test). Pas de tick. |
| **Navigation** | Transition Slots â†’ Ma citÃ©e (avec Ã©tat de test ou Ã©tat initial fixe). Barre haut sur Ma citÃ©e (ressources en dur ou Ã©tat minimal) ; lien Â« Carte du monde Â» â†’ Ã©cran Carte (vide ou placeholder). |

**CritÃ¨re de fin :** Parcours complet Loading â†’ Landing â†’ Slots â†’ Ma citÃ©e â†’ Carte du monde, sans crash.

### Phase 2 â€” Gestion (Ma citÃ©e) et sauvegarde

| Livrable | Contenu |
|----------|---------|
| **GameState complet** | Structure Â§ 1 (ressources, cap, moral, allocation, meta). Carte : cites/routes/deplacements vides ou 1 citÃ© joueur. |
| **MiyuClickerIdleSim** | ImplÃ©mentation `tick`, `apply_allocation`, `apply_click`. Formules minimales (ex. production = gens_affectÃ©s * k par seconde). |
| **MiyuClickerSave** | ImplÃ©mentation slot_write, slot_read, slot_list (fichiers JSON dans app_data_dir). |
| **Ã‰cran Ma citÃ©e** | Barre 2 lignes (donnÃ©es depuis GameState) ; 4 boutons (clic â†’ apply_click) ; liste dÃ©roulante affectation (sliders â†’ apply_allocation). Timer : tick(state, delta) Ã  chaque frame (ou intervalle fixe). |
| **Slots rÃ©el** | slot_list() pour afficher date/occupÃ© ; Nouvelle partie â†’ init state, slot_id ; Charger â†’ slot_read ; Sauvegarder (menu config) â†’ slot_write. |

**CritÃ¨re de fin :** Clic et affectation mettent Ã  jour les ressources ; sauvegarde et chargement restaurant lâ€™Ã©tat.

### Phase 3 â€” Carte et combat

| Livrable | Contenu |
|----------|---------|
| **ModÃ¨le carte** | GÃ©nÃ©ration ou chargement dâ€™un graphe (citÃ©s + routes) ; au moins 1 citÃ© joueur, N citÃ©s adverses. |
| **MiyuClickerCarte** | move_troops, model_update (avancement dÃ©placements), resolve_arrival. |
| **MiyuClickerCombat** | resolve(attaquant, defenseur, â€¦) â†’ vainqueur, troupes restantes. |
| **Ã‰cran Carte du monde** | Rendu citÃ©s et routes (Ã©lÃ©ments SVG inline ou canvas Dioxus) ; clic citÃ© â†’ infos ; envoi troupes (input + bouton) ; tick carte (model_update) en parallÃ¨le du tick Sim. |
| **IntÃ©gration** | Ã‰tat partagÃ© : mÃªme GameState pour Sim et Carte ; troupes dÃ©duites de la citÃ© joueur Ã  lâ€™envoi, mises Ã  jour aprÃ¨s combat. |

**CritÃ¨re de fin :** Envoi de troupes, dÃ©placement, rÃ©solution combat, conquÃªte de citÃ© (bonus ressources optionnel pour MVP).

### Phase 4 â€” Polish (optionnel dans MVP)

| Livrable | Contenu |
|----------|---------|
| **MiyuClickerSprites** | Chargement packs UI (`ui/game_ui_pack`), boutons et icÃ´nes, barres. |
| **Erreurs et confirmation** | Message si chargement Ã©choue ; confirmation Ã©crasement slot. |
| **RÃ©solution / Langue** | Menu config : rÃ©solution fenÃªtre, langue (i18n minimal). |

---

## 6. RÃ©fÃ©rences

- [MiyuClicker - MVP Ecrans et Mecaniques](MiyukiniClicker%20-%20MVP%20Ecrans%20et%20Mecaniques.md)
- [MiyuClicker - Operateurs et Toolkits](MiyukiniClicker%20-%20Operateurs%20et%20Toolkits.md)
- [MiyuClicker - Ergonomie Ecran Gestion](MiyukiniClicker%20-%20Ergonomie%20Ecran%20Gestion.md)
- [MiyuClicker - Parcours Utilisateur](MiyukiniClicker%20-%20Parcours%20Utilisateur.md)
- [MiyuClicker - Document Fondateur](MiyukiniClicker%20-%20Document%20Fondateur.md)
- [Miyukini - Stack UI Dioxus](..//..//_index.md)

---

**Document crÃ©Ã© le :** 2026-02-01  
**DerniÃ¨re mise Ã  jour :** 2026-02-11  
**Statut :** Guide dâ€™implÃ©mentation MVP â€” modÃ¨le dâ€™Ã©tat, specs Ã©crans, APIs, format sauvegarde, phases


