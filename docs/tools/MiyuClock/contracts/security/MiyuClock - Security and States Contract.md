# MiyuClock â€” Security and States Contract

## 1. Contexte

Ce document dÃ©finit le contrat de **sÃ©curitÃ© et d'Ã©tats systÃ¨me** pour le kit MiyuClock. Il Ã©tablit le niveau de sÃ©curitÃ© du Toolkit, les Ã©tats autorisÃ©s et interdits pour son utilisation, et l'alignement avec WorrySentinel et Caring Nanny.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- Le niveau de sÃ©curitÃ© du kit MiyuClock (niveau 0 ou 1)
- Les Ã©tats systÃ¨me autorisÃ©s (HEALTHY, DEGRADED)
- Les Ã©tats systÃ¨me interdits (SECURITY_LOCKDOWN, MAINTENANCE, etc.)
- L'alignement avec WorrySentinel (niveaux de sÃ©curitÃ©) et Caring Nanny (Ã©tats de confiance)

Ce document **ne couvre pas** :
- Le modÃ¨le de menace dÃ©taillÃ© (voir contrats KindMother / MiyukiniAdmin si pertinent)
- Les permissions par OpÃ©rateur (voir Master Butler - Permission Registry)
- Les tests de sÃ©curitÃ© (voir MiyuClock - Unit Tests Contract / Cycle Tests Contract)

---

## 3. Niveau de sÃ©curitÃ©

### 3.1 Niveau du Toolkit

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit MiyuClock** | **0 ou 1** (donnÃ©es publiques / standard) |
| **Justification** | Les Tools MiyuClock fournissent uniquement la mesure du temps (instant prÃ©sent, delta) ; aucune donnÃ©e sensible ni identitÃ©. Le niveau est cohÃ©rent avec WorrySentinel (time measurement = 0 ou 1 selon politique). Le niveau du Toolkit est au moins Ã©gal au maximum des niveaux de ses Tools composants. |

### 3.2 Alignement WorrySentinel

| Niveau | Nom | Description | MiyuClock |
|--------|-----|-------------|----------|
| **0** | **Public** | **DonnÃ©es publiques** | **Niveau nominal possible** |
| **1** | **Standard** | **DonnÃ©es standard** | **Niveau nominal possible selon politique** |
| 2 | Sensitive | DonnÃ©es sensibles | Non applicable au kit par dÃ©faut |
| 3 | Critical | DonnÃ©es critiques | Non applicable au kit par dÃ©faut |
| 4 | Highest | SÃ©curitÃ© maximale | Non applicable au kit par dÃ©faut |

### 3.3 Invariants sÃ©curitÃ©

| Code | Invariant |
|------|-----------|
| **INV-SEC-1** | Aucun appel Ã  un Tool MiyuClock n'est autorisÃ© si le niveau de sÃ©curitÃ© actuel (WorrySentinel) est infÃ©rieur au niveau requis (0 ou 1) pour les opÃ©rations de mesure du temps |
| **INV-SEC-2** | Le niveau du Toolkit ne peut pas Ãªtre abaissÃ© sans rÃ©vision contractuelle |
| **INV-SEC-3** | Les opÃ©rations de mesure du temps (now, delta) sont assujetties au niveau de sÃ©curitÃ© dÃ©fini pour le Toolkit |

---

## 4. Ã‰tats systÃ¨me autorisÃ©s

### 4.1 Ã‰tats autorisÃ©s

| Ã‰tat | Description | Usage MiyuClock |
|------|-------------|----------------|
| **HEALTHY** | SystÃ¨me sain, toutes capacitÃ©s disponibles | Tous les Tools MiyuClock sont utilisables |
| **DEGRADED** | IncohÃ©rence persistante, capacitÃ©s rÃ©duites | Utilisation selon politique Caring Nanny (ex. now prioritaire, delta restreint ou autorisÃ©) |

### 4.2 Alignement Caring Nanny (Ã‰tats de confiance)

| Ã‰tat confiance | Nom | Description | MiyuClock |
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
| **SECURITY_LOCKDOWN** | Verrouillage sÃ©curitÃ© | Aucun Tool MiyuClock utilisable (ou now seul selon politique) |
| **MAINTENANCE** | Maintenance en cours | Aucun Tool MiyuClock utilisable |
| **Autres** | Selon [Master Butler - Toolkit Composition Contract](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) | Les Ã©tats disallowed_states du Toolkit s'appliquent |

### 5.2 RÃ¨gle d'exÃ©cution

> **Aucun Tool MiyuClock ne doit Ãªtre exÃ©cutÃ© si l'Ã©tat systÃ¨me (Caring Nanny) est dans la liste des Ã©tats interdits pour le Toolkit.**

---

## 6. Contraintes d'utilisation

### 6.1 PrÃ©-conditions Ã  l'appel

Avant toute exÃ©cution d'un Tool MiyuClock :
1. WorrySentinel : le niveau de sÃ©curitÃ© actuel est au moins Ã©gal au niveau requis (0 ou 1 selon politique).
2. Caring Nanny : l'Ã©tat systÃ¨me est dans la liste des Ã©tats autorisÃ©s (HEALTHY ou DEGRADED selon politique).
3. StrongFather : la dÃ©cision est ALLOW pour l'opÃ©ration demandÃ©e.

### 6.2 Refus d'exÃ©cution

Si l'une des prÃ©-conditions n'est pas remplie, l'exÃ©cution est refusÃ©e ; le Tool MiyuClock ne doit pas Ãªtre invoquÃ©. La rÃ©ponse est gÃ©rÃ©e par la gouvernance (BondingBrother / StrongFather), pas par MiyuClock lui-mÃªme.

---

## 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuClock - Documentation Fondatrice | [MiyuClock - Documentation Fondatrice](../../MiyuClock%20-%20Documentation%20Fondatrice.md) |
| MiyuClock - Tool Governance Compliance Contract | [MiyuClock - Tool Governance Compliance Contract](../governance/MiyuClock%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Master Butler - Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| Glossaire â€” Ã‰tats de confiance, Niveaux de sÃ©curitÃ© | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence


