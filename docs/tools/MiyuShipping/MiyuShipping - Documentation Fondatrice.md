# MiyuShipping â€” Documentation Fondatrice

## 1. Contexte

**MiyuShipping** est le **kit d'outils (Toolkit)** de livraison et d'expÃ©dition (tarifs, zones, Ã©tiquettes, comparaison transporteurs, suivi colis, expÃ©ditions) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de calcul de tarifs, de rÃ©solution des zones, de crÃ©ation et d'impression d'Ã©tiquettes, de comparaison de tarifs transporteurs, de suivi et de gestion des expÃ©ditions, alignÃ©s sur KindMother pour les rÃ¨gles et l'Ã©tat des commandes.

L'autoritÃ© sur les donnÃ©es (zones, rÃ¨gles de livraison, Ã©tat des commandes et expÃ©ditions) appartient Ã  **KindMother** (Core de donnÃ©es, Strate 4). MiyuShipping expose des capacitÃ©s d'exÃ©cution gouvernÃ©e (rate, zones, label, rates.compare, tracking, shipment) sans remplacer KindMother ni StrongFather ; les OpÃ©rateurs passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :**

- L'identitÃ© et la dÃ©finition canonique de MiyuShipping
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliquÃ©s)
- Le niveau de sÃ©curitÃ© et les Ã©tats systÃ¨me autorisÃ©s ou interdits
- La relation avec KindMother et MiyuStore (livraison, commandes)
- L'alignement avec le protocole MIP v1 pour une future implÃ©mentation indexable

**Hors scope :**

- L'implÃ©mentation dÃ©taillÃ©e (intÃ©gration transporteurs USPS, UPS, DHL, etc.)
- Toute dÃ©cision de politique de livraison ou d'enlÃ¨vement â€” celle-ci reste du ressort de StrongFather et des Cores
- Le catalogue et le checkout (MiyuStore)

---

## 3. DÃ©finition canonique

> **MiyuShipping est une composition officielle d'outils de livraison et d'expÃ©dition (tarifs, zones, Ã©tiquettes, comparaison transporteurs, suivi, expÃ©ditions), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuShipping **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuShipping **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (calculer tarif, rÃ©soudre zones, crÃ©er/imprimer Ã©tiquette, comparer tarifs, suivi, crÃ©er/lister expÃ©ditions) sans dÃ©cider de la politique de livraison ni des transporteurs.

**RÃ¨gle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacitÃ©. Les capacitÃ©s viennent exclusivement des Tools composants. Toute persistance (expÃ©ditions, Ã©tat commande) et toute rÃ¨gle (zones, tarifs) sont sous autoritÃ© KindMother (WriteIntent ou donnÃ©es).

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.commerce.shipping` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `commerce` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

MiyuShipping est composÃ© des Tools suivants (format canonique `tool.commerce.shipping.<action>` ou `tool.commerce.shipping.<sous-domaine>.<action>`). Le dÃ©tail de chaque outil (action, niveau de sÃ©curitÃ©, capability_id) sera dÃ©crit dans MiyuShipping - Reference Outils (phase ultÃ©rieure).

| ToolId | Description courte |
|--------|---------------------|
| `tool.commerce.shipping.rate` | Calcule le tarif de livraison pour un panier/zone fourni |
| `tool.commerce.shipping.zones.resolve` | RÃ©sout les zones de livraison applicables |
| `tool.commerce.shipping.label.create` | CrÃ©e une Ã©tiquette d'expÃ©dition pour une commande/colis fourni |
| `tool.commerce.shipping.label.print` | Produit les donnÃ©es d'impression d'une Ã©tiquette (exÃ©cution seule) |
| `tool.commerce.shipping.rates.compare` | Compare les tarifs de plusieurs transporteurs pour un colis donnÃ© |
| `tool.commerce.shipping.tracking.get` | Retourne le statut de suivi d'un envoi (identifiant fourni) |
| `tool.commerce.shipping.shipment.create` | CrÃ©e une expÃ©dition (colis) pour une commande ; WriteIntent si Ã©tat commande gÃ©rÃ© |
| `tool.commerce.shipping.shipment.list` | Liste les expÃ©ditions d'une commande |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuShipping en contient huit.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : dÃ©cision (crÃ©ation Ã©tiquette, expÃ©dition) = StrongFather ; toute Ã©criture (expÃ©dition, Ã©tat) = WriteIntent KindMother. Le Toolkit est dÃ©clarÃ© dans Master Butler et compatibilisÃ© par Ever Buddy ([Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)).

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **0 Ã  2** selon opÃ©ration (tarifs/suivi 0â€“1, Ã©tiquettes/expÃ©ditions 2) ; cohÃ©rent avec WorrySentinel. Le niveau du Toolkit est au moins Ã©gal au maximum des niveaux de ses Tools composants. |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon Toolkit Composition Contract) |

---

## 8. Relation avec KindMother et MiyuStore

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuShipping - Tool Governance Compliance Contract](./contracts/governance/MiyuShipping%20-%20Tool%20Governance%20Compliance%20Contract.md).

- **KindMother** est l'autoritÃ© sur les rÃ¨gles de livraison (zones, tarifs), l'Ã©tat des commandes et des expÃ©ditions. MiyuShipping exÃ©cute des capacitÃ©s (rate, zones, label, tracking, shipment) **sans dÃ©cider** de la politique de livraison ; les rÃ¨gles sont fournies par KindMother ou dans le flux.
- **MiyuStore** inclut les Tools de base `tool.commerce.shipping.rate` et `tool.commerce.shipping.zones.resolve` pour le checkout ; MiyuShipping agrÃ¨ge l'ensemble des Tools livraison (rate, zones, Ã©tiquettes, comparaison, suivi, expÃ©ditions) pour le Service complet de livraison et d'expÃ©dition.

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](..//..//miyukini-webway-system//reference//_index.md), [MiyuStore - Documentation Fondatrice](../MiyuStore/MiyuStore%20-%20Documentation%20Fondatrice.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuShipping sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). RÃ©fÃ©rence : [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

- **Domaine** : `commerce` â€” cohÃ©rent avec la projection domains.json (blocs du domaine Â« commerce Â»).
- **Layer** : outil / toolkit (Strate 6) â€” Ã  reflÃ©ter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuShipping est une unitÃ© logique pouvant devenir un **bloc MSCM** Ã  l'implÃ©mentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) |
| Ã‰quivalents Boutique CMS Reservation SaaS | [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](..//..//miyukini-webway-system//reference//_index.md) |
| MiyuStore - Documentation Fondatrice | [MiyuStore - Documentation Fondatrice](../MiyuStore/MiyuStore%20-%20Documentation%20Fondatrice.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


