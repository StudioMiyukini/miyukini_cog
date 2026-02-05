# Odoo SMS Marketing — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application SMS Marketing d'Odoo, identifiant les personas, scénarios d'usage, étapes d'onboarding et points de friction pour guider l'implémentation d'un équivalent dans Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, workflows et interface SMS Marketing.

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs
- Parcours d'onboarding
- Scénarios d'usage principaux (création SMS, campagnes, listes, blacklist, reporting)
- Points de friction identifiés
- Recommandations pour Miyukini

**Hors scope :**
- Détails techniques d'implémentation
- Spécifications UI/UX détaillées (document dédié)

---

## 1. Personas et Rôles

### 1.1 Responsable Marketing (Marketing Manager)

**Profil :**
- Utilisateur principal des campagnes SMS
- Crée et planifie les envois, gère les listes et campagnes
- Consulte les rapports et A/B tests
- Gère les blacklists et la conformité opt-out

**Permissions :**
- Création / édition / envoi de mailings SMS
- Gestion des listes de diffusion et des contacts
- Accès au reporting et aux campagnes
- Configuration opt-out et blacklist (selon droits)

### 1.2 Chargé(e) de Communication

**Profil :**
- Rédige et envoie des SMS promotionnels ou informatifs
- S’appuie sur les listes préexistantes
- Peut planifier des envois (événements, offres)

**Permissions :**
- Création et envoi de mailings (listes prédéfinies)
- Lecture des listes et contacts
- Pas nécessairement accès à la configuration globale ni aux campagnes avancées

### 1.3 Administrateur / Configurateur

**Profil :**
- Active les campagnes et A/B tests (paramètres Email Marketing / SMS Marketing)
- Configure Twilio si besoin
- Gère les modèles SMS techniques (mode développeur)
- Importe des blacklists (migration)

**Permissions :**
- Accès Configuration (Blacklisted Phone Numbers, Link Tracker, paramètres)
- Mode développeur pour SMS Templates
- Gestion des crédits / passerelles

### 1.4 Commercial / SDR

**Profil :**
- Envoie des SMS ciblés depuis la fiche Contact (icône SMS)
- Consulte l’historique des mailings dans le Chatter
- Peut déclencher des envois groupés (Contacts ‣ Liste ‣ Action ‣ Send SMS)

**Permissions :**
- Envoi depuis fiche contact ou action de masse sur contacts
- Lecture du Chatter et historique des communications

---

## 2. Parcours d'Onboarding

### 2.1 Première Utilisation de l’App SMS Marketing

**Étapes :**

1. **Activation de l’app** SMS Marketing (installée depuis les Apps).
2. **Vérification des crédits** : SMS Marketing s’appuie sur les crédits IAP Odoo (ou Twilio) ; sans crédits, les envois ne partent pas.
3. **Option campagnes** : Aller dans Email Marketing ‣ Configuration ‣ Paramètres, activer « Mailing Campaigns » (et « A/B Test ») si besoin.
4. **Première liste** : Mailing Lists ‣ Mailing Lists ‣ Créer (nom, option « Is Public »).
5. **Contacts** : Importer ou ajouter des contacts (Mailing List Contacts ou synchronisation avec Contacts).
6. **Premier mailing** : Dashboard SMS Marketing ‣ Créer ‣ Renseigner sujet, destinataires (ex. liste), contenu SMS, puis Send ou Schedule.

**Durée estimée :** 30 min à 1 h pour un premier envoi simple.

**Points de friction identifiés :**
- Dépendance aux crédits IAP (coût, gestion par l’entreprise).
- Paramètre « Mailing Campaigns » dans Email Marketing (pas dans SMS Marketing) peut prêter à confusion.
- Mode développeur nécessaire pour gérer les modèles SMS techniques (SMS Templates).

### 2.2 Mise en Place des Listes et de la Conformité

**Étapes :**

1. **Listes de diffusion** : Créer les listes (newsletter, prospects, clients, etc.), définir si publiques (opt-in/opt-out).
2. **Blacklist** : Vérifier Configuration ‣ Blacklisted Phone Numbers ; importer une blacklist existante si migration.
3. **Opt-out** : Sur les mailings, activer « Include opt-out link » dans l’onglet Settings.
4. **Page de gestion des abonnements** : S’assurer que les listes publiques sont accessibles aux destinataires (Subscription Management).

---

## 3. Scénarios d'Usage Principaux

