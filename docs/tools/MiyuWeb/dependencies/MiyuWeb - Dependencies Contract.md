# MiyuWeb â€” Dependencies Contract

## 1. Contexte

Ce document dÃ©finit le contrat des **dÃ©pendances** du kit MiyuWeb. Il Ã©tablit la liste fermÃ©e des dÃ©pendances (KindMother, Master Butler, BondingBrother, StrongFather, WorrySentinel, Caring Nanny, Ever Buddy, Kernel), la relation indirecte Ã  MiyuSQL (donnÃ©es fournies dans le flux), l'absence de dÃ©pendance mÃ©tier, et l'ordre ou les contraintes d'utilisation.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- La liste fermÃ©e des dÃ©pendances de MiyuWeb (Cores et Kernel)
- La relation indirecte Ã  MiyuSQL (donnÃ©es en flux ; MiyuWeb ne lit pas la base)
- L'absence de dÃ©pendance mÃ©tier (OpÃ©rateurs, produits, rÃ¨gles mÃ©tier)
- L'ordre et les contraintes (flux d'appel, prÃ©-conditions)
- Les invariants de dÃ©pendance (INV-DEP-*)

Ce document **ne couvre pas** :
- Les dÃ©pendances d'implÃ©mentation (moteur de rendu, sandbox JS, CSP, librairies techniques) â€” hors scope documentaire fondateur
- Les dÃ©pendances des Cores eux-mÃªmes

---

## 3. Principe fondamental

### 3.1 Liste fermÃ©e

> **MiyuWeb ne dÃ©pend que des Cores et du Kernel dÃ©finis dans ce contrat. Aucune dÃ©pendance mÃ©tier (OpÃ©rateur, produit, rÃ¨gle mÃ©tier) n'est autorisÃ©e. MiyuWeb ne lit jamais la base directement ; les donnÃ©es (templates, assets) lui sont fournies dans le flux gouvernÃ©.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-DEP-1** | MiyuWeb ne connaÃ®t pas les OpÃ©rateurs ; il reÃ§oit un contexte gouvernÃ© (BondingBrother) |
| **INV-DEP-2** | MiyuWeb ne dÃ©pend d'aucun produit ni rÃ¨gle mÃ©tier applicative |
| **INV-DEP-3** | Toute invocation de MiyuWeb passe par la mÃ©diation BondingBrother et la gouvernance (Master Butler, WorrySentinel, Caring Nanny, StrongFather) |
| **INV-DEP-4** | MiyuWeb ne lit jamais la base ; toute donnÃ©e (template, asset) est fournie dans le flux â€” aucune lecture directe, aucun contournement KindMother ou MiyuSQL |
| **INV-DEP-5** | MiyuWeb n'est invoquÃ© qu'aprÃ¨s dÃ©cision ALLOW de StrongFather |

---

## 4. Liste fermÃ©e des dÃ©pendances

### 4.1 Cores (Strate 4)

| DÃ©pendance | RÃ´le pour MiyuWeb | Contrainte |
|------------|-------------------|------------|
| **KindMother** | AutoritÃ© sur les donnÃ©es (dont templates et assets) ; les donnÃ©es consommÃ©es par MiyuWeb sont fournies dans le flux aprÃ¨s lecture/Ã©criture sous autoritÃ© KindMother (Ã©ventuellement via MiyuSQL) | MiyuWeb n'accÃ¨de jamais Ã  la base ; il reÃ§oit les donnÃ©es (contenu de template, contenu ou mÃ©tadonnÃ©es d'assets) dans le flux gouvernÃ© |
| **Master Butler** | Catalogue des Tools et Toolkits ; permissions ; dÃ©claration de MiyuWeb et des ToolIds | MiyuWeb est invoquÃ© aprÃ¨s vÃ©rification Master Butler |
| **StrongFather** | DÃ©cision ALLOW/DENY pour l'utilisation des Tools | MiyuWeb n'est invoquÃ© qu'en cas d'ALLOW |
| **WorrySentinel** | Niveau de sÃ©curitÃ© ; vÃ©rification que le niveau actuel permet l'appel | PrÃ©-condition Ã  l'invocation |
| **Caring Nanny** | Ã‰tat systÃ¨me ; vÃ©rification que l'Ã©tat (HEALTHY, DEGRADED, etc.) permet l'appel | PrÃ©-condition Ã  l'invocation |
| **Ever Buddy** | Cycle de vie et compatibilitÃ© des Outils ; validation des versions et de la composition du Toolkit | MiyuWeb est dÃ©clarÃ© et compatibilisÃ© selon le Toolkit Composition Contract |

