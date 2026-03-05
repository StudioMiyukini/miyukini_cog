# MiyuClicker â€” Layout Lord of Click (blocs organiques et molÃ©cules)

## Contexte

Ce document dÃ©crit le **layout cible** de lâ€™Ã©cran de gestion **Lord of Click** (MiyuClicker) selon une hiÃ©rarchie visuelle claire :

- **Rectangles marron** = **blocs organiques** : zones structurelles, conteneurs de haut niveau (en-tÃªtes, zones de vision, bandes de notification).
- **Contours noirs** = **molÃ©cules ou atomes** : composants interactifs et fonctionnels (panneaux de ressources, liste de bÃ¢timents, champs, boutons, barres de progression).

Les textes sur le wireframe servent de **guide** pour nommer et placer les Ã©lÃ©ments.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre :** Structure du layout Ã©cran Â« Ma citÃ©e Â» ; mapping blocs organiques / molÃ©cules ; correspondance avec le code existant (`crates/miyuclicker`, `MiyuClickerService`).
- **Hors pÃ©rimÃ¨tre :** SpÃ©cifications de jeu (coÃ»ts, formules), autres Ã©crans (Loading, Landing, Slots, Carte du monde).

---

## 1. Convention de nommage

| ReprÃ©sentation | Terme | RÃ´le |
|----------------|--------|------|
| **Rectangle marron** (contour Ã©pais) | **Bloc organique** | Zone structurelle, conteneur ; dÃ©finit la hiÃ©rarchie et lâ€™emplacement des zones majeures. |
| **Contour noir** (fins ou Ã©pais) | **MolÃ©cule ou atome** | Composant interactif ou fonctionnel : panneau, liste, champ, bouton, barre de progression, scrollbar. |

---

## 2. Blocs organiques (structure)

De **haut en bas** :

### 2.1 Header Central

- **Bloc organique** en haut, pleine largeur.
- RÃ´le : barre supÃ©rieure globale (ressources principales, horloge, vitesses, navigation, configuration).
- **Correspondance code :** `ui_bar` (ligne 1 : Or, Gens, Soldats, MaÃ§ons, Recherche, Bonheur ; ligne 2 : Nourriture, Bois, Pierre, Fer, Outils, Armes ; Ã  droite : Ma citÃ©e, Carte du monde, âš™).

### 2.2 Header Lord of Click

- **Bloc organique** juste sous le Header Central.
- RÃ´le : en-tÃªte spÃ©cifique au jeu (titre Â« Lord of Click Â», Ã©ventuellement sous-navigation ou onglets).
- **Correspondance code :** Peut Ãªtre fusionnÃ© avec le Header Central ou dÃ©diÃ© (Ã  prÃ©ciser selon maquette).

### 2.3 Zone de vision de la citÃ©

- **Bloc organique** central, grande surface.
- RÃ´le : affichage de la reprÃ©sentation visuelle de la ville (ciel / sol, sprites, boutons de clic type Champs, Bois, etc.).
- **Correspondance code :** Zone allouÃ©e dans `ui_ma_citee_central_content` (rectangle ciel/terre + boutons de clic Ã  gauche).

### 2.4 Notification In Game en dÃ©filement

- **Bloc organique** en bande horizontale sous la zone de vision.
- RÃ´le : messages ou Ã©vÃ©nements du jeu qui dÃ©filent (annonces, gains, alertes).
- **Correspondance code :** Ã€ ajouter si souhaitÃ© (ex. bande dÃ©diÃ©e alimentÃ©e par `dev_log` ou un flux Â« Ã©vÃ©nements joueur Â»).

---

## 3. MolÃ©cules ou atomes (composants)

### 3.1 Panneau de ressources (colonne gauche)

- **Contour noir** : panneau vertical contenant les **blocs de ressources**.
- Chaque ressource = **atome** avec contour noir et libellÃ© type Â« +1 Or Â», Â« +1 Food Â», Â« +1 pop Â», Â« +1 Tech Â» (guide ; en jeu : Or, Nourriture, Gens/Pop, Recherche, etc.).
- **Correspondance code :** Les ressources sont actuellement dans `ui_bar` (en haut). Le wireframe suggÃ¨re une **colonne gauche** dÃ©diÃ©e aux indicateurs de gain (+1 / sec ou par clic) ; Ã  aligner avec lâ€™ergonomie cible (soit dÃ©placer une partie des ressources en colonne, soit dupliquer les indicateurs).

### 3.2 Panneau de gestion des bÃ¢timents (colonne droite)

- **Contour noir** : panneau englobant la liste des bÃ¢timents et la recherche.

#### 3.2.1 Recherche et filtrage des bÃ¢timents

- **Atome** : champ de saisie (contour noir), libellÃ© guide Â« Recherche et filtrage des batiments Â».
- **Correspondance code :** Ã€ ajouter (filtre texte sur la liste des bÃ¢timents).

#### 3.2.2 Liste des bÃ¢timents (Ã©lÃ©ments individuels)

- Chaque **ligne** = **molÃ©cule** avec contour noir, contenant :
  - **icÃ´ne** (carrÃ© contour noir),
  - **Lvl** (niveau),
  - **Nom du batiment**,
  - **Description**,
  - **nombre de maÃ§on** (ou Â« maÃ§ons Â»),
  - **CoÃ»t de construction / CoÃ»t en travail**,
  - **barre de pts de construction** (atome),
  - **Button Construire** (atome).
- **Correspondance code :** `ui_building_cards` (cartes Maison, Ferme, Scierie, CarriÃ¨re, Mine, Atelier, Forge) ; Ã  faire Ã©voluer vers une liste scrollable avec recherche/filtre et mise en forme type ligne (icÃ´ne, Lvl, nom, description, maÃ§ons, coÃ»t, barre, bouton).

#### 3.2.3 Barre de dÃ©filement

- **Atome** : scrollbar verticale Ã  droite de la liste.
- **Correspondance code :** DÃ©jÃ  prÃ©sent dans `CSS overflow-y: auto` sur le conteneur des cartes bÃ¢timents.

---

## 4. SchÃ©ma rÃ©capitulatif (ordre vertical)

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  BLOC ORGANIQUE : Header Central                                â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚  BLOC ORGANIQUE : header Lord of Click                           â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                  â”‚
â”‚  BLOC ORGANIQUE : zone de vision de la citÃ©                      â”‚
â”‚                                                                  â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚  BLOC ORGANIQUE : notification In Game en dÃ©filement            â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚  MOLÃ‰CULES  â”‚  MOLÃ‰CULES : Panneau bÃ¢timents                    â”‚
â”‚  Ressources â”‚  â”œâ”€ Recherche et filtrage (atome)                  â”‚
â”‚  +1 Or      â”‚  â”œâ”€ Ligne bÃ¢timent 1 (icÃ´ne, Lvl, nom, desc,      â”‚
â”‚  +1 Food    â”‚  â”‚    maÃ§ons, coÃ»t, barre, Button Construire)     â”‚
â”‚  +1 pop     â”‚  â”œâ”€ Ligne bÃ¢timent 2 â€¦                             â”‚
â”‚  +1 Tech    â”‚  â””â”€ Scrollbar (atome)                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 5. Correspondance avec le code actuel

| Ã‰lÃ©ment layout | Fichier / fonction | Statut |
|----------------|--------------------|--------|
| Header Central | `app.rs` â†’ `ui_bar` | PrÃ©sent (2 lignes ressources + nav + config) |
| Header Lord of Click | â€” | Optionnel ou fusionnÃ© |
| Zone de vision citÃ© | `ui_ma_citee_central_content` (rectangle ciel/sol + boutons clic) | PrÃ©sent |
| Notification dÃ©filement | â€” | Ã€ ajouter si requis |
| Panneau ressources (colonne) | Ressources dans `ui_bar` | Ã€ dÃ©placer ou dupliquer en colonne gauche selon maquette |
| Panneau bÃ¢timents | `ui_building_cards` | PrÃ©sent (cartes verticales) ; Ã  faire Ã©voluer en liste + recherche |
| Recherche / filtrage | â€” | Ã€ ajouter |
| Ligne bÃ¢timent (icÃ´ne, Lvl, nom, desc, maÃ§ons, coÃ»t, barre, bouton) | `ui_card_maison`, `ui_card_zone` | Partiel ; formaliser en ligne unique |
| Scrollbar liste bÃ¢timents | `CSS overflow-y: auto` | PrÃ©sent |

---

## 6. RÃ©fÃ©rences

- **Wireframe :** `images/references/Lord of click layout.jpg` (rectangles marron = blocs organiques, contours noirs = molÃ©cules/atomes).
- **Code :** `crates/miyuclicker/src/app.rs` (Ã©cran Ma citÃ©e, `ui_ma_citee`, `ui_bar`, `ui_building_cards`).
- **Ergonomie :** [MiyuClicker - Ergonomie Ecran Gestion](MiyukiniClicker%20-%20Ergonomie%20Ecran%20Gestion.md).
- **BÃ¢timents et maÃ§ons :** [MiyuClicker - Batiments Macons et Construction](MiyukiniClicker%20-%20Batiments%20Macons%20et%20Construction.md).

---

**Document :** MiyuClicker â€” Layout Lord of Click (blocs organiques et molÃ©cules)  
**Statut :** SpÃ©cification de layout (rÃ©fÃ©rence pour implÃ©mentation).

