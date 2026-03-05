# MiyuAuth â€” Security and States Contract

## 1. Contexte

Ce document dÃ©finit le contrat de **sÃ©curitÃ© et d'Ã©tats systÃ¨me** pour le kit MiyuAuth. Il Ã©tablit le niveau de sÃ©curitÃ© du Toolkit, les Ã©tats autorisÃ©s et interdits pour son utilisation, et l'alignement avec WorrySentinel et Caring Nanny.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- Le niveau de sÃ©curitÃ© du kit MiyuAuth (niveau 2 ou 3 selon politique identitÃ©)
- Les Ã©tats systÃ¨me autorisÃ©s (HEALTHY, DEGRADED)
- Les Ã©tats systÃ¨me interdits (SECURITY_LOCKDOWN, MAINTENANCE, etc.)
- L'alignement avec WorrySentinel (niveaux de sÃ©curitÃ©) et Caring Nanny (Ã©tats de confiance)

Ce document **ne couvre pas** :
- Le modÃ¨le de menace dÃ©taillÃ© (voir contrats KindMother / MiyukiniAdmin si pertinent)
- Les permissions par OpÃ©rateur (voir Master Butler - Permission Registry)
- Les tests de sÃ©curitÃ© (voir MiyuAuth - Unit Tests Contract / Cycle Tests Contract)

---

## 3. Niveau de sÃ©curitÃ©

### 3.1 Niveau du Toolkit

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit MiyuAuth** | **2 ou 3** (donnÃ©es sensibles / identitÃ©) |
| **Justification** | Les Tools MiyuAuth manipulent l'identitÃ© utilisateur (rÃ©solution rÃ´le, attestation, vÃ©rification Passeport/Visa). Le niveau est cohÃ©rent avec WorrySentinel (Identity Tools = 2 ou 3 selon politique). Le niveau du Toolkit est au moins Ã©gal au maximum des niveaux de ses Tools composants. |

### 3.2 Alignement WorrySentinel

| Niveau | Nom | Description | MiyuAuth |
|--------|-----|-------------|----------|
| 0 | Public | DonnÃ©es publiques | Non applicable |
| 1 | Standard | DonnÃ©es standard | Non applicable au kit par dÃ©faut |
| **2** | **Sensitive** | **DonnÃ©es utilisateur / identitÃ©** | **Niveau nominal possible** |
| **3** | **Critical** | **DonnÃ©es critiques / identitÃ©** | **Niveau nominal possible selon politique** |
| 4 | Highest | SÃ©curitÃ© maximale | Non applicable au kit par dÃ©faut |

### 3.3 Invariants sÃ©curitÃ©

| Code | Invariant |
|------|-----------|
| **INV-SEC-1** | Aucun appel Ã  un Tool MiyuAuth n'est autorisÃ© si le niveau de sÃ©curitÃ© actuel (WorrySentinel) est infÃ©rieur au niveau requis (2 ou 3) pour les opÃ©rations d'identitÃ© |
| **INV-SEC-2** | Le niveau du Toolkit ne peut pas Ãªtre abaissÃ© sans rÃ©vision contractuelle |
| **INV-SEC-3** | Les opÃ©rations d'identitÃ© (resolve, attest, verify, role) sont assujetties au niveau de sÃ©curitÃ© dÃ©fini pour le Toolkit |

---

## 4. Ã‰tats systÃ¨me autorisÃ©s

### 4.1 Ã‰tats autorisÃ©s

| Ã‰tat | Description | Usage MiyuAuth |
|------|-------------|----------------|
| **HEALTHY** | SystÃ¨me sain, toutes capacitÃ©s disponibles | Tous les Tools MiyuAuth sont utilisables |
| **DEGRADED** | IncohÃ©rence persistante, capacitÃ©s rÃ©duites | Utilisation selon politique Caring Nanny (ex. rÃ©solution prioritaire, attestation restreinte ou interdite) |

### 4.2 Alignement Caring Nanny (Ã‰tats de confiance)

| Ã‰tat confiance | Nom | Description | MiyuAuth |
|----------------|-----|-------------|----------|
| **T0** | Normal | SystÃ¨me sain | AutorisÃ© |
| **T1** | Instable | Anomalie dÃ©tectÃ©e | AutorisÃ© avec surveillance |
| **T2** | DÃ©gradÃ© | CapacitÃ©s rÃ©duites | AutorisÃ© selon politique |
| T3 | Restreint | Suspicion forte | Interdit ou trÃ¨s restreint |
| T4 | BloquÃ© | IntÃ©gritÃ© rompue | Interdit |

---

## 5. Ã‰tats systÃ¨me interdits

### 5.1 Ã‰tats interdits

| Ã‰tat | Description | Effet |
|------|-------------|-------|
| **SECURITY_LOCKDOWN** | Verrouillage sÃ©curitÃ© | Aucun Tool MiyuAuth d'attestation ou de vÃ©rification sensible ; rÃ©solution Ã©ventuellement restreinte |
| **MAINTENANCE** | Maintenance en cours | Aucun Tool MiyuAuth utilisable (ou rÃ©solution seule selon politique) |
| **Autres** | Selon [Master Butler - Toolkit Composition Contract](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) | Les Ã©tats disallowed_states du Toolkit s'appliquent |

### 5.2 RÃ¨gle d'exÃ©cution

> **Aucun Tool MiyuAuth ne doit Ãªtre exÃ©cutÃ© si l'Ã©tat systÃ¨me (Caring Nanny) est dans la liste des Ã©tats interdits pour le Toolkit.**

---

## 6. Contraintes d'utilisation

### 6.1 PrÃ©-conditions Ã  l'appel

Avant toute exÃ©cution d'un Tool MiyuAuth :
1. WorrySentinel : le niveau de sÃ©curitÃ© actuel est au moins Ã©gal au niveau requis (2 ou 3 selon politique).
2. Caring Nanny : l'Ã©tat systÃ¨me est dans la liste des Ã©tats autorisÃ©s (HEALTHY ou DEGRADED selon politique).
3. StrongFather : la dÃ©cision est ALLOW pour l'opÃ©ration demandÃ©e.
4. KindMother : pour toute utilisation de confiance inter-domaines (identitÃ©), la validation KindMother est obtenue.

### 6.2 Refus d'exÃ©cution

Si l'une des prÃ©-conditions n'est pas remplie, l'exÃ©cution est refusÃ©e ; le Tool MiyuAuth ne doit pas Ãªtre invoquÃ©. La rÃ©ponse est gÃ©rÃ©e par la gouvernance (BondingBrother / StrongFather), pas par MiyuAuth lui-mÃªme.

---

## 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuAuth - Documentation Fondatrice | [MiyuAuth - Documentation Fondatrice](../../MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| MiyuAuth - Tool Governance Compliance Contract | [MiyuAuth - Tool Governance Compliance Contract](../governance/MiyuAuth%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Master Butler - Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| Glossaire â€” Ã‰tats de confiance, Niveaux de sÃ©curitÃ© | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |
| Security Levels (rÃ©fÃ©rence conceptuelle) | [Miyukini Conceptual References - Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence


