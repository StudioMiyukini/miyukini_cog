# MiyuComptaReports â€” Documentation Fondatrice

## 1. Contexte

**MiyuComptaReports** est le **kit d'outils (Toolkit)** de rapports comptables (livre des recettes, bilan, compte de rÃ©sultat, liasse fiscale, flux de trÃ©sorerie, export Ã©critures) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de gÃ©nÃ©ration des rapports et d'export des Ã©critures, alignÃ©s sur [Ã‰quivalents ComptabilitÃ© IndÃ©pendants](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (Ã©critures, paramÃ¨tres rapports) appartient Ã  **KindMother**. MiyuComptaReports expose des capacitÃ©s de **lecture agrÃ©gÃ©e** et d'**export** ; l'autorisation d'export sensible (liasse, Ã©critures) relÃ¨ve de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuComptaReports, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'implÃ©mentation dÃ©taillÃ©e (formats liasse fiscale par rÃ©gime) ; la tenue des livres (voir MiyuComptaLedger).

---

## 3. DÃ©finition canonique

> **MiyuComptaReports est une composition officielle d'outils de rapports comptables (livre des recettes, bilan, liasse, flux de trÃ©sorerie, export Ã©critures), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuComptaReports **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuComptaReports **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques de gÃ©nÃ©ration et export ; autorisation export sensible = StrongFather.

**RÃ¨gle fondamentale :** Les rapports sont gÃ©nÃ©rÃ©s Ã  partir des donnÃ©es KindMother (lecture). L'export (liasse, Ã©critures) = autorisation StrongFather.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.compta.reports` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `compta` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuComptaReports - Reference Outils](./MiyuComptaReports%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.compta.report.livre_recettes.generate` | GÃ©nÃ¨re le livre des recettes |
| `tool.compta.report.balance.generate` | GÃ©nÃ¨re bilan / compte de rÃ©sultat |
| `tool.compta.report.liasse.generate` | GÃ©nÃ¨re la liasse fiscale (export) ; autorisation = StrongFather |
| `tool.compta.report.cashflow.generate` | GÃ©nÃ¨re un rapport flux de trÃ©sorerie / prÃ©visionnel |
| `tool.compta.export.ledger` | Export des Ã©critures (format fourni) ; autorisation = StrongFather |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuComptaReports en contient cinq.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : export sensible = StrongFather ; les Tools lisent KindMother (rapports en lecture).

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **1 Ã  2** (export liasse, Ã©critures = sensible) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es : Ã©critures, paramÃ¨tres rapports. Les Tools de gÃ©nÃ©ration **lisent** les donnÃ©es KindMother ; ils n'Ã©crivent pas (sauf export si mandatÃ©). L'export (liasse, ledger) = autorisation StrongFather.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuComptaReports - Tool Governance Compliance Contract](./contracts/governance/MiyuComptaReports%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuComptaReports sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuComptaReports devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Ã‰quivalents ComptabilitÃ© IndÃ©pendants | [Miyukini Conceptual References - Equivalents Comptabilite Independants](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


