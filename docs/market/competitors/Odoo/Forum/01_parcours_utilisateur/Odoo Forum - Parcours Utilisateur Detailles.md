# Odoo Forum — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **Forum** d'Odoo, identifiant les personas, scénarios d'usage, processus d'onboarding et points de friction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18.0/19.0 — Forum

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

### 1.1 Visiteur (Non connecté)

**Profil :**
- Rôle : Consultation uniquement (si forum en mode Public).
- Responsabilités : Lire questions et réponses, parcourir par tags.

**Besoins :**
- Accès lecture au forum public.
- Pas de vote, pas de post, pas de commentaire.

**Permissions :**
- Forum en confidentialité « Public » : lecture seule.
- Forum « Signed In » ou « Some users » : pas d’accès au contenu (ou redirection login).

### 1.2 Membre (Utilisateur connecté — faible karma)

**Profil :**
- Rôle : Participation limitée selon karma.
- Responsabilités : Poser des questions (souvent en validation), répondre, commenter, voter quand seuils atteints.

**Besoins :**
- Comprendre les seuils de karma (droits affichés ou expliqués).
- Savoir qu’une question peut être « en attente de validation » (une seule à la fois).
- Recevoir des points (email validé : +3, interactions : gains karma).

**Permissions :**
- Ask questions / Answer questions (défaut 3 karma).
- Edit own posts (1), Comment (1).
- Upvote à partir de 5 karma.
- Pas de downvote, modération, ni fermeture tant que seuils non atteints.

### 1.3 Contributeur (Karma moyen à élevé)

**Profil :**
- Rôle : Répondre, voter, marquer meilleure réponse (sur ses questions), éditer ses posts, fermer ses questions.
- Responsabilités : Qualité des réponses, respect des tags, utilisation des votes.

**Besoins :**
- Tri et filtres (tags, Last Updated, Most Voted, etc.).
- Favoris et suivi (notifications).
- Création de tags (si karma suffisant).
- Affichage bio détaillée (si 750 karma).

**Permissions :**
- Droits progressifs selon karma (downvote 50, close own 100, accept answer on own 20, nofollow 500, etc.).

### 1.4 Modérateur

**Profil :**
- Rôle : Validation, signalements, fermeture, suppression, accès aux outils de modération.
- Responsabilités : Valider les posts en attente, traiter les signalements, fermer avec motif, gérer archives/suppressions.

**Besoins :**
- Sidebar « Moderation tools » : To Validate, Flagged, Closed.
- Liste des posts à valider / signalés / fermés.
- Choix du motif de fermeture (Close Reasons) et raison offensif.
- Actions bulk (export, archive, unarchive, delete) depuis le backend.

**Permissions :**
- Moderate posts (défaut 1000 karma) ou rôle dédié.
- Edit/close/delete all posts, unlink all comments, accept answer on all questions.

### 1.5 Administrateur Forum / Site

**Profil :**
- Rôle : Configuration des forums, karma, rangs, badges, tags, close reasons.
- Responsabilités : Créer des forums, définir mode (Questions/Discussions), confidentialité, seuils karma, rangs et badges.

**Besoins :**
- Website ‣ Configuration ‣ Forum : Forums, Tags, Ranks, Badges, Close Reasons.
- Onglets Karma Gains et Karma Related Rights par forum.
- Suivi karma (Settings ‣ Gamification Tools ‣ Karma Tracking en mode développeur).

**Permissions :**
- Accès configuration site / forum (droits backend Odoo).

---

## 2. Parcours d'Onboarding

### 2.1 Nouveau visiteur — Première question

**Scénario :**
1. Arrivée sur le forum (public ou après connexion).
2. Clic « New Post ».
3. Saisie titre, description, tags (≤ 5).
4. Clic « Post Your Question ».
5. Si karma &lt; 100 : message « en attente de validation », une seule question en attente.
6. Email de validation non fait : incitation à valider l’email (+3 karma).
7. Après validation par modérateur (ou karma ≥ 100) : question visible.

**Points d’aide :**
- Message clair « Your question is pending validation ».
- Rappel « Validate your email to earn 3 karma ».
- Limite « one pending question » expliquée.

### 2.2 Premier passage en modérateur

**Scénario :**
1. Accès à la section Moderation tools (sidebar).
2. « To Validate » : liste des questions/réponses en attente.
3. Clic sur un post → lecture → Validate ou rejet (selon interface).
4. « Flagged » : liste des signalements → Accept (lever flag) ou Mark as offensive (raison, -100 karma).
5. « Closed » : consultation des posts fermés, possibilité Reopen ou Delete.

**Points d’aide :**
- Documentation interne sur Close Reasons (Basic vs Offensive).
- Attention aux droits sensibles (Edit all, Delete all, Moderate) rappelée dans la doc Odoo.

