# MiyuProfile — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuProfile** (`toolkit.identity.profile`). Chaque outil est une capacité atomique gouvernée ; la décision (modification, rangs) relève de StrongFather, la persistance de KindMother.

**Référence :** [MiyuProfile - Documentation Fondatrice](./MiyuProfile%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|-----------------|------|
| `tool.profile.get` | Récupère le profil | 1 | Lecture |
| `tool.profile.update` | Met à jour le profil | 2 | Autorisation StrongFather ; WriteIntent KindMother |
| `tool.profile.field.list` | Liste les champs (schéma) | 0–1 | Lecture |
| `tool.profile.field.get` | Récupère un champ | 1 | Lecture |
| `tool.profile.field.set` | Met à jour un champ | 2 | WriteIntent KindMother |
| `tool.profile.avatar.get` | Récupère l'avatar | 0–1 | Lecture |
| `tool.profile.avatar.set` | Met à jour l'avatar | 2 | WriteIntent KindMother ou MiyuMedia |
| `tool.profile.avatar.resolve` | Résout avatar (Gravatar) | 0–1 | Exécution seule |
| `tool.profile.signature.get` | Récupère la signature | 0–1 | Lecture |
| `tool.profile.signature.set` | Met à jour la signature | 2 | WriteIntent KindMother |
| `tool.profile.rank.list` | Liste les rangs | 0–1 | Lecture ; règles StrongFather |
| `tool.profile.rank.resolve` | Résout le rang utilisateur | 0–1 | Lecture ; règles StrongFather |
| `tool.profile.preferences.get` | Récupère les préférences | 1 | Lecture |
| `tool.profile.preferences.set` | Met à jour les préférences | 2 | WriteIntent KindMother |

---

**Invariant :** Règles d'attribution des rangs = StrongFather. Toute écriture = WriteIntent KindMother.
