# Odoo Forum — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Forum** d'Odoo (versions 18.0 / 19.0), à partir de la documentation et des usages standards. Il identifie les composants d'interface, patterns de navigation, formulaires et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18.0 — Forum (front end et configuration)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Vues front end (liste de questions, détail post, formulaire New Post)
- Sidebar (tags, tri, modération)
- Actions sur les posts (vote, favoris, suivi, best answer, menu …)
- Configuration backend (Forums, Tags, Ranks, Badges, Close Reasons)
- Design responsive et accessibilité (orientations doc Odoo)

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Vues Front End (Site Web)

### 1.1 Liste des questions (forum principal)

**Caractéristiques :**
- Affichage des questions (titre, extrait, auteur, date, votes, réponses, résolu si mode Questions).
- Tri utilisateur : Newest, Last Updated, Most Voted, Relevance, Answered (plus options côté front : total replies, total views, last activity).
- Filtrage par **tags** (sidebar « Tags », lien « View all » pour tous les tags).
- Bouton **New Post** pour créer une question.

**Colonnes / champs visibles (type liste ou cartes) :**
- Titre (lien vers le post).
- Auteur (avatar / nom).
- Date (publication ou dernière mise à jour).
- Nombre de réponses.
- Nombre de vues (si implémenté).
- Score de votes (up - down).
- Indicateur « résolu » (check) en mode Questions.
- Tags (badges ou liens).

### 1.2 Détail d’un post (question + réponses + commentaires)

**Structure :**
- **En-tête** : Titre, auteur, date, tags, actions (favori ☆, suivi 🔔, vote ▲▼, partage, menu …).
- **Corps** : Contenu de la question (HTML).
- **Réponses** : Liste ordonnée (ex. par votes ou date) ; une réponse peut être marquée **Best** (✔) en mode Questions.
- **Commentaires** : Sous la question ou sous chaque réponse (icône 💬).
- **Zone de réponse** : Formulaire « Answer » (une réponse par utilisateur ; sinon commentaire).
- **Menu contextuel (…)** : Edit, Close, Delete, Flag, Convert (comment ↔ answer), View related Helpdesk ticket.

**Actions par icône :**
- ☆ : Marquer en favori.
- 🔔 : Suivre et recevoir les notifications.
- ▲ / ▼ : Vote pour / contre (selon karma).
- ✔ : Marquer comme meilleure réponse (mode Questions, selon karma).
- 💬 : Ouvrir / afficher les commentaires.
- Partage : Facebook, Twitter, LinkedIn.
- … : Edit, Close, Delete, Flag, Convert, View Helpdesk ticket.

### 1.3 Formulaire « New Post »

**Champs :**
- **Title** : Titre de la question (obligatoire).
- **Description** : Contenu riche (éditeur HTML ; images et liens si karma suffisant).
- **Tags** : Jusqu’à 5 tags ; création possible si droit « Create new tags » (ex. 30 karma).
- Bouton **Post Your Question**.

**Comportement :**
- Si karma &lt; 100 : message « en attente de validation » après envoi.
- Validation email : incitation si non validée (+3 karma après validation).

---

## 2. Sidebar et Navigation

### 2.1 Section Tags

- Liste de tags (souvent par popularité ou usage).
- Clic sur un tag → filtre les questions avec ce tag.
- Lien **View all** → page ou modal listant tous les tags.

### 2.2 Tri (sort)

- Options utilisateur documentées : Newest, Last Updated, Most Voted, Relevance, Answered.
- Côté front : possibilité d’autres tris (total replies, total views, last activity).

### 2.3 Moderation tools (modérateurs)

- **To Validate** : Accès aux questions/réponses en attente.
- **Flagged** : Liste des posts signalés ; actions Accept / Mark as offensive (raison).
- **Closed** : Liste des questions fermées ; actions Reopen, Delete.

### 2.4 Profil utilisateur

