# Odoo eLearning — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **eLearning** (LMS) d'Odoo, en identifiant les personas, scénarios d'usage, processus d'onboarding et points de friction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18.0/19.0 — Websites / eLearning

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs
- Parcours d'onboarding
- Scénarios d'usage principaux
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles Utilisateurs

### 1.1 Administrateur eLearning / Responsable formation

**Profil :**
- Rôle : Configuration et gouvernance du LMS
- Responsabilités :
  - Activer les options (Certifications, Paid Courses, Mailing, Forum)
  - Créer et gérer les Course Groups et tags (cours, contenu)
  - Définir les bonnes pratiques de création de cours et de contenu
  - Superviser les statistiques (vues, inscriptions, certifications)

**Besoins :**
- Paramètres centralisés (eLearning ‣ Configuration ‣ Settings, Course Groups, Content Tags)
- Vue d’ensemble des cours (back-end : eLearning ‣ Courses ‣ Courses)
- Rapports sur la progression et les certifications

**Permissions :**
- Accès Configuration eLearning
- Droits d’édition sur tous les cours et contenus

### 1.2 Créateur de cours / Formateur

**Profil :**
- Rôle : Conception et mise à jour des parcours de formation
- Responsabilités :
  - Créer des cours (titre, tags, image, sections, contenus, certification)
  - Configurer les options (accès, inscription, prérequis, karma)
  - Rédiger les descriptions et messages d’inscription
  - Publier cours et contenus sur le front-end
  - Envoyer des invitations (Enroll Policy On Invitation) ou gérer les produits (On Payment)
  - Utiliser Contact Attendees pour les mailings (si Mailing activé)

