# MiyuPosKitchen — Documentation Fondatrice

## 1. Contexte

**MiyuPosKitchen** est le **kit d'outils (Toolkit)** restaurant / bar (cuisine, affichage cuisine, type de service, tickets prédéfinis) de l'écosystème Miyukini. Il intègre les outils d'envoi commande à l'imprimante cuisine, d'affichage cuisine (push ordre, mise à jour statut), de définition du type de service (sur place / à emporter / livraison) et d'assignation de libellé prédéfini aux tickets (ex. Table 1, Table 2), alignés sur le document [Équivalents PoS Logiciel Caisse](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20PoS%20Logiciel%20Caisse.md).

Les données (ordres cuisine, statuts) relèvent du flux ou de **KindMother**. MiyuPosKitchen expose des capacités d'exécution gouvernée ; les Opérateurs (ex. Opérateur Restaurant) passent par la gouvernance pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuPosKitchen, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother, l'alignement MIP.

**Hors scope :** l'implémentation détaillée (pilotes imprimante, affichage physique).

---

## 3. Définition canonique

> **MiyuPosKitchen est une composition officielle d'outils restaurant / bar (imprimante cuisine, affichage cuisine, type de service, tickets prédéfinis), déclarée et gouvernée par l'environnement.**

- MiyuPosKitchen **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuPosKitchen **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (envoi à l'imprimante, push affichage, assignation type de service / libellé ticket).

**Règle fondamentale :** Les Tools opèrent sur des données fournies dans le flux ; toute persistance (ordre cuisine, statut) = WriteIntent KindMother si applicable.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.pos.miyuposkitchen` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `pos` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuPosKitchen - Reference Outils](./MiyuPosKitchen%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.pos.kitchen.print` | Envoie la commande à l'imprimante cuisine (données fournies) |
| `tool.pos.kitchen.order.push` | Envoie un ordre à l'affichage cuisine |
| `tool.pos.kitchen.order.update_status` | Met à jour le statut d'un ordre cuisine (en cours, prêt) |
| `tool.pos.order.service_type.set` | Définit le type de service (sur place / à emporter / livraison) |
| `tool.pos.ticket.preset.assign` | Assigne un libellé prédéfini (ex. Table 1) à un ticket |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuPosKitchen en contient cinq.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : kit lecture/affichage cuisine ; toute persistance éventuelle = WriteIntent KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **1** (détail par outil dans Reference Outils) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données d'ordres cuisine et statuts si persistés. Les Tools MiyuPosKitchen exécutent des capacités atomiques (impression, affichage, assignation) ; les données sont fournies dans le flux ou persistées via WriteIntent KindMother.

Les obligations de conformité détaillées sont dans [MiyuPosKitchen - Tool Governance Compliance Contract](./contracts/governance/MiyuPosKitchen%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuPosKitchen sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Tools MiyuPosKitchen devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

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
