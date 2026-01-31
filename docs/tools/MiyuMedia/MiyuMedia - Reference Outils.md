# MiyuMedia — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuMedia** (`toolkit.content.media`). Chaque outil est une capacité atomique gouvernée ; persistance des médias = WriteIntent KindMother.

**Référence :** [MiyuMedia - Documentation Fondatrice](./MiyuMedia%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|------------------|------|
| `tool.media.upload` | Enregistrer un média | 1–2 | À partir du flux ; persistance KindMother |
| `tool.media.serve` | Servir un média (stream ou métadonnées) | 0–1 | Données fournies dans le flux |
| `tool.media.transform` | Produire une variante (miniature, recadrage) | 0–1 | Données fournies ; pas de décision politique |

---

**Invariant :** Toute écriture (upload, métadonnées) = **WriteIntent** vers KindMother. Le kit ne décide pas de la politique de stockage ni des quotas (StrongFather / Cores).
