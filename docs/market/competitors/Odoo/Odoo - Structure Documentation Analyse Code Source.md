# Odoo — Structure de Documentation — Analyse Code Source

**Date :** 2026-02-01  
**Objectif :** Structure pour documenter l'analyse approfondie du code source Odoo

---

## Contexte

Cette structure organise l'analyse détaillée du code source Odoo extraite du repository GitHub pour chaque application, couvrant :
- Logique métier
- Parcours utilisateurs
- UI/UX
- Mécanismes cross-app

---

## Structure de documentation

```
docs/market/competitors/Odoo/
├── _index.md (index général)
├── 00_analyse_code_source/
│   ├── _index.md (index analyse code source)
│   ├── Odoo - Methodologie Analyse Code Source.md
│   └── Odoo - Architecture Generale Code Source.md
│
├── 01_finance/
│   ├── _index.md
│   ├── accounting/
│   │   ├── Odoo Accounting - Logique Metier.md
│   │   ├── Odoo Accounting - Parcours Utilisateur.md
│   │   ├── Odoo Accounting - UI UX.md
│   │   └── Odoo Accounting - Integrations Cross App.md
│   ├── invoicing/
│   │   ├── Odoo Invoicing - Logique Metier.md
│   │   ├── Odoo Invoicing - Parcours Utilisateur.md
│   │   ├── Odoo Invoicing - UI UX.md
│   │   └── Odoo Invoicing - Integrations Cross App.md
│   ├── expenses/
│   │   ├── Odoo Expenses - Logique Metier.md
│   │   ├── Odoo Expenses - Parcours Utilisateur.md
│   │   ├── Odoo Expenses - UI UX.md
│   │   └── Odoo Expenses - Integrations Cross App.md
│   ├── spreadsheet/
│   │   ├── Odoo Spreadsheet - Logique Metier.md
│   │   ├── Odoo Spreadsheet - Parcours Utilisateur.md
│   │   ├── Odoo Spreadsheet - UI UX.md
│   │   └── Odoo Spreadsheet - Integrations Cross App.md
│   ├── documents/
│   │   ├── Odoo Documents - Logique Metier.md
│   │   ├── Odoo Documents - Parcours Utilisateur.md
│   │   ├── Odoo Documents - UI UX.md
│   │   └── Odoo Documents - Integrations Cross App.md
│   └── sign/
│       ├── Odoo Sign - Logique Metier.md
│       ├── Odoo Sign - Parcours Utilisateur.md
│       ├── Odoo Sign - UI UX.md
│       └── Odoo Sign - Integrations Cross App.md
│
├── 02_sales/
│   ├── _index.md
│   ├── crm/
│   │   ├── Odoo CRM - Logique Metier.md
│   │   ├── Odoo CRM - Parcours Utilisateur.md
│   │   ├── Odoo CRM - UI UX.md
│   │   └── Odoo CRM - Integrations Cross App.md
│   ├── sales/
│   │   ├── Odoo Sales - Logique Metier.md
│   │   ├── Odoo Sales - Parcours Utilisateur.md
│   │   ├── Odoo Sales - UI UX.md
│   │   └── Odoo Sales - Integrations Cross App.md
│   ├── pos_shop/
│   │   ├── Odoo POS Shop - Logique Metier.md
│   │   ├── Odoo POS Shop - Parcours Utilisateur.md
│   │   ├── Odoo POS Shop - UI UX.md
│   │   └── Odoo POS Shop - Integrations Cross App.md
│   ├── pos_restaurant/
│   │   ├── Odoo POS Restaurant - Logique Metier.md
│   │   ├── Odoo POS Restaurant - Parcours Utilisateur.md
│   │   ├── Odoo POS Restaurant - UI UX.md
│   │   └── Odoo POS Restaurant - Integrations Cross App.md
│   ├── subscriptions/
│   │   ├── Odoo Subscriptions - Logique Metier.md
│   │   ├── Odoo Subscriptions - Parcours Utilisateur.md
│   │   ├── Odoo Subscriptions - UI UX.md
│   │   └── Odoo Subscriptions - Integrations Cross App.md
│   └── rental/
│       ├── Odoo Rental - Logique Metier.md
│       ├── Odoo Rental - Parcours Utilisateur.md
│       ├── Odoo Rental - UI UX.md
│       └── Odoo Rental - Integrations Cross App.md
│
├── 03_websites/
│   ├── _index.md
│   ├── website/
│   │   ├── Odoo Website - Logique Metier.md
│   │   ├── Odoo Website - Parcours Utilisateur.md
│   │   ├── Odoo Website - UI UX.md
│   │   └── Odoo Website - Integrations Cross App.md
│   ├── ecommerce/
│   │   ├── Odoo eCommerce - Logique Metier.md
│   │   ├── Odoo eCommerce - Parcours Utilisateur.md
│   │   ├── Odoo eCommerce - UI UX.md
│   │   └── Odoo eCommerce - Integrations Cross App.md
│   ├── blog/
│   │   ├── Odoo Blog - Logique Metier.md
│   │   ├── Odoo Blog - Parcours Utilisateur.md
│   │   ├── Odoo Blog - UI UX.md
│   │   └── Odoo Blog - Integrations Cross App.md
│   ├── forum/
│   │   ├── Odoo Forum - Logique Metier.md
│   │   ├── Odoo Forum - Parcours Utilisateur.md
│   │   ├── Odoo Forum - UI UX.md
│   │   └── Odoo Forum - Integrations Cross App.md
│   ├── live_chat/
│   │   ├── Odoo Live Chat - Logique Metier.md
│   │   ├── Odoo Live Chat - Parcours Utilisateur.md
│   │   ├── Odoo Live Chat - UI UX.md
│   │   └── Odoo Live Chat - Integrations Cross App.md
│   └── elearning/
│       ├── Odoo eLearning - Logique Metier.md
│       ├── Odoo eLearning - Parcours Utilisateur.md
│       ├── Odoo eLearning - UI UX.md
│       └── Odoo eLearning - Integrations Cross App.md
│
├── 04_supply_chain/
│   ├── _index.md
│   ├── inventory/
│   │   ├── Odoo Inventory - Logique Metier.md
│   │   ├── Odoo Inventory - Parcours Utilisateur.md
│   │   ├── Odoo Inventory - UI UX.md
│   │   └── Odoo Inventory - Integrations Cross App.md
│   ├── manufacturing/
│   │   ├── Odoo Manufacturing - Logique Metier.md
│   │   ├── Odoo Manufacturing - Parcours Utilisateur.md
│   │   ├── Odoo Manufacturing - UI UX.md
│   │   └── Odoo Manufacturing - Integrations Cross App.md
│   ├── plm/
│   │   ├── Odoo PLM - Logique Metier.md
│   │   ├── Odoo PLM - Parcours Utilisateur.md
│   │   ├── Odoo PLM - UI UX.md
│   │   └── Odoo PLM - Integrations Cross App.md
│   ├── purchase/
│   │   ├── Odoo Purchase - Logique Metier.md
│   │   ├── Odoo Purchase - Parcours Utilisateur.md
│   │   ├── Odoo Purchase - UI UX.md
│   │   └── Odoo Purchase - Integrations Cross App.md
│   ├── maintenance/
│   │   ├── Odoo Maintenance - Logique Metier.md
│   │   ├── Odoo Maintenance - Parcours Utilisateur.md
│   │   ├── Odoo Maintenance - UI UX.md
│   │   └── Odoo Maintenance - Integrations Cross App.md
│   └── quality/
│       ├── Odoo Quality - Logique Metier.md
│       ├── Odoo Quality - Parcours Utilisateur.md
│       ├── Odoo Quality - UI UX.md
│       └── Odoo Quality - Integrations Cross App.md
│
├── 05_hr/
│   ├── _index.md
│   ├── employees/
│   │   ├── Odoo Employees - Logique Metier.md
│   │   ├── Odoo Employees - Parcours Utilisateur.md
│   │   ├── Odoo Employees - UI UX.md
│   │   └── Odoo Employees - Integrations Cross App.md
│   ├── recruitment/
│   │   ├── Odoo Recruitment - Logique Metier.md
│   │   ├── Odoo Recruitment - Parcours Utilisateur.md
│   │   ├── Odoo Recruitment - UI UX.md
│   │   └── Odoo Recruitment - Integrations Cross App.md
│   ├── time_off/
│   │   ├── Odoo Time Off - Logique Metier.md
│   │   ├── Odoo Time Off - Parcours Utilisateur.md
│   │   ├── Odoo Time Off - UI UX.md
│   │   └── Odoo Time Off - Integrations Cross App.md
│   ├── appraisals/
│   │   ├── Odoo Appraisals - Logique Metier.md
│   │   ├── Odoo Appraisals - Parcours Utilisateur.md
│   │   ├── Odoo Appraisals - UI UX.md
│   │   └── Odoo Appraisals - Integrations Cross App.md
│   ├── referrals/
│   │   ├── Odoo Referrals - Logique Metier.md
│   │   ├── Odoo Referrals - Parcours Utilisateur.md
│   │   ├── Odoo Referrals - UI UX.md
│   │   └── Odoo Referrals - Integrations Cross App.md
│   └── fleet/
│       ├── Odoo Fleet - Logique Metier.md
│       ├── Odoo Fleet - Parcours Utilisateur.md
│       ├── Odoo Fleet - UI UX.md
│       └── Odoo Fleet - Integrations Cross App.md
│
├── 06_marketing/
│   ├── _index.md
│   ├── social_marketing/
│   │   ├── Odoo Social Marketing - Logique Metier.md
│   │   ├── Odoo Social Marketing - Parcours Utilisateur.md
│   │   ├── Odoo Social Marketing - UI UX.md
│   │   └── Odoo Social Marketing - Integrations Cross App.md
│   ├── email_marketing/
│   │   ├── Odoo Email Marketing - Logique Metier.md
│   │   ├── Odoo Email Marketing - Parcours Utilisateur.md
│   │   ├── Odoo Email Marketing - UI UX.md
│   │   └── Odoo Email Marketing - Integrations Cross App.md
│   ├── sms_marketing/
│   │   ├── Odoo SMS Marketing - Logique Metier.md
│   │   ├── Odoo SMS Marketing - Parcours Utilisateur.md
│   │   ├── Odoo SMS Marketing - UI UX.md
│   │   └── Odoo SMS Marketing - Integrations Cross App.md
│   ├── events/
│   │   ├── Odoo Events - Logique Metier.md
│   │   ├── Odoo Events - Parcours Utilisateur.md
│   │   ├── Odoo Events - UI UX.md
│   │   └── Odoo Events - Integrations Cross App.md
│   ├── marketing_automation/
│   │   ├── Odoo Marketing Automation - Logique Metier.md
│   │   ├── Odoo Marketing Automation - Parcours Utilisateur.md
│   │   ├── Odoo Marketing Automation - UI UX.md
│   │   └── Odoo Marketing Automation - Integrations Cross App.md
│   └── surveys/
│       ├── Odoo Surveys - Logique Metier.md
│       ├── Odoo Surveys - Parcours Utilisateur.md
│       ├── Odoo Surveys - UI UX.md
│       └── Odoo Surveys - Integrations Cross App.md
│
├── 07_services/
│   ├── _index.md
│   ├── project/
│   │   ├── Odoo Project - Logique Metier.md
│   │   ├── Odoo Project - Parcours Utilisateur.md
│   │   ├── Odoo Project - UI UX.md
│   │   └── Odoo Project - Integrations Cross App.md
│   ├── timesheet/
│   │   ├── Odoo Timesheet - Logique Metier.md
│   │   ├── Odoo Timesheet - Parcours Utilisateur.md
│   │   ├── Odoo Timesheet - UI UX.md
│   │   └── Odoo Timesheet - Integrations Cross App.md
│   ├── field_service/
│   │   ├── Odoo Field Service - Logique Metier.md
│   │   ├── Odoo Field Service - Parcours Utilisateur.md
│   │   ├── Odoo Field Service - UI UX.md
│   │   └── Odoo Field Service - Integrations Cross App.md
│   ├── helpdesk/
│   │   ├── Odoo Helpdesk - Logique Metier.md
│   │   ├── Odoo Helpdesk - Parcours Utilisateur.md
│   │   ├── Odoo Helpdesk - UI UX.md
│   │   └── Odoo Helpdesk - Integrations Cross App.md
│   ├── planning/
│   │   ├── Odoo Planning - Logique Metier.md
│   │   ├── Odoo Planning - Parcours Utilisateur.md
│   │   ├── Odoo Planning - UI UX.md
│   │   └── Odoo Planning - Integrations Cross App.md
│   └── appointments/
│       ├── Odoo Appointments - Logique Metier.md
│       ├── Odoo Appointments - Parcours Utilisateur.md
│       ├── Odoo Appointments - UI UX.md
│       └── Odoo Appointments - Integrations Cross App.md
│
└── 08_productivity/
    ├── _index.md
    ├── discuss/
    │   ├── Odoo Discuss - Logique Metier.md
    │   ├── Odoo Discuss - Parcours Utilisateur.md
    │   ├── Odoo Discuss - UI UX.md
    │   └── Odoo Discuss - Integrations Cross App.md
    ├── approvals/
    │   ├── Odoo Approvals - Logique Metier.md
    │   ├── Odoo Approvals - Parcours Utilisateur.md
    │   ├── Odoo Approvals - UI UX.md
    │   └── Odoo Approvals - Integrations Cross App.md
    ├── iot/
    │   ├── Odoo IoT - Logique Metier.md
    │   ├── Odoo IoT - Parcours Utilisateur.md
    │   ├── Odoo IoT - UI UX.md
    │   └── Odoo IoT - Integrations Cross App.md
    ├── voip/
    │   ├── Odoo VoIP - Logique Metier.md
    │   ├── Odoo VoIP - Parcours Utilisateur.md
    │   ├── Odoo VoIP - UI UX.md
    │   └── Odoo VoIP - Integrations Cross App.md
    ├── knowledge/
    │   ├── Odoo Knowledge - Logique Metier.md
    │   ├── Odoo Knowledge - Parcours Utilisateur.md
    │   ├── Odoo Knowledge - UI UX.md
    │   └── Odoo Knowledge - Integrations Cross App.md
    └── whatsapp/
        ├── Odoo WhatsApp - Logique Metier.md
        ├── Odoo WhatsApp - Parcours Utilisateur.md
        ├── Odoo WhatsApp - UI UX.md
        └── Odoo WhatsApp - Integrations Cross App.md
```

