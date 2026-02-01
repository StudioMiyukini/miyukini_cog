# Odoo Spreadsheet — Index de l'Analyse

## Statut

✅ **Analyse complète à 100% (7/7 documents)**

---

## Documents de l'Analyse

### 1. Logique Métier
📄 [Odoo Spreadsheet - Logique Metier Complete.md](./00_logique_metier/Odoo%20Spreadsheet%20-%20Logique%20Metier%20Complete.md)

**Contenu :**
- Concepts de données (classeur, feuille, source de données)
- Types de sources (liste, pivot, graphique)
- Formules et fonctions (standard + Odoo-specific)
- Règles de liaison aux modèles Odoo
- Templates, versions, locale, conversion dashboard

### 2. Parcours Utilisateur
📄 [Odoo Spreadsheet - Parcours Utilisateur Detailles.md](./01_parcours_utilisateur/Odoo%20Spreadsheet%20-%20Parcours%20Utilisateur%20Detailles.md)

**Contenu :**
- Personas (Editor, Viewer, Administrateur)
- Parcours d'onboarding
- Scénarios d'usage principaux
- Points de friction identifiés
- Recommandations pour Miyukini

### 3. UI/UX
📄 [Odoo Spreadsheet - Analyse UI UX.md](./02_ui_ux/Odoo%20Spreadsheet%20-%20Analyse%20UI%20UX.md)

**Contenu :**
- Structure éditeur (grille, barre de formule, onglets)
- Menus et panneaux (File, Data, Settings, Version history)
- Patterns d'interaction (insertion, liens, partage)
- Raccourcis et accessibilité
- Recommandations pour Miyukini

### 4. Intégrations Cross-App
📄 [Odoo Spreadsheet - Integrations Cross App.md](./03_integrations/Odoo%20Spreadsheet%20-%20Integrations%20Cross%20App.md)

**Contenu :**
- Dépendances (Documents, o-spreadsheet, Accounting, Dashboards)
- Flux listes/pivots/graphiques depuis les apps métier
- Fonctions Odoo (comptabilité)
- Filtres globaux, templates, versions
- Recommandations pour Miyukini

### 5. Spécifications Opérateurs Miyukini
📄 [Odoo Spreadsheet - Specifications Operateurs Miyukini.md](./04_specifications_miyukini/Odoo%20Spreadsheet%20-%20Specifications%20Operateurs%20Miyukini.md)

**Contenu :**
- Opérateurs (SpreadsheetOperator, DataSourceOperator, TemplateOperator, VersionOperator, SpreadsheetUI)
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

### 6. Guide Intégration COG
📄 [Odoo Spreadsheet - Guide Integration COG.md](./05_integration_cog/Odoo%20Spreadsheet%20-%20Guide%20Integration%20COG.md)

**Contenu :**
- Architecture d'intégration COG
- Patterns WriteIntent et Mandates
- Exemples de code pseudo-Rust (création classeur, source, formules, partage, version)
- Gestion des gouvernances

### 7. Guide Implémentation
📄 [Odoo Spreadsheet - Guide Implementation.md](./06_guides_implementation/Odoo%20Spreadsheet%20-%20Guide%20Implementation.md)

**Contenu :**
- Architecture technique (crates miyuspreadsheet, miyuspreadsheet-ui)
- Schémas de données (Spreadsheet, DataSource, Version, Template)
- API et contrats
- Plan de développement par phases (MVP → Complet)
- Bornage fonctionnel

---

## Service Miyukini Proposé

**Nom :** `MiyukiniSpreadsheet` ou `MiyuSpreadsheet`

**Opérateurs :**
- `SpreadsheetOperator` : Gestion des classeurs
- `SpreadsheetDataSourceOperator` : Gestion des sources (liste, pivot, graph) et formules Odoo-like
- `SpreadsheetTemplateOperator` : Gestion des templates
- `SpreadsheetVersionOperator` : Gestion des versions
- `SpreadsheetUI` : Interface utilisateur Spreadsheet

**Équipe d'Opérateurs :** `SpreadsheetService`

---

## Source d'Analyse

**Documentation :** Odoo 19.0 — Productivity / Spreadsheet (intégré à Odoo Documents)

**Repositories :** 
- `https://github.com/odoo/odoo` (addons documents / spreadsheet)
- `https://github.com/odoo/o-spreadsheet` (moteur spreadsheet)

**Version analysée :** Odoo 19.0

**Date d'analyse :** 2026-02-01

---

## Notes

- Le module Spreadsheet fait partie de l'app **Odoo Documents** (Productivity).
- Intégrations multiples : toute app avec vues liste/pivot/graph, Accounting (fonctions Odoo), Dashboards.
- Moteur o-spreadsheet : composant open-source séparé (JavaScript/TypeScript).
- Données sensibles (comptabilité, RH) : niveau sécurité 3 ; gouvernance WorrySentinel et Mandats obligatoires.
