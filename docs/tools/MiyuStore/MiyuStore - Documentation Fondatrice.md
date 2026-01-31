# MiyuStore — Documentation Fondatrice

## 1. Contexte

**MiyuStore** est le **kit d'outils (Toolkit)** de boutique en ligne (catalogue, panier, checkout, paiement, livraison, commandes) de l'écosystème Miyukini. Il intègre les outils de gestion des produits, du panier, du checkout, des paiements, de la livraison et des commandes, alignés sur KindMother pour la persistance des données.

L'autorité sur les données (produits, paniers, commandes, règles livraison et paiement) appartient à **KindMother** (Core de données, Strate 4). MiyuStore expose des capacités d'exécution gouvernée (produit, panier, checkout, paiement, livraison, commande) sans remplacer KindMother ni StrongFather ; les Opérateurs passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :**

- L'identité et la définition canonique de MiyuStore
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliqués)
- Le niveau de sécurité et les états système autorisés ou interdits
- La relation avec KindMother (persistance produits, paniers, commandes)
- L'alignement avec le protocole MIP v1 pour une future implémentation indexable

**Hors scope :**

- L'implémentation détaillée (gateways paiement, transporteurs)
- Toute décision d'autorisation de paiement, de promo ou de politique commerciale — celle-ci reste du ressort de StrongFather et des Cores
- Les étiquettes et expéditions détaillées (MiyuShipping)

---

## 3. Définition canonique

> **MiyuStore est une composition officielle d'outils de boutique en ligne (catalogue, panier, checkout, paiement, livraison, commandes), déclarée et gouvernée par l'environnement.**

- MiyuStore **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuStore **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (produit, panier, checkout, paiement, livraison, commande) sans décider des autorisations, promos ou politiques commerciales.

**Règle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacité. Les capacités viennent exclusivement des Tools composants. Toute persistance (produits, paniers, commandes) et toute décision (autorisation paiement, checkout) sont sous autorité KindMother (WriteIntent) et StrongFather (ALLOW/DENY).

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.commerce.store` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `commerce` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuStore - Reference Outils](./MiyuStore%20-%20Reference%20Outils.md). MiyuStore est composé des Tools suivants (format canonique `tool.commerce.<sous-domaine>.<action>`).

| ToolId | Description courte |
|--------|---------------------|
| `tool.commerce.product.list` | Liste des produits selon filtres fournis |
| `tool.commerce.product.resolve` | Résout un produit par identifiant |
| `tool.commerce.product.variations` | Liste les variations d'un produit |
| `tool.commerce.product.create` | Crée un produit (exécution ; décision = StrongFather) |
| `tool.commerce.product.update` | Met à jour un produit |
| `tool.commerce.cart.add` | Ajoute une ligne au panier à partir de données fournies |
| `tool.commerce.cart.update` | Met à jour une ligne du panier |
| `tool.commerce.cart.remove` | Supprime une ligne du panier |
| `tool.commerce.cart.compute` | Calcule totaux, taxes, livraison du panier (règles fournies) |
| `tool.commerce.checkout.validate` | Valide les données de checkout (structure, champs) |
| `tool.commerce.checkout.submit` | Soumet le checkout et crée la commande (WriteIntent KindMother) |
| `tool.commerce.payment.capture` | Capture un paiement (exécution ; autorisation = StrongFather) |
| `tool.commerce.payment.refund` | Rembourse un paiement |
| `tool.commerce.payment.status` | Retourne le statut d'un paiement |
| `tool.commerce.shipping.rate` | Calcule le tarif de livraison pour un panier/zone fourni |
| `tool.commerce.shipping.zones.resolve` | Résout les zones de livraison applicables |
| `tool.commerce.order.create` | Crée une commande (exécution ; souvent appelé par checkout.submit) |
| `tool.commerce.order.update` | Met à jour une commande (statut, champs) |
| `tool.commerce.order.status` | Retourne le statut d'une commande |
| `tool.commerce.order.list` | Liste les commandes selon filtres fournis |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuStore en contient vingt.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : WorrySentinel applique le niveau de sécurité paiement ; décision (checkout, paiement, création commande) = StrongFather ; toute écriture (produit, panier, commande) = WriteIntent KindMother. Le Toolkit est déclaré dans Master Butler et compatibilisé par Ever Buddy ([Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)).

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 3** selon opération (catalogue 0–1, panier/checkout 1–2, paiement 3) ; cohérent avec WorrySentinel. Le niveau du Toolkit est au moins égal au maximum des niveaux de ses Tools composants. |
| **États autorisés** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon Toolkit Composition Contract) |

---

## 8. Relation avec KindMother et MiyuShipping

- **KindMother** est l'autorité sur toutes les données commerce : produits, paniers, commandes, règles de livraison et de paiement. Toute création ou mise à jour passe par **WriteIntent** vers KindMother ; MiyuStore exécute des capacités **sans décider** de l'autorisation (StrongFather) ni de la politique commerciale.
- **MiyuShipping** (toolkit.commerce.shipping) complète MiyuStore pour les étiquettes, la comparaison de tarifs transporteurs, le suivi colis et les expéditions ; MiyuStore inclut les Tools de base (shipping.rate, shipping.zones.resolve) nécessaires au checkout.

**Référence :** [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuStore sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). Référence : [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

- **Domaine** : `commerce` — cohérent avec la projection domains.json (blocs du domaine « commerce »).
- **Layer** : outil / toolkit (Strate 6) — à refléter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuStore est une unité logique pouvant devenir un **bloc MSCM** à l'implémentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Équivalents Boutique CMS Reservation SaaS | [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| MiyuShipping - Documentation Fondatrice | [MiyuShipping - Documentation Fondatrice](../MiyuShipping/MiyuShipping%20-%20Documentation%20Fondatrice.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
