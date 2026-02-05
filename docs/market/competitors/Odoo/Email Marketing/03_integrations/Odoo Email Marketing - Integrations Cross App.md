# Odoo Email Marketing — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Email Marketing** d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** `__manifest__.py` et modules mass_mailing, documentation Odoo 19.0

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances explicites avec d'autres apps Odoo
- Flux de données inter-apps (Contacts, CRM, Sales, Events, UTM, Mail, Link Tracker)
- Mécanismes d'intégration (modèles étendus, domaines de filtre, campagnes)
- APIs et hooks utilisés
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Modules requis (`__manifest__.py`)

| Module | Rôle |
|--------|------|
| **contacts** | res.partner ; ciblage « Contact » et lien mailing.contact ↔ partner |
| **mail** | mail.mail, mail.thread, mail.blacklist (étendu) ; activités, discussion |
| **html_builder** | Éditeur drag-and-drop pour le corps HTML des emails |
| **utm** | utm.campaign, utm.medium, utm.source ; attribution marketing |
| **link_tracker** | Suivi des clics dans les emails (redirection + enregistrement) |
| **social_media** | Intégration campagnes sociales (Send SMS, Add Post, Push) depuis la fiche campagne |
| **web_tour** | Tours guidés (mass_mailing_tour) |
| **digest** | Digest quotidien (24H Stat Mailing Reports) |

### 1.2 Modules optionnels (non dans depends)

- **crm** : Ciblage « Lead/Opportunity » (mailing_filter model crm.lead) ; métriques campagnes (Opportunities, Revenues).
- **event** : Ciblage « Event Registration » (mailing_filter model event.registration).
- **sale** : Ciblage « Sales Order » (mailing_filter model sale.order) ; métriques campagnes (Quotations, Revenues).
- **website** : Formulaires d’inscription / désabonnement sur le site (portail mailing).
- **sms** : Boutons « Send SMS » sur les campagnes (si SMS Marketing ou social_media).

---

## 2. Intégrations Détaillées

### 2.1 Intégration avec Contacts (contacts)

**Flux :**
- **Ciblage :** Recipients = « Contact » → filtre sur `res.partner` (domaine personnalisable).
- **Lien contact :** `mailing.contact.partner_id` → `res.partner` ; un contact email marketing peut être lié à une fiche Contact ou exister seul (Mailing Contact uniquement).

**Modèles étendus :**
- `res.partner` (mass_mailing/models/res_partner.py) : champs ou comportements liés au mailing (ex. blacklist, opt-out, abonnements).
- `mailing.contact` : optionnellement `partner_id` Many2one vers `res.partner`.

**Recommandations Miyukini :**
- Opérateur Email Marketing consomme MiyuContacts (ou équivalent) pour le ciblage « Contact » et la cohérence des coordonnées.
- WriteIntent pour toute mise à jour de préférences (listes, blacklist) vers KindMother ; pas de modification directe du référentiel Contact par l’email marketing sans gouvernance.

### 2.2 Intégration avec CRM (crm)

**Flux :**
- **Ciblage :** Recipients = « Lead/Opportunity » → filtre sur `crm.lead` (stages, tags, lost reasons, sales teams, pays, etc.).
- **Métriques campagnes :** Si Mailing Campaigns activé, smart buttons « Opportunities », « Revenues » sur `utm.campaign` (agrégation des opportunités et revenus liés aux clics/emails de la campagne).

**Mécanismes :**
- `mailing.filter` : `model_id` = crm.lead ; `mailing_domain` = domaine de recherche.
- UTM sur les liens des emails → attribution des leads/opportunities à la campagne (utm_campaign_id sur crm.lead ou équivalent).
- Traces : `mailing.trace` peut avoir `model` = 'crm.lead', `res_id` = id du lead.

**Recommandations Miyukini :**
- Équipe Email Marketing + CRM sous Mandat commun pour les campagnes lead ; StrongFather décide de l’envoi ; flux ciblage en lecture seule depuis CRM (pas d’écriture CRM par l’email marketing sauf via contrats explicites).

### 2.3 Intégration avec Events (event)

**Flux :**
- **Ciblage :** Recipients = « Event Registration » → filtre sur `event.registration` (événement, statut, etc.).
- **Cas d’usage :** Rappels, annonces, enquêtes post-événement.

**Mécanismes :**
- `mailing.filter` : `model_id` = event.registration ; domaine sur événement, date, état.
- Traces : `model` = 'event.registration', `res_id` = id de l’inscription.

**Recommandations Miyukini :**
- Opérateur Email Marketing consomme le référentiel Events (MiyuEvents ou équivalent) en lecture pour le ciblage ; pas de modification des inscriptions depuis l’email marketing.

### 2.4 Intégration avec Sales (sale)

**Flux :**
- **Ciblage :** Recipients = « Sales Order » → filtre sur `sale.order` (état, client, date, etc.).
- **Métriques campagnes :** Smart buttons « Quotations », « Revenues » sur `utm.campaign` (devis et revenus liés à la campagne).

**Mécanismes :**
- `mailing.filter` : `model_id` = sale.order.
- UTM sur les liens → attribution des commandes à la campagne (utm_campaign_id sur sale.order ou lignes).

**Recommandations Miyukini :**
- Consommation en lecture du référentiel Sales pour ciblage ; agrégation des métriques (quotations, revenues) via contrat d’équipe avec l’Opérateur Sales, sans écriture directe sur les commandes par l’email marketing.

### 2.5 Intégration avec Mail (mail)

