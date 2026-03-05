# MiyuJobs â€” Documentation Fondatrice

## 1. Contexte

**MiyuJobs** est le **kit d'outils (Toolkit)** de planification et de file d'attente de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de planification (Ã  une date/heure ou selon expression cron), d'enfilement et de traitement de tÃ¢ches asynchrones (queue), sans logique mÃ©tier â€” la dÃ©cision de planifier ou d'enfiler relÃ¨ve de **StrongFather** ; MiyuJobs exÃ©cute la planification et l'enfilement.

L'autoritÃ© sur les donnÃ©es mÃ©tier (contenu des jobs, rÃ©sultats) appartient Ã  **KindMother**. MiyuJobs expose des capacitÃ©s d'exÃ©cution gouvernÃ©e (planifier Ã , planifier cron, enfiler, traiter) ; les dÃ©cisions (quoi planifier, quand, quoi enfiler) relÃ¨vent de **StrongFather** et des OpÃ©rateurs.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuJobs, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'implÃ©mentation dÃ©taillÃ©e (scheduler, broker de queue) ; la logique mÃ©tier des tÃ¢ches exÃ©cutÃ©es (OpÃ©rateurs / StrongFather).

---

## 3. DÃ©finition canonique

> **MiyuJobs est une composition officielle d'outils de planification et de file d'attente (schedule at, cron, enqueue, process), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuJobs **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuJobs **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (planifier Ã  une date, planifier cron, enfiler une tÃ¢che, traiter une tÃ¢che) ; dÃ©cision de planifier/enfiler = StrongFather ; contenu de la tÃ¢che fourni dans le flux.

**RÃ¨gle fondamentale :** Un Tool MiyuJobs **exÃ©cute** la planification ou l'enfilement ; il **ne dÃ©cide pas** ce qui doit Ãªtre planifiÃ© ou exÃ©cutÃ© â€” cela relÃ¨ve de StrongFather et des OpÃ©rateurs.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.jobs.miyujobs` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `jobs` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuJobs - Reference Outils](./MiyuJobs%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.jobs.schedule.at` | Planifie une exÃ©cution Ã  une date/heure fournie ; autorisation = StrongFather |
| `tool.jobs.schedule.cron` | Planifie une exÃ©cution selon expression cron fournie ; autorisation = StrongFather |
| `tool.jobs.queue.enqueue` | Enfile une tÃ¢che (payload fourni) dans une queue ; autorisation = StrongFather |
| `tool.jobs.queue.process` | Traite une tÃ¢che (ou un lot) depuis une queue ; exÃ©cution selon handler fourni dans le flux |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuJobs en contient quatre.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : **dÃ©cision de planifier/enfiler = StrongFather** ; contenu des tÃ¢ches fourni dans le flux ; exÃ©cution des tÃ¢ches = OpÃ©rateurs / gouvernance.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **1 Ã  2** (tÃ¢ches peuvent contenir des donnÃ©es sensibles) |
| **Ã‰tats autorisÃ©s** | Tous sauf restriction WorrySentinel / Caring Nanny |
| **Ã‰tats interdits** | Selon politique (ex. blocage planification en DEGRADED) |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es mÃ©tier. Les payloads des jobs peuvent Ãªtre persistÃ©s (queue, historique) ; Ã©criture = **WriteIntent** vers KindMother ou stockage gouvernÃ©. MiyuJobs n'exÃ©cute pas la logique mÃ©tier des tÃ¢ches : il planifie, enfile et dÃ©clenche le traitement ; le contenu et le handler sont fournis dans le flux gouvernÃ©.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuJobs - Tool Governance Compliance Contract](./contracts/governance/MiyuJobs%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

Ã€ l'implÃ©mentation : chaque Tool MiyuJobs est une unitÃ© logique pouvant devenir un **bloc MSCM** : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

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


