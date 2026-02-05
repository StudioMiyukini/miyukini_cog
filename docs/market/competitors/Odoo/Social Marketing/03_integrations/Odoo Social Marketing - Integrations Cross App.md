# Odoo Social Marketing — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Social Marketing** d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Documentation Odoo 18.0/19.0, structure des applications Marketing

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec les autres apps Odoo
- Flux de données inter-apps
- Mécanismes d'intégration (UTM, CRM, Website, Email, SMS)
- APIs externes (réseaux sociaux)
- Événements et données partagés

---

## 1. Dépendances Principales

### 1.1 Modules requis (typiques)

**Dépendances explicites (structure type `social` / `social_marketing`) :**
- `base` : Partenaires, sociétés, utilisateurs
- `mail` : Messagerie, activités, chatter
- `utm` : Campagnes UTM (utm.campaign, medium, source) pour le suivi
- `website` : Visiteurs, push notifications (website.visitor, paramètres push)
- `web` : Framework web et interface

**Dépendances optionnelles pour fonctionnalités complètes :**
- `crm` : Création de leads depuis les commentaires
- `sale` : Smart buttons Revenues, Quotations sur les campagnes
- `account` / `invoicing` : Revenus et facturation liés aux campagnes
- `mass_mailing` (Email Marketing) : Envois mailing dans les campagnes (Send New Mailing)
- `sms` (SMS Marketing) : Envois SMS dans les campagnes (Send SMS)

---

## 2. Intégrations Détaillées

### 2.1 UTM (Campagnes marketing)

**Flux :**
```
Social Marketing (posts, campagnes) → utm.campaign, utm.medium, utm.source
                                    → Suivi des revenus, leads, devis
```

**Mécanismes :**
- Les posts et campagnes sont rattachés à `utm.campaign` (campaign_id sur social.post, campagne sociale).
- Les liens diffusés peuvent inclure des paramètres UTM pour attribuer trafic et conversions.
- Consolidation des métriques par campagne (revenus, devis, leads) via les modèles Sale, CRM, Account.

**Recommandations Miyukini :**
- Service ou Opérateur dédié au suivi des sources (UTM) avec persistance KindMother et décision StrongFather pour attribution.

### 2.2 CRM (Leads depuis commentaires)

**Flux :**
```
Commentaire sur post → Action "Create Lead" → crm.lead
```

**Mécanismes :**
- Depuis la pop-up d’un post : menu contextuel sur un commentaire ‣ « Create Lead ».
- Fenêtre « Convert Post to Lead » : choix (nouveau client, lien client existant, pas de lien) puis création d’un enregistrement `crm.lead` avec contexte pré-rempli (commentaire, post, compte).
- Lien traçable post / commentaire → lead.

**Champs / données transmises :**
- Contexte du commentaire (texte, auteur si disponible, plateforme)
- Référence au post et au compte social
- Partenaire optionnel (si « Link to an existing customer »)

**Recommandations Miyukini :**
- Intégration BondingBrother entre Opérateur Social (commentaires) et Opérateur CRM (création lead) avec Mandat de Permission et WriteIntent pour la création de lead.

### 2.3 Sales & Invoicing (Métriques campagnes)

**Flux :**
```
Campagne (UTM) → sale.order, account.move → Smart buttons Revenues, Quotations
```

**Mécanismes :**
- Les campagnes sont liées aux commandes et factures via UTM (campaign_id, medium, source).
- Sur la fiche campagne : smart buttons **Revenues**, **Quotations** (et **Leads**) qui ouvrent des vues filtrées sur les enregistrements liés à cette campagne.
- Permet d’analyser le ROI par campagne.

**Recommandations Miyukini :**
- Équipe d’Opérateurs (Social + Sales + Invoicing) avec Contrat d’Équipe pour l’attribution des revenus et devis aux campagnes ; lecture seule des métriques côté Social.

### 2.4 Website (Visiteurs et push notifications)

**Flux :**
```
Website → website.visitor (traçage) → Social Marketing ‣ Visitors
Website ‣ Settings ‣ Enable Web Push Notifications → Option "Push" dans Post on
social.post (push) → Notification navigateur → Visiteur
```

**Mécanismes :**
- **Visiteurs** : Modèle `website.visitor` alimenté par le site ; liste et actions (Email, SMS) dans Social Marketing ‣ Visitors.
- **Push notifications** : Activation dans Website ‣ Configuration ‣ Settings ; ensuite les sites peuvent être sélectionnés dans « Post on » pour envoyer une push (titre, URL cible, icône, règles de ciblage, option Local Time).

