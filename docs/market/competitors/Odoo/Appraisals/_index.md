# Odoo Appraisals — Index de Documentation

## Contexte

Ce dossier contient l'**analyse complète** de l'application **Appraisals** (Évaluations / Entretiens annuels) d'Odoo, réalisée selon la méthodologie standardisée. L'analyse couvre la logique métier, les parcours utilisateur, l'UI/UX, les intégrations, les spécifications Opérateurs Miyukini, l'intégration COG et les guides d'implémentation.

**Date d'analyse :** 2026-02-01  
**Source :** Documentation Odoo 18.0/19.0

---

## Structure de Documentation

### 00_logique_metier/
- **[Odoo Appraisals - Logique Métier Complète](./00_logique_metier/Odoo%20Appraisals%20-%20Logique%20Metier%20Complete.md)**
  - Modèles de données (appraisal, plan, template, goals, 360, evaluation scale)
  - Règles métier et contraintes (planification, visibilité, final rating)
  - Workflow (planification → auto-évaluation → feedback manager → revue → clôture)
  - Goals et Skills dans le cadre des évaluations
  - 360 Feedback et templates
  - Analyse et rapports (appraisal analysis, skills evolution)

### 01_parcours_utilisateur/
- **[Odoo Appraisals - Parcours Utilisateur Détaillés](./01_parcours_utilisateur/Odoo%20Appraisals%20-%20Parcours%20Utilisateur%20Detailles.md)**
  - Personas et rôles (HR/Admin, Manager, Employé)
  - Parcours de planification (automatique et manuelle)
  - Scénarios : auto-évaluation, feedback manager, 360, réunion, clôture, objectifs
  - Points de friction identifiés
  - Recommandations pour Miyukini

### 02_ui_ux/
- **[Odoo Appraisals - Analyse UI/UX](./02_ui_ux/Odoo%20Appraisals%20-%20Analyse%20UI%20UX.md)**
  - Structure de navigation et menus
  - Dashboard et cartes Appraisals
  - Formulaire appraisal (onglets Appraisal, Skills, Private Note)
  - Goals, Configuration (Templates, Evaluation Scale, 360, Tags)
  - Patterns d'interaction (toggles visibilité, Confirm, Mark as Done, Ask Feedback, Meetings)
  - Analyse et rapports

### 03_integrations/
- **[Odoo Appraisals - Intégrations Cross-App](./03_integrations/Odoo%20Appraisals%20-%20Integrations%20Cross%20App.md)**
  - Dépendances (Employees, Mail, Surveys, Calendar)
  - Flux de données inter-apps
  - Mécanismes d'intégration (employé, templates Surveys, 360, réunion)
  - Recommandations pour Miyukini

### 04_specifications_miyukini/
- **[Odoo Appraisals - Spécifications Opérateurs Miyukini](./04_specifications_miyukini/Odoo%20Appraisals%20-%20Specifications%20Operateurs%20Miyukini.md)**
  - Architecture Opérateurs (6 Opérateurs identifiés)
  - Équipe d'Opérateurs AppraisalService
  - Contrat d'Équipe
  - Mandats de Permission
  - Niveaux de sécurité (1–3 selon données)
  - Intégration avec les Cores et EmployeeService (MiyuHR)

### 05_integration_cog/
- **[Odoo Appraisals - Guide Intégration COG](./05_integration_cog/Odoo%20Appraisals%20-%20Guide%20Integration%20COG.md)**
  - Architecture d'intégration COG
  - Patterns WriteIntent et Mandates (création, confirmation, feedback, rating, clôture, goals, 360)
  - Exemples de code (pseudo-code Rust)
  - Gestion des erreurs et rollback
  - Intégration avec Kits existants (MiyuHR, MiyuNotify, MiyuSurveys, MiyuPlanning)

### 06_guides_implementation/
- **[Odoo Appraisals - Guide Implémentation](./06_guides_implementation/Odoo%20Appraisals%20-%20Guide%20Implementation.md)**
  - Architecture technique détaillée (crates miyuappraisals, miyuappraisals_templates, miyuappraisals_goals, miyuappraisals_360, miyuappraisals_ui)
  - Schémas de données (Appraisal, Goal, Plan, Template, EvaluationScale)
  - API et contrats
  - Plan de développement par phases (MVP → Complet)
  - Bornage fonctionnel
  - Critères d'acceptation
  - Risques et mitigation

---

## Résumé Exécutif

### Fonctionnalités Principales Identifiées

1. **Cycle d'évaluation**
   - Planification (automatique 6m/6m/12m ou manuelle)
   - Auto-évaluation (Employee's Feedback : My work, My future, My feelings)
   - Feedback manager (Feedback, Evaluation, Improvements)
   - Toggles visibilité (visible to manager / visible to employee)
   - Réunion (Meeting) et revue Skills / Goals
   - Final Rating et Private Note (managers uniquement)
   - Mark as Done / Reopen ; mise à jour next_appraisal_date et skills

2. **Objectifs (Goals)**
   - CRUD (avancement 0–100 %, deadline, tags, description)
   - Mark as Done ; revue pendant l'appraisal

3. **Templates et échelle**
   - Modèles d'évaluation (sections employé/manager) ; templates par département
   - Échelle de notation personnalisable (Evaluation Scale)

4. **360 Feedback**
   - Demande de retours (Ask Feedback) ; envoi email ; dashboard et résultats

5. **Analyse et rapports**
   - Appraisal analysis (filtres par statut)
   - Skills evolution (amélioration, compétences ciblées)

### Architecture Miyukini Proposée

**6 Opérateurs :**
- AppraisalOperator (cycle d'évaluation)
- AppraisalPlanOperator (planification automatique)
- AppraisalTemplateOperator (modèles d'évaluation)
- AppraisalGoalsOperator (objectifs)
- Appraisal360Operator (360 Feedback)
- AppraisalUI (interface)

**1 Équipe d'Opérateurs :** AppraisalService

**Correspondance Miyukini :** **MiyuAppraisals** (ou **MiyukiniAppraisals**) — AppraisalService

**Niveaux de sécurité :** 1–3 selon données (Standard à Critical pour Private Note et rating)

**Intégration Cores :**
- StrongFather : Décisions (création, confirmation, clôture, 360)
- KindMother : Persistance (WriteIntent)
- Master Butler : Permissions (Manager, Employé, HR/Admin)
- WorrySentinel : Sécurité (Private Note, rating)
- TAMR : Intervention humaine (réunion, décision finale)
- Ever Buddy : Cycle de vie (statuts, templates)

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

**Document** : Odoo Appraisals — Index de Documentation  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : ✅ Analyse complète à 100% — référence pour implémentation Miyukini (MiyuAppraisals)
