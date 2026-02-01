# Odoo Referrals — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Referrals** d'Odoo, identifiant les dépendances, flux de données et mécanismes d'intégration.

**Source d'analyse :** Documentation Odoo 18.0/19.0 — Applications HR / Referrals

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances obligatoires et optionnelles
- Flux de données avec Employees, Recruitment, Website
- Intégration points / stages (Recruitment), partage (Mail, SMS, WhatsApp), reporting, Documents/Spreadsheet
- Événements et hooks typiques

---

## 1. Dépendances Principales

### 1.1 Modules Requis (obligatoires)

**Dépendances explicites (documentation) :**
- **Employees** (`hr`) : le référent est un employé ; identification de l’utilisateur courant comme référent.
- **Recruitment** (`hr_recruitment`) : candidatures (`hr.applicant`), stages (`hr.recruitment.stage`), champ `referrer_id` sur candidature, configuration des points par stage (« Show in Referrals », Points).
- **Website** : publication des postes, pages des offres d’emploi, liens de suivi utilisés dans les partages (email, SMS, WhatsApp, réseaux sociaux).

Sans l’une de ces trois apps, Referrals ne peut pas fonctionner.

### 1.2 Modules Optionnels

- **Mail** : templates « Send Job Offer by Mail » (partage poste ou liste de postes), notifications (ex. responsable récompense à l’achat).
- **IAP** (In-App Purchases) : envoi **SMS** depuis les cartes postes (crédits requis).
- **WhatsApp** : envoi WhatsApp depuis les cartes postes (configuration Odoo WhatsApp).
- **Documents** / **Spreadsheet** : option **Insert in Spreadsheet** dans le rapport Referrals (Reporting).

---

## 2. Intégrations Détaillées

### 2.1 Intégration avec Employees (HR)

**Flux :**
```
Referrals app → utilisateur courant = employé (hr.employee / res.users)
             → referrer_id sur hr.applicant = employé référent
```

**Mécanismes :**
- L’app Referrals est utilisée par des **employés** (ou utilisateurs liés à un employé). Le référent est l’employé associé à l’utilisateur connecté.
- Lors de la création d’une candidature (formulaire site, alias email, etc.) avec **lien de suivi** Referrals, le système enregistre `referrer_id` = employé courant.
- Dashboard Referrals : photo, niveau, points, parrainages — tout est indexé par cet employé.

**Champs liés :**
- `hr.employee` (ou res.users) : identité du référent.
- `hr.applicant.referrer_id` : lien candidature → référent.

**Recommandations pour Miyukini :**
- Intégration native avec l’Opérateur HR / Employees (MiyuHR) pour résoudre l’identité du référent et les droits d’accès (employé actif, société, etc.).

---

### 2.2 Intégration avec Recruitment

**Flux :**
```
hr.applicant (referrer_id) + hr.recruitment.stage (points, show_in_referrals)
    → progression candidature dans les stages
    → attribution des points au référent à chaque stage atteint
    → affichage dans Referrals (My Referrals, Ongoing, Successful)
```

**Mécanismes :**
- **Candidatures** : Referrals lit les candidatures où `referrer_id` = employé courant (ou tous pour le reporting admin). Affichage : nom, sujet, poste, recruteur, points gagnés, stages avec checkmarks.
- **Stages** : la configuration des **points** par stage et « Show in Referrals » se fait dans **Recruitment** (Configuration > Stages), pas dans Referrals. Referrals consomme cette configuration pour calculer et afficher les points.
- **Embauche** : quand une candidature passe au stage « Hired » (Contract Signed), Referrals considère le parrainage comme **Successful** ; au prochain accès du référent, écran « Hired » (choix d’avatar).
- **Reporting** : données agrégées par canal (medium) et par employé (nombre référés, embauchés, refusés, en cours) — source : `hr.applicant` + UTM / referrer.

**Champs liés :**
- `hr.applicant` : referrer_id, stage_id, job_id, utm_*, etc.
- `hr.recruitment.stage` : points, show_in_referrals (ou équivalent).

**Recommandations pour Miyukini :**
- Réutiliser le même modèle de candidature et de stages que l’équivalent Recruitment (MiyuRecruitment). Champ « référent » sur la candidature ; synchronisation des événements « passage de stage » pour créditer les points dans Referrals sans dupliquer la logique de recrutement.

---

### 2.3 Intégration avec Website

**Flux :**
```
Referrals > View Jobs → liste des postes publiés (website_published)
         → Job Page → ouvre l’URL du poste sur le site
         → partage email / SMS / WhatsApp / réseaux → lien de suivi vers page poste (ou liste)
         → candidat postule sur le site → candidature créée avec referrer_id (tracking)
```

