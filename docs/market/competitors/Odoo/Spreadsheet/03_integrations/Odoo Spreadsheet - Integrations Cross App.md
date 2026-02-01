# Odoo Spreadsheet — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Spreadsheet** d'Odoo (intégrée à Odoo Documents), identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Documentation Odoo 19.0 (Productivity / Spreadsheet, Documents)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres apps Odoo (Documents, Dashboards, Accounting, etc.)
- Flux de données inter-apps (insertion liste/pivot/graph, formules Odoo)
- Mécanismes d'intégration (sources de données, filtres globaux)
- APIs et hooks utilisés (o-spreadsheet, Documents)
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Modules Requis

**Dépendances implicites (Spreadsheet fait partie de Odoo Documents / Productivity) :**
- **Documents (documents)** : Stockage des classeurs, dossiers, partage, droits (Viewer/Editor), corbeille
- **Web (web)** : Framework web, interface utilisateur
- **Base** : Modèles, utilisateurs, entreprises, langues (locale)

### 1.2 Moteur o-spreadsheet

- **o-spreadsheet** : Composant open-source Odoo (répertoire séparé sur GitHub) — moteur de calcul, grille, formules, graphiques
- Intégration côté Odoo : persistance du contenu (JSON), sources de données (listes, pivots, graphiques) reliées aux modèles Odoo
- Documentation technique : `o-spreadsheet/doc/extending/` (architecture, business_feature)

### 1.3 Modules Optionnels (Intégrations si installés)

**Toute app exposant des vues liste / pivot / graph :**
- **Sales** : Commandes, devis, lignes ; insertion listes, pivots, graphiques
- **CRM** : Opportunités, leads ; insertion listes, pivots, graphiques
- **Inventory** : Mouvements, produits ; insertion listes, pivots, graphiques
- **Project** : Tâches, projets ; insertion listes, pivots, graphiques
- **Purchase** : Commandes d'achat ; insertion listes, pivots, graphiques
- **Accounting** : Écritures, comptes ; insertion listes, pivots, graphiques + **fonctions Odoo** (ODOO.BALANCE, ODOO.CREDIT, ODOO.DEBIT, ODOO.FISCALYEAR.*, etc.)
- **HR, Marketing, etc.** : Idem selon vues disponibles

**Dashboards :**
- **Dashboards (spreadsheet_dashboard ou équivalent)** : Conversion classeur → dashboard ; édition du classeur sous-jacent via l'app Dashboards

---

## 2. Intégrations Détaillées

### 2.1 Intégration avec Documents

**Flux :**
```
Spreadsheet (classeur) → doc.document (ou équivalent) → Dossiers, Partage, Droits
```

**Mécanismes :**
- Création classeur = création d'un document dans Documents (type spreadsheet)
- Stockage : contenu JSON (o-spreadsheet) + métadonnées (nom, locale, dossier, propriétaire)
- Partage : même mécanisme que Documents (utilisateurs, lien ; Viewer/Editor)
- Corbeille : Move to trash → déplacement vers dossier corbeille ; suppression définitive après 30 jours (politique Documents)
- Templates : classeurs marqués comme templates ; Configuration ‣ Spreadsheet Templates pour gestion (copie, édition, suppression)

**Champs / concepts liés :**
- Document ID, nom, dossier_id, owner_id, permission (Viewer/Editor)
- Contenu : feuilles, cellules, formules, sources de données (références modèles, domain, tri, colonnes/dimensions/mesures)

**Recommandations pour Miyukini :**
- Intégration native avec un Opérateur Documents / Fichiers Miyukini
- Classeur = document typé « spreadsheet » avec métadonnées (locale, template, version)
- Partage et droits alignés sur Mandats et permissions (StrongFather, Master Butler)

### 2.2 Intégration avec les apps métier (Listes, Pivots, Graphiques)

**Flux :**
```
Vue Liste/Pivot/Graph (app X) → Insert in Spreadsheet → Source de données (model, domain, sort, columns/dimensions/measures)
                                                       → Classeur (feuille, formules ODOO.LIST / PIVOT / Chart)
```

