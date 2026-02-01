# Odoo Forum — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique** pour développer l'équivalent Forum dans Miyukini, en s'appuyant sur le toolkit existant **MiyuForum** et les spécifications Opérateurs.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique et relation avec MiyuForum (toolkit)
- Spécifications des crates Rust (opérateurs vs tools)
- Schémas de données (forum, post, tag, karma)
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## 1. Architecture Technique

### 1.1 Positionnement MiyuForum vs Opérateurs

**MiyuForum (existant)** : **Kit d'outils** (Strate 6) — tools uniquement, pas de décision métier.
- `board` (list, get, create, update)
- `category` (list, get, create, update)
- `topic` (create, list, get, update, sticky, announce, export_pdf, export_text)
- `post` (create, list, get, update)
- `readtrack` (list, mark)

**Opérateurs à développer (Strate 7)** : consomment les tools MiyuForum sous gouvernance et exposent le **service** Forum (équivalent Odoo).
- ForumOperator, PostOperator, TagOperator, KarmaOperator, ModerationOperator, ForumUI

**Principe :** Les opérateurs appellent les tools MiyuForum (board_*, topic_*, post_*, etc.) uniquement après validation StrongFather, Master Butler, KarmaOperator, et soumettent les écritures en WriteIntent à KindMother. Les tools ne décident jamais.

### 1.2 Structure des crates (cible)

```
crates/
├── miyuforum/                      # Existant — toolkit (tools)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── board.rs
│   │   ├── category.rs
│   │   ├── topic.rs
│   │   ├── post.rs
│   │   ├── readtrack.rs
│   │   ├── context.rs
│   │   └── errors.rs
│   └── Cargo.toml
│
├── miyukini-forum/                 # ForumOperator (optionnel si logique dans miyukini-forum-service)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── forum.rs                # Modèle Forum (board), config karma
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-forum-post/            # PostOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── post.rs                 # Modèle Post (question/answer/comment)
│   │   ├── vote.rs
│   │   ├── best_answer.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-forum-tag/             # TagOperator (ou intégré dans miyukini-forum)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── tag.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-forum-karma/           # KarmaOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── karma.rs                # Karma, gains/pertes
│   │   ├── rights.rs               # Seuils, vérification droits
│   │   ├── rank.rs
│   │   ├── badge.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-forum-moderation/      # ModerationOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── validate.rs
│   │   ├── flag.rs
│   │   ├── close.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyukini-forum-ui/              # ForumUI
    ├── src/
    │   ├── lib.rs
    │   ├── views/
    │   │   ├── forum_list.rs
    │   │   ├── post_detail.rs
    │   │   ├── new_post.rs
    │   │   └── moderation.rs
    │   └── admin_cell.rs
    └── Cargo.toml
```

**Option simplifiée (MVP)** : Une seule crate `miyukini-forum-service` regroupant ForumOperator, PostOperator, TagOperator, KarmaOperator, ModerationOperator, puis crate séparée `miyukini-forum-ui`.

### 1.3 Dépendances principales

**Cores Miyukini :**
- `miyukini-kernel`
- `miyukini-central` (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy, TAMR)

**Kits existants :**
- `miyuforum` : tools board, topic, post, category, readtrack
- `miyucontacts` : utilisateurs / partenaires (auteur)
- `miyunotify` : notifications (suivi, nouvelle réponse)
- `miyauth` : identité, session
- `miyuweb` : pages, routes (exposition forum sur le site)
- `miyumoderationforum` : possible réutilisation pour règles modération (si existant)

---

## 2. Schémas de Données

### 2.1 Forum (Board)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Forum {
    pub id: ForumId,
    pub name: String,
    pub mode: ForumMode,           // Questions | Discussions
    pub default_sort: DefaultSort,  // Newest | LastUpdated | MostVoted | Relevance | Answered
    pub privacy: ForumPrivacy,     // Public | SignedIn | SomeUsers
    pub authorized_group_id: Option<GroupId>,
    pub karma_gain_ids: Vec<KarmaGainId>,
    pub karma_right_ids: Vec<KarmaRightId>,
    pub active: bool,
}
```

### 2.2 Post

```rust
pub enum PostType { Question, Answer, Comment }

pub enum PostState { Draft, PendingValidation, Published, Closed, FlaggedOffensive, Deleted }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: PostId,
    pub forum_id: ForumId,
    pub parent_post_id: Option<PostId>,
    pub post_type: PostType,
    pub author_id: UserId,
    pub title: Option<String>,
    pub content: String,
    pub tag_ids: Vec<TagId>,
    pub state: PostState,
    pub best_answer_id: Option<PostId>,
    pub vote_count: i32,
    pub answer_count: u32,
    pub close_reason_id: Option<CloseReasonId>,
    pub hidden_from_non_moderators: bool,
    pub create_date: DateTime,
    pub write_date: DateTime,
}
```

### 2.3 Tag

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: TagId,
    pub forum_id: ForumId,
    pub name: String,
    pub post_count: u32,
}
```

### 2.4 Karma & Rights

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KarmaProfile {
    pub user_id: UserId,
    pub total_karma: i32,
    pub rank_id: Option<RankId>,
    pub badge_ids: Vec<BadgeId>,
}

pub struct KarmaRight {
    pub id: KarmaRightId,
    pub forum_id: ForumId,
    pub right_key: String,  // ask_questions, answer_questions, upvote, ...
    pub required_karma: i32,
}
```

---

## 3. API et Contrats (résumé)

| Opérateur | Méthodes principales |
|-----------|----------------------|
| ForumOperator | `create_forum`, `update_forum`, `list_forums`, `get_forum` |
| PostOperator | `create_post`, `update_post`, `delete_post`, `vote`, `set_best_answer`, `list_posts`, `get_post` |
| TagOperator | `create_tag`, `update_tag`, `list_tags`, `get_tag` |
| KarmaOperator | `get_karma`, `check_rights`, `apply_gain`, `apply_loss`, `list_ranks`, `grant_badge` |
| ModerationOperator | `validate_post`, `list_pending`, `list_flagged`, `list_closed`, `mark_offensive`, `close_post`, `reopen_post` |
| ForumUI | Rendering only ; appelle les opérateurs via BondingBrother |

---

## 4. Plan de Développement par Phases

### Phase 1 — MVP (2–3 sprints)

- **Forum** : Un forum par site (ou par tenant), mode Questions, privacy Public ou Signed In.
- **Posts** : Questions + réponses (une par utilisateur et par question) + commentaires ; pas de validation préalable (seuil « ask without validation » = 0 ou bas).
- **Tags** : Liste, association à une question (≤ 5), filtrage ; pas de création par utilisateur en MVP.
- **Karma** : Karma simplifié : compteur unique, gains (ask +2, answer accepted +15, upvote +5/+10), pas de perte en MVP ; droits : ask 3, answer 3, upvote 5, best answer 20.
- **Modération** : Pas de « To Validate » ; Flag + Close avec motif simple ; pas de -100 karma en MVP.
- **UI** : Liste questions, détail post (question + réponses + commentaires), New Post, vote, best answer, sidebar tags.

**Livrables :** MiyuForum utilisé par PostOperator/ForumOperator ; une crate miyukini-forum-service (ou 2 : service + karma) ; miyukini-forum-ui (liste + détail + new post).

### Phase 2 — Karma complet et modération

- **Karma** : Pertes (-2 downvote, -100 flag) ; tous les seuils « Karma Related Rights » configurables par forum ; rangs et badges (attribution manuelle).
- **Modération** : To Validate (une question en attente par utilisateur si karma < 100) ; Flagged et Closed ; Close Reasons (Basic / Offensive) ; -100 karma sur offensif/spam.
- **Configuration** : Backend-like pour Forums, Tags, Ranks, Badges, Close Reasons.

**Livrables :** KarmaOperator complet ; ModerationOperator ; vues Moderation (To Validate, Flagged, Closed) ; écrans config.

### Phase 3 — Avancé

- **Badges** : Attribution automatique via challenges (Ever Buddy / gamification).
- **Helpdesk** : Lien post ↔ ticket (MiyuHelpdesk si existant).
- **SEO** : Nofollow sur liens des utilisateurs sous seuil karma.
- **Profil** : Bio détaillée, badges par niveau (Bronze/Silver/Gold), karma visible selon droits.
- **Multi-forums** : Plusieurs forums par site, configuration karma par forum.

---

## 5. Bornage Fonctionnel

| Fonctionnalité | MVP | Complet |
|----------------|-----|---------|
| Mode Questions / Discussions | Questions uniquement | Les deux |
| Privacy | Public, Signed In | + Some users |
| Tri | Newest, Last Updated | + Most Voted, Relevance, Answered |
| New Post (question) | Oui | Oui |
| Réponse (une par user) | Oui | Oui |
| Commentaires | Oui | Oui |
| Vote up/down | Up uniquement | Les deux |
| Best answer | Oui | Oui |
| Tags (≤ 5) | Oui, liste fixe | Oui + création si karma |
| Karma gains | Ask, Answer accepted, Upvote | Tous (doc Odoo) |
| Karma losses | Non | Downvote, Flag -100 |
| Karma rights (seuils) | Fixes (3, 5, 20) | Configurables par forum |
| Validation préalable | Non | Oui (1 post en attente/user) |
| Modération (Validate / Flag / Close) | Close simple | To Validate, Flagged, Closed, Close Reasons |
| Rangs / Badges | Non | Oui (manuels puis challenges) |
| Helpdesk link | Non | Oui si MiyuHelpdesk |
| ForumUI | Liste + détail + New Post | + Moderation + Config |

---

## 6. Alignement avec MiyuForum existant

- **Board** ↔ Forum (conteneur) ; `board_list`, `board_get`, `board_create`, `board_update` utilisés par ForumOperator.
- **Topic** ↔ Post de type Question (sujet) ; `topic_create`, `topic_list`, `topic_get`, `topic_update`, `topic_sticky`, `topic_announce` utilisés par PostOperator (sticky/announce = StrongFather).
- **Post** ↔ Réponses et commentaires ; `post_create`, `post_list`, `post_get`, `post_update`.
- **Category** ↔ Optionnel (groupement de forums) ; `category_*` si multi-forums avec catégories.
- **Readtrack** ↔ Suivi lu ; `readtrack_list`, `readtrack_mark` pour « non lus ».

Toute écriture (create/update) sur board, topic, post doit passer par WriteIntent KindMother et décision StrongFather ; les tools MiyuForum sont appelés avec un `GovernedContext` et un Mandat valide.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
