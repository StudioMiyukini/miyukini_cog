# MiyuStory — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuStory** (`toolkit.social.story`). Chaque outil est une capacité atomique gouvernée ; la décision (création, visibilité) relève de StrongFather, la persistance de KindMother. Cycle de vie (expiration) = Ever Buddy.

**Référence :** [MiyuStory - Documentation Fondatrice](./MiyuStory%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|-----------------|------|
| `tool.social.story.create` | Crée une story | 1–2 | Autorisation StrongFather ; WriteIntent KindMother |
| `tool.social.story.list` | Liste les stories | 0–1 | Lecture ; filtres fournis |
| `tool.social.story.get` | Récupère une story | 0–1 | Lecture |
| `tool.social.story.reaction.add` | Ajoute une réaction à une story | 1 | WriteIntent KindMother |

---

**Invariant :** Décision (création, visibilité) = StrongFather. Expiration 24h = Ever Buddy. Toute écriture = WriteIntent KindMother.
