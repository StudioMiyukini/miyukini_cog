# MiyuClock â€” RÃ©fÃ©rence des outils

## 1. Contexte

Ce document dÃ©crit **chaque outil (Tool)** composant le kit MiyuClock. Il constitue la rÃ©fÃ©rence technique des capacitÃ©s atomiques de mesure du temps (instant prÃ©sent, delta entre instants) sans dÃ©cision mÃ©tier ni persistance. Les Tools sont gouvernÃ©s par les Cores (Master Butler, WorrySentinel, Caring Nanny, StrongFather) ; la mesure du temps s'appuie sur le trait Clock du Kernel (conformitÃ© **LOI-4** â€” pas de temps global).

**RÃ©fÃ©rence du kit :** [MiyuClock - Documentation Fondatrice](./MiyuClock%20-%20Documentation%20Fondatrice.md)

---

## 2. PortÃ©e / Scope

**Ce document fournit :**

- La liste exhaustive des Tools du kit MiyuClock
- Pour chaque Tool : **ToolId**, **nom lisible**, **action** (phrase courte Â« fait quoi Â»), **niveau de sÃ©curitÃ©** typique, **capability_id** si applicable

**Hors scope :**

- L'implÃ©mentation (source d'horloge, prÃ©cision)
- La persistance des timestamps (OpÃ©rateur + KindMother/MiyuSQL)

---

## 3. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sÃ©curitÃ© | capability_id (ex.) |
|--------|-------------|--------|------------------|----------------------|
| `tool.time.now` | Instant prÃ©sent | Retourne l'instant prÃ©sent (rÃ©fÃ©rence locale ; pas de timezone imposÃ©e). | 0 ou 1 | `time.now` |
| `tool.time.delta` | Delta entre instants | Retourne la durÃ©e Ã©coulÃ©e entre deux instants fournis dans le flux (t_prev, t_now) ; ne dÃ©cide pas. | 0 ou 1 | `time.delta` |

**Format ToolId :** `tool.<domain>.<action>` â€” conforme au [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md).

---

## 4. DÃ©tail par outil (rÃ©sumÃ©)

### 4.1 Instant prÃ©sent

- **tool.time.now** â€” Retourne l'instant prÃ©sent selon l'horloge locale fournie par le Kernel (Clock). Type de sortie : valeur d'instant (trace / horodatage). Aucune timezone imposÃ©e ; aucune dÃ©pendance Ã  un temps global (LOI-4). Ne persiste pas ; la valeur est fournie dans le flux pour usage par l'OpÃ©rateur (ex. passage Ã  KindMother/MiyuSQL pour persistance si besoin).

### 4.2 Delta entre instants

- **tool.time.delta** â€” Retourne la durÃ©e Ã©coulÃ©e entre deux instants fournis en entrÃ©e dans le flux (t_prev, t_now ou rÃ©fÃ©rences Ã©quivalentes). EntrÃ©es : deux instants (types compatibles avec la sortie de `tool.time.now` ou rÃ©fÃ©rences). Sortie : durÃ©e (delta). Ne dÃ©cide pas ; ne persiste pas. Aucune dÃ©pendance Ã  un temps global.

---

## 5. Alignement MIP

Chaque outil listÃ© ci-dessus est conÃ§u pour Ãªtre une **unitÃ© logique** pouvant devenir un **bloc MSCM** Ã  l'implÃ©mentation :

- **id** : identifiant du bloc (ex. dÃ©rivÃ© du ToolId)
- **do** : description fonctionnelle courte (ex. Â« retourne l'instant prÃ©sent local Â», Â« retourne la durÃ©e entre deux instants Â»)
- **role** : rÃ´le sÃ©mantique (ex. `time`)
- **layer** : couche (Strate 6 â€” outil / toolkit)

Ã€ l'implÃ©mentation, le code fournissant ces Tools devra Ãªtre balisÃ© MSCM afin d'alimenter **blocks.json**, **domains.json**, **layers.json** selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 6. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Documentation Fondatrice MiyuClock | [MiyuClock - Documentation Fondatrice](./MiyuClock%20-%20Documentation%20Fondatrice.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Lois Autonomie (LOI-4) | [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence


