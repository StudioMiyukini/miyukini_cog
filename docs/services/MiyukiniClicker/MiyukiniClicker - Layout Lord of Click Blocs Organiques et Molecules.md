# MiyuClicker — Layout Lord of Click (blocs organiques et molécules)

## Contexte

Ce document décrit le **layout cible** de l’écran de gestion **Lord of Click** (MiyuClicker) selon une hiérarchie visuelle claire :

- **Rectangles marron** = **blocs organiques** : zones structurelles, conteneurs de haut niveau (en-têtes, zones de vision, bandes de notification).
- **Contours noirs** = **molécules ou atomes** : composants interactifs et fonctionnels (panneaux de ressources, liste de bâtiments, champs, boutons, barres de progression).

Les textes sur le wireframe servent de **guide** pour nommer et placer les éléments.

## Portée / Scope

- **Périmètre :** Structure du layout écran « Ma citée » ; mapping blocs organiques / molécules ; correspondance avec le code existant (`crates/miyuclicker`, `MiyuClickerService`).
- **Hors périmètre :** Spécifications de jeu (coûts, formules), autres écrans (Loading, Landing, Slots, Carte du monde).

---

## 1. Convention de nommage

| Représentation | Terme | Rôle |
|----------------|--------|------|
| **Rectangle marron** (contour épais) | **Bloc organique** | Zone structurelle, conteneur ; définit la hiérarchie et l’emplacement des zones majeures. |
| **Contour noir** (fins ou épais) | **Molécule ou atome** | Composant interactif ou fonctionnel : panneau, liste, champ, bouton, barre de progression, scrollbar. |

---

## 2. Blocs organiques (structure)

De **haut en bas** :

### 2.1 Header Central

- **Bloc organique** en haut, pleine largeur.
- Rôle : barre supérieure globale (ressources principales, horloge, vitesses, navigation, configuration).
- **Correspondance code :** `ui_bar` (ligne 1 : Or, Gens, Soldats, Maçons, Recherche, Bonheur ; ligne 2 : Nourriture, Bois, Pierre, Fer, Outils, Armes ; à droite : Ma citée, Carte du monde, ⚙).

### 2.2 Header Lord of Click

- **Bloc organique** juste sous le Header Central.
- Rôle : en-tête spécifique au jeu (titre « Lord of Click », éventuellement sous-navigation ou onglets).
- **Correspondance code :** Peut être fusionné avec le Header Central ou dédié (à préciser selon maquette).

### 2.3 Zone de vision de la cité

- **Bloc organique** central, grande surface.
- Rôle : affichage de la représentation visuelle de la ville (ciel / sol, sprites, boutons de clic type Champs, Bois, etc.).
- **Correspondance code :** Zone allouée dans `ui_ma_citee_central_content` (rectangle ciel/terre + boutons de clic à gauche).

### 2.4 Notification In Game en défilement

- **Bloc organique** en bande horizontale sous la zone de vision.
- Rôle : messages ou événements du jeu qui défilent (annonces, gains, alertes).
- **Correspondance code :** À ajouter si souhaité (ex. bande dédiée alimentée par `dev_log` ou un flux « événements joueur »).

---

## 3. Molécules ou atomes (composants)

### 3.1 Panneau de ressources (colonne gauche)

- **Contour noir** : panneau vertical contenant les **blocs de ressources**.
- Chaque ressource = **atome** avec contour noir et libellé type « +1 Or », « +1 Food », « +1 pop », « +1 Tech » (guide ; en jeu : Or, Nourriture, Gens/Pop, Recherche, etc.).
- **Correspondance code :** Les ressources sont actuellement dans `ui_bar` (en haut). Le wireframe suggère une **colonne gauche** dédiée aux indicateurs de gain (+1 / sec ou par clic) ; à aligner avec l’ergonomie cible (soit déplacer une partie des ressources en colonne, soit dupliquer les indicateurs).

### 3.2 Panneau de gestion des bâtiments (colonne droite)

- **Contour noir** : panneau englobant la liste des bâtiments et la recherche.

#### 3.2.1 Recherche et filtrage des bâtiments

- **Atome** : champ de saisie (contour noir), libellé guide « Recherche et filtrage des batiments ».
- **Correspondance code :** À ajouter (filtre texte sur la liste des bâtiments).

#### 3.2.2 Liste des bâtiments (éléments individuels)

- Chaque **ligne** = **molécule** avec contour noir, contenant :
  - **icône** (carré contour noir),
  - **Lvl** (niveau),
  - **Nom du batiment**,
  - **Description**,
  - **nombre de maçon** (ou « maçons »),
  - **Coût de construction / Coût en travail**,
  - **barre de pts de construction** (atome),
  - **Button Construire** (atome).
- **Correspondance code :** `ui_building_cards` (cartes Maison, Ferme, Scierie, Carrière, Mine, Atelier, Forge) ; à faire évoluer vers une liste scrollable avec recherche/filtre et mise en forme type ligne (icône, Lvl, nom, description, maçons, coût, barre, bouton).

#### 3.2.3 Barre de défilement

- **Atome** : scrollbar verticale à droite de la liste.
- **Correspondance code :** Déjà présent dans `CSS overflow-y: auto` sur le conteneur des cartes bâtiments.

---

## 4. Schéma récapitulatif (ordre vertical)

```
┌─────────────────────────────────────────────────────────────────┐
│  BLOC ORGANIQUE : Header Central                                │
├─────────────────────────────────────────────────────────────────┤
│  BLOC ORGANIQUE : header Lord of Click                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  BLOC ORGANIQUE : zone de vision de la cité                      │
│                                                                  │
├─────────────────────────────────────────────────────────────────┤
│  BLOC ORGANIQUE : notification In Game en défilement            │
├──────────────┬──────────────────────────────────────────────────┤
│  MOLÉCULES  │  MOLÉCULES : Panneau bâtiments                    │
│  Ressources │  ├─ Recherche et filtrage (atome)                  │
│  +1 Or      │  ├─ Ligne bâtiment 1 (icône, Lvl, nom, desc,      │
│  +1 Food    │  │    maçons, coût, barre, Button Construire)     │
│  +1 pop     │  ├─ Ligne bâtiment 2 …                             │
│  +1 Tech    │  └─ Scrollbar (atome)                              │
└──────────────┴──────────────────────────────────────────────────┘
```

---

## 5. Correspondance avec le code actuel

| Élément layout | Fichier / fonction | Statut |
|----------------|--------------------|--------|
| Header Central | `app.rs` → `ui_bar` | Présent (2 lignes ressources + nav + config) |
| Header Lord of Click | — | Optionnel ou fusionné |
| Zone de vision cité | `ui_ma_citee_central_content` (rectangle ciel/sol + boutons clic) | Présent |
| Notification défilement | — | À ajouter si requis |
| Panneau ressources (colonne) | Ressources dans `ui_bar` | À déplacer ou dupliquer en colonne gauche selon maquette |
| Panneau bâtiments | `ui_building_cards` | Présent (cartes verticales) ; à faire évoluer en liste + recherche |
| Recherche / filtrage | — | À ajouter |
| Ligne bâtiment (icône, Lvl, nom, desc, maçons, coût, barre, bouton) | `ui_card_maison`, `ui_card_zone` | Partiel ; formaliser en ligne unique |
| Scrollbar liste bâtiments | `CSS overflow-y: auto` | Présent |

---

## 6. Références

- **Wireframe :** `images/references/Lord of click layout.jpg` (rectangles marron = blocs organiques, contours noirs = molécules/atomes).
- **Code :** `crates/miyuclicker/src/app.rs` (écran Ma citée, `ui_ma_citee`, `ui_bar`, `ui_building_cards`).
- **Ergonomie :** [MiyuClicker - Ergonomie Ecran Gestion](MiyuClicker%20-%20Ergonomie%20Ecran%20Gestion.md).
- **Bâtiments et maçons :** [MiyuClicker - Batiments Macons et Construction](MiyuClicker%20-%20Batiments%20Macons%20et%20Construction.md).

---

**Document :** MiyuClicker — Layout Lord of Click (blocs organiques et molécules)  
**Statut :** Spécification de layout (référence pour implémentation).
