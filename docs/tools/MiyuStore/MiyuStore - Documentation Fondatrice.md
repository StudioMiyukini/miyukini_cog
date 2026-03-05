# MiyuStore â€” Documentation Fondatrice

## 1. Contexte

**MiyuStore** est le **kit d'outils (Toolkit)** de boutique en ligne (catalogue, panier, checkout, paiement, livraison, commandes) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de gestion des produits, du panier, du checkout, des paiements, de la livraison et des commandes, alignÃ©s sur KindMother pour la persistance des donnÃ©es.

L'autoritÃ© sur les donnÃ©es (produits, paniers, commandes, rÃ¨gles livraison et paiement) appartient Ã  **KindMother** (Core de donnÃ©es, Strate 4). MiyuStore expose des capacitÃ©s d'exÃ©cution gouvernÃ©e (produit, panier, checkout, paiement, livraison, commande) sans remplacer KindMother ni StrongFather ; les OpÃ©rateurs passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :**

- L'identitÃ© et la dÃ©finition canonique de MiyuStore
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliquÃ©s)
- Le niveau de sÃ©curitÃ© et les Ã©tats systÃ¨me autorisÃ©s ou interdits
- La relation avec KindMother (persistance produits, paniers, commandes)
- L'alignement avec le protocole MIP v1 pour une future implÃ©mentation indexable

**Hors scope :**

- L'implÃ©mentation dÃ©taillÃ©e (gateways paiement, transporteurs)
- Toute dÃ©cision d'autorisation de paiement, de promo ou de politique commerciale â€” celle-ci reste du ressort de StrongFather et des Cores
- Les Ã©tiquettes et expÃ©ditions dÃ©taillÃ©es (MiyuShipping)

---

## 3. DÃ©finition canonique

> **MiyuStore est une composition officielle d'outils de boutique en ligne (catalogue, panier, checkout, paiement, livraison, commandes), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuStore **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuStore **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (produit, panier, checkout, paiement, livraison, commande) sans dÃ©cider des autorisations, promos ou politiques commerciales.

**RÃ¨gle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacitÃ©. Les capacitÃ©s viennent exclusivement des Tools composants. Toute persistance (produits, paniers, commandes) et toute dÃ©cision (autorisation paiement, checkout) sont sous autoritÃ© KindMother (WriteIntent) et StrongFather (ALLOW/DENY).

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.commerce.store` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `commerce` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuStore - Reference Outils](./MiyuStore%20-%20Reference%20Outils.md). MiyuStore est composÃ© des Tools suivants (format canonique `tool.commerce.<sous-domaine>.<action>`).

| ToolId | Description courte |
|--------|---------------------|
| `tool.commerce.product.list` | Liste des produits selon filtres fournis |
| `tool.commerce.product.resolve` | RÃ©sout un produit par identifiant |
| `tool.commerce.product.variations` | Liste les variations d'un produit |
| `tool.commerce.product.create` | CrÃ©e un produit (exÃ©cution ; dÃ©cision = StrongFather) |
| `tool.commerce.product.update` | Met Ã  jour un produit |
| `tool.commerce.cart.add` | Ajoute une ligne au panier Ã  partir de donnÃ©es fournies |
| `tool.commerce.cart.update` | Met Ã  jour une ligne du panier |
| `tool.commerce.cart.remove` | Supprime une ligne du panier |
| `tool.commerce.cart.compute` | Calcule totaux, taxes, livraison du panier (rÃ¨gles fournies) |
| `tool.commerce.checkout.validate` | Valide les donnÃ©es de checkout (structure, champs) |
| `tool.commerce.checkout.submit` | Soumet le checkout et crÃ©e la commande (WriteIntent KindMother) |
| `tool.commerce.payment.capture` | Capture un paiement (exÃ©cution ; autorisation = StrongFather) |
| `tool.commerce.payment.refund` | Rembourse un paiement |
| `tool.commerce.payment.status` | Retourne le statut d'un paiement |
| `tool.commerce.shipping.rate` | Calcule le tarif de livraison pour un panier/zone fourni |
| `tool.commerce.shipping.zones.resolve` | RÃ©sout les zones de livraison applicables |
| `tool.commerce.order.create` | CrÃ©e une commande (exÃ©cution ; souvent appelÃ© par checkout.submit) |
| `tool.commerce.order.update` | Met Ã  jour une commande (statut, champs) |
| `tool.commerce.order.status` | Retourne le statut d'une commande |
| `tool.commerce.order.list` | Liste les commandes selon filtres fournis |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuStore en contient vingt.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : WorrySentinel applique le niveau de sÃ©curitÃ© paiement ; dÃ©cision (checkout, paiement, crÃ©ation commande) = StrongFather ; toute Ã©criture (produit, panier, commande) = WriteIntent KindMother. Le Toolkit est dÃ©clarÃ© dans Master Butler et compatibilisÃ© par Ever Buddy ([Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)).

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **0 Ã  3** selon opÃ©ration (catalogue 0â€“1, panier/checkout 1â€“2, paiement 3) ; cohÃ©rent avec WorrySentinel. Le niveau du Toolkit est au moins Ã©gal au maximum des niveaux de ses Tools composants. |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon Toolkit Composition Contract) |

---

## 8. Relation avec KindMother et MiyuShipping

- **KindMother** est l'autoritÃ© sur toutes les donnÃ©es commerce : produits, paniers, commandes, rÃ¨gles de livraison et de paiement. Toute crÃ©ation ou mise Ã  jour passe par **WriteIntent** vers KindMother ; MiyuStore exÃ©cute des capacitÃ©s **sans dÃ©cider** de l'autorisation (StrongFather) ni de la politique commerciale.
- **MiyuShipping** (toolkit.commerce.shipping) complÃ¨te MiyuStore pour les Ã©tiquettes, la comparaison de tarifs transporteurs, le suivi colis et les expÃ©ditions ; MiyuStore inclut les Tools de base (shipping.rate, shipping.zones.resolve) nÃ©cessaires au checkout.

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](..//..//miyukini-webway-system//reference//_index.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuStore sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). RÃ©fÃ©rence : [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

- **Domaine** : `commerce` â€” cohÃ©rent avec la projection domains.json (blocs du domaine Â« commerce Â»).
- **Layer** : outil / toolkit (Strate 6) â€” Ã  reflÃ©ter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuStore est une unitÃ© logique pouvant devenir un **bloc MSCM** Ã  l'implÃ©mentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) |
| Ã‰quivalents Boutique CMS Reservation SaaS | [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| MiyuShipping - Documentation Fondatrice | [MiyuShipping - Documentation Fondatrice](../MiyuShipping/MiyuShipping%20-%20Documentation%20Fondatrice.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


