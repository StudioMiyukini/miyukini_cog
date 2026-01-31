# MiyuComptaLedger — Documentation Fondatrice

## 1. Contexte

**MiyuComptaLedger** est le **kit d'outils (Toolkit)** de tenue des livres comptables (synchronisation bancaire, écritures, catégorisation, TVA, rapprochement, structure) de l'écosystème Miyukini. Il intègre les outils de synchro bancaire, de catégorisation des écritures, de résolution TVA, de rapprochement et de résolution de structure juridique, alignés sur [Équivalents Comptabilité Indépendants](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Comptabilite%20Independants.md).

L'autorité sur les données (écritures, transactions bancaires, règles de catégorisation, paramètres TVA, structures) appartient à **KindMother**. MiyuComptaLedger expose des capacités d'exécution gouvernée ; la validation des rapprochements relève de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuComptaLedger, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'implémentation détaillée (API/EBICS/agrégateurs bancaires) ; les rapports (voir MiyuComptaReports).

---

## 3. Définition canonique

> **MiyuComptaLedger est une composition officielle d'outils de tenue des livres (banque, écritures, rapprochement, TVA, structure), déclarée et gouvernée par l'environnement.**

- MiyuComptaLedger **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuComptaLedger **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; validation rapprochement = StrongFather.

**Règle fondamentale :** Toute écriture (bank.sync, transaction.categorize, reconciliation.record, company.structure.register) = WriteIntent vers KindMother.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.compta.ledger` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `compta` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuComptaLedger - Reference Outils](./MiyuComptaLedger%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.compta.bank.sync` | Déclenche ou enregistre une synchronisation bancaire (API/EBICS/agrégateur) |
| `tool.compta.transaction.categorize` | Catégorise une écriture (exécution ; règles fournies) |
| `tool.compta.transaction.vat.resolve` | Rattache un taux TVA à une écriture |
| `tool.compta.reconciliation.suggest` | Propose des rapprochements (sans décider) |
| `tool.compta.reconciliation.record` | Enregistre un rapprochement validé ; autorisation = StrongFather |
| `tool.company.structure.resolve` | Résout la structure juridique courante (micro, EURL, etc.) pour le contexte |
| `tool.company.structure.register` | Enregistre une structure (WriteIntent KindMother) |
| `tool.company.siret.resolve` | Récupère les informations depuis SIRET/INSEE (lecture seule) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuComptaLedger en contient huit.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : validation rapprochement = StrongFather ; toute écriture = WriteIntent KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 2** (données bancaires sensibles) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données : écritures, transactions bancaires, règles de catégorisation, paramètres TVA, structures. Toute écriture passe par **WriteIntent** sous autorité KindMother. MiyuComptaLedger exécute des capacités atomiques ; validation rapprochement = StrongFather.

Les obligations de conformité détaillées sont dans [MiyuComptaLedger - Tool Governance Compliance Contract](./contracts/governance/MiyuComptaLedger%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuComptaLedger sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Tools MiyuComptaLedger devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

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
