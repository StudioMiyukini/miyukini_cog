# MiyuFeeds — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuFeeds** (`toolkit.content.feeds`). Chaque outil est une capacité atomique gouvernée ; lecture des données = KindMother ; pas d'écriture métier.

**Référence :** [MiyuFeeds - Documentation Fondatrice](./MiyuFeeds%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|-----------------|------|
| `tool.feed.atom.board` | Génère un flux ATOM pour le board | 0–2 | Lecture KindMother ; décision accès = StrongFather |
| `tool.feed.atom.forum` | Génère un flux ATOM pour un forum | 0–2 | Lecture KindMother ; décision accès = StrongFather |
| `tool.feed.atom.topic` | Génère un flux ATOM pour un topic | 0–2 | Lecture KindMother ; décision accès = StrongFather |

---

**Invariant :** Pas d'écriture métier. Décision d'accès au flux = StrongFather. Lecture = KindMother.
