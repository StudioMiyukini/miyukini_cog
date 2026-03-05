# MiyuClicker â€” OpÃ©rateurs et Toolkits (mapping MVP)

## Contexte

Ce document dÃ©crit le **mapping OpÃ©rateurs / Toolkits** pour le MVP MiyuClicker : quels **Toolkits** (Kits dâ€™Outils) et **Outils** (Tools) sont **utilisÃ©s** ou **crÃ©Ã©s**, et par quels **OpÃ©rateurs**. Il complÃ¨te le [MVP Ã‰crans et MÃ©caniques](MiyukiniClicker%20-%20MVP%20Ecrans%20et%20Mecaniques.md).

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre :** Inventaire des Toolkits (existants ou Ã  crÃ©er), Outils composants, OpÃ©rateurs consommateurs, flux UI â†’ Sim / Save / Carte.
- **Hors pÃ©rimÃ¨tre :** ImplÃ©mentation technique dÃ©taillÃ©e (crates, API), contrats de gouvernance formels.

---

## 1. Vue dâ€™ensemble

| OpÃ©rateur | Type | Toolkits consommÃ©s | RÃ´le MVP |
|-----------|------|--------------------|----------|
| **MiyuClickerUI** | OpÃ©rateur dâ€™Interface | Dioxus, MiyuClickerSprites, MiyuClickerCarte (rendu) | Rendu de tous les Ã©crans, barre, boutons, liste dÃ©roulante. |
| **MiyuClickerSim** | OpÃ©rateur de Service | MiyuClickerIdleSim | Tick simulation (ressources, affectations, moral, cap). |
| **MiyuClickerSave** | OpÃ©rateur de Service | MiyuClickerSave (Tools) | Sauvegarde / chargement 3 slots. |
| **MiyuClickerCombat** | OpÃ©rateur de Service / Tool | MiyuClickerCombat (Tool) | RÃ©solution combat (stats + hasard). |
| **MiyuClickerCarte** | OpÃ©rateur de Service | MiyuClickerCarte, MiyuClickerCombat | ModÃ¨le carte, dÃ©placements, combats. |

---

## 2. Toolkits â€” Utiliser

### 2.1 Stack UI (Dioxus)

| Attribut | DÃ©tail |
|----------|--------|
| **Source** | Stack UI officielle Miyukini. |
| **RÃ©fÃ©rence** | [Miyukini - Stack UI Dioxus](..//..//_index.md). |
| **RÃ´le** | FenÃªtres, panels, boutons, labels, sliders, listes, custom painting (carte). |
| **ConsommÃ© par** | **MiyuClickerUI**. |
| **Usage MVP** | Tous les Ã©crans (Loading, Landing, Slots, Ma citÃ©e, Carte du monde) ; barre 2 lignes ; 4 gros boutons ; liste dÃ©roulante ; menu config. |

---

## 3. Toolkits â€” CrÃ©er (MVP)

### 3.1 MiyuClickerSprites

| Attribut | DÃ©tail |
|----------|--------|
| **ToolkitId (proposÃ©)** | `toolkit.miyuclicker.sprites` |
| **RÃ´le** | Chargement dâ€™images, spritesheets, cache textures, dÃ©coupage en frames, animation par frame. |
| **Outils (Tools) proposÃ©s** | `tool.miyuclicker.sprites.load`, `tool.miyuclicker.sprites.frame_rect`, `tool.miyuclicker.sprites.animate` (avancement delta â†’ frame index). |
| **ConsommÃ© par** | **MiyuClickerUI**. |
| **Sources assets** | `ui/game_ui_pack` (Cute_Fantasy_UI, Cute_Fantasy, etc.) â€” voir [Reference Packs UI Jeux](MiyukiniClicker%20-%20Reference%20Packs%20UI%20Jeux.md). |

### 3.2 MiyuClickerIdleSim

| Attribut | DÃ©tail |
|----------|--------|
| **ToolkitId (proposÃ©)** | `toolkit.miyuclicker.idlesim` |
| **RÃ´le** | Simulation tick : mise Ã  jour des ressources (production, consommation), moral, fÃ©conditÃ©, cap gens, affectations. |
| **Outils (Tools) proposÃ©s** | `tool.miyuclicker.idlesim.tick` (Ã©tat + delta â†’ nouvel Ã©tat), `tool.miyuclicker.idlesim.apply_allocation` (affectation gens â†’ production par tick). |
| **ConsommÃ© par** | **MiyuClickerSim** (OpÃ©rateur). |
| **DonnÃ©es** | Ã‰tat du jeu (ressources, gens, soldats, affectations, habitations, moral, etc.) fourni dans le flux ; pas dâ€™accÃ¨s direct Ã  la persistance. |

### 3.3 MiyuClickerSave

| Attribut | DÃ©tail |
|----------|--------|
| **ToolkitId (proposÃ©)** | `toolkit.miyuclicker.save` |
| **RÃ´le** | SÃ©rialisation / dÃ©sÃ©rialisation de lâ€™Ã©tat partie ; lecture / Ã©criture des 3 slots (sauvegarde fichier JSON via serde + I/O). |
| **Outils (Tools) proposÃ©s** | `tool.miyuclicker.save.slot_write` (slot_id, Ã©tat), `tool.miyuclicker.save.slot_read` (slot_id â†’ Ã©tat), `tool.miyuclicker.save.slot_list` (â†’ mÃ©tadonnÃ©es 3 slots : date, rÃ©sumÃ©). |
| **ConsommÃ© par** | **MiyuClickerSave** (OpÃ©rateur), **MiyuClickerUI** (affichage slots, dÃ©clenchement sauvegarde/chargement). |

### 3.4 MiyuClickerCombat

| Attribut | DÃ©tail |
|----------|--------|
| **Identifiant** | Peut Ãªtre un **Tool** unique plutÃ´t quâ€™un Toolkit (une seule capacitÃ© : rÃ©solution combat). |
| **ToolId (proposÃ©)** | `tool.miyuclicker.combat.resolve` |
| **RÃ´le** | RÃ©solution dâ€™un combat : attaquant (nombre, stats), dÃ©fenseur (nombre, stats), hasard â†’ vainqueur, troupes restantes (attaquant, dÃ©fenseur). |
| **ConsommÃ© par** | **MiyuClickerCarte** (OpÃ©rateur) â€” appelÃ© Ã  lâ€™arrivÃ©e des troupes sur une citÃ© adverse. |

### 3.5 MiyuClickerCarte

| Attribut | DÃ©tail |
|----------|--------|
| **ToolkitId (proposÃ©)** | `toolkit.miyuclicker.carte` |
| **RÃ´le** | ModÃ¨le de la carte (nÅ“uds = citÃ©s, arÃªtes = routes) ; dÃ©placements en cours (from, to, progress) ; rendu (Ã©lÃ©ments SVG/canvas Dioxus) ; hit-test (clic â†’ citÃ©). |
| **Outils (Tools) proposÃ©s** | `tool.miyuclicker.carte.model_update` (dÃ©placements, conquÃªtes), `tool.miyuclicker.carte.move_troops` (citÃ©_from, citÃ©_to, nombre), `tool.miyuclicker.carte.resolve_arrival` (dÃ©placement arrivÃ© â†’ dÃ©clenche combat si citÃ© adverse). |
| **ConsommÃ© par** | **MiyuClickerUI** (rendu, clic), **MiyuClickerCarte** (OpÃ©rateur) pour la logique dÃ©placements et combats. |

---

## 4. OpÃ©rateurs â€” DÃ©tail

### 4.1 MiyuClickerUI (OpÃ©rateur dâ€™Interface)

| Attribut | DÃ©tail |
|----------|--------|
| **Type** | OpÃ©rateur dâ€™Interface |
| **RÃ´le** | Rendu de tous les Ã©crans ; rÃ©ception des entrÃ©es utilisateur ; envoi des **intentions** vers les autres OpÃ©rateurs (Sim, Save, Carte). |
| **Toolkits** | Dioxus (stack), MiyuClickerSprites (assets), MiyuClickerCarte (rendu carte). |
| **Ã‰crans** | Loading, Landing, Slots, Ma citÃ©e (barre + 4 boutons + liste affectation), Carte du monde, menu config (roue). |
| **Flux sortants** | Clic Champs/Ateliers/ChÃ¢teau/Village â†’ intention Â« gain clic Â» â†’ Sim ; affectation gens â†’ intention Â« allocation Â» â†’ Sim ; sauvegarder / charger â†’ Save ; envoyer troupes â†’ Carte ; tick (timer) â†’ Sim. |

### 4.2 MiyuClickerSim (OpÃ©rateur de Service)

| Attribut | DÃ©tail |
|----------|--------|
| **Type** | OpÃ©rateur de Service |
| **RÃ´le** | ExÃ©cution du **tick** de simulation : mise Ã  jour des ressources, consommation nourriture, production selon affectations, moral, cap gens. |
| **Toolkits** | MiyuClickerIdleSim |
| **EntrÃ©es** | Ã‰tat courant + delta temps ; Ã©ventuellement intention Â« gain clic Â» (Champs, Ateliers, etc.) ou Â« allocation Â» (affectation gens). |
| **Sorties** | Nouvel Ã©tat (ressources, gens, soldats, moral, etc.) ; lâ€™UI lit cet Ã©tat pour affichage. |

### 4.3 MiyuClickerSave (OpÃ©rateur de Service)

| Attribut | DÃ©tail |
|----------|--------|
| **Type** | OpÃ©rateur de Service |
| **RÃ´le** | Sauvegarde et chargement des 3 slots ; fourniture des mÃ©tadonnÃ©es (date, rÃ©sumÃ©) pour lâ€™Ã©cran Slots. |
| **Toolkits** | MiyuClickerSave (Tools) |
| **EntrÃ©es** | Intention Â« sauvegarder slot N Â», Â« charger slot N Â», Â« lister slots Â». |
| **Sorties** | Ã‰tat chargÃ© (pour Sim / Carte) ; liste des mÃ©tadonnÃ©es slots (pour UI). |

### 4.4 MiyuClickerCombat (OpÃ©rateur de Service / Tool)

| Attribut | DÃ©tail |
|----------|--------|
| **Type** | OpÃ©rateur de Service ou Tool unique |
| **RÃ´le** | RÃ©solution dâ€™un combat : attaquant vs dÃ©fenseur â†’ vainqueur, troupes restantes. |
| **Tool** | `tool.miyuclicker.combat.resolve` |
| **ConsommÃ© par** | MiyuClickerCarte (OpÃ©rateur) lors de lâ€™arrivÃ©e des troupes sur une citÃ© adverse. |

### 4.5 MiyuClickerCarte (OpÃ©rateur de Service)

| Attribut | DÃ©tail |
|----------|--------|
| **Type** | OpÃ©rateur de Service |
| **RÃ´le** | Gestion du **modÃ¨le** carte (citÃ©s, routes, propriÃ©tÃ©, troupes) ; dÃ©placements en cours ; dÃ©clenchement des combats Ã  lâ€™arrivÃ©e ; mise Ã  jour aprÃ¨s combat (citÃ© conquise, troupes restantes). |
| **Toolkits** | MiyuClickerCarte (modÃ¨le, dÃ©placements), MiyuClickerCombat (rÃ©solution). |
| **EntrÃ©es** | Intention Â« envoyer X soldats de A vers B Â» ; tick (avancement des dÃ©placements). |
| **Sorties** | ModÃ¨le carte Ã  jour (pour rendu UI) ; Ã©vÃ©nements Â« combat rÃ©solu Â», Â« citÃ© conquise Â». |

---

## 5. Service MiyuClicker â€” AgrÃ©gat

| Ã‰lÃ©ment | DÃ©tail |
|--------|--------|
| **Service** | MiyuClicker |
| **Nature** | Ã‰quipe dâ€™OpÃ©rateurs (ou agrÃ©gat) dÃ©livrant le **jeu** (capacitÃ© perÃ§ue par le joueur). |
| **OpÃ©rateurs** | MiyuClickerUI, MiyuClickerSim, MiyuClickerSave, MiyuClickerCombat, MiyuClickerCarte |
| **Contrat dâ€™Ã©quipe (MVP)** | UI â†’ Sim (tick, clic, allocation) ; UI â†’ Save (sauvegarder, charger, lister) ; UI â†’ Carte (envoyer troupes, afficher carte) ; Carte â†’ Combat (rÃ©solution) ; Sim et Carte partagent lâ€™Ã©tat (ressources, citÃ©s, troupes) via flux ou Ã©tat commun. |
| **Mandat de permission** | Pour une session de jeu, StrongFather peut Ã©mettre un mandat autorisant cette Ã©quipe Ã  collaborer ; BondingBrother assure la mÃ©diation. Pour le MVP, implÃ©mentation simplifiÃ©e possible (appels directs sans COG complet). |

---

## 6. SynthÃ¨se â€” CrÃ©ation / utilisation

| Ã‰lÃ©ment | Action | RÃ©fÃ©rence |
|--------|--------|-----------|
| **Dioxus** | Utiliser | Stack UI Miyukini |
| **MiyuClickerSprites** | CrÃ©er | Toolkit + Tools load, frame_rect, animate |
| **MiyuClickerIdleSim** | CrÃ©er | Toolkit + Tools tick, apply_allocation |
| **MiyuClickerSave** | CrÃ©er | Toolkit + Tools slot_write, slot_read, slot_list |
| **MiyuClickerCombat** | CrÃ©er | Tool resolve |
| **MiyuClickerCarte** | CrÃ©er | Toolkit + Tools model_update, move_troops, resolve_arrival |
| **MiyuClickerUI** | CrÃ©er | OpÃ©rateur dâ€™Interface |
| **MiyuClickerSim** | CrÃ©er | OpÃ©rateur de Service |
| **MiyuClickerSave (OpÃ©rateur)** | CrÃ©er | OpÃ©rateur de Service |
| **MiyuClickerCombat (OpÃ©rateur)** | CrÃ©er | OpÃ©rateur de Service / Tool |
| **MiyuClickerCarte (OpÃ©rateur)** | CrÃ©er | OpÃ©rateur de Service |
| **Service MiyuClicker** | CrÃ©er | AgrÃ©gat des OpÃ©rateurs ci-dessus |

---

## 7. RÃ©fÃ©rences

- [MiyuClicker - MVP Ecrans et Mecaniques](MiyukiniClicker%20-%20MVP%20Ecrans%20et%20Mecaniques.md)
- [MiyuClicker - Guide Implementation MVP](MiyukiniClicker%20-%20Guide%20Implementation%20MVP.md)
- [MiyuClicker - Document Fondateur](MiyukiniClicker%20-%20Document%20Fondateur.md)
- [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) (OpÃ©rateur, Toolkit, Service)

---

**Document crÃ©Ã© le :** 2026-02-01  
**DerniÃ¨re mise Ã  jour :** 2026-02-11  
**Statut :** Mapping OpÃ©rateurs et Toolkits pour le MVP MiyuClicker


