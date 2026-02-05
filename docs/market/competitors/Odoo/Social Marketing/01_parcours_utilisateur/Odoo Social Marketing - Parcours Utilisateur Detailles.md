# Odoo Social Marketing — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application Social Marketing d'Odoo, identifiant les personas, scénarios d'usage, étapes d'onboarding et points de friction pour guider l'implémentation d'un équivalent dans Miyukini.

**Source d'analyse :** Documentation Odoo 18.0/19.0, interface utilisateur et workflows documentés.

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs
- Parcours d'onboarding (connexion des comptes, premier post)
- Scénarios d'usage principaux
- Points de friction identifiés
- Recommandations pour Miyukini

**Hors scope :**
- Détails techniques d'implémentation
- Spécifications UI/UX détaillées (document dédié)

---

## 1. Personas et Rôles

### 1.1 Responsable Marketing / Content Manager

**Profil :**
- Utilisateur principal de l'application
- Crée et planifie les publications sur les réseaux sociaux
- Gère les campagnes multi-canal (posts, email, SMS, push)
- Consulte les insights et métriques

**Permissions :**
- Connexion des comptes sociaux (streams)
- Création, édition, planification des posts
- Gestion des campagnes (création, étapes, ajout de contenus)
- Consultation des visiteurs et envoi d’email/SMS
- Création de leads depuis les commentaires

### 1.2 Community Manager

**Profil :**
- Répond aux commentaires et interagit avec la communauté
- Crée des leads à partir des commentaires
- Consulte le Feed et les performances par stream

**Permissions :**
- Lecture des posts et commentaires
- Réponse aux commentaires
- Création de leads depuis les commentaires
- Consultation des insights par compte

### 1.3 Responsable CRM / Sales

**Profil :**
- Utilise les leads générés depuis les commentaires
- Consulte les métriques campagnes (revenus, devis, leads)
- Ne gère pas nécessairement la création de posts

**Permissions :**
- Accès aux smart buttons campagnes (Revenues, Quotations, Leads)
- Traitement des leads créés depuis Social Marketing
- Lecture des campagnes et statistiques

### 1.4 Administrateur / Configuration

**Profil :**
- Configure les comptes sociaux et les streams
- Gère la page Social Media (Configuration ‣ Social Media) et Social Accounts / Social Streams
- Active les push notifications (Website ‣ Configuration ‣ Settings)

**Permissions :**
- Configuration ‣ Social Media, Social Accounts, Social Streams
- Gestion des connexions OAuth
- Paramétrage Website (push notifications)

---

## 2. Parcours d'Onboarding

### 2.1 Première utilisation

**Étapes :**

1. **Accès à l’application**
   - Ouvrir l’app Social Marketing depuis le menu Apps.

2. **Ajout d’un premier stream (compte)**
   - Cliquer sur « Add A Stream » (coin supérieur gauche).
   - Choisir « Link a new account for a business ».
   - Sélectionner la plateforme : Facebook, Instagram, LinkedIn, Twitter, YouTube.
   - Redirection vers la page d’autorisation du réseau ; accorder les permissions.
   - Retour sur le Feed ; une nouvelle colonne apparaît pour ce compte.

3. **Création du premier post**
   - Cliquer sur « New Post » (tableau de bord) ou Posts ‣ New.
   - Renseigner « Post on » (au moins un compte ou push notification).
   - Saisir le message, optionnellement attacher des images, lier une campagne.
   - Choisir « Send Now » ou « Schedule later » (date/heure).
   - Cliquer sur « Post » ou « Schedule ».

4. **Multi-company (si applicable)**
   - S’assurer d’ajouter toutes les pages pour toutes les sociétés en même temps pour éviter les erreurs de permission et la déconnexion.

### 2.2 Configuration avancée

- **Push notifications** : Website ‣ Configuration ‣ Settings ‣ Enable Web Push Notifications, renseigner les champs et enregistrer.
- **Campagnes** : Email Marketing ‣ Configuration ‣ Settings ‣ activer « Mailing Campaigns » pour « Send New Mailing » dans les campagnes.
- **SMS dans les campagnes** : Installer l’app SMS Marketing pour afficher « Send SMS » sur les templates de campagne.

---

## 3. Scénarios d'Usage Principaux

### 3.1 Publier immédiatement sur plusieurs réseaux

1. Social Marketing ‣ New Post.
2. Dans « Post on », cocher plusieurs comptes (ex. Facebook, Instagram, Twitter).
3. Saisir le message, joindre des images si besoin.
4. Choisir « Send Now ».
5. Cliquer sur « Post ».

**Résultat :** Publication simultanée sur les comptes sélectionnés.

### 3.2 Planifier une publication

1. New Post.
2. Renseigner « Post on », message, images, campagne éventuelle.
3. Choisir « Schedule later », renseigner la date/heure dans le calendrier, Apply.
4. Cliquer sur « Schedule ».

**Résultat :** Post en état « Scheduled », publié automatiquement à la date/heure choisie.

### 3.3 Créer un lead depuis un commentaire

1. Sur le tableau de bord, cliquer sur un post pour ouvrir la fenêtre du post.
2. Descendre jusqu’au commentaire concerné.
3. Cliquer sur les trois points à droite du commentaire ‣ « Create Lead ».
4. Dans « Convert Post to Lead » : choisir Create a new customer, Link to an existing customer, ou Do not link to a customer (et si besoin sélectionner le client).
5. Cliquer sur « Convert ».

**Résultat :** Ouverture d’un formulaire lead (CRM) pré-rempli avec le contexte du commentaire.

### 3.4 Lancer une campagne multi-canal

1. Social Marketing ‣ Campaigns ‣ Create (ou + dans une colonne du kanban).
2. Saisir le nom, le responsable, les tags, puis Add (ou Edit pour la fiche complète).
3. Sur la template de campagne : Send New Mailing, Send SMS, Send Social Post, Push Notification (selon apps et paramètres).
4. Créer chaque type de contenu ; ils apparaissent dans les onglets Mailings, SMS, Social Media, Push Notifications.
5. Consulter les smart buttons (Revenues, Quotations, Leads) pour le suivi.

### 3.5 Consulter les insights d’un compte

1. Sur le Feed, repérer le stream (colonne) concerné.
2. Cliquer sur le lien « Insights » en haut du stream.
3. Consultation des KPIs et statistiques de la plateforme (redirection ou intégration selon la plateforme).

### 3.6 Contacter un visiteur (Email / SMS)

1. Social Marketing ‣ Visitors.
2. Vue Kanban (défaut), List ou Graph.
3. Ouvrir un visiteur ayant des coordonnées en base.
4. Utiliser les options Email et/ou SMS pour envoyer un message.

---

## 4. Points de Friction Identifiés

1. **Multi-company et OAuth** : Si toutes les sociétés n’activent pas les mêmes pages ensemble, erreurs de permission et déconnexion ; message d’erreur à clarifier pour l’utilisateur.
2. **Limite du nombre de pages** : ~40 pages sous la même société peut atteindre les limites d’API ; pas de gestion explicite du quota dans l’UI.
3. **Instagram via Facebook** : Nécessité d’un compte et d’une page Facebook liés ; pas toujours évident pour un nouveau utilisateur.
4. **Push notifications** : Dépendance à Website et à l’activation explicite des paramètres ; un utilisateur peut ne pas voir l’option « Post on » push si mal configuré.
5. **Insights en multi-company** : Déconnexion d’une page entraîne la perte des insights ; il faut supprimer le stream et le reconnecter.
6. **Création de lead** : Workflow en plusieurs clics (post → commentaire → menu → Convert) ; pourrait être simplifié ou guidé.

---

## 5. Recommandations pour Miyukini

1. **Onboarding guidé** : Parcours pas à pas (connexion du premier compte, premier post, première campagne) avec messages clairs et liens vers la configuration (push, mailing campaigns).
2. **Gestion des erreurs OAuth** : Messages explicites (quota, multi-company, token révoqué) et actions proposées (réautoriser, contacter l’admin).
3. **Personas distincts** : Permissions et mandats adaptés (Content Manager, Community Manager, CRM/Sales, Admin) avec Contrat d’Équipe et Mandats de Permission.
4. **Création de lead** : Exposer une capacité dédiée « Create Lead from Comment » avec traçabilité (StrongFather, KindMother) et intégration CRM claire.
5. **Campagnes** : Modéliser les campagnes comme regroupement de contenus multi-canal avec métriques unifiées (revenus, leads, devis) et gouvernance par StrongFather/KindMother.
6. **Visiteurs** : Respect de la vie privée et du consentement (RGPD) pour push et emails/SMS ; niveaux de sécurité et audit (WorrySentinel).

---

**Document** : Odoo Social Marketing — Parcours Utilisateur Détaillés  
**Version** : 1.0  
**Date** : 2026-02-01
