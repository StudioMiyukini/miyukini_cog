# Odoo Time Off — Index de l'Analyse

## Statut

✅ **Analyse complète à 100% (7/7 documents)**

---

## Documents de l'Analyse

### 1. Logique Métier
📄 [Odoo Time Off - Logique Metier Complete.md](./00_logique_metier/Odoo%20Time%20Off%20-%20Logique%20Metier%20Complete.md)

**Contenu :**
- Modèles de données (hr.leave.type, hr.leave, hr.leave.allocation, hr.leave.accrual, jours fériés, jours obligatoires)
- Règles métier et contraintes
- Workflows et transitions d'état (demandes, allocations)
- Calculs (jours/heures, solde, accrual)
- Intégrations avec HR, Resource, Calendar, Payroll, Timesheet

### 2. Parcours Utilisateur
📄 [Odoo Time Off - Parcours Utilisateur Detailles.md](./01_parcours_utilisateur/Odoo%20Time%20Off%20-%20Parcours%20Utilisateur%20Detailles.md)

**Contenu :**
- Personas (Employé, Manager/Approbateur, Time Off Officer, Administrateur)
- Parcours d'onboarding
- Scénarios d'usage principaux (poser une demande, valider, créer allocations, Overview, rapports)
- Points de friction identifiés
- Recommandations pour Miyukini

### 3. UI/UX
📄 [Odoo Time Off - Analyse UI UX.md](./02_ui_ux/Odoo%20Time%20Off%20-%20Analyse%20UI%20UX.md)

**Contenu :**
- Sections principales (My Time Off, Overview, Management, Configuration, Reporting)
- Vues List, Form, Calendar, Dashboard
- Widgets et patterns de navigation
- Formulaires et wizards (demande, annulation, batch allocations)
- Recommandations pour Miyukini

### 4. Intégrations Cross-App
📄 [Odoo Time Off - Integrations Cross App.md](./03_integrations/Odoo%20Time%20Off%20-%20Integrations%20Cross%20App.md)

**Contenu :**
- Dépendances (hr, calendar, resource ; optionnel mail, hr_payroll, hr_timesheet)
- Flux de données inter-apps
- Mécanismes d'intégration (employé, calendrier, jours fériés, événements, work entries, timesheet)
- Recommandations pour Miyukini

### 5. Spécifications Opérateurs Miyukini
📄 [Odoo Time Off - Specifications Operateurs Miyukini.md](./04_specifications_miyukini/Odoo%20Time%20Off%20-%20Specifications%20Operateurs%20Miyukini.md)

**Contenu :**
- Opérateurs identifiés (LeaveRequestOperator, LeaveAllocationOperator, LeaveTypeOperator, LeaveAccrualOperator, LeaveCalendarOperator, TimeOffUI)
- Contrat d'équipe TimeOffService et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

### 6. Guide Intégration COG
📄 [Odoo Time Off - Guide Integration COG.md](./05_integration_cog/Odoo%20Time%20Off%20-%20Guide%20Integration%20COG.md)

**Contenu :**
- Architecture d'intégration COG
- Patterns WriteIntent et Mandates
- Exemples de code pseudo-Rust (création demande, validation, batch allocations, solde)
- Intégrations post-validation (Calendar, Payroll, Timesheet)

### 7. Guide Implémentation
📄 [Odoo Time Off - Guide Implementation.md](./06_guides_implementation/Odoo%20Time%20Off%20-%20Guide%20Implementation.md)

**Contenu :**
- Architecture technique détaillée
- Spécifications des crates Rust (miyutimeoff, miyutimeoff-ui)
- Schémas de données (LeaveRequest, LeaveAllocation, LeaveType, Accrual, PublicHoliday, MandatoryDay)
- API et contrats
- Plan de développement par phases (MVP → Complet → Avancé)
- Bornage fonctionnel

---

## Service Miyukini Proposé

**Nom :** `MiyukiniTimeOff` ou `MiyuTimeOff`

**Opérateurs :**
- **LeaveRequestOperator** : Gestion des demandes de congé
- **LeaveAllocationOperator** : Gestion des allocations et soldes
- **LeaveTypeOperator** : Gestion des types de congé
- **LeaveAccrualOperator** : Gestion des plans d'acquisition
- **LeaveCalendarOperator** : Jours fériés et jours obligatoires
- **TimeOffUI** : Interface utilisateur Time Off

**Équipe d'Opérateurs :** TimeOffService

---

## Source d'Analyse

**Repository :** `https://github.com/odoo/odoo/tree/19.0/addons/hr_holidays`

**Documentation :** Odoo 19.0 — Time Off

**Version analysée :** Odoo 19.0

**Date d'analyse :** 2026-02-01

---

## Notes

- Application HR centrée sur les congés et absences
- Intégrations multiples (HR, Resource, Calendar, Payroll, Timesheet, Mail)
- Workflow de validation (simple ou double) et calcul du solde essentiels
- Plans d'acquisition (accrual) et jours fériés/jours obligatoires pour couverture complète
