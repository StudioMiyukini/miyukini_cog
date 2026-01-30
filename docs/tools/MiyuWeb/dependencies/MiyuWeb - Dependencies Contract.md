# MiyuWeb — Dependencies Contract

## 1. Contexte

Ce document définit le contrat des **dépendances** du kit MiyuWeb. Il établit la liste fermée des dépendances (KindMother, Master Butler, BondingBrother, StrongFather, WorrySentinel, Caring Nanny, Ever Buddy, Kernel), la relation indirecte à MiyuSQL (données fournies dans le flux), l'absence de dépendance métier, et l'ordre ou les contraintes d'utilisation.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

Ce document définit :
- La liste fermée des dépendances de MiyuWeb (Cores et Kernel)
- La relation indirecte à MiyuSQL (données en flux ; MiyuWeb ne lit pas la base)
- L'absence de dépendance métier (Opérateurs, produits, règles métier)
- L'ordre et les contraintes (flux d'appel, pré-conditions)
- Les invariants de dépendance (INV-DEP-*)

Ce document **ne couvre pas** :
- Les dépendances d'implémentation (moteur de rendu, sandbox JS, CSP, librairies techniques) — hors scope documentaire fondateur
- Les dépendances des Cores eux-mêmes

---

## 3. Principe fondamental

### 3.1 Liste fermée

> **MiyuWeb ne dépend que des Cores et du Kernel définis dans ce contrat. Aucune dépendance métier (Opérateur, produit, règle métier) n'est autorisée. MiyuWeb ne lit jamais la base directement ; les données (templates, assets) lui sont fournies dans le flux gouverné.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-DEP-1** | MiyuWeb ne connaît pas les Opérateurs ; il reçoit un contexte gouverné (BondingBrother) |
| **INV-DEP-2** | MiyuWeb ne dépend d'aucun produit ni règle métier applicative |
| **INV-DEP-3** | Toute invocation de MiyuWeb passe par la médiation BondingBrother et la gouvernance (Master Butler, WorrySentinel, Caring Nanny, StrongFather) |
| **INV-DEP-4** | MiyuWeb ne lit jamais la base ; toute donnée (template, asset) est fournie dans le flux — aucune lecture directe, aucun contournement KindMother ou MiyuSQL |
| **INV-DEP-5** | MiyuWeb n'est invoqué qu'après décision ALLOW de StrongFather |

---

## 4. Liste fermée des dépendances

### 4.1 Cores (Strate 4)

| Dépendance | Rôle pour MiyuWeb | Contrainte |
|------------|-------------------|------------|
| **KindMother** | Autorité sur les données (dont templates et assets) ; les données consommées par MiyuWeb sont fournies dans le flux après lecture/écriture sous autorité KindMother (éventuellement via MiyuSQL) | MiyuWeb n'accède jamais à la base ; il reçoit les données (contenu de template, contenu ou métadonnées d'assets) dans le flux gouverné |
| **Master Butler** | Catalogue des Tools et Toolkits ; permissions ; déclaration de MiyuWeb et des ToolIds | MiyuWeb est invoqué après vérification Master Butler |
| **StrongFather** | Décision ALLOW/DENY pour l'utilisation des Tools | MiyuWeb n'est invoqué qu'en cas d'ALLOW |
| **WorrySentinel** | Niveau de sécurité ; vérification que le niveau actuel permet l'appel | Pré-condition à l'invocation |
| **Caring Nanny** | État système ; vérification que l'état (HEALTHY, DEGRADED, etc.) permet l'appel | Pré-condition à l'invocation |
| **Ever Buddy** | Cycle de vie et compatibilité des Outils ; validation des versions et de la composition du Toolkit | MiyuWeb est déclaré et compatibilisé selon le Toolkit Composition Contract |

### 4.2 Interface & Médiation (Strate 5)

| Dépendance | Rôle pour MiyuWeb | Contrainte |
|------------|-------------------|------------|
| **BondingBrother** | Médiation ; traduction de l'intention ; préparation du contexte ; passage des demandes (et des données en flux) vers les Cores | MiyuWeb reçoit les demandes et les données (templates, assets) via BondingBrother (ou via le flux gouverné initié par BondingBrother) |

### 4.3 Kernel (Strate K)

| Dépendance | Rôle pour MiyuWeb | Contrainte |
|------------|-------------------|------------|
| **Kernel** | Id (identifiants), Logger (traçabilité), Clock (horodatage), Config (configuration locale), Lifecycle | Usage minimal et neutre ; pas de logique métier ; conformité aux invariants Kernel |

---

## 5. Relation indirecte à MiyuSQL

MiyuWeb **ne dépend pas** de MiyuSQL (aucun appel direct). La relation est **indirecte** :

- **KindMother** est l'autorité sur les données (dont templates et assets). La **persistance** (lecture/écriture en base) est exécutée par **MiyuSQL** sous mandat KindMother.
- Les données ainsi persistées ou lues par MiyuSQL (sous KindMother) peuvent être **fournies dans le flux** à MiyuWeb — par exemple après qu'un Opérateur ou la gouvernance ait demandé une lecture via MiyuSQL, le résultat est transmis en entrée à MiyuWeb pour rendu, résolution thème, service d'asset, etc.
- MiyuWeb opère uniquement sur des **données fournies dans le flux** ; il ne lit pas la base, n'appelle pas MiyuSQL, et ne contourne jamais KindMother.

**Référence :** [MiyuWeb - Documentation Fondatrice](../MiyuWeb%20-%20Documentation%20Fondatrice.md) (sections 8 et 8bis), [MiyuWeb - KindMother Integration Contract](../contracts/integration/MiyuWeb%20-%20KindMother%20Integration%20Contract.md).

---

## 6. Ordre et contraintes

### 6.1 Flux d'invocation (ordre)

L'ordre d'implication des dépendances lors d'un appel à un Tool MiyuWeb est :

1. **Opérateur** (hors dépendance MiyuWeb) émet une intention (ex. afficher une page, servir un asset).
2. **BondingBrother** — médiation, traduction, contexte ; préparation des données en flux (templates, assets) si elles proviennent d'une lecture préalable sous KindMother/MiyuSQL.
3. **Master Butler** — vérification Tool/Toolkit, permissions.
4. **WorrySentinel** — niveau de sécurité.
5. **Caring Nanny** — état système.
6. **StrongFather** — décision ALLOW/DENY.
7. Si ALLOW : les données (templates, assets) sont déjà dans le flux ou fournies par le demandeur ; **KindMother** a autorité sur toute donnée persistée (MiyuWeb ne lit pas la base).
8. **MiyuWeb** — exécution du Tool mandaté (html.render, layout.render, theme.resolve, script.execute, script.compile, asset.serve, form.validate, event.dispatch, input.capture) sur les **données fournies dans le flux**.

### 6.2 Contraintes

| Contrainte | Description |
|------------|-------------|
| **Pas d'invocation directe** | MiyuWeb n'est jamais invoqué directement par un Opérateur ; toujours via BondingBrother et la chaîne de gouvernance |
| **Pas de bypass** | Aucune dépendance ne peut être contournée (pas de lecture directe en base, pas d'exécution sans StrongFather ALLOW, pas de données sans flux gouverné) |
| **Pas de dépendance inverse métier** | Aucun Core ni Kernel ne dépend de MiyuWeb pour sa logique métier ; MiyuWeb est un outil consommé par le flux |

---

## 7. Absence de dépendance métier

### 7.1 Ce dont MiyuWeb ne dépend pas

| Type | Exemples | Raison |
|------|----------|--------|
| **Opérateurs** | MiyukiniAdmin, tout Opérateur de domaine | MiyuWeb est un Toolkit ; les Opérateurs utilisent MiyuWeb via la gouvernance |
| **Produits / Règles métier** | Schémas applicatifs, règles métier, choix de contenu | MiyuWeb n'interprète pas le métier ; il exécute des capacités (rendu, thème, script, asset, formulaire, événement) sur des données fournies |
| **MiyuSQL (direct)** | Appels directs à MiyuSQL | MiyuWeb ne lit pas la base ; les données sont fournies dans le flux (éventuellement après lecture par MiyuSQL sous KindMother en amont) |
| **Autres Toolkits** | Kits d'outils métier (hors flux de données) | MiyuWeb est indépendant des autres Toolkits ; pas de couplage fonctionnel direct |
| **Services externes** | APIs externes, réseau métier | Conformité LOI-1 ; pas de dépendance externe critique |

### 7.2 Dépendances techniques (hors scope contractuel)

Les dépendances techniques (moteur de rendu, sandbox JS, CSP, librairies d'assets, etc.) sont hors scope de ce contrat fondateur. Elles seront définies dans le guide d'implémentation (MiyuWeb - Reference Implementation Guidelines) et doivent rester neutres (pas de logique métier, pas d'accès direct à la base).

---

## 8. Références croisées

| Document | Lien |
|----------|------|
| MiyuWeb - Documentation Fondatrice | [MiyuWeb - Documentation Fondatrice](../MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| MiyuWeb - KindMother Integration Contract | [MiyuWeb - KindMother Integration Contract](../contracts/integration/MiyuWeb%20-%20KindMother%20Integration%20Contract.md) |
| MiyuWeb - Tool Governance Compliance Contract | [MiyuWeb - Tool Governance Compliance Contract](../contracts/governance/MiyuWeb%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| MiyuWeb - Runtime Boundary Contract | [MiyuWeb - Runtime Boundary Contract](../contracts/boundaries/MiyuWeb%20-%20Runtime%20Boundary%20Contract.md) |
| MiyuWeb - Security and States Contract | [MiyuWeb - Security and States Contract](../contracts/security/MiyuWeb%20-%20Security%20and%20States%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de référence
