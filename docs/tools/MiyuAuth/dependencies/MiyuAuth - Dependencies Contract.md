# MiyuAuth â€” Dependencies Contract

## 1. Contexte

Ce document dÃ©finit le contrat des **dÃ©pendances** du kit MiyuAuth. Il Ã©tablit la liste fermÃ©e des dÃ©pendances (KindMother, Master Butler, BondingBrother, StrongFather, WorrySentinel, Caring Nanny, Kernel), l'absence de dÃ©pendance mÃ©tier, et l'ordre ou les contraintes d'utilisation.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- La liste fermÃ©e des dÃ©pendances de MiyuAuth (Cores et Kernel)
- L'absence de dÃ©pendance mÃ©tier (OpÃ©rateurs, produits, rÃ¨gles mÃ©tier)
- L'ordre et les contraintes (flux d'appel, prÃ©-conditions)
- Les invariants de dÃ©pendance

Ce document **ne couvre pas** :
- Les dÃ©pendances d'implÃ©mentation (stockage identitÃ©, signatures, librairies techniques) â€” hors scope documentaire fondateur
- Les dÃ©pendances des Cores eux-mÃªmes

---

## 3. Principe fondamental

### 3.1 Liste fermÃ©e

> **MiyuAuth ne dÃ©pend que des Cores et du Kernel dÃ©finis dans ce contrat. Aucune dÃ©pendance mÃ©tier (OpÃ©rateur, produit, rÃ¨gle mÃ©tier) n'est autorisÃ©e.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-DEP-1** | MiyuAuth ne connaÃ®t pas les OpÃ©rateurs ; il reÃ§oit un contexte gouvernÃ© (BondingBrother) |
| **INV-DEP-2** | MiyuAuth ne dÃ©pend d'aucun produit ni rÃ¨gle mÃ©tier applicative |
| **INV-DEP-3** | Toute invocation de MiyuAuth passe par la mÃ©diation BondingBrother et la gouvernance (Master Butler, WorrySentinel, Caring Nanny, StrongFather) |
| **INV-DEP-4** | Toute utilisation de confiance inter-domaines pour l'identitÃ© est validÃ©e par KindMother uniquement |
| **INV-DEP-5** | MiyuAuth n'est invoquÃ© qu'aprÃ¨s dÃ©cision ALLOW de StrongFather |

---

## 4. Liste fermÃ©e des dÃ©pendances

### 4.1 Cores (Strate 4)

| DÃ©pendance | RÃ´le pour MiyuAuth | Contrainte |
|------------|---------------------|------------|
| **KindMother** | Validateur unique de la confiance inter-domaines ; toute confiance utilisÃ©e pour l'identitÃ© est validÃ©e par KindMother | MiyuAuth n'utilise aucune confiance non validÃ©e par KindMother ; pas de dÃ©lÃ©gation de validation |
| **Master Butler** | Catalogue des Tools et Toolkits ; permissions ; dÃ©claration de MiyuAuth et des ToolIds | MiyuAuth est invoquÃ© aprÃ¨s vÃ©rification Master Butler |
| **StrongFather** | DÃ©cision ALLOW/DENY pour l'utilisation des Tools | MiyuAuth n'est invoquÃ© qu'en cas d'ALLOW |
| **WorrySentinel** | Niveau de sÃ©curitÃ© ; vÃ©rification que le niveau actuel permet l'appel | PrÃ©-condition Ã  l'invocation |
| **Caring Nanny** | Ã‰tat systÃ¨me ; vÃ©rification que l'Ã©tat (HEALTHY, DEGRADED, etc.) permet l'appel | PrÃ©-condition Ã  l'invocation |

### 4.2 Interface & MÃ©diation (Strate 5)

| DÃ©pendance | RÃ´le pour MiyuAuth | Contrainte |
|------------|---------------------|------------|
| **BondingBrother** | MÃ©diation ; traduction de l'intention ; prÃ©paration du contexte ; passage des demandes vers les Cores | MiyuAuth reÃ§oit les demandes via BondingBrother (ou via le flux gouvernÃ© initiÃ© par BondingBrother) |

### 4.3 Kernel (Strate K)

| DÃ©pendance | RÃ´le pour MiyuAuth | Contrainte |
|------------|---------------------|------------|
| **Kernel** | Id (identifiants), Logger (traÃ§abilitÃ©), Clock (horodatage), Config (configuration locale), Lifecycle | Usage minimal et neutre ; pas de logique mÃ©tier ; conformitÃ© aux invariants Kernel |

---

## 5. Ordre et contraintes

### 5.1 Flux d'invocation (ordre)

L'ordre d'implication des dÃ©pendances lors d'un appel Ã  un Tool MiyuAuth est :

1. **OpÃ©rateur** (hors dÃ©pendance MiyuAuth) Ã©met une intention.
2. **BondingBrother** â€” mÃ©diation, traduction, contexte.
3. **Master Butler** â€” vÃ©rification Tool/Toolkit, permissions.
4. **WorrySentinel** â€” niveau de sÃ©curitÃ©.
5. **Caring Nanny** â€” Ã©tat systÃ¨me.
6. **StrongFather** â€” dÃ©cision ALLOW/DENY.
7. Si ALLOW et si confiance inter-domaines nÃ©cessaire : **KindMother** â€” validation de la confiance.
8. **MiyuAuth** â€” exÃ©cution du Tool mandatÃ© (resolve, attest, verify, role).

### 5.2 Contraintes

| Contrainte | Description |
|------------|-------------|
| **Pas d'invocation directe** | MiyuAuth n'est jamais invoquÃ© directement par un OpÃ©rateur ; toujours via BondingBrother et la chaÃ®ne de gouvernance |
| **Pas de bypass** | Aucune dÃ©pendance ne peut Ãªtre contournÃ©e (pas d'utilisation de confiance sans KindMother, pas d'exÃ©cution sans StrongFather ALLOW) |
| **Pas de dÃ©pendance inverse mÃ©tier** | Aucun Core ni Kernel ne dÃ©pend de MiyuAuth pour sa logique mÃ©tier ; MiyuAuth est un outil consommÃ© par le flux |

---

## 6. Absence de dÃ©pendance mÃ©tier

### 6.1 Ce dont MiyuAuth ne dÃ©pend pas

| Type | Exemples | Raison |
|------|----------|--------|
| **OpÃ©rateurs** | MiyukiniAdmin, tout OpÃ©rateur de domaine | MiyuAuth est un Toolkit ; les OpÃ©rateurs utilisent MiyuAuth via la gouvernance |
| **Produits / RÃ¨gles mÃ©tier** | SchÃ©mas applicatifs, rÃ¨gles mÃ©tier | MiyuAuth n'interprÃ¨te pas le mÃ©tier ; il exÃ©cute des capacitÃ©s mandatees |
| **Autres Toolkits** | MiyuSQL, kits d'outils mÃ©tier | MiyuAuth est indÃ©pendant des autres Toolkits ; pas de couplage fonctionnel direct. La **persistance** des donnÃ©es d'identification (Passeport, Visa) est exÃ©cutÃ©e via **MiyuSQL** sous autoritÃ© **KindMother** ; MiyuAuth ne lit ni n'Ã©crit en base â€” il opÃ¨re sur des donnÃ©es fournies dans le flux (voir [MiyuAuth - Documentation Fondatrice](../MiyuAuth%20-%20Documentation%20Fondatrice.md) section 8bis). |
| **Services externes** | APIs externes, rÃ©seau mÃ©tier | ConformitÃ© LOI-1 ; pas de dÃ©pendance externe critique |

### 6.2 DÃ©pendances techniques (hors scope contractuel)

Les dÃ©pendances techniques (stockage identitÃ©, signatures, librairies) sont hors scope de ce contrat fondateur. Elles seront dÃ©finies dans le guide d'implÃ©mentation (MiyuAuth - Reference Implementation Guidelines) et doivent rester neutres (pas de logique mÃ©tier).

---

## 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuAuth - Documentation Fondatrice | [MiyuAuth - Documentation Fondatrice](../MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| MiyuAuth - KindMother Integration Contract | [MiyuAuth - KindMother Integration Contract](../contracts/integration/MiyuAuth%20-%20KindMother%20Integration%20Contract.md) |
| MiyuAuth - Tool Governance Compliance Contract | [MiyuAuth - Tool Governance Compliance Contract](../contracts/governance/MiyuAuth%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| MiyuAuth - Runtime Boundary Contract | [MiyuAuth - Runtime Boundary Contract](../contracts/boundaries/MiyuAuth%20-%20Runtime%20Boundary%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence

