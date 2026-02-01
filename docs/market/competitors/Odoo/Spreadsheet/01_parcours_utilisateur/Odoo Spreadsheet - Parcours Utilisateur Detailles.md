# Odoo Spreadsheet — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **Spreadsheet** d'Odoo (intégrée à Odoo Documents), identifiant les personas, scénarios d'usage, processus d'onboarding et points de friction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0 (Productivity / Spreadsheet)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs
- Parcours d'onboarding
- Scénarios d'usage principaux
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles Utilisateurs

### 1.1 Analyste / Contrôleur (Editor)

**Profil :**
- Rôle : Création et édition de classeurs, rapports, tableaux de bord
- Responsabilités :
  - Créer des classeurs (vierges ou à partir de templates)
  - Insérer des listes, pivots et graphiques depuis les apps Odoo
  - Utiliser les formules et fonctions Odoo
  - Configurer filtres globaux et mise en forme
  - Partager et gérer les droits (Viewer/Editor)
  - Sauvegarder en template, gérer l’historique des versions
  - Convertir un classeur en dashboard

**Besoins :**
- Accès à toutes les vues liste/pivot/graph des apps installées
- Formules Odoo (listes, pivots, comptabilité) documentées et stables
- Rafraîchissement des données fiable
- Raccourcis clavier (ex. Ctrl+K pour la palette de commandes)
- Paramètres régionaux par classeur
- Export .xlsx pour diffusion hors Odoo

**Permissions :**
- Droits Editor sur le classeur (ou sur le dossier Documents)
- Droits de lecture sur les modèles utilisés dans les sources de données

### 1.2 Lecteur / Direction (Viewer)

**Profil :**
- Rôle : Consultation des rapports et tableaux de bord
- Responsabilités :
  - Ouvrir les classeurs partagés
  - Consulter les données à jour (listes, pivots, graphiques)
  - Télécharger en .xlsx si nécessaire
  - Suivre les tableaux de bord dérivés des classeurs

**Besoins :**
- Interface claire, pas d’édition accidentelle
- Données rafraîchies à l’ouverture
- Export .xlsx pour archivage ou présentation

**Permissions :**
- Droits Viewer sur le classeur (ou hérités du dossier)

### 1.3 Administrateur / Power User

**Profil :**
- Gestion des templates (Documents ‣ Configuration ‣ Spreadsheet Templates)
- Création de templates d’entreprise (budget, commissions, KPIs)
- Configuration des dashboards à partir de classeurs
- Gestion des dossiers et partage au niveau Documents

**Besoins :**
- Création, copie, édition et suppression de templates
- Contrôle des groupes d’accès sur les dashboards
- Cohérence des paramètres régionaux sur les templates

---

## 2. Parcours d'Onboarding

### 2.1 Premier classeur

1. Ouvrir l’app **Documents**
2. Naviguer vers le dossier cible (ou « All »)
3. **New** → **Spreadsheet**
4. Choisir **Blank spreadsheet** ou un template → **Create**
5. Renommer le classeur (clic sur « Untitled spreadsheet »)
6. Optionnel : File ‣ Settings pour vérifier la locale

### 2.2 Première insertion de données Odoo

