# Odoo Email Marketing — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Email Marketing** d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour l’équivalent Email Marketing
- Contrats d’équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d’ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **MailingCampaignOperator** | Gestion des envois (mailings) et planification | Opérateur de Service |
| **MailingListOperator** | Gestion des listes de diffusion | Opérateur de Service |
| **MailingContactOperator** | Gestion des contacts email (listes, abonnements) | Opérateur de Service |
| **MailingBlacklistOperator** | Gestion de la liste noire (désabonnement global) | Opérateur de Service |
| **MailingTraceOperator** | Traces et métriques (envoi, ouverture, clic, réponse) | Opérateur de Service |
| **MailingContentOperator** | Rédaction et templates (corps, sujet, preview) | Opérateur de Service |
| **MailingFilterOperator** | Filtres dynamiques de ciblage (Contact, Lead, Event, etc.) | Opérateur de Service |
| **MailingCampaignUTMOperator** | Campagnes UTM et agrégation métriques | Opérateur de Service |
| **MailingUI** | Interface utilisateur Email Marketing | Opérateur d’Interface |

### 1.2 Équipe d’Opérateurs : EmailMarketingService

**Définition :**
> **EmailMarketingService est une Équipe d’Opérateurs qui collabore sous règles explicites pour délivrer le service d’emailing marketing (listes, campagnes, envoi, tracking, conformité).**

**Composition :**
- MailingCampaignOperator (niveau sécurité 2)
- MailingListOperator (niveau sécurité 2)
- MailingContactOperator (niveau sécurité 2)
- MailingBlacklistOperator (niveau sécurité 2)
- MailingTraceOperator (niveau sécurité 2)
- MailingContentOperator (niveau sécurité 1–2)
- MailingFilterOperator (niveau sécurité 2)
- MailingCampaignUTMOperator (niveau sécurité 2)
- MailingUI (niveau sécurité 1)

**Correspondance Miyukini :** MiyuEmailMarketing / MiyukiniEmailMarketing (EmailMarketingService)

---

## 2. Opérateurs détaillés

### 2.1 MailingCampaignOperator

**Rôle :** Gestion des envois (mailings) : création, planification, envoi, annulation, états (draft, in_queue, sending, done, cancel).

**Capacités :**
- Création / modification de mailings (sujet, destinataires, date planifiée)
- Planification (Schedule) et envoi immédiat (Send)
- Gestion A/B test (pourcentages, critère gagnant, date d’envoi final)
- Consultation des KPIs (sent, delivered, opened, clicked, replied, bounced)
- Annulation (draft / in_queue)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather :** Décision d’envoi ou planification (quota, blacklist, conformité)
- **KindMother :** Persistance des mailings et des états (WriteIntent)
- **Master Butler :** Permissions création / envoi / planification
- **WorrySentinel :** Quota quotidien, blacklist, niveau de données personnelles
- **Ever Buddy :** Cycle de vie (draft → sent, annulation)

**Contrat d’équipe :**
- Consomme : MailingListOperator, MailingContactOperator, MailingBlacklistOperator, MailingFilterOperator, MailingContentOperator, MailingTraceOperator, MailingCampaignUTMOperator
- Expose : `mailing.create`, `mailing.update`, `mailing.schedule`, `mailing.send`, `mailing.cancel`

**Mandat de Permission requis :**
- Envoi / planification : Mandat avec StrongFather (décision) + WorrySentinel (quota, blacklist) + KindMother (WriteIntent pour traces)
- Création / modification : Mandat avec KindMother (WriteIntent) + StrongFather (décision)

### 2.2 MailingListOperator

**Rôle :** Gestion des listes de diffusion (création, modification, suppression, association contacts).

**Capacités :**
- Création / modification / suppression de listes
- Ajout / retrait de contacts (via MailingContactOperator)
- Import de contacts (fichier ou API)
- Comptage des contacts par liste

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather :** Décision de création / modification / suppression
- **KindMother :** Persistance des listes et des abonnements (WriteIntent)
- **Master Butler :** Permissions sur les listes
- **WorrySentinel :** Données personnelles

**Contrat d’équipe :**
- Consomme : MailingContactOperator
- Expose : `mailing_list.create`, `mailing_list.update`, `mailing_list.delete`, `mailing_list.add_contacts`, `mailing_list.remove_contacts`

**Mandat de Permission requis :**
- Toute écriture : Mandat avec KindMother (WriteIntent) + StrongFather (décision)

### 2.3 MailingContactOperator

**Rôle :** Gestion des contacts email (fiches mailing, abonnements aux listes, lien optionnel avec Contact partenaire).

**Capacités :**
- Création / modification de contacts email (email, nom, listes)
- Gestion des abonnements (liste ↔ contact)
- Lien avec référentiel Contact (MiyuContacts ou équivalent) si applicable
- Exclusion blacklist avant envoi (lecture MailingBlacklistOperator)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather :** Décision de création / modification
- **KindMother :** Persistance des contacts et abonnements (WriteIntent)
- **Master Butler :** Permissions
- **WorrySentinel :** Données personnelles, conformité consentement

**Contrat d’équipe :**
- Consomme : MailingListOperator, MailingBlacklistOperator ; optionnel MiyuContacts
- Expose : `mailing_contact.create`, `mailing_contact.update`, `mailing_contact.subscribe`, `mailing_contact.unsubscribe`

**Mandat de Permission requis :**
- Écriture : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Désabonnement : Mandat avec KindMother (WriteIntent) + option blacklist (MailingBlacklistOperator)

### 2.4 MailingBlacklistOperator

**Rôle :** Gestion de la liste noire (emails exclus de tout envoi).

**Capacités :**
- Ajout / retrait d’adresses en blacklist
- Vérification avant envoi (liste des exclus)
- Option « blacklist when unsubscribing » (écriture depuis portail désabonnement)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather :** Décision d’ajout / retrait (ou politique explicite)
- **KindMother :** Persistance de la blacklist (WriteIntent)
- **WorrySentinel :** Conformité et audit

**Contrat d’équipe :**
- Consommé par : MailingCampaignOperator, MailingContactOperator
- Expose : `blacklist.check`, `blacklist.add`, `blacklist.remove`

**Mandat de Permission requis :**
- Ajout / retrait : Mandat avec KindMother (WriteIntent) + StrongFather (décision)

### 2.5 MailingTraceOperator

**Rôle :** Enregistrement des traces par destinataire (envoi, livraison, ouverture, clic, réponse, rebond) et agrégation des KPIs.

**Capacités :**
- Création / mise à jour des traces (état : outgoing, sent, open, reply, bounce, exception, cancel)
- Agrégation des compteurs sur le mailing (kpi_mail_sent, kpi_mail_opened, etc.)
- Fourniture des données pour rapports et digest (24H)
- Lien avec link_tracker pour les clics

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **KindMother :** Persistance des traces (WriteIntent)
- **Master Butler :** Permissions lecture (analytics)
- **WorrySentinel :** Données comportementales personnelles

**Contrat d’équipe :**
- Consommé par : MailingCampaignOperator, outils d’envoi (SMTP), link_tracker
- Expose : `trace.create`, `trace.update`, `trace.aggregate_kpis`, `trace.report`

**Mandat de Permission requis :**
- Écriture des traces : Mandat avec KindMother (WriteIntent) ; pas de décision StrongFather sur le fait de tracer (décision déjà prise à l’envoi)

### 2.6 MailingContentOperator

**Rôle :** Rédaction du contenu (sujet, corps HTML, preview text, pièces jointes) et gestion des templates.

**Capacités :**
- Création / modification du contenu (sujet, body_html, preview)
- Gestion des modèles (templates) réutilisables
- Pas d’envoi ni de décision ; uniquement contenu

**Niveau de sécurité :** 1–2 (Standard à Sensitive selon contenu)

**Gouvernance :**
- **StrongFather :** Validation du contenu avant envoi (optionnel, selon politique)
- **KindMother :** Persistance des brouillons et templates (WriteIntent)
- **Master Butler :** Permissions édition

**Contrat d’équipe :**
- Consommé par : MailingCampaignOperator
- Expose : `content.create`, `content.update`, `content.template.save`, `content.render_preview`

**Mandat de Permission requis :**
- Édition : Mandat avec KindMother (WriteIntent)

### 2.7 MailingFilterOperator

**Rôle :** Définition des filtres dynamiques de ciblage (Contact, Lead/Opportunity, Event Registration, Sales Order, Mailing Contact).

**Capacités :**
- Création / modification de filtres (modèle cible, domaine de recherche)
- Évaluation du nombre d’enregistrements correspondants (pour affichage avant envoi)
- Fourniture des destinataires (emails) pour un mailing selon le filtre
- Lecture seule sur les modèles cibles (CRM, Events, Sales, Contacts)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather :** Décision d’utilisation d’un filtre pour un envoi (avec MailingCampaignOperator)
- **Master Butler :** Permissions sur les filtres
- **WorrySentinel :** Vérification que les données ciblées sont autorisées

**Contrat d’équipe :**
- Consomme : MiyuContacts, MiyuCRM, MiyuEvents, MiyuSales (lecture seule)
- Consommé par : MailingCampaignOperator
- Expose : `filter.create`, `filter.update`, `filter.resolve_recipients`, `filter.count`

**Mandat de Permission requis :**
- Création / modification filtre : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Résolution destinataires : Mandat avec lecture des apps cibles (délégation par BondingBrother)

### 2.8 MailingCampaignUTMOperator

**Rôle :** Campagnes UTM (regroupement de mailings) et agrégation des métriques (Revenues, Quotations, Opportunities, Clicks).

**Capacités :**
- Création / modification de campagnes (nom, responsable, tags)
- Association mailings → campagne
- Agrégation des métriques depuis MailingTraceOperator et depuis CRM/Sales (Opportunities, Quotations, Revenues) si contrats en place
- Exposition des smart buttons (Revenues, Quotations, Opportunities, Clicks)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather :** Décision de création / modification campagne
- **KindMother :** Persistance des campagnes et associations (WriteIntent)
- **Ever Buddy :** Cycle de vie campagne

**Contrat d’équipe :**
- Consomme : MailingCampaignOperator (mailings), MailingTraceOperator (clics) ; optionnel MiyuCRM, MiyuSales (métriques)
- Expose : `campaign.create`, `campaign.update`, `campaign.attach_mailing`, `campaign.metrics`

**Mandat de Permission requis :**
- Écriture campagne : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Lecture métriques CRM/Sales : selon Contrat d’équipe avec Opérateurs CRM/Sales

### 2.9 MailingUI

**Rôle :** Interface utilisateur (tableau de bord Mailings, Campagnes, listes, contacts, formulaire email, paramètres, portail désabonnement).

**Capacités :**
- Vues Liste, Kanban, Calendar, Graph pour mailings et campagnes
- Formulaire email (sujet, destinataires, Mail Body, A/B Tests, Settings)
- Gestion des listes et contacts (CRUD, import)
- Configuration (Settings) : campagnes, blacklist, serveur dédié, 24H reports
- Portail désabonnement (Façade publique gouvernée)

**Niveau de sécurité :** 1 (Standard pour l’UI ; les données affichées restent soumises au niveau des Opérateurs)

**Gouvernance :**
- **Master Butler :** Permissions d’accès aux écrans
- **WorrySentinel :** Pas d’affichage de données au-delà du niveau autorisé
- **BondingBrother :** Médiation entre l’utilisateur et les Opérateurs (pas d’exécution directe)

**Contrat d’équipe :**
- Consomme tous les Opérateurs de l’équipe EmailMarketingService
- Expose : écrans et actions (Create, Send, Schedule, Test, etc.) selon Mandats

**Mandat de Permission requis :**
- Toute action déclenchée depuis l’UI nécessite le Mandat correspondant (envoi, édition liste, blacklist, etc.)

---

## 3. Contrat d’équipe EmailMarketingService

**Flux autorisés (résumé) :**
- MailingUI → MailingCampaignOperator (création, envoi, planification) ; MailingCampaignOperator → MailingListOperator, MailingContactOperator, MailingFilterOperator, MailingContentOperator, MailingBlacklistOperator, MailingTraceOperator, MailingCampaignUTMOperator
- MailingListOperator ↔ MailingContactOperator (abonnements)
- MailingCampaignOperator → MailingTraceOperator (écriture traces) ; MailingTraceOperator → agrégation KPIs → MailingCampaignOperator / MailingCampaignUTMOperator
- Portail désabonnement (Façade) → MailingContactOperator (unsubscribe) + optionnel MailingBlacklistOperator (add)

**Règles :**
- Pas de communication directe entre Opérateurs hors flux définis ; passage par BondingBrother et Mandats.
- Toute écriture (listes, contacts, blacklist, mailings, traces, campagnes) via WriteIntent vers KindMother.
- Envoi massif : Mandat explicite StrongFather + WorrySentinel (quota, blacklist).

---

## 4. Niveaux de sécurité

| Donnée | Niveau | Justification |
|--------|--------|----------------|
| Listes, contacts, abonnements | 2 (Sensitive) | Données personnelles (email) |
| Blacklist | 2 (Sensitive) | Données personnelles |
| Mailings (sujet, corps, destinataires) | 2 (Sensitive) | Données personnelles + contenu |
| Traces (ouverture, clic, réponse) | 2 (Sensitive) | Comportement personnels |
| Campagnes UTM | 2 (Sensitive) | Regroupement de données sensibles |
| Contenu public (templates génériques) | 1 (Standard) | Pas de données personnelles |
| UI (affichage) | 1 | Interface ; données sous-jacentes déjà protégées |

---

**Document :** Odoo Email Marketing — Spécifications Opérateurs Miyukini  
**Version :** 1.0  
**Date :** 2026-02-01
