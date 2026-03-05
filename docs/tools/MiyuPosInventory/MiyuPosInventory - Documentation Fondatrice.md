# MiyuPosInventory â€” Documentation Fondatrice

## 1. Contexte

**MiyuPosInventory** est le **kit d'outils (Toolkit)** d'inventaire multi-magasin de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de stock (lecture, ajustement), import d'articles, alertes stock bas, bons de commande fournisseur, transferts entre sites, inventaire physique (comptage, rÃ©conciliation), production (recettes/composants), Ã©tiquettes codes-barres, historique et valorisation, alignÃ©s sur le document [Ã‰quivalents PoS Logiciel Caisse](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (articles, stock, mouvements, bons de commande, transferts, comptages) appartient Ã  **KindMother** (Core de donnÃ©es, Strate 4). MiyuPosInventory expose des capacitÃ©s d'exÃ©cution gouvernÃ©e sans remplacer KindMother ni StrongFather ; les OpÃ©rateurs (ex. OpÃ©rateur Inventaire) passent par la gouvernance pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuPosInventory, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother, l'alignement MIP.

**Hors scope :** l'implÃ©mentation dÃ©taillÃ©e ; toute dÃ©cision (ajustement stock, transfert, rÃ©conciliation) â€” ressort de StrongFather.

---

## 3. DÃ©finition canonique

> **MiyuPosInventory est une composition officielle d'outils d'inventaire (stock, import, alertes, bons de commande, transferts, comptages, production, Ã©tiquettes, historique, valorisation), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuPosInventory **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuPosInventory **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; les dÃ©cisions (ajustement, transfert, rÃ©conciliation) appartiennent Ã  StrongFather.

**RÃ¨gle fondamentale :** Toute Ã©criture passe par WriteIntent vers KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.pos.miyuposinventory` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `pos` / `inventory` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuPosInventory - Reference Outils](./MiyuPosInventory%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.inventory.stock.get` | Retourne le stock (et composants si applicable) pour un article/site |
| `tool.inventory.stock.adjust` | Ajuste le stock (rÃ©ception, casse, perte) ; dÃ©cision = StrongFather |
| `tool.inventory.import.items` | Importe des articles Ã  partir d'un flux structurÃ© (ex. CSV) |
| `tool.inventory.alert.low.evaluate` | Ã‰value les articles sous seuil bas |
| `tool.inventory.purchase_order.create` | CrÃ©e un bon de commande fournisseur |
| `tool.inventory.purchase_order.update` | Met Ã  jour un bon de commande |
| `tool.inventory.purchase_order.track` | Retourne le statut / suivi d'un bon de commande |
| `tool.inventory.transfer.create` | CrÃ©e un transfert entre sites |
| `tool.inventory.transfer.execute` | ExÃ©cute (confirme) un transfert ; autorisation = StrongFather |
| `tool.inventory.transfer.list` | Liste les transferts |
| `tool.inventory.count.start` | DÃ©marre une session d'inventaire physique |
| `tool.inventory.count.record` | Enregistre un comptage (article, quantitÃ©) pour une session |
| `tool.inventory.count.reconcile` | ClÃ´ture un comptage et propose/applique les Ã©carts ; dÃ©cision = StrongFather |
| `tool.inventory.production.record` | Enregistre une production (dÃ©bit composants, crÃ©dit produit) |
| `tool.pos.label.print` | Imprime une Ã©tiquette code-barres |
| `tool.inventory.history.list` | Liste l'historique des mouvements |
| `tool.inventory.valuation.report` | Retourne un rapport de valorisation (lecture seule) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuPosInventory en contient dix-sept.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : dÃ©cision mÃ©tier (ajustement stock, transfert, rÃ©conciliation) = StrongFather ; toute persistance = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **0 Ã  2** (dÃ©tail par outil dans Reference Outils) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur toutes les donnÃ©es : articles, stock, mouvements, bons de commande, transferts, comptages. Toute Ã©criture passe par **WriteIntent** sous autoritÃ© KindMother. MiyuPosInventory exÃ©cute des capacitÃ©s atomiques ; il ne dÃ©cide pas (ajustement, transfert, rÃ©conciliation = StrongFather).

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuPosInventory - Tool Governance Compliance Contract](./contracts/governance/MiyuPosInventory%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuPosInventory sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuPosInventory devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Ã‰quivalents PoS Logiciel Caisse | [Miyukini Conceptual References - Equivalents PoS Logiciel Caisse](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


