# Odoo Recruitment — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application Recruitment d'Odoo, identifiant les personas, scénarios d'usage, étapes d'onboarding et points de friction pour guider l'implémentation d'un équivalent dans Miyukini.

**Source d'analyse :** Documentation Odoo 18.0/19.0, workflows et interface Recruitment.

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs
- Parcours d'onboarding
- Scénarios d'usage principaux (recruteur, candidat, manager)
- Points de friction identifiés
- Recommandations pour Miyukini

**Hors scope :**
- Détails techniques d'implémentation
- Spécifications UI/UX détaillées (document dédié)

---

## 1. Personas et Rôles

### 1.1 Recruteur (Recruiter / Officer)

**Profil :**
- Utilisateur principal de l'application
- Gère les postes, les candidatures et le pipeline
- Déplace les candidats d'un stage à l'autre, envoie des emails et des enquêtes
- Planifie les entretiens et envoie les offres

**Permissions :**
- Accès aux postes et candidatures
- Création / modification de candidatures
- Envoi d'emails et d'enquêtes
- Passage de stages, refus, création d'employé (selon droits)

### 1.2 Responsable recrutement / RH

**Profil :**
- Configure les postes, les stages et les paramètres
- Consulte les rapports (Source analysis, Velocity, Team performance)
- Valide les offres ou processus selon l'organisation

