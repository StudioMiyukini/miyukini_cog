# MiyuClicker — Guide d’implémentation MVP

## Contexte

Ce document constitue le **guide d’implémentation** du MVP MiyuClicker. Il fournit le **modèle d’état**, les **spécifications détaillées par écran**, les **APIs des Toolkits et Opérateurs**, le **format de sauvegarde** et les **phases d’implémentation** pour permettre le développement du jeu de façon structurée et alignée avec l’architecture Miyukini.

## Portée / Scope

- **Périmètre :** Modèle de données (état partie), specs UI par écran (widgets, layout, transitions), signatures des Tools/Opérateurs, schéma de sauvegarde, ordre de livraison par phases.
- **Hors périmètre :** Formules d’équilibrage numériques, assets graphiques, implémentation effective du code (crates, fichiers) — ce guide est une **spécification pour l’implémentation**.

---

## 1. Modèle d’état (état de la partie)

L’état du jeu est la **source de vérité** partagée entre MiyuClickerSim, MiyuClickerSave, MiyuClickerCarte et MiyuClickerUI. Il doit être **sérialisable** pour la sauvegarde.

### 1.1 Ressources (stocks)

| Champ | Type | Description | Valeur initiale (nouvelle partie) |
|-------|------|-------------|-----------------------------------|
| `or` | `i64` ou `f64` | Stock d’or | 0 |
| `gens` | `i64` | Population civile (disponible + affectée) | 10 |
| `soldats` | `i64` | Nombre de soldats | 0 |
| `recherche` | `i64` ou `f64` | Points de recherche | 0 |
| `nourriture` | `f64` | Stock nourriture | 100 |
| `bois` | `f64` | Stock bois | 50 |
| `pierre` | `f64` | Stock pierre | 50 |
| `fer` | `f64` | Stock fer | 0 |
| `outils` | `f64` | Stock outils | 20 |
| `armes` | `i64` | Stock armes | 0 |

**Règle :** Les stocks ne peuvent pas être négatifs (clamp à 0 après chaque tick / action).

### 1.2 Cap et moral

| Champ | Type | Description | Valeur initiale |
|-------|------|-------------|-----------------|
| `cap_gens` | `i64` | Plafond de population = f(habitations) | 20 |
| `habitations` | `i64` | Nombre d’habitations construites | 1 |
| `moral` | `f64` | Moral (0..1) ; impacte fécondité | 1.0 |
| `fecondite` | `f64` | Fécondité (0..1) ; impacte croissance gens | 1.0 |

**Règle métier :** Si `nourriture` < consommation sur un tick, `moral` et `fecondite` baissent ; sinon ils remontent (plafonnés à 1.0).

### 1.3 Affectation des gens (allocation)

Structure : **map** lieu → nombre de gens affectés.

| Lieu (clé) | Ressource produite par tick |
|------------|-----------------------------|
| `champs` | Nourriture |
| `ateliers` | Outils |
| `scierie` | Bois |
| `carriere` | Pierre |
| `mine` | Fer |
| `recherche` | Points de recherche |

**Contrainte :** Somme des gens affectés ≤ `gens`. Les gens « disponibles » = `gens` − somme(affectations).

**Type proposé (Rust) :** `HashMap<String, i64>` ou struct `Allocation { champs, ateliers, scierie, carriere, mine, recherche }`.

### 1.4 Carte (MVP)

| Élément | Type | Description |
|--------|------|-------------|
| **Cités** | Liste de `Cite` | Chaque cité : `id`, `nom`, `x`, `y` (coords écran ou logiques), `proprietaire` (joueur / adverse), `troupes`, `niveau` (optionnel). |
| **Routes** | Liste de `Route` | Paire `(cite_id_a, cite_id_b)`, `duree_deplacement` (secondes simulées). |
| **Déplacements en cours** | Liste de `Deplacement` | `from_cite_id`, `to_cite_id`, `troupes`, `progress` (0..1), `attaquant` (bool). |

**Propriétaire :** `Joueur` | `Adverse(id)` pour distinguer les cités adverses.

