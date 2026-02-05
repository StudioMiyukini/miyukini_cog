# Odoo Email Marketing — Index de Documentation

## Contexte

Ce dossier contient l'**analyse complète** de l'application **Email Marketing** d'Odoo, réalisée selon la méthodologie standardisée. L'analyse couvre la logique métier, les parcours utilisateur, l'UI/UX, les intégrations, les spécifications Opérateurs Miyukini, l'intégration COG et les guides d'implémentation.

**Date d'analyse :** 2026-02-01  
**Source :** Code source GitHub Odoo 19.0 (addons/mass_mailing), documentation Odoo 18/19

---

## Structure de Documentation

### 00_logique_metier/
- **[Odoo Email Marketing - Logique Métier Complète](./00_logique_metier/Odoo%20Email%20Marketing%20-%20Logique%20Metier%20Complete.md)**
  - Modèles de données (mailing.mailing, mailing.list, mailing.contact, mailing.trace, mail.blacklist, mailing.filter, mailing.subscription, utm.campaign)
  - Règles métier et contraintes (blacklist, désabonnement, A/B tests)
  - Workflows et transitions d'état (Draft, In Queue, Sending, Sent)
  - Calculs et métriques (KPIs, agrégation campagnes)
  - Points d'attention pour Miyukini

### 01_parcours_utilisateur/
- **[Odoo Email Marketing - Parcours Utilisateur Détaillés](./01_parcours_utilisateur/Odoo%20Email%20Marketing%20-%20Parcours%20Utilisateur%20Detailles.md)**
  - Personas et rôles (Responsable Marketing, Rédacteur, Administrateur, Destinataire)
  - Parcours d'onboarding et première campagne
  - Scénarios d'usage (newsletter, ciblage dynamique, A/B test, campagnes UTM, listes et désabonnements, réactivation leads)
  - Points de friction identifiés
  - Recommandations pour Miyukini

### 02_ui_ux/
- **[Odoo Email Marketing - Analyse UI/UX](./02_ui_ux/Odoo%20Email%20Marketing%20-%20Analyse%20UI%20UX.md)**
  - Vues principales (List, Kanban, Calendar, Graph) pour Mailings et Campagnes
  - Formulaire email (Subject, Recipients, Mail Body, A/B Tests, Settings)
  - Boutons Send, Schedule, Test
  - Campagnes (Kanban, formulaire, création depuis mailing)
  - Listes et contacts, Configuration (Settings), portail désabonnement
  - Composants techniques (builder, snippets, thèmes, rapports)

### 03_integrations/
- **[Odoo Email Marketing - Intégrations Cross-App](./03_integrations/Odoo%20Email%20Marketing%20-%20Integrations%20Cross%20App.md)**
  - Dépendances (contacts, mail, html_builder, utm, link_tracker, social_media, web_tour, digest)
  - Intégration avec Contacts, CRM, Events, Sales, Mail, UTM, Link Tracker, HTML Builder, Digest, Social Media
  - Flux de données et recommandations pour Miyukini

### 04_specifications_miyukini/
- **[Odoo Email Marketing - Spécifications Opérateurs Miyukini](./04_specifications_miyukini/Odoo%20Email%20Marketing%20-%20Specifications%20Operateurs%20Miyukini.md)**
  - Architecture Opérateurs (9 Opérateurs identifiés)
  - Équipe d'Opérateurs EmailMarketingService
  - Contrat d'équipe et Mandats de Permission
  - Niveaux de sécurité (1-2 selon données)
  - Intégration avec les Cores

### 05_integration_cog/
- **[Odoo Email Marketing - Guide Intégration COG](./05_integration_cog/Odoo%20Email%20Marketing%20-%20Guide%20Integration%20COG.md)**
  - Architecture d'intégration COG
  - Patterns d'implémentation (WriteIntent, Mandats, envoi massif, désabonnement, filtres)
  - Exemples de code (pseudo-code Rust)
  - Gestion des erreurs et rollback
  - Intégration avec Kits existants

### 06_guides_implementation/
- **[Odoo Email Marketing - Guide Implémentation](./06_guides_implementation/Odoo%20Email%20Marketing%20-%20Guide%20Implementation.md)**
  - Architecture technique (crates Rust proposées)
  - Schémas de données (Mailing, List, Contact, Blacklist, Trace, Filter, CampaignUtm)
  - API et contrats
  - Plan de développement par phases (MVP → Complet)
  - Bornage fonctionnel et critères d'acceptation
  - Risques et mitigation

---

## Résumé Exécutif

### Fonctionnalités principales identifiées

1. **Envoi et planification**
   - Création/modification de mailings (sujet, destinataires, corps HTML)
   - Envoi immédiat ou planification (Schedule)
   - États Draft, In Queue, Sending, Sent, Cancel

2. **Listes et contacts**
   - Listes de diffusion, contacts email, abonnements
   - Import de contacts, lien optionnel avec Contact (res.partner)
   - Blacklist (exclusion globale des envois)

3. **Ciblage**
   - Destinataires par liste(s) ou par filtre dynamique (Contact, Lead/Opportunity, Event Registration, Sales Order, Mailing Contact)
   - Filtres composables (domaine Odoo)

4. **Contenu**
   - Sujet, corps HTML (templates et blocs drag-and-drop), preview text
   - A/B test (pourcentage, critère gagnant, date d'envoi final)

5. **Métriques et campagnes**
   - Traces par destinataire (sent, open, click, reply, bounce)
   - KPIs agrégés sur mailing (Sent, Delivered %, Opened %, Clicked %, Replied %)
   - Campagnes UTM (regroupement de mailings) avec métriques Revenues, Quotations, Opportunities, Clicks

6. **Conformité**
   - Désabonnement (lien dans l'email, portail) ; option blacklist lors du désabonnement
   - Paramètres : Mailing Campaigns, Blacklist Option, Dedicated Server, 24H Stat Mailing Reports

### Architecture Miyukini proposée

**9 Opérateurs :**
- MailingCampaignOperator, MailingListOperator, MailingContactOperator, MailingBlacklistOperator, MailingTraceOperator, MailingContentOperator, MailingFilterOperator, MailingCampaignUTMOperator, MailingUI

**1 Équipe d'Opérateurs :** EmailMarketingService

**Correspondance Miyukini :** MiyuEmailMarketing / MiyukiniEmailMarketing (EmailMarketingService)

**Niveaux de sécurité :** 1 (UI, templates génériques) à 2 (Sensitive) pour listes, contacts, blacklist, mailings, traces, campagnes

**Intégration Cores :**
- StrongFather : Décisions (envoi, planification, création listes/contacts/blacklist)
- KindMother : Persistance (WriteIntent)
- Master Butler : Permissions
- WorrySentinel : Quota, blacklist, niveau données personnelles
- Ever Buddy : Cycle de vie (mailings, campagnes)

---

## Statut de l'analyse

| Document | Statut | Version |
|----------|--------|---------|
| Logique Métier | ✅ Complété | 1.0 |
| Parcours Utilisateur | ✅ Complété | 1.0 |
| UI/UX | ✅ Complété | 1.0 |
| Intégrations Cross-App | ✅ Complété | 1.0 |
| Spécifications Opérateurs Miyukini | ✅ Complété | 1.0 |
| Guide Intégration COG | ✅ Complété | 1.0 |
| Guide Implémentation | ✅ Complété | 1.0 |

---

## Navigation

- **Index Odoo :** [../_index.md](../_index.md)
- **Analyse concurrentielle :** [../Odoo - Analyse Concurrentielle Complete.md](../Odoo%20-%20Analyse%20Concurrentielle%20Complete.md)

---

**Document :** Odoo Email Marketing — Index de Documentation  
**Version :** 1.0  
**Date :** 2026-02-01  
**Statut :** ✅ Analyse complète à 100% — référence pour implémentation Miyukini