**Besoins :**
- Création rapide depuis le back-end ou le front-end
- Gestion des sections et contenus (Image, Article, Document, Video, Quiz)
- Intégration certification (Surveys)
- Prévisualisation avant publication
- Statistiques par contenu (# Public Views, # Total Views)

**Permissions :**
- Accès eLearning ‣ Courses ‣ Courses, Contents
- Droits d’édition sur les cours dont il est responsable
- Accès Website pour publication
- Accès Surveys pour certifications
- Accès eCommerce pour produits type Course (si Paid Courses)

### 1.3 Apprenant / Participant

**Profil :**
- Rôle : Suivre les cours, passer les quiz et certifications, obtenir des badges / karma
- Responsabilités :
  - Découvrir les cours (All Courses, filtres par Course Groups / tags)
  - S’inscrire (open, invitation, paiement)
  - Consulter les contenus dans l’ordre (Training) ou libre (Documentation)
  - Passer les quiz et la certification
  - Lire les ressources additionnelles, participer au forum (si activé)
  - Liker, commenter, soumettre des avis (selon karma)

**Besoins :**
- Navigation claire sur le site (cours, sections, contenus)
- Indicateurs de progression (avancement, temps estimé)
- Accès aux ressources (téléchargement si Allow Download)
- Retour sur les quiz (réponses correctes, commentaires)
- Certification et karma visibles

**Permissions :**
- Accès front-end (site) selon Show course to et Enroll Policy
- Inscription selon politique (Open, Invitation, Payment)
- Accès aux contenus publiés du cours inscrit

### 1.4 Utilisateur invité (non inscrit)

**Profil :**
- Rôle : Visiteur du site consultant l’offre de formation
- Responsabilités :
  - Parcourir All Courses et filtrer par groupes / tags
  - Consulter les contenus en « Allow Preview » si disponibles
  - S’inscrire si politique Open, ou demander une invitation / acheter (On Payment)

**Besoins :**
- Catalogue de cours lisible et filtrable
- Aperçu des contenus en preview
- Processus d’inscription simple (lien, email, paiement)

**Permissions :**
- Accès public au site eLearning selon Show course to (Everyone)
- Aucun accès aux contenus réservés aux inscrits sauf Allow Preview

### 1.5 Responsable commercial / Marketing

**Profil :**
- Rôle : Vente de formations (Paid Courses), communication aux inscrits
- Responsabilités :
  - Associer un produit (type Course) au cours (On Payment)
  - Utiliser Contact Attendees pour campagnes (Mailing)
  - Suivre les inscriptions et revenus

**Besoins :**
- Lien cours ↔ produit eCommerce
- Envoi de mailings aux inscrits (Mailing)
- Données d’inscription et de complétion

**Permissions :**
- Accès eLearning (cours, options) et eCommerce (produits)
- Accès Mailing pour Contact Attendees

---

## 2. Parcours d’Onboarding

### 2.1 Premier déploiement (administrateur)

1. Installer / activer l’app eLearning (et dépendances : Website, éventuellement Surveys, eCommerce, Forum, Mailing)
2. eLearning ‣ Configuration ‣ Settings : activer Certifications, Paid Courses, Mailing, Forum selon besoin
3. Configuration ‣ Course Groups : créer les groupes (nom, Menu Entry, tags avec couleurs)
4. Configuration ‣ Content Tags : créer les tags de type de leçon (ex. théorie, exercices)
5. Créer un premier cours de test (titre, section, un contenu) et le publier sur le front-end pour valider le flux

### 2.2 Premier cours (formateur)

1. eLearning ‣ Courses ‣ New
2. Renseigner titre, tags, image, Description
3. Onglet Content : Add Section, Add Content (au moins un contenu : Video ou Document par exemple)
4. Onglet Options : Responsible, Website, Show course to, Enroll Policy (Open / Invitation / Payment), Prerequisites si besoin, Display (Training ou Documentation)
5. Onglet Karma : récompenses Review / Finish, seuils Add Review / Add Comment / Vote
6. Sauvegarder
7. Go to Website : publier les contenus puis le cours
8. Tester l’inscription et le parcours apprenant

### 2.3 Premier apprenant

1. Accéder au site (URL eLearning / All Courses)
2. Parcourir ou filtrer par Course Groups / tags
3. Cliquer sur un cours, lire la description
4. S’inscrire (bouton selon politique : Join, Request Invitation, Buy)
5. Accéder au contenu, suivre les leçons, passer quiz / certification si présents
6. Consulter progression, karma, certification

---

## 3. Scénarios d’Usage Principaux

### 3.1 Création et publication d’un cours complet

- **Acteur :** Formateur
- **Étapes :** New course → titre, tags, image → Content : sections + contenus (Video, Document, Quiz) + certification → Description, Options (accès, inscription, prérequis), Karma → Save → Go to Website → publier contenus puis cours
- **Résultat :** Cours visible sur le site, inscrits peuvent le suivre

### 3.2 Inscription par invitation

- **Acteur :** Formateur + Apprenant
- **Étapes :** Formateur configure Enroll Policy = On Invitation, Enroll Message → Invite → Copy link ou Send by Email → Apprenant reçoit le lien/email → ouvre la page → s’inscrit → accède au cours
- **Résultat :** Apprenant inscrit sans paiement, accès aux contenus

### 3.3 Vente de cours (Paid Courses)

- **Acteur :** Admin / Formateur, Apprenant
- **Étapes :** Settings : Paid Courses activé → Produit type Course créé dans eCommerce → Cours : Enroll Policy = On Payment, Product sélectionné → Apprenant va sur le site → clique sur le cours → Buy → paiement → inscription automatique → accès au cours
- **Résultat :** Revenus trackés, accès conditionné au paiement

### 3.4 Parcours avec prérequis

- **Acteur :** Apprenant
- **Étapes :** Cours B a Cours A en prérequis → Apprenant accède au catalogue → Cours B affiché mais verrouillé ou message « Complete Course A first » → Apprenant suit Cours A jusqu’à complétion → déblocage Cours B → inscription et suivi
- **Résultat :** Parcours ordonné, prérequis respectés

### 3.5 Certification et karma

- **Acteur :** Apprenant
- **Étapes :** Cours avec certification (Surveys) et karma (Review, Finish) → Apprenant termine les contenus → passe la certification → obtient la certification + karma Finish → peut soumettre un avis (karma Review) → selon seuils karma, peut commenter et voter
- **Résultat :** Certification délivrée, gamification active

### 3.6 Mailings aux inscrits

- **Acteur :** Formateur / Marketing
- **Étapes :** Ouvrir un cours → Contact Attendees (si Mailing activé) → composer ou choisir un modèle → envoi en masse aux inscrits
- **Résultat :** Communication ciblée (annonce nouveau contenu, rappel, etc.)

---

## 4. Points de Friction Identifiés

### 4.1 Publication dédoublée

- **Constat :** Cours et contenus sont publiés séparément sur le front-end ; une erreur courante est de publier le cours sans avoir publié les contenus, ou l’inverse.
- **Recommandation Miyukini :** Option « Publish course and all contents » ou workflow guidé (publier contenus puis cours en une action).

### 4.2 Prérequis et ordre (Training vs Documentation)

- **Constat :** En mode Training, l’ordre est imposé ; en mode Documentation, l’ordre est libre. Les prérequis (cours entiers) sont distincts de l’ordre des contenus.
- **Recommandation Miyukini :** Clarifier dans l’UI la différence prérequis (cours) vs ordre des contenus (sections).

### 4.3 Certification (Surveys) et eLearning

- **Constat :** La certification repose sur l’app Surveys ; la configuration se fait dans les deux contextes (eLearning pour l’ajout au cours, Surveys pour le questionnaire).
- **Recommandation Miyukini :** Unifier l’expérience (un seul lieu pour définir certification + questions) ou bien documenter le flux Surveys ↔ eLearning.

### 4.4 Multi-website

- **Constat :** Champ Website sur le cours pour restreindre l’affichage ; les utilisateurs peuvent oublier de le renseigner ou de filtrer par site.
- **Recommandation Miyukini :** Règles par défaut claires (site courant, héritage) et filtres explicites dans les vues back-end.

### 4.5 Karma et droits (Review, Comment, Vote)

- **Constat :** Les seuils karma peuvent bloquer les nouveaux participants (pas assez de points pour commenter).
- **Recommandation Miyukini :** Valeurs par défaut raisonnables (ex. 0 pour Comment) et explication dans l’UI (tooltip, aide).

---

## 5. Recommandations pour Miyukini

- **Service proposé :** Miyukini eLearning ou Miyu eLearning (Équipe d’Opérateurs : eLearningService).
- **Personas :** Conserver les rôles Administrateur, Formateur, Apprenant, Invité, Marketing ; les faire correspondre à des Mandats de Permission et niveaux de sécurité (WorrySentinel).
- **Publication :** Unifier la notion de publication (cours + contenus) avec un workflow explicite et option « tout publier ».
- **Certification :** Intégrer l’équivalent Surveys (MiyuSurveys ou module dédié) avec un contrat d’équipe clair (eLearningService + SurveyOperator).
- **Paid Courses :** Intégration avec MiyuStore / MiyuBilling pour produits type Course et inscription On Payment.
- **Karma :** Modéliser les récompenses et seuils comme données gouvernées (KindMother, WriteIntent) et exposées via l’Opérateur eLearning.
- **Parcours :** Prérequis et ordre de contenus gérés de façon explicite (modèles Prérequis, Section, ordre) pour éviter les ambiguïtés.

---

**Document créé le :** 2026-02-01  
**Version :** 1.0
