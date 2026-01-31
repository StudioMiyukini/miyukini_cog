# MiyuSocialModeration — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuSocialModeration** (`toolkit.social.moderation`). Chaque outil est une capacité atomique gouvernée ; **toute décision de modération = StrongFather**, la persistance de KindMother.

**Référence :** [MiyuSocialModeration - Documentation Fondatrice](./MiyuSocialModeration%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|-----------------|------|
| `tool.moderation.report.create` | Crée un signalement | 2 | WriteIntent KindMother |
| `tool.moderation.report.list` | Liste les signalements | 2–3 | Lecture |
| `tool.social.block.add` | Bloque un utilisateur | 2–3 | Décision StrongFather ; WriteIntent KindMother |
| `tool.social.block.remove` | Débloque un utilisateur | 2–3 | WriteIntent KindMother |
| `tool.social.block.list` | Liste les utilisateurs bloqués | 2 | Lecture |
| `tool.social.post.delete` | Supprime un post (visibilité) | 3 | Décision StrongFather ; WriteIntent KindMother |

---

**Invariant :** Toute décision de modération = StrongFather. Toute écriture = WriteIntent KindMother.
