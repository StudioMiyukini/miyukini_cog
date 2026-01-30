# MiyuClock — KindMother Integration Contract

## 1. Contexte

Ce document définit le contrat d'intégration entre **MiyuClock** (kit d'outils de mesure du temps) et **KindMother** (Core de données, Strate 4). MiyuClock **ne persiste pas** ; toute utilisation de timestamps pour la persistance (écriture en base, audit, logs métier) relève de l'**Opérateur** et de **KindMother/MiyuSQL**. MiyuClock fournit des valeurs de temps dans le flux gouverné ; il ne lit ni n'écrit en base.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

Ce document définit :
- L'absence de persistance par MiyuClock (ni lecture ni écriture en base)
- Le périmètre minimal : MiyuClock ne persiste pas ; usage des timestamps = Opérateur + KindMother/MiyuSQL
- Les invariants INV-KM-* adaptés au kit de mesure du temps (peu nombreux)

Ce document **ne couvre pas** :
- L'implémentation interne de KindMother ou de MiyuSQL
- Les contrats MiyuClock hors intégration (gouvernance, sécurité, bornage)
- Le détail du modèle WriteIntent (voir KindMother)

---

## 3. Principe fondamental

### 3.1 MiyuClock ne persiste pas

> **MiyuClock ne lit ni n'écrit en base. Toute persistance de timestamps (écriture en base, audit, logs métier) relève de l'Opérateur et de KindMother/MiyuSQL. MiyuClock fournit des valeurs de temps dans le flux gouverné ; l'Opérateur décide de leur usage et KindMother mandate la persistance si besoin.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-KM-1** | MiyuClock ne lit jamais en base ; il ne dépend pas de MiyuSQL pour la mesure du temps |
| **INV-KM-2** | MiyuClock n'écrit jamais en base ; il ne produit pas de WriteIntent, ne persiste pas de timestamps |
| **INV-KM-3** | Toute persistance de timestamps (ex. horodatage d'un enregistrement, audit) est exécutée par l'Opérateur via KindMother/MiyuSQL ; MiyuClock fournit la valeur dans le flux, pas la persistance |
| **INV-KM-4** | MiyuClock n'exécute que ce qui a été autorisé par la gouvernance (StrongFather) ; il ne mandate pas KindMother |

---

## 4. Rôle des Tools MiyuClock

### 4.1 Capacités exécutées, pas de persistance

| ToolId | Rôle | Persistance / KindMother |
|--------|------|---------------------------|
| `tool.time.now` | Retourne l'instant présent (référence locale) dans le flux | La valeur est fournie dans le flux ; si l'Opérateur doit persister un timestamp, il le fait via KindMother/MiyuSQL ; MiyuClock ne persiste pas |
| `tool.time.delta` | Retourne la durée écoulée entre deux instants fournis dans le flux | Calcul dans le flux ; aucune lecture ni écriture en base ; si le résultat doit être persisté, c'est l'Opérateur + KindMother |

### 4.2 Ce que MiyuClock ne fait jamais

| Interdiction | Description |
|-------------|-------------|
| **INTERDIT-1** | Lire des timestamps depuis la base (KindMother/MiyuSQL) pour la mesure elle-même |
| **INTERDIT-2** | Écrire ou persister des timestamps en base (WriteIntent, MiyuSQL) |
| **INTERDIT-3** | Mandater KindMother ou MiyuSQL pour une opération de persistance |
| **INTERDIT-4** | Détenir un état persisté ; MiyuClock est sans état de persistance |

---

## 5. Flux typique : mesure puis persistance (par l'Opérateur)

```
Opérateur
    │
    │ 1. Demande d'utilisation d'un Tool MiyuClock (now ou delta)
    ▼
BondingBrother ──► Master Butler ──► WorrySentinel ──► Caring Nanny ──► StrongFather
    │                                                                      │
    │ 2. ALLOW                                                             │
    ▼                                                                      │
MiyuClock : exécution (tool.time.now ou tool.time.delta)
    │
    │ 3. Valeur de temps retournée dans le flux (instant ou durée)
    ▼
Opérateur reçoit la valeur
    │
    │ 4. Si persistance nécessaire : Opérateur soumet WriteIntent / mandat à KindMother
    ▼
KindMother / MiyuSQL : persistance (horodatage, audit, etc.) — hors périmètre MiyuClock
```

MiyuClock s'arrête à l'étape 3. Les étapes 4 et suivantes relèvent de l'Opérateur et de KindMother ; MiyuClock n'y participe pas.

---

## 6. Absence de contournement

Aucun chemin ne peut faire persister des timestamps **par MiyuClock** :
1. MiyuClock ne produit pas de WriteIntent
2. MiyuClock n'appelle pas MiyuSQL
3. MiyuClock ne lit pas en base pour obtenir un instant (il utilise le Kernel Clock pour `tool.time.now` et les entrées du flux pour `tool.time.delta`)

La persistance des timestamps est toujours le fait de l'Opérateur et de KindMother/MiyuSQL.

---

## 7. Références croisées

| Document | Lien |
|----------|------|
| MiyuClock - Documentation Fondatrice | [MiyuClock - Documentation Fondatrice](../../MiyuClock%20-%20Documentation%20Fondatrice.md) |
| MiyuClock - Runtime Boundary Contract | [MiyuClock - Runtime Boundary Contract](../boundaries/MiyuClock%20-%20Runtime%20Boundary%20Contract.md) |
| KindMother - Index | [KindMother - Index](../../../../core/KindMother/_index.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Lois Autonomie (LOI-4) | [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de référence