---

## 3. Scénarios d'Usage Principaux

### 3.1 Poser une question et recevoir une réponse

**Acteur :** Membre

**Scénario :**
1. New Post → Titre, Description, Tags (≤ 5) → Post Your Question.
2. Question publiée (ou en attente).
3. Suivi (bell) pour recevoir les notifications.
4. Lecture des réponses ; vote (up/down) si karma suffisant.
5. En mode Questions : marquer une réponse comme « best » (si karma ≥ 20 pour ses questions).
6. Question affichée comme résolue.

**Points de friction Odoo :**
- Une seule réponse par utilisateur et par question (les autres en commentaires) peut surprendre.
- Seuil « Ask questions without validation » (100) peu visible pour les nouveaux.

**Recommandations Miyukini :**
- Exposer clairement les droits (karma ou équivalent) dans l’UI.
- Notifications explicites (nouvelle réponse, best answer, modération).

### 3.2 Répondre et gagner en réputation

**Acteur :** Contributeur

**Scénario :**
1. Parcourir les questions (tri : Last Updated, Most Voted, etc.).
2. Filtrer par tag.
3. Ouvrir une question → Répondre (une réponse par utilisateur ; sinon commenter).
4. Recevoir des votes : +10 si answer upvoted, +15 si answer accepted.
5. Monter en karma → débloquer droits (downvote, close own, create tags, etc.).
6. Consulter rangs et badges (profil, sidebar).

**Points de friction Odoo :**
- Règles « one answer per user per question » et « comments unlimited » à expliquer.
- Karma partagé entre forum, eLearning, etc. : comportement à clarifier pour l’utilisateur.

**Recommandations Miyukini :**
- Aide contextuelle sur les règles de réponse vs commentaire.
- Tableau ou profil « Droits débloqués » en fonction du karma (ou équivalent).

### 3.3 Modérer (validation, signalements, fermeture)

**Acteur :** Modérateur

**Scénario :**
1. To Validate : ouvrir chaque post en attente → Valider ou rejeter.
2. Flagged : pour chaque signalement → Accepter (lever flag) ou Marquer comme offensif (raison, -100 karma).
3. Fermeture : ouvrir une question → … → Close → Choisir Close Reason → Confirmer.
4. Si motif offensif/spam : -100 karma à l’auteur, post masqué aux non-moderateurs.
5. Backend : Forum ‣ Posts ‣ Actions (Export, Archive, Unarchive, Delete).

**Points de friction Odoo :**
- Risque de donner des droits trop larges (Edit/Delete all) à toute personne atteignant le karma.
- Close Reasons (Basic vs Offensive) à configurer correctement.

**Recommandations Miyukini :**
- Séparer « réputation » (karma) et « rôle modérateur » (attribution explicite par gouvernance).
- Audit trail des actions de modération (TAMR / WorrySentinel).

---

## 4. Points de Friction Identifiés

| Friction | Description | Piste Miyukini |
|----------|-------------|-----------------|
| Une réponse par utilisateur | Règle peu évidente pour les nouveaux | Aide contextuelle, message en cas de « deuxième réponse » (rediriger vers commentaire) |
| Question en attente unique | Blocage si plusieurs questions avant validation | Message clair + possibilité de retirer/éditer la question en attente |
| Karma partagé multi-apps | Comportement non évident | Documenter ou limiter le scope (forum seul) selon produit |
| Droits sensibles liés au karma | Edit/Delete all à 300/1000 karma | Rôle modérateur explicite (StrongFather / Master Butler) plutôt que seuil unique |
| Tags non exposés par défaut | Gestion forum.tag en backend | Exposer une vue Tags dans l’admin ou l’équivalent Miyukini |
| Modération et SEO | Nofollow selon karma | Conserver la logique « niveau de confiance » pour les liens (WorrySentinel / politique SEO) |

---

## 5. Recommandations pour Miyukini

- **Personas** : Aligner Visiteur, Membre, Contributeur, Modérateur, Admin sur les rôles et Mandats de Permission (StrongFather, Master Butler).
- **Onboarding** : Premier post guidé (titres, tags, attente de validation) et communication claire sur les seuils (équivalent karma).
- **Karma / réputation** : Modéliser en gouvernance (décisions StrongFather, historisation KindMother) et exposant les « droits débloqués » dans l’UI.
- **Modération** : TAMR pour intervention humaine (validation, signalement, fermeture) ; WorrySentinel pour niveaux de confiance ; audit des actions.
- **Équipe d’Opérateurs** : ForumService avec ForumOperator, PostOperator, TagOperator, ModerationOperator, ForumUI (voir document Spécifications Opérateurs Miyukini).

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
