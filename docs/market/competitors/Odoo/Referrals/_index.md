# Odoo Referrals — Index de Documentation

## Contexte

Ce dossier contient l'**analyse complète** de l'application **Referrals** (Parrainage employé) d'Odoo, réalisée selon la méthodologie standardisée. L'analyse couvre la logique métier, les parcours utilisateur, l'UI/UX, les intégrations, les spécifications Opérateurs Miyukini, l'intégration COG et les guides d'implémentation.

**Date d'analyse :** 2026-02-01  
**Source :** Documentation Odoo 18.0/19.0 — Applications HR / Referrals

---

## Structure de Documentation

### 00_logique_metier/
- **[Odoo Referrals - Logique Métier Complète](./00_logique_metier/Odoo%20Referrals%20-%20Logique%20Metier%20Complete.md)**
  - Modèles de données (référent, points, stages, récompenses, niveaux, onboarding, amis)
  - Règles métier et contraintes (points par stage, échange récompenses)
  - Workflow parrainage (partage → candidature → progression → embauche → points)
  - Gamification (niveaux, avatars), configuration (récompenses, slides, niveaux, amis, alertes)
  - Points d'attention pour Miyukini

### 01_parcours_utilisateur/
- **[Odoo Referrals - Parcours Utilisateur Détaillés](./01_parcours_utilisateur/Odoo%20Referrals%20-%20Parcours%20Utilisateur%20Detailles.md)**
  - Personas et rôles (Référent, Officer, Administrator)
  - Parcours d'onboarding (slides, première utilisation)
  - Scénarios d'usage (partage postes, suivi parrainages, récompenses, level up, écran Hired, reporting)
  - Points de friction identifiés
  - Recommandations pour Miyukini

### 02_ui_ux/
- **[Odoo Referrals - Analyse UI/UX](./02_ui_ux/Odoo%20Referrals%20-%20Analyse%20UI%20UX.md)**
  - Dashboard (points, niveau, anneau, View Jobs, Rewards, Referrals/Ongoing/Successful)
  - Onboarding (slides, Skip, Next, Start Now)
  - Écran Hired (choix avatar pour parrainé embauché)
  - View Jobs (cartes postes, partage email/SMS/WhatsApp/réseaux)
  - My Referrals (cartes parrainages, badges, barre de progression)
  - Rewards (boutique, Buy)
  - Configuration et Reporting (admin)

### 03_integrations/
- **[Odoo Referrals - Intégrations Cross-App](./03_integrations/Odoo%20Referrals%20-%20Integrations%20Cross%20App.md)**
  - Dépendances obligatoires (Employees, Recruitment, Website)
  - Intégration avec Recruitment (candidatures, referrer_id, stages, points)
  - Intégration avec Website (postes publiés, liens de suivi)
  - Intégration avec Mail (templates partage, notification responsable récompense)
  - SMS / WhatsApp (IAP, config WhatsApp)
  - Reporting et Documents/Spreadsheet (optionnel)

### 04_specifications_miyukini/
- **[Odoo Referrals - Spécifications Opérateurs Miyukini](./04_specifications_miyukini/Odoo%20Referrals%20-%20Specifications%20Operateurs%20Miyukini.md)**
  - Architecture Opérateurs (7 Opérateurs identifiés)
  - Équipe d'Opérateurs ReferralsService
  - Contrat d'Équipe
  - Mandats de Permission (ReferralsUser, Administrator)
  - Niveaux de sécurité (1–2)
  - Intégration avec les Cores

### 05_integration_cog/
- **[Odoo Referrals - Guide Intégration COG](./05_integration_cog/Odoo%20Referrals%20-%20Guide%20Integration%20COG.md)**
  - Architecture d'intégration COG
  - Patterns (WriteIntent points, achat récompense, événement Recruitment, lien de suivi)
  - Exemples de code (pseudo-code Rust)
  - Gestion des erreurs et rollback
  - Intégration avec MiyuHR, MiyuRecruitment, MiyuWeb, MiyuNotify

### 06_guides_implementation/
- **[Odoo Referrals - Guide Implémentation](./06_guides_implementation/Odoo%20Referrals%20-%20Guide%20Implementation.md)**
  - Architecture technique (crates miyu-referrals)
  - Schémas de données (points, récompenses, niveaux, onboarding, amis)
  - API et contrats
  - Plan de développement par phases (MVP → Complet)
  - Bornage fonctionnel et critères d'acceptation
  - Risques et mitigation

---

## Résumé Exécutif

### Fonctionnalités Principales Identifiées

1. **Points**
   - Crédit à chaque stage atteint par un candidat parrainé (config dans Recruitment)
   - Total earned / to spend ; débit à l'achat de récompense

2. **Récompenses**
   - Catalogue (nom, coût, description, photo, responsable livraison)
   - Achat si solde suffisant ; notification au responsable

3. **Niveaux et Level up**
   - Gamification (niveaux, avatars) ; level up sans coût en points

4. **Partage de postes**
   - View Jobs (postes publiés), lien de suivi, envoi email/SMS/WhatsApp, partage réseaux (Facebook, X, LinkedIn)
   - Email a friend (liste complète de postes)

5. **Mes parrainages**
   - Referrals / Ongoing / Successful ; cartes par candidature (points, barre de progression, stages)

6. **Onboarding et Hired**
   - 4 slides ; Skip / Start Now ; écran Hired (choix avatar pour parrainé embauché)

7. **Configuration et Reporting**
   - Admin : Onboarding, Levels, Friends, Rewards, Alerts ; Reporting (Employees Referral Analysis par canal et par référent)

### Architecture Miyukini Proposée

**7 Opérateurs :**
- ReferralsPoints (points, crédit, débit)
- ReferralsRewards (catalogue, achat)
- ReferralsLevels (niveaux, level up)
- ReferralsShare (partage postes, liens de suivi)
- ReferralsOnboarding (slides, état)
- ReferralsReporting (rapport admin)
- ReferralsUI (interface)

**1 Équipe d'Opérateurs :** ReferralsService

**Correspondance Miyukini :** **MiyukiniReferrals** (ou **MiyuReferrals**) — ReferralsService

**Niveaux de sécurité :** 1 (Standard) à 2 (Sensitive) selon données (points, reporting)

**Intégration Cores :**
- KindMother : Persistance (points, récompenses, achats, niveaux, onboarding, amis)
- Master Butler : Permissions (Referral User / Officer / Administrator)
- WorrySentinel : Données personnelles (référents, parrainages)
- Ever Buddy : Évolution niveaux et récompenses

---

## Statut de l'Analyse

| Document | Statut | Version |
|----------|--------|--------|
| Logique Métier | ✅ Complété | 1.0 |
| Parcours Utilisateur | ✅ Complété | 1.0 |
| UI/UX | ✅ Complété | 1.0 |
| Intégrations Cross-App | ✅ Complété | 1.0 |
| Spécifications Opérateurs Miyukini | ✅ Complété | 1.0 |
| Guide Intégration COG | ✅ Complété | 1.0 |
| Guide Implémentation | ✅ Complété | 1.0 |

---

**Document** : Odoo Referrals — Index de Documentation  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : ✅ Analyse complète à 100% — référence pour implémentation Miyukini (MiyukiniReferrals / MiyuReferrals)
