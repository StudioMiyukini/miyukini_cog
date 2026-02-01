# Odoo Recruitment — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Recruitment** d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Documentation Odoo 18.0/19.0 — Applications HR / Recruitment

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres apps Odoo
- Flux de données inter-apps
- Mécanismes d'intégration (alias email, Documents, Mail, Surveys, Website, Referrals, UTM)
- Événements et hooks typiques

---

## 1. Dépendances Principales

### 1.1 Modules Requis (typiques)

**Dépendances explicites (manifest) :**
- `base` : Partenaires, sociétés, utilisateurs
- `hr` : Employés, départements, compétences (hr.employee, hr.department, hr.skill)
- `mail` : Chatter, activités, modèles d'email
- `utm` : Source, medium, campagne (sourcing et reporting)
- `web` : Framework web

### 1.2 Modules Optionnels

**Dépendances optionnelles :**
- `hr_recruitment` (core recruitment) : Postes, candidatures, stages
- `documents` : Stockage des CV et pièces jointes (dossier Recruitment)
- `website` : Publication des postes, formulaire de candidature en ligne
- `survey` : Enquêtes / formulaires d'entretien (Send Interview Survey)
- `hr_referral` : Références employés, points (Referred By User, Show in Referrals, Points par stage)
- `hr_contract` / `hr_contract_salary` : Modèles de contrat, offre salariale
- `iap` : SMS, OCR CV (In-App Purchases, crédits)
- `portal` : Portail candidat (consultation statut, documents) si activé

---

## 2. Intégrations Détaillées

### 2.1 Intégration avec HR (Employees)

**Flux :**
```
hr.applicant (Contract Signed) → Create Employee → hr.employee
```

**Mécanismes :**
- Action « Create Employee » depuis la fiche candidat au stage Contract Signed (ou Hired).
- Création d’un enregistrement `hr.employee` à partir des données candidat (nom, email, téléphone, département, société, etc.).
- Liaison possible au partenaire (`res.partner`) si existant ou création.
- Données héritées : département, société, poste (job_id), recruteur, etc.

**Champs liés :**
- `hr.employee` : name, work_email, work_phone, department_id, job_id, company_id, address_id, etc.
- `hr.applicant` : partner_name, email_from, partner_phone, job_id, department_id, company_id, etc.

**Recommandations pour Miyukini :**
- Intégration native avec Opérateur HR / Employees (MiyuHR).
- Création d’employé gouvernée par StrongFather (décision) et KindMother (WriteIntent).
- Lien candidature → employé tracé et révocable selon politique.

### 2.2 Intégration avec Mail

**Flux :**
```
hr.applicant ↔ chatter (messages, activités)
hr.recruitment.stage → template_id → envoi email automatique à l'entrée dans le stage
```

**Mécanismes :**
- Chatter sur `hr.applicant` : messages, activités, historique, pièces jointes.
- Modèles d’email (mail.template) : Applicant Acknowledgement, Interest, Schedule Interview, Refuse, Not interested anymore, etc.
- Envoi automatique : à l’entrée dans un stage, si un `template_id` est configuré sur le stage.
- Envoi manuel : « Send message » / Full composer depuis le chatter ; choix du template, pièces jointes.

**Champs liés :**
- `hr.recruitment.stage.template_id` : Modèle d’email pour envoi auto.
- `hr.applicant.email_from` : Destinataire principal (requis pour envoi).

**Recommandations pour Miyukini :**
- Intégration avec Opérateur Mail / Notify (MiyuNotify) pour envoi et traçabilité.
- Templates gouvernés (contenu, destinataires) et audit des envois automatiques.

### 2.3 Intégration avec Documents

**Flux :**
```
CV / pièces jointes (candidature, formulaire site, email alias) → Documents app (dossier Recruitment)
```

**Mécanismes :**
- Les CV et pièces jointes des candidatures sont stockés dans l’app Documents (dossier « Recruitment »).
- Accès depuis la fiche candidat : section Fichiers du chatter, ou affichage à droite (Résumé Display) si activé en paramètres.
- Option « Résumé Digitization (OCR) » : extraction nom, email, téléphone depuis le CV (IAP, crédits).

**Champs / concepts liés :**
- `documents.document` (ou équivalent) : stockage des fichiers.
- Dossier « Recruitment » : regroupement des CV et pièces par recrutement.
- Lien candidature ↔ document (attachment ou relation dédiée).

**Recommandations pour Miyukini :**
- Intégration avec Opérateur Documents / Media (MiyuDocuments ou MiyuMedia) pour stockage et affichage.
- Politique de rétention et confidentialité (WorrySentinel) pour les CV.

### 2.4 Intégration avec Website

**Flux :**
```
Publication poste (hr.job) → Page site → Formulaire candidature → création hr.applicant
```

**Mécanismes :**
- Publication des postes sur le site : liste des offres, page détail (Job Summary, Process Details, lieu, type d’emploi, etc.).
- Formulaire de candidature en ligne : nom, email, téléphone, CV, présentation courte ; soumission → création automatique de `hr.applicant` pour le poste concerné.
- Champs requis du formulaire configurables (selon personnalisation du site).

**Champs liés :**
- `hr.job` : website_published, job_summary, address_id, employment_type, process_details (Time to Answer, Process, Days to get an Offer).
- `hr.applicant` : job_id, partner_name, email_from, partner_phone, description, pièces jointes (CV).

**Recommandations pour Miyukini :**
- Intégration avec Opérateur Website (MiyuWeb) pour publication des postes et formulaire de candidature.
- Création de candidature depuis le site gouvernée (validation, anti-spam) et tracée.

### 2.5 Intégration avec Surveys

**Flux :**
```
hr.job.interview_form_id (survey.survey) → Send Interview (fiche candidat) → email avec lien survey → survey.user_input
```

**Mécanismes :**
- Formulaire d’entretien rattaché au poste (`interview_form_id`) : enquête (survey.survey) utilisée comme questionnaire / certification / test.
- Envoi manuel « Send Interview » depuis la fiche candidat : popup (destinataires, modèle d’email, pièces, date limite) → envoi email avec lien vers l’enquête.
- Réponses stockées dans l’app Surveys (survey.user_input) ; lien possible vers la candidature pour suivi.
- Paramètre global « Send Interview Survey » : active la fonctionnalité ; installe l’app Surveys si nécessaire.

**Champs liés :**
- `hr.job.interview_form_id` : Many2one survey.survey.
- `hr.applicant` : email_from (requis pour envoi du lien).
- Lien survey.user_input ↔ hr.applicant (selon implémentation Odoo).

**Recommandations pour Miyukini :**
- Intégration avec Opérateur Surveys / Forms (MiyuPolls ou équivalent) pour enquêtes d’entretien.
- Envoi du lien gouverné (permissions, quota) et tracé ; réponses associées à la candidature.

### 2.6 Intégration avec Referrals (hr_referral)

**Flux :**
```
hr.applicant.referrer_id (employé référent) + stage « Show in Referrals » + Points → attribution de points au référent
```

**Mécanismes :**
- Champ « Referred By User » sur la candidature : sélection de l’employé ayant référé le candidat.
- Sur un stage : options « Show in Referrals » et « Points ». Quand une candidature entre dans ce stage, le référent gagne les points configurés.
- L’app Referrals affiche les recommandations et les points par employé.

**Champs liés :**
- `hr.applicant.referrer_id` : Many2one res.users (employé référent).
- `hr.recruitment.stage` : show_in_referral (bool), points (integer).
- Référentiel Referrals : recommandations, historique, solde de points.

**Recommandations pour Miyukini :**
- Intégration avec Opérateur Referrals (Miyukini Referrals ou équivalent) pour attribution de points sous gouvernance (StrongFather, Master Butler).
- Traçabilité recommandations ↔ candidature ↔ employé.

### 2.7 Intégration avec UTM (sourcing)

**Flux :**
```
Source / Medium / Campaign (formulaire site, lien campagne) → hr.applicant.utm_source_id, utm_medium_id, utm_campaign_id
→ Rapports Source analysis, Velocity, etc.
```

**Mécanismes :**
- Champs UTM sur la candidature : utm_source_id, utm_medium_id, utm_campaign_id (utm.mixin).
- Renseignés automatiquement si le candidat arrive via un lien tracé (site, email marketing, job board), ou manuellement.
- Rapports Recruitment : analyse par source, medium, campagne ; délais (velocity) ; performance par recruteur (team performance).

