# MiyuMedia — Documentation Fondatrice

## 1. Contexte

**MiyuMedia** est le **kit d'outils (Toolkit)** de gestion des médias (upload, service, transformation) de l'écosystème Miyukini. Il intègre les outils d'enregistrement, de service et de transformation des médias (miniatures, recadrage), alignés sur KindMother pour la persistance des données.

L'autorité sur les données médias appartient à **KindMother** (Core de données, Strate 4). MiyuMedia expose des capacités d'exécution gouvernée (upload, serve, transform) sans remplacer KindMother ni StrongFather ; les Opérateurs passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :**

- L'identité et la définition canonique de MiyuMedia
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliqués)
- Le niveau de sécurité et les états système autorisés ou interdits
- La relation avec KindMother (persistance médias)
- L'alignement avec le protocole MIP v1 pour une future implémentation indexable

**Hors scope :**

- L'implémentation détaillée (stockage binaire, génération de miniatures)
- Toute décision de politique de stockage ou de quota — celle-ci reste du ressort de StrongFather et des Cores
- La gestion éditoriale des contenus (MiyuCMS)

---

## 3. Définition canonique

> **MiyuMedia est une composition officielle d'outils de gestion des médias (upload, service, transformation), déclarée et gouvernée par l'environnement.**

- MiyuMedia **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuMedia **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (enregistrer un média, servir un média, produire une variante) sans décider de la politique de stockage ni des quotas.

**Règle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacité. Les capacités viennent exclusivement des Tools composants. Toute persistance des médias est sous autorité KindMother (WriteIntent).

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.content.media` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `content` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

MiyuMedia est composé des Tools suivants (format canonique `tool.<domain>.<action>`). Le détail de chaque outil (action, niveau de sécurité, capability_id) sera décrit dans MiyuMedia - Reference Outils (phase ultérieure).

| ToolId | Description courte |
|--------|---------------------|
| `tool.media.upload` | Enregistre un média à partir du flux ; persistance = KindMother |
| `tool.media.serve` | Sert un média (stream ou métadonnées) à partir de données fournies |
| `tool.media.transform` | Produit une variante (miniature, recadrage) à partir de données fournies |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuMedia en contient trois.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : toute écriture (upload, métadonnées) = WriteIntent KindMother. Le Toolkit est déclaré dans Master Butler et compatibilisé par Ever Buddy ([Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)).

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 2** selon politique d'exposition (médias publics à sensibles) ; cohérent avec WorrySentinel. Le niveau du Toolkit est au moins égal au maximum des niveaux de ses Tools composants. |
| **États autorisés** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon Toolkit Composition Contract) |

---

## 8. Relation avec KindMother et MiyuCMS

- **KindMother** est l'autorité sur toutes les données médias. Toute création (upload) passe par **WriteIntent** vers KindMother ; MiyuMedia exécute des capacités (upload, serve, transform) **sans décider** de la politique de stockage.
- **MiyuCMS** agrège MiyuMedia (tool.media.*) dans son périmètre pour le Service CMS complet ; MiyuMedia peut être utilisé seul pour des contextes où seule la gestion des médias est requise.

**Référence :** [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuMedia sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol) :

- **Domaine** : `content` — cohérent avec la projection domains.json (blocs du domaine « content »).
- **Layer** : outil / toolkit (Strate 6) — à refléter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuMedia est une unité logique pouvant devenir un **bloc MSCM** à l'implémentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Équivalents Boutique CMS Reservation SaaS | [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| MiyuCMS - Documentation Fondatrice | [MiyuCMS - Documentation Fondatrice](../MiyuCMS/MiyuCMS%20-%20Documentation%20Fondatrice.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