**Données partagées :**
- website.visitor (session, partenaire si connu, site)
- Paramètres push (clés, etc.) stockés côté Website

**Recommandations Miyukini :**
- Opérateur Website expose visiteurs et paramètres push ; Opérateur Social Marketing consomme avec Mandat et respect RGPD (WorrySentinel, consentement).

### 2.5 Email Marketing (Mailing Campaigns)

**Flux :**
```
Campagne Social → Send New Mailing → mass_mailing.mailing → Envoi email
```

**Mécanismes :**
- Sur la template de campagne : bouton **Send New Mailing** (si Email Marketing installé et option « Mailing Campaigns » activée dans Configuration ‣ Settings).
- Création d’un mailing lié à la campagne ; envoi et statistiques visibles dans l’onglet Mailings de la campagne.
- Campagne = regroupement multi-canal (social, email, SMS, push).

**Recommandations Miyukini :**
- Équipe d’Opérateurs Marketing (Social + Email) ; Mandat pour ajouter un mailing à une campagne ; KindMother pour persistance des envois.

### 2.6 SMS Marketing

**Flux :**
```
Campagne Social → Send SMS → sms (modèle SMS) → Envoi SMS
```

**Mécanismes :**
- Bouton **Send SMS** sur la template de campagne (si app SMS Marketing installée).
- Création d’un envoi SMS lié à la campagne ; affiché dans l’onglet SMS de la campagne.

**Recommandations Miyukini :**
- Même principe que Email : Opérateur SMS consommé par l’Équipe Marketing ; gouvernance et permissions (StrongFather, Master Butler).

---

## 3. APIs Externes (Réseaux sociaux)

### 3.1 Plateformes supportées

- **Facebook** : Pages professionnelles ; OAuth, publication, récupération des commentaires et métriques (Insights).
- **Instagram** : Via API Facebook (compte et page Facebook liés) ; publication, commentaires, insights.
- **LinkedIn** : Comptes professionnels ; OAuth, publication, métriques.
- **Twitter / X** : Compte professionnel ; OAuth, publication (limite caractères), métriques.
- **YouTube** : Chaîne ; OAuth, publication, métriques.

### 3.2 Mécanismes

- **OAuth** : Connexion des comptes via redirection vers la plateforme ; tokens stockés (social.account ou équivalent).
- **Publication** : Appels API pour créer un post sur chaque plateforme à la date prévue (immédiat ou planifié).
- **Lecture** : Récupération des posts, commentaires, likes pour affichage dans le Feed et création de leads.
- **Insights** : Lien ou appels API vers les KPIs de la plateforme (affichés via « Insights » sur chaque stream).

### 3.3 Limitations documentées

- Nombre de pages par société (~40) limité par les APIs.
- Multi-company : toutes les sociétés doivent activer les pages en même temps sous peine d’erreurs de permission et déconnexion.
- Instagram : dépendance à Facebook (page + compte).

**Recommandations Miyukini :**
- Adapter / Opérateur dédié « Social Connector » pour OAuth et appels API ; stockage sécurisé des tokens (WorrySentinel, niveau de sécurité élevé) ; gestion des erreurs et quotas explicite dans l’UI.

---

## 4. Synthèse des Flux

| App / Système   | Flux entrant vers Social | Flux sortant depuis Social |
|-----------------|---------------------------|-----------------------------|
| **UTM**         | -                         | Campagnes, posts liés UTM  |
| **CRM**        | -                         | Création leads (commentaires) |
| **Sales**      | Données commandes (UTM)   | Smart buttons Revenues, Quotations |
| **Website**    | Visiteurs, config push    | Push notifications, liste Visitors |
| **Email**      | -                         | Mailings dans campagnes     |
| **SMS**        | -                         | SMS dans campagnes          |
| **APIs externes** | Tokens, posts, commentaires, insights | Publications, réponses commentaires |

---

## 5. Événements et Données Partagés

- **Publication réussie** : Mise à jour état `social.post` (published), `published_date` ; possible notification interne (mail/activité).
- **Échec publication** : État `failed` ; log ou message utilisateur pour cause (API, token, quota).
- **Création lead** : Création `crm.lead` + lien vers commentaire/post/compte ; activité ou notification CRM.
- **Campagne** : Ajout de contenu (post, mailing, SMS, push) met à jour les onglets et les smart buttons (revenus, devis, leads) selon les enregistrements UTM.

---

**Document** : Odoo Social Marketing — Intégrations Cross-App  
**Version** : 1.0  
**Date** : 2026-02-01
