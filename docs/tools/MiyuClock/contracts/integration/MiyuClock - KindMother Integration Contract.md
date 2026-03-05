# MiyuClock â€” KindMother Integration Contract

## 1. Contexte

Ce document dÃ©finit le contrat d'intÃ©gration entre **MiyuClock** (kit d'outils de mesure du temps) et **KindMother** (Core de donnÃ©es, Strate 4). MiyuClock **ne persiste pas** ; toute utilisation de timestamps pour la persistance (Ã©criture en base, audit, logs mÃ©tier) relÃ¨ve de l'**OpÃ©rateur** et de **KindMother/MiyuSQL**. MiyuClock fournit des valeurs de temps dans le flux gouvernÃ© ; il ne lit ni n'Ã©crit en base.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- L'absence de persistance par MiyuClock (ni lecture ni Ã©criture en base)
- Le pÃ©rimÃ¨tre minimal : MiyuClock ne persiste pas ; usage des timestamps = OpÃ©rateur + KindMother/MiyuSQL
- Les invariants INV-KM-* adaptÃ©s au kit de mesure du temps (peu nombreux)

Ce document **ne couvre pas** :
- L'implÃ©mentation interne de KindMother ou de MiyuSQL
- Les contrats MiyuClock hors intÃ©gration (gouvernance, sÃ©curitÃ©, bornage)
- Le dÃ©tail du modÃ¨le WriteIntent (voir KindMother)

---

## 3. Principe fondamental

### 3.1 MiyuClock ne persiste pas

> **MiyuClock ne lit ni n'Ã©crit en base. Toute persistance de timestamps (Ã©criture en base, audit, logs mÃ©tier) relÃ¨ve de l'OpÃ©rateur et de KindMother/MiyuSQL. MiyuClock fournit des valeurs de temps dans le flux gouvernÃ© ; l'OpÃ©rateur dÃ©cide de leur usage et KindMother mandate la persistance si besoin.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-KM-1** | MiyuClock ne lit jamais en base ; il ne dÃ©pend pas de MiyuSQL pour la mesure du temps |
| **INV-KM-2** | MiyuClock n'Ã©crit jamais en base ; il ne produit pas de WriteIntent, ne persiste pas de timestamps |
| **INV-KM-3** | Toute persistance de timestamps (ex. horodatage d'un enregistrement, audit) est exÃ©cutÃ©e par l'OpÃ©rateur via KindMother/MiyuSQL ; MiyuClock fournit la valeur dans le flux, pas la persistance |
| **INV-KM-4** | MiyuClock n'exÃ©cute que ce qui a Ã©tÃ© autorisÃ© par la gouvernance (StrongFather) ; il ne mandate pas KindMother |

---

## 4. RÃ´le des Tools MiyuClock

### 4.1 CapacitÃ©s exÃ©cutÃ©es, pas de persistance

| ToolId | RÃ´le | Persistance / KindMother |
|--------|------|---------------------------|
| `tool.time.now` | Retourne l'instant prÃ©sent (rÃ©fÃ©rence locale) dans le flux | La valeur est fournie dans le flux ; si l'OpÃ©rateur doit persister un timestamp, il le fait via KindMother/MiyuSQL ; MiyuClock ne persiste pas |
| `tool.time.delta` | Retourne la durÃ©e Ã©coulÃ©e entre deux instants fournis dans le flux | Calcul dans le flux ; aucune lecture ni Ã©criture en base ; si le rÃ©sultat doit Ãªtre persistÃ©, c'est l'OpÃ©rateur + KindMother |

### 4.2 Ce que MiyuClock ne fait jamais

| Interdiction | Description |
|-------------|-------------|
| **INTERDIT-1** | Lire des timestamps depuis la base (KindMother/MiyuSQL) pour la mesure elle-mÃªme |
| **INTERDIT-2** | Ã‰crire ou persister des timestamps en base (WriteIntent, MiyuSQL) |
| **INTERDIT-3** | Mandater KindMother ou MiyuSQL pour une opÃ©ration de persistance |
| **INTERDIT-4** | DÃ©tenir un Ã©tat persistÃ© ; MiyuClock est sans Ã©tat de persistance |

---

## 5. Flux typique : mesure puis persistance (par l'OpÃ©rateur)

```
OpÃ©rateur
    â”‚
    â”‚ 1. Demande d'utilisation d'un Tool MiyuClock (now ou delta)
    â–¼
BondingBrother â”€â”€â–º Master Butler â”€â”€â–º WorrySentinel â”€â”€â–º Caring Nanny â”€â”€â–º StrongFather
    â”‚                                                                      â”‚
    â”‚ 2. ALLOW                                                             â”‚
    â–¼                                                                      â”‚
MiyuClock : exÃ©cution (tool.time.now ou tool.time.delta)
    â”‚
    â”‚ 3. Valeur de temps retournÃ©e dans le flux (instant ou durÃ©e)
    â–¼
OpÃ©rateur reÃ§oit la valeur
    â”‚
    â”‚ 4. Si persistance nÃ©cessaire : OpÃ©rateur soumet WriteIntent / mandat Ã  KindMother
    â–¼
KindMother / MiyuSQL : persistance (horodatage, audit, etc.) â€” hors pÃ©rimÃ¨tre MiyuClock
```

MiyuClock s'arrÃªte Ã  l'Ã©tape 3. Les Ã©tapes 4 et suivantes relÃ¨vent de l'OpÃ©rateur et de KindMother ; MiyuClock n'y participe pas.

---

## 6. Absence de contournement

Aucun chemin ne peut faire persister des timestamps **par MiyuClock** :
1. MiyuClock ne produit pas de WriteIntent
2. MiyuClock n'appelle pas MiyuSQL
3. MiyuClock ne lit pas en base pour obtenir un instant (il utilise le Kernel Clock pour `tool.time.now` et les entrÃ©es du flux pour `tool.time.delta`)

La persistance des timestamps est toujours le fait de l'OpÃ©rateur et de KindMother/MiyuSQL.

---

## 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuClock - Documentation Fondatrice | [MiyuClock - Documentation Fondatrice](../../MiyuClock%20-%20Documentation%20Fondatrice.md) |
| MiyuClock - Runtime Boundary Contract | [MiyuClock - Runtime Boundary Contract](../boundaries/MiyuClock%20-%20Runtime%20Boundary%20Contract.md) |
| KindMother - Index | [KindMother - Index](..//..//..//..//cores//KindMother//_index.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |
| Lois Autonomie (LOI-4) | [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence


