# MiyuExpense â€” Documentation Fondatrice

## 1. Contexte

**MiyuExpense** est le **kit d'outils (Toolkit)** de notes de frais et indemnitÃ©s (justificatifs, OCR, notes de frais, validation, indemnitÃ©s kilomÃ©triques, export vers compta) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de capture et extraction OCR des justificatifs, de crÃ©ation et validation des notes de frais, de calcul et export des indemnitÃ©s kilomÃ©triques, et d'export vers la comptabilitÃ©, alignÃ©s sur [Ã‰quivalents ComptabilitÃ© IndÃ©pendants](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (justificatifs, notes de frais, barÃ¨me kilomÃ©trique, validations) appartient Ã  **KindMother**. MiyuExpense expose des capacitÃ©s d'exÃ©cution gouvernÃ©e ; la **validation** des notes de frais et l'**export** vers compta relÃ¨vent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuExpense, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'implÃ©mentation dÃ©taillÃ©e (OCR, barÃ¨me kilomÃ©trique par pays) ; la tenue des livres (voir MiyuComptaLedger).

---

## 3. DÃ©finition canonique

> **MiyuExpense est une composition officielle d'outils de notes de frais et indemnitÃ©s (justificatifs, OCR, notes de frais, validation, indemnitÃ©s kilomÃ©triques, export), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuExpense **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuExpense **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; validation note de frais et export = StrongFather.

**RÃ¨gle fondamentale :** Toute Ã©criture (receipt.capture, claim.create, claim.update, claim.validate) = WriteIntent vers KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.expense.claims` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `expense` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuExpense - Reference Outils](./MiyuExpense%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.expense.receipt.capture` | Enregistre un justificatif (photo/scan) ; WriteIntent KindMother |
| `tool.expense.receipt.extract` | Extrait les donnÃ©es d'un justificatif par OCR (exÃ©cution seule) |
| `tool.expense.claim.create` | CrÃ©e une note de frais Ã  partir de donnÃ©es fournies |
| `tool.expense.claim.update` | Met Ã  jour une note de frais |
| `tool.expense.claim.list` | Liste les notes de frais (filtres fournis) |
| `tool.expense.claim.validate` | Valide une note de frais (workflow ; dÃ©cision = StrongFather) |
| `tool.expense.mileage.calculate` | Calcule les indemnitÃ©s kilomÃ©triques selon barÃ¨me fourni |
| `tool.expense.mileage.export` | Export PDF/CSV des indemnitÃ©s pour administration |
| `tool.expense.claim.export` | Export des notes de frais vers compta ; autorisation = StrongFather |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuExpense en contient neuf.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : validation note de frais et export = StrongFather ; toute Ã©criture = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **0 Ã  2** (validation, export = sensible) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es : justificatifs, notes de frais, barÃ¨me kilomÃ©trique, validations. Toute Ã©criture (receipt.capture, claim.create, claim.update, claim.validate) passe par **WriteIntent** sous autoritÃ© KindMother. MiyuExpense exÃ©cute des capacitÃ©s atomiques ; validation et export = StrongFather.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuExpense - Tool Governance Compliance Contract](./contracts/governance/MiyuExpense%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuExpense sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuExpense devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

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


