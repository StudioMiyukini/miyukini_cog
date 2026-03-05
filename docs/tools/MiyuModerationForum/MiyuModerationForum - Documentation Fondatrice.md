# MiyuModerationForum â€” Documentation Fondatrice

## 1. Contexte

**MiyuModerationForum** est le **kit d'outils (Toolkit)** de modÃ©ration forum (file d'attente, signalements, lock/move/merge/split, avertissements, bannissements, notes) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de file de modÃ©ration, de signalement, d'actions sur topics/posts et d'avertissements/bannissements, alignÃ©s sur [Ã‰quivalents Moteur Forum](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (file, reports, avertissements, bannissements, notes) appartient Ã  **KindMother**. MiyuModerationForum expose des capacitÃ©s d'exÃ©cution gouvernÃ©e ; **toutes les dÃ©cisions** (accepter, rejeter, lock, move, merge, split, avertir, bannir) relÃ¨vent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuModerationForum, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** la structure forum (MiyuForum) ; l'implÃ©mentation dÃ©taillÃ©e (politique de modÃ©ration).

---

## 3. DÃ©finition canonique

> **MiyuModerationForum est une composition officielle d'outils de modÃ©ration forum (file, reports, lock/move/merge/split, avertissements, bannissements, notes), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuModerationForum **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuModerationForum **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; **dÃ©cision (toute action de modÃ©ration) = StrongFather**.

**RÃ¨gle fondamentale :** Toute action (lock, move, merge, split, avertissement, bannissement, note) = dÃ©cision StrongFather + **WriteIntent** vers KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.moderation.forum` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `moderation` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuModerationForum - Reference Outils](./MiyuModerationForum%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.moderation.queue.list` | Liste la file d'attente (filtres fournis) |
| `tool.moderation.queue.get` | RÃ©cupÃ¨re un Ã©lÃ©ment de la file |
| `tool.moderation.report.create` | CrÃ©e un signalement ; WriteIntent KindMother |
| `tool.moderation.report.list` | Liste les signalements |
| `tool.forum.topic.lock` | Verrouille un topic ; dÃ©cision = StrongFather |
| `tool.forum.topic.move` | DÃ©place un topic ; dÃ©cision = StrongFather |
| `tool.forum.topic.merge` | Fusionne des topics ; dÃ©cision = StrongFather |
| `tool.forum.topic.split` | Scinde un topic ; dÃ©cision = StrongFather |
| `tool.forum.topic.delete` | Supprime un topic ; dÃ©cision = StrongFather |
| `tool.forum.topic.copy` | Copie un topic ; dÃ©cision = StrongFather |
| `tool.forum.post.edit` | Ã‰dite un post (modÃ©ration) ; dÃ©cision = StrongFather |
| `tool.forum.post.lock` | Verrouille un post ; dÃ©cision = StrongFather |
| `tool.forum.post.delete` | Supprime un post ; dÃ©cision = StrongFather |
| `tool.moderation.warning.create` | CrÃ©e un avertissement ; WriteIntent KindMother |
| `tool.moderation.warning.list` | Liste les avertissements |
| `tool.moderation.ban.create` | CrÃ©e un bannissement ; WriteIntent KindMother |
| `tool.moderation.ban.list` | Liste les bannissements |
| `tool.moderation.usernote.create` | CrÃ©e une note modÃ©rateur ; WriteIntent KindMother |
| `tool.moderation.usernote.list` | Liste les notes modÃ©rateur |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuModerationForum en contient dix-neuf.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : **toute dÃ©cision de modÃ©ration (lock, move, merge, split, avertissement, bannissement) = StrongFather** ; toute Ã©criture = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **2 Ã  3** (actions de modÃ©ration sensibles) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es : file, reports, avertissements, bannissements, notes. Toute action de modÃ©ration qui modifie des donnÃ©es passe par **WriteIntent** vers KindMother. DÃ©cision = StrongFather.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuModerationForum - Tool Governance Compliance Contract](./contracts/governance/MiyuModerationForum%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuModerationForum sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuModerationForum devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

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