1. Ouvrir une **vue liste** (ex. Sales ‣ Quotations)
2. Adapter filtres/tri/colonnes si besoin
3. **Actions** (ou menu) → **Spreadsheet** → **Insert list in spreadsheet**
4. Saisir le nom de la liste, le nombre de lignes, choisir classeur existant ou « Blank spreadsheet » → **Confirm**
5. Le classeur s’ouvre avec une nouvelle feuille contenant la liste ; le panneau **Data** affiche la source (List #1)
6. Découverte des formules : `ODOO.LIST.HEADER`, `ODOO.LIST` dans les cellules

### 2.3 Utilisation d’un template

1. Documents → **New** → **Spreadsheet**
2. Choisir un template (ex. rapport mensuel) → **Create**
3. Adapter les données (filtres, champs) via les propriétés des sources dans le menu **Data**

---

## 3. Scénarios d'Usage Principaux

### 3.1 Rapport de ventes par commercial

- Insérer une **liste** ou un **pivot** depuis Sales (commandes ou lignes)
- Dimensions : commercial, période ; mesures : montant, quantité
- Ajouter graphiques (courbes, barres)
- Filtres globaux : période, région
- Partager en Viewer à la direction
- Option : convertir en dashboard pour suivi récurrent

### 3.2 Suivi budgétaire / comptabilité

- Utiliser les **fonctions Odoo** (ODOO.BALANCE, ODOO.CREDIT, ODOO.DEBIT, ODOO.FISCALYEAR.START/END)
- Construire un classeur avec comptes, périodes, écarts
- Template « Budget mensuel » réutilisable
- Locale cohérente (devise, dates)

### 3.3 Tableau de bord multi-sources

- Plusieurs feuilles : une par thème (ventes, stocks, projets)
- Listes et pivots issus de plusieurs apps (Sales, Inventory, Project)
- Filtres globaux communs (date, société)
- Liens cliquables vers menus Odoo (drill-down)
- File ‣ Add to dashboard pour exposition en tableau de bord

### 3.4 Import / adaptation de fichiers existants

- **Upload** d’un .xlsx ou .csv dans Documents
- **Open with Odoo Spreadsheet**
- Option « Send source file to trash » ou conserver l’original
- Enrichir avec listes/pivots Odoo ou formules Odoo

### 3.5 Collaboration et versions

- Partager le classeur (File ‣ Share) : utilisateurs ou lien
- « Freeze and share » si le lecteur n’a pas les droits sur les données sous-jacentes
- File ‣ See version history : consulter, restaurer ou copier une version
- Nommer des versions pour les jalons importants

---

## 4. Points de Friction Identifiés

### 4.1 Données

- **Liste à taille fixe :** les nouvelles lignes en base ne s’affichent pas automatiquement ; il faut ajouter des lignes ou ré-insérer la liste.
- **Domain + filtres globaux :** double filtrage possible ; il faut éviter de dupliquer les critères entre domain initial et filtre global.
- **Export .xlsx :** perte des formules Odoo (remplacées par les valeurs) — pas de réutilisation « dynamique » hors Odoo.

### 4.2 Templates

- Enregistrer comme template crée une copie figée ; modifier le template ensuite nécessite d’éditer le template depuis Configuration.
- Pas de notion de « template personnel » vs « template global » dans la doc standard.

### 4.3 Dashboards

- Après conversion, le classeur n’est plus dans Documents ; toute modification se fait via l’app Dashboards.
- Premier onglet = face du dashboard ; à anticiper lors de la conception.

### 4.4 Performance

- Gros volumes (listes/pivots avec beaucoup de lignes ou de dimensions) peuvent dégrader le temps d’ouverture et de rafraîchissement.
- Recommandation doc : préparer la vue (filtres, colonnes) avant insertion.

### 4.5 Locale

- Si la locale du classeur diffère du profil utilisateur, icône globe et message d’avertissement — peut surprendre en environnement multilingue.

---

## 5. Recommandations pour Miyukini

### 5.1 Opérateur Spreadsheet (MiyuSpreadsheet / MiyukiniSpreadsheet)

- **Création/édition de classeurs** gouvernée par StrongFather + KindMother (WriteIntent) ; persistance des métadonnées et du contenu (feuilles, sources, formules).
- **Sources de données** : contrat clair avec les Opérateurs exposant des vues (liste/pivot/graph) ; pas d’accès direct aux modèles sans Mandat.
- **Templates** : entités de premier ordre (création, versioning, partage) avec Ever Buddy pour cycle de vie.
- **Versions** : snapshots immuables ; restauration = nouvelle révision avec traçabilité (qui, quand, pourquoi si nommage).

### 5.2 Parcours utilisateur

- Onboarding guidé : premier classeur, première liste, premier filtre global.
- Rôles explicites : Viewer (lecture + export) / Editor (édition, partage, versions, templates).
- Documentation intégrée des formules « Miyukini » (équivalent ODOO.LIST, PIVOT, fonctions comptables).

### 5.3 UX

- Palette de commandes (type Ctrl+K) pour accès rapide aux actions.
- Indication claire de la locale du classeur et des écarts avec le profil.
- Avertissements si une source pointe vers un modèle auquel l’utilisateur n’a plus accès (données masquées ou erreur explicite).

### 5.4 Sécurité et gouvernance

- Niveau de sécurité selon criticité des données (listes comptables, RH, etc.) ; WorrySentinel pour bloquer ou dégrader si contexte dégradé.
- Mandats de permission pour « insérer une liste depuis l’opérateur X » et « utiliser les fonctions financières ».
- Audit : qui a créé/modifié/partagé quel classeur et quelle version.

---

**Document créé le :** 2026-02-01
