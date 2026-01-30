# MiyuWeb — KindMother Integration Contract

## 1. Contexte

Ce document définit le contrat d'intégration entre **MiyuWeb** (kit d'outils d'affichage de contenu web) et **KindMother** (Core de données, Strate 4). KindMother est l'autorité absolue sur les données, y compris les **templates** et **assets** utilisés pour l'affichage web. MiyuSQL exécute la persistance (lecture/écriture) sous mandat KindMother. MiyuWeb **ne lit pas la base** ; il reçoit les données (templates, assets) **dans le flux gouverné** et exécute uniquement rendu, résolution thème/layout, script, service d'asset, formulaire et événements.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

Ce document définit :
- Le rôle unique de KindMother comme autorité sur les données (dont templates et assets)
- Le rôle de MiyuSQL comme exécutant de la persistance sous mandat KindMother
- L'invariant : MiyuWeb ne lit jamais la base ; il opère sur des données fournies dans le flux
- L'absence de contournement KindMother / MiyuSQL

Ce document **ne couvre pas** :
- L'implémentation interne de KindMother
- Les contrats MiyuWeb hors intégration (gouvernance, sécurité, bornage)
- Le détail du cycle de vie WriteIntent (voir KindMother - Write Intent Lifecycle Contract)

---

## 3. Principe fondamental

### 3.1 KindMother = autorité sur les données ; MiyuWeb = exécution sur données en flux

> **KindMother est l'autorité sur toutes les données, dont les templates et les assets. MiyuSQL exécute la persistance (lecture/écriture) sous mandat KindMother. MiyuWeb ne lit pas la base ; il reçoit les templates et assets dans le flux gouverné et exécute uniquement les capacités d'affichage (rendu, thème, script, asset.serve, formulaire, événements).**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-KM-1** | Aucune lecture directe de la base par MiyuWeb ; toutes les données (templates, contenu, métadonnées ou contenu d'assets) sont fournies dans le flux gouverné |
| **INV-KM-2** | MiyuWeb n'exécute que sur des données déjà présentes dans le flux — éventuellement lues via MiyuSQL sous autorité KindMother en amont, ou transmises par un Opérateur |
| **INV-KM-3** | Aucun contournement de KindMother ni de MiyuSQL pour accéder aux templates ou assets persistés |
| **INV-KM-4** | MiyuWeb n'exécute que ce qui a été autorisé par la gouvernance (StrongFather, KindMother) |
| **INV-KM-5** | MiyuWeb n'ajoute aucune logique métier ; il orchestre des capacités atomiques d'affichage web |

---

## 4. Rôle des acteurs — Templates et assets

### 4.1 Répartition des responsabilités

| Responsabilité | Acteur | Toolkit / Core | MiyuWeb |
|---------------|--------|-----------------|---------|
| **Autorité sur les données** (dont templates, assets) | KindMother | Core Strate 4 | N'intervient pas |
| **Persistance : lecture / écriture** (requêtes, transactions, cache) | KindMother mandate, **MiyuSQL** exécute | MiyuSQL | N'intervient pas |
| **Rendu, résolution thème/layout, script, asset.serve, formulaire, événements** (sans lire la base) | **MiyuWeb** exécute sur des données fournies | MiyuWeb | Exécute sur les données reçues dans le flux |

### 4.2 Ce que MiyuWeb ne fait jamais

| Interdiction | Description |
|-------------|-------------|
| **INTERDIT-1** | Lire la base (templates, assets, ou toute donnée persistée) directement |
| **INTERDIT-2** | Contourner KindMother ou MiyuSQL pour obtenir des templates ou assets |
| **INTERDIT-3** | Persister ou modifier des données en base (écriture = KindMother + MiyuSQL) |
| **INTERDIT-4** | Décider du contenu ou de la logique métier ; MiyuWeb exécute sur des données fournies |

---

## 5. Flux de données (templates et assets)

### 5.1 Flux typique : lecture puis rendu

1. Un Opérateur a besoin d'afficher du contenu web (page, formulaire, asset).
2. **KindMother** (sous gouvernance) mandate une **lecture** en base (ex. récupérer un template par identifiant, ou des assets).
3. **MiyuSQL** exécute la requête (ex. `tool.query.execute` SELECT) sous autorité KindMother et retourne les données au flux.
4. Le flux fournit le template et les données à **MiyuWeb** pour **rendu** (`tool.web.html.render`, `tool.web.layout.render`), **résolution de thème** (`tool.web.theme.resolve`), **service d'asset** (`tool.web.asset.serve`), ou autres Tools MiyuWeb.
5. MiyuWeb retourne le résultat (HTML, asset servi, etc.) **sans accéder lui-même à la base**.

### 5.2 Relation MiyuWeb ↔ MiyuSQL

MiyuWeb **ne dépend pas** de MiyuSQL (pas d'appel direct). La relation est **indirecte** via KindMother et le flux gouverné : les données persistées ou lues par MiyuSQL (sous KindMother) sont celles sur lesquelles MiyuWeb peut être invoqué **une fois qu'elles sont fournies en entrée** dans le flux.

---

## 6. Absence de contournement

Aucun chemin ne peut contourner :

1. La médiation BondingBrother (intention, contexte)
2. Le catalogue Master Butler (Tool/Toolkit, permissions)
3. Les Cores WorrySentinel et Caring Nanny (sécurité, état système)
4. La décision StrongFather (ALLOW/DENY)
5. L'autorité KindMother sur les données (templates, assets) et l'exécution de la persistance via MiyuSQL

MiyuWeb n'exécute que dans le cadre de ce flux ; il ne lit jamais la base et ne contourne jamais KindMother ni MiyuSQL.

---

## 7. Références croisées

| Document | Lien |
|----------|------|
| MiyuWeb - Documentation Fondatrice | [MiyuWeb - Documentation Fondatrice](../../MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| MiyuWeb - Security and States Contract | [MiyuWeb - Security and States Contract](../security/MiyuWeb%20-%20Security%20and%20States%20Contract.md) |
| KindMother - Index | [KindMother - Index](../../../core/KindMother/_index.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](../../MiyuSQL/MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| MiyuSQL - KindMother Integration Contract | [MiyuSQL - KindMother Integration Contract](../../MiyuSQL/contracts/integration/MiyuSQL%20-%20KindMother%20Integration%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de référence
