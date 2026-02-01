# Odoo Appraisals — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **Appraisals** (Évaluations) d'Odoo, identifiant les personas, scénarios d'usage, processus de planification à clôture et points de friction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18.0/19.0 (Schedule appraisals, Conduct appraisals, Templates, 360 Feedback, Goals)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles (HR / Admin, Manager, Employé)
- Parcours de planification (automatique et manuelle)
- Scénarios : auto-évaluation, feedback manager, 360, réunion, clôture, objectifs
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles Utilisateurs

### 1.1 Administrateur / HR (Configuration)

**Profil :**
- Rôle : Configuration globale Appraisals (plans, templates, échelle d'évaluation, 360, tags objectifs)
- Responsabilités : Activer la planification automatique, définir les plans (6m, 6m, 12m), créer ou modifier les templates, gérer l'échelle de notation, créer les surveys 360, gérer les tags des objectifs

**Besoins :**
- Accès Configuration → Settings (Appraisals Plan, Appraisals Automation)
- Configuration → Appraisal Templates (liste, édition Default Template ou création)
- Configuration → Evaluation Scale (ajout de notes)
- Configuration → 360 Feedback (dashboard, création surveys)
- Configuration → Tags (objectifs)
- Vue sur toutes les appraisals et rapports (Appraisal analysis, Skills evolution)

**Permissions :** Droits complets Appraisals ; accès Surveys pour marquer les templates « Appraisal ».

### 1.2 Manager

**Profil :**
- Rôle : Planifier, conduire et clôturer les évaluations de ses subordonnés
- Responsabilités : Créer des appraisals manuelles (promotion, transfert), confirmer les appraisals, demander du 360 feedback, remplir le Manager's Feedback, planifier la réunion, revoir objectifs et compétences, attribuer la note finale, ajouter une note privée, marquer Done

**Besoins :**
- Liste / cartes des appraisals de son périmètre (ses équipes)
- Création manuelle appraisal (employé, date, template) → Confirm
- Bouton Ask Feedback (360) sur une appraisal confirmée
- Remplissage Manager's Feedback (Feedback, Evaluation, Improvements)
- Toggle Visible to Employee (quand il souhaite dévoiler son feedback)
- Planification réunion (activité Meeting depuis dashboard ou fiche appraisal)
- Onglet Skills : revue et mise à jour des niveaux + justifications
- Goals : consultation et mise à jour de l'avancement
- Final Rating (liste déroulante) et Private Note
- Mark as Done / Reopen

**Permissions :** Accès aux appraisals des employés dont il est manager (hiérarchie / département).

### 1.3 Employé

**Profil :**
- Rôle : Réaliser l'auto-évaluation et mettre à jour ses compétences dans le cadre de l'appraisal
- Responsabilités : Remplir Employee's Feedback (My work, My future, My feelings), mettre à jour les niveaux de compétences et justifications, passer le toggle Visible to Manager, participer à la réunion

**Besoins :**
- Accès à ses propres appraisals (liste / carte)
- Ouverture de l'appraisal via lien email ou Appraisals app
- Remplissage des sections Employee's Feedback
- Onglet Skills : mise à jour Skill Level et Justification si changement
- Toggle Not Visible to Manager → Visible to Manager (indicateur vert côté manager)
- Pas d'accès à Manager's Feedback tant que le manager ne le rend pas visible
- Pas d'accès à l'onglet Private Note
- Consultation éventuelle des objectifs (Goals) et de la réunion (calendrier)

**Permissions :** Lecture/écriture sur ses propres appraisals (feedback employé + skills) ; pas de droit sur les autres appraisals ni sur la configuration.

---

## 2. Parcours de Planification

### 2.1 Planification automatique

1. **Activation** : Appraisals → Configuration → Settings → cocher Appraisals Automation (et configurer Appraisals Plans : 6, 6, 12 mois).
2. **Effet** : Odoo crée et peut confirmer automatiquement les appraisals selon le plan ; Next Appraisal Date sur la fiche employé affiche « Ongoing » puis la date de la prochaine évaluation après clôture.
3. **Modification du plan** : Changer les mois dans Appraisals Plans ; tous les employés avec Next Appraisal Date vide sont mis à jour.

**Points d'attention :** Comprendre l’impact « every employee record whose Next Appraisal Date is empty » avant de modifier le plan.

### 2.2 Planification manuelle

1. **Déclencheur** : Promotion, transfert de poste ou de département, évaluation à la demande.
2. **Actions** : Appraisals → New → sélectionner l’employé (Manager, Job Position, Department se remplissent) → Appraisal Date → Template (Default ou autre) → **Confirm**.
3. **Résultat** : Appraisal en statut Confirmed ; l’employé est notifié par email (lien vers l’appraisal).
4. **Suite** : Employé et manager peuvent commencer à remplir l’appraisal.

---

## 3. Scénarios d'Usage Principaux

### 3.1 Auto-évaluation (Employé)

1. Réception email de confirmation d’appraisal (lien).
2. Ouverture de l’appraisal dans l’app Appraisals.
3. Remplissage des questions **Employee's Feedback** (My work, My future, My feelings).
4. Onglet **Skills** : pour chaque compétence dont le niveau a changé, sélectionner le nouveau niveau, renseigner **Justification** si utile.
5. Passer le toggle **Not Visible to Manager** → **Visible to Manager** (réponses visibles par le manager ; point vert sur la carte appraisal).

**Friction possible :** L’employé peut oublier de rendre visible son feedback ; le manager ne voit pas l’avancement tant que le toggle n’est pas activé.

### 3.2 Feedback manager et 360

1. **Option 360** : Sur l’appraisal confirmée, le manager clique **Ask Feedback** → choisit les destinataires (collègues) → adapte le message et la date limite → Send. Les collègues reçoivent l’email et remplissent le survey.
2. Manager consulte les objectifs (Goals) et l’historique des compétences.
3. Manager remplit **Manager's Feedback** (Feedback, Evaluation, Improvements).
4. Manager peut laisser **Not Visible to Employee** jusqu’à l’entretien, puis passer à **Visible to Employee**.

**Friction possible :** Délai des réponses 360 ; risque de feedback manager visible trop tôt si le toggle est activé avant l’entretien.

### 3.3 Réunion d’évaluation

1. **Planification** : Depuis le dashboard Appraisals (icône activité sous la date) ou depuis la fiche appraisal (bouton **Meetings** / **No Meeting**) → Schedule an activity → Type = Meeting → renseigner heure, participants (employé par défaut), option Odoo meeting (vidéocall) → Save & Close.
2. **Contenu** : Discussion des deux feedbacks (employé + manager), revue des Skills et Goals, ajustements éventuels des niveaux et de l’avancement des objectifs.
3. Les modifications Skills peuvent être faites après la réunion si le manager n’avait pas toutes les informations avant.

### 3.4 Clôture de l’appraisal

1. Manager ouvre l’onglet **Private Note** si besoin (invisible à l’employé).
2. Manager sélectionne **Final Rating** (Needs improvement, Meets expectations, Exceeds expectations, etc. — ou échelle personnalisée).
3. Manager clique **Mark as Done** → statut Done ; bouton devient **Reopen**. Aucune modification possible sauf Reopen → Confirm → modifications → Mark as Done.
4. **Next Appraisal Date** (sur fiche employé) mise à jour selon le plan si activé.

### 3.5 Objectifs (Goals)

- **Création** : Appraisals → Goals → New → Goal, Employee (Manager auto-rempli), Progress, Deadline, Tags, Description (et checklist si besoin).
- **Mise à jour** : Pendant ou hors appraisal, le manager ouvre le goal → change Progress (0–100 %) ; ajout de notes dans Description recommandé.
- **Clôture** : **Mark as Done** → Progress 100 %, objectif marqué terminé (affichage vert 100 % sur le dashboard Goals).

---

## 4. Points de Friction Identifiés

| Friction | Description | Recommandation Miyukini |
|----------|-------------|--------------------------|
| Visibilité toggle | L’employé peut oublier de passer « Visible to Manager » ; le manager ne sait pas si l’auto-évaluation est terminée sans ouvrir l’appraisal | Rappel ou indicateur clair (badge « En attente de visibilité ») ; notification manager quand visible |
| 360 délai | Les retours 360 peuvent arriver tard ; risque de bloquer le feedback manager | Date limite visible ; option « continuer sans attendre tous les 360 » |
| Template Surveys | Les templates sont dans Surveys ; il faut penser à cocher « Appraisal » pour les voir dans Appraisals | Documentation interne ; possiblement un wizard ou lien direct depuis Appraisals |
| Reopen peu visible | Après Mark as Done, modifier nécessite Reopen puis à nouveau Confirm / Mark as Done | Affichage explicite du statut Done et du bouton Reopen ; message de confirmation avant réouverture |
| Next Appraisal Date vide vs Ongoing | Comportement « plan met à jour tous les employés avec date vide » peut surprendre | Avertissement en configuration ; log d’impact si possible |

---

## 5. Recommandations pour Miyukini

- **Parcours guidé** : Pour l’employé, un parcours pas à pas (Feedback → Skills → Visible to Manager) avec rappels légers.
- **Notifications gouvernées** : Confirmation d’appraisal, rappel 360, rappel réunion, clôture — via MiyuNotify avec Mandat et sans spam.
- **Mandats** : Chaque action (créer, confirmer, demander 360, noter, clôturer) doit être couverte par un Mandat de Permission (StrongFather, Master Butler).
- **Private Note** : Niveau de sécurité élevé (WorrySentinel) ; audit des accès.
- **TAMR** : Réunion et décision finale (rating, note privée) comme points d’intervention humaine explicites.

---

**Document** : Odoo Appraisals — Parcours Utilisateur Détaillés  
**Version** : 1.0  
**Date** : 2026-02-01
