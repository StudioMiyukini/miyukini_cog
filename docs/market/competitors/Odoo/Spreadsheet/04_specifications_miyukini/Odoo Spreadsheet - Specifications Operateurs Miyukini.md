# Odoo Spreadsheet — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Spreadsheet** d'Odoo (intégrée à Odoo Documents).

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour l'équivalent Spreadsheet
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d'Ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **SpreadsheetOperator** | Gestion des classeurs (création, édition, contenu, sources) | Opérateur de Service |
| **SpreadsheetDataSourceOperator** | Gestion des sources de données (liste, pivot, graph) et liaison aux modèles | Opérateur de Service |
| **SpreadsheetTemplateOperator** | Gestion des templates de classeurs | Opérateur de Service |
| **SpreadsheetVersionOperator** | Gestion de l'historique des versions (snapshots, restauration) | Opérateur de Service |
| **SpreadsheetUI** | Interface utilisateur Spreadsheet (grille, menus, panneaux) | Opérateur d'Interface |

**Note :** Le stockage physique des documents (dossiers, partage) peut être délégué à un Opérateur Documents/Fichiers existant ou dédié.

### 1.2 Équipe d'Opérateurs : SpreadsheetService

**Définition :**
> **SpreadsheetService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de classeurs, rapports et tableaux de bord basés sur des données gouvernées.**

**Composition :**
- SpreadsheetOperator (niveau sécurité 2)
- SpreadsheetDataSourceOperator (niveau sécurité 2)
- SpreadsheetTemplateOperator (niveau sécurité 2)
- SpreadsheetVersionOperator (niveau sécurité 2)
- SpreadsheetUI (niveau sécurité 1)
- Opérateur Documents/Fichiers (niveau sécurité 1–2, selon partage)

---

## 2. Opérateurs Détaillés

### 2.1 SpreadsheetOperator

**Rôle :** Gestion des classeurs (création, métadonnées, feuilles, locale, partage, conversion dashboard).

**Capacités :**
- Création/modification de classeurs
- Gestion des feuilles (onglets, ordre, nom)
- Paramètres régionaux (locale) par classeur
- Partage (Viewer/Editor) et option « Freeze and share »
- Conversion en tableau de bord (exposition en vue dashboard)
- Liaison avec Documents (stockage, dossiers, corbeille)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création/modification classeur, partage, conversion dashboard
- **KindMother** : Persistance des classeurs (WriteIntent)
- **Master Butler** : Permissions création/édition/partage
- **WorrySentinel** : Vérification niveau sécurité, isolation cross-équipe
- **Ever Buddy** : Cycle de vie classeur (archivage, corbeille)

**Contrat d'équipe :**
- Consomme : SpreadsheetDataSourceOperator (sources), SpreadsheetTemplateOperator (templates), SpreadsheetVersionOperator (versions), Opérateur Documents (stockage, partage)
- Expose : `spreadsheet.create`, `spreadsheet.update`, `spreadsheet.share`, `spreadsheet.convert_to_dashboard`, `spreadsheet.export_xlsx`

