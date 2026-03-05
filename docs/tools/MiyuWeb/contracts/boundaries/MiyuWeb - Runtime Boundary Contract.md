# MiyuWeb â€” Runtime Boundary Contract

## 1. Contexte

Ce document dÃ©finit le **bornage (frontiÃ¨res d'exÃ©cution)** du kit MiyuWeb. Il Ã©tablit ce que MiyuWeb ne fait jamais, les frontiÃ¨res avec les Cores, et les invariants de limite. MiyuWeb est un Kit d'Outils qui orchestre des capacitÃ©s atomiques d'affichage web (rendu HTML, scripts, assets, thÃ¨me, layout, formulaires, Ã©vÃ©nements) sans dÃ©cision de contenu ni accÃ¨s direct Ã  la base.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- Ce que MiyuWeb ne fait jamais (pas de dÃ©cision ALLOW/DENY, pas de choix de contenu, pas d'accÃ¨s direct Ã  la base, pas de capacitÃ© hors Tools composants)
- Les frontiÃ¨res avec les Cores (KindMother, StrongFather, Master Butler, WorrySentinel, Caring Nanny, BondingBrother)
- Les invariants de limite (bornage)

Ce document **ne couvre pas** :
- Les frontiÃ¨res internes de KindMother (voir KindMother - Runtime Boundary & Enforcement Contract)
- L'implÃ©mentation technique des Tools

---

## 3. Principe fondamental

### 3.1 Bornage

> **MiyuWeb exÃ©cute des capacitÃ©s gouvernÃ©es d'affichage web (rendu, rÃ©solution thÃ¨me/layout, script, asset, formulaire, Ã©vÃ©nement). Il ne dÃ©cide jamais du contenu, ne prend jamais de dÃ©cision ALLOW/DENY, et n'accÃ¨de jamais Ã  la base directement â€” les templates et assets sont fournis dans le flux.**

### 3.2 Ce que MiyuWeb ne fait jamais

| Code | Interdiction |
|------|--------------|
| **BOUND-1** | **Pas de dÃ©cision ALLOW/DENY** â€” MiyuWeb ne dÃ©cide pas si une action doit Ãªtre faite (ALLOW/DENY = StrongFather). Il exÃ©cute uniquement ce qui a Ã©tÃ© autorisÃ©. |
| **BOUND-2** | **Pas de choix de contenu** â€” MiyuWeb ne dÃ©cide pas quel contenu afficher ; il rend, rÃ©sout ou sert ce qui lui est fourni dans le flux (templates, donnÃ©es, assets). Il n'interprÃ¨te pas le mÃ©tier. |
| **BOUND-3** | **Pas d'accÃ¨s direct Ã  la base** â€” MiyuWeb ne lit pas la base (templates, assets). Il reÃ§oit les donnÃ©es (contenu de template, contenu ou mÃ©tadonnÃ©es d'assets) dans le flux gouvernÃ©, aprÃ¨s lecture via MiyuSQL sous autoritÃ© KindMother ou transmission par un OpÃ©rateur. |
| **BOUND-4** | **Pas de modification du contexte d'autorisation** â€” MiyuWeb ne modifie pas les permissions, ne crÃ©e pas de mandat, ne rÃ©voque rien. Il utilise le contexte fourni. |
| **BOUND-5** | **Pas de connaissance de l'OpÃ©rateur appelant** â€” MiyuWeb ne connaÃ®t pas l'identitÃ© mÃ©tier de l'OpÃ©rateur ; il reÃ§oit un contexte gouvernÃ© (permissions, niveau, instance). |
| **BOUND-6** | **Pas de capacitÃ© nouvelle** â€” MiyuWeb n'ajoute aucune capacitÃ© qui n'existe pas dans ses Tools composants. Il orchestre, n'invente pas. |

---

## 4. FrontiÃ¨res avec les Cores

### 4.1 KindMother

| FrontiÃ¨re | Description |
|-----------|-------------|
| **AutoritÃ© sur les donnÃ©es** | KindMother est l'autoritÃ© sur toutes les donnÃ©es, dont templates et assets. MiyuWeb exÃ©cute des capacitÃ©s (rendu, rÃ©solution thÃ¨me/layout, script, asset, formulaire, Ã©vÃ©nement) sur des donnÃ©es fournies dans le flux ; il ne lit pas la base. |
| **Limite** | MiyuWeb ne persiste pas, ne lit pas les templates ni les assets en base. Il n'est alimentÃ© que par des donnÃ©es dÃ©jÃ  lues via MiyuSQL sous autoritÃ© KindMother ou transmises dans le flux gouvernÃ©. |

### 4.2 StrongFather

