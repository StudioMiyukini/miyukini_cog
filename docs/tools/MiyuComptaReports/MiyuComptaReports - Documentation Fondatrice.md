# MiyuComptaReports — Documentation Fondatrice

## 1. Contexte

**MiyuComptaReports** est le **kit d'outils (Toolkit)** de rapports comptables (livre des recettes, bilan, compte de résultat, liasse fiscale, flux de trésorerie, export écritures) de l'écosystème Miyukini. Il intègre les outils de génération des rapports et d'export des écritures, alignés sur [Équivalents Comptabilité Indépendants](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Comptabilite%20Independants.md).

L'autorité sur les données (écritures, paramètres rapports) appartient à **KindMother**. MiyuComptaReports expose des capacités de **lecture agrégée** et d'**export** ; l'autorisation d'export sensible (liasse, écritures) relève de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuComptaReports, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'implémentation détaillée (formats liasse fiscale par régime) ; la tenue des livres (voir MiyuComptaLedger).

---

## 3. Définition canonique

> **MiyuComptaReports est une composition officielle d'outils de rapports comptables (livre des recettes, bilan, liasse, flux de trésorerie, export écritures), déclarée et gouvernée par l'environnement.**

- MiyuComptaReports **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuComptaReports **n'ajoute aucune logique métier** : il orchestre des capacités atomiques de génération et export ; autorisation export sensible = StrongFather.

**Règle fondamentale :** Les rapports sont générés à partir des données KindMother (lecture). L'export (liasse, écritures) = autorisation StrongFather.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.compta.reports` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `compta` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuComptaReports - Reference Outils](./MiyuComptaReports%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.compta.report.livre_recettes.generate` | Génère le livre des recettes |
| `tool.compta.report.balance.generate` | Génère bilan / compte de résultat |
| `tool.compta.report.liasse.generate` | Génère la liasse fiscale (export) ; autorisation = StrongFather |
| `tool.compta.report.cashflow.generate` | Génère un rapport flux de trésorerie / prévisionnel |
| `tool.compta.export.ledger` | Export des écritures (format fourni) ; autorisation = StrongFather |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuComptaReports en contient cinq.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : export sensible = StrongFather ; les Tools lisent KindMother (rapports en lecture).

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **1 à 2** (export liasse, écritures = sensible) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données : écritures, paramètres rapports. Les Tools de génération **lisent** les données KindMother ; ils n'écrivent pas (sauf export si mandaté). L'export (liasse, ledger) = autorisation StrongFather.

Les obligations de conformité détaillées sont dans [MiyuComptaReports - Tool Governance Compliance Contract](./contracts/governance/MiyuComptaReports%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuComptaReports sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Tools MiyuComptaReports devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Équivalents Comptabilité Indépendants | [Miyukini Conceptual References - Equivalents Comptabilite Independants](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Comptabilite%20Independants.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
