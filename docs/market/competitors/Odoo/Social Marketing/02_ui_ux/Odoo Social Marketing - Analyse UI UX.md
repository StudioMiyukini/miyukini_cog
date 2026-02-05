# Odoo Social Marketing — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Social Marketing** d'Odoo (versions 18.0 / 19.0), à partir de la documentation officielle. Il identifie les vues, composants, patterns de navigation et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18.0/19.0 (Social Marketing, Social posts, Social campaigns)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Tableau de bord principal (Feed) et colonnes streams
- Vues Posts (Kanban, Calendar, List, Pivot)
- Vues Campagnes (Kanban, List)
- Formulaire détail post et options (Message, Post on, Campaign, When, Push Notification Options)
- Page Campagne (template) et contenus multi-canal
- Configuration (Social Media, Social Accounts, Social Streams)
- Visiteurs (Kanban, List, Graph)
- Pop-up post et création de lead depuis commentaire

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Tableau de Bord Principal (Feed)

### 1.1 Structure

- **Zone centrale** : Feed avec une **colonne par stream** (compte social connecté).
- Chaque colonne affiche les publications de ce compte et les interactions (likes, commentaires).
- **En-tête de colonne** : Lien « Insights » vers les KPIs de la plateforme.
- **Bouton « Add A Stream »** : Coin supérieur gauche ; ouvre la fenêtre « Add a Stream » pour lier un nouveau compte (Facebook, Instagram, LinkedIn, Twitter, YouTube).
- **Bouton « New Post »** : Coin supérieur droit ; ouvre le formulaire de création de post.

### 1.2 Interaction

- Clic sur un post dans une colonne → **pop-up** avec contenu du post, métriques d’engagement (likes, commentaires) et zone « Write a comment… » pour répondre.
- Menu contextuel (trois points) sur un commentaire → **Create Lead** (conversion en lead CRM).

---

## 2. Vues Posts (Social Marketing ‣ Posts)

### 2.1 Choix de vues

Quatre vues disponibles (icônes en haut à droite sous la barre de recherche) :
- **Kanban** (défaut) : Cartes par post.
- **Calendar** : Calendrier des dates de publication (publiées ou planifiées) ; clic sur une date ouvre un formulaire de post pour cette date.
- **List** : Liste avec colonnes (Social Accounts, Message, Status) ; barre latérale gauche pour filtrer par Statut et par Comptes connectés.
- **Pivot** : Grille analytique personnalisable (mesures, dimensions) ; options « Insert in Spreadsheet », Flip Axis, Expand All, Download.

### 2.2 Formulaire détail post (création / édition)

**Sections principales :**

| Zone | Champs / Options |
|------|-------------------|
| **Your Post** | Company (multi-company), **Post on** (comptes sociaux + Push Notification), **Message** (contenu, emojis), **Attach Images** |
| **Campaign** | Champ optionnel ; sélection d’une campagne existante ou création (Create / Create and edit…) |
| **When** | Send Now / Schedule later ; si Schedule later → **Scheduled Date** (calendrier date/heure, Apply) |
| **Push Notification Options** | (Si Push sélectionné dans Post on) Notification Title, Target URL, Icon Image, Local Time, **Match all records** (règles de ciblage + Add condition, AND/OR) |

**Comportement :**
- **Post on** : Au moins une option obligatoire ; Push Notification n’apparaît que si Web Push Notifications est activé dans Website.
- **Twitter** : Compteur de caractères sous le champ Message.
- Bouton principal : « Post » (Send Now) ou « Schedule » (Schedule later).
- Aperçu visuel à droite : rendu du post pour chaque compte sélectionné (et push si applicable).

---

## 3. Vues Campagnes (Social Marketing ‣ Campaigns)

### 3.1 Kanban (défaut)

- **Colonnes** = étapes (stages) du pipeline.
- **Carte** : Nom de la campagne, responsable, tags.
- **Actions sur une colonne** : Icône engrenage (au survol, à gauche du +) → Fold, Edit Stage, Delete.
- **Nouvelle colonne** : Défilement horizontal ‣ « Add a Column » ‣ saisie et Add.
- **Création campagne** : Bouton « Create » (coin supérieur gauche) ou « + » en haut à droite d’une colonne → formulaire rapide (Campaign Name, Responsible, Tags) puis Add ou Edit pour la fiche complète.

