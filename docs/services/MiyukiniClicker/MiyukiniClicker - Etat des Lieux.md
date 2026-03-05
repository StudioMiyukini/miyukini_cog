# MiyuClicker â€” Ã‰tat des lieux (code vs spec)

## Contexte

Ce document constitue la **source de vÃ©ritÃ©** sur ce qui est **implÃ©mentÃ©** par rapport aux spÃ©cifications (Guide MVP, guides Zone CitÃ© et BÃ¢timents). Il permet de reprendre le dÃ©veloppement sans ambiguÃ¯tÃ©.

**RÃ©fÃ©rences :** [MiyuClicker - Guide Implementation MVP](MiyukiniClicker%20-%20Guide%20Implementation%20MVP.md), [MiyuClicker - Guide Implementation Zone Cite et Construction](MiyukiniClicker%20-%20Guide%20Implementation%20Zone%20Cite%20et%20Construction.md), [MiyuClicker - Batiments Macons et Construction](MiyukiniClicker%20-%20Batiments%20Macons%20et%20Construction.md).

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre :** Comparaison code actuel (`crates/miyuclicker`) vs specs ; statut Fait / Partiel / Non fait par brique.
- **Mise Ã  jour :** Ã€ mettre Ã  jour Ã  chaque livraison significative ou avant de reprendre une phase de dev.

---

## 1. Ã‰crans

| Ã‰cran | Spec (Guide MVP) | Statut | Commentaire |
|-------|------------------|--------|-------------|
| **Loading** | Barre ou spinner, transition auto vers Landing | **Fait** | `ui_loading`, progression, transition quand `loading_done`. |
| **Landing** | Titre, [Jouer], roue config | **Fait** | Bouton Jouer â†’ Slots ; menu config (roue) prÃ©sent. |
| **Slots** | 3 slots, Nouvelle partie / Charger / Retour, confirmation Ã©crasement | **Fait** | `slot_list`, Nouvelle partie, Charger, Retour ; `confirm_overwrite` pour Ã©crasement. |
| **Ma citÃ©e** | Barre 2 lignes (ressources), 4 boutons clic, liste affectation, lien Carte, config, tick | **Fait** | Barre ressources (ligne 1 : or, gens, soldats, recherche ; ligne 2 : nourriture, bois, pierre, fer, outils, armes) ; boutons clic (Champs, Bois, Pierre, Fer, Ateliers, ChÃ¢teau, Village) + Guilde maÃ§ons ; cartes bÃ¢timents + construction ; lien Carte du monde ; config ; tick. |
| **Carte du monde** | Barre identique, rendu citÃ©s/routes/dÃ©placements, clic citÃ©, envoi troupes, tick carte | **Fait** | Rendu cercles/lignes ; panneau citÃ© sÃ©lectionnÃ©e ; envoi troupes (pending_move_troops) ; model_update + resolve_arrival. |

**SynthÃ¨se Ã©crans :** Tous les Ã©crans du Guide MVP sont implÃ©mentÃ©s. Ma citÃ©e dÃ©passe la spec (boutons Bois/Pierre/Fer en plus des 4 Â« gros Â» ; cartes bÃ¢timents et construction ; zone citÃ© ciel/sol).

---

## 2. ModÃ¨le dâ€™Ã©tat (GameState)

### 2.1 Ressources (stocks)

| Champ (spec) | Code | Statut | Commentaire |
|--------------|------|--------|-------------|
| `or` | `state.or` (i64) | **Fait** | PrÃ©sent. |
| `gens` | `state.gens` | **Fait** | |
| `soldats` | `state.soldats` | **Fait** | |
| `recherche` | `state.recherche` | **Fait** | |
| `nourriture` | `state.nourriture` (f64) | **Fait** | |
| `bois` | `state.bois` | **Fait** | |
| `pierre` | `state.pierre` | **Fait** | |
| `fer` | `state.fer` | **Fait** | |
| `outils` | `state.outils` | **Fait** | |
| `armes` | `state.armes` | **Fait** | |

### 2.2 Cap et moral

| Champ (spec) | Code | Statut | Commentaire |
|--------------|------|--------|-------------|
| `cap_gens` | `state.cap_gens` | **Fait** | DÃ©rivÃ© des maisons (cap = maisons Ã— 4). |
| `habitations` | `state.maisons` | **Fait** | Nom diffÃ©rent (serde alias `habitations` en sauvegarde pour rÃ©trocompat). |
| `moral` | `state.moral` | **Fait** | 0..1 ; affichÃ© en % (bonheur). |
| `fecondite` | `state.fecondite` | **Fait** | |

**Champs en plus par rapport au Guide MVP (bÃ¢timents, construction, Game Over) :**  
`maisons`, `caserne_lvl`, `grenier_lvl`, `depot_lvl`, `entrepot_lvl`, `macons`, `construction_maison` (+ paid), `construction_caserne/grenier/depot/entrepot`, `allocation_macons`, `jours_nourriture_zero`, `game_over`, `route_duree_by_pair`. Tous **implÃ©mentÃ©s** et alignÃ©s avec les docs BÃ¢timents / Zone CitÃ©.

### 2.3 Affectation des gens (allocation)

| Spec | Code | Statut | Commentaire |
|------|------|--------|-------------|
| champs, ateliers, scierie, carriere, mine, recherche | `Allocation` (mÃªmes champs) | **Fait** | |
| forge | `Allocation.forge` | **Fait** | En plus du Guide MVP (production armes). |
| Contrainte total â‰¤ gens | `idlesim::apply_allocation` | **Fait** | |

### 2.4 Carte (MVP)

| Ã‰lÃ©ment | Code | Statut | Commentaire |
|---------|------|--------|-------------|
| CitÃ©s (id, nom, x, y, proprietaire, troupes) | `Cite` | **Fait** | Pas de champ `niveau` (optionnel en spec). |
| Routes (cite_a, cite_b, duree_s) | `Route` | **Fait** | |
| DÃ©placements (from, to, troupes, progress, attaquant) | `Deplacement` | **Fait** | |
| Cache durÃ©e routes | `route_duree_by_pair` | **Fait** | Optimisation (Ã©vite recalcul Ã  chaque tick). |

### 2.5 MÃ©tadonnÃ©es partie

| Champ | Code | Statut |
|-------|------|--------|
| temps_simule_s | `state.temps_simule_s` | **Fait** |
| slot_id | `state.slot_id` | **Fait** |
| version_sauvegarde | `state.version_sauvegarde` | **Fait** |

**SynthÃ¨se modÃ¨le :** AlignÃ© sur le Guide MVP avec extensions (bÃ¢timents, maÃ§ons, construction, Game Over, forge). Pas dâ€™Ã©cart bloquant.

---

## 3. MÃ©caniques de jeu

| MÃ©canique | Spec | Statut | Commentaire |
|-----------|------|--------|-------------|
| **Clic** | Champs, Ateliers, ChÃ¢teau, Village | **Fait** | + Bois, Pierre, Fer (gain +1 par clic). |
| **Tick (simulation)** | tick(state, delta), production/consommation, moral, fÃ©conditÃ© | **Fait** | `idlesim::tick` ; production par lieu, consommation nourriture, moral/fecondite, Game Over 7j Ã  0 nourriture. |
| **Sauvegarde** | slot_write, slot_read, slot_list (JSON) | **Fait** | Fichiers par slot dans `data_dir` ; SlotMetadata (date, occupÃ©). |
| **Combat** | resolve(attaquant, defenseur, stats) â†’ vainqueur, troupes restantes | **Fait** | `combat::resolve` ; seed en paramÃ¨tre (code utilise SystemTime pour seed en prod â€” non dÃ©terministe, voir Audit). |
| **Envoi troupes** | move_troops(from, to, count) | **Fait** | `carte::move_troops` ; dÃ©duction troupes source, crÃ©ation `Deplacement`. |
| **Avancement dÃ©placements** | model_update(delta), resolve_arrival | **Fait** | `carte::model_update` ; Ã  arrivÃ©e appel `combat::resolve` si citÃ© adverse. |
| **Construction** | CoÃ»ts, pts par maÃ§on, niveau bÃ¢timents (Guide BÃ¢timents) | **Fait** | CoÃ»ts et pts dans `idlesim` ; paiement au clic Â« Construire Â» (maison) ou dans try_complete selon bÃ¢timent ; allocation maÃ§ons. |
| **Guilde maÃ§ons** | 1 pop â†’ 1 maÃ§on | **Fait** | `convert_pop_to_macon` (idlesim). |

---

## 4. Specs additionnelles (Zone CitÃ©, BÃ¢timents)

| Ã‰lÃ©ment | Document | Statut | Commentaire |
|---------|----------|--------|-------------|
| Bouton construction (vert/blanc, paiement au clic) | Guide Zone CitÃ© | **Fait** | Cartes bÃ¢timents avec bouton Construire, conditions ressources, dÃ©marrage construction. |
| Zone citÃ© (ciel 60 % / sol 40 %, min 200 px) | Guide Zone CitÃ© | **Fait** | Rectangle ciel (bleu) + sol (vert) entre header et liste. |
| Sprites personnages 3Ã—1 px, dÃ©placement alÃ©atoire | Guide Zone CitÃ© | **Non fait** | Pas de sprites ni mouvement dans la zone citÃ©. |
| BÃ¢timents (Maison, Caserne, Grenier, DÃ©pÃ´t, EntrepÃ´t) | BÃ¢timents MaÃ§ons | **Fait** | Niveaux, caps (soldats, nourriture, matiÃ¨res, manufacturÃ©s), construction par maÃ§ons. |
| Game Over (7 jours Ã  0 nourriture) | SystÃ¨me Bonheur | **Fait** | `jours_nourriture_zero`, `game_over` ; overlay Â« Game Over Â» + retour Slots. |

---

## 5. APIs Toolkits / OpÃ©rateurs (alignement code)

Les Â« Tools Â» du Guide MVP sont implÃ©mentÃ©s en **fonctions de module** (pas de couche OpÃ©rateur sÃ©parÃ©e) :

| API spec | ImplÃ©mentation | Fichier |
|----------|----------------|---------|
| idlesim.tick | `idlesim::tick` | idlesim.rs |
| idlesim.apply_allocation | `idlesim::apply_allocation` | idlesim.rs |
| idlesim.apply_click | `idlesim::apply_click` | idlesim.rs |
| save.slot_write / slot_read / slot_list | `save::slot_write`, `slot_read`, `slot_list` | save.rs |
| combat.resolve | `combat::resolve` | combat.rs |
| carte.move_troops | `carte::move_troops` | carte.rs |
| carte.model_update | `carte::model_update` | carte.rs |

**MiyuClickerSprites (Toolkit) :** Non implÃ©mentÃ© en tant que Toolkit (pas de `load`, `frame_rect`, `animate`). Chemins UI dans `ui_assets.rs` ; pas de cache textures/spritesheets centralisÃ© dans le crate.

---

## 6. SynthÃ¨se globale

| Domaine | Statut global | Prochaine Ã©tape suggÃ©rÃ©e |
|---------|----------------|---------------------------|
| Ã‰crans MVP | **Fait** | Polish (rÃ©solution, langue) si besoin. |
| ModÃ¨le dâ€™Ã©tat | **Fait** (avec extensions) | Tenir Ã  jour ce doc si ajout de champs. |
| MÃ©caniques (clic, tick, save, combat, carte) | **Fait** | Voir Audit (perf, DRY, testabilitÃ©). |
| Zone CitÃ© (sprites + mouvement) | **Partiel** | ImplÃ©menter sprites 3Ã—1 et dÃ©placement alÃ©atoire si prioritÃ©. |
| MiyuClickerSprites Toolkit | **Non fait** | Optionnel MVP ; utile pour Phase 4 polish. |

---

**DerniÃ¨re mise Ã  jour :** 2026-02-02  
**Statut :** Document de rÃ©fÃ©rence â€” Ã©tat des lieux code vs spec

