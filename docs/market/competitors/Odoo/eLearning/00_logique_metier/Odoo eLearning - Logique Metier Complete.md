# Odoo eLearning — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **eLearning** (LMS — Learning Management System) d'Odoo (version 18/19), à partir de la documentation officielle et des fonctionnalités publiées. Il identifie les modèles de données conceptuels, règles métier, workflows et mécanismes pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18.0/19.0 — Websites / eLearning

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles conceptuels (Cours, Contenus, Sections, Groupes de cours, Tags, Certifications)
- Règles métier et contraintes (inscription, prérequis, publication, karma)
- Workflows (création cours → contenu → publication ; parcours apprenant ; certification)
- Politiques d'accès (Everyone, Signed In, Course Attendees ; Open, On Invitation, On Payment)
- Gamification (karma, badges, récompenses)
- Intégrations (Surveys, Website, eCommerce, Forum, Mailing)

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles Conceptuels

### 1.1 Cours (Course / slide.channel)

**Rôle :** Conteneur principal d'un parcours de formation. Un cours regroupe des sections, des contenus et éventuellement une certification.

**Concepts clés :**
- **Titre** : Libellé du cours
- **Tags** : Catégorisation pour filtrage (Course Groups) et affichage sur le site
- **Image** : Illustration du cours (placeholder éditable)
- **Onglets** : Content, Description, Options, Karma
- **Responsable** : Utilisateur responsable du cours
- **Site web** : Si multi-website, restriction à un site donné
- **Publication** : Publié / Non publié (géré côté front-end)

**Règles métier :**
- Un cours est créé depuis le back-end (eLearning ‣ Courses ‣ Courses) ou rapidement depuis le front-end
- Les contenus sont ajoutés via l’onglet Content (sections, contenus, certification)
- La description apparaît sous le titre sur le site
- Publication : le cours et chaque contenu sont publiés séparément ; publier un cours le rend visible, mais les contenus doivent être publiés individuellement avant ou avec le cours
- Dépublication d’un cours rend le cours et tous ses contenus indisponibles

**Workflow :**
- Création → ajout sections/contenus/certification → configuration options (accès, inscription, karma) → publication front-end

### 1.2 Section (Section de cours)

**Rôle :** Subdivision logique d’un cours pour organiser les contenus (chapitres).

**Concepts clés :**
- **Nom / ordre** : Géré dans l’onglet Content du cours
- **Contenus** : Liste de contenus (slides) dans la section
- **Certification** : Élément optionnel en fin de cours (app Surveys)

**Règles métier :**
- Ajout via « Add Section » dans l’onglet Content
- Les contenus sont ajoutés par section via « Add Content »
- Une certification peut être ajoutée via « Add Certification » (dépendance Surveys)

### 1.3 Contenu (Content / slide.slide)

**Rôle :** Unité pédagogique au sein d’un cours — leçon, ressource ou quiz.

**Types de contenu :**
- **Image** : Upload (JPG, JPEG, PNG, SVG, GIF, WEBP, max 25 MB) ou lien Google Drive
- **Article** : Page web personnalisable via Website Builder (front-end)
- **Document** : PDF upload ou lien Google Drive (Slides, Doc, Sheets)
- **Video** : Lien YouTube, Google Drive ou Vimeo
- **Quiz** : Questions avec réponses (voir onglet Quiz)

**Concepts clés (onglet Document) :**
- **Cours** : Cours auquel le contenu appartient
- **Type de contenu** : Image, Article, Document, Video, Quiz
- **Responsable** : Utilisateur responsable (défaut : créateur du cours)
- **Durée** : Temps estimé pour terminer la leçon
- **Allow Preview** : Contenu accessible à tous (y compris non inscrits)
- **Allow Download** : (Document uniquement) Téléchargement autorisé
- **# of Public Views** / **# Total Views** : Statistiques de vues (lecture seule)

**Onglet Description :** Texte affiché dans la section « About » du contenu sur le site.

**Onglet Additional Resources :** Liens ou fichiers complémentaires pour les participants.

**Onglet Quiz :** Pour type Quiz — questions, réponses, « Is correct answer », commentaires par réponse ; récompenses karma selon le nombre de tentatives.

**Règles métier :**
- Un contenu appartient à un seul cours
- Création possible depuis eLearning ‣ Courses ‣ Contents ou depuis la fiche cours (Add content)
- Tags de contenu (Content Tags) : visibles sur le dashboard du cours (ex. théorie, exercices)
- Publication : chaque contenu est publié/dépublié individuellement sur le front-end

**Workflow :**
- Création → choix type → remplissage onglets (Document, Description, Additional Resources, Quiz si applicable) → publication front-end

### 1.4 Groupes de cours (Course Groups)

**Rôle :** Filtrage et découverte des cours sur le site (dashboard « All Courses »). Les utilisateurs filtrent par groupe / tags.

**Concepts clés :**
- **Nom du groupe** : Libellé
- **Menu Entry** : Si activé, le groupe apparaît dans le menu du site pour la recherche
- **Tags** : Liste de tags associés au groupe (avec couleur par tag)

**Règles métier :**
- Configuration : eLearning ‣ Configuration ‣ Course Groups
- Les cours sont associés à des tags ; les groupes agrègent des tags pour le filtrage
- Un cours peut avoir plusieurs tags ; un groupe peut avoir plusieurs tags

### 1.5 Tags (Cours et Contenu)

**Rôle :**
- **Tags de cours** : Catégorisation des cours (Course Groups, filtrage site)
- **Content Tags** : Identification du type de leçon (théorie, exercices, etc.) sur le dashboard du cours

**Règles métier :**
- Tags de cours : définis sur la fiche cours ; utilisés dans Course Groups
- Content Tags : eLearning ‣ Configuration ‣ Content Tags ; assignés aux contenus

### 1.6 Certification

**Rôle :** Évaluation des compétences et délivrance d’une certification (intégration app **Surveys**).

**Concepts clés :**
- Ajout via « Add Certification » dans l’onglet Content d’un cours
- Questionnaire basé sur Surveys (types : choix multiple, texte, numérique, date, matrice, etc.)
- Récompenses karma possibles selon le nombre de tentatives
- Certification officielle des compétences à la réussite

**Règles métier :**
- Option activée dans eLearning ‣ Configuration ‣ Settings (Certifications)
- Partie intégrante du parcours (en fin de cours ou selon structure)
- Les apprenants passent le questionnaire ; en cas de succès, certification délivrée

### 1.7 Options de cours (Options tab)

**Course :**
- **Responsible** : Utilisateur responsable
- **Website** : Restriction au site (multi-website)

**Communication :**
- **Allow Reviews** : Les participants peuvent liker, commenter, soumettre des avis
- **Forum** : Forum dédié au cours (si option Forum activée dans Settings)
- **New Content Notification** : Modèle d’email envoyé aux inscrits à l’ajout de contenu
- **Completion Notification** : Modèle d’email envoyé à la fin du cours
- **Contact Attendees** : Envoi de mailings de masse aux inscrits (si Mailing activé dans Settings)

**Access rights :**
- **Prerequisites** : Un ou plusieurs cours à avoir complétés avant d’accéder à ce cours
- **Prerequisite Of** : (lecture seule) Cours qui ont ce cours en prérequis
- **Show course to** : Everyone | Signed In | Course Attendees
- **Enroll Policy** :
  - **Open** : Inscription libre
  - **On Invitation** : Inscription sur invitation (lien ou email) ; message d’inscription configurable
  - **On Payment** : Inscription après achat (produit associé) ; option « Paid Courses » requise dans Settings ; seul un produit de type « Course » peut être sélectionné

**Display :**
- **Training** : Contenu en ordre imposé (parcours de formation)
- **Documentation** : Contenu consultable dans n’importe quel ordre ; champ **Featured Content** pour mettre en avant des contenus sur la page d’accueil du cours

### 1.8 Karma (Gamification)

**Rôle :** Points et récompenses pour encourager la participation.

**Récompenses (attribuées aux participants) :**
- **Review** : Karma pour avoir soumis un avis
- **Finish** : Karma pour avoir terminé le cours

**Droits d’accès (karma requis) :**
- **Add Review** : Points karma nécessaires pour ajouter un avis
- **Add Comment** : Points karma pour commenter
- **Vote** : Points karma pour voter

**Règles métier :**
- Configuration dans l’onglet Karma du cours
- Les points sont attribués automatiquement selon les actions (review, finish)
- Les seuils karma conditionnent la possibilité de review, comment, vote

---

## 2. Règles Métier Transverses

### 2.1 Publication (front-end)

- Les cours et contenus sont **publiés depuis le front-end** (bouton / switch Published / Unpublished)
- Accès : bouton « Go to Website » sur la fiche cours ou contenu
- **Cours** : publication du cours le rend visible ; dépublication rend le cours et tous ses contenus indisponibles
- **Contenus** : chaque contenu est publié séparément ; une bonne pratique est de publier les contenus avant de publier le cours
- Un contenu publié n’est visible par l’audience que si le cours dont il fait partie est lui-même publié

