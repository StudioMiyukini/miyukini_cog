# Odoo Employees — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Employees** (Employés) d'Odoo (version 19.0), à partir de la documentation officielle et du modèle HR. Il identifie les modèles de données, règles métier, workflows et mécanismes pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, module `hr` (Human Resources base)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (hr.employee, hr.department, contrats, certifications, badges, équipements)
- Règles métier et contraintes (hiérarchie, présence, droits)
- Workflows (onboarding, offboarding, retention)
- Gestion des compétences (skills, résumé, certifications)
- Présence (attendances, statut utilisateur, contrôle avancé)
- Organisation du travail (horaires, télétravail, lieux)

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `hr.employee` (Employé)

**Rôle :** Représente un employé — fiche centralisée regroupant informations générales, historique professionnel, informations de travail, données personnelles et paramètres RH.

**Champs clés (synthèse documentée) :**

#### Identification et général
- `name` : Nom de l'employé (requis)
- `job_id` / job_title : Poste (lié à hr.job si Recruitment installé)
- `company_id` : Société (requis)
- `department_id` : Département
- `parent_id` : Manager (hiérarchie)
- `coach_id` : Coach
- `user_id` : Utilisateur Odoo lié (optionnel — un employé n'a pas besoin d'être utilisateur)
- `work_contact_id` : Contact travail (res.partner)
- `image_1920` / avatar : Photo
- `barcode` / `badge_id` : Badge / code-barres (présence, POS)
- `pin` : Code PIN (présence, POS)
- `active` : Actif (soft delete)

#### Contact travail
- `work_email` : Email professionnel
- `work_phone` : Téléphone professionnel
- `work_mobile` : Mobile professionnel

#### Informations travail (Work Information)
- `address_id` : Adresse de travail
- `work_location_id` : Lieu de travail par défaut (Home, Office, Other)
- `resource_calendar_id` : Horaires de travail (référence Payroll / Working Schedules)
- `tz` : Fuseau horaire
- **Approbateurs** (selon apps installées) : Expense, Time Off, Timesheet, Attendance
- **Remote Work** (si activé) : Lieu par jour de la semaine (Lundi–Dimanche)
- **Planning** (si app Planning) : Roles, Default Role

#### Informations privées (Private Information)
- `address_home_id` : Adresse personnelle
- `private_email`, `private_phone` : Contact privé
- `bank_account_id` : Compte bancaire (remboursements, paie)
- `km_home_work` : Distance domicile–travail
- **Emergency** : Contact urgence (nom, téléphone)
- **Family status** : Situation familiale, conjoint, enfants à charge
- **Citizenship** : Nationalité, N° identification, SSN, passeport, genre, date/lieu de naissance, non-résident
- **Education** : Niveau certificat, domaine d'études, école
- **Work permit** : Visa, titre de séjour, dates d'expiration, fichier

#### Payroll (selon localisation)
- `legal_name` : Nom légal (fiscal)
- `payslip_lang` : Langue des bulletins
- `registration_number` : Numéro d'identification employé

#### Paramètres (Settings)
- `employee_type` : Type (Employee, Worker, Student, Trainee, Contractor, Freelancer)
- **Application** : Hourly Cost (Manufacturing), Fleet Mobility Card
- **Attendance / POS** : PIN, Badge ID

**Règles métier :**
- Un employé peut exister sans être utilisateur Odoo (pas de facturation licence).
- Manager et Coach peuvent être pré-remplis depuis le département.
- Les approbateurs (Expenses, Time Off, Timesheets, Attendance) doivent avoir les droits correspondants dans les apps concernées.
- Compte bancaire doit être marqué « Trusted » pour paiements / paie.
- Horaires de travail sont alignés sur les Working Schedules (Payroll) et par société.

### 1.2 Modèle `hr.department` (Département)

**Rôle :** Structure organisationnelle ; hiérarchie de départements avec manager et coach par défaut.

**Champs clés :**
- `name` : Nom du département
- `manager_id` : Manager du département (hr.employee)
- `parent_id` : Département parent
- `company_id` : Société (multi-company)
- Enfants : employés et sous-départements

**Règles métier :**
- Sélection d'un département peut auto-remplir Manager et Coach sur la fiche employé.
- Hiérarchie utilisée pour les droits (approbations, reporting).

### 1.3 Contrats (hr.contract)

**Rôle :** Contrats de travail liés à l'employé (gérés dans l'app Payroll / Contracts). Dates, type, salaire, etc.

**Lien :** Employé → contrats (one2many). Un employé peut avoir plusieurs contrats (historique).

### 1.4 Certifications (hr.certification)

**Rôle :** Certifications / expertises (SME). Liées aux employés ; affichées dans l'onglet Résumé si Skills Management activé.

### 1.5 Badges (hr.badge)

**Rôle :** Badges décernés aux employés (performance, réalisations). Attribués manuellement ou par règles.

### 1.6 Équipements (hr.equipment)

**Rôle :** Équipements attribués aux employés (ordinateurs, téléphones, etc.). Suivi et traçabilité.

### 1.7 Offboarding

**Rôle :** Processus de sortie (désactivation compte, récupération équipements, archivage). Documenté dans Employees (Offboarding).

### 1.8 Employee retention report

**Rôle :** Rapport d’analyse du taux de rétention (taux de départ, tendances).

---

## 2. Règles Métier et Contraintes

### 2.1 Hiérarchie et approbations

- **Manager** : parent_id ou department.manager_id ; utilisé pour approbations (expenses, time off, etc.).
- **Coach** : coach_id ; rôle de mentorat / évaluation.
- **Approbateurs** : un par domaine (Expense, Time Off, Timesheet, Attendance) ; doivent avoir les droits dans l’app correspondante.

### 2.2 Présence (Presence Display)

Trois modes (paramétrables dans Configuration) :

1. **Based on attendances** : Présent si pointé dans l’app Attendances.
2. **Based on user status in system** : Présent si l’employé est connecté à Odoo.
3. **Advanced Presence Control** (optionnel) :
   - **Based on number of emails sent** : présent si au moins N e-mails envoyés par heure.
   - **Based on IP Address** : présent uniquement depuis des IPs d’entreprise (liste configurable).

### 2.3 Skills Management

- Option dans Configuration pour afficher l’onglet **Résumé** (work experience, skills, certifications).
- **Skill Types** : catégories (ex. Languages, Soft Skills) avec liste de compétences et niveaux (nom + pourcentage).
- Niveaux ont un « default level » et une couleur.
- Seuls les utilisateurs avec droits « Officer: Manage all employees » ou Administrator peuvent ajouter/éditer les compétences.

### 2.4 Remote Work

- Option dans Configuration pour afficher **Remote Work** dans Work Information.
- Lieu par jour de la semaine (Lundi–Dimanche) : Home, Office, Other ou non défini (jours non travaillés).
- Icônes sur la fiche employé (emplacement, couleur selon présent/absent/hors horaire).

### 2.5 Work organization

- **Company Working Hours** : horaires par défaut de la société (ex. Standard 40 h/semaine, 32 h). Alignés sur Payroll (Working Schedules).
- Les horaires sont par société ; en multi-company chaque société a les siens.

### 2.6 Employee update rights

- Option **Employee Editing** : autoriser les employés à modifier leurs propres données sur leur fiche.

---

## 3. Workflows

### 3.1 Création d’un nouvel employé

1. Création fiche (nom, société requis).
2. Renseigner : poste, département, manager, coach, contact travail, lieu.
3. Optionnel : Résumé (expériences, compétences), Work Information (horaires, approbateurs, télétravail), Private Information (banque, urgence, citoyenneté, permis de travail), Payroll, Settings (type, user_id, PIN, badge).
4. Lier un utilisateur si besoin (Create User depuis la fiche).

### 3.2 Onboarding

Processus documenté (Employees → Onboarding) : étapes d’intégration, checklist, affectation équipements, formations.

### 3.3 Offboarding

Processus de sortie : désactivation, archivage, récupération équipements, clôture accès.

### 3.4 Retention

Rapport de rétention : indicateurs et tendances sur les départs.

---

## 4. Intégrations Métier (résumé)

- **Payroll** : contrats, horaires, paie, compte bancaire, champs légaux.
- **Recruitment** : postes (hr.job) pour Job Position.
- **Expenses** : approbateur dépenses par employé.
- **Time Off** : approbateur congés.
- **Timesheets** : approbateur feuilles de temps.
- **Attendances** : présence, PIN, badge, kiosque.
- **Appraisals** : date prochaine évaluation, historique.
- **Planning** : rôles et rôle par défaut.
- **Fleet** : Fleet Mobility Card.
- **Manufacturing** : coût horaire (work center).

---

## 5. Points d’Attention pour Miyukini

1. **Séparation Employé / Utilisateur** : conserver la notion « fiche employé » sans obligation de compte utilisateur.
2. **Hiérarchie et mandats** : manager / coach / approbateurs comme base pour StrongFather (décisions) et Master Butler (permissions).
3. **Présence** : traiter les trois modes (pointage, connexion, signaux avancés) avec Caring Nanny / états de confiance si pertinent.
4. **Données sensibles** : Private Information et Payroll → niveau de sécurité élevé (WorrySentinel), isolation stricte.
5. **Skills et certifications** : modélisation en référentiels gouvernés (Ever Buddy pour évolution).
6. **Multi-société** : company_id cohérent avec COG / environnements.

---

**Document** : Odoo Employees — Logique Métier Complète  
**Version** : 1.0  
**Date** : 2026-02-01
