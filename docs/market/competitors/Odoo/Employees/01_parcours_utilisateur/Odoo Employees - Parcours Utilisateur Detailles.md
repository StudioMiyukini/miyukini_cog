# Odoo Employees — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **Employees** (Employés) d'Odoo, identifiant les personas, scénarios d'usage, processus d'onboarding/offboarding et points de friction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0 (Employees, New employees, Departments, Certifications, Badges, Equipment, Offboarding, Retention report)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles (HR Officer, Manager, Employé, Administrateur)
- Parcours d'onboarding et création de fiche employé
- Scénarios : départements, certifications, badges, équipements, offboarding
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles Utilisateurs

### 1.1 Administrateur (HR Administrator)

**Profil :**
- Rôle : Configuration globale et droits complets sur les employés
- Responsabilités : Paramètres Employees (présence, skills, télétravail, horaires société), gestion des utilisateurs et droits, vue sur toutes les fiches

**Besoins :**
- Configuration Presence Display, Skills Management, Remote Work, Employee Editing
- Company Working Hours (alignement Payroll)
- Accès à toutes les fiches et rapports (retention, offboarding)

**Permissions :** Droits complets Employees ; peut gérer Skill Types, Work Locations, Approvers.

### 1.2 HR Officer (Officer: Manage all employees)

**Profil :**
- Rôle : Gestion courante des fiches employés et des structures
- Responsabilités : Création/modification fiches, départements, ajout/édition des compétences, certifications, badges, équipements

**Besoins :**
- Liste / Kanban / Formulaire employés et départements
- Création nouvel employé (tous onglets)
- Gestion des compétences (Skill Types, niveaux, couleurs)
- Attribution certifications, badges, équipements
- Processus offboarding

**Permissions :** Gestion de tous les employés ; pas nécessairement la configuration globale (Settings).

### 1.3 Manager (department / hiérarchie)

**Profil :**
- Rôle : Manager d’équipe ou de département
- Responsabilités : Validation des demandes (congés, notes de frais, timesheets selon apps), vue sur son équipe, éventuellement coach

**Besoins :**
- Vue sur ses subordonnés (parent_id / department)
- Fiches employés de son périmètre
- Pas de création de Skill Types ; peut voir les compétences

**Permissions :** Dépend des apps (Expenses, Time Off, etc.) ; dans Employees, accès limité aux fiches selon règles d’accès.

### 1.4 Employé (Employee)

**Profil :**
- Rôle : Consulter et éventuellement éditer sa propre fiche
- Responsabilités : Mise à jour de ses infos si « Employee Editing » activé (contact, adresse, etc.)

**Besoins :**
- Accès à sa propre fiche (lecture, voire édition partielle)
- Vue horaires, lieu de travail, approbateurs
- Pas d’accès aux autres employés ni à la configuration

**Permissions :** Lecture fiche personnelle ; édition si option Employee Editing activée.

---

## 2. Parcours d’Onboarding

### 2.1 Création d’un nouvel employé

1. **Accès** : Employees → New
2. **Obligatoire** : Nom, Société
3. **Général** : Photo, Job Position (libre et/ou liste), Work Email/Phone/Mobile, Tags, Département, Job Position (liste), Manager, Coach, Company
4. **Optionnel** : Next Appraisal Date (si Appraisals)
5. **Résumé** : Expériences, Éducation, Compétences (si Skills Management) — types de compétences préalablement configurés
6. **Work Information** : Adresse travail, Lieu, Approvers (Expense, Time Off, Timesheet, Attendance si apps installées), Remote Work (par jour), Horaires, Fuseau, Planning (rôles si app Planning)
7. **Private Information** : Adresse privée, banque (Trusted), urgence, situation familiale, citoyenneté, éducation, permis de travail
8. **Payroll** : Selon localisation (Legal Name, langue bulletin, numéro d’enregistrement)
9. **Settings** : Type employé, Related User (création utilisateur si besoin), Hourly Cost, Fleet Mobility Card, PIN, Badge ID
10. **Sauvegarde** : Auto-save possible ; sauvegarde manuelle disponible

