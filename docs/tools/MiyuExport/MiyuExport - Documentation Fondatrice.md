# MiyuExport — Documentation Fondatrice

## 1. Contexte

**MiyuExport** est le **kit d'outils (Toolkit)** d'export et de génération de documents de l'écosystème Miyukini. Il intègre les outils de génération CSV, Excel (XLSX) et PDF à partir de données et templates fournis, sans logique métier — les données à exporter et les options (délimiteurs, locale, template) sont fournies dans le flux gouverné ; la décision d'exporter ou de publier relève de **StrongFather**.

L'autorité sur les données métier appartient à **KindMother**. MiyuExport expose des capacités d'exécution gouvernée (générer CSV, XLSX, PDF) ; les décisions sur ce qui doit être exporté ou rendu relèvent de **StrongFather** et des Opérateurs.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuExport, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'implémentation détaillée (moteur PDF, bibliothèques Excel) ; les règles métier d'export (qui exporte quoi, quand) = StrongFather / Opérateurs.

---

## 3. Définition canonique

> **MiyuExport est une composition officielle d'outils d'export et de génération de documents (CSV, XLSX, PDF), déclarée et gouvernée par l'environnement.**

- MiyuExport **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuExport **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (générer CSV, XLSX, PDF à partir de données et options fournis) ; pas de décision sur le périmètre ni le contenu à exporter.

**Règle fondamentale :** Un Tool MiyuExport exécute sur des **données et options fournies** ; il ne lit pas la base directement — les données sont fournies dans le flux (après lecture via MiyuSQL sous autorité KindMother).

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.export.miyuexport` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `export` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuExport - Reference Outils](./MiyuExport%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.export.csv.generate` | Génère un fichier CSV à partir de données et options fournis (délimiteur, encodage, locale) |
| `tool.export.xlsx.generate` | Génère un fichier Excel (XLSX) à partir de données et options fournis (feuilles, format) |
| `tool.export.pdf.render` | Rend un PDF à partir d'un template et de données fournis ; ne décide pas du contenu |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuExport en contient trois.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : données et options fournies dans le flux ; décision d'export = StrongFather ; MiyuExport n'écrit pas la base métier (produit un flux binaire ou fichier).

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **1 à 3** (données exportées peuvent être sensibles) |
| **États autorisés** | Tous sauf restriction WorrySentinel |
| **États interdits** | Selon politique (ex. blocage export en SECURITY_LOCKDOWN) |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données. MiyuExport **ne lit pas** la base directement : les données à exporter sont **fournies dans le flux** (après lecture via MiyuSQL sous autorité KindMother). MiyuExport produit un flux binaire (CSV, XLSX, PDF) ; pas d'écriture en base métier par MiyuExport.

Les obligations de conformité détaillées sont dans [MiyuExport - Tool Governance Compliance Contract](./contracts/governance/MiyuExport%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

À l'implémentation : chaque Tool MiyuExport est une unité logique pouvant devenir un **bloc MSCM** : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
