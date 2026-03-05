# MiyuSQL â€” KindMother Integration Contract

## 1. Contexte

Ce document definit le contrat d'integration entre **MiyuSQL** (kit d'outils de manipulation de donnees en base) et **KindMother** (Core de donnees, Strate 4). KindMother est l'autorite absolue sur les donnees et la persistance ; MiyuSQL expose des capacites d'execution gouvernÃ©e (requete, transaction, cache) sans remplacer KindMother.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. Portee / Scope

Ce document definit :
- L'autorite exclusive de KindMother sur les donnees
- Le passage obligatoire par WriteIntent pour toute ecriture
- L'execution des Tools MiyuSQL Â« sous autorite Â» KindMother
- L'interdiction de tout contournement

Ce document **ne couvre pas** :
- L'implementation interne de KindMother
- Les contrats MiyuSQL hors integration (gouvernance, securite, bornage)
- Le detail du cycle de vie WriteIntent (voir KindMother - Write Intent Lifecycle Contract)

---

## 3. Principe Fondamental

### 3.1 Autorite Exclusive de KindMother

> **KindMother est l'autorite absolue sur les donnees. Les Tools MiyuSQL executent des capacites gouvernÃ©es sous autorite KindMother ; ils ne decident jamais des donnees a modifier.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-KM-1** | Toute operation de donnees (lecture/ecriture/transaction) est sous autorite KindMother |
| **INV-KM-2** | Toute ecriture passe par une WriteIntent validee par KindMother |
| **INV-KM-3** | Les Tools MiyuSQL n'executent que ce qui a ete autorise par la gouvernance (StrongFather, KindMother) |
| **INV-KM-4** | Aucun acces direct a la base en dehors du flux gouvernÃ© (BondingBrother â†’ Cores â†’ KindMother) |
| **INV-KM-5** | MiyuSQL n'ajoute aucune logique metier ; il orchestre des capacites atomiques |

---

## 4. Passage Obligatoire par WriteIntent pour Ecritures

### 4.1 Definition

Une **WriteIntent** (intention d'ecriture) est une demande formelle de modification des donnees, soumise a KindMother pour validation et application. Voir [KindMother - Write Intent Lifecycle Contract](..//..//..//..//cores//KindMother//contracts//lifecycle//KindMother%20-%20Write%20Intent%20Lifecycle%20Contract.md).

### 4.2 Regle Absolue

| Regle | Description |
|-------|-------------|
| **WRITE-1** | Toute modification de donnees (INSERT, UPDATE, DELETE, DDL ciblee) DOIT etre precedee d'une WriteIntent emise par l'Operateur (ou l'adaptateur) et validee par KindMother |
| **WRITE-2** | Les Tools MiyuSQL (`tool.query.execute`, `tool.transaction.*`) n'appliquent une ecriture que si une WriteIntent a ete acceptee et que l'execution est mandatee par KindMother |
| **WRITE-3** | Aucune ecriture Â« directe Â» (bypass WriteIntent) n'est autorisee |

### 4.3 Flux Ecriture

```
Operateur / Adaptateur
        â”‚
        â”‚ 1. Emission WriteIntent (cible, contenu, contexte)
        â–¼
BondingBrother â”€â”€â–º Master Butler â”€â”€â–º WorrySentinel â”€â”€â–º Caring Nanny â”€â”€â–º StrongFather
        â”‚                                                                      â”‚
        â”‚ 2. ALLOW                                                             â”‚
        â–¼                                                                      â”‚
KindMother : validation WriteIntent, etat ACCEPTEE                             â”‚
        â”‚                                                                      â”‚
        â”‚ 3. Mandat d'execution (tool.query.execute / transaction)            â”‚
        â–¼                                                                      â”‚
MiyuSQL Tools : execution gouvernÃ©e (sous autorite KindMother)
        â”‚
        â–¼
KindMother : application effective, etat APPLIQUEE
```

---

## 5. Execution des Tools MiyuSQL Â« Sous Autorite Â» KindMother

### 5.1 Role des Tools MiyuSQL

| ToolId (ex.) | Role | Autorite |
|--------------|------|----------|
| `tool.query.execute` | Execute la requete mandatee | KindMother mandate ; MiyuSQL execute |
| `tool.transaction.begin` / `commit` / `rollback` | Gere la transaction mandatee | KindMother valide le contexte ; MiyuSQL execute |
| `tool.schema.read` | Lit les metadonnees schema | Lecture seule ; pas de WriteIntent requise |
| `tool.cache.get` / `set` / `invalidate` | Cache gouvernÃ© | Politique definie par l'environnement ; execution sous autorite |

### 5.2 Ce que MiyuSQL ne fait jamais

| Interdiction | Description |
|-------------|-------------|
| **INTERDIT-1** | Decider quelles donnees modifier (decision = StrongFather / KindMother) |
| **INTERDIT-2** | Executer une ecriture sans WriteIntent acceptee |
| **INTERDIT-3** | Contourner KindMother pour acceder a la base |
| **INTERDIT-4** | Introduire de la logique metier (choix de schema, regles metier) |

---

## 6. Absence de Contournement

### 6.1 Contournement Interdit

Aucun chemin d'acces aux donnees ne peut contourner :
1. La mediation BondingBrother (intention, contexte)
2. Le catalogue Master Butler (Tool/Toolkit, permissions)
3. Les Cores WorrySentinel et Caring Nanny (securite, etat systeme)
4. La decision StrongFather (ALLOW/DENY)
5. L'autorite KindMother (validation WriteIntent, application)

### 6.2 Lecture Seule

Les operations de **lecture seule** (SELECT, `tool.schema.read`) passent par le meme flux de gouvernance ; la WriteIntent n'est pas requise pour la lecture, mais l'acces reste sous autorite KindMother et les Cores.

---

## 7. References Croisees

| Document | Lien |
|----------|------|
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](../../MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| KindMother - Write Intent Lifecycle Contract | [KindMother - Write Intent Lifecycle Contract](..//..//..//..//cores//KindMother//contracts//lifecycle//KindMother%20-%20Write%20Intent%20Lifecycle%20Contract.md) |
| KindMother - Index | [KindMother - Index](..//..//..//_index.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |
| Intention d'Ecriture (WriteIntent) | Glossaire â€” WriteIntent |

---

**Date de creation :** 2026-01-29  
**Version :** 1.0  
**Statut :** Contrat de reference



