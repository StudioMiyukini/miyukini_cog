# Odoo Recruitment — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Recruitment** (Recrutement) d'Odoo (versions 18.0 / 19.0), à partir de la documentation officielle et des modèles standards. Il identifie les modèles de données, règles métier, workflows, et mécanismes pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18.0/19.0 — Applications HR / Recruitment

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (hr.job, hr.applicant)
- Règles métier et contraintes
- Workflow de recrutement et stages
- Gestion des candidatures (création, progression, refus, embauche)
- Intégration CV / Documents, enquêtes, templates email
- Règles de sourcing (source, medium, UTM) et références

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `hr.job` (Poste / Job Position)

**Rôle :** Représente un **poste à pourvoir** — offre d'emploi avec configuration, publication et cibles de recrutement.

**Champs clés (documentation) :**

#### Identification
- `name` : Intitulé du poste (ex. Sales Manager, Mechanical Engineer)
- `sequence` : Ordre d'affichage
- `active` : Poste actif / archivé

#### Recrutement
- `application_count` / `application_ids` : Nombre / liste des candidatures
- `no_of_hired_employee` : Nombre d'embauches réalisées
- `no_of_recruitment` : Cible d'embauche (nombre de personnes à recruter)
- `recruiter_id` : Recruteur responsable (Many2one res.users)
- `interviewer_ids` : Intervieweurs (Many2many res.users)
- `alias_id` / `alias_email` : Alias email pour recevoir les CV (création auto candidature)

#### Département et localisation
- `department_id` : Département (Many2one hr.department)
- `address_id` : Lieu de travail (Many2one res.partner) — visible sur le site
- `company_id` : Société (multi-company)

#### Type d'emploi et rémunération
- `employment_type` : Type (Permanent, Temporary, Seasonal, Full-Time, Intern, Student, Apprenticeship, Thesis, Statutory, Employee)
- `expected_skills` : Compétences attendues (liens hr.skill)
- `salary_min` / `salary_max` : Fourchette salariale (optionnel)
- `salary_currency_id` : Devise
- `resource_calendar_id` : Horaires de travail (ex. 40h/semaine)

#### Publication et processus
- `website_id` : Site web de publication
- `job_summary` : Description du poste (visible site)
- `process_details` : Détails du processus (Time to Answer, Process steps, Days to get an Offer)
- `interview_form_id` : Formulaire d'entretien / enquête (survey.survey)
- `contract_template_id` : Modèle de contrat pour l'offre
- `industry_id` : Secteur d'activité (pour job boards)
- `mission_start_date` / `mission_end_date` : Dates de mission (postes temporaires)

**Règles métier :**
- Un poste peut avoir zéro ou plusieurs candidatures (`application_ids`).
- L'alias email crée automatiquement une candidature (`hr.applicant`) à réception d'un email/CV.
- Les stages (hr.recruitment.stage) s'appliquent à tous les postes sauf si marqués « spécifiques à un poste ».
- Suppression : un poste peut être archivé ; suppression logique selon règles Odoo.

---

### 1.2 Modèle `hr.applicant` (Candidature)

**Rôle :** Représente une **candidature** — un candidat pour un poste donné, avec suivi par stages.

**Champs clés :**

