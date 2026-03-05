# MiyuClock â€” Documentation Fondatrice

## 1. Contexte

**MiyuClock** est le **kit d'outils (Toolkit)** de mesure du temps de l'Ã©cosystÃ¨me Miyukini (Strate 6). Il expose des capacitÃ©s atomiques â€” instant prÃ©sent, delta entre instants â€” dans le flux gouvernÃ©, sans dÃ©cision mÃ©tier et sans temps global, en conformitÃ© avec **LOI-4** (pas de temps global requis).

Le **Kernel** fournit un trait **Clock** (trace / horodatage) ; MiyuClock est la couche Tool qui expose la mesure du temps aux OpÃ©rateurs via la gouvernance (Master Butler, WorrySentinel, Caring Nanny, StrongFather). MiyuClock **ne persiste pas** ; toute utilisation de timestamps pour la persistance relÃ¨ve de l'OpÃ©rateur et de KindMother/MiyuSQL.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :**

- L'identitÃ© et la dÃ©finition canonique de MiyuClock
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliquÃ©s)
- Le niveau de sÃ©curitÃ© et les Ã©tats systÃ¨me autorisÃ©s ou interdits
- La relation avec le Kernel (Clock) et l'absence de persistance (KindMother/MiyuSQL pour les timestamps)
- L'alignement avec le protocole MIP v1 pour une future implÃ©mentation indexable
- La conformitÃ© **LOI-4** (horloge locale uniquement, pas de dÃ©pendance Ã  un temps global)

**Hors scope :**

- L'implÃ©mentation dÃ©taillÃ©e (source d'horloge, prÃ©cision)
- Toute persistance de timestamps â€” celle-ci reste du ressort des OpÃ©rateurs et de KindMother/MiyuSQL

---

## 3. DÃ©finition canonique

> **MiyuClock est une composition officielle d'outils de mesure du temps (instant prÃ©sent, delta entre instants), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuClock **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuClock **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (obtenir l'instant prÃ©sent, calculer une durÃ©e entre deux instants fournis dans le flux) sans dÃ©cider ni persister.

**RÃ¨gle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacitÃ©. Les capacitÃ©s viennent exclusivement des Tools composants. MiyuClock ne persiste pas ; toute utilisation de timestamps pour la persistance relÃ¨ve de l'OpÃ©rateur et de KindMother/MiyuSQL.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.time.miyuclock` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `time` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

MiyuClock est composÃ© des Tools suivants (format canonique `tool.time.<action>`). Le dÃ©tail de chaque outil (action, niveau de sÃ©curitÃ©, capability_id) est dÃ©crit dans [MiyuClock - Reference Outils](./MiyuClock%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.time.now` | Retourne l'instant prÃ©sent (rÃ©fÃ©rence locale ; pas de timezone imposÃ©e) |
| `tool.time.delta` | Retourne la durÃ©e Ã©coulÃ©e entre deux instants fournis dans le flux (entrÃ©es : t_prev, t_now ou rÃ©fÃ©rences ; sortie : durÃ©e) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuClock en contient deux.

**LOI-4 :** Horloge locale uniquement ; aucune dÃ©pendance Ã  un temps global (NTP, serveur de temps externe).

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : la mesure du temps est fournie par le Kernel (Clock) ; MiyuClock ne persiste pas (trace only).

Le Toolkit MiyuClock est **dÃ©clarÃ©** dans Master Butler et **compatibilisÃ©** par Ever Buddy (cycle de vie, versions des Outils) selon le [Master Butler - Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md).

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) (schÃ©ma de flux complet).

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **0 ou 1** (dÃ©tail dans [MiyuClock - Security and States Contract](./contracts/security/MiyuClock%20-%20Security%20and%20States%20Contract.md)) ; cohÃ©rent avec WorrySentinel. Le niveau du Toolkit est au moins Ã©gal au maximum des niveaux de ses Tools composants. |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon [Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)) |

---

## 8. Relation avec le Kernel (Clock) et absence de persistance

- Le **Kernel** fournit le trait **Clock** (trace / horodatage local). MiyuClock s'appuie sur ce trait pour fournir `tool.time.now` et les instants nÃ©cessaires Ã  `tool.time.delta`. Aucune dÃ©pendance Ã  un temps global (conformitÃ© **LOI-4**).
- **MiyuClock ne persiste pas.** Toute utilisation de timestamps pour la persistance (Ã©criture en base, audit, logs mÃ©tier) relÃ¨ve de l'**OpÃ©rateur** et de **KindMother/MiyuSQL**. MiyuClock ne lit ni n'Ã©crit en base ; il fournit des valeurs de temps dans le flux gouvernÃ©.

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//miyukini-webway-system//reference//_index.md) (LOI-4).

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuClock - Tool Governance Compliance Contract](./contracts/governance/MiyuClock%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuClock sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol) :

- **Domaine** : `time` â€” cohÃ©rent avec la projection domains.json (blocs du domaine Â« time Â»).
- **Layer** : outil / toolkit (Strate 6) â€” Ã  reflÃ©ter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuClock (now, delta) est une unitÃ© logique pouvant devenir un **bloc MSCM** Ã  l'implÃ©mentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuClock devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md). La documentation ne gÃ©nÃ¨re pas les fichiers `mscm_index/*` ; elle dÃ©finit les concepts pour une indexation future.

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) |
| Lois Autonomie (LOI-4) | [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |
| Pyramide Architecture | [Miyukini Conceptual References - Pyramide Architecture Complete](..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


