# MiyuBookmarks — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuBookmarks** (`toolkit.content.bookmarks`). Chaque outil est une capacité atomique gouvernée ; la décision (ajout autorisé) relève de StrongFather, la persistance de KindMother.

**Référence :** [MiyuBookmarks - Documentation Fondatrice](./MiyuBookmarks%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|-----------------|------|
| `tool.bookmark.add` | Ajoute un signet (cible fournie) | 1 | Autorisation StrongFather ; WriteIntent KindMother |
| `tool.bookmark.remove` | Supprime un signet | 1 | WriteIntent KindMother |
| `tool.bookmark.list` | Liste les signets (filtres fournis) | 1 | Lecture |

---

**Invariant :** Décision (ajout autorisé, quota) = StrongFather. Toute écriture = WriteIntent KindMother.
