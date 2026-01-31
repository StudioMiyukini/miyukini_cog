# MiyuModerationForum — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuModerationForum** (`toolkit.moderation.forum`). Chaque outil est une capacité atomique gouvernée ; **toute décision de modération = StrongFather**, la persistance de KindMother.

**Référence :** [MiyuModerationForum - Documentation Fondatrice](./MiyuModerationForum%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|-----------------|------|
| `tool.moderation.queue.list` | Liste la file d'attente | 2–3 | Lecture |
| `tool.moderation.queue.get` | Récupère un élément | 2–3 | Lecture |
| `tool.moderation.report.create` | Crée un signalement | 2 | WriteIntent KindMother |
| `tool.moderation.report.list` | Liste les signalements | 2–3 | Lecture |
| `tool.forum.topic.lock` | Verrouille un topic | 3 | Décision StrongFather ; WriteIntent KindMother |
| `tool.forum.topic.move` | Déplace un topic | 3 | Décision StrongFather ; WriteIntent KindMother |
| `tool.forum.topic.merge` | Fusionne des topics | 3 | Décision StrongFather ; WriteIntent KindMother |
| `tool.forum.topic.split` | Scinde un topic | 3 | Décision StrongFather ; WriteIntent KindMother |
| `tool.forum.topic.delete` | Supprime un topic | 3 | Décision StrongFather ; WriteIntent KindMother |
| `tool.forum.topic.copy` | Copie un topic | 3 | Décision StrongFather ; WriteIntent KindMother |
| `tool.forum.post.edit` | Édite un post (modération) | 3 | Décision StrongFather ; WriteIntent KindMother |
| `tool.forum.post.lock` | Verrouille un post | 3 | Décision StrongFather ; WriteIntent KindMother |
| `tool.forum.post.delete` | Supprime un post | 3 | Décision StrongFather ; WriteIntent KindMother |
| `tool.moderation.warning.create` | Crée un avertissement | 3 | Décision StrongFather ; WriteIntent KindMother |
| `tool.moderation.warning.list` | Liste les avertissements | 2–3 | Lecture |
| `tool.moderation.ban.create` | Crée un bannissement | 3 | Décision StrongFather ; WriteIntent KindMother |
| `tool.moderation.ban.list` | Liste les bannissements | 2–3 | Lecture |
| `tool.moderation.usernote.create` | Crée une note modérateur | 3 | WriteIntent KindMother |
| `tool.moderation.usernote.list` | Liste les notes | 2–3 | Lecture |

---

**Invariant :** Toute décision de modération = StrongFather. Toute écriture = WriteIntent KindMother.