**Mandat de Permission requis :**
- Création classeur : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Modification classeur : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Partage : Mandat avec StrongFather (décision) + Opérateur Documents (droits)
- Conversion dashboard : Mandat avec StrongFather (décision) + Master Butler (groupes d'accès)

### 2.2 SpreadsheetDataSourceOperator

**Rôle :** Gestion des sources de données (liste, pivot, graph) et liaison aux modèles/vues des Opérateurs métier.

**Capacités :**
- Création/suppression de sources (List, Pivot, Chart)
- Définition model, domain, tri, colonnes/dimensions/mesures
- Rafraîchissement des données (à la demande ou à l'ouverture)
- Résolution des formules ODOO.LIST, PIVOT, et fonctions Odoo-like (BALANCE, CREDIT, etc.)
- Application des filtres globaux au domain

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision d'accès aux données (quelle vue, quel modèle)
- **KindMother** : Lecture des données (ReadIntent ou équivalent) ; pas d'écriture métier via Spreadsheet
- **Master Butler** : Permissions « insérer liste depuis X », « utiliser fonctions Y »
- **WorrySentinel** : Niveau sécurité des données (comptabilité, RH, etc.)

**Contrat d'équipe :**
- Consommé par : SpreadsheetOperator
- Consomme : Opérateurs métier (MiyuSales, MiyuInvoice, MiyuContacts, etc.) via vues exposées (liste, pivot, graph) ; MiyuCalc / MiyuInvoice pour fonctions financières
- Expose : `datasource.create`, `datasource.refresh`, `datasource.delete`, `formula.resolve` (liste, pivot, odoo_functions)

**Mandat de Permission requis :**
- Insérer liste depuis un Opérateur X : Mandat avec StrongFather (décision) + Opérateur X (exposition vue liste)
- Utiliser fonctions financières : Mandat avec MiyuInvoice / miyucptaledger (lecture comptable) + Master Butler (permission)
- Rafraîchissement : Mandat avec KindMother (lecture) + WorrySentinel (niveau sécurité)

### 2.3 SpreadsheetTemplateOperator

**Rôle :** Gestion des templates de classeurs (création, copie, édition, suppression).

**Capacités :**
- Enregistrement d'un classeur comme template
- Création d'un classeur à partir d'un template
- Gestion des templates (Configuration) : copie, édition, suppression
- Templates globaux à la base (ou scope défini par gouvernance)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de créer/supprimer un template
- **KindMother** : Persistance des templates (WriteIntent)
- **Master Butler** : Permissions « créer template », « utiliser template »
- **Ever Buddy** : Cycle de vie template (dépréciation, retrait)

**Contrat d'équipe :**
- Consommé par : SpreadsheetOperator
- Consomme : KindMother (persistance)
- Expose : `template.create`, `template.copy`, `template.update`, `template.delete`, `template.list`

**Mandat de Permission requis :**
- Création template : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Utilisation template : Mandat avec SpreadsheetOperator (création classeur depuis template)

### 2.4 SpreadsheetVersionOperator

**Rôle :** Gestion de l'historique des versions (snapshots automatiques, restauration, copie).

**Capacités :**
- Sauvegarde automatique des versions (politique configurable)
- Liste des versions (utilisateur, date/heure, nom optionnel)
- Consultation en lecture seule d'une version
- Restauration (remplacement contenu courant par la version)
- Copie d'une version (nouveau classeur)
- Nommage des versions (jalons)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de restaurer (impact sur contenu courant)
- **KindMother** : Persistance des snapshots (immutables) ; WriteIntent pour « restaurer » (écriture du contenu courant)
- **Master Butler** : Permissions « voir historique », « restaurer », « copier version »
- **Ever Buddy** : Rétention, purge (politique de conservation des versions)

**Contrat d'équipe :**
- Consommé par : SpreadsheetOperator
- Consomme : KindMother (snapshots, contenu courant)
- Expose : `version.list`, `version.get`, `version.restore`, `version.copy`, `version.rename`

**Mandat de Permission requis :**
- Restauration : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Consultation historique : Mandat avec Master Butler (permission « voir historique »)

### 2.5 SpreadsheetUI

**Rôle :** Interface utilisateur Spreadsheet (grille, barre de formule, onglets, menus, panneaux).

**Capacités :**
- Affichage et édition de la grille (cellules, formules)
- Menus File, Edit, Insert, Data, View
- Panneaux Data (sources), Filtres globaux, Settings (locale), Version history
- Insertion listes/pivots/graphiques (initiée depuis les Opérateurs métier ou depuis le menu Data)
- Liens cliquables (menu Miyukini, feuille, URL)
- Raccourcis clavier (palette de commandes type Ctrl+K)
- Export .xlsx (délégué au backend)

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **BondingBrother** : Traduction des intentions utilisateur vers SpreadsheetOperator / DataSourceOperator
- **Master Butler** : Permissions d'affichage (Viewer vs Editor)
- **WorrySentinel** : Pas d'affichage de données au-delà du niveau sécurité du Mandat

**Contrat d'équipe :**
- Consommé par : Utilisateur final
- Consomme : SpreadsheetOperator, SpreadsheetDataSourceOperator, SpreadsheetTemplateOperator, SpreadsheetVersionOperator
- Expose : Toutes les actions UI (création, édition, partage, rafraîchissement, export, etc.) via BondingBrother

**Mandat de Permission requis :**
- Toute action d'édition : Mandat couvrant SpreadsheetOperator + KindMother (WriteIntent) selon action
- Lecture seule : Mandat Viewer (pas d'écriture)

---

## 3. Contrats d'Équipe et Mandats

### 3.1 Contrat d'Équipe SpreadsheetService

**Membres :** SpreadsheetOperator, SpreadsheetDataSourceOperator, SpreadsheetTemplateOperator, SpreadsheetVersionOperator, SpreadsheetUI, Opérateur Documents (optionnel).

**Flux autorisés :**
- SpreadsheetUI → BondingBrother → SpreadsheetOperator, SpreadsheetDataSourceOperator, SpreadsheetTemplateOperator, SpreadsheetVersionOperator
- SpreadsheetOperator → SpreadsheetDataSourceOperator, SpreadsheetTemplateOperator, SpreadsheetVersionOperator, Documents
- SpreadsheetDataSourceOperator → Opérateurs métier (vues liste/pivot/graph), MiyuInvoice / MiyuCalc (fonctions financières)

**Types d'échanges :**
- Intentions (création, modification, partage, rafraîchissement)
- Données (contenu classeur, sources, valeurs de cellules, templates, versions)
- Permissions et décisions (StrongFather, Master Butler, WorrySentinel)

**Niveau de validation :** StrongFather pour toute décision affectant la persistance ou le partage ; Master Butler pour toute vérification de permission.

### 3.2 Mandats de Permission typiques

| Action | Mandat requis |
|--------|----------------|
| Créer un classeur | StrongFather (décision) + KindMother (WriteIntent) |
| Modifier un classeur | StrongFather (décision) + KindMother (WriteIntent) |
| Partager (Viewer/Editor) | StrongFather (décision) + Documents (droits) |
| Insérer une liste depuis MiyuSales | StrongFather (décision) + MiyuSales (exposition vue liste) |
| Utiliser ODOO.BALANCE (équivalent) | MiyuInvoice / miyucptaledger (lecture) + Master Butler (permission) |
| Rafraîchir toutes les sources | KindMother (lecture) + WorrySentinel (niveau sécurité) |
| Restaurer une version | StrongFather (décision) + KindMother (WriteIntent) |
| Créer un template | StrongFather (décision) + KindMother (WriteIntent) |
| Convertir en dashboard | StrongFather (décision) + Master Butler (groupes d'accès) |

---

## 4. Niveaux de Sécurité

| Opérateur / Données | Niveau | Justification |
|---------------------|--------|----------------|
| SpreadsheetOperator | 2 (Sensitive) | Contenu peut inclure données métier sensibles |
| SpreadsheetDataSourceOperator | 2 (Sensitive) | Accès aux modèles métier (ventes, comptabilité, etc.) |
| SpreadsheetTemplateOperator | 2 (Sensitive) | Templates peuvent figer des structures à données sensibles |
| SpreadsheetVersionOperator | 2 (Sensitive) | Historique des contenus sensibles |
| SpreadsheetUI | 1 (Standard) | Interface ; données affichées selon Mandat |
| Données listes/pivots (ventes, CRM) | 2 | Données métier |
| Données comptables (BALANCE, etc.) | 3 (Critical) | Données financières |
| Filtres globaux, locale | 1 | Paramètres peu sensibles |

**Règle :** Un flux ne peut pas descendre en niveau de sécurité ; WorrySentinel bloque ou dégrade si le Mandat ne couvre pas le niveau requis.

---

## 5. Intégration avec les Cores

- **StrongFather** : Toute décision de création/ modification/ partage/ restauration/ conversion dashboard/ template
- **KindMother** : Persistance classeurs, sources (métadonnées), templates, versions (snapshots + contenu courant)
- **Master Butler** : Permissions « créer/éditer classeur », « insérer liste depuis X », « utiliser fonctions Y », « voir historique », « partager »
- **WorrySentinel** : Niveau de sécurité des données affichées/modifiées ; blocage si contexte dégradé
- **Ever Buddy** : Cycle de vie classeur (archivage, corbeille), templates (dépréciation), versions (rétention)
- **BondingBrother** : Traduction des intentions UI vers les Opérateurs et les Cores

---

**Document créé le :** 2026-02-01