### 1.5 Métadonnées partie

| Champ | Type | Description |
|-------|------|-------------|
| `temps_simule_s` | `f64` | Temps simulé écoulé (secondes) |
| `slot_id` | `u8` | Slot de sauvegarde (1, 2, 3) |
| `version_sauvegarde` | `String` | Ex. `"1.0"` pour compatibilité future |

### 1.6 Synthèse structure état (pseudo-Rust)

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

## 2. Spécifications détaillées par écran

### 2.1 Écran Loading

| Élément | Spécification |
|--------|----------------|
| **Layout** | Plein écran (ou zone centrale). Fond : couleur ou texture. |
| **Widgets** | Titre / logo (centré) ; barre de progression ou spinner ; texte optionnel « Chargement… » ou pourcentage. |
| **Données** | Progression 0..1 (assets chargés, init terminée). |
| **Transition** | Dès que `loading_done == true` → écran **Landing**. Pas de bouton utilisateur. |
| **Erreur** | Si échec chargement : message + bouton « Réessayer » ou « Quitter ». |

**États UI :** `Loading { progress }` → `Landing`.

### 2.2 Écran Landing

| Élément | Spécification |
|--------|----------------|
| **Layout** | Titre / logo en haut ou centre ; bouton **[Jouer]** bien visible (centre ou sous le titre) ; icône **roue de configuration** en haut à droite. |
| **Widgets** | `Label` titre ; `Button` « Jouer » ; `Button` icône engrenage → ouvre menu déroulant. |
| **Menu config (déroulant)** | Ancré à droite de la roue. Entrées : **Sauvegarder** (désactivé ou masqué au Landing), **Changer la résolution**, **Langue**, **À propos**. Clic extérieur ou après action → fermeture. |
| **Transition** | Clic [Jouer] → écran **Slots**. |

**États UI :** `Landing` ; sous-état optionnel `ConfigMenuOpen`.

### 2.3 Écran Sélection des slots

| Élément | Spécification |
|--------|----------------|
| **Layout** | Titre « Choisir une sauvegarde » ; 3 zones (slot 1, 2, 3) ; chaque zone : résumé (date/heure si existant, ou « Vide »), bouton **[Nouvelle partie]** ou **[Charger]** ; bouton **[Retour]** (vers Landing). |
| **Données** | Appel `save.slot_list()` → métadonnées des 3 slots (date, résumé, occupé). |
| **Actions** | **[Nouvelle partie]** sur slot N : init état initial, `slot_id = N`, transition → **Ma citée**. **[Charger]** sur slot N : `save.slot_read(N)` → état, transition → **Ma citée**. **[Retour]** → **Landing**. |
| **Écrasement** | Si « Nouvelle partie » sur slot occupé : afficher confirmation « Écraser la sauvegarde ? » ; si oui → écraser puis lancer. |

**États UI :** `Slots` ; sous-état optionnel `ConfirmOverwrite(slot_id)`.

### 2.4 Écran Ma citée (fenêtre principale — gestion)

| Zone | Widgets / Layout |
|------|------------------|
| **Barre haut (ligne 1)** | Icône + valeur pour **Or**, **Gens**, **Soldats**, **Recherche** (ordre fixe) ; puis **Ma citée** (actif / surligné), **Carte du monde** (lien), **⚙** (roue config). |
| **Barre haut (ligne 2)** | Icône + valeur pour **Nourriture**, **Bois**, **Pierre**, **Fer**, **Outils**, **Armes** (ordre fixe). |
| **Partie gauche** | 4 **gros boutons** : **Champs**, **Ateliers**, **Château**, **Village**. Clic → intention vers Sim (gain clic). |
| **Partie droite** | **Liste déroulante** (ou panneau extensible) : pour chaque lieu (Champs, Ateliers, Scierie, Carrière, Mine, Recherche), afficher lieu + ressource générée + **nombre de gens affectés** (slider ou spinbox). Contrainte : total affecté ≤ gens. |
| **Menu config** | Identique au Landing ; **Sauvegarder** actif → appelle `save.slot_write(slot_id, state)`. |

