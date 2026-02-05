# Odoo SMS Marketing — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **SMS Marketing** d'Odoo (version 19.0), à partir de la documentation officielle. Il identifie les vues, patterns de navigation, formulaires et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0 (SMS Marketing), description des écrans et parcours.

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Tableau de bord principal et vues des mailings (Kanban, List, Calendar, Graph)
- Formulaire de création / édition de mailing SMS
- Interfaces listes de diffusion, contacts, blacklist
- Campagnes et A/B Test
- Reporting et Configuration
- Patterns de navigation et raccourcis

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Logique métier détaillée (document dédié)

---

## 1. Tableau de Bord Principal (SMS Marketing)

### 1.1 Vue par défaut : Kanban

- **Contenu** : Cartes des SMS mailings créés, avec informations principales et **statut** (Draft, In Queue, Sending, Sent).
- **Action** : Bouton **Create** (en haut à gauche) pour ouvrir un nouveau formulaire de mailing.
- **Coin supérieur droit** : Choix de vue (Kanban, List, Calendar, Graph).

### 1.2 Autres vues du tableau de bord

- **List** : Même jeu de données en liste (colonnes : sujet, statut, destinataires, dates, etc.).
- **Calendar** : Mailings positionnés sur le calendrier (planifiés ou envoyés). Clic sur une date future ouvre un formulaire vierge pour planifier un envoi à cette date.
- **Graph** : Données des mailings en graphiques et courbes, avec possibilité de grouper et trier pour l’analyse.

---

## 2. Formulaire de Mailing SMS (Création / Édition)

### 2.1 Champs principaux (au-dessus des onglets)

- **Subject** : Sujet / libellé du mailing.
- **Recipients** : Type de destinataires (Mailing List, Contact, etc.).
- **Select Mailing List** : Affiché si Recipients = Mailing List.
- Si Recipients = Contact (ou autre) : **filtres domaine** (équation par défaut ou « Add Filter ») pour cibler les contacts.

### 2.2 Onglet SMS Content

- **Champ texte** : Contenu du SMS (liens et emojis autorisés).
- **Sous le champ** : Nombre de caractères et **nombre de segments SMS** (coût).
- **Info** : Icône d’information pour le prix par pays si disponible.

### 2.3 Onglet Settings

- **Include opt-out link** : Case à cocher pour ajouter un lien de désinscription.
- **Tracking** : Champ **Responsible** (employé responsable).

### 2.4 Onglet A/B Test (si Mailing Campaigns activé)

- **Allow A/B Testing** : Case à cocher.
- Si activé : **pourcentage** de destinataires pour le test, **Winner Selection** (Manual, Highest Click Rate, Leads, Quotations, Revenues), **Send Final On** (date/heure).
- **Create an Alternate Version** : Bouton pour créer une variante du mailing.

### 2.5 Actions d’envoi (header ou barre d’actions)

- **Send** : Envoi immédiat.
- **Schedule** : Choix date et heure d’envoi.
- **Test** : Envoi à un ou plusieurs numéros de test (séparés par des virgules).

---

## 3. Listes de Diffusion

### 3.1 Menu Mailing Lists

- **Mailing Lists ‣ Mailing Lists** : Liste des listes (nom, statistiques).
- **Create** : Formulaire vierge (nom, **Is Public**).
- **Fiche liste** : Smart buttons (Recipients, Mailings, etc.), champs Nom, Is Public, édition via Edit / Save.

### 3.2 Mailing List Contacts

- **Mailing Lists ‣ Mailing List Contacts** : Liste des contacts de toutes les listes.
- **Filtre par défaut** : « Exclude Blacklisted Phone » dans la barre de recherche.
- Colonnes typiques : contact, liste(s), téléphone, email, etc.

---

## 4. Blacklist

