# MiyuCMS — Documentation Fondatrice

## 1. Contexte

**MiyuCMS** est le **kit d'outils (Toolkit)** de gestion de contenu éditorial (posts, pages, médias, révisions, commentaires) de l'écosystème Miyukini. Il intègre les outils de création, mise à jour, publication et planification de contenus, de gestion des révisions, des commentaires et des médias, alignés sur KindMother pour la persistance des données.

L'autorité sur les données (contenus, médias, révisions, commentaires) appartient à **KindMother** (Core de données, Strate 4). MiyuCMS expose des capacités d'exécution gouvernée (créer, mettre à jour, publier, planifier, révisions, commentaires, médias) sans remplacer KindMother ni StrongFather ; les Opérateurs passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :**

- L'identité et la définition canonique de MiyuCMS
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliqués)
- Le niveau de sécurité et les états système autorisés ou interdits
- La relation avec KindMother (persistance contenus, médias, révisions, commentaires)
- L'alignement avec le protocole MIP v1 pour une future implémentation indexable

**Hors scope :**

- L'implémentation détaillée (stockage, schémas DB)
- Toute décision de publication, modération ou politique éditoriale — celle-ci reste du ressort de StrongFather et des Cores
- L'affichage des contenus (MiyuWeb, données fournies dans le flux)

---

## 3. Définition canonique

> **MiyuCMS est une composition officielle d'outils de gestion de contenu éditorial (création, mise à jour, publication, planification, révisions, commentaires, médias), déclarée et gouvernée par l'environnement.**

- MiyuCMS **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuCMS **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (créer, mettre à jour, publier, planifier, gérer révisions et commentaires, upload/serve/transform médias) sans décider de la politique de publication ni de la modération.

**Règle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacité. Les capacités viennent exclusivement des Tools composants. Toute persistance (contenus, médias, révisions, commentaires) est sous autorité KindMother (WriteIntent).

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.content.cms` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `content` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

MiyuCMS est composé des Tools suivants (format canonique `tool.<domain>.<action>` ou `tool.<domain>.<sous-domaine>.<action>`). Le détail de chaque outil (action, niveau de sécurité, capability_id) sera décrit dans MiyuCMS - Reference Outils (phase ultérieure).

| ToolId | Description courte |
|--------|---------------------|
| `tool.content.create` | Crée un brouillon de contenu à partir de données fournies ; ne décide pas de la politique de publication |
| `tool.content.update` | Met à jour un contenu existant à partir de données fournies |
| `tool.content.publish` | Marque un contenu comme publié (exécution seule ; décision = StrongFather) |
| `tool.content.schedule` | Planifie une publication à une date/heure fournie |
| `tool.content.revision.list` | Liste les révisions d'un contenu |
| `tool.content.revision.restore` | Restaure une révision donnée (exécution ; décision = StrongFather) |
| `tool.content.revision.compare` | Compare deux révisions (lecture seule) |
| `tool.content.comment.create` | Crée un commentaire à partir de données fournies |
| `tool.content.comment.moderate` | Applique une action de modération (approuver, rejeter) ; décision = StrongFather |
| `tool.content.comment.list` | Liste les commentaires d'un contenu (filtres fournis dans le flux) |
| `tool.media.upload` | Enregistre un média à partir du flux ; persistance = KindMother |
| `tool.media.serve` | Sert un média (stream ou métadonnées) à partir de données fournies |
| `tool.media.transform` | Produit une variante (miniature, recadrage) à partir de données fournies |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuCMS en contient treize.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : décision (publication, modération) = StrongFather ; toute écriture (contenu, média, révision, commentaire) = WriteIntent KindMother. Le Toolkit est déclaré dans Master Butler et compatibilisé par Ever Buddy ([Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)).

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 2** selon politique d'exposition (contenu public à éditorial sensible) ; cohérent avec WorrySentinel. Le niveau du Toolkit est au moins égal au maximum des niveaux de ses Tools composants. |
| **États autorisés** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon Toolkit Composition Contract) |

---

## 8. Relation avec KindMother

- **KindMother** est l'autorité sur toutes les données de contenu : contenus (posts, pages), médias, révisions, commentaires. Toute création, mise à jour ou suppression passe par **WriteIntent** vers KindMother ; MiyuCMS exécute des capacités (create, update, publish, schedule, revision.*, comment.*, media.*) **sans décider** de la politique éditoriale.
- L'affichage des contenus est du ressort de **MiyuWeb** (données fournies dans le flux) ; MiyuCMS ne gère que la gestion éditoriale et la persistance gouvernée.

**Référence :** [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuCMS sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol) :

- **Domaine** : `content` — cohérent avec la projection domains.json (blocs du domaine « content »).
- **Layer** : outil / toolkit (Strate 6) — à refléter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuCMS est une unité logique pouvant devenir un **bloc MSCM** à l'implémentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Équivalents Boutique CMS Reservation SaaS | [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| Pyramide Architecture | [Miyukini Conceptual References - Pyramide Architecture Complete](../../reference/Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
