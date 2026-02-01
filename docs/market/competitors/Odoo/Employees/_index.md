# Odoo Employees — Index de Documentation

## Contexte

Ce dossier contient l'**analyse complète** de l'application **Employees** (Employés) d'Odoo, réalisée selon la méthodologie standardisée. L'analyse couvre la logique métier, les parcours utilisateur, l'UI/UX, les intégrations, les spécifications Opérateurs Miyukini, l'intégration COG et les guides d'implémentation.

**Date d'analyse :** 2026-02-01  
**Source :** Documentation Odoo 19.0

---

## Structure de Documentation

### 00_logique_metier/
- **[Odoo Employees - Logique Métier Complète](./00_logique_metier/Odoo%20Employees%20-%20Logique%20Metier%20Complete.md)**
  - Modèles de données (hr.employee, hr.department, contrats, certifications, badges, équipements)
  - Règles métier et contraintes (hiérarchie, présence, droits)
  - Workflows (onboarding, offboarding, retention)
  - Gestion des compétences (skills, résumé, certifications)
  - Présence (attendances, statut utilisateur, contrôle avancé)
  - Organisation du travail (horaires, télétravail, lieux)

### 01_parcours_utilisateur/
- **[Odoo Employees - Parcours Utilisateur Détaillés](./01_parcours_utilisateur/Odoo%20Employees%20-%20Parcours%20Utilisateur%20Detailles.md)**
  - Personas et rôles (HR Officer, Manager, Employé, Administrateur)
  - Parcours d'onboarding et création de fiche employé
  - Scénarios : départements, certifications, badges, équipements, offboarding
  - Points de friction identifiés
  - Recommandations pour Miyukini

### 02_ui_ux/
- **[Odoo Employees - Analyse UI/UX](./02_ui_ux/Odoo%20Employees%20-%20Analyse%20UI%20UX.md)**
  - Structure de navigation et menus
  - Formulaire employé (onglets et sections)
  - Vues liste / Kanban employés et départements
  - Configuration (Settings)
  - Patterns d'interaction et feedback

### 03_integrations/
- **[Odoo Employees - Intégrations Cross-App](./03_integrations/Odoo%20Employees%20-%20Integrations%20Cross%20App.md)**
  - Dépendances avec autres modules Odoo (Payroll, Recruitment, Expenses, Time Off, Timesheets, Attendances, Appraisals, Planning, Fleet, Manufacturing)
  - Flux de données inter-apps
  - Mécanismes d'intégration (employé comme ressource centrale)
  - Recommandations pour Miyukini

### 04_specifications_miyukini/
- **[Odoo Employees - Spécifications Opérateurs Miyukini](./04_specifications_miyukini/Odoo%20Employees%20-%20Specifications%20Operateurs%20Miyukini.md)**
  - Architecture Opérateurs (7 Opérateurs identifiés)
  - Équipe d'Opérateurs EmployeeService (HR Base)
  - Contrat d'Équipe
  - Mandats de Permission (Standard, Édition, Offboarding)
  - Niveaux de sécurité (1–3 selon données)
  - Intégration avec les Cores

### 05_integration_cog/
- **[Odoo Employees - Guide Intégration COG](./05_integration_cog/Odoo%20Employees%20-%20Guide%20Integration%20COG.md)**
  - Architecture d'intégration COG
  - Patterns WriteIntent et Mandates (création, modification, offboarding, présence)
  - Exemples de code (pseudo-code Rust)
  - Gestion des erreurs et rollback
  - Intégration avec Kits existants (MiyuContacts, MiyuAttendances)

### 06_guides_implementation/
- **[Odoo Employees - Guide Implémentation](./06_guides_implementation/Odoo%20Employees%20-%20Guide%20Implementation.md)**
  - Architecture technique détaillée (crates miyuhr, miyuhr_skills, miyuhr_presence, miyuhr_equipment, miyuhr_offboarding, miyuhr_ui)
  - Schémas de données (Employee, Department, SkillType, PresenceStatus)
  - API et contrats
  - Plan de développement par phases (MVP → Complet)
  - Bornage fonctionnel
  - Critères d'acceptation
  - Risques et mitigation

---

## Résumé Exécutif

### Fonctionnalités Principales Identifiées

1. **Fiches employés**
   - Création/modification fiches (général, work information, private, payroll, settings)
   - Hiérarchie (manager, coach, département)
   - Lien utilisateur optionnel ; création utilisateur depuis fiche
   - Approbateurs par domaine (Expense, Time Off, Timesheet, Attendance)

2. **Départements**
   - Structure hiérarchique (manager, parent)
   - Propagation manager/coach sur fiche employé

3. **Compétences et résumé**
   - Skill Types (catégories, compétences, niveaux)
   - Attribution compétences aux employés
   - Lignes de résumé (expériences, éducation, certifications)

4. **Présence**
   - Trois modes : Based on attendances / Based on user status / Advanced (emails, IP)
   - Indicateurs sur cartes employés (lieu, statut)

5. **Télétravail et lieux**
   - Remote Work (lieu par jour)
   - Work Locations (référentiel)

6. **Équipements**
   - Attribution équipements aux employés
   - Récupération lors de l'offboarding

7. **Offboarding**
   - Processus de sortie (désactivation, récupération équipements, révocation mandats)
   - Rapport de rétention

### Architecture Miyukini Proposée

**7 Opérateurs :**
- EmployeeOperator (fiches employés)
- DepartmentOperator (départements)
- EmployeeSkillsOperator (compétences, résumé)
- EmployeePresenceOperator (présence)
- EmployeeEquipmentOperator (équipements)
- EmployeeOffboardingOperator (offboarding)
- EmployeeUI (interface)

**1 Équipe d'Opérateurs :** EmployeeService (HR Base)

**Correspondance Miyukini :** **MiyuHR** (ou **MiyukiniHR**) — EmployeeService

**Niveaux de sécurité :** 1–3 selon données (Standard à Critical)

**Intégration Cores :**
- StrongFather : Décisions (création, modification, offboarding)
- KindMother : Persistance (WriteIntent)
- Master Butler : Permissions (Officer, Manager, Employee self-edit)
- WorrySentinel : Sécurité (données privées, paie, offboarding)
- Caring Nanny : État présence
- Ever Buddy : Cycle de vie (actif/inactif, Skill Types)
- TAMR : Validation humaine offboarding

---

## Statut de l'Analyse

| Document | Statut | Version |
|----------|--------|---------|
| Logique Métier | ✅ Complété | 1.0 |
| Parcours Utilisateur | ✅ Complété | 1.0 |
| UI/UX | ✅ Complété | 1.0 |
| Intégrations Cross-App | ✅ Complété | 1.0 |
| Spécifications Opérateurs Miyukini | ✅ Complété | 1.0 |
| Guide Intégration COG | ✅ Complété | 1.0 |
| Guide Implémentation | ✅ Complété | 1.0 |

---

## Prochaines Étapes

1. **Valider les spécifications** : Revue avec équipe technique
2. **Démarrer l'implémentation** : Phase 1 (MVP) selon guide
3. **Itérer** : Selon feedback et besoins utilisateurs

---

**Document** : Odoo Employees — Index de Documentation  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : ✅ Analyse complète à 100% — référence pour implémentation Miyukini (MiyuHR)
