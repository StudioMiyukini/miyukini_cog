# Odoo Recruitment — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application Recruitment d'Odoo, en respectant l'architecture COG et la gouvernance Miyukini.

**Références :**
- [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- [Odoo Recruitment - Logique Métier](../00_logique_metier/Odoo%20Recruitment%20-%20Logique%20Metier%20Complete.md)

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour équivalents Recruitment
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores (StrongFather, KindMother, Master Butler, WorrySentinel)

**Hors scope :**
- Implémentation technique détaillée (voir Guide d'Implémentation)
- Spécifications UI/UX (document dédié)

---

## 1. Architecture Opérateurs

### 1.1 Vue d'Ensemble

L'équivalent Recruitment dans Miyukini s'appuie sur un **service COG Miyukini Recruitment** (ou **MiyukiniRecruitment**), avec des **Opérateurs spécialisés** pour le recrutement.

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **RecruitmentJob** | Gestion des postes à pourvoir | Opérateur de Service |
| **RecruitmentApplicant** | Gestion des candidatures et pipeline | Opérateur de Service |
| **RecruitmentStage** | Gestion des stages du pipeline | Opérateur de Service |
| **RecruitmentOffer** | Gestion des offres et délais (salary package) | Opérateur de Service |
| **RecruitmentSurvey** | Envoi et suivi des enquêtes d'entretien | Opérateur de Service |
| **RecruitmentReporting** | Rapports (Source, Velocity, Team performance) | Opérateur de Service |
| **RecruitmentUI** | Interface utilisateur recrutement | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : RecruitmentService

**Définition :**
> **RecruitmentService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de recrutement.**

**Composition :**
- RecruitmentJob (niveau sécurité 1–2)
- RecruitmentApplicant (niveau sécurité 2)
- RecruitmentStage (niveau sécurité 1)
- RecruitmentOffer (niveau sécurité 2–3)
- RecruitmentSurvey (niveau sécurité 2)
- RecruitmentReporting (niveau sécurité 1–2)
- RecruitmentUI (niveau sécurité 1)

**Contrat d'Équipe :** Voir section 2.

---

## 2. Opérateurs Détaillés

### 2.1 RecruitmentJob

**Rôle :** Gestion des postes à pourvoir (équivalent hr.job).

**Capacités :**
- Création / modification de postes
- Configuration département, lieu, type d'emploi, description, processus
- Gestion alias email, recruteur, intervieweurs, formulaire d'entretien, modèle de contrat
- Publication (intégration Website)

**Niveau de sécurité :** 1–2 (Standard à Sensitive selon données exposées)

**Gouvernance :**
- **KindMother** : Persistance des postes (WriteIntent)
- **Master Butler** : Permissions de création / modification
- **WorrySentinel** : Niveau sécurité, état système

**Contrat d'équipe :**
- Consomme : RecruitmentStage (stages par défaut ou spécifiques)
- Expose : `job.create`, `job.update`, `job.publish`, `job.get`

**Mandat de Permission requis :**
- Création / modification poste : Mandat avec RecruitmentJob

### 2.2 RecruitmentApplicant

**Rôle :** Gestion des candidatures et du pipeline (équivalent hr.applicant).

**Capacités :**
- Création / modification de candidatures
- Passage d'un stage à l'autre (transitions)
- Refus (motif, envoi email optionnel)
- Création d'employé depuis candidature (intégration HR)
- Gestion statuts par carte (In Progress, Blocked, Ready for Next Stage)

**Niveau de sécurité :** 2 (Sensitive) — données personnelles et salariales

**Gouvernance :**
- **StrongFather** : Décision de refus, décision de création d'employé
- **KindMother** : Persistance des candidatures (WriteIntent)
- **Master Butler** : Permissions de création / modification / passage de stage
- **WorrySentinel** : Données sensibles (CV, coordonnées, salaires)

**Contrat d'équipe :**
- Consomme : RecruitmentJob (poste), RecruitmentStage (stages), RecruitmentOffer (offre)
- Expose : `applicant.create`, `applicant.update`, `applicant.stage.move`, `applicant.refuse`, `applicant.create_employee`

**Mandat de Permission requis :**
- Création candidature : Mandat avec RecruitmentJob
- Passage de stage : Mandat avec RecruitmentApplicant + RecruitmentStage
- Refus : Mandat avec StrongFather (décision)
- Création employé : Mandat avec StrongFather + intégration HR (KindMother)

### 2.3 RecruitmentStage

**Rôle :** Gestion des stages du pipeline (équivalent hr.recruitment.stage).

**Capacités :**
- Création / modification / suppression de stages (sous contraintes : aucun candidat dans le stage à supprimer)
- Configuration template email (envoi auto à l'entrée), Folded, Hired Stage, Job Specific, Show in Referrals, Points
- Libellés des statuts (In Progress, Blocked, Ready for Next Stage)

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **KindMother** : Persistance des stages (WriteIntent)
- **Master Butler** : Permissions de configuration
- **Ever Buddy** : Compatibilité des changements (stages utilisés par des candidatures)

**Contrat d'équipe :**
- Consommé par : RecruitmentApplicant, RecruitmentJob
- Expose : `stage.create`, `stage.update`, `stage.delete`, `stage.get`, `stage.list`

### 2.4 RecruitmentOffer

**Rôle :** Gestion des offres et délais (salary package configurator).

**Capacités :**
- Configuration du nombre de jours de validité de l'offre (global ou par poste)
- Suivi des offres envoyées, dates d'expiration
- Prétentions / proposé (expected_salary, proposed_salary, avantages) — liaison avec candidature

**Niveau de sécurité :** 2–3 (Sensitive à Critical pour montants)

**Gouvernance :**
- **KindMother** : Persistance des données d'offre (WriteIntent)
- **Master Butler** : Permissions
- **WorrySentinel** : Données salariales sensibles

**Contrat d'équipe :**
- Consommé par : RecruitmentApplicant
- Expose : `offer.config`, `offer.get`, `offer.expiry.check`

### 2.5 RecruitmentSurvey

**Rôle :** Envoi et suivi des enquêtes d'entretien (Send Interview Survey).

**Capacités :**
- Envoi d'une enquête (survey) au candidat (email avec lien)
- Liaison enquête ↔ candidature (réponses, date limite)
- Intégration avec Opérateur Surveys / Forms (MiyuPolls ou équivalent)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **Master Butler** : Permissions d'envoi
- **KindMother** : Traçabilité envoi / réponses (WriteIntent si stockage local)
- Intégration externe : Opérateur Surveys pour exécution des enquêtes

**Contrat d'équipe :**
- Consomme : RecruitmentApplicant (email, candidat), Opérateur Surveys, Opérateur Mail (MiyuNotify)
- Expose : `survey.send`, `survey.status`

### 2.6 RecruitmentReporting

**Rôle :** Rapports (Source analysis, Velocity, Team performance).

**Capacités :**
- Agrégations par source, medium, campagne (UTM)
- Délais (velocity) : temps par stage, temps total
- Performance par recruteur, par poste
- Export (rapports, tableaux)

**Niveau de sécurité :** 1–2 (données agrégées ; données personnelles protégées)

**Gouvernance :**
- **Master Butler** : Permissions de consultation des rapports
- **WorrySentinel** : Pas d'exposition de données personnelles identifiantes dans les rapports non autorisés

**Contrat d'équipe :**
- Consomme : RecruitmentApplicant (données agrégées), RecruitmentJob
- Expose : `report.source`, `report.velocity`, `report.team_performance`

### 2.7 RecruitmentUI

**Rôle :** Interface utilisateur recrutement (tableau de bord, Kanban, formulaires).

**Capacités :**
- Tableau de bord postes (Kanban)
- Kanban candidatures par poste (stages, Quick Add, glisser-déposer)
- Formulaires poste et candidat (onglets, chatter, affichage CV)
- Configuration paramètres et stages

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **Master Butler** : Permissions d'accès aux écrans
- **WorrySentinel** : Affichage conditionnel des données sensibles (CV, salaires) selon rôle

**Contrat d'équipe :**
- Consomme : RecruitmentJob, RecruitmentApplicant, RecruitmentStage, RecruitmentOffer, RecruitmentSurvey, RecruitmentReporting
- Expose : écrans et actions UI

---

## 3. Contrat d'Équipe RecruitmentService

**Flux autorisés :**
- RecruitmentUI → RecruitmentJob, RecruitmentApplicant, RecruitmentStage, RecruitmentOffer, RecruitmentSurvey, RecruitmentReporting
- RecruitmentApplicant → RecruitmentJob, RecruitmentStage, RecruitmentOffer ; → StrongFather (refuse, create_employee) ; → KindMother (WriteIntent)
- RecruitmentJob → RecruitmentStage (stages par défaut ou spécifiques)
- RecruitmentSurvey → RecruitmentApplicant, Mail, Surveys (externe)
- RecruitmentReporting → RecruitmentApplicant, RecruitmentJob (lecture agrégée)

**Types de données échangeables :**
- Poste : identifiant, libellé, département, lieu, type d'emploi, alias, recruteur, intervieweurs, etc.
- Candidature : identifiant, candidat, email, téléphone, poste, stage, statut, sourcing (UTM), référent, offres (montants sous contrôle)
- Stage : identifiant, libellé, séquence, template email, hired, folded, job_ids, points Referrals
- Rapports : agrégations, pas de données personnelles identifiantes sans autorisation

**Conditions préalables :**
- Mandat de Permission valide pour l'équipe RecruitmentService (StrongFather)
- Niveau de confiance système T0–T2 (WorrySentinel)

**Niveau de validation requis :**
- Création / modification poste : validation Master Butler
- Création / modification candidature : validation Master Butler ; refus / création employé : validation StrongFather
- Modification / suppression stage : validation Master Butler + contraintes Ever Buddy (candidatures dans le stage)

---

## 4. Mandats de Permission

**Mandat Standard (Recrutement quotidien) :**
- RecruitmentJob : lecture, création, modification (postes)
- RecruitmentApplicant : lecture, création, modification, passage de stage (candidatures)
- RecruitmentStage : lecture
- RecruitmentOffer : lecture, configuration délais
- RecruitmentSurvey : envoi enquêtes (si droits)
- RecruitmentReporting : consultation rapports
- Durée : session ou délai défini ; révocation : fin de mission, changement de rôle, alerte WorrySentinel

**Mandat Validation (Refus / Embauche) :**
- StrongFather : décision de refus, décision de création d'employé
- Délégation possible selon politique (seuils, rôle)
- Révocation : idem Mandat Standard

**Mandat Configuration (Paramètres, stages) :**
- RecruitmentStage : création, modification, suppression (sous contraintes)
- Paramètres globaux (Process, Résumé Display, Salary package, etc.)
- Réservé aux rôles RH / Admin selon politique

---

## 5. Intégration avec les Cores

- **StrongFather** : Décision de refus candidat, décision de création d'employé depuis candidature.
- **KindMother** : Persistance Job, Applicant, Stage, Offer (WriteIntent) ; traçabilité.
- **Master Butler** : Permissions (recruteur, officer, intervieweur, RH, admin) ; capacités par opérateur.
- **WorrySentinel** : Niveau de sécurité 1–3 selon données (postes publics vs. données personnelles/salaires) ; état système T0–T2 pour opérations sensibles.
- **Ever Buddy** : Gestion des stages (évolution, dépréciation) ; compatibilité candidatures existantes.
- **BondingBrother** : Médiation des demandes UI → Opérateurs et Cores (pas d'autorité).

---

## 6. Correspondance Miyukini

**Service Miyukini proposé :** **MiyukiniRecruitment** (ou **MiyuRecruitment**) — RecruitmentService

**Opérateurs :** RecruitmentJob, RecruitmentApplicant, RecruitmentStage, RecruitmentOffer, RecruitmentSurvey, RecruitmentReporting, RecruitmentUI

**Équipe d'Opérateurs :** RecruitmentService

**Niveaux de sécurité :** 1 (Standard) à 3 (Critical) selon données (postes, candidatures, salaires).

---

**Document** : Odoo Recruitment — Spécifications Opérateurs Miyukini  
**Version** : 1.0  
**Date** : 2026-02-01
