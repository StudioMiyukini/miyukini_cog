# MiyuPolls — Documentation Fondatrice

## 1. Contexte

**MiyuPolls** est le **kit d'outils (Toolkit)** de sondages (création, vote, résultats) de l'écosystème Miyukini. Il intègre les outils de création de sondage, de vote et de consultation des résultats, alignés sur [Équivalents Moteur Forum](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Moteur%20Forum.md).

L'autorité sur les données (sondages, options, votes) appartient à **KindMother**. MiyuPolls expose des capacités d'exécution gouvernée ; les décisions (création autorisée, vote autorisé, règles de clôture) relèvent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuPolls, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'affichage des sondages (MiyuWeb) ; l'implémentation détaillée (stockage, anonymat).

---

## 3. Définition canonique

> **MiyuPolls est une composition officielle d'outils de sondages (création, vote, résultats), déclarée et gouvernée par l'environnement.**

- MiyuPolls **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuPolls **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; décision (création, vote autorisé) = StrongFather.

**Règle fondamentale :** Toute écriture (sondage, vote) = **WriteIntent** vers KindMother.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.content.polls` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `content` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuPolls - Reference Outils](./MiyuPolls%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.poll.create` | Crée un sondage (question, options, règles fournies) ; autorisation = StrongFather |
| `tool.poll.vote` | Enregistre un vote (sondage, option fournis) ; autorisation = StrongFather |
| `tool.poll.list` | Liste les sondages (filtres fournis) |
| `tool.poll.result` | Récupère les résultats d'un sondage (agrégés) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuPolls en contient quatre.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : **décision (création, vote autorisé, clôture) = StrongFather** ; toute écriture = WriteIntent KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **1 à 2** (données de vote sensibles selon politique) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données : sondages, options, votes. Toute création (sondage, vote) passe par **WriteIntent** vers KindMother.

Les obligations de conformité détaillées sont dans [MiyuPolls - Tool Governance Compliance Contract](./contracts/governance/MiyuPolls%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuPolls sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Tools MiyuPolls devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Équivalents Moteur Forum | [Miyukini Conceptual References - Equivalents Moteur Forum](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Moteur%20Forum.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
