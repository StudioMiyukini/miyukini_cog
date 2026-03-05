# MiyuExport â€” Documentation Fondatrice

## 1. Contexte

**MiyuExport** est le **kit d'outils (Toolkit)** d'export et de gÃ©nÃ©ration de documents de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de gÃ©nÃ©ration CSV, Excel (XLSX) et PDF Ã  partir de donnÃ©es et templates fournis, sans logique mÃ©tier â€” les donnÃ©es Ã  exporter et les options (dÃ©limiteurs, locale, template) sont fournies dans le flux gouvernÃ© ; la dÃ©cision d'exporter ou de publier relÃ¨ve de **StrongFather**.

L'autoritÃ© sur les donnÃ©es mÃ©tier appartient Ã  **KindMother**. MiyuExport expose des capacitÃ©s d'exÃ©cution gouvernÃ©e (gÃ©nÃ©rer CSV, XLSX, PDF) ; les dÃ©cisions sur ce qui doit Ãªtre exportÃ© ou rendu relÃ¨vent de **StrongFather** et des OpÃ©rateurs.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuExport, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'implÃ©mentation dÃ©taillÃ©e (moteur PDF, bibliothÃ¨ques Excel) ; les rÃ¨gles mÃ©tier d'export (qui exporte quoi, quand) = StrongFather / OpÃ©rateurs.

---

## 3. DÃ©finition canonique

> **MiyuExport est une composition officielle d'outils d'export et de gÃ©nÃ©ration de documents (CSV, XLSX, PDF), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuExport **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuExport **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (gÃ©nÃ©rer CSV, XLSX, PDF Ã  partir de donnÃ©es et options fournis) ; pas de dÃ©cision sur le pÃ©rimÃ¨tre ni le contenu Ã  exporter.

**RÃ¨gle fondamentale :** Un Tool MiyuExport exÃ©cute sur des **donnÃ©es et options fournies** ; il ne lit pas la base directement â€” les donnÃ©es sont fournies dans le flux (aprÃ¨s lecture via MiyuSQL sous autoritÃ© KindMother).

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.export.miyuexport` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `export` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuExport - Reference Outils](./MiyuExport%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.export.csv.generate` | GÃ©nÃ¨re un fichier CSV Ã  partir de donnÃ©es et options fournis (dÃ©limiteur, encodage, locale) |
| `tool.export.xlsx.generate` | GÃ©nÃ¨re un fichier Excel (XLSX) Ã  partir de donnÃ©es et options fournis (feuilles, format) |
| `tool.export.pdf.render` | Rend un PDF Ã  partir d'un template et de donnÃ©es fournis ; ne dÃ©cide pas du contenu |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuExport en contient trois.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : donnÃ©es et options fournies dans le flux ; dÃ©cision d'export = StrongFather ; MiyuExport n'Ã©crit pas la base mÃ©tier (produit un flux binaire ou fichier).

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **1 Ã  3** (donnÃ©es exportÃ©es peuvent Ãªtre sensibles) |
| **Ã‰tats autorisÃ©s** | Tous sauf restriction WorrySentinel |
| **Ã‰tats interdits** | Selon politique (ex. blocage export en SECURITY_LOCKDOWN) |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es. MiyuExport **ne lit pas** la base directement : les donnÃ©es Ã  exporter sont **fournies dans le flux** (aprÃ¨s lecture via MiyuSQL sous autoritÃ© KindMother). MiyuExport produit un flux binaire (CSV, XLSX, PDF) ; pas d'Ã©criture en base mÃ©tier par MiyuExport.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuExport - Tool Governance Compliance Contract](./contracts/governance/MiyuExport%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

Ã€ l'implÃ©mentation : chaque Tool MiyuExport est une unitÃ© logique pouvant devenir un **bloc MSCM** : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


