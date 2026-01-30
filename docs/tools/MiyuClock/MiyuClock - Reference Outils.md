# MiyuClock — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuClock. Il constitue la référence technique des capacités atomiques de mesure du temps (instant présent, delta entre instants) sans décision métier ni persistance. Les Tools sont gouvernés par les Cores (Master Butler, WorrySentinel, Caring Nanny, StrongFather) ; la mesure du temps s'appuie sur le trait Clock du Kernel (conformité **LOI-4** — pas de temps global).

**Référence du kit :** [MiyuClock - Documentation Fondatrice](./MiyuClock%20-%20Documentation%20Fondatrice.md)

---

## 2. Portée / Scope

**Ce document fournit :**

- La liste exhaustive des Tools du kit MiyuClock
- Pour chaque Tool : **ToolId**, **nom lisible**, **action** (phrase courte « fait quoi »), **niveau de sécurité** typique, **capability_id** si applicable

**Hors scope :**

- L'implémentation (source d'horloge, précision)
- La persistance des timestamps (Opérateur + KindMother/MiyuSQL)

---

## 3. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité | capability_id (ex.) |
|--------|-------------|--------|------------------|----------------------|
| `tool.time.now` | Instant présent | Retourne l'instant présent (référence locale ; pas de timezone imposée). | 0 ou 1 | `time.now` |
| `tool.time.delta` | Delta entre instants | Retourne la durée écoulée entre deux instants fournis dans le flux (t_prev, t_now) ; ne décide pas. | 0 ou 1 | `time.delta` |

**Format ToolId :** `tool.<domain>.<action>` — conforme au [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md).

---

## 4. Détail par outil (résumé)

### 4.1 Instant présent

- **tool.time.now** — Retourne l'instant présent selon l'horloge locale fournie par le Kernel (Clock). Type de sortie : valeur d'instant (trace / horodatage). Aucune timezone imposée ; aucune dépendance à un temps global (LOI-4). Ne persiste pas ; la valeur est fournie dans le flux pour usage par l'Opérateur (ex. passage à KindMother/MiyuSQL pour persistance si besoin).

### 4.2 Delta entre instants

- **tool.time.delta** — Retourne la durée écoulée entre deux instants fournis en entrée dans le flux (t_prev, t_now ou références équivalentes). Entrées : deux instants (types compatibles avec la sortie de `tool.time.now` ou références). Sortie : durée (delta). Ne décide pas ; ne persiste pas. Aucune dépendance à un temps global.

---

## 5. Alignement MIP

Chaque outil listé ci-dessus est conçu pour être une **unité logique** pouvant devenir un **bloc MSCM** à l'implémentation :

- **id** : identifiant du bloc (ex. dérivé du ToolId)
- **do** : description fonctionnelle courte (ex. « retourne l'instant présent local », « retourne la durée entre deux instants »)
- **role** : rôle sémantique (ex. `time`)
- **layer** : couche (Strate 6 — outil / toolkit)

À l'implémentation, le code fournissant ces Tools devra être balisé MSCM afin d'alimenter **blocks.json**, **domains.json**, **layers.json** selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 6. Références croisées

| Document | Lien |
|----------|------|
| Documentation Fondatrice MiyuClock | [MiyuClock - Documentation Fondatrice](./MiyuClock%20-%20Documentation%20Fondatrice.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Lois Autonomie (LOI-4) | [Miyukini Conceptual References - Lois Autonomie Systeme](../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
