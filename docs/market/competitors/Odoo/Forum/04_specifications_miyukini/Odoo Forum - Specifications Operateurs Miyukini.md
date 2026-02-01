# Odoo Forum — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Forum** d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour équivalent Forum
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d'Ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **ForumOperator** | Gestion des forums (boards) | Opérateur de Service |
| **PostOperator** | Gestion des posts (questions, réponses, commentaires) | Opérateur de Service |
| **TagOperator** | Gestion des tags | Opérateur de Service |
| **KarmaOperator** | Réputation, droits, rangs et badges | Opérateur de Service |
| **ModerationOperator** | Validation, signalement, clôture | Opérateur de Service |
| **ForumUI** | Interface utilisateur Forum | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : ForumService

**Définition :**
> **ForumService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de forum (questions/réponses, karma, modération).**

**Composition :**
- ForumOperator (niveau sécurité 2)
- PostOperator (niveau sécurité 2)
- TagOperator (niveau sécurité 1)
- KarmaOperator (niveau sécurité 2)
- ModerationOperator (niveau sécurité 2–3)
- ForumUI (niveau sécurité 1)

---

## 2. Opérateurs Détaillés

### 2.1 ForumOperator

**Rôle :** Gestion des forums (conteneurs : nom, mode Questions/Discussions, tri par défaut, confidentialité, configuration karma).

**Capacités :**
- Création / modification de forums
- Configuration mode (Questions vs Discussions), tri, confidentialité
- Configuration karma gains et karma-related rights (seuils)
- Exposition des forums pour lecture (selon privacy)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création/modification forum, changement privacy
- **KindMother** : Persistance des forums (WriteIntent)
- **Master Butler** : Permissions de création/modification forum
- **WorrySentinel** : Niveau de sécurité, exposition publique (Façade)

**Contrat d'équipe :**
- Consomme : TagOperator (tags du forum), KarmaOperator (seuils)
- Expose : `forum.create`, `forum.update`, `forum.list`, `forum.get`

**Mandat de Permission requis :**
- Création/modification forum : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Lecture forum : Mandat ou Façade Publique Gouvernée (si Public)

### 2.2 PostOperator

**Rôle :** Gestion des posts (questions, réponses, commentaires) : CRUD, vote, best answer, favoris, suivi.

**Capacités :**
- Création/modification/suppression de questions, réponses, commentaires
- Règles métier : une réponse par utilisateur et par question ; best answer en mode Questions
- Votes (up/down) selon karma
- Favoris et suivi (notifications)
- Exposition des posts selon état (publié, en attente, fermé, masqué)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de publication, best answer, fermeture (côté décision)
- **KindMother** : Persistance des posts (WriteIntent)
- **Master Butler** : Droits (équivalent karma) : ask, answer, vote, edit, close, delete
- **WorrySentinel** : Contenu (modération), niveau sécurité
- **KarmaOperator** : Consultation des seuils avant d’autoriser une action

**Contrat d'équipe :**
- Consommé par : ForumUI, ModerationOperator
- Consomme : ForumOperator, TagOperator, KarmaOperator, MiyuNotify (suivi)
- Expose : `post.create`, `post.update`, `post.delete`, `post.vote`, `post.best_answer`, `post.list`, `post.get`

**Mandat de Permission requis :**
- Création post : Mandat avec KindMother (WriteIntent) + StrongFather (décision) + KarmaOperator (seuils)
- Vote / best answer : Mandat avec KindMother (WriteIntent) + KarmaOperator (seuils)
- Lecture : Mandat ou Façade (selon privacy forum et état du post)

### 2.3 TagOperator

**Rôle :** Gestion des tags (création, association aux questions, filtrage).

**Capacités :**
- Création/modification de tags liés à un forum
- Association tags ↔ questions (jusqu’à 5 par question)
- Liste et filtrage par tag
- Droits : « Create new tags », « Change question tags » (équivalent karma)

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **StrongFather** : Décision de création de tag (optionnel)
- **KindMother** : Persistance des tags et associations (WriteIntent)
- **Master Butler** : Permissions création/modification tags

**Contrat d'équipe :**
- Consommé par : ForumOperator, PostOperator, ForumUI
- Consomme : ForumOperator (forum parent)
- Expose : `tag.create`, `tag.update`, `tag.list`, `tag.get`

**Mandat de Permission requis :**
- Création tag : Mandat avec KindMother (WriteIntent) + KarmaOperator (seuil « Create new tags »)
- Modification tags d’une question : Mandat avec PostOperator + KarmaOperator (seuil « Change question tags »)

### 2.4 KarmaOperator

**Rôle :** Gestion de la réputation (karma), des seuils de droits, des rangs et badges.

**Capacités :**
- Calcul et historisation du karma (gains/pertes par interaction)
- Exposition des seuils « karma-related rights » aux autres opérateurs (PostOperator, ModerationOperator)
- Gestion des rangs (seuils karma) et badges (attribution manuelle ou défis)
- Validation email : attribution des points initiaux (+3)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision d’attribution manuelle de karma/badges (optionnel)
- **KindMother** : Persistance du karma et des badges (WriteIntent)
- **Master Butler** : Lecture des droits « débloqués » selon karma
- **Ever Buddy** : Cycle de vie (historique karma, rangs)

**Contrat d'équipe :**
- Consommé par : PostOperator, ModerationOperator, ForumUI (affichage profil)
- Consomme : MiyuAuth / res.users (identité)
- Expose : `karma.get`, `karma.gain`, `karma.loss`, `rights.check`, `rank.list`, `badge.grant`

**Mandat de Permission requis :**
- Modification karma (gain/perte) : déclenchée par PostOperator/ModerationOperator avec Mandat
- Lecture karma/droits : Mandat avec PostOperator ou ForumUI
- Attribution manuelle badge/karma : Mandat avec StrongFather (décision) + KindMother (WriteIntent)

### 2.5 ModerationOperator

**Rôle :** Modération : validation des posts en attente, traitement des signalements, fermeture avec motif.

**Capacités :**
- Liste « To Validate » : valider ou rejeter les posts en attente
- Liste « Flagged » : accepter (lever flag) ou marquer offensif (raison, -100 karma, masquage)
- Liste « Closed » : rouvrir, supprimer
- Fermeture avec Close Reason (Basic / Offensive) ; application -100 karma si motif offensif/spam
- Audit des actions de modération

**Niveau de sécurité :** 2–3 (Sensitive à Critical selon données)

**Gouvernance :**
- **StrongFather** : Décision de validation, de marquage offensif, de fermeture
- **KindMother** : Persistance des états (post validé, fermé, masqué) et historique (WriteIntent)
- **Master Butler** : Droit « Moderate posts » (ou rôle modérateur explicite)
- **WorrySentinel** : Niveau de confiance, contenu sensible
- **TAMR** : Intervention humaine (validation, signalement, fermeture)
- **KarmaOperator** : Application -100 karma en cas d’offensif/flag

**Contrat d'équipe :**
- Consommé par : ForumUI (sidebar Moderation tools)
- Consomme : PostOperator, KarmaOperator
- Expose : `moderation.validate`, `moderation.flag_accept`, `moderation.flag_offensive`, `moderation.close`, `moderation.reopen`, `moderation.list_pending`, `moderation.list_flagged`, `moderation.list_closed`

**Mandat de Permission requis :**
- Toute action de modération : Mandat avec StrongFather (décision) + KindMother (WriteIntent) + Master Butler (droit Moderate) + KarmaOperator (si -100 karma)

### 2.6 ForumUI

**Rôle :** Interface utilisateur Forum (liste questions, détail post, New Post, sidebar tags/tri/modération).

**Capacités :**
- Affichage liste des questions (tri, filtres tags)
- Affichage détail post (question + réponses + commentaires), votes, best answer, favoris, suivi
- Formulaire New Post (titre, description, tags)
- Actions : Answer, Comment, Vote, Best answer, Edit, Close, Delete, Flag, Convert, View Helpdesk ticket
- Sidebar : Tags, tri, Moderation tools (To Validate, Flagged, Closed)
- Configuration (backend-like) : Forums, Tags, Ranks, Badges, Close Reasons (si opérateur admin)

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **Master Butler** : Permissions d’accès aux vues
- **WorrySentinel** : Filtrage des données selon droits et confidentialité forum

