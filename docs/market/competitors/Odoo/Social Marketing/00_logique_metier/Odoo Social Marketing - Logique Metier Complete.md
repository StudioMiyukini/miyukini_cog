# Odoo Social Marketing — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Social Marketing** d'Odoo (versions 18.0 / 19.0), à partir de la documentation officielle et du code source. Il identifie les modèles de données, règles métier, workflows, calculs et mécanismes pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18.0/19.0, `https://github.com/odoo/odoo/tree/19.0/addons/social`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (social.account, social.stream, social.post, social.campaign, website.visitor)
- Règles métier et contraintes (comptes, streams, posts, campagnes)
- Workflows et transitions d'état (posts : brouillon, planifié, publié ; campagnes par étapes)
- Création de leads depuis les commentaires
- Insights et KPIs par plateforme
- Visiteurs web et push notifications

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `social.account` (Compte Réseau Social)

**Rôle :** Représente un **compte** de réseau social (page Facebook, compte Instagram, LinkedIn, Twitter/X, YouTube, etc.) connecté à Odoo.

**Champs clés :**
- `name` : Nom du compte (ex. nom de la page)
- `handle` / `short_name` : Identifiant court / handle (ex. @company)
- `media_type` : Type de média (facebook, instagram, linkedin, twitter, youtube, push_notification)
- `company_id` : Many2one vers `res.company` (entreprise propriétaire)
- `create_uid` : Utilisateur ayant créé le compte
- `website_id` : Many2one vers `website` (pour push notifications / site associé)

**Règles métier :**
- Seuls les **comptes professionnels / pages** peuvent être ajoutés (pas de profils personnels).
- Chaque compte est associé à une entreprise (multi-company).
- Instagram est connecté via l’API Facebook (compte Facebook lié requis).
- Limitation documentée : un grand nombre de pages (~40) sous la même société peut atteindre les limites d’API.

### 1.2 Modèle `social.stream` (Flux / Stream)

**Rôle :** Représente un **flux** affiché sur le tableau de bord Social Marketing (colonnes du Feed). Un stream est lié à un compte social et affiche les publications et interactions.

**Champs clés :**
- `name` / `title` : Titre du flux
- `account_id` : Many2one vers `social.account`
- `stream_type` : Type de flux (posts, keyword, etc.)
- `company_id` : Many2one vers `res.company`
- `create_uid` : Créateur du stream

**Règles métier :**
- Un stream est créé après autorisation OAuth sur la plateforme (Facebook, Instagram, etc.).
- Le tableau de bord affiche une colonne par stream.
- Les streams sont configurables via Configuration ‣ Social Streams.

### 1.3 Modèle `social.post` (Publication)

**Rôle :** Représente une **publication** sur les réseaux sociaux (ou une push notification).

**Champs clés :**
- `message` : Contenu texte du post
- `account_ids` / `stream_ids` : Comptes ou streams sur lesquels publier (Post on)
- `company_id` : Many2one vers `res.company` (multi-company)
- `state` : État (draft, scheduled, published, failed)
- `scheduled_date` : Date/heure de publication planifiée (si state = scheduled)
- `published_date` : Date/heure de publication effective
- `campaign_id` : Many2one vers `utm.campaign` ou modèle de campagne social
- `image_ids` : Pièces jointes images (Attach Images)
- `website_visitor_ids` : Cible pour push notification (optionnel)
- `push_notification_title` : Titre de la push notification
- `push_notification_target_url` : URL cible de la push
- `push_notification_icon` : Icône de la push
- `push_notification_local_time` : Boolean (envoyer à l’heure locale du visiteur)
- `push_notification_match_domain` : Règles de ciblage (Match all records)

**Types de publication :**
- Publication sur un ou plusieurs comptes (Facebook, Instagram, LinkedIn, Twitter, YouTube).
- Push notification vers les visiteurs du site (nécessite Website + Enable Web Push Notifications).

**États (state) :**
- `draft` : Brouillon
- `scheduled` : Planifié (scheduled_date renseignée)
- `published` : Publié
- `failed` : Échec de publication

**Règles métier :**
- Au moins **un** canal (Post on) doit être sélectionné.
- Pour Twitter : limite de caractères et compteur affiché.
- Push notifications : option « Local Time » pour envoyer à l’heure locale du visiteur.
- Les posts peuvent être rattachés à une **campagne** (campaign_id).

### 1.4 Modèle Campagne (social.campaign / utm.campaign)

**Rôle :** Représente une **campagne** de marketing social (regroupement de contenus et canaux : mailing, SMS, posts sociaux, push).

**Champs clés :**
- `name` : Nom de la campagne
- `user_id` : Responsable (Many2one vers `res.users`)
- `tag_ids` : Tags (Many2many)
- `stage_id` : Étape du pipeline (kanban)
- `mailing_ids` : Envois email (si Email Marketing activé)
- `sms_ids` : Envois SMS (si SMS Marketing installé)
- `post_ids` : Publications sociales liées
- `push_notification_ids` : Push notifications liées
- Smart buttons : Revenues, Quotations, Leads, etc. (intégration Sales, CRM)

**Règles métier :**
- Les campagnes ont des **étapes** (stages) configurables (kanban).
- Plusieurs formes de contenu peuvent être ajoutées : Send New Mailing, Send SMS, Send Social Post, Push Notification.
- Intégration avec Sales, Invoicing, CRM, Website pour métriques (revenus, devis, leads).

### 1.5 Visiteurs (website.visitor)

**Rôle :** Représente un **visiteur** du site web (traçage pour push notifications, email, SMS).

**Champs clés :**
- Identifiant visiteur (session / cookie)
- `partner_id` : Many2one vers `res.partner` (si contact connu)
- `website_id` : Site concerné
- Données de visite (pages, durée, etc.)

**Règles métier :**
- Les visiteurs sont listés dans Social Marketing ‣ Visitors.
- Vues : Kanban (défaut), List, Graph.
- Actions : Envoyer Email, Envoyer SMS (si contact présent en base).

### 1.6 Commentaires et leads

**Rôle :** Les commentaires sur les posts sont récupérés via les APIs des plateformes. Une action métier permet de **créer un lead (CRM)** depuis un commentaire.

**Règles métier :**
- Depuis un post : menu contextuel sur un commentaire ‣ « Create Lead ».
- Fenêtre « Convert Post to Lead » : Create a new customer, Link to an existing customer, Do not link to a customer.
- Création d’un enregistrement lead (CRM) avec contexte pré-rempli depuis le commentaire.

---

## 2. Workflows et États

### 2.1 Workflow des publications (social.post)

```
draft → scheduled → published
   ↓         ↓           ↓
   └─────────┴───→ failed (en cas d’erreur API)
```

- **draft** : Création, édition du message, choix des comptes, images, campagne, option « When » = Send Now ou Schedule later.
- **scheduled** : Lorsque « Schedule later » est choisi et une date/heure renseignée ; le bouton d’action devient « Schedule ».
- **published** : Envoi immédiat (Send Now) ou à la date planifiée ; mise à jour de `published_date`.
- **failed** : En cas d’échec de l’API (réseau, quota, token révoqué, etc.).

### 2.2 Workflow des campagnes

- Campagnes gérées en **pipeline Kanban** par étapes (stages).
- Actions sur une campagne : création, édition, ajout de contenus (Mailing, SMS, Social Post, Push Notification).
- Chaque type de contenu apparaît dans un onglet dédié sur la fiche campagne (Mailings, SMS, Social Media, Push Notifications).
- Smart buttons pour analyser Revenues, Quotations, Leads, etc.

---

## 3. Règles Métier Transverses

### 3.1 Comptes et autorisation

- **OAuth** : Connexion des comptes via les pages d’autorisation des plateformes (Facebook, Instagram, LinkedIn, Twitter, YouTube).
- **Multi-company** : En multi-société, si toutes les sociétés n’activent pas les mêmes pages en même temps, des erreurs de permission peuvent survenir ; toutes les pages doivent être ajoutées pour toutes les sociétés concernées pour éviter la déconnexion.
- **Instagram** : Nécessite un compte Facebook et une page Facebook liée (même API).

### 3.2 Push notifications

- Nécessite **Website** avec « Enable Web Push Notifications » activé (Configuration ‣ Settings).
- Champs optionnels : Notification Title, Target URL, Icon Image, Local Time, règles « Match all records » pour cibler des segments de visiteurs.

### 3.3 Insights

- Chaque stream (compte) peut exposer un lien **Insights** vers les KPIs de la plateforme (Facebook, etc.).
- En multi-company, si une page est déconnectée, les insights de cette page sont perdus ; il faut supprimer le stream et le reconnecter.

### 3.4 UTM et campagnes

- Les campagnes sont liées au système **UTM** (utm.campaign) pour le suivi des sources (campaign, medium, source).
- Permet le suivi des revenus et leads par campagne (intégration CRM, Sales, Invoicing).

---

## 4. Points d'Attention pour Miyukini

1. **Séparation des responsabilités** : Comptes (social.account), Flux (social.stream), Publications (social.post), Campagnes, Visiteurs — chaque entité peut correspondre à un Opérateur ou à des capacités bien délimitées.
2. **Gouvernance** : Décisions de publication (StrongFather), persistance des posts et comptes (KindMother), permissions (Master Butler), conformité et risques (WorrySentinel).
3. **Intégrations** : CRM (leads depuis commentaires), Sales/Invoicing (revenus par campagne), Website (visiteurs, push), Email Marketing (mailings dans campagnes), SMS Marketing (SMS dans campagnes).
4. **Limitations documentées** : Nombre de pages par société, multi-company, déconnexion OAuth — à prendre en compte pour le design des contrats et des messages d’erreur.
5. **Données sensibles** : Tokens OAuth, données de visiteurs — niveau de sécurité et stockage conforme (WorrySentinel, KindMother).

---

**Document** : Odoo Social Marketing — Logique Métier Complète  
**Version** : 1.0  
**Date** : 2026-02-01
