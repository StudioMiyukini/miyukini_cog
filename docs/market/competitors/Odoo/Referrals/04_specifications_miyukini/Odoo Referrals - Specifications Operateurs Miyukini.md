# Odoo Referrals — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application Referrals d'Odoo, en respectant l'architecture COG et la gouvernance Miyukini.

**Références :**
- [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- [Odoo Referrals - Logique Métier](../00_logique_metier/Odoo%20Referrals%20-%20Logique%20Metier%20Complete.md)

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour l’équivalent Referrals
- Contrat d’équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy)

**Hors scope :**
- Implémentation technique détaillée (voir Guide d’Implémentation)
- Spécifications UI/UX détaillées (document dédié)

---

## 1. Architecture Opérateurs

### 1.1 Vue d’ensemble

L’équivalent Referrals dans Miyukini s’appuie sur un **service COG Miyukini Referrals** (ou **MiyukiniReferrals** / **MiyuReferrals**), avec des **Opérateurs spécialisés** pour le parrainage employé : points, récompenses, niveaux, partage de postes, reporting.

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **ReferralsPoints** | Gestion des points (crédit par stage, total, à dépenser) | Opérateur de Service |
| **ReferralsRewards** | Catalogue et échange points → récompenses | Opérateur de Service |
| **ReferralsLevels** | Niveaux et avatars (gamification) | Opérateur de Service |
| **ReferralsShare** | Partage de postes (liens de suivi, email/SMS/WhatsApp, réseaux) | Opérateur de Service |
| **ReferralsOnboarding** | Slides et première utilisation | Opérateur de Service |
| **ReferralsReporting** | Rapports (canaux, référents, embauchés) | Opérateur de Service |
| **ReferralsUI** | Interface Referrals (dashboard, vues, configuration) | Opérateur d’Interface |

### 1.2 Équipe d’Opérateurs : ReferralsService

**Définition :**
> **ReferralsService est une Équipe d’Opérateurs qui collabore sous règles explicites pour délivrer le service de parrainage employé (points, récompenses, partage, reporting).**

**Composition :**
- ReferralsPoints (niveau sécurité 1–2)
- ReferralsRewards (niveau sécurité 1–2)
- ReferralsLevels (niveau sécurité 1)
- ReferralsShare (niveau sécurité 1)
- ReferralsOnboarding (niveau sécurité 1)
- ReferralsReporting (niveau sécurité 2 — données agrégées, admin)
- ReferralsUI (niveau sécurité 1)

**Contrat d’équipe :** voir section 2.

---

## 2. Opérateurs Détaillés

### 2.1 ReferralsPoints

**Rôle :** Gestion des **points** du référent (crédit à chaque stage atteint par un candidat parrainé, total accumulé, points à dépenser).

**Capacités :**
- Lecture du total points earned et points to spend pour l’employé courant
- Crédit des points à la réception d’un événement « candidature parrainée a atteint le stage X » (intégration Recruitment)
- Débit des points à l’achat d’une récompense (coordination avec ReferralsRewards)

**Niveau de sécurité :** 1–2 (Standard à Sensitive selon exposition des données par référent)

**Gouvernance :**
- **KindMother** : persistance des mouvements de points et des soldes
- **Master Butler** : droits de lecture (son propre solde) et d’utilisation (échange)
- **WorrySentinel** : niveau de sécurité et traçabilité

**Contrat d’équipe :**
- Consomme : événements Recruitment (passage de stage pour une candidature avec référent)
- Expose : `points.balance`, `points.credit`, `points.debit` (interne)
- Consomme : ReferralsRewards (débit à l’achat)

**Mandat de Permission requis :**
- Lecture du solde : Mandat ReferralsUser (ou équivalent)
- Crédit : déclenché par le flux Recruitment (Mandat côté Recruitment)

### 2.2 ReferralsRewards

**Rôle :** Catalogue des **récompenses** et **échange points contre récompense** (achat).

**Capacités :**
- Liste des récompenses (nom, coût, description, photo, responsable livraison)
- Achat : vérification du solde (ReferralsPoints), débit, enregistrement de l’achat, notification au responsable

**Niveau de sécurité :** 1–2

**Gouvernance :**
- **StrongFather** : pas de décision métier forte (l’achat est une action utilisateur autorisée si solde suffisant)
- **KindMother** : persistance des récompenses et des achats
- **Master Butler** : création/modification des récompenses réservée aux Administrators ; achat réservé aux Referral User / Officer / Admin