**Mécanismes :**
- **Insertion** : depuis une vue liste, pivot ou graph d'une app Odoo ; action « Insert list in spreadsheet » / « Insert in Spreadsheet »
- **Source de données** : créée dans le classeur ; lien vers le modèle Odoo (model), domain, tri, colonnes (liste) ou dimensions/mesures (pivot)
- **Rafraîchissement** : à l'ouverture du classeur, au rechargement de la page, ou via « Data ‣ Refresh all data »
- **Formules** : `ODOO.LIST.HEADER`, `ODOO.LIST` (listes) ; `PIVOT.HEADER`, `PIVOT.VALUE`, `PIVOT` (pivots) ; graphiques = affichage uniquement, pas de formules cellule
- **Accès enregistrements** : « See record » / « See records » → ouverture de la vue Odoo correspondante (filtrage sur l'enregistrement ou l'ensemble concerné)

**APIs / hooks (côté Odoo) :**
- Action serveur ou menu « Insert in Spreadsheet » sur les vues liste/pivot/graph
- RPC ou API pour : récupération des données (model, domain, fields/sort pour liste ; dimensions/measures pour pivot) ; mise à jour des sources côté classeur
- Résolution des formules ODOO.LIST / PIVOT côté backend (ou moteur hybride) pour renvoyer les valeurs aux cellules

**Recommandations pour Miyukini :**
- Contrat d'équipe : Opérateur Spreadsheet consomme les Opérateurs métier (MiyuSales, MiyuInvoice, etc.) via « vues » exposées (liste, pivot, graph)
- Pas d'accès direct aux modèles ; passage par BondingBrother et Mandats (StrongFather, Master Butler)
- Rafraîchissement des sources = appels gouvernés (permissions, niveau de sécurité WorrySentinel)

### 2.3 Intégration avec Accounting (Fonctions Odoo)

**Flux :**
```
Classeur (formules ODOO.BALANCE, ODOO.CREDIT, etc.) → Backend Accounting → Comptes, Périodes, Sociétés
```

**Mécanismes :**
- **Fonctions Odoo** documentées : ODOO.ACCOUNT.GROUP, ODOO.BALANCE, ODOO.BALANCE.TAG, ODOO.CREDIT, ODOO.DEBIT, ODOO.CURRENCY.RATE, ODOO.FISCALYEAR.START/END, ODOO.PARTNER.BALANCE, ODOO.RESIDUAL
- Arguments typiques : account_codes, date_range, offset, company_id, include_unposted (selon fonction)
- Résolution côté backend Accounting ; résultat renvoyé à la cellule

**Recommandations pour Miyukini :**
- Fonctions « Miyukini » équivalentes (MiyuCalc / MiyuInvoice / miyucptaledger) : BALANCE, CREDIT, DEBIT, FISCALYEAR, etc.
- Chaque fonction = appel gouverné (KindMother pour lecture comptable, Master Butler pour permissions, WorrySentinel pour niveau sécurité)
- Pas d'exécution de formules côté client sur des données sensibles sans Mandat

### 2.4 Intégration avec Filtres globaux

**Flux :**
```
Filtres globaux (nom, valeur) → Combinés avec le domain de chaque source → Données chargées dans les listes/pivots
```

**Mécanismes :**
- Filtres globaux définis au niveau du classeur
- Chaque source (liste, pivot) a un domain ; au chargement, domain effectif = domain de la source ∩ critères des filtres globaux
- Formule `ODOO.FILTER.VALUE(filter_name)` pour utiliser la valeur courante d'un filtre dans une cellule
- Panneau « Filtres globaux » pour gérer et afficher les valeurs

**Recommandations pour Miyukini :**
- Filtres globaux = paramètres du classeur (nom, type, valeur par défaut) ; application cohérente à toutes les sources
- Traçabilité : quels filtres sont appliqués à quel rafraîchissement (audit)

### 2.5 Intégration avec Dashboards

**Flux :**
```
Classeur → File ‣ Add to dashboard → Dashboard créé (nom, section, groupes d'accès)
                                   → Classeur retiré de Documents, géré via app Dashboards
```

**Mécanismes :**
- Conversion : création d'une entité « dashboard » à partir du classeur ; premier onglet = face avant du dashboard
- Le classeur n'est plus éditable depuis Documents ; toute modification se fait via l'app Dashboards (édition du classeur sous-jacent)
- Groupes d'accès : qui peut voir le dashboard (aligné sur les groupes Odoo)

**Recommandations pour Miyukini :**
- Équivalent « Tableaux de bord » Miyukini : classeur convertible en vue « dashboard » (lecture seule ou édition selon Mandat)
- Gouvernance : StrongFather pour décision « convertir en dashboard » ; Master Butler pour groupes d'accès

### 2.6 Intégration avec Templates et Versions

**Templates :**
- Enregistrement : File ‣ Save as template → nom du template ; le template est global à la base
- Création classeur : New ‣ Spreadsheet → choix « Blank » ou template
- Gestion : Documents ‣ Configuration ‣ Spreadsheet Templates (copie, édition, suppression)
- Pas de notion « template personnel » dans la doc standard

**Versions :**
- Sauvegarde automatique des versions à chaque modification (ou politique similaire)
- File ‣ See version history : liste des versions (utilisateur, date/heure) ; consultation en lecture seule ; Restore / Make a copy
- Nommage des versions pour jalons

**Recommandations pour Miyukini :**
- Templates : entités de premier ordre (KindMother) ; cycle de vie géré par Ever Buddy
- Versions : snapshots immuables ; restauration = nouvelle révision avec traçabilité (qui, quand)

---

## 3. Synthèse des Flux Inter-Apps

| App / Composant | Direction | Données / API |
|-----------------|-----------|----------------|
| Documents | Spreadsheet → Documents | Stockage classeur, dossiers, partage, corbeille |
| Sales / CRM / Inventory / … | App → Spreadsheet | Insertion liste/pivot/graph ; model, domain, sort, columns/dimensions/measures |
| Spreadsheet | Spreadsheet → App | Rafraîchissement données ; formules ODOO.LIST, PIVOT ; See record(s) |
| Accounting | Spreadsheet → Accounting | Formules ODOO.BALANCE, CREDIT, DEBIT, FISCALYEAR, etc. |
| Dashboards | Spreadsheet → Dashboards | Conversion classeur → dashboard ; édition via Dashboards |
| o-spreadsheet | Odoo ↔ o-spreadsheet | Contenu JSON, calcul formules, rendu grille/graphiques |

---

## 4. Recommandations pour Miyukini

### 4.1 Architecture d'intégration

- **Opérateur Spreadsheet** (MiyuSpreadsheet / MiyukiniSpreadsheet) : création/édition classeurs, sources, formules, templates, versions
- **Opérateur Documents** (ou Fichiers) : stockage, dossiers, partage, droits
- **Opérateurs métier** : exposition de « vues » (liste, pivot, graph) consommées par Spreadsheet via BondingBrother et Mandats
- **Cores** : StrongFather (décisions partage, conversion dashboard), KindMother (persistance classeurs, templates, versions), Master Butler (permissions), WorrySentinel (niveaux sécurité)

### 4.2 Contrats d'équipe

- SpreadsheetService : SpreadsheetOperator + DocumentsOperator + consommateurs des Opérateurs métier (vues)
- Mandats : « Insérer une liste depuis MiyuSales », « Utiliser les fonctions MiyuInvoice (BALANCE, etc.) », « Partager en Viewer/Editor »

### 4.3 Sécurité et audit

- Niveau de sécurité par type de données (listes comptables, RH, etc.) ; WorrySentinel pour bloquer ou dégrader
- Audit : qui a créé/modifié/partagé quel classeur, quelle version, quelles sources

---

**Document créé le :** 2026-02-01
