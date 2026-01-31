# MiyuSocialMessaging — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuSocialMessaging** (`toolkit.social.messaging`). Chaque outil est une capacité atomique gouvernée ; la décision d'envoi relève de StrongFather, la persistance de KindMother.

**Référence :** [MiyuSocialMessaging - Documentation Fondatrice](./MiyuSocialMessaging%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|-----------------|------|
| `tool.social.dm.send` | Envoie un DM | 2 | Autorisation StrongFather ; WriteIntent KindMother |
| `tool.social.dm.list` | Liste les messages d'une conversation | 1–2 | Lecture |
| `tool.social.dm.get` | Récupère un message | 1–2 | Lecture |
| `tool.social.conversation.list` | Liste les conversations | 1–2 | Lecture |
| `tool.social.conversation.get` | Récupère une conversation (fil) | 1–2 | Lecture |
| `tool.social.dm.reaction.add` | Ajoute une réaction à un DM | 1 | WriteIntent KindMother |
| `tool.social.dm.reaction.remove` | Supprime une réaction | 1 | WriteIntent KindMother |
| `tool.social.dm.readmark.set` | Marque comme lu | 1–2 | WriteIntent KindMother |

---

**Invariant :** Décision d'envoi = StrongFather. Toute écriture = WriteIntent KindMother.