### 2.2 Prérequis

- Un cours peut exiger un ou plusieurs **prérequis** (autres cours)
- L’accès au cours n’est autorisé qu’après complétion des prérequis (selon logique Odoo)
- Le champ **Prerequisite Of** (lecture seule) liste les cours qui dépendent de ce cours

### 2.3 Inscription (Enroll Policy)

- **Open** : Toute personne (selon « Show course to ») peut s’inscrire sans action supplémentaire
- **On Invitation** : L’administrateur envoie un lien (Copy) ou un email (Invite) ; le message d’inscription (Enroll Message) est affiché sous le titre
- **On Payment** : L’utilisateur doit acheter le produit (type Course) associé ; nécessite eCommerce + option Paid Courses dans Settings

### 2.4 Visibilité (Show course to)

- **Everyone** : Visible et accessible selon la politique d’inscription (y compris non connectés si Open)
- **Signed In** : Réservé aux utilisateurs connectés
- **Course Attendees** : Réservé aux personnes déjà inscrites au cours

### 2.5 Paramètres eLearning (Settings)

- **Certifications** : Active les certifications (Surveys) dans les cours
- **Paid Courses** : Active la vente de cours (produit type Course, inscription On Payment)
- **Mailing** : Active les mailings de masse aux inscrits (Contact Attendees) et les modèles d’email (New Content, Completion)
- **Forum** : Active le forum dédié par cours

---

## 3. Workflows Principaux

### 3.1 Création et publication d’un cours

1. eLearning ‣ Courses ‣ New
2. Titre, tags, image, onglets Content (sections, contenus, certification), Description, Options (accès, inscription, affichage), Karma
3. Sauvegarde
4. Front-end : « Go to Website » → publier chaque contenu si besoin → publier le cours

### 3.2 Parcours apprenant

1. Découverte : site → All Courses (filtres par Course Groups / tags)
2. Inscription : selon Enroll Policy (open / invitation / paiement)
3. Consommation : contenus dans l’ordre (Training) ou libre (Documentation)
4. Quiz / Certification : passage des évaluations, obtention de la certification si succès
5. Fin de cours : notification (Completion Notification), karma (Finish)

### 3.3 Création d’un contenu

1. eLearning ‣ Courses ‣ Contents ‣ New (ou depuis fiche cours, Add content)
2. Titre, tags, onglet Document : cours, type (Image, Article, Document, Video, Quiz), responsable, durée, Allow Preview (Allow Download si Document)
3. Description, Additional Resources, Quiz (si type Quiz)
4. Publication sur le front-end

---

## 4. Intégrations Métier

### 4.1 Surveys (Certifications)

- Les certifications sont des enquêtes (survey) Odoo
- Types de questions : choix multiple (simple/multiple), texte, numérique, date, matrice, etc.
- Récompenses karma par nombre de tentatives
- Résultat : certification officielle des compétences

### 4.2 Website

- Cours et contenus sont affichés sur le site (pages dynamiques)
- Article : contenu éditable via Website Builder
- Multi-website : champ Website sur le cours pour restreindre l’affichage

### 4.3 eCommerce (Paid Courses)

- Produit de type **Course** lié au cours
- Inscription « On Payment » : achat du produit → accès au cours
- Option Paid Courses dans Settings

### 4.4 Forum

- Option Forum dans Settings
- Onglet Options ‣ Communication : forum dédié au cours
- Communauté : questions, entraide

### 4.5 Mailing

- Option Mailing dans Settings
- Modèles d’email : New Content Notification, Completion Notification
- Bouton Contact Attendees : envoi de mailings aux inscrits

---

## 5. Synthèse des Entités

| Entité | Rôle principal |
|--------|-----------------|
| **Cours** | Conteneur formation (sections, contenus, certification, options, karma) |
| **Section** | Subdivision du cours (chapitres) |
| **Contenu** | Unité pédagogique (Image, Article, Document, Video, Quiz) |
| **Course Group** | Filtrage et découverte (tags, menu site) |
| **Tags (cours)** | Catégorisation cours |
| **Content Tags** | Type de leçon (théorie, exercices) |
| **Certification** | Évaluation et certification (Surveys) |
| **Karma** | Gamification (récompenses, droits review/comment/vote) |

---

**Document créé le :** 2026-02-01  
**Version :** 1.0
