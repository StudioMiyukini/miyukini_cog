# Odoo Recruitment — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Recruitment** d'Odoo (18.0 / 19.0), à partir de la documentation officielle. Il identifie les vues, patterns de navigation, formulaires et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18.0 — Applications HR / Recruitment

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Tableau de bord et vues Kanban (postes, candidatures)
- Formulaires poste et candidat
- Patterns de navigation et actions contextuelles
- Stages, statuts et couleurs
- Intégration chatter, pièces jointes, affichage CV
- Paramètres et configuration des stages

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Vues Principales

### 1.1 Tableau de bord Recruitment (Vue par défaut)

**Rôle :** Point d’entrée — affichage des **postes** en cartes Kanban.

**Caractéristiques :**
- Une carte par poste (`hr.job`).
- Bannière **PUBLISHED** en haut à droite si le poste est actif et publié (candidatures acceptées).
- Clic sur une carte : accès aux détails du poste ou à la liste/Kanban des candidatures pour ce poste.
- Bouton **New** (en haut à gauche) : création d’un nouveau poste (popup ou formulaire).
- Bouton **(#) New Applications** (smart button sur la carte) : accès direct au Kanban des candidatures du poste.

**Contenu type d’une carte poste :**
- Titre du poste
- Indicateur nombre de candidatures / nouvelles candidatures
- Statut publié / non publié
- Menu (dropdown) : Configuration, autres actions

### 1.2 Kanban Candidatures (Applications)

**Accès :** Depuis une carte poste > « New Applications » (ou équivalent).

**Structure :**
- **Colonnes = stages** (New, Initial Qualification, First Interview, Second Interview, Contract Proposal, Contract Signed).
- **Contract Signed** : colonne repliée par défaut (grise, cartes masquées) ; clic sur la colonne pour déplier.
- **Barre de couleur** sous le nom du stage : indicateur d’état des candidatures de la colonne (vert = prêt pour la suite, rouge = bloqué, gris = en cours).
- **Cartes** : une carte par candidature (`hr.applicant`).

**Carte candidat (Kanban) :**
- Titre : nom du candidat
- **Pastille de statut** (cercle en bas à gauche) : In Progress (gris), Blocked (rouge), Ready for Next Stage (vert). Clic pour ouvrir un popup et changer le statut.
- Informations secondaires possibles : poste, recruteur, email, téléphone (selon mise en page).
- Glisser-déposer entre colonnes pour changer de stage.

**Actions rapides par colonne :**
- **Quick Add** : bouton « + » en haut à droite de la colonne pour ajouter un candidat directement dans ce stage (saisie minimale : Candidat, Poste).

### 1.3 Formulaire Poste (Job Position)

**Accès :** Clic sur une carte poste > menu (engrenage) > Configuration, ou création New.

**Organisation :**
- **Onglet Recruitment** : Département, Lieu, Industrie, Alias email, Type d’emploi, Horaires, Fourchette salariale, Compétences attendues, Société (multi-company), Dates mission, Cible (nombre à recruter), Site web, Recruteur, Intervieweurs, Formulaire d’entretien, Modèle de contrat.
- **Onglet Job Summary** : Description du poste (visible sur le site).
- **Onglet Application Info** : Détails du processus (Time to Answer, Process steps, Days to get an Offer) — texte affiché aux candidats en ligne.
- **Smart buttons** : Nombre de candidatures, etc.
- **Menu** : Configuration, autres actions (publier, archiver, etc.).

### 1.4 Formulaire Candidat (Applicant)

**Accès :** Clic sur une carte candidat dans le Kanban Applications.

**Organisation :**
- **Barre de statut** en haut : les stages sont affichés ; le stage actuel est mis en évidence. Clic sur un stage pour y déplacer la candidature.
- **Boutons d’action** : Send Interview (envoi enquête), autres actions selon contexte (Refuse, Create Employee quand au bon stage).
- **Section Candidate** : Évaluation (étoiles), Candidat (nom), Email, Téléphone, LinkedIn, Poste, Recruteur, Intervieweurs, Tags.
- **Onglet Notes** : Notes internes.
- **Onglet Details** : Diplôme, Disponibilité, Prétentions / Avantages, Proposé / Avantages, Département, Société, Source, Medium, Referred By User.
- **Onglet Skills** : Compétences (liées au référentiel Employees).
- **Chatter** : Messages, activités, historique, pièces jointes.
- **Affichage CV (Résumé Display)** : Si activé en paramètres et fenêtre en plein écran, le CV (PDF) s’affiche à droite de la fiche ; sinon lien dans le chatter (Fichiers).

**Statuts par carte (rappel) :**
- Cercle cliquable en bas à gauche de la carte (Kanban) ou équivalent sur la fiche : choix entre In Progress, Blocked, Ready for Next Stage. La barre colorée sous chaque colonne Kanban se met à jour en conséquence.

---

## 2. Paramètres et Configuration

### 2.1 Settings (Configuration > Settings)

**Sections :**
- **Process** : Send interview survey (oui/non, lien vers Surveys), Salary package configurator (nombre de jours validité offre), Résumé display (CV sur la fiche à droite).
- **In-App Purchases** : Send SMS (lien gestion crédits), Résumé digitization OCR (Do not digitize / Digitize on demand only / Digitize automatically, liens crédits et services).

### 2.2 Stages (personnalisation)

**Depuis le Kanban candidatures :**
- **Nouveau stage** : clic sur « + » (Stage) > saisie du titre > Add.
- **Modifier un stage** : survol du nom du stage > icône (engrenage) > Edit.
- **Formulaire Edit stage** : Stage Name, Email Template (envoi auto à l’entrée), Folded in Kanban, Hired Stage, Job Specific (postes concernés), Show in Referrals + Points, libellés des trois statuts (Tooltips), Requirements (notes internes).
- **Suppression** : Engrenage > Delete ; impossible s’il reste des candidatures dans le stage (message d’erreur).

---

## 3. Communication et Fichiers

### 3.1 Emails

- **Envoi manuel** : Chatter > « Send message » > Full composer > choix du modèle d’email (templates Recruitment), pièces jointes, envoi.
- **Templates** : Recruitment: Applicant Acknowledgement, Interest, Schedule Interview, Refuse, Not interested anymore, etc. ; placeholders dynamiques (nom candidat, poste, etc.).
- **Envoi auto** : configuré par stage (Email Template) ; à l’entrée dans le stage, envoi au candidat.

### 3.2 Enquêtes / Interviews

- **Send Interview** : bouton en haut à gauche de la fiche candidat > popup (destinataires, sujet, corps, modèle, pièces, date limite) > envoi du lien survey.
- Nécessite une adresse email sur la fiche et l’app Surveys activée.

### 3.3 Documents / CV

- CV et pièces stockés dans l’app Documents (dossier Recruitment) et visibles dans le chatter (Fichiers).
- Option « Résumé Display » : affichage du CV à droite en plein écran.

---

## 4. Patterns de Navigation et Actions

- **Tableau de bord > Poste > Candidatures** : navigation hiérarchique poste puis pipeline par stage.
- **Glisser-déposer** : changement de stage dans le Kanban.
- **Barre de stages** sur la fiche candidat : changement de stage sans quitter la fiche.
- **Quick Add** : ajout rapide dans un stage donné.
- **Menus contextuels** : par carte (poste ou candidat) pour Configuration, Refuse, Archive, etc.
- **Smart buttons** : accès direct aux candidatures, rapports, depuis la fiche poste.

---

## 5. Design et Accessibilité

- **Couleurs** : vert / rouge / gris pour les statuts (Ready / Blocked / In Progress) ; colonnes repliées en gris.
- **Responsive** : Kanban et formulaires adaptés ; affichage CV à droite conditionné au plein écran.
- **Messages d’erreur** : ex. suppression de stage impossible si candidatures présentes ; saisie email requise pour envoi d’enquête.

---

## 6. Points pour Miyukini

- **Écrans** : Dashboard postes (Kanban), Kanban candidatures par poste, Formulaire poste (onglets), Formulaire candidat (stages + onglets + chatter).
- **Composants** : Cartes Kanban avec statuts à 3 états, barre de stages cliquable, Quick Add par colonne, Chatter intégré, affichage conditionnel du CV.
- **Navigation** : Hiérarchie Postes > Candidatures > Fiche ; breadcrumb et smart buttons pour ne pas perdre le contexte.
- **Cohérence** : Réutiliser les patterns Odoo déjà analysés (Project, CRM) pour listes, formulaires et états, en les adaptant au vocabulaire Recrutement (stages, candidats, postes).

---

**Document** : Odoo Recruitment — Analyse UI/UX  
**Version** : 1.0  
**Date** : 2026-02-01
