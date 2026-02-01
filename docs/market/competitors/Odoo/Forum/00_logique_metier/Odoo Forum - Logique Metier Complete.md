# Odoo Forum — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Forum** d'Odoo (versions 18.0 / 19.0), à partir de la documentation officielle et des usages standards. Il identifie les modèles de données, règles métier, workflows, karma et mécanismes de gouvernance pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18.0/19.0 — Websites / Forum

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (forum.forum, forum.post, forum.tag)
- Règles métier et contraintes (karma, droits, modération)
- Workflows et états (questions, réponses, validation, clôture)
- Système de karma (gains, droits, rangs)
- Gamification (rangs, badges)
- Tags et filtrage
- Modération (validation, signalement, clôture)

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `forum.forum` (Forum)

**Rôle :** Représente un **forum** — conteneur de posts avec configuration (mode, tri, confidentialité, karma).

**Champs et paramètres clés :**

#### Identification et affichage
- `name` : Nom du forum (obligatoire)
- Mode : **Questions** (marquage « meilleure réponse », statut résolu) ou **Discussions** (sans meilleure réponse)
- **Default Sort** : Tri par défaut des questions
  - Newest : date de publication
  - Last Updated : dernière activité (réponses et commentaires)
  - Most Voted : nombre de votes
  - Relevance : pertinence (formule)
  - Answered : probabilité d’être répondu (formule)

#### Confidentialité
- **Privacy** : Public (tous) / Signed In (utilisateurs connectés) / Some users (groupe autorisé)

#### Karma
- **Karma gains** : Points gagnés/perdus par interaction (configurable par forum)
- **Karma-related rights** : Seuils de karma pour chaque droit (poser question, répondre, voter, modérer, etc.)

**Règles métier :**
- Un seul forum peut être en mode Questions ou Discussions.
- Les points de karma sont partagés entre tous les forums, cours et outils du même site Odoo.
- Les nouveaux utilisateurs reçoivent **3 points** après validation de l’email.

---

### 1.2 Modèle `forum.post` (Post / Question / Réponse)

**Rôle :** Représente un **post** — question, réponse ou commentaire dans un forum.

**Types de post :**
- **Question** : Post racine (sujet)
- **Réponse** : Réponse à une question (une seule réponse par utilisateur et par question ; commentaires multiples autorisés)
- **Commentaire** : Commentaire sous une question ou une réponse

**Champs et concepts clés :**
- Titre (questions)
- Description / contenu (HTML)
- Auteur (`res.users` ou visiteur)
- Forum parent (`forum.forum`)
- Post parent (question si réponse/commentaire)
- **Best answer** : En mode Questions, une réponse peut être marquée comme « meilleure réponse » (question alors « résolue »)
- Votes (up/down)
- Favoris, suivi (notifications)
- Tags (sur les questions)
- État : brouillon, en attente de validation, publié, fermé, supprimé, signalé

**Règles métier :**
- **Une seule réponse par utilisateur** par question (les autres interventions sont des commentaires).
- En mode Questions : une seule réponse peut être « meilleure » par question.
- Questions en attente de validation si l’utilisateur n’a pas le karma requis ; **une seule question en attente** par utilisateur et par forum.
- Fermeture avec motif (configuration Close Reasons) ; motifs « offensif » ou « spam » entraînent -100 karma pour l’auteur.
- Signalement (flag) : examen modérateur ; si confirmé offensif, -100 karma et masquage pour les non-moderateurs.

---

### 1.3 Modèle `forum.tag`

**Rôle :** Catégoriser et filtrer les posts (questions).

**Caractéristiques :**
- Lié à un forum (`forum.forum`)
- Utilisé pour le filtrage côté front (sidebar « Tags », « View all »)
- Création de nouveaux tags à la publication si l’utilisateur a le karma requis (droit « Create new tags », défaut 30)
- Gestion des tags : Website ‣ Configuration ‣ Forum ‣ Tags (vues custom possibles car non exposées par défaut)

**Règles métier :**
- Jusqu’à **cinq tags** par question (documentation utilisateur).
- Modification des tags d’une question : droit « Change question tags » (défaut 75 karma).

---

### 1.4 Karma — Gains (configurables)

| Interaction              | Description                          | Karma par défaut |
|--------------------------|--------------------------------------|------------------|
| Asking a question        | Publier une question                 | +2               |
| Question upvoted         | Un autre vote pour votre question    | +5               |
| Question downvoted       | Un autre vote contre votre question  | -2               |
| Answer upvoted           | Un autre vote pour votre réponse     | +10              |
| Answer downvoted         | Un autre vote contre votre réponse   | -2               |
| Accepting an answer      | Vous marquez une réponse comme best  | +2               |
| Answer accepted          | Une de vos réponses marquée best     | +15              |
| Answer flagged           | Question/réponse signalée offensif   | -100             |

---

### 1.5 Karma — Droits (seuils configurables)

| Fonctionnalité                          | Description                    | Karma par défaut |
|----------------------------------------|--------------------------------|------------------|
| Ask questions                           | Poster des questions           | 3                |
| Answer questions                        | Poster des réponses            | 3                |
| Upvote                                  | Voter pour                     | 5                |
| Downvote                                | Voter contre                   | 50               |
| Edit own posts                          | Modifier ses posts            | 1                |
| Edit all posts                          | Modifier tous les posts        | 300              |
| Close own posts                         | Fermer ses questions          | 100              |
| Close all posts                         | Fermer toute question          | 500              |
| Delete own posts                        | Supprimer ses posts            | 500              |
| Delete all posts                        | Supprimer tous les posts       | 1000             |
| Nofollow links                          | Liens en nofollow (SEO)        | 500              |
| Accept an answer on own questions      | Marquer best sur ses questions | 20               |
| Accept an answer to all questions      | Marquer best sur toute question| 500              |
| Editor: image and links                 | Images et liens dans les posts | 30               |
| Comment own posts                       | Commenter ses posts            | 1                |
| Comment all posts                       | Commenter tout post            | 1                |
| Convert own answers/comments            | Convertir réponse ↔ commentaire| 50               |
| Convert all answers/comments            | Idem sur tous les posts        | 500              |
| Unlink own comments                     | Supprimer ses commentaires     | 50               |
| Unlink all comments                     | Supprimer tout commentaire     | 500              |
| Ask questions without validation        | Pas de validation préalable     | 100              |
| Flag a post as offensive                | Signaler offensif              | 500              |
| Moderate posts                          | Accès modération               | 1000             |
| Change question tags                    | Modifier les tags              | 75               |
| Create new tags                         | Créer de nouveaux tags         | 30               |
| Display detailed user biography         | Bio détaillée au survol        | 750              |

---

### 1.6 Rangs et Badges (Gamification)

**Rangs :**
- Basés sur le **total de karma**.
- Configuration : Website ‣ Configuration ‣ Forum ‣ Ranks.
- Champs : nom, karma requis, description, message de motivation, image.

**Badges :**
- Configuration : Website ‣ Configuration ‣ Forum ‣ Badges.
- **Attribution manuelle** : qui peut attribuer (tous, liste d’utilisateurs, détenteurs de certains badges), limite mensuelle optionnelle.
- **Attribution automatique** : via défis (challenges) ; niveau optionnel (Bronze, Silver, Gold).

---

### 1.7 Modération

**To Validate :**
- Liste des questions/réponses en attente de validation (utilisateur sans karma « Ask questions without validation »).
- Une question en attente par utilisateur et par forum ; l’utilisateur ne peut pas poster d’autres questions tant qu’elle n’est pas validée.

**Flagged :**
- Posts signalés comme offensifs.
- Actions : Accepter (lever le flag) ou Marquer comme offensif (raison, -100 karma, masquage pour non-moderateurs).

**Close Reasons :**
- Website ‣ Configuration ‣ Forum ‣ Close Reasons.
- Type : Basic (fermeture) ou Offensive (posts signalés).
- Motifs « Spam or advertising » ou « Contains offensive or malicious remarks » : -100 karma à l’auteur.

**Actions bulk (backend) :**
- Website ‣ Configuration ‣ Forum ‣ Forums ‣ [Forum] ‣ Posts ‣ Actions : Export, Archive, Unarchive, Delete.

---

### 1.8 Intégration Helpdesk

- Depuis un post, lien possible vers un **ticket Helpdesk** associé (vue « View related Helpdesk ticket » dans le menu du post).

---

## 2. Workflows

### 2.1 Publication d’une question

1. Utilisateur clique « New Post ».
2. Saisie titre, description, tags (≤ 5).
3. Si karma &lt; seuil « Ask questions without validation » → post en **attente de validation** (une seule par utilisateur/forum).
4. Sinon (ou après validation modérateur) → post **publié**.
5. Notifications aux followers du forum / abonnés (selon implémentation).

### 2.2 Réponse et meilleure réponse (mode Questions)

1. Utilisateur répond (une réponse par utilisateur et par question ; sinon commentaire).
2. Autres utilisateurs peuvent voter (up/down) si karma suffisant.
3. Auteur de la question (ou modérateur si droit « Accept an answer to all questions ») peut marquer **une** réponse comme **best**.
4. Question affichée comme **résolue** (solved).

### 2.3 Modération

1. **Validation** : Modérateur valide les posts en attente.
2. **Signalement** : Utilisateur signale (flag) → Modérateur accepte ou confirme offensif (raison, -100 karma, masquage).
3. **Fermeture** : Modérateur ou auteur (si droit) ferme avec un motif (Close Reason) ; motifs offensifs/spam → -100 karma.

---

## 3. Synthèse pour Miyukini

- **Forum** = conteneur (équivalent board/category dans MiyuForum).
- **Post** = topic (sujet) + réponses/commentaires (posts).
- **Karma** = règles métier à porter en gouvernance (StrongFather / Master Butler) et éventuellement en WriteIntent pour historiser.
- **Tags, rangs, badges** = modèles annexes à prévoir (tags déjà présents dans MiyuForum).
- **Modération** = workflows TAMR / WorrySentinel (validation, signalement, clôture) et droits via Master Butler.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
