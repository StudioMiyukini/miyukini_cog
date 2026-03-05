# MiyuCMS â€” Documentation Fondatrice

## 1. Contexte

**MiyuCMS** est le **kit d'outils (Toolkit)** de gestion de contenu Ã©ditorial (posts, pages, mÃ©dias, rÃ©visions, commentaires) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de crÃ©ation, mise Ã  jour, publication et planification de contenus, de gestion des rÃ©visions, des commentaires et des mÃ©dias, alignÃ©s sur KindMother pour la persistance des donnÃ©es.

L'autoritÃ© sur les donnÃ©es (contenus, mÃ©dias, rÃ©visions, commentaires) appartient Ã  **KindMother** (Core de donnÃ©es, Strate 4). MiyuCMS expose des capacitÃ©s d'exÃ©cution gouvernÃ©e (crÃ©er, mettre Ã  jour, publier, planifier, rÃ©visions, commentaires, mÃ©dias) sans remplacer KindMother ni StrongFather ; les OpÃ©rateurs passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :**

- L'identitÃ© et la dÃ©finition canonique de MiyuCMS
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliquÃ©s)
- Le niveau de sÃ©curitÃ© et les Ã©tats systÃ¨me autorisÃ©s ou interdits
- La relation avec KindMother (persistance contenus, mÃ©dias, rÃ©visions, commentaires)
- L'alignement avec le protocole MIP v1 pour une future implÃ©mentation indexable

**Hors scope :**

- L'implÃ©mentation dÃ©taillÃ©e (stockage, schÃ©mas DB)
- Toute dÃ©cision de publication, modÃ©ration ou politique Ã©ditoriale â€” celle-ci reste du ressort de StrongFather et des Cores
- L'affichage des contenus (MiyuWeb, donnÃ©es fournies dans le flux)

---

## 3. DÃ©finition canonique

> **MiyuCMS est une composition officielle d'outils de gestion de contenu Ã©ditorial (crÃ©ation, mise Ã  jour, publication, planification, rÃ©visions, commentaires, mÃ©dias), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuCMS **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuCMS **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (crÃ©er, mettre Ã  jour, publier, planifier, gÃ©rer rÃ©visions et commentaires, upload/serve/transform mÃ©dias) sans dÃ©cider de la politique de publication ni de la modÃ©ration.

**RÃ¨gle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacitÃ©. Les capacitÃ©s viennent exclusivement des Tools composants. Toute persistance (contenus, mÃ©dias, rÃ©visions, commentaires) est sous autoritÃ© KindMother (WriteIntent).

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.content.cms` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `content` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuCMS - Reference Outils](./MiyuCMS%20-%20Reference%20Outils.md). MiyuCMS est composÃ© des Tools suivants (format canonique `tool.<domain>.<action>` ou `tool.<domain>.<sous-domaine>.<action>`).

| ToolId | Description courte |
|--------|---------------------|
| `tool.content.create` | CrÃ©e un brouillon de contenu Ã  partir de donnÃ©es fournies ; ne dÃ©cide pas de la politique de publication |
| `tool.content.update` | Met Ã  jour un contenu existant Ã  partir de donnÃ©es fournies |
| `tool.content.publish` | Marque un contenu comme publiÃ© (exÃ©cution seule ; dÃ©cision = StrongFather) |
| `tool.content.schedule` | Planifie une publication Ã  une date/heure fournie |
| `tool.content.revision.list` | Liste les rÃ©visions d'un contenu |
| `tool.content.revision.restore` | Restaure une rÃ©vision donnÃ©e (exÃ©cution ; dÃ©cision = StrongFather) |
| `tool.content.revision.compare` | Compare deux rÃ©visions (lecture seule) |
| `tool.content.comment.create` | CrÃ©e un commentaire Ã  partir de donnÃ©es fournies |
| `tool.content.comment.moderate` | Applique une action de modÃ©ration (approuver, rejeter) ; dÃ©cision = StrongFather |
| `tool.content.comment.list` | Liste les commentaires d'un contenu (filtres fournis dans le flux) |
| `tool.media.upload` | Enregistre un mÃ©dia Ã  partir du flux ; persistance = KindMother |
| `tool.media.serve` | Sert un mÃ©dia (stream ou mÃ©tadonnÃ©es) Ã  partir de donnÃ©es fournies |
| `tool.media.transform` | Produit une variante (miniature, recadrage) Ã  partir de donnÃ©es fournies |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuCMS en contient treize.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : dÃ©cision (publication, modÃ©ration) = StrongFather ; toute Ã©criture (contenu, mÃ©dia, rÃ©vision, commentaire) = WriteIntent KindMother. Le Toolkit est dÃ©clarÃ© dans Master Butler et compatibilisÃ© par Ever Buddy ([Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)).

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **0 Ã  2** selon politique d'exposition (contenu public Ã  Ã©ditorial sensible) ; cohÃ©rent avec WorrySentinel. Le niveau du Toolkit est au moins Ã©gal au maximum des niveaux de ses Tools composants. |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon Toolkit Composition Contract) |

---

## 8. Relation avec KindMother

- **KindMother** est l'autoritÃ© sur toutes les donnÃ©es de contenu : contenus (posts, pages), mÃ©dias, rÃ©visions, commentaires. Toute crÃ©ation, mise Ã  jour ou suppression passe par **WriteIntent** vers KindMother ; MiyuCMS exÃ©cute des capacitÃ©s (create, update, publish, schedule, revision.*, comment.*, media.*) **sans dÃ©cider** de la politique Ã©ditoriale.
- L'affichage des contenus est du ressort de **MiyuWeb** (donnÃ©es fournies dans le flux) ; MiyuCMS ne gÃ¨re que la gestion Ã©ditoriale et la persistance gouvernÃ©e.

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](..//..//miyukini-webway-system//reference//_index.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuCMS sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). RÃ©fÃ©rence : [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

- **Domaine** : `content` â€” cohÃ©rent avec la projection domains.json (blocs du domaine Â« content Â»).
- **Layer** : outil / toolkit (Strate 6) â€” Ã  reflÃ©ter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuCMS est une unitÃ© logique pouvant devenir un **bloc MSCM** Ã  l'implÃ©mentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) |
| Ã‰quivalents Boutique CMS Reservation SaaS | [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| Pyramide Architecture | [Miyukini Conceptual References - Pyramide Architecture Complete](..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


