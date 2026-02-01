# Odoo eLearning — Index de l'Analyse

## Statut

✅ **Analyse complète à 100% (7/7 documents)**

---

## Documents de l'Analyse

### 1. Logique Métier
📄 [Odoo eLearning - Logique Metier Complete.md](./00_logique_metier/Odoo%20eLearning%20-%20Logique%20Metier%20Complete.md)

**Contenu :**
- Modèles conceptuels (Cours, Sections, Contenus, Course Groups, Tags, Certification)
- Règles métier (publication, prérequis, inscription Open/Invitation/Payment, karma)
- Workflows (création cours → contenu → publication ; parcours apprenant ; certification)
- Options (Show course to, Enroll Policy, Display Training/Documentation)
- Intégrations (Surveys, Website, eCommerce, Forum, Mailing)

### 2. Parcours Utilisateur
📄 [Odoo eLearning - Parcours Utilisateur Detailles.md](./01_parcours_utilisateur/Odoo%20eLearning%20-%20Parcours%20Utilisateur%20Detailles.md)

**Contenu :**
- Personas (Administrateur eLearning, Formateur, Apprenant, Invité, Marketing)
- Parcours d'onboarding (premier déploiement, premier cours, premier apprenant)
- Scénarios d'usage (création/publication cours, inscription invitation/paiement, prérequis, certification/karma, mailings)
- Points de friction identifiés
- Recommandations pour Miyukini

### 3. UI/UX
📄 [Odoo eLearning - Analyse UI UX.md](./02_ui_ux/Odoo%20eLearning%20-%20Analyse%20UI%20UX.md)

**Contenu :**
- Structure back-end (menus, fiches cours/contenu, onglets Content, Description, Options, Karma)
- Structure front-end (All Courses, page cours, page contenu, publication)
- Composants (cartes cours, sections, contenus, quiz, certification)
- Patterns de navigation et actions (Go to Website, switch Published)
- Recommandations pour Miyukini

### 4. Intégrations Cross-App
📄 [Odoo eLearning - Integrations Cross App.md](./03_integrations/Odoo%20eLearning%20-%20Integrations%20Cross%20App.md)

**Contenu :**
- Dépendances (Website, Surveys, eCommerce, Mailing, Forum)
- Flux de données (catalogue, certifications, Paid Courses, Contact Attendees, forum)
- Mécanismes d'intégration détaillés
- Recommandations Miyukini (MiyuWeb, MiyuSurveys, MiyuStore, MiyuNotify, MiyuForum)

### 5. Spécifications Opérateurs Miyukini
📄 [Odoo eLearning - Specifications Operateurs Miyukini.md](./04_specifications_miyukini/Odoo%20eLearning%20-%20Specifications%20Operateurs%20Miyukini.md)

**Contenu :**
- Opérateurs (eLearningCourseOperator, eLearningContentOperator, eLearningEnrollmentOperator, eLearningGroupOperator, eLearningCertificationOperator, eLearningUI)
- Contrat d'équipe eLearningService
- Mandats de Permission et niveaux de sécurité
- Intégrations externes (MiyuWeb, MiyuSurveys, MiyuStore, MiyuNotify, MiyuForum)

### 6. Guide Intégration COG
📄 [Odoo eLearning - Guide Integration COG.md](./05_integration_cog/Odoo%20eLearning%20-%20Guide%20Integration%20COG.md)

**Contenu :**
- Architecture d'intégration COG
- Patterns WriteIntent et Mandates (cours, publication, inscription invitation/payment, certification)
- Exemples de code pseudo-Rust
- Vérification d'accès (Show course to)

### 7. Guide Implémentation
📄 [Odoo eLearning - Guide Implementation.md](./06_guides_implementation/Odoo%20eLearning%20-%20Guide%20Implementation.md)

**Contenu :**
- Architecture technique (crates proposées : miyuelearning-course, content, enrollment, group, certification, ui)
- Schémas de données (Course, Section, Content, Enrollment, Certification, Group, Tag)
- API et contrats par Opérateur
- Plan de développement par phases (MVP → Complet)
- Bornage fonctionnel et critères d'acceptation

---

## Service Miyukini Proposé

**Nom :** `Miyukini eLearning` ou `Miyu eLearning`

**Opérateurs :**
- **eLearningCourseOperator** : Gestion des cours (sections, options, karma, publication)
- **eLearningContentOperator** : Gestion des contenus (Image, Article, Document, Video, Quiz)
- **eLearningEnrollmentOperator** : Inscriptions (open, invitation, paiement), progression, karma
- **eLearningGroupOperator** : Course Groups et tags (cours, contenu)
- **eLearningCertificationOperator** : Liaison certifications (MiyuSurveys), délivrance
- **eLearningUI** : Interface back-end et front-end (catalogue, cours, contenus)

**Équipe d'Opérateurs :** `eLearningService`

---

## Source d'Analyse

**Documentation :** Odoo 18.0/19.0 — Websites / eLearning

**Version analysée :** Odoo 18.0 / 19.0

**Date d'analyse :** 2026-02-01

---

## Notes

- Application LMS (Learning Management System) avec cours, contenus, sections, certifications et gamification (karma)
- Intégrations : Website (pages), Surveys (certifications), eCommerce (Paid Courses), Mailing (Contact Attendees, notifications), Forum (forum dédié par cours)
- Publication gérée sur le front-end (cours et contenus séparément) ; prérequis et ordre (Training vs Documentation) à modéliser explicitement
- Correspondance Miyukini : Miyu eLearning / Miyukini eLearning (eLearningService)
