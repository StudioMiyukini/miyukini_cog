# Odoo Spreadsheet — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Spreadsheet** d'Odoo (version 19.0), intégrée au module **Odoo Documents**. Il identifie les concepts de données, règles métier, sources de données, formules et mécanismes de liaison avec la base Odoo pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, module Spreadsheet (Productivity), intégré à Odoo Documents

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Concepts de données (classeur, feuille, cellule, source de données)
- Types de sources de données (liste, pivot, graphique)
- Formules et fonctions (standard + Odoo-specific)
- Règles de liaison aux modèles Odoo
- Gestion des templates
- Historique des versions
- Paramètres régionaux (locale)
- Conversion en tableau de bord (Dashboards)

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Concepts de Données

### 1.1 Classeur (Spreadsheet)

**Rôle :** Représente un **classeur** — document stocké dans Odoo Documents, contenant une ou plusieurs feuilles, des sources de données et des paramètres.

**Concepts clés :**
- **Stockage :** Fichier dans l'app Documents (dossier, droits d'accès)
- **Format :** Données JSON interne (moteur o-spreadsheet) + métadonnées Odoo
- **Identification :** Document ID (doc.document ou équivalent)
- **Nom :** Titre éditable par l'utilisateur
- **Locale :** Paramètres régionaux (séparateurs, formats date/heure, premier jour de la semaine)
- **Versions :** Historique automatique des sauvegardes

**Règles métier :**
- Un classeur appartient à un dossier Documents
- Droits : Viewer (lecture, téléchargement) / Editor (édition, partage, version history)
- Téléchargement .xlsx : formules Odoo converties en valeurs au moment du téléchargement
- Conversion en dashboard : le classeur est déplacé vers Dashboards (supprimé de Documents)

---

### 1.2 Feuille (Sheet)

**Rôle :** Onglet du classeur — contient cellules, listes insérées, tableaux croisés dynamiques, graphiques.

**Concepts clés :**
- **Nom :** Identifiant feuille (éditable)
- **Contenu :** Grille de cellules + objets insérés (listes, pivots, graphiques)
- **Ordre :** Onglets réordonnables
- **Premier onglet :** Utilisé par défaut comme face avant du dashboard si conversion

---

### 1.3 Source de Données (Data Source)

**Rôle :** Connexion entre le classeur et un **modèle Odoo** (ou vue liste/pivot/graph). Chaque liste, pivot ou graphique inséré crée une source de données.