### 4.2 Interface & MÃ©diation (Strate 5)

| DÃ©pendance | RÃ´le pour MiyuWeb | Contrainte |
|------------|-------------------|------------|
| **BondingBrother** | MÃ©diation ; traduction de l'intention ; prÃ©paration du contexte ; passage des demandes (et des donnÃ©es en flux) vers les Cores | MiyuWeb reÃ§oit les demandes et les donnÃ©es (templates, assets) via BondingBrother (ou via le flux gouvernÃ© initiÃ© par BondingBrother) |

### 4.3 Kernel (Strate K)

| DÃ©pendance | RÃ´le pour MiyuWeb | Contrainte |
|------------|-------------------|------------|
| **Kernel** | Id (identifiants), Logger (traÃ§abilitÃ©), Clock (horodatage), Config (configuration locale), Lifecycle | Usage minimal et neutre ; pas de logique mÃ©tier ; conformitÃ© aux invariants Kernel |

---

## 5. Relation indirecte Ã  MiyuSQL

MiyuWeb **ne dÃ©pend pas** de MiyuSQL (aucun appel direct). La relation est **indirecte** :

- **KindMother** est l'autoritÃ© sur les donnÃ©es (dont templates et assets). La **persistance** (lecture/Ã©criture en base) est exÃ©cutÃ©e par **MiyuSQL** sous mandat KindMother.
- Les donnÃ©es ainsi persistÃ©es ou lues par MiyuSQL (sous KindMother) peuvent Ãªtre **fournies dans le flux** Ã  MiyuWeb â€” par exemple aprÃ¨s qu'un OpÃ©rateur ou la gouvernance ait demandÃ© une lecture via MiyuSQL, le rÃ©sultat est transmis en entrÃ©e Ã  MiyuWeb pour rendu, rÃ©solution thÃ¨me, service d'asset, etc.
- MiyuWeb opÃ¨re uniquement sur des **donnÃ©es fournies dans le flux** ; il ne lit pas la base, n'appelle pas MiyuSQL, et ne contourne jamais KindMother.

**RÃ©fÃ©rence :** [MiyuWeb - Documentation Fondatrice](../MiyuWeb%20-%20Documentation%20Fondatrice.md) (sections 8 et 8bis), [MiyuWeb - KindMother Integration Contract](../contracts/integration/MiyuWeb%20-%20KindMother%20Integration%20Contract.md).

---

## 6. Ordre et contraintes

### 6.1 Flux d'invocation (ordre)

L'ordre d'implication des dÃ©pendances lors d'un appel Ã  un Tool MiyuWeb est :

1. **OpÃ©rateur** (hors dÃ©pendance MiyuWeb) Ã©met une intention (ex. afficher une page, servir un asset).
2. **BondingBrother** â€” mÃ©diation, traduction, contexte ; prÃ©paration des donnÃ©es en flux (templates, assets) si elles proviennent d'une lecture prÃ©alable sous KindMother/MiyuSQL.
3. **Master Butler** â€” vÃ©rification Tool/Toolkit, permissions.
4. **WorrySentinel** â€” niveau de sÃ©curitÃ©.
5. **Caring Nanny** â€” Ã©tat systÃ¨me.
6. **StrongFather** â€” dÃ©cision ALLOW/DENY.
7. Si ALLOW : les donnÃ©es (templates, assets) sont dÃ©jÃ  dans le flux ou fournies par le demandeur ; **KindMother** a autoritÃ© sur toute donnÃ©e persistÃ©e (MiyuWeb ne lit pas la base).
8. **MiyuWeb** â€” exÃ©cution du Tool mandatÃ© (html.render, layout.render, theme.resolve, script.execute, script.compile, asset.serve, form.validate, event.dispatch, input.capture) sur les **donnÃ©es fournies dans le flux**.

