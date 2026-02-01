# Odoo Time Off — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **Time Off** (Congés / Absences) d'Odoo, identifiant les personas, scénarios d'usage, processus d'onboarding et points de friction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, code source hr_holidays

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs
- Parcours d'onboarding
- Scénarios d'usage principaux
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles Utilisateurs

### 1.1 Employé (Employee / Employee User)

**Profil :**
- Rôle opérationnel : Consulter ses soldes et poser des demandes de congé
- Responsabilités :
  - Consulter ses soldes par type de congé (My Time Off)
  - Créer une demande de congé (dates, type)
  - Suivre le statut de ses demandes (en attente, validée, refusée)
  - Consulter l’Overview (calendrier de ses congés)
  - Éventuellement demander une allocation supplémentaire (si type autorise)

**Besoins :**
- Vue "My Time Off" claire (soldes, demandes en cours, historique)
- Formulaire simple pour poser une demande (dates, type, optionnel : pièce jointe)
- Notifications sur validation / refus
- Compréhension du solde restant après validation

**Permissions :**
- Accès Time Off standard (employee)
- My Time Off, Overview (ses congés)
- Pas d’accès à Configuration, Management, Allocations (sauf ses propres demandes)

### 1.2 Manager / Approbateur employé (Employee's Approver)

**Profil :**
- Rôle de validation : Valider ou refuser les demandes de congé des employés dont il est l’approbateur
- Défini sur la fiche employé (Work Information — Time Off Approver)
- Responsabilités :
  - Consulter les demandes en attente de son équipe
  - Approuver ou refuser les demandes (selon type : By Employee's Approver)
  - Consulter l’Overview de son équipe

**Besoins :**
- Vue "Management" ou équivalent avec demandes à approuver
- Contexte (solde, type, dates) pour décider
  - Pas d’accès à la configuration des types ni aux allocations (sauf si aussi Time Off Officer)

**Permissions :**
- Droits employé + validation des demandes dont il est approbateur
- Pas obligatoirement Time Off Officer

### 1.3 Time Off Officer

**Profil :**
- Rôle administratif : Gestion des congés au niveau entreprise/département
- Responsabilités :
  - Configurer les types de congé (Configuration > Time Off Types)
  - Créer et gérer les allocations (Management > Allocations)
  - Valider/refuser les demandes (selon type : By Time Off Officer ou double validation)
  - Gérer les plans d’acquisition (Accrual Plans), jours fériés, jours obligatoires
  - Consulter les rapports (Time Off Summary, By Department, Analysis)
  - Overview équipe / entreprise

**Besoins :**
- Accès complet à Configuration, Management, Allocations, Reporting
- Wizards pour créer des allocations en lot (Generate Allocations)
- Vue claire des demandes en attente et des soldes
- Export / rapports pour paie et pilotage

**Permissions :**
- `hr_holidays.group_hr_holidays_user` (Time Off User) et/ou `group_hr_holidays_manager` (Time Off Officer)
- Accès à toutes les sections Time Off sauf paramètres système (réservés Admin)

### 1.4 Administrateur (Administrator)

**Profil :**
- Rôle système : Configuration avancée, droits, intégrations
- Responsabilités :
  - Droits d’accès (Time Off Officer, etc.)
  - Configuration multi-société (company sur types, plans, fériés)
  - Mode développeur : options Timesheets (projet/tâche par type de congé)
  - Intégration Payroll (Work Entry Type sur types de congé)

**Besoins :**
- Contrôle total sur la configuration et les droits
- Cohérence avec Employés (Work Information, approbateur), Payroll, Timesheet

---

## 2. Parcours d'Onboarding

### 2.1 Premier accès employé

1. **Accès à l’app Time Off** (menu ou dashboard)
2. **My Time Off** : découverte des soldes par type (si allocations déjà créées)
3. **Overview** : calendrier (vide ou avec congés déjà posés)
4. **Première demande** : clic "New" ou "Request Time Off" → formulaire (type, dates) → envoi
5. **Suivi** : statut "To Approve" puis "Approved" ou "Refused"
6. **Notification** : email / chatter sur validation ou refus

**Points d’attention :**
- Soldes vides si aucune allocation : message clair ou redirection vers manager/HR
- Types disponibles selon configuration (nom, unité jour/demi-jour/heure)

### 2.2 Premier accès Time Off Officer

1. **Configuration** : Types de congé (vérifier/modifier les 4 par défaut : Paid, Sick, Unpaid, Compensatory, Extra Hours)
2. **Jours fériés** : Configuration > Public Holidays — créer les jours fériés de l’entreprise
3. **Plans d’acquisition** (optionnel) : Configuration > Accrual Plans — règles d’acquisition
4. **Allocations** : Management > Allocations — créer les allocations (individuelles ou en lot via wizard)
5. **Droits** : S’assurer que les approbateurs sont renseignés sur les fiches employés (Work Information)
6. **Jours obligatoires** (optionnel) : Configuration > Mandatory Days — bloquer des dates

### 2.3 Intégration Employés et Payroll

- **Employés** : Onglet Work Information — Time Off Approver (obligatoire si type "By Employee's Approver")
- **Payroll** : Sur chaque type de congé, Work Entry Type pour générer les work entries
- **Timesheet** (si utilisé) : Projet/Tâche par type (mode développeur) pour les lignes de congé

---

## 3. Scénarios d'Usage Principaux

### 3.1 Poser une demande de congé (employé)

1. Time Off > My Time Off (ou Overview)
2. "New" / "Request Time Off"
3. Choisir le type de congé, les dates (et demi-journées/heures si proposé)
4. Vérifier le solde affiché (optionnel : pièce jointe si type autorise)
5. Envoyer la demande
6. Attendre validation (notification à l’approbateur)
7. Recevoir notification (approuvé / refusé)
8. Si approuvé : congé visible dans Overview et My Time Off ; solde mis à jour

**Variantes :**
- Demande en heures (si type en heures) : saisie heures
- Demande avec jour férié inclus : un jour de moins consommé si "Public Holiday Included"
- Demande au-delà du solde : possible si "Extra Days Requests Allowed" (soumis à validation)
- Solde négatif : possible si "Allow Negative Cap" avec plafond

### 3.2 Valider une demande (manager / Time Off Officer)

1. Time Off > Management (ou menu "To Approve" / demandes en attente)
2. Liste des demandes à approuver (filtre par équipe, type, période)
3. Ouvrir une demande : détail (employé, type, dates, solde avant/après)
4. Approuver ou refuser (avec motif optionnel)
5. L’employé est notifié ; si approuvé, le congé est visible et le solde mis à jour

**Double validation (By Employee's Approver and Time Off Officer) :**
- D’abord validation par l’Employee's Approver (état validate1)
- Puis validation par le Time Off Officer (état validate)

### 3.3 Créer des allocations (Time Off Officer)

1. Time Off > Management > Allocations
2. "New" : employé(s), type de congé, nombre de jours/heures, période
3. Ou utiliser le wizard "Generate Allocations" pour créer en lot (employés, type, montant, période)
4. Envoyer les allocations (confirm)
5. Valider (selon type : No Validation, By Time Off Officer, etc.)
6. Les soldes apparaissent dans My Time Off pour chaque employé

**Variante — Plans d’acquisition :**
- Les allocations sont générées automatiquement par le plan (cron/batch) selon les règles (fréquence, montant, plafonds, report)

### 3.4 Consulter l’Overview (équipe / entreprise)

1. Time Off > Overview
2. Choix de la période (Today, Week, Month, Year, custom)
3. Filtre : Mes congés / Mon équipe / Toute l’entreprise (selon droits)
4. Calendrier : barres colorées par employé (validé = plein, à approuver = rayé)
5. Total en bas : nombre de personnes absentes par jour
6. Clic sur un congé : détail ; "View" pour ouvrir la demande

### 3.5 Consulter les rapports

1. Time Off > Reporting (ou menu rapports)
2. **Time Off Summary** : synthèse par employé, type, période
3. **Time Off by Department** : congés par département
4. **Time Off Analysis** : analyse détaillée (pivot/graph)
5. Export PDF/Excel selon vues Odoo

### 3.6 Annuler un congé

1. Ouvrir la demande de congé (état Validated)
2. Action "Cancel" ou wizard "Cancel Leave"
3. Confirmer l’annulation
4. Le solde est recrédité ; le congé disparaît de l’Overview (et des work entries / timesheet si intégrés)

---

## 4. Points de Friction Identifiés

### 4.1 Soldes et allocations

- **Soldes vides à l’arrivée** : si aucune allocation n’a été créée, l’employé ne peut pas poser de congé ; message ou processus d’onboarding HR nécessaire
- **Multi-allocation** : plusieurs lignes d’allocation pour un même type (ex. report + nouvelle année) ; affichage du "solde total" doit rester clair
- **Accrual** : compréhension des règles d’acquisition (fréquence, plafonds, report) pour les employés et les managers

### 4.2 Validation

- **Double validation** : deux acteurs (Employee's Approver puis Time Off Officer) ; délai et clarté des notifications
- **Approver manquant** : si type "By Employee's Approver" et aucun approbateur sur la fiche employé, blocage ou comportement par défaut à expliciter
- **Refus sans motif** : possibilité d’ajouter un motif pour éviter les malentendus

### 4.3 Jours fériés et obligatoires

- **Fériés non configurés** : les employés peuvent poser un jour férié comme congé si "Public Holiday Included" n’est pas utilisé ou mal configuré
- **Jours obligatoires** : si mal communiqués, risque de demandes refusées sans explication claire (présence obligatoire)

### 4.4 UX et navigation

- **My Time Off vs Overview** : deux entrées pour "mes congés" ; hiérarchie ou fusion possible pour simplifier
- **Demandes en attente** : visibilité immédiate (badge, filtre "To Approve") pour les approbateurs
- **Mobile** : expérience sur petit écran pour poser une demande et consulter les soldes

### 4.5 Intégrations

- **Payroll** : cohérence Work Entry Type / types de congé et périodes de paie
- **Timesheet** : projet/tâche par type en mode développeur uniquement ; pas toujours exposé aux utilisateurs
- **Calendar** : création de réunion à la validation — configuration meeting type par type de congé à maintenir

---

## 5. Recommandations pour Miyukini

### 5.1 Parcours employé

- **Un seul point d’entrée "Mes congés"** : soldes + demandes + calendrier dans une vue cohérente (équivalent My Time Off + Overview unifié)
- **Formulaire de demande guidé** : type → dates → solde affiché en direct → envoi ; pièce jointe optionnelle selon type
- **Notifications gouvernées** : validation/refus via MiyuNotify, avec motif de refus possible
- **Transparence des règles** : affichage des règles d’acquisition (accrual) et des jours obligatoires pour limiter les refus "surprise"

### 5.2 Parcours approbateur / officier

- **Mandats de Permission** : validation des demandes sous Mandat (StrongFather + Master Butler) ; périmètre clair (mon équipe / tous selon rôle)
- **Vue "À approuver"** prioritaire : filtre par défaut, badge, rappels (Caring Nanny / MiyuNotify)
- **Allocations en lot** : outil dédié (wizard) avec choix employés, type, montant, période ; traçabilité des créations
- **Rapports** : Time Off Summary / By Department / Analysis exposés via Opérateur avec droits et gouvernance

### 5.3 Configuration

- **Types de congé** : modèle KindMother (WriteIntent) ; règles de validation (No Validation, By Approver, By Officer, Double) et options (jours fériés inclus, pièce jointe, negative cap) configurables
- **Jours fériés et jours obligatoires** : partagés avec Resource/Calendar/Planning ; une seule source de vérité
- **Plans d’acquisition** : règles explicites (fréquence, montant, plafonds, report) ; exécution par batch/cron avec traçabilité

### 5.4 Sécurité et gouvernance

- **Niveaux de sécurité** : données congés = sensibles (niveau 2) ; accès en lecture/écriture selon rôle (employé / approver / officer)
- **Audit** : traçabilité des validations/refus et des créations d’allocations (qui, quand, quoi)
- **Collaboration mandatée** : Time Off Operator avec HR (employés, approbateurs), Payroll (work entries), Calendar (événements) sous Contrats d’équipe et Mandats

---

## Références

- Documentation Odoo 19.0 — Time Off (Request time off, Allocations, Management, Overview, Reporting)
- Odoo — New employees (Work Information, Time Off Approver)
- Analyses Miyukini : Project (parcours), Employees (HR)
