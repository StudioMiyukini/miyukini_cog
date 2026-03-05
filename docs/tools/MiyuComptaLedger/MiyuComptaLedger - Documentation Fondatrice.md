# MiyuComptaLedger â€” Documentation Fondatrice

## 1. Contexte

**MiyuComptaLedger** est le **kit d'outils (Toolkit)** de tenue des livres comptables (synchronisation bancaire, Ã©critures, catÃ©gorisation, TVA, rapprochement, structure) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de synchro bancaire, de catÃ©gorisation des Ã©critures, de rÃ©solution TVA, de rapprochement et de rÃ©solution de structure juridique, alignÃ©s sur [Ã‰quivalents ComptabilitÃ© IndÃ©pendants](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (Ã©critures, transactions bancaires, rÃ¨gles de catÃ©gorisation, paramÃ¨tres TVA, structures) appartient Ã  **KindMother**. MiyuComptaLedger expose des capacitÃ©s d'exÃ©cution gouvernÃ©e ; la validation des rapprochements relÃ¨ve de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuComptaLedger, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'implÃ©mentation dÃ©taillÃ©e (API/EBICS/agrÃ©gateurs bancaires) ; les rapports (voir MiyuComptaReports).

---

## 3. DÃ©finition canonique

> **MiyuComptaLedger est une composition officielle d'outils de tenue des livres (banque, Ã©critures, rapprochement, TVA, structure), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuComptaLedger **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuComptaLedger **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; validation rapprochement = StrongFather.

**RÃ¨gle fondamentale :** Toute Ã©criture (bank.sync, transaction.categorize, reconciliation.record, company.structure.register) = WriteIntent vers KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.compta.ledger` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `compta` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuComptaLedger - Reference Outils](./MiyuComptaLedger%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.compta.bank.sync` | DÃ©clenche ou enregistre une synchronisation bancaire (API/EBICS/agrÃ©gateur) |
| `tool.compta.transaction.categorize` | CatÃ©gorise une Ã©criture (exÃ©cution ; rÃ¨gles fournies) |
| `tool.compta.transaction.vat.resolve` | Rattache un taux TVA Ã  une Ã©criture |
| `tool.compta.reconciliation.suggest` | Propose des rapprochements (sans dÃ©cider) |
| `tool.compta.reconciliation.record` | Enregistre un rapprochement validÃ© ; autorisation = StrongFather |
| `tool.company.structure.resolve` | RÃ©sout la structure juridique courante (micro, EURL, etc.) pour le contexte |
| `tool.company.structure.register` | Enregistre une structure (WriteIntent KindMother) |
| `tool.company.siret.resolve` | RÃ©cupÃ¨re les informations depuis SIRET/INSEE (lecture seule) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuComptaLedger en contient huit.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : validation rapprochement = StrongFather ; toute Ã©criture = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **0 Ã  2** (donnÃ©es bancaires sensibles) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es : Ã©critures, transactions bancaires, rÃ¨gles de catÃ©gorisation, paramÃ¨tres TVA, structures. Toute Ã©criture passe par **WriteIntent** sous autoritÃ© KindMother. MiyuComptaLedger exÃ©cute des capacitÃ©s atomiques ; validation rapprochement = StrongFather.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuComptaLedger - Tool Governance Compliance Contract](./contracts/governance/MiyuComptaLedger%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuComptaLedger sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuComptaLedger devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

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


