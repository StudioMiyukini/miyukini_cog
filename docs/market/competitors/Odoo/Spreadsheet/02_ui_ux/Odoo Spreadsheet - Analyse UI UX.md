# Odoo Spreadsheet — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Spreadsheet** d'Odoo (intégrée à Odoo Documents). Il identifie les composants d'interface, patterns de navigation, menus, panneaux et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0 (Productivity / Spreadsheet)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Structure générale de l’éditeur (grille, barre de formule, onglets)
- Menus et barre d’outils
- Panneaux (Data, propriétés, filtres globaux, paramètres, historique)
- Patterns d’interaction (insertion, édition, liens, partage)
- Raccourcis et accessibilité
- Recommandations pour Miyukini

**Hors scope :**
- Implémentation technique détaillée (guide d’implémentation)
- Logique métier (document dédié)

---

## 1. Structure Générale de l’Éditeur

### 1.1 Zone principale

- **Grille de cellules** : feuille active, cellules éditables, sélection, plages
- **Barre de formule** : affichage et édition de la formule de la cellule active
- **Onglets de feuilles** : en bas, nom de la feuille (ex. « Quotations by Total (List #1) »), ajout d’onglet, réorganisation, renommage
- **En-tête de colonnes / lignes** : repères A, B, C… et 1, 2, 3…

### 1.2 Barre de menu (menu bar)

- **File** : New, Share, Download (.xlsx), Print, Save as template, Move to trash, Add to dashboard, See version history, Settings
- **Edit** : Find and replace (Ctrl+H), Copy (Ctrl+C), etc.
- **Insert** : Listes, colonnes/lignes, liens (menu Odoo, feuille, URL)
- **Data** : Refresh all data, Re-insert list, gestion des sources et filtres globaux
- **View** : Affichage des formules (Show ‣ Formulas), etc.
- Raccourci global : **Ctrl+K** (ou **Command+K**) — **palette de commandes** pour parcourir et exécuter les commandes du tableur au clavier

### 1.3 Nom du classeur

- En haut : titre du classeur (ex. « Untitled spreadsheet ») — clic pour renommer

---

## 2. Menus et Actions Principales

### 2.1 File

| Action | Description |
|--------|-------------|
| New | Nouveau classeur (dans le même contexte Documents) |
| Share | Configurer accès (utilisateurs, lien), option « Freeze and share » |
| Download | Export .xlsx (formules Odoo → valeurs) |
| Print | Impression |
| Save as template | Enregistrer comme template (nom confirmé) |
| Move to trash | Déplacer vers la corbeille Documents |
| Add to dashboard | Créer un dashboard à partir du classeur |
| See version history | Ouvrir le panneau des versions |
| Settings | Ouvrir le panneau des paramètres (locale) |

### 2.2 Insert

- **List / Pivot / Chart** : selon le contexte (souvent initié depuis une vue Odoo)
- **Insert column/row** : colonne à gauche/droite, ligne au-dessus/en-dessous
- **Link** : Lien menu Odoo, feuille du classeur, ou URL ; libellé du lien

### 2.3 Data

- **Refresh all data** : Rafraîchir toutes les sources de données
- **Re-insert list** : Ré-insérer une liste existante (choix de la source, nombre de lignes, confirmation)
- **Data menu** : Liste des sources (List #1, Pivot #1, Chart #1…) avec icônes distinctes ; clic pour ouvrir le panneau de propriétés de la source
- **Filtres globaux** : Gestion et valeurs des filtres (panneau dédié)

### 2.4 View

- **Show ‣ Formulas** : Afficher les formules dans les cellules
- Autres options d’affichage (non détaillées dans la doc utilisateur)

---

## 3. Panneaux Latéraux

### 3.1 Panneau Data (sources de données)

- Liste des sources avec icône (liste / pivot / graphique), ID et nom (ex. « (#1) Sales Analysis by Product »)
- Clic sur une source → panneau de **propriétés** à droite
- **List** : List #, List Name, Model, Columns, Domain (Edit domain), Sorting ; engrenage → Duplicate / Delete
- **Pivot** : Pivot #, Name, Model, Columns, Rows, Measures, Domain ; Duplicate / Delete
- **Chart** : Configuration et design (onglets dans le panneau)
- Avertissement si une source n’a plus de liste/pivot visible dans le classeur (liste supprimée de la feuille mais source encore présente)
- **Pin** en haut du panneau pour garder ouvert tout en ouvrant un autre (ex. filtres globaux)

### 3.2 Panneau Filtres globaux

- Liste des filtres globaux du classeur
- Valeur courante par filtre (utilisée dans les formules via `ODOO.FILTER.VALUE(filter_name)`)
- Ouverture possible à côté du panneau Data (pin)

### 3.3 Panneau Paramètres (Settings)

- **Locale (regional settings)** : séparateurs milliers/décimales, formats date/heure, premier jour de la semaine
- Dropdown pour changer la locale du classeur
- Ouvert via File ‣ Settings

### 3.4 Panneau Version history

- Liste des versions (utilisateur, date/heure)
- Clic sur une version : affichage en lecture seule
- Actions par version : **Restore this version**, **Make a copy** (nouveau classeur)
- Nommage : clic sur la date/heure pour saisir un nom ; date/heure affichée sous le nom
- Ouvert via File ‣ See version history

### 3.5 Panneau Propriétés liste / pivot

- Accessible aussi par clic droit sur la liste/pivot : « See list properties » / « See pivot properties »
- Pour un graphique : icône menu en haut à droite du graphique → **Edit** → panneau de configuration (dont « Link to Odoo menu »)

---

## 4. Patterns d’Interaction

### 4.1 Insertion d’une liste depuis Odoo

1. Vue liste ouverte dans une app Odoo
2. **Actions** → **Spreadsheet** → **Insert list in spreadsheet**
3. Fenêtre : nom de la liste, nombre de lignes, choix du classeur (nouveau ou existant) → **Confirm**
4. Ouverture du classeur avec une nouvelle feuille ; panneau Data affiche la source List #n
5. Édition du domain / tri / colonnes depuis le panneau propriétés

### 4.2 Insertion pivot / graphique

- Même principe depuis une vue **pivot** ou **graph** : « Insert in Spreadsheet » → nom, classeur → **Confirm**
- Pivot : feuille dédiée ; Graphique : inséré sur la première feuille (documentation 19)

### 4.3 Ajout de lignes à une liste

- Sélection de la dernière ligne → poignée (carré bleu) → glisser vers le bas pour ajouter des lignes (formules copiées)
- Ou **Data** ‣ **Re-insert list** → choisir la liste, nombre de lignes → **Confirm** (réécrit la zone)

### 4.4 Ajout de colonnes à une liste

- Insérer une colonne (Insert ‣ Column left/right ou clic droit)
- Copier l’en-tête d’une colonne existante dans la nouvelle ; modifier le nom de champ dans la formule (sélecteur de champs du modèle)
- Étendre les formules vers le bas (double-clic sur la poignée de l’en-tête)

### 4.5 Liens cliquables

- **Depuis une cellule** : Insert ‣ Link (ou clic droit ‣ Insert link) → Lien menu Odoo / Feuille / URL + libellé
- **Depuis un graphique** : Edit du graphique → onglet Configuration → « Link to Odoo menu »

### 4.6 Accès aux enregistrements sous-jacents

- **Liste** : clic droit sur une cellule de la ligne → **See record**
- **Pivot** : clic droit sur une cellule → **See records**
- **Graphique** : clic sur un point de données → liste Odoo correspondante
- Clic milieu ou Ctrl+clic (Command+clic sur Mac) pour ouvrir dans un nouvel onglet

### 4.7 Dupliquer / supprimer une source

- **Liste / Pivot** : panneau propriétés → engrenage → **Duplicate** (nouvelle source, nouvel ID) ou **Delete** (supprime la source ; la table affichée peut rester en « valeurs » si déjà collée en « Paste as value »)
- **Graphique** : suppression du graphique dans la feuille = suppression de la source

---

## 5. Raccourcis et Accessibilité

- **Ctrl+K / Command+K** : palette de commandes (parcourir et exécuter les commandes au clavier)
- **Ctrl+H** : Find and replace
- **Ctrl+C** : Copy (y compris en vue version en lecture seule)
- **Icône globe** : affichée si la locale du classeur diffère de celle du profil utilisateur ; survol pour message d’avertissement
- Pas de détail dans la doc sur les raccourcis complets (référence à la doc « Keyboard shortcuts » Odoo)

---

## 6. Comportements Spécifiques

### 6.1 Viewer vs Editor

- **Viewer** : pas d’édition des cellules, pas de File ‣ Share / Save as template / See version history / Settings ; Download et consultation possibles
- **Editor** : toutes les actions d’édition, partage, templates, versions, paramètres

### 6.2 Freeze and share

- Lors du partage : option pour « figer » les données et partager le classeur à des utilisateurs n’ayant pas les droits sur les données Odoo sous-jacentes (affichage des valeurs sans lien actif)

### 6.3 Export .xlsx

- Les formules Odoo et non compatibles Excel sont remplacées par leurs valeurs au moment du téléchargement
- Viewer peut aussi télécharger

---

## 7. Recommandations pour Miyukini

### 7.1 Structure UI

- Conserver une structure proche : grille + barre de formule + onglets de feuilles + menu File / Edit / Insert / Data / View
- **Data** comme point central : sources, rafraîchissement, ré-insertion, filtres globaux
- Panneaux fermables et épinglables (Data, Filtres, Settings, Version history)

### 7.2 Cohérence avec l’écosystème

- Intégration dans un « Documents » ou « Fichiers » Miyukini : création, stockage, dossiers, partage
- Raccourci type Ctrl+K pour palette de commandes
- Indication claire du rôle (Viewer / Editor) et de la locale

### 7.3 Accessibilité et performance

- Raccourcis clavier documentés et cohérents
- Feedback visuel lors du rafraîchissement des données (loading)
- Messages explicites si une source n’est plus accessible (droits, modèle supprimé)

### 7.4 Partage et gouvernance

- Dialogue de partage aligné sur les Mandats et permissions Miyukini (StrongFather, Master Butler)
- Option « Freeze and share » avec traçabilité (qui a partagé, avec quelles options)

---

**Document créé le :** 2026-02-01
