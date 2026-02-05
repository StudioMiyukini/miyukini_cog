# Miyukini Survivor — Ecrans et UI

## Contexte

Ce document précise l'**affichage** du service Miyukini Survivor : un **seul écran** avec **overlays** et **fenêtres**, et le **layout** (barre haute, zone de jeu, sidebar droite). Il complète le [Document Fondateur](./Miyukini%20Survivor%20-%20Document%20Fondateur.md) et le [Gameplay et mécaniques](./Miyukini%20Survivor%20-%20Gameplay%20et%20Mecaniques.md).

## Portée / Scope

- **Périmètre :** Structure de l'écran, barres, zone de jeu, sidebar, fenêtres (Stats, Skills, Inventaire, Build, Recruit, construction), overlays.
- **Hors périmètre :** Maquettes visuelles détaillées, design system, implémentation technique.

---

## 1. Principe d'affichage

- **Un seul écran** : toute l'expérience de jeu se fait sur un même écran.
- **Overlays** et **fenêtres** : informations et actions complémentaires s'affichent en superposition (barre haute, sidebar, fenêtres modales ou panneaux).
- Les boutons **« Stats »**, **« Skills »**, **« Inventaire »**, **« Build »**, **« Recruit »** ouvrent des **fenêtres / frames** de type **Ultima Online** ou **Mortal Online** : panneaux indépendants, déplaçables (draggable), pouvant être ouverts, fermés et disposés sur l'écran (paper-doll, skills, inventaire, etc.), sans bloquer la zone de jeu.

---

## 2. Layout général

### 2.1 Barre en haut (informations de run)

- **À gauche :**
  - **Numéro de la vague** (phase Bataille).
  - **Nombre d'ennemis restants** avant la fin de la vague.
- **À droite :**
  - **Quantité d'or disponible**.
- **Juste en dessous** (sous cette barre ou intégré) :
  - **Barre d'XP** (progression vers le prochain level).

### 2.2 Barre de boutons (en haut)

- **Une autre barre en haut** regroupe les **boutons** permettant d'ouvrir des fenêtres :
  - **« Stats »** → fenêtre **Stats** (statistiques du joueur, Château, run).
  - **« Skills »** → fenêtre **Skills** (arbres de compétences, points à répartir).
  - **« Inventaire »** → fenêtre **Inventaire** (équipement, slots, objets portés).
  - **« Build »** → fenêtre **Build** (construction de tours / bâtiments ; peut recouper la sidebar ou en être l’ouverture).
  - **« Recruit »** → fenêtre **Recruit** (recrutement des troupes de soldats).
- Chaque bouton ouvre la **fenêtre / frame correspondante** (style UO / Mortal Online : panneau indépendant, déplaçable, réductible).

### 2.3 Zone de jeu (centre / plein écran)

- **Espace principal** où évoluent :
  - Le **joueur** (avatar, déplacement 8 directions).
  - Le **Château** (au centre).
  - Les **ennemis** (depuis les bords vers le Château).
  - Les **tours** (construites en phase Préparation).
- En phase Préparation :
  - **Grille** de placement des bâtiments.
  - **Disque vert** (opacité) centré sur le Château = **zone de construction**.
  - **Fantôme** du bâtiment (vert si constructible, rouge si impossible) collé au curseur lors de la construction.

### 2.4 Sidebar à droite

- **S'affiche pendant la phase de Préparation** (paix).
- Contenu :
  - **Liste de bâtiments** (tours) avec pour chaque bloc :
    - Bouton **« Construire »**
    - Bouton **« Info »**
    - **Nom du bâtiment**
    - **Coût en or**
    - **Courte description**
  - (Optionnel) accès à l'**équipement** (achats).
- **Bouton « Skills »** : ouvre la **fenêtre des compétences** (arbres de compétences, onglets, répartition des points).

---

## 3. Fenêtres et overlays

Les fenêtres **Stats**, **Skills**, **Inventaire**, **Build**, **Recruit** sont des **frames** type **Ultima Online** / **Mortal Online** : panneaux indépendants, déplaçables (draggable), ouvrables/fermables, disposables sur l'écran sans masquer la zone de jeu. Plusieurs fenêtres peuvent être ouvertes simultanément.

### 3.1 Fenêtre « Stats »

- Ouverte via le bouton **« Stats »** (barre de boutons en haut).
- Contenu : **statistiques** du joueur, du Château, de la run (PV, armure, dégâts, or, XP, etc.).

### 3.2 Fenêtre « Skills » (compétences)

- Ouverte via le bouton **« Skills »** (barre de boutons en haut ou sidebar).
- Contenu :
  - **Arbres de compétences** rangés par **onglet**.
  - **Points de compétences** à répartir (gagnés en level up).
  - Le joueur alloue ses points dans n'importe quel arbre **tant qu'il remplit les conditions** (prérequis).
- Affichage : **fenêtre** (modale ou panneau) par-dessus la zone de jeu ou à côté.’### 3.3 Fenêtre « Inventaire »

- Ouverte via le bouton **« Inventaire »** (barre de boutons en haut).
- Contenu : **slots d'équipement**, **objets portés**, **inventaire non occupé** (voir [Gameplay](./Miyukini%20Survivor%20-%20Gameplay%20et%20Mecaniques.md)).

### 3.4 Fenêtre « Build » (construction)

- Ouverte via le bouton **« Build »** (barre de boutons en haut).
- Contenu : **liste de bâtiments** (tours) avec Construire / Info, **grille de placement** ; peut recouper la sidebar Préparation.

### 3.5 Fenêtre « Recruit » (recrutement)

- Ouverte via le bouton **« Recruit »** (barre de boutons en haut).
- Contenu : **recrutement des troupes de soldats** (achat, renforts, gestion de la troupe).

### 3.6 Fenêtre « Info » bâtiment

- Ouverte via le bouton **« Info »** d'un bloc bâtiment (sidebar ou fenêtre Build).
- Affiche **toutes les métriques** du bâtiment (PV, dégâts, portée, effets, etc.).

### 3.7 Construction (fantôme + grille)

- Lors du **placement d’un bâtiment** (après « Construire ») :
  - **Grille de construction** visible dans la zone de jeu ; **cases de 20×20 px**.
  - **Fantôme** du bâtiment suivant le **curseur**.
  - **Couleur** : **vert** si constructible (dans la zone + conditions OK), **rouge** si non constructible.
  - **Clic** pour valider l’emplacement (si vert).

---

## 4. Résumé du layout

| Zone | Contenu | Visible |
|------|---------|--------|
| **Barre haute (infos)** | N° vague, ennemis restants (gauche) ; or (droite) ; barre d'XP (en dessous) | Toujours / selon phase |
| **Barre de boutons (haut)** | Boutons **Stats**, **Skills**, **Inventaire**, **Build**, **Recruit** | Toujours |
| **Centre / plein écran** | Barre d’XP | Toujours (ou selon phase) |
| **Centre / plein écran** | Zone de jeu (joueur, Château, ennemis, tours) ; en Préparation : grille + disque vert + fantôme | Toujours |
| **Sidebar droite** | Liste bâtiments (Construire, Info, nom, coût, description) ; bouton Skills | **Phase Préparation uniquement** |
| **Fenêtre Skills** | Arbres de compétences, onglets, points à répartir | Ouverture sur action |
| **Fenêtre Info** | Métriques complètes d’un bâtiment | Ouverture sur action |

---

## 5. Références

- [Miyukini Survivor - Document Fondateur](./Miyukini%20Survivor%20-%20Document%20Fondateur.md)
- [Miyukini Survivor - Gameplay et Mecaniques](./Miyukini%20Survivor%20-%20Gameplay%20et%20Mecaniques.md)

---

**Document créé le :** 2026-02-04  
**Dernière mise à jour :** 2026-02-04  
**Révision :** Barre de boutons en haut (Stats, Skills, Inventaire, Build, Recruit) ; fenêtres/frames type **Ultima Online** / **Mortal Online** (panneaux indépendants, déplaçables, ouvrables/fermables).
