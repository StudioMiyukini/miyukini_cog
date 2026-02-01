# Odoo eLearning — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **eLearning** (LMS) d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour l'équivalent eLearning
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d'ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **eLearningCourseOperator** | Gestion des cours (création, sections, options, karma, publication) | Opérateur de Service |
| **eLearningContentOperator** | Gestion des contenus (Image, Article, Document, Video, Quiz) | Opérateur de Service |
| **eLearningEnrollmentOperator** | Gestion des inscriptions (open, invitation, paiement), progression | Opérateur de Service |
| **eLearningGroupOperator** | Gestion des groupes de cours et tags (cours, contenu) | Opérateur de Service |
| **eLearningCertificationOperator** | Liaison certifications (Surveys), délivrance, karma | Opérateur de Service |
| **eLearningUI** | Interface catalogue, cours, contenus (back-end et front-end) | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : eLearningService

**Définition :**
> **eLearningService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de formation en ligne (LMS).**

**Composition :**
- eLearningCourseOperator (niveau sécurité 2)
- eLearningContentOperator (niveau sécurité 2)
- eLearningEnrollmentOperator (niveau sécurité 2)
- eLearningGroupOperator (niveau sécurité 1)
- eLearningCertificationOperator (niveau sécurité 2)
- eLearningUI (niveau sécurité 1)

---

## 2. Opérateurs Détaillés

### 2.1 eLearningCourseOperator

**Rôle :** Gestion des cours (création, sections, options, karma, prérequis, publication).

**Capacités :**
- Création / modification / suppression de cours
- Gestion des sections (ordre, libellé)
- Configuration options (Responsible, Website, Show course to, Enroll Policy, Prerequisites, Display : Training / Documentation, Featured Content)
- Configuration communication (Allow Reviews, Forum, New Content Notification, Completion Notification, Contact Attendees)
- Configuration karma (récompenses Review / Finish, seuils Add Review / Add Comment / Vote)
- Publication / dépublication (cours et contenus)
- Liaison certification (Surveys)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création/modification/suppression cours, publication
- **KindMother** : Persistance des cours (WriteIntent)
- **Master Butler** : Permissions création/modification cours
- **WorrySentinel** : Niveau sécurité, isolation par site/entreprise
- **Ever Buddy** : Cycle de vie (prérequis, ordre, dépréciation)

**Contrat d'équipe :**
- Consomme : eLearningContentOperator (contenus du cours), eLearningGroupOperator (tags, groupes), eLearningEnrollmentOperator (inscriptions), eLearningCertificationOperator (certification), MiyuNotify (notifications), MiyuForum (forum)
- Expose : `course.create`, `course.update`, `course.delete`, `course.publish`, `course.unpublish`, `course.sections.manage`

**Mandat de Permission requis :**
- Création cours : Mandat KindMother (WriteIntent) + StrongFather (décision)
- Publication : Mandat StrongFather (décision) + KindMother (WriteIntent pour statut)
- Contact Attendees : Mandat eLearningEnrollmentOperator (liste inscrits) + MiyuNotify (envoi)

### 2.2 eLearningContentOperator

**Rôle :** Gestion des contenus (Image, Article, Document, Video, Quiz ; description, ressources additionnelles, quiz).

**Capacités :**
- Création / modification / suppression de contenus
- Types : Image (upload, Google Drive), Article (page Website), Document (PDF, Google Drive), Video (YouTube, Vimeo, Google Drive), Quiz (questions, réponses, karma)
- Métadonnées : Course, Responsible, Duration, Allow Preview, Allow Download (Document)
- Description, Additional Resources
- Quiz : questions, réponses, Is correct answer, Comment, récompenses karma par tentatives
- Publication / dépublication par contenu
- Statistiques : # Public Views, # Total Views

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision création/modification/suppression contenu, publication
- **KindMother** : Persistance des contenus (WriteIntent)
- **Master Butler** : Permissions sur contenus
- **WorrySentinel** : Niveau sécurité, isolation par cours/site

**Contrat d'équipe :**
- Consomme : eLearningCourseOperator (cours parent), eLearningGroupOperator (content tags), MiyuWeb (Article = page)
- Expose : `content.create`, `content.update`, `content.delete`, `content.publish`, `content.unpublish`, `content.quiz.manage`

