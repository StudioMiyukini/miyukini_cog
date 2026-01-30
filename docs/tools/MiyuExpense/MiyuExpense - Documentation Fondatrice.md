# MiyuExpense — Documentation Fondatrice

## 1. Contexte

**MiyuExpense** est le **kit d'outils (Toolkit)** de notes de frais et indemnités (justificatifs, OCR, notes de frais, validation, indemnités kilométriques, export vers compta) de l'écosystème Miyukini. Il intègre les outils de capture et extraction OCR des justificatifs, de création et validation des notes de frais, de calcul et export des indemnités kilométriques, et d'export vers la comptabilité, alignés sur [Équivalents Comptabilité Indépendants](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Comptabilite%20Independants.md).

L'autorité sur les données (justificatifs, notes de frais, barème kilométrique, validations) appartient à **KindMother**. MiyuExpense expose des capacités d'exécution gouvernée ; la **validation** des notes de frais et l'**export** vers compta relèvent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuExpense, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'implémentation détaillée (OCR, barème kilométrique par pays) ; la tenue des livres (voir MiyuComptaLedger).

---

## 3. Définition canonique

> **MiyuExpense est une composition officielle d'outils de notes de frais et indemnités (justificatifs, OCR, notes de frais, validation, indemnités kilométriques, export), déclarée et gouvernée par l'environnement.**

- MiyuExpense **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuExpense **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; validation note de frais et export = StrongFather.

**Règle fondamentale :** Toute écriture (receipt.capture, claim.create, claim.update, claim.validate) = WriteIntent vers KindMother.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.expense.claims` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `expense` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuExpense - Reference Outils](./MiyuExpense%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.expense.receipt.capture` | Enregistre un justificatif (photo/scan) ; WriteIntent KindMother |
| `tool.expense.receipt.extract` | Extrait les données d'un justificatif par OCR (exécution seule) |
| `tool.expense.claim.create` | Crée une note de frais à partir de données fournies |
| `tool.expense.claim.update` | Met à jour une note de frais |
| `tool.expense.claim.list` | Liste les notes de frais (filtres fournis) |
| `tool.expense.claim.validate` | Valide une note de frais (workflow ; décision = StrongFather) |
| `tool.expense.mileage.calculate` | Calcule les indemnités kilométriques selon barème fourni |
| `tool.expense.mileage.export` | Export PDF/CSV des indemnités pour administration |
| `tool.expense.claim.export` | Export des notes de frais vers compta ; autorisation = StrongFather |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuExpense en contient neuf.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : validation note de frais et export = StrongFather ; toute écriture = WriteIntent KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 2** (validation, export = sensible) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données : justificatifs, notes de frais, barème kilométrique, validations. Toute écriture (receipt.capture, claim.create, claim.update, claim.validate) passe par **WriteIntent** sous autorité KindMother. MiyuExpense exécute des capacités atomiques ; validation et export = StrongFather.

Les obligations de conformité détaillées sont dans [MiyuExpense - Tool Governance Compliance Contract](./contracts/governance/MiyuExpense%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Équivalents Comptabilité Indépendants | [Miyukini Conceptual References - Equivalents Comptabilite Independants](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Comptabilite%20Independants.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