| FrontiÃ¨re | Description |
|-----------|-------------|
| **DÃ©cision** | StrongFather dÃ©cide ALLOW ou DENY. MiyuWeb n'est invoquÃ© qu'en cas d'ALLOW. |
| **Limite** | MiyuWeb ne prend aucune dÃ©cision stratÃ©gique. Il n'Ã©met pas de mandat, ne rÃ©voque rien, ne confÃ¨re aucune autorisation. |

### 4.3 Master Butler

| FrontiÃ¨re | Description |
|-----------|-------------|
| **Catalogue** | Master Butler dÃ©clare le Toolkit et les Tools. MiyuWeb n'enregistre pas lui-mÃªme les Tools ; il est dÃ©clarÃ© par l'environnement. |
| **Limite** | MiyuWeb ne gÃ¨re pas les permissions ni le catalogue. Il est invoquÃ© aprÃ¨s vÃ©rification Master Butler. |

### 4.4 WorrySentinel et Caring Nanny

| FrontiÃ¨re | Description |
|-----------|-------------|
| **SÃ©curitÃ© et Ã©tat** | WorrySentinel (niveau de sÃ©curitÃ©) et Caring Nanny (Ã©tat systÃ¨me) sont vÃ©rifiÃ©s avant l'appel Ã  MiyuWeb. |
| **Limite** | MiyuWeb ne modifie pas le niveau de sÃ©curitÃ© ni l'Ã©tat systÃ¨me. Il n'est invoquÃ© que si les prÃ©-conditions sont remplies. |

### 4.5 BondingBrother

| FrontiÃ¨re | Description |
|-----------|-------------|
| **MÃ©diation** | BondingBrother traduit l'intention et prÃ©pare le contexte. MiyuWeb reÃ§oit une demande dÃ©jÃ  mÃ©diÃ©e (donnÃ©es, template, asset Ã  rendre ou Ã  servir). |
| **Limite** | MiyuWeb ne mÃ©die pas les intentions ; il exÃ©cute la capacitÃ© (render, resolve, execute, serve, validate, dispatch, capture) fournie dans le contexte gouvernÃ©. |

---

## 5. Invariants de limite

Les invariants MiyuWeb utilisent des prÃ©fixes catÃ©goriels (BOUND = bornage). Pour le format canonique des invariants des Cores, voir [Miyukini Conceptual References - Standardisation NumÃ©ration Invariants](..//..//..//..//miyukini-webway-system//reference//_index.md).

| Code | Invariant |
|------|-----------|
| **INV-BOUND-1** | Aucun accÃ¨s direct Ã  la base (templates, assets) ; toutes les donnÃ©es sont fournies dans le flux gouvernÃ© |
| **INV-BOUND-2** | Aucune exÃ©cution sans passage par la gouvernance (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather) |
| **INV-BOUND-3** | Aucune dÃ©cision ALLOW/DENY ou choix de contenu dans MiyuWeb ; exÃ©cution uniquement |
| **INV-BOUND-4** | Aucune interprÃ©tation mÃ©tier du contenu ; MiyuWeb rend, rÃ©sout ou sert ce qui est fourni |
| **INV-BOUND-5** | Le Toolkit n'expose que les capacitÃ©s de ses Tools composants ; pas de capacitÃ© nouvelle |

---

## 6. RÃ©ponses aux violations

### 6.1 Comportement attendu

Si une condition de bornage est violÃ©e (ex. appel sans gouvernance, tentative d'accÃ¨s direct Ã  la base, dÃ©cision de contenu), MiyuWeb ne doit pas exÃ©cuter. La rÃ©ponse (rejet, erreur explicite) est gÃ©rÃ©e par la couche gouvernance (BondingBrother / StrongFather / KindMother), pas par MiyuWeb lui-mÃªme.

### 6.2 TraÃ§abilitÃ©

Toute tentative d'appel hors bornage doit Ãªtre tracÃ©e (observability, audit) selon les contrats KindMother et Caring Nanny ; MiyuWeb ne dÃ©cide pas du contenu du trace, il peut fournir un signal d'Ã©chec au flux gouvernÃ©.

---

## 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuWeb - Documentation Fondatrice | [MiyuWeb - Documentation Fondatrice](../../MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| MiyuWeb - KindMother Integration Contract | [MiyuWeb - KindMother Integration Contract](../integration/MiyuWeb%20-%20KindMother%20Integration%20Contract.md) |
| MiyuWeb - Tool Governance Compliance Contract | [MiyuWeb - Tool Governance Compliance Contract](../governance/MiyuWeb%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| KindMother - Runtime Boundary & Enforcement Contract | [KindMother - Runtime Boundary & Enforcement Contract](..//..//..//..//cores//KindMother//contracts//boundaries//KindMother%20-%20Runtime%20Boundary%20%26%20Enforcement%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |
| Standardisation NumÃ©ration Invariants | [Miyukini Conceptual References - Standardisation NumÃ©ration Invariants](..//..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence


