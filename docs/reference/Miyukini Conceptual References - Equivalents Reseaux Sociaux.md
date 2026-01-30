# Miyukini Conceptual References — Équivalents réseaux sociaux dominants

## Contexte

Ce document constitue la **référence conceptuelle** pour transposer, dans l'environnement Miyukini COG, les fonctionnalités des **réseaux sociaux dominants** (Facebook, Instagram, X/Twitter, LinkedIn, TikTok, YouTube, messageries sociales). Il vise à permettre la création d'**Outils**, **Opérateurs** et **Services** Miyukini pour proposer des **services sociaux gouvernés** :

- **Fil d'actualité** (posts, flux algorithmique ou chronologique, réactions, partages, commentaires)
- **Contenu éphémère** (stories, statuts 24h)
- **Messagerie sociale** (DMs, conversations, groupes de discussion)
- **Profil et identité sociale** (bio, avatar, liens, abonnés / abonnements)
- **Découverte** (explore, tendances, hashtags, recommandations)
- **Médias** (photos, vidéos courtes/longues, reels, shorts)
- **Modération et signalement** (signaler, modération contenu, blocage)
- **Notifications et abonnements** (abonnement créateur, alertes, préférences)

Il **s'appuie sur** la documentation conceptuelle existante : [Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md), [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md), [Opérateurs et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md), [Mandats et Équipes Opérateurs](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md), [Pyramide Architecture Complète](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md), [Définition COG](./Miyukini%20Conceptual%20References%20-%20Definition%20COG.md).

---

## Fondements conceptuels (alignement documentation existante)

Ce document applique les **définitions canoniques** et **règles** des références listées ci-dessus. Les équivalents réseaux sociaux respectent en particulier :

### Outils (Tools) — [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)

- **Définition :** Un Outil est une capacité exécutable, sans autorité, sans décision métier, sans connaissance de l'Opérateur appelant, gouvernée par les Cores.
- **Règle :** *« Un Outil fait, mais ne décide jamais. »* Les Tools sociaux (ex. `tool.social.post.create`, `tool.social.feed.list`) exécutent des actions ; la décision (autoriser publication, modération, visibilité) appartient à **StrongFather**.

### Kits d'Outils (Toolkits) — [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)

- **Définition :** Un Kit d'Outils est une composition officielle d'Outils, validée et déclarée par l'environnement, optimisée pour efficience et cohérence.
- **Règle :** *« Un Kit d'Outils n'ajoute aucune capacité nouvelle, il orchestre proprement des Outils existants. »* Les Toolkits sociaux (`toolkit.social.feed`, `toolkit.social.messaging`, etc.) agrègent des Tools existants sans logique métier propre.

### Opérateurs (Operators) — [Opérateurs et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md)

- **Définition :** Un Opérateur est une entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l'utilisateur au sein d'un environnement Miyukini.
- Les Opérateurs sociaux (Fil d'actualité, Messagerie sociale, Découverte, etc.) sont des **Opérateurs de Domaine** ou **d'Interface** (Strate 7) ; ils n'ont pas d'autorité propre et passent par la gouvernance pour toute action.

### Service vs Opérateur — [Mandats et Équipes Opérateurs](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md)

- **Service** = capacité perçue par l'utilisateur. **Opérateur** = unité d'exécution gouvernée.
- **Règle :** *« Un Service peut être porté par un Opérateur... ou par une Équipe d'Opérateurs. »* Le service « réseau social » (fil, messagerie, découverte, modération) peut être livré par une **Équipe d'Opérateurs** sous **Contrat d'équipe** et **Mandat de Permission**.

### Données et écriture — [Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md)

- **KindMother** : autorité sur toutes les données (posts, réactions, messages, profils sociaux, abonnements, modération). Toute écriture passe par **WriteIntent** sous autorité KindMother.
- **StrongFather** : décision ALLOW/DENY (publication, visibilité, modération, blocage). N'exécute jamais.

---

## Portée / Scope

**Ce document définit :**

- L'**analyse PR (Product Review)** des réseaux sociaux dominants : périmètre fonctionnel et capacités transverses.
- La **cartographie** réseaux sociaux → Outils, Opérateurs, Services Miyukini.
- Les **Kits et Outils** déjà couverts ou à créer pour un service social gouverné.
- Les **Opérateurs** (Domaine, Interface) à déployer et les **Cores** impliqués.

**Hors scope :**

- L'implémentation technique détaillée (APIs tierces, algorithmes de recommandation).
- Les contrats d'intégration par outil (voir documentations fondatrices des Tools).
- La stratégie commerciale ou marketing des services sociaux.

**Statut :** Document de référence normatif — source de vérité pour la conception des équivalents réseaux sociaux Miyukini.

---

## 1. Analyse PR — Réseaux sociaux dominants

### 1.1 Périmètre analysé

| Réseau | Type dominant | Capacités principales retenues |
|--------|----------------|-------------------------------|
| **Facebook** | Réseau généraliste | Fil d'actualité, groupes, pages, messagerie (Messenger), événements, stories, réactions, partages, commentaires |
| **Instagram** | Visuel / court | Posts photo/vidéo, stories, reels, DMs, explore, hashtags, abonnés/abonnements |
| **X (Twitter)** | Micro-blog / réel | Tweets, threads, DMs, espaces (audio), tendances, likes, retweets, citations |
| **LinkedIn** | Professionnel | Profil pro, réseau (connexions), offres d'emploi, contenu (posts, articles), messagerie, groupes |
| **TikTok** | Vidéo courte | Courtes vidéos, FYP (For You), duets, commentaires, likes, abonnements |
| **YouTube** | Vidéo longue | Vidéos, chaînes, abonnements, commentaires, shorts, tendances |
| **Messageries sociales** (WhatsApp, Telegram, Signal) | Messagerie | Conversations 1-1, groupes, statut éphémère, médias, réactions |

### 1.2 Synthèse des capacités transverses

Les capacités ci-dessous sont **transverses** à plusieurs réseaux ; elles constituent le périmètre fonctionnel à couvrir en Miyukini COG.

| Domaine fonctionnel | Capacités | Réseaux typiques |
|---------------------|-----------|------------------|
| **Fil / flux** | Publication (post, tweet, reel, vidéo), liste chronologique ou algorithmique, pagination, filtres | Facebook, Instagram, X, LinkedIn, TikTok, YouTube |
| **Réactions et engagement** | Like, réaction (emoji), partage, retweet, citation, commentaire | Tous |
| **Contenu éphémère** | Story, statut 24h, expiration | Facebook, Instagram, WhatsApp, Telegram |
| **Messagerie sociale** | DM, conversation 1-1, groupe de discussion, pièces jointes, réactions | Facebook (Messenger), Instagram, X, LinkedIn, WhatsApp, Telegram |
| **Profil social** | Bio, avatar, liens, abonnés / abonnements, liste suivis | Instagram, X, LinkedIn, TikTok, YouTube |
| **Découverte** | Explore, tendances, hashtags, recommandations (FYP, suggestions) | Instagram, X, TikTok, YouTube |
| **Médias** | Upload photo/vidéo, transformation (miniature, format), galerie | Tous |
| **Modération** | Signalement, modération contenu, blocage utilisateur, règles de communauté | Tous |
| **Notifications** | Abonnement créateur/compte, alertes (like, comment, DM), préférences | Tous |

---

## 2. Équivalents déjà couverts par le projet Miyukini

Les Kits et Tools suivants **existent déjà** dans [docs/tools/](../tools/_index.md) et couvrent une partie des besoins réseaux sociaux :

| Fonctionnalité sociale | Équivalent Miyukini existant | Détail |
|-----------------------|------------------------------|--------|
| **Contenu éditorial / posts** | **MiyuCMS** | `tool.content.create`, `tool.content.update`, `tool.content.publish`, révisions |
| **Commentaires** | **MiyuCMS** | `tool.content.comment.create`, `tool.content.comment.list`, `tool.content.comment.moderate` |
| **Médias (photo, vidéo)** | **MiyuCMS** + **MiyuMedia** | `tool.media.upload`, `tool.media.serve`, `tool.media.transform` |
| **Identité / connexion** | **MiyuAuth** | `tool.identity.resolve`, `tool.identity.attest`, `tool.identity.verify`, `tool.identity.role` |
| **Rôles / permissions** | **Master Butler** + **StrongFather** | Décision ALLOW/DENY = StrongFather ; catalogue = Master Butler |
| **Affichage web / UI** | **MiyuWeb**, **MiyuWidgets** | Rendu HTML, layout, thème, formulaires |
| **Données / requêtes** | **MiyuSQL** | Requêtes, transactions, cache |
| **Notifications** | **MiyuNotify** | Envoi, préférences, canaux |
| **Recherche** | **MiyuSearch** | Recherche fulltext, filtres |
| **Horloge** | **MiyuClock** | Instant présent, delta (LOI-4) |

**Invariant :** Aucun de ces Kits ne fournit la **structure sociale dédiée** : fil d'actualité (flux agrégé multi-sources), réactions (like, partage, retweet), stories (éphémère 24h), messagerie sociale (DMs, conversations), profil social (abonnés/abonnements), découverte (tendances, hashtags, recommandations), ni modération sociale (signalement, blocage). Ces capacités sont à modéliser en **Outils et Opérateurs sociaux** dédiés.

---

## 3. Cartographie réseaux sociaux → Miyukini COG

### 3.1 Fil d'actualité et flux

| Fonctionnalité réseau social | Équivalent Miyukini | Type | Détail |
|-----------------------------|---------------------|------|--------|
| **Publication (post, tweet, reel)** | Tools social | Tools | `tool.social.post.create`, `tool.social.post.update`, `tool.social.post.delete`, `tool.social.post.resolve` ; persistance = KindMother ; autorisation = StrongFather. |
| **Liste fil (chronologique / algorithme)** | Tools social | Tools | `tool.social.feed.list` (source, tri, pagination, filtres fournis) ; pas de décision métier dans le Tool — politique de tri = Cores. |
| **Réactions (like, emoji)** | Tools social | Tools | `tool.social.reaction.add`, `tool.social.reaction.remove`, `tool.social.reaction.list` ; autorisation = StrongFather. |
| **Partage / retweet / citation** | Tools social | Tools | `tool.social.share.create`, `tool.social.share.list`, `tool.social.share.delete` ; type (partage simple, citation) fourni ; autorisation = StrongFather. |
| **Commentaires (déjà partiel)** | MiyuCMS + extension | Tools | Réutilisation `tool.content.comment.*` avec type « commentaire social » ou `tool.social.comment.create/list` ; gouvernance = StrongFather. |
| **Pagination / curseur** | Paramètres flux | Flux | `tool.social.feed.list` (cursor, limit) ; pas de Tool dédié. |

### 3.2 Contenu éphémère (stories, statut 24h)

| Fonctionnalité réseau social | Équivalent Miyukini | Type | Détail |
|-----------------------------|---------------------|------|--------|
| **Publier une story** | Tools social | Tools | `tool.social.story.create` ; TTL (ex. 24h) fourni ; persistance = KindMother ; autorisation = StrongFather. |
| **Liste stories (comptes suivis)** | Tools social | Tools | `tool.social.story.list` (filtre auteur, non expirées) ; expiration = politique ou KindMother. |
| **Expiration / purge** | Données + politique | Core | Règle d'expiration (KindMother ou job Ever Buddy) ; pas de Tool métier dédié. |
| **Réaction à une story** | Tools social | Tools | `tool.social.reaction.add` (cible = story) ou `tool.social.story.reaction.add`. |

### 3.3 Messagerie sociale (DMs, conversations)

| Fonctionnalité réseau social | Équivalent Miyukini | Type | Détail |
|-----------------------------|---------------------|------|--------|
| **Envoyer DM** | Tools messaging social | Tools | `tool.social.dm.send` ; destinataire(s), contenu, pièces jointes fournis ; autorisation = StrongFather. |
| **Conversation 1-1 ou groupe** | Tools messaging | Tools | `tool.social.conversation.list`, `tool.social.conversation.resolve`, `tool.social.conversation.create` (groupe) ; persistance = KindMother. |
| **Liste messages d'une conversation** | Tools messaging | Tools | `tool.social.dm.list` (conversation_id, pagination) ; autorisation = StrongFather. |
| **Réaction à un message** | Tools messaging | Tools | `tool.social.dm.reaction.add`, `tool.social.dm.reaction.remove` ; autorisation = StrongFather. |
| **Statut « vu »** | Données + Tools | Core + Tools | `tool.social.dm.readmark.update`, `tool.social.dm.readmark.list` ; persistance = KindMother. |
| **Blocage conversation / utilisateur** | Tools modération | Tools | `tool.social.block.add`, `tool.social.block.remove`, `tool.social.block.list` ; décision = StrongFather. |

### 3.4 Profil social (bio, abonnés, abonnements)

| Fonctionnalité réseau social | Équivalent Miyukini | Type | Détail |
|-----------------------------|---------------------|------|--------|
| **Profil social (bio, liens, avatar)** | Toolkit profil + MiyuAuth | Tools | `tool.profile.get`, `tool.profile.update` (voir Équivalents Forum) ; ou `tool.social.profile.get`, `tool.social.profile.update` ; persistance = KindMother. |
| **Abonnement (follow)** | Tools social | Tools | `tool.social.follow.add`, `tool.social.follow.remove`, `tool.social.follow.list`, `tool.social.followers.list`, `tool.social.following.list` ; autorisation = StrongFather. |
| **Liste abonnés / abonnements** | Tools social | Tools | `tool.social.followers.list`, `tool.social.following.list` (pagination) ; pas de décision dans le Tool. |

### 3.5 Découverte (explore, tendances, hashtags)

| Fonctionnalité réseau social | Équivalent Miyukini | Type | Détail |
|-----------------------------|---------------------|------|--------|
| **Hashtags** | Tools social | Tools | `tool.social.hashtag.resolve`, `tool.social.hashtag.trending.list`, `tool.social.post.list` (filter by hashtag) ; données = KindMother. |
| **Tendances (trending)** | Tools social | Tools | `tool.social.trending.list` (région, catégorie fournis) ; politique de calcul = Cores ; Tool applique. |
| **Explore / recommandations** | Tools social | Tools | `tool.social.discover.list` (critères fournis par politique) ; pas de logique algorithmique dans le Tool — paramètres = flux gouverné. |
| **Recherche sociale** | MiyuSearch + extension | Tools | `tool.search.fulltext` (scope = social posts, users) ou `tool.social.search` ; filtres = flux. |

### 3.6 Modération et signalement

| Fonctionnalité réseau social | Équivalent Miyukini | Type | Détail |
|-----------------------------|---------------------|------|--------|
| **Signalement (report)** | Tools modération | Tools | `tool.moderation.report.create`, `tool.moderation.report.list` (voir Équivalents Forum) ; cible = post, story, message, utilisateur ; décision = StrongFather. |
| **Blocage utilisateur** | Tools social | Tools | `tool.social.block.add`, `tool.social.block.remove`, `tool.social.block.list` ; décision = StrongFather. |
| **Modération contenu (masquer, supprimer)** | MiyuCMS modération + StrongFather | Tools + Core | `tool.content.update` (visibility) ou `tool.social.post.delete` ; décision = StrongFather. |
| **Règles de communauté** | Politique | Core | Border Guard / StrongFather ; pas de Tool dédié. |

### 3.7 Notifications et abonnements

| Fonctionnalité réseau social | Équivalent Miyukini | Type | Détail |
|-----------------------------|---------------------|------|--------|
| **Abonnement notifications (créateur, compte)** | MiyuNotify | Tools | `tool.notify.subscribe` (cible = compte, type = new_post, comment, etc.) ; déjà couvert. |
| **Préférences notifications** | MiyuNotify | Tools | `tool.notify.preferences.get`, `tool.notify.preferences.set` ; déjà couvert. |
| **Liste notifications (activité)** | MiyuNotify | Tools | `tool.notify.list` (non lues, par type) ; déjà couvert. |

---

## 4. Équivalents manquants — Kits et Tools à créer

Les éléments suivants **ne sont pas encore présents** dans la liste des Tools du projet ([docs/tools/_index.md](../tools/_index.md)). Ils sont proposés pour couvrir un service social gouverné.

### 4.1 Synthèse — Kits d'outils (Toolkits) proposés

| ToolkitId proposé | Domaine | Composition (résumé) | Usage principal |
|-------------------|---------|----------------------|------------------|
| `toolkit.social.feed` | social | tool.social.post.*, tool.social.feed.list, tool.social.reaction.*, tool.social.share.*, tool.social.comment.* (ou content.comment) | Fil d'actualité : publication, flux, réactions, partages, commentaires |
| `toolkit.social.story` | social | tool.social.story.create, tool.social.story.list, tool.social.story.reaction.add | Contenu éphémère (stories 24h) |
| `toolkit.social.messaging` | social | tool.social.dm.send, tool.social.dm.list, tool.social.conversation.*, tool.social.dm.reaction.*, tool.social.dm.readmark.* | Messagerie sociale (DMs, conversations) |
| `toolkit.social.profile` | social | tool.social.profile.get/update, tool.social.follow.*, tool.social.followers.list, tool.social.following.list | Profil social, abonnés, abonnements |
| `toolkit.social.discovery` | social | tool.social.hashtag.*, tool.social.trending.list, tool.social.discover.list, tool.social.search (ou search.fulltext scope=social) | Découverte : hashtags, tendances, explore |
| `toolkit.social.moderation` | social | tool.moderation.report.*, tool.social.block.*, tool.social.post.delete (visibility) | Signalement, blocage, modération contenu |

**Invariant :** Chaque Toolkit contient au moins deux Tools. Les Toolkits sont validés par Ever Buddy et déclarés au Master Butler.

### 4.2 Synthèse — Outils (Tools) proposés par domaine

Format ToolId : `tool.social.<sous-domaine>.<action>` (conforme Master Butler).

#### 4.2.1 Fil et posts (feed, post)

| ToolId | Action courte |
|--------|----------------|
| `tool.social.post.create` | Crée un post (contenu, visibilité, pièces jointes fournis) |
| `tool.social.post.update` | Met à jour un post |
| `tool.social.post.delete` | Supprime ou masque un post ; autorisation = StrongFather |
| `tool.social.post.resolve` | Retourne un post par identifiant |
| `tool.social.post.list` | Liste les posts (filtres : auteur, hashtag, visibilité) |
| `tool.social.feed.list` | Liste le fil d'actualité (sources, tri, pagination fournis) |
| `tool.social.reaction.add` | Ajoute une réaction (like, emoji) à un post ou story |
| `tool.social.reaction.remove` | Retire une réaction |
| `tool.social.reaction.list` | Liste les réactions d'une cible |
| `tool.social.share.create` | Crée un partage (simple ou avec citation) |
| `tool.social.share.list` | Liste les partages d'un post |
| `tool.social.share.delete` | Supprime un partage |
| `tool.social.comment.create` | Crée un commentaire sur un post (ou réutilisation content.comment) |
| `tool.social.comment.list` | Liste les commentaires d'un post |

#### 4.2.2 Stories (contenu éphémère)

| ToolId | Action courte |
|--------|----------------|
| `tool.social.story.create` | Crée une story (contenu, TTL fournis) |
| `tool.social.story.list` | Liste les stories (auteur, non expirées) |
| `tool.social.story.reaction.add` | Ajoute une réaction à une story |

#### 4.2.3 Messagerie sociale (dm, conversation)

| ToolId | Action courte |
|--------|----------------|
| `tool.social.dm.send` | Envoie un message (conversation, contenu, pièces jointes) |
| `tool.social.dm.list` | Liste les messages d'une conversation |
| `tool.social.dm.resolve` | Retourne un message par identifiant |
| `tool.social.conversation.create` | Crée une conversation (1-1 ou groupe) |
| `tool.social.conversation.list` | Liste les conversations de l'utilisateur |
| `tool.social.conversation.resolve` | Retourne une conversation par identifiant |
| `tool.social.dm.reaction.add` | Ajoute une réaction à un message |
| `tool.social.dm.reaction.remove` | Retire une réaction |
| `tool.social.dm.readmark.update` | Marque des messages comme lus |
| `tool.social.dm.readmark.list` | Liste les marques de lecture (conversation) |

#### 4.2.4 Profil social et suivi (profile, follow)

| ToolId | Action courte |
|--------|----------------|
| `tool.social.profile.get` | Retourne le profil social (bio, liens, avatar, compteurs) |
| `tool.social.profile.update` | Met à jour le profil social |
| `tool.social.follow.add` | Abonne l'utilisateur courant à un compte |
| `tool.social.follow.remove` | Désabonne |
| `tool.social.follow.list` | Liste les abonnements (avec statut) |
| `tool.social.followers.list` | Liste les abonnés d'un compte |
| `tool.social.following.list` | Liste les comptes suivis par un utilisateur |

#### 4.2.5 Découverte (hashtag, trending, discover)

| ToolId | Action courte |
|--------|----------------|
| `tool.social.hashtag.resolve` | Retourne un hashtag par libellé |
| `tool.social.hashtag.trending.list` | Liste les hashtags tendance |
| `tool.social.trending.list` | Liste les tendances (posts, sujets ; critères fournis) |
| `tool.social.discover.list` | Liste les contenus « explore » (paramètres fournis par politique) |
| `tool.social.search` | Recherche sociale (posts, comptes, hashtags) ; ou scope=social sur tool.search.fulltext |

#### 4.2.6 Modération et blocage (block)

| ToolId | Action courte |
|--------|----------------|
| `tool.social.block.add` | Bloque un utilisateur (ou conversation) ; décision = StrongFather |
| `tool.social.block.remove` | Débloque |
| `tool.social.block.list` | Liste les utilisateurs bloqués |

*(Signalement : réutilisation `tool.moderation.report.create`, `tool.moderation.report.list`.)*

---

## 5. Opérateurs proposés pour un service réseau social

| Opérateur | Type | Service perçu | Tools principaux |
|-----------|------|----------------|-------------------|
| **Fil d'actualité** | Domaine | Publication, flux, réactions, partages, commentaires | tool.social.post.*, tool.social.feed.list, tool.social.reaction.*, tool.social.share.*, tool.social.comment.* |
| **Stories** | Domaine | Contenu éphémère (24h) | tool.social.story.* |
| **Messagerie sociale** | Domaine | DMs, conversations, réactions, lu | tool.social.dm.*, tool.social.conversation.* |
| **Profil social** | Domaine | Bio, abonnés, abonnements | tool.social.profile.*, tool.social.follow.* |
| **Découverte** | Domaine | Tendances, hashtags, explore, recherche | tool.social.hashtag.*, tool.social.trending.list, tool.social.discover.list, tool.social.search |
| **Modération sociale** | Domaine | Signalement, blocage, modération contenu | tool.moderation.report.*, tool.social.block.*, tool.social.post.delete |
| **Interface Fil** | Interface | Affichage fil, publication, réactions | MiyuWeb + tool.social.feed.*, tool.social.post.* |
| **Interface Messagerie** | Interface | Affichage conversations, envoi DMs | MiyuWeb + tool.social.dm.*, tool.social.conversation.* |
| **Interface Profil** | Interface | Affichage profil, abonnés, paramètres | MiyuWeb + tool.social.profile.*, tool.social.follow.* |

*Identité et rôles de base : MiyuAuth + Master Butler + StrongFather. Notifications : MiyuNotify.*

---

## 6. Équipes d'Opérateurs et Contrats d'équipe

Pour délivrer un **Service** « réseau social » (fil + stories + messagerie + découverte + modération), on constitue une **Équipe d'Opérateurs** liée par un **Contrat d'équipe**. L'équipe n'existe opérationnellement que sous un **Mandat de Permission** émis par StrongFather.

### 6.1 Exemple — Équipe « Réseau social »

| Membre | Rôle dans l'équipe |
|--------|--------------------|
| Opérateur Fil d'actualité | Fournit publication, flux, réactions, partages, commentaires |
| Opérateur Stories | Fournit contenu éphémère (création, liste, réactions) |
| Opérateur Messagerie sociale | Fournit DMs, conversations, marques de lecture |
| Opérateur Profil social | Fournit profil, abonnés, abonnements |
| Opérateur Découverte | Fournit tendances, hashtags, explore, recherche |
| Opérateur Modération sociale | Gère signalements, blocages, masquage/suppression contenu |
| Opérateur Interface Fil | Expose l'UI fil, publication, réactions |
| Opérateur Interface Messagerie | Expose l'UI conversations et DMs |
| Opérateur Interface Profil | Expose l'UI profil et paramètres |

**Flux autorisés (exemple) :** Interface Fil → Fil d'actualité (lecture/écriture posts, réactions) ; Interface Fil → Profil social (lecture avatar, bio) ; Modération sociale → Fil (delete, visibility) ; Interface Messagerie → Messagerie sociale (lecture/envoi). Pas de communication directe entre Opérateurs sans BondingBrother et Mandat.

**Contrat d'équipe :** Définit statiquement les membres, les flux, les types de données échangeables, les conditions préalables et le niveau de validation requis. Validé par StrongFather.

---

## 7. Cores impliqués et flux de gouvernance

Les Cores **ne font jamais d'exécution** ; ils gouvernent, décident ou observent.

| Core | Rôle dans le périmètre réseaux sociaux |
|------|----------------------------------------|
| **KindMother** | Autorité sur toutes les données : posts, stories, messages, réactions, partages, profils sociaux, abonnements, blocages, modération. WriteIntent pour toute écriture. |
| **StrongFather** | Décision finale ALLOW/DENY : publication post/story, réaction, partage, envoi DM, follow, blocage, modération (masquer, supprimer). Émission et révocation des Mandats. Validation des Contrats d'équipe. |
| **Master Butler** | Déclaration des Tools et Toolkits (social.feed, social.story, social.messaging, social.profile, social.discovery, social.moderation). Permissions et capabilities. |
| **BondingBrother** | Médiation des intentions (lecteur, posteur, modérateur, admin) ; traduction vers les Cores et les Opérateurs. |
| **WorrySentinel** | Niveau de sécurité (données personnelles, DMs, modération) ; blocage si menace ou état dégradé. |
| **Caring Nanny** | État système ; blocage des Tools si environnement dégradé. |
| **Ever Buddy** | Cycle de vie : dépréciation Tools/Toolkits, compatibilité versions, expiration stories. |
| **Border Guard** | Frontières et niveaux de confiance ; multi-tenant si espaces multiples. |
| **TAMR** | Points d'intervention humaine (arbitrage modération, déblocage) si définis. |

**Flux générique :** Opérateur → BondingBrother → Master Butler (existence Tool, permissions) → WorrySentinel (niveau sécurité) → Caring Nanny (état système) → StrongFather (ALLOW/DENY) → Exécution Tool ; toute persistance passe par WriteIntent KindMother.

---

## 8. Tableau de correspondance réseau → équivalent Miyukini (résumé)

| Réseau / capacité | Opérateur(s) Miyukini | Toolkit(s) principal(aux) |
|------------------|------------------------|---------------------------|
| Facebook (fil, groupes, Messenger, stories) | Fil d'actualité, Messagerie sociale, Stories | toolkit.social.feed, toolkit.social.messaging, toolkit.social.story |
| Instagram (posts, stories, reels, DMs, explore) | Fil, Stories, Découverte, Messagerie sociale | toolkit.social.feed, toolkit.social.story, toolkit.social.discovery, toolkit.social.messaging |
| X/Twitter (tweets, DMs, tendances) | Fil, Messagerie sociale, Découverte | toolkit.social.feed, toolkit.social.messaging, toolkit.social.discovery |
| LinkedIn (profil, réseau, contenu, messagerie) | Profil social, Fil, Messagerie sociale | toolkit.social.profile, toolkit.social.feed, toolkit.social.messaging |
| TikTok (vidéos, FYP, commentaires) | Fil, Découverte | toolkit.social.feed, toolkit.social.discovery (+ MiyuMedia pour vidéo) |
| YouTube (vidéos, chaînes, abonnements) | Fil (chaîne), Profil social | toolkit.social.feed, toolkit.social.profile (+ MiyuMedia) |
| WhatsApp / Telegram (chat, groupes, statut) | Messagerie sociale, Stories | toolkit.social.messaging, toolkit.social.story |

---

## 9. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Opérateurs et Terminologie | [Miyukini Conceptual References - Operators et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md) |
| Mandats et Équipes Opérateurs | [Miyukini Conceptual References - Mandats et Equipes Operators](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md) |
| Pyramide Architecture Complète | [Miyukini Conceptual References - Pyramide Architecture Complete](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) |
| Équivalents Moteur Forum | [Miyukini Conceptual References - Equivalents Moteur Forum](./Miyukini%20Conceptual%20References%20-%20Equivalents%20Moteur%20Forum.md) |
| Index Tools (docs/tools) | [Tools — Index de navigation](../tools/_index.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence normatif — Équivalents réseaux sociaux dominants