**Contrat d’équipe :**
- Consomme : ReferralsPoints (débit)
- Expose : `rewards.list`, `rewards.buy`
- Configuration : ReferralsConfig (ou Admin) pour CRUD récompenses

**Mandat de Permission requis :**
- Achat : Mandat ReferralsUser + solde suffisant
- Création/édition récompense : Mandat Administrator Referrals/Recruitment

### 2.3 ReferralsLevels

**Rôle :** **Niveaux** et **avatars** (gamification) — aucun impact fonctionnel, uniquement affichage et « level up ».

**Capacités :**
- Lecture des niveaux (nom, points requis total, image)
- Calcul du niveau actuel du référent à partir du total points earned
- Enregistrement du « level up » (changement d’avatar affiché) sans déduction de points

**Niveau de sécurité :** 1

**Gouvernance :**
- **KindMother** : persistance des niveaux (config) et du niveau actuel par utilisateur
- **Master Butler** : modification des niveaux réservée aux Administrators
- **Ever Buddy** : évolution des niveaux (dépréciation, compatibilité)

**Contrat d’équipe :**
- Consomme : ReferralsPoints (total earned pour calcul du niveau)
- Expose : `levels.current`, `levels.up` (action level up)

**Mandat de Permission requis :**
- Level up : Mandat ReferralsUser
- Configuration niveaux : Mandat Administrator

### 2.4 ReferralsShare

**Rôle :** **Partage des postes** : génération des liens de suivi, envoi email / SMS / WhatsApp, partage réseaux sociaux (liste des postes publiés).

**Capacités :**
- Liste des postes « publiés » (intégration Website / Recruitment)
- Génération de liens de suivi (référent identifié) vers page poste ou liste de postes
- Envoi email (template) avec lien de suivi
- Envoi SMS / WhatsApp (optionnel, selon modules)
- Ouverture page poste pour partage Facebook / X / LinkedIn (lien de suivi)

**Niveau de sécurité :** 1

**Gouvernance :**
- **Master Butler** : droit de partager (Referral User et au-dessus)
- **WorrySentinel** : pas d’exposition de données sensibles dans les liens

**Contrat d’équipe :**
- Consomme : Recruitment (postes publiés), Website (URLs), MiyuNotify (email/SMS/WhatsApp)
- Expose : `share.jobs_list`, `share.job_link`, `share.send_email`, `share.send_sms`, `share.send_whatsapp`

**Mandat de Permission requis :**
- Partager : Mandat ReferralsUser

### 2.5 ReferralsOnboarding

**Rôle :** **Slides d’onboarding** et état « première utilisation » (Skip, Start Now).

**Capacités :**
- Lecture des slides (ordre, texte, image, société)
- Marquer l’onboarding comme terminé (Start Now) ou Skip
- Affichage conditionnel : onboarding vs dashboard vs écran Hired

**Niveau de sécurité :** 1

**Gouvernance :**
- **KindMother** : persistance des slides et de l’état « onboarding vu » par utilisateur
- **Master Butler** : édition des slides réservée aux Administrators

**Contrat d’équipe :**
- Expose : `onboarding.slides`, `onboarding.complete`, `onboarding.skip`
- Consomme : ReferralsUI pour affichage

**Mandat de Permission requis :**
- Compléter / Skip : Mandat ReferralsUser
- Configuration slides : Mandat Administrator

### 2.6 ReferralsReporting

**Rôle :** **Rapports** (Employees Referral Analysis) : par canal (medium), par employé (référés, embauchés, refusés, en cours). Réservé aux Administrators.

**Capacités :**
- Agrégation des candidatures avec référent : par utm_medium (ou canal), par stage (Not Hired, In Progress, Hired)
- Vue Pivot : par employé (référent), total référés, embauchés
- Filtres (date, société)
- Export / Insert in Spreadsheet (optionnel)

**Niveau de sécurité :** 2 (Sensitive) — données agrégées et identification des référents

**Gouvernance :**
- **Master Butler** : accès réservé aux Administrators
- **WorrySentinel** : conformité données personnelles, audit des accès
- **KindMother** : pas d’écriture ; lecture sur données Recruitment + Referrals

**Contrat d’équipe :**
- Consomme : Recruitment (candidatures, referrer_id, stage_id, utm_*), ReferralsPoints (optionnel pour cohérence)
- Expose : `reporting.referral_analysis` (données agrégées)

**Mandat de Permission requis :**
- Accès reporting : Mandat Administrator Referrals/Recruitment

### 2.7 ReferralsUI

**Rôle :** **Interface** Referrals : dashboard, View Jobs, My Referrals, Rewards, Onboarding, Hired (choix avatar), Configuration (Onboarding, Levels, Friends, Rewards, Alerts), Reporting.

**Capacités :**
- Affichage du dashboard (points, niveau, anneau, boutons)
- Navigation vers View Jobs, Rewards, My Referrals, Email a friend
- Écran Onboarding, écran Hired, formulaires Configuration et Reporting
- Pas de logique métier : délégation aux Opérateurs ReferralsPoints, ReferralsRewards, ReferralsLevels, ReferralsShare, ReferralsOnboarding, ReferralsReporting

**Niveau de sécurité :** 1

**Gouvernance :**
- **Master Butler** : visibilité des menus selon rôle (User / Officer / Administrator)

**Contrat d’équipe :**
- Consomme : tous les autres Opérateurs Referrals pour affichage et actions
- Expose : écrans et actions UI

**Mandat de Permission requis :**
- Accès Referrals : Mandat ReferralsUser (ou Officer / Administrator)

---

## 3. Contrat d’Équipe ReferralsService

**Membres :** ReferralsPoints, ReferralsRewards, ReferralsLevels, ReferralsShare, ReferralsOnboarding, ReferralsReporting, ReferralsUI

**Flux autorisés (résumé) :**
- ReferralsUI → tous les autres (lecture + actions)
- ReferralsPoints ↔ ReferralsRewards (débit à l’achat)
- ReferralsPoints → ReferralsLevels (total earned pour niveau)
- ReferralsShare → Recruitment / Website / MiyuNotify (postes, liens, envoi)
- ReferralsReporting → Recruitment (lecture candidatures + referrer)
- ReferralsOnboarding → ReferralsUI (état slides, complete, skip)

**Direction des flux :** UI comme point d’entrée ; pas de communication directe entre Opérateurs métier sans passer par les Cores (KindMother pour persistance, Master Butler pour permissions).

**Types d’échanges :** requêtes lecture, commandes (buy, level up, share, complete onboarding), événements (crédit points depuis Recruitment).

**Niveau de validation :** selon politique (Mandat ReferralsUser pour usage courant, Mandat Administrator pour configuration et reporting).

---

## 4. Mandats de Permission

| Contexte | Mandat | Émetteur |
|----------|--------|----------|
| Accès app Referrals (dashboard, jobs, parrainages, récompenses) | ReferralsUser (ou Officer / Admin) | StrongFather (délégation selon rôle Recruitment) |
| Achat récompense | ReferralsUser + solde suffisant | StrongFather / Master Butler |
| Level up | ReferralsUser | Master Butler |
| Partager postes | ReferralsUser | Master Butler |
| Configuration (Onboarding, Levels, Friends, Rewards, Alerts) | Administrator Referrals/Recruitment | StrongFather |
| Reporting | Administrator Referrals/Recruitment | StrongFather |

---

## 5. Intégration avec les Cores

- **StrongFather** : pas de décision métier forte dans Referrals (autorisations déléguées via Mandats). Décision d’embauche reste dans Recruitment.
- **KindMother** : persistance points, récompenses, achats, niveaux, onboarding, amis (avatars), alertes. WriteIntent pour toute écriture.
- **Master Butler** : permissions Referral User / Officer / Administrator ; contrôle accès configuration et reporting.
- **WorrySentinel** : niveau de sécurité 1–2 selon données (identité référent, candidats parrainés, reporting) ; traçabilité et audit.
- **Ever Buddy** : évolution des niveaux et récompenses (dépréciation, compatibilité).

---

## 6. Correspondance Miyukini

**Service proposé :** **MiyukiniReferrals** ou **MiyuReferrals** — ReferralsService

**Niveaux de sécurité :** 1 (Standard) pour dashboard, partage, niveaux, onboarding ; 2 (Sensitive) pour points détaillés et reporting.

**Intégration obligatoire :** MiyuHR (employé référent), MiyuRecruitment (candidatures, stages, points), MiyuWeb (postes publiés, liens de suivi). Optionnel : MiyuNotify (email, SMS, WhatsApp, notifications).

---

**Document** : Odoo Referrals — Spécifications Opérateurs Miyukini  
**Version** : 1.0  
**Date** : 2026-02-01
