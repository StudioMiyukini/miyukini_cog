# MiyuWeb â€” Security and States Contract

## 1. Contexte

Ce document dÃ©finit le contrat de **sÃ©curitÃ© et d'Ã©tats systÃ¨me** pour le kit MiyuWeb. Il Ã©tablit le niveau de sÃ©curitÃ© du Toolkit, les Ã©tats autorisÃ©s et interdits pour son utilisation, et l'alignement avec WorrySentinel et Caring Nanny. Il mentionne les risques spÃ©cifiques au domaine web (XSS, CSP) dans le pÃ©rimÃ¨tre d'exposition du Toolkit.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- Le niveau de sÃ©curitÃ© du kit MiyuWeb (niveau 0, 1 ou 2 selon politique d'exposition)
- Les Ã©tats systÃ¨me autorisÃ©s (HEALTHY, DEGRADED)
- Les Ã©tats systÃ¨me interdits (SECURITY_LOCKDOWN, MAINTENANCE, etc.)
- L'alignement avec WorrySentinel (niveaux de sÃ©curitÃ©) et Caring Nanny (Ã©tats de confiance)
- Les contraintes de sÃ©curitÃ© applicables au domaine web (sanitization, CSP)

Ce document **ne couvre pas** :
- Le modÃ¨le de menace dÃ©taillÃ© (voir contrats KindMother / MiyukiniAdmin si pertinent)
- Les permissions par OpÃ©rateur (voir Master Butler - Permission Registry)
- Les tests de sÃ©curitÃ© (voir MiyuWeb - Unit Tests Contract / Cycle Tests Contract)

---

## 3. Niveau de sÃ©curitÃ©

### 3.1 Niveau du Toolkit

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit MiyuWeb** | **0, 1 ou 2** selon politique d'exposition |
| **Justification** | Les Tools MiyuWeb manipulent l'affichage de contenu web (rendu HTML, scripts, assets, formulaires, Ã©vÃ©nements). Le niveau dÃ©pend de l'exposition : contenu public (0), standard (1), contenu sensible ou contrÃ´lÃ© (2). Le niveau du Toolkit est au moins Ã©gal au maximum des niveaux de ses Tools composants. CohÃ©rent avec WorrySentinel. |

### 3.2 Alignement WorrySentinel

| Niveau | Nom | Description | MiyuWeb |
|--------|-----|-------------|----------|
| **0** | **Public** | **DonnÃ©es publiques, exposition faible** | **Niveau nominal possible (faÃ§ade publique)** |
| **1** | **Standard** | **DonnÃ©es standard, contraintes de base** | **Niveau nominal possible** |
| **2** | **Sensitive** | **DonnÃ©es sensibles, contraintes renforcÃ©es** | **Niveau nominal possible selon politique** |
| 3 | Critical | DonnÃ©es critiques | Non applicable au kit par dÃ©faut |
| 4 | Highest | SÃ©curitÃ© maximale | Non applicable au kit par dÃ©faut |

### 3.3 Invariants sÃ©curitÃ©

| Code | Invariant |
|------|-----------|
| **INV-SEC-1** | Aucun appel Ã  un Tool MiyuWeb n'est autorisÃ© si le niveau de sÃ©curitÃ© actuel (WorrySentinel) est infÃ©rieur au niveau requis (0, 1 ou 2 selon politique d'exposition) pour les opÃ©rations web |
| **INV-SEC-2** | Le niveau du Toolkit ne peut pas Ãªtre abaissÃ© sans rÃ©vision contractuelle |
| **INV-SEC-3** | Les opÃ©rations de rendu, script, asset, formulaire et Ã©vÃ©nement sont assujetties au niveau de sÃ©curitÃ© dÃ©fini pour le Toolkit |

---

## 4. Risques web (XSS, CSP)

### 4.1 PÃ©rimÃ¨tre contractuel

MiyuWeb opÃ¨re sur des **donnÃ©es fournies dans le flux** (templates, contenu, assets). Les risques suivants relÃ¨vent du pÃ©rimÃ¨tre d'exposition du Toolkit et doivent Ãªtre pris en compte par l'implÃ©mentation et la gouvernance :

| Risque | Description | Contrainte contractuelle |
|--------|-------------|---------------------------|
| **XSS (Cross-Site Scripting)** | Injection de script malveillant dans le contenu rendu | Tout contenu destinÃ© au rendu HTML ou Ã  l'exÃ©cution de script doit Ãªtre traitÃ© selon la politique de sanitization dÃ©finie par l'environnement ; MiyuWeb n'introduit pas de contenu non gouvernÃ© |
| **CSP (Content Security Policy)** | Politique de sÃ©curitÃ© du contenu (sources autorisÃ©es, inline, eval) | L'implÃ©mentation des Tools MiyuWeb (notamment `tool.web.script.execute`, `tool.web.html.render`) doit Ãªtre compatible avec les directives CSP dÃ©finies par WorrySentinel / environnement ; MiyuWeb ne contourne pas la CSP |