---

## Structure des documents par app

Chaque app aura 4 documents standardisés :

### 1. Logique Métier
- Modèles de données
- Règles métier
- Workflows
- Calculs et algorithmes
- États et transitions
- Validations

### 2. Parcours Utilisateur
- Scénarios d'usage
- Étapes détaillées
- Personas cibles
- Cas d'usage principaux
- Points de friction

### 3. UI/UX
- Interfaces utilisateur
- Composants visuels
- Navigation
- Responsive design
- Accessibilité
- Patterns d'interaction

### 4. Integrations Cross App
- Dépendances avec autres apps
- Flux de données inter-apps
- Mécanismes d'intégration
- APIs utilisées
- Événements partagés

---

## Méthodologie d'analyse

1. **Exploration du code source**
   - Analyse du dossier `addons/[app_name]/`
   - Lecture des modèles (`models/`)
   - Analyse des vues (`views/`)
   - Examen des contrôleurs (`controllers/`)
   - Review des dépendances (`__manifest__.py`)

2. **Extraction des informations**
   - Logique métier depuis les modèles Python
   - Parcours depuis les workflows et wizards
   - UI/UX depuis les templates et vues XML
   - Intégrations depuis les dépendances et hooks

3. **Documentation structurée**
   - Format standardisé par app
   - Exemples de code pertinents
   - Schémas et diagrammes
   - Comparaisons avec Miyukini

---

## Statut de l'analyse

**À faire :** Analyse complète du code source pour chaque app  
**Priorité :** Apps principales (Accounting, CRM, Sales, Inventory, Project)  
**Méthode :** Analyse progressive par catégorie

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
