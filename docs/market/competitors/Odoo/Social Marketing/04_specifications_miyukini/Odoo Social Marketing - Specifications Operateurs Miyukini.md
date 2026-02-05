# Odoo Social Marketing — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Social Marketing** d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour l'équivalent Social Marketing
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d'Ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **SocialAccountOperator** | Gestion des comptes réseaux sociaux | Opérateur de Service |
| **SocialStreamOperator** | Gestion des flux (streams) par compte | Opérateur de Service |
| **SocialPostOperator** | Création, planification, publication des posts | Opérateur de Service |
| **SocialCampaignOperator** | Gestion des campagnes multi-canal | Opérateur de Service |
| **SocialLeadOperator** | Création de leads depuis les commentaires | Opérateur de Service |
| **SocialVisitorOperator** | Consultation et actions sur les visiteurs web | Opérateur de Service |
| **SocialInsightsOperator** | Agrégation des insights / KPIs par compte | Opérateur de Service |
| **SocialMarketingUI** | Interface utilisateur Social Marketing | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : SocialMarketingService

**Définition :**
> **SocialMarketingService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de gestion des comptes sociaux, publications, campagnes, création de leads depuis les commentaires et suivi des visiteurs.**

**Composition :**
- SocialAccountOperator (niveau sécurité 2)
- SocialStreamOperator (niveau sécurité 2)
- SocialPostOperator (niveau sécurité 2)
- SocialCampaignOperator (niveau sécurité 2)
- SocialLeadOperator (niveau sécurité 2)
- SocialVisitorOperator (niveau sécurité 2)
- SocialInsightsOperator (niveau sécurité 2, lecture / agrégation)
- SocialMarketingUI (niveau sécurité 1)

---

## 2. Opérateurs Détaillés

### 2.1 SocialAccountOperator

**Rôle :** Gestion des comptes réseaux sociaux (connexion OAuth, métadonnées, entreprise).

**Capacités :**
- Connexion / déconnexion de comptes (Facebook, Instagram, LinkedIn, Twitter, YouTube, Push)
- Lecture et mise à jour des métadonnées (name, handle, media_type, company_id)
- Exposition de la liste des comptes pour les streams et les posts

**Niveau de sécurité :** 2 (Sensitive) — tokens OAuth et données comptes

**Gouvernance :**
- **StrongFather** : Décision de connexion / déconnexion de compte
- **KindMother** : Persistance des comptes et tokens (WriteIntent, stockage sécurisé)
- **Master Butler** : Permissions (qui peut connecter / modifier des comptes)
- **WorrySentinel** : Sécurité des tokens, conformité RGPD
- **Ever Buddy** : Cycle de vie compte (révocation token)

**Contrat d'équipe :**
- Consomme : res.company, utilisateur (create_uid)
- Expose : `account.connect`, `account.disconnect`, `account.list`, `account.read`

**Mandat de Permission requis :**
- Connexion compte : Mandat avec KindMother (WriteIntent) + StrongFather (décision) + WorrySentinel (validation)
- Lecture liste : Mandat avec Master Butler (permission lecture)

### 2.2 SocialStreamOperator

**Rôle :** Gestion des flux (streams) affichés sur le tableau de bord ; liaison compte → flux.

**Capacités :**
- Création / modification / suppression de streams
- Association stream ↔ compte (account_id)
- Type de stream (posts, keyword, etc.)
- Exposition des streams pour le Feed et la configuration

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création / modification / suppression de stream
- **KindMother** : Persistance des streams (WriteIntent)
- **Master Butler** : Permissions
- **WorrySentinel** : Vérification niveau sécurité

**Contrat d'équipe :**
- Consomme : SocialAccountOperator (compte)
- Expose : `stream.create`, `stream.update`, `stream.delete`, `stream.list`

**Mandat de Permission requis :**
- Création / modification stream : Mandat avec KindMother (WriteIntent) + StrongFather (décision)

### 2.3 SocialPostOperator

**Rôle :** Création, planification et publication des posts (réseaux sociaux et push notifications).

**Capacités :**
- Création / édition de posts (message, images, comptes cibles, campagne, date)
- Planification (draft → scheduled → published)
- Publication immédiate ou à date/heure (Send Now / Schedule later)
- Options push (titre, URL cible, icône, Local Time, règles de ciblage)
- Gestion des états (draft, scheduled, published, failed)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de publication (immédiate ou planifiée)
- **KindMother** : Persistance des posts (WriteIntent)
- **Master Butler** : Permissions création / planification / publication
- **WorrySentinel** : Contrôle contenu et ciblage (optionnel)
- **Ever Buddy** : Cycle de vie post (états, échecs)

