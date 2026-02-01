# Odoo Documents — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Documents** d'Odoo (version 18/19), à partir de la documentation officielle. Il identifie les vues, patterns de navigation, composants et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18.0/19.0 — Productivity / Documents

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Structure générale (arbre + zone de contenu)
- Vues (liste, grille, prévisualisation, détails)
- Actions et menus (New, Actions, Share, Info & Tags)
- Patterns de navigation et raccourcis
- Chatter et panneau de détails
- Recommandations pour Miyukini

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Structure Générale

### 1.1 Layout principal

- **Gauche :** Arborescence (tree) des **sections** et **dossiers**
  - Sections : All, Company, My Drive, Shared with me, Recent, Trash
  - Dossiers et sous-dossiers développables/repliables
  - Clic sur une section ou un dossier pour afficher son contenu à droite
- **Droite :** Zone de contenu
  - Liste ou grille de dossiers/fichiers du contexte sélectionné
  - Barre d’outils : New, filtres, vue (liste/grille), recherche
- **Prévisualisation :** Clic sur un fichier → ouverture en overlay ou panneau (PDF, images, vidéos) avec barre d’actions (Action, Share, Download, etc.)
- **Fermeture prévisualisation :** Esc ou icône (close)

### 1.2 Glisser-déposer

- Fichier ou dossier peut être **glissé-déposé** vers un autre dossier ou une autre section pour le déplacer
- Upload par glisser-déposer d’un fichier depuis le poste vers un dossier de l’app

---

## 2. Arborescence (Tree)

### 2.1 Éléments affichés

- Icônes/types par section (All, Company, My Drive, etc.)
- Dossiers avec hiérarchie (indentation, expand/collapse)
- Favoris (étoile) sur dossiers — filtre « Starred » disponible
- Raccourcis (sous-dossiers ou fichiers) visibles dans l’arbre selon le contexte

### 2.2 Actions sur dossier (engrenage au-dessus de l’arbre)

- **Download** : Télécharger le dossier en .zip (fichiers et sous-dossiers)
- **Rename** : Renommer le dossier
- **Share** : Partager / gérer les droits d’accès
- **Add shortcut** : Créer un raccourci (sous-dossier uniquement)
- **Add star** : Marquer en favori
- **Info & Tags** : Ouvrir le panneau Détails + chatter
- **Move to trash** : Déplacer vers la corbeille
- **Actions on Select** : Choisir les actions serveur affichées pour les fichiers du dossier
- **Automations** : Créer des règles d’actions automatiques (Studio)

---

## 3. Zone de Contenu (Liste / Grille)

### 3.1 Barre supérieure

- **New** : Menu déroulant
  - Upload
  - Link (URL + nom + dossier)
  - Spreadsheet
  - Folder
  - Request (demande de document)
- **Recherche** : Barre de recherche (valeurs, recherche plein texte selon config Odoo)
- **Filtres** : Favoris (Starred), etc. ; filtres sauvegardés et partagés possibles
- **Vue** : Liste vs Grille (icônes de bascule)
- **Actions** (sur sélection) : Selon dossier (ex. Merge PDFs, taguer, déplacer)

### 3.2 Vue Liste

- Colonnes typiques : nom, type, taille, date modification, propriétaire, tags, etc.
- Sélection multiple : cases à cocher pour actions groupées (tag, déplacer, Merge PDFs, etc.)
- Tri par colonne
- Clic sur une ligne : ouvrir le fichier (prévisualisation) ou le dossier (navigation)

### 3.3 Vue Grille

- Cartes/tuiles par fichier ou dossier (icône, nom, métadonnées réduites)
- Même sélection multiple et actions groupées que en liste
- Clic : ouverture en prévisualisation ou entrée dans le dossier

### 3.4 Prévisualisation fichier

- **Zone centrale** : Rendu du fichier (PDF, image, vidéo, etc.) ou message pour types non prévisualisables
- **Barre supérieure** :
  - **Action** (menu) : Duplicate, Move to Trash, Rename, Info & tags, Create shortcut, Manage versions, Lock, Copy Links, Split PDF (si PDF)
  - **Share**
  - **Download**
  - Boutons définis au niveau du dossier (Actions on Select)
- **Raccourcis** : Shift+S (PDF) pour ajouter/supprimer toutes les coupures ; icône ciseaux entre pages pour Split PDF ; Delete pour supprimer une page sélectionnée

---

## 4. Panneau Détails (Info & Tags)

- Ouverture : bouton **Info & Tags** (en haut à droite à côté des icônes de vue) sur un fichier ou un dossier
- **Contenu :**
  - Dossier : nom (éditable), taille / nombre d’éléments, propriétaire, contact, alias email (avec domaine), type d’activité et assignation optionnels, tags appliqués aux fichiers reçus par alias
  - Fichier : dossier (changement possible), taille, propriétaire, contact, tags
- **Chatter** : Historique des messages et activités (discussion, activités planifiées)
- **Règle** : Pour qu’un utilisateur voie un fichier depuis son profil, il doit être défini comme **Contact** et avoir au moins Viewer

---

## 5. Partage (Share)

- **Déclencheur :** Bouton Share (fichier ouvert) ou Actions (engrenage) ‣ Share (dossier)
- **Fenêtre / pop-up** :
  - Ajout d’utilisateurs ou de contacts (dropdown ou saisie email)
  - Niveau : **Viewer** ou **Editor**
  - **Expiration** : date optionnelle (icône calendrier au survol du contact)
  - **Retrait** : icône supprimer au survol d’une ligne de permission
  - **Accès général** : Internal users / Anyone with the link → Viewer, Editor, ou None
  - Pour « Anyone with the link » : **Discoverable** ou **Must have the link to access**
- **Note** : Sur le portail, les utilisateurs publics doivent avoir le lien à la première connexion si « Must have the link » est activé

---

## 6. Demandes de documents (Request)

- **Création :** New ‣ Request
- **Formulaire** : Document Name, Request To (personne), Due Date In, Folder, Tags, Message
- **Bouton** : Request → création du placeholder et de l’activité
- **Suivi** : Vue Activités (Activity view), colonne **Requested Document** ; clic sur la date pour détail
- **Actions sur une demande** : Upload (remplacer le placeholder), Edit, Cancel, Send reminder (Preview puis Send Now)
- **Rappel groupé** : Menu (ellipsis) dans la colonne Requested Document ‣ Document Request: Reminder

---

## 7. Configuration (Paramètres et Tags)

- **Documents ‣ Configuration ‣ Paramètres**
  - Deletion delay (days)
  - File centralization : par app, choix du dossier et des tags ; sous-dossiers (ex. Comptabilité par journal)
- **Documents ‣ Configuration ‣ Tags**
  - Liste des tags : New, Tag Name, Color, Tooltip

---

## 8. Patterns de Navigation et Raccourcis

- **Recherche** : Barre de recherche pour retrouver rapidement dossiers/fichiers
- **Favoris (Starred)** : Filtre pour n’afficher que les dossiers marqués
- **Recent** : Accès rapide aux fichiers récemment modifiés
- **Esc** : Fermer la prévisualisation du fichier
- **Shift+S** (dans Split PDF) : Ajouter ou supprimer toutes les coupures entre pages
- **Drag & drop** : Déplacer fichier/dossier ; upload depuis le bureau

---

## 9. Recommandations pour Miyukini

- **Layout** : Conserver une arborescence à gauche (sections + dossiers) et zone de contenu à droite (liste/grille + prévisualisation)
- **Opérateur d’interface** : DocumentsUI exposant les vues (tree, list, grid, preview, details panel, share dialog) sans logique métier
- **Actions** : Toutes les actions (New, Share, Trash, Split/Merge PDF, Request, etc.) passent par BondingBrother vers les Opérateurs concernés avec Mandats
- **Chatter** : Réutiliser le pattern MiyuNotify / activités pour le panneau Info & Tags
- **Responsive** : Adapter la largeur de l’arbre et la disposition liste/grille sur petits écrans
- **Accessibilité** : Labels, contraste, navigation clavier (Esc, focus sur Share, Request, filtres)
- **Cohérence** : Aligner les libellés et le style sur le glossaire Miyukini (Mandat, Opérateur, etc.)

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
