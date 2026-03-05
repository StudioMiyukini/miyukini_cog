# MiyuWeb â€” KindMother Integration Contract

## 1. Contexte

Ce document dÃ©finit le contrat d'intÃ©gration entre **MiyuWeb** (kit d'outils d'affichage de contenu web) et **KindMother** (Core de donnÃ©es, Strate 4). KindMother est l'autoritÃ© absolue sur les donnÃ©es, y compris les **templates** et **assets** utilisÃ©s pour l'affichage web. MiyuSQL exÃ©cute la persistance (lecture/Ã©criture) sous mandat KindMother. MiyuWeb **ne lit pas la base** ; il reÃ§oit les donnÃ©es (templates, assets) **dans le flux gouvernÃ©** et exÃ©cute uniquement rendu, rÃ©solution thÃ¨me/layout, script, service d'asset, formulaire et Ã©vÃ©nements.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- Le rÃ´le unique de KindMother comme autoritÃ© sur les donnÃ©es (dont templates et assets)
- Le rÃ´le de MiyuSQL comme exÃ©cutant de la persistance sous mandat KindMother
- L'invariant : MiyuWeb ne lit jamais la base ; il opÃ¨re sur des donnÃ©es fournies dans le flux
- L'absence de contournement KindMother / MiyuSQL

Ce document **ne couvre pas** :
- L'implÃ©mentation interne de KindMother
- Les contrats MiyuWeb hors intÃ©gration (gouvernance, sÃ©curitÃ©, bornage)
- Le dÃ©tail du cycle de vie WriteIntent (voir KindMother - Write Intent Lifecycle Contract)

---

## 3. Principe fondamental

### 3.1 KindMother = autoritÃ© sur les donnÃ©es ; MiyuWeb = exÃ©cution sur donnÃ©es en flux

> **KindMother est l'autoritÃ© sur toutes les donnÃ©es, dont les templates et les assets. MiyuSQL exÃ©cute la persistance (lecture/Ã©criture) sous mandat KindMother. MiyuWeb ne lit pas la base ; il reÃ§oit les templates et assets dans le flux gouvernÃ© et exÃ©cute uniquement les capacitÃ©s d'affichage (rendu, thÃ¨me, script, asset.serve, formulaire, Ã©vÃ©nements).**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-KM-1** | Aucune lecture directe de la base par MiyuWeb ; toutes les donnÃ©es (templates, contenu, mÃ©tadonnÃ©es ou contenu d'assets) sont fournies dans le flux gouvernÃ© |
| **INV-KM-2** | MiyuWeb n'exÃ©cute que sur des donnÃ©es dÃ©jÃ  prÃ©sentes dans le flux â€” Ã©ventuellement lues via MiyuSQL sous autoritÃ© KindMother en amont, ou transmises par un OpÃ©rateur |
| **INV-KM-3** | Aucun contournement de KindMother ni de MiyuSQL pour accÃ©der aux templates ou assets persistÃ©s |
| **INV-KM-4** | MiyuWeb n'exÃ©cute que ce qui a Ã©tÃ© autorisÃ© par la gouvernance (StrongFather, KindMother) |
| **INV-KM-5** | MiyuWeb n'ajoute aucune logique mÃ©tier ; il orchestre des capacitÃ©s atomiques d'affichage web |

---

## 4. RÃ´le des acteurs â€” Templates et assets

### 4.1 RÃ©partition des responsabilitÃ©s

| ResponsabilitÃ© | Acteur | Toolkit / Core | MiyuWeb |
|---------------|--------|-----------------|---------|
| **AutoritÃ© sur les donnÃ©es** (dont templates, assets) | KindMother | Core Strate 4 | N'intervient pas |
| **Persistance : lecture / Ã©criture** (requÃªtes, transactions, cache) | KindMother mandate, **MiyuSQL** exÃ©cute | MiyuSQL | N'intervient pas |
| **Rendu, rÃ©solution thÃ¨me/layout, script, asset.serve, formulaire, Ã©vÃ©nements** (sans lire la base) | **MiyuWeb** exÃ©cute sur des donnÃ©es fournies | MiyuWeb | ExÃ©cute sur les donnÃ©es reÃ§ues dans le flux |

### 4.2 Ce que MiyuWeb ne fait jamais

| Interdiction | Description |
|-------------|-------------|
| **INTERDIT-1** | Lire la base (templates, assets, ou toute donnÃ©e persistÃ©e) directement |
| **INTERDIT-2** | Contourner KindMother ou MiyuSQL pour obtenir des templates ou assets |
| **INTERDIT-3** | Persister ou modifier des donnÃ©es en base (Ã©criture = KindMother + MiyuSQL) |
| **INTERDIT-4** | DÃ©cider du contenu ou de la logique mÃ©tier ; MiyuWeb exÃ©cute sur des donnÃ©es fournies |

---

## 5. Flux de donnÃ©es (templates et assets)

### 5.1 Flux typique : lecture puis rendu

1. Un OpÃ©rateur a besoin d'afficher du contenu web (page, formulaire, asset).
2. **KindMother** (sous gouvernance) mandate une **lecture** en base (ex. rÃ©cupÃ©rer un template par identifiant, ou des assets).
3. **MiyuSQL** exÃ©cute la requÃªte (ex. `tool.query.execute` SELECT) sous autoritÃ© KindMother et retourne les donnÃ©es au flux.
4. Le flux fournit le template et les donnÃ©es Ã  **MiyuWeb** pour **rendu** (`tool.web.html.render`, `tool.web.layout.render`), **rÃ©solution de thÃ¨me** (`tool.web.theme.resolve`), **service d'asset** (`tool.web.asset.serve`), ou autres Tools MiyuWeb.
5. MiyuWeb retourne le rÃ©sultat (HTML, asset servi, etc.) **sans accÃ©der lui-mÃªme Ã  la base**.

### 5.2 Relation MiyuWeb â†” MiyuSQL

MiyuWeb **ne dÃ©pend pas** de MiyuSQL (pas d'appel direct). La relation est **indirecte** via KindMother et le flux gouvernÃ© : les donnÃ©es persistÃ©es ou lues par MiyuSQL (sous KindMother) sont celles sur lesquelles MiyuWeb peut Ãªtre invoquÃ© **une fois qu'elles sont fournies en entrÃ©e** dans le flux.

---

## 6. Absence de contournement

Aucun chemin ne peut contourner :

1. La mÃ©diation BondingBrother (intention, contexte)
2. Le catalogue Master Butler (Tool/Toolkit, permissions)
3. Les Cores WorrySentinel et Caring Nanny (sÃ©curitÃ©, Ã©tat systÃ¨me)
4. La dÃ©cision StrongFather (ALLOW/DENY)
5. L'autoritÃ© KindMother sur les donnÃ©es (templates, assets) et l'exÃ©cution de la persistance via MiyuSQL

MiyuWeb n'exÃ©cute que dans le cadre de ce flux ; il ne lit jamais la base et ne contourne jamais KindMother ni MiyuSQL.

---

## 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuWeb - Documentation Fondatrice | [MiyuWeb - Documentation Fondatrice](../../MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| MiyuWeb - Security and States Contract | [MiyuWeb - Security and States Contract](../security/MiyuWeb%20-%20Security%20and%20States%20Contract.md) |
| KindMother - Index | [KindMother - Index](..//..//..//_index.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](..//..//..//MiyuSQL//MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| MiyuSQL - KindMother Integration Contract | [MiyuSQL - KindMother Integration Contract](..//..//..//MiyuSQL//contracts//integration//MiyuSQL%20-%20KindMother%20Integration%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence



