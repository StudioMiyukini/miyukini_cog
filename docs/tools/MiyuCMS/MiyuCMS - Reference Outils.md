# MiyuCMS — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuCMS** (`toolkit.content.cms`). Chaque outil est une capacité atomique gouvernée ; décision (publication, modération) = StrongFather ; persistance (contenus, médias, révisions, commentaires) = WriteIntent KindMother.

**Référence :** [MiyuCMS - Documentation Fondatrice](./MiyuCMS%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|------------------|------|
| `tool.content.create` | Créer un brouillon de contenu | 1–2 | Données fournies ; WriteIntent KindMother ; pas de décision publication |
| `tool.content.update` | Mettre à jour un contenu | 1–2 | WriteIntent KindMother |
| `tool.content.publish` | Marquer un contenu comme publié | 1–2 | Exécution seule ; décision StrongFather |
| `tool.content.schedule` | Planifier une publication | 1–2 | Date/heure fournie ; WriteIntent KindMother |
| `tool.content.revision.list` | Lister les révisions d'un contenu | 0–1 | Lecture gouvernée |
| `tool.content.revision.restore` | Restaurer une révision | 1–2 | Exécution ; décision StrongFather ; WriteIntent KindMother |
| `tool.content.revision.compare` | Comparer deux révisions | 0–1 | Lecture seule |
| `tool.content.comment.create` | Créer un commentaire | 0–1 | Données fournies ; WriteIntent KindMother |
| `tool.content.comment.moderate` | Appliquer modération (approuver, rejeter) | 1–2 | Décision StrongFather ; WriteIntent KindMother |
| `tool.content.comment.list` | Lister les commentaires d'un contenu | 0–1 | Filtres fournis dans le flux |
| `tool.media.upload` | Enregistrer un média | 1–2 | Persistance KindMother |
| `tool.media.serve` | Servir un média (stream ou métadonnées) | 0–1 | Données fournies dans le flux |
| `tool.media.transform` | Produire une variante (miniature, recadrage) | 0–1 | Données fournies ; pas de persistance directe |

---

**Invariant :** Toute écriture (contenu, média, révision, commentaire) = **WriteIntent** vers KindMother. Décision (publication, modération) = StrongFather.
