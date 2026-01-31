# MiyuPosInventory — Documentation Fondatrice

## 1. Contexte

**MiyuPosInventory** est le **kit d'outils (Toolkit)** d'inventaire multi-magasin de l'écosystème Miyukini. Il intègre les outils de stock (lecture, ajustement), import d'articles, alertes stock bas, bons de commande fournisseur, transferts entre sites, inventaire physique (comptage, réconciliation), production (recettes/composants), étiquettes codes-barres, historique et valorisation, alignés sur le document [Équivalents PoS Logiciel Caisse](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20PoS%20Logiciel%20Caisse.md).

L'autorité sur les données (articles, stock, mouvements, bons de commande, transferts, comptages) appartient à **KindMother** (Core de données, Strate 4). MiyuPosInventory expose des capacités d'exécution gouvernée sans remplacer KindMother ni StrongFather ; les Opérateurs (ex. Opérateur Inventaire) passent par la gouvernance pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuPosInventory, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother, l'alignement MIP.

**Hors scope :** l'implémentation détaillée ; toute décision (ajustement stock, transfert, réconciliation) — ressort de StrongFather.

---

## 3. Définition canonique

> **MiyuPosInventory est une composition officielle d'outils d'inventaire (stock, import, alertes, bons de commande, transferts, comptages, production, étiquettes, historique, valorisation), déclarée et gouvernée par l'environnement.**

- MiyuPosInventory **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuPosInventory **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; les décisions (ajustement, transfert, réconciliation) appartiennent à StrongFather.

**Règle fondamentale :** Toute écriture passe par WriteIntent vers KindMother.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.pos.miyuposinventory` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `pos` / `inventory` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuPosInventory - Reference Outils](./MiyuPosInventory%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.inventory.stock.get` | Retourne le stock (et composants si applicable) pour un article/site |
| `tool.inventory.stock.adjust` | Ajuste le stock (réception, casse, perte) ; décision = StrongFather |
| `tool.inventory.import.items` | Importe des articles à partir d'un flux structuré (ex. CSV) |
| `tool.inventory.alert.low.evaluate` | Évalue les articles sous seuil bas |
| `tool.inventory.purchase_order.create` | Crée un bon de commande fournisseur |
| `tool.inventory.purchase_order.update` | Met à jour un bon de commande |
| `tool.inventory.purchase_order.track` | Retourne le statut / suivi d'un bon de commande |
| `tool.inventory.transfer.create` | Crée un transfert entre sites |
| `tool.inventory.transfer.execute` | Exécute (confirme) un transfert ; autorisation = StrongFather |
| `tool.inventory.transfer.list` | Liste les transferts |
| `tool.inventory.count.start` | Démarre une session d'inventaire physique |
| `tool.inventory.count.record` | Enregistre un comptage (article, quantité) pour une session |
| `tool.inventory.count.reconcile` | Clôture un comptage et propose/applique les écarts ; décision = StrongFather |
| `tool.inventory.production.record` | Enregistre une production (débit composants, crédit produit) |
| `tool.pos.label.print` | Imprime une étiquette code-barres |
| `tool.inventory.history.list` | Liste l'historique des mouvements |
| `tool.inventory.valuation.report` | Retourne un rapport de valorisation (lecture seule) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuPosInventory en contient dix-sept.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : décision métier (ajustement stock, transfert, réconciliation) = StrongFather ; toute persistance = WriteIntent KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 2** (détail par outil dans Reference Outils) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur toutes les données : articles, stock, mouvements, bons de commande, transferts, comptages. Toute écriture passe par **WriteIntent** sous autorité KindMother. MiyuPosInventory exécute des capacités atomiques ; il ne décide pas (ajustement, transfert, réconciliation = StrongFather).

Les obligations de conformité détaillées sont dans [MiyuPosInventory - Tool Governance Compliance Contract](./contracts/governance/MiyuPosInventory%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuPosInventory sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Tools MiyuPosInventory devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Équivalents PoS Logiciel Caisse | [Miyukini Conceptual References - Equivalents PoS Logiciel Caisse](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20PoS%20Logiciel%20Caisse.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
