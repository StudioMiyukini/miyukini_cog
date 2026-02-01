# Odoo Employees — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Employees** (Employés) d'Odoo (version 19.0), à partir de la documentation officielle. Il identifie les vues, onglets, composants et patterns de navigation pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0 (Employees, New employees, Departments, Settings)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Structure de navigation et menus
- Formulaire employé (onglets et sections)
- Vues liste / Kanban employés et départements
- Configuration (Settings)
- Patterns d’interaction et feedback

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Structure de Navigation

### 1.1 Menu principal Employees

- **Employees** (app racine)
  - **Dashboard / liste** : Vue par défaut des employés
  - **New** : Création nouvel employé (formulaire vide)
  - **Departments** : Création et gestion des départements
  - **Certifications** : Gestion des certifications
  - **Badges** : Gestion des badges
  - **Equipment** : Gestion des équipements
  - **Offboarding** : Processus de sortie
  - **Employee retention report** : Rapport de rétention
- **Configuration**
  - **Settings** : Presence Display, Skills Management, Remote Work, Employee Editing, Company Working Hours
  - **Skill Types** : Types de compétences, compétences, niveaux, couleurs
  - **Work Locations** : Lieux de travail (adresse, couverture, société)

### 1.2 Entrées secondaires

- Depuis une fiche employé : liens internes vers Département, Manager, Coach, Company, Working Hours, etc. (ouverture formulaire associé).
- Create User : bouton en fin de ligne « Related User » pour créer un utilisateur à partir de l’employé.

---

## 2. Formulaire Employé (hr.employee)

### 2.1 En-tête / Général

- **Photo** : Zone image en haut à droite ; clic sur icône Edit pour upload.
- **Nom de l’employé** (requis).
- **Job Position** (texte libre sous le nom).
- **Work Contact** : Work Email, Work Phone, Work Mobile.
- **Tags** : Sélection ou création de tags (réutilisables).
- **Company** (requis), **Department**, **Job Position** (liste), **Manager**, **Coach**.
- **Next Appraisal Date** : Visible si app Appraisals installée ; sélecteur de date.
- **Sauvegarde** : Icône « Save manually » ; sauvegarde automatique possible pendant la saisie.

### 2.2 Onglets

#### Onglet Résumé (Resumé)

- **Résumé** : Lignes d’expérience (titre, type Experience/Education/Social Media/Internal Certification, Display Type Classic/Certification, durée, description). Boutons « Create a new entry », « ADD », « Save & Close », « Save & New ».
- **Skills** : Bouton « Pick a skill from the list » ; pour chaque compétence : Skill Type (radio), Skill (liste), Skill Level (liste + barre de progression). Les Skill Types doivent être configurés en amont (Configuration → Skill Types).
- **Skill types** : Configuration des types (nom, SKILLS, LEVELS avec nom et pourcentage, Default Level, DISPLAY couleur).

#### Onglet Work Information

- **LOCATION** : Work Address, Work Location (Home/Office/Other ou création).
- **APPROVERS** (si droits Officer/Admin et apps installées) : Expense, Time Off, Timesheet, Attendance — un approbateur par champ.
- **REMOTE WORK** (si paramètre activé) : Lieu par jour (Lundi–Dimanche) ; options Home, Office, Other, Unspecified.
- **SCHEDULE** : Working Hours (liste déroulante, lien vers détail), Timezone.
- **PLANNING** (si app Planning) : Roles, Default Role.

#### Onglet Private Information

- **PRIVATE CONTACT** : Private Address, Email, Phone ; Bank Account (création/édition, Trusted) ; Home–Work Distance ; Private Car Plate.
- **EMERGENCY** : Contact Name, Contact Phone.
- **FAMILY STATUS** : Marital Status, Spouse Name/Birthdate si marié/cohabitant, Number of Dependent Children.
- **CITIZENSHIP** : Nationality, Identification No, SSN No, Passport No, Gender, Date of Birth, Place/Country of Birth, Non-resident.
- **EDUCATION** : Certificate Level, Field of Study, School.
- **WORK PERMIT** : Visa No, Work Permit No, dates d’expiration, upload fichier.

#### Onglet Payroll

- Selon localisation : Legal Name, Payslip Language, Registration Number of the Employee, etc.

#### Onglet Settings