**Mécanismes :**
- Seuls les **postes publiés** sur le site sont visibles dans Referrals (View Jobs).
- Les **liens de suivi** dans les emails/SMS/WhatsApp et réseaux sociaux pointent vers les pages Website (poste ou liste de postes) et portent un paramètre (ou token) pour identifier le référent à l’application.
- Formulaire de candidature sur le site : enregistrement de `referrer_id` à partir du lien de suivi.

**Recommandations pour Miyukini :**
- Intégration avec l’Opérateur Web / Website (MiyuWeb) pour : liste des postes publiés, génération des URLs et des liens de suivi, récupération du référent dans le formulaire de candidature.

---

### 2.4 Intégration avec Mail

**Flux :**
```
Referrals > Send Email (poste ou liste) → template « Send Job Offer by Mail »
         → champ Email, Subject, Body avec lien de suivi
         → envoi mail
Referrals > Rewards > Buy → alerte au Gift Responsible (notification / mail)
```

**Mécanismes :**
- Modèles d’email (mail.template) pour « Send Job Offer by Mail » (poste unique ou liste), avec placeholders (titre du poste, lien de suivi).
- À l’achat d’une récompense : notification au responsable (Gift Responsible) pour livraison.

**Recommandations pour Miyukini :**
- Opérateur Notify (MiyuNotify) pour envoi d’emails de partage et notifications (achat récompense), avec templates gouvernés et traçabilité.

---

### 2.5 Intégration SMS / WhatsApp (IAP, WhatsApp)

**Flux :**
```
Referrals > View Jobs > Send SMS → popup Recipient, Body → envoi (crédits IAP)
Referrals > View Jobs > Send WhatsApp → popup Recipient, message → envoi (config WhatsApp)
```

**Mécanismes :**
- SMS : dépendance IAP, crédits achetés.
- WhatsApp : module WhatsApp Odoo configuré ; pas de dépendance Referrals directe autre que l’usage de l’API d’envoi.

**Recommandations pour Miyukini :**
- Intégration optionnelle avec des canaux de notification (SMS, WhatsApp) via MiyuNotify ou équivalent, avec gouvernance des coûts et des consentements.

---

### 2.6 Reporting et Documents / Spreadsheet

**Flux :**
```
Referrals > Reporting → Employees Referral Analysis
         → données : hr.applicant (referrer_id, stage_id, utm_medium_id, etc.)
         → vue Bar Chart (empilée) ou Pivot
         → option Insert in Spreadsheet → insertion dans un document (Documents / Spreadsheet)
```

**Mécanismes :**
- Rapport basé sur les candidatures avec référent, agrégation par canal (medium) et par étape (Not Hired, In Progress, Hired) ; filtres par date, par employé en vue Pivot.
- Insert in Spreadsheet : export du tableau de rapport vers l’app Documents/Spreadsheet si installée.

**Recommandations pour Miyukini :**
- Opérateur de reporting dédié ou partie de MiyuRecruitment / MiyuReferrals, avec export contrôlé (données personnelles, droits Administrator). Intégration optionnelle avec un module type Spreadsheet/Documents pour insertion de tableaux.

---

## 3. Synthèse des Flux

| App          | Flux principal                                                                 |
|-------------|----------------------------------------------------------------------------------|
| Employees   | Identité du référent ; referrer_id sur candidature                              |
| Recruitment | Candidatures, stages, points par stage ; embauche → Successful ; reporting        |
| Website     | Postes publiés ; liens de suivi ; formulaire candidature avec referrer_id      |
| Mail        | Templates partage postes ; notification responsable récompense                 |
| IAP/WhatsApp| Envoi SMS / WhatsApp depuis View Jobs (optionnel)                              |
| Documents   | Insert in Spreadsheet depuis Reporting (optionnel)                               |

---

## 4. Points d’Attention pour Miyukini

- **Cohérence des identifiants** : référent = employé (MiyuHR) ; candidature = enregistrement Recruitment (MiyuRecruitment) avec champ référent.
- **Événements** : passage de stage en Recruitment doit déclencher le crédit de points côté Referrals (sans couplage fort : événement ou file de traitement).
- **Liens de suivi** : génération sécurisée, non falsifiable, et respect de la vie privée (pas d’exposition inutile de données).
- **Reporting** : accès réservé aux administrateurs ; agrégation et export conformes à la politique de données (anonymisation ou périmètre autorisé).

---

**Document** : Odoo Referrals — Intégrations Cross-App  
**Version** : 1.0  
**Date** : 2026-02-01