**Contrat d'équipe :**
- Consomme : ForumOperator, PostOperator, TagOperator, KarmaOperator, ModerationOperator
- Expose : `ui.render_forum_list`, `ui.render_post_detail`, `ui.render_new_post`, `ui.render_moderation`, `ui.render_config`

**Mandat de Permission requis :**
- Lecture : Mandat avec ForumOperator/PostOperator (ou Façade si Public)
- Création/édition post : Mandat avec PostOperator
- Modération : Mandat avec ModerationOperator

---

## 3. Contrats d'Équipe

### 3.1 Contrat ForumService

**Opérateurs membres :**
- ForumOperator, PostOperator, TagOperator, KarmaOperator, ModerationOperator, ForumUI

**Flux autorisés :**
- ForumUI → ForumOperator (lecture forums)
- ForumUI → PostOperator (lecture/écriture posts)
- ForumUI → TagOperator (lecture/filtrage tags)
- ForumUI → KarmaOperator (lecture karma/droits, profil)
- ForumUI → ModerationOperator (listes et actions modération)
- PostOperator → ForumOperator (forum parent)
- PostOperator → TagOperator (tags des questions)
- PostOperator → KarmaOperator (vérification seuils, enregistrement gains/pertes)
- PostOperator → MiyuNotify (suivi)
- ModerationOperator → PostOperator (changer état, masquer)
- ModerationOperator → KarmaOperator (appliquer -100 karma)

**Types d'échanges :**
- Requêtes CRUD (forums, posts, tags)
- Vérification droits (karma)
- Gains/pertes karma
- Notifications (nouvelle réponse, validation, signalement)
- Actions modération (validate, flag, close)

**Niveau de validation requis :**
- Création/modification : StrongFather (décision) + KindMother (WriteIntent)
- Droits : Master Butler + KarmaOperator (seuils)
- Modération : TAMR (intervention humaine) + WorrySentinel

---

## 4. Mandats de Permission

### 4.1 Mandat Lecture Forum (Public)

**Émis par :** StrongFather / Façade Publique Gouvernée  
**Portée :** Lecture des forums et posts publiés (privacy Public).  
**Contenu :** Accès en lecture seule ; pas d’écriture, pas de modération.

### 4.2 Mandat Création Post

**Émis par :** StrongFather  
**Portée :** Création question/réponse/commentaire.  
**Contenu :** PostOperator (create), KindMother (WriteIntent), KarmaOperator (seuils ask/answer/comment).  
**Révocation :** Si karma insuffisant ou violation des règles.

### 4.3 Mandat Modération

**Émis par :** StrongFather  
**Portée :** Validation, signalement, fermeture.  
**Contenu :** ModerationOperator, PostOperator (update état), KarmaOperator (appliquer -100), KindMother (WriteIntent).  
**Révocation :** Retrait du rôle modérateur ou fin de session.

### 4.4 Mandat Configuration Forum

**Émis par :** StrongFather  
**Portée :** Création/modification forums, karma gains/rights, tags, rangs, badges, close reasons.  
**Contenu :** ForumOperator, KarmaOperator, TagOperator, KindMother (WriteIntent).  
**Révocation :** Retrait des droits admin site/forum.

---

## 5. Niveaux de Sécurité (WorrySentinel)

| Donnée | Niveau | Justification |
|--------|--------|----------------|
| Forum (config, privacy) | 2 | Données de configuration sensible |
| Post (contenu utilisateur) | 2 | Contenu potentiellement personnel ou sensible |
| Karma, rangs, badges | 2 | Réputation et droits |
| Actions modération | 2–3 | Décisions impactant visibilité et réputation |
| Tags | 1 | Données peu sensibles |
| UI (lecture publique) | 1 | Façade Publique Gouvernée |

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
