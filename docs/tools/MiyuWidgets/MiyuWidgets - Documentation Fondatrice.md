# MiyuWidgets â€” Documentation Fondatrice

## 1. Contexte

**MiyuWidgets** est le **kit d'outils (Toolkit)** de rendu de blocs et de composition de layout pour l'Ã©dition visuelle de pages et de thÃ¨mes (page builder) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils d'application de layout, de rendu de widgets (texte, image, bouton, grille, conteneur) et de rÃ©solution de template, complÃ©mentaires Ã  MiyuWeb pour l'affichage web.

L'autoritÃ© sur les donnÃ©es (templates, structures de layout) appartient Ã  **KindMother** (Core de donnÃ©es, Strate 4). MiyuWidgets expose des capacitÃ©s d'exÃ©cution gouvernÃ©e (layout.apply, widget.*.render, template.resolve) sans remplacer KindMother ni MiyuWeb ; les OpÃ©rateurs passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils. MiyuWidgets opÃ¨re sur des **donnÃ©es fournies dans le flux** â€” il ne lit pas la base directement.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :**

- L'identitÃ© et la dÃ©finition canonique de MiyuWidgets
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliquÃ©s)
- Le niveau de sÃ©curitÃ© et les Ã©tats systÃ¨me autorisÃ©s ou interdits
- La relation avec MiyuWeb et KindMother (templates, donnÃ©es dans le flux)
- L'alignement avec le protocole MIP v1 pour une future implÃ©mentation indexable

**Hors scope :**

- L'implÃ©mentation dÃ©taillÃ©e (moteur de rendu, sandbox)
- Toute dÃ©cision de contenu ou de logique mÃ©tier â€” celle-ci reste du ressort des OpÃ©rateurs et des Cores
- Les capacitÃ©s de base MiyuWeb (html.render, layout.render, theme.resolve, etc.) â€” MiyuWidgets les complÃ¨te pour le page builder

---

## 3. DÃ©finition canonique

> **MiyuWidgets est une composition officielle d'outils de rendu de blocs et de composition de layout (layout.apply, widgets, template.resolve), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuWidgets **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuWidgets **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (appliquer une modification de layout, rendre un bloc texte/image/bouton/grille/conteneur, rÃ©soudre un template) sans dÃ©cider du contenu ni accÃ©der Ã  la base.

**RÃ¨gle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacitÃ©. Les capacitÃ©s viennent exclusivement des Tools composants. Les donnÃ©es (templates, structures) sont fournies dans le flux ; la persistance relÃ¨ve de KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.web.widgets` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `web` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuWidgets - Reference Outils](./MiyuWidgets%20-%20Reference%20Outils.md). MiyuWidgets est composÃ© des Tools suivants (format canonique `tool.web.<sous-domaine>.<action>`).

| ToolId | Description courte |
|--------|---------------------|
| `tool.web.layout.apply` | Applique une modification de layout (structure) Ã  partir de donnÃ©es fournies ; exÃ©cution seule |
| `tool.web.widget.text.render` | Rend un bloc texte Ã  partir de donnÃ©es fournies |
| `tool.web.widget.image.render` | Rend un bloc image Ã  partir de donnÃ©es fournies |
| `tool.web.widget.button.render` | Rend un bloc bouton Ã  partir de donnÃ©es fournies |
| `tool.web.widget.grid.render` | Rend une grille de blocs Ã  partir de donnÃ©es fournies |
| `tool.web.widget.container.render` | Rend un conteneur (section/colonnes) Ã  partir de donnÃ©es fournies |
| `tool.web.template.resolve` | RÃ©sout un template (structure) par identifiant ; donnÃ©es fournies dans le flux |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuWidgets en contient sept.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : les Tools opÃ¨rent sur des **donnÃ©es fournies dans le flux** (MiyuWidgets ne lit pas la base directement) ; persistance templates/layouts = KindMother. Le Toolkit est dÃ©clarÃ© dans Master Butler et compatibilisÃ© par Ever Buddy ([Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)).

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **0 Ã  2** selon politique d'exposition (page builder Ã©ditorial) ; cohÃ©rent avec WorrySentinel. Le niveau du Toolkit est au moins Ã©gal au maximum des niveaux de ses Tools composants. |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon Toolkit Composition Contract) |

---

## 8. Relation avec MiyuWeb et KindMother

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuWidgets - Tool Governance Compliance Contract](./contracts/governance/MiyuWidgets%20-%20Tool%20Governance%20Compliance%20Contract.md).

- **MiyuWeb** fournit les capacitÃ©s de base (html.render, layout.render, theme.resolve, form.validate, event.dispatch, input.capture, etc.). MiyuWidgets **complÃ¨te** MiyuWeb pour l'Ã©dition visuelle de pages et de thÃ¨mes (widgets, layout.apply, template.resolve). L'OpÃ©rateur d'Interface Â« Ã‰diteur de pages / thÃ¨me Â» utilise MiyuWeb + MiyuWidgets.
- **KindMother** est l'autoritÃ© sur les donnÃ©es (templates, structures de layout) ; ces donnÃ©es sont fournies dans le flux Ã  MiyuWidgets, qui ne lit pas la base directement.
- Les Tools MiyuWidgets opÃ¨rent sur des **donnÃ©es fournies dans le flux** ; la persistance des templates et layouts relÃ¨ve de KindMother (et Ã©ventuellement MiyuSQL sous mandat KindMother).

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](..//..//miyukini-webway-system//reference//_index.md), [MiyuWeb - Documentation Fondatrice](../MiyuWeb/MiyuWeb%20-%20Documentation%20Fondatrice.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuWidgets sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). RÃ©fÃ©rence : [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

- **Domaine** : `web` â€” cohÃ©rent avec la projection domains.json (blocs du domaine Â« web Â»).
- **Layer** : outil / toolkit (Strate 6) â€” Ã  reflÃ©ter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuWidgets est une unitÃ© logique pouvant devenir un **bloc MSCM** Ã  l'implÃ©mentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) |
| Ã‰quivalents Boutique CMS Reservation SaaS | [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](..//..//miyukini-webway-system//reference//_index.md) |
| MiyuWeb - Documentation Fondatrice | [MiyuWeb - Documentation Fondatrice](../MiyuWeb/MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