### 4.2 RÃ¨gle

> **MiyuWeb exÃ©cute le rendu et les scripts sur des donnÃ©es fournies dans le flux ; la responsabilitÃ© de la sanitization et du respect de la CSP incombe Ã  l'implÃ©mentation et Ã  la gouvernance (WorrySentinel, politique d'exposition), pas Ã  la dÃ©cision de contenu par MiyuWeb.**

---

## 5. Ã‰tats systÃ¨me autorisÃ©s

### 5.1 Ã‰tats autorisÃ©s

| Ã‰tat | Description | Usage MiyuWeb |
|------|-------------|----------------|
| **HEALTHY** | SystÃ¨me sain, toutes capacitÃ©s disponibles | Tous les Tools MiyuWeb sont utilisables |
| **DEGRADED** | IncohÃ©rence persistante, capacitÃ©s rÃ©duites | Utilisation selon politique Caring Nanny (ex. rendu prioritaire, scripts restreints ou dÃ©sactivÃ©s) |

### 5.2 Alignement Caring Nanny (Ã‰tats de confiance)

| Ã‰tat confiance | Nom | Description | MiyuWeb |
|----------------|-----|-------------|----------|
| **T0** | Normal | SystÃ¨me sain | AutorisÃ© |
| **T1** | Instable | Anomalie dÃ©tectÃ©e | AutorisÃ© avec surveillance |
| **T2** | DÃ©gradÃ© | CapacitÃ©s rÃ©duites | AutorisÃ© selon politique |
| T3 | Restreint | Suspicion forte | Interdit ou trÃ¨s restreint |
| T4 | BloquÃ© | IntÃ©gritÃ© rompue | Interdit |

---

## 6. Ã‰tats systÃ¨me interdits

### 6.1 Ã‰tats interdits

| Ã‰tat | Description | Effet |
|------|-------------|-------|
| **SECURITY_LOCKDOWN** | Verrouillage sÃ©curitÃ© | Aucun Tool MiyuWeb d'exÃ©cution de script ou de rendu non sÃ©curisÃ© ; rendu restreint selon politique |
| **MAINTENANCE** | Maintenance en cours | Aucun Tool MiyuWeb utilisable (ou rendu seul selon politique) |
| **Autres** | Selon [Master Butler - Toolkit Composition Contract](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) | Les Ã©tats disallowed_states du Toolkit s'appliquent |

### 6.2 RÃ¨gle d'exÃ©cution

> **Aucun Tool MiyuWeb ne doit Ãªtre exÃ©cutÃ© si l'Ã©tat systÃ¨me (Caring Nanny) est dans la liste des Ã©tats interdits pour le Toolkit.**

---

## 7. Contraintes d'utilisation

### 7.1 PrÃ©-conditions Ã  l'appel

Avant toute exÃ©cution d'un Tool MiyuWeb :
1. WorrySentinel : le niveau de sÃ©curitÃ© actuel est au moins Ã©gal au niveau requis (0, 1 ou 2 selon politique).
2. Caring Nanny : l'Ã©tat systÃ¨me est dans la liste des Ã©tats autorisÃ©s (HEALTHY ou DEGRADED selon politique).
3. StrongFather : la dÃ©cision est ALLOW pour l'opÃ©ration demandÃ©e.
4. KindMother : les donnÃ©es (templates, assets) fournies dans le flux proviennent d'un chemin gouvernÃ© ; MiyuWeb ne lit pas la base directement (voir [MiyuWeb - KindMother Integration Contract](../integration/MiyuWeb%20-%20KindMother%20Integration%20Contract.md)).

### 7.2 Refus d'exÃ©cution

Si l'une des prÃ©-conditions n'est pas remplie, l'exÃ©cution est refusÃ©e ; le Tool MiyuWeb ne doit pas Ãªtre invoquÃ©. La rÃ©ponse est gÃ©rÃ©e par la gouvernance (BondingBrother / StrongFather), pas par MiyuWeb lui-mÃªme.

---

## 8. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuWeb - Documentation Fondatrice | [MiyuWeb - Documentation Fondatrice](../../MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| MiyuWeb - Tool Governance Compliance Contract | [MiyuWeb - Tool Governance Compliance Contract](../governance/MiyuWeb%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| MiyuWeb - KindMother Integration Contract | [MiyuWeb - KindMother Integration Contract](../integration/MiyuWeb%20-%20KindMother%20Integration%20Contract.md) |
| Master Butler - Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| Glossaire â€” Ã‰tats de confiance, Niveaux de sÃ©curitÃ© | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |
| Security Levels (rÃ©fÃ©rence conceptuelle) | [Miyukini Conceptual References - Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence


