# Odoo Social Marketing — Index de Documentation

## Contexte

Ce dossier contient l'**analyse complète** de l'application **Social Marketing** d'Odoo, réalisée selon la méthodologie standardisée. L'analyse couvre la logique métier, les parcours utilisateur, l'UI/UX, les intégrations, les spécifications Opérateurs Miyukini, l'intégration COG et les guides d'implémentation.

**Date d'analyse :** 2026-02-01  
**Source :** Documentation Odoo 18.0/19.0

---

## Structure de Documentation

### 00_logique_metier/
- **[Odoo Social Marketing - Logique Métier Complète](./00_logique_metier/Odoo%20Social%20Marketing%20-%20Logique%20Metier%20Complete.md)**
  - Modèles de données (social.account, social.stream, social.post, campagnes, visiteurs)
  - Règles métier et contraintes
  - Workflows et transitions d'état (posts, campagnes)
  - Création de leads depuis les commentaires
  - Insights et push notifications
  - Points d'attention pour Miyukini

### 01_parcours_utilisateur/
- **[Odoo Social Marketing - Parcours Utilisateur Détaillés](./01_parcours_utilisateur/Odoo%20Social%20Marketing%20-%20Parcours%20Utilisateur%20Detailles.md)**
  - Personas et rôles utilisateurs (Content Manager, Community Manager, CRM/Sales, Admin)
  - Parcours d'onboarding (connexion comptes, premier post)
  - Scénarios d'usage principaux
  - Points de friction identifiés
  - Recommandations pour Miyukini

### 02_ui_ux/
- **[Odoo Social Marketing - Analyse UI/UX](./02_ui_ux/Odoo%20Social%20Marketing%20-%20Analyse%20UI%20UX.md)**
  - Tableau de bord Feed (colonnes streams)
  - Vues Posts (Kanban, Calendar, List, Pivot)
  - Vues Campagnes (Kanban, List)
  - Formulaire détail post et options (Post on, Message, Campaign, When, Push)
  - Configuration (Social Media, Social Accounts, Social Streams)
  - Visiteurs (Kanban, List, Graph)
  - Patterns de navigation

### 03_integrations/
- **[Odoo Social Marketing - Intégrations Cross-App](./03_integrations/Odoo%20Social%20Marketing%20-%20Integrations%20Cross%20App.md)**
  - UTM (campagnes, suivi revenus / leads / devis)
  - CRM (création leads depuis commentaires)
  - Sales & Invoicing (métriques campagnes)
  - Website (visiteurs, push notifications)
  - Email Marketing (mailings dans campagnes)
  - SMS Marketing (SMS dans campagnes)
  - APIs externes (Facebook, Instagram, LinkedIn, Twitter, YouTube)
  - Mécanismes d'intégration

### 04_specifications_miyukini/
- **[Odoo Social Marketing - Spécifications Opérateurs Miyukini](./04_specifications_miyukini/Odoo%20Social%20Marketing%20-%20Specifications%20Operateurs%20Miyukini.md)**
  - Architecture Opérateurs (8 Opérateurs identifiés)
  - Équipe d'Opérateurs SocialMarketingService
  - Contrat d'Équipe
  - Mandats de Permission (comptes, posts, campagnes, leads, visiteurs)
  - Niveaux de sécurité (1-2)
  - Intégration avec les Cores
  - **Correspondance Miyukini** : MiyuSocial / MiyukiniSocial (SocialMarketingService)

### 05_integration_cog/
- **[Odoo Social Marketing - Guide Intégration COG](./05_integration_cog/Odoo%20Social%20Marketing%20-%20Guide%20Integration%20COG.md)**
  - Architecture d'intégration COG
  - Patterns WriteIntent et Mandates
  - Exemples de code pseudo-Rust (connexion compte, création post, création lead depuis commentaire)
  - Gestion des gouvernances
  - Intégration avec Kits existants (MiyuCRM, MiyuMail, MiyuSMS, MiyuWeb)

### 06_guides_implementation/
- **[Odoo Social Marketing - Guide Implémentation](./06_guides_implementation/Odoo%20Social%20Marketing%20-%20Guide%20Implementation.md)**
  - Architecture technique détaillée
  - Structure des crates Rust (miyusocial-account, stream, post, campaign, lead, visitor, insights, ui)
  - Schémas de données complets
  - API et contrats
  - Plan de développement par phases (MVP → Complet)
  - Bornage fonctionnel
  - Critères d'acceptation
  - Risques et mitigation

---

## Résumé Exécutif

### Fonctionnalités Principales Identifiées

1. **Comptes et streams**
   - Connexion OAuth (Facebook, Instagram, LinkedIn, Twitter, YouTube, Push)
   - Gestion des flux (streams) sur le tableau de bord Feed

2. **Publications**
   - Création et planification de posts (Send Now / Schedule later)
   - Publication sur plusieurs comptes simultanément
   - Images, campagne UTM, push notifications (titre, URL cible, icône, ciblage)

3. **Campagnes multi-canal**
   - Pipeline Kanban par étapes
   - Contenus : Send New Mailing, Send SMS, Send Social Post, Push Notification
   - Smart buttons Revenues, Quotations, Leads (intégration Sales, Invoicing, CRM)

4. **Création de leads depuis les commentaires**
   - Action « Create Lead » sur un commentaire
   - Convert Post to Lead (nouveau client, lien client existant, pas de lien)

5. **Visiteurs**
   - Liste des visiteurs web (Kanban, List, Graph)
   - Actions Email / SMS si contact en base

6. **Insights**
   - Lien « Insights » par stream vers les KPIs de la plateforme

### Architecture Miyukini Proposée

**8 Opérateurs :**
- SocialAccountOperator (comptes)
- SocialStreamOperator (flux)
- SocialPostOperator (publications)
- SocialCampaignOperator (campagnes)
- SocialLeadOperator (leads depuis commentaires)
- SocialVisitorOperator (visiteurs)
- SocialInsightsOperator (insights)
- SocialMarketingUI (interface)

**1 Équipe d'Opérateurs :** SocialMarketingService

**Niveaux de sécurité :** 1 (UI) à 2 (données comptes, posts, campagnes, visiteurs, leads)

**Intégration Cores :**
- StrongFather : Décisions (connexion, publication, création lead, envoi Email/SMS)
- KindMother : Persistance (WriteIntent comptes, streams, posts, campagnes, leads)
- Master Butler : Permissions
- WorrySentinel : Tokens, RGPD, consentement
- Ever Buddy : Cycle de vie post / compte
- BondingBrother : Traduction intentions → Opérateurs

**Correspondance Miyukini :** MiyuSocial / MiyukiniSocial (SocialMarketingService)

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

## Prochaines Étapes

1. **Valider les spécifications** : Revue avec équipe technique
2. **Démarrer l'implémentation** : Phase 1 (MVP) selon guide (comptes, streams, posts Send Now)
3. **Itérer** : Planification, push, campagnes, leads, visiteurs, insights selon phases

---

**Document** : Odoo Social Marketing — Index de Documentation  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : ✅ Analyse complète à 100% — référence pour implémentation Miyukini
