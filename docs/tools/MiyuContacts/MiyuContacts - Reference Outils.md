# MiyuContacts — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuContacts** (`toolkit.communication.contacts`). Chaque outil est une capacité atomique gouvernée ; la décision (ajout autorisé) relève de StrongFather, la persistance de KindMother.

**Référence :** [MiyuContacts - Documentation Fondatrice](./MiyuContacts%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|-----------------|------|
| `tool.contacts.friend.add` | Ajoute un ami | 1–2 | Autorisation StrongFather ; WriteIntent KindMother |
| `tool.contacts.friend.remove` | Supprime un ami | 1–2 | WriteIntent KindMother |
| `tool.contacts.friend.list` | Liste les amis | 1 | Lecture |
| `tool.contacts.foe.add` | Ajoute un ennemi | 1–2 | Autorisation StrongFather ; WriteIntent KindMother |
| `tool.contacts.foe.remove` | Supprime un ennemi | 1–2 | WriteIntent KindMother |
| `tool.contacts.foe.list` | Liste les ennemis | 1 | Lecture |
| `tool.contacts.list` | Liste les contacts (type fourni) | 1 | Lecture |

---

**Invariant :** Décision (ajout autorisé) = StrongFather. Toute écriture = WriteIntent KindMother.
