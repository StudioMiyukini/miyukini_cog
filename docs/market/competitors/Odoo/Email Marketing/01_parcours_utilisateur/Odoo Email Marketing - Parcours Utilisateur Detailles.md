# Odoo Email Marketing — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application Email Marketing d'Odoo, identifiant les personas, scénarios d'usage, étapes d'onboarding et points de friction pour guider l'implémentation d'un équivalent dans Miyukini.

**Source d'analyse :** Documentation utilisateur Odoo 18/19, interface et workflows identifiés.

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs
- Parcours d'onboarding
- Scénarios d'usage principaux (création campagne, listes, A/B test, métriques)
- Points de friction identifiés
- Recommandations pour Miyukini

**Hors scope :**
- Détails techniques d'implémentation
- Spécifications UI/UX détaillées (document dédié)

---

## 1. Personas et Rôles

### 1.1 Responsable Marketing (Marketing Manager)

**Profil :**
- Utilisateur principal de l'application
- Crée et planifie les campagnes email
- Définit les cibles (listes, filtres Lead/Opportunity, Event Registration)
- Analyse les métriques (ouvertures, clics, réponses)
- Gère les campagnes UTM et le lien avec CRM / Ventes

**Permissions :**
- Accès complet aux Mailings, Listes, Contacts
- Envoi et planification
- Configuration des campagnes (si activé)
- Consultation des traces et rapports

### 1.2 Rédacteur / Créatif (Content Editor)

**Profil :**
- Rédige les emails (sujet, corps, preview text)
- Utilise les modèles et blocs drag-and-drop
- Peu ou pas d’accès à la configuration des listes ou à l’envoi

**Permissions :**
- Création/édition du contenu des mailings (sujet, corps, pièces jointes)
- Accès en lecture aux listes et filtres pour choisir les destinataires
- Envoi de tests

### 1.3 Administrateur (Administrator)

**Profil :**
- Active les options (Mailing Campaigns, Blacklist, Serveur dédié, 24H Stat Reports)
- Gère les alias (Send From, Reply To)
- Gère la blacklist et les désabonnements
- Configure le serveur SMTP dédié (si utilisé)

**Permissions :**
- Accès Configuration → Settings
- Gestion blacklist, listes globales
- Paramètres d’envoi et quotas

### 1.4 Destinataire / Contact

**Profil :**
- Reçoit les emails
- Peut ouvrir, cliquer, répondre
- Peut se désabonner (lien dans l’email ou portail)
- Peut être blacklisté (option « Blacklist when unsubscribing »)

---

## 2. Parcours d'Onboarding

### 2.1 Première installation

**Étapes :**

1. **Installation de l’app Email Marketing**
   - Depuis les Apps : installer « Email Marketing » (module mass_mailing)
   - Dépendances installées : contacts, mail, html_builder, utm, link_tracker, social_media, web_tour, digest

2. **Premier accès au tableau de bord**
   - Clic sur l’icône Email Marketing → vue Mailings (liste par défaut)
   - Filtre par défaut « My Mailings » (mailings de l’utilisateur connecté)

3. **Configuration optionnelle (Settings)**
   - Email Marketing → Configuration → Settings
   - Activer « Mailing Campaigns » pour gérer des campagnes (Kanban Campagnes, métriques agrégées)
   - Activer « Blacklist Option when Unsubscribing »
   - Configurer « Dedicated Server » si serveur SMTP dédié
   - Activer « 24H Stat Mailing Reports » pour digest quotidien

4. **Création des premières listes**
   - Mailing Lists → Nouvelle liste
   - Import de contacts ou ajout manuel
   - Ou utilisation de filtres dynamiques (Contact, Lead/Opportunity, etc.) sans liste

**Durée estimée :** 30 min à 1 h pour une configuration de base

**Points de friction identifiés :**
- Nombre d’options (campagnes, blacklist, serveur) peut dérouter
- Compréhension différence « Liste » vs « Filtre dynamique » nécessaire

### 2.2 Première campagne

**Étapes :**

1. **Nouveau mailing**
   - Mailings → Nouveau
   - Saisie de l’objet (obligatoire)
   - Choix des destinataires : Liste(s) ou Filtre (Contact, Lead/Opportunity, Event Registration, etc.)

2. **Corps du message**
   - Onglet Mail Body : choix d’un thème (template) puis personnalisation par blocs
   - Ou modèle « Plain Text » et éditeur riche (/ commandes)
   - Onglet Settings : Preview text, Send From, Reply To, pièces jointes, Responsable (et Campagne si activé)

3. **Test**
   - Bouton « Test » → saisie d’adresses de test → « Send Test »
   - Vérification rendu et liens

4. **Envoi ou planification**
   - « Send » → confirmation « Ready to unleash emails? » → « Send to all »
   - Ou « Schedule » → choix date/heure → statut « In Queue »

5. **Suivi**
   - Après envoi : statut « Sent », colonnes Sent, Delivered (%), Opened (%), Clicked (%), Replied (%), ouverture de la fiche pour détails et traces

**Points de friction identifiés :**
- Limite quotidienne d’envoi : si dépassée, pas d’envoi automatique le lendemain ; il faut rouvrir et « Retry »
- Filtres dynamiques : nécessité de bien configurer le domaine pour éviter envoi à de mauvais contacts

---

## 3. Scénarios d'Usage Principaux

### 3.1 Campagne one-shot (newsletter)

- **Objectif :** Envoyer une newsletter à une ou plusieurs listes.
- **Parcours :** Nouveau → Objet → Sélection listes → Mail Body (template) → Settings (From, Reply To, Preview) → Test → Send ou Schedule.
- **Suivi :** Vue liste Mailings, colonnes Sent / Delivered / Opened / Clicked / Replied.

### 3.2 Ciblage dynamique (leads, événements)

- **Objectif :** Envoyer à tous les leads en statut « New » ou à tous les inscrits à un événement.
- **Parcours :** Nouveau → Objet → Recipients = Lead/Opportunity (ou Event Registration) → « Modify filter » → configuration du domaine (stages, dates, etc.) → vérification du nombre d’enregistrements → Mail Body → Test → Send/Schedule.
- **Suivi :** Idem ; possibilité de croiser avec UTM et CRM.

### 3.3 A/B test (objet ou contenu)

- **Objectif :** Tester deux versions (sujet ou corps) et envoyer la gagnante au reste.
- **Parcours :** Création version A → onglet A/B Tests → « Allow A/B Testing » → pourcentage (ex. 10 %) → Winner Selection (ex. Highest Open Rate) → Send Final On (date) → « Create an Alternative Version » → création version B → envoi ; à la date prévue, sélection du gagnant et envoi au reste.
- **Suivi :** Comparaison des métriques entre versions, puis métriques de la version gagnante sur l’ensemble.

### 3.4 Campagnes multi-mails (Mailing Campaigns activé)

- **Objectif :** Grouper plusieurs mailings sous une campagne (utm.campaign) et suivre Revenues, Quotations, Opportunities, Clicks.
- **Parcours :** Configuration → activer Mailing Campaigns → Campagnes → Nouvelle campagne (nom, responsable, tags) → depuis un mailing, onglet Settings → champ Campaign → associer la campagne → envoi des mailings ; consultation des smart buttons Revenues, Quotations, Opportunities, Clicks sur la fiche campagne.

### 3.5 Gestion des listes et désabonnements

- **Listes :** Mailing Lists → création, import contacts, ajout manuel ; consultation contact_count.
- **Désabonnement :** Lien dans l’email ou portail → formulaire désabonnement (choix listes, option blacklist) ; enregistrement mailing.subscription.optout et mise à jour listes / blacklist.
- **Blacklist :** Configuration → option « Blacklist when unsubscribing » ; consultation/édition manuelle de la blacklist si nécessaire.

### 3.6 Réactivation de leads perdus (Lost leads)

- **Objectif :** Cibler les leads marqués « Lost » pour une campagne de réactivation.
- **Parcours :** Nouveau → Recipients = Lead/Opportunity → Modify filter → Lost Reasons / Stage = Lost → Mail Body adapté → Send/Schedule.
- **Documentation Odoo :** « Lost leads reactivation email » (lien depuis doc Email Marketing).

---

## 4. Points de Friction Identifiés

- **Quota quotidien :** Limite globale d’emails ; les mailings en surplus ne partent pas automatiquement le lendemain (action « Retry » manuelle).
- **Complexité des filtres :** Équation de domaine (AND/OR, branches) puissante mais demande une bonne compréhension du modèle (Lead, Contact, Event, etc.).
- **A/B test :** Plusieurs écrans (pourcentage, critère gagnant, date finale, versions alternatives) ; risque d’oubli de « Send Final On » ou mauvaise interprétation des métriques.
- **Différence Liste vs Filtre :** Utilisateurs peu expérimentés peuvent confondre liste figée et filtre dynamique (nombre de destinataires qui peut changer).
- **Tracking :** Ouverture/clics dépendent du pixel et des liens trackés ; certains clients email bloquent les images → taux d’ouverture sous-estimé.
- **RGPD / consentement :** Odoo fournit désabonnement et blacklist ; la preuve de consentement et le délai de rétention des traces sont à gérer en politique interne.

---

## 5. Recommandations pour Miyukini

- **Personas clairs :** Distinguer Opérateur « Rédaction », « Envoi/Planification », « Listes/Contacts », « Analytiques » pour alignement avec les Opérateurs Miyukini.
- **Onboarding guidé :** Premier parcours : créer une liste → créer un mailing → envoyer un test → planifier un envoi → consulter les métriques.
- **Mandats explicites :** Envoi massif = acte gouverné (StrongFather + WorrySentinel) ; affichage clair des quotas et de la blacklist avant envoi.
- **Transparence conformité :** Préférences du contact (listes, blacklist) et preuve de consentement exposées de façon gouvernée (KindMother, audit).
- **Feedback utilisateur :** Messages clairs en cas de limite quotidienne (avec proposition « Retry » ou date de prochain envoi possible).

---

**Document :** Odoo Email Marketing — Parcours Utilisateur Détaillés  
**Version :** 1.0  
**Date :** 2026-02-01