**Recommandations pour Miyukini :**
- Conserver les champs UTM sur l’entité Candidature pour reporting et attribution.
- Intégration avec Opérateur Marketing / UTM si existant pour cohérence des campagnes.

### 2.8 Alias email (création automatique de candidatures)

**Flux :**
```
Email envoyé à l'alias du poste (hr.job.alias_id) → parsing (expéditeur, pièces jointes) → création hr.applicant
```

**Mécanismes :**
- Chaque poste peut avoir un alias email (ex. jobs-sales@company.com). Les emails reçus sur cet alias déclenchent la création d’une candidature pour ce poste.
- Données extraites : expéditeur (email, nom), pièces jointes (CV), corps du message (optionnel).
- Option « Résumé Digitization (OCR) » : extraction nom, email, téléphone depuis le CV joint (IAP).

**Recommandations pour Miyukini :**
- Intégration avec Opérateur Mail / Inbound (MiyuNotify ou service d’entrée email) pour parsing et création de candidature gouvernée (validation, déduplication, anti-spam).

### 2.9 IAP (SMS, OCR)

**Flux :**
```
Send SMS (fiche candidat) → crédits IAP → envoi SMS
Résumé Digitization (OCR) → crédits IAP → extraction données CV → mise à jour fiche candidat
```

**Mécanismes :**
- Send SMS : fonctionnalité payante (crédits) ; envoi de SMS depuis la fiche candidat.
- Résumé Digitization : option « Do not digitize » / « Digitize on demand only » / « Digitize automatically » ; extraction nom, email, téléphone depuis le CV (IAP).
- Liens « Manage Service & Buy Credits », « View My Services » dans les paramètres.

**Recommandations pour Miyukini :**
- Découpler les fonctionnalités core (pipeline, candidatures, stages) des services IAP ; proposer des extensions optionnelles pour SMS et OCR si besoin.
- Gouvernance des données extraites (validation, confidentialité).

---

## 3. Synthèse des Flux

| App / Module   | Flux principal                                      | Données échangées                          |
|----------------|-----------------------------------------------------|--------------------------------------------|
| HR Employees   | Candidature → Create Employee                       | hr.employee créé depuis hr.applicant        |
| Mail           | Chatter, templates, envoi auto par stage          | Messages, activités, pièces                |
| Documents      | Stockage CV et pièces                              | Dossier Recruitment, fichiers              |
| Website        | Publication postes, formulaire candidature         | hr.job (publié), hr.applicant (créé)       |
| Surveys        | Enquêtes d’entretien, Send Interview               | survey.survey, lien candidature            |
| Referrals      | Referred By User, points par stage                  | referrer_id, points                        |
| UTM            | Sourcing, rapports                                  | utm_source_id, utm_medium_id, utm_campaign_id |
| Alias email    | Réception email → création candidature             | Parsing email + pièces → hr.applicant      |
| IAP            | SMS, OCR CV                                        | Crédits, données extraites                 |

---

## 4. Recommandations pour Miyukini

- **HR** : Intégration native avec MiyuHR pour création d’employé depuis la candidature (Mandat, WriteIntent).
- **Mail / Notify** : MiyuNotify pour envoi et templates ; traçabilité et conformité.
- **Documents / Media** : MiyuDocuments ou MiyuMedia pour CV et pièces ; politique de rétention et confidentialité.
- **Website** : MiyuWeb pour publication des postes et formulaire de candidature ; création de candidature sécurisée.
- **Surveys / Forms** : MiyuPolls ou équivalent pour enquêtes d’entretien ; lien candidature ↔ réponses.
- **Referrals** : Opérateur dédié pour référents et points ; gouvernance des attributions.
- **UTM** : Champs UTM conservés sur Candidature ; reporting et attribution des canaux.
- **Alias email** : Service d’entrée email gouverné pour création de candidatures.
- **IAP** : Optionnel ; découplage des services payants (SMS, OCR) du cœur recrutement.

---

**Document** : Odoo Recruitment — Intégrations Cross-App  
**Version** : 1.0  
**Date** : 2026-02-01
