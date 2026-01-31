# MiyuJobs — Documentation Fondatrice

## 1. Contexte

**MiyuJobs** est le **kit d'outils (Toolkit)** de planification et de file d'attente de l'écosystème Miyukini. Il intègre les outils de planification (à une date/heure ou selon expression cron), d'enfilement et de traitement de tâches asynchrones (queue), sans logique métier — la décision de planifier ou d'enfiler relève de **StrongFather** ; MiyuJobs exécute la planification et l'enfilement.

L'autorité sur les données métier (contenu des jobs, résultats) appartient à **KindMother**. MiyuJobs expose des capacités d'exécution gouvernée (planifier à, planifier cron, enfiler, traiter) ; les décisions (quoi planifier, quand, quoi enfiler) relèvent de **StrongFather** et des Opérateurs.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuJobs, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'implémentation détaillée (scheduler, broker de queue) ; la logique métier des tâches exécutées (Opérateurs / StrongFather).

---

## 3. Définition canonique

> **MiyuJobs est une composition officielle d'outils de planification et de file d'attente (schedule at, cron, enqueue, process), déclarée et gouvernée par l'environnement.**

- MiyuJobs **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuJobs **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (planifier à une date, planifier cron, enfiler une tâche, traiter une tâche) ; décision de planifier/enfiler = StrongFather ; contenu de la tâche fourni dans le flux.

**Règle fondamentale :** Un Tool MiyuJobs **exécute** la planification ou l'enfilement ; il **ne décide pas** ce qui doit être planifié ou exécuté — cela relève de StrongFather et des Opérateurs.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.jobs.miyujobs` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `jobs` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuJobs - Reference Outils](./MiyuJobs%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.jobs.schedule.at` | Planifie une exécution à une date/heure fournie ; autorisation = StrongFather |
| `tool.jobs.schedule.cron` | Planifie une exécution selon expression cron fournie ; autorisation = StrongFather |
| `tool.jobs.queue.enqueue` | Enfile une tâche (payload fourni) dans une queue ; autorisation = StrongFather |
| `tool.jobs.queue.process` | Traite une tâche (ou un lot) depuis une queue ; exécution selon handler fourni dans le flux |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuJobs en contient quatre.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : **décision de planifier/enfiler = StrongFather** ; contenu des tâches fourni dans le flux ; exécution des tâches = Opérateurs / gouvernance.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **1 à 2** (tâches peuvent contenir des données sensibles) |
| **États autorisés** | Tous sauf restriction WorrySentinel / Caring Nanny |
| **États interdits** | Selon politique (ex. blocage planification en DEGRADED) |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données métier. Les payloads des jobs peuvent être persistés (queue, historique) ; écriture = **WriteIntent** vers KindMother ou stockage gouverné. MiyuJobs n'exécute pas la logique métier des tâches : il planifie, enfile et déclenche le traitement ; le contenu et le handler sont fournis dans le flux gouverné.

Les obligations de conformité détaillées sont dans [MiyuJobs - Tool Governance Compliance Contract](./contracts/governance/MiyuJobs%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

À l'implémentation : chaque Tool MiyuJobs est une unité logique pouvant devenir un **bloc MSCM** : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

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
