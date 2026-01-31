# MiyuWidgets — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuWidgets** (`toolkit.web.widgets`). Chaque outil est une capacité atomique gouvernée ; les Tools opèrent sur des **données fournies dans le flux** ; persistance (templates, layouts) = KindMother.

**Référence :** [MiyuWidgets - Documentation Fondatrice](./MiyuWidgets%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|------------------|------|
| `tool.web.layout.apply` | Appliquer une modification de layout | 0–2 | Structure à partir de données fournies ; exécution seule |
| `tool.web.widget.text.render` | Rendre un bloc texte | 0–1 | Données fournies dans le flux |
| `tool.web.widget.image.render` | Rendre un bloc image | 0–1 | Données fournies dans le flux |
| `tool.web.widget.button.render` | Rendre un bloc bouton | 0–1 | Données fournies dans le flux |
| `tool.web.widget.grid.render` | Rendre une grille de blocs | 0–1 | Données fournies dans le flux |
| `tool.web.widget.container.render` | Rendre un conteneur (section/colonnes) | 0–1 | Données fournies dans le flux |
| `tool.web.template.resolve` | Résoudre un template par identifiant | 0–1 | Données fournies dans le flux ; pas de lecture base directe |

---

**Invariant :** Les Tools opèrent sur des **données fournies dans le flux** ; MiyuWidgets ne lit pas la base directement. Persistance des templates et layouts = KindMother.
