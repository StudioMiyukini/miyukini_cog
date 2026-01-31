# MiyuSocialProfile — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuSocialProfile** (`toolkit.social.profile`). Chaque outil est une capacité atomique gouvernée ; la décision (modification, follow) relève de StrongFather, la persistance de KindMother.

**Référence :** [MiyuSocialProfile - Documentation Fondatrice](./MiyuSocialProfile%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|-----------------|------|
| `tool.social.profile.get` | Récupère le profil social | 0–1 | Lecture |
| `tool.social.profile.update` | Met à jour le profil social | 2 | Autorisation StrongFather ; WriteIntent KindMother |
| `tool.social.follow.add` | Ajoute un abonnement (follow) | 1–2 | WriteIntent KindMother |
| `tool.social.follow.remove` | Supprime un abonnement | 1–2 | WriteIntent KindMother |
| `tool.social.followers.list` | Liste les abonnés | 0–1 | Lecture |
| `tool.social.following.list` | Liste les abonnements | 0–1 | Lecture |

---

**Invariant :** Décision (modification, follow autorisé) = StrongFather. Toute écriture = WriteIntent KindMother.
