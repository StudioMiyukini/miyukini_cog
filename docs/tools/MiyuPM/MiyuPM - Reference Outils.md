# MiyuPM — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuPM** (`toolkit.communication.pm`). Chaque outil est une capacité atomique gouvernée ; la décision d'envoi relève de StrongFather, la persistance de KindMother.

**Référence :** [MiyuPM - Documentation Fondatrice](./MiyuPM%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|-----------------|------|
| `tool.pm.send` | Envoie un message privé | 2 | Autorisation StrongFather ; WriteIntent KindMother |
| `tool.pm.list` | Liste les messages (dossier, filtres) | 1–2 | Lecture |
| `tool.pm.get` | Récupère un message | 1–2 | Lecture |
| `tool.pm.folder.list` | Liste les dossiers | 1 | Lecture |
| `tool.pm.folder.create` | Crée un dossier | 2 | WriteIntent KindMother |
| `tool.pm.folder.update` | Met à jour un dossier | 2 | WriteIntent KindMother |
| `tool.pm.draft.create` | Crée un brouillon | 2 | WriteIntent KindMother |
| `tool.pm.draft.update` | Met à jour un brouillon | 2 | WriteIntent KindMother |
| `tool.pm.draft.list` | Liste les brouillons | 1–2 | Lecture |
| `tool.pm.conversation.list` | Liste les conversations | 1–2 | Lecture |
| `tool.pm.conversation.get` | Récupère une conversation (fil) | 1–2 | Lecture |
| `tool.pm.export` | Exporte les messages | 2 | Exécution seule ; pas d'écriture métier |

---

**Invariant :** Décision d'envoi = StrongFather. Toute écriture = WriteIntent KindMother.