- **SMS Marketing ‣ Configuration ‣ Blacklisted Phone Numbers** : Liste des numéros blacklistés.
- **Create** : Formulaire (numéro, case **Active**).
- **Fiche numéro** : Bouton **Unblacklist** pour retirer de la blacklist.
- **Favorites ‣ Import records** : Import d’une blacklist (migration).

---

## 5. Campagnes (Mailing Campaigns)

### 5.1 Page Campaigns

- **Menu** : SMS Marketing ‣ **Campaigns** (visible si Mailing Campaigns activé).
- **Vue** : Tableau de bord des campagnes (étapes / colonnes selon configuration), avec infos (nombre d’emails, posts, SMS, push).

### 5.2 Formulaire de campagne

- **Actions** : Send New Mailing, **Send SMS**, Send Social Post, Push Notifications.
- **Smart buttons** en haut : Engagement, Opportunities, etc. (métriques).
- **Champs** : Campaign Name, Responsible, Tags.
- **Onglets** : Un onglet par type d’envoi ajouté (SMS, email, etc.) pour consulter/éditer les mailings de la campagne.

---

## 6. Envoi depuis l’App Contacts

### 6.1 Fiche contact

- **Icône SMS** à côté du champ **Phone Number** : ouvre l’envoi d’un SMS direct à ce contact.

### 6.2 Vue liste Contacts

- Sélection multiple de contacts ‣ **Action ‣ Send SMS** : envoi groupé (saisie du message puis envoi).

### 6.3 Chatter

- Sous la fiche contact : historique des mailings envoyés (traçabilité des communications).

---

## 7. Reporting

- **SMS Marketing ‣ Reporting** : Page d’analyse.
- **Filtres et Mesures** : Menus déroulants pour configurer les métriques.
- **Vues** : Graph, List, Cohort pour visualiser les performances des mailings (envois, clics, conversions).

---

## 8. Configuration et Technique

### 8.1 Configuration SMS Marketing

- **Configuration ‣ Blacklisted Phone Numbers** : Gestion de la blacklist.
- **Configuration ‣ Link Tracker** (si disponible) : Suivi des liens utilisés dans les SMS.

### 8.2 Paramètres Email Marketing (impact SMS)

- **Email Marketing ‣ Configuration ‣ Settings** : Activation **Mailing Campaigns** (et A/B Test) pour SMS Marketing.

### 8.3 Mode développeur

- **Settings ‣ Developer Tools ‣ Activate the Developer Mode**.
- **Settings ‣ Technical ‣ SMS Templates** : Gestion des modèles SMS techniques (liste, création, édition).

---

## 9. Patterns de Navigation et UX

- **Entrée principale** : App SMS Marketing → Kanban des mailings → Create ou clic sur une carte pour éditer.
- **Destinataires** : Choix global (liste vs contact) puis affinage (liste précise ou filtres domaine).
- **Coût visible** : Caractères et segments SMS affichés sous le champ contenu ; lien vers tarifs par pays si disponible.
- **Envoi** : Trois options explicites (Send / Schedule / Test) pour éviter les envois accidentels.
- **Conformité** : Opt-out et blacklist accessibles depuis Configuration et depuis les listes/contacts.
- **Traçabilité** : Chatter sur les contacts, Reporting pour les analyses agrégées.

---

## 10. Points d’Attention pour Miyukini

- **Écrans** : Distinguer « écran de conception » (création mailing, campagnes, listes) et « écran de livraison » (dashboard, reporting, historique).
- **Sécurité** : Champs sensibles (numéros, listes) et actions critiques (envoi, blacklist) à protéger selon niveaux de sécurité (WorrySentinel, Master Butler).
- **Accessibilité** : Labels clairs pour Recipients, opt-out, A/B Test ; feedback immédiat sur caractères/segments et statut d’envoi.
- **Responsive** : Prise en compte des vues Liste/Kanban/Calendar sur différentes tailles d’écran pour les opérateurs terrain (mobile).

---

**Document** : Odoo SMS Marketing — Analyse UI/UX  
**Version** : 1.0  
**Date** : 2026-02-01
