# Odoo Appraisals — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Appraisals** (Évaluations) d'Odoo (versions 18.0 / 19.0), à partir de la documentation officielle. Il identifie les vues, onglets, composants et patterns de navigation pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18.0/19.0 (Appraisals, Schedule, Conduct, Templates, 360, Goals, Analysis)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Structure de navigation et menus
- Dashboard et cartes Appraisals
- Formulaire Appraisal (onglets Appraisal, Skills, Private Note)
- Vues Goals, Configuration (Templates, Evaluation Scale, 360, Tags)
- Patterns d'interaction (toggles visibilité, boutons Confirm, Mark as Done, Ask Feedback, Meetings)
- Rapports (Appraisal analysis, Skills evolution)

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Structure de Navigation

### 1.1 Menu principal Appraisals

- **Appraisals** (app racine)
  - **Dashboard / liste** : Vue par défaut — cartes ou liste des appraisals
  - **New** : Création d’une nouvelle appraisal (formulaire vide)
  - **Goals** : Liste des objectifs (groupés par employé par défaut)
- **Configuration**
  - **Settings** : Appraisals Plan (mois), Appraisals Automation (case à cocher)
  - **Appraisal Templates** : Liste des templates (Default Template + personnalisés)
  - **Evaluation Scale** : Liste des notes (Needs improvement, Meets expectations, etc.)
  - **360 Feedback** : Dashboard des surveys 360 (nom, responsable, questions, durée moyenne, enregistrés, complétés, certifiés)
  - **Tags** : Tags pour les objectifs (External, Hard Skills, Internal, Programming, Soft Skills, Training)

### 1.2 Entrées secondaires

- Depuis **Employees** : champ **Next Appraisal Date** sur la fiche employé (si Appraisals installé).
- Depuis une **appraisal** : liens vers employé, manager, département, template ; smart button **Meetings** / **No Meeting** ; bouton **Ask Feedback** (si confirmée).
- **Surveys** (app externe) : création/édition des templates ; option « Appraisal » pour les rendre disponibles dans Appraisals.

---

## 2. Dashboard et Cartes Appraisals

### 2.1 Vue par défaut

- **Cartes** (Kanban-like) : une carte par appraisal avec informations clés (employé, date, statut).
- **Indicateur** : point vert sur la carte lorsque l’employé a marqué son feedback « Visible to Manager ».
- **Icône activité** : sous la date sur la carte → ouverture pop-up pour planifier une activité (Meeting).

### 2.2 Clic sur une carte

- Ouverture du **formulaire Appraisal** (détail complet).

### 2.3 Boutons d’action principaux (en-tête formulaire)

- **New** : Nouvelle appraisal (depuis le dashboard).
- **Confirm** : Passe l’appraisal de brouillon à confirmée ; notifie l’employé.
- **Ask Feedback** : Visible uniquement si appraisal confirmée ; ouvre pop-up email (destinataires, message, Answer Deadline).
- **Mark as Done** : Clôture l’appraisal (statut Done) ; le bouton devient **Reopen**.
- **Reopen** : Réouvre une appraisal Done pour modifications ; puis à nouveau Confirm et Mark as Done si besoin.

---

## 3. Formulaire Appraisal

### 3.1 En-tête / champs principaux

- **Employé** : liste déroulante (employee_id) ; à la sélection, Manager, Job Position, Department se remplissent depuis la fiche employé.
- **Appraisal Date** : date de complétion prévue (sélecteur calendrier) ; mise à jour typiquement à la clôture.
- **Next Appraisal Date** : « Ongoing » si plan actif ; sinon date de la prochaine évaluation (mise à jour à la clôture).
- **Appraisal Template** : liste déroulante (Default Template par défaut).

### 3.2 Onglet Appraisal (contenu du template)

- **Employee's Feedback** : sections My work, My future, My feelings (questions du template).
  - Toggle **Not Visible to Manager** / **Visible to Manager** (gris → vert quand visible).
- **Manager's Feedback** : sections Feedback, Evaluation, Improvements.
  - Toggle **Not Visible to Employee** / **Visible to Employee** (gris → vert quand visible).

**Pattern** : Chaque section contient des questions ; réponses en texte libre ou selon le type de question Survey.

### 3.3 Onglet Skills

- **Visibilité** : n’apparaît qu’après **confirmation** de l’appraisal.
- **Contenu** : compétences reprises depuis la fiche employé ; pour chaque compétence :
  - Skill (type / nom)
  - **Skill Level** : liste déroulante (niveaux disponibles)
  - **Progress** : mise à jour automatique selon le niveau
  - **Justification** : champ texte (optionnel, pour expliquer un changement de niveau)
- **Mise à jour** : possible par l’employé (auto-évaluation) et le manager (après entretien) ; les changements sont reflétés sur la fiche employé après clôture.

### 3.4 Onglet Private Note

- **Visibilité** : réservé aux managers ; **invisible** à l’employé (l’onglet n’apparaît pas sur son écran).
- **Contenu** : zone de texte libre pour notes privées manager ; n’impacte pas le Final Rating.
- **Usage** : à tout moment pendant le processus d’appraisal.

### 3.5 Final Rating et clôture

- **Final Rating** : liste déroulante en haut du formulaire (ou zone dédiée) — Needs improvement, Meets expectations, Exceeds expectations, Strongly Exceeds Expectations, Good (ou échelle personnalisée depuis Configuration → Evaluation Scale).
- **Mark as Done** : bouton principal de clôture ; après clic, statut Done et bouton remplacé par **Reopen**.

---

## 4. Planification de réunion

### 4.1 Depuis le dashboard

- Clic sur l’**icône activité** sous la date de l’appraisal → pop-up activité → **Schedule an activity** → Activity Type = **Meeting** → formulaire New Event (heure, participants, option Odoo meeting / vidéocall) → Save & Close. La réunion apparaît au calendrier ; les participants sont notifiés par email.

### 4.2 Depuis la fiche appraisal

- Smart button **Meetings** (ou **No Meeting** si aucune réunion) → ouverture du calendrier / création de réunion ; même flux que ci-dessus.

---

## 5. Goals (Objectifs)

### 5.1 Liste Goals

- **Navigation** : Appraisals → Goals.
- **Vue par défaut** : liste groupée par **Employee** ; clic sur un employé pour déplier ses objectifs.
- **Colonnes / infos** : Name, Created on, Progress (%), Employee.

### 5.2 Formulaire Goal

- **Champs** : Goal (nom), Employee (Manager auto-rempli), Progress (0 %, 25 %, 50 %, 75 %, 100 %), Manager, Deadline, Tags.
- **Onglet Description** : détails ; possibilité de checklist (étapes).
- **Actions** : **Mark as Done** (passe à 100 %, bandeau vert Done sur la carte).

---

## 6. Configuration

### 6.1 Settings

- **Appraisals Plans** : champs numériques (mois) — ex. 6, 6, 12.
- **Appraisals Automation** : case à cocher (planification et confirmation automatiques).

### 6.2 Appraisal Templates

- **Vue liste** : nom du template (ex. Default Template).
- **Clic** : ouverture du template (structure Surveys) — questions Employee's Feedback et Manager's Feedback ; édition des libellés et ajout/suppression de questions.

### 6.3 Evaluation Scale

- **Vue liste** : une ligne par note (Needs improvement, Meets expectations, etc.).
- **New** : ajout d’une ligne ; saisie du libellé.

### 6.4 360 Feedback

- **Dashboard** : une ligne par survey avec Survey Name, Responsible, Questions, Average Duration, Registered, Completed, Certified.
- **Actions par ligne** : **Test** (prévisualisation sans soumettre), **See Results** (analytiques des réponses ; export PDF via icône Print).
- **New** : création d’un nouveau survey (renvoi à l’app Surveys).

### 6.5 Tags

- **Vue liste** : Tags des objectifs (External, Hard Skills, Internal, etc.).
- **New** : ajout d’un tag.

---

## 7. Analyse et rapports

### 7.1 Appraisal analysis

- **Cas d’usage** : « View only the user's appraisals » (filtre par employé / manager).
- **Vues** : groupement par statut (draft, confirmed, done) ; filtres par statut.

### 7.2 Skills evolution

- **Cas d’usage** : Assess highest improvement ; Identify employees with specific skills.
- **Rapport** : évolution des compétences dans le temps (lié aux appraisals et mises à jour Skills).

---

## 8. Patterns d’interaction et feedback

| Élément | Pattern |
|--------|---------|
| **Toggles visibilité** | Gris = caché, Vert = visible ; libellés « Not Visible to Manager » / « Visible to Manager » et « Not Visible to Employee » / « Visible to Employee » |
| **Confirm** | Une fois les champs obligatoires renseignés (employé, date, template) ; déclenche notification employé et déblocage des onglets (Skills, etc.) |
| **Ask Feedback** | Pop-up email avec destinataires, message, Answer Deadline |
| **Meetings** | Intégration activités Odoo (Meeting) ; option Odoo meeting (vidéocall URL) |
| **Mark as Done / Reopen** | Irréversibilité visuelle (bouton qui change) ; Reopen pour corriger puis reclôturer |
| **Indicateur carte** | Point vert = employé a rendu son feedback visible au manager |

---

**Document** : Odoo Appraisals — Analyse UI/UX  
**Version** : 1.0  
**Date** : 2026-02-01
