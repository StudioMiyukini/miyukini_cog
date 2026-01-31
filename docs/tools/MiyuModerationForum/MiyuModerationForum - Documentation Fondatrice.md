# MiyuModerationForum — Documentation Fondatrice

## 1. Contexte

**MiyuModerationForum** est le **kit d'outils (Toolkit)** de modération forum (file d'attente, signalements, lock/move/merge/split, avertissements, bannissements, notes) de l'écosystème Miyukini. Il intègre les outils de file de modération, de signalement, d'actions sur topics/posts et d'avertissements/bannissements, alignés sur [Équivalents Moteur Forum](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Moteur%20Forum.md).

L'autorité sur les données (file, reports, avertissements, bannissements, notes) appartient à **KindMother**. MiyuModerationForum expose des capacités d'exécution gouvernée ; **toutes les décisions** (accepter, rejeter, lock, move, merge, split, avertir, bannir) relèvent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuModerationForum, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** la structure forum (MiyuForum) ; l'implémentation détaillée (politique de modération).

---

## 3. Définition canonique

> **MiyuModerationForum est une composition officielle d'outils de modération forum (file, reports, lock/move/merge/split, avertissements, bannissements, notes), déclarée et gouvernée par l'environnement.**

- MiyuModerationForum **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuModerationForum **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; **décision (toute action de modération) = StrongFather**.

**Règle fondamentale :** Toute action (lock, move, merge, split, avertissement, bannissement, note) = décision StrongFather + **WriteIntent** vers KindMother.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.moderation.forum` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `moderation` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuModerationForum - Reference Outils](./MiyuModerationForum%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.moderation.queue.list` | Liste la file d'attente (filtres fournis) |
| `tool.moderation.queue.get` | Récupère un élément de la file |
| `tool.moderation.report.create` | Crée un signalement ; WriteIntent KindMother |
| `tool.moderation.report.list` | Liste les signalements |
| `tool.forum.topic.lock` | Verrouille un topic ; décision = StrongFather |
| `tool.forum.topic.move` | Déplace un topic ; décision = StrongFather |
| `tool.forum.topic.merge` | Fusionne des topics ; décision = StrongFather |
| `tool.forum.topic.split` | Scinde un topic ; décision = StrongFather |
| `tool.forum.topic.delete` | Supprime un topic ; décision = StrongFather |
| `tool.forum.topic.copy` | Copie un topic ; décision = StrongFather |
| `tool.forum.post.edit` | Édite un post (modération) ; décision = StrongFather |
| `tool.forum.post.lock` | Verrouille un post ; décision = StrongFather |
| `tool.forum.post.delete` | Supprime un post ; décision = StrongFather |
| `tool.moderation.warning.create` | Crée un avertissement ; WriteIntent KindMother |
| `tool.moderation.warning.list` | Liste les avertissements |
| `tool.moderation.ban.create` | Crée un bannissement ; WriteIntent KindMother |
| `tool.moderation.ban.list` | Liste les bannissements |
| `tool.moderation.usernote.create` | Crée une note modérateur ; WriteIntent KindMother |
| `tool.moderation.usernote.list` | Liste les notes modérateur |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuModerationForum en contient dix-neuf.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : **toute décision de modération (lock, move, merge, split, avertissement, bannissement) = StrongFather** ; toute écriture = WriteIntent KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **2 à 3** (actions de modération sensibles) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données : file, reports, avertissements, bannissements, notes. Toute action de modération qui modifie des données passe par **WriteIntent** vers KindMother. Décision = StrongFather.

Les obligations de conformité détaillées sont dans [MiyuModerationForum - Tool Governance Compliance Contract](./contracts/governance/MiyuModerationForum%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuModerationForum sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Tools MiyuModerationForum devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

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
