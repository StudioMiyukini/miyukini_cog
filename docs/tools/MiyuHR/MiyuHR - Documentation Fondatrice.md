# MiyuHR â€” Documentation Fondatrice

## 1. Contexte

**MiyuHR** est le **kit d'outils (Toolkit)** RH (pointeuse, plannings) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils d'enregistrement entrÃ©e/sortie (time clock) et de lecture du planning (shifts) pour un employÃ©/pÃ©riode, alignÃ©s sur le document [Ã‰quivalents PoS Logiciel Caisse](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (heures, plannings) appartient Ã  **KindMother** (Core de donnÃ©es, Strate 4). Les permissions (qui peut pointeuse, qui peut lire les plannings) relÃ¨vent de **Master Butler** et **StrongFather**. MiyuHR expose des capacitÃ©s d'exÃ©cution gouvernÃ©e ; les OpÃ©rateurs (ex. OpÃ©rateur RH) passent par la gouvernance pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuHR, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother, l'alignement MIP.

**Hors scope :** l'implÃ©mentation dÃ©taillÃ©e ; la gestion des droits employÃ©s (Master Butler, StrongFather).

---

## 3. DÃ©finition canonique

> **MiyuHR est une composition officielle d'outils RH (pointeuse entrÃ©e/sortie, lecture planning), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuHR **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuHR **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (enregistrement entrÃ©e/sortie, lecture planning) ; les permissions = Master Butler + StrongFather.

**RÃ¨gle fondamentale :** Toute Ã©criture (time_clock.in, time_clock.out) = WriteIntent vers KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.hr.miyuhr` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `hr` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuHR - Reference Outils](./MiyuHR%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.hr.time_clock.in` | Enregistre une entrÃ©e (dÃ©but de shift) |
| `tool.hr.time_clock.out` | Enregistre une sortie (fin de shift) |
| `tool.hr.schedule.get` | Retourne le planning (shifts) pour un employÃ©/pÃ©riode |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuHR en contient trois.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : permissions employÃ© = Master Butler + StrongFather ; toute Ã©criture = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **1 Ã  2** (dÃ©tail par outil dans Reference Outils) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es : heures (entrÃ©es/sorties), plannings. Toute Ã©criture (time_clock.in, time_clock.out) passe par **WriteIntent** sous autoritÃ© KindMother. MiyuHR exÃ©cute des capacitÃ©s atomiques ; les permissions (qui peut pointeuse) = Master Butler + StrongFather.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuHR - Tool Governance Compliance Contract](./contracts/governance/MiyuHR%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuHR sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuHR devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Ã‰quivalents PoS Logiciel Caisse | [Miyukini Conceptual References - Equivalents PoS Logiciel Caisse](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


