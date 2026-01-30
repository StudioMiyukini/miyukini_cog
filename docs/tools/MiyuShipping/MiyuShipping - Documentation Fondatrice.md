# MiyuShipping — Documentation Fondatrice

## 1. Contexte

**MiyuShipping** est le **kit d'outils (Toolkit)** de livraison et d'expédition (tarifs, zones, étiquettes, comparaison transporteurs, suivi colis, expéditions) de l'écosystème Miyukini. Il intègre les outils de calcul de tarifs, de résolution des zones, de création et d'impression d'étiquettes, de comparaison de tarifs transporteurs, de suivi et de gestion des expéditions, alignés sur KindMother pour les règles et l'état des commandes.

L'autorité sur les données (zones, règles de livraison, état des commandes et expéditions) appartient à **KindMother** (Core de données, Strate 4). MiyuShipping expose des capacités d'exécution gouvernée (rate, zones, label, rates.compare, tracking, shipment) sans remplacer KindMother ni StrongFather ; les Opérateurs passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :**

- L'identité et la définition canonique de MiyuShipping
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliqués)
- Le niveau de sécurité et les états système autorisés ou interdits
- La relation avec KindMother et MiyuStore (livraison, commandes)
- L'alignement avec le protocole MIP v1 pour une future implémentation indexable

**Hors scope :**

- L'implémentation détaillée (intégration transporteurs USPS, UPS, DHL, etc.)
- Toute décision de politique de livraison ou d'enlèvement — celle-ci reste du ressort de StrongFather et des Cores
- Le catalogue et le checkout (MiyuStore)

---

## 3. Définition canonique

> **MiyuShipping est une composition officielle d'outils de livraison et d'expédition (tarifs, zones, étiquettes, comparaison transporteurs, suivi, expéditions), déclarée et gouvernée par l'environnement.**

- MiyuShipping **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuShipping **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (calculer tarif, résoudre zones, créer/imprimer étiquette, comparer tarifs, suivi, créer/lister expéditions) sans décider de la politique de livraison ni des transporteurs.

**Règle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacité. Les capacités viennent exclusivement des Tools composants. Toute persistance (expéditions, état commande) et toute règle (zones, tarifs) sont sous autorité KindMother (WriteIntent ou données).

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.commerce.shipping` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `commerce` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

MiyuShipping est composé des Tools suivants (format canonique `tool.commerce.shipping.<action>` ou `tool.commerce.shipping.<sous-domaine>.<action>`). Le détail de chaque outil (action, niveau de sécurité, capability_id) sera décrit dans MiyuShipping - Reference Outils (phase ultérieure).

| ToolId | Description courte |
|--------|---------------------|
| `tool.commerce.shipping.rate` | Calcule le tarif de livraison pour un panier/zone fourni |
| `tool.commerce.shipping.zones.resolve` | Résout les zones de livraison applicables |
| `tool.commerce.shipping.label.create` | Crée une étiquette d'expédition pour une commande/colis fourni |
| `tool.commerce.shipping.label.print` | Produit les données d'impression d'une étiquette (exécution seule) |
| `tool.commerce.shipping.rates.compare` | Compare les tarifs de plusieurs transporteurs pour un colis donné |
| `tool.commerce.shipping.tracking.get` | Retourne le statut de suivi d'un envoi (identifiant fourni) |
| `tool.commerce.shipping.shipment.create` | Crée une expédition (colis) pour une commande ; WriteIntent si état commande géré |
| `tool.commerce.shipping.shipment.list` | Liste les expéditions d'une commande |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuShipping en contient huit.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : décision (création étiquette, expédition) = StrongFather ; toute écriture (expédition, état) = WriteIntent KindMother. Le Toolkit est déclaré dans Master Butler et compatibilisé par Ever Buddy ([Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)).

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 2** selon opération (tarifs/suivi 0–1, étiquettes/expéditions 2) ; cohérent avec WorrySentinel. Le niveau du Toolkit est au moins égal au maximum des niveaux de ses Tools composants. |
| **États autorisés** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon Toolkit Composition Contract) |

---

## 8. Relation avec KindMother et MiyuStore

- **KindMother** est l'autorité sur les règles de livraison (zones, tarifs), l'état des commandes et des expéditions. MiyuShipping exécute des capacités (rate, zones, label, tracking, shipment) **sans décider** de la politique de livraison ; les règles sont fournies par KindMother ou dans le flux.
- **MiyuStore** inclut les Tools de base `tool.commerce.shipping.rate` et `tool.commerce.shipping.zones.resolve` pour le checkout ; MiyuShipping agrège l'ensemble des Tools livraison (rate, zones, étiquettes, comparaison, suivi, expéditions) pour le Service complet de livraison et d'expédition.

**Référence :** [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md), [MiyuStore - Documentation Fondatrice](../MiyuStore/MiyuStore%20-%20Documentation%20Fondatrice.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuShipping sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol) :

- **Domaine** : `commerce` — cohérent avec la projection domains.json (blocs du domaine « commerce »).
- **Layer** : outil / toolkit (Strate 6) — à refléter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuShipping est une unité logique pouvant devenir un **bloc MSCM** à l'implémentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Équivalents Boutique CMS Reservation SaaS | [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md) |
| MiyuStore - Documentation Fondatrice | [MiyuStore - Documentation Fondatrice](../MiyuStore/MiyuStore%20-%20Documentation%20Fondatrice.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