**Propriétés :**
- **Type :** List (#1, #2…), Pivot (#1, #2…), Chart
- **ID :** Affecté séquentiellement (List #1, Pivot #1, etc.)
- **Nom :** Libellé utilisateur
- **Modèle :** Modèle Odoo source (ex. `sale.order`, `account.move.line`)
- **Domain :** Filtres (règles) pour restreindre les enregistrements
- **Tri :** Champs et sens (ascendant/descendant)
- **Colonnes / Lignes / Mesures :** Selon type (liste = colonnes, pivot = dimensions + mesures)

**Règles métier :**
- Les données sont rafraîchies à l'ouverture du classeur, au rechargement de la page, ou via « Data ‣ Refresh all data »
- Supprimer une liste ou un pivot de la feuille ne supprime pas la source ; suppression explicite dans le panneau Data
- Supprimer un graphique supprime sa source de données
- Les filtres globaux s'appliquent en combinaison avec le domain de la source

---

## 2. Types de Sources et Formules

### 2.1 Liste (List)

**Insertion :** Depuis une vue liste Odoo → « Insert list in spreadsheet ».

**Propriétés :**
- Model, Columns (champs visibles), Domain, Sorting
- Nombre de lignes insérées (par défaut = premier page de la liste)

**Formules Odoo :**
- `=ODOO.LIST.HEADER(list_id, field_name)` — en-tête de colonne
- `=ODOO.LIST(list_id, index, field_name)` — valeur de la cellule (index = numéro de ligne dans la liste avant insertion)

**Règles :**
- Ajout de lignes : étirer la dernière ligne ou « Data ‣ Re-insert list »
- Ajout de colonnes : insérer colonne, copier en-tête, modifier le nom de champ dans la formule
- Dupliquer une liste = nouvelle source (nouvel ID)
- Coller « Paste as value » : coupe le lien avec la base

---

### 2.2 Tableau croisé dynamique (Pivot)

**Insertion :** Depuis une vue pivot Odoo → « Insert in Spreadsheet ».

**Propriétés :**
- Model, Columns, Rows (dimensions), Measures
- Domain, (optionnel) conversion en pivot dynamique pour modifier dimensions/mesures dans le tableur

**Formules (pivot statique) :**
- `=PIVOT.HEADER(pivot_id, [domain_field_name, …], [domain_value, …])`
- `=PIVOT.VALUE(pivot_id, measure_name, [domain_field_name, …], [domain_value, …])`
- `measure_name` ex. : `product_uom_qty:sum`
- `domain_field_name` ex. : `user_id` ou `date_order:month`

**Pivot dynamique :**
- Fonction `=PIVOT(pivot_id, [row_count], [include_total], [include_column_titles], [column_count])`
- Permet de modifier colonnes, lignes et mesures depuis le tableur

---

### 2.3 Graphique (Chart)

**Insertion :** Depuis une vue graph Odoo → « Insert in Spreadsheet ».

**Propriétés :**
- Configuration et design modifiables dans le panneau
- Lien vers menu Odoo possible (drill-down)
- Pas de formules : données affichées uniquement, pas de calcul dans les cellules

**Règles :**
- Clic sur un point de données → ouverture de la liste Odoo correspondante
- Suppression du graphique = suppression de la source

---

## 3. Fonctions et Formules

### 3.1 Fonctions standard

Catégories documentées Odoo : Array, Database, Date, Engineering, Filter, Financial, Info, Logical, Lookup, Math, Operators, Parser, Statistical, Text, Web. Compatibilité large avec Excel (références Microsoft documentées).

### 3.2 Fonctions spécifiques Odoo

**Listes :**
- `ODOO.LIST.HEADER(list_id, field_name)`
- `ODOO.LIST(list_id, index, field_name)`

**Pivots :**
- `PIVOT.HEADER`, `PIVOT.VALUE`, `PIVOT` (dynamique)

**Filtres :**
- `ODOO.FILTER.VALUE(filter_name)` — valeur courante d’un filtre global

**Comptabilité / financier :**
- `ODOO.ACCOUNT.GROUP(type)` — IDs de comptes d’un groupe
- `ODOO.BALANCE(account_codes, date_range, [offset], [company_id], [include_unposted])`
- `ODOO.BALANCE.TAG(account_tag_ids, …)`
- `ODOO.CREDIT`, `ODOO.DEBIT`
- `ODOO.CURRENCY.RATE(currency_from, currency_to, [date])`
- `ODOO.FISCALYEAR.START(day, [company_id])`, `ODOO.FISCALYEAR.END(day, [company_id])`
- `ODOO.PARTNER.BALANCE(partner_ids, …)`
- `ODOO.RESIDUAL(…)`

**Export .xlsx :** Les formules non compatibles Excel sont remplacées par leur valeur au moment de l’export.

---

## 4. Règles Métier Transverses

### 4.1 Filtres globaux

- Définis au niveau du classeur
- S’appliquent à toutes les sources dont le domain est combiné avec la valeur du filtre
- Ne pas utiliser les mêmes critères en filtre global et dans le domain initial de la liste/pivot pour éviter les redondances

### 4.2 Liens cliquables

- Cellule → lien menu Odoo, autre feuille, ou URL externe
- Graphique → lien menu Odoo (configuration du graphique)

### 4.3 Données financières

- Insertion via les fonctions Odoo (comptes, soldes, crédit/débit, exercice, devise, partenaire) sans insertion préalable de liste/pivot

### 4.4 Templates

- Tout classeur peut être enregistré comme template (File ‣ Save as template)
- Les templates sont globaux à la base
- Création classeur : « Blank spreadsheet » ou choix d’un template
- Gestion : Documents ‣ Configuration ‣ Spreadsheet Templates (copie, édition, suppression)

### 4.5 Historique des versions

- Sauvegarde automatique des versions
- File ‣ See version history : consultation, restauration, copie d’une version, versions nommées
- Réservé aux utilisateurs avec droits Editor

### 4.6 Paramètres régionaux (locale)

- Gérés au niveau du classeur (séparateurs milliers/décimales, formats date/heure, premier jour de la semaine)
- Défaut = locale de l’utilisateur créateur
- File ‣ Settings pour consulter/modifier
- Icône globe si la locale du classeur diffère de celle du profil utilisateur

### 4.7 Conversion en tableau de bord

- File ‣ Add to dashboard
- Création d’un dashboard avec nom, section, groupes d’accès
- Le classeur est ensuite géré uniquement via l’app Dashboards (supprimé de Documents)

---

## 5. Intégration avec Odoo Documents et autres apps

- **Documents :** Stockage, dossiers, partage, droits (Viewer/Editor), corbeille
- **Toute app avec vues liste/pivot/graph :** Insertion de listes, pivots, graphiques (Sales, CRM, Inventory, Accounting, etc.)
- **Accounting :** Fonctions Odoo comptables (BALANCE, CREDIT, DEBIT, FISCALYEAR, etc.)
- **Dashboards :** Classeur comme base d’un dashboard ; édition du classeur sous-jacent via Dashboards

---

## 6. Synthèse pour Miyukini

**Entités métier à modéliser :**
- Classeur (document + métadonnées + locale)
- Feuille (onglet, ordre)
- Source de données (type, modèle, domain, tri, colonnes/dimensions/mesures)
- Formules (références aux sources, fonctions Odoo-like)
- Template (classeur figé réutilisable)
- Version (snapshot pour historique)
- Filtre global (nom, valeur, application aux sources)

**Règles à respecter :**
- Isolation des sources (ID stable, suppression explicite)
- Rafraîchissement des données à l’ouverture / demande
- Export (équivalent .xlsx) : formules « métier » converties en valeurs
- Gouvernance des accès (lecture / édition) et traçabilité des versions

---

**Document créé le :** 2026-02-01
