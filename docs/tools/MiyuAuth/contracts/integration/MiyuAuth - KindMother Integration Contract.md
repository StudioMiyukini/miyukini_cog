# MiyuAuth â€” KindMother Integration Contract

## 1. Contexte

Ce document dÃ©finit le contrat d'intÃ©gration entre **MiyuAuth** (kit d'outils d'identitÃ© utilisateur) et **KindMother** (Core de donnÃ©es, Strate 4). KindMother est l'unique validateur de la confiance inter-domaines ([KindMother - Identity & Cross-Domain Trust Contract](..//..//..//..//cores//KindMother//contracts//authority//KindMother%20-%20Identity%20%26%20Cross-Domain%20Trust%20Contract.md)). MiyuAuth exÃ©cute des capacitÃ©s (resolve, attest, verify, role) sans dÃ©cider de la confiance ; toute confiance utilisÃ©e pour l'identitÃ© est validÃ©e par KindMother.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- Le rÃ´le unique de KindMother comme validateur de la confiance inter-domaines
- L'exÃ©cution des capacitÃ©s MiyuAuth (resolve, attest, verify, role) sans dÃ©cision de confiance
- L'invariant : aucune confiance sans validation KindMother, pas de dÃ©lÃ©gation de validation

Ce document **ne couvre pas** :
- L'implÃ©mentation interne de KindMother
- Les contrats MiyuAuth hors intÃ©gration (gouvernance, sÃ©curitÃ©, bornage)
- Le dÃ©tail du modÃ¨le Identity & Cross-Domain Trust (voir KindMother - Identity & Cross-Domain Trust Contract)

---

## 3. Principe fondamental

### 3.1 KindMother = validateur unique de la confiance

> **KindMother est l'unique validateur de toute confiance inter-domaines. MiyuAuth exÃ©cute des capacitÃ©s (resolve, attest, verify, role) sans dÃ©cider de la confiance ; toute confiance utilisÃ©e pour l'identitÃ© est validÃ©e par KindMother.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-KM-1** | Aucune confiance inter-domaines n'est utilisÃ©e pour l'identitÃ© sans validation explicite par KindMother |
| **INV-KM-2** | MiyuAuth ne valide pas la confiance ; il exÃ©cute les capacitÃ©s mandatees aprÃ¨s validation KindMother |
| **INV-KM-3** | La validation de la confiance n'est pas dÃ©lÃ©guable ; KindMother ne dÃ©lÃ¨gue pas Ã  MiyuAuth ni Ã  un adaptateur |
| **INV-KM-4** | MiyuAuth n'exÃ©cute que ce qui a Ã©tÃ© autorisÃ© par la gouvernance (StrongFather, KindMother) |
| **INV-KM-5** | MiyuAuth n'ajoute aucune logique mÃ©tier ; il orchestre des capacitÃ©s atomiques d'identitÃ© |

---

## 4. RÃ´le des Tools MiyuAuth

### 4.1 CapacitÃ©s exÃ©cutÃ©es, pas de dÃ©cision de confiance

| ToolId | RÃ´le | AutoritÃ© / Validation |
|--------|------|------------------------|
| `tool.identity.resolve` | RÃ©sout un contexte d'identitÃ© (citoyen, visiteur, externe) Ã  partir des donnÃ©es fournies | Toute confiance utilisÃ©e pour la rÃ©solution est validÃ©e par KindMother ; MiyuAuth exÃ©cute, ne dÃ©cide pas |
| `tool.identity.attest` | Produit une attestation d'identitÃ© pour un contexte validÃ© | Le contexte doit avoir Ã©tÃ© validÃ© par KindMother ; MiyuAuth exÃ©cute l'attestation |
| `tool.identity.verify` | VÃ©rifie un Passeport Utilisateur ou un Visa de Connexion (structure, signature) | VÃ©rification technique ; la validation de la confiance reste Ã  KindMother |
| `tool.identity.role` | Retourne le rÃ´le rÃ©solu (citoyen, visiteur, externe) | Contexte gouvernÃ© ; MiyuAuth exÃ©cute, ne confÃ¨re pas d'autorisation |

### 4.2 Ce que MiyuAuth ne fait jamais

| Interdiction | Description |
|-------------|-------------|
| **INTERDIT-1** | DÃ©cider de la confiance inter-domaines (validation = KindMother uniquement) |
| **INTERDIT-2** | Utiliser une confiance non validÃ©e par KindMother pour l'identitÃ© |
| **INTERDIT-3** | DÃ©lÃ©guer ou recevoir une dÃ©lÃ©gation de validation de confiance |
| **INTERDIT-4** | Traiter la reconnaissance d'identitÃ© comme une autorisation (identitÃ© â‰  autorisation) |

---

## 5. Flux de confiance

```
OpÃ©rateur / Adaptateur
        â”‚
        â”‚ 1. Demande d'utilisation d'un Tool MiyuAuth (resolve, attest, verify, role)
        â–¼
BondingBrother â”€â”€â–º Master Butler â”€â”€â–º WorrySentinel â”€â”€â–º Caring Nanny â”€â”€â–º StrongFather
        â”‚                                                                      â”‚
        â”‚ 2. ALLOW                                                             â”‚
        â–¼                                                                      â”‚
KindMother : validation de la confiance (si nÃ©cessaire pour l'identitÃ©)       â”‚
        â”‚                                                                      â”‚
        â”‚ 3. Mandat d'exÃ©cution (tool.identity.*)                             â”‚
        â–¼                                                                      â”‚
MiyuAuth Tools : exÃ©cution gouvernÃ©e (sans dÃ©cision de confiance)
```

---

## 6. Absence de contournement

Aucun chemin ne peut contourner :
1. La mÃ©diation BondingBrother (intention, contexte)
2. Le catalogue Master Butler (Tool/Toolkit, permissions)
3. Les Cores WorrySentinel et Caring Nanny (sÃ©curitÃ©, Ã©tat systÃ¨me)
4. La dÃ©cision StrongFather (ALLOW/DENY)
5. La validation KindMother (confiance inter-domaines pour l'identitÃ©)

MiyuAuth n'exÃ©cute que dans le cadre de ce flux ; il ne valide jamais la confiance lui-mÃªme.

---

## 6bis. Relation avec MiyuSQL â€” DonnÃ©es d'identification, Passeport, Visa

### 6bis.1 Persistance des donnÃ©es d'identification

La **persistance** (lecture / Ã©criture en base) des donnÃ©es d'identification, des Passeports Utilisateurs et des Visas de Connexion relÃ¨ve de **KindMother** et est **exÃ©cutÃ©e via MiyuSQL** lorsque KindMother mandate les opÃ©rations (WriteIntent pour les Ã©critures, mandat d'exÃ©cution pour les lectures). MiyuAuth **ne persiste pas** et **ne lit pas** en base ; il ne dÃ©pend pas de MiyuSQL et n'accÃ¨de pas Ã  la persistance.

| OpÃ©ration | AutoritÃ© | ExÃ©cution technique | MiyuAuth |
|-----------|----------|----------------------|----------|
| Stockage (crÃ©ation, mise Ã  jour) Passeport / Visa | KindMother | MiyuSQL (sous WriteIntent) | N'intervient pas |
| Lecture Passeport / Visa depuis la base | KindMother | MiyuSQL (mandat d'exÃ©cution) | N'intervient pas |
| VÃ©rification (structure, signature) d'un artefact fourni | â€” | MiyuAuth (`tool.identity.verify`) | ExÃ©cute sur l'artefact reÃ§u |
| RÃ©solution rÃ´le / contexte Ã  partir de donnÃ©es fournies | â€” | MiyuAuth (`tool.identity.resolve`, `tool.identity.role`) | ExÃ©cute sur le contexte reÃ§u |

### 6bis.2 Flux gouvernÃ© typique

Les donnÃ©es (Passeport, Visa, enregistrements d'identitÃ©) sont d'abord **lues ou produites** sous autoritÃ© KindMother (avec MiyuSQL pour la persistance). Elles sont ensuite **fournies au flux** (contexte, session, paramÃ¨tres). MiyuAuth est invoquÃ© sur ces donnÃ©es **dÃ©jÃ  prÃ©sentes dans le flux** pour vÃ©rification, rÃ©solution ou attestation â€” sans accÃ©der lui-mÃªme Ã  la base.

**RÃ©fÃ©rence :** [MiyuAuth - Documentation Fondatrice](../../MiyuAuth%20-%20Documentation%20Fondatrice.md) (section 8bis Relation avec MiyuSQL), [MiyuSQL - KindMother Integration Contract](..//..//..//MiyuSQL//contracts//integration//MiyuSQL%20-%20KindMother%20Integration%20Contract.md).

---

## 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuAuth - Documentation Fondatrice | [MiyuAuth - Documentation Fondatrice](../../MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| KindMother - Identity & Cross-Domain Trust Contract | [KindMother - Identity & Cross-Domain Trust Contract](..//..//..//..//cores//KindMother//contracts//authority//KindMother%20-%20Identity%20%26%20Cross-Domain%20Trust%20Contract.md) |
| KindMother - Index | [KindMother - Index](..//..//..//_index.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |
| Connexion Inter-COG | [Miyukini Conceptual References - Connexion Inter-COG](..//..//..//..//miyukini-webway-system//reference//_index.md) |
| MiyuSQL - KindMother Integration Contract | [MiyuSQL - KindMother Integration Contract](..//..//..//MiyuSQL//contracts//integration//MiyuSQL%20-%20KindMother%20Integration%20Contract.md) |
| Security Levels (rÃ©fÃ©rence conceptuelle) | [Miyukini Conceptual References - Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence



