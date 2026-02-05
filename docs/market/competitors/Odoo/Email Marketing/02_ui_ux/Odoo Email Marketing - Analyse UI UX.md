# Odoo Email Marketing — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Email Marketing** d'Odoo (version 19.0), extraite de la documentation et du code source. Il identifie les vues, composants, patterns de navigation et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18/19, `https://github.com/odoo/odoo/tree/19.0/addons/mass_mailing/views`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Vues principales (List, Kanban, Calendar, Graph) pour Mailings et Campagnes
- Formulaire de création/édition d’email (onglets Subject, Recipients, Mail Body, A/B Tests, Settings)
- Composants spécialisés (builder drag-and-drop, filtres destinataires, widgets métriques)
- Patterns de navigation et recherche (filtres, regroupements, favoris)
- Portail désabonnement et préférences
- Paramètres (Configuration → Settings)

**Hors scope :**
- Implémentation technique détaillée (guide d’implémentation)
- Logique métier (document dédié)

---

## 1. Vues Principales

### 1.1 Tableau de bord Mailings (vue par défaut)

**Entrée :** Email Marketing → icône app → vue **Mailings** (liste par défaut).

**Filtre par défaut :** « My Mailings » (mailings de l’utilisateur courant). Suppression du filtre via ✖ à côté du filtre pour afficher tous les mailings.

**Vues disponibles (icônes en haut à droite) :**
- **Liste (☰)** : vue par défaut
- **Kanban**
- **Calendar**
- **Graph**

### 1.2 Vue Liste — Mailings

**Colonnes principales :**
- Date (date d’envoi)
- Subject (objet)
- Responsible (créateur / assigné)
- Sent (nombre envoyé)
- Delivered (%) (pourcentage livré)
- Opened (%) (pourcentage ouvert)
- Clicked (%) (pourcentage cliqué)
- Replied (%) (pourcentage répondu)
- Status (Draft, In Queue, Sent)

**Options :** Icône « Additional Options » (lignes avec points) à droite des colonnes pour afficher/masquer des colonnes.

### 1.3 Vue Kanban — Mailings

**Colonnes (stages) :**
- **Draft** : email en cours de rédaction
- **In Queue** : planifié
- **Sending** : en cours d’envoi
- **Sent** : envoyé

**Carte (card) :** Infos résumées (sujet, métriques, statut). Au survol coin supérieur droit : ⋮ (trois points) → Colorer, Supprimer, Archiver.

### 1.4 Vue Calendar — Mailings

- Calendrier mensuel (par défaut) : dates d’envoi ou de planification.
- Période : Day, Week, Month, Year, Show weekends (menu en haut à gauche).
- Navigation : flèches ◀ ▶ ; bouton « Today » pour revenir à la date du jour.
- Filtres à droite : Responsible, Status (cases à cocher).
- Icône panel-right pour masquer la barre latérale.

### 1.5 Vue Graph — Mailings

- Graphique en barres par défaut ; autres types : ligne, camembert.
- Mesures : « Measures » (ex. A/B Testing percentage, Count).
- Bouton « Insert in Spreadsheet » (si app Documents installée).
- Options de vue à droite du type de graphique.

### 1.6 Recherche, Filtres, Regroupements, Favoris

**Accès :** Icône ▼ à droite de la barre de recherche → menu déroulant.

**Filtres :**
- My Mailings, Sent Date, A/B Tests, A/B Tests to review, Archived, Add Custom Filter (popup avec 3 champs pour règles personnalisées).

**Group by :**
- Status, Sent By, Sent Period (Year, Quarter, Month, Week, Day), Add Custom Group.

**Favoris :**
- « Save current search » → titre, cases « Default filter » et « Shared » → Save. Sauvegardes visibles dans la section Favorites du menu.

---

## 2. Formulaire Email (Création / Édition)

### 2.1 En-tête

- **Champs principaux :** Subject (obligatoire), Recipients (liste ou filtre dynamique).
- **Subject :** Icône emoji (smiley +) en fin de champ ; étoile pour sauvegarder le mailing comme modèle (Mail Body).
- **Recipients :** Par défaut « Mailing List » → champ « Select mailing lists » (multi). Autres options : Contact, Event Registration, Lead/Opportunity, Mailing Contact, Sales Order → affichage d’un filtre « équation » sous le champ pour affiner (domaine Odoo).
- **Modify filter (→)** : Ajout de lignes de critères (AND/OR, branches) ; nombre d’enregistrements correspondants affiché en vert sous le filtre.

### 2.2 Onglets

**Mail Body**
- Choix de modèles prédéfinis (thèmes).
- Blocs drag-and-drop dans la barre latérale droite : Blocks, Customize, Design.
- Modèle « Plain Text » : corps vide + éditeur riche ; saisie `/` ouvre un menu d’éléments (titres, listes, etc.).
- Personnalisation de chaque bloc (texte, images, boutons, en-têtes, pieds de page).

**A/B Tests**
- Case « Allow A/B Testing » (non obligatoire).
- Si activé : champ « on (%) » (pourcentage de destinataires pour cette version, défaut 10), « Winner Selection » (Manual, Highest Open Rate, Highest Click Rate, Highest Reply Rate, Leads, Quotations, Revenues), « Send Final On » (date d’envoi de la version gagnante au reste).
- Bouton « Create an Alternative Version » : ouvre un nouvel onglet Mail Body pour une version alternative.

**Settings**
- **Email Content :** Preview Text, Send From, Reply To, Attachments, Responsible.
- Si **Mailing Campaigns** activé : champ **Campaign** (recherche ou « Create "[name]" » / « Create and edit… »).
- Section **Tracking** : Responsible ; Campaign (si activé).

### 2.3 Boutons d’action (envoi, planification, test)

- **Send :** Popup « Ready to unleash emails? » → « Send to all » → statut → Sent.
- **Schedule :** Popup « When do you want to send your mailing? » → champ « Send on » (calendrier) → Apply → Schedule → statut In Queue.
- **Test :** Popup « Test Mailing » → champ Recipients (adresses de test) → « Send Test ».

---

## 3. Campagnes (si Mailing Campaigns activé)

### 3.1 Menu et liste

- **Campaigns** dans l’en-tête Email Marketing → page Campagnes (Kanban par défaut).
- Vue liste : icône ☰.

### 3.2 Kanban Campagnes

- Stages (ex. New, etc.) ; cartes avec Campaign Name, Responsible, Tags.
- Bouton « New » ou ➕ en haut d’une colonne → carte rapide : Campaign Name, Responsible, Tags → Add / Edit / Delete (icône poubelle).
- « Edit » ouvre le formulaire campagne (après saisie du nom).

### 3.3 Formulaire Campagne

- Champs : Campaign Name, Responsible, Tags.
- Smart buttons : Revenues, Quotations, Opportunities, Clicks.
- Boutons : Send Mailing, Send SMS, Add Post, Add Push (notifications).
- Statut en haut à droite.

### 3.4 Création depuis un mailing

- Onglet Settings du mailing → champ Campaign → saisie du nom → « Create "[name]" » ou « Create and edit… » → popup Create Campaign (nom, responsable, tags, Add Post, Send Push, statut) → Save & Close / Discard.

---

## 4. Listes de diffusion et contacts

- **Mailing Lists :** Liste des listes (nom, nombre de contacts, etc.) ; formulaire liste (contacts, import, etc.).
- **Mailing List Contacts :** Contacts propres à l’app (sans fiche Contact) ; Email Marketing → Mailing Lists → Mailing List Contacts.
- **Import / ajout :** Wizards d’import et d’ajout à une liste (vues dans wizard/).

---

## 5. Configuration (Settings)

**Chemin :** Email Marketing → Configuration → Settings.

**Options :**
- **Mailing Campaigns** : activer la gestion des campagnes (menu Campaigns, champ Campaign sur les mailings).
- **Blacklist Option when Unsubscribing** : proposer la blacklist lors du désabonnement.
- **Dedicated Server** : serveur SMTP dédié ; lien vers la configuration du serveur.
- **24H Stat Mailing Reports** : rapport de performances des mailings envoyés la veille (digest).

---

## 6. Portail et désabonnement

- **Pages portail :** Désabonnement (mailing_templates_portal_unsubscribe.xml), formulaire de feedback (mailing_portal_subscription_feedback.xml), formulaire d’inscription (mailing_portal_subscription_form.xml), blocklist (mailing_portal_subscription_blocklist.xml).
- **Assets frontend :** mailing_portal.scss, subscribe.js, XML des blocs portail (voir manifest mass_mailing.assets).

---

## 7. Composants techniques (référence code)

- **Builder :** html_builder (mass_mailing.assets_builder, iframe) ; snippets dans views/snippets/ (colonnes, headers, headings, images, text, marketing, masonry, people, footer, etc.).
- **Éditeur / champs :** mass_mailing/static/src/editor, fields, iframe ; widget filtre destinataires (mailing_filter_widget, mailing_m2o_filter.js).
- **Thèmes mail :** mass_mailing/static/src/scss/themes (mass_mailing_mail.scss, themes_templates.xml, snippets_themes.xml).
- **Mobile :** mass_mailing_mobile.scss, mailing_mobile_preview_content.xml.
- **Rapports :** report/mailing_trace_report_views.xml (traces par campagne).

---

## 8. Recommandations pour Miyukini

- **Vues standardisées :** Conserver l’équivalent Liste / Kanban / Calendar / Graph pour les mailings et campagnes, avec filtres et regroupements explicites.
- **Formulaire en onglets :** Sujet + Destinataires en tête ; onglets Contenu, A/B Test, Paramètres (From, Reply To, Tracking, Campagne).
- **Builder gouverné :** Contenu email = capacité d’un Opérateur (sans autorité) ; validation StrongFather avant envoi ; pas d’exécution directe par les Cores.
- **Transparence avant envoi :** Affichage du nombre de destinataires, exclusion blacklist, quota restant, et rappel conformité (consentement).
- **Portail désabonnement :** Façade publique gouvernée (Mandat Public d’Accès) ; pas d’entrée dans le COG ; enregistrement des préférences via WriteIntent vers KindMother.

---

**Document :** Odoo Email Marketing — Analyse UI/UX  
**Version :** 1.0  
**Date :** 2026-02-01
