# MiyuDeclarations — Documentation Fondatrice

## 1. Contexte

**MiyuDeclarations** est le **kit d'outils (Toolkit)** de déclarations fiscales et sociales (URSSAF, TVA, échéances, historique, estimateur cotisations) de l'écosystème Miyukini. Il intègre les outils de préparation et soumission des déclarations URSSAF et TVA, de liste des échéances et de l'historique, et d'estimation des cotisations (micro), alignés sur [Équivalents Comptabilité Indépendants](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Comptabilite%20Independants.md).

L'autorité sur les données (données déclarations, historique, calendrier échéances) appartient à **KindMother**. MiyuDeclarations expose des capacités d'exécution gouvernée ; la **soumission** des déclarations (URSSAF, TVA) relève de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuDeclarations, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'implémentation détaillée (télédéclaration URSSAF, formulaires TVA) ; les règles fiscales par pays.

---

## 3. Définition canonique

> **MiyuDeclarations est une composition officielle d'outils de déclarations fiscales et sociales (URSSAF, TVA, échéances, historique, estimateur cotisations), déclarée et gouvernée par l'environnement.**

- MiyuDeclarations **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuDeclarations **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; soumission déclaration = StrongFather.

**Règle fondamentale :** Préparation = exécution (données fournies). Soumission = décision StrongFather. Toute écriture (historique déclaration) = WriteIntent vers KindMother.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.compta.declarations` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `compta` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuDeclarations - Reference Outils](./MiyuDeclarations%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.compta.declaration.urssaf.prepare` | Prépare les données de déclaration URSSAF (CA, etc.) |
| `tool.compta.declaration.urssaf.submit` | Soumet la déclaration URSSAF (télédéclaration) ; autorisation = StrongFather |
| `tool.compta.declaration.tva.prepare` | Prépare la déclaration TVA |
| `tool.compta.declaration.tva.submit` | Soumet la déclaration TVA ; autorisation = StrongFather |
| `tool.compta.declaration.deadline.list` | Liste les échéances fiscales et sociales (données fournies) |
| `tool.compta.declaration.list` | Liste l'historique des déclarations (filtres fournis) |
| `tool.compta.declaration.estimate.cotisations` | Calcule une estimation des cotisations (micro) à partir de CA fourni |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuDeclarations en contient sept.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : soumission déclaration = StrongFather ; toute écriture (historique) = WriteIntent KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 2** (soumission déclaration = sensible) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données : données déclarations, historique, calendrier échéances. Toute écriture (historique après soumission) passe par **WriteIntent** sous autorité KindMother. MiyuDeclarations exécute des capacités atomiques ; soumission = StrongFather.

Les obligations de conformité détaillées sont dans [MiyuDeclarations - Tool Governance Compliance Contract](./contracts/governance/MiyuDeclarations%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuDeclarations sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Tools MiyuDeclarations devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

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
