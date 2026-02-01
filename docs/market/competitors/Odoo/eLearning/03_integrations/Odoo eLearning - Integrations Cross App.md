# Odoo eLearning — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **eLearning** (LMS) d'Odoo, en identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Documentation Odoo 18.0/19.0 — Websites / eLearning, et fonctionnalités publiées

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres apps Odoo
- Flux de données inter-apps
- Mécanismes d'intégration (certifications, paiement, mailing, forum)
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Modules requis (conceptuels)

**Fonctionnalités de base :**
- **Website** : Hébergement des pages cours et contenus, menu, thème, multi-website
- **Base / Web** : Framework, vues, authentification, utilisateurs
- **Mail** (ou équivalent) : Notifications (New Content, Completion), templates d’email

**Fonctionnalités optionnelles (Settings) :**
- **Surveys** : Certifications (questionnaires, notation, délivrance de certification)
- **eCommerce / Sale** : Paid Courses — produits de type « Course », panier, paiement, inscription après achat
- **Mailing** : Contact Attendees (mailings de masse aux inscrits), modèles d’email
- **Forum** : Forum dédié par cours (questions, entraide)

### 1.2 Flux de données

```
eLearning (cours, contenus, sections, inscriptions)
    │
    ├── Website : affichage catalogue, pages cours/contenus, publication
    ├── Surveys : certification (survey liée au cours), résultats, certification délivrée
    ├── eCommerce : produit type Course → achat → inscription au cours
    ├── Mailing : liste des inscrits par cours → envoi mailings, modèles New Content / Completion
    └── Forum : forum par cours (sujets, réponses)

Données entrantes :
- Website : domaine, site (multi-website), thème
- Surveys : questionnaire de certification, questions/réponses, seuil de réussite
- eCommerce : produit (Course), prix, commande
- Utilisateurs : inscription (open / invitation / paiement), progression, karma
```

---

## 2. Intégrations Détaillées

### 2.1 Website

**Rôle :** eLearning s’appuie sur le site Odoo pour exposer les cours et contenus.

**Flux :**
- **Catalogue** : Page « All Courses » (liste/grille de cours, filtres Course Groups)
- **Page cours** : URL dédiée par cours ; affichage description, inscription, sections/contenus
- **Page contenu** : URL par contenu (leçon) ; affichage selon type (Video, Document, Article, Image, Quiz)
- **Publication** : Switch Published/Unpublished sur le front-end ; visibilité selon statut
- **Multi-website** : Champ Website sur le cours pour restreindre l’affichage à un site
- **Article (contenu)** : Contenu éditable via Website Builder (pages dynamiques)

**Données échangées :**
- eLearning → Website : cours (titre, image, description, tags), contenus (titre, type, durée, ordre), groupes (tags, menu), paramètres (Show course to, Enroll Policy)
- Website → eLearning : contexte site (multi-website), utilisateur connecté (Signed In), clics (inscription, achat)

**Recommandations Miyukini :**
- Équivalent : MiyuWeb / MiyukiniWeb pour les pages catalogue, cours et contenus
- Publication gérée par l’Opérateur eLearning avec exposition via Façade Publique Gouvernée (Mandat Public d’Accès pour visiteurs non connectés)
- Contenu type « Article » : page dynamique gérée par le même service ou par MiyuCMS avec lien explicite cours ↔ page

### 2.2 Surveys (Certifications)

**Rôle :** Les certifications eLearning sont des enquêtes (surveys) Odoo intégrées au parcours.

**Flux :**
- **Configuration** : eLearning ‣ Configuration ‣ Settings → activer Certifications
- **Création** : Dans un cours, onglet Content → Add Certification → création/liaison d’un survey
- **Types de questions** : Choix multiple (simple/multiple), texte (ligne/multilignes), numérique, date/datetime, matrice
- **Parcours apprenant** : En fin de cours (ou selon structure), l’apprenant passe le questionnaire ; résultat (succès/échec) ; en cas de succès, certification délivrée et éventuellement karma
- **Récompenses** : Karma selon le nombre de tentatives (configuré dans le contenu certification ou le survey)

**Données échangées :**
- eLearning → Surveys : référence au survey (certification), cours, ordre dans le parcours
- Surveys → eLearning : réponses, score, statut (réussi/échoué), délivrance certification ; éventuellement karma

**Recommandations Miyukini :**
- Opérateur dédié MiyuSurveys (ou module Surveys) avec contrat d’équipe eLearningService + SurveyOperator
- Certification = entité gouvernée (StrongFather, KindMother) : lien cours ↔ survey, règles de délivrance, traçabilité
- WriteIntent pour enregistrer résultat certification et mise à jour progression/karma

### 2.3 eCommerce (Paid Courses)

**Rôle :** Vente d’accès aux cours via un produit de type « Course ».

**Flux :**
- **Configuration** : eLearning ‣ Configuration ‣ Settings → activer Paid Courses
- **Produit** : Dans eCommerce/Vente, créer un produit avec **Product Type = Course**
- **Cours** : Dans le cours, Options ‣ Enroll Policy = On Payment, sélection du **Product**
- **Parcours acheteur** : Visiteur → page cours → Buy → panier → paiement → inscription automatique au cours → accès aux contenus
- **Revenus** : Suivi des ventes et revenus par produit (Course)

**Données échangées :**
- eLearning → eCommerce : liste des cours avec On Payment et produit associé
- eCommerce → eLearning : commande payée (produit Course) → création/liaison inscription au cours pour l’acheteur

**Recommandations Miyukini :**
- Intégration MiyuStore / MiyuBilling : produit type « Course », lien produit ↔ cours
- Après paiement validé : Mandat StrongFather + WriteIntent KindMother pour créer l’inscription (CourseEnrollment) et ouvrir l’accès
- Pas de logique métier eCommerce dans l’Opérateur eLearning ; interface claire (contrat d’équipe) entre eLearningService et StoreOperator

### 2.4 Mailing

**Rôle :** Envoi d’emails aux inscrits (Contact Attendees) et notifications automatiques (New Content, Completion).

**Flux :**
- **Configuration** : eLearning ‣ Configuration ‣ Settings → activer Mailing
- **Contact Attendees** : Sur la fiche cours, bouton **Contact Attendees** → composition ou choix de modèle → envoi en masse aux inscrits
- **Modèles** : Options ‣ Communication → **New Content Notification** (email à l’ajout de contenu), **Completion Notification** (email à la fin du cours) ; édition via Internal link (modèle d’email)

**Données échangées :**
- eLearning → Mailing : liste des inscrits par cours (emails, noms), modèles New Content / Completion
- Mailing → eLearning : aucun retour métier direct (envoi uniquement)

**Recommandations Miyukini :**
- MiyuNotify pour notifications (New Content, Completion) et mailings
- Contact Attendees = capacité de l’Opérateur eLearning (ou équipe) avec Mandat pour accéder à la liste des inscrits et déclencher MiyuNotify
- Modèles d’email stockés et versionnés (KindMother) ; envoi via MiyuNotify avec audit (WorrySentinel)

### 2.5 Forum

**Rôle :** Forum dédié par cours pour les questions et l’entraide entre participants.

**Flux :**
- **Configuration** : eLearning ‣ Configuration ‣ Settings → activer Forum
- **Cours** : Onglet Options ‣ Communication → **Forum** : ajout d’un forum dédié au cours
- **Affichage** : Sur la page cours (front-end), accès au forum (sujets, réponses)
- **Droits** : Selon paramètres forum (utilisateurs connectés, inscrits au cours, etc.)

**Données échangées :**
- eLearning → Forum : création/liaison d’un forum par cours ; lien cours ↔ forum
- Forum → eLearning : pas de donnée métier eLearning ; forum = espace de discussion indépendant

**Recommandations Miyukini :**
- MiyuForum (ou équivalent) avec liaison explicite cours ↔ forum
- Contrat d’équipe : eLearningService peut créer/lier un forum à un cours (StrongFather, KindMother) ; la modération et le contenu relèvent de l’Opérateur Forum
- Accès au forum depuis la page cours (lien ou embed) selon gouvernance (Show course to, Course Attendees)

---

## 3. Synthèse des Dépendances

| App | Type | Rôle |
|-----|------|------|
| **Website** | Requise | Pages catalogue, cours, contenus ; publication ; multi-website ; Article (Website Builder) |
| **Surveys** | Optionnelle (Settings) | Certifications (questionnaires, délivrance) |
| **eCommerce / Sale** | Optionnelle (Settings) | Paid Courses (produit Course, paiement, inscription) |
| **Mailing** | Optionnelle (Settings) | Contact Attendees, New Content / Completion Notification |
| **Forum** | Optionnelle (Settings) | Forum dédié par cours |

---

## 4. Recommandations Miyukini (synthèse)

- **Website** : MiyuWeb pour exposition catalogue et pages ; publication et visibilité alignées sur Façade Publique Gouvernée et Mandat Public d’Accès.
- **Certifications** : MiyuSurveys (ou module Surveys) en contrat d’équipe avec eLearningService ; WriteIntent pour résultats et certifications ; traçabilité et niveau de sécurité (WorrySentinel).
- **Paid Courses** : MiyuStore / MiyuBilling pour produits type Course ; après paiement, création inscription via StrongFather + KindMother ; pas de duplication de logique panier/paiement dans eLearning.
- **Mailing** : MiyuNotify pour toutes les sorties email (notifications, Contact Attendees) ; listes d’inscrits fournies par eLearningService sous Mandat.
- **Forum** : MiyuForum avec lien cours ↔ forum ; gouvernance partagée (accès selon cours, modération côté Forum).

---

**Document créé le :** 2026-02-01  
**Version :** 1.0