**Flux :**
- Envoi : génération de `mail.mail` (ou équivalent) par mailing ; file d’envoi ou envoi SMTP.
- Blacklist : `mail.blacklist` (email, active, reason) ; consulté avant chaque envoi pour exclure les destinataires.
- Discussion : `mail.thread` sur `mailing.mailing` (activités, messages) ; réponses détectées pour « Replied » et métriques.
- Rendu : `mail.render.mixin` (ou mail_render_mixin) pour variables (nom, prénom, etc.) et tracking (pixel, liens).

**Modèles étendus :**
- `mail.mail` (mass_mailing/models/mail_mail.py) : champs ou comportements spécifiques mass_mailing (trace, campagne).
- `mail.thread` (mass_mailing/models/mail_thread.py) : suivi des réponses pour mailing.trace.
- `mail.blacklist` (mass_mailing/models/mail_blacklist.py) : extension ou utilisation directe pour blacklist.

**Recommandations Miyukini :**
- Canal d’envoi (SMTP) et file d’emails gérés par un Opérateur/Kit dédié ; blacklist comme donnée gouvernée (KindMother, WriteIntent) ; réponses et tracking sans logique métier dans le Core.

### 2.6 Intégration avec UTM (utm)

**Flux :**
- Chaque mailing peut être rattaché à une campagne UTM (`utm.campaign`) si « Mailing Campaigns » activé.
- Les liens dans les emails peuvent inclure utm_source, utm_medium, utm_campaign pour attribution.
- Agrégation des métriques (Revenues, Quotations, Opportunities, Clicks) au niveau `utm.campaign`.

**Modèles :**
- `utm.campaign` (étendu dans mass_mailing/models/utm_campaign.py) : smart buttons, champs liés aux mailings et aux métriques CRM/Sales.
- `utm.medium`, `utm.source` : utilisés pour le tracking des liens.

**Recommandations Miyukini :**
- Campagne = entité gouvernée (StrongFather, Ever Buddy pour cycle de vie) ; métriques agrégées fournies par les Opérateurs CRM/Sales/Email selon Contrat d’Équipe, pas par un Core.

### 2.7 Intégration avec Link Tracker (link_tracker)

**Flux :**
- Les URLs dans le corps des emails sont réécrites pour passer par le serveur Odoo (ou service link_tracker) ; à chaque clic, enregistrement du clic + mise à jour de `mailing.trace` (state = open/clicked selon implémentation).
- Agrégation des clics pour KPI « Clicked » sur `mailing.mailing` et sur campagne.

**Recommandations Miyukini :**
- Tracking des clics = Outil ou Opérateur sans autorité ; enregistrement des traces via WriteIntent vers KindMother ; pas d’exécution par les Cores.

### 2.8 Intégration avec HTML Builder (html_builder)

**Flux :**
- Contenu HTML des mailings construit avec le builder (drag-and-drop) ; assets mass_mailing.assets_builder, iframe, snippets.
- Rendu final (variables, tracking) via mail_render_mixin.

**Recommandations Miyukini :**
- Builder = capacité d’édition de contenu (Opérateur ou Kit) ; le contenu est une donnée ; validation avant envoi par StrongFather ; pas de logique métier dans le builder.

### 2.9 Intégration avec Digest (digest)

**Flux :**
- Option « 24H Stat Mailing Reports » : le module digest envoie un résumé des performances des mailings envoyés la veille (données depuis mailing.mailing / mailing.trace).

**Recommandations Miyukini :**
- Digest = Opérateur ou Kit consommant les métriques (lecture seule) ; pas d’écriture sur les mailings par le digest.

### 2.10 Intégration avec Social Media (social_media)

**Flux :**
- Depuis la fiche campagne (utm.campaign) : boutons « Send SMS », « Add Post », « Add Push » pour orchestrer canaux sociaux et push en plus de l’email.
- Dépendance explicite dans mass_mailing ; implémentation réelle selon modules SMS / Social / Push installés.

**Recommandations Miyukini :**
- Campagne multi-canal = Équipe d’Opérateurs (Email, SMS, Social, Push) sous un même Mandat ; chaque canal reste gouverné par ses propres règles (StrongFather, WorrySentinel).

---

## 3. Synthèse des flux

- **Contacts / CRM / Events / Sales :** Lecture seule pour ciblage (filtres) ; UTM et traces pour attribution et métriques.
- **Mail :** Envoi, blacklist, réponses, rendu ; extensions de modèles mail.
- **UTM :** Campagnes et agrégation des KPIs ; campagnes = entité de regroupement.
- **Link Tracker :** Clics et mise à jour des traces.
- **HTML Builder / Digest / Social :** Contenu, rapports, multi-canal.

---

## 4. Recommandations pour Miyukini

- **Contrats d’équipe explicites :** Email Marketing avec Contacts, CRM, Sales, Events en lecture ; écriture limitée aux listes, contacts mailing, blacklist, traces, campagnes (via WriteIntent).
- **Pas de confiance implicite :** Chaque app reste souveraine ; les données partagées (ciblage, UTM) sont définies par contrat et mandats.
- **Sécurité :** Données personnelles (emails, comportement) en niveau Sensitive/Critical ; WorrySentinel et Master Butler pour quotas, blacklist et permissions.
- **Bridge inter-COG :** Si campagnes ou contacts sont gérés dans un autre COG, utiliser le pattern Bridge (transport uniquement, pas de gouvernance partagée).

---

**Document :** Odoo Email Marketing — Intégrations Cross-App  
**Version :** 1.0  
**Date :** 2026-02-01
