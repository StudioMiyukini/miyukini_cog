# Odoo Forum — Index de l'Analyse

## Statut

✅ **Analyse complète à 100% (7/7 documents)**

---

## Documents de l'Analyse

### 1. Logique Métier
📄 [Odoo Forum - Logique Metier Complete.md](./00_logique_metier/Odoo%20Forum%20-%20Logique%20Metier%20Complete.md)

**Contenu :**
- Modèles de données (forum.forum, forum.post, forum.tag)
- Règles métier et contraintes (karma, droits, modération)
- Workflows (questions, réponses, validation, clôture)
- Système karma (gains, droits, rangs), gamification (badges), tags

### 2. Parcours Utilisateur
📄 [Odoo Forum - Parcours Utilisateur Detailles.md](./01_parcours_utilisateur/Odoo%20Forum%20-%20Parcours%20Utilisateur%20Detailles.md)

**Contenu :**
- Personas (Visiteur, Membre, Contributeur, Modérateur, Admin)
- Parcours d'onboarding
- Scénarios d'usage principaux
- Points de friction identifiés
- Recommandations pour Miyukini

### 3. UI/UX
📄 [Odoo Forum - Analyse UI UX.md](./02_ui_ux/Odoo%20Forum%20-%20Analyse%20UI%20UX.md)

**Contenu :**
- Vues front end (liste questions, détail post, New Post)
- Sidebar (tags, tri, Moderation tools)
- Actions sur les posts (vote, favoris, suivi, best answer, menu)
- Configuration backend (Forums, Tags, Ranks, Badges, Close Reasons)
- Recommandations pour Miyukini

### 4. Intégrations Cross-App
📄 [Odoo Forum - Integrations Cross App.md](./03_integrations/Odoo%20Forum%20-%20Integrations%20Cross%20App.md)

**Contenu :**
- Dépendances (website, mail, gamification, portal, helpdesk)
- Flux de données inter-apps
- Mécanismes d'intégration (karma, modération, utilisateurs)
- Recommandations pour Miyukini

### 5. Spécifications Opérateurs Miyukini
📄 [Odoo Forum - Specifications Operateurs Miyukini.md](./04_specifications_miyukini/Odoo%20Forum%20-%20Specifications%20Operateurs%20Miyukini.md)

**Contenu :**
- Opérateurs identifiés (ForumOperator, PostOperator, TagOperator, KarmaOperator, ModerationOperator, ForumUI)
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

### 6. Guide Intégration COG
📄 [Odoo Forum - Guide Integration COG.md](./05_integration_cog/Odoo%20Forum%20-%20Guide%20Integration%20COG.md)

**Contenu :**
- Architecture d'intégration COG
- Patterns WriteIntent et Mandates (post, karma, modération)
- Exemples de code pseudo-Rust
- Gestion des gouvernances

### 7. Guide Implémentation
📄 [Odoo Forum - Guide Implementation.md](./06_guides_implementation/Odoo%20Forum%20-%20Guide%20Implementation.md)

**Contenu :**
- Architecture technique et relation avec MiyuForum (toolkit)
- Spécifications des crates Rust
- Schémas de données (forum, post, tag, karma)
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## Service Miyukini Proposé

**Nom :** `MiyukiniForum` ou `MiyuForum` (toolkit existant) + Opérateurs Forum

**Opérateurs :**
- **ForumOperator** : Gestion des forums (boards)
- **PostOperator** : Gestion des posts (questions, réponses, commentaires)
- **TagOperator** : Gestion des tags
- **KarmaOperator** : Réputation, droits, rangs et badges
- **ModerationOperator** : Validation, signalement, clôture
- **ForumUI** : Interface utilisateur Forum

**Équipe d'Opérateurs :** `ForumService`

**Toolkit existant :** `miyuforum` (board, category, topic, post, readtrack) — utilisé par les opérateurs sous gouvernance.

---

## Source d'Analyse

**Documentation :** Odoo 18.0 / 19.0 — Websites / Forum

**Date d'analyse :** 2026-02-01

---

## Notes

- Application orientée support client et communauté (Q&A, karma, modération).
- Karma partagé possible avec eLearning et autres apps site.
- Modération et TAMR recommandés pour traçabilité et intervention humaine.
- MiyuForum (crate) existe déjà ; les opérateurs Forum consomment ses tools sous COG.