### 6.2 Contraintes

| Contrainte | Description |
|------------|-------------|
| **Pas d'invocation directe** | MiyuWeb n'est jamais invoquÃ© directement par un OpÃ©rateur ; toujours via BondingBrother et la chaÃ®ne de gouvernance |
| **Pas de bypass** | Aucune dÃ©pendance ne peut Ãªtre contournÃ©e (pas de lecture directe en base, pas d'exÃ©cution sans StrongFather ALLOW, pas de donnÃ©es sans flux gouvernÃ©) |
| **Pas de dÃ©pendance inverse mÃ©tier** | Aucun Core ni Kernel ne dÃ©pend de MiyuWeb pour sa logique mÃ©tier ; MiyuWeb est un outil consommÃ© par le flux |

---

## 7. Absence de dÃ©pendance mÃ©tier

### 7.1 Ce dont MiyuWeb ne dÃ©pend pas

| Type | Exemples | Raison |
|------|----------|--------|
| **OpÃ©rateurs** | MiyukiniAdmin, tout OpÃ©rateur de domaine | MiyuWeb est un Toolkit ; les OpÃ©rateurs utilisent MiyuWeb via la gouvernance |
| **Produits / RÃ¨gles mÃ©tier** | SchÃ©mas applicatifs, rÃ¨gles mÃ©tier, choix de contenu | MiyuWeb n'interprÃ¨te pas le mÃ©tier ; il exÃ©cute des capacitÃ©s (rendu, thÃ¨me, script, asset, formulaire, Ã©vÃ©nement) sur des donnÃ©es fournies |
| **MiyuSQL (direct)** | Appels directs Ã  MiyuSQL | MiyuWeb ne lit pas la base ; les donnÃ©es sont fournies dans le flux (Ã©ventuellement aprÃ¨s lecture par MiyuSQL sous KindMother en amont) |
| **Autres Toolkits** | Kits d'outils mÃ©tier (hors flux de donnÃ©es) | MiyuWeb est indÃ©pendant des autres Toolkits ; pas de couplage fonctionnel direct |
| **Services externes** | APIs externes, rÃ©seau mÃ©tier | ConformitÃ© LOI-1 ; pas de dÃ©pendance externe critique |

### 7.2 DÃ©pendances techniques (hors scope contractuel)

Les dÃ©pendances techniques (moteur de rendu, sandbox JS, CSP, librairies d'assets, etc.) sont hors scope de ce contrat fondateur. Elles seront dÃ©finies dans le guide d'implÃ©mentation (MiyuWeb - Reference Implementation Guidelines) et doivent rester neutres (pas de logique mÃ©tier, pas d'accÃ¨s direct Ã  la base).

---

## 8. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuWeb - Documentation Fondatrice | [MiyuWeb - Documentation Fondatrice](../MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| MiyuWeb - KindMother Integration Contract | [MiyuWeb - KindMother Integration Contract](../contracts/integration/MiyuWeb%20-%20KindMother%20Integration%20Contract.md) |
| MiyuWeb - Tool Governance Compliance Contract | [MiyuWeb - Tool Governance Compliance Contract](../contracts/governance/MiyuWeb%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| MiyuWeb - Runtime Boundary Contract | [MiyuWeb - Runtime Boundary Contract](../contracts/boundaries/MiyuWeb%20-%20Runtime%20Boundary%20Contract.md) |
| MiyuWeb - Security and States Contract | [MiyuWeb - Security and States Contract](../contracts/security/MiyuWeb%20-%20Security%20and%20States%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence

