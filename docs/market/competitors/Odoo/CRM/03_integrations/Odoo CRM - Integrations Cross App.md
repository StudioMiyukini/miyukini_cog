# Odoo CRM — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **CRM** d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Code source GitHub Odoo 19.0

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres apps Odoo
- Flux de données inter-apps
- Mécanismes d'intégration
- APIs et hooks utilisés
- Événements partagés

---

## 1. Dépendances Principales

### 1.1 Modules Requis

**Dépendances explicites (`__manifest__.py`) :**
- `base` : Fonctionnalités de base (partners, companies, users, teams)
- `mail` : Messagerie et activités
- `portal` : Portail client (consultation opportunités)
- `web` : Framework web
- `utm` : Tracking marketing (campaigns, medium, source)

### 1.2 Modules Optionnels

**Dépendances optionnelles :**
- `sale` : Intégration ventes (conversion Opportunity → Quotation)
- `sale_management` : Gestion ventes avancée
- `account` : Intégration comptabilité (revenus, factures)
- `project` : Intégration projets (tâches depuis opportunités)
- `calendar` : Intégration calendrier (réunions depuis opportunités)
- `website` : Portail web public
- `website_form` : Formulaires web (création leads depuis site)

---

## 2. Intégrations Détaillées

### 2.1 Intégration avec Sales

**Flux :**
```
CRM Opportunity → Sales Quotation → Sales Order → Invoice
```

**Mécanismes :**
- Conversion Opportunity → Quotation : Wizard `crm.lead2opportunity.partner`
- Lien bidirectionnel : `opportunity_id` sur `sale.order` ↔ `sale_order_ids` sur `crm.lead`
- Synchronisation équipe : `team_id`, `user_id` depuis CRM vers Sales
- Synchronisation revenus : `expected_revenue` depuis CRM vers Sales
- Tags CRM : `tag_ids` (Many2many vers `crm.tag`) partagés

**Champs liés :**
- `opportunity_id` : Opportunité source (sur `sale.order`)
- `sale_order_ids` : Commandes générées (sur `crm.lead`)
- `sale_order_count` : Nombre de commandes (computed)
- `team_id` : Équipe commerciale (partagée)
- `user_id` : Commercial (partagé)
- `expected_revenue` : Revenu attendu (synchronisé)

**Hooks utilisés :**
- `crm.lead.action_set_won()` : Création commande lors de victoire
- `crm.lead._convert_opportunity_to_quotation()` : Conversion en devis
- `sale.order._compute_opportunity_id()` : Lien vers opportunité

**Wizard de conversion :**
- `crm.lead2opportunity.partner` : Wizard conversion Lead → Opportunity
- Options : Créer nouveau partenaire, utiliser partenaire existant, fusionner leads

**Recommandations pour Miyukini :**
- Intégration native avec Miyukini Sales
- Conversion Opportunity → Quotation fluide
- Lien bidirectionnel opportunité ↔ commande
- Synchronisation équipe et revenus

### 2.2 Intégration avec Accounting

**Flux :**
```
CRM Opportunity → Sales Order → Invoice → Revenue Recognition
```

**Mécanismes :**
- Revenus générés indirectement via Sales depuis CRM
- Tracking revenus : `expected_revenue`, `recurring_revenue` sur opportunités
- Lien factures : Via Sales Order → Invoice
- Revenus récurrents : `recurring_revenue` + `recurring_plan` (MRR)

**Champs liés :**
- `expected_revenue` : Revenu attendu (monétaire)
- `recurring_revenue` : Revenu récurrent (monétaire)
- `recurring_plan` : Plan récurrent (mensuel, annuel, etc.)
- `recurring_revenue_monthly` : MRR calculé (computed)

**Recommandations pour Miyukini :**
- Intégration indirecte via Miyukini Sales
- Tracking revenus depuis opportunités
- Support revenus récurrents (MRR)
- Lien opportunité → factures via commandes

### 2.3 Intégration avec Project

**Flux :**
```
CRM Opportunity → Project Task → Timesheet → Invoice
```

**Mécanismes :**
- Création tâches projet depuis opportunités
- Lien : `project_id`, `task_id` sur `crm.lead`
- Facturation timesheet depuis projets liés

**Champs liés :**
- `project_id` : Projet lié
- `task_id` : Tâche liée
- `project_issue_count` : Nombre de tâches (computed)

**Recommandations pour Miyukini :**
- Intégration avec module Project (si développé)
- Création tâches depuis opportunités
- Lien opportunité ↔ projet/tâche
- Facturation timesheet depuis projets

### 2.4 Intégration avec Calendar

**Flux :**
```
CRM Opportunity → Calendar Event (Meeting)
```

**Mécanismes :**
- Création réunions depuis opportunités
- Lien : `meeting_ids` sur `crm.lead`
- Planification depuis opportunité

**Champs liés :**
- `meeting_ids` : Réunions liées (One2many vers `calendar.event`)
- `meeting_count` : Nombre de réunions (computed)
- `meeting_display_label` : Libellé affichage réunions
- `meeting_display_date` : Date affichage réunions

**Actions :**
- `action_schedule_meeting` : Planifier réunion depuis opportunité

**Recommandations pour Miyukini :**
- Intégration avec module Calendar/Booking (si développé)
- Création rendez-vous depuis opportunités
- Lien opportunité ↔ réunions
- Planification depuis opportunité

### 2.5 Intégration avec Portal

**Flux :**
```
CRM Opportunity → Portal → Customer View
```

**Mécanismes :**
- Accès client aux opportunités via portail (si activé)
- Consultation historique
- Communication via portail

**Templates :**
- Templates portail pour opportunités (si activé)

**Recommandations pour Miyukini :**
- Portail client pour consultation opportunités (optionnel)
- Communication via portail
- Historique accessible

### 2.6 Intégration avec Website

**Flux :**
```
Website Form → CRM Lead
```

**Mécanismes :**
- Création leads depuis formulaires web
- Intégration `website_form` : Soumission formulaires → création leads
- Tracking marketing : `campaign_id`, `medium_id`, `source_id` depuis URL

**Champs liés :**
- `campaign_id` : Campagne marketing (depuis UTM)
- `medium_id` : Support marketing (depuis UTM)
- `source_id` : Source marketing (depuis UTM)
- `referred` : Référent (depuis UTM)

**Recommandations pour Miyukini :**
- Intégration avec module Website (si développé)
- Création leads depuis formulaires web
- Tracking marketing intégré (UTM)

---

## 3. Flux de Données Inter-Apps

### 3.1 Flux Principal Ventes

```
Website Form (website)
    ↓
CRM Lead (crm)
    ↓
CRM Opportunity (crm)
    ↓
Sales Quotation (sale)
    ↓
Sales Order (sale)
    ↓
Account Invoice (account)
    ↓
Account Payment (account)
```

### 3.2 Flux Principal Projet

```
CRM Opportunity (crm)
    ↓
Project Task (project)
    ↓
Timesheet (project.task)
    ↓
Invoice Line (account.move.line)
    ↓
Account Invoice (account)
```

### 3.3 Flux Calendrier

```
CRM Opportunity (crm)
    ↓
Calendar Event (calendar)
    ↓
Meeting → Follow-up → Opportunity Update
```

### 3.4 Flux Données Partagées

**Données partagées :**
- **Partner** : Client partagé entre CRM, Sales, Accounting, Portal
- **Team** : Équipe commerciale partagée entre CRM, Sales
- **User** : Commercial partagé entre CRM, Sales, Project
- **Currency** : Devises partagées entre toutes les apps
- **Company** : Entreprises partagées entre toutes les apps
- **Campaign/Medium/Source** : Tracking marketing partagé (UTM)
- **Tag** : Tags CRM partagés avec Sales

---

## 4. Mécanismes d'Intégration

### 4.1 Hooks et Overrides

**Hooks utilisés :**
- `crm.lead.action_set_won()` : Action lors de victoire (création commande)
- `crm.lead._convert_opportunity_to_quotation()` : Conversion en devis
- `crm.lead._onchange_partner_id()` : Mise à jour depuis partenaire
- `crm.lead._onchange_user_id()` : Mise à jour équipe depuis utilisateur
- `crm.lead._compute_sale_order_count()` : Calcul nombre commandes

**Overrides :**
- `sale.order` : Lien vers `opportunity_id`
- `res.partner` : Champs CRM (opportunity_count, lead_count)
- `calendar.event` : Lien vers `opportunity_id`

### 4.2 Événements et Signaux

**Événements :**
- Conversion Lead → Opportunity → Création partenaire (si nécessaire)
- Victoire Opportunity → Création commande (si configuré)
- Création réunion → Lien vers opportunité
- Création tâche → Lien vers opportunité

**Signaux :**
- `onchange_partner_id` : Mise à jour informations depuis partenaire
- `onchange_user_id` : Mise à jour équipe depuis utilisateur
- `onchange_team_id` : Mise à jour utilisateur depuis équipe

### 4.3 APIs et Méthodes Publiques

**Méthodes principales :**
- `crm.lead.action_set_won()` : Marquer comme gagné
- `crm.lead.action_set_lost()` : Marquer comme perdu
- `crm.lead.action_convert_opportunity()` : Convertir en opportunité
- `crm.lead.action_schedule_meeting()` : Planifier réunion
- `crm.lead.action_show_potential_duplicates()` : Afficher doublons

**Wizards :**
- `crm.lead2opportunity.partner` : Conversion Lead → Opportunity
- `crm.lead.lost` : Marquer comme perdu (avec raison)

**APIs externes :**
- Pas d'API REST publique dans CRM core
- APIs via `website_form` pour création leads depuis web
- APIs via `portal` pour consultation client

---

## 5. Intégrations avec Services Externes

### 5.1 Email Marketing

**Intégration :**
- `mass_mailing` : Envoi emails marketing vers leads
- Blacklist email : Gestion blacklist pour emails
- Tracking ouvertures/clics : Via UTM

**Flux :**
```
CRM Lead → Mass Mailing → Email Sent → Tracking → Lead Update
```

### 5.2 SMS Marketing

**Intégration :**
- `sms` : Envoi SMS vers leads
- Blacklist téléphone : Gestion blacklist pour téléphones
- Tracking envois : Via UTM

**Flux :**
```
CRM Lead → SMS → SMS Sent → Tracking → Lead Update
```

### 5.3 Formulaires Web

**Intégration :**
- `website_form` : Création leads depuis formulaires web
- Tracking UTM : Campagne, support, source depuis URL
- Validation automatique : Email, téléphone

**Flux :**
```
Website Form → Submission → CRM Lead Created → Assignment → Follow-up
```

---

## 6. Recommandations pour Miyukini

### 6.1 Intégrations Prioritaires

**Intégrations natives :**
1. **Miyukini Sales** : Conversion Opportunity → Quotation
2. **MiyuContacts** : Clients et gestion partenaire
3. **MiyuBooking** : Rendez-vous et réunions
4. **MiyuInvoice** : Tracking revenus (indirect via Sales)
5. **MiyuStore** : Produits et catalogues (pour opportunités)

### 6.2 Patterns d'Intégration

**Actions :**
- Conversion Lead → Opportunity → Quotation fluide
- Lien bidirectionnel opportunité ↔ commande
- Synchronisation équipe et revenus
- Tracking marketing intégré (UTM)

**Gouvernance COG :**
- StrongFather : Autorisation conversion, création commande
- KindMother : Persistance via WriteIntent
- Master Butler : Permissions CRM
- WorrySentinel : Sécurité données commerciales

### 6.3 Architecture d'Intégration

**Opérateurs proposés :**
1. **CRMLeadOperator** : Gestion leads
2. **CRMOpportunityOperator** : Gestion opportunités
3. **CRMConversionOperator** : Conversion Lead → Opportunity → Quotation
4. **CRMActivityOperator** : Gestion activités
5. **CRMTeamOperator** : Gestion équipes
6. **CRMReportOperator** : Génération rapports
7. **CRMUI** : Interface utilisateur

**Intégrations via BondingBrother :**
- Traduction intentions depuis Sales/Booking
- Traduction réponses vers sources
- Médiation sans autorité

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
