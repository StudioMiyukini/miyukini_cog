# MiyuForum — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuForum** (`toolkit.community.forum`). Chaque outil est une capacité atomique gouvernée ; la décision métier relève de StrongFather, la persistance de KindMother.

**Référence :** [MiyuForum - Documentation Fondatrice](./MiyuForum%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|-----------------|------|
| `tool.forum.category.list` | Liste les catégories | 0–1 | Lecture |
| `tool.forum.category.get` | Récupère une catégorie | 0–1 | Lecture |
| `tool.forum.category.create` | Crée une catégorie | 2 | WriteIntent KindMother |
| `tool.forum.category.update` | Met à jour une catégorie | 2 | WriteIntent KindMother |
| `tool.forum.board.list` | Liste les forums/boards | 0–1 | Lecture |
| `tool.forum.board.get` | Récupère un board | 0–1 | Lecture |
| `tool.forum.board.create` | Crée un board | 2 | WriteIntent KindMother |
| `tool.forum.board.update` | Met à jour un board | 2 | WriteIntent KindMother |
| `tool.forum.topic.create` | Crée un topic | 1–2 | WriteIntent KindMother |
| `tool.forum.topic.list` | Liste les topics (filtres fournis) | 0–1 | Lecture |
| `tool.forum.topic.get` | Récupère un topic | 0–1 | Lecture |
| `tool.forum.topic.update` | Met à jour un topic | 2 | WriteIntent KindMother |
| `tool.forum.topic.sticky` | Marque/démarque sticky | 2 | Décision StrongFather |
| `tool.forum.topic.announce` | Marque/démarque annonce | 2 | Décision StrongFather |
| `tool.forum.post.create` | Crée un post | 1–2 | WriteIntent KindMother |
| `tool.forum.post.list` | Liste les posts d'un topic | 0–1 | Lecture |
| `tool.forum.post.get` | Récupère un post | 0–1 | Lecture |
| `tool.forum.post.update` | Met à jour un post | 2 | WriteIntent KindMother |
| `tool.forum.readtrack.mark` | Marque comme lu | 1 | WriteIntent KindMother |
| `tool.forum.readtrack.list` | Liste le suivi lu (utilisateur/board) | 1 | Lecture |
| `tool.forum.topic.export.pdf` | Exporte un topic en PDF | 1 | Exécution seule |
| `tool.forum.topic.export.text` | Exporte un topic en texte | 0–1 | Exécution seule |

---

**Invariant :** Aucun Tool ne décide de la politique (création, sticky, annonce) ; décision = StrongFather. Toute écriture = WriteIntent KindMother.
