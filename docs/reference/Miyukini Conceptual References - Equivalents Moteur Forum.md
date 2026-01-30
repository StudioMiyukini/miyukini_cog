# Miyukini Conceptual References — Équivalents moteur de forum (phpBB et similaires)

## Contexte

Ce document constitue la **référence conceptuelle** pour transposer, dans l'environnement Miyukini COG, les fonctionnalités des **moteurs de forum** tels que **phpBB**, **vBulletin**, **XenForo** et logiciels de forum génériques. Il vise à permettre la création d'**outils**, **opérateurs** et **services** Miyukini pour proposer des **services forum spécialisés** :

- **Structure forum** (catégories, forums, sous-forums, topics, posts)
- **Contenu et formatage** (BBCode, pièces jointes, brouillons, citations, sondages, smilies/emoji)
- **Utilisateurs et profils** (inscription, signatures, avatars, rangs, champs profil, liste en ligne, amis/ennemis)
- **Modération** (file d'attente, lock/move/merge/split topics, sticky/annonces, avertissements, bannissements, notes modérateurs)
- **Messagerie privée** (envoi, dossiers, BCC, groupes, brouillons, pièces jointes, vue conversation)
- **Abonnements et notifications** (abonnement forums/topics, signets, notifications email/in-app, flux ATOM)
- **Recherche** (fulltext topics/posts/utilisateurs, filtres avancés)
- **Sécurité et anti-spam** (CAPTCHA, flood control, signalement, approbation)

Il **s'appuie sur** la documentation conceptuelle existante : [Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md), [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md), [Opérateurs et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md), [Mandats et Équipes Opérateurs](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md), [Pyramide Architecture Complète](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md), [Définition COG](./Miyukini%20Conceptual%20References%20-%20Definition%20COG.md).

---

## Fondements conceptuels (alignement documentation existante)

Ce document applique les **définitions canoniques** et **règles** des références listées ci-dessus. Les équivalents forum respectent en particulier :

### Outils (Tools) — [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)

- **Définition :** Un Outil est une capacité exécutable, sans autorité, sans décision métier, sans connaissance de l'Opérateur appelant, gouvernée par les Cores.
- **Règle :** *« Un Outil fait, mais ne décide jamais. »* Les Tools forum (ex. `tool.forum.topic.create`, `tool.pm.send`) exécutent des actions ; la décision (autoriser publication, modération, bannissement) appartient à **StrongFather**.

### Kits d'Outils (Toolkits) — [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)

- **Définition :** Un Kit d'Outils est une composition officielle d'Outils, validée et déclarée par l'environnement, optimisée pour efficience et cohérence.
- **Règle :** *« Un Kit d'Outils n'ajoute aucune capacité nouvelle, il orchestre proprement des Outils existants. »* Les Toolkits forum (`toolkit.community.forum`, `toolkit.communication.pm`, etc.) agrègent des Tools existants sans logique métier propre.

### Opérateurs (Operators) — [Opérateurs et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md)

- **Définition :** Un Opérateur est une entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l'utilisateur au sein d'un environnement Miyukini.
- Les Opérateurs forum (Forum, Modération, Messagerie, etc.) sont des **Opérateurs de Domaine** ou **d'Interface** (Strate 7) ; ils n'ont pas d'autorité propre et passent par la gouvernance pour toute action.

### Service vs Opérateur — [Mandats et Équipes Opérateurs](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md)

- **Service** = capacité perçue par l'utilisateur. **Opérateur** = unité d'exécution gouvernée.
- **Règle :** *« Un Service peut être porté par un Opérateur... ou par une Équipe d'Opérateurs. »* Le service « forum communautaire » (discussions, MP, modération, notifications) peut être livré par une **Équipe d'Opérateurs** sous **Contrat d'équipe** et **Mandat de Permission**.

### Données et écriture — [Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md)

- **KindMother** : autorité sur toutes les données (topics, posts, messages privés, abonnements, signets, profils étendus, modération). Toute écriture passe par **WriteIntent** sous autorité KindMother.
- **StrongFather** : décision ALLOW/DENY (publication, modération, bannissement, avertissements). N'exécute jamais.

---

## Portée / Scope

**Ce document définit :**

- La cartographie détaillée **moteur de forum (phpBB et similaires)** → Outils, Opérateurs, Services Miyukini
- Les **Kits d'outils (Toolkits)** et **Outils (Tools)** déjà couverts par le projet Miyukini
- Les **Kits et Outils manquants** à créer pour couvrir un service forum complet, conformes à [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)
- Les **Opérateurs** (Domaine, Interface) à déployer pour un service forum, conformes à [Opérateurs et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md)
- Les **Cores** impliqués et les flux de gouvernance

**Hors scope :**

- L'implémentation technique détaillée (code, schémas DB)
- Les contrats d'intégration par outil (voir documentations fondatrices des Tools)
- La stratégie commerciale ou marketing des services forum

**Statut :** Document de référence normatif — source de vérité pour la conception des équivalents forum Miyukini.

---

## 1. Périmètre cible et objectifs

### 1.1 Équivalents logiciels ciblés

| Équivalent | Rôle | Objectif Miyukini |
|------------|------|--------------------|
| **phpBB** | Forum open source (catégories, topics, posts, MP, modération, permissions, extensions) | Opérateur Forum + Toolkits forum, PM, notify, moderation, search, antispam |
| **vBulletin / XenForo** | Forums commerciaux (structure similaire, profils avancés, médias) | Même modèle COG ; réutilisation MiyuCMS/MiyuMedia pour contenus et médias |
| **Moteur de forum générique** | Discussions, fils, modération, utilisateurs | Toolkits atomiques + Opérateurs gouvernés |

### 1.2 Services utilisateur visés

| Service | Description | Opérateurs / Tools principaux |
|--------|-------------|-------------------------------|
| **Forum de discussion** | Catégories, forums, topics, posts, sticky, annonces | Opérateur Forum (Domaine), Toolkit community.forum |
| **Messagerie privée** | Envoi, réception, dossiers, BCC, groupes, brouillons | Opérateur Messagerie ou Domaine, Toolkit communication.pm |
| **Modération** | File d'attente, lock/move/merge/split, avertissements, bannissements | Opérateur Modération ou extension Forum, Toolkit moderation.forum |
| **Abonnements et notifications** | Abonnement forums/topics, signets, email, flux ATOM | Toolkit communication.notify, Toolkit content.feeds, Toolkit content.bookmarks |
| **Profil et communauté** | Signatures, avatars, rangs, amis/ennemis, présence | Toolkit identity.profile, Toolkit communication.contacts, Tools presence |
| **Recherche et découverte** | Fulltext, filtres, « sans réponse », « actifs » | Toolkit content.search |
| **Anti-spam et sécurité** | CAPTCHA, flood control, signalement, approbation | Toolkit security.antispam, Tools moderation.report |

---

## 2. Équivalents déjà couverts par le projet Miyukini

Les Kits et Tools suivants **existent déjà** dans [docs/tools/](../tools/_index.md) et couvrent une partie des besoins forum :

| Fonctionnalité forum | Équivalent Miyukini existant | Détail |
|----------------------|------------------------------|--------|
| **Posts / contenu éditorial** | **MiyuCMS** | `tool.content.create`, `tool.content.update`, `tool.content.publish`, `tool.content.schedule`, révisions |
| **Commentaires (et modération basique)** | **MiyuCMS** | `tool.content.comment.create`, `tool.content.comment.list`, `tool.content.comment.moderate` |
| **Médias / pièces jointes** | **MiyuCMS** + **MiyuMedia** | `tool.media.upload`, `tool.media.serve`, `tool.media.transform` (dont miniatures) |
| **Identité / rôle / connexion** | **MiyuAuth** | `tool.identity.resolve`, `tool.identity.attest`, `tool.identity.verify`, `tool.identity.role` (citoyen / visiteur / externe) |
| **Rôles / permissions (concept)** | **Master Butler** + **StrongFather** | Décision ALLOW/DENY = StrongFather ; catalogue = Master Butler |
| **Affichage web / thèmes / formulaires** | **MiyuWeb**, **MiyuWidgets** | Rendu HTML, layout, thème, formulaires, widgets |
| **Données / requêtes** | **MiyuSQL** | Requêtes, transactions, cache |
| **Horloge (trace only)** | **MiyuClock** | Instant présent, delta (LOI-4) |

**Invariant :** Aucun de ces Kits ne fournit la **structure forum** (topics, catégories/forums, fils de discussion), la **messagerie privée**, les **sondages**, les **abonnements/notifications**, les **signets**, la **présence**, le **profil étendu** (signature, avatar, rangs), les **contacts** (amis/ennemis), la **modération étendue** (lock/move/merge/split, avertissements, bannissements), l'**anti-spam** (CAPTCHA, flood), ni la **recherche fulltext** dédiée.

---

## 3. Cartographie moteur de forum → Miyukini COG

### 3.1 Structure forum (catégories, forums, topics, posts)

| Fonctionnalité forum | Équivalent Miyukini | Type | Détail |
|----------------------|---------------------|------|--------|
| **Catégories** | Données KindMother + Tools forum | Core + Tools | `tool.forum.category.list` ; conteneurs organisationnels. |
| **Forums / sous-forums** | Données KindMother + Tools forum | Core + Tools | `tool.forum.board.list`, `tool.forum.board.resolve` ; hiérarchie illimitée. |
| **Topics (fils plats)** | Tools forum | Tools | `tool.forum.topic.create`, `tool.forum.topic.update`, `tool.forum.topic.list`, `tool.forum.topic.resolve` ; persistance = KindMother. |
| **Posts (réponses dans un topic)** | Tools forum (ou réutilisation content) | Tools | `tool.forum.post.create`, `tool.forum.post.update`, `tool.forum.post.list` ; ou extension MiyuCMS avec type « post forum ». |
| **Sticky / annonces** | Tools forum | Tools | `tool.forum.topic.sticky.set`, `tool.forum.topic.announcement.set` ; décision = StrongFather. |
| **Dernier post (last post)** | Données KindMother + affichage | Core | Résolu par requête/liste ; pas de Tool dédié. |
| **Tri (topic/post)** | Paramètres de liste fournis dans le flux | Flux | `tool.forum.topic.list` (sort, order) ; pas de décision métier dans le Tool. |
| **Suivi lu / non lu** | Données KindMother + Tools | Core + Tools | `tool.forum.readtrack.update`, `tool.forum.readtrack.list` ; persistance = KindMother. |
| **Impression / email topic** | Tools export / rendu | Tools | `tool.forum.topic.export.print`, `tool.forum.topic.export.email` ; contenu fourni dans le flux. |

### 3.2 Modération forum

| Fonctionnalité forum | Équivalent Miyukini | Type | Détail |
|----------------------|---------------------|------|--------|
| **File modération (posts/topics)** | Tools modération | Tools | `tool.moderation.queue.list`, `tool.moderation.queue.resolve` ; décision approbation = StrongFather. |
| **Signalement (report)** | Tools modération | Tools | `tool.moderation.report.create`, `tool.moderation.report.list` ; traitement = StrongFather. |
| **Lock / unlock topic** | Tools forum | Tools | `tool.forum.topic.lock`, `tool.forum.topic.unlock` ; autorisation = StrongFather. |
| **Move topic** | Tools forum | Tools | `tool.forum.topic.move` ; destination fournie ; autorisation = StrongFather. |
| **Merge / split topics** | Tools forum | Tools | `tool.forum.topic.merge`, `tool.forum.topic.split` ; autorisation = StrongFather. |
| **Delete topic / post** | Tools forum | Tools | `tool.forum.topic.delete`, `tool.forum.post.delete` ; soft delete optionnel (données) ; autorisation = StrongFather. |
| **Copy topic** | Tools forum | Tools | `tool.forum.topic.copy` ; destination fournie. |
| **Edit post (modérateur)** | Tools forum | Tools | `tool.forum.post.edit` (contenu fourni) ; autorisation = StrongFather. |
| **Lock post (auteur ne peut plus éditer)** | Tools forum | Tools | `tool.forum.post.lock` ; autorisation = StrongFather. |
| **Changer auteur post** | Tools forum | Tools | `tool.forum.post.author.change` ; autorisation = StrongFather. |
| **Annonces globales** | Données + affichage | Core + flux | Topic avec flag « annonce globale » ; liste côté Opérateur. |
| **Avertissements (warnings)** | Tools modération | Tools | `tool.moderation.warning.issue`, `tool.moderation.warning.list`, `tool.moderation.warning.revoke` ; règles = StrongFather. |
| **Bannissement / suspension** | Tools modération | Tools | `tool.moderation.ban.issue`, `tool.moderation.ban.revoke`, `tool.moderation.ban.list` ; décision = StrongFather. |
| **Notes modérateurs (user notes)** | Tools modération | Tools | `tool.moderation.usernote.add`, `tool.moderation.usernote.list` ; visibles modérateurs uniquement. |

### 3.3 Messagerie privée (MP)

| Fonctionnalité forum | Équivalent Miyukini | Type | Détail |
|----------------------|---------------------|------|--------|
| **Envoyer MP** | Tools PM | Tools | `tool.pm.send` ; destinataires, BCC, groupe fournis dans le flux ; autorisation = StrongFather. |
| **Dossiers personnalisés** | Tools PM | Tools | `tool.pm.folder.create`, `tool.pm.folder.update`, `tool.pm.folder.delete`, `tool.pm.folder.list` ; règles tri = données ou flux. |
| **Liste / lecture MP** | Tools PM | Tools | `tool.pm.list`, `tool.pm.get` ; filtres (dossier, non lu) fournis. |
| **Brouillons MP** | Tools PM | Tools | `tool.pm.draft.save`, `tool.pm.draft.list`, `tool.pm.draft.send` ; persistance = KindMother. |
| **Pièces jointes en MP** | MiyuMedia + flux | Tools | Données pièce jointe fournies ; `tool.media.upload` réutilisé ou `tool.pm.attachment.add`. |
| **Vue conversation** | Données KindMother + Tools | Core + Tools | `tool.pm.conversation.list` (fil par destinataires) ; persistance = KindMother. |
| **Export MP** | Tools PM | Tools | `tool.pm.export` ; format fourni (archive) ; autorisation = StrongFather. |

### 3.4 Contenu et formatage

| Fonctionnalité forum | Équivalent Miyukini | Type | Détail |
|----------------------|---------------------|------|--------|
| **BBCode / custom BBCode** | Tools formatage | Tools | `tool.content.format.parse` (BBCode → structure), `tool.content.format.render` (structure → HTML) ; ou extension MiyuWeb. |
| **Smilies / emoji** | Règles d'affichage ou Tools | Tools / flux | Option dans `tool.content.format.render` ou données thème (MiyuWeb). |
| **Citations (nested)** | Données + rendu | Flux | Structure citation dans le contenu ; rendu = MiyuWeb / format. |
| **Censure de mots** | Tools modération / contenu | Tools | `tool.content.censor.apply` (liste mots fournie) ; règles = StrongFather. |
| **Syntax highlighting (code)** | Rendu | Tools | Option dans `tool.content.format.render` ou bloc dédié MiyuWeb. |
| **Brouillons posts** | Tools forum ou content | Tools | `tool.forum.post.draft.save`, `tool.forum.post.draft.list` ; ou `tool.content.create` (brouillon) avec type « post ». |
| **Aperçu avant envoi** | Rendu côté client / Opérateur | UX | Données fournies → MiyuWeb rendu ; pas de Tool dédié. |
| **Sondages (polls)** | Tools polls | Tools | `tool.poll.create`, `tool.poll.vote`, `tool.poll.list`, `tool.poll.result` ; durée, changement de vote = données/règles (StrongFather). |

### 3.5 Utilisateurs et profils

| Fonctionnalité forum | Équivalent Miyukini | Type | Détail |
|----------------------|---------------------|------|--------|
| **Inscription (CAPTCHA, complexité)** | MiyuAuth + Toolkit antispam | Toolkit + Tools | Inscription = flux gouverné ; `tool.antispam.captcha.verify`, `tool.antispam.flood.check` (voir § 3.8). |
| **Signatures** | Toolkit profil | Tools | `tool.profile.signature.get`, `tool.profile.signature.set` ; persistance = KindMother. |
| **Avatars / Gravatar** | Toolkit profil | Tools | `tool.profile.avatar.get`, `tool.profile.avatar.set`, `tool.profile.avatar.resolve` (Gravatar) ; stockage = KindMother ou MiyuMedia. |
| **Rangs (ranks)** | Toolkit profil | Tools | `tool.profile.rank.resolve`, `tool.profile.rank.list` ; règles attribution = StrongFather ; données = KindMother. |
| **Champs profil personnalisés** | Toolkit profil | Tools | `tool.profile.field.list`, `tool.profile.field.get`, `tool.profile.field.set` ; schéma = KindMother. |
| **Liste « qui est en ligne »** | Tools présence | Tools | `tool.presence.list` (filtre période) ; données = KindMother. |
| **Préférences utilisateur** | Données KindMother + Tools | Core + Tools | `tool.profile.preferences.get`, `tool.profile.preferences.set` (tri, langue, notifications). |
| **Amis / ennemis (friend/foe)** | Toolkit contacts | Tools | `tool.contacts.friend.add`, `tool.contacts.friend.remove`, `tool.contacts.friend.list` ; idem foe ; persistance = KindMother. |
| **Carnet d'adresses (pour MP)** | Toolkit contacts | Tools | Réutilisation `tool.contacts.friend.list` ou `tool.pm.addressbook.list`. |

### 3.6 Abonnements et notifications

| Fonctionnalité forum | Équivalent Miyukini | Type | Détail |
|----------------------|---------------------|------|--------|
| **Abonnement forum / topic** | Toolkit notify | Tools | `tool.notify.subscribe` (cible = forum ou topic), `tool.notify.unsubscribe`, `tool.notify.subscription.list` ; persistance = KindMother. |
| **Préférences notifications** | Toolkit notify | Tools | `tool.notify.preferences.get`, `tool.notify.preferences.set` (email, in-app, etc.) ; décision envoi = StrongFather. |
| **Envoi notification (email, in-app)** | Toolkit notify | Tools | `tool.notify.send` (canal, destinataire, contenu fournis) ; exécution seule ; décision = StrongFather. |
| **Menu notifications** | Données + affichage | Opérateur | Liste fournie par `tool.notify.list` (non lues, etc.) ; rendu = MiyuWeb. |
| **Flux ATOM (board, forum, topic)** | Toolkit feeds | Tools | `tool.feed.atom.board`, `tool.feed.atom.forum`, `tool.feed.atom.topic` ; contenu fourni dans le flux. |
| **Signets (bookmarks)** | Toolkit bookmarks | Tools | `tool.bookmark.add`, `tool.bookmark.remove`, `tool.bookmark.list` ; cible = topic (ou entité générique). |

### 3.7 Recherche

| Fonctionnalité forum | Équivalent Miyukini | Type | Détail |
|----------------------|---------------------|------|--------|
| **Recherche fulltext (topics, posts)** | Toolkit search | Tools | `tool.search.fulltext` (scope, requête, filtres fournis) ; backends = implémentation (MySQL, PostgreSQL, Sphinx). |
| **Filtres avancés** | Paramètres flux | Flux | Auteur, forum, date, « sans réponse », « actifs », « depuis dernière visite » = paramètres de `tool.search.fulltext` ou `tool.forum.topic.list`. |
| **Recherche par auteur** | Tools search / forum | Tools | `tool.search.by_author` ou filtre sur `tool.forum.topic.list` / `tool.forum.post.list`. |
| **Flood control recherche** | Toolkit antispam | Tools | `tool.antispam.rate_limit.check` (scope = search) ; décision bloquer = StrongFather. |

### 3.8 Anti-spam et sécurité

| Fonctionnalité forum | Équivalent Miyukini | Type | Détail |
|----------------------|---------------------|------|--------|
| **CAPTCHA (inscription, post)** | Toolkit antispam | Tools | `tool.antispam.captcha.generate`, `tool.antispam.captcha.verify` ; intégration reCAPTCHA = implémentation. |
| **Flood control (posts, PMs)** | Toolkit antispam | Tools | `tool.antispam.flood.check` (scope = post, pm, registration) ; seuils = données KindMother ou flux ; décision = StrongFather. |
| **Limite tentatives inscription** | Toolkit antispam | Tools | `tool.antispam.rate_limit.check` (scope = registration) ; décision = StrongFather. |
| **Approbation posts (avant publication)** | Modération + flux | Tools + flux | `tool.moderation.queue.list` + `tool.moderation.queue.resolve` (approve) ; décision = StrongFather. |

---

## 4. Équivalents manquants — Kits et Tools à créer

Les éléments suivants **ne sont pas encore présents** dans la liste des Tools du projet ([docs/tools/_index.md](../tools/_index.md)). Ils sont proposés pour couvrir un service forum complet.

### 4.1 Synthèse — Kits d'outils (Toolkits) manquants

| ToolkitId proposé | Domaine | Composition (résumé) | Usage principal |
|-------------------|---------|----------------------|------------------|
| `toolkit.community.forum` | community | tool.forum.category.*, tool.forum.board.*, tool.forum.topic.*, tool.forum.post.*, tool.forum.readtrack.*, tool.forum.topic.export.* | Structure forum : catégories, forums, topics, posts, sticky, annonces, suivi lu |
| `toolkit.communication.pm` | communication | tool.pm.send, tool.pm.list, tool.pm.get, tool.pm.folder.*, tool.pm.draft.*, tool.pm.conversation.*, tool.pm.export | Messagerie privée : envoi, dossiers, brouillons, conversation, export |
| `toolkit.content.polls` | content | tool.poll.create, tool.poll.vote, tool.poll.list, tool.poll.result | Sondages : création, vote, résultats |
| `toolkit.communication.notify` | communication | tool.notify.subscribe, tool.notify.unsubscribe, tool.notify.subscription.list, tool.notify.preferences.*, tool.notify.send, tool.notify.list | Abonnements et notifications : subscribe, préférences, envoi |
| `toolkit.content.feeds` | content | tool.feed.atom.board, tool.feed.atom.forum, tool.feed.atom.topic | Flux ATOM (board, forum, topic) |
| `toolkit.content.bookmarks` | content | tool.bookmark.add, tool.bookmark.remove, tool.bookmark.list | Signets (topics ou entités) |
| `toolkit.identity.profile` | identity | tool.profile.get, tool.profile.update, tool.profile.field.*, tool.profile.avatar.*, tool.profile.signature.*, tool.profile.rank.*, tool.profile.preferences.* | Profil étendu : champs, signature, avatar, rangs, préférences |
| `toolkit.communication.contacts` | communication | tool.contacts.friend.*, tool.contacts.foe.* (ou tool.contacts.list avec type) | Liste amis/ennemis, carnet d'adresses |
| `toolkit.moderation.forum` | moderation | tool.moderation.queue.*, tool.moderation.report.*, tool.forum.topic.lock/move/merge/split/delete/copy, tool.forum.post.edit/lock/delete, tool.moderation.warning.*, tool.moderation.ban.*, tool.moderation.usernote.* | Modération forum : file, reports, lock/move/merge/split, avertissements, bannissements, notes |
| `toolkit.security.antispam` | security | tool.antispam.captcha.generate/verify, tool.antispam.flood.check, tool.antispam.rate_limit.check | CAPTCHA, flood control, limite tentatives |
| `toolkit.content.search` | content | tool.search.fulltext, tool.search.by_author, (filtres avancés en paramètres) | Recherche fulltext topics/posts/utilisateurs |
| **Tools présence** | community / presence | tool.presence.list, tool.presence.heartbeat (optionnel) | Liste « qui est en ligne », dernière activité |
| **Tools formatage** | content | tool.content.format.parse, tool.content.format.render (BBCode, smilies), tool.content.censor.apply | BBCode, rendu, censure mots |

**Invariant :** Chaque Toolkit contient au moins deux Tools. Les Toolkits sont validés par Ever Buddy (cycle de vie, versions) et déclarés au Master Butler.

### 4.2 Synthèse — Outils (Tools) manquants par domaine

Format ToolId : `tool.<domain>.<action>` ou `tool.<domain>.<sous-domaine>.<action>` (conforme Master Butler).

#### 4.2.1 Forum (forum)

| ToolId | Action courte |
|--------|----------------|
| `tool.forum.category.list` | Liste les catégories (conteneurs forums) |
| `tool.forum.board.list` | Liste les forums (d'un forum ou catégorie) |
| `tool.forum.board.resolve` | Résout un forum par identifiant |
| `tool.forum.topic.create` | Crée un topic à partir de données fournies |
| `tool.forum.topic.update` | Met à jour un topic |
| `tool.forum.topic.list` | Liste les topics (filtres, tri, pagination) |
| `tool.forum.topic.resolve` | Résout un topic par identifiant |
| `tool.forum.topic.lock` | Verrouille un topic (plus de réponses) ; autorisation = StrongFather |
| `tool.forum.topic.unlock` | Déverrouille un topic |
| `tool.forum.topic.move` | Déplace un topic vers un autre forum |
| `tool.forum.topic.merge` | Fusionne des topics |
| `tool.forum.topic.split` | Scinde un topic |
| `tool.forum.topic.delete` | Supprime un topic (soft delete optionnel) |
| `tool.forum.topic.copy` | Copie un topic vers un forum |
| `tool.forum.topic.sticky.set` | Marque topic en sticky |
| `tool.forum.topic.announcement.set` | Marque topic en annonce |
| `tool.forum.post.create` | Crée un post dans un topic |
| `tool.forum.post.update` | Met à jour un post |
| `tool.forum.post.list` | Liste les posts d'un topic |
| `tool.forum.post.edit` | Édition modérateur (contenu fourni) |
| `tool.forum.post.lock` | Verrouille un post (auteur ne peut plus éditer) |
| `tool.forum.post.delete` | Supprime un post |
| `tool.forum.post.author.change` | Change l'auteur d'un post ; autorisation = StrongFather |
| `tool.forum.readtrack.update` | Met à jour le suivi lu/non lu |
| `tool.forum.readtrack.list` | Liste les suivi lu (par utilisateur) |
| `tool.forum.topic.export.print` | Produit une version imprimable d'un topic |
| `tool.forum.topic.export.email` | Envoie un topic par email (destinataire fourni) |
| `tool.forum.post.draft.save` | Sauvegarde un brouillon de post |
| `tool.forum.post.draft.list` | Liste les brouillons de post |

#### 4.2.2 Messagerie privée (pm)

| ToolId | Action courte |
|--------|----------------|
| `tool.pm.send` | Envoie un message privé (destinataires, BCC, groupe fournis) |
| `tool.pm.list` | Liste les messages (dossier, filtres) |
| `tool.pm.get` | Retourne un message par identifiant |
| `tool.pm.folder.create` | Crée un dossier personnalisé |
| `tool.pm.folder.update` | Met à jour un dossier |
| `tool.pm.folder.delete` | Supprime un dossier |
| `tool.pm.folder.list` | Liste les dossiers de l'utilisateur |
| `tool.pm.draft.save` | Sauvegarde un brouillon de MP |
| `tool.pm.draft.list` | Liste les brouillons |
| `tool.pm.draft.send` | Envoie un brouillon (identifiant fourni) |
| `tool.pm.conversation.list` | Liste les conversations (fil par destinataires) |
| `tool.pm.export` | Exporte les MP (archive) ; autorisation = StrongFather |

#### 4.2.3 Sondages (poll)

| ToolId | Action courte |
|--------|----------------|
| `tool.poll.create` | Crée un sondage (options, durée fournies) |
| `tool.poll.vote` | Enregistre un vote ; règles = StrongFather |
| `tool.poll.list` | Liste les sondages (filtres) |
| `tool.poll.result` | Retourne les résultats d'un sondage |

#### 4.2.4 Notifications (notify)

| ToolId | Action courte |
|--------|----------------|
| `tool.notify.subscribe` | Abonne à une cible (forum, topic) |
| `tool.notify.unsubscribe` | Désabonne |
| `tool.notify.subscription.list` | Liste les abonnements de l'utilisateur |
| `tool.notify.preferences.get` | Retourne les préférences de notification |
| `tool.notify.preferences.set` | Met à jour les préférences |
| `tool.notify.send` | Envoie une notification (canal, destinataire, contenu fournis) |
| `tool.notify.list` | Liste les notifications (non lues, etc.) |

#### 4.2.5 Flux (feed)

| ToolId | Action courte |
|--------|----------------|
| `tool.feed.atom.board` | Génère le flux ATOM du board |
| `tool.feed.atom.forum` | Génère le flux ATOM d'un forum |
| `tool.feed.atom.topic` | Génère le flux ATOM d'un topic |

#### 4.2.6 Signets (bookmark)

| ToolId | Action courte |
|--------|----------------|
| `tool.bookmark.add` | Ajoute un signet (cible = topic ou entité) |
| `tool.bookmark.remove` | Retire un signet |
| `tool.bookmark.list` | Liste les signets de l'utilisateur |

#### 4.2.7 Profil (profile)

| ToolId | Action courte |
|--------|----------------|
| `tool.profile.get` | Retourne le profil (étendu) d'un utilisateur |
| `tool.profile.update` | Met à jour le profil (champs fournis) |
| `tool.profile.field.list` | Liste les champs profil (définition) |
| `tool.profile.field.get` | Retourne la valeur d'un champ |
| `tool.profile.field.set` | Met à jour un champ |
| `tool.profile.avatar.get` | Retourne l'URL ou les données avatar |
| `tool.profile.avatar.set` | Définit l'avatar (données fournies) |
| `tool.profile.avatar.resolve` | Résout avatar (Gravatar si configuré) |
| `tool.profile.signature.get` | Retourne la signature |
| `tool.profile.signature.set` | Définit la signature |
| `tool.profile.rank.resolve` | Résout le rang affiché pour un utilisateur |
| `tool.profile.rank.list` | Liste les rangs (définition) |
| `tool.profile.preferences.get` | Retourne les préférences (tri, langue, notifications) |
| `tool.profile.preferences.set` | Met à jour les préférences |

#### 4.2.8 Contacts (contacts)

| ToolId | Action courte |
|--------|----------------|
| `tool.contacts.friend.add` | Ajoute un ami |
| `tool.contacts.friend.remove` | Retire un ami |
| `tool.contacts.friend.list` | Liste les amis |
| `tool.contacts.foe.add` | Ajoute à la liste ennemis |
| `tool.contacts.foe.remove` | Retire de la liste ennemis |
| `tool.contacts.foe.list` | Liste les ennemis |

#### 4.2.9 Modération (moderation)

| ToolId | Action courte |
|--------|----------------|
| `tool.moderation.queue.list` | Liste les éléments en file (posts/topics à approuver) |
| `tool.moderation.queue.resolve` | Traite un élément (approve/reject) ; décision = StrongFather |
| `tool.moderation.report.create` | Signale un post ou MP |
| `tool.moderation.report.list` | Liste les signalements |
| `tool.moderation.warning.issue` | Émet un avertissement à un utilisateur |
| `tool.moderation.warning.list` | Liste les avertissements (utilisateur ou global) |
| `tool.moderation.warning.revoke` | Révoque un avertissement |
| `tool.moderation.ban.issue` | Bannit (username, email, IP) ; décision = StrongFather |
| `tool.moderation.ban.revoke` | Lève un bannissement |
| `tool.moderation.ban.list` | Liste les bannissements actifs |
| `tool.moderation.usernote.add` | Ajoute une note modérateur sur un utilisateur |
| `tool.moderation.usernote.list` | Liste les notes modérateurs (utilisateur) |

#### 4.2.10 Anti-spam (antispam)

| ToolId | Action courte |
|--------|----------------|
| `tool.antispam.captcha.generate` | Génère un défi CAPTCHA |
| `tool.antispam.captcha.verify` | Vérifie la réponse CAPTCHA |
| `tool.antispam.flood.check` | Vérifie si l'action (post, pm, registration) est autorisée (pas de flood) |
| `tool.antispam.rate_limit.check` | Vérifie le rate limit (scope = search, registration, etc.) |

#### 4.2.11 Recherche (search)

| ToolId | Action courte |
|--------|----------------|
| `tool.search.fulltext` | Recherche fulltext (scope = topics, posts, users ; requête et filtres fournis) |
| `tool.search.by_author` | Recherche par auteur (topics ou posts) |

#### 4.2.12 Présence (presence)

| ToolId | Action courte |
|--------|----------------|
| `tool.presence.list` | Liste les utilisateurs en ligne (ou actifs sur une période) |
| `tool.presence.heartbeat` | Enregistre une activité (optionnel, pour « dernière activité ») |

#### 4.2.13 Formatage / contenu (content.format, content.censor)

| ToolId | Action courte |
|--------|----------------|
| `tool.content.format.parse` | Parse un contenu (ex. BBCode) en structure |
| `tool.content.format.render` | Rend une structure en HTML (BBCode, smilies, code) |
| `tool.content.censor.apply` | Applique la censure de mots (liste fournie ou KindMother) |

---

## 5. Opérateurs proposés pour un service forum

| Opérateur | Type | Service perçu | Tools principaux |
|-----------|------|----------------|-------------------|
| **Forum** | Domaine | Discussions (catégories, forums, topics, posts) | tool.forum.*, tool.content.format.*, tool.bookmark.*, tool.notify.subscribe.* |
| **Modération** | Domaine | Modération (file, reports, lock/move/merge/split, warnings, bans) | tool.moderation.*, tool.forum.topic.lock/move/merge/split/delete, tool.forum.post.edit/lock |
| **Messagerie privée** | Domaine | MP (envoi, dossiers, brouillons, conversations) | tool.pm.*, tool.contacts.friend.list (carnet) |
| **Profil / Communauté** | Domaine (ou Interface) | Profil (signature, avatar, rangs, préférences), amis/ennemis, présence | tool.profile.*, tool.contacts.*, tool.presence.* |
| **Interface Forum** | Interface | Affichage forum (liste forums, topics, posts, formulaire réponse) | MiyuWeb + données flux ; tool.forum.*, tool.content.format.render |
| **Interface MP** | Interface | Affichage messagerie (boîte de réception, envoi, dossiers) | MiyuWeb + tool.pm.* |

*Identité et rôles de base : MiyuAuth + Master Butler + StrongFather — pas d'Opérateur « utilisateurs » dédié ; le profil étendu (signature, avatar, rangs) est couvert par l'Opérateur Profil / Communauté et le Toolkit identity.profile.*

---

## 6. Équipes d'Opérateurs et Contrats d'équipe

Pour délivrer un **Service** « forum communautaire » (discussions + MP + modération + notifications), on constitue une **Équipe d'Opérateurs** liée par un **Contrat d'équipe** (membres, flux autorisés, types d'échanges, niveau de validation). L'équipe n'existe opérationnellement que sous un **Mandat de Permission** émis par StrongFather.

### 6.1 Exemple — Équipe « Forum communautaire »

| Membre | Rôle dans l'équipe |
|--------|--------------------|
| Opérateur Forum | Fournit structure (catégories, forums, topics, posts) et affichage liste/détail |
| Opérateur Interface Forum | Expose l'UI lecture/écriture (topics, posts, formulaire réponse) |
| Opérateur Modération | Gère file modération, reports, lock/move/merge/split, warnings, bans |
| Opérateur Messagerie privée | Fournit envoi/réception MP, dossiers, brouillons |
| Opérateur Profil / Communauté | Fournit profil étendu, amis/ennemis, présence |
| Opérateur Interface MP (optionnel) | Expose l'UI messagerie |

**Flux autorisés (exemple) :** Interface Forum → Forum (lecture/écriture topics, posts) ; Interface Forum → Profil (lecture avatar, signature) ; Modération → Forum (lock, move, etc.) ; Interface MP → Messagerie (lecture/envoi). Pas de communication directe Forum ↔ Modération sans passer par BondingBrother et Mandat.

**Contrat d'équipe :** Définit statiquement les membres, les flux, les types de données échangeables, les conditions préalables et le niveau de validation requis. Validé par StrongFather.

---

## 7. Cores impliqués et flux de gouvernance

Les Cores **ne font jamais d'exécution** ; ils gouvernent, décident ou observent.

| Core | Rôle dans le périmètre forum |
|------|------------------------------|
| **KindMother** | Autorité sur toutes les données : topics, posts, MP, abonnements, signets, profils étendus, modération (warnings, bans, notes), suivi lu. WriteIntent pour toute écriture. |
| **StrongFather** | Décision finale ALLOW/DENY : publication topic/post, modération (approve, lock, move, ban, warning), envoi MP, abonnement. Émission et révocation des Mandats de Permission. Validation des Contrats d'équipe. |
| **Master Butler** | Déclaration des Tools et Toolkits (forum, pm, notify, moderation, antispam, search, profile, contacts, etc.). Permissions et capabilities. |
| **BondingBrother** | Médiation des intentions (lecteur, posteur, modérateur, admin) ; traduction vers les Cores et les Opérateurs. |
| **WorrySentinel** | Niveau de sécurité (données sensibles, MP, modération) ; blocage si menace ou état dégradé. |
| **Caring Nanny** | État système (HEALTHY, DEGRADED, etc.) ; blocage des Tools si environnement dégradé. |
| **Ever Buddy** | Cycle de vie : dépréciation Tools/Toolkits, compatibilité versions (forum, PM, notify). |
| **Border Guard** | Frontières et niveaux de confiance ; multi-tenant si forum multi-espaces. |
| **TAMR** | Points d'intervention humaine (arbitrage modération, déblocage utilisateur) si définis. |

**Flux générique :** Opérateur → BondingBrother → Master Butler (existence Tool, permissions) → WorrySentinel (niveau sécurité) → Caring Nanny (état système) → StrongFather (ALLOW/DENY) → Exécution Tool ; toute persistance passe par WriteIntent KindMother.

---

## 8. Priorisation suggérée pour mise en œuvre

Pour livrer un service forum « type phpBB » de façon progressive :

| Priorité | Élément | Justification |
|----------|---------|---------------|
| 1 | **MiyuForum** (toolkit.community.forum) + **MiyuModeration** (toolkit.moderation.forum) | Structure de base (topics, posts, modération lock/move/merge/split) |
| 2 | **MiyuPM** (toolkit.communication.pm) | Messagerie privée (fonctionnalité centrale forum) |
| 3 | **MiyuNotify** (toolkit.communication.notify) + **MiyuFeeds** (toolkit.content.feeds) | Abonnements et flux ATOM |
| 4 | **MiyuPolls** (toolkit.content.polls), **MiyuBookmarks** (toolkit.content.bookmarks) | Sondages et signets |
| 5 | **MiyuProfile** (toolkit.identity.profile), **MiyuContacts** (toolkit.communication.contacts), **MiyuPresence** (tool.presence.*) | Profil, amis/ennemis, présence |
| 6 | **MiyuAntiSpam** (toolkit.security.antispam), **MiyuSearch** (toolkit.content.search), **Formatage** (tool.content.format.*, tool.content.censor.apply) | Anti-spam, recherche, BBCode/censure |

---

## 9. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Opérateurs et Terminologie | [Miyukini Conceptual References - Operators et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md) |
| Mandats et Équipes Opérateurs | [Miyukini Conceptual References - Mandats et Equipes Operators](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md) |
| Pyramide Architecture Complète | [Miyukini Conceptual References - Pyramide Architecture Complete](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) |
| Équivalents Boutique CMS Réservation SaaS | [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](./Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md) |
| Équivalents PoS Logiciel Caisse | [Miyukini Conceptual References - Equivalents PoS Logiciel Caisse](./Miyukini%20Conceptual%20References%20-%20Equivalents%20PoS%20Logiciel%20Caisse.md) |
| Index Tools (docs/tools) | [Tools — Index de navigation](../tools/_index.md) |
| MiyuCMS Documentation Fondatrice | [MiyuCMS - Documentation Fondatrice](../tools/MiyuCMS/MiyuCMS%20-%20Documentation%20Fondatrice.md) |
| MiyuAuth Documentation Fondatrice | [MiyuAuth - Documentation Fondatrice](../tools/MiyuAuth/MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| Master Butler - Tool Governance Contract | (voir core/MasterButler/contracts/tools) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence normatif — Équivalents moteur de forum (phpBB et similaires)
