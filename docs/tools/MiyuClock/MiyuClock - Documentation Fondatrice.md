# MiyuClock — Documentation Fondatrice

## 1. Contexte

**MiyuClock** est le **kit d'outils (Toolkit)** de mesure du temps de l'écosystème Miyukini (Strate 6). Il expose des capacités atomiques — instant présent, delta entre instants — dans le flux gouverné, sans décision métier et sans temps global, en conformité avec **LOI-4** (pas de temps global requis).

Le **Kernel** fournit un trait **Clock** (trace / horodatage) ; MiyuClock est la couche Tool qui expose la mesure du temps aux Opérateurs via la gouvernance (Master Butler, WorrySentinel, Caring Nanny, StrongFather). MiyuClock **ne persiste pas** ; toute utilisation de timestamps pour la persistance relève de l'Opérateur et de KindMother/MiyuSQL.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :**

- L'identité et la définition canonique de MiyuClock
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliqués)
- Le niveau de sécurité et les états système autorisés ou interdits
- La relation avec le Kernel (Clock) et l'absence de persistance (KindMother/MiyuSQL pour les timestamps)
- L'alignement avec le protocole MIP v1 pour une future implémentation indexable
- La conformité **LOI-4** (horloge locale uniquement, pas de dépendance à un temps global)

**Hors scope :**

- L'implémentation détaillée (source d'horloge, précision)
- Toute persistance de timestamps — celle-ci reste du ressort des Opérateurs et de KindMother/MiyuSQL

---

## 3. Définition canonique

> **MiyuClock est une composition officielle d'outils de mesure du temps (instant présent, delta entre instants), déclarée et gouvernée par l'environnement.**

- MiyuClock **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuClock **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (obtenir l'instant présent, calculer une durée entre deux instants fournis dans le flux) sans décider ni persister.

**Règle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacité. Les capacités viennent exclusivement des Tools composants. MiyuClock ne persiste pas ; toute utilisation de timestamps pour la persistance relève de l'Opérateur et de KindMother/MiyuSQL.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.time.miyuclock` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `time` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

MiyuClock est composé des Tools suivants (format canonique `tool.time.<action>`). Le détail de chaque outil (action, niveau de sécurité, capability_id) est décrit dans [MiyuClock - Reference Outils](./MiyuClock%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.time.now` | Retourne l'instant présent (référence locale ; pas de timezone imposée) |
| `tool.time.delta` | Retourne la durée écoulée entre deux instants fournis dans le flux (entrées : t_prev, t_now ou références ; sortie : durée) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuClock en contient deux.

**LOI-4 :** Horloge locale uniquement ; aucune dépendance à un temps global (NTP, serveur de temps externe).

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : la mesure du temps est fournie par le Kernel (Clock) ; MiyuClock ne persiste pas (trace only).

Le Toolkit MiyuClock est **déclaré** dans Master Butler et **compatibilisé** par Ever Buddy (cycle de vie, versions des Outils) selon le [Master Butler - Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md).

**Référence :** [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) (schéma de flux complet).

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 ou 1** (détail dans [MiyuClock - Security and States Contract](./contracts/security/MiyuClock%20-%20Security%20and%20States%20Contract.md)) ; cohérent avec WorrySentinel. Le niveau du Toolkit est au moins égal au maximum des niveaux de ses Tools composants. |
| **États autorisés** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon [Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)) |

---

## 8. Relation avec le Kernel (Clock) et absence de persistance

- Le **Kernel** fournit le trait **Clock** (trace / horodatage local). MiyuClock s'appuie sur ce trait pour fournir `tool.time.now` et les instants nécessaires à `tool.time.delta`. Aucune dépendance à un temps global (conformité **LOI-4**).
- **MiyuClock ne persiste pas.** Toute utilisation de timestamps pour la persistance (écriture en base, audit, logs métier) relève de l'**Opérateur** et de **KindMother/MiyuSQL**. MiyuClock ne lit ni n'écrit en base ; il fournit des valeurs de temps dans le flux gouverné.

**Référence :** [Miyukini Conceptual References - Lois Autonomie Systeme](../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) (LOI-4).

Les obligations de conformité détaillées sont dans [MiyuClock - Tool Governance Compliance Contract](./contracts/governance/MiyuClock%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuClock sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol) :

- **Domaine** : `time` — cohérent avec la projection domains.json (blocs du domaine « time »).
- **Layer** : outil / toolkit (Strate 6) — à refléter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuClock (now, delta) est une unité logique pouvant devenir un **bloc MSCM** à l'implémentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

À l'implémentation, le code fournissant les Tools MiyuClock devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md). La documentation ne génère pas les fichiers `mscm_index/*` ; elle définit les concepts pour une indexation future.

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Lois Autonomie (LOI-4) | [Miyukini Conceptual References - Lois Autonomie Systeme](../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |
| Pyramide Architecture | [Miyukini Conceptual References - Pyramide Architecture Complete](../../reference/Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
