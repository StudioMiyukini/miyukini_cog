# Odoo Quality — Index de l'Analyse

## Statut

✅ **Analyse complète à 100% (7/7 documents)**

---

## Documents de l'Analyse

### 1. Logique Métier
📄 [Odoo Quality - Logique Metier Complete.md](./00_logique_metier/Odoo%20Quality%20-%20Logique%20Metier%20Complete.md)

**Contenu :**
- Modèles de données (Quality Control Points, Quality Checks, Quality Alerts, Failure Locations)
- Règles métier et contraintes (QCP, Control Per, Control Frequency, types de contrôle)
- Workflows (contrôles : pending → passed/failed ; alertes : stages Kanban)
- Types de contrôles (Instructions, Pass-Fail, Measure, Picture, Worksheet, Spreadsheet, Register Production, Print label)
- Droits d'accès (Quality User, Quality Manager)
- Intégration Manufacturing et Inventory

### 2. Parcours Utilisateur
📄 [Odoo Quality - Parcours Utilisateur Detailles.md](./01_parcours_utilisateur/Odoo%20Quality%20-%20Parcours%20Utilisateur%20Detailles.md)

**Contenu :**
- Personas (Responsable Qualité, Opérateur Qualité, Responsable Production, Responsable Entrepôt)
- Parcours d'onboarding
- Scénarios d'usage (QCP sur MO, contrôle sur work order Shop Floor, contrôle manuel, alerte depuis picking, reporting)
- Points de friction identifiés
- Recommandations pour Miyukini

### 3. UI/UX
📄 [Odoo Quality - Analyse UI UX.md](./02_ui_ux/Odoo%20Quality%20-%20Analyse%20UI%20UX.md)

**Contenu :**
- Vues principales (Control Points, Quality Checks, Quality Alerts)
- Formulaires QCP, Check, Alert
- Patterns de navigation (menu Quality, boutons sur MO/picking, Shop Floor)
- Configuration (équipes, templates, Failure Locations)
- Recommandations pour Miyukini

### 4. Intégrations Cross-App
📄 [Odoo Quality - Integrations Cross App.md](./03_integrations/Odoo%20Quality%20-%20Integrations%20Cross%20App.md)

**Contenu :**
- Dépendances (stock obligatoire ; mrp, mail optionnels)
- Flux de données (Stock, MRP, Mail, Produits)
- Recommandations pour Miyukini

### 5. Spécifications Opérateurs Miyukini
📄 [Odoo Quality - Specifications Operateurs Miyukini.md](./04_specifications_miyukini/Odoo%20Quality%20-%20Specifications%20Operateurs%20Miyukini.md)

**Contenu :**
- Opérateurs identifiés (QualityControlPointOperator, QualityCheckOperator, QualityAlertOperator, QualityTeamOperator, FailureLocationOperator, QualityMetricsOperator, QualityUI)
- Contrat d'équipe QualityService
- Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

### 6. Guide Intégration COG
📄 [Odoo Quality - Guide Integration COG.md](./05_integration_cog/Odoo%20Quality%20-%20Guide%20Integration%20COG.md)

**Contenu :**
- Architecture d'intégration COG
- Patterns WriteIntent et Mandates
- Exemples de code pseudo-Rust (création QCP, traitement contrôle Pass/Fail, création alerte, création automatique contrôles depuis QCP)
- Gestion des gouvernances

### 7. Guide Implémentation
📄 [Odoo Quality - Guide Implementation.md](./06_guides_implementation/Odoo%20Quality%20-%20Guide%20Implementation.md)

**Contenu :**
- Architecture technique détaillée
- Spécifications des crates Rust (QCP, Check, Alert, Team, FailureLocation, Metrics, UI)
- Schémas de données (QualityControlPoint, QualityCheck, QualityAlert, QualityTeam, FailureLocation)
- API et contrats
- Plan de développement par phases (MVP → Inventory → Manufacturing → Shop Floor → Failure Locations et Rapports)
- Bornage fonctionnel

---

## Service Miyukini Proposé

**Nom :** `MiyukiniQuality` ou `MiyuQuality`

**Opérateurs :**
- `QualityControlPointOperator` : Gestion des points de contrôle qualité (QCP)
- `QualityCheckOperator` : Gestion des contrôles qualité (création auto/manuelle, traitement Pass/Fail)
- `QualityAlertOperator` : Gestion des alertes qualité
- `QualityTeamOperator` : Gestion des équipes qualité
- `FailureLocationOperator` : Gestion des lieux de défaillance
- `QualityMetricsOperator` : Calculs et rapports (conformité, causes)
- `QualityUI` : Interface utilisateur Quality

**Équipe d'Opérateurs :** `QualityService`

---

## Source d'Analyse

**Documentation :** Odoo 19.0 — Supply Chain / Quality

**Références :**
- [Quality — Odoo 19.0](https://www.odoo.com/documentation/19.0/applications/inventory_and_mrp/quality.html)
- [Quality control points](https://www.odoo.com/documentation/19.0/applications/inventory_and_mrp/quality/quality_management/quality_control_points.html)
- [Quality checks](https://www.odoo.com/documentation/19.0/applications/inventory_and_mrp/quality/quality_management/quality_checks.html)
- [Quality alerts](https://www.odoo.com/documentation/19.0/applications/inventory_and_mrp/quality/quality_management/quality_alerts.html)

**Version analysée :** Odoo 19.0

**Date d'analyse :** 2026-02-01

---

## Notes

- Application Supply Chain ; dépendances : stock (obligatoire), mrp et mail (optionnels).
- QCP = seule source de contrôles pour les work orders ; création manuelle possible pour MO et pickings.
- Types de contrôle : Instructions, Pass-Fail, Measure, Take a Picture, Worksheet, Spreadsheet, Register Production, Print label.
- Alertes créables depuis l'app Quality, un MO, un picking ou le Shop Floor (menu ⋮ sur carte work order).
