# MiyuClock â€” Runtime Boundary Contract

## 1. Contexte

Ce document dÃ©finit le **bornage (frontiÃ¨res d'exÃ©cution)** du kit MiyuClock. Il Ã©tablit ce que MiyuClock ne fait jamais, les frontiÃ¨res avec le Kernel (Clock) et les Cores, et les invariants de limite. MiyuClock est un Kit d'Outils qui orchestre des capacitÃ©s atomiques de mesure du temps (instant prÃ©sent, delta entre instants) sans dÃ©cision mÃ©tier, sans persistance et sans temps global (conformitÃ© **LOI-4**).

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- Ce que MiyuClock ne fait jamais (pas de persistance, pas de dÃ©cision mÃ©tier, pas de timezone imposÃ©e, pas de dÃ©pendance temps global, pas de capacitÃ© hors Tools composants)
- Les frontiÃ¨res avec le Kernel (Clock) et les Cores (Master Butler, WorrySentinel, Caring Nanny, StrongFather, BondingBrother)
- Les invariants de limite (bornage)

Ce document **ne couvre pas** :
- L'implÃ©mentation technique du trait Clock du Kernel
- La persistance des timestamps (voir MiyuClock - KindMother Integration Contract)

---

## 3. Principe fondamental

### 3.1 Bornage

> **MiyuClock exÃ©cute des capacitÃ©s gouvernÃ©es de mesure du temps (now, delta). Il ne persiste jamais, ne dÃ©cide jamais de logique mÃ©tier, n'impose jamais de timezone, et ne dÃ©pend jamais d'un temps global (LOI-4).**

### 3.2 Ce que MiyuClock ne fait jamais

| Code | Interdiction |
|------|--------------|
| **BOUND-1** | **Pas de persistance** â€” MiyuClock ne lit ni n'Ã©crit en base. Toute utilisation de timestamps pour la persistance relÃ¨ve de l'OpÃ©rateur et de KindMother/MiyuSQL. |
| **BOUND-2** | **Pas de dÃ©cision mÃ©tier** â€” MiyuClock ne dÃ©cide pas si une action doit Ãªtre faite (ALLOW/DENY = StrongFather). Il exÃ©cute uniquement la mesure du temps mandatee. |
| **BOUND-3** | **Pas de timezone imposÃ©e** â€” MiyuClock ne impose aucune timezone ; l'instant fourni par `tool.time.now` est une rÃ©fÃ©rence locale. L'interprÃ©tation timezone reste Ã  l'OpÃ©rateur ou au flux. |
| **BOUND-4** | **Pas de temps global** â€” MiyuClock ne dÃ©pend d'aucun serveur de temps externe (NTP, etc.). ConformitÃ© **LOI-4** : horloge locale uniquement. |
| **BOUND-5** | **Pas de modification d'Ã©tat mÃ©tier** â€” MiyuClock ne modifie pas les permissions, ne crÃ©e pas de mandat, ne rÃ©voque rien. Il fournit des valeurs de temps dans le flux. |
| **BOUND-6** | **Pas de capacitÃ© nouvelle** â€” MiyuClock n'ajoute aucune capacitÃ© qui n'existe pas dans ses Tools composants (now, delta). Il orchestre, n'invente pas. |

---

## 4. FrontiÃ¨re avec le Kernel (Clock)

### 4.1 RÃ´le du Kernel

| FrontiÃ¨re | Description |
|-----------|-------------|
| **Source de temps** | Le Kernel fournit le trait **Clock** (trace / horodatage local). MiyuClock s'appuie sur ce trait pour fournir `tool.time.now` et les instants nÃ©cessaires Ã  `tool.time.delta`. |
| **Limite** | MiyuClock ne remplace pas le Clock du Kernel ; il expose la mesure du temps aux OpÃ©rateurs via la gouvernance. MiyuClock ne dÃ©finit pas la source d'horloge (Kernel). |

### 4.2 Invariant LOI-4

Aucune dÃ©pendance Ã  un temps global. L'horloge est locale ; le Kernel (Clock) est la seule source de temps pour MiyuClock. Aucun appel Ã  NTP, serveur de temps externe ou temps universel requis.

---

## 5. FrontiÃ¨res avec les Cores

### 5.1 StrongFather

| FrontiÃ¨re | Description |
|-----------|-------------|
| **DÃ©cision** | StrongFather dÃ©cide ALLOW ou DENY. MiyuClock n'est invoquÃ© qu'en cas d'ALLOW. |
| **Limite** | MiyuClock ne prend aucune dÃ©cision stratÃ©gique. Il n'Ã©met pas de mandat, ne rÃ©voque rien. |

### 5.2 Master Butler

| FrontiÃ¨re | Description |
|-----------|-------------|
| **Catalogue** | Master Butler dÃ©clare le Toolkit et les Tools. MiyuClock n'enregistre pas lui-mÃªme les Tools ; il est dÃ©clarÃ© par l'environnement. |
| **Limite** | MiyuClock ne gÃ¨re pas les permissions ni le catalogue. Il est invoquÃ© aprÃ¨s vÃ©rification Master Butler. |

### 5.3 WorrySentinel et Caring Nanny

| FrontiÃ¨re | Description |
|-----------|-------------|
| **SÃ©curitÃ© et Ã©tat** | WorrySentinel (niveau de sÃ©curitÃ©) et Caring Nanny (Ã©tat systÃ¨me) sont vÃ©rifiÃ©s avant l'appel Ã  MiyuClock. |
| **Limite** | MiyuClock ne modifie pas le niveau de sÃ©curitÃ© ni l'Ã©tat systÃ¨me. Il n'est invoquÃ© que si les prÃ©-conditions sont remplies. |

### 5.4 BondingBrother

| FrontiÃ¨re | Description |
|-----------|-------------|
| **MÃ©diation** | BondingBrother traduit l'intention et prÃ©pare le contexte. MiyuClock reÃ§oit une demande dÃ©jÃ  mÃ©diÃ©e. |
| **Limite** | MiyuClock ne mÃ©die pas les intentions ; il exÃ©cute la capacitÃ© (now ou delta) fournie dans le contexte gouvernÃ©. |

---

## 6. Invariants de limite

Les invariants MiyuClock utilisent des prÃ©fixes catÃ©goriels (BOUND = bornage). Pour le format canonique des invariants des Cores, voir [Miyukini Conceptual References - Standardisation Numeration Invariants](..//..//..//..//miyukini-webway-system//reference//_index.md).

| Code | Invariant |
|------|-----------|
| **INV-BOUND-1** | Aucune persistance ; MiyuClock ne lit ni n'Ã©crit en base. Toute persistance de timestamps = OpÃ©rateur + KindMother/MiyuSQL. |
| **INV-BOUND-2** | Aucune exÃ©cution sans passage par la gouvernance (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather) |
| **INV-BOUND-3** | Aucune dÃ©cision mÃ©tier dans MiyuClock ; exÃ©cution de la mesure du temps uniquement |
| **INV-BOUND-4** | Aucune dÃ©pendance Ã  un temps global (LOI-4) ; horloge locale (Kernel Clock) uniquement |
| **INV-BOUND-5** | Le Toolkit n'expose que les capacitÃ©s de ses Tools composants (now, delta) ; pas de capacitÃ© nouvelle |

---

## 7. RÃ©ponses aux violations

### 7.1 Comportement attendu

Si une condition de bornage est violÃ©e (ex. appel sans gouvernance, tentative de persistance par MiyuClock, dÃ©pendance temps global), MiyuClock ne doit pas exÃ©cuter. La rÃ©ponse (rejet, erreur explicite) est gÃ©rÃ©e par la couche gouvernance (BondingBrother / StrongFather) ou par le contrat d'intÃ©gration (KindMother pour la persistance), pas par MiyuClock lui-mÃªme.

### 7.2 TraÃ§abilitÃ©

Toute tentative d'appel hors bornage doit Ãªtre tracÃ©e (observability, audit) selon les contrats Caring Nanny et KindMother ; MiyuClock ne dÃ©cide pas du contenu du trace, il peut fournir un signal d'Ã©chec au flux gouvernÃ©.

---

## 8. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuClock - Documentation Fondatrice | [MiyuClock - Documentation Fondatrice](../../MiyuClock%20-%20Documentation%20Fondatrice.md) |
| MiyuClock - KindMother Integration Contract | [MiyuClock - KindMother Integration Contract](../integration/MiyuClock%20-%20KindMother%20Integration%20Contract.md) |
| MiyuClock - Tool Governance Compliance Contract | [MiyuClock - Tool Governance Compliance Contract](../governance/MiyuClock%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Lois Autonomie (LOI-4) | [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |
| Standardisation NumÃ©ration Invariants | [Miyukini Conceptual References - Standardisation NumÃ©ration Invariants](..//..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence

