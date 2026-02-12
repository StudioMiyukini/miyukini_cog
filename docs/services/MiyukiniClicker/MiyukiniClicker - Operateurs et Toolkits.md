# MiyuClicker — Opérateurs et Toolkits (mapping MVP)

## Contexte

Ce document décrit le **mapping Opérateurs / Toolkits** pour le MVP MiyuClicker : quels **Toolkits** (Kits d’Outils) et **Outils** (Tools) sont **utilisés** ou **créés**, et par quels **Opérateurs**. Il complète le [MVP Écrans et Mécaniques](MiyuClicker%20-%20MVP%20Ecrans%20et%20Mecaniques.md).

## Portée / Scope

- **Périmètre :** Inventaire des Toolkits (existants ou à créer), Outils composants, Opérateurs consommateurs, flux UI → Sim / Save / Carte.
- **Hors périmètre :** Implémentation technique détaillée (crates, API), contrats de gouvernance formels.

---

## 1. Vue d’ensemble

| Opérateur | Type | Toolkits consommés | Rôle MVP |
|-----------|------|--------------------|----------|
| **MiyuClickerUI** | Opérateur d’Interface | Dioxus, MiyuClickerSprites, MiyuClickerCarte (rendu) | Rendu de tous les écrans, barre, boutons, liste déroulante. |
| **MiyuClickerSim** | Opérateur de Service | MiyuClickerIdleSim | Tick simulation (ressources, affectations, moral, cap). |
| **MiyuClickerSave** | Opérateur de Service | MiyuClickerSave (Tools) | Sauvegarde / chargement 3 slots. |
| **MiyuClickerCombat** | Opérateur de Service / Tool | MiyuClickerCombat (Tool) | Résolution combat (stats + hasard). |
| **MiyuClickerCarte** | Opérateur de Service | MiyuClickerCarte, MiyuClickerCombat | Modèle carte, déplacements, combats. |

---

## 2. Toolkits — Utiliser

### 2.1 Stack UI (Dioxus)

| Attribut | Détail |
|----------|--------|
| **Source** | Stack UI officielle Miyukini. |
| **Référence** | [Miyukini - Stack UI Dioxus](../../ux_ui/Miyukini%20-%20Stack%20UI%20Dioxus.md). |
| **Rôle** | Fenêtres, panels, boutons, labels, sliders, listes, custom painting (carte). |
| **Consommé par** | **MiyuClickerUI**. |
| **Usage MVP** | Tous les écrans (Loading, Landing, Slots, Ma citée, Carte du monde) ; barre 2 lignes ; 4 gros boutons ; liste déroulante ; menu config. |

---

## 3. Toolkits — Créer (MVP)

### 3.1 MiyuClickerSprites

| Attribut | Détail |
|----------|--------|
| **ToolkitId (proposé)** | `toolkit.miyuclicker.sprites` |
| **Rôle** | Chargement d’images, spritesheets, cache textures, découpage en frames, animation par frame. |
| **Outils (Tools) proposés** | `tool.miyuclicker.sprites.load`, `tool.miyuclicker.sprites.frame_rect`, `tool.miyuclicker.sprites.animate` (avancement delta → frame index). |
| **Consommé par** | **MiyuClickerUI**. |
| **Sources assets** | `ui/game_ui_pack` (Cute_Fantasy_UI, Cute_Fantasy, etc.) — voir [Reference Packs UI Jeux](MiyuClicker%20-%20Reference%20Packs%20UI%20Jeux.md). |

### 3.2 MiyuClickerIdleSim

| Attribut | Détail |
|----------|--------|
| **ToolkitId (proposé)** | `toolkit.miyuclicker.idlesim` |
| **Rôle** | Simulation tick : mise à jour des ressources (production, consommation), moral, fécondité, cap gens, affectations. |
| **Outils (Tools) proposés** | `tool.miyuclicker.idlesim.tick` (état + delta → nouvel état), `tool.miyuclicker.idlesim.apply_allocation` (affectation gens → production par tick). |
| **Consommé par** | **MiyuClickerSim** (Opérateur). |
| **Données** | État du jeu (ressources, gens, soldats, affectations, habitations, moral, etc.) fourni dans le flux ; pas d’accès direct à la persistance. |

### 3.3 MiyuClickerSave

| Attribut | Détail |
|----------|--------|
| **ToolkitId (proposé)** | `toolkit.miyuclicker.save` |
| **Rôle** | Sérialisation / désérialisation de l’état partie ; lecture / écriture des 3 slots (sauvegarde fichier JSON via serde + I/O). |
| **Outils (Tools) proposés** | `tool.miyuclicker.save.slot_write` (slot_id, état), `tool.miyuclicker.save.slot_read` (slot_id → état), `tool.miyuclicker.save.slot_list` (→ métadonnées 3 slots : date, résumé). |
| **Consommé par** | **MiyuClickerSave** (Opérateur), **MiyuClickerUI** (affichage slots, déclenchement sauvegarde/chargement). |

### 3.4 MiyuClickerCombat

| Attribut | Détail |
|----------|--------|
| **Identifiant** | Peut être un **Tool** unique plutôt qu’un Toolkit (une seule capacité : résolution combat). |
| **ToolId (proposé)** | `tool.miyuclicker.combat.resolve` |
| **Rôle** | Résolution d’un combat : attaquant (nombre, stats), défenseur (nombre, stats), hasard → vainqueur, troupes restantes (attaquant, défenseur). |
| **Consommé par** | **MiyuClickerCarte** (Opérateur) — appelé à l’arrivée des troupes sur une cité adverse. |

### 3.5 MiyuClickerCarte

| Attribut | Détail |
|----------|--------|
| **ToolkitId (proposé)** | `toolkit.miyuclicker.carte` |
| **Rôle** | Modèle de la carte (nœuds = cités, arêtes = routes) ; déplacements en cours (from, to, progress) ; rendu (éléments SVG/canvas Dioxus) ; hit-test (clic → cité). |
| **Outils (Tools) proposés** | `tool.miyuclicker.carte.model_update` (déplacements, conquêtes), `tool.miyuclicker.carte.move_troops` (cité_from, cité_to, nombre), `tool.miyuclicker.carte.resolve_arrival` (déplacement arrivé → déclenche combat si cité adverse). |
| **Consommé par** | **MiyuClickerUI** (rendu, clic), **MiyuClickerCarte** (Opérateur) pour la logique déplacements et combats. |

---

## 4. Opérateurs — Détail

### 4.1 MiyuClickerUI (Opérateur d’Interface)

| Attribut | Détail |
|----------|--------|
| **Type** | Opérateur d’Interface |
| **Rôle** | Rendu de tous les écrans ; réception des entrées utilisateur ; envoi des **intentions** vers les autres Opérateurs (Sim, Save, Carte). |
| **Toolkits** | Dioxus (stack), MiyuClickerSprites (assets), MiyuClickerCarte (rendu carte). |
| **Écrans** | Loading, Landing, Slots, Ma citée (barre + 4 boutons + liste affectation), Carte du monde, menu config (roue). |
| **Flux sortants** | Clic Champs/Ateliers/Château/Village → intention « gain clic » → Sim ; affectation gens → intention « allocation » → Sim ; sauvegarder / charger → Save ; envoyer troupes → Carte ; tick (timer) → Sim. |

### 4.2 MiyuClickerSim (Opérateur de Service)

| Attribut | Détail |
|----------|--------|
| **Type** | Opérateur de Service |
| **Rôle** | Exécution du **tick** de simulation : mise à jour des ressources, consommation nourriture, production selon affectations, moral, cap gens. |
| **Toolkits** | MiyuClickerIdleSim |
| **Entrées** | État courant + delta temps ; éventuellement intention « gain clic » (Champs, Ateliers, etc.) ou « allocation » (affectation gens). |
| **Sorties** | Nouvel état (ressources, gens, soldats, moral, etc.) ; l’UI lit cet état pour affichage. |

### 4.3 MiyuClickerSave (Opérateur de Service)

| Attribut | Détail |
|----------|--------|
| **Type** | Opérateur de Service |
| **Rôle** | Sauvegarde et chargement des 3 slots ; fourniture des métadonnées (date, résumé) pour l’écran Slots. |
| **Toolkits** | MiyuClickerSave (Tools) |
| **Entrées** | Intention « sauvegarder slot N », « charger slot N », « lister slots ». |
| **Sorties** | État chargé (pour Sim / Carte) ; liste des métadonnées slots (pour UI). |

### 4.4 MiyuClickerCombat (Opérateur de Service / Tool)

| Attribut | Détail |
|----------|--------|
| **Type** | Opérateur de Service ou Tool unique |
| **Rôle** | Résolution d’un combat : attaquant vs défenseur → vainqueur, troupes restantes. |
| **Tool** | `tool.miyuclicker.combat.resolve` |
| **Consommé par** | MiyuClickerCarte (Opérateur) lors de l’arrivée des troupes sur une cité adverse. |

### 4.5 MiyuClickerCarte (Opérateur de Service)

| Attribut | Détail |
|----------|--------|
| **Type** | Opérateur de Service |
| **Rôle** | Gestion du **modèle** carte (cités, routes, propriété, troupes) ; déplacements en cours ; déclenchement des combats à l’arrivée ; mise à jour après combat (cité conquise, troupes restantes). |
| **Toolkits** | MiyuClickerCarte (modèle, déplacements), MiyuClickerCombat (résolution). |
| **Entrées** | Intention « envoyer X soldats de A vers B » ; tick (avancement des déplacements). |
| **Sorties** | Modèle carte à jour (pour rendu UI) ; événements « combat résolu », « cité conquise ». |

---

## 5. Service MiyuClicker — Agrégat

| Élément | Détail |
|--------|--------|
| **Service** | MiyuClicker |
| **Nature** | Équipe d’Opérateurs (ou agrégat) délivrant le **jeu** (capacité perçue par le joueur). |
| **Opérateurs** | MiyuClickerUI, MiyuClickerSim, MiyuClickerSave, MiyuClickerCombat, MiyuClickerCarte |
| **Contrat d’équipe (MVP)** | UI → Sim (tick, clic, allocation) ; UI → Save (sauvegarder, charger, lister) ; UI → Carte (envoyer troupes, afficher carte) ; Carte → Combat (résolution) ; Sim et Carte partagent l’état (ressources, cités, troupes) via flux ou état commun. |
| **Mandat de permission** | Pour une session de jeu, StrongFather peut émettre un mandat autorisant cette équipe à collaborer ; BondingBrother assure la médiation. Pour le MVP, implémentation simplifiée possible (appels directs sans COG complet). |

---

## 6. Synthèse — Création / utilisation

| Élément | Action | Référence |
|--------|--------|-----------|
| **Dioxus** | Utiliser | Stack UI Miyukini |
| **MiyuClickerSprites** | Créer | Toolkit + Tools load, frame_rect, animate |
| **MiyuClickerIdleSim** | Créer | Toolkit + Tools tick, apply_allocation |
| **MiyuClickerSave** | Créer | Toolkit + Tools slot_write, slot_read, slot_list |
| **MiyuClickerCombat** | Créer | Tool resolve |
| **MiyuClickerCarte** | Créer | Toolkit + Tools model_update, move_troops, resolve_arrival |
| **MiyuClickerUI** | Créer | Opérateur d’Interface |
| **MiyuClickerSim** | Créer | Opérateur de Service |
| **MiyuClickerSave (Opérateur)** | Créer | Opérateur de Service |
| **MiyuClickerCombat (Opérateur)** | Créer | Opérateur de Service / Tool |
| **MiyuClickerCarte (Opérateur)** | Créer | Opérateur de Service |
| **Service MiyuClicker** | Créer | Agrégat des Opérateurs ci-dessus |

---

## 7. Références

- [MiyuClicker - MVP Ecrans et Mecaniques](MiyuClicker%20-%20MVP%20Ecrans%20et%20Mecaniques.md)
- [MiyuClicker - Guide Implementation MVP](MiyuClicker%20-%20Guide%20Implementation%20MVP.md)
- [MiyuClicker - Document Fondateur](MiyuClicker%20-%20Document%20Fondateur.md)
- [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) (Opérateur, Toolkit, Service)

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-11  
**Statut :** Mapping Opérateurs et Toolkits pour le MVP MiyuClicker
