# Odoo eLearning — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **eLearning** (LMS) d'Odoo (version 18/19), à partir de la documentation officielle. Il identifie les vues, patterns de navigation, composants et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18.0/19.0 — Websites / eLearning

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Structure back-end (menus, fiches cours/contenu, onglets)
- Structure front-end (catalogue cours, page cours, contenus, publication)
- Composants (cartes cours, sections, contenus, quiz, certification)
- Patterns de navigation et actions
- Recommandations pour Miyukini

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Structure Back-End

### 1.1 Menus principaux

- **eLearning ‣ Courses ‣ Courses** : Liste / kanban des cours
- **eLearning ‣ Courses ‣ Contents** : Liste des contenus (tous cours confondus)
- **eLearning ‣ Configuration ‣ Course Groups** : Groupes de cours (filtres site)
- **eLearning ‣ Configuration ‣ Content Tags** : Tags de type de leçon
- **eLearning ‣ Configuration ‣ Settings** : Certifications, Paid Courses, Mailing, Forum

### 1.2 Fiche Cours

- **Vue carte (kanban)** : Clic sur une carte → ouverture du cours en back-end ; bouton **View course** → accès front-end
- **Formulaire cours** :
  - **Titre**, **Tags**, **Image** (placeholder avec (Edit) au survol)
  - **Onglets** : Content, Description, Options, Karma
- **Onglet Content** :
  - **Add Section** : ajout de sections (chapitres)
  - **Add Content** : ajout de contenus (slides) dans la section
  - **Add Certification** : ajout d’une certification (Surveys)
  - Liste des sections et contenus (ordre, titre, type)
- **Onglet Description** : Texte court sous le titre sur le site
- **Onglet Options** :
  - **Course** : Responsible, Website
  - **Communication** : Allow Reviews, Forum, New Content Notification, Completion Notification, (Contact Attendees en en-tête si Mailing)
  - **Access rights** : Prerequisites, Prerequisite Of, Show course to, Enroll Policy (Open / On Invitation / On Payment, Enroll Message, Invite, Product)
  - **Display** : Training / Documentation, Featured Content
- **Onglet Karma** :
  - **Rewards** : Karma pour Review, Finish
  - **Access Rights** : Karma requis pour Add Review, Add Comment, Vote
- **Actions** : **Go to Website** (smart button) pour accéder au front-end et publier

### 1.3 Fiche Contenu (Content)

- **Accès** : eLearning ‣ Courses ‣ Contents ‣ New, ou depuis un cours (Add content)
- **Champs principaux** : Content Title, Tags (content tags)
- **Onglets** :
  - **Document** : Course, Content Type (Image, Article, Document, Video, Quiz), Responsible, Duration, Allow Preview, Allow Download (Document), # of Public Views, # Total Views
  - **Description** : Texte « About » sur le site
  - **Additional Resources** : Lignes (lien ou fichier)
  - **Quiz** : Points Reward (karma par tentatives), lignes de questions (Question Name, réponses, Is correct answer, Comment)
- **Article** : Bouton **Go to Website** pour éditer la page avec Website Builder
- **Actions** : **Go to Website** pour publier le contenu sur le front-end

### 1.4 Configuration

- **Course Groups** : Liste ; New → Course Group Name, Menu Entry, Tag Name (avec couleur)
- **Content Tags** : Liste ; New → nom du tag
- **Settings** : Cases à cocher Certifications, Paid Courses, Mailing, Forum

---

## 2. Structure Front-End (Site)

### 2.1 Catalogue des cours (All Courses)

- **Vue** : Dashboard / grille de **cartes cours**
- **Filtres** : **Course Groups** (menu ou filtres par tags) pour affiner par thème, niveau, etc.
- **Carte cours** : Image, titre, description courte, tags, éventuellement indicateur (gratuit / payant, prérequis)
- **Clic** : Ouverture de la page du cours (description, inscription, liste des sections/contenus)

### 2.2 Page Cours

- **En-tête** : Titre, description, image
- **Publication** : Switch **Unpublished** / **Published** (coin supérieur droit)
- **Inscription** :
  - **Open** : Bouton type « Join » / « S’inscrire »
  - **On Invitation** : Message d’inscription (Enroll Message), lien ou email déjà envoyé par le formateur
  - **On Payment** : Bouton « Buy » / prix, redirection eCommerce
- **Contenu** :
  - **Training** : Liste des sections et contenus dans l’ordre ; progression (indicateur par contenu)
  - **Documentation** : Contenu consultable dans tout ordre ; **Featured Content** mis en avant sur la page d’accueil du cours
- **Dashboard du cours** : Liste des sections avec contenus (titres, types, tags contenu, durée)
- **Clic sur un contenu** : Ouverture de la leçon (lecture vidéo, document, article, quiz)

### 2.3 Page Contenu (leçon)

- **Type Image** : Affichage de l’image (upload ou Google Drive)
- **Type Article** : Page web (Website Builder)
- **Type Document** : Visualisation PDF ou lien Google (Drive) ; téléchargement si Allow Download
- **Type Video** : Lecteur intégré (YouTube, Vimeo, Google Drive)
- **Type Quiz** : Questions avec réponses ; sélection des réponses, validation ; feedback (Comment par réponse), récompenses karma selon tentatives
- **About** : Section description (onglet Description du contenu)
- **Additional Resources** : Liens ou fichiers listés
- **Publication** : Switch Unpublished / Published (coin supérieur droit)
- **Progression** : Indication « next » / « previous » ou barre de progression selon mode Training

### 2.4 Certification

- Intégrée comme étape du cours (Surveys)
- **UI** : Questionnaire (types de questions Surveys) ; envoi des réponses ; résultat (succès / échec) ; délivrance de la certification et karma si succès

### 2.5 Publication

- **Cours** : Page cours → switch **Published** (le cours devient visible)
- **Contenu** : Page contenu → switch **Published** (le contenu devient visible dans le cours si le cours est publié)
- **Bon usage** : Publier d’abord les contenus, puis le cours
- **Dépublication cours** : Le cours et tous ses contenus deviennent indisponibles

---

## 3. Composants et Patterns

### 3.1 Cartes cours (back-end et front-end)

- **Back-end** : Kanban des cours ; clic → formulaire ; **View course** → front-end
- **Front-end** : Grille de cartes (image, titre, description, tags) ; clic → page cours
- **Pattern** : Carte = résumé visuel + accès au détail

### 3.2 Onglets (cours et contenu)

- **Cours** : Content, Description, Options, Karma
- **Contenu** : Document, Description, Additional Resources, Quiz
- **Pattern** : Regroupement logique des champs et actions

### 3.3 Bouton « Go to Website »

- Présent sur la fiche cours et la fiche contenu
- **Action** : Navigation vers la page correspondante sur le site pour publication et prévisualisation
- **Pattern** : Pont back-end ↔ front-end

### 3.4 Switch Published / Unpublished

- **Emplacement** : Coin supérieur droit de la page cours ou contenu (front-end)
- **Comportement** : Bascule entre visible et masqué pour l’audience
- **Pattern** : Contrôle de visibilité immédiat

### 3.5 Filtres Course Groups

- **Emplacement** : Page All Courses (menu ou barre de filtres)
- **Comportement** : Filtrage des cours par groupe / tags
- **Pattern** : Découverte guidée par catégories

### 3.6 Quiz (front-end)

- Questions affichées une à une ou en liste selon configuration Surveys
- Réponses : choix unique/multiple, texte, numérique, date, matrice
- **Is correct answer** : Marqué en back-end ; feedback (Comment) à l’affichage
- **Récompenses karma** : Selon nombre de tentatives (configuré dans l’onglet Quiz du contenu)
- **Pattern** : Évaluation + gamification

---

## 4. Navigation et Actions Clés

### 4.1 Back-end

| Action | Emplacement | Résultat |
|--------|-------------|----------|
| New (cours) | Courses ‣ Courses | Création d’un cours |
| New (contenu) | Courses ‣ Contents ou fiche cours (Add content) | Création d’un contenu |
| Edit (image cours) | Fiche cours, survol placeholder | Édition image |
| Add Section / Add Content / Add Certification | Fiche cours, onglet Content | Ajout section, contenu, certification |
| Invite (inscription) | Fiche cours, Options ‣ Enroll Policy On Invitation | Copie lien ou envoi email |
| Contact Attendees | Fiche cours (en-tête si Mailing) | Mailing aux inscrits |
| Go to Website | Fiche cours ou contenu | Ouverture front-end |
| View course | Carte cours (back-end) | Ouverture page cours (front-end) |

### 4.2 Front-end

| Action | Emplacement | Résultat |
|--------|-------------|----------|
| Filtrer par Course Group | All Courses | Liste cours filtrée |
| S’inscrire (Open) | Page cours | Inscription directe |
| Acheter (On Payment) | Page cours | Panier / paiement |
| Suivre un contenu | Page cours → clic contenu | Ouverture leçon |
| Passer un quiz | Page contenu (type Quiz) | Réponses, validation, karma |
| Passer la certification | Étape certification du cours | Questionnaire Surveys, résultat, certification |
| Publier / Dépublier | Switch page cours ou contenu | Visibilité pour l’audience |

---

## 5. Recommandations pour Miyukini

- **Catalogue** : Conserver le pattern cartes + filtres par groupes/tags ; prévoir responsive et accessibilité (contraste, focus, labels).
- **Publication** : Regrouper la publication cours + contenus (option « Publish all ») et garder le switch visible sur chaque entité pour ajustements fins.
- **Parcours** : Différencier clairement Training (ordre imposé, barre de progression) et Documentation (navigation libre, Featured Content).
- **Quiz / Certification** : Aligner l’UI sur le modèle Surveys (MiyuSurveys) avec feedback immédiat et karma visible.
- **Back-end** : Conserver les onglets Content, Description, Options, Karma pour le cours ; Document, Description, Additional Resources, Quiz pour le contenu ; prévoir raccourcis (ex. « Add content » depuis la fiche cours qui pré-remplit le cours).
- **Go to Website** : Équivalent « Ouvrir sur le site » avec contexte (cours ou contenu) pour prévisualisation et publication sans quitter l’admin.

---

**Document créé le :** 2026-02-01  
**Version :** 1.0