### 3.2 List

- Même jeu de données qu’en Kanban, affichage en liste.

### 3.3 Template de campagne (fiche détail)

- **En-tête** : Nom, responsable, tags.
- **Boutons d’ajout de contenu** (selon apps et paramètres) :
  - **Send New Mailing** (si Mailing Campaigns activé dans Email Marketing)
  - **Send SMS** (si SMS Marketing installé)
  - **Send Social Post**
  - **Push Notification** (template post avec Push déjà pré-sélectionné)
- **Onglets** : Mailings, SMS, Social Media, Push Notifications (apparaissent au fur et à mesure des contenus ajoutés).
- **Smart buttons** : Revenues, Quotations, Leads, etc. (liens vers les enregistrements associés pour analyse).

---

## 4. Configuration

### 4.1 Social Media (Social Marketing ‣ Configuration ‣ Social Media)

- Page listant les plateformes : Facebook, Instagram, LinkedIn, Twitter, YouTube, Push Notifications.
- Chaque plateforme a un bouton **« Link account »** pour lancer la connexion OAuth.

### 4.2 Social Accounts (Configuration ‣ Social Accounts)

- Liste : Name, Handle/Short Name, Social Media, Created by, Company.
- Clic sur un compte pour modifier.

### 4.3 Social Streams (Configuration ‣ Social Streams)

- Liste : Social Media, Title, Type (Posts, Keyword, etc.), Created by, Company.
- Clic sur un stream pour modifier.

---

## 5. Visiteurs (Social Marketing ‣ Visitors)

- **Vue par défaut** : Kanban.
- **Autres vues** : List, Graph (en haut à droite).
- **Contenu** : Informations des visiteurs (identification, contexte).
- **Actions** : Si le visiteur a un contact en base : **Email**, **SMS**.

---

## 6. Patterns de Navigation

- **Menu principal** : Social Marketing → Feed (tableau de bord), Posts, Campaigns, Visitors ; Configuration → Social Media, Social Accounts, Social Streams.
- **Création** : New Post (dashboard ou Posts), Create campaign (Campaigns), Add a Stream (dashboard).
- **Détail** : Clic sur un post (liste/kanban/calendar) → formulaire ; clic sur un post dans le Feed → pop-up ; clic sur une campagne → template campagne.
- **Contextuel** : Trois points sur un commentaire → Create Lead ; engrenage sur une colonne campagne → Fold / Edit Stage / Delete.

---

## 7. Design et Accessibilité

- **Feed** : Layout type colonnes, adapté à plusieurs comptes ; pop-up pour détail post et commentaires.
- **Formulaires** : Champs groupés (Your Post, Campaign, When, Push Notification Options) ; aperçu visuel pour le rendu multi-plateforme.
- **Campagnes** : Kanban drag-and-drop pour les étapes ; smart buttons pour les métriques.
- **Responsive** : Documentation ne détaille pas explicitement le mobile ; interface Odoo standard responsive.

---

## 8. Recommandations pour Miyukini

1. **Opérateur d’interface SocialMarketingUI** : Regrouper Feed, Posts, Campagnes, Visiteurs et Configuration dans une navigation cohérente.
2. **Vues réutilisables** : List, Kanban, Calendar, Pivot pour les posts ; Kanban/List pour les campagnes ; Kanban/List/Graph pour les visiteurs.
3. **Formulaire post** : Séparation claire Post on / Message / Images / Campaign / When / Push options ; validation « au moins un canal » et affichage conditionnel Push selon configuration.
4. **Création de lead** : Action contextuelle claire (menu sur commentaire) avec fenêtre de conversion explicite (nouveau client / lien client existant / pas de lien).
5. **Campagnes** : Template unique avec onglets par type de contenu et smart buttons métriques ; gouvernance et permissions par Opérateur (StrongFather, Master Butler).

---

**Document** : Odoo Social Marketing — Analyse UI/UX  
**Version** : 1.0  
**Date** : 2026-02-01
