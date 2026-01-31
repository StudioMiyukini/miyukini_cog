# MiyuDiscovery — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuDiscovery** (`toolkit.social.discovery`). Chaque outil est une capacité atomique gouvernée ; la décision (politique explore, tendances) relève de StrongFather, la lecture de KindMother.

**Référence :** [MiyuDiscovery - Documentation Fondatrice](./MiyuDiscovery%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|-----------------|------|
| `tool.social.hashtag.list` | Liste les hashtags | 0–1 | Lecture |
| `tool.social.hashtag.get` | Récupère un hashtag et ses posts | 0–1 | Lecture |
| `tool.social.hashtag.trending` | Liste les hashtags tendance | 0–1 | Lecture ; politique StrongFather |
| `tool.social.trending.list` | Liste les tendances (posts, sujets) | 0–2 | Lecture ; politique StrongFather |
| `tool.social.discover.list` | Liste le contenu explore | 0–2 | Lecture ; filtres fournis |
| `tool.social.search` | Recherche sociale (scope=social) | 0–2 | Lecture ; ou MiyuSearch scope=social |

---

**Invariant :** Décision (politique explore, tendances) = StrongFather. Lecture = KindMother.
