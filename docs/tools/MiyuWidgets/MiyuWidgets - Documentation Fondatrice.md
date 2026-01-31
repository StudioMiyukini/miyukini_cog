# MiyuWidgets — Documentation Fondatrice

## 1. Contexte

**MiyuWidgets** est le **kit d'outils (Toolkit)** de rendu de blocs et de composition de layout pour l'édition visuelle de pages et de thèmes (page builder) de l'écosystème Miyukini. Il intègre les outils d'application de layout, de rendu de widgets (texte, image, bouton, grille, conteneur) et de résolution de template, complémentaires à MiyuWeb pour l'affichage web.

L'autorité sur les données (templates, structures de layout) appartient à **KindMother** (Core de données, Strate 4). MiyuWidgets expose des capacités d'exécution gouvernée (layout.apply, widget.*.render, template.resolve) sans remplacer KindMother ni MiyuWeb ; les Opérateurs passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils. MiyuWidgets opère sur des **données fournies dans le flux** — il ne lit pas la base directement.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :**

- L'identité et la définition canonique de MiyuWidgets
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliqués)
- Le niveau de sécurité et les états système autorisés ou interdits
- La relation avec MiyuWeb et KindMother (templates, données dans le flux)
- L'alignement avec le protocole MIP v1 pour une future implémentation indexable

**Hors scope :**

- L'implémentation détaillée (moteur de rendu, sandbox)
- Toute décision de contenu ou de logique métier — celle-ci reste du ressort des Opérateurs et des Cores
- Les capacités de base MiyuWeb (html.render, layout.render, theme.resolve, etc.) — MiyuWidgets les complète pour le page builder

---

## 3. Définition canonique

> **MiyuWidgets est une composition officielle d'outils de rendu de blocs et de composition de layout (layout.apply, widgets, template.resolve), déclarée et gouvernée par l'environnement.**

- MiyuWidgets **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuWidgets **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (appliquer une modification de layout, rendre un bloc texte/image/bouton/grille/conteneur, résoudre un template) sans décider du contenu ni accéder à la base.

**Règle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacité. Les capacités viennent exclusivement des Tools composants. Les données (templates, structures) sont fournies dans le flux ; la persistance relève de KindMother.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.web.widgets` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `web` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuWidgets - Reference Outils](./MiyuWidgets%20-%20Reference%20Outils.md). MiyuWidgets est composé des Tools suivants (format canonique `tool.web.<sous-domaine>.<action>`).

| ToolId | Description courte |
|--------|---------------------|
| `tool.web.layout.apply` | Applique une modification de layout (structure) à partir de données fournies ; exécution seule |
| `tool.web.widget.text.render` | Rend un bloc texte à partir de données fournies |
| `tool.web.widget.image.render` | Rend un bloc image à partir de données fournies |
| `tool.web.widget.button.render` | Rend un bloc bouton à partir de données fournies |
| `tool.web.widget.grid.render` | Rend une grille de blocs à partir de données fournies |
| `tool.web.widget.container.render` | Rend un conteneur (section/colonnes) à partir de données fournies |
| `tool.web.template.resolve` | Résout un template (structure) par identifiant ; données fournies dans le flux |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuWidgets en contient sept.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : les Tools opèrent sur des **données fournies dans le flux** (MiyuWidgets ne lit pas la base directement) ; persistance templates/layouts = KindMother. Le Toolkit est déclaré dans Master Butler et compatibilisé par Ever Buddy ([Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)).

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 2** selon politique d'exposition (page builder éditorial) ; cohérent avec WorrySentinel. Le niveau du Toolkit est au moins égal au maximum des niveaux de ses Tools composants. |
| **États autorisés** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon Toolkit Composition Contract) |

---

## 8. Relation avec MiyuWeb et KindMother

Les obligations de conformité détaillées sont dans [MiyuWidgets - Tool Governance Compliance Contract](./contracts/governance/MiyuWidgets%20-%20Tool%20Governance%20Compliance%20Contract.md).

- **MiyuWeb** fournit les capacités de base (html.render, layout.render, theme.resolve, form.validate, event.dispatch, input.capture, etc.). MiyuWidgets **complète** MiyuWeb pour l'édition visuelle de pages et de thèmes (widgets, layout.apply, template.resolve). L'Opérateur d'Interface « Éditeur de pages / thème » utilise MiyuWeb + MiyuWidgets.
- **KindMother** est l'autorité sur les données (templates, structures de layout) ; ces données sont fournies dans le flux à MiyuWidgets, qui ne lit pas la base directement.
- Les Tools MiyuWidgets opèrent sur des **données fournies dans le flux** ; la persistance des templates et layouts relève de KindMother (et éventuellement MiyuSQL sous mandat KindMother).

**Référence :** [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md), [MiyuWeb - Documentation Fondatrice](../MiyuWeb/MiyuWeb%20-%20Documentation%20Fondatrice.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuWidgets sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). Référence : [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

- **Domaine** : `web` — cohérent avec la projection domains.json (blocs du domaine « web »).
- **Layer** : outil / toolkit (Strate 6) — à refléter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuWidgets est une unité logique pouvant devenir un **bloc MSCM** à l'implémentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Équivalents Boutique CMS Reservation SaaS | [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md) |
| MiyuWeb - Documentation Fondatrice | [MiyuWeb - Documentation Fondatrice](../MiyuWeb/MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
