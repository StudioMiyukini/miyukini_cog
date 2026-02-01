# Odoo Recruitment — Index de Documentation

## Contexte

Ce dossier contient l'**analyse complète** de l'application **Recruitment** (Recrutement) d'Odoo, réalisée selon la méthodologie standardisée. L'analyse couvre la logique métier, les parcours utilisateur, l'UI/UX, les intégrations, les spécifications Opérateurs Miyukini, l'intégration COG et les guides d'implémentation.

**Date d'analyse :** 2026-02-01  
**Source :** Documentation Odoo 18.0/19.0 — Applications HR / Recruitment

---

## Structure de Documentation

### 00_logique_metier/
- **[Odoo Recruitment - Logique Métier Complète](./00_logique_metier/Odoo%20Recruitment%20-%20Logique%20Metier%20Complete.md)**
  - Modèles de données (hr.job, hr.applicant, hr.recruitment.stage)
  - Règles métier et contraintes
  - Workflow de recrutement (6 stages par défaut)
  - Gestion candidatures (création, progression, refus, embauche)
  - Intégration CV, enquêtes, templates email, sourcing (UTM), Referrals
  - Points d'attention pour Miyukini

### 01_parcours_utilisateur/
- **[Odoo Recruitment - Parcours Utilisateur Détaillés](./01_parcours_utilisateur/Odoo%20Recruitment%20-%20Parcours%20Utilisateur%20Detailles.md)**
  - Personas et rôles (Recruteur, RH, Intervieweur, Candidat, Référent)
  - Parcours d'onboarding (configuration, première utilisation)
  - Scénarios d'usage principaux
  - Points de friction identifiés
  - Recommandations pour Miyukini

### 02_ui_ux/
- **[Odoo Recruitment - Analyse UI/UX](./02_ui_ux/Odoo%20Recruitment%20-%20Analyse%20UI%20UX.md)**
  - Tableau de bord postes (Kanban)
  - Kanban candidatures par stage (Quick Add, statuts, colonnes repliées)
  - Formulaires poste et candidat (onglets, chatter, affichage CV)
  - Paramètres et configuration des stages
  - Patterns de navigation et communication

### 03_integrations/
- **[Odoo Recruitment - Intégrations Cross-App](./03_integrations/Odoo%20Recruitment%20-%20Integrations%20Cross%20App.md)**
  - Intégration avec HR (Employees) — Create Employee
  - Intégration avec Mail (chatter, templates, envoi auto par stage)
  - Intégration avec Documents (CV, Résumé Display, OCR IAP)
  - Intégration avec Website (publication postes, formulaire candidature)
  - Intégration avec Surveys (Send Interview)
  - Intégration avec Referrals (référents, points par stage)
  - UTM (sourcing, rapports), Alias email, IAP (SMS, OCR)

### 04_specifications_miyukini/
- **[Odoo Recruitment - Spécifications Opérateurs Miyukini](./04_specifications_miyukini/Odoo%20Recruitment%20-%20Specifications%20Operateurs%20Miyukini.md)**
  - Architecture Opérateurs (7 Opérateurs identifiés)
  - Équipe d'Opérateurs RecruitmentService
  - Contrat d'Équipe
  - Mandats de Permission (Standard, Validation, Configuration)
  - Niveaux de sécurité (1–3 selon données)
  - Intégration avec les Cores

### 05_integration_cog/
- **[Odoo Recruitment - Guide Intégration COG](./05_integration_cog/Odoo%20Recruitment%20-%20Guide%20Integration%20COG.md)**
  - Architecture d'intégration COG
  - Patterns d'implémentation (WriteIntent, Mandats, Refus, Création employé)
  - Exemples de code (pseudo-code Rust)
  - Gestion des erreurs et rollback
  - Intégration avec Kits existants (MiyuHR, MiyuNotify, MiyuDocuments, MiyuWeb, MiyuPolls)

### 06_guides_implementation/
- **[Odoo Recruitment - Guide Implémentation](./06_guides_implementation/Odoo%20Recruitment%20-%20Guide%20Implementation.md)**
  - Architecture technique (crates)
  - Schémas de données (Job, Applicant, Stage)
  - API et contrats
  - Plan de développement par phases (MVP → Intégrations → Reporting → Optionnel)
  - Bornage fonctionnel et critères d'acceptation
  - Risques et mitigation

---

## Résumé Exécutif

### Fonctionnalités Principales Identifiées

1. **Postes (hr.job)**
   - Création / modification de postes, alias email, recruteur, intervieweurs
   - Publication sur le site, formulaire d'entretien, modèle de contrat

2. **Candidatures (hr.applicant)**
   - Pipeline par stages (New → Initial Qualification → First Interview → Second Interview → Contract Proposal → Contract Signed)
   - Création manuelle ou automatique (formulaire site, alias email)
   - Statuts par carte : In Progress, Blocked, Ready for Next Stage
   - Refus avec motif et email optionnel
   - Création employé depuis candidature (stage Contract Signed / Hired)

3. **Stages (hr.recruitment.stage)**
   - Configuration globale ou par poste, template email (envoi auto), Hired Stage, Referrals (points)

4. **Intégrations**
   - HR (Create Employee), Mail (templates, chatter), Documents (CV), Website (publication, formulaire), Surveys (entretiens), Referrals (référents, points), UTM (sourcing), IAP (SMS, OCR)

### Architecture Miyukini Proposée

**7 Opérateurs :**
- RecruitmentJob (postes)
- RecruitmentApplicant (candidatures, pipeline)
- RecruitmentStage (stages)
- RecruitmentOffer (offres, délais)
- RecruitmentSurvey (enquêtes d'entretien)
- RecruitmentReporting (rapports Source, Velocity, Team performance)
- RecruitmentUI (interface)

**1 Équipe d'Opérateurs :** RecruitmentService

**Correspondance Miyukini :** **MiyukiniRecruitment** (ou **MiyuRecruitment**) — RecruitmentService

**Niveaux de sécurité :** 1 (Standard) à 3 (Critical) selon données (postes, candidatures, salaires)

**Intégration Cores :**
- StrongFather : Décisions (refus, création employé)
- KindMother : Persistance (WriteIntent Job, Applicant, Stage, Offer)
- Master Butler : Permissions (recruteur, officer, intervieweur, RH, admin)
- WorrySentinel : Données sensibles (CV, coordonnées, salaires)
- Ever Buddy : Évolution des stages

---

## Statut de l'Analyse

| Document | Statut | Version |
|----------|--------|--------|
| Logique Métier | ✅ Complété | 1.0 |
| Parcours Utilisateur | ✅ Complété | 1.0 |
| UI/UX | ✅ Complété | 1.0 |
| Intégrations Cross-App | ✅ Complété | 1.0 |
| Spécifications Opérateurs Miyukini | ✅ Complété | 1.0 |
| Guide Intégration COG | ✅ Complété | 1.0 |
| Guide Implémentation | ✅ Complété | 1.0 |

---

**Document** : Odoo Recruitment — Index de Documentation  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : ✅ Analyse complète à 100% — référence pour implémentation Miyukini (MiyukiniRecruitment / MiyuRecruitment)
