# MiyuPosKitchen â€” Documentation Fondatrice

## 1. Contexte

**MiyuPosKitchen** est le **kit d'outils (Toolkit)** restaurant / bar (cuisine, affichage cuisine, type de service, tickets prÃ©dÃ©finis) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils d'envoi commande Ã  l'imprimante cuisine, d'affichage cuisine (push ordre, mise Ã  jour statut), de dÃ©finition du type de service (sur place / Ã  emporter / livraison) et d'assignation de libellÃ© prÃ©dÃ©fini aux tickets (ex. Table 1, Table 2), alignÃ©s sur le document [Ã‰quivalents PoS Logiciel Caisse](..//..//miyukini-webway-system//reference//_index.md).

Les donnÃ©es (ordres cuisine, statuts) relÃ¨vent du flux ou de **KindMother**. MiyuPosKitchen expose des capacitÃ©s d'exÃ©cution gouvernÃ©e ; les OpÃ©rateurs (ex. OpÃ©rateur Restaurant) passent par la gouvernance pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuPosKitchen, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother, l'alignement MIP.

**Hors scope :** l'implÃ©mentation dÃ©taillÃ©e (pilotes imprimante, affichage physique).

---

## 3. DÃ©finition canonique

> **MiyuPosKitchen est une composition officielle d'outils restaurant / bar (imprimante cuisine, affichage cuisine, type de service, tickets prÃ©dÃ©finis), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuPosKitchen **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuPosKitchen **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (envoi Ã  l'imprimante, push affichage, assignation type de service / libellÃ© ticket).

**RÃ¨gle fondamentale :** Les Tools opÃ¨rent sur des donnÃ©es fournies dans le flux ; toute persistance (ordre cuisine, statut) = WriteIntent KindMother si applicable.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.pos.miyuposkitchen` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `pos` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuPosKitchen - Reference Outils](./MiyuPosKitchen%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.pos.kitchen.print` | Envoie la commande Ã  l'imprimante cuisine (donnÃ©es fournies) |
| `tool.pos.kitchen.order.push` | Envoie un ordre Ã  l'affichage cuisine |
| `tool.pos.kitchen.order.update_status` | Met Ã  jour le statut d'un ordre cuisine (en cours, prÃªt) |
| `tool.pos.order.service_type.set` | DÃ©finit le type de service (sur place / Ã  emporter / livraison) |
| `tool.pos.ticket.preset.assign` | Assigne un libellÃ© prÃ©dÃ©fini (ex. Table 1) Ã  un ticket |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuPosKitchen en contient cinq.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : kit lecture/affichage cuisine ; toute persistance Ã©ventuelle = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **1** (dÃ©tail par outil dans Reference Outils) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es d'ordres cuisine et statuts si persistÃ©s. Les Tools MiyuPosKitchen exÃ©cutent des capacitÃ©s atomiques (impression, affichage, assignation) ; les donnÃ©es sont fournies dans le flux ou persistÃ©es via WriteIntent KindMother.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuPosKitchen - Tool Governance Compliance Contract](./contracts/governance/MiyuPosKitchen%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuPosKitchen sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuPosKitchen devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

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


