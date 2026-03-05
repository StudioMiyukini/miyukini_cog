# MiyuDeclarations â€” Documentation Fondatrice

## 1. Contexte

**MiyuDeclarations** est le **kit d'outils (Toolkit)** de dÃ©clarations fiscales et sociales (URSSAF, TVA, Ã©chÃ©ances, historique, estimateur cotisations) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de prÃ©paration et soumission des dÃ©clarations URSSAF et TVA, de liste des Ã©chÃ©ances et de l'historique, et d'estimation des cotisations (micro), alignÃ©s sur [Ã‰quivalents ComptabilitÃ© IndÃ©pendants](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (donnÃ©es dÃ©clarations, historique, calendrier Ã©chÃ©ances) appartient Ã  **KindMother**. MiyuDeclarations expose des capacitÃ©s d'exÃ©cution gouvernÃ©e ; la **soumission** des dÃ©clarations (URSSAF, TVA) relÃ¨ve de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuDeclarations, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'implÃ©mentation dÃ©taillÃ©e (tÃ©lÃ©dÃ©claration URSSAF, formulaires TVA) ; les rÃ¨gles fiscales par pays.

---

## 3. DÃ©finition canonique

> **MiyuDeclarations est une composition officielle d'outils de dÃ©clarations fiscales et sociales (URSSAF, TVA, Ã©chÃ©ances, historique, estimateur cotisations), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuDeclarations **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuDeclarations **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; soumission dÃ©claration = StrongFather.

**RÃ¨gle fondamentale :** PrÃ©paration = exÃ©cution (donnÃ©es fournies). Soumission = dÃ©cision StrongFather. Toute Ã©criture (historique dÃ©claration) = WriteIntent vers KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.compta.declarations` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `compta` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuDeclarations - Reference Outils](./MiyuDeclarations%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.compta.declaration.urssaf.prepare` | PrÃ©pare les donnÃ©es de dÃ©claration URSSAF (CA, etc.) |
| `tool.compta.declaration.urssaf.submit` | Soumet la dÃ©claration URSSAF (tÃ©lÃ©dÃ©claration) ; autorisation = StrongFather |
| `tool.compta.declaration.tva.prepare` | PrÃ©pare la dÃ©claration TVA |
| `tool.compta.declaration.tva.submit` | Soumet la dÃ©claration TVA ; autorisation = StrongFather |
| `tool.compta.declaration.deadline.list` | Liste les Ã©chÃ©ances fiscales et sociales (donnÃ©es fournies) |
| `tool.compta.declaration.list` | Liste l'historique des dÃ©clarations (filtres fournis) |
| `tool.compta.declaration.estimate.cotisations` | Calcule une estimation des cotisations (micro) Ã  partir de CA fourni |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuDeclarations en contient sept.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : soumission dÃ©claration = StrongFather ; toute Ã©criture (historique) = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **0 Ã  2** (soumission dÃ©claration = sensible) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es : donnÃ©es dÃ©clarations, historique, calendrier Ã©chÃ©ances. Toute Ã©criture (historique aprÃ¨s soumission) passe par **WriteIntent** sous autoritÃ© KindMother. MiyuDeclarations exÃ©cute des capacitÃ©s atomiques ; soumission = StrongFather.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuDeclarations - Tool Governance Compliance Contract](./contracts/governance/MiyuDeclarations%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuDeclarations sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuDeclarations devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

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


