# MiyuPolls â€” Documentation Fondatrice

## 1. Contexte

**MiyuPolls** est le **kit d'outils (Toolkit)** de sondages (crÃ©ation, vote, rÃ©sultats) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de crÃ©ation de sondage, de vote et de consultation des rÃ©sultats, alignÃ©s sur [Ã‰quivalents Moteur Forum](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (sondages, options, votes) appartient Ã  **KindMother**. MiyuPolls expose des capacitÃ©s d'exÃ©cution gouvernÃ©e ; les dÃ©cisions (crÃ©ation autorisÃ©e, vote autorisÃ©, rÃ¨gles de clÃ´ture) relÃ¨vent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuPolls, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'affichage des sondages (MiyuWeb) ; l'implÃ©mentation dÃ©taillÃ©e (stockage, anonymat).

---

## 3. DÃ©finition canonique

> **MiyuPolls est une composition officielle d'outils de sondages (crÃ©ation, vote, rÃ©sultats), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuPolls **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuPolls **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; dÃ©cision (crÃ©ation, vote autorisÃ©) = StrongFather.

**RÃ¨gle fondamentale :** Toute Ã©criture (sondage, vote) = **WriteIntent** vers KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.content.polls` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `content` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuPolls - Reference Outils](./MiyuPolls%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.poll.create` | CrÃ©e un sondage (question, options, rÃ¨gles fournies) ; autorisation = StrongFather |
| `tool.poll.vote` | Enregistre un vote (sondage, option fournis) ; autorisation = StrongFather |
| `tool.poll.list` | Liste les sondages (filtres fournis) |
| `tool.poll.result` | RÃ©cupÃ¨re les rÃ©sultats d'un sondage (agrÃ©gÃ©s) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuPolls en contient quatre.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : **dÃ©cision (crÃ©ation, vote autorisÃ©, clÃ´ture) = StrongFather** ; toute Ã©criture = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **1 Ã  2** (donnÃ©es de vote sensibles selon politique) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es : sondages, options, votes. Toute crÃ©ation (sondage, vote) passe par **WriteIntent** vers KindMother.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuPolls - Tool Governance Compliance Contract](./contracts/governance/MiyuPolls%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuPolls sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuPolls devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Ã‰quivalents Moteur Forum | [Miyukini Conceptual References - Equivalents Moteur Forum](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