- **STATUS** : Employee Type (Employee, Worker, Student, Trainee, Contractor, Freelancer), Related User (lien + Create User).
- **APPLICATION SETTINGS** : Hourly Cost, Fleet Mobility Card.
- **ATTENDANCE/POINT OF SALE** (si apps installées) : PIN Code, Badge ID (Generate / Print Badge).

### 2.3 Composants récurrents

- **Listes déroulantes** : Sélection ou « Create / Create and edit… » pour nouveaux enregistrements.
- **Liens internes** : Flèche à côté d’un champ pour ouvrir le formulaire lié (Company, Department, Working Hours, etc.).
- **Sauvegarde** : Save & Close, Save & New sur les sous-formulaires (résumé, compétences, etc.).
- **Icônes** : Edit (photo), Save manually, Internal link.

---

## 3. Vues Liste et Kanban

### 3.1 Liste employés

- Colonnes typiques : Nom, Département, Job Position, Manager, Company, Contact (email/téléphone), Statut présence si pertinent.
- Filtres : Par département, société, actifs/inactifs, etc.
- Groupements : Département, Manager, Société.
- Recherche : Sur nom, email, poste.

### 3.2 Kanban employés

- Cartes employés avec photo, nom, poste, département.
- Si **Remote Work** activé : icône lieu (home/building/autre) et couleur (présent/absent/hors horaire) en haut à droite de la carte.
- Clic sur carte → ouverture formulaire.

### 3.3 Départements

- Liste ou structure hiérarchique (arbre).
- Formulaire département : Nom, Manager, Parent, Company.

---

## 4. Configuration (Settings)

### 4.1 Employees

- **Presence Display** : Choix — Based on attendances / Based on user status in system / Advanced Presence Control (emails sent, IP addresses).
- **Skills Management** : Activer/désactiver pour afficher l’onglet Résumé (work experience, skills, certifications).
- **Remote Work** : Activer pour afficher lieu par jour (Work Information) et icônes sur les cartes.
- **Employee Editing** : Autoriser les employés à modifier leur propre fiche.

### 4.2 Work organization

- **Company Working Hours** : Liste déroulante (ex. Standard 40 h/semaine, 32 h, Appointment Resource Default Calendar). Aligné avec Payroll (Working Schedules).

### 4.3 Skill Types

- Liste des types ; New → Formulaire : Skill Type (nom), SKILLS (lignes), LEVELS (nom, progress %, default), DISPLAY (couleur).
- Niveaux réordonnés par progression après sauvegarde (ordre décroissant).

### 4.4 Work Locations

- Liste des lieux ; New → Work Location, Work Address, Cover Image (home/building/map marker), Company (multi-company).

---

## 5. Patterns d’Interaction et Feedback

- **Auto-save** : Formulaire employé peut sauvegarder automatiquement ; sauvegarde manuelle toujours disponible.
- **Création liée** : Champs avec « Create (…) » / « Create and edit… » pour créer un enregistrement lié sans quitter la page.
- **Validation** : Champs requis (nom, société) ; messages d’erreur si soumission incomplète.
- **Breadcrumb** : Retour au formulaire employé après édition d’un enregistrement lié.
- **Notes et avertissements** : Messages dans la doc (ex. banque Trusted, droits approbateurs, horaires par société).

---

## 6. Recommandations pour Miyukini

1. **Formulaire par onglets** : Conserver une structure par blocs (Général, Work Information, Private, Payroll, Settings) avec niveaux de sécurité adaptés (masquage/lecture seule selon rôle).
2. **Référentiels** : Skill Types et Work Locations comme écrans de configuration dédiés ; pas de création à la volée depuis la fiche si on souhaite garder la cohérence des référentiels.
3. **Présence / télétravail** : Indicateurs visuels (icônes, couleurs) sur les cartes sans surcharger ; options configurables centralisées.
4. **Approbateurs** : Liste déroulante filtrée par droits (équivalent Master Butler) avec indication claire des droits requis par app.
5. **Responsive** : Formulaire long ; prévoir navigation par onglets et ancres pour mobile/tablette.
6. **Accessibilité** : Labels explicites, contraste, navigation clavier pour formulaire et configuration.

---

**Document** : Odoo Employees — Analyse UI/UX  
**Version** : 1.0  
**Date** : 2026-02-01
