# MiyuPosSales â€” Documentation Fondatrice

## 1. Contexte

**MiyuPosSales** est le **kit d'outils (Toolkit)** de caisse et ventes au point de vente (PoS) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils d'enregistrement des ventes (tickets, lignes, reÃ§us), remises, remboursements, gestion de caisse, variantes et modificateurs d'articles, codes-barres, contexte magasin et affichage client, alignÃ©s sur le document [Ã‰quivalents PoS Logiciel Caisse](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (ventes, tickets, reÃ§us, mouvements caisse) appartient Ã  **KindMother** (Core de donnÃ©es, Strate 4). MiyuPosSales expose des capacitÃ©s d'exÃ©cution gouvernÃ©e sans remplacer KindMother ni StrongFather ; les OpÃ©rateurs (ex. OpÃ©rateur Caisse) passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :**

- L'identitÃ© et la dÃ©finition canonique de MiyuPosSales
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliquÃ©s)
- Le niveau de sÃ©curitÃ© et les Ã©tats systÃ¨me autorisÃ©s ou interdits
- La relation avec KindMother (WriteIntent pour toute Ã©criture)
- L'alignement avec le protocole MIP v1 pour une future implÃ©mentation indexable

**Hors scope :**

- L'implÃ©mentation dÃ©taillÃ©e (persistance, matÃ©riel imprimante/tiroir)
- Toute dÃ©cision ALLOW/DENY (remboursement, remise) â€” ressort de StrongFather

---

## 3. DÃ©finition canonique

> **MiyuPosSales est une composition officielle d'outils de caisse et ventes PoS (ventes, tickets, reÃ§us, remises, remboursements, caisse, articles, codes-barres, contexte magasin, affichage client), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuPosSales **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuPosSales **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques sans dÃ©cider des autorisations (remboursement, remise = StrongFather).

**RÃ¨gle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacitÃ©. Toute Ã©criture passe par WriteIntent vers KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.pos.miyupossales` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `pos` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

MiyuPosSales est composÃ© des Tools suivants (format canonique `tool.pos.<sous-domaine>.<action>`). Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuPosSales - Reference Outils](./MiyuPosSales%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.pos.sale.create` | CrÃ©e une vente (ticket) |
| `tool.pos.sale.add_item` | Ajoute une ligne Ã  une vente |
| `tool.pos.sale.remove_item` | Retire une ligne |
| `tool.pos.ticket.open` | Ouvre un ticket |
| `tool.pos.ticket.save` | Sauvegarde un ticket |
| `tool.pos.ticket.close` | ClÃ´ture un ticket |
| `tool.pos.ticket.list` | Liste les tickets |
| `tool.pos.discount.apply` | Applique une remise |
| `tool.pos.refund.record` | Enregistre un remboursement ; autorisation = StrongFather |
| `tool.pos.receipt.render` | Produit le contenu du reÃ§u |
| `tool.pos.receipt.print` | Imprime le reÃ§u |
| `tool.pos.receipt.send` | Envoie le reÃ§u par email |
| `tool.pos.receipt.list` | Liste les reÃ§us |
| `tool.pos.item.variant.resolve` | RÃ©sout une variante article |
| `tool.pos.item.modifier.apply` | Applique des modificateurs |
| `tool.pos.cash.register.open` | Ouvre une session caisse |
| `tool.pos.cash.register.close` | ClÃ´ture une session caisse |
| `tool.pos.cash.movement.record` | Enregistre un mouvement espÃ¨ces |
| `tool.pos.barcode.parse` | Parse un code-barres |
| `tool.pos.context.store.resolve` | RÃ©sout le magasin courant |
| `tool.pos.display.push` | Envoie les donnÃ©es Ã  l'Ã©cran client |
| `tool.pos.order.service_type.set` | DÃ©finit le type de service (sur place / Ã  emporter / livraison) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuPosSales en contient vingt-deux.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : dÃ©cision mÃ©tier (remboursement, remise) = StrongFather ; toute Ã©criture = WriteIntent KindMother. Le Toolkit est dÃ©clarÃ© dans Master Butler et compatibilisÃ© par Ever Buddy ([Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)).

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **1 Ã  2** (dÃ©tail par outil dans Reference Outils) ; cohÃ©rent avec WorrySentinel. |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon Toolkit Composition Contract) |

---

## 8. Relation avec KindMother

- **KindMother** est l'autoritÃ© sur toutes les donnÃ©es : ventes, tickets, reÃ§us, mouvements caisse, catalogue. Toute Ã©criture (vente, ticket, reÃ§u, ouverture/fermeture caisse) passe par **WriteIntent** sous autoritÃ© KindMother.
- **MiyuPosSales** n'exÃ©cute que des capacitÃ©s atomiques ; il ne dÃ©cide pas (remboursement autorisÃ© ou non = StrongFather). Les donnÃ©es sont fournies dans le flux ou persistÃ©es via KindMother/MiyuSQL en amont.

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Equivalents PoS Logiciel Caisse](..//..//miyukini-webway-system//reference//_index.md).

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuPosSales - Tool Governance Compliance Contract](./contracts/governance/MiyuPosSales%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

- **Domaine** : `pos` â€” cohÃ©rent avec la projection domains.json.
- **Layer** : Strate 6 (Tools & Toolkits).
- **Blocs** : chaque Tool MiyuPosSales est une unitÃ© logique pouvant devenir un **bloc MSCM** Ã  l'implÃ©mentation.

Ã€ l'implÃ©mentation, le code devra Ãªtre balisÃ© MSCM afin que l'index MIP soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) |
| Ã‰quivalents PoS Logiciel Caisse | [Miyukini Conceptual References - Equivalents PoS Logiciel Caisse](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| Pyramide Architecture | [Miyukini Conceptual References - Pyramide Architecture Complete](..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


