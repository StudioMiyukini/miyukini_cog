# Odoo Forum — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Forum** d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Documentation Odoo 18.0/19.0 — Forum et catalogue apps

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres apps Odoo (website, mail, gamification, portal, helpdesk)
- Flux de données inter-apps
- Mécanismes d'intégration (karma, modération, utilisateurs)
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Modules requis (typiques Forum Odoo)

- **website** : Pages web, menu, structure du site, publication du forum.
- **mail** : Chatter, notifications, suivi des posts (followers), messages sur les posts.
- **gamification** (ou équivalent) : Karma, suivi des points, rangs et badges (challenges).
- **portal** (ou **auth_signup**) : Inscription, connexion, validation email (+3 karma).
- **web** : Framework web front et back.

### 1.2 Modules optionnels

- **helpdesk** : Lien post ↔ ticket Helpdesk (menu « View related Helpdesk ticket » sur un post).
- **rating** : Possible extension pour notation (si utilisée sur les réponses).
- **digest** : Résumés email (activité forum).
- **website_forum** (nom possible selon version) : App Forum elle-même, dépend de website + mail + gamification.

---

## 2. Intégrations Détaillées

### 2.1 Website

**Flux :**
- Forum exposé comme **page(s)** du site (URL dédiée, menu).
- Contrôle d’accès selon **Privacy** du forum (Public / Signed In / Some users).
- Thème et mise en page du site appliqués au forum.

**Mécanismes :**
- Contrôleurs web pour liste questions, détail post, New Post, vote, best answer, commentaires, modération.
- Assets (CSS/JS) pour sidebar, tri, tags, actions sur les posts.
- SEO : nofollow sur liens des utilisateurs sous seuil karma.

**Recommandations Miyukini :**
- MiyuWeb ou équivalent pour exposer le forum (pages, routes, menus).
- Façade Publique Gouvernée si forum public ; Mandat Public d’Accès ou Visa pour utilisateurs identifiés.

### 2.2 Mail (Messagerie / Notifications)

**Flux :**
- **Followers** sur les posts : abonnés notifiés (email / in-app) à nouvelles réponses, commentaires, best answer.
- Notifications : nouvelle réponse, question validée, post signalé (modérateurs), etc.
- Chatter possible sur les posts (backend) pour historique et pièces jointes.

**Champs / comportements :**
- `message_ids` / `message_follower_ids` sur `forum.post` (si modèle mail.thread).
- Envoi d’emails selon préférences utilisateur (notifications forum).

**Recommandations Miyukini :**
- MiyuNotify pour notifications (nouvelle réponse, validation, modération).
- BondingBrother pour traduire « suivre un post » en Mandat + abonnement notifications.

### 2.3 Gamification (Karma, Rangs, Badges)

**Flux :**
- **Karma** : un seul compteur par utilisateur (ou par site) ; partagé entre Forum, eLearning, etc.
- Chaque interaction (question, vote, best answer, flag) → gain/perte de karma (configurable par forum).
- **Rangs** : basés sur le total karma (seuils configurables).
- **Badges** : attribués manuellement (qui peut attribuer, limite mensuelle) ou automatiquement via **challenges**.

**Mécanismes :**
- Modèle (ex. `gamification.karma` ou équivalent) mis à jour à chaque action.
- Droits forum (Ask, Answer, Upvote, Moderate, etc.) = seuils karma lus à l’exécution.
- Karma Tracking : Settings ‣ Gamification Tools ‣ Karma (mode développeur) pour suivi et ajustement manuel.

**Recommandations Miyukini :**
- Modéliser « réputation » (équivalent karma) en gouvernance : StrongFather pour décisions, KindMother pour historiser les changements.
- Master Butler pour droits « débloqués » selon niveau de réputation.
- Ever Buddy pour cycle de vie (création post, validation, clôture) et historique.

### 2.4 Portal / Auth

**Flux :**
- **Inscription** : auth_signup / portail → création compte.
- **Validation email** : +3 karma à la première validation.
- **Connexion** : nécessaire pour poster, voter, commenter (sauf si forum public en lecture seule).
- **Profil** : affichage karma, bio, badges (selon droits « Display detailed user biography » et « View profile »).

**Recommandations Miyukini :**
- MiyuAuth / MiyuPortal pour identité et accès ; COG Hébergeur pour session.
- Passeport / Visa si forum fédéré (visite inter-COG).

### 2.5 Helpdesk

**Flux :**
- Lien **post ↔ ticket** : un post peut être associé à un ticket Helpdesk.
- Depuis un post : menu (…) → **View related Helpdesk ticket**.
- Usage : support client (question posée sur le forum, ticket créé en parallèle ou a posteriori).

**Mécanismes :**
- Champ relation (ex. `helpdesk_ticket_id` sur `forum.post` ou lien inverse sur ticket).
- Droits : visibilité du lien selon droits Helpdesk et Forum.

**Recommandations Miyukini :**
- Si MiyuHelpdesk existe : relation explicite post ↔ ticket (WriteIntent KindMother, décision StrongFather pour lier/délier).
- BondingBrother pour intention « ouvrir le ticket lié » depuis l’UI Forum.

---

## 3. Synthèse des Flux

| Source | Cible | Données / Événement |
|--------|--------|----------------------|
| Forum | Website | Pages, URLs, menu, thème |
| Forum | Mail | Followers, notifications (réponse, validation, signalement) |
| Forum | Gamification | Karma (gains/pertes), rangs, badges, challenges |
| Forum | Portal / Auth | Utilisateur connecté, validation email, profil |
| Forum | Helpdesk | Lien post ↔ ticket |
| Website | Forum | Paramètres site (ex. karma pour voir profil) |
| Gamification | Forum | Droits (seuils karma) à l’exécution |

---

## 4. APIs et Hooks (orientations)

- **Contrôleurs** : routes pour liste, détail, create post, vote, best answer, comment, flag, close, validate (modération).
- **Modèles** : `forum.forum`, `forum.post`, `forum.tag` ; héritage `mail.thread` si chatter ; appels aux modèles gamification pour karma.
- **Hooks** : après création post → calcul karma ; après validation → notification ; après flag/close → karma -100 et masquage.
- **Security** : règles d’accès (groups / record rules) selon privacy forum et karma.

---

## 5. Recommandations pour Miyukini

- **Équipe ForumService** : ForumOperator, PostOperator, TagOperator, KarmaOperator (ou intégré dans PostOperator), ModerationOperator, ForumUI.
- **Contrats d’équipe** : ForumUI ↔ MiyuWeb (pages), ForumOperator ↔ MiyuNotify (notifications), ForumOperator ↔ MiyuAuth/MiyuPortal (identité), ForumOperator ↔ MiyuHelpdesk (lien ticket).
- **Mandats** : Lecture publique (Façade) ; écriture et modération sous Mandat de Permission (StrongFather, Master Butler, WorrySentinel pour modération).
- **Karma / réputation** : Cœur métier Forum ; gouvernance explicite (StrongFather + KindMother) plutôt qu’un module séparé opaque.
- **Modération** : TAMR pour validation, signalement, fermeture ; audit et traçabilité (Maintenance explicable).

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
