# MiyuAuth â€” Runtime Boundary Contract

## 1. Contexte

Ce document dÃ©finit le **bornage (frontiÃ¨res d'exÃ©cution)** du kit MiyuAuth. Il Ã©tablit ce que MiyuAuth ne fait jamais, les frontiÃ¨res avec les Cores, et les invariants de limite. MiyuAuth est un Kit d'Outils qui orchestre des capacitÃ©s atomiques d'identitÃ© sans dÃ©cision de confiance ni d'autorisation.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- Ce que MiyuAuth ne fait jamais (pas de dÃ©cision ALLOW/DENY, pas d'autorisation mÃ©tier, pas de confiance sans validation KindMother, pas de capacitÃ© hors Tools composants)
- Les frontiÃ¨res avec les Cores (KindMother, StrongFather, Master Butler, WorrySentinel, Caring Nanny, BondingBrother)
- Les invariants de limite (bornage)

Ce document **ne couvre pas** :
- Les frontiÃ¨res internes de KindMother (voir KindMother - Identity & Cross-Domain Trust Contract)
- L'implÃ©mentation technique des Tools

---

## 3. Principe fondamental

### 3.1 Bornage

> **MiyuAuth exÃ©cute des capacitÃ©s gouvernÃ©es d'identitÃ© (rÃ©solution, attestation, vÃ©rification, rÃ´le). Il ne dÃ©cide jamais de la confiance, ne prend jamais de dÃ©cision ALLOW/DENY, et n'utilise jamais de confiance non validÃ©e par KindMother.**

### 3.2 Ce que MiyuAuth ne fait jamais

| Code | Interdiction |
|------|--------------|
| **BOUND-1** | **Pas de dÃ©cision ALLOW/DENY** â€” MiyuAuth ne dÃ©cide pas si une action doit Ãªtre faite (ALLOW/DENY = StrongFather). Il exÃ©cute uniquement ce qui a Ã©tÃ© autorisÃ©. |
| **BOUND-2** | **Pas d'autorisation mÃ©tier** â€” MiyuAuth ne confÃ¨re aucune autorisation ; l'autorisation reste Ã  StrongFather et au COG HÃ©bergeur. Il ne traite pas la reconnaissance d'identitÃ© comme une autorisation. |
| **BOUND-3** | **Pas de confiance sans validation KindMother** â€” MiyuAuth n'utilise aucune confiance inter-domaines non validÃ©e par KindMother. Toute confiance utilisÃ©e pour l'identitÃ© est validÃ©e par KindMother (Identity & Cross-Domain Trust). |
| **BOUND-4** | **Pas de modification du contexte d'autorisation** â€” MiyuAuth ne modifie pas les permissions, ne crÃ©e pas de mandat, ne rÃ©voque rien. Il utilise le contexte fourni. |
| **BOUND-5** | **Pas de connaissance de l'OpÃ©rateur appelant** â€” MiyuAuth ne connaÃ®t pas l'identitÃ© mÃ©tier de l'OpÃ©rateur ; il reÃ§oit un contexte gouvernÃ© (permissions, niveau, instance). |
| **BOUND-6** | **Pas de capacitÃ© nouvelle** â€” MiyuAuth n'ajoute aucune capacitÃ© qui n'existe pas dans ses Tools composants. Il orchestre, n'invente pas. |

---

## 4. FrontiÃ¨res avec les Cores

### 4.1 KindMother

| FrontiÃ¨re | Description |
|-----------|-------------|
| **Validation de confiance** | KindMother est l'unique validateur de la confiance inter-domaines. MiyuAuth exÃ©cute des capacitÃ©s (resolve, attest, verify, role) sans dÃ©cider de la confiance ; toute confiance utilisÃ©e pour l'identitÃ© est validÃ©e par KindMother. |
| **Limite** | MiyuAuth ne valide pas la confiance, ne dÃ©lÃ¨gue pas la validation. Il exÃ©cute les capacitÃ©s mandatees aprÃ¨s validation KindMother. |

### 4.2 StrongFather

| FrontiÃ¨re | Description |
|-----------|-------------|
| **DÃ©cision** | StrongFather dÃ©cide ALLOW ou DENY. MiyuAuth n'est invoquÃ© qu'en cas d'ALLOW. |
| **Limite** | MiyuAuth ne prend aucune dÃ©cision stratÃ©gique. Il n'Ã©met pas de mandat, ne rÃ©voque rien, ne confÃ¨re aucune autorisation. |

### 4.3 Master Butler

| FrontiÃ¨re | Description |
|-----------|-------------|
| **Catalogue** | Master Butler dÃ©clare le Toolkit et les Tools. MiyuAuth n'enregistre pas lui-mÃªme les Tools ; il est dÃ©clarÃ© par l'environnement. |
| **Limite** | MiyuAuth ne gÃ¨re pas les permissions ni le catalogue. Il est invoquÃ© aprÃ¨s vÃ©rification Master Butler. |

### 4.4 WorrySentinel et Caring Nanny

| FrontiÃ¨re | Description |
|-----------|-------------|
| **SÃ©curitÃ© et Ã©tat** | WorrySentinel (niveau de sÃ©curitÃ©) et Caring Nanny (Ã©tat systÃ¨me) sont vÃ©rifiÃ©s avant l'appel Ã  MiyuAuth. |
| **Limite** | MiyuAuth ne modifie pas le niveau de sÃ©curitÃ© ni l'Ã©tat systÃ¨me. Il n'est invoquÃ© que si les prÃ©-conditions sont remplies. |

### 4.5 BondingBrother

| FrontiÃ¨re | Description |
|-----------|-------------|
| **MÃ©diation** | BondingBrother traduit l'intention et prÃ©pare le contexte. MiyuAuth reÃ§oit une demande dÃ©jÃ  mÃ©diÃ©e. |
| **Limite** | MiyuAuth ne mÃ©die pas les intentions ; il exÃ©cute la capacitÃ© (resolve, attest, verify, role) fournie dans le contexte gouvernÃ©. |

---

## 5. Invariants de limite

Les invariants MiyuAuth utilisent des prÃ©fixes catÃ©goriels (BOUND = bornage). Pour le format canonique des invariants des Cores, voir [Miyukini Conceptual References - Standardisation NumÃ©ration Invariants](..//..//..//..//miyukini-webway-system//reference//_index.md).

| Code | Invariant |
|------|-----------|
| **INV-BOUND-1** | Aucune utilisation de confiance inter-domaines sans validation KindMother |
| **INV-BOUND-2** | Aucune exÃ©cution sans passage par la gouvernance (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather) |
| **INV-BOUND-3** | Aucune dÃ©cision ALLOW/DENY ou autorisation mÃ©tier dans MiyuAuth ; exÃ©cution uniquement |
| **INV-BOUND-4** | Aucune reconnaissance d'identitÃ© traitÃ©e comme autorisation ; identitÃ© â‰  autorisation |
| **INV-BOUND-5** | Le Toolkit n'expose que les capacitÃ©s de ses Tools composants ; pas de capacitÃ© nouvelle |

---

## 6. RÃ©ponses aux violations

### 6.1 Comportement attendu

Si une condition de bornage est violÃ©e (ex. appel sans gouvernance, utilisation de confiance non validÃ©e par KindMother), MiyuAuth ne doit pas exÃ©cuter. La rÃ©ponse (rejet, erreur explicite) est gÃ©rÃ©e par la couche gouvernance (BondingBrother / StrongFather / KindMother), pas par MiyuAuth lui-mÃªme.

### 6.2 TraÃ§abilitÃ©

Toute tentative d'appel hors bornage doit Ãªtre tracÃ©e (observability, audit) selon les contrats KindMother et Caring Nanny ; MiyuAuth ne dÃ©cide pas du contenu du trace, il peut fournir un signal d'Ã©chec au flux gouvernÃ©.

---

## 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuAuth - Documentation Fondatrice | [MiyuAuth - Documentation Fondatrice](../../MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| MiyuAuth - KindMother Integration Contract | [MiyuAuth - KindMother Integration Contract](../integration/MiyuAuth%20-%20KindMother%20Integration%20Contract.md) |
| MiyuAuth - Tool Governance Compliance Contract | [MiyuAuth - Tool Governance Compliance Contract](../governance/MiyuAuth%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| KindMother - Identity & Cross-Domain Trust Contract | [KindMother - Identity & Cross-Domain Trust Contract](..//..//..//..//cores//KindMother//contracts//authority//KindMother%20-%20Identity%20%26%20Cross-Domain%20Trust%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |
| Standardisation NumÃ©ration Invariants | [Miyukini Conceptual References - Standardisation NumÃ©ration Invariants](..//..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence


