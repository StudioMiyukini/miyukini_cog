# MiyuClicker — État des lieux (code vs spec)

## Contexte

Ce document constitue la **source de vérité** sur ce qui est **implémenté** par rapport aux spécifications (Guide MVP, guides Zone Cité et Bâtiments). Il permet de reprendre le développement sans ambiguïté.

**Références :** [MiyuClicker - Guide Implementation MVP](MiyuClicker%20-%20Guide%20Implementation%20MVP.md), [MiyuClicker - Guide Implementation Zone Cite et Construction](MiyuClicker%20-%20Guide%20Implementation%20Zone%20Cite%20et%20Construction.md), [MiyuClicker - Batiments Macons et Construction](MiyuClicker%20-%20Batiments%20Macons%20et%20Construction.md).

## Portée / Scope

- **Périmètre :** Comparaison code actuel (`crates/miyuclicker`) vs specs ; statut Fait / Partiel / Non fait par brique.
- **Mise à jour :** À mettre à jour à chaque livraison significative ou avant de reprendre une phase de dev.

---

## 1. Écrans

| Écran | Spec (Guide MVP) | Statut | Commentaire |
|-------|------------------|--------|-------------|
| **Loading** | Barre ou spinner, transition auto vers Landing | **Fait** | `ui_loading`, progression, transition quand `loading_done`. |
| **Landing** | Titre, [Jouer], roue config | **Fait** | Bouton Jouer → Slots ; menu config (roue) présent. |
| **Slots** | 3 slots, Nouvelle partie / Charger / Retour, confirmation écrasement | **Fait** | `slot_list`, Nouvelle partie, Charger, Retour ; `confirm_overwrite` pour écrasement. |
| **Ma citée** | Barre 2 lignes (ressources), 4 boutons clic, liste affectation, lien Carte, config, tick | **Fait** | Barre ressources (ligne 1 : or, gens, soldats, recherche ; ligne 2 : nourriture, bois, pierre, fer, outils, armes) ; boutons clic (Champs, Bois, Pierre, Fer, Ateliers, Château, Village) + Guilde maçons ; cartes bâtiments + construction ; lien Carte du monde ; config ; tick. |
| **Carte du monde** | Barre identique, rendu cités/routes/déplacements, clic cité, envoi troupes, tick carte | **Fait** | Rendu cercles/lignes ; panneau cité sélectionnée ; envoi troupes (pending_move_troops) ; model_update + resolve_arrival. |

**Synthèse écrans :** Tous les écrans du Guide MVP sont implémentés. Ma citée dépasse la spec (boutons Bois/Pierre/Fer en plus des 4 « gros » ; cartes bâtiments et construction ; zone cité ciel/sol).

---

## 2. Modèle d’état (GameState)

### 2.1 Ressources (stocks)

| Champ (spec) | Code | Statut | Commentaire |
|--------------|------|--------|-------------|
| `or` | `state.or` (i64) | **Fait** | Présent. |
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
| `cap_gens` | `state.cap_gens` | **Fait** | Dérivé des maisons (cap = maisons × 4). |
| `habitations` | `state.maisons` | **Fait** | Nom différent (serde alias `habitations` en sauvegarde pour rétrocompat). |
| `moral` | `state.moral` | **Fait** | 0..1 ; affiché en % (bonheur). |
| `fecondite` | `state.fecondite` | **Fait** | |

**Champs en plus par rapport au Guide MVP (bâtiments, construction, Game Over) :**  
`maisons`, `caserne_lvl`, `grenier_lvl`, `depot_lvl`, `entrepot_lvl`, `macons`, `construction_maison` (+ paid), `construction_caserne/grenier/depot/entrepot`, `allocation_macons`, `jours_nourriture_zero`, `game_over`, `route_duree_by_pair`. Tous **implémentés** et alignés avec les docs Bâtiments / Zone Cité.

### 2.3 Affectation des gens (allocation)

| Spec | Code | Statut | Commentaire |
|------|------|--------|-------------|
| champs, ateliers, scierie, carriere, mine, recherche | `Allocation` (mêmes champs) | **Fait** | |
| forge | `Allocation.forge` | **Fait** | En plus du Guide MVP (production armes). |
| Contrainte total ≤ gens | `idlesim::apply_allocation` | **Fait** | |

### 2.4 Carte (MVP)

| Élément | Code | Statut | Commentaire |
|---------|------|--------|-------------|
| Cités (id, nom, x, y, proprietaire, troupes) | `Cite` | **Fait** | Pas de champ `niveau` (optionnel en spec). |
| Routes (cite_a, cite_b, duree_s) | `Route` | **Fait** | |
| Déplacements (from, to, troupes, progress, attaquant) | `Deplacement` | **Fait** | |
| Cache durée routes | `route_duree_by_pair` | **Fait** | Optimisation (évite recalcul à chaque tick). |

### 2.5 Métadonnées partie

| Champ | Code | Statut |
|-------|------|--------|
| temps_simule_s | `state.temps_simule_s` | **Fait** |
| slot_id | `state.slot_id` | **Fait** |
| version_sauvegarde | `state.version_sauvegarde` | **Fait** |

**Synthèse modèle :** Aligné sur le Guide MVP avec extensions (bâtiments, maçons, construction, Game Over, forge). Pas d’écart bloquant.

---

## 3. Mécaniques de jeu

| Mécanique | Spec | Statut | Commentaire |
|-----------|------|--------|-------------|
| **Clic** | Champs, Ateliers, Château, Village | **Fait** | + Bois, Pierre, Fer (gain +1 par clic). |
| **Tick (simulation)** | tick(state, delta), production/consommation, moral, fécondité | **Fait** | `idlesim::tick` ; production par lieu, consommation nourriture, moral/fecondite, Game Over 7j à 0 nourriture. |
| **Sauvegarde** | slot_write, slot_read, slot_list (JSON) | **Fait** | Fichiers par slot dans `data_dir` ; SlotMetadata (date, occupé). |
| **Combat** | resolve(attaquant, defenseur, stats) → vainqueur, troupes restantes | **Fait** | `combat::resolve` ; seed en paramètre (code utilise SystemTime pour seed en prod — non déterministe, voir Audit). |
| **Envoi troupes** | move_troops(from, to, count) | **Fait** | `carte::move_troops` ; déduction troupes source, création `Deplacement`. |
| **Avancement déplacements** | model_update(delta), resolve_arrival | **Fait** | `carte::model_update` ; à arrivée appel `combat::resolve` si cité adverse. |
| **Construction** | Coûts, pts par maçon, niveau bâtiments (Guide Bâtiments) | **Fait** | Coûts et pts dans `idlesim` ; paiement au clic « Construire » (maison) ou dans try_complete selon bâtiment ; allocation maçons. |
| **Guilde maçons** | 1 pop → 1 maçon | **Fait** | `convert_pop_to_macon` (idlesim). |

---

## 4. Specs additionnelles (Zone Cité, Bâtiments)

| Élément | Document | Statut | Commentaire |
|---------|----------|--------|-------------|
| Bouton construction (vert/blanc, paiement au clic) | Guide Zone Cité | **Fait** | Cartes bâtiments avec bouton Construire, conditions ressources, démarrage construction. |
| Zone cité (ciel 60 % / sol 40 %, min 200 px) | Guide Zone Cité | **Fait** | Rectangle ciel (bleu) + sol (vert) entre header et liste. |
| Sprites personnages 3×1 px, déplacement aléatoire | Guide Zone Cité | **Non fait** | Pas de sprites ni mouvement dans la zone cité. |
| Bâtiments (Maison, Caserne, Grenier, Dépôt, Entrepôt) | Bâtiments Maçons | **Fait** | Niveaux, caps (soldats, nourriture, matières, manufacturés), construction par maçons. |
| Game Over (7 jours à 0 nourriture) | Système Bonheur | **Fait** | `jours_nourriture_zero`, `game_over` ; overlay « Game Over » + retour Slots. |

---

## 5. APIs Toolkits / Opérateurs (alignement code)

Les « Tools » du Guide MVP sont implémentés en **fonctions de module** (pas de couche Opérateur séparée) :

| API spec | Implémentation | Fichier |
|----------|----------------|---------|
| idlesim.tick | `idlesim::tick` | idlesim.rs |
| idlesim.apply_allocation | `idlesim::apply_allocation` | idlesim.rs |
| idlesim.apply_click | `idlesim::apply_click` | idlesim.rs |
| save.slot_write / slot_read / slot_list | `save::slot_write`, `slot_read`, `slot_list` | save.rs |
| combat.resolve | `combat::resolve` | combat.rs |
| carte.move_troops | `carte::move_troops` | carte.rs |
| carte.model_update | `carte::model_update` | carte.rs |

**MiyuClickerSprites (Toolkit) :** Non implémenté en tant que Toolkit (pas de `load`, `frame_rect`, `animate`). Chemins UI dans `ui_assets.rs` ; pas de cache textures/spritesheets centralisé dans le crate.

---

## 6. Synthèse globale

| Domaine | Statut global | Prochaine étape suggérée |
|---------|----------------|---------------------------|
| Écrans MVP | **Fait** | Polish (résolution, langue) si besoin. |
| Modèle d’état | **Fait** (avec extensions) | Tenir à jour ce doc si ajout de champs. |
| Mécaniques (clic, tick, save, combat, carte) | **Fait** | Voir Audit (perf, DRY, testabilité). |
| Zone Cité (sprites + mouvement) | **Partiel** | Implémenter sprites 3×1 et déplacement aléatoire si priorité. |
| MiyuClickerSprites Toolkit | **Non fait** | Optionnel MVP ; utile pour Phase 4 polish. |

---

**Dernière mise à jour :** 2026-02-02  
**Statut :** Document de référence — état des lieux code vs spec