**Permissions :**
- Configuration (Settings, stages, modèles d'email)
- Création / édition de postes
- Accès aux rapports et tableaux de bord
- Gestion des droits (recruteurs, intervieweurs)

### 1.3 Intervieweur (Interviewer)

**Profil :**
- Conduit les entretiens pour certains postes
- Visible dans la liste des intervieweurs du poste ou de la candidature
- Peut avoir des droits limités (voir candidatures assignées, ajouter des notes)

**Permissions :**
- Accès aux candidatures des postes où il est intervieweur (selon configuration)
- Possibilité de consulter / annoter, selon droits

### 1.4 Candidat (Applicant — externe)

**Profil :**
- Postule via le site (formulaire), par email (alias du poste) ou est saisi manuellement
- Reçoit des emails (accusé de réception, convocations, offres)
- Peut remplir des enquêtes / formulaires d'entretien (lien Survey)
- Ne dispose pas d'accès back-office Odoo (sauf portail candidat si activé)

**Parcours :**
- Découverte de l'offre (site, job board, référence)
- Candidature (formulaire ou email)
- Réception d’emails automatiques ou manuels
- Entretiens (planification, enquêtes)
- Réception et signature d’offre

### 1.5 Référent (Employé — app Referrals)

**Profil :**
- Employé qui recommande un candidat (Referrals)
- Gagne des points quand la candidature progresse (stages configurés « Show in Referrals » + Points)
- Consulte l’app Referrals pour suivre ses recommandations

---

## 2. Parcours d'Onboarding

### 2.1 Première configuration (Responsable RH / Admin)

**Étapes :**

1. **Activation de l’app Recruitment**
   - Installation du module Recruitment (et éventuellement Employees, Documents, Website, Surveys selon besoins).

2. **Paramètres (Configuration > Settings)**
   - Process : Send interview survey (oui/non), Salary package (nombre de jours validité offre), Résumé Display (affichage CV sur fiche).
   - In-App Purchases : Send SMS, Résumé digitization (OCR) — optionnel, crédits IAP.

3. **Stages**
   - Vérifier les 6 stages par défaut (New, Initial Qualification, First Interview, Second Interview, Contract Proposal, Contract Signed).
   - Adapter noms, modèles d’email, « Hired Stage », « Folded », stages spécifiques à certains postes.

4. **Modèles d’email**
   - Vérifier / personnaliser : Applicant Acknowledgement, Interest, Schedule Interview, Refuse, Not interested anymore, etc.

5. **Création des premiers postes**
   - Nom, alias email, département, lieu, type d’emploi, description (Job Summary), processus (Application Info), recruteur, intervieweurs, formulaire d’entretien, modèle de contrat.

6. **Publication (si Website)**
   - Publication des postes sur le site pour recevoir des candidatures en ligne.

**Durée estimée :** 1 à 2 heures pour une configuration de base.

**Points de friction identifiés :**
- Dépendances multiples (Employees, Mail, Documents, Surveys, Website) selon le niveau de fonctionnalités.
- Paramètres IAP (SMS, OCR) et crédits à gérer.
- Personnalisation des stages et emails demande une bonne compréhension du flux.

### 2.2 Première utilisation (Recruteur)

**Étapes :**

1. **Accès à l’app Recruitment**
   - Tableau de bord : cartes des postes (Kanban).

2. **Ouverture des candidatures d’un poste**
   - Clic sur un poste > bouton « New Applications » (ou équivalent) > Kanban des candidatures par stage.

3. **Ajout manuel d’un candidat**
   - Quick Add (plus dans une colonne) ou bouton New > saisie nom, email, téléphone, poste, etc.

4. **Traitement d’une candidature**
   - Ouverture de la fiche > lecture CV/notes > déplacement en Kanban ou changement de stage > envoi d’email ou d’enquête si besoin.

5. **Envoi d’une offre**
   - Passage au stage Contract Proposal > envoi de l’offre (email / pièce) > après signature, passage Contract Signed > Create Employee.

**Points de friction identifiés :**
- Multiplicité des onglets et champs sur la fiche candidat (Candidate, Notes, Details, Skills).
- Compréhension des trois statuts par carte (In Progress, Blocked, Ready for Next Stage) et de la barre de statut par colonne.
- Dépendance à l’email pour envoi d’enquêtes et templates.

---

## 3. Scénarios d'Usage Principaux

### 3.1 Publier un poste et recevoir des candidatures

- **Acteur :** Responsable RH / Recruteur  
- **Actions :** Créer un poste, renseigner alias email, description, processus, publier sur le site (si activé).  
- **Résultat :** Candidatures créées automatiquement (formulaire site ou email vers alias).  
- **Miyukini :** Opérateur Publication postes + intégration Website / Mail pour création de candidatures.

### 3.2 Traiter une candidature de A à Z

- **Acteur :** Recruteur  
- **Actions :** New → Initial Qualification → First Interview (envoi convocation) → Second Interview → Contract Proposal (envoi offre) → Contract Signed → Create Employee.  
- **Résultat :** Candidat embauché, fiche employé créée.  
- **Miyukini :** Workflow gouverné (stages, transitions, actions) avec Mandats et WriteIntent pour création employé.

### 3.3 Refuser un candidat

- **Acteur :** Recruteur  
- **Actions :** Ouvrir la fiche > Refuse > choisir motif, option d’envoi email (template Refuse / Not interested).  
- **Résultat :** Candidature refusée, email envoyé si choisi.  
- **Miyukini :** Action « Refuse » avec motif et option notification, gouvernée par permissions.

### 3.4 Envoyer une enquête / formulaire d’entretien

- **Acteur :** Recruteur  
- **Actions :** Fiche candidat > Send Interview > choisir enquête, destinataires, date limite, envoyer.  
- **Résultat :** Email avec lien Survey ; candidat remplit l’enquête.  
- **Miyukini :** Intégration avec Opérateur Surveys / Forms, envoi gouverné et tracé.

### 3.5 Analyser les canaux de sourcing

- **Acteur :** Responsable RH  
- **Actions :** Consulter rapports Source analysis, Velocity, Team performance.  
- **Résultat :** Vue par source/medium/campagne, délais, performance des recruteurs.  
- **Miyukini :** Opérateur Reporting Recrutement, agrégations sur champs UTM et référents.

### 3.6 Gérer les références (Referrals)

- **Acteur :** Employé référent, RH  
- **Actions :** Candidat saisi avec « Referred By User » ; candidature avance jusqu’à un stage avec points > référent gagne des points.  
- **Résultat :** Incitation au référencement, reporting dans l’app Referrals.  
- **Miyukini :** Intégration avec Opérateur Referrals, attribution de points sous gouvernance.

---

## 4. Points de Friction Identifiés

- **Configuration** : Nombreux paramètres (Process, IAP, stages, emails) ; risque de confusion entre global et par poste.
- **Stages** : Suppression impossible si des candidatures sont encore dans le stage ; déplacement ou archivage préalable requis.
- **Affichage CV** : « Résumé Display » dépend du plein écran ; sur petite fenêtre le CV apparaît dans le chatter (Fichiers).
- **Enquêtes** : Dépendance à l’app Surveys et à la présence d’un email sur la fiche pour envoyer le lien.
- **IAP** : SMS et OCR CV nécessitent crédits et gestion de services externes.
- **Multi-postes** : Kanban et rapports sont par poste ; vue globale « toutes candidatures » possible mais navigation principalement par poste.
- **Droits** : Distinction Recruiter / Officer / Intervieweur à bien configurer pour éviter accès trop larges ou trop restreints.

---

## 5. Recommandations pour Miyukini

- **Personas** : Modéliser Recruteur, Manager RH, Intervieweur, Candidat (externe), Référent avec permissions distinctes (Master Butler).
- **Onboarding** : Assistant de configuration (stages par défaut, 1–2 modèles d’email, 1 poste exemple) pour réduire la friction.
- **Parcours candidat** : Candidature, emails, enquêtes, offre — tout tracé et sécurisé (WorrySentinel), sans exposer de données sensibles aux mauvais rôles.
- **Workflow** : Pipeline configurable (stages) avec transitions explicites et actions optionnelles (email, enquête) ; refus et embauche comme actions gouvernées (StrongFather / KindMother).
- **Reporting** : Prévoir rapports Source, Velocity, Performance par recruteur/poste dès le périmètre MVP pour valoriser l’usage.

---

**Document** : Odoo Recruitment — Parcours Utilisateur Détaillés  
**Version** : 1.0  
**Date** : 2026-02-01