**Transitions :** Clic « Carte du monde » → écran **Carte du monde**. Clic roue → menu config. Tick (timer) → appel Sim `tick(state, delta)` ; mise à jour état ; repaint.

**Données affichées :** Lecture de `GameState` (ressources, allocation) ; mise à jour après chaque tick et après chaque clic (intention traitée par Sim).

### 2.5 Écran Carte du monde

| Élément | Spécification |
|--------|----------------|
| **Layout** | Même barre haut (ligne 1 + 2) que Ma citée ; **Carte du monde** surligné, **Ma citée** lien. Zone carte : custom painting (egui) ou zone cliquable. |
| **Rendu carte** | Nœuds = cités (cercles ou sprites) ; arêtes = routes (lignes) ; déplacements en cours (ligne en pointillés ou segment progressif). Couleur / icône selon propriétaire (joueur / adverse). |
| **Interaction** | Clic sur une cité → afficher panneau latéral ou popup : nom, propriétaire, troupes. Si cité adverse et cité joueur sélectionnée : bouton « Envoyer X soldats » (X saisi ou slider) → intention `carte.move_troops(from, to, count)`. |
| **Tick** | Avancement des déplacements (`progress += delta / duree`) ; à arrivée (`progress >= 1`) → `carte.resolve_arrival` → Combat si cité adverse → mise à jour cités et troupes. |

**Données** : `GameState.cites`, `routes`, `deplacements` ; lecture par UI, mise à jour par Opérateur MiyuClickerCarte.

---

## 3. APIs Toolkits et Opérateurs

Les signatures ci-dessous sont **contractuelles** pour l’implémentation : entrées/sorties, pas de logique décisionnelle dans les Tools (ils exécutent uniquement).

### 3.1 MiyuClickerSprites (Toolkit)

| Tool | Entrées | Sortie | Description |
|------|---------|--------|-------------|
| `tool.miyuclicker.sprites.load` | `path: &str` ou `bytes`, `id: &str` | `Result<(), Error>` | Charge une image / spritesheet, enregistre dans le cache avec `id`. |
| `tool.miyuclicker.sprites.frame_rect` | `id: &str`, `frame_index: u32` | `Option<Rect>` | Retourne le rectangle (UV ou pixels) pour la frame donnée (grille spritesheet). |
| `tool.miyuclicker.sprites.animate` | `id: &str`, `anim_id: &str`, `delta_s: f32` | `u32` (frame courante) | Avance l’animation de `delta_s` ; retourne l’index de frame à afficher. |

### 3.2 MiyuClickerIdleSim (Toolkit)

| Tool | Entrées | Sortie | Description |
|------|---------|--------|-------------|
| `tool.miyuclicker.idlesim.tick` | `state: &mut GameState`, `delta_s: f64` | `()` | Met à jour ressources (production selon allocation, consommation nourriture), moral, fécondité, cap_gens. Applique gains/coûts des clics si pré-appliqués dans state (ou appel séparé). |
| `tool.miyuclicker.idlesim.apply_allocation` | `state: &mut GameState`, `allocation: &Allocation` | `Result<(), Error>` | Affecte l’allocation dans state (avec vérification somme ≤ gens). |
| `tool.miyuclicker.idlesim.apply_click` | `state: &mut GameState`, `target: ClickTarget` | `()` | Applique un gain au clic : Champs → +nourriture, Ateliers → +outils, Château → +soldats (si coût OK), Village → +gens (si nourriture et cap OK). |

**ClickTarget :** enum `Champs | Ateliers | Chateau | Village`.

### 3.3 MiyuClickerSave (Toolkit)

| Tool | Entrées | Sortie | Description |
|------|---------|--------|-------------|
| `tool.miyuclicker.save.slot_write` | `slot_id: u8`, `state: &GameState` | `Result<(), Error>` | Sérialise `state` et écrit dans le slot (fichier ou eframe persistence). |
| `tool.miyuclicker.save.slot_read` | `slot_id: u8` | `Result<GameState, Error>` | Lit et désérialise l’état du slot. |
| `tool.miyuclicker.save.slot_list` | `()` | `Vec<SlotMetadata>` | Retourne pour chaque slot (1, 2, 3) : occupé, date, résumé (optionnel). |