#### Identification candidat
- `partner_name` ou `name` : Nom du candidat (obligatoire en création manuelle)
- `email_from` : Email (requis pour envoi d'emails / enquêtes)
- `partner_phone` : Téléphone
- `partner_id` : Lien vers res.partner si contact existant (ex. employé, ancien candidat)
- `linkedin_profile` : Profil LinkedIn

#### Poste et recrutement
- `job_id` : Poste concerné (Many2one hr.job)
- `user_id` : Recruteur responsable (défaut depuis hr.job)
- `department_id` : Département (hérité du poste)
- `company_id` : Société (multi-company)
- `interviewer_ids` : Intervieweurs (hérités ou surchargés)

#### Workflow
- `stage_id` : Stage actuel (Many2one hr.recruitment.stage)
- Statuts par carte : In Progress (gris), Blocked (rouge), Ready for Next Stage (vert) — configurables par stage

#### Détails candidat
- `degree_id` : Niveau d'études (Graduate, Bachelor, Master, Doctoral)
- `availability` : Date de disponibilité
- `description` : Présentation courte (peut alimenter le chatter en « Other Information »)
- `notes` : Notes internes

#### Package salarial
- `expected_salary` : Prétentions salariales
- `proposed_salary` : Salaire proposé
- `salary_expected_extra` / `salary_proposed_extra` : Avantages (texte)
- `offer_date` : Date d'offre
- `refuse_reason_id` : Motif de refus (si refusé)
- `day_to_offer` : Jours avant expiration de l'offre (config global ou par poste)

#### Sourcing (UTM / Référencement)
- `utm_source_id` : Source (Search engine, LinkedIn, Newsletter, etc.)
- `utm_medium_id` : Support (Email, Website, etc.)
- `utm_campaign_id` : Campagne
- `referrer_id` : Employé ayant référé (référent) — pour app Referrals

#### Fichiers et communication
- Pièces jointes / CV : stockés (Documents app), option « affichage CV sur la fiche » (Résumé Display)
- Chatter : messages, activités, historique

**Règles métier :**
- Champs requis minimaux (création) : nom du candidat ; email et téléphone souvent requis par poste / formulaire site.
- Passage d'un stage à l'autre : glisser-déposer Kanban ou changement de `stage_id` sur la fiche ; emails automatiques possibles par stage (template configuré sur le stage).
- Stage « Hired » : cocher « Hired Stage » sur un stage ; quand la candidature entre dans ce stage, bannière « Hired » et détermination de la date d'embauche.
- Refus : motif de refus possible ; template email « Refuse » / « Not interested anymore ».
- Embauche : depuis le stage Contract Signed, action « Create Employee » — création d’un enregistrement `hr.employee` à partir du candidat.
- Un candidat (partner_id ou email) peut avoir plusieurs candidatures (postes différents ou réouvertures).

---

### 1.3 Modèle `hr.recruitment.stage` (Stage de recrutement)

**Rôle :** Définit les **étapes du pipeline** (colonnes Kanban) pour les candidatures.

**Champs typiques :**
- `name` : Libellé du stage (New, Initial Qualification, First Interview, Second Interview, Contract Proposal, Contract Signed)
- `sequence` : Ordre des colonnes
- `fold` : Colonne repliée par défaut en Kanban (ex. Contract Signed)
- `template_id` : Modèle d’email envoyé automatiquement à l’entrée dans le stage
- `hired_stage` : Boolean — ce stage = « embauché » (bannière Hired, date d’embauche)
- `job_ids` : Postes concernés (optionnel) — si vide, stage global ; si renseigné, stage spécifique à certains postes
- `requirements` : Notes internes / exigences du stage
- **Referrals** : « Show in Referrals », « Points » — visibilité dans l’app Referrals et attribution de points au référent
- **Tooltips / statuts** : libellés des 3 statuts (In Progress, Blocked, Ready for Next Stage) — couleurs fixes (gris, rouge, vert)

**Règles métier :**
- Les stages s’appliquent à tous les postes sauf si `job_ids` est renseigné (stage spécifique).
- Suppression d’un stage : impossible s’il reste des candidatures dans ce stage ; il faut les déplacer ou archiver.
- Un seul stage peut être marqué « Hired » pour le calcul de la date d’embauche.

---

## 2. Workflow de Recrutement

### 2.1 Pipeline par défaut (6 stages)

1. **New** — Toute nouvelle candidature (site, email, saisie manuelle).
2. **Initial Qualification** — Tri des candidats potentiels ; pas d’action auto par défaut.
3. **First Interview** — Premier entretien ; option : envoi auto email « Schedule Interview » (lien calendrier).
4. **Second Interview** — Deuxième entretien.
5. **Contract Proposal** — Offre envoyée ; délai de validité configurable (jours).
6. **Contract Signed** — Contrat signé, candidat embauché ; colonne souvent repliée ; stage « Hired » si coché.

### 2.2 Transitions

- **Avancement** : glisser-déposer en Kanban ou changement de stage sur la fiche candidat.
- **Refus** : action dédiée (refuse) avec motif et option d’envoi d’email (template Refuse / Not interested).
- **Embauche** : depuis Contract Signed, action « Create Employee » — création `hr.employee`, liaison possible au partenaire / utilisateur.

### 2.3 Automatisations par stage

- **Email à l’entrée** : chaque stage peut avoir un `template_id` ; à l’entrée dans le stage, envoi automatique de l’email au candidat.
- **Enquête / Interview** : envoi manuel d’un « Interview » (survey) depuis la fiche candidat ; le formulaire d’entretien peut être attaché au poste (`interview_form_id`).

---

## 3. Règles Métier Transverses

### 3.1 Candidatures

- **Création** : manuelle (bouton New ou Quick Add par stage), ou automatique (alias email, formulaire site).
- **Champs requis** : au minimum nom ; email et téléphone selon configuration du poste / formulaire.
- **CV** : stockage dans Documents (dossier Recruitment) ; option « Résumé Display » pour afficher le CV sur la fiche (plein écran).
- **OCR CV (IAP)** : option « Résumé Digitization » — extraction nom, email, téléphone depuis le CV (à la demande ou automatique).

### 3.2 Offres et délais

- **Salary package configurator** : nombre de jours de validité de l’offre (paramètre global).
- Après expiration : l’offre n’est plus disponible (logique métier côté interface / état).

### 3.3 Enquêtes / Interviews

- **Send Interview Survey** : paramètre global ; installe l’app Surveys si nécessaire.
- Formulaire d’entretien : rattaché au poste (`interview_form_id`) ; envoi manuel « Send Interview » depuis la fiche candidat (email avec lien survey).
- Les enquêtes sont gérées par l’app Surveys (survey.survey, survey.user_input).

### 3.4 Références (Referrals)

- Si l’app Referrals est installée : champs « Referred By User », « Show in Referrals » et « Points » sur les stages.
- Quand une candidature atteint un stage avec points, le référent gagne des points.

### 3.5 Sourcing et reporting

- **Source / Medium / Campaign** : champs UTM sur la candidature pour analyse des canaux (rapports Source analysis, Velocity, etc.).
- **Referrer** : lien vers l’employé référent pour rapports et bonus Referrals.

---

## 4. Points d’Attention pour Miyukini

- **Modèles cibles** : équivalents `Job` (poste), `Applicant` (candidature), `RecruitmentStage` (stage).
- **Workflow** : pipeline configurable par étapes avec transitions et éventuelles actions automatiques (email).
- **Gouvernance** : création/modification de candidatures, passage de stage, refus, embauche — à encadrer par permissions (Master Butler) et décisions (StrongFather) si besoin.
- **Persistance** : KindMother pour Job, Applicant, Stage ; WriteIntent pour toute création/modification.
- **Intégrations** : Documents (CV), Mail (templates, chatter), Website (formulaires, publication), Surveys (entretiens), HR Employees (création employé), Referrals (référents et points), UTM (sourcing).
- **Données sensibles** : CV, coordonnées, salaires — niveau de sécurité élevé (WorrySentinel) et traçabilité.

---

**Document** : Odoo Recruitment — Logique Métier Complète  
**Version** : 1.0  
**Date** : 2026-02-01
