# MiyuSocialFeed — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuSocialFeed** (`toolkit.social.feed`). Chaque outil est une capacité atomique gouvernée ; la décision (publication, visibilité) relève de StrongFather, la persistance de KindMother.

**Référence :** [MiyuSocialFeed - Documentation Fondatrice](./MiyuSocialFeed%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|-----------------|------|
| `tool.social.post.create` | Crée une publication | 1–2 | Autorisation StrongFather ; WriteIntent KindMother |
| `tool.social.post.update` | Met à jour une publication | 2 | WriteIntent KindMother |
| `tool.social.post.delete` | Supprime une publication | 2–3 | Décision StrongFather ; WriteIntent KindMother |
| `tool.social.post.get` | Récupère une publication | 0–1 | Lecture |
| `tool.social.feed.list` | Liste le flux | 0–2 | Lecture ; filtres fournis |
| `tool.social.reaction.add` | Ajoute une réaction | 1 | WriteIntent KindMother |
| `tool.social.reaction.remove` | Supprime une réaction | 1 | WriteIntent KindMother |
| `tool.social.reaction.list` | Liste les réactions | 0–1 | Lecture |
| `tool.social.share.create` | Crée un partage | 1–2 | Autorisation StrongFather ; WriteIntent KindMother |
| `tool.social.share.list` | Liste les partages | 0–1 | Lecture |
| `tool.social.comment.create` | Crée un commentaire | 1–2 | WriteIntent KindMother |
| `tool.social.comment.list` | Liste les commentaires | 0–1 | Lecture |
| `tool.social.comment.delete` | Supprime un commentaire | 2 | Décision StrongFather ; WriteIntent KindMother |

---

**Invariant :** Décision (publication, visibilité, suppression) = StrongFather. Toute écriture = WriteIntent KindMother.