**SlotMetadata :** `{ slot_id, occupied: bool, saved_at: Option<DateTime>`, `summary: Option<String> }`.

### 3.4 MiyuClickerCombat (Tool)

| Tool | Entrées | Sortie | Description |
|------|---------|--------|-------------|
| `tool.miyuclicker.combat.resolve` | `attaquant: i64`, `defenseur: i64`, `stats_att: f64`, `stats_def: f64` | `CombatResult` | Résolution combat : hasard + puissance relative → `{ vainqueur: Joueur|Adverse, troupes_att_restantes: i64, troupes_def_restantes: i64 }`. |

### 3.5 MiyuClickerCarte (Toolkit)

| Tool | Entrées | Sortie | Description |
|------|---------|--------|-------------|
| `tool.miyuclicker.carte.move_troops` | `state: &mut GameState`, `from_cite_id: &str`, `to_cite_id: &str`, `count: i64` | `Result<(), Error>` | Enregistre un déplacement (from, to, count) ; déduit les troupes de la cité source. |
| `tool.miyuclicker.carte.model_update` | `state: &mut GameState`, `delta_s: f64` | `()` | Avance les déplacements (progress) ; à arrivée, appelle `combat.resolve` si cité adverse, sinon renfort ; met à jour cités (conquête, troupes). |
| `tool.miyuclicker.carte.resolve_arrival` | `state: &mut GameState`, `deplacement_id: usize` | `()` | Traite l’arrivée d’un déplacement : combat ou renfort ; met à jour state. |

### 3.6 Opérateurs — flux d’appel (résumé)

| Opérateur | Appelé par | Appelle |
|------------|------------|---------|
| **MiyuClickerUI** | Boucle eframe | MiyuClickerSim (tick, apply_click, apply_allocation), MiyuClickerSave (slot_list, slot_read, slot_write), MiyuClickerCarte (move_troops, model_update pour tick carte). |
| **MiyuClickerSim** | MiyuClickerUI | MiyuClickerIdleSim (tick, apply_allocation, apply_click). |
| **MiyuClickerSave** | MiyuClickerUI | MiyuClickerSave Tools (slot_*). |
| **MiyuClickerCarte** | MiyuClickerUI (tick carte) | MiyuClickerCarte (model_update), MiyuClickerCombat (resolve). |

---

## 4. Format de sauvegarde

Format recommandé : **JSON** (serde_json) pour lisibilité et debug ; binaire (bincode) possible pour taille.

### 4.1 Schéma JSON (structure)

- **Version** : champ `version` (ex. `"1.0"`) en tête pour migrations futures.
- **État** : même structure que `GameState` (§ 1) :
  - `resources` : or, gens, soldats, recherche, nourriture, bois, pierre, fer, outils, armes
  - `cap_moral` : cap_gens, habitations, moral, fecondite
  - `allocation` : objet { champs, ateliers, scierie, carriere, mine, recherche }
  - `carte` : cites (liste), routes (liste), deplacements (liste)
  - `meta` : temps_simule_s, slot_id, version_sauvegarde

### 4.2 Fichiers slots

- **Emplacement** : répertoire de données eframe (ex. `app_data_dir`) ou répertoire projet : `miyuclicker_slot_1.json`, `miyuclicker_slot_2.json`, `miyuclicker_slot_3.json`.
- **Métadonnées pour slot_list** : dérivées du fichier (date modification) et d’un champ `saved_at` + `summary` dans le JSON (ou en en-tête court).

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

## 5. Phases d’implémentation

### Phase 1 — Shell et navigation (sans simulation)