- Au survol avatar / nom : popover avec karma, biographie, badges (si droit « Display detailed user biography », ex. 750 karma).
- Par défaut : 150 karma pour voir le profil d’un autre utilisateur (configurable au niveau site).

---

## 3. Configuration Backend (Website ‣ Configuration ‣ Forum)

### 3.1 Forum : Forums

- **Vue liste** : Liste des forums (nom, mode, confidentialité, etc.).
- **Formulaire forum** :
  - Nom, Mode (Questions / Discussions), Default Sort, Privacy.
  - Onglet **Karma Gains** : tableau Interaction / Karma (éditable).
  - Onglet **Karma Related Rights** : tableau Fonctionnalité / Karma requis (éditable).
- **Smart button** : Posts (nombre) → liste des posts du forum.
- **Actions** sur les posts : Export, Archive, Unarchive, Delete.

### 3.2 Forum : Tags

- Création / édition de tags, liaison au forum.
- Pas de vues par défaut dans certaines versions ; création de vues personnalisées possible (form, tree).

### 3.3 Forum : Ranks

- Formulaire : Rank Name, Required Karma, Description, Motivational message, Image.
- Liste des rangs par ordre de karma.

### 3.4 Forum : Badges

- Formulaire : Nom, description, image.
- **Allowance to Grant** : Everyone / A selected list of users / People having some badges (Required Badges).
- **Monthly Limited Sending** : optionnel, nombre par mois.
- **Assign automatically** : No one, assigned through challenges → liaison à des défis (challenges).
- **Forum Badge Level** : Bronze, Silver, Gold.

### 3.5 Forum : Close Reasons

- Liste et formulaire : raison de fermeture, **Reason Type** (Basic pour fermeture, Offensive pour signalements).

---

## 4. Patterns d’Interaction

| Action | Où | Condition |
|--------|-----|-----------|
| Lire questions | Liste forum | Confidentialité forum |
| Lire un post | Détail post | Idem |
| New Post | Bouton liste ou header | Connecté, karma Ask questions |
| Répondre | Formulaire sous question | Connecté, karma Answer questions |
| Commenter | Icône 💬 | karma Comment own/all |
| Voter | ▲ ▼ sur question/réponse | karma Upvote/Downvote |
| Favori / Suivi | ☆ / 🔔 | Connecté |
| Best answer | ✔ | Auteur question ou droit Accept answer (karma) |
| Edit / Close / Delete | Menu … | karma Edit/Close/Delete own/all |
| Flag | Menu … | karma Flag |
| Convert answer ↔ comment | Menu … | karma Convert |
| Voir ticket Helpdesk | Menu … | Intégration Helpdesk installée |
| Modération | Sidebar To Validate / Flagged / Closed | karma Moderate (ou rôle) |
| Config forums / karma / tags / rangs / badges | Backend | Droits config site |

---

## 5. Design et Accessibilité

- **Responsive** : Forum conçu pour le web ; liste et détail adaptés aux écrans (documentation Odoo).
- **SEO** : Nofollow sur les liens des utilisateurs sous le seuil karma (ex. 500) pour limiter le spam.
- **Images** : Éditeur avec images et liens selon karma (ex. 30).
- **Messages** : Attente de validation, confirmation best answer, raisons de fermeture / offensif explicites pour éviter les malentendus.

---

## 6. Recommandations pour Miyukini

- **ForumUI** : Écrans équivalents liste questions, détail post (question + réponses + commentaires), formulaire New Post, sidebar tags + tri + modération.
- **Rôles et karma** : Exposer les droits (équivalent karma) dans l’UI (tooltips, page « Mes droits », profil).
- **Modération** : Vues dédiées To Validate, Flagged, Closed avec audit trail (TAMR).
- **Configuration** : Écrans admin pour forums, tags, rangs, badges, close reasons et seuils « karma » (gouvernance StrongFather / Master Butler).
- **Helpdesk** : Lien post → ticket (si service Helpdesk présent) comme dans Odoo.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
