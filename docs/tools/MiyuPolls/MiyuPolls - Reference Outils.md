# MiyuPolls — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuPolls** (`toolkit.content.polls`). Chaque outil est une capacité atomique gouvernée ; la décision (création, vote) relève de StrongFather, la persistance de KindMother.

**Référence :** [MiyuPolls - Documentation Fondatrice](./MiyuPolls%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|-----------------|------|
| `tool.poll.create` | Crée un sondage | 2 | Autorisation StrongFather ; WriteIntent KindMother |
| `tool.poll.vote` | Enregistre un vote | 1–2 | Autorisation StrongFather ; WriteIntent KindMother |
| `tool.poll.list` | Liste les sondages | 0–1 | Lecture |
| `tool.poll.result` | Récupère les résultats (agrégés) | 0–1 | Lecture ; pas d'écriture |

---

**Invariant :** Décision (création, vote autorisé, clôture) = StrongFather. Toute écriture = WriteIntent KindMother.