**Points d’attention :**
- Département sélectionné peut pré-remplir Manager et Coach
- Les approbateurs listés doivent avoir les droits dans les apps correspondantes (Expenses, Time Off, etc.)
- Compte bancaire doit être marqué Trusted pour paie / paiements

### 2.2 Onboarding (processus)

Documenté dans Employees → Onboarding : étapes d’intégration, tâches, affectation équipements, formations. Parcours guidé après création de la fiche.

---

## 3. Scénarios d’Usage Principaux

### 3.1 Gestion des départements

- **Création** : Employees → Configuration → Départements (ou équivalent) → New
- **Champs** : Nom, Manager, Département parent, Société
- **Effet** : Lors de la sélection du département sur une fiche employé, Manager et Coach peuvent être renseignés automatiquement

### 3.2 Certifications

- **Configuration** : Certifications comme « sujets d’expertise »
- **Attribution** : Depuis la fiche employé ou liste des certifications
- **Affichage** : Onglet Résumé (si Skills Management activé)

### 3.3 Badges

- **Attribution** : Badges pour performance / réalisations
- **Gestion** : Liste des badges, attribution aux employés

### 3.4 Équipements

- **Attribution** : Lier équipements (ordinateur, téléphone, etc.) à un employé
- **Suivi** : Liste des équipements par employé ; récupération lors de l’offboarding

### 3.5 Offboarding

- **Déclenchement** : Processus de sortie (fin de collaboration)
- **Étapes** : Désactivation compte, récupération équipements, archivage, clôture accès
- **Documentation** : Employees → Offboarding

### 3.6 Rapport de rétention

- **Accès** : Employees → Employee retention report
- **Usage** : Analyse du taux de rétention, tendances des départs

### 3.7 Présence et télétravail

- **Configuration** : Settings → Presence Display (attendances / user status / avancé)
- **Télétravail** : Si Remote Work activé, renseignement du lieu par jour sur la fiche ; icônes présence/lieu sur les cartes employés

---

## 4. Points de Friction Identifiés

1. **Droits des approbateurs** : Les listes d’approbateurs (Expense, Time Off, etc.) ne montrent que les utilisateurs ayant les bons droits dans chaque app ; configuration multi-app parfois lourde.
2. **Skills** : Les compétences ne peuvent être ajoutées que depuis les Skill Types (dashboard) ; pas de création à la volée depuis la fiche employé.
3. **Horaires** : Dépendance à Payroll (Working Schedules) ; en multi-société chaque société doit avoir ses horaires.
4. **Banque** : Compte bancaire « Trusted » obligatoire pour paie ; risque d’erreur si oubli.
5. **Employé vs Utilisateur** : Création d’un utilisateur depuis la fiche (Create User) bien documentée mais notion « employé sans utilisateur » à expliquer aux nouveaux.
6. **Localisation** : Champs Payroll et légaux très variables selon pays ; nécessité de vérifier avec comptabilité / paie.

---

## 5. Recommandations pour Miyukini

1. **Personas alignés** : Distinguer clairement Administrateur RH, Officer, Manager, Employé avec Mandats de Permission (StrongFather, Master Butler) et Contrats d’équipe.
2. **Onboarding guidé** : Proposer un parcours étape par étape (checklist, états) avec traçabilité (Ever Buddy, TAMR).
3. **Approbateurs** : Modéliser les approbateurs par domaine (expenses, time_off, timesheet, attendance) avec vérification des capacités (Master Butler) et exposition claire des droits requis.
4. **Skills** : Référentiels gouvernés (Skill Types, niveaux) avec possibilité d’évolutions contrôlées (Ever Buddy) et droits d’édition réservés (Officer/Admin).
5. **Présence** : Exposer les trois modes (pointage, connexion, avancé) comme options configurables sans logique métier dispersée ; états de confiance (Caring Nanny) si pertinent.
6. **Données sensibles** : Isoler Private Information et Payroll (WorrySentinel, niveau Critical) ; accès en lecture/écriture strictement mandaté.
7. **Offboarding** : Workflow explicite (états, étapes) avec récupération équipements et révocation des mandats (StrongFather, Ever Buddy).

---

**Document** : Odoo Employees — Parcours Utilisateur Détaillés  
**Version** : 1.0  
**Date** : 2026-02-01