**Contrat d'équipe :**
- Consomme : SocialAccountOperator / SocialStreamOperator (comptes), SocialCampaignOperator (campagne), UTM (campaign_id), Website (push)
- Expose : `post.create`, `post.update`, `post.schedule`, `post.publish`, `post.list`

**Mandat de Permission requis :**
- Création / planification : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Publication (envoi effectif) : Mandat avec StrongFather (décision) + appel APIs externes (gouverné)

### 2.4 SocialCampaignOperator

**Rôle :** Gestion des campagnes multi-canal (regroupement posts, mailings, SMS, push).

**Capacités :**
- Création / modification des campagnes (nom, responsable, tags, étapes)
- Gestion des étapes (stages) du pipeline Kanban
- Liaison avec contenus : posts, mailings, SMS, push notifications
- Exposition des smart buttons (Revenues, Quotations, Leads) via intégration UTM / Sales / CRM

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création / modification campagne et ajout de contenus
- **KindMother** : Persistance des campagnes et liens contenus (WriteIntent)
- **Master Butler** : Permissions
- **WorrySentinel** : Vérification niveau sécurité

**Contrat d'équipe :**
- Consomme : SocialPostOperator (posts), MiyuMail / MiyuSMS (si présents), utm.campaign
- Expose : `campaign.create`, `campaign.update`, `campaign.add_content`, `campaign.stages`, `campaign.metrics`

**Mandat de Permission requis :**
- Création / modification campagne : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Ajout de contenu (post, mailing, SMS, push) : Mandat avec StrongFather (décision) + Opérateur concerné

### 2.5 SocialLeadOperator

**Rôle :** Création de leads (CRM) depuis les commentaires sur les posts.

**Capacités :**
- Action « Create Lead » à partir d’un commentaire (contexte post, compte, commentaire)
- Choix : nouveau client, lien client existant, pas de lien
- Création de l’enregistrement lead avec données pré-remplies

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création du lead
- **KindMother** : Persistance du lead (WriteIntent) — délégation au module CRM
- **Master Butler** : Permissions création lead depuis Social
- **WorrySentinel** : Vérification données (pas de fuite)

**Contrat d'équipe :**
- Consomme : SocialPostOperator (commentaires, post, compte), MiyuCRM ou équivalent (crm.lead)
- Expose : `lead.create_from_comment`

**Mandat de Permission requis :**
- Création lead depuis commentaire : Mandat avec StrongFather (décision) + KindMother (WriteIntent lead) + Master Butler (permission CRM)

### 2.6 SocialVisitorOperator

**Rôle :** Consultation des visiteurs web et actions (Email, SMS).

**Capacités :**
- Liste des visiteurs (Kanban, List, Graph)
- Lecture des données visiteurs (identification, partenaire si connu)
- Déclenchement d’envoi Email / SMS vers un visiteur (si contact en base)

**Niveau de sécurité :** 2 (Sensitive) — données de visite et contact

**Gouvernance :**
- **StrongFather** : Décision d’envoi (Email / SMS)
- **KindMother** : Pas d’écriture directe des visiteurs ; envoi via MiyuMail / MiyuSMS
- **Master Butler** : Permissions lecture visiteurs et envoi
- **WorrySentinel** : RGPD, consentement, opt-out

**Contrat d'équipe :**
- Consomme : Website (website.visitor), MiyuMail, MiyuSMS
- Expose : `visitor.list`, `visitor.read`, `visitor.send_email`, `visitor.send_sms`

**Mandat de Permission requis :**
- Lecture visiteurs : Mandat avec Master Butler (permission)
- Envoi Email / SMS : Mandat avec StrongFather (décision) + MiyuMail / MiyuSMS

### 2.7 SocialInsightsOperator

**Rôle :** Agrégation et exposition des insights / KPIs par compte (stream).

**Capacités :**
- Lecture des métriques par plateforme (via APIs ou liens externes)
- Exposition du lien « Insights » par stream
- Pas d’écriture ; lecture seule et agrégation

**Niveau de sécurité :** 2 (Sensitive) — données analytiques

**Gouvernance :**
- **Master Butler** : Permissions lecture insights
- **WorrySentinel** : Pas d’exposition de données personnelles non autorisées

**Contrat d'équipe :**
- Consomme : SocialAccountOperator / SocialStreamOperator, APIs plateformes (lecture)
- Expose : `insights.by_stream`, `insights.link`

**Mandat de Permission requis :**
- Lecture insights : Mandat avec Master Butler (permission)

### 2.8 SocialMarketingUI

**Rôle :** Interface utilisateur du service Social Marketing (Feed, Posts, Campagnes, Visiteurs, Configuration).

**Capacités :**
- Affichage du Feed (colonnes streams), Posts (Kanban, Calendar, List, Pivot), Campagnes (Kanban, List), Visiteurs (Kanban, List, Graph)
- Formulaires : création / édition post, création / édition campagne, configuration comptes et streams
- Actions : New Post, Add Stream, Create Lead from comment, Send Email / SMS visiteur
- Navigation : Social Marketing ‣ Feed, Posts, Campaigns, Visitors ; Configuration ‣ Social Media, Social Accounts, Social Streams

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **BondingBrother** : Traduction des intentions utilisateur vers les Opérateurs
- **Master Butler** : Permissions d’affichage et d’action
- **WorrySentinel** : Pas d’affichage de données hors périmètre

**Contrat d'équipe :**
- Consomme : Tous les Opérateurs Social (Account, Stream, Post, Campaign, Lead, Visitor, Insights)
- Expose : Vues et actions UI ; pas d’API métier directe (passage par BondingBrother)

**Mandat de Permission requis :**
- Accès UI : Mandat avec Master Butler (permissions par écran / action)

---

## 3. Contrat d'Équipe SocialMarketingService

**Opérateurs membres :**
- SocialAccountOperator, SocialStreamOperator, SocialPostOperator, SocialCampaignOperator, SocialLeadOperator, SocialVisitorOperator, SocialInsightsOperator, SocialMarketingUI

**Flux autorisés (résumé) :**
- SocialMarketingUI → BondingBrother → SocialPostOperator, SocialCampaignOperator, SocialAccountOperator, SocialStreamOperator, SocialLeadOperator, SocialVisitorOperator, SocialInsightsOperator
- SocialPostOperator → SocialAccountOperator, SocialStreamOperator, SocialCampaignOperator (lecture / liaison)
- SocialCampaignOperator → SocialPostOperator, UTM, MiyuMail, MiyuSMS (selon apps)
- SocialLeadOperator → SocialPostOperator (commentaires), MiyuCRM (création lead)
- SocialVisitorOperator → Website (visitors), MiyuMail, MiyuSMS

**Direction des flux :** Toujours via BondingBrother ; pas de communication directe entre Opérateurs métier sans Mandat.

**Types d’échanges :** Intentions utilisateur → Décisions (StrongFather) → WriteIntent (KindMother) ; lectures via Mandats.

**Niveau de sécurité :** 1 (UI) à 2 (données comptes, posts, campagnes, visiteurs, leads).

**Conditions préalables :** Comptes connectés pour publier ; Website configuré pour push ; CRM pour création de leads ; Email/SMS pour campagnes et visiteurs.

---

## 4. Mandats de Permission Typiques

| Action | Mandat |
|--------|--------|
| Connexion compte social | StrongFather + KindMother (WriteIntent compte/token) + WorrySentinel |
| Création / planification post | StrongFather + KindMother (WriteIntent post) |
| Publication post (envoi API) | StrongFather + gouvernance API |
| Création campagne | StrongFather + KindMother (WriteIntent campagne) |
| Ajout contenu à campagne | StrongFather + Opérateur contenu (Post, Mail, SMS) |
| Create Lead from comment | StrongFather + KindMother (lead) + Master Butler (CRM) |
| Envoi Email / SMS visiteur | StrongFather + MiyuMail / MiyuSMS |
| Lecture insights | Master Butler |

---

## 5. Correspondance Miyukini

**Service proposé :** **MiyuSocial** / **MiyukiniSocial** (SocialMarketingService)

**Équipe d'Opérateurs :** SocialMarketingService  
**Opérateurs :** SocialAccountOperator, SocialStreamOperator, SocialPostOperator, SocialCampaignOperator, SocialLeadOperator, SocialVisitorOperator, SocialInsightsOperator, SocialMarketingUI

**Intégrations Cores :** StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy, BondingBrother

**Intégrations optionnelles :** MiyuCRM (leads), MiyuSales / MiyuInvoice (métriques campagnes), MiyuWeb (visiteurs, push), MiyuMail (mailings), MiyuSMS (SMS)

---

**Document** : Odoo Social Marketing — Spécifications Opérateurs Miyukini  
**Version** : 1.0  
**Date** : 2026-02-01