### 3.1 Créer et Envoyer un SMS Simple (Liste de diffusion)

1. Ouvrir SMS Marketing ‣ Créer.
2. Saisir un **Subject**.
3. **Recipients** : choisir « Mailing List », sélectionner la liste.
4. **SMS Content** : rédiger le message (caractères et segments affichés).
5. **Settings** : activer « Include opt-out link » si besoin, définir le Responsable.
6. **Send** : choisir Send (immédiat), Schedule (date/heure), ou Test (numéros de test).
7. Suivi sur le dashboard (états Draft → In Queue → Sending → Sent).

### 3.2 Cibler des Contacts par Filtres

1. Créer un nouveau mailing.
2. **Recipients** : choisir « Contact ».
3. Affiner avec les filtres (ex. Country = France, Blacklist = not set) ou « Add Filter ».
4. Rédiger le contenu, configurer et envoyer comme en 3.1.

### 3.3 Planifier une Campagne avec A/B Test

1. Activer Mailing Campaigns (Email Marketing ‣ Configuration).
2. SMS Marketing ‣ Campaigns ‣ Créer une campagne (nom, responsable, tags).
3. Dans la campagne, « Send SMS » : créer un mailing SMS.
4. Onglet **A/B Test** : activer « Allow A/B Testing », définir pourcentage, Winner Selection (ex. Highest Click Rate), Send Final On.
5. Créer une ou plusieurs variantes (« Create an Alternate Version »).
6. Planifier l’envoi (Schedule) ; Odoo enverra les variantes au sous-ensemble puis la gagnante au reste à la date prévue.

### 3.4 Envoyer un SMS depuis la Fiche Contact

1. Aller dans Contacts, ouvrir un contact.
2. Cliquer sur l’icône **SMS** (à côté du numéro).
3. Rédiger et envoyer le message (envoi direct, hors mailing de masse).

### 3.5 Envoyer un SMS à Plusieurs Contacts (Action de masse)

1. Contacts ‣ passer en vue Liste.
2. Sélectionner les contacts concernés.
3. Action ‣ **Send SMS**.
4. Rédiger le message et confirmer l’envoi.

### 3.6 Gérer Listes et Blacklist

- **Listes** : Mailing Lists ‣ Mailing Lists (créer, modifier, consulter statistiques via smart buttons).
- **Contacts des listes** : Mailing Lists ‣ Mailing List Contacts (avec filtre « Exclude Blacklisted Phone » par défaut).
- **Blacklist** : SMS Marketing ‣ Configuration ‣ Blacklisted Phone Numbers (ajout manuel, Unblacklist, Import records).

### 3.7 Consulter les Analyses (Reporting)

1. SMS Marketing ‣ **Reporting**.
2. Choisir Filtres et Mesures, vues Graph / List / Cohort.
3. Analyser les performances des mailings (envois, clics, conversions selon mesures configurées).

---

## 4. Points de Friction Identifiés

- **Crédits IAP** : Envoi bloqué sans crédits ; pas d’alerte proactive toujours visible selon la version.
- **Paramètres dispersés** : Mailing Campaigns dans Email Marketing ; SMS Templates dans Paramètres ‣ Technique (mode développeur).
- **Twilio** : Configuration séparée pour certains pays ; double possibilité IAP / Twilio à expliquer.
- **Segments SMS** : Utilisateur peut sous-estimer le coût (multi-segments) si le nombre de segments n’est pas mis en avant.
- **Blacklist vs listes** : Compréhension « liste d’exclusion » vs « désabonnement global » à clarifier en formation.

---

## 5. Recommandations pour Miyukini

- **Personas** : Prévoir des rôles équivalents (Marketing, Communication, Admin, Commercial) avec permissions distinctes (Master Butler, Mandats).
- **Onboarding** : Checklist « Premier envoi » (crédits / passerelle, une liste, un mailing test, opt-out activé).
- **Parcours** : Séparer clairement « Envoi depuis contact » (conversationnel) et « Campagne / mailing de masse » (gouvernance, consentement, blacklist).
- **Conformité** : Mettre en avant opt-out, blacklist et preuves d’audit dans les parcours (WorrySentinel, traçabilité).
- **Reporting** : Exposer les métriques (envois, clics, désinscriptions) dans un parcours dédié, aligné sur les besoins Marketing et Légal.

---

**Document** : Odoo SMS Marketing — Parcours Utilisateur Détaillés  
**Version** : 1.0  
**Date** : 2026-02-01