| Livrable | Contenu |
|----------|---------|
| **Projet eframe** | Création du projet Rust (crate binaire), dépendances egui, eframe, serde, serde_json. |
| **Écrans** | Loading (barre ou spinner) → Landing (titre, [Jouer], roue config) → Slots (3 slots, appel mock slot_list, [Retour]). Pas de chargement/sauvegarde réel. |
| **État global** | `AppState` : écran courant (Loading, Landing, Slots, MaCitee, CarteMonde), `GameState` minimal (ressources à 0 ou valeurs de test). Pas de tick. |
| **Navigation** | Transition Slots → Ma citée (avec état de test ou état initial fixe). Barre haut sur Ma citée (ressources en dur ou état minimal) ; lien « Carte du monde » → écran Carte (vide ou placeholder). |

**Critère de fin :** Parcours complet Loading → Landing → Slots → Ma citée → Carte du monde, sans crash.

### Phase 2 — Gestion (Ma citée) et sauvegarde

| Livrable | Contenu |
|----------|---------|
| **GameState complet** | Structure § 1 (ressources, cap, moral, allocation, meta). Carte : cites/routes/deplacements vides ou 1 cité joueur. |
| **MiyuClickerIdleSim** | Implémentation `tick`, `apply_allocation`, `apply_click`. Formules minimales (ex. production = gens_affectés * k par seconde). |
| **MiyuClickerSave** | Implémentation slot_write, slot_read, slot_list (fichiers JSON dans app_data_dir). |
| **Écran Ma citée** | Barre 2 lignes (données depuis GameState) ; 4 boutons (clic → apply_click) ; liste déroulante affectation (sliders → apply_allocation). Timer : tick(state, delta) à chaque frame (ou intervalle fixe). |
| **Slots réel** | slot_list() pour afficher date/occupé ; Nouvelle partie → init state, slot_id ; Charger → slot_read ; Sauvegarder (menu config) → slot_write. |

**Critère de fin :** Clic et affectation mettent à jour les ressources ; sauvegarde et chargement restaurant l’état.

### Phase 3 — Carte et combat

| Livrable | Contenu |
|----------|---------|
| **Modèle carte** | Génération ou chargement d’un graphe (cités + routes) ; au moins 1 cité joueur, N cités adverses. |
| **MiyuClickerCarte** | move_troops, model_update (avancement déplacements), resolve_arrival. |
| **MiyuClickerCombat** | resolve(attaquant, defenseur, …) → vainqueur, troupes restantes. |
| **Écran Carte du monde** | Rendu cités et routes (egui painter) ; clic cité → infos ; envoi troupes (input + bouton) ; tick carte (model_update) en parallèle du tick Sim. |
| **Intégration** | État partagé : même GameState pour Sim et Carte ; troupes déduites de la cité joueur à l’envoi, mises à jour après combat. |

**Critère de fin :** Envoi de troupes, déplacement, résolution combat, conquête de cité (bonus ressources optionnel pour MVP).

### Phase 4 — Polish (optionnel dans MVP)

| Livrable | Contenu |
|----------|---------|
| **MiyuClickerSprites** | Chargement packs UI (`ui/game_ui_pack`), boutons et icônes, barres. |
| **Erreurs et confirmation** | Message si chargement échoue ; confirmation écrasement slot. |
| **Résolution / Langue** | Menu config : résolution fenêtre, langue (i18n minimal). |

---

## 6. Références

- [MiyuClicker - MVP Ecrans et Mecaniques](MiyuClicker%20-%20MVP%20Ecrans%20et%20Mecaniques.md)
- [MiyuClicker - Operateurs et Toolkits](MiyuClicker%20-%20Operateurs%20et%20Toolkits.md)
- [MiyuClicker - Ergonomie Ecran Gestion](MiyuClicker%20-%20Ergonomie%20Ecran%20Gestion.md)
- [MiyuClicker - Parcours Utilisateur](MiyuClicker%20-%20Parcours%20Utilisateur.md)
- [MiyuClicker - Document Fondateur](MiyuClicker%20-%20Document%20Fondateur.md)
- [Miyukini - Stack UI egui eframe](../../ux_ui/Miyukini%20-%20Stack%20UI%20egui%20eframe.md)

---

**Document créé le :** 2026-02-01  
**Statut :** Guide d’implémentation MVP — modèle d’état, specs écrans, APIs, format sauvegarde, phases
