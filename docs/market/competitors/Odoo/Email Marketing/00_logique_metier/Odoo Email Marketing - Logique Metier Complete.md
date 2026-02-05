# Odoo Email Marketing — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Email Marketing** d'Odoo (version 19.0), extraite du code source GitHub. Il identifie les modèles de données, règles métier, workflows, calculs et mécanismes de gouvernance pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** `https://github.com/odoo/odoo/tree/19.0/addons/mass_mailing`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (mailing.mailing, mailing.list, mailing.contact, mailing.trace)
- Règles métier et contraintes (blacklist, désabonnement, A/B tests)
- Workflows et transitions d'état (Draft, In Queue, Sending, Sent)
- Calculs et métriques (taux d'ouverture, clics, réponses, livraison)
- Système de listes et abonnements
- Intégration UTM et suivi des liens

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `mailing.mailing` (Campagne / Envoi massif)

**Rôle :** Représente une campagne d'emailing ou un envoi massif unique.

**Champs clés :**
- `subject` : Objet de l'email (obligatoire)
- `email_from` : Adresse expéditeur (alias)
- `reply_to` : Adresse de réponse
- `body_html` : Contenu HTML du message (builder drag-and-drop)
- `preview` : Texte d'aperçu (affiché à côté de l'objet dans la boîte de réception)
- `state` : État (draft, in_queue, sending, done, cancel)
- `sent_date` : Date d'envoi effectif
- `schedule_date` : Date d'envoi planifié
- `mailing_type` : Type (mail pour email)
- `contact_list_ids` : Listes de contacts cibles (Many2many)
- `mailing_filter_id` : Filtre dynamique (Contact, Lead/Opportunity, Event Registration, etc.)
- `campaign_id` : Campagne UTM associée (si fonctionnalité activée)
- `user_id` : Responsable
- `ab_testing_enabled` : A/B testing activé
- `ab_testing_percentage` : Pourcentage de destinataires pour cette version (A/B)
- `ab_testing_winner_selection` : Critère de sélection du gagnant (Manual, Highest Open Rate, Highest Click Rate, etc.)
- `ab_testing_schedule_date` : Date de sélection du gagnant et envoi final
- `kpi_mail_sent` : Nombre d'emails envoyés
- `kpi_mail_delivered` : Nombre livrés
- `kpi_mail_opened` : Nombre ouverts
- `kpi_mail_clicked` : Nombre de clics
- `kpi_mail_replied` : Nombre de réponses
- `kpi_mail_bounced` : Nombre de rebonds

**États (state) :**
- `draft` : Brouillon (modifiable)
- `in_queue` : Planifié, en attente d'envoi
- `sending` : En cours d'envoi
- `done` : Envoyé
- `cancel` : Annulé

**Règles métier :**
- L'objet (subject) est obligatoire pour envoyer
- Les destinataires sont soit des listes (mailing_list_ids), soit un filtre dynamique (mailing_filter_id)
- En A/B test : un seul envoi par destinataire ; sélection du gagnant selon critère configuré ; envoi de la version gagnante au reste des destinataires à la date prévue
- Limite quotidienne d'envoi (paramètre système) ; les envois non effectués nécessitent un clic « Retry »

### 1.2 Modèle `mailing.list` (Liste de diffusion)

**Rôle :** Liste de contacts cibles pour les campagnes.

**Champs clés :**
- `name` : Nom de la liste
- `contact_ids` : Contacts abonnés (One2many via mailing.contact ou relation)
- `subscription_ids` : Abonnements (mailing.subscription)
- `contact_count` : Nombre de contacts (calculé)
- `company_id` : Entreprise (multi-company)

**Règles métier :**
- Une liste peut être utilisée par plusieurs campagnes
- Les contacts sont ajoutés via import, formulaire d'inscription, ou manuellement
- Gestion des désabonnements : contact retiré de la liste ou blacklisté

### 1.3 Modèle `mailing.contact` (Contact Email Marketing)

**Rôle :** Contact destinataire (peut être lié à res.partner ou autonome).

**Champs clés :**
- `email` : Adresse email (obligatoire)
- `name` : Nom affiché
- `list_ids` : Listes auxquelles le contact est abonné (Many2many)
- `subscription_ids` : Abonnements (mailing.subscription)
- `partner_id` : Lien optionnel vers res.partner (Contacts)
- `company_id` : Entreprise
- `unsubscription_ids` : Historique désabonnement (mailing.subscription.optout)

**Règles métier :**
- Un contact peut appartenir à plusieurs listes
- Si blacklisté (mail.blacklist), il n'est plus inclus dans les envois
- Les contacts « Mailing Contact » uniquement n'ont pas de fiche Contact (Contacts app)

### 1.4 Modèle `mailing.trace` (Trace / Statistiques par destinataire)

**Rôle :** Trace d'envoi et statistiques par email envoyé (un enregistrement par destinataire × campagne).

**Champs clés :**
- `mailing_id` : Campagne (mailing.mailing)
- `email` : Adresse du destinataire
- `res_id` : ID de l'enregistrement source (contact, lead, etc.)
- `model` : Modèle source (mailing.contact, crm.lead, etc.)
- `state` : État (outgoing, sent, open, reply, bounce, exception, cancel)
- `sent_datetime` : Date/heure d'envoi
- `open_datetime` : Date/heure d'ouverture (si trackée)
- `links_click_ids` : Clics sur les liens (link_tracker)
- `failure_type` : Type d'échec (si bounce/exception)

**États (state) :**
- `outgoing` : En attente d'envoi
- `sent` : Envoyé
- `open` : Ouvert
- `reply` : Réponse reçue
- `bounce` : Rebond
- `exception` : Erreur
- `cancel` : Annulé

**Règles métier :**
- Une trace par couple (mailing_id, email) pour un envoi donné
- Les métriques agrégées (kpi_*) sur mailing.mailing sont dérivées des traces
- Tracking d'ouverture via pixel ou lien tracké ; tracking de clics via link_tracker

### 1.5 Modèle `mail.blacklist` (Liste noire)

**Rôle :** Adresses email exclues des envois (désabonnement global ou blacklist).

**Champs clés :**
- `email` : Adresse blacklistée
- `active` : Actif (exclusion effective)
- `reason` : Raison (optionnel)

**Règles métier :**
- Tout email présent en blacklist est exclu des destinataires avant envoi
- Option « Blacklist Option when Unsubscribing » : lors du désabonnement, l'utilisateur peut être ajouté à la blacklist
- Géré au niveau global (res.company ou config)

### 1.6 Modèle `mailing.filter` (Filtre de destinataires)

**Rôle :** Définition dynamique des destinataires (domaine Odoo) pour Contact, Lead/Opportunity, Event Registration, Sales Order, Mailing Contact.

**Champs clés :**
- `name` : Nom du filtre
- `mailing_domain` : Domaine de recherche (ex. leads en statut « New », contacts par pays)
- `model_id` : Modèle cible (res.partner, crm.lead, event.registration, sale.order, mailing.contact)
- `mailing_id` : Campagne associée (optionnel)

**Règles métier :**
- Permet de cibler sans créer de liste figée (ex. tous les leads créés ce mois-ci, tous les inscrits à un événement)
- Le nombre d'enregistrements correspondant au filtre est affiché sous le filtre (équation)
- Filtres composables (AND/OR, branches) dans l’UI

### 1.7 Modèles `mailing.subscription` et `mailing.subscription.optout`

**mailing.subscription :** Lien contact ↔ liste (abonnement actif).

**mailing.subscription.optout :** Raison de désabonnement (feedback) et liste concernée ; option blacklist.

**Règles métier :**
- Désabonnement = retrait de la liste et/ou ajout à la blacklist
- Gestion des préférences (portal) : formulaire de désabonnement avec choix de listes et option blacklist

### 1.8 Modèles UTM et `link_tracker`

- **utm.campaign** : Campagne marketing (nom, responsable). Si « Mailing Campaigns » activé, les mailings sont rattachés à une campagne ; métriques agrégées (Revenues, Quotations, Opportunities, Clicks) au niveau campagne.
- **link_tracker** (module link_tracker) : Suivi des clics sur les liens dans les emails (redirection via serveur Odoo + enregistrement clic).
- **mail_render_mixin** : Rendu du corps HTML avec variables (nom, prénom, etc.) et tracking (pixel, liens).

---

## 2. Workflows et Transitions d'État

### 2.1 Workflow principal `mailing.mailing`

```
draft → in_queue (Schedule) → sending (Cron/Worker) → done
  ↓           ↓                    ↓
cancel      cancel               cancel
```

- **Draft → In Queue :** Utilisateur clique « Schedule » et choisit date/heure.
- **In Queue → Sending :** Cron ou worker traite les mailings dont `schedule_date <= now`.
- **Sending → Done :** Tous les emails ont été traités (envoi SMTP ou mise en file).
- **Cancel :** Depuis draft ou in_queue (selon implémentation).

### 2.2 Workflow A/B Test

1. Création de 2 (ou plus) versions du mailing (même campagne, `ab_testing_enabled=True`).
2. Chaque version a un `ab_testing_percentage` (ex. 10 % / 90 %).
3. Envoi aux pourcentages configurés (un seul email par destinataire).
4. À la date `ab_testing_schedule_date`, sélection du gagnant selon `ab_testing_winner_selection`.
5. Envoi de la version gagnante au reste des destinataires (ceux qui n’ont pas encore reçu).

### 2.3 Gestion des erreurs et limites

- **Quota quotidien :** Si limite atteinte, les mailings restent en file ; « Retry » le lendemain ou après relève de la limite.
- **Bounce / Exception :** Mise à jour de la trace (`mailing.trace`) ; possibilité de blacklister automatiquement les rebonds (config).

---

## 3. Calculs et Métriques

### 3.1 KPIs sur `mailing.mailing`

- **Sent :** Nombre d’emails effectivement envoyés (comptés en fin d’envoi).
- **Delivered (%) :** (kpi_mail_delivered / kpi_mail_sent) × 100 (estimation si pas de feedback SMTP détaillé).
- **Opened (%) :** (kpi_mail_opened / kpi_mail_sent) × 100 (tracking pixel ou lien).
- **Clicked (%) :** (kpi_mail_clicked / kpi_mail_sent) × 100 (link_tracker).
- **Replied (%) :** (kpi_mail_replied / kpi_mail_sent) × 100 (réponse détectée via mail thread).
- **Bounced :** Comptage des traces en état bounce.

### 3.2 Agrégation au niveau Campagne (utm.campaign)

Si « Mailing Campaigns » activé : sommes ou comptages des métriques des mailings de la campagne (Revenues, Quotations, Opportunities, Clicks) pour tableaux de bord.

### 3.3 Rapports 24H

Option « 24H Stat Mailing Reports » : envoi d’un digest (module digest) avec performances des mailings envoyés la veille.

---

## 4. Règles Métier Critiques

1. **Destinataires :** Toujours exclure les emails présents dans `mail.blacklist` avant envoi.
2. **Objet obligatoire :** Un mailing ne peut pas être envoyé sans `subject`.
3. **Destinataires obligatoires :** Soit au moins une liste (mailing_list_ids), soit un filtre (mailing_filter_id) avec au moins un enregistrement.
4. **Un envoi par destinataire en A/B :** Chaque email reçu une seule fois pendant le test A/B.
5. **Serveur dédié (optionnel) :** Paramètre pour utiliser un serveur SMTP dédié aux envois marketing (Configuration → Settings).
6. **Preview text :** Optionnel ; améliore le taux d’ouverture en inbox.
7. **Template et builder :** Contenu HTML via thèmes/snippets (drag-and-drop) ; personnalisation par champs (contact, partenaire, etc.) via mail_render_mixin.

---

## 5. Points d'Attention pour Miyukini

- **Sécurité et conformité :** RGPD / consentement, désabonnement en 1 clic, preuve de consentement (audit).
- **Niveau de sécurité :** Données personnelles (email, comportement) → niveau Sensitive (2) ou Critical (3) selon politique.
- **Opérateurs distincts :** Séparation claire entre gestion des listes, rédaction/envoi, tracking, blacklist, campagnes UTM.
- **WriteIntent :** Toute création/modification de liste, contact, blacklist, envoi planifié doit passer par KindMother.
- **Mandats :** Envoi massif = acte fort ; Mandat explicite avec StrongFather (décision) et WorrySentinel (quota, blacklist, conformité).
- **Pas d’exécution par les Cores :** Les Cores décident et gouvernent ; l’envoi SMTP et le tracking sont exécutés par des Opérateurs/Outils.

---

**Document :** Odoo Email Marketing — Logique Métier Complète  
**Version :** 1.0  
**Date :** 2026-02-01