**Mandat de Permission requis :**
- Création contenu : Mandat KindMother (WriteIntent) + StrongFather (décision)
- Publication : Mandat StrongFather + KindMother (statut)

### 2.3 eLearningEnrollmentOperator

**Rôle :** Gestion des inscriptions (open, invitation, paiement), progression, liste des inscrits.

**Capacités :**
- Inscription (Open : direct ; On Invitation : lien/email ; On Payment : après achat produit Course)
- Génération lien d'invitation, envoi email (MiyuNotify)
- Liaison produit (Course) ↔ cours pour On Payment (MiyuStore / MiyuBilling)
- Suivi progression (contenus complétés, certification)
- Liste des inscrits (Contact Attendees)
- Karma : attribution (Review, Finish) et vérification seuils (Add Review, Add Comment, Vote)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision inscription (invitation, paiement validé)
- **KindMother** : Persistance inscriptions et progression (WriteIntent)
- **Master Butler** : Permissions inscription (Show course to, Enroll Policy)
- **WorrySentinel** : Niveau sécurité, données personnelles (inscrits)

**Contrat d'équipe :**
- Consomme : eLearningCourseOperator (politique d'accès), MiyuNotify (invitations, notifications), MiyuStore (paiement Course)
- Expose : `enrollment.create`, `enrollment.list`, `enrollment.progress.update`, `enrollment.invite`, `enrollment.karma.grant`

**Mandat de Permission requis :**
- Inscription (Open) : Mandat StrongFather + KindMother
- Inscription (Invitation) : Mandat StrongFather + KindMother + MiyuNotify (envoi)
- Inscription (Payment) : Mandat MiyuStore (paiement validé) + StrongFather + KindMother
- Contact Attendees : Mandat eLearningCourseOperator (responsable cours) + liste inscrits (données sensibles, WorrySentinel)

### 2.4 eLearningGroupOperator

**Rôle :** Gestion des groupes de cours et des tags (cours, contenu).

**Capacités :**
- Création / modification / suppression de Course Groups (nom, Menu Entry, tags avec couleurs)
- Création / modification / suppression de Content Tags
- Affectation tags aux cours et aux contenus
- Filtrage catalogue (All Courses) par groupes / tags

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **StrongFather** : Décision création/modification groupes et tags
- **KindMother** : Persistance groupes et tags (WriteIntent)
- **Master Butler** : Permissions configuration
- **WorrySentinel** : Niveau sécurité bas (données non sensibles)

**Contrat d'équipe :**
- Consomme : aucun (données de référence)
- Expose : `group.create`, `group.update`, `group.list`, `tag.create`, `tag.update`, `tag.list`

**Mandat de Permission requis :**
- Création groupe / tag : Mandat KindMother (WriteIntent) + StrongFather (décision)

### 2.5 eLearningCertificationOperator

**Rôle :** Liaison certifications (Surveys), délivrance, karma.

**Capacités :**
- Liaison cours ↔ survey (certification)
- Déclenchement passage certification (parcours apprenant)
- Réception résultat (Surveys) : succès / échec
- Délivrance certification (enregistrement, traçabilité)
- Attribution karma selon nombre de tentatives (configuré dans contenu/survey)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision délivrance certification
- **KindMother** : Persistance résultats et certifications (WriteIntent)
- **Master Butler** : Permissions passage certification
- **WorrySentinel** : Niveau sécurité (résultats, certifications)

**Contrat d'équipe :**
- Consomme : eLearningCourseOperator (cours, certification liée), eLearningEnrollmentOperator (progression, karma), MiyuSurveys (questionnaire, réponses, score)
- Expose : `certification.link`, `certification.pass`, `certification.deliver`, `certification.karma.grant`

**Mandat de Permission requis :**
- Délivrance certification : Mandat MiyuSurveys (résultat) + StrongFather + KindMother + eLearningEnrollmentOperator (karma)

### 2.6 eLearningUI

**Rôle :** Interface utilisateur (back-end : fiches cours/contenu, configuration ; front-end : catalogue, pages cours/contenus, publication).

**Capacités :**
- Back-end : menus Courses (Courses, Contents), Configuration (Course Groups, Content Tags, Settings) ; formulaires cours (Content, Description, Options, Karma) et contenu (Document, Description, Additional Resources, Quiz) ; Go to Website
- Front-end : All Courses (cartes, filtres Course Groups), page cours (description, inscription, sections/contenus), page contenu (leçon, quiz, certification), switch Published/Unpublished
- Affichage progression, karma, certification

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **StrongFather** : Pas de décision directe ; traduction des intentions vers les Opérateurs
- **Master Butler** : Permissions d'affichage (selon Show course to, inscription)
- **WorrySentinel** : Niveau sécurité affichage (données personnelles masquées si besoin)

**Contrat d'équipe :**
- Consomme : eLearningCourseOperator, eLearningContentOperator, eLearningEnrollmentOperator, eLearningGroupOperator, eLearningCertificationOperator
- Expose : vues back-end et front-end ; pas d'API métier directe (passage par BondingBrother)

**Mandat de Permission requis :**
- Affichage catalogue : Mandat selon Show course to (Everyone / Signed In / Course Attendees)
- Édition cours / contenu : Mandat des Opérateurs concernés (Course, Content)
- Publication : Mandat eLearningCourseOperator / eLearningContentOperator

---

## 3. Contrat d'Équipe eLearningService

### 3.1 Flux autorisés

| De → Vers | Flux | Données |
|-----------|------|---------|
| eLearningUI → eLearningCourseOperator | Création/modification/publication cours | Course, Sections, Options, Karma |
| eLearningUI → eLearningContentOperator | Création/modification/publication contenu | Content, Quiz, Resources |
| eLearningUI → eLearningEnrollmentOperator | Inscription, liste inscrits, progression | Enrollment, Progress, Invite |
| eLearningUI → eLearningGroupOperator | Groupes, tags | Group, Tag |
| eLearningUI → eLearningCertificationOperator | Passage certification, délivrance | Certification, Result |
| eLearningCourseOperator → eLearningContentOperator | Contenus du cours | Content ids |
| eLearningCourseOperator → eLearningEnrollmentOperator | Contact Attendees | Course id, list inscrits |
| eLearningEnrollmentOperator → eLearningCertificationOperator | Karma certification | Karma grant |
| eLearningCertificationOperator → MiyuSurveys | Questionnaire, résultat | Survey id, Answers, Score |

### 3.2 Niveaux de sécurité

- **eLearningCourseOperator, eLearningContentOperator, eLearningEnrollmentOperator, eLearningCertificationOperator** : Niveau 2 (Sensitive) — données formation, inscriptions, certifications
- **eLearningGroupOperator, eLearningUI** : Niveau 1 (Standard) — configuration et affichage

### 3.3 Règles de collaboration

- Aucune communication directe entre Opérateurs ; passage obligatoire par BondingBrother
- Toute inscription (open, invitation, paiement) nécessite un Mandat StrongFather + KindMother
- Publication cours/contenu : Mandat StrongFather + KindMother (statut)
- Contact Attendees : Mandat eLearningEnrollmentOperator (liste) + MiyuNotify (envoi) ; pas d'exposition des emails sans Mandat
- Certification : Mandat MiyuSurveys (résultat) + StrongFather + KindMother pour délivrance

---

## 4. Intégrations Externes

| Service externe | Opérateur consommateur | Rôle |
|-----------------|-------------------------|------|
| **MiyuWeb** | eLearningUI, eLearningContentOperator | Pages catalogue, cours, contenus ; Article = page |
| **MiyuSurveys** | eLearningCertificationOperator | Questionnaire certification, résultat, score |
| **MiyuStore / MiyuBilling** | eLearningEnrollmentOperator | Produit Course, paiement, inscription après achat |
| **MiyuNotify** | eLearningCourseOperator, eLearningEnrollmentOperator | New Content / Completion Notification, Contact Attendees, invitations |
| **MiyuForum** | eLearningCourseOperator | Forum dédié par cours |

---

**Document créé le :** 2026-02-01  
**Version :** 1.0
